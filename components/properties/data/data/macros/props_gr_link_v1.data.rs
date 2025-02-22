// @generated
/// Implement `DataProvider<GraphemeLinkV1Marker>` on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_props_gr_link_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.65"]
        impl $provider {
            #[doc(hidden)]
            pub const SINGLETON_PROPS_GR_LINK_V1: &'static <icu_properties::provider::GraphemeLinkV1Marker as icu_provider::DataMarker>::Yokeable = &icu_properties::provider::PropertyCodePointSetV1::InversionList(unsafe {
                #[allow(unused_unsafe)]
                icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"M\t\0\0N\t\0\0\xCD\t\0\0\xCE\t\0\0M\n\0\0N\n\0\0\xCD\n\0\0\xCE\n\0\0M\x0B\0\0N\x0B\0\0\xCD\x0B\0\0\xCE\x0B\0\0M\x0C\0\0N\x0C\0\0\xCD\x0C\0\0\xCE\x0C\0\0;\r\0\0=\r\0\0M\r\0\0N\r\0\0\xCA\r\0\0\xCB\r\0\0:\x0E\0\0;\x0E\0\0\xBA\x0E\0\0\xBB\x0E\0\0\x84\x0F\0\0\x85\x0F\0\09\x10\0\0;\x10\0\0\x14\x17\0\0\x16\x17\0\x004\x17\0\x005\x17\0\0\xD2\x17\0\0\xD3\x17\0\0`\x1A\0\0a\x1A\0\0D\x1B\0\0E\x1B\0\0\xAA\x1B\0\0\xAC\x1B\0\0\xF2\x1B\0\0\xF4\x1B\0\0\x7F-\0\0\x80-\0\0\x06\xA8\0\0\x07\xA8\0\0,\xA8\0\0-\xA8\0\0\xC4\xA8\0\0\xC5\xA8\0\0S\xA9\0\0T\xA9\0\0\xC0\xA9\0\0\xC1\xA9\0\0\xF6\xAA\0\0\xF7\xAA\0\0\xED\xAB\0\0\xEE\xAB\0\0?\n\x01\0@\n\x01\0F\x10\x01\0G\x10\x01\0p\x10\x01\0q\x10\x01\0\x7F\x10\x01\0\x80\x10\x01\0\xB9\x10\x01\0\xBA\x10\x01\x003\x11\x01\x005\x11\x01\0\xC0\x11\x01\0\xC1\x11\x01\x005\x12\x01\x006\x12\x01\0\xEA\x12\x01\0\xEB\x12\x01\0M\x13\x01\0N\x13\x01\0B\x14\x01\0C\x14\x01\0\xC2\x14\x01\0\xC3\x14\x01\0\xBF\x15\x01\0\xC0\x15\x01\0?\x16\x01\0@\x16\x01\0\xB6\x16\x01\0\xB7\x16\x01\0+\x17\x01\0,\x17\x01\09\x18\x01\0:\x18\x01\0=\x19\x01\0?\x19\x01\0\xE0\x19\x01\0\xE1\x19\x01\x004\x1A\x01\x005\x1A\x01\0G\x1A\x01\0H\x1A\x01\0\x99\x1A\x01\0\x9A\x1A\x01\0?\x1C\x01\0@\x1C\x01\0D\x1D\x01\0F\x1D\x01\0\x97\x1D\x01\0\x98\x1D\x01\0A\x1F\x01\0C\x1F\x01\0") }, 65u32)
            });
        }
        #[clippy::msrv = "1.65"]
        impl icu_provider::DataProvider<icu_properties::provider::GraphemeLinkV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_properties::provider::GraphemeLinkV1Marker>, icu_provider::DataError> {
                if req.locale.is_empty() {
                    Ok(icu_provider::DataResponse { payload: Some(icu_provider::DataPayload::from_static_ref(Self::SINGLETON_PROPS_GR_LINK_V1)), metadata: Default::default() })
                } else {
                    Err(icu_provider::DataErrorKind::ExtraneousLocale.with_req(<icu_properties::provider::GraphemeLinkV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
                }
            }
        }
    };
}
