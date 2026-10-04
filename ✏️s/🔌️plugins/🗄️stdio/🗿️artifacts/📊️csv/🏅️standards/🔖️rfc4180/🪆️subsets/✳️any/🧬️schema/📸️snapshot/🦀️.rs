//! 🧬️ CsvSnapshot schema — persistent fields + the real RFC4180 codec (dissolved out of the
//! former `⚙️engine`, ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES — kept beside the
//! `ArtifactDsl`/`ArtifactPack` impls that call it directly, mirroring `json`'s own already-
//! established `parse_json_text`/`write_json_text` placement in its `📸️snapshot/🦀️.rs`).

use crate::STDIO_CSV_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;

#[path = "🪶️sqlite/🦀️.rs"]
mod sqlite;
#[path = "🚦️native/🦀️.rs"]
mod sqlite_native;
#[cfg(test)]
#[path = "🧪️tests/🪶️sqlite/🦀️.rs"]
mod sqlite_tests;

fn default_true() -> bool {
    true
}

//#region 🔖️Field
/// 🔤 One RFC 4180 field value plus whether the source quoted it — rfc4180's own optional
/// quoting means whether a field WAS quoted is real information worth preserving losslessly,
/// so re-serializing can reproduce the exact source bytes rather than a lossy normal form
/// (https://www.rfc-editor.org/rfc/rfc4180#section-2, rule 5).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct CsvField {
    #[value(default)]
    pub value: String,
    #[value(default)]
    pub quoted: bool,
}
//#endregion 🔖️Field

//#region 🔖️Record
/// 📄 One RFC 4180 record (row) — a strong-like entity, index-keyed within
/// `CsvSnapshot::records`. Field COUNT is real, per-record information (rfc4180 is a
/// loosely-typed grid on the wire even though most producers keep it rectangular).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct CsvRecord {
    #[value(default)]
    pub fields: Vec<CsvField>,
}
//#endregion 🔖️Record

//#region 🔖️Snapshot
/// 📸️ Persisted `stdio.csv` snapshot (RFC 4180 table, with a header-row option). The
/// header row (when present) is `records[0]` — RFC 4180 draws no structural distinction
/// between a header record and a data record, only a convention of which one comes first.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.csv")]
pub struct CsvSnapshot {
    #[state(artifact)]
    pub schema: String,
    /// 📑 Whether the first record is a header row (RFC 4180's own optional convention).
    #[state(artifact)]
    #[value(default = "default_true")]
    pub has_header: bool,
    #[state(artifact)]
    #[value(default)]
    pub records: Vec<CsvRecord>,
}

impl Default for CsvSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_CSV_DOCUMENT_SCHEMA.into(), has_header: true, records: Vec::new() }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️Codec
//#region 🔖️Tokenizer
/// 📥 RFC 4180 record tokenizer over the WHOLE text (not line-by-line, so a quoted
/// field's embedded `\n`/`\r\n` is consumed as data, not a record boundary), handling
/// escaped `""` quotes, both CRLF and bare-LF line endings, and tracking per-field
/// whether the source actually wrapped it in quotes (real, losslessly-retained
/// information per RFC 4180 §2 rule 5 — quoting is optional).
fn parse_csv_records(text: &str) -> Vec<CsvRecord> {
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
fn escape_field(field: &CsvField) -> String {
    let needs_quote = field.quoted || field.value.contains(',') || field.value.contains('"') || field.value.contains('\n') || field.value.contains('\r');
    if needs_quote {
        format!("\"{}\"", field.value.replace('"', "\"\""))
    } else {
        field.value.clone()
    }
}

fn write_csv_records(records: &[CsvRecord], line_ending: &str) -> String {
    let mut out = String::new();
    for record in records {
        let csv_field_separator = ",";
        out.push_str(&record.fields.iter().map(escape_field).collect::<Vec<_>>().join(csv_field_separator));
        out.push_str(line_ending);
    }
    out
}
//#endregion 🔖️Tokenizer

//#region 🔖️SnapshotCodec
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
//#endregion 🔖️SnapshotCodec
//#endregion 🔖️Codec

//#region 🔖️DocumentHelpers
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
//#endregion 🔖️DocumentHelpers

//#region 🔖️HandcraftedArtifactCodecs
/// 📥️ Reads authored CSV files or the declared logical Text document.
pub fn read_csv_source_text(text:&str)->Result<CsvSnapshot,semio_framework_diagnostic::TextError>{if text.starts_with("semio "){<CsvSnapshot as store::ArtifactDsl>::parse_dsl(text)}else{Ok(decode_csv_with(text,true))}}
/// 📥️ Reads the logical Pack document or authored UTF-8 CSV bytes.
pub fn read_csv_source_binary(bytes:&[u8])->Result<CsvSnapshot,store::PackError>{if bytes.starts_with(&[137,83,69,77,13,10,26,10]){<CsvSnapshot as store::ArtifactPack>::decode_pack(bytes)}else{let text=std::str::from_utf8(bytes).map_err(|error|store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,error.to_string())))?;read_csv_source_text(text).map_err(store::PackError::from)}}

impl store::ArtifactDsl for CsvSnapshot{
 const EXTENSION:&'static str="csv";
 fn envelope_id()->&'static str{"stdio.csv"}
 fn parse_dsl(text:&str)->Result<Self,semio_framework_diagnostic::TextError>{let(envelope,body)=store::semio_format::split_text_preamble(text).map_err(|error|semio_framework_diagnostic::TextError::from_value_error(error.into_value_error(),semio_framework_diagnostic::TextSpan::at(1,1)))?;if !envelope.matches_identity(Self::envelope_id(),store::semio_format::Component::Dsl,1){return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"CSV logical Text envelope mismatch",semio_framework_diagnostic::TextSpan::at(1,1)))}Self::__dsl_from_record(&semio_framework_dsl_record::parse_exact(body,&Self::__dsl_spec(),&Default::default())?)}
 fn print_dsl(&self)->String{let body=semio_framework_dsl_record::print(&self.__dsl_to_record(),&Self::__dsl_spec(),semio_framework_dsl_record::JoinMode::Document);let envelope=store::semio_format::SemioEnvelope::from_envelope_id(Self::envelope_id(),store::semio_format::Component::Dsl,1).expect("declared CSV logical envelope");store::semio_format::wrap_text(&envelope,&body)}
}
impl store::ArtifactPack for CsvSnapshot{
 fn record_spec()->Option<semio_framework_dsl_record::RecordSpec>{Some(Self::__dsl_spec())}
 fn sqlite_snapshot_codec()->Option<store::ArtifactSqliteSnapshotCodec>{Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec())}
 fn encode_pack_with(&self,options:&store::PackEncodeOptions)->Result<Vec<u8>,store::PackError>{let body=store::pack_rt::encode_document(&Self::__dsl_spec(),&self.__dsl_to_record(),options)?;let envelope=store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(),store::semio_format::Component::Pack,1).map_err(|error|store::PackError::from(error.into_value_error()))?;Ok(store::semio_format::wrap_binary(&envelope,&body))}
 fn decode_pack_with(bytes:&[u8],options:&store::PackDecodeOptions)->Result<Self,store::PackError>{let(envelope,body)=store::semio_format::unwrap_binary(bytes).map_err(|error|store::PackError::from(error.into_value_error()))?;if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(),store::semio_format::Component::Pack,1){return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"CSV logical Pack envelope mismatch")))}Self::__dsl_from_record(&store::pack_rt::decode_document(&body,&Self::__dsl_spec(),options)?.0).map_err(store::PackError::from)}
}
//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
