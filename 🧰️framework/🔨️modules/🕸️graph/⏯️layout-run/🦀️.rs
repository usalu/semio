//! ⏯️ Domain-neutral, stepped, cancellable force-directed layout run: the shared `runJob` every graph plugin's
//! `reorganize`/`forceLayout` tool drives through the ToolRun ledger.
//!
//! - One iteration per fuel unit; every iteration is sliced into resumable phases (tree, repulsion, springs,
//!   integration) that check the step deadline, so one step stays bounded for large graphs.
//! - Barnes-Hut repulsion above `pairwiseMaxBodies`, exact pairwise at or below it.
//! - Per-node trace records (`Entity` subjects) with displacement verdicts; counters iterations, energy, largest
//!   displacement and settled nodes; stages initialize, iterate, settle.
//! - Provisional move ops come from a caller encoder; the provisional list is compacted to one op per moved node at
//!   every checkpoint and at settle, so finalize publishes exactly the moved nodes.
//!
//! Schema of record: `🧬️schema/🔣️.json`. Contract: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️13/INTERACTIVE-TOOLS-VISIBLE-PROCESS/📋️tool-run-contract.md` §3.7, §5 W3-F.

use dsl::{FromValue, ToValue};
use semio_framework_job::{Checkpoint, CommitCandidate, InteractiveJob, InteractiveJobCloseStep, JobFault, JobPayloadStream, RetainedJobPayload, StepContext, StepOutcome, JOB_PAYLOAD_PAGE_BYTES};
use semio_framework_tool_run::{
    JobKindId, ToolRunCounter, ToolRunCounterDefinition, ToolRunDefinition, ToolRunIdentity, ToolRunProgress, ToolRunReasonDefinition, ToolRunRebasePolicy, ToolRunReconfigurePolicy, ToolRunSettingsReads, ToolRunStageDefinition, ToolRunState, ToolRunStepArg, ToolRunStepKind, ToolRunStepRing, ToolRunTickWriter,
    ToolRunTraceKind, ToolRunTraceSubject, ToolRunVerdict, TOOL_RUN_PROVISIONAL_OPS_MAX,
};
use serde::{Deserialize, Serialize};
use ui::wgpu::LocalizedLabel;

//#region 🔖️Limits
/// 🚰️ Estimated tick bytes after which an iteration or compaction tick is flushed (one 16 KiB job payload page).
pub const LAYOUT_RUN_TICK_FLUSH_BYTES: usize = 12 * 1024;
/// 🧱️ Largest move op one encoder call may return.
pub const LAYOUT_RUN_OP_BYTES_MAX: usize = 2 * 1024;
/// 🔢️ Largest graph one run lays out: every node must fit one op in half of the provisional op cap.
pub const LAYOUT_RUN_NODES_MAX: usize = TOOL_RUN_PROVISIONAL_OPS_MAX as usize / 2;
/// ⏱️ Repulsion queries between two deadline checks.
pub const LAYOUT_RUN_REPULSION_CHUNK: usize = 4;
/// ⏱️ Linear body or edge updates between two deadline checks.
pub const LAYOUT_RUN_LINEAR_CHUNK: usize = 128;
/// 🌳️ Quadtree depth after which coincident bodies share one leaf.
pub const LAYOUT_RUN_TREE_DEPTH_MAX: u32 = 32;
/// 🪪️ Checkpoint magic, `LRC1` read as a little-endian `u32`.
pub const LAYOUT_RUN_CHECKPOINT_MAGIC: u32 = 0x4C52_4331;
const NONE: u32 = u32::MAX;
const MIN_DISTANCE: f64 = 1e-4;
//#endregion 🔖️Limits

//#region 🔖️Vocabulary
/// 🪜️ Run stages, indices into the `ToolRunDefinition` stages.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LayoutRunStage {
    Initialize,
    Iterate,
    Settle,
}

impl LayoutRunStage {
    pub const ALL: [Self; 3] = [Self::Initialize, Self::Iterate, Self::Settle];

    pub fn index(self) -> u16 {
        self as u16
    }

    pub fn id(self) -> &'static str {
        match self {
            Self::Initialize => "initialize",
            Self::Iterate => "iterate",
            Self::Settle => "settle",
        }
    }

    pub fn label(self) -> LocalizedLabel {
        match self {
            Self::Initialize => LocalizedLabel::native("Initialize", "Initialisieren"),
            Self::Iterate => LocalizedLabel::native("Iterate", "Iterieren"),
            Self::Settle => LocalizedLabel::native("Settle", "Stabilisieren"),
        }
    }
}

/// 🧮️ Run counters; `Energy` and `MaxDisplacement` are fixed point in thousandths.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LayoutRunCounter {
    Iterations,
    Energy,
    MaxDisplacement,
    Settled,
}

impl LayoutRunCounter {
    pub const ALL: [Self; 4] = [Self::Iterations, Self::Energy, Self::MaxDisplacement, Self::Settled];

    pub fn index(self) -> u16 {
        self as u16
    }

    pub fn id(self) -> &'static str {
        match self {
            Self::Iterations => "iterations",
            Self::Energy => "energy",
            Self::MaxDisplacement => "maxDisplacement",
            Self::Settled => "settled",
        }
    }

    pub fn fixed_point(self) -> u64 {
        match self {
            Self::Energy | Self::MaxDisplacement => 1000,
            Self::Iterations | Self::Settled => 1,
        }
    }

    pub fn label(self) -> LocalizedLabel {
        match self {
            Self::Iterations => LocalizedLabel::native("Iterations", "Iterationen"),
            Self::Energy => LocalizedLabel::native("Kinetic energy (thousandths)", "Kinetische Energie (Tausendstel)"),
            Self::MaxDisplacement => LocalizedLabel::native("Largest displacement (thousandths)", "Größte Verschiebung (Tausendstel)"),
            Self::Settled => LocalizedLabel::native("Settled nodes", "Ruhende Knoten"),
        }
    }
}

/// 🗯️ Trace and step reasons; `code()` is the ordinal.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LayoutRunReason {
    Moving,
    Settled,
    Pinned,
    Diverged,
    Unsettled,
    Initialized,
    Converged,
    IterationLimit,
    Resumed,
}

impl LayoutRunReason {
    pub const ALL: [Self; 9] = [Self::Moving, Self::Settled, Self::Pinned, Self::Diverged, Self::Unsettled, Self::Initialized, Self::Converged, Self::IterationLimit, Self::Resumed];

    pub fn code(self) -> u16 {
        self as u16
    }

    pub fn from_code(code: u16) -> Option<Self> {
        Self::ALL.get(usize::from(code)).copied()
    }

    pub fn id(self) -> &'static str {
        match self {
            Self::Moving => "moving",
            Self::Settled => "settled",
            Self::Pinned => "pinned",
            Self::Diverged => "diverged",
            Self::Unsettled => "unsettled",
            Self::Initialized => "initialized",
            Self::Converged => "converged",
            Self::IterationLimit => "iterationLimit",
            Self::Resumed => "resumed",
        }
    }

    pub fn verdict(self) -> ToolRunVerdict {
        match self {
            Self::Moving | Self::Initialized | Self::Resumed => ToolRunVerdict::Testing,
            Self::Settled | Self::Pinned | Self::Converged => ToolRunVerdict::Success,
            Self::Unsettled | Self::IterationLimit => ToolRunVerdict::Warning,
            Self::Diverged => ToolRunVerdict::Danger,
        }
    }

    pub fn template(self) -> LocalizedLabel {
        match self {
            Self::Moving => LocalizedLabel::native("Moving", "In Bewegung"),
            Self::Settled => LocalizedLabel::native("Settled", "Zur Ruhe gekommen"),
            Self::Pinned => LocalizedLabel::native("Pinned", "Fixiert"),
            Self::Diverged => LocalizedLabel::native("{0} nodes diverged and were reset to their last finite position", "{0} Knoten divergierten und wurden auf ihre letzte endliche Position zurückgesetzt"),
            Self::Unsettled => LocalizedLabel::native("Still moving when the iteration limit was reached", "Beim Erreichen der Iterationsgrenze noch in Bewegung"),
            Self::Initialized => LocalizedLabel::native("{0} nodes and {1} edges prepared", "{0} Knoten und {1} Kanten vorbereitet"),
            Self::Converged => LocalizedLabel::native("Converged after {0} iterations (largest displacement {1}, energy {2})", "Nach {0} Iterationen konvergiert (größte Verschiebung {1}, Energie {2})"),
            Self::IterationLimit => LocalizedLabel::native("Iteration limit {0} reached (largest displacement {1}, energy {2})", "Iterationsgrenze {0} erreicht (größte Verschiebung {1}, Energie {2})"),
            Self::Resumed => LocalizedLabel::native("Resumed at iteration {0}", "Bei Iteration {0} fortgesetzt"),
        }
    }
}

/// 🏁️ Why a run settled.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum LayoutRunStop {
    Converged,
    IterationLimit,
}

/// 📜️ The `ToolRunDefinition` every consumer declares for its layout tool: mutating, `rebase: restart` (a changed
/// node set invalidates the graph), `reconfigure: resume` (settings continue from the last checkpoint), entity trace.
pub fn layout_run_definition(run_job: JobKindId) -> ToolRunDefinition {
    ToolRunDefinition {
        mutating: true,
        rebase: ToolRunRebasePolicy::Restart,
        reconfigure: ToolRunReconfigurePolicy::Resume,
        unit: LocalizedLabel::native("iteration", "Iteration"),
        stages: LayoutRunStage::ALL.iter().map(|stage| ToolRunStageDefinition { id: stage.id().to_string(), label: stage.label() }).collect(),
        counters: LayoutRunCounter::ALL.iter().map(|counter| ToolRunCounterDefinition { id: counter.id().to_string(), label: counter.label() }).collect(),
        reasons: LayoutRunReason::ALL.iter().map(|reason| ToolRunReasonDefinition { code: reason.code(), id: reason.id().to_string(), verdict: reason.verdict(), template: reason.template() }).collect(),
        trace: ToolRunTraceKind::Entity,
        run_job,
        revalidate_job: None,
        settings: ToolRunSettingsReads::default(),
        windows: Vec::new(),
    }
}
//#endregion 🔖️Vocabulary

//#region 🔖️Graph
/// 📍️ A world position.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(deny_unknown_fields)]
#[value(deny_unknown_fields)]
pub struct LayoutRunPoint {
    pub x: f64,
    pub y: f64,
}

impl LayoutRunPoint {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

/// 🔵️ One body: `entity` keys the provisional entity set and the trace subject, `origin` is the committed position
/// (`None` = unplaced, seeded), a pinned node never moves, `anchor` adds a spring toward a target position.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct LayoutRunNode {
    pub entity: u64,
    pub origin: Option<LayoutRunPoint>,
    pub radius: f64,
    pub pinned: bool,
    pub anchor: Option<LayoutRunPoint>,
}

/// 🔗️ One undirected spring between node indices.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct LayoutRunEdge {
    pub source: u32,
    pub target: u32,
    pub weight: f64,
}

/// 🕸️ The graph a run lays out.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct LayoutRunGraph {
    pub nodes: Vec<LayoutRunNode>,
    pub edges: Vec<LayoutRunEdge>,
}

/// 🚫️ Why a graph cannot be laid out.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LayoutRunGraphError {
    TooManyNodes,
    NonFiniteOrigin { node: u32 },
    NonFiniteAnchor { node: u32 },
    NonPositiveRadius { node: u32 },
    PinnedWithoutOrigin { node: u32 },
    EdgeOutOfRange { edge: u32 },
    InvalidWeight { edge: u32 },
}

impl LayoutRunGraph {
    /// ✅️ Checks the graph against the run's invariants.
    pub fn validate(&self) -> Result<(), LayoutRunGraphError> {
        if self.nodes.len() > LAYOUT_RUN_NODES_MAX {
            return Err(LayoutRunGraphError::TooManyNodes);
        }
        let finite = |point: Option<LayoutRunPoint>| point.is_none_or(|point| point.x.is_finite() && point.y.is_finite());
        for (index, node) in self.nodes.iter().enumerate() {
            let node_index = index as u32;
            if !finite(node.origin) {
                return Err(LayoutRunGraphError::NonFiniteOrigin { node: node_index });
            }
            if !finite(node.anchor) {
                return Err(LayoutRunGraphError::NonFiniteAnchor { node: node_index });
            }
            if !(node.radius.is_finite() && node.radius > 0.0) {
                return Err(LayoutRunGraphError::NonPositiveRadius { node: node_index });
            }
            if node.pinned && node.origin.is_none() {
                return Err(LayoutRunGraphError::PinnedWithoutOrigin { node: node_index });
            }
        }
        for (index, edge) in self.edges.iter().enumerate() {
            if edge.source as usize >= self.nodes.len() || edge.target as usize >= self.nodes.len() {
                return Err(LayoutRunGraphError::EdgeOutOfRange { edge: index as u32 });
            }
            if !(edge.weight.is_finite() && edge.weight >= 0.0) {
                return Err(LayoutRunGraphError::InvalidWeight { edge: index as u32 });
            }
        }
        Ok(())
    }

    /// 🔏️ FNV-1a 64 over the graph layout per the schema's `graphDigest` rule; a checkpoint only resumes the same graph.
    pub fn digest(&self) -> u64 {
        let mut hash = Fnv::new();
        hash.u32(self.nodes.len() as u32);
        for node in &self.nodes {
            hash.u64(node.entity);
            hash.u8(u8::from(node.pinned));
            hash.point(node.origin);
            hash.f64(node.radius);
            hash.point(node.anchor);
        }
        hash.u32(self.edges.len() as u32);
        for edge in &self.edges {
            hash.u32(edge.source);
            hash.u32(edge.target);
            hash.f64(edge.weight);
        }
        hash.finish()
    }
}

struct Fnv(u64);

impl Fnv {
    fn new() -> Self {
        Self(0xcbf2_9ce4_8422_2325)
    }

    fn bytes(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.0 = (self.0 ^ u64::from(*byte)).wrapping_mul(0x0000_0100_0000_01b3);
        }
    }

    fn u8(&mut self, value: u8) {
        self.bytes(&[value]);
    }

    fn u32(&mut self, value: u32) {
        self.bytes(&value.to_le_bytes());
    }

    fn u64(&mut self, value: u64) {
        self.bytes(&value.to_le_bytes());
    }

    fn f64(&mut self, value: f64) {
        self.bytes(&value.to_bits().to_le_bytes());
    }

    fn point(&mut self, point: Option<LayoutRunPoint>) {
        match point {
            Some(point) => {
                self.u8(1);
                self.f64(point.x);
                self.f64(point.y);
            }
            None => self.u8(0),
        }
    }

    fn finish(&self) -> u64 {
        self.0
    }
}

/// 🔏️ FNV-1a 64 over positions per the schema's `positionsDigest` rule.
pub fn layout_run_positions_digest(positions: &[[f64; 2]]) -> u64 {
    let mut hash = Fnv::new();
    for position in positions {
        hash.f64(position[0]);
        hash.f64(position[1]);
    }
    hash.finish()
}
//#endregion 🔖️Graph

//#region 🔖️Config
/// 📉️ How repulsion decays with distance: `inverseSquare` (`1/d²`, local) or `inverse` (`1/d`, long-range, untangles
/// rings and trees the way Fruchterman-Reingold does).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum LayoutRunFalloff {
    InverseSquare,
    Inverse,
}

impl LayoutRunFalloff {
    #[inline]
    fn divisor(self, distance: f64) -> f64 {
        match self {
            Self::InverseSquare => distance * distance,
            Self::Inverse => distance,
        }
    }
}

/// 🪢️ How a spring pulls: `linear` (Hooke, `d − idealEdgeLength`) or `quadratic` (Fruchterman-Reingold, `d²/idealEdgeLength`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum LayoutRunSpringLaw {
    Linear,
    Quadratic,
}

impl LayoutRunSpringLaw {
    #[inline]
    fn stretch(self, distance: f64, ideal: f64) -> f64 {
        match self {
            Self::Linear => distance - ideal,
            Self::Quadratic => distance * distance / ideal,
        }
    }

    #[inline]
    fn anchor_scale(self, distance: f64, ideal: f64) -> f64 {
        match self {
            Self::Linear => 1.0,
            Self::Quadratic => distance / ideal,
        }
    }
}

/// ⚙️ Every knob of a layout run; see the schema's `LayoutRunConfig` description for the physics.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct LayoutRunConfig {
    pub max_iterations: u32,
    pub ideal_edge_length: f64,
    pub repulsion_strength: f64,
    pub repulsion_falloff: LayoutRunFalloff,
    pub spring_strength: f64,
    pub spring_law: LayoutRunSpringLaw,
    pub gravity: f64,
    pub center: Option<LayoutRunPoint>,
    pub anchor_strength: f64,
    pub time_step: f64,
    pub velocity_damping: f64,
    pub max_speed: f64,
    pub cooling_floor: f64,
    pub seed: u64,
    pub barnes_hut_theta: f64,
    pub pairwise_max_bodies: u32,
    pub settle_displacement: f64,
    pub settle_iterations: u32,
    pub emit_displacement: f64,
    pub checkpoint_iterations: u32,
    pub compact_ops: u32,
}

impl Default for LayoutRunConfig {
    fn default() -> Self {
        Self {
            max_iterations: 420,
            ideal_edge_length: 140.0,
            repulsion_strength: 49.0,
            repulsion_falloff: LayoutRunFalloff::Inverse,
            spring_strength: 1.0,
            spring_law: LayoutRunSpringLaw::Quadratic,
            gravity: 0.0,
            center: None,
            anchor_strength: 1.0,
            time_step: 0.85,
            velocity_damping: 0.88,
            max_speed: 48.0,
            cooling_floor: 0.08,
            seed: 1,
            barnes_hut_theta: 0.78,
            pairwise_max_bodies: 56,
            settle_displacement: 0.5,
            settle_iterations: 8,
            emit_displacement: 0.25,
            checkpoint_iterations: 64,
            compact_ops: 16_384,
        }
    }
}

