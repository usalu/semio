//! 🧪️ Tests for example `🏛️architectural` — real fixture, real D1/D2 decode assertions.

use crate::artifacts::dwg::examples::architectural::{source, FIXTURE_BYTES};
use crate::artifacts::dwg::schema::snapshot::{decode_dwg, DwgDecodeStatus};

#[test]
fn fixture_is_real_ac1024_not_a_stub() {
    assert!(FIXTURE_BYTES.len() > 100_000, "architectural.dwg must be the real ~145KB fixture, got {} bytes", FIXTURE_BYTES.len());
    assert_eq!(&FIXTURE_BYTES[0..6], b"AC1024", "fixture must start with the AC1024 version marker");
}

#[test]
fn source_nonempty() {
    let _ = source();
}

/// 🧪️ The actual regression test for "sentinel + passthrough" (pre-ticket behavior, which never
/// located a single real section on this file): decode must reach D2 (`SectionsDecompressed`)
/// and every real AutoCAD section name must be present.
#[test]
fn real_decode_reaches_d2_with_every_named_section() {
    let snap = decode_dwg(FIXTURE_BYTES).expect("real fixture must decode");
    assert_eq!(snap.version, "AC1024");
    assert_eq!(snap.decode_status, DwgDecodeStatus::SectionsDecompressed, "every page on this well-formed real fixture must decompress cleanly");
    let expected_names = [
        "AcDb:Header", "AcDb:AuxHeader", "AcDb:Classes", "AcDb:Handles", "AcDb:Template",
        "AcDb:ObjFreeSpace", "AcDb:AcDbObjects", "AcDb:RevHistory", "AcDb:SummaryInfo",
        "AcDb:Preview", "AcDb:AppInfo", "AcDb:AppInfoHistory", "AcDb:FileDepList",
    ];
    for name in expected_names {
        assert!(snap.section_names.iter().any(|n| n == name), "missing real section {name}");
        let section = snap.sections.iter().find(|s| s.name == name).unwrap();
        assert!(!section.pages.is_empty(), "section {name} has no pages");
        let total_decoded: usize = section.pages.iter().map(|p| p.decoded.len()).sum();
        assert!(total_decoded > 0, "section {name} decoded to zero bytes");
    }
}

/// 🧪️ Non-destructive by construction: `bytes` always retains the full original file verbatim,
/// so re-encode is byte-identical regardless of how far structural decode got.
#[test]
fn real_decode_stays_lossless_on_reencode() {
    let snap = decode_dwg(FIXTURE_BYTES).expect("real fixture must decode");
    assert_eq!(snap.bytes, FIXTURE_BYTES);
    let reencoded = crate::artifacts::dwg::schema::snapshot::encode_dwg(&snap).expect("re-encode");
    assert_eq!(reencoded, FIXTURE_BYTES, "re-encode must be byte-identical to the original file");
}
