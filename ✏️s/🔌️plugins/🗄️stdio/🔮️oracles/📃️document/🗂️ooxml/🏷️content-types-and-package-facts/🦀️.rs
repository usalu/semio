//! 🏷️ Content types and package facts of the OOXML engine: the content-type map, the main part, the relationship types and the declared namespaces. See [`super`].

use super::*;

//#region 🔖️ContentTypes
pub const CONTENT_TYPES_PART: &str = "[Content_Types].xml";

/// 🏷️ `([(extension, content type)], [(part name, content type)])` read out of
/// `[Content_Types].xml`.
pub fn content_types(parts: &[(String, Vec<u8>)]) -> Result<(Vec<(String, String)>, Vec<(String, String)>), String> {
    let bytes = part_bytes(parts, CONTENT_TYPES_PART).ok_or("package has no [Content_Types].xml")?;
    let text = std::str::from_utf8(bytes).map_err(|error| format!("[Content_Types].xml is not valid utf-8: {error}"))?;
    let mut reader = Reader::from_str(text);
    let (mut defaults, mut overrides) = (Vec::new(), Vec::new());
    loop {
        match reader.read_event().map_err(|error| format!("quick-xml parse error at byte {}: {error}", reader.error_position()))? {
            Event::Eof => break,
            Event::Start(start) | Event::Empty(start) => {
                let name = element_name(&start)?;
                let attrs = attributes_of(&start)?;
                let attribute = |key: &str| attrs.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone()).unwrap_or_default();
                match name.as_str() {
                    "Default" => defaults.push((attribute("Extension").to_ascii_lowercase(), attribute("ContentType"))),
                    "Override" => overrides.push((attribute("PartName"), attribute("ContentType"))),
                    _ => {}
                }
            }
            _ => {}
        }
    }
    Ok((defaults, overrides))
}

/// ✍️ Regenerates `[Content_Types].xml` from a typed table with `quick-xml`'s writer — the
/// engine rebuilds the part rather than splicing it, for the same reason `write_parts` rebuilds
/// the container.
pub fn write_content_types(parts: &mut Vec<(String, Vec<u8>)>, defaults: &[(String, String)], overrides: &[(String, String)]) -> Result<(), String> {
    let mut writer = Writer::new(Cursor::new(Vec::new()));
    writer.write_event(Event::Decl(quick_xml::events::BytesDecl::new("1.0", Some("UTF-8"), Some("yes")))).map_err(|error| error.to_string())?;
    let mut root = BytesStart::new("Types");
    root.push_attribute(("xmlns", "http://schemas.openxmlformats.org/package/2006/content-types"));
    writer.write_event(Event::Start(root)).map_err(|error| error.to_string())?;
    for (extension, content_type) in defaults {
        let mut entry = BytesStart::new("Default");
        entry.push_attribute(("Extension", extension.as_str()));
        entry.push_attribute(("ContentType", content_type.as_str()));
        writer.write_event(Event::Empty(entry)).map_err(|error| error.to_string())?;
    }
    for (part_name, content_type) in overrides {
        let mut entry = BytesStart::new("Override");
        entry.push_attribute(("PartName", part_name.as_str()));
        entry.push_attribute(("ContentType", content_type.as_str()));
        writer.write_event(Event::Empty(entry)).map_err(|error| error.to_string())?;
    }
    writer.write_event(Event::End(quick_xml::events::BytesEnd::new("Types"))).map_err(|error| error.to_string())?;
    set_part(parts, CONTENT_TYPES_PART, writer.into_inner().into_inner());
    Ok(())
}

/// ✍️ Sets — or, with `None`, removes — the `Override` entry for `part_path`.
pub fn set_content_type_override(parts: &mut Vec<(String, Vec<u8>)>, part_path: &str, content_type: Option<&str>) -> Result<(), String> {
    let part_name = format!("/{}", part_path.trim_start_matches('/'));
    let (defaults, mut overrides) = content_types(parts)?;
    overrides.retain(|(name, _)| *name != part_name);
    if let Some(content_type) = content_type {
        overrides.push((part_name, content_type.to_string()));
    }
    write_content_types(parts, &defaults, &overrides)
}

