//! 📡️ Puzzle 3d artifact — the state-patch-representation codec: `encode_op`/`decode_op` for
//! `Puzzle3dOperation`'s binary wire form, `encode_engine_command`/`decode_engine_command` for the
//! headless engine's own `Puzzle3dEngineCommand` envelope, plus the `DocumentEnvelope`/
//! `DocumentStore` aliases every puzzle-3d host binds. Renamed from the pre-consolidation
//! `📡️protocol` module; both wire formats are unchanged (`dsl::DslOps`'s generated `OpBinary`).

use crate::artifacts::puzzle3d::op::Puzzle3dOperation;
use crate::artifacts::puzzle3d::Puzzle3dProjection;
use protocol::OpBinary;
use store::{DocumentEnvelope, DocumentStore};

/// 📦️ Encodes a `Puzzle3dOperation` to its binary command form.
pub fn encode_op(operation: &Puzzle3dOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `Puzzle3dOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<Puzzle3dOperation, protocol::ProtocolError> {
    Puzzle3dOperation::decode_op(bytes)
}

//#region 🔖️Store
pub type Puzzle3dEnvelope = DocumentEnvelope<Puzzle3dProjection, Puzzle3dOperation>;
pub type Puzzle3dStore = DocumentStore<Puzzle3dProjection, Puzzle3dOperation>;
//#endregion 🔖️Store

//#region 🔖️Puzzle3dEngineCommand
/// 🎯️ Re-exports the puzzle 3d app-engine command envelope (`HEADLESS-ENGINE-LAW-AND-OFFENDER-FIXES`).
/// `#[derive(dsl::DslOps)]` is applied where the type is declared, in `⚙️engine` — not here — because
/// the derive's generated code needs `SceneConfig`/`BrushPlacePayload` (types `⚙️engine` owns) by
/// value; re-exporting it here plus wrapping `encode_op`/`decode_op` mirrors exactly how
/// `Puzzle3dOperation` (declared in `🔧️op`) is surfaced above.
pub use crate::artifacts::puzzle3d::engine::{Puzzle3dEngineCommand, Puzzle3dEngineOutcome};

/// 📦️ Encodes a `Puzzle3dEngineCommand` to its binary command form.
pub fn encode_engine_command(command: &Puzzle3dEngineCommand) -> Result<Vec<u8>, protocol::ProtocolError> {
    command.encode_op()
}

/// 📖️ Decodes a `Puzzle3dEngineCommand` from its binary command form.
pub fn decode_engine_command(bytes: &[u8]) -> Result<Puzzle3dEngineCommand, protocol::ProtocolError> {
    Puzzle3dEngineCommand::decode_op(bytes)
}
//#endregion 🔖️Puzzle3dEngineCommand

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puzzle3d_document_vcs_replays_granular_operations() {
        use crate::artifacts::puzzle3d::engine::empty_puzzle3d_projection;
        use crate::artifacts::puzzle3d::{Puzzle3dObject, PUZZLE_3D_SCHEMA};
        use store::{create_document_envelope, DocumentCommand};

        let mut store = Puzzle3dStore::new(create_document_envelope(PUZZLE_3D_SCHEMA, "puzzle3d", empty_puzzle3d_projection(), None));
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![Puzzle3dOperation::SetObject {
                    index: 0,
                    object: Puzzle3dObject { id: "o1".into(), label: None, object_kind: None, origin: [0.0, 0.0, 0.0], orientation: None, scale: None, mesh_url: None, vortices: Vec::new(), hidden: false, locked: false },
                }],
                description: None,
            })
            .expect("apply");
        let projection = store.projection().expect("projection");
        assert_eq!(projection.objects.len(), 1);
        assert_eq!(projection.objects[0].id, "o1");
    }

    /// 🔗️ Minimal scene JSON matching `engine::SceneConfig`'s real wire shape (camelCase, per its
    /// `#[serde(rename = ...)]` attrs) — deserialized rather than struct-literal-built since
    /// `SceneConfig`'s fields are private to `⚙️engine` (this node only needs the type nameable, not
    /// its fields, to carry it inside `Puzzle3dEngineCommand::SetScene`).
    pub(crate) fn sample_scene_config() -> crate::artifacts::puzzle3d::engine::SceneConfig {
        let json = r#"{
            "fixture": {
                "objects": [{"id": "host", "objectKind": "Host", "meshUrl": "/test/host.glb", "origin": [0,0,0], "orientation": [0,0,0,1], "vortices": [{"id": "v0", "vortexKind": "port-a", "position": [0,0,0], "direction": [0,0,-1]}]}],
                "attractions": [],
                "targetVolumes": []
            },
            "kindCatalogs": {"objects": [{"id": "Host", "meshUrl": "/test/host.glb", "vortices": []}], "vortices": [{"id": "port-a"}], "cables": []},
            "kindCompatibility": [],
            "overlapBudget": 0.02,
            "seed": 1
        }"#;
        serde_json::from_str(json).expect("sample scene config parses")
    }

    #[test]
    fn engine_command_set_scene_binary_round_trips_and_agrees_with_text() {
        let command = Puzzle3dEngineCommand::SetScene { scene: sample_scene_config() };
        store::test_support::assert_op_text_binary_equivalence(&command);
        let bytes = encode_engine_command(&command).expect("encode");
        assert_eq!(decode_engine_command(&bytes).expect("decode"), command);
    }

    #[test]
    fn engine_command_brush_preview_binary_round_trips_and_agrees_with_text() {
        let command = Puzzle3dEngineCommand::BrushPreview { vortex_full_id: "host:v0".to_string(), candidate_index: 2 };
        store::test_support::assert_op_text_binary_equivalence(&command);
        let bytes = encode_engine_command(&command).expect("encode");
        assert_eq!(decode_engine_command(&bytes).expect("decode"), command);
    }

    #[test]
    fn engine_command_update_kind_weights_binary_round_trips_and_agrees_with_text() {
        let mut object_weights = std::collections::BTreeMap::new();
        object_weights.insert("Host".to_string(), 0.5);
        let command = Puzzle3dEngineCommand::UpdateKindWeights { object_weights, vortex_weights: std::collections::BTreeMap::new() };
        store::test_support::assert_op_text_binary_equivalence(&command);
        let bytes = encode_engine_command(&command).expect("encode");
        assert_eq!(decode_engine_command(&bytes).expect("decode"), command);
    }

    /// 🎯️ Behavioral parity: `dispatch` must reach the exact same engine logic the old JSON-string
    /// wasm-bindgen methods delegated to — `SetScene` seeds a fill session, `ApplyFillCount`/
    /// `ComposeFillDisplay` read/apply its prefix, matching what `⚙️engine`'s own
    /// `precompute_session_native_wrapper_exercises_public_methods` test already asserts for the
    /// pre-dispatch API.
    #[test]
    fn dispatch_set_scene_then_apply_and_compose_fill_count_round_trip() {
        use crate::artifacts::puzzle3d::engine::{Puzzle3dEngineOutcome, Puzzle3dPrecomputeSession};

        let mut session = Puzzle3dPrecomputeSession::new();
        session.dispatch(Puzzle3dEngineCommand::SetScene { scene: sample_scene_config() }).expect("set scene");
        assert!(!session.fill_is_done(), "a freshly seeded fill session has not stalled or hit max_count yet");

        session.precompute_step(50);

        let outcome = session.dispatch(Puzzle3dEngineCommand::ComposeFillDisplay { count: 0 }).expect("compose fill display");
        let Puzzle3dEngineOutcome::Fixture(fixture) = outcome else { panic!("expected a Fixture outcome") };
        assert!(fixture.objects.iter().any(|object| object.id == "host"), "the base scene's host object must survive compose_fill_display(0)");

        let outcome = session.dispatch(Puzzle3dEngineCommand::ApplyFillCount { count: 0 }).expect("apply fill count");
        let Puzzle3dEngineOutcome::Fixture(fixture) = outcome else { panic!("expected a Fixture outcome") };
        assert!(fixture.objects.iter().any(|object| object.id == "host"));
    }

    #[test]
    fn dispatch_brush_preview_without_scene_returns_none() {
        use crate::artifacts::puzzle3d::engine::{Puzzle3dEngineOutcome, Puzzle3dPrecomputeSession};

        let mut session = Puzzle3dPrecomputeSession::new();
        let outcome = session.dispatch(Puzzle3dEngineCommand::BrushPreview { vortex_full_id: "host:v0".to_string(), candidate_index: 0 }).expect("brush preview never errors");
        assert_eq!(outcome, Puzzle3dEngineOutcome::BrushPreview(None), "no scene means no cached brush candidates yet");
    }
}
//#endregion 🧪️Tests

