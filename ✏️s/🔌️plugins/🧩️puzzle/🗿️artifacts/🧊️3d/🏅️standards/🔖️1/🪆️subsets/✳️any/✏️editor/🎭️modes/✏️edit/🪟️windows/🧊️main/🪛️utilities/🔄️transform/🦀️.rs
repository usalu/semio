//! 🔄️ Main-window utility — Transform: the world gumball. Its Utility Options are the Move/Rotate
//! flags that compose which handles the gumball draws (scale handles are deliberately absent — a
//! puzzle-3d object's scale comes from its kind catalog, not from a free drag).
//!
//! 🛠️ It is also the transform TOOL: a `🔄️machine` statechart whose effects are `ToolYield`s, driven by the
//! `🛠️tool-machine` runner. Every selection transform — a gumball translate/rotate/scale, a target-volume
//! gumball relocate, a Relocate-utility drop, an inspector origin `delta` — enters as a
//! [`Puzzle3dSelectionRecord`] and leaves as ONE `ToolTransaction`: the parametric `drag-`/`rotate-`/
//! `scale-selection` leaf plus the `connect-vortices` a drop attracts, targets and attraction ids literal.
//! Tool state is never history; the yielded mutations are (design
//! `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/📋️design.md` §5, §8). Both hosts
//! paint the gesture locally and dispatch it ONCE on release, so every gesture is a one-shot transaction.

use semio_framework_pack_json::json;
use crate::editor::puzzle3d::config::Puzzle3dRuntime;
use crate::editor::puzzle3d::terminology::Puzzle3dLabels;
use crate::editor::puzzle3d::{derive_attraction_params, puzzle3d_action, puzzle3d_vortex_full_id, PUZZLE3D_PLAY_CONTROLLER_ID};
use crate::standards::v1::subsets::any::schema::mutations::{apply_puzzle3d_mutation,connect_vortices,drag_selection,rotate_selection,scale_selection,Puzzle3dMutation};

use crate::Puzzle3dSnapshot;

use semio_framework_pack_json::Value;
use machine::Command;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::UtilityDefinition;
use semio_framework_plugin::WindowMeasure;
use semio_framework_tool_machine::{GestureChart, GesturePhase, ToolYield};
use std::sync::Arc;

pub const UTILITY_ID: &str = "transform";

/// 🪪️ The editor app id every transform-tool transaction's `tool` is scoped by: `<appId>#<verb>`.
pub const PUZZLE3D_EDITOR_APP_ID: &str = "s.puzzle.puzzle3d@1/*#editor";

/// 🧱️ Stitched into the app manifest by `crate::editor::puzzle3d::create_puzzle3d_app`.
pub fn definition() -> UtilityDefinition {
    UtilityDefinition::new(UTILITY_ID, LocalizedLabel::native("Transform", "Transformieren"), "transform-3d")
}

/// 🎛️ Utility Options for the Transform utility — Move and Rotate flags. Tagged with this utility's
/// id as a routing envelope only; `partition_window_measures` unwraps the children so they render
/// flat under the Transform toggle (the toggle already owns that row, hence the empty group label).
pub fn options(runtime: &Puzzle3dRuntime, labels: &Puzzle3dLabels) -> WindowMeasure {
    WindowMeasure::Group {
        id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-utility-options-transform"),
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
                id: "puzzle3d-transform-move".into(),
                icon_id: "move-3d".into(),
                label: Some(labels.move_flag.into()),
                pressed: runtime.transform_move,
                text: None,
                on_change: puzzle3d_action("setTransformGumballFlag", Some(json!({ "flag": "move" }))),
            },
            WindowMeasure::Toggle {
                id: "puzzle3d-transform-rotate".into(),
                icon_id: "rotate-cw".into(),
                label: Some(labels.rotate_flag.into()),
                pressed: runtime.transform_rotate,
                text: None,
                on_change: puzzle3d_action("setTransformGumballFlag", Some(json!({ "flag": "rotate" }))),
            },
        ],
    }
}

//#region 🎬️Record
/// 🎬️ How one selection transform moves its targets — the parameters of the leaf it yields.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Puzzle3dSelectionMotion {
    Drag { offset: [f64; 3] },
    Rotate { axis: [f64; 3], angle: f64 },
    Scale { factors: [f64; 3] },
}

