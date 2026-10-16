//! 🔎️ PDF page editor inspector: the field descriptions, inputs and draft targets of the object window.

use super::*;

//#region 🔎️Inspector
struct InspectorField {
    id: &'static str,
    label_en: &'static str,
    label_de: &'static str,
    action: &'static str,
    key: &'static str,
    kind: ui::InputKind,
    value: String,
    numbers: Vec<(&'static str, f64)>,
    text: Option<String>,
    object_key: Option<String>,
}

pub(super) fn inspector_fields(snapshot: &PdfSnapshot, object: &PageObject) -> Vec<InspectorField> {
    let mut fields = Vec::new();
    let geometry = |id, label_en, label_de, key, value, x, y, width, height| InspectorField { id, label_en, label_de, action: "resize", key, kind: ui::InputKind::Number, value: shown_number(value), numbers: vec![("x", x), ("y", y), ("width", width), ("height", height)], text: None, object_key: None };
    match object.kind {
        ObjectKind::Text => {
            let font = snapshot.font(&object.font).map(|font| font.base_font().to_string()).unwrap_or_else(|| object.font.clone());
            fields.push(InspectorField { id: "pdf-object-text", label_en: "Text", label_de: "Text", action: "set-text", key: "text", kind: ui::InputKind::Text, value: object.text.clone(), numbers: Vec::new(), text: None, object_key: None });
            fields.push(InspectorField { id: "pdf-object-font", label_en: "Font", label_de: "Schrift", action: "set-font", key: "text", kind: ui::InputKind::Text, value: font, numbers: Vec::new(), text: None, object_key: None });
            fields.push(placed("pdf-object-x", "X", "X", "move", "x", object.x, &[("y", object.y)]));
            fields.push(placed("pdf-object-y", "Y", "Y", "move", "y", object.y, &[("x", object.x)]));
            fields.push(geometry("pdf-object-size", "Font size", "Schriftgröße", "height", object.font_size, object.x, object.y, object.width, object.font_size));
            fields.extend(color_fields("set-fill", object.fill, &[]));
        }
        ObjectKind::Vector => {
            fields.push(geometry("pdf-object-x", "X", "X", "x", object.x, object.x, object.y, object.width, object.height));
            fields.push(geometry("pdf-object-y", "Y", "Y", "y", object.y, object.x, object.y, object.width, object.height));
            fields.push(geometry("pdf-object-width", "Width", "Breite", "width", object.width, object.x, object.y, object.width, object.height));
            fields.push(geometry("pdf-object-height", "Height", "Höhe", "height", object.height, object.x, object.y, object.width, object.height));
            fields.extend(color_fields("set-fill", object.fill, &[]));
            fields.extend(color_fields("set-stroke", object.stroke, &[("width", object.line_width)]));
            fields.push(placed("pdf-object-line", "Line width", "Linienstärke", "set-stroke", "width", object.line_width, &[("red", object.stroke[0]), ("green", object.stroke[1]), ("blue", object.stroke[2])]));
        }
        ObjectKind::Image | ObjectKind::Form => {
            fields.push(geometry("pdf-object-x", "X", "X", "x", object.x, object.x, object.y, object.width, object.height));
            fields.push(geometry("pdf-object-y", "Y", "Y", "y", object.y, object.x, object.y, object.width, object.height));
            fields.push(geometry("pdf-object-width", "Width", "Breite", "width", object.width, object.x, object.y, object.width, object.height));
            fields.push(geometry("pdf-object-height", "Height", "Höhe", "height", object.height, object.x, object.y, object.width, object.height));
            if object.kind == ObjectKind::Image {
                if let Some(image) = snapshot.image(&object.text) {
                    fields.push(resource_number("pdf-object-interpolate", "Interpolate", "Interpolieren", "x", if image.interpolate { 1.0 } else { 0.0 }, "image.interpolate", &object.text));
                    fields.push(resource_text("pdf-object-intent", "Intent", "Absicht", image.intent.clone().unwrap_or_default(), "image.intent", &object.text));
                    fields.push(resource_text("pdf-object-decode", "Decode", "Dekodierung", image.decode.iter().map(|value| shown_number(*value)).collect::<Vec<_>>().join(","), "image.decode", &object.text));
                }
            }
        }
        ObjectKind::Shading => {
            fields.push(placed("pdf-object-x", "X", "X", "move", "x", object.x, &[("y", object.y)]));
            fields.push(placed("pdf-object-y", "Y", "Y", "move", "y", object.y, &[("x", object.x)]));
            fields.push(geometry("pdf-object-width", "Width", "Breite", "width", object.width, object.x, object.y, object.width, object.height));
            fields.push(geometry("pdf-object-height", "Height", "Höhe", "height", object.height, object.x, object.y, object.width, object.height));
        }
        ObjectKind::Annotation => {
            fields.push(InspectorField { id: "pdf-object-text", label_en: "Text", label_de: "Text", action: "set-annotation", key: "text", kind: ui::InputKind::Text, value: object.text.clone(), numbers: vec![("x", object.x), ("y", object.y), ("width", object.width), ("height", object.height)], text: None, object_key: None });
            fields.push(annotation_box("pdf-object-x", "X", "X", "x", object));
            fields.push(annotation_box("pdf-object-y", "Y", "Y", "y", object));
            fields.push(annotation_box("pdf-object-width", "Width", "Breite", "width", object));
            fields.push(annotation_box("pdf-object-height", "Height", "Höhe", "height", object));
            if let Ok((_, annotation)) = annotation_at(snapshot, object.page, &object.id) {
                let color = [annotation.color.first().copied().unwrap_or(0.0), annotation.color.get(1).copied().unwrap_or(0.0), annotation.color.get(2).copied().unwrap_or(0.0)];
                fields.extend(color_fields("set-annotation-style", color, &[]));
                let border = annotation.border.as_ref().map(|border| border.width).unwrap_or(0.0);
                fields.push(placed("pdf-object-border", "Border", "Rahmen", "set-annotation-border", "x", border, &[]));
            }
        }
    }
    fields
}

pub(super) fn page_fields(snapshot: &PdfSnapshot) -> Vec<InspectorField> {
    let (width, height, rotation) = snapshot.pages.first().map(|page| (page.width(), page.height(), page.rotate as f64)).unwrap_or((0.0, 0.0, 0.0));
    vec![
        InspectorField { id: "pdf-page-title", label_en: "Title", label_de: "Titel", action: "set-info", key: "text", kind: ui::InputKind::Text, value: snapshot.info.title.clone().unwrap_or_default(), numbers: Vec::new(), text: None, object_key: Some(String::new()) },
        InspectorField { id: "pdf-page-author", label_en: "Author", label_de: "Autor", action: "set-info", key: "extra", kind: ui::InputKind::Text, value: snapshot.info.author.clone().unwrap_or_default(), numbers: Vec::new(), text: None, object_key: Some(String::new()) },
        InspectorField { id: "pdf-page-language", label_en: "Language", label_de: "Sprache", action: "set-language", key: "text", kind: ui::InputKind::Text, value: snapshot.language.clone().unwrap_or_default(), numbers: Vec::new(), text: None, object_key: Some(String::new()) },
        placed("pdf-page-width", "Page width", "Seitenbreite", "set-page-size", "width", width, &[("height", height)]),
        placed("pdf-page-height", "Page height", "Seitenhöhe", "set-page-size", "height", height, &[("width", width)]),
        placed("pdf-page-rotation", "Rotation", "Drehung", "set-page-rotation", "x", rotation, &[]),
    ]
}

fn resource_number(id: &'static str, label_en: &'static str, label_de: &'static str, key: &'static str, value: f64, aspect: &str, resource_id: &str) -> InspectorField {
    InspectorField { id, label_en, label_de, action: "set-resource-detail", key, kind: ui::InputKind::Number, value: shown_number(value), numbers: Vec::new(), text: Some(aspect.to_string()), object_key: Some(resource_id.to_string()) }
}

fn resource_text(id: &'static str, label_en: &'static str, label_de: &'static str, value: String, aspect: &str, resource_id: &str) -> InspectorField {
    InspectorField { id, label_en, label_de, action: "set-resource-detail", key: "extra", kind: ui::InputKind::Text, value, numbers: Vec::new(), text: Some(aspect.to_string()), object_key: Some(resource_id.to_string()) }
}

pub(super) fn placed(id: &'static str, label_en: &'static str, label_de: &'static str, action: &'static str, key: &'static str, value: f64, numbers: &[(&'static str, f64)]) -> InspectorField {
    InspectorField { id, label_en, label_de, action, key, kind: ui::InputKind::Number, value: shown_number(value), numbers: numbers.to_vec(), text: None, object_key: None }
}