/// 🚫️ The first config field outside its schema range.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LayoutRunConfigError {
    pub field: &'static str,
}

impl LayoutRunConfig {
    /// ✅️ Checks every field against the schema ranges.
    pub fn validate(&self) -> Result<(), LayoutRunConfigError> {
        let positive = |value: f64| value.is_finite() && value > 0.0;
        let non_negative = |value: f64| value.is_finite() && value >= 0.0;
        let checks: [(&'static str, bool); 17] = [
            ("maxIterations", self.max_iterations >= 1),
            ("idealEdgeLength", positive(self.ideal_edge_length)),
            ("repulsionStrength", non_negative(self.repulsion_strength)),
            ("springStrength", non_negative(self.spring_strength)),
            ("gravity", non_negative(self.gravity)),
            ("center", self.center.is_none_or(|center| center.x.is_finite() && center.y.is_finite())),
            ("anchorStrength", non_negative(self.anchor_strength)),
            ("timeStep", positive(self.time_step)),
            ("velocityDamping", non_negative(self.velocity_damping) && self.velocity_damping <= 1.0),
            ("maxSpeed", positive(self.max_speed)),
            ("coolingFloor", positive(self.cooling_floor) && self.cooling_floor <= 1.0),
            ("barnesHutTheta", non_negative(self.barnes_hut_theta) && self.barnes_hut_theta <= 2.0),
            ("settleDisplacement", positive(self.settle_displacement)),
            ("settleIterations", self.settle_iterations >= 1),
            ("emitDisplacement", non_negative(self.emit_displacement)),
            ("checkpointIterations", self.checkpoint_iterations >= 1),
            ("compactOps", self.compact_ops >= 1 && self.compact_ops <= TOOL_RUN_PROVISIONAL_OPS_MAX),
        ];
        checks.iter().find(|(_, ok)| !ok).map_or(Ok(()), |(field, _)| Err(LayoutRunConfigError { field }))
    }
}
//#endregion 🔖️Config

//#region 🔖️Encoder
/// 💥️ An encoder refused to encode a move.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LayoutRunEncodeError(pub String);

/// ✍️ Encodes one provisional move of node `node` (entity `entity`) to `position` as `OpBinary` bytes of the consumer's
/// artifact mutation. Implemented for every matching `FnMut` closure.
pub trait LayoutRunOpEncoder: Send {
    fn encode_move(&mut self, node: u32, entity: u64, position: LayoutRunPoint) -> Result<Vec<u8>, LayoutRunEncodeError>;
}

impl<F: FnMut(u32, u64, LayoutRunPoint) -> Result<Vec<u8>, LayoutRunEncodeError> + Send> LayoutRunOpEncoder for F {
    fn encode_move(&mut self, node: u32, entity: u64, position: LayoutRunPoint) -> Result<Vec<u8>, LayoutRunEncodeError> {
        self(node, entity, position)
    }
}
//#endregion 🔖️Encoder

//#region 🔖️Checkpoint
/// 📸️ The 44-byte little-endian resume position a run reports after each compaction.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LayoutRunCheckpoint {
    pub node_count: u32,
    pub graph_digest: u64,
    pub iteration: u32,
    pub settle_streak: u32,
    pub provisional_len: u32,
    pub center: [f64; 2],
}

impl LayoutRunCheckpoint {
    pub const BYTES: usize = 44;

    pub fn encode(self) -> [u8; Self::BYTES] {
        let mut bytes = [0u8; Self::BYTES];
        bytes[0..4].copy_from_slice(&LAYOUT_RUN_CHECKPOINT_MAGIC.to_le_bytes());
        bytes[4..8].copy_from_slice(&self.node_count.to_le_bytes());
        bytes[8..16].copy_from_slice(&self.graph_digest.to_le_bytes());
        bytes[16..20].copy_from_slice(&self.iteration.to_le_bytes());
        bytes[20..24].copy_from_slice(&self.settle_streak.to_le_bytes());
        bytes[24..28].copy_from_slice(&self.provisional_len.to_le_bytes());
        bytes[28..36].copy_from_slice(&self.center[0].to_bits().to_le_bytes());
        bytes[36..44].copy_from_slice(&self.center[1].to_bits().to_le_bytes());
        bytes
    }

    pub fn decode(bytes: &[u8]) -> Option<Self> {
        let bytes: &[u8; Self::BYTES] = bytes.try_into().ok()?;
        let u32_at = |at: usize| u32::from_le_bytes(bytes[at..at + 4].try_into().expect("four bytes"));
        let u64_at = |at: usize| u64::from_le_bytes(bytes[at..at + 8].try_into().expect("eight bytes"));
        (u32_at(0) == LAYOUT_RUN_CHECKPOINT_MAGIC).then(|| Self {
            node_count: u32_at(4),
            graph_digest: u64_at(8),
            iteration: u32_at(16),
            settle_streak: u32_at(20),
            provisional_len: u32_at(24),
            center: [f64::from_bits(u64_at(28)), f64::from_bits(u64_at(36))],
        })
    }
}

/// 🚫️ Why a run cannot resume.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LayoutRunResumeError {
    Malformed,
    Foreign,
    Positions,
    Graph(LayoutRunGraphError),
    Config(LayoutRunConfigError),
}

/// 🚫️ Why a run cannot start.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LayoutRunStartError {
    Graph(LayoutRunGraphError),
    Config(LayoutRunConfigError),
}
//#endregion 🔖️Checkpoint

//#region 🔖️Quadtree
#[derive(Clone, Copy, Debug)]
struct LayoutRunCell {
    center: [f64; 2],
    half: f64,
    mass: f64,
    moment: [f64; 2],
    children: [u32; 4],
    body: u32,
    count: u32,
    depth: u32,
}

impl LayoutRunCell {
    fn empty(center: [f64; 2], half: f64, depth: u32) -> Self {
        Self { center, half, mass: 0.0, moment: [0.0, 0.0], children: [0; 4], body: NONE, count: 0, depth }
    }

    fn is_leaf(&self) -> bool {
        self.children == [0; 4]
    }

    fn quadrant(&self, point: [f64; 2]) -> usize {
        usize::from(point[0] >= self.center[0]) | (usize::from(point[1] >= self.center[1]) << 1)
    }
}

/// 🌳️ Arena Barnes-Hut quadtree; cleared and rebuilt every iteration with retained capacity.
#[derive(Debug, Default)]
struct LayoutRunTree {
    cells: Vec<LayoutRunCell>,
    next: Vec<u32>,
    stack: Vec<u32>,
}

impl LayoutRunTree {
    fn reset(&mut self, bounds: [f64; 4], bodies: usize) {
        self.cells.clear();
        self.next.clear();
        self.next.resize(bodies, NONE);
        let half = ((bounds[2] - bounds[0]).max(bounds[3] - bounds[1]) * 0.5).max(1.0) + 1.0;
        self.cells.push(LayoutRunCell::empty([(bounds[0] + bounds[2]) * 0.5, (bounds[1] + bounds[3]) * 0.5], half, 0));
    }

    fn child(&mut self, cell: usize, quadrant: usize) -> usize {
        let existing = self.cells[cell].children[quadrant];
        if existing != 0 {
            return existing as usize;
        }
        let parent = self.cells[cell];
        let offset = parent.half * 0.5;
        let center = [parent.center[0] + if quadrant & 1 == 1 { offset } else { -offset }, parent.center[1] + if quadrant & 2 == 2 { offset } else { -offset }];
        let index = self.cells.len();
        self.cells.push(LayoutRunCell::empty(center, offset, parent.depth + 1));
        self.cells[cell].children[quadrant] = index as u32;
        index
    }

    fn insert(&mut self, body: u32, positions: &[[f64; 2]], masses: &[f64]) {
        let point = positions[body as usize];
        let mass = masses[body as usize];
        let mut cell = 0usize;
        loop {
            let current = &mut self.cells[cell];
            current.mass += mass;
            current.moment[0] += mass * point[0];
            current.moment[1] += mass * point[1];
            current.count += 1;
            if current.is_leaf() {
                if current.count == 1 {
                    current.body = body;
                    self.next[body as usize] = NONE;
                    return;
                }
                if current.depth >= LAYOUT_RUN_TREE_DEPTH_MAX {
                    self.next[body as usize] = current.body;
                    current.body = body;
                    return;
                }
                let existing = current.body;
                current.body = NONE;
                let quadrant = current.quadrant(positions[existing as usize]);
                let child = self.child(cell, quadrant);
                let existing_mass = masses[existing as usize];
                let existing_point = positions[existing as usize];
                let target = &mut self.cells[child];
                target.mass = existing_mass;
                target.moment = [existing_mass * existing_point[0], existing_mass * existing_point[1]];
                target.count = 1;
                target.body = existing;
            }
            let quadrant = self.cells[cell].quadrant(point);
            cell = self.child(cell, quadrant);
        }
    }

