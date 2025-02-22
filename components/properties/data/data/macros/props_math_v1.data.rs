// @generated
/// Implement `DataProvider<MathV1Marker>` on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_props_math_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.65"]
        impl $provider {
            #[doc(hidden)]
            pub const SINGLETON_PROPS_MATH_V1: &'static <icu_properties::provider::MathV1Marker as icu_provider::DataMarker>::Yokeable = &icu_properties::provider::PropertyCodePointSetV1::InversionList(unsafe {
                #[allow(unused_unsafe)]
                icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"+\0\0\0,\0\0\0<\0\0\0?\0\0\0^\0\0\0_\0\0\0|\0\0\0}\0\0\0~\0\0\0\x7F\0\0\0\xAC\0\0\0\xAD\0\0\0\xB1\0\0\0\xB2\0\0\0\xD7\0\0\0\xD8\0\0\0\xF7\0\0\0\xF8\0\0\0\xD0\x03\0\0\xD3\x03\0\0\xD5\x03\0\0\xD6\x03\0\0\xF0\x03\0\0\xF2\x03\0\0\xF4\x03\0\0\xF7\x03\0\0\x06\x06\0\0\t\x06\0\0\x16 \0\0\x17 \0\x002 \0\x005 \0\0@ \0\0A \0\0D \0\0E \0\0R \0\0S \0\0a \0\0e \0\0z \0\0\x7F \0\0\x8A \0\0\x8F \0\0\xD0 \0\0\xDD \0\0\xE1 \0\0\xE2 \0\0\xE5 \0\0\xE7 \0\0\xEB \0\0\xF0 \0\0\x02!\0\0\x03!\0\0\x07!\0\0\x08!\0\0\n!\0\0\x14!\0\0\x15!\0\0\x16!\0\0\x18!\0\0\x1E!\0\0$!\0\0%!\0\0(!\0\0*!\0\0,!\0\0.!\0\0/!\0\x002!\0\x003!\0\09!\0\0<!\0\0J!\0\0K!\0\0L!\0\0\x90!\0\0\xA8!\0\0\xA9!\0\0\xAF!\0\0\xB0!\0\0\xB2!\0\0\xB6!\0\0\xB8!\0\0\xBC!\0\0\xDC!\0\0\xDD!\0\0\xDE!\0\0\xE4!\0\0\xE6!\0\0\xF4!\0\0\0#\0\0\x08#\0\0\x0C#\0\0 #\0\0\"#\0\0|#\0\0}#\0\0\x9B#\0\0\xB6#\0\0\xB7#\0\0\xB8#\0\0\xD0#\0\0\xD1#\0\0\xDC#\0\0\xE3#\0\0\xA0%\0\0\xA2%\0\0\xAE%\0\0\xB8%\0\0\xBC%\0\0\xC2%\0\0\xC6%\0\0\xC8%\0\0\xCA%\0\0\xCC%\0\0\xCF%\0\0\xD4%\0\0\xE2%\0\0\xE3%\0\0\xE4%\0\0\xE5%\0\0\xE7%\0\0\xED%\0\0\xF8%\0\0\0&\0\0\x05&\0\0\x07&\0\0@&\0\0A&\0\0B&\0\0C&\0\0`&\0\0d&\0\0m&\0\0p&\0\0\xC0'\0\0\0(\0\0\0)\0\0\0+\0\x000+\0\0E+\0\0G+\0\0M+\0\0)\xFB\0\0*\xFB\0\0a\xFE\0\0g\xFE\0\0h\xFE\0\0i\xFE\0\0\x0B\xFF\0\0\x0C\xFF\0\0\x1C\xFF\0\0\x1F\xFF\0\0<\xFF\0\0=\xFF\0\0>\xFF\0\0?\xFF\0\0\\\xFF\0\0]\xFF\0\0^\xFF\0\0_\xFF\0\0\xE2\xFF\0\0\xE3\xFF\0\0\xE9\xFF\0\0\xED\xFF\0\0\0\xD4\x01\0U\xD4\x01\0V\xD4\x01\0\x9D\xD4\x01\0\x9E\xD4\x01\0\xA0\xD4\x01\0\xA2\xD4\x01\0\xA3\xD4\x01\0\xA5\xD4\x01\0\xA7\xD4\x01\0\xA9\xD4\x01\0\xAD\xD4\x01\0\xAE\xD4\x01\0\xBA\xD4\x01\0\xBB\xD4\x01\0\xBC\xD4\x01\0\xBD\xD4\x01\0\xC4\xD4\x01\0\xC5\xD4\x01\0\x06\xD5\x01\0\x07\xD5\x01\0\x0B\xD5\x01\0\r\xD5\x01\0\x15\xD5\x01\0\x16\xD5\x01\0\x1D\xD5\x01\0\x1E\xD5\x01\0:\xD5\x01\0;\xD5\x01\0?\xD5\x01\0@\xD5\x01\0E\xD5\x01\0F\xD5\x01\0G\xD5\x01\0J\xD5\x01\0Q\xD5\x01\0R\xD5\x01\0\xA6\xD6\x01\0\xA8\xD6\x01\0\xCC\xD7\x01\0\xCE\xD7\x01\0\0\xD8\x01\0\0\xEE\x01\0\x04\xEE\x01\0\x05\xEE\x01\0 \xEE\x01\0!\xEE\x01\0#\xEE\x01\0$\xEE\x01\0%\xEE\x01\0'\xEE\x01\0(\xEE\x01\0)\xEE\x01\x003\xEE\x01\x004\xEE\x01\08\xEE\x01\09\xEE\x01\0:\xEE\x01\0;\xEE\x01\0<\xEE\x01\0B\xEE\x01\0C\xEE\x01\0G\xEE\x01\0H\xEE\x01\0I\xEE\x01\0J\xEE\x01\0K\xEE\x01\0L\xEE\x01\0M\xEE\x01\0P\xEE\x01\0Q\xEE\x01\0S\xEE\x01\0T\xEE\x01\0U\xEE\x01\0W\xEE\x01\0X\xEE\x01\0Y\xEE\x01\0Z\xEE\x01\0[\xEE\x01\0\\\xEE\x01\0]\xEE\x01\0^\xEE\x01\0_\xEE\x01\0`\xEE\x01\0a\xEE\x01\0c\xEE\x01\0d\xEE\x01\0e\xEE\x01\0g\xEE\x01\0k\xEE\x01\0l\xEE\x01\0s\xEE\x01\0t\xEE\x01\0x\xEE\x01\0y\xEE\x01\0}\xEE\x01\0~\xEE\x01\0\x7F\xEE\x01\0\x80\xEE\x01\0\x8A\xEE\x01\0\x8B\xEE\x01\0\x9C\xEE\x01\0\xA1\xEE\x01\0\xA4\xEE\x01\0\xA5\xEE\x01\0\xAA\xEE\x01\0\xAB\xEE\x01\0\xBC\xEE\x01\0\xF0\xEE\x01\0\xF2\xEE\x01\0") }, 2310u32)
            });
        }
        #[clippy::msrv = "1.65"]
        impl icu_provider::DataProvider<icu_properties::provider::MathV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_properties::provider::MathV1Marker>, icu_provider::DataError> {
                if req.locale.is_empty() {
                    Ok(icu_provider::DataResponse { payload: Some(icu_provider::DataPayload::from_static_ref(Self::SINGLETON_PROPS_MATH_V1)), metadata: Default::default() })
                } else {
                    Err(icu_provider::DataErrorKind::ExtraneousLocale.with_req(<icu_properties::provider::MathV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
                }
            }
        }
    };
}
