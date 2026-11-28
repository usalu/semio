//! 🗣️ SpreadsheetML namespaces, relationships and logical XML traversal.

//#region 🔖️Constants
pub const SML_NS: &str = "http://schemas.openxmlformats.org/spreadsheetml/2006/main";
pub const SML_NS_STRICT: &str = "http://purl.oclc.org/ooxml/spreadsheetml/main";
pub const R_NS: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";
pub const R_NS_STRICT: &str = "http://purl.oclc.org/ooxml/officeDocument/relationships";
pub const WORKBOOK_PART: &str = "xl/workbook.xml";
pub const SHARED_STRINGS_PART: &str = "xl/sharedStrings.xml";
pub const WORKBOOK_CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml";

/// 🏷️ Matches the registered Workbook part media type by its case-insensitive name.
pub fn workbook_media_type_matches(content_type: &str) -> bool {
    content_type.split(';').next().is_some_and(|name| name.eq_ignore_ascii_case(WORKBOOK_CONTENT_TYPE))
}
pub const WORKSHEET_CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml";
pub const SHARED_STRINGS_CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.spreadsheetml.sharedStrings+xml";
pub const REL_TYPE_WORKSHEET: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet";
pub const REL_TYPE_SHARED_STRINGS: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/sharedStrings";
/// 🏅️ ISO/IEC 29500-1 Strict's officeDocument relationship TYPE for the package-root -> workbook
/// pointer (Strict's Annex replaces every `schemas.openxmlformats.org` relationship-type URI with
/// a `purl.oclc.org/ooxml` equivalent, not just the content markup namespaces -- ticket
/// 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES W3's `🔒️strict` subset). Recognized here
/// (decode/sniff, additively, alongside the Transitional URI above) so a genuinely Strict-shaped
/// package can be decoded at all -- without this, `decode_xlsx` would reject every real Strict
/// document with `MissingWorkbookRelationship` before the `🔒️strict` subset analyzer ever ran.
pub const REL_TYPE_OFFICE_DOCUMENT_STRICT: &str = "http://purl.oclc.org/ooxml/officeDocument/relationships/officeDocument";
/// 🏅️ Strict's `sharedStrings` relationship TYPE, same rationale as above -- without recognizing
/// it, any Strict document using shared strings would hard-fail decode with an out-of-range
/// shared-string index (the shared-strings part would never be found).
pub const REL_TYPE_SHARED_STRINGS_STRICT: &str = "http://purl.oclc.org/ooxml/officeDocument/relationships/sharedStrings";
/// 🏅️ Strict's `worksheet` relationship TYPE — the workbook-owned pointer that gives a part its worksheet role in a Strict
/// package, same rationale as the two Strict relationship types above.
pub const REL_TYPE_WORKSHEET_STRICT: &str = "http://purl.oclc.org/ooxml/officeDocument/relationships/worksheet";

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn attr(name: &str, value: &str) -> semio_s_artifact_stdio_xml::schema::snapshot::XmlAttr {
    semio_s_artifact_stdio_xml::schema::snapshot::XmlAttr { name: name.into(), value: value.into() }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn attr_val<'a>(attrs: &'a [semio_s_artifact_stdio_xml::schema::snapshot::XmlAttr], name: &str) -> Option<&'a str> {
    attrs.iter().find(|a| a.name == name).map(|a| a.value.as_str())
}

/// 🧭️ Extends an inherited namespace scope with declarations authored on one XML element.
pub fn namespace_scope(parent: &[(String, String)], node: &semio_s_artifact_stdio_xml::schema::snapshot::XmlNode) -> Vec<(String, String)> {
    let mut scope = parent.to_vec();
    let semio_s_artifact_stdio_xml::schema::snapshot::XmlNode::Element { attrs, .. } = node else { return scope };
    for attribute in attrs {
        let prefix = if attribute.name == "xmlns" { Some("") } else { attribute.name.strip_prefix("xmlns:") };
        if let Some(prefix) = prefix {
            if let Some(binding) = scope.iter_mut().find(|(bound, _)| bound == prefix) {
                binding.1.clone_from(&attribute.value);
            } else {
                scope.push((prefix.into(), attribute.value.clone()));
            }
        }
    }
    scope
}

