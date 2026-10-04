//! ✏️ `tiff` edit (any) — Main window: real `ImageWindowKit`
//! render of the current document (read-only native canvas; typed edits live in Details).

use crate::standards::v6_0::subsets::document::io::encode_tiff_page_png;
use crate::standards::v6_0::subsets::document::schema::snapshot::TiffSnapshot;
use crate::editor::tiff_any::component::config::{selected_ifd, TiffEditorConfig};
use semio_framework_plugin::app::{ImageView, ImageWindowKit};
use semio_framework_plugin::{ActionArgDef, ActionDefinition, ActionId, ActionKind, ArgSchema, Buildable, BuiltNode, HasBase, HasChildren, PluginAssemblyError, Trigger, UiMapBuilder, UiValue, WindowKindDefinition, WindowKit};
use semio_framework_ui_contract::{self as ui, Label as UiLabel};
use semio_framework_ui_locale::Locale;
use semio_framework_ui_locale::LocalizedLabel;

pub const WINDOW_KIND_ID: &str = ImageWindowKit::KIND_ID;
pub const BODY_KEY: &str = ImageWindowKit::KIND_ID;
pub const SELECT_IFD_ACTION_ID: &str = "select-ifd";
const CONTROLLER_ID: &str = "s.stdio.tiff@6.0/*#editor";

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn definition() -> WindowKindDefinition {
    let mut definition = ImageWindowKit::window_kind();
    definition.actions.push(select_ifd_action());
    definition
}

pub fn select_ifd_action() -> ActionDefinition {
    let mut ifd = ActionArgDef::number("ifdIndex", LocalizedLabel::native("Image page", "Bildseite")).required().default_value(&0);
    if let ArgSchema::Number { min, max, step, integer, .. } = &mut ifd.schema {
        *min = Some(0.0);
        *max = Some(u32::MAX as f64);
        *step = Some(1.0);
        *integer = true;
    }
    ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog(SELECT_IFD_ACTION_ID, LocalizedLabel::native("Select Image Page", "Bildseite auswählen"), ActionKind::View).with_args(vec![ifd]) }
}

fn error(code: &'static str, message: impl Into<String>) -> PluginAssemblyError {
    PluginAssemblyError::new(code, message)
}

fn select_arguments(index: usize) -> semio_framework_plugin::UiAssemblyResult<UiValue> {
    let mut arguments = UiMapBuilder::try_new().ok_or_else(|| error("stdio.tiff.page.arguments", "page argument map capacity"))?;
    arguments.try_insert("ifdIndex".into(), UiValue::Number(index as f64)).map_err(|_| error("stdio.tiff.page.arguments", "page argument capacity"))?;
    Ok(UiValue::Map(arguments.finish()))
}

fn page_button(id: &str, label: &str, index: usize) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let action = ActionId::try_v1(CONTROLLER_ID, SELECT_IFD_ACTION_ID).ok_or_else(|| error("stdio.tiff.page.action", "invalid page action id"))?;
    ui::button(UiLabel::try_from(label).map_err(|_| error("stdio.tiff.page.label", "page label exceeds UI bound"))?)
        .try_id(id)
        .map_err(|_| error("stdio.tiff.page.id", "page control id exceeds UI bound"))?
        .try_on_with(Trigger::Activate, action, select_arguments(index)?)
        .map_err(|_| error("stdio.tiff.page.binding", "page control binding admission"))?
        .try_build()
        .map_err(|_| error("stdio.tiff.page.build", "page control build admission"))
}

fn page_selector(snapshot: &TiffSnapshot, active: Option<usize>, locale: Locale) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let status = match (active, locale) {
        (Some(index), Locale::En) => format!("Image page {} of {}", index + 1, snapshot.ifds.len()),
        (Some(index), Locale::De) => format!("Bildseite {} von {}", index + 1, snapshot.ifds.len()),
        (None, Locale::En) => "No image pages".into(),
        (None, Locale::De) => "Keine Bildseiten".into(),
    };
    let status = ui::text(UiLabel::try_from(status.as_str()).map_err(|_| error("stdio.tiff.page.status", "page status exceeds UI bound"))?)
        .try_id("tiff-image-page-status")
        .map_err(|_| error("stdio.tiff.page.status", "page status id admission"))?
        .try_build()
        .map_err(|_| error("stdio.tiff.page.status", "page status build admission"))?;
    let mut controls = vec![status];
    if let Some(index) = active {
        if index > 0 {
            controls.push(page_button("tiff-image-page-previous", match locale { Locale::En => "Previous image page", Locale::De => "Vorherige Bildseite" }, index - 1)?);
        }
        if index + 1 < snapshot.ifds.len() {
            controls.push(page_button("tiff-image-page-next", match locale { Locale::En => "Next image page", Locale::De => "Nächste Bildseite" }, index + 1)?);
        }
    }
    ui::row()
        .try_id("tiff-image-page-selector")
        .map_err(|_| error("stdio.tiff.page.selector", "page selector id admission"))?
        .try_children(controls)
        .map_err(|_| error("stdio.tiff.page.selector", "page selector child admission"))?
        .try_build()
        .map_err(|_| error("stdio.tiff.page.selector", "page selector build admission"))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn render(snapshot: &TiffSnapshot, config: &TiffEditorConfig, locale: Locale) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let selected = selected_ifd(config, snapshot);
    let selector = page_selector(snapshot, selected, locale)?;
    match selected.and_then(|index| image_view(snapshot, index).ok()) {
        Some(view) => ImageWindowKit::render_with_accessory(&view, selector),
        None => ImageWindowKit::render_unavailable_with_accessory(locale, selector),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn image_view(snapshot: &TiffSnapshot, ifd_index: usize) -> semio_framework_plugin::UiAssemblyResult<ImageView> {
    let page = encode_tiff_page_png(snapshot, ifd_index).map_err(|message| semio_framework_plugin::PluginAssemblyError::new("stdio.tiff.preview", message))?;
    Ok(ImageView { width: page.width, height: page.height, mime: "image/png".into(), base64: semio_s_artifact_stdio_contract::base64_standard(&page.bytes) })
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
