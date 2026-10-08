//! 🔧️ Drawing text mutation codecs and native JSON bridges.
//! Mutation apply/inverse live in `🧬️mutations`; this facet only handcrafts the op wire forms.

pub use crate::mutations::{drawing_op_for_layer_field, DrawingMutation};
use crate::DrawingSnapshot;

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

//#endregion 🔖️HandcraftedOpCodecs

/// 📜️ Describes the artifact mutation dialect.
pub const MUTATION_GRAMMAR_SEMIO: &str = include_str!("📖️mutations.grammar.semio");
/// 🧭️ Identifies the artifact mutation grammar.
pub const MUTATION_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️mutations.grammar.semio");

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::mutations::*;
use crate::schema::{find_drawing_layer, layer_base};
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
use crate::schema::{find_drawing_layer, layer_base};
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

/// 🌉️ Decodes native JSON payloads, applies their domain mutation and encodes its answer.
pub fn apply_drawing_mutation_json(snapshot_json: &str, mutation_json: &str) -> Result<String, String> {
    let (snapshot, mutation) = bridge_decode_pair(snapshot_json, mutation_json)?;
    let (applied, messages) = bridge_step(&snapshot, &mutation)?;
    Ok(bridge_render(&applied, &messages))
}

/// ↩️ Encodes the result of one mutation followed by every domain inverse step.
pub fn undo_drawing_mutation_json(snapshot_json: &str, mutation_json: &str) -> Result<String, String> {
    use protocol::Mutation;
    let (base, mutation) = bridge_decode_pair(snapshot_json, mutation_json)?;
    let (mut current, mut messages) = bridge_step(&base, &mutation)?;
    for undo in <DrawingMutation as Mutation<DrawingSnapshot>>::inverse(&mutation, &base).map_err(semio_framework_value::ValueError::into_message)? {
        let (next, raised) = bridge_step(&current, &undo)?;
        current = next;
        messages.extend(raised);
    }
    Ok(bridge_render(&current, &messages))
}

#[path="🎛️field-input/🦀️.rs"]
pub mod field_input;

//#region 🌉️Apply
// The single central-apply entry points of the text/native bridge live here, outside the schema tree: only editors, io and
// stores call `protocol::apply_diff`.
/// 🩹 Applies one field patch directly to `doc` — used by callers that don't need the mutation
/// value itself (`drawing_op_for_layer_field` is the undoable/command-facing entry point).
pub fn patch_layer_field(doc: &DrawingSnapshot, layer_id: &str, field: &str, value: &semio_framework_value::DslValue) -> protocol::MutationApplyResult<DrawingSnapshot> {
    use protocol::Mutation;
    match drawing_op_for_layer_field(doc, layer_id, field, value) {
        Some(operation) => protocol::apply_diff(operation.diff(doc).diff(), doc).map_err(|error| error.under(["layers", layer_id])),
        None => Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "layer field cannot be patched").at(["layers", layer_id, field])),
    }
}
/// ▶️ Applies `mutation` to `snapshot` through its own diff — the artifact's single apply entry
/// point (mirrors dag's `apply_dag_mutation`/puzzle5d's `apply_puzzle5d_mutation`). A rejecting
/// diff carries an empty `DrawingDiff`, so the snapshot is left untouched and `Ok(())` is still
/// returned; read [`protocol::MutationOutcome::messages`] to distinguish the two.
pub fn apply_drawing_mutation(snapshot: &mut DrawingSnapshot, mutation: &DrawingMutation) -> protocol::MutationApplyResult<()> {
    *snapshot = protocol::apply_diff(<DrawingMutation as protocol::Mutation<DrawingSnapshot>>::diff(mutation, snapshot).diff(), snapshot)?;
    Ok(())
}

/// ▶️ One diff-and-apply step, keeping the diagnostic codes the outcome raised — a rejected or
/// no-op kind is a RESULT this bridge reports, never an error it swallows.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn bridge_step(snapshot: &DrawingSnapshot, mutation: &DrawingMutation) -> Result<(DrawingSnapshot, Vec<String>), String> {
    use protocol::Mutation;
    let outcome = <DrawingMutation as Mutation<DrawingSnapshot>>::diff(mutation, snapshot);
    let messages: Vec<String> = outcome.messages().iter().map(|message| message.code.0.clone()).collect();
    match protocol::apply_diff(outcome.diff(), snapshot) {
        Ok(next) => Ok((next, messages)),
        Err(error) => Err(format!("{error:?}")),
    }
}



//#endregion 🌉️Apply
