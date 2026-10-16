//! 📜️ Imperative artifact — the document entity this plugin's app edits: a `Path` of control-flow
//! `Step`s (`state.set`/`log.print`/`control.if`/`control.while`/`math.add`/…), each addressable by a
//! [`PathRef`] for nested `control.*` bodies (drag-and-drop into blocks).
//!
//! `Path`/`Step` are NOT owned here — they live in the shared kernel crate `imperative_engine`
//! (`✏️s/🔨️modules/📜️imperative`, package `semio-s-kernel-imperative`; **do not confuse this kernel crate
//! with this plugin** — same "imperative" name, different crate, different location, a legitimate
//! dependency this plugin has always had). `Dictionary`/`Registry` come from the framework's
//! `neural_engine` kernel. This component re-exports the app-facing surface so every sibling taxonomy
//! node (`🔺️diff`, `🔧️op`, `🗣️dsl`, `📸️snapshot`, `📡️spr`, `⚙️engine`) names one artifact-owned symbol
//! instead of reaching into either kernel path directly.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🦀️.rs"]
mod art_procedure_demo_tests;
extern crate semio_framework_schema as framework_schema;

use std::collections::BTreeMap;

/// 🧩️ Built-in operators mounted by the procedure runtime.
pub use semio_s_artifact_imperative_procedure_snapshot::extensions;

pub use semio_s_artifact_imperative_procedure_snapshot::{Dictionary, PROCEDURE_DIALECT, PROCEDURE_DOCUMENT_SCHEMA, Path, PathRef, ProcedureDiff, ProcedureMutation, ProcedureSnapshot, Registry, Step, Value, artifact_kind, seed_dictionary};

pub use semio_s_artifact_imperative_procedure_snapshot::{PROCEDURE_BODY_EDGE, PROCEDURE_SEQUENCE_EDGE, ProcedureFlowChild, ProcedureTextChild, SemioFlowMutation, flow_content_snapshot_from_path, path_from_flow_content_snapshot, procedure_body_edge_id, procedure_flow_child_handle, procedure_sequence_edge_id, procedure_snapshot_naming, procedure_text_child_handle, seed_from_text_content_snapshot, text_content_snapshot_from_seed};

pub use semio_s_artifact_imperative_procedure_schema::{ProcedureScene, genesis_procedure_child_pack, procedure_child_emit, procedure_derivable_scene, procedure_edit_emit, procedure_flow_leaves, procedure_scene, procedure_scene_from_children};

pub use semio_s_artifact_imperative_procedure_editor::{declaration, definition, pilot_languages};

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

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
                    pub use semio_s_artifact_imperative_procedure_schema::standards::v1::subsets::any::schema::*;

                    pub use semio_s_artifact_imperative_procedure_snapshot::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_imperative_procedure_inferences::standards::v1::subsets::any::schema::inferences;
                    pub use semio_s_artifact_imperative_procedure_snapshot::standards::v1::subsets::any::schema::diff;
                    pub use semio_s_artifact_imperative_procedure_snapshot::standards::v1::subsets::any::schema::operations;
                    pub use semio_s_artifact_imperative_procedure_snapshot::standards::v1::subsets::any::schema::mutations;
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_imperative_procedure_inferences::standards::v1::subsets::any::io::*;
pub mod sqlite {
    pub use semio_s_artifact_imperative_procedure_viewer::standards::v1::subsets::any::io::sqlite::*;
    pub use semio_s_artifact_imperative_procedure_snapshot::standards::v1::subsets::any::io::sqlite::snapshot;
}
pub mod text {
    pub use semio_s_artifact_imperative_procedure_inferences::standards::v1::subsets::any::io::text::*;
    pub use semio_s_artifact_imperative_procedure_snapshot::standards::v1::subsets::any::io::text::inferences;
    pub use semio_s_artifact_imperative_procedure_snapshot::standards::v1::subsets::any::io::text::mutations;
}

                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_imperative_procedure_schema::standards::v1::subsets::any::io::import::deserializers::artifacts::txt;
                                pub use semio_s_artifact_imperative_procedure_schema::standards::v1::subsets::any::io::import::deserializers::artifacts::json;
                            }
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_imperative_procedure_viewer::standards::v1::subsets::any::io::export::serializers::artifacts::txt;
                                pub use semio_s_artifact_imperative_procedure_viewer::standards::v1::subsets::any::io::export::serializers::artifacts::json;
                            }
                        }
                    }
                }
            }
        }
    }
}

