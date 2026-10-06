//! 🧬️ XmlSnapshot schema — persistent fields + real codecs.

use crate::STDIO_XML_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;

//#region 🔖️XmlModel
/// 🏷️ XML attribute pair.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, Default)]
#[value(rename_all = "camelCase")]
pub struct XmlAttr {
    pub name: String,
    pub value: String,
}

/// 🌳 XML node: element, text, CDATA, comment, or processing instruction. `CData`/`Comment`/
/// `ProcessingInstruction` are distinct from `Text` (rather than folding them into escaped text)
/// so decode->encode preserves the ORIGINAL form -- a `<![CDATA[...]]>` section (common inside
/// real SVG `<style>`/`<script>` elements) re-emits as CDATA, not as entity-escaped text, and a
/// `<!--comment-->` between siblings survives instead of being silently dropped.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum XmlNode {
    Element {
        name: String,
        #[value(default)]
        attrs: Vec<XmlAttr>,
        #[value(default)]
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
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, Default)]
#[value(rename_all = "camelCase")]
pub struct XmlDocument {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub root: Option<XmlNode>,
    /// 📜️ Parsed document type declaration.
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub doctype: Option<XmlDoctype>,
    /// 🏳️ The typed `<?xml version="1.0" encoding="..." standalone="..."?>` XML declaration, if
    /// the source document had one -- unlike `doctype` this IS structurally decoded (three named
    /// fields) since `version`/`encoding`/`standalone` are each independently meaningful and each
    /// independently diffable/mutable (`XmlMutation::SetDeclaration`), where a raw-string DOCTYPE
    /// has no such sub-structure worth decoding.
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub declaration: Option<XmlDeclaration>,
    /// 🧭 Logical comments and processing instructions preceding the root element.
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub prolog: Vec<XmlNode>,
    /// 🧹 Logical comments and processing instructions following the root element.
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub epilog: Vec<XmlNode>,
}

/// 📜️ Logical XML document type declaration.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, Default)]
#[value(rename_all = "camelCase")]
pub struct XmlDoctype {
    /// 🧭 Number of logical prolog nodes preceding this declaration.
    #[value(default, skip_serializing_if = "is_zero", serialize_with="position::to_value", deserialize_with="position::from_value", serialize_controlled_with="position::to_value_controlled", deserialize_controlled_with="position::from_value_controlled", retire_with="std::mem::drop")]
    pub prolog_position: u64,
    pub name: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub external_id: Option<XmlExternalId>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub declarations: Vec<XmlDtdDeclaration>,
}

fn is_zero(value: &u64) -> bool {
    *value == 0
}

impl From<&str> for XmlDoctype {
    fn from(value: &str) -> Self {
        parse_doctype(value).expect("valid XML document type literal")
    }
}

/// 🔗️ Standard SYSTEM or PUBLIC external identifier.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum XmlExternalId {
    System { system_id: String },
    Public { public_id: String, system_id: String },
}

/// 🏷️ Parsed internal general or parameter entity declaration.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum XmlDtdDeclaration {
    Entity { parameter: bool, name: String, value: String },
}

/// 🗣️ Which delimiter quotes the XML declaration's pseudo-attribute values. XML 1.0 §2.8 admits
/// `"` and `'` interchangeably, so this is real document state rather than retained source text: a
/// document written `<?xml version='1.0' encoding='UTF-8'?>` has to come back out that way
/// (`🎨️svg`'s own `exact_native_analyzer_text_and_pack_roundtrip`/`…_composer_…` laws read a real
/// third-party file byte for byte, and the declaration was the one place the writer normalized).
/// `Double` is the default spelling, so a declaration that never names a quote is the one every
/// generator in this tree emits and every committed fixture already carries.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, value_derive::ToValue, value_derive::FromValue, value_derive::RetainedClone, value_derive::RetireOwned)]
#[value(rename_all = "camelCase")]
pub enum XmlQuote {
    #[default]
    Double,
    Single,
}

impl XmlQuote {
    /// 🔡️ The delimiter itself.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn as_char(self) -> char {
        match self {
            Self::Double => '"',
            Self::Single => '\'',
        }
    }

    /// 🔡️ The delimiter a parser just consumed; anything that is not `'` is the double quote,
    /// because `parse_attr_value` has already rejected every other byte in that position.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_char(quote: char) -> Self {
        if quote == '\'' {
            Self::Single
        } else {
            Self::Double
        }
    }

    /// 🫥 `skip_serializing_if` for the default spelling: a double-quoted declaration writes no
    /// `quote` key at all, which is what keeps every pre-existing wire value byte-identical.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double)
    }
}

/// 🏳️ Typed XML declaration (`<?xml version="1.0" encoding="UTF-8" standalone="yes"?>`).
/// `version` is mandatory per the XML 1.0 spec whenever a declaration is present at all;
/// `encoding`/`standalone` are each independently optional. `quote` is the delimiter all three
/// pseudo-attributes are written with (see [`XmlQuote`]).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, Default)]
#[value(rename_all = "camelCase")]
pub struct XmlDeclaration {
    pub version: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub standalone: Option<bool>,
    #[value(default, skip_serializing_if = "XmlQuote::is_double")]
    pub quote: XmlQuote,
}

impl XmlDeclaration {
    /// 🏳️ A declaration in the DEFAULT spelling — every caller that MINTS one (rather than reading
    /// one out of a document) goes through here, so adding a further modeled facet of the
    /// declaration can never again silently miss a struct literal. A reader that recovered a real
    /// delimiter builds the struct directly and sets [`XmlDeclaration::quote`] itself.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn new(version: impl Into<String>, encoding: Option<String>, standalone: Option<bool>) -> Self {
        Self { version: version.into(), encoding, standalone, quote: XmlQuote::Double }
    }
}

//#endregion 🔖️XmlModel

//#region 🔖️Snapshot
/// 📸️ Persisted `stdio.xml` snapshot.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.xml")]
pub struct XmlSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default)]
    pub doc: XmlDocument,
}

impl Default for XmlSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_XML_DOCUMENT_SCHEMA.into(), doc: XmlDocument::default() }
    }
}

impl XmlSnapshot {
    /// 🪞️ Returns the lossless logical state used by diff and mutation laws.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn semantic_projection(&self) -> Self {
        self.clone()
    }

    /// 📥️ Parses XML into its lossless logical model.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn import_utf8(bytes: &[u8]) -> Result<Self, String> {
        let text = std::str::from_utf8(bytes).map_err(|error| error.to_string())?;
        Ok(Self { schema: STDIO_XML_DOCUMENT_SCHEMA.into(), doc: xml_document_from_text(text)? })
    }

    /// 📤️ Deterministically materializes XML from the logical model.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn export_utf8(&self) -> Result<Vec<u8>, String> {
        Ok(xml_document_to_text_checked(&self.doc)?.into_bytes())
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️XmlTextCodec
/// 🔤 Escapes character data for text-node content. Per XML 1.0 §2.11, only the two-character
/// sequence `#xD #xA` and any lone `#xD` are normalized (to `#xA`) on the NEXT parse -- a literal
/// tab or `\n` is legal, untouched, and round-trips as-is, so only `\r` needs re-escaping here
/// (as `&#13;`) to survive; escaping `\n` too would be a needless (though harmless) divergence
/// from what the spec actually requires. This is deliberately narrower than [`xml_escape_attr`] --
/// see that function's doc for why attribute values need a wider set of characters escaped.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn xml_escape_text(s: &str) -> String {
    let mut out = String::new();
    for ch in s.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '\r' => out.push_str("&#13;"),
            _ => out.push(ch),
        }
    }
    out
}

/// 🔤 Escapes character data for a double-quoted attribute value. Per XML 1.0 §3.3.3, attribute
/// value normalization replaces every literal tab/`\n`/`\r` with a single space on the NEXT parse
/// (after line-break normalization already folds `\r`/`\r\n` to `\n`) -- but a character reference
/// like `&#9;`/`&#10;`/`&#13;` is exempt from that step and survives verbatim. So a value decoded
/// from such a reference (real example: the folded base64 `xlink:href` in the committed
/// `qr-code.svg` fixture, which carries dozens of `&#10;`) MUST be re-escaped as a reference on
/// write, or the byte written is a literal newline that silently collapses to a space next parse,
/// changing the value's meaning. This is deliberately wider than [`xml_escape_text`], whose text
/// content has no such normalization step for `\t`/`\n`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn xml_escape_attr(s: &str) -> String {
    let mut out = String::new();
    for ch in s.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '"' => out.push_str("&quot;"),
            '\t' => out.push_str("&#9;"),
            '\n' => out.push_str("&#10;"),
            '\r' => out.push_str("&#13;"),
            _ => out.push(ch),
        }
    }
    out
}



// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn xml_document_to_text(doc: &XmlDocument) -> String {
    xml_document_to_text_checked(doc).expect("valid XML document boundaries")
}

