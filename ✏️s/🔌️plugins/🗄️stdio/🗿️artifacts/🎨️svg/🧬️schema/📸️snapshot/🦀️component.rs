//! 🧬️ SvgSnapshot schema — persistent fields + real codecs.

use crate::artifacts::svg::STDIO_SVG_DOCUMENT_SCHEMA;
use crate::artifacts::xml::schema::snapshot::{xml_document_from_text, xml_document_to_text, XmlDocument, XmlNode};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.svg")]
pub struct SvgSnapshot {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub doc: XmlDocument,
}

impl Default for SvgSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_SVG_DOCUMENT_SCHEMA.into(), doc: XmlDocument::default() }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️SvgCodec
pub fn parse_svg_xml(text: &str) -> Result<XmlDocument, String> {
    let doc = xml_document_from_text(text)?;
    if let Some(XmlNode::Element { name, .. }) = &doc.root {
        if name != "svg" && !name.ends_with(":svg") {
            return Err("root element must be svg".into());
        }
    } else {
        return Err("svg document requires root element".into());
    }
    Ok(doc)
}

pub fn write_svg_xml(doc: &XmlDocument) -> String {
    xml_document_to_text(doc)
}
//#endregion 🔖️SvgCodec

//#region 🔖️HandcraftedDocumentCodecs
impl store::DocumentDsl for SvgSnapshot {
    const EXTENSION: &'static str = "svg";
    fn envelope_id() -> &'static str { "stdio.svg" }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let doc = parse_svg_xml(body).map_err(|e| store::TextError::new(format!("svg parse: {e}"), dsl::TextSpan::at(1, 1)))?;
        Ok(Self { schema: STDIO_SVG_DOCUMENT_SCHEMA.into(), doc })
    }
    fn print_dsl(&self) -> String {
        let body = write_svg_xml(&self.doc);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::DocumentPack for SvgSnapshot {
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
        Ok(Self { schema: STDIO_SVG_DOCUMENT_SCHEMA.into(), doc })
    }
}
//#endregion 🔖️HandcraftedDocumentCodecs
