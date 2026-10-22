//! 🧬️ Drawing artifact — semantic document mutation dispatch enum. Every variant is a single-field
//! tuple wrapping a handcrafted `protocol::MutationKind` payload (see the `🧬️mutations/<slug>/`
//! triad leaves); `#[derive(dsl::Mutations)]` generates `impl protocol::Mutation<DrawingSnapshot>`
//! and `impl protocol::SemanticMutation<DrawingSnapshot>` from those payloads — no hand-written
//! apply/diff/inverse dispatch here.

#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;


use semio_s_artifact_draw_drawing_model::{find_drawing_layer, layer_base};
use semio_s_artifact_draw_drawing_model::{DrawingLayerNode, FillStyle, StrokeStyle};
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;
use semio_s_artifact_draw_drawing_structure::import_image_asset::mutation::ImportImageAsset;
use semio_s_artifact_draw_drawing_structure::remove_image_asset::mutation::RemoveImageAsset;

//#region 🔖️Mutations
#[derive(Clone, Debug, PartialEq, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[value(tag = "mutation", rename_all = "camelCase")]
#[cfg_attr(any(test, feature = "testkit"), serde(tag = "mutation", rename_all = "camelCase"))]
#[mutations(snapshot = DrawingSnapshot, diff = semio_s_artifact_draw_drawing_diff::DrawingDiff, schema = "drawing.drawing", reference = DrawingSnapshot, reference_diff = semio_s_artifact_draw_drawing_diff::DrawingDiff)]
pub enum DrawingMutation {
    SetLayerVisible(SetLayerVisible),
    SetLayerLocked(SetLayerLocked),
    SetLayerOpacity(SetLayerOpacity),
    SetLayerBlendMode(SetLayerBlendMode),
    RenameLayer(RenameLayer),
    UpdateLayerTransform(UpdateLayerTransform),
    ReplaceLayerFill(ReplaceLayerFill),
    ReplaceLayerStroke(ReplaceLayerStroke),
    SetLayerBooleanOperation(SetLayerBooleanOperation),
    UpdateLayerTraceParams(UpdateLayerTraceParams),
    CreateLayer(CreateLayer),
    DuplicateLayer(DuplicateLayer),
    DeleteLayer(DeleteLayer),
    ReorderLayer(ReorderLayer),
    UpdatePathGeometry(UpdatePathGeometry),
    UpdateText(UpdateText),
    SetLayerFillRule(SetLayerFillRule),
    SetGroupIsolation(SetGroupIsolation),
    DragLayers(DragLayers),
    RotateLayers(RotateLayers),
    ScaleLayers(ScaleLayers),
    DragPathPoints(DragPathPoints),
    UpdateImage(UpdateImage),
    SetShapeCoordinate(SetShapeCoordinate),
    ImportImageAsset(ImportImageAsset),
    RemoveImageAsset(RemoveImageAsset),
}
//#endregion 🔖️Mutations
use semio_s_artifact_draw_drawing_style::update_text::mutation::update_text;
use semio_s_artifact_draw_drawing_style::update_text::mutation::UpdateText;

//#region 🔖️FieldPatch


fn field_color(value:&semio_framework_value::DslValue)->Option<[f64;4]>{let parts=value.as_array()?;if parts.len()!=4{return None;}let mut output=[0.0;4];for(index,part)in parts.iter().enumerate(){let number=part.as_f64()?;if !number.is_finite()||!(0.0..=1.0).contains(&number){return None;}output[index]=number;}Some(output)}
fn field_dash(value:&semio_framework_value::DslValue)->Option<Option<Vec<f64>>>{if matches!(value,semio_framework_value::DslValue::Null){return Some(None);}let samples=value.as_array()?;if samples.len()>64{return None;}let mut output=Vec::with_capacity(samples.len());for sample in samples{let number=sample.as_f64()?;if !number.is_finite()||number<0.0{return None;}output.push(number);}Some(output.iter().any(|value|*value>0.0).then_some(output))}

