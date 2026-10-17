//! 🔬️ tests of `semio-s-artifact-dag-dag` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as protocol;

#[cfg(test)]
pub use crate::standards::v1::subsets::any::schema::snapshot::DagSnapshot;
#[cfg(test)]
pub use crate::snapshot::schema::empty_snapshot;
#[cfg(test)]
pub use semio_framework_os_infinite_directed_presentation::DagHostSnapshotEdge;
#[cfg(test)]
pub use semio_framework_os_infinite_directed_presentation::DagNodeSpec;
#[cfg(test)]
pub use crate::snapshot::schema::default_snapshot;
pub use semio_s_artifact_dag_dag_inferences::{DagScene, dag_content_child_handle};

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
                    pub use semio_s_artifact_dag_dag_inferences::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod inferences {
                        pub use semio_s_artifact_dag_dag_inferences::standards::v1::subsets::any::schema::inferences::*;
                        #[cfg(test)]
                        //#region 🧪️Tests
                        #[path = "../../🦀️.rs"]
                        pub mod tests;
                        #[path = "."]
                        pub mod topology {
                            pub use semio_s_artifact_dag_dag_inferences::standards::v1::subsets::any::schema::inferences::topology::*;
                            #[cfg(test)]
                            //#region 🧪️Tests
                            #[path = "../../../../🧭topology/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                    }
                    #[path = "."]
                    pub mod diff {
                        pub use semio_s_artifact_dag_dag_diff::standards::v1::subsets::any::schema::diff::*;
                        #[cfg(test)]
                        #[path = "../../../../../🔺️diff/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "../../../../../../🚪️io/📝️text/🦀️.rs"]
                    pub mod text;
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
        pub mod presence {
            pub use semio_s_artifact_dag_dag_diff::editor::dag::presence::*;
            #[cfg(test)]
            #[path = "../../../../../../✏️editor/👥️presence/🧪️tests/🔬️mutation-vectors/🦀️.rs"]
            pub mod mutation_vectors;
        }
        #[path = "."]
        pub mod terminology {
            pub use semio_s_artifact_dag_dag_diff::editor::dag::terminology::*;
            #[cfg(test)]
            #[path = "../../../../../../✏️editor/🗣️terminology/🧪️tests/🔬️unit/🦀️.rs"]
            pub mod tests;
        }
    }
}
