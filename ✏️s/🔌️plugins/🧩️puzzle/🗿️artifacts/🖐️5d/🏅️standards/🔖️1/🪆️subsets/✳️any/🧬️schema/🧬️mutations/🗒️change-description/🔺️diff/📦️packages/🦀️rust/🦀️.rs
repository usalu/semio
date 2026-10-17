//! 🔺️ mut-any-3 of `semio-s-artifact-puzzle-5d` — a path-preserving carve of the artifact root mounts.

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
                    pub use semio_s_artifact_puzzle_5d_kind_catalogs::standards::v1::subsets::any::schema::kind_catalogs;
                    pub use semio_s_artifact_puzzle_5d_snapshot::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_puzzle_5d_diff::standards::v1::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod change_description {
                            #[path = "../../../🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod connect_kind_compatibility {
                            #[path = "../../../../🤝connect-kind-compatibility/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../🤝connect-kind-compatibility/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../🤝connect-kind-compatibility/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod disconnect_kind_compatibility {
                            #[path = "../../../../💔disconnect-kind-compatibility/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../💔disconnect-kind-compatibility/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../💔disconnect-kind-compatibility/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod replace_kind_catalogs {
                            #[path = "../../../../📚replace-kind-catalogs/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../📚replace-kind-catalogs/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../📚replace-kind-catalogs/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod create_target_volume {
                            #[path = "../../../../🌍create-target-volume/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../🌍create-target-volume/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../🌍create-target-volume/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod delete_target_volume {
                            #[path = "../../../../🪦delete-target-volume/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../🪦delete-target-volume/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../🪦delete-target-volume/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod move_target_volume {
                            #[path = "../../../../🚀move-target-volume/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../🚀move-target-volume/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../🚀move-target-volume/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod rotate_target_volume {
                            #[path = "../../../../🌀rotate-target-volume/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../🌀rotate-target-volume/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../🌀rotate-target-volume/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod scale_target_volume {
                            #[path = "../../../../📐scale-target-volume/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../📐scale-target-volume/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../📐scale-target-volume/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                    }
                }
            }
        }
    }
}
