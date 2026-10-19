//! 🧱️ Board gestures of the puzzle 5d editor kit: the retained board-event and create-fastener work. See [`super`].

use crate::editor::puzzle5d::snapshot::Puzzle5dPlaySnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;
use crate::editor::puzzle5d::config::Puzzle5dCamera2d;
use crate::editor::puzzle5d::window as window_ownership;
use semio_framework_plugin::kernel::UiDirtyScope;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::InteractionTarget;
use semio_framework_plugin::InteractionWrite;
use semio_framework_plugin::MergeMode;
use semio_framework_value::FromValue;
use semio_framework_pack_json::{parse, Value};
use crate::editor::puzzle5d::kit::add_node_and_brush_part::Puzzle5dAddBrushPartWork;
use crate::editor::puzzle5d::interaction::Puzzle5dEditor;
use crate::editor::puzzle5d::entities::{PUZZLE5D_GRANULARITY_FASTENER, PUZZLE5D_GRANULARITY_PART, PUZZLE5D_INTERACTION_DOMAIN, puzzle5d_editor_projection, puzzle5d_grip_full_id};
use crate::editor::puzzle5d::modes::edit;
use crate::editor::puzzle5d::modes::edit::windows::world3d;
use semio_framework_job::StepContext;
use crate::editor::puzzle5d::interaction::{Puzzle5dCommand, puzzle5d_notice_emit};

//#region 🔖️PlayApp
//#region 🧵️RetainedCommands
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle5dBoardEventsStage {
    Open,
    Scan,
    Decode,
    Dispatch,
    ScanEdge,
    ScanDeleteEdges,
    Brush,
    DrainBrush,
    CloseBrush,
    Complete,
    Closing,
}

/// ♻️ The owners one `Puzzle5dBoardEventsWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle5dBoardEventsWorkOwners {
    pub mutations: Vec<Puzzle5dMutation>,
    brush_first: Option<Puzzle5dMutation>,
    brush_second: Option<Puzzle5dMutation>,
    event: Option<Value>,
    drags: Vec<crate::editor::puzzle5d::modes::edit::windows::world3d::utilities::transform::Puzzle5dSelectionRecord>,
    drag_at: Option<usize>,
    pending_source: Option<String>,
    pending_target: Option<String>,
    pending_edge_id: Option<String>,
    pending_edge_kind: Option<String>,
    pending_delete_id: Option<String>,
    pub camera2d: Option<Puzzle5dCamera2d>,
    select_ids: Option<Vec<String>>,
    removed_ids: Vec<String>,
    placed_id: Option<String>,
}

pub struct Puzzle5dBoardEventsWork {
    pub stage: Puzzle5dBoardEventsStage,
    byte_cursor: usize,
    event_start: Option<usize>,
    event_end: usize,
    pub depth: usize,
    in_string: bool,
    escape: bool,
    event: Option<Value>,
    /// 🎬️ Every `drag` gesture record of the batch, each one board drag the transform tool yields — the whole
    /// batch's drags are ONE tool transaction.
    drags: Vec<crate::editor::puzzle5d::modes::edit::windows::world3d::utilities::transform::Puzzle5dSelectionRecord>,
    /// 📍️ Where the first drag sits among the batch's mutations — the tool's leaves publish there, in event order.
    drag_at: Option<usize>,
    /// 🌱️ The admission's authoring seed the drags' tool transaction is minted from.
    pub authoring_seed: String,
    pending_source: Option<String>,
    pending_target: Option<String>,
    pending_edge_id: Option<String>,
    pending_edge_kind: Option<String>,
    pub fastener_cursor: usize,
    pending_delete_id: Option<String>,
    pub brush: Option<Puzzle5dAddBrushPartWork>,
    brush_first: Option<Puzzle5dMutation>,
    brush_second: Option<Puzzle5dMutation>,
    pub camera2d: Option<Puzzle5dCamera2d>,
    pub mutations: Vec<Puzzle5dMutation>,
    pub operation_nonce: u64,
    pub fresh_cursor: u64,
    /// 🕹️ The LAST `select` row of the batch is the engine's whole selection set (replace semantics), so
    /// later rows supersede earlier ones inside one batch.
    select_ids: Option<Vec<String>>,
    /// 🧹️ Ids this batch removed from the document — a delete must not leave a dangling selection, so each
    /// one is subtracted from the live selection through the sanctioned reducer channel.
    removed_ids: Vec<String>,
    /// 🖌️ The part a `brushPlace` row created, re-selected so the next brush gesture chains off it.
    placed_id: Option<String>,
    locked_refused: bool,
    pub close_owners: crate::puzzle_job::WorkClosing<Puzzle5dBoardEventsWorkOwners>,
}

