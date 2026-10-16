//! 📄️ PDF page editor document edits: outline, page boxes, metadata, viewer preferences, encryption, forms and font programs.

use super::*;

//#region 📄️DocumentEdits
pub(super) fn set_outline(snapshot: &PdfSnapshot, page: usize, index: f64, title: &str) -> Result<Vec<PdfMutation>, Fault> {
    if title.is_empty() {
        return Err(fault("an outline needs a title"));
    }
    let mut outlines = snapshot.outlines.clone();
    let item_index = if index < 0.0 { outlines.len() } else { index.round() as usize };
    if item_index < outlines.len() {
        outlines[item_index].title = title.to_string();
        outlines[item_index].destination = PdfOutlineItem::to_page(title, page as u32).destination;
    } else {
        outlines.push(PdfOutlineItem::to_page(title, page as u32));
    }
    Ok(vec![PdfMutation::SetOutlines(SetOutlines { outlines })])
}

pub(super) fn set_page_rotation(snapshot: &PdfSnapshot, page: usize, degrees: f64) -> Result<Vec<PdfMutation>, Fault> {
    require_page(snapshot, page)?;
    let rotation = degrees.round() as i32;
    if !matches!(rotation, 0 | 90 | 180 | 270) {
        return Err(fault("page rotation must be 0, 90, 180, or 270"));
    }
    Ok(vec![PdfMutation::SetPageRotation(SetPageRotation { index: page, rotation: rotation as u16 })])
}

pub(super) fn set_page_box(snapshot: &PdfSnapshot, page: usize, kind: &str, x: f64, y: f64, width: f64, height: f64) -> Result<Vec<PdfMutation>, Fault> {
    require_page(snapshot, page)?;
    let kind = match kind {
        "crop" => PdfPageBox::Crop,
        "bleed" => PdfPageBox::Bleed,
        "trim" => PdfPageBox::Trim,
        "art" => PdfPageBox::Art,
        _ => return Err(fault("page box must be crop, bleed, trim, or art")),
    };
    let rect = if width == 0.0 && height == 0.0 {
        None
    } else if width > 0.0 && height > 0.0 {
        Some([x, y, x + width, y + height])
    } else {
        return Err(fault("a page box needs a positive width and height"));
    };
    Ok(vec![PdfMutation::SetPageBox(SetPageBox { index: page, kind, rect })])
}

pub(super) fn set_page_user_unit(snapshot: &PdfSnapshot, page: usize, unit: f64) -> Result<Vec<PdfMutation>, Fault> {
    require_page(snapshot, page)?;
    let user_unit = if unit <= 0.0 { None } else { Some(unit) };
    Ok(vec![PdfMutation::SetPageUserUnit(SetPageUserUnit { index: page, user_unit })])
}

pub(super) fn page_layout(name: &str) -> Result<Option<PdfPageLayout>, Fault> {
    Ok(Some(match name {
        "" => return Ok(None),
        "singlePage" => PdfPageLayout::SinglePage,
        "oneColumn" => PdfPageLayout::OneColumn,
        "twoColumnLeft" => PdfPageLayout::TwoColumnLeft,
        "twoColumnRight" => PdfPageLayout::TwoColumnRight,
        "twoPageLeft" => PdfPageLayout::TwoPageLeft,
        "twoPageRight" => PdfPageLayout::TwoPageRight,
        other => return Err(fault(format!("unknown page layout '{other}'"))),
    }))
}

pub(super) fn page_mode(name: &str) -> Result<Option<PdfPageMode>, Fault> {
    Ok(Some(match name {
        "" => return Ok(None),
        "useNone" => PdfPageMode::UseNone,
        "useOutlines" => PdfPageMode::UseOutlines,
        "useThumbs" => PdfPageMode::UseThumbs,
        "fullScreen" => PdfPageMode::FullScreen,
        "useOc" => PdfPageMode::UseOc,
        "useAttachments" => PdfPageMode::UseAttachments,
        other => return Err(fault(format!("unknown page mode '{other}'"))),
    }))
}

