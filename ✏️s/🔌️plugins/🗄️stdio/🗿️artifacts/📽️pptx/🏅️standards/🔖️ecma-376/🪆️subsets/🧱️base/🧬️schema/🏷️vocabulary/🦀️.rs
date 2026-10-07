//! 🏷️ Pptx namespaces, relationships and logical XML names.
//#region 🔖️Constants
pub const A_NS: &str = "http://schemas.openxmlformats.org/drawingml/2006/main";
pub const P_NS: &str = "http://schemas.openxmlformats.org/presentationml/2006/main";
pub const R_NS: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";
pub const A_NS_STRICT: &str = "http://purl.oclc.org/ooxml/drawingml/main";
pub const P_NS_STRICT: &str = "http://purl.oclc.org/ooxml/presentationml/main";
pub const R_NS_STRICT: &str = "http://purl.oclc.org/ooxml/officeDocument/relationships";
pub const DRAWINGML_NAMESPACES: &[&str] = &[A_NS, A_NS_STRICT];
pub const PRESENTATIONML_NAMESPACES: &[&str] = &[P_NS, P_NS_STRICT];
pub const OFFICE_RELATIONSHIP_NAMESPACES: &[&str] = &[R_NS, R_NS_STRICT];

pub const PRESENTATION_PART: &str = "ppt/presentation.xml";
pub const SLIDE_MASTER_PART: &str = "ppt/slideMasters/slideMaster1.xml";
pub const SLIDE_LAYOUT_PART: &str = "ppt/slideLayouts/slideLayout1.xml";
pub const THEME_PART: &str = "ppt/theme/theme1.xml";

pub const PRESENTATION_CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml";
pub const SLIDE_CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.presentationml.slide+xml";
pub const SLIDE_LAYOUT_CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.presentationml.slideLayout+xml";
pub const SLIDE_MASTER_CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.presentationml.slideMaster+xml";
pub const THEME_CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.theme+xml";

pub const REL_TYPE_SLIDE: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide";
pub const REL_TYPE_SLIDE_LAYOUT: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideLayout";
pub const REL_TYPE_SLIDE_MASTER: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideMaster";
pub const REL_TYPE_THEME: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme";

/// 🏅️ ISO/IEC 29500-1:2016 Strict's officeDocument relationship type -- Strict packages carry
/// this instead of `REL_TYPE_OFFICE_DOCUMENT` (see `🪆️subsets/🔣️.json`'s "strictRelBase"
/// citation, ticket 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES). `regenerate_presentation_parts`
/// never writes this -- this engine's own writer only ever emits Transitional -- but `decode_pptx`
/// and `sniff_pptx_bytes` must still recognize a genuine Strict-relationship-typed input package,
/// or the `🔒️strict` subset's analyzer could never see real Strict bytes at all.
pub const REL_TYPE_OFFICE_DOCUMENT_STRICT: &str = "http://purl.oclc.org/ooxml/officeDocument/relationships/officeDocument";

/// 🧭️ Resolves the package root's officeDocument relationship regardless of whether it was
/// authored under the Transitional or the Strict relationship-type namespace -- see
/// `REL_TYPE_OFFICE_DOCUMENT_STRICT`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn resolve_office_document_relationship(opc: &semio_s_artifact_stdio_zip::opc::OpcPackage) -> Option<String> {
    opc.resolve_relationship("", semio_s_artifact_stdio_zip::opc::REL_TYPE_OFFICE_DOCUMENT).or_else(|| opc.resolve_relationship("", REL_TYPE_OFFICE_DOCUMENT_STRICT))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn attr(name: &str, value: &str) -> semio_s_artifact_stdio_xml::schema::snapshot::XmlAttr {
    semio_s_artifact_stdio_xml::schema::snapshot::XmlAttr { name: name.into(), value: value.into() }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn attr_val<'a>(attrs: &'a [semio_s_artifact_stdio_xml::schema::snapshot::XmlAttr], name: &str) -> Option<&'a str> {
    attrs.iter().find(|a| a.name == name).map(|a| a.value.as_str())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn find_child<'a>(children: &'a [semio_s_artifact_stdio_xml::schema::snapshot::XmlNode], name: &str) -> Option<&'a semio_s_artifact_stdio_xml::schema::snapshot::XmlNode> {
    children.iter().find(|c| matches!(c, semio_s_artifact_stdio_xml::schema::snapshot::XmlNode::Element { name: n, .. } if n == name))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn element_children(node: &semio_s_artifact_stdio_xml::schema::snapshot::XmlNode) -> &[semio_s_artifact_stdio_xml::schema::snapshot::XmlNode] {
    match node {
        semio_s_artifact_stdio_xml::schema::snapshot::XmlNode::Element { children, .. } => children,
        _ => &[],
    }
}

