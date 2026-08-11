//! 🧬️ TsvSnapshot schema — persistent fields; real IANA TSV codec lives in `⚙️engine`.
//! IANA text/tab-separated-values (https://www.iana.org/assignments/media-types/text/tab-separated-values)
//! has NO quoting/escaping mechanism — unlike csv, a field can never legally contain a literal
//! tab (0x09) or newline (0x0A/0x0D) byte; there is no way to escape one. This codec does not
//! invent one either: it is a byte-exact split/rejoin on `\t`/line-ending, matching the real W0
//! fixture's own verification method (`verify_tsv.py`) exactly. Own types — deliberately NOT
//! merged into csv's (different standard, different grammar, no shared quoting semantics).

use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Ids
pub const STDIO_TSV_DOCUMENT_SCHEMA: &str = "stdio.tsv";
//#endregion 🔖️Ids

//#region 🔖️LineEnding
/// ↩️ The file's own line-ending convention. IANA TSV doesn't mandate one; real files use either.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LineEnding {
    Lf,
    Crlf,
}

impl LineEnding {
    pub fn as_str(self) -> &'static str {
        match self {
            LineEnding::Lf => "\n",
            LineEnding::Crlf => "\r\n",
        }
    }
}

impl Default for LineEnding {
    fn default() -> Self {
        LineEnding::Lf
    }
}
//#endregion 🔖️LineEnding

//#region 🔖️Snapshot
/// 📸️ Persisted `stdio.tsv` snapshot — a raw row grid (no header/data distinction; IANA TSV
/// draws none structurally) + the two pieces of whole-file retention metadata a byte-exact
/// split/rejoin needs: whether the source ended with a line terminator, and which one it used.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.tsv")]
pub struct TsvSnapshot {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub records: Vec<Vec<String>>,
    #[state(persistent)]
    #[serde(default)]
    pub trailing_newline: bool,
    #[state(persistent)]
    #[serde(default)]
    pub line_ending: LineEnding,
}

impl Default for TsvSnapshot {
    fn default() -> Self {
        Self {
            schema: STDIO_TSV_DOCUMENT_SCHEMA.into(),
            records: Vec::new(),
            trailing_newline: false,
            line_ending: LineEnding::default(),
        }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs
// 🔗 Real byte-exact split/rejoin codec lives in `⚙️engine::decode_tsv`/`encode_tsv`
// (https://www.iana.org/assignments/media-types/text/tab-separated-values).
impl store::ArtifactDsl for TsvSnapshot {
    const EXTENSION: &'static str = "tsv";
    fn envelope_id() -> &'static str { STDIO_TSV_DOCUMENT_SCHEMA }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        Ok(crate::artifacts::tsv::standards::iana::engine::decode_tsv(body))
    }
    fn print_dsl(&self) -> String {
        let body = crate::artifacts::tsv::standards::iana::engine::encode_tsv(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for TsvSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = crate::artifacts::tsv::standards::iana::engine::encode_tsv(self).into_bytes();
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
        Ok(crate::artifacts::tsv::standards::iana::engine::decode_tsv(&text))
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs
