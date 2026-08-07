//! 🔧️ Draw artifact — operation enum + apply/backwards (constitutional: op).


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::draw::diff::{DrawDiff, DrawLayerBasePatch, DrawLayerTreeAdd, DrawLayerTreePatch};
use crate::artifacts::draw::engine::{clone_draw_layer_node, extract_layer_node, find_draw_layer, find_draw_layer_location, hex_to_rgba, insert_layer, layer_base, layer_base_mut, mutate_draw_layer, remove_layer_from_tree};
use crate::artifacts::draw::{DrawDocument, DrawLayerNode, FillStyle, StrokeStyle};
use protocol::Operation;
use serde::{Deserialize, Serialize};

//#region 🔖️Types
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
#[serde(tag = "operation", rename_all = "camelCase")]
pub enum DrawOperation {
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
    SetDocument {
        #[dsl(block)]
        document: DrawDocument,
    },
}

//#region 🔖️OpCodec
/// 🎞️ Handcrafted OpText (P6).
impl protocol::OpText for DrawOperation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(
                    line,
                    &spec_fn(),
                    &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline },
                )?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

/// 🎯️ Handcrafted OpBinary (P6).
impl protocol::OpBinary for DrawOperation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let ordinal = variants.iter().position(|(k, _)| *k == keyword).ok_or(protocol::ProtocolError::Malformed {
            what: "op variant",
            offset: 0,
            detail: format!("keyword {keyword:?} is not a declared variant"),
        })?;
        let spec = (variants[ordinal].1)();
        let body = store::pack_rt::encode_record_body(&spec, &record, &store::PackEncodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        let mut out = Vec::with_capacity(body.len() + 3);
        out.push(OP_BINARY_FORMAT);
        store::pack_rt::write_varint_u64(&mut out, ordinal as u64);
        out.extend_from_slice(&body);
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let mut reader = store::pack_rt::ByteReader::new(bytes);
        let format = reader.read_u8()?;
        if format != OP_BINARY_FORMAT {
            return Err(protocol::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {format}") });
        }
        let ordinal = reader.read_varint_u64()?;
        let variants = <Self as dsl::DslVariants>::variants();
        let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or(protocol::ProtocolError::Malformed {
            what: "op variant",
            offset: 1,
            detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()),
        })?;
        let spec = spec_fn();
        let body = &bytes[reader.position()..];
        let (record, _report) = store::pack_rt::decode_record_body(body, &spec, &store::PackDecodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        <Self as dsl::DslVariants>::from_named_record(keyword, &record).map_err(|error| protocol::ProtocolError::Malformed {
            what: "op record",
            offset: reader.position() as u64,
            detail: error.to_string(),
        })
    }
}
//#endregion 🔖️OpCodec


