//! 🪆️ The artifact binds its retained content to the intrinsic graph-query implementation.
use crate::JackSnapshot;
use semio_framework_value::ValueError;
pub use semio_s_artifact_trinity_jack_graph_query::{JackFlatPosition,JackFlatPositionUv,infer_jack_flat_position};
/// 🔎️ Admits the original local child owner before the consumer inference runs.
pub fn compute_flat_position(snapshot:&JackSnapshot)->Result<JackFlatPosition,ValueError>{
 let owner=crate::jack_content_for_handle(&snapshot.content)?;
 infer_jack_flat_position(crate::jack_content_identity(&snapshot.content),snapshot.root_node_id.as_deref(),crate::jack_local_graph_query(owner.as_ref()))
}
#[cfg(test)]
#[path="🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
