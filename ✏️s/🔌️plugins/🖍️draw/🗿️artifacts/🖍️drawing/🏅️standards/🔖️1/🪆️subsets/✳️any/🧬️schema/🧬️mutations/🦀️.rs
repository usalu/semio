//! 🧬️ Drawing artifact — semantic document mutation dispatch enum. Every variant is a single-field
//! tuple wrapping a handcrafted `protocol::MutationKind` payload (see the `🧬️mutations/<slug>/`
//! triad leaves); `#[derive(dsl::Mutations)]` generates `impl protocol::Mutation<DrawingSnapshot>`
//! and `impl protocol::SemanticMutation<DrawingSnapshot>` from those payloads — no hand-written
//! apply/diff/inverse dispatch here.

use crate::schema::{find_drawing_layer, hex_to_rgba, layer_base};
use crate::{DrawingLayerNode, DrawingSnapshot, FillStyle, StrokeStyle};

//#region 🔖️Mutations
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations)]
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
/// ⌨️ Decode inspector input according to its field, preserving numeric-looking text.
pub fn parse_layer_field_input(field: &str, value: &str) -> semio_framework_value::DslValue {
    let parsed = semio_framework_pack_json::parse(value, semio_framework_pack_json::JsonMemberPolicy::Reject).ok().map(|parsed| semio_framework_pack_json::to_dsl_value(&parsed));
    if matches!(field, "textContent" | "name" | "blendMode" | "fillColor" | "fillRule" | "strokeColor" | "strokeCap" | "strokeJoin" | "strokeDash" | "booleanOperation") {
        if let Some(semio_framework_value::DslValue::String(text)) = parsed { return semio_framework_value::DslValue::String(text); }
        return semio_framework_value::DslValue::String(value.into());
    }
    parsed.unwrap_or_else(|| semio_framework_value::DslValue::String(value.into()))
}

