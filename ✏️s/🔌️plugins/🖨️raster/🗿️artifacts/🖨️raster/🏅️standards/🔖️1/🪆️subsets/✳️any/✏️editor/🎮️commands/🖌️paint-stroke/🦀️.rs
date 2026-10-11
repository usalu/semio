//! 🖌️ Raster play app commands — `paint-stroke`: the brush and eraser TOOL, in the layer image's pixels. A stroke arrives
//! in one dispatch (no `phase`) or streams over many (`phase: stream` ticks carrying the new samples, then `commit`);
//! a host cancel (`phase: abort` with its `reason`) leaves zero trace. The tool is a `🔄️machine` statechart whose effects
//! are `ToolYield`s, driven by the `🛠️tool-machine` runner: a stroke is ONE `ToolTransaction` holding ONE parametric
//! `paint-stroke` leaf — the layer, the target and brush the session holds, the points and the pixel selection the
//! stroke is clipped to — published as one edit, one history row, whose brush and points time travel edits. Between
//! stream ticks the open transaction lives in the Composite window's transient (`🖼️composite/🫧️transient`) and the
//! window paints the committed document with the provisional leaf applied by the ONE rasterizer. Tool state is never
//! history; the yielded leaf is (design §5, §17.2, ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING). The one-shot pixel
//! tool, the layer refusals and the session colour and selection readers are shared with the bucket (`🪣️fill-region`).
#![allow(unexpected_cfgs)]

use crate::editor::raster::config::{RasterConfig, RasterConfigMutation};
use crate::editor::raster::modes::edit::windows::composite::transient::{self, RasterCompositeWindowTransient, RasterStrokeToolState};
use crate::editor::raster::{RasterCommand, RasterPlayApp};
use crate::mutations::paint_stroke::{PaintStroke as PaintStrokeLeaf, RasterBrush, RasterSelectionSpan, RasterStrokePoint, RASTER_STROKE_MAXIMUM_POINTS};
use crate::op::RasterMutation;
use crate::standards::v1::subsets::any::schema::{find_layer, layer_protection, layer_visible};
use crate::{RasterLayerNode, RasterSnapshot};
use machine::Command;
use semio_framework_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep};
use semio_framework_plugin::{ArtifactView, ConfigView, EditorApp, Emit, EphemeralEmit, Fault};
use semio_framework_tool_machine::{drive_gesture, GesturePhase, GestureTool, ToolAbortReason, ToolMachineRunner, ToolRefusal, ToolStep, ToolTransaction, ToolTransactionState, ToolYield};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Payload
/// 🪪️ The editor whose paint tool authors every stroke transaction: `<appId>#paintStroke`.
pub const RASTER_PAINT_TOOL_ID: &str = "s.raster.raster@1/*#editor#paintStroke";

/// 🔑️ The transaction key of a stroke's leaf — every tick upserts it, so an open stroke holds ONE net leaf.
pub const RASTER_PAINT_TOOL_KEY: &str = "stroke:0";

/// 🖌️ One stroke dispatch as both hosts send it: the layer, the tool (`brush` or `eraser`) and the samples in the target
/// image's pixels, in drawing order, split into their x and y columns; `phase` places it in the streamed stroke `gesture`
/// names (`stream`, `commit`, `abort` with a `reason`; absent: the whole stroke at once).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "paint-stroke")]
pub struct PaintStroke {
    pub layer_id: String,
    pub tool: String,
    pub xs: Vec<f64>,
    pub ys: Vec<f64>,
    pub phase: Option<String>,
    pub reason: Option<String>,
    pub gesture: Option<String>,
}
//#endregion 🔖️Payload

//#region 🎨️Leaf
/// 🏷️ A raster tool refusal under its named code `raster.<area>.<name>`, localized through [`raster_fault_notices`].
pub(crate) fn raster_fault(code: &'static str) -> Fault {
    Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new(code), code)
}

