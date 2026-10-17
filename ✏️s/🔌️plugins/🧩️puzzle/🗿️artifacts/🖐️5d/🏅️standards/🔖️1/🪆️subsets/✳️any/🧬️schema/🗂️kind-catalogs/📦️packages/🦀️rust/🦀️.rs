//! 🗂️ kind-catalogs of `semio-s-artifact-puzzle-5d` — a path-preserving carve of the artifact root mounts.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as dsl;
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
                    pub use semio_s_artifact_puzzle_5d_document_records::standards::v1::subsets::any::schema::document_records;
                    #[path = "../../🦀️.rs"]
                    pub mod kind_catalogs;
                    #[path = "../../../../../../../../🤖️generated/📇️registry/🦀️.rs"]
                    pub mod graph_manifest;
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../../🧬️mutations/🧰️support/🦀️.rs"]
                        pub mod support;
                    }
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
        pub mod terminology {
            pub use semio_s_artifact_puzzle_5d_document_records::editor::puzzle5d::terminology::*;
            #[cfg(test)]
            #[path = "../../../../✏️editor/🗣️terminology/🧪️tests/🔬️unit/🦀️.rs"]
            pub mod tests;
        }
        #[path = "."]
        pub mod precompute {
            #[path = "."]
            pub mod geometry {
                pub use semio_s_artifact_puzzle_5d_document_records::editor::puzzle5d::precompute::geometry::*;
                #[cfg(test)]
                #[path = "../../../../✏️editor/🧠️precompute/📐️geometry/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod world3d {
                        #[path = "."]
                        pub mod actions {
                            #[path = "../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️3d/🎬️actions/📐️scale-selection/🦀️.rs"]
                            pub mod scale_selection;
                            #[path = "../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️3d/🎬️actions/📷️set-camera/🦀️.rs"]
                            pub mod set_camera;
                            #[path = "../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️3d/🎬️actions/↔️translate-selection/🦀️.rs"]
                            pub mod translate_selection;
                            #[path = "../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️3d/🎬️actions/🚚️world-relocate/🦀️.rs"]
                            pub mod world_relocate;
                        }
                        #[path = "."]
                        pub mod utilities {
                            #[path = "../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️3d/🪛️utilities/🚚️world-relocate/🦀️.rs"]
                            pub mod world_relocate;
                        }
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod puzzle5d {
        #[path = "."]
        pub mod presence {
            #[path = "../../../../👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "../../../../👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
    }
}
