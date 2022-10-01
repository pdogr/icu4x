// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]
use std::fs;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fxhash::FxHasher64;
use highway::PortableHash;
use seahash::SeaHasher;
use t1ha::T1haHasher;
use xxhash_rust::xxh3::{xxh3_64, Xxh3Builder};
use xxhash_rust::xxh64::Xxh64Builder;

use core::hash::{Hash, Hasher};
use lazy_static::lazy_static;
use paste::paste;
use wyhash::WyHash;
use zerovec::maps::ZeroMapKV;
use zerovec::vecs::{Index32, VarZeroSlice, VarZeroVec};
use zerovec::{GAZeroHashMapStatic, HashFn, HashFnWithSeed, PAZeroHashMapStatic, Split3Fn};

const DATA: [(&str, &str); 16] = [
    ("ar", "Arabic"),
    ("bn", "Bangla"),
    ("ccp", "Chakma"),
    ("chr", "Cherokee"),
    ("el", "Greek"),
    ("en", "English"),
    ("eo", "Esperanto"),
    ("es", "Spanish"),
    ("fr", "French"),
    ("iu", "Inuktitut"),
    ("ja", "Japanese"),
    ("ru", "Russian"),
    ("sr", "Serbian"),
    ("th", "Thai"),
    ("tr", "Turkish"),
    ("zh", "Chinese"),
];

lazy_static! {
    static ref LIKELY_SUBTAGS_DATA: Vec<(String, String)> = {
        let data = fs::read_to_string("benches/testdata/likelySubtags.json").expect("Open file");
        let json: serde_json::Value = serde_json::from_str(&data).expect("Unable to deserialize.");
        let map = json.as_object().expect("Expected a map.");
        map.into_iter()
            .map(|kv| (kv.0.clone(), kv.1.as_str().unwrap().to_string()))
            .collect()
    };
}

/*
/// Run this function to print new data to the console. Requires the optional `serde` feature.
#[allow(dead_code)]
fn generate_zerohashmap() {
    let map = build_zerohashmap(false);
    let buf = postcard::to_stdvec(&map).unwrap();
    println!("{:?}", buf);
}
*/

#[inline(always)]
fn build_likely_subtags_data() -> Vec<(String, String)> {
    let mut kv = Vec::with_capacity(2000);
    for (key, value) in LIKELY_SUBTAGS_DATA.iter() {
        kv.push((key.to_string(), value.to_string()));
    }
    kv
}

#[inline(always)]
fn build_data(large: bool) -> Vec<(String, String)> {
    let mut kv = match large {
        true => Vec::with_capacity(8192 * DATA.len()),
        false => Vec::with_capacity(DATA.len()),
    };

    for (key, value) in DATA.iter() {
        if large {
            for n in 0..8192 {
                kv.push((format!("{}{}", key, n), value.to_string()));
            }
        } else {
            kv.push((key.to_string(), value.to_string()));
        }
    }
    kv
}

fn sanity_test(
    kv: &[(String, String)],
    hm: &PAZeroHashMapStatic<Index32Str, Index32Str>,
    split_fn: Split3Fn,
    hash_fn: HashFn<Index32Str>,
) {
    for (k, v) in kv.iter() {
        assert_eq!(
            hm.get(indexify(k), split_fn, hash_fn).map(|x| &x.0),
            Some(v.as_ref())
        );
    }
}

