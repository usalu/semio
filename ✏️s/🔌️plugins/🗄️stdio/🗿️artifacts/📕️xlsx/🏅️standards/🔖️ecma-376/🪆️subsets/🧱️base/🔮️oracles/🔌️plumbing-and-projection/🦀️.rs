//! 🔌️ Plumbing and projection of the XLSX oracle: the package plumbing and the workbook and plumbing projections. See [`super`].

use super::*;

//#region 🔖️Plumbing
/// 🪢️ The four OPC-plumbing kinds (`set-relationship`, `remove-relationship`, `set-content-type`, `remove-content-type`), performed on `[Content_Types].xml` and the `*.rels`
/// parts through the `zip` + `quick-xml` pairing the six OOXML conformance subsets already run on -- the rows of these parts are flat, so each is read into its
/// ordered rows, edited, and written back; every other part is carried verbatim. `calamine`/`rust_xlsxwriter` never see the plumbing.
pub(super) mod plumbing {
    use quick_xml::events::{BytesStart, Event};
    use quick_xml::reader::Reader;
    use quick_xml::XmlVersion;
    use semio_repo_test_host::Json;
    use semio_s_plugin_stdio_document_test_oracle::ooxml::{read_parts, relationships_part_path, write_parts};

    const CONTENT_TYPES_PART: &str = "[Content_Types].xml";
    const RELATIONSHIPS_NS: &str = "http://schemas.openxmlformats.org/package/2006/relationships";

    type Attrs = Vec<(String, String)>;

    /// 🧱️ A plumbing part: its root element and the ordered flat rows below it.
    struct Flat {
        root: String,
        root_attrs: Attrs,
        rows: Vec<(String, Attrs)>,
    }

