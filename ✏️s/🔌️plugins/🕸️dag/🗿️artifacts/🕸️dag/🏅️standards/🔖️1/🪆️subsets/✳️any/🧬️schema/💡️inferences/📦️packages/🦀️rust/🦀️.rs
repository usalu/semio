//! 💡️ inferences of `semio-s-artifact-dag-dag` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;

pub use crate::standards::v1::subsets::any::schema::snapshot::DagSnapshot;
pub use semio_framework_os_infinite_directed_presentation::DagHostSnapshotEdge;
pub use semio_framework_os_infinite_directed_presentation::DagNodeSpec;
pub use semio_framework_os_infinite_directed_presentation::DagExpandedPaths;
pub use semio_framework_os_infinite_directed_presentation::DagNodeKind;
pub use semio_framework_os_infinite_directed_presentation::DagPreviewContent;
pub use semio_framework_os_infinite_directed_presentation::IoPortSpec;
pub use crate::editor::dag::config::io::artifact_items::{DAG_DOCUMENT_SCHEMA};
pub use crate::editor::dag::config::io::artifact_items_3::{DAG_EDGE_KIND, DAG_EDGE_NATIVE_FIELDS, DAG_NODE_NATIVE_FIELDS, DagContentChild, DagScene, SemioGraphMutation, dag_content_child_handle, dag_content_snapshot, dag_edge_of_graph, dag_graph_edge, dag_graph_node, dag_node_of_graph, dag_record_properties, dag_scene_of_content, dsl_value_of, semio_value_of};
pub use crate::editor::dag::config::io::artifact_items_6::{dag_derivable_scene, genesis_dag_child_pack};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
