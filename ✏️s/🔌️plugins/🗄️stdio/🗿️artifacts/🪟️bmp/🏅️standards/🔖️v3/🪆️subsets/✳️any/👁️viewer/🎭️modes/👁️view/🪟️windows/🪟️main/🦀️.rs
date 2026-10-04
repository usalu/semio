//! 👁️ `bmp` view (any) — browser-safe PNG projection of canonical BMP bytes.

use crate::standards::v_v3::subsets::any::io::bmp_png_preview;
use crate::standards::v_v3::subsets::any::schema::snapshot::BmpSnapshot;
use semio_framework_plugin::app::{ImageView, ImageWindowKit};
use semio_framework_plugin::{BuiltNode, WindowKindDefinition, WindowKit};
use semio_framework_ui_locale::Locale;

pub const WINDOW_KIND_ID: &str = ImageWindowKit::KIND_ID;
pub const BODY_KEY: &str = ImageWindowKit::KIND_ID;

pub fn definition() -> WindowKindDefinition {
    ImageWindowKit::window_kind()
}

pub fn render(snapshot: &BmpSnapshot, locale: Locale) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    match image_view(snapshot) {
        Ok(view) => ImageWindowKit::render(&view),
        Err(_) => ImageWindowKit::render_unavailable(locale),
    }
}

fn image_view(snapshot: &BmpSnapshot) -> semio_framework_plugin::UiAssemblyResult<ImageView> {
    let preview = bmp_png_preview(snapshot).map_err(|message| semio_framework_plugin::PluginAssemblyError::new("stdio.bmp.preview", message))?;
    Ok(ImageView { width: preview.width, height: preview.height, mime: "image/png".into(), base64: semio_s_artifact_stdio_contract::base64_standard(&preview.bytes) })
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