    impl Flat {
        fn attr<'a>(row: &'a (String, Attrs), key: &str) -> Option<&'a str> {
            row.1.iter().find(|(name, _)| name == key).map(|(_, value)| value.as_str())
        }

        /// 📍️ The row index of the `tag` row whose `key` attribute is `value`.
        fn position(&self, tag: &str, key: &str, value: &str) -> Option<usize> {
            self.rows.iter().position(|row| row.0 == tag && Self::attr(row, key) == Some(value))
        }

        /// 📍️ The physical position a new `tag` row takes: before the `index`-th existing row of its tag, else after the last of its tag -- a content-types default
        /// after the last default, an override and a relationship at the end.
        fn slot(&self, tag: &str, index: Option<usize>) -> usize {
            let slots: Vec<usize> = self.rows.iter().enumerate().filter(|(_, row)| row.0 == tag).map(|(at, _)| at).collect();
            match index.and_then(|at| slots.get(at).copied()) {
                Some(physical) => physical,
                None => match slots.last() {
                    Some(last) => last + 1,
                    None if tag == "Default" => 0,
                    None => self.rows.len(),
                },
            }
        }
    }

    fn attrs_of(start: &BytesStart) -> Result<Attrs, String> {
        start
            .attributes()
            .map(|attribute| {
                let attribute = attribute.map_err(|error| error.to_string())?;
                let value = attribute.normalized_value(XmlVersion::Explicit1_0).map_err(|error| error.to_string())?;
                Ok((attribute.key.as_ref().to_string(), value.to_string()))
            })
            .collect()
    }

    fn parse(bytes: &[u8]) -> Result<Flat, String> {
        let text = std::str::from_utf8(bytes).map_err(|error| error.to_string())?;
        let mut reader = Reader::from_str(text);
        let mut flat: Option<Flat> = None;
        let mut depth = 0usize;
        loop {
            match reader.read_event().map_err(|error| format!("quick-xml parse error at byte {}: {error}", reader.error_position()))? {
                Event::Start(start) => {
                    match flat.as_mut() {
                        None => flat = Some(Flat { root: start.name().as_ref().to_string(), root_attrs: attrs_of(&start)?, rows: Vec::new() }),
                        Some(flat) if depth == 1 => flat.rows.push((start.name().as_ref().to_string(), attrs_of(&start)?)),
                        Some(_) => {}
                    }
                    depth += 1;
                }
                Event::Empty(start) => match flat.as_mut() {
                    None => flat = Some(Flat { root: start.name().as_ref().to_string(), root_attrs: attrs_of(&start)?, rows: Vec::new() }),
                    Some(flat) if depth == 1 => flat.rows.push((start.name().as_ref().to_string(), attrs_of(&start)?)),
                    Some(_) => {}
                },
                Event::End(_) => depth = depth.saturating_sub(1),
                Event::Eof => break,
                _ => {}
            }
        }
        flat.ok_or_else(|| "part has no root element".to_string())
    }

    fn escape(text: &str) -> String {
        text.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
    }

    fn render(flat: &Flat) -> Vec<u8> {
        let attrs = |attrs: &Attrs| attrs.iter().map(|(key, value)| format!(" {key}=\"{}\"", escape(value))).collect::<String>();
        let mut out = format!("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?><{}{}>", flat.root, attrs(&flat.root_attrs));
        for row in &flat.rows {
            out.push_str(&format!("<{}{}/>", row.0, attrs(&row.1)));
        }
        out.push_str(&format!("</{}>", flat.root));
        out.into_bytes()
    }

    fn optional_index(params: &Json) -> Option<usize> {
        match params.get("index") {
            Some(Json::Number(number)) if *number >= 0.0 => Some(*number as usize),
            _ => None,
        }
    }

    fn flag(params: &Json, key: &str) -> bool {
        matches!(params.get(key), Some(Json::Bool(true)))
    }

    /// 🏷️ The row tag, its key attribute and the normalized key of a content-types entry.
    fn entry(params: &Json) -> (bool, &'static str, &'static str, String) {
        let is_override = flag(params, "isOverride");
        let (tag, key) = if is_override { ("Override", "PartName") } else { ("Default", "Extension") };
        let name = params.str("name");
        (is_override, tag, key, name)
    }

    fn part_of(parts: &[(String, Vec<u8>)], path: &str) -> Option<Result<Flat, String>> {
        parts.iter().find(|(name, _)| name == path).map(|(_, bytes)| parse(bytes))
    }

    fn put(parts: &mut Vec<(String, Vec<u8>)>, path: &str, flat: &Flat) {
        let bytes = render(flat);
        match parts.iter_mut().find(|(name, _)| name == path) {
            Some(existing) => existing.1 = bytes,
            None => parts.push((path.to_string(), bytes)),
        }
    }

    pub fn apply(input: &[u8], kind: &str, params: &Json) -> Result<Vec<u8>, String> {
        let mut parts = read_parts(input)?;
        match kind {
            "set-relationship" => {
                let path = relationships_part_path(&params.str("owner"));
                let id = params.str("id");
                let mut attrs = vec![("Id".to_string(), id.clone()), ("Type".to_string(), params.str("relType")), ("Target".to_string(), params.str("target"))];
                if flag(params, "external") {
                    attrs.push(("TargetMode".to_string(), "External".to_string()));
                }
                let row = ("Relationship".to_string(), attrs);
                let mut flat = match part_of(&parts, &path) {
                    Some(flat) => flat?,
                    None => Flat { root: "Relationships".to_string(), root_attrs: vec![("xmlns".to_string(), RELATIONSHIPS_NS.to_string())], rows: Vec::new() },
                };
                match flat.position("Relationship", "Id", &id) {
                    Some(at) => flat.rows[at] = row,
                    None => {
                        let physical = flat.slot("Relationship", optional_index(params));
                        flat.rows.insert(physical, row);
                    }
                }
                put(&mut parts, &path, &flat);
            }
            "remove-relationship" => {
                let owner = params.str("owner");
                let path = relationships_part_path(&owner);
                let id = params.str("id");
                let mut flat = part_of(&parts, &path).ok_or_else(|| format!("remove-relationship: no relationships are owned by {owner:?}"))??;
                let at = flat.position("Relationship", "Id", &id).ok_or_else(|| format!("remove-relationship: {owner:?} owns no relationship {id:?}"))?;
                flat.rows.remove(at);
                if flat.rows.iter().any(|row| row.0 == "Relationship") {
                    put(&mut parts, &path, &flat);
                } else {
                    parts.retain(|(name, _)| *name != path);
                }
            }
            "set-content-type" => {
                let (is_override, tag, key, name) = entry(params);
                let mut flat = part_of(&parts, CONTENT_TYPES_PART).ok_or("the package has no [Content_Types].xml")??;
                let content_type = params.str("contentType");
                if !is_override && flat.rows.iter().any(|row| row.0 == "Default" && Flat::attr(row, key).is_some_and(|existing| existing != name && existing.eq_ignore_ascii_case(&name))) {
                    return Err(format!("set-content-type: default extension {name:?} already exists in another letter case"));
                }
                match flat.position(tag, key, &name) {
                    Some(at) => {
                        if let Some(existing) = flat.rows[at].1.iter_mut().find(|(attr, _)| attr == "ContentType") {
                            existing.1 = content_type;
                        }
                    }
                    None => {
                        let physical = flat.slot(tag, optional_index(params));
                        flat.rows.insert(physical, (tag.to_string(), vec![(key.to_string(), name), ("ContentType".to_string(), content_type)]));
                    }
                }
                put(&mut parts, CONTENT_TYPES_PART, &flat);
            }
            "remove-content-type" => {
                let (_, tag, key, name) = entry(params);
                let mut flat = part_of(&parts, CONTENT_TYPES_PART).ok_or("the package has no [Content_Types].xml")??;
                let at = flat.position(tag, key, &name).ok_or_else(|| format!("remove-content-type: the content types hold no {tag} {name:?}"))?;
                flat.rows.remove(at);
                put(&mut parts, CONTENT_TYPES_PART, &flat);
            }
            other => return Err(format!("{other:?} is no plumbing kind")),
        }
        write_parts(&parts)
    }

    /// ↩️ Undoes a plumbing `kind` on `mutated`, sourcing the previous row and its position from `original`: the previous row restored at its position, or the row the forward
    /// wrote removed.
    pub fn undo(original: &[u8], mutated: &[u8], kind: &str, params: &Json) -> Result<Vec<u8>, String> {
        let parts = read_parts(original)?;
        let object = |entries: Vec<(&str, Json)>| Json::Object(entries.into_iter().map(|(key, value)| (key.to_string(), value)).collect());
        let (undo_kind, undo_params) = match kind {
            "set-relationship" | "remove-relationship" => {
                let owner = params.str("owner");
                let id = params.str("id");
                let previous = match part_of(&parts, &relationships_part_path(&owner)) {
                    Some(flat) => {
                        let flat = flat?;
                        flat.position("Relationship", "Id", &id).map(|at| (flat.rows.iter().take(at).filter(|row| row.0 == "Relationship").count(), flat.rows[at].clone()))
                    }
                    None => None,
                };
                match previous {
                    Some((at, row)) => (
                        "set-relationship",
                        object(vec![
                            ("owner", Json::String(owner)),
                            ("id", Json::String(id)),
                            ("relType", Json::String(Flat::attr(&row, "Type").unwrap_or_default().to_string())),
                            ("target", Json::String(Flat::attr(&row, "Target").unwrap_or_default().to_string())),
                            ("external", Json::Bool(Flat::attr(&row, "TargetMode") == Some("External"))),
                            ("index", Json::Number(at as f64)),
                        ]),
                    ),
                    None if kind == "set-relationship" => ("remove-relationship", object(vec![("owner", Json::String(owner)), ("id", Json::String(id))])),
                    None => return Ok(mutated.to_vec()),
                }
            }
            _ => {
                let (is_override, tag, key, name) = entry(params);
                let flat = part_of(&parts, CONTENT_TYPES_PART).ok_or("the package has no [Content_Types].xml")??;
                match flat.position(tag, key, &name) {
                    Some(at) => (
                        "set-content-type",
                        object(vec![
                            ("isOverride", Json::Bool(is_override)),
                            ("name", Json::String(name)),
                            ("contentType", Json::String(Flat::attr(&flat.rows[at], "ContentType").unwrap_or_default().to_string())),
                            ("index", Json::Number(flat.rows.iter().take(at).filter(|row| row.0 == tag).count() as f64)),
                        ]),
                    ),
                    None if kind == "set-content-type" => ("remove-content-type", object(vec![("isOverride", Json::Bool(is_override)), ("name", Json::String(name))])),
                    None => return Ok(mutated.to_vec()),
                }
            }
        };
        apply(mutated, undo_kind, &undo_params)
    }

    /// 🪢️ The owner part a `*.rels` part belongs to (`""` for the package root).
    fn owner_of(path: &str) -> Option<String> {
        let file = path.rsplit('/').next()?;
        let stem = file.strip_suffix(".rels")?;
        let directory = path[..path.len() - file.len()].strip_suffix("_rels/")?;
        Some(format!("{directory}{stem}"))
    }

    /// 🪢️ The package plumbing in the order the package states it: the `[Content_Types].xml` defaults and overrides as ordered pairs, and every owner's relationships as ordered
    /// `{id, type, target, external}` rows (owners sorted by part name).
    pub fn project(bytes: &[u8]) -> Result<Json, String> {
        let parts = read_parts(bytes)?;
        let types = part_of(&parts, CONTENT_TYPES_PART).ok_or("the package has no [Content_Types].xml")??;
        let pairs = |tag: &str, key: &str| Json::Array(types.rows.iter().filter(|row| row.0 == tag).map(|row| Json::Array(vec![Json::String(Flat::attr(row, key).unwrap_or_default().to_string()), Json::String(Flat::attr(row, "ContentType").unwrap_or_default().to_string())])).collect());
        let mut owners: Vec<(String, String)> = parts.iter().filter_map(|(path, _)| owner_of(path).map(|owner| (owner, path.clone()))).collect();
        owners.sort();
        let mut relationships = Vec::with_capacity(owners.len());
        for (owner, path) in owners {
            let flat = part_of(&parts, &path).ok_or("a listed relationships part vanished")??;
            let rows = flat
                .rows
                .iter()
                .filter(|row| row.0 == "Relationship")
                .map(|row| {
                    Json::Object(vec![
                        ("id".to_string(), Json::String(Flat::attr(row, "Id").unwrap_or_default().to_string())),
                        ("type".to_string(), Json::String(Flat::attr(row, "Type").unwrap_or_default().to_string())),
                        ("target".to_string(), Json::String(Flat::attr(row, "Target").unwrap_or_default().to_string())),
                        ("external".to_string(), Json::Bool(Flat::attr(row, "TargetMode") == Some("External"))),
                    ])
                })
                .collect();
            relationships.push((owner, Json::Array(rows)));
        }
        Ok(Json::Object(vec![("defaults".to_string(), pairs("Default", "Extension")), ("overrides".to_string(), pairs("Override", "PartName")), ("relationships".to_string(), Json::Object(relationships))]))
    }
}

