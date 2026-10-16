//! 🎛️ Layout run configuration: force laws and falloff, the op encoder seam and the resumable checkpoint.

use super::*;

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
    pub(crate) fn divisor(self, distance: f64) -> f64 {
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
    pub(crate) fn stretch(self, distance: f64, ideal: f64) -> f64 {
        match self {
            Self::Linear => distance - ideal,
            Self::Quadratic => distance * distance / ideal,
        }
    }

    #[inline]
    pub(crate) fn anchor_scale(self, distance: f64, ideal: f64) -> f64 {
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