impl Puzzle3dSelectionMotion {
    /// 🎚️ Whether a leaf admits the motion: finite numbers and positive factors.
    pub fn admissible(&self) -> bool {
        match *self {
            Self::Drag { offset } => offset.iter().all(|value| value.is_finite()),
            Self::Rotate { axis, angle } => axis.iter().all(|value| value.is_finite()) && angle.is_finite(),
            Self::Scale { factors } => factors.iter().all(|value| value.is_finite() && *value > 0.0),
        }
    }

    /// 🏃️ Whether the motion is admissible and moves: a non-zero offset, a real turn about a real axis,
    /// factors other than one.
    pub fn moves(&self) -> bool {
        self.admissible()
            && match *self {
                Self::Drag { offset } => offset != [0.0; 3],
                Self::Rotate { axis, angle } => angle != 0.0 && (axis[0] * axis[0] + axis[1] * axis[1] + axis[2] * axis[2]).sqrt() >= 1e-8,
                Self::Scale { factors } => factors != [1.0; 3],
            }
    }
}

/// 🎬️ One selection transform the transform tool yields: the literal target ids, the motion, and the
/// `(attracting, attracted)` full vortex ids its drop attracts once the targets moved.
#[derive(Clone, Debug, PartialEq)]
pub struct Puzzle3dSelectionRecord {
    pub targets: Vec<String>,
    pub motion: Puzzle3dSelectionMotion,
    pub attractions: Vec<(String, String)>,
}

impl Puzzle3dSelectionRecord {
    /// 🎯️ A transform of `targets` (deduplicated in first-seen order) that attracts nothing.
    pub fn new(targets: impl IntoIterator<Item = String>, motion: Puzzle3dSelectionMotion) -> Self {
        Self { targets: puzzle3d_unique_targets(targets), motion, attractions: Vec::new() }
    }

    /// 🧩️ The record one gumball pose-delta verb states: `translateSelection{dx, dy, dz}`,
    /// `rotateSelection{ax, ay, az, angle}` or `scaleSelection{sx, sy, sz}` over `targets`; `None` for
    /// another verb or a non-finite number.
    pub fn from_gumball(verb: &str, args: Option<&Value>, targets: impl IntoIterator<Item = String>) -> Option<Self> {
        let number = |key: &str, fallback: f64| args.and_then(|args| args.get(key)).and_then(Value::as_f64).map_or(Some(fallback), |value| value.is_finite().then_some(value));
        let motion = match verb {
            "translateSelection" => Puzzle3dSelectionMotion::Drag { offset: [number("dx", 0.0)?, number("dy", 0.0)?, number("dz", 0.0)?] },
            "rotateSelection" => Puzzle3dSelectionMotion::Rotate { axis: [number("ax", 0.0)?, number("ay", 0.0)?, number("az", 0.0)?], angle: number("angle", 0.0)? },
            "scaleSelection" => Puzzle3dSelectionMotion::Scale { factors: [number("sx", 1.0)?, number("sy", 1.0)?, number("sz", 1.0)?] },
            _ => return None,
        };
        Some(Self::new(targets, motion))
    }

    /// 🧊️ The record one target-volume gumball relocate states — `{volumeId, mode, before, after}` with
    /// `before`/`after` the gumball poses `{position, quaternion, scale}` — as the RELATIVE motion from
    /// `before` to `after`: the offset, the turn `after · before⁻¹`, or the per-axis factors.
    pub fn from_pose_delta(args: Option<&Value>) -> Option<Self> {
        let args = args?;
        let volume = args.get("volumeId").and_then(Value::as_str).filter(|id| !id.is_empty())?.to_string();
        let triple = |pose: &str, key: &str| -> Option<[f64; 3]> {
            let values = args.get(pose)?.get(key)?.as_array()?;
            Some([values.first()?.as_f64()?, values.get(1)?.as_f64()?, values.get(2)?.as_f64()?])
        };
        let quaternion = |pose: &str| -> Option<[f64; 4]> {
            let values = args.get(pose)?.get("quaternion")?.as_array()?;
            Some([values.first()?.as_f64()?, values.get(1)?.as_f64()?, values.get(2)?.as_f64()?, values.get(3)?.as_f64()?])
        };
        let motion = match args.get("mode").and_then(Value::as_str)? {
            "translate" => {
                let (before, after) = (triple("before", "position")?, triple("after", "position")?);
                Puzzle3dSelectionMotion::Drag { offset: [after[0] - before[0], after[1] - before[1], after[2] - before[2]] }
            }
            "rotate" => {
                let (before, after) = (quaternion("before")?, quaternion("after")?);
                let turn = crate::standards::v1::subsets::any::schema::mutations::quat_mul(after, [-before[0], -before[1], -before[2], before[3]]);
                let sine = (1.0 - turn[3] * turn[3]).max(0.0).sqrt();
                let axis = if sine < 1e-9 { [0.0, 0.0, 1.0] } else { [turn[0] / sine, turn[1] / sine, turn[2] / sine] };
                Puzzle3dSelectionMotion::Rotate { axis, angle: 2.0 * turn[3].clamp(-1.0, 1.0).acos() }
            }
            "scale" => {
                let (before, after) = (triple("before", "scale")?, triple("after", "scale")?);
                Puzzle3dSelectionMotion::Scale { factors: [after[0] / before[0], after[1] / before[1], after[2] / before[2]] }
            }
            _ => return None,
        };
        Some(Self::new([volume], motion)).filter(|record| record.motion.admissible())
    }

