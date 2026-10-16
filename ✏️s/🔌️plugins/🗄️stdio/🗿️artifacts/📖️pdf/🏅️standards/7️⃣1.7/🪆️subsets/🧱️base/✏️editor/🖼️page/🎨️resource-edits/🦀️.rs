//! 🎨️ PDF page editor resource edits: glyphs, indirect objects, meshes, graphics states, patterns, colour spaces and font metrics.

use super::*;

//#region 🎨️ResourceEdits
pub(super) fn set_glyph(snapshot: &PdfSnapshot, id: &str, name: &str, x: f64, y: f64, width: f64, height: f64) -> Result<Vec<PdfMutation>, Fault> {
    if name.is_empty() {
        return Err(fault("a glyph needs a name"));
    }
    let mut font = snapshot.fonts.iter().find(|font| font.id == id).cloned().ok_or_else(|| fault(format!("pdf font '{id}' is gone")))?;
    let content = vec![PdfOp::Rectangle { x, y, width: width.max(1.0), height: height.max(1.0) }, PdfOp::Fill];
    match &mut font.kind {
        PdfFontKind::Type3 { char_procs, .. } => {
            if let Some(procedure) = char_procs.iter_mut().find(|procedure| procedure.name == name) {
                procedure.content = content;
            } else {
                char_procs.push(PdfCharProc { name: name.to_string(), content });
            }
        }
        _ => return Err(fault("glyph procedures belong to a Type 3 font")),
    }
    Ok(vec![PdfMutation::SetFont(SetFont { font, index: None })])
}

pub(super) fn set_indirect_object(snapshot: &PdfSnapshot, number: f64, generation: f64, name: &str) -> Result<Vec<PdfMutation>, Fault> {
    let id = ObjRef { num: number.max(0.0).round() as u32, gen: generation.max(0.0).round() as u16 };
    if name.is_empty() {
        if snapshot.objects.iter().any(|object| object.id == id) {
            return Ok(vec![PdfMutation::RemoveObject(RemoveObject { id, admitted_stream_roles: None })]);
        }
        return Err(fault("pdf object is gone"));
    }
    let value = PdfObject::Name(name.to_string());
    let mutation = if snapshot.objects.iter().any(|object| object.id == id) { PdfMutation::SetObjectValue(SetObjectValue { id, value, index: None, admitted_stream_roles: None }) } else { PdfMutation::InsertObject(InsertObject { id, value, index: None, admitted_stream_roles: None }) };
    Ok(vec![mutation])
}

pub(super) fn set_mesh_reference(snapshot: &PdfSnapshot, id: &str, decode_text: &str, artifact_id: &str) -> Result<Vec<PdfMutation>, Fault> {
    let mut shading = snapshot.shadings.iter().find(|item| item.id == id).cloned().ok_or_else(|| fault(format!("pdf shading '{id}' is gone")))?;
    let PdfShadingKind::Mesh { reference, decode, .. } = &mut shading.kind else { return Err(fault(format!("pdf shading '{id}' is not a mesh"))) };
    if !artifact_id.is_empty() { reference.artifact_id = artifact_id.into(); }
    if !decode_text.is_empty() {
        *decode = decode_text.split(',').map(|item| item.trim().parse::<f64>().map_err(|_| fault("mesh decode is comma-separated numbers"))).collect::<Result<Vec<_>, _>>()?;
    }
    Ok(vec![PdfMutation::SetShading(SetShading { shading, index: None })])
}

pub(super) fn line_cap(number: f64) -> Result<PdfLineCap, Fault> {
    match number.round() as i32 {
        0 => Ok(PdfLineCap::Butt),
        1 => Ok(PdfLineCap::Round),
        2 => Ok(PdfLineCap::Square),
        _ => Err(fault("a line cap is 0, 1, or 2")),
    }
}

fn line_join(number: f64) -> Result<PdfLineJoin, Fault> {
    match number.round() as i32 {
        0 => Ok(PdfLineJoin::Miter),
        1 => Ok(PdfLineJoin::Round),
        2 => Ok(PdfLineJoin::Bevel),
        _ => Err(fault("a line join is 0, 1, or 2")),
    }
}

