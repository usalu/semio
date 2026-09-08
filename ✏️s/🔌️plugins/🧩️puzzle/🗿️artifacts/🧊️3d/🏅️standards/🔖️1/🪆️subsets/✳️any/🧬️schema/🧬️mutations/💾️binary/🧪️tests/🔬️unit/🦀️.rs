
use super::*;

#[test]
fn puzzle3d_document_vcs_replays_granular_operations() {
    use crate::standards::v1::subsets::any::schema::empty_puzzle3d_snapshot;
    use crate::{PUZZLE_3D_SCHEMA, Puzzle3dObject};
    use store::{ArtifactCommand, create_document_envelope};

    let mut store = semio_framework::io::resolve_ready(Puzzle3dStore::new(create_document_envelope(PUZZLE_3D_SCHEMA, "puzzle3d", empty_puzzle3d_snapshot(), None))).expect("store");
    semio_framework::io::resolve_ready(store.dispatch(ArtifactCommand::Apply {
        mutations: vec![crate::standards::v1::subsets::any::schema::mutations::create_object(
            Puzzle3dObject { id: "o1".into(), label: None, object_kind: None, anchor: Default::default(), origin: [0.0, 0.0, 0.0], orientation: None, scale: None, mesh_url: None, vortices: Vec::new(), hidden: false, locked: false },
            None,
        )],
        description: None,
    }))
    .expect("apply");
    let projection = store.snapshot().expect("projection");
    assert_eq!(projection.objects.len(), 1);
    assert_eq!(projection.objects[0].id, "o1");
}

/// 🔗️ Minimal scene JSON matching `SceneConfig`'s real wire shape (camelCase, per its
/// `#[serde(rename = ...)]` attrs) — deserialized rather than struct-literal-built since
/// `SceneConfig`'s fields are `pub(crate)` (this node only needs the type nameable, not its
/// fields, to carry it inside `Puzzle3dEngineCommand::SetScene`). A byte-identical copy of this
/// helper also lives in `crate::editor::puzzle3d::precompute`'s own test module, for the two
/// dispatch tests that moved there (a schema test file must not depend on the app).
pub(crate) fn sample_scene_config() -> crate::standards::v1::subsets::any::schema::SceneConfig {
    let json = r#"{
            "fixture": {
                "objects": [{"id": "host", "objectKind": "Host", "meshUrl": "/test/host.glb", "origin": [0,0,0], "orientation": [0,0,0,1], "vortices": [{"id": "v0", "vortexKind": "port-a", "position": [0,0,0], "direction": [0,0,-1]}]}],
                "attractions": [],
                "targetVolumes": []
            },
            "kindCatalogs": {"objects": [{"id": "Host", "representations": [{"id": "r0", "name": "default", "url": "/test/host.glb"}], "vortices": []}], "vortices": [{"id": "port-a"}], "cables": []},
            "kindCompatibility": [],
            "overlapBudget": 0.02,
            "seed": 1
        }"#;
    serde_json::from_str(json).expect("sample scene config parses")
}

#[test]
fn engine_command_set_scene_binary_round_trips_and_agrees_with_text() {
    let command = Puzzle3dEngineCommand::SetScene { scene: sample_scene_config() };
    semio_framework_os_kernel::os_store::test_support::assert_op_text_binary_equivalence(&command);
    let bytes = encode_engine_command(&command).expect("encode");
    assert_eq!(decode_engine_command(&bytes).expect("decode"), command);
}

#[test]
fn engine_command_brush_preview_binary_round_trips_and_agrees_with_text() {
    let command = Puzzle3dEngineCommand::BrushPreview { vortex_full_id: "host:v0".to_string(), candidate_index: 2 };
    semio_framework_os_kernel::os_store::test_support::assert_op_text_binary_equivalence(&command);
    let bytes = encode_engine_command(&command).expect("encode");
    assert_eq!(decode_engine_command(&bytes).expect("decode"), command);
}

#[test]
fn engine_command_update_kind_weights_binary_round_trips_and_agrees_with_text() {
    let mut object_weights = std::collections::BTreeMap::new();
    object_weights.insert("Host".to_string(), 0.5);
    let command = Puzzle3dEngineCommand::UpdateKindWeights { object_weights, vortex_weights: std::collections::BTreeMap::new() };
    semio_framework_os_kernel::os_store::test_support::assert_op_text_binary_equivalence(&command);
    let bytes = encode_engine_command(&command).expect("encode");
    assert_eq!(decode_engine_command(&bytes).expect("decode"), command);
}

// 🎯️ Behavioral-parity dispatch tests (`dispatch_set_scene_then_apply_and_compose_fill_count_round_trip`,
// `dispatch_brush_preview_without_scene_returns_none`) relocated to
// `crate::editor::puzzle3d::precompute`'s own test module (ticket
// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): they construct
// `Puzzle3dPrecomputeSession`, which is now an app type this schema test file must not depend on.
