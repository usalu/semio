
use super::*;
use crate::CsvMutation;

#[semio_framework_async_macros::async_test]
async fn empty_snapshot_matches_schema() {
    let snapshot = empty_csv_snapshot();
    assert_eq!(snapshot.schema, STDIO_CSV_DOCUMENT_SCHEMA);
}

#[semio_framework_async_macros::async_test]
async fn codec_round_trip() {
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

#[semio_framework_async_macros::async_test]
async fn quoted_field_with_embedded_comma_and_escaped_quote() {
    let text = "name,note\n\"Doe, John\",\"He said \"\"hi\"\"\"\n";
    let snap = decode_csv_with(text, true);
    assert_eq!(field_values(&snap.records[0]), vec!["name", "note"]);
    assert_eq!(field_values(&snap.records[1]), vec!["Doe, John".to_string(), "He said \"hi\"".to_string()]);
    assert!(snap.records[1].fields[0].quoted, "comma-containing field must be recorded as quoted");
    assert!(snap.records[1].fields[1].quoted);
    assert!(!snap.records[0].fields[0].quoted, "unquoted header field stays unquoted");
}

#[semio_framework_async_macros::async_test]
async fn quoted_field_with_embedded_newline_spans_records() {
    let text = "a,b\n\"line1\nline2\",2\n";
    let snap = decode_csv_with(text, true);
    assert_eq!(field_values(&snap.records[1]), vec!["line1\nline2".to_string(), "2".to_string()]);
}

#[semio_framework_async_macros::async_test]
async fn crlf_and_lf_both_parse_to_the_same_records() {
    let lf = "a,b\n1,2\n3,4\n";
    let crlf = "a,b\r\n1,2\r\n3,4\r\n";
    assert_eq!(decode_csv_with(lf, true), decode_csv_with(crlf, true));
}

#[semio_framework_async_macros::async_test]
async fn header_row_option_is_pure_metadata_first_record_always_decoded() {
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

#[semio_framework_async_macros::async_test]
async fn quoted_flag_round_trips_even_when_not_structurally_required() {
    // 🔒 A field that didn't NEED quoting but WAS quoted in the source must re-encode
    // quoted (lossless retention, not a lossy structural-minimum normal form).
    let snap = CsvSnapshot { schema: STDIO_CSV_DOCUMENT_SCHEMA.into(), has_header: false, records: vec![CsvRecord { fields: vec![CsvField { value: "plain".into(), quoted: true }] }] };
    let text = encode_csv(&snap);
    assert_eq!(text, "\"plain\"\n");
    let reparsed = decode_csv_with(&text, false);
    assert_eq!(reparsed.records, snap.records);
}

#[semio_framework_async_macros::async_test]
async fn encode_with_crlf_round_trips() {
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
#[semio_framework_async_macros::async_test]
async fn codec_retention_law() {
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
#[semio_framework_async_macros::async_test]
#[ignore]
async fn zzz_generate_p2p1_fixtures() {
    let repo_root = {
        let mut dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        loop {
            let mut marker = dir.clone();
            marker.push("nx.json");
            if marker.is_file() {
                break dir;
            }
            assert!(dir.pop(), "could not find repo root");
        }
    };
    let mut assets = repo_root.clone();
    assets.push("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/📚️examples/🎬️demo/🖼️assets");
    let demo = demo_csv_snapshot();
    let pack_bytes = <CsvSnapshot as store::ArtifactPack>::encode_pack(&demo);
    let mut pack_path = assets.clone();
    pack_path.push("🎒️.pack.semio");
    std::fs::write(pack_path, &pack_bytes).unwrap();
    let mutation = CsvMutation::InsertRecord(crate::schema::mutations::insert_record::InsertRecord { index: 1, record: CsvRecord { fields: vec![CsvField { value: "brand-new".into(), quoted: true }] } });
    let op_bytes = <CsvMutation as protocol::OpBinary>::encode_op(&mutation).unwrap();
    let mut op_path = assets.clone();
    op_path.push("📡️example.spr.semio");
    std::fs::write(op_path, &op_bytes).unwrap();
    eprintln!("[DEBUG] wrote {} pack bytes, {} spr bytes", pack_bytes.len(), op_bytes.len());
}
//#endregion 🔖️ScratchFixtureGen