/// 🎛️ Generic single-field layer editor bridge (properties panel / bulk patch commands) — maps a
/// owned `field` name + typed `value` onto the one semantic mutation that owns that field. Returns
/// `None` for an unknown field or a field that doesn't apply to `layer`'s kind.
pub fn drawing_op_for_layer_field(doc: &DrawingSnapshot, layer_id: &(impl semio_framework_value::paged::Utf8Text + ?Sized), field: &str, value: &semio_framework_value::DslValue) -> Option<DrawingMutation> {
    let layer = find_drawing_layer(doc, layer_id)?;
    let layer_id = &layer_base(layer).id;
    if let Ok(coordinate)=semio_s_artifact_draw_drawing_model::shape_geometry::ShapeCoordinateField::parse(field) {
        let DrawingLayerNode::Shape(shape)=layer else{return None;};
        let value=value.as_f64().filter(|number|number.is_finite())?;
        semio_s_artifact_draw_drawing_model::shape_geometry::shape_coordinate(shape,&coordinate,None).ok()?;
        if value<0.0&&matches!(coordinate,semio_s_artifact_draw_drawing_model::shape_geometry::ShapeCoordinateField::RectWidth|semio_s_artifact_draw_drawing_model::shape_geometry::ShapeCoordinateField::RectHeight|semio_s_artifact_draw_drawing_model::shape_geometry::ShapeCoordinateField::EllipseRx|semio_s_artifact_draw_drawing_model::shape_geometry::ShapeCoordinateField::EllipseRy|semio_s_artifact_draw_drawing_model::shape_geometry::ShapeCoordinateField::CircleR){return None;}
        return Some(set_shape_coordinate(layer_id.clone(),coordinate,None,value));
    }
    let finite = || value.as_f64().filter(|number| number.is_finite());
    match field {
        "name" => { value.as_str()?; }
        "imageKey" => { if !matches!(layer,DrawingLayerNode::Image(_)){return None;}value.as_str()?; }
        "imageWidth" | "imageHeight" => { if !matches!(layer,DrawingLayerNode::Image(_))||finite()?<=0.0{return None;} }
        "textContent" => { if !matches!(layer, DrawingLayerNode::Text(_)) { return None; } value.as_str()?; }
        "fontFamily" => {if !matches!(layer,DrawingLayerNode::Text(_)){return None;}semio_s_artifact_draw_drawing_model::font_family::DrawingFontFamily::parse(value.as_str()?).ok()?;}
        "textSize" => { if !matches!(layer, DrawingLayerNode::Text(_)) || finite()? <= 0.0 { return None; } }
        "visible" | "locked" | "fillEnabled" | "strokeEnabled" => { value.as_bool()?; }
        "opacity" | "traceThreshold" => { if !(0.0..=1.0).contains(&finite()?) { return None; } }
        "strokeWidth" | "traceSimplify" => { if finite()? < 0.0 { return None; } }
        "transformX" | "transformY" | "transformRotation" | "transformShear" | "rotationDegrees" => { finite()?; }
        "transformScaleX" | "transformScaleY" => { finite()?; }
        "fillColor" | "strokeColor" => { field_color(value)?; }

        "isolation" => {if !matches!(layer,DrawingLayerNode::Group(_)){return None;}value.as_bool()?;}
        "fillRule" => { semio_s_artifact_draw_drawing_model::fill_rule::FillRule::parse(value.as_str()?).ok()?; }
        "strokeCap" => { semio_s_artifact_draw_drawing_model::stroke::StrokeCap::parse(value.as_str()?).ok()?; }
        "strokeJoin" => { semio_s_artifact_draw_drawing_model::stroke::StrokeJoin::parse(value.as_str()?).ok()?; }
        "strokeDash" => { field_dash(value)?; }
        "blendMode" => { if !semio_s_artifact_draw_drawing_model::DRAWING_BLEND_MODES.contains(&value.as_str()?) { return None; } }
        "booleanOperation" => { if !matches!(layer, DrawingLayerNode::Boolean(_)) || !semio_s_artifact_draw_drawing_model::DRAWING_BOOLEAN_OPERATIONS.contains(&value.as_str()?) { return None; } }
        _ => return None,
    }
    let operation = match field {
        "name" => rename_layer(layer_id.clone(), value.as_str().unwrap_or("").into()),
        "textContent" | "textSize" | "fontFamily" => {
            let DrawingLayerNode::Text(text) = layer else { return None; };
            update_text(layer_id.clone(), if field == "textContent" { value.as_str()?.into() } else { text.content.clone() }, if field == "textSize" { finite()? } else { text.size },if field=="fontFamily"{semio_s_artifact_draw_drawing_model::font_family::DrawingFontFamily::parse(value.as_str()?).ok()?}else{text.font_family})
        }
        "imageKey" | "imageWidth" | "imageHeight" => {
            let DrawingLayerNode::Image(image)=layer else{return None;};
            update_image(layer_id.clone(),if field=="imageKey"{value.as_str()?.into()}else{image.image_key.clone()},if field=="imageWidth"{finite()?}else{image.width},if field=="imageHeight"{finite()?}else{image.height})
        }
        "opacity" => set_layer_opacity(layer_id.clone(), value.as_f64().unwrap_or(1.0)),
        "visible" => set_layer_visible(layer_id.clone(), value.as_bool().unwrap_or(true)),
        "locked" => set_layer_locked(layer_id.clone(), value.as_bool().unwrap_or(false)),
        "blendMode" => set_layer_blend_mode(layer_id.clone(), value.as_str().unwrap_or("normal").into()),
        "booleanOperation" => set_layer_boolean_operation(layer_id.clone(), value.as_str().unwrap_or("union").into()),
        "transformX" | "transformY" | "transformScaleX" | "transformScaleY" | "transformRotation" | "transformShear" | "rotationDegrees" => {
            let mut transform = layer_base(layer).transform.clone();
            match field {
                "transformX" => transform.x = value.as_f64().unwrap_or(0.0),
                "transformY" => transform.y = value.as_f64().unwrap_or(0.0),
                "transformScaleX" => transform.scale_x = value.as_f64().unwrap_or(1.0),
                "transformScaleY" => transform.scale_y = value.as_f64().unwrap_or(1.0),
                "transformShear" => transform.shear = finite()?,
                "rotationDegrees" => transform.rotation = finite()?.to_radians(),
                _ => transform.rotation = finite()?,
            }
            update_layer_transform(layer_id.clone(), transform)
        }
        "fillColor" => {
            let alpha = layer_base(layer).attributes.fill.as_ref().map_or(1.0, |fill| match fill {
                FillStyle::Solid { color } => color[3],
                FillStyle::LinearGradient { .. } | FillStyle::RadialGradient { .. } => 1.0,
            });
            replace_layer_fill(layer_id.clone(), Some(FillStyle::Solid { color: {let mut color=field_color(value)?;color[3]=alpha;color} }))
        }
        "isolation" => set_group_isolation(layer_id.clone(),value.as_bool()?),
        "fillRule" => set_layer_fill_rule(layer_id.clone(),semio_s_artifact_draw_drawing_model::fill_rule::FillRule::parse(value.as_str()?).ok()?),
        "fillEnabled" => replace_layer_fill(layer_id.clone(), if value.as_bool()? { Some(layer_base(layer).attributes.fill.clone().unwrap_or(FillStyle::Solid { color: [0.0, 0.0, 0.0, 1.0] })) } else { None }),
        "strokeEnabled" => replace_layer_stroke(layer_id.clone(), if value.as_bool()? { Some(layer_base(layer).attributes.stroke.clone().unwrap_or(StrokeStyle { color: [0.0, 0.0, 0.0, 1.0], width: 1.0, cap: semio_s_artifact_draw_drawing_model::stroke::StrokeCap::Butt, join: semio_s_artifact_draw_drawing_model::stroke::StrokeJoin::Miter, dash: None })) } else { None }),
        "strokeWidth" | "strokeColor" | "strokeCap" | "strokeJoin" | "strokeDash" => {
            let mut stroke = layer_base(layer).attributes.stroke.clone().unwrap_or(StrokeStyle { color: [0.0, 0.0, 0.0, 1.0], width: 1.0, cap: semio_s_artifact_draw_drawing_model::stroke::StrokeCap::Butt, join: semio_s_artifact_draw_drawing_model::stroke::StrokeJoin::Miter, dash: None });
            match field {
                "strokeWidth" => stroke.width = finite()?,
                "strokeColor" => stroke.color = {let mut color=field_color(value)?;color[3]=stroke.color[3];color},
                "strokeCap" => stroke.cap = semio_s_artifact_draw_drawing_model::stroke::StrokeCap::parse(value.as_str()?).ok()?,
                "strokeJoin" => stroke.join = semio_s_artifact_draw_drawing_model::stroke::StrokeJoin::parse(value.as_str()?).ok()?,
                _ => stroke.dash = field_dash(value)?.map(Into::into),
            }
            replace_layer_stroke(layer_id.clone(), Some(stroke))
        }
        "traceThreshold" => {
            let DrawingLayerNode::Trace(trace) = layer else { return None };
            let mut params = trace.params.clone();
            params.threshold = value.as_f64().unwrap_or(0.5);
            update_layer_trace_params(layer_id.clone(), params)
        }
        "traceSimplify" => {
            let DrawingLayerNode::Trace(trace) = layer else { return None };
            let mut params = trace.params.clone();
            params.simplify_epsilon = value.as_f64().unwrap_or(1.5);
            update_layer_trace_params(layer_id.clone(), params)
        }
        _ => return None,
    };
    Some(operation)
}

//#endregion 🔖️FieldPatch

// 🪆️ These fourteen leaves now live under their own semantic subset (`structure`/`style`/
// `transform`/`metadata`, see `../../../🔣️.json`), not as siblings of this catalog any more — this
// catalog is the one thing every subset composes back through, so its `use`s are fully qualified.
use semio_s_artifact_draw_drawing_metadata::rename_layer::mutation::rename_layer;
use semio_s_artifact_draw_drawing_metadata::rename_layer::mutation::RenameLayer;
use semio_s_artifact_draw_drawing_metadata::set_layer_locked::mutation::set_layer_locked;
use semio_s_artifact_draw_drawing_metadata::set_layer_locked::mutation::SetLayerLocked;
use semio_s_artifact_draw_drawing_metadata::set_layer_visible::mutation::set_layer_visible;
use semio_s_artifact_draw_drawing_metadata::set_layer_visible::mutation::SetLayerVisible;
use semio_s_artifact_draw_drawing_structure::create_layer::mutation::create_layer;
use semio_s_artifact_draw_drawing_structure::create_layer::mutation::CreateLayer;
use semio_s_artifact_draw_drawing_structure::delete_layer::mutation::delete_layer;
use semio_s_artifact_draw_drawing_structure::delete_layer::mutation::DeleteLayer;
use semio_s_artifact_draw_drawing_structure::duplicate_layer::mutation::duplicate_layer;
use semio_s_artifact_draw_drawing_structure::duplicate_layer::mutation::DuplicateLayer;
use semio_s_artifact_draw_drawing_structure::reorder_layer::mutation::reorder_layer;
use semio_s_artifact_draw_drawing_structure::reorder_layer::mutation::ReorderLayer;
use semio_s_artifact_draw_drawing_style::replace_layer_fill::mutation::replace_layer_fill;
use semio_s_artifact_draw_drawing_style::replace_layer_fill::mutation::ReplaceLayerFill;
use semio_s_artifact_draw_drawing_style::replace_layer_stroke::mutation::replace_layer_stroke;
use semio_s_artifact_draw_drawing_style::replace_layer_stroke::mutation::ReplaceLayerStroke;
use semio_s_artifact_draw_drawing_style::set_layer_blend_mode::mutation::set_layer_blend_mode;
use semio_s_artifact_draw_drawing_style::set_layer_blend_mode::mutation::SetLayerBlendMode;
use semio_s_artifact_draw_drawing_style::set_layer_opacity::mutation::set_layer_opacity;
use semio_s_artifact_draw_drawing_style::set_layer_opacity::mutation::SetLayerOpacity;
use semio_s_artifact_draw_drawing_transform::set_layer_boolean_operation::mutation::set_layer_boolean_operation;
use semio_s_artifact_draw_drawing_transform::set_layer_boolean_operation::mutation::SetLayerBooleanOperation;
use semio_s_artifact_draw_drawing_transform::update_layer_trace_params::mutation::update_layer_trace_params;
use semio_s_artifact_draw_drawing_transform::update_layer_trace_params::mutation::UpdateLayerTraceParams;
use semio_s_artifact_draw_drawing_transform::update_layer_transform::mutation::update_layer_transform;
use semio_s_artifact_draw_drawing_transform::update_layer_transform::mutation::UpdateLayerTransform;

