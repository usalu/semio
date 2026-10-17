//! 👁️ viewer of `semio-s-artifact-space-home` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

pub use crate::standards::v1::subsets::any::schema::snapshot::SHomeSnapshot;
pub use semio_s_artifact_space_home_inferences::{S_HOME_DOCUMENT_SCHEMA};
pub use semio_s_artifact_space_home_kinds_conformance::{HOME_DIALECT};

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
                    pub use semio_s_artifact_space_home_inferences::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_space_home_operations::standards::v1::subsets::any::schema::mutations::*;
                        #[path = "."]
                        pub mod change_catalog_generation {
                            pub use semio_s_artifact_space_home_operations::standards::v1::subsets::any::schema::mutations::change_catalog_generation::*;
                            pub use semio_s_artifact_space_home_operations::standards::v1::subsets::any::schema::mutations::change_catalog_generation::diff;
                            pub use semio_s_artifact_space_home_operations::standards::v1::subsets::any::schema::mutations::change_catalog_generation::inverse;
                        }
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod home {
        pub use semio_s_artifact_space_home_kinds_conformance::editor::home::*;
        #[path = "."]
        pub mod config {
            pub use semio_s_artifact_space_home_examples::editor::home::config::*;
            pub use semio_s_artifact_space_home_examples::editor::home::config::schema;
        }
        pub use semio_s_artifact_space_home_editor::editor::home::transient;
        pub use semio_s_artifact_space_home_examples::editor::home::terminology;
        pub use semio_s_artifact_space_home_kinds_conformance::editor::home::commands;
        #[path = "."]
        pub mod modes {
            pub use semio_s_artifact_space_home_kinds_conformance::editor::home::modes::*;
            #[path = "."]
            pub mod explore {
                pub use semio_s_artifact_space_home_kinds_conformance::editor::home::modes::explore::*;
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_space_home_kinds_conformance::editor::home::modes::explore::windows::*;
                    #[path = "."]
                    pub mod main {
                        pub use semio_s_artifact_space_home_kinds_conformance::editor::home::modes::explore::windows::main::*;
                        #[cfg(test)]
                        #[path = "../../../✏️editor/🎭️modes/🔎️explore/🪟️windows/🏠️main/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                }
            }
        }
        #[cfg(test)]
        #[path = "../../../✏️editor/🧪️tests/🔬️unit/🦀️.rs"]
        pub mod tests;
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod home {
        #[path = "../../🦀️.rs"]
                mod component;
        pub use component::*;
        pub use semio_s_artifact_space_home_examples::viewer::home::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_space_home_examples::viewer::home::modes::view::*;
                pub use semio_s_artifact_space_home_examples::viewer::home::modes::view::windows;
            }
        }
    }
}