pub(super) fn require_page(snapshot: &PdfSnapshot, page: usize) -> Result<(), Fault> {
    if page >= snapshot.pages.len() {
        Err(fault("pdf page is gone"))
    } else {
        Ok(())
    }
}

pub(super) fn none_if_empty(text: &str) -> Option<String> {
    if text.is_empty() { None } else { Some(text.to_string()) }
}

pub(super) fn set_optional_content(snapshot: &PdfSnapshot, id: &str, name: &str, on: f64) -> Result<Vec<PdfMutation>, Fault> {
    if id.is_empty() || name.is_empty() {
        return Err(fault("an optional content group needs an id and a name"));
    }
    let mut content = snapshot.optional_content.clone().unwrap_or_default();
    if let Some(group) = content.groups.iter_mut().find(|group| group.id == id) {
        group.name = name.to_string();
    } else {
        content.groups.push(PdfOptionalContentGroup { id: id.to_string(), name: name.to_string(), intent: Vec::new(), usage: Vec::new() });
    }
    content.on.retain(|item| item != id);
    content.off.retain(|item| item != id);
    if on >= 0.5 { content.on.push(id.to_string()) } else { content.off.push(id.to_string()) }
    Ok(vec![PdfMutation::SetOptionalContent(SetOptionalContent { content: Some(content) })])
}

pub(super) fn set_embedded_file(snapshot: &PdfSnapshot, id: &str, file_name: &str, contents: &str) -> Result<Vec<PdfMutation>, Fault> {
    if id.is_empty() {
        return Err(fault("an embedded file needs an id"));
    }
    if file_name.is_empty() {
        return Ok(vec![PdfMutation::RemoveEmbeddedFile(RemoveEmbeddedFile { id: id.to_string() })]);
    }
    let previous = snapshot.embedded_files.iter().find(|file| file.id == id);
    Ok(vec![PdfMutation::SetEmbeddedFile(SetEmbeddedFile { file: PdfEmbeddedFile { id: id.to_string(), file_name: file_name.to_string(), description: previous.and_then(|file| file.description.clone()), mime_type: previous.and_then(|file| file.mime_type.clone()), data: contents.as_bytes().to_vec(), creation_date: previous.and_then(|file| file.creation_date.clone()), modification_date: previous.and_then(|file| file.modification_date.clone()), relationship: previous.and_then(|file| file.relationship.clone()), listed: true }, index: None })])
}

pub(super) fn set_named_destination(name: &str, page: usize) -> Result<Vec<PdfMutation>, Fault> {
    if name.is_empty() {
        return Err(fault("a named destination needs a name"));
    }
    Ok(vec![PdfMutation::SetNamedDestination(SetNamedDestination { destination: PdfNamedDestination { name: name.to_string(), destination: PdfDestination::Page { page: page as u32, fit: PdfDestinationFit::Fit } }, index: None })])
}

pub(super) fn set_page_label(snapshot: &PdfSnapshot, page: usize, style: &str, prefix: &str, start: f64) -> Result<Vec<PdfMutation>, Fault> {
    let style = match style {
        "" => None,
        "decimal" => Some(PdfPageLabelStyle::Decimal),
        "romanUpper" => Some(PdfPageLabelStyle::RomanUpper),
        "romanLower" => Some(PdfPageLabelStyle::RomanLower),
        "lettersUpper" => Some(PdfPageLabelStyle::LettersUpper),
        "lettersLower" => Some(PdfPageLabelStyle::LettersLower),
        other => return Err(fault(format!("unknown page label style '{other}'"))),
    };
    let range = PdfPageLabelRange { start_index: page as u32, style, prefix: none_if_empty(prefix), start: start.max(1.0).round() as u32 };
    let mut labels = snapshot.page_labels.clone();
    if let Some(existing) = labels.iter_mut().find(|item| item.start_index == range.start_index) {
        *existing = range;
    } else {
        labels.push(range);
    }
    Ok(vec![PdfMutation::SetPageLabels(SetPageLabels { labels })])
}

