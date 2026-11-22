//! ✍️ PDF page editor authoring: new annotations, form value fill with appearance regeneration, page replace, merge and extract.

use semio_framework_io_base64::{base64_standard_decode_controlled, base64_standard_encode_controlled, Base64Control};
use semio_plugin::Fault;
use semio_s_artifact_stdio_pdf_editor_document::{fault, none_if_empty};
use semio_s_artifact_stdio_pdf_io_export::standards::v1_7::subsets::base::io::{export::encode_pdf, import::decode_pdf, mutation_bridge::apply_pdf_mutation};
use semio_s_artifact_stdio_pdf_mut_family_8::standards::v1_7::subsets::base::schema::mutations::{InsertAnnotation, InsertPage, PdfMutation, RemoveCatalogEntry, RemoveTrailerEntry, SetCatalogEntry, SetInfo, SetTrailerEntry, SetViewerPreferences, RemoveNamedDestination, ReplacePage, SetNamedDestination, SetOutlines, SetAcroForm, SetAnnotation, SetColorSpace, SetExtGState, SetFont, SetForm, SetImage, SetPattern, SetProperties, SetShading};
use semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::schema::snapshot::{
    PdfAcroForm, PdfAnnotation, PdfAnnotationKind, PdfAppearance, PdfAppearanceEntry, PdfAppearanceState, PdfBorderStyle, PdfDestination, PdfDestinationFit, PdfExtGState, PdfFont, PdfFontKind, PdfFormField, PdfFormFieldKind, PdfDictEntry, PdfFormXObject, PdfNamedDestination, PdfObject, PdfOp, PdfViewerPreferences, PdfOutlineItem, PdfPage, PdfPattern, PdfPatternKind, PdfPropertyList, PdfRect, PdfSnapshot, PdfSoftMask, PdfTextString,
};
use serde_json::json;
use std::collections::HashSet;
use std::time::{Duration, Instant};

//#region ✍️Authoring
pub const MAX_SOURCE_BYTES: u32 = 16 * 1024 * 1024;
pub const OPERATION_DEADLINE: Duration = Duration::from_secs(20);
const MULTIPLY: &str = "MarkupMultiply";

/// ⏳ One progress report of a merge, replace or extract: the phase and how much of it is done.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Progress {
    pub phase: &'static str,
    pub done: usize,
    pub total: usize,
}

pub type Control<'a> = &'a mut dyn FnMut(Progress) -> bool;

/// 🛑 A cancellation source that refuses further work once `limit` has passed.
pub fn deadline_control(limit: Duration) -> impl FnMut(Progress) -> bool {
    let start = Instant::now();
    move |_| start.elapsed() < limit
}

fn step(control: &mut dyn FnMut(Progress) -> bool, phase: &'static str, done: usize, total: usize) -> Result<(), Fault> {
    if control(Progress { phase, done, total }) { Ok(()) } else { Err(fault(format!("the {phase} step was cancelled"))) }
}
const FLAG_READ_ONLY: u32 = 1;
const FLAG_RADIO: u32 = 1 << 15;
const FLAG_PUSH_BUTTON: u32 = 1 << 16;
const FLAG_COMBO: u32 = 1 << 17;
const FLAG_EDIT: u32 = 1 << 18;
const FLAG_MULTI_SELECT: u32 = 1 << 21;
const FLAG_PASSWORD: u32 = 1 << 13;

fn csv_numbers(text: &str) -> Result<Vec<f64>, Fault> {
    text.split(',').map(str::trim).filter(|item| !item.is_empty()).map(|item| item.parse::<f64>().map_err(|_| fault("a list needs numbers"))).collect()
}

fn rect_of(x: f64, y: f64, width: f64, height: f64) -> Result<PdfRect, Fault> {
    if ![x, y, width, height].iter().all(|value| value.is_finite()) || width <= 0.0 || height <= 0.0 {
        return Err(fault("an annotation needs a positive rectangle"));
    }
    Ok([x, y, x + width, y + height])
}

fn quads_of(rect: PdfRect, extra: &str) -> Result<Vec<f64>, Fault> {
    if extra.is_empty() {
        let [left, bottom, right, top] = rect;
        return Ok(vec![left, top, right, top, left, bottom, right, bottom]);
    }
    let quads = csv_numbers(extra)?;
    if quads.is_empty() || quads.len() % 8 != 0 {
        return Err(fault("quad points come in groups of eight numbers"));
    }
    Ok(quads)
}

fn ink_of(extra: &str) -> Result<(Vec<Vec<f64>>, PdfRect), Fault> {
    let paths = extra.split(';').filter(|path| !path.trim().is_empty()).map(csv_numbers).collect::<Result<Vec<_>, _>>()?;
    if paths.is_empty() || paths.iter().any(|path| path.len() < 4 || path.len() % 2 != 0) {
        return Err(fault("an ink stroke needs at least two points"));
    }
    let points = || paths.iter().flat_map(|path| path.chunks(2));
    let fold = |select: fn(&[f64]) -> f64, join: fn(f64, f64) -> f64, start: f64| points().map(select).fold(start, join);
    let (left, right) = (fold(|point| point[0], f64::min, f64::INFINITY), fold(|point| point[0], f64::max, f64::NEG_INFINITY));
    let (bottom, top) = (fold(|point| point[1], f64::min, f64::INFINITY), fold(|point| point[1], f64::max, f64::NEG_INFINITY));
    Ok((paths, [left - 1.0, bottom - 1.0, right + 1.0, top + 1.0]))
}

/// 📌 Creates one annotation (note, highlight, underline, strike-out, ink or link) at the end of the page's annotation list; see [`InsertAnnotation`].
pub fn create_annotation(snapshot: &PdfSnapshot, page: usize, kind: &str, text: &str, extra: &str, x: f64, y: f64, width: f64, height: f64, color: [f64; 3]) -> Result<Vec<PdfMutation>, Fault> {
    let target = snapshot.pages.get(page).ok_or_else(|| fault(format!("pdf page {page} does not exist")))?;
    let tinted = |fallback: [f64; 3]| (if color == [0.0; 3] { fallback } else { color }).to_vec();
    let mut annotation = match kind {
        "note" => {
            let (width, height) = (if width > 0.0 { width } else { 20.0 }, if height > 0.0 { height } else { 20.0 });
            let mut note = PdfAnnotation::new(rect_of(x, y, width, height)?, PdfAnnotationKind::Text { open: false, icon: Some("Note".into()), state: None, state_model: None });
            note.flags = 28;
            note.color = tinted([1.0, 0.9, 0.2]);
            note
        }
        "highlight" | "underline" | "strikeOut" | "squiggly" => {
            let rect = rect_of(x, y, width, height)?;
            let quad_points = quads_of(rect, extra)?;
            let kind = match kind {
                "highlight" => PdfAnnotationKind::Highlight { quad_points },
                "underline" => PdfAnnotationKind::Underline { quad_points },
                "squiggly" => PdfAnnotationKind::Squiggly { quad_points },
                _ => PdfAnnotationKind::StrikeOut { quad_points },
            };
            let mut markup = PdfAnnotation::new(rect, kind);
            markup.color = tinted(if matches!(markup.kind, PdfAnnotationKind::Highlight { .. }) { [1.0, 1.0, 0.0] } else { [1.0, 0.0, 0.0] });
            markup
        }
        "ink" => {
            let (paths, rect) = ink_of(extra)?;
            let mut ink = PdfAnnotation::new(rect, PdfAnnotationKind::Ink { paths });
            ink.border = Some(PdfBorderStyle { width: if width > 0.0 { width } else { 2.0 }, style: Some("S".into()), dash: None, radii: None });
            ink.color = color.to_vec();
            ink
        }
        "link" => {
            let mut link = PdfAnnotation::link(rect_of(x, y, width, height)?, text);
            if let Some(number) = text.strip_prefix("page:") {
                let destination = number.trim().parse::<usize>().ok().filter(|number| (1..=snapshot.pages.len()).contains(number)).ok_or_else(|| fault("a page link needs an existing page number"))?;
                link.kind = PdfAnnotationKind::Link { action: None, destination: Some(PdfDestination::Page { page: (destination - 1) as u32, fit: PdfDestinationFit::Fit }), highlight: None, quad_points: Vec::new() };
            } else if text.is_empty() {
                return Err(fault("a link needs an address or page:N"));
            }
            return Ok(vec![PdfMutation::InsertAnnotation(InsertAnnotation { index: page, at: target.annotations.len(), annotation: link })]);
        }
        other => return Err(fault(format!("unknown annotation to create '{other}'"))),
    };
    annotation.contents = none_if_empty(text);
    let at = target.annotations.len();
    let id = format!("Mk{page}_{at}");
    let mut mutations = Vec::new();
    if let Some(form) = markup_form(&id, &annotation) {
        if matches!(annotation.kind, PdfAnnotationKind::Highlight { .. }) && !snapshot.ext_g_states.iter().any(|state| state.id == MULTIPLY) {
            mutations.push(PdfMutation::SetExtGState(SetExtGState { state: PdfExtGState { id: MULTIPLY.into(), blend_mode: Some(vec!["Multiply".into()]), ..Default::default() }, index: None }));
        }
        mutations.push(PdfMutation::SetForm(SetForm { form, index: None }));
        annotation.appearance = Some(single(&id));
    }
    mutations.push(PdfMutation::InsertAnnotation(InsertAnnotation { index: page, at, annotation }));
    Ok(mutations)
}

