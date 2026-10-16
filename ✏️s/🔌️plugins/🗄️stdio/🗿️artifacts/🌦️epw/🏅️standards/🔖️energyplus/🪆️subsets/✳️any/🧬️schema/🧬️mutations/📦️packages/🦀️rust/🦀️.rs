//! 🧬️ mutations of `semio-s-artifact-stdio-epw` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_value_derive as value_derive;

pub use standards::energyplus::subsets::any::schema::mutations::EpwMutation;
pub use standards::energyplus::subsets::any::schema::snapshot::EpwSnapshot;
#[cfg(test)]
pub use semio_s_artifact_stdio_contract::apply_mutation;
pub use crate::viewer::epw::artifact_items::{STDIO_EPW_DOCUMENT_SCHEMA};

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod energyplus {
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
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../../🚪️io/📝️text/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                        pub use semio_s_artifact_stdio_epw_snapshot::standards::energyplus::subsets::any::io::text::diff;
                    }
                }
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_epw_schema::standards::energyplus::subsets::any::schema::*;
                    pub use semio_s_artifact_stdio_epw_snapshot::standards::energyplus::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod diff {
                        pub use semio_s_artifact_stdio_epw_snapshot::standards::energyplus::subsets::any::schema::diff::*;
                        #[cfg(test)]
                        #[path = "../../../🔺️diff/🧪️tests/🔬️handcrafted-diff-codec/🦀️.rs"]
                        mod handcrafted_diff_codec_tests;
                    }
                    #[path = "."]
                    pub mod inferences {
                        pub use semio_s_artifact_stdio_epw_schema::standards::energyplus::subsets::any::schema::inferences::*;
                        #[path = "."]
                        pub mod climate {
                            pub use semio_s_artifact_stdio_epw_schema::standards::energyplus::subsets::any::schema::inferences::climate::*;
                            #[cfg(test)]
                            //#region 🧪️Tests
                            #[path = "../../../💡️inferences/🌡️climate/🧪️tests/🔬️unit/🦀️.rs"]
                            mod tests;
                        }
                        #[cfg(test)]
                        //#region 🧪️Tests
                        #[path = "../../../💡️inferences/🧪️tests/🔬️unit/🦀️.rs"]
                        mod tests;
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod epw {
        #[path = "../../../../👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../../../👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_stdio_epw_snapshot::viewer::epw::modes::view::windows::*;
                    #[path = "."]
                    pub mod main {
                        pub use semio_s_artifact_stdio_epw_snapshot::viewer::epw::modes::view::windows::main::*;
                        #[cfg(test)]
                        #[path = "../../../../👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🧪️tests/🔬️unit/🦀️.rs"]
                        mod tests;
                    }
                }
            }
        }
    }
}
