//! 🎛️ Revision-pinned replacement control for a selected PDF image.

use semio_plugin::{
    ActionId, Buildable, BuiltNode, HasBase, PluginAssemblyError, UiAssemblyResult, UiMapBuilder, UiText, UiValue,
    plugin_app_close_prelude as ui,
};

/// 📂 Builds a localized picker action for one selected image placement.
pub fn image_replacement_button(controller: &str, page: usize, at: usize, revision: u64, locale: semio_ui_locale::Locale) -> UiAssemblyResult<BuiltNode> {
    let fail = || PluginAssemblyError::new("pdf.object.image-action", "PDF image picker control admission failed");
    let guard = UiText::try_from_str(&format!("{revision:016x}")).ok_or_else(fail)?;
    let mut args = UiMapBuilder::try_new().ok_or_else(fail)?;
    for (key, value) in [("page", UiValue::Number(crate::document::page_number(page) as f64)), ("operator", UiValue::Number(at as f64)), ("text", UiValue::Text(guard))] {
        args.try_insert(key.into(), value).map_err(|_| fail())?;
    }
    let action = ActionId::try_v1(controller, "replace-image").ok_or_else(fail)?;
    let label = match locale {
        semio_ui_locale::Locale::De => "Bild durch PNG ersetzen",
        semio_ui_locale::Locale::En => "Replace Image with PNG",
    };
    ui::button(ui::Label(UiText::clipped(label))).try_id("pdf-replace-image").map_err(|_| fail())?
        .try_on_with(ui::Trigger::Activate, action, UiValue::Map(args.finish())).map_err(|_| fail())?
        .try_build().map_err(|_| fail())
}