/// 🪢️ The package plumbing of `bytes`, in the order the package states it. @see [`plumbing::project`].
pub fn project_xlsx_plumbing(bytes: &[u8]) -> Result<Json, String> {
    plumbing::project(bytes)
}
//#endregion 🔖️Plumbing

//#region 🔖️Projection
/// 👁️ Projects XLSX bytes with the INDEPENDENT `calamine` reader onto the `semantic-spreadsheet-v1`
/// shape this case's oracle and subject are both compared through. `expected_shared_string_count` is
/// caller-tracked metadata (like `csv`'s `has_header`, see that subset's own oracle module) rather
/// than read from these bytes — `calamine` cannot observe the raw pool size either (module doc
/// comment); the caller computes the oracle side by arithmetic and reads the subject side from its
/// own real `XlsxWorkbook::shared_strings.len()`, so the two are still genuinely compared, just not
/// both independently derived from bytes.
pub fn project_xlsx_workbook(bytes: &[u8], expected_shared_string_count: usize) -> Result<Json, String> {
    let sheets = read_workbook_grid(bytes)?;
    Ok(Json::Object(vec![
        ("format".to_string(), Json::String("xlsx".to_string())),
        ("sharedStringCount".to_string(), Json::Number(expected_shared_string_count as f64)),
        (
            "sheets".to_string(),
            Json::Array(
                sheets
                    .into_iter()
                    .map(|(name, cells)| {
                        Json::Object(vec![
                            ("name".to_string(), Json::String(name)),
                            (
                                "cells".to_string(),
                                Json::Array(
                                    cells
                                        .into_iter()
                                        .map(|(row, col, value)| {
                                            Json::Object(vec![
                                                ("row".to_string(), Json::Number((row + 1) as f64)),
                                                ("col".to_string(), Json::Number(col as f64)),
                                                (
                                                    "value".to_string(),
                                                    match value {
                                                        GridValue::Number(n) => Json::Number(n),
                                                        GridValue::Bool(b) => Json::Bool(b),
                                                        GridValue::Text(t) => Json::String(t),
                                                    },
                                                ),
                                            ])
                                        })
                                        .collect(),
                                ),
                            ),
                        ])
                    })
                    .collect(),
            ),
        ),
    ]))
}
//#endregion 🔖️Projection
