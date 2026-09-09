//! ✏️ `tiff` edit (any) — Main window: real `ImageWindowKit`
//! render of the current document (editable variant).

use crate::standards::v6_0::subsets::document::io::encode_tiff;
use crate::standards::v6_0::subsets::document::schema::snapshot::TiffSnapshot;
use semio_framework_plugin::app::{ImageView, ImageWindowKit};
use semio_framework_plugin::{BuiltNode, WindowKindDefinition, WindowKit};

pub const WINDOW_KIND_ID: &str = ImageWindowKit::KIND_ID;
pub const BODY_KEY: &str = ImageWindowKit::KIND_ID;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn definition() -> WindowKindDefinition {
    ImageWindowKit::editable_window_kind()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn render(snapshot: &TiffSnapshot) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    ImageWindowKit::render(&image_view(snapshot))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn image_view(snapshot: &TiffSnapshot) -> ImageView {
    let bytes = encode_tiff(snapshot).ok().unwrap_or_default();
    ImageView { width: 0, height: 0, mime: "image/tiff".into(), base64: semio_s_artifact_stdio_contract::base64_standard(&bytes) }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