//#region 🔖️Apply
/// ↩️ The typed mutation steps that undo `mutation` against `snapshot`.
pub fn inverse_drawing_mutation(snapshot: &DrawingSnapshot, mutation: &DrawingMutation) -> Result<Vec<DrawingMutation>, semio_framework_value::ValueError> {
    <DrawingMutation as protocol::ColdMutation<DrawingSnapshot>>::inverse(mutation, snapshot)
}
//#endregion 🔖️Apply

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🌉️ExternalCodecBridge


//#endregion 🌉️ExternalCodecBridge

//#region 🔖️Kinds
/// 🏷️ Kebab-case spelling of every `DrawingMutation` variant, in declaration order — the vocabulary
/// the `drawing-1-any` catalog (`../../🔣️oracle.json`) declares and the `mutate-drawing-1`
/// exhaustive case measures itself against. Ten of the fourteen address ONE layer of the recursive
/// tree by id, `create-layer`/`duplicate-layer`/`reorder-layer` address a parent plus an index, and
/// `update-layer-trace-params` exists only for the trace node kind. `kinds_match_the_enum_and_the_
/// catalog` below is what keeps this list honest against the enum, since the framework never parses
/// Rust.
pub const KINDS: &[&str] = &[
    "set-layer-visible",
    "set-layer-locked",
    "set-layer-opacity",
    "set-layer-blend-mode",
    "rename-layer",
    "update-layer-transform",
    "replace-layer-fill",
    "replace-layer-stroke",
    "set-layer-boolean-operation",
    "update-layer-trace-params",
    "create-layer",
    "duplicate-layer",
    "delete-layer",
    "reorder-layer",
    "update-path-geometry",
    "update-text",
    "set-layer-fill-rule",
    "set-group-isolation",
    "drag-layers",
    "rotate-layers",
    "scale-layers",
    "drag-path-points",
    "update-image",
    "set-shape-coordinate",
    "import-image-asset",
    "remove-image-asset",
];
//#endregion 🔖️Kinds