fn markup_form(id: &str, annotation: &PdfAnnotation) -> Option<PdfFormXObject> {
    let channel = |index: usize| annotation.color.get(index).copied().unwrap_or(0.0);
    let (r, g, b) = (channel(0), channel(1), channel(2));
    let stroke = vec![PdfOp::SetStrokeRgb { r, g, b }, PdfOp::SetLineWidth { width: 1.0 }];
    let content = match &annotation.kind {
        PdfAnnotationKind::Highlight { quad_points } => {
            let mut content = vec![PdfOp::SetExtGState { name: MULTIPLY.into() }, PdfOp::SetFillRgb { r, g, b }];
            for quad in quad_points.chunks_exact(8) {
                content.extend([PdfOp::MoveTo { x: quad[0], y: quad[1] }, PdfOp::LineTo { x: quad[2], y: quad[3] }, PdfOp::LineTo { x: quad[6], y: quad[7] }, PdfOp::LineTo { x: quad[4], y: quad[5] }, PdfOp::ClosePath, PdfOp::Fill]);
            }
            content
        }
        PdfAnnotationKind::Underline { quad_points } | PdfAnnotationKind::StrikeOut { quad_points } => {
            let strike = matches!(annotation.kind, PdfAnnotationKind::StrikeOut { .. });
            let mut content = stroke;
            for quad in quad_points.chunks_exact(8) {
                let y = if strike { (quad[1] + quad[5]) / 2.0 } else { quad[5] };
                content.extend([PdfOp::MoveTo { x: quad[4], y }, PdfOp::LineTo { x: quad[6], y }, PdfOp::Stroke]);
            }
            content
        }
        PdfAnnotationKind::Squiggly { quad_points } => {
            let mut content = stroke;
            for quad in quad_points.chunks_exact(8) {
                let steps = (((quad[6] - quad[4]) / 2.0).ceil().max(1.0) as usize).min(10_000);
                content.push(PdfOp::MoveTo { x: quad[4], y: quad[5] });
                content.extend((1..=steps).map(|index| PdfOp::LineTo { x: (quad[4] + index as f64 * 2.0).min(quad[6]), y: quad[5] + if index % 2 == 0 { 0.0 } else { 2.0 } }));
                content.push(PdfOp::Stroke);
            }
            content
        }
        PdfAnnotationKind::Ink { paths } => {
            let mut content = vec![PdfOp::SetStrokeRgb { r, g, b }, PdfOp::SetLineWidth { width: annotation.border.as_ref().map_or(1.0, |border| border.width) }];
            for path in paths {
                let mut points = path.chunks_exact(2);
                if let Some(first) = points.next() {
                    content.push(PdfOp::MoveTo { x: first[0], y: first[1] });
                    content.extend(points.map(|point| PdfOp::LineTo { x: point[0], y: point[1] }));
                    content.push(PdfOp::Stroke);
                }
            }
            content
        }
        PdfAnnotationKind::Text { .. } => {
            let [left, bottom, right, top] = annotation.rect;
            vec![PdfOp::SetFillRgb { r, g, b }, PdfOp::SetStrokeGray { gray: 0.0 }, PdfOp::SetLineWidth { width: 1.0 }, PdfOp::Rectangle { x: left, y: bottom, width: right - left, height: top - bottom }, PdfOp::FillStroke]
        }
        _ => return None,
    };
    Some(PdfFormXObject::new(id, annotation.rect, content))
}

fn find_field<'a>(fields: &'a mut [PdfFormField], path: &str, prefix: &str) -> Option<&'a mut PdfFormField> {
    for field in fields.iter_mut() {
        let full = if prefix.is_empty() { field.name.clone() } else { format!("{prefix}.{}", field.name) };
        if full == path {
            return Some(field);
        }
        if path.starts_with(&format!("{full}.")) {
            return find_field(&mut field.children, path, &full);
        }
    }
    None
}

fn on_state(annotation: &PdfAnnotation) -> Option<String> {
    match &annotation.appearance.as_ref()?.normal {
        PdfAppearanceEntry::States { states } => states.iter().map(|entry| entry.state.clone()).find(|state| state != "Off"),
        PdfAppearanceEntry::Single { .. } => None,
    }
}

fn truthy(value: &str) -> bool {
    !matches!(value.trim().to_ascii_lowercase().as_str(), "" | "off" | "false" | "0" | "no")
}

fn appearance_parts(default_appearance: &str) -> (String, f64, Vec<PdfOp>) {
    let tokens: Vec<&str> = default_appearance.split_whitespace().collect();
    let number = |index: usize| tokens.get(index).and_then(|token| token.parse::<f64>().ok());
    let font = tokens.iter().position(|token| *token == "Tf").filter(|position| *position >= 2);
    let (name, size) = font.map_or((String::new(), 0.0), |position| (tokens[position - 2].trim_start_matches('/').to_string(), number(position - 1).unwrap_or(0.0)));
    let color = tokens.iter().position(|token| matches!(*token, "g" | "rg")).map_or(PdfOp::SetFillGray { gray: 0.0 }, |position| match tokens[position] {
        "g" => PdfOp::SetFillGray { gray: position.checked_sub(1).and_then(number).unwrap_or(0.0) },
        _ => PdfOp::SetFillRgb { r: position.checked_sub(3).and_then(number).unwrap_or(0.0), g: position.checked_sub(2).and_then(number).unwrap_or(0.0), b: position.checked_sub(1).and_then(number).unwrap_or(0.0) },
    });
    (name, size, vec![color])
}

fn text_form(snapshot: &PdfSnapshot, id: &str, rect: PdfRect, text: &str, default_appearance: &str, quadding: u32) -> Option<PdfFormXObject> {
    let (name, size, color) = appearance_parts(default_appearance);
    let font = snapshot.fonts.iter().find(|font| font.id == name).or_else(|| snapshot.fonts.first())?.id.clone();
    let (width, height) = (rect[2] - rect[0], rect[3] - rect[1]);
    let size = if size > 0.0 { size } else { (height - 4.0).clamp(4.0, 12.0) };
    let advance = text.chars().count() as f64 * size * 0.5;
    let left = match quadding {
        1 => (width - advance) / 2.0,
        2 => width - 2.0 - advance,
        _ => 2.0,
    }
    .max(2.0);
    let baseline = (height - size) / 2.0 + size * 0.22;
    let mut content = vec![PdfOp::BeginMarkedContent { tag: "Tx".into() }, PdfOp::Save, PdfOp::Rectangle { x: 1.0, y: 1.0, width: (width - 2.0).max(1.0), height: (height - 2.0).max(1.0) }, PdfOp::Clip, PdfOp::EndPath, PdfOp::BeginText, PdfOp::SetFont { name: font, size }];
    content.extend(color);
    content.extend([PdfOp::SetTextMatrix { matrix: [1.0, 0.0, 0.0, 1.0, left, baseline] }, PdfOp::ShowText { text: PdfTextString::text(text) }, PdfOp::EndText, PdfOp::Restore, PdfOp::EndMarkedContent]);
    Some(PdfFormXObject::new(id, [0.0, 0.0, width, height], content))
}

