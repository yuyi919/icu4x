// @generated
/// Implement `DataProvider<CompatibilityDecompositionTablesV1Marker>` on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_normalizer_nfkdex_v1 {
    ($ provider : ty) => {
        #[clippy::msrv = "1.67"]
        const _: () = <$provider>::MUST_USE_MAKE_PROVIDER_MACRO;
        #[clippy::msrv = "1.67"]
        impl $provider {
            #[doc(hidden)]
            pub const SINGLETON_NORMALIZER_NFKDEX_V1: &'static <icu::normalizer::provider::CompatibilityDecompositionTablesV1Marker as icu_provider::DataMarker>::Yokeable = &icu::normalizer::provider::DecompositionTablesV1 { scalars16: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x140We\x150\x140\xDDR\x150\x140\xD7v\x150\x140Sb\x150\x140\xB9p\x150\x140\x89[\x150\x140\x8CN\x150\x140\tN\x150\x140,g\x150\xB30\xB30{0K0D\0J\0M\0R\0M\0D\0M\0C\0W\0C\0P\0P\0V\0S\0S\0S\0D\0M\0V\0H\0V\0W\0Z\0C\0D\0\x140S\0\x150(\0Z\0)\0(\0Y\0)\0(\0X\0)\0(\0W\0)\0(\0V\0)\0(\0U\0)\0(\0T\0)\0(\0S\0)\0(\0R\0)\0(\0Q\0)\0(\0P\0)\0(\0O\0)\0(\0N\0)\0(\0M\0)\0(\0L\0)\0(\0K\0)\0(\0J\0)\0(\0I\0)\0(\0H\0)\0(\0G\0)\0(\0F\0)\0(\0E\0)\0(\0D\0)\0(\0C\0)\0(\0B\0)\0(\0A\0)\09\0,\08\0,\x007\0,\x006\0,\x005\0,\x004\0,\x003\0,\x002\0,\x001\0,\x000\0,\x000\0.\0D\x06'\x06D\x06'\x06U\x06D\x06'\x06T\x06D\x06'\x06S\x06.\0.\0.\0.\0.\x001\x06\xCC\x06'\x06D\x06,\x06D\x06 \0,\x06D\x06'\x06D\x06G\x065\x06D\x06I\x06H\x063\x06D\x06E\x069\x06D\x06J\x06G\x061\x063\x06H\x06D\x065\x06D\x069\x06E\x06E\x06-\x06E\x06/\x06'\x06C\x06(\x061\x06'\x06D\x06D\x06G\x06B\x06D\x06\xD2\x065\x06D\x06\xD2\x06F\x06,\x06J\x063\x06.\x06J\x065\x06E\x06E\x069\x06,\x06E\x06C\x06E\x06E\x06(\x06-\x06J\x06A\x06E\x06J\x06E\x06,\x06J\x06-\x06,\x06J\x06,\x06-\x06J\x06F\x06,\x06-\x06D\x06,\x06E\x06E\x06.\x06J\x06C\x06E\x06J\x069\x06E\x06J\x06D\x06-\x06E\x06B\x06E\x06-\x06F\x06-\x06J\x06B\x06E\x06J\x06E\x06E\x06J\x06J\x06E\x06J\x06J\x06,\x06J\x06J\x06-\x06J\x06D\x06E\x06J\x06D\x06,\x06J\x066\x06-\x06J\x064\x06-\x06J\x065\x06-\x06J\x063\x06.\x06I\x06,\x06E\x06I\x06,\x06-\x06I\x06,\x06E\x06J\x06*\x06E\x06I\x06*\x06E\x06J\x06*\x06.\x06I\x06*\x06.\x06J\x06*\x06,\x06I\x06*\x06,\x06J\x06(\x06.\x06J\x06F\x06E\x06I\x06F\x06E\x06J\x06F\x06,\x06I\x06F\x06,\x06E\x06F\x06-\x06I\x06F\x06-\x06E\x06G\x06E\x06E\x06G\x06E\x06,\x06E\x06,\x06.\x06E\x06.\x06E\x06E\x06.\x06,\x06E\x06,\x06-\x06E\x06-\x06J\x06E\x06-\x06,\x06D\x06E\x06-\x06D\x06.\x06E\x06D\x06,\x06,\x06D\x06-\x06I\x06D\x06-\x06J\x06B\x06E\x06E\x06A\x06.\x06E\x06:\x06E\x06I\x06:\x06E\x06J\x06:\x06E\x06E\x069\x06E\x06I\x067\x06E\x06J\x067\x06E\x06E\x067\x06E\x06-\x066\x06.\x06E\x066\x06-\x06I\x064\x06E\x06E\x064\x06E\x06.\x064\x06,\x06J\x064\x06-\x06E\x065\x06-\x06-\x063\x06E\x06E\x063\x06E\x06,\x063\x06E\x06-\x063\x06,\x06I\x063\x06,\x06-\x063\x06-\x06,\x06-\x06E\x06I\x06-\x06E\x06J\x06,\x06E\x06-\x06*\x06E\x06.\x06*\x06E\x06-\x06*\x06E\x06,\x06*\x06.\x06E\x06*\x06-\x06E\x06*\x06-\x06,\x06*\x06,\x06E\x068\x06E\x064\x06.\x064\x06G\x063\x06G\x066\x061\x065\x061\x063\x061\x064\x061\x066\x06J\x066\x06I\x065\x06J\x065\x06I\x064\x06J\x064\x06I\x063\x06J\x063\x06I\x06:\x06J\x06:\x06I\x069\x06J\x069\x06I\x067\x06J\x067\x06I\x06@\x06P\x06Q\x06@\x06O\x06Q\x06@\x06N\x06Q\x06F\x06G\x06C\x06D\x06+\x06G\x06+\x06E\x06*\x06G\x06(\x06G\x06(\x06E\x06J\x06T\x06G\x06J\x06T\x06E\x06J\x06.\x06G\x06,\x06F\x06.\x06C\x06.\x06C\x06-\x06C\x06,\x06B\x06-\x06A\x06-\x06A\x06,\x06:\x06,\x067\x06-\x066\x06E\x066\x06,\x065\x06.\x06(\x06,\x06J\x06T\x06.\x06J\x06T\x06-\x06J\x06T\x06,\x06J\x06I\x06J\x062\x06J\x061\x06F\x06J\x06F\x06I\x06F\x06F\x06F\x062\x06F\x061\x06E\x06'\x06C\x06J\x06C\x06I\x06C\x06'\x06B\x06J\x06B\x06I\x06A\x06J\x06A\x06I\x06+\x06J\x06+\x06I\x06+\x06F\x06+\x062\x06+\x061\x06*\x06J\x06*\x06I\x06*\x06F\x06*\x062\x06*\x061\x06(\x06J\x06(\x06I\x06(\x06F\x06(\x062\x06J\x06T\x06J\x06J\x06T\x06I\x06J\x06T\x06F\x06J\x06T\x062\x06J\x06T\x061\x06 \0Q\x06p\x06 \0P\x06Q\x06 \0O\x06Q\x06 \0N\x06Q\x06 \0M\x06Q\x06 \0L\x06Q\x06G\x06I\x06.\x06-\x06+\x06,\x06J\x06T\x06\xD0\x06J\x06T\x06\xC8\x06J\x06T\x06\xC6\x06J\x06T\x06\xC7\x06J\x06T\x06H\x06J\x06T\x06\xD5\x06J\x06T\x06'\x06\xC7\x06t\x06\xD0\x05\xDC\x05t\x05m\x05~\x05v\x05t\x05k\x05t\x05e\x05t\x05v\x05s\0t\0f\0f\0l\0f\0f\0i\0g\0a\0l\x003\x001\0\xE5e3\x000\0\xE5e2\09\0\xE5e2\08\0\xE5e2\x007\0\xE5e2\x006\0\xE5e2\x005\0\xE5e2\x004\0\xE5e2\x003\0\xE5e2\x002\0\xE5e2\x001\0\xE5e2\x000\0\xE5e1\09\0\xE5e1\08\0\xE5e1\x007\0\xE5e1\x006\0\xE5e1\x005\0\xE5e1\x004\0\xE5e1\x003\0\xE5e1\x002\0\xE5e1\x001\0\xE5e1\x000\0\xE5eA\0\x15\"m\0V\0\x15\"m\0W\0b\0S\0v\0s\0r\0P\0R\0P\0P\0M\0p\0.\0m\0.\0P\0H\0m\0o\0l\0m\0i\0l\0m\0b\0l\0x\0l\0o\0g\0l\0n\0k\0t\0K\0M\0K\0K\0i\0n\0H\0P\0h\0a\0G\0y\0d\0B\0C\0o\0.\0C\0\x15\"k\0g\0c\0d\0c\0c\0B\0q\0a\0.\0m\0.\0M\0\xA9\x03k\0\xA9\x03M\0W\0k\0W\0\xBC\x03W\0n\0W\0p\0W\0k\0V\0\xBC\x03V\0n\0V\0p\0V\0m\0s\0\xBC\x03s\0n\0s\0p\0s\0r\0a\0d\0\x15\"s\x002\0G\0P\0a\0M\0P\0a\0k\0P\0a\0m\0\x15\"s\x002\0k\0m\x003\0c\0m\x003\0m\0m\x003\0k\0m\x002\0c\0m\x002\0m\0m\x002\0\xBC\x03m\0n\0m\0f\0m\0k\0l\0d\0l\0m\0l\0\xBC\x03l\0T\0H\0z\0G\0H\0z\0M\0H\0z\0k\0H\0z\0m\0g\0\xBC\x03g\0\xBC\x03F\0n\0F\0p\0F\0k\0c\0a\0l\0G\0B\0M\0B\0K\0B\0k\0A\0m\0A\0\xBC\x03A\0n\0A\0p\0A\0*h\x0F_\x1AO>y\x0Ef\xBBl'Yck-f\x8CTs^\x10bI\0U\0d\0m\x003\0d\0m\x002\0p\0c\0o\0V\0b\0a\0r\0A\0U\0d\0a\0h\0P\0a\x002\x004\0\xB9p2\x003\0\xB9p2\x002\0\xB9p2\x001\0\xB9p2\x000\0\xB9p1\09\0\xB9p1\08\0\xB9p1\x007\0\xB9p1\x006\0\xB9p1\x005\0\xB9p1\x004\0\xB9p1\x003\0\xB9p1\x002\0\xB9p1\x001\0\xB9p1\x000\0\xB9p\xEF0\xC30\xC80\xEC0\xF30\xC80\xB10\x990\xF30\xEC0\xE00\xEB0\xFC0\xD50\x990\xEB0\xEB0\xD20\x9A0\xFC0\xEA0\xE90\xEA0\xC30\xC80\xEB0\xE60\xA20\xF30\xE40\xFC0\xEB0\xE40\xFC0\xC80\x990\xE10\xFC0\xC80\xEB0\xE10\xAB0\x990\xC80\xF30\xDF0\xEA0\xCF0\x990\xFC0\xEB0\xDF0\xAF0\xED0\xF30\xDE0\xF30\xB70\xE70\xF30\xDE0\xEB0\xAF0\xDE0\xC30\xCF0\xDE0\xA40\xEB0\xDE0\xA40\xAF0\xED0\xDB0\xFC0\xF30\xDB0\xFC0\xEB0\xDB0\x9A0\xF30\xC80\x990\xDB0\xF30\xDB0\x990\xEB0\xC80\xDB0\x9A0\xA40\xF30\xC80\xD80\x990\xFC0\xBF0\xD80\x9A0\xFC0\xB70\x990\xD80\x9A0\xF30\xB90\xD80\xEB0\xC40\xD80\x9A0\xCB0\xD20\xD80\x9A0\xBD0\xD80\xAF0\xBF0\xFC0\xEB0\xD50\xE90\xF30\xD50\x990\xC30\xB70\xA70\xEB0\xD50\xA30\xFC0\xC80\xD50\xA10\xE90\xC30\xC80\x990\xD20\x990\xEB0\xD20\x9A0\xB30\xD20\x9A0\xAF0\xEB0\xD20\x9A0\xA20\xB90\xC80\xEB0\xCF0\x990\xFC0\xEC0\xEB0\xCF0\x9A0\xFC0\xC40\xCF0\x9A0\xFC0\xBB0\xF30\xC80\xCF0\xA40\xC40\xCE0\xC30\xC80\xCA0\xCE0\xC80\x990\xEB0\xC60\x990\xB70\xBF0\x990\xFC0\xB90\xBB0\xF30\xC10\xB70\xEA0\xF30\xAF0\x990\xB50\xF30\xC10\xFC0\xE00\xB50\xA40\xAF0\xEB0\xB30\xFC0\xDB0\x9A0\xB30\xEB0\xCA0\xB10\xFC0\xB90\xAF0\xED0\xFC0\xCD0\xAF0\xEB0\xBB0\x990\xA40\xED0\xAF0\x990\xE90\xE00\xC80\xF30\xAD0\xED0\xEF0\xC30\xC80\xAD0\xED0\xE10\xFC0\xC80\xEB0\xAD0\xED0\xAF0\x990\xE90\xE00\xAD0\x990\xEB0\xBF0\x990\xFC0\xAD0\xE50\xEA0\xFC0\xAD0\x990\xCB0\xFC0\xAD0\x990\xAB0\x990\xAB0\x990\xF30\xDE0\xAB0\x990\xED0\xF30\xAB0\xED0\xEA0\xFC0\xAB0\xE90\xC30\xC80\xAB0\xA40\xEA0\xAA0\xFC0\xE00\xAA0\xF30\xB90\xA80\xFC0\xAB0\xFC0\xA80\xB90\xAF0\xFC0\xC80\x990\xA60\xA90\xF30\xA40\xF30\xC10\xA40\xCB0\xF30\xAF0\x990\xA20\xFC0\xEB0\xA20\xF30\xD80\x9A0\xA20\xA20\xEB0\xD50\xA10\xA20\xCF0\x9A0\xFC0\xC80\xE4N\x8CTL\0T\0D\0e\0V\0e\0r\0g\0H\0g\x001\x002\0\x08g1\x001\0\x08g1\x000\0\x08g9\0\x08g8\0\x08g7\0\x08g6\0\x08g5\0\x08g4\0\x08g3\0\x08g5\x000\x004\09\x004\08\x004\x007\x004\x006\x004\x005\x004\x004\x004\x003\x004\x002\x004\x001\x004\x000\x003\09\x003\08\x003\x007\x003\x006\0\x0B\x11n\x11\x0C\x11n\x11\x0B\x11t\x11\x0E\x11a\x11\xB7\x11\0\x11i\x11\x12\x11a\x11\x11\x11a\x11\x10\x11a\x11\x0F\x11a\x11\x0C\x11a\x11\x0B\x11a\x11\t\x11a\x11\x07\x11a\x11\x06\x11a\x11\x05\x11a\x11\x03\x11a\x11\x02\x11a\x11\0\x11a\x113\x005\x003\x003\x003\x002\0P\0T\0E\0(\0\xF3\x81)\0(\0\xEA\x81)\0(\0\x11O)\0(\0my)\0(\0TS)\0(\0\xC7\x8C)\0(\0\x01O)\0(\0\xE3v)\0(\0f[)\0(\0|T)\0(\0\xE3N)\0(\0\xB4R)\0(\0]y)\0(\0\xA1\x8C)\0(\0yr)\0(\0\rT)\0(\0>y)\0(\0\tg)\0(\0*h)\0(\0\xE5e)\0(\0\x1FW)\0(\0\xD1\x91)\0(\0(g)\0(\x004l)\0(\0kp)\0(\0\x08g)\0(\0AS)\0(\0]N)\0(\0kQ)\0(\0\x03N)\0(\0mQ)\0(\0\x94N)\0(\0\xDBV)\0(\0\tN)\0(\0\x8CN)\0(\0\0N)\0(\0\x0B\x11i\x11\x12\x11n\x11)\0(\0\x0B\x11i\x11\x0C\x11e\x11\xAB\x11)\0(\0\x0C\x11n\x11)\0(\0\x12\x11a\x11)\0(\0\x11\x11a\x11)\0(\0\x10\x11a\x11)\0(\0\x0F\x11a\x11)\0(\0\x0E\x11a\x11)\0(\0\x0C\x11a\x11)\0(\0\x0B\x11a\x11)\0(\0\t\x11a\x11)\0(\0\x07\x11a\x11)\0(\0\x06\x11a\x11)\0(\0\x05\x11a\x11)\0(\0\x03\x11a\x11)\0(\0\x02\x11a\x11)\0(\0\0\x11a\x11)\0(\0\x12\x11)\0(\0\x11\x11)\0(\0\x10\x11)\0(\0\x0F\x11)\0(\0\x0E\x11)\0(\0\x0C\x11)\0(\0\x0B\x11)\0(\0\t\x11)\0(\0\x07\x11)\0(\0\x06\x11)\0(\0\x05\x11)\0(\0\x03\x11)\0(\0\x02\x11)\0(\0\0\x11)\0\xB30\xC80\x880\x8A0=\0=\0=\0:\0:\0=\0+\"+\"+\"+\"(\0z\0)\0(\0y\0)\0(\0x\0)\0(\0w\0)\0(\0v\0)\0(\0u\0)\0(\0t\0)\0(\0s\0)\0(\0r\0)\0(\0q\0)\0(\0p\0)\0(\0o\0)\0(\0n\0)\0(\0m\0)\0(\0l\0)\0(\0k\0)\0(\0j\0)\0(\0i\0)\0(\0h\0)\0(\0g\0)\0(\0f\0)\0(\0e\0)\0(\0d\0)\0(\0c\0)\0(\0b\0)\0(\0a\0)\x002\x000\0.\x001\09\0.\x001\08\0.\x001\x007\0.\x001\x006\0.\x001\x005\0.\x001\x004\0.\x001\x003\0.\x001\x002\0.\x001\x001\0.\x001\x000\0.\0(\x002\x000\0)\0(\x001\09\0)\0(\x001\08\0)\0(\x001\x007\0)\0(\x001\x006\0)\0(\x001\x005\0)\0(\x001\x004\0)\0(\x001\x003\0)\0(\x001\x002\0)\0(\x001\x001\0)\0(\x001\x000\0)\0(\09\0)\0(\08\0)\0(\x007\0)\0(\x006\0)\0(\x005\0)\0(\x004\0)\0(\x003\0)\0(\x002\0)\0(\x001\0)\0.\".\".\"0\0D 3\0x\0i\0i\0i\0x\0v\0i\0i\0i\0i\0v\0X\0I\0I\0I\0X\0V\0I\0I\0I\0I\0V\x001\0D 7\0D 8\x005\0D 8\x003\0D 8\x001\0D 8\x005\0D 6\x001\0D 6\x004\0D 5\x003\0D 5\x002\0D 5\x001\0D 5\x002\0D 3\x001\0D 3\x001\0D 1\x000\x001\0D 9\0F\0A\0X\0T\0M\0T\0E\0L\0S\0M\0N\0o\0\xB0\0F\0c\0/\0u\0c\0/\0o\0\xB0\0C\0a\0/\0s\0a\0/\0c\0R\0s\x002 2 2 2 !\0?\0?\0!\0!\0!\x005 5 5  \0\x08\x03\x01\x03 \0\x08\x03\0\x03 \0\x14\x03B\x03 \0\x14\x03\x01\x03 \0\x14\x03\0\x03 \0\x13\x03B\x03 \0\x13\x03\x01\x03 \0\x13\x03\0\x03 \0\x08\x03B\x03a\0\xBE\x02\xB3\x0Fq\x0F\x80\x0F\xB2\x0Fq\x0F\x80\x0F\xAB\x0E\xA1\x0E\xAB\x0E\x99\x0E\xCD\x0E\xB2\x0EM\x0E2\x0EJ\x06t\x06H\x06t\x06'\x06t\x06e\x05\x82\x05d\0z\0D\0z\0D\0Z\0n\0j\0N\0j\0N\0J\0l\0j\0L\0j\0L\0J\0d\0z\0\x0C\x03D\0z\0\x0C\x03D\0Z\0\x0C\x03\xBC\x02n\0l\0\xB7\0L\0\xB7\0i\0j\0I\0J\x003\0D 4\x001\0D 2\x001\0D 4\0d\0j\0m\0r\0m\0d\0m\0c\0w\0c\0p\0p\0v\0s\0s\0s\0d\0m\0v\0h\0v\0w\0z\0\x140s\0\x150a\0\x15\"m\0v\0\x15\"m\0w\0b\0s\0v\0p\0r\0p\0p\0m\0p\0h\0k\0k\0h\0p\0g\0y\0d\0b\0c\0\x15\"k\0g\0b\0q\0m\0\xC9\x03k\0\xC9\x03k\0w\0\xBC\x03w\0n\0w\0p\0w\0k\0v\0\xBC\x03v\0n\0v\0g\0p\0a\0m\0p\0a\0k\0p\0a\0t\0h\0z\0g\0h\0z\0m\0h\0z\0k\0h\0z\0\xBC\x03f\0n\0f\0p\0f\0k\0b\0k\0a\0m\0a\0\xBC\x03a\0n\0a\0i\0u\0o\0v\0a\0u\0h\0p\0a\0l\0t\0d\0e\0v\0h\0g\0p\0t\0e\0f\0a\0x\0t\0m\0t\0e\0l\0s\0m\0n\0o\0\xB0\0f\0\xB0\0c\0r\0s\0\xC9\x03\xB9\x03\xC9\x03B\x03\xB9\x03\xC9\x03\x01\x03\xB9\x03\xC9\x03\0\x03\xB9\x03\xB7\x03\xB9\x03\xB7\x03B\x03\xB9\x03\xB7\x03\x01\x03\xB9\x03\xB7\x03\0\x03\xB9\x03\xB1\x03\xB9\x03\xB1\x03B\x03\xB9\x03\xB1\x03\x01\x03\xB9\x03\xB1\x03\0\x03\xB9\x03\xC9\x03\x14\x03B\x03\xB9\x03\xC9\x03\x13\x03B\x03\xB9\x03\xC9\x03\x14\x03\x01\x03\xB9\x03\xC9\x03\x13\x03\x01\x03\xB9\x03\xC9\x03\x14\x03\0\x03\xB9\x03\xC9\x03\x13\x03\0\x03\xB9\x03\xC9\x03\x14\x03\xB9\x03\xC9\x03\x13\x03\xB9\x03\xB7\x03\x14\x03B\x03\xB9\x03\xB7\x03\x13\x03B\x03\xB9\x03\xB7\x03\x14\x03\x01\x03\xB9\x03\xB7\x03\x13\x03\x01\x03\xB9\x03\xB7\x03\x14\x03\0\x03\xB9\x03\xB7\x03\x13\x03\0\x03\xB9\x03\xB7\x03\x14\x03\xB9\x03\xB7\x03\x13\x03\xB9\x03\xB1\x03\x14\x03B\x03\xB9\x03\xB1\x03\x13\x03B\x03\xB9\x03\xB1\x03\x14\x03\x01\x03\xB9\x03\xB1\x03\x13\x03\x01\x03\xB9\x03\xB1\x03\x14\x03\0\x03\xB9\x03\xB1\x03\x13\x03\0\x03\xB9\x03\xB1\x03\x14\x03\xB9\x03\xB1\x03\x13\x03\xB9\x03 \0\xB9\x03") }, scalars24: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x1E\xDF\x01\n\xDF\x01\x08\xDF\x01\x06\xDF\x01\x05\xDF\x01\x04\xDF\x01C\xE9\x01B\xE9\x01A\xE9\x01@\xE9\x01?\xE9\x01>\xE9\x01=\xE9\x01<\xE9\x01;\xE9\x01:\xE9\x019\xE9\x018\xE9\x017\xE9\x016\xE9\x015\xE9\x014\xE9\x013\xE9\x012\xE9\x011\xE9\x010\xE9\x01/\xE9\x01.\xE9\x01-\xE9\x01,\xE9\x01+\xE9\x01*\xE9\x01)\xE9\x01(\xE9\x01'\xE9\x01&\xE9\x01%\xE9\x01$\xE9\x01#\xE9\x01\"\xE9\x01\x7Fn\x01~n\x01}n\x01|n\x01{n\x01zn\x01yn\x01xn\x01wn\x01vn\x01un\x01tn\x01sn\x01rn\x01qn\x01pn\x01on\x01nn\x01mn\x01ln\x01kn\x01jn\x01in\x01hn\x01gn\x01fn\x01en\x01dn\x01cn\x01bn\x01an\x01`n\x01\xDF\x18\x01\xDE\x18\x01\xDD\x18\x01\xDC\x18\x01\xDB\x18\x01\xDA\x18\x01\xD9\x18\x01\xD8\x18\x01\xD7\x18\x01\xD6\x18\x01\xD5\x18\x01\xD4\x18\x01\xD3\x18\x01\xD2\x18\x01\xD1\x18\x01\xD0\x18\x01\xCF\x18\x01\xCE\x18\x01\xCD\x18\x01\xCC\x18\x01\xCB\x18\x01\xCA\x18\x01\xC9\x18\x01\xC8\x18\x01\xC7\x18\x01\xC6\x18\x01\xC5\x18\x01\xC4\x18\x01\xC3\x18\x01\xC2\x18\x01\xC1\x18\x01\xC0\x18\x01\xF2\x0C\x01\xF1\x0C\x01\xF0\x0C\x01\xEF\x0C\x01\xEE\x0C\x01\xED\x0C\x01\xEC\x0C\x01\xEB\x0C\x01\xEA\x0C\x01\xE9\x0C\x01\xE8\x0C\x01\xE7\x0C\x01\xE6\x0C\x01\xE5\x0C\x01\xE4\x0C\x01\xE3\x0C\x01\xE2\x0C\x01\xE1\x0C\x01\xE0\x0C\x01\xDF\x0C\x01\xDE\x0C\x01\xDD\x0C\x01\xDC\x0C\x01\xDB\x0C\x01\xDA\x0C\x01\xD9\x0C\x01\xD8\x0C\x01\xD7\x0C\x01\xD6\x0C\x01\xD5\x0C\x01\xD4\x0C\x01\xD3\x0C\x01\xD2\x0C\x01\xD1\x0C\x01\xD0\x0C\x01\xCF\x0C\x01\xCE\x0C\x01\xCD\x0C\x01\xCC\x0C\x01\xCB\x0C\x01\xCA\x0C\x01\xC9\x0C\x01\xC8\x0C\x01\xC7\x0C\x01\xC6\x0C\x01\xC5\x0C\x01\xC4\x0C\x01\xC3\x0C\x01\xC2\x0C\x01\xC1\x0C\x01\xC0\x0C\x01\xBC\x05\x01\xBB\x05\x01\xB9\x05\x01\xB8\x05\x01\xB7\x05\x01\xB6\x05\x01\xB5\x05\x01\xB4\x05\x01\xB3\x05\x01\xB1\x05\x01\xB0\x05\x01\xAF\x05\x01\xAE\x05\x01\xAD\x05\x01\xAC\x05\x01\xAB\x05\x01\xAA\x05\x01\xA9\x05\x01\xA8\x05\x01\xA7\x05\x01\xA6\x05\x01\xA5\x05\x01\xA4\x05\x01\xA3\x05\x01\xA1\x05\x01\xA0\x05\x01\x9F\x05\x01\x9E\x05\x01\x9D\x05\x01\x9C\x05\x01\x9B\x05\x01\x9A\x05\x01\x99\x05\x01\x98\x05\x01\x97\x05\x01\xFB\x04\x01\xFA\x04\x01\xF9\x04\x01\xF8\x04\x01\xF7\x04\x01\xF6\x04\x01\xF5\x04\x01\xF4\x04\x01\xF3\x04\x01\xF2\x04\x01\xF1\x04\x01\xF0\x04\x01\xEF\x04\x01\xEE\x04\x01\xED\x04\x01\xEC\x04\x01\xEB\x04\x01\xEA\x04\x01\xE9\x04\x01\xE8\x04\x01\xE7\x04\x01\xE6\x04\x01\xE5\x04\x01\xE4\x04\x01\xE3\x04\x01\xE2\x04\x01\xE1\x04\x01\xE0\x04\x01\xDF\x04\x01\xDE\x04\x01\xDD\x04\x01\xDC\x04\x01\xDB\x04\x01\xDA\x04\x01\xD9\x04\x01\xD8\x04\x01O\x04\x01N\x04\x01M\x04\x01L\x04\x01K\x04\x01J\x04\x01I\x04\x01H\x04\x01G\x04\x01F\x04\x01E\x04\x01D\x04\x01C\x04\x01B\x04\x01A\x04\x01@\x04\x01?\x04\x01>\x04\x01=\x04\x01<\x04\x01;\x04\x01:\x04\x019\x04\x018\x04\x017\x04\x016\x04\x015\x04\x014\x04\x013\x04\x012\x04\x011\x04\x010\x04\x01/\x04\x01.\x04\x01-\x04\x01,\x04\x01+\x04\x01*\x04\x01)\x04\x01(\x04\x01") } };
        }
        #[clippy::msrv = "1.67"]
        impl icu_provider::DataProvider<icu::normalizer::provider::CompatibilityDecompositionTablesV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu::normalizer::provider::CompatibilityDecompositionTablesV1Marker>, icu_provider::DataError> {
                if req.locale.is_empty() {
                    Ok(icu_provider::DataResponse { payload: Some(icu_provider::DataPayload::from_static_ref(Self::SINGLETON_NORMALIZER_NFKDEX_V1)), metadata: Default::default() })
                } else {
                    Err(icu_provider::DataErrorKind::ExtraneousLocale.with_req(<icu::normalizer::provider::CompatibilityDecompositionTablesV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
                }
            }
        }
    };
}
