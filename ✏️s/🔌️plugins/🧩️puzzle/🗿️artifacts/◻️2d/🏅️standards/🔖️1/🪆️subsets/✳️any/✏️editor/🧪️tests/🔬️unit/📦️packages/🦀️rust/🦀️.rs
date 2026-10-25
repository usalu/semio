//! 🔬️ unit-tests of `semio-s-artifact-puzzle-2d` — a path-preserving carve of the artifact root mounts.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;

#[cfg(test)]
pub use crate::standards::v1::subsets::any::schema::snapshot::Puzzle2dSnapshot;

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
                    pub use semio_s_artifact_puzzle_2d_records::standards::v1::subsets::any::schema::records;
                    pub use semio_s_artifact_puzzle_2d_snapshot::standards::v1::subsets::any::schema::snapshot;
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
                        pub mod drag_selection {
                            pub use semio_s_artifact_puzzle_2d_mut_any_4::standards::v1::subsets::any::schema::mutations::drag_selection::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any_4::standards::v1::subsets::any::schema::mutations::drag_selection::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any_4::standards::v1::subsets::any::schema::mutations::drag_selection::inverse;
                        }
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
        #[cfg(any(test, feature = "testkit"))]
        pub use semio_s_artifact_puzzle_2d_v1::editor::puzzle2d::testkit;
        pub use semio_s_artifact_puzzle_2d_text_inverse::editor::puzzle2d::kit;
        pub use semio_s_artifact_puzzle_2d_retained::editor::puzzle2d::retained;
        pub use semio_s_artifact_puzzle_2d_v1::editor::puzzle2d::session;
        #[path = "."]
        pub mod config {
            pub use semio_s_artifact_puzzle_2d_records::editor::puzzle2d::config::*;
        }
        pub use semio_s_artifact_puzzle_2d_snapshot::editor::puzzle2d::window;
        #[path = "."]
        pub mod presence {
            pub use semio_s_artifact_puzzle_2d_records::editor::puzzle2d::presence::*;
        }
        #[path = "."]
        pub mod engine {
            pub use semio_s_artifact_puzzle_2d_engine::editor::puzzle2d::engine::*;
            pub use semio_s_artifact_puzzle_2d_board_host::editor::puzzle2d::engine::board_host;
        }
        pub use semio_s_artifact_puzzle_2d_tests::editor::puzzle2d::precompute;
        #[path = "."]
        pub mod commands {
            pub use semio_s_artifact_puzzle_2d_mut_any_10::editor::puzzle2d::commands::set_active_example;
            pub use semio_s_artifact_puzzle_2d_retained::editor::puzzle2d::commands::import_snapshot;
        }
        #[cfg(test)]
        #[path = "../../🦀️.rs"]
        pub mod unit_tests;
        #[cfg(test)]
        #[path = "../../../🧪️select-tool-transactions/🦀️.rs"]
        pub mod select_tool_transaction_tests;
        #[cfg(test)]
        #[path = "../../../🧪️board-tools/🦀️.rs"]
        pub mod board_tool_tests;
        #[cfg(test)]
        #[path = "../../../🧪️recorded-kinds/🦀️.rs"]
        pub mod recorded_kind_tests;
        #[cfg(test)]
        #[path = "../../../🧪️select-tool-history/🦀️.rs"]
        pub mod select_tool_history_tests;
        #[cfg(test)]
        #[path = "../../../🧵️retained-wiring/🦀️.rs"]
        pub mod retained_wiring_tests;
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
                pub use semio_s_artifact_puzzle_2d_tests::editor::puzzle2d::modes::edit::canvas;
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
pub use semio_s_artifact_puzzle_2d_mut_any_10::host;
