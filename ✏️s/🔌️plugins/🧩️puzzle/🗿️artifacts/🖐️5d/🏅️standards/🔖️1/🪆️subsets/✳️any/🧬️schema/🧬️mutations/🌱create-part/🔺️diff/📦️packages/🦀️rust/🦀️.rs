//! 🔺️ mut-any-4 of `semio-s-artifact-puzzle-5d` — a path-preserving carve of the artifact root mounts.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_value_derive as value_derive;

pub use crate::standards::v1::subsets::any::schema::snapshot::Puzzle5dSnapshot;

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
                    pub use semio_s_artifact_puzzle_5d_document_records::standards::v1::subsets::any::schema::document_records;
                    pub use semio_s_artifact_puzzle_5d_snapshot::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod inferences {
                        pub use semio_s_artifact_puzzle_5d_diff::standards::v1::subsets::any::schema::inferences::*;
                    }
                    pub use semio_s_artifact_puzzle_5d_diff::standards::v1::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod create_part {
                            #[path = "../../../🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod delete_part {
                            #[path = "../../../../🗑️delete-part/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../🗑️delete-part/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../🗑️delete-part/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod add_part_grip {
                            #[path = "../../../../➕add-part-grip/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../➕add-part-grip/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../➕add-part-grip/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod remove_part_grip {
                            #[path = "../../../../➖remove-part-grip/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../➖remove-part-grip/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../➖remove-part-grip/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod connect_grips {
                            pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::connect_grips::*;
                            pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::connect_grips::diff;
                            pub use semio_s_artifact_puzzle_5d_mut_any_2::standards::v1::subsets::any::schema::mutations::connect_grips::inverse;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_puzzle_5d_snapshot::standards::v1::subsets::any::io::text::snapshot;
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod puzzle5d {
        #[cfg(test)]
        #[path = "../../../../../../📚️examples/🏗️nakagin-capsule-tower/🧪️tests/🧩️example/🦀️.rs"]
        pub mod nakagin_capsule_tower_tests;
    }
}
