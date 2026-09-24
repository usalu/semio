
use crate::standards::v1::subsets::any::io::export::serializers::artifacts::json::v_rfc8259::any::json_text;
use crate::standards::v1::subsets::any::io::export::serializers::artifacts::txt::v_utf_8::any::dsl_text;
use crate::standards::v1::subsets::any::io::export::serializers::artifacts::zip::v2_0::any::Block3dIntoZip;
use crate::standards::v1::subsets::any::io::import::deserializers::artifacts::json::v_rfc8259::any::from_json_text;
use crate::standards::v1::subsets::any::io::import::deserializers::artifacts::txt::v_utf_8::any::from_dsl_text;
use crate::standards::v1::subsets::any::io::import::deserializers::artifacts::zip::v2_0::any::{ZIP_MAGIC, from_zip_bytes};
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::IoPayload;

/// 📄️ Every handcrafted `.semio` DSL example asset of this subset — the language-agnostic
/// fixtures the TypeScript mirror's own test reads too.
const EXAMPLES: &[(&str, &str)] = &[
    ("hexagonal-cut-concrete-forest-left", include_str!("../../../📚️examples/🌲️hexagonal-cut-concrete-forest-left/🖼️assets/🌲️hexagonal-cut-concrete-forest-left/🗣️.dsl.semio")),
    ("nakagin-capsule", include_str!("../../../📚️examples/🏢️nakagin-capsule/🖼️assets/🏢️nakagin-capsule/🗣️.dsl.semio")),
];

#[semio_framework_async_macros::async_test]
async fn txt_round_trips_every_example() {
    for (id, text) in EXAMPLES {
        let snapshot = from_dsl_text(text).unwrap_or_else(|error| panic!("{id}: {error:?}"));
        let printed = dsl_text(&snapshot);
        assert_eq!(printed.trim_end_matches('\n'), text.trim_end_matches('\n'), "{id}: txt export must reproduce the example asset");
        assert_eq!(from_dsl_text(&printed).unwrap(), snapshot, "{id}: txt is not a fixed point");
    }
}

#[semio_framework_async_macros::async_test]
async fn json_round_trips_every_example() {
    for (id, text) in EXAMPLES {
        let snapshot = from_dsl_text(text).unwrap_or_else(|error| panic!("{id}: {error:?}"));
        let json = json_text(&snapshot);
        assert_eq!(from_json_text(&json).unwrap_or_else(|error| panic!("{id}: {error:?}")), snapshot, "{id}: json is not a lossless round trip");
    }
}

#[semio_framework_async_macros::async_test]
async fn zip_round_trips_every_example_as_a_real_archive() {
    for (id, text) in EXAMPLES {
        let snapshot = from_dsl_text(text).unwrap_or_else(|error| panic!("{id}: {error:?}"));
        let IoPayload::Binary(bytes) = Block3dIntoZip::serialize(&snapshot).await.unwrap().value else {
            panic!("{id}: zip export must be a binary payload");
        };
        assert!(bytes.starts_with(ZIP_MAGIC), "{id}: zip export must be a real zip 2.0 container");
        assert_eq!(from_zip_bytes(&bytes).unwrap_or_else(|error| panic!("{id}: {error:?}")), snapshot, "{id}: zip is not a lossless round trip");
    }
}

#[semio_framework_async_macros::async_test]
async fn io_declaration_registers_both_directions_of_all_three_formats() {
    let declaration = super::io();
    assert_eq!(declaration.entries.len(), 6, "three formats x two directions");
    let own = "s.block.block3d";
    let mut foreign: Vec<&str> = Vec::new();
    for entry in declaration.entries {
        assert!(entry.from.artifact_kind == own || entry.into.artifact_kind == own, "every entry must touch this subset's own dialect");
        foreign.push(if entry.from.artifact_kind == own { entry.into.artifact_kind } else { entry.from.artifact_kind });
    }
    foreign.sort_unstable();
    foreign.dedup();
    assert_eq!(foreign, vec!["s.stdio.json", "s.stdio.txt", "s.stdio.zip"]);
}
