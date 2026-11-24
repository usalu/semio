//! 🔎️ PDF page editor inspector: the field descriptions, inputs and draft targets of the object window.

use super::*;

//#region 🔎️Inspector
pub(crate) struct InspectorField {
    pub id: &'static str,
    pub label_en: &'static str,
    pub label_de: &'static str,
    pub action: &'static str,
    pub key: &'static str,
    pub kind: ui::InputKind,
    pub value: String,
    pub numbers: Vec<(&'static str, f64)>,
    pub text: Option<String>,
    pub object_key: Option<String>,
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
            fields.push(placed("pdf-object-size", "Font size", "Schriftgröße", "set-font-size", "size", object.font_size, &[]));
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
                if let PdfAnnotationKind::Widget { field: Some(name), .. } = &annotation.kind {
                    fields.push(InspectorField { id: "pdf-object-field-value", label_en: "Field value", label_de: "Feldwert", action: "fill-form-field", key: "text", kind: ui::InputKind::Text, value: form_field_value(snapshot, name), numbers: Vec::new(), text: None, object_key: Some(name.clone()) });
                }
            }
        }
    }
    fields
}

fn form_field_value(snapshot: &PdfSnapshot, path: &str) -> String {
    fn find<'a>(fields: &'a [PdfFormField], path: &str, prefix: &str) -> Option<&'a PdfFormField> {
        fields.iter().find_map(|field| {
            let full = if prefix.is_empty() { field.name.clone() } else { format!("{prefix}.{}", field.name) };
            if full == path { Some(field) } else { find(&field.children, path, &full) }
        })
    }
    let Some(field) = snapshot.acro_form.as_ref().and_then(|form| find(&form.fields, path, "")) else { return String::new() };
    match &field.kind {
        PdfFormFieldKind::Text { value, .. } | PdfFormFieldKind::Button { value, .. } => value.clone().unwrap_or_default(),
        PdfFormFieldKind::Choice { values, .. } => values.join(","),
        PdfFormFieldKind::Signature { .. } | PdfFormFieldKind::Container => String::new(),
    }
}

/// ✍️ The page-level authoring controls: create annotations, link, merge, replace and extract; see [`create_annotation`].
pub(super) fn authoring_controls(page: usize, size: (f64, f64), controller_id: &str, publication_revision: semio_plugin::UiPublicationRevision, locale: Locale) -> UiAssemblyResult<Vec<semio_ui_contract::BuiltNode>> {
    let label = |en: &'static str, de: &'static str| pick_label(locale, en, de);
    let top = (size.1 - 72.0).max(0.0);
    let place = |width: f64, height: f64| vec![("x", 36.0), ("y", top), ("width", width), ("height", height)];
    let strokes = format!("36,{top},156,{}", top + 20.0);
    let mut buttons: Vec<(&str, &str, UiValue, &str)> = Vec::new();
    for (id, kind, en, de, numbers, extra) in [
        ("pdf-add-note", "note", "Add note", "Notiz hinzufügen", place(20.0, 20.0), ""),
        ("pdf-add-highlight", "highlight", "Add highlight", "Hervorhebung hinzufügen", place(120.0, 14.0), ""),
        ("pdf-add-underline", "underline", "Add underline", "Unterstreichung hinzufügen", place(120.0, 14.0), ""),
        ("pdf-add-strikeout", "strikeOut", "Add strike-out", "Durchstreichung hinzufügen", place(120.0, 14.0), ""),
        ("pdf-add-ink", "ink", "Add freehand stroke", "Freihandlinie hinzufügen", Vec::new(), strokes.as_str()),
    ] {
        let mut arguments = object_args(page, kind, "", &numbers)?;
        if !extra.is_empty() {
            arguments = insert_named_text_arg(arguments, "extra", extra)?;
        }
        buttons.push((id, label(en, de), arguments, "create-annotation"));
    }
    for (id, action, en, de, extra, number) in [
        ("pdf-merge-pick", "merge-pick", "Insert pages from PDF", "Seiten aus PDF einfügen", String::new(), page + 1),
        ("pdf-replace-pick", "replace-page-pick", "Replace page from PDF", "Seite aus PDF ersetzen", String::new(), page),
        ("pdf-replace-blank", "replace-page", "Replace page with blank page", "Seite durch leere Seite ersetzen", String::new(), page),
        ("pdf-extract-page", "extract-pages", "Extract page to PDF", "Seite als PDF extrahieren", (page + 1).to_string(), page),
    ] {
        let mut arguments = object_args(number, "", "", &[])?;
        if !extra.is_empty() {
            arguments = insert_named_text_arg(arguments, "extra", &extra)?;
        }
        buttons.push((id, label(en, de), arguments, action));
    }
    let mut nodes = Vec::new();
    for (id, text, arguments, action) in buttons {
        nodes.push(ui::button(ui::Label(UiText::clipped(text))).try_id(id).map_err(|_| inspector_error("pdf.object.authoring-id"))?
            .try_on_with(ui::Trigger::Activate, inspector_action(controller_id, action)?, arguments).map_err(|_| inspector_error("pdf.object.authoring-binding"))?
            .try_build().map_err(|_| inspector_error("pdf.object.authoring"))?);
    }
    let link = InspectorField { id: "pdf-add-link", label_en: "Add link (address or page:N)", label_de: "Link hinzufügen (Adresse oder page:N)", action: "create-annotation", key: "text", kind: ui::InputKind::Text, value: String::new(), numbers: place(120.0, 14.0), text: None, object_key: Some("link".into()) };
    nodes.push(field_input(page, "", &link, controller_id, publication_revision, locale)?);
    Ok(nodes)
}

