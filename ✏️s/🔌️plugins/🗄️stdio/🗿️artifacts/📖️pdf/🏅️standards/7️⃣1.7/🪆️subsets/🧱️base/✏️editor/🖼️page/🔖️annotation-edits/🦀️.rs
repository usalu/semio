//! 🔖️ PDF page editor annotation and form edits: kinds, borders, markup, field data, styles and transitions.

use super::*;

//#region 🔖️AnnotationEdits
pub(super) fn annotation_at(snapshot: &PdfSnapshot, page: usize, object_id: &str) -> Result<(usize, PdfAnnotation), Fault> {
    let index = object_id.rsplit_once(":a").and_then(|(_, raw)| raw.parse::<usize>().ok()).ok_or_else(|| fault(format!("pdf annotation '{object_id}' is not addressable")))?;
    let annotation = snapshot.pages.get(page).and_then(|item| item.annotations.get(index)).cloned().ok_or_else(|| fault(format!("pdf annotation '{object_id}' is gone")))?;
    Ok((index, annotation))
}

fn store_annotation(page: usize, index: usize, annotation: PdfAnnotation) -> Vec<PdfMutation> {
    vec![PdfMutation::SetAnnotation(SetAnnotation { index: page, at: index, annotation })]
}

pub(super) fn csv_numbers(text: &str) -> Result<Vec<f64>, Fault> {
    text.split(',').map(str::trim).filter(|item| !item.is_empty()).map(|item| item.parse::<f64>().map_err(|_| fault("a list needs numbers"))).collect()
}

fn upsert_name_entry(entries: &mut Vec<PdfDictEntry>, key: &str, value: &str) -> Result<(), Fault> {
    if key.is_empty() {
        return Err(fault("an entry needs a key"));
    }
    if value.is_empty() {
        entries.retain(|item| item.key != key);
    } else if let Some(existing) = entries.iter_mut().find(|item| item.key == key) {
        *existing = PdfDictEntry::new(key, PdfObject::Name(value.to_string()));
    } else {
        entries.push(PdfDictEntry::new(key, PdfObject::Name(value.to_string())));
    }
    Ok(())
}

pub(super) fn set_annotation_border(snapshot: &PdfSnapshot, page: usize, object_id: &str, style: &str, dash: &str, width: f64, radius_x: f64, radius_y: f64) -> Result<Vec<PdfMutation>, Fault> {
    let (index, mut annotation) = annotation_at(snapshot, page, object_id)?;
    annotation.border = if style == "clear" {
        None
    } else {
        Some(PdfBorderStyle { width, style: none_if_empty(style), dash: if dash.is_empty() { None } else { Some(csv_numbers(dash)?) }, radii: (radius_x >= 0.0).then_some([radius_x, radius_y]) })
    };
    Ok(store_annotation(page, index, annotation))
}

pub(super) fn set_annotation_markup(snapshot: &PdfSnapshot, page: usize, object_id: &str, aspect: &str, value: &str, number: f64) -> Result<Vec<PdfMutation>, Fault> {
    let (index, mut annotation) = annotation_at(snapshot, page, object_id)?;
    match aspect {
        "layer" => annotation.optional_content = none_if_empty(value),
        "structParent" => annotation.struct_parent = (number >= 0.0).then_some(number.round() as u32),
        "entry" => {
            let (key, entry) = value.split_once('=').filter(|(key, _)| !key.is_empty()).ok_or_else(|| fault("an entry needs key=value"))?;
            upsert_name_entry(&mut annotation.extra, key, entry)?;
        }
        "title" | "subject" | "richContents" | "replyType" | "intent" | "opacity" | "popup" | "inReplyTo" | "creationDate" => {
            let markup = annotation.markup.get_or_insert_with(PdfMarkupAnnotation::default);
            match aspect {
                "title" => markup.title = none_if_empty(value),
                "subject" => markup.subject = none_if_empty(value),
                "richContents" => markup.rich_contents = none_if_empty(value),
                "replyType" => markup.reply_type = none_if_empty(value),
                "intent" => markup.intent = none_if_empty(value),
                "opacity" => markup.opacity = (number >= 0.0).then_some(number.clamp(0.0, 1.0)),
                "popup" => markup.popup = (number >= 0.0).then_some(number.round() as u64),
                "inReplyTo" => markup.in_reply_to = (number >= 0.0).then_some(number.round() as u64),
                "creationDate" => markup.creation_date = parse_optional_date(value)?,
                _ => {}
            }
            if annotation.markup.as_ref().is_some_and(|item| item == &PdfMarkupAnnotation::default()) {
                annotation.markup = None;
            }
        }
        other => return Err(fault(format!("unknown annotation markup '{other}'"))),
    }
    Ok(store_annotation(page, index, annotation))
}