/// 🎛️ Generic single-field layer editor bridge (properties panel / bulk patch commands) — maps a
/// wire `field` name + JSON `value` onto the one semantic mutation that owns that field. Returns
/// `None` for an unknown field or a field that doesn't apply to `layer`'s kind.
pub fn drawing_op_for_layer_field(doc: &DrawingSnapshot, layer_id: &str, field: &str, value: &semio_framework_value::DslValue) -> Option<DrawingMutation> {
    let layer = find_drawing_layer(doc, layer_id)?;
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
        "fillColor" | "strokeColor" => {
            let color = value.as_str()?.strip_prefix('#')?;
            if !matches!(color.len(), 3 | 6) || !color.bytes().all(|byte| byte.is_ascii_hexdigit()) { return None; }
        }
        "isolation" => {if !matches!(layer,DrawingLayerNode::Group(_)){return None;}value.as_bool()?;}
        "fillRule" => { crate::FillRule::parse(value.as_str()?).ok()?; }
        "strokeCap" => { crate::StrokeCap::parse(value.as_str()?).ok()?; }
        "strokeJoin" => { crate::StrokeJoin::parse(value.as_str()?).ok()?; }
        "strokeDash" => { crate::schema::stroke::parse_stroke_dash(value.as_str()?).ok()?; }
        "blendMode" => { if !crate::DRAWING_BLEND_MODES.contains(&value.as_str()?) { return None; } }
        "booleanOperation" => { if !matches!(layer, DrawingLayerNode::Boolean(_)) || !matches!(value.as_str()?, "union" | "intersect" | "subtract" | "exclude") { return None; } }
        _ => return None,
    }
    let operation = match field {
        "name" => rename_layer(layer_id.into(), value.as_str().unwrap_or("").into()),
        "textContent" | "textSize" => {
            let DrawingLayerNode::Text(text) = layer else { return None; };
            update_text(layer_id.into(), if field == "textContent" { value.as_str()?.into() } else { text.content.clone() }, if field == "textSize" { finite()? } else { text.size })
        }
        "opacity" => set_layer_opacity(layer_id.into(), value.as_f64().unwrap_or(1.0)),
        "visible" => set_layer_visible(layer_id.into(), value.as_bool().unwrap_or(true)),
        "locked" => set_layer_locked(layer_id.into(), value.as_bool().unwrap_or(false)),
        "blendMode" => set_layer_blend_mode(layer_id.into(), value.as_str().unwrap_or("normal").into()),
        "booleanOperation" => set_layer_boolean_operation(layer_id.into(), value.as_str().unwrap_or("union").into()),
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
            update_layer_transform(layer_id.into(), transform)
        }
        "fillColor" => {
            let alpha = layer_base(layer).attributes.fill.as_ref().map_or(1.0, |fill| match fill {
                FillStyle::Solid { color } => color[3],
                FillStyle::LinearGradient { .. } | FillStyle::RadialGradient { .. } => 1.0,
            });
            replace_layer_fill(layer_id.into(), Some(FillStyle::Solid { color: hex_to_rgba(value.as_str().unwrap_or("#000000"), alpha) }))
        }
        "isolation" => set_group_isolation(layer_id.into(),value.as_bool()?),
        "fillRule" => set_layer_fill_rule(layer_id.into(),crate::FillRule::parse(value.as_str()?).ok()?),
        "fillEnabled" => replace_layer_fill(layer_id.into(), if value.as_bool()? { Some(layer_base(layer).attributes.fill.clone().unwrap_or(FillStyle::Solid { color: [0.0, 0.0, 0.0, 1.0] })) } else { None }),
        "strokeEnabled" => replace_layer_stroke(layer_id.into(), if value.as_bool()? { Some(layer_base(layer).attributes.stroke.clone().unwrap_or(StrokeStyle { color: [0.0, 0.0, 0.0, 1.0], width: 1.0, cap: crate::StrokeCap::Butt, join: crate::StrokeJoin::Miter, dash: None })) } else { None }),
        "strokeWidth" | "strokeColor" | "strokeCap" | "strokeJoin" | "strokeDash" => {
            let mut stroke = layer_base(layer).attributes.stroke.clone().unwrap_or(StrokeStyle { color: [0.0, 0.0, 0.0, 1.0], width: 1.0, cap: crate::StrokeCap::Butt, join: crate::StrokeJoin::Miter, dash: None });
            match field {
                "strokeWidth" => stroke.width = finite()?,
                "strokeColor" => stroke.color = hex_to_rgba(value.as_str()?, stroke.color[3]),
                "strokeCap" => stroke.cap = crate::StrokeCap::parse(value.as_str()?).ok()?,
                "strokeJoin" => stroke.join = crate::StrokeJoin::parse(value.as_str()?).ok()?,
                _ => stroke.dash = crate::schema::stroke::parse_stroke_dash(value.as_str()?).ok()?,
            }
            replace_layer_stroke(layer_id.into(), Some(stroke))
        }
        "traceThreshold" => {
            let DrawingLayerNode::Trace(trace) = layer else { return None };
            let mut params = trace.params.clone();
            params.threshold = value.as_f64().unwrap_or(0.5);
            update_layer_trace_params(layer_id.into(), params)
        }
        "traceSimplify" => {
            let DrawingLayerNode::Trace(trace) = layer else { return None };
            let mut params = trace.params.clone();
            params.simplify_epsilon = value.as_f64().unwrap_or(1.5);
            update_layer_trace_params(layer_id.into(), params)
        }
        _ => return None,
    };
    Some(operation)
}