fn lower_first(name: String) -> String {
    let mut characters = name.chars();
    characters.next().map(|first| first.to_lowercase().chain(characters).collect()).unwrap_or_default()
}

fn text_field(id: &'static str, label_en: &'static str, label_de: &'static str, action: &'static str, key: &'static str, value: String, numbers: Vec<(&'static str, f64)>, object_key: Option<&str>) -> InspectorField {
    InspectorField { id, label_en, label_de, action, key, kind: ui::InputKind::Text, value, numbers, text: None, object_key: object_key.map(str::to_string) }
}

/// 📚 Document-level inputs: layout, mode, metadata, tagging, bookmark, destination, document id, open link and page label prefix.
fn document_fields(snapshot: &PdfSnapshot) -> Vec<InspectorField> {
    let flag = |value: bool| if value { 1.0 } else { 0.0 };
    let mark = snapshot.mark_info.as_ref().map_or((0.0, 0.0, 0.0), |mark| (flag(mark.marked), flag(mark.user_properties), flag(mark.suspects)));
    let identity = |index: usize| snapshot.document_id.as_ref().map(|id| String::from_utf8_lossy(&id[index]).into_owned()).unwrap_or_default();
    vec![
        text_field("pdf-doc-layout", "Page layout (singlePage, oneColumn, twoColumnLeft, twoColumnRight, twoPageLeft, twoPageRight)", "Seitenlayout (singlePage, oneColumn, twoColumnLeft, twoColumnRight, twoPageLeft, twoPageRight)", "set-page-layout", "text", snapshot.page_layout.as_ref().map(|layout| lower_first(format!("{layout:?}"))).unwrap_or_default(), Vec::new(), Some("")),
        text_field("pdf-doc-mode", "Page mode (useNone, useOutlines, useThumbs, fullScreen, useOc, useAttachments)", "Seitenmodus (useNone, useOutlines, useThumbs, fullScreen, useOc, useAttachments)", "set-page-mode", "text", snapshot.page_mode.as_ref().map(|mode| lower_first(format!("{mode:?}"))).unwrap_or_default(), Vec::new(), Some("")),
        text_field("pdf-doc-metadata", "XMP metadata", "XMP-Metadaten", "set-metadata", "text", snapshot.metadata.clone().unwrap_or_default(), Vec::new(), Some("")),
        placed("pdf-doc-marked", "Tagged (1 yes, 0 no)", "Mit Tags (1 ja, 0 nein)", "set-mark-info", "x", mark.0, &[("y", mark.1), ("width", mark.2)]),
        placed("pdf-doc-user-properties", "User properties (1 yes, 0 no)", "Benutzereigenschaften (1 ja, 0 nein)", "set-mark-info", "y", mark.1, &[("x", mark.0), ("width", mark.2)]),
        placed("pdf-doc-suspects", "Suspects (1 yes, 0 no)", "Verdachtsfälle (1 ja, 0 nein)", "set-mark-info", "width", mark.2, &[("x", mark.0), ("y", mark.1)]),
        text_field("pdf-doc-id-permanent", "Permanent document id", "Dauerhafte Dokumentkennung", "set-document-id", "text", identity(0), Vec::new(), Some("")),
        text_field("pdf-doc-id-changing", "Changing document id", "Wechselnde Dokumentkennung", "set-document-id", "extra", identity(1), Vec::new(), Some("")),
        text_field("pdf-doc-open-link", "Open action link address", "Adresse der Öffnen-Aktion", "set-open-action", "text", String::new(), Vec::new(), Some("uri")),
        text_field("pdf-page-bookmark", "Add bookmark for this page", "Lesezeichen für diese Seite hinzufügen", "set-outline", "text", String::new(), vec![("x", -1.0)], Some("")),
        text_field("pdf-page-destination", "Named destination for this page", "Benanntes Ziel für diese Seite", "set-named-destination", "object", String::new(), Vec::new(), Some("")),
        text_field("pdf-page-label-prefix", "Page label prefix", "Präfix der Seitenbeschriftung", "set-page-label", "extra", String::new(), vec![("x", 1.0)], Some("")),
    ]
}

