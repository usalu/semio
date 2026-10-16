//! 📖️ Playbook artifact — the document entity this plugin's app edits.
//!
//! Step/block/expr records live in the shared kernel `playbook` crate; this plugin owns
//! `PlaybookSnapshot`, `PlaybookArtifact`, facet schemas, and app-facing wrappers.
//!
//! The steps live in ONE composed CHILD, `flow` (stdio's `s.stdio.semio@v1/flow`): one `FlowNode` per step, its
//! `blocks`/`description` JSON-encoded into params, the step order the chain of `sequence` edges. Composed content is edited
//! only on the child lane and read by composing parent + child on read (design §20.15 of ticket
//! 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING) — see `🔖️ContentBridge` and `🔖️ChildLane` below.

extern crate semio_framework as semio_framework;
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
pub use crate::standards::v1::subsets::any::schema::mutations::central_apply;

#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🦀️.rs"]
mod art_playbook_demo_tests;
extern crate semio_framework_schema as framework_schema;
use semio_framework_artifact_playbook_playbook as playbook;

use semio_s_artifact_stdio_semio_flow::schema::mutations::{SemioFlowMutation};

pub use semio_s_artifact_playbook_playbook_snapshot::{PLAYBOOK_ARTIFACT_SCHEMA_ID, PLAYBOOK_BUILTIN_KINDS, PLAYBOOK_DIALECT, PLAYBOOK_DOCUMENT_SCHEMA, PlaybookBlock, PlaybookBlockOption, PlaybookDiff, PlaybookExpr, PlaybookMutation, PlaybookSnapshot, PlaybookSpec, PlaybookStep, PlaybookStringList, PlaybookVectorField, artifact_kind, empty_playbook_snapshot};

//#region 🔖️SchemaExports
/// 🧬️ The schema root type of this artifact, exported next to the other schema types but hosted by no moved item: nothing in the crate names it through the root.
pub use crate::schema::PlaybookArtifact;
//#endregion 🔖️SchemaExports

pub use semio_s_artifact_playbook_playbook_snapshot::{PLAYBOOK_BLOCK_PARAM_PREFIX, PLAYBOOK_DESCRIPTION_PARAM, PLAYBOOK_FLOW_SLOT, PLAYBOOK_GENESIS_FLOW_ID, PLAYBOOK_NEXT_PORT, PLAYBOOK_PREV_PORT, PLAYBOOK_SEQUENCE_EDGE_KIND, PLAYBOOK_STEP_NODE_KIND, PLAYBOOK_STEP_SPACING, PlaybookFlowChild, flow_content_snapshot_from_steps, playbook_block_param_key, playbook_chain, playbook_flow_child, playbook_sequence_edge, playbook_step_from_node, playbook_step_node, playbook_step_order, steps_from_flow_content};

pub use semio_s_artifact_playbook_playbook_inferences::{PLAYBOOK_BLOCK_DUPLICATE, PLAYBOOK_BLOCK_MISSING, PLAYBOOK_FLOW_CONTENT, PLAYBOOK_FLOW_DIALECT, PLAYBOOK_FLOW_PROJECTION, PLAYBOOK_FLOW_UNAVAILABLE, PLAYBOOK_STEP_DUPLICATE, PLAYBOOK_STEP_MISSING, genesis_playbook_child_pack, playbook_child_restore_projection, playbook_composed_spec, playbook_fault, playbook_fault_notices, playbook_flow_content, playbook_genesis_flow};

pub use semio_s_artifact_playbook_playbook_inferences::{playbook_add_block_leaves, playbook_add_step_leaves, playbook_block_rows, playbook_chain_leaves, playbook_flow_emit, playbook_insert_block_leaf, playbook_move_block_leaves, playbook_move_step_leaves, playbook_remove_block_leaf, playbook_remove_block_leaves, playbook_remove_step_leaves, playbook_step_node_of, playbook_step_of};

pub use semio_s_artifact_playbook_playbook_v1::{PlaybookApplication, artifact, definition, pilot_languages};

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        pub use semio_s_artifact_playbook_playbook_v1::standards::v1::*;

        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                pub use semio_s_artifact_playbook_playbook_v1::standards::v1::subsets::any::*;

                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_playbook_playbook_schema::standards::v1::subsets::any::schema::*;

                    pub use semio_s_artifact_playbook_playbook_snapshot::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_playbook_playbook_inferences::standards::v1::subsets::any::schema::inferences;
                    pub use semio_s_artifact_playbook_playbook_snapshot::standards::v1::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_playbook_playbook_snapshot::standards::v1::subsets::any::schema::mutations::*;

                        pub use semio_s_artifact_playbook_playbook_snapshot::standards::v1::subsets::any::schema::mutations::central_apply;
                        #[path = "."]
                        pub mod change_title {
                            pub use semio_s_artifact_playbook_playbook_snapshot::standards::v1::subsets::any::schema::mutations::change_title::*;
                            pub use semio_s_artifact_playbook_playbook_snapshot::standards::v1::subsets::any::schema::mutations::change_title::diff;
                            pub use semio_s_artifact_playbook_playbook_snapshot::standards::v1::subsets::any::schema::mutations::change_title::inverse;

                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_playbook_playbook_inferences::standards::v1::subsets::any::io::*;
pub mod sqlite {
    pub use semio_s_artifact_playbook_playbook_examples::standards::v1::subsets::any::io::sqlite::*;
    pub use semio_s_artifact_playbook_playbook_snapshot::standards::v1::subsets::any::io::sqlite::snapshot;
}
pub mod text {
    pub use semio_s_artifact_playbook_playbook_inferences::standards::v1::subsets::any::io::text::*;
    pub use semio_s_artifact_playbook_playbook_snapshot::standards::v1::subsets::any::io::text::inferences;
}

                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_playbook_playbook_schema::standards::v1::subsets::any::io::import::deserializers::artifacts::txt;
                                pub use semio_s_artifact_playbook_playbook_schema::standards::v1::subsets::any::io::import::deserializers::artifacts::json;
                            }
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_playbook_playbook_schema::standards::v1::subsets::any::io::export::serializers::artifacts::txt;
                                pub use semio_s_artifact_playbook_playbook_schema::standards::v1::subsets::any::io::export::serializers::artifacts::json;
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

