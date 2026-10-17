//! 🧬️ schema-2 of `semio-s-artifact-stdio-json` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub use schema::snapshot::JsonSnapshot;
pub mod schema {
    pub use super::standards::v_rfc8259::subsets::base::schema::*;
}

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v_rfc8259 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod base {
                #[path = "."]
                pub mod schema {
                    #[path = "../../🦀️.rs"]
                    mod component;
                    pub use component::*;
                    pub use semio_s_artifact_stdio_json_schema::standards::v_rfc8259::subsets::base::schema::snapshot;
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../🚪️io/💾️binary/📸️snapshot/🦀️.rs"]
                        pub mod snapshot;
                    }
                }
            }
            #[path = "."]
            pub mod i_json {
                #[path = "."]
                pub mod examples {
                    pub use semio_s_artifact_stdio_json_number::standards::v_rfc8259::subsets::i_json::examples::*;
                    #[path = "."]
                    pub mod demo {
                        pub use semio_s_artifact_stdio_json_number::standards::v_rfc8259::subsets::i_json::examples::demo::*;
                        #[cfg(test)]
                        #[path = "../../../../🛜️i-json/📚️examples/🎬️demo/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                }
                #[path = "."]
                pub mod schema {
                    #[path = "../../../../🛜️i-json/🧬️schema/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    pub use semio_s_artifact_stdio_json_schema::standards::v_rfc8259::subsets::i_json::schema::conformance;
                }
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod json_i_json {
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../../../🛜️i-json/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../../../🛜️i-json/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
}
