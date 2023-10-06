// @generated
/// Implement `DataProvider<UppercaseV1Marker>` on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_props_upper_v1 {
    ($ provider : ty) => {
        #[clippy::msrv = "1.67"]
        const _: () = <$provider>::MUST_USE_MAKE_PROVIDER_MACRO;
        #[clippy::msrv = "1.67"]
        impl $provider {
            #[doc(hidden)]
            pub const SINGLETON_PROPS_UPPER_V1: &'static <icu::properties::provider::UppercaseV1Marker as icu_provider::DataMarker>::Yokeable = &icu::properties::provider::PropertyCodePointSetV1::InversionList(unsafe {
                #[allow(unused_unsafe)]
                icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"A\0\0\0[\0\0\0\xC0\0\0\0\xD7\0\0\0\xD8\0\0\0\xDF\0\0\0\0\x01\0\0\x01\x01\0\0\x02\x01\0\0\x03\x01\0\0\x04\x01\0\0\x05\x01\0\0\x06\x01\0\0\x07\x01\0\0\x08\x01\0\0\t\x01\0\0\n\x01\0\0\x0B\x01\0\0\x0C\x01\0\0\r\x01\0\0\x0E\x01\0\0\x0F\x01\0\0\x10\x01\0\0\x11\x01\0\0\x12\x01\0\0\x13\x01\0\0\x14\x01\0\0\x15\x01\0\0\x16\x01\0\0\x17\x01\0\0\x18\x01\0\0\x19\x01\0\0\x1A\x01\0\0\x1B\x01\0\0\x1C\x01\0\0\x1D\x01\0\0\x1E\x01\0\0\x1F\x01\0\0 \x01\0\0!\x01\0\0\"\x01\0\0#\x01\0\0$\x01\0\0%\x01\0\0&\x01\0\0'\x01\0\0(\x01\0\0)\x01\0\0*\x01\0\0+\x01\0\0,\x01\0\0-\x01\0\0.\x01\0\0/\x01\0\x000\x01\0\x001\x01\0\x002\x01\0\x003\x01\0\x004\x01\0\x005\x01\0\x006\x01\0\x007\x01\0\09\x01\0\0:\x01\0\0;\x01\0\0<\x01\0\0=\x01\0\0>\x01\0\0?\x01\0\0@\x01\0\0A\x01\0\0B\x01\0\0C\x01\0\0D\x01\0\0E\x01\0\0F\x01\0\0G\x01\0\0H\x01\0\0J\x01\0\0K\x01\0\0L\x01\0\0M\x01\0\0N\x01\0\0O\x01\0\0P\x01\0\0Q\x01\0\0R\x01\0\0S\x01\0\0T\x01\0\0U\x01\0\0V\x01\0\0W\x01\0\0X\x01\0\0Y\x01\0\0Z\x01\0\0[\x01\0\0\\\x01\0\0]\x01\0\0^\x01\0\0_\x01\0\0`\x01\0\0a\x01\0\0b\x01\0\0c\x01\0\0d\x01\0\0e\x01\0\0f\x01\0\0g\x01\0\0h\x01\0\0i\x01\0\0j\x01\0\0k\x01\0\0l\x01\0\0m\x01\0\0n\x01\0\0o\x01\0\0p\x01\0\0q\x01\0\0r\x01\0\0s\x01\0\0t\x01\0\0u\x01\0\0v\x01\0\0w\x01\0\0x\x01\0\0z\x01\0\0{\x01\0\0|\x01\0\0}\x01\0\0~\x01\0\0\x81\x01\0\0\x83\x01\0\0\x84\x01\0\0\x85\x01\0\0\x86\x01\0\0\x88\x01\0\0\x89\x01\0\0\x8C\x01\0\0\x8E\x01\0\0\x92\x01\0\0\x93\x01\0\0\x95\x01\0\0\x96\x01\0\0\x99\x01\0\0\x9C\x01\0\0\x9E\x01\0\0\x9F\x01\0\0\xA1\x01\0\0\xA2\x01\0\0\xA3\x01\0\0\xA4\x01\0\0\xA5\x01\0\0\xA6\x01\0\0\xA8\x01\0\0\xA9\x01\0\0\xAA\x01\0\0\xAC\x01\0\0\xAD\x01\0\0\xAE\x01\0\0\xB0\x01\0\0\xB1\x01\0\0\xB4\x01\0\0\xB5\x01\0\0\xB6\x01\0\0\xB7\x01\0\0\xB9\x01\0\0\xBC\x01\0\0\xBD\x01\0\0\xC4\x01\0\0\xC5\x01\0\0\xC7\x01\0\0\xC8\x01\0\0\xCA\x01\0\0\xCB\x01\0\0\xCD\x01\0\0\xCE\x01\0\0\xCF\x01\0\0\xD0\x01\0\0\xD1\x01\0\0\xD2\x01\0\0\xD3\x01\0\0\xD4\x01\0\0\xD5\x01\0\0\xD6\x01\0\0\xD7\x01\0\0\xD8\x01\0\0\xD9\x01\0\0\xDA\x01\0\0\xDB\x01\0\0\xDC\x01\0\0\xDE\x01\0\0\xDF\x01\0\0\xE0\x01\0\0\xE1\x01\0\0\xE2\x01\0\0\xE3\x01\0\0\xE4\x01\0\0\xE5\x01\0\0\xE6\x01\0\0\xE7\x01\0\0\xE8\x01\0\0\xE9\x01\0\0\xEA\x01\0\0\xEB\x01\0\0\xEC\x01\0\0\xED\x01\0\0\xEE\x01\0\0\xEF\x01\0\0\xF1\x01\0\0\xF2\x01\0\0\xF4\x01\0\0\xF5\x01\0\0\xF6\x01\0\0\xF9\x01\0\0\xFA\x01\0\0\xFB\x01\0\0\xFC\x01\0\0\xFD\x01\0\0\xFE\x01\0\0\xFF\x01\0\0\0\x02\0\0\x01\x02\0\0\x02\x02\0\0\x03\x02\0\0\x04\x02\0\0\x05\x02\0\0\x06\x02\0\0\x07\x02\0\0\x08\x02\0\0\t\x02\0\0\n\x02\0\0\x0B\x02\0\0\x0C\x02\0\0\r\x02\0\0\x0E\x02\0\0\x0F\x02\0\0\x10\x02\0\0\x11\x02\0\0\x12\x02\0\0\x13\x02\0\0\x14\x02\0\0\x15\x02\0\0\x16\x02\0\0\x17\x02\0\0\x18\x02\0\0\x19\x02\0\0\x1A\x02\0\0\x1B\x02\0\0\x1C\x02\0\0\x1D\x02\0\0\x1E\x02\0\0\x1F\x02\0\0 \x02\0\0!\x02\0\0\"\x02\0\0#\x02\0\0$\x02\0\0%\x02\0\0&\x02\0\0'\x02\0\0(\x02\0\0)\x02\0\0*\x02\0\0+\x02\0\0,\x02\0\0-\x02\0\0.\x02\0\0/\x02\0\x000\x02\0\x001\x02\0\x002\x02\0\x003\x02\0\0:\x02\0\0<\x02\0\0=\x02\0\0?\x02\0\0A\x02\0\0B\x02\0\0C\x02\0\0G\x02\0\0H\x02\0\0I\x02\0\0J\x02\0\0K\x02\0\0L\x02\0\0M\x02\0\0N\x02\0\0O\x02\0\0p\x03\0\0q\x03\0\0r\x03\0\0s\x03\0\0v\x03\0\0w\x03\0\0\x7F\x03\0\0\x80\x03\0\0\x86\x03\0\0\x87\x03\0\0\x88\x03\0\0\x8B\x03\0\0\x8C\x03\0\0\x8D\x03\0\0\x8E\x03\0\0\x90\x03\0\0\x91\x03\0\0\xA2\x03\0\0\xA3\x03\0\0\xAC\x03\0\0\xCF\x03\0\0\xD0\x03\0\0\xD2\x03\0\0\xD5\x03\0\0\xD8\x03\0\0\xD9\x03\0\0\xDA\x03\0\0\xDB\x03\0\0\xDC\x03\0\0\xDD\x03\0\0\xDE\x03\0\0\xDF\x03\0\0\xE0\x03\0\0\xE1\x03\0\0\xE2\x03\0\0\xE3\x03\0\0\xE4\x03\0\0\xE5\x03\0\0\xE6\x03\0\0\xE7\x03\0\0\xE8\x03\0\0\xE9\x03\0\0\xEA\x03\0\0\xEB\x03\0\0\xEC\x03\0\0\xED\x03\0\0\xEE\x03\0\0\xEF\x03\0\0\xF4\x03\0\0\xF5\x03\0\0\xF7\x03\0\0\xF8\x03\0\0\xF9\x03\0\0\xFB\x03\0\0\xFD\x03\0\x000\x04\0\0`\x04\0\0a\x04\0\0b\x04\0\0c\x04\0\0d\x04\0\0e\x04\0\0f\x04\0\0g\x04\0\0h\x04\0\0i\x04\0\0j\x04\0\0k\x04\0\0l\x04\0\0m\x04\0\0n\x04\0\0o\x04\0\0p\x04\0\0q\x04\0\0r\x04\0\0s\x04\0\0t\x04\0\0u\x04\0\0v\x04\0\0w\x04\0\0x\x04\0\0y\x04\0\0z\x04\0\0{\x04\0\0|\x04\0\0}\x04\0\0~\x04\0\0\x7F\x04\0\0\x80\x04\0\0\x81\x04\0\0\x8A\x04\0\0\x8B\x04\0\0\x8C\x04\0\0\x8D\x04\0\0\x8E\x04\0\0\x8F\x04\0\0\x90\x04\0\0\x91\x04\0\0\x92\x04\0\0\x93\x04\0\0\x94\x04\0\0\x95\x04\0\0\x96\x04\0\0\x97\x04\0\0\x98\x04\0\0\x99\x04\0\0\x9A\x04\0\0\x9B\x04\0\0\x9C\x04\0\0\x9D\x04\0\0\x9E\x04\0\0\x9F\x04\0\0\xA0\x04\0\0\xA1\x04\0\0\xA2\x04\0\0\xA3\x04\0\0\xA4\x04\0\0\xA5\x04\0\0\xA6\x04\0\0\xA7\x04\0\0\xA8\x04\0\0\xA9\x04\0\0\xAA\x04\0\0\xAB\x04\0\0\xAC\x04\0\0\xAD\x04\0\0\xAE\x04\0\0\xAF\x04\0\0\xB0\x04\0\0\xB1\x04\0\0\xB2\x04\0\0\xB3\x04\0\0\xB4\x04\0\0\xB5\x04\0\0\xB6\x04\0\0\xB7\x04\0\0\xB8\x04\0\0\xB9\x04\0\0\xBA\x04\0\0\xBB\x04\0\0\xBC\x04\0\0\xBD\x04\0\0\xBE\x04\0\0\xBF\x04\0\0\xC0\x04\0\0\xC2\x04\0\0\xC3\x04\0\0\xC4\x04\0\0\xC5\x04\0\0\xC6\x04\0\0\xC7\x04\0\0\xC8\x04\0\0\xC9\x04\0\0\xCA\x04\0\0\xCB\x04\0\0\xCC\x04\0\0\xCD\x04\0\0\xCE\x04\0\0\xD0\x04\0\0\xD1\x04\0\0\xD2\x04\0\0\xD3\x04\0\0\xD4\x04\0\0\xD5\x04\0\0\xD6\x04\0\0\xD7\x04\0\0\xD8\x04\0\0\xD9\x04\0\0\xDA\x04\0\0\xDB\x04\0\0\xDC\x04\0\0\xDD\x04\0\0\xDE\x04\0\0\xDF\x04\0\0\xE0\x04\0\0\xE1\x04\0\0\xE2\x04\0\0\xE3\x04\0\0\xE4\x04\0\0\xE5\x04\0\0\xE6\x04\0\0\xE7\x04\0\0\xE8\x04\0\0\xE9\x04\0\0\xEA\x04\0\0\xEB\x04\0\0\xEC\x04\0\0\xED\x04\0\0\xEE\x04\0\0\xEF\x04\0\0\xF0\x04\0\0\xF1\x04\0\0\xF2\x04\0\0\xF3\x04\0\0\xF4\x04\0\0\xF5\x04\0\0\xF6\x04\0\0\xF7\x04\0\0\xF8\x04\0\0\xF9\x04\0\0\xFA\x04\0\0\xFB\x04\0\0\xFC\x04\0\0\xFD\x04\0\0\xFE\x04\0\0\xFF\x04\0\0\0\x05\0\0\x01\x05\0\0\x02\x05\0\0\x03\x05\0\0\x04\x05\0\0\x05\x05\0\0\x06\x05\0\0\x07\x05\0\0\x08\x05\0\0\t\x05\0\0\n\x05\0\0\x0B\x05\0\0\x0C\x05\0\0\r\x05\0\0\x0E\x05\0\0\x0F\x05\0\0\x10\x05\0\0\x11\x05\0\0\x12\x05\0\0\x13\x05\0\0\x14\x05\0\0\x15\x05\0\0\x16\x05\0\0\x17\x05\0\0\x18\x05\0\0\x19\x05\0\0\x1A\x05\0\0\x1B\x05\0\0\x1C\x05\0\0\x1D\x05\0\0\x1E\x05\0\0\x1F\x05\0\0 \x05\0\0!\x05\0\0\"\x05\0\0#\x05\0\0$\x05\0\0%\x05\0\0&\x05\0\0'\x05\0\0(\x05\0\0)\x05\0\0*\x05\0\0+\x05\0\0,\x05\0\0-\x05\0\0.\x05\0\0/\x05\0\x001\x05\0\0W\x05\0\0\xA0\x10\0\0\xC6\x10\0\0\xC7\x10\0\0\xC8\x10\0\0\xCD\x10\0\0\xCE\x10\0\0\xA0\x13\0\0\xF6\x13\0\0\x90\x1C\0\0\xBB\x1C\0\0\xBD\x1C\0\0\xC0\x1C\0\0\0\x1E\0\0\x01\x1E\0\0\x02\x1E\0\0\x03\x1E\0\0\x04\x1E\0\0\x05\x1E\0\0\x06\x1E\0\0\x07\x1E\0\0\x08\x1E\0\0\t\x1E\0\0\n\x1E\0\0\x0B\x1E\0\0\x0C\x1E\0\0\r\x1E\0\0\x0E\x1E\0\0\x0F\x1E\0\0\x10\x1E\0\0\x11\x1E\0\0\x12\x1E\0\0\x13\x1E\0\0\x14\x1E\0\0\x15\x1E\0\0\x16\x1E\0\0\x17\x1E\0\0\x18\x1E\0\0\x19\x1E\0\0\x1A\x1E\0\0\x1B\x1E\0\0\x1C\x1E\0\0\x1D\x1E\0\0\x1E\x1E\0\0\x1F\x1E\0\0 \x1E\0\0!\x1E\0\0\"\x1E\0\0#\x1E\0\0$\x1E\0\0%\x1E\0\0&\x1E\0\0'\x1E\0\0(\x1E\0\0)\x1E\0\0*\x1E\0\0+\x1E\0\0,\x1E\0\0-\x1E\0\0.\x1E\0\0/\x1E\0\x000\x1E\0\x001\x1E\0\x002\x1E\0\x003\x1E\0\x004\x1E\0\x005\x1E\0\x006\x1E\0\x007\x1E\0\08\x1E\0\09\x1E\0\0:\x1E\0\0;\x1E\0\0<\x1E\0\0=\x1E\0\0>\x1E\0\0?\x1E\0\0@\x1E\0\0A\x1E\0\0B\x1E\0\0C\x1E\0\0D\x1E\0\0E\x1E\0\0F\x1E\0\0G\x1E\0\0H\x1E\0\0I\x1E\0\0J\x1E\0\0K\x1E\0\0L\x1E\0\0M\x1E\0\0N\x1E\0\0O\x1E\0\0P\x1E\0\0Q\x1E\0\0R\x1E\0\0S\x1E\0\0T\x1E\0\0U\x1E\0\0V\x1E\0\0W\x1E\0\0X\x1E\0\0Y\x1E\0\0Z\x1E\0\0[\x1E\0\0\\\x1E\0\0]\x1E\0\0^\x1E\0\0_\x1E\0\0`\x1E\0\0a\x1E\0\0b\x1E\0\0c\x1E\0\0d\x1E\0\0e\x1E\0\0f\x1E\0\0g\x1E\0\0h\x1E\0\0i\x1E\0\0j\x1E\0\0k\x1E\0\0l\x1E\0\0m\x1E\0\0n\x1E\0\0o\x1E\0\0p\x1E\0\0q\x1E\0\0r\x1E\0\0s\x1E\0\0t\x1E\0\0u\x1E\0\0v\x1E\0\0w\x1E\0\0x\x1E\0\0y\x1E\0\0z\x1E\0\0{\x1E\0\0|\x1E\0\0}\x1E\0\0~\x1E\0\0\x7F\x1E\0\0\x80\x1E\0\0\x81\x1E\0\0\x82\x1E\0\0\x83\x1E\0\0\x84\x1E\0\0\x85\x1E\0\0\x86\x1E\0\0\x87\x1E\0\0\x88\x1E\0\0\x89\x1E\0\0\x8A\x1E\0\0\x8B\x1E\0\0\x8C\x1E\0\0\x8D\x1E\0\0\x8E\x1E\0\0\x8F\x1E\0\0\x90\x1E\0\0\x91\x1E\0\0\x92\x1E\0\0\x93\x1E\0\0\x94\x1E\0\0\x95\x1E\0\0\x9E\x1E\0\0\x9F\x1E\0\0\xA0\x1E\0\0\xA1\x1E\0\0\xA2\x1E\0\0\xA3\x1E\0\0\xA4\x1E\0\0\xA5\x1E\0\0\xA6\x1E\0\0\xA7\x1E\0\0\xA8\x1E\0\0\xA9\x1E\0\0\xAA\x1E\0\0\xAB\x1E\0\0\xAC\x1E\0\0\xAD\x1E\0\0\xAE\x1E\0\0\xAF\x1E\0\0\xB0\x1E\0\0\xB1\x1E\0\0\xB2\x1E\0\0\xB3\x1E\0\0\xB4\x1E\0\0\xB5\x1E\0\0\xB6\x1E\0\0\xB7\x1E\0\0\xB8\x1E\0\0\xB9\x1E\0\0\xBA\x1E\0\0\xBB\x1E\0\0\xBC\x1E\0\0\xBD\x1E\0\0\xBE\x1E\0\0\xBF\x1E\0\0\xC0\x1E\0\0\xC1\x1E\0\0\xC2\x1E\0\0\xC3\x1E\0\0\xC4\x1E\0\0\xC5\x1E\0\0\xC6\x1E\0\0\xC7\x1E\0\0\xC8\x1E\0\0\xC9\x1E\0\0\xCA\x1E\0\0\xCB\x1E\0\0\xCC\x1E\0\0\xCD\x1E\0\0\xCE\x1E\0\0\xCF\x1E\0\0\xD0\x1E\0\0\xD1\x1E\0\0\xD2\x1E\0\0\xD3\x1E\0\0\xD4\x1E\0\0\xD5\x1E\0\0\xD6\x1E\0\0\xD7\x1E\0\0\xD8\x1E\0\0\xD9\x1E\0\0\xDA\x1E\0\0\xDB\x1E\0\0\xDC\x1E\0\0\xDD\x1E\0\0\xDE\x1E\0\0\xDF\x1E\0\0\xE0\x1E\0\0\xE1\x1E\0\0\xE2\x1E\0\0\xE3\x1E\0\0\xE4\x1E\0\0\xE5\x1E\0\0\xE6\x1E\0\0\xE7\x1E\0\0\xE8\x1E\0\0\xE9\x1E\0\0\xEA\x1E\0\0\xEB\x1E\0\0\xEC\x1E\0\0\xED\x1E\0\0\xEE\x1E\0\0\xEF\x1E\0\0\xF0\x1E\0\0\xF1\x1E\0\0\xF2\x1E\0\0\xF3\x1E\0\0\xF4\x1E\0\0\xF5\x1E\0\0\xF6\x1E\0\0\xF7\x1E\0\0\xF8\x1E\0\0\xF9\x1E\0\0\xFA\x1E\0\0\xFB\x1E\0\0\xFC\x1E\0\0\xFD\x1E\0\0\xFE\x1E\0\0\xFF\x1E\0\0\x08\x1F\0\0\x10\x1F\0\0\x18\x1F\0\0\x1E\x1F\0\0(\x1F\0\x000\x1F\0\08\x1F\0\0@\x1F\0\0H\x1F\0\0N\x1F\0\0Y\x1F\0\0Z\x1F\0\0[\x1F\0\0\\\x1F\0\0]\x1F\0\0^\x1F\0\0_\x1F\0\0`\x1F\0\0h\x1F\0\0p\x1F\0\0\xB8\x1F\0\0\xBC\x1F\0\0\xC8\x1F\0\0\xCC\x1F\0\0\xD8\x1F\0\0\xDC\x1F\0\0\xE8\x1F\0\0\xED\x1F\0\0\xF8\x1F\0\0\xFC\x1F\0\0\x02!\0\0\x03!\0\0\x07!\0\0\x08!\0\0\x0B!\0\0\x0E!\0\0\x10!\0\0\x13!\0\0\x15!\0\0\x16!\0\0\x19!\0\0\x1E!\0\0$!\0\0%!\0\0&!\0\0'!\0\0(!\0\0)!\0\0*!\0\0.!\0\x000!\0\x004!\0\0>!\0\0@!\0\0E!\0\0F!\0\0`!\0\0p!\0\0\x83!\0\0\x84!\0\0\xB6$\0\0\xD0$\0\0\0,\0\x000,\0\0`,\0\0a,\0\0b,\0\0e,\0\0g,\0\0h,\0\0i,\0\0j,\0\0k,\0\0l,\0\0m,\0\0q,\0\0r,\0\0s,\0\0u,\0\0v,\0\0~,\0\0\x81,\0\0\x82,\0\0\x83,\0\0\x84,\0\0\x85,\0\0\x86,\0\0\x87,\0\0\x88,\0\0\x89,\0\0\x8A,\0\0\x8B,\0\0\x8C,\0\0\x8D,\0\0\x8E,\0\0\x8F,\0\0\x90,\0\0\x91,\0\0\x92,\0\0\x93,\0\0\x94,\0\0\x95,\0\0\x96,\0\0\x97,\0\0\x98,\0\0\x99,\0\0\x9A,\0\0\x9B,\0\0\x9C,\0\0\x9D,\0\0\x9E,\0\0\x9F,\0\0\xA0,\0\0\xA1,\0\0\xA2,\0\0\xA3,\0\0\xA4,\0\0\xA5,\0\0\xA6,\0\0\xA7,\0\0\xA8,\0\0\xA9,\0\0\xAA,\0\0\xAB,\0\0\xAC,\0\0\xAD,\0\0\xAE,\0\0\xAF,\0\0\xB0,\0\0\xB1,\0\0\xB2,\0\0\xB3,\0\0\xB4,\0\0\xB5,\0\0\xB6,\0\0\xB7,\0\0\xB8,\0\0\xB9,\0\0\xBA,\0\0\xBB,\0\0\xBC,\0\0\xBD,\0\0\xBE,\0\0\xBF,\0\0\xC0,\0\0\xC1,\0\0\xC2,\0\0\xC3,\0\0\xC4,\0\0\xC5,\0\0\xC6,\0\0\xC7,\0\0\xC8,\0\0\xC9,\0\0\xCA,\0\0\xCB,\0\0\xCC,\0\0\xCD,\0\0\xCE,\0\0\xCF,\0\0\xD0,\0\0\xD1,\0\0\xD2,\0\0\xD3,\0\0\xD4,\0\0\xD5,\0\0\xD6,\0\0\xD7,\0\0\xD8,\0\0\xD9,\0\0\xDA,\0\0\xDB,\0\0\xDC,\0\0\xDD,\0\0\xDE,\0\0\xDF,\0\0\xE0,\0\0\xE1,\0\0\xE2,\0\0\xE3,\0\0\xEB,\0\0\xEC,\0\0\xED,\0\0\xEE,\0\0\xF2,\0\0\xF3,\0\0@\xA6\0\0A\xA6\0\0B\xA6\0\0C\xA6\0\0D\xA6\0\0E\xA6\0\0F\xA6\0\0G\xA6\0\0H\xA6\0\0I\xA6\0\0J\xA6\0\0K\xA6\0\0L\xA6\0\0M\xA6\0\0N\xA6\0\0O\xA6\0\0P\xA6\0\0Q\xA6\0\0R\xA6\0\0S\xA6\0\0T\xA6\0\0U\xA6\0\0V\xA6\0\0W\xA6\0\0X\xA6\0\0Y\xA6\0\0Z\xA6\0\0[\xA6\0\0\\\xA6\0\0]\xA6\0\0^\xA6\0\0_\xA6\0\0`\xA6\0\0a\xA6\0\0b\xA6\0\0c\xA6\0\0d\xA6\0\0e\xA6\0\0f\xA6\0\0g\xA6\0\0h\xA6\0\0i\xA6\0\0j\xA6\0\0k\xA6\0\0l\xA6\0\0m\xA6\0\0\x80\xA6\0\0\x81\xA6\0\0\x82\xA6\0\0\x83\xA6\0\0\x84\xA6\0\0\x85\xA6\0\0\x86\xA6\0\0\x87\xA6\0\0\x88\xA6\0\0\x89\xA6\0\0\x8A\xA6\0\0\x8B\xA6\0\0\x8C\xA6\0\0\x8D\xA6\0\0\x8E\xA6\0\0\x8F\xA6\0\0\x90\xA6\0\0\x91\xA6\0\0\x92\xA6\0\0\x93\xA6\0\0\x94\xA6\0\0\x95\xA6\0\0\x96\xA6\0\0\x97\xA6\0\0\x98\xA6\0\0\x99\xA6\0\0\x9A\xA6\0\0\x9B\xA6\0\0\"\xA7\0\0#\xA7\0\0$\xA7\0\0%\xA7\0\0&\xA7\0\0'\xA7\0\0(\xA7\0\0)\xA7\0\0*\xA7\0\0+\xA7\0\0,\xA7\0\0-\xA7\0\0.\xA7\0\0/\xA7\0\x002\xA7\0\x003\xA7\0\x004\xA7\0\x005\xA7\0\x006\xA7\0\x007\xA7\0\08\xA7\0\09\xA7\0\0:\xA7\0\0;\xA7\0\0<\xA7\0\0=\xA7\0\0>\xA7\0\0?\xA7\0\0@\xA7\0\0A\xA7\0\0B\xA7\0\0C\xA7\0\0D\xA7\0\0E\xA7\0\0F\xA7\0\0G\xA7\0\0H\xA7\0\0I\xA7\0\0J\xA7\0\0K\xA7\0\0L\xA7\0\0M\xA7\0\0N\xA7\0\0O\xA7\0\0P\xA7\0\0Q\xA7\0\0R\xA7\0\0S\xA7\0\0T\xA7\0\0U\xA7\0\0V\xA7\0\0W\xA7\0\0X\xA7\0\0Y\xA7\0\0Z\xA7\0\0[\xA7\0\0\\\xA7\0\0]\xA7\0\0^\xA7\0\0_\xA7\0\0`\xA7\0\0a\xA7\0\0b\xA7\0\0c\xA7\0\0d\xA7\0\0e\xA7\0\0f\xA7\0\0g\xA7\0\0h\xA7\0\0i\xA7\0\0j\xA7\0\0k\xA7\0\0l\xA7\0\0m\xA7\0\0n\xA7\0\0o\xA7\0\0y\xA7\0\0z\xA7\0\0{\xA7\0\0|\xA7\0\0}\xA7\0\0\x7F\xA7\0\0\x80\xA7\0\0\x81\xA7\0\0\x82\xA7\0\0\x83\xA7\0\0\x84\xA7\0\0\x85\xA7\0\0\x86\xA7\0\0\x87\xA7\0\0\x8B\xA7\0\0\x8C\xA7\0\0\x8D\xA7\0\0\x8E\xA7\0\0\x90\xA7\0\0\x91\xA7\0\0\x92\xA7\0\0\x93\xA7\0\0\x96\xA7\0\0\x97\xA7\0\0\x98\xA7\0\0\x99\xA7\0\0\x9A\xA7\0\0\x9B\xA7\0\0\x9C\xA7\0\0\x9D\xA7\0\0\x9E\xA7\0\0\x9F\xA7\0\0\xA0\xA7\0\0\xA1\xA7\0\0\xA2\xA7\0\0\xA3\xA7\0\0\xA4\xA7\0\0\xA5\xA7\0\0\xA6\xA7\0\0\xA7\xA7\0\0\xA8\xA7\0\0\xA9\xA7\0\0\xAA\xA7\0\0\xAF\xA7\0\0\xB0\xA7\0\0\xB5\xA7\0\0\xB6\xA7\0\0\xB7\xA7\0\0\xB8\xA7\0\0\xB9\xA7\0\0\xBA\xA7\0\0\xBB\xA7\0\0\xBC\xA7\0\0\xBD\xA7\0\0\xBE\xA7\0\0\xBF\xA7\0\0\xC0\xA7\0\0\xC1\xA7\0\0\xC2\xA7\0\0\xC3\xA7\0\0\xC4\xA7\0\0\xC8\xA7\0\0\xC9\xA7\0\0\xCA\xA7\0\0\xD0\xA7\0\0\xD1\xA7\0\0\xD6\xA7\0\0\xD7\xA7\0\0\xD8\xA7\0\0\xD9\xA7\0\0\xF5\xA7\0\0\xF6\xA7\0\0!\xFF\0\0;\xFF\0\0\0\x04\x01\0(\x04\x01\0\xB0\x04\x01\0\xD4\x04\x01\0p\x05\x01\0{\x05\x01\0|\x05\x01\0\x8B\x05\x01\0\x8C\x05\x01\0\x93\x05\x01\0\x94\x05\x01\0\x96\x05\x01\0\x80\x0C\x01\0\xB3\x0C\x01\0\xA0\x18\x01\0\xC0\x18\x01\0@n\x01\0`n\x01\0\0\xD4\x01\0\x1A\xD4\x01\x004\xD4\x01\0N\xD4\x01\0h\xD4\x01\0\x82\xD4\x01\0\x9C\xD4\x01\0\x9D\xD4\x01\0\x9E\xD4\x01\0\xA0\xD4\x01\0\xA2\xD4\x01\0\xA3\xD4\x01\0\xA5\xD4\x01\0\xA7\xD4\x01\0\xA9\xD4\x01\0\xAD\xD4\x01\0\xAE\xD4\x01\0\xB6\xD4\x01\0\xD0\xD4\x01\0\xEA\xD4\x01\0\x04\xD5\x01\0\x06\xD5\x01\0\x07\xD5\x01\0\x0B\xD5\x01\0\r\xD5\x01\0\x15\xD5\x01\0\x16\xD5\x01\0\x1D\xD5\x01\08\xD5\x01\0:\xD5\x01\0;\xD5\x01\0?\xD5\x01\0@\xD5\x01\0E\xD5\x01\0F\xD5\x01\0G\xD5\x01\0J\xD5\x01\0Q\xD5\x01\0l\xD5\x01\0\x86\xD5\x01\0\xA0\xD5\x01\0\xBA\xD5\x01\0\xD4\xD5\x01\0\xEE\xD5\x01\0\x08\xD6\x01\0\"\xD6\x01\0<\xD6\x01\0V\xD6\x01\0p\xD6\x01\0\x8A\xD6\x01\0\xA8\xD6\x01\0\xC1\xD6\x01\0\xE2\xD6\x01\0\xFB\xD6\x01\0\x1C\xD7\x01\x005\xD7\x01\0V\xD7\x01\0o\xD7\x01\0\x90\xD7\x01\0\xA9\xD7\x01\0\xCA\xD7\x01\0\xCB\xD7\x01\0\0\xE9\x01\0\"\xE9\x01\x000\xF1\x01\0J\xF1\x01\0P\xF1\x01\0j\xF1\x01\0p\xF1\x01\0\x8A\xF1\x01\0") }, 1951u32)
            });
        }
        #[clippy::msrv = "1.67"]
        impl icu_provider::DataProvider<icu::properties::provider::UppercaseV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu::properties::provider::UppercaseV1Marker>, icu_provider::DataError> {
                if req.locale.is_empty() {
                    Ok(icu_provider::DataResponse { payload: Some(icu_provider::DataPayload::from_static_ref(Self::SINGLETON_PROPS_UPPER_V1)), metadata: Default::default() })
                } else {
                    Err(icu_provider::DataErrorKind::ExtraneousLocale.with_req(<icu::properties::provider::UppercaseV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
                }
            }
        }
    };
}
