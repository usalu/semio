//! 🪆️ The artifact binds its retained content to the intrinsic graph-query implementation.
use crate::JackSnapshot;
use semio_framework_value::ValueError;
pub use semio_s_artifact_trinity_jack_graph_query::{JackTopology,infer_jack_topology};
/// 🔎️ Admits the original local child owner before the consumer inference runs.
pub fn compute_topology(snapshot:&JackSnapshot)->Result<JackTopology,ValueError>{
 let owner=crate::jack_content_for_handle(&snapshot.content)?;
 infer_jack_topology(crate::jack_content_identity(&snapshot.content),crate::jack_local_graph_query(owner.as_ref()))
}
#[cfg(test)]
#[path="🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
