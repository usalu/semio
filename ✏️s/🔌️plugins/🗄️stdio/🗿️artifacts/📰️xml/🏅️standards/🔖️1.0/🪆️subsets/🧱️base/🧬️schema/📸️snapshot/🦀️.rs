//! 🧬️ XmlSnapshot logical state and validation.

use crate::STDIO_XML_DOCUMENT_SCHEMA;
#[path = "💡️markup-facts/🦀️.rs"]
pub mod markup_facts;
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


}
//#endregion 🔖️Snapshot

/// 🛡️ Validates the authored XML declaration for UTF-8 publication.
pub fn validate_xml_declaration_boundary(declaration:Option<&XmlDeclaration>)->Result<(),String>{
    if let Some(declaration) = declaration {
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
    Ok(())
}

/// 🛡️ Rejects declaration or boundary state that cannot be published by the UTF-8 XML codec.
pub fn validate_xml_document_boundaries(doc: &XmlDocument) -> Result<(), String> {
    validate_xml_declaration_boundary(doc.declaration.as_ref())?;
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



//#region 🔖️HandcraftedArtifactCodecs



//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🔖️Tests







#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests

#[path="🧭️position/🦀️.rs"]
pub(crate) mod position;

#[path="🧬️retained/🦀️.rs"]
pub mod retained;


























#[path="🧺️ownership/🦀️.rs"]
pub mod ownership;