/// 📣️ The en/de notices of the raster tool refusals (design §20.12): strokes, fills, filters, transforms and the retained route.
pub fn raster_fault_notices() -> &'static [(&'static str, semio_framework_ui_locale::LocalizedLabel)] {
    use semio_framework_ui_locale::LocalizedLabel;
    static NOTICES: std::sync::LazyLock<[(&str, LocalizedLabel); 16]> = std::sync::LazyLock::new(|| {
        [
            ("raster.stroke.phase-invalid", LocalizedLabel::native("This stroke step is not known.", "Dieser Strichschritt ist unbekannt.")),
            ("raster.stroke.window-required", LocalizedLabel::native("A streamed stroke needs an open composite window.", "Ein gestreamter Strich braucht ein geöffnetes Kompositionsfenster.")),
            ("raster.stroke.gesture-required", LocalizedLabel::native("A streamed stroke needs its press.", "Ein gestreamter Strich braucht seinen Tastendruck.")),
            ("raster.paint.points-invalid", LocalizedLabel::native("The stroke has no valid points.", "Der Strich hat keine gültigen Punkte.")),
            ("raster.paint.tool-invalid", LocalizedLabel::native("Only the brush and the eraser paint strokes.", "Nur Pinsel und Radierer malen Striche.")),
            ("raster.paint.layer-hidden", LocalizedLabel::native("Show the layer to paint on it.", "Die Ebene einblenden, um darauf zu malen.")),
            ("raster.paint.target-missing", LocalizedLabel::native("The selected layer has nothing to paint: no pixels, or no mask while painting the mask.", "Die gewählte Ebene hat nichts zum Malen: keine Pixel, oder keine Maske beim Malen der Maske.")),
            ("raster.brush.color-invalid", LocalizedLabel::native("The brush colour is not a valid #rrggbb colour.", "Die Pinselfarbe ist keine gültige #rrggbb-Farbe.")),
            ("raster.fill.seed-invalid", LocalizedLabel::native("The fill starts outside the layer's pixels.", "Die Füllung beginnt außerhalb der Pixel der Ebene.")),
            ("raster.fill.invalid", LocalizedLabel::native("The selection cannot be filled.", "Die Auswahl lässt sich nicht füllen.")),
            ("raster.fill.tolerance-invalid", LocalizedLabel::native("The colour tolerance must lie between 0 and 255.", "Die Farbtoleranz muss zwischen 0 und 255 liegen.")),
            ("raster.filter.invalid", LocalizedLabel::native("This filter or its amount is not valid.", "Dieser Filter oder seine Stärke ist ungültig.")),
            ("raster.filter.pixels-only", LocalizedLabel::native("Filters change pixels; switch from painting the mask to the pixels.", "Filter ändern Pixel; vom Malen der Maske zu den Pixeln wechseln.")),
            ("raster.transform.invalid", LocalizedLabel::native("This rotation, size or crop is not valid for the image.", "Diese Drehung, Größe oder dieser Zuschnitt ist für das Bild ungültig.")),
            ("raster.transform.pixels-only", LocalizedLabel::native("Transforms change pixels; switch from painting the mask to the pixels.", "Transformationen ändern Pixel; vom Malen der Maske zu den Pixeln wechseln.")),
            ("raster.document.too-large", LocalizedLabel::native("The image has too many layers and assets for this action.", "Das Bild hat zu viele Ebenen und Assets für diese Aktion.")),
        ]
    });
    &*NOTICES
}

/// 🚫️ Why `layer_id` cannot take a repaint of the session's paint target on `document`: no such layer, a hidden or
/// locked one, a layer without pixels or, painting the mask, without a mask; `None` when it can.
pub(crate) fn paint_target_refusal(layer_id: &str, document: &RasterSnapshot, config: &RasterConfig) -> Option<Fault> {
    let Some(layer) = find_layer(&document.layers, layer_id) else { return Some(Fault::from("raster-layer-not-found")) };
    if !layer_visible(layer) {
        return Some(raster_fault("raster.paint.layer-hidden"));
    }
    if !layer_protection(&document.layers, layer_id).is_some_and(|protection| protection.editable) {
        return Some(Fault::from("raster-layer-locked"));
    }
    let paints = match (config.paint_target.as_str(), layer) {
        ("mask", RasterLayerNode::Pixel { mask, .. } | RasterLayerNode::Group { mask, .. }) => mask.is_some(),
        ("pixels", RasterLayerNode::Pixel { .. }) => true,
        _ => false,
    };
    (!paints).then(|| raster_fault("raster.paint.target-missing"))
}

