//! 🧩️ Puzzle 2d artifact — the `board.ports.directed.v1` document schema: the `Puzzle2dSnapshot`
//! (schema/camera/nodes/edges/meta), its node/handle/edge/kind-compatibility records, and the
//! `artifact_kind()` spec the play app's manifest binds. Sibling nodes: `🔺️diff`, `🔧️op`, `🗣️dsl`,
//! `🎒️pack`, `📡️spr`. No `⚙️engine` sibling anymore (ticket
//! 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W1e): an artifact is a `🧬️schema` plus a `🚪️io`
//! system, never an engine — the old `⚙️engine`'s pure/document-only pieces moved into `🧬️schema`
//! and `🚪️io` (this file's own `declaration()` and I/O registry), and its genuinely
//! stateful `BoardHost` facade moved to `🎛️apps/◻️2d/⚙️engine` (since relocated again, ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET, to `✏️editor/⚙️engine` under this artifact's own
//! `✏️editor` surface).

#![allow(clippy::result_large_err)]

use semio_framework_test_fixtures::fixture_text;
use semio_s_artifact_puzzle_2d_unit_tests::editor::puzzle2d::snapshot::PUZZLE2D_RETAINED_TOOL_IDS;

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_value_derive as value_derive;

pub use semio_s_plugin_puzzle_board_scene as board_scene;

pub use semio_s_plugin_puzzle_jobs as puzzle_job;

pub use semio_s_plugin_puzzle_retained as retained_command;

#[cfg(test)]
fn retained_command_test_catalog() -> (&'static str, &'static str, &'static [&'static str], &'static str) {
    ("puzzle2d", "board.ports.directed.v1", semio_s_artifact_puzzle_2d_mut_any_10::editor::puzzle2d::snapshot::PUZZLE2D_RETAINED_TOOL_IDS, fixture_text!("@fixtures/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🗄️retained-jobs/🔣️.json"))
}

//#region 🔖️Document

//#endregion 🔖️Document

//#region 🔖️Snapshot
//#endregion 🔖️Snapshot

//#region 🔖️Dialect

//#endregion 🔖️Dialect

//#region 🔖️ArtifactKind

//#endregion 🔖️ArtifactKind

pub use semio_s_artifact_puzzle_2d_v1::{ArtifactApps, artifact, definition, pilot_languages};

pub use crate::editor::puzzle2d::snapshot::Puzzle2dPlaySnapshot;

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        pub use semio_s_artifact_puzzle_2d_v1::standards::v1::*;

        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                pub use semio_s_artifact_puzzle_2d_v1::standards::v1::subsets::any::*;

                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_puzzle_2d_schema::standards::v1::subsets::any::schema::*;

                    #[cfg(any(test, feature = "testkit"))]
