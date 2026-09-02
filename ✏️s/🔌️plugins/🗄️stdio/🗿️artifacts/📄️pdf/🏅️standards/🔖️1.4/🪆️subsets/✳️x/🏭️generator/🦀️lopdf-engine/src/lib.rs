//! 📄️ The `pdf@1.4/x` mutation vocabulary and structural projection, expressed ENTIRELY through
//! `lopdf` 0.44's own public COS API.
//!
//! `base` is the generic layer: pages, objects, dictionary entries and trailer entries, with no
//! conformance class layered on top. Every kind here is an ordinary COS graph edit, which is exactly
//! what `lopdf` exposes — so the mutation is applied through the library and the result is read back
//! through it, and nothing in this repository predicts what the judge is judging.
//!
//! @see ../../../🦀️oracle.rs — the subset's own vocabulary.

use lopdf::{dictionary, Document, Object, ObjectId, Stream};

pub const KINDS: &[&str] = &[
    "set-page-size",
    "collapse-page-size",
];

fn escape(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out
}

fn catalog_id(document: &Document) -> ObjectId {
    document.trailer.get(b"Root").and_then(Object::as_reference).expect("a trailer /Root")
}

fn pages_id(document: &Document) -> ObjectId {
    document
        .get_object(catalog_id(document))
        .and_then(Object::as_dict)
        .and_then(|d| d.get(b"Pages"))
        .and_then(Object::as_reference)
        .expect("a catalog /Pages")
}

fn kids(document: &Document) -> Vec<ObjectId> {
    document
        .get_object(pages_id(document))
        .and_then(Object::as_dict)
        .and_then(|d| d.get(b"Kids"))
        .and_then(Object::as_array)
        .map(|array| array.iter().filter_map(|e| e.as_reference().ok()).collect())
        .unwrap_or_default()
}

fn set_kids(document: &mut Document, order: Vec<ObjectId>) {
    let id = pages_id(document);
    let count = order.len() as i64;
    if let Ok(dict) = document.get_object_mut(id).and_then(Object::as_dict_mut) {
        dict.set("Kids", order.into_iter().map(Object::Reference).collect::<Vec<_>>());
        dict.set("Count", count);
    }
}

fn new_page(document: &mut Document, text: &str) -> ObjectId {
    let parent = pages_id(document);
    let content = document.add_object(Stream::new(dictionary! {}, format!("BT /F1 12 Tf 72 720 Td ({text}) Tj ET").into_bytes()));
    document.add_object(dictionary! {
        "Type" => "Page",
        "Parent" => parent,
        "Contents" => content,
        "MediaBox" => vec![0.into(), 0.into(), Object::Real(612.0), Object::Real(792.0)],
        "Resources" => dictionary! {},
    })
}

/// 🌱️ A two-page deterministic seed. Two pages, not one: `remove-page` and `move-page` are only
/// observable when there is more than one, and a corpus whose mutations are not observable is not
/// evidence. Plus a spare object for `remove-object`/`set-object-value` to target.
pub fn build_seed() -> Vec<u8> {
    let mut document = Document::with_version("1.7");
    let pages = document.new_object_id();
    let mut make = |document: &mut Document, text: &str| {
        let content = document.add_object(Stream::new(dictionary! {}, format!("BT /F1 12 Tf 72 720 Td ({text}) Tj ET").into_bytes()));
        document.add_object(dictionary! {
            "Type" => "Page",
            "Parent" => pages,
            "Contents" => content,
            "MediaBox" => vec![0.into(), 0.into(), Object::Real(612.0), Object::Real(792.0)],
            "Resources" => dictionary! {},
        })
    };
    let first = make(&mut document, "pdf-1-4-x fixture seed page one");
    let second = make(&mut document, "pdf-1-4-x fixture seed page two");
    document.objects.insert(
        pages,
        Object::Dictionary(dictionary! { "Type" => "Pages", "Kids" => vec![first.into(), second.into()], "Count" => 2 }),
    );
    document.add_object(dictionary! { "Type" => "SpareMarker", "Label" => Object::string_literal("spare") });
    let catalog = document.add_object(dictionary! { "Type" => "Catalog", "Pages" => pages });
    let info = document.add_object(dictionary! {
        "Title" => Object::string_literal("pdf-1-4-x fixture seed"),
        "Author" => Object::string_literal("pdf-1-4-x-lopdf-engine"),
    });
    document.trailer.set("Root", catalog);
    document.trailer.set("Info", info);
    let mut out = Vec::new();
    document.save_to(&mut out).expect("lopdf saves the document it just built");
    out
}

