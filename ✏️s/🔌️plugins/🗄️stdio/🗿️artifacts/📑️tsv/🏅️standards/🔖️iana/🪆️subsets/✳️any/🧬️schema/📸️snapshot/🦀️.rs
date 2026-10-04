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

#[path = "🪶️sqlite/🦀️.rs"]
mod sqlite;
#[path = "🚦️native/🦀️.rs"]
mod sqlite_native;
#[cfg(test)]
#[path = "🧪️tests/🪶️sqlite/🦀️.rs"]
mod sqlite_tests;

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
/// 📥️ Decodes TSV text via a byte-exact split on the file's own line ending, then `\t` per line
/// — no quoting, no escaping, no coercion (matches the real W0 fixture's own `verify_tsv.py`
/// verification method exactly: split on `\n`, then each line on `\t`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_tsv(text: &str) -> TsvSnapshot {
    let line_ending = if text.contains("\r\n") { LineEnding::Crlf } else { LineEnding::Lf };
    let sep = line_ending.as_str();
    let trailing_newline = text.ends_with(sep);
    let body = if trailing_newline { &text[..text.len() - sep.len()] } else { text };
    let records: Vec<Vec<String>> = if body.is_empty() { Vec::new() } else { body.split(sep).map(|line| line.split('\t').map(|s| s.to_string()).collect()).collect() };
    TsvSnapshot { schema: STDIO_TSV_DOCUMENT_SCHEMA.into(), records, trailing_newline, line_ending }
}

/// 📤️ Encodes via a byte-exact rejoin: `\t` within a row, the snapshot's own `line_ending`
/// between rows, plus a final terminator iff `trailing_newline` is set.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_tsv(snap: &TsvSnapshot) -> String {
    let sep = snap.line_ending.as_str();
    let mut out = snap.records.iter().map(|r| r.join("\t")).collect::<Vec<_>>().join(sep);
    if snap.trailing_newline {
        out.push_str(sep);
    }
    out
}

/// 📄️ The persisted document (`semio iana.tsv.dsl v1` envelope + body) through the artifact's own `ArtifactDsl` codec,
/// reachable for a caller that cannot name the trait.
pub fn parse_tsv_document(text: &str) -> Result<TsvSnapshot, String> {
    <TsvSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| error.to_string())
}

/// 📄️ The inverse of [`parse_tsv_document`]: the snapshot printed as its enveloped document.
pub fn print_tsv_document(snapshot: &TsvSnapshot) -> String {
    <TsvSnapshot as store::ArtifactDsl>::print_dsl(snapshot)
}
//#endregion 🔖️SnapshotCodec

//#region 🔖️HandcraftedArtifactCodecs
/// 📥️ Reads authored TSV files or the declared logical Text document.
pub fn read_tsv_source_text(text:&str)->Result<TsvSnapshot,semio_framework_diagnostic::TextError>{if text.starts_with("semio "){<TsvSnapshot as store::ArtifactDsl>::parse_dsl(text)}else{Ok(decode_tsv(text))}}
/// 📥️ Reads the logical Pack document or authored UTF-8 TSV bytes.
pub fn read_tsv_source_binary(bytes:&[u8])->Result<TsvSnapshot,store::PackError>{if bytes.starts_with(&[137,83,69,77,13,10,26,10]){<TsvSnapshot as store::ArtifactPack>::decode_pack(bytes)}else{let text=std::str::from_utf8(bytes).map_err(|error|store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,error.to_string())))?;read_tsv_source_text(text).map_err(store::PackError::from)}}

impl store::ArtifactDsl for TsvSnapshot{
 const EXTENSION:&'static str="tsv";
 fn envelope_id()->&'static str{"stdio.tsv"}
 fn parse_dsl(text:&str)->Result<Self,semio_framework_diagnostic::TextError>{let(envelope,body)=store::semio_format::split_text_preamble(text).map_err(|error|semio_framework_diagnostic::TextError::from_value_error(error.into_value_error(),semio_framework_diagnostic::TextSpan::at(1,1)))?;if !envelope.matches_identity(Self::envelope_id(),store::semio_format::Component::Dsl,1){return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"TSV logical Text envelope mismatch",semio_framework_diagnostic::TextSpan::at(1,1)))}Self::__dsl_from_record(&semio_framework_dsl_record::parse_exact(body,&Self::__dsl_spec(),&Default::default())?)}
 fn print_dsl(&self)->String{let body=semio_framework_dsl_record::print(&self.__dsl_to_record(),&Self::__dsl_spec(),semio_framework_dsl_record::JoinMode::Document);let envelope=store::semio_format::SemioEnvelope::from_envelope_id(Self::envelope_id(),store::semio_format::Component::Dsl,1).expect("declared TSV logical envelope");store::semio_format::wrap_text(&envelope,&body)}
}
impl store::ArtifactPack for TsvSnapshot{
 fn record_spec()->Option<semio_framework_dsl_record::RecordSpec>{Some(Self::__dsl_spec())}
 fn sqlite_snapshot_codec()->Option<store::ArtifactSqliteSnapshotCodec>{Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec())}
 fn encode_pack_with(&self,options:&store::PackEncodeOptions)->Result<Vec<u8>,store::PackError>{let body=store::pack_rt::encode_document(&Self::__dsl_spec(),&self.__dsl_to_record(),options)?;let envelope=store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(),store::semio_format::Component::Pack,1).map_err(|error|store::PackError::from(error.into_value_error()))?;Ok(store::semio_format::wrap_binary(&envelope,&body))}
 fn decode_pack_with(bytes:&[u8],options:&store::PackDecodeOptions)->Result<Self,store::PackError>{let(envelope,body)=store::semio_format::unwrap_binary(bytes).map_err(|error|store::PackError::from(error.into_value_error()))?;if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(),store::semio_format::Component::Pack,1){return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"TSV logical Pack envelope mismatch")))}Self::__dsl_from_record(&store::pack_rt::decode_document(&body,&Self::__dsl_spec(),options)?.0).map_err(store::PackError::from)}
}
//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
