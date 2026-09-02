//! 📄️ The `pdf@1.7/a` mutation vocabulary and conformance projection, expressed ENTIRELY through
//! `lopdf` 0.44's own public COS API.
//!
//! Both halves matter and neither may come from this repository:
//!
//! * [`apply`] performs each of the eighteen declared mutations by editing the object graph through
//!   `lopdf` — never through `semio-s-plugin-stdio-test-oracle`'s `oracle_apply_mutation`, which is
//!   what made the previous fixture corpus inadmissible as evidence.
//! * [`project`] reads the conformance axes back through `lopdf`, so the expected state is the
//!   committed `after` half of a fixture rather than something we computed.
//!
//! @see ../../../🦀️oracle.rs — the subset's own vocabulary and conformance axes.

use lopdf::{dictionary, Dictionary, Document, Object, ObjectId, Stream};

/// 🧾️ Kebab-case spelling of every kind this subset declares, in declaration order.
pub const KINDS: &[&str] = &[
    "insert-encryption-dictionary",
    "remove-encryption-dictionary",
    "insert-javascript-action",
    "remove-javascript-action",
    "insert-launch-action",
    "remove-launch-action",
    "insert-embedded-file",
    "remove-embedded-file",
    "set-af-relationship",
    "remove-af-relationship",
    "set-output-intent",
    "remove-output-intent",
    "embed-font-file",
    "remove-font-file",
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

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn catalog_id(document: &Document) -> ObjectId {
    document.trailer.get(b"Root").and_then(Object::as_reference).expect("a trailer /Root")
}

fn page_ids(document: &Document) -> Vec<ObjectId> {
    document.get_pages().values().copied().collect()
}

fn descriptor_ids(document: &Document) -> Vec<ObjectId> {
    let mut ids: Vec<ObjectId> = document
        .objects
        .iter()
        .filter(|(_, object)| {
            object
                .as_dict()
                .ok()
                .and_then(|d| d.get(b"Type").ok())
                .and_then(|t| t.as_name().ok())
                .map(|name| name == b"FontDescriptor")
                .unwrap_or(false)
        })
        .map(|(id, _)| *id)
        .collect();
    ids.sort_unstable();
    ids
}

/// 🌱️ The deterministic seed document — one page, two `/FontDescriptor` objects each carrying a tiny
/// `/FontFile2`, an `/Info /Title`. Two descriptors, not one: `embed-font-file` strips descriptor 0's
/// own program during arrangement, and the donor program it re-links has to still exist. No wall-clock
/// and no randomness, so the bytes are reproducible on every run.
pub fn build_seed() -> Vec<u8> {
    let mut document = Document::with_version("1.7");
    let pages_id = document.new_object_id();
    let content_id = document.add_object(Stream::new(dictionary! {}, b"BT /F1 12 Tf 72 720 Td (pdf-1-7-a fixture seed) Tj ET".to_vec()));
    let page_id = document.add_object(dictionary! {
        "Type" => "Page",
        "Parent" => pages_id,
        "Contents" => content_id,
        "MediaBox" => vec![0.into(), 0.into(), Object::Real(612.0), Object::Real(792.0)],
        "Resources" => dictionary! {},
    });
    document.objects.insert(pages_id, Object::Dictionary(dictionary! { "Type" => "Pages", "Kids" => vec![page_id.into()], "Count" => 1 }));

    let font_a = document.add_object(Stream::new(dictionary! {}, b"FONT-PROGRAM-A-0000000000000000".to_vec()));
    let font_b = document.add_object(Stream::new(dictionary! {}, b"FONT-PROGRAM-B-1111111111111111".to_vec()));
    document.add_object(dictionary! { "Type" => "FontDescriptor", "FontFile2" => font_a });
    document.add_object(dictionary! { "Type" => "FontDescriptor", "FontFile2" => font_b });

    let catalog = document.add_object(dictionary! { "Type" => "Catalog", "Pages" => pages_id });
    let info = document.add_object(dictionary! {
        "Title" => Object::string_literal("pdf-1-7-a fixture seed"),
        "Author" => Object::string_literal("pdf-1-7-a-lopdf-engine"),
    });
    document.trailer.set("Root", catalog);
    document.trailer.set("Info", info);

    let mut out = Vec::new();
    document.save_to(&mut out).expect("lopdf saves the document it just built");
    out
}

fn set_catalog(document: &mut Document, key: &str, value: Object) {
    let id = catalog_id(document);
    if let Ok(dict) = document.get_object_mut(id).and_then(Object::as_dict_mut) {
        dict.set(key, value);
    }
}

fn remove_catalog(document: &mut Document, key: &str) {
    let id = catalog_id(document);
    if let Ok(dict) = document.get_object_mut(id).and_then(Object::as_dict_mut) {
        dict.remove(key.as_bytes());
    }
}

fn output_intent(document: &mut Document, identifier: &str) -> Object {
    let profile = document.add_object(Stream::new(dictionary! { "N" => 3 }, b"ICC-PROFILE-BYTES".to_vec()));
    Object::Array(vec![Object::Dictionary(dictionary! {
        "Type" => "OutputIntent",
        "S" => "GTS_PDFX",
        "OutputConditionIdentifier" => Object::string_literal(identifier),
        "DestOutputProfile" => profile,
    })])
}


fn embedded_filespec(document: &mut Document, relationship: Option<&str>) -> ObjectId {
    let stream = document.add_object(Stream::new(dictionary! { "Type" => "EmbeddedFile" }, b"ATTACHED-PAYLOAD-BYTES".to_vec()));
    let mut spec = dictionary! {
        "Type" => "Filespec",
        "F" => Object::string_literal("attachment.txt"),
        "EF" => dictionary! { "F" => stream },
    };
    if let Some(role) = relationship {
        spec.set("AFRelationship", Object::Name(role.as_bytes().to_vec()));
    }
    document.add_object(spec)
}

fn attach_embedded_file(document: &mut Document, relationship: Option<&str>) {
    let spec = embedded_filespec(document, relationship);
    let tree = document.add_object(dictionary! { "Names" => vec![Object::string_literal("attachment.txt"), spec.into()] });
    let names = document.add_object(dictionary! { "EmbeddedFiles" => tree });
    set_catalog(document, "Names", Object::Reference(names));
    set_catalog(document, "AF", Object::Array(vec![Object::Reference(spec)]));
}

/// 🌾️ ARRANGEMENT — the precondition a kind needs before its forward mutation is meaningful.
/// A `remove-*` needs the thing present; `embed-font-file` needs descriptor 0's own program absent so
/// that re-embedding it is observable rather than a no-op.
pub fn arrange(kind: &str, bytes: &[u8]) -> Vec<u8> {
    let mut document = Document::load_mem(bytes).expect("lopdf reloads the seed it wrote");
    match kind {
        "remove-encryption-dictionary" => {
            let encrypt = document.add_object(dictionary! { "Filter" => "Standard", "V" => 2, "R" => 3 });
            document.trailer.set("Encrypt", encrypt);
        }
        "remove-output-intent" => {
            let intents = output_intent(&mut document, "sRGB IEC61966-2.1");
            set_catalog(&mut document, "OutputIntents", intents);
        }
        "remove-trim-box" => {
            let page = page_ids(&document)[0];
            if let Ok(dict) = document.get_object_mut(page).and_then(Object::as_dict_mut) {
                dict.set("TrimBox", vec![Object::Real(8.5), Object::Real(8.5), Object::Real(586.776), Object::Real(833.39)]);
            }
        }
        "embed-font-file" => {
            let descriptor = descriptor_ids(&document)[0];
            if let Ok(dict) = document.get_object_mut(descriptor).and_then(Object::as_dict_mut) {
                dict.remove(b"FontFile2");
            }
        }
        "remove-javascript-action" => {
            let action = document.add_object(dictionary! { "S" => "JavaScript", "JS" => Object::string_literal("app.alert('this document phones home');") });
            let names = document.add_object(dictionary! { "Names" => vec![Object::string_literal("EmbeddedJS"), action.into()] });
            let root = document.add_object(dictionary! { "JavaScript" => names });
            set_catalog(&mut document, "Names", Object::Reference(root));
        }
        "remove-launch-action" => {
            let action = document.add_object(dictionary! { "S" => "Launch", "F" => Object::string_literal("render-plots.bat") });
            let page = page_ids(&document)[0];
            let annotation = document.add_object(dictionary! { "Type" => "Annot", "Subtype" => "Link", "A" => action });
            if let Ok(dict) = document.get_object_mut(page).and_then(Object::as_dict_mut) {
                dict.set("Annots", vec![annotation.into()]);
            }
        }
        "remove-media-annotation" => {
            let page = page_ids(&document)[0];
            let annotation = document.add_object(dictionary! {
                "Type" => "Annot",
                "Subtype" => "Screen",
                "T" => Object::string_literal("narration"),
                "MediaType" => "Sound",
            });
            if let Ok(dict) = document.get_object_mut(page).and_then(Object::as_dict_mut) {
                dict.set("Annots", vec![annotation.into()]);
            }
        }
        "remove-embedded-file" | "remove-af-relationship" => attach_embedded_file(&mut document, Some("Data")),
        "set-af-relationship" => attach_embedded_file(&mut document, None),
        "remove-dpart-root" | "remove-dpart-metadata" => {
            let metadata = document.add_object(Stream::new(dictionary! { "Type" => "Metadata", "Subtype" => "XML" }, b"<dpart job=\"run 4711, recipient block A\"/>".to_vec()));
            let part = document.add_object(dictionary! { "Type" => "DPart", "Metadata" => metadata });
            let root = document.add_object(dictionary! { "Type" => "DPartRoot", "DPartRootNode" => part });
            set_catalog(&mut document, "DPartRoot", Object::Reference(root));
        }
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
        "insert-encryption-dictionary" => {
            let encrypt = document.add_object(dictionary! { "Filter" => "Standard", "V" => 2, "R" => 3 });
            document.trailer.set("Encrypt", encrypt);
        }
        "remove-encryption-dictionary" => {
            document.trailer.remove(b"Encrypt");
        }
        "set-output-intent" => {
            let intents = output_intent(&mut document, "sRGB IEC61966-2.1");
            set_catalog(&mut document, "OutputIntents", intents);
        }
        "remove-output-intent" => remove_catalog(&mut document, "OutputIntents"),
        "set-trim-box" => {
            let page = page_ids(&document)[0];
            let dict = document.get_object_mut(page).and_then(Object::as_dict_mut).map_err(|e| e.to_string())?;
            dict.set("TrimBox", vec![Object::Real(8.5), Object::Real(8.5), Object::Real(586.776), Object::Real(833.39)]);
        }
        "remove-trim-box" => {
            let page = page_ids(&document)[0];
            let dict = document.get_object_mut(page).and_then(Object::as_dict_mut).map_err(|e| e.to_string())?;
            dict.remove(b"TrimBox");
        }
        "embed-font-file" => {
            // 🔗️Re-embeds the DONOR program (descriptor ordinal 1's) into descriptor ordinal 0, whose own
            // program `arrange` stripped — so the mutation is an observable re-linking, not a no-op.
            let ids = descriptor_ids(&document);
            let donor = document
                .get_object(ids[1])
                .and_then(Object::as_dict)
                .map_err(|e| e.to_string())?
                .get(b"FontFile2")
                .and_then(Object::as_reference)
                .map_err(|e| e.to_string())?;
            let dict = document.get_object_mut(ids[0]).and_then(Object::as_dict_mut).map_err(|e| e.to_string())?;
            dict.set("FontFile2", Object::Reference(donor));
        }
        "remove-font-file" => {
            let ids = descriptor_ids(&document);
            let dict = document.get_object_mut(ids[0]).and_then(Object::as_dict_mut).map_err(|e| e.to_string())?;
            dict.remove(b"FontFile2");
        }
        "insert-javascript-action" => {
            let action = document.add_object(dictionary! { "S" => "JavaScript", "JS" => Object::string_literal("app.alert('this document phones home');") });
            let names = document.add_object(dictionary! { "Names" => vec![Object::string_literal("EmbeddedJS"), action.into()] });
            let root = document.add_object(dictionary! { "JavaScript" => names });
            set_catalog(&mut document, "Names", Object::Reference(root));
        }
        "remove-javascript-action" => remove_catalog(&mut document, "Names"),
        "insert-launch-action" => {
            let action = document.add_object(dictionary! { "S" => "Launch", "F" => Object::string_literal("render-plots.bat") });
            let page = page_ids(&document)[0];
            let annotation = document.add_object(dictionary! { "Type" => "Annot", "Subtype" => "Link", "A" => action });
            let dict = document.get_object_mut(page).and_then(Object::as_dict_mut).map_err(|e| e.to_string())?;
            dict.set("Annots", vec![annotation.into()]);
        }
        "remove-launch-action" | "remove-media-annotation" => {
            let page = page_ids(&document)[0];
            let dict = document.get_object_mut(page).and_then(Object::as_dict_mut).map_err(|e| e.to_string())?;
            dict.remove(b"Annots");
        }
        "insert-media-annotation" => {
            let page = page_ids(&document)[0];
            let annotation = document.add_object(dictionary! {
                "Type" => "Annot",
                "Subtype" => "Screen",
                "T" => Object::string_literal("site walkthrough"),
                "MediaType" => "Movie",
            });
            let dict = document.get_object_mut(page).and_then(Object::as_dict_mut).map_err(|e| e.to_string())?;
            dict.set("Annots", vec![annotation.into()]);
        }
        "insert-embedded-file" => attach_embedded_file(&mut document, Some("Data")),
        "remove-embedded-file" => {
            remove_catalog(&mut document, "Names");
            remove_catalog(&mut document, "AF");
        }
        "set-af-relationship" => {
            let spec = {
                let catalog = document.get_object(catalog_id(&document)).and_then(Object::as_dict).map_err(|e| e.to_string())?;
                catalog.get(b"AF").and_then(Object::as_array).map_err(|e| e.to_string())?[0].as_reference().map_err(|e| e.to_string())?
            };
            let dict = document.get_object_mut(spec).and_then(Object::as_dict_mut).map_err(|e| e.to_string())?;
            dict.set("AFRelationship", Object::Name(b"Source".to_vec()));
        }
        "remove-af-relationship" => {
            let spec = {
                let catalog = document.get_object(catalog_id(&document)).and_then(Object::as_dict).map_err(|e| e.to_string())?;
                catalog.get(b"AF").and_then(Object::as_array).map_err(|e| e.to_string())?[0].as_reference().map_err(|e| e.to_string())?
            };
            let dict = document.get_object_mut(spec).and_then(Object::as_dict_mut).map_err(|e| e.to_string())?;
            dict.remove(b"AFRelationship");
        }
        "set-dpart-root" => {
            let metadata = document.add_object(Stream::new(dictionary! { "Type" => "Metadata", "Subtype" => "XML" }, b"<dpart job=\"run 4711, recipient block A\"/>".to_vec()));
            let part = document.add_object(dictionary! { "Type" => "DPart", "Metadata" => metadata });
            let root = document.add_object(dictionary! { "Type" => "DPartRoot", "DPartRootNode" => part });
            set_catalog(&mut document, "DPartRoot", Object::Reference(root));
        }
        "remove-dpart-root" => remove_catalog(&mut document, "DPartRoot"),
        "set-dpart-metadata" => {
            let metadata = document.add_object(Stream::new(dictionary! { "Type" => "Metadata", "Subtype" => "XML" }, b"<dpart job=\"run 4712, recipient block B\"/>".to_vec()));
            let part = document.add_object(dictionary! { "Type" => "DPart", "Metadata" => metadata });
            let root = document.add_object(dictionary! { "Type" => "DPartRoot", "DPartRootNode" => part });
            set_catalog(&mut document, "DPartRoot", Object::Reference(root));
        }
        "remove-dpart-metadata" => {
            let root = {
                let catalog = document.get_object(catalog_id(&document)).and_then(Object::as_dict).map_err(|e| e.to_string())?;
                catalog.get(b"DPartRoot").and_then(Object::as_reference).map_err(|e| e.to_string())?
            };
            let node = document.get_object(root).and_then(Object::as_dict).map_err(|e| e.to_string())?.get(b"DPartRootNode").and_then(Object::as_reference).map_err(|e| e.to_string())?;
            let dict = document.get_object_mut(node).and_then(Object::as_dict_mut).map_err(|e| e.to_string())?;
            dict.remove(b"Metadata");
        }
        other => return Err(format!("unknown kind {other}")),
    }
    let mut out = Vec::new();
    document.save_to(&mut out).map_err(|error| error.to_string())?;
    Ok(out)
}

fn annots_of(document: &Document, page: ObjectId) -> Vec<Dictionary> {
    let Ok(dict) = document.get_object(page).and_then(Object::as_dict) else { return Vec::new() };
    let Ok(array) = dict.get(b"Annots").and_then(Object::as_array) else { return Vec::new() };
    array
        .iter()
        .filter_map(|entry| match entry {
            Object::Reference(id) => document.get_object(*id).ok().and_then(|o| o.as_dict().ok()).cloned(),
            Object::Dictionary(d) => Some(d.clone()),
            _ => None,
        })
        .collect()
}

/// 📄️ The conformance projection, read back through `lopdf` — the SAME axes the subset's own checker
/// reads, so this subset is never judged on an axis it does not declare.
pub fn project(bytes: &[u8]) -> Result<String, String> {
    let document = Document::load_mem(bytes).map_err(|error| error.to_string())?;
    let catalog = document.get_object(catalog_id(&document)).and_then(Object::as_dict).map_err(|e| e.to_string())?.clone();
    let pages = page_ids(&document);

    let mut out = String::from("{\"subset\":\"a\"");
    out.push_str(&format!(",\"pageCount\":{}", pages.len()));

    let encrypted = document.trailer.get(b"Encrypt").is_ok();
    out.push_str(&format!(",\"encryptionDictionaries\":{}", if encrypted { "[{\"present\":true}]" } else { "[]" }));

    out.push_str(",\"outputIntents\":[");
    if let Ok(array) = catalog.get(b"OutputIntents").and_then(Object::as_array) {
        for (index, entry) in array.iter().enumerate() {
            let dict = match entry {
                Object::Reference(id) => document.get_object(*id).ok().and_then(|o| o.as_dict().ok()).cloned(),
                Object::Dictionary(d) => Some(d.clone()),
                _ => None,
            };
            let Some(dict) = dict else { continue };
            if index > 0 {
                out.push(',');
            }
            let identifier = dict.get(b"OutputConditionIdentifier").and_then(Object::as_str).map(|s| String::from_utf8_lossy(s).to_string()).unwrap_or_default();
            let subtype = dict.get(b"S").and_then(Object::as_name).map(|s| String::from_utf8_lossy(s).to_string()).unwrap_or_default();
            out.push_str(&format!("{{\"subtype\":\"{}\",\"identifier\":\"{}\",\"hasDestProfile\":{}}}", escape(&subtype), escape(&identifier), dict.get(b"DestOutputProfile").is_ok()));
        }
    }
    out.push(']');

    out.push_str(",\"pageBoxes\":[");
    for (index, page) in pages.iter().enumerate() {
        if index > 0 {
            out.push(',');
        }
        let dict = document.get_object(*page).and_then(Object::as_dict).map_err(|e| e.to_string())?;
        let boxed = |key: &[u8]| -> String {
            match dict.get(key).and_then(Object::as_array) {
                Ok(values) => {
                    let parts: Vec<String> = values.iter().map(|v| v.as_float().map(|f| format!("{f}")).unwrap_or_else(|_| "null".to_string())).collect();
                    format!("[{}]", parts.join(","))
                }
                Err(_) => "null".to_string(),
            }
        };
        out.push_str(&format!("{{\"trimBox\":{},\"artBox\":{}}}", boxed(b"TrimBox"), boxed(b"ArtBox")));
    }
    out.push(']');

    out.push_str(",\"fontPrograms\":[");
    for (index, descriptor) in descriptor_ids(&document).iter().enumerate() {
        let dict = document.get_object(*descriptor).and_then(Object::as_dict).map_err(|e| e.to_string())?;
        let Ok(reference) = dict.get(b"FontFile2").and_then(Object::as_reference) else { continue };
        let Ok(stream) = document.get_object(reference).and_then(Object::as_stream) else { continue };
        if index > 0 && !out.ends_with('[') {
            out.push(',');
        }
        out.push_str(&format!("{{\"key\":\"FontFile2\",\"programBytes\":{},\"programHex\":\"{}\"}}", stream.content.len(), hex(&stream.content)));
    }
    out.push(']');

    let has_js = catalog
        .get(b"Names")
        .and_then(Object::as_reference)
        .ok()
        .and_then(|id| document.get_object(id).ok())
        .and_then(|o| o.as_dict().ok())
        .map(|d| d.get(b"JavaScript").is_ok())
        .unwrap_or(false);
    out.push_str(&format!(",\"javaScriptActions\":{}", if has_js { "[{\"present\":true}]" } else { "[]" }));

    let mut launches = 0usize;
    let mut media = Vec::<String>::new();
    for page in &pages {
        for annotation in annots_of(&document, *page) {
            if let Ok(action) = annotation.get(b"A").and_then(Object::as_reference) {
                if let Ok(dict) = document.get_object(action).and_then(Object::as_dict) {
                    if dict.get(b"S").and_then(Object::as_name).map(|n| n == b"Launch").unwrap_or(false) {
                        launches += 1;
                    }
                }
            }
            if annotation.get(b"Subtype").and_then(Object::as_name).map(|n| n == b"Screen").unwrap_or(false) {
                let title = annotation.get(b"T").and_then(Object::as_str).map(|s| String::from_utf8_lossy(s).to_string()).unwrap_or_default();
                let media_type = annotation.get(b"MediaType").and_then(Object::as_name).map(|s| String::from_utf8_lossy(s).to_string()).unwrap_or_default();
                media.push(format!("{{\"title\":\"{}\",\"mediaType\":\"{}\"}}", escape(&title), escape(&media_type)));
            }
        }
    }
    out.push_str(&format!(",\"launchActions\":{}", if launches > 0 { format!("[{{\"count\":{launches}}}]") } else { "[]".to_string() }));
    out.push_str(&format!(",\"mediaAnnotations\":[{}]", media.join(",")));

    out.push_str(",\"embeddedFiles\":[");
    if let Ok(array) = catalog.get(b"AF").and_then(Object::as_array) {
        for (index, entry) in array.iter().enumerate() {
            let Ok(id) = entry.as_reference() else { continue };
            let Ok(spec) = document.get_object(id).and_then(Object::as_dict) else { continue };
            if index > 0 {
                out.push(',');
            }
            let name = spec.get(b"F").and_then(Object::as_str).map(|s| String::from_utf8_lossy(s).to_string()).unwrap_or_default();
            let relationship = match spec.get(b"AFRelationship").and_then(Object::as_name) {
                Ok(role) => format!("\"{}\"", escape(&String::from_utf8_lossy(role))),
                Err(_) => "null".to_string(),
            };
            out.push_str(&format!("{{\"name\":\"{}\",\"afRelationship\":{}}}", escape(&name), relationship));
        }
    }
    out.push(']');

    let dpart = catalog
        .get(b"DPartRoot")
        .and_then(Object::as_reference)
        .ok()
        .and_then(|id| document.get_object(id).ok())
        .and_then(|o| o.as_dict().ok())
        .cloned();
    match dpart {
        Some(root) => {
            let metadata = root
                .get(b"DPartRootNode")
                .and_then(Object::as_reference)
                .ok()
                .and_then(|id| document.get_object(id).ok())
                .and_then(|o| o.as_dict().ok())
                .and_then(|node| node.get(b"Metadata").and_then(Object::as_reference).ok())
                .and_then(|id| document.get_object(id).ok())
                .and_then(|o| o.as_stream().ok())
                .map(|stream| String::from_utf8_lossy(&stream.content).to_string());
            match metadata {
                Some(text) => out.push_str(&format!(",\"dpartRoot\":{{\"present\":true,\"metadata\":\"{}\"}}", escape(&text))),
                None => out.push_str(",\"dpartRoot\":{\"present\":true,\"metadata\":null}"),
            }
        }
        None => out.push_str(",\"dpartRoot\":null"),
    }

    out.push('}');
    Ok(out)
}
