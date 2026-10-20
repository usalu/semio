//! ⚙️ Forward mutation of the PDF engine: the conformance mutation entry and its in-place application. See [`super`].

use super::*;

//#region 🔖️Forward
/// 🦠️ Performs one declared conformance-class mutation with the independent `lopdf`
/// implementation and returns the re-serialized document. An unrecognised kind is an error,
/// never a silent no-op: a mutation that is quietly skipped reports as a passing test.
pub fn apply_conformance_mutation(input: &[u8], spec: &Json, profile: &PdfConformanceProfile) -> Result<Vec<u8>, String> {
    let kind = spec.str("kind");
    if kind.is_empty() {
        return Err("mutation spec carries no `kind`".to_string());
    }
    let mut document = load(input)?;
    apply_in_place(&mut document, &kind, &params_of(spec), profile)?;
    save(&mut document)
}

fn apply_in_place(document: &mut Document, kind: &str, params: &Json, profile: &PdfConformanceProfile) -> Result<(), String> {
    match kind {
        "insert-encryption-dictionary" => {
            let version = number(params, "version").unwrap_or(2.0) as i64;
            let revision = number(params, "revision").unwrap_or(3.0) as i64;
            if encryption_dictionaries(document).iter().any(|(_, v, r)| *v == version && *r == revision) {
                return Err(format!("insert-encryption-dictionary: the document already carries a /V {version} /R {revision} Standard Security Handler dictionary"));
            }
            document.add_object(encryption_dictionary(version, revision));
            Ok(())
        }
        "remove-encryption-dictionary" => apply_remove_encryption_dictionary(document, params),
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
        "insert-media-annotation" => apply_insert_media_annotation(document, kind, params),
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
        "insert-embedded-file" => apply_insert_embedded_file(document, params),
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
        "set-af-relationship" => apply_set_af_relationship(document, params),
        "remove-af-relationship" => apply_remove_af_relationship(document, params),
        "set-output-intent" => apply_set_output_intent(document, params, profile),
        "remove-output-intent" => {
            if output_intents(document).is_empty() {
                return Err("remove-output-intent: /Root carries no OutputIntents to remove".to_string());
            }
            let catalog = catalog_dict_mut(document)?;
            catalog.remove(b"OutputIntents");
            Ok(())
        }
        "embed-font-file" => apply_embed_font_file(document, params),
        "remove-font-file" => {
            let descriptor_ordinal = ordinal(params, "descriptorOrdinal")?;
            let descriptors = font_descriptors(document);
            let descriptor = *descriptors.get(descriptor_ordinal).ok_or_else(|| format!("remove-font-file: descriptor ordinal {descriptor_ordinal} is out of range ({} descriptors)", descriptors.len()))?;
            let (key, _, _, _) = font_program(document, descriptor).ok_or_else(|| format!("remove-font-file: descriptor ordinal {descriptor_ordinal} carries no embedded font program"))?;
            let dict = document.get_dictionary_mut(descriptor).map_err(|error| format!("remove-font-file: {error}"))?;
            dict.remove(key.as_bytes());
            Ok(())
        }
        "set-trim-box" => apply_set_trim_box(document, params),
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
        "set-struct-tree-root" => apply_set_struct_tree_root(document),
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
            let display = matches!(params.get("display"), Some(Json::Bool(true)));
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
        "insert-signature-field" => apply_insert_signature_field(document, params),
        "remove-signature-field" => apply_remove_signature_field(document, params),
        "set-dpart-root" => apply_set_dpart_root(document, params),
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

/// 🧱️ Runs the `remove encryption dictionary` kind of `apply_in_place`.
fn apply_remove_encryption_dictionary(document: &mut Document, params: &Json) -> Result<(), String> {
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

/// 🪵️ Runs the `insert media annotation` kind of `apply_in_place`.
fn apply_insert_media_annotation(document: &mut Document, kind: &str, params: &Json) -> Result<(), String> {
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

/// 🔩️ Runs the `insert embedded file` kind of `apply_in_place`.
fn apply_insert_embedded_file(document: &mut Document, params: &Json) -> Result<(), String> {
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

/// 🧲️ Runs the `set af relationship` kind of `apply_in_place`.
fn apply_set_af_relationship(document: &mut Document, params: &Json) -> Result<(), String> {
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

/// 🪛️ Runs the `remove af relationship` kind of `apply_in_place`.
fn apply_remove_af_relationship(document: &mut Document, params: &Json) -> Result<(), String> {
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

/// 🔧️ Runs the `set output intent` kind of `apply_in_place`.
fn apply_set_output_intent(document: &mut Document, params: &Json, profile: &PdfConformanceProfile) -> Result<(), String> {
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

/// 🔨️ Runs the `embed font file` kind of `apply_in_place`.
fn apply_embed_font_file(document: &mut Document, params: &Json) -> Result<(), String> {
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
    let program = program_reference(params)?;
    let dict = document.get_dictionary_mut(descriptor).map_err(|error| format!("embed-font-file: {error}"))?;
    dict.set(key, Object::Reference(program));
    Ok(())
}

/// 🪚️ Runs the `set trim box` kind of `apply_in_place`.
fn apply_set_trim_box(document: &mut Document, params: &Json) -> Result<(), String> {
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

/// 🧰️ Runs the `set struct tree root` kind of `apply_in_place`.
fn apply_set_struct_tree_root(document: &mut Document) -> Result<(), String> {
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

/// 🧪️ Runs the `insert signature field` kind of `apply_in_place`.
fn apply_insert_signature_field(document: &mut Document, params: &Json) -> Result<(), String> {
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

/// 🧫️ Runs the `remove signature field` kind of `apply_in_place`.
fn apply_remove_signature_field(document: &mut Document, params: &Json) -> Result<(), String> {
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

/// 🧬️ Runs the `set dpart root` kind of `apply_in_place`.
fn apply_set_dpart_root(document: &mut Document, params: &Json) -> Result<(), String> {
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

/// 🔤️ The donor font-program object `embed-font-file` points a descriptor at — the leaf wire's exact `program: {num,
/// gen}` reference, which a feature row and an engine-computed inverse both carry.
fn program_reference(params: &Json) -> Result<ObjectId, String> {
    let reference = params.get("program").ok_or("embed-font-file: `program` is required")?;
    let num = number(reference, "num").ok_or("embed-font-file: `program.num` must be a number")? as u32;
    let generation = number(reference, "gen").ok_or("embed-font-file: `program.gen` must be a number")? as u16;
    Ok((num, generation))
}
//#endregion 🔖️Forward