    /// 🧮️ The parametric leaf this record yields, over its targets deduplicated in first-seen order.
    pub fn mutation(&self) -> Puzzle3dMutation {
        let targets = puzzle3d_unique_targets(self.targets.iter().cloned());
        match self.motion {
            Puzzle3dSelectionMotion::Drag { offset } => drag_selection(targets, offset),
            Puzzle3dSelectionMotion::Rotate { axis, angle } => rotate_selection(targets, axis, angle),
            Puzzle3dSelectionMotion::Scale { factors } => scale_selection(targets, factors),
        }
    }

    /// 🔎️ Whether this record moves anything on `base`: a moving motion and at least one target that is an
    /// unlocked object or target volume.
    pub fn applies_to(&self, base: &Puzzle3dSnapshot) -> bool {
        self.motion.moves() && self.targets.iter().any(|id| base.objects.iter().any(|object| &object.id == id && !object.locked) || base.target_volumes.iter().any(|volume| &volume.id == id && !volume.locked))
    }

    /// 🔒️ The tool-level refusal: the motion moves, yet nothing this record names can, and a lock is why.
    pub fn refused_as_locked(&self, base: &Puzzle3dSnapshot) -> bool {
        self.motion.moves() && !self.applies_to(base) && self.targets.iter().any(|id| base.objects.iter().any(|object| &object.id == id && object.locked) || base.target_volumes.iter().any(|volume| &volume.id == id && volume.locked))
    }
}

/// 🚚️ The record one Relocate-utility drop states: `object_id` dragged from its BASE origin to `position`, and
/// every vortex its first vortex lands within `radius` of — on another object, not yet attracted to it — as an
/// `(attracting, attracted)` pair, the stationary vortex attracting the moved one. `None` for an unknown object.
/// The whole scan in one call; the retained work pages it through [`Puzzle3dRelocateScan`].
pub fn puzzle3d_relocate_record(base: &Puzzle3dSnapshot, object_id: &str, position: [f64; 3], radius: f64) -> Option<Puzzle3dSelectionRecord> {
    let mut scan = Puzzle3dRelocateScan::begin(base, object_id, position, radius)?;
    while !scan.step(base, usize::MAX) {}
    Some(scan.finish())
}

/// 📄️ Objects one page of a relocate proximity scan measures — the retained work's per-step share.
pub const PUZZLE3D_RELOCATE_SCAN_PAGE: usize = 16;

/// 🔭️ The paged proximity scan of one Relocate-utility drop: [`Self::begin`] states the drag and indexes the vortices
/// already attracted to the moved one, every [`Self::step`] measures the vortices of one page of objects, and
/// [`Self::finish`] hands over the record — so a drop on a large scene reports progress page by page and stays
/// cancellable between pages, with exactly the pairs (and order) the one-call scan finds.
#[derive(Clone, Debug, PartialEq)]
pub struct Puzzle3dRelocateScan {
    record: Puzzle3dSelectionRecord,
    source: Option<(String, [f64; 3])>,
    attracted: std::collections::HashSet<String>,
    radius: f64,
    cursor: usize,
}

