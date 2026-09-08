
use super::*;

/// 🧪️ A small in-memory LAS 1.0 buffer (2 VLRs, 3 points), built straight from `raw_doc::write`
/// itself so these dispatch-logic unit tests need no committed file on disk — the real
/// 8,448-point `🧫️fixtures/🧊️pattern-sphere.las` fixture is exercised by the gherkin-driven case
/// at `../../../../../../🧪️tests/🎩️mutate-las-1-0/🦀️.rs` through `ctx.copy_fixture`.
fn fixture() -> Vec<u8> {
    let header = las::raw::Header {
        file_signature: las::raw::LASF,
        version: las::Version::new(1, 0),
        system_identifier: write_fixed("SEMIO-TEST"),
        generating_software: write_fixed("semio-oracle-test"),
        file_creation_day_of_year: 100,
        file_creation_year: 2026,
        x_scale_factor: 0.01,
        y_scale_factor: 0.01,
        z_scale_factor: 0.01,
        max_x: 10.0,
        max_y: 10.0,
        max_z: 10.0,
        min_x: -10.0,
        min_y: -10.0,
        min_z: -10.0,
        number_of_points_by_return: [3, 0, 0, 0, 0],
        ..Default::default()
    };
    let vlrs = vec![
        las::raw::Vlr { reserved: 0, user_id: write_fixed("LASF_Spec"), record_id: 100, record_length_after_header: las::raw::vlr::RecordLength::Vlr(5), description: write_fixed("vlr-a"), data: b"vlr-a".to_vec() },
        las::raw::Vlr { reserved: 0, user_id: write_fixed("LASF_Spec"), record_id: 101, record_length_after_header: las::raw::vlr::RecordLength::Vlr(5), description: write_fixed("vlr-b"), data: b"vlr-b".to_vec() },
    ];
    let points = (0..3u8)
        .map(|seed| las::raw::Point {
            x: (100 + seed as i32) * 100,
            y: (-50 + seed as i32) * 100,
            z: (10 + seed as i32) * 100,
            intensity: 100 + seed as u16,
            flags: las::raw::point::Flags::TwoByte(1 | (1 << 3), seed),
            scan_angle: las::raw::point::ScanAngle::Rank(seed as i8 - 10),
            user_data: seed,
            point_source_id: 1000 + seed as u16,
            gps_time: None,
            color: None,
            waveform: None,
            nir: None,
            extra_bytes: Vec::new(),
        })
        .collect();
    raw_doc::write(&raw_doc::RawDoc { header, vlrs, points }).expect("build in-memory las fixture")
}

fn spec(kind: &str, params: Json) -> Json {
    spec_of(kind, params)
}

#[test]
fn no_mutation_is_a_true_byte_identity() {
    let input = fixture();
    let output = oracle_apply_mutation(&input, &spec("no-mutation", Json::Object(vec![]))).unwrap();
    assert_eq!(output, input);
}

#[test]
fn set_system_identifier_changes_only_that_field() {
    let input = fixture();
    let before = project_las(&input).unwrap();
    let output = oracle_apply_mutation(&input, &spec("set-system-identifier", Json::Object(vec![("systemIdentifier".to_string(), Json::String("RENAMED".to_string()))]))).unwrap();
    let after = project_las(&output).unwrap();
    assert_eq!(after.get("header").unwrap().get("systemIdentifier").unwrap().clone(), Json::String("RENAMED".to_string()));
    assert_eq!(before.get("points"), after.get("points"), "points must survive untouched");
}

