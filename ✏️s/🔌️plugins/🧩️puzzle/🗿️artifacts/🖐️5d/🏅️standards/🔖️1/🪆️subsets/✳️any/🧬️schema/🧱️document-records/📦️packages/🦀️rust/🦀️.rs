//! 🧱️ document-records of `semio-s-artifact-puzzle-5d` — a path-preserving carve of the artifact root mounts.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

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
                    #[path = "."]
                    pub mod graph_manifest {
                        #[path = "../../../../../../../../🤖️generated/🖐️puzzle5d-default/🦀️.rs"]
                        pub mod puzzle5d_default;
                    }
                    #[path = "../../🦀️.rs"]
                    pub mod document_records;
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../../🚪️io/💾️binary/💡️inferences/🦀️.rs"]
                        pub mod inferences;
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../../🚪️io/📝️text/💡️inferences/🦀️.rs"]
                        pub mod inferences;
                    }
                    #[path = "../../../../🚪️io/🪶️sqlite/🦀️.rs"]
                    pub mod sqlite;
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
        pub mod wasm {
            #[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
            #[path = "."]
            pub mod bridge {
                #[path = "../../../../../../../../../../🌉️wasm/⚠️diagnostic/🦀️.rs"]
                pub mod fault_bridge;
            }
        }
        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo_session {
                #[path = "../../../../✏️editor/📚️examples/🎬️demo-session/🦀️.rs"]
                mod component;
                pub use component::*;
                #[cfg(test)]
                #[path = "../../../../✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🦀️.rs"]
                pub mod tests;
            }
        }
        #[path = "."]
        pub mod config {
            #[path = "../../../../✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod presence {
            #[path = "../../../../✏️editor/👥️presence/🦀️.rs"]
            mod component;
            pub use component::*;
            #[path = "../../../../✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "../../../../✏️editor/🗣️terminology/🦀️.rs"]
        pub mod terminology;
        #[path = "."]
        pub mod precompute {
            #[path = "../../../../✏️editor/🧠️precompute/📐️geometry/🦀️.rs"]
            pub mod geometry;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod board2d {
                        #[path = "."]
                        pub mod actions {
                            #[path = "../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/◻️2d/🎬️actions/🧩️apply-board-events/🦀️.rs"]
                            pub mod apply_board_events;
                            #[path = "../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/◻️2d/🎬️actions/📷️set-camera/🦀️.rs"]
                            pub mod set_camera;
                        }
                        #[path = "."]
                        pub mod utilities {
                            #[path = "../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/◻️2d/🪛️utilities/🖱️select/🦀️.rs"]
                            pub mod select;
                        }
                    }
                    #[path = "."]
                    pub mod world3d {
                        #[path = "."]
                        pub mod actions {
                            #[path = "../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️3d/🎬️actions/🔄️rotate-selection/🦀️.rs"]
                            pub mod rotate_selection;
                        }
                    }
                }
            }
        }
    }
}