    fn repulsion(&mut self, body: u32, positions: &[[f64; 2]], radii: &[f64], strength: f64, falloff: LayoutRunFalloff, theta_squared: f64) -> [f64; 2] {
        let point = positions[body as usize];
        let radius = radii[body as usize];
        let mut force = [0.0, 0.0];
        self.stack.clear();
        self.stack.push(0);
        while let Some(index) = self.stack.pop() {
            let cell = self.cells[index as usize];
            if cell.count == 0 {
                continue;
            }
            if cell.is_leaf() {
                let mut other = cell.body;
                while other != NONE {
                    if other != body {
                        let push = pairwise_repulsion(point, positions[other as usize], radius, radii[other as usize], strength, falloff);
                        force[0] += push[0];
                        force[1] += push[1];
                    }
                    other = self.next[other as usize];
                }
                continue;
            }
            let dx = cell.moment[0] / cell.mass - point[0];
            let dy = cell.moment[1] / cell.mass - point[1];
            let distance_squared = dx * dx + dy * dy;
            let inside = (point[0] - cell.center[0]).abs() <= cell.half && (point[1] - cell.center[1]).abs() <= cell.half;
            let size = 2.0 * cell.half;
            if !inside && size * size < theta_squared * distance_squared {
                let distance = distance_squared.sqrt().max(MIN_DISTANCE);
                let magnitude = strength * radius * cell.mass / falloff.divisor(distance);
                force[0] -= dx / distance * magnitude;
                force[1] -= dy / distance * magnitude;
            } else {
                self.stack.extend(cell.children.iter().copied().filter(|child| *child != 0));
            }
        }
        force
    }
}

#[inline]
fn pairwise_repulsion(point: [f64; 2], other: [f64; 2], radius: f64, other_radius: f64, strength: f64, falloff: LayoutRunFalloff) -> [f64; 2] {
    let dx = other[0] - point[0];
    let dy = other[1] - point[1];
    let distance = (dx * dx + dy * dy).sqrt().max(MIN_DISTANCE);
    let magnitude = strength * (radius * other_radius).max(1.0) / falloff.divisor(distance);
    [-dx / distance * magnitude, -dy / distance * magnitude]
}

fn split_mix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

fn unit_interval(state: &mut u64) -> f64 {
    (split_mix64(state) >> 11) as f64 / (1u64 << 53) as f64
}
//#endregion 🔖️Quadtree

//#region 🔖️Job
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum LayoutRunPhase {
    Initialize { cursor: u32 },
    Tree { cursor: u32 },
    Repulse { cursor: u32 },
    Attract { cursor: u32 },
    Integrate { cursor: u32 },
    Compact { cursor: u32, settle: bool },
    Done,
}

/// ⏯️ The stepped layout run. Build with `new` or `resume`, box it as the ledger's `ToolRunJob`.
pub struct LayoutRunJob<E: LayoutRunOpEncoder> {
    encoder: E,
    config: LayoutRunConfig,
    writer: ToolRunTickWriter,
    graph_digest: u64,
    entities: Vec<u64>,
    radii: Vec<f64>,
    pinned: Vec<bool>,
    origins: Vec<[f64; 2]>,
    anchors: Vec<[f64; 2]>,
    edges: Vec<(u32, u32, f64)>,
    positions: Vec<[f64; 2]>,
    velocities: Vec<[f64; 2]>,
    forces: Vec<[f64; 2]>,
    displacements: Vec<f64>,
    emitted: Vec<[f64; 2]>,
    epoch_entities: Vec<bool>,
    reasons: Vec<u16>,
    diverged: Vec<bool>,
    tree: LayoutRunTree,
    center: [f64; 2],
    bounds: [f64; 4],
    phase: LayoutRunPhase,
    checkpoint_owed: bool,
    resumed: bool,
    iteration: u32,
    settle_streak: u32,
    energy: f64,
    max_displacement: f64,
    settled: u32,
    diverged_count: u32,
    emit_cursor: u32,
    checkpoints: u32,
    stop: Option<LayoutRunStop>,
    closing: bool,
}

impl<E: LayoutRunOpEncoder> LayoutRunJob<E> {
    /// 🌱️ A fresh run: unplaced nodes are seeded from `config.seed` around their anchor or the placed centroid.
    pub fn new(identity: ToolRunIdentity, graph: &LayoutRunGraph, config: LayoutRunConfig, encoder: E) -> Result<Self, LayoutRunStartError> {
        graph.validate().map_err(LayoutRunStartError::Graph)?;
        config.validate().map_err(LayoutRunStartError::Config)?;
        let mut job = Self::allocate(identity, 0, graph, config, encoder);
        job.seed(graph);
        job.center = config.center.map_or_else(|| job.centroid(), |center| [center.x, center.y]);
        job.refresh_bounds();
        Ok(job)
    }

    /// ⏩️ Continues from a checkpoint of the same graph with (possibly changed) settings. `positions` are the nodes'
    /// current positions (the ToolRun overlay), `provisional_len` the ledger's provisional op count.
    pub fn resume(identity: ToolRunIdentity, graph: &LayoutRunGraph, config: LayoutRunConfig, encoder: E, positions: &[LayoutRunPoint], checkpoint: &[u8], provisional_len: u32) -> Result<Self, LayoutRunResumeError> {
        graph.validate().map_err(LayoutRunResumeError::Graph)?;
        config.validate().map_err(LayoutRunResumeError::Config)?;
        let checkpoint = LayoutRunCheckpoint::decode(checkpoint).ok_or(LayoutRunResumeError::Malformed)?;
        if checkpoint.node_count as usize != graph.nodes.len() || checkpoint.graph_digest != graph.digest() || checkpoint.provisional_len > provisional_len {
            return Err(LayoutRunResumeError::Foreign);
        }
        if positions.len() != graph.nodes.len() || positions.iter().any(|point| !(point.x.is_finite() && point.y.is_finite())) {
            return Err(LayoutRunResumeError::Positions);
        }
        let mut job = Self::allocate(identity, provisional_len, graph, config, encoder);
        for (index, point) in positions.iter().enumerate() {
            let position = if job.pinned[index] { job.origins[index] } else { [point.x, point.y] };
            job.positions[index] = position;
            job.emitted[index] = position;
            job.epoch_entities[index] = job.moved(index);
        }
        job.iteration = checkpoint.iteration;
        job.settle_streak = checkpoint.settle_streak;
        job.center = config.center.map_or(checkpoint.center, |center| [center.x, center.y]);
        job.resumed = true;
        job.refresh_bounds();
        Ok(job)
    }

    fn allocate(identity: ToolRunIdentity, provisional_len: u32, graph: &LayoutRunGraph, config: LayoutRunConfig, encoder: E) -> Self {
        let count = graph.nodes.len();
        let point = |point: Option<LayoutRunPoint>| point.map_or([f64::NAN, f64::NAN], |point| [point.x, point.y]);
        let origins: Vec<[f64; 2]> = graph.nodes.iter().map(|node| point(node.origin)).collect();
        Self {
            encoder,
            config,
            writer: ToolRunTickWriter::with_provisional_base(identity, provisional_len),
            graph_digest: graph.digest(),
            entities: graph.nodes.iter().map(|node| node.entity).collect(),
            radii: graph.nodes.iter().map(|node| node.radius).collect(),
            pinned: graph.nodes.iter().map(|node| node.pinned).collect(),
            anchors: graph.nodes.iter().map(|node| point(node.anchor)).collect(),
            edges: graph.edges.iter().map(|edge| (edge.source, edge.target, edge.weight)).collect(),
            positions: origins.clone(),
            emitted: origins.clone(),
            origins,
            velocities: vec![[0.0, 0.0]; count],
            forces: vec![[0.0, 0.0]; count],
            displacements: vec![f64::INFINITY; count],
            epoch_entities: vec![false; count],
            reasons: vec![u16::MAX; count],
            diverged: vec![false; count],
            tree: LayoutRunTree::default(),
            center: [0.0, 0.0],
            bounds: [0.0; 4],
            phase: LayoutRunPhase::Initialize { cursor: 0 },
            checkpoint_owed: false,
            resumed: false,
            iteration: 0,
            settle_streak: 0,
            energy: 0.0,
            max_displacement: 0.0,
            settled: 0,
            diverged_count: 0,
            emit_cursor: 0,
            checkpoints: 0,
            stop: None,
            closing: false,
        }
    }

    fn seed(&mut self, graph: &LayoutRunGraph) {
        let placed: Vec<[f64; 2]> = self.origins.iter().copied().filter(|origin| origin[0].is_finite()).collect();
        let fallback = self.config.center.map_or([0.0, 0.0], |center| [center.x, center.y]);
        let anchor = if placed.is_empty() { fallback } else { [placed.iter().map(|point| point[0]).sum::<f64>() / placed.len() as f64, placed.iter().map(|point| point[1]).sum::<f64>() / placed.len() as f64] };
        let unplaced = graph.nodes.len() - placed.len();
        let side = self.config.ideal_edge_length * (unplaced.max(1) as f64).sqrt();
        let mut state = self.config.seed;
        for index in 0..graph.nodes.len() {
            if self.positions[index][0].is_finite() {
                continue;
            }
            let around = if self.anchors[index][0].is_finite() { self.anchors[index] } else { anchor };
            let u = unit_interval(&mut state);
            let v = unit_interval(&mut state);
            self.positions[index] = [around[0] + (u - 0.5) * side, around[1] + (v - 0.5) * side];
        }
    }

