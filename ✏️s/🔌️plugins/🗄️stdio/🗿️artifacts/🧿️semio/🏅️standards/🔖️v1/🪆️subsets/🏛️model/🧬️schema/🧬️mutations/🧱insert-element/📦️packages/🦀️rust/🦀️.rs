//! 🧱 mut-family of `semio-s-artifact-stdio-semio-model` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod schema {
    pub use semio_s_artifact_stdio_semio_model_snapshot::schema::snapshot;
    pub use semio_s_artifact_stdio_semio_model_examples::schema::diff;
    #[path = "."]
    pub mod mutations {
        #[path = "../../🦀️.rs"]
        pub mod insert_element;
        #[path = "../../../🪢insert-relation/🦀️.rs"]
        pub mod insert_relation;
        #[path = "../../../🏗️insert-spatial-node/🦀️.rs"]
        pub mod insert_spatial_node;
        #[path = "../../../🔨remove-element/🦀️.rs"]
        pub mod remove_element;
        #[path = "../../../✂️remove-relation/🦀️.rs"]
        pub mod remove_relation;
        #[path = "../../../🕳️remove-spatial-node/🦀️.rs"]
        pub mod remove_spatial_node;
        #[path = "../../../🔧️set-relation/🦀️.rs"]
        pub mod set_relation;
        #[path = "../../../🧭set-spatial-node/🦀️.rs"]
        pub mod set_spatial_node;
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod modes {
        #[path = "."]
        pub mod edit {
            #[path = "../../../../../✏️editor/🎭️modes/✏️edit/🦀️.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod windows {
                pub use semio_s_artifact_stdio_semio_model_examples::editor::modes::edit::windows::*;
                #[path = "."]
                pub mod main {
                    pub use semio_s_artifact_stdio_semio_model_examples::editor::modes::edit::windows::main::*;
                    #[cfg(test)]
                    #[path = "../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🧪️tests/🔬️unit/🦀️.rs"]
                    pub mod tests;
                }
            }
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
            pub use semio_s_artifact_stdio_semio_model_examples::viewer::modes::view::windows;
        }
    }
}
