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

/// 🌳 XML node: element, text, CDATA, comment, or processing instruction. `CData`/`Comment`/
/// `ProcessingInstruction` are distinct from `Text` (rather than folding them into escaped text)
/// so decode->encode preserves the ORIGINAL form -- a `<![CDATA[...]]>` section (common inside
/// real SVG `<style>`/`<script>` elements) re-emits as CDATA, not as entity-escaped text, and a
/// `<!--comment-->` between siblings survives instead of being silently dropped.
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
    /// 🔤️ Character data. Entities (`&amp;` `&lt;` `&gt;` `&quot;` `&apos;` `&#NNN;` `&#xHHHH;`)
    /// are decoded to their literal characters on read and re-escaped on write -- `text` here is
    /// always the LITERAL (unescaped) content, never the wire form.
    Text { text: String },
    /// 📦️ `<![CDATA[...]]>` section -- `text` is the literal content, verbatim, never escaped.
    CData { text: String },
    /// 💬️ `<!--...-->` comment, preserved verbatim (not interpreted, not escaped).
    Comment { text: String },
    /// ❓️ `<?target data?>` processing instruction (anywhere a PI can appear inside content --
    /// the XML *declaration* itself, `<?xml version="1.0"?>`, is handled separately and not
    /// represented as a node).
    ProcessingInstruction { target: String, data: String },
}

/// 📰 Well-formed XML document root.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct XmlDocument {
    #[serde(default)]
    pub root: Option<XmlNode>,
    /// 📜️ Parsed document type declaration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doctype: Option<XmlDoctype>,
    /// 🏳️ The typed `<?xml version="1.0" encoding="..." standalone="..."?>` XML declaration, if
    /// the source document had one -- unlike `doctype` this IS structurally decoded (three named
    /// fields) since `version`/`encoding`/`standalone` are each independently meaningful and each
    /// independently diffable/mutable (`XmlMutation::SetDeclaration`), where a raw-string DOCTYPE
    /// has no such sub-structure worth decoding.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub declaration: Option<XmlDeclaration>,
    /// 🧭 Logical comments and processing instructions preceding the root element.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub prolog: Vec<XmlNode>,
}

/// 📜️ Logical XML document type declaration.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct XmlDoctype {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_id: Option<XmlExternalId>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub declarations: Vec<XmlDtdDeclaration>,
}

impl From<&str> for XmlDoctype {
    fn from(value: &str) -> Self {
        semio_framework_plugin::resolve_ready(parse_doctype(value)).expect("valid XML document type literal")
    }
}

/// 🔗️ Standard SYSTEM or PUBLIC external identifier.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum XmlExternalId {
    System { system_id: String },
    Public { public_id: String, system_id: String },
}

/// 🏷️ Parsed internal general or parameter entity declaration.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum XmlDtdDeclaration {
    Entity { parameter: bool, name: String, value: String },
}

/// 🏳️ Typed XML declaration (`<?xml version="1.0" encoding="UTF-8" standalone="yes"?>`).
/// `version` is mandatory per the XML 1.0 spec whenever a declaration is present at all;
/// `encoding`/`standalone` are each independently optional.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct XmlDeclaration {
    pub version: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub standalone: Option<bool>,
}

//#endregion 🔖️XmlModel

//#region 🔖️Snapshot
/// 📸️ Persisted `stdio.xml` snapshot.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.xml")]
pub struct XmlSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[serde(default)]
    pub doc: XmlDocument,
}

impl Default for XmlSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_XML_DOCUMENT_SCHEMA.into(), doc: XmlDocument::default() }
    }
}

impl XmlSnapshot {
    /// 🪞️ Returns the lossless logical state used by diff and mutation laws.
    pub async fn semantic_projection(&self) -> Self {
        self.clone()
    }

    /// 📥️ Parses XML into its lossless logical model.
    pub async fn import_utf8(bytes: &[u8]) -> Result<Self, String> {
        let text = std::str::from_utf8(bytes).map_err(|error| error.to_string())?;
        Ok(Self { schema: STDIO_XML_DOCUMENT_SCHEMA.into(), doc: xml_document_from_text(text).await? })
    }