    fn centroid(&self) -> [f64; 2] {
        if self.positions.is_empty() {
            return [0.0, 0.0];
        }
        let count = self.positions.len() as f64;
        [self.positions.iter().map(|point| point[0]).sum::<f64>() / count, self.positions.iter().map(|point| point[1]).sum::<f64>() / count]
    }

    fn refresh_bounds(&mut self) {
        self.bounds = self.positions.iter().fold([f64::INFINITY, f64::INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY], |bounds, point| [bounds[0].min(point[0]), bounds[1].min(point[1]), bounds[2].max(point[0]), bounds[3].max(point[1])]);
    }

    fn moved(&self, index: usize) -> bool {
        !self.pinned[index] && (self.origins[index][0].is_nan() || self.positions[index] != self.origins[index])
    }

    pub fn identity(&self) -> ToolRunIdentity {
        self.writer.identity()
    }

    pub fn stage(&self) -> LayoutRunStage {
        match self.phase {
            LayoutRunPhase::Initialize { .. } => LayoutRunStage::Initialize,
            LayoutRunPhase::Compact { settle: true, .. } | LayoutRunPhase::Done => LayoutRunStage::Settle,
            _ => LayoutRunStage::Iterate,
        }
    }

    pub fn iteration(&self) -> u32 {
        self.iteration
    }

    pub fn stop(&self) -> Option<LayoutRunStop> {
        self.stop
    }

    pub fn checkpoints(&self) -> u32 {
        self.checkpoints
    }

    pub fn positions(&self) -> Vec<LayoutRunPoint> {
        self.positions.iter().map(|point| LayoutRunPoint::new(point[0], point[1])).collect()
    }

    pub fn positions_digest(&self) -> u64 {
        layout_run_positions_digest(&self.positions)
    }

    /// 🧮️ Counter values in `LayoutRunCounter::ALL` order.
    pub fn counters(&self) -> [u64; 4] {
        [u64::from(self.iteration), (self.energy * 1000.0).round() as u64, (self.max_displacement * 1000.0).round() as u64, u64::from(self.settled)]
    }

    pub fn checkpoint(&self) -> LayoutRunCheckpoint {
        LayoutRunCheckpoint { node_count: self.positions.len() as u32, graph_digest: self.graph_digest, iteration: self.iteration, settle_streak: self.settle_streak, provisional_len: self.writer.provisional_len(), center: self.center }
    }

    fn cooling(&self) -> f64 {
        (1.0 - f64::from(self.iteration) / f64::from(self.config.max_iterations)).max(self.config.cooling_floor)
    }

    fn uses_tree(&self) -> bool {
        self.positions.len() > self.config.pairwise_max_bodies as usize && self.config.barnes_hut_theta > 0.0
    }

    fn compact_threshold(&self) -> u32 {
        self.config.compact_ops.max(2 * self.positions.len() as u32).min(TOOL_RUN_PROVISIONAL_OPS_MAX)
    }

    fn trace_reason(&self, index: usize) -> LayoutRunReason {
        if self.pinned[index] {
            LayoutRunReason::Pinned
        } else if self.diverged[index] {
            LayoutRunReason::Diverged
        } else if self.displacements[index] < self.config.settle_displacement {
            LayoutRunReason::Settled
        } else {
            LayoutRunReason::Moving
        }
    }

    fn final_reason(&self, index: usize) -> LayoutRunReason {
        match self.trace_reason(index) {
            LayoutRunReason::Moving => LayoutRunReason::Unsettled,
            reason => reason,
        }
    }

    fn upsert(&mut self, index: usize, reason: LayoutRunReason) {
        self.reasons[index] = reason.code();
        self.writer.upsert(index as u64, reason.verdict(), reason.code(), ToolRunTraceSubject::Entity { entity: self.entities[index] });
    }

    fn append_move(&mut self, index: usize) -> Result<(), String> {
        let position = self.positions[index];
        let op = self.encoder.encode_move(index as u32, self.entities[index], LayoutRunPoint::new(position[0], position[1])).map_err(|error| error.0)?;
        if op.len() > LAYOUT_RUN_OP_BYTES_MAX {
            return Err(format!("layout move op of node {index} takes {} bytes, above {LAYOUT_RUN_OP_BYTES_MAX}", op.len()));
        }
        self.writer.append_op(op).map_err(|error| format!("{error:?}"))?;
        if !self.epoch_entities[index] {
            self.epoch_entities[index] = true;
            self.writer.append_entity(self.entities[index]);
        }
        self.emitted[index] = position;
        Ok(())
    }

    fn progress(&mut self, state: ToolRunState) {
        let counters = self.counters();
        self.writer.progress(ToolRunProgress {
            identity: self.writer.identity(),
            sequence: 0,
            state,
            stage: self.stage().index(),
            completed: u64::from(self.iteration),
            total: Some(u64::from(self.config.max_iterations.max(self.iteration))),
            counters: LayoutRunCounter::ALL.iter().map(|counter| ToolRunCounter { counter: counter.index(), value: counters[counter.index() as usize] }).collect(),
            units_per_second: 0.0,
            conflicts: 0,
            steps: ToolRunStepRing::new(),
        });
    }

    fn step_args(&self) -> [ToolRunStepArg; 3] {
        [ToolRunStepArg::Unsigned(u64::from(self.iteration)), ToolRunStepArg::Float(self.max_displacement), ToolRunStepArg::Float(self.energy)]
    }

    fn flush(&mut self, cx: &mut StepContext<'_>) -> StepOutcome {
        let Some(tick) = self.writer.finish() else { return StepOutcome::Yield };
        match tick.encode() {
            Ok(bytes) if bytes.len() <= JOB_PAYLOAD_PAGE_BYTES => match cx.payload_from_bytes(JobPayloadStream::Preview, &bytes) {
                Ok(payload) => StepOutcome::PreviewReady(payload),
                Err(rejected) => {
                    drop(rejected.into_source());
                    Self::fault(cx, "layout run preview page was refused")
                }
            },
            Ok(bytes) => Self::fault(cx, &format!("layout run tick takes {} bytes, above one payload page", bytes.len())),
            Err(error) => Self::fault(cx, &format!("layout run tick does not encode: {error:?}")),
        }
    }

    fn fault(cx: &mut StepContext<'_>, message: &str) -> StepOutcome {
        let detail = match cx.payload_from_bytes(JobPayloadStream::Fault, &message.as_bytes()[..message.len().min(JOB_PAYLOAD_PAGE_BYTES)]) {
            Ok(detail) => detail,
            Err(rejected) => {
                drop(rejected.into_source());
                RetainedJobPayload::empty(JobPayloadStream::Fault)
            }
        };
        StepOutcome::Fault(JobFault { detail })
    }

    fn initialize(&mut self, cx: &mut StepContext<'_>, cursor: u32) -> StepOutcome {
        let count = self.positions.len();
        if cursor == 0 {
            let (reason, args) = if self.resumed { (LayoutRunReason::Resumed, vec![ToolRunStepArg::Unsigned(u64::from(self.iteration))]) } else { (LayoutRunReason::Initialized, vec![ToolRunStepArg::Unsigned(count as u64), ToolRunStepArg::Unsigned(self.edges.len() as u64)]) };
            if self.writer.step(ToolRunStepKind::Info, LayoutRunStage::Initialize.index(), reason.code(), None, &args).is_err() {
                return Self::fault(cx, "layout run step arguments exceed the limit");
            }
        }
        let mut index = cursor as usize;
        if !self.resumed {
            while index < count && self.writer.pending_bytes() < LAYOUT_RUN_TICK_FLUSH_BYTES {
                let reason = if self.pinned[index] { LayoutRunReason::Pinned } else { LayoutRunReason::Moving };
                self.upsert(index, reason);
                index += 1;
            }
        }
        self.progress(ToolRunState::Running);
        if self.resumed || index == count {
            self.phase = if count == 0 {
                self.settle_phase(LayoutRunStop::Converged)
            } else if self.iteration >= self.config.max_iterations {
                self.settle_phase(LayoutRunStop::IterationLimit)
            } else {
                LayoutRunPhase::Tree { cursor: 0 }
            };
        } else {
            self.phase = LayoutRunPhase::Initialize { cursor: index as u32 };
        }
        self.flush(cx)
    }

    fn settle_phase(&mut self, stop: LayoutRunStop) -> LayoutRunPhase {
        self.stop = Some(stop);
        LayoutRunPhase::Compact { cursor: 0, settle: true }
    }

