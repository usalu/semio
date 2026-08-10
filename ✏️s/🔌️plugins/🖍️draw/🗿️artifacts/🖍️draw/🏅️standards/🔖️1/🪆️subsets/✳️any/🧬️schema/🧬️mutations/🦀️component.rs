//! 🧬️ Draw artifact — document mutation dispatch enum + apply helpers.

use crate::artifacts::draw::diff::{
    diff_add_layer, diff_from_snapshot, diff_remove_layer, diff_set_boolean_operation, diff_set_fill,
    diff_set_layer_blend_mode, diff_set_layer_locked, diff_set_layer_name, diff_set_layer_opacity,
    diff_set_layer_transform, diff_set_layer_visible, diff_set_snapshot, diff_set_stroke,
    diff_set_trace_params, DrawDiff,
};
use crate::artifacts::draw::engine::{
    clone_draw_layer_node, extract_layer_node, find_draw_layer, find_draw_layer_location, hex_to_rgba,
    insert_layer, layer_base, layer_base_mut, mutate_draw_layer, remove_layer_from_tree,
};
use crate::artifacts::draw::{DrawLayerNode, DrawSnapshot, FillStyle, StrokeStyle};
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum DrawMutation {
    SetLayerVisible {
        layer_id: String,
        visible: bool,
    },
    SetLayerLocked {
        layer_id: String,
        locked: bool,
    },
    SetLayerOpacity {
        layer_id: String,
        opacity: f64,
    },
    SetLayerBlendMode {
        layer_id: String,
        blend_mode: String,
    },
    SetLayerName {
        layer_id: String,
        name: String,
    },
    SetLayerTransform {
        layer_id: String,
        #[dsl(block)]
        transform: crate::artifacts::draw::DrawTransform,
    },
    SetFill {
        layer_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[dsl(statements, block)]
        fill: Option<FillStyle>,
    },
    SetStroke {
        layer_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[dsl(block)]
        stroke: Option<StrokeStyle>,
    },
    SetBooleanOperation {
        layer_id: String,
        boolean_operation: String,
    },
    SetTraceParams {
        layer_id: String,
        #[dsl(block)]
        params: crate::artifacts::draw::DrawTraceParams,
    },
    AddLayer {
        #[serde(skip_serializing_if = "Option::is_none")]
        parent_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        index: Option<usize>,
        #[dsl(statements)]
        layer: Box<DrawLayerNode>,
    },
    DuplicateLayer {
        layer_id: String,
    },
    RemoveLayer {
        layer_id: String,
    },
    ReorderLayer {
        layer_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        parent_id: Option<String>,
        index: usize,
    },
    SetSnapshot {
        #[dsl(block)]
        snapshot: DrawSnapshot,
    },
}
pub fn apply_draw_edit_mutation(doc: &DrawSnapshot, edit: &DrawMutation) -> DrawSnapshot {
    match edit {
        DrawMutation::SetSnapshot { snapshot } => snapshot.clone(),
        DrawMutation::SetLayerVisible { layer_id, visible } => mutate_draw_layer(doc, layer_id, |layer| {
            layer_base_mut(layer).visible = *visible;
        }),
        DrawMutation::SetLayerLocked { layer_id, locked } => mutate_draw_layer(doc, layer_id, |layer| {
            layer_base_mut(layer).locked = *locked;
        }),
        DrawMutation::SetLayerOpacity { layer_id, opacity } => mutate_draw_layer(doc, layer_id, |layer| {
            layer_base_mut(layer).opacity = *opacity;
        }),
        DrawMutation::SetLayerBlendMode { layer_id, blend_mode } => mutate_draw_layer(doc, layer_id, |layer| {
            layer_base_mut(layer).blend_mode = blend_mode.clone();
        }),
        DrawMutation::SetLayerName { layer_id, name } => mutate_draw_layer(doc, layer_id, |layer| {
            layer_base_mut(layer).name = name.clone();
        }),
        DrawMutation::SetLayerTransform { layer_id, transform } => mutate_draw_layer(doc, layer_id, |layer| {
            layer_base_mut(layer).transform = transform.clone();
        }),
        DrawMutation::SetFill { layer_id, fill } => mutate_draw_layer(doc, layer_id, |layer| {
            layer_base_mut(layer).attributes.fill = fill.clone();
        }),
        DrawMutation::SetStroke { layer_id, stroke } => mutate_draw_layer(doc, layer_id, |layer| {
            layer_base_mut(layer).attributes.stroke = stroke.clone();
        }),
        DrawMutation::SetBooleanOperation { layer_id, boolean_operation } => mutate_draw_layer(doc, layer_id, |layer| {
            if let DrawLayerNode::Boolean(boolean) = layer {
                boolean.operation = boolean_operation.clone();
            }
        }),
        DrawMutation::SetTraceParams { layer_id, params } => mutate_draw_layer(doc, layer_id, |layer| {
            if let DrawLayerNode::Trace(trace) = layer {
                trace.params = params.clone();
            }
        }),
        DrawMutation::AddLayer { parent_id, index, layer } => {
            let mut next = doc.clone();
            let at = index.unwrap_or(next.layers.len());
            insert_layer(&mut next.layers, parent_id.as_deref(), at, layer.as_ref().clone());
            next
        }
        DrawMutation::DuplicateLayer { layer_id: source_id } => {
            if let Some(layer) = find_draw_layer(doc, source_id).cloned() {
                let duplicate = clone_draw_layer_node(&layer, " copy");
                let mut next = doc.clone();
                if let Some(location) = find_draw_layer_location(doc, source_id) {
                    insert_layer(&mut next.layers, location.parent_id.as_deref(), location.index + 1, duplicate);
                } else {
                    next.layers.push(duplicate);
                }
                next
            } else {
                doc.clone()
            }
        }
        DrawMutation::RemoveLayer { layer_id } => {
            let mut next = doc.clone();
            remove_layer_from_tree(&mut next.layers, layer_id);
            next
        }
        DrawMutation::ReorderLayer { layer_id, parent_id, index } => {
            let mut next = doc.clone();
            if let Some(node) = extract_layer_node(&mut next.layers, layer_id) {
                insert_layer(&mut next.layers, parent_id.as_deref(), *index, node);
            }
            next
        }
    }
}

