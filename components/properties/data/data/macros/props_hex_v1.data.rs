// @generated
/// Implement `DataProvider<HexDigitV1Marker>` on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_props_hex_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.65"]
        impl $provider {
            #[doc(hidden)]
            pub const SINGLETON_PROPS_HEX_V1: &'static <icu_properties::provider::HexDigitV1Marker as icu_provider::DataMarker>::Yokeable = &icu_properties::provider::PropertyCodePointSetV1::InversionList(unsafe {
                #[allow(unused_unsafe)]
                icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"0\0\0\0:\0\0\0A\0\0\0G\0\0\0a\0\0\0g\0\0\0\x10\xFF\0\0\x1A\xFF\0\0!\xFF\0\0'\xFF\0\0A\xFF\0\0G\xFF\0\0") }, 44u32)
            });
        }
        #[clippy::msrv = "1.65"]
        impl icu_provider::DataProvider<icu_properties::provider::HexDigitV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_properties::provider::HexDigitV1Marker>, icu_provider::DataError> {
                if req.locale.is_empty() {
                    Ok(icu_provider::DataResponse { payload: Some(icu_provider::DataPayload::from_static_ref(Self::SINGLETON_PROPS_HEX_V1)), metadata: Default::default() })
                } else {
                    Err(icu_provider::DataErrorKind::ExtraneousLocale.with_req(<icu_properties::provider::HexDigitV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
                }
            }
        }
    };
}
