//! 🚪️ io of `semio-s-artifact-stdio-semio-mesh` — a path-preserving carve of the artifact root mounts.

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
    pub use semio_s_artifact_stdio_semio_mesh_schema::schema::*;
    pub use semio_s_artifact_stdio_semio_mesh_snapshot::schema::snapshot;
    pub use semio_s_artifact_stdio_semio_mesh_examples::schema::diff;
    pub use semio_s_artifact_stdio_semio_mesh_examples::schema::inferences;
    #[path = "."]
    pub mod mutations {
        pub use semio_s_artifact_stdio_semio_mesh_mutations::schema::mutations::*;
        pub use semio_s_artifact_stdio_semio_mesh_schema::schema::mutations::*;
        pub use semio_s_artifact_stdio_semio_mesh_mutations::schema::mutations::delete_texture;
        #[cfg(test)]
        #[path = "../../../🧬️schema/🧬️mutations/🧪️tests/🔬️fixture/🦀️.rs"]
        mod fixture_tests;
    }
}
#[path = "."]
pub mod editor {
    #[path = "../../../✏️editor/🧭️edit-rules/🦀️.rs"]
    pub mod edit_rules;
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
            pub use semio_s_artifact_stdio_semio_mesh_schema::viewer::modes::view::*;
            pub use semio_s_artifact_stdio_semio_mesh_schema::viewer::modes::view::windows;
        }
    }
}
