//! 🧬️ XmlSnapshot schema — persistent fields + real codecs.

use crate::artifacts::xml::STDIO_XML_DOCUMENT_SCHEMA;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️XmlModel
/// 🏷️ XML attribute pair.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct XmlAttr {
    pub name: String,
    pub value: String,
}

/// 🌳 XML node (element or text).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum XmlNode {
    Element {
        name: String,
        #[serde(default)]
        attrs: Vec<XmlAttr>,
        #[serde(default)]
        children: Vec<XmlNode>,
    },
    Text {
        text: String,
    },
}

/// 📰 Well-formed XML document root.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct XmlDocument {
    #[serde(default)]
    pub root: Option<XmlNode>,
}
//#endregion 🔖️XmlModel

//#region 🔖️Snapshot
/// 📸️ Persisted `stdio.xml` snapshot.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.xml")]
pub struct XmlSnapshot {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub doc: XmlDocument,
}

impl Default for XmlSnapshot {
    fn default() -> Self {
        Self {
            schema: STDIO_XML_DOCUMENT_SCHEMA.into(),
            doc: XmlDocument::default(),
        }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️XmlTextCodec
fn xml_escape_text(s: &str) -> String {
    let mut out = String::new();
    for ch in s.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&apos;"),
            _ => out.push(ch),
        }
    }
    out
}

fn xml_escape_attr(s: &str) -> String {
    xml_escape_text(s)
}

pub fn xml_document_to_text(doc: &XmlDocument) -> String {
    match &doc.root {
        None => String::new(),
        Some(node) => xml_node_to_text(node, 0),
    }
}

fn xml_node_to_text(node: &XmlNode, _depth: usize) -> String {
    match node {
        XmlNode::Text { text } => xml_escape_text(text),
        XmlNode::Element { name, attrs, children } => {
            let mut out = format!("<{}", name);
            for attr in attrs {
                out.push_str(&format!(" {}=\"{}\"", attr.name, xml_escape_attr(&attr.value)));
            }
            if children.is_empty() {
                out.push_str("/>");
                return out;
            }
            out.push('>');
            for child in children {
                out.push_str(&xml_node_to_text(child, _depth + 1));
            }
            out.push_str(&format!("</{}>", name));
            out
        }
    }
}

pub fn xml_document_from_text(text: &str) -> Result<XmlDocument, String> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return Ok(XmlDocument::default());
    }
    let mut pos = 0;
    skip_misc(trimmed, &mut pos)?;
    let root = parse_node(trimmed, &mut pos)?;
    skip_misc(trimmed, &mut pos)?;
    if pos < trimmed.len() {
        return Err("trailing content after root element".into());
    }
    Ok(XmlDocument { root: Some(root) })
}

fn skip_misc(s: &str, pos: &mut usize) -> Result<(), String> {
    loop {
        skip_ws(s, pos);
        if s[*pos..].starts_with("<?") {
            let end = s[*pos..].find("?>").ok_or("unclosed processing instruction")?;
            *pos += end + 2;
            continue;
        }
        if s[*pos..].starts_with("<!--") {
            let end = s[*pos..].find("-->").ok_or("unclosed comment")?;
            *pos += end + 3;
            continue;
        }
        break;
    }
    Ok(())
}

fn skip_ws(s: &str, pos: &mut usize) {
    while *pos < s.len() && s.as_bytes()[*pos].is_ascii_whitespace() {
        *pos += 1;
    }
}

fn parse_name(s: &str, pos: &mut usize) -> Result<String, String> {
    let start = *pos;
    if *pos >= s.len() || !is_name_start(s[*pos..].chars().next().unwrap()) {
        return Err("expected XML name".into());
    }
    *pos += 1;
    while *pos < s.len() {
        let ch = s[*pos..].chars().next().unwrap();
        if is_name_char(ch) {
            *pos += ch.len_utf8();
        } else {
            break;
        }
    }
    Ok(s[start..*pos].to_string())
}

fn is_name_start(ch: char) -> bool {
    ch == ':' || ch.is_ascii_alphabetic() || ch == '_'
}

