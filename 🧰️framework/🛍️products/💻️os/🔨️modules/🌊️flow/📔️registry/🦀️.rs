//! 📔️ Flow registry ownership and host cache projection.
pub use semio_framework_artifact_flow_flow::registry::*;
use neural_engine::{Dictionary, NeuralCache};
use crate::host::FlowCoreError;

/// 🌱️ Seeds a shared neural cache entry from a host-mediated extension eval response.
pub fn seed_flow_eval_node_cache(cache: &NeuralCache, node_hash: u64, output_json: &str) -> Result<(), FlowCoreError> {
    let dict: Dictionary = semio_framework_pack_json::from_json_str(output_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| FlowCoreError::Json(error.to_string()))?;
    cache.seed(node_hash, dict);
    Ok(())
}


#[cfg(test)]
#[path = "🧪️tests/📔️registry/🦀️.rs"]
mod tests;
