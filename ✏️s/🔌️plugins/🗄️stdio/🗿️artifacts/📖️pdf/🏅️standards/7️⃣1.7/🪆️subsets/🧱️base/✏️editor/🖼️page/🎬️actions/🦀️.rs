//! 🎬️ PDF page editor actions: action payloads turned into mutations, the page edit emission and the pointer interaction.

use super::*;

//#region 🎬️Actions
/// 🧾 Reads a page-edit action into a replayable payload.
pub fn edit_from_action(action: &str, args: Option<&semio_framework_value::DslValue>) -> Result<PdfPageEdit, Fault> {
    if !is_page_action(action) {
        return Err(fault(format!("unknown pdf page action '{action}'")));
    }
    let page_index = || req_index(args, "page");
    let object_id = || req_text(args, "object");
    let text_value = || req_text(args, "text");
    if action == "move-page" {
        return Ok(payload_edit(action, page_index()?, String::new(), String::new(), String::new(), req_num(args, "x")?, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0));
    }
    let (page, object, text, extra, x, y, width, height, red, green, blue) = match action {
        "set-text" => (page_index()?, object_id()?, text_value()?, String::new(), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        "move" => (page_index()?, object_id()?, String::new(), String::new(), req_num(args, "x")?, req_num(args, "y")?, 0.0, 0.0, 0.0, 0.0, 0.0),
        "delete" => (page_index()?, object_id()?, String::new(), String::new(), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        "remove-page" => (page_index()?, String::new(), String::new(), String::new(), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        "resize" => (page_index()?, object_id()?, String::new(), String::new(), req_num(args, "x")?, req_num(args, "y")?, req_num(args, "width")?, req_num(args, "height")?, 0.0, 0.0, 0.0),
        "insert-rectangle" | "insert-line" => (page_index()?, String::new(), String::new(), String::new(), req_num(args, "x")?, req_num(args, "y")?, req_num(args, "width")?, req_num(args, "height")?, opt_num(args, "red", 0.0), opt_num(args, "green", 0.0), opt_num(args, "blue", 0.0)),
        "set-fill" => (page_index()?, object_id()?, String::new(), String::new(), 0.0, 0.0, 0.0, 0.0, req_num(args, "red")?, req_num(args, "green")?, req_num(args, "blue")?),
        "set-stroke" => (page_index()?, object_id()?, String::new(), String::new(), 0.0, 0.0, req_num(args, "width")?, 0.0, req_num(args, "red")?, req_num(args, "green")?, req_num(args, "blue")?),
        "insert-text" => (page_index()?, String::new(), text_value()?, String::new(), req_num(args, "x")?, req_num(args, "y")?, 0.0, req_num(args, "height")?, opt_num(args, "red", 0.0), opt_num(args, "green", 0.0), opt_num(args, "blue", 0.0)),
        "insert-image" | "insert-page" | "set-page-size" => (page_index()?, String::new(), String::new(), String::new(), opt_num(args, "x", 0.0), opt_num(args, "y", 0.0), req_num(args, "width")?, req_num(args, "height")?, 0.0, 0.0, 0.0),
        "set-info" => (0, text_arg(args, "object"), text_arg(args, "text"), text_arg(args, "extra"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        "set-annotation" => (page_index()?, object_id()?, text_value()?, String::new(), req_num(args, "x")?, req_num(args, "y")?, req_num(args, "width")?, req_num(args, "height")?, 0.0, 0.0, 0.0),
        "set-image" => (page_index()?, object_id()?, text_value()?, String::new(), 0.0, 0.0, req_num(args, "width")?, req_num(args, "height")?, 0.0, 0.0, 0.0),
        "set-font" => (page_index()?, object_id()?, text_value()?, String::new(), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        "set-outline" => (page_index()?, String::new(), text_value()?, String::new(), opt_num(args, "x", -1.0), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        "set-page-rotation" | "set-page-user-unit" => (page_index()?, String::new(), String::new(), String::new(), req_num(args, "x")?, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        "set-page-box" => (page_index()?, text_value()?, String::new(), String::new(), req_num(args, "x")?, req_num(args, "y")?, req_num(args, "width")?, req_num(args, "height")?, 0.0, 0.0, 0.0),
        "set-language" | "set-page-layout" | "set-page-mode" | "set-metadata" => (0, String::new(), text_arg(args, "text"), String::new(), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        "set-optional-content" => (0, object_id()?, text_value()?, String::new(), opt_num(args, "x", 1.0), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        "set-embedded-file" => (0, object_id()?, text_arg(args, "text"), text_arg(args, "extra"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        "remove-embedded-file" | "remove-named-destination" => (0, object_id()?, String::new(), String::new(), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        "set-named-destination" => (page_index()?, object_id()?, String::new(), String::new(), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        "set-page-label" => (page_index()?, String::new(), text_arg(args, "text"), text_arg(args, "extra"), opt_num(args, "x", 1.0), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        "set-mark-info" => (0, String::new(), String::new(), String::new(), opt_num(args, "x", 0.0), opt_num(args, "y", 0.0), opt_num(args, "width", 0.0), 0.0, 0.0, 0.0, 0.0),
        "set-viewer-preferences" => (0, object_id()?, text_arg(args, "text"), String::new(), req_num(args, "x")?, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        "set-info-field" => (0, text_arg(args, "object"), text_arg(args, "text"), String::new(), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        "set-page-extra" => (page_index()?, text_arg(args, "object"), text_arg(args, "text"), text_arg(args, "extra"), opt_num(args, "x", 0.0), opt_num(args, "y", 0.0), 0.0, 0.0, 0.0, 0.0, 0.0),
        "set-annotation-style" => (page_index()?, object_id()?, text_value()?, text_arg(args, "extra"), opt_num(args, "x", 0.0), 0.0, 0.0, 0.0, opt_num(args, "red", 0.0), opt_num(args, "green", 0.0), opt_num(args, "blue", 0.0)),
        "set-annotation-border" => (page_index()?, object_id()?, text_arg(args, "text"), text_arg(args, "extra"), opt_num(args, "x", 0.0), opt_num(args, "y", -1.0), opt_num(args, "width", 0.0), 0.0, 0.0, 0.0, 0.0),
        "set-annotation-markup" => (page_index()?, object_id()?, text_value()?, text_arg(args, "extra"), opt_num(args, "x", 0.0), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        "set-form-settings" => (0, text_arg(args, "object"), text_arg(args, "text"), text_arg(args, "extra"), opt_num(args, "x", 0.0), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        "set-extra-entry" => (0, text_arg(args, "object"), text_arg(args, "text"), text_arg(args, "extra"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        "set-annotation-kind" => (page_index()?, object_id()?, text_value()?, text_arg(args, "extra"), opt_num(args, "x", 0.0), opt_num(args, "y", 0.0), opt_num(args, "width", 0.0), opt_num(args, "height", 0.0), opt_num(args, "red", 0.0), opt_num(args, "green", 0.0), opt_num(args, "blue", 0.0)),
        "set-field-data" => (0, object_id()?, text_value()?, text_arg(args, "extra"), opt_num(args, "x", 0.0), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        "set-resource-detail" => (0, object_id()?, text_value()?, text_arg(args, "extra"), opt_num(args, "x", 0.0), opt_num(args, "y", 0.0), opt_num(args, "width", 0.0), opt_num(args, "height", 0.0), opt_num(args, "red", 0.0), opt_num(args, "green", 0.0), opt_num(args, "blue", 0.0)),
        "set-encryption" => (0, text_arg(args, "object"), text_arg(args, "text"), text_arg(args, "extra"), opt_num(args, "x", -1.0), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        "set-output-intent" | "set-form-field" | "set-document-id" => (0, text_arg(args, "object"), text_arg(args, "text"), text_arg(args, "extra"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        "set-open-action" => (opt_index(args, "page"), text_arg(args, "object"), text_arg(args, "text"), String::new(), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        "set-font-reference" => (0, object_id()?, text_value()?, text_arg(args, "extra"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        "set-graphics-state" => (0, object_id()?, text_arg(args, "text"), String::new(), req_num(args, "x")?, req_num(args, "y")?, 0.0, 0.0, 0.0, 0.0, 0.0),
        "set-pattern" => (0, object_id()?, text_arg(args, "text"), String::new(), opt_num(args, "x", 0.0), opt_num(args, "y", 0.0), opt_num(args, "width", 8.0), opt_num(args, "height", 8.0), 0.0, 0.0, 0.0),
        "set-color-space" => (0, object_id()?, text_value()?, text_arg(args, "extra"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        "set-properties" => (0, object_id()?, text_value()?, text_arg(args, "extra"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        "set-font-metrics" => (0, object_id()?, text_arg(args, "text"), text_arg(args, "extra"), opt_num(args, "x", -1.0), opt_num(args, "y", 0.0), 0.0, 0.0, 0.0, 0.0, 0.0),
        "set-image-mask" | "set-catalog-entry" | "set-trailer-entry" => (0, object_id()?, text_arg(args, "text"), text_arg(args, "extra"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        "set-form-content" => (0, object_id()?, text_arg(args, "text"), String::new(), opt_num(args, "x", 0.0), opt_num(args, "y", 0.0), opt_num(args, "width", 8.0), opt_num(args, "height", 8.0), 0.0, 0.0, 0.0),
        "set-page-transition" => (page_index()?, String::new(), text_arg(args, "text"), text_arg(args, "extra"), opt_num(args, "x", 0.0), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        "set-annotation-appearance" => (page_index()?, object_id()?, text_arg(args, "text"), String::new(), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        "set-glyph" => (0, object_id()?, text_value()?, String::new(), opt_num(args, "x", 0.0), opt_num(args, "y", 0.0), opt_num(args, "width", 10.0), opt_num(args, "height", 10.0), 0.0, 0.0, 0.0),
        "set-indirect-object" => (0, String::new(), text_arg(args, "text"), String::new(), req_num(args, "x")?, opt_num(args, "y", 0.0), 0.0, 0.0, 0.0, 0.0, 0.0),
        "set-mesh-reference" => (0, object_id()?, text_arg(args, "text"), text_arg(args, "extra"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        "canvasPointerDown" | "canvasPointerMove" | "canvasPointerUp" => (0, String::new(), String::new(), if pointer_extend(args) { "shift".into() } else { String::new() }, opt_num(args, "x", 0.0), opt_num(args, "y", 0.0), 0.0, 0.0, 0.0, 0.0, 0.0),
        _ => return Err(fault(format!("unknown pdf page action '{action}'"))),
    };
    Ok(payload_edit(action, page, object, text, extra, x, y, width, height, red, green, blue))
}

/// 🧬 Applies a replayable page edit to the typed document.
pub fn apply_payload(snapshot: &PdfSnapshot, action: &str, payload: &str) -> Result<Vec<PdfMutation>, Fault> {
    let value: Value = serde_json::from_str(payload).map_err(|error| fault(error.to_string()))?;
    let page = value.get("page").and_then(Value::as_u64).unwrap_or(0) as usize;
    let object = value.get("object").and_then(Value::as_str).unwrap_or("").to_string();
    let text = value.get("text").and_then(Value::as_str).unwrap_or("").to_string();
    let extra = value.get("extra").and_then(Value::as_str).unwrap_or("").to_string();
    let x = json_f64(&value, "x");
    let y = json_f64(&value, "y");
    let width = json_f64(&value, "width");
    let height = json_f64(&value, "height");
    let red = json_f64(&value, "red");
    let green = json_f64(&value, "green");
    let blue = json_f64(&value, "blue");
    match action {
        "set-info" => {
            let mut info = snapshot.info.clone();
            if !text.is_empty() {
                info.title = Some(text);
            }
            if !extra.is_empty() {
                info.author = Some(extra);
            }
            if !object.is_empty() {
                info.subject = Some(object);
            }
            Ok(vec![PdfMutation::SetInfo(SetInfo { info })])
        }
        "insert-page" => Ok(vec![PdfMutation::InsertPage(InsertPage { index: page, page: PdfPage::new(width.max(1.0), height.max(1.0)) })]),
        "remove-page" => Ok(vec![PdfMutation::RemovePage(RemovePage { index: page })]),
        "move-page" => Ok(vec![PdfMutation::MovePage(MovePage { from: page, to: x.max(0.0) as usize })]),
        "set-page-size" => Ok(vec![PdfMutation::SetPageMediaBox(SetPageMediaBox { index: page, media_box: [0.0, 0.0, width.max(1.0), height.max(1.0)] })]),
        "insert-text" | "insert-rectangle" | "insert-line" | "insert-image" => insert_object(snapshot, action, page, x, y, width, height, red, green, blue, &text),
        "set-image" => replace_image_samples(snapshot, &object, width, height, &text),
        "set-font" => set_font_of_text(snapshot, &object, &text),
        "set-outline" => set_outline(snapshot, page, x, &text),
        "set-page-rotation" => set_page_rotation(snapshot, page, x),
        "set-page-box" => set_page_box(snapshot, page, &object, x, y, width, height),
        "set-page-user-unit" => set_page_user_unit(snapshot, page, x),
        "set-language" => Ok(vec![PdfMutation::SetLanguage(SetLanguage { language: none_if_empty(&text) })]),
        "set-page-layout" => Ok(vec![PdfMutation::SetPageLayout(SetPageLayout { layout: page_layout(&text)? })]),
        "set-page-mode" => Ok(vec![PdfMutation::SetPageMode(SetPageMode { mode: page_mode(&text)? })]),
        "set-optional-content" => set_optional_content(snapshot, &object, &text, x),
        "set-embedded-file" => set_embedded_file(snapshot, &object, &text, &extra),
        "remove-embedded-file" => remove_named(&object, "an embedded file needs an id").map(|id| vec![PdfMutation::RemoveEmbeddedFile(RemoveEmbeddedFile { id })]),
        "set-named-destination" => set_named_destination(&object, page),
        "remove-named-destination" => remove_named(&object, "a named destination needs a name").map(|name| vec![PdfMutation::RemoveNamedDestination(RemoveNamedDestination { name })]),
        "set-page-label" => set_page_label(snapshot, page, &text, &extra, x),
        "set-mark-info" => Ok(set_mark_info(x, y, width)),
        "set-metadata" => Ok(vec![PdfMutation::SetMetadata(SetMetadata { xmp: none_if_empty(&text) })]),
        "set-viewer-preferences" => set_viewer_preference(snapshot, &object, &text, x),
        "set-info-field" => set_info_field(snapshot, &object, &text),
        "set-page-extra" => set_page_extra(snapshot, page, &object, &text, &extra, x, y),
        "set-annotation-style" => set_annotation_style(snapshot, page, &object, &text, &extra, red, green, blue, x),
        "set-annotation-border" => set_annotation_border(snapshot, page, &object, &text, &extra, x, y, width),
        "set-annotation-markup" => set_annotation_markup(snapshot, page, &object, &text, &extra, x),
        "set-form-settings" => set_form_settings(snapshot, &object, &text, &extra, x),
        "set-extra-entry" => set_extra_entry(snapshot, &object, &extra, &text),
        "set-annotation-kind" => set_annotation_kind(snapshot, page, &object, &text, &extra, x, y, width, height, red, green, blue),
        "set-field-data" => set_field_data(snapshot, &object, &text, &extra, x),
        "set-resource-detail" => set_resource_detail(snapshot, &object, &text, &extra, x, y, width, height, red, green, blue),
        "set-encryption" => set_encryption(&text, &extra, &object, x),
        "set-output-intent" => set_output_intent(snapshot, &object, &text, &extra),
        "set-form-field" => set_form_field(snapshot, &object, &text, &extra),
        "set-open-action" => set_open_action(&object, &text, page),
        "set-document-id" => Ok(set_document_id(&text, &extra)),
        "set-font-reference" => set_font_program(snapshot, &object, &text, &extra),
        "set-graphics-state" => set_graphics_state(snapshot, &object, &text, x, y),
        "set-pattern" => set_pattern(snapshot, &object, &text, x, y, width, height),
        "set-color-space" => set_color_space(snapshot, &object, &text, &extra),
        "set-properties" => set_properties(snapshot, &object, &text, &extra),
        "set-font-metrics" => set_font_metrics(snapshot, &object, &text, &extra, x, y),
        "set-image-mask" => set_image_mask(snapshot, &object, &text, &extra),
        "set-form-content" => set_form_content(snapshot, &object, &text, x, y, width, height),
        "set-page-transition" => set_page_transition(snapshot, page, &text, &extra, x),
        "set-catalog-entry" => set_named_object(&object, &text, "a catalog entry needs a key", |key| PdfMutation::RemoveCatalogEntry(RemoveCatalogEntry { key }), |key, value| PdfMutation::SetCatalogEntry(SetCatalogEntry { key, value, index: None })),
        "set-trailer-entry" => set_named_object(&object, &text, "a trailer entry needs a key", |key| PdfMutation::RemoveTrailerEntry(RemoveTrailerEntry { key }), |key, value| PdfMutation::SetTrailerEntry(SetTrailerEntry { key, value, index: None })),
        "set-annotation-appearance" => set_annotation_appearance(snapshot, page, &object, &text),
        "set-glyph" => set_glyph(snapshot, &object, &text, x, y, width, height),
        "set-indirect-object" => set_indirect_object(snapshot, x, y, &text),
        "set-mesh-reference" => set_mesh_reference(snapshot, &object, &text, &extra),
        "set-annotation" => edit_annotation(snapshot, page, &object, &text, x, y, width, height),
        "canvasPointerDown" | "canvasPointerMove" | "canvasPointerUp" => Ok(Vec::new()),
        _ => edit_object(snapshot, action, &object, x, y, width, height, red, green, blue, &text),
    }
}

/// 🖱️ A page edit, or a canvas hit that redispatches selection and leaves the PDF unchanged.
pub fn emit_page_edit(snapshot: &PdfSnapshot, action: &str, payload: &str) -> Result<semio_framework_plugin::Emit<PdfMutation>, Fault> {
    match action {
        "canvasPointerMove" | "canvasPointerUp" => Ok(semio_framework_plugin::Emit::default()),
        "canvasPointerDown" => {
            let effect = pointer_down_effect(snapshot, payload)?;
            Ok(semio_framework_plugin::Emit::effect(effect))
        }
        _ => {
            let mutations = apply_payload(snapshot, action, payload)?;
            Ok(semio_framework_plugin::Emit { artifact_mutations: mutations, ..Default::default() })
        }
    }
}

/// 🎯 Declares the page-object selection domain and points the page window at it.
pub fn install_object_interaction(definition: &mut semio_framework_plugin::AppDefinition) {
    definition.interactions.push(semio_framework_plugin::InteractionDefinition {
        id: OBJECT_DOMAIN.into(),
        label: LocalizedLabel::native("Objects", "Objekte"),
        granularities: vec![semio_framework_plugin::GranularityDefinition { id: "object".into(), label: LocalizedLabel::native("Object", "Objekt"), icon_id: "square".into() }],
        hierarchy: semio_framework_plugin::HierarchyProvider::Flat,
        hover: semio_framework_plugin::HoverSpec::default(),
        selection: semio_framework_plugin::SelectionSpec {
            modes: vec![semio_framework_plugin::SelectionMode::Multiple, semio_framework_plugin::SelectionMode::Single],
            methods: vec![semio_framework_plugin::SelectionMethod::Pick],
            merges: vec![semio_framework_plugin::MergeMode::Replace, semio_framework_plugin::MergeMode::Invertive],
            transitive: false,
            broadcast: true,
        },
    });
    for window in definition.window_kinds.iter_mut() {
        if window.id == WINDOW_KIND_ID {
            window.interactions = vec![semio_framework_plugin::InteractionRef::new(OBJECT_DOMAIN)];
        }
    }
}

fn pointer_down_effect(snapshot: &PdfSnapshot, payload: &str) -> Result<semio_framework_plugin::Effect, Fault> {
    let value: Value = serde_json::from_str(payload).map_err(|error| fault(error.to_string()))?;
    let extend = value.get("extra").and_then(Value::as_str) == Some("shift") || value.get("shift").and_then(Value::as_bool).unwrap_or(false) || json_f64(&value, "shift") >= 0.5;
    let merge = if extend { "invertive" } else { "replace" };
    Ok(match hit_test(snapshot, json_f64(&value, "x"), json_f64(&value, "y")) {
        Some(id) => select_effect(&id, merge),
        None => semio_framework_plugin::Effect::DispatchAction { req: semio_framework_plugin::RequestId(113), action: semio_framework_plugin::CLEAR_SELECTION_ACTION_ID.into(), args: None, delay_ms: 0 },
    })
}

fn select_effect(id: &str, merge: &str) -> semio_framework_plugin::Effect {
    semio_framework_plugin::Effect::DispatchAction {
        req: semio_framework_plugin::RequestId(115),
        action: semio_framework_plugin::INTERACTION_SELECT_ACTION_ID.into(),
        args: Some(semio_framework_value::DslValue::from(json!({ "domainId": OBJECT_DOMAIN, "targets": json!([{ "granularity": "object", "id": id }]).to_string(), "merge": merge, "method": "pick" }))),
        delay_ms: 0,
    }
}

fn pointer_extend(args: Option<&semio_framework_value::DslValue>) -> bool {
    let Some(semio_framework_value::DslValue::Object(entries)) = args else { return false };
    entries.iter().any(|(key, value)| {
        (key == "shift" || key == "ctrl" || key == "meta") && match value {
            semio_framework_value::DslValue::Bool(flag) => *flag,
            semio_framework_value::DslValue::Number(number) => number.as_f64() >= 0.5,
            semio_framework_value::DslValue::String(text) => text == "true" || text == "1",
            _ => false,
        }
    })
}
//#endregion 🎬️Actions
