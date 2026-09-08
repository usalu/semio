
//! 🔒️ Byte-level `OpBinary` round-trip guard for the semantic-mutations-overhaul document
//! vocabulary (ticket `26/08/12/SEMANTIC-MUTATIONS-OVERHAUL`) plus the (unrelated, unchanged)
//! frozen headless-engine-command wire below. The pre-overhaul whole-record-upsert /
//! whole-document-replace wire bytes this guard used to freeze for `Puzzle3dMutation` no
//! longer exist — that vocabulary is banned outright, not preserved — so the document-mutation
//! half now asserts the NEW operations' `OpText`/`OpBinary` round-trip instead of pinning byte
//! literals for a wire shape this ticket deliberately changed.
use super::*;
use crate as puzzle_3d;
use crate::standards::v1::subsets::any::schema::mutations::{change_object_anchor, connect_vortices, create_object, delete_object};
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
    vec![create_object(object, Some(0)), change_object_anchor("o1".into(), puzzle_3d::Puzzle3dObjectAnchor::Derived), delete_object("o1".into()), connect_vortices("a1".into(), "o1:v0".into(), "o2:v0".into(), 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0)]
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
