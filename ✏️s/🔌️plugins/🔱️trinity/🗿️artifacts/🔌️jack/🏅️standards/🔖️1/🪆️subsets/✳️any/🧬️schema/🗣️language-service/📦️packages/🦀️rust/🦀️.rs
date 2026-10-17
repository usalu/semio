//! 🗣️ language-service of `semio-s-artifact-trinity-jack` — a path-preserving carve of the artifact root mounts.

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
                        #[path = "../../../../🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                        #[path = "../../../../🚪️io/💾️binary/🔺️diff/🦀️.rs"]
                        pub mod diff;
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../../🚪️io/📝️text/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                        #[path = "../../../../🚪️io/📝️text/🔺️diff/🦀️.rs"]
                        pub mod diff;
                    }
                    #[path="../../../../🚪️io/🔣️json/🦀️.rs"] pub mod json_native;
                    #[path = "../../../../🚪️io/🪶️sqlite/🦀️.rs"]
                    pub mod sqlite;
                }
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_trinity_jack_model::standards::v1::subsets::any::schema::errors;
                    pub use semio_s_artifact_trinity_jack_model::standards::v1::subsets::any::schema::manifest;
                    pub use semio_s_artifact_trinity_jack_graph_manifest_tests::standards::v1::subsets::any::schema::graph_manifest;
                    pub use semio_s_artifact_trinity_jack_model::standards::v1::subsets::any::schema::graph;
                    #[path = "../../../../../../../../🔨️modules/🧠️memory-graph/🦀️.rs"]
                    pub mod memory;
                    pub use semio_s_artifact_trinity_jack_inferences::standards::v1::subsets::any::schema::content;
                    pub use semio_s_artifact_trinity_jack_model::standards::v1::subsets::any::schema::content_read;
                    #[path = "../../../../../../../../🔨️modules/🏗️composition/🦀️.rs"]
                    pub mod composition;
                    pub use semio_s_artifact_trinity_jack_model::standards::v1::subsets::any::schema::json_transport;
                    #[path = "../../../🌳️ast/🦀️.rs"]
                    pub mod ast;
                    pub use semio_s_artifact_trinity_jack_model::standards::v1::subsets::any::schema::lexer;
                    #[path = "../../🦀️.rs"]
                    pub mod language_service;
                    pub use semio_s_artifact_trinity_jack_inferences::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod diff {
                        #[path = "../../../🔺️diff/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod operations {
                        #[path = "../../../⚙️operations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../../🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod set_query {
                            #[path = "../../../🧬️mutations/🔎️set-query/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../🧬️mutations/🔎️set-query/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🧬️mutations/🔎️set-query/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod jack {
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../../../👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "../../../../👁️viewer/🎭️modes/👁️view/🪟️windows/🌐️graph/🦀️.rs"]
                    pub mod graph;
                }
            }
        }
    }
}
