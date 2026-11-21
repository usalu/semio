//! ⏳️ each deferred body's producer (`ExampleSource::document_json`); a fixture the DSL grammar no longer reads panics
//! 📚️ Every example the puzzle 5d subset registers builds its document. The descriptor probe and the example picker run
//! there and traps the guest (ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`, S3-PUZZLE). Twins:
//! `◻️2d/…/🧪️tests/🧪️every-example`, `🧊️3d/…/🧪️tests/🧪️every-example`.

use crate::standards::v1::subsets::any::examples;
use crate::standards::v1::subsets::any::io::text::snapshot::parse_dsl;
use crate::Puzzle5dSnapshot;
use semio_plugin::ExampleSourceBody;

/// 🖐️ Each registered example's authored DSL parses, its produced document decodes to that same snapshot, and the
/// snapshot round-trips through the DSL and pack codecs.
/// 📦️ Delivers the authored capsule-dream asset from its repository file the way a host's `InstanceOpen` does, then reads it through the example's own delivery path.
fn delivered_capsule_dream_text() -> &'static str {
    let (name, ..) = crate::examples::puzzle5d::capsule_dream::dsl_bytes_digest();
    let bytes = std::fs::read(semio_framework_test_fixtures::repo_root!().join(name)).unwrap_or_else(|error| panic!("{name}: {error}"));
    semio_plugin::reactor::admit_package_assets(vec![(name.to_owned(), bytes)]);
    crate::examples::puzzle5d::capsule_dream::dsl_text()
}

#[test]
fn every_registered_example_builds_its_document() {
    assert!(!examples().is_empty(), "puzzle 5d registers examples");
    for source in examples() {
        let ExampleSourceBody::Deferred { source_suffix, source: authored, .. } = source.body() else { panic!("{} is a deferred DSL example", source.id()) };
        let text = if source_suffix.ends_with(".digest") { delivered_capsule_dream_text() } else { std::str::from_utf8(authored).unwrap_or_else(|error| panic!("{} is UTF-8: {error}", source.id())) };
        let parsed = parse_dsl(text).unwrap_or_else(|error| panic!("{} example DSL parses: {error}", source.id()));
        let built: Puzzle5dSnapshot = semio_framework_pack_json::from_json_str(&source.document_json(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_else(|error| panic!("{} document decodes: {error:?}", source.id()));
        assert!(built == parsed, "{} document equals its authored DSL", source.id());
        if let Some(handle) = &built.kind_catalogs {
            use crate::standards::v1::subsets::any::schema::kind_catalogs::{kind_catalogs_child_handle, kind_catalogs_of};
            let catalogs = kind_catalogs_of(&built.kind_catalogs, &built.kind_catalogs_extra).expect("authored catalog");
            assert_eq!(*handle, kind_catalogs_child_handle(&catalogs), "{} catalog handle matches its authored kit types", source.id());
        }
        semio_os_kernel::os_store::test_support::assert_dsl_round_trip(&built);
        semio_os_kernel::os_store::test_support::assert_dsl_pack_equivalence(&built);
    }
}

/// 🔢️ Each shipped example parses via `parse_dsl` into the part/fastener counts of the language-agnostic `📚️example-counts` fixture, and its pack JSON (the `puzzle5dParseDslJson` wasm payload Storybook reads) re-reads through `serde_json` with the same counts and a `3d.origin` per part.
#[test]
fn every_example_parses_into_its_fixture_counts() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/📚️example-counts/🔣️.json")).expect("fixture is JSON");
    let sources = examples();
    let expected = fixture["examples"].as_array().expect("examples array");
    assert_eq!(expected.len(), sources.len());
    for source in sources {
        let id = source.id();
        let row = expected.iter().find(|row| row["id"] == id).unwrap_or_else(|| panic!("{id} has a fixture count row"));
        let ExampleSourceBody::Deferred { source_suffix, source: authored, .. } = source.body() else { panic!("{id} is a deferred DSL example") };
        let text = if source_suffix.ends_with(".digest") { delivered_capsule_dream_text() } else { std::str::from_utf8(authored).unwrap_or_else(|error| panic!("{id} is UTF-8: {error}")) };
        let snapshot = parse_dsl(text).unwrap_or_else(|error| panic!("{id} parses: {error}"));
        assert_eq!(snapshot.parts.len() as u64, row["parts"].as_u64().unwrap(), "{id} parts");
        assert_eq!(snapshot.fasteners.len() as u64, row["fasteners"].as_u64().unwrap(), "{id} fasteners");
        let json: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&snapshot)).expect("pack JSON is JSON");
        assert_eq!(json["parts"].as_array().map(Vec::len), Some(snapshot.parts.len()), "{id} JSON parts");
        assert_eq!(json["fasteners"].as_array().map_or(0, Vec::len), snapshot.fasteners.len(), "{id} JSON fasteners");
        assert!(json["parts"].as_array().unwrap().iter().all(|part| part["3d"]["origin"].as_array().is_some_and(|origin| origin.len() == 3)), "{id} parts carry 3d.origin");
    }
}

/// 🎛️ The boot example's original mutation backing fits one mounted release.
#[test]
fn boot_example_mutation_backing_fits_a_mounted_release() {
    use crate::editor::puzzle5d::interaction::Puzzle5dCommand;
    use crate::editor::puzzle5d::kit::example_and_brush_mesh::Puzzle5dSetActiveExampleWork;
    use crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;
    use crate::standards::v1::subsets::any::schema::play_snapshot::Puzzle5dPlaySnapshot;
    let current = Puzzle5dPlaySnapshot::new(Puzzle5dSnapshot::default());
    let command = Puzzle5dCommand::try_from_action("setActiveExample", Some(semio_framework_pack_json::json!({ "exampleId": "concrete-forest" })), None).expect("registered example command");
    let target = Puzzle5dSetActiveExampleWork::target(&command).unwrap();
    let rows = target.parts.len() + target.fasteners.len()
        + target.kind_compatibility.as_ref().and_then(semio_framework_pack_json::Value::as_array).map_or(0, Vec::len);
    let capacity = Puzzle5dSetActiveExampleWork::mutation_capacity(&command, &current);
    let demand = Puzzle5dSetActiveExampleWork::original_birth_demands(capacity).unwrap();
    assert_eq!(capacity, 4 + rows);
    assert_eq!(demand.capacity_bytes, capacity * std::mem::size_of::<Puzzle5dMutation>());
    assert!(demand.capacity_bytes <= semio_plugin::artifact_app_laws::fixture_mounted_policy().close.maximum_release_bytes);
}
