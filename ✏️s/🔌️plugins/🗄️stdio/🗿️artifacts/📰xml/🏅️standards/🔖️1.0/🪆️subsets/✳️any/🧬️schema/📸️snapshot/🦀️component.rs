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
    Text {
        text: String,
    },
    /// 📦️ `<![CDATA[...]]>` section -- `text` is the literal content, verbatim, never escaped.
    CData {
        text: String,
    },
    /// 💬️ `<!--...-->` comment, preserved verbatim (not interpreted, not escaped).
    Comment {
        text: String,
    },
    /// ❓️ `<?target data?>` processing instruction (anywhere a PI can appear inside content --
    /// the XML *declaration* itself, `<?xml version="1.0"?>`, is handled separately and not
    /// represented as a node).
    ProcessingInstruction {
        target: String,
        data: String,
    },
}

/// 📰 Well-formed XML document root.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct XmlDocument {
    #[serde(default)]
    pub root: Option<XmlNode>,
    /// 📜️ The raw `<!DOCTYPE ...>` declaration text (if present), kept verbatim -- NOT deeply
    /// parsed (no DTD validation), just preserved so real files that carry one (most SVG 1.1
    /// files exported by Illustrator/Inkscape do) parse at all instead of hard-failing, and
    /// round-trip losslessly.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doctype: Option<String>,
    /// 🏳️ The typed `<?xml version="1.0" encoding="..." standalone="..."?>` XML declaration, if
    /// the source document had one -- unlike `doctype` this IS structurally decoded (three named
    /// fields) since `version`/`encoding`/`standalone` are each independently meaningful and each
    /// independently diffable/mutable (`XmlMutation::SetDeclaration`), where a raw-string DOCTYPE
    /// has no such sub-structure worth decoding.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub declaration: Option<XmlDeclaration>,
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
            _ => out.push(ch),
        }
    }
    out
}

fn xml_escape_attr(s: &str) -> String {
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
fn xml_unescape_text(s: &str) -> Result<String, String> {
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

pub fn xml_document_to_text(doc: &XmlDocument) -> String {
    let mut out = String::new();
    if let Some(decl) = &doc.declaration {
        out.push_str("<?xml version=\"");
        out.push_str(&decl.version);
        out.push('"');
        if let Some(encoding) = &decl.encoding {
            out.push_str(" encoding=\"");
            out.push_str(encoding);
            out.push('"');
        }
        if let Some(standalone) = decl.standalone {
            out.push_str(" standalone=\"");
            out.push_str(if standalone { "yes" } else { "no" });
            out.push('"');
        }
        out.push_str("?>\n");
    }
    if let Some(doctype) = &doc.doctype {
        out.push_str(doctype);
        out.push('\n');
    }
    if let Some(node) = &doc.root {
        out.push_str(&xml_node_to_text(node, 0));
    }
    out
}

fn xml_node_to_text(node: &XmlNode, _depth: usize) -> String {
    match node {
        XmlNode::Text { text } => xml_escape_text(text),
        XmlNode::CData { text } => format!("<![CDATA[{text}]]>"),
        XmlNode::Comment { text } => format!("<!--{text}-->"),
        XmlNode::ProcessingInstruction { target, data } => {
            if data.is_empty() { format!("<?{target}?>") } else { format!("<?{target} {data}?>") }
        }
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
    let declaration = parse_xml_declaration_prolog(trimmed, &mut pos)?;
    let doctype = skip_misc(trimmed, &mut pos)?;
    let root = parse_node(trimmed, &mut pos)?;
    skip_misc(trimmed, &mut pos)?;
    if pos < trimmed.len() {
        return Err("trailing content after root element".into());
    }
    Ok(XmlDocument { root: Some(root), doctype, declaration })
}

/// 🏳️ Parses the leading `<?xml version="1.0" encoding="..." standalone="..."?>` declaration, if
/// present. Per the XML 1.0 spec the declaration (when present at all) MUST be the very first
/// thing in the document -- unlike ordinary processing instructions it is not represented as an
/// `XmlNode::ProcessingInstruction` and is only ever looked for here, at the very start.
fn parse_xml_declaration_prolog(s: &str, pos: &mut usize) -> Result<Option<XmlDeclaration>, String> {
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
        let name = parse_name(s, pos)?;
        skip_ws(s, pos);
        if s[*pos..].chars().next() != Some('=') {
            return Err("expected = in xml declaration".into());
        }
        *pos += 1;
        let value = parse_attr_value(s, pos)?;
        match name.as_str() {
            "version" => version = Some(value),
            "encoding" => encoding = Some(value),
            "standalone" => standalone = Some(value == "yes"),
            other => return Err(format!("unknown xml declaration attribute {other}")),
        }
    }
    *pos += 2;
    Ok(Some(XmlDeclaration {
        version: version.ok_or("xml declaration missing version")?,
        encoding,
        standalone,
    }))
}

/// 🚧️ Skips XML-declaration (`<?xml ...?>`), processing instructions, comments, and a `<!DOCTYPE
/// ...>` declaration (incl. an internal subset in `[...]`, which itself may contain nested `[`/`]`
/// in entity declarations -- bracket-depth-tracked, not a naive `find("]>")`). Returns the raw
/// DOCTYPE text if one was seen (prolog PIs/comments before the root element are still discarded
/// -- only the root subtree and the doctype are represented in the model). Before this fix, ANY
/// `<!DOCTYPE ...>` caused a hard parse failure one level up (`parse_name` rejects the leading
/// `!`), which is why most real-world SVG 1.1 files (virtually all of which declare one) could
/// not be parsed at all.
fn skip_misc(s: &str, pos: &mut usize) -> Result<Option<String>, String> {
    let mut doctype = None;
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
            doctype = Some(s[start..*pos].to_string());
            continue;
        }
        break;
    }
    Ok(doctype)
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
        if *pos >= s.len() {
            return Err(format!("unclosed element <{name}>: unexpected end of input"));
        }
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
            let target = parse_name(s, pos)?;
            skip_ws(s, pos);
            let end = s[*pos..].find("?>").ok_or("unclosed processing instruction")?;
            let data = s[*pos..*pos + end].to_string();
            *pos += end + 2;
            children.push(XmlNode::ProcessingInstruction { target, data });
            continue;
        }
        if s[*pos..].starts_with('<') {
            children.push(parse_node(s, pos)?);
            continue;
        }
        let start = *pos;
        while *pos < s.len() && !s[*pos..].starts_with('<') {
            *pos += 1;
        }
        let raw = &s[start..*pos];
        if !raw.is_empty() {
            let text = xml_unescape_text(raw)?;
            children.push(XmlNode::Text { text });
        }
    }
    Ok(XmlNode::Element { name, attrs, children })
}
//#endregion 🔖️XmlTextCodec

//#region 🔖️HandcraftedArtifactCodecs
impl store::ArtifactDsl for XmlSnapshot {
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
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for XmlSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = serde_json::to_vec(&self.doc).map_err(|e| store::PackError::Schema(e.to_string()))?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        ).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes)
            .map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::ArtifactDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let _ = options;
        let doc = serde_json::from_slice(&inner).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(Self { schema: STDIO_XML_DOCUMENT_SCHEMA.into(), doc })
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs
