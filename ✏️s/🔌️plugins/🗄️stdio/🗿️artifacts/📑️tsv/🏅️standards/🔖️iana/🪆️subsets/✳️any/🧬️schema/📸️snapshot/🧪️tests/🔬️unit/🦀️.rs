use super::*;

const REAL_FIXTURE: &str = include_str!("../../../../📚️examples/🎬️demo/🖼️assets/📊️.tsv");

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
/// (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/📚️examples/🎬️demo/🖼️assets/📊️.tsv`,
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