fn color_fields(action: &'static str, color: [f64; 3], extra: &[(&'static str, f64)]) -> Vec<InspectorField> {
    let channels = [("red", "Red", "Rot", color[0]), ("green", "Green", "Grün", color[1]), ("blue", "Blue", "Blau", color[2])];
    channels
        .into_iter()
        .map(|(key, en, de, value)| {
            let mut numbers = vec![("red", color[0]), ("green", color[1]), ("blue", color[2])];
            numbers.extend_from_slice(extra);
            let id = match action {
                "set-fill" => match key { "red" => "pdf-object-fill-red", "green" => "pdf-object-fill-green", _ => "pdf-object-fill-blue" },
                "set-annotation-style" => match key { "red" => "pdf-object-color-red", "green" => "pdf-object-color-green", _ => "pdf-object-color-blue" },
                _ => match key { "red" => "pdf-object-stroke-red", "green" => "pdf-object-stroke-green", _ => "pdf-object-stroke-blue" },
            };
            let text = (action == "set-annotation-style").then(|| "color".to_string());
            InspectorField { id, label_en: en, label_de: de, action, key, kind: ui::InputKind::Number, value: shown_number(value), numbers, text, object_key: None }
        })
        .collect()
}

fn annotation_box(id: &'static str, label_en: &'static str, label_de: &'static str, key: &'static str, object: &PageObject) -> InspectorField {
    let value = match key {
        "x" => object.x,
        "y" => object.y,
        "width" => object.width,
        _ => object.height,
    };
    InspectorField { id, label_en, label_de, action: "set-annotation", key, kind: ui::InputKind::Number, value: shown_number(value), numbers: vec![("x", object.x), ("y", object.y), ("width", object.width), ("height", object.height)], text: Some(object.text.clone()), object_key: None }
}

