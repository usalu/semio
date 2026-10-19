//! 🗂️ Sourcing curation artifact — the document entities this plugin's curation app edits: a catalogue of
//! object kinds (parametric geometry + typology + availability) and a curated selection.

// 🔓️ R7 — `SourcingModule` (schema/🦀️.rs) declares `async fn` methods and is `#[dyn_enum]`-closed
// into `SourcingModules`; Send comes structurally from that concrete enum (R3), so this lint's suggested
// `-> impl Future + Send` fix is never taken, and the method is never made sync to silence it.
#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
#[path = "🌉️apply/🦀️.rs"]
pub mod central_apply;
#[cfg(test)]
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_schema as framework_schema;

use {semio_framework_plugin::MediaClass, semio_framework_plugin::MediaForm};

pub use crate::schema::mutations::SourcingMutation;

pub use crate::schema::diff::CurationDiff;

pub use semio_s_artifact_sourcing_curation_domain::{SOURCING_CURATION_SCHEMA};
pub use crate::schema::snapshot::CurationSnapshot;

pub use semio_s_artifact_sourcing_curation_inferences::{SOURCING_DIALECT};

pub use semio_s_artifact_sourcing_curation_domain::{GeometryRecipe};

pub use semio_s_artifact_sourcing_curation_domain::{ObjectKind};

pub use semio_s_artifact_sourcing_curation_examples::{CuratedItem, Filters, SortDirection, TableSort};

pub use semio_s_artifact_sourcing_curation_domain::{CATALOG_CHILD_SLOT, ObjectKindExtra, catalog_child_handle, catalog_snapshot_from_stock, curation_child_restore_projection, curation_snapshot_from_stock, genesis_catalog_pack, kit_type_from_object_kind, object_kind_extra_from_object_kind, object_kind_from_parts, stock_extra_from_stock, stock_from_catalog_and_extra, stock_of, validate_catalog_payload};

pub use semio_s_artifact_sourcing_curation_tests::{artifact_kind};

pub use semio_s_artifact_sourcing_curation_tests::{SourcingApplication, artifact, definition, language_spec, pilot_languages};

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
/// 🧪️ The mounted-app harness (`context`) of the sourcing editor: it drives the whole app, so it sits above every layer and the tests of commands, windows and panels reach it by this path.
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs"]
pub mod editor_unit_tests;
//#endregion 🧪️Tests

