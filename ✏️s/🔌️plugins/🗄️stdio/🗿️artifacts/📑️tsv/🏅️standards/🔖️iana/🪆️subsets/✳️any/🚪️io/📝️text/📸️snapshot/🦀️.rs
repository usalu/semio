//! 📝️ Text representation codec surface for `stdio.tsv` (snapshot).

pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type TsvSnapshotText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::iana::subsets::any::schema::snapshot::*;
use framework_schema::ArtifactSchema;

impl store::ArtifactDsl for TsvSnapshot{
 const EXTENSION:&'static str="tsv";
 fn envelope_id()->&'static str{"stdio.tsv"}
 fn parse_dsl(text:&str)->Result<Self,semio_framework_diagnostic::TextError>{let(envelope,body)=store::semio_format::split_text_preamble(text).map_err(|error|semio_framework_diagnostic::TextError::from_value_error(error.into_value_error(),semio_framework_diagnostic::TextSpan::at(1,1)))?;if !envelope.matches_identity(Self::envelope_id(),store::semio_format::Component::Dsl,1){return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"TSV logical Text envelope mismatch",semio_framework_diagnostic::TextSpan::at(1,1)))}Self::__dsl_from_record(&semio_framework_dsl_record::parse_exact(body,&Self::__dsl_spec(),&Default::default())?)}
 fn print_dsl(&self)->String{let body=semio_framework_dsl_record::print(&self.__dsl_to_record(),&Self::__dsl_spec(),semio_framework_dsl_record::JoinMode::Document);let envelope=store::semio_format::SemioEnvelope::from_envelope_id(Self::envelope_id(),store::semio_format::Component::Dsl,1).expect("declared TSV logical envelope");store::semio_format::wrap_text(&envelope,&body)}
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod snapshot_wire_codec {
use super::*;
use crate::standards::iana::subsets::any::schema::snapshot::*;
use framework_schema::ArtifactSchema;

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
}
pub use snapshot_wire_codec::*;

#[allow(unused_imports)]
mod snapshot_wire2_codec {
use super::*;
use crate::standards::iana::subsets::any::schema::snapshot::*;
use framework_schema::ArtifactSchema;

/// 📥️ Reads authored TSV files or the declared logical Text document.
pub fn read_tsv_source_text(text:&str)->Result<TsvSnapshot,semio_framework_diagnostic::TextError>{if text.starts_with("semio "){<TsvSnapshot as store::ArtifactDsl>::parse_dsl(text)}else{Ok(decode_tsv(text))}}
}
pub use snapshot_wire2_codec::*;
