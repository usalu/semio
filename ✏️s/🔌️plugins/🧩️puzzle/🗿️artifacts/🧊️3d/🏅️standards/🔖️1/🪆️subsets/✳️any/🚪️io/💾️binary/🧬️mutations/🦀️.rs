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

use crate::standards::v1::subsets::any::schema::mutations::Puzzle3dMutation;
use crate::Puzzle3dSnapshot;
use protocol::OpBinary;


/// 📦️ Encodes a `Puzzle3dMutation` to its binary command form.
pub fn encode_op(operation: &Puzzle3dMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `Puzzle3dMutation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<Puzzle3dMutation, protocol::ProtocolError> {
    Puzzle3dMutation::decode_op(bytes)
}

//#region 🔖️Store









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
use crate::standards::v1::subsets::any::schema::{Puzzle3dEngineCommand, Puzzle3dEngineOutcome};

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
use crate::editor::puzzle3d::snapshot::Puzzle3dPlaySnapshot;
use crate::standards::v1::subsets::any::schema::mutations::*;
use crate::standards::v1::subsets::any::schema::diff::Puzzle3dDiff;
use crate::Puzzle3dSnapshot;
use protocol::{Mutation, MutationDiff};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::standards::v1::subsets::any::schema::mutations::add_object_vortex::mutation::{add_object_vortex, AddObjectVortex};
use crate::standards::v1::subsets::any::schema::mutations::change_domain::mutation::{change_domain, ChangeDomain};
use crate::standards::v1::subsets::any::schema::mutations::change_object_anchor::mutation::{change_object_anchor, ChangeObjectAnchor};
use crate::standards::v1::subsets::any::schema::mutations::change_object_hidden::mutation::{change_object_hidden, ChangeObjectHidden};
use crate::standards::v1::subsets::any::schema::mutations::change_object_kind::mutation::{change_object_kind, ChangeObjectKind};
use crate::standards::v1::subsets::any::schema::mutations::change_object_locked::mutation::{change_object_locked, ChangeObjectLocked};
use crate::standards::v1::subsets::any::schema::mutations::change_object_mesh::mutation::{change_object_mesh, ChangeObjectMesh};
use crate::standards::v1::subsets::any::schema::mutations::change_reference_hidden::mutation::{change_reference_hidden, ChangeReferenceHidden};
use crate::standards::v1::subsets::any::schema::mutations::change_reference_locked::mutation::{change_reference_locked, ChangeReferenceLocked};
use crate::standards::v1::subsets::any::schema::mutations::change_target_volume_hidden::mutation::{change_target_volume_hidden, ChangeTargetVolumeHidden};
use crate::standards::v1::subsets::any::schema::mutations::change_target_volume_locked::mutation::{change_target_volume_locked, ChangeTargetVolumeLocked};
use crate::standards::v1::subsets::any::schema::mutations::connect_kind_compatibility::mutation::{connect_kind_compatibility, ConnectKindCompatibility};
use crate::standards::v1::subsets::any::schema::mutations::connect_vortices::mutation::{connect_vortices, ConnectVortices};
use crate::standards::v1::subsets::any::schema::mutations::create_object::mutation::{create_object, CreateObject};
use crate::standards::v1::subsets::any::schema::mutations::create_reference::mutation::{create_reference, CreateReference};
use crate::standards::v1::subsets::any::schema::mutations::create_target_volume::mutation::{create_target_volume, CreateTargetVolume};
use crate::standards::v1::subsets::any::schema::mutations::delete_object::mutation::{delete_object, DeleteObject};
use crate::standards::v1::subsets::any::schema::mutations::delete_reference::mutation::{delete_reference, DeleteReference};
use crate::standards::v1::subsets::any::schema::mutations::delete_target_volume::mutation::{delete_target_volume, DeleteTargetVolume};
use crate::standards::v1::subsets::any::schema::mutations::disconnect_kind_compatibility::mutation::{disconnect_kind_compatibility, DisconnectKindCompatibility};
use crate::standards::v1::subsets::any::schema::mutations::disconnect_vortices::mutation::{disconnect_vortices, DisconnectVortices};
use crate::standards::v1::subsets::any::schema::mutations::drag_selection::mutation::{drag_selection, DragSelection};
use crate::standards::v1::subsets::any::schema::mutations::edit_object_label::mutation::{edit_object_label, EditObjectLabel};
use crate::standards::v1::subsets::any::schema::mutations::move_object::mutation::{move_object, MoveObject};
use crate::standards::v1::subsets::any::schema::mutations::move_reference::mutation::{move_reference, MoveReference};
use crate::standards::v1::subsets::any::schema::mutations::move_target_volume::mutation::{move_target_volume, MoveTargetVolume};
use crate::standards::v1::subsets::any::schema::mutations::remove_object_vortex::mutation::{remove_object_vortex, RemoveObjectVortex};
use crate::standards::v1::subsets::any::schema::mutations::replace_attraction_geometry::mutation::{replace_attraction_geometry, ReplaceAttractionGeometry};
use crate::standards::v1::subsets::any::schema::mutations::replace_kind_catalogs::mutation::{replace_kind_catalogs, ReplaceKindCatalogs};
use crate::standards::v1::subsets::any::schema::mutations::replace_object_vortex::mutation::{replace_object_vortex, ReplaceObjectVortex};
use crate::standards::v1::subsets::any::schema::mutations::replace_reference_source::mutation::{replace_reference_source, ReplaceReferenceSource};
use crate::standards::v1::subsets::any::schema::mutations::resize_reference::mutation::{resize_reference, ResizeReference};
use crate::standards::v1::subsets::any::schema::mutations::rotate_object::mutation::{rotate_object, RotateObject};
use crate::standards::v1::subsets::any::schema::mutations::rotate_selection::mutation::{rotate_selection, RotateSelection};
use crate::standards::v1::subsets::any::schema::mutations::rotate_target_volume::mutation::{rotate_target_volume, RotateTargetVolume};
use crate::standards::v1::subsets::any::schema::mutations::scale_object::mutation::{scale_object, ScaleObject};
use crate::standards::v1::subsets::any::schema::mutations::scale_selection::mutation::{scale_selection, ScaleSelection};
use crate::standards::v1::subsets::any::schema::mutations::scale_target_volume::mutation::{scale_target_volume, ScaleTargetVolume};

/// 📦️ Packs through the typed authority, so the play kind shares `Puzzle3dSnapshot`'s derived record
/// layout and pack-schema identity.
impl store::ArtifactPack for Puzzle3dPlaySnapshot {
    fn sqlite_snapshot_codec() -> Option<store::ArtifactSqliteSnapshotCodec> {Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec())}
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        self.typed().encode_pack_with(options)
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        <Puzzle3dSnapshot as store::ArtifactPack>::decode_pack_with(bytes, options).map(Self::from_typed)
    }

    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        <Puzzle3dSnapshot as store::ArtifactPack>::record_spec()
    }
}
}
pub use mutations_codec::*;

mod native_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::mutations::{inverse_puzzle3d_mutation,Puzzle3dMutation};
use crate::editor::puzzle3d::snapshot::Puzzle3dPlaySnapshot;
pub use mutations_codec::*;

impl protocol::OpBinary for Puzzle3dMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_tagged_op(include_str!("📡️.protocol.semio"), self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_tagged_op(include_str!("📡️.protocol.semio"), bytes)
    }
}
}