pub fn apply_draw_edit_operation(doc: &DrawDocument, edit: &DrawOperation) -> DrawDocument {
    match edit {
        DrawOperation::SetDocument { document } => document.clone(),
        DrawOperation::SetLayerVisible { layer_id, visible } => mutate_draw_layer(doc, layer_id, |layer| {
            layer_base_mut(layer).visible = *visible;
        }),
        DrawOperation::SetLayerLocked { layer_id, locked } => mutate_draw_layer(doc, layer_id, |layer| {
            layer_base_mut(layer).locked = *locked;
        }),
        DrawOperation::SetLayerOpacity { layer_id, opacity } => mutate_draw_layer(doc, layer_id, |layer| {
            layer_base_mut(layer).opacity = *opacity;
        }),
        DrawOperation::SetLayerBlendMode { layer_id, blend_mode } => mutate_draw_layer(doc, layer_id, |layer| {
            layer_base_mut(layer).blend_mode = blend_mode.clone();
        }),
        DrawOperation::SetLayerName { layer_id, name } => mutate_draw_layer(doc, layer_id, |layer| {
            layer_base_mut(layer).name = name.clone();
        }),
        DrawOperation::SetLayerTransform { layer_id, transform } => mutate_draw_layer(doc, layer_id, |layer| {
            layer_base_mut(layer).transform = transform.clone();
        }),
        DrawOperation::SetFill { layer_id, fill } => mutate_draw_layer(doc, layer_id, |layer| {
            layer_base_mut(layer).attributes.fill = fill.clone();
        }),
        DrawOperation::SetStroke { layer_id, stroke } => mutate_draw_layer(doc, layer_id, |layer| {
            layer_base_mut(layer).attributes.stroke = stroke.clone();
        }),
        DrawOperation::SetBooleanOperation { layer_id, boolean_operation } => mutate_draw_layer(doc, layer_id, |layer| {
            if let DrawLayerNode::Boolean(boolean) = layer {
                boolean.operation = boolean_operation.clone();
            }
        }),
        DrawOperation::SetTraceParams { layer_id, params } => mutate_draw_layer(doc, layer_id, |layer| {
            if let DrawLayerNode::Trace(trace) = layer {
                trace.params = params.clone();
            }
        }),
        DrawOperation::AddLayer { parent_id, index, layer } => {
            let mut next = doc.clone();
            let at = index.unwrap_or(next.layers.len());
            insert_layer(&mut next.layers, parent_id.as_deref(), at, layer.as_ref().clone());
            next
        }
        DrawOperation::DuplicateLayer { layer_id: source_id } => {
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
        DrawOperation::RemoveLayer { layer_id } => {
            let mut next = doc.clone();
            remove_layer_from_tree(&mut next.layers, layer_id);
            next
        }
        DrawOperation::ReorderLayer { layer_id, parent_id, index } => {
            let mut next = doc.clone();
            if let Some(node) = extract_layer_node(&mut next.layers, layer_id) {
                insert_layer(&mut next.layers, parent_id.as_deref(), *index, node);
            }
            next
        }
    }
}

pub fn draw_op_for_layer_field(doc: &DrawDocument, layer_id: &str, field: &str, value: &serde_json::Value) -> Option<DrawOperation> {
    let layer = find_draw_layer(doc, layer_id)?;
    let operation = match field {
        "name" => DrawOperation::SetLayerName { layer_id: layer_id.into(), name: value.as_str().unwrap_or("").into() },
        "opacity" => DrawOperation::SetLayerOpacity { layer_id: layer_id.into(), opacity: value.as_f64().unwrap_or(1.0) },
        "visible" => DrawOperation::SetLayerVisible { layer_id: layer_id.into(), visible: value.as_bool().unwrap_or(true) },
        "locked" => DrawOperation::SetLayerLocked { layer_id: layer_id.into(), locked: value.as_bool().unwrap_or(false) },
        "blendMode" => DrawOperation::SetLayerBlendMode { layer_id: layer_id.into(), blend_mode: value.as_str().unwrap_or("normal").into() },
        "booleanOperation" => DrawOperation::SetBooleanOperation { layer_id: layer_id.into(), boolean_operation: value.as_str().unwrap_or("union").into() },
        "transformX" | "transformY" | "transformScaleX" | "transformScaleY" | "transformRotation" => {
            let mut transform = layer_base(layer).transform.clone();
            match field {
                "transformX" => transform.x = value.as_f64().unwrap_or(0.0),
                "transformY" => transform.y = value.as_f64().unwrap_or(0.0),
                "transformScaleX" => transform.scale_x = value.as_f64().unwrap_or(1.0),
                "transformScaleY" => transform.scale_y = value.as_f64().unwrap_or(1.0),
                _ => transform.rotation = value.as_f64().unwrap_or(0.0),
            }
            DrawOperation::SetLayerTransform { layer_id: layer_id.into(), transform }
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
            DrawOperation::SetFill { layer_id: layer_id.into(), fill: Some(FillStyle::Solid { color: hex_to_rgba(value.as_str().unwrap_or("#000000"), alpha) }) }
        }
        "strokeWidth" => {
            let stroke = layer_base(layer).attributes.stroke.clone().unwrap_or(StrokeStyle { color: [0.0, 0.0, 0.0, 1.0], width: 1.0, cap: "butt".into(), join: "miter".into(), dash: None });
            DrawOperation::SetStroke { layer_id: layer_id.into(), stroke: Some(StrokeStyle { width: value.as_f64().unwrap_or(1.0), ..stroke }) }
        }
        "traceThreshold" => {
            let DrawLayerNode::Trace(trace) = layer else { return None };
            let mut params = trace.params.clone();
            params.threshold = value.as_f64().unwrap_or(0.5);
            DrawOperation::SetTraceParams { layer_id: layer_id.into(), params }
        }
        "traceSimplify" => {
            let DrawLayerNode::Trace(trace) = layer else { return None };
            let mut params = trace.params.clone();
            params.simplify_epsilon = value.as_f64().unwrap_or(1.5);
            DrawOperation::SetTraceParams { layer_id: layer_id.into(), params }
        }
        _ => return None,
    };
    Some(operation)
}

pub fn patch_layer_field(doc: &DrawDocument, layer_id: &str, field: &str, value: &serde_json::Value) -> DrawDocument {
    match draw_op_for_layer_field(doc, layer_id, field, value) {
        Some(operation) => apply_draw_edit_operation(doc, &operation),
        None => doc.clone(),
    }
}
//#endregion 🔖️Types

//#region 🔖️Vcs
impl Operation<DrawDocument> for DrawOperation {
    type Diff = DrawDiff;

    fn diff(&self, _projection: &DrawDocument) -> DrawDiff {
        match self {
            DrawOperation::SetDocument { document } => DrawDiff { document: Some(document.clone()), ..Default::default() },
            DrawOperation::SetLayerVisible { layer_id, visible } => DrawDiff { layer_patches: vec![DrawLayerTreePatch { layer_id: layer_id.clone(), base: DrawLayerBasePatch { visible: Some(*visible), ..Default::default() } }], ..Default::default() },
            DrawOperation::SetLayerLocked { layer_id, locked } => DrawDiff { layer_patches: vec![DrawLayerTreePatch { layer_id: layer_id.clone(), base: DrawLayerBasePatch { locked: Some(*locked), ..Default::default() } }], ..Default::default() },
            DrawOperation::SetLayerName { layer_id, name } => DrawDiff { layer_patches: vec![DrawLayerTreePatch { layer_id: layer_id.clone(), base: DrawLayerBasePatch { name: Some(name.clone()), ..Default::default() } }], ..Default::default() },
            DrawOperation::SetLayerOpacity { layer_id, opacity } => DrawDiff { layer_patches: vec![DrawLayerTreePatch { layer_id: layer_id.clone(), base: DrawLayerBasePatch { opacity: Some(*opacity), ..Default::default() } }], ..Default::default() },
            DrawOperation::SetLayerBlendMode { layer_id, blend_mode } => {
                DrawDiff { layer_patches: vec![DrawLayerTreePatch { layer_id: layer_id.clone(), base: DrawLayerBasePatch { blend_mode: Some(blend_mode.clone()), ..Default::default() } }], ..Default::default() }
            }
            DrawOperation::AddLayer { parent_id, index, layer } => DrawDiff { layers_added: vec![DrawLayerTreeAdd { parent_id: parent_id.clone(), index: *index, layer: layer.as_ref().clone() }], ..Default::default() },
            DrawOperation::RemoveLayer { layer_id } => DrawDiff { layers_removed: vec![layer_id.clone()], ..Default::default() },
            _ => DrawDiff { document: Some(apply_draw_edit_operation(_projection, self)), ..Default::default() },
        }
    }

    fn backwards(&self, projection: &DrawDocument) -> Vec<Self> {
        vec![DrawOperation::SetDocument { document: projection.clone() }]
    }
}
//#endregion 🔖️Vcs
