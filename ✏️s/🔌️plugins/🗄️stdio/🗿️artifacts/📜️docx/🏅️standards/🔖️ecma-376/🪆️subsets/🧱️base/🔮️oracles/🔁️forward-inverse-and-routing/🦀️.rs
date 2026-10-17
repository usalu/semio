//! 🔁️ The forward mutation kinds, their computed inverses and the routing and projection entry points.

use super::*;

//#region 🔖️Forward
/// 🦠️ Applies one declared mutation kind, described by `spec` (`{"kind": ..., "params": {...}}`),
/// to an already-decoded package. An unrecognised kind is an error, never a silent no-op.
fn apply_kind(pkg: &mut WPackage, kind: &str, params: &Json) -> Result<(), String> {
    match kind {
        "insert-block" => {
            let path = json_to_path(&params.get("path").cloned().unwrap_or(Json::Null));
            let block = json_to_block(&params.get("block").cloned().unwrap_or(Json::Null))?;
            let list = resolve_blocks_mut(&mut pkg.body, &path.segments).ok_or("insert-block: path does not resolve to a block list")?;
            let index = path.index.min(list.len());
            list.insert(index, block);
        }
        "remove-block" => {
            let path = json_to_path(&params.get("path").cloned().unwrap_or(Json::Null));
            let list = resolve_blocks_mut(&mut pkg.body, &path.segments).ok_or("remove-block: path does not resolve to a block list")?;
            if path.index >= list.len() {
                return Err(format!("remove-block: index {} out of range (len {})", path.index, list.len()));
            }
            list.remove(path.index);
        }
        "set-block-content" => {
            let path = json_to_path(&params.get("path").cloned().unwrap_or(Json::Null));
            let block = json_to_block(&params.get("block").cloned().unwrap_or(Json::Null))?;
            let list = resolve_blocks_mut(&mut pkg.body, &path.segments).ok_or("set-block-content: path does not resolve to a block list")?;
            let slot = list.get_mut(path.index).ok_or_else(|| format!("set-block-content: index {} out of range", path.index))?;
            *slot = block;
        }
        "set-run-text" => {
            let (path, run_index) = resolve_run_address(pkg, params.get("address").unwrap_or(&Json::Null))?;
            let list = resolve_blocks_mut(&mut pkg.body, &path.segments).ok_or("set-run-text: path does not resolve to a block list")?;
            let WBlock::Paragraph(paragraph) = list.get_mut(path.index).ok_or("set-run-text: index out of range")? else { return Err("set-run-text: addressed block is not a paragraph".to_string()) };
            let run = paragraph.runs.get_mut(run_index).ok_or("set-run-text: run index out of range")?;
            run.text = params.str("text");
        }
        "set-run-formatting" => {
            let (path, run_index) = resolve_run_address(pkg, params.get("address").unwrap_or(&Json::Null))?;
            let list = resolve_blocks_mut(&mut pkg.body, &path.segments).ok_or("set-run-formatting: path does not resolve to a block list")?;
            let WBlock::Paragraph(paragraph) = list.get_mut(path.index).ok_or("set-run-formatting: index out of range")? else { return Err("set-run-formatting: addressed block is not a paragraph".to_string()) };
            let run = paragraph.runs.get_mut(run_index).ok_or("set-run-formatting: run index out of range")?;
            run.bold = bool_field(params, "bold");
            run.italic = bool_field(params, "italic");
            run.underline = bool_field(params, "underline");
        }
        "insert-style" => pkg.styles.push(json_to_style(&params.get("style").cloned().unwrap_or(Json::Null))),
        "remove-style" => {
            let id = params.str("id");
            pkg.styles.retain(|style| style.id != id);
        }
        "set-style-name" => {
            let id = params.str("id");
            let style = pkg.styles.iter_mut().find(|style| style.id == id).ok_or_else(|| format!("set-style-name: no style {id:?}"))?;
            style.name = params.str("name");
        }
        "set-style-based-on" => {
            let id = params.str("id");
            let style = pkg.styles.iter_mut().find(|style| style.id == id).ok_or_else(|| format!("set-style-based-on: no style {id:?}"))?;
            style.based_on = non_empty(params, "based_on");
        }
        "set-part" => {
            let path = params.str("path");
            let content_type = params.str("content_type");
            pkg.opc.set_part(&path, &content_type, byte_array(params.get("bytes"))?);
        }
        "remove-part" => {
            let path = params.str("path").trim_start_matches('/').to_string();
            pkg.opc.parts.retain(|part| part.path != path);
        }
        "set-relationship" => {
            let owner = params.str("owner");
            let relationship = ORel { id: params.str("id"), rel_type: params.str("relType"), target: params.str("target"), external: bool_field(params, "external") };
            let list = pkg.opc.relationships.entry(owner).or_default();
            match list.iter().position(|existing| existing.id == relationship.id) {
                Some(at) => list[at] = relationship,
                None => {
                    let at = optional_index(params).map_or(list.len(), |at| at.min(list.len()));
                    list.insert(at, relationship);
                }
            }
        }
        "remove-relationship" => {
            let owner = params.str("owner");
            let id = params.str("id");
            let list = pkg.opc.relationships.get_mut(&owner).ok_or_else(|| format!("remove-relationship: no relationships are owned by {owner:?}"))?;
            let at = list.iter().position(|existing| existing.id == id).ok_or_else(|| format!("remove-relationship: {owner:?} owns no relationship {id:?}"))?;
            list.remove(at);
            if list.is_empty() {
                pkg.opc.relationships.remove(&owner);
            }
        }
        "set-content-type" => {
            let is_override = bool_field(params, "isOverride");
            let name = params.str("name");
            let content_type = params.str("contentType");
            let entries = if is_override { &mut pkg.opc.content_types.overrides } else { &mut pkg.opc.content_types.defaults };
            if !is_override && entries.iter().any(|(key, _)| *key != name && key.eq_ignore_ascii_case(&name)) {
                return Err(format!("set-content-type: default extension {name:?} already exists in another letter case"));
            }
            match entries.iter().position(|(key, _)| *key == name) {
                Some(at) => entries[at].1 = content_type,
                None => {
                    let at = optional_index(params).map_or(entries.len(), |at| at.min(entries.len()));
                    entries.insert(at, (name, content_type));
                }
            }
        }
        "remove-content-type" => {
            let is_override = bool_field(params, "isOverride");
            let name = params.str("name");
            let entries = if is_override { &mut pkg.opc.content_types.overrides } else { &mut pkg.opc.content_types.defaults };
            let at = entries.iter().position(|(key, _)| *key == name).ok_or_else(|| format!("remove-content-type: the content types hold no {} {name:?}", if is_override { "override" } else { "default" }))?;
            entries.remove(at);
        }
        other => return Err(format!("mutation kind {other:?} has no oracle implementation")),
    }
    Ok(())
}
//#endregion 🔖️Forward

