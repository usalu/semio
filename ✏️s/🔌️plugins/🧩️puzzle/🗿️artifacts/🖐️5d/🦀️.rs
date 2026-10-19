//! 🧩️ Puzzle 5d artifact — the `puzzle.5d` document schema: the `Puzzle5dSnapshot` (schema/domain/
//! label/meta/kindCatalogs/kindCompatibility/parts/fasteners), its unified 2d+3d part/grip/fastener
//! records, the `Puzzle5dScale` scalar-or-triple pose scale, and the `artifact_kind()` spec the play app's
//! manifest binds. Sibling nodes: `🔺️diff`, `🔧️op`, `🗣️dsl`, `🎒️pack`, `📡️spr`. No `⚙️engine` node —
//! per ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES, an artifact is a `🧬️schema` +
//! `🚪️io` system only; behaviour lives in the sibling editor module, `crate::editor::puzzle5d`.

#![allow(clippy::result_large_err)]

use semio_framework_test_fixtures::fixture_text;

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
    ("puzzle5d", "puzzle.5d", editor::puzzle5d::PUZZLE5D_RETAINED_TOOL_IDS, fixture_text!("@fixtures/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🗄️retained-jobs/🔣️.json"))
}

pub use semio_s_artifact_puzzle_5d_snapshot::{PUZZLE_5D_SCHEMA};

pub use semio_s_artifact_puzzle_5d_mut_any_8::{PUZZLE5D_DIALECT};

//#region 📐️Scale

//#endregion 📐️Scale

//#region 🔖️KindCatalogComposition
// 🧩️ Ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM W4d: `Puzzle5dSnapshot.kind_catalogs`
// duplicated stdio's `s.stdio.semio.kit` type-registry vocabulary (`SemioKitType { id, name,
// category }`) four times over (part/grip/fastener/rope kind catalogs), each row far richer than
// `SemioKitType` can represent. Same split-and-compose pattern `sourcing`'s wave-4 migration used for
// its own `stock: Vec<ObjectKind>` (see `../../../🪵️sourcing/🗿️artifacts/🗂️curation/🦀️.rs`'s
// `🔖️CatalogComposition` region, this migration's primary precedent): every kind-catalog row splits
// into a shared `SemioKitType` half (id/name/category) plus a puzzle5d-owned `*Extra` half carrying
// everything `SemioKitType` cannot, id-joined back together by `kind_catalogs_of`.

//#region 🔖️RowConverters

//#endregion 🔖️RowConverters

//#region 🔖️WholeListConverters

//#endregion 🔖️WholeListConverters

//#endregion 🔖️KindCatalogComposition

//#region 🔖️Snapshot
//#endregion 🔖️Snapshot

pub use semio_s_artifact_puzzle_5d_puzzle5d::{artifact_kind};

pub use semio_s_artifact_puzzle_5d_v1::{ArtifactApps, artifact, definition, pilot_languages};