pub(super) fn set_form_settings(snapshot: &PdfSnapshot, aspect: &str, text: &str, extra: &str, number: f64) -> Result<Vec<PdfMutation>, Fault> {
    if aspect.is_empty() {
        return Err(fault("a form setting needs a name"));
    }
    let mut form = snapshot.acro_form.clone().unwrap_or_default();
    match aspect {
        "signatureFlags" => form.signature_flags = number.max(0.0).round() as u32,
        "defaultAppearance" => form.default_appearance = none_if_empty(text),
        "quadding" => form.quadding = (number >= 0.0).then_some(number.round() as u32),
        "defaultFonts" => form.default_fonts = if text.is_empty() { Vec::new() } else { text.split(',').map(str::trim).filter(|item| !item.is_empty()).map(str::to_string).collect() },
        "entry" => upsert_name_entry(&mut form.extra, extra, text)?,
        field_name => {
            let field = form.fields.iter_mut().find(|field| field.name == field_name).ok_or_else(|| fault(format!("pdf form field '{field_name}' is gone")))?;
            match text {
                "appearance" => field.default_appearance = none_if_empty(extra),
                "quadding" => field.quadding = (number >= 0.0).then_some(number.round() as u32),
                "flags" => field.flags = number.max(0.0).round() as u32,
                "alternate" => field.alternate_name = none_if_empty(extra),
                "mapping" => field.mapping_name = none_if_empty(extra),
                "action" => {
                    let (key, value) = extra.split_once('=').filter(|(key, _)| !key.is_empty()).ok_or_else(|| fault("an entry needs key=value"))?;
                    upsert_name_entry(&mut field.additional_actions, key, value)?;
                }
                "entry" => {
                    let (key, value) = extra.split_once('=').filter(|(key, _)| !key.is_empty()).ok_or_else(|| fault("an entry needs key=value"))?;
                    upsert_name_entry(&mut field.extra, key, value)?;
                }
                other => return Err(fault(format!("unknown form field setting '{other}'"))),
            }
        }
    }
    Ok(vec![PdfMutation::SetAcroForm(SetAcroForm { form: Some(form) })])
}

pub(super) fn set_extra_entry(snapshot: &PdfSnapshot, owner: &str, key: &str, value: &str) -> Result<Vec<PdfMutation>, Fault> {
    match owner {
        "info" => {
            let mut info = snapshot.info.clone();
            upsert_name_entry(&mut info.extra, key, value)?;
            Ok(vec![PdfMutation::SetInfo(SetInfo { info })])
        }
        "viewer" => {
            let mut preferences = snapshot.viewer_preferences.clone().unwrap_or_default();
            upsert_name_entry(&mut preferences.extra, key, value)?;
            Ok(vec![PdfMutation::SetViewerPreferences(SetViewerPreferences { preferences: (preferences != PdfViewerPreferences::default()).then_some(preferences) })])
        }
        other => Err(fault(format!("unknown extra dictionary '{other}'"))),
    }
}

fn two_names(value: &str) -> Result<Option<[String; 2]>, Fault> {
    if value.is_empty() {
        return Ok(None);
    }
    let names: Vec<String> = value.split(',').map(str::trim).filter(|item| !item.is_empty()).map(str::to_string).collect();
    names.try_into().map(Some).map_err(|_| fault("a line needs two endings"))
}

fn ink_paths(value: &str) -> Result<Vec<Vec<f64>>, Fault> {
    if value.is_empty() {
        return Ok(Vec::new());
    }
    value.split(';').map(csv_numbers).collect()
}

fn name_entry(value: &str) -> Result<(&str, &str), Fault> {
    value.split_once('=').filter(|(key, _)| !key.is_empty()).ok_or_else(|| fault("an entry needs key=value"))
}

