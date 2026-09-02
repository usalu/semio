//! 📡️ Puzzle 3d artifact — the state-patch-representation codec: `encode_op`/`decode_op` for
//! `Puzzle3dMutation`'s binary wire form, `encode_engine_command`/`decode_engine_command` for the
//! headless engine's own `Puzzle3dEngineCommand` envelope, plus the `ArtifactEnvelope`/
//! `ArtifactStore` aliases every puzzle-3d host binds. Renamed from the pre-consolidation
//! `📡️protocol` module; both wire formats are unchanged (`dsl::DslOps`'s generated `OpBinary`).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::artifacts::puzzle3d::schema::mutations::text::Puzzle3dMutation;
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;
use protocol::OpBinary;
use store::{ArtifactEnvelope, ArtifactStore};

/// 📦️ Encodes a `Puzzle3dMutation` to its binary command form.
pub fn encode_op(operation: &Puzzle3dMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `Puzzle3dMutation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<Puzzle3dMutation, protocol::ProtocolError> {
    Puzzle3dMutation::decode_op(bytes)
}

//#region 🔖️Store
pub type Puzzle3dEnvelope = ArtifactEnvelope<Puzzle3dSnapshot, Puzzle3dMutation>;
pub type Puzzle3dStore = ArtifactStore<Puzzle3dSnapshot, Puzzle3dMutation>;
//#endregion 🔖️Store

//#region 🔖️Puzzle3dEngineCommand
/// 🎯️ Re-exports the puzzle 3d precompute command envelope. `#[derive(dsl::DslEnum)]` is applied
/// where the type is declared, in `🧬️schema/🦀️component.rs` — not here — because the derive's
/// generated code needs `SceneConfig`/`BrushPlacePayload` (types that file owns) by value;
/// re-exporting it here plus wrapping `encode_op`/`decode_op` mirrors exactly how `Puzzle3dMutation`
/// (declared in `🔧️op`) is surfaced above. Relocated off the former `⚙️engine` (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — the stateful session that dispatches this
/// envelope now lives app-side, at `crate::editor::puzzle3d::precompute`, but the envelope itself is
/// pure data and stays schema-side.
pub use crate::artifacts::puzzle3d::schema::{Puzzle3dEngineCommand, Puzzle3dEngineOutcome};

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
        use crate::artifacts::puzzle3d::schema::empty_puzzle3d_snapshot;
        use crate::artifacts::puzzle3d::{Puzzle3dObject, PUZZLE_3D_SCHEMA};
        use store::{create_document_envelope, ArtifactCommand};

        let mut store = semio_framework::io::resolve_ready(Puzzle3dStore::new(create_document_envelope(PUZZLE_3D_SCHEMA, "puzzle3d", empty_puzzle3d_snapshot(), None))).expect("store");
        semio_framework::io::resolve_ready(store.dispatch(ArtifactCommand::Apply {
            mutations: vec![crate::artifacts::puzzle3d::mutations::create_object(
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
    pub(crate) fn sample_scene_config() -> crate::artifacts::puzzle3d::schema::SceneConfig {
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
}
//#endregion 🧪️Tests

//#region 🔒️WireFormatGuard
#[cfg(test)]
mod wire_format_guard {
    //! 🔒️ Byte-level `OpBinary` round-trip guard for the semantic-mutations-overhaul document
    //! vocabulary (ticket `26/08/12/SEMANTIC-MUTATIONS-OVERHAUL`) plus the (unrelated, unchanged)
    //! frozen headless-engine-command wire below. The pre-overhaul whole-record-upsert /
    //! whole-document-replace wire bytes this guard used to freeze for `Puzzle3dMutation` no
    //! longer exist — that vocabulary is banned outright, not preserved — so the document-mutation
    //! half now asserts the NEW operations' `OpText`/`OpBinary` round-trip instead of pinning byte
    //! literals for a wire shape this ticket deliberately changed.
    use super::*;
    use crate::artifacts::puzzle3d as puzzle_3d;
    use crate::artifacts::puzzle3d::mutations::{change_object_anchor, connect_vortices, create_object, delete_object};
    use protocol::OpText;

    fn ops() -> Vec<Puzzle3dMutation> {
        let object = puzzle_3d::Puzzle3dObject {
            id: "o1".into(),
            label: Some("L".into()),
            object_kind: Some("Capsule".into()),
            anchor: puzzle_3d::Puzzle3dObjectAnchor::Fixed,
            origin: [1.0, 2.0, 3.0],
            orientation: Some([0.0, 0.0, 0.0, 1.0]),
            scale: Some(puzzle_3d::Puzzle3dScale::Vec3([2.0, 3.0, 4.0])),
            mesh_url: Some("/m.glb".into()),
            vortices: Vec::new(),
            hidden: false,
            locked: true,
        };
        vec![
            create_object(object, Some(0)),
            change_object_anchor("o1".into(), puzzle_3d::Puzzle3dObjectAnchor::Derived),
            delete_object("o1".into()),
            connect_vortices("a1".into(), "o1:v0".into(), "o2:v0".into(), 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0),
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

    /// 🔒️ Same frozen capture for the headless engine-command codec.
    const PRE_MIGRATION_ENGINE_COMMAND_WIRE: &[&str] = &[
        "apply-fill-count count=7 | 7 | 01020001000407",
        "compose-fill-display count=9 | 7 | 01030001000409",
        "update-kind-weights object-weights={ Host=0.5 } vortex-weights={ } | 26 | 01040104486f737402001001060005000000000000e03f011000",
        "brush-preview vortex-full-id=\"host:v0\" candidate-index=2 | 18 | 01050107686f73743a763002000600010402",
    ];

    /// ⚖️ Every document-mutation operation prints, parses, encodes, and decodes back to an equal
    /// value.
    #[test]
    fn operations_round_trip_text_and_binary() {
        let operations = ops();
        assert!(!operations.is_empty());
        for operation in &operations {
            semio_framework_os_kernel::os_store::test_support::assert_op_text_binary_equivalence(operation);
            let line = operation.print_op();
            assert_eq!(&Puzzle3dMutation::parse_op(&line).expect("parse_op"), operation);
            let bytes = encode_op(operation).expect("encode");
            assert_eq!(&decode_op(&bytes).expect("decode"), operation);
        }
        let created = operations
            .iter()
            .find_map(|op| match op {
                Puzzle3dMutation::CreateObject(payload) => Some(payload),
                _ => None,
            })
            .expect("create-object covered");
        assert_eq!(created.object.anchor, puzzle_3d::Puzzle3dObjectAnchor::Fixed);
        let connected = operations
            .iter()
            .find_map(|op| match op {
                Puzzle3dMutation::ConnectVortices(payload) => Some(payload),
                _ => None,
            })
            .expect("connect-vortices covered");
        assert_eq!(connected.x, 7.0);
        assert_eq!(connected.y, 8.0);
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