macro_rules! lookup_benchmark_ga {
    ($hash_fn: ident) => {
        paste! {
            fn [<ga_ $hash_fn _lookup_small>](c: &mut Criterion) {
                let kv = black_box(build_data(false));
                let hm:GAZeroHashMapStatic<Index32Str,Index32Str> =
                    GAZeroHashMapStatic::build_from_iter(
                        kv.iter().map(|kv| (indexify(&kv.0), indexify(&kv.1))),
                        $hash_fn
                    );
                for (k, v) in kv.iter() {
                    assert_eq!(
                        hm.get(indexify(k), $hash_fn).map(|x| &x.0),
                        Some(v.as_ref())
                    );
                }
                c.bench_function(concat!("zhm/lookup/small/ga/", stringify!($hash_fn)),
                    |b| {
                        b.iter(|| {
                            assert_eq!(
                                hm.get(black_box(indexify("iu")), $hash_fn).map(|x| &x.0),
                                Some("Inuktitut")
                            );
                            assert_eq!(
                                hm.get(black_box(indexify("zz")),
                                $hash_fn).map(|x| &x.0),
                                None
                            );
                        });
                    });
            }

            fn [<ga_ $hash_fn _ lookup_likely_subtags>](c: &mut Criterion) {
                let kv = black_box(build_likely_subtags_data());
                let hm:GAZeroHashMapStatic<Index32Str,Index32Str> =
                    GAZeroHashMapStatic::build_from_iter(
                        kv.iter().map(|kv| (indexify(&kv.0), indexify(&kv.1))),
                        $hash_fn
                    );
                for (k, v) in kv.iter() {
                    assert_eq!(
                        hm.get(indexify(k), $hash_fn).map(|x| &x.0),
                        Some(v.as_ref())
                    );
                }

                c.bench_function(concat!("zhm/lookup/likelySubtags/ga/", stringify!($hash_fn)),
                    |b| {
                        b.iter(|| {
                            assert_eq!(
                                hm.get(black_box(indexify("mn-Mong")),  $hash_fn).map(|x| &x.0),
                                Some("mn-Mong-CN")
                            );
                            assert_eq!(
                                hm.get(black_box(indexify("zz")),
                                $hash_fn).map(|x| &x.0),
                                None
                            );
                        });
                    });
            }

            fn [<ga_ $hash_fn  _lookup_large>](c: &mut Criterion) {
                let kv = black_box(build_data(true));
                let hm:GAZeroHashMapStatic<Index32Str,Index32Str> =
                    GAZeroHashMapStatic::build_from_iter(
                        kv.iter().map(|kv| (indexify(&kv.0), indexify(&kv.1))),
                        $hash_fn
                    );
                for (k, v) in kv.iter() {
                    assert_eq!(
                        hm.get(indexify(k), $hash_fn).map(|x| &x.0),
                        Some(v.as_ref())
                    );
                }
                c.bench_function(concat!("zhm/lookup/large/ga/",stringify!($hash_fn)),
                    |b| {
                        b.iter(|| {
                            assert_eq!(
                                hm.get(black_box(indexify("iu3333")), $hash_fn).map(|x| &x.0),
                                Some("Inuktitut")
                            );
                            assert_eq!(
                                hm.get(black_box(indexify("zz")),
                                $hash_fn).map(|x| &x.0),
                                None
                            );
                        });
                    });
            }
        }
    };
}

macro_rules! lookup_benchmark_pa {
    ($hash_fn: ident, $split_fn: ident) => {
        paste! {
            fn [<pa_ $hash_fn _ $split_fn _lookup_small>](c: &mut Criterion) {
                let kv = black_box(build_data(false));
                let hm:PAZeroHashMapStatic<Index32Str,Index32Str> =
                    PAZeroHashMapStatic::build_from_iter(
                        kv.iter().map(|kv| (indexify(&kv.0), indexify(&kv.1))),
                        $split_fn,
                        $hash_fn
                    );
                sanity_test(&kv, &hm, $split_fn, $hash_fn);
                c.bench_function(concat!("zhm/lookup/small/pa/",stringify!($split_fn),"/", stringify!($hash_fn)),
                    |b| {
                        b.iter(|| {
                            assert_eq!(
                                hm.get(black_box(indexify("iu")), $split_fn, $hash_fn).map(|x| &x.0),
                                Some("Inuktitut")
                            );
                            assert_eq!(
                                hm.get(black_box(indexify("zz")),
                                $split_fn,
                                $hash_fn).map(|x| &x.0),
                                None
                            );
                        });
                    });
            }

            fn [<pa_ $hash_fn _ $split_fn _lookup_likely_subtags>](c: &mut Criterion) {
                let kv = black_box(build_likely_subtags_data());
                let hm:PAZeroHashMapStatic<Index32Str,Index32Str> =
                    PAZeroHashMapStatic::build_from_iter(
                        kv.iter().map(|kv| (indexify(&kv.0), indexify(&kv.1))),
                        $split_fn,
                        $hash_fn
                    );
                sanity_test(&kv, &hm, $split_fn, $hash_fn);
                c.bench_function(concat!("zhm/lookup/likelySubtags/pa/",stringify!($split_fn),"/", stringify!($hash_fn)),
                    |b| {
                        b.iter(|| {
                            assert_eq!(
                                hm.get(black_box(indexify("mn-Mong")), $split_fn, $hash_fn).map(|x| &x.0),
                                Some("mn-Mong-CN")
                            );
                            assert_eq!(
                                hm.get(black_box(indexify("zz")),
                                $split_fn,
                                $hash_fn).map(|x| &x.0),
                                None
                            );
                        });
                    });
            }

            fn [<pa_ $hash_fn _ $split_fn _lookup_large>](c: &mut Criterion) {
                let kv = black_box(build_data(true));
                let hm:PAZeroHashMapStatic<Index32Str,Index32Str> =
                    PAZeroHashMapStatic::build_from_iter(
                        kv.iter().map(|kv| (indexify(&kv.0), indexify(&kv.1))),
                        $split_fn,
                        $hash_fn
                    );
                sanity_test(&kv, &hm, $split_fn, $hash_fn);
                c.bench_function(concat!("zhm/lookup/large/pa/",stringify!($split_fn),"/", stringify!($hash_fn)),
                    |b| {
                        b.iter(|| {
                            assert_eq!(
                                hm.get(black_box(indexify("iu3333")), $split_fn, $hash_fn).map(|x| &x.0),
                                Some("Inuktitut")
                            );
                            assert_eq!(
                                hm.get(black_box(indexify("zz")),
                                $split_fn,
                                $hash_fn).map(|x| &x.0),
                                None
                            );
                        });
                    });
            }
        }
    };
}