pub fn draw_op_for_layer_field(doc: &DrawSnapshot, layer_id: &str, field: &str, value: &serde_json::Value) -> Option<DrawMutation> {
    let layer = find_draw_layer(doc, layer_id)?;
    let operation = match field {
        "name" => DrawMutation::SetLayerName { layer_id: layer_id.into(), name: value.as_str().unwrap_or("").into() },
        "opacity" => DrawMutation::SetLayerOpacity { layer_id: layer_id.into(), opacity: value.as_f64().unwrap_or(1.0) },
        "visible" => DrawMutation::SetLayerVisible { layer_id: layer_id.into(), visible: value.as_bool().unwrap_or(true) },
        "locked" => DrawMutation::SetLayerLocked { layer_id: layer_id.into(), locked: value.as_bool().unwrap_or(false) },
        "blendMode" => DrawMutation::SetLayerBlendMode { layer_id: layer_id.into(), blend_mode: value.as_str().unwrap_or("normal").into() },
        "booleanOperation" => DrawMutation::SetBooleanOperation { layer_id: layer_id.into(), boolean_operation: value.as_str().unwrap_or("union").into() },
        "transformX" | "transformY" | "transformScaleX" | "transformScaleY" | "transformRotation" => {
            let mut transform = layer_base(layer).transform.clone();
            match field {
                "transformX" => transform.x = value.as_f64().unwrap_or(0.0),
                "transformY" => transform.y = value.as_f64().unwrap_or(0.0),
                "transformScaleX" => transform.scale_x = value.as_f64().unwrap_or(1.0),
                "transformScaleY" => transform.scale_y = value.as_f64().unwrap_or(1.0),
                _ => transform.rotation = value.as_f64().unwrap_or(0.0),
            }
            DrawMutation::SetLayerTransform { layer_id: layer_id.into(), transform }
        }
        "fillColor" => {
            let alpha = layer_base(layer)
                .attributes
                .fill
                .as_ref()
                .map_or(1.0, |fill| match fill {
                    FillStyle::Solid { color } => color[3],
                    FillStyle::LinearGradient { .. } | FillStyle::RadialGradient { .. } => 1.0,
                });
            DrawMutation::SetFill { layer_id: layer_id.into(), fill: Some(FillStyle::Solid { color: hex_to_rgba(value.as_str().unwrap_or("#000000"), alpha) }) }
        }
        "strokeWidth" => {
            let stroke = layer_base(layer).attributes.stroke.clone().unwrap_or(StrokeStyle { color: [0.0, 0.0, 0.0, 1.0], width: 1.0, cap: "butt".into(), join: "miter".into(), dash: None });
            DrawMutation::SetStroke { layer_id: layer_id.into(), stroke: Some(StrokeStyle { width: value.as_f64().unwrap_or(1.0), ..stroke }) }
        }
        "traceThreshold" => {
            let DrawLayerNode::Trace(trace) = layer else { return None };
            let mut params = trace.params.clone();
            params.threshold = value.as_f64().unwrap_or(0.5);
            DrawMutation::SetTraceParams { layer_id: layer_id.into(), params }
        }
        "traceSimplify" => {
            let DrawLayerNode::Trace(trace) = layer else { return None };
            let mut params = trace.params.clone();
            params.simplify_epsilon = value.as_f64().unwrap_or(1.5);
            DrawMutation::SetTraceParams { layer_id: layer_id.into(), params }
        }
        _ => return None,
    };
    Some(operation)
}

