//! 🧬️ mut-any of `semio-s-artifact-puzzle-5d` — a path-preserving carve of the artifact root mounts.

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
                    pub use semio_s_artifact_puzzle_5d_diff::standards::v1::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod replace_part_2d_geometry {
                            #[path = "../../🧊replace-part2d-geometry/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../🧊replace-part2d-geometry/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🧊replace-part2d-geometry/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod edit_part_2d_text {
                            #[path = "../../✏️edit-part2d-text/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../✏️edit-part2d-text/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../✏️edit-part2d-text/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_part_2d_icon {
                            #[path = "../../🎨change-part2d-icon/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../🎨change-part2d-icon/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🎨change-part2d-icon/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_part_2d_hidden {
                            #[path = "../../🙈change-part2d-hidden/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../🙈change-part2d-hidden/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🙈change-part2d-hidden/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_part_2d_locked {
                            #[path = "../../🔒change-part2d-locked/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../🔒change-part2d-locked/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🔒change-part2d-locked/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod move_part_3d {
                            #[path = "../../🚀move-part3d/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../🚀move-part3d/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🚀move-part3d/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod rotate_part_3d {
                            #[path = "../../🔃rotate-part3d/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../🔃rotate-part3d/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🔃rotate-part3d/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod scale_part_3d {
                            #[path = "../../📏scale-part3d/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../📏scale-part3d/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../📏scale-part3d/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_part_3d_mesh {
                            #[path = "../../🧱change-part3d-mesh/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../🧱change-part3d-mesh/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🧱change-part3d-mesh/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                    }
                }
            }
        }
    }
}
