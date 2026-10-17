//! 🧬️ schema of `semio-s-artifact-dag-dag` — a path-preserving carve of the artifact root mounts.

extern crate infinite_canvas as infinite_board_port_directed_dag;
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;

pub use crate::standards::v1::subsets::any::schema::snapshot::DagSnapshot;
pub use crate::standards::v1::subsets::any::schema::mutations::DagMutation;
pub use semio_framework_os_infinite_directed_presentation::DagNodeKind;
pub use semio_framework_os_infinite_directed_presentation::DagPreviewContent;
pub use semio_framework_os_infinite_directed_presentation::IoPortSpec;
pub use semio_s_artifact_stdio_semio_graph::schema::mutations::SemioGraphMutation;
pub use crate::snapshot::schema::default_snapshot;
pub use semio_framework_os_infinite_directed_presentation::DagExpandedPaths;
pub use semio_framework_os_infinite_directed_presentation::DagHostSnapshotEdge;
pub use semio_framework_os_infinite_directed_presentation::DagNodeSpec;
pub use crate::snapshot::schema::empty_snapshot;
pub use semio_s_artifact_dag_dag_inferences::{DAG_DOCUMENT_SCHEMA, DagContentChild, DagScene, dag_graph_edge, dag_graph_node, dag_scene_of_content, semio_value_of};
pub use crate::standards::v1::subsets::any::schema::artifact_items_2::{DAG_DIALECT};
pub use crate::standards::v1::subsets::any::schema::artifact_items_4::{dag_scene, dag_scene_from_children};
pub use crate::standards::v1::subsets::any::schema::artifact_items_5::{dag_child_emit};
pub use crate::standards::v1::subsets::any::schema::artifact_items_7::{create_edge_leaf, create_node_leaf, delete_edge_leaf, drag_nodes_leaf, rename_node_leaf};
pub use crate::standards::v1::subsets::any::schema::artifact_items_8::{DagCamera};

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
                    #[path = "../../🦀️.rs"]
                    mod component;
                    pub use component::*;
                    pub use semio_s_artifact_dag_dag_inferences::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "../../../🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                    }
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_dag_dag_tests::standards::v1::subsets::any::io::text::*;
                        #[path = "../../../🚪️io/📝️text/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                    }
                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_dag_dag_inferences::standards::v1::subsets::any::io::import::deserializers::artifacts::txt;
                                pub use semio_s_artifact_dag_dag_inferences::standards::v1::subsets::any::io::import::deserializers::artifacts::json;
                            }
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_dag_dag_inferences::standards::v1::subsets::any::io::export::serializers::artifacts::txt;
                                pub use semio_s_artifact_dag_dag_inferences::standards::v1::subsets::any::io::export::serializers::artifacts::json;
                            }
                        }
                    }
                }
            }
        }
    }
}
pub use semio_s_artifact_dag_dag_diff::diff;
pub use semio_s_artifact_dag_dag_inferences::snapshot;
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod dag {
        #[path = "."]
        pub mod config {
            pub use semio_s_artifact_dag_dag_config::editor::dag::config::*;
        }
        #[path = "."]
        pub mod commands {
            #[path = "../../../✏️editor/🎮️commands/✂️disconnect/🦀️.rs"]
            pub mod disconnect;
            #[path = "../../../✏️editor/🎮️commands/👇️graph-pointer-down/🦀️.rs"]
            pub mod graph_pointer_down;
            #[path = "../../../✏️editor/🎮️commands/🔭️node-graph-viewport/🦀️.rs"]
            pub mod node_graph_viewport;
        }
    }
}
