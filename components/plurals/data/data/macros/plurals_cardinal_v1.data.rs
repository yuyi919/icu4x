// @generated
/// Implement `DataProvider<CardinalV1Marker>` on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_plurals_cardinal_v1 {
    ($ provider : ty) => {
        #[clippy::msrv = "1.67"]
        const _: () = <$provider>::MUST_USE_MAKE_PROVIDER_MACRO;
        #[clippy::msrv = "1.67"]
        impl icu_provider::DataProvider<icu::plurals::provider::CardinalV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu::plurals::provider::CardinalV1Marker>, icu_provider::DataError> {
                static UND: <icu::plurals::provider::CardinalV1Marker as icu_provider::DataMarker>::Yokeable = icu::plurals::provider::PluralRulesV1 { zero: None, one: None, two: None, few: None, many: None };
                static BHO: <icu::plurals::provider::CardinalV1Marker as icu_provider::DataMarker>::Yokeable = icu::plurals::provider::PluralRulesV1 { zero: None, one: Some(icu::plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC0\0\0\0\0\0\0\0\0\x01\0\0\0") })), two: None, few: None, many: None };
                static AF: <icu::plurals::provider::CardinalV1Marker as icu_provider::DataMarker>::Yokeable = icu::plurals::provider::PluralRulesV1 { zero: None, one: Some(icu::plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC0\0\0\0\0\x01\0\0\0\x01\0\0\0") })), two: None, few: None, many: None };
                static ES: <icu::plurals::provider::CardinalV1Marker as icu_provider::DataMarker>::Yokeable = icu::plurals::provider::PluralRulesV1 { zero: None, one: Some(icu::plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC0\0\0\0\0\x01\0\0\0\x01\0\0\0") })), two: None, few: None, many: Some(icu::plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x05\0\0\0\0\0\r\0\x1A\0'\x004\0\xC7\0\0\0\0\0\0\0\0\0\0\0\0\x81\0\0\0\0\0\0\0\0\0\0\0\0\xC1@B\x0F\0\0\0\0\0\0\0\0\0\xC2\0\0\0\0\0\0\0\0\0\0\0\0\x07\0\0\0\0\0\0\0\0\x05\0\0\0") })) };
                static SAT: <icu::plurals::provider::CardinalV1Marker as icu_provider::DataMarker>::Yokeable = icu::plurals::provider::PluralRulesV1 { zero: None, one: Some(icu::plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC0\0\0\0\0\x01\0\0\0\x01\0\0\0") })), two: Some(icu::plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC0\0\0\0\0\x02\0\0\0\x02\0\0\0") })), few: None, many: None };
                static GA: <icu::plurals::provider::CardinalV1Marker as icu_provider::DataMarker>::Yokeable = icu::plurals::provider::PluralRulesV1 { zero: None, one: Some(icu::plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC0\0\0\0\0\x01\0\0\0\x01\0\0\0") })), two: Some(icu::plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC0\0\0\0\0\x02\0\0\0\x02\0\0\0") })), few: Some(icu::plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC0\0\0\0\0\x03\0\0\0\x06\0\0\0") })), many: Some(icu::plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC0\0\0\0\0\x07\0\0\0\n\0\0\0") })) };
                static GD: <icu::plurals::provider::CardinalV1Marker as icu_provider::DataMarker>::Yokeable = icu::plurals::provider::PluralRulesV1 { zero: None, one: Some(icu::plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC0\0\0\0\0\x01\0\0\0\x01\0\0\0\x0B\0\0\0\x0B\0\0\0") })), two: Some(icu::plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC0\0\0\0\0\x02\0\0\0\x02\0\0\0\x0C\0\0\0\x0C\0\0\0") })), few: Some(icu::plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC0\0\0\0\0\x03\0\0\0\n\0\0\0\r\0\0\0\x13\0\0\0") })), many: None };
                static HY: <icu::plurals::provider::CardinalV1Marker as icu_provider::DataMarker>::Yokeable = icu::plurals::provider::PluralRulesV1 { zero: None, one: Some(icu::plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC1\0\0\0\0\0\0\0\0\0\0\0\0\x01\0\0\0\x01\0\0\0") })), two: None, few: None, many: None };
                static FR: <icu::plurals::provider::CardinalV1Marker as icu_provider::DataMarker>::Yokeable = icu::plurals::provider::PluralRulesV1 { zero: None, one: Some(icu::plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC1\0\0\0\0\0\0\0\0\0\0\0\0\x01\0\0\0\x01\0\0\0") })), two: None, few: None, many: Some(icu::plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x05\0\0\0\0\0\r\0\x1A\0'\x004\0\xC7\0\0\0\0\0\0\0\0\0\0\0\0\x81\0\0\0\0\0\0\0\0\0\0\0\0\xC1@B\x0F\0\0\0\0\0\0\0\0\0\xC2\0\0\0\0\0\0\0\0\0\0\0\0\x07\0\0\0\0\0\0\0\0\x05\0\0\0") })) };
                static PT: <icu::plurals::provider::CardinalV1Marker as icu_provider::DataMarker>::Yokeable = icu::plurals::provider::PluralRulesV1 { zero: None, one: Some(icu::plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC1\0\0\0\0\0\0\0\0\x01\0\0\0") })), two: None, few: None, many: Some(icu::plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x05\0\0\0\0\0\r\0\x1A\0'\x004\0\xC7\0\0\0\0\0\0\0\0\0\0\0\0\x81\0\0\0\0\0\0\0\0\0\0\0\0\xC1@B\x0F\0\0\0\0\0\0\0\0\0\xC2\0\0\0\0\0\0\0\0\0\0\0\0\x07\0\0\0\0\0\0\0\0\x05\0\0\0") })) };
                static BE: <icu::plurals::provider::CardinalV1Marker as icu_provider::DataMarker>::Yokeable = icu::plurals::provider::PluralRulesV1 { zero: None, one: Some(icu::plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\r\0\xC0\n\0\0\0\x01\0\0\0\x01\0\0\0\x80d\0\0\0\x0B\0\0\0\x0B\0\0\0") })), two: None, few: Some(icu::plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\r\0\xC0\n\0\0\0\x02\0\0\0\x04\0\0\0\x80d\0\0\0\x0C\0\0\0\x0E\0\0\0") })), many: Some(icu::plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\r\0\x1A\0\xC0\n\0\0\0\0\0\0\0\0\0\0\0@\n\0\0\0\x05\0\0\0\t\0\0\0@d\0\0\0\x0B\0\0\0\x0E\0\0\0") })) };
                static BR: <icu::plurals::provider::CardinalV1Marker as icu_provider::DataMarker>::Yokeable = icu::plurals::provider::PluralRulesV1 { zero: None, one: Some(icu::plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\r\0\xC0\n\0\0\0\x01\0\0\0\x01\0\0\0\x80d\0\0\0\x0B\0\0\0\x0B\0\0\0G\0\0\0G\0\0\0[\0\0\0[\0\0\0") })), two: Some(icu::plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\r\0\xC0\n\0\0\0\x02\0\0\0\x02\0\0\0\x80d\0\0\0\x0C\0\0\0\x0C\0\0\0H\0\0\0H\0\0\0\\\0\0\0\\\0\0\0") })), few: Some(icu::plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x15\0\xC0\n\0\0\0\x03\0\0\0\x04\0\0\0\t\0\0\0\t\0\0\0\x80d\0\0\0\n\0\0\0\x13\0\0\0F\0\0\0O\0\0\0Z\0\0\0c\0\0\0") })), many: Some(icu::plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\r\0\x80\0\0\0\0\0\0\0\0\0\0\0\0\xC0@B\x0F\0\0\0\0\0\0\0\0\0") })) };
                static LT: <icu::plurals::provider::CardinalV1Marker as icu_provider::DataMarker>::Yokeable = icu::plurals::provider::PluralRulesV1 { zero: None, one: Some(icu::plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\r\0\xC0\n\0\0\0\x01\0\0\0\x01\0\0\0\x80d\0\0\0\x0B\0\0\0\x13\0\0\0") })), two: None, few: Some(icu::plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\r\0\xC0\n\0\0\0\x02\0\0\0\t\0\0\0\x80d\0\0\0\x0B\0\0\0\x13\0\0\0") })), many: Some(icu::plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\x84\0\0\0\0\0\0\0\0\0\0\0\0") })) };
                static AM: <icu::plurals::provider::CardinalV1Marker as icu_provider::DataMarker>::Yokeable = icu::plurals::provider::PluralRulesV1 { zero: None, one: Some(icu::plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\r\0\xC1\0\0\0\0\0\0\0\0\0\0\0\0@\0\0\0\0\x01\0\0\0\x01\0\0\0") })), two: None, few: None, many: None };
                static AST: <icu::plurals::provider::CardinalV1Marker as icu_provider::DataMarker>::Yokeable = icu::plurals::provider::PluralRulesV1 { zero: None, one: Some(icu::plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\r\0\xC1\0\0\0\0\x01\0\0\0\x01\0\0\0\xC2\0\0\0\0\0\0\0\0\0\0\0\0") })), two: None, few: None, many: None };
                static CA: <icu::plurals::provider::CardinalV1Marker as icu_provider::DataMarker>::Yokeable = icu::plurals::provider::PluralRulesV1 { zero: None, one: Some(icu::plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\r\0\xC1\0\0\0\0\x01\0\0\0\x01\0\0\0\xC2\0\0\0\0\0\0\0\0\0\0\0\0") })), two: None, few: None, many: Some(icu::plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x05\0\0\0\0\0\r\0\x1A\0'\x004\0\xC7\0\0\0\0\0\0\0\0\0\0\0\0\x81\0\0\0\0\0\0\0\0\0\0\0\0\xC1@B\x0F\0\0\0\0\0\0\0\0\0\xC2\0\0\0\0\0\0\0\0\0\0\0\0\x07\0\0\0\0\0\0\0\0\x05\0\0\0") })) };
                static CS: <icu::plurals::provider::CardinalV1Marker as icu_provider::DataMarker>::Yokeable = icu::plurals::provider::PluralRulesV1 { zero: None, one: Some(icu::plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\r\0\xC1\0\0\0\0\x01\0\0\0\x01\0\0\0\xC2\0\0\0\0\0\0\0\0\0\0\0\0") })), two: None, few: Some(icu::plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\r\0\xC1\0\0\0\0\x02\0\0\0\x04\0\0\0\xC2\0\0\0\0\0\0\0\0\0\0\0\0") })), many: Some(icu::plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\x82\0\0\0\0\0\0\0\0\0\0\0\0") })) };
                static PL: <icu::plurals::provider::CardinalV1Marker as icu_provider::DataMarker>::Yokeable = icu::plurals::provider::PluralRulesV1 { zero: None, one: Some(icu::plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\r\0\xC1\0\0\0\0\x01\0\0\0\x01\0\0\0\xC2\0\0\0\0\0\0\0\0\0\0\0\0") })), two: None, few: Some(icu::plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\r\0\x1A\0\xC2\0\0\0\0\0\0\0\0\0\0\0\0\xC1\n\0\0\0\x02\0\0\0\x04\0\0\0\x81d\0\0\0\x0C\0\0\0\x0E\0\0\0") })), many: Some(icu::plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\r\0\x1A\0'\x004\0A\0N\0\xC2\0\0\0\0\0\0\0\0\0\0\0\0\x81\0\0\0\0\x01\0\0\0\x01\0\0\0\xC1\n\0\0\0\0\0\0\0\x01\0\0\0B\0\0\0\0\0\0\0\0\0\0\0\0\xC1\n\0\0\0\x05\0\0\0\t\0\0\0B\0\0\0\0\0\0\0\0\0\0\0\0\xC1d\0\0\0\x0C\0\0\0\x0E\0\0\0") })) };
                static RO: <icu::plurals::provider::CardinalV1Marker as icu_provider::DataMarker>::Yokeable = icu::plurals::provider::PluralRulesV1 { zero: None, one: Some(icu::plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\r\0\xC1\0\0\0\0\x01\0\0\0\x01\0\0\0\xC2\0\0\0\0\0\0\0\0\0\0\0\0") })), two: None, few: Some(icu::plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\r\0\x1A\0'\0\x82\0\0\0\0\0\0\0\0\0\0\0\0@\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x01\0\0\0\x01\0\0\0\xC0d\0\0\0\x01\0\0\0\x13\0\0\0") })), many: None };
                static SL: <icu::plurals::provider::CardinalV1Marker as icu_provider::DataMarker>::Yokeable = icu::plurals::provider::PluralRulesV1 { zero: None, one: Some(icu::plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\r\0\xC2\0\0\0\0\0\0\0\0\0\0\0\0\xC1d\0\0\0\x01\0\0\0\x01\0\0\0") })), two: Some(icu::plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\r\0\xC2\0\0\0\0\0\0\0\0\0\0\0\0\xC1d\0\0\0\x02\0\0\0\x02\0\0\0") })), few: Some(icu::plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\r\0\x1A\0\xC2\0\0\0\0\0\0\0\0\0\0\0\0\xC1d\0\0\0\x03\0\0\0\x04\0\0\0\x02\0\0\0\0\0\0\0\0\0\0\0\0") })), many: None };
                static DA: <icu::plurals::provider::CardinalV1Marker as icu_provider::DataMarker>::Yokeable = icu::plurals::provider::PluralRulesV1 { zero: None, one: Some(icu::plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\r\0\x1A\0\xC0\0\0\0\0\x01\0\0\0\x01\0\0\0\x05\0\0\0\0\0\0\0\0\0\0\0\0\xC1\0\0\0\0\0\0\0\0\0\0\0\0\x01\0\0\0\x01\0\0\0") })), two: None, few: None, many: None };
                static RU: <icu::plurals::provider::CardinalV1Marker as icu_provider::DataMarker>::Yokeable = icu::plurals::provider::PluralRulesV1 { zero: None, one: Some(icu::plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\r\0\x1A\0\xC2\0\0\0\0\0\0\0\0\0\0\0\0\xC1\n\0\0\0\x01\0\0\0\x01\0\0\0\x81d\0\0\0\x0B\0\0\0\x0B\0\0\0") })), two: None, few: Some(icu::plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\r\0\x1A\0\xC2\0\0\0\0\0\0\0\0\0\0\0\0\xC1\n\0\0\0\x02\0\0\0\x04\0\0\0\x81d\0\0\0\x0C\0\0\0\x0E\0\0\0") })), many: Some(icu::plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x06\0\0\0\0\0\r\0\x1A\0'\x004\0A\0\xC2\0\0\0\0\0\0\0\0\0\0\0\0\xC1\n\0\0\0\0\0\0\0\0\0\0\0B\0\0\0\0\0\0\0\0\0\0\0\0\xC1\n\0\0\0\x05\0\0\0\t\0\0\0B\0\0\0\0\0\0\0\0\0\0\0\0\xC1d\0\0\0\x0B\0\0\0\x0E\0\0\0") })) };
                static DSB: <icu::plurals::provider::CardinalV1Marker as icu_provider::DataMarker>::Yokeable = icu::plurals::provider::PluralRulesV1 { zero: None, one: Some(icu::plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\r\0\x1A\0\xC2\0\0\0\0\0\0\0\0\0\0\0\0\xC1d\0\0\0\x01\0\0\0\x01\0\0\0Dd\0\0\0\x01\0\0\0\x01\0\0\0") })), two: Some(icu::plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\r\0\x1A\0\xC2\0\0\0\0\0\0\0\0\0\0\0\0\xC1d\0\0\0\x02\0\0\0\x02\0\0\0Dd\0\0\0\x02\0\0\0\x02\0\0\0") })), few: Some(icu::plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\r\0\x1A\0\xC2\0\0\0\0\0\0\0\0\0\0\0\0\xC1d\0\0\0\x03\0\0\0\x04\0\0\0Dd\0\0\0\x03\0\0\0\x04\0\0\0") })), many: None };
                static SI: <icu::plurals::provider::CardinalV1Marker as icu_provider::DataMarker>::Yokeable = icu::plurals::provider::PluralRulesV1 { zero: None, one: Some(icu::plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\x15\0\"\0\xC0\0\0\0\0\0\0\0\0\0\0\0\0\x01\0\0\0\x01\0\0\0A\0\0\0\0\0\0\0\0\0\0\0\0\xC4\0\0\0\0\x01\0\0\0\x01\0\0\0") })), two: None, few: None, many: None };
                static HE: <icu::plurals::provider::CardinalV1Marker as icu_provider::DataMarker>::Yokeable = icu::plurals::provider::PluralRulesV1 { zero: None, one: Some(icu::plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\r\0\x1A\0'\0\xC1\0\0\0\0\x01\0\0\0\x01\0\0\0\xC2\0\0\0\0\0\0\0\0\0\0\0\0A\0\0\0\0\0\0\0\0\0\0\0\0\x82\0\0\0\0\0\0\0\0\0\0\0\0") })), two: Some(icu::plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\r\0\xC1\0\0\0\0\x02\0\0\0\x02\0\0\0\xC2\0\0\0\0\0\0\0\0\0\0\0\0") })), few: None, many: None };
                static MK: <icu::plurals::provider::CardinalV1Marker as icu_provider::DataMarker>::Yokeable = icu::plurals::provider::PluralRulesV1 { zero: None, one: Some(icu::plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x05\0\0\0\0\0\r\0\x1A\0'\x004\0\xC2\0\0\0\0\0\0\0\0\0\0\0\0\xC1\n\0\0\0\x01\0\0\0\x01\0\0\0\x81d\0\0\0\x0B\0\0\0\x0B\0\0\0D\n\0\0\0\x01\0\0\0\x01\0\0\0\x84d\0\0\0\x0B\0\0\0\x0B\0\0\0") })), two: None, few: None, many: None };
                static BS: <icu::plurals::provider::CardinalV1Marker as icu_provider::DataMarker>::Yokeable = icu::plurals::provider::PluralRulesV1 { zero: None, one: Some(icu::plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x05\0\0\0\0\0\r\0\x1A\0'\x004\0\xC2\0\0\0\0\0\0\0\0\0\0\0\0\xC1\n\0\0\0\x01\0\0\0\x01\0\0\0\x81d\0\0\0\x0B\0\0\0\x0B\0\0\0D\n\0\0\0\x01\0\0\0\x01\0\0\0\x84d\0\0\0\x0B\0\0\0\x0B\0\0\0") })), two: None, few: Some(icu::plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x05\0\0\0\0\0\r\0\x1A\0'\x004\0\xC2\0\0\0\0\0\0\0\0\0\0\0\0\xC1\n\0\0\0\x02\0\0\0\x04\0\0\0\x81d\0\0\0\x0C\0\0\0\x0E\0\0\0D\n\0\0\0\x02\0\0\0\x04\0\0\0\x84d\0\0\0\x0C\0\0\0\x0E\0\0\0") })), many: None };
                static IS: <icu::plurals::provider::CardinalV1Marker as icu_provider::DataMarker>::Yokeable = icu::plurals::provider::PluralRulesV1 { zero: None, one: Some(icu::plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x05\0\0\0\0\0\r\0\x1A\0'\x004\0\xC5\0\0\0\0\0\0\0\0\0\0\0\0\xC1\n\0\0\0\x01\0\0\0\x01\0\0\0\x81d\0\0\0\x0B\0\0\0\x0B\0\0\0E\n\0\0\0\x01\0\0\0\x01\0\0\0\x85d\0\0\0\x0B\0\0\0\x0B\0\0\0") })), two: None, few: None, many: None };
                static CEB: <icu::plurals::provider::CardinalV1Marker as icu_provider::DataMarker>::Yokeable = icu::plurals::provider::PluralRulesV1 { zero: None, one: Some(icu::plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x06\0\0\0\0\0\r\0*\x007\0T\0a\0\xC2\0\0\0\0\0\0\0\0\0\0\0\0\xC1\0\0\0\0\x01\0\0\0\x01\0\0\0\x02\0\0\0\x02\0\0\0\x03\0\0\0\x03\0\0\0B\0\0\0\0\0\0\0\0\0\0\0\0\x81\n\0\0\0\x04\0\0\0\x04\0\0\0\x06\0\0\0\x06\0\0\0\t\0\0\0\t\0\0\0\x02\0\0\0\0\0\0\0\0\0\0\0\0\x84\n\0\0\0\x04\0\0\0\x04\0\0\0\x06\0\0\0\x06\0\0\0\t\0\0\0\t\0\0\0") })), two: None, few: None, many: None };
                static CY: <icu::plurals::provider::CardinalV1Marker as icu_provider::DataMarker>::Yokeable = icu::plurals::provider::PluralRulesV1 { zero: Some(icu::plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC0\0\0\0\0\0\0\0\0\0\0\0\0") })), one: Some(icu::plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC0\0\0\0\0\x01\0\0\0\x01\0\0\0") })), two: Some(icu::plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC0\0\0\0\0\x02\0\0\0\x02\0\0\0") })), few: Some(icu::plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC0\0\0\0\0\x03\0\0\0\x03\0\0\0") })), many: Some(icu::plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC0\0\0\0\0\x06\0\0\0\x06\0\0\0") })) };
                static AR: <icu::plurals::provider::CardinalV1Marker as icu_provider::DataMarker>::Yokeable = icu::plurals::provider::PluralRulesV1 { zero: Some(icu::plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC0\0\0\0\0\0\0\0\0\0\0\0\0") })), one: Some(icu::plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC0\0\0\0\0\x01\0\0\0\x01\0\0\0") })), two: Some(icu::plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC0\0\0\0\0\x02\0\0\0\x02\0\0\0") })), few: Some(icu::plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC0d\0\0\0\x03\0\0\0\n\0\0\0") })), many: Some(icu::plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC0d\0\0\0\x0B\0\0\0c\0\0\0") })) };
                static LV: <icu::plurals::provider::CardinalV1Marker as icu_provider::DataMarker>::Yokeable = icu::plurals::provider::PluralRulesV1 { zero: Some(icu::plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\r\0\x1A\0'\0\xC0\n\0\0\0\0\0\0\0\0\0\0\0@d\0\0\0\x0B\0\0\0\x13\0\0\0B\0\0\0\0\x02\0\0\0\x02\0\0\0\xC4d\0\0\0\x0B\0\0\0\x13\0\0\0") })), one: Some(icu::plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x07\0\0\0\0\0\r\0\x1A\0'\x004\0A\0N\0\xC0\n\0\0\0\x01\0\0\0\x01\0\0\0\x80d\0\0\0\x0B\0\0\0\x0B\0\0\0B\0\0\0\0\x02\0\0\0\x02\0\0\0\xC4\n\0\0\0\x01\0\0\0\x01\0\0\0\x84d\0\0\0\x0B\0\0\0\x0B\0\0\0\x02\0\0\0\0\x02\0\0\0\x02\0\0\0\xC4\n\0\0\0\x01\0\0\0\x01\0\0\0") })), two: None, few: None, many: None };
                static VALUES: [&<icu::plurals::provider::CardinalV1Marker as icu_provider::DataMarker>::Yokeable; 92usize] = [&AF, &AM, &AR, &AM, &AST, &AF, &BE, &AF, &BHO, &AM, &BR, &AF, &BS, &CA, &CEB, &AF, &CS, &CY, &DA, &AST, &AM, &DSB, &AF, &AST, &ES, &AST, &AF, &AM, &AST, &CEB, &AF, &FR, &GA, &GD, &AST, &AM, &AF, &HE, &AM, &BS, &DSB, &AF, &HY, &AST, &IS, &CA, &AF, &AF, &AM, &AF, &AF, &LT, &LV, &MK, &AF, &AF, &AF, &AF, &AST, &AF, &AF, &BHO, &AM, &PL, &AF, &PT, &CA, &AF, &RO, &RU, &SAT, &AST, &AF, &SI, &CS, &SL, &AF, &AF, &BS, &AST, &AST, &AF, &AF, &BHO, &AF, &AF, &RU, &UND, &AST, &AF, &AF, &AM];
                static KEYS: [&str; 92usize] = ["af", "am", "ar", "as", "ast", "az", "be", "bg", "bho", "bn", "br", "brx", "bs", "ca", "ceb", "chr", "cs", "cy", "da", "de", "doi", "dsb", "el", "en", "es", "et", "eu", "fa", "fi", "fil", "fo", "fr", "ga", "gd", "gl", "gu", "ha", "he", "hi", "hr", "hsb", "hu", "hy", "ia", "is", "it", "ka", "kk", "kn", "ks", "ky", "lt", "lv", "mk", "ml", "mn", "mr", "ne", "nl", "no", "or", "pa", "pcm", "pl", "ps", "pt", "pt-PT", "rm", "ro", "ru", "sat", "sc", "sd", "si", "sk", "sl", "so", "sq", "sr", "sv", "sw", "ta", "te", "ti", "tk", "tr", "uk", "und", "ur", "uz", "xh", "zu"];
                let mut metadata = icu_provider::DataResponseMetadata::default();
                let payload = if let Ok(payload) = KEYS.binary_search_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse()).map(|i| *unsafe { VALUES.get_unchecked(i) }) {
                    payload
                } else {
                    const FALLBACKER: icu::locid_transform::fallback::LocaleFallbackerWithConfig<'static> = icu::locid_transform::fallback::LocaleFallbacker::new().for_config(<icu::plurals::provider::CardinalV1Marker as icu_provider::KeyedDataMarker>::KEY.fallback_config());
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