    /// 📤️ Deterministically materializes XML from the logical model.
    pub async fn export_utf8(&self) -> Result<Vec<u8>, String> {
        Ok(xml_document_to_text(&self.doc).await.into_bytes())
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️XmlTextCodec
async fn xml_escape_text(s: &str) -> String {
    let mut out = String::new();
    for ch in s.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            _ => out.push(ch),
        }
    }
    out
}

async fn xml_escape_attr(s: &str) -> String {
    let mut out = String::new();
    for ch in s.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '"' => out.push_str("&quot;"),
            _ => out.push(ch),
        }
    }
    out
}

/// 🔓️ Decode the five predefined XML entities plus numeric character references (`&#NNN;`,
/// `&#xHHHH;`/`&#XHHHH;`) into their literal characters. This is the read-side half that was
/// entirely missing before: without it, `&amp;` in a source document is kept as the 5 literal
/// characters `&`,`a`,`m`,`p`,`;` in the `Text` node, and the NEXT write-side escape turns that
/// lone `&` into `&amp;` again -- so every decode->encode cycle grows `&amp;` into `&amp;amp;`
/// into `&amp;amp;amp;`, permanently corrupting the document. An unrecognized/malformed entity is
/// a hard parse error (never silently dropped or passed through raw).
async fn xml_unescape_text(s: &str) -> Result<String, String> {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.char_indices().peekable();
    while let Some((i, ch)) = chars.next() {
        if ch != '&' {
            out.push(ch);
            continue;
        }
        let rest = &s[i..];
        let end = rest.find(';').ok_or_else(|| format!("unterminated entity reference at byte {i}"))?;
        let entity = &rest[1..end];
        let decoded = if let Some(numeric) = entity.strip_prefix('#') {
            let code = if let Some(hex) = numeric.strip_prefix('x').or_else(|| numeric.strip_prefix('X')) {
                u32::from_str_radix(hex, 16).map_err(|_| format!("invalid hex character reference &{entity};"))?
            } else {
                numeric.parse::<u32>().map_err(|_| format!("invalid decimal character reference &{entity};"))?
            };
            char::from_u32(code).ok_or_else(|| format!("invalid unicode scalar &{entity};"))?
        } else {
            match entity {
                "amp" => '&',
                "lt" => '<',
                "gt" => '>',
                "quot" => '"',
                "apos" => '\'',
                other => return Err(format!("unknown entity &{other};")),
            }
        };
        out.push(decoded);
        // Advance the char iterator past the consumed entity (end index is relative to `rest`,
        // i.e. `i`-relative; convert to an absolute byte offset and skip to just past the `;`).
        let consume_to = i + end + 1;
        while let Some(&(j, _)) = chars.peek() {
            if j < consume_to {
                chars.next();
            } else {
                break;
            }
        }
    }
    Ok(out)
}

pub async fn xml_document_to_text(doc: &XmlDocument) -> String {
    let mut out = String::new();
    if let Some(decl) = &doc.declaration {
        out.push_str("<?xml version=\"");
        out.push_str(&decl.version);
        out.push('\"');
        if let Some(encoding) = &decl.encoding {
            out.push_str(" encoding=\"");
            out.push_str(encoding);
            out.push('\"');
        }
        if let Some(standalone) = decl.standalone {
            out.push_str(" standalone=\"");
            out.push_str(if standalone { "yes" } else { "no" });
            out.push('\"');
        }
        out.push_str("?>\n");
    }
    for node in &doc.prolog {
        xml_node_to_text(node, 0, &mut out);
        out.push('\n');
    }
    if let Some(doctype) = &doc.doctype {
        xml_doctype_to_text(doctype, &mut out);
        out.push('\n');
    }
    if let Some(node) = &doc.root {
        xml_node_to_text(node, 0, &mut out);
    }
    out
}

