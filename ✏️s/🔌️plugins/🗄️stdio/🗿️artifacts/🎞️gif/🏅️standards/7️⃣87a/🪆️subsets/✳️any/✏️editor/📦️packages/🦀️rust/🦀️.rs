//! ✏️ editor of `semio-s-artifact-stdio-gif` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

pub(crate) use semio_s_artifact_stdio_contract::base64_standard;
pub use schema::snapshot::STDIO_GIF89A_DOCUMENT_SCHEMA;
pub mod schema {
    pub use super::standards::v89a::subsets::any::schema::*;
}
pub use semio_s_artifact_stdio_gif_inferences::{STDIO_GIF_DOCUMENT_SCHEMA, native_codecs};
pub use crate::editor::gif_87a::artifact_items_3::{ARTIFACT_DEFINITION_SCHEMA};
pub use crate::editor::gif_87a::artifact_items_4::{definition};
pub use crate::editor::gif_87a::artifact_items_5::{formats};
pub use crate::editor::gif_87a::artifact_items_7::{artifact_kind, assembly, declaration};
pub use crate::editor::gif_87a::artifact_items_8::{contribution};
pub use crate::editor::gif_87a::artifact_items_9::{GIF_87A_DIALECT, GIF_89A_DIALECT};
#[cfg(any(test, feature = "testkit"))]
pub use crate::editor::gif_87a::artifact_items_9::{register_sqlite_test_declaration};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