fn spare_id(document: &Document) -> Option<ObjectId> {
    let mut ids: Vec<ObjectId> = document
        .objects
        .iter()
        .filter(|(_, object)| {
            object
                .as_dict()
                .ok()
                .and_then(|d| d.get(b"Type").ok())
                .and_then(|t| t.as_name().ok())
                .map(|name| name == b"SpareMarker")
                .unwrap_or(false)
        })
        .map(|(id, _)| *id)
        .collect();
    ids.sort_unstable();
    ids.first().copied()
}

/// 🌾️ ARRANGEMENT — puts each kind's precondition in place.
pub fn arrange(kind: &str, bytes: &[u8]) -> Vec<u8> {
    let mut document = Document::load_mem(bytes).expect("lopdf reloads the seed it wrote");
    match kind {
        "remove-dict-entry" => {
            let id = catalog_id(&document);
            if let Ok(dict) = document.get_object_mut(id).and_then(Object::as_dict_mut) {
                dict.set("SemioMarker", Object::string_literal("removable"));
            }
        }
        "remove-trailer-entry" => document.trailer.set("SemioMarker", Object::string_literal("removable")),
        "set-page-crop-box" | "set-page-rotation" | "set-page-media-box" => {}
        _ => {}
    }
    let mut out = Vec::new();
    document.save_to(&mut out).expect("lopdf saves the arranged document");
    out
}