async fn xml_doctype_to_text(doctype: &XmlDoctype, out: &mut String) {
    out.push_str("<!DOCTYPE ");
    out.push_str(&doctype.name);
    if let Some(external_id) = &doctype.external_id {
        match external_id {
            XmlExternalId::System { system_id } => {
                out.push_str(" SYSTEM \"");
                out.push_str(&xml_escape_attr(system_id));
                out.push('\"');
            }
            XmlExternalId::Public { public_id, system_id } => {
                out.push_str(" PUBLIC \"");
                out.push_str(&xml_escape_attr(public_id));
                out.push_str("\" \"");
                out.push_str(&xml_escape_attr(system_id));
                out.push('\"');
            }
        }
    }
    if !doctype.declarations.is_empty() {
        out.push_str(" [");
        for declaration in &doctype.declarations {
            match declaration {
                XmlDtdDeclaration::Entity { parameter, name, value } => {
                    out.push_str("<!ENTITY ");
                    if *parameter {
                        out.push_str("% ");
                    }
                    out.push_str(name);
                    out.push_str(" \"");
                    out.push_str(&xml_escape_attr(value));
                    out.push_str("\">");
                }
            }
        }
        out.push(']');
    }
    out.push('>');
}

async fn xml_current_column(out: &str) -> usize {
    out.rsplit_once('\n').map_or(out.len(), |(_, line)| line.len())
}

async fn xml_node_to_text(node: &XmlNode, depth: usize, out: &mut String) {
    match node {
        XmlNode::Text { text } => out.push_str(&xml_escape_text(text)),
        XmlNode::CData { text } => {
            out.push_str("<![CDATA[");
            out.push_str(text);
            out.push_str("]]>");
        }
        XmlNode::Comment { text } => {
            out.push_str("<!--");
            out.push_str(text);
            out.push_str("-->");
        }
        XmlNode::ProcessingInstruction { target, data } => {
            out.push_str("<?");
            out.push_str(target);
            if !data.is_empty() {
                out.push(' ');
                out.push_str(data);
            }
            out.push_str("?>");
        }
        XmlNode::Element { name, attrs, children } => {
            out.push('<');
            out.push_str(name);
            for (index, attr) in attrs.iter().enumerate() {
                let rendered = format!("{}=\"{}\"", attr.name, xml_escape_attr(&attr.value));
                let closing_width = if index + 1 == attrs.len() {
                    match children.as_slice() {
                        [] => 2,
                        [XmlNode::Text { text }] => xml_escape_text(text).await.chars().count() + name.len() + 4,
                        _ => 1,
                    }
                } else {
                    0
                };
                if xml_current_column(out) + 1 + rendered.len() + closing_width > 120 {
                    out.push('\n');
                    out.push_str(&" ".repeat((depth + 1) * 4));
                } else {
                    out.push(' ');
                }
                out.push_str(&rendered);
            }
            if children.is_empty() {
                out.push_str("/>");
                return;
            }
            out.push('>');
            for child in children {
                xml_node_to_text(child, depth + 1, out);
            }
            out.push_str("</");
            out.push_str(name);
            out.push('>');
        }
    }
}

pub async fn xml_document_from_text(text: &str) -> Result<XmlDocument, String> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return Ok(XmlDocument::default());
    }
    let mut pos = 0;
    let declaration = parse_xml_declaration_prolog(trimmed, &mut pos).await?;
    let (doctype, prolog) = skip_misc(trimmed, &mut pos).await?;
    let root = parse_node(trimmed, &mut pos).await?;
    let _ = skip_misc(trimmed, &mut pos).await?;
    if pos < trimmed.len() {
        return Err("trailing content after root element".into());
    }
    Ok(XmlDocument { root: Some(root), doctype, declaration, prolog })
}

