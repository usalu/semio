use super::*;

const REAL_FIXTURE: &str = include_str!("../../../📚️examples/🎬️demo/🖼️assets/🧪️example/🌦️.epw");

#[semio_framework_async_macros::async_test]
async fn sniffs_and_parses_a_real_shaped_location_line() {
    let line = "LOCATION,Hannover,Niedersachsen,DEU,semio-fixture,10238,52.37,9.74,1.0,55.0";
    assert!(sniff_real_bytes(line.as_bytes()));
    let loc = parse_location_line(line).expect("parse");
    assert_eq!(loc.city, "Hannover");
    assert_eq!(loc.latitude, "52.37");
    assert_eq!(loc.elevation, "55.0");
}

#[semio_framework_async_macros::async_test]
async fn rejects_a_short_location_line() {
    assert!(parse_location_line("LOCATION,Hannover").is_err());
}

#[semio_framework_async_macros::async_test]
async fn rejects_a_record_with_the_wrong_column_count() {
    assert!(parse_record_line("2026,1,15,1,0").is_err());
}

#[semio_framework_async_macros::async_test]
async fn decodes_the_real_fixture_with_all_24_records_and_35_columns() {
    let snap = decode_epw(REAL_FIXTURE).expect("decode real fixture");
    assert_eq!(snap.location.city, "Hannover");
    assert_eq!(snap.location.latitude, "52.37");
    assert_eq!(snap.location.time_zone, "1.0");
    assert_eq!(snap.location.elevation, "55.0");
    assert_eq!(snap.records.len(), 24, "fixture is one full day, hour-ending 1..24");
    for (i, r) in snap.records.iter().enumerate() {
        assert_eq!(r.hour, (i + 1).to_string(), "hour column must run 1..24 in order");
        assert_eq!(r.fields().len(), EPW_RECORD_FIELD_COUNT);
    }
    assert_eq!(snap.data_periods.records_per_hour, 1);
    assert_eq!(snap.data_periods.periods.len(), 1);
    assert_eq!(snap.data_periods.periods[0].name, "Data");
}

//#region 🔖️CodecRetentionLaw
/// 🔁️ decode→encode is byte-preserving on the real W0 fixture
/// (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/📚️examples/🎬️demo/🖼️assets/🌦️example.epw`,
/// verified upstream by `verify_epw.py`): all 24 records × 35 columns exact, all 8 header
/// lines exact, byte-for-byte incl. CRLF.
#[semio_framework_async_macros::async_test]
async fn codec_retention_law() {
    let snap = decode_epw(REAL_FIXTURE).expect("decode real fixture");
    let reencoded = encode_epw(&snap);
    assert_eq!(reencoded, REAL_FIXTURE, "decode->encode must be byte-preserving on the real W0 fixture");

    let reparsed = decode_epw(&reencoded).expect("re-decode");
    assert_eq!(reparsed, snap, "re-parsing the re-encoded text must yield the identical snapshot");
}
//#endregion 🔖️CodecRetentionLaw