/// 📤 Validates and deterministically materializes one logical XML document.
pub fn xml_document_to_text_checked(doc: &XmlDocument) -> Result<String, String> {
    validate_xml_document_boundaries(doc)?;
    let mut out = String::new();
    if let Some(decl) = &doc.declaration {
        let quote = decl.quote.as_char();
        out.push_str("<?xml version=");
        out.push(quote);
        out.push_str(&decl.version);
        out.push(quote);
        if let Some(encoding) = &decl.encoding {
            out.push_str(" encoding=");
            out.push(quote);
            out.push_str(encoding);
            out.push(quote);
        }
        if let Some(standalone) = decl.standalone {
            out.push_str(" standalone=");
            out.push(quote);
            out.push_str(if standalone { "yes" } else { "no" });
            out.push(quote);
        }
        out.push_str("?>\n");
    }
    for (index, node) in doc.prolog.iter().enumerate() {
        if doc.doctype.as_ref().is_some_and(|doctype| doctype.prolog_position == index as u64) {
            xml_doctype_to_text(doc.doctype.as_ref().expect("checked doctype"), &mut out);
            out.push('\n');
        }
        xml_node_to_text(node, 0, &mut out);
        out.push('\n');
    }
    if let Some(doctype) = doc.doctype.as_ref().filter(|doctype| doctype.prolog_position == doc.prolog.len() as u64) {
        xml_doctype_to_text(doctype, &mut out);
        out.push('\n');
    }
    if let Some(node) = &doc.root {
        xml_node_to_text(node, 0, &mut out);
    }
    for node in &doc.epilog {
        if !out.is_empty() {
            out.push('\n');
        }
        xml_node_to_text(node, 0, &mut out);
    }
    Ok(out)
}

/// 🛡️ Rejects declaration or boundary state that cannot be published by the UTF-8 XML codec.
pub fn validate_xml_document_boundaries(doc: &XmlDocument) -> Result<(), String> {
    if let Some(declaration) = &doc.declaration {
        let delimiter = declaration.quote.as_char();
        if declaration.version.contains(delimiter) {
            return Err(format!("XML declaration version contains its {} quote delimiter", if delimiter == '"' { "double" } else { "single" }));
        }
        let valid_version = declaration.version.strip_prefix("1.").is_some_and(|minor| !minor.is_empty() && minor.chars().all(|character| character.is_ascii_digit()));
        if !valid_version {
            return Err(format!("XML declaration version {} is not a valid VersionNum", declaration.version));
        }
        if let Some(encoding) = &declaration.encoding {
            if encoding.contains(delimiter) {
                return Err(format!("XML declaration encoding contains its {} quote delimiter", if delimiter == '"' { "double" } else { "single" }));
            }
            let mut characters = encoding.chars();
            if !characters.next().is_some_and(|character| character.is_ascii_alphabetic()) || !characters.all(|character| character.is_ascii_alphanumeric() || matches!(character, '.' | '_' | '-')) {
                return Err(format!("XML declaration encoding {encoding} is not a valid EncName"));
            }
            if !encoding.eq_ignore_ascii_case("UTF-8") {
                return Err(format!("XML UTF-8 transport cannot declare encoding {encoding}"));
            }
        }
    }
    for (boundary, nodes) in [("prolog", &doc.prolog), ("epilog", &doc.epilog)] {
        if nodes.iter().any(|node| !matches!(node, XmlNode::Comment { .. } | XmlNode::ProcessingInstruction { .. })) {
            return Err(format!("XML {boundary} may contain only comments and processing instructions"));
        }
    }
    if let Some(doctype) = &doc.doctype {
        if doctype.prolog_position > doc.prolog.len() as u64 {
            return Err(format!("DOCTYPE prolog position {} exceeds prolog length {}", doctype.prolog_position, doc.prolog.len()));
        }
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn xml_doctype_to_text(doctype: &XmlDoctype, out: &mut String) {
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn xml_current_column(out: &str) -> usize {
    out.rsplit_once('\n').map_or(out.len(), |(_, line)| line.len())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn xml_node_to_text(node: &XmlNode, depth: usize, out: &mut String) {
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
                        [XmlNode::Text { text }] => xml_escape_text(text).chars().count() + name.len() + 4,
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn xml_document_from_text(text: &str) -> Result<XmlDocument, String> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return Ok(XmlDocument::default());
    }
    let mut pos = 0;
    let declaration = parse_xml_declaration_prolog(trimmed, &mut pos)?;
    let (doctype, prolog) = parse_misc(trimmed, &mut pos, true)?;
    let root = parse_node(trimmed, &mut pos)?;
    let (_, epilog) = parse_misc(trimmed, &mut pos, false)?;
    if pos < trimmed.len() {
        return Err("trailing content after root element".into());
    }
    let document = XmlDocument { root: Some(root), doctype, declaration, prolog, epilog };
    validate_xml_document_boundaries(&document)?;
    Ok(document)
}






















//#endregion 🔖️XmlTextCodec

//#region 🔖️HandcraftedArtifactCodecs



//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🔖️Tests







#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests

#[path="🧭️position/🦀️.rs"]
mod position;

#[path="🧬️retained/🦀️.rs"]
pub mod retained;

/// 🧩️ Typed XML components for enclosing owned native documents.
pub use native_encoding::{XmlNativeEmission,emit_xml_native_document,emit_xml_native_node,emit_xml_native_snapshot_fields};
pub use native_decoding::{XmlNativeInput,read_xml_native_document,read_xml_native_node,read_xml_native_snapshot_fields};
























