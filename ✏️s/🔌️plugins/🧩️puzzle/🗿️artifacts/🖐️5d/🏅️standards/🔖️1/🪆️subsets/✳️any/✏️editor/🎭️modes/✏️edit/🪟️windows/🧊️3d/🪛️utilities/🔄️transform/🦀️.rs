//! 🔄️ 3D-window utility — Transform: the world gumball, puzzle 3d's `🪛️utilities/🔄️transform` twin. Its
//! Utility Options are the Move/Rotate flags that compose which handles the gumball draws (scale handles
//! are deliberately absent — a part's scale comes from its kind catalog, not from a free drag). Bound
//! only by the 3D world window — the 2D board window drags parts natively.
//!
//! 🛠️ It is also the transform TOOL, the twin of puzzle 3d's: a `🔄️machine` statechart whose effects are
//! `ToolYield`s, driven by the `🛠️tool-machine` runner. Every selection transform — a gumball
//! translate/rotate/scale, a typed `move`/`rotate`/`scale` submit, a target-volume gumball relocate, a world
//! drop, an inspector `x`/`y`/origin `delta`, a board drag — enters as a [`Puzzle5dSelectionRecord`] and leaves
//! as ONE `ToolTransaction`: the parametric `drag-selection2d`/`drag-selection3d`/`rotate-selection3d`/
//! `scale-selection3d` leaf plus the `connect-grips` a drop fastens, targets and fastener ids literal. Tool
//! state is never history; the yielded mutations are (design
//! `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/📋️design.md` §5, §8).

use semio_framework_pack_json::json;
use crate::editor::puzzle5d::config::Puzzle5dRuntime;
use crate::editor::puzzle5d::terminology::Puzzle5dLabels;
use crate::editor::puzzle5d::{puzzle5d_action, puzzle5d_grip_full_id, quat_rotate_vector, PUZZLE5D_PLAY_CONTROLLER_ID};
use crate::standards::v1::subsets::any::schema::mutations::{apply_puzzle5d_mutation,connect_grips,drag_selection_2d,drag_selection_3d,rotate_selection_3d,scale_selection_3d,Puzzle5dMutation};

use crate::Puzzle5dSnapshot;

use machine::Command;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::UtilityDefinition;
use semio_framework_plugin::WindowMeasure;
use semio_framework_tool_machine::{GestureChart, GesturePhase, ToolYield};
use semio_s_artifact_puzzle_3d::editor::puzzle3d::modes::edit::windows::main::utilities::transform::{puzzle3d_unique_targets, Puzzle3dSelectionMotion, Puzzle3dSelectionRecord};
use std::sync::Arc;

pub const UTILITY_ID: &str = "transform";

/// 🪪️ The editor app id every transform-tool transaction's `tool` is scoped by: `<appId>#<verb>`.
pub const PUZZLE5D_EDITOR_APP_ID: &str = "s.puzzle.puzzle5d@1/*#editor";

/// 🧱️ Stitched into the app manifest by `crate::editor::puzzle5d::create_puzzle5d_app`.
pub fn definition() -> UtilityDefinition {
    UtilityDefinition::new(UTILITY_ID, LocalizedLabel::native("Transform", "Transformieren"), "transform-3d")
}

/// 🎛️ Utility Options for the Transform utility — the Move and Rotate flags (`setTransformGumballFlag`);
/// with both off `puzzle5d_gumball_active` refuses to render a gumball nobody could grab. Tagged with this
/// utility's id as a routing envelope only; `partition_window_measures` unwraps the children so they render
/// flat under the Transform toggle (the toggle already owns that row, hence the empty group label).
pub fn options(runtime: &Puzzle5dRuntime, labels: &Puzzle5dLabels) -> WindowMeasure {
    WindowMeasure::Group {
        id: format!("{PUZZLE5D_PLAY_CONTROLLER_ID}-utility-options-{UTILITY_ID}"),
        label: String::new(),
        default_open: Some(true),
        active_utility_id: Some(UTILITY_ID.into()),
        value: None,
        min: None,
        max: None,
        step: None,
        ready: None,
        loading: None,
        waiting: None,
        on_change: None,
        children: vec![
            WindowMeasure::Toggle {
                id: "puzzle5d-transform-move".into(),
                icon_id: "move-3d".into(),
                label: Some(labels.move_handle.into()),
                pressed: runtime.transform_move,
                text: None,
                on_change: puzzle5d_action("setTransformGumballFlag", Some(semio_framework_pack_json::json!({ "flag": "move" }))),
            },
            WindowMeasure::Toggle {
                id: "puzzle5d-transform-rotate".into(),
                icon_id: "rotate-cw".into(),
                label: Some(labels.rotate_handle.into()),
                pressed: runtime.transform_rotate,
                text: None,
                on_change: puzzle5d_action("setTransformGumballFlag", Some(semio_framework_pack_json::json!({ "flag": "rotate" }))),
            },
        ],
    }
}

