//! 📡️ Puzzle 2d artifact — the state-patch-representation codec: `encode_op`/`decode_op` for
//! `Puzzle2dMutation`'s binary wire form, plus the `ArtifactEnvelope`/`ArtifactStore` aliases every
//! puzzle-2d host binds. Renamed from the pre-consolidation `📡️protocol` module; the wire format is
//! unchanged (`dsl::DslOps`'s generated `OpBinary`).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation;
use crate::Puzzle2dSnapshot;
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
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🔒️WireFormatGuard
#[cfg(test)]
#[path = "🧪️tests/🔬️wire-format-guard/🦀️.rs"]
mod wire_format_guard;
//#endregion 🔒️WireFormatGuard

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::editor::puzzle2d::snapshot::Puzzle2dPlaySnapshot;
use crate::standards::v1::subsets::any::schema::mutations::*;
use crate::standards::v1::subsets::any::schema::diff::Puzzle2dDiff;
use crate::Puzzle2dSnapshot;
use protocol::{Mutation, MutationDiff};
use serde_json::Value;
use crate::standards::v1::subsets::any::schema::mutations::add_node_handle::{add_node_handle, AddNodeHandle};
use crate::standards::v1::subsets::any::schema::mutations::change_edge_kind::{change_edge_kind, ChangeEdgeKind};
use crate::standards::v1::subsets::any::schema::mutations::change_edge_locked::{change_edge_locked, ChangeEdgeLocked};
use crate::standards::v1::subsets::any::schema::mutations::change_edge_tips::{change_edge_tips, ChangeEdgeTips};
use crate::standards::v1::subsets::any::schema::mutations::change_edge_visible::{change_edge_visible, ChangeEdgeVisible};
use crate::standards::v1::subsets::any::schema::mutations::change_manifest_id::{change_manifest_id, ChangeManifestId};
use crate::standards::v1::subsets::any::schema::mutations::change_node_anchor::{change_node_anchor, ChangeNodeAnchor};
use crate::standards::v1::subsets::any::schema::mutations::change_node_icon::{change_node_icon, ChangeNodeIcon};
use crate::standards::v1::subsets::any::schema::mutations::change_node_kind::{change_node_kind, ChangeNodeKind};
use crate::standards::v1::subsets::any::schema::mutations::change_node_locked::{change_node_locked, ChangeNodeLocked};
use crate::standards::v1::subsets::any::schema::mutations::change_node_root::{change_node_root, ChangeNodeRoot};
use crate::standards::v1::subsets::any::schema::mutations::change_node_visible::{change_node_visible, ChangeNodeVisible};
use crate::standards::v1::subsets::any::schema::mutations::change_target_region_hidden::{change_target_region_hidden, ChangeTargetRegionHidden};
use crate::standards::v1::subsets::any::schema::mutations::change_target_region_locked::{change_target_region_locked, ChangeTargetRegionLocked};
use crate::standards::v1::subsets::any::schema::mutations::connect_handles::{connect_handles, connect_handles_in_proximity, ConnectHandles};
use crate::standards::v1::subsets::any::schema::mutations::connect_kind_compatibility::{connect_kind_compatibility, ConnectKindCompatibility};
use crate::standards::v1::subsets::any::schema::mutations::create_node::{create_node, CreateNode};
use crate::standards::v1::subsets::any::schema::mutations::create_target_region::{create_target_region, CreateTargetRegion};
use crate::standards::v1::subsets::any::schema::mutations::delete_node::{delete_node, DeleteNode};
use crate::standards::v1::subsets::any::schema::mutations::delete_target_region::{delete_target_region, DeleteTargetRegion};
use crate::standards::v1::subsets::any::schema::mutations::drag_selection::{drag_selection, DragSelection};
use crate::standards::v1::subsets::any::schema::mutations::disconnect_handles::{disconnect_handles, DisconnectHandles};
use crate::standards::v1::subsets::any::schema::mutations::disconnect_kind_compatibility::{disconnect_kind_compatibility, DisconnectKindCompatibility};
use crate::standards::v1::subsets::any::schema::mutations::edit_node_text::{edit_node_text, EditNodeText};
use crate::standards::v1::subsets::any::schema::mutations::edit_target_region_label::{edit_target_region_label, EditTargetRegionLabel};
use crate::standards::v1::subsets::any::schema::mutations::move_node::{move_node, MoveNode};
use crate::standards::v1::subsets::any::schema::mutations::move_target_region::{move_target_region, MoveTargetRegion};
use crate::standards::v1::subsets::any::schema::mutations::remove_node_handle::{remove_node_handle, RemoveNodeHandle};
use crate::standards::v1::subsets::any::schema::mutations::replace_edge_geometry::{replace_edge_geometry, ReplaceEdgeGeometry};
use crate::standards::v1::subsets::any::schema::mutations::replace_kind_catalogs::{replace_kind_catalogs, ReplaceKindCatalogs};
use crate::standards::v1::subsets::any::schema::mutations::replace_node_geometry::{replace_node_geometry, ReplaceNodeGeometry};
use crate::standards::v1::subsets::any::schema::mutations::replace_node_handle::{replace_node_handle, ReplaceNodeHandle};
use crate::standards::v1::subsets::any::schema::mutations::resize_target_region::{resize_target_region, ResizeTargetRegion};
use crate::standards::v1::subsets::any::schema::mutations::rotate_selection::{rotate_selection, RotateSelection};
use crate::standards::v1::subsets::any::schema::mutations::scale_node::{scale_node, ScaleNode};
use crate::standards::v1::subsets::any::schema::mutations::scale_selection::{scale_selection, ScaleSelection};

/// 📦️ Packs through the typed authority, so the play kind shares `Puzzle2dSnapshot`'s derived record
/// layout and pack-schema identity.
impl store::ArtifactPack for Puzzle2dPlaySnapshot {
    fn sqlite_snapshot_codec() -> Option<store::ArtifactSqliteSnapshotCodec> {
        Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec())
    }

    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        self.typed().encode_pack_with(options)
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        <Puzzle2dSnapshot as store::ArtifactPack>::decode_pack_with(bytes, options).map(Self::from_typed)
    }

    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        <Puzzle2dSnapshot as store::ArtifactPack>::record_spec()
    }
}
}
pub use mutations_codec::*;

mod native_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::mutations::{inverse_puzzle2d_mutation,Puzzle2dMutation};
use crate::editor::puzzle2d::snapshot::Puzzle2dPlaySnapshot;
pub use mutations_codec::*;

impl protocol::OpBinary for Puzzle2dMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_tagged_op(include_str!("📡️.protocol.semio"), self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_tagged_op(include_str!("📡️.protocol.semio"), bytes)
    }
}
}
