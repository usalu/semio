//! 🌱️ Typed registry cache ingestion.
use neural_engine::{Dictionary, NeuralCache};
/// 🌱️ Transfers an admitted node output into the shared neural cache.
pub fn seed_flow_eval_node_cache(cache: &NeuralCache, node_hash: u64, output: Dictionary) { cache.seed(node_hash, output); }
