//! 🎪 `stdio.gif` artifact — stdio reference format.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub use semio_s_artifact_stdio_contract::{apply_mutation_checked, mutation_from_payload_json, mutation_inverse, mutation_payload_json, MutationRefusal};

pub(crate) use semio_s_artifact_stdio_contract::base64_standard;

pub use schema::diff::GifDiff;
pub use schema::mutations::GifMutation;
pub use schema::snapshot::GifSnapshot;
pub use schema::snapshot::STDIO_GIF89A_DOCUMENT_SCHEMA;
pub use schema::GifArtifact;
/// 🧾️ The DSL contract `GifSnapshot` implements, re-exported for clients that print or parse it.
pub use pack::snapshot::ArtifactDsl;

pub use semio_s_artifact_stdio_gif_inferences::{STDIO_GIF_DOCUMENT_SCHEMA};

pub use semio_s_artifact_stdio_gif_inferences::{GIF_ARTIFACT_SCHEMA_ID};

pub use semio_s_artifact_stdio_gif_editor::{ARTIFACT_DEFINITION_SCHEMA};

pub use semio_s_artifact_stdio_gif_editor::{definition};

pub use semio_s_artifact_stdio_gif_editor::{formats};

pub use semio_s_artifact_stdio_gif_inferences::{native_codecs};

pub use semio_s_artifact_stdio_gif_editor::{contribution};

pub use semio_s_artifact_stdio_gif_editor::{GIF_87A_DIALECT, GIF_89A_DIALECT};
#[cfg(test)]
pub use semio_s_artifact_stdio_gif_editor::{register_sqlite_test_declaration};

pub use semio_s_artifact_stdio_gif_editor::{artifact_kind, assembly, declaration};
//#region 🚪️DerivedIoRegistry
pub use semio_s_artifact_stdio_gif_schema_2::io_registry;
//#endregion 🚪️DerivedIoRegistry

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v87a {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_gif_schema::standards::v87a::subsets::any::schema::*;

                    pub use semio_s_artifact_stdio_gif_inferences::standards::v87a::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_stdio_gif_inferences::standards::v87a::subsets::any::schema::inferences;
                    pub use semio_s_artifact_stdio_gif_inferences::standards::v87a::subsets::any::schema::diff;
                    pub use semio_s_artifact_stdio_gif_mutations::standards::v87a::subsets::any::schema::mutations;
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_stdio_gif_schema_2::standards::v87a::subsets::any::io::*;
pub mod sqlite {
    pub use semio_s_artifact_stdio_gif_snapshot::standards::v87a::subsets::any::io::sqlite::*;
    pub use semio_s_artifact_stdio_gif_inferences::standards::v87a::subsets::any::io::sqlite::snapshot;
}

                    pub use semio_s_artifact_stdio_gif_schema_2::standards::v87a::subsets::any::io::import;
                    pub use semio_s_artifact_stdio_gif_editor::standards::v87a::subsets::any::io::export;
                }
            }
        }
    }

    #[path = "."]
    pub mod v89a {
        pub use semio_s_artifact_stdio_gif_snapshot::standards::v89a::migrations;
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_gif_schema_2::standards::v89a::subsets::any::schema::*;

                    pub use semio_s_artifact_stdio_gif_snapshot::standards::v89a::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_stdio_gif_schema::standards::v89a::subsets::any::schema::inferences;
                    pub use semio_s_artifact_stdio_gif_mutations::standards::v89a::subsets::any::schema::diff;
                    pub use semio_s_artifact_stdio_gif_mutations_2::standards::v89a::subsets::any::schema::mutations;
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_stdio_gif_schema_2::standards::v89a::subsets::any::io::*;
pub mod sqlite {
    pub use semio_s_artifact_stdio_gif_inferences::standards::v89a::subsets::any::io::sqlite::*;
    pub use semio_s_artifact_stdio_gif_snapshot::standards::v89a::subsets::any::io::sqlite::snapshot;
}

                    pub use semio_s_artifact_stdio_gif_editor::standards::v89a::subsets::any::io::import;
                    pub use semio_s_artifact_stdio_gif_editor::standards::v89a::subsets::any::io::export;
                }
            }
        }
    }
}

// ---- Crate-root module paths: the crate's public surface ----
// 🔀️ S-6 (`.claude/plans/the-current-schemas-are-scalable-journal.md`): 89a is the richer
// standard (frames/GCE/loop vs. 87a's single-image model) and is now canonical here; 87a
// stays reachable under its own explicit `standards::v87a::` path for callers that need it.
pub mod schema {
    pub use super::standards::v89a::subsets::any::schema::*;
}

#[path = "."]
pub mod examples {
    pub use semio_s_artifact_stdio_gif_snapshot::examples::demo;
    #[path = "."]
    pub mod dancing {
        pub use semio_s_artifact_stdio_gif_schema_2::examples::dancing::*;

    }
}

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod gif_87a {
        pub use semio_s_artifact_stdio_gif_editor::editor::gif_87a::*;

        pub use semio_s_artifact_stdio_gif_snapshot::editor::gif_87a::presence;
        pub use semio_s_artifact_stdio_gif_snapshot::editor::gif_87a::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_stdio_gif_editor::editor::gif_87a::modes::edit::*;

                pub use semio_s_artifact_stdio_gif_editor::editor::gif_87a::modes::edit::windows;
            }
        }
    }
    #[path = "."]
    pub mod gif_89a {
        pub use semio_s_artifact_stdio_gif_editor::editor::gif_89a::*;

        pub use semio_s_artifact_stdio_gif_snapshot::editor::gif_89a::presence;
        pub use semio_s_artifact_stdio_gif_snapshot::editor::gif_89a::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_stdio_gif_editor::editor::gif_89a::modes::edit::*;

                pub use semio_s_artifact_stdio_gif_editor::editor::gif_89a::modes::edit::windows;
            }
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod gif_87a {
        pub use semio_s_artifact_stdio_gif_editor::viewer::gif_87a::*;

        pub use semio_s_artifact_stdio_gif_snapshot::viewer::gif_87a::presence;
        pub use semio_s_artifact_stdio_gif_snapshot::viewer::gif_87a::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_stdio_gif_editor::viewer::gif_87a::modes::view::*;

                pub use semio_s_artifact_stdio_gif_editor::viewer::gif_87a::modes::view::windows;
            }
        }
    }
    #[path = "."]
    pub mod gif_89a {
        pub use semio_s_artifact_stdio_gif_editor::viewer::gif_89a::*;

        pub use semio_s_artifact_stdio_gif_snapshot::viewer::gif_89a::presence;
        pub use semio_s_artifact_stdio_gif_snapshot::viewer::gif_89a::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_stdio_gif_editor::viewer::gif_89a::modes::view::*;

                pub use semio_s_artifact_stdio_gif_editor::viewer::gif_89a::modes::view::windows;
            }
        }
    }
}

pub use crate::standards::v89a::subsets::any::io::{GifBuilderConstruction, GifParts, GifAnalyzerAnalysis, GifBuilderFacets, GifBuilder, GifAnalyzer, GifComposer};

#[cfg(test)]
#[path = "🏅️standards/7️⃣87a/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod hoisted_snapshot_0;
#[cfg(test)]
#[path = "🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod hoisted_snapshot_1;
