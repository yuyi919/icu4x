// @generated
/// Implement `DataProvider<DashV1Marker>` on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_props_dash_v1 {
    ($ provider : ty) => {
        #[clippy::msrv = "1.67"]
        const _: () = <$provider>::MUST_USE_MAKE_PROVIDER_MACRO;
        #[clippy::msrv = "1.67"]
        impl $provider {
            #[doc(hidden)]
            pub const SINGLETON_PROPS_DASH_V1: &'static <icu::properties::provider::DashV1Marker as icu_provider::DataMarker>::Yokeable = &icu::properties::provider::PropertyCodePointSetV1::InversionList(unsafe {
                #[allow(unused_unsafe)]
                icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"-\0\0\0.\0\0\0\x8A\x05\0\0\x8B\x05\0\0\xBE\x05\0\0\xBF\x05\0\0\0\x14\0\0\x01\x14\0\0\x06\x18\0\0\x07\x18\0\0\x10 \0\0\x16 \0\0S \0\0T \0\0{ \0\0| \0\0\x8B \0\0\x8C \0\0\x12\"\0\0\x13\"\0\0\x17.\0\0\x18.\0\0\x1A.\0\0\x1B.\0\0:.\0\0<.\0\0@.\0\0A.\0\0].\0\0^.\0\0\x1C0\0\0\x1D0\0\x0000\0\x0010\0\0\xA00\0\0\xA10\0\x001\xFE\0\x003\xFE\0\0X\xFE\0\0Y\xFE\0\0c\xFE\0\0d\xFE\0\0\r\xFF\0\0\x0E\xFF\0\0\xAD\x0E\x01\0\xAE\x0E\x01\0") }, 30u32)
            });
        }
        #[clippy::msrv = "1.67"]
        impl icu_provider::DataProvider<icu::properties::provider::DashV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu::properties::provider::DashV1Marker>, icu_provider::DataError> {
                if req.locale.is_empty() {
                    Ok(icu_provider::DataResponse { payload: Some(icu_provider::DataPayload::from_static_ref(Self::SINGLETON_PROPS_DASH_V1)), metadata: Default::default() })
                } else {
                    Err(icu_provider::DataErrorKind::ExtraneousLocale.with_req(<icu::properties::provider::DashV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
                }
            }
        }
    };
}
