//! 📄️ The `pdf@1.7/base` mutation vocabulary and structural projection, expressed ENTIRELY through
//! `lopdf` 0.44's own public COS API.
//!
//! `base` is the generic layer: pages, objects, dictionary entries and trailer entries, plus the
//! resources, annotations, content operators, catalog entries, document identity and encryption the
//! typed model carries, with no conformance class layered on top. Every kind here is an ordinary COS
//! graph edit, which is exactly what `lopdf` exposes — so the mutation is applied through the library
//! and the result is read back through it, and nothing in this repository predicts what the judge is
//! judging.
//!
//! @see ../../../🔮️oracles/🦀️.rs — the subset's own vocabulary.

use lopdf::{dictionary, Dictionary, Document, EncryptionState, EncryptionVersion, Object, ObjectId, Permissions, Stream, StringFormat};

pub const KINDS: &[&str] = &[
    "insert-page",
    "remove-page",
    "move-page",
    "set-page-media-box",
    "set-page-crop-box",
    "set-page-rotation",
    "set-page-content",
    "append-page-content",
    "set-info",
    "insert-object",
    "remove-object",
    "set-object-value",
    "set-dict-entry",
    "remove-dict-entry",
    "set-trailer-entry",
    "remove-trailer-entry",
    "set-page-box",
    "set-page-user-unit",
    "insert-content",
    "remove-content",
    "replace-content",
    "insert-annotation",
    "remove-annotation",
    "set-annotation",
    "set-font",
    "remove-font",
    "set-image",
    "remove-image",
    "set-form",
    "remove-form",
    "set-ext-g-state",
    "remove-ext-g-state",
    "set-shading",
    "remove-shading",
    "set-pattern",
    "remove-pattern",
    "set-color-space",
    "remove-color-space",
    "set-properties",
    "remove-properties",
    "set-embedded-file",
    "remove-embedded-file",
    "set-outlines",
    "set-named-destination",
    "remove-named-destination",
    "set-page-labels",
    "set-output-intents",
    "set-acro-form",
    "set-optional-content",
    "set-page-layout",
    "set-page-mode",
    "set-viewer-preferences",
    "set-open-action",
    "set-language",
    "set-mark-info",
    "set-metadata",
    "set-document-id",
    "set-encryption",
    "set-catalog-entry",
    "remove-catalog-entry",
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
    let make = |document: &mut Document, text: &str| {
        let content = document.add_object(Stream::new(dictionary! {}, format!("BT /F1 12 Tf 72 720 Td ({text}) Tj ET").into_bytes()));
        document.add_object(dictionary! {
            "Type" => "Page",
            "Parent" => pages,
            "Contents" => content,
            "MediaBox" => vec![0.into(), 0.into(), Object::Real(612.0), Object::Real(792.0)],
            "Resources" => dictionary! {},
        })
    };
    let first = make(&mut document, "pdf-1-7-base fixture seed page one");
    let second = make(&mut document, "pdf-1-7-base fixture seed page two");
    document.objects.insert(
        pages,
        Object::Dictionary(dictionary! { "Type" => "Pages", "Kids" => vec![first.into(), second.into()], "Count" => 2 }),
    );
    document.add_object(dictionary! { "Type" => "SpareMarker", "Label" => Object::string_literal("spare") });
    let catalog = document.add_object(dictionary! { "Type" => "Catalog", "Pages" => pages });
    let info = document.add_object(dictionary! {
        "Title" => Object::string_literal("pdf-1-7-base fixture seed"),
        "Author" => Object::string_literal("pdf-1-7-base-lopdf-engine"),
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

//#region 🧩️ResourceCatalogAndPageKinds
/// 📄️ The first page: every page-, resource- and annotation-level kind edits it, so the second page
/// stays the untouched witness that the edit landed on exactly one page.
fn first_page(document: &Document) -> ObjectId {
    kids(document)[0]
}

fn dict_mut(document: &mut Document, id: ObjectId) -> Result<&mut Dictionary, String> {
    document.get_object_mut(id).and_then(Object::as_dict_mut).map_err(|error| error.to_string())
}

fn catalog_mut(document: &mut Document) -> Result<&mut Dictionary, String> {
    let id = catalog_id(document);
    dict_mut(document, id)
}

fn hex(bytes: &[u8]) -> Object {
    Object::String(bytes.to_vec(), StringFormat::Hexadecimal)
}

fn reals(values: &[f32]) -> Object {
    Object::Array(values.iter().map(|value| Object::Real(*value)).collect())
}

/// 🧭️ The `/Resources` category and entry name a resource kind edits, or `None` for any other kind.
fn resource_slot(kind: &str) -> Option<(&'static str, &'static str)> {
    match kind.trim_start_matches("set-").trim_start_matches("remove-") {
        "font" => Some(("Font", "F9")),
        "image" => Some(("XObject", "Im9")),
        "form" => Some(("XObject", "Fx9")),
        "ext-g-state" => Some(("ExtGState", "GS9")),
        "shading" => Some(("Shading", "Sh9")),
        "pattern" => Some(("Pattern", "P9")),
        "color-space" => Some(("ColorSpace", "CS9")),
        "properties" => Some(("Properties", "MC9")),
        _ => None,
    }
}

/// 🧱️ The value a resource kind installs, added as an indirect object unless it is an array.
fn resource_value(document: &mut Document, category: &str, name: &str) -> Object {
    let value = match (category, name) {
        ("Font", _) => Object::Dictionary(dictionary! { "Type" => "Font", "Subtype" => "Type1", "BaseFont" => "Helvetica", "Encoding" => "WinAnsiEncoding" }),
        ("XObject", "Im9") => Object::Stream(Stream::new(dictionary! { "Type" => "XObject", "Subtype" => "Image", "Width" => 2, "Height" => 2, "ColorSpace" => "DeviceGray", "BitsPerComponent" => 8 }, vec![0, 64, 128, 255])),
        ("XObject", _) => Object::Stream(Stream::new(dictionary! { "Type" => "XObject", "Subtype" => "Form", "BBox" => vec![0.into(), 0.into(), 10.into(), 10.into()] }, b"0 0 10 10 re f".to_vec())),
        ("ExtGState", _) => Object::Dictionary(dictionary! { "Type" => "ExtGState", "CA" => Object::Real(0.5), "ca" => Object::Real(0.5) }),
        ("Shading", _) => Object::Dictionary(axial_shading()),
        ("Pattern", _) => Object::Dictionary(dictionary! { "Type" => "Pattern", "PatternType" => 2, "Shading" => axial_shading() }),
        ("ColorSpace", _) => return Object::Array(vec!["CalRGB".into(), Object::Dictionary(dictionary! { "WhitePoint" => reals(&[0.9505, 1.0, 1.089]), "Gamma" => reals(&[2.2, 2.2, 2.2]) })]),
        _ => Object::Dictionary(dictionary! { "Semio" => Object::string_literal("marked content properties") }),
    };
    Object::Reference(document.add_object(value))
}

fn axial_shading() -> Dictionary {
    dictionary! {
        "ShadingType" => 2,
        "ColorSpace" => "DeviceRGB",
        "Coords" => vec![0.into(), 0.into(), 100.into(), 0.into()],
        "Function" => dictionary! { "FunctionType" => 2, "Domain" => vec![0.into(), 1.into()], "C0" => reals(&[1.0, 0.0, 0.0]), "C1" => reals(&[0.0, 0.0, 1.0]), "N" => 1 },
        "Extend" => vec![true.into(), true.into()],
    }
}

/// 📚️ Sets (`Some`) or removes (`None`) one entry of the first page's `/Resources /<category>`.
fn set_resource(document: &mut Document, category: &str, name: &str, value: Option<Object>) -> Result<(), String> {
    let page = first_page(document);
    let page = dict_mut(document, page)?;
    if !page.has(b"Resources") {
        page.set("Resources", dictionary! {});
    }
    let resources = page.get_mut(b"Resources").and_then(Object::as_dict_mut).map_err(|error| error.to_string())?;
    if !resources.has(category.as_bytes()) {
        resources.set(category, dictionary! {});
    }
    let entries = resources.get_mut(category.as_bytes()).and_then(Object::as_dict_mut).map_err(|error| error.to_string())?;
    match value {
        Some(value) => entries.set(name, value),
        None => {
            entries.remove(name.as_bytes());
        }
    }
    Ok(())
}

/// 🌳️ Replaces the catalog's `/Names /<tree>` with a flat name tree holding exactly `entries`.
fn set_name_tree(document: &mut Document, tree: &str, entries: Vec<(&str, Object)>) -> Result<(), String> {
    let names: Vec<Object> = entries.into_iter().flat_map(|(key, value)| [Object::string_literal(key), value]).collect();
    let catalog = catalog_mut(document)?;
    if !catalog.has(b"Names") {
        catalog.set("Names", dictionary! {});
    }
    let trees = catalog.get_mut(b"Names").and_then(Object::as_dict_mut).map_err(|error| error.to_string())?;
    trees.set(tree, dictionary! { "Names" => names });
    Ok(())
}

fn embedded_file(document: &mut Document) -> Object {
    let data = document.add_object(Stream::new(dictionary! { "Type" => "EmbeddedFile" }, b"semio embedded notes".to_vec()));
    Object::Reference(document.add_object(dictionary! { "Type" => "Filespec", "F" => Object::string_literal("notes.txt"), "UF" => Object::string_literal("notes.txt"), "EF" => dictionary! { "F" => data } }))
}

fn annotation(document: &mut Document, contents: &str) -> ObjectId {
    document.add_object(dictionary! { "Type" => "Annot", "Subtype" => "Text", "Rect" => vec![100.into(), 100.into(), 120.into(), 120.into()], "Contents" => Object::string_literal(contents) })
}

fn first_annotation(document: &Document) -> Result<ObjectId, String> {
    document
        .get_object(first_page(document))
        .and_then(Object::as_dict)
        .and_then(|page| page.get(b"Annots"))
        .and_then(Object::as_array)
        .map_err(|error| error.to_string())?
        .first()
        .and_then(|annotation| annotation.as_reference().ok())
        .ok_or_else(|| "the first page carries no annotation".to_string())
}

/// 🖋️ Rewrites the first page's content stream in place through `edit`.
fn edit_first_page_content(document: &mut Document, edit: impl FnOnce(&str) -> String) -> Result<(), String> {
    let contents = document.get_object(first_page(document)).and_then(Object::as_dict).and_then(|page| page.get(b"Contents")).and_then(Object::as_reference).map_err(|error| error.to_string())?;
    let stream = document.get_object_mut(contents).and_then(Object::as_stream_mut).map_err(|error| error.to_string())?;
    let edited = edit(&String::from_utf8_lossy(&stream.content));
    stream.set_content(edited.into_bytes());
    Ok(())
}

/// 🎲️ ISO 32000-1 §7.6.3.4 (Algorithm 5) ends `/U` with 16 bytes of ARBITRARY padding, which lopdf
/// draws at random; a reader ignores them. Zeroing them through the COS API is one valid choice
/// among the ones the standard allows, and it is what keeps this pair byte-reproducible.
fn settle_user_padding(document: &mut Document) -> Result<(), String> {
    let encrypt = document.trailer.get(b"Encrypt").and_then(Object::as_reference).map_err(|error| error.to_string())?;
    if let Ok(Object::String(user, _)) = dict_mut(document, encrypt)?.get_mut(b"U") {
        user[16..].fill(0);
    }
    Ok(())
}

/// 🌾️ The preconditions of the resource, catalog and page kinds: whatever a `remove-*` or an in-place
/// edit takes away must be there first, written by lopdf like everything else.
fn arrange_extended(kind: &str, document: &mut Document) -> Result<(), String> {
    if let Some((category, name)) = resource_slot(kind).filter(|_| kind.starts_with("remove-")) {
        let value = resource_value(document, category, name);
        return set_resource(document, category, name, Some(value));
    }
    match kind {
        "remove-annotation" | "set-annotation" => {
            let page = first_page(document);
            let annotation = annotation(document, "semio note");
            dict_mut(document, page)?.set("Annots", vec![Object::Reference(annotation)]);
        }
        "remove-named-destination" => {
            let page = kids(document)[1];
            set_name_tree(document, "Dests", vec![("chapter-one", vec![Object::Reference(page), "Fit".into()].into())])?;
        }
        "remove-embedded-file" => {
            let file = embedded_file(document);
            set_name_tree(document, "EmbeddedFiles", vec![("notes.txt", file)])?;
        }
        "remove-catalog-entry" => catalog_mut(document)?.set("SemioCustom", Object::string_literal("catalog entry")),
        "set-encryption" => document.trailer.set("ID", vec![hex(&[0x5e; 16]), hex(&[0x5e; 16])]),
        _ => {}
    }
    Ok(())
}

/// ✍️ The forward resource, catalog and page kinds, each an ordinary COS edit through lopdf.
fn apply_extended(kind: &str, document: &mut Document) -> Result<(), String> {
    if let Some((category, name)) = resource_slot(kind) {
        let value = kind.starts_with("set-").then(|| resource_value(document, category, name));
        return set_resource(document, category, name, value);
    }
    let page = first_page(document);
    match kind {
        "set-page-box" => dict_mut(document, page)?.set("TrimBox", vec![10.into(), 10.into(), 602.into(), 782.into()]),
        "set-page-user-unit" => dict_mut(document, page)?.set("UserUnit", Object::Real(2.0)),
        "insert-content" => edit_first_page_content(document, |content| format!("q 0.5 g 10 10 50 50 re f Q\n{content}"))?,
        "replace-content" => edit_first_page_content(document, |content| content.replacen("72 720 Td", "144 360 Td", 1))?,
        "remove-content" => edit_first_page_content(document, |content| match (content.find('('), content.find(") Tj")) {
            (Some(open), Some(close)) => format!("{}{}", &content[..open], content[close + ") Tj".len()..].trim_start()),
            _ => content.to_string(),
        })?,
        "insert-annotation" => {
            let annotation = annotation(document, "semio note");
            dict_mut(document, page)?.set("Annots", vec![Object::Reference(annotation)]);
        }
        "set-annotation" => {
            let annotation = first_annotation(document)?;
            dict_mut(document, annotation)?.set("Contents", Object::string_literal("changed note"));
        }
        "remove-annotation" => {
            dict_mut(document, page)?.remove(b"Annots");
        }
        "set-embedded-file" => {
            let file = embedded_file(document);
            set_name_tree(document, "EmbeddedFiles", vec![("notes.txt", file)])?;
        }
        "remove-embedded-file" => set_name_tree(document, "EmbeddedFiles", Vec::new())?,
        "set-outlines" => {
            let outlines = document.new_object_id();
            let item = document.add_object(dictionary! { "Title" => Object::string_literal("Start"), "Parent" => outlines, "Dest" => vec![Object::Reference(page), "Fit".into()] });
            document.objects.insert(outlines, Object::Dictionary(dictionary! { "Type" => "Outlines", "First" => item, "Last" => item, "Count" => 1 }));
            catalog_mut(document)?.set("Outlines", outlines);
        }
        "set-named-destination" => {
            let second = kids(document)[1];
            set_name_tree(document, "Dests", vec![("chapter-one", vec![Object::Reference(second), "Fit".into()].into())])?;
        }
        "remove-named-destination" => set_name_tree(document, "Dests", Vec::new())?,
        "set-page-labels" => catalog_mut(document)?.set("PageLabels", dictionary! { "Nums" => vec![0.into(), dictionary! { "S" => "r" }.into()] }),
        "set-output-intents" => catalog_mut(document)?.set(
            "OutputIntents",
            vec![dictionary! { "Type" => "OutputIntent", "S" => "GTS_PDFA1", "OutputConditionIdentifier" => Object::string_literal("sRGB IEC61966-2.1"), "RegistryName" => Object::string_literal("http://www.color.org"), "Info" => Object::string_literal("sRGB") }.into()],
        ),
        "set-acro-form" => catalog_mut(document)?.set("AcroForm", dictionary! { "Fields" => Vec::<Object>::new(), "NeedAppearances" => true }),
        "set-optional-content" => {
            let group = document.add_object(dictionary! { "Type" => "OCG", "Name" => Object::string_literal("Semio layer") });
            catalog_mut(document)?.set("OCProperties", dictionary! { "OCGs" => vec![Object::Reference(group)], "D" => dictionary! { "Order" => vec![Object::Reference(group)], "ON" => vec![Object::Reference(group)] } });
        }
        "set-page-layout" => catalog_mut(document)?.set("PageLayout", "TwoColumnLeft"),
        "set-page-mode" => catalog_mut(document)?.set("PageMode", "FullScreen"),
        "set-viewer-preferences" => catalog_mut(document)?.set("ViewerPreferences", dictionary! { "HideToolbar" => true, "DisplayDocTitle" => true }),
        "set-open-action" => catalog_mut(document)?.set("OpenAction", vec![Object::Reference(page), "Fit".into()]),
        "set-language" => catalog_mut(document)?.set("Lang", Object::string_literal("de-CH")),
        "set-mark-info" => catalog_mut(document)?.set("MarkInfo", dictionary! { "Marked" => true }),
        "set-metadata" => {
            let xmp = b"<?xpacket begin=\"\" id=\"W5M0MpCehiHzreSzNTczkc9d\"?><x:xmpmeta xmlns:x=\"adobe:ns:meta/\"><rdf:RDF xmlns:rdf=\"http://www.w3.org/1999/02/22-rdf-syntax-ns#\"><rdf:Description rdf:about=\"\" xmlns:dc=\"http://purl.org/dc/elements/1.1/\"><dc:format>application/pdf</dc:format></rdf:Description></rdf:RDF></x:xmpmeta><?xpacket end=\"w\"?>".to_vec();
            let metadata = document.add_object(Stream::new(dictionary! { "Type" => "Metadata", "Subtype" => "XML" }, xmp));
            catalog_mut(document)?.set("Metadata", metadata);
        }
        "set-document-id" => document.trailer.set("ID", vec![hex(&[0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef]), hex(&[0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54, 0x32, 0x10, 0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54, 0x32, 0x10])]),
        "set-encryption" => {
            let state = EncryptionState::try_from(EncryptionVersion::V2 { document: &*document, owner_password: "semio-owner", user_password: "", key_length: 128, permissions: Permissions::default() }).map_err(|error| error.to_string())?;
            document.encrypt(&state).map_err(|error| error.to_string())?;
            settle_user_padding(document)?;
        }
        "set-catalog-entry" => catalog_mut(document)?.set("SemioCustom", Object::string_literal("catalog entry")),
        "remove-catalog-entry" => {
            catalog_mut(document)?.remove(b"SemioCustom");
        }
        other => return Err(format!("unknown kind {other}")),
    }
    Ok(())
}
//#endregion 🧩️ResourceCatalogAndPageKinds

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
        other => arrange_extended(other, &mut document).expect("lopdf arranges the extended kind's precondition"),
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
        other => apply_extended(other, &mut document)?,
    }
    let mut out = Vec::new();
    document.save_to(&mut out).map_err(|error| error.to_string())?;
    Ok(out)
}

/// 🔣️ A canonical rendering of one COS value for the projection: dictionary keys sorted, strings as
/// hex, page references as their page index, other references resolved `depth` levels deep, and the
/// writer's own bookkeeping (`/Length`, `/Parent`) left out — so the rendering states what a reader
/// sees and never which object number a writer happened to pick.
fn render(document: &Document, object: &Object, pages: &[ObjectId], depth: usize) -> String {
    match object {
        Object::Null => "null".to_string(),
        Object::Boolean(value) => value.to_string(),
        Object::Integer(value) => value.to_string(),
        Object::Real(value) => format!("{value}"),
        Object::Name(name) => format!("\"/{}\"", escape(&String::from_utf8_lossy(name))),
        Object::String(bytes, _) => format!("\"<{}>\"", bytes.iter().map(|byte| format!("{byte:02x}")).collect::<String>()),
        Object::Array(items) => format!("[{}]", items.iter().map(|item| render(document, item, pages, depth)).collect::<Vec<_>>().join(",")),
        Object::Dictionary(dict) => render_dict(document, dict, pages, depth),
        Object::Stream(stream) => format!("{{\"dict\":{},\"data\":\"<{}>\"}}", render_dict(document, &stream.dict, pages, depth), stream.content.iter().map(|byte| format!("{byte:02x}")).collect::<String>()),
        Object::Reference(id) => match pages.iter().position(|page| page == id) {
            Some(index) => format!("\"page:{index}\""),
            None if depth == 0 => "\"ref\"".to_string(),
            None => document.get_object(*id).map(|target| render(document, target, pages, depth - 1)).unwrap_or_else(|_| "\"dangling\"".to_string()),
        },
    }
}

fn render_dict(document: &Document, dict: &Dictionary, pages: &[ObjectId], depth: usize) -> String {
    let mut entries: Vec<(String, String)> = dict.iter().filter(|(key, _)| !matches!(key.as_slice(), b"Length" | b"Parent")).map(|(key, value)| (String::from_utf8_lossy(key).to_string(), render(document, value, pages, depth))).collect();
    entries.sort();
    format!("{{{}}}", entries.iter().map(|(key, value)| format!("\"{}\":{value}", escape(key))).collect::<Vec<_>>().join(","))
}

/// 📄️ The structural projection: the page list in ORDER (so `move-page` is observable), each page's
/// boxes, rotation and content bytes, the `/Info` entries, and the catalog and trailer marker entries.
pub fn project(bytes: &[u8]) -> Result<String, String> {
    let document = Document::load_mem(bytes).map_err(|error| error.to_string())?;
    let order = kids(&document);
    let mut out = String::from("{\"subset\":\"base\"");
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

    // 🧩️The surfaces the resource, catalog and page kinds edit: every page dictionary except its
    // content (projected above), the catalog except its page tree, the trailer /ID and whether the file
    // was encrypted. lopdf decrypts on load, so an encrypted file projects its plain values.
    let page_dicts: Vec<String> = order
        .iter()
        .map(|page| {
            let mut dict = document.get_object(*page).and_then(Object::as_dict).cloned().unwrap_or_default();
            dict.remove(b"Contents");
            render_dict(&document, &dict, &order, 4)
        })
        .collect();
    out.push_str(&format!(",\"pageDictionaries\":[{}]", page_dicts.join(",")));
    let mut catalog = catalog.clone();
    catalog.remove(b"Pages");
    out.push_str(&format!(",\"catalog\":{}", render_dict(&document, &catalog, &order, 5)));
    out.push_str(&format!(",\"trailerId\":{}", document.trailer.get(b"ID").map(|id| render(&document, id, &order, 0)).unwrap_or_else(|_| "null".to_string())));
    out.push_str(&format!(",\"encrypted\":{}", document.was_encrypted() || document.is_encrypted()));
    out.push('}');
    Ok(out)
}
