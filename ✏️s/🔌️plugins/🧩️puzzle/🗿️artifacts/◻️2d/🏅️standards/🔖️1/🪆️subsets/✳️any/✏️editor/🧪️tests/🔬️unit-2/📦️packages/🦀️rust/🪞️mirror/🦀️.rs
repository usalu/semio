//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        pub use semio_s_artifact_puzzle_2d_v1::standards::v1::*;
        #[cfg(test)]
        #[path = "../../../../🤖️agent-lane/🦀️.rs"]
        pub mod agent_lane_tests;
        #[cfg(test)]
        #[path = "../../../../🔬️clipboard/🦀️.rs"]
        pub mod clipboard_tests;
        #[cfg(test)]
        #[path = "../../../../🔬️locks/🦀️.rs"]
        pub mod lock_tests;
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                pub use semio_s_artifact_puzzle_2d_v1::standards::v1::subsets::any::*;
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_puzzle_2d_schema::standards::v1::subsets::any::schema::*;
                    pub use semio_s_artifact_puzzle_2d_records::standards::v1::subsets::any::schema::records;
                    pub use semio_s_artifact_puzzle_2d_snapshot::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_puzzle_2d_inferences::standards::v1::subsets::any::schema::inferences;
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
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_puzzle_2d_snapshot::standards::v1::subsets::any::io::text::snapshot;
                        pub use semio_s_artifact_puzzle_2d_mutations::standards::v1::subsets::any::io::text::mutations;
                        pub use semio_s_artifact_puzzle_2d_mut_any_4::standards::v1::subsets::any::io::text::diff;
                    }
                    #[path = "."]
                    pub mod binary {
                        pub use semio_s_artifact_puzzle_2d_snapshot::standards::v1::subsets::any::io::binary::snapshot;
                        pub use semio_s_artifact_puzzle_2d_mutations::standards::v1::subsets::any::io::binary::mutations;
                    }
                }
            }
        }
    }
}
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
        pub use semio_s_artifact_puzzle_2d_mut_any_10::editor::puzzle2d::snapshot;
        pub use semio_s_artifact_puzzle_2d_text_inverse::editor::puzzle2d::recorder;
        pub use semio_s_artifact_puzzle_2d_interaction::editor::puzzle2d::interaction;
        pub use semio_s_artifact_puzzle_2d_text_inverse::editor::puzzle2d::kit;
        pub use semio_s_artifact_puzzle_2d_retained::editor::puzzle2d::retained;
        #[path = "."]
        pub mod config {
            pub use semio_s_artifact_puzzle_2d_records::editor::puzzle2d::config::*;
            pub use semio_s_artifact_puzzle_2d_records::editor::puzzle2d::config::schema;
        }
        pub use semio_s_artifact_puzzle_2d_snapshot::editor::puzzle2d::window;
        #[path = "."]
        pub mod presence {
            pub use semio_s_artifact_puzzle_2d_records::editor::puzzle2d::presence::*;
        }
        pub use semio_s_artifact_puzzle_2d_records::editor::puzzle2d::terminology;
        #[path = "."]
        pub mod engine {
            pub use semio_s_artifact_puzzle_2d_engine::editor::puzzle2d::engine::*;
            pub use semio_s_artifact_puzzle_2d_board_host::editor::puzzle2d::engine::board_host;
        }
        pub use semio_s_artifact_puzzle_2d_tests::editor::puzzle2d::precompute;
        #[path = "."]
        pub mod commands {
            pub use semio_s_artifact_puzzle_2d_mut_any_10::editor::puzzle2d::commands::set_active_example;
            #[path = "."]
            pub mod set_fill_count {
                pub use semio_s_artifact_puzzle_2d_interaction::editor::puzzle2d::commands::set_fill_count::*;
            }
        }
        #[path = "."]
        pub mod panels {
            #[path = "."]
            pub mod catalogue {
                pub use semio_s_artifact_puzzle_2d_retained::editor::puzzle2d::panels::catalogue::*;
            }
            pub use semio_s_artifact_puzzle_2d_retained::editor::puzzle2d::panels::artifact;
            #[path = "."]
            pub mod inspection {
                pub use semio_s_artifact_puzzle_2d_retained::editor::puzzle2d::panels::inspection::*;
            }
            pub use semio_s_artifact_puzzle_2d_retained::editor::puzzle2d::panels::settings;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_puzzle_2d_retained::editor::puzzle2d::modes::edit::*;
                pub use semio_s_artifact_puzzle_2d_tests::editor::puzzle2d::modes::edit::canvas;
                pub use semio_s_artifact_puzzle_2d_interaction::editor::puzzle2d::modes::edit::tools;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod overview {
                        pub use semio_s_artifact_puzzle_2d_retained::editor::puzzle2d::modes::edit::windows::overview::*;
                        #[path = "."]
                        pub mod utilities {
                            pub use semio_s_artifact_puzzle_2d_graph_manifest::editor::puzzle2d::modes::edit::windows::overview::utilities::brush;
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
    }
}