fn annotation_kind(name: &str) -> Result<PdfAnnotationKind, Fault> {
    Ok(match name {
        "text" => PdfAnnotationKind::Text { open: false, icon: None, state: None, state_model: None },
        "link" => PdfAnnotationKind::Link { action: None, destination: None, highlight: None, quad_points: Vec::new() },
        "freeText" => PdfAnnotationKind::FreeText { default_appearance: String::new(), quadding: 0, callout: None, line_ending: None, rich_text: None },
        "line" => PdfAnnotationKind::Line { points: [0.0; 4], line_endings: None, interior_color: None, leader_length: None, caption: false },
        "square" => PdfAnnotationKind::Square { interior_color: None, rect_differences: None },
        "circle" => PdfAnnotationKind::Circle { interior_color: None, rect_differences: None },
        "polygon" => PdfAnnotationKind::Polygon { vertices: Vec::new(), interior_color: None },
        "polyLine" => PdfAnnotationKind::PolyLine { vertices: Vec::new(), line_endings: None, interior_color: None },
        "highlight" => PdfAnnotationKind::Highlight { quad_points: Vec::new() },
        "underline" => PdfAnnotationKind::Underline { quad_points: Vec::new() },
        "squiggly" => PdfAnnotationKind::Squiggly { quad_points: Vec::new() },
        "strikeOut" => PdfAnnotationKind::StrikeOut { quad_points: Vec::new() },
        "stamp" => PdfAnnotationKind::Stamp { icon: None },
        "caret" => PdfAnnotationKind::Caret { rect_differences: None, symbol: None },
        "ink" => PdfAnnotationKind::Ink { paths: Vec::new() },
        "popup" => PdfAnnotationKind::Popup { parent: None, open: false },
        "file" => PdfAnnotationKind::FileAttachment { file: PdfFileSpecification::Embedded { file: String::new() }, icon: None },
        "sound" => PdfAnnotationKind::Sound { sound: Vec::new(), icon: None },
        "movie" => PdfAnnotationKind::Movie { title: None, movie: Vec::new(), activation: None },
        "widget" => PdfAnnotationKind::Widget { field: None, highlight: None, characteristics: Vec::new(), action: None, additional_actions: Vec::new() },
        "screen" => PdfAnnotationKind::Screen { title: None, characteristics: Vec::new(), action: None, additional_actions: Vec::new() },
        "printerMark" => PdfAnnotationKind::PrinterMark { mark_style: None, colorants: Vec::new() },
        "trapNet" => PdfAnnotationKind::TrapNet { entries: Vec::new() },
        "watermark" => PdfAnnotationKind::Watermark { fixed_print: None },
        "threeD" => PdfAnnotationKind::ThreeD { entries: Vec::new() },
        "redact" => PdfAnnotationKind::Redact { quad_points: Vec::new(), interior_color: None, overlay_text: None, repeat: false, default_appearance: None, quadding: 0 },
        other => return Err(fault(format!("unknown annotation kind '{other}'"))),
    })
}