/// 🩹 Applies one field patch directly to `doc` — used by callers that don't need the mutation
/// value itself (`drawing_op_for_layer_field` is the undoable/command-facing entry point).
pub fn patch_layer_field(doc: &DrawingSnapshot, layer_id: &str, field: &str, value: &semio_framework_value::DslValue) -> protocol::MutationApplyResult<DrawingSnapshot> {
    use protocol::{Mutation, MutationDiff};
    match drawing_op_for_layer_field(doc, layer_id, field, value) {
        Some(operation) => operation.diff(doc).diff().apply(doc).map_err(|error| error.under(["layers", layer_id])),
        None => Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "layer field cannot be patched").at(["layers", layer_id, field])),
    }
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
/// ▶️ Applies `mutation` to `snapshot` through its own diff — the artifact's single apply entry
/// point (mirrors dag's `apply_dag_mutation`/puzzle5d's `apply_puzzle5d_mutation`). A rejecting
/// diff carries an empty `DrawingDiff`, so the snapshot is left untouched and `Ok(())` is still
/// returned; read [`protocol::MutationOutcome::messages`] to distinguish the two.
pub fn apply_drawing_mutation(snapshot: &mut DrawingSnapshot, mutation: &DrawingMutation) -> protocol::MutationApplyResult<()> {
    use store::MutationDiff;
    let next = <DrawingMutation as protocol::Mutation<DrawingSnapshot>>::diff(mutation, snapshot).diff().apply(snapshot)?;
    *snapshot = next;
    Ok(())
}

/// ↩️ The typed mutation steps that undo `mutation` against `snapshot`.
pub fn inverse_drawing_mutation(snapshot: &DrawingSnapshot, mutation: &DrawingMutation) -> Result<Vec<DrawingMutation>, semio_framework_value::ValueError> {
    Ok({
    <DrawingMutation as protocol::Mutation<DrawingSnapshot>>::inverse(mutation, snapshot)?

    })
}
//#endregion 🔖️Apply

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🌉️ExternalCodecBridge
/// 🧩️ Decodes one committed `📸️snapshot/⬅️before/🔣️.json` document together with the
/// `🦠️mutation/🔣️.json` payload beside it — the same bytes the leaf's own fixture test
/// reads — into real typed values.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn bridge_decode_pair(snapshot_json: &str, mutation_json: &str) -> Result<(DrawingSnapshot, DrawingMutation), String> {
    let snapshot: DrawingSnapshot = semio_framework_pack_json::from_json_str(snapshot_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| format!("the committed drawing snapshot JSON does not decode: {error}"))?;
    let mutation: DrawingMutation = semio_framework_pack_json::from_json_str(mutation_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| format!("the committed drawing mutation JSON does not decode: {error}"))?;
    Ok((snapshot, mutation))
}

/// ▶️ One diff-and-apply step, keeping the diagnostic codes the outcome raised — a rejected or
/// no-op kind is a RESULT this bridge reports, never an error it swallows.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn bridge_step(snapshot: &DrawingSnapshot, mutation: &DrawingMutation) -> Result<(DrawingSnapshot, Vec<String>), String> {
    use protocol::{Mutation, MutationDiff};
    let outcome = <DrawingMutation as Mutation<DrawingSnapshot>>::diff(mutation, snapshot);
    let messages: Vec<String> = outcome.messages().iter().map(|message| message.code.0.clone()).collect();
    match MutationDiff::apply(outcome.diff(), snapshot) {
        Ok(next) => Ok((next, messages)),
        Err(error) => Err(format!("{error:?}")),
    }
}

/// 📤️ The bridge's answer shape: the resulting document beside the codes it raised, so a caller
/// that cannot name `protocol::MutationOutcome` can still tell an application from a refusal.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn bridge_render(snapshot: &DrawingSnapshot, messages: &[String]) -> String {
    let report = semio_framework_value::DslValue::object([("snapshot".to_string(), semio_framework_value::ToValue::to_value(snapshot)), ("messages".to_string(), semio_framework_value::ToValue::to_value(messages))]);
    semio_framework_pack_json::to_json_string(&report)
}

