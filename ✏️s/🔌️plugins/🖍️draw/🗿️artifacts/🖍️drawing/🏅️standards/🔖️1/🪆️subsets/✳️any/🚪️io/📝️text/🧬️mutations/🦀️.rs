//! 🔧️ Drawing artifact — OpText/OpBinary codecs + grammar for serializing `DrawingMutation`.
//! Mutation apply/inverse live in `🧬️mutations`; this facet only handcrafts the op wire forms.

pub use crate::mutations::{drawing_op_for_layer_field, patch_layer_field, DrawingMutation};

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️HandcraftedOpCodecs
/// ⚡️ P6 handcrafted OpText/OpBinary (derive no longer emits these traits).
impl protocol::OpText for DrawingMutation {
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

impl protocol::OpBinary for DrawingMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️HandcraftedOpCodecs

/// 📜️ Describes the artifact mutation dialect.
pub const MUTATION_GRAMMAR_SEMIO: &str = include_str!("📖️mutations.grammar.semio");
/// 🧭️ Identifies the artifact mutation grammar.
pub const MUTATION_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️mutations.grammar.semio");

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::mutations::*;
use crate::schema::{find_drawing_layer, hex_to_rgba, layer_base};
use crate::{DrawingLayerNode, DrawingSnapshot, FillStyle, StrokeStyle};
use crate::standards::v1::subsets::style::schema::mutations::update_text::mutation::{update_text, UpdateText};
use crate::standards::v1::subsets::metadata::schema::mutations::rename_layer::mutation::{rename_layer, RenameLayer};
use crate::standards::v1::subsets::metadata::schema::mutations::set_layer_locked::mutation::{set_layer_locked, SetLayerLocked};
use crate::standards::v1::subsets::metadata::schema::mutations::set_layer_visible::mutation::{set_layer_visible, SetLayerVisible};
use crate::standards::v1::subsets::structure::schema::mutations::create_layer::mutation::{create_layer, CreateLayer};
use crate::standards::v1::subsets::structure::schema::mutations::delete_layer::mutation::{delete_layer, DeleteLayer};
use crate::standards::v1::subsets::structure::schema::mutations::duplicate_layer::mutation::{duplicate_layer, DuplicateLayer};
use crate::standards::v1::subsets::structure::schema::mutations::reorder_layer::mutation::{reorder_layer, ReorderLayer};
use crate::standards::v1::subsets::style::schema::mutations::replace_layer_fill::mutation::{replace_layer_fill, ReplaceLayerFill};
use crate::standards::v1::subsets::style::schema::mutations::replace_layer_stroke::mutation::{replace_layer_stroke, ReplaceLayerStroke};
use crate::standards::v1::subsets::style::schema::mutations::set_layer_blend_mode::mutation::{set_layer_blend_mode, SetLayerBlendMode};
use crate::standards::v1::subsets::style::schema::mutations::set_layer_opacity::mutation::{set_layer_opacity, SetLayerOpacity};
use crate::standards::v1::subsets::transform::schema::mutations::set_layer_boolean_operation::mutation::{set_layer_boolean_operation, SetLayerBooleanOperation};
use crate::standards::v1::subsets::transform::schema::mutations::update_layer_trace_params::mutation::{update_layer_trace_params, UpdateLayerTraceParams};
use crate::standards::v1::subsets::transform::schema::mutations::update_layer_transform::mutation::{update_layer_transform, UpdateLayerTransform};
use crate::standards::v1::subsets::transform::schema::mutations::update_path_geometry::mutation::{update_path_geometry, UpdatePathGeometry};
use crate::standards::v1::subsets::style::schema::mutations::set_layer_fill_rule::mutation::{set_layer_fill_rule,SetLayerFillRule};
use crate::standards::v1::subsets::style::schema::mutations::set_group_isolation::mutation::{set_group_isolation,SetGroupIsolation};
use crate::standards::v1::subsets::transform::schema::mutations::drag_layers::mutation::{drag_layers, DragLayers};
use crate::standards::v1::subsets::transform::schema::mutations::rotate_layers::mutation::{rotate_layers, RotateLayers};
use crate::standards::v1::subsets::transform::schema::mutations::scale_layers::mutation::{scale_layers, ScaleLayers};
use crate::standards::v1::subsets::transform::schema::mutations::drag_path_points::mutation::{drag_path_points, DragPathPoints, DrawingPathPointTarget};

/// ⌨️ Decode inspector input according to its field, preserving numeric-looking text.
pub fn parse_layer_field_input(field: &str, value: &str) -> semio_framework_value::DslValue {
    let parsed = semio_framework_pack_json::parse(value, semio_framework_pack_json::JsonMemberPolicy::Reject).ok().map(|parsed| semio_framework_pack_json::to_dsl_value(&parsed));
    if matches!(field, "textContent" | "name" | "blendMode" | "fillColor" | "fillRule" | "strokeColor" | "strokeCap" | "strokeJoin" | "strokeDash" | "booleanOperation") {
        if let Some(semio_framework_value::DslValue::String(text)) = parsed { return semio_framework_value::DslValue::String(text); }
        return semio_framework_value::DslValue::String(value.into());
    }
    parsed.unwrap_or_else(|| semio_framework_value::DslValue::String(value.into()))
}

/// 📤️ The bridge's answer shape: the resulting document beside the codes it raised, so a caller
/// that cannot name `protocol::MutationOutcome` can still tell an application from a refusal.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn bridge_render(snapshot: &DrawingSnapshot, messages: &[String]) -> String {
    let report = semio_framework_value::DslValue::object([("snapshot".to_string(), semio_framework_value::ToValue::to_value(snapshot)), ("messages".to_string(), semio_framework_value::ToValue::to_value(messages))]);
    semio_framework_pack_json::to_json_string(&report)
}
}
pub use mutations_codec::*;

