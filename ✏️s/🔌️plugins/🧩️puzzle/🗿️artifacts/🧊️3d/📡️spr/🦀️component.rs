//! 📡️ Puzzle 3d artifact — the state-patch-representation codec: `encode_op`/`decode_op` for
//! `Puzzle3dOperation`'s binary wire form, `encode_engine_command`/`decode_engine_command` for the
//! headless engine's own `Puzzle3dEngineCommand` envelope, plus the `DocumentEnvelope`/
//! `DocumentStore` aliases every puzzle-3d host binds. Renamed from the pre-consolidation
//! `📡️protocol` module; both wire formats are unchanged (`dsl::DslOps`'s generated `OpBinary`).


//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol


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

//#region 🔒️WireFormatGuard
#[cfg(test)]
mod wire_format_guard {
    //! 🔒️ The permanent byte-level regression guard for this artifact's spr codec, frozen from the
    //! pre-consolidation `📡️protocol` crate (master ticket
    //! `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`, TEMPLATE.md §0.4/§7).
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
        let document = Puzzle3dProjection::default();
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

    fn engine_commands() -> Vec<Puzzle3dEngineCommand> {
        let mut object_weights = std::collections::BTreeMap::new();
        object_weights.insert("Host".to_string(), 0.5);
        vec![
            Puzzle3dEngineCommand::ApplyFillCount { count: 7 },
            Puzzle3dEngineCommand::ComposeFillDisplay { count: 9 },
            Puzzle3dEngineCommand::UpdateKindWeights { object_weights, vortex_weights: std::collections::BTreeMap::new() },
            Puzzle3dEngineCommand::BrushPreview { vortex_full_id: "host:v0".to_string(), candidate_index: 2 },
        ]
    }

    /// 🔒️ The exact `print_op | byte-length | hex` of every operation row, captured from the
    /// pre-consolidation `📡️protocol` crate BEFORE this plugin was merged into one crate. A
    /// round-trip law is self-consistent and would happily pass on a silently changed format;
    /// only these frozen bytes prove the wire did not move.
    const PRE_MIGRATION_OPERATION_WIRE: &[&str] = &[
        "setObject index=0 object { id=o1 label=L object-kind=Capsule origin=@1,2,3 orientation=0,0,0,1 mesh-url=\"/m.glb\" hidden=false locked=true scale=[ 2 3 4 ] vortices=[ id=v0 vortex-kind=k position=@0,0,0 direction=^0,0,1 radius=3 hidden=false locked=false ] } | 220 | 010006062f6d2e676c620743617073756c65014c016b026f3102763002000400010e0d0a000604010602020601031503000000000000f03f00000000000000400000000000000840041504000000000000000000000000000000000000000000000000000000000000f03f051503000000000000004000000000000008400000000000001040060600070c010d0700060501060303150300000000000000000000000000000000000000000000000004150300000000000000000000000000000000000000000000f03f050500000000000008400601070108010902",
        "removeObject id=o1 | 10 | 010101026f3101000600",
        "setAttraction index=1 attraction { id=a1 attracting=\"o1:v0\" attracted=\"o2:v0\" gap=1 shift=2 rise=3 rotation=4 turn=5 tilt=6 } | 95 | 010203026131056f313a7630056f323a763002000401010e0d090006000106010206020305000000000000f03f0405000000000000004005050000000000000840060500000000000010400705000000000000144008050000000000001840",
        "removeAttraction id=a1 | 10 | 01030102613101000600",
        "setTargetVolume index=2 target-volume { id=t1 origin=@0,1,2 orientation=0,0,0,1 hidden=false locked=false scale=[ 5 ] } | 94 | 01040102743102000402010e0d060006000115030000000000000000000000000000f03f0000000000000040021504000000000000000000000000000000000000000000000000000000000000f03f031501000000000000144004010501",
        "removeTargetVolume id=t1 | 10 | 01050102743101000600",
        "setReference index=3 reference { id=r1 origin=@0,0,0 width-world=4m locked=false hidden=false source=url=\"/u.png\" media-kind=image } | 80 | 010603062f752e706e6705696d61676502723102000403010e0d06000602010d020006000106010215030000000000000000000000000000000000000000000000000305000000000000104004010501",
        "removeReference id=r1 | 10 | 01070102723101000600",
        "setMeta meta { kind-compatibility [source:REF target:REF bidirectional:BOOL important:BOOL specificity:TEXT] { a b true false vortex } } | 43 | 0108030161016206766f7274657801000e0d01011401050000050001000501020001010300010004000502",
        "setDocument document { schema=puzzle.3d domain=architecture meta { kind-compatibility [source:REF target:REF bidirectional:BOOL important:BOOL specificity:TEXT] { } } objects [id:TEXT label:TEXT object-kind:REF origin:CRD orientation:TUPLE scale:LIST mesh-url:TEXT vortices:LIST hidden:BOOL locked:BOOL] { } attractions [id:TEXT attracting:TEXT attracted:TEXT gap:NUM shift:NUM rise:NUM rotation:NUM turn:NUM tilt:NUM] { } target-volumes [id:TEXT origin:CRD orientation:TUPLE scale:LIST hidden:BOOL locked:BOOL] { } references [id:TEXT source:REC origin:CRD width-world:QTY locked:BOOL hidden:BOOL] { } } | 169 | 0109020c6172636869746563747572650970757a7a6c652e336401000e0d07000601010600020e0d01011400050000050100050200010300010400050314000a000005010005020005030000040000050000060005070000080001090001041400090000050100050200050300040400040500040600040700040800040514000600000501000002000003000004000105000106140006000005010000020000030004040001050001",
    ];