const SplitD: Split3Fn = |hash: u64, m: u32| {
    (
        ((hash >> 48) as u32 % m) as usize,
        (hash >> 24) as u32 & 0xffffff,
        ((hash & 0xffffff) as u32),
    )
};

const wyhash: HashFn<Index32Str> = |k: &Index32Str| -> u64 { wyhash_seed(k, 0x00) };
const t1ha: HashFn<Index32Str> = |k: &Index32Str| -> u64 { t1ha_seed(k, 0) };
const fxhash: HashFn<Index32Str> = |k: &Index32Str| -> u64 { fxhash_seed(k, 0) };
const highwayhash: HashFn<Index32Str> = |k: &Index32Str| -> u64 { highwayhash_seed(k, 0) };
const seahash: HashFn<Index32Str> = |k: &Index32Str| -> u64 { seahash_seed(k, 0) };
const xxh3hash: HashFn<Index32Str> = |k: &Index32Str| -> u64 { xxh3hash_seed(k, 0) };
const xx64hash: HashFn<Index32Str> = |k: &Index32Str| -> u64 { xx64hash_seed(k, 0) };

const wyhash_seed: HashFnWithSeed<Index32Str> = |k: &Index32Str, seed: u32| -> u64 {
    let mut hasher = WyHash::with_seed(seed.into());
    k.hash(&mut hasher);
    hasher.finish()
};

const t1ha_seed: HashFnWithSeed<Index32Str> = |k: &Index32Str, seed: u32| -> u64 {
    let mut hasher = T1haHasher::with_seed(seed.into());
    k.hash(&mut hasher);
    hasher.finish()
};

const fxhash_seed: HashFnWithSeed<Index32Str> = |k: &Index32Str, seed: u32| -> u64 {
    let mut hasher = FxHasher64::default();
    seed.hash(&mut hasher);
    k.hash(&mut hasher);
    hasher.finish()
};

const highwayhash_seed: HashFnWithSeed<Index32Str> = |k: &Index32Str, seed: u32| -> u64 {
    let mut hasher = PortableHash::new(highway::Key([seed.into(), 0, 0, 0]));
    k.hash(&mut hasher);
    hasher.finish()
};

const xxh3hash_seed: HashFnWithSeed<Index32Str> = |k: &Index32Str, seed: u32| -> u64 {
    let mut hasher = Xxh3Builder::new().with_seed(seed.into()).build();
    k.hash(&mut hasher);
    hasher.finish()
};

const xx64hash_seed: HashFnWithSeed<Index32Str> = |k: &Index32Str, seed: u32| -> u64 {
    let mut hasher = Xxh64Builder::new(seed.into()).build();
    k.hash(&mut hasher);
    hasher.finish()
};

const seahash_seed: HashFnWithSeed<Index32Str> = |k: &Index32Str, seed: u32| -> u64 {
    let mut hasher = SeaHasher::with_seeds(0x00, 0x00, 0x00, seed.into());
    k.hash(&mut hasher);
    hasher.finish()
};