impl Puzzle5dBoardEventsWork {
    pub fn new(authoring_seed: String) -> Self {
        Self { close_owners: Default::default(),
            stage: Puzzle5dBoardEventsStage::Open,
            byte_cursor: 0,
            event_start: None,
            event_end: 0,
            depth: 0,
            in_string: false,
            escape: false,
            event: None,
            drags: Vec::new(),
            drag_at: None,
            authoring_seed,
            pending_source: None,
            pending_target: None,
            pending_edge_id: None,
            pending_edge_kind: None,
            fastener_cursor: 0,
            pending_delete_id: None,
            brush: None,
            brush_first: None,
            brush_second: None,
            camera2d: None,
            mutations: Vec::with_capacity(crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS),
            operation_nonce: 0,
            fresh_cursor: 0,
            select_ids: None,
            removed_ids: Vec::new(),
            placed_id: None,
            locked_refused: false,
        }
    }
}

impl Puzzle5dBoardEventsWork {
    pub fn source(command: &Puzzle5dCommand) -> Result<&str, Fault> {
        command.args().and_then(|args| args.get("eventsJson")).and_then(Value::as_str).ok_or_else(|| Fault::from("puzzle5d-board-events-input-missing"))
    }

    pub fn push(&mut self, mutation: Puzzle5dMutation) -> Result<(), Fault> {
        if self.mutations.len() >= crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS {
            return Err(Fault::from("puzzle5d-board-events-mutation-capacity"));
        }
        self.mutations.push(mutation);
        Ok(())
    }

    fn next_event(&mut self) {
        self.event = None;
        self.event_start = None;
        self.event_end = 0;
        self.depth = 0;
        self.in_string = false;
        self.escape = false;
        self.stage = Puzzle5dBoardEventsStage::Scan;
    }