/// 🎨️ `#rrggbb` as four unit channels, opaque.
pub(crate) fn hex_color(value: &str) -> Option<Vec<f64>> {
    crate::editor::raster::config::valid_brush_color(value).then(|| {
        let rgb = u32::from_str_radix(&value[1..], 16).unwrap_or(0);
        vec![f64::from((rgb >> 16) & 0xff) / 255.0, f64::from((rgb >> 8) & 0xff) / 255.0, f64::from(rgb & 0xff) / 255.0, 1.0]
    })
}

/// ✂️ The session's pixel selection as the leaf's runs, when it selects THIS layer's target.
pub(crate) fn selection(config: &RasterConfig, layer_id: &str) -> Result<Option<Vec<RasterSelectionSpan>>, Fault> {
    let Some(selection) = config.pixel_selection.as_ref().filter(|selection| selection.layer_id == layer_id && selection.target == config.paint_target) else { return Ok(None) };
    let count = selection.validate()?;
    crate::editor::raster::selection::selection_spans(&selection.spans, count)?;
    Ok(Some(selection.spans.clone()))
}

/// 🧱️ The one leaf a released stroke means on `document` under the session `config`, refused (zero trace) when the
/// stroke cannot paint: no such layer, a hidden or locked one, a layer without pixels (or, painting the mask, without a
/// mask), or no samples.
pub fn paint_stroke_leaf(payload: &PaintStroke, document: &RasterSnapshot, config: &RasterConfig) -> Result<RasterMutation, Fault> {
    if !matches!(payload.tool.as_str(), "brush" | "eraser") {
        return Err(raster_fault("raster.paint.tool-invalid"));
    }
    if payload.xs.len() != payload.ys.len() || payload.xs.is_empty() || payload.xs.len() > RASTER_STROKE_MAXIMUM_POINTS || !payload.xs.iter().chain(&payload.ys).all(|value| value.is_finite()) {
        return Err(raster_fault("raster.paint.points-invalid"));
    }
    if let Some(refusal) = paint_target_refusal(&payload.layer_id, document, config) {
        return Err(refusal);
    }
    let target = config.paint_target.as_str();
    let color = match target {
        "mask" => {
            let grey = f64::from(config.mask_value.min(255)) / 255.0;
            vec![grey, grey, grey, 1.0]
        }
        _ => hex_color(&config.brush_color).ok_or_else(|| raster_fault("raster.brush.color-invalid"))?,
    };
    let unit = |value: f64| if value.is_finite() { value.clamp(0.0, 1.0) } else { 1.0 };
    let brush = RasterBrush { size: config.brush_size.clamp(0.1, 4096.0), hardness: unit(config.brush_hardness), opacity: unit(config.brush_opacity), color };
    let points = payload.xs.iter().zip(&payload.ys).map(|(x, y)| RasterStrokePoint { x: *x, y: *y }).collect();
    Ok(RasterMutation::PaintStroke(PaintStrokeLeaf { layer_id: payload.layer_id.clone(), target: target.to_string(), tool: payload.tool.clone(), brush, points, selection: selection(config, &payload.layer_id)? }))
}
//#endregion 🎨️Leaf

//#region 🛠️PaintTool
/// 📨️ One paint event: the leaf a stroke opens with (a one-shot, a first stream tick, a commit with nothing open) and the
/// samples the dispatch carries, appended to an open stroke — dispatch inputs, never tool state.
#[derive(Clone, Debug)]
pub struct PaintToolRequest {
    pub stroke: Option<RasterMutation>,
    pub points: Vec<RasterStrokePoint>,
}

/// 🧰️ The paint tool's context: the leaf a streamed stroke has accumulated so far (`None` at rest).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PaintToolContext {
    pub stroke: Option<RasterMutation>,
}

fn paint_tool_context(input: PaintToolContext) -> PaintToolContext {
    input
}

/// ➕️ `stroke` followed by `points`, up to [`RASTER_STROKE_MAXIMUM_POINTS`]; a point repeating the one before it adds
/// nothing (a pointer resting inside one pixel samples it many times).
fn extended(mut stroke: RasterMutation, points: &[RasterStrokePoint]) -> RasterMutation {
    if let RasterMutation::PaintStroke(leaf) = &mut stroke {
        for point in points {
            if leaf.points.len() >= RASTER_STROKE_MAXIMUM_POINTS {
                break;
            }
            if leaf.points.last() != Some(point) {
                leaf.points.push(*point);
            }
        }
    }
    stroke
}