fn clear_number(value: &str, number: f64) -> Option<f64> {
    if value == "clear" || number < 0.0 { None } else { Some(number) }
}

pub(super) fn set_resource_detail(snapshot: &PdfSnapshot, id: &str, aspect: &str, value: &str, x: f64, y: f64, width: f64, height: f64, red: f64, green: f64, blue: f64) -> Result<Vec<PdfMutation>, Fault> {
    let (owner, field) = aspect.split_once('.').ok_or_else(|| fault("a resource detail needs owner.field"))?;
    match owner {
        "outline" => {
            let mut outlines = snapshot.outlines.clone();
            let index = id.parse::<usize>().map_err(|_| fault("an outline index needs a whole number"))?;
            let item = outlines.get_mut(index).ok_or_else(|| fault("that outline is gone"))?;
            match field {
                "bold" => item.bold = x >= 0.5,
                "italic" => item.italic = x >= 0.5,
                "open" => item.open = x >= 0.5,
                "color" => item.color = if value == "clear" { None } else { Some([red, green, blue]) },
                "uri" => item.action = none_if_empty(value).map(PdfAction::uri),
                other => return Err(fault(format!("an outline has no '{other}'"))),
            }
            Ok(vec![PdfMutation::SetOutlines(SetOutlines { outlines })])
        }
        "image" => {
            let mut image = snapshot.images.iter().find(|image| image.id == id).cloned().ok_or_else(|| fault(format!("pdf image '{id}' is gone")))?;
            match field {
                "interpolate" => image.interpolate = x >= 0.5,
                "decode" => image.decode = if value.is_empty() { Vec::new() } else { csv_numbers(value)? },
                "intent" => image.intent = none_if_empty(value),
                "matte" => image.matte = if value.is_empty() || value == "clear" { None } else { Some(csv_numbers(value)?) },
                other => return Err(fault(format!("an image has no '{other}'"))),
            }
            Ok(vec![PdfMutation::SetImage(SetImage { image, index: None })])
        }
        "graphics" => {
            let mut state = snapshot.ext_g_states.iter().find(|item| item.id == id).cloned().unwrap_or_else(|| PdfExtGState { id: id.to_string(), ..PdfExtGState::default() });
            match field {
                "cap" => state.line_cap = Some(line_cap(x)?),
                "join" => state.line_join = Some(line_join(x)?),
                "miter" => state.miter_limit = clear_number(value, x),
                "dash" => state.dash = if value.is_empty() { None } else { Some((csv_numbers(value)?, x.max(0.0))) },
                "intent" => state.rendering_intent = none_if_empty(value),
                "overprintStroke" => state.overprint_stroke = (value != "clear").then_some(x >= 0.5),
                "overprintFill" => state.overprint_fill = (value != "clear").then_some(x >= 0.5),
                "flatness" => state.flatness = clear_number(value, x),
                other => return Err(fault(format!("a graphics state has no '{other}'"))),
            }
            Ok(vec![PdfMutation::SetExtGState(SetExtGState { state, index: None })])
        }
        "font" => {
            let mut font = snapshot.fonts.iter().find(|font| font.id == id).cloned().ok_or_else(|| fault(format!("pdf font '{id}' is gone")))?;
            let base_font = match &font.kind {
                PdfFontKind::Type1 { base_font, .. } | PdfFontKind::TrueType { base_font, .. } => base_font.clone(),
                _ => return Err(fault("font descriptor details apply to a simple font")),
            };
            let descriptor = match &mut font.kind {
                PdfFontKind::Type1 { descriptor, .. } | PdfFontKind::TrueType { descriptor, .. } => descriptor,
                _ => unreachable!(),
            };
            let slot = descriptor.get_or_insert_with(|| PdfFontDescriptor { font_name: base_font, ..PdfFontDescriptor::default() });
            match field {
                "flags" => slot.flags = x.max(0.0).round() as u32,
                "italicAngle" => slot.italic_angle = x,
                "ascent" => slot.ascent = x,
                "descent" => slot.descent = x,
                "capHeight" => slot.cap_height = x,
                "stemV" => slot.stem_v = x,
                "bbox" => slot.font_bbox = [x, y, x + width, y + height],
                other => return Err(fault(format!("a font descriptor has no '{other}'"))),
            }
            Ok(vec![PdfMutation::SetFont(SetFont { font, index: None })])
        }
        "form" => {
            let mut form = snapshot.forms.iter().find(|form| form.id == id).cloned().ok_or_else(|| fault(format!("pdf form '{id}' is gone")))?;
            match field {
                "layer" => form.optional_content = none_if_empty(value),
                "structParent" => form.struct_parent = (x >= 0.0).then_some(x.round() as u32),
                "group" if value == "clear" => form.group = None,
                "group" => {
                    let color_space = match value {
                        "" => None,
                        "deviceGray" => Some(PdfColorSpace::DeviceGray),
                        "deviceRgb" => Some(PdfColorSpace::DeviceRgb),
                        "deviceCmyk" => Some(PdfColorSpace::DeviceCmyk),
                        other => return Err(fault(format!("unknown group color space '{other}'"))),
                    };
                    form.group = Some(PdfTransparencyGroup { color_space, isolated: x >= 0.5, knockout: y >= 0.5 });
                }
                other => return Err(fault(format!("a form has no '{other}'"))),
            }
            Ok(vec![PdfMutation::SetForm(SetForm { form, index: None })])
        }
        "shading" => {
            let mut shading = snapshot.shadings.iter().find(|item| item.id == id).cloned().ok_or_else(|| fault(format!("pdf shading '{id}' is gone")))?;
            match field {
                "antiAlias" => shading.anti_alias = x >= 0.5,
                "background" => shading.background = if value.is_empty() || value == "clear" { None } else { Some(csv_numbers(value)?) },
                "bbox" => shading.bbox = if x < 0.0 { None } else { Some([x, y, x + width, y + height]) },
                other => return Err(fault(format!("a shading has no '{other}'"))),
            }
            Ok(vec![PdfMutation::SetShading(SetShading { shading, index: None })])
        }
        "layers" => {
            let mut content = snapshot.optional_content.clone().unwrap_or_default();
            match field {
                "name" => content.name = none_if_empty(value),
                "baseOff" => content.base_state_off = x >= 0.5,
                other => return Err(fault(format!("optional content has no '{other}'"))),
            }
            Ok(vec![PdfMutation::SetOptionalContent(SetOptionalContent { content: Some(content) })])
        }
        other => Err(fault(format!("unknown resource owner '{other}'"))),
    }
}