fn entry_input(family: &str, id: String, label: &str, key: &str, value: &str, controller_id: &str, publication_revision: semio_plugin::UiPublicationRevision) -> UiAssemblyResult<semio_ui_contract::BuiltNode> {
    let mut arguments = object_args(0, family, "text", &[])?;
    arguments = insert_named_text_arg(arguments, "extra", key)?;
    let draft_target = inspector_draft_target(controller_id, "set-entry", 0, &format!("{family}/{key}"), "text")?;
    ui::input(ui::InputKind::Text)
        .value(inspector_text(value)?)
        .commit(inspector_text("blur")?)
        .draft_target(draft_target)
        .publication_revision(publication_revision)
        .try_label(label)
        .map_err(|_| inspector_error("pdf.entry.label"))?
        .try_id(id)
        .map_err(|_| inspector_error("pdf.entry.id"))?
        .try_on_with(ui::Trigger::Commit, inspector_action(controller_id, "set-entry")?, arguments)
        .map_err(|_| inspector_error("pdf.entry.binding"))?
        .try_build()
        .map_err(|_| inspector_error("pdf.entry.input"))
}

/// 🗝️ The generic key-value form: a headed group with one labelled input per editable entry, read-only lines for other entries and an add input taking `key=value`.
pub(super) fn entry_form(snapshot: &PdfSnapshot, family: &'static str, heading: &str, controller_id: &str, publication_revision: semio_plugin::UiPublicationRevision, locale: Locale) -> UiAssemblyResult<semio_ui_contract::BuiltNode> {
    let mut column = ui::column().try_id(format!("pdf-entries-{family}")).map_err(|_| inspector_error("pdf.entry.group-id"))?.try_label(heading).map_err(|_| inspector_error("pdf.entry.group-label"))?;
    let title = ui::text(ui::Label(UiText::clipped(heading))).emphasize(true).try_id(format!("pdf-entries-{family}-heading")).map_err(|_| inspector_error("pdf.entry.heading-id"))?.try_build().map_err(|_| inspector_error("pdf.entry.heading"))?;
    column = column.try_child(title).map_err(|_| inspector_error("pdf.entry.heading"))?;
    for (index, (key, value, editable)) in entry_rows(snapshot, family).into_iter().enumerate() {
        let node = if editable {
            entry_input(family, format!("pdf-entry-{family}-{index}"), &key, &key, &value, controller_id, publication_revision)?
        } else {
            ui::text(ui::Label(UiText::clipped(&format!("{key}: {value}")))).try_id(format!("pdf-entry-{family}-{index}")).map_err(|_| inspector_error("pdf.entry.line-id"))?.try_build().map_err(|_| inspector_error("pdf.entry.line"))?
        };
        column = column.try_child(node).map_err(|_| inspector_error("pdf.entry.row"))?;
    }
    let add = pick(locale, "Add entry (key=value)", "Eintrag hinzufügen (Schlüssel=Wert)");
    column = column.try_child(entry_input(family, format!("pdf-entry-{family}-add"), &add, "", "", controller_id, publication_revision)?).map_err(|_| inspector_error("pdf.entry.add"))?;
    column.try_build().map_err(|_| inspector_error("pdf.entry.group"))
}