pub use crate::standards::v1::subsets::any::schema::play_snapshot::Puzzle5dPlaySnapshot;

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1 {
                pub use semio_s_artifact_puzzle_5d_v1::standards::v1::*;

                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        pub use semio_s_artifact_puzzle_5d_v1::standards::v1::subsets::any::*;

                        /// 📐️ The 5D flatten solver — the Rust twin of the React target's own
                        /// `🔖️Flatten` region. Ungated: the wgpu renderer needs it without the
                        /// app-assembly feature (ticket 26/09/17 packet W2k).
                        pub use semio_s_artifact_puzzle_5d_diff::standards::v1::subsets::any::geometry;

                        #[path = "."]
                        pub mod schema {
pub use semio_s_artifact_puzzle_5d_mut_any_5::standards::v1::subsets::any::schema::play_snapshot;

                            pub use semio_s_artifact_puzzle_5d_schema::standards::v1::subsets::any::schema::*;

                            pub use semio_s_artifact_puzzle_5d_snapshot::standards::v1::subsets::any::schema::snapshot;
                            #[path = "."]
                            pub mod inferences {
                                pub use semio_s_artifact_puzzle_5d_diff::standards::v1::subsets::any::schema::inferences::*;

                                pub use semio_s_artifact_puzzle_5d_diff::standards::v1::subsets::any::schema::inferences::flat_position;
                            }
                            pub use semio_s_artifact_puzzle_5d_diff::standards::v1::subsets::any::schema::diff;
                            #[path = "."]
                            pub mod mutations {
                                pub use semio_s_artifact_puzzle_5d_mut_any_5::standards::v1::subsets::any::schema::mutations::*;

                                #[path = "."]
                                pub mod create_part {
                                    pub use semio_s_artifact_puzzle_5d_mut_any_4::standards::v1::subsets::any::schema::mutations::create_part::diff;
                                    pub use semio_s_artifact_puzzle_5d_mut_any_4::standards::v1::subsets::any::schema::mutations::create_part::inverse;
                                    pub use semio_s_artifact_puzzle_5d_mut_any_4::standards::v1::subsets::any::schema::mutations::create_part::*;

                                }
                                #[path = "."]
                                pub mod delete_part {
                                    pub use semio_s_artifact_puzzle_5d_mut_any_4::standards::v1::subsets::any::schema::mutations::delete_part::diff;
                                    pub use semio_s_artifact_puzzle_5d_mut_any_4::standards::v1::subsets::any::schema::mutations::delete_part::inverse;
                                    pub use semio_s_artifact_puzzle_5d_mut_any_4::standards::v1::subsets::any::schema::mutations::delete_part::*;

                                }
                                #[path = "."]
                                pub mod move_part_2d {
                                    pub use semio_s_artifact_puzzle_5d_diff::standards::v1::subsets::any::schema::mutations::move_part_2d::diff;
                                    pub use semio_s_artifact_puzzle_5d_diff::standards::v1::subsets::any::schema::mutations::move_part_2d::inverse;
                                    pub use semio_s_artifact_puzzle_5d_diff::standards::v1::subsets::any::schema::mutations::move_part_2d::*;

                                }
                                #[path = "."]
                                pub mod replace_part_2d_geometry {
                                    pub use semio_s_artifact_puzzle_5d_mut_any_part2d::standards::v1::subsets::any::schema::mutations::replace_part_2d_geometry::diff;
                                    pub use semio_s_artifact_puzzle_5d_mut_any_part2d::standards::v1::subsets::any::schema::mutations::replace_part_2d_geometry::inverse;
                                    pub use semio_s_artifact_puzzle_5d_mut_any_part2d::standards::v1::subsets::any::schema::mutations::replace_part_2d_geometry::*;

                                }
                                #[path = "."]
                                pub mod edit_part_2d_text {
                                    pub use semio_s_artifact_puzzle_5d_mut_any_part2d::standards::v1::subsets::any::schema::mutations::edit_part_2d_text::diff;
                                    pub use semio_s_artifact_puzzle_5d_mut_any_part2d::standards::v1::subsets::any::schema::mutations::edit_part_2d_text::inverse;
                                    pub use semio_s_artifact_puzzle_5d_mut_any_part2d::standards::v1::subsets::any::schema::mutations::edit_part_2d_text::*;

                                }
                                #[path = "."]
                                pub mod change_part_2d_icon {
                                    pub use semio_s_artifact_puzzle_5d_mut_any_part2d::standards::v1::subsets::any::schema::mutations::change_part_2d_icon::diff;
                                    pub use semio_s_artifact_puzzle_5d_mut_any_part2d::standards::v1::subsets::any::schema::mutations::change_part_2d_icon::inverse;
                                    pub use semio_s_artifact_puzzle_5d_mut_any_part2d::standards::v1::subsets::any::schema::mutations::change_part_2d_icon::*;

                                }
                                #[path = "."]
                                pub mod change_part_2d_hidden {
                                    pub use semio_s_artifact_puzzle_5d_mut_any_part2d::standards::v1::subsets::any::schema::mutations::change_part_2d_hidden::diff;
                                    pub use semio_s_artifact_puzzle_5d_mut_any_part2d::standards::v1::subsets::any::schema::mutations::change_part_2d_hidden::inverse;
                                    pub use semio_s_artifact_puzzle_5d_mut_any_part2d::standards::v1::subsets::any::schema::mutations::change_part_2d_hidden::*;

                                }
                                #[path = "."]
                                pub mod change_part_2d_locked {
                                    pub use semio_s_artifact_puzzle_5d_mut_any_part2d::standards::v1::subsets::any::schema::mutations::change_part_2d_locked::diff;
                                    pub use semio_s_artifact_puzzle_5d_mut_any_part2d::standards::v1::subsets::any::schema::mutations::change_part_2d_locked::inverse;
                                    pub use semio_s_artifact_puzzle_5d_mut_any_part2d::standards::v1::subsets::any::schema::mutations::change_part_2d_locked::*;

                                }
                                #[path = "."]
                                pub mod move_part_3d {
                                    pub use semio_s_artifact_puzzle_5d_mut_any::standards::v1::subsets::any::schema::mutations::move_part_3d::diff;
                                    pub use semio_s_artifact_puzzle_5d_mut_any::standards::v1::subsets::any::schema::mutations::move_part_3d::inverse;
                                    pub use semio_s_artifact_puzzle_5d_mut_any::standards::v1::subsets::any::schema::mutations::move_part_3d::*;

                                }
                                #[path = "."]
                                pub mod rotate_part_3d {
                                    pub use semio_s_artifact_puzzle_5d_mut_any::standards::v1::subsets::any::schema::mutations::rotate_part_3d::diff;
                                    pub use semio_s_artifact_puzzle_5d_mut_any::standards::v1::subsets::any::schema::mutations::rotate_part_3d::inverse;
                                    pub use semio_s_artifact_puzzle_5d_mut_any::standards::v1::subsets::any::schema::mutations::rotate_part_3d::*;

                                }
                                #[path = "."]
                                pub mod scale_part_3d {
                                    pub use semio_s_artifact_puzzle_5d_mut_any::standards::v1::subsets::any::schema::mutations::scale_part_3d::diff;
                                    pub use semio_s_artifact_puzzle_5d_mut_any::standards::v1::subsets::any::schema::mutations::scale_part_3d::inverse;
                                    pub use semio_s_artifact_puzzle_5d_mut_any::standards::v1::subsets::any::schema::mutations::scale_part_3d::*;

                                }
                                #[path = "."]
                                pub mod change_part_3d_mesh {
                                    pub use semio_s_artifact_puzzle_5d_mut_any::standards::v1::subsets::any::schema::mutations::change_part_3d_mesh::diff;
                                    pub use semio_s_artifact_puzzle_5d_mut_any::standards::v1::subsets::any::schema::mutations::change_part_3d_mesh::inverse;
                                    pub use semio_s_artifact_puzzle_5d_mut_any::standards::v1::subsets::any::schema::mutations::change_part_3d_mesh::*;

                                }
                                #[path = "."]
                                pub mod edit_part_3d_label {
                                    pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::edit_part_3d_label::diff;
                                    pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::edit_part_3d_label::inverse;
                                    pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::edit_part_3d_label::*;

                                }
                                #[path = "."]
                                pub mod change_part_kind {
                                    pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_part_kind::diff;
                                    pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_part_kind::inverse;
                                    pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_part_kind::*;

                                }
                                #[path = "."]
                                pub mod change_part_anchor {
                                    pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_part_anchor::diff;
                                    pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_part_anchor::inverse;
                                    pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_part_anchor::*;

                                }
                                #[path = "."]
                                pub mod add_part_grip {
                                    pub use semio_s_artifact_puzzle_5d_mut_any_4::standards::v1::subsets::any::schema::mutations::add_part_grip::diff;
                                    pub use semio_s_artifact_puzzle_5d_mut_any_4::standards::v1::subsets::any::schema::mutations::add_part_grip::inverse;
                                    pub use semio_s_artifact_puzzle_5d_mut_any_4::standards::v1::subsets::any::schema::mutations::add_part_grip::*;

                                }
                                #[path = "."]
                                pub mod remove_part_grip {
                                    pub use semio_s_artifact_puzzle_5d_mut_any_4::standards::v1::subsets::any::schema::mutations::remove_part_grip::diff;
                                    pub use semio_s_artifact_puzzle_5d_mut_any_4::standards::v1::subsets::any::schema::mutations::remove_part_grip::inverse;
                                    pub use semio_s_artifact_puzzle_5d_mut_any_4::standards::v1::subsets::any::schema::mutations::remove_part_grip::*;

                                }
                                #[path = "."]
                                pub mod replace_part_grip {
                                    pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::replace_part_grip::diff;
                                    pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::replace_part_grip::inverse;
                                    pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::replace_part_grip::*;

                                }
                                #[path = "."]
                                pub mod connect_grips {
                                    pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::connect_grips::diff;
                                    pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::connect_grips::inverse;
                                    pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::connect_grips::*;

                                }
                                #[path = "."]
                                pub mod disconnect_grips {
                                    pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::disconnect_grips::diff;
                                    pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::disconnect_grips::inverse;
                                    pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::disconnect_grips::*;

                                }
                                #[path = "."]
                                pub mod replace_fastener_geometry {
                                    pub use semio_s_artifact_puzzle_5d_mut_any_5::standards::v1::subsets::any::schema::mutations::replace_fastener_geometry::diff;
                                    pub use semio_s_artifact_puzzle_5d_mut_any_5::standards::v1::subsets::any::schema::mutations::replace_fastener_geometry::inverse;
                                    pub use semio_s_artifact_puzzle_5d_mut_any_5::standards::v1::subsets::any::schema::mutations::replace_fastener_geometry::*;

                                }
                                #[path = "."]
                                pub mod change_fastener_kind {
                                    pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_fastener_kind::diff;
                                    pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_fastener_kind::inverse;
                                    pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_fastener_kind::*;

                                }
                                #[path = "."]
                                pub mod rename_puzzle5d {
                                    pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::rename_puzzle5d::diff;
                                    pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::rename_puzzle5d::inverse;
                                    pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::rename_puzzle5d::*;

                                }
                                #[path = "."]
                                pub mod change_domain {
                                    pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_domain::diff;
                                    pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_domain::inverse;
                                    pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_domain::*;

                                }
                                #[path = "."]
                                pub mod change_description {
                                    pub use semio_s_artifact_puzzle_5d_mut_any_3::standards::v1::subsets::any::schema::mutations::change_description::diff;
                                    pub use semio_s_artifact_puzzle_5d_mut_any_3::standards::v1::subsets::any::schema::mutations::change_description::inverse;
                                    pub use semio_s_artifact_puzzle_5d_mut_any_3::standards::v1::subsets::any::schema::mutations::change_description::*;

                                }
                                #[path = "."]
                                pub mod connect_kind_compatibility {
                                    pub use semio_s_artifact_puzzle_5d_mut_any_3::standards::v1::subsets::any::schema::mutations::connect_kind_compatibility::diff;
                                    pub use semio_s_artifact_puzzle_5d_mut_any_3::standards::v1::subsets::any::schema::mutations::connect_kind_compatibility::inverse;
                                    pub use semio_s_artifact_puzzle_5d_mut_any_3::standards::v1::subsets::any::schema::mutations::connect_kind_compatibility::*;

                                }
                                #[path = "."]
                                pub mod disconnect_kind_compatibility {
                                    pub use semio_s_artifact_puzzle_5d_mut_any_3::standards::v1::subsets::any::schema::mutations::disconnect_kind_compatibility::diff;
                                    pub use semio_s_artifact_puzzle_5d_mut_any_3::standards::v1::subsets::any::schema::mutations::disconnect_kind_compatibility::inverse;
                                    pub use semio_s_artifact_puzzle_5d_mut_any_3::standards::v1::subsets::any::schema::mutations::disconnect_kind_compatibility::*;

                                }
                                #[path = "."]
                                pub mod replace_kind_catalogs {
                                    pub use semio_s_artifact_puzzle_5d_mut_any_3::standards::v1::subsets::any::schema::mutations::replace_kind_catalogs::diff;
                                    pub use semio_s_artifact_puzzle_5d_mut_any_3::standards::v1::subsets::any::schema::mutations::replace_kind_catalogs::inverse;
                                    pub use semio_s_artifact_puzzle_5d_mut_any_3::standards::v1::subsets::any::schema::mutations::replace_kind_catalogs::*;

                                }
                                #[path = "."]
                                pub mod create_target_volume {
                                    pub use semio_s_artifact_puzzle_5d_mut_any_3::standards::v1::subsets::any::schema::mutations::create_target_volume::diff;
                                    pub use semio_s_artifact_puzzle_5d_mut_any_3::standards::v1::subsets::any::schema::mutations::create_target_volume::inverse;
                                    pub use semio_s_artifact_puzzle_5d_mut_any_3::standards::v1::subsets::any::schema::mutations::create_target_volume::*;

                                }
                                #[path = "."]
                                pub mod delete_target_volume {
                                    pub use semio_s_artifact_puzzle_5d_mut_any_3::standards::v1::subsets::any::schema::mutations::delete_target_volume::diff;
                                    pub use semio_s_artifact_puzzle_5d_mut_any_3::standards::v1::subsets::any::schema::mutations::delete_target_volume::inverse;
                                    pub use semio_s_artifact_puzzle_5d_mut_any_3::standards::v1::subsets::any::schema::mutations::delete_target_volume::*;

                                }
                                #[path = "."]
                                pub mod move_target_volume {
                                    pub use semio_s_artifact_puzzle_5d_mut_any_3::standards::v1::subsets::any::schema::mutations::move_target_volume::diff;
                                    pub use semio_s_artifact_puzzle_5d_mut_any_3::standards::v1::subsets::any::schema::mutations::move_target_volume::inverse;
                                    pub use semio_s_artifact_puzzle_5d_mut_any_3::standards::v1::subsets::any::schema::mutations::move_target_volume::*;

                                }
                                #[path = "."]
                                pub mod rotate_target_volume {
                                    pub use semio_s_artifact_puzzle_5d_mut_any_3::standards::v1::subsets::any::schema::mutations::rotate_target_volume::diff;
                                    pub use semio_s_artifact_puzzle_5d_mut_any_3::standards::v1::subsets::any::schema::mutations::rotate_target_volume::inverse;
                                    pub use semio_s_artifact_puzzle_5d_mut_any_3::standards::v1::subsets::any::schema::mutations::rotate_target_volume::*;

                                }
                                #[path = "."]
                                pub mod scale_target_volume {
                                    pub use semio_s_artifact_puzzle_5d_mut_any_3::standards::v1::subsets::any::schema::mutations::scale_target_volume::diff;
                                    pub use semio_s_artifact_puzzle_5d_mut_any_3::standards::v1::subsets::any::schema::mutations::scale_target_volume::inverse;
                                    pub use semio_s_artifact_puzzle_5d_mut_any_3::standards::v1::subsets::any::schema::mutations::scale_target_volume::*;

                                }
                                #[path = "."]
                                pub mod change_target_volume_hidden {
                                    pub use semio_s_artifact_puzzle_5d_diff_2::standards::v1::subsets::any::schema::mutations::change_target_volume_hidden::diff;
                                    pub use semio_s_artifact_puzzle_5d_diff_2::standards::v1::subsets::any::schema::mutations::change_target_volume_hidden::inverse;
                                    pub use semio_s_artifact_puzzle_5d_diff_2::standards::v1::subsets::any::schema::mutations::change_target_volume_hidden::*;

                                }
                                #[path = "."]
                                pub mod change_target_volume_locked {
                                    pub use semio_s_artifact_puzzle_5d_diff_2::standards::v1::subsets::any::schema::mutations::change_target_volume_locked::diff;
                                    pub use semio_s_artifact_puzzle_5d_diff_2::standards::v1::subsets::any::schema::mutations::change_target_volume_locked::inverse;
                                    pub use semio_s_artifact_puzzle_5d_diff_2::standards::v1::subsets::any::schema::mutations::change_target_volume_locked::*;

                                }
                                #[path = "."]
                                pub mod drag_selection_2d {
                                    pub use semio_s_artifact_puzzle_5d_mut_any_5::standards::v1::subsets::any::schema::mutations::drag_selection_2d::diff;
                                    pub use semio_s_artifact_puzzle_5d_mut_any_5::standards::v1::subsets::any::schema::mutations::drag_selection_2d::inverse;
                                    pub use semio_s_artifact_puzzle_5d_mut_any_5::standards::v1::subsets::any::schema::mutations::drag_selection_2d::*;

                                }
                                #[path = "."]
                                pub mod drag_selection_3d {
                                    pub use semio_s_artifact_puzzle_5d_mut_any_5::standards::v1::subsets::any::schema::mutations::drag_selection_3d::diff;
                                    pub use semio_s_artifact_puzzle_5d_mut_any_5::standards::v1::subsets::any::schema::mutations::drag_selection_3d::inverse;
                                    pub use semio_s_artifact_puzzle_5d_mut_any_5::standards::v1::subsets::any::schema::mutations::drag_selection_3d::*;

                                }
                                #[path = "."]
                                pub mod rotate_selection_3d {
                                    pub use semio_s_artifact_puzzle_5d_mut_any_5::standards::v1::subsets::any::schema::mutations::rotate_selection_3d::diff;
                                    pub use semio_s_artifact_puzzle_5d_mut_any_5::standards::v1::subsets::any::schema::mutations::rotate_selection_3d::inverse;
                                    pub use semio_s_artifact_puzzle_5d_mut_any_5::standards::v1::subsets::any::schema::mutations::rotate_selection_3d::*;

                                }
                                #[path = "."]
                                pub mod scale_selection_3d {
                                    pub use semio_s_artifact_puzzle_5d_mut_any_5::standards::v1::subsets::any::schema::mutations::scale_selection_3d::diff;
                                    pub use semio_s_artifact_puzzle_5d_mut_any_5::standards::v1::subsets::any::schema::mutations::scale_selection_3d::inverse;
                                    pub use semio_s_artifact_puzzle_5d_mut_any_5::standards::v1::subsets::any::schema::mutations::scale_selection_3d::*;

                                }
                            }
                            pub use semio_s_artifact_puzzle_5d_diff::standards::v1::subsets::any::schema::transfer;
                        }
                        #[path = "."]
                        pub mod io {
                            pub use semio_s_artifact_puzzle_5d_tests::standards::v1::subsets::any::io::*;
pub mod sqlite {
    pub use semio_s_artifact_puzzle_5d_document_records::standards::v1::subsets::any::io::sqlite::*;
    pub use semio_s_artifact_puzzle_5d_snapshot::standards::v1::subsets::any::io::sqlite::snapshot;
}

                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        pub use semio_s_artifact_puzzle_5d_snapshot::standards::v1::subsets::any::io::import::deserializers::artifacts::zip;
                                        pub use semio_s_artifact_puzzle_5d_diff::standards::v1::subsets::any::io::import::deserializers::artifacts::txt;
                                        pub use semio_s_artifact_puzzle_5d_diff::standards::v1::subsets::any::io::import::deserializers::artifacts::json;
                                    }
                                }
                            }
                            #[path = "."]
                            pub mod export {
                                #[path = "."]
                                pub mod serializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        pub use semio_s_artifact_puzzle_5d_diff::standards::v1::subsets::any::io::export::serializers::artifacts::zip;
                                        pub use semio_s_artifact_puzzle_5d_diff::standards::v1::subsets::any::io::export::serializers::artifacts::txt;
                                        pub use semio_s_artifact_puzzle_5d_tests::standards::v1::subsets::any::io::export::serializers::artifacts::png;
                                        pub use semio_s_artifact_puzzle_5d_diff::standards::v1::subsets::any::io::export::serializers::artifacts::json;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

                pub use crate::standards::v1::subsets::any::schema::diff::Puzzle5dDiff;
        pub use crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;

pub use semio_s_artifact_puzzle_5d_mut_any_5::{apply_puzzle5d_mutation};
        pub use crate::standards::v1::subsets::any::schema::snapshot::Puzzle5dSnapshot;

#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod puzzle5d {
        pub use semio_s_artifact_puzzle_5d_diff_2::examples::puzzle5d::capsule_dream;
        pub use semio_s_artifact_puzzle_5d_diff_2::examples::puzzle5d::concrete_forest;
        pub use semio_s_artifact_puzzle_5d_diff_2::examples::puzzle5d::nakagin_capsule_tower;

    }
}

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod puzzle5d {
pub use semio_s_artifact_puzzle_5d_puzzle5d::editor::puzzle5d::*;

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo_session {
                pub use semio_s_artifact_puzzle_5d_document_records::editor::puzzle5d::examples::demo_session::*;

            }
        }

        #[path = "."]
        pub mod config {
            pub use semio_s_artifact_puzzle_5d_tests::editor::puzzle5d::config::*;

            pub use semio_s_artifact_puzzle_5d_document_records::editor::puzzle5d::config::schema;
        }

        pub mod window {
    pub use semio_s_artifact_puzzle_5d_tests::editor::puzzle5d::window::*;
    pub use semio_s_artifact_puzzle_5d_catalog::editor::puzzle5d::window::schema;
}

        #[path = "."]
        pub mod presence {
            pub use semio_s_artifact_puzzle_5d_document_records::editor::puzzle5d::presence::*;

            pub use semio_s_artifact_puzzle_5d_document_records::editor::puzzle5d::presence::schema;
        }
        pub use semio_s_artifact_puzzle_5d_document_records::editor::puzzle5d::terminology;
        pub use semio_s_artifact_puzzle_5d_v1::editor::puzzle5d::wasm;

        #[path = "."]
        pub mod precompute {
            pub use semio_s_artifact_puzzle_5d_catalog::editor::puzzle5d::precompute::*;

            pub use semio_s_artifact_puzzle_5d_interaction::editor::puzzle5d::precompute::fill;
            pub use semio_s_artifact_puzzle_5d_document_records::editor::puzzle5d::precompute::geometry;
            pub use semio_s_artifact_puzzle_5d_interaction::editor::puzzle5d::precompute::brush;
        }

        #[path = "."]
        pub mod commands {
            pub use semio_s_artifact_puzzle_5d_interaction::editor::puzzle5d::commands::add_brush_part;
            pub use semio_s_artifact_puzzle_5d_interaction::editor::puzzle5d::commands::apply_sun;
            pub use semio_s_artifact_puzzle_5d_dispatch::editor::puzzle5d::commands::cycle_brush_candidate;
            pub use semio_s_artifact_puzzle_5d_interaction::editor::puzzle5d::commands::target_brush_suggestions;
            pub use semio_s_artifact_puzzle_5d_store_preparation_commands::editor::puzzle5d::commands::open_vortex_suggestions;
            pub use semio_s_artifact_puzzle_5d_store_preparation_commands::editor::puzzle5d::commands::close_vortex_suggestions;
            pub use semio_s_artifact_puzzle_5d_interaction::editor::puzzle5d::commands::hover_suggestion;
            pub use semio_s_artifact_puzzle_5d_dispatch::editor::puzzle5d::commands::accept_suggestion;
            pub use semio_s_artifact_puzzle_5d_store_preparation_commands::editor::puzzle5d::commands::delete_selection;
            pub use semio_s_artifact_puzzle_5d_store_preparation_commands::editor::puzzle5d::commands::duplicate_selection;
            pub use semio_s_artifact_puzzle_5d_store_preparation_commands::editor::puzzle5d::commands::engagement_abort;
            pub use semio_s_artifact_puzzle_5d_store_preparation_commands::editor::puzzle5d::commands::engagement_control_select;
            pub use semio_s_artifact_puzzle_5d_store_preparation_commands::editor::puzzle5d::commands::engagement_input;
            pub use semio_s_artifact_puzzle_5d_store_preparation_commands::editor::puzzle5d::commands::engagement_repeat_last;
            pub use semio_s_artifact_puzzle_5d_interaction::editor::puzzle5d::commands::engagement_submit;
            pub use semio_s_artifact_puzzle_5d_catalog::editor::puzzle5d::commands::proximity_connect;
            pub use semio_s_artifact_puzzle_5d_store_preparation_commands::editor::puzzle5d::commands::register_brush_mesh;
            pub use semio_s_artifact_puzzle_5d_interaction::editor::puzzle5d::commands::rotate_selection;
            pub use semio_s_artifact_puzzle_5d_interaction::editor::puzzle5d::commands::scale_selection;
            pub use semio_s_artifact_puzzle_5d_store_preparation_commands::editor::puzzle5d::commands::select_same_kind;
            pub use semio_s_artifact_puzzle_5d_store_preparation_commands::editor::puzzle5d::commands::set_brush_placement_contact_tolerance;
            pub use semio_s_artifact_puzzle_5d_dispatch::editor::puzzle5d::commands::set_camera;
            pub use semio_s_artifact_puzzle_5d_store_preparation_commands::editor::puzzle5d::commands::set_camera_2d;
            pub use semio_s_artifact_puzzle_5d_store_preparation_commands::editor::puzzle5d::commands::set_camera_3d;
            pub use semio_s_artifact_puzzle_5d_interaction::editor::puzzle5d::commands::set_fill_count;
            pub use semio_s_artifact_puzzle_5d_tests::editor::puzzle5d::commands::export_snapshot;
            pub use semio_s_artifact_puzzle_5d_store_preparation_commands::editor::puzzle5d::commands::import_snapshot;
            pub use semio_s_artifact_puzzle_5d_store_preparation_commands::editor::puzzle5d::commands::open_import_snapshot;
            pub use semio_s_artifact_puzzle_5d_store_preparation_commands::editor::puzzle5d::commands::open_add_part_dialog;
            pub use semio_s_artifact_puzzle_5d_store_preparation_commands::editor::puzzle5d::commands::set_grid_factor;
            pub use semio_s_artifact_puzzle_5d_store_preparation_commands::editor::puzzle5d::commands::set_grid_snap_enabled;
            pub use semio_s_artifact_puzzle_5d_store_preparation_commands::editor::puzzle5d::commands::set_grid_spacing;
            pub use semio_s_artifact_puzzle_5d_store_preparation_commands::editor::puzzle5d::commands::set_grid_visible;
            pub use semio_s_artifact_puzzle_5d_store_preparation_commands::editor::puzzle5d::commands::set_grip_direction;
            pub use semio_s_artifact_puzzle_5d_store_preparation_commands::editor::puzzle5d::commands::set_grip_show;
            pub use semio_s_artifact_puzzle_5d_store_preparation_commands::editor::puzzle5d::commands::set_lod_automatic;
            pub use semio_s_artifact_puzzle_5d_store_preparation_commands::editor::puzzle5d::commands::set_lod_depth_variable;
            pub use semio_s_artifact_puzzle_5d_store_preparation_commands::editor::puzzle5d::commands::set_lod_manual;
            pub use semio_s_artifact_puzzle_5d_store_preparation_commands::editor::puzzle5d::commands::set_projection;
            pub use semio_s_artifact_puzzle_5d_mut_any_7::editor::puzzle5d::commands::set_selectable_kind;
            pub use semio_s_artifact_puzzle_5d_mut_any_7::editor::puzzle5d::commands::set_transform_gumball_flag;
            pub use semio_s_artifact_puzzle_5d_dispatch::editor::puzzle5d::commands::set_kind_weight;
            pub use semio_s_artifact_puzzle_5d_dispatch::editor::puzzle5d::commands::set_lod_mode;
            pub use semio_s_artifact_puzzle_5d_dispatch::editor::puzzle5d::commands::set_proximity_radius;
            pub use semio_s_artifact_puzzle_5d_dispatch::editor::puzzle5d::commands::set_chunk_size;
            pub use semio_s_artifact_puzzle_5d_dispatch::editor::puzzle5d::commands::set_selection_flag;
            pub use semio_s_artifact_puzzle_5d_dispatch::editor::puzzle5d::commands::set_suggestion_offset;
            pub use semio_s_artifact_puzzle_5d_interaction::editor::puzzle5d::commands::translate_selection;
            pub use semio_s_artifact_puzzle_5d_dispatch::editor::puzzle5d::commands::world_relocate;
            pub use semio_s_artifact_puzzle_5d_interaction::editor::puzzle5d::commands::focus_selection;
            pub use semio_s_artifact_puzzle_5d_dispatch::editor::puzzle5d::commands::add_target_volume;
            pub use semio_s_artifact_puzzle_5d_dispatch::editor::puzzle5d::commands::delete_target_volume;
            pub use semio_s_artifact_puzzle_5d_dispatch::editor::puzzle5d::commands::relocate_target_volume;
            pub use semio_s_artifact_puzzle_5d_dispatch::editor::puzzle5d::commands::set_target_volume_flag;
            pub use semio_s_artifact_puzzle_5d_dispatch::editor::puzzle5d::commands::set_voxel_dims;
        }

        #[path = "."]
        pub mod panels {
            pub use semio_s_artifact_puzzle_5d_dispatch_panels::editor::puzzle5d::panels::catalogue;
            pub use semio_s_artifact_puzzle_5d_dispatch_panels::editor::puzzle5d::panels::artifact;
            pub use semio_s_artifact_puzzle_5d_dispatch_panels::editor::puzzle5d::panels::inspection;
            pub use semio_s_artifact_puzzle_5d_dispatch_panels::editor::puzzle5d::panels::settings;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_puzzle_5d_interaction::editor::puzzle5d::modes::edit::*;

                pub use semio_s_artifact_puzzle_5d_catalog::editor::puzzle5d::modes::edit::options;

                pub use semio_s_artifact_puzzle_5d_catalog::editor::puzzle5d::modes::edit::tools;

                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod board2d {
                        pub use semio_s_artifact_puzzle_5d_dispatch::editor::puzzle5d::modes::edit::windows::board2d::*;

                        #[path = "."]
                        pub mod actions {
                            pub use semio_s_artifact_puzzle_5d_document_records::editor::puzzle5d::modes::edit::windows::board2d::actions::apply_board_events;
                            pub use semio_s_artifact_puzzle_5d_document_records::editor::puzzle5d::modes::edit::windows::board2d::actions::set_camera;
                        }

                        #[path = "."]
                        pub mod options {
                            pub use semio_s_artifact_puzzle_5d_tests::editor::puzzle5d::modes::edit::windows::board2d::options::grid;
                            pub use semio_s_artifact_puzzle_5d_tests::editor::puzzle5d::modes::edit::windows::board2d::options::lod;
                            pub use semio_s_artifact_puzzle_5d_tests::editor::puzzle5d::modes::edit::windows::board2d::options::select;
                        }

                        #[path = "."]
                        pub mod utilities {
                            pub use semio_s_artifact_puzzle_5d_tests::editor::puzzle5d::modes::edit::windows::board2d::utilities::brush;
                            pub use semio_s_artifact_puzzle_5d_document_records::editor::puzzle5d::modes::edit::windows::board2d::utilities::select;
                        }
                    }

                    #[path = "."]
                    pub mod world3d {
                        pub use semio_s_artifact_puzzle_5d_interaction::editor::puzzle5d::modes::edit::windows::world3d::*;

                        #[path = "."]
                        pub mod actions {
                            pub use semio_s_artifact_puzzle_5d_document_records::editor::puzzle5d::modes::edit::windows::world3d::actions::rotate_selection;
                            pub use semio_s_artifact_puzzle_5d_kind_catalogs::editor::puzzle5d::modes::edit::windows::world3d::actions::scale_selection;
                            pub use semio_s_artifact_puzzle_5d_kind_catalogs::editor::puzzle5d::modes::edit::windows::world3d::actions::set_camera;
                            pub use semio_s_artifact_puzzle_5d_kind_catalogs::editor::puzzle5d::modes::edit::windows::world3d::actions::translate_selection;
                            pub use semio_s_artifact_puzzle_5d_kind_catalogs::editor::puzzle5d::modes::edit::windows::world3d::actions::world_relocate;
                        }

                        #[path = "."]
                        pub mod options {
                            pub use semio_s_artifact_puzzle_5d_tests::editor::puzzle5d::modes::edit::windows::world3d::options::grid;
                            pub use semio_s_artifact_puzzle_5d_tests::editor::puzzle5d::modes::edit::windows::world3d::options::grip;
                            pub use semio_s_artifact_puzzle_5d_tests::editor::puzzle5d::modes::edit::windows::world3d::options::lod;
                            pub use semio_s_artifact_puzzle_5d_tests::editor::puzzle5d::modes::edit::windows::world3d::options::projection;
                            pub use semio_s_artifact_puzzle_5d_tests::editor::puzzle5d::modes::edit::windows::world3d::options::select;
                            pub use semio_s_artifact_puzzle_5d_tests::editor::puzzle5d::modes::edit::windows::world3d::options::sun;
                        }

                        #[path = "."]
                        pub mod utilities {
                            pub use semio_s_artifact_puzzle_5d_catalog::editor::puzzle5d::modes::edit::windows::world3d::utilities::transform;
                            pub use semio_s_artifact_puzzle_5d_kind_catalogs::editor::puzzle5d::modes::edit::windows::world3d::utilities::world_relocate;
                            pub use semio_s_artifact_puzzle_5d_tests::editor::puzzle5d::modes::edit::windows::world3d::utilities::volume_brush;
                        }
                    }
                }
            }
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod puzzle5d {
pub use semio_s_artifact_puzzle_5d_mut_any_8::viewer::puzzle5d::*;

pub use semio_s_artifact_puzzle_5d_kind_catalogs::viewer::puzzle5d::presence;
pub use semio_s_artifact_puzzle_5d_kind_catalogs::viewer::puzzle5d::config;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_puzzle_5d_diff::viewer::puzzle5d::modes::view::*;

                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_puzzle_5d_diff::viewer::puzzle5d::modes::view::windows::board2d;

                    pub use semio_s_artifact_puzzle_5d_diff::viewer::puzzle5d::modes::view::windows::world3d;
                }
            }
        }
    }
}

pub use crate::standards::v1::subsets::any::io::{Puzzle5dBuilderConstruction, Puzzle5dParts, Puzzle5dAnalyzerAnalysis, Puzzle5dBuilderFacets, Puzzle5dBuilder, Puzzle5dAnalyzer, Puzzle5dComposer};

pub use semio_s_artifact_puzzle_5d_tests::host;

#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_0;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_1;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod hoisted_snapshot_2;