pub use semio_s_artifact_puzzle_2d_records::standards::v1::subsets::any::schema::testkit;
                    pub use semio_s_artifact_puzzle_2d_snapshot::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_puzzle_2d_inferences::standards::v1::subsets::any::schema::inferences;
                    pub use semio_s_artifact_puzzle_2d_schema::standards::v1::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod preparation {
                        pub use semio_s_artifact_puzzle_2d_mutations::standards::v1::subsets::any::schema::preparation::literal_id_inverse;
                        pub mod create_node_preparation {
    pub use semio_s_artifact_puzzle_2d_mutations::standards::v1::subsets::any::schema::preparation::create_node_preparation::*;
    pub use semio_s_artifact_puzzle_2d_connect_handles_preparation::standards::v1::subsets::any::schema::preparation::create_node_preparation::inverse;
}
                        pub use semio_s_artifact_puzzle_2d_mutations::standards::v1::subsets::any::schema::preparation::delete_node_preparation;
                        pub use semio_s_artifact_puzzle_2d_mut_any_5::standards::v1::subsets::any::schema::preparation::move_node_preparation;
                        pub mod connect_handles_preparation {
    pub use semio_s_artifact_puzzle_2d_connect_handles_preparation::standards::v1::subsets::any::schema::preparation::connect_handles_preparation::*;
    pub use semio_s_artifact_puzzle_2d_mut_any_10::standards::v1::subsets::any::schema::preparation::connect_handles_preparation::inverse;
}
                        pub use semio_s_artifact_puzzle_2d_mut_any_5::standards::v1::subsets::any::schema::preparation::flag_preparation;
                        pub use semio_s_artifact_puzzle_2d_mut_any_5::standards::v1::subsets::any::schema::preparation::text_preparation;
                        pub use semio_s_artifact_puzzle_2d_text_inverse::standards::v1::subsets::any::schema::preparation::text_inverse;
                        pub use semio_s_artifact_puzzle_2d_records::standards::v1::subsets::any::schema::preparation::native_preparation_child;
                    }
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_puzzle_2d_mutations::standards::v1::subsets::any::schema::mutations::*;

                        #[path = "."]
                        pub mod create_node {
                            pub use semio_s_artifact_puzzle_2d_mutations::standards::v1::subsets::any::schema::mutations::create_node::*;
                            pub use semio_s_artifact_puzzle_2d_mutations::standards::v1::subsets::any::schema::mutations::create_node::diff;
                            pub use semio_s_artifact_puzzle_2d_mutations::standards::v1::subsets::any::schema::mutations::create_node::inverse;

                        }
                        #[path = "."]
                        pub mod delete_node {
                            pub use semio_s_artifact_puzzle_2d_mutations::standards::v1::subsets::any::schema::mutations::delete_node::*;
                            pub use semio_s_artifact_puzzle_2d_mutations::standards::v1::subsets::any::schema::mutations::delete_node::diff;
                            pub use semio_s_artifact_puzzle_2d_mutations::standards::v1::subsets::any::schema::mutations::delete_node::inverse;

                        }
                        #[path = "."]
                        pub mod move_node {
                            pub use semio_s_artifact_puzzle_2d_mut_any::standards::v1::subsets::any::schema::mutations::move_node::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any::standards::v1::subsets::any::schema::mutations::move_node::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any::standards::v1::subsets::any::schema::mutations::move_node::inverse;

                        }
                        #[path = "."]
                        pub mod replace_node_geometry {
                            pub use semio_s_artifact_puzzle_2d_mut_any::standards::v1::subsets::any::schema::mutations::replace_node_geometry::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any::standards::v1::subsets::any::schema::mutations::replace_node_geometry::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any::standards::v1::subsets::any::schema::mutations::replace_node_geometry::inverse;

                        }
                        #[path = "."]
                        pub mod change_node_kind {
                            pub use semio_s_artifact_puzzle_2d_mut_any::standards::v1::subsets::any::schema::mutations::change_node_kind::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any::standards::v1::subsets::any::schema::mutations::change_node_kind::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any::standards::v1::subsets::any::schema::mutations::change_node_kind::inverse;

                        }
                        #[path = "."]
                        pub mod edit_node_text {
                            pub use semio_s_artifact_puzzle_2d_mut_any::standards::v1::subsets::any::schema::mutations::edit_node_text::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any::standards::v1::subsets::any::schema::mutations::edit_node_text::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any::standards::v1::subsets::any::schema::mutations::edit_node_text::inverse;

                        }
                        #[path = "."]
                        pub mod change_node_icon {
                            pub use semio_s_artifact_puzzle_2d_mut_any::standards::v1::subsets::any::schema::mutations::change_node_icon::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any::standards::v1::subsets::any::schema::mutations::change_node_icon::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any::standards::v1::subsets::any::schema::mutations::change_node_icon::inverse;

                        }
                        #[path = "."]
                        pub mod scale_node {
                            pub use semio_s_artifact_puzzle_2d_mut_any::standards::v1::subsets::any::schema::mutations::scale_node::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any::standards::v1::subsets::any::schema::mutations::scale_node::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any::standards::v1::subsets::any::schema::mutations::scale_node::inverse;

                        }
                        #[path = "."]
                        pub mod change_node_visible {
                            pub use semio_s_artifact_puzzle_2d_mut_any::standards::v1::subsets::any::schema::mutations::change_node_visible::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any::standards::v1::subsets::any::schema::mutations::change_node_visible::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any::standards::v1::subsets::any::schema::mutations::change_node_visible::inverse;

                        }
                        #[path = "."]
                        pub mod change_node_locked {
                            pub use semio_s_artifact_puzzle_2d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_node_locked::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_node_locked::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_node_locked::inverse;

                        }
                        #[path = "."]
                        pub mod change_node_root {
                            pub use semio_s_artifact_puzzle_2d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_node_root::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_node_root::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_node_root::inverse;

                        }
                        #[path = "."]
                        pub mod change_node_anchor {
                            pub use semio_s_artifact_puzzle_2d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_node_anchor::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_node_anchor::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_node_anchor::inverse;

                        }
                        #[path = "."]
                        pub mod add_node_handle {
                            pub use semio_s_artifact_puzzle_2d_mutations::standards::v1::subsets::any::schema::mutations::add_node_handle::*;
                            pub use semio_s_artifact_puzzle_2d_mutations::standards::v1::subsets::any::schema::mutations::add_node_handle::diff;
                            pub use semio_s_artifact_puzzle_2d_mutations::standards::v1::subsets::any::schema::mutations::add_node_handle::inverse;

                        }
                        #[path = "."]
                        pub mod remove_node_handle {
                            pub use semio_s_artifact_puzzle_2d_mutations::standards::v1::subsets::any::schema::mutations::remove_node_handle::*;
                            pub use semio_s_artifact_puzzle_2d_mutations::standards::v1::subsets::any::schema::mutations::remove_node_handle::diff;
                            pub use semio_s_artifact_puzzle_2d_mutations::standards::v1::subsets::any::schema::mutations::remove_node_handle::inverse;

                        }
                        #[path = "."]
                        pub mod replace_node_handle {
                            pub use semio_s_artifact_puzzle_2d_mut_any_2::standards::v1::subsets::any::schema::mutations::replace_node_handle::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any_2::standards::v1::subsets::any::schema::mutations::replace_node_handle::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any_2::standards::v1::subsets::any::schema::mutations::replace_node_handle::inverse;

                        }
                        #[path = "."]
                        pub mod connect_handles {
                            pub use semio_s_artifact_puzzle_2d_mut_any_4::standards::v1::subsets::any::schema::mutations::connect_handles::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any_4::standards::v1::subsets::any::schema::mutations::connect_handles::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any_4::standards::v1::subsets::any::schema::mutations::connect_handles::inverse;

                        }
                        #[path = "."]
                        pub mod disconnect_handles {
                            pub use semio_s_artifact_puzzle_2d_mut_any_4::standards::v1::subsets::any::schema::mutations::disconnect_handles::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any_4::standards::v1::subsets::any::schema::mutations::disconnect_handles::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any_4::standards::v1::subsets::any::schema::mutations::disconnect_handles::inverse;

                        }
                        #[path = "."]
                        pub mod replace_edge_geometry {
                            pub use semio_s_artifact_puzzle_2d_mut_any_2::standards::v1::subsets::any::schema::mutations::replace_edge_geometry::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any_2::standards::v1::subsets::any::schema::mutations::replace_edge_geometry::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any_2::standards::v1::subsets::any::schema::mutations::replace_edge_geometry::inverse;

                        }
                        #[path = "."]
                        pub mod change_edge_kind {
                            pub use semio_s_artifact_puzzle_2d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_edge_kind::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_edge_kind::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_edge_kind::inverse;

                        }
                        #[path = "."]
                        pub mod change_edge_tips {
                            pub use semio_s_artifact_puzzle_2d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_edge_tips::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_edge_tips::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_edge_tips::inverse;

                        }
                        #[path = "."]
                        pub mod change_edge_visible {
                            pub use semio_s_artifact_puzzle_2d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_edge_visible::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_edge_visible::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_edge_visible::inverse;

                        }
                        #[path = "."]
                        pub mod change_edge_locked {
                            pub use semio_s_artifact_puzzle_2d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_edge_locked::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_edge_locked::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_edge_locked::inverse;

                        }
                        #[path = "."]
                        pub mod change_manifest_id {
                            pub use semio_s_artifact_puzzle_2d_mut_any_3::standards::v1::subsets::any::schema::mutations::change_manifest_id::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any_3::standards::v1::subsets::any::schema::mutations::change_manifest_id::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any_3::standards::v1::subsets::any::schema::mutations::change_manifest_id::inverse;

                        }
                        #[path = "."]
                        pub mod connect_kind_compatibility {
                            pub use semio_s_artifact_puzzle_2d_mut_any_3::standards::v1::subsets::any::schema::mutations::connect_kind_compatibility::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any_3::standards::v1::subsets::any::schema::mutations::connect_kind_compatibility::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any_3::standards::v1::subsets::any::schema::mutations::connect_kind_compatibility::inverse;

                        }
                        #[path = "."]
                        pub mod disconnect_kind_compatibility {
                            pub use semio_s_artifact_puzzle_2d_mut_any_3::standards::v1::subsets::any::schema::mutations::disconnect_kind_compatibility::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any_3::standards::v1::subsets::any::schema::mutations::disconnect_kind_compatibility::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any_3::standards::v1::subsets::any::schema::mutations::disconnect_kind_compatibility::inverse;

                        }
                        #[path = "."]
                        pub mod replace_kind_catalogs {
                            pub use semio_s_artifact_puzzle_2d_mut_any_3::standards::v1::subsets::any::schema::mutations::replace_kind_catalogs::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any_3::standards::v1::subsets::any::schema::mutations::replace_kind_catalogs::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any_3::standards::v1::subsets::any::schema::mutations::replace_kind_catalogs::inverse;

                        }
                        #[path = "."]
                        pub mod create_target_region {
                            pub use semio_s_artifact_puzzle_2d_mut_any_3::standards::v1::subsets::any::schema::mutations::create_target_region::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any_3::standards::v1::subsets::any::schema::mutations::create_target_region::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any_3::standards::v1::subsets::any::schema::mutations::create_target_region::inverse;

                        }
                        #[path = "."]
                        pub mod delete_target_region {
                            pub use semio_s_artifact_puzzle_2d_mut_any_3::standards::v1::subsets::any::schema::mutations::delete_target_region::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any_3::standards::v1::subsets::any::schema::mutations::delete_target_region::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any_3::standards::v1::subsets::any::schema::mutations::delete_target_region::inverse;

                        }
                        #[path = "."]
                        pub mod move_target_region {
                            pub use semio_s_artifact_puzzle_2d_mut_any_3::standards::v1::subsets::any::schema::mutations::move_target_region::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any_3::standards::v1::subsets::any::schema::mutations::move_target_region::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any_3::standards::v1::subsets::any::schema::mutations::move_target_region::inverse;

                        }
                        #[path = "."]
                        pub mod resize_target_region {
                            pub use semio_s_artifact_puzzle_2d_mut_any_3::standards::v1::subsets::any::schema::mutations::resize_target_region::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any_3::standards::v1::subsets::any::schema::mutations::resize_target_region::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any_3::standards::v1::subsets::any::schema::mutations::resize_target_region::inverse;

                        }
                        #[path = "."]
                        pub mod edit_target_region_label {
                            pub use semio_s_artifact_puzzle_2d_mut_any_3::standards::v1::subsets::any::schema::mutations::edit_target_region_label::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any_3::standards::v1::subsets::any::schema::mutations::edit_target_region_label::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any_3::standards::v1::subsets::any::schema::mutations::edit_target_region_label::inverse;

                        }
                        #[path = "."]
                        pub mod change_target_region_hidden {
                            pub use semio_s_artifact_puzzle_2d_mut_any_4::standards::v1::subsets::any::schema::mutations::change_target_region_hidden::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any_4::standards::v1::subsets::any::schema::mutations::change_target_region_hidden::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any_4::standards::v1::subsets::any::schema::mutations::change_target_region_hidden::inverse;

                        }
                        #[path = "."]
                        pub mod change_target_region_locked {
                            pub use semio_s_artifact_puzzle_2d_mut_any_4::standards::v1::subsets::any::schema::mutations::change_target_region_locked::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any_4::standards::v1::subsets::any::schema::mutations::change_target_region_locked::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any_4::standards::v1::subsets::any::schema::mutations::change_target_region_locked::inverse;

                        }
                        #[path = "."]
                        pub mod drag_selection {
                            pub use semio_s_artifact_puzzle_2d_mut_any_4::standards::v1::subsets::any::schema::mutations::drag_selection::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any_4::standards::v1::subsets::any::schema::mutations::drag_selection::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any_4::standards::v1::subsets::any::schema::mutations::drag_selection::inverse;

                        }
                        #[path = "."]
                        pub mod rotate_selection {
                            pub use semio_s_artifact_puzzle_2d_mut_any_4::standards::v1::subsets::any::schema::mutations::rotate_selection::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any_4::standards::v1::subsets::any::schema::mutations::rotate_selection::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any_4::standards::v1::subsets::any::schema::mutations::rotate_selection::inverse;

                        }
                        #[path = "."]
                        pub mod scale_selection {
                            pub use semio_s_artifact_puzzle_2d_mut_any_4::standards::v1::subsets::any::schema::mutations::scale_selection::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any_4::standards::v1::subsets::any::schema::mutations::scale_selection::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any_4::standards::v1::subsets::any::schema::mutations::scale_selection::inverse;

                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_puzzle_2d_text_inverse::standards::v1::subsets::any::io::*;

                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_puzzle_2d_snapshot::standards::v1::subsets::any::io::import::deserializers::artifacts::txt;
                                pub use semio_s_artifact_puzzle_2d_inferences::standards::v1::subsets::any::io::import::deserializers::artifacts::json;
                            }
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_puzzle_2d_text_inverse::standards::v1::subsets::any::io::export::serializers::artifacts::svg;
                                pub use semio_s_artifact_puzzle_2d_snapshot::standards::v1::subsets::any::io::export::serializers::artifacts::txt;
                                pub use semio_s_artifact_puzzle_2d_text_inverse::standards::v1::subsets::any::io::export::serializers::artifacts::pdf;
                                pub use semio_s_artifact_puzzle_2d_text_inverse::standards::v1::subsets::any::io::export::serializers::artifacts::png;
                                pub use semio_s_artifact_puzzle_2d_inferences::standards::v1::subsets::any::io::export::serializers::artifacts::json;
                                pub use semio_s_artifact_puzzle_2d_text_inverse::standards::v1::subsets::any::io::export::serializers::artifacts::dwg;
                                pub use semio_s_artifact_puzzle_2d_text_inverse::standards::v1::subsets::any::io::export::serializers::artifacts::dxf;
                            }
                        }
                    }
                }
            }
        }
    }
}