    fn iterate(&mut self, cx: &mut StepContext<'_>) -> Option<StepOutcome> {
        let count = self.positions.len();
        let cool = self.cooling();
        loop {
            if cx.is_cancelled() {
                return Some(StepOutcome::Cancelled);
            }
            match self.phase {
                LayoutRunPhase::Tree { cursor } => {
                    if !self.uses_tree() {
                        self.forces.iter_mut().for_each(|force| *force = [0.0, 0.0]);
                        self.phase = LayoutRunPhase::Repulse { cursor: 0 };
                        continue;
                    }
                    if cursor == 0 {
                        self.forces.iter_mut().for_each(|force| *force = [0.0, 0.0]);
                        self.tree.reset(self.bounds, count);
                    }
                    let end = (cursor as usize + LAYOUT_RUN_LINEAR_CHUNK).min(count);
                    for body in cursor as usize..end {
                        self.tree.insert(body as u32, &self.positions, &self.radii);
                    }
                    self.phase = if end == count { LayoutRunPhase::Repulse { cursor: 0 } } else { LayoutRunPhase::Tree { cursor: end as u32 } };
                }
                LayoutRunPhase::Repulse { cursor } => {
                    let strength = self.config.repulsion_strength * cool;
                    let end = (cursor as usize + LAYOUT_RUN_REPULSION_CHUNK).min(count);
                    if self.uses_tree() {
                        let theta_squared = self.config.barnes_hut_theta * self.config.barnes_hut_theta;
                        for body in cursor as usize..end {
                            let push = self.tree.repulsion(body as u32, &self.positions, &self.radii, strength, self.config.repulsion_falloff, theta_squared);
                            self.forces[body][0] += push[0];
                            self.forces[body][1] += push[1];
                        }
                    } else {
                        for body in cursor as usize..end {
                            for other in body + 1..count {
                                let push = pairwise_repulsion(self.positions[body], self.positions[other], self.radii[body], self.radii[other], strength, self.config.repulsion_falloff);
                                self.forces[body][0] += push[0];
                                self.forces[body][1] += push[1];
                                self.forces[other][0] -= push[0];
                                self.forces[other][1] -= push[1];
                            }
                        }
                    }
                    self.phase = if end == count { LayoutRunPhase::Attract { cursor: 0 } } else { LayoutRunPhase::Repulse { cursor: end as u32 } };
                }
                LayoutRunPhase::Attract { cursor } => {
                    let strength = self.config.spring_strength * cool;
                    let ideal = self.config.ideal_edge_length;
                    let end = (cursor as usize + LAYOUT_RUN_LINEAR_CHUNK).min(self.edges.len());
                    for edge in cursor as usize..end {
                        let (source, target, weight) = self.edges[edge];
                        let (a, b) = (self.positions[source as usize], self.positions[target as usize]);
                        let (dx, dy) = (b[0] - a[0], b[1] - a[1]);
                        let distance = (dx * dx + dy * dy).sqrt().max(MIN_DISTANCE);
                        let magnitude = strength * self.config.spring_law.stretch(distance, ideal) * weight;
                        let pull = [dx / distance * magnitude, dy / distance * magnitude];
                        self.forces[source as usize][0] += pull[0];
                        self.forces[source as usize][1] += pull[1];
                        self.forces[target as usize][0] -= pull[0];
                        self.forces[target as usize][1] -= pull[1];
                    }
                    self.phase = if end == self.edges.len() { LayoutRunPhase::Integrate { cursor: 0 } } else { LayoutRunPhase::Attract { cursor: end as u32 } };
                }
                LayoutRunPhase::Integrate { cursor } => {
                    if cursor == 0 {
                        self.energy = 0.0;
                        self.max_displacement = 0.0;
                        self.settled = 0;
                        self.diverged_count = 0;
                        self.bounds = [f64::INFINITY, f64::INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY];
                    }
                    let end = (cursor as usize + LAYOUT_RUN_LINEAR_CHUNK).min(count);
                    for body in cursor as usize..end {
                        self.integrate(body, cool);
                    }
                    if end < count {
                        self.phase = LayoutRunPhase::Integrate { cursor: end as u32 };
                    } else {
                        return Some(self.finish_iteration(cx));
                    }
                }
                _ => return None,
            }
            if cx.deadline_exceeded() {
                return Some(StepOutcome::Yield);
            }
        }
    }

    fn integrate(&mut self, body: usize, cool: f64) {
        let point = self.positions[body];
        if self.pinned[body] {
            self.positions[body] = self.origins[body];
            self.velocities[body] = [0.0, 0.0];
            self.displacements[body] = 0.0;
            self.diverged[body] = false;
        } else {
            let gravity = self.config.gravity * cool;
            let anchor_strength = self.config.anchor_strength * cool;
            let mut force = self.forces[body];
            force[0] += (self.center[0] - point[0]) * gravity;
            force[1] += (self.center[1] - point[1]) * gravity;
            let anchor = self.anchors[body];
            if anchor[0].is_finite() {
                let (dx, dy) = (anchor[0] - point[0], anchor[1] - point[1]);
                let pull = anchor_strength * self.config.spring_law.anchor_scale((dx * dx + dy * dy).sqrt(), self.config.ideal_edge_length);
                force[0] += dx * pull;
                force[1] += dy * pull;
            }
            let dt = self.config.time_step * cool.sqrt();
            let damping = self.config.velocity_damping;
            let mut velocity = [(self.velocities[body][0] + force[0] * dt) * damping, (self.velocities[body][1] + force[1] * dt) * damping];
            let speed = (velocity[0] * velocity[0] + velocity[1] * velocity[1]).sqrt();
            if speed > self.config.max_speed {
                let scale = self.config.max_speed / speed;
                velocity = [velocity[0] * scale, velocity[1] * scale];
            }
            let next = [point[0] + velocity[0] * dt, point[1] + velocity[1] * dt];
            if next[0].is_finite() && next[1].is_finite() && velocity[0].is_finite() && velocity[1].is_finite() {
                let (dx, dy) = (next[0] - point[0], next[1] - point[1]);
                self.positions[body] = next;
                self.velocities[body] = velocity;
                self.displacements[body] = (dx * dx + dy * dy).sqrt();
                self.diverged[body] = false;
            } else {
                self.velocities[body] = [0.0, 0.0];
                self.displacements[body] = 0.0;
                self.diverged[body] = true;
                self.diverged_count += 1;
            }
        }
        let velocity = self.velocities[body];
        self.energy += 0.5 * (velocity[0] * velocity[0] + velocity[1] * velocity[1]);
        self.max_displacement = self.max_displacement.max(self.displacements[body]);
        if !self.diverged[body] && self.displacements[body] < self.config.settle_displacement {
            self.settled += 1;
        }
        let point = self.positions[body];
        self.bounds = [self.bounds[0].min(point[0]), self.bounds[1].min(point[1]), self.bounds[2].max(point[0]), self.bounds[3].max(point[1])];
    }

    fn finish_iteration(&mut self, cx: &mut StepContext<'_>) -> StepOutcome {
        self.iteration += 1;
        cx.consume_fuel(1);
        self.settle_streak = if self.diverged_count == 0 && self.max_displacement < self.config.settle_displacement { self.settle_streak + 1 } else { 0 };
        if self.diverged_count > 0 && self.writer.step(ToolRunStepKind::Danger, LayoutRunStage::Iterate.index(), LayoutRunReason::Diverged.code(), None, &[ToolRunStepArg::Unsigned(u64::from(self.diverged_count))]).is_err() {
            return Self::fault(cx, "layout run step arguments exceed the limit");
        }
        let threshold = self.compact_threshold();
        let count = self.positions.len();
        let mut index = self.emit_cursor as usize % count;
        let mut compact = false;
        for _ in 0..count {
            if self.writer.pending_bytes() >= LAYOUT_RUN_TICK_FLUSH_BYTES {
                break;
            }
            let reason = self.trace_reason(index);
            let emitted = self.emitted[index];
            let drift = if emitted[0].is_nan() { f64::INFINITY } else { ((self.positions[index][0] - emitted[0]).powi(2) + (self.positions[index][1] - emitted[1]).powi(2)).sqrt() };
            let moves = !self.pinned[index] && drift > 0.0 && drift >= self.config.emit_displacement;
            if moves && self.writer.provisional_len() >= threshold {
                compact = true;
                break;
            }
            if moves {
                if let Err(message) = self.append_move(index) {
                    return Self::fault(cx, &message);
                }
            }
            if moves || reason.code() != self.reasons[index] {
                self.upsert(index, reason);
            }
            index = (index + 1) % count;
        }
        self.emit_cursor = index as u32;
        self.progress(ToolRunState::Running);
        self.phase = if self.settle_streak >= self.config.settle_iterations {
            self.settle_phase(LayoutRunStop::Converged)
        } else if self.iteration >= self.config.max_iterations {
            self.settle_phase(LayoutRunStop::IterationLimit)
        } else if compact || self.iteration.is_multiple_of(self.config.checkpoint_iterations) || self.writer.provisional_len() >= threshold {
            LayoutRunPhase::Compact { cursor: 0, settle: false }
        } else {
            LayoutRunPhase::Tree { cursor: 0 }
        };
        self.flush(cx)
    }

