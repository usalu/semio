//! 📸️ snapshot of `semio-s-artifact-stdio-binary` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub use schema::snapshot::BinarySnapshot;
pub mod schema {
    pub use super::standards::v_raw::subsets::any::schema::*;
}
pub use crate::editor::binary::edit_rules::artifact_items::{STDIO_BINARY_DOCUMENT_SCHEMA};
pub use crate::editor::binary::edit_rules::artifact_items_2::{BINARY_ARTIFACT_SCHEMA_ID};
pub use crate::editor::binary::edit_rules::artifact_items_3::{ARTIFACT_DEFINITION_SCHEMA};
pub use crate::editor::binary::edit_rules::artifact_items_4::{definition};
pub use crate::editor::binary::edit_rules::artifact_items_5::{formats};
pub use crate::editor::binary::edit_rules::artifact_items_6::{native_codecs};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
