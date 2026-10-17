//! 🔁️ Arrange, inverse and projection of the PDF engine: the arrange spec, the inverse spec and the semantic projection. See [`super`].

use super::*;

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
        "remove-display-doc-title" => vec![kind_spec("set-display-doc-title", vec![("display", Json::Bool(true))])],
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
            let (key, program, _, _) = font_program(&document, descriptor).ok_or_else(|| format!("remove-font-file has no inverse: descriptor ordinal {index} carries no embedded font program in the base"))?;
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
            Some(previous) => kind_spec("set-display-doc-title", vec![("display", Json::Bool(previous))]),
            None => kind_spec("remove-display-doc-title", vec![]),
        },
        "remove-display-doc-title" => kind_spec("set-display-doc-title", vec![("display", Json::Bool(true))]),
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
                        Some((key, _, size, sum)) => json_object(vec![("key", Json::String(key)), ("programBytes", Json::Number(size as f64)), ("programDigest", Json::String(sum))]),
                        None => json_object(vec![("key", Json::Null), ("programBytes", Json::Null), ("programDigest", Json::Null)]),
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
