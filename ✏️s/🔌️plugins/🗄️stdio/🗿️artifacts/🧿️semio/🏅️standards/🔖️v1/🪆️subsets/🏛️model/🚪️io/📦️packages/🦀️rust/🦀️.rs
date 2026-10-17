//! 🚪️ io of `semio-s-artifact-stdio-semio-model` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod io {
    #[path = "../../🦀️.rs"]
    mod component;
    pub use component::*;
}
#[path = "."]
pub mod schema {
    pub use semio_s_artifact_stdio_semio_model_schema::schema::*;
    pub use semio_s_artifact_stdio_semio_model_snapshot::schema::snapshot;
    pub use semio_s_artifact_stdio_semio_model_examples::schema::diff;
    pub use semio_s_artifact_stdio_semio_model_schema::schema::inferences;
    #[path = "."]
    pub mod mutations {
        pub use semio_s_artifact_stdio_semio_model_mutations::schema::mutations::*;
        #[cfg(test)]
        #[path = "../../../🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs"]
        pub mod tests;
    }
}
#[path = "."]
pub mod editor {
    #[path = "../../../✏️editor/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "."]
    pub mod modes {
        #[path = "."]
        pub mod edit {
            pub use semio_s_artifact_stdio_semio_model_mut_family::editor::modes::edit::*;
            pub use semio_s_artifact_stdio_semio_model_examples::editor::modes::edit::windows;
        }
    }
    pub use semio_s_artifact_stdio_semio_model_examples::editor::edit_rules;
}
#[path = "."]
pub mod viewer {
    #[path = "../../../👁️viewer/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "."]
    pub mod modes {
        #[path = "."]
        pub mod view {
            pub use semio_s_artifact_stdio_semio_model_mut_family::viewer::modes::view::*;
            #[path = "."]
            pub mod windows {
                pub use semio_s_artifact_stdio_semio_model_examples::viewer::modes::view::windows::*;
                #[path = "."]
                pub mod main {
                    pub use semio_s_artifact_stdio_semio_model_examples::viewer::modes::view::windows::main::*;
                    #[cfg(test)]
                    #[path = "../../../👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🧪️tests/🔬️unit/🦀️.rs"]
                    pub mod tests;
                }
            }
        }
    }
}