pub(super) fn set_annotation_kind(snapshot: &PdfSnapshot, page: usize, object_id: &str, aspect: &str, value: &str, x: f64, y: f64, width: f64, height: f64, red: f64, green: f64, blue: f64) -> Result<Vec<PdfMutation>, Fault> {
    let (index, mut annotation) = annotation_at(snapshot, page, object_id)?;
    if aspect == "kind" {
        annotation.kind = annotation_kind(value)?;
        return Ok(store_annotation(page, index, annotation));
    }
    let color = if value == "clear" { None } else { Some(vec![red, green, blue]) };
    let differences = if x < 0.0 { None } else { Some([x, y, x + width, y + height]) };
    let unknown = |field: &str| fault(format!("this annotation has no '{field}'"));
    match &mut annotation.kind {
        PdfAnnotationKind::Text { open, icon, state, state_model } => match aspect {
            "open" => *open = x >= 0.5,
            "icon" => *icon = none_if_empty(value),
            "state" => *state = none_if_empty(value),
            "stateModel" => *state_model = none_if_empty(value),
            other => return Err(unknown(other)),
        },
        PdfAnnotationKind::Link { action, destination, highlight, quad_points } => match aspect {
            "uri" => *action = none_if_empty(value).map(PdfAction::uri),
            "page" => *destination = (x >= 0.0).then_some(PdfDestination::Page { page: x.round() as u32, fit: PdfDestinationFit::Fit }),
            "highlight" => *highlight = none_if_empty(value),
            "quads" => *quad_points = if value.is_empty() { Vec::new() } else { csv_numbers(value)? },
            other => return Err(unknown(other)),
        },
        PdfAnnotationKind::FreeText { default_appearance, quadding, callout, line_ending, rich_text } => match aspect {
            "appearance" => *default_appearance = value.to_string(),
            "quadding" => *quadding = x.max(0.0).round() as u32,
            "callout" => *callout = if value.is_empty() { None } else { Some(csv_numbers(value)?) },
            "ending" => *line_ending = none_if_empty(value),
            "rich" => *rich_text = none_if_empty(value),
            other => return Err(unknown(other)),
        },
        PdfAnnotationKind::Line { points, line_endings, interior_color, leader_length, caption } => match aspect {
            "points" => {
                let numbers = csv_numbers(value)?;
                let [x1, y1, x2, y2] = <[f64; 4]>::try_from(numbers).map_err(|_| fault("a line needs four coordinates"))?;
                *points = [x1, y1, x2, y2];
            }
            "endings" => *line_endings = two_names(value)?,
            "interior" => *interior_color = color,
            "leader" => *leader_length = (x >= 0.0).then_some(x),
            "caption" => *caption = x >= 0.5,
            other => return Err(unknown(other)),
        },
        PdfAnnotationKind::Square { interior_color, rect_differences } | PdfAnnotationKind::Circle { interior_color, rect_differences } => match aspect {
            "interior" => *interior_color = color,
            "differences" => *rect_differences = differences,
            other => return Err(unknown(other)),
        },
        PdfAnnotationKind::Polygon { vertices, interior_color } => match aspect {
            "vertices" => *vertices = if value.is_empty() { Vec::new() } else { csv_numbers(value)? },
            "interior" => *interior_color = color,
            other => return Err(unknown(other)),
        },
        PdfAnnotationKind::PolyLine { vertices, line_endings, interior_color } => match aspect {
            "vertices" => *vertices = if value.is_empty() { Vec::new() } else { csv_numbers(value)? },
            "endings" => *line_endings = two_names(value)?,
            "interior" => *interior_color = color,
            other => return Err(unknown(other)),
        },
        PdfAnnotationKind::Highlight { quad_points } | PdfAnnotationKind::Underline { quad_points } | PdfAnnotationKind::Squiggly { quad_points } | PdfAnnotationKind::StrikeOut { quad_points } => match aspect {
            "quads" => *quad_points = if value.is_empty() { Vec::new() } else { csv_numbers(value)? },
            other => return Err(unknown(other)),
        },
        PdfAnnotationKind::Stamp { icon } => match aspect {
            "icon" => *icon = none_if_empty(value),
            other => return Err(unknown(other)),
        },
        PdfAnnotationKind::Caret { rect_differences, symbol } => match aspect {
            "differences" => *rect_differences = differences,
            "symbol" => *symbol = none_if_empty(value),
            other => return Err(unknown(other)),
        },
        PdfAnnotationKind::Ink { paths } => match aspect {
            "paths" => *paths = ink_paths(value)?,
            other => return Err(unknown(other)),
        },
        PdfAnnotationKind::Popup { parent, open } => match aspect {
            "parent" => *parent = (x >= 0.0).then_some(x.round() as u64),
            "open" => *open = x >= 0.5,
            other => return Err(unknown(other)),
        },
        PdfAnnotationKind::FileAttachment { file, icon } => match aspect {
            "embedded" => *file = PdfFileSpecification::Embedded { file: value.to_string() },
            "path" => *file = PdfFileSpecification::Path { path: value.to_string() },
            "icon" => *icon = none_if_empty(value),
            other => return Err(unknown(other)),
        },
        PdfAnnotationKind::Sound { sound, icon } => match aspect {
            "icon" => *icon = none_if_empty(value),
            "entry" => {
                let (key, entry) = name_entry(value)?;
                upsert_name_entry(sound, key, entry)?;
            }
            other => return Err(unknown(other)),
        },
        PdfAnnotationKind::Movie { title, movie, activation } => match aspect {
            "title" => *title = none_if_empty(value),
            "entry" => {
                let (key, entry) = name_entry(value)?;
                upsert_name_entry(movie, key, entry)?;
            }
            "activation" => {
                let entries = activation.get_or_insert_with(Vec::new);
                let (key, entry) = name_entry(value)?;
                upsert_name_entry(entries, key, entry)?;
            }
            other => return Err(unknown(other)),
        },
        PdfAnnotationKind::Widget { field, highlight, characteristics, action, additional_actions } => match aspect {
            "field" => *field = none_if_empty(value),
            "highlight" => *highlight = none_if_empty(value),
            "uri" => *action = none_if_empty(value).map(PdfAction::uri),
            "entry" => {
                let (key, entry) = name_entry(value)?;
                upsert_name_entry(characteristics, key, entry)?;
            }
            "action" => {
                let (key, entry) = name_entry(value)?;
                upsert_name_entry(additional_actions, key, entry)?;
            }
            other => return Err(unknown(other)),
        },
        PdfAnnotationKind::Screen { title, characteristics, action, additional_actions } => match aspect {
            "title" => *title = none_if_empty(value),
            "uri" => *action = none_if_empty(value).map(PdfAction::uri),
            "entry" => {
                let (key, entry) = name_entry(value)?;
                upsert_name_entry(characteristics, key, entry)?;
            }
            "action" => {
                let (key, entry) = name_entry(value)?;
                upsert_name_entry(additional_actions, key, entry)?;
            }
            other => return Err(unknown(other)),
        },
        PdfAnnotationKind::PrinterMark { mark_style, colorants } => match aspect {
            "style" => *mark_style = none_if_empty(value),
            "entry" => {
                let (key, entry) = name_entry(value)?;
                upsert_name_entry(colorants, key, entry)?;
            }
            other => return Err(unknown(other)),
        },
        PdfAnnotationKind::TrapNet { entries } | PdfAnnotationKind::ThreeD { entries } => match aspect {
            "entry" => {
                let (key, entry) = name_entry(value)?;
                upsert_name_entry(entries, key, entry)?;
            }
            other => return Err(unknown(other)),
        },
        PdfAnnotationKind::Watermark { fixed_print } => match aspect {
            "entry" => {
                let entries = fixed_print.get_or_insert_with(Vec::new);
                let (key, entry) = name_entry(value)?;
                upsert_name_entry(entries, key, entry)?;
            }
            other => return Err(unknown(other)),
        },
        PdfAnnotationKind::Redact { quad_points, interior_color, overlay_text, repeat, default_appearance, quadding } => match aspect {
            "quads" => *quad_points = if value.is_empty() { Vec::new() } else { csv_numbers(value)? },
            "interior" => *interior_color = color,
            "overlay" => *overlay_text = none_if_empty(value),
            "repeat" => *repeat = x >= 0.5,
            "appearance" => *default_appearance = none_if_empty(value),
            "quadding" => *quadding = x.max(0.0).round() as u32,
            other => return Err(unknown(other)),
        },
        PdfAnnotationKind::Unknown { subtype, entries } => match aspect {
            "subtype" => *subtype = value.to_string(),
            "entry" => {
                let (key, entry) = name_entry(value)?;
                upsert_name_entry(entries, key, entry)?;
            }
            other => return Err(unknown(other)),
        },
    }
    Ok(store_annotation(page, index, annotation))
}

