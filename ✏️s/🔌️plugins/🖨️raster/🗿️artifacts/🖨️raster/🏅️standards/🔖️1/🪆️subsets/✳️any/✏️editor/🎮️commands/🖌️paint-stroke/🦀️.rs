//! 🖌️ Raster play app commands — `paint-stroke`: the brush and eraser TOOL. A host keeps a stroke's samples local while
//! the pointer is down (painting its own provisional overlay) and dispatches the stroke ONCE on release, in the layer
//! image's pixels; a cancelled stroke is never dispatched. The tool is a `🔄️machine` statechart whose effects are
//! `ToolYield`s, driven by the `🛠️tool-machine` runner: one release is ONE `ToolTransaction` holding ONE parametric
//! `paint-stroke` leaf — the layer, the target and brush the session holds, the points and the pixel selection the
//! stroke is clipped to — published as one edit, one history row, whose brush and points time travel edits. Tool state
//! is never history; the yielded leaf is (design §5, §17.2, ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING).
#![allow(unexpected_cfgs)]

use crate::editor::raster::config::{RasterConfig, RasterConfigMutation};
use crate::mutations::paint_stroke::{PaintStroke as PaintStrokeLeaf, RasterBrush, RasterSelectionSpan, RasterStrokePoint, RASTER_STROKE_MAXIMUM_POINTS};
use crate::op::RasterMutation;
use crate::standards::v1::subsets::any::schema::{find_layer, layer_protection, layer_visible};
use crate::{RasterLayerNode, RasterSnapshot};
use machine::Command;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_tool_machine::{ToolMachineRunner, ToolStep, ToolYield};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Payload
/// 🪪️ The editor whose paint tool authors every stroke transaction: `<appId>#paintStroke`.
pub const RASTER_PAINT_TOOL_ID: &str = "s.raster.raster@1/*#editor#paintStroke";

/// 🖌️ One released stroke as both hosts dispatch it: the layer, the tool (`brush` or `eraser`) and the samples in the
/// target image's pixels, in drawing order, split into their x and y columns.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "paint-stroke")]
pub struct PaintStroke {
    pub layer_id: String,
    pub tool: String,
    pub xs: Vec<f64>,
    pub ys: Vec<f64>,
}
//#endregion 🔖️Payload

//#region 🎨️Leaf
fn fault(code: &'static str) -> Fault {
    Fault::from(code)
}

/// 🎨️ `#rrggbb` as four unit channels, opaque.
fn hex_color(value: &str) -> Option<Vec<f64>> {
    crate::editor::raster::config::valid_brush_color(value).then(|| {
        let rgb = u32::from_str_radix(&value[1..], 16).unwrap_or(0);
        vec![f64::from((rgb >> 16) & 0xff) / 255.0, f64::from((rgb >> 8) & 0xff) / 255.0, f64::from(rgb & 0xff) / 255.0, 1.0]
    })
}

/// ✂️ The session's pixel selection as the leaf's runs, when it selects THIS layer's target.
fn selection(config: &RasterConfig, layer_id: &str) -> Result<Option<Vec<RasterSelectionSpan>>, Fault> {
    let Some(selection) = config.pixel_selection.as_ref().filter(|selection| selection.layer_id == layer_id && selection.target == config.paint_target) else { return Ok(None) };
    let count = selection.validate()?;
    let spans = crate::editor::raster::selection::selection_spans(&selection.spans, count)?;
    Ok(Some(spans.into_iter().map(|(start, end, coverage)| RasterSelectionSpan { start: start as u32, length: (end - start) as u32, coverage: u32::from(coverage) }).collect()))
}

/// 🧱️ The one leaf a released stroke means on `document` under the session `config`, refused (zero trace) when the
/// stroke cannot paint: no such layer, a hidden or locked one, a layer without pixels (or, painting the mask, without a
/// mask), or no samples.
pub fn paint_stroke_leaf(payload: &PaintStroke, document: &RasterSnapshot, config: &RasterConfig) -> Result<RasterMutation, Fault> {
    if !matches!(payload.tool.as_str(), "brush" | "eraser") {
        return Err(fault("raster-paint-tool-invalid"));
    }
    if payload.xs.len() != payload.ys.len() || payload.xs.is_empty() || payload.xs.len() > RASTER_STROKE_MAXIMUM_POINTS || !payload.xs.iter().chain(&payload.ys).all(|value| value.is_finite()) {
        return Err(fault("raster-paint-points-invalid"));
    }
    let layer = find_layer(&document.layers, &payload.layer_id).ok_or_else(|| fault("raster-layer-not-found"))?;
    if !layer_visible(layer) {
        return Err(fault("raster-paint-layer-hidden"));
    }
    if !layer_protection(&document.layers, &payload.layer_id).is_some_and(|protection| protection.editable) {
        return Err(fault("raster-layer-locked"));
    }
    let target = config.paint_target.as_str();
    let paints = match (target, layer) {
        ("mask", RasterLayerNode::Pixel { mask, .. } | RasterLayerNode::Group { mask, .. }) => mask.is_some(),
        ("pixels", RasterLayerNode::Pixel { .. }) => true,
        _ => false,
    };
    if !paints {
        return Err(fault("raster-paint-target-missing"));
    }
    let color = match target {
        "mask" => {
            let grey = f64::from(config.mask_value.min(255)) / 255.0;
            vec![grey, grey, grey, 1.0]
        }
        _ => hex_color(&config.brush_color).ok_or_else(|| fault("raster-brush-color-invalid"))?,
    };
    let unit = |value: f64| if value.is_finite() { value.clamp(0.0, 1.0) } else { 1.0 };
    let brush = RasterBrush { size: config.brush_size.clamp(0.1, 4096.0), hardness: unit(config.brush_hardness), opacity: unit(config.brush_opacity), color };
    let points = payload.xs.iter().zip(&payload.ys).map(|(x, y)| RasterStrokePoint { x: *x, y: *y }).collect();
    Ok(RasterMutation::PaintStroke(PaintStrokeLeaf { layer_id: payload.layer_id.clone(), target: target.to_string(), tool: payload.tool.clone(), brush, points, selection: selection(config, &payload.layer_id)? }))
}
//#endregion 🎨️Leaf

