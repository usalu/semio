//! 🔺️ mut-any-2 of `semio-s-artifact-puzzle-5d` — a path-preserving carve of the artifact root mounts.

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
                        pub mod edit_part_3d_label {
                            #[path = "../../../🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_part_kind {
                            #[path = "../../../../🏗️change-part-kind/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../🏗️change-part-kind/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../🏗️change-part-kind/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_part_anchor {
                            #[path = "../../../../⚓change-part-anchor/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../⚓change-part-anchor/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../⚓change-part-anchor/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod replace_part_grip {
                            #[path = "../../../../🔌replace-part-grip/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../🔌replace-part-grip/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../🔌replace-part-grip/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod connect_grips {
                            #[path = "../../../../🪢️connect-grips/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../🪢️connect-grips/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../🪢️connect-grips/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod disconnect_grips {
                            #[path = "../../../../✂️disconnect-grips/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../✂️disconnect-grips/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../✂️disconnect-grips/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_fastener_kind {
                            #[path = "../../../../🎯change-fastener-kind/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../🎯change-fastener-kind/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../🎯change-fastener-kind/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod rename_puzzle5d {
                            #[path = "../../../../🏷️rename-puzzle5d/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../🏷️rename-puzzle5d/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../🏷️rename-puzzle5d/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_domain {
                            #[path = "../../../../🌐change-domain/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../🌐change-domain/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../🌐change-domain/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                    }
                }
            }
        }
    }
}