fn choice_options(value: &str) -> Vec<(String, String)> {
    value.split(',').map(str::trim).filter(|item| !item.is_empty()).map(|item| match item.split_once('=') { Some((export, display)) => (export.to_string(), display.to_string()), None => (item.to_string(), item.to_string()) }).collect()
}

fn csv_strings(value: &str) -> Vec<String> {
    value.split(',').map(str::trim).filter(|item| !item.is_empty()).map(str::to_string).collect()
}

pub(super) fn set_field_data(snapshot: &PdfSnapshot, name: &str, aspect: &str, value: &str, number: f64) -> Result<Vec<PdfMutation>, Fault> {
    if name.is_empty() {
        return Err(fault("a form field needs a name"));
    }
    let mut form = snapshot.acro_form.clone().ok_or_else(|| fault("the document has no form"))?;
    let field = form.fields.iter_mut().find(|field| field.name == name).ok_or_else(|| fault(format!("pdf form field '{name}' is gone")))?;
    match &mut field.kind {
        PdfFormFieldKind::Choice { values, default_values, options, top_index } => match aspect {
            "options" => *options = choice_options(value),
            "values" => *values = csv_strings(value),
            "defaults" => *default_values = csv_strings(value),
            "top" => *top_index = (number >= 0.0).then_some(number.round() as u32),
            other => return Err(fault(format!("a choice field has no '{other}'"))),
        },
        PdfFormFieldKind::Text { default_value, max_length, rich_value, .. } => match aspect {
            "default" => *default_value = none_if_empty(value),
            "maxLength" => *max_length = (number >= 0.0).then_some(number.round() as u32),
            "rich" => *rich_value = none_if_empty(value),
            other => return Err(fault(format!("a text field has no '{other}'"))),
        },
        PdfFormFieldKind::Button { default_value, options, .. } => match aspect {
            "default" => *default_value = none_if_empty(value),
            "options" => *options = csv_strings(value),
            other => return Err(fault(format!("a button field has no '{other}'"))),
        },
        PdfFormFieldKind::Signature { value: signature } => match aspect {
            "entry" => {
                let entries = signature.get_or_insert_with(Vec::new);
                let (key, entry) = name_entry(value)?;
                upsert_name_entry(entries, key, entry)?;
            }
            other => return Err(fault(format!("a signature field has no '{other}'"))),
        },
        PdfFormFieldKind::Container => return Err(fault("a container field has no value")),
    }
    Ok(vec![PdfMutation::SetAcroForm(SetAcroForm { form: Some(form) })])
}