/// 🏳️ Parses the leading `<?xml version="1.0" encoding="..." standalone="..."?>` declaration, if
/// present. Per the XML 1.0 spec the declaration (when present at all) MUST be the very first
/// thing in the document -- unlike ordinary processing instructions it is not represented as an
/// `XmlNode::ProcessingInstruction` and is only ever looked for here, at the very start.
async fn parse_xml_declaration_prolog(s: &str, pos: &mut usize) -> Result<Option<XmlDeclaration>, String> {
    if !s[*pos..].starts_with("<?xml") {
        return Ok(None);
    }
    // Distinguish the reserved `<?xml ...?>` declaration target from an ordinary PI whose target
    // merely starts with the same four letters (e.g. `<?xml-stylesheet ...?>`).
    let after = s[*pos + "<?xml".len()..].chars().next();
    match after {
        Some(c) if c.is_ascii_whitespace() || c == '?' => {}
        _ => return Ok(None),
    }
    *pos += "<?xml".len();
    let mut version = None;
    let mut encoding = None;
    let mut standalone = None;
    loop {
        skip_ws(s, pos);
        if s[*pos..].starts_with("?>") {
            break;
        }
        let name = parse_name(s, pos).await?;
        skip_ws(s, pos);
        if s[*pos..].chars().next() != Some('=') {
            return Err("expected = in xml declaration".into());
        }
        *pos += 1;
        let value = xml_unescape_text(&parse_attr_value(s, pos).await?).await?;
        match name.as_str() {
            "version" => version = Some(value),
            "encoding" => encoding = Some(value),
            "standalone" => standalone = Some(value == "yes"),
            other => return Err(format!("unknown xml declaration attribute {other}")),
        }
    }
    *pos += 2;
    Ok(Some(XmlDeclaration { version: version.ok_or("xml declaration missing version")?, encoding, standalone }))
}

/// 🚧️ Parses prolog processing instructions, comments, and a typed document declaration.
async fn skip_misc(s: &str, pos: &mut usize) -> Result<(Option<XmlDoctype>, Vec<XmlNode>), String> {
    let mut doctype = None;
    let mut nodes = Vec::new();
    loop {
        skip_ws(s, pos);
        if s[*pos..].starts_with("<?") {
            *pos += 2;
            let target = parse_name(s, pos).await?;
            skip_ws(s, pos);
            let end = s[*pos..].find("?>").ok_or("unclosed processing instruction")?;
            let data = s[*pos..*pos + end].to_string();
            *pos += end + 2;
            nodes.push(XmlNode::ProcessingInstruction { target, data });
            continue;
        }
        if s[*pos..].starts_with("<!--") {
            *pos += 4;
            let end = s[*pos..].find("-->").ok_or("unclosed comment")?;
            nodes.push(XmlNode::Comment { text: s[*pos..*pos + end].to_string() });
            *pos += end + 3;
            continue;
        }
        if s[*pos..].starts_with("<!DOCTYPE") || s[*pos..].starts_with("<!doctype") {
            let start = *pos;
            *pos += "<!DOCTYPE".len();
            let mut depth = 0i32;
            loop {
                if *pos >= s.len() {
                    return Err("unclosed DOCTYPE declaration".into());
                }
                let byte = s.as_bytes()[*pos];
                match byte {
                    b'[' => depth += 1,
                    b']' => depth -= 1,
                    b'>' if depth <= 0 => {
                        *pos += 1;
                        break;
                    }
                    _ => {}
                }
                *pos += 1;
            }
            doctype = Some(parse_doctype(&s[start..*pos]).await?);
            continue;
        }
        break;
    }
    Ok((doctype, nodes))
}

