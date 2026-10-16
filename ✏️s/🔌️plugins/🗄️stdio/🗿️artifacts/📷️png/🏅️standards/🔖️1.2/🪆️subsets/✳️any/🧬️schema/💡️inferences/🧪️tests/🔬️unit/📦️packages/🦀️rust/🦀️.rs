//! 🔬️ tests of `semio-s-artifact-stdio-png` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

pub use schema::snapshot::PngSnapshot;
pub(crate) use semio_s_artifact_stdio_contract::base64_standard;
#[cfg(test)]
pub use schema::mutations::PngMutation;
pub mod schema {
    pub use super::standards::v1_2::subsets::any::schema::*;
}
pub use semio_s_artifact_stdio_png_snapshot::{ARTIFACT_DEFINITION_SCHEMA, STDIO_PNG_DOCUMENT_SCHEMA, formats};
pub use semio_s_artifact_stdio_png_mutations::{native_codec};
pub use crate::editor::png::artifact_items_4::{artifact_kind};
pub use crate::editor::png::artifact_items_6::{native_codecs};
pub use crate::editor::png::artifact_items_7::{definition};
pub use crate::editor::png::artifact_items_9::{assembly, declaration, pilot_languages};
pub use crate::editor::png::artifact_items_10::{contribution};
pub use crate::editor::png::artifact_items_11::{PNG_DIALECT};
pub use crate::editor::png::artifact_items_12::{register};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
