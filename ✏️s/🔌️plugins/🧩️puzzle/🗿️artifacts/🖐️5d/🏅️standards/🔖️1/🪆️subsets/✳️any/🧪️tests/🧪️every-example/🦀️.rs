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
        semio_os_store_test_support::assert_dsl_round_trip(&built);
        semio_os_store_test_support::assert_dsl_pack_equivalence(&built);
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
    use crate::standards::v1::subsets::any::schema::mutations::{Puzzle5dMutation, SCENE_PART_SPLICE_BATCH, SCENE_ROW_CHUNK, SCENE_SPLICE_BATCH};
    use crate::standards::v1::subsets::any::schema::play_snapshot::Puzzle5dPlaySnapshot;
    let current = Puzzle5dPlaySnapshot::new(Puzzle5dSnapshot::default());
    let command = Puzzle5dCommand::try_from_action("setActiveExample", Some(semio_framework_pack_json::json!({ "exampleId": "concrete-forest" })), None).expect("registered example command");
    let target = Puzzle5dSetActiveExampleWork::target(&command).unwrap();
    let compatibility = target.kind_compatibility.as_ref().and_then(semio_framework_pack_json::Value::as_array).map_or(0, Vec::len);
    let rows = [(target.parts.len(), SCENE_PART_SPLICE_BATCH), (target.fasteners.len(), SCENE_SPLICE_BATCH), (compatibility, SCENE_SPLICE_BATCH)].into_iter()
        .map(|(count, limit)| count.div_ceil(SCENE_ROW_CHUNK).div_ceil(limit)).sum::<usize>();
    let capacity = Puzzle5dSetActiveExampleWork::mutation_capacity(&command, &current);
    let demand = Puzzle5dSetActiveExampleWork::original_birth_demands(capacity).unwrap();
    assert_eq!(capacity, 4 + rows);
    assert_eq!(demand.capacity_bytes, capacity * std::mem::size_of::<Puzzle5dMutation>());
    assert!(demand.capacity_bytes <= semio_plugin::artifact_app_laws::fixture_mounted_policy().close.maximum_release_bytes);
}

use crate::standards::v1::subsets::any::schema::mutations::{Puzzle5dMutation, inverse_puzzle5d_mutation};
use protocol::ColdMutation;
use protocol::{OpBinary, OpText};
use semio_framework_value::FromValue;

fn assert_operation_wire(operation: &Puzzle5dMutation, tag: u8) {
    use crate::standards::v1::subsets::any::io::binary::mutations::prepared_operation_wire_source;
    use semio_framework_job::{Generation, OriginalJobFixtureCaller, allocate_operation_id};
    use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress};
    use semio_os_kernel::original_receiving::{ReceivingStock, receive_to_completion};
    use semio_os_kernel::os_store::ArtifactPreparedOperationCursor;

    let grant = RetainedCloneGrant {
        maximum_items: 1 << 20, maximum_copy_bytes: 1 << 24,
        maximum_capacity_bytes: 1 << 24, maximum_release_bytes: 1 << 24, maximum_depth: 128,
    };
    let mut caller = OriginalJobFixtureCaller::admit(allocate_operation_id(), Generation(1), grant, || Some(0), u64::MAX, 1 << 20);
    caller.birth_returns(grant);
    let source = prepared_operation_wire_source(operation).unwrap();
    let mut cursor = ArtifactPreparedOperationCursor::default();
    let mut prepared = Vec::new();
    for _ in 0..4096 {
        let grant = caller.within_stock(grant);
        caller.turn(grant, |cx| {
            let mut output = [0; 64];
            let (step, work) = cx.with_original_effect(grant, |_, effect| {
                if cursor.complete { cursor.close(effect) } else { cursor.advance(source, &mut output, effect) }
            })?.expect("funded operation wire step");
            let step = step.map_err(|error| error.reason)?;
            cx.consume_work(work);
            cx.consume_retained(RetainedCloneProgress {
                copied_items: step.processed_items, copied_bytes: step.copied_bytes,
                retained_capacity_bytes: step.retained_capacity_bytes, released_bytes: step.released_bytes,
            })?;
            prepared.extend_from_slice(&output[..step.written_bytes]);
            Ok(())
        }).expect("original operation wire");
        if cursor.closed { break; }
    }
    assert!(cursor.closed, "prepared operation wire settles and closes");
    caller.retire(grant);

    let direct = operation.encode_op().unwrap();
    assert_eq!(&direct[..2], &[1, tag]);
    assert_eq!(prepared, direct);
    assert_eq!(Puzzle5dMutation::decode_op(&direct).unwrap(), *operation);
    let retained = receive_to_completion::<Puzzle5dMutation>(&direct, (0, direct.len()), ReceivingStock {
        stock: grant, quantum: grant, fuel: 1 << 20,
    }).expect("retained operation receiving");
    assert_eq!(retained, *operation);
}

