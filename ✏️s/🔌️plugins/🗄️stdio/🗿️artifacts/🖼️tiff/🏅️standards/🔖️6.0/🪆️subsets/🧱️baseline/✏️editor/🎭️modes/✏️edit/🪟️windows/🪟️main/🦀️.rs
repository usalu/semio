//! ✏️ `tiff` edit (baseline) — Main window: real `ImageWindowKit`
//! render of the current document (read-only native canvas; typed edits live in Details).

use crate::standards::v6_0::subsets::baseline::schema::snapshot::TiffSnapshot;
use crate::standards::v6_0::subsets::document::io::encode_tiff_page_png;
use semio_framework_plugin::app::{ImageView, ImageWindowKit};
use semio_framework_plugin::{BuiltNode, WindowKindDefinition, WindowKit};
use semio_framework_ui_locale::Locale;

pub const WINDOW_KIND_ID: &str = ImageWindowKit::KIND_ID;
pub const BODY_KEY: &str = ImageWindowKit::KIND_ID;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn definition() -> WindowKindDefinition {
    ImageWindowKit::window_kind()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn render(snapshot: &TiffSnapshot, locale: Locale) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    match image_view(snapshot) {
        Ok(view) => ImageWindowKit::render(&view),
        Err(_) => ImageWindowKit::render_unavailable(locale),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn image_view(snapshot: &TiffSnapshot) -> semio_framework_plugin::UiAssemblyResult<ImageView> {
    let page = encode_tiff_page_png(snapshot, 0).map_err(|message| semio_framework_plugin::PluginAssemblyError::new("stdio.tiff.preview", message))?;
    Ok(ImageView { width: page.width, height: page.height, mime: "image/png".into(), base64: semio_s_artifact_stdio_contract::base64_standard(&page.bytes) })
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