pub(super) fn set_graphics_state(snapshot: &PdfSnapshot, id: &str, blend: &str, fill_alpha: f64, stroke_alpha: f64) -> Result<Vec<PdfMutation>, Fault> {
    if id.is_empty() {
        return Err(fault("a graphics state needs an id"));
    }
    let mut state = snapshot.ext_g_states.iter().find(|item| item.id == id).cloned().unwrap_or_else(|| PdfExtGState { id: id.to_string(), ..PdfExtGState::default() });
    state.fill_alpha = Some(fill_alpha.clamp(0.0, 1.0));
    state.stroke_alpha = Some(stroke_alpha.clamp(0.0, 1.0));
    if !blend.is_empty() {
        state.blend_mode = Some(vec![blend.to_string()]);
    }
    Ok(vec![PdfMutation::SetExtGState(SetExtGState { state, index: None })])
}

pub(super) fn set_pattern(snapshot: &PdfSnapshot, id: &str, shading: &str, x: f64, y: f64, width: f64, height: f64) -> Result<Vec<PdfMutation>, Fault> {
    if id.is_empty() {
        return Err(fault("a pattern needs an id"));
    }
    let width = if width <= 0.0 { 8.0 } else { width };
    let height = if height <= 0.0 { 8.0 } else { height };
    let kind = if shading.is_empty() {
        PdfPatternKind::Tiling { paint_type: 1, tiling_type: 1, bbox: [x, y, x + width, y + height], x_step: width, y_step: height, content: Vec::new() }
    } else if snapshot.shadings.iter().any(|item| item.id == shading) {
        PdfPatternKind::Shading { shading: shading.to_string(), ext_g_state: None }
    } else {
        return Err(fault(format!("pdf shading '{shading}' is gone")));
    };
    let matrix = snapshot.patterns.iter().find(|item| item.id == id).map(|item| item.matrix).unwrap_or(PDF_IDENTITY_MATRIX);
    Ok(vec![PdfMutation::SetPattern(SetPattern { pattern: PdfPattern { id: id.to_string(), matrix, kind, extra: Vec::new() }, index: None })])
}

pub(super) fn set_color_space(snapshot: &PdfSnapshot, name: &str, kind: &str, separation: &str) -> Result<Vec<PdfMutation>, Fault> {
    if name.is_empty() {
        return Err(fault("a color space needs a name"));
    }
    let color_space = match kind {
        "deviceGray" => PdfColorSpace::DeviceGray,
        "deviceRgb" => PdfColorSpace::DeviceRgb,
        "deviceCmyk" => PdfColorSpace::DeviceCmyk,
        "separation" if !separation.is_empty() => PdfColorSpace::Separation { name: separation.to_string(), alternate: Box::new(PdfColorSpace::DeviceRgb), tint_transform: PdfFunction::Exponential { domain: vec![0.0, 1.0], range: None, c0: vec![0.0, 0.0, 0.0], c1: vec![1.0, 0.0, 0.0], n: 1.0 } },
        "separation" => return Err(fault("a separation needs a name")),
        other => return Err(fault(format!("unknown color space '{other}'"))),
    };
    let _ = snapshot;
    Ok(vec![PdfMutation::SetColorSpace(SetColorSpace { color_space: PdfNamedColorSpace { name: name.to_string(), color_space }, index: None })])
}

pub(super) fn set_properties(snapshot: &PdfSnapshot, name: &str, key: &str, value: &str) -> Result<Vec<PdfMutation>, Fault> {
    if name.is_empty() || key.is_empty() {
        return Err(fault("a property list needs a name and a key"));
    }
    let mut properties = snapshot.properties.iter().find(|item| item.name == name).cloned().unwrap_or(PdfNamedProperties { name: name.to_string(), entries: Vec::new() });
    let entry = PdfDictEntry::new(key, PdfObject::Name(value.to_string()));
    if let Some(existing) = properties.entries.iter_mut().find(|item| item.key == key) {
        *existing = entry;
    } else {
        properties.entries.push(entry);
    }
    Ok(vec![PdfMutation::SetProperties(SetProperties { properties, index: None })])
}

pub(super) fn set_font_metrics(snapshot: &PdfSnapshot, id: &str, encoding_name: &str, widths_text: &str, first: f64, ascent: f64) -> Result<Vec<PdfMutation>, Fault> {
    let mut font = snapshot.fonts.iter().find(|font| font.id == id).cloned().ok_or_else(|| fault(format!("pdf font '{id}' is gone")))?;
    let encoding = font_encoding(encoding_name)?;
    let widths = font_widths(widths_text)?;
    match &mut font.kind {
        PdfFontKind::Type1 { base_font, encoding: slot, first_char, widths: slot_widths, descriptor, .. } | PdfFontKind::TrueType { base_font, encoding: slot, first_char, widths: slot_widths, descriptor, .. } => {
            write_simple_metrics(base_font, slot, first_char, slot_widths, descriptor, encoding, widths, first, ascent);
        }
        _ => return Err(fault("font metrics apply to a simple font")),
    }
    Ok(vec![PdfMutation::SetFont(SetFont { font, index: None })])
}

fn write_simple_metrics(base_font: &str, slot: &mut PdfSimpleEncoding, first_char: &mut u32, slot_widths: &mut Vec<f64>, descriptor: &mut Option<PdfFontDescriptor>, encoding: Option<PdfSimpleEncoding>, widths: Option<Vec<f64>>, first: f64, ascent: f64) {
    if let Some(encoding) = encoding {
        *slot = encoding;
    }
    if first >= 0.0 {
        *first_char = first.round() as u32;
    }
    if let Some(widths) = widths {
        *slot_widths = widths;
    }
    if ascent != 0.0 {
        descriptor.get_or_insert_with(|| PdfFontDescriptor { font_name: base_font.to_string(), ..PdfFontDescriptor::default() }).ascent = ascent;
    }
}

fn font_encoding(name: &str) -> Result<Option<PdfSimpleEncoding>, Fault> {
    let base = match name {
        "" => return Ok(None),
        "winAnsi" => PdfBaseEncoding::WinAnsi,
        "macRoman" => PdfBaseEncoding::MacRoman,
        "macExpert" => PdfBaseEncoding::MacExpert,
        "standard" => PdfBaseEncoding::Standard,
        other => return Err(fault(format!("unknown font encoding '{other}'"))),
    };
    Ok(Some(PdfSimpleEncoding { base: Some(base), differences: Vec::new() }))
}

fn font_widths(text: &str) -> Result<Option<Vec<f64>>, Fault> {
    if text.is_empty() {
        return Ok(None);
    }
    text.split(',').map(|item| item.trim().parse::<f64>().map_err(|_| fault("font widths are comma-separated numbers"))).collect::<Result<Vec<_>, _>>().map(Some)
}

pub(super) fn remove_named(name: &str, message: &str) -> Result<String, Fault> {
    if name.is_empty() { Err(fault(message)) } else { Ok(name.to_string()) }
}

pub(super) fn edit_shading(snapshot: &PdfSnapshot, object: &PageObject, action: &str, x: f64, y: f64, width: f64, height: f64) -> Result<Vec<PdfMutation>, Fault> {
    let mut shading = snapshot.shadings.iter().find(|item| item.id == object.text).cloned().ok_or_else(|| fault(format!("pdf shading '{}' is gone", object.text)))?;
    let (dx, dy) = user_delta(object.ctm, x - object.x, y - object.y);
    let (x0, y0) = unapply(object.ctm, x, y);
    let (x1, y1) = unapply(object.ctm, x + width, y + height);
    match &mut shading.kind {
        PdfShadingKind::Axial { coords, .. } => {
            if action == "move" {
                coords[0] += dx;
                coords[2] += dx;
                coords[1] += dy;
                coords[3] += dy;
            } else {
                *coords = [x0, y0, x1, y1];
            }
        }
        PdfShadingKind::Radial { coords, .. } => {
            if action == "move" {
                coords[0] += dx;
                coords[3] += dx;
                coords[1] += dy;
                coords[4] += dy;
            } else {
                *coords = [x0, y0, width.max(1.0) / 2.0, x1, y1, height.max(1.0) / 2.0];
            }
        }
        PdfShadingKind::FunctionBased { matrix, .. } => {
            let mut placed = matrix.unwrap_or(PDF_IDENTITY_MATRIX);
            if action == "move" {
                placed[4] += dx;
                placed[5] += dy;
            } else {
                placed[4] = x0;
                placed[5] = y0;
            }
            *matrix = Some(placed);
        }
        PdfShadingKind::Mesh { decode, .. } if decode.len() >= 4 => {
            if action == "move" {
                decode[0] += dx;
                decode[1] += dx;
                decode[2] += dy;
                decode[3] += dy;
            } else {
                decode[0] = x0.min(x1);
                decode[1] = x0.max(x1);
                decode[2] = y0.min(y1);
                decode[3] = y0.max(y1);
            }
        }
        PdfShadingKind::Mesh { .. } => {}
    }
    if let Some(bbox) = &mut shading.bbox {
        if action == "move" {
            bbox[0] += dx;
            bbox[2] += dx;
            bbox[1] += dy;
            bbox[3] += dy;
        } else {
            *bbox = [x0, y0, x1, y1];
        }
    }
    Ok(vec![PdfMutation::SetShading(SetShading { shading, index: None })])
}

pub(super) fn image_color_space(width: u32, height: u32, bytes: usize) -> Result<PdfColorSpace, Fault> {
    let pixels = width as usize * height as usize;
    if bytes == pixels * 3 {
        Ok(PdfColorSpace::DeviceRgb)
    } else if bytes == pixels {
        Ok(PdfColorSpace::DeviceGray)
    } else {
        Err(fault(format!("image samples are {bytes} bytes; expected {pixels} gray or {} rgb", pixels * 3)))
    }
}
//#endregion 🎨️ResourceEdits
