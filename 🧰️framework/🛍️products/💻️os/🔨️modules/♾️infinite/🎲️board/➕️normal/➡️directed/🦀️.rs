//! ➡️ Normal directed graph: node-to-node relationships (mindmaps).

pub use crate::infinite::board::*;

/// ➡️ Node graph engine without ports; relationships are directed node pairs.
pub type DirectedGraphEngine = GraphEngine<Normal, Directed>;

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