fn stroke_opens(_context: &PaintToolContext, event: Option<&paint_tool::Event>) -> bool {
    matches!(event, Some(paint_tool::Event::Stroke(request) | paint_tool::Event::Stream(request) | paint_tool::Event::Finish(request)) if request.stroke.is_some())
}

fn yield_stroke(_context: &mut PaintToolContext, event: Option<&paint_tool::Event>, sink: &mut Vec<Command<paint_tool::PaintTool>>) {
    let Some(paint_tool::Event::Stroke(request) | paint_tool::Event::Finish(request)) = event else { return };
    let Some(stroke) = request.stroke.clone() else { return };
    sink.push(Command::Effect(ToolYield::upsert(RASTER_PAINT_TOOL_KEY, stroke)));
    sink.push(Command::Effect(ToolYield::Commit));
}

fn begin_stream(context: &mut PaintToolContext, event: Option<&paint_tool::Event>, sink: &mut Vec<Command<paint_tool::PaintTool>>) {
    let Some(paint_tool::Event::Stream(request)) = event else { return };
    let Some(stroke) = request.stroke.clone() else { return };
    sink.push(Command::Effect(ToolYield::upsert(RASTER_PAINT_TOOL_KEY, stroke.clone())));
    context.stroke = Some(stroke);
}

fn continue_stream(context: &mut PaintToolContext, event: Option<&paint_tool::Event>, sink: &mut Vec<Command<paint_tool::PaintTool>>) {
    let (Some(paint_tool::Event::Stream(request)), Some(stroke)) = (event, context.stroke.take()) else { return };
    let stroke = extended(stroke, &request.points);
    sink.push(Command::Effect(ToolYield::upsert(RASTER_PAINT_TOOL_KEY, stroke.clone())));
    context.stroke = Some(stroke);
}

fn finish_stream(context: &mut PaintToolContext, event: Option<&paint_tool::Event>, sink: &mut Vec<Command<paint_tool::PaintTool>>) {
    let (Some(paint_tool::Event::Finish(request)), Some(stroke)) = (event, context.stroke.take()) else { return };
    sink.push(Command::Effect(ToolYield::upsert(RASTER_PAINT_TOOL_KEY, extended(stroke, &request.points))));
    sink.push(Command::Effect(ToolYield::Commit));
}

fn cancel_stream(context: &mut PaintToolContext, _event: Option<&paint_tool::Event>, sink: &mut Vec<Command<paint_tool::PaintTool>>) {
    context.stroke = None;
    sink.push(Command::Effect(ToolYield::Abort));
}

