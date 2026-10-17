//! 🧬️ mut-any of `semio-s-artifact-puzzle-2d` — a path-preserving carve of the artifact root mounts.

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
                        #[path = "../../✋️selection/🦀️.rs"]
                        pub mod selection;
                        #[path = "."]
                        pub mod move_node {
                            #[path = "../../🧭️pose/📍move-node/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../🧭️pose/📍move-node/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🧭️pose/📍move-node/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod replace_node_geometry {
                            #[path = "../../🟢️node/🧊replace-node-geometry/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../🟢️node/🧊replace-node-geometry/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🟢️node/🧊replace-node-geometry/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_node_kind {
                            #[path = "../../🟢️node/🏗️change-node-kind/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../🟢️node/🏗️change-node-kind/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🟢️node/🏗️change-node-kind/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod edit_node_text {
                            #[path = "../../🟢️node/✏️edit-node-text/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../🟢️node/✏️edit-node-text/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🟢️node/✏️edit-node-text/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_node_icon {
                            #[path = "../../🟢️node/🎨change-node-icon/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../🟢️node/🎨change-node-icon/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🟢️node/🎨change-node-icon/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod scale_node {
                            #[path = "../../🧭️pose/📏scale-node/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../🧭️pose/📏scale-node/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🧭️pose/📏scale-node/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_node_visible {
                            #[path = "../../🟢️node/👁️change-node-visible/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../🟢️node/👁️change-node-visible/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🟢️node/👁️change-node-visible/↩️inverse/🦀️.rs"]
                            pub mod inverse;
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
        #[path = "../../../../✏️editor/🌉️wasm/🦀️.rs"]
        pub mod wasm;
        #[path = "."]
        pub mod engine {
            pub use semio_s_artifact_puzzle_2d_engine::editor::puzzle2d::engine::*;
            #[path = "."]
            pub mod board_host {
                #[cfg(any(test, feature = "testkit"))]
                pub use semio_s_artifact_puzzle_2d_board_host::editor::puzzle2d::engine::board_host::testkit;
            }
            #[cfg(test)]
            #[path = "../../../../✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs"]
            pub mod linking;
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod puzzle2d {
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../../../👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_puzzle_2d_inferences::viewer::puzzle2d::modes::view::windows::*;
                    #[path = "."]
                    pub mod board {
                        pub use semio_s_artifact_puzzle_2d_inferences::viewer::puzzle2d::modes::view::windows::board::*;
                        #[cfg(test)]
                        #[path = "../../../../👁️viewer/🎭️modes/👁️view/🪟️windows/🎲️board/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                }
            }
        }
    }
}