//#region 🧪️WireBaselineDump
#[cfg(test)]
mod wire_baseline_dump {
    use super::*;
    use crate::artifacts::puzzle3d as puzzle_3d;
    use protocol::OpText;
    use serde_json::json;

    fn ops() -> Vec<Puzzle3dOperation> {
        let object: puzzle_3d::Puzzle3dObject = serde_json::from_value(json!({"id":"o1","label":"L","objectKind":"Capsule","origin":[1.0,2.0,3.0],"orientation":[0.0,0.0,0.0,1.0],"scale":[2.0,3.0,4.0],"meshUrl":"/m.glb","vortices":[{"id":"v0","vortexKind":"k","position":[0.0,0.0,0.0],"direction":[0.0,0.0,1.0],"radius":3.0,"hidden":false,"locked":false}],"hidden":false,"locked":true})).unwrap();
        let attraction: puzzle_3d::Puzzle3dAttraction = serde_json::from_value(json!({"id":"a1","attracting":"o1:v0","attracted":"o2:v0","gap":1.0,"shift":2.0,"rise":3.0,"rotation":4.0,"turn":5.0,"tilt":6.0})).unwrap();
        let target_volume: puzzle_3d::Puzzle3dTargetVolume = serde_json::from_value(json!({"id":"t1","origin":[0.0,1.0,2.0],"orientation":[0.0,0.0,0.0,1.0],"scale":5.0,"hidden":false,"locked":false})).unwrap();
        let reference: puzzle_3d::Puzzle3dReference = serde_json::from_value(json!({"id":"r1","source":{"url":"/u.png","mediaKind":"image"},"origin":[0.0,0.0,0.0],"widthWorld":4.0,"locked":false,"hidden":false})).unwrap();
        let meta: puzzle_3d::Puzzle3dMeta = serde_json::from_value(json!({"kindCompatibility":[{"source":"a","target":"b","bidirectional":true,"important":false,"specificity":"vortex"}]})).unwrap();
        let document = puzzle_3d::Puzzle3dProjection::default();
        vec![
            Puzzle3dOperation::SetObject { index: 0, object },
            Puzzle3dOperation::RemoveObject { id: "o1".into() },
            Puzzle3dOperation::SetAttraction { index: 1, attraction },
            Puzzle3dOperation::RemoveAttraction { id: "a1".into() },
            Puzzle3dOperation::SetTargetVolume { index: 2, target_volume },
            Puzzle3dOperation::RemoveTargetVolume { id: "t1".into() },
            Puzzle3dOperation::SetReference { index: 3, reference },
            Puzzle3dOperation::RemoveReference { id: "r1".into() },
            Puzzle3dOperation::SetMeta { meta },
            Puzzle3dOperation::SetDocument { document },
        ]
    }

    #[test]
    fn debug_wire_dump() {
        for operation in ops() {
            let text = operation.print_op();
            let bytes = encode_op(&operation).expect("encode");
            let hex: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
            println!("[WIRE] {text} | {} | {hex}", bytes.len());
            assert_eq!(decode_op(&bytes).expect("decode"), operation);
        }
        let mut object_weights = std::collections::BTreeMap::new();
        object_weights.insert("Host".to_string(), 0.5);
        for command in [
            Puzzle3dEngineCommand::ApplyFillCount { count: 7 },
            Puzzle3dEngineCommand::ComposeFillDisplay { count: 9 },
            Puzzle3dEngineCommand::UpdateKindWeights { object_weights, vortex_weights: std::collections::BTreeMap::new() },
            Puzzle3dEngineCommand::BrushPreview { vortex_full_id: "host:v0".to_string(), candidate_index: 2 },
        ] {
            let text = command.print_op();
            let bytes = encode_engine_command(&command).expect("encode");
            let hex: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
            println!("[WIRE-ENGINE] {text} | {} | {hex}", bytes.len());
            assert_eq!(decode_engine_command(&bytes).expect("decode"), command);
        }
    }
}
//#endregion 🧪️WireBaselineDump
