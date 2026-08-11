//! ⚙️ CsvEngine — owns a real `CsvArtifact` + the real RFC 4180 codec.

use crate::artifacts::csv::schema::snapshot::{CsvField, CsvRecord};
use crate::artifacts::csv::{CsvArtifact, CsvDiff, CsvMutation, CsvSnapshot, STDIO_CSV_DOCUMENT_SCHEMA};

//#region 🔖️DocumentHelpers
/// 🌱 Empty persisted snapshot.
pub fn empty_csv_snapshot() -> CsvSnapshot {
    CsvSnapshot::default()
}
//#endregion 🔖️DocumentHelpers

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
    let needs_quote = field.quoted
        || field.value.contains(',')
        || field.value.contains('"')
        || field.value.contains('\n')
        || field.value.contains('\r');
    if needs_quote {
        format!("\"{}\"", field.value.replace('"', "\"\""))
    } else {
        field.value.clone()
    }
}

fn write_csv_records(records: &[CsvRecord], line_ending: &str) -> String {
    let mut out = String::new();
    for record in records {
        out.push_str(&record.fields.iter().map(escape_field).collect::<Vec<_>>().join(","));
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

//#region 🔖️Register
/// 🗂️ Registers codecs and the artifact schema descriptor.
pub fn register() {
    crate::artifacts::csv::composer::register();
    register_artifact_schema();
    register_pilot_languages();
    store::register_document_codec(store::ArtifactCodec::of::<CsvSnapshot, CsvMutation>(STDIO_CSV_DOCUMENT_SCHEMA));
}

/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary).
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.csv",
        extension: Some("csv"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::csv::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::csv::schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::csv::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::csv::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.csv"),
    });
}

/// 📌️ Registers schema leaves for `s.stdio.csv`.
pub fn register_artifact_schema() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::csv::schema::csv_artifact_schema_descriptor());
}
//#endregion 🔖️Register

//#region 🔖️ArtifactEngine
/// ⚙️ `stdio.csv` artifact engine.
pub struct CsvEngine {
    artifact_state: CsvArtifact,
    snapshot_state: CsvSnapshot,
}

impl CsvEngine {
    /// 🏗️ Builds an engine from a persisted snapshot.
    pub fn new(snapshot: CsvSnapshot) -> Self {
        let artifact_state = CsvArtifact::from_snapshot(snapshot.clone());
        Self { artifact_state, snapshot_state: snapshot }
    }
}
//#endregion 🔖️ArtifactEngine

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_snapshot_matches_schema() {
        let snapshot = empty_csv_snapshot();
        assert_eq!(snapshot.schema, STDIO_CSV_DOCUMENT_SCHEMA);
    }

    #[test]
    fn codec_round_trip() {
        let snap = empty_csv_snapshot();
        let text = store::ArtifactDsl::print_dsl(&snap);
        let parsed = <CsvSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(parsed.schema, snap.schema);
        let bytes = store::ArtifactPack::encode_pack(&snap);
        let decoded = <CsvSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(decoded, snap);
    }

    fn field_values(record: &CsvRecord) -> Vec<String> {
        record.fields.iter().map(|f| f.value.clone()).collect()
    }

    #[test]
    fn quoted_field_with_embedded_comma_and_escaped_quote() {
        let text = "name,note\n\"Doe, John\",\"He said \"\"hi\"\"\"\n";
        let snap = decode_csv_with(text, true);
        assert_eq!(field_values(&snap.records[0]), vec!["name", "note"]);
        assert_eq!(field_values(&snap.records[1]), vec!["Doe, John".to_string(), "He said \"hi\"".to_string()]);
        assert!(snap.records[1].fields[0].quoted, "comma-containing field must be recorded as quoted");
        assert!(snap.records[1].fields[1].quoted);
        assert!(!snap.records[0].fields[0].quoted, "unquoted header field stays unquoted");
    }

    #[test]
    fn quoted_field_with_embedded_newline_spans_records() {
        let text = "a,b\n\"line1\nline2\",2\n";
        let snap = decode_csv_with(text, true);
        assert_eq!(field_values(&snap.records[1]), vec!["line1\nline2".to_string(), "2".to_string()]);
    }

    #[test]
    fn crlf_and_lf_both_parse_to_the_same_records() {
        let lf = "a,b\n1,2\n3,4\n";
        let crlf = "a,b\r\n1,2\r\n3,4\r\n";
        assert_eq!(decode_csv_with(lf, true), decode_csv_with(crlf, true));
    }

    #[test]
    fn header_row_option_is_pure_metadata_first_record_always_decoded() {
        let text = "1,2\n3,4\n";
        let with_header = decode_csv_with(text, true);
        assert!(with_header.has_header);
        assert_eq!(with_header.records.len(), 2);
        assert_eq!(field_values(&with_header.records[0]), vec!["1", "2"]);
        assert_eq!(field_values(&with_header.records[1]), vec!["3", "4"]);

        let without_header = decode_csv_with(text, false);
        assert!(!without_header.has_header);
        assert_eq!(without_header.records, with_header.records);
    }

    #[test]
    fn quoted_flag_round_trips_even_when_not_structurally_required() {
        // 🔒 A field that didn't NEED quoting but WAS quoted in the source must re-encode
        // quoted (lossless retention, not a lossy structural-minimum normal form).
        let snap = CsvSnapshot {
            schema: STDIO_CSV_DOCUMENT_SCHEMA.into(),
            has_header: false,
            records: vec![CsvRecord { fields: vec![CsvField { value: "plain".into(), quoted: true }] }],
        };
        let text = encode_csv(&snap);
        assert_eq!(text, "\"plain\"\n");
        let reparsed = decode_csv_with(&text, false);
        assert_eq!(reparsed.records, snap.records);
    }

    #[test]
    fn encode_with_crlf_round_trips() {
        let snap = decode_csv_with("a,b\n1,2\n", true);
        let crlf_text = encode_csv_with(&snap, "\r\n");
        assert!(crlf_text.contains("\r\n"));
        let reparsed = decode_csv_with(&crlf_text, true);
        assert_eq!(reparsed, snap);
    }
}
//#endregion 🧪️Tests