/// 🔎️ The effective content type of `part_path`: its `Override` first, else the `Default` for
/// its extension.
pub fn resolve_content_type(defaults: &[(String, String)], overrides: &[(String, String)], part_path: &str) -> String {
    let part_name = format!("/{}", part_path.trim_start_matches('/'));
    if let Some((_, content_type)) = overrides.iter().find(|(name, _)| *name == part_name) {
        return content_type.clone();
    }
    let extension = part_path.rsplit('.').next().unwrap_or_default().to_ascii_lowercase();
    defaults.iter().find(|(candidate, _)| *candidate == extension).map(|(_, content_type)| content_type.clone()).unwrap_or_default()
}
//#endregion 🔖️ContentTypes

//#region 🔖️PackageFacts
/// 🧭️ The root `officeDocument` relationship's target — the main part, matched by relationship
/// type SUFFIX so it resolves under either conformance class.
pub fn main_part(parts: &[(String, Vec<u8>)]) -> Result<String, String> {
    let bytes = part_bytes(parts, "_rels/.rels").ok_or("package has no _rels/.rels")?;
    let text = std::str::from_utf8(bytes).map_err(|error| format!("_rels/.rels is not valid utf-8: {error}"))?;
    let mut reader = Reader::from_str(text);
    loop {
        match reader.read_event().map_err(|error| format!("quick-xml parse error at byte {}: {error}", reader.error_position()))? {
            Event::Eof => return Err("package root relationships declare no officeDocument relationship".to_string()),
            Event::Start(start) | Event::Empty(start) => {
                let attrs = attributes_of(&start)?;
                let attribute = |key: &str| attrs.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone()).unwrap_or_default();
                if attribute("Type").ends_with("/officeDocument") {
                    return Ok(attribute("Target").trim_start_matches('/').to_string());
                }
            }
            _ => {}
        }
    }
}

/// 🔗️ Every distinct `Relationship/@Type` in every `.rels` part of the package, sorted.
pub fn relationship_types(parts: &[(String, Vec<u8>)]) -> Result<Vec<String>, String> {
    let mut types = Vec::new();
    for (path, bytes) in parts.iter().filter(|(path, _)| path.ends_with(".rels")) {
        let text = std::str::from_utf8(bytes).map_err(|error| format!("{path} is not valid utf-8: {error}"))?;
        let mut reader = Reader::from_str(text);
        loop {
            match reader.read_event().map_err(|error| format!("quick-xml parse error in {path} at byte {}: {error}", reader.error_position()))? {
                Event::Eof => break,
                Event::Start(start) | Event::Empty(start) => {
                    for (key, value) in attributes_of(&start)? {
                        if key == "Type" && !types.contains(&value) {
                            types.push(value);
                        }
                    }
                }
                _ => {}
            }
        }
    }
    types.sort();
    Ok(types)
}

/// 🏷️ Every distinct OOXML-family namespace URI declared anywhere in the package — the
/// conformance class made observable. Both ISO/IEC 29500 families and the legacy VML urn are
/// recognised; anything else (Dublin Core, the markup-compatibility namespace, a vendor
/// extension) is not part of the class and is deliberately not projected.
pub fn declared_namespaces(parts: &[(String, Vec<u8>)]) -> Result<Vec<String>, String> {
    const FAMILIES: [&str; 3] = ["http://schemas.openxmlformats.org/", "http://purl.oclc.org/ooxml/", "urn:schemas-microsoft-com:vml"];
    let mut namespaces: Vec<String> = Vec::new();
    for (path, bytes) in parts.iter().filter(|(path, _)| is_xml_part(path)) {
        let Ok(text) = std::str::from_utf8(bytes) else { continue };
        let mut reader = Reader::from_str(text);
        loop {
            match reader.read_event().map_err(|error| format!("quick-xml parse error in {path} at byte {}: {error}", reader.error_position()))? {
                Event::Eof => break,
                Event::Start(start) | Event::Empty(start) => {
                    for (key, value) in attributes_of(&start)? {
                        if (key == "xmlns" || key.starts_with("xmlns:")) && FAMILIES.iter().any(|family| value.starts_with(family)) && !namespaces.contains(&value) {
                            namespaces.push(value);
                        }
                    }
                }
                _ => {}
            }
        }
    }
    namespaces.sort();
    Ok(namespaces)
}