    fn compact(&mut self, cx: &mut StepContext<'_>, cursor: u32, settle: bool) -> StepOutcome {
        let count = self.positions.len();
        if cursor == 0 {
            self.writer.retract_to(0);
            self.epoch_entities.iter_mut().for_each(|flag| *flag = false);
            if !settle {
                self.velocities.iter_mut().for_each(|velocity| *velocity = [0.0, 0.0]);
            }
        }
        let mut index = cursor as usize;
        while index < count && self.writer.pending_bytes() < LAYOUT_RUN_TICK_FLUSH_BYTES {
            if self.moved(index) {
                if let Err(message) = self.append_move(index) {
                    return Self::fault(cx, &message);
                }
            } else {
                self.emitted[index] = self.positions[index];
            }
            if settle {
                let reason = self.final_reason(index);
                self.upsert(index, reason);
            }
            index += 1;
        }
        if index < count {
            self.phase = LayoutRunPhase::Compact { cursor: index as u32, settle };
            self.progress(ToolRunState::Running);
            return self.flush(cx);
        }
        if settle {
            let (kind, reason) = match self.stop {
                Some(LayoutRunStop::IterationLimit) => (ToolRunStepKind::Warning, LayoutRunReason::IterationLimit),
                _ => (ToolRunStepKind::Success, LayoutRunReason::Converged),
            };
            if self.writer.step(kind, LayoutRunStage::Settle.index(), reason.code(), None, &self.step_args()).is_err() {
                return Self::fault(cx, "layout run step arguments exceed the limit");
            }
            self.phase = LayoutRunPhase::Done;
            self.progress(ToolRunState::Complete);
        } else {
            self.checkpoints += 1;
            self.phase = LayoutRunPhase::Tree { cursor: 0 };
            self.checkpoint_owed = true;
            self.progress(ToolRunState::Running);
        }
        self.flush(cx)
    }

    fn release_one(&mut self) -> Option<usize> {
        fn take<T>(values: &mut Vec<T>) -> Option<usize> {
            (values.capacity() != 0).then(|| {
                let bytes = values.capacity() * size_of::<T>();
                *values = Vec::new();
                bytes
            })
        }
        take(&mut self.positions)
            .or_else(|| take(&mut self.velocities))
            .or_else(|| take(&mut self.forces))
            .or_else(|| take(&mut self.displacements))
            .or_else(|| take(&mut self.emitted))
            .or_else(|| take(&mut self.origins))
            .or_else(|| take(&mut self.anchors))
            .or_else(|| take(&mut self.radii))
            .or_else(|| take(&mut self.entities))
            .or_else(|| take(&mut self.pinned))
            .or_else(|| take(&mut self.epoch_entities))
            .or_else(|| take(&mut self.reasons))
            .or_else(|| take(&mut self.diverged))
            .or_else(|| take(&mut self.edges))
            .or_else(|| take(&mut self.tree.cells))
            .or_else(|| take(&mut self.tree.next))
            .or_else(|| take(&mut self.tree.stack))
    }

    fn owns_nothing(&self) -> bool {
        self.positions.capacity() == 0
            && self.velocities.capacity() == 0
            && self.forces.capacity() == 0
            && self.displacements.capacity() == 0
            && self.emitted.capacity() == 0
            && self.origins.capacity() == 0
            && self.anchors.capacity() == 0
            && self.radii.capacity() == 0
            && self.entities.capacity() == 0
            && self.pinned.capacity() == 0
            && self.epoch_entities.capacity() == 0
            && self.reasons.capacity() == 0
            && self.diverged.capacity() == 0
            && self.edges.capacity() == 0
            && self.tree.cells.capacity() == 0
            && self.tree.next.capacity() == 0
            && self.tree.stack.capacity() == 0
    }
}

impl<E: LayoutRunOpEncoder> InteractiveJob for LayoutRunJob<E> {
    fn step(&mut self, cx: &mut StepContext<'_>) -> StepOutcome {
        if cx.is_cancelled() {
            return StepOutcome::Cancelled;
        }
        if std::mem::take(&mut self.checkpoint_owed) {
            return match cx.payload_from_bytes(JobPayloadStream::CheckpointState, &self.checkpoint().encode()) {
                Ok(state) => StepOutcome::CheckpointReady(Checkpoint { state, applied_progress: u64::from(self.iteration) }),
                Err(rejected) => {
                    drop(rejected.into_source());
                    StepOutcome::Yield
                }
            };
        }
        match self.phase {
            LayoutRunPhase::Initialize { cursor } => self.initialize(cx, cursor),
            LayoutRunPhase::Compact { cursor, settle } => self.compact(cx, cursor, settle),
            LayoutRunPhase::Done => StepOutcome::Complete(CommitCandidate { state: RetainedJobPayload::empty(JobPayloadStream::CommitState), output: RetainedJobPayload::empty(JobPayloadStream::CommitOutput) }),
            _ => self.iterate(cx).unwrap_or(StepOutcome::Yield),
        }
    }

    fn begin_close(&mut self) {
        self.closing = true;
        self.writer = ToolRunTickWriter::new(self.writer.identity());
    }

    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> InteractiveJobCloseStep {
        if !self.closing || maximum_items == 0 {
            return InteractiveJobCloseStep::Blocked;
        }
        match self.release_one() {
            Some(released_bytes) => InteractiveJobCloseStep::Pending { released_items: 1, released_bytes },
            None => InteractiveJobCloseStep::Complete,
        }
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.owns_nothing()
    }
}

/// ⏩️ Where a ToolRun ledger resumes a layout run: its checkpoint, the nodes' current overlay positions and its
/// provisional op count.
#[derive(Clone, Debug, PartialEq)]
pub struct LayoutRunResume<'a> {
    pub checkpoint: &'a [u8],
    pub positions: Vec<LayoutRunPoint>,
    pub provisional_len: u32,
}

/// 🧵️ The consumer entry every layout tool's `build_tool_run_job` calls: resumes while the checkpoint and positions
/// still describe this graph, starts fresh otherwise; graph and config errors refuse either way.
pub fn layout_run_job<E: LayoutRunOpEncoder>(identity: ToolRunIdentity, graph: &LayoutRunGraph, config: LayoutRunConfig, encoder: impl Fn() -> E, resume: Option<LayoutRunResume<'_>>) -> Result<LayoutRunJob<E>, LayoutRunStartError> {
    if let Some(resume) = resume {
        match LayoutRunJob::resume(identity, graph, config, encoder(), &resume.positions, resume.checkpoint, resume.provisional_len) {
            Ok(job) => return Ok(job),
            Err(LayoutRunResumeError::Graph(error)) => return Err(LayoutRunStartError::Graph(error)),
            Err(LayoutRunResumeError::Config(error)) => return Err(LayoutRunStartError::Config(error)),
            Err(LayoutRunResumeError::Malformed | LayoutRunResumeError::Foreign | LayoutRunResumeError::Positions) => {}
        }
    }
    LayoutRunJob::new(identity, graph, config, encoder())
}

/// 🪪️ A consumer node id's entity: the first eight little-endian digest bytes of the id, the same key the ToolRun
/// provisional entity set and the `provisional` instance stamp use.
pub fn layout_run_entity(id: &str) -> u64 {
    u64::from_le_bytes(semio_framework_hash::hash(id.as_bytes()).as_bytes()[..8].try_into().expect("eight digest bytes"))
}

/// 📍️ A resume's overlay positions: every node's origin (unplaced = NaN, which refuses the resume) with the
/// provisional `moves` (node index, position) folded in order.
pub fn layout_run_overlay_positions(graph: &LayoutRunGraph, moves: impl IntoIterator<Item = (u32, LayoutRunPoint)>) -> Vec<LayoutRunPoint> {
    let mut positions: Vec<LayoutRunPoint> = graph.nodes.iter().map(|node| node.origin.unwrap_or(LayoutRunPoint::new(f64::NAN, f64::NAN))).collect();
    for (node, position) in moves {
        if let Some(slot) = positions.get_mut(node as usize) {
            *slot = position;
        }
    }
    positions
}
//#endregion 🔖️Job

//#region 🔖️Testing
/// 🔮️ Test support shared by this crate's and every consumer plugin's tests (`testing` feature): the third-party
/// layout-quality oracle (normalized stress: Euclidean distances optimally rescaled against hop distances, squared
/// relative error averaged over all connected node pairs; the `fdg-sim` Fruchterman-Reingold layout from the same
/// initial positions) and a driver that records a run the way the ToolRun ledger folds its ticks.
#[cfg(any(test, feature = "testing"))]
pub mod testing {
    use super::{LayoutRunGraph, LayoutRunPoint, LayoutRunStage};
    use fdg_sim::force::fruchterman_reingold;
    use fdg_sim::glam::Vec3;
    use fdg_sim::{Dimensions, ForceGraph, ForceGraphHelper, Simulation, SimulationParameters};
    use semio_framework_job::{drive_step, root_cancel_token, Generation, InteractiveJob, InteractiveJobCloseStep, InteractiveStage, JobPayloadCloseStep, OperationId, RetainedJobPayload, StepBudget, StepOutcome, JOB_PAYLOAD_PAGE_BYTES, INTERACTIVE_LANE_WALL_US};
    use semio_framework_tool_run::{ToolRunProgress, ToolRunStep, ToolRunTick, ToolRunTraceOp, ToolRunTraceStore, ToolRunVerdict};
    use serde::Deserialize;
    use std::collections::{BTreeSet, VecDeque};