async fn parse_doctype(text: &str) -> Result<XmlDoctype, String> {
    let mut pos = "<!DOCTYPE".len();
    skip_ws(text, &mut pos);
    let name = parse_name(text, &mut pos).await?;
    skip_ws(text, &mut pos);
    let external_id = if text[pos..].starts_with("SYSTEM") {
        pos += "SYSTEM".len();
        Some(XmlExternalId::System { system_id: xml_unescape_text(&parse_attr_value(text, &mut pos).await?).await? })
    } else if text[pos..].starts_with("PUBLIC") {
        pos += "PUBLIC".len();
        let public_id = xml_unescape_text(&parse_attr_value(text, &mut pos).await?).await?;
        let system_id = xml_unescape_text(&parse_attr_value(text, &mut pos).await?).await?;
        Some(XmlExternalId::Public { public_id, system_id })
    } else {
        None
    };
    skip_ws(text, &mut pos);
    let mut declarations = Vec::new();
    if text[pos..].starts_with('[') {
        pos += 1;
        loop {
            skip_ws(text, &mut pos);
            if text[pos..].starts_with(']') {
                pos += 1;
                break;
            }
            if !text[pos..].starts_with("<!ENTITY") {
                return Err("unsupported XML DTD declaration; only typed ENTITY declarations are modeled".into());
            }
            pos += "<!ENTITY".len();
            skip_ws(text, &mut pos);
            let parameter = text[pos..].starts_with('%');
            if parameter {
                pos += 1;
                skip_ws(text, &mut pos);
            }
            let entity_name = parse_name(text, &mut pos).await?;
            let value = xml_unescape_text(&parse_attr_value(text, &mut pos).await?).await?;
            skip_ws(text, &mut pos);
            if !text[pos..].starts_with('>') {
                return Err("expected > after XML entity declaration".into());
            }
            pos += 1;
            declarations.push(XmlDtdDeclaration::Entity { parameter, name: entity_name, value });
        }
    }
    skip_ws(text, &mut pos);
    if !text[pos..].starts_with('>') {
        return Err("expected > after XML document type declaration".into());
    }
    pos += 1;
    if pos != text.len() {
        return Err("trailing content in XML document type declaration".into());
    }
    Ok(XmlDoctype { name, external_id, declarations })
}

async fn skip_ws(s: &str, pos: &mut usize) {
    while *pos < s.len() && s.as_bytes()[*pos].is_ascii_whitespace() {
        *pos += 1;
    }
}

async fn parse_name(s: &str, pos: &mut usize) -> Result<String, String> {
    let start = *pos;
    if *pos >= s.len() || !is_name_start(s[*pos..].chars().next().unwrap()) {
        return Err("expected XML name".into());
    }
    *pos += 1;
    while *pos < s.len() {
        let ch = s[*pos..].chars().next().unwrap();
        if is_name_char(ch).await {
            *pos += ch.len_utf8();
        } else {
            break;
        }
    }
    Ok(s[start..*pos].to_string())
}

async fn is_name_start(ch: char) -> bool {
    ch == ':' || ch.is_ascii_alphabetic() || ch == '_'
}

async fn is_name_char(ch: char) -> bool {
    is_name_start(ch).await || ch.is_ascii_digit() || ch == '-' || ch == '.'
}

