// @generated
/// Implement `DataProvider<PrependedConcatenationMarkV1Marker>` on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_props_pcm_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.65"]
        impl $provider {
            #[doc(hidden)]
            pub const SINGLETON_PROPS_PCM_V1: &'static <icu_properties::provider::PrependedConcatenationMarkV1Marker as icu_provider::DataMarker>::Yokeable = &icu_properties::provider::PropertyCodePointSetV1::InversionList(unsafe {
                #[allow(unused_unsafe)]
                icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0\x06\0\0\x06\x06\0\0\xDD\x06\0\0\xDE\x06\0\0\x0F\x07\0\0\x10\x07\0\0\x90\x08\0\0\x92\x08\0\0\xE2\x08\0\0\xE3\x08\0\0\xBD\x10\x01\0\xBE\x10\x01\0\xCD\x10\x01\0\xCE\x10\x01\0") }, 13u32)
            });
        }
        #[clippy::msrv = "1.65"]
        impl icu_provider::DataProvider<icu_properties::provider::PrependedConcatenationMarkV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_properties::provider::PrependedConcatenationMarkV1Marker>, icu_provider::DataError> {
                if req.locale.is_empty() {
                    Ok(icu_provider::DataResponse { payload: Some(icu_provider::DataPayload::from_static_ref(Self::SINGLETON_PROPS_PCM_V1)), metadata: Default::default() })
                } else {
                    Err(icu_provider::DataErrorKind::ExtraneousLocale.with_req(<icu_properties::provider::PrependedConcatenationMarkV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
                }
            }
        }
    };
}
