//! 🧩️ Layout run input: the points, nodes and edges a run lays out, their validation and the positions digest.

use super::*;

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
    pub(crate) fn new() -> Self {
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

    pub(crate) fn finish(&self) -> u64 {
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