/// 🌉️ Applies one committed mutation payload to one committed before-document and answers
/// `{"snapshot": …, "messages": [ … ]}`.
///
/// The bridge exists because the generated Rust test host links only `semio-repo-test-host` and,
/// behind its `sut` feature, this crate — `dsl`, `protocol` and `store` are private
/// extern-crate aliases (`🦀️.rs`) and cannot be named from a case adapter. Same shape and same
/// reason as `🗄️stdio`'s `decode_semio_mesh_mutation_json`/`apply_semio_mesh_mutation` pair.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_drawing_mutation_json(snapshot_json: &str, mutation_json: &str) -> Result<String, String> {
    let (snapshot, mutation) = bridge_decode_pair(snapshot_json, mutation_json)?;
    let (applied, messages) = bridge_step(&snapshot, &mutation)?;
    Ok(bridge_render(&applied, &messages))
}

/// ↩️ Applies one committed mutation payload and then EVERY step of its own computed inverse,
/// answering in the same shape — the metamorphic half of the evidence the `drawing-mutation-semantics` no-oracle
/// decision rests on. The inverse is computed against the PRE-mutation document, which is the only
/// state that carries what a delete removed.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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

/// 🔁️ Parses the committed `.dsl.semio` example, prints it back and parses that, answering
/// `{"printed": …, "snapshot": …, "reparsed": …}` so a caller can weigh the identity law's two
/// halves — the bytes against the committed artifact, and the projection against itself.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn round_trip_drawing_dsl(text: &str) -> Result<String, String> {
    use store::ArtifactDsl;
    let parsed = <DrawingSnapshot as ArtifactDsl>::parse_dsl(text).map_err(|error| format!("the committed drawing example does not parse: {error:?}"))?;
    let printed = <DrawingSnapshot as ArtifactDsl>::print_dsl(&parsed);
    let reparsed = <DrawingSnapshot as ArtifactDsl>::parse_dsl(&printed).map_err(|error| format!("the reprinted drawing document does not parse: {error:?}"))?;
    let report = semio_framework_value::DslValue::object([("printed".to_string(), semio_framework_value::ToValue::to_value(&printed)), ("snapshot".to_string(), semio_framework_value::ToValue::to_value(&parsed)), ("reparsed".to_string(), semio_framework_value::ToValue::to_value(&reparsed))]);
    Ok(semio_framework_pack_json::to_json_string(&report))
}
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
    pub addressed_ancestor: Option<&'a str>,
}

/// 🗂️ Every layer of `base` whose id is in `ids`, in document (pre-)order, placed in the world.
pub fn drawing_placed_layers<'a>(base: &'a DrawingSnapshot, ids: &[&str]) -> Vec<DrawingPlacedLayer<'a>> {
    let mut placed = Vec::new();
    let mut stack: Vec<(std::slice::Iter<'a, DrawingLayerNode>, [f64; 6], bool, Option<&'a str>)> = vec![(base.layers.iter(), DRAWING_IDENTITY_MATRIX, true, None)];
    while let Some((layers, parent, editable, ancestor)) = stack.last_mut() {
        let Some(layer) = layers.next() else {
            stack.pop();
            continue;
        };
        let (parent, ancestor) = (*parent, *ancestor);
        let layer_base = layer_base(layer);
        let editable = *editable && layer_base.visible && !layer_base.locked;
        let addressed = ids.contains(&layer_base.id.as_str());
        if addressed {
            placed.push(DrawingPlacedLayer { layer, parent, editable, addressed_ancestor: ancestor });
        }
        if let DrawingLayerNode::Group(group) = layer {
            let matrix = crate::schema::geometry::multiply(parent, crate::schema::drawing_transform_to_matrix(&layer_base.transform));
            stack.push((group.children.iter(), matrix, editable, if addressed { Some(layer_base.id.as_str()) } else { ancestor }));
        }
    }
    placed
}

