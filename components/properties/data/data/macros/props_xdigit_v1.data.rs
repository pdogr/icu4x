// @generated
/// Implement `DataProvider<XdigitV1Marker>` on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_props_xdigit_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.65"]
        impl $provider {
            #[doc(hidden)]
            pub const SINGLETON_PROPS_XDIGIT_V1: &'static <icu_properties::provider::XdigitV1Marker as icu_provider::DataMarker>::Yokeable = &icu_properties::provider::PropertyCodePointSetV1::InversionList(unsafe {
                #[allow(unused_unsafe)]
                icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"0\0\0\0:\0\0\0A\0\0\0G\0\0\0a\0\0\0g\0\0\0`\x06\0\0j\x06\0\0\xF0\x06\0\0\xFA\x06\0\0\xC0\x07\0\0\xCA\x07\0\0f\t\0\0p\t\0\0\xE6\t\0\0\xF0\t\0\0f\n\0\0p\n\0\0\xE6\n\0\0\xF0\n\0\0f\x0B\0\0p\x0B\0\0\xE6\x0B\0\0\xF0\x0B\0\0f\x0C\0\0p\x0C\0\0\xE6\x0C\0\0\xF0\x0C\0\0f\r\0\0p\r\0\0\xE6\r\0\0\xF0\r\0\0P\x0E\0\0Z\x0E\0\0\xD0\x0E\0\0\xDA\x0E\0\0 \x0F\0\0*\x0F\0\0@\x10\0\0J\x10\0\0\x90\x10\0\0\x9A\x10\0\0\xE0\x17\0\0\xEA\x17\0\0\x10\x18\0\0\x1A\x18\0\0F\x19\0\0P\x19\0\0\xD0\x19\0\0\xDA\x19\0\0\x80\x1A\0\0\x8A\x1A\0\0\x90\x1A\0\0\x9A\x1A\0\0P\x1B\0\0Z\x1B\0\0\xB0\x1B\0\0\xBA\x1B\0\0@\x1C\0\0J\x1C\0\0P\x1C\0\0Z\x1C\0\0 \xA6\0\0*\xA6\0\0\xD0\xA8\0\0\xDA\xA8\0\0\0\xA9\0\0\n\xA9\0\0\xD0\xA9\0\0\xDA\xA9\0\0\xF0\xA9\0\0\xFA\xA9\0\0P\xAA\0\0Z\xAA\0\0\xF0\xAB\0\0\xFA\xAB\0\0\x10\xFF\0\0\x1A\xFF\0\0!\xFF\0\0'\xFF\0\0A\xFF\0\0G\xFF\0\0\xA0\x04\x01\0\xAA\x04\x01\x000\r\x01\0:\r\x01\0f\x10\x01\0p\x10\x01\0\xF0\x10\x01\0\xFA\x10\x01\x006\x11\x01\0@\x11\x01\0\xD0\x11\x01\0\xDA\x11\x01\0\xF0\x12\x01\0\xFA\x12\x01\0P\x14\x01\0Z\x14\x01\0\xD0\x14\x01\0\xDA\x14\x01\0P\x16\x01\0Z\x16\x01\0\xC0\x16\x01\0\xCA\x16\x01\x000\x17\x01\0:\x17\x01\0\xE0\x18\x01\0\xEA\x18\x01\0P\x19\x01\0Z\x19\x01\0P\x1C\x01\0Z\x1C\x01\0P\x1D\x01\0Z\x1D\x01\0\xA0\x1D\x01\0\xAA\x1D\x01\0P\x1F\x01\0Z\x1F\x01\0`j\x01\0jj\x01\0\xC0j\x01\0\xCAj\x01\0Pk\x01\0Zk\x01\0\xCE\xD7\x01\0\0\xD8\x01\0@\xE1\x01\0J\xE1\x01\0\xF0\xE2\x01\0\xFA\xE2\x01\0\xF0\xE4\x01\0\xFA\xE4\x01\0P\xE9\x01\0Z\xE9\x01\0\xF0\xFB\x01\0\xFA\xFB\x01\0") }, 704u32)
            });
        }
        #[clippy::msrv = "1.65"]
        impl icu_provider::DataProvider<icu_properties::provider::XdigitV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_properties::provider::XdigitV1Marker>, icu_provider::DataError> {
                if req.locale.is_empty() {
                    Ok(icu_provider::DataResponse { payload: Some(icu_provider::DataPayload::from_static_ref(Self::SINGLETON_PROPS_XDIGIT_V1)), metadata: Default::default() })
                } else {
                    Err(icu_provider::DataErrorKind::ExtraneousLocale.with_req(<icu_properties::provider::XdigitV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
                }
            }
        }
    };
}
