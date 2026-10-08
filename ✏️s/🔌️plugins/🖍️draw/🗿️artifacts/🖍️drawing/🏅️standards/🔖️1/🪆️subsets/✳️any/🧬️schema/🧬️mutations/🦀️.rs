//! 🧬️ Drawing artifact — semantic document mutation dispatch enum. Every variant is a single-field
//! tuple wrapping a handcrafted `protocol::MutationKind` payload (see the `🧬️mutations/<slug>/`
//! triad leaves); `#[derive(dsl::Mutations)]` generates `impl protocol::Mutation<DrawingSnapshot>`
//! and `impl protocol::SemanticMutation<DrawingSnapshot>` from those payloads — no hand-written
//! apply/diff/inverse dispatch here.

use crate::schema::{find_drawing_layer, layer_base};
use crate::{DrawingLayerNode, DrawingSnapshot, FillStyle, StrokeStyle};

//#region 🔖️Mutations
#[derive(Clone, Debug, PartialEq, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(tag = "mutation", rename_all = "camelCase")]
#[cfg_attr(test, serde(tag = "mutation", rename_all = "camelCase"))]
#[mutations(snapshot = DrawingSnapshot, diff = crate::diff::DrawingDiff, schema = "drawing.drawing")]
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
}
//#endregion 🔖️Mutations
pub use crate::standards::v1::subsets::style::schema::mutations::update_text::mutation::{update_text, UpdateText};

//#region 🔖️FieldPatch


fn field_color(value:&semio_framework_value::DslValue)->Option<[f64;4]>{let parts=value.as_array()?;if parts.len()!=4{return None;}let mut output=[0.0;4];for(index,part)in parts.iter().enumerate(){let number=part.as_f64()?;if !number.is_finite()||!(0.0..=1.0).contains(&number){return None;}output[index]=number;}Some(output)}
fn field_dash(value:&semio_framework_value::DslValue)->Option<Option<Vec<f64>>>{if matches!(value,semio_framework_value::DslValue::Null){return Some(None);}let samples=value.as_array()?;if samples.len()>64{return None;}let mut output=Vec::with_capacity(samples.len());for sample in samples{let number=sample.as_f64()?;if !number.is_finite()||number<0.0{return None;}output.push(number);}Some(output.iter().any(|value|*value>0.0).then_some(output))}