//#region 🔖️Inverse
/// 🔗️ The wire payload that writes `relationship` into `owner` at `index`.
fn relationship_params(owner: &str, relationship: &ORel, index: Option<usize>) -> Json {
    let mut entries = vec![
        ("owner".to_string(), Json::String(owner.to_string())),
        ("id".to_string(), Json::String(relationship.id.clone())),
        ("relType".to_string(), Json::String(relationship.rel_type.clone())),
        ("target".to_string(), Json::String(relationship.target.clone())),
        ("external".to_string(), Json::Bool(relationship.external)),
    ];
    if let Some(at) = index {
        entries.push(("index".to_string(), Json::Number(at as f64)));
    }
    Json::Object(entries)
}

/// 📇️ The wire payload that gives the `[Content_Types].xml` entry `name` the content type `content_type` at `index`.
fn content_type_params(is_override: bool, name: &str, content_type: &str, index: Option<usize>) -> Json {
    let mut entries = vec![("isOverride".to_string(), Json::Bool(is_override)), ("name".to_string(), Json::String(name.to_string())), ("contentType".to_string(), Json::String(content_type.to_string()))];
    if let Some(at) = index {
        entries.push(("index".to_string(), Json::Number(at as f64)));
    }
    Json::Object(entries)
}

/// ↩️ Reads `base` (the CURRENT, pre-mutation package) to build the wire specs that undo `{kind, params}` — none when
/// there is nothing to undo — the same law `DocxMutation::inverse` proves at the Rust-model level, computed here
/// against the reference libraries instead.
fn inverse_spec(base: &WPackage, kind: &str, params: &Json) -> Result<Vec<Json>, String> {
    let spec = |inverse_kind: &str, inverse_params: Json| vec![Json::Object(vec![("kind".to_string(), Json::String(inverse_kind.to_string())), ("params".to_string(), inverse_params)])];
    let obj = |entries: Vec<(&str, Json)>| Json::Object(entries.into_iter().map(|(key, value)| (key.to_string(), value)).collect());
    Ok(match kind {
        "insert-block" => {
            let path_json = params.get("path").cloned().unwrap_or(Json::Null);
            spec("remove-block", obj(vec![("path", path_json)]))
        }
        "remove-block" => {
            let path = json_to_path(&params.get("path").cloned().unwrap_or(Json::Null));
            let removed = resolve_blocks(&base.body, &path.segments).and_then(|list| list.get(path.index)).ok_or("inverse remove-block: original package has no block at this path")?;
            spec("insert-block", obj(vec![("path", path_to_json(&path)), ("block", block_to_json(removed))]))
        }
        "set-block-content" => {
            let path = json_to_path(&params.get("path").cloned().unwrap_or(Json::Null));
            let original = resolve_blocks(&base.body, &path.segments).and_then(|list| list.get(path.index)).ok_or("inverse set-block-content: original package has no block at this path")?;
            spec("set-block-content", obj(vec![("path", path_to_json(&path)), ("block", block_to_json(original))]))
        }
        "set-run-text" => {
            let address = params.get("address").cloned().unwrap_or(Json::Null);
            let (path, run_index) = resolve_run_address(base, &address)?;
            let list = resolve_blocks(&base.body, &path.segments).ok_or("inverse set-run-text: original package has no block list at this path")?;
            let Some(WBlock::Paragraph(paragraph)) = list.get(path.index) else { return Err("inverse set-run-text: addressed block is not a paragraph".to_string()) };
            let run = paragraph.runs.get(run_index).ok_or("inverse set-run-text: run index out of range")?;
            spec("set-run-text", obj(vec![("address", address), ("text", Json::String(run.text.clone()))]))
        }
        "set-run-formatting" => {
            let address = params.get("address").cloned().unwrap_or(Json::Null);
            let (path, run_index) = resolve_run_address(base, &address)?;
            let list = resolve_blocks(&base.body, &path.segments).ok_or("inverse set-run-formatting: original package has no block list at this path")?;
            let Some(WBlock::Paragraph(paragraph)) = list.get(path.index) else { return Err("inverse set-run-formatting: addressed block is not a paragraph".to_string()) };
            let run = paragraph.runs.get(run_index).ok_or("inverse set-run-formatting: run index out of range")?;
            spec("set-run-formatting", obj(vec![("address", address), ("bold", Json::Bool(run.bold)), ("italic", Json::Bool(run.italic)), ("underline", Json::Bool(run.underline))]))
        }
        "insert-style" => {
            let style = params.get("style").cloned().unwrap_or(Json::Null);
            spec("remove-style", obj(vec![("id", Json::String(style.str("id")))]))
        }
        // ↩️ `remove-style` is invertible only for the LAST style, and that is a property of the
        // vocabulary rather than of this oracle: `DocxMutation::InsertStyle` carries only a
        // `style` and appends (`../🧬️schema/🧬️mutations/🦀️.rs:181` → `diff_insert_style`),
        // so no declared kind can put a style back at an INTERIOR position. Removing `Title`
        // from this fixture's `[Normal, Title, Heading1, Heading2, Heading3, Code, TableCell]`
        // and appending it again leaves `Heading1` where `Title` was — which is exactly what the
        // case's inverse law caught. Refusing outright is the honest answer; returning an undo
        // that does not undo is not. The production `inverse()` at that file's line 227 has the
        // same gap and answers `InsertStyle` regardless.
        "remove-style" => {
            let id = params.str("id");
            match base.styles.iter().position(|style| style.id == id) {
                Some(index) if index + 1 == base.styles.len() => spec("insert-style", obj(vec![("style", style_to_json(&base.styles[index]))])),
                Some(index) => {
                    return Err(format!(
                        "remove-style at index {index} of {} has no inverse in this vocabulary: insert-style carries only a style and appends, so no declared kind can put {id:?} back where it was",
                        base.styles.len()
                    ))
                }
                None => Vec::new(),
            }
        }
        "set-style-name" => {
            let id = params.str("id");
            let style = base.styles.iter().find(|style| style.id == id).ok_or_else(|| format!("inverse set-style-name: no style {id:?}"))?;
            spec("set-style-name", obj(vec![("id", Json::String(id)), ("name", Json::String(style.name.clone()))]))
        }
        "set-style-based-on" => {
            let id = params.str("id");
            let style = base.styles.iter().find(|style| style.id == id).ok_or_else(|| format!("inverse set-style-based-on: no style {id:?}"))?;
            spec(
                "set-style-based-on",
                obj(vec![
                    ("id", Json::String(id)),
                    (
                        "based_on",
                        match &style.based_on {
                            Some(v) => Json::String(v.clone()),
                            None => Json::Null,
                        },
                    ),
                ]),
            )
        }
        "set-part" => {
            let path = params.str("path");
            match base.opc.part(&path) {
                Some(part) => spec("set-part", obj(vec![("path", Json::String(path)), ("content_type", Json::String(part.content_type.clone())), ("bytes", bytes_to_json(&part.bytes))])),
                None => spec("remove-part", obj(vec![("path", Json::String(path))])),
            }
        }
        "remove-part" => {
            let path = params.str("path");
            match base.opc.part(&path) {
                Some(part) => spec("set-part", obj(vec![("path", Json::String(path)), ("content_type", Json::String(part.content_type.clone())), ("bytes", bytes_to_json(&part.bytes))])),
                None => Vec::new(),
            }
        }
        "set-relationship" => {
            let owner = params.str("owner");
            let id = params.str("id");
            match base.opc.relationships.get(&owner).and_then(|list| list.iter().position(|existing| existing.id == id).map(|at| (at, &list[at]))) {
                Some((at, previous)) => spec("set-relationship", relationship_params(&owner, previous, Some(at))),
                None => spec("remove-relationship", obj(vec![("owner", Json::String(owner)), ("id", Json::String(id))])),
            }
        }
        "remove-relationship" => {
            let owner = params.str("owner");
            let id = params.str("id");
            match base.opc.relationships.get(&owner).and_then(|list| list.iter().position(|existing| existing.id == id).map(|at| (at, &list[at]))) {
                Some((at, previous)) => spec("set-relationship", relationship_params(&owner, previous, Some(at))),
                None => Vec::new(),
            }
        }
        "set-content-type" => {
            let is_override = bool_field(params, "isOverride");
            let name = params.str("name");
            let entries = if is_override { &base.opc.content_types.overrides } else { &base.opc.content_types.defaults };
            match entries.iter().position(|(key, _)| *key == name) {
                Some(at) => spec("set-content-type", content_type_params(is_override, &name, &entries[at].1, Some(at))),
                None => spec("remove-content-type", obj(vec![("isOverride", Json::Bool(is_override)), ("name", Json::String(name))])),
            }
        }
        "remove-content-type" => {
            let is_override = bool_field(params, "isOverride");
            let name = params.str("name");
            let entries = if is_override { &base.opc.content_types.overrides } else { &base.opc.content_types.defaults };
            match entries.iter().position(|(key, _)| *key == name) {
                Some(at) => spec("set-content-type", content_type_params(is_override, &name, &entries[at].1, Some(at))),
                None => Vec::new(),
            }
        }
        other => return Err(format!("mutation kind {other:?} has no oracle inverse implementation")),
    })
}
//#endregion 🔖️Inverse

