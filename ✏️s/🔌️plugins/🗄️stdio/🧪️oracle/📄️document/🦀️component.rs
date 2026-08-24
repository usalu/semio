//! 🔮️ Document-family oracles: the PDF reference implementations, and the OOXML CONFORMANCE-CLASS
//! engine the six `✳️strict`/`✳️transitional` subsets of `📕️xlsx`, `📜️docx` and `🎞️pptx` share.
//! Every registered reference library is wrapped behind an owned interface — no external type
//! appears in this module's public API, so nothing downstream can accidentally depend on
//! `pdf-writer`, `lopdf`, `zip` or `quick-xml`. Compiled only with the `oracles` feature, which no
//! production target enables.
//!
//! @see 📇️registry/🔣️component.json — the approved oracle registry these functions implement.

use semio_repo_test_host::Json;

//#region 🔖️PdfSpec
/// 📄️ Owned description of a PDF to create. Deliberately independent of any writer library so the
/// same spec drives the oracle and every repository implementation.
#[derive(Debug, Clone)]
pub struct PdfPageSpec {
    pub media_box: [f32; 4],
    pub content: String,
}

/// 📄️ Owned creation request: version, pages, and the normative document metadata.
#[derive(Debug, Clone)]
pub struct PdfSpec {
    pub version: (u8, u8),
    pub pages: Vec<PdfPageSpec>,
    pub title: Option<String>,
    pub author: Option<String>,
}

impl PdfSpec {
    /// 📄️ Reads a spec out of a scenario's owned JSON payload.
    pub fn from_json(value: &Json) -> PdfSpec {
        let version = value.str("version");
        let mut parts = version.split('.');
        let major = parts.next().and_then(|part| part.parse::<u8>().ok()).unwrap_or(1);
        let minor = parts.next().and_then(|part| part.parse::<u8>().ok()).unwrap_or(7);
        let pages = value
            .array("pages")
            .iter()
            .map(|page| {
                let numbers: Vec<f32> = match page.get("mediaBox") {
                    Some(Json::Array(items)) => items
                        .iter()
                        .map(|item| match item {
                            Json::Number(number) => *number as f32,
                            _ => 0.0,
                        })
                        .collect(),
                    _ => vec![0.0, 0.0, 595.0, 842.0],
                };
                let media_box = [numbers.first().copied().unwrap_or(0.0), numbers.get(1).copied().unwrap_or(0.0), numbers.get(2).copied().unwrap_or(595.0), numbers.get(3).copied().unwrap_or(842.0)];
                PdfPageSpec { media_box, content: page.str("content") }
            })
            .collect();
        let title = match value.get("title") {
            Some(Json::String(text)) => Some(text.clone()),
            _ => None,
        };
        let author = match value.get("author") {
            Some(Json::String(text)) => Some(text.clone()),
            _ => None,
        };
        PdfSpec { version: (major, minor), pages, title, author }
    }
}
//#endregion 🔖️PdfSpec

//#region 🔖️PdfCreationOracle
/// 🔮️ Creates a PDF with the registered `pdf-writer` creation oracle.
/// @see https://github.com/typst/pdf-writer
#[cfg(feature = "oracles")]
pub fn oracle_create_pdf(spec: &PdfSpec) -> Result<Vec<u8>, String> {
    use pdf_writer::{Content, Finish, Name, Pdf, Rect, Ref, Str};

    let mut pdf = Pdf::new();
    pdf.set_version(spec.version.0, spec.version.1);

    let catalog_id = Ref::new(1);
    let page_tree_id = Ref::new(2);
    let info_id = Ref::new(3);
    let font_id = Ref::new(4);
    let mut next = 5i32;

    let page_ids: Vec<Ref> = spec
        .pages
        .iter()
        .map(|_| {
            let id = Ref::new(next);
            next += 2;
            id
        })
        .collect();

    pdf.catalog(catalog_id).pages(page_tree_id);
    {
        let mut pages = pdf.pages(page_tree_id);
        pages.kids(page_ids.iter().copied());
        pages.count(spec.pages.len() as i32);
        pages.finish();
    }

    for (index, page_spec) in spec.pages.iter().enumerate() {
        let _ = index;
        let page_id = page_ids[index];
        let content_id = Ref::new(page_id.get() + 1);
        {
            let mut page = pdf.page(page_id);
            page.parent(page_tree_id);
            page.media_box(Rect::new(page_spec.media_box[0], page_spec.media_box[1], page_spec.media_box[2], page_spec.media_box[3]));
            page.resources().fonts().pair(Name(b"F1"), font_id);
            page.contents(content_id);
            page.finish();
        }
        let mut content = Content::new();
        if !page_spec.content.is_empty() {
            content.begin_text();
            content.set_font(Name(b"F1"), 12.0);
            content.next_line(72.0, page_spec.media_box[3] - 72.0);
            content.show(Str(page_spec.content.as_bytes()));
            content.end_text();
        }
        pdf.stream(content_id, &content.finish());
    }

    pdf.type1_font(font_id).base_font(Name(b"Helvetica"));

    {
        let mut info = pdf.document_info(info_id);
        if let Some(title) = &spec.title {
            info.title(pdf_writer::TextStr(title));
        }
        if let Some(author) = &spec.author {
            info.author(pdf_writer::TextStr(author));
        }
        info.finish();
    }

    Ok(pdf.finish())
}
//#endregion 🔖️PdfCreationOracle

//#region 🔖️PdfEditingOracle
/// 🔮️ Replaces the document metadata of an existing PDF with the registered `lopdf` editing oracle.
/// @see https://github.com/J-F-Liu/lopdf
#[cfg(feature = "oracles")]
pub fn oracle_replace_metadata(input: &[u8], title: Option<&str>, author: Option<&str>) -> Result<Vec<u8>, String> {
    use lopdf::{Dictionary, Document, Object};

    let mut document = Document::load_mem(input).map_err(|error| format!("lopdf could not parse the input: {}", error))?;
    let mut info = Dictionary::new();
    if let Some(value) = title {
        info.set("Title", Object::string_literal(value));
    }
    if let Some(value) = author {
        info.set("Author", Object::string_literal(value));
    }
    let info_id = document.add_object(Object::Dictionary(info));
    document.trailer.set("Info", Object::Reference(info_id));
    let mut out = Vec::new();
    document.save_to(&mut out).map_err(|error| format!("lopdf could not save: {}", error))?;
    Ok(out)
}