//#region 🧪️KindsCatalog
#[cfg(test)]
#[path = "🧪️tests/🔬️kinds-catalog/🦀️.rs"]
mod kinds_catalog_tests;
//#endregion 🧪️KindsCatalog

use semio_s_artifact_draw_drawing_transform::update_path_geometry::mutation::UpdatePathGeometry;

use semio_s_artifact_draw_drawing_style::set_layer_fill_rule::mutation::set_layer_fill_rule;
use semio_s_artifact_draw_drawing_style::set_layer_fill_rule::mutation::SetLayerFillRule;

use semio_s_artifact_draw_drawing_style::set_group_isolation::mutation::set_group_isolation;
use semio_s_artifact_draw_drawing_style::set_group_isolation::mutation::SetGroupIsolation;

use semio_s_artifact_draw_drawing_transform::drag_layers::mutation::drag_layers;
use semio_s_artifact_draw_drawing_transform::drag_layers::mutation::DragLayers;
use semio_s_artifact_draw_drawing_transform::rotate_layers::mutation::rotate_layers;
use semio_s_artifact_draw_drawing_transform::rotate_layers::mutation::RotateLayers;
use semio_s_artifact_draw_drawing_transform::scale_layers::mutation::scale_layers;
use semio_s_artifact_draw_drawing_transform::scale_layers::mutation::ScaleLayers;
use semio_s_artifact_draw_drawing_transform::drag_path_points::mutation::drag_path_points;
use semio_s_artifact_draw_drawing_transform::drag_path_points::mutation::DragPathPoints;
use semio_s_artifact_draw_drawing_transform::drag_path_points::mutation::DrawingPathPointTarget;

//#region 🔖️SelectionTransform


//#endregion 🔖️SelectionTransform

use semio_s_artifact_draw_drawing_style::update_image::mutation::update_image;
use semio_s_artifact_draw_drawing_style::update_image::mutation::UpdateImage;

use semio_s_artifact_draw_drawing_transform::shape_coordinate::mutation::set_shape_coordinate;
use semio_s_artifact_draw_drawing_transform::shape_coordinate::mutation::SetShapeCoordinate;

/// 🏗️ `DrawingSnapshot`/`DrawingMutation`'s `ArtifactBuilder` is the ordinary `Mutation`/`MutationDiff`
/// algebra with no custom build logic beyond that — the old hand-rolled `DrawingBuilderConstruction`
/// (`empty`/`from_snapshot`/`from_text`/`from_binary`/`mutate`/`absorb`/`build`) matched
/// `SnapshotBuilder<S, M>`'s own documented "apply `outcome.diff()` once, fold a rejection into a
/// Fatal message" idiom exactly, so this subset writes zero builder boilerplate (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE task 3).
pub type Construction = semio_framework_plugin::app::SnapshotBuilder<DrawingSnapshot, DrawingMutation>;

//#region 🪢️TaxonomyMounts
#[path = "../../🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
pub mod binary;
#[path = "../../../🏷️metadata/🧬️schema/🧬️mutations/✏️rename-layer/🎮️prepare/🦀️.rs"]
pub mod rename_prepare;
#[path = "../../🚪️io/📝️text/🧬️mutations/🦀️.rs"]
pub mod text;
//#endregion 🪢️TaxonomyMounts