/// 🗝️ Every document dictionary family as a key-value form.
pub(super) fn entry_forms(snapshot: &PdfSnapshot, controller_id: &str, publication_revision: semio_plugin::UiPublicationRevision, locale: Locale) -> UiAssemblyResult<Vec<semio_ui_contract::BuiltNode>> {
    let heading = |en: &'static str, de: &'static str| pick_label(locale, en, de);
    [("catalog", heading("Catalog entries", "Katalogeinträge")), ("trailer", heading("Trailer entries", "Trailer-Einträge")), ("info", heading("Extra info entries", "Zusätzliche Infoeinträge")), ("viewer", heading("Extra viewer preferences", "Zusätzliche Betrachtereinstellungen"))]
        .into_iter()
        .map(|(family, title)| entry_form(snapshot, family, title, controller_id, publication_revision, locale))
        .collect()
}

pub(super) fn page_fields(snapshot: &PdfSnapshot, index: usize) -> Vec<InspectorField> {
    let (width, height, rotation, user_unit) = snapshot.pages.get(index).map(|page| (page.width(), page.height(), page.rotate as f64, page.user_unit.unwrap_or(1.0))).unwrap_or((0.0, 0.0, 0.0, 1.0));
    use semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::io::text::snapshot::date::print_pdf_date;
    let text = |value: &Option<String>| value.clone().unwrap_or_default();
    let info = &snapshot.info;
    [
        ("pdf-page-title", "Title", "Titel", "title", text(&info.title)),
        ("pdf-page-author", "Author", "Autor", "author", text(&info.author)),
        ("pdf-page-subject", "Subject", "Thema", "subject", text(&info.subject)),
        ("pdf-page-keywords", "Keywords", "Schlagwörter", "keywords", text(&info.keywords)),
        ("pdf-page-creator", "Creator", "Erstellt mit", "creator", text(&info.creator)),
        ("pdf-page-producer", "Producer", "PDF-Generator", "producer", text(&info.producer)),
        ("pdf-page-created", "Creation date", "Erstellungsdatum", "creationDate", info.creation_date.as_ref().map(print_pdf_date).unwrap_or_default()),
        ("pdf-page-modified", "Modification date", "Änderungsdatum", "modificationDate", info.modification_date.as_ref().map(print_pdf_date).unwrap_or_default()),
        ("pdf-page-trapped", "Trapped", "Überfüllt", "trapped", text(&info.trapped)),
    ].into_iter().map(|(id, label_en, label_de, key, value)| InspectorField { id, label_en, label_de, action: "set-info-field", key: "text", kind: ui::InputKind::Text, value, numbers: Vec::new(), text: None, object_key: Some(key.into()) }).chain([
        InspectorField { id: "pdf-page-language", label_en: "Language", label_de: "Sprache", action: "set-language", key: "text", kind: ui::InputKind::Text, value: snapshot.language.clone().unwrap_or_default(), numbers: Vec::new(), text: None, object_key: Some(String::new()) },
        placed("pdf-page-width", "Page width", "Seitenbreite", "set-page-size", "width", width, &[("height", height)]),
        placed("pdf-page-height", "Page height", "Seitenhöhe", "set-page-size", "height", height, &[("width", width)]),
        placed("pdf-page-rotation", "Rotation", "Drehung", "set-page-rotation", "x", rotation, &[]),
        placed("pdf-page-user-unit", "User unit scale", "Maßstab der Benutzereinheit", "set-page-user-unit", "x", user_unit, &[]),
    ]).chain(document_fields(snapshot)).collect()
}