/// 🧭️ Resolves an element qualified name; the default namespace applies to unprefixed elements.
pub fn expanded_element_name(name: &str, scope: &[(String, String)]) -> Result<(String, String), String> {
    let (prefix, local) = name.split_once(':').unwrap_or(("", name));
    let namespace = if prefix.is_empty() {
        scope.iter().rev().find(|(bound, _)| bound.is_empty()).map_or("", |(_, value)| value.as_str())
    } else {
        scope.iter().rev().find(|(bound, _)| bound == prefix).map(|(_, value)| value.as_str()).ok_or_else(|| format!("unbound XML namespace prefix in element {name}"))?
    };
    Ok((namespace.into(), local.into()))
}

/// 🧭️ Resolves an attribute qualified name; the default namespace never applies to attributes.
pub fn expanded_attribute_name(name: &str, scope: &[(String, String)]) -> Result<(String, String), String> {
    let Some((prefix, local)) = name.split_once(':') else { return Ok((String::new(), name.into())) };
    let namespace =
        if prefix == "xml" { "http://www.w3.org/XML/1998/namespace" } else if prefix == "xmlns" { "http://www.w3.org/2000/xmlns/" } else { scope.iter().rev().find(|(bound, _)| bound == prefix).map(|(_, value)| value.as_str()).ok_or_else(|| format!("unbound XML namespace prefix in attribute {name}"))? };
    Ok((namespace.into(), local.into()))
}

/// 🧭️ Matches an element by exact namespace URI and local name.
pub fn element_matches(node: &semio_s_artifact_stdio_xml::schema::snapshot::XmlNode, scope: &[(String, String)], namespaces: &[&str], local: &str) -> Result<bool, String> {
    let semio_s_artifact_stdio_xml::schema::snapshot::XmlNode::Element { name, .. } = node else { return Ok(false) };
    let (namespace, name) = expanded_element_name(name, scope)?;
    Ok(name == local && namespaces.contains(&namespace.as_str()))
}

/// 🧭️ Reads an attribute by exact namespace URI and local name.
pub fn attribute_value<'a>(node: &'a semio_s_artifact_stdio_xml::schema::snapshot::XmlNode, scope: &[(String, String)], namespaces: &[&str], local: &str) -> Result<Option<&'a str>, String> {
    let semio_s_artifact_stdio_xml::schema::snapshot::XmlNode::Element { attrs, .. } = node else { return Ok(None) };
    for attribute in attrs {
        let (namespace, name) = expanded_attribute_name(&attribute.name, scope)?;
        if name == local && namespaces.contains(&namespace.as_str()) {
            return Ok(Some(attribute.value.as_str()));
        }
    }
    Ok(None)
}

/// 🔗️ The namespaces a workbook part binds where it uses them: the namespace of its root element, the distinct namespace URIs of every prefixed `id` attribute (the relationship ids such as `<sheet r:id>`), and its unprefixed `conformance` attribute.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct WorkbookNamespaces {
    pub main: String,
    pub relationship_ids: Vec<String>,
    pub conformance: Option<String>,
}

/// 🧭️ Resolves the prefixes of a workbook root through the namespace bindings in scope at each element that uses them, so a prefix declared on a descendant (`<sheet xmlns:r=…>`) binds exactly like one declared on the root; an undeclared prefix is refused.
pub fn workbook_namespaces(root: &semio_s_artifact_stdio_xml::schema::snapshot::XmlNode) -> Result<WorkbookNamespaces, String> {
    use semio_s_artifact_stdio_xml::schema::snapshot::XmlNode;
    let XmlNode::Element { name, attrs, .. } = root else { return Err("workbook root is not an element".into()) };
    let root_scope = namespace_scope(&[], root);
    let (main, _) = expanded_element_name(name, &root_scope)?;
    let mut relationship_ids = Vec::new();
    let mut pending = vec![(root, root_scope)];
    while let Some((node, scope)) = pending.pop() {
        let XmlNode::Element { attrs, children, .. } = node else { continue };
        for attribute in attrs.iter().filter(|attribute| attribute.name.contains(':') && !attribute.name.starts_with("xmlns:")) {
            let (namespace, local) = expanded_attribute_name(&attribute.name, &scope)?;
            if local == "id" && !relationship_ids.contains(&namespace) {
                relationship_ids.push(namespace);
            }
        }
        for child in children.iter().rev() {
            pending.push((child, namespace_scope(&scope, child)));
        }
    }
    let conformance = attrs.iter().find(|attribute| attribute.name == "conformance").map(|attribute| attribute.value.clone());
    Ok(WorkbookNamespaces { main, relationship_ids, conformance })
}
//#endregion 🔖️Constants

