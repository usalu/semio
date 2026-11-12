//! ✏️ `jpg` edit (baseline) — Main window: editable `ImageWindowKit` render with region painting and the re-encode notice.

use crate::standards::v_jfif_1_01::subsets::document::io::encode_jpg;
use crate::standards::v_jfif_1_01::subsets::baseline::schema::snapshot::JpgSnapshot as JpgSnapshot;
use semio_plugin::app::{ImageView, ImageWindowKit};
use semio_plugin::{Buildable, BuiltNode, HasBase, WindowKindDefinition, WindowKit};
use semio_plugin::plugin_app_close_prelude::{self as ui, Label};
use semio_ui_locale::{Locale, LocalizedLabel};
use semio_plugin::{ActionArgDef, ActionDefinition, ActionKind, ArgSchema};

pub const MAXIMUM_INTERACTIVE_ROWS: u32 = 65_536;

pub const WINDOW_KIND_ID: &str = ImageWindowKit::KIND_ID;
pub const BODY_KEY: &str = ImageWindowKit::KIND_ID;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn definition() -> WindowKindDefinition {
    let mut definition = ImageWindowKit::editable_window_kind();
    definition.actions.push(paint_region_action());
    definition
}

/// ⚠️ Saving re-encodes the lossy scan; the notice states this limitation and what is preserved.
fn reencode_notice(locale: Locale) -> semio_plugin::UiAssemblyResult<BuiltNode> {
    let text = match locale {
        Locale::En => "Saving re-encodes the JPEG scan (lossy). Metadata segments such as EXIF and ICC profiles are preserved.",
        Locale::De => "Beim Speichern wird der JPEG-Scan neu kodiert (verlustbehaftet). Metadaten-Segmente wie EXIF und ICC-Profile bleiben erhalten.",
    };
    ui::text(Label::try_from(text).map_err(|_| semio_plugin::PluginAssemblyError::new("stdio.jpg.reencode-notice", "re-encode notice exceeds UI bound"))?)
        .try_id("jpg-reencode-notice")
        .map_err(|_| semio_plugin::PluginAssemblyError::new("stdio.jpg.reencode-notice", "re-encode notice id admission"))?
        .try_build()
        .map_err(|_| semio_plugin::PluginAssemblyError::new("stdio.jpg.reencode-notice", "re-encode notice build admission"))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn render(snapshot: &JpgSnapshot, locale: Locale) -> semio_plugin::UiAssemblyResult<BuiltNode> {
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

fn integer_arg(id: &'static str, en: &'static str, de: &'static str, minimum: f64, maximum: f64, default: u32) -> ActionArgDef {
    let mut argument = ActionArgDef::number(id, LocalizedLabel::native(en, de)).required().default_value(&default);
    if let ArgSchema::Number { min, max, step, integer, .. } = &mut argument.schema {
        *min = Some(minimum);
        *max = Some(maximum);
        *step = Some(1.0);
        *integer = true;
    }
    argument
}

/// 🎨 Publishes the JPEG raster region action shared by the window and editor.
pub fn paint_region_action() -> ActionDefinition {
    ActionDefinition::bounded_catalog("paint-region", LocalizedLabel::native("Paint Region", "Bereich malen"), ActionKind::Mutation).with_args(vec![
        integer_arg("x", "Left (x)", "Links (x)", 0.0, u32::MAX as f64, 0),
        integer_arg("y", "Top (y)", "Oben (y)", 0.0, u32::MAX as f64, 0),
        integer_arg("width", "Width", "Breite", 1.0, u32::MAX as f64, 1),
        integer_arg("height", "Height", "Höhe", 1.0, f64::from(MAXIMUM_INTERACTIVE_ROWS), 1),
        integer_arg("red", "Red", "Rot", 0.0, 255.0, 0),
        integer_arg("green", "Green", "Grün", 0.0, 255.0, 0),
        integer_arg("blue", "Blue", "Blau", 0.0, 255.0, 0),
        integer_arg("alpha", "Alpha", "Alpha", 0.0, 255.0, 255),
    ])
}