fn button_form(id: &str, rect: PdfRect, radio: bool, on: bool) -> PdfFormXObject {
    let (width, height) = (rect[2] - rect[0], rect[3] - rect[1]);
    let content = match (on, radio) {
        (false, _) => Vec::new(),
        (true, false) => vec![PdfOp::SetLineWidth { width: (height * 0.12).max(1.0) }, PdfOp::SetStrokeGray { gray: 0.0 }, PdfOp::MoveTo { x: width * 0.2, y: height * 0.5 }, PdfOp::LineTo { x: width * 0.42, y: height * 0.22 }, PdfOp::LineTo { x: width * 0.8, y: height * 0.8 }, PdfOp::Stroke],
        (true, true) => {
            let (cx, cy, rx, ry, k) = (width / 2.0, height / 2.0, width * 0.25, height * 0.25, 0.5523);
            vec![
                PdfOp::SetFillGray { gray: 0.0 },
                PdfOp::MoveTo { x: cx + rx, y: cy },
                PdfOp::CurveTo { x1: cx + rx, y1: cy + ry * k, x2: cx + rx * k, y2: cy + ry, x3: cx, y3: cy + ry },
                PdfOp::CurveTo { x1: cx - rx * k, y1: cy + ry, x2: cx - rx, y2: cy + ry * k, x3: cx - rx, y3: cy },
                PdfOp::CurveTo { x1: cx - rx, y1: cy - ry * k, x2: cx - rx * k, y2: cy - ry, x3: cx, y3: cy - ry },
                PdfOp::CurveTo { x1: cx + rx * k, y1: cy - ry, x2: cx + rx, y2: cy - ry * k, x3: cx + rx, y3: cy },
                PdfOp::Fill,
            ]
        }
    };
    PdfFormXObject::new(id, [0.0, 0.0, width, height], content)
}

fn widget_rect(snapshot: &PdfSnapshot, widget: [u32; 2]) -> Result<PdfAnnotation, Fault> {
    snapshot.pages.get(widget[0] as usize).and_then(|page| page.annotations.get(widget[1] as usize)).cloned().ok_or_else(|| fault("a form widget points at a missing annotation"))
}

fn single(form: &str) -> PdfAppearance {
    PdfAppearance { normal: PdfAppearanceEntry::Single { form: form.to_string() }, rollover: None, down: None }
}

/// 📝 Fills one form field with `value` and regenerates the appearance of each of its widgets; see §12.7.4.
pub fn fill_form_field(snapshot: &PdfSnapshot, name: &str, value: &str) -> Result<Vec<PdfMutation>, Fault> {
    let mut form = snapshot.acro_form.clone().ok_or_else(|| fault("the document has no form"))?;
    let (form_appearance, form_quadding) = (form.default_appearance.clone().unwrap_or_default(), form.quadding.unwrap_or(0));
    let field = find_field(&mut form.fields, name, "").ok_or_else(|| fault(format!("pdf form field '{name}' is gone")))?;
    if field.flags & FLAG_READ_ONLY != 0 {
        return Err(fault(format!("pdf form field '{name}' is read only")));
    }
    let widgets = field.widgets.clone();
    let mut forms = Vec::new();
    let mut annotations: Vec<(usize, usize, PdfAnnotation)> = Vec::new();
    let appearance = field.default_appearance.clone().unwrap_or(form_appearance);
    let quadding = field.quadding.unwrap_or(form_quadding);
    let display = match &mut field.kind {
        PdfFormFieldKind::Text { value: stored, max_length, .. } => {
            if max_length.is_some_and(|limit| value.chars().count() > limit as usize) {
                return Err(fault(format!("the value is longer than the {} characters '{name}' allows", max_length.unwrap_or(0))));
            }
            *stored = none_if_empty(value);
            Some(if field.flags & FLAG_PASSWORD != 0 { "*".repeat(value.chars().count()) } else { value.to_string() })
        }
        PdfFormFieldKind::Choice { values, options, .. } => {
            let chosen: Vec<String> = value.split(',').map(str::trim).filter(|item| !item.is_empty()).map(str::to_string).collect();
            if chosen.len() > 1 && field.flags & FLAG_MULTI_SELECT == 0 {
                return Err(fault(format!("pdf form field '{name}' accepts one value")));
            }
            let editable = field.flags & FLAG_COMBO != 0 && field.flags & FLAG_EDIT != 0;
            if let Some(unknown) = chosen.iter().find(|item| !editable && !options.iter().any(|(export, _)| export == *item)) {
                return Err(fault(format!("'{unknown}' is not an option of '{name}'")));
            }
            let shown = chosen.first().map(|item| options.iter().find(|(export, _)| export == item).map_or(item.clone(), |(_, display)| display.clone()));
            *values = chosen;
            shown
        }
        PdfFormFieldKind::Button { value: stored, options, .. } => {
            if field.flags & FLAG_PUSH_BUTTON != 0 {
                return Err(fault(format!("pdf form field '{name}' is a push button and holds no value")));
            }
            let radio = field.flags & FLAG_RADIO != 0;
            let named = truthy(value) && !matches!(value.trim().to_ascii_lowercase().as_str(), "true" | "on" | "1" | "yes");
            let states: Vec<String> = widgets.iter().enumerate().map(|(position, widget)| widget_rect(snapshot, *widget).ok().and_then(|annotation| on_state(&annotation)).or_else(|| options.get(position).cloned().filter(|_| radio)).unwrap_or_else(|| if radio { position.to_string() } else { "Yes".into() })).collect();
            let chosen = if !truthy(value) {
                "Off".to_string()
            } else if named {
                if !states.iter().any(|state| state == value.trim()) && !states.is_empty() {
                    return Err(fault(format!("'{}' is not a state of '{name}'", value.trim())));
                }
                value.trim().to_string()
            } else {
                states.first().cloned().unwrap_or_else(|| "Yes".into())
            };
            *stored = Some(chosen.clone());
            for (position, widget) in widgets.iter().enumerate() {
                let mut annotation = widget_rect(snapshot, *widget)?;
                let state = if states[position] == chosen { chosen.clone() } else { "Off".to_string() };
                if on_state(&annotation).is_none() {
                    let (on_id, off_id) = (format!("Ap{}_{}_{}", widget[0], widget[1], "On"), format!("Ap{}_{}_{}", widget[0], widget[1], "Off"));
                    forms.push(button_form(&on_id, annotation.rect, radio, true));
                    forms.push(button_form(&off_id, annotation.rect, radio, false));
                    annotation.appearance = Some(PdfAppearance {
                        normal: PdfAppearanceEntry::States { states: vec![PdfAppearanceState { state: states[position].clone(), form: on_id }, PdfAppearanceState { state: "Off".into(), form: off_id }] },
                        rollover: None,
                        down: None,
                    });
                }
                annotation.appearance_state = Some(state);
                annotations.push((widget[0] as usize, widget[1] as usize, annotation));
            }
            None
        }
        PdfFormFieldKind::Signature { .. } | PdfFormFieldKind::Container => return Err(fault(format!("pdf form field '{name}' holds no fillable value"))),
    };
    let mut complete = true;
    if let Some(shown) = display {
        for widget in &widgets {
            let mut annotation = widget_rect(snapshot, *widget)?;
            let id = format!("Ap{}_{}", widget[0], widget[1]);
            match text_form(snapshot, &id, annotation.rect, &shown, &appearance, quadding) {
                Some(generated) => {
                    forms.push(generated);
                    annotation.appearance = Some(single(&id));
                    annotations.push((widget[0] as usize, widget[1] as usize, annotation));
                }
                None => complete = false,
            }
        }
    }
    if !complete {
        form.need_appearances = true;
    }
    let mut mutations: Vec<PdfMutation> = forms.into_iter().map(|form| PdfMutation::SetForm(SetForm { form, index: None })).collect();
    mutations.extend(annotations.into_iter().map(|(page, at, annotation)| PdfMutation::SetAnnotation(SetAnnotation { index: page, at, annotation })));
    mutations.push(PdfMutation::SetAcroForm(SetAcroForm { form: Some(form) }));
    Ok(mutations)
}

/// 🔢 Reads 1-based page numbers and ranges such as `1-3,5` into ascending-or-given 0-based indices.
pub fn parse_page_selection(selection: &str, count: usize) -> Result<Vec<usize>, Fault> {
    let mut picks = Vec::new();
    for part in selection.split(',').map(str::trim).filter(|part| !part.is_empty()) {
        let (first, last) = part.split_once('-').map_or((part, part), |(first, last)| (first.trim(), last.trim()));
        let (first, last) = (first.parse::<usize>(), last.parse::<usize>());
        let (Ok(first), Ok(last)) = (first, last) else { return Err(fault(format!("'{part}' is not a page range"))) };
        if first == 0 || last < first || last > count {
            return Err(fault(format!("page range '{part}' is outside the {count} pages")));
        }
        picks.extend((first..=last).map(|number| number - 1));
    }
    if picks.is_empty() {
        return Err(fault("a page selection needs at least one page"));
    }
    Ok(picks)
}