impl Puzzle3dRelocateScan {
    /// 🎬️ The scan of `object_id` dropped at `position`; `None` for an unknown object. An object without a vortex
    /// attracts nothing, so its scan is already done.
    pub fn begin(base: &Puzzle3dSnapshot, object_id: &str, position: [f64; 3], radius: f64) -> Option<Self> {
        let object = base.objects.iter().find(|object| object.id == object_id)?;
        let offset = [position[0] - object.origin[0], position[1] - object.origin[1], position[2] - object.origin[2]];
        let source = object.vortices.first().map(|vortex| (puzzle3d_vortex_full_id(&object.id, &vortex.id), puzzle3d_world_point(position, object.orientation, vortex.position)));
        let attracted = source.as_ref().map_or_else(Default::default, |(source, _)| {
            base.attractions.iter().filter_map(|entry| if &entry.attracting == source { Some(entry.attracted.clone()) } else if &entry.attracted == source { Some(entry.attracting.clone()) } else { None }).collect()
        });
        let cursor = if source.is_some() { 0 } else { base.objects.len() };
        Some(Self { record: Puzzle3dSelectionRecord { targets: vec![object.id.clone()], motion: Puzzle3dSelectionMotion::Drag { offset }, attractions: Vec::new() }, source, attracted, radius, cursor })
    }

    /// 📏️ Measures the vortices of the next `page` objects (at least one); `true` once every object is measured.
    pub fn step(&mut self, base: &Puzzle3dSnapshot, page: usize) -> bool {
        let end = self.cursor.saturating_add(page.max(1)).min(base.objects.len());
        if let Some((source, from)) = self.source.as_ref() {
            for other in base.objects.get(self.cursor..end).into_iter().flatten().filter(|other| other.id != self.record.targets[0]) {
                for candidate in &other.vortices {
                    let target = puzzle3d_vortex_full_id(&other.id, &candidate.id);
                    let at = puzzle3d_world_point(other.origin, other.orientation, candidate.position);
                    if target != *source && !self.attracted.contains(&target) && ((from[0] - at[0]).powi(2) + (from[1] - at[1]).powi(2) + (from[2] - at[2]).powi(2)).sqrt() <= self.radius {
                        self.record.attractions.push((target, source.clone()));
                    }
                }
            }
        }
        self.cursor = end;
        self.cursor >= base.objects.len()
    }

    /// 📊️ `(measured objects, all objects)` of `base`.
    pub fn progress(&self, base: &Puzzle3dSnapshot) -> (usize, usize) {
        (self.cursor.min(base.objects.len()), base.objects.len())
    }

    /// 🏁️ The scanned drop record.
    pub fn finish(self) -> Puzzle3dSelectionRecord {
        self.record
    }
}

/// 🌐️ A vortex's local `position` placed in the world by its object's `origin` and `orientation`.
fn puzzle3d_world_point(origin: [f64; 3], orientation: Option<[f64; 4]>, local: [f64; 3]) -> [f64; 3] {
    let turned = crate::editor::puzzle3d::quat_rotate_vector(orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]), local);
    [origin[0] + turned[0], origin[1] + turned[1], origin[2] + turned[2]]
}

/// 🧹️ `targets` without repeats, in first-seen order — the one target list every leaf the tool yields carries.
pub fn puzzle3d_unique_targets(targets: impl IntoIterator<Item = String>) -> Vec<String> {
    let mut seen = std::collections::HashSet::new();
    targets.into_iter().filter(|id| seen.insert(id.clone())).collect()
}
//#endregion 🎬️Record

//#region 🛠️TransformTool
/// 📨️ What one transform-tool event carries: the committed document it yields against and the records —
/// dispatch inputs, never tool state.
#[derive(Clone, Debug)]
pub struct TransformToolRequest {
    pub base: Arc<Puzzle3dSnapshot>,
    pub records: Vec<Puzzle3dSelectionRecord>,
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
    sink.extend(puzzle3d_selection_yields(&request.base, &request.records).into_iter().map(|(key, mutation)| Command::Effect(ToolYield::upsert(key, mutation))));
    sink.push(Command::Effect(ToolYield::Commit));
}

machine::statechart! {
    machine transform_tool {
        context: TransformToolContext;
        event Event { Records(TransformToolRequest) }
        input: TransformToolContext;
        output: ();
        effect: ToolYield<Puzzle3dMutation>;
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
    fn execute_effect(&mut self, _actor: machine::ActorId, _effect: ToolYield<Puzzle3dMutation>) {}
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
        format!("{PUZZLE3D_EDITOR_APP_ID}#{verb}")
    }

    fn host() -> TransformToolHost {
        TransformToolHost
    }

    fn input() -> TransformToolContext {
        TransformToolContext
    }

    fn restore(_entries: &[(String, Puzzle3dMutation)], _context: &semio_framework_value::DslValue) -> Option<TransformToolContext> {
        None
    }

    fn event(phase: GesturePhase, _at_rest: bool, tick: Option<TransformToolRequest>) -> Option<transform_tool::Event> {
        tick.filter(|_| phase == GesturePhase::Once).map(transform_tool::Event::Records)
    }
}

