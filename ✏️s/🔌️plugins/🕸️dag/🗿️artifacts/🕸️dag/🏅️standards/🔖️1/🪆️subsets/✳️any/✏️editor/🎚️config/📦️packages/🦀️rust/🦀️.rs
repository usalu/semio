//! 🎚️ config of `semio-s-artifact-dag-dag` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        #[path = "."]
                        pub mod snapshot {
                            pub use semio_s_artifact_dag_dag_inferences::standards::v1::subsets::any::io::text::snapshot::*;
                            #[cfg(test)]
                            #[path = "../../../../🚪️io/📝️text/📸️snapshot/🧪️tests/🔬️semio-grammar-conformance/🦀️.rs"]
                            pub mod semio_grammar_conformance;
                        }
                        #[path = "."]
                        pub mod diff {
                            pub use semio_s_artifact_dag_dag_diff::standards::v1::subsets::any::io::text::diff::*;
                            #[cfg(test)]
                            #[path = "../../../../🚪️io/📝️text/🔺️diff/🧪️tests/🔬️semio-grammar-conformance/🦀️.rs"]
                            pub mod semio_grammar_conformance;
                        }
                        #[path = "../../../../🚪️io/📝️text/💡️inferences/🦀️.rs"]
                        pub mod inferences;
                    }
                    #[path = "../../../../🚪️io/💾️binary/🦀️.rs"]
                    pub mod binary;
                    #[path = "../../../../🚪️io/🪶️sqlite/🦀️.rs"]
                    pub mod sqlite;
                }
            }
        }
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod dag {
        #[path = "."]
        pub mod config {
            #[path = "../../🦀️.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod io {
                #[path = "."]
                pub mod text {
                    #[path = "../../🚪️io/📝️text/🧬️mutations/🦀️.rs"]
                    pub mod mutations;
                }
                #[path = "."]
                pub mod binary {
                    #[path = "../../🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
                    pub mod mutations;
                }
            }
        }
        #[path = "."]
        pub mod presence {
            #[path = "../../../👥️presence/🚪️io/🦀️.rs"]
            pub mod io;
        }
        #[path = "../../../📚️examples/🎬️demo-session/🦀️.rs"]
        pub mod demo_session;
        #[cfg(test)]
        #[path = "../../../📚️examples/🎬️demo-session/🧪️tests/🧩️example/🦀️.rs"]
        pub mod example;
    }
}
