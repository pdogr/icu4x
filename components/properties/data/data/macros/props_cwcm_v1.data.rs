// @generated
/// Implement `DataProvider<ChangesWhenCasemappedV1Marker>` on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_props_cwcm_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.65"]
        impl $provider {
            #[doc(hidden)]
            pub const SINGLETON_PROPS_CWCM_V1: &'static <icu_properties::provider::ChangesWhenCasemappedV1Marker as icu_provider::DataMarker>::Yokeable = &icu_properties::provider::PropertyCodePointSetV1::InversionList(unsafe {
                #[allow(unused_unsafe)]
                icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"A\0\0\0[\0\0\0a\0\0\0{\0\0\0\xB5\0\0\0\xB6\0\0\0\xC0\0\0\0\xD7\0\0\0\xD8\0\0\0\xF7\0\0\0\xF8\0\0\08\x01\0\09\x01\0\0\x8D\x01\0\0\x8E\x01\0\0\x9B\x01\0\0\x9C\x01\0\0\xAA\x01\0\0\xAC\x01\0\0\xBA\x01\0\0\xBC\x01\0\0\xBE\x01\0\0\xBF\x01\0\0\xC0\x01\0\0\xC4\x01\0\0!\x02\0\0\"\x02\0\x004\x02\0\0:\x02\0\0U\x02\0\0V\x02\0\0X\x02\0\0Y\x02\0\0Z\x02\0\0[\x02\0\0]\x02\0\0`\x02\0\0b\x02\0\0c\x02\0\0d\x02\0\0e\x02\0\0g\x02\0\0h\x02\0\0m\x02\0\0o\x02\0\0p\x02\0\0q\x02\0\0s\x02\0\0u\x02\0\0v\x02\0\0}\x02\0\0~\x02\0\0\x80\x02\0\0\x81\x02\0\0\x82\x02\0\0\x84\x02\0\0\x87\x02\0\0\x8D\x02\0\0\x92\x02\0\0\x93\x02\0\0\x9D\x02\0\0\x9F\x02\0\0E\x03\0\0F\x03\0\0p\x03\0\0t\x03\0\0v\x03\0\0x\x03\0\0{\x03\0\0~\x03\0\0\x7F\x03\0\0\x80\x03\0\0\x86\x03\0\0\x87\x03\0\0\x88\x03\0\0\x8B\x03\0\0\x8C\x03\0\0\x8D\x03\0\0\x8E\x03\0\0\xA2\x03\0\0\xA3\x03\0\0\xD2\x03\0\0\xD5\x03\0\0\xF6\x03\0\0\xF7\x03\0\0\xFC\x03\0\0\xFD\x03\0\0\x82\x04\0\0\x8A\x04\0\x000\x05\0\x001\x05\0\0W\x05\0\0a\x05\0\0\x88\x05\0\0\xA0\x10\0\0\xC6\x10\0\0\xC7\x10\0\0\xC8\x10\0\0\xCD\x10\0\0\xCE\x10\0\0\xD0\x10\0\0\xFB\x10\0\0\xFD\x10\0\0\0\x11\0\0\xA0\x13\0\0\xF6\x13\0\0\xF8\x13\0\0\xFE\x13\0\0\x80\x1C\0\0\x89\x1C\0\0\x90\x1C\0\0\xBB\x1C\0\0\xBD\x1C\0\0\xC0\x1C\0\0y\x1D\0\0z\x1D\0\0}\x1D\0\0~\x1D\0\0\x8E\x1D\0\0\x8F\x1D\0\0\0\x1E\0\0\x9C\x1E\0\0\x9E\x1E\0\0\x9F\x1E\0\0\xA0\x1E\0\0\x16\x1F\0\0\x18\x1F\0\0\x1E\x1F\0\0 \x1F\0\0F\x1F\0\0H\x1F\0\0N\x1F\0\0P\x1F\0\0X\x1F\0\0Y\x1F\0\0Z\x1F\0\0[\x1F\0\0\\\x1F\0\0]\x1F\0\0^\x1F\0\0_\x1F\0\0~\x1F\0\0\x80\x1F\0\0\xB5\x1F\0\0\xB6\x1F\0\0\xBD\x1F\0\0\xBE\x1F\0\0\xBF\x1F\0\0\xC2\x1F\0\0\xC5\x1F\0\0\xC6\x1F\0\0\xCD\x1F\0\0\xD0\x1F\0\0\xD4\x1F\0\0\xD6\x1F\0\0\xDC\x1F\0\0\xE0\x1F\0\0\xED\x1F\0\0\xF2\x1F\0\0\xF5\x1F\0\0\xF6\x1F\0\0\xFD\x1F\0\0&!\0\0'!\0\0*!\0\0,!\0\x002!\0\x003!\0\0N!\0\0O!\0\0`!\0\0\x80!\0\0\x83!\0\0\x85!\0\0\xB6$\0\0\xEA$\0\0\0,\0\0q,\0\0r,\0\0t,\0\0u,\0\0w,\0\0~,\0\0\xE4,\0\0\xEB,\0\0\xEF,\0\0\xF2,\0\0\xF4,\0\0\0-\0\0&-\0\0'-\0\0(-\0\0--\0\0.-\0\0@\xA6\0\0n\xA6\0\0\x80\xA6\0\0\x9C\xA6\0\0\"\xA7\0\x000\xA7\0\x002\xA7\0\0p\xA7\0\0y\xA7\0\0\x88\xA7\0\0\x8B\xA7\0\0\x8E\xA7\0\0\x90\xA7\0\0\x95\xA7\0\0\x96\xA7\0\0\xAF\xA7\0\0\xB0\xA7\0\0\xCB\xA7\0\0\xD0\xA7\0\0\xD2\xA7\0\0\xD6\xA7\0\0\xDA\xA7\0\0\xF5\xA7\0\0\xF7\xA7\0\0S\xAB\0\0T\xAB\0\0p\xAB\0\0\xC0\xAB\0\0\0\xFB\0\0\x07\xFB\0\0\x13\xFB\0\0\x18\xFB\0\0!\xFF\0\0;\xFF\0\0A\xFF\0\0[\xFF\0\0\0\x04\x01\0P\x04\x01\0\xB0\x04\x01\0\xD4\x04\x01\0\xD8\x04\x01\0\xFC\x04\x01\0p\x05\x01\0{\x05\x01\0|\x05\x01\0\x8B\x05\x01\0\x8C\x05\x01\0\x93\x05\x01\0\x94\x05\x01\0\x96\x05\x01\0\x97\x05\x01\0\xA2\x05\x01\0\xA3\x05\x01\0\xB2\x05\x01\0\xB3\x05\x01\0\xBA\x05\x01\0\xBB\x05\x01\0\xBD\x05\x01\0\x80\x0C\x01\0\xB3\x0C\x01\0\xC0\x0C\x01\0\xF3\x0C\x01\0\xA0\x18\x01\0\xE0\x18\x01\0@n\x01\0\x80n\x01\0\0\xE9\x01\0D\xE9\x01\0") }, 2927u32)
            });
        }
        #[clippy::msrv = "1.65"]
        impl icu_provider::DataProvider<icu_properties::provider::ChangesWhenCasemappedV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_properties::provider::ChangesWhenCasemappedV1Marker>, icu_provider::DataError> {
                if req.locale.is_empty() {
                    Ok(icu_provider::DataResponse { payload: Some(icu_provider::DataPayload::from_static_ref(Self::SINGLETON_PROPS_CWCM_V1)), metadata: Default::default() })
                } else {
                    Err(icu_provider::DataErrorKind::ExtraneousLocale.with_req(<icu_properties::provider::ChangesWhenCasemappedV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
                }
            }
        }
    };
}
