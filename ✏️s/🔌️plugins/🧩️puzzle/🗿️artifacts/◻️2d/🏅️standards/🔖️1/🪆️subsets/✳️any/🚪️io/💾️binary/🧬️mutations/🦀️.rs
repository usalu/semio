//! `Puzzle2dMutation`'s binary wire form, plus the `ArtifactEnvelope`/`ArtifactStore` aliases every
//! 📡️ Puzzle 2d artifact — the state-patch-representation codec: `encode_op`/`decode_op` for
//! puzzle-2d host binds. Renamed from the pre-consolidation `📡️protocol` module; the wire format is
//! unchanged (`dsl::DslOps`'s generated `OpBinary`).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation;
#[cfg(test)] use crate::Puzzle2dSnapshot;
use protocol::OpBinary;


/// 📦️ Encodes a `Puzzle2dMutation` to its binary command form.
pub fn encode_op(operation: &Puzzle2dMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `Puzzle2dMutation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<Puzzle2dMutation, protocol::ProtocolError> {
    Puzzle2dMutation::decode_op(bytes)
}

//#region 🔖️Store









//#endregion 🔖️Store

//#region 🧪️Tests

//#endregion 🧪️Tests

//#region 🔒️WireFormatGuard
#[cfg(test)]
#[path = "🧪️tests/🔬️wire-format-guard/🦀️.rs"]
mod wire_format_guard;
//#endregion 🔒️WireFormatGuard

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::Puzzle2dPlaySnapshot;
use crate::standards::v1::subsets::any::schema::mutations::*;
use crate::standards::v1::subsets::any::schema::diff::Puzzle2dDiff;
use crate::Puzzle2dSnapshot;
use protocol::Mutation;
use protocol::MutationDiff;
use serde_json::Value;
use crate::standards::v1::subsets::any::schema::mutations::add_node_handle::add_node_handle;
use crate::standards::v1::subsets::any::schema::mutations::add_node_handle::AddNodeHandle;
use crate::standards::v1::subsets::any::schema::mutations::change_edge_kind::change_edge_kind;
use crate::standards::v1::subsets::any::schema::mutations::change_edge_kind::ChangeEdgeKind;
use crate::standards::v1::subsets::any::schema::mutations::change_edge_locked::change_edge_locked;
use crate::standards::v1::subsets::any::schema::mutations::change_edge_locked::ChangeEdgeLocked;
use crate::standards::v1::subsets::any::schema::mutations::change_edge_tips::change_edge_tips;
use crate::standards::v1::subsets::any::schema::mutations::change_edge_tips::ChangeEdgeTips;
use crate::standards::v1::subsets::any::schema::mutations::change_edge_visible::change_edge_visible;
use crate::standards::v1::subsets::any::schema::mutations::change_edge_visible::ChangeEdgeVisible;
use crate::standards::v1::subsets::any::schema::mutations::change_manifest_id::change_manifest_id;
use crate::standards::v1::subsets::any::schema::mutations::change_manifest_id::ChangeManifestId;
use crate::standards::v1::subsets::any::schema::mutations::change_node_anchor::change_node_anchor;
use crate::standards::v1::subsets::any::schema::mutations::change_node_anchor::ChangeNodeAnchor;
use crate::standards::v1::subsets::any::schema::mutations::change_node_icon::change_node_icon;
use crate::standards::v1::subsets::any::schema::mutations::change_node_icon::ChangeNodeIcon;
use crate::standards::v1::subsets::any::schema::mutations::change_node_kind::change_node_kind;
use crate::standards::v1::subsets::any::schema::mutations::change_node_kind::ChangeNodeKind;
use crate::standards::v1::subsets::any::schema::mutations::change_node_locked::change_node_locked;
use crate::standards::v1::subsets::any::schema::mutations::change_node_locked::ChangeNodeLocked;
use crate::standards::v1::subsets::any::schema::mutations::change_node_root::change_node_root;
use crate::standards::v1::subsets::any::schema::mutations::change_node_root::ChangeNodeRoot;
use crate::standards::v1::subsets::any::schema::mutations::change_node_visible::change_node_visible;
use crate::standards::v1::subsets::any::schema::mutations::change_node_visible::ChangeNodeVisible;
use crate::standards::v1::subsets::any::schema::mutations::change_target_region_hidden::change_target_region_hidden;
use crate::standards::v1::subsets::any::schema::mutations::change_target_region_hidden::ChangeTargetRegionHidden;
use crate::standards::v1::subsets::any::schema::mutations::change_target_region_locked::change_target_region_locked;
use crate::standards::v1::subsets::any::schema::mutations::change_target_region_locked::ChangeTargetRegionLocked;
use crate::standards::v1::subsets::any::schema::mutations::connect_handles::connect_handles;
use crate::standards::v1::subsets::any::schema::mutations::connect_handles::connect_handles_in_proximity;
use crate::standards::v1::subsets::any::schema::mutations::connect_handles::ConnectHandles;
use crate::standards::v1::subsets::any::schema::mutations::connect_kind_compatibility::connect_kind_compatibility;
use crate::standards::v1::subsets::any::schema::mutations::connect_kind_compatibility::ConnectKindCompatibility;
use crate::standards::v1::subsets::any::schema::mutations::create_node::create_node;
use crate::standards::v1::subsets::any::schema::mutations::create_node::CreateNode;
use crate::standards::v1::subsets::any::schema::mutations::create_target_region::create_target_region;
use crate::standards::v1::subsets::any::schema::mutations::create_target_region::CreateTargetRegion;
use crate::standards::v1::subsets::any::schema::mutations::delete_node::delete_node;
use crate::standards::v1::subsets::any::schema::mutations::delete_node::DeleteNode;
use crate::standards::v1::subsets::any::schema::mutations::delete_target_region::delete_target_region;
use crate::standards::v1::subsets::any::schema::mutations::delete_target_region::DeleteTargetRegion;
use crate::standards::v1::subsets::any::schema::mutations::drag_selection::drag_selection;
use crate::standards::v1::subsets::any::schema::mutations::drag_selection::DragSelection;
use crate::standards::v1::subsets::any::schema::mutations::disconnect_handles::disconnect_handles;
use crate::standards::v1::subsets::any::schema::mutations::disconnect_handles::DisconnectHandles;
use crate::standards::v1::subsets::any::schema::mutations::disconnect_kind_compatibility::disconnect_kind_compatibility;
use crate::standards::v1::subsets::any::schema::mutations::disconnect_kind_compatibility::DisconnectKindCompatibility;
use crate::standards::v1::subsets::any::schema::mutations::edit_node_text::edit_node_text;
use crate::standards::v1::subsets::any::schema::mutations::edit_node_text::EditNodeText;
use crate::standards::v1::subsets::any::schema::mutations::edit_target_region_label::edit_target_region_label;
use crate::standards::v1::subsets::any::schema::mutations::edit_target_region_label::EditTargetRegionLabel;
use crate::standards::v1::subsets::any::schema::mutations::move_node::move_node;
use crate::standards::v1::subsets::any::schema::mutations::move_node::MoveNode;
use crate::standards::v1::subsets::any::schema::mutations::move_target_region::move_target_region;
use crate::standards::v1::subsets::any::schema::mutations::move_target_region::MoveTargetRegion;
use crate::standards::v1::subsets::any::schema::mutations::remove_node_handle::remove_node_handle;
use crate::standards::v1::subsets::any::schema::mutations::remove_node_handle::RemoveNodeHandle;
use crate::standards::v1::subsets::any::schema::mutations::replace_edge_geometry::replace_edge_geometry;
use crate::standards::v1::subsets::any::schema::mutations::replace_edge_geometry::ReplaceEdgeGeometry;
use crate::standards::v1::subsets::any::schema::mutations::replace_kind_catalogs::replace_kind_catalogs;
use crate::standards::v1::subsets::any::schema::mutations::replace_kind_catalogs::ReplaceKindCatalogs;
use crate::standards::v1::subsets::any::schema::mutations::replace_node_geometry::replace_node_geometry;
use crate::standards::v1::subsets::any::schema::mutations::replace_node_geometry::ReplaceNodeGeometry;
use crate::standards::v1::subsets::any::schema::mutations::replace_node_handle::replace_node_handle;
use crate::standards::v1::subsets::any::schema::mutations::replace_node_handle::ReplaceNodeHandle;
use crate::standards::v1::subsets::any::schema::mutations::resize_target_region::resize_target_region;
use crate::standards::v1::subsets::any::schema::mutations::resize_target_region::ResizeTargetRegion;
use crate::standards::v1::subsets::any::schema::mutations::rotate_selection::rotate_selection;
use crate::standards::v1::subsets::any::schema::mutations::rotate_selection::RotateSelection;
use crate::standards::v1::subsets::any::schema::mutations::scale_node::scale_node;
use crate::standards::v1::subsets::any::schema::mutations::scale_node::ScaleNode;
use crate::standards::v1::subsets::any::schema::mutations::scale_selection::scale_selection;
use crate::standards::v1::subsets::any::schema::mutations::scale_selection::ScaleSelection;

}
#[cfg(test)] pub use mutations_codec::*;

mod native_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::mutations::inverse_puzzle2d_mutation;
use crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation;
use crate::Puzzle2dPlaySnapshot;
pub use mutations_codec::*;

impl OpBinary for Puzzle2dMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_tagged_op(include_str!("📡️.protocol.semio"), self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_tagged_op(include_str!("📡️.protocol.semio"), bytes)
    }
}
}

/// 🫳️ Borrows the original concrete leaf of a mutation and its canonical protocol tag before publication: the authoring wire source the artifact store requires.
pub fn prepared_operation_wire_source(operation: &crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation) -> Option<store::ArtifactPreparedOperationSource<'_>> {
    fn source<T: semio_framework_dsl_record::DslField + semio_framework_dsl_record::BorrowedDslRecord>(value: &T) -> Option<store::ArtifactPreparedOperationSource<'_>> {
        let spec = T::RECORD;
        let keyword = spec.keyword?;
        let tag = dsl::protocol_record::records(COMPONENT_PROTOCOL_SEMIO).find(|(kind, _)| *kind == keyword)?.1;
        Some(store::ArtifactPreparedOperationSource::Pack { tag, body: value, spec })
    }
    match operation {
        crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation::CreateNode(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation::DeleteNode(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation::MoveNode(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation::ReplaceNodeGeometry(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation::ChangeNodeKind(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation::EditNodeText(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation::ChangeNodeIcon(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation::ScaleNode(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation::ChangeNodeVisible(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation::ChangeNodeLocked(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation::ChangeNodeRoot(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation::ChangeNodeAnchor(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation::AddNodeHandle(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation::RemoveNodeHandle(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation::ReplaceNodeHandle(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation::ConnectHandles(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation::DisconnectHandles(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation::ReplaceEdgeGeometry(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation::ChangeEdgeKind(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation::ChangeEdgeTips(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation::ChangeEdgeVisible(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation::ChangeEdgeLocked(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation::ChangeManifestId(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation::ConnectKindCompatibility(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation::DisconnectKindCompatibility(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation::ReplaceKindCatalogs(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation::CreateTargetRegion(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation::DeleteTargetRegion(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation::MoveTargetRegion(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation::ResizeTargetRegion(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation::EditTargetRegionLabel(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation::ChangeTargetRegionHidden(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation::ChangeTargetRegionLocked(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation::DragSelection(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation::RotateSelection(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation::ScaleSelection(value) => source(value),
    }
}