/// 🚨️ The schema-stated target invariant every selection transform shares: at least one id, none repeated.
pub fn drawing_targets_invariant(targets: &[String]) -> Result<(), &'static str> {
    if targets.is_empty() {
        return Err("a selection transform addresses at least one layer");
    }
    let unique: std::collections::BTreeSet<&str> = targets.iter().map(String::as_str).collect();
    if unique.len() != targets.len() {
        return Err("a selection transform addresses every layer once");
    }
    Ok(())
}

/// 🧭️ The one diff every layer selection transform builds: `place` maps a surviving layer's transform through its parent's
/// world matrix, a layer whose addressed ancestor moved moves with it, locked, hidden, missing or singular targets are
/// skipped (`mutation.partial`), none left is `mutation.target-missing`, nothing moving is `mutation.no-op`.
pub fn drawing_selection_diff(base: &DrawingSnapshot, targets: &[String], place: impl Fn(&crate::DrawingTransform, [f64; 6]) -> Option<crate::DrawingTransform>) -> protocol::MutationOutcome<crate::diff::DrawingDiff> {
    if let Err(reason) = drawing_targets_invariant(targets) {
        return protocol::MutationOutcome::fatal("mutation.invariant", reason, targets.to_vec());
    }
    let ids: Vec<&str> = targets.iter().map(String::as_str).collect();
    let placed = drawing_placed_layers(base, &ids);
    let (mut moved, mut locked, mut singular, mut patched) = (std::collections::BTreeSet::<&str>::new(), Vec::new(), Vec::new(), Vec::new());
    let mut applies = false;
    for entry in &placed {
        let source = layer_base(entry.layer);
        if entry.addressed_ancestor.is_some_and(|ancestor| moved.contains(ancestor)) {
            applies = true;
            continue;
        }
        if !entry.editable {
            locked.push(source.id.clone());
            continue;
        }
        match place(&source.transform, entry.parent) {
            Some(next) => {
                applies = true;
                moved.insert(source.id.as_str());
                if next != source.transform {
                    patched.push((source.id.clone(), next));
                }
            }
            None => singular.push(source.id.clone()),
        }
    }
    if !applies {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("none of the {} target(s) is a visible, unlocked layer this transform can place", targets.len()), targets.to_vec());
    }
    let missing: Vec<String> = targets.iter().filter(|id| !placed.iter().any(|entry| layer_base(entry.layer).id == **id)).cloned().collect();
    let partial: Vec<protocol::MutationMessage> = [(missing, "not in this drawing"), (locked, "locked or hidden"), (singular, "placed through a singular transform")]
        .into_iter()
        .filter(|(skipped, _)| !skipped.is_empty())
        .map(|(skipped, reason)| protocol::MutationMessage::warning("mutation.partial", format!("{} of {} target(s) skipped ({reason}): {}", skipped.len(), targets.len(), skipped.join(", "))).at(skipped))
        .collect();
    if patched.is_empty() {
        return protocol::MutationOutcome::new(crate::diff::DrawingDiff::default()).absorb_messages(partial.into_iter().chain([protocol::MutationMessage::warning("mutation.no-op", "no layer changes its transform").at(targets.to_vec())]));
    }
    protocol::MutationOutcome::new(crate::diff::diff_set_layer_transforms(patched)).absorb_messages(partial)
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

/// ↩️ Exact base-derived inverse of a layer selection transform: `update-layer-transform` back to every BASE transform its
/// forward outcome patches — absolute setters, never a negated motion that would accumulate float error.
pub fn drawing_selection_inverse(base: &DrawingSnapshot, outcome: protocol::MutationOutcome<crate::diff::DrawingDiff>) -> Result<Vec<DrawingMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    outcome.diff().layers.iter().flat_map(|delta| delta.patched.iter()).filter_map(|entry| find_drawing_layer(base, &entry.id).map(|layer| update_layer_transform(entry.id.clone(), layer_base(layer).transform.clone()))).collect()

    })())
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
