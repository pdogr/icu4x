// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

pub use icu_datagen::options::*;

use icu_provider::prelude::*;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Config {
    #[serde(default, skip_serializing_if = "is_default")]
    pub keys: KeyInclude,
    #[serde(default, skip_serializing_if = "is_default")]
    pub locales: LocaleInclude,
    pub cldr: PathOrTag,
    pub icu_export: PathOrTag,
    pub segmenter_lstm: PathOrTag,
    #[serde(default, skip_serializing_if = "is_default")]
    pub trie_type: TrieType,
    #[serde(default, skip_serializing_if = "is_default")]
    pub collation_han_database: CollationHanDatabase,
    #[serde(default, skip_serializing_if = "is_default")]
    pub collations: HashSet<String>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub segmenter_models: SegmenterModelInclude,
    pub export: Export,
    #[serde(default, skip_serializing_if = "is_default")]
    pub fallback: FallbackMode,
    #[serde(default, skip_serializing_if = "is_default")]
    pub overwrite: bool,
}

fn is_default<T: Default + PartialEq>(value: &T) -> bool {
    value == &T::default()
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum KeyInclude {
    None,
    All,
    Explicit(#[serde(with = "data_key_as_str")] HashSet<DataKey>),
    ForBinary(PathBuf),
}

impl Default for KeyInclude {
    fn default() -> Self {
        Self::All
    }
}

mod data_key_as_str {
    use super::*;
    use serde::{de::*, ser::*};
    use std::borrow::Cow;

    pub fn serialize<S: Serializer>(selff: &HashSet<DataKey>, ser: S) -> Result<S::Ok, S::Error> {
        selff
            .iter()
            .map(|k| k.path().get())
            .collect::<HashSet<_>>()
            .serialize(ser)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(de: D) -> Result<HashSet<DataKey>, D::Error> {
        HashSet::<Cow<'de, str>>::deserialize(de)?
            .into_iter()
            .map(|s| icu_datagen::key(&s).ok_or(s))
            .collect::<Result<_, _>>()
            .map_err(|s| D::Error::custom(format!("Unknown key {s}")))
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum PathOrTag {
    Path(PathBuf),
    Tag(String),
    Latest,
    None,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum Export {
    Fs {
        path: PathBuf,
        syntax: FsSyntax,
        #[serde(default, skip_serializing_if = "is_default")]
        fingerprint: bool,
    },
    Blob {
        path: PathBuf,
    },
    Baked {
        path: PathBuf,
        #[serde(default, skip_serializing_if = "is_default")]
        pretty: bool,
        #[serde(default, skip_serializing_if = "is_default")]
        insert_feature_gates: bool,
        #[serde(default, skip_serializing_if = "is_default")]
        use_meta_crate: bool,
    },
}

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum FsSyntax {
    Postcard,
    Json,
    Bincode,
    JsonPretty,
}