//#region 🔖️Routing
pub fn apply_mutation(input: &[u8], kind: &str, params: &Json) -> Result<Vec<u8>, String> {
    let mut pkg = read_package(input)?;
    apply_kind(&mut pkg, kind, params)?;
    write_package(&pkg)
}

/// ↩️ Applies `{kind, params}` and then its computed inverse, in sequence, and returns the
/// re-serialized result — the caller compares its projection against the ORIGINAL input's own.
pub fn apply_mutation_inverse(input: &[u8], kind: &str, params: &Json) -> Result<Vec<u8>, String> {
    let base = read_package(input)?;
    let mut current = apply_mutation(input, kind, params)?;
    for undo in inverse_spec(&base, kind, params)? {
        current = apply_mutation(&current, &undo.str("kind"), undo.get("params").unwrap_or(&Json::Null))?;
    }
    Ok(current)
}

/// 🔄️ The package read and re-written through this oracle's own reader and writer alone.
pub fn round_trip(input: &[u8]) -> Result<Vec<u8>, String> {
    write_package(&read_package(input)?)
}

//#region 🔖️Projection
/// 🪢️ The package plumbing in the order the package states it: the `[Content_Types].xml` defaults and overrides as ordered pairs, and every owner's relationships
/// as ordered `{id, type, target, external}` rows (owners sorted by part name). The positions the OPC-layer kinds write are part of what is compared.
fn plumbing_json(opc: &OPackage) -> Json {
    let pairs = |entries: &[(String, String)]| Json::Array(entries.iter().map(|(key, value)| Json::Array(vec![Json::String(key.clone()), Json::String(value.clone())])).collect());
    let mut owners: Vec<&String> = opc.relationships.keys().collect();
    owners.sort();
    let relationships = owners
        .into_iter()
        .map(|owner| {
            let rows = opc.relationships[owner]
                .iter()
                .map(|rel| Json::Object(vec![("id".to_string(), Json::String(rel.id.clone())), ("type".to_string(), Json::String(rel.rel_type.clone())), ("target".to_string(), Json::String(rel.target.clone())), ("external".to_string(), Json::Bool(rel.external))]))
                .collect();
            (owner.clone(), Json::Array(rows))
        })
        .collect();
    Json::Object(vec![("defaults".to_string(), pairs(&opc.content_types.defaults)), ("overrides".to_string(), pairs(&opc.content_types.overrides)), ("relationships".to_string(), Json::Object(relationships))])
}

