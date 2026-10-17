//! 🕸️ mut-family-2 of `semio-s-artifact-stdio-semio` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod schema {
    #[path = "."]
    pub mod mutations {
        #[path = "../../🦀️.rs"]
        pub mod apply_mesh;
        #[path = "../../../🏛️apply-model/🦀️.rs"]
        pub mod apply_model;
        #[path = "../../../📦apply-object/🦀️.rs"]
        pub mod apply_object;
        #[path = "../../../📽️apply-presentation/🦀️.rs"]
        pub mod apply_presentation;
        #[path = "../../../🗂️apply-table/🦀️.rs"]
        pub mod apply_table;
        #[path = "../../../🔤apply-text/🦀️.rs"]
        pub mod apply_text;
        #[path = "../../../🔢apply-value/🦀️.rs"]
        pub mod apply_value;
        #[path = "../../../🎬apply-video/🦀️.rs"]
        pub mod apply_video;
    }
    pub use semio_s_artifact_stdio_semio_snapshot::schema::snapshot;
    pub use semio_s_artifact_stdio_semio_inferences::schema::diff;
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
                pub use semio_s_artifact_stdio_semio_inferences::editor::modes::edit::windows::*;
                #[path = "."]
                pub mod main {
                    pub use semio_s_artifact_stdio_semio_inferences::editor::modes::edit::windows::main::*;
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
            pub use semio_s_artifact_stdio_semio_inferences::viewer::modes::view::windows;
        }
    }
}
