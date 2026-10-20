//! 📐 mut-family-2 of `semio-s-artifact-stdio-semio-drawing` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod schema {
    pub use semio_s_artifact_stdio_semio_drawing_snapshot::schema::snapshot;
    pub use semio_s_artifact_stdio_semio_drawing_inferences::schema::diff;
    #[path = "."]
    pub mod inferences {
        pub use semio_s_artifact_stdio_semio_drawing_inferences::schema::inferences::*;
        #[path = "."]
        pub mod flattened_scene {
            pub use semio_s_artifact_stdio_semio_drawing_inferences::schema::inferences::flattened_scene::*;
            #[cfg(test)]
            #[path = "../../../../💡️inferences/🎛️flattened-scene/🧪️tests/🔬️unit/🦀️.rs"]
            pub mod tests;
        }
    }
    #[path = "."]
    pub mod mutations {
        pub use semio_s_artifact_stdio_semio_drawing_inferences::schema::mutations::unflatten_node;
        #[path = "."]
        pub mod change_stroke_width {
            #[path = "../../🦀️.rs"]
            mod component;
            #[path = "../../🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod reorder_nodes {
            #[path = "../../../🔀reorder-nodes/🦀️.rs"]
            mod component;
            #[path = "../../../🔀reorder-nodes/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../../🔀reorder-nodes/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod rotate_node {
            #[path = "../../../🔄rotate-node/🦀️.rs"]
            mod component;
            #[path = "../../../🔄rotate-node/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../../🔄rotate-node/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod change_stroke_color {
            #[path = "../../../🖌️change-stroke-color/🦀️.rs"]
            mod component;
            #[path = "../../../🖌️change-stroke-color/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../../🖌️change-stroke-color/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod drag_nodes {
            #[path = "../../../🖐️drag-nodes/🦀️.rs"]
            mod component;
            #[path = "../../../🖐️drag-nodes/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../../🖐️drag-nodes/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod replace_path {
            #[path = "../../../🛤️replace-path/🦀️.rs"]
            mod component;
            #[path = "../../../🛤️replace-path/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../../🛤️replace-path/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod replace_fill {
            #[path = "../../../🪣replace-fill/🦀️.rs"]
            mod component;
            #[path = "../../../🪣replace-fill/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../../🪣replace-fill/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod flatten_node {
            #[path = "../../../🫓flatten-node/🦀️.rs"]
            mod component;
            #[path = "../../../🫓flatten-node/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../../🫓flatten-node/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod modes {
        #[path = "."]
        pub mod view {
            #[path = "../../../../../👁️viewer/🎭️modes/👁️view/🦀️.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod windows {
                pub use semio_s_artifact_stdio_semio_drawing_schema::viewer::modes::view::windows::*;
                #[path = "."]
                pub mod main {
                    pub use semio_s_artifact_stdio_semio_drawing_schema::viewer::modes::view::windows::main::*;
                    #[cfg(test)]
                    #[path = "../../../../../👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🧪️tests/🔬️unit/🦀️.rs"]
                    pub mod tests;
                }
            }
        }
    }
}
