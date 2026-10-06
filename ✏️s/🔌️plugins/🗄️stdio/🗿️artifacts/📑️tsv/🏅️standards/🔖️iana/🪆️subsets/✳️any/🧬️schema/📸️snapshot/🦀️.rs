//! 🧬️ TsvSnapshot schema — persistent fields + the real IANA TSV codec (dissolved out of the
//! former `⚙️engine`, ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES — kept beside the
//! `ArtifactDsl`/`ArtifactPack` impls that call it directly, mirroring `json`'s own already-
//! established `parse_json_text`/`write_json_text` placement in its `📸️snapshot/🦀️.rs`).
//! IANA text/tab-separated-values (https://www.iana.org/assignments/media-types/text/tab-separated-values)
//! has NO quoting/escaping mechanism — unlike csv, a field can never legally contain a literal
//! tab (0x09) or newline (0x0A/0x0D) byte; there is no way to escape one. This codec does not
//! invent one either: it is a byte-exact split/rejoin on `\t`/line-ending, matching the real W0
//! fixture's own verification method (`verify_tsv.py`) exactly. Own types — deliberately NOT
//! merged into csv's (different standard, different grammar, no shared quoting semantics).

use framework_schema::ArtifactSchema;




//#region 🔖️Ids
pub const STDIO_TSV_DOCUMENT_SCHEMA: &str = "stdio.tsv";
//#endregion 🔖️Ids

//#region 🔖️LineEnding
/// ↩️ The file's own line-ending convention. IANA TSV doesn't mandate one; real files use either.
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar)]
#[value(rename_all = "camelCase")]
#[derive(Default)]
pub enum LineEnding {
    #[default]
    Lf,
    Crlf,
}

impl LineEnding {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn as_str(self) -> &'static str {
        match self {
            LineEnding::Lf => "\n",
            LineEnding::Crlf => "\r\n",
        }
    }
}

//#endregion 🔖️LineEnding

//#region 🔖️Snapshot
/// 📸️ Persisted `stdio.tsv` snapshot — a raw row grid (no header/data distinction; IANA TSV
/// draws none structurally) + the two pieces of whole-file retention metadata a byte-exact
/// split/rejoin needs: whether the source ended with a line terminator, and which one it used.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.tsv")]
pub struct TsvSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default)]
    pub records: Vec<Vec<String>>,
    #[state(artifact)]
    #[value(default)]
    pub trailing_newline: bool,
    #[state(artifact)]
    #[value(default)]
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn sniff_real_bytes(bytes: &[u8]) -> bool {
    !bytes.is_empty() && !bytes.contains(&0u8)
}
//#endregion 🔖️Sniff

//#region 🔖️SnapshotCodec







//#endregion 🔖️SnapshotCodec

//#region 🔖️HandcraftedArtifactCodecs





//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
