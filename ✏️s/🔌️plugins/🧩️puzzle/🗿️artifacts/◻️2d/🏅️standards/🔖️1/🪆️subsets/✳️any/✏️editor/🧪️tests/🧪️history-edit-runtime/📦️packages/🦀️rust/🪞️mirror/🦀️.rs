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
                        pub mod connect_handles {
                            pub use semio_s_artifact_puzzle_2d_mut_any_4::standards::v1::subsets::any::schema::mutations::connect_handles::*;
                            pub use semio_s_artifact_puzzle_2d_mut_any_4::standards::v1::subsets::any::schema::mutations::connect_handles::diff;
                            pub use semio_s_artifact_puzzle_2d_mut_any_4::standards::v1::subsets::any::schema::mutations::connect_handles::inverse;
                        }
                    }
                }
            }
        }
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
        pub use semio_s_artifact_puzzle_2d_interaction::editor::puzzle2d::interaction;
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
        pub use semio_s_artifact_puzzle_2d_records::editor::puzzle2d::terminology;
        #[path = "."]
        pub mod engine {
            pub use semio_s_artifact_puzzle_2d_engine::editor::puzzle2d::engine::*;
            pub use semio_s_artifact_puzzle_2d_board_host::editor::puzzle2d::engine::board_host;
        }
        #[path = "."]
        pub mod precompute {
            pub use semio_s_artifact_puzzle_2d_tests::editor::puzzle2d::precompute::*;
            #[path = "."]
            pub mod fill {
                pub use semio_s_artifact_puzzle_2d_tests::editor::puzzle2d::precompute::fill::*;
                #[cfg(test)]
                #[path = "../../../../../⏳️precompute/🪣️fill/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
        }
        #[path = "."]
        pub mod commands {
            #[path = "."]
            pub mod create_edge {
                pub use semio_s_artifact_puzzle_2d_interaction::editor::puzzle2d::commands::create_edge::*;
                #[cfg(test)]
                #[path = "../../../../../🎮️commands/🧊️document/💞️create-edge/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod proximity_connect {
                pub use semio_s_artifact_puzzle_2d_interaction::editor::puzzle2d::commands::proximity_connect::*;
                #[cfg(test)]
                #[path = "../../../../../🎮️commands/🧊️document/🔗️proximity-connect/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod engagement_submit {
                pub use semio_s_artifact_puzzle_2d_retained::editor::puzzle2d::commands::engagement_submit::*;
                #[cfg(test)]
                #[path = "../../../../../🎮️commands/🎛️runtime/📨️engagement-submit/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            pub use semio_s_artifact_puzzle_2d_mut_any_10::editor::puzzle2d::commands::set_active_example;
        }
        #[cfg(test)]
        #[path = "../../../🦀️.rs"]
        pub mod history_edit_runtime_tests;
        #[path = "."]
        pub mod panels {
            pub use semio_s_artifact_puzzle_2d_retained_panels::editor::puzzle2d::panels::catalogue;
            #[path = "."]
            pub mod artifact {
                pub use semio_s_artifact_puzzle_2d_retained_panels::editor::puzzle2d::panels::artifact::*;
                #[cfg(test)]
                #[path = "../../../../../📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            pub use semio_s_artifact_puzzle_2d_retained_panels::editor::puzzle2d::panels::inspection;
            pub use semio_s_artifact_puzzle_2d_retained_panels::editor::puzzle2d::panels::settings;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_puzzle_2d_retained::editor::puzzle2d::modes::edit::*;
                pub use semio_s_artifact_puzzle_2d_tests::editor::puzzle2d::modes::edit::canvas;
                #[path = "."]
                pub mod options {
                    #[path = "."]
                    pub mod brush {
                        pub use semio_s_artifact_puzzle_2d_tests::editor::puzzle2d::modes::edit::options::brush::*;
                        #[cfg(test)]
                        #[path = "../../../../../🎭️modes/✏️edit/☑️options/🖌️brush/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                }
                #[path = "."]
                pub mod tools {
                    pub use semio_s_artifact_puzzle_2d_interaction::editor::puzzle2d::modes::edit::tools::*;
                    #[path = "."]
                    pub mod fill {
                        pub use semio_s_artifact_puzzle_2d_interaction::editor::puzzle2d::modes::edit::tools::fill::*;
                        #[cfg(test)]
                        #[path = "../../../../../🎭️modes/✏️edit/🛠️tools/🪣️fill/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                }
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_puzzle_2d_retained::editor::puzzle2d::modes::edit::windows::*;
                    #[path = "."]
                    pub mod overview {
                        pub use semio_s_artifact_puzzle_2d_retained::editor::puzzle2d::modes::edit::windows::overview::*;
                        #[cfg(test)]
                        #[path = "../../../../../🎭️modes/✏️edit/🪟️windows/👁️overview/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
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