/// 🌿️ An XML part's logical content — element names, attributes by name, text, in document order — as one
/// canonical string, so a digest over it moves with the content and never with a writer's layout of it (line ends,
/// whitespace inside a start tag, attribute order, quote style).
fn logical_xml(node: &XNode, out: &mut String) {
    match node {
        XNode::Text(text) => out.push_str(&format!("{text:?}")),
        XNode::Element { name, attrs, children } => {
            let mut sorted: Vec<&(String, String)> = attrs.iter().collect();
            sorted.sort();
            out.push('(');
            out.push_str(name);
            for (key, value) in sorted {
                out.push_str(&format!(" {key}={value:?}"));
            }
            for child in children {
                out.push(' ');
                logical_xml(child, out);
            }
            out.push(')');
        }
    }
}

/// 🧾️ A part's content digest: an XML-bearing part (ECMA-376 Part 2 §10.1.2.2 — an `+xml`/`/xml` content type or
/// an `.xml` name) digested over its logical content, whose bytes are writer freedom; any other part over its bytes.
fn part_digest(part: &OPart) -> String {
    let xml = part.content_type.ends_with("+xml") || part.content_type.ends_with("/xml") || part.path.to_ascii_lowercase().ends_with(".xml");
    match xml.then(|| parse_xml(&part.bytes).ok()).flatten() {
        Some(root) => {
            let mut canonical = String::new();
            logical_xml(&root, &mut canonical);
            digest(canonical.as_bytes())
        }
        None => digest(&part.bytes),
    }
}