//#region 🎬️Record
/// 🎬️ How one selection transform moves its targets: a board drag of the flat poses, or a world motion of
/// the spatial ones — puzzle 3d's own [`Puzzle3dSelectionMotion`], so both artifacts read one gesture vocabulary.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Puzzle5dSelectionMotion {
    Board { dx: f64, dy: f64 },
    World(Puzzle3dSelectionMotion),
}

impl Puzzle5dSelectionMotion {
    /// 🏃️ Whether the motion is admissible and moves: a finite non-zero board offset, or a moving world motion.
    pub fn moves(&self) -> bool {
        match *self {
            Self::Board { dx, dy } => dx.is_finite() && dy.is_finite() && (dx != 0.0 || dy != 0.0),
            Self::World(motion) => motion.moves(),
        }
    }
}

/// 🎬️ One selection transform the transform tool yields: the literal target ids, the motion, and the
/// `(source, target)` full grip ids its drop fastens once the targets moved.
#[derive(Clone, Debug, PartialEq)]
pub struct Puzzle5dSelectionRecord {
    pub targets: Vec<String>,
    pub motion: Puzzle5dSelectionMotion,
    pub fastenings: Vec<(String, String)>,
}

impl Puzzle5dSelectionRecord {
    /// 🎯️ A transform of `targets` (deduplicated in first-seen order) that fastens nothing.
    pub fn new(targets: impl IntoIterator<Item = String>, motion: Puzzle5dSelectionMotion) -> Self {
        Self { targets: puzzle3d_unique_targets(targets), motion, fastenings: Vec::new() }
    }

    /// 🌍️ The world record puzzle 3d's record parsers state — a gumball pose delta
    /// ([`Puzzle3dSelectionRecord::from_gumball`]) or a target-volume relocate
    /// ([`Puzzle3dSelectionRecord::from_pose_delta`]).
    pub fn world(record: Puzzle3dSelectionRecord) -> Self {
        Self::new(record.targets, Puzzle5dSelectionMotion::World(record.motion))
    }

    /// 🧮️ The parametric leaf this record yields, over its targets deduplicated in first-seen order.
    pub fn mutation(&self) -> Puzzle5dMutation {
        let targets = puzzle3d_unique_targets(self.targets.iter().cloned());
        match self.motion {
            Puzzle5dSelectionMotion::Board { dx, dy } => drag_selection_2d(targets, dx, dy),
            Puzzle5dSelectionMotion::World(Puzzle3dSelectionMotion::Drag { offset }) => drag_selection_3d(targets, offset),
            Puzzle5dSelectionMotion::World(Puzzle3dSelectionMotion::Rotate { axis, angle }) => rotate_selection_3d(targets, axis, angle),
            Puzzle5dSelectionMotion::World(Puzzle3dSelectionMotion::Scale { factors }) => scale_selection_3d(targets, factors),
        }
    }

    /// 🔎️ Whether this record moves anything on `base`: a moving motion and at least one target that is a part
    /// whose board pin is unlocked, or — for a world motion — an unlocked target volume.
    pub fn applies_to(&self, base: &Puzzle5dSnapshot) -> bool {
        let world = matches!(self.motion, Puzzle5dSelectionMotion::World(_));
        self.motion.moves() && self.targets.iter().any(|id| base.parts.iter().any(|part| &part.id == id && part.part_2d.locked != Some(true)) || (world && base.target_volumes.iter().any(|volume| &volume.id == id && !volume.locked)))
    }

