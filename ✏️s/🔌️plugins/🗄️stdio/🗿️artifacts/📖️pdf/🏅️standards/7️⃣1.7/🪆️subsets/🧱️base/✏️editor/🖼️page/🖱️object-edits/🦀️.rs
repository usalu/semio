//! 🖱️ PDF page editor object edits: move, resize, insert and restyle of page objects, annotations and fonts.

use super::*;

//#region 🖱️ObjectEdits
pub(super) fn payload_edit(action: &str, page: u32, object: String, text: String, extra: String, x: f64, y: f64, width: f64, height: f64, red: f64, green: f64, blue: f64) -> PdfPageEdit {
    PdfPageEdit { action: action.to_string(), payload: json!({ "page": page, "object": object, "text": text, "extra": extra, "x": x, "y": y, "width": width, "height": height, "red": red, "green": green, "blue": blue }).to_string() }
}

pub(super) fn edit_object(snapshot: &PdfSnapshot, action: &str, object_id: &str, x: f64, y: f64, width: f64, height: f64, red: f64, green: f64, blue: f64, text: &str) -> Result<Vec<PdfMutation>, Fault> {
    let object = objects(snapshot).into_iter().find(|item| item.id == object_id).ok_or_else(|| fault(format!("pdf object '{object_id}' is not on the page")))?;
    if object.kind == ObjectKind::Annotation {
        return edit_annotation(snapshot, object.page, object_id, text, x, y, width, height);
    }
    if object.kind == ObjectKind::Shading && (action == "move" || action == "resize") {
        return edit_shading(snapshot, &object, action, x, y, width, height);
    }
    let page = snapshot.pages.get(object.page).ok_or_else(|| fault("pdf page is gone"))?;
    let mut content = page.content.clone();
    match action {
        "set-text" => replace_shown_text(&mut content, object.end, text),
        "delete" => {
            let end = object.end.min(content.len().saturating_sub(1));
            if object.start <= end && end < content.len() {
                content.drain(object.start..=end);
            }
        }
        "move" => translate_object(&mut content, &object, x - object.x, y - object.y),
        "resize" => resize_object(&mut content, &object, x, y, width, height),
        "set-fill" => set_fill(&mut content, &object, red, green, blue),
        "set-stroke" => set_stroke(&mut content, &object, red, green, blue, width),
        other => return Err(fault(format!("unknown pdf page action '{other}'"))),
    }
    Ok(vec![PdfMutation::SetPageContent(SetPageContent { index: object.page, content })])
}

pub(super) fn insert_object(snapshot: &PdfSnapshot, action: &str, page_index: usize, x: f64, y: f64, width: f64, height: f64, red: f64, green: f64, blue: f64, text: &str) -> Result<Vec<PdfMutation>, Fault> {
    let page = snapshot.pages.get(page_index).ok_or_else(|| fault(format!("pdf page {page_index} does not exist")))?;
    let mut content = page.content.clone();
    let mut extra = Vec::new();
    match action {
        "insert-text" => {
            let (font_id, font_mutation) = ensure_font(snapshot);
            if let Some(mutation) = font_mutation {
                extra.push(mutation);
            }
            let size = if height > 0.0 { height } else { 12.0 };
            content.extend([
                PdfOp::BeginText,
                PdfOp::SetFont { name: font_id, size },
                PdfOp::SetFillRgb { r: red, g: green, b: blue },
                PdfOp::SetTextMatrix { matrix: [1.0, 0.0, 0.0, 1.0, x, y] },
                PdfOp::ShowText { text: PdfTextString::text(text) },
                PdfOp::EndText,
            ]);
        }
        "insert-rectangle" => content.extend([PdfOp::SetFillRgb { r: red, g: green, b: blue }, PdfOp::Rectangle { x, y, width, height }, PdfOp::Fill]),
        "insert-line" => content.extend([
            PdfOp::SetStrokeRgb { r: red, g: green, b: blue },
            PdfOp::SetLineWidth { width: 1.0 },
            PdfOp::MoveTo { x, y },
            PdfOp::LineTo { x: x + width, y: y + height },
            PdfOp::Stroke,
        ]),
        "insert-image" => {
            let id = format!("Im{}", snapshot.images.len() + 1);
            let image = PdfImage::rgb8(&id, 8, 8, vec![180; 8 * 8 * 3]);
            extra.push(PdfMutation::SetImage(SetImage { image, index: None }));
            content.extend([PdfOp::Save, PdfOp::Transform { matrix: [width.max(1.0), 0.0, 0.0, height.max(1.0), x, y] }, PdfOp::PaintXObject { name: id }, PdfOp::Restore]);
        }
        other => return Err(fault(format!("unknown pdf page action '{other}'"))),
    }
    extra.push(PdfMutation::SetPageContent(SetPageContent { index: page_index, content }));
    Ok(extra)
}

pub(super) fn edit_annotation(snapshot: &PdfSnapshot, page_index: usize, object_id: &str, text: &str, x: f64, y: f64, width: f64, height: f64) -> Result<Vec<PdfMutation>, Fault> {
    let index = object_id.rsplit_once(":a").and_then(|(_, raw)| raw.parse::<usize>().ok()).ok_or_else(|| fault(format!("pdf annotation '{object_id}' is not addressable")))?;
    let annotation = snapshot.pages.get(page_index).and_then(|page| page.annotations.get(index)).cloned().ok_or_else(|| fault(format!("pdf annotation '{object_id}' is gone")))?;
    let mut annotation: PdfAnnotation = annotation;
    annotation.contents = Some(text.to_string());
    if width > 0.0 && height > 0.0 {
        annotation.rect = [x, y, x + width, y + height];
    }
    Ok(vec![PdfMutation::SetAnnotation(SetAnnotation { index: page_index, at: index, annotation })])
}

pub(super) fn replace_image_samples(snapshot: &PdfSnapshot, object_id: &str, width: f64, height: f64, hex: &str) -> Result<Vec<PdfMutation>, Fault> {
    let object = objects(snapshot).into_iter().find(|item| item.id == object_id).ok_or_else(|| fault(format!("pdf object '{object_id}' is not on the page")))?;
    if object.kind != ObjectKind::Image {
        return Err(fault(format!("pdf object '{object_id}' is not an image")));
    }
    let pixel_width = width.max(1.0).round() as u32;
    let pixel_height = height.max(1.0).round() as u32;
    let values=hex.split(|character:char|character==','||character.is_whitespace()).filter(|part|!part.is_empty()).map(|part|part.parse::<u8>().map(u32::from).map_err(|_|fault("image samples must be logical values from 0 to 255"))).collect::<Result<Vec<_>,_>>()?;
    let color_space = image_color_space(pixel_width, pixel_height, values.len())?;
    if let Some(image) = snapshot.images.iter().find(|image| image.id == object.text) {
        let mut image = image.clone();
        image.width = pixel_width;
        image.height = pixel_height;
        image.body=PdfImageBody::Samples {values};
        image.bits_per_component = 8;
        image.color_space = Some(color_space);
        return Ok(vec![PdfMutation::SetImage(SetImage { image, index: None })]);
    }
    let page = snapshot.pages.get(object.page).ok_or_else(|| fault("pdf page is gone"))?;
    let mut content = page.content.clone();
    let Some(PdfOp::InlineImage { image }) = content.get_mut(object.end) else { return Err(fault(format!("pdf image '{object_id}' is not a stored image"))) };
    image.width = pixel_width;
    image.height = pixel_height;
    image.body=PdfImageBody::Samples {values};
    image.bits_per_component = 8;
    image.color_space = Some(color_space);
    Ok(vec![PdfMutation::SetPageContent(SetPageContent { index: object.page, content })])
}

pub(super) fn set_font_of_text(snapshot: &PdfSnapshot, object_id: &str, base_font: &str) -> Result<Vec<PdfMutation>, Fault> {
    if base_font.is_empty() {
        return Err(fault("a text run needs a font name"));
    }
    let object = objects(snapshot).into_iter().find(|item| item.id == object_id).ok_or_else(|| fault(format!("pdf object '{object_id}' is not on the page")))?;
    if object.kind != ObjectKind::Text {
        return Err(fault(format!("pdf object '{object_id}' is not text")));
    }
    let (font_id, font_mutation) = font_for_base(snapshot, base_font);
    let page = snapshot.pages.get(object.page).ok_or_else(|| fault("pdf page is gone"))?;
    let mut content = page.content.clone();
    content.insert(object.end, PdfOp::SetFont { name: font_id, size: object.font_size.max(1.0) });
    let mut mutations = Vec::new();
    if let Some(mutation) = font_mutation {
        mutations.push(mutation);
    }
    mutations.push(PdfMutation::SetPageContent(SetPageContent { index: object.page, content }));
    Ok(mutations)
}

fn font_for_base(snapshot: &PdfSnapshot, base_font: &str) -> (String, Option<PdfMutation>) {
    if let Some(font) = snapshot.fonts.iter().find(|font| font.base_font() == base_font) {
        return (font.id.clone(), None);
    }
    let id = format!("F{}", snapshot.fonts.len() + 1);
    (id.clone(), Some(PdfMutation::SetFont(SetFont { font: PdfFont::standard(id, base_font), index: None })))
}
//#endregion 🖱️ObjectEdits