fn is_name_char(ch: char) -> bool {
    is_name_start(ch) || ch.is_ascii_digit() || ch == '-' || ch == '.'
}

fn parse_attr_value(s: &str, pos: &mut usize) -> Result<String, String> {
    skip_ws(s, pos);
    let quote = s[*pos..].chars().next().ok_or("expected attribute value")?;
    if quote != '"' && quote != '\'' {
        return Err("attribute value must be quoted".into());
    }
    *pos += 1;
    let start = *pos;
    while *pos < s.len() {
        let ch = s[*pos..].chars().next().unwrap();
        if ch == quote {
            let value = s[start..*pos].to_string();
            *pos += 1;
            return Ok(value);
        }
        *pos += ch.len_utf8();
    }
    Err("unclosed attribute value".into())
}

fn parse_attrs(s: &str, pos: &mut usize) -> Result<Vec<XmlAttr>, String> {
    let mut attrs = Vec::new();
    loop {
        skip_ws(s, pos);
        if *pos >= s.len() || s[*pos..].starts_with(">") || s[*pos..].starts_with("/>") {
            break;
        }
        let name = parse_name(s, pos)?;
        skip_ws(s, pos);
        if s[*pos..].chars().next() != Some('=') {
            return Err("expected = in attribute".into());
        }
        *pos += 1;
        let value = parse_attr_value(s, pos)?;
        attrs.push(XmlAttr { name, value });
    }
    Ok(attrs)
}

fn parse_node(s: &str, pos: &mut usize) -> Result<XmlNode, String> {
    skip_misc(s, pos)?;
    if *pos >= s.len() || !s[*pos..].starts_with('<') {
        return Err("expected element start".into());
    }
  if s[*pos..].starts_with("</") {
        return Err("unexpected closing tag".into());
    }
    *pos += 1;
    let name = parse_name(s, pos)?;
    let attrs = parse_attrs(s, pos)?;
    skip_ws(s, pos);
    if s[*pos..].starts_with("/>") {
        *pos += 2;
        return Ok(XmlNode::Element { name, attrs, children: vec![] });
    }
    if !s[*pos..].starts_with('>') {
        return Err("expected > or />".into());
    }
    *pos += 1;
    let mut children = Vec::new();
    loop {
        skip_misc(s, pos)?;
        if s[*pos..].starts_with("</") {
            *pos += 2;
            let close = parse_name(s, pos)?;
            skip_ws(s, pos);
            if !s[*pos..].starts_with('>') {
                return Err("expected > on closing tag".into());
            }
            *pos += 1;
            if close != name {
                return Err(format!("closing tag mismatch: expected </{}>, got </{}>", name, close));
            }
            break;
        }
        if *pos < s.len() && s[*pos..].starts_with('<') {
            children.push(parse_node(s, pos)?);
            continue;
        }
        let start = *pos;
        while *pos < s.len() && !s[*pos..].starts_with('<') {
            *pos += 1;
        }
        let text = s[start..*pos].to_string();
        if !text.is_empty() {
            children.push(XmlNode::Text { text });
        }
    }
    Ok(XmlNode::Element { name, attrs, children })
}
//#endregion 🔖️XmlTextCodec

//#region 🔖️HandcraftedDocumentCodecs
impl store::DocumentDsl for XmlSnapshot {
    const EXTENSION: &'static str = "xml";
    fn envelope_id() -> &'static str { "stdio.xml" }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let doc = xml_document_from_text(body).map_err(|e| {
            store::TextError::new(format!("xml parse: {e}"), dsl::TextSpan::at(1, 1))
        })?;
        Ok(Self { schema: STDIO_XML_DOCUMENT_SCHEMA.into(), doc })
    }
    fn print_dsl(&self) -> String {
        let body = xml_document_to_text(&self.doc);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::DocumentPack for XmlSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = serde_json::to_vec(&self.doc).map_err(|e| store::PackError::Schema(e.to_string()))?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        ).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes)
            .map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::DocumentDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::DocumentDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let _ = options;
        let doc = serde_json::from_slice(&inner).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(Self { schema: STDIO_XML_DOCUMENT_SCHEMA.into(), doc })
    }
}
//#endregion 🔖️HandcraftedDocumentCodecs