    fn scan_one(&mut self, source: &str) -> Result<(), Fault> {
        let bytes = source.as_bytes();
        let Some(byte) = bytes.get(self.byte_cursor).copied() else {
            return Err(Fault::from("puzzle5d-board-events-array-unterminated"));
        };
        match self.stage {
            Puzzle5dBoardEventsStage::Open => {
                self.byte_cursor += 1;
                if byte.is_ascii_whitespace() {
                    return Ok(());
                }
                if byte != b'[' {
                    return Err(Fault::from("puzzle5d-board-events-array-malformed"));
                }
                self.stage = Puzzle5dBoardEventsStage::Scan;
            }
            Puzzle5dBoardEventsStage::Scan if self.event_start.is_none() => {
                self.byte_cursor += 1;
                if byte.is_ascii_whitespace() || byte == b',' {
                    return Ok(());
                }
                if byte == b']' {
                    self.stage = Puzzle5dBoardEventsStage::Complete;
                    return Ok(());
                }
                if byte != b'{' {
                    return Err(Fault::from("puzzle5d-board-events-event-malformed"));
                }
                self.event_start = Some(self.byte_cursor - 1);
                self.depth = 1;
            }
            Puzzle5dBoardEventsStage::Scan => {
                self.byte_cursor += 1;
                if self.in_string {
                    if self.escape {
                        self.escape = false;
                    } else if byte == b'\\' {
                        self.escape = true;
                    } else if byte == b'"' {
                        self.in_string = false;
                    }
                    return Ok(());
                }
                if byte == b'"' {
                    self.in_string = true;
                } else if byte == b'{' || byte == b'[' {
                    self.depth = self.depth.checked_add(1).ok_or_else(|| Fault::from("puzzle5d-board-events-depth-capacity"))?;
                } else if byte == b'}' || byte == b']' {
                    self.depth = self.depth.checked_sub(1).ok_or_else(|| Fault::from("puzzle5d-board-events-depth-malformed"))?;
                    if self.depth == 0 {
                        self.event_end = self.byte_cursor;
                        self.stage = Puzzle5dBoardEventsStage::Decode;
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn take_payload(&mut self) -> Value {
        self.event.as_mut().and_then(Value::as_object_mut).and_then(|event| event.get_mut("payload")).map_or(Value::Null, |value| std::mem::replace(value, Value::Null))
    }
}

impl Puzzle5dBoardEventsWork {
    /// 🪪️ Binds the operation nonce its minted ids derive from.
    pub fn bound(mut self, operation_nonce: u64) -> Self {
        self.operation_nonce = operation_nonce;
        self
    }
}

impl<A: Puzzle5dEditor> semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<A>> for Puzzle5dBoardEventsWork {
    fn tool_id(&self) -> &'static str {
        "applyBoardEvents"
    }

    fn extent(&self, command: &Puzzle5dCommand, snapshot: &Puzzle5dPlaySnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<A>>>) -> Option<usize> {
        let projection = puzzle5d_editor_projection(snapshot);
        let bytes = Self::source(command).ok()?.len();
        let document_items = projection.get("parts").and_then(Value::as_array).map_or(0, Vec::len).checked_add(projection.get("fasteners").and_then(Value::as_array).map_or(0, Vec::len))?.checked_add(2)?;
        let items = bytes.checked_mul(document_items)?;
        (bytes <= crate::retained_command::PUZZLE_COMMAND_RAW_BYTES && items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items.max(1))
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        crate::retained_command::step_demands::<EditorApp<A>>(size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, cx: &mut StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<A>>, semio_framework_job::OriginalTurnFailure> {
        let original: Result<_, Fault> = (|| {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config: _, interaction: _, hover: _hover, context, .. } = *input;
        let view_state = context.and_then(|context| context.view_state.as_ref());
        let window_config_snapshot = context.and_then(|context| context.window_config.as_ref());
        let projection = puzzle5d_editor_projection(snapshot);
        let source = Self::source(command)?;
        match self.stage {
            Puzzle5dBoardEventsStage::Open | Puzzle5dBoardEventsStage::Scan => {
                self.scan_one(source)?;
                Ok(crate::puzzle_progress_step!("puzzle5d-board-event-scan", "Reading board event", "Board-Ereignis wird gelesen"))
            }
            Puzzle5dBoardEventsStage::Decode => {
                let start = self.event_start.ok_or_else(|| Fault::from("puzzle5d-board-events-event-owner-missing"))?;
                self.event = Some(parse(source.get(start..self.event_end).ok_or_else(|| Fault::from("puzzle5d-board-events-event-range"))?, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|_| Fault::from("puzzle5d-board-events-event-malformed"))?);
                self.stage = Puzzle5dBoardEventsStage::Dispatch;
                Ok(crate::puzzle_progress_step!("puzzle5d-board-event-decode", "Decoding board event", "Board-Ereignis wird dekodiert"))
            }
            Puzzle5dBoardEventsStage::Dispatch => {
                let name = self.event.as_ref().and_then(|event| event.get("name")).and_then(Value::as_str).map(str::to_string);
                let payload = self.take_payload();
                match name.as_deref() {
                    Some("camera") => {
                        self.camera2d = Some(<Puzzle5dCamera2d as FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&payload)).map_err(|_| Fault::from("puzzle5d-board-events-camera-malformed"))?);
                        self.next_event();
                    }
                    // 🎬️ A board drag is ONE `drag` gesture record: its targets (each once) move by its offset as a
                    // `drag-selection2d` leaf the transform tool yields at completion — relative, so the drag replays
                    // on whatever flat poses its base holds. The board drag moves ONLY the flat pose: the world origin
                    // stays where the 3d pane put it, which is what makes a board drag a plan edit. Other gesture
                    // kinds move no part here.
                    Some("gesture") => {
                        let drag = payload.get("kind").and_then(Value::as_str) == Some("drag");
                        let offset = payload.get("dx").and_then(Value::as_f64).zip(payload.get("dy").and_then(Value::as_f64)).filter(|(dx, dy)| dx.is_finite() && dy.is_finite());
                        if let Some((dx, dy)) = offset.filter(|_| drag) {
                            let targets = payload.get("targets").and_then(Value::as_array).into_iter().flatten().filter_map(Value::as_str).filter(|id| !id.is_empty()).map(str::to_string);
                            let record = crate::editor::puzzle5d::modes::edit::windows::world3d::utilities::transform::Puzzle5dSelectionRecord::new(targets, crate::editor::puzzle5d::modes::edit::windows::world3d::utilities::transform::Puzzle5dSelectionMotion::Board { dx, dy });
                            if self.drags.len() >= crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS {
                                return Err(Fault::from("puzzle5d-board-events-drag-capacity"));
                            }
                            self.drag_at.get_or_insert(self.mutations.len());
                            self.drags.push(record);
                        }
                        self.next_event();
                    }
                    Some("edgeCreate") => {
                        self.pending_source = payload.get("source").and_then(Value::as_str).filter(|id| !id.is_empty()).map(str::to_string);
                        self.pending_target = payload.get("target").and_then(Value::as_str).filter(|id| !id.is_empty()).map(str::to_string);
                        self.pending_edge_id = payload.get("id").and_then(Value::as_str).filter(|id| !id.is_empty()).map(str::to_string);
                        if self.pending_edge_id.is_none() {
                            self.pending_edge_id = Some(format!("fastener-{:016x}-{}", self.operation_nonce, self.fresh_cursor));
                            self.fresh_cursor = self.fresh_cursor.saturating_add(1);
                        }
                        self.pending_edge_kind = payload.get("edgeKind").and_then(Value::as_str).filter(|kind| !kind.is_empty()).map(str::to_string);
                        self.fastener_cursor = 0;
                        self.stage = Puzzle5dBoardEventsStage::ScanEdge;
                    }
                    Some("edgeDelete") => {
                        if let Some(id) = payload.get("id").and_then(Value::as_str).filter(|id| !id.is_empty()) {
                            self.push(crate::standards::v1::subsets::any::schema::mutations::disconnect_grips(id.to_string()))?;
                            self.removed_ids.push(id.to_string());
                        }
                        self.next_event();
                    }
                    Some("select") => {
                        self.select_ids = Some(payload.get("ids").and_then(Value::as_array).map(|ids| ids.iter().filter_map(Value::as_str).map(str::to_string).collect()).unwrap_or_default());
                        self.next_event();
                    }
                    Some("nodeDelete") => {
                        self.pending_delete_id = payload.get("id").and_then(Value::as_str).filter(|id| !id.is_empty()).map(str::to_string);
                        self.fastener_cursor = 0;
                        self.stage = Puzzle5dBoardEventsStage::ScanDeleteEdges;
                    }
                    Some("brushPlace") => {
                        let mut brush = Puzzle5dAddBrushPartWork::from_board_payload(payload);
                        brush.operation_nonce = self.operation_nonce;
                        brush.fresh_cursor = self.fresh_cursor;
                        self.fresh_cursor = self.fresh_cursor.saturating_add(2);
                        self.brush = Some(brush);
                        self.stage = Puzzle5dBoardEventsStage::Brush;
                    }
                    _ => self.next_event(),
                }
                Ok(crate::puzzle_progress_step!("puzzle5d-board-event-dispatch", "Applying board event", "Board-Ereignis wird angewendet"))
            }
            Puzzle5dBoardEventsStage::ScanEdge => {
                if self.pending_source.is_none() || self.pending_target.is_none() {
                    self.next_event();
                    return Ok(crate::puzzle_progress_step!("puzzle5d-board-edge", "Checking board edge", "Board-Kante wird geprüft"));
                }
                if let Some(fastener) = projection.get("fasteners").and_then(Value::as_array).and_then(|fasteners| fasteners.get(self.fastener_cursor)) {
                    self.fastener_cursor += 1;
                    let source = fastener.get("source").and_then(Value::as_str);
                    let target = fastener.get("target").and_then(Value::as_str);
                    if (source == self.pending_source.as_deref() && target == self.pending_target.as_deref()) || (source == self.pending_target.as_deref() && target == self.pending_source.as_deref()) {
                        self.pending_source = None;
                        self.pending_target = None;
                    }
                    return Ok(crate::puzzle_progress_step!("puzzle5d-board-edge", "Checking board edge", "Board-Kante wird geprüft"));
                }
                let id = self.pending_edge_id.take().expect("preflighted edge id");
                let source = self.pending_source.take().expect("preflighted edge source");
                let target = self.pending_target.take().expect("preflighted edge target");
                let kind = self.pending_edge_kind.take();
                self.push(crate::standards::v1::subsets::any::schema::mutations::connect_grips(id, source, target, kind, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, None))?;
                self.next_event();
                Ok(crate::puzzle_progress_step!("puzzle5d-board-edge", "Creating board edge", "Board-Kante wird erstellt"))
            }
            Puzzle5dBoardEventsStage::ScanDeleteEdges => {
                let Some(id) = self.pending_delete_id.as_deref() else {
                    self.next_event();
                    return Ok(crate::puzzle_progress_step!("puzzle5d-board-delete", "Deleting board node", "Board-Knoten wird gelöscht"));
                };
                if let Some(fastener) = projection.get("fasteners").and_then(Value::as_array).and_then(|fasteners| fasteners.get(self.fastener_cursor)) {
                    self.fastener_cursor += 1;
                    let incident = fastener.get("source").and_then(Value::as_str).is_some_and(|grip| grip.split_once(':').is_some_and(|(part_id, _)| part_id == id))
                        || fastener.get("target").and_then(Value::as_str).is_some_and(|grip| grip.split_once(':').is_some_and(|(part_id, _)| part_id == id));
                    if incident {
                        if let Some(fastener_id) = fastener.get("id").and_then(Value::as_str) {
                            self.push(crate::standards::v1::subsets::any::schema::mutations::disconnect_grips(fastener_id.to_string()))?;
                        }
                    }
                    return Ok(crate::puzzle_progress_step!("puzzle5d-board-delete-edge", "Removing attached edge", "Verbundene Kante wird entfernt"));
                }
                let id = self.pending_delete_id.take().expect("preflighted deleted part");
                self.removed_ids.push(id.clone());
                self.push(crate::standards::v1::subsets::any::schema::mutations::delete_part(id))?;
                self.next_event();
                Ok(crate::puzzle_progress_step!("puzzle5d-board-delete", "Deleting board node", "Board-Knoten wird gelöscht"))
            }
            Puzzle5dBoardEventsStage::Brush => {
                let brush = self.brush.as_mut().ok_or_else(|| Fault::from("puzzle5d-board-brush-owner-missing"))?;
                match <Puzzle5dAddBrushPartWork as semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<A>>>::step(brush, input, cx)? {
                    semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Progress { stage, preview } => Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Progress { stage, preview }),
                    semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Replay { stage, preview } => Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Replay { stage, preview }),
                    semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(emit) => {
                        self.placed_id = emit.interaction_writes.iter().flat_map(|write| write.targets.iter()).map(|target| target.id.clone()).next();
                        let mut mutations = emit.artifact_mutations.into_iter();
                        self.brush_first = mutations.next();
                        self.brush_second = mutations.next();
                        if mutations.next().is_some() {
                            return Err(Fault::from("puzzle5d-board-brush-output-capacity"));
                        }
                        self.stage = Puzzle5dBoardEventsStage::DrainBrush;
                        Ok(crate::puzzle_progress_step!("puzzle5d-board-brush-transfer", "Publishing brush mutation", "Pinselmutation wird veröffentlicht"))
                    }
                    semio_framework_plugin::retained_command::ArtifactCommandWorkStep::CompleteDownload { .. } | semio_framework_plugin::retained_command::ArtifactCommandWorkStep::CompleteWithEphemeral { .. } => Err(Fault::from("puzzle5d-board-brush-download-unsupported")),
                }
            }
            Puzzle5dBoardEventsStage::DrainBrush => {
                if let Some(mutation) = self.brush_first.take() {
                    self.push(mutation)?;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-board-brush-transfer", "Publishing brush mutation", "Pinselmutation wird veröffentlicht"));
                }
                if let Some(mutation) = self.brush_second.take() {
                    self.push(mutation)?;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-board-brush-transfer", "Publishing brush mutation", "Pinselmutation wird veröffentlicht"));
                }
                if let Some(brush) = self.brush.as_mut() {
                    <Puzzle5dAddBrushPartWork as semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<A>>>::begin_close(brush);
                }
                self.stage = Puzzle5dBoardEventsStage::CloseBrush;
                Ok(crate::puzzle_progress_step!("puzzle5d-board-brush-close", "Releasing brush owners", "Pinseleigentümer werden freigegeben"))
            }
            Puzzle5dBoardEventsStage::CloseBrush => {
                let Some(brush) = self.brush.as_mut() else {
                    self.next_event();
                    return Ok(crate::puzzle_progress_step!("puzzle5d-board-event-scan", "Reading board event", "Board-Ereignis wird gelesen"));
                };
                match <Puzzle5dAddBrushPartWork as semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<A>>>::close_step(brush, cx).map_err(|_| Fault::from("puzzle5d-board-brush-close-refused"))? {
                    semio_framework_job::InteractiveJobCloseStep::Pending { .. } | semio_framework_job::InteractiveJobCloseStep::Complete { .. } => {}
                    semio_framework_job::InteractiveJobCloseStep::Blocked => return Err(Fault::from("puzzle5d-board-brush-close-blocked")),
                }
                if <Puzzle5dAddBrushPartWork as semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<A>>>::terminal_is_empty(brush) {
                    self.brush.take();
                    self.next_event();
                }
                Ok(crate::puzzle_progress_step!("puzzle5d-board-brush-close", "Releasing brush owners", "Pinseleigentümer werden freigegeben"))
            }
            Puzzle5dBoardEventsStage::Complete => {
                self.stage = Puzzle5dBoardEventsStage::Closing;
                // 🪟️ A board `camera` row is the pane's own persisted pose. Dropping it (which this work did
                // until 5A2) made pan/zoom in the board window revert on every refresh.
                let camera2d = self.camera2d.take();
                let window_config_mutations = match (camera2d, view_state) {
                    (Some(camera2d), Some(view)) => {
                        let mut next = window_ownership::config_from_snapshot(window_config_snapshot);
                        next.camera2d = camera2d;
                        vec![window_ownership::addressed_config(view, next)?]
                    }
                    _ => Vec::new(),
                };
                let mut interaction_writes = Vec::new();
                match (self.select_ids.take(), self.placed_id.take()) {
                    (Some(ids), _) => interaction_writes.push(InteractionWrite::replace(PUZZLE5D_INTERACTION_DOMAIN, PUZZLE5D_GRANULARITY_PART, ids)),
                    (None, Some(id)) => interaction_writes.push(InteractionWrite::replace(PUZZLE5D_INTERACTION_DOMAIN, PUZZLE5D_GRANULARITY_PART, [id])),
                    (None, None) => {}
                }
                let removed = std::mem::take(&mut self.removed_ids);
                if !removed.is_empty() {
                    // 🧹️ A delete clears what it removed through a SUBTRACTIVE write naming exactly those ids;
                    // an empty `Replace` is a silent no-op in `protocol::next_selection`.
                    let targets: Vec<InteractionTarget> = removed
                        .into_iter()
                        .flat_map(|id| [InteractionTarget { granularity: PUZZLE5D_GRANULARITY_PART.to_string(), id: id.clone() }, InteractionTarget { granularity: PUZZLE5D_GRANULARITY_FASTENER.to_string(), id }])
                        .collect();
                    interaction_writes.push(InteractionWrite { domain: PUZZLE5D_INTERACTION_DOMAIN.into(), targets, merge: MergeMode::Subtractive });
                }
                let drags = std::mem::take(&mut self.drags);
                let drag_at = self.drag_at.take().unwrap_or(self.mutations.len()).min(self.mutations.len());
                let base = snapshot.typed_arc();
                // 🔒️ A locked part refuses the drag; the rest of the batch still lands, and the leaf names the
                // skipped part as `mutation.partial`.
                self.locked_refused |= drags.iter().any(|record| record.targets.iter().any(|id| base.parts.iter().any(|part| &part.id == id && part.part_2d.locked == Some(true))));
                let request = crate::editor::puzzle5d::modes::edit::windows::world3d::utilities::transform::TransformToolRequest { base, records: drags };
                let committed = (!request.records.is_empty()).then(|| crate::editor::puzzle5d::modes::edit::windows::world3d::utilities::transform::puzzle5d_transform_tool_commit("applyBoardEvents", &self.authoring_seed, request)).flatten();
                let mut artifact_mutations = std::mem::take(&mut self.mutations);
                let transaction = committed.map(|(transaction, leaves)| {
                    let later = artifact_mutations.split_off(drag_at);
                    artifact_mutations.extend(leaves);
                    artifact_mutations.extend(later);
                    transaction
                });
                let effects = if self.locked_refused {
                    self.locked_refused = false;
                    puzzle5d_notice_emit(view_state, |labels| labels.selection_locked.as_str()).effects
                } else {
                    Vec::new()
                };
                // 🛠️ A committed drag transaction stamps every op of this ONE edit.
                Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit {
                    artifact_mutations,
                    window_config_mutations,
                    interaction_writes,
                    effects,
                    transaction: transaction.filter(|_| !self.authoring_seed.is_empty()),
                    ui_scope: UiDirtyScope::Full,
                    ..Default::default()
                }))
            }
            Puzzle5dBoardEventsStage::Closing => Err(Fault::from("puzzle5d-board-events-closing")),
        }
    
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle5dBoardEventsStage::Closing;
        if let Some(brush) = self.brush.as_mut().filter(|brush| !brush.is_closing()) {
            <Puzzle5dAddBrushPartWork as semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<A>>>::begin_close(brush);
        }
        self.close_owners.stage(Puzzle5dBoardEventsWorkOwners { mutations: std::mem::take(&mut self.mutations), brush_first: std::mem::take(&mut self.brush_first), brush_second: std::mem::take(&mut self.brush_second), event: std::mem::take(&mut self.event), drags: std::mem::take(&mut self.drags), drag_at: std::mem::take(&mut self.drag_at), pending_source: std::mem::take(&mut self.pending_source), pending_target: std::mem::take(&mut self.pending_target), pending_edge_id: std::mem::take(&mut self.pending_edge_id), pending_edge_kind: std::mem::take(&mut self.pending_edge_kind), pending_delete_id: std::mem::take(&mut self.pending_delete_id), camera2d: std::mem::take(&mut self.camera2d), select_ids: std::mem::take(&mut self.select_ids), removed_ids: std::mem::take(&mut self.removed_ids), placed_id: std::mem::take(&mut self.placed_id) });
    }

    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        let Some(brush) = self.brush.as_mut() else { return semio_framework_job::close_original_granted_step(cx,|grant|self.close_owners.close_step(grant)) };
        let step = <Puzzle5dAddBrushPartWork as semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<A>>>::close_step(brush, cx)?;
        if <Puzzle5dAddBrushPartWork as semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<A>>>::terminal_is_empty(brush) {
            self.brush.take();
        }
        Ok(match step {
            semio_framework_job::InteractiveJobCloseStep::Complete { progress } => semio_framework_job::InteractiveJobCloseStep::Pending { progress },
            other => other,
        })
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        if let Some(brush) = self.brush.as_ref() {
            return <Puzzle5dAddBrushPartWork as semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<A>>>::next_close_copy_byte_demand(brush);
        }
        Ok(self.close_owners.demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        if let Some(brush) = self.brush.as_ref() {
            return <Puzzle5dAddBrushPartWork as semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<A>>>::next_close_capacity_byte_demand(brush, maximum_copy_bytes);
        }
        Ok(self.close_owners.demands(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        if let Some(brush) = self.brush.as_ref() {
            return <Puzzle5dAddBrushPartWork as semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<A>>>::next_close_release_byte_demand(brush);
        }
        Ok(self.close_owners.demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        if let Some(brush) = self.brush.as_ref() {
            return <Puzzle5dAddBrushPartWork as semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<A>>>::next_close_depth_demand(brush);
        }
        Ok(self.close_owners.demands(0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage == Puzzle5dBoardEventsStage::Closing && self.brush.is_none() && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle5dCreateFastenerStage {
    Source,
    Target,
    Existing,
    Compatibility,
    Emit,
    Complete,
    Closing,
}

pub enum Puzzle5dGripScan {
    Progress,
    Found(Option<String>),
    Exhausted,
}

/// ♻️ The owners one `Puzzle5dCreateFastenerWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle5dCreateFastenerWorkOwners {
    pub mutation: Option<Puzzle5dMutation>,
    pub source_kind: Option<String>,
    pub target_kind: Option<String>,
}

pub struct Puzzle5dCreateFastenerWork {
    pub stage: Puzzle5dCreateFastenerStage,
    pub part_cursor: usize,
    pub grip_cursor: usize,
    pub fastener_cursor: usize,
    pub compatibility_cursor: usize,
    pub processed_units: usize,
    pub source_kind: Option<String>,
    pub target_kind: Option<String>,
    pub mutation: Option<Puzzle5dMutation>,
    pub operation_nonce: u64,
    pub close_owners: crate::puzzle_job::WorkClosing<Puzzle5dCreateFastenerWorkOwners>,
}

impl Default for Puzzle5dCreateFastenerWork {
    fn default() -> Self {
        Self { close_owners: Default::default(), stage: Puzzle5dCreateFastenerStage::Source, part_cursor: 0, grip_cursor: 0, fastener_cursor: 0, compatibility_cursor: 0, processed_units: 0, source_kind: None, target_kind: None, mutation: None, operation_nonce: 0 }
    }
}

impl Puzzle5dCreateFastenerWork {
    fn endpoint<'a>(command: &'a Puzzle5dCommand, primary: &str, alias: &str) -> &'a str {
        command.args().and_then(|args| args.get(primary).or_else(|| args.get(alias))).and_then(Value::as_str).filter(|id| !id.is_empty()).unwrap_or("")
    }

    pub fn scan_grip(&mut self, snapshot: &Puzzle5dPlaySnapshot, target: &str) -> Puzzle5dGripScan {
        let projection = puzzle5d_editor_projection(snapshot);
        let Some(part) = projection.get("parts").and_then(Value::as_array).and_then(|parts| parts.get(self.part_cursor)) else {
            return Puzzle5dGripScan::Exhausted;
        };
        let Some(grip) = part.get("grips").and_then(Value::as_array).and_then(|grips| grips.get(self.grip_cursor)) else {
            self.part_cursor += 1;
            self.grip_cursor = 0;
            return Puzzle5dGripScan::Progress;
        };
        self.grip_cursor += 1;
        let Some(part_id) = part.get("id").and_then(Value::as_str) else { return Puzzle5dGripScan::Progress };
        let Some(grip_id) = grip.get("id").and_then(Value::as_str) else { return Puzzle5dGripScan::Progress };
        if puzzle5d_grip_full_id(part_id, grip_id) != target {
            return Puzzle5dGripScan::Progress;
        }
        let kind = grip.get("gripKind").and_then(Value::as_str).filter(|kind| !kind.is_empty()).or_else(|| grip.get("2d").and_then(|value| value.get("gripKind")).and_then(Value::as_str).filter(|kind| !kind.is_empty())).map(str::to_string);
        Puzzle5dGripScan::Found(kind)
    }

    pub fn complete_empty<A: Puzzle5dEditor>(&mut self) -> semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<A>> {
        self.stage = Puzzle5dCreateFastenerStage::Complete;
        semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit::default())
    }

    fn arg_f64(command: &Puzzle5dCommand, key: &str) -> f64 {
        command.args().and_then(|args| args.get(key)).and_then(Value::as_f64).unwrap_or(0.0)
    }
}

impl Puzzle5dCreateFastenerWork {
    /// 🪪️ Binds the operation nonce its minted ids derive from.
    pub fn bound(mut self, operation_nonce: u64) -> Self {
        self.operation_nonce = operation_nonce;
        self
    }
}

impl<A: Puzzle5dEditor> semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<A>> for Puzzle5dCreateFastenerWork {
    fn tool_id(&self) -> &'static str {
        "createFastener"
    }

    fn extent(&self, _command: &Puzzle5dCommand, snapshot: &Puzzle5dPlaySnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<A>>>) -> Option<usize> {
        let projection = puzzle5d_editor_projection(snapshot);
        let parts = projection.get("parts").and_then(Value::as_array).map_or(0, Vec::len);
        let fasteners = projection.get("fasteners").and_then(Value::as_array).map_or(0, Vec::len);
        let compatibility = projection.get("kindCompatibility").and_then(Value::as_array).map_or(0, Vec::len);
        let items = parts.checked_mul(2)?.checked_add(fasteners)?.checked_add(compatibility)?.checked_add(1)?;
        (items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        crate::retained_command::step_demands::<EditorApp<A>>(size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _cx: &mut StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<A>>, semio_framework_job::OriginalTurnFailure> {
        let original: Result<_, Fault> = (|| {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config: _config, interaction: _interaction, hover: _hover, context: _context, .. } = *input;
        let projection = puzzle5d_editor_projection(snapshot);
        if self.processed_units >= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS {
            return Err(Fault::from("puzzle5d-create-fastener-work-capacity"));
        }
        self.processed_units += 1;
        let source = Self::endpoint(command, "source", "attracting");
        let target = Self::endpoint(command, "target", "attracted");
        if source.is_empty() || target.is_empty() || source == target {
            return Ok(self.complete_empty());
        }
        match self.stage {
            Puzzle5dCreateFastenerStage::Source => match self.scan_grip(snapshot, source) {
                Puzzle5dGripScan::Progress => Ok(crate::puzzle_progress_step!("puzzle5d-create-fastener-source", "Finding source grip", "Quellgriff wird gesucht")),
                Puzzle5dGripScan::Found(kind) => {
                    self.source_kind = kind;
                    self.part_cursor = 0;
                    self.grip_cursor = 0;
                    self.stage = Puzzle5dCreateFastenerStage::Target;
                    Ok(crate::puzzle_progress_step!("puzzle5d-create-fastener-target", "Finding target grip", "Zielgriff wird gesucht"))
                }
                Puzzle5dGripScan::Exhausted => Ok(self.complete_empty()),
            },
            Puzzle5dCreateFastenerStage::Target => match self.scan_grip(snapshot, target) {
                Puzzle5dGripScan::Progress => Ok(crate::puzzle_progress_step!("puzzle5d-create-fastener-target", "Finding target grip", "Zielgriff wird gesucht")),
                Puzzle5dGripScan::Found(kind) => {
                    self.target_kind = kind;
                    self.stage = Puzzle5dCreateFastenerStage::Existing;
                    Ok(crate::puzzle_progress_step!("puzzle5d-create-fastener-existing", "Checking existing fastener", "Bestehende Verbindung wird geprüft"))
                }
                Puzzle5dGripScan::Exhausted => Ok(self.complete_empty()),
            },
            Puzzle5dCreateFastenerStage::Existing => {
                if let Some(fastener) = projection.get("fasteners").and_then(Value::as_array).and_then(|fasteners| fasteners.get(self.fastener_cursor)) {
                    self.fastener_cursor += 1;
                    let existing_source = fastener.get("source").and_then(Value::as_str).unwrap_or("");
                    let existing_target = fastener.get("target").and_then(Value::as_str).unwrap_or("");
                    if (existing_source == source && existing_target == target) || (existing_source == target && existing_target == source) {
                        return Ok(self.complete_empty());
                    }
                    return Ok(crate::puzzle_progress_step!("puzzle5d-create-fastener-existing", "Checking existing fastener", "Bestehende Verbindung wird geprüft"));
                }
                self.stage = Puzzle5dCreateFastenerStage::Compatibility;
                Ok(crate::puzzle_progress_step!("puzzle5d-create-fastener-compatibility", "Checking kind compatibility", "Artkompatibilität wird geprüft"))
            }
            Puzzle5dCreateFastenerStage::Compatibility => {
                if self.source_kind.is_none() || self.target_kind.is_none() {
                    self.stage = Puzzle5dCreateFastenerStage::Emit;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-create-fastener-emit", "Creating fastener", "Verbindung wird erstellt"));
                }
                let rows = projection.get("kindCompatibility").and_then(Value::as_array).map(Vec::as_slice).unwrap_or_default();
                if rows.is_empty() {
                    self.stage = Puzzle5dCreateFastenerStage::Emit;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-create-fastener-emit", "Creating fastener", "Verbindung wird erstellt"));
                }
                let Some(row) = rows.get(self.compatibility_cursor) else { return Ok(self.complete_empty()) };
                self.compatibility_cursor += 1;
                let row_source = row.get("source").and_then(Value::as_str).unwrap_or("");
                let row_target = row.get("target").and_then(Value::as_str).unwrap_or("");
                let bidirectional = row.get("bidirectional").and_then(Value::as_bool).unwrap_or(false);
                let source_kind = self.source_kind.as_deref().unwrap_or("");
                let target_kind = self.target_kind.as_deref().unwrap_or("");
                if (row_source == source_kind && row_target == target_kind) || (bidirectional && row_source == target_kind && row_target == source_kind) {
                    self.stage = Puzzle5dCreateFastenerStage::Emit;
                }
                Ok(crate::puzzle_progress_step!("puzzle5d-create-fastener-compatibility", "Checking kind compatibility", "Artkompatibilität wird geprüft"))
            }
            Puzzle5dCreateFastenerStage::Emit => {
                let id = command.args().and_then(|args| args.get("id").or_else(|| args.get("fastenerId"))).and_then(Value::as_str).filter(|id| !id.is_empty()).map_or_else(|| format!("fastener-{:016x}-0", self.operation_nonce), str::to_string);
                let fastener_kind = command.args().and_then(|args| args.get("fastenerKind").or_else(|| args.get("edgeKind"))).and_then(Value::as_str).filter(|kind| !kind.is_empty()).map(str::to_string);
                self.mutation = Some(crate::standards::v1::subsets::any::schema::mutations::connect_grips(
                    id,
                    source.to_string(),
                    target.to_string(),
                    fastener_kind,
                    Self::arg_f64(command, "gap"),
                    Self::arg_f64(command, "shift"),
                    Self::arg_f64(command, "rise"),
                    Self::arg_f64(command, "rotation"),
                    Self::arg_f64(command, "turn"),
                    Self::arg_f64(command, "tilt"),
                    Self::arg_f64(command, "x"),
                    Self::arg_f64(command, "y"), None,
                ));
                self.stage = Puzzle5dCreateFastenerStage::Complete;
                Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit { artifact_mutations: self.mutation.take().into_iter().collect(), ui_scope: UiDirtyScope::Full, ..Default::default() }))
            }
            Puzzle5dCreateFastenerStage::Complete => Err(Fault::from("puzzle5d-create-fastener-complete-repolled")),
            Puzzle5dCreateFastenerStage::Closing => Err(Fault::from("puzzle5d-create-fastener-closing")),
        }
    
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle5dCreateFastenerStage::Closing;
        self.close_owners.stage(Puzzle5dCreateFastenerWorkOwners { mutation: std::mem::take(&mut self.mutation), source_kind: std::mem::take(&mut self.source_kind), target_kind: std::mem::take(&mut self.target_kind) });
    }

    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        semio_framework_job::close_original_granted_step(cx,|grant|{
            self.close_owners.close_step(grant)
        })
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage == Puzzle5dCreateFastenerStage::Closing && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
}
//#endregion 🧵️RetainedCommands
//#endregion 🔖️PlayApp
