// @generated
/// Implement `DataProvider<LongHourRelativeTimeFormatDataV1Marker>` on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_relativetime_long_hour_v1 {
    ($ provider : ty) => {
        #[clippy::msrv = "1.67"]
        const _: () = <$provider>::MUST_USE_MAKE_PROVIDER_MACRO;
        #[clippy::msrv = "1.67"]
        impl icu_provider::DataProvider<icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker>, icu_provider::DataError> {
                static PCM: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0D\xC3\xADs \xC3\xA1wa") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" áwa wé dọ\u{301}n pas"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" áwa wé dọ\u{301}n pas"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("Fọ  áwa wé de kọm"), index: 5u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("Fọ  áwa wé de kọm"), index: 5u8 } },
                };
                static SO: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0Saacadan") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" saacad kahor"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" saacadood kahor"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" saacad"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" saacadood"), index: 0u8 } },
                };
                static KGP: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC3\xB3ra tag k\xC3\xA3") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("óra  si ser"), index: 5u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("óra  si ser"), index: 5u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("óra  kar kỹ"), index: 5u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("óra  kar kỹ"), index: 5u8 } },
                };
                static IS: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC3\xBEessa stundina") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("fyrir  klukkustund"), index: 6u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("fyrir  klukkustundum"), index: 6u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("eftir  klukkustund"), index: 6u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("eftir  klukkustundir"), index: 6u8 } },
                };
                static TK: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC5\x9Fu sagat") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" sagat öň"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" sagat öň"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" sagatdan"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" sagatdan"), index: 0u8 } },
                };
                static LV: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC5\xA1aj\xC4\x81 stund\xC4\x81") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pirms  stundām"), index: 6u8 }), one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pirms  stundas"), index: 6u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pirms  stundām"), index: 6u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pēc  stundām"), index: 5u8 }), one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pēc  stundas"), index: 5u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pēc  stundām"), index: 5u8 } },
                };
                static LT: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC5\xA1i\xC4\x85 valand\xC4\x85") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("prieš  valandą"), index: 7u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("prieš  valandas"), index: 7u8 }), many: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("prieš  valandos"), index: 7u8 }), other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("prieš  valandų"), index: 7u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("po  valandos"), index: 3u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("po  valandų"), index: 3u8 }), many: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("po  valandos"), index: 3u8 }), other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("po  valandų"), index: 3u8 } },
                };
                static EL: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xCF\x84\xCF\x81\xCE\xAD\xCF\x87\xCE\xBF\xCF\x85\xCF\x83\xCE\xB1 \xCF\x8E\xCF\x81\xCE\xB1") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("πριν από  ώρα"), index: 16u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("πριν από  ώρες"), index: 16u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("σε  ώρα"), index: 5u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("σε  ώρες"), index: 5u8 } },
                };
                static TT: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xD0\xB1\xD1\x83 \xD1\x81\xD3\x99\xD0\xB3\xD0\xB0\xD1\x82\xD1\x8C\xD1\x82\xD3\x99") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" сәгать элек"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" сәгатьтән"), index: 0u8 } },
                };
                static BG: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xD0\xB2 \xD1\x82\xD0\xBE\xD0\xB7\xD0\xB8 \xD1\x87\xD0\xB0\xD1\x81") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("преди  час"), index: 11u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("преди  часа"), index: 11u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("след  час"), index: 9u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("след  часа"), index: 9u8 } },
                };
                static RU: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xD0\xB2 \xD1\x8D\xD1\x82\xD0\xBE\xD1\x82 \xD1\x87\xD0\xB0\xD1\x81") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" час назад"), index: 0u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" часа назад"), index: 0u8 }), many: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" часов назад"), index: 0u8 }), other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" часа назад"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("через  час"), index: 11u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("через  часа"), index: 11u8 }), many: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("через  часов"), index: 11u8 }), other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("через  часа"), index: 11u8 } },
                };
                static SR: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xD0\xBE\xD0\xB2\xD0\xBE\xD0\xB3 \xD1\x81\xD0\xB0\xD1\x82\xD0\xB0") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("пре  сата"), index: 7u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("пре  сата"), index: 7u8 }), many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("пре  сати"), index: 7u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("за  сат"), index: 5u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("за  сата"), index: 5u8 }), many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("за  сати"), index: 5u8 } },
                };
                static BS_CYRL: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xD0\xBE\xD0\xB2\xD0\xBE\xD0\xB3 \xD1\x81\xD0\xB0\xD1\x82\xD0\xB0") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("прије  сат"), index: 11u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("прије  сата"), index: 11u8 }), many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("прије  сати"), index: 11u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("за  сат"), index: 5u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("за  сата"), index: 5u8 }), many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("за  сати"), index: 5u8 } },
                };
                static SR_BA: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xD0\xBE\xD0\xB2\xD0\xBE\xD0\xB3 \xD1\x81\xD0\xB0\xD1\x82\xD0\xB0") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("прије  сата"), index: 11u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("прије  сата"), index: 11u8 }), many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("прије  сати"), index: 11u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("за  сат"), index: 5u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("за  сата"), index: 5u8 }), many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("за  сати"), index: 5u8 } },
                };
                static KK: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xD0\xBE\xD1\x81\xD1\x8B \xD1\x81\xD0\xB0\xD2\x93\xD0\xB0\xD1\x82") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" сағат бұрын"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" сағат бұрын"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" сағаттан кейін"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" сағаттан кейін"), index: 0u8 } },
                };
                static TG: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xD1\x81\xD0\xBE\xD0\xB0\xD1\x82\xD0\xB8 \xD2\xB7\xD0\xBE\xD1\x80\xD3\xA3") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" соат пеш"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("пас аз  соат"), index: 12u8 } },
                };
                static BE: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xD1\x83 \xD0\xB3\xD1\x8D\xD1\x82\xD1\x83 \xD0\xB3\xD0\xB0\xD0\xB4\xD0\xB7\xD1\x96\xD0\xBD\xD1\x83") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" гадзіну таму"), index: 0u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" гадзіны таму"), index: 0u8 }), many: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" гадзін таму"), index: 0u8 }), other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" гадзіны таму"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("праз  гадзіну"), index: 9u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("праз  гадзіны"), index: 9u8 }), many: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("праз  гадзін"), index: 9u8 }), other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("праз  гадзіны"), index: 9u8 } },
                };
                static KY: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xD1\x83\xD1\x88\xD1\x83\xD0\xBB \xD1\x81\xD0\xB0\xD0\xB0\xD1\x82\xD1\x82\xD0\xB0") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" саат мурун"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" саат мурун"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" сааттан кийин"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" сааттан кийин"), index: 0u8 } },
                };
                static UK: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xD1\x86\xD1\x96\xD1\x94\xD1\x97 \xD0\xB3\xD0\xBE\xD0\xB4\xD0\xB8\xD0\xBD\xD0\xB8") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" годину тому"), index: 0u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" години тому"), index: 0u8 }), many: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" годин тому"), index: 0u8 }), other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" години тому"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("через  годину"), index: 11u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("через  години"), index: 11u8 }), many: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("через  годин"), index: 11u8 }), other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("через  години"), index: 11u8 } },
                };
                static MK: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xD1\x87\xD0\xB0\xD1\x81\xD0\xBE\xD0\xB2") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("пред  час"), index: 9u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("пред  часа"), index: 9u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("за  час"), index: 5u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("за  часа"), index: 5u8 } },
                };
                static MN: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xD1\x8D\xD0\xBD\xD1\x8D \xD1\x86\xD0\xB0\xD0\xB3") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" цагийн өмнө"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" цагийн өмнө"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" цагийн дараа"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" цагийн дараа"), index: 0u8 } },
                };
                static HY: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xD5\xA1\xD5\xB5\xD5\xBD \xD5\xAA\xD5\xA1\xD5\xB4\xD5\xAB\xD5\xB6") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ժամ առաջ"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ժամ առաջ"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ժամից"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ժամից"), index: 0u8 } },
                };
                static HE: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xD7\x91\xD7\xA9\xD7\xA2\xD7\x94 \xD7\x96\xD7\x95") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("לפני שעה"), index: 255u8 }), two: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("לפני שעתיים"), index: 255u8 }), few: None, many: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("לפני  שעות"), index: 9u8 }), other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("לפני  שעות"), index: 9u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("בעוד שעה"), index: 255u8 }), two: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("בעוד שעתיים"), index: 255u8 }), few: None, many: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("בעוד  שעות"), index: 9u8 }), other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("בעוד  שעות"), index: 9u8 } },
                };
                static UR: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xD8\xA7\xD8\xB3 \xDA\xAF\xDA\xBE\xD9\x86\xD9\xB9\xDB\x92") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" گھنٹہ پہلے"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" گھنٹے پہلے"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" گھنٹے میں"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" گھنٹے میں"), index: 0u8 } },
                };
                static AR: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xD8\xA7\xD9\x84\xD8\xB3\xD8\xA7\xD8\xB9\xD8\xA9 \xD8\xA7\xD9\x84\xD8\xAD\xD8\xA7\xD9\x84\xD9\x8A\xD8\xA9") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("قبل  ساعة"), index: 7u8 }), one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("قبل ساعة واحدة"), index: 255u8 }), two: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("قبل ساعتين"), index: 255u8 }), few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("قبل  ساعات"), index: 7u8 }), many: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("قبل  ساعة"), index: 7u8 }), other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("قبل  ساعة"), index: 7u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("خلال  ساعة"), index: 9u8 }), one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("خلال ساعة واحدة"), index: 255u8 }), two: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("خلال ساعتين"), index: 255u8 }), few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("خلال  ساعات"), index: 9u8 }), many: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("خلال  ساعة"), index: 9u8 }), other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("خلال  ساعة"), index: 9u8 } },
                };
                static PS: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xD8\xAF\xD8\xA7 \xD8\xB3\xD8\xA7\xD8\xB9\xD8\xAA") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ساعت مخکې"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ساعتونه مخکې"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("په  ساعت کې"), index: 5u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("په  ساعتو کې"), index: 5u8 } },
                };
                static FA: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xD9\x87\xD9\x85\xDB\x8C\xD9\x86 \xD8\xB3\xD8\xA7\xD8\xB9\xD8\xAA") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ساعت پیش"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ساعت پیش"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ساعت بعد"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ساعت بعد"), index: 0u8 } },
                };
                static SD: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xD9\x87\xD9\x86 \xDA\xAA\xD9\x84\xDA\xAA") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ڪلاڪ پهرين"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ڪلاڪ پهرين"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ڪلاڪ ۾"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ڪلاڪ ۾"), index: 0u8 } },
                };
                static MAI: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE0\xA4\x8F\xE0\xA4\xB9\xE0\xA4\xBF \xE0\xA4\x98\xE0\xA4\x82\xE0\xA4\x9F\xE0\xA4\xBE") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" घ\u{902}टा पहिल\u{947}"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" घ\u{902}टा म\u{947}"), index: 0u8 } },
                };
                static MR: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE0\xA4\xA4\xE0\xA4\xBE\xE0\xA4\xB8\xE0\xA4\xBE\xE0\xA4\xA4") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" तासाप\u{942}र\u{94d}वी"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" तासा\u{902}प\u{942}र\u{94d}वी"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" तासामध\u{94d}य\u{947}"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" तासा\u{902}मध\u{94d}य\u{947}"), index: 0u8 } },
                };
                static BRX: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE0\xA4\xAC\xE0\xA5\x87 \xE0\xA4\x98\xE0\xA4\xA8\xE0\xA5\x8D\xE0\xA4\x9F\xE0\xA4\xBE") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" घन\u{94d}टा सिगा\u{902}"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" घन\u{94d}टा सिगा\u{902}"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" घन\u{94d}टायाव"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" घन\u{94d}टायाव"), index: 0u8 } },
                };
                static NE: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE0\xA4\xAF\xE0\xA4\xB8 \xE0\xA4\x98\xE0\xA4\xA1\xE0\xA5\x80\xE0\xA4\xAE\xE0\xA4\xBE") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" घण\u{94d}टा पहिल\u{947}"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" घण\u{94d}टा पहिल\u{947}"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" घण\u{94d}टामा"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" घण\u{94d}टामा"), index: 0u8 } },
                };
                static HI: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE0\xA4\xAF\xE0\xA4\xB9 \xE0\xA4\x98\xE0\xA4\x82\xE0\xA4\x9F\xE0\xA4\xBE") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" घ\u{902}ट\u{947} पहल\u{947}"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" घ\u{902}ट\u{947} पहल\u{947}"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" घ\u{902}ट\u{947} म\u{947}\u{902}"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" घ\u{902}ट\u{947} म\u{947}\u{902}"), index: 0u8 } },
                };
                static KOK: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE0\xA4\xB9\xE0\xA5\x87\xE0\xA4\x82 \xE0\xA4\xB5\xE0\xA4\xB0") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" वरा आदी\u{902}"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" वरा\u{902}नी\u{902}"), index: 0u8 } },
                };
                static BN: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE0\xA6\x8F\xE0\xA6\x87 \xE0\xA6\x98\xE0\xA6\xA3\xE0\xA7\x8D\xE0\xA6\x9F\xE0\xA6\xBE\xE0\xA6\xAF\xE0\xA6\xBC") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ঘন\u{9cd}ট\u{9be} আগে"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ঘন\u{9cd}ট\u{9be} আগে"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ঘন\u{9cd}ট\u{9be}য\u{9bc}"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ঘন\u{9cd}ট\u{9be}য\u{9bc}"), index: 0u8 } },
                };
                static AS: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE0\xA6\x8F\xE0\xA6\x87\xE0\xA6\x9F\xE0\xA7\x8B \xE0\xA6\x98\xE0\xA6\xA3\xE0\xA7\x8D\xE0\xA6\x9F\xE0\xA6\xBE\xE0\xA6\xA4") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ঘণ\u{9cd}ট\u{9be} প\u{9c2}ৰ\u{9cd}বে"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ঘণ\u{9cd}ট\u{9be} প\u{9c2}ৰ\u{9cd}বে"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ঘণ\u{9cd}ট\u{9be}ত"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ঘণ\u{9cd}ট\u{9be}ত"), index: 0u8 } },
                };
                static PA: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE0\xA8\x87\xE0\xA8\xB8 \xE0\xA8\x98\xE0\xA9\xB0\xE0\xA8\x9F\xE0\xA9\x87") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ਘ\u{a70}ਟਾ ਪਹਿਲਾ\u{a02}"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ਘ\u{a70}ਟ\u{a47} ਪਹਿਲਾ\u{a02}"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ਘ\u{a70}ਟ\u{a47} ਵਿ\u{a71}ਚ"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ਘ\u{a70}ਟਿਆ\u{a02} ਵਿ\u{a71}ਚ"), index: 0u8 } },
                };
                static GU: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE0\xAA\x86 \xE0\xAA\x95\xE0\xAA\xB2\xE0\xAA\xBE\xE0\xAA\x95") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" કલાક પહ\u{ac7}લા\u{a82}"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" કલાક પહ\u{ac7}લા\u{a82}"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" કલાકમા\u{a82}"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" કલાકમા\u{a82}"), index: 0u8 } },
                };
                static OR: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE0\xAC\x8F\xE0\xAC\xB9\xE0\xAC\xBF \xE0\xAC\x98\xE0\xAC\xA3\xE0\xAD\x8D\xE0\xAC\x9F\xE0\xAC\xBE") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ଘଣ\u{b4d}ଟ\u{b3e} ପ\u{b42}ର\u{b4d}ବେ"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ଘଣ\u{b4d}ଟ\u{b3e} ପ\u{b42}ର\u{b4d}ବେ"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ଘଣ\u{b4d}ଟ\u{b3e}ରେ"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ଘଣ\u{b4d}ଟ\u{b3e}ରେ"), index: 0u8 } },
                };
                static TA: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE0\xAE\x87\xE0\xAE\xA8\xE0\xAF\x8D\xE0\xAE\xA4 \xE0\xAE\x92\xE0\xAE\xB0\xE0\xAF\x81 \xE0\xAE\xAE\xE0\xAE\xA3\xE0\xAE\xBF\xE0\xAE\xA8\xE0\xAF\x87\xE0\xAE\xB0\xE0\xAE\xA4\xE0\xAF\x8D\xE0\xAE\xA4\xE0\xAE\xBF\xE0\xAE\xB2\xE0\xAF\x8D") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" மணிநேரம\u{bcd} முன\u{bcd}"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" மணிநேரம\u{bcd} முன\u{bcd}"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" மணிநேரத\u{bcd}தில\u{bcd}"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" மணிநேரத\u{bcd}தில\u{bcd}"), index: 0u8 } },
                };
                static TE: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE0\xB0\x88 \xE0\xB0\x97\xE0\xB0\x82\xE0\xB0\x9F") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" గంట క\u{c4d}ర\u{c3f}తం"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" గంటల క\u{c4d}ర\u{c3f}తం"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" గంటల\u{c4b}"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" గంటల\u{c4d}ల\u{c4b}"), index: 0u8 } },
                };
                static KN: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE0\xB2\x88 \xE0\xB2\x97\xE0\xB2\x82\xE0\xB2\x9F\xE0\xB3\x86") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ಗಂಟ\u{cc6} ಹ\u{cbf}ಂದ\u{cc6}"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ಗಂಟ\u{cc6}ಗಳ ಹ\u{cbf}ಂದ\u{cc6}"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ಗಂಟ\u{cc6}ಯಲ\u{ccd}ಲ\u{cbf}"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ಗಂಟ\u{cc6}ಗಳಲ\u{ccd}ಲ\u{cbf}"), index: 0u8 } },
                };
                static ML: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE0\xB4\x88 \xE0\xB4\xAE\xE0\xB4\xA3\xE0\xB4\xBF\xE0\xB4\x95\xE0\xB5\x8D\xE0\xB4\x95\xE0\xB5\x82\xE0\xB4\xB1\xE0\xB4\xBF\xE0\xB5\xBD") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" മണിക\u{d4d}ക\u{d42}ർ മ\u{d41}മ\u{d4d}പ\u{d4d}"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" മണിക\u{d4d}ക\u{d42}ർ മ\u{d41}മ\u{d4d}പ\u{d4d}"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" മണിക\u{d4d}ക\u{d42}റിൽ"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" മണിക\u{d4d}ക\u{d42}റിൽ"), index: 0u8 } },
                };
                static SI: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE0\xB6\xB8\xE0\xB7\x99\xE0\xB6\xB8 \xE0\xB6\xB4\xE0\xB7\x90\xE0\xB6\xBA") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("පැය කට පෙර"), index: 10u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("පැය කට පෙර"), index: 10u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("පැය ක\u{dd2}න\u{dca}"), index: 10u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("පැය ක\u{dd2}න\u{dca}"), index: 10u8 } },
                };
                static TH: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE0\xB8\x8A\xE0\xB8\xB1\xE0\xB9\x88\xE0\xB8\xA7\xE0\xB9\x82\xE0\xB8\xA1\xE0\xB8\x87\xE0\xB8\x99\xE0\xB8\xB5\xE0\xB9\x89") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ช\u{e31}\u{e48}วโมงท\u{e35}\u{e48}ผ\u{e48}านมา"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ในอ\u{e35}ก  ช\u{e31}\u{e48}วโมง"), index: 16u8 } },
                };
                static LO: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE0\xBA\x8A\xE0\xBA\xBB\xE0\xBB\x88\xE0\xBA\xA7\xE0\xBB\x82\xE0\xBA\xA1\xE0\xBA\x87\xE0\xBA\x99\xE0\xBA\xB5\xE0\xBB\x89") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ຊ\u{ebb}\u{ec8}ວໂມງກ\u{ec8}ອນ"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ໃນອ\u{eb5}ກ  ຊ\u{ebb}\u{ec8}ວໂມງ"), index: 16u8 } },
                };
                static MY: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE1\x80\xA4\xE1\x80\xA1\xE1\x80\x81\xE1\x80\xBB\xE1\x80\xAD\xE1\x80\x94\xE1\x80\xBA") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ပြ\u{102e}းခ\u{1032}\u{1037}သည\u{1037}\u{103a}  နာရ\u{102e}"), index: 34u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" နာရ\u{102e}အတ\u{103d}င\u{103a}း"), index: 0u8 } },
                };
                static KA: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE1\x83\x90\xE1\x83\x9B \xE1\x83\xA1\xE1\x83\x90\xE1\x83\x90\xE1\x83\x97\xE1\x83\xA8\xE1\x83\x98") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" საათის წინ"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" საათის წინ"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" საათში"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" საათში"), index: 0u8 } },
                };
                static TI: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE1\x8A\xA3\xE1\x89\xA5\xE1\x8B\x9A \xE1\x88\xB0\xE1\x8B\x93\xE1\x89\xB5") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ቅድሚ  ሰዓት"), index: 10u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ቅድሚ  ሰዓት"), index: 10u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ኣብ  ሰዓት"), index: 7u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ኣብ  ሰዓት"), index: 7u8 } },
                };
                static AM: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE1\x8B\xAD\xE1\x88\x85 \xE1\x88\xB0\xE1\x8B\x93\xE1\x89\xB5") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ከ ሰዓት በፊት"), index: 3u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ከ ሰዓቶች በፊት"), index: 3u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("በ ሰዓት ውስጥ"), index: 3u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("በ ሰዓቶች ውስጥ"), index: 3u8 } },
                };
                static CHR: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE1\x8E\xAF\xE1\x8E\xA0 \xE1\x8F\x91\xE1\x8F\x9F\xE1\x8E\xB6\xE1\x8F\x93") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ᏑᏟᎶᏓ ᏥᎨᏒ"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ᎢᏳᏟᎶᏓ ᏥᎨᏒ"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ᎾᎿ  ᏑᏟᎶᏓ"), index: 7u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ᎾᎿ  ᎢᏳᏟᎶᏓ"), index: 7u8 } },
                };
                static KM: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE1\x9E\x98\xE1\x9F\x89\xE1\x9F\x84\xE1\x9E\x84\xE1\x9E\x93\xE1\x9F\x81\xE1\x9F\x87") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ម\u{17c9}ោង\u{200b}ម\u{17bb}ន"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ក\u{17d2}ន\u{17bb}ង\u{200b}រយៈ\u{200b}ពេល  ម\u{17c9}ោង"), index: 40u8 } },
                };
                static YUE_HANS: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE5\x91\xA2\xE4\xB8\xAA\xE5\xB0\x8F\xE6\x97\xB6") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" 小时前"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" 小时后"), index: 0u8 } },
                };
                static YUE: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE5\x91\xA2\xE5\x80\x8B\xE5\xB0\x8F\xE6\x99\x82") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" 小時前"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" 小時後"), index: 0u8 } },
                };
                static ZH: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE8\xBF\x99\xE4\xB8\x80\xE6\x97\xB6\xE9\x97\xB4 / \xE6\xAD\xA4\xE6\x97\xB6") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("小时前"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("小时后"), index: 0u8 } },
                };
                static ZH_HANT: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE9\x80\x99\xE4\xB8\x80\xE5\xB0\x8F\xE6\x99\x82") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" 小時前"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" 小時後"), index: 0u8 } },
                };
                static ZH_HK: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xE9\x80\x99\xE5\x80\x8B\xE5\xB0\x8F\xE6\x99\x82") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" 小時前"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" 小時後"), index: 0u8 } },
                };
                static KO: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xED\x98\x84\xEC\x9E\xAC \xEC\x8B\x9C\xEA\xB0\x84") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("시간 전"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("시간 후"), index: 0u8 } },
                };
                static FF_ADLM: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xF0\x9E\xA4\xB2\xF0\x9E\xA5\x8B\xF0\x9E\xA4\xA3\xF0\x9E\xA4\xAD\xF0\x9E\xA5\x85 \xF0\x9E\xA4\xAF\xF0\x9E\xA4\xAE\xF0\x9E\xA4\xAE \xF0\x9E\xA4\xB2\xF0\x9E\xA5\x8B\xF0\x9E\xA4\xB6\xF0\x9E\xA4\xA2\xF0\x9E\xA4\xA5\xF0\x9E\xA4\xB2\xF0\x9E\xA5\x8B\xF0\x9E\xA4\xA3\xF0\x9E\xA4\xAD") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" 𞤲𞥋𞤶𞤢𞤥𞤲𞥋𞤣𞤭 𞤱𞤵𞤤𞤭\u{1e945}𞤲𞥋𞤣𞤭"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" 𞤲𞥋𞤶𞤢𞤥𞤤𞤭 𞤱𞤵𞤤𞤭\u{1e945}𞤯𞤭"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("𞤲𞥋𞤣𞤫𞤪  𞤲𞥋𞤶𞤢𞤥𞤲𞥋𞤣𞤭"), index: 21u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("𞤲𞥋𞤣𞤫𞤪  𞤲𞥋𞤶𞤢𞤥𞤤𞤭"), index: 21u8 } },
                };
                static GD: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0am broinn uair a th\xC3\xACde") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" uair a thìde air ais"), index: 0u8 }), two: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" uair a thìde air ais"), index: 0u8 }), few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" uairean a thìde air ais"), index: 0u8 }), many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" uair a thìde air ais"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("an ceann  uair a thìde"), index: 9u8 }), two: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("an ceann  uair a thìde"), index: 9u8 }), few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("an ceann  uairean a thìde"), index: 9u8 }), many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("an ceann  uair a thìde"), index: 9u8 } },
                };
                static GA: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0an uair seo") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" uair an chloig ó shin"), index: 0u8 }), two: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" uair an chloig ó shin"), index: 0u8 }), few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" uair an chloig ó shin"), index: 0u8 }), many: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" n-uair an chloig ó shin"), index: 0u8 }), other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" uair an chloig ó shin"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("i gceann  uair an chloig"), index: 9u8 }), two: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("i gceann  uair an chloig"), index: 9u8 }), few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("i gceann  uair an chloig"), index: 9u8 }), many: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("i gceann  n-uair an chloig"), index: 9u8 }), other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("i gceann  uair an chloig"), index: 9u8 } },
                };
                static CA: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0aquesta hora") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("fa  hora"), index: 3u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("fa  hores"), index: 3u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("d’aquí a  hora"), index: 12u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("d’aquí a  hores"), index: 12u8 } },
                };
                static NL: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0binnen een uur") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" uur geleden"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" uur geleden"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("over  uur"), index: 5u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("over  uur"), index: 5u8 } },
                };
                static TR: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0bu saat") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" saat önce"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" saat önce"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" saat sonra"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" saat sonra"), index: 0u8 } },
                };
                static AZ: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0bu saat") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" saat öncə"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" saat öncə"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" saat ərzində"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" saat ərzində"), index: 0u8 } },
                };
                static FR: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0cette heure-ci") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("il y a  heure"), index: 7u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("il y a  heures"), index: 7u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("dans  heure"), index: 5u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("dans  heures"), index: 5u8 } },
                };
                static WO: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0ci waxtu wii") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" waxtu ci ginaaw"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("fileek  waxtu"), index: 7u8 } },
                };
                static SC: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0cust\xE2\x80\x99ora") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ora a como"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" oras a como"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("intro de  ora"), index: 9u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("intro de  oras"), index: 9u8 } },
                };
                static BR: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0d\xCA\xBCan eur-ma\xC3\xB1") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" eur zo"), index: 0u8 }), two: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" eur zo"), index: 0u8 }), few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" eur zo"), index: 0u8 }), many: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" a eurioù zo"), index: 0u8 }), other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" eur zo"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("a-benn  eur"), index: 7u8 }), two: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("a-benn  eur"), index: 7u8 }), few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("a-benn  eur"), index: 7u8 }), many: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("a-benn  a eurioù"), index: 7u8 }), other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("a-benn  eur"), index: 7u8 } },
                };
                static SV: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0denna timme") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("för  timme sedan"), index: 5u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("för  timmar sedan"), index: 5u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("om  timme"), index: 3u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("om  timmar"), index: 3u8 } },
                };
                static DA: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0denne time") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("for  time siden"), index: 4u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("for  timer siden"), index: 4u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("om  time"), index: 3u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("om  timer"), index: 3u8 } },
                };
                static NN: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0denne timen") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("for  time sidan"), index: 4u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("for  timar sidan"), index: 4u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("om  time"), index: 3u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("om  timar"), index: 3u8 } },
                };
                static NO: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0denne timen") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("for  time siden"), index: 4u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("for  timer siden"), index: 4u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("om  time"), index: 3u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("om  timer"), index: 3u8 } },
                };
                static HU: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0ebben az \xC3\xB3r\xC3\xA1ban") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" órával ezelőtt"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" órával ezelőtt"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" óra múlva"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" óra múlva"), index: 0u8 } },
                };
                static IG: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0elekere a") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- h"), index: 1u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ h"), index: 1u8 } },
                };
                static KEA: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0es ora li") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("a ten  ora"), index: 6u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("di li  ora"), index: 6u8 } },
                };
                static ES: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0esta hora") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("hace  hora"), index: 5u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("hace  horas"), index: 5u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("dentro de  hora"), index: 10u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("dentro de  horas"), index: 10u8 } },
                };
                static GL: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0esta hora") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("hai  hora"), index: 4u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("hai  horas"), index: 4u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("en  hora"), index: 3u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("en  horas"), index: 3u8 } },
                };
                static AST: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0esta hora") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("hai  hora"), index: 4u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("hai  hores"), index: 4u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("en  hora"), index: 3u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("en  hores"), index: 3u8 } },
                };
                static PT_PT: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0esta hora") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("há  hora"), index: 4u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("há  horas"), index: 4u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("dentro de  hora"), index: 10u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("dentro de  horas"), index: 10u8 } },
                };
                static PT: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0esta hora") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("há  hora"), index: 4u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("há  horas"), index: 4u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("em  hora"), index: 3u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("em  horas"), index: 3u8 } },
                };
                static VI: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0gi\xE1\xBB\x9D n\xC3\xA0y") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" giờ trước"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("sau  giờ nữa"), index: 4u8 } },
                };
                static FO: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0hendan t\xC3\xADman") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" tími síðan"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" tímar síðan"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("um  tíma"), index: 3u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("um  tímar"), index: 3u8 } },
                };
                static AF: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0hierdie uur") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" uur gelede"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" uur gelede"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("oor  uur"), index: 4u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("oor  uur"), index: 4u8 } },
                };
                static MI: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0i t\xC4\x93nei h\xC4\x81ora") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- h"), index: 1u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ h"), index: 1u8 } },
                };
                static DE: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0in dieser Stunde") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("vor  Stunde"), index: 4u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("vor  Stunden"), index: 4u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("in  Stunde"), index: 3u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("in  Stunden"), index: 3u8 } },
                };
                static IA: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0iste hora") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" hora retro"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" horas retro"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("in  hora"), index: 3u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("in  horas"), index: 3u8 } },
                };
                static JV: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0jam iki") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" jam kepungkur"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ing  jam"), index: 4u8 } },
                };
                static MS: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0jam ini") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" jam lalu"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("dalam  jam"), index: 6u8 } },
                };
                static ID: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0jam ini") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" jam yang lalu"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("dalam  jam"), index: 6u8 } },
                };
                static SQ: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0k\xC3\xABt\xC3\xAB or\xC3\xAB") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" orë më parë"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" orë më parë"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pas  ore"), index: 4u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pas  orësh"), index: 4u8 } },
                };
                static QU: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0kay hora") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- h"), index: 1u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ h"), index: 1u8 } },
                };
                static TO: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0ko e houa \xCA\xBBeni") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("houa ʻe  kuoʻosi"), index: 9u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ʻi he houa ʻe "), index: 16u8 } },
                };
                static YRL: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0ku\xC3\xA1 hura") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("aikué  hura"), index: 7u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("aikué  hura itá"), index: 7u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" hura resê"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" hura resê itá"), index: 0u8 } },
                };
                static ZU: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0leli hora") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ihora eledlule"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("emahoreni angu- edlule"), index: 15u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ehoreni elingu- elizayo"), index: 15u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("emahoreni angu- ezayo"), index: 15u8 } },
                };
                static FIL: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0ngayong oras") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" oras ang nakalipas"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" oras ang nakalipas"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("sa  oras"), index: 3u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("sa  oras"), index: 3u8 } },
                };
                static RO: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0ora aceasta") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("acum  oră"), index: 5u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("acum  ore"), index: 5u8 }), many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("acum  de ore"), index: 5u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("peste  oră"), index: 6u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("peste  ore"), index: 6u8 }), many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("peste  de ore"), index: 6u8 } },
                };
                static EU: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0ordu honetan") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("Duela  ordu"), index: 6u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("Duela  ordu"), index: 6u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ordu barru"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ordu barru"), index: 0u8 } },
                };
                static BS: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0ovaj sat") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("prije  sat"), index: 6u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("prije  sata"), index: 6u8 }), many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("prije  sati"), index: 6u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  sat"), index: 3u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  sata"), index: 3u8 }), many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  sati"), index: 3u8 } },
                };
                static SR_LATN: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0ovog sata") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pre  sata"), index: 4u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pre  sata"), index: 4u8 }), many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pre  sati"), index: 4u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  sat"), index: 3u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  sata"), index: 3u8 }), many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  sati"), index: 3u8 } },
                };
                static SR_LATN_BA: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0ovog sata") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("prije  sata"), index: 6u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("prije  sata"), index: 6u8 }), many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("prije  sati"), index: 6u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  sat"), index: 3u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  sata"), index: 3u8 }), many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  sati"), index: 3u8 } },
                };
                static ET: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0praegusel tunnil") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" tunni eest"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" tunni eest"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" tunni pärast"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" tunni pärast"), index: 0u8 } },
                };
                static IT: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0quest\xE2\x80\x99ora") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ora fa"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ore fa"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("tra  ora"), index: 4u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("tra  ore"), index: 4u8 } },
                };
                static SW: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0saa hii") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("saa  iliyopita"), index: 4u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("saa  zilizopita"), index: 4u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("baada ya saa "), index: 13u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("baada ya saa "), index: 13u8 } },
                };
                static UZ: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0shu soatda") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" soat oldin"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" soat oldin"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" soatdan keyin"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" soatdan keyin"), index: 0u8 } },
                };
                static FI: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0t\xC3\xA4m\xC3\xA4n tunnin aikana") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" tunti sitten"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" tuntia sitten"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" tunnin päästä"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" tunnin päästä"), index: 0u8 } },
                };
                static PL: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0ta godzina") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" godzinę temu"), index: 0u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" godziny temu"), index: 0u8 }), many: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" godzin temu"), index: 0u8 }), other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" godziny temu"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  godzinę"), index: 3u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  godziny"), index: 3u8 }), many: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  godzin"), index: 3u8 }), other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  godziny"), index: 3u8 } },
                };
                static UND: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0this hour") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("- h"), index: 1u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("+ h"), index: 1u8 } },
                };
                static EN: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0this hour") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" hour ago"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" hours ago"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("in  hour"), index: 3u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("in  hours"), index: 3u8 } },
                };
                static UZ_CYRL: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0this hour") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" соат олдин"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" соат олдин"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" соатдан сўнг"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" соатдан сўнг"), index: 0u8 } },
                };
                static CS: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0tuto hodinu") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("před  hodinou"), index: 6u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("před  hodinami"), index: 6u8 }), many: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("před  hodiny"), index: 6u8 }), other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("před  hodinami"), index: 6u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  hodinu"), index: 3u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  hodiny"), index: 3u8 }), many: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  hodiny"), index: 3u8 }), other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  hodin"), index: 3u8 } },
                };
                static SL: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0v tej uri") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pred  uro"), index: 5u8 }), two: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pred  urama"), index: 5u8 }), few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pred  urami"), index: 5u8 }), many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pred  urami"), index: 5u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("čez  uro"), index: 5u8 }), two: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("čez  uri"), index: 5u8 }), few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("čez  ure"), index: 5u8 }), many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("čez  ur"), index: 5u8 } },
                };
                static SK: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0v tejto hodine") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pred  hodinou"), index: 5u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pred  hodinami"), index: 5u8 }), many: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pred  hodinou"), index: 5u8 }), other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pred  hodinami"), index: 5u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("o  hodinu"), index: 2u8 }), two: None, few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("o  hodiny"), index: 2u8 }), many: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("o  hodiny"), index: 2u8 }), other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("o  hodín"), index: 2u8 } },
                };
                static DSB: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0w to\xC5\x9B tej g\xC3\xB3\xC5\xBAinje") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pśed  góźinu"), index: 6u8 }), two: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pśed  góźinoma"), index: 6u8 }), few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pśed  góźinami"), index: 6u8 }), many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("pśed  góźinami"), index: 6u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  góźinu"), index: 3u8 }), two: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  góźinje"), index: 3u8 }), few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  góźiny"), index: 3u8 }), many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  góźin"), index: 3u8 } },
                };
                static HSB: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0w tutej hod\xC5\xBAinje") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("před  hodźinu"), index: 6u8 }), two: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("před  hodźinomaj"), index: 6u8 }), few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("před  hodźinami"), index: 6u8 }), many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("před  hodźinami"), index: 6u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  hodźinu"), index: 3u8 }), two: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  hodźinje"), index: 3u8 }), few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  hodźiny"), index: 3u8 }), many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("za  hodźin"), index: 3u8 } },
                };
                static HA: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0wannan awa") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" awa da ya gabata"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" awa da ya gabata"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("cikin  awa"), index: 6u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("cikin  awa"), index: 6u8 } },
                };
                static HI_LATN: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0yah ghanta") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ghante pahle"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ghante pahle"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ghante mein"), index: 0u8 }), two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" ghante mein"), index: 0u8 } },
                };
                static CY: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0yr awr hon") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" awr yn ôl"), index: 0u8 }), one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" awr yn ôl"), index: 0u8 }), two: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" awr yn ôl"), index: 0u8 }), few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" awr yn ôl"), index: 0u8 }), many: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" awr yn ôl"), index: 0u8 }), other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" awr yn ôl"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ymhen  awr"), index: 6u8 }), one: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ymhen awr"), index: 255u8 }), two: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ymhen  awr"), index: 6u8 }), few: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ymhen  awr"), index: 6u8 }), many: Some(icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ymhen  awr"), index: 6u8 }), other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed("ymhen  awr"), index: 6u8 } },
                };
                static JA: <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable = icu::relativetime::provider::RelativeTimePatternDataV1 {
                    relatives: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\x001 \xE6\x99\x82\xE9\x96\x93\xE4\xBB\xA5\xE5\x86\x85") })
                    },
                    past: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" 時間前"), index: 0u8 } },
                    future: icu::relativetime::provider::PluralRulesCategoryMapping { zero: None, one: None, two: None, few: None, many: None, other: icu::relativetime::provider::SingularSubPattern { pattern: alloc::borrow::Cow::Borrowed(" 時間後"), index: 0u8 } },
                };
                static VALUES: [&<icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::Yokeable; 121usize] = [&AF, &AM, &AR, &AS, &AST, &AZ, &BE, &BG, &BN, &BR, &BRX, &BS, &BS_CYRL, &CA, &CHR, &CS, &CY, &DA, &DE, &DSB, &EL, &EN, &ES, &ET, &EU, &FA, &FF_ADLM, &FI, &FIL, &FO, &FR, &GA, &GD, &GL, &GU, &HA, &HE, &HI, &HI_LATN, &BS, &HSB, &HU, &HY, &IA, &ID, &IG, &IS, &IT, &JA, &JV, &KA, &KEA, &KGP, &KK, &KM, &KN, &KO, &KOK, &KY, &LO, &LT, &LV, &MAI, &MI, &MK, &ML, &MN, &MR, &MS, &MY, &NE, &NL, &NN, &NO, &OR, &PA, &PCM, &PL, &PS, &PT, &PT_PT, &QU, &RO, &RU, &SC, &SD, &SI, &SK, &SL, &SO, &SQ, &SR, &SR_BA, &SR_LATN, &SR_LATN_BA, &SV, &SW, &TA, &TE, &TG, &TH, &TI, &TK, &TO, &TR, &TT, &UK, &UND, &UR, &UZ, &UZ_CYRL, &VI, &WO, &YRL, &YUE, &YUE_HANS, &ZH, &ZH_HK, &ZH_HANT, &ZH_HK, &ZU];
                static KEYS: [&str; 121usize] = ["af", "am", "ar", "as", "ast", "az", "be", "bg", "bn", "br", "brx", "bs", "bs-Cyrl", "ca", "chr", "cs", "cy", "da", "de", "dsb", "el", "en", "es", "et", "eu", "fa", "ff-Adlm", "fi", "fil", "fo", "fr", "ga", "gd", "gl", "gu", "ha", "he", "hi", "hi-Latn", "hr", "hsb", "hu", "hy", "ia", "id", "ig", "is", "it", "ja", "jv", "ka", "kea", "kgp", "kk", "km", "kn", "ko", "kok", "ky", "lo", "lt", "lv", "mai", "mi", "mk", "ml", "mn", "mr", "ms", "my", "ne", "nl", "nn", "no", "or", "pa", "pcm", "pl", "ps", "pt", "pt-PT", "qu", "ro", "ru", "sc", "sd", "si", "sk", "sl", "so", "sq", "sr", "sr-BA", "sr-Latn", "sr-Latn-BA", "sv", "sw", "ta", "te", "tg", "th", "ti", "tk", "to", "tr", "tt", "uk", "und", "ur", "uz", "uz-Cyrl", "vi", "wo", "yrl", "yue", "yue-Hans", "zh", "zh-HK", "zh-Hant", "zh-MO", "zu"];
                let mut metadata = icu_provider::DataResponseMetadata::default();
                let payload = if let Ok(payload) = KEYS.binary_search_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse()).map(|i| *unsafe { VALUES.get_unchecked(i) }) {
                    payload
                } else {
                    const FALLBACKER: icu::locid_transform::fallback::LocaleFallbackerWithConfig<'static> = icu::locid_transform::fallback::LocaleFallbacker::new().for_config(<icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.fallback_config());
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
