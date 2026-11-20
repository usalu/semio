//! ♿️ PDF page editor text layer: the accessible, localized reading-order view of the page canvas.
//! The logical model carries no typed structure tree, so reading order is the content-stream order of the page objects;
//! the document language, the tagged-PDF flag and the title come from the catalog and info dictionary.
//! [PDF/UA](https://www.iso.org/standard/64599.html)

use super::*;

//#region ♿️TextLayer
pub const TEXT_LAYER_WINDOW_KIND_ID: &str = "pdf.text-layer";
pub const TEXT_LAYER_BODY_KEY: &str = "pdf.text-layer";

/// 🪟 Document facts and keyboard-selectable objects in page order.
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
    text_layer_rows(snapshot, locale).into_iter().map(|(id, label, _)| (id, label)).collect()
}

fn text_layer_rows(snapshot: &PdfSnapshot, locale: Locale) -> Vec<(String, String, Option<String>)> {
    let mut lines = Vec::new();
    let title = snapshot.info.title.clone().filter(|title| !title.is_empty()).unwrap_or_else(|| pick(locale, "untitled", "ohne Titel"));
    lines.push(("title".to_string(), format!("{}: {title}", pick(locale, "Title", "Titel")), None));
    let language = snapshot.language.clone().filter(|language| !language.is_empty()).unwrap_or_else(|| pick(locale, "not declared", "nicht angegeben"));
    lines.push(("language".to_string(), format!("{}: {language}", pick(locale, "Language", "Sprache")), None));
    let tagged = snapshot.mark_info.as_ref().is_some_and(|info| info.marked);
    lines.push(("tagged".to_string(), format!("{}: {}", pick(locale, "Tagged PDF", "Getaggtes PDF"), if tagged { pick(locale, "yes", "ja") } else { pick(locale, "no", "nein") }), None));
    let mut all = objects(snapshot).into_iter().peekable();
    for page in 0..snapshot.pages.len() {
        lines.push((format!("page-{page}"), format!("{} {} / {}", pick(locale, "Page", "Seite"), page + 1, snapshot.pages.len()), Some(format!("p{page}:paper"))));
        while all.peek().is_some_and(|object| object.page == page) {
            let Some(object) = all.next() else { break };
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
            lines.push((object.id.replace(':', "-"), format!("{kind}{content}"), Some(object.id)));
        }
    }
    lines
}

/// ♿️ The accessible text layer window body.
pub fn render_text_layer(snapshot: &PdfSnapshot, controller_id: &str, locale: Locale) -> UiAssemblyResult<semio_ui_contract::BuiltNode> {
    let error = |code: &'static str| PluginAssemblyError::new(code, "pdf text layer admission failed");
    let mut column = ui::column().try_id("pdf-text-layer").map_err(|_| error("pdf.text-layer.root-id"))?;
    for (id, line, target) in text_layer_rows(snapshot, locale) {
        let label = ui::Label(UiText::clipped(&line));
        let row_id = format!("pdf-text-layer-{id}");
        let node = if let Some(target) = target {
            let targets = json!([{ "granularity": "object", "id": target }]).to_string();
            let mut arguments = UiMapBuilder::try_new().ok_or_else(|| error("pdf.text-layer.selection"))?;
            for (key, value) in [("domainId", OBJECT_DOMAIN), ("targets", targets.as_str()), ("merge", "replace"), ("method", "pick")] {
                arguments.try_insert(key.to_owned(), UiValue::Text(UiText::clipped(value))).map_err(|_| error("pdf.text-layer.selection"))?;
            }
            ui::button(label).try_id(row_id).map_err(|_| error("pdf.text-layer.line-id"))?
                .try_on_with(ui::Trigger::Activate, inspector_action(controller_id, semio_plugin::INTERACTION_SELECT_ACTION_ID)?, UiValue::Map(arguments.finish()))
                .map_err(|_| error("pdf.text-layer.selection"))?.try_build()
        } else {
            ui::text(label).try_id(row_id).map_err(|_| error("pdf.text-layer.line-id"))?.try_build()
        }.map_err(|_| error("pdf.text-layer.line"))?;
        column = column.try_child(node).map_err(|_| error("pdf.text-layer.child"))?;
    }
    column.try_build().map_err(|_| error("pdf.text-layer.root"))
}
//#endregion ♿️TextLayer
