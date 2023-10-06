// @generated
/// Implement `DataProvider<DiacriticV1Marker>` on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_props_dia_v1 {
    ($ provider : ty) => {
        #[clippy::msrv = "1.67"]
        const _: () = <$provider>::MUST_USE_MAKE_PROVIDER_MACRO;
        #[clippy::msrv = "1.67"]
        impl $provider {
            #[doc(hidden)]
            pub const SINGLETON_PROPS_DIA_V1: &'static <icu::properties::provider::DiacriticV1Marker as icu_provider::DataMarker>::Yokeable = &icu::properties::provider::PropertyCodePointSetV1::InversionList(unsafe {
                #[allow(unused_unsafe)]
                icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"^\0\0\0_\0\0\0`\0\0\0a\0\0\0\xA8\0\0\0\xA9\0\0\0\xAF\0\0\0\xB0\0\0\0\xB4\0\0\0\xB5\0\0\0\xB7\0\0\0\xB9\0\0\0\xB0\x02\0\0O\x03\0\0P\x03\0\0X\x03\0\0]\x03\0\0c\x03\0\0t\x03\0\0v\x03\0\0z\x03\0\0{\x03\0\0\x84\x03\0\0\x86\x03\0\0\x83\x04\0\0\x88\x04\0\0Y\x05\0\0Z\x05\0\0\x91\x05\0\0\xA2\x05\0\0\xA3\x05\0\0\xBE\x05\0\0\xBF\x05\0\0\xC0\x05\0\0\xC1\x05\0\0\xC3\x05\0\0\xC4\x05\0\0\xC5\x05\0\0K\x06\0\0S\x06\0\0W\x06\0\0Y\x06\0\0\xDF\x06\0\0\xE1\x06\0\0\xE5\x06\0\0\xE7\x06\0\0\xEA\x06\0\0\xED\x06\0\x000\x07\0\0K\x07\0\0\xA6\x07\0\0\xB1\x07\0\0\xEB\x07\0\0\xF6\x07\0\0\x18\x08\0\0\x1A\x08\0\0\x98\x08\0\0\xA0\x08\0\0\xC9\x08\0\0\xD3\x08\0\0\xE3\x08\0\0\xFF\x08\0\0<\t\0\0=\t\0\0M\t\0\0N\t\0\0Q\t\0\0U\t\0\0q\t\0\0r\t\0\0\xBC\t\0\0\xBD\t\0\0\xCD\t\0\0\xCE\t\0\0<\n\0\0=\n\0\0M\n\0\0N\n\0\0\xBC\n\0\0\xBD\n\0\0\xCD\n\0\0\xCE\n\0\0\xFD\n\0\0\0\x0B\0\0<\x0B\0\0=\x0B\0\0M\x0B\0\0N\x0B\0\0U\x0B\0\0V\x0B\0\0\xCD\x0B\0\0\xCE\x0B\0\0<\x0C\0\0=\x0C\0\0M\x0C\0\0N\x0C\0\0\xBC\x0C\0\0\xBD\x0C\0\0\xCD\x0C\0\0\xCE\x0C\0\0;\r\0\0=\r\0\0M\r\0\0N\r\0\0\xCA\r\0\0\xCB\r\0\0G\x0E\0\0M\x0E\0\0N\x0E\0\0O\x0E\0\0\xBA\x0E\0\0\xBB\x0E\0\0\xC8\x0E\0\0\xCD\x0E\0\0\x18\x0F\0\0\x1A\x0F\0\x005\x0F\0\x006\x0F\0\x007\x0F\0\08\x0F\0\09\x0F\0\0:\x0F\0\0>\x0F\0\0@\x0F\0\0\x82\x0F\0\0\x85\x0F\0\0\x86\x0F\0\0\x88\x0F\0\0\xC6\x0F\0\0\xC7\x0F\0\x007\x10\0\08\x10\0\09\x10\0\0;\x10\0\0c\x10\0\0e\x10\0\0i\x10\0\0n\x10\0\0\x87\x10\0\0\x8E\x10\0\0\x8F\x10\0\0\x90\x10\0\0\x9A\x10\0\0\x9C\x10\0\0]\x13\0\0`\x13\0\0\x14\x17\0\0\x16\x17\0\0\xC9\x17\0\0\xD4\x17\0\0\xDD\x17\0\0\xDE\x17\0\09\x19\0\0<\x19\0\0u\x1A\0\0}\x1A\0\0\x7F\x1A\0\0\x80\x1A\0\0\xB0\x1A\0\0\xBF\x1A\0\0\xC1\x1A\0\0\xCC\x1A\0\x004\x1B\0\x005\x1B\0\0D\x1B\0\0E\x1B\0\0k\x1B\0\0t\x1B\0\0\xAA\x1B\0\0\xAC\x1B\0\x006\x1C\0\08\x1C\0\0x\x1C\0\0~\x1C\0\0\xD0\x1C\0\0\xE9\x1C\0\0\xED\x1C\0\0\xEE\x1C\0\0\xF4\x1C\0\0\xF5\x1C\0\0\xF7\x1C\0\0\xFA\x1C\0\0,\x1D\0\0k\x1D\0\0\xC4\x1D\0\0\xD0\x1D\0\0\xF5\x1D\0\0\0\x1E\0\0\xBD\x1F\0\0\xBE\x1F\0\0\xBF\x1F\0\0\xC2\x1F\0\0\xCD\x1F\0\0\xD0\x1F\0\0\xDD\x1F\0\0\xE0\x1F\0\0\xED\x1F\0\0\xF0\x1F\0\0\xFD\x1F\0\0\xFF\x1F\0\0\xEF,\0\0\xF2,\0\0/.\0\x000.\0\0*0\0\x0000\0\0\x990\0\0\x9D0\0\0\xFC0\0\0\xFD0\0\0o\xA6\0\0p\xA6\0\0|\xA6\0\0~\xA6\0\0\x7F\xA6\0\0\x80\xA6\0\0\x9C\xA6\0\0\x9E\xA6\0\0\xF0\xA6\0\0\xF2\xA6\0\0\0\xA7\0\0\"\xA7\0\0\x88\xA7\0\0\x8B\xA7\0\0\xF8\xA7\0\0\xFA\xA7\0\0\xC4\xA8\0\0\xC5\xA8\0\0\xE0\xA8\0\0\xF2\xA8\0\0+\xA9\0\0/\xA9\0\0S\xA9\0\0T\xA9\0\0\xB3\xA9\0\0\xB4\xA9\0\0\xC0\xA9\0\0\xC1\xA9\0\0\xE5\xA9\0\0\xE6\xA9\0\0{\xAA\0\0~\xAA\0\0\xBF\xAA\0\0\xC3\xAA\0\0\xF6\xAA\0\0\xF7\xAA\0\0[\xAB\0\0`\xAB\0\0i\xAB\0\0l\xAB\0\0\xEC\xAB\0\0\xEE\xAB\0\0\x1E\xFB\0\0\x1F\xFB\0\0 \xFE\0\x000\xFE\0\0>\xFF\0\0?\xFF\0\0@\xFF\0\0A\xFF\0\0p\xFF\0\0q\xFF\0\0\x9E\xFF\0\0\xA0\xFF\0\0\xE3\xFF\0\0\xE4\xFF\0\0\xE0\x02\x01\0\xE1\x02\x01\0\x80\x07\x01\0\x86\x07\x01\0\x87\x07\x01\0\xB1\x07\x01\0\xB2\x07\x01\0\xBB\x07\x01\0\xE5\n\x01\0\xE7\n\x01\0\"\r\x01\0(\r\x01\0\xFD\x0E\x01\0\0\x0F\x01\0F\x0F\x01\0Q\x0F\x01\0\x82\x0F\x01\0\x86\x0F\x01\0F\x10\x01\0G\x10\x01\0p\x10\x01\0q\x10\x01\0\xB9\x10\x01\0\xBB\x10\x01\x003\x11\x01\x005\x11\x01\0s\x11\x01\0t\x11\x01\0\xC0\x11\x01\0\xC1\x11\x01\0\xCA\x11\x01\0\xCD\x11\x01\x005\x12\x01\x007\x12\x01\0\xE9\x12\x01\0\xEB\x12\x01\0<\x13\x01\0=\x13\x01\0M\x13\x01\0N\x13\x01\0f\x13\x01\0m\x13\x01\0p\x13\x01\0u\x13\x01\0B\x14\x01\0C\x14\x01\0F\x14\x01\0G\x14\x01\0\xC2\x14\x01\0\xC4\x14\x01\0\xBF\x15\x01\0\xC1\x15\x01\0?\x16\x01\0@\x16\x01\0\xB6\x16\x01\0\xB8\x16\x01\0+\x17\x01\0,\x17\x01\09\x18\x01\0;\x18\x01\0=\x19\x01\0?\x19\x01\0C\x19\x01\0D\x19\x01\0\xE0\x19\x01\0\xE1\x19\x01\x004\x1A\x01\x005\x1A\x01\0G\x1A\x01\0H\x1A\x01\0\x99\x1A\x01\0\x9A\x1A\x01\0?\x1C\x01\0@\x1C\x01\0B\x1D\x01\0C\x1D\x01\0D\x1D\x01\0F\x1D\x01\0\x97\x1D\x01\0\x98\x1D\x01\0G4\x01\0V4\x01\0\xF0j\x01\0\xF5j\x01\x000k\x01\x007k\x01\0\x8Fo\x01\0\xA0o\x01\0\xF0o\x01\0\xF2o\x01\0\xF0\xAF\x01\0\xF4\xAF\x01\0\xF5\xAF\x01\0\xFC\xAF\x01\0\xFD\xAF\x01\0\xFF\xAF\x01\0\0\xCF\x01\0.\xCF\x01\x000\xCF\x01\0G\xCF\x01\0g\xD1\x01\0j\xD1\x01\0m\xD1\x01\0s\xD1\x01\0{\xD1\x01\0\x83\xD1\x01\0\x85\xD1\x01\0\x8C\xD1\x01\0\xAA\xD1\x01\0\xAE\xD1\x01\x000\xE0\x01\0n\xE0\x01\x000\xE1\x01\x007\xE1\x01\0\xAE\xE2\x01\0\xAF\xE2\x01\0\xEC\xE2\x01\0\xF0\xE2\x01\0\xD0\xE8\x01\0\xD7\xE8\x01\0D\xE9\x01\0G\xE9\x01\0H\xE9\x01\0K\xE9\x01\0") }, 1144u32)
            });
        }
        #[clippy::msrv = "1.67"]
        impl icu_provider::DataProvider<icu::properties::provider::DiacriticV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu::properties::provider::DiacriticV1Marker>, icu_provider::DataError> {
                if req.locale.is_empty() {
                    Ok(icu_provider::DataResponse { payload: Some(icu_provider::DataPayload::from_static_ref(Self::SINGLETON_PROPS_DIA_V1)), metadata: Default::default() })
                } else {
                    Err(icu_provider::DataErrorKind::ExtraneousLocale.with_req(<icu::properties::provider::DiacriticV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
                }
            }
        }
    };
}
