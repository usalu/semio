//! 🔖️ v1 of `semio-s-artifact-dag-dag` — a path-preserving carve of the artifact root mounts.

pub use semio_s_artifact_dag_dag_schema::{DAG_DIALECT};
pub use crate::standards::v1::artifact_items_10::{DagApplication, artifact, definition};

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "../../🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "../../🪆️subsets/✳️any/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_dag_dag_schema::standards::v1::subsets::any::schema::*;
                    pub use semio_s_artifact_dag_dag_inferences::standards::v1::subsets::any::schema::inferences;
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_dag_dag_schema::standards::v1::subsets::any::io::*;
                }
            }
        }
    }
}
pub use semio_s_artifact_dag_dag_inferences::examples;
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod dag {
        pub use semio_s_artifact_dag_dag_dag::editor::dag::*;
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod dag {
        pub use semio_s_artifact_dag_dag_document_behavior_tests::viewer::dag::*;
    }
}