/// 👁️ This subset's own semantic projection: `body`/`styles` — the ordered, document-order-
/// sensitive WordprocessingML block tree and style list, independently re-derived by re-parsing
/// `word/document.xml`/`word/styles.xml` through `quick-xml` — plus `parts`, every OTHER real OPC
/// part (docProps/*, anything `set-part`/`remove-part` touched) projected as an UNORDERED
/// path-keyed map of `{contentType, digest}` ([`part_digest`]) so writer-freedom part order and XML layout never
/// register as a difference. `word/document.xml`/`word/styles.xml` themselves are excluded from `parts` since
/// two independent writers legitimately differ byte-for-byte on non-semantic form for the exact
/// same document — that's exactly why `body`/`styles` exist as the typed comparison instead.
pub fn project(bytes: &[u8]) -> Result<Json, String> {
    let pkg = read_package(bytes)?;
    let mut part_entries: Vec<(String, Json)> = pkg
        .opc
        .parts
        .iter()
        .filter(|part| part.path != pkg.main_path && Some(&part.path) != pkg.styles_path.as_ref())
        .map(|part| (part.path.clone(), Json::Object(vec![("contentType".to_string(), Json::String(part.content_type.clone())), ("digest".to_string(), Json::String(part_digest(part)))])))
        .collect();
    part_entries.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(Json::Object(vec![
        ("body".to_string(), Json::Array(pkg.body.iter().map(block_to_json).collect())),
        ("styles".to_string(), Json::Array(pkg.styles.iter().map(style_to_json).collect())),
        ("parts".to_string(), Json::Object(part_entries)),
        ("plumbing".to_string(), plumbing_json(&pkg.opc)),
    ]))
}
//#endregion 🔖️Projection
//#endregion 🔖️Routing