async fn parse_attr_value(s: &str, pos: &mut usize) -> Result<String, String> {
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

async fn parse_attrs(s: &str, pos: &mut usize) -> Result<Vec<XmlAttr>, String> {
    let mut attrs = Vec::new();
    loop {
        skip_ws(s, pos);
        if *pos >= s.len() || s[*pos..].starts_with(">") || s[*pos..].starts_with("/>") {
            break;
        }
        let name = parse_name(s, pos).await?;
        skip_ws(s, pos);
        if s[*pos..].chars().next() != Some('=') {
            return Err("expected = in attribute".into());
        }
        *pos += 1;
        let value = xml_unescape_text(&parse_attr_value(s, pos).await?).await?;
        attrs.push(XmlAttr { name, value });
    }
    Ok(attrs)
}

async fn parse_node(s: &str, pos: &mut usize) -> Result<XmlNode, String> {
    if *pos >= s.len() || !s[*pos..].starts_with('<') {
        return Err("expected element start".into());
    }
    if s[*pos..].starts_with("</") {
        return Err("unexpected closing tag".into());
    }
    *pos += 1;
    let name = parse_name(s, pos).await?;
    let attrs = parse_attrs(s, pos).await?;
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
        if *pos >= s.len() {
            return Err(format!("unclosed element <{name}>: unexpected end of input"));
        }
        if s[*pos..].starts_with("</") {
            *pos += 2;
            let close = parse_name(s, pos).await?;
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
        if s[*pos..].starts_with("<![CDATA[") {
            *pos += "<![CDATA[".len();
            let end = s[*pos..].find("]]>").ok_or("unclosed CDATA section")?;
            let text = s[*pos..*pos + end].to_string();
            *pos += end + 3;
            children.push(XmlNode::CData { text });
            continue;
        }
        if s[*pos..].starts_with("<!--") {
            *pos += 4;
            let end = s[*pos..].find("-->").ok_or("unclosed comment")?;
            let text = s[*pos..*pos + end].to_string();
            *pos += end + 3;
            children.push(XmlNode::Comment { text });
            continue;
        }
        if s[*pos..].starts_with("<?") {
            *pos += 2;
            let target = parse_name(s, pos).await?;
            skip_ws(s, pos);
            let end = s[*pos..].find("?>").ok_or("unclosed processing instruction")?;
            let data = s[*pos..*pos + end].to_string();
            *pos += end + 2;
            children.push(XmlNode::ProcessingInstruction { target, data });
            continue;
        }
        if s[*pos..].starts_with('<') {
            children.push(parse_node(s, pos).await?);
            continue;
        }
        let start = *pos;
        while *pos < s.len() && !s[*pos..].starts_with('<') {
            *pos += s[*pos..].chars().next().unwrap().len_utf8();
        }
        let raw = &s[start..*pos];
        if !raw.is_empty() {
            let text = xml_unescape_text(raw).await?;
            children.push(XmlNode::Text { text });
        }
    }
    Ok(XmlNode::Element { name, attrs, children })
}
//#endregion 🔖️XmlTextCodec

//#region 🔖️HandcraftedArtifactCodecs
impl store::ArtifactDsl for XmlSnapshot {
    const EXTENSION: &'static str = "xml";
    async fn envelope_id() -> &'static str {
        "stdio.xml"
    }

    async fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        match store::semio_format::split_text_preamble(text) {
            Ok((_, body)) => crate::artifacts::xml::schema::mutations::dec_xml_snapshot(body.trim()).await.map_err(|e| store::TextError::new(format!("xml state parse: {e}"), dsl::TextSpan::at(1, 1))),
            Err(_) => Self::import_utf8(text.as_bytes()).await.map_err(|e| store::TextError::new(format!("xml parse: {e}"), dsl::TextSpan::at(1, 1))),
        }
    }
    async fn print_dsl(&self) -> String {
        let body = crate::artifacts::xml::schema::mutations::enc_xml_snapshot(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id().await, store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 🧪️ P2-FG1: `stdio.xml` is TEXT-NATIVE (per the W0 census row) — there is no "binary XML"; the
/// pack container is the SEMIO envelope wrapping the artifact's own REAL wire text
/// (`xml_document_to_text`/`xml_document_from_text`) verbatim, same treatment json's own
/// `ArtifactPack` gives its RFC8259 text (`🔣️json/…/📸️snapshot/🦀️component.rs`'s
/// `write_json_text(&self.value).into_bytes()`). Replaces the previous `serde_json::to_vec`/
/// `from_slice` placeholder, which satisfied the trait but was a literal-JSON-payload-disguised-as-
/// binary violation of `POLICY_STDIO_JSON_TRANSFER_BAN` (flagged by name in the P2-W0 recon report,
/// `xml` row, "Yes — in scope").
impl store::ArtifactPack for XmlSnapshot {
    async fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let mut raw = vec![1];
        crate::artifacts::xml::schema::mutations::enc_xml_snapshot_bin(self, &mut raw);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id().await, store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    async fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let _ = options;
        let mut reader = store::ByteReader::new(&inner).await;
        let version = reader.read_u8().await.map_err(|e| store::PackError::Schema(e.to_string()))?;
        if version != 1 {
            return Err(store::PackError::Schema(format!("unsupported xml snapshot state version {version}")));
        }
        crate::artifacts::xml::schema::mutations::dec_xml_snapshot_bin(&mut reader).await.map_err(store::PackError::Schema)
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs
