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

//#region 🔖️PdfConformanceClass
/// 🏅️ The PDF CONFORMANCE-CLASS engine — one independent `lopdf` 0.44 implementation shared by the
/// six PDF 1.7 subsets that police a conformance CLASS rather than document content: `✳️a`
/// (ISO 19005-2/-3), `✳️e` (ISO 24517-1), `✳️h` (the PDF Healthcare Best Practices Guide), `✳️ua`
/// (ISO 14289-1), `✳️vt` (ISO 16612-2) and `✳️x` (ISO 15930-7).
///
/// It lives in the shared family module rather than in any one subset for the same reason the OOXML
/// engine above does: all six genuinely share the MECHANISM — every axis their conformance checkers
/// read is a fact of the COS object graph (a `/Root` key, an action dictionary's `/S` subtype, a
/// `/FontDescriptor`'s embedded font program, an `/OutputIntent`'s `/S` marker) — and differ only in
/// WHICH axes they police and with which marker. Each subset owns that selection in its own
/// [`PdfConformanceProfile`] and its own `KINDS`, and refuses a kind it does not declare even when
/// this engine could perform it.
///
/// Nothing here interprets page content: pages, media boxes, content streams and `/Info`-as-document
/// metadata are the `✳️any` subset's vocabulary and its own `lopdf` pairing answers for them. This
/// engine reads and writes exactly the object-graph surface a PDF conformance class is defined on.
///
/// 🔓️ ONE deliberate scope note, recorded rather than glossed: `insert-encryption-dictionary` adds a
/// free-standing Standard Security Handler dictionary OBJECT and does not link it from the trailer's
/// `/Encrypt`. That is faithful to what the subsets actually check — every `check_*_conformance`
/// scans `snapshot.objects` for the `/Filter /Standard` + `/V` + `/R` + `/O` + `/U` shape and never
/// reads the trailer — and it is the only form the mutation can take and still leave a document both
/// producers can re-read, since a genuinely `/Encrypt`-linked trailer makes every string and stream
/// in the file ciphertext.
#[cfg(feature = "oracles")]
pub mod pdf_conformance {
    use lopdf::{Dictionary, Document, Object, ObjectId, Stream};
    use semio_repo_test_host::Json;

    //#region 🔖️Profile
    /// 🏅️ One subset's conformance coordinates: which marker its OutputIntent axis demands, whether
    /// that intent must carry a `/DestOutputProfile`, and exactly which axes its own checker reads.
    /// `axes` is both the projection's field list and the stamp recipe `set-snapshot` executes, so a
    /// subset can never project an axis its checker does not read, nor stamp one it does not own.
    pub struct PdfConformanceProfile {
        pub subset: &'static str,
        pub output_intent_subtype: &'static str,
        pub output_intent_dest_profile: bool,
        pub axes: &'static [&'static str],
    }
    //#endregion 🔖️Profile

    //#region 🔖️Spec
    fn params_of(spec: &Json) -> Json {
        spec.get("params").cloned().unwrap_or(Json::Null)
    }

    fn number(params: &Json, key: &str) -> Option<f64> {
        match params.get(key) {
            Some(Json::Number(value)) => Some(*value),
            _ => None,
        }
    }

    fn ordinal(params: &Json, key: &str) -> Result<usize, String> {
        number(params, key).map(|value| value.max(0.0) as usize).ok_or_else(|| format!("parameter `{key}` is required and must be a number"))
    }

    fn required(params: &Json, key: &str) -> Result<String, String> {
        match params.get(key) {
            Some(Json::String(value)) if !value.is_empty() => Ok(value.clone()),
            _ => Err(format!("parameter `{key}` is required and must be a non-empty string")),
        }
    }

    fn json_object(pairs: Vec<(&str, Json)>) -> Json {
        Json::Object(pairs.into_iter().map(|(key, value)| (key.to_string(), value)).collect())
    }

    fn kind_spec(kind: &str, pairs: Vec<(&str, Json)>) -> Json {
        json_object(vec![("kind", Json::String(kind.to_string())), ("params", json_object(pairs))])
    }

    fn optional_string(value: Option<String>) -> Json {
        value.map(Json::String).unwrap_or(Json::Null)
    }

    fn number_array(values: &[f32]) -> Json {
        Json::Array(values.iter().map(|value| Json::Number(*value as f64)).collect())
    }
    //#endregion 🔖️Spec

    //#region 🔖️Container
    fn load(input: &[u8]) -> Result<Document, String> {
        Document::load_mem(input).map_err(|error| format!("independent PDF reader could not parse the document: {error}"))
    }

    fn save(document: &mut Document) -> Result<Vec<u8>, String> {
        let mut out = Vec::new();
        document.save_to(&mut out).map_err(|error| format!("independent PDF writer could not save: {error}"))?;
        Ok(out)
    }

    /// 🔁️ The reference implementation's own decode/re-encode: `lopdf` parses the whole object
    /// graph and writes a fresh file from that graph alone.
    pub fn round_trip(input: &[u8]) -> Result<Vec<u8>, String> {
        save(&mut load(input)?)
    }

    fn catalog_id(document: &Document) -> Result<ObjectId, String> {
        document.trailer.get(b"Root").and_then(Object::as_reference).map_err(|error| format!("the trailer carries no /Root reference: {error}"))
    }

    fn catalog_dict(document: &Document) -> Result<&Dictionary, String> {
        let id = catalog_id(document)?;
        document.get_dictionary(id).map_err(|error| format!("/Root does not resolve to a dictionary: {error}"))
    }

    fn catalog_dict_mut(document: &mut Document) -> Result<&mut Dictionary, String> {
        let id = catalog_id(document)?;
        document.get_dictionary_mut(id).map_err(|error| format!("/Root does not resolve to a dictionary: {error}"))
    }

    fn name_at(dict: &Dictionary, key: &str) -> Option<String> {
        dict.get(key.as_bytes()).ok().and_then(|value| value.as_name().ok()).map(|bytes| String::from_utf8_lossy(bytes).into_owned())
    }

    fn text_at(dict: &Dictionary, key: &str) -> Option<String> {
        dict.get(key.as_bytes()).ok().and_then(|value| value.as_str().ok()).map(|bytes| String::from_utf8_lossy(bytes).into_owned())
    }

    fn deref<'d>(document: &'d Document, value: &'d Object) -> Option<&'d Object> {
        match value {
            Object::Reference(id) => document.get_object(*id).ok(),
            other => Some(other),
        }
    }

    fn deref_dict<'d>(document: &'d Document, value: &'d Object) -> Option<&'d Dictionary> {
        deref(document, value).and_then(|resolved| resolved.as_dict().ok())
    }
    //#endregion 🔖️Container

    //#region 🔖️Scans
    /// 🔒️ Every object with the Standard Security Handler shape the conformance checkers scan for.
    fn encryption_dictionaries(document: &Document) -> Vec<(ObjectId, i64, i64)> {
        document
            .objects
            .iter()
            .filter_map(|(id, object)| {
                let dict = object.as_dict().ok()?;
                if name_at(dict, "Filter").as_deref() != Some("Standard") {
                    return None;
                }
                let version = dict.get(b"V").ok()?.as_i64().ok()?;
                let revision = dict.get(b"R").ok()?.as_i64().ok()?;
                if !dict.has(b"O") || !dict.has(b"U") {
                    return None;
                }
                Some((*id, version, revision))
            })
            .collect()
    }

    /// 📜️ Every action dictionary with the given `/S` subtype, paired with the payload the subset's
    /// own diagnostics quote — `/JS` for JavaScript, `/F` for Launch.
    fn actions(document: &Document, subtype: &str, payload_key: &str) -> Vec<(ObjectId, String)> {
        document
            .objects
            .iter()
            .filter_map(|(id, object)| {
                let dict = object.as_dict().ok()?;
                if name_at(dict, "S").as_deref() != Some(subtype) {
                    return None;
                }
                Some((*id, text_at(dict, payload_key).unwrap_or_default()))
            })
            .collect()
    }

    /// 🎬️ Every `/Subtype /Movie` or `/Subtype /Sound` annotation. `/Subtype /3D` is a distinct,
    /// explicitly allowed name and never matches — the same distinction `check_e_conformance` draws.
    fn media_annotations(document: &Document) -> Vec<(ObjectId, String, String)> {
        document
            .objects
            .iter()
            .filter_map(|(id, object)| {
                let dict = object.as_dict().ok()?;
                let subtype = name_at(dict, "Subtype")?;
                if subtype != "Movie" && subtype != "Sound" {
                    return None;
                }
                Some((*id, subtype, text_at(dict, "T").unwrap_or_default()))
            })
            .collect()
    }

    /// 📎️ Every `/Type /Filespec` object, with the file name, whether it carries an `/EF` attached
    /// stream, and its `/AFRelationship` if any — the exact triple PDF/A's embedded-file axis reads.
    fn file_specs(document: &Document) -> Vec<(ObjectId, String, bool, Option<String>)> {
        document
            .objects
            .iter()
            .filter_map(|(id, object)| {
                let dict = object.as_dict().ok()?;
                if name_at(dict, "Type").as_deref() != Some("Filespec") {
                    return None;
                }
                let file_name = text_at(dict, "F").or_else(|| text_at(dict, "UF")).unwrap_or_default();
                Some((*id, file_name, dict.has(b"EF"), name_at(dict, "AFRelationship")))
            })
            .collect()
    }

    /// 🏳️ Every intent reachable from `/Root/OutputIntents`, with its `/S` marker, its output
    /// condition identifier and whether it carries a `/DestOutputProfile`.
    fn output_intents(document: &Document) -> Vec<(String, String, bool)> {
        let Ok(catalog) = catalog_dict(document) else { return Vec::new() };
        let Ok(intents) = catalog.get(b"OutputIntents") else { return Vec::new() };
        let Some(items) = deref(document, intents).and_then(|value| value.as_array().ok()) else { return Vec::new() };
        items
            .iter()
            .filter_map(|item| {
                let dict = deref_dict(document, item)?;
                Some((name_at(dict, "S").unwrap_or_default(), text_at(dict, "OutputConditionIdentifier").unwrap_or_default(), dict.has(b"DestOutputProfile")))
            })
            .collect()
    }

    const FONT_PROGRAM_KEYS: [&str; 3] = ["FontFile", "FontFile2", "FontFile3"];

    /// 🔤️ Every `/Type /FontDescriptor` object, in object-number order — a stable ordinal space no
    /// mutation in this vocabulary adds to or removes from, which is what lets a scenario address
    /// one by ordinal instead of by an object number nobody can read off a feature file.
    fn font_descriptors(document: &Document) -> Vec<ObjectId> {
        document.objects.iter().filter(|(_, object)| object.as_dict().map(|dict| name_at(dict, "Type").as_deref() == Some("FontDescriptor")).unwrap_or(false)).map(|(id, _)| *id).collect()
    }

    /// 🔤️ The embedded font program a descriptor points at: which of the three keys carries it, the
    /// object it references, and that object's stream length.
    fn font_program(document: &Document, descriptor: ObjectId) -> Option<(String, ObjectId, usize)> {
        let dict = document.get_dictionary(descriptor).ok()?;
        for key in FONT_PROGRAM_KEYS {
            if let Ok(value) = dict.get(key.as_bytes()) {
                let id = value.as_reference().ok()?;
                let size = document.get_object(id).ok().and_then(|object| object.as_stream().ok()).map(|stream| stream.content.len()).unwrap_or(0);
                return Some((key.to_string(), id, size));
            }
        }
        None
    }

    /// 🔤️ Every distinct font-program object currently referenced by any descriptor, in
    /// object-number order — the ordinal space `embed-font-file` names its donor program in.
    fn font_programs(document: &Document) -> Vec<ObjectId> {
        let mut ids: Vec<ObjectId> = font_descriptors(document).into_iter().filter_map(|descriptor| font_program(document, descriptor).map(|(_, id, _)| id)).collect();
        ids.sort();
        ids.dedup();
        ids
    }

    fn page_at(document: &Document, index: usize) -> Result<ObjectId, String> {
        document.get_pages().get(&(index as u32 + 1)).copied().ok_or_else(|| format!("page index {index} is out of range"))
    }

    fn box_of(document: &Document, page: ObjectId, key: &str) -> Option<Vec<f32>> {
        let dict = document.get_dictionary(page).ok()?;
        let value = dict.get(key.as_bytes()).ok()?;
        let items = deref(document, value)?.as_array().ok()?;
        Some(items.iter().map(|item| item.as_float().unwrap_or(0.0)).collect())
    }

    /// ✍️ Every `/AcroForm` field whose `/FT` is `/Sig`, by its `/T` title — the signature-flow axis
    /// the PDF Healthcare guide's checker reads.
    fn signature_fields(document: &Document) -> Vec<(ObjectId, String)> {
        let Ok(catalog) = catalog_dict(document) else { return Vec::new() };
        let Ok(acro_form) = catalog.get(b"AcroForm") else { return Vec::new() };
        let Some(form) = deref_dict(document, acro_form) else { return Vec::new() };
        let Some(fields) = form.get(b"Fields").ok().and_then(|value| deref(document, value)).and_then(|value| value.as_array().ok()) else { return Vec::new() };
        fields
            .iter()
            .filter_map(|item| {
                let id = item.as_reference().ok()?;
                let dict = document.get_dictionary(id).ok()?;
                if name_at(dict, "FT").as_deref() != Some("Sig") {
                    return None;
                }
                Some((id, text_at(dict, "T").unwrap_or_default()))
            })
            .collect()
    }

    /// 🗂️ The `/DPartRoot` node chain: its root node and, per node, whether it carries `/DPM` and
    /// what that metadata's `/Job` entry says.
    fn dpart_nodes(document: &Document) -> Option<(ObjectId, Vec<(ObjectId, Option<String>)>)> {
        let catalog = catalog_dict(document).ok()?;
        let root_id = catalog.get(b"DPartRoot").ok()?.as_reference().ok()?;
        let root = document.get_dictionary(root_id).ok()?;
        let node_id = root.get(b"DPartRootNode").ok()?.as_reference().ok()?;
        let node = document.get_dictionary(node_id).ok()?;
        let metadata = node.get(b"DPM").ok().and_then(|value| deref_dict(document, value)).and_then(|dict| text_at(dict, "Job"));
        Some((root_id, vec![(node_id, metadata)]))
    }

    fn info_id(document: &Document) -> Option<ObjectId> {
        document.trailer.get(b"Info").ok().and_then(|value| value.as_reference().ok())
    }

    fn info_entry(document: &Document, key: &str) -> Option<String> {
        let id = info_id(document)?;
        let dict = document.get_dictionary(id).ok()?;
        text_at(dict, key)
    }

    fn set_info_entry(document: &mut Document, key: &str, value: &str) -> Result<(), String> {
        match info_id(document) {
            Some(id) => {
                let dict = document.get_dictionary_mut(id).map_err(|error| format!("/Info does not resolve to a dictionary: {error}"))?;
                dict.set(key, Object::string_literal(value));
            }
            None => {
                let mut dict = Dictionary::new();
                dict.set(key, Object::string_literal(value));
                let id = document.add_object(Object::Dictionary(dict));
                document.trailer.set("Info", Object::Reference(id));
            }
        }
        Ok(())
    }
    //#endregion 🔖️Scans

    //#region 🔖️Builders
    /// 🔒️ A real Standard Security Handler dictionary — the `/Filter`/`/V`/`/R`/`/O`/`/U` shape the
    /// checkers match, with 32-byte owner and user strings of the length ISO 32000-1 §7.6.3.3 fixes.
    fn encryption_dictionary(version: i64, revision: i64) -> Object {
        let mut dict = Dictionary::new();
        dict.set("Filter", Object::Name(b"Standard".to_vec()));
        dict.set("V", Object::Integer(version));
        dict.set("R", Object::Integer(revision));
        dict.set("O", Object::string_literal(vec![0x4fu8; 32]));
        dict.set("U", Object::string_literal(vec![0x55u8; 32]));
        dict.set("P", Object::Integer(-1));
        dict.set("Length", Object::Integer(128));
        Object::Dictionary(dict)
    }

    fn javascript_action(script: &str) -> Object {
        let mut dict = Dictionary::new();
        dict.set("Type", Object::Name(b"Action".to_vec()));
        dict.set("S", Object::Name(b"JavaScript".to_vec()));
        dict.set("JS", Object::string_literal(script));
        Object::Dictionary(dict)
    }

    fn launch_action(target: &str) -> Object {
        let mut dict = Dictionary::new();
        dict.set("Type", Object::Name(b"Action".to_vec()));
        dict.set("S", Object::Name(b"Launch".to_vec()));
        dict.set("F", Object::string_literal(target));
        Object::Dictionary(dict)
    }

    fn media_annotation(subtype: &str, title: &str) -> Object {
        let mut dict = Dictionary::new();
        dict.set("Type", Object::Name(b"Annot".to_vec()));
        dict.set("Subtype", Object::Name(subtype.as_bytes().to_vec()));
        dict.set("T", Object::string_literal(title));
        dict.set("Rect", Object::Array(vec![Object::Integer(0), Object::Integer(0), Object::Integer(144), Object::Integer(96)]));
        Object::Dictionary(dict)
    }
    //#endregion 🔖️Builders

    //#region 🔖️Forward
    /// 🦠️ Performs one declared conformance-class mutation with the independent `lopdf`
    /// implementation and returns the re-serialized document. An unrecognised kind is an error,
    /// never a silent no-op: a mutation that is quietly skipped reports as a passing test.
    pub fn apply_conformance_mutation(input: &[u8], spec: &Json, profile: &PdfConformanceProfile) -> Result<Vec<u8>, String> {
        let kind = spec.str("kind");
        if kind.is_empty() {
            return Err("mutation spec carries no `kind`".to_string());
        }
        if kind == "no-mutation" {
            return Ok(input.to_vec());
        }
        let mut document = load(input)?;
        apply_in_place(&mut document, &kind, &params_of(spec), profile)?;
        save(&mut document)
    }

    fn apply_in_place(document: &mut Document, kind: &str, params: &Json, profile: &PdfConformanceProfile) -> Result<(), String> {
        match kind {
            "no-mutation" => Ok(()),
            "set-snapshot" => {
                let conformance = required(params, "conformance")?;
                match conformance.as_str() {
                    "stamped" => stamp(document, profile, true),
                    "stripped" => stamp(document, profile, false),
                    other => Err(format!("set-snapshot: `conformance` must be \"stamped\" or \"stripped\", got {other:?}")),
                }
            }
            "insert-encryption-dictionary" => {
                let version = number(params, "version").unwrap_or(2.0) as i64;
                let revision = number(params, "revision").unwrap_or(3.0) as i64;
                if encryption_dictionaries(document).iter().any(|(_, v, r)| *v == version && *r == revision) {
                    return Err(format!("insert-encryption-dictionary: the document already carries a /V {version} /R {revision} Standard Security Handler dictionary"));
                }
                document.add_object(encryption_dictionary(version, revision));
                Ok(())
            }
            "remove-encryption-dictionary" => {
                let version = number(params, "version").unwrap_or(2.0) as i64;
                let revision = number(params, "revision").unwrap_or(3.0) as i64;
                let target = encryption_dictionaries(document).into_iter().find(|(_, v, r)| *v == version && *r == revision).map(|(id, _, _)| id);
                match target {
                    Some(id) => {
                        document.objects.remove(&id);
                        Ok(())
                    }
                    None => Err(format!("remove-encryption-dictionary: no /V {version} /R {revision} Standard Security Handler dictionary is present")),
                }
            }
            "insert-javascript-action" => {
                let script = required(params, "script")?;
                if actions(document, "JavaScript", "JS").iter().any(|(_, payload)| payload == &script) {
                    return Err(format!("insert-javascript-action: an /S /JavaScript action carrying {script:?} is already present"));
                }
                document.add_object(javascript_action(&script));
                Ok(())
            }
            "remove-javascript-action" => {
                let script = required(params, "script")?;
                match actions(document, "JavaScript", "JS").into_iter().find(|(_, payload)| payload == &script) {
                    Some((id, _)) => {
                        document.objects.remove(&id);
                        Ok(())
                    }
                    None => Err(format!("remove-javascript-action: no /S /JavaScript action carrying {script:?} is present")),
                }
            }
            "insert-launch-action" => {
                let target = required(params, "target")?;
                if actions(document, "Launch", "F").iter().any(|(_, payload)| payload == &target) {
                    return Err(format!("insert-launch-action: an /S /Launch action targeting {target:?} is already present"));
                }
                document.add_object(launch_action(&target));
                Ok(())
            }
            "remove-launch-action" => {
                let target = required(params, "target")?;
                match actions(document, "Launch", "F").into_iter().find(|(_, payload)| payload == &target) {
                    Some((id, _)) => {
                        document.objects.remove(&id);
                        Ok(())
                    }
                    None => Err(format!("remove-launch-action: no /S /Launch action targeting {target:?} is present")),
                }
            }
            "insert-media-annotation" => {
                let subtype = required(params, "subtype")?;
                if subtype != "Movie" && subtype != "Sound" {
                    return Err(format!("insert-media-annotation: `subtype` must be \"Movie\" or \"Sound\", got {subtype:?}"));
                }
                let title = required(params, "title")?;
                if media_annotations(document).iter().any(|(_, kind, name)| kind == &subtype && name == &title) {
                    return Err(format!("insert-media-annotation: a /Subtype /{subtype} annotation titled {title:?} is already present"));
                }
                document.add_object(media_annotation(&subtype, &title));
                Ok(())
            }
            "remove-media-annotation" => {
                let subtype = required(params, "subtype")?;
                let title = required(params, "title")?;
                match media_annotations(document).into_iter().find(|(_, kind, name)| kind == &subtype && name == &title) {
                    Some((id, _, _)) => {
                        document.objects.remove(&id);
                        Ok(())
                    }
                    None => Err(format!("remove-media-annotation: no /Subtype /{subtype} annotation titled {title:?} is present")),
                }
            }
            "insert-embedded-file" => {
                let file_name = required(params, "fileName")?;
                if file_specs(document).iter().any(|(_, name, _, _)| name == &file_name) {
                    return Err(format!("insert-embedded-file: a /Type /Filespec for {file_name:?} is already present"));
                }
                let payload = document.add_object(Object::Stream(Stream::new(Dictionary::new(), format!("attached payload for {file_name}").into_bytes())));
                let mut embedded = Dictionary::new();
                embedded.set("F", Object::Reference(payload));
                let mut spec = Dictionary::new();
                spec.set("Type", Object::Name(b"Filespec".to_vec()));
                spec.set("F", Object::string_literal(file_name.as_str()));
                spec.set("UF", Object::string_literal(file_name.as_str()));
                spec.set("EF", Object::Dictionary(embedded));
                document.add_object(Object::Dictionary(spec));
                Ok(())
            }
            "remove-embedded-file" => {
                let file_name = required(params, "fileName")?;
                match file_specs(document).into_iter().find(|(_, name, _, _)| name == &file_name) {
                    Some((id, _, _, _)) => {
                        document.objects.remove(&id);
                        Ok(())
                    }
                    None => Err(format!("remove-embedded-file: no /Type /Filespec for {file_name:?} is present")),
                }
            }
            "set-af-relationship" => {
                let file_name = required(params, "fileName")?;
                let relationship = required(params, "relationship")?;
                let target = file_specs(document).into_iter().find(|(_, name, _, _)| name == &file_name).map(|(id, _, _, _)| id);
                match target {
                    Some(id) => {
                        let dict = document.get_dictionary_mut(id).map_err(|error| format!("set-af-relationship: {error}"))?;
                        dict.set("AFRelationship", Object::Name(relationship.into_bytes()));
                        Ok(())
                    }
                    None => Err(format!("set-af-relationship: no /Type /Filespec for {file_name:?} is present")),
                }
            }
            "remove-af-relationship" => {
                let file_name = required(params, "fileName")?;
                let target = file_specs(document).into_iter().find(|(_, name, _, _)| name == &file_name);
                match target {
                    Some((id, _, _, None)) => Err(format!("remove-af-relationship: the /Type /Filespec for {file_name:?} carries no /AFRelationship to remove ({id:?})")),
                    Some((id, _, _, Some(_))) => {
                        let dict = document.get_dictionary_mut(id).map_err(|error| format!("remove-af-relationship: {error}"))?;
                        dict.remove(b"AFRelationship");
                        Ok(())
                    }
                    None => Err(format!("remove-af-relationship: no /Type /Filespec for {file_name:?} is present")),
                }
            }
            "set-output-intent" => {
                let identifier = required(params, "identifier")?;
                if output_intents(document).iter().any(|(subtype, _, _)| subtype == profile.output_intent_subtype) {
                    return Err(format!("set-output-intent: an intent with /S /{} is already reachable from /Root/OutputIntents", profile.output_intent_subtype));
                }
                let mut intent = Dictionary::new();
                intent.set("Type", Object::Name(b"OutputIntent".to_vec()));
                intent.set("S", Object::Name(profile.output_intent_subtype.as_bytes().to_vec()));
                intent.set("OutputConditionIdentifier", Object::string_literal(identifier.as_str()));
                intent.set("Info", Object::string_literal(identifier.as_str()));
                if profile.output_intent_dest_profile {
                    let mut stream_dict = Dictionary::new();
                    stream_dict.set("N", Object::Integer(3));
                    let profile_id = document.add_object(Object::Stream(Stream::new(stream_dict, format!("ICC destination output profile for {identifier}").into_bytes())));
                    intent.set("DestOutputProfile", Object::Reference(profile_id));
                }
                let intent_id = document.add_object(Object::Dictionary(intent));
                let catalog = catalog_dict_mut(document)?;
                catalog.set("OutputIntents", Object::Array(vec![Object::Reference(intent_id)]));
                Ok(())
            }
            "remove-output-intent" => {
                if output_intents(document).is_empty() {
                    return Err("remove-output-intent: /Root carries no OutputIntents to remove".to_string());
                }
                let catalog = catalog_dict_mut(document)?;
                catalog.remove(b"OutputIntents");
                Ok(())
            }
            "embed-font-file" => {
                let descriptor_ordinal = ordinal(params, "descriptorOrdinal")?;
                let key = required(params, "key")?;
                if !FONT_PROGRAM_KEYS.contains(&key.as_str()) {
                    return Err(format!("embed-font-file: `key` must be one of {FONT_PROGRAM_KEYS:?}, got {key:?}"));
                }
                let descriptors = font_descriptors(document);
                let descriptor = *descriptors.get(descriptor_ordinal).ok_or_else(|| format!("embed-font-file: descriptor ordinal {descriptor_ordinal} is out of range ({} descriptors)", descriptors.len()))?;
                if font_program(document, descriptor).is_some() {
                    return Err(format!("embed-font-file: descriptor ordinal {descriptor_ordinal} already carries an embedded font program"));
                }
                let program = program_reference(document, params)?;
                let dict = document.get_dictionary_mut(descriptor).map_err(|error| format!("embed-font-file: {error}"))?;
                dict.set(key, Object::Reference(program));
                Ok(())
            }
            "remove-font-file" => {
                let descriptor_ordinal = ordinal(params, "descriptorOrdinal")?;
                let descriptors = font_descriptors(document);
                let descriptor = *descriptors.get(descriptor_ordinal).ok_or_else(|| format!("remove-font-file: descriptor ordinal {descriptor_ordinal} is out of range ({} descriptors)", descriptors.len()))?;
                let (key, _, _) = font_program(document, descriptor).ok_or_else(|| format!("remove-font-file: descriptor ordinal {descriptor_ordinal} carries no embedded font program"))?;
                let dict = document.get_dictionary_mut(descriptor).map_err(|error| format!("remove-font-file: {error}"))?;
                dict.remove(key.as_bytes());
                Ok(())
            }
            "set-trim-box" => {
                let page_index = ordinal(params, "pageIndex")?;
                let values = params.array("trimBox");
                if values.len() != 4 {
                    return Err("set-trim-box: `trimBox` must be an array of four numbers".to_string());
                }
                let page = page_at(document, page_index)?;
                let entries: Vec<Object> = values
                    .iter()
                    .map(|value| match value {
                        Json::Number(number) => Object::Real(*number as f32),
                        _ => Object::Real(0.0),
                    })
                    .collect();
                let dict = document.get_dictionary_mut(page).map_err(|error| format!("set-trim-box: {error}"))?;
                dict.set("TrimBox", Object::Array(entries));
                Ok(())
            }
            "remove-trim-box" => {
                let page_index = ordinal(params, "pageIndex")?;
                let page = page_at(document, page_index)?;
                if box_of(document, page, "TrimBox").is_none() {
                    return Err(format!("remove-trim-box: page {page_index} carries no /TrimBox"));
                }
                let dict = document.get_dictionary_mut(page).map_err(|error| format!("remove-trim-box: {error}"))?;
                dict.remove(b"TrimBox");
                Ok(())
            }
            "set-mark-info" => {
                let marked = matches!(params.get("marked"), Some(Json::Bool(true)));
                let mut mark_info = Dictionary::new();
                mark_info.set("Marked", Object::Boolean(marked));
                let catalog = catalog_dict_mut(document)?;
                catalog.set("MarkInfo", Object::Dictionary(mark_info));
                Ok(())
            }
            "remove-mark-info" => {
                let catalog = catalog_dict_mut(document)?;
                if catalog.remove(b"MarkInfo").is_none() {
                    return Err("remove-mark-info: /Root carries no /MarkInfo".to_string());
                }
                Ok(())
            }
            "set-struct-tree-root" => {
                let mut root = Dictionary::new();
                root.set("Type", Object::Name(b"StructTreeRoot".to_vec()));
                root.set("K", Object::Array(Vec::new()));
                let id = document.add_object(Object::Dictionary(root));
                let catalog = catalog_dict_mut(document)?;
                if catalog.has(b"StructTreeRoot") {
                    return Err("set-struct-tree-root: /Root already carries a /StructTreeRoot".to_string());
                }
                catalog.set("StructTreeRoot", Object::Reference(id));
                Ok(())
            }
            "remove-struct-tree-root" => {
                let catalog = catalog_dict_mut(document)?;
                if catalog.remove(b"StructTreeRoot").is_none() {
                    return Err("remove-struct-tree-root: /Root carries no /StructTreeRoot".to_string());
                }
                Ok(())
            }
            "set-lang" => {
                let lang = required(params, "lang")?;
                let catalog = catalog_dict_mut(document)?;
                catalog.set("Lang", Object::string_literal(lang));
                Ok(())
            }
            "remove-lang" => {
                let catalog = catalog_dict_mut(document)?;
                if catalog.remove(b"Lang").is_none() {
                    return Err("remove-lang: /Root carries no /Lang".to_string());
                }
                Ok(())
            }
            "set-display-doc-title" => {
                let display = matches!(params.get("displayDocTitle"), Some(Json::Bool(true)));
                let mut preferences = Dictionary::new();
                preferences.set("DisplayDocTitle", Object::Boolean(display));
                let catalog = catalog_dict_mut(document)?;
                catalog.set("ViewerPreferences", Object::Dictionary(preferences));
                Ok(())
            }
            "remove-display-doc-title" => {
                let catalog = catalog_dict_mut(document)?;
                if catalog.remove(b"ViewerPreferences").is_none() {
                    return Err("remove-display-doc-title: /Root carries no /ViewerPreferences".to_string());
                }
                Ok(())
            }
            "set-info-title" => set_info_entry(document, "Title", &params.str("title")),
            "set-info-author" => set_info_entry(document, "Author", &params.str("author")),
            "insert-signature-field" => {
                let name = required(params, "name")?;
                if signature_fields(document).iter().any(|(_, title)| title == &name) {
                    return Err(format!("insert-signature-field: a /FT /Sig field titled {name:?} is already present"));
                }
                let mut field = Dictionary::new();
                field.set("FT", Object::Name(b"Sig".to_vec()));
                field.set("T", Object::string_literal(name.as_str()));
                let field_id = document.add_object(Object::Dictionary(field));
                let existing: Vec<Object> = signature_fields(document).into_iter().map(|(id, _)| Object::Reference(id)).collect();
                let mut fields = existing;
                fields.push(Object::Reference(field_id));
                let mut form = Dictionary::new();
                form.set("Fields", Object::Array(fields));
                let catalog = catalog_dict_mut(document)?;
                catalog.set("AcroForm", Object::Dictionary(form));
                Ok(())
            }
            "remove-signature-field" => {
                let name = required(params, "name")?;
                let target = signature_fields(document).into_iter().find(|(_, title)| title == &name).map(|(id, _)| id);
                let Some(field_id) = target else { return Err(format!("remove-signature-field: no /FT /Sig field titled {name:?} is present")) };
                document.objects.remove(&field_id);
                let remaining: Vec<Object> = signature_fields(document).into_iter().map(|(id, _)| Object::Reference(id)).collect();
                let catalog = catalog_dict_mut(document)?;
                if remaining.is_empty() {
                    catalog.remove(b"AcroForm");
                } else {
                    let mut form = Dictionary::new();
                    form.set("Fields", Object::Array(remaining));
                    catalog.set("AcroForm", Object::Dictionary(form));
                }
                Ok(())
            }
            "set-dpart-root" => {
                if dpart_nodes(document).is_some() {
                    return Err("set-dpart-root: /Root already carries a /DPartRoot".to_string());
                }
                let job = params.str("job");
                let mut node = Dictionary::new();
                node.set("Type", Object::Name(b"DPart".to_vec()));
                if !job.is_empty() {
                    let mut metadata = Dictionary::new();
                    metadata.set("Job", Object::string_literal(job.as_str()));
                    node.set("DPM", Object::Dictionary(metadata));
                }
                let node_id = document.add_object(Object::Dictionary(node));
                let mut root = Dictionary::new();
                root.set("Type", Object::Name(b"DPartRoot".to_vec()));
                root.set("DPartRootNode", Object::Reference(node_id));
                let root_id = document.add_object(Object::Dictionary(root));
                let catalog = catalog_dict_mut(document)?;
                catalog.set("DPartRoot", Object::Reference(root_id));
                Ok(())
            }
            "remove-dpart-root" => {
                if dpart_nodes(document).is_none() {
                    return Err("remove-dpart-root: /Root carries no /DPartRoot".to_string());
                }
                let catalog = catalog_dict_mut(document)?;
                catalog.remove(b"DPartRoot");
                Ok(())
            }
            "set-dpart-metadata" => {
                let job = required(params, "job")?;
                let (_, nodes) = dpart_nodes(document).ok_or("set-dpart-metadata: /Root carries no /DPartRoot")?;
                let (node_id, _) = nodes[0];
                let mut metadata = Dictionary::new();
                metadata.set("Job", Object::string_literal(job.as_str()));
                let dict = document.get_dictionary_mut(node_id).map_err(|error| format!("set-dpart-metadata: {error}"))?;
                dict.set("DPM", Object::Dictionary(metadata));
                Ok(())
            }
            "remove-dpart-metadata" => {
                let (_, nodes) = dpart_nodes(document).ok_or("remove-dpart-metadata: /Root carries no /DPartRoot")?;
                let (node_id, metadata) = nodes[0].clone();
                if metadata.is_none() {
                    return Err("remove-dpart-metadata: the /DPart node carries no /DPM to remove".to_string());
                }
                let dict = document.get_dictionary_mut(node_id).map_err(|error| format!("remove-dpart-metadata: {error}"))?;
                dict.remove(b"DPM");
                Ok(())
            }
            other => Err(format!("mutation kind {other:?} has no oracle implementation")),
        }
    }

    /// 🔤️ The donor font-program object `embed-font-file` points a descriptor at, named either
    /// exactly (`program: {num, gen}`, what an engine-computed inverse carries) or by ordinal into
    /// the document's own program list (`programOrdinal`, what a feature row can actually author).
    fn program_reference(document: &Document, params: &Json) -> Result<ObjectId, String> {
        if let Some(reference) = params.get("program") {
            let num = number(reference, "num").ok_or("embed-font-file: `program.num` must be a number")? as u32;
            let generation = number(reference, "gen").unwrap_or(0.0) as u16;
            return Ok((num, generation));
        }
        let programs = font_programs(document);
        let index = ordinal(params, "programOrdinal")?;
        programs.get(index).copied().ok_or_else(|| format!("embed-font-file: program ordinal {index} is out of range ({} font programs)", programs.len()))
    }

    /// 🏅️ Stamps every axis this profile OWNS into (or out of) its conformant state. Only the axes
    /// whose conformant state is the PRESENCE of something are stamped: an axis whose conformant
    /// state is the ABSENCE of a forbidden construct (encryption, JavaScript, launch actions, media
    /// annotations, unrelated embedded files) is already conformant on any document that does not
    /// carry it, and adding one to be able to remove it again would be theatre, not a stamp.
    fn stamp(document: &mut Document, profile: &PdfConformanceProfile, stamped: bool) -> Result<(), String> {
        for axis in profile.axes {
            match *axis {
                "outputIntents" => {
                    let present = !output_intents(document).is_empty();
                    if stamped && !present {
                        apply_in_place(document, "set-output-intent", &json_object(vec![("identifier", Json::String("sRGB IEC61966-2.1".to_string()))]), profile)?;
                    } else if !stamped && present {
                        apply_in_place(document, "remove-output-intent", &Json::Null, profile)?;
                    }
                }
                "markInfo" => {
                    let present = catalog_dict(document).map(|catalog| catalog.has(b"MarkInfo")).unwrap_or(false);
                    if stamped {
                        apply_in_place(document, "set-mark-info", &json_object(vec![("marked", Json::Bool(true))]), profile)?;
                    } else if present {
                        apply_in_place(document, "remove-mark-info", &Json::Null, profile)?;
                    }
                }
                "structTreeRoot" => {
                    let present = catalog_dict(document).map(|catalog| catalog.has(b"StructTreeRoot")).unwrap_or(false);
                    if stamped && !present {
                        apply_in_place(document, "set-struct-tree-root", &Json::Null, profile)?;
                    } else if !stamped && present {
                        apply_in_place(document, "remove-struct-tree-root", &Json::Null, profile)?;
                    }
                }
                "lang" => {
                    let present = catalog_dict(document).map(|catalog| catalog.has(b"Lang")).unwrap_or(false);
                    if stamped {
                        apply_in_place(document, "set-lang", &json_object(vec![("lang", Json::String("en-GB".to_string()))]), profile)?;
                    } else if present {
                        apply_in_place(document, "remove-lang", &Json::Null, profile)?;
                    }
                }
                "displayDocTitle" => {
                    let present = catalog_dict(document).map(|catalog| catalog.has(b"ViewerPreferences")).unwrap_or(false);
                    if stamped {
                        apply_in_place(document, "set-display-doc-title", &json_object(vec![("displayDocTitle", Json::Bool(true))]), profile)?;
                    } else if present {
                        apply_in_place(document, "remove-display-doc-title", &Json::Null, profile)?;
                    }
                }
                "infoTitle" => {
                    let title = if stamped { format!("A {} conformant document", profile.subset.to_uppercase()) } else { String::new() };
                    set_info_entry(document, "Title", &title)?;
                }
                "infoAuthor" => {
                    let author = if stamped { "semio stdio conformance stamp".to_string() } else { String::new() };
                    set_info_entry(document, "Author", &author)?;
                }
                "signatureFields" => {
                    let present = signature_fields(document).iter().any(|(_, title)| title == "Signature1");
                    if stamped && !present {
                        apply_in_place(document, "insert-signature-field", &json_object(vec![("name", Json::String("Signature1".to_string()))]), profile)?;
                    } else if !stamped && present {
                        apply_in_place(document, "remove-signature-field", &json_object(vec![("name", Json::String("Signature1".to_string()))]), profile)?;
                    }
                }
                "dpartRoot" => {
                    let present = dpart_nodes(document).is_some();
                    if stamped && !present {
                        apply_in_place(document, "set-dpart-root", &json_object(vec![("job", Json::String("variable-data job 1".to_string()))]), profile)?;
                    } else if !stamped && present {
                        apply_in_place(document, "remove-dpart-root", &Json::Null, profile)?;
                    }
                }
                "pageBoxes" => {
                    let pages = document.get_pages().len();
                    for index in 0..pages {
                        let page = page_at(document, index)?;
                        let present = box_of(document, page, "TrimBox").is_some();
                        if stamped {
                            let media = box_of(document, page, "MediaBox").unwrap_or_else(|| vec![0.0, 0.0, 612.0, 792.0]);
                            let trim = Json::Array(media.iter().map(|value| Json::Number(*value as f64)).collect());
                            apply_in_place(document, "set-trim-box", &Json::Object(vec![("pageIndex".to_string(), Json::Number(index as f64)), ("trimBox".to_string(), trim)]), profile)?;
                        } else if present {
                            apply_in_place(document, "remove-trim-box", &json_object(vec![("pageIndex", Json::Number(index as f64))]), profile)?;
                        }
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
    //#endregion 🔖️Forward

    //#region 🔖️Arrange
    /// 🎬️ Prepares the input a kind needs its target to be present (or absent) in. The committed
    /// bachelor-thesis fixture carries NO encryption dictionary, no JavaScript or launch action, no
    /// media annotation, no embedded file, no OutputIntent, no `/MarkInfo`, `/StructTreeRoot`,
    /// `/Lang`, `/ViewerPreferences`, `/AcroForm`, `/DPartRoot` and no `/TrimBox` — a fact about the
    /// real corpus, verified by scanning the file and recorded in every feature that uses it — so
    /// every REMOVAL kind is exercised on the real document after this same independent
    /// implementation has inserted its target. `embed-font-file` is the mirror case: all 23 of the
    /// fixture's `/FontDescriptor` objects already carry an embedded program, so its target
    /// descriptor is emptied first. Every other kind reads the committed bytes untouched.
    pub fn conformance_arrange(input: &[u8], forward: &Json, profile: &PdfConformanceProfile) -> Result<Vec<u8>, String> {
        let params = params_of(forward);
        let file_name = || Json::String(params.str("fileName"));
        let version = || params.get("version").cloned().unwrap_or(Json::Number(2.0));
        let revision = || params.get("revision").cloned().unwrap_or(Json::Number(3.0));
        let page_index = || params.get("pageIndex").cloned().unwrap_or(Json::Number(0.0));
        let steps: Vec<Json> = match forward.str("kind").as_str() {
            "remove-encryption-dictionary" => vec![kind_spec("insert-encryption-dictionary", vec![("version", version()), ("revision", revision())])],
            "remove-javascript-action" => vec![kind_spec("insert-javascript-action", vec![("script", Json::String(params.str("script")))])],
            "remove-launch-action" => vec![kind_spec("insert-launch-action", vec![("target", Json::String(params.str("target")))])],
            "remove-media-annotation" => vec![kind_spec("insert-media-annotation", vec![("subtype", Json::String(params.str("subtype"))), ("title", Json::String(params.str("title")))])],
            "remove-embedded-file" | "set-af-relationship" => vec![kind_spec("insert-embedded-file", vec![("fileName", file_name())])],
            "remove-af-relationship" => vec![
                kind_spec("insert-embedded-file", vec![("fileName", file_name())]),
                kind_spec("set-af-relationship", vec![("fileName", file_name()), ("relationship", Json::String("Supplement".to_string()))]),
            ],
            "remove-output-intent" => vec![kind_spec("set-output-intent", vec![("identifier", Json::String("sRGB IEC61966-2.1".to_string()))])],
            "remove-mark-info" => vec![kind_spec("set-mark-info", vec![("marked", Json::Bool(true))])],
            "remove-struct-tree-root" => vec![kind_spec("set-struct-tree-root", vec![])],
            "remove-lang" => vec![kind_spec("set-lang", vec![("lang", Json::String("de-CH".to_string()))])],
            "remove-display-doc-title" => vec![kind_spec("set-display-doc-title", vec![("displayDocTitle", Json::Bool(true))])],
            "remove-signature-field" => vec![kind_spec("insert-signature-field", vec![("name", Json::String(params.str("name")))])],
            "remove-trim-box" => vec![kind_spec("set-trim-box", vec![("pageIndex", page_index()), ("trimBox", Json::Array(vec![Json::Number(0.0), Json::Number(0.0), Json::Number(595.276), Json::Number(841.89)]))])],
            "remove-dpart-root" | "set-dpart-metadata" | "remove-dpart-metadata" => vec![kind_spec("set-dpart-root", vec![("job", Json::String("arranged variable-data job".to_string()))])],
            "embed-font-file" => vec![kind_spec("remove-font-file", vec![("descriptorOrdinal", params.get("descriptorOrdinal").cloned().unwrap_or(Json::Number(0.0)))])],
            _ => Vec::new(),
        };
        let mut current = input.to_vec();
        for step in steps {
            current = apply_conformance_mutation(&current, &step, profile)?;
        }
        Ok(current)
    }

    //#endregion 🔖️Arrange

    //#region 🔖️Inverse
    /// ↩️ The undo of `forward`, computed by reading whatever pre-mutation state it needs straight
    /// out of `base` through the SAME independent implementation the mutation runs on — never by
    /// calling this repository's own `Mutation::inverse`, which would defeat the point of an
    /// independently computed oracle. `base` is the mutation's real pre-state, i.e. the output of
    /// [`conformance_arrange`].
    pub fn conformance_inverse_spec(base: &[u8], forward: &Json, _profile: &PdfConformanceProfile) -> Result<Json, String> {
        let params = params_of(forward);
        let document = load(base)?;
        let carry = |key: &str| Json::String(params.str(key));
        Ok(match forward.str("kind").as_str() {
            "no-mutation" => kind_spec("no-mutation", vec![]),
            "set-snapshot" => {
                let back = if params.str("conformance") == "stamped" { "stripped" } else { "stamped" };
                kind_spec("set-snapshot", vec![("conformance", Json::String(back.to_string()))])
            }
            "insert-encryption-dictionary" => kind_spec("remove-encryption-dictionary", vec![("version", params.get("version").cloned().unwrap_or(Json::Number(2.0))), ("revision", params.get("revision").cloned().unwrap_or(Json::Number(3.0)))]),
            "remove-encryption-dictionary" => kind_spec("insert-encryption-dictionary", vec![("version", params.get("version").cloned().unwrap_or(Json::Number(2.0))), ("revision", params.get("revision").cloned().unwrap_or(Json::Number(3.0)))]),
            "insert-javascript-action" => kind_spec("remove-javascript-action", vec![("script", carry("script"))]),
            "remove-javascript-action" => kind_spec("insert-javascript-action", vec![("script", carry("script"))]),
            "insert-launch-action" => kind_spec("remove-launch-action", vec![("target", carry("target"))]),
            "remove-launch-action" => kind_spec("insert-launch-action", vec![("target", carry("target"))]),
            "insert-media-annotation" => kind_spec("remove-media-annotation", vec![("subtype", carry("subtype")), ("title", carry("title"))]),
            "remove-media-annotation" => kind_spec("insert-media-annotation", vec![("subtype", carry("subtype")), ("title", carry("title"))]),
            "insert-embedded-file" => kind_spec("remove-embedded-file", vec![("fileName", carry("fileName"))]),
            "remove-embedded-file" => kind_spec("insert-embedded-file", vec![("fileName", carry("fileName"))]),
            "set-af-relationship" => {
                let name = params.str("fileName");
                match file_specs(&document).into_iter().find(|(_, file, _, _)| file == &name).and_then(|(_, _, _, relationship)| relationship) {
                    Some(previous) => kind_spec("set-af-relationship", vec![("fileName", carry("fileName")), ("relationship", Json::String(previous))]),
                    None => kind_spec("remove-af-relationship", vec![("fileName", carry("fileName"))]),
                }
            }
            "remove-af-relationship" => {
                let name = params.str("fileName");
                let previous = file_specs(&document).into_iter().find(|(_, file, _, _)| file == &name).and_then(|(_, _, _, relationship)| relationship).ok_or_else(|| format!("remove-af-relationship has no inverse: the base's Filespec for {name:?} carries no /AFRelationship"))?;
                kind_spec("set-af-relationship", vec![("fileName", carry("fileName")), ("relationship", Json::String(previous))])
            }
            "set-output-intent" => kind_spec("remove-output-intent", vec![]),
            "remove-output-intent" => {
                let identifier = output_intents(&document).first().map(|(_, identifier, _)| identifier.clone()).ok_or("remove-output-intent has no inverse: the base carries no OutputIntent to restore")?;
                kind_spec("set-output-intent", vec![("identifier", Json::String(identifier))])
            }
            "embed-font-file" => kind_spec("remove-font-file", vec![("descriptorOrdinal", params.get("descriptorOrdinal").cloned().unwrap_or(Json::Number(0.0)))]),
            "remove-font-file" => {
                let index = ordinal(&params, "descriptorOrdinal")?;
                let descriptors = font_descriptors(&document);
                let descriptor = *descriptors.get(index).ok_or_else(|| format!("remove-font-file has no inverse: descriptor ordinal {index} is out of range"))?;
                let (key, program, _) = font_program(&document, descriptor).ok_or_else(|| format!("remove-font-file has no inverse: descriptor ordinal {index} carries no embedded font program in the base"))?;
                kind_spec(
                    "embed-font-file",
                    vec![
                        ("descriptorOrdinal", Json::Number(index as f64)),
                        ("key", Json::String(key)),
                        ("program", Json::Object(vec![("num".to_string(), Json::Number(program.0 as f64)), ("gen".to_string(), Json::Number(program.1 as f64))])),
                    ],
                )
            }
            "set-trim-box" => {
                let index = ordinal(&params, "pageIndex")?;
                let page = page_at(&document, index)?;
                match box_of(&document, page, "TrimBox") {
                    Some(previous) => kind_spec("set-trim-box", vec![("pageIndex", Json::Number(index as f64)), ("trimBox", number_array(&previous))]),
                    None => kind_spec("remove-trim-box", vec![("pageIndex", Json::Number(index as f64))]),
                }
            }
            "remove-trim-box" => {
                let index = ordinal(&params, "pageIndex")?;
                let page = page_at(&document, index)?;
                let previous = box_of(&document, page, "TrimBox").ok_or_else(|| format!("remove-trim-box has no inverse: page {index} carries no /TrimBox in the base"))?;
                kind_spec("set-trim-box", vec![("pageIndex", Json::Number(index as f64)), ("trimBox", number_array(&previous))])
            }
            "set-mark-info" => match catalog_dict(&document).ok().and_then(|catalog| catalog.get(b"MarkInfo").ok().and_then(|value| deref_dict(&document, value)).map(|dict| dict.get(b"Marked").ok().and_then(|value| value.as_bool().ok()).unwrap_or(false))) {
                Some(previous) => kind_spec("set-mark-info", vec![("marked", Json::Bool(previous))]),
                None => kind_spec("remove-mark-info", vec![]),
            },
            "remove-mark-info" => kind_spec("set-mark-info", vec![("marked", Json::Bool(true))]),
            "set-struct-tree-root" => kind_spec("remove-struct-tree-root", vec![]),
            "remove-struct-tree-root" => kind_spec("set-struct-tree-root", vec![]),
            "set-lang" => match catalog_dict(&document).ok().and_then(|catalog| text_at(catalog, "Lang")) {
                Some(previous) => kind_spec("set-lang", vec![("lang", Json::String(previous))]),
                None => kind_spec("remove-lang", vec![]),
            },
            "remove-lang" => {
                let previous = catalog_dict(&document).ok().and_then(|catalog| text_at(catalog, "Lang")).ok_or("remove-lang has no inverse: the base carries no /Lang")?;
                kind_spec("set-lang", vec![("lang", Json::String(previous))])
            }
            "set-display-doc-title" => match catalog_dict(&document).ok().and_then(|catalog| catalog.get(b"ViewerPreferences").ok().and_then(|value| deref_dict(&document, value)).map(|dict| dict.get(b"DisplayDocTitle").ok().and_then(|value| value.as_bool().ok()).unwrap_or(false))) {
                Some(previous) => kind_spec("set-display-doc-title", vec![("displayDocTitle", Json::Bool(previous))]),
                None => kind_spec("remove-display-doc-title", vec![]),
            },
            "remove-display-doc-title" => kind_spec("set-display-doc-title", vec![("displayDocTitle", Json::Bool(true))]),
            "set-info-title" => kind_spec("set-info-title", vec![("title", Json::String(info_entry(&document, "Title").unwrap_or_default()))]),
            "set-info-author" => kind_spec("set-info-author", vec![("author", Json::String(info_entry(&document, "Author").unwrap_or_default()))]),
            "insert-signature-field" => kind_spec("remove-signature-field", vec![("name", carry("name"))]),
            "remove-signature-field" => kind_spec("insert-signature-field", vec![("name", carry("name"))]),
            "set-dpart-root" => kind_spec("remove-dpart-root", vec![]),
            "remove-dpart-root" => {
                let job = dpart_nodes(&document).and_then(|(_, nodes)| nodes[0].1.clone()).unwrap_or_default();
                kind_spec("set-dpart-root", vec![("job", Json::String(job))])
            }
            "set-dpart-metadata" => match dpart_nodes(&document).and_then(|(_, nodes)| nodes[0].1.clone()) {
                Some(previous) => kind_spec("set-dpart-metadata", vec![("job", Json::String(previous))]),
                None => kind_spec("remove-dpart-metadata", vec![]),
            },
            "remove-dpart-metadata" => {
                let previous = dpart_nodes(&document).and_then(|(_, nodes)| nodes[0].1.clone()).ok_or("remove-dpart-metadata has no inverse: the base's /DPart node carries no /DPM")?;
                kind_spec("set-dpart-metadata", vec![("job", Json::String(previous))])
            }
            other => return Err(format!("no inverse rule for kind {other:?}")),
        })
    }
    //#endregion 🔖️Inverse

    //#region 🔖️Projection
    /// 👁️ The conformance-class projection, scoped to exactly the axes `profile` declares — a
    /// subset can never be judged on an axis its own checker does not read, and never escape one it
    /// does. Every field is read back out of the BYTES by this independent implementation; nothing
    /// is carried by the caller.
    ///
    /// 🔢️ Object NUMBERS are deliberately absent. A conformance class is defined on what the
    /// document contains, not on where a writer chose to put it: re-inserting a removed action at a
    /// fresh object number is a faithful undo, and a projection that recorded the number would
    /// report a false divergence for it. What the projection records instead is the content each
    /// checker's own diagnostic quotes.
    pub fn project(input: &[u8], profile: &PdfConformanceProfile) -> Result<Json, String> {
        let document = load(input)?;
        let mut fields: Vec<(String, Json)> = vec![("subset".to_string(), Json::String(profile.subset.to_string())), ("pageCount".to_string(), Json::Number(document.get_pages().len() as f64))];
        for axis in profile.axes {
            let value = match *axis {
                "encryptionDictionaries" => Json::Array(encryption_dictionaries(&document).into_iter().map(|(_, version, revision)| json_object(vec![("version", Json::Number(version as f64)), ("revision", Json::Number(revision as f64))])).collect()),
                "javaScriptActions" => Json::Array(actions(&document, "JavaScript", "JS").into_iter().map(|(_, script)| Json::String(script)).collect()),
                "launchActions" => Json::Array(actions(&document, "Launch", "F").into_iter().map(|(_, target)| Json::String(target)).collect()),
                "mediaAnnotations" => Json::Array(media_annotations(&document).into_iter().map(|(_, subtype, title)| json_object(vec![("subtype", Json::String(subtype)), ("title", Json::String(title))])).collect()),
                "embeddedFiles" => Json::Array(
                    file_specs(&document)
                        .into_iter()
                        .map(|(_, name, has_stream, relationship)| json_object(vec![("fileName", Json::String(name)), ("hasEmbeddedStream", Json::Bool(has_stream)), ("afRelationship", optional_string(relationship))]))
                        .collect(),
                ),
                "outputIntents" => Json::Array(
                    output_intents(&document)
                        .into_iter()
                        .map(|(subtype, identifier, dest)| json_object(vec![("subtype", Json::String(subtype)), ("outputConditionIdentifier", Json::String(identifier)), ("hasDestOutputProfile", Json::Bool(dest))]))
                        .collect(),
                ),
                "fontPrograms" => Json::Array(
                    font_descriptors(&document)
                        .into_iter()
                        .map(|descriptor| match font_program(&document, descriptor) {
                            Some((key, _, size)) => json_object(vec![("key", Json::String(key)), ("programBytes", Json::Number(size as f64))]),
                            None => json_object(vec![("key", Json::Null), ("programBytes", Json::Null)]),
                        })
                        .collect(),
                ),
                "pageBoxes" => {
                    let mut boxes = Vec::new();
                    for index in 0..document.get_pages().len() {
                        let page = page_at(&document, index)?;
                        boxes.push(json_object(vec![
                            ("trimBox", box_of(&document, page, "TrimBox").map(|values| number_array(&values)).unwrap_or(Json::Null)),
                            ("artBox", box_of(&document, page, "ArtBox").map(|values| number_array(&values)).unwrap_or(Json::Null)),
                        ]));
                    }
                    Json::Array(boxes)
                }
                "markInfo" => match catalog_dict(&document).ok().and_then(|catalog| catalog.get(b"MarkInfo").ok().and_then(|value| deref_dict(&document, value))) {
                    Some(dict) => Json::Bool(dict.get(b"Marked").ok().and_then(|value| value.as_bool().ok()).unwrap_or(false)),
                    None => Json::Null,
                },
                "structTreeRoot" => Json::Bool(catalog_dict(&document).map(|catalog| catalog.has(b"StructTreeRoot")).unwrap_or(false)),
                "lang" => optional_string(catalog_dict(&document).ok().and_then(|catalog| text_at(catalog, "Lang"))),
                "displayDocTitle" => match catalog_dict(&document).ok().and_then(|catalog| catalog.get(b"ViewerPreferences").ok().and_then(|value| deref_dict(&document, value))) {
                    Some(dict) => Json::Bool(dict.get(b"DisplayDocTitle").ok().and_then(|value| value.as_bool().ok()).unwrap_or(false)),
                    None => Json::Null,
                },
                "infoTitle" => optional_string(info_entry(&document, "Title")),
                "infoAuthor" => optional_string(info_entry(&document, "Author")),
                "signatureFields" => Json::Array(signature_fields(&document).into_iter().map(|(_, title)| Json::String(title)).collect()),
                "dpartRoot" => match dpart_nodes(&document) {
                    Some((_, nodes)) => Json::Array(nodes.into_iter().map(|(_, metadata)| json_object(vec![("hasMetadata", Json::Bool(metadata.is_some())), ("job", optional_string(metadata))])).collect()),
                    None => Json::Null,
                },
                other => return Err(format!("projection axis {other:?} is not implemented by the PDF conformance engine")),
            };
            fields.push((axis.to_string(), value));
        }
        Ok(Json::Object(fields))
    }
    //#endregion 🔖️Projection
}
//#endregion 🔖️PdfConformanceClass
