//! ↔️ Undirected Board engine specialization.
pub use crate::infinite::board::*;
pub type UndirectedGraphEngine=GraphEngine<Normal,Undirected>;
#[cfg(test)]
#[path="🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