//#region 🔖️ColumnLetters
/// 🔤️ 0-indexed column number -> spreadsheet column letters (`0 -> "A"`, `25 -> "Z"`, `26 -> "AA"`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn column_letter(mut index: u32) -> String {
    let mut letters = Vec::new();
    loop {
        letters.push((b'A' + (index % 26) as u8) as char);
        if index < 26 {
            break;
        }
        index = index / 26 - 1;
    }
    letters.iter().rev().collect()
}

/// 🔤️ Inverse of `column_letter`: spreadsheet column letters -> 0-indexed column number
/// (`"A" -> 0`, `"Z" -> 25`, `"AA" -> 26`). `None` on empty or non-alphabetic input.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn column_index(letters: &str) -> Option<u32> {
    if letters.is_empty() || !letters.chars().all(|c| c.is_ascii_alphabetic()) {
        return None;
    }
    let mut idx: u32 = 0;
    for c in letters.chars() {
        idx = idx.checked_mul(26)?.checked_add(c.to_ascii_uppercase() as u32 - 'A' as u32 + 1)?;
    }
    Some(idx - 1)
}

/// 🔤️ Splits an A1-style cell reference (`"B2"`) into its column-letter prefix (`"B"`) — only the
/// column part is needed by the decoder, since row is already known from the enclosing `<row r>`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn column_letters_of(reference: &str) -> &str {
    reference.trim_end_matches(|c: char| c.is_ascii_digit())
}
//#endregion 🔖️ColumnLetters

pub const XLSX_MAX_ROW: u32 = 1_048_576;
pub const XLSX_MAX_COLUMN: u32 = 16_383;

/// 🧭️ Specifies which sheetData coordinate is carried forward between siblings.
#[derive(Clone, Copy)]
pub enum WorksheetAxis {
    Rows,
    Columns(u32),
}

impl WorksheetAxis {
    /// 📍️ Interprets an explicit reference or the next position on this axis.
    pub fn coordinate(self, reference: Option<&str>, previous: Option<u32>) -> Result<u32, String> {
        let first = u32::from(matches!(self, Self::Rows));
        let position = match (self, reference) {
            (Self::Rows, Some(row)) => row.parse::<u32>().map_err(|_| format!("invalid SpreadsheetML row reference {row:?}"))?,
            (Self::Columns(row), Some(cell)) => {
                let letters = column_letters_of(cell);
                let column = column_index(letters).ok_or_else(|| format!("invalid SpreadsheetML cell reference {cell:?}"))?;
                if cell[letters.len()..].parse::<u32>().ok() != Some(row) {
                    return Err(format!("SpreadsheetML cell {cell:?} does not belong to row {row}"));
                }
                column
            }
            (_, None) => match previous {
                Some(position) => position.checked_add(1).ok_or_else(|| "SpreadsheetML coordinate overflow".to_string())?,
                None => first,
            },
        };
        let maximum = match self { Self::Rows => XLSX_MAX_ROW, Self::Columns(_) => XLSX_MAX_COLUMN };
        if !(first..=maximum).contains(&position) {
            return Err(format!("SpreadsheetML coordinate {position} is outside {first}..={maximum}"));
        }
        Ok(position)
    }
}

/// 🧩️ Carries the logical coordinate together with the unchanged XML child index and scope.
pub struct PositionedWorksheetChild<'a> {
    pub index: usize,
    pub node: &'a semio_s_artifact_stdio_xml::schema::snapshot::XmlNode,
    pub scope: Vec<(String, String)>,
    pub position: u32,
    pub explicit: bool,
}

