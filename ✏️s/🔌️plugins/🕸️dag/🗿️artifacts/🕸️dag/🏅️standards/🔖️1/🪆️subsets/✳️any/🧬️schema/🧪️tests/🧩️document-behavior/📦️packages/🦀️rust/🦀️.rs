//! 🧩️ document-behavior-tests of `semio-s-artifact-dag-dag` — a path-preserving carve of the artifact root mounts.

extern crate infinite_canvas as infinite_board_port_directed_dag;
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

pub use crate::standards::v1::subsets::any::schema::mutations::DagMutation;
pub use crate::standards::v1::subsets::any::schema::snapshot::DagSnapshot;
pub use semio_s_artifact_stdio_semio_graph::schema::mutations::SemioGraphMutation;
pub use crate::snapshot::schema::default_snapshot;
pub use crate::snapshot::schema::empty_snapshot;
#[cfg(test)]
pub use semio_framework_os_infinite_directed_presentation::DagNodeKind;
#[cfg(test)]
pub use semio_framework_os_infinite_directed_presentation::DagHostSnapshotEdge;
#[cfg(test)]
pub use crate::standards::v1::subsets::any::schema::diff::DagDiff;
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
pub use semio_s_artifact_dag_dag_inferences::{DAG_DOCUMENT_SCHEMA, DagScene, dag_content_snapshot, dag_edge_of_graph, dag_graph_edge, dag_scene_of_content, genesis_dag_child_pack};
pub use semio_s_artifact_dag_dag_schema::{DAG_DIALECT, create_edge_leaf, create_node_leaf, dag_child_emit, dag_scene, rename_node_leaf};
pub use crate::viewer::dag::artifact_items_11::{dag_child_restore_projection};

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_dag_dag_schema::standards::v1::subsets::any::schema::*;
                    #[cfg(test)]
                    #[path = "../../🦀️.rs"]
                    pub mod document_behavior_tests;
                    #[cfg(test)]
                    #[path = "../../../🪪️document-contract/🦀️.rs"]
                    pub mod document_contract_tests;
                    pub use semio_s_artifact_dag_dag_inferences::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_dag_dag_diff::standards::v1::subsets::any::schema::diff;
                    pub use semio_s_artifact_dag_dag_schema::standards::v1::subsets::any::schema::mutations;
                }
            }
        }
    }
}
pub use semio_s_artifact_dag_dag_inferences::snapshot;
pub use semio_s_artifact_dag_dag_inferences::examples;
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod dag {
        #[path = "."]
        pub mod config {
            pub use semio_s_artifact_dag_dag_config::editor::dag::config::*;
        }
        pub use semio_s_artifact_dag_dag_diff::editor::dag::terminology;
        #[path = "."]
        pub mod commands {
            #[path = "../../../../../✏️editor/🎮️commands/➕️add-node/🦀️.rs"]
            pub mod add_node;
            #[path = "../../../../../✏️editor/🎮️commands/🔌️connect-media-ports/🦀️.rs"]
            pub mod connect_media_ports;
            #[path = "../../../../../✏️editor/🎮️commands/🗑️delete-selection/🦀️.rs"]
            pub mod delete_selection;
            #[path = "../../../../../✏️editor/🎮️commands/🩹️patch-dag-nodes/🦀️.rs"]
            pub mod patch_dag_nodes;
            #[path = "../../../../../✏️editor/🎮️commands/➖️remove-node/🦀️.rs"]
            pub mod remove_node;
            #[path = "../../../../../✏️editor/🎮️commands/🏷️rename-dag-node/🦀️.rs"]
            pub mod rename_dag_node;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../../../../✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod tools {
                    #[path = "../../../../../✏️editor/🎭️modes/✏️edit/🛠️tools/🗂️reorganize/🦀️.rs"]
                    pub mod reorganize;
                }
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_dag_dag_diff::editor::dag::modes::edit::windows::compiled;
                    #[path = "../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️main/🦀️.rs"]
                    pub mod main;
                }
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod dag {
        #[path = "../../../../../👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../../../../👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "../../../../../👁️viewer/🎭️modes/👁️view/🪟️windows/🕸️main/🦀️.rs"]
                    pub mod main;
                }
            }
        }
    }
}
#[path = "."]
pub mod host {
#[path = "../../../../../../../../../🔨️modules/🏠️host/🧰️owned/🦀️.rs"]
pub mod owned;
}
