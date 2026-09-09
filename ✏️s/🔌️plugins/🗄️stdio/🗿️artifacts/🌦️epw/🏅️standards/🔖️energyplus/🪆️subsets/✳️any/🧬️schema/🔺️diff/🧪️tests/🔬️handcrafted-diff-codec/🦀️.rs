use super::*;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn location(city: &str) -> EpwLocation {
    EpwLocation { city: city.into(), state_province: "NI".into(), country: "DEU".into(), source: "SRC".into(), wmo: "10238".into(), latitude: "52.37".into(), longitude: "9.74".into(), time_zone: "1.0".into(), elevation: "55.0".into() }
}
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn record(seed: &str) -> EpwRecord {
    let mut r = EpwRecord::default();
    r.year = "2026".into();
    r.month = "1".into();
    r.day = "15".into();
    r.hour = seed.into();
    r.dry_bulb_temp = format!("-{seed}.0");
    r
}
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn snapshot(city: &str) -> EpwSnapshot {
    EpwSnapshot { location: location(city), records: vec![record("1"), record("2"), record("3")], ..EpwSnapshot::default() }
}

#[semio_framework_async_macros::async_test]
async fn diff_codec_text_binary_roundtrip_law() {
    let a = snapshot("Hannover");
    let mut b = snapshot("Berlin");
    b.records[1] = record("2-modified, tricky [value]");
    let cases = vec![EpwDiff::default(), EpwDiff::between(&a, &b), EpwDiff::between(&b, &a)];
    for d in cases {
        let printed = d.print_diff();
        assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
        let parsed = EpwDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
        assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

        let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
        let decoded = EpwDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
        assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
    }
}