/// 🧩️ Parts carrying `mc:AlternateContent`, the markup-compatibility escape hatch ISO/IEC
/// 29500-1 Strict warns on.
pub fn alternate_content_parts(parts: &[(String, Vec<u8>)]) -> Vec<String> {
    let mut found: Vec<String> = parts.iter().filter(|(path, bytes)| is_xml_part(path) && contains(bytes, "mc:AlternateContent")).map(|(path, _)| path.clone()).collect();
    found.sort();
    found
}
//#endregion 🔖️PackageFacts

//#region 🔖️PackageWire
/// ✂️ XML character escaping for text content and, with `attribute`, for a double-quoted attribute value — whitespace
/// an attribute carries is written as character references so a reader's attribute normalization keeps it.
pub(super) fn escape_xml(text: &str, attribute: bool) -> String {
    let mut out = String::with_capacity(text.len());
    for character in text.chars() {
        match character {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' if attribute => out.push_str("&quot;"),
            '\t' if attribute => out.push_str("&#9;"),
            '\n' if attribute => out.push_str("&#10;"),
            '\r' if attribute => out.push_str("&#13;"),
            other => out.push(other),
        }
    }
    out
}

/// 🧵️ One wire `XmlNode` (`{kind: element|text|cData|comment|processingInstruction, …}`) as XML text.
pub(super) fn xml_node_text(node: &Json, out: &mut String) -> Result<(), String> {
    match node.str("kind").as_str() {
        "element" => {
            let name = node.str("name");
            out.push('<');
            out.push_str(&name);
            for attribute in node.array("attrs") {
                out.push_str(&format!(" {}=\"{}\"", attribute.str("name"), escape_xml(&attribute.str("value"), true)));
            }
            let children = node.array("children");
            if children.is_empty() {
                out.push_str("/>");
                return Ok(());
            }
            out.push('>');
            for child in &children {
                xml_node_text(child, out)?;
            }
            out.push_str(&format!("</{name}>"));
        }
        "text" => out.push_str(&escape_xml(&node.str("text"), false)),
        "cData" => out.push_str(&format!("<![CDATA[{}]]>", node.str("text"))),
        "comment" => out.push_str(&format!("<!--{}-->", node.str("text"))),
        "processingInstruction" => out.push_str(&format!("<?{} {}?>", node.str("target"), node.str("data"))),
        other => return Err(format!("unknown XML node kind {other:?}")),
    }
    Ok(())
}

/// 🧵️ One wire `XmlDocument` (`{declaration?, prolog?, root?, epilog?}`) as the XML text of an OPC part. A document type
/// declaration has no place in an OPC part, so a document carrying one is refused rather than silently dropped.
pub(super) fn xml_document_text(document: &Json) -> Result<String, String> {
    if document.get("doctype").is_some() {
        return Err("an OPC XML part carries no document type declaration".to_string());
    }
    let mut out = String::new();
    if let Some(declaration) = document.get("declaration") {
        let quote = if declaration.str("quote") == "single" { '\'' } else { '"' };
        out.push_str(&format!("<?xml version={quote}{}{quote}", declaration.str("version")));
        if let Some(Json::String(encoding)) = declaration.get("encoding") {
            out.push_str(&format!(" encoding={quote}{encoding}{quote}"));
        }
        if let Some(Json::Bool(standalone)) = declaration.get("standalone") {
            out.push_str(&format!(" standalone={quote}{}{quote}", if *standalone { "yes" } else { "no" }));
        }
        out.push_str("?>");
    }
    for node in document.array("prolog") {
        xml_node_text(&node, &mut out)?;
    }
    if let Some(root) = document.get("root") {
        xml_node_text(root, &mut out)?;
    }
    for node in document.array("epilog") {
        xml_node_text(&node, &mut out)?;
    }
    Ok(out)
}
//#endregion 🔖️PackageWire