// 🎭️ The pixel tool's control flow (plain comment: rustdoc cannot document a macro invocation): a one-shot (and a commit
// with nothing open) yields and commits at rest; a stream opens `painting`, whose ticks extend the one leaf and whose
// finish commits ONE transaction; a cancel aborts it.
machine::statechart! {
    machine paint_tool {
        context: PaintToolContext;
        event Event { Stroke(PaintToolRequest), Stream(PaintToolRequest), Finish(PaintToolRequest), Cancel }
        input: PaintToolContext;
        output: ();
        effect: ToolYield<RasterMutation>;
        context_from_input: paint_tool_context;
        initial: idle;
        state idle {
            on Stroke if stroke_opens => idle do yield_stroke;
            on Finish if stroke_opens => idle do yield_stroke;
            on Stream if stroke_opens => painting do begin_stream;
        }
        state painting {
            on Stream => painting do continue_stream;
            on Finish => idle do finish_stream;
            on Cancel => idle do cancel_stream;
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

/// 🛠️ One Composite window's paint tool for one dispatch, driven by the shared streamed-gesture runner
/// ([`drive_gesture`]): started at rest, or resumed from the stroke its window transient persisted, and persisted back
/// while its transaction is open. A stroke's gesture identity is its PRESS: a tick of another press interrupts the open
/// stroke like another verb (`captureLost`); its base never moves, because the leaf names samples and brush, never the
/// pixels under them, so it repaints on any base.
pub struct RasterPaintTool {
    runner: ToolMachineRunner<paint_tool::PaintTool, PaintToolHost>,
    press: String,
    authoring_seed: String,
}

impl RasterPaintTool {
    /// 🚀️ The tool at rest, authoring `tool` transactions for `press` under this admission's seed.
    pub fn at_rest(tool: &str, press: &str, authoring_seed: &str) -> Result<Self, ToolRefusal> {
        let runner = ToolMachineRunner::start(tool, protocol::ActorId(authoring_seed.into()), PaintToolContext::default(), PaintToolHost)?;
        Ok(Self { runner, press: press.to_string(), authoring_seed: authoring_seed.to_string() })
    }
}

impl GestureTool for RasterPaintTool {
    type Gesture = RasterStrokeToolState;
    type Tick = PaintToolRequest;
    type Mutation = RasterMutation;

    fn start(press: &str, authoring_seed: &str, _base_revision: &str) -> Result<Self, ToolRefusal> {
        Self::at_rest(RASTER_PAINT_TOOL_ID, press, authoring_seed)
    }

    /// ⏯️ The stroke a window transient persisted, restored by stable ids with its open transaction; a state the current
    /// chart cannot restore is refused (`Closed`), so the runner drops it with zero trace.
    fn resume(state: &RasterStrokeToolState) -> Result<Self, ToolRefusal> {
        let definition = <paint_tool::PaintTool as machine::Machine>::definition();
        let persisted = machine::PersistedSnapshot { version: 1, fingerprint: definition.fingerprint, states: state.states.clone(), history: Vec::new(), done: false };
        let leaf: RasterMutation = semio_framework_value::FromValue::from_value(state.stroke.clone()).map_err(|_| ToolRefusal::Closed)?;
        if !matches!(leaf, RasterMutation::PaintStroke(_)) {
            return Err(ToolRefusal::Closed);
        }
        let snapshot = machine::restore::<paint_tool::PaintTool, machine::NoMigrations>(&persisted, PaintToolContext { stroke: Some(leaf.clone()) }, &[]).map_err(|_| ToolRefusal::Closed)?;
        let transaction = ToolTransaction::resume(state.transaction.clone(), vec![(RASTER_PAINT_TOOL_KEY.to_string(), leaf)]);
        let runner = ToolMachineRunner::resume(RASTER_PAINT_TOOL_ID, protocol::ActorId(state.authoring_seed.as_str().into()), PaintToolContext::default(), snapshot, Some(transaction), PaintToolHost)?;
        Ok(Self { runner, press: state.gesture.clone(), authoring_seed: state.authoring_seed.clone() })
    }

    fn verb(&self) -> &str {
        &self.press
    }

    fn base_revision(&self) -> &str {
        ""
    }

    fn abort(&mut self, reason: ToolAbortReason) {
        self.runner.abort(reason);
    }

    fn send(&mut self, phase: GesturePhase, tick: Option<PaintToolRequest>) -> Result<ToolStep<RasterMutation>, ToolRefusal> {
        let request = tick.unwrap_or(PaintToolRequest { stroke: None, points: Vec::new() });
        let event = match phase {
            GesturePhase::Stream => paint_tool::Event::Stream(request),
            GesturePhase::Commit => paint_tool::Event::Finish(request),
            GesturePhase::Once => paint_tool::Event::Stroke(request),
            GesturePhase::Abort(_) => paint_tool::Event::Cancel,
        };
        self.runner.send(event, semio_framework_tool_machine::authoring_clock(0))
    }

    /// 💾️ The open stroke to persist in the window transient: `Some` only while its transaction is open.
    fn persist(self) -> Option<RasterStrokeToolState> {
        let (snapshot, transaction) = self.runner.into_parts();
        let transaction = transaction.filter(|transaction| transaction.state() == ToolTransactionState::Open)?;
        let (_, leaf) = transaction.entries().iter().find(|(key, _)| key == RASTER_PAINT_TOOL_KEY)?;
        Some(RasterStrokeToolState { gesture: self.press, states: machine::persist(&snapshot).states, authoring_seed: self.authoring_seed, transaction: transaction.reference().clone(), stroke: semio_framework_value::ToValue::to_value(leaf) })
    }
}

/// 🛠️ Runs one released gesture's leaf (a brush stroke, a bucket fill) through the pixel tool at rest as ONE
/// transaction of `tool`, its ref minted from the admission's `authoring_seed` and the host clock.
pub fn raster_tool_commit(tool: &str, authoring_seed: &str, stroke: RasterMutation) -> Option<(protocol::TransactionRef, Vec<RasterMutation>)> {
    let mut runner = RasterPaintTool::at_rest(tool, "", authoring_seed).ok()?;
    match runner.send(GesturePhase::Once, Some(PaintToolRequest { stroke: Some(stroke), points: Vec::new() })).ok()? {
        ToolStep::Committed(transaction, mutations) => Some((transaction, mutations)),
        ToolStep::Idle | ToolStep::Open | ToolStep::Aborted(..) | ToolStep::Empty(_) => None,
    }
}

/// 👁️ The document a Composite window paints while its paint tool holds an open stroke: the provisional leaf applied to
/// `document` by the ONE rasterizer — a preview only this window sees, never history. `None` at rest or when the leaf
/// does not apply. The caller retires the preview it renders.
pub fn raster_stroke_preview(document: &RasterSnapshot, transient: &RasterCompositeWindowTransient) -> Option<RasterSnapshot> {
    let leaf: RasterMutation = semio_framework_value::FromValue::from_value(transient.stroke.as_ref()?.stroke.clone()).ok()?;
    crate::standards::v1::subsets::any::io::text::mutations::apply_raster_mutation(document, &leaf).ok()
}
//#endregion 🛠️PaintTool

/// 🖌️ One stroke dispatch through the Composite window's paint tool `window`, on the shared streamed-gesture runner: the
/// emission and the window's next partition. A one-shot or a commit is ONE edit stamped with its `TransactionRef` (plain
/// without an admission — a render or test view); a stream tick publishes nothing on the document; a one-shot or a tick of
/// another press aborts the open stroke (`captureLost`) first; an abort, a commit with nothing to paint, an unrestorable
/// stroke and a late tick of a press the window already closed leave zero trace; a stroke the layer cannot take is refused.
pub fn handle_in_window(payload: &PaintStroke, doc: &ArtifactView<'_, RasterSnapshot>, cfg: &ConfigView<'_, RasterConfig>, window: &RasterCompositeWindowTransient) -> Result<(Emit<RasterMutation, RasterConfigMutation>, RasterCompositeWindowTransient), Fault> {
    let phase = GesturePhase::parse(payload.phase.as_deref(), payload.reason.as_deref()).ok_or_else(|| raster_fault("raster.stroke.phase-invalid"))?;
    if payload.xs.len() != payload.ys.len() || payload.xs.len() > RASTER_STROKE_MAXIMUM_POINTS || !payload.xs.iter().chain(&payload.ys).all(|value| value.is_finite()) {
        return Err(raster_fault("raster.paint.points-invalid"));
    }
    let gesture = payload.gesture.as_deref().filter(|gesture| !gesture.is_empty());
    if matches!(phase, GesturePhase::Stream | GesturePhase::Commit) && gesture.is_none() {
        return Err(raster_fault("raster.stroke.gesture-required"));
    }
    if gesture.is_some() && window.closed.as_deref() == gesture {
        return Ok((Emit::default(), window.clone()));
    }
    let open = window.stroke.as_deref();
    let closing = |press: Option<&str>| press.map(str::to_string).or_else(|| window.closed.clone());
    if let (GesturePhase::Abort(_), Some(state), Some(press)) = (phase, open, gesture) {
        if state.gesture != press {
            return Ok((Emit::default(), RasterCompositeWindowTransient { stroke: window.stroke.clone(), closed: Some(press.to_string()) }));
        }
    }
    let press = gesture.unwrap_or_default();
    let continues = phase != GesturePhase::Once && open.is_some_and(|state| state.gesture == press);
    let stroke = match phase {
        GesturePhase::Abort(_) => None,
        _ if continues => None,
        GesturePhase::Stream | GesturePhase::Commit if payload.xs.is_empty() => None,
        _ => Some(paint_stroke_leaf(payload, doc.snapshot, cfg.snapshot)?),
    };
    let tick = PaintToolRequest { stroke, points: payload.xs.iter().zip(&payload.ys).map(|(x, y)| RasterStrokePoint { x: *x, y: *y }).collect() };
    let seed = doc.operation_optional().map(|operation| operation.authoring_seed.as_str()).unwrap_or_default();
    let drive = drive_gesture::<RasterPaintTool>(open, press, phase, Some(tick), seed, "").map_err(|refusal| semio_framework_plugin::Fault::new(semio_framework_plugin::FaultOrigin::App, refusal.code(), "the paint tool refused the stroke"))?;
    let stroke = drive.next.map_or_else(|| window.stroke.clone(), |next| next.map(Box::new));
    Ok(match drive.committed {
        Some((transaction, mutations)) if !seed.is_empty() => (Emit::commit_transaction(transaction, mutations), RasterCompositeWindowTransient { stroke: None, closed: closing(gesture) }),
        Some((_, mutations)) => (Emit::mutations(mutations), RasterCompositeWindowTransient { stroke: None, closed: closing(gesture) }),
        None => match phase {
            GesturePhase::Abort(_) => (Emit::default(), RasterCompositeWindowTransient { stroke: None, closed: closing(gesture.or(open.map(|state| state.gesture.as_str()))) }),
            GesturePhase::Commit => (Emit::default(), RasterCompositeWindowTransient { stroke: None, closed: closing(gesture) }),
            GesturePhase::Once | GesturePhase::Stream => (Emit::default(), RasterCompositeWindowTransient { stroke, closed: window.closed.clone() }),
        },
    })
}

/// 🖌️ One stroke dispatch without a window: a one-shot is ONE edit, an abort finds nothing to end, and a streamed
/// phase is refused — its tool state lives in the Composite window that streams it.
pub fn handle(payload: &PaintStroke, doc: &ArtifactView<'_, RasterSnapshot>, cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterMutation, RasterConfigMutation>, Fault> {
    match GesturePhase::parse(payload.phase.as_deref(), payload.reason.as_deref()) {
        Some(GesturePhase::Once) => handle_in_window(payload, doc, cfg, &RasterCompositeWindowTransient::default()).map(|(emit, _)| emit),
        Some(GesturePhase::Abort(_)) => Ok(Emit::default()),
        Some(GesturePhase::Stream | GesturePhase::Commit) => Err(raster_fault("raster.stroke.window-required")),
        None => Err(raster_fault("raster.stroke.phase-invalid")),
    }
}

//#region 🧵️RetainedWork
/// 🧵️ The retained work of `paintStroke`: one dispatch through the dispatching Composite window's paint tool, publishing
/// the edit on the document lane and the window's next tool state on the window-transient lane.
#[derive(Default)]
pub struct PaintStrokeWork {
    consumed: bool,
}

impl ArtifactCommandWork<EditorApp<RasterPlayApp>> for PaintStrokeWork {
    fn tool_id(&self) -> &'static str {
        "paintStroke"
    }

    fn extent(&self, command: &RasterCommand, _snapshot: &RasterSnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<RasterPlayApp>>>) -> Option<usize> {
        matches!(command, RasterCommand::PaintStroke(_)).then_some(1)
    }

    fn work_demands(&self, input: &ArtifactCommandInputs<'_, EditorApp<RasterPlayApp>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        let RasterCommand::PaintStroke(payload) = input.command else { return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "raster paint command required")) };
        let samples = payload.xs.len().saturating_add(payload.ys.len()).saturating_mul(std::mem::size_of::<f64>());
        let copy_bytes = std::mem::size_of::<(Emit<RasterMutation, RasterConfigMutation>, RasterCompositeWindowTransient)>().saturating_add(samples);
        Ok(semio_framework_value::RetirementDemand { copy_bytes, capacity_bytes: samples, depth: 1, ..Default::default() })
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, EditorApp<RasterPlayApp>>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<EditorApp<RasterPlayApp>>, Fault> {
        if self.consumed {
            return Err(Fault::from("raster-paint-stroke-work-repeated"));
        }
        self.consumed = true;
        let RasterCommand::PaintStroke(payload) = input.command else { return Err(Fault::from("raster-paint-stroke-route-mismatch")) };
        let doc = ArtifactView::with_operation(input.snapshot, input.history, input.operation.clone());
        let cfg = ConfigView { snapshot: input.config, window: None };
        let window = transient::from_snapshot(input.context.and_then(|context| context.window_transient.as_ref()));
        let (emit, next) = handle_in_window(payload, &doc, &cfg, &window)?;
        let window_transient = match input.context.and_then(|context| context.view_state.as_ref()) {
            Some(view) if next != window => vec![transient::addressed(view, next)?],
            None if next != window => return Err(raster_fault("raster.stroke.window-required")),
            _ => Vec::new(),
        };
        Ok(ArtifactCommandWorkStep::CompleteWithEphemeral { emit, ephemeral: EphemeralEmit { window_transient, ..Default::default() } })
    }
}
//#endregion 🧵️RetainedWork

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