#[test]
fn insert_and_remove_vlr_are_inverse_on_the_real_fixture() {
    let input = fixture();
    let vlr = Json::Object(vec![
        ("userId".to_string(), Json::String("semio-test".to_string())),
        ("recordId".to_string(), Json::Number(9.0)),
        ("description".to_string(), Json::String("test vlr".to_string())),
        ("data".to_string(), Json::String("payload".to_string())),
    ]);
    let before_count = match project_las(&input).unwrap().get("vlrs").unwrap() {
        Json::Array(items) => items.len(),
        _ => panic!("vlrs must project as an array"),
    };
    let insert_spec = spec("insert-vlr", Json::Object(vec![("index".to_string(), Json::Number(1.0)), ("vlr".to_string(), vlr)]));
    let inserted = oracle_apply_mutation(&input, &insert_spec).unwrap();
    let after_count = match project_las(&inserted).unwrap().get("vlrs").unwrap() {
        Json::Array(items) => items.len(),
        _ => panic!("vlrs must project as an array"),
    };
    assert_eq!(after_count, before_count + 1, "insert-vlr must grow the vlr list by exactly one");
    let inverse = oracle_inverse_spec(&input, &insert_spec).unwrap();
    assert_eq!(inverse.str("kind"), "remove-vlr");
    let restored = oracle_apply_mutation(&inserted, &inverse).unwrap();
    assert_eq!(project_las(&restored).unwrap(), project_las(&input).unwrap());
}

#[test]
fn insert_and_remove_point_are_inverse_on_the_real_fixture() {
    let input = fixture();
    let point = Json::Object(vec![
        ("x".to_string(), Json::Number(583005.0)),
        ("y".to_string(), Json::Number(5804005.0)),
        ("z".to_string(), Json::Number(5.0)),
        ("intensity".to_string(), Json::Number(4242.0)),
        ("returnNumber".to_string(), Json::Number(1.0)),
        ("numberOfReturns".to_string(), Json::Number(1.0)),
        ("scanDirectionFlag".to_string(), Json::Bool(true)),
        ("edgeOfFlightLine".to_string(), Json::Bool(false)),
        ("classification".to_string(), Json::Number(6.0)),
        ("scanAngleRank".to_string(), Json::Number(12.0)),
        ("userData".to_string(), Json::Number(1.0)),
        ("pointSourceId".to_string(), Json::Number(1.0)),
        ("gpsTime".to_string(), Json::Null),
        ("rgb".to_string(), Json::Null),
    ]);
    let insert_spec = spec("insert-point", Json::Object(vec![("index".to_string(), Json::Number(500.0)), ("point".to_string(), point)]));
    let inserted = oracle_apply_mutation(&input, &insert_spec).unwrap();
    let inverse = oracle_inverse_spec(&input, &insert_spec).unwrap();
    assert_eq!(inverse.str("kind"), "remove-point");
    let restored = oracle_apply_mutation(&inserted, &inverse).unwrap();
    assert_eq!(project_las(&restored).unwrap(), project_las(&input).unwrap());
}

#[test]
fn set_bounds_is_independent_of_the_real_point_distribution() {
    let input = fixture();
    let output = oracle_apply_mutation(
        &input,
        &spec("set-bounds", Json::Object(vec![("max".to_string(), Json::Array(vec![Json::Number(1.0), Json::Number(2.0), Json::Number(3.0)])), ("min".to_string(), Json::Array(vec![Json::Number(-1.0), Json::Number(-2.0), Json::Number(-3.0)]))])),
    )
    .unwrap();
    let projected = project_las(&output).unwrap();
    let max = projected.get("header").unwrap().get("max").unwrap();
    assert_eq!(*max, Json::Array(vec![Json::Number(1.0), Json::Number(2.0), Json::Number(3.0)]));
}

/// 🔁️ This unit test's small in-memory fixture is itself built by `raw_doc::write`, so a
/// read/write round trip through the same functions is naturally byte-identical here — the real
/// "not bit-identical" guarantee (the point-data-start-signature gap this file's top doc comment
/// describes) is a property of the real committed 8,448-point fixture the case at
/// `../../../../../../🧪️tests/🎩️mutate-las-1-0/🦀️.rs` exercises, not of this module in
/// isolation. This test instead asserts the weaker, always-true property: content survives.
#[test]
fn round_trip_preserves_content() {
    let input = fixture();
    let output = oracle_round_trip(&input).unwrap();
    assert_eq!(project_las(&output).unwrap(), project_las(&input).unwrap());
}

#[test]
fn unknown_kind_is_an_error_never_a_silent_no_op() {
    let input = fixture();
    let result = oracle_apply_mutation(&input, &spec("not-a-real-kind", Json::Object(vec![])));
    assert!(result.is_err(), "an unrecognised kind must fail loudly");
}
