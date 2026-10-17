//! 🆔 mut-any-3 of `semio-s-artifact-puzzle-2d` — a path-preserving carve of the artifact root mounts.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_value_derive as value_derive;

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
                    pub use semio_s_artifact_puzzle_2d_schema::standards::v1::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod change_manifest_id {
                            #[path = "../../🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod connect_kind_compatibility {
                            #[path = "../../../🤝connect-kind-compatibility/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../🤝connect-kind-compatibility/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🤝connect-kind-compatibility/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod disconnect_kind_compatibility {
                            #[path = "../../../💔disconnect-kind-compatibility/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../💔disconnect-kind-compatibility/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../💔disconnect-kind-compatibility/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod replace_kind_catalogs {
                            #[path = "../../../📚replace-kind-catalogs/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../📚replace-kind-catalogs/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../📚replace-kind-catalogs/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod create_target_region {
                            #[path = "../../../../🌍️region/🌍create-target-region/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../🌍️region/🌍create-target-region/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../🌍️region/🌍create-target-region/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod delete_target_region {
                            #[path = "../../../../🌍️region/🪦delete-target-region/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../🌍️region/🪦delete-target-region/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../🌍️region/🪦delete-target-region/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod move_target_region {
                            #[path = "../../../../🌍️region/🚀move-target-region/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../🌍️region/🚀move-target-region/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../🌍️region/🚀move-target-region/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod resize_target_region {
                            #[path = "../../../../🌍️region/📐resize-target-region/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../🌍️region/📐resize-target-region/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../🌍️region/📐resize-target-region/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod edit_target_region_label {
                            #[path = "../../../../🌍️region/🖋️edit-target-region-label/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../🌍️region/🖋️edit-target-region-label/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../🌍️region/🖋️edit-target-region-label/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                    }
                }
            }
        }
    }
}
