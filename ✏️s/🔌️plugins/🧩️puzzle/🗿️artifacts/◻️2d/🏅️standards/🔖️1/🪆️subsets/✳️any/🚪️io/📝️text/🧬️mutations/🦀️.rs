//! ⚡️ Puzzle2d artifact — OpText/OpBinary codecs + grammar for `Puzzle2dMutation`.

use crate::standards::v1::subsets::any::schema::mutations::{apply_puzzle2d_mutation, inverse_puzzle2d_mutation, puzzle2d_document_delta_operations, Puzzle2dMutation, Puzzle2dPlaySnapshot};

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️HandcraftedOpCodecs
impl protocol::OpText for Puzzle2dMutation {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = semio_framework_dsl_record::parse(line, &(spec_fn.ordinary)(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Inline })?;
                return <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("unknown mutation line '{line}'")).to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        semio_framework_dsl_record::print(&record, &(spec_fn.ordinary)(), semio_framework_dsl_record::JoinMode::Inline)
    }
}

impl protocol::OpBinary for Puzzle2dMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_tagged_op(include_str!("../../💾️binary/🧬️mutations/📡️.protocol.semio"), self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_tagged_op(include_str!("../../💾️binary/🧬️mutations/📡️.protocol.semio"), bytes)
    }
}
//#endregion 🔖️HandcraftedOpCodecs

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
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

impl store::ArtifactDsl for Puzzle2dPlaySnapshot {
    const EXTENSION: &'static str = "puzzle2d-play";

    fn envelope_id() -> &'static str {
        <Puzzle2dSnapshot as store::ArtifactDsl>::envelope_id()
    }

    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        <Puzzle2dSnapshot as store::ArtifactDsl>::parse_dsl(text).map(Self::from_typed)
    }

    fn print_dsl(&self) -> String {
        <Puzzle2dSnapshot as store::ArtifactDsl>::print_dsl(self.typed())
    }
}
}
pub use mutations_codec::*;

#[allow(unused_imports)]
mod mutations_wire_codec {
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

/// 🪚️ `mutation` with every numeric input rounded to the `precision` its input schema declares for it
/// (`x-semio-ui.precision`, decimal places) — what a gesture commits, so the row label, the history editor's value and
/// its stepper state one number on every host (a host that measures a drag in `f32` records 59.99996 for 60). An input
/// that declares no precision keeps its value.
pub fn puzzle2d_declared_precision(mutation: Puzzle2dMutation) -> Puzzle2dMutation {
    let Some(schema) = Mutation::<Puzzle2dSnapshot>::input_schema(&mutation).and_then(|schema| serde_json::from_str::<Value>(schema).ok()) else {
        return mutation;
    };
    let mut payload = Value::from(Mutation::<Puzzle2dSnapshot>::payload_value(&mutation));
    let mut rounded = false;
    for (name, declared) in schema.get("properties").and_then(Value::as_object).into_iter().flatten() {
        let Some(precision) = declared.get("x-semio-ui").and_then(|ui| ui.get("precision")).and_then(Value::as_u64) else { continue };
        let Some(value) = payload.get(name.as_str()).and_then(Value::as_f64) else { continue };
        let scale = 10f64.powi(precision.min(15) as i32);
        let quantized = (value * scale).round() / scale;
        if quantized != value || (quantized == 0.0 && value.is_sign_negative()) {
            payload[name.as_str()] = Value::from(if quantized == 0.0 { 0.0 } else { quantized });
            rounded = true;
        }
    }
    if !rounded {
        return mutation;
    }
    Mutation::<Puzzle2dSnapshot>::with_payload_value(&mutation, semio_framework_value::DslValue::from(&payload)).unwrap_or(mutation)
}
}
pub use mutations_wire_codec::*;