/// ✍️ The forward mutation, performed through `lopdf`'s own COS API.
pub fn apply(kind: &str, bytes: &[u8]) -> Result<Vec<u8>, String> {
    let mut document = Document::load_mem(bytes).map_err(|error| error.to_string())?;
    match kind {
        "insert-page" => {
            let page = new_page(&mut document, "inserted page");
            let mut order = kids(&document);
            order.insert(1, page);
            set_kids(&mut document, order);
        }
        "remove-page" => {
            let mut order = kids(&document);
            order.pop();
            set_kids(&mut document, order);
        }
        "move-page" => {
            let mut order = kids(&document);
            order.reverse();
            set_kids(&mut document, order);
        }
        "set-page-media-box" => {
            let page = kids(&document)[0];
            let dict = document.get_object_mut(page).and_then(Object::as_dict_mut).map_err(|e| e.to_string())?;
            dict.set("MediaBox", vec![0.into(), 0.into(), Object::Real(595.276), Object::Real(841.89)]);
        }
        "set-page-crop-box" => {
            let page = kids(&document)[0];
            let dict = document.get_object_mut(page).and_then(Object::as_dict_mut).map_err(|e| e.to_string())?;
            dict.set("CropBox", vec![Object::Real(9.0), Object::Real(9.0), Object::Real(586.0), Object::Real(833.0)]);
        }
        "set-page-rotation" => {
            let page = kids(&document)[0];
            let dict = document.get_object_mut(page).and_then(Object::as_dict_mut).map_err(|e| e.to_string())?;
            dict.set("Rotate", 90);
        }
        "set-page-text" | "replace-page-text" => {
            let page = kids(&document)[0];
            let content = document.add_object(Stream::new(dictionary! {}, b"BT /F1 12 Tf 72 700 Td (replaced page text) Tj ET".to_vec()));
            let dict = document.get_object_mut(page).and_then(Object::as_dict_mut).map_err(|e| e.to_string())?;
            dict.set("Contents", Object::Reference(content));
        }
        "clear-page-text" => {
            let page = kids(&document)[0];
            let content = document.add_object(Stream::new(dictionary! {}, Vec::new()));
            let dict = document.get_object_mut(page).and_then(Object::as_dict_mut).map_err(|e| e.to_string())?;
            dict.set("Contents", Object::Reference(content));
        }
        "resize-page" | "set-page-size" => {
            let page = kids(&document)[0];
            let dict = document.get_object_mut(page).and_then(Object::as_dict_mut).map_err(|e| e.to_string())?;
            dict.set("MediaBox", vec![0.into(), 0.into(), Object::Real(420.0), Object::Real(595.0)]);
        }
        "collapse-page-size" => {
            let page = kids(&document)[0];
            let dict = document.get_object_mut(page).and_then(Object::as_dict_mut).map_err(|e| e.to_string())?;
            dict.set("MediaBox", vec![0.into(), 0.into(), Object::Real(1.0), Object::Real(1.0)]);
        }
        "set-page-content" => {
            let page = kids(&document)[0];
            let content = document.add_object(Stream::new(dictionary! {}, b"BT /F1 12 Tf 72 700 Td (replaced content) Tj ET".to_vec()));
            let dict = document.get_object_mut(page).and_then(Object::as_dict_mut).map_err(|e| e.to_string())?;
            dict.set("Contents", Object::Reference(content));
        }
        "append-page-content" => {
            let page = kids(&document)[0];
            let existing = document
                .get_object(page)
                .and_then(Object::as_dict)
                .map_err(|e| e.to_string())?
                .get(b"Contents")
                .and_then(Object::as_reference)
                .map_err(|e| e.to_string())?;
            let mut content = document.get_object(existing).and_then(Object::as_stream).map_err(|e| e.to_string())?.content.clone();
            content.extend_from_slice(b"\nBT /F1 10 Tf 72 680 Td (appended) Tj ET");
            let added = document.add_object(Stream::new(dictionary! {}, content));
            let dict = document.get_object_mut(page).and_then(Object::as_dict_mut).map_err(|e| e.to_string())?;
            dict.set("Contents", Object::Reference(added));
        }
        "set-info" => {
            let info = document.trailer.get(b"Info").and_then(Object::as_reference).map_err(|e| e.to_string())?;
            let dict = document.get_object_mut(info).and_then(Object::as_dict_mut).map_err(|e| e.to_string())?;
            dict.set("Title", Object::string_literal("a replaced title"));
        }
        "insert-object" => {
            document.add_object(dictionary! { "Type" => "SemioInserted", "Label" => Object::string_literal("inserted") });
        }
        "remove-object" => {
            let spare = spare_id(&document).ok_or_else(|| "no spare object".to_string())?;
            document.objects.remove(&spare);
        }
        "set-object-value" => {
            let spare = spare_id(&document).ok_or_else(|| "no spare object".to_string())?;
            let dict = document.get_object_mut(spare).and_then(Object::as_dict_mut).map_err(|e| e.to_string())?;
            dict.set("Label", Object::string_literal("changed"));
        }
        "set-dict-entry" => {
            let id = catalog_id(&document);
            let dict = document.get_object_mut(id).and_then(Object::as_dict_mut).map_err(|e| e.to_string())?;
            dict.set("SemioMarker", Object::string_literal("set"));
        }
        "remove-dict-entry" => {
            let id = catalog_id(&document);
            let dict = document.get_object_mut(id).and_then(Object::as_dict_mut).map_err(|e| e.to_string())?;
            dict.remove(b"SemioMarker");
        }
        "set-trailer-entry" => document.trailer.set("SemioMarker", Object::string_literal("set")),
        "remove-trailer-entry" => {
            document.trailer.remove(b"SemioMarker");
        }
        other => return Err(format!("unknown kind {other}")),
    }
    let mut out = Vec::new();
    document.save_to(&mut out).map_err(|error| error.to_string())?;
    Ok(out)
}

