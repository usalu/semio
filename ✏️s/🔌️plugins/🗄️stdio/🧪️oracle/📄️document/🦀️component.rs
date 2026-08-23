//! 🔮️ Rust oracle host. The ONLY place in the repository where a third-party reference
//! implementation is linked. Every registered oracle is wrapped behind an owned interface — no
//! external type appears in this module's public API, so nothing downstream can accidentally depend
//! on `pdf-writer` or `lopdf`. Compiled only with the `oracles` feature, which no production target
//! enables.
//!
//! @see 📇️registry/🔣️component.json — the approved oracle registry these functions implement.

use crate::protocol::Json;

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