pub use semio_s_artifact_playbook_playbook_snapshot::op;

pub use semio_s_artifact_playbook_playbook_snapshot::diff;
pub mod mutations {
    pub use crate::standards::v1::subsets::any::schema::mutations::*;
}
pub use semio_s_artifact_playbook_playbook_snapshot::snapshot;

pub use semio_s_artifact_playbook_playbook_examples::examples;

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod playbook {
        pub use semio_s_artifact_playbook_playbook_inferences::editor::playbook::*;
pub use semio_s_artifact_playbook_playbook_examples::editor::playbook::demo_session;

        pub use semio_s_artifact_playbook_playbook_examples::editor::playbook::engine;

        #[path = "."]
        pub mod config {
            pub use semio_s_artifact_playbook_playbook_examples::editor::playbook::config::*;
pub mod io {
    pub use semio_s_artifact_playbook_playbook_snapshot::editor::playbook::config::io::*;
    pub mod binary {
        pub use semio_s_artifact_playbook_playbook_viewer::editor::playbook::config::io::binary::*;
        pub use semio_s_artifact_playbook_playbook_examples::editor::playbook::config::io::binary::mutations;
    }
    pub mod text {
        pub use semio_s_artifact_playbook_playbook_viewer::editor::playbook::config::io::text::*;
        pub use semio_s_artifact_playbook_playbook_examples::editor::playbook::config::io::text::mutations;
    }
}

            pub use semio_s_artifact_playbook_playbook_viewer::editor::playbook::config::schema;
        }

        pub use semio_s_artifact_playbook_playbook_viewer::editor::playbook::terminology;

        #[path = "."]
        pub mod commands {
            pub use semio_s_artifact_playbook_playbook_inferences::editor::playbook::commands::add_block;
            pub use semio_s_artifact_playbook_playbook_inferences::editor::playbook::commands::add_step;
            pub use semio_s_artifact_playbook_playbook_inferences::editor::playbook::commands::move_block;
            pub use semio_s_artifact_playbook_playbook_inferences::editor::playbook::commands::move_step;
            pub use semio_s_artifact_playbook_playbook_inferences::editor::playbook::commands::remove_block;
            pub use semio_s_artifact_playbook_playbook_inferences::editor::playbook::commands::remove_step;
            pub use semio_s_artifact_playbook_playbook_inferences::editor::playbook::commands::set_active_example;
            pub use semio_s_artifact_playbook_playbook_examples::editor::playbook::commands::set_contributions;
            pub use semio_s_artifact_playbook_playbook_examples::editor::playbook::commands::update_playbook;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod builder {
                pub use semio_s_artifact_playbook_playbook_inferences::editor::playbook::modes::builder::*;

                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_playbook_playbook_examples::editor::playbook::modes::builder::windows::builder;
                    pub use semio_s_artifact_playbook_playbook_viewer::editor::playbook::modes::builder::windows::changes;
                    pub use semio_s_artifact_playbook_playbook_viewer::editor::playbook::modes::builder::windows::steps;
                    pub use semio_s_artifact_playbook_playbook_viewer::editor::playbook::modes::builder::windows::activity;
                    pub use semio_s_artifact_playbook_playbook_viewer::editor::playbook::modes::builder::windows::source;
                    pub use semio_s_artifact_playbook_playbook_inferences::editor::playbook::modes::builder::windows::files;
                }
            }
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod playbook {
        pub use semio_s_artifact_playbook_playbook_inferences::viewer::playbook::*;

        pub use semio_s_artifact_playbook_playbook_viewer::viewer::playbook::presence;
        pub use semio_s_artifact_playbook_playbook_viewer::viewer::playbook::config;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_playbook_playbook_inferences::viewer::playbook::modes::view::*;

                pub use semio_s_artifact_playbook_playbook_viewer::viewer::playbook::modes::view::windows;
            }
        }
    }
}

pub use crate::standards::v1::subsets::any::io::{PlaybookBuilderConstruction, PlaybookParts, PlaybookAnalyzerAnalysis, PlaybookBuilderFacets, PlaybookBuilder, PlaybookAnalyzer, PlaybookComposer};

#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_0;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_1;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_2;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod hoisted_snapshot_3;
