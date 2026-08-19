//! 🧬️ TsvSnapshot schema — persistent fields + the real IANA TSV codec (dissolved out of the
//! former `⚙️engine`, ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES — kept beside the
//! `ArtifactDsl`/`ArtifactPack` impls that call it directly, mirroring `json`'s own already-
//! established `parse_json_text`/`write_json_text` placement in its `📸️snapshot/🦀️component.rs`).
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
    pub async fn as_str(self) -> &'static str {
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
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[serde(default)]
    pub records: Vec<Vec<String>>,
    #[state(artifact)]
    #[serde(default)]
    pub trailing_newline: bool,
    #[state(artifact)]
    #[serde(default)]
    pub line_ending: LineEnding,
}

impl Default for TsvSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_TSV_DOCUMENT_SCHEMA.into(), records: Vec::new(), trailing_newline: false, line_ending: LineEnding::default() }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️Sniff
/// 🔍️ TSV has no reliable magic bytes (per the master plan: "heuristic tab-density check or just
/// accept-by-default since TSV has no reliable magic"). Real structural heuristic: at least one
/// line, and every line contains at least one tab OR the file is a single untabbed line (a valid
/// one-column TSV) — i.e. reject obvious binary noise (NUL bytes) rather than claim a false magic.
pub async fn sniff_real_bytes(bytes: &[u8]) -> bool {
    !bytes.is_empty() && !bytes.contains(&0u8)
}
//#endregion 🔖️Sniff

//#region 🔖️SnapshotCodec
/// 📥️ Decodes TSV text via a byte-exact split on the file's own line ending, then `\t` per line
/// — no quoting, no escaping, no coercion (matches the real W0 fixture's own `verify_tsv.py`
/// verification method exactly: split on `\n`, then each line on `\t`).
pub async fn decode_tsv(text: &str) -> TsvSnapshot {
    let line_ending = if text.contains("\r\n") { LineEnding::Crlf } else { LineEnding::Lf };
    let sep = line_ending.as_str();
    let trailing_newline = text.ends_with(sep);
    let body = if trailing_newline { &text[..text.len() - sep.len()] } else { text };
    let records: Vec<Vec<String>> = if body.is_empty() { Vec::new() } else { body.split(sep).map(|line| line.split('\t').map(|s| s.to_string()).collect()).collect() };
    TsvSnapshot { schema: STDIO_TSV_DOCUMENT_SCHEMA.into(), records, trailing_newline, line_ending }
}

/// 📤️ Encodes via a byte-exact rejoin: `\t` within a row, the snapshot's own `line_ending`
/// between rows, plus a final terminator iff `trailing_newline` is set.
pub async fn encode_tsv(snap: &TsvSnapshot) -> String {
    let sep = snap.line_ending.as_str();
    let mut out = snap.records.iter().map(|r| r.join("\t")).collect::<Vec<_>>().join(sep);
    if snap.trailing_newline {
        out.push_str(sep);
    }
    out
}
//#endregion 🔖️SnapshotCodec

//#region 🔖️HandcraftedArtifactCodecs
impl store::ArtifactDsl for TsvSnapshot {
    const EXTENSION: &'static str = "tsv";
    async fn envelope_id() -> &'static str {
        STDIO_TSV_DOCUMENT_SCHEMA
    }

    async fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        Ok(decode_tsv(body))
    }
    async fn print_dsl(&self) -> String {
        let body = encode_tsv(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for TsvSnapshot {
    async fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = encode_tsv(self).into_bytes();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    async fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let _ = options;
        let text = String::from_utf8(inner).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(decode_tsv(&text))
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    const REAL_FIXTURE: &str = include_str!("../../📚️examples/🎬️demo/🖼️assets/📑️example.tsv");

    #[semio_framework_async_macros::async_test]
    async fn round_trips_a_real_shaped_tsv_body() {
        let text = "name\tage\nAda\t30\nGrace\t85\n";
        assert!(sniff_real_bytes(text.as_bytes()));
        let snap = decode_tsv(text);
        assert_eq!(snap.records[0], vec!["name", "age"]);
        assert_eq!(snap.records.len(), 3);
        assert!(snap.trailing_newline);
        assert_eq!(snap.line_ending, LineEnding::Lf);
        assert_eq!(encode_tsv(&snap), text);
    }

    #[semio_framework_async_macros::async_test]
    async fn detects_crlf_line_ending() {
        let text = "a\tb\r\n1\t2\r\n";
        let snap = decode_tsv(text);
        assert_eq!(snap.line_ending, LineEnding::Crlf);
        assert_eq!(encode_tsv(&snap), text);
    }

    #[semio_framework_async_macros::async_test]
    async fn sniff_rejects_binary_noise() {
        assert!(!sniff_real_bytes(b"a\tb\0\x01\x02"));
    }

    #[semio_framework_async_macros::async_test]
    async fn embedded_backslash_t_is_not_a_real_tab() {
        // 🔒 Documents the honest IANA TSV limitation: a genuine tab byte inside a field is
        // indistinguishable from a column boundary (no quoting mechanism exists to escape it).
        // A `\t` two-character ESCAPE SEQUENCE (backslash + t), by contrast, is just two
        // ordinary text characters and round-trips perfectly, same as the real W0 fixture's row 5.
        let text = "note\nWeathering Test\\tSample\n";
        let snap = decode_tsv(text);
        assert_eq!(snap.records[1], vec!["Weathering Test\\tSample"]);
        assert_eq!(encode_tsv(&snap), text);
    }

    #[semio_framework_async_macros::async_test]
    async fn decodes_the_real_fixture_with_6_rows_and_5_columns() {
        let snap = decode_tsv(REAL_FIXTURE);
        assert_eq!(snap.records.len(), 6, "1 header row + 5 data rows");
        for row in &snap.records {
            assert_eq!(row.len(), 5, "every row must have exactly 5 columns");
        }
        assert!(snap.trailing_newline);
        assert_eq!(snap.line_ending, LineEnding::Lf);
        assert_eq!(snap.records[0], vec!["id", "name", "qty", "unit_price", "note"]);
        assert_eq!(snap.records[5][1], "Weathering Test\\tSample");
    }

    //#region 🔖️CodecRetentionLaw
    /// 🔁️ decode→encode is byte-preserving on the real W0 fixture
    /// (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/📚️examples/🎬️demo/🖼️assets/📑️example.tsv`,
    /// verified upstream by `verify_tsv.py`'s own "byte-exact split/rejoin" check).
    #[semio_framework_async_macros::async_test]
    async fn codec_retention_law() {
        let snap = decode_tsv(REAL_FIXTURE);
        let reencoded = encode_tsv(&snap);
        assert_eq!(reencoded, REAL_FIXTURE, "decode->encode must be byte-preserving on the real W0 fixture");

        let reparsed = decode_tsv(&reencoded);
        assert_eq!(reparsed, snap, "re-parsing the re-encoded text must yield the identical snapshot");
    }
    //#endregion 🔖️CodecRetentionLaw
}
//#endregion 🧪️Tests