    /// 🔒️ The tool-level refusal: the motion moves, yet nothing this record names can, and a lock is why.
    pub fn refused_as_locked(&self, base: &Puzzle5dSnapshot) -> bool {
        self.motion.moves() && !self.applies_to(base) && self.targets.iter().any(|id| base.parts.iter().any(|part| &part.id == id && part.part_2d.locked == Some(true)) || base.target_volumes.iter().any(|volume| &volume.id == id && volume.locked))
    }

    /// 🔭️ Whether any target this record names is in `base` at all.
    pub fn names_any(&self, base: &Puzzle5dSnapshot) -> bool {
        self.targets.iter().any(|id| base.parts.iter().any(|part| &part.id == id) || base.target_volumes.iter().any(|volume| &volume.id == id))
    }
}

/// 🚚️ The record one world drop states: `part_id` dragged from its BASE origin to `position`, and every grip its
/// first grip lands within `radius` of — on another part, not yet fastened to it — as a `(source, target)` pair,
/// the moved grip the source. `None` for an unknown part. The whole scan in one call; the retained work pages it
/// through [`Puzzle5dRelocateScan`].
pub fn puzzle5d_relocate_record(base: &Puzzle5dSnapshot, part_id: &str, position: [f64; 3], radius: f64) -> Option<Puzzle5dSelectionRecord> {
    let mut scan = Puzzle5dRelocateScan::begin(base, part_id, position, radius)?;
    while !scan.step(base, usize::MAX) {}
    Some(scan.finish())
}

/// 📄️ Parts one page of a world-drop proximity scan measures — the retained work's per-step share.
pub const PUZZLE5D_RELOCATE_SCAN_PAGE: usize = 16;

/// 🔭️ The paged proximity scan of one world drop: [`Self::begin`] states the drag and indexes the grips already
/// fastened to the moved one, every [`Self::step`] measures the grips of one page of parts, and [`Self::finish`]
/// hands over the record — so a drop on a large puzzle reports progress page by page and stays cancellable between
/// pages, with exactly the pairs (and order) the one-call scan finds.
#[derive(Clone, Debug, PartialEq)]
pub struct Puzzle5dRelocateScan {
    record: Puzzle5dSelectionRecord,
    source: Option<(String, [f64; 3])>,
    fastened: std::collections::HashSet<String>,
    radius: f64,
    cursor: usize,
}

impl Puzzle5dRelocateScan {
    /// 🎬️ The scan of `part_id` dropped at `position`; `None` for an unknown part. A part without a grip fastens
    /// nothing, so its scan is already done.
    pub fn begin(base: &Puzzle5dSnapshot, part_id: &str, position: [f64; 3], radius: f64) -> Option<Self> {
        let part = base.parts.iter().find(|part| part.id == part_id)?;
        let offset = [position[0] - part.part_3d.origin[0], position[1] - part.part_3d.origin[1], position[2] - part.part_3d.origin[2]];
        let source = part.grips.first().map(|grip| (puzzle5d_grip_full_id(&part.id, &grip.id), puzzle5d_world_point(position, part.part_3d.orientation, grip.grip_3d.position)));
        let fastened = source.as_ref().map_or_else(Default::default, |(source, _)| {
            base.fasteners.iter().filter_map(|entry| if &entry.source == source { Some(entry.target.clone()) } else if &entry.target == source { Some(entry.source.clone()) } else { None }).collect()
        });
        let cursor = if source.is_some() { 0 } else { base.parts.len() };
        Some(Self { record: Puzzle5dSelectionRecord { targets: vec![part.id.clone()], motion: Puzzle5dSelectionMotion::World(Puzzle3dSelectionMotion::Drag { offset }), fastenings: Vec::new() }, source, fastened, radius: radius.max(0.0), cursor })
    }