pub(super) fn set_annotation_style(snapshot: &PdfSnapshot, page: usize, object_id: &str, aspect: &str, value: &str, red: f64, green: f64, blue: f64, flags: f64) -> Result<Vec<PdfMutation>, Fault> {
    let index = object_id.rsplit_once(":a").and_then(|(_, raw)| raw.parse::<usize>().ok()).ok_or_else(|| fault(format!("pdf annotation '{object_id}' is not addressable")))?;
    let mut annotation = snapshot.pages.get(page).and_then(|item| item.annotations.get(index)).cloned().ok_or_else(|| fault(format!("pdf annotation '{object_id}' is gone")))?;
    match aspect {
        "color" => annotation.color = if value == "clear" { Vec::new() } else { vec![red, green, blue] },
        "flags" => annotation.flags = flags.max(0.0).round() as u32,
        "name" => annotation.name = none_if_empty(value),
        "state" => annotation.appearance_state = none_if_empty(value),
        "modified" => annotation.modified = none_if_empty(value),
        other => return Err(fault(format!("unknown annotation style '{other}'"))),
    }
    Ok(vec![PdfMutation::SetAnnotation(SetAnnotation { index: page, at: index, annotation })])
}

pub(super) fn set_page_transition(snapshot: &PdfSnapshot, page: usize, style: &str, direction: &str, duration: f64) -> Result<Vec<PdfMutation>, Fault> {
    require_page(snapshot, page)?;
    let mut next = snapshot.pages[page].clone();
    let target = &mut next;
    target.transition = if style.is_empty() {
        None
    } else {
        let mut entries = vec![PdfDictEntry::new("S", PdfObject::Name(style.to_string()))];
        if !direction.is_empty() {
            entries.push(PdfDictEntry::new("Di", PdfObject::Name(direction.to_string())));
        }
        Some(entries)
    };
    if duration < 0.0 {
        target.duration = None;
    } else if duration > 0.0 {
        target.duration = Some(duration);
    }
    Ok(vec![PdfMutation::ReplacePage(ReplacePage { index: page, page: next })])
}

pub(super) fn set_named_object(key: &str, value: &str, missing: &str, remove: impl FnOnce(String) -> PdfMutation, set: impl FnOnce(String, PdfObject) -> PdfMutation) -> Result<Vec<PdfMutation>, Fault> {
    if key.is_empty() {
        return Err(fault(missing));
    }
    Ok(vec![if value.is_empty() { remove(key.to_string()) } else { set(key.to_string(), PdfObject::Name(value.to_string())) }])
}

pub(super) fn set_annotation_appearance(snapshot: &PdfSnapshot, page: usize, object_id: &str, form: &str) -> Result<Vec<PdfMutation>, Fault> {
    let index = object_id.rsplit_once(":a").and_then(|(_, raw)| raw.parse::<usize>().ok()).ok_or_else(|| fault(format!("pdf annotation '{object_id}' is not addressable")))?;
    let mut annotation = snapshot.pages.get(page).and_then(|item| item.annotations.get(index)).cloned().ok_or_else(|| fault(format!("pdf annotation '{object_id}' is gone")))?;
    annotation.appearance = if form.is_empty() { None } else { Some(PdfAppearance { normal: PdfAppearanceEntry::Single { form: form.to_string() }, rollover: None, down: None }) };
    Ok(vec![PdfMutation::SetAnnotation(SetAnnotation { index: page, at: index, annotation })])
}
//#endregion 🔖️AnnotationEdits