#[allow(unused_imports)]
mod mutations_wire_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::mutations::*;
use crate::schema::{find_drawing_layer, hex_to_rgba, layer_base};
use crate::{DrawingLayerNode, DrawingSnapshot, FillStyle, StrokeStyle};
use crate::standards::v1::subsets::style::schema::mutations::update_text::mutation::{update_text, UpdateText};
use crate::standards::v1::subsets::metadata::schema::mutations::rename_layer::mutation::{rename_layer, RenameLayer};
use crate::standards::v1::subsets::metadata::schema::mutations::set_layer_locked::mutation::{set_layer_locked, SetLayerLocked};
use crate::standards::v1::subsets::metadata::schema::mutations::set_layer_visible::mutation::{set_layer_visible, SetLayerVisible};
use crate::standards::v1::subsets::structure::schema::mutations::create_layer::mutation::{create_layer, CreateLayer};
use crate::standards::v1::subsets::structure::schema::mutations::delete_layer::mutation::{delete_layer, DeleteLayer};
use crate::standards::v1::subsets::structure::schema::mutations::duplicate_layer::mutation::{duplicate_layer, DuplicateLayer};
use crate::standards::v1::subsets::structure::schema::mutations::reorder_layer::mutation::{reorder_layer, ReorderLayer};
use crate::standards::v1::subsets::style::schema::mutations::replace_layer_fill::mutation::{replace_layer_fill, ReplaceLayerFill};
use crate::standards::v1::subsets::style::schema::mutations::replace_layer_stroke::mutation::{replace_layer_stroke, ReplaceLayerStroke};
use crate::standards::v1::subsets::style::schema::mutations::set_layer_blend_mode::mutation::{set_layer_blend_mode, SetLayerBlendMode};
use crate::standards::v1::subsets::style::schema::mutations::set_layer_opacity::mutation::{set_layer_opacity, SetLayerOpacity};
use crate::standards::v1::subsets::transform::schema::mutations::set_layer_boolean_operation::mutation::{set_layer_boolean_operation, SetLayerBooleanOperation};
use crate::standards::v1::subsets::transform::schema::mutations::update_layer_trace_params::mutation::{update_layer_trace_params, UpdateLayerTraceParams};
use crate::standards::v1::subsets::transform::schema::mutations::update_layer_transform::mutation::{update_layer_transform, UpdateLayerTransform};
use crate::standards::v1::subsets::transform::schema::mutations::update_path_geometry::mutation::{update_path_geometry, UpdatePathGeometry};
use crate::standards::v1::subsets::style::schema::mutations::set_layer_fill_rule::mutation::{set_layer_fill_rule,SetLayerFillRule};
use crate::standards::v1::subsets::style::schema::mutations::set_group_isolation::mutation::{set_group_isolation,SetGroupIsolation};
use crate::standards::v1::subsets::transform::schema::mutations::drag_layers::mutation::{drag_layers, DragLayers};
use crate::standards::v1::subsets::transform::schema::mutations::rotate_layers::mutation::{rotate_layers, RotateLayers};
use crate::standards::v1::subsets::transform::schema::mutations::scale_layers::mutation::{scale_layers, ScaleLayers};
use crate::standards::v1::subsets::transform::schema::mutations::drag_path_points::mutation::{drag_path_points, DragPathPoints, DrawingPathPointTarget};

/// 🧩️ Decodes one committed `📸️snapshot/⬅️before/🔣️.json` document together with the
/// `🦠️mutation/🔣️.json` payload beside it — the same bytes the leaf's own fixture test
/// reads — into real typed values.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn bridge_decode_pair(snapshot_json: &str, mutation_json: &str) -> Result<(DrawingSnapshot, DrawingMutation), String> {
    let snapshot: DrawingSnapshot = semio_framework_pack_json::from_json_str(snapshot_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| format!("the committed drawing snapshot JSON does not decode: {error}"))?;
    let mutation: DrawingMutation = semio_framework_pack_json::from_json_str(mutation_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| format!("the committed drawing mutation JSON does not decode: {error}"))?;
    Ok((snapshot, mutation))
}
}
pub use mutations_wire_codec::*;