    /// 🎛️ The `fdg-sim` Fruchterman-Reingold parameters a fixture's `oracle` section carries.
    #[derive(Clone, Copy, Debug, PartialEq, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct LayoutRunOracleParameters {
        pub fdg_scale: f32,
        pub fdg_cooloff: f32,
        pub fdg_dt: f32,
        pub fdg_updates: u64,
    }

    /// 🦘️ All-pairs hop distances over the undirected edges (`u32::MAX` = disconnected).
    pub fn layout_run_hop_distances(graph: &LayoutRunGraph) -> Vec<Vec<u32>> {
        let count = graph.nodes.len();
        let mut adjacency = vec![Vec::new(); count];
        for edge in &graph.edges {
            adjacency[edge.source as usize].push(edge.target as usize);
            adjacency[edge.target as usize].push(edge.source as usize);
        }
        (0..count)
            .map(|source| {
                let mut distance = vec![u32::MAX; count];
                let mut queue = VecDeque::from([source]);
                distance[source] = 0;
                while let Some(node) = queue.pop_front() {
                    for &next in &adjacency[node] {
                        if distance[next] == u32::MAX {
                            distance[next] = distance[node] + 1;
                            queue.push_back(next);
                        }
                    }
                }
                distance
            })
            .collect()
    }

    /// 📏️ Scale-invariant normalized stress of `positions` against `hops`; disconnected pairs are skipped.
    pub fn layout_run_normalized_stress(positions: &[[f64; 2]], hops: &[Vec<u32>]) -> f64 {
        let mut pairs = Vec::new();
        for i in 0..positions.len() {
            for j in i + 1..positions.len() {
                if hops[i][j] == u32::MAX {
                    continue;
                }
                let hop = f64::from(hops[i][j]);
                let euclid = ((positions[i][0] - positions[j][0]).powi(2) + (positions[i][1] - positions[j][1]).powi(2)).sqrt();
                pairs.push((euclid, hop));
            }
        }
        if pairs.is_empty() {
            return 0.0;
        }
        let scale = pairs.iter().map(|(euclid, hop)| euclid / hop).sum::<f64>() / pairs.iter().map(|(euclid, hop)| euclid * euclid / (hop * hop)).sum::<f64>();
        pairs.iter().map(|(euclid, hop)| ((scale * euclid - hop) / hop).powi(2)).sum::<f64>() / pairs.len() as f64
    }

    /// 🌀️ The `fdg-sim` Fruchterman-Reingold layout of `graph` started from `initial`.
    pub fn layout_run_fdg_layout(graph: &LayoutRunGraph, initial: &[LayoutRunPoint], parameters: LayoutRunOracleParameters) -> Vec<[f64; 2]> {
        let mut force_graph: ForceGraph<(), ()> = ForceGraph::default();
        let indices: Vec<_> = (0..graph.nodes.len()).map(|index| force_graph.add_force_node(format!("{index}"), ())).collect();
        for edge in &graph.edges {
            force_graph.add_edge(indices[edge.source as usize], indices[edge.target as usize], ());
        }
        let mut simulation = Simulation::from_graph(force_graph, SimulationParameters::new(200.0, Dimensions::Two, fruchterman_reingold(parameters.fdg_scale, parameters.fdg_cooloff)));
        for (index, node) in indices.iter().enumerate() {
            let weight = &mut simulation.get_graph_mut()[*node];
            let location = Vec3::new(initial[index].x as f32, initial[index].y as f32, 0.0);
            weight.location = location;
            weight.old_location = location;
            weight.velocity = Vec3::ZERO;
        }
        for _ in 0..parameters.fdg_updates {
            simulation.update(parameters.fdg_dt);
        }
        indices.iter().map(|node| simulation.get_graph()[*node].location).map(|location| [f64::from(location.x), f64::from(location.y)]).collect()
    }

    /// 🪜️ `petgraph` longest-path layering of the directed `edges` over `count` nodes: every source sits on layer 0 and
    /// every other node one layer past its deepest predecessor (`None` = cyclic).
    pub fn layout_run_longest_path_layers(count: usize, edges: &[(u32, u32)]) -> Option<Vec<u32>> {
        let mut graph = fdg_sim::petgraph::graph::DiGraph::<(), ()>::with_capacity(count, edges.len());
        let nodes: Vec<_> = (0..count).map(|_| graph.add_node(())).collect();
        for (source, target) in edges {
            graph.add_edge(nodes[*source as usize], nodes[*target as usize], ());
        }
        let order = fdg_sim::petgraph::algo::toposort(&graph, None).ok()?;
        let mut layers = vec![0_u32; count];
        for node in order {
            let layer = layers[node.index()];
            for next in graph.neighbors(node) {
                layers[next.index()] = layers[next.index()].max(layer + 1);
            }
        }
        Some(layers)
    }

    /// 📼️ Everything a driven run published, folded like the ToolRun ledger folds ticks.
    #[derive(Debug, Default)]
    pub struct LayoutRunRecording {
        pub ops: Vec<Vec<u8>>,
        pub entities: Vec<(u32, u64)>,
        pub upserts: Vec<(u64, ToolRunVerdict, u16)>,
        pub trace: Option<ToolRunTraceStore>,
        pub steps: Vec<ToolRunStep>,
        pub progress: Option<ToolRunProgress>,
        pub ticks: u32,
        pub checkpoints: u32,
        pub step_micros: Vec<u64>,
        pub complete: bool,
    }

    impl LayoutRunRecording {
        /// 🧮️ The provisional entity set after every retract.
        pub fn entity_set(&self) -> BTreeSet<u64> {
            self.entities.iter().map(|(_, entity)| *entity).collect()
        }

        fn apply(&mut self, tick: ToolRunTick) -> Result<(), String> {
            self.ticks += 1;
            if let Some(length) = tick.retract_to {
                self.ops.truncate(length as usize);
                self.entities.retain(|(mark, _)| *mark <= length);
            }
            self.ops.extend(tick.append_ops);
            let end = self.ops.len() as u32;
            self.entities.extend(tick.append_entities.into_iter().map(|entity| (end, entity)));
            let iterating = tick.progress.as_ref().is_some_and(|progress| progress.stage != LayoutRunStage::Initialize.index());
            let store = self.trace.get_or_insert_with(|| ToolRunTraceStore::new(tick.identity));
            for page in &tick.trace {
                store.apply_page(page).map_err(|rejection| format!("{rejection:?}"))?;
                if iterating {
                    self.upserts.extend(page.ops.iter().filter_map(|op| if let ToolRunTraceOp::Upsert { key, verdict, reason, .. } = op { Some((*key, *verdict, *reason)) } else { None }));
                }
            }
            self.steps.extend(tick.steps);
            if tick.progress.is_some() {
                self.progress = tick.progress;
            }
            Ok(())
        }
    }

    fn frozen_now() -> Option<u64> {
        Some(0)
    }

    fn payload_bytes(payload: &RetainedJobPayload) -> Vec<u8> {
        (0..payload.page_count()).filter_map(|index| payload.page(index)).flatten().copied().collect()
    }

    /// ▶️ Drives `job` through `drive_step` to `Complete` with `fuel` per step and a frozen clock, so the slicing is
    /// the job's own (one iteration per step) and every recorded step time is a whole, never deadline-cut step.
    pub fn layout_run_drive<J: InteractiveJob + ?Sized>(job: &mut J, fuel: u64) -> Result<LayoutRunRecording, String> {
        let mut recording = LayoutRunRecording::default();
        let cancel = root_cancel_token();
        let (mut sequence, mut verdict) = (0, None);
        while !recording.complete {
            let started = std::time::Instant::now();
            let mut outcome = drive_step(job, "layout-run-testing", OperationId(1), Generation(1), InteractiveStage::InteractiveStep, StepBudget::new(fuel, INTERACTIVE_LANE_WALL_US), cancel.clone(), frozen_now, &mut sequence, &mut verdict);
            recording.step_micros.push(started.elapsed().as_micros() as u64);
            let observed = match &outcome {
                StepOutcome::PreviewReady(payload) => ToolRunTick::decode(&payload_bytes(payload)).map_err(|error| format!("{error:?}")).and_then(|tick| recording.apply(tick)),
                StepOutcome::CheckpointReady(_) => {
                    recording.checkpoints += 1;
                    Ok(())
                }
                StepOutcome::Complete(_) => {
                    recording.complete = true;
                    Ok(())
                }
                StepOutcome::Yield => Ok(()),
                StepOutcome::Cancelled => Err("layout run cancelled".to_string()),
                StepOutcome::Fault(fault) => Err(String::from_utf8_lossy(&payload_bytes(&fault.detail)).into_owned()),
            };
            while outcome.close_step(1, JOB_PAYLOAD_PAGE_BYTES) != JobPayloadCloseStep::Complete {}
            observed?;
        }
        Ok(recording)
    }

    /// 🧹️ Closes a job the way the ledger does, one owned buffer per `close_step`.
    pub fn layout_run_close<J: InteractiveJob + ?Sized>(job: &mut J) {
        job.begin_close();
        while job.close_step(1, JOB_PAYLOAD_PAGE_BYTES) != InteractiveJobCloseStep::Complete {}
    }
}
//#endregion 🔖️Testing

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "🧪️tests/🔬️oracle/🦀️.rs"]
mod oracle_law;
