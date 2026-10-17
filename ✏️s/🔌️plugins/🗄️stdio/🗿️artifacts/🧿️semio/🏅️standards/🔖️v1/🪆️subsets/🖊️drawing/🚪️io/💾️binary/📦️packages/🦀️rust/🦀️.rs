//! 💾️ binary of `semio-s-artifact-stdio-semio-drawing` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as protocol;

#[path = "."]
pub mod schema {
    pub use semio_s_artifact_stdio_semio_drawing_snapshot::schema::snapshot;
    pub use semio_s_artifact_stdio_semio_drawing_inferences::schema::diff;
    #[path = "."]
    pub mod mutations {
        pub use semio_s_artifact_stdio_semio_drawing_editor::schema::mutations::*;
        pub use semio_s_artifact_stdio_semio_drawing_inferences::schema::mutations::unflatten_node;
        #[cfg(test)]
        #[path = "../../../../🧬️schema/🧬️mutations/🧪️tests/🔬️fixture/🦀️.rs"]
        pub mod fixture_tests;
    }
}
#[path = "."]
pub mod io {
    #[path = "../../🦀️.rs"]
    pub mod binary;
    #[path = "../../../📝️text/🦀️.rs"]
    pub mod text;
}
