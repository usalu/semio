//! 🚪️ io of `semio-s-artifact-stdio-semio-image` — a path-preserving carve of the artifact root mounts.

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
    pub use semio_s_artifact_stdio_semio_image_schema::schema::*;
    pub use semio_s_artifact_stdio_semio_image_snapshot::schema::snapshot;
    pub use semio_s_artifact_stdio_semio_image_diff::schema::diff;
    pub use semio_s_artifact_stdio_semio_image_schema::schema::inferences;
    pub use semio_s_artifact_stdio_semio_image_editor::schema::mutations;
}
