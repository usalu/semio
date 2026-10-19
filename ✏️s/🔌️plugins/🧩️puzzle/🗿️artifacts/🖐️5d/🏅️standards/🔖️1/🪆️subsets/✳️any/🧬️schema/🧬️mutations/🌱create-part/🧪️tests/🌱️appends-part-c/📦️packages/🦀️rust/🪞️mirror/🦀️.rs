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
#[path = "../../../../../../../../🧬️schema/🎞️play-snapshot/🦀️.rs"]
pub mod play_snapshot;

                    pub use semio_s_artifact_puzzle_5d_document_records::standards::v1::subsets::any::schema::document_records;
                    pub use semio_s_artifact_puzzle_5d_snapshot::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_puzzle_5d_diff::standards::v1::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../../../../../🦀️.rs"]
                        mod component;
                        pub use component::*;
                        pub use semio_s_artifact_puzzle_5d_kind_catalogs::standards::v1::subsets::any::schema::mutations::support;
                        #[path = "."]
                        pub mod create_part {
                            pub use semio_s_artifact_puzzle_5d_mut_any_4::standards::v1::subsets::any::schema::mutations::create_part::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any_4::standards::v1::subsets::any::schema::mutations::create_part::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any_4::standards::v1::subsets::any::schema::mutations::create_part::inverse;
                            #[cfg(test)]
                            #[path = "../../../🦀️.rs"]
                            pub mod tests_appends_part_c;
                        }
                        #[path = "."]
                        pub mod delete_part {
                            pub use semio_s_artifact_puzzle_5d_mut_any_4::standards::v1::subsets::any::schema::mutations::delete_part::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any_4::standards::v1::subsets::any::schema::mutations::delete_part::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any_4::standards::v1::subsets::any::schema::mutations::delete_part::inverse;
                            #[cfg(test)]
                            #[path = "../../../../../../🗑️delete-part/🧪️tests/🚫️removes/🦀️.rs"]
                            pub mod tests_removes_part_a_and_severs_fastener;
                            #[cfg(test)]
                            #[path = "../../../../../../🗑️delete-part/🧪️tests/🗑️removes-middle-part/🦀️.rs"]
                            pub mod tests_removes_the_middle_part_and_its_fasteners;
                        }
                        #[path = "."]
                        pub mod move_part_2d {
                            pub use semio_s_artifact_puzzle_5d_diff::standards::v1::subsets::any::schema::mutations::move_part_2d::*;
                            pub use semio_s_artifact_puzzle_5d_diff::standards::v1::subsets::any::schema::mutations::move_part_2d::diff;
                            pub use semio_s_artifact_puzzle_5d_diff::standards::v1::subsets::any::schema::mutations::move_part_2d::inverse;
                        }
                        #[path = "."]
                        pub mod replace_part_2d_geometry {
                            pub use semio_s_artifact_puzzle_5d_mut_any::standards::v1::subsets::any::schema::mutations::replace_part_2d_geometry::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any::standards::v1::subsets::any::schema::mutations::replace_part_2d_geometry::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any::standards::v1::subsets::any::schema::mutations::replace_part_2d_geometry::inverse;
                        }
                        #[path = "."]
                        pub mod edit_part_2d_text {
                            pub use semio_s_artifact_puzzle_5d_mut_any::standards::v1::subsets::any::schema::mutations::edit_part_2d_text::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any::standards::v1::subsets::any::schema::mutations::edit_part_2d_text::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any::standards::v1::subsets::any::schema::mutations::edit_part_2d_text::inverse;
                        }
                        #[path = "."]
                        pub mod change_part_2d_icon {
                            pub use semio_s_artifact_puzzle_5d_mut_any::standards::v1::subsets::any::schema::mutations::change_part_2d_icon::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any::standards::v1::subsets::any::schema::mutations::change_part_2d_icon::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any::standards::v1::subsets::any::schema::mutations::change_part_2d_icon::inverse;
                        }
                        #[path = "."]
                        pub mod change_part_2d_hidden {
                            pub use semio_s_artifact_puzzle_5d_mut_any::standards::v1::subsets::any::schema::mutations::change_part_2d_hidden::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any::standards::v1::subsets::any::schema::mutations::change_part_2d_hidden::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any::standards::v1::subsets::any::schema::mutations::change_part_2d_hidden::inverse;
                        }
                        #[path = "."]
                        pub mod change_part_2d_locked {
                            pub use semio_s_artifact_puzzle_5d_mut_any::standards::v1::subsets::any::schema::mutations::change_part_2d_locked::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any::standards::v1::subsets::any::schema::mutations::change_part_2d_locked::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any::standards::v1::subsets::any::schema::mutations::change_part_2d_locked::inverse;
                        }
                        #[path = "."]
                        pub mod move_part_3d {
                            pub use semio_s_artifact_puzzle_5d_mut_any::standards::v1::subsets::any::schema::mutations::move_part_3d::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any::standards::v1::subsets::any::schema::mutations::move_part_3d::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any::standards::v1::subsets::any::schema::mutations::move_part_3d::inverse;
                        }
                        #[path = "."]
                        pub mod rotate_part_3d {
                            pub use semio_s_artifact_puzzle_5d_mut_any::standards::v1::subsets::any::schema::mutations::rotate_part_3d::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any::standards::v1::subsets::any::schema::mutations::rotate_part_3d::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any::standards::v1::subsets::any::schema::mutations::rotate_part_3d::inverse;
                        }
                        #[path = "."]
                        pub mod scale_part_3d {
                            pub use semio_s_artifact_puzzle_5d_mut_any::standards::v1::subsets::any::schema::mutations::scale_part_3d::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any::standards::v1::subsets::any::schema::mutations::scale_part_3d::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any::standards::v1::subsets::any::schema::mutations::scale_part_3d::inverse;
                        }
                        #[path = "."]
                        pub mod change_part_3d_mesh {
                            pub use semio_s_artifact_puzzle_5d_mut_any::standards::v1::subsets::any::schema::mutations::change_part_3d_mesh::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any::standards::v1::subsets::any::schema::mutations::change_part_3d_mesh::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any::standards::v1::subsets::any::schema::mutations::change_part_3d_mesh::inverse;
                        }
                        #[path = "."]
                        pub mod edit_part_3d_label {
                            pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::edit_part_3d_label::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::edit_part_3d_label::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::edit_part_3d_label::inverse;
                        }
                        #[path = "."]
                        pub mod change_part_kind {
                            pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_part_kind::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_part_kind::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_part_kind::inverse;
                        }
                        #[path = "."]
                        pub mod change_part_anchor {
                            pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_part_anchor::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_part_anchor::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_part_anchor::inverse;
                        }
                        #[path = "."]
                        pub mod add_part_grip {
                            pub use semio_s_artifact_puzzle_5d_mut_any_4::standards::v1::subsets::any::schema::mutations::add_part_grip::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any_4::standards::v1::subsets::any::schema::mutations::add_part_grip::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any_4::standards::v1::subsets::any::schema::mutations::add_part_grip::inverse;
                        }
                        #[path = "."]
                        pub mod remove_part_grip {
                            pub use semio_s_artifact_puzzle_5d_mut_any_4::standards::v1::subsets::any::schema::mutations::remove_part_grip::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any_4::standards::v1::subsets::any::schema::mutations::remove_part_grip::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any_4::standards::v1::subsets::any::schema::mutations::remove_part_grip::inverse;
                        }
                        #[path = "."]
                        pub mod replace_part_grip {
                            pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::replace_part_grip::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::replace_part_grip::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::replace_part_grip::inverse;
                        }
                        #[path = "."]
                        pub mod connect_grips {
                            pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::connect_grips::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::connect_grips::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::connect_grips::inverse;
                        }
                        #[path = "."]
                        pub mod disconnect_grips {
                            pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::disconnect_grips::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::disconnect_grips::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::disconnect_grips::inverse;
                        }
                        #[path = "."]
                        pub mod replace_fastener_geometry {
                            #[path = "../../../../../../🧮replace-fastener-geometry/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../../../🧮replace-fastener-geometry/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../../../🧮replace-fastener-geometry/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_fastener_kind {
                            pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_fastener_kind::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_fastener_kind::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_fastener_kind::inverse;
                        }
                        #[path = "."]
                        pub mod rename_puzzle5d {
                            pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::rename_puzzle5d::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::rename_puzzle5d::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::rename_puzzle5d::inverse;
                        }
                        #[path = "."]
                        pub mod change_domain {
                            pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_domain::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_domain::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::change_domain::inverse;
                        }
                        #[path = "."]
                        pub mod change_description {
                            pub use semio_s_artifact_puzzle_5d_mut_any_3::standards::v1::subsets::any::schema::mutations::change_description::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any_3::standards::v1::subsets::any::schema::mutations::change_description::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any_3::standards::v1::subsets::any::schema::mutations::change_description::inverse;
                        }
                        #[path = "."]
                        pub mod connect_kind_compatibility {
                            pub use semio_s_artifact_puzzle_5d_mut_any_3::standards::v1::subsets::any::schema::mutations::connect_kind_compatibility::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any_3::standards::v1::subsets::any::schema::mutations::connect_kind_compatibility::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any_3::standards::v1::subsets::any::schema::mutations::connect_kind_compatibility::inverse;
                        }
                        #[path = "."]
                        pub mod disconnect_kind_compatibility {
                            pub use semio_s_artifact_puzzle_5d_mut_any_3::standards::v1::subsets::any::schema::mutations::disconnect_kind_compatibility::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any_3::standards::v1::subsets::any::schema::mutations::disconnect_kind_compatibility::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any_3::standards::v1::subsets::any::schema::mutations::disconnect_kind_compatibility::inverse;
                        }
                        #[path = "."]
                        pub mod replace_kind_catalogs {
                            pub use semio_s_artifact_puzzle_5d_mut_any_3::standards::v1::subsets::any::schema::mutations::replace_kind_catalogs::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any_3::standards::v1::subsets::any::schema::mutations::replace_kind_catalogs::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any_3::standards::v1::subsets::any::schema::mutations::replace_kind_catalogs::inverse;
                        }
                        #[path = "."]
                        pub mod create_target_volume {
                            pub use semio_s_artifact_puzzle_5d_mut_any_3::standards::v1::subsets::any::schema::mutations::create_target_volume::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any_3::standards::v1::subsets::any::schema::mutations::create_target_volume::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any_3::standards::v1::subsets::any::schema::mutations::create_target_volume::inverse;
                        }
                        #[path = "."]
                        pub mod delete_target_volume {
                            pub use semio_s_artifact_puzzle_5d_mut_any_3::standards::v1::subsets::any::schema::mutations::delete_target_volume::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any_3::standards::v1::subsets::any::schema::mutations::delete_target_volume::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any_3::standards::v1::subsets::any::schema::mutations::delete_target_volume::inverse;
                        }
                        #[path = "."]
                        pub mod move_target_volume {
                            pub use semio_s_artifact_puzzle_5d_mut_any_3::standards::v1::subsets::any::schema::mutations::move_target_volume::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any_3::standards::v1::subsets::any::schema::mutations::move_target_volume::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any_3::standards::v1::subsets::any::schema::mutations::move_target_volume::inverse;
                        }
                        #[path = "."]
                        pub mod rotate_target_volume {
                            pub use semio_s_artifact_puzzle_5d_mut_any_3::standards::v1::subsets::any::schema::mutations::rotate_target_volume::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any_3::standards::v1::subsets::any::schema::mutations::rotate_target_volume::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any_3::standards::v1::subsets::any::schema::mutations::rotate_target_volume::inverse;
                        }
                        #[path = "."]
                        pub mod scale_target_volume {
                            pub use semio_s_artifact_puzzle_5d_mut_any_3::standards::v1::subsets::any::schema::mutations::scale_target_volume::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any_3::standards::v1::subsets::any::schema::mutations::scale_target_volume::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any_3::standards::v1::subsets::any::schema::mutations::scale_target_volume::inverse;
                        }
                        #[path = "."]
                        pub mod change_target_volume_hidden {
                            pub use semio_s_artifact_puzzle_5d_diff_2::standards::v1::subsets::any::schema::mutations::change_target_volume_hidden::*;
                            pub use semio_s_artifact_puzzle_5d_diff_2::standards::v1::subsets::any::schema::mutations::change_target_volume_hidden::diff;
                            pub use semio_s_artifact_puzzle_5d_diff_2::standards::v1::subsets::any::schema::mutations::change_target_volume_hidden::inverse;
                        }
                        #[path = "."]
                        pub mod change_target_volume_locked {
                            pub use semio_s_artifact_puzzle_5d_diff_2::standards::v1::subsets::any::schema::mutations::change_target_volume_locked::*;
                            pub use semio_s_artifact_puzzle_5d_diff_2::standards::v1::subsets::any::schema::mutations::change_target_volume_locked::diff;
                            pub use semio_s_artifact_puzzle_5d_diff_2::standards::v1::subsets::any::schema::mutations::change_target_volume_locked::inverse;
                        }
                        #[path = "."]
                        pub mod drag_selection_2d {
                            #[path = "../../../../../../✋️drag-selection2d/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../../../✋️drag-selection2d/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../../../✋️drag-selection2d/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod drag_selection_3d {
                            #[path = "../../../../../../🚚️drag-selection3d/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../../../🚚️drag-selection3d/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../../../🚚️drag-selection3d/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod rotate_selection_3d {
                            #[path = "../../../../../../🔄️rotate-selection3d/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../../../🔄️rotate-selection3d/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../../../🔄️rotate-selection3d/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod scale_selection_3d {
                            #[path = "../../../../../../🔍️scale-selection3d/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../../../🔍️scale-selection3d/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../../../🔍️scale-selection3d/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../../../../../../🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../../../../../../🚪️io/📝️text/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod puzzle5d {

    }
}
