//! ✏️ `jpg` edit (any) — Main window: editable `ImageWindowKit` render with region painting and the re-encode notice.

use crate::standards::v_jfif_1_01::subsets::document::io::encode_jpg;
use crate::standards::v_jfif_1_01::subsets::document::schema::snapshot::JpgSnapshot as JpgSnapshot;
use semio_framework_plugin::app::{ImageView, ImageWindowKit};
use semio_framework_plugin::{Buildable, BuiltNode, HasBase, WindowKindDefinition, WindowKit};
use semio_framework_ui_contract::{self as ui, Label};
use semio_framework_ui_locale::Locale;

pub const WINDOW_KIND_ID: &str = ImageWindowKit::KIND_ID;
pub const BODY_KEY: &str = ImageWindowKit::KIND_ID;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn definition() -> WindowKindDefinition {
    let mut definition = ImageWindowKit::editable_window_kind();
    definition.actions.push(crate::editor::jpg_any::paint_region::action());
    definition
}

/// ⚠️ Saving re-encodes the lossy scan; the notice states this limitation and what is preserved.
fn reencode_notice(locale: Locale) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let text = match locale {
        Locale::En => "Saving re-encodes the JPEG scan (lossy). Metadata segments such as EXIF and ICC profiles are preserved.",
        Locale::De => "Beim Speichern wird der JPEG-Scan neu kodiert (verlustbehaftet). Metadaten-Segmente wie EXIF und ICC-Profile bleiben erhalten.",
    };
    ui::text(Label::try_from(text).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("stdio.jpg.reencode-notice", "re-encode notice exceeds UI bound"))?)
        .try_id("jpg-reencode-notice")
        .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("stdio.jpg.reencode-notice", "re-encode notice id admission"))?
        .try_build()
        .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("stdio.jpg.reencode-notice", "re-encode notice build admission"))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn render(snapshot: &JpgSnapshot, locale: Locale) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    ImageWindowKit::render_with_accessory(&image_view(snapshot), reencode_notice(locale)?)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn image_view(snapshot: &JpgSnapshot) -> ImageView {
    let bytes = encode_jpg(snapshot, &crate::standards::v_jfif_1_01::subsets::document::io::JpgEncodeOptions::default()).ok().unwrap_or_default();
    ImageView { width: snapshot.image.width, height: snapshot.image.height, mime: "image/jpeg".into(), base64: semio_s_artifact_stdio_contract::base64_standard(&bytes) }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
