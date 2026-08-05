//! ⚖️ Block 3D app — binary command protocol surface + laws (constitutional: protocol).

use block_3d_op::Block3dOperation;
use protocol::OpBinary;
use serde::{Deserialize, Serialize};

/// 📦️ Encodes a `Block3dOperation` to its binary command form.
pub fn encode_op(operation: &Block3dOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `Block3dOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<Block3dOperation, protocol::ProtocolError> {
    Block3dOperation::decode_op(bytes)
}

//#region 🔖️Block3dCommand
/// 🎯️ `Block3dPlayApp::Command` — the sole dispatch surface for block-3d's behavior, one variant per
/// declared manifest action (`block_3d_ui::create_block3d_app`). Mirrors
/// `shooting_protocol::ShootingCommand`'s shape/derive conventions exactly.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum Block3dCommand {
    #[dsl(key = "patchObjectKind")]
    PatchObjectKind { field: String, value: String },
    #[dsl(key = "addRepresentation")]
    AddRepresentation,
    #[dsl(key = "removeRepresentation")]
    RemoveRepresentation { id: String },
    #[dsl(key = "addVortexKind")]
    AddVortexKind,
    #[dsl(key = "removeVortexKind")]
    RemoveVortexKind { id: String },
    #[dsl(key = "addVortex")]
    AddVortex,
    #[dsl(key = "removeVortex")]
    RemoveVortex { id: String },
    #[dsl(key = "setActiveExample")]
    SetActiveExample { id: String },
    #[dsl(key = "edit")]
    Edit { text: String },
    // 👁️ Config-only (was `Block3dPlayApp`'s `RefCell` runtime fields) — emit `config_operations`, never document operations.
    #[dsl(key = "setSelection")]
    SetSelection { ids: Vec<String> },
    #[dsl(key = "setActiveRepresentation")]
    SetActiveRepresentation { representation_id: Option<String> },
    #[dsl(key = "setWindowRepresentations")]
    SetWindowRepresentations { window_id: String, representation_ids: Vec<String> },
    #[dsl(key = "toggleWindowRepresentation")]
    ToggleWindowRepresentation { window_id: String, representation_id: String, visible: bool },
    #[dsl(key = "setWindowArrangement")]
    SetWindowArrangement { window_id: String, arrangement: String },
    #[dsl(key = "setWindowSpacing")]
    SetWindowSpacing { window_id: String, spacing: f64 },
    #[dsl(key = "setActiveUtility")]
    SetActiveUtility { window_id: String, utility_id: String },
    #[dsl(key = "setBrushVortexKind")]
    SetBrushVortexKind { vortex_kind_id: Option<String> },
    #[dsl(key = "setBrushRadius")]
    SetBrushRadius { radius: f64 },
    #[dsl(key = "setBrushFlip")]
    SetBrushFlip { flip: bool },
    #[dsl(key = "hoverSurface")]
    HoverSurface { window_id: String, object_id: String, position: [f64; 3], normal: [f64; 3] },
    #[dsl(key = "leaveSurface")]
    LeaveSurface,
    #[dsl(key = "placeVortex")]
    PlaceVortex { window_id: String, object_id: String, position: [f64; 3], normal: [f64; 3] },
    #[dsl(key = "setCamera")]
    SetCamera { camera: block_shared::BlockCamera3d },
    #[dsl(key = "selectVortex")]
    SelectVortex { full_id: String, merge: bool },
    #[dsl(key = "hoverVortex")]
    HoverVortex { full_id: Option<String> },
    #[dsl(key = "patchRepresentation")]
    PatchRepresentation { id: String, field: String, value: String },
}
//#endregion 🔖️Block3dCommand

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    // 🧪️ [DEBUG] TEMP wire baseline dump — TEMPLATE.md §0.4. Remove after capturing.
    #[test]
    fn dump_wire_baseline_block3d() {
        let commands = vec![
            Block3dCommand::PatchObjectKind { field: "name".into(), value: "x".into() },
            Block3dCommand::AddRepresentation,
            Block3dCommand::RemoveRepresentation { id: "r0".into() },
            Block3dCommand::AddVortexKind,
            Block3dCommand::RemoveVortexKind { id: "v0".into() },
            Block3dCommand::AddVortex,
            Block3dCommand::RemoveVortex { id: "v0".into() },
            Block3dCommand::SetActiveExample { id: "capsule".into() },
            Block3dCommand::Edit { text: "{}".into() },
            Block3dCommand::SetSelection { ids: vec!["r0".into()] },
            Block3dCommand::SetSelection { ids: vec![] },
            Block3dCommand::SetActiveRepresentation { representation_id: Some("r0".into()) },
            Block3dCommand::SetActiveRepresentation { representation_id: None },
            Block3dCommand::SetWindowRepresentations { window_id: "w0".into(), representation_ids: vec!["r0".into()] },
            Block3dCommand::ToggleWindowRepresentation { window_id: "w0".into(), representation_id: "r0".into(), visible: true },
            Block3dCommand::SetWindowArrangement { window_id: "w0".into(), arrangement: "x".into() },
            Block3dCommand::SetWindowSpacing { window_id: "w0".into(), spacing: 8.0 },
            Block3dCommand::SetActiveUtility { window_id: "w0".into(), utility_id: "select".into() },
            Block3dCommand::SetBrushVortexKind { vortex_kind_id: Some("v0".into()) },
            Block3dCommand::SetBrushVortexKind { vortex_kind_id: None },
            Block3dCommand::SetBrushRadius { radius: 0.3 },
            Block3dCommand::SetBrushFlip { flip: true },
            Block3dCommand::HoverSurface { window_id: "w0".into(), object_id: "r0".into(), position: [0.0, 0.0, 0.0], normal: [0.0, 1.0, 0.0] },
            Block3dCommand::LeaveSurface,
            Block3dCommand::PlaceVortex { window_id: "w0".into(), object_id: "r0".into(), position: [0.0, 0.0, 0.0], normal: [0.0, 1.0, 0.0] },
            Block3dCommand::SetCamera { camera: block_shared::BlockCamera3d::default() },
            Block3dCommand::SelectVortex { full_id: "r0:v0".into(), merge: true },
            Block3dCommand::HoverVortex { full_id: Some("r0:v0".into()) },
            Block3dCommand::HoverVortex { full_id: None },
            Block3dCommand::PatchRepresentation { id: "r0".into(), field: "name".into(), value: "x".into() },
        ];
        for c in &commands {
            let printed = protocol::OpText::print_op(c);
            let bytes = c.encode_op().expect("encode");
            let hex: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
            println!("[DEBUG] {:?} | printed={:?} | len={} | hex={}", c, printed, bytes.len(), hex);
        }
    }

    #[test]
    fn block3d_document_vcs_replays_granular_operations() {
        use block_3d::BLOCK_3D_SCHEMA;
        use block_3d_op::Block3dStore;
        use block_shared::BlockKindIdentity;
        use store::{create_document_envelope, DocumentCommand};

        let mut store = Block3dStore::new(create_document_envelope(BLOCK_3D_SCHEMA, "block3d", block_3d_engine::empty_block3d_definition(), None));
        store
            .dispatch(DocumentCommand::Apply { operations: vec![Block3dOperation::SetObjectKind { object_kind: BlockKindIdentity { id: "o1".into(), name: "o1".into(), label: "O1".into(), ..Default::default() } }], description: None })
            .expect("apply");
        let projection = store.projection().expect("projection");
        assert_eq!(projection.object_kind.id, "o1");
    }

    #[test]
    fn block3d_command_binary_round_trips() {
        let command = Block3dCommand::RemoveVortex { id: "v0".into() };
        let bytes = command.encode_op().expect("encode");
        assert_eq!(Block3dCommand::decode_op(&bytes).expect("decode"), command);
        let selection = Block3dCommand::SetSelection { ids: vec!["representation:r0".into()] };
        let bytes = selection.encode_op().expect("encode");
        assert_eq!(Block3dCommand::decode_op(&bytes).expect("decode"), selection);
    }
}
//#endregion 🧪️Tests