fn never_cancelled(_: semio_framework_io_base64::Base64Progress) -> bool {
    true
}

/// 📥 Decodes a picked PDF (data URL or plain base64) into a snapshot.
pub fn decode_source(payload: &str) -> Result<PdfSnapshot, Fault> {
    let encoded = payload.split_once("base64,").map_or(payload, |(_, rest)| rest);
    let mut progress = never_cancelled;
    let bytes = base64_standard_decode_controlled(encoded.trim().as_bytes(), &mut Base64Control { maximum_output_bytes: MAX_SOURCE_BYTES as usize, progress: &mut progress }).map_err(|error| fault(format!("the picked pdf is not readable: {error}")))?;
    decode_pdf(&bytes).map_err(|error| fault(format!("the picked pdf is not a valid pdf: {error}")))
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Kind {
    Font,
    XObject,
    GState,
    Shading,
    Pattern,
    Space,
    Props,
}

fn visit_ops(ops: &mut [PdfOp], visit: &mut dyn FnMut(Kind, &mut String)) {
    for op in ops {
        match op {
            PdfOp::SetExtGState { name } => visit(Kind::GState, name),
            PdfOp::SetFont { name, .. } => visit(Kind::Font, name),
            PdfOp::SetStrokeColorSpace { name } | PdfOp::SetFillColorSpace { name } => visit(Kind::Space, name),
            PdfOp::SetStrokeColorN { pattern: Some(name), .. } | PdfOp::SetFillColorN { pattern: Some(name), .. } => visit(Kind::Pattern, name),
            PdfOp::PaintShading { name } => visit(Kind::Shading, name),
            PdfOp::PaintXObject { name } => visit(Kind::XObject, name),
            PdfOp::MarkedContentPointWithProperties { properties: PdfPropertyList::Named { name }, .. } | PdfOp::BeginMarkedContentWithProperties { properties: PdfPropertyList::Named { name }, .. } => visit(Kind::Props, name),
            _ => {}
        }
    }
}

fn visit_appearance(appearance: &mut PdfAppearance, visit: &mut dyn FnMut(Kind, &mut String)) {
    let mut entry = |entry: &mut PdfAppearanceEntry| match entry {
        PdfAppearanceEntry::Single { form } => visit(Kind::XObject, form),
        PdfAppearanceEntry::States { states } => states.iter_mut().for_each(|state| visit(Kind::XObject, &mut state.form)),
    };
    entry(&mut appearance.normal);
    appearance.rollover.iter_mut().chain(appearance.down.iter_mut()).for_each(&mut entry);
}

fn visit_page(page: &mut PdfPage, visit: &mut dyn FnMut(Kind, &mut String)) {
    visit_ops(&mut page.content, visit);
    page.annotations.iter_mut().filter_map(|annotation| annotation.appearance.as_mut()).for_each(|appearance| visit_appearance(appearance, visit));
}

fn visit_font(font: &mut PdfFont, visit: &mut dyn FnMut(Kind, &mut String)) {
    if let PdfFontKind::Type3 { char_procs, .. } = &mut font.kind {
        char_procs.iter_mut().for_each(|procedure| visit_ops(&mut procedure.content, visit));
    }
}

fn visit_pattern(pattern: &mut PdfPattern, visit: &mut dyn FnMut(Kind, &mut String)) {
    match &mut pattern.kind {
        PdfPatternKind::Tiling { content, .. } => visit_ops(content, visit),
        PdfPatternKind::Shading { shading, ext_g_state } => {
            visit(Kind::Shading, shading);
            if let Some(state) = ext_g_state {
                visit(Kind::GState, state);
            }
        }
    }
}

fn visit_state(state: &mut PdfExtGState, visit: &mut dyn FnMut(Kind, &mut String)) {
    if let Some((font, _)) = &mut state.font {
        visit(Kind::Font, font);
    }
    if let Some(PdfSoftMask::Alpha { group, .. } | PdfSoftMask::Luminosity { group, .. }) = &mut state.soft_mask {
        visit(Kind::XObject, group);
    }
}

fn exists(source: &PdfSnapshot, kind: Kind, name: &str) -> bool {
    match kind {
        Kind::Font => source.fonts.iter().any(|item| item.id == name),
        Kind::XObject => source.forms.iter().any(|item| item.id == name) || source.images.iter().any(|item| item.id == name),
        Kind::GState => source.ext_g_states.iter().any(|item| item.id == name),
        Kind::Shading => source.shadings.iter().any(|item| item.id == name),
        Kind::Pattern => source.patterns.iter().any(|item| item.id == name),
        Kind::Space => source.color_spaces.iter().any(|item| item.name == name),
        Kind::Props => source.properties.iter().any(|item| item.name == name),
    }
}

fn used_resources(source: &PdfSnapshot, pages: &[PdfPage]) -> HashSet<(Kind, String)> {
    let mut used = HashSet::new();
    let mut note = |kind: Kind, name: &mut String| {
        if exists(source, kind, name) {
            used.insert((kind, name.clone()));
        }
    };
    pages.iter().cloned().for_each(|mut page| visit_page(&mut page, &mut note));
    loop {
        let before = used.len();
        let mut grown = used.clone();
        let mut note = |kind: Kind, name: &mut String| {
            if exists(source, kind, name) {
                grown.insert((kind, name.clone()));
            }
        };
        for (kind, name) in &used {
            match kind {
                Kind::XObject => {
                    source.forms.iter().filter(|form| &form.id == name).for_each(|form| visit_ops(&mut form.content.clone(), &mut note));
                    source.images.iter().filter(|image| &image.id == name).for_each(|image| image.soft_mask.clone().into_iter().for_each(|mut mask| note(Kind::XObject, &mut mask)));
                }
                Kind::Font => source.fonts.iter().filter(|font| &font.id == name).for_each(|font| visit_font(&mut font.clone(), &mut note)),
                Kind::Pattern => source.patterns.iter().filter(|pattern| &pattern.id == name).for_each(|pattern| visit_pattern(&mut pattern.clone(), &mut note)),
                Kind::GState => source.ext_g_states.iter().filter(|state| &state.id == name).for_each(|state| visit_state(&mut state.clone(), &mut note)),
                _ => {}
            }
        }
        used = grown;
        if used.len() == before {
            return used;
        }
    }
}

fn unused_prefix(snapshot: &PdfSnapshot) -> String {
    let taken = |prefix: &str| {
        snapshot.fonts.iter().map(|item| &item.id).chain(snapshot.images.iter().map(|item| &item.id)).chain(snapshot.forms.iter().map(|item| &item.id)).chain(snapshot.ext_g_states.iter().map(|item| &item.id)).chain(snapshot.shadings.iter().map(|item| &item.id)).chain(snapshot.patterns.iter().map(|item| &item.id)).chain(snapshot.color_spaces.iter().map(|item| &item.name)).chain(snapshot.properties.iter().map(|item| &item.name)).any(|name| name.starts_with(prefix))
    };
    (1..).map(|number| format!("m{number}_")).find(|prefix| !taken(prefix)).unwrap_or_default()
}

pub enum ImportTarget {
    Insert(usize),
    Replace(usize),
}

fn page_destination(page: &mut PdfPage, positions: &[Option<u32>]) {
    for annotation in &mut page.annotations {
        if let PdfAnnotationKind::Link { destination, .. } = &mut annotation.kind {
            if let Some(PdfDestination::Page { page: target, fit }) = destination.clone() {
                *destination = positions.get(target as usize).copied().flatten().map(|page| PdfDestination::Page { page, fit });
            }
        }
    }
}

fn retarget_fields(fields: &mut [PdfFormField], map: &dyn Fn(u32) -> Option<u32>) {
    for field in fields {
        field.widgets = field.widgets.iter().filter_map(|[page, annotation]| map(*page).map(|page| [page, *annotation])).collect();
        retarget_fields(&mut field.children, map);
    }
}

fn retarget_destination(destination: &mut Option<PdfDestination>, map: &dyn Fn(u32) -> Option<u32>) {
    if let Some(PdfDestination::Page { page, fit }) = destination.clone() {
        *destination = map(page).map(|page| PdfDestination::Page { page, fit });
    }
}

fn retarget_outline(items: &mut [PdfOutlineItem], map: &dyn Fn(u32) -> Option<u32>) {
    for item in items {
        retarget_destination(&mut item.destination, map);
        retarget_outline(&mut item.children, map);
    }
}

/// 🧭 Rewrites page links, outline entries and named destinations that point at pages `map` renumbers.
pub fn retarget_documents(snapshot: &PdfSnapshot, map: &dyn Fn(u32) -> Option<u32>) -> Vec<PdfMutation> {
    let mut mutations = retarget_links(snapshot, map);
    let mut outlines = snapshot.outlines.clone();
    retarget_outline(&mut outlines, map);
    if outlines != snapshot.outlines {
        mutations.push(PdfMutation::SetOutlines(SetOutlines { outlines }));
    }
    for PdfNamedDestination { name, destination } in &snapshot.named_destinations {
        let mut moved = Some(destination.clone());
        retarget_destination(&mut moved, map);
        match moved {
            Some(next) if &next == destination => {}
            Some(next) => mutations.push(PdfMutation::SetNamedDestination(SetNamedDestination { destination: PdfNamedDestination { name: name.clone(), destination: next }, index: None })),
            None => mutations.push(PdfMutation::RemoveNamedDestination(RemoveNamedDestination { name: name.clone() })),
        }
    }
    mutations
}

/// 🔗 Rewrites the page links that point at pages `map` renumbers; the page itself must already hold its new index.
pub fn retarget_links(snapshot: &PdfSnapshot, map: &dyn Fn(u32) -> Option<u32>) -> Vec<PdfMutation> {
    let mut mutations = Vec::new();
    for (old, page) in snapshot.pages.iter().enumerate() {
        let Some(new) = map(old as u32) else { continue };
        for (at, annotation) in page.annotations.iter().enumerate() {
            let PdfAnnotationKind::Link { destination: Some(PdfDestination::Page { page: target, fit }), .. } = &annotation.kind else { continue };
            let mapped = map(*target);
            if mapped == Some(*target) {
                continue;
            }
            let mut next = annotation.clone();
            if let PdfAnnotationKind::Link { destination, .. } = &mut next.kind {
                *destination = mapped.map(|page| PdfDestination::Page { page, fit: fit.clone() });
            }
            mutations.push(PdfMutation::SetAnnotation(SetAnnotation { index: new as usize, at, annotation: next }));
        }
    }
    mutations
}

/// 🔢 Renumbers links and form widgets after a page insert, move or remove, so `map` sends each old page index to its new one or to `None` when the page is gone.
pub fn retarget_pages(snapshot: &PdfSnapshot, map: &dyn Fn(u32) -> Option<u32>) -> Vec<PdfMutation> {
    let mut mutations = retarget_documents(snapshot, map);
    if let Some(form) = &snapshot.acro_form {
        let mut next = form.clone();
        retarget_fields(&mut next.fields, map);
        if &next != form {
            mutations.push(PdfMutation::SetAcroForm(SetAcroForm { form: Some(next) }));
        }
    }
    mutations
}

pub fn insert_page_map(index: usize, count: usize) -> impl Fn(u32) -> Option<u32> {
    move |page| Some(if page as usize >= index { page + count as u32 } else { page })
}

pub fn remove_page_map(index: usize) -> impl Fn(u32) -> Option<u32> {
    move |page| match (page as usize).cmp(&index) {
        std::cmp::Ordering::Less => Some(page),
        std::cmp::Ordering::Equal => None,
        std::cmp::Ordering::Greater => Some(page - 1),
    }
}

pub fn move_page_map(from: usize, to: usize, count: usize) -> impl Fn(u32) -> Option<u32> {
    let to = to.min(count.saturating_sub(1));
    move |page| {
        let page = page as usize;
        Some(match () {
            _ if page == from => to,
            _ if from < to && page > from && page <= to => page - 1,
            _ if from > to && page >= to && page < from => page + 1,
            _ => page,
        } as u32)
    }
}

fn field_widgets(field: &mut PdfFormField, map: &dyn Fn(u32) -> Option<u32>) -> bool {
    field.widgets = field.widgets.iter().filter_map(|[page, annotation]| map(*page).map(|page| [page, *annotation])).collect();
    field.children.retain_mut(|child| field_widgets(child, map));
    !field.widgets.is_empty() || !field.children.is_empty()
}

/// 📚 Plans the mutations that bring the picked `source` pages into `snapshot`, with every used resource renamed under a fresh prefix and form fields rewired.
pub fn import_pages(snapshot: &PdfSnapshot, source: &PdfSnapshot, picks: &[usize], target: ImportTarget, prefix: Option<&str>) -> Result<Vec<PdfMutation>, Fault> {
    import_pages_controlled(snapshot, source, picks, target, prefix, &mut deadline_control(OPERATION_DEADLINE))
}

/// 📚 Like [`import_pages`], reporting [`Progress`] and stopping with a fault as soon as `control` returns false; nothing is applied then.
pub fn import_pages_controlled(snapshot: &PdfSnapshot, source: &PdfSnapshot, picks: &[usize], target: ImportTarget, prefix: Option<&str>, control: Control<'_>) -> Result<Vec<PdfMutation>, Fault> {
    step(control, "scan", 0, picks.len())?;
    let mut plan = ImportPlan::new(snapshot, std::sync::Arc::new(source.clone()), picks, target, prefix)?;
    while !plan.resources_done() {
        step(control, "resources", plan.done(), plan.total())?;
        plan.advance(1);
    }
    step(control, "pages", plan.done(), plan.total())?;
    Ok(plan.finish(snapshot))
}

enum Task {
    Font(usize),
    Image(usize),
    Form(usize),
    State(usize),
    Shading(usize),
    Pattern(usize),
    Space(usize),
    Props(usize),
}

/// 📚 A resumable page import: one resource per [`ImportPlan::advance`] unit, so a retained job can yield, report progress and stop between units.
pub struct ImportPlan {
    source: std::sync::Arc<PdfSnapshot>,
    picks: Vec<usize>,
    start: usize,
    replacing: bool,
    prefix: String,
    used: HashSet<(Kind, String)>,
    pages: Vec<PdfPage>,
    positions: Vec<Option<u32>>,
    tasks: Vec<Task>,
    cursor: usize,
    mutations: Vec<PdfMutation>,
}

impl ImportPlan {
    pub fn new(snapshot: &PdfSnapshot, source: std::sync::Arc<PdfSnapshot>, picks: &[usize], target: ImportTarget, prefix: Option<&str>) -> Result<Self, Fault> {
        if picks.iter().any(|pick| *pick >= source.pages.len()) {
            return Err(fault("the picked pdf does not have that page"));
        }
        let prefix = prefix.map_or_else(|| unused_prefix(snapshot), str::to_string);
        let (start, replacing) = match target {
            ImportTarget::Insert(at) => (at.min(snapshot.pages.len()), false),
            ImportTarget::Replace(page) if page < snapshot.pages.len() && picks.len() == 1 => (page, true),
            ImportTarget::Replace(_) => return Err(fault("replace needs one existing page and one picked page")),
        };
        let mut pages: Vec<PdfPage> = picks.iter().map(|pick| source.pages[*pick].clone()).collect();
        let used = used_resources(&source, &pages);
        let positions: Vec<Option<u32>> = (0..source.pages.len()).map(|index| picks.iter().position(|pick| *pick == index).map(|position| (start + position) as u32)).collect();
        {
            let mut rename = |kind: Kind, name: &mut String| {
                if used.contains(&(kind, name.clone())) {
                    *name = format!("{prefix}{name}");
                }
            };
            for page in &mut pages {
                visit_page(page, &mut rename);
                page_destination(page, &positions);
            }
        }
        let named = |kind: Kind, name: &str| used.contains(&(kind, name.to_string()));
        let mut tasks = Vec::new();
        tasks.extend(source.fonts.iter().enumerate().filter(|(_, item)| named(Kind::Font, &item.id)).map(|(index, _)| Task::Font(index)));
        tasks.extend(source.images.iter().enumerate().filter(|(_, item)| named(Kind::XObject, &item.id)).map(|(index, _)| Task::Image(index)));
        tasks.extend(source.forms.iter().enumerate().filter(|(_, item)| named(Kind::XObject, &item.id)).map(|(index, _)| Task::Form(index)));
        tasks.extend(source.ext_g_states.iter().enumerate().filter(|(_, item)| named(Kind::GState, &item.id)).map(|(index, _)| Task::State(index)));
        tasks.extend(source.shadings.iter().enumerate().filter(|(_, item)| named(Kind::Shading, &item.id)).map(|(index, _)| Task::Shading(index)));
        tasks.extend(source.patterns.iter().enumerate().filter(|(_, item)| named(Kind::Pattern, &item.id)).map(|(index, _)| Task::Pattern(index)));
        tasks.extend(source.color_spaces.iter().enumerate().filter(|(_, item)| named(Kind::Space, &item.name)).map(|(index, _)| Task::Space(index)));
        tasks.extend(source.properties.iter().enumerate().filter(|(_, item)| named(Kind::Props, &item.name)).map(|(index, _)| Task::Props(index)));
        Ok(Self { source, picks: picks.to_vec(), start, replacing, prefix, used, pages, positions, tasks, cursor: 0, mutations: Vec::new() })
    }

    pub fn total(&self) -> usize {
        self.tasks.len() + self.pages.len()
    }

    pub fn done(&self) -> usize {
        self.cursor
    }

    pub fn resources_done(&self) -> bool {
        self.cursor >= self.tasks.len()
    }

    /// ▶️ Copies up to `units` resources; each unit renames one resource and the references it holds.
    pub fn advance(&mut self, units: usize) {
        let Self { source, used, prefix, tasks, cursor, mutations, .. } = self;
        let mut rename = |kind: Kind, name: &mut String| {
            if used.contains(&(kind, name.clone())) {
                *name = format!("{prefix}{name}");
            }
        };
        let renamed = |id: &str| format!("{prefix}{id}");
        for _ in 0..units {
            let Some(task) = tasks.get(*cursor) else { return };
            *cursor += 1;
            mutations.push(match task {
                Task::Font(index) => {
                    let mut font = source.fonts[*index].clone();
                    visit_font(&mut font, &mut rename);
                    font.id = renamed(&font.id);
                    PdfMutation::SetFont(SetFont { font, index: None })
                }
                Task::Image(index) => {
                    let mut image = source.images[*index].clone();
                    image.soft_mask = image.soft_mask.map(|mask| if used.contains(&(Kind::XObject, mask.clone())) { renamed(&mask) } else { mask });
                    image.id = renamed(&image.id);
                    PdfMutation::SetImage(SetImage { image, index: None })
                }
                Task::Form(index) => {
                    let mut form = source.forms[*index].clone();
                    visit_ops(&mut form.content, &mut rename);
                    form.id = renamed(&form.id);
                    PdfMutation::SetForm(SetForm { form, index: None })
                }
                Task::State(index) => {
                    let mut state = source.ext_g_states[*index].clone();
                    visit_state(&mut state, &mut rename);
                    state.id = renamed(&state.id);
                    PdfMutation::SetExtGState(SetExtGState { state, index: None })
                }
                Task::Shading(index) => {
                    let mut shading = source.shadings[*index].clone();
                    shading.id = renamed(&shading.id);
                    PdfMutation::SetShading(SetShading { shading, index: None })
                }
                Task::Pattern(index) => {
                    let mut pattern = source.patterns[*index].clone();
                    visit_pattern(&mut pattern, &mut rename);
                    pattern.id = renamed(&pattern.id);
                    PdfMutation::SetPattern(SetPattern { pattern, index: None })
                }
                Task::Space(index) => {
                    let mut color_space = source.color_spaces[*index].clone();
                    color_space.name = renamed(&color_space.name);
                    PdfMutation::SetColorSpace(SetColorSpace { color_space, index: None })
                }
                Task::Props(index) => {
                    let mut properties = source.properties[*index].clone();
                    properties.name = renamed(&properties.name);
                    PdfMutation::SetProperties(SetProperties { properties, index: None })
                }
            });
        }
    }

    /// 🏁 Adds the pages, renumbered links and merged form fields and returns every planned mutation.
    pub fn finish(mut self, snapshot: &PdfSnapshot) -> Vec<PdfMutation> {
        let incoming = merged_form(snapshot, &self.source, &self.positions, self.start, self.picks.len(), self.replacing, &mut self.pages);
        let (start, replacing, count) = (self.start, self.replacing, self.picks.len());
        let mut mutations = self.mutations;
        for (offset, page) in self.pages.into_iter().enumerate() {
            mutations.push(if replacing { PdfMutation::ReplacePage(ReplacePage { index: start, page }) } else { PdfMutation::InsertPage(InsertPage { index: start + offset, page }) });
        }
        if !replacing {
            mutations.extend(retarget_documents(snapshot, &insert_page_map(start, count)));
        }
        if let Some(form) = incoming {
            mutations.push(PdfMutation::SetAcroForm(SetAcroForm { form: Some(form) }));
        }
        mutations
    }
}

/// 📥 A resumable merge or replace from a picked PDF: read the base64, decode through the paged natural decoder, then plan resource by resource.
pub struct ImportJob {
    payload: String,
    selection: String,
    page: usize,
    replace: bool,
    bytes: Option<Vec<u8>>,
    decoder: Option<Box<dyn semio_plugin::NaturalFileDecodeCursor<PdfSnapshot>>>,
    plan: Option<ImportPlan>,
}

fn natural_contract() -> semio_plugin::ToolExecutionContract {
    semio_plugin::ToolExecutionContract::bounded_first_step(MAX_SOURCE_BYTES as usize, 4_096, 1, MAX_SOURCE_BYTES as usize, 7_500)
}

impl ImportJob {
    pub fn new(page: usize, payload: &str, selection: &str, replace: bool) -> Self {
        Self { payload: payload.to_string(), selection: selection.to_string(), page, replace, bytes: None, decoder: None, plan: None }
    }

    pub fn stage(&self) -> &'static str {
        match (&self.plan, &self.decoder, &self.bytes) {
            (Some(_), _, _) => "resources",
            (None, Some(_), _) => "decode",
            (None, None, Some(_)) => "open",
            _ => "read",
        }
    }

    fn plan_from(&mut self, snapshot: &PdfSnapshot, source: PdfSnapshot) -> Result<(), Fault> {
        let picks = if self.selection.is_empty() { if self.replace { vec![0] } else { (0..source.pages.len()).collect() } } else { parse_page_selection(&self.selection, source.pages.len())? };
        let target = if self.replace { ImportTarget::Replace(self.page) } else { ImportTarget::Insert(self.page) };
        self.plan = Some(ImportPlan::new(snapshot, std::sync::Arc::new(source), &picks, target, None)?);
        Ok(())
    }

    /// ▶️ One bounded unit of work; `Some` carries the finished mutations. With a step context the source is decoded by the paged natural decoder, one governed turn at a time.
    pub fn advance(&mut self, snapshot: &PdfSnapshot, units: usize, context: Option<&mut semio_plugin::__job::StepContext<'_>>) -> Result<Option<Vec<PdfMutation>>, Fault> {
        if self.plan.is_none() {
            if let Some(decoder) = self.decoder.as_mut() {
                let context = context.ok_or_else(|| fault("the paged decoder needs a step context"))?;
                match decoder.advance(context).map_err(|error| fault(format!("the picked pdf is not a valid pdf: {error:?}")))? {
                    semio_plugin::NaturalFileDecodeStep::Complete { snapshot: source, .. } => {
                        self.decoder = None;
                        self.plan_from(snapshot, source)?;
                    }
                    semio_plugin::NaturalFileDecodeStep::Yield | semio_plugin::NaturalFileDecodeStep::Progress(_) => {}
                }
                return Ok(None);
            }
            if let Some(bytes) = self.bytes.take() {
                if context.is_none() {
                    let source = decode_pdf(&bytes).map_err(|error| fault(format!("the picked pdf is not a valid pdf: {error}")))?;
                    self.plan_from(snapshot, source)?;
                    return Ok(None);
                }
                match semio_s_artifact_stdio_pdf_io_export::standards::v1_7::subsets::base::io::import::build_pdf_natural_decode(bytes, natural_contract()).map_err(|error| fault(format!("the picked pdf could not be opened: {error:?}")))? {
                    semio_plugin::NaturalFileDecodeOwner::Controlled(cursor) => self.decoder = Some(cursor),
                    semio_plugin::NaturalFileDecodeOwner::Synchronous(bytes) => {
                        let source = decode_pdf(&bytes).map_err(|error| fault(format!("the picked pdf is not a valid pdf: {error}")))?;
                        self.plan_from(snapshot, source)?;
                    }
                }
                return Ok(None);
            }
            let encoded = self.payload.split_once("base64,").map_or(self.payload.as_str(), |(_, rest)| rest).trim().to_string();
            let mut progress = never_cancelled;
            self.bytes = Some(base64_standard_decode_controlled(encoded.as_bytes(), &mut Base64Control { maximum_output_bytes: MAX_SOURCE_BYTES as usize, progress: &mut progress }).map_err(|error| fault(format!("the picked pdf is not readable: {error}")))?);
            self.payload = String::new();
            return Ok(None);
        }
        let Some(plan) = self.plan.as_mut() else { return Ok(None) };
        if !plan.resources_done() {
            plan.advance(units);
            return Ok(None);
        }
        Ok(self.plan.take().map(|plan| plan.finish(snapshot)))
    }
}

/// ✂️ A resumable extraction of one page selection to PDF bytes.
pub struct ExtractJob {
    source: std::sync::Arc<PdfSnapshot>,
    picks: Vec<usize>,
    out: PdfSnapshot,
    plan: Option<ImportPlan>,
    pending: std::collections::VecDeque<PdfMutation>,
    assembling: bool,
}

impl ExtractJob {
    pub fn new(source: std::sync::Arc<PdfSnapshot>, selection: &str) -> Result<Self, Fault> {
        let picks = parse_page_selection(selection, source.pages.len())?;
        let mut out = PdfSnapshot::default();
        out.info = source.info.clone();
        out.language = source.language.clone();
        Ok(Self { source, picks, out, plan: None, pending: Default::default(), assembling: false })
    }

    pub fn stage(&self) -> &'static str {
        if self.plan.is_none() && !self.assembling { "scan" } else if self.assembling { "assemble" } else { "resources" }
    }

    pub fn advance(&mut self, units: usize) -> Result<Option<Vec<u8>>, Fault> {
        if self.assembling {
            for _ in 0..units {
                let Some(mutation) = self.pending.pop_front() else {
                    if self.out.pages.len() != self.picks.len() {
                        return Err(fault("the extracted pages could not be assembled"));
                    }
                    return encode_pdf(&self.out).map(Some).map_err(|error| fault(format!("the extracted pdf could not be written: {error}")));
                };
                let _ = apply_pdf_mutation(&mut self.out, &mutation);
            }
            return Ok(None);
        }
        let Some(plan) = self.plan.as_mut() else {
            self.plan = Some(ImportPlan::new(&self.out, self.source.clone(), &self.picks, ImportTarget::Insert(0), Some(""))?);
            return Ok(None);
        };
        if !plan.resources_done() {
            plan.advance(units);
            return Ok(None);
        }
        self.pending = self.plan.take().map(|plan| plan.finish(&self.out)).unwrap_or_default().into();
        self.assembling = true;
        Ok(None)
    }
}

fn merged_form(snapshot: &PdfSnapshot, source: &PdfSnapshot, positions: &[Option<u32>], start: usize, added: usize, replacing: bool, pages: &mut [PdfPage]) -> Option<PdfAcroForm> {
    let mut incoming: Vec<PdfFormField> = source.acro_form.as_ref().map(|form| form.fields.clone()).unwrap_or_default();
    let map = |page: u32| positions.get(page as usize).copied().flatten();
    incoming.retain_mut(|field| field_widgets(field, &map));
    if incoming.is_empty() && snapshot.acro_form.is_none() {
        return None;
    }
    let mut form = snapshot.acro_form.clone().unwrap_or_else(|| PdfAcroForm { need_appearances: source.acro_form.as_ref().is_some_and(|form| form.need_appearances), ..Default::default() });
    let shift = |page: u32| if replacing { (page as usize != start).then_some(page) } else if page as usize >= start { Some(page + added as u32) } else { Some(page) };
    retarget_fields(&mut form.fields, &shift);
    for mut field in incoming {
        let old = field.name.clone();
        let taken = |name: &str| form.fields.iter().any(|existing| existing.name == name);
        if taken(&old) {
            field.name = (2..).map(|number| format!("{old}_{number}")).find(|name| !taken(name)).unwrap_or(old.clone());
            for page in pages.iter_mut() {
                for annotation in &mut page.annotations {
                    if let PdfAnnotationKind::Widget { field: Some(name), .. } = &mut annotation.kind {
                        if *name == old || name.starts_with(&format!("{old}.")) {
                            *name = format!("{}{}", field.name, &name[old.len()..]);
                        }
                    }
                }
            }
        }
        form.fields.push(field);
    }
    Some(form)
}