#[test]
fn scene_splice_replays_both_codecs_and_restores_the_exact_snapshot() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔀️scene/🔣️.json")).unwrap();
    let value = |key: &str| semio_framework_pack_json::from_json_str::<Puzzle5dSnapshot>(
        &fixture[key].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject,
    ).unwrap();
    let before = value("before");
    let delete: Puzzle5dMutation = crate::standards::v1::subsets::any::schema::mutations::delete_part(before.parts[0].id.clone());
    let delete_wire: Vec<u8> = serde_json::from_value(fixture["deletePartWire"].clone()).unwrap();
    assert_eq!(delete.encode_op().unwrap(), delete_wire);
    assert_operation_wire(&delete, 1);
    let mut scene = before.clone();
    let mut inverse = Vec::new();
    for step in fixture["steps"].as_array().unwrap() {
        let decode = |key: &str| Puzzle5dMutation::from_value(semio_framework_pack_json::from_json_str(
            &step[key].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject,
        ).unwrap()).unwrap();
        let forward = decode("forward");
        let undo = decode("inverse");
        for operation in [&forward, &undo] {
            assert_eq!(Puzzle5dMutation::parse_op(&operation.print_op()).unwrap(), *operation);
            assert_operation_wire(operation, 39);
        }
        assert_eq!(inverse_puzzle5d_mutation(&scene, &forward).unwrap(), [undo.clone()]);
        scene = protocol::apply_diff_cold(ColdMutation::<Puzzle5dSnapshot>::diff(&forward, &scene).diff(), &scene).unwrap();
        inverse.push(undo);
    }
    assert_eq!(scene, value("after"));
    for undo in inverse.iter().rev() {
        scene = protocol::apply_diff_cold(ColdMutation::<Puzzle5dSnapshot>::diff(undo, &scene).diff(), &scene).unwrap();
    }
    assert_eq!(scene, before);
}

#[test]
fn scene_splice_batch_replays_both_codecs_and_restores_the_exact_snapshot() {
    use crate::standards::v1::subsets::any::schema::mutations::{SpliceSceneRowBatch, SCENE_SPLICE_BATCH};

    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔀️scene/🔣️.json")).unwrap();
    let snapshot = |key: &str| semio_framework_pack_json::from_json_str::<Puzzle5dSnapshot>(
        &fixture[key].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject,
    ).unwrap();
    let rows = fixture["steps"].as_array().unwrap();
    let edits = |key: &str| rows.iter().map(|row| {
        let operation = semio_framework_pack_json::from_json_str::<Puzzle5dMutation>(
            &row[key].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject,
        ).unwrap();
        let Puzzle5dMutation::SpliceSceneRows(edit) = operation else { panic!("fixture row must be a scene splice") };
        edit
    }).collect::<Vec<_>>();
    let forward = Puzzle5dMutation::from(SpliceSceneRowBatch { edits: edits("forward") });
    let mut inverse = edits("inverse");
    inverse.reverse();
    let inverse = Puzzle5dMutation::from(SpliceSceneRowBatch { edits: inverse });

    for operation in [&forward, &inverse] {
        assert_eq!(Puzzle5dMutation::parse_op(&operation.print_op()).unwrap(), *operation);
        assert_operation_wire(operation, 40);
    }
    let apply = |operation: &Puzzle5dMutation, base: &Puzzle5dSnapshot| {
        protocol::apply_diff_cold(ColdMutation::<Puzzle5dSnapshot>::diff(operation, base).diff(), base).unwrap()
    };
    let before = snapshot("before");
    assert_eq!(inverse_puzzle5d_mutation(&before, &forward).unwrap(), [inverse.clone()]);
    let after = apply(&forward, &before);
    assert_eq!(after, snapshot("after"));
    assert_eq!(apply(&inverse, &after), before);

    let rows = edits("forward");
    let too_many = Puzzle5dMutation::from(SpliceSceneRowBatch { edits: vec![rows[0].clone(); SCENE_SPLICE_BATCH + 1] });
    assert!(matches!(ColdMutation::<Puzzle5dSnapshot>::diff(&too_many, &before).admit(), protocol::MutationAdmission::Refused(_)));
}

#[test]
fn capsule_scene_batches_fit_a_store_operation() {
    use crate::standards::v1::subsets::any::schema::mutations::{SpliceSceneRowBatch, SpliceSceneRows, SCENE_PART_SPLICE_BATCH, SCENE_ROW_CHUNK, SCENE_SPLICE_BATCH};
    use semio_os_kernel::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES;

    let snapshot = parse_dsl(delivered_capsule_dream_text()).unwrap();
    let parts = snapshot.parts.iter().collect::<Vec<_>>();
    let count = SCENE_ROW_CHUNK * SCENE_PART_SPLICE_BATCH;
    for (group, rows) in parts.chunks(count).enumerate() {
        let edits = rows.chunks(SCENE_ROW_CHUNK).enumerate().map(|(index, parts)| SpliceSceneRows {
            table: "parts".into(),
            start: group * count + index * SCENE_ROW_CHUNK,
            parts: parts.iter().map(|part| (*part).clone()).collect(),
            ..Default::default()
        }).collect();
        let operation = Puzzle5dMutation::from(SpliceSceneRowBatch { edits });
        let wire = operation.encode_op().unwrap();
        assert!(wire.len() <= ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES, "Capsule parts group {group} uses {} bytes", wire.len());
    }
    let count = SCENE_ROW_CHUNK * SCENE_SPLICE_BATCH;
    for (group, rows) in snapshot.fasteners.chunks(count).enumerate() {
        let edits = rows.chunks(SCENE_ROW_CHUNK).enumerate().map(|(index, fasteners)| SpliceSceneRows {
            table: "fasteners".into(),
            start: group * count + index * SCENE_ROW_CHUNK,
            fasteners: fasteners.to_vec(),
            ..Default::default()
        }).collect();
        let operation = Puzzle5dMutation::from(SpliceSceneRowBatch { edits });
        let wire = operation.encode_op().unwrap();
        assert!(wire.len() <= ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES, "Capsule fasteners group {group} uses {} bytes", wire.len());
    }
}
