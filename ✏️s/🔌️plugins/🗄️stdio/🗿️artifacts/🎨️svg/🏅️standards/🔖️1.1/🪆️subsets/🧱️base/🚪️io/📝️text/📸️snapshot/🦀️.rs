//! 📝️ Text representation codec surface for `stdio.svg` (snapshot).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type SvgSnapshotText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1_1::subsets::base::schema::snapshot::*;
use crate::STDIO_SVG_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;
use crate::schema::snapshot::{SvgAttr, SvgDocument, SvgNode};
use semio_s_artifact_stdio_xml::standards::v1_0::subsets::base::io::text::snapshot::{xml_document_from_text, xml_document_to_text_checked};

impl store::ArtifactDsl for SvgSnapshot {
    const EXTENSION: &'static str = "svg";
    fn envelope_id() -> &'static str {
        "stdio.svg"
    }

    /// 📥️ Two real inputs, told apart by the envelope: text that CARRIES a semio preamble is this
    /// artifact's own snapshot DSL (`encode_snapshot`'s structured body), and text that does not is
    /// the document's own `.svg` markup, which [`SvgSnapshot::import_utf8`] parses losslessly. Same
    /// two-branch shape the sibling `📰️xml` artifact's `parse_dsl` uses, and for the same reason:
    /// `from_text` (this subset's `DerivedConstruction`, and `📚️examples`' own raw markup) hands raw
    /// SVG straight in, and refusing it made every such caller fail on the preamble check alone.
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        match store::semio_format::split_text_preamble(text) {
            Ok((_, body)) => crate::standards::v1_1::subsets::base::io::text::snapshot::decode_snapshot(body.trim()).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("svg state parse: {e}"), semio_framework_diagnostic::TextSpan::at(1, 1))),
            Err(_) => Self::import_utf8(text.as_bytes()).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("svg parse: {e}"), semio_framework_diagnostic::TextSpan::at(1, 1))),
        }
    }
    fn print_dsl(&self) -> String {
        let body = crate::standards::v1_1::subsets::base::io::text::snapshot::encode_snapshot(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod snapshot_wire_codec {
use super::*;
use crate::standards::v1_1::subsets::base::schema::snapshot::*;
use crate::STDIO_SVG_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;
use crate::schema::snapshot::{SvgAttr, SvgDocument, SvgNode};
use semio_s_artifact_stdio_xml::standards::v1_0::subsets::base::io::text::snapshot::{xml_document_from_text, xml_document_to_text_checked};

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn parse_svg_xml(text: &str) -> Result<SvgDocument, String> {
    let doc = crate::standards::v1_1::subsets::base::io::text::snapshot::attributes::bind_svg_document(xml_document_from_text(text)?)?;
    if let Some(SvgNode::Element { name, .. }) = &doc.root {
        if name != "svg" && !name.ends_with(":svg") {
            return Err("root element must be svg".into());
        }
    } else {
        return Err("svg document requires root element".into());
    }
    Ok(doc)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn write_svg_xml(doc: &SvgDocument) -> Result<String, String> {
    doc.validate_attribute_owners()?;
    {let mut native=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(crate::standards::v1_1::subsets::base::io::text::snapshot::attributes::native_svg_document(doc),semio_s_artifact_stdio_xml::schema::snapshot::ownership::retire_xml_document);xml_document_to_text_checked(native.as_mut())}
}


}
pub use snapshot_wire_codec::*;

#[allow(unused_imports)]
mod snapshot_wire2_codec {
use super::*;
use crate::standards::v1_1::subsets::base::schema::snapshot::*;
use crate::STDIO_SVG_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;
use crate::schema::snapshot::{SvgAttr, SvgDocument, SvgNode};
use semio_s_artifact_stdio_xml::standards::v1_0::subsets::base::io::text::snapshot::{xml_document_from_text, xml_document_to_text_checked};


}
pub use snapshot_wire2_codec::*;

#[allow(unused_imports)]
mod snapshot_wire3_codec {
use crate::standards::v1_1::subsets::base::schema::mutation_support::*;
use crate::schema::diff::{diff_at_path, SvgAttrAdded, SvgAttrModified, SvgAttributesDiff, SvgDiff, SvgElementDiff, SvgNodeDiff};
use crate::schema::snapshot::node_at;
use crate::SvgSnapshot;
use crate::schema::snapshot::{SvgDocument, SvgNode};

pub(crate) fn encode_snapshot(snapshot: &SvgSnapshot) -> String {
    use crate::standards::v1_1::subsets::base::io::text::diff::enc_svg_node;
    use crate::standards::v1_1::subsets::base::io::text::diff::{enc_doctype};
    use crate::standards::v1_1::subsets::base::io::text::diff::{enc_declaration};
    use crate::standards::v1_1::subsets::base::io::text::diff::{enc_prolog};
    use crate::standards::v1_1::subsets::base::io::text::diff::{encode_option};
    use crate::standards::v1_1::subsets::base::io::text::diff::{enc_str};
    format!(
        "[{},{},{},{},{},{}]",
        enc_str(&snapshot.schema),
        encode_option(&snapshot.doc.root, enc_svg_node),
        encode_option(&snapshot.doc.doctype, enc_doctype),
        encode_option(&snapshot.doc.declaration, enc_declaration),
        enc_prolog(&snapshot.doc.prolog),
        enc_prolog(&snapshot.doc.epilog)
    )
}

pub(crate) fn decode_snapshot(value: &str) -> Result<SvgSnapshot, String> {
    use crate::standards::v1_1::subsets::base::io::text::diff::{dec_svg_node};
    use crate::standards::v1_1::subsets::base::io::text::diff::{dec_doctype};
    use crate::standards::v1_1::subsets::base::io::text::diff::{dec_declaration};
    use crate::standards::v1_1::subsets::base::io::text::diff::{dec_prolog};
    use crate::standards::v1_1::subsets::base::io::text::diff::{decode_option};
    use crate::standards::v1_1::subsets::base::io::text::diff::{strip_brackets};
    use crate::standards::v1_1::subsets::base::io::text::diff::{split_top_level};
    use crate::standards::v1_1::subsets::base::io::text::diff::{dec_str};
    let parts = split_top_level(strip_brackets(value)?, ',');
    let [schema, root, doctype, declaration, prolog, epilog] = parts.as_slice() else {
        return Err(format!("svg snapshot: expected 6 fields, got {}", parts.len()));
    };
    let snapshot = SvgSnapshot {
        schema: dec_str(schema)?,
        doc: SvgDocument { root: decode_option(root, dec_svg_node)?, doctype: decode_option(doctype, dec_doctype)?, declaration: decode_option(declaration, dec_declaration)?, prolog: dec_prolog(prolog)?, epilog: dec_prolog(epilog)? },
    };
    Ok(snapshot)
}
}
pub use snapshot_wire3_codec::*;

impl crate::SvgSnapshot {
    pub fn import_utf8(bytes: &[u8]) -> Result<Self, String> {
        let text = std::str::from_utf8(bytes).map_err(|error| format!("svg source is not UTF-8: {error}"))?;
        Ok(Self { schema: crate::STDIO_SVG_DOCUMENT_SCHEMA.into(), doc: crate::standards::v1_1::subsets::base::io::text::snapshot::parse_svg_xml(text)? })
    }
}

impl crate::SvgSnapshot {
    pub fn export_utf8(&self) -> Result<Vec<u8>, String> {
        self.validate_natural()?;
        Ok(write_svg_xml(&self.doc)?.into_bytes())
    }
}

#[path="🧮️attributes/🦀️.rs"]
pub mod attributes;
pub use attributes::*;

#[cfg(test)]
#[path="🧪️tests/🔬️component/🦀️.rs"]
mod component_tests;