pub(super) fn set_mark_info(marked: f64, user_properties: f64, suspects: f64) -> Vec<PdfMutation> {
    let info = PdfMarkInfo { marked: marked >= 0.5, user_properties: user_properties >= 0.5, suspects: suspects >= 0.5 };
    vec![PdfMutation::SetMarkInfo(SetMarkInfo { info: (info != PdfMarkInfo::default()).then_some(info) })]
}

pub(super) fn set_info_field(snapshot: &PdfSnapshot, field: &str, value: &str) -> Result<Vec<PdfMutation>, Fault> {
    let mut info = snapshot.info.clone();
    match field {
        "keywords" => info.keywords = none_if_empty(value),
        "creator" => info.creator = none_if_empty(value),
        "producer" => info.producer = none_if_empty(value),
        "trapped" => info.trapped = none_if_empty(value),
        "creationDate" => info.creation_date = parse_optional_date(value)?,
        "modificationDate" => info.modification_date = parse_optional_date(value)?,
        other => return Err(fault(format!("unknown info field '{other}'"))),
    }
    Ok(vec![PdfMutation::SetInfo(SetInfo { info })])
}

pub(super) fn parse_optional_date(value: &str) -> Result<Option<PdfDate>, Fault> {
    if value.is_empty() {
        return Ok(None);
    }
    crate::standards::v1_7::subsets::base::io::text::snapshot::date::parse_pdf_date(value).map(Some).ok_or_else(|| fault("a document date needs D:YYYYMMDDHHmmSS"))
}

pub(super) fn set_viewer_preference(snapshot: &PdfSnapshot, flag: &str, text: &str, value: f64) -> Result<Vec<PdfMutation>, Fault> {
    let mut preferences = snapshot.viewer_preferences.clone().unwrap_or_default();
    let on = value >= 0.5;
    match flag {
        "hideToolbar" => preferences.hide_toolbar = on,
        "hideMenubar" => preferences.hide_menubar = on,
        "hideWindowUi" => preferences.hide_window_ui = on,
        "fitWindow" => preferences.fit_window = on,
        "centerWindow" => preferences.center_window = on,
        "displayDocTitle" => preferences.display_doc_title = on,
        "pickTrayByPdfSize" => preferences.pick_tray_by_pdf_size = on,
        "numCopies" => preferences.num_copies = (value >= 0.0).then_some(value.round() as u32),
        "nonFullScreenPageMode" => preferences.non_full_screen_page_mode = page_mode(text)?,
        "printPageRange" => preferences.print_page_range = if text.is_empty() { Vec::new() } else { text.split(',').map(str::trim).filter(|item| !item.is_empty()).map(|item| item.parse::<u32>().map_err(|_| fault("a print range needs whole page numbers"))).collect::<Result<Vec<_>, _>>()? },
        "direction" => preferences.direction = none_if_empty(text),
        "viewArea" => preferences.view_area = none_if_empty(text),
        "viewClip" => preferences.view_clip = none_if_empty(text),
        "printArea" => preferences.print_area = none_if_empty(text),
        "printClip" => preferences.print_clip = none_if_empty(text),
        "printScaling" => preferences.print_scaling = none_if_empty(text),
        "duplex" => preferences.duplex = none_if_empty(text),
        other => return Err(fault(format!("unknown viewer preference '{other}'"))),
    }
    Ok(vec![PdfMutation::SetViewerPreferences(SetViewerPreferences { preferences: (preferences != PdfViewerPreferences::default()).then_some(preferences) })])
}

pub(super) fn set_encryption(algorithm: &str, user_password: &str, owner_password: &str, permissions: f64) -> Result<Vec<PdfMutation>, Fault> {
    if algorithm.is_empty() {
        return Ok(vec![PdfMutation::SetEncryption(SetEncryption { encryption: None })]);
    }
    let algorithm = match algorithm {
        "rc4-40" => PdfEncryptionAlgorithm::Rc4_40,
        "rc4-128" => PdfEncryptionAlgorithm::Rc4_128,
        "aes128" => PdfEncryptionAlgorithm::Aes128,
        "aes256" => PdfEncryptionAlgorithm::Aes256,
        other => return Err(fault(format!("unknown encryption '{other}'"))),
    };
    Ok(vec![PdfMutation::SetEncryption(SetEncryption { encryption: Some(PdfEncryption { algorithm, permissions: permissions.round() as i32, user_password: user_password.to_string(), owner_password: none_if_empty(owner_password), encrypt_metadata: true }) })])
}