// ---- Shims: keep pre-migration module paths resolving for external callers ----
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}

pub use semio_s_artifact_imperative_procedure_snapshot::diff;
pub mod mutations {
    pub use crate::standards::v1::subsets::any::schema::mutations::*;
}
pub use semio_s_artifact_imperative_procedure_snapshot::snapshot;

pub use semio_s_artifact_imperative_procedure_schema::examples;

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod procedure {
        pub use semio_s_artifact_imperative_procedure_editor::editor::procedure::*;
pub use semio_s_artifact_imperative_procedure_viewer::editor::procedure::demo_session;
pub use semio_s_artifact_imperative_procedure_viewer::editor::procedure::schema;

        pub use semio_s_artifact_imperative_procedure_inferences::editor::procedure::engine;

        #[path = "."]
        pub mod config {
            pub use semio_s_artifact_imperative_procedure_inferences::editor::procedure::config::*;
pub mod io {
    pub use semio_s_artifact_imperative_procedure_viewer::editor::procedure::config::io::*;
    pub mod binary {
        pub use semio_s_artifact_imperative_procedure_snapshot::editor::procedure::config::io::binary::*;
        pub use semio_s_artifact_imperative_procedure_inferences::editor::procedure::config::io::binary::mutations;
    }
    pub mod text {
        pub use semio_s_artifact_imperative_procedure_snapshot::editor::procedure::config::io::text::*;
        pub use semio_s_artifact_imperative_procedure_inferences::editor::procedure::config::io::text::mutations;
    }
}

            pub use semio_s_artifact_imperative_procedure_viewer::editor::procedure::config::schema;
        }

        pub use semio_s_artifact_imperative_procedure_viewer::editor::procedure::terminology;
        pub use semio_s_artifact_imperative_procedure_viewer::editor::procedure::wasm;

        #[path = "."]
        pub mod commands {
            pub use semio_s_artifact_imperative_procedure_viewer_2::editor::procedure::commands::add_step;
            pub use semio_s_artifact_imperative_procedure_viewer_2::editor::procedure::commands::add_step_at;
            pub use semio_s_artifact_imperative_procedure_viewer_2::editor::procedure::commands::move_step;
            pub use semio_s_artifact_imperative_procedure_viewer_2::editor::procedure::commands::move_step_at;
            pub use semio_s_artifact_imperative_procedure_viewer_2::editor::procedure::commands::remove_step;
            pub use semio_s_artifact_imperative_procedure_viewer_2::editor::procedure::commands::remove_step_at;
            pub use semio_s_artifact_imperative_procedure_viewer_2::editor::procedure::commands::run;
            pub use semio_s_artifact_imperative_procedure_editor::editor::procedure::commands::set_active_example;
            pub use semio_s_artifact_imperative_procedure_viewer_2::editor::procedure::commands::set_contributions;
            pub use semio_s_artifact_imperative_procedure_viewer_2::editor::procedure::commands::set_step_params;
            pub use semio_s_artifact_imperative_procedure_viewer_2::editor::procedure::commands::set_step_params_at;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_imperative_procedure_viewer_2::editor::procedure::modes::edit::*;

                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_imperative_procedure_viewer::editor::procedure::modes::edit::windows::main;
                    pub use semio_s_artifact_imperative_procedure_viewer_2::editor::procedure::modes::edit::windows::script;
                }
            }
        }

        pub use semio_s_artifact_imperative_procedure_editor::editor::procedure::panels;
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod procedure {
        pub use semio_s_artifact_imperative_procedure_inferences::viewer::procedure::*;

        pub use semio_s_artifact_imperative_procedure_viewer::viewer::procedure::presence;
        pub use semio_s_artifact_imperative_procedure_viewer::viewer::procedure::config;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_imperative_procedure_inferences::viewer::procedure::modes::view::*;

                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_imperative_procedure_inferences::viewer::procedure::modes::view::windows::main;
                    pub use semio_s_artifact_imperative_procedure_inferences::viewer::procedure::modes::view::windows::script;
                }
            }
        }
    }
}

pub use semio_s_artifact_imperative_procedure_inferences::{procedure_child_restore_projection};

pub use crate::standards::v1::subsets::any::io::{ProcedureBuilderConstruction, ProcedureParts, ProcedureAnalyzerAnalysis, ProcedureBuilderFacets, ProcedureBuilder, ProcedureAnalyzer, ProcedureComposer};

#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_0;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod hoisted_snapshot_1;
