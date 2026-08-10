//! 🧬️ XmlSnapshot schema — persistent fields + real codecs.

use crate::artifacts::xml::STDIO_XML_DOCUMENT_SCHEMA;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

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
    PLACEHOLDER_PUB_VALUE PLACEHOLDER_VALUE_TYPE,
}

impl Default for XmlSnapshot {
    fn default() -> Self {
        Self {
            schema: STDIO_XML_DOCUMENT_SCHEMA.into(),
            PLACEHOLDER_VALUE_COLON PLACEHOLDER_VALUE_TYPE::Null,
        }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedDocumentCodecs
impl store::DocumentDsl for XmlSnapshot {
    const EXTENSION: &'static str = "xml";
    fn envelope_id() -> &'static str { "stdio.xml" }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let value = serde_xml::from_str(body.trim()).map_err(|e| {
            store::TextError::new(format!("xml parse: {e}"), dsl::TextSpan::at(1, 1))
        })?;
        Ok(Self { schema: STDIO_XML_DOCUMENT_SCHEMA.into(), value })
    }
    fn print_dsl(&self) -> String {
        let body = serde_xml::to_string_pretty(&PLACEHOLDER_SELF_VALUE).unwrap_or_else(|_| "null".into());
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

        let raw = serde_xml::to_vec(&PLACEHOLDER_SELF_VALUE).map_err(|e| store::PackError::Schema(e.to_string()))?;
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
        let value = serde_xml::from_slice(&inner).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(Self { schema: STDIO_XML_DOCUMENT_SCHEMA.into(), value })
    }
}
//#endregion 🔖️HandcraftedDocumentCodecs