pub(super) fn set_output_intent(snapshot: &PdfSnapshot, subtype: &str, identifier: &str, info: &str) -> Result<Vec<PdfMutation>, Fault> {
    if subtype.is_empty() {
        return Ok(vec![PdfMutation::SetOutputIntents(SetOutputIntents { intents: Vec::new() })]);
    }
    if identifier.is_empty() {
        return Err(fault("an output intent needs a condition identifier"));
    }
    let mut intents = snapshot.output_intents.clone();
    let profile = intents.iter().find(|item| item.subtype == subtype).and_then(|item| item.profile.clone());
    let intent = PdfOutputIntent { subtype: subtype.to_string(), condition_identifier: identifier.to_string(), condition: None, registry_name: None, info: none_if_empty(info), profile };
    if let Some(existing) = intents.iter_mut().find(|item| item.subtype == subtype) {
        *existing = intent;
    } else {
        intents.push(intent);
    }
    Ok(vec![PdfMutation::SetOutputIntents(SetOutputIntents { intents })])
}

pub(super) fn set_form_field(snapshot: &PdfSnapshot, name: &str, value: &str, kind: &str) -> Result<Vec<PdfMutation>, Fault> {
    if name.is_empty() {
        return Ok(vec![PdfMutation::SetAcroForm(SetAcroForm { form: None })]);
    }
    let field_kind = match kind {
        "" | "text" => PdfFormFieldKind::Text { value: none_if_empty(value), default_value: None, max_length: None, rich_value: None },
        "button" => PdfFormFieldKind::Button { value: none_if_empty(value), default_value: None, options: Vec::new() },
        "choice" => PdfFormFieldKind::Choice { values: none_if_empty(value).into_iter().collect(), default_values: Vec::new(), options: Vec::new(), top_index: None },
        other => return Err(fault(format!("unknown form field kind '{other}'"))),
    };
    let mut form = snapshot.acro_form.clone().unwrap_or_default();
    form.need_appearances = true;
    if let Some(field) = form.fields.iter_mut().find(|field| field.name == name) {
        field.kind = field_kind;
    } else {
        form.fields.push(PdfFormField { name: name.to_string(), kind: field_kind, flags: 0, alternate_name: None, mapping_name: None, default_appearance: None, quadding: None, widgets: Vec::new(), children: Vec::new(), additional_actions: Vec::new(), extra: Vec::new() });
    }
    Ok(vec![PdfMutation::SetAcroForm(SetAcroForm { form: Some(form) })])
}

pub(super) fn set_open_action(kind: &str, uri: &str, page: usize) -> Result<Vec<PdfMutation>, Fault> {
    let action = match kind {
        "" => None,
        "uri" if !uri.is_empty() => Some(PdfOpenAction::Action { action: PdfAction::uri(uri) }),
        "uri" => return Err(fault("an open uri needs an address")),
        "page" => Some(PdfOpenAction::Destination { destination: PdfDestination::Page { page: page as u32, fit: PdfDestinationFit::Fit } }),
        other => return Err(fault(format!("unknown open action '{other}'"))),
    };
    Ok(vec![PdfMutation::SetOpenAction(SetOpenAction { action })])
}

pub(super) fn set_document_id(permanent: &str, changing: &str) -> Vec<PdfMutation> {
    let id = if permanent.is_empty() && changing.is_empty() {
        None
    } else {
        let first = if permanent.is_empty() { changing.as_bytes().to_vec() } else { permanent.as_bytes().to_vec() };
        let second = if changing.is_empty() { first.clone() } else { changing.as_bytes().to_vec() };
        Some([first, second])
    };
    vec![PdfMutation::SetDocumentId(SetDocumentId { id })]
}

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
    let object = objects(snapshot).into_iter().find(|item| item.id == address).ok_or_else(|| fault(format!("pdf image '{address}' is gone")))?;
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
