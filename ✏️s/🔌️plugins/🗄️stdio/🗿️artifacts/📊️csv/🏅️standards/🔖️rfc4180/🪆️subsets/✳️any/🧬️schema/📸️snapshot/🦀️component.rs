//! 🧬️ CsvSnapshot schema — persistent fields + the real RFC4180 codec (dissolved out of the
//! former `⚙️engine`, ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES — kept beside the
//! `ArtifactDsl`/`ArtifactPack` impls that call it directly, mirroring `json`'s own already-
//! established `parse_json_text`/`write_json_text` placement in its `📸️snapshot/🦀️component.rs`).

use crate::artifacts::csv::STDIO_CSV_DOCUMENT_SCHEMA;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

fn default_true() -> bool {
    true
}

//#region 🔖️Field
/// 🔤 One RFC 4180 field value plus whether the source quoted it — rfc4180's own optional
/// quoting means whether a field WAS quoted is real information worth preserving losslessly,
/// so re-serializing can reproduce the exact source bytes rather than a lossy normal form
/// (https://www.rfc-editor.org/rfc/rfc4180#section-2, rule 5).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CsvField {
    #[serde(default)]
    pub value: String,
    #[serde(default)]
    pub quoted: bool,
}
//#endregion 🔖️Field

//#region 🔖️Record
/// 📄 One RFC 4180 record (row) — a strong-like entity, index-keyed within
/// `CsvSnapshot::records`. Field COUNT is real, per-record information (rfc4180 is a
/// loosely-typed grid on the wire even though most producers keep it rectangular).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CsvRecord {
    #[serde(default)]
    pub fields: Vec<CsvField>,
}
//#endregion 🔖️Record

//#region 🔖️Snapshot
/// 📸️ Persisted `stdio.csv` snapshot (RFC 4180 table, with a header-row option). The
/// header row (when present) is `records[0]` — RFC 4180 draws no structural distinction
/// between a header record and a data record, only a convention of which one comes first.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.csv")]
pub struct CsvSnapshot {
    #[state(artifact)]
    pub schema: String,
    /// 📑 Whether the first record is a header row (RFC 4180's own optional convention).
    #[state(artifact)]
    #[serde(default = "default_true")]
    pub has_header: bool,
    #[state(artifact)]
    #[serde(default)]
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

//#region 🔖️DocumentHelpers
/// 🌱 Empty persisted snapshot.
pub fn empty_csv_snapshot() -> CsvSnapshot {
    CsvSnapshot::default()
}

/// 📄️ The `demo` example, parsed once from `examples::demo::PRIMARY_TEXT` — the single source
/// of truth `🗣️example.dsl.semio` is genuinely `print_dsl` of (P2-P1 `fixture_honesty_law`),
/// same pattern as `note::semio_example_snapshot`.
pub fn demo_csv_snapshot() -> CsvSnapshot {
    <CsvSnapshot as store::ArtifactDsl>::parse_dsl(crate::artifacts::csv::examples::demo::PRIMARY_TEXT).unwrap_or_else(|_| empty_csv_snapshot())
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️HandcraftedArtifactCodecs
impl store::ArtifactDsl for CsvSnapshot {
    const EXTENSION: &'static str = "csv";
    fn envelope_id() -> &'static str {
        "stdio.csv"
    }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        Ok(decode_csv_with(body, true))
    }
    fn print_dsl(&self) -> String {
        let body = encode_csv(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for CsvSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = encode_csv(self).into_bytes();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let _ = options;
        let text = String::from_utf8(inner).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(decode_csv_with(&text, true))
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::csv::CsvMutation;

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
        let snap = CsvSnapshot { schema: STDIO_CSV_DOCUMENT_SCHEMA.into(), has_header: false, records: vec![CsvRecord { fields: vec![CsvField { value: "plain".into(), quoted: true }] }] };
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

    //#region 🔖️CodecRetentionLaw
    /// 🔁️ decode→encode is byte-preserving on a fixture exercising every retention-sensitive
    /// case at once: unquoted fields, a field quoted only because it's structurally required
    /// (embedded comma), a field quoted despite NOT being structurally required (pure
    /// retention), an empty field, and an embedded-newline field spanning lines.
    #[test]
    fn codec_retention_law() {
        let fixture = "name,note,tag,blank\n\"Doe, John\",\"He said \"\"hi\"\"\",\"kept-quoted\",\n\"multi\nline\",x,y,z\n";
        let snap = decode_csv_with(fixture, true);
        let reencoded = encode_csv(&snap);
        assert_eq!(reencoded, fixture, "decode->encode must be byte-preserving on this fixture");

        let reparsed = decode_csv_with(&reencoded, true);
        assert_eq!(reparsed, snap, "re-parsing the re-encoded text must yield the identical snapshot");
    }
    //#endregion 🔖️CodecRetentionLaw

    //#region 🔖️ScratchFixtureGen
    /// 🧪️[DEBUG] one-shot scratch generator — writes real `encode_pack`/`encode_op` bytes to the
    /// committed fixture paths. Run once via `--ignored`, then this region is deleted (never a
    /// permanent side-effecting test; CLAUDE.md bans migration scripts left behind).
    #[test]
    #[ignore]
    fn zzz_generate_p2p1_fixtures() {
        let repo_root = {
            let mut dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            loop {
                if dir.join("nx.json").is_file() {
                    break dir;
                }
                assert!(dir.pop(), "could not find repo root");
            }
        };
        let assets = repo_root.join("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/📚️examples/🎬️demo/🖼️assets");
        let demo = demo_csv_snapshot();
        let pack_bytes = <CsvSnapshot as store::ArtifactPack>::encode_pack(&demo);
        std::fs::write(assets.join("🎒️example.pack.semio"), &pack_bytes).unwrap();
        let mutation = CsvMutation::InsertRecord { index: 1, record: CsvRecord { fields: vec![CsvField { value: "brand-new".into(), quoted: true }] } };
        let op_bytes = <CsvMutation as protocol::OpBinary>::encode_op(&mutation).unwrap();
        std::fs::write(assets.join("📡️example.spr.semio"), &op_bytes).unwrap();
        eprintln!("[DEBUG] wrote {} pack bytes, {} spr bytes", pack_bytes.len(), op_bytes.len());
    }
    //#endregion 🔖️ScratchFixtureGen
}
//#endregion 🧪️Tests
