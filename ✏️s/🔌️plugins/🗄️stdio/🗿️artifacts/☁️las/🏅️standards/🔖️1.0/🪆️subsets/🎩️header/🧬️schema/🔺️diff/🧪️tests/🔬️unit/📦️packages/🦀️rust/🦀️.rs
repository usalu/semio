//! 🔬️ tests of `semio-s-artifact-stdio-las` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as protocol;

#[cfg(test)]
pub use schema::snapshot::LasSnapshot;
#[cfg(test)]
pub mod schema {
    pub use super::standards::v1_0::subsets::any::schema::*;
}
pub use semio_s_artifact_stdio_las_snapshot::{STDIO_LAS_DOCUMENT_SCHEMA};

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
                    pub use semio_s_artifact_stdio_las_snapshot::standards::v1_0::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod diff {
                        pub use semio_s_artifact_stdio_las_snapshot::standards::v1_0::subsets::any::schema::diff::*;
                        #[cfg(test)]
                        #[path = "../../🦀️.rs"]
                        mod tests;
                    }
                    #[path = "."]
                    pub mod inferences {
                        pub use semio_s_artifact_stdio_las_schema::standards::v1_0::subsets::any::schema::inferences::*;
                        #[path = "."]
                        pub mod bounds {
                            pub use semio_s_artifact_stdio_las_schema::standards::v1_0::subsets::any::schema::inferences::bounds::*;
                            #[cfg(test)]
                            //#region 🧪️Tests
                            #[path = "../../../../../💡️inferences/📦bounds/🧪️tests/🔬️unit/🦀️.rs"]
                            mod tests;
                        }
                        #[cfg(test)]
                        //#region 🧪️Tests
                        #[path = "../../../../../💡️inferences/🧪️tests/🔬️unit/🦀️.rs"]
                        mod tests;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../../../../🚪️io/💾️binary/📸️snapshot/🦀️.rs"]
                        pub mod snapshot;
                    }
                    #[path = "../../../../../../🚪️io/🪶️sqlite/🦀️.rs"]
                    pub mod sqlite;
                }
            }
        }
    }
}
#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod demo {
        #[path = "../../../../../../📚️examples/🎬️demo/🦀️.rs"]
        mod component;
        pub use component::*;
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod las {
        #[path = "../../../../../../✏️editor/🧭️edit-rules/🦀️.rs"]
        pub mod edit_rules;
        #[path = "."]
        pub mod presence {
            #[path = "../../../../../../✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "../../../../../../✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod las {
        #[path = "."]
        pub mod presence {
            #[path = "../../../../../../👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "../../../../../../👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
    }
}