pub fn patch_layer_field(doc: &DrawSnapshot, layer_id: &str, field: &str, value: &serde_json::Value) -> DrawSnapshot {
    match draw_op_for_layer_field(doc, layer_id, field, value) {
        Some(operation) => apply_draw_edit_mutation(doc, &operation),
        None => doc.clone(),
    }
}


/// ↩️ Computes the inverse mutations from pre-state (document snapshot).
pub fn inverse_draw_mutation(snapshot: &DrawSnapshot, _mutation: &DrawMutation) -> Vec<DrawMutation> {
    vec![DrawMutation::SetSnapshot { snapshot: snapshot.clone() }]
}

//#region 🔖️MutationImpl
impl Mutation<DrawSnapshot> for DrawMutation {
    type Diff = DrawDiff;

    fn diff(&self, snapshot: &DrawSnapshot) -> DrawDiff {
        match self {
            DrawMutation::SetSnapshot { snapshot } => diff_set_snapshot(snapshot),
            DrawMutation::SetLayerVisible { layer_id, visible } => diff_set_layer_visible(layer_id, *visible),
            DrawMutation::SetLayerLocked { layer_id, locked } => diff_set_layer_locked(layer_id, *locked),
            DrawMutation::SetLayerName { layer_id, name } => diff_set_layer_name(layer_id, name),
            DrawMutation::SetLayerOpacity { layer_id, opacity } => diff_set_layer_opacity(layer_id, *opacity),
            DrawMutation::SetLayerBlendMode { layer_id, blend_mode } => diff_set_layer_blend_mode(layer_id, blend_mode),
            DrawMutation::SetLayerTransform { layer_id, transform } => diff_set_layer_transform(layer_id, transform),
            DrawMutation::SetFill { layer_id, fill } => diff_set_fill(layer_id, fill),
            DrawMutation::SetStroke { layer_id, stroke } => diff_set_stroke(layer_id, stroke),
            DrawMutation::SetBooleanOperation { layer_id, boolean_operation } => {
                diff_set_boolean_operation(layer_id, boolean_operation)
            }
            DrawMutation::SetTraceParams { layer_id, params } => diff_set_trace_params(layer_id, params),
            DrawMutation::AddLayer { parent_id, index: _, layer } if parent_id.is_none() => {
                diff_add_layer(layer.as_ref().clone())
            }
            DrawMutation::RemoveLayer { layer_id } => diff_remove_layer(layer_id),
            _ => diff_from_snapshot(apply_draw_edit_mutation(snapshot, self)),
        }
    }

    fn inverse(&self, snapshot: &DrawSnapshot) -> Vec<Self> {
        inverse_draw_mutation(snapshot, self)
    }
}
//#endregion 🔖️MutationImpl

pub use super::set_layer_visible::mutation::{set_layer_visible, SetLayerVisible};
pub use super::set_layer_locked::mutation::{set_layer_locked, SetLayerLocked};
pub use super::set_layer_opacity::mutation::{set_layer_opacity, SetLayerOpacity};
pub use super::set_layer_blend_mode::mutation::{set_layer_blend_mode, SetLayerBlendMode};
pub use super::set_layer_name::mutation::{set_layer_name, SetLayerName};
pub use super::set_layer_transform::mutation::{set_layer_transform, SetLayerTransform};
pub use super::set_fill::mutation::{set_fill, SetFill};
pub use super::set_stroke::mutation::{set_stroke, SetStroke};
pub use super::set_boolean_operation::mutation::{set_boolean_operation, SetBooleanOperation};
pub use super::set_trace_params::mutation::{set_trace_params, SetTraceParams};
pub use super::add_layer::mutation::{add_layer, AddLayer};
pub use super::duplicate_layer::mutation::{duplicate_layer, DuplicateLayer};
pub use super::remove_layer::mutation::{remove_layer, RemoveLayer};
pub use super::reorder_layer::mutation::{reorder_layer, ReorderLayer};
pub use super::set_snapshot::mutation::{set_snapshot, SetSnapshot};