pub(super) fn page_boundary_fields(page: &PdfPage) -> impl Iterator<Item = InspectorField> {
    [
        ("pdf-page-media-box", "Media box", "Medienrahmen", "box.media", Some(page.media_box)),
        ("pdf-page-crop-box", "Crop box", "Beschnittrahmen", "box.crop", page.crop_box),
        ("pdf-page-bleed-box", "Bleed box", "Anschnittrahmen", "box.bleed", page.bleed_box),
        ("pdf-page-trim-box", "Trim box", "Endformatrahmen", "box.trim", page.trim_box),
        ("pdf-page-art-box", "Art box", "Objektrahmen", "box.art", page.art_box),
    ].into_iter().map(|(id, label_en, label_de, kind, rect)| InspectorField {
        id, label_en, label_de, action: "set-page-extra", key: "text", kind: ui::InputKind::Text,
        value: rect.map(|coordinates| coordinates.map(|value| value.to_string()).join(" ")).unwrap_or_default(),
        numbers: Vec::new(), text: None, object_key: Some(kind.into()),
    })
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
    publication_revision: semio_plugin::UiPublicationRevision,
    locale: Locale,
) -> UiAssemblyResult<semio_ui_contract::BuiltNode> {
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
    arguments = insert_named_text_arg(arguments, "locale", locale.as_str())?;
    let draft_target = inspector_draft_target(controller_id, field.action, page, object_id, field.key)?;
    let quad = field.action == "set-page-extra" && object_id.starts_with("box.");
    let (kind, shown) = match field.kind {
        ui::InputKind::Number => (ui::InputKind::Text, localized_number(&field.value, locale)),
        other if quad => (other, localized_quad(&field.value, locale)),
        other => (other, field.value.clone()),
    };
    let builder = ui::input(kind)
        .value(inspector_text(&shown)?)
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
    let mut entries = vec![("page", UiValue::Number(page_number(page) as f64)), ("object", UiValue::Text(inspector_text(object_id)?)), ("field", UiValue::Text(inspector_text(field)?))];
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
    insert_named_text_arg(arguments, "text", text)
}

pub(super) fn insert_named_text_arg(arguments: UiValue, key: &str, text: &str) -> UiAssemblyResult<UiValue> {
    let UiValue::Map(map) = arguments else { return Err(inspector_error("pdf.object.args")) };
    let mut rebuilt = UiMapBuilder::try_new().ok_or_else(|| inspector_error("pdf.object.args"))?;
    for (key, value) in map.iter() {
        rebuilt.try_insert(key.as_str().to_string(), value.credited_clone().ok_or_else(|| inspector_error("pdf.object.arg"))?).map_err(|_| inspector_error("pdf.object.arg"))?;
    }
    rebuilt.try_insert(key.to_string(), UiValue::Text(inspector_text(text)?)).map_err(|_| inspector_error("pdf.object.arg"))?;
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
    format_pdf_number(value, None)
}

fn localized_number(canonical: &str, locale: Locale) -> String {
    parse_pdf_number(canonical, None).map_or_else(|| canonical.to_string(), |value| format_pdf_number(value, Some(locale)))
}

fn localized_quad(canonical: &str, locale: Locale) -> String {
    canonical.split_whitespace().map(|token| token.replace('.', &pdf_decimal_separator(Some(locale)).to_string())).collect::<Vec<_>>().join(" ")
}
//#endregion 🔎️Inspector