/// ✂️ Builds the standalone PDF bytes holding the picked pages of `snapshot`.
pub fn extract_pdf(snapshot: &PdfSnapshot, picks: &[usize]) -> Result<Vec<u8>, Fault> {
    extract_pdf_controlled(snapshot, picks, &mut deadline_control(OPERATION_DEADLINE))
}

/// ✂️ Like [`extract_pdf`], reporting [`Progress`] and stopping as soon as `control` returns false.
pub fn extract_pdf_controlled(snapshot: &PdfSnapshot, picks: &[usize], control: Control<'_>) -> Result<Vec<u8>, Fault> {
    let selection = picks.iter().map(|pick| (pick + 1).to_string()).collect::<Vec<_>>().join(",");
    let mut job = ExtractJob::new(std::sync::Arc::new(snapshot.clone()), &selection)?;
    loop {
        step(control, job.stage(), 0, picks.len())?;
        if let Some(bytes) = job.advance(1)? {
            return Ok(bytes);
        }
    }
}

pub fn download_effect(name: &str, bytes: &[u8]) -> Result<semio_plugin::Effect, Fault> {
    let mut progress = never_cancelled;
    let data = base64_standard_encode_controlled(bytes, &mut Base64Control { maximum_output_bytes: (MAX_SOURCE_BYTES as usize).div_ceil(3) * 4 * 4, progress: &mut progress }).map_err(|error| fault(format!("the extracted pdf is too large: {error}")))?;
    Ok(semio_plugin::Effect::DownloadMediaExport { filename: name.to_string(), mime_type: "application/pdf".into(), data, encoding: Some("base64".into()) })
}

/// 🏷️ The file name of the `number`th (0-based) extracted selection.
pub fn extract_file_name(number: usize, group: &str) -> String {
    format!("pages-{}-{}.pdf", number + 1, group.replace(['-', ','], "_"))
}

/// 🧾 The page-edit payload fields the retained authoring work reads.
pub struct Fields {
    pub page: usize,
    pub text: String,
    pub extra: String,
    pub width: f64,
    pub height: f64,
}

pub fn fields(payload: &str) -> Result<Fields, Fault> {
    let value: serde_json::Value = serde_json::from_str(payload).map_err(|error| fault(error.to_string()))?;
    let text = |key: &str| value.get(key).and_then(serde_json::Value::as_str).unwrap_or("").to_string();
    let number = |key: &str| value.get(key).and_then(serde_json::Value::as_f64).unwrap_or(0.0);
    Ok(Fields { page: number("page") as usize, text: text("text"), extra: text("extra"), width: number("width"), height: number("height") })
}

/// ✂️ Extracts each `;`-separated page selection into its own PDF download.
pub fn extract_effects(snapshot: &PdfSnapshot, selections: &str) -> Result<Vec<semio_plugin::Effect>, Fault> {
    let groups: Vec<&str> = selections.split(';').map(str::trim).filter(|group| !group.is_empty()).collect();
    if groups.is_empty() {
        return Err(fault("extract needs a page selection"));
    }
    groups.iter().enumerate().map(|(number, group)| download_effect(&extract_file_name(number, group), &extract_pdf(snapshot, &parse_page_selection(group, snapshot.pages.len())?)?)).collect()
}

/// 📥 Asks the host for a PDF and re-dispatches `action` with its data URL and `page`.
pub fn pick_source_effect(action: &str, page: usize, selection: &str) -> semio_plugin::Effect {
    semio_plugin::Effect::RequestFileOpen {
        req: semio_plugin::RequestId(0x5044_4650),
        accept: "application/pdf,.pdf".into(),
        read_as: Some("dataUrl".into()),
        import_action: action.into(),
        args: Some(semio_framework_value::DslValue::from(json!({ "page": page, "extra": selection }))),
        multiple: false,
        max_file_bytes: Some(MAX_SOURCE_BYTES),
    }
}

/// 📚 Merges picked pages of the PDF in `payload` before page `page`.
pub fn merge_document(snapshot: &PdfSnapshot, page: usize, payload: &str, selection: &str) -> Result<Vec<PdfMutation>, Fault> {
    let source = decode_source(payload)?;
    let picks = if selection.is_empty() { (0..source.pages.len()).collect() } else { parse_page_selection(selection, source.pages.len())? };
    import_pages(snapshot, &source, &picks, ImportTarget::Insert(page), None)
}