lookup_benchmark_pa!(wyhash, SplitD);
lookup_benchmark_pa!(t1ha, SplitD);
lookup_benchmark_pa!(fxhash, SplitD);
lookup_benchmark_pa!(highwayhash, SplitD);
lookup_benchmark_pa!(xxh3hash, SplitD);
lookup_benchmark_pa!(xx64hash, SplitD);
lookup_benchmark_pa!(seahash, SplitD);

/*
fn read_large_zerohashmap_postcard_bytes() -> Vec<u8> {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/benches/testdata/large_zerohashmap.postcard"
    );
    fs::read(path).unwrap()
}
*/

criterion_group!(
    pa_benches,
    pa_wyhash_SplitD_lookup_small,
    pa_t1ha_SplitD_lookup_small,
    pa_fxhash_SplitD_lookup_small,
    pa_highwayhash_SplitD_lookup_small,
    pa_xxh3hash_SplitD_lookup_small,
    pa_xx64hash_SplitD_lookup_small,
    pa_seahash_SplitD_lookup_small,
    pa_wyhash_SplitD_lookup_large,
    pa_t1ha_SplitD_lookup_large,
    pa_fxhash_SplitD_lookup_large,
    pa_highwayhash_SplitD_lookup_large,
    pa_xxh3hash_SplitD_lookup_large,
    pa_xx64hash_SplitD_lookup_large,
    pa_seahash_SplitD_lookup_large,
    pa_wyhash_SplitD_lookup_likely_subtags,
    pa_t1ha_SplitD_lookup_likely_subtags,
    pa_fxhash_SplitD_lookup_likely_subtags,
    pa_highwayhash_SplitD_lookup_likely_subtags,
    pa_xxh3hash_SplitD_lookup_likely_subtags,
    pa_xx64hash_SplitD_lookup_likely_subtags,
    pa_seahash_SplitD_lookup_likely_subtags,
);

lookup_benchmark_ga!(wyhash_seed);
lookup_benchmark_ga!(t1ha_seed);
lookup_benchmark_ga!(fxhash_seed);
lookup_benchmark_ga!(highwayhash_seed);
lookup_benchmark_ga!(xxh3hash_seed);
lookup_benchmark_ga!(xx64hash_seed);
lookup_benchmark_ga!(seahash_seed);

criterion_group!(
    ga_benches,
    ga_wyhash_seed_lookup_small,
    ga_t1ha_seed_lookup_small,
    ga_fxhash_seed_lookup_small,
    ga_highwayhash_seed_lookup_small,
    ga_xxh3hash_seed_lookup_small,
    ga_seahash_seed_lookup_small,
    ga_xx64hash_seed_lookup_small,
    ga_wyhash_seed_lookup_large,
    ga_t1ha_seed_lookup_large,
    // ga_fxhash_seed_lookup_large,
    ga_highwayhash_seed_lookup_large,
    ga_xxh3hash_seed_lookup_large,
    ga_xx64hash_seed_lookup_large,
    ga_seahash_seed_lookup_large,
    ga_wyhash_seed_lookup_likely_subtags,
    ga_t1ha_seed_lookup_likely_subtags,
    ga_fxhash_seed_lookup_likely_subtags,
    ga_highwayhash_seed_lookup_likely_subtags,
    ga_xxh3hash_seed_lookup_likely_subtags,
    ga_seahash_seed_lookup_likely_subtags,
    ga_xx64hash_seed_lookup_likely_subtags,
);

criterion_main!(pa_benches, ga_benches);

#[zerovec::make_varule(Index32Str)]
#[zerovec::skip_derive(ZeroMapKV)]
#[derive(Eq, PartialEq, Ord, PartialOrd, serde::Serialize, serde::Deserialize)]
#[zerovec::derive(Serialize, Deserialize, Hash)]
pub(crate) struct Index32StrBorrowed<'a>(#[serde(borrow)] pub &'a str);

impl<'a> ZeroMapKV<'a> for Index32Str {
    type Container = VarZeroVec<'a, Index32Str, Index32>;
    type Slice = VarZeroSlice<Index32Str, Index32>;
    type GetType = Index32Str;
    type OwnedType = Box<Index32Str>;
}

#[inline]
fn indexify(s: &str) -> &Index32Str {
    unsafe { ::core::mem::transmute(s) }
}
