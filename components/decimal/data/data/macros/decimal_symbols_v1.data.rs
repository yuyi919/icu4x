// @generated
/// Implement `DataProvider<DecimalSymbolsV1Marker>` on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_decimal_symbols_v1 {
    ($ provider : ty) => {
        #[clippy::msrv = "1.67"]
        const _: () = <$provider>::MUST_USE_MAKE_PROVIDER_MACRO;
        #[clippy::msrv = "1.67"]
        impl icu_provider::DataProvider<icu::decimal::provider::DecimalSymbolsV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu::decimal::provider::DecimalSymbolsV1Marker>, icu_provider::DataError> {
                static AST: <icu::decimal::provider::DecimalSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::decimal::provider::DecimalSymbolsV1 { minus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("-"), suffix: alloc::borrow::Cow::Borrowed("") }, plus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("+"), suffix: alloc::borrow::Cow::Borrowed("") }, decimal_separator: alloc::borrow::Cow::Borrowed(","), grouping_separator: alloc::borrow::Cow::Borrowed("."), grouping_sizes: icu::decimal::provider::GroupingSizesV1 { primary: 3u8, secondary: 3u8, min_grouping: 1u8 }, digits: ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'] };
                static LO_U_NU_LAOO: <icu::decimal::provider::DecimalSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::decimal::provider::DecimalSymbolsV1 { minus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("-"), suffix: alloc::borrow::Cow::Borrowed("") }, plus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("+"), suffix: alloc::borrow::Cow::Borrowed("") }, decimal_separator: alloc::borrow::Cow::Borrowed(","), grouping_separator: alloc::borrow::Cow::Borrowed("."), grouping_sizes: icu::decimal::provider::GroupingSizesV1 { primary: 3u8, secondary: 3u8, min_grouping: 1u8 }, digits: ['໐', '໑', '໒', '໓', '໔', '໕', '໖', '໗', '໘', '໙'] };
                static KM_U_NU_KHMR: <icu::decimal::provider::DecimalSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::decimal::provider::DecimalSymbolsV1 { minus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("-"), suffix: alloc::borrow::Cow::Borrowed("") }, plus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("+"), suffix: alloc::borrow::Cow::Borrowed("") }, decimal_separator: alloc::borrow::Cow::Borrowed(","), grouping_separator: alloc::borrow::Cow::Borrowed("."), grouping_sizes: icu::decimal::provider::GroupingSizesV1 { primary: 3u8, secondary: 3u8, min_grouping: 1u8 }, digits: ['០', '១', '២', '៣', '៤', '៥', '៦', '៧', '៨', '៩'] };
                static JV_U_NU_JAVA: <icu::decimal::provider::DecimalSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::decimal::provider::DecimalSymbolsV1 { minus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("-"), suffix: alloc::borrow::Cow::Borrowed("") }, plus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("+"), suffix: alloc::borrow::Cow::Borrowed("") }, decimal_separator: alloc::borrow::Cow::Borrowed(","), grouping_separator: alloc::borrow::Cow::Borrowed("."), grouping_sizes: icu::decimal::provider::GroupingSizesV1 { primary: 3u8, secondary: 3u8, min_grouping: 1u8 }, digits: ['꧐', '꧑', '꧒', '꧓', '꧔', '꧕', '꧖', '꧗', '꧘', '꧙'] };
                static ES: <icu::decimal::provider::DecimalSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::decimal::provider::DecimalSymbolsV1 { minus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("-"), suffix: alloc::borrow::Cow::Borrowed("") }, plus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("+"), suffix: alloc::borrow::Cow::Borrowed("") }, decimal_separator: alloc::borrow::Cow::Borrowed(","), grouping_separator: alloc::borrow::Cow::Borrowed("."), grouping_sizes: icu::decimal::provider::GroupingSizesV1 { primary: 3u8, secondary: 3u8, min_grouping: 2u8 }, digits: ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'] };
                static FR: <icu::decimal::provider::DecimalSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::decimal::provider::DecimalSymbolsV1 { minus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("-"), suffix: alloc::borrow::Cow::Borrowed("") }, plus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("+"), suffix: alloc::borrow::Cow::Borrowed("") }, decimal_separator: alloc::borrow::Cow::Borrowed(","), grouping_separator: alloc::borrow::Cow::Borrowed("\u{202f}"), grouping_sizes: icu::decimal::provider::GroupingSizesV1 { primary: 3u8, secondary: 3u8, min_grouping: 1u8 }, digits: ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'] };
                static AF: <icu::decimal::provider::DecimalSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::decimal::provider::DecimalSymbolsV1 { minus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("-"), suffix: alloc::borrow::Cow::Borrowed("") }, plus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("+"), suffix: alloc::borrow::Cow::Borrowed("") }, decimal_separator: alloc::borrow::Cow::Borrowed(","), grouping_separator: alloc::borrow::Cow::Borrowed("\u{a0}"), grouping_sizes: icu::decimal::provider::GroupingSizesV1 { primary: 3u8, secondary: 3u8, min_grouping: 1u8 }, digits: ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'] };
                static BE: <icu::decimal::provider::DecimalSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::decimal::provider::DecimalSymbolsV1 { minus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("-"), suffix: alloc::borrow::Cow::Borrowed("") }, plus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("+"), suffix: alloc::borrow::Cow::Borrowed("") }, decimal_separator: alloc::borrow::Cow::Borrowed(","), grouping_separator: alloc::borrow::Cow::Borrowed("\u{a0}"), grouping_sizes: icu::decimal::provider::GroupingSizesV1 { primary: 3u8, secondary: 3u8, min_grouping: 2u8 }, digits: ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'] };
                static AS_U_NU_LATN: <icu::decimal::provider::DecimalSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::decimal::provider::DecimalSymbolsV1 { minus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("-"), suffix: alloc::borrow::Cow::Borrowed("") }, plus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("+"), suffix: alloc::borrow::Cow::Borrowed("") }, decimal_separator: alloc::borrow::Cow::Borrowed("."), grouping_separator: alloc::borrow::Cow::Borrowed(","), grouping_sizes: icu::decimal::provider::GroupingSizesV1 { primary: 3u8, secondary: 2u8, min_grouping: 1u8 }, digits: ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'] };
                static BRX_U_NU_DEVA: <icu::decimal::provider::DecimalSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::decimal::provider::DecimalSymbolsV1 { minus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("-"), suffix: alloc::borrow::Cow::Borrowed("") }, plus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("+"), suffix: alloc::borrow::Cow::Borrowed("") }, decimal_separator: alloc::borrow::Cow::Borrowed("."), grouping_separator: alloc::borrow::Cow::Borrowed(","), grouping_sizes: icu::decimal::provider::GroupingSizesV1 { primary: 3u8, secondary: 2u8, min_grouping: 1u8 }, digits: ['०', '१', '२', '३', '४', '५', '६', '७', '८', '९'] };
                static AS: <icu::decimal::provider::DecimalSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::decimal::provider::DecimalSymbolsV1 { minus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("-"), suffix: alloc::borrow::Cow::Borrowed("") }, plus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("+"), suffix: alloc::borrow::Cow::Borrowed("") }, decimal_separator: alloc::borrow::Cow::Borrowed("."), grouping_separator: alloc::borrow::Cow::Borrowed(","), grouping_sizes: icu::decimal::provider::GroupingSizesV1 { primary: 3u8, secondary: 2u8, min_grouping: 1u8 }, digits: ['০', '১', '২', '৩', '৪', '৫', '৬', '৭', '৮', '৯'] };
                static PA_U_NU_GURU: <icu::decimal::provider::DecimalSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::decimal::provider::DecimalSymbolsV1 { minus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("-"), suffix: alloc::borrow::Cow::Borrowed("") }, plus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("+"), suffix: alloc::borrow::Cow::Borrowed("") }, decimal_separator: alloc::borrow::Cow::Borrowed("."), grouping_separator: alloc::borrow::Cow::Borrowed(","), grouping_sizes: icu::decimal::provider::GroupingSizesV1 { primary: 3u8, secondary: 2u8, min_grouping: 1u8 }, digits: ['੦', '੧', '੨', '੩', '੪', '੫', '੬', '੭', '੮', '੯'] };
                static GU_U_NU_GUJR: <icu::decimal::provider::DecimalSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::decimal::provider::DecimalSymbolsV1 { minus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("-"), suffix: alloc::borrow::Cow::Borrowed("") }, plus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("+"), suffix: alloc::borrow::Cow::Borrowed("") }, decimal_separator: alloc::borrow::Cow::Borrowed("."), grouping_separator: alloc::borrow::Cow::Borrowed(","), grouping_sizes: icu::decimal::provider::GroupingSizesV1 { primary: 3u8, secondary: 2u8, min_grouping: 1u8 }, digits: ['૦', '૧', '૨', '૩', '૪', '૫', '૬', '૭', '૮', '૯'] };
                static OR_U_NU_ORYA: <icu::decimal::provider::DecimalSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::decimal::provider::DecimalSymbolsV1 { minus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("-"), suffix: alloc::borrow::Cow::Borrowed("") }, plus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("+"), suffix: alloc::borrow::Cow::Borrowed("") }, decimal_separator: alloc::borrow::Cow::Borrowed("."), grouping_separator: alloc::borrow::Cow::Borrowed(","), grouping_sizes: icu::decimal::provider::GroupingSizesV1 { primary: 3u8, secondary: 2u8, min_grouping: 1u8 }, digits: ['୦', '୧', '୨', '୩', '୪', '୫', '୬', '୭', '୮', '୯'] };
                static TA_U_NU_TAMLDEC: <icu::decimal::provider::DecimalSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::decimal::provider::DecimalSymbolsV1 { minus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("-"), suffix: alloc::borrow::Cow::Borrowed("") }, plus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("+"), suffix: alloc::borrow::Cow::Borrowed("") }, decimal_separator: alloc::borrow::Cow::Borrowed("."), grouping_separator: alloc::borrow::Cow::Borrowed(","), grouping_sizes: icu::decimal::provider::GroupingSizesV1 { primary: 3u8, secondary: 2u8, min_grouping: 1u8 }, digits: ['௦', '௧', '௨', '௩', '௪', '௫', '௬', '௭', '௮', '௯'] };
                static ML_U_NU_MLYM: <icu::decimal::provider::DecimalSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::decimal::provider::DecimalSymbolsV1 { minus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("-"), suffix: alloc::borrow::Cow::Borrowed("") }, plus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("+"), suffix: alloc::borrow::Cow::Borrowed("") }, decimal_separator: alloc::borrow::Cow::Borrowed("."), grouping_separator: alloc::borrow::Cow::Borrowed(","), grouping_sizes: icu::decimal::provider::GroupingSizesV1 { primary: 3u8, secondary: 2u8, min_grouping: 1u8 }, digits: ['൦', '൧', '൨', '൩', '൪', '൫', '൬', '൭', '൮', '൯'] };
                static UND: <icu::decimal::provider::DecimalSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::decimal::provider::DecimalSymbolsV1 { minus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("-"), suffix: alloc::borrow::Cow::Borrowed("") }, plus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("+"), suffix: alloc::borrow::Cow::Borrowed("") }, decimal_separator: alloc::borrow::Cow::Borrowed("."), grouping_separator: alloc::borrow::Cow::Borrowed(","), grouping_sizes: icu::decimal::provider::GroupingSizesV1 { primary: 3u8, secondary: 3u8, min_grouping: 1u8 }, digits: ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'] };
                static BGC: <icu::decimal::provider::DecimalSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::decimal::provider::DecimalSymbolsV1 { minus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("-"), suffix: alloc::borrow::Cow::Borrowed("") }, plus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("+"), suffix: alloc::borrow::Cow::Borrowed("") }, decimal_separator: alloc::borrow::Cow::Borrowed("."), grouping_separator: alloc::borrow::Cow::Borrowed(","), grouping_sizes: icu::decimal::provider::GroupingSizesV1 { primary: 3u8, secondary: 3u8, min_grouping: 1u8 }, digits: ['०', '१', '२', '३', '४', '५', '६', '७', '८', '९'] };
                static MNI: <icu::decimal::provider::DecimalSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::decimal::provider::DecimalSymbolsV1 { minus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("-"), suffix: alloc::borrow::Cow::Borrowed("") }, plus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("+"), suffix: alloc::borrow::Cow::Borrowed("") }, decimal_separator: alloc::borrow::Cow::Borrowed("."), grouping_separator: alloc::borrow::Cow::Borrowed(","), grouping_sizes: icu::decimal::provider::GroupingSizesV1 { primary: 3u8, secondary: 3u8, min_grouping: 1u8 }, digits: ['০', '১', '২', '৩', '৪', '৫', '৬', '৭', '৮', '৯'] };
                static TE_U_NU_TELU: <icu::decimal::provider::DecimalSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::decimal::provider::DecimalSymbolsV1 { minus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("-"), suffix: alloc::borrow::Cow::Borrowed("") }, plus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("+"), suffix: alloc::borrow::Cow::Borrowed("") }, decimal_separator: alloc::borrow::Cow::Borrowed("."), grouping_separator: alloc::borrow::Cow::Borrowed(","), grouping_sizes: icu::decimal::provider::GroupingSizesV1 { primary: 3u8, secondary: 3u8, min_grouping: 1u8 }, digits: ['౦', '౧', '౨', '౩', '౪', '౫', '౬', '౭', '౮', '౯'] };
                static KN_U_NU_KNDA: <icu::decimal::provider::DecimalSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::decimal::provider::DecimalSymbolsV1 { minus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("-"), suffix: alloc::borrow::Cow::Borrowed("") }, plus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("+"), suffix: alloc::borrow::Cow::Borrowed("") }, decimal_separator: alloc::borrow::Cow::Borrowed("."), grouping_separator: alloc::borrow::Cow::Borrowed(","), grouping_sizes: icu::decimal::provider::GroupingSizesV1 { primary: 3u8, secondary: 3u8, min_grouping: 1u8 }, digits: ['೦', '೧', '೨', '೩', '೪', '೫', '೬', '೭', '೮', '೯'] };
                static TH_U_NU_THAI: <icu::decimal::provider::DecimalSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::decimal::provider::DecimalSymbolsV1 { minus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("-"), suffix: alloc::borrow::Cow::Borrowed("") }, plus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("+"), suffix: alloc::borrow::Cow::Borrowed("") }, decimal_separator: alloc::borrow::Cow::Borrowed("."), grouping_separator: alloc::borrow::Cow::Borrowed(","), grouping_sizes: icu::decimal::provider::GroupingSizesV1 { primary: 3u8, secondary: 3u8, min_grouping: 1u8 }, digits: ['๐', '๑', '๒', '๓', '๔', '๕', '๖', '๗', '๘', '๙'] };
                static MY: <icu::decimal::provider::DecimalSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::decimal::provider::DecimalSymbolsV1 { minus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("-"), suffix: alloc::borrow::Cow::Borrowed("") }, plus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("+"), suffix: alloc::borrow::Cow::Borrowed("") }, decimal_separator: alloc::borrow::Cow::Borrowed("."), grouping_separator: alloc::borrow::Cow::Borrowed(","), grouping_sizes: icu::decimal::provider::GroupingSizesV1 { primary: 3u8, secondary: 3u8, min_grouping: 1u8 }, digits: ['၀', '၁', '၂', '၃', '၄', '၅', '၆', '၇', '၈', '၉'] };
                static SAT: <icu::decimal::provider::DecimalSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::decimal::provider::DecimalSymbolsV1 { minus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("-"), suffix: alloc::borrow::Cow::Borrowed("") }, plus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("+"), suffix: alloc::borrow::Cow::Borrowed("") }, decimal_separator: alloc::borrow::Cow::Borrowed("."), grouping_separator: alloc::borrow::Cow::Borrowed(","), grouping_sizes: icu::decimal::provider::GroupingSizesV1 { primary: 3u8, secondary: 3u8, min_grouping: 1u8 }, digits: ['᱐', '᱑', '᱒', '᱓', '᱔', '᱕', '᱖', '᱗', '᱘', '᱙'] };
                static YUE_HANS_U_NU_HANIDEC: <icu::decimal::provider::DecimalSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::decimal::provider::DecimalSymbolsV1 { minus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("-"), suffix: alloc::borrow::Cow::Borrowed("") }, plus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("+"), suffix: alloc::borrow::Cow::Borrowed("") }, decimal_separator: alloc::borrow::Cow::Borrowed("."), grouping_separator: alloc::borrow::Cow::Borrowed(","), grouping_sizes: icu::decimal::provider::GroupingSizesV1 { primary: 3u8, secondary: 3u8, min_grouping: 1u8 }, digits: ['〇', '一', '二', '三', '四', '五', '六', '七', '八', '九'] };
                static XH: <icu::decimal::provider::DecimalSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::decimal::provider::DecimalSymbolsV1 { minus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("-"), suffix: alloc::borrow::Cow::Borrowed("") }, plus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("+"), suffix: alloc::borrow::Cow::Borrowed("") }, decimal_separator: alloc::borrow::Cow::Borrowed("."), grouping_separator: alloc::borrow::Cow::Borrowed("\u{a0}"), grouping_sizes: icu::decimal::provider::GroupingSizesV1 { primary: 3u8, secondary: 3u8, min_grouping: 1u8 }, digits: ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'] };
                static KS_U_NU_LATN: <icu::decimal::provider::DecimalSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::decimal::provider::DecimalSymbolsV1 { minus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("-"), suffix: alloc::borrow::Cow::Borrowed("") }, plus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("+"), suffix: alloc::borrow::Cow::Borrowed("") }, decimal_separator: alloc::borrow::Cow::Borrowed("."), grouping_separator: alloc::borrow::Cow::Borrowed("،"), grouping_sizes: icu::decimal::provider::GroupingSizesV1 { primary: 3u8, secondary: 3u8, min_grouping: 1u8 }, digits: ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'] };
                static DE_CH: <icu::decimal::provider::DecimalSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::decimal::provider::DecimalSymbolsV1 { minus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("-"), suffix: alloc::borrow::Cow::Borrowed("") }, plus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("+"), suffix: alloc::borrow::Cow::Borrowed("") }, decimal_separator: alloc::borrow::Cow::Borrowed("."), grouping_separator: alloc::borrow::Cow::Borrowed("’"), grouping_sizes: icu::decimal::provider::GroupingSizesV1 { primary: 3u8, secondary: 3u8, min_grouping: 1u8 }, digits: ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'] };
                static FF_ADLM_U_NU_LATN: <icu::decimal::provider::DecimalSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::decimal::provider::DecimalSymbolsV1 { minus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("-"), suffix: alloc::borrow::Cow::Borrowed("") }, plus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("+"), suffix: alloc::borrow::Cow::Borrowed("") }, decimal_separator: alloc::borrow::Cow::Borrowed("."), grouping_separator: alloc::borrow::Cow::Borrowed("⹁"), grouping_sizes: icu::decimal::provider::GroupingSizesV1 { primary: 3u8, secondary: 3u8, min_grouping: 1u8 }, digits: ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'] };
                static FF_ADLM: <icu::decimal::provider::DecimalSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::decimal::provider::DecimalSymbolsV1 { minus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("-"), suffix: alloc::borrow::Cow::Borrowed("") }, plus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("+"), suffix: alloc::borrow::Cow::Borrowed("") }, decimal_separator: alloc::borrow::Cow::Borrowed("."), grouping_separator: alloc::borrow::Cow::Borrowed("⹁"), grouping_sizes: icu::decimal::provider::GroupingSizesV1 { primary: 3u8, secondary: 3u8, min_grouping: 1u8 }, digits: ['𞥐', '𞥑', '𞥒', '𞥓', '𞥔', '𞥕', '𞥖', '𞥗', '𞥘', '𞥙'] };
                static AR_DZ: <icu::decimal::provider::DecimalSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::decimal::provider::DecimalSymbolsV1 { minus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("\u{200e}-"), suffix: alloc::borrow::Cow::Borrowed("") }, plus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("\u{200e}+"), suffix: alloc::borrow::Cow::Borrowed("") }, decimal_separator: alloc::borrow::Cow::Borrowed(","), grouping_separator: alloc::borrow::Cow::Borrowed("."), grouping_sizes: icu::decimal::provider::GroupingSizesV1 { primary: 3u8, secondary: 3u8, min_grouping: 1u8 }, digits: ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'] };
                static AR_AE: <icu::decimal::provider::DecimalSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::decimal::provider::DecimalSymbolsV1 { minus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("\u{200e}-"), suffix: alloc::borrow::Cow::Borrowed("") }, plus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("\u{200e}+"), suffix: alloc::borrow::Cow::Borrowed("") }, decimal_separator: alloc::borrow::Cow::Borrowed("."), grouping_separator: alloc::borrow::Cow::Borrowed(","), grouping_sizes: icu::decimal::provider::GroupingSizesV1 { primary: 3u8, secondary: 3u8, min_grouping: 1u8 }, digits: ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'] };
                static KS: <icu::decimal::provider::DecimalSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::decimal::provider::DecimalSymbolsV1 { minus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("\u{200e}-\u{200e}"), suffix: alloc::borrow::Cow::Borrowed("") }, plus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("\u{200e}+\u{200e}"), suffix: alloc::borrow::Cow::Borrowed("") }, decimal_separator: alloc::borrow::Cow::Borrowed("٫"), grouping_separator: alloc::borrow::Cow::Borrowed("٬"), grouping_sizes: icu::decimal::provider::GroupingSizesV1 { primary: 3u8, secondary: 3u8, min_grouping: 1u8 }, digits: ['۰', '۱', '۲', '۳', '۴', '۵', '۶', '۷', '۸', '۹'] };
                static PS_U_NU_LATN: <icu::decimal::provider::DecimalSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::decimal::provider::DecimalSymbolsV1 { minus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("\u{200e}−"), suffix: alloc::borrow::Cow::Borrowed("") }, plus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("\u{200e}+"), suffix: alloc::borrow::Cow::Borrowed("") }, decimal_separator: alloc::borrow::Cow::Borrowed(","), grouping_separator: alloc::borrow::Cow::Borrowed("."), grouping_sizes: icu::decimal::provider::GroupingSizesV1 { primary: 3u8, secondary: 3u8, min_grouping: 1u8 }, digits: ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'] };
                static FA_U_NU_LATN: <icu::decimal::provider::DecimalSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::decimal::provider::DecimalSymbolsV1 { minus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("\u{200e}−"), suffix: alloc::borrow::Cow::Borrowed("") }, plus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("\u{200e}+"), suffix: alloc::borrow::Cow::Borrowed("") }, decimal_separator: alloc::borrow::Cow::Borrowed("."), grouping_separator: alloc::borrow::Cow::Borrowed(","), grouping_sizes: icu::decimal::provider::GroupingSizesV1 { primary: 3u8, secondary: 3u8, min_grouping: 1u8 }, digits: ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'] };
                static FA: <icu::decimal::provider::DecimalSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::decimal::provider::DecimalSymbolsV1 { minus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("\u{200e}−"), suffix: alloc::borrow::Cow::Borrowed("") }, plus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("\u{200e}+"), suffix: alloc::borrow::Cow::Borrowed("") }, decimal_separator: alloc::borrow::Cow::Borrowed("٫"), grouping_separator: alloc::borrow::Cow::Borrowed("٬"), grouping_sizes: icu::decimal::provider::GroupingSizesV1 { primary: 3u8, secondary: 3u8, min_grouping: 1u8 }, digits: ['۰', '۱', '۲', '۳', '۴', '۵', '۶', '۷', '۸', '۹'] };
                static AR: <icu::decimal::provider::DecimalSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::decimal::provider::DecimalSymbolsV1 { minus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("\u{61c}-"), suffix: alloc::borrow::Cow::Borrowed("") }, plus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("\u{61c}+"), suffix: alloc::borrow::Cow::Borrowed("") }, decimal_separator: alloc::borrow::Cow::Borrowed("٫"), grouping_separator: alloc::borrow::Cow::Borrowed("٬"), grouping_sizes: icu::decimal::provider::GroupingSizesV1 { primary: 3u8, secondary: 3u8, min_grouping: 1u8 }, digits: ['٠', '١', '٢', '٣', '٤', '٥', '٦', '٧', '٨', '٩'] };
                static EU: <icu::decimal::provider::DecimalSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::decimal::provider::DecimalSymbolsV1 { minus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("−"), suffix: alloc::borrow::Cow::Borrowed("") }, plus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("+"), suffix: alloc::borrow::Cow::Borrowed("") }, decimal_separator: alloc::borrow::Cow::Borrowed(","), grouping_separator: alloc::borrow::Cow::Borrowed("."), grouping_sizes: icu::decimal::provider::GroupingSizesV1 { primary: 3u8, secondary: 3u8, min_grouping: 1u8 }, digits: ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'] };
                static FI: <icu::decimal::provider::DecimalSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::decimal::provider::DecimalSymbolsV1 { minus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("−"), suffix: alloc::borrow::Cow::Borrowed("") }, plus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("+"), suffix: alloc::borrow::Cow::Borrowed("") }, decimal_separator: alloc::borrow::Cow::Borrowed(","), grouping_separator: alloc::borrow::Cow::Borrowed("\u{a0}"), grouping_sizes: icu::decimal::provider::GroupingSizesV1 { primary: 3u8, secondary: 3u8, min_grouping: 1u8 }, digits: ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'] };
                static ET: <icu::decimal::provider::DecimalSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::decimal::provider::DecimalSymbolsV1 { minus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("−"), suffix: alloc::borrow::Cow::Borrowed("") }, plus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("+"), suffix: alloc::borrow::Cow::Borrowed("") }, decimal_separator: alloc::borrow::Cow::Borrowed(","), grouping_separator: alloc::borrow::Cow::Borrowed("\u{a0}"), grouping_sizes: icu::decimal::provider::GroupingSizesV1 { primary: 3u8, secondary: 3u8, min_grouping: 2u8 }, digits: ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'] };
                static RM: <icu::decimal::provider::DecimalSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::decimal::provider::DecimalSymbolsV1 { minus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("−"), suffix: alloc::borrow::Cow::Borrowed("") }, plus_sign_affixes: icu::decimal::provider::AffixesV1 { prefix: alloc::borrow::Cow::Borrowed("+"), suffix: alloc::borrow::Cow::Borrowed("") }, decimal_separator: alloc::borrow::Cow::Borrowed("."), grouping_separator: alloc::borrow::Cow::Borrowed("’"), grouping_sizes: icu::decimal::provider::GroupingSizesV1 { primary: 3u8, secondary: 3u8, min_grouping: 1u8 }, digits: ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'] };
                static VALUES: [&<icu::decimal::provider::DecimalSymbolsV1Marker as icu_provider::DataMarker>::Yokeable; 168usize] = [&AF, &AR, &AR_AE, &AR_DZ, &AR_AE, &AR_DZ, &AR_DZ, &AR_DZ, &AR_DZ, &AR_DZ, &AR_AE, &AS, &AS_U_NU_LATN, &AST, &AST, &BE, &BE, &BGC, &BGC, &AS, &AS_U_NU_LATN, &AF, &AS_U_NU_LATN, &BRX_U_NU_DEVA, &AST, &AST, &AST, &AF, &AF, &AST, &AST, &AF, &DE_CH, &DE_CH, &BGC, &AST, &AST, &AST, &AST, &DE_CH, &AST, &AST, &AF, &AS_U_NU_LATN, &AST, &AF, &AST, &ES, &AST, &AST, &AST, &AST, &AF, &AST, &AST, &AST, &AST, &ET, &EU, &FA, &FA_U_NU_LATN, &FF_ADLM, &FF_ADLM_U_NU_LATN, &FI, &EU, &FR, &AF, &AST, &AST, &AST, &AS_U_NU_LATN, &GU_U_NU_GUJR, &AR_AE, &AS_U_NU_LATN, &BRX_U_NU_DEVA, &EU, &AST, &AF, &AF, &ES, &AST, &AST, &AST, &DE_CH, &AST, &JV_U_NU_JAVA, &BE, &AF, &AST, &AF, &AST, &KM_U_NU_KHMR, &KN_U_NU_KNDA, &BGC, &KS, &KS_U_NU_LATN, &AF, &AST, &LO_U_NU_LAOO, &FI, &BE, &BGC, &AST, &AS_U_NU_LATN, &ML_U_NU_MLYM, &MNI, &BRX_U_NU_DEVA, &AS_U_NU_LATN, &AST, &AST, &MY, &BRX_U_NU_DEVA, &AS_U_NU_LATN, &AST, &FI, &AS_U_NU_LATN, &OR_U_NU_ORYA, &AS_U_NU_LATN, &PA_U_NU_GURU, &BE, &KS, &PS_U_NU_LATN, &AST, &AF, &BE, &AST, &BGC, &RM, &AST, &AF, &BE, &BRX_U_NU_DEVA, &AS_U_NU_LATN, &SAT, &AST, &AR, &AF, &EU, &BE, &AST, &AST, &AST, &FI, &AST, &AS_U_NU_LATN, &TA_U_NU_TAMLDEC, &AS_U_NU_LATN, &TE_U_NU_TELU, &AF, &TH_U_NU_THAI, &AF, &AST, &AF, &AF, &UND, &AR_AE, &KS, &KS, &AF, &AF, &AST, &AST, &XH, &AST, &YUE_HANS_U_NU_HANIDEC, &YUE_HANS_U_NU_HANIDEC, &YUE_HANS_U_NU_HANIDEC, &YUE_HANS_U_NU_HANIDEC];
                static KEYS: [&str; 168usize] = ["af", "ar", "ar-AE", "ar-DZ", "ar-EH", "ar-LB-u-nu-latn", "ar-LY", "ar-MA", "ar-MR-u-nu-latn", "ar-TN", "ar-u-nu-latn", "as", "as-u-nu-latn", "ast", "az", "be", "bg", "bgc", "bho", "bn", "bn-u-nu-latn", "br", "brx", "brx-u-nu-deva", "bs", "bs-Cyrl", "ca", "cs", "cv", "da", "de", "de-AT", "de-CH", "de-LI", "doi-u-nu-deva", "dsb", "el", "en-AT", "en-BE", "en-CH", "en-DE", "en-DK", "en-FI", "en-IN", "en-NL", "en-SE", "en-SI", "es", "es-AR", "es-BO", "es-CL", "es-CO", "es-CR", "es-EC", "es-PY", "es-UY", "es-VE", "et", "eu", "fa", "fa-u-nu-latn", "ff-Adlm", "ff-Adlm-u-nu-latn", "fi", "fo", "fr", "fr-CA", "fr-LU", "fr-MA", "gl", "gu", "gu-u-nu-gujr", "he", "hi", "hi-u-nu-deva", "hr", "hsb", "hu", "hy", "ia", "id", "is", "it", "it-CH", "jv", "jv-u-nu-java", "ka", "kea", "kgp", "kk", "km", "km-u-nu-khmr", "kn-u-nu-knda", "kok-u-nu-deva", "ks", "ks-u-nu-latn", "ky", "lo", "lo-u-nu-laoo", "lt", "lv", "mai-u-nu-deva", "mk", "ml", "ml-u-nu-mlym", "mni", "mr", "mr-u-nu-latn", "ms-BN", "ms-ID", "my", "ne", "ne-u-nu-latn", "nl", "no", "or", "or-u-nu-orya", "pa", "pa-u-nu-guru", "pl", "ps", "ps-u-nu-latn", "pt", "pt-AO", "pt-PT", "qu-BO", "raj", "rm", "ro", "ru", "ru-UA", "sa", "sa-u-nu-latn", "sat", "sc", "sd", "sk", "sl", "sq", "sr", "sr-Latn", "su", "sv", "sw-CD", "ta", "ta-u-nu-tamldec", "te", "te-u-nu-telu", "tg", "th-u-nu-thai", "tk", "tr", "tt", "uk", "und", "ur", "ur-IN", "ur-u-nu-arabext", "uz", "uz-Cyrl", "vi", "wo", "xh", "yrl", "yue-Hans-u-nu-hanidec", "yue-u-nu-hanidec", "zh-Hant-u-nu-hanidec", "zh-u-nu-hanidec"];
                let mut metadata = icu_provider::DataResponseMetadata::default();
                let payload = if let Ok(payload) = KEYS.binary_search_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse()).map(|i| *unsafe { VALUES.get_unchecked(i) }) {
                    payload
                } else {
                    const FALLBACKER: icu::locid_transform::fallback::LocaleFallbackerWithConfig<'static> = icu::locid_transform::fallback::LocaleFallbacker::new().for_config(<icu::decimal::provider::DecimalSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.fallback_config());
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
