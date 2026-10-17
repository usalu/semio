//! 🚪️ io of `semio-s-artifact-stdio-semio-brep` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

#[path = "."]
pub mod io {
    #[path = "../../🦀️.rs"]
    mod component;
    pub use component::*;
}
#[path = "."]
pub mod schema {
    pub use semio_s_artifact_stdio_semio_brep_schema::schema::*;
    #[path = "."]
    pub mod snapshot {
        pub use semio_s_artifact_stdio_semio_brep_snapshot::schema::snapshot::*;
    }
    pub use semio_s_artifact_stdio_semio_brep_examples::schema::diff;
    pub use semio_s_artifact_stdio_semio_brep_examples::schema::inferences;
    #[path = "."]
    pub mod mutations {
        pub use semio_s_artifact_stdio_semio_brep_viewer::schema::mutations::*;
        pub use semio_s_artifact_stdio_semio_brep_mut_family::schema::mutations::move_vertex;
        #[cfg(test)]
        #[path = "../../../🧬️schema/🧬️mutations/🧪️tests/🔬️fixture/🦀️.rs"]
        pub mod fixture_tests;
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
            #[path = "../../../✏️editor/🎭️modes/✏️edit/🦀️.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod windows {
                #[path = "."]
                pub mod main {
                    #[path = "../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                    mod component;
                    pub use component::*;
                }
            }
        }
    }
    pub use semio_s_artifact_stdio_semio_brep_viewer::editor::edit_rules;
}
#[path = "."]
pub mod viewer {
    pub use semio_s_artifact_stdio_semio_brep_viewer::viewer::*;
}