/// 🧭️ Computes the namespace bindings in scope at one retained XML node.
pub fn namespace_scope(parent: &[(String, String)], node: &semio_s_artifact_stdio_xml::schema::snapshot::XmlNode) -> Vec<(String, String)> {
    let mut scope = parent.to_vec();
    let semio_s_artifact_stdio_xml::schema::snapshot::XmlNode::Element { attrs, .. } = node else { return scope };
    for attr in attrs {
        let prefix = if attr.name == "xmlns" { Some("") } else { attr.name.strip_prefix("xmlns:") };
        let Some(prefix) = prefix else { continue };
        if let Some(existing) = scope.iter_mut().find(|(bound, _)| bound == prefix) {
            existing.1 = attr.value.clone();
        } else {
            scope.push((prefix.into(), attr.value.clone()));
        }
    }
    scope
}

/// 🏷️ Resolves a qualified element name through its lexical namespace scope.
pub fn expanded_element_name(name: &str, scope: &[(String, String)]) -> Result<(String, String), String> {
    let (prefix, local) = name.split_once(':').unwrap_or(("", name));
    let namespace = scope.iter().rev().find(|(bound, _)| bound == prefix).map(|(_, uri)| uri.clone()).ok_or_else(|| format!("unbound XML prefix {prefix:?} on {name}"))?;
    Ok((namespace, local.into()))
}

/// 🔎️ Tests one retained element by namespace URI and local name, independent of its prefix.
pub fn element_matches(node: &semio_s_artifact_stdio_xml::schema::snapshot::XmlNode, scope: &[(String, String)], namespaces: &[&str], local: &str) -> Result<bool, String> {
    let semio_s_artifact_stdio_xml::schema::snapshot::XmlNode::Element { name, .. } = node else { return Ok(false) };
    let (namespace, actual_local) = expanded_element_name(name, scope)?;
    Ok(actual_local == local && namespaces.contains(&namespace.as_str()))
}

/// 🏷️ Reads an attribute by expanded name. An empty namespace matches unqualified attributes.
pub fn attribute_value<'a>(node: &'a semio_s_artifact_stdio_xml::schema::snapshot::XmlNode, scope: &[(String, String)], namespaces: &[&str], local: &str) -> Result<Option<&'a str>, String> {
    let semio_s_artifact_stdio_xml::schema::snapshot::XmlNode::Element { attrs, .. } = node else { return Ok(None) };
    for attr in attrs {
        if attr.name == "xmlns" || attr.name.starts_with("xmlns:") {
            continue;
        }
        let (prefix, actual_local) = attr.name.split_once(':').unwrap_or(("", attr.name.as_str()));
        if actual_local != local {
            continue;
        }
        let namespace = if prefix.is_empty() { "" } else { scope.iter().rev().find(|(bound, _)| bound == prefix).map(|(_, uri)| uri.as_str()).ok_or_else(|| format!("unbound XML attribute prefix {prefix:?}"))? };
        if namespaces.contains(&namespace) {
            return Ok(Some(attr.value.as_str()));
        }
    }
    Ok(None)
}

