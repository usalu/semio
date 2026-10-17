//! 🪆️ The artifact binds its retained content to the intrinsic graph-query implementation.
use crate::standards::v1::subsets::any::schema::snapshot::JackSnapshot;
use semio_framework_value::ValueError;
pub use semio_s_artifact_trinity_jack_graph_query::{JackFlatPosition,JackFlatPositionUv,infer_jack_flat_position};
/// 🔎️ Admits the original local child owner before the consumer inference runs.
pub fn compute_flat_position(snapshot:&JackSnapshot)->Result<JackFlatPosition,ValueError>{
 let owner=crate::standards::v1::subsets::any::schema::content::jack_content_for_handle(&snapshot.content)?;
 infer_jack_flat_position(crate::standards::v1::subsets::any::schema::content::jack_content_identity(&snapshot.content),snapshot.root_node_id.as_deref(),crate::standards::v1::subsets::any::schema::content::jack_local_graph_query(owner.as_ref()))
}
