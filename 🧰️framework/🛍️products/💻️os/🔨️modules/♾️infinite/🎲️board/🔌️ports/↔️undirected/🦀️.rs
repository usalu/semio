//! ↔ Port undirected graph: handle-to-handle edges without direction.

pub use crate::infinite::board::ports::*;

/// ↔ Port graph engine; handle endpoints are unordered pairs.
pub type UndirectedPortGraphEngine = GraphEngine<Ported, Undirected>;

/// 🪢️ Port edge with handle endpoints (unordered).
pub type Edge = GraphEdge<HandleId>;

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
