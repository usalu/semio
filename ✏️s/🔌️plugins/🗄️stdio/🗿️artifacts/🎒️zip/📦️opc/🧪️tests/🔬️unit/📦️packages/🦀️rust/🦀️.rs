//! 🔬️ tests of `semio-s-artifact-stdio-zip` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

#[cfg(test)]
pub use schema::snapshot::ZipSnapshot;
#[cfg(test)]
pub mod schema {
    pub use super::standards::v2_0::subsets::base::schema::*;
}
pub use semio_s_artifact_stdio_zip_inferences::{STDIO_ZIP_DOCUMENT_SCHEMA};

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v2_0 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod base {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_zip_inferences::standards::v2_0::subsets::base::schema::snapshot;
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_stdio_zip_schema::standards::v2_0::subsets::base::io::*;
                }
            }
        }
    }
}
#[path = "."]
pub mod opc {
    pub use semio_s_artifact_stdio_zip_opc::opc::*;
    pub use semio_s_artifact_stdio_zip_opc::opc::*;
    #[cfg(test)]
    #[path = "../../🦀️.rs"]
    pub mod tests;
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod zip {
        #[path = "."]
        pub mod base {
            #[path = "."]
            pub mod modes {
                #[path = "."]
                pub mod edit {
                    pub use semio_s_artifact_stdio_zip_editor::editor::zip::base::modes::edit::*;
                    #[cfg(test)]
                    #[path = "../../../../../🏅️standards/🔖️2.0/🪆️subsets/🧱️base/✏️editor/🎭️modes/✏️edit/🧪️tests/🔬️unit/🦀️.rs"]
                    pub mod tests;
                    #[path = "."]
                    pub mod windows {
                        pub use semio_s_artifact_stdio_zip_editor::editor::zip::base::modes::edit::windows::*;
                        #[path = "."]
                        pub mod main {
                            pub use semio_s_artifact_stdio_zip_editor::editor::zip::base::modes::edit::windows::main::*;
                            #[cfg(test)]
                            #[path = "../../../../../🏅️standards/🔖️2.0/🪆️subsets/🧱️base/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                    }
                }
            }
        }
        #[path = "."]
        pub mod iso21320 {
            #[path = "."]
            pub mod modes {
                #[path = "."]
                pub mod edit {
                    pub use semio_s_artifact_stdio_zip_editor::editor::zip::iso21320::modes::edit::*;
                    #[cfg(test)]
                    #[path = "../../../../../🏅️standards/🔖️2.0/🪆️subsets/🌐️iso21320/✏️editor/🎭️modes/✏️edit/🧪️tests/🔬️unit/🦀️.rs"]
                    pub mod tests;
                    #[path = "."]
                    pub mod windows {
                        pub use semio_s_artifact_stdio_zip_editor::editor::zip::iso21320::modes::edit::windows::*;
                        #[path = "."]
                        pub mod main {
                            pub use semio_s_artifact_stdio_zip_editor::editor::zip::iso21320::modes::edit::windows::main::*;
                            #[cfg(test)]
                            #[path = "../../../../../🏅️standards/🔖️2.0/🪆️subsets/🌐️iso21320/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                    }
                }
            }
        }
    }
}
