//! 💡️ inferences of `semio-s-artifact-trinity-jack` — a path-preserving carve of the artifact root mounts.

#![allow(clippy::unnecessary_wraps)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

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
                    pub mod binary {
                        #[path = "../../../../🚪️io/💾️binary/📸️snapshot/🦀️.rs"]
                        pub mod snapshot;
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../../🚪️io/📝️text/📸️snapshot/🦀️.rs"]
                        pub mod snapshot;
                    }
                    #[path = "."]
                    pub mod sqlite {
                        #[path = "../../../../🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs"]
                        pub mod snapshot;
                    }
                }
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_trinity_jack_model::standards::v1::subsets::any::schema::errors;
                    pub use semio_s_artifact_trinity_jack_model::standards::v1::subsets::any::schema::manifest;
                    pub use semio_s_artifact_trinity_jack_graph_manifest_tests::standards::v1::subsets::any::schema::graph_manifest;
                    pub use semio_s_artifact_trinity_jack_model::standards::v1::subsets::any::schema::graph;
                    #[path = "../../../../../../../../🪆️content/🦀️.rs"]
                    pub mod content;
                    pub use semio_s_artifact_trinity_jack_model::standards::v1::subsets::any::schema::content_read;
                    pub use semio_s_artifact_trinity_jack_model::standards::v1::subsets::any::schema::json_transport;
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "../../../📸️snapshot/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod inferences {
                        #[path = "../../🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod topology {
                            #[path = "../../🧭topology/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod flat_position {
                            #[path = "../../🎛️flat-position/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
            }
        }
    }
}