/// 🧮️ Resolves row or cell coordinates in document order. Only matching SpreadsheetML elements advance the axis.
/// See [Microsoft's optional cell reference semantics](https://learn.microsoft.com/en-us/openspecs/office_standards/ms-oi29500/2fd4e47f-0965-4c60-95bd-cff980b6c325).
pub fn positioned_worksheet_children<'a>(
    children: &'a [semio_s_artifact_stdio_xml::schema::snapshot::XmlNode],
    parent_scope: &[(String, String)],
    namespace: &str,
    axis: WorksheetAxis,
) -> impl Iterator<Item = Result<PositionedWorksheetChild<'a>, String>> + 'a {
    let (parent_scope, namespace) = (parent_scope.to_vec(), namespace.to_owned());
    let local = match axis { WorksheetAxis::Rows => "row", WorksheetAxis::Columns(_) => "c" };
    let mut previous = None;
    children.iter().enumerate().filter_map(move |(index, node)| {
        let scope = namespace_scope(&parent_scope, node);
        match element_matches(node, &scope, &[namespace.as_str()], local) {
            Ok(false) => return None,
            Err(error) => return Some(Err(error)),
            Ok(true) => {}
        }
        Some((|| {
            let reference = attribute_value(node, &scope, &[""], "r")?;
            let position = axis.coordinate(reference, previous)?;
            let child = PositionedWorksheetChild { index, node, scope, position, explicit: reference.is_some() };
            previous = Some(position);
            Ok(child)
        })())
    })
}


#[cfg(test)]
mod namespace_declaration_tests {
    use super::expanded_attribute_name;

    /// 🧪️ Every case of the language-agnostic fixture resolves exactly as declared; `xmlns:*` declaration attributes are never refused as unbound prefixes.
    /// @see ./🧪️tests/🧫️fixtures/🔣️.json
    #[test]
    fn attribute_names_resolve_as_the_fixture_declares() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️tests/🧫️fixtures/🔣️.json")).expect("fixture is JSON");
        let cases = fixture["cases"].as_array().expect("cases");
        assert!(!cases.is_empty());
        for case in cases {
            let scope: Vec<(String, String)> = case["scope"].as_array().expect("scope").iter().map(|pair| (pair[0].as_str().expect("prefix").to_string(), pair[1].as_str().expect("uri").to_string())).collect();
            let resolved = expanded_attribute_name(case["attribute"].as_str().expect("attribute"), &scope);
            match case["expected"].as_object() {
                Some(expected) => assert_eq!(resolved, Ok((expected["namespace"].as_str().expect("namespace").to_string(), expected["local"].as_str().expect("local").to_string())), "{}", case["name"]),
                None => assert!(resolved.is_err(), "{}", case["name"]),
            }
        }
    }
    /// 🧪️ Every workbook of the fixture binds its relationship-id prefix wherever it declares it: on the root, on a descendant only, under another prefix, or not at all.
    /// @see ./🧪️tests/🧫️fixtures/🔣️.json
    #[test]
    fn workbook_namespaces_resolve_as_the_fixture_declares() {
        use semio_s_artifact_stdio_xml::standards::v1_0::subsets::base::io::text::snapshot::xml_document_from_text;
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️tests/🧫️fixtures/🔣️.json")).expect("fixture is JSON");
        let cases = fixture["workbooks"].as_array().expect("workbooks");
        assert!(cases.iter().any(|case| case["name"].as_str().is_some_and(|name| name.contains("only on a child"))));
        for case in cases {
            let document = xml_document_from_text(case["xml"].as_str().expect("xml")).expect("fixture xml parses");
            let resolved = super::workbook_namespaces(document.root.as_ref().expect("root"));
            match case["expected"].as_object() {
                Some(expected) => {
                    let resolved = resolved.unwrap_or_else(|error| panic!("{}: {error}", case["name"]));
                    assert_eq!(resolved.main, expected["main"].as_str().expect("main"), "{}", case["name"]);
                    let ids: Vec<&str> = expected["relationshipIds"].as_array().expect("ids").iter().map(|id| id.as_str().expect("uri")).collect();
                    assert_eq!(resolved.relationship_ids, ids, "{}", case["name"]);
                }
                None => assert!(resolved.is_err(), "{}", case["name"]),
            }
        }
    }
}
