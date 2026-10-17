//! 🔺️ diff of `semio-s-artifact-dag-dag` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;

pub use crate::standards::v1::subsets::any::schema::snapshot::DagSnapshot;
pub use semio_s_artifact_dag_dag_inferences::{DAG_DOCUMENT_SCHEMA, DagContentChild, DagScene};

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
                    pub mod diff {
                        #[path = "../../🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_dag_dag_inferences::standards::v1::subsets::any::io::text::snapshot;
                        #[path = "../../../../🚪️io/📝️text/🔺️diff/🦀️.rs"]
                        pub mod diff;
                    }
                    #[path = "."]
                    pub mod binary {
                        #[path = "."]
                        pub mod snapshot {
                            pub use semio_s_artifact_dag_dag_inferences::standards::v1::subsets::any::io::binary::snapshot::*;
                            #[cfg(test)]
                            #[path = "../../../../🚪️io/💾️binary/📸️snapshot/🧪️tests/🔬️semio-protocol-conformance/🦀️.rs"]
                            pub mod semio_protocol_conformance;
                        }
                        #[path = "../../../../🚪️io/💾️binary/🔺️diff/🦀️.rs"]
                        pub mod diff;
                    }
                }
            }
        }
    }
}
pub mod diff {
    pub use crate::standards::v1::subsets::any::schema::diff::*;
    pub mod schema {
        pub use crate::standards::v1::subsets::any::schema::diff::*;
    }

}
pub use semio_s_artifact_dag_dag_inferences::examples;
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod dag {
        #[path = "."]
        pub mod config {
            #[path = "."]
            pub mod io {
                #[path = "../../../../✏️editor/🎚️config/🚪️io/💾️binary/🦀️.rs"]
                pub mod binary;
                #[path = "../../../../✏️editor/🎚️config/🚪️io/📝️text/🦀️.rs"]
                pub mod text;
            }
            #[path = "../../../../✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod presence {
            #[path = "../../../../✏️editor/👥️presence/🦀️.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod io {
                #[path = "."]
                pub mod text {
                    #[path = "../../../../✏️editor/👥️presence/🚪️io/📝️text/🧬️mutations/🦀️.rs"]
                    pub mod mutations;
                }
                #[path = "."]
                pub mod binary {
                    #[path = "../../../../✏️editor/👥️presence/🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
                    pub mod mutations;
                }
            }
            #[path = "../../../../✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "../../../../✏️editor/🗣️terminology/🦀️.rs"]
        pub mod terminology;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "."]
                pub mod windows {
                    #[path = "../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🧬️compiled/🦀️.rs"]
                    pub mod compiled;
                }
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod dag {
        #[path = "."]
        pub mod presence {
            #[path = "../../../../👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "../../../../👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
    }
}
