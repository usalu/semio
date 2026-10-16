//! ✏️ editor of `semio-s-artifact-stdio-epw` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

pub use standards::energyplus::subsets::any::schema::mutations::EpwMutation;
pub use standards::energyplus::subsets::any::schema::snapshot::EpwSnapshot;
#[cfg(test)]
pub use semio_s_artifact_stdio_contract::apply_mutation;
pub use semio_s_artifact_stdio_epw_mutations::{STDIO_EPW_DOCUMENT_SCHEMA};

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod energyplus {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_epw_schema::standards::energyplus::subsets::any::schema::*;
                    pub use semio_s_artifact_stdio_epw_snapshot::standards::energyplus::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_stdio_epw_mutations::standards::energyplus::subsets::any::schema::mutations;
                }
                #[path = "."]
                pub mod io {
                    #[path = "../../../🚪️io/💾️binary/🦀️.rs"]
                    pub mod binary;
                    #[path = "../../../🚪️io/📝️text/🦀️.rs"]
                    pub mod text;
                }
            }
        }
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod epw {
        #[path = "../../🦀️.rs"]
        mod component;
        pub use component::*;
        pub use semio_s_artifact_stdio_epw_snapshot::editor::epw::edit_rules;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
}
