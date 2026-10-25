//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

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
                    pub use semio_s_artifact_note_note_schema::standards::v1::subsets::any::schema::*;
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../../🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        pub use semio_s_artifact_note_note_schema::standards::v1::subsets::any::io::binary::snapshot;
                        #[path = "../../../../../🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                    }
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_note_note_schema::standards::v1::subsets::any::io::text::snapshot;
                        #[path = "../../../../../🚪️io/📝️text/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                    }
                    #[path = "."]
                    pub mod sqlite {
                        pub use semio_s_artifact_note_note_schema::standards::v1::subsets::any::io::sqlite::snapshot;
                    }
                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod svg {
                                    #[path = "."]
                                    pub mod v1_1 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../../../🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            #[path = "."]
            pub mod ink {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod change_pencil_width {
                            pub use semio_s_artifact_note_note_mut_family::standards::v1::subsets::ink::schema::mutations::change_pencil_width::*;
                            pub use semio_s_artifact_note_note_mut_family::standards::v1::subsets::ink::schema::mutations::change_pencil_width::diff;
                            pub use semio_s_artifact_note_note_mut_family::standards::v1::subsets::ink::schema::mutations::change_pencil_width::inverse;
                        }
                        #[path = "."]
                        pub mod change_eraser_radius {
                            pub use semio_s_artifact_note_note_mut_family::standards::v1::subsets::ink::schema::mutations::change_eraser_radius::*;
                            pub use semio_s_artifact_note_note_mut_family::standards::v1::subsets::ink::schema::mutations::change_eraser_radius::diff;
                            pub use semio_s_artifact_note_note_mut_family::standards::v1::subsets::ink::schema::mutations::change_eraser_radius::inverse;
                        }
                        #[path = "."]
                        pub mod change_block_ink_width {
                            pub use semio_s_artifact_note_note_mut_family::standards::v1::subsets::ink::schema::mutations::change_block_ink_width::*;
                            pub use semio_s_artifact_note_note_mut_family::standards::v1::subsets::ink::schema::mutations::change_block_ink_width::diff;
                            pub use semio_s_artifact_note_note_mut_family::standards::v1::subsets::ink::schema::mutations::change_block_ink_width::inverse;
                        }
                        #[path = "."]
                        pub mod edit_block_ink_stroke {
                            pub use semio_s_artifact_note_note_mut_family::standards::v1::subsets::ink::schema::mutations::edit_block_ink_stroke::*;
                            pub use semio_s_artifact_note_note_mut_family::standards::v1::subsets::ink::schema::mutations::edit_block_ink_stroke::diff;
                            pub use semio_s_artifact_note_note_mut_family::standards::v1::subsets::ink::schema::mutations::edit_block_ink_stroke::inverse;
                        }
                    }
                }
            }
            #[path = "."]
            pub mod text {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod edit_block_text {
                            pub use semio_s_artifact_note_note_mut_family::standards::v1::subsets::text::schema::mutations::edit_block_text::*;
                            pub use semio_s_artifact_note_note_mut_family::standards::v1::subsets::text::schema::mutations::edit_block_text::diff;
                            pub use semio_s_artifact_note_note_mut_family::standards::v1::subsets::text::schema::mutations::edit_block_text::inverse;
                        }
                    }
                }
            }
            #[path = "."]
            pub mod math {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod edit_block_math {
                            pub use semio_s_artifact_note_note_mut_family::standards::v1::subsets::math::schema::mutations::edit_block_math::*;
                            pub use semio_s_artifact_note_note_mut_family::standards::v1::subsets::math::schema::mutations::edit_block_math::diff;
                            pub use semio_s_artifact_note_note_mut_family::standards::v1::subsets::math::schema::mutations::edit_block_math::inverse;
                        }
                    }
                }
            }
            #[path = "."]
            pub mod table {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod insert_table_row {
                            pub use semio_s_artifact_note_note_insert_table_row::standards::v1::subsets::table::schema::mutations::insert_table_row::*;
                            pub use semio_s_artifact_note_note_insert_table_row::standards::v1::subsets::table::schema::mutations::insert_table_row::diff;
                            pub use semio_s_artifact_note_note_insert_table_row::standards::v1::subsets::table::schema::mutations::insert_table_row::inverse;
                        }
                        #[path = "."]
                        pub mod remove_table_row {
                            pub use semio_s_artifact_note_note_insert_table_row::standards::v1::subsets::table::schema::mutations::remove_table_row::*;
                            pub use semio_s_artifact_note_note_insert_table_row::standards::v1::subsets::table::schema::mutations::remove_table_row::diff;
                            pub use semio_s_artifact_note_note_insert_table_row::standards::v1::subsets::table::schema::mutations::remove_table_row::inverse;
                        }
                        #[path = "."]
                        pub mod insert_table_column {
                            pub use semio_s_artifact_note_note_insert_table_row::standards::v1::subsets::table::schema::mutations::insert_table_column::*;
                            pub use semio_s_artifact_note_note_insert_table_row::standards::v1::subsets::table::schema::mutations::insert_table_column::diff;
                            pub use semio_s_artifact_note_note_insert_table_row::standards::v1::subsets::table::schema::mutations::insert_table_column::inverse;
                        }
                        #[path = "."]
                        pub mod remove_table_column {
                            pub use semio_s_artifact_note_note_insert_table_row::standards::v1::subsets::table::schema::mutations::remove_table_column::*;
                            pub use semio_s_artifact_note_note_insert_table_row::standards::v1::subsets::table::schema::mutations::remove_table_column::diff;
                            pub use semio_s_artifact_note_note_insert_table_row::standards::v1::subsets::table::schema::mutations::remove_table_column::inverse;
                        }
                    }
                }
            }
            #[path = "."]
            pub mod asset {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod create_asset {
                            pub use semio_s_artifact_note_note_mut_family_2::standards::v1::subsets::asset::schema::mutations::create_asset::*;
                            pub use semio_s_artifact_note_note_mut_family_2::standards::v1::subsets::asset::schema::mutations::create_asset::diff;
                            pub use semio_s_artifact_note_note_mut_family_2::standards::v1::subsets::asset::schema::mutations::create_asset::inverse;
                        }
                        #[path = "."]
                        pub mod replace_asset_payload {
                            pub use semio_s_artifact_note_note_mut_family_2::standards::v1::subsets::asset::schema::mutations::replace_asset_payload::*;
                            pub use semio_s_artifact_note_note_mut_family_2::standards::v1::subsets::asset::schema::mutations::replace_asset_payload::diff;
                            pub use semio_s_artifact_note_note_mut_family_2::standards::v1::subsets::asset::schema::mutations::replace_asset_payload::inverse;
                        }
                        #[path = "."]
                        pub mod delete_asset {
                            pub use semio_s_artifact_note_note_mut_family_2::standards::v1::subsets::asset::schema::mutations::delete_asset::*;
                            pub use semio_s_artifact_note_note_mut_family_2::standards::v1::subsets::asset::schema::mutations::delete_asset::diff;
                            pub use semio_s_artifact_note_note_mut_family_2::standards::v1::subsets::asset::schema::mutations::delete_asset::inverse;
                        }
                    }
                }
            }
            #[path = "."]
            pub mod block {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod create_block {
                            pub use semio_s_artifact_note_note_mut_family_2::standards::v1::subsets::block::schema::mutations::create_block::*;
                            pub use semio_s_artifact_note_note_mut_family_2::standards::v1::subsets::block::schema::mutations::create_block::diff;
                            pub use semio_s_artifact_note_note_mut_family_2::standards::v1::subsets::block::schema::mutations::create_block::inverse;
                        }
                        #[path = "."]
                        pub mod delete_block {
                            pub use semio_s_artifact_note_note_mut_family_2::standards::v1::subsets::block::schema::mutations::delete_block::*;
                            pub use semio_s_artifact_note_note_mut_family_2::standards::v1::subsets::block::schema::mutations::delete_block::diff;
                            pub use semio_s_artifact_note_note_mut_family_2::standards::v1::subsets::block::schema::mutations::delete_block::inverse;
                        }
                        #[path = "."]
                        pub mod delete_blocks {
                            pub use semio_s_artifact_note_note_delete_blocks::standards::v1::subsets::block::schema::mutations::delete_blocks::*;
                            pub use semio_s_artifact_note_note_delete_blocks::standards::v1::subsets::block::schema::mutations::delete_blocks::diff;
                            pub use semio_s_artifact_note_note_delete_blocks::standards::v1::subsets::block::schema::mutations::delete_blocks::inverse;
                        }
                        #[path = "."]
                        pub mod duplicate_block {
                            pub use semio_s_artifact_note_note_delete_blocks::standards::v1::subsets::block::schema::mutations::duplicate_block::*;
                            pub use semio_s_artifact_note_note_delete_blocks::standards::v1::subsets::block::schema::mutations::duplicate_block::diff;
                            pub use semio_s_artifact_note_note_delete_blocks::standards::v1::subsets::block::schema::mutations::duplicate_block::inverse;
                        }
                        #[path = "."]
                        pub mod duplicate_blocks {
                            pub use semio_s_artifact_note_note_delete_blocks::standards::v1::subsets::block::schema::mutations::duplicate_blocks::*;
                            pub use semio_s_artifact_note_note_delete_blocks::standards::v1::subsets::block::schema::mutations::duplicate_blocks::diff;
                            pub use semio_s_artifact_note_note_delete_blocks::standards::v1::subsets::block::schema::mutations::duplicate_blocks::inverse;
                        }
                        #[path = "."]
                        pub mod move_block_to_container {
                            pub use semio_s_artifact_note_note_mut_family_2::standards::v1::subsets::block::schema::mutations::move_block_to_container::*;
                            pub use semio_s_artifact_note_note_mut_family_2::standards::v1::subsets::block::schema::mutations::move_block_to_container::diff;
                            pub use semio_s_artifact_note_note_mut_family_2::standards::v1::subsets::block::schema::mutations::move_block_to_container::inverse;
                        }
                        #[path = "."]
                        pub mod drag_blocks {
                            pub use semio_s_artifact_note_note_mut_family_2::standards::v1::subsets::block::schema::mutations::drag_blocks::*;
                            pub use semio_s_artifact_note_note_mut_family_2::standards::v1::subsets::block::schema::mutations::drag_blocks::diff;
                            pub use semio_s_artifact_note_note_mut_family_2::standards::v1::subsets::block::schema::mutations::drag_blocks::inverse;
                        }
                        #[path = "."]
                        pub mod rename_block {
                            pub use semio_s_artifact_note_note_mut_family_2::standards::v1::subsets::block::schema::mutations::rename_block::*;
                            pub use semio_s_artifact_note_note_mut_family_2::standards::v1::subsets::block::schema::mutations::rename_block::diff;
                            pub use semio_s_artifact_note_note_mut_family_2::standards::v1::subsets::block::schema::mutations::rename_block::inverse;
                        }
                        #[path = "."]
                        pub mod change_block_visible {
                            pub use semio_s_artifact_note_note_mut_family_2::standards::v1::subsets::block::schema::mutations::change_block_visible::*;
                            pub use semio_s_artifact_note_note_mut_family_2::standards::v1::subsets::block::schema::mutations::change_block_visible::diff;
                            pub use semio_s_artifact_note_note_mut_family_2::standards::v1::subsets::block::schema::mutations::change_block_visible::inverse;
                        }
                        #[path = "."]
                        pub mod change_block_locked {
                            pub use semio_s_artifact_note_note_mut_family::standards::v1::subsets::block::schema::mutations::change_block_locked::*;
                            pub use semio_s_artifact_note_note_mut_family::standards::v1::subsets::block::schema::mutations::change_block_locked::diff;
                            pub use semio_s_artifact_note_note_mut_family::standards::v1::subsets::block::schema::mutations::change_block_locked::inverse;
                        }
                        #[path = "."]
                        pub mod move_block {
                            pub use semio_s_artifact_note_note_mut_family_3::standards::v1::subsets::block::schema::mutations::move_block::*;
                            pub use semio_s_artifact_note_note_mut_family_3::standards::v1::subsets::block::schema::mutations::move_block::diff;
                            pub use semio_s_artifact_note_note_mut_family_3::standards::v1::subsets::block::schema::mutations::move_block::inverse;
                        }
                        #[path = "."]
                        pub mod resize_block {
                            pub use semio_s_artifact_note_note_mut_family_3::standards::v1::subsets::block::schema::mutations::resize_block::*;
                            pub use semio_s_artifact_note_note_mut_family_3::standards::v1::subsets::block::schema::mutations::resize_block::diff;
                            pub use semio_s_artifact_note_note_mut_family_3::standards::v1::subsets::block::schema::mutations::resize_block::inverse;
                        }
                        #[path = "."]
                        pub mod change_block_font_size {
                            pub use semio_s_artifact_note_note_mut_family_3::standards::v1::subsets::block::schema::mutations::change_block_font_size::*;
                            pub use semio_s_artifact_note_note_mut_family_3::standards::v1::subsets::block::schema::mutations::change_block_font_size::diff;
                            pub use semio_s_artifact_note_note_mut_family_3::standards::v1::subsets::block::schema::mutations::change_block_font_size::inverse;
                        }
                    }
                }
            }
            #[path = "."]
            pub mod canvas {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod change_grid_visible {
                            pub use semio_s_artifact_note_note_mut_family_3::standards::v1::subsets::canvas::schema::mutations::change_grid_visible::*;
                            pub use semio_s_artifact_note_note_mut_family_3::standards::v1::subsets::canvas::schema::mutations::change_grid_visible::diff;
                            pub use semio_s_artifact_note_note_mut_family_3::standards::v1::subsets::canvas::schema::mutations::change_grid_visible::inverse;
                        }
                        #[path = "."]
                        pub mod change_grid_spacing {
                            pub use semio_s_artifact_note_note_mut_family_3::standards::v1::subsets::canvas::schema::mutations::change_grid_spacing::*;
                            pub use semio_s_artifact_note_note_mut_family_3::standards::v1::subsets::canvas::schema::mutations::change_grid_spacing::diff;
                            pub use semio_s_artifact_note_note_mut_family_3::standards::v1::subsets::canvas::schema::mutations::change_grid_spacing::inverse;
                        }
                        #[path = "."]
                        pub mod change_grid_subdivisions {
                            pub use semio_s_artifact_note_note_mut_family_3::standards::v1::subsets::canvas::schema::mutations::change_grid_subdivisions::*;
                            pub use semio_s_artifact_note_note_mut_family_3::standards::v1::subsets::canvas::schema::mutations::change_grid_subdivisions::diff;
                            pub use semio_s_artifact_note_note_mut_family_3::standards::v1::subsets::canvas::schema::mutations::change_grid_subdivisions::inverse;
                        }
                        #[path = "."]
                        pub mod change_grid_opacity {
                            pub use semio_s_artifact_note_note_mut_family_3::standards::v1::subsets::canvas::schema::mutations::change_grid_opacity::*;
                            pub use semio_s_artifact_note_note_mut_family_3::standards::v1::subsets::canvas::schema::mutations::change_grid_opacity::diff;
                            pub use semio_s_artifact_note_note_mut_family_3::standards::v1::subsets::canvas::schema::mutations::change_grid_opacity::inverse;
                        }
                        #[path = "."]
                        pub mod change_snap_enabled {
                            pub use semio_s_artifact_note_note_mut_family_3::standards::v1::subsets::canvas::schema::mutations::change_snap_enabled::*;
                            pub use semio_s_artifact_note_note_mut_family_3::standards::v1::subsets::canvas::schema::mutations::change_snap_enabled::diff;
                            pub use semio_s_artifact_note_note_mut_family_3::standards::v1::subsets::canvas::schema::mutations::change_snap_enabled::inverse;
                        }
                        #[path = "."]
                        pub mod change_snap_grid_spacing {
                            pub use semio_s_artifact_note_note_mut_family_3::standards::v1::subsets::canvas::schema::mutations::change_snap_grid_spacing::*;
                            pub use semio_s_artifact_note_note_mut_family_3::standards::v1::subsets::canvas::schema::mutations::change_snap_grid_spacing::diff;
                            pub use semio_s_artifact_note_note_mut_family_3::standards::v1::subsets::canvas::schema::mutations::change_snap_grid_spacing::inverse;
                        }
                    }
                }
            }
            #[path = "."]
            pub mod document {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod rename_note {
                            pub use semio_s_artifact_note_note_delete_blocks::standards::v1::subsets::document::schema::mutations::rename_note::*;
                            pub use semio_s_artifact_note_note_delete_blocks::standards::v1::subsets::document::schema::mutations::rename_note::diff;
                            pub use semio_s_artifact_note_note_delete_blocks::standards::v1::subsets::document::schema::mutations::rename_note::inverse;
                        }
                    }
                }
            }
        }
    }
}
pub mod snapshot {
    pub mod schema {
        pub use crate::standards::v1::subsets::any::schema::snapshot::*;
    }

}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod note {
        pub use semio_s_artifact_note_note_diff::editor::note::support;
    }
}
#[path = "."]
pub mod host {
#[path = "../../../../../../../../../🔨️modules/🏠️host/🧰️owned/🦀️.rs"]
pub mod owned;
}
