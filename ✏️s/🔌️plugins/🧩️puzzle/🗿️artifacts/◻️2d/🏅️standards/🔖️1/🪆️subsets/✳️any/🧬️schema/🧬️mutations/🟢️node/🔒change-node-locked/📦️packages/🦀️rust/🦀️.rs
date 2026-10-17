//! 🔒 mut-any-2 of `semio-s-artifact-puzzle-2d` — a path-preserving carve of the artifact root mounts.

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
                        pub mod change_node_locked {
                            #[path = "../../🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_node_root {
                            #[path = "../../../🌟change-node-root/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../🌟change-node-root/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🌟change-node-root/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_node_anchor {
                            #[path = "../../../⚓change-node-anchor/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../⚓change-node-anchor/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../⚓change-node-anchor/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod replace_node_handle {
                            #[path = "../../../../🔌️handle/🔌replace-node-handle/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../🔌️handle/🔌replace-node-handle/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../🔌️handle/🔌replace-node-handle/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod replace_edge_geometry {
                            #[path = "../../../../〰️edge/🧮replace-edge-geometry/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../〰️edge/🧮replace-edge-geometry/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../〰️edge/🧮replace-edge-geometry/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_edge_kind {
                            #[path = "../../../../〰️edge/🏷️change-edge-kind/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../〰️edge/🏷️change-edge-kind/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../〰️edge/🏷️change-edge-kind/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_edge_tips {
                            #[path = "../../../../〰️edge/🖇️change-edge-tips/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../〰️edge/🖇️change-edge-tips/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../〰️edge/🖇️change-edge-tips/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_edge_visible {
                            #[path = "../../../../〰️edge/👀change-edge-visible/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../〰️edge/👀change-edge-visible/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../〰️edge/👀change-edge-visible/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_edge_locked {
                            #[path = "../../../../〰️edge/🔐change-edge-locked/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../〰️edge/🔐change-edge-locked/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../〰️edge/🔐change-edge-locked/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                    }
                }
            }
        }
    }
}
