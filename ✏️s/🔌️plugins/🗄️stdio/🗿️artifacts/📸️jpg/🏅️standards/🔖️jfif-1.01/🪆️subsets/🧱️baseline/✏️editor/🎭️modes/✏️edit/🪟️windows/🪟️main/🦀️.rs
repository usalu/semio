//! ✏️ `jpg` edit (baseline) — Main window: real `ImageWindowKit`
//! render of the current document (editable variant).

use crate::standards::v_jfif_1_01::subsets::baseline::schema::snapshot::JpgSnapshot;
use crate::standards::v_jfif_1_01::subsets::document::io::encode_jpg;
use semio_framework_plugin::app::{ImageView, ImageWindowKit};
use semio_framework_plugin::{BuiltNode, WindowKindDefinition, WindowKit};

pub const WINDOW_KIND_ID: &str = ImageWindowKit::KIND_ID;
pub const BODY_KEY: &str = ImageWindowKit::KIND_ID;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn definition() -> WindowKindDefinition {
    ImageWindowKit::editable_window_kind()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn render(snapshot: &JpgSnapshot) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    ImageWindowKit::render(&image_view(snapshot))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn image_view(snapshot: &JpgSnapshot) -> ImageView {
    let bytes = encode_jpg(snapshot).ok().unwrap_or_default();
    ImageView { width: snapshot.width, height: snapshot.height, mime: "image/jpeg".into(), base64: crate::base64_standard(&bytes) }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