/// 🔮️ Deletes one 1-based page from an existing PDF with the registered `lopdf` editing oracle.
#[cfg(feature = "oracles")]
pub fn oracle_delete_page(input: &[u8], page_number: u32) -> Result<Vec<u8>, String> {
    use lopdf::Document;

    let mut document = Document::load_mem(input).map_err(|error| format!("lopdf could not parse the input: {}", error))?;
    document.delete_pages(&[page_number]);
    let mut out = Vec::new();
    document.save_to(&mut out).map_err(|error| format!("lopdf could not save: {}", error))?;
    Ok(out)
}
//#endregion 🔖️PdfEditingOracle

//#region 🔖️PdfProjection
/// 👁️ Projects PDF bytes onto the owned `semantic-pdf-v1` shape using an INDEPENDENT parser, so a
/// producer can never be checked against its own reading of what it wrote. Nondeterministic
/// artefacts (object numbers, xref offsets, timestamps, ids) are excluded by construction; the
/// comparison profile strips any that slip through.
#[cfg(feature = "oracles")]
pub fn project_pdf(input: &[u8]) -> Result<Json, String> {
    use lopdf::{Document, Object};

    let document = Document::load_mem(input).map_err(|error| format!("independent reader could not parse the document: {}", error))?;
    let pages = document.get_pages();

    let number = |object: &Object| -> f64 {
        match object {
            Object::Integer(value) => *value as f64,
            Object::Real(value) => *value as f64,
            _ => 0.0,
        }
    };

    let mut page_entries: Vec<Json> = Vec::new();
    for (_, page_id) in pages.iter() {
        let dictionary = document.get_dictionary(*page_id).map_err(|error| format!("page dictionary unreadable: {}", error))?;
        let media_box = match dictionary.get(b"MediaBox").ok().and_then(|value| value.as_array().ok()) {
            Some(items) => Json::Array(items.iter().map(|item| Json::Number(number(item))).collect()),
            None => Json::Null,
        };
        let content = document.get_page_content(*page_id);
        let operators: Vec<Json> = lopdf::content::Content::decode(&content).map(|decoded| decoded.operations.iter().map(|operation| Json::String(operation.operator.clone())).collect()).unwrap_or_default();
        let text: Vec<Json> = lopdf::content::Content::decode(&content)
            .map(|decoded| {
                decoded
                    .operations
                    .iter()
                    .filter(|operation| operation.operator == "Tj")
                    .flat_map(|operation| operation.operands.iter())
                    .filter_map(|operand| operand.as_str().ok().map(|bytes| Json::String(String::from_utf8_lossy(bytes).to_string())))
                    .collect()
            })
            .unwrap_or_default();
        page_entries.push(Json::Object(vec![("mediaBox".to_string(), media_box), ("contentOperators".to_string(), Json::Array(operators)), ("text".to_string(), Json::Array(text))]));
    }

    let metadata_value = |key: &[u8]| -> Json {
        document
            .trailer
            .get(b"Info")
            .ok()
            .and_then(|info| match info {
                Object::Reference(id) => document.get_dictionary(*id).ok(),
                Object::Dictionary(dictionary) => Some(dictionary),
                _ => None,
            })
            .and_then(|dictionary| dictionary.get(key).ok())
            .and_then(|value| value.as_str().ok())
            .map(|bytes| Json::String(String::from_utf8_lossy(bytes).to_string()))
            .unwrap_or(Json::Null)
    };

    Ok(Json::Object(vec![
        ("version".to_string(), Json::String(document.version.clone())),
        ("pageCount".to_string(), Json::Number(pages.len() as f64)),
        ("pages".to_string(), Json::Array(page_entries)),
        ("metadata".to_string(), Json::Object(vec![("title".to_string(), metadata_value(b"Title")), ("author".to_string(), metadata_value(b"Author"))])),
        ("parsedByIndependentReader".to_string(), Json::Bool(true)),
    ]))
}
//#endregion 🔖️PdfProjection

//#region 🔖️Unavailable
/// 🚫️ Without the `oracles` feature the reference implementations are not linked at all, and every
/// entry point fails loudly. A missing oracle must never degrade into a silently skipped test.
#[cfg(not(feature = "oracles"))]
pub fn oracle_create_pdf(_spec: &PdfSpec) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

#[cfg(not(feature = "oracles"))]
pub fn oracle_replace_metadata(_input: &[u8], _title: Option<&str>, _author: Option<&str>) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

#[cfg(not(feature = "oracles"))]
pub fn oracle_delete_page(_input: &[u8], _page_number: u32) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

#[cfg(not(feature = "oracles"))]
pub fn project_pdf(_input: &[u8]) -> Result<Json, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️Unavailable

//#region 🔖️OoxmlConformanceClass
/// 🏅️ The OOXML CONFORMANCE-CLASS engine — one independent OPC + XML implementation (`zip` 6 for
/// the container, `quick-xml` 0.42 for every part), shared by the six subsets that police a
/// conformance class rather than document content: `📕️xlsx`, `📜️docx` and `🎞️pptx`, each in its
/// `✳️strict` (ISO/IEC 29500-1) and `✳️transitional` (ISO/IEC 29500-4) flavour.
///
/// It lives in the shared family module rather than in any one subset because all six genuinely
/// share it: the axes their conformance checkers read — the main-namespace declaration, the
/// DrawingML namespace, the `officeDocument` relationship base, the root `conformance` attribute,
/// the legacy VML and `mc:AlternateContent` markup — are package-level facts of the OPC container,
/// identical in mechanism across the three formats and differing only in which part carries them.
///
/// Nothing here interprets a workbook, a document body or a slide: that is the `✳️any` subsets'
/// vocabulary, and their own reference pairings answer for it. This engine reads and writes the
/// package, which is exactly the surface a conformance class is defined on.
#[cfg(feature = "oracles")]
pub mod ooxml {
    use quick_xml::events::{BytesStart, Event};
    use quick_xml::reader::Reader;
    use quick_xml::writer::Writer;
    use quick_xml::XmlVersion;
    use semio_repo_test_host::Json;
    use std::io::{Cursor, Read, Write};

    //#region 🔖️Container
    /// 📦️ Independent container read: every zip entry in its stored order, verbatim.
    pub fn read_parts(input: &[u8]) -> Result<Vec<(String, Vec<u8>)>, String> {
        let mut archive = zip::ZipArchive::new(Cursor::new(input.to_vec())).map_err(|error| format!("independent OPC reader could not open the package: {error}"))?;
        let mut parts = Vec::new();
        for index in 0..archive.len() {
            let mut member = archive.by_index(index).map_err(|error| format!("independent OPC reader could not read entry {index}: {error}"))?;
            if member.is_dir() {
                continue;
            }
            let name = member.name().to_string();
            let mut bytes = Vec::new();
            member.read_to_end(&mut bytes).map_err(|error| format!("independent OPC reader could not inflate entry {name:?}: {error}"))?;
            parts.push((name, bytes));
        }
        Ok(parts)
    }

    /// 📦️ Independent container write: a brand-new deflate archive assembled from `parts`, never a
    /// patch of the input bytes — rebuilding the whole container is what makes this a second
    /// producer rather than a byte editor.
    pub fn write_parts(parts: &[(String, Vec<u8>)]) -> Result<Vec<u8>, String> {
        let mut cursor = Cursor::new(Vec::new());
        {
            let mut writer = zip::ZipWriter::new(&mut cursor);
            let options: zip::write::FileOptions<'_, ()> = zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Deflated);
            for (name, bytes) in parts {
                writer.start_file(name.clone(), options).map_err(|error| format!("independent OPC writer could not start entry {name:?}: {error}"))?;
                writer.write_all(bytes).map_err(|error| format!("independent OPC writer could not write entry {name:?}: {error}"))?;
            }
            writer.finish().map_err(|error| format!("independent OPC writer could not finish the package: {error}"))?;
        }
        Ok(cursor.into_inner())
    }

