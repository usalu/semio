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
pub use semio_s_artifact_puzzle_5d_mut_any_5::standards::v1::subsets::any::schema::play_snapshot;

                    pub use semio_s_artifact_puzzle_5d_document_records::standards::v1::subsets::any::schema::document_records;
                    pub use semio_s_artifact_puzzle_5d_snapshot::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_puzzle_5d_mut_any_5::standards::v1::subsets::any::schema::mutations::*;
                        #[path = "."]
                        pub mod create_part {
                            pub use semio_s_artifact_puzzle_5d_mut_any_4::standards::v1::subsets::any::schema::mutations::create_part::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any_4::standards::v1::subsets::any::schema::mutations::create_part::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any_4::standards::v1::subsets::any::schema::mutations::create_part::inverse;
                        }
                        #[path = "."]
                        pub mod delete_part {
                            pub use semio_s_artifact_puzzle_5d_mut_any_4::standards::v1::subsets::any::schema::mutations::delete_part::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any_4::standards::v1::subsets::any::schema::mutations::delete_part::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any_4::standards::v1::subsets::any::schema::mutations::delete_part::inverse;
                        }
                        #[path = "."]
                        pub mod remove_part_grip {
                            pub use semio_s_artifact_puzzle_5d_mut_any_4::standards::v1::subsets::any::schema::mutations::remove_part_grip::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any_4::standards::v1::subsets::any::schema::mutations::remove_part_grip::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any_4::standards::v1::subsets::any::schema::mutations::remove_part_grip::inverse;
                        }
                        #[path = "."]
                        pub mod disconnect_grips {
                            pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::disconnect_grips::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::disconnect_grips::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::disconnect_grips::inverse;
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
    pub mod puzzle5d {
        #[path = "."]
        pub mod kit {
            #[path = "../../../🦀️.rs"]
            pub mod store_preparation;
        }

        pub use semio_s_artifact_puzzle_5d_mut_any_8::editor::puzzle5d::entities;
        pub use semio_s_artifact_puzzle_5d_tests::editor::puzzle5d::view;
        pub use semio_s_artifact_puzzle_5d_interaction::editor::puzzle5d::interaction;
        #[path = "."]
        pub mod config {
            pub use semio_s_artifact_puzzle_5d_tests::editor::puzzle5d::config::*;
        }
        pub use semio_s_artifact_puzzle_5d_tests::editor::puzzle5d::window;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_puzzle_5d_interaction::editor::puzzle5d::modes::edit::*;
                pub use semio_s_artifact_puzzle_5d_catalog::editor::puzzle5d::modes::edit::tools;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod world3d {
                        #[path = "."]
                        pub mod utilities {
                            pub use semio_s_artifact_puzzle_5d_catalog::editor::puzzle5d::modes::edit::windows::world3d::utilities::transform;
                        }
                    }
                }
            }
        }
    }
}
