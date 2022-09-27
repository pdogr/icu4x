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
use paste::paste;
use wyhash::WyHash;
use zerovec::maps::ZeroMapKV;
use zerovec::vecs::{Index32, VarZeroSlice, VarZeroVec};
use zerovec::{HashFn, PAZeroHashMapStatic, Split3Fn};

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

macro_rules! lookup_benchmark_pa {
    ($name: ident, $hash_fn: ident, $split_fn: ident) => {
        paste! {
            fn [<$hash_fn _ $split_fn _lookup_small>](c: &mut Criterion) {
                let kv = black_box(build_data(false));
                let hm:PAZeroHashMapStatic<Index32Str,Index32Str> =
                    PAZeroHashMapStatic::build_from_iter(
                        kv.iter().map(|kv| (indexify(&kv.0), indexify(&kv.1))),
                        $split_fn,
                        $hash_fn
                    );
                sanity_test(&kv, &hm, $split_fn, $hash_fn);
                c.bench_function(concat!("zhms/lookup/small/",stringify!($split_fn),"/", stringify!($hash_fn)),
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
            fn [<$hash_fn _ $split_fn _lookup_large>](c: &mut Criterion) {
                let kv = black_box(build_data(true));
                let hm:PAZeroHashMapStatic<Index32Str,Index32Str> =
                    PAZeroHashMapStatic::build_from_iter(
                        kv.iter().map(|kv| (indexify(&kv.0), indexify(&kv.1))),
                        $split_fn,
                        $hash_fn
                    );
                sanity_test(&kv, &hm, $split_fn, $hash_fn);
                c.bench_function(concat!("zhms/lookup/large/",stringify!($split_fn),"/", stringify!($hash_fn)),
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

const S_16_24_24: Split3Fn = |hash: u64, m: u32| {
    (
        ((hash >> 48) as u32 % m) as usize,
        (hash >> 24) as u32 & 0xffffff,
        ((hash & 0xffffff) as u32),
    )
};

const wyhash: HashFn<Index32Str> = |k: &Index32Str| -> u64 {
    let mut hasher = WyHash::with_seed(0);
    k.hash(&mut hasher);
    hasher.finish()
};

const t1ha: HashFn<Index32Str> = |k: &Index32Str| -> u64 {
    let mut hasher = T1haHasher::with_seed(0);
    k.hash(&mut hasher);
    hasher.finish()
};

const fxhash: HashFn<Index32Str> = |k: &Index32Str| -> u64 {
    let mut hasher = FxHasher64::default();
    k.hash(&mut hasher);
    hasher.finish()
};

const highwayhash: HashFn<Index32Str> = |k: &Index32Str| -> u64 {
    let mut hasher = PortableHash::new(highway::Key([0, 0, 0, 0]));
    k.hash(&mut hasher);
    hasher.finish()
};

const xxh3hash: HashFn<Index32Str> = |k: &Index32Str| -> u64 {
    let mut hasher = Xxh3Builder::new().with_seed(0x00).build();
    k.hash(&mut hasher);
    hasher.finish()
};

const xx64hash: HashFn<Index32Str> = |k: &Index32Str| -> u64 {
    let mut hasher = Xxh64Builder::new(0x00).build();
    k.hash(&mut hasher);
    hasher.finish()
};

const seahash: HashFn<Index32Str> = |k: &Index32Str| -> u64 {
    let mut hasher = SeaHasher::with_seeds(0x00, 0x00, 0x00, 0x00);
    k.hash(&mut hasher);
    hasher.finish()
};

lookup_benchmark_pa!(test, wyhash, S_16_24_24);
lookup_benchmark_pa!(test, t1ha, S_16_24_24);
lookup_benchmark_pa!(test, fxhash, S_16_24_24);
lookup_benchmark_pa!(test, highwayhash, S_16_24_24);
lookup_benchmark_pa!(test, xxh3hash, S_16_24_24);
lookup_benchmark_pa!(test, xx64hash, S_16_24_24);
lookup_benchmark_pa!(test, seahash, S_16_24_24);

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
    benches,
    wyhash_S_16_24_24_lookup_small,
    wyhash_S_16_24_24_lookup_large,
    t1ha_S_16_24_24_lookup_small,
    t1ha_S_16_24_24_lookup_large,
    fxhash_S_16_24_24_lookup_small,
    fxhash_S_16_24_24_lookup_large,
    highwayhash_S_16_24_24_lookup_small,
    highwayhash_S_16_24_24_lookup_large,
    xxh3hash_S_16_24_24_lookup_small,
    xxh3hash_S_16_24_24_lookup_large,
    xx64hash_S_16_24_24_lookup_small,
    xx64hash_S_16_24_24_lookup_large,
    seahash_S_16_24_24_lookup_small,
    seahash_S_16_24_24_lookup_large
);
criterion_main!(benches);

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
