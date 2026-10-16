//! 🧬️ schema of `semio-s-artifact-stdio-ply` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub use schema::snapshot::PlySnapshot;
#[cfg(test)]
pub use semio_s_artifact_stdio_contract::apply_mutation;
pub mod schema {
    pub use super::standards::v1_0::subsets::any::schema::*;
}
pub use semio_s_artifact_stdio_ply_inferences::{STDIO_PLY_DOCUMENT_SCHEMA};

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1_0 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    #[path = "../../🦀️.rs"]
                    mod component;
                    pub use component::*;
                    pub use semio_s_artifact_stdio_ply_inferences::standards::v1_0::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_stdio_ply_inferences::standards::v1_0::subsets::any::schema::diff;
                    pub use semio_s_artifact_stdio_ply_mutations::standards::v1_0::subsets::any::schema::mutations;
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../🚪️io/💾️binary/📸️snapshot/🦀️.rs"]
                        pub mod snapshot;
                        pub use semio_s_artifact_stdio_ply_inferences::standards::v1_0::subsets::any::io::binary::diff;
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod ply {
        pub use semio_s_artifact_stdio_ply_mutations::editor::ply::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_stdio_ply_inferences::editor::ply::modes::edit::windows::*;
                    #[path = "."]
                    pub mod main {
                        pub use semio_s_artifact_stdio_ply_inferences::editor::ply::modes::edit::windows::main::*;
                        #[cfg(test)]
                        #[path = "../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🧪️tests/🔬️unit/🦀️.rs"]
                        mod tests;
                    }
                }
            }
        }
        #[cfg(test)]
        #[path = "../../../✏️editor/🧪️tests/🔬️unit/🦀️.rs"]
        mod tests;
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod ply {
        #[path = "../../../👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../../👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_stdio_ply_inferences::viewer::ply::modes::view::windows::*;
                    #[path = "."]
                    pub mod main {
                        pub use semio_s_artifact_stdio_ply_inferences::viewer::ply::modes::view::windows::main::*;
                        #[cfg(test)]
                        #[path = "../../../👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🧪️tests/🔬️unit/🦀️.rs"]
                        mod tests;
                    }
                }
            }
        }
    }
}
