//! 📸️ snapshot of `semio-s-artifact-puzzle-2d` — a path-preserving carve of the artifact root mounts.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
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
                    #[path = "../../🔎️lookup/🦀️.rs"]
                    pub mod lookup;
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "../../🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../../🚪️io/💾️binary/📸️snapshot/🦀️.rs"]
                        pub mod snapshot;
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../../🚪️io/📝️text/📸️snapshot/🦀️.rs"]
                        pub mod snapshot;
                    }
                    #[path = "."]
                    pub mod sqlite {
                        #[path = "../../../../🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs"]
                        pub mod snapshot;
                    }
                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod txt {
                                    #[path = "."]
                                    pub mod v_utf_8 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../../🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod txt {
                                    #[path = "."]
                                    pub mod v_utf_8 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../../🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                            }
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
        #[path = "."]
        pub mod wasm {
            #[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
            #[path = "../../../../✏️editor/🌉️wasm/🌉️bridge/🦀️.rs"]
            pub mod bridge;
        }
        #[path = "."]
        pub mod config {
            pub use semio_s_artifact_puzzle_2d_records::editor::puzzle2d::config::*;
            #[cfg(test)]
            #[path = "../../../../✏️editor/🎚️config/🧪️tests/🔬️unit/🦀️.rs"]
            pub mod tests;
        }
        #[path = "../../../../✏️editor/🪟️window/🦀️.rs"]
        pub mod window;
        #[path = "."]
        pub mod terminology {
            pub use semio_s_artifact_puzzle_2d_records::editor::puzzle2d::terminology::*;
            #[cfg(test)]
            #[path = "../../../../✏️editor/🗣️terminology/🧪️tests/🔬️unit/🦀️.rs"]
            pub mod tests;
        }
        #[path = "."]
        pub mod engine {
            pub use semio_s_artifact_puzzle_2d_engine::editor::puzzle2d::engine::*;
            #[path = "."]
            pub mod icons {
                pub use semio_s_artifact_puzzle_2d_engine::editor::puzzle2d::engine::icons::*;
                #[path = "../../../../✏️editor/⚙️engine/🔣️icons/🌱️metabolism/🦀️.rs"]
                pub mod board_metabolism_icons;
            }
            pub use semio_s_artifact_puzzle_2d_board_host::editor::puzzle2d::engine::board_host;
        }
    }
}
