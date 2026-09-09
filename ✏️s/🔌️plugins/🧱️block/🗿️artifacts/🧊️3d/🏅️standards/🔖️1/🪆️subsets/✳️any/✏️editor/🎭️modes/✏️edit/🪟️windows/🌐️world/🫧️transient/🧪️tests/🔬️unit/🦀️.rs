use super::*;
use protocol::{Mutation, MutationDiff, OpBinary, OpText};
use std::collections::BTreeMap;
use store::{ArtifactDsl, ArtifactPack};

#[test]
fn preview_replacement_publication_and_retirement_obey_tiny_grants() {
    use semio_framework_plugin::WindowTransientOwner;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔀️codec-contracts/🔣️.json")).unwrap();
    let owners = Block3dWorldWindowTransientOwner::build_owners();
    for row in fixture["cases"].as_array().unwrap() {
        let before: Block3dWorldWindowTransient = serde_json::from_value(row["before"].clone()).unwrap();
        let after: Block3dWorldWindowTransient = serde_json::from_value(row["after"].clone()).unwrap();
        let mut store = store::TransientStore::<_, Block3dWorldWindowTransientMutation>::new(before);
        let mut publication = store.begin_publish_one_leased(semio_framework_job::OperationId(1), 0, SetBrushPreview { preview: after.brush_preview }.into(), owners.preparation.as_ref(), owners.state_retirement.clone()).unwrap();
        let zero = store::ArtifactStoreOneItemGrant { maximum_items: 0, maximum_bytes: 4096 };
        assert!(matches!(store.advance_publish_one(&mut publication, zero).unwrap(), store::ArtifactStoreOneItemAdvance::Blocked));
        assert_eq!(publication.progress().completed_items, 0);
        let grant = store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 1 };
        for _ in 0..8 {
            if matches!(store.advance_publish_one(&mut publication, grant).unwrap(), store::ArtifactStoreOneItemAdvance::Published(_)) { break; }
        }
        assert_eq!(serde_json::to_value(store.current_root().as_ref()).unwrap(), row["after"]);
        assert_eq!(publication.progress().completed_bytes, 0);
        assert!(publication.acknowledge());
        assert_eq!(publication.close_step(zero).unwrap(), store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        for _ in 0..4096 {
            match publication.close_step(grant).unwrap() {
                store::SnapshotRetirementStep::Complete => break,
                store::SnapshotRetirementStep::Pending { released_items, released_bytes } => assert!(released_items <= 1 && released_bytes <= 1),
                store::SnapshotRetirementStep::Blocked => panic!("unaliased preview publication must close"),
            }
        }
        assert!(publication.terminal_is_empty());
    }
    eprintln!("[DEBUG] Block3D preview: neutral serde snapshots published and retired under one-item/one-byte grants; zero-item grants preserve ownership");
}

#[test]
fn preview_partition_matches_language_neutral_json_oracle() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪟️preview-partition/🔣️.json")).expect("fixture");
    let mut typed = BTreeMap::<String, Block3dWorldWindowTransient>::new();
    let mut oracle = serde_json::Map::new();
    for step in fixture["steps"].as_array().expect("steps") {
        let window_id = step["windowId"].as_str().expect("window id").to_string();
        let preview = serde_json::from_value::<Option<Block3dBrushPreview>>(step["preview"].clone()).expect("preview");
        let mutation = Block3dWorldWindowTransientMutation::from(SetBrushPreview { preview });
        let state = typed.entry(window_id.clone()).or_default();
        Mutation::diff(&mutation, state).apply_to(state);
        oracle.insert(window_id, step["preview"].clone());
        store::os_store::test_support::assert_op_text_binary_equivalence(&mutation);
    }
    let typed = typed.into_iter().map(|(window_id, state)| (window_id, serde_json::to_value(state.brush_preview).expect("typed preview"))).collect::<serde_json::Map<_, _>>();
    assert_eq!(serde_json::Value::Object(typed), fixture["expected"]);
    assert_eq!(serde_json::Value::Object(oracle), fixture["expected"]);
}

#[test]
fn block3d_world_preview_codecs_and_inverse_match_neutral_vectors() {
    let oracle: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔀️codec-contracts/🔣️.json")).unwrap();
    for row in oracle["cases"].as_array().unwrap() {
        let before: Block3dWorldWindowTransient = dsl::json::from_json_str(&row["before"].to_string()).unwrap();
        let expected: Block3dWorldWindowTransient = dsl::json::from_json_str(&row["after"].to_string()).unwrap();
        let operation: Block3dWorldWindowTransientMutation = SetBrushPreview { preview: expected.brush_preview.clone() }.into();
        let outcome = operation.diff(&before);
        assert!(outcome.messages().is_empty());
        let applied = outcome.diff().apply(&before).unwrap();
        assert_eq!(applied, expected);
        let encoded: serde_json::Value = serde_json::from_str(&dsl::json::to_json_string(&applied)).unwrap();
        assert_eq!(encoded, row["after"]);
        assert_eq!(Block3dWorldWindowTransient::parse_dsl(&applied.print_dsl()).unwrap(), applied);
        let packed = applied.encode_pack_with(&Default::default()).unwrap();
        assert_eq!(Block3dWorldWindowTransient::decode_pack_with(&packed, &Default::default()).unwrap(), applied);
        let operation_wire = dsl::json::to_json_string(&operation);
        let independent: serde_json::Value = serde_json::from_str(&operation_wire).unwrap();
        assert_eq!(dsl::json::from_json_str::<Block3dWorldWindowTransientMutation>(&serde_json::to_string(&independent).unwrap()).unwrap(), operation);
        assert_eq!(Block3dWorldWindowTransientMutation::parse_op(&operation.print_op()).unwrap(), operation);
        assert_eq!(Block3dWorldWindowTransientMutation::decode_op(&operation.encode_op().unwrap()).unwrap(), operation);
        let mut restored = applied;
        for inverse in operation.inverse(&before) {
            restored = inverse.diff(&restored).diff().apply(&restored).unwrap();
        }
        assert_eq!(restored, before);
    }
}