//#region 🛠️PaintTool
/// 📨️ One released stroke: the leaf it paints — a dispatch input, never tool state.
#[derive(Clone, Debug)]
pub struct PaintToolRequest {
    pub stroke: RasterMutation,
}

/// 🧰️ The paint tool's context: nothing survives an event, because a released stroke is one event.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PaintToolContext;

fn paint_tool_context(input: PaintToolContext) -> PaintToolContext {
    input
}

fn stroke_paints(_context: &PaintToolContext, event: Option<&paint_tool::Event>) -> bool {
    matches!(event, Some(paint_tool::Event::Stroke(_)))
}

fn yield_stroke(_context: &mut PaintToolContext, event: Option<&paint_tool::Event>, sink: &mut Vec<Command<paint_tool::PaintTool>>) {
    let Some(paint_tool::Event::Stroke(request)) = event else { return };
    sink.push(Command::Effect(ToolYield::upsert("stroke:0", request.stroke.clone())));
    sink.push(Command::Effect(ToolYield::Commit));
}

machine::statechart! {
    machine paint_tool {
        context: PaintToolContext;
        event Event { Stroke(PaintToolRequest) }
        input: PaintToolContext;
        output: ();
        effect: ToolYield<RasterMutation>;
        context_from_input: paint_tool_context;
        initial: idle;
        state idle {
            on Stroke if stroke_paints => idle do yield_stroke;
        }
    }
}

/// 🧷️ The paint tool's host: its chart declares no timer, no invoke and no foreign effect.
pub struct PaintToolHost;

impl machine::Host<paint_tool::PaintTool> for PaintToolHost {
    fn execute_effect(&mut self, _actor: machine::ActorId, _effect: ToolYield<RasterMutation>) {}
    fn schedule(&mut self, _actor: machine::ActorId, _timer: machine::TimerId, _delay_ms: u64) {}
    fn cancel_timer(&mut self, _actor: machine::ActorId, _timer: machine::TimerId) {}
    fn start_task(&mut self, _actor: machine::ActorId, _invoke: machine::InvokeId) {}
    fn cancel_task(&mut self, _actor: machine::ActorId, _invoke: machine::InvokeId) {}
    fn now_ms(&self) -> u64 {
        semio_framework_job::default_now_ms().unwrap_or(0)
    }
}

/// 🛠️ Runs one released stroke through a paint tool at rest as ONE transaction of [`RASTER_PAINT_TOOL_ID`], its ref
/// minted from the admission's `authoring_seed` and the host clock.
pub fn raster_paint_commit(authoring_seed: &str, stroke: RasterMutation) -> Option<(protocol::TransactionRef, Vec<RasterMutation>)> {
    let mut runner = ToolMachineRunner::<paint_tool::PaintTool, PaintToolHost>::start(RASTER_PAINT_TOOL_ID, protocol::ActorId(authoring_seed.to_string()), PaintToolContext, PaintToolHost).ok()?;
    let clock = protocol::HybridLogicalTimestamp { actor: 0, physical_ms: semio_framework_job::default_now_ms().unwrap_or(0), logical: 0 };
    match runner.send(paint_tool::Event::Stroke(PaintToolRequest { stroke }), clock).ok()? {
        ToolStep::Committed(transaction, mutations) => Some((transaction, mutations)),
        ToolStep::Idle | ToolStep::Open | ToolStep::Aborted(..) | ToolStep::Empty(_) => None,
    }
}
//#endregion 🛠️PaintTool

/// 🖌️ One released stroke: ONE edit stamped with its `TransactionRef` (plain without an admission — a render or test
/// view), or a refusal that leaves zero trace.
pub fn handle(payload: &PaintStroke, doc: &ArtifactView<'_, RasterSnapshot>, cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterMutation, RasterConfigMutation>, Fault> {
    let stroke = paint_stroke_leaf(payload, doc.snapshot, cfg.snapshot)?;
    let seed = doc.operation_optional().map(|operation| operation.authoring_seed.as_str()).unwrap_or_default();
    Ok(match raster_paint_commit(seed, stroke) {
        Some((transaction, mutations)) if !seed.is_empty() => Emit::commit_transaction(transaction, mutations),
        Some((_, mutations)) => Emit::mutations(mutations),
        None => Emit::default(),
    })
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