    /// 📏️ Measures the grips of the next `page` parts (at least one); `true` once every part is measured.
    pub fn step(&mut self, base: &Puzzle5dSnapshot, page: usize) -> bool {
        let end = self.cursor.saturating_add(page.max(1)).min(base.parts.len());
        if let Some((source, from)) = self.source.as_ref() {
            for other in base.parts.get(self.cursor..end).into_iter().flatten().filter(|other| other.id != self.record.targets[0]) {
                for candidate in &other.grips {
                    let target = puzzle5d_grip_full_id(&other.id, &candidate.id);
                    let at = puzzle5d_world_point(other.part_3d.origin, other.part_3d.orientation, candidate.grip_3d.position);
                    if target != *source && !self.fastened.contains(&target) && ((from[0] - at[0]).powi(2) + (from[1] - at[1]).powi(2) + (from[2] - at[2]).powi(2)).sqrt() <= self.radius {
                        self.record.fastenings.push((source.clone(), target));
                    }
                }
            }
        }
        self.cursor = end;
        self.cursor >= base.parts.len()
    }

    /// 📊️ `(measured parts, all parts)` of `base`.
    pub fn progress(&self, base: &Puzzle5dSnapshot) -> (usize, usize) {
        (self.cursor.min(base.parts.len()), base.parts.len())
    }

    /// 🏁️ The scanned drop record.
    pub fn finish(self) -> Puzzle5dSelectionRecord {
        self.record
    }
}

fn puzzle5d_world_point(origin: [f64; 3], orientation: Option<[f64; 4]>, local: [f64; 3]) -> [f64; 3] {
    let turned = quat_rotate_vector(orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]), local);
    [origin[0] + turned[0], origin[1] + turned[1], origin[2] + turned[2]]
}
//#endregion 🎬️Record

//#region 🛠️TransformTool
/// 📨️ What one transform-tool event carries: the committed document it yields against and the records —
/// dispatch inputs, never tool state.
#[derive(Clone, Debug)]
pub struct TransformToolRequest {
    pub base: Arc<Puzzle5dSnapshot>,
    pub records: Vec<Puzzle5dSelectionRecord>,
}

/// 🧰️ The transform tool's context: a gesture reaches the guest as ONE release dispatch, so the tool keeps
/// nothing between events.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TransformToolContext;

fn transform_tool_context(input: TransformToolContext) -> TransformToolContext {
    input
}

fn records_apply(_context: &TransformToolContext, event: Option<&transform_tool::Event>) -> bool {
    matches!(event, Some(transform_tool::Event::Records(request)) if request.records.iter().any(|record| record.applies_to(&request.base)))
}

fn yield_records(_context: &mut TransformToolContext, event: Option<&transform_tool::Event>, sink: &mut Vec<Command<transform_tool::TransformTool>>) {
    let Some(transform_tool::Event::Records(request)) = event else { return };
    sink.extend(puzzle5d_selection_yields(&request.base, &request.records).into_iter().map(|(key, mutation)| Command::Effect(ToolYield::upsert(key, mutation))));
    sink.push(Command::Effect(ToolYield::Commit));
}

machine::statechart! {
    machine transform_tool {
        context: TransformToolContext;
        event Event { Records(TransformToolRequest) }
        input: TransformToolContext;
        output: ();
        effect: ToolYield<Puzzle5dMutation>;
        context_from_input: transform_tool_context;
        initial: idle;
        state idle {
            on Records if records_apply => idle do yield_records;
        }
    }
}

/// 🧷️ The transform tool's host: its chart declares no timer, no invoke and no foreign effect, so every duty
/// is empty.
pub struct TransformToolHost;

impl machine::Host<transform_tool::TransformTool> for TransformToolHost {
    fn execute_effect(&mut self, _actor: machine::ActorId, _effect: ToolYield<Puzzle5dMutation>) {}
    fn schedule(&mut self, _actor: machine::ActorId, _timer: machine::TimerId, _delay_ms: u64) {}
    fn cancel_timer(&mut self, _actor: machine::ActorId, _timer: machine::TimerId) {}
    fn start_task(&mut self, _actor: machine::ActorId, _invoke: machine::InvokeId) {}
    fn cancel_task(&mut self, _actor: machine::ActorId, _invoke: machine::InvokeId) {}
    fn now_ms(&self) -> u64 {
        semio_framework_job::default_now_ms().unwrap_or(0)
    }
}

/// 🧭️ The released transform on the shared gesture runner; every event is a one-step transaction and no
/// chart context survives its release. The parametric leaves fold against the document the dispatch receives.
impl GestureChart for transform_tool::TransformTool {
    type Tick = TransformToolRequest;
    type Host = TransformToolHost;

