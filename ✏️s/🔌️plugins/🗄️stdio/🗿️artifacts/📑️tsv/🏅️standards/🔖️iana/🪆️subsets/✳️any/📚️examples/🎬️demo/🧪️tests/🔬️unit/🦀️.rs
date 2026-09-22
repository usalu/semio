use super::*;
use crate::standards::iana::subsets::any::schema::snapshot::{decode_tsv, encode_tsv, TsvSnapshot};

#[semio_framework_async_macros::async_test]
async fn demo_source_nonempty() {
    assert!(!PRIMARY_TEXT.is_empty());
    let _ = source();
}

/// 📊️ The shipped example body IS this subset's own printed form of the authored TSV file.
///
/// The law the hex scaffold passed everything else by: a byte-exact split/rejoin codec round-trips
/// ANY text, so `parse → print → parse` said nothing about whether the body was a table at all.
/// This pins the two things the `stdio-tsv` pane actually depends on — the body parses back to the
/// exact grid `🖼️assets/📊️.tsv` holds (5 columns × 6 records, header `id…note`), and it is
/// byte-for-byte what `print_dsl` emits for that grid, preamble included. A body that is not this
/// subset's DSL (hex, base64, a stray pack) fails here instead of reaching a pane as one cell.
#[semio_framework_async_macros::async_test]
async fn demo_dsl_is_this_subsets_own_printed_table() {
    let parsed: TsvSnapshot = store::ArtifactDsl::parse_dsl(PRIMARY_TEXT).expect("the demo body parses as stdio.tsv DSL");
    assert_eq!(encode_tsv(&parsed), RAW_TSV, "the demo body is not the authored TSV file");
    assert_eq!(store::ArtifactDsl::print_dsl(&parsed), PRIMARY_TEXT, "the demo body is not this crate's own printed form — regenerate it with zzz_write_demo_fixtures");
    assert_eq!(parsed, decode_tsv(RAW_TSV), "the demo body and the authored TSV file disagree");
    assert_eq!(parsed.records.len(), 6, "the demo table is a header row plus five data rows");
    assert!(parsed.records.iter().all(|record| record.len() == 5), "every demo record owns the header's five fields, got {:?}", parsed.records.iter().map(Vec::len).collect::<Vec<_>>());
    assert_eq!(parsed.records[0], ["id", "name", "qty", "unit_price", "note"], "the demo header names the price-list columns");
}

/// ✍️ Regenerates `🗣️.dsl.semio` (and its pack sibling) from `🖼️assets/📊️.tsv` with THIS crate's
/// own printer — never by hand. `cargo test -p semio-s-artifact-stdio-tsv --lib -- --ignored zzz_write`.
#[semio_framework_async_macros::async_test]
#[ignore]
async fn zzz_write_demo_fixtures() {
    let snapshot = decode_tsv(RAW_TSV);
    let text = store::ArtifactDsl::print_dsl(&snapshot);
    let pack: Vec<u8> = store::ArtifactPack::encode_pack(&snapshot);
    let assets = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️iana/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets");
    std::fs::write(assets.join("🗣️.dsl.semio"), &text).unwrap_or_else(|error| panic!("write {}: {error}", assets.display()));
    std::fs::write(assets.join("🎒️.pack.semio"), &pack).unwrap_or_else(|error| panic!("write {}: {error}", assets.display()));
}