pub(super) fn field_input(
    page: usize,
    object_id: &str,
    field: &InspectorField,
    controller_id: &str,
    publication_revision: semio_framework_plugin::UiPublicationRevision,
    locale: Locale,
) -> UiAssemblyResult<semio_framework_ui_contract::BuiltNode> {
    let label = match locale {
        Locale::De => field.label_de,
        Locale::En => field.label_en,
    };
    let mut numbers = field.numbers.clone();
    numbers.retain(|(key, _)| *key != field.key);
    let object_id = field.object_key.as_deref().unwrap_or(object_id);
    let mut arguments = object_args(page, object_id, field.key, &numbers)?;
    if let Some(text) = &field.text {
        arguments = insert_text_arg(arguments, text)?;
    }
    let draft_target = inspector_draft_target(controller_id, field.action, page, object_id, field.key)?;
    let builder = ui::input(field.kind)
        .value(inspector_text(&field.value)?)
        .commit(inspector_text("blur")?)
        .draft_target(draft_target)
        .publication_revision(publication_revision)
        .try_label(label)
        .map_err(|_| inspector_error("pdf.object.input-label"))?
        .try_id(field.id)
        .map_err(|_| inspector_error("pdf.object.input-id"))?;
    builder.try_on_with(ui::Trigger::Commit, inspector_action(controller_id, field.action)?, arguments).map_err(|_| inspector_error("pdf.object.binding"))?.try_build().map_err(|_| inspector_error("pdf.object.input"))
}

fn inspector_draft_target(controller_id: &str, action_id: &str, page: usize, object_id: &str, field: &str) -> UiAssemblyResult<UiText> {
    inspector_text(&format!(
        "pdf-inspector.v1/{}/{}/{}/{}/{}/{}/{}/{}/{}",
        controller_id.len(),
        controller_id,
        action_id.len(),
        action_id,
        object_id.len(),
        object_id,
        field.len(),
        field,
        page
    ))
    .map_err(|_| inspector_error("pdf.object.draft-target"))
}

pub(super) fn object_args(page: usize, object_id: &str, field: &str, numbers: &[(&str, f64)]) -> UiAssemblyResult<UiValue> {
    let mut entries = vec![("page", UiValue::Number(page as f64)), ("object", UiValue::Text(inspector_text(object_id)?)), ("field", UiValue::Text(inspector_text(field)?))];
    for (key, value) in numbers {
        entries.push((key, UiValue::Number(*value)));
    }
    let mut map = UiMapBuilder::try_new().ok_or_else(|| inspector_error("pdf.object.args"))?;
    for (key, value) in entries {
        map.try_insert(key.to_string(), value).map_err(|_| inspector_error("pdf.object.arg"))?;
    }
    Ok(UiValue::Map(map.finish()))
}

fn insert_text_arg(arguments: UiValue, text: &str) -> UiAssemblyResult<UiValue> {
    let UiValue::Map(map) = arguments else { return Err(inspector_error("pdf.object.args")) };
    let mut rebuilt = UiMapBuilder::try_new().ok_or_else(|| inspector_error("pdf.object.args"))?;
    for (key, value) in map.iter() {
        rebuilt.try_insert(key.as_str().to_string(), value.credited_clone().ok_or_else(|| inspector_error("pdf.object.arg"))?).map_err(|_| inspector_error("pdf.object.arg"))?;
    }
    rebuilt.try_insert("text".to_string(), UiValue::Text(inspector_text(text)?)).map_err(|_| inspector_error("pdf.object.arg"))?;
    Ok(UiValue::Map(rebuilt.finish()))
}

pub(super) fn inspector_action(controller_id: &str, name: &str) -> UiAssemblyResult<ActionId> {
    ActionId::try_v1(controller_id, name).ok_or_else(|| inspector_error("pdf.object.action"))
}

fn inspector_text(value: &str) -> UiAssemblyResult<UiText> {
    UiText::try_from_str(value).ok_or_else(|| inspector_error("pdf.object.text"))
}

pub(super) fn inspector_error(code: &'static str) -> PluginAssemblyError {
    PluginAssemblyError::new(code, "pdf object inspector admission failed")
}

fn shown_number(value: f64) -> String {
    if !value.is_finite() {
        return "0".into();
    }
    let text = format!("{value:.4}");
    let trimmed = text.trim_end_matches('0').trim_end_matches('.').to_string();
    if trimmed.is_empty() || trimmed == "-" { "0".into() } else { trimmed }
}
//#endregion 🔎️Inspector