/// 📄️ The structural projection: the page list in ORDER (so `move-page` is observable), each page's
/// boxes, rotation and content bytes, the `/Info` entries, and the catalog and trailer marker entries.
pub fn project(bytes: &[u8]) -> Result<String, String> {
    let document = Document::load_mem(bytes).map_err(|error| error.to_string())?;
    let order = kids(&document);
    let mut out = String::from("{\"subset\":\"x\"");
    out.push_str(&format!(",\"pageCount\":{}", order.len()));

    out.push_str(",\"pages\":[");
    for (index, page) in order.iter().enumerate() {
        if index > 0 {
            out.push(',');
        }
        let dict = document.get_object(*page).and_then(Object::as_dict).map_err(|e| e.to_string())?;
        let boxed = |key: &[u8]| -> String {
            match dict.get(key).and_then(Object::as_array) {
                Ok(values) => format!("[{}]", values.iter().map(|v| v.as_float().map(|f| format!("{f}")).unwrap_or_else(|_| "null".to_string())).collect::<Vec<_>>().join(",")),
                Err(_) => "null".to_string(),
            }
        };
        let rotation = dict.get(b"Rotate").and_then(Object::as_i64).map(|v| v.to_string()).unwrap_or_else(|_| "null".to_string());
        let content = dict
            .get(b"Contents")
            .and_then(Object::as_reference)
            .ok()
            .and_then(|id| document.get_object(id).ok())
            .and_then(|o| o.as_stream().ok())
            .map(|stream| String::from_utf8_lossy(&stream.content).to_string())
            .unwrap_or_default();
        out.push_str(&format!(
            "{{\"mediaBox\":{},\"cropBox\":{},\"rotate\":{},\"content\":\"{}\"}}",
            boxed(b"MediaBox"),
            boxed(b"CropBox"),
            rotation,
            escape(&content)
        ));
    }
    out.push(']');

    let catalog = document.get_object(catalog_id(&document)).and_then(Object::as_dict).map_err(|e| e.to_string())?;
    let marker = |value: Result<&Object, lopdf::Error>| -> String {
        match value {
            Ok(Object::String(bytes, _)) => format!("\"{}\"", escape(&String::from_utf8_lossy(bytes))),
            Ok(_) => "\"present\"".to_string(),
            Err(_) => "null".to_string(),
        }
    };
    out.push_str(&format!(",\"catalogMarker\":{}", marker(catalog.get(b"SemioMarker"))));
    out.push_str(&format!(",\"trailerMarker\":{}", marker(document.trailer.get(b"SemioMarker"))));

    let info = document.trailer.get(b"Info").and_then(Object::as_reference).ok().and_then(|id| document.get_object(id).ok()).and_then(|o| o.as_dict().ok());
    match info {
        Some(dict) => {
            let field = |key: &[u8]| -> String {
                dict.get(key).and_then(Object::as_str).map(|s| format!("\"{}\"", escape(&String::from_utf8_lossy(s)))).unwrap_or_else(|_| "null".to_string())
            };
            out.push_str(&format!(",\"info\":{{\"title\":{},\"author\":{}}}", field(b"Title"), field(b"Author")));
        }
        None => out.push_str(",\"info\":null"),
    }

    // 🔢️Object inventory by /Type — what `insert-object` and `remove-object` move, and the one axis a
    // page-level projection would miss entirely.
    let mut spare = 0usize;
    let mut inserted = 0usize;
    let mut spare_label = "null".to_string();
    for object in document.objects.values() {
        let Ok(dict) = object.as_dict() else { continue };
        match dict.get(b"Type").and_then(Object::as_name) {
            Ok(name) if name == b"SpareMarker" => {
                spare += 1;
                spare_label = dict.get(b"Label").and_then(Object::as_str).map(|s| format!("\"{}\"", escape(&String::from_utf8_lossy(s)))).unwrap_or_else(|_| "null".to_string());
            }
            Ok(name) if name == b"SemioInserted" => inserted += 1,
            _ => {}
        }
    }
    out.push_str(&format!(",\"spareObjects\":{spare},\"spareLabel\":{spare_label},\"insertedObjects\":{inserted}"));
    out.push('}');
    Ok(out)
}
