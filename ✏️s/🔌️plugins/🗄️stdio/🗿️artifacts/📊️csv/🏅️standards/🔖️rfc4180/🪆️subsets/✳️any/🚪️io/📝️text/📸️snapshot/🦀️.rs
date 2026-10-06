//! 📝️ Text representation codec surface for `stdio.csv` (snapshot).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type CsvSnapshotText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v_rfc4180::subsets::any::schema::snapshot::*;
use crate::STDIO_CSV_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;

impl store::ArtifactDsl for CsvSnapshot{
 const EXTENSION:&'static str="csv";
 fn envelope_id()->&'static str{"stdio.csv"}
 fn parse_dsl(text:&str)->Result<Self,semio_framework_diagnostic::TextError>{let(envelope,body)=store::semio_format::split_text_preamble(text).map_err(|error|semio_framework_diagnostic::TextError::from_value_error(error.into_value_error(),semio_framework_diagnostic::TextSpan::at(1,1)))?;if !envelope.matches_identity(Self::envelope_id(),store::semio_format::Component::Dsl,1){return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"CSV logical Text envelope mismatch",semio_framework_diagnostic::TextSpan::at(1,1)))}Self::__dsl_from_record(&semio_framework_dsl_record::parse_exact(body,&Self::__dsl_spec(),&Default::default())?)}
 fn print_dsl(&self)->String{let body=semio_framework_dsl_record::print(&self.__dsl_to_record(),&Self::__dsl_spec(),semio_framework_dsl_record::JoinMode::Document);let envelope=store::semio_format::SemioEnvelope::from_envelope_id(Self::envelope_id(),store::semio_format::Component::Dsl,1).expect("declared CSV logical envelope");store::semio_format::wrap_text(&envelope,&body)}
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod snapshot_wire_codec {
use super::*;
use crate::standards::v_rfc4180::subsets::any::schema::snapshot::*;
use crate::STDIO_CSV_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;

/// 📥 RFC 4180 record tokenizer over the WHOLE text (not line-by-line, so a quoted
/// field's embedded `\n`/`\r\n` is consumed as data, not a record boundary), handling
/// escaped `""` quotes, both CRLF and bare-LF line endings, and tracking per-field
/// whether the source actually wrapped it in quotes (real, losslessly-retained
/// information per RFC 4180 §2 rule 5 — quoting is optional).
pub(crate) fn parse_csv_records(text: &str) -> Vec<CsvRecord> {
    let mut records = Vec::new();
    let mut fields: Vec<CsvField> = Vec::new();
    let mut cur = String::new();
    let mut cur_quoted = false;
    let mut in_quotes = false;
    let mut chars = text.chars().peekable();
    fn take_field(cur: &mut String, cur_quoted: &mut bool) -> CsvField {
        CsvField { value: std::mem::take(cur), quoted: std::mem::take(cur_quoted) }
    }
    while let Some(ch) = chars.next() {
        if in_quotes {
            if ch == '"' {
                if chars.peek() == Some(&'"') {
                    chars.next();
                    cur.push('"');
                } else {
                    in_quotes = false;
                }
            } else {
                cur.push(ch);
            }
            continue;
        }
        match ch {
            '"' => {
                in_quotes = true;
                cur_quoted = true;
            }
            ',' => fields.push(take_field(&mut cur, &mut cur_quoted)),
            '\r' => {
                if chars.peek() == Some(&'\n') {
                    chars.next();
                }
                fields.push(take_field(&mut cur, &mut cur_quoted));
                records.push(CsvRecord { fields: std::mem::take(&mut fields) });
            }
            '\n' => {
                fields.push(take_field(&mut cur, &mut cur_quoted));
                records.push(CsvRecord { fields: std::mem::take(&mut fields) });
            }
            _ => cur.push(ch),
        }
    }
    if !cur.is_empty() || cur_quoted || !fields.is_empty() {
        fields.push(take_field(&mut cur, &mut cur_quoted));
        records.push(CsvRecord { fields });
    }
    records
}

/// 📤 Quotes a field when the source quoted it OR when RFC 4180 §2 rule 6 REQUIRES
/// quoting (the value itself contains a comma, quote, or line break).
pub(crate) fn escape_field(field: &CsvField) -> String {
    let needs_quote = field.quoted || field.value.contains(',') || field.value.contains('"') || field.value.contains('\n') || field.value.contains('\r');
    if needs_quote {
        format!("\"{}\"", field.value.replace('"', "\"\""))
    } else {
        field.value.clone()
    }
}

pub(crate) fn write_csv_records(records: &[CsvRecord], line_ending: &str) -> String {
    let mut out = String::new();
    for record in records {
        let csv_field_separator = ",";
        out.push_str(&record.fields.iter().map(escape_field).collect::<Vec<_>>().join(csv_field_separator));
        out.push_str(line_ending);
    }
    out
}

/// 📥 Decodes RFC 4180 text into a snapshot. `has_header` is pure metadata about whether
/// `records[0]` should be read as a header row — RFC 4180 draws no structural distinction
/// between a header record and a data record on the wire, so decoding never drops or
/// relocates the first record.
pub fn decode_csv_with(text: &str, has_header: bool) -> CsvSnapshot {
    let records = parse_csv_records(text);
    CsvSnapshot { schema: STDIO_CSV_DOCUMENT_SCHEMA.into(), has_header, records }
}

/// 📥 Decodes assuming a header row is present (the pre-existing default behavior).
pub fn decode_csv(text: &str) -> Result<CsvSnapshot, String> {
    Ok(decode_csv_with(text, true))
}

/// 📤 Encodes with LF line endings.
pub fn encode_csv(snap: &CsvSnapshot) -> String {
    encode_csv_with(snap, "\n")
}

/// 📤 Encodes with a caller-chosen line ending (`"\n"` or `"\r\n"`).
pub fn encode_csv_with(snap: &CsvSnapshot, line_ending: &str) -> String {
    if snap.records.is_empty() {
        return String::new();
    }
    write_csv_records(&snap.records, line_ending)
}
}
pub use snapshot_wire_codec::*;

#[allow(unused_imports)]
mod snapshot_wire2_codec {
use super::*;
use crate::standards::v_rfc4180::subsets::any::schema::snapshot::*;
use crate::STDIO_CSV_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;

/// 📥️ Reads authored CSV files or the declared logical Text document.
pub fn read_csv_source_text(text:&str)->Result<CsvSnapshot,semio_framework_diagnostic::TextError>{if text.starts_with("semio "){<CsvSnapshot as store::ArtifactDsl>::parse_dsl(text)}else{Ok(decode_csv_with(text,true))}}
}
pub use snapshot_wire2_codec::*;

#[allow(unused_imports)]
mod snapshot_wire3_codec {
use super::*;
use crate::standards::v_rfc4180::subsets::any::schema::snapshot::*;
use crate::STDIO_CSV_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;

/// 🌱 Empty persisted snapshot.
pub fn empty_csv_snapshot() -> CsvSnapshot {
    CsvSnapshot::default()
}

/// 📄️ The `demo` example, parsed once from `examples::demo::PRIMARY_TEXT` — the single source
/// of truth `🗣️.dsl.semio` is genuinely `print_dsl` of (P2-P1 `fixture_honesty_law`),
/// same pattern as `note::semio_example_snapshot`.
pub fn demo_csv_snapshot() -> CsvSnapshot {
    <CsvSnapshot as store::ArtifactDsl>::parse_dsl(crate::examples::demo::PRIMARY_TEXT).unwrap_or_else(|_| empty_csv_snapshot())
}
}
pub use snapshot_wire3_codec::*;
