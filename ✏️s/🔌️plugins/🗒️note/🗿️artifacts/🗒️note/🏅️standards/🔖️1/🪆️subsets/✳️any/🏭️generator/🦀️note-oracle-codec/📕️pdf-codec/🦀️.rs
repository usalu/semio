//! 📄️ PDF carrier — writer AND reader, both `lopdf` 0.44, matching `lopdf-note-text-reader` in
//! `../../🧪️oracle/🔣️.json`. Reproduces `NoteIntoPdf::serialize`'s body exactly: `title` + every
//! `Text` block's paragraph runs, space-joined, onto ONE page's `Tj` text stream — no visibility
//! filter (the same cross-carrier bug DXF has, confirmed on a second serializer), no position, no
//! other block kind.

use crate::{flatten_all, obj, s, Block, Json, NoteDoc};
use lopdf::{content::Content, content::Operation, dictionary, Document, Object, Stream};

/// 🧾️ `title` then every `Text` block's runs, space-joined, trimmed — UNFILTERED by visibility,
/// reproducing the exact bug `NoteIntoPdf`/`NoteIntoDxf` share.
fn page_text(doc: &NoteDoc) -> String {
    let mut text = String::new();
    if let Some(title) = &doc.title {
        text.push_str(title);
        text.push(' ');
    }
    for block in flatten_all(doc) {
        if let Block::Text { text: block_text, .. } = block {
            text.push_str(block_text);
            text.push(' ');
        }
    }
    text.trim().to_string()
}

fn page_size(doc: &NoteDoc) -> (f64, f64) {
    let mut max_x = 1024.0_f64;
    let mut max_y = 1024.0_f64;
    for block in flatten_all(doc) {
        let (x, y, _r, width, height, visible) = block.common();
        if !visible {
            continue;
        }
        max_x = max_x.max(x + width);
        max_y = max_y.max(y + height);
    }
    (max_x.max(1.0).round(), max_y.max(1.0).round())
}

/// ✍️ Writes a real PDF 1.4 document with `lopdf`'s own `Document`/`Stream`/`save_to` — nothing
/// hand-formatted, and deliberately no `/Producer`/`/CreationDate` (never set, so never leaks a
/// wall-clock stamp into the committed bytes — MEASURED via `fixture reproduce`, not assumed).
pub fn write_pdf(doc: &NoteDoc) -> Result<Vec<u8>, String> {
    let (w, h) = page_size(doc);
    let mut document = Document::with_version("1.4");
    let pages_id = document.new_object_id();
    let content = Content { operations: vec![Operation::new("Tj", vec![Object::string_literal(page_text(doc))])] };
    let content_id = document.add_object(Stream::new(dictionary! {}, content.encode().map_err(|e| format!("lopdf encode content: {e}"))?));
    let page_id = document.add_object(dictionary! { "Type" => "Page", "Parent" => pages_id, "Contents" => content_id, "MediaBox" => vec![0.into(), 0.into(), w.into(), h.into()] });
    let pages = dictionary! { "Type" => "Pages", "Kids" => vec![page_id.into()], "Count" => 1 };
    document.objects.insert(pages_id, Object::Dictionary(pages));
    let catalog_id = document.add_object(dictionary! { "Type" => "Catalog", "Pages" => pages_id });
    document.trailer.set("Root", catalog_id);
    let mut bytes = Vec::new();
    document.save_to(&mut bytes).map_err(|e| format!("lopdf save: {e}"))?;
    Ok(bytes)
}

/// 📖 Reads PDF bytes with `lopdf`'s own `Document::load_mem` and extracts every `Tj`/`TJ` text
/// operand from the (single) page's content stream — independent of this file's own writer.
pub fn project_pdf(bytes: &[u8]) -> Result<Vec<String>, String> {
    let document = Document::load_mem(bytes).map_err(|e| format!("lopdf load: {e}"))?;
    let mut texts = Vec::new();
    for (_, page_id) in document.get_pages() {
        let page_content = document.get_page_content(page_id);
        let decoded = Content::decode(&page_content).map_err(|e| format!("lopdf decode content: {e}"))?;
        for operation in decoded.operations.iter().filter(|op| op.operator == "Tj" || op.operator == "TJ") {
            for operand in &operation.operands {
                if let Ok(bytes) = operand.as_str() {
                    texts.push(String::from_utf8_lossy(bytes).into_owned());
                }
            }
        }
    }
    Ok(texts)
}

pub fn project_pdf_json(bytes: &[u8]) -> Result<Json, String> {
    let texts = project_pdf(bytes)?;
    Ok(obj(vec![("pageCount", Json::Int(1)), ("text", Json::Arr(texts.into_iter().map(|t| s(&t)).collect()))]))
}

pub fn compare_pdf(expected: &[u8], actual: &[u8]) -> Result<(bool, Vec<String>), String> {
    let e = project_pdf(expected)?;
    let a = project_pdf(actual)?;
    let mut problems = Vec::new();
    if e != a {
        problems.push(format!("page text differs: expected {e:?} actual {a:?}"));
    }
    Ok((problems.is_empty(), problems))
}
