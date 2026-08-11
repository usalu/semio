//! 🧬️ CsvSnapshot schema — persistent fields; real RFC4180 codec lives in `⚙️engine`.

use crate::artifacts::csv::STDIO_CSV_DOCUMENT_SCHEMA;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

fn default_true() -> bool { true }

//#region 🔖️Field
/// 🔤 One RFC 4180 field value plus whether the source quoted it — rfc4180's own optional
/// quoting means whether a field WAS quoted is real information worth preserving losslessly,
/// so re-serializing can reproduce the exact source bytes rather than a lossy normal form
/// (https://www.rfc-editor.org/rfc/rfc4180#section-2, rule 5).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CsvField {
    #[serde(default)]
    pub value: String,
    #[serde(default)]
    pub quoted: bool,
}
//#endregion 🔖️Field

//#region 🔖️Record
/// 📄 One RFC 4180 record (row) — a strong-like entity, index-keyed within
/// `CsvSnapshot::records`. Field COUNT is real, per-record information (rfc4180 is a
/// loosely-typed grid on the wire even though most producers keep it rectangular).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CsvRecord {
    #[serde(default)]
    pub fields: Vec<CsvField>,
}
//#endregion 🔖️Record

//#region 🔖️Snapshot
/// 📸️ Persisted `stdio.csv` snapshot (RFC 4180 table, with a header-row option). The
/// header row (when present) is `records[0]` — RFC 4180 draws no structural distinction
/// between a header record and a data record, only a convention of which one comes first.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.csv")]
pub struct CsvSnapshot {
    #[state(persistent)]
    pub schema: String,
    /// 📑 Whether the first record is a header row (RFC 4180's own optional convention).
    #[state(persistent)]
    #[serde(default = "default_true")]
    pub has_header: bool,
    #[state(persistent)]
    #[serde(default)]
    pub records: Vec<CsvRecord>,
}

impl Default for CsvSnapshot {
    fn default() -> Self {
        Self {
            schema: STDIO_CSV_DOCUMENT_SCHEMA.into(),
            has_header: true,
            records: Vec::new(),
        }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs
// 🔗 Real tokenizer lives in `⚙️engine::decode_csv_with`/`encode_csv_with`
// (https://www.rfc-editor.org/rfc/rfc4180).
impl store::ArtifactDsl for CsvSnapshot {
    const EXTENSION: &'static str = "csv";
    fn envelope_id() -> &'static str { "stdio.csv" }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        Ok(crate::artifacts::csv::engine::decode_csv_with(body, true))
    }
    fn print_dsl(&self) -> String {
        let body = crate::artifacts::csv::engine::encode_csv(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for CsvSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = crate::artifacts::csv::engine::encode_csv(self).into_bytes();
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
        let text = String::from_utf8(inner).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(crate::artifacts::csv::engine::decode_csv_with(&text, true))
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs
