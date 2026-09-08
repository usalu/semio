
use crate::Block5dSnapshot;
use crate::standards::v1::subsets::any::io::export::serializers::artifacts::json::v_rfc8259::any::json_text;
use crate::standards::v1::subsets::any::io::export::serializers::artifacts::obj::v3_0::any::Block5dIntoObj;
use crate::standards::v1::subsets::any::io::export::serializers::artifacts::png::v1_2::any::Block5dIntoPng;
use crate::standards::v1::subsets::any::io::export::serializers::artifacts::stl::v_ascii::any::Block5dIntoStl;
use crate::standards::v1::subsets::any::io::export::serializers::artifacts::txt::v_utf_8::any::dsl_text;
use crate::standards::v1::subsets::any::io::export::serializers::artifacts::zip::v2_0::any::Block5dIntoZip;
use crate::standards::v1::subsets::any::io::import::deserializers::artifacts::json::v_rfc8259::any::from_json_text;
use crate::standards::v1::subsets::any::io::import::deserializers::artifacts::obj::v3_0::any::ObjIntoBlock5d;
use crate::standards::v1::subsets::any::io::import::deserializers::artifacts::png::v1_2::any::PngIntoBlock5d;
use crate::standards::v1::subsets::any::io::import::deserializers::artifacts::stl::v_ascii::any::StlIntoBlock5d;
use crate::standards::v1::subsets::any::io::import::deserializers::artifacts::txt::v_utf_8::any::from_dsl_text;
use crate::standards::v1::subsets::any::io::import::deserializers::artifacts::zip::v2_0::any::{ZIP_MAGIC, from_zip_bytes};
use semio_framework::io::io_mechanism::{Deserializer, Serializer};
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
        let IoPayload::Binary(bytes) = Block5dIntoZip::serialize(&snapshot).await.unwrap().value else {
            panic!("{id}: zip export must be a binary payload");
        };
        assert!(bytes.starts_with(ZIP_MAGIC), "{id}: zip export must be a real zip 2.0 container");
        assert_eq!(from_zip_bytes(&bytes).unwrap_or_else(|error| panic!("{id}: {error:?}")), snapshot, "{id}: zip is not a lossless round trip");
    }
}

#[semio_framework_async_macros::async_test]
async fn geometry_and_raster_hops_refuse_with_a_reason() {
    let snapshot = Block5dSnapshot::default();
    let empty_text = IoPayload::Text(String::new());
    let empty_binary = IoPayload::Binary(Vec::new());
    for message in [
        Block5dIntoStl::serialize(&snapshot).await.expect_err("stl export must refuse").message,
        Block5dIntoObj::serialize(&snapshot).await.expect_err("obj export must refuse").message,
        Block5dIntoPng::serialize(&snapshot).await.expect_err("png export must refuse").message,
        StlIntoBlock5d::deserialize(&empty_text).await.expect_err("stl import must refuse").message,
        ObjIntoBlock5d::deserialize(&empty_text).await.expect_err("obj import must refuse").message,
        PngIntoBlock5d::deserialize(&empty_binary).await.expect_err("png import must refuse").message,
    ] {
        assert!(message.contains("not supported for"), "every refusing hop must name the reason, got: {message}");
    }
}

/// 🧫️ The exact json bytes the TypeScript mirror (`🚪️io/🧪️tests/🟦️.ts`) writes for the same
/// example assets — a disagreement fails HERE as well as in `bun test`, so neither
/// implementation can drift silently.
const JSON_PARITY_FIXTURES: &[(&str, &str)] = &[("hexagonal-cut-concrete-forest-left", include_str!("../🧫️fixtures/⬅️hexagonal-cut-concrete-forest-left.json")), ("nakagin-capsule", include_str!("../🧫️fixtures/🏢️nakagin-capsule.json"))];

#[semio_framework_async_macros::async_test]
async fn json_matches_the_typescript_parity_fixture() {
    for ((id, text), (fixture_id, fixture)) in EXAMPLES.iter().zip(JSON_PARITY_FIXTURES) {
        assert_eq!(id, fixture_id, "EXAMPLES and JSON_PARITY_FIXTURES must list the same assets in the same order");
        let snapshot = from_dsl_text(text).unwrap_or_else(|error| panic!("{id}: {error:?}"));
        assert_eq!(json_text(&snapshot).as_str(), *fixture, "{id}: the Rust json must match the TypeScript parity fixture byte for byte");
    }
}

#[semio_framework_async_macros::async_test]
async fn io_declaration_registers_both_directions_of_all_six_formats() {
    let declaration = super::io();
    assert_eq!(declaration.entries.len(), 12, "six formats x two directions");
    let own = "s.block.block5d";
    let mut foreign: Vec<&str> = Vec::new();
    for entry in declaration.entries {
        assert!(entry.from.artifact_kind == own || entry.into.artifact_kind == own, "every entry must touch this subset's own dialect");
        foreign.push(if entry.from.artifact_kind == own { entry.into.artifact_kind } else { entry.from.artifact_kind });
    }
    foreign.sort_unstable();
    foreign.dedup();
    assert_eq!(foreign, vec!["s.stdio.json", "s.stdio.obj", "s.stdio.png", "s.stdio.stl", "s.stdio.txt", "s.stdio.zip"]);
}
