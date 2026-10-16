//! 🎬️ examples of `semio-s-artifact-stdio-stl` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

#[cfg(test)]
pub use schema::snapshot::StlSnapshot;
#[cfg(test)]
pub use semio_s_artifact_stdio_contract::apply_mutation;
#[cfg(test)]
pub mod engine {
    pub use super::standards::v_ascii::engine::*;
}
#[cfg(test)]
pub mod schema {
    pub use super::standards::v_ascii::subsets::any::schema::*;
}
pub use semio_s_artifact_stdio_stl_inferences::{STDIO_STL_DOCUMENT_SCHEMA};
pub use semio_s_artifact_stdio_stl_viewer::{ARTIFACT_DEFINITION_SCHEMA, assembly, definition, formats, native_codecs};
pub use crate::editor::stl::artifact_items_10::{contribution};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