/// 🪄 Replaces page `page` with a blank page, or with one page of the PDF in `payload`.
pub fn replace_page_with(snapshot: &PdfSnapshot, page: usize, payload: &str, selection: &str, width: f64, height: f64) -> Result<Vec<PdfMutation>, Fault> {
    let existing = snapshot.pages.get(page).ok_or_else(|| fault(format!("pdf page {page} does not exist")))?;
    if payload.is_empty() {
        let (width, height) = (if width > 0.0 { width } else { existing.width() }, if height > 0.0 { height } else { existing.height() });
        return Ok(vec![PdfMutation::ReplacePage(ReplacePage { index: page, page: PdfPage::new(width, height) })]);
    }
    let source = decode_source(payload)?;
    let picks = if selection.is_empty() { vec![0] } else { parse_page_selection(selection, source.pages.len())? };
    import_pages(snapshot, &source, &picks, ImportTarget::Replace(page), None)
}
/// 🗝️ The document key-value families the generic entry form edits; see [`set_entry`].
pub const ENTRY_FAMILIES: [&str; 4] = ["catalog", "trailer", "info", "viewer"];

fn upsert_name(entries: &mut Vec<PdfDictEntry>, key: &str, value: &str) {
    entries.retain(|entry| entry.key != key);
    if !value.is_empty() {
        entries.push(PdfDictEntry::new(key, PdfObject::Name(value.to_string())));
    }
}

/// 🗝️ Sets (or, with an empty value, removes) one name-valued entry of a document dictionary family; with an empty `key` the `value` reads `key=value`.
pub fn set_entry(snapshot: &PdfSnapshot, family: &str, key: &str, value: &str) -> Result<Vec<PdfMutation>, Fault> {
    let (key, value) = if key.is_empty() {
        let (key, value) = value.split_once('=').ok_or_else(|| fault("an entry needs key=value"))?;
        (key.trim().to_string(), value.trim().to_string())
    } else {
        (key.to_string(), value.to_string())
    };
    if key.is_empty() {
        return Err(fault("an entry needs a key"));
    }
    Ok(vec![match family {
        "catalog" if value.is_empty() => PdfMutation::RemoveCatalogEntry(RemoveCatalogEntry { key }),
        "catalog" => PdfMutation::SetCatalogEntry(SetCatalogEntry { key, value: PdfObject::Name(value), index: None }),
        "trailer" if value.is_empty() => PdfMutation::RemoveTrailerEntry(RemoveTrailerEntry { key }),
        "trailer" => PdfMutation::SetTrailerEntry(SetTrailerEntry { key, value: PdfObject::Name(value), index: None }),
        "info" => {
            let mut info = snapshot.info.clone();
            upsert_name(&mut info.extra, &key, &value);
            PdfMutation::SetInfo(SetInfo { info })
        }
        "viewer" => {
            let mut preferences = snapshot.viewer_preferences.clone().unwrap_or_default();
            upsert_name(&mut preferences.extra, &key, &value);
            PdfMutation::SetViewerPreferences(SetViewerPreferences { preferences: (preferences != PdfViewerPreferences::default()).then_some(preferences) })
        }
        other => return Err(fault(format!("unknown entry family '{other}'"))),
    }])
}

/// 🗝️ The current entries of one family as `(key, editable name value or read-only summary, editable)`.
pub fn entry_rows(snapshot: &PdfSnapshot, family: &str) -> Vec<(String, String, bool)> {
    let entries: &[PdfDictEntry] = match family {
        "catalog" => &snapshot.catalog_extra,
        "trailer" => &snapshot.trailer,
        "info" => &snapshot.info.extra,
        "viewer" => snapshot.viewer_preferences.as_ref().map_or(&[], |preferences| &preferences.extra),
        _ => &[],
    };
    entries.iter().map(|entry| match &entry.value {
        PdfObject::Name(name) => (entry.key.clone(), name.clone(), true),
        other => (entry.key.clone(), format!("{other:?}").chars().take(60).collect(), false),
    }).collect()
}
//#endregion ✍️Authoring

//#region ✍️Work
use semio_plugin::__job::{OriginalTurnFailure, StepContext};
use semio_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep};
use semio_plugin::ArtifactApp;

pub const TOOLS: [&str; 3] = ["merge-document", "replace-page", "extract-pages"];
const UNITS: usize = 8;

/// 🧵 Retained work for merge, replace-from-PDF and extract: one bounded unit per scheduler step with progress, fuel yielding and cancellation between units.
enum Job {
    Import(ImportJob),
    Extract { groups: std::collections::VecDeque<(usize, String)>, current: Option<ExtractJob>, effects: Vec<semio_plugin::Effect> },
}

pub struct PageAuthoringWork<A: ArtifactApp> {
    tool: &'static str,
    payload: fn(&A::Command) -> Option<&str>,
    maximum_items: usize,
    job: Option<Job>,
}

impl<A: ArtifactApp> PageAuthoringWork<A> {
    /// 🧵 `payload` reads the page-edit payload out of the editor's command; `maximum_items` is the editor's declared retained extent.
    pub fn new(tool: &'static str, payload: fn(&A::Command) -> Option<&str>, maximum_items: usize) -> Self {
        Self { tool, payload, maximum_items, job: None }
    }
}

fn refused(message: &str) -> OriginalTurnFailure {
    Fault::from(message).into()
}

impl<A: ArtifactApp<Snapshot = PdfSnapshot, Mutation = PdfMutation>> ArtifactCommandWork<A> for PageAuthoringWork<A> {
    fn tool_id(&self) -> &'static str {
        self.tool
    }

    fn extent(&self, command: &A::Command, _snapshot: &PdfSnapshot, _interaction: &semio_plugin::InteractionState, _context: Option<&semio_plugin::app::ArtifactOwnedToolJobContext<A>>) -> Option<usize> {
        (self.payload)(command).map(|_| self.maximum_items)
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, A>, cx: &mut StepContext<'_>) -> Result<ArtifactCommandWorkStep<A>, OriginalTurnFailure> {
        if cx.is_cancelled() {
            return Err(refused("pdf-authoring-cancelled"));
        }
        let Some(payload) = (self.payload)(input.command) else { return Err(refused("pdf-authoring-command-mismatch")) };
        if cx.fuel_remaining() < UNITS as u64 {
            return Ok(ArtifactCommandWorkStep::Pending { stage: "authoring-yield", preview: b"" });
        }
        cx.consume_fuel(UNITS as u64);
        let snapshot = input.snapshot;
        let outcome: Result<ArtifactCommandWorkStep<A>, semio_plugin::Fault> = (|| {
            let Fields { page, text, extra, width, height } = fields(payload)?;
            if self.job.is_none() {
                self.job = Some(match self.tool {
                    "extract-pages" => Job::Extract { groups: extra.split(';').map(str::trim).filter(|group| !group.is_empty()).map(str::to_string).enumerate().collect(), current: None, effects: Vec::new() },
                    "replace-page" if text.is_empty() => {
                        let mutations = replace_page_with(snapshot, page, "", "", width, height)?;
                        return Ok(ArtifactCommandWorkStep::Complete(semio_plugin::Emit { artifact_mutations: mutations, ..Default::default() }));
                    }
                    other => Job::Import(ImportJob::new(page, &text, &extra, other == "replace-page")),
                });
            }
            match self.job.as_mut().expect("authoring job") {
                Job::Import(job) => match job.advance(snapshot, UNITS, Some(&mut *cx))? {
                    Some(mutations) => Ok(ArtifactCommandWorkStep::Complete(semio_plugin::Emit { artifact_mutations: mutations, ..Default::default() })),
                    None => Ok(ArtifactCommandWorkStep::Progress { stage: job.stage(), preview: b"" }),
                },
                Job::Extract { groups, current, effects } => {
                    if current.is_none() {
                        let Some((_, group)) = groups.front() else {
                            let effects = std::mem::take(effects);
                            if effects.is_empty() {
                                return Err(Fault::from("extract needs a page selection"));
                            }
                            return Ok(ArtifactCommandWorkStep::Complete(semio_plugin::Emit { effects, ..Default::default() }));
                        };
                        let source = input.snapshot_owner.cloned().unwrap_or_else(|| std::sync::Arc::new(snapshot.clone()));
                        *current = Some(ExtractJob::new(source, group)?);
                    }
                    let job = current.as_mut().expect("extract job");
                    match job.advance(UNITS)? {
                        Some(bytes) => {
                            let (number, group) = groups.pop_front().expect("extract group");
                            effects.push(download_effect(&extract_file_name(number, &group), &bytes)?);
                            *current = None;
                            Ok(ArtifactCommandWorkStep::Progress { stage: "encode", preview: b"" })
                        }
                        None => Ok(ArtifactCommandWorkStep::Progress { stage: job.stage(), preview: b"" }),
                    }
                }
            }
        })();
        outcome.map_err(OriginalTurnFailure::from)
    }

}
//#endregion ✍️Work
