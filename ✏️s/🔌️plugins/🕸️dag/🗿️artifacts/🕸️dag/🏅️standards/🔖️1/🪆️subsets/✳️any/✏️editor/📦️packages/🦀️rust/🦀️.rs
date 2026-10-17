//! ✏️ dag of `semio-s-artifact-dag-dag` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

pub use crate::standards::v1::subsets::any::schema::mutations::DagMutation;
pub use crate::standards::v1::subsets::any::schema::snapshot::DagSnapshot;
pub use crate::snapshot::schema::default_snapshot;
pub use semio_s_artifact_stdio_semio_graph::schema::mutations::SemioGraphMutation;
pub use crate::snapshot::schema::empty_snapshot;
#[cfg(test)]
pub use semio_framework_os_infinite_directed_presentation::DagNodeKind;
#[cfg(test)]
pub use semio_framework_os_infinite_directed_presentation::DagHostSnapshotEdge;
#[cfg(test)]
pub use semio_framework_os_infinite_directed_presentation::DagNodeSpec;
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
pub use semio_s_artifact_dag_dag_inferences::{DAG_DOCUMENT_SCHEMA, DagScene, dag_scene_of_content, genesis_dag_child_pack};
pub use semio_s_artifact_dag_dag_schema::{DAG_DIALECT, create_edge_leaf, dag_child_emit, dag_scene, dag_scene_from_children, delete_edge_leaf, drag_nodes_leaf};
pub use semio_s_artifact_dag_dag_document_behavior_tests::{dag_child_restore_projection};
pub use crate::editor::dag::artifact_items_9::{artifact_kind};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
