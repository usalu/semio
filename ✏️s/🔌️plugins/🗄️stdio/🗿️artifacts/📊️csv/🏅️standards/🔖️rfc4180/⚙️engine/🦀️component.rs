//! ⚙️ CsvEngine — owns a real `CsvArtifact` + the real RFC 4180 codec.

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
/// escaped `""` quotes and both CRLF and bare-LF line endings.
fn parse_csv_records(text: &str) -> Vec<Vec<String>> {
    let mut records = Vec::new();
    let mut fields: Vec<String> = Vec::new();
    let mut cur = String::new();
    let mut in_quotes = false;
    let mut chars = text.chars().peekable();
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
            '"' => in_quotes = true,
            ',' => fields.push(std::mem::take(&mut cur)),
            '\r' => {
                if chars.peek() == Some(&'\n') {
                    chars.next();
                }
                fields.push(std::mem::take(&mut cur));
                records.push(std::mem::take(&mut fields));
            }
            '\n' => {
                fields.push(std::mem::take(&mut cur));
                records.push(std::mem::take(&mut fields));
            }
            _ => cur.push(ch),
        }
    }
    if !cur.is_empty() || !fields.is_empty() {
        fields.push(cur);
        records.push(fields);
    }
    records
}

fn escape_field(field: &str) -> String {
    if field.contains(',') || field.contains('"') || field.contains('\n') || field.contains('\r') {
        format!("\"{}\"", field.replace('"', "\"\""))
    } else {
        field.to_string()
    }
}

fn write_csv_records(records: &[Vec<String>], line_ending: &str) -> String {
    let mut out = String::new();
    for record in records {
        out.push_str(&record.iter().map(|f| escape_field(f)).collect::<Vec<_>>().join(","));
        out.push_str(line_ending);
    }
    out
}
//#endregion 🔖️Tokenizer

//#region 🔖️SnapshotCodec
/// 📥 Decodes RFC 4180 text into a snapshot, treating the first record as the header
/// row when `has_header` is true (the RFC4180 "header-row option").
pub fn decode_csv_with(text: &str, has_header: bool) -> CsvSnapshot {
    let mut records = parse_csv_records(text);
    let headers = if has_header && !records.is_empty() { records.remove(0) } else { Vec::new() };
    CsvSnapshot { schema: STDIO_CSV_DOCUMENT_SCHEMA.into(), has_header, headers, rows: records }
}

/// 📥 Decodes assuming a header row is present (the pre-existing default behavior).
pub fn decode_csv(text: &str) -> Result<CsvSnapshot, String> {
    Ok(decode_csv_with(text, true))
}

/// 📤 Encodes with LF line endings.
pub fn encode_csv(snap: &CsvSnapshot) -> String {
    encode_csv_with(snap, "\n")
}

/// 📤 Encodes with a caller-chosen line ending (`"\n"` or `"\r\n"`), honoring `has_header`.
pub fn encode_csv_with(snap: &CsvSnapshot, line_ending: &str) -> String {
    if snap.headers.is_empty() && snap.rows.is_empty() {
        return String::new();
    }
    let mut records = Vec::new();
    if snap.has_header {
        records.push(snap.headers.clone());
    }
    records.extend(snap.rows.iter().cloned());
    write_csv_records(&records, line_ending)
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

impl protocol::ArtifactEngine for CsvEngine {
    type Artifact = CsvArtifact;
    type Snapshot = CsvSnapshot;
    type Mutation = CsvMutation;
    type Diff = CsvDiff;

    fn artifact(&self) -> &Self::Artifact {
        &self.artifact_state
    }

    fn snapshot(&self) -> &Self::Snapshot {
        &self.snapshot_state
    }

    fn apply(&mut self, mutation: &Self::Mutation) -> Result<Self::Diff, protocol::EngineFault> {
        let diff = <Self::Mutation as protocol::Mutation<Self::Snapshot>>::diff(mutation, &self.snapshot_state);
        self.snapshot_state = <Self::Diff as protocol::MutationDiff<Self::Snapshot>>::apply(&diff, &self.snapshot_state);
        self.artifact_state.set_snapshot(self.snapshot_state.clone());
        Ok(diff)
    }

    fn inverse(&self, mutation: &Self::Mutation) -> Vec<Self::Mutation> {
        <Self::Mutation as protocol::Mutation<Self::Snapshot>>::inverse(mutation, &self.snapshot_state)
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

    #[test]
    fn quoted_field_with_embedded_comma_and_escaped_quote() {
        let text = "name,note\n\"Doe, John\",\"He said \"\"hi\"\"\"\n";
        let snap = decode_csv_with(text, true);
        assert_eq!(snap.headers, vec!["name", "note"]);
        assert_eq!(snap.rows, vec![vec!["Doe, John".to_string(), "He said \"hi\"".to_string()]]);
    }

    #[test]
    fn quoted_field_with_embedded_newline_spans_records() {
        let text = "a,b\n\"line1\nline2\",2\n";
        let snap = decode_csv_with(text, true);
        assert_eq!(snap.rows, vec![vec!["line1\nline2".to_string(), "2".to_string()]]);
    }

    #[test]
    fn crlf_and_lf_both_parse_to_the_same_records() {
        let lf = "a,b\n1,2\n3,4\n";
        let crlf = "a,b\r\n1,2\r\n3,4\r\n";
        assert_eq!(decode_csv_with(lf, true), decode_csv_with(crlf, true));
    }

    #[test]
    fn header_row_option_toggles_whether_first_record_is_consumed() {
        let text = "1,2\n3,4\n";
        let with_header = decode_csv_with(text, true);
        assert_eq!(with_header.headers, vec!["1", "2"]);
        assert_eq!(with_header.rows, vec![vec!["3".to_string(), "4".to_string()]]);

        let without_header = decode_csv_with(text, false);
        assert!(without_header.headers.is_empty());
        assert_eq!(without_header.rows, vec![vec!["1".to_string(), "2".to_string()], vec!["3".to_string(), "4".to_string()]]);
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