    const BASE_BOUND: bool = false;

    fn tool(verb: &str) -> String {
        format!("{PUZZLE5D_EDITOR_APP_ID}#{verb}")
    }

    fn host() -> TransformToolHost {
        TransformToolHost
    }

    fn input() -> TransformToolContext {
        TransformToolContext
    }

    fn restore(_entries: &[(String, Puzzle5dMutation)], _context: &semio_framework_value::DslValue) -> Option<TransformToolContext> {
        None
    }

    fn event(phase: GesturePhase, _at_rest: bool, tick: Option<TransformToolRequest>) -> Option<transform_tool::Event> {
        tick.filter(|_| phase == GesturePhase::Once).map(transform_tool::Event::Records)
    }
}

/// 🛠️ One release through the shared gesture driver: its transaction and yielded mutations, or zero trace
/// for an empty, locked, missing or motionless request. The framework owns the clock and transaction identity.
pub fn puzzle5d_transform_tool_commit(verb: &str, authoring_seed: &str, request: TransformToolRequest) -> Option<(protocol::TransactionRef, Vec<Puzzle5dMutation>)> {
    semio_framework_tool_machine::drive_chart_gesture::<transform_tool::TransformTool>(None, verb, GesturePhase::Once, Some(request), authoring_seed, "").ok()?.committed
}

/// 🧮️ What the transform tool yields for `records` on `base`, keyed: each moving record's parametric leaf,
/// then the `connect-grips` its drop fastens — each pair still free on the moved state, with zero offsets as
/// the drop always wrote them, its id minted HERE, deterministically from the pair and the document, so a
/// replay never mints again.
pub fn puzzle5d_selection_yields(base: &Puzzle5dSnapshot, records: &[Puzzle5dSelectionRecord]) -> Vec<(String, Puzzle5dMutation)> {
    let fastens = records.iter().any(|record| !record.fastenings.is_empty());
    let mut state = fastens.then(|| base.clone());
    let mut yields = Vec::new();
    for (index, record) in records.iter().enumerate() {
        let current = state.as_ref().unwrap_or(base);
        if !record.applies_to(current) {
            continue;
        }
        let leaf = record.mutation();
        if let Some(state) = state.as_mut() {
            if apply_puzzle5d_mutation(state, &leaf).is_err() {
                continue;
            }
        }
        yields.push((format!("selection:{index}"), leaf));
        let Some(state) = state.as_mut() else { continue };
        for (source, target) in &record.fastenings {
            let fastened = state.fasteners.iter().any(|entry| (&entry.source, &entry.target) == (source, target) || (&entry.source, &entry.target) == (target, source));
            let owner = |full_id: &str| state.parts.iter().find(|part| part.grips.iter().any(|grip| puzzle5d_grip_full_id(&part.id, &grip.id) == full_id)).map(|part| part.id.clone());
            let (Some(source_part), Some(target_part)) = (owner(source), owner(target)) else { continue };
            if fastened || source == target || source_part == target_part {
                continue;
            }
            let id = puzzle5d_minted_fastener_id(state, source, target);
            let connect = connect_grips(id.clone(), source.clone(), target.clone(), None, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, None);
            if apply_puzzle5d_mutation(state, &connect).is_ok() {
                yields.push((format!("fastener:{id}"), connect));
            }
        }
    }
    yields
}

/// 🆔️ The fastener id a yielded connection carries: `fastener-<source>-<target>`, suffixed `-2`, `-3`, … past
/// any id the document already holds — a pure function of the pair and the document, never a process counter.
pub fn puzzle5d_minted_fastener_id(document: &Puzzle5dSnapshot, source: &str, target: &str) -> String {
    let candidate = format!("fastener-{source}-{target}");
    let taken = |id: &str| document.fasteners.iter().any(|fastener| fastener.id == id);
    if !taken(&candidate) {
        return candidate;
    }
    (2usize..).map(|serial| format!("{candidate}-{serial}")).find(|id| !taken(id)).expect("an unbounded serial finds a free id")
}
//#endregion 🛠️TransformTool

//#region 🧪️Tests
#[cfg(test)]
#[path = "../../../../../../🧪️tests/🧪️transform-tool/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