#[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1 {
                pub use semio_s_artifact_sourcing_curation_tests::standards::v1::*;

                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        pub use semio_s_artifact_sourcing_curation_tests::standards::v1::subsets::any::*;

                        #[path = "."]
                        pub mod schema {
                            pub use semio_s_artifact_sourcing_curation_schema::standards::v1::subsets::any::schema::*;
                            pub use semio_s_artifact_sourcing_curation_domain::standards::v1::subsets::any::schema::domain;

                            pub use semio_s_artifact_sourcing_curation_domain::standards::v1::subsets::any::schema::snapshot;
                            pub use semio_s_artifact_sourcing_curation_inferences::standards::v1::subsets::any::schema::inferences;
                            pub use semio_s_artifact_sourcing_curation_domain::standards::v1::subsets::any::schema::diff;
                            pub use semio_s_artifact_sourcing_curation_operations::standards::v1::subsets::any::schema::operations;
                            #[path = "."]
                            pub mod mutations {
                                pub use semio_s_artifact_sourcing_curation_operations::standards::v1::subsets::any::schema::mutations::*;

                                #[path = "."]
                                pub mod create_curated_item {
                                    pub use semio_s_artifact_sourcing_curation_operations::standards::v1::subsets::any::schema::mutations::create_curated_item::*;
                                    pub use semio_s_artifact_sourcing_curation_operations::standards::v1::subsets::any::schema::mutations::create_curated_item::diff;
                                    pub use semio_s_artifact_sourcing_curation_operations::standards::v1::subsets::any::schema::mutations::create_curated_item::inverse;

                                }
                                #[path = "."]
                                pub mod delete_curated_item {
                                    pub use semio_s_artifact_sourcing_curation_operations::standards::v1::subsets::any::schema::mutations::delete_curated_item::*;
                                    pub use semio_s_artifact_sourcing_curation_operations::standards::v1::subsets::any::schema::mutations::delete_curated_item::diff;
                                    pub use semio_s_artifact_sourcing_curation_operations::standards::v1::subsets::any::schema::mutations::delete_curated_item::inverse;

                                }
                                #[path = "."]
                                pub mod change_curated_item_count {
                                    pub use semio_s_artifact_sourcing_curation_operations::standards::v1::subsets::any::schema::mutations::change_curated_item_count::*;
                                    pub use semio_s_artifact_sourcing_curation_operations::standards::v1::subsets::any::schema::mutations::change_curated_item_count::diff;
                                    pub use semio_s_artifact_sourcing_curation_operations::standards::v1::subsets::any::schema::mutations::change_curated_item_count::inverse;

                                }
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            pub use semio_s_artifact_sourcing_curation_tests::standards::v1::subsets::any::io::*;
pub mod sqlite {
    pub use semio_s_artifact_sourcing_curation_examples::standards::v1::subsets::any::io::sqlite::*;
    pub use semio_s_artifact_sourcing_curation_domain::standards::v1::subsets::any::io::sqlite::snapshot;
}

                            pub use semio_s_artifact_sourcing_curation_domain::standards::v1::subsets::any::io::snapshot;
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        pub use semio_s_artifact_sourcing_curation_domain::standards::v1::subsets::any::io::import::deserializers::artifacts::zip;
                                        pub use semio_s_artifact_sourcing_curation_domain::standards::v1::subsets::any::io::import::deserializers::artifacts::txt;
                                        pub use semio_s_artifact_sourcing_curation_domain::standards::v1::subsets::any::io::import::deserializers::artifacts::json;
                                    }
                                }
                            }
                            #[path = "."]
                            pub mod export {
                                #[path = "."]
                                pub mod serializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        pub use semio_s_artifact_sourcing_curation_domain::standards::v1::subsets::any::io::export::serializers::artifacts::zip;
                                        pub use semio_s_artifact_sourcing_curation_domain::standards::v1::subsets::any::io::export::serializers::artifacts::txt;
                                        pub use semio_s_artifact_sourcing_curation_operations::standards::v1::subsets::any::io::export::serializers::artifacts::json;
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

        pub mod diff {
            pub use crate::standards::v1::subsets::any::schema::diff::*;
            pub mod schema {
                pub use crate::standards::v1::subsets::any::schema::diff::*;
            }

        }
        pub mod mutations {
            pub use crate::standards::v1::subsets::any::schema::mutations::*;
            pub mod schema {
                pub use crate::standards::v1::subsets::any::schema::mutations::*;
            }

        }
        pub mod snapshot {
            pub use crate::standards::v1::subsets::any::schema::snapshot::*;
            pub mod schema {
                pub use crate::standards::v1::subsets::any::schema::snapshot::*;
            }

        }

        pub use semio_s_artifact_sourcing_curation_examples::examples;

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod sourcing {
        pub use semio_s_artifact_sourcing_curation_tests::editor::sourcing::*;
pub use semio_s_artifact_sourcing_curation_examples::editor::sourcing::demo_session;

        pub use semio_s_artifact_sourcing_curation_operations::editor::sourcing::editor_vocabulary;

        #[path = "."]
        pub mod config {
            pub use semio_s_artifact_sourcing_curation_examples::editor::sourcing::config::*;

            pub use semio_s_artifact_sourcing_curation_examples::editor::sourcing::config::schema;
        }

        #[path = "."]
        pub mod presence {
            pub use semio_s_artifact_sourcing_curation_domain::editor::sourcing::presence::*;

            pub use semio_s_artifact_sourcing_curation_domain::editor::sourcing::presence::schema;
        }
        pub use semio_s_artifact_sourcing_curation_domain::editor::sourcing::terminology;

        #[path = "."]
        pub mod commands {
            pub use semio_s_artifact_sourcing_curation_inferences::editor::sourcing::commands::curation_add;
            pub use semio_s_artifact_sourcing_curation_inferences::editor::sourcing::commands::curation_remove;
            pub use semio_s_artifact_sourcing_curation_inferences::editor::sourcing::commands::curation_set_count;
            pub use semio_s_artifact_sourcing_curation_inferences::editor::sourcing::commands::drop_on_curated;
            pub use semio_s_artifact_sourcing_curation_inferences::editor::sourcing::commands::drop_on_pool;
            pub use semio_s_artifact_sourcing_curation_schema::editor::sourcing::commands::set_active_example;
            pub use semio_s_artifact_sourcing_curation_inferences::editor::sourcing::commands::load_document_json;
            pub use semio_s_artifact_sourcing_curation_inferences::editor::sourcing::commands::set_contributions;
            pub use semio_s_artifact_sourcing_curation_inferences::editor::sourcing::commands::set_filter_min_availability;
            pub use semio_s_artifact_sourcing_curation_inferences::editor::sourcing::commands::set_filter_module;
            pub use semio_s_artifact_sourcing_curation_inferences::editor::sourcing::commands::set_filter_query;
            pub use semio_s_artifact_sourcing_curation_inferences::editor::sourcing::commands::set_filter_typology;
            pub use semio_s_artifact_sourcing_curation_inferences::editor::sourcing::commands::sort_table;
            pub use semio_s_artifact_sourcing_curation_inferences::editor::sourcing::commands::stock_from_catalogue;
            pub use semio_s_artifact_sourcing_curation_inferences::editor::sourcing::commands::set_grid_instance_display;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_sourcing_curation_tests::editor::sourcing::modes::edit::*;

                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_sourcing_curation_operations::editor::sourcing::modes::edit::windows::curated;
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🔢️grid"]
                    pub mod grid {
                        pub use semio_s_artifact_sourcing_curation_inferences::editor::sourcing::modes::edit::windows::grid::*;

                        pub use semio_s_artifact_sourcing_curation_inferences::editor::sourcing::modes::edit::windows::grid::config;

                        pub use semio_s_artifact_sourcing_curation_inferences::editor::sourcing::modes::edit::windows::grid::instance_display;
                    }
                    pub use semio_s_artifact_sourcing_curation_inferences::editor::sourcing::modes::edit::windows::pool;
                    pub use semio_s_artifact_sourcing_curation_tests::editor::sourcing::modes::edit::windows::preview;
                }
            }
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod sourcing {
        pub use semio_s_artifact_sourcing_curation_inferences::viewer::sourcing::*;

        pub use semio_s_artifact_sourcing_curation_domain::viewer::sourcing::presence;
        pub use semio_s_artifact_sourcing_curation_domain::viewer::sourcing::config;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_sourcing_curation_operations::viewer::sourcing::modes::view::*;

                pub use semio_s_artifact_sourcing_curation_operations::viewer::sourcing::modes::view::windows;
            }
        }
    }
}

#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_0;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_1;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod hoisted_snapshot_2;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/➕️curation-add/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_3;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📥️load-document-json/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_4;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📉️set-filter-min-availability/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_5;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧺️curated/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_6;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🔢️grid/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_7;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🏊️pool/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_8;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_9;