/// 🛠️ One release through the shared gesture driver: its transaction and yielded mutations, or zero trace
/// for an empty, locked, missing or motionless request. The framework owns the clock and transaction identity.
pub fn puzzle3d_transform_tool_commit(verb: &str, authoring_seed: &str, request: TransformToolRequest) -> Option<(protocol::TransactionRef, Vec<Puzzle3dMutation>)> {
    semio_framework_tool_machine::drive_chart_gesture::<transform_tool::TransformTool>(None, verb, GesturePhase::Once, Some(request), authoring_seed, "").ok()?.committed
}

/// 🧮️ What the transform tool yields for `records` on `base`, keyed: each moving record's parametric leaf,
/// then the `connect-vortices` its drop attracts — each pair still free on the moved state, its six
/// parameters derived from the moved poses so resolving never jumps an endpoint, its id minted HERE,
/// deterministically from the pair and the document, so a replay never mints again.
pub fn puzzle3d_selection_yields(base: &Puzzle3dSnapshot, records: &[Puzzle3dSelectionRecord]) -> Vec<(String, Puzzle3dMutation)> {
    let attracts = records.iter().any(|record| !record.attractions.is_empty());
    let mut state = attracts.then(|| base.clone());
    let mut yields = Vec::new();
    for (index, record) in records.iter().enumerate() {
        let current = state.as_ref().unwrap_or(base);
        if !record.applies_to(current) {
            continue;
        }
        let leaf = record.mutation();
        if let Some(state) = state.as_mut() {
            if apply_puzzle3d_mutation(state, &leaf).is_err() {
                continue;
            }
        }
        yields.push((format!("selection:{index}"), leaf));
        let Some(state) = state.as_mut() else { continue };
        for (attracting, attracted) in &record.attractions {
            let connected = state.attractions.iter().any(|entry| (&entry.attracting, &entry.attracted) == (attracting, attracted) || (&entry.attracting, &entry.attracted) == (attracted, attracting));
            let (Some(parent), Some(child)) = (puzzle3d_vortex_pose(state, attracting), puzzle3d_vortex_pose(state, attracted)) else { continue };
            if connected || attracting == attracted || parent.object_id == child.object_id {
                continue;
            }
            let (gap, shift, rise, rotation, turn, tilt) = derive_attraction_params(parent.origin, parent.orientation, parent.position, parent.direction, child.position, child.direction, child.origin, child.orientation);
            let id = puzzle3d_minted_attraction_id(state, attracting, attracted);
            let connect = connect_vortices(id.clone(), attracting.clone(), attracted.clone(), gap, shift, rise, rotation, turn, tilt, 0.0, 0.0, None);
            if apply_puzzle3d_mutation(state, &connect).is_ok() {
                yields.push((format!("attraction:{id}"), connect));
            }
        }
    }
    yields
}

/// 📌️ One vortex's owning object pose and local port geometry, read off the typed document.
struct Puzzle3dVortexPose {
    object_id: String,
    origin: [f64; 3],
    orientation: [f64; 4],
    position: [f64; 3],
    direction: [f64; 3],
}

fn puzzle3d_vortex_pose(document: &Puzzle3dSnapshot, full_id: &str) -> Option<Puzzle3dVortexPose> {
    document.objects.iter().find_map(|object| {
        object.vortices.iter().find(|vortex| puzzle3d_vortex_full_id(&object.id, &vortex.id) == full_id).map(|vortex| Puzzle3dVortexPose {
            object_id: object.id.clone(),
            origin: object.origin,
            orientation: object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]),
            position: vortex.position,
            direction: vortex.direction.unwrap_or([0.0, 0.0, -1.0]),
        })
    })
}

/// 🆔️ The attraction id a yielded connection carries: `attraction-<attracting>-<attracted>`, suffixed `-2`,
/// `-3`, … past any id the document already holds — a pure function of the pair and the document, never a
/// process counter.
pub fn puzzle3d_minted_attraction_id(document: &Puzzle3dSnapshot, attracting: &str, attracted: &str) -> String {
    let candidate = format!("attraction-{attracting}-{attracted}");
    let taken = |id: &str| document.attractions.iter().any(|attraction| attraction.id == id);
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
