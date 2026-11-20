//! 📄️ PDF page editor document edits: outline, page boxes, metadata, viewer preferences, encryption, forms and font programs.

use super::*;

//#region 📄️DocumentEdits
pub(super) fn set_font_program(snapshot: &PdfSnapshot, id: &str, kind: &str, artifact_id: &str) -> Result<Vec<PdfMutation>, Fault> {
    let mut font = snapshot.fonts.iter().find(|font| font.id == id).cloned().ok_or_else(|| fault(format!("pdf font '{id}' is gone")))?;
    let reference=semio_framework_artifact_reference::ArtifactRef {artifact_id:artifact_id.into(),dialect:semio_framework_artifact_reference::ArtifactDialect {artifact_kind:"s.stdio.binary".into(),standard:"raw".into(),subset:"*".into()}};
    let program = font_program(kind, reference)?;
    font.kind = place_font_program(font.kind, program)?;
    Ok(vec![PdfMutation::SetFont(SetFont { font, index: None })])
}

fn font_program(kind:&str,reference:semio_framework_artifact_reference::ArtifactRef)->Result<PdfFontProgram,Fault>{Ok(match kind {"type1"=>PdfFontProgram::Type1 {reference},"truetype"=>PdfFontProgram::TrueType {reference},"cff"=>PdfFontProgram::Cff {reference},"cidcff"=>PdfFontProgram::CidCff {reference},"opentype"=>PdfFontProgram::OpenType {reference},other=>return Err(fault(format!("unknown font program '{other}'")))})}

fn place_font_program(kind: PdfFontKind, program: PdfFontProgram) -> Result<PdfFontKind, Fault> {
    let type1 = matches!(program, PdfFontProgram::Type1 { .. } | PdfFontProgram::Cff { .. });
    let outlined = matches!(program, PdfFontProgram::TrueType { .. } | PdfFontProgram::OpenType { .. });
    let cid = matches!(program, PdfFontProgram::CidCff { .. }) || outlined;
    match kind {
        PdfFontKind::Type1 { base_font, encoding, first_char, widths, descriptor, .. } if type1 => Ok(PdfFontKind::Type1 { base_font, encoding, first_char, widths, descriptor, program: Some(program) }),
        PdfFontKind::Type1 { base_font, encoding, first_char, widths, descriptor, .. } if outlined => Ok(PdfFontKind::TrueType { base_font, encoding, first_char, widths, descriptor, program: Some(program) }),
        PdfFontKind::TrueType { base_font, encoding, first_char, widths, descriptor, .. } if outlined => Ok(PdfFontKind::TrueType { base_font, encoding, first_char, widths, descriptor, program: Some(program) }),
        PdfFontKind::Type0 { base_font, cmap, mut descendant } if cid => {
            descendant.program = Some(program);
            Ok(PdfFontKind::Type0 { base_font, cmap, descendant })
        }
        _ => Err(fault("that font program does not fit this font")),
    }
}

pub(super) fn set_image_mask(snapshot: &PdfSnapshot, address: &str, kind: &str, value: &str) -> Result<Vec<PdfMutation>, Fault> {
    let mut image = image_by_address(snapshot, address)?;
    match kind {
        "" => {
            image.mask = None;
            image.soft_mask = None;
        }
        "stencil" if !value.is_empty() => image.mask = Some(PdfImageMask::Stencil { image: value.to_string() }),
        "soft" if !value.is_empty() => image.soft_mask = Some(value.to_string()),
        "colorKey" => {
            let ranges = value.split(',').map(|item| item.trim().parse::<u32>().map_err(|_| fault("a color key is comma-separated integers"))).collect::<Result<Vec<_>, _>>()?;
            if ranges.is_empty() {
                return Err(fault("a color key needs ranges"));
            }
            image.mask = Some(PdfImageMask::ColorKey { ranges });
        }
        "stencil" | "soft" => return Err(fault("a mask needs an image id")),
        other => return Err(fault(format!("unknown image mask '{other}'"))),
    }
    Ok(vec![PdfMutation::SetImage(SetImage { image, index: None })])
}

fn image_by_address(snapshot: &PdfSnapshot, address: &str) -> Result<PdfImage, Fault> {
    if let Some(image) = snapshot.images.iter().find(|image| image.id == address) {
        return Ok(image.clone());
    }
    let object = find_object(snapshot, address).ok_or_else(|| fault(format!("pdf image '{address}' is gone")))?;
    if object.kind != ObjectKind::Image {
        return Err(fault(format!("pdf object '{address}' is not an image")));
    }
    snapshot.images.iter().find(|image| image.id == object.text).cloned().ok_or_else(|| fault(format!("pdf image '{}' is not a stored image", object.text)))
}

pub(super) fn set_form_content(snapshot: &PdfSnapshot, id: &str, text: &str, x: f64, y: f64, width: f64, height: f64) -> Result<Vec<PdfMutation>, Fault> {
    if id.is_empty() {
        return Err(fault("a form needs an id"));
    }
    let font = snapshot.fonts.first().map(|font| font.id.clone()).unwrap_or_else(|| "F1".into());
    let content = if text.is_empty() {
        vec![PdfOp::Rectangle { x, y, width: width.max(1.0), height: height.max(1.0) }, PdfOp::Fill]
    } else {
        vec![PdfOp::BeginText, PdfOp::SetFont { name: font, size: height.max(1.0) }, PdfOp::SetTextMatrix { matrix: [1.0, 0.0, 0.0, 1.0, x, y] }, PdfOp::ShowText { text: PdfTextString::text(text) }, PdfOp::EndText]
    };
    let form = match snapshot.forms.iter().find(|form| form.id == id) {
        Some(form) => {
            let mut form = form.clone();
            form.content = content;
            form
        }
        None => PdfFormXObject::new(id, [x, y, x + width.max(1.0), y + height.max(1.0)], content),
    };
    Ok(vec![PdfMutation::SetForm(SetForm { form, index: None })])
}

pub(super) fn set_page_extra(snapshot: &PdfSnapshot, page: usize, kind: &str, text: &str, extra: &str, x: f64, y: f64) -> Result<Vec<PdfMutation>, Fault> {
    if let Some(kind) = kind.strip_prefix("box.") {
        return set_page_boundary(snapshot, page, kind, text);
    }
    require_page(snapshot, page)?;
    let mut next = snapshot.pages[page].clone();
    let target = &mut next;
    match kind {
        "thumbnail" => target.thumbnail = none_if_empty(text),
        "metadata" => target.metadata = none_if_empty(text),
        "structParents" => target.struct_parents = (x >= 0.0).then_some(x.round() as u32),
        "group" if text == "clear" => target.group = None,
        "group" => {
            let color_space = match text {
                "" => None,
                "deviceGray" => Some(PdfColorSpace::DeviceGray),
                "deviceRgb" => Some(PdfColorSpace::DeviceRgb),
                "deviceCmyk" => Some(PdfColorSpace::DeviceCmyk),
                other => return Err(fault(format!("unknown group color space '{other}'"))),
            };
            target.group = Some(PdfTransparencyGroup { color_space, isolated: x >= 0.5, knockout: y >= 0.5 });
        }
        "action" | "entry" => {
            if extra.is_empty() {
                return Err(fault("a page entry needs a key"));
            }
            let entries = if kind == "action" { &mut target.additional_actions } else { &mut target.extra };
            if text.is_empty() {
                entries.retain(|item| item.key != extra);
            } else if let Some(existing) = entries.iter_mut().find(|item| item.key == extra) {
                *existing = PdfDictEntry::new(extra, PdfObject::Name(text.to_string()));
            } else {
                entries.push(PdfDictEntry::new(extra, PdfObject::Name(text.to_string())));
            }
        }
        other => return Err(fault(format!("unknown page extra '{other}'"))),
    }
    Ok(vec![PdfMutation::ReplacePage(ReplacePage { index: page, page: next })])
}
//#endregion 📄️DocumentEdits
