//! 🧬️ TxtSnapshot schema — persistent fields + real codecs.

use crate::artifacts::txt::STDIO_TXT_DOCUMENT_SCHEMA;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️LineEnding
/// ⏎️ Which newline sequence terminates each line of a `stdio.txt` document.
///
/// 🧪️ F6: `dsl::DslScalar` — a plain unit-variant-only enum binds as `DslField` directly (§3a of
/// `f6-recon-report.md`: `DslScalar` is one of the two derive sources for `DslField`, unit
/// variants only), letting `Option<LineEnding>`/`LineEnding` fields embed in `TxtDiff`/
/// `TxtSnapshot`/`TxtMutation` below without hand-rolling.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum LineEnding {
    #[default]
    Lf,
    CrLf,
}

impl LineEnding {
    /// 🔤️ The literal byte sequence this line ending prints as.
    pub fn as_str(self) -> &'static str {
        match self {
            LineEnding::Lf => "\n",
            LineEnding::CrLf => "\r\n",
        }
    }
}
//#endregion 🔖️LineEnding

//#region 🔖️Snapshot
/// 📸️ Persisted `stdio.txt` snapshot — complete per the "a text file is a sequence of lines"
/// spec: line content, whether the last line is newline-terminated, and which newline style
/// the document uses (spec covers exactly `Lf`/`CrLf`, never a mixed per-line style).
///
/// 🧪️ F6: `dsl::DslRecord` added alongside the existing hand-rolled `store::ArtifactDsl`/
/// `store::ArtifactPack` below — NOT a replacement. `DslRecord` only gives this type `DslField`
/// (so it can be embedded as a variant payload, e.g. `TxtMutation::SetSnapshot{snapshot}`), it
/// does not touch the artifact's own honest line-joined-by-line-ending envelope format.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.txt")]
pub struct TxtSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[serde(default)]
    pub lines: Vec<String>,
    #[state(artifact)]
    #[serde(default)]
    pub trailing_newline: bool,
    #[state(artifact)]
    #[serde(default)]
    pub line_ending: LineEnding,
}

impl Default for TxtSnapshot {
    fn default() -> Self {
        Self {
            schema: STDIO_TXT_DOCUMENT_SCHEMA.into(),
            lines: Vec::new(),
            trailing_newline: false,
            line_ending: LineEnding::Lf,
        }
    }
}

impl TxtSnapshot {
    /// 🧵️ Reconstructs the full text body (lines joined by `line_ending`, with the trailing
    /// newline appended iff `trailing_newline`). Inverse of [`Self::from_body`].
    pub fn to_body(&self) -> String {
        let sep = self.line_ending.as_str();
        let mut out = self.lines.join(sep);
        if self.trailing_newline {
            out.push_str(sep);
        }
        out
    }

    /// 🔍️ Splits a raw text body into `(lines, trailing_newline, line_ending)`. An empty body
    /// is zero lines (not one empty line) — the only case that needs special-casing, since
    /// `"".split(sep)` would otherwise yield a single empty-string element.
    pub fn from_body(body: &str) -> Self {
        if body.is_empty() {
            return Self { schema: STDIO_TXT_DOCUMENT_SCHEMA.into(), lines: Vec::new(), trailing_newline: false, line_ending: LineEnding::Lf };
        }
        let line_ending = if body.contains("\r\n") { LineEnding::CrLf } else { LineEnding::Lf };
        let sep = line_ending.as_str();
        let trailing_newline = body.ends_with(sep);
        let content = if trailing_newline { &body[..body.len() - sep.len()] } else { body };
        let lines: Vec<String> = content.split(sep).map(String::from).collect();
        Self { schema: STDIO_TXT_DOCUMENT_SCHEMA.into(), lines, trailing_newline, line_ending }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs
impl store::ArtifactDsl for TxtSnapshot {
    const EXTENSION: &'static str = "txt";
    fn envelope_id() -> &'static str { "stdio.txt" }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest.to_string(),
            Err(_) => text.to_string(),
        };
        Ok(Self::from_body(&body))
    }
    fn print_dsl(&self) -> String {
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &self.to_body())
    }
}

impl store::ArtifactPack for TxtSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;

        let raw = self.to_body();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        ).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, raw.as_bytes()))
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
        let body = String::from_utf8(inner).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(Self::from_body(&body))
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs
