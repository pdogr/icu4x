// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::{MutableZeroVecLike, ZeroMapKV, ZeroVecLike};
use crate::ZeroVec;
use alloc::borrow::Borrow;
use alloc::vec;
use alloc::vec::Vec;
use core::hash::Hash;

pub type Split3Fn = fn(u64, u32) -> (usize, u32, u32);
pub type HashFn<K> = for<'a> fn(&'a K) -> u64;
pub type HashFnWithSeed<K> = for<'a> fn(&'a K, u32) -> u64;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PAHashIndex<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    displacements: ZeroVec<'a, (u32, u32)>,
}

impl<'a> PAHashIndex<'a> {
    #[inline]
    pub fn compute_displacement(f: (u32, u32), d: (u32, u32), m: u32) -> usize {
        (f.1.wrapping_mul(d.0).wrapping_add(f.0).wrapping_add(d.1) % m) as usize
    }
    /// Build the hashIndex and permute keys, values according to the hash.
    #[inline]
    #[allow(clippy::indexing_slicing, clippy::unwrap_used)] // proper documentation at each occurence
    pub fn build_from_kv_containers_with_split3_with_hf<'b, K, V>(
        keys: &mut K::Container,
        values: &mut V::Container,
        splitter: Split3Fn,
        compute_hash_fn: HashFn<K>,
    ) -> Self
    where
        K: ZeroMapKV<'a> + 'b + ?Sized + Hash,
        V: ZeroMapKV<'a> + ?Sized,
    {
        let len = keys.zvl_len();
        let mut bucket_sizes = vec![0; len];
        let mut bucket_flatten = Vec::with_capacity(len);
        for i in 0..len {
            let h = K::Container::zvl_get_as_t(keys.zvl_get(i).unwrap(), |k| {
                let hash = compute_hash_fn(k);
                splitter(hash, len as u32)
            });
            bucket_sizes[h.0] += 1;
            bucket_flatten.push((h, i));
        }
        bucket_flatten
            .sort_by(|&(ha, _), &(hb, _)| (bucket_sizes[hb.0], hb).cmp(&(bucket_sizes[ha.0], ha)));

        let mut generation = 0;
        let mut occupied = vec![false; len];
        let mut assignments = vec![0; len];
        let mut current_displacements = Vec::with_capacity(16);
        let mut displacements = vec![(0, 0); len];
        let mut reverse_mapping = vec![0; len];

        let mut start = 0;
        while start < len {
            let g = bucket_flatten[start].0 .0;
            let end = start + bucket_sizes[g];
            let buckets = &bucket_flatten[start..end];

            'd0: for d0 in 0..len as u32 {
                'd1: for d1 in 0..len as u32 {
                    current_displacements.clear();
                    generation += 1;

                    for ((_, f0, f1), _) in buckets {
                        let displacement_idx =
                            PAHashIndex::compute_displacement((*f0, *f1), (d0, d1), len as u32);

                        if occupied[displacement_idx] || assignments[displacement_idx] == generation
                        {
                            continue 'd1;
                        }
                        assignments[displacement_idx] = generation;

                        current_displacements.push(displacement_idx);
                    }

                    // Successfully found a (d0, d1), store it as index g.
                    // g < displacements.len() due to modulo operation
                    displacements[g] = (d0, d1);

                    for (i, displacement_idx) in current_displacements.iter().enumerate() {
                        // `current_displacements` has same size as `buckets`
                        let (_, idx) = &buckets[i];

                        // displacement_idx is always within bounds
                        occupied[*displacement_idx] = true;
                        reverse_mapping[*displacement_idx] = *idx;
                    }
                    break 'd0;
                }
            }

            start = end;
        }

        keys.zvl_permute(&mut reverse_mapping.clone());
        values.zvl_permute(&mut reverse_mapping);

        Self {
            displacements: ZeroVec::alloc_from_slice(&displacements),
        }
    }

    #[inline]
    pub fn index<'b, K, A>(
        &'b self,
        k: A,
        splitter: Split3Fn,
        compute_hash_fn: HashFn<K>,
    ) -> Option<usize>
    where
        K: Hash + ?Sized,
        A: Borrow<K>,
    {
        let hash = compute_hash_fn(k.borrow());
        let (g, f0, f1) = splitter(hash, self.displacements.len() as u32);
        let (d0, d1) = self.displacements.get(g).unwrap();
        Some(PAHashIndex::compute_displacement(
            (f0, f1),
            (d0, d1),
            self.displacements.len() as u32,
        ))
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GAHashIndex<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    seeds: ZeroVec<'a, u32>,
}

impl<'a> GAHashIndex<'a> {
    /// Build the hashIndex and permute keys, values according to the hash.
    #[inline]
    #[allow(clippy::indexing_slicing, clippy::unwrap_used)] // proper documentation at each occurence
    pub fn build_from_kv_containers_with_hf<'b, K, V>(
        keys: &mut K::Container,
        values: &mut V::Container,
        compute_hash_fn: HashFnWithSeed<K>,
    ) -> Self
    where
        K: ZeroMapKV<'a> + 'b + ?Sized + Hash,
        V: ZeroMapKV<'a> + ?Sized,
    {
        let len = keys.zvl_len();
        let mut bucket_sizes = vec![0; len];
        let mut bucket_flatten = Vec::with_capacity(len);
        for i in 0..len {
            let h = K::Container::zvl_get_as_t(keys.zvl_get(i).unwrap(), |k| {
                (compute_hash_fn(k, 0x00) % len as u64) as u32
            });
            bucket_sizes[h as usize] += 1;
            bucket_flatten.push((h, i));
        }
        bucket_flatten.sort_by(|&(ha, _), &(hb, _)| {
            (bucket_sizes[hb as usize], hb).cmp(&(bucket_sizes[ha as usize], ha))
        });

        let mut generation = 0;
        let mut occupied = vec![false; len];
        let mut assignments = vec![0; len];
        let mut current_displacements = Vec::with_capacity(16);
        let mut seeds = vec![0; len];
        let mut reverse_mapping = vec![0; len];

        let mut start = 0;
        while start < len {
            let g = bucket_flatten[start].0 as usize;
            let end = start + bucket_sizes[g];
            let buckets = &bucket_flatten[start..end];

            'seed: for seed in 1u32.. {
                current_displacements.clear();
                generation += 1;

                for (_, i) in buckets {
                    let displacement_idx =
                        K::Container::zvl_get_as_t(keys.zvl_get(*i).unwrap(), |k| {
                            (compute_hash_fn(k, seed) % len as u64) as u32
                        }) as usize;
                    if occupied[displacement_idx] || assignments[displacement_idx] == generation {
                        continue 'seed;
                    }
                    assignments[displacement_idx] = generation;

                    current_displacements.push(displacement_idx);
                }
                seeds[g] = seed;
                for (i, displacement_idx) in current_displacements.iter().enumerate() {
                    // `current_displacements` has same size as `buckets`
                    let (_, idx) = &buckets[i];

                    // displacement_idx is always within bounds
                    occupied[*displacement_idx] = true;
                    reverse_mapping[*displacement_idx] = *idx;
                }
                break;
            }

            start = end;
        }

        keys.zvl_permute(&mut reverse_mapping.clone());
        values.zvl_permute(&mut reverse_mapping);

        Self {
            seeds: ZeroVec::alloc_from_slice(&seeds),
        }
    }

    #[inline]
    pub fn index<'b, K, A>(&'b self, k: A, compute_hash_fn: HashFnWithSeed<K>) -> Option<usize>
    where
        K: Hash + ?Sized,
        A: Borrow<K>,
    {
        let l1 = (compute_hash_fn(k.borrow(), 0x00) % self.seeds.len() as u64) as u32;
        let seed = self.seeds.get(l1 as usize).unwrap();
        if seed == 0 {
            None
        } else {
            Some((compute_hash_fn(k.borrow(), seed) % self.seeds.len() as u64) as u32 as usize)
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PAZeroHashMapStatic<'a, K, V>
where
    K: ZeroMapKV<'a> + ?Sized,
    V: ZeroMapKV<'a> + ?Sized,
{
    #[cfg_attr(feature = "serde", serde(borrow))]
    index: PAHashIndex<'a>,
    keys: K::Container,
    values: V::Container,
}

impl<'a, K, V> PAZeroHashMapStatic<'a, K, V>
where
    K: ZeroMapKV<'a> + ?Sized + Hash + Eq,
    V: ZeroMapKV<'a> + ?Sized,
{
    #[inline]
    pub fn get<'b, A>(
        &'b self,
        kb: A,
        splitter: Split3Fn,
        compute_hash_fn: HashFn<K>,
    ) -> Option<&'b V::GetType>
    where
        A: Borrow<K>,
    {
        let k = kb.borrow();
        let i = self.index.index(k, splitter, compute_hash_fn)?;
        #[allow(clippy::unwrap_used)] // i is in 0..self.keys.len()
        let found = self.keys.zvl_get(i).unwrap();
        if K::Container::zvl_get_as_t(found, |found| found == k) {
            self.values.zvl_get(i)
        } else {
            None
        }
    }

    pub fn build_from_iter<A, B, I>(iter: I, splitter: Split3Fn, compute_hash_fn: HashFn<K>) -> Self
    where
        A: Borrow<K>,
        B: Borrow<V>,
        I: Iterator<Item = (A, B)>,
    {
        let size_hint = match iter.size_hint() {
            (_, Some(upper)) => upper,
            (lower, None) => lower,
        };

        let mut keys = K::Container::zvl_with_capacity(size_hint);
        let mut values = V::Container::zvl_with_capacity(size_hint);
        for (k, v) in iter {
            keys.zvl_push(k.borrow());
            values.zvl_push(v.borrow());
        }
        let index = PAHashIndex::build_from_kv_containers_with_split3_with_hf::<K, V>(
            &mut keys,
            &mut values,
            splitter,
            compute_hash_fn,
        );
        Self {
            index,
            values,
            keys,
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GAZeroHashMapStatic<'a, K, V>
where
    K: ZeroMapKV<'a> + ?Sized,
    V: ZeroMapKV<'a> + ?Sized,
{
    #[cfg_attr(feature = "serde", serde(borrow))]
    index: GAHashIndex<'a>,
    keys: K::Container,
    values: V::Container,
}

impl<'a, K, V> GAZeroHashMapStatic<'a, K, V>
where
    K: ZeroMapKV<'a> + ?Sized + Hash + Eq,
    V: ZeroMapKV<'a> + ?Sized,
{
    #[inline]
    pub fn get<'b, A>(&'b self, kb: A, compute_hash_fn: HashFnWithSeed<K>) -> Option<&'b V::GetType>
    where
        A: Borrow<K>,
    {
        let k = kb.borrow();
        let i = self.index.index(k, compute_hash_fn)?;
        #[allow(clippy::unwrap_used)] // i is in 0..self.keys.len()
        let found = self.keys.zvl_get(i).unwrap();
        if K::Container::zvl_get_as_t(found, |found| found == k) {
            self.values.zvl_get(i)
        } else {
            None
        }
    }

    pub fn build_from_iter<A, B, I>(iter: I, compute_hash_fn: HashFnWithSeed<K>) -> Self
    where
        A: Borrow<K>,
        B: Borrow<V>,
        I: Iterator<Item = (A, B)>,
    {
        let size_hint = match iter.size_hint() {
            (_, Some(upper)) => upper,
            (lower, None) => lower,
        };

        let mut keys = K::Container::zvl_with_capacity(size_hint);
        let mut values = V::Container::zvl_with_capacity(size_hint);
        for (k, v) in iter {
            keys.zvl_push(k.borrow());
            values.zvl_push(v.borrow());
        }
        let index = GAHashIndex::build_from_kv_containers_with_hf::<K, V>(
            &mut keys,
            &mut values,
            compute_hash_fn,
        );
        Self {
            index,
            values,
            keys,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ule::AsULE;
    use core::hash::Hasher;
    use rand::{distributions::Standard, Rng, SeedableRng};
    use rand_pcg::Lcg64Xsh32;
    use wyhash::WyHash;

    #[test]
    fn test_zhms_u64k_u64v() {
        const N: usize = 1 << 15;
        let seed = u64::from_le_bytes(*b"testseed");
        let rng = Lcg64Xsh32::seed_from_u64(seed);
        let kv: Vec<(u64, u64)> = rng.sample_iter(&Standard).take(N).collect();

        let splitter = |hash: u64, m: u32| {
            (
                ((hash >> 48) as u32 % m) as usize,
                (hash >> 24) as u32 & 0xffffff,
                ((hash & 0xffffff) as u32),
            )
        };

        let hash_fn = |k: &u64| -> u64 {
            let mut hasher = WyHash::with_seed(0);
            k.hash(&mut hasher);
            hasher.finish()
        };

        let hash_fn_seed = |k: &u64, seed: u32| -> u64 {
            let mut hasher = WyHash::with_seed(seed.into());
            k.hash(&mut hasher);
            hasher.finish()
        };

        let pahm: PAZeroHashMapStatic<u64, u64> = PAZeroHashMapStatic::build_from_iter(
            kv.iter().map(|e| (&e.0, &e.1)),
            splitter,
            hash_fn,
        );

        let gahm: GAZeroHashMapStatic<u64, u64> =
            GAZeroHashMapStatic::build_from_iter(kv.iter().map(|e| (&e.0, &e.1)), hash_fn_seed);

        for (k, v) in kv.clone() {
            assert_eq!(
                pahm.get(&k, splitter, hash_fn)
                    .copied()
                    .map(<u64 as AsULE>::from_unaligned),
                Some(v),
            );
        }

        for (k, v) in kv {
            assert_eq!(
                gahm.get(&k, hash_fn_seed)
                    .copied()
                    .map(<u64 as AsULE>::from_unaligned),
                Some(v),
            );
        }
    }
}
