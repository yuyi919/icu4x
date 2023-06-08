// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crlify::BufWriterWithLineEndingFix;
use icu_datagen::prelude::*;
use rust_format::*;
use std::fs::File;
use std::io::Write;
use std::path::Path;

include!("../../locales.rs.data");

fn main() {
    #![allow(deprecated)] // want to keep old datagen code path covered

    simple_logger::SimpleLogger::new()
        .env()
        .with_level(log::LevelFilter::Info)
        .init()
        .unwrap();

    let out_dir = Path::new(concat!(
        core::env!("CARGO_MANIFEST_DIR"),
        "/../../provider/testdata/data/"
    ));

    let source = SourceData::offline()
        .with_cldr(repodata::paths::cldr(), Default::default())
        .unwrap()
        .with_icuexport(repodata::paths::icuexport())
        .unwrap()
        .with_segmenter_lstm(repodata::paths::lstm())
        .unwrap();

    let json_out = Out::Fs {
        output_path: repodata::paths::json(),
        serializer: Box::new(syntax::Json::pretty()),
        overwrite: true,
        fingerprint: true,
    };

    let postcard_out = Out::Fs {
        output_path: out_dir.join("postcard"),
        serializer: Box::<syntax::Postcard>::default(),
        overwrite: true,
        fingerprint: true,
    };

    let blob_out = Out::Blob(Box::new(
        File::create(out_dir.join("testdata.postcard")).unwrap(),
    ));

    let mut options = BakedOptions::default();
    options.insert_feature_gates = true;
    options.use_separate_crates = true;
    options.overwrite = true;
    options.pretty = true;
    let mod_out = Out::Baked {
        mod_directory: out_dir.join("baked"),
        options,
    };

    icu_datagen::datagen(
        Some(LOCALES),
        &icu_datagen::all_keys_with_experimental()
            .into_iter()
            .chain([icu_provider::hello_world::HelloWorldV1Marker::KEY])
            .collect::<Vec<_>>(),
        &source,
        vec![json_out, blob_out, mod_out, postcard_out],
    )
    .unwrap();

    let mut metadata =
        BufWriterWithLineEndingFix::new(File::create(out_dir.join("metadata.rs.data")).unwrap());

    metadata
        .write_all(
            "\
             // DO NOT EDIT\n\
             // This file is generated by `make-testdata` from\n\
             // * tools/testdata-scripts/locales.rs.data,\n\
             // * `icu_datagen::SourceData::LATEST_TESTED_*`.\n\
             \n\
            "
            .as_bytes(),
        )
        .unwrap();

    let locales = databake::Bake::bake(LOCALES, &Default::default());
    // repodata corresponds to these tags.
    let cldr_tag = SourceData::LATEST_TESTED_CLDR_TAG;
    let icu_tag = SourceData::LATEST_TESTED_ICUEXPORT_TAG;
    let lstm_tag = SourceData::LATEST_TESTED_SEGMENTER_LSTM_TAG;

    metadata
        .write_all(
            RustFmt::new()
                .format_tokens(quote::quote! {
                    pub const LOCALES: &[icu_locid::LanguageIdentifier] = &#locales;
                    pub const CLDR_TAG: &str = #cldr_tag;
                    pub const ICUEXPORT_TAG: &str = #icu_tag;
                    pub const SEGMENTER_LSTM_TAG: &str = #lstm_tag;
                })
                .unwrap()
                .as_bytes(),
        )
        .unwrap();
}