    pub fn part_bytes<'a>(parts: &'a [(String, Vec<u8>)], path: &str) -> Option<&'a [u8]> {
        parts.iter().find(|(name, _)| name == path).map(|(_, bytes)| bytes.as_slice())
    }

    pub fn set_part(parts: &mut Vec<(String, Vec<u8>)>, path: &str, bytes: Vec<u8>) {
        match parts.iter_mut().find(|(name, _)| name == path) {
            Some(existing) => existing.1 = bytes,
            None => parts.push((path.to_string(), bytes)),
        }
    }

    pub fn remove_part(parts: &mut Vec<(String, Vec<u8>)>, path: &str) -> bool {
        let before = parts.len();
        parts.retain(|(name, _)| name != path);
        parts.len() != before
    }

    /// 📰️ Whether an entry is XML this engine may parse. Media parts (`.png`, `.jpeg`, …) and the
    /// `.bin` OLE payloads a real deck carries are left byte-for-byte alone.
    pub fn is_xml_part(path: &str) -> bool {
        let lower = path.to_ascii_lowercase();
        lower.ends_with(".xml") || lower.ends_with(".rels")
    }

    fn contains(haystack: &[u8], needle: &str) -> bool {
        !needle.is_empty() && haystack.windows(needle.len()).any(|window| window == needle.as_bytes())
    }
    //#endregion 🔖️Container

    //#region 🔖️XmlEdits
    fn attributes_of(start: &BytesStart) -> Result<Vec<(String, String)>, String> {
        start
            .attributes()
            .map(|attribute| {
                let attribute = attribute.map_err(|error| error.to_string())?;
                let value = attribute.normalized_value(XmlVersion::Explicit1_0).map_err(|error| error.to_string())?;
                Ok((attribute.key.as_ref().to_string(), value.to_string()))
            })
            .collect()
    }

    fn element_name(start: &BytesStart) -> Result<String, String> {
        Ok(start.name().as_ref().to_string())
    }

    fn rebuilt(name: &str, attrs: &[(String, String)]) -> BytesStart<'static> {
        let mut rebuilt = BytesStart::new(name.to_string());
        for (key, value) in attrs {
            rebuilt.push_attribute((key.as_str(), value.as_str()));
        }
        rebuilt
    }

    /// ✍️ Streams `part` through `quick-xml`, offering every element's attribute list to `edit`
    /// (`(name, depth, attrs) -> changed`), and re-emits the whole document with `quick-xml`'s own
    /// writer. Declaration, doctype, comments, processing instructions, text, entity references and
    /// CDATA are written back unchanged.
    fn rewrite_elements(part: &[u8], mut edit: impl FnMut(&str, usize, &mut Vec<(String, String)>) -> bool) -> Result<Vec<u8>, String> {
        let text = std::str::from_utf8(part).map_err(|error| format!("part is not valid utf-8: {error}"))?;
        let mut reader = Reader::from_str(text);
        let mut writer = Writer::new(Cursor::new(Vec::new()));
        let mut depth = 0usize;
        loop {
            let event = reader.read_event().map_err(|error| format!("quick-xml parse error at byte {}: {error}", reader.error_position()))?;
            match event {
                Event::Eof => break,
                Event::Start(start) => {
                    let name = element_name(&start)?;
                    let mut attrs = attributes_of(&start)?;
                    let changed = edit(&name, depth, &mut attrs);
                    depth += 1;
                    let emitted = if changed { rebuilt(&name, &attrs) } else { start.into_owned() };
                    writer.write_event(Event::Start(emitted)).map_err(|error| error.to_string())?;
                }
                Event::Empty(start) => {
                    let name = element_name(&start)?;
                    let mut attrs = attributes_of(&start)?;
                    let changed = edit(&name, depth, &mut attrs);
                    let emitted = if changed { rebuilt(&name, &attrs) } else { start.into_owned() };
                    writer.write_event(Event::Empty(emitted)).map_err(|error| error.to_string())?;
                }
                Event::End(end) => {
                    depth = depth.saturating_sub(1);
                    writer.write_event(Event::End(end)).map_err(|error| error.to_string())?;
                }
                other => writer.write_event(other).map_err(|error| error.to_string())?,
            }
        }
        Ok(writer.into_inner().into_inner())
    }

    /// 🔎️ The root element's `(name, attributes)`, read independently.
    pub fn root_element(part: &[u8]) -> Result<(String, Vec<(String, String)>), String> {
        let text = std::str::from_utf8(part).map_err(|error| format!("part is not valid utf-8: {error}"))?;
        let mut reader = Reader::from_str(text);
        loop {
            match reader.read_event().map_err(|error| format!("quick-xml parse error at byte {}: {error}", reader.error_position()))? {
                Event::Eof => return Err("part has no root element".to_string()),
                Event::Start(start) | Event::Empty(start) => return Ok((element_name(&start)?, attributes_of(&start)?)),
                _ => {}
            }
        }
    }

    pub fn root_attribute(part: &[u8], name: &str) -> Result<Option<String>, String> {
        Ok(root_element(part)?.1.into_iter().find(|(key, _)| key == name).map(|(_, value)| value))
    }

    /// ✍️ Sets — or, with `None`, removes — one attribute on the ROOT element only.
    pub fn set_root_attribute(part: &[u8], name: &str, value: Option<&str>) -> Result<Vec<u8>, String> {
        let mut seen_root = false;
        rewrite_elements(part, |_, depth, attrs| {
            if depth != 0 || seen_root {
                return false;
            }
            seen_root = true;
            match (attrs.iter().position(|(key, _)| key == name), value) {
                (Some(index), Some(value)) => attrs[index].1 = value.to_string(),
                (Some(index), None) => {
                    attrs.remove(index);
                }
                (None, Some(value)) => attrs.push((name.to_string(), value.to_string())),
                (None, None) => return false,
            }
            true
        })
    }

    /// ✍️ Replaces every attribute value that exactly equals one of `from` with `to`, on every
    /// element of the part. A namespace declaration is an ordinary attribute at the XML level, which
    /// is why one primitive covers `xmlns`, `xmlns:w`, `xmlns:a` and whatever prefixed alias a real
    /// package happens to use.
    pub fn replace_attribute_values(part: &[u8], from: &[&str], to: &str) -> Result<Vec<u8>, String> {
        rewrite_elements(part, |_, _, attrs| {
            let mut changed = false;
            for (_, value) in attrs.iter_mut() {
                if from.contains(&value.as_str()) && value != to {
                    *value = to.to_string();
                    changed = true;
                }
            }
            changed
        })
    }

    /// ✍️ Replaces the leading `from` prefix of every attribute value that starts with it by `to` —
    /// the relationship-base swap, where `…/officeDocument/2006/relationships/officeDocument` becomes
    /// `…/ooxml/officeDocument/relationships/officeDocument` without touching the package
    /// relationship types (`…/package/2006/relationships/…`) both conformance classes share.
    pub fn replace_attribute_value_prefixes(part: &[u8], from: &[&str], to: &str) -> Result<Vec<u8>, String> {
        rewrite_elements(part, |_, _, attrs| {
            let mut changed = false;
            for (_, value) in attrs.iter_mut() {
                for prefix in from {
                    if value.starts_with(prefix) && *prefix != to {
                        *value = format!("{to}{}", &value[prefix.len()..]);
                        changed = true;
                        break;
                    }
                }
            }
            changed
        })
    }
    /// ➕️ Appends `markup` as the last child of the part's ROOT element. The fragment is parsed by
    /// `quick-xml` and forwarded event by event, never spliced as text, so malformed markup fails
    /// loudly instead of producing a package that only looks edited.
    pub fn append_root_child(part: &[u8], markup: &str) -> Result<Vec<u8>, String> {
        let text = std::str::from_utf8(part).map_err(|error| format!("part is not valid utf-8: {error}"))?;
        let mut reader = Reader::from_str(text);
        let mut writer = Writer::new(Cursor::new(Vec::new()));
        let mut depth = 0usize;
        let mut inserted = false;
        loop {
            let event = reader.read_event().map_err(|error| format!("quick-xml parse error at byte {}: {error}", reader.error_position()))?;
            match event {
                Event::Eof => break,
                Event::Start(start) => {
                    depth += 1;
                    writer.write_event(Event::Start(start)).map_err(|error| error.to_string())?;
                }
                Event::End(end) => {
                    if depth == 1 && !inserted {
                        let mut fragment = Reader::from_str(markup);
                        loop {
                            match fragment.read_event().map_err(|error| format!("quick-xml rejected the fragment at byte {}: {error}", fragment.error_position()))? {
                                Event::Eof => break,
                                event => writer.write_event(event).map_err(|error| error.to_string())?,
                            }
                        }
                        inserted = true;
                    }
                    depth = depth.saturating_sub(1);
                    writer.write_event(Event::End(end)).map_err(|error| error.to_string())?;
                }
                other => writer.write_event(other).map_err(|error| error.to_string())?,
            }
        }
        if !inserted {
            return Err("part has no root element to append a child to".to_string());
        }
        Ok(writer.into_inner().into_inner())
    }

    /// ➖️ Drops every direct child of the ROOT element named `name`, subtree and all.
    pub fn remove_root_children(part: &[u8], name: &str) -> Result<Vec<u8>, String> {
        let text = std::str::from_utf8(part).map_err(|error| format!("part is not valid utf-8: {error}"))?;
        let mut reader = Reader::from_str(text);
        let mut writer = Writer::new(Cursor::new(Vec::new()));
        let mut depth = 0usize;
        let mut skipping = 0usize;
        let mut removed = false;
        loop {
            let event = reader.read_event().map_err(|error| format!("quick-xml parse error at byte {}: {error}", reader.error_position()))?;
            match event {
                Event::Eof => break,
                Event::Start(start) => {
                    let matched = depth == 1 && element_name(&start)? == name;
                    depth += 1;
                    if skipping > 0 || matched {
                        skipping += 1;
                        removed = true;
                        continue;
                    }
                    writer.write_event(Event::Start(start)).map_err(|error| error.to_string())?;
                }
                Event::Empty(start) => {
                    if skipping > 0 {
                        continue;
                    }
                    if depth == 1 && element_name(&start)? == name {
                        removed = true;
                        continue;
                    }
                    writer.write_event(Event::Empty(start)).map_err(|error| error.to_string())?;
                }
                Event::End(end) => {
                    depth = depth.saturating_sub(1);
                    if skipping > 0 {
                        skipping -= 1;
                        continue;
                    }
                    writer.write_event(Event::End(end)).map_err(|error| error.to_string())?;
                }
                other => {
                    if skipping > 0 {
                        continue;
                    }
                    writer.write_event(other).map_err(|error| error.to_string())?;
                }
            }
        }
        if !removed {
            return Err(format!("part declares no root child named {name:?} to remove"));
        }
        Ok(writer.into_inner().into_inner())
    }

    //#endregion 🔖️XmlEdits

    //#region 🔖️ContentTypes
    pub const CONTENT_TYPES_PART: &str = "[Content_Types].xml";

    /// 🏷️ `([(extension, content type)], [(part name, content type)])` read out of
    /// `[Content_Types].xml`.
    pub fn content_types(parts: &[(String, Vec<u8>)]) -> Result<(Vec<(String, String)>, Vec<(String, String)>), String> {
        let bytes = part_bytes(parts, CONTENT_TYPES_PART).ok_or("package has no [Content_Types].xml")?;
        let text = std::str::from_utf8(bytes).map_err(|error| format!("[Content_Types].xml is not valid utf-8: {error}"))?;
        let mut reader = Reader::from_str(text);
        let (mut defaults, mut overrides) = (Vec::new(), Vec::new());
        loop {
            match reader.read_event().map_err(|error| format!("quick-xml parse error at byte {}: {error}", reader.error_position()))? {
                Event::Eof => break,
                Event::Start(start) | Event::Empty(start) => {
                    let name = element_name(&start)?;
                    let attrs = attributes_of(&start)?;
                    let attribute = |key: &str| attrs.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone()).unwrap_or_default();
                    match name.as_str() {
                        "Default" => defaults.push((attribute("Extension").to_ascii_lowercase(), attribute("ContentType"))),
                        "Override" => overrides.push((attribute("PartName"), attribute("ContentType"))),
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        Ok((defaults, overrides))
    }

    /// ✍️ Regenerates `[Content_Types].xml` from a typed table with `quick-xml`'s writer — the
    /// engine rebuilds the part rather than splicing it, for the same reason `write_parts` rebuilds
    /// the container.
    pub fn write_content_types(parts: &mut Vec<(String, Vec<u8>)>, defaults: &[(String, String)], overrides: &[(String, String)]) -> Result<(), String> {
        let mut writer = Writer::new(Cursor::new(Vec::new()));
        writer.write_event(Event::Decl(quick_xml::events::BytesDecl::new("1.0", Some("UTF-8"), Some("yes")))).map_err(|error| error.to_string())?;
        let mut root = BytesStart::new("Types");
        root.push_attribute(("xmlns", "http://schemas.openxmlformats.org/package/2006/content-types"));
        writer.write_event(Event::Start(root)).map_err(|error| error.to_string())?;
        for (extension, content_type) in defaults {
            let mut entry = BytesStart::new("Default");
            entry.push_attribute(("Extension", extension.as_str()));
            entry.push_attribute(("ContentType", content_type.as_str()));
            writer.write_event(Event::Empty(entry)).map_err(|error| error.to_string())?;
        }
        for (part_name, content_type) in overrides {
            let mut entry = BytesStart::new("Override");
            entry.push_attribute(("PartName", part_name.as_str()));
            entry.push_attribute(("ContentType", content_type.as_str()));
            writer.write_event(Event::Empty(entry)).map_err(|error| error.to_string())?;
        }
        writer.write_event(Event::End(quick_xml::events::BytesEnd::new("Types"))).map_err(|error| error.to_string())?;
        set_part(parts, CONTENT_TYPES_PART, writer.into_inner().into_inner());
        Ok(())
    }

    /// ✍️ Sets — or, with `None`, removes — the `Override` entry for `part_path`.
    pub fn set_content_type_override(parts: &mut Vec<(String, Vec<u8>)>, part_path: &str, content_type: Option<&str>) -> Result<(), String> {
        let part_name = format!("/{}", part_path.trim_start_matches('/'));
        let (defaults, mut overrides) = content_types(parts)?;
        overrides.retain(|(name, _)| *name != part_name);
        if let Some(content_type) = content_type {
            overrides.push((part_name, content_type.to_string()));
        }
        write_content_types(parts, &defaults, &overrides)
    }

    /// 🔎️ The effective content type of `part_path`: its `Override` first, else the `Default` for
    /// its extension.
    pub fn resolve_content_type(defaults: &[(String, String)], overrides: &[(String, String)], part_path: &str) -> String {
        let part_name = format!("/{}", part_path.trim_start_matches('/'));
        if let Some((_, content_type)) = overrides.iter().find(|(name, _)| *name == part_name) {
            return content_type.clone();
        }
        let extension = part_path.rsplit('.').next().unwrap_or_default().to_ascii_lowercase();
        defaults.iter().find(|(candidate, _)| *candidate == extension).map(|(_, content_type)| content_type.clone()).unwrap_or_default()
    }
    //#endregion 🔖️ContentTypes

    //#region 🔖️PackageFacts
    /// 🧭️ The root `officeDocument` relationship's target — the main part, matched by relationship
    /// type SUFFIX so it resolves under either conformance class.
    pub fn main_part(parts: &[(String, Vec<u8>)]) -> Result<String, String> {
        let bytes = part_bytes(parts, "_rels/.rels").ok_or("package has no _rels/.rels")?;
        let text = std::str::from_utf8(bytes).map_err(|error| format!("_rels/.rels is not valid utf-8: {error}"))?;
        let mut reader = Reader::from_str(text);
        loop {
            match reader.read_event().map_err(|error| format!("quick-xml parse error at byte {}: {error}", reader.error_position()))? {
                Event::Eof => return Err("package root relationships declare no officeDocument relationship".to_string()),
                Event::Start(start) | Event::Empty(start) => {
                    let attrs = attributes_of(&start)?;
                    let attribute = |key: &str| attrs.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone()).unwrap_or_default();
                    if attribute("Type").ends_with("/officeDocument") {
                        return Ok(attribute("Target").trim_start_matches('/').to_string());
                    }
                }
                _ => {}
            }
        }
    }

    /// 🔗️ Every distinct `Relationship/@Type` in every `.rels` part of the package, sorted.
    pub fn relationship_types(parts: &[(String, Vec<u8>)]) -> Result<Vec<String>, String> {
        let mut types = Vec::new();
        for (path, bytes) in parts.iter().filter(|(path, _)| path.ends_with(".rels")) {
            let text = std::str::from_utf8(bytes).map_err(|error| format!("{path} is not valid utf-8: {error}"))?;
            let mut reader = Reader::from_str(text);
            loop {
                match reader.read_event().map_err(|error| format!("quick-xml parse error in {path} at byte {}: {error}", reader.error_position()))? {
                    Event::Eof => break,
                    Event::Start(start) | Event::Empty(start) => {
                        for (key, value) in attributes_of(&start)? {
                            if key == "Type" && !types.contains(&value) {
                                types.push(value);
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        types.sort();
        Ok(types)
    }

    /// 🏷️ Every distinct OOXML-family namespace URI declared anywhere in the package — the
    /// conformance class made observable. Both ISO/IEC 29500 families and the legacy VML urn are
    /// recognised; anything else (Dublin Core, the markup-compatibility namespace, a vendor
    /// extension) is not part of the class and is deliberately not projected.
    pub fn declared_namespaces(parts: &[(String, Vec<u8>)]) -> Result<Vec<String>, String> {
        const FAMILIES: [&str; 3] = ["http://schemas.openxmlformats.org/", "http://purl.oclc.org/ooxml/", "urn:schemas-microsoft-com:vml"];
        let mut namespaces: Vec<String> = Vec::new();
        for (path, bytes) in parts.iter().filter(|(path, _)| is_xml_part(path)) {
            let Ok(text) = std::str::from_utf8(bytes) else { continue };
            let mut reader = Reader::from_str(text);
            loop {
                match reader.read_event().map_err(|error| format!("quick-xml parse error in {path} at byte {}: {error}", reader.error_position()))? {
                    Event::Eof => break,
                    Event::Start(start) | Event::Empty(start) => {
                        for (key, value) in attributes_of(&start)? {
                            if (key == "xmlns" || key.starts_with("xmlns:")) && FAMILIES.iter().any(|family| value.starts_with(family)) && !namespaces.contains(&value) {
                                namespaces.push(value);
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        namespaces.sort();
        Ok(namespaces)
    }

    /// 🧩️ Parts carrying `mc:AlternateContent`, the markup-compatibility escape hatch ISO/IEC
    /// 29500-1 Strict warns on.
    pub fn alternate_content_parts(parts: &[(String, Vec<u8>)]) -> Vec<String> {
        let mut found: Vec<String> = parts.iter().filter(|(path, bytes)| is_xml_part(path) && contains(bytes, "mc:AlternateContent")).map(|(path, _)| path.clone()).collect();
        found.sort();
        found
    }
    //#endregion 🔖️PackageFacts

    //#region 🔖️ConformanceMutations
    /// 🏅️ One artifact's conformance-class coordinates. Each pair is `[transitional, strict]` — the
    /// ISO/IEC 29500-4 value first, the ISO/IEC 29500-1 value second — which is what makes the class
    /// stamp bijective and therefore exactly invertible.
    #[derive(Clone, Copy, Debug)]
    pub struct OoxmlProfile {
        pub format: &'static str,
        pub main_namespaces: [&'static str; 2],
        pub drawing_namespaces: Option<[&'static str; 2]>,
        pub relationship_namespaces: [&'static str; 2],
        pub relationship_bases: [&'static str; 2],
        pub vml_content_type: &'static str,
    }

    fn params_of(spec: &Json) -> Json {
        spec.get("params").cloned().unwrap_or(Json::Null)
    }

    fn text(params: &Json, key: &str) -> String {
        match params.get(key) {
            Some(Json::String(value)) => value.clone(),
            _ => String::new(),
        }
    }

    /// 🧩️ The canonical legacy-VML part body the engine inserts when a mutation does not carry its
    /// own `markup`. Real VML: the `urn:schemas-microsoft-com:vml` namespace on a `v:shape`, which is
    /// exactly what the ✳️strict conformance checkers scan a part's bytes for.
    pub const VML_MARKUP: &str = "<xml xmlns:v=\"urn:schemas-microsoft-com:vml\"><v:shape id=\"legacyShape\" type=\"#_x0000_t202\"/></xml>";

    /// 🧩️ The canonical markup-compatibility fragment the engine appends when a mutation does not
    /// carry its own `markup`.
    pub const ALTERNATE_CONTENT_MARKUP: &str = "<mc:AlternateContent xmlns:mc=\"http://schemas.openxmlformats.org/markup-compatibility/2006\"><mc:Fallback/></mc:AlternateContent>";

    fn markup_or_default(params: &Json, fallback: &str) -> String {
        let carried = text(params, "markup");
        if carried.is_empty() {
            fallback.to_string()
        } else {
            carried
        }
    }

    fn rewrite_namespaces(parts: &mut Vec<(String, Vec<u8>)>, pair: &[&str; 2], to: &str) -> Result<(), String> {
        if to.is_empty() {
            return Err("namespace mutation carries no target namespace".to_string());
        }
        let targets: Vec<(String, Vec<u8>)> = parts.iter().filter(|(path, _)| is_xml_part(path)).cloned().collect();
        for (path, bytes) in targets {
            let rewritten = replace_attribute_values(&bytes, &pair[..], to).map_err(|error| format!("{path}: {error}"))?;
            set_part(parts, &path, rewritten);
        }
        Ok(())
    }

    fn rewrite_relationship_bases(parts: &mut Vec<(String, Vec<u8>)>, pair: &[&str; 2], to: &str) -> Result<(), String> {
        if to.is_empty() {
            return Err("relationship-base mutation carries no target base".to_string());
        }
        let targets: Vec<(String, Vec<u8>)> = parts.iter().filter(|(path, _)| path.ends_with(".rels")).cloned().collect();
        for (path, bytes) in targets {
            let rewritten = replace_attribute_value_prefixes(&bytes, &pair[..], to).map_err(|error| format!("{path}: {error}"))?;
            set_part(parts, &path, rewritten);
        }
        Ok(())
    }

    /// 🏅️ Stamps the WHOLE package into one conformance class: both namespace families, the
    /// `officeDocument` relationship base, and the main part's own `conformance` attribute. Bijective
    /// by construction, so stamping back is an exact inverse.
    pub fn stamp_conformance_class(parts: &mut Vec<(String, Vec<u8>)>, profile: &OoxmlProfile, strict: bool) -> Result<(), String> {
        let index = usize::from(strict);
        rewrite_namespaces(parts, &profile.main_namespaces, profile.main_namespaces[index])?;
        if let Some(pair) = profile.drawing_namespaces {
            rewrite_namespaces(parts, &pair, pair[index])?;
        }
        rewrite_namespaces(parts, &profile.relationship_namespaces, profile.relationship_namespaces[index])?;
        rewrite_relationship_bases(parts, &profile.relationship_bases, profile.relationship_bases[index])?;
        let main = main_part(parts)?;
        let bytes = part_bytes(parts, &main).ok_or_else(|| format!("main part {main} is declared but absent"))?.to_vec();
        let stamped = set_root_attribute(&bytes, "conformance", if strict { Some("strict") } else { None })?;
        set_part(parts, &main, stamped);
        Ok(())
    }

    /// 🦠️ Performs one declared conformance-class mutation with the independent `zip` + `quick-xml`
    /// implementation and returns the re-serialized package. An unrecognised kind is an error, never
    /// a silent no-op: a mutation that is quietly skipped reports as a passing test.
    pub fn apply_conformance_mutation(input: &[u8], spec: &Json, profile: &OoxmlProfile) -> Result<Vec<u8>, String> {
        let params = params_of(spec);
        let kind = spec.str("kind");
        if kind.is_empty() {
            return Err("mutation spec carries no `kind`".to_string());
        }
        if kind == "no-mutation" {
            return Ok(input.to_vec());
        }
        let mut parts = read_parts(input)?;
        match kind.as_str() {
            "set-snapshot" => {
                let class = text(&params, "conformanceClass");
                if class != "strict" && class != "transitional" {
                    return Err(format!("set-snapshot: conformanceClass must be \"strict\" or \"transitional\", got {class:?}"));
                }
                stamp_conformance_class(&mut parts, profile, class == "strict")?;
            }
            "set-main-namespace" => rewrite_namespaces(&mut parts, &profile.main_namespaces, &text(&params, "namespace"))?,
            "set-drawing-namespace" => {
                let pair = profile.drawing_namespaces.ok_or("set-drawing-namespace: this artifact declares no DrawingML namespace pair")?;
                rewrite_namespaces(&mut parts, &pair, &text(&params, "namespace"))?;
            }
            "set-relationships-namespace" => rewrite_namespaces(&mut parts, &profile.relationship_namespaces, &text(&params, "namespace"))?,
            "set-relationship-base" => rewrite_relationship_bases(&mut parts, &profile.relationship_bases, &text(&params, "base"))?,
            "set-conformance-attribute" | "remove-conformance-attribute" => {
                let main = main_part(&parts)?;
                let bytes = part_bytes(&parts, &main).ok_or_else(|| format!("main part {main} is declared but absent"))?.to_vec();
                let value = text(&params, "value");
                let stamped = set_root_attribute(&bytes, "conformance", if kind == "set-conformance-attribute" { Some(value.as_str()) } else { None })?;
                if kind == "remove-conformance-attribute" && stamped == bytes {
                    return Err(format!("remove-conformance-attribute: {main} declares no conformance attribute to remove"));
                }
                set_part(&mut parts, &main, stamped);
            }
            "insert-vml-part" => {
                let path = text(&params, "path");
                if part_bytes(&parts, &path).is_some() {
                    return Err(format!("insert-vml-part: {path} already exists"));
                }
                set_part(&mut parts, &path, markup_or_default(&params, VML_MARKUP).into_bytes());
                set_content_type_override(&mut parts, &path, Some(profile.vml_content_type))?;
            }
            "remove-vml-part" => {
                let path = text(&params, "path");
                if !remove_part(&mut parts, &path) {
                    return Err(format!("remove-vml-part: {path} is not in the package"));
                }
                set_content_type_override(&mut parts, &path, None)?;
            }
            "insert-alternate-content" => {
                let path = text(&params, "path");
                let bytes = part_bytes(&parts, &path).ok_or_else(|| format!("insert-alternate-content: {path} is not in the package"))?.to_vec();
                let appended = append_root_child(&bytes, &markup_or_default(&params, ALTERNATE_CONTENT_MARKUP))?;
                set_part(&mut parts, &path, appended);
            }
            "remove-alternate-content" => {
                let path = text(&params, "path");
                let bytes = part_bytes(&parts, &path).ok_or_else(|| format!("remove-alternate-content: {path} is not in the package"))?.to_vec();
                let stripped = remove_root_children(&bytes, "mc:AlternateContent")?;
                set_part(&mut parts, &path, stripped);
            }
            "set-worksheet-content-type" => {
                let path = text(&params, "path");
                if part_bytes(&parts, &path).is_none() {
                    return Err(format!("set-worksheet-content-type: {path} is not in the package"));
                }
                set_content_type_override(&mut parts, &path, Some(&text(&params, "contentType")))?;
            }
            other => return Err(format!("mutation kind {other:?} has no oracle implementation ({} input byte(s))", input.len())),
        }
        write_parts(&parts)
    }

    /// 🔁️ The reference implementation's own decode/re-encode: read every entry with `zip`, rebuild
    /// the container from those entries alone. Proves the independent container codec is stable on
    /// the real package before the subject's own codec is asked to be.
    pub fn round_trip(input: &[u8]) -> Result<Vec<u8>, String> {
        write_parts(&read_parts(input)?)
    }

    fn json_object(pairs: Vec<(&str, Json)>) -> Json {
        Json::Object(pairs.into_iter().map(|(key, value)| (key.to_string(), value)).collect())
    }

    fn kind_spec(kind: &str, pairs: Vec<(&str, Json)>) -> Json {
        json_object(vec![("kind", Json::String(kind.to_string())), ("params", json_object(pairs))])
    }

    /// 🔎️ Which member of a `[transitional, strict]` namespace pair the package actually declares.
    fn declared_pair_member(parts: &[(String, Vec<u8>)], pair: &[&str; 2]) -> Result<String, String> {
        let declared = declared_namespaces(parts)?;
        pair.iter().find(|candidate| declared.iter().any(|value| value == *candidate)).map(|found| found.to_string()).ok_or_else(|| format!("package declares neither {} nor {}", pair[0], pair[1]))
    }

    /// 🔎️ Which member of a `[transitional, strict]` relationship-base pair the package's own
    /// relationship types are built on.
    fn declared_pair_base(parts: &[(String, Vec<u8>)], pair: &[&str; 2]) -> Result<String, String> {
        let types = relationship_types(parts)?;
        pair.iter().find(|candidate| types.iter().any(|value| value.starts_with(*candidate))).map(|found| found.to_string()).ok_or_else(|| format!("package declares no relationship type under {} or {}", pair[0], pair[1]))
    }

    /// 🎬️ Prepares the input a REMOVAL kind needs a target in. Not one of this repository's real
    /// ECMA-376 packages carries VML, `mc:AlternateContent` or a `conformance` attribute — a fact
    /// about the corpus, recorded here rather than papered over — so the three removal kinds are
    /// exercised on the real package after the SAME independent implementation has inserted their
    /// target. Every other kind reads the committed bytes untouched.
    pub fn conformance_arrange(input: &[u8], forward: &Json, profile: &OoxmlProfile) -> Result<Vec<u8>, String> {
        let params = params_of(forward);
        let path = || Json::String(text(&params, "path"));
        match forward.str("kind").as_str() {
            "remove-conformance-attribute" => apply_conformance_mutation(input, &kind_spec("set-conformance-attribute", vec![("value", Json::String("strict".to_string()))]), profile),
            "remove-vml-part" => apply_conformance_mutation(input, &kind_spec("insert-vml-part", vec![("path", path())]), profile),
            "remove-alternate-content" => apply_conformance_mutation(input, &kind_spec("insert-alternate-content", vec![("path", path())]), profile),
            _ => Ok(input.to_vec()),
        }
    }

    /// ↩️ The undo of `forward`, computed by reading whatever pre-mutation state it needs straight
    /// out of `base` through the SAME independent implementation the mutation runs on — never by
    /// calling this repository's own `Mutation::inverse`, which would defeat the point of an
    /// independently computed oracle. `base` is the mutation's real pre-state, i.e. the output of
    /// [`conformance_arrange`].
    pub fn conformance_inverse_spec(base: &[u8], forward: &Json, profile: &OoxmlProfile) -> Result<Json, String> {
        let params = params_of(forward);
        let parts = read_parts(base)?;
        let path = || Json::String(text(&params, "path"));
        let conformance = || -> Result<Option<String>, String> {
            let main = main_part(&parts)?;
            let bytes = part_bytes(&parts, &main).ok_or_else(|| format!("main part {main} is declared but absent"))?;
            root_attribute(bytes, "conformance")
        };
        Ok(match forward.str("kind").as_str() {
            "no-mutation" => kind_spec("no-mutation", vec![]),
            "set-snapshot" => {
                let forward_class = text(&params, "conformanceClass");
                let back = if forward_class == "strict" { "transitional" } else { "strict" };
                kind_spec("set-snapshot", vec![("conformanceClass", Json::String(back.to_string()))])
            }
            "set-main-namespace" => kind_spec("set-main-namespace", vec![("namespace", Json::String(declared_pair_member(&parts, &profile.main_namespaces)?))]),
            "set-drawing-namespace" => {
                let pair = profile.drawing_namespaces.ok_or("set-drawing-namespace: this artifact declares no DrawingML namespace pair")?;
                kind_spec("set-drawing-namespace", vec![("namespace", Json::String(declared_pair_member(&parts, &pair)?))])
            }
            "set-relationships-namespace" => kind_spec("set-relationships-namespace", vec![("namespace", Json::String(declared_pair_member(&parts, &profile.relationship_namespaces)?))]),
            "set-relationship-base" => kind_spec("set-relationship-base", vec![("base", Json::String(declared_pair_base(&parts, &profile.relationship_bases)?))]),
            "set-conformance-attribute" => match conformance()? {
                Some(value) => kind_spec("set-conformance-attribute", vec![("value", Json::String(value))]),
                None => kind_spec("remove-conformance-attribute", vec![]),
            },
            "remove-conformance-attribute" => match conformance()? {
                Some(value) => kind_spec("set-conformance-attribute", vec![("value", Json::String(value))]),
                None => return Err("remove-conformance-attribute has no inverse: the base declares no conformance attribute".to_string()),
            },
            "insert-vml-part" => kind_spec("remove-vml-part", vec![("path", path())]),
            "remove-vml-part" => kind_spec("insert-vml-part", vec![("path", path())]),
            "insert-alternate-content" => kind_spec("remove-alternate-content", vec![("path", path())]),
            "remove-alternate-content" => kind_spec("insert-alternate-content", vec![("path", path())]),
            "set-worksheet-content-type" => {
                let target = text(&params, "path");
                let (defaults, overrides) = content_types(&parts)?;
                kind_spec("set-worksheet-content-type", vec![("path", Json::String(target.clone())), ("contentType", Json::String(resolve_content_type(&defaults, &overrides, &target)))])
            }
            other => return Err(format!("no inverse rule for kind {other:?}")),
        })
    }

    //#endregion 🔖️ConformanceMutations

    //#region 🔖️Projection
    /// 👁️ The conformance-class projection every `✳️strict`/`✳️transitional` OOXML subset is
    /// compared through: the package's part inventory with resolved content types, the main part's
    /// own root element and attributes, every declared OOXML namespace, every relationship type, and
    /// the legacy-markup inventory. Everything is read back out of the BYTES by this independent
    /// implementation — nothing is carried by the caller.
    pub fn project(input: &[u8], format: &str) -> Result<Json, String> {
        let parts = read_parts(input)?;
        let (defaults, overrides) = content_types(&parts)?;
        let main = main_part(&parts)?;
        let main_bytes = part_bytes(&parts, &main).ok_or_else(|| format!("main part {main} is declared by _rels/.rels but absent from the package"))?;
        let (root_name, mut root_attrs) = root_element(main_bytes)?;
        root_attrs.sort();
        let mut inventory: Vec<(String, String)> = parts.iter().map(|(path, _)| (path.clone(), resolve_content_type(&defaults, &overrides, path))).collect();
        inventory.sort();
        let namespaces = declared_namespaces(&parts)?;
        let relationship_types = relationship_types(&parts)?;
        let alternate_content = alternate_content_parts(&parts);
        let strings = |values: Vec<String>| Json::Array(values.into_iter().map(Json::String).collect());
        Ok(Json::Object(vec![
            ("format".to_string(), Json::String(format.to_string())),
            ("mainPart".to_string(), Json::String(main)),
            ("mainRootName".to_string(), Json::String(root_name)),
            (
                "mainRootAttributes".to_string(),
                Json::Array(root_attrs.into_iter().map(|(name, value)| Json::Object(vec![("name".to_string(), Json::String(name)), ("value".to_string(), Json::String(value))])).collect()),
            ),
            (
                "parts".to_string(),
                Json::Array(inventory.into_iter().map(|(path, content_type)| Json::Object(vec![("path".to_string(), Json::String(path)), ("contentType".to_string(), Json::String(content_type))])).collect()),
            ),
            ("namespaces".to_string(), strings(namespaces)),
            ("relationshipTypes".to_string(), strings(relationship_types)),
            ("alternateContentParts".to_string(), strings(alternate_content)),
        ]))
    }
    //#endregion 🔖️Projection
}
//#endregion 🔖️OoxmlConformanceClass