pub use crate::standards::v1::subsets::any::schema::diff::Puzzle2dDiff;
pub use crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation;

pub use crate::standards::v1::subsets::any::schema::snapshot::Puzzle2dSnapshot;

#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod puzzle2d {
        pub use semio_s_artifact_puzzle_2d_inferences::examples::puzzle2d::concrete_forest;

        pub use semio_s_artifact_puzzle_2d_inferences::examples::puzzle2d::nakagin_capsule_tower;

    }
}

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod puzzle2d {
        pub use semio_s_artifact_puzzle_2d_v1::editor::puzzle2d::*;

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo_session {
                pub use semio_s_artifact_puzzle_2d_records::editor::puzzle2d::examples::demo_session::*;

            }
        }

        #[path = "."]
        pub mod config {
            pub use semio_s_artifact_puzzle_2d_records::editor::puzzle2d::config::*;

            pub use semio_s_artifact_puzzle_2d_records::editor::puzzle2d::config::schema;
        }

        pub mod window {
    pub use semio_s_artifact_puzzle_2d_snapshot::editor::puzzle2d::window::*;
    pub use semio_s_artifact_puzzle_2d_records::editor::puzzle2d::window::schema;
}

        #[path = "."]
        pub mod presence {
            pub use semio_s_artifact_puzzle_2d_records::editor::puzzle2d::presence::*;

            pub use semio_s_artifact_puzzle_2d_records::editor::puzzle2d::presence::schema;
        }
        pub use semio_s_artifact_puzzle_2d_records::editor::puzzle2d::terminology;
        pub use semio_s_artifact_puzzle_2d_mut_any::editor::puzzle2d::wasm;

        #[path = "."]
        pub mod engine {
            pub use semio_s_artifact_puzzle_2d_engine::editor::puzzle2d::engine::*;

            pub use semio_s_artifact_puzzle_2d_board_host::editor::puzzle2d::engine::board_host;

            pub use semio_s_artifact_puzzle_2d_inferences::editor::puzzle2d::engine::icons;
            pub use semio_s_artifact_puzzle_2d_engine::editor::puzzle2d::engine::layout;

        }

        pub use semio_s_artifact_puzzle_2d_tests::editor::puzzle2d::precompute;

        #[path = "."]
        pub mod commands {
            pub use semio_s_artifact_puzzle_2d_interaction::editor::puzzle2d::commands::add_node;
            pub use semio_s_artifact_puzzle_2d_retained::editor::puzzle2d::commands::apply_board_events;
            pub use semio_s_artifact_puzzle_2d_interaction::editor::puzzle2d::commands::close_handle_suggestions;
            pub use semio_s_artifact_puzzle_2d_retained::editor::puzzle2d::commands::accept_suggestion;
            pub use semio_s_artifact_puzzle_2d_interaction::editor::puzzle2d::commands::target_brush_suggestions;
            pub use semio_s_artifact_puzzle_2d_interaction::editor::puzzle2d::commands::cycle_candidate;
            pub use semio_s_artifact_puzzle_2d_interaction::editor::puzzle2d::commands::create_edge;
            pub use semio_s_artifact_puzzle_2d_interaction::editor::puzzle2d::commands::delete_edge;
            pub use semio_s_artifact_puzzle_2d_interaction::editor::puzzle2d::commands::proximity_connect;
            pub use semio_s_artifact_puzzle_2d_interaction::editor::puzzle2d::commands::set_proximity_radius;
            pub use semio_s_artifact_puzzle_2d_interaction::editor::puzzle2d::commands::delete_selection;
            pub use semio_s_artifact_puzzle_2d_interaction::editor::puzzle2d::commands::duplicate_selection;
            pub use semio_s_artifact_puzzle_2d_interaction::editor::puzzle2d::commands::engagement_abort;
            pub use semio_s_artifact_puzzle_2d_interaction::editor::puzzle2d::commands::engagement_control_select;
            pub use semio_s_artifact_puzzle_2d_interaction::editor::puzzle2d::commands::engagement_input;
            pub use semio_s_artifact_puzzle_2d_retained::editor::puzzle2d::commands::engagement_submit;
            pub use semio_s_artifact_puzzle_2d_interaction::editor::puzzle2d::commands::focus_selection;
            pub use semio_s_artifact_puzzle_2d_interaction::editor::puzzle2d::commands::force_layout;
            pub use semio_s_artifact_puzzle_2d_interaction::editor::puzzle2d::commands::lod_scale_json;
            pub use semio_s_artifact_puzzle_2d_interaction::editor::puzzle2d::commands::open_handle_suggestions;
            pub use semio_s_artifact_puzzle_2d_interaction::editor::puzzle2d::commands::hover_suggestion;
            pub use semio_s_artifact_puzzle_2d_interaction::editor::puzzle2d::commands::patch_inspector;
            pub use semio_s_artifact_puzzle_2d_interaction::editor::puzzle2d::commands::select_same_kind;
            pub use semio_s_artifact_puzzle_2d_mut_any_10::editor::puzzle2d::commands::set_active_example;
            pub use semio_s_artifact_puzzle_2d_interaction::editor::puzzle2d::commands::set_brush_kind_weights;
            pub use semio_s_artifact_puzzle_2d_interaction::editor::puzzle2d::commands::set_brush_node_size;
            pub use semio_s_artifact_puzzle_2d_interaction::editor::puzzle2d::commands::set_camera;
            pub use semio_s_artifact_puzzle_2d_interaction::editor::puzzle2d::commands::set_fill_count;
            pub use semio_s_artifact_puzzle_2d_mut_any_8::editor::puzzle2d::commands::set_grid_factor;
            pub use semio_s_artifact_puzzle_2d_mut_any_8::editor::puzzle2d::commands::set_grid_snap_enabled;
            pub use semio_s_artifact_puzzle_2d_mut_any_8::editor::puzzle2d::commands::set_grid_visible;
            pub use semio_s_artifact_puzzle_2d_mut_any_8::editor::puzzle2d::commands::set_selectable_kind;
            pub use semio_s_artifact_puzzle_2d_mut_any_7::editor::puzzle2d::commands::set_brush_placement_contact_tolerance;
            pub use semio_s_artifact_puzzle_2d_mut_any_7::editor::puzzle2d::commands::set_brush_placement_overlap_budget;
            pub use semio_s_artifact_puzzle_2d_retained::editor::puzzle2d::commands::engagement_repeat_last;
            pub use semio_s_artifact_puzzle_2d_mut_any_7::editor::puzzle2d::commands::set_lod_mode_for_pane;
            pub use semio_s_artifact_puzzle_2d_mut_any_6::editor::puzzle2d::commands::translate_selection;
            pub use semio_s_artifact_puzzle_2d_mut_any_6::editor::puzzle2d::commands::rotate_selection;
            pub use semio_s_artifact_puzzle_2d_retained::editor::puzzle2d::commands::scale_selection;
            pub use semio_s_artifact_puzzle_2d_retained::editor::puzzle2d::commands::export_snapshot;
            pub use semio_s_artifact_puzzle_2d_retained::editor::puzzle2d::commands::import_snapshot;
            pub use semio_s_artifact_puzzle_2d_retained::editor::puzzle2d::commands::open_import_snapshot;
            pub use semio_s_artifact_puzzle_2d_retained::editor::puzzle2d::commands::set_selection_flag;
            pub use semio_s_artifact_puzzle_2d_retained::editor::puzzle2d::commands::set_suggestion_offset;
            pub use semio_s_artifact_puzzle_2d_retained::editor::puzzle2d::commands::set_transform_gumball_flag;
            pub use semio_s_artifact_puzzle_2d_retained::editor::puzzle2d::commands::add_target_region;
            pub use semio_s_artifact_puzzle_2d_retained::editor::puzzle2d::commands::delete_target_region;
            pub use semio_s_artifact_puzzle_2d_retained::editor::puzzle2d::commands::relocate_target_region;
            pub use semio_s_artifact_puzzle_2d_retained::editor::puzzle2d::commands::set_target_region_flag;
            pub use semio_s_artifact_puzzle_2d_retained::editor::puzzle2d::commands::set_area_brush_size;
        }

        #[path = "."]
        pub mod panels {
            pub use semio_s_artifact_puzzle_2d_retained_panels::editor::puzzle2d::panels::catalogue;
            pub use semio_s_artifact_puzzle_2d_retained_panels::editor::puzzle2d::panels::artifact;
            pub use semio_s_artifact_puzzle_2d_retained_panels::editor::puzzle2d::panels::inspection;
            pub use semio_s_artifact_puzzle_2d_retained_panels::editor::puzzle2d::panels::settings;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_puzzle_2d_retained::editor::puzzle2d::modes::edit::*;

                #[path = "."]
                pub mod options {
                    pub use semio_s_artifact_puzzle_2d_tests::editor::puzzle2d::modes::edit::options::brush;
                    pub use semio_s_artifact_puzzle_2d_tests::editor::puzzle2d::modes::edit::options::grid;
                    pub use semio_s_artifact_puzzle_2d_tests::editor::puzzle2d::modes::edit::options::transform;
                    pub use semio_s_artifact_puzzle_2d_tests::editor::puzzle2d::modes::edit::options::lod;
                    pub use semio_s_artifact_puzzle_2d_tests::editor::puzzle2d::modes::edit::options::select;
                }

                pub use semio_s_artifact_puzzle_2d_interaction::editor::puzzle2d::modes::edit::tools;

                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod overview {
                        pub use semio_s_artifact_puzzle_2d_retained::editor::puzzle2d::modes::edit::windows::overview::*;

                        #[path = "."]
                        pub mod utilities {
                            pub use semio_s_artifact_puzzle_2d_tests::editor::puzzle2d::modes::edit::windows::overview::utilities::brush;
                            pub use semio_s_artifact_puzzle_2d_tests::editor::puzzle2d::modes::edit::windows::overview::utilities::area_brush;
                            pub use semio_s_artifact_puzzle_2d_tests::editor::puzzle2d::modes::edit::windows::overview::utilities::select;
                        }
                    }

                    pub use semio_s_artifact_puzzle_2d_retained::editor::puzzle2d::modes::edit::windows::detail;
                    pub use semio_s_artifact_puzzle_2d_retained::editor::puzzle2d::modes::edit::windows::selection;
                }
            }
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod puzzle2d {
        pub use semio_s_artifact_puzzle_2d_mut_any_9::viewer::puzzle2d::*;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_puzzle_2d_mut_any::viewer::puzzle2d::modes::view::*;

                pub use semio_s_artifact_puzzle_2d_inferences::viewer::puzzle2d::modes::view::windows;
            }
        }
    }
}

pub use crate::standards::v1::subsets::any::io::{Puzzle2dBuilderConstruction, Puzzle2dParts, Puzzle2dAnalyzerAnalysis, Puzzle2dBuilderFacets, Puzzle2dBuilder, Puzzle2dAnalyzer, Puzzle2dComposer};

pub use semio_s_artifact_puzzle_2d_mut_any_10::host;

#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod hoisted_snapshot_0;
