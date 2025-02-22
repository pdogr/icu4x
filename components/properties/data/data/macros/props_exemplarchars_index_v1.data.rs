// @generated
/// Implement `DataProvider<ExemplarCharactersIndexV1Marker>` on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_props_exemplarchars_index_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.65"]
        impl icu_provider::DataProvider<icu_properties::provider::ExemplarCharactersIndexV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_properties::provider::ExemplarCharactersIndexV1Marker>, icu_provider::DataError> {
                static PS: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\x06\0\0#\x06\0\0'\x06\0\0)\x06\0\0*\x06\0\0;\x06\0\0A\x06\0\0C\x06\0\0D\x06\0\0I\x06\0\0|\x06\0\0}\x06\0\0~\x06\0\0\x7F\x06\0\0\x81\x06\0\0\x82\x06\0\0\x85\x06\0\0\x87\x06\0\0\x89\x06\0\0\x8A\x06\0\0\x93\x06\0\0\x94\x06\0\0\x96\x06\0\0\x97\x06\0\0\x98\x06\0\0\x99\x06\0\0\x9A\x06\0\0\x9B\x06\0\0\xA9\x06\0\0\xAA\x06\0\0\xAB\x06\0\0\xAC\x06\0\0\xBC\x06\0\0\xBD\x06\0\0\xCC\x06\0\0\xCD\x06\0\0") }, 42u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static UR: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"!\x06\0\0\"\x06\0\0'\x06\0\0)\x06\0\0*\x06\0\0;\x06\0\0A\x06\0\0C\x06\0\0D\x06\0\0G\x06\0\0H\x06\0\0I\x06\0\0y\x06\0\0z\x06\0\0~\x06\0\0\x7F\x06\0\0\x86\x06\0\0\x87\x06\0\0\x88\x06\0\0\x89\x06\0\0\x91\x06\0\0\x92\x06\0\0\x98\x06\0\0\x99\x06\0\0\xA9\x06\0\0\xAA\x06\0\0\xAF\x06\0\0\xB0\x06\0\0\xBE\x06\0\0\xBF\x06\0\0\xC1\x06\0\0\xC2\x06\0\0\xCC\x06\0\0\xCD\x06\0\0\xD2\x06\0\0\xD3\x06\0\0") }, 38u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static SD: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"'\x06\0\0)\x06\0\0*\x06\0\0;\x06\0\0A\x06\0\0C\x06\0\0D\x06\0\0I\x06\0\0J\x06\0\0K\x06\0\0z\x06\0\0|\x06\0\0}\x06\0\0\x81\x06\0\0\x83\x06\0\0\x85\x06\0\0\x86\x06\0\0\x88\x06\0\0\x8A\x06\0\0\x8B\x06\0\0\x8C\x06\0\0\x8E\x06\0\0\x8F\x06\0\0\x90\x06\0\0\x99\x06\0\0\x9A\x06\0\0\xA6\x06\0\0\xA7\x06\0\0\xA9\x06\0\0\xAB\x06\0\0\xAF\x06\0\0\xB0\x06\0\0\xB1\x06\0\0\xB2\x06\0\0\xB3\x06\0\0\xB4\x06\0\0\xBB\x06\0\0\xBC\x06\0\0\xBE\x06\0\0\xBF\x06\0\0") }, 50u32)
                    },
                    unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x04\0\xD8\xAC\xDA\xBE\xDA\xAF\xDA\xBE") },
                ));
                static AR: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"'\x06\0\0)\x06\0\0*\x06\0\0;\x06\0\0A\x06\0\0I\x06\0\0J\x06\0\0K\x06\0\0") }, 28u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static KO: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"11\0\x0021\0\x0041\0\x0051\0\x0071\0\081\0\091\0\0:1\0\0A1\0\0C1\0\0E1\0\0F1\0\0G1\0\0I1\0\0J1\0\0O1\0\0") }, 14u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static HY: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"1\x05\0\0W\x05\0\0") }, 38u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static MI: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"A\0\0\0B\0\0\0E\0\0\0F\0\0\0H\0\0\0J\0\0\0K\0\0\0L\0\0\0M\0\0\0Q\0\0\0R\0\0\0S\0\0\0T\0\0\0V\0\0\0W\0\0\0X\0\0\0") }, 13u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static TO: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"A\0\0\0B\0\0\0E\0\0\0G\0\0\0H\0\0\0J\0\0\0K\0\0\0Q\0\0\0S\0\0\0W\0\0\0\xBB\x02\0\0\xBC\x02\0\0") }, 16u32)
                    },
                    unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0NG") },
                ));
                static KGP: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"A\0\0\0B\0\0\0E\0\0\0L\0\0\0M\0\0\0Q\0\0\0R\0\0\0W\0\0\0Y\0\0\0Z\0\0\0") }, 18u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static QU: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"A\0\0\0B\0\0\0H\0\0\0J\0\0\0K\0\0\0O\0\0\0P\0\0\0R\0\0\0S\0\0\0V\0\0\0W\0\0\0X\0\0\0Y\0\0\0Z\0\0\0\xD1\0\0\0\xD2\0\0\0") }, 15u32)
                    },
                    unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x02\0ChLl") },
                ));
                static YRL: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"A\0\0\0C\0\0\0D\0\0\0F\0\0\0G\0\0\0H\0\0\0I\0\0\0J\0\0\0K\0\0\0L\0\0\0M\0\0\0O\0\0\0P\0\0\0Q\0\0\0R\0\0\0V\0\0\0W\0\0\0Z\0\0\0") }, 17u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static YO: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"A\0\0\0C\0\0\0D\0\0\0Q\0\0\0R\0\0\0V\0\0\0W\0\0\0X\0\0\0Y\0\0\0Z\0\0\0") }, 21u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static TK: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"A\0\0\0C\0\0\0D\0\0\0Q\0\0\0R\0\0\0V\0\0\0W\0\0\0X\0\0\0Y\0\0\0[\0\0\0\xC4\0\0\0\xC5\0\0\0\xC7\0\0\0\xC8\0\0\0\xD6\0\0\0\xD7\0\0\0\xDC\0\0\0\xDE\0\0\0G\x01\0\0H\x01\0\0^\x01\0\0_\x01\0\0}\x01\0\0~\x01\0\0") }, 30u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static KEA: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"A\0\0\0C\0\0\0D\0\0\0Q\0\0\0R\0\0\0W\0\0\0X\0\0\0Y\0\0\0Z\0\0\0[\0\0\0") }, 22u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static PCM: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"A\0\0\0C\0\0\0D\0\0\0Q\0\0\0R\0\0\0X\0\0\0Y\0\0\0[\0\0\0") }, 23u32)
                    },
                    unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0CH") },
                ));
                static UZ: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"A\0\0\0C\0\0\0D\0\0\0W\0\0\0X\0\0\0[\0\0\0") }, 24u32)
                    },
                    unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\x02\0\x05\0\x08\0ChG\xCA\xBBO\xCA\xBBSh") },
                ));
                static JV: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"A\0\0\0F\0\0\0G\0\0\0Q\0\0\0R\0\0\0V\0\0\0W\0\0\0X\0\0\0Y\0\0\0Z\0\0\0\xC2\0\0\0\xC3\0\0\0\xC5\0\0\0\xC6\0\0\0\xC8\0\0\0\xCB\0\0\0\xCC\0\0\0\xCD\0\0\0\xD2\0\0\0\xD3\0\0\0\xD9\0\0\0\xDA\0\0\0") }, 29u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static GD: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"A\0\0\0J\0\0\0L\0\0\0Q\0\0\0R\0\0\0V\0\0\0") }, 18u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static AST: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"A\0\0\0J\0\0\0L\0\0\0W\0\0\0X\0\0\0[\0\0\0\xD1\0\0\0\xD2\0\0\0") }, 24u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static HA: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"A\0\0\0P\0\0\0R\0\0\0V\0\0\0W\0\0\0X\0\0\0Y\0\0\0[\0\0\0\x81\x01\0\0\x82\x01\0\0\x8A\x01\0\0\x8B\x01\0\0\x98\x01\0\0\x99\x01\0\0\xB3\x01\0\0\xB4\x01\0\0") }, 26u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static LT: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"A\0\0\0Q\0\0\0R\0\0\0W\0\0\0Y\0\0\0[\0\0\0\x04\x01\0\0\x05\x01\0\0\x0C\x01\0\0\r\x01\0\0\x16\x01\0\0\x17\x01\0\0\x18\x01\0\0\x19\x01\0\0.\x01\0\0/\x01\0\0`\x01\0\0a\x01\0\0j\x01\0\0k\x01\0\0r\x01\0\0s\x01\0\0}\x01\0\0~\x01\0\0") }, 32u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static BR: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"A\0\0\0Q\0\0\0R\0\0\0[\0\0\0") }, 25u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static SQ: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"A\0\0\0W\0\0\0X\0\0\0[\0\0\0\xC7\0\0\0\xC8\0\0\0\xCB\0\0\0\xCC\0\0\0") }, 27u32)
                    },
                    unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\t\0\0\0\0\0\x02\0\x04\0\x06\0\x08\0\n\0\x0C\0\x0E\0\x10\0DHGJLLNJRRSHTHXHZH") },
                ));
                static SC: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"A\0\0\0[\0\0\0") }, 26u32)
                    },
                    unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0TZ") },
                ));
                static CY: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"A\0\0\0[\0\0\0") }, 26u32)
                    },
                    unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x08\0\0\0\0\0\x02\0\x04\0\x06\0\x08\0\n\0\x0C\0\x0E\0CHDDFFLLNGPHRHTH") },
                ));
                static AF: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"A\0\0\0[\0\0\0") }, 26u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static WO: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"A\0\0\0[\0\0\0J\x01\0\0K\x01\0\0") }, 27u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static LV: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"A\0\0\0[\0\0\0\0\x01\0\0\x01\x01\0\0\x0C\x01\0\0\r\x01\0\0\x12\x01\0\0\x13\x01\0\0\"\x01\0\0#\x01\0\0*\x01\0\0+\x01\0\x006\x01\0\x007\x01\0\0;\x01\0\0<\x01\0\0E\x01\0\0F\x01\0\0`\x01\0\0a\x01\0\0j\x01\0\0k\x01\0\0}\x01\0\0~\x01\0\0") }, 37u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static DSB: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"A\0\0\0[\0\0\0\x06\x01\0\0\x07\x01\0\0\x0C\x01\0\0\r\x01\0\0A\x01\0\0B\x01\0\0Z\x01\0\0[\x01\0\0`\x01\0\0a\x01\0\0y\x01\0\0z\x01\0\0}\x01\0\0~\x01\0\0") }, 33u32)
                    },
                    unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0Ch") },
                ));
                static HSB: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"A\0\0\0[\0\0\0\x06\x01\0\0\x07\x01\0\0\x0C\x01\0\0\r\x01\0\0A\x01\0\0B\x01\0\0`\x01\0\0a\x01\0\0}\x01\0\0~\x01\0\0") }, 31u32)
                    },
                    unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x02\0CHD\xC5\xB9") },
                ));
                static HR: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"A\0\0\0[\0\0\0\x06\x01\0\0\x07\x01\0\0\x0C\x01\0\0\r\x01\0\0\x10\x01\0\0\x11\x01\0\0`\x01\0\0a\x01\0\0}\x01\0\0~\x01\0\0") }, 31u32)
                    },
                    unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x03\0\x05\0D\xC5\xBDLJNJ") },
                ));
                static SL: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"A\0\0\0[\0\0\0\x06\x01\0\0\x07\x01\0\0\x0C\x01\0\0\r\x01\0\0\x10\x01\0\0\x11\x01\0\0`\x01\0\0a\x01\0\0}\x01\0\0~\x01\0\0") }, 31u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static BS: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"A\0\0\0[\0\0\0\x06\x01\0\0\x07\x01\0\0\x0C\x01\0\0\r\x01\0\0`\x01\0\0a\x01\0\0}\x01\0\0~\x01\0\0") }, 30u32)
                    },
                    unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x03\0\x05\0D\xC5\xBDLJNJ") },
                ));
                static CS: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"A\0\0\0[\0\0\0\x0C\x01\0\0\r\x01\0\0X\x01\0\0Y\x01\0\0`\x01\0\0a\x01\0\0}\x01\0\0~\x01\0\0") }, 30u32)
                    },
                    unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0CH") },
                ));
                static HA_NE: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"A\0\0\0[\0\0\0\x81\x01\0\0\x82\x01\0\0\x8A\x01\0\0\x8B\x01\0\0\x98\x01\0\0\x99\x01\0\0\xB3\x01\0\0\xB4\x01\0\0") }, 30u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static RM: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"A\0\0\0[\0\0\0\xC0\0\0\0\xC1\0\0\0\xC8\0\0\0\xCA\0\0\0\xCC\0\0\0\xCD\0\0\0\xD2\0\0\0\xD3\0\0\0\xD9\0\0\0\xDA\0\0\0") }, 32u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static IS: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"A\0\0\0[\0\0\0\xC1\0\0\0\xC2\0\0\0\xC6\0\0\0\xC7\0\0\0\xC9\0\0\0\xCA\0\0\0\xCD\0\0\0\xCE\0\0\0\xD0\0\0\0\xD1\0\0\0\xD3\0\0\0\xD4\0\0\0\xD6\0\0\0\xD7\0\0\0\xDA\0\0\0\xDB\0\0\0\xDD\0\0\0\xDF\0\0\0") }, 36u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static FO: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"A\0\0\0[\0\0\0\xC1\0\0\0\xC2\0\0\0\xC6\0\0\0\xC7\0\0\0\xCD\0\0\0\xCE\0\0\0\xD0\0\0\0\xD1\0\0\0\xD3\0\0\0\xD4\0\0\0\xD8\0\0\0\xD9\0\0\0\xDA\0\0\0\xDB\0\0\0\xDD\0\0\0\xDE\0\0\0") }, 34u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static HU: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"A\0\0\0[\0\0\0\xC1\0\0\0\xC2\0\0\0\xC9\0\0\0\xCA\0\0\0\xCD\0\0\0\xCE\0\0\0\xD3\0\0\0\xD4\0\0\0\xD6\0\0\0\xD7\0\0\0\xDA\0\0\0\xDB\0\0\0\xDC\0\0\0\xDD\0\0\0P\x01\0\0Q\x01\0\0p\x01\0\0q\x01\0\0") }, 35u32)
                    },
                    unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\t\0\0\0\0\0\x02\0\x04\0\x07\0\t\0\x0B\0\r\0\x0F\0\x11\0CSDZDZSGYLYNYSZTYZS") },
                ));
                static VI: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"A\0\0\0[\0\0\0\xC2\0\0\0\xC3\0\0\0\xCA\0\0\0\xCB\0\0\0\xD4\0\0\0\xD5\0\0\0\x02\x01\0\0\x03\x01\0\0\x10\x01\0\0\x11\x01\0\0\xA0\x01\0\0\xA1\x01\0\0\xAF\x01\0\0\xB0\x01\0\0") }, 33u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static RO: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"A\0\0\0[\0\0\0\xC2\0\0\0\xC3\0\0\0\xCE\0\0\0\xCF\0\0\0\x02\x01\0\0\x03\x01\0\0\x18\x02\0\0\x19\x02\0\0\x1A\x02\0\0\x1B\x02\0\0") }, 31u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static SK: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"A\0\0\0[\0\0\0\xC4\0\0\0\xC5\0\0\0\xD4\0\0\0\xD5\0\0\0\x0C\x01\0\0\r\x01\0\0\x0E\x01\0\0\x0F\x01\0\0=\x01\0\0>\x01\0\0`\x01\0\0a\x01\0\0d\x01\0\0e\x01\0\0}\x01\0\0~\x01\0\0") }, 34u32)
                    },
                    unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0CH") },
                ));
                static ET: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"A\0\0\0[\0\0\0\xC4\0\0\0\xC5\0\0\0\xD5\0\0\0\xD7\0\0\0\xDC\0\0\0\xDD\0\0\0`\x01\0\0a\x01\0\0}\x01\0\0~\x01\0\0") }, 32u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static DE_AT: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"A\0\0\0[\0\0\0\xC4\0\0\0\xC5\0\0\0\xD6\0\0\0\xD7\0\0\0\xDC\0\0\0\xDD\0\0\0") }, 29u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static FI: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"A\0\0\0[\0\0\0\xC4\0\0\0\xC6\0\0\0\xD6\0\0\0\xD7\0\0\0") }, 29u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static DA: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"A\0\0\0[\0\0\0\xC5\0\0\0\xC7\0\0\0\xD8\0\0\0\xD9\0\0\0") }, 29u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static AZ: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"A\0\0\0[\0\0\0\xC7\0\0\0\xC8\0\0\0\xD6\0\0\0\xD7\0\0\0\xDC\0\0\0\xDD\0\0\0\x1E\x01\0\0\x1F\x01\0\x000\x01\0\x001\x01\0\0^\x01\0\0_\x01\0\0\x8F\x01\0\0\x90\x01\0\0") }, 33u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static TR: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"A\0\0\0[\0\0\0\xC7\0\0\0\xC8\0\0\0\xD6\0\0\0\xD7\0\0\0\xDC\0\0\0\xDD\0\0\x000\x01\0\x001\x01\0\0^\x01\0\0_\x01\0\0") }, 31u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static SU: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"A\0\0\0[\0\0\0\xC9\0\0\0\xCA\0\0\0") }, 27u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static FIL: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"A\0\0\0[\0\0\0\xD1\0\0\0\xD2\0\0\0") }, 27u32)
                    },
                    unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0Ng") },
                ));
                static ES: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"A\0\0\0[\0\0\0\xD1\0\0\0\xD2\0\0\0") }, 27u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static PL: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"A\0\0\0[\0\0\0\xD3\0\0\0\xD4\0\0\0\x06\x01\0\0\x07\x01\0\0A\x01\0\0B\x01\0\0Z\x01\0\0[\x01\0\0y\x01\0\0z\x01\0\0{\x01\0\0|\x01\0\0") }, 32u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static JA: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"B0\0\0C0\0\0K0\0\0L0\0\0U0\0\0V0\0\0_0\0\0`0\0\0j0\0\0k0\0\0o0\0\0p0\0\0~0\0\0\x7F0\0\0\x840\0\0\x850\0\0\x890\0\0\x8A0\0\0\x8F0\0\0\x900\0\0") }, 10u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static SO: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"B\0\0\0E\0\0\0F\0\0\0I\0\0\0J\0\0\0O\0\0\0Q\0\0\0U\0\0\0W\0\0\0Z\0\0\0") }, 18u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static FA: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\"\x06\0\0#\x06\0\0'\x06\0\0)\x06\0\0*\x06\0\0;\x06\0\0A\x06\0\0C\x06\0\0D\x06\0\0I\x06\0\0~\x06\0\0\x7F\x06\0\0\x86\x06\0\0\x87\x06\0\0\x98\x06\0\0\x99\x06\0\0\xA9\x06\0\0\xAA\x06\0\0\xAF\x06\0\0\xB0\x06\0\0\xCC\x06\0\0\xCD\x06\0\0") }, 33u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static ZH_HANT: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0N\0\0\x01N\0\0(N\0\0)N\0\x006N\0\x007N\0\0?N\0\0@N\0\0YN\0\0ZN\0\0\x85N\0\0\x86N\0\0\x8CN\0\0\x8DN\0\0\xA0N\0\0\xA1N\0\0\xBAN\0\0\xBBN\0\0?Q\0\0@Q\0\0eQ\0\0fQ\0\0kQ\0\0lQ\0\0\x82Q\0\0\x83Q\0\0\x96Q\0\0\x97Q\0\0\xABQ\0\0\xACQ\0\0\xE0Q\0\0\xE1Q\0\0\xF5Q\0\0\xF6Q\0\0\0R\0\0\x01R\0\0\x9BR\0\0\x9CR\0\0\xF9R\0\0\xFAR\0\0\x15S\0\0\x16S\0\0\x1AS\0\0\x1BS\0\08S\0\09S\0\0AS\0\0BS\0\0\\S\0\0]S\0\0iS\0\0jS\0\0\x82S\0\0\x83S\0\0\xB6S\0\0\xB7S\0\0\xC8S\0\0\xC9S\0\0\xE3S\0\0\xE4S\0\0\xD7V\0\0\xD8V\0\0\x1FW\0\0 W\0\0\xEBX\0\0\xECX\0\0\x02Y\0\0\x03Y\0\0\nY\0\0\x0BY\0\0\x15Y\0\0\x16Y\0\0'Y\0\0(Y\0\0sY\0\0tY\0\0P[\0\0Q[\0\0\x80[\0\0\x81[\0\0\xF8[\0\0\xF9[\0\0\x0F\\\0\0\x10\\\0\0\"\\\0\0#\\\0\08\\\0\09\\\0\0n\\\0\0o\\\0\0q\\\0\0r\\\0\0\xDB]\0\0\xDC]\0\0\xE5]\0\0\xE6]\0\0\xF1]\0\0\xF2]\0\0\xFE]\0\0\xFF]\0\0r^\0\0s^\0\0z^\0\0{^\0\0\x7F^\0\0\x80^\0\0\xF4^\0\0\xF5^\0\0\xFE^\0\0\xFF^\0\0\x0B_\0\0\x0C_\0\0\x13_\0\0\x14_\0\0P_\0\0Q_\0\0a_\0\0b_\0\0s_\0\0t_\0\0\xC3_\0\0\xC4_\0\0\x08b\0\0\tb\0\x006b\0\x007b\0\0Kb\0\0Lb\0\0/e\0\x000e\0\x004e\0\x005e\0\0\x87e\0\0\x88e\0\0\x97e\0\0\x98e\0\0\xA4e\0\0\xA5e\0\0\xB9e\0\0\xBAe\0\0\xE0e\0\0\xE1e\0\0\xE5e\0\0\xE6e\0\0\xF0f\0\0\xF1f\0\0\x08g\0\0\tg\0\0(g\0\0)g\0\0 k\0\0!k\0\0bk\0\0ck\0\0yk\0\0zk\0\0\xB3k\0\0\xB4k\0\0\xCBk\0\0\xCCk\0\0\xD4k\0\0\xD5k\0\0\xDBk\0\0\xDCk\0\0\x0Fl\0\0\x10l\0\0\x14l\0\0\x15l\0\x004l\0\x005l\0\0kp\0\0lp\0\0*r\0\0+r\0\x006r\0\x007r\0\0;r\0\0<r\0\0?r\0\0@r\0\0Gr\0\0Hr\0\0Yr\0\0Zr\0\0[r\0\0\\r\0\0\xACr\0\0\xADr\0\0\x84s\0\0\x85s\0\0\x89s\0\0\x8As\0\0\xDCt\0\0\xDDt\0\0\xE6t\0\0\xE7t\0\0\x18u\0\0\x19u\0\0\x1Fu\0\0 u\0\0(u\0\0)u\0\x000u\0\x001u\0\0\x8Bu\0\0\x8Cu\0\0\x92u\0\0\x93u\0\0vv\0\0wv\0\0}v\0\0~v\0\0\xAEv\0\0\xAFv\0\0\xBFv\0\0\xC0v\0\0\xEEv\0\0\xEFv\0\0\xDBw\0\0\xDCw\0\0\xE2w\0\0\xE3w\0\0\xF3w\0\0\xF4w\0\0:y\0\0;y\0\0\xB8y\0\0\xB9y\0\0\xBEy\0\0\xBFy\0\0tz\0\0uz\0\0\xCBz\0\0\xCCz\0\0\xF9z\0\0\xFAz\0\0s|\0\0t|\0\0\xF8|\0\0\xF9|\0\x006\x7F\0\x007\x7F\0\0Q\x7F\0\0R\x7F\0\0\x8A\x7F\0\0\x8B\x7F\0\0\xBD\x7F\0\0\xBE\x7F\0\0\x01\x80\0\0\x02\x80\0\0\x0C\x80\0\0\r\x80\0\0\x12\x80\0\0\x13\x80\0\x003\x80\0\x004\x80\0\0\x7F\x80\0\0\x80\x80\0\0\x89\x80\0\0\x8A\x80\0\0\xE3\x81\0\0\xE4\x81\0\0\xEA\x81\0\0\xEB\x81\0\0\xF3\x81\0\0\xF4\x81\0\0\xFC\x81\0\0\xFD\x81\0\0\x0C\x82\0\0\r\x82\0\0\x1B\x82\0\0\x1C\x82\0\0\x1F\x82\0\0 \x82\0\0n\x82\0\0o\x82\0\0r\x82\0\0s\x82\0\0x\x82\0\0y\x82\0\0M\x86\0\0N\x86\0\0k\x86\0\0l\x86\0\0@\x88\0\0A\x88\0\0L\x88\0\0M\x88\0\0c\x88\0\0d\x88\0\0~\x89\0\0\x7F\x89\0\0\x8B\x89\0\0\x8C\x89\0\0\xD2\x89\0\0\xD3\x89\0\0\0\x8A\0\0\x01\x8A\0\x007\x8C\0\08\x8C\0\0F\x8C\0\0G\x8C\0\0U\x8C\0\0V\x8C\0\0x\x8C\0\0y\x8C\0\0\x9D\x8C\0\0\x9E\x8C\0\0d\x8D\0\0e\x8D\0\0p\x8D\0\0q\x8D\0\0\xB3\x8D\0\0\xB4\x8D\0\0\xAB\x8E\0\0\xAC\x8E\0\0\xCA\x8E\0\0\xCB\x8E\0\0\x9B\x8F\0\0\x9C\x8F\0\0\xB0\x8F\0\0\xB1\x8F\0\0\xB5\x8F\0\0\xB6\x8F\0\0\x91\x90\0\0\x92\x90\0\0I\x91\0\0J\x91\0\0\xC6\x91\0\0\xC7\x91\0\0\xCC\x91\0\0\xCD\x91\0\0\xD1\x91\0\0\xD2\x91\0\0w\x95\0\0x\x95\0\0\x80\x95\0\0\x81\x95\0\0\x1C\x96\0\0\x1D\x96\0\0\xB6\x96\0\0\xB7\x96\0\0\xB9\x96\0\0\xBA\x96\0\0\xE8\x96\0\0\xE9\x96\0\0Q\x97\0\0R\x97\0\0^\x97\0\0_\x97\0\0b\x97\0\0c\x97\0\0i\x97\0\0j\x97\0\0\xCB\x97\0\0\xCC\x97\0\0\xED\x97\0\0\xEE\x97\0\0\xF3\x97\0\0\xF4\x97\0\0\x01\x98\0\0\x02\x98\0\0\xA8\x98\0\0\xA9\x98\0\0\xDB\x98\0\0\xDC\x98\0\0\xDF\x98\0\0\xE0\x98\0\0\x96\x99\0\0\x97\x99\0\0\x99\x99\0\0\x9A\x99\0\0\xAC\x99\0\0\xAD\x99\0\0\xA8\x9A\0\0\xA9\x9A\0\0\xD8\x9A\0\0\xD9\x9A\0\0\xDF\x9A\0\0\xE0\x9A\0\0%\x9B\0\0&\x9B\0\0/\x9B\0\x000\x9B\0\x002\x9B\0\x003\x9B\0\0<\x9B\0\0=\x9B\0\0Z\x9B\0\0[\x9B\0\0\xE5\x9C\0\0\xE6\x9C\0\0u\x9E\0\0v\x9E\0\0\x7F\x9E\0\0\x80\x9E\0\0\xA5\x9E\0\0\xA6\x9E\0\0\xBB\x9E\0\0\xBC\x9E\0\0\xC3\x9E\0\0\xC4\x9E\0\0\xCD\x9E\0\0\xCE\x9E\0\0\xD1\x9E\0\0\xD2\x9E\0\0\xF9\x9E\0\0\xFA\x9E\0\0\xFD\x9E\0\0\xFE\x9E\0\0\x0E\x9F\0\0\x0F\x9F\0\0\x13\x9F\0\0\x14\x9F\0\0 \x9F\0\0!\x9F\0\0;\x9F\0\0<\x9F\0\0J\x9F\0\0K\x9F\0\0R\x9F\0\0S\x9F\0\0\x8D\x9F\0\0\x8E\x9F\0\0\x9C\x9F\0\0\x9D\x9F\0\0\xA0\x9F\0\0\xA1\x9F\0\0") }, 214u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static YUE: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0N\0\0\x02N\0\0\x08N\0\0\tN\0\0\rN\0\0\x0EN\0\0\x14N\0\0\x15N\0\0\x1EN\0\0\x1FN\0\0&N\0\0'N\0\x002N\0\x003N\0\0XN\0\0YN\0\0~N\0\0\x7FN\0\0\x82N\0\0\x83N\0\0\xADN\0\0\xAEN\0\0\x80P\0\0\x81P\0\0\xCEP\0\0\xCFP\0\0\xF5P\0\0\xF6P\0\0\x10Q\0\0\x11Q\0\0\x1FQ\0\0 Q\0\x003Q\0\x004Q\0\x007Q\0\08Q\0\0;Q\0\0<Q\0\0\xE2S\0\0\xE3S\0\0\xB4V\0\0\xB5V\0\0\xCCV\0\0\xCDV\0\0\xD1V\0\0\xD2V\0\0\xF3^\0\0\xF4^\0\0") }, 25u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static MY: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0\x10\0\0\"\x10\0\0") }, 34u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static TI: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0\x12\0\0\x01\x12\0\0\x08\x12\0\0\t\x12\0\0\x10\x12\0\0\x11\x12\0\0\x18\x12\0\0\x19\x12\0\0 \x12\0\0!\x12\0\0(\x12\0\0)\x12\0\x000\x12\0\x001\x12\0\08\x12\0\09\x12\0\0@\x12\0\0A\x12\0\0H\x12\0\0I\x12\0\0P\x12\0\0Q\x12\0\0X\x12\0\0Y\x12\0\0`\x12\0\0a\x12\0\0h\x12\0\0i\x12\0\0p\x12\0\0q\x12\0\0x\x12\0\0y\x12\0\0\x80\x12\0\0\x81\x12\0\0\x88\x12\0\0\x89\x12\0\0\x90\x12\0\0\x91\x12\0\0\x98\x12\0\0\x99\x12\0\0\xA0\x12\0\0\xA1\x12\0\0\xA8\x12\0\0\xA9\x12\0\0\xB0\x12\0\0\xB1\x12\0\0\xB8\x12\0\0\xB9\x12\0\0\xC0\x12\0\0\xC1\x12\0\0\xC8\x12\0\0\xC9\x12\0\0\xD0\x12\0\0\xD1\x12\0\0\xD8\x12\0\0\xD9\x12\0\0\xE0\x12\0\0\xE1\x12\0\0\xE8\x12\0\0\xE9\x12\0\0\xF0\x12\0\0\xF1\x12\0\0\0\x13\0\0\x01\x13\0\0\x08\x13\0\0\t\x13\0\0\x10\x13\0\0\x11\x13\0\0 \x13\0\0!\x13\0\0(\x13\0\0)\x13\0\x000\x13\0\x001\x13\0\08\x13\0\09\x13\0\0@\x13\0\0A\x13\0\0H\x13\0\0I\x13\0\0P\x13\0\0Q\x13\0\0") }, 41u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static AM: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0\x12\0\0\x01\x12\0\0\x08\x12\0\0\t\x12\0\0\x10\x12\0\0\x11\x12\0\0\x18\x12\0\0\x19\x12\0\0 \x12\0\0!\x12\0\0(\x12\0\0)\x12\0\x000\x12\0\x001\x12\0\08\x12\0\09\x12\0\0@\x12\0\0A\x12\0\0H\x12\0\0I\x12\0\0`\x12\0\0a\x12\0\0h\x12\0\0i\x12\0\0p\x12\0\0q\x12\0\0x\x12\0\0y\x12\0\0\x80\x12\0\0\x81\x12\0\0\x88\x12\0\0\x89\x12\0\0\x90\x12\0\0\x91\x12\0\0\x98\x12\0\0\x99\x12\0\0\xA0\x12\0\0\xA1\x12\0\0\xA8\x12\0\0\xA9\x12\0\0\xB0\x12\0\0\xB1\x12\0\0\xB8\x12\0\0\xB9\x12\0\0\xC8\x12\0\0\xC9\x12\0\0\xD0\x12\0\0\xD1\x12\0\0\xD8\x12\0\0\xD9\x12\0\0\xE0\x12\0\0\xE1\x12\0\0\xE8\x12\0\0\xE9\x12\0\0\xF0\x12\0\0\xF1\x12\0\0\0\x13\0\0\x01\x13\0\0\x08\x13\0\0\t\x13\0\0\x10\x13\0\0\x11\x13\0\0 \x13\0\0!\x13\0\0(\x13\0\0)\x13\0\x000\x13\0\x001\x13\0\08\x13\0\09\x13\0\0@\x13\0\0A\x13\0\0H\x13\0\0I\x13\0\0P\x13\0\0Q\x13\0\0") }, 38u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static FF_ADLM: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0\xE9\x01\0\x01\xE9\x01\0\x1B\xE9\x01\0\x1C\xE9\x01\0") }, 2u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static BE: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\x04\0\0\x02\x04\0\0\x06\x04\0\0\x07\x04\0\0\x10\x04\0\0\x18\x04\0\0\x19\x04\0\0)\x04\0\0+\x04\0\x000\x04\0\0") }, 31u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static KK: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\x04\0\0\x02\x04\0\0\x06\x04\0\0\x07\x04\0\0\x10\x04\0\x000\x04\0\0\x92\x04\0\0\x93\x04\0\0\x9A\x04\0\0\x9B\x04\0\0\xA2\x04\0\0\xA3\x04\0\0\xAE\x04\0\0\xAF\x04\0\0\xB0\x04\0\0\xB1\x04\0\0\xBA\x04\0\0\xBB\x04\0\0\xD8\x04\0\0\xD9\x04\0\0\xE8\x04\0\0\xE9\x04\0\0") }, 42u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static TG: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\x04\0\0\x02\x04\0\0\x10\x04\0\0&\x04\0\0'\x04\0\0)\x04\0\0*\x04\0\0+\x04\0\0-\x04\0\x000\x04\0\0\x92\x04\0\0\x93\x04\0\0\x9A\x04\0\0\x9B\x04\0\0\xB2\x04\0\0\xB3\x04\0\0\xB6\x04\0\0\xB7\x04\0\0\xE2\x04\0\0\xE3\x04\0\0\xEE\x04\0\0\xEF\x04\0\0") }, 35u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static RU: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\x04\0\0\x02\x04\0\0\x10\x04\0\0*\x04\0\0+\x04\0\0,\x04\0\0-\x04\0\x000\x04\0\0") }, 31u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static TT: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\x04\0\0\x02\x04\0\0\x10\x04\0\x000\x04\0\0\x96\x04\0\0\x97\x04\0\0\xA2\x04\0\0\xA3\x04\0\0\xAE\x04\0\0\xAF\x04\0\0\xBA\x04\0\0\xBB\x04\0\0\xD8\x04\0\0\xD9\x04\0\0\xE8\x04\0\0\xE9\x04\0\0") }, 39u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static KY: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\x04\0\0\x02\x04\0\0\x10\x04\0\x000\x04\0\0\xA2\x04\0\0\xA3\x04\0\0\xAE\x04\0\0\xAF\x04\0\0\xE8\x04\0\0\xE9\x04\0\0") }, 36u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static CV: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\x04\0\0\x02\x04\0\0\x10\x04\0\x000\x04\0\0\xAA\x04\0\0\xAB\x04\0\0\xD0\x04\0\0\xD1\x04\0\0\xD6\x04\0\0\xD7\x04\0\0\xF2\x04\0\0\xF3\x04\0\0") }, 37u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static TH: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x01\x0E\0\0/\x0E\0\0") }, 46u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static MR: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\t\0\0\x04\t\0\0\x05\t\0\0\r\t\0\0\x0F\t\0\0\x12\t\0\0\x13\t\0\0)\t\0\0*\t\0\x001\t\0\x002\t\0\x004\t\0\x005\t\0\0:\t\0\0=\t\0\0>\t\0\0E\t\0\0F\t\0\0M\t\0\0N\t\0\0P\t\0\0Q\t\0\0\r \0\0\x0E \0\0") }, 54u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static BS_CYRL: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x02\x04\0\0\x03\x04\0\0\x08\x04\0\0\x0C\x04\0\0\x0F\x04\0\0\x19\x04\0\0\x1A\x04\0\0)\x04\0\0") }, 30u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static MK: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x03\x04\0\0\x04\x04\0\0\x05\x04\0\0\x06\x04\0\0\x08\x04\0\0\x0B\x04\0\0\x0C\x04\0\0\r\x04\0\0\x0F\x04\0\0\x19\x04\0\0\x1A\x04\0\0)\x04\0\0") }, 31u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static UK: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x04\x04\0\0\x05\x04\0\0\x06\x04\0\0\x08\x04\0\0\x10\x04\0\0*\x04\0\0.\x04\0\x000\x04\0\0\x90\x04\0\0\x91\x04\0\0") }, 32u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static PA: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\n\0\0\x06\n\0\0\x15\n\0\0)\n\0\0*\n\0\x001\n\0\x002\n\0\x003\n\0\x005\n\0\x006\n\0\08\n\0\0:\n\0\0\\\n\0\0]\n\0\0r\n\0\0t\n\0\0") }, 35u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static ML: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\r\0\0\x0C\r\0\0\x0E\r\0\0\x11\r\0\0\x12\r\0\0)\r\0\0*\r\0\0:\r\0\0") }, 49u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static DOI: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\t\0\0\r\t\0\0\x0F\t\0\0\x11\t\0\0\x13\t\0\0)\t\0\0*\t\0\x001\t\0\x002\t\0\x004\t\0\x005\t\0\0:\t\0\0`\t\0\0b\t\0\0") }, 48u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static MAI: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\t\0\0\r\t\0\0\x0F\t\0\0\x11\t\0\0\x13\t\0\0\x19\t\0\0\x1A\t\0\0)\t\0\0*\t\0\x001\t\0\x002\t\0\x003\t\0\x005\t\0\0:\t\0\0<\t\0\0=\t\0\0a\t\0\0b\t\0\0") }, 46u32)
                    },
                    unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\x06\0\x0C\0\x15\0\x1E\0$\0-\0\xE0\xA4\x85\xE0\xA4\x82\xE0\xA4\x85\xE0\xA4\x83\xE0\xA4\x95\xE0\xA5\x8D\xE0\xA4\xB7\xE0\xA4\x9C\xE0\xA5\x8D\xE0\xA4\x9E\xE0\xA4\xA1\xE0\xA4\x82\xE0\xA4\xA4\xE0\xA5\x8D\xE0\xA4\xB0\xE0\xA4\xB6\xE0\xA5\x8D\xE0\xA4\xB0") },
                ));
                static BRX: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\t\0\0\x0B\t\0\0\r\t\0\0\x0E\t\0\0\x0F\t\0\0\x12\t\0\0\x13\t\0\0\x19\t\0\0\x1A\t\0\0)\t\0\0*\t\0\x001\t\0\x002\t\0\x004\t\0\x005\t\0\0:\t\0\0") }, 45u32)
                    },
                    unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE0\xA4\xA1\xE0\xA4\xBC") },
                ));
                static HI: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\t\0\0\x0C\t\0\0\x0F\t\0\0\x11\t\0\0\x13\t\0\0)\t\0\0*\t\0\x001\t\0\x002\t\0\x003\t\0\x005\t\0\0:\t\0\0") }, 44u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static KOK: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\t\0\0\x0E\t\0\0\x0F\t\0\0\x12\t\0\0\x13\t\0\0)\t\0\0*\t\0\x001\t\0\x002\t\0\x004\t\0\x005\t\0\0:\t\0\0") }, 48u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static OR: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x0B\0\0\x0C\x0B\0\0\x0F\x0B\0\0\x11\x0B\0\0\x13\x0B\0\0)\x0B\0\0*\x0B\0\x001\x0B\0\x002\x0B\0\x004\x0B\0\x006\x0B\0\0:\x0B\0\0") }, 44u32)
                    },
                    unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE0\xAC\x95\xE0\xAD\x8D\xE0\xAC\xB7") },
                ));
                static TE: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x05\x0C\0\0\x0C\x0C\0\0\x0E\x0C\0\0\x11\x0C\0\0\x12\x0C\0\0)\x0C\0\0*\x0C\0\x004\x0C\0\x005\x0C\0\0:\x0C\0\0`\x0C\0\0a\x0C\0\0") }, 49u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static BG: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x10\x04\0\0*\x04\0\0.\x04\0\x000\x04\0\0") }, 28u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static MN: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x10\x04\0\x000\x04\0\0\xAE\x04\0\0\xAF\x04\0\0\xE8\x04\0\0\xE9\x04\0\0") }, 34u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static KM: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x80\x17\0\0\x9D\x17\0\0\x9F\x17\0\0\xA3\x17\0\0\xA5\x17\0\0\xA8\x17\0\0\xA9\x17\0\0\xB2\x17\0\0\xB3\x17\0\0\xB4\x17\0\0") }, 46u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static LO: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x81\x0E\0\0\x83\x0E\0\0\x84\x0E\0\0\x85\x0E\0\0\x87\x0E\0\0\x89\x0E\0\0\x8A\x0E\0\0\x8B\x0E\0\0\x8D\x0E\0\0\x8E\x0E\0\0\x94\x0E\0\0\x98\x0E\0\0\x99\x0E\0\0\xA0\x0E\0\0\xA1\x0E\0\0\xA4\x0E\0\0\xA5\x0E\0\0\xA6\x0E\0\0\xA7\x0E\0\0\xA8\x0E\0\0\xAA\x0E\0\0\xAC\x0E\0\0\xAD\x0E\0\0\xAF\x0E\0\0") }, 27u32)
                    },
                    unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x06\0\0\0\0\0\x06\0\x0C\0\x12\0\x18\0\x1E\0\xE0\xBA\xAB\xE0\xBA\x87\xE0\xBA\xAB\xE0\xBA\x8D\xE0\xBA\xAB\xE0\xBA\x99\xE0\xBA\xAB\xE0\xBA\xA1\xE0\xBA\xAB\xE0\xBA\xA5\xE0\xBA\xAB\xE0\xBA\xA7") },
                ));
                static AS: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x82\t\0\0\x84\t\0\0\x85\t\0\0\x8C\t\0\0\x8F\t\0\0\x91\t\0\0\x93\t\0\0\xA9\t\0\0\xAA\t\0\0\xB0\t\0\0\xB2\t\0\0\xB3\t\0\0\xB6\t\0\0\xBA\t\0\0\xBC\t\0\0\xBD\t\0\0\xCD\t\0\0\xCF\t\0\0\xF0\t\0\0\xF2\t\0\0") }, 49u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static GU: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x85\n\0\0\x8C\n\0\0\x8D\n\0\0\x8E\n\0\0\x8F\n\0\0\x92\n\0\0\x93\n\0\0\xA9\n\0\0\xAA\n\0\0\xB1\n\0\0\xB2\n\0\0\xB4\n\0\0\xB5\n\0\0\xBA\n\0\0") }, 47u32)
                    },
                    unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x05\0\0\0\0\0\x06\0\x0C\0\x15\0\x1E\0\xE0\xAA\x85\xE0\xAA\x82\xE0\xAA\x85\xE0\xAA\x83\xE0\xAA\x95\xE0\xAB\x8D\xE0\xAA\xB7\xE0\xAA\x9C\xE0\xAB\x8D\xE0\xAA\x9E\xE0\xAA\xA4\xE0\xAB\x8D\xE0\xAA\xB0") },
                ));
                static SI: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x85\r\0\0\x8E\r\0\0\x91\r\0\0\x97\r\0\0\x9A\r\0\0\xA6\r\0\0\xA7\r\0\0\xB2\r\0\0\xB3\r\0\0\xBC\r\0\0\xBD\r\0\0\xBE\r\0\0\xC0\r\0\0\xC7\r\0\0") }, 55u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static BN: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x85\t\0\0\x8C\t\0\0\x8F\t\0\0\x91\t\0\0\x93\t\0\0\xA9\t\0\0\xAA\t\0\0\xB1\t\0\0\xB2\t\0\0\xB3\t\0\0\xB6\t\0\0\xBA\t\0\0") }, 43u32)
                    },
                    unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE0\xA6\x95\xE0\xA7\x8D\xE0\xA6\xB7") },
                ));
                static TA: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x85\x0B\0\0\x8B\x0B\0\0\x8E\x0B\0\0\x91\x0B\0\0\x92\x0B\0\0\x96\x0B\0\0\x99\x0B\0\0\x9B\x0B\0\0\x9E\x0B\0\0\xA0\x0B\0\0\xA3\x0B\0\0\xA5\x0B\0\0\xA8\x0B\0\0\xAB\x0B\0\0\xAE\x0B\0\0\xB6\x0B\0\0") }, 30u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static KN: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x85\x0C\0\0\x8D\x0C\0\0\x8E\x0C\0\0\x91\x0C\0\0\x92\x0C\0\0\xA9\x0C\0\0\xAA\x0C\0\0\xB4\x0C\0\0\xB5\x0C\0\0\xBA\x0C\0\0\xDE\x0C\0\0\xDF\x0C\0\0\xE0\x0C\0\0\xE2\x0C\0\0") }, 52u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static EL: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x91\x03\0\0\xA2\x03\0\0\xA3\x03\0\0\xAA\x03\0\0") }, 24u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static CHR: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xA0\x13\0\0\xA1\x13\0\0\xA6\x13\0\0\xA7\x13\0\0\xAD\x13\0\0\xAE\x13\0\0\xB3\x13\0\0\xB4\x13\0\0\xB9\x13\0\0\xBA\x13\0\0\xBE\x13\0\0\xBF\x13\0\0\xC6\x13\0\0\xC7\x13\0\0\xCC\x13\0\0\xCD\x13\0\0\xD3\x13\0\0\xD4\x13\0\0\xDC\x13\0\0\xDD\x13\0\0\xE3\x13\0\0\xE4\x13\0\0\xE9\x13\0\0\xEA\x13\0\0\xEF\x13\0\0\xF0\x13\0\0") }, 13u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static HE: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xD0\x05\0\0\xDA\x05\0\0\xDB\x05\0\0\xDD\x05\0\0\xDE\x05\0\0\xDF\x05\0\0\xE0\x05\0\0\xE3\x05\0\0\xE4\x05\0\0\xE5\x05\0\0\xE6\x05\0\0\xEB\x05\0\0") }, 22u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static KA: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xD0\x10\0\0\xF1\x10\0\0") }, 33u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static BGC: <icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable = icu_properties::provider::PropertyUnicodeSetV1::CPInversionListStrList(icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList::from_parts_unchecked(
                    unsafe {
                        #[allow(unused_unsafe)]
                        icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(zerovec::ZeroVec::new(), 0u32)
                    },
                    zerovec::VarZeroVec::new(),
                ));
                static VALUES: [&<icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::DataMarker>::Yokeable; 444usize] = [&AF, &AF, &AM, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AR, &AS, &AST, &AZ, &AZ, &BE, &BE, &BG, &BGC, &BGC, &BN, &BN, &BR, &BRX, &BS, &BS_CYRL, &BS, &AF, &AF, &AF, &AF, &AF, &AF, &CHR, &CS, &CV, &CY, &DA, &DA, &AF, &DE_AT, &AF, &AF, &AF, &AF, &AF, &DOI, &DSB, &EL, &EL, &EL, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &ES, &ES, &ES, &ES, &ES, &ES, &ES, &ES, &ES, &ES, &ES, &ES, &ES, &ES, &ES, &ES, &ES, &ES, &ES, &ES, &ES, &ES, &ES, &ES, &ES, &ES, &ES, &ES, &ET, &AF, &FA, &FA, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FF_ADLM, &FI, &FIL, &FO, &FO, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &GD, &ES, &GU, &HA, &HA, &HA_NE, &HE, &HI, &AF, &HR, &HR, &HSB, &HU, &HY, &AF, &AF, &AF, &IS, &AF, &AF, &AF, &AF, &JA, &JV, &KA, &KEA, &KGP, &KK, &KM, &KN, &KO, &KO, &KOK, &BGC, &BGC, &BGC, &KY, &LO, &LT, &LV, &MAI, &MI, &MK, &ML, &MN, &BGC, &BGC, &MR, &AF, &AF, &AF, &AF, &MY, &DA, &DA, &HI, &HI, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &DA, &DA, &OR, &PA, &PA, &PCM, &PL, &PS, &PS, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &AF, &QU, &QU, &QU, &BGC, &RM, &RO, &RO, &RU, &RU, &RU, &RU, &RU, &RU, &DOI, &BGC, &BGC, &SC, &SD, &SD, &BGC, &SI, &SK, &SL, &SO, &SO, &SO, &SO, &SQ, &SQ, &SQ, &BS_CYRL, &BS_CYRL, &BS_CYRL, &BS_CYRL, &BS, &BS, &BS, &BS, &SU, &SU, &FI, &FI, &FI, &PCM, &PCM, &PCM, &PCM, &TA, &TA, &TA, &TA, &TE, &TG, &TH, &TI, &TI, &TK, &TO, &TR, &TR, &TT, &UK, &BGC, &UR, &UR, &UZ, &BGC, &UZ, &VI, &WO, &AF, &YO, &YO, &YRL, &YRL, &YRL, &YUE, &AF, &YUE, &AF, &AF, &AF, &ZH_HANT, &ZH_HANT, &ZH_HANT, &AF];
                static KEYS: [&str; 444usize] = ["af", "af-NA", "am", "ar", "ar-AE", "ar-BH", "ar-DJ", "ar-DZ", "ar-EG", "ar-EH", "ar-ER", "ar-IL", "ar-IQ", "ar-JO", "ar-KM", "ar-KW", "ar-LB", "ar-LY", "ar-MA", "ar-MR", "ar-OM", "ar-PS", "ar-QA", "ar-SA", "ar-SD", "ar-SO", "ar-SS", "ar-SY", "ar-TD", "ar-TN", "ar-YE", "as", "ast", "az", "az-Latn", "be", "be-tarask", "bg", "bgc", "bho", "bn", "bn-IN", "br", "brx", "bs", "bs-Cyrl", "bs-Latn", "ca", "ca-AD", "ca-ES-valencia", "ca-FR", "ca-IT", "ceb", "chr", "cs", "cv", "cy", "da", "da-GL", "de", "de-AT", "de-BE", "de-CH", "de-IT", "de-LI", "de-LU", "doi", "dsb", "el", "el-CY", "el-polyton", "en", "en-001", "en-150", "en-AE", "en-AG", "en-AI", "en-AS", "en-AT", "en-AU", "en-BB", "en-BE", "en-BI", "en-BM", "en-BS", "en-BW", "en-BZ", "en-CA", "en-CC", "en-CH", "en-CK", "en-CM", "en-CX", "en-CY", "en-DE", "en-DG", "en-DK", "en-DM", "en-ER", "en-FI", "en-FJ", "en-FK", "en-FM", "en-GB", "en-GD", "en-GG", "en-GH", "en-GI", "en-GM", "en-GU", "en-GY", "en-HK", "en-IE", "en-IL", "en-IM", "en-IN", "en-IO", "en-JE", "en-JM", "en-KE", "en-KI", "en-KN", "en-KY", "en-LC", "en-LR", "en-LS", "en-MG", "en-MH", "en-MO", "en-MP", "en-MS", "en-MT", "en-MU", "en-MV", "en-MW", "en-MY", "en-NA", "en-NF", "en-NG", "en-NL", "en-NR", "en-NU", "en-NZ", "en-PG", "en-PH", "en-PK", "en-PN", "en-PR", "en-PW", "en-RW", "en-SB", "en-SC", "en-SD", "en-SE", "en-SG", "en-SH", "en-SI", "en-SL", "en-SS", "en-SX", "en-SZ", "en-TC", "en-TK", "en-TO", "en-TT", "en-TV", "en-TZ", "en-UG", "en-UM", "en-VC", "en-VG", "en-VI", "en-VU", "en-WS", "en-ZA", "en-ZM", "en-ZW", "es", "es-419", "es-AR", "es-BO", "es-BR", "es-BZ", "es-CL", "es-CO", "es-CR", "es-CU", "es-DO", "es-EA", "es-EC", "es-GQ", "es-GT", "es-HN", "es-IC", "es-MX", "es-NI", "es-PA", "es-PE", "es-PH", "es-PR", "es-PY", "es-SV", "es-US", "es-UY", "es-VE", "et", "eu", "fa", "fa-AF", "ff-Adlm", "ff-Adlm-BF", "ff-Adlm-CM", "ff-Adlm-GH", "ff-Adlm-GM", "ff-Adlm-GW", "ff-Adlm-LR", "ff-Adlm-MR", "ff-Adlm-NE", "ff-Adlm-NG", "ff-Adlm-SL", "ff-Adlm-SN", "fi", "fil", "fo", "fo-DK", "fr", "fr-BE", "fr-BF", "fr-BI", "fr-BJ", "fr-BL", "fr-CA", "fr-CD", "fr-CF", "fr-CG", "fr-CH", "fr-CI", "fr-CM", "fr-DJ", "fr-DZ", "fr-GA", "fr-GF", "fr-GN", "fr-GP", "fr-GQ", "fr-HT", "fr-KM", "fr-LU", "fr-MA", "fr-MC", "fr-MF", "fr-MG", "fr-ML", "fr-MQ", "fr-MR", "fr-MU", "fr-NC", "fr-NE", "fr-PF", "fr-PM", "fr-RE", "fr-RW", "fr-SC", "fr-SN", "fr-SY", "fr-TD", "fr-TG", "fr-TN", "fr-VU", "fr-WF", "fr-YT", "ga", "ga-GB", "gd", "gl", "gu", "ha", "ha-GH", "ha-NE", "he", "hi", "hi-Latn", "hr", "hr-BA", "hsb", "hu", "hy", "ia", "id", "ig", "is", "it", "it-CH", "it-SM", "it-VA", "ja", "jv", "ka", "kea", "kgp", "kk", "km", "kn", "ko", "ko-KP", "kok", "ks", "ks-Arab", "ks-Deva", "ky", "lo", "lt", "lv", "mai", "mi", "mk", "ml", "mn", "mni", "mni-Beng", "mr", "ms", "ms-BN", "ms-ID", "ms-SG", "my", "nb", "nb-SJ", "ne", "ne-IN", "nl", "nl-AW", "nl-BE", "nl-BQ", "nl-CW", "nl-SR", "nl-SX", "nn", "no", "or", "pa", "pa-Guru", "pcm", "pl", "ps", "ps-PK", "pt", "pt-AO", "pt-CH", "pt-CV", "pt-GQ", "pt-GW", "pt-LU", "pt-MO", "pt-MZ", "pt-PT", "pt-ST", "pt-TL", "qu", "qu-BO", "qu-EC", "raj", "rm", "ro", "ro-MD", "ru", "ru-BY", "ru-KG", "ru-KZ", "ru-MD", "ru-UA", "sa", "sat", "sat-Olck", "sc", "sd", "sd-Arab", "sd-Deva", "si", "sk", "sl", "so", "so-DJ", "so-ET", "so-KE", "sq", "sq-MK", "sq-XK", "sr", "sr-Cyrl", "sr-Cyrl-BA", "sr-Cyrl-XK", "sr-Latn", "sr-Latn-BA", "sr-Latn-ME", "sr-Latn-XK", "su", "su-Latn", "sv", "sv-AX", "sv-FI", "sw", "sw-CD", "sw-KE", "sw-UG", "ta", "ta-LK", "ta-MY", "ta-SG", "te", "tg", "th", "ti", "ti-ER", "tk", "to", "tr", "tr-CY", "tt", "uk", "und", "ur", "ur-IN", "uz", "uz-Cyrl", "uz-Latn", "vi", "wo", "xh", "yo", "yo-BJ", "yrl", "yrl-CO", "yrl-VE", "yue", "yue-Hans", "yue-Hant", "zh", "zh-Hans", "zh-Hans-SG", "zh-Hant", "zh-Hant-HK", "zh-Hant-MO", "zu"];
                let mut metadata = icu_provider::DataResponseMetadata::default();
                let payload = if let Ok(payload) = KEYS.binary_search_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse()).map(|i| *unsafe { VALUES.get_unchecked(i) }) {
                    payload
                } else {
                    const FALLBACKER: icu_locid_transform::fallback::LocaleFallbackerWithConfig<'static> = icu_locid_transform::fallback::LocaleFallbacker::new().for_config(icu_locid_transform::fallback::LocaleFallbackConfig::from_key(<icu_properties::provider::ExemplarCharactersIndexV1Marker as icu_provider::KeyedDataMarker>::KEY));
                    let mut fallback_iterator = FALLBACKER.fallback_for(req.locale.clone());
                    loop {
                        if let Ok(payload) = KEYS.binary_search_by(|k| fallback_iterator.get().strict_cmp(k.as_bytes()).reverse()).map(|i| *unsafe { VALUES.get_unchecked(i) }) {
                            metadata.locale = Some(fallback_iterator.take());
                            break payload;
                        }
                        fallback_iterator.step();
                    }
                };
                Ok(icu_provider::DataResponse { payload: Some(icu_provider::DataPayload::from_static_ref(payload)), metadata })
            }
        }
    };
}