/// 🎛️ Generic single-field layer editor bridge (properties panel / bulk patch commands) — maps a
/// owned `field` name + typed `value` onto the one semantic mutation that owns that field. Returns
/// `None` for an unknown field or a field that doesn't apply to `layer`'s kind.
pub fn drawing_op_for_layer_field(doc: &DrawingSnapshot, layer_id: &(impl semio_framework_value::paged::Utf8Text + ?Sized), field: &str, value: &semio_framework_value::DslValue) -> Option<DrawingMutation> {
    let layer = find_drawing_layer(doc, layer_id)?;
    let layer_id = &layer_base(layer).id;
    let finite = || value.as_f64().filter(|number| number.is_finite());
    match field {
        "name" => { value.as_str()?; }
        "textContent" => { if !matches!(layer, DrawingLayerNode::Text(_)) { return None; } value.as_str()?; }
        "textSize" => { if !matches!(layer, DrawingLayerNode::Text(_)) || finite()? <= 0.0 { return None; } }
        "visible" | "locked" | "fillEnabled" | "strokeEnabled" => { value.as_bool()?; }
        "opacity" | "traceThreshold" => { if !(0.0..=1.0).contains(&finite()?) { return None; } }
        "strokeWidth" | "traceSimplify" => { if finite()? < 0.0 { return None; } }
        "transformX" | "transformY" | "transformRotation" | "transformShear" | "rotationDegrees" => { finite()?; }
        "transformScaleX" | "transformScaleY" => { finite()?; }
        "fillColor" | "strokeColor" => { field_color(value)?; }

        "isolation" => {if !matches!(layer,DrawingLayerNode::Group(_)){return None;}value.as_bool()?;}
        "fillRule" => { crate::FillRule::parse(value.as_str()?).ok()?; }
        "strokeCap" => { crate::StrokeCap::parse(value.as_str()?).ok()?; }
        "strokeJoin" => { crate::StrokeJoin::parse(value.as_str()?).ok()?; }
        "strokeDash" => { field_dash(value)?; }
        "blendMode" => { if !crate::DRAWING_BLEND_MODES.contains(&value.as_str()?) { return None; } }
        "booleanOperation" => { if !matches!(layer, DrawingLayerNode::Boolean(_)) || !matches!(value.as_str()?, "union" | "intersect" | "subtract" | "exclude") { return None; } }
        _ => return None,
    }
    let operation = match field {
        "name" => rename_layer(layer_id.clone(), value.as_str().unwrap_or("").into()),
        "textContent" | "textSize" => {
            let DrawingLayerNode::Text(text) = layer else { return None; };
            update_text(layer_id.clone(), if field == "textContent" { value.as_str()?.into() } else { text.content.clone() }, if field == "textSize" { finite()? } else { text.size })
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
        "fillRule" => set_layer_fill_rule(layer_id.clone(),crate::FillRule::parse(value.as_str()?).ok()?),
        "fillEnabled" => replace_layer_fill(layer_id.clone(), if value.as_bool()? { Some(layer_base(layer).attributes.fill.clone().unwrap_or(FillStyle::Solid { color: [0.0, 0.0, 0.0, 1.0] })) } else { None }),
        "strokeEnabled" => replace_layer_stroke(layer_id.clone(), if value.as_bool()? { Some(layer_base(layer).attributes.stroke.clone().unwrap_or(StrokeStyle { color: [0.0, 0.0, 0.0, 1.0], width: 1.0, cap: crate::StrokeCap::Butt, join: crate::StrokeJoin::Miter, dash: None })) } else { None }),
        "strokeWidth" | "strokeColor" | "strokeCap" | "strokeJoin" | "strokeDash" => {
            let mut stroke = layer_base(layer).attributes.stroke.clone().unwrap_or(StrokeStyle { color: [0.0, 0.0, 0.0, 1.0], width: 1.0, cap: crate::StrokeCap::Butt, join: crate::StrokeJoin::Miter, dash: None });
            match field {
                "strokeWidth" => stroke.width = finite()?,
                "strokeColor" => stroke.color = {let mut color=field_color(value)?;color[3]=stroke.color[3];color},
                "strokeCap" => stroke.cap = crate::StrokeCap::parse(value.as_str()?).ok()?,
                "strokeJoin" => stroke.join = crate::StrokeJoin::parse(value.as_str()?).ok()?,
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
pub use crate::standards::v1::subsets::metadata::schema::mutations::rename_layer::mutation::{rename_layer, RenameLayer};
pub use crate::standards::v1::subsets::metadata::schema::mutations::set_layer_locked::mutation::{set_layer_locked, SetLayerLocked};
pub use crate::standards::v1::subsets::metadata::schema::mutations::set_layer_visible::mutation::{set_layer_visible, SetLayerVisible};
pub use crate::standards::v1::subsets::structure::schema::mutations::create_layer::mutation::{create_layer, CreateLayer};
pub use crate::standards::v1::subsets::structure::schema::mutations::delete_layer::mutation::{delete_layer, DeleteLayer};
pub use crate::standards::v1::subsets::structure::schema::mutations::duplicate_layer::mutation::{duplicate_layer, DuplicateLayer};
pub use crate::standards::v1::subsets::structure::schema::mutations::reorder_layer::mutation::{reorder_layer, ReorderLayer};
pub use crate::standards::v1::subsets::style::schema::mutations::replace_layer_fill::mutation::{replace_layer_fill, ReplaceLayerFill};
pub use crate::standards::v1::subsets::style::schema::mutations::replace_layer_stroke::mutation::{replace_layer_stroke, ReplaceLayerStroke};
pub use crate::standards::v1::subsets::style::schema::mutations::set_layer_blend_mode::mutation::{set_layer_blend_mode, SetLayerBlendMode};
pub use crate::standards::v1::subsets::style::schema::mutations::set_layer_opacity::mutation::{set_layer_opacity, SetLayerOpacity};
pub use crate::standards::v1::subsets::transform::schema::mutations::set_layer_boolean_operation::mutation::{set_layer_boolean_operation, SetLayerBooleanOperation};
pub use crate::standards::v1::subsets::transform::schema::mutations::update_layer_trace_params::mutation::{update_layer_trace_params, UpdateLayerTraceParams};
pub use crate::standards::v1::subsets::transform::schema::mutations::update_layer_transform::mutation::{update_layer_transform, UpdateLayerTransform};

//#region 🔖️Apply
/// ↩️ The typed mutation steps that undo `mutation` against `snapshot`.
pub fn inverse_drawing_mutation(snapshot: &DrawingSnapshot, mutation: &DrawingMutation) -> Result<Vec<DrawingMutation>, semio_framework_value::ValueError> {
    <DrawingMutation as protocol::Mutation<DrawingSnapshot>>::inverse(mutation, snapshot)
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
];
//#endregion 🔖️Kinds

//#region 🧪️KindsCatalog
#[cfg(test)]
#[path = "🧪️tests/🔬️kinds-catalog/🦀️.rs"]
mod kinds_catalog_tests;
//#endregion 🧪️KindsCatalog

pub use crate::standards::v1::subsets::transform::schema::mutations::update_path_geometry::mutation::{update_path_geometry, UpdatePathGeometry};

pub use crate::standards::v1::subsets::style::schema::mutations::set_layer_fill_rule::mutation::{set_layer_fill_rule,SetLayerFillRule};

pub use crate::standards::v1::subsets::style::schema::mutations::set_group_isolation::mutation::{set_group_isolation,SetGroupIsolation};

pub use crate::standards::v1::subsets::transform::schema::mutations::drag_layers::mutation::{drag_layers, DragLayers};
pub use crate::standards::v1::subsets::transform::schema::mutations::rotate_layers::mutation::{rotate_layers, RotateLayers};
pub use crate::standards::v1::subsets::transform::schema::mutations::scale_layers::mutation::{scale_layers, ScaleLayers};
pub use crate::standards::v1::subsets::transform::schema::mutations::drag_path_points::mutation::{drag_path_points, DragPathPoints, DrawingPathPointTarget};

//#region 🔖️SelectionTransform
/// 🧮️ The identity affine matrix `[a, b, c, d, e, f]`.
pub const DRAWING_IDENTITY_MATRIX: [f64; 6] = [1.0, 0.0, 0.0, 1.0, 0.0, 0.0];

/// 🌍️ One addressed layer of `base` in document order: its parent chain's world matrix, whether it and every ancestor is
/// visible and unlocked, and the nearest addressed ancestor, if any.
pub struct DrawingPlacedLayer<'a> {
    pub layer: &'a DrawingLayerNode,
    pub parent: [f64; 6],
    pub editable: bool,
    pub addressed_ancestor: Option<&'a semio_framework_value::paged::PagedUtf8<{usize::MAX}>>,
}

/// 🗂️ Every layer of `base` whose id is in `ids`, in document (pre-)order, placed in the world.
pub fn drawing_placed_layers<'a, T: semio_framework_value::paged::Utf8Text>(base: &'a DrawingSnapshot, ids: &[T]) -> Vec<DrawingPlacedLayer<'a>> {
    let mut placed = Vec::new();
    let mut stack: Vec<(semio_framework_value::list::PagedIter<'a, DrawingLayerNode, {usize::MAX}>, [f64; 6], bool, Option<&'a semio_framework_value::paged::PagedUtf8<{usize::MAX}>>)> = vec![(base.layers.iter(), DRAWING_IDENTITY_MATRIX, true, None)];
    while let Some((layers, parent, editable, ancestor)) = stack.last_mut() {
        let Some(layer) = layers.next() else {
            stack.pop();
            continue;
        };
        let (parent, ancestor) = (*parent, *ancestor);
        let layer_base = layer_base(layer);
        let editable = *editable && layer_base.visible && !layer_base.locked;
        let addressed = ids.iter().any(|id| layer_base.id.eq_text(id));
        if addressed {
            placed.push(DrawingPlacedLayer { layer, parent, editable, addressed_ancestor: ancestor });
        }
        if let DrawingLayerNode::Group(group) = layer {
            let matrix = crate::schema::geometry::multiply(parent, crate::schema::drawing_transform_to_matrix(&layer_base.transform));
            stack.push((group.children.iter(), matrix, editable, if addressed { Some(&layer_base.id) } else { ancestor }));
        }
    }
    placed
}

/// 🚨️ The schema-stated target invariant every selection transform shares: at least one id, none repeated.
pub fn drawing_targets_invariant(targets: &semio_framework_value::list::PagedList<semio_framework_value::paged::PagedUtf8<{usize::MAX}>, {usize::MAX}>) -> Result<(), &'static str> {
    if targets.is_empty() {
        return Err("a selection transform addresses at least one layer");
    }
    let unique: std::collections::BTreeSet<_> = targets.iter().collect();
    if unique.len() != targets.len() {
        return Err("a selection transform addresses every layer once");
    }
    Ok(())
}

/// 🚨️ The `mutation.partial` warnings of a layer selection transform: addressed ids missing from the drawing, locked or hidden,
/// or placed through a singular transform — each skipped, never fatal.
pub fn drawing_selection_partial(targets: &semio_framework_value::list::PagedList<semio_framework_value::paged::PagedUtf8<{usize::MAX}>, {usize::MAX}>, placed: &[DrawingPlacedLayer<'_>], locked: Vec<String>, singular: Vec<String>) -> Vec<protocol::MutationMessage> {
    let missing: Vec<String> = targets.iter().filter(|id| !placed.iter().any(|entry| &layer_base(entry.layer).id == *id)).map(|id| id.to_string_owner()).collect();
    [(missing, "not in this drawing"), (locked, "locked or hidden"), (singular, "placed through a singular transform")]
        .into_iter()
        .filter(|(skipped, _)| !skipped.is_empty())
        .map(|(skipped, reason)| protocol::MutationMessage::warning("mutation.partial", format!("{} of {} target(s) skipped ({reason}): {}", skipped.len(), targets.len(), skipped.join(", "))).at(skipped))
        .collect()
}

/// ↔️ A layer's transform after its world-space drag by `delta` through its parent's world matrix — only the origin moves.
pub fn drawing_dragged_transform(source: &crate::DrawingTransform, parent: [f64; 6], delta: [f64; 2]) -> Option<crate::DrawingTransform> {
    let [x, y, scale_x, scale_y, rotation] = crate::schema::geometry::translation::translate([source.x, source.y, source.scale_x, source.scale_y, source.rotation], parent, delta)?;
    Some(crate::DrawingTransform { x, y, scale_x, scale_y, rotation, shear: source.shear })
}

/// 🌐️ A layer's transform after the world-space affine `motion` through its parent's world matrix:
/// `parent⁻¹ · motion · parent · local`.
pub fn drawing_moved_transform(source: &crate::DrawingTransform, parent: [f64; 6], motion: [f64; 6]) -> Option<crate::DrawingTransform> {
    use crate::schema::geometry::{inverse, multiply};
    let local = multiply(inverse(parent)?, multiply(motion, multiply(parent, crate::schema::drawing_transform_to_matrix(source))));
    local.iter().all(|value| value.is_finite()).then(|| crate::schema::drawing_matrix_to_transform(local))
}

/// 🔄️ The world-space rotation by `angle` radians about `(pivot_x, pivot_y)`.
pub fn drawing_rotation_matrix(pivot_x: f64, pivot_y: f64, angle: f64) -> [f64; 6] {
    let (s, c) = angle.sin_cos();
    [c, s, -s, c, pivot_x - c * pivot_x + s * pivot_y, pivot_y - s * pivot_x - c * pivot_y]
}

/// 📐️ The world-space scaling by `(scale_x, scale_y)` about `(pivot_x, pivot_y)`.
pub fn drawing_scaling_matrix(pivot_x: f64, pivot_y: f64, scale_x: f64, scale_y: f64) -> [f64; 6] {
    [scale_x, 0.0, 0.0, scale_y, pivot_x * (1.0 - scale_x), pivot_y * (1.0 - scale_y)]
}

/// 🔢️ A selection label's number, `(en, de)`: two decimals at most, trailing zeros trimmed, a German decimal comma.
pub fn drawing_label_number(value: f64) -> (String, String) {
    let rounded = (value * 100.0).round() / 100.0;
    let text = format!("{:.2}", if rounded == 0.0 { 0.0 } else { rounded });
    let en = text.trim_end_matches('0').trim_end_matches('.').to_string();
    let de = en.replace('.', ",");
    (en, de)
}

/// 🔠️ A selection label's counted noun, `(en, de)`: "1 layer" / "1 Ebene", "3 layers" / "3 Ebenen".
pub fn drawing_label_layers(count: usize) -> (String, String) {
    match count {
        1 => ("1 layer".to_string(), "1 Ebene".to_string()),
        count => (format!("{count} layers"), format!("{count} Ebenen")),
    }
}
//#endregion 🔖️SelectionTransform