    /// 🔒️ Same frozen capture for the headless engine-command codec.
    const PRE_MIGRATION_ENGINE_COMMAND_WIRE: &[&str] = &[
        "apply-fill-count count=7 | 7 | 01020001000407",
        "compose-fill-display count=9 | 7 | 01030001000409",
        "update-kind-weights object-weights={ Host=0.5 } vortex-weights={ } | 26 | 01040104486f737402001001060005000000000000e03f011000",
        "brush-preview vortex-full-id=\"host:v0\" candidate-index=2 | 18 | 01050107686f73743a763002000600010402",
    ];

    /// ⚖️ Every operation row still prints and encodes to its pre-migration bytes, and still
    /// decodes back to the same value.
    #[test]
    fn operation_rows_keep_their_pre_migration_wire_bytes() {
        let operations = ops();
        assert_eq!(operations.len(), PRE_MIGRATION_OPERATION_WIRE.len(), "every operation variant must be covered by the frozen wire table");
        for (operation, expected) in operations.iter().zip(PRE_MIGRATION_OPERATION_WIRE) {
            let bytes = encode_op(operation).expect("encode");
            let hex: String = bytes.iter().map(|byte| format!("{byte:02x}")).collect();
            assert_eq!(&format!("{} | {} | {hex}", operation.print_op(), bytes.len()), expected);
            assert_eq!(&decode_op(&bytes).expect("decode"), operation);
        }
    }

    /// ⚖️ Same law for the engine-command codec.
    #[test]
    fn engine_command_rows_keep_their_pre_migration_wire_bytes() {
        let commands = engine_commands();
        assert_eq!(commands.len(), PRE_MIGRATION_ENGINE_COMMAND_WIRE.len(), "every engine-command variant covered here must be in the frozen wire table");
        for (command, expected) in commands.iter().zip(PRE_MIGRATION_ENGINE_COMMAND_WIRE) {
            let bytes = encode_engine_command(command).expect("encode");
            let hex: String = bytes.iter().map(|byte| format!("{byte:02x}")).collect();
            assert_eq!(&format!("{} | {} | {hex}", command.print_op(), bytes.len()), expected);
            assert_eq!(&decode_engine_command(&bytes).expect("decode"), command);
        }
    }
}
//#endregion 🔒️WireFormatGuard
