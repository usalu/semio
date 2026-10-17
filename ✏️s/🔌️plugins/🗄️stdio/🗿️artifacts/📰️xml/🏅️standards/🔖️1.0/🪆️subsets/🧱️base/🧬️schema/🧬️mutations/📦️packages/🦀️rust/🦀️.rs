//! 🧬️ mutations of `semio-s-artifact-stdio-xml` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

pub use schema::snapshot::XmlSnapshot;
pub mod schema {
    pub use super::standards::v1_0::subsets::base::schema::*;
}
pub use semio_s_artifact_stdio_xml_inferences::{ARTIFACT_DEFINITION_SCHEMA, artifact_kind};
pub use semio_s_artifact_stdio_xml_snapshot::{STDIO_XML_DOCUMENT_SCHEMA};
pub use crate::editor::xml_any::modes::edit::artifact_items_5::{native_codec};
pub use crate::editor::xml_any::modes::edit::artifact_items_6::{native_codecs};
pub use crate::editor::xml_any::modes::edit::artifact_items_7::{definition};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
