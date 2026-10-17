//! 🔺️ diff-2 of `semio-s-artifact-puzzle-5d` — a path-preserving carve of the artifact root mounts.

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
                    pub use semio_s_artifact_puzzle_5d_schema::standards::v1::subsets::any::schema::*;
                    pub use semio_s_artifact_puzzle_5d_document_records::standards::v1::subsets::any::schema::document_records;
                    pub use semio_s_artifact_puzzle_5d_kind_catalogs::standards::v1::subsets::any::schema::kind_catalogs;
                    pub use semio_s_artifact_puzzle_5d_snapshot::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod inferences {
                        pub use semio_s_artifact_puzzle_5d_diff::standards::v1::subsets::any::schema::inferences::*;
                        pub use semio_s_artifact_puzzle_5d_diff::standards::v1::subsets::any::schema::inferences::flat_position;
                    }
                    pub use semio_s_artifact_puzzle_5d_diff::standards::v1::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod change_target_volume_hidden {
                            #[path = "../../../🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_target_volume_locked {
                            #[path = "../../../../🔐change-target-volume-locked/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../🔐change-target-volume-locked/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../🔐change-target-volume-locked/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                    }
                    #[path = "."]
                    pub mod transfer {
                        pub use semio_s_artifact_puzzle_5d_diff::standards::v1::subsets::any::schema::transfer::*;
                        pub use semio_s_artifact_puzzle_5d_diff::standards::v1::subsets::any::schema::transfer::*;
                        #[cfg(test)]
                        #[path = "../../../../../✂️transfer/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../../../../🚪️io/💾️binary/🔺️diff/🦀️.rs"]
                        pub mod diff;
                    }
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_puzzle_5d_snapshot::standards::v1::subsets::any::io::text::snapshot;
                        #[path = "../../../../../../🚪️io/📝️text/🔺️diff/🦀️.rs"]
                        pub mod diff;
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
        #[path = "../../../../../../📚️examples/🌙️capsule-dream/🦀️.rs"]
        pub mod capsule_dream;
        #[path = "../../../../../../📚️examples/🌲️concrete-forest/🦀️.rs"]
        pub mod concrete_forest;
        #[path = "../../../../../../📚️examples/🏗️nakagin-capsule-tower/🦀️.rs"]
        pub mod nakagin_capsule_tower;
        #[cfg(test)]
        #[path = "../../../../../../📚️examples/🌙️capsule-dream/🧪️tests/🧩️example/🦀️.rs"]
        pub mod capsule_dream_tests;
        #[cfg(test)]
        #[path = "../../../../../../📚️examples/🌲️concrete-forest/🧪️tests/🧩️example/🦀️.rs"]
        pub mod concrete_forest_tests;
    }
}
