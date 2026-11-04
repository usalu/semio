//! ♿️ PDF page editor text layer: the accessible, localized reading-order view of the page canvas.
//! The logical model carries no typed structure tree, so reading order is the content-stream order of the page objects;
//! the document language, the tagged-PDF flag and the title come from the catalog and info dictionary.
//! [PDF/UA](https://www.iso.org/standard/64599.html)

use super::*;

//#region ♿️TextLayer
pub const TEXT_LAYER_WINDOW_KIND_ID: &str = "pdf.text-layer";
pub const TEXT_LAYER_BODY_KEY: &str = "pdf.text-layer";

/// 🪟 Read-only text alternative of the page canvas: document facts, then every page's objects in reading order.
pub fn text_layer_window_definition() -> WindowKindDefinition {
    WindowKindDefinition {
        initial_utility_id: None,
        id: TEXT_LAYER_WINDOW_KIND_ID.into(),
        label: LocalizedLabel::native("Text layer", "Textebene"),
        body_key: TEXT_LAYER_BODY_KEY.into(),
        surface_kind: SurfaceKind::BlockList,
        icon_id: "file-text".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        interactions: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}

fn pick(locale: Locale, en: &str, de: &str) -> String {
    match locale {
        Locale::De => de.to_string(),
        Locale::En => en.to_string(),
    }
}

/// 📝 One localized line per document fact and per page object, in reading order.
pub fn text_layer_lines(snapshot: &PdfSnapshot, locale: Locale) -> Vec<(String, String)> {
    let mut lines = Vec::new();
    let title = snapshot.info.title.clone().filter(|title| !title.is_empty()).unwrap_or_else(|| pick(locale, "untitled", "ohne Titel"));
    lines.push(("title".to_string(), format!("{}: {title}", pick(locale, "Title", "Titel"))));
    let language = snapshot.language.clone().filter(|language| !language.is_empty()).unwrap_or_else(|| pick(locale, "not declared", "nicht angegeben"));
    lines.push(("language".to_string(), format!("{}: {language}", pick(locale, "Language", "Sprache"))));
    let tagged = snapshot.mark_info.as_ref().is_some_and(|info| info.marked);
    lines.push(("tagged".to_string(), format!("{}: {}", pick(locale, "Tagged PDF", "Getaggtes PDF"), if tagged { pick(locale, "yes", "ja") } else { pick(locale, "no", "nein") })));
    let all = objects(snapshot);
    for page in 0..snapshot.pages.len() {
        lines.push((format!("page-{page}"), format!("{} {} / {}", pick(locale, "Page", "Seite"), page + 1, snapshot.pages.len())));
        for (index, object) in all.iter().filter(|object| object.page == page).enumerate() {
            let kind = match (object.kind, locale) {
                (ObjectKind::Text, Locale::En) => "Text",
                (ObjectKind::Text, Locale::De) => "Text",
                (ObjectKind::Vector, Locale::En) => "Vector graphic",
                (ObjectKind::Vector, Locale::De) => "Vektorgrafik",
                (ObjectKind::Image, Locale::En) => "Image",
                (ObjectKind::Image, Locale::De) => "Bild",
                (ObjectKind::Form, Locale::En) => "Form",
                (ObjectKind::Form, Locale::De) => "Formular",
                (ObjectKind::Shading, Locale::En) => "Shading",
                (ObjectKind::Shading, Locale::De) => "Verlauf",
                (ObjectKind::Annotation, Locale::En) => "Annotation",
                (ObjectKind::Annotation, Locale::De) => "Anmerkung",
            };
            let content = if object.text.is_empty() { String::new() } else { format!(": {}", object.text) };
            lines.push((format!("page-{page}-object-{index}"), format!("{kind}{content}")));
        }
    }
    lines
}

/// ♿️ The accessible text layer window body.
pub fn render_text_layer(snapshot: &PdfSnapshot, locale: Locale) -> UiAssemblyResult<semio_framework_ui_contract::BuiltNode> {
    let error = |code: &'static str| PluginAssemblyError::new(code, "pdf text layer admission failed");
    let mut column = ui::column().try_id("pdf-text-layer").map_err(|_| error("pdf.text-layer.root-id"))?;
    for (id, line) in text_layer_lines(snapshot, locale) {
        let node = ui::text(ui::Label(UiText::clipped(&line))).try_id(format!("pdf-text-layer-{id}")).map_err(|_| error("pdf.text-layer.line-id"))?.try_build().map_err(|_| error("pdf.text-layer.line"))?;
        column = column.try_child(node).map_err(|_| error("pdf.text-layer.child"))?;
    }
    column.try_build().map_err(|_| error("pdf.text-layer.root"))
}
//#endregion ♿️TextLayer
