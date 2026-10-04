//! ✏️ PNG window with an explicit bounded RGBA8 preview projection.

use crate::standards::v1_2::subsets::any::io::{png_layout, png_preview, PngNativeProfile};
use crate::standards::v1_2::subsets::any::schema::snapshot::PngSnapshot;
use semio_framework_plugin::app::{ImageView, ImageWindowKit};
use semio_framework_plugin::{Buildable, BuiltNode, HasBase, WindowKindDefinition, WindowKit};
use semio_framework_ui_contract::{self as ui, Label};
use semio_framework_ui_locale::Locale;

pub const WINDOW_KIND_ID: &str = ImageWindowKit::KIND_ID;
pub const BODY_KEY: &str = ImageWindowKit::KIND_ID;

pub fn definition() -> WindowKindDefinition {
    let mut definition = ImageWindowKit::editable_window_kind();
    definition.actions = vec![crate::editor::png::patch_pixel_region::action()];
    definition.actions.extend(crate::editor::png::paint_native_region::actions());
    definition
}

pub fn render(snapshot: &PngSnapshot, locale: Locale) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let accessory = native_profile_accessory(snapshot, locale)?;
    match image_view(snapshot) {
        Ok(view) => ImageWindowKit::render_with_accessory(&view, accessory),
        Err(_) => ImageWindowKit::render_unavailable_with_accessory(locale, accessory),
    }
}

fn native_profile_accessory(snapshot: &PngSnapshot, locale: Locale) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let layout = png_layout(snapshot).map_err(|message| semio_framework_plugin::PluginAssemblyError::new("stdio.png.native-profile", message))?;
    let profile = match layout.color_type {
        crate::schema::snapshot::PngColorType::Palette => PngNativeProfile::Indexed,
        crate::schema::snapshot::PngColorType::Grayscale => PngNativeProfile::Grayscale,
        crate::schema::snapshot::PngColorType::GrayscaleAlpha => PngNativeProfile::GrayscaleAlpha,
        crate::schema::snapshot::PngColorType::Rgb => PngNativeProfile::Rgb,
        crate::schema::snapshot::PngColorType::Rgba => PngNativeProfile::Rgba,
    };
    let profile_name = match (profile, locale) {
        (PngNativeProfile::Indexed, Locale::En) => "palette index",
        (PngNativeProfile::Indexed, Locale::De) => "Palettenindex",
        (PngNativeProfile::Grayscale, Locale::En) => "grayscale",
        (PngNativeProfile::Grayscale, Locale::De) => "Graustufe",
        (PngNativeProfile::GrayscaleAlpha, Locale::En) => "grayscale alpha",
        (PngNativeProfile::GrayscaleAlpha, Locale::De) => "Graustufe mit Alpha",
        (PngNativeProfile::Rgb, _) => "RGB",
        (PngNativeProfile::Rgba, _) => "RGBA",
    };
    let interlace = match (layout.interlace, locale) {
        (true, Locale::En) => ", Adam7 preserved",
        (true, Locale::De) => ", Adam7 bleibt erhalten",
        (false, _) => "",
    };
    let maximum = if layout.bit_depth == 16 { u16::MAX } else { ((1u32 << layout.bit_depth) - 1) as u16 };
    let range = if profile == PngNativeProfile::Indexed {
        let entries = layout.chunks.iter().find(|chunk| chunk.kind == *b"PLTE").map_or(0, |chunk| (chunk.data_end - chunk.data_start) / 3);
        match locale {
            Locale::En => format!(", valid indices 0–{}", entries.saturating_sub(1)),
            Locale::De => format!(", gültige Indizes 0–{}", entries.saturating_sub(1)),
        }
    } else {
        match locale {
            Locale::En => format!(", valid samples 0–{maximum}"),
            Locale::De => format!(", gültige Abtastwerte 0–{maximum}"),
        }
    };
    let text = match locale {
        Locale::En => format!("Native paint: {profile_name}, {}-bit samples{range}{interlace}", layout.bit_depth),
        Locale::De => format!("Natives Malen: {profile_name}, {}-Bit-Abtastwerte{range}{interlace}", layout.bit_depth),
    };
    ui::text(Label::try_from(text.as_str()).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("stdio.png.native-profile", "native profile label exceeds UI bound"))?)
        .try_id("png-native-profile-control")
        .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("stdio.png.native-profile", "native profile control id admission"))?
        .try_build()
        .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("stdio.png.native-profile", "native profile control build admission"))
}

fn image_view(snapshot: &PngSnapshot) -> semio_framework_plugin::UiAssemblyResult<ImageView> {
    let preview = png_preview(snapshot).map_err(|message| semio_framework_plugin::PluginAssemblyError::new("stdio.png.preview", message))?;
    Ok(ImageView { width: preview.width, height: preview.height, mime: "image/png".into(), base64: crate::base64_standard(&preview.bytes) })
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
