
use super::*;
use crate::schema::{demo_las_snapshot, empty_las_snapshot};

#[semio_framework_async_macros::async_test]
async fn empty_snapshot_matches_schema() {
    let snapshot = empty_las_snapshot();
    assert_eq!(snapshot.schema, STDIO_LAS_DOCUMENT_SCHEMA);
}

#[semio_framework_async_macros::async_test]
async fn codec_round_trip() {
    let snap = empty_las_snapshot();
    let text = store::ArtifactDsl::print_dsl(&snap);
    let parsed = <LasSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
    assert_eq!(parsed.schema, snap.schema);
    let bytes = store::ArtifactPack::encode_pack(&snap);
    let decoded = <LasSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(decoded, snap);
}

//#region 🔖️Fixtures
/// 🧪 7 points with varied per-field values (not all zero/default) so a naive stub that
/// only reads x/y/z would fail these assertions on intensity/classification/flags/etc.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sample_points(fmt: u8) -> Vec<LasPoint> {
    (0..7)
        .map(|i| {
            let base = LasPoint {
                x: 100.0 + i as f64 * 1.23,
                y: -50.0 + i as f64 * 0.5,
                z: 10.0 + i as f64 * 0.01,
                intensity: 100 + i as u16 * 10,
                return_number: (i % 5) as u8,
                number_of_returns: ((i + 1) % 5) as u8,
                scan_direction_flag: i % 2 == 0,
                edge_of_flight_line: i % 3 == 0,
                classification: (i * 2) as u8,
                scan_angle_rank: (i as i8) - 3,
                user_data: i as u8,
                point_source_id: 1000 + i as u16,
                gps_time: None,
                rgb: None,
            };
            match fmt {
                1 => LasPoint { gps_time: Some(123456.789 + i as f64), ..base },
                2 => LasPoint { rgb: Some((1000 + i as u16, 2000 + i as u16, 3000 + i as u16)), ..base },
                3 => LasPoint { gps_time: Some(123456.789 + i as f64), rgb: Some((1000 + i as u16, 2000 + i as u16, 3000 + i as u16)), ..base },
                _ => base,
            }
        })
        .collect()
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sample_vlrs() -> Vec<LasVlr> {
    vec![
        LasVlr { user_id: "LASF_Projection".into(), record_id: 34735, description: "GeoKeyDirectoryTag".into(), data: vec![1, 0, 1, 0, 0, 0, 3, 0] },
        LasVlr { user_id: "semio".into(), record_id: 1, description: "custom metadata".into(), data: b"hello vlr".to_vec() },
    ]
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn snapshot_with(fmt: u8, vlrs: Vec<LasVlr>) -> LasSnapshot {
    let points = sample_points(fmt);
    LasSnapshot {
        schema: STDIO_LAS_DOCUMENT_SCHEMA.into(),
        header: LasHeader {
            version_major: 1,
            version_minor: 2,
            system_identifier: "SEMIO".into(),
            generating_software: "semio-las-engine".into(),
            creation_day_of_year: 123,
            creation_year: 2026,
            number_of_vlrs: vlrs.len() as u32,
            number_of_point_records: points.len() as u32,
            points_by_return: [1, 2, 3, 1, 0],
            max_x: 900.0,
            min_x: 0.0,
            max_y: 900.0,
            min_y: -900.0,
            max_z: 100.0,
            min_z: -100.0,
            ..LasHeader::default()
        },
        vlrs,
        points,
    }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn assert_points_match(a: &LasPoint, b: &LasPoint) {
    assert!((a.x - b.x).abs() < 1e-6, "x mismatch: {} vs {}", a.x, b.x);
    assert!((a.y - b.y).abs() < 1e-6, "y mismatch: {} vs {}", a.y, b.y);
    assert!((a.z - b.z).abs() < 1e-6, "z mismatch: {} vs {}", a.z, b.z);
    assert_eq!(a.intensity, b.intensity);
    assert_eq!(a.return_number, b.return_number);
    assert_eq!(a.number_of_returns, b.number_of_returns);
    assert_eq!(a.scan_direction_flag, b.scan_direction_flag);
    assert_eq!(a.edge_of_flight_line, b.edge_of_flight_line);
    assert_eq!(a.classification, b.classification);
    assert_eq!(a.scan_angle_rank, b.scan_angle_rank);
    assert_eq!(a.user_data, b.user_data);
    assert_eq!(a.point_source_id, b.point_source_id);
    match (a.gps_time, b.gps_time) {
        (Some(x), Some(y)) => assert!((x - y).abs() < 1e-6, "gps_time mismatch: {x} vs {y}"),
        (None, None) => {}
        other => panic!("gps_time presence mismatch: {other:?}"),
    }
    assert_eq!(a.rgb, b.rgb);
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn assert_vlrs_match(a: &LasVlr, b: &LasVlr) {
    assert_eq!(a.user_id, b.user_id);
    assert_eq!(a.record_id, b.record_id);
    assert_eq!(a.description, b.description);
    assert_eq!(a.data, b.data);
}
//#endregion 🔖️Fixtures

#[semio_framework_async_macros::async_test]
async fn format0_round_trip_all_fields() {
    let snap = snapshot_with(0, sample_vlrs());
    let bytes = encode_las(&snap).expect("encode fmt0");
    assert_eq!(bytes[104], 0, "point data format byte must be 0");
    assert_eq!(u16::from_le_bytes(bytes[105..107].try_into().unwrap()), 20);
    let decoded = decode_las(&bytes).expect("decode fmt0");
    assert_eq!(decoded.points.len(), snap.points.len());
    for (a, b) in snap.points.iter().zip(decoded.points.iter()) {
        assert_points_match(a, b);
        assert_eq!(b.gps_time, None);
        assert_eq!(b.rgb, None);
    }
    assert_eq!(decoded.vlrs.len(), snap.vlrs.len());
    for (a, b) in snap.vlrs.iter().zip(decoded.vlrs.iter()) {
        assert_vlrs_match(a, b);
    }
    assert_eq!(decoded.header.system_identifier, "SEMIO");
    assert_eq!(decoded.header.generating_software, "semio-las-engine");
    assert_eq!(decoded.header.creation_day_of_year, 123);
    assert_eq!(decoded.header.creation_year, 2026);
    assert_eq!(decoded.header.points_by_return, [1, 2, 3, 1, 0]);
    assert!((decoded.header.max_x - 900.0).abs() < 1e-9);
    assert!((decoded.header.min_y - (-900.0)).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn format1_round_trip_gps_time() {
    let snap = snapshot_with(1, sample_vlrs());
    let bytes = encode_las(&snap).expect("encode fmt1");
    assert_eq!(bytes[104], 1, "point data format byte must be 1");
    assert_eq!(u16::from_le_bytes(bytes[105..107].try_into().unwrap()), 28);
    let decoded = decode_las(&bytes).expect("decode fmt1");
    for (a, b) in snap.points.iter().zip(decoded.points.iter()) {
        assert_points_match(a, b);
        assert!(b.gps_time.is_some(), "format 1 must decode a gps_time");
        assert_eq!(b.rgb, None);
    }
}

#[semio_framework_async_macros::async_test]
async fn format2_round_trip_rgb() {
    let snap = snapshot_with(2, vec![]);
    let bytes = encode_las(&snap).expect("encode fmt2");
    assert_eq!(bytes[104], 2, "point data format byte must be 2");
    assert_eq!(u16::from_le_bytes(bytes[105..107].try_into().unwrap()), 26);
    let decoded = decode_las(&bytes).expect("decode fmt2");
    for (a, b) in snap.points.iter().zip(decoded.points.iter()) {
        assert_points_match(a, b);
        assert!(b.rgb.is_some(), "format 2 must decode rgb");
        assert_eq!(b.gps_time, None);
    }
    assert!(decoded.vlrs.is_empty(), "no vlrs on this fixture");
    assert_eq!(decoded.header.offset_to_point_data, 227, "offset_to_point_data with zero vlrs must equal the fixed header size");
}

#[semio_framework_async_macros::async_test]
async fn format3_round_trip_gps_time_and_rgb() {
    let snap = snapshot_with(3, sample_vlrs());
    let bytes = encode_las(&snap).expect("encode fmt3");
    assert_eq!(bytes[104], 3, "point data format byte must be 3");
    assert_eq!(u16::from_le_bytes(bytes[105..107].try_into().unwrap()), 34);
    let decoded = decode_las(&bytes).expect("decode fmt3");
    for (a, b) in snap.points.iter().zip(decoded.points.iter()) {
        assert_points_match(a, b);
        assert!(b.gps_time.is_some());
        assert!(b.rgb.is_some());
    }
}

#[semio_framework_async_macros::async_test]
async fn vlrs_shift_offset_to_point_data() {
    let with_vlrs = snapshot_with(0, sample_vlrs());
    let without_vlrs = snapshot_with(0, vec![]);
    let bytes_with = encode_las(&with_vlrs).expect("encode with vlrs");
    let bytes_without = encode_las(&without_vlrs).expect("encode without vlrs");
    let decoded_with = decode_las(&bytes_with).expect("decode with vlrs");
    let decoded_without = decode_las(&bytes_without).expect("decode without vlrs");
    assert!(decoded_with.header.offset_to_point_data > decoded_without.header.offset_to_point_data, "vlr bytes must push point data further out");
    assert_eq!(decoded_without.header.offset_to_point_data, 227);
    let expected_vlr_span: u32 = sample_vlrs().iter().map(|v| 54 + v.data.len() as u32).sum();
    assert_eq!(decoded_with.header.offset_to_point_data, 227 + expected_vlr_span);
    assert_eq!(decoded_with.header.number_of_vlrs, 2);
}

#[semio_framework_async_macros::async_test]
async fn point_offset_is_trusted_not_hardcoded_to_227() {
    let snap = snapshot_with(0, vec![]);
    let bytes = encode_las(&snap).expect("encode");
    let old_header = 227usize;
    let new_header = 200usize; // still >= 179 so every fixed header field we read still fits
    let mut shrunk = bytes[0..new_header].to_vec();
    shrunk[94..96].copy_from_slice(&(new_header as u16).to_le_bytes());
    shrunk[96..100].copy_from_slice(&(new_header as u32).to_le_bytes());
    shrunk.extend_from_slice(&bytes[old_header..]);
    let decoded = decode_las(&shrunk).expect("decode with non-227 header size");
    assert_eq!(decoded.points.len(), snap.points.len());
    for (a, b) in snap.points.iter().zip(decoded.points.iter()) {
        assert_points_match(a, b);
    }
}

#[semio_framework_async_macros::async_test]
async fn las_1_4_extended_point_count_fallback() {
    let snap = snapshot_with(0, vec![]);
    let bytes = encode_las(&snap).expect("encode");
    let header_size = 375usize;
    let mut out = vec![0u8; header_size];
    out[0..4].copy_from_slice(b"LASF");
    out[24] = 1;
    out[25] = 4; // version 1.4
    out[94..96].copy_from_slice(&(header_size as u16).to_le_bytes());
    out[96..100].copy_from_slice(&(header_size as u32).to_le_bytes());
    out[104] = 0;
    out[105..107].copy_from_slice(&20u16.to_le_bytes());
    out[107..111].copy_from_slice(&0u32.to_le_bytes()); // legacy count deliberately 0
    out[131..179].copy_from_slice(&bytes[131..179]); // reuse scale/offset from the fixture
    out[247..255].copy_from_slice(&(snap.points.len() as u64).to_le_bytes());
    out.extend_from_slice(&bytes[227..]); // point records
    let decoded = decode_las(&out).expect("decode las 1.4 extended count");
    assert_eq!(decoded.points.len(), snap.points.len(), "must fall back to the extended 1.4 point count");
    for (a, b) in snap.points.iter().zip(decoded.points.iter()) {
        assert_points_match(a, b);
    }
}

#[semio_framework_async_macros::async_test]
async fn unsupported_point_format_is_rejected() {
    let snap = snapshot_with(0, vec![]);
    let mut bytes = encode_las(&snap).expect("encode");
    bytes[104] = 99;
    let err = decode_las(&bytes).unwrap_err();
    assert!(err.contains("unsupported point data format"), "unexpected error: {err}");
}

#[semio_framework_async_macros::async_test]
async fn bad_signature_is_rejected() {
    let mut bytes = vec![0u8; 300];
    bytes[0..4].copy_from_slice(b"NOPE");
    let err = decode_las(&bytes).unwrap_err();
    assert!(err.contains("signature"));
}

#[semio_framework_async_macros::async_test]
async fn header_too_short_is_rejected() {
    let mut bytes = vec![0u8; 50];
    bytes[0..4].copy_from_slice(b"LASF");
    let err = decode_las(&bytes).unwrap_err();
    assert!(err.contains("too short"), "unexpected error: {err}");
}

#[semio_framework_async_macros::async_test]
async fn demo_snapshot_round_trip() {
    let snap = demo_las_snapshot();
    let text = store::ArtifactDsl::print_dsl(&snap);
    let parsed = <LasSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
    assert_eq!(parsed, snap);
    let bytes = store::ArtifactPack::encode_pack(&snap);
    let decoded = <LasSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(decoded, snap);
}

//#region 🔖️ConformanceLaws
/// 🧪️ Per-artifact conformance laws — grammar/protocol parseability, `Recognizer` against
/// real fixtures AND real `print_op`/`print_diff` output, `walk_protocol` against real
/// `encode_pack`/`encode_op`/`encode_diff` bytes, and the fixture-honesty round-trip.
mod conformance_laws {
    use super::*;
    use crate::schema::{diff, mutations, snapshot};
    use protocol::{DiffCodec, OpBinary, OpText};

    #[semio_framework_async_macros::async_test]
    async fn committed_facet_files_parse() {
        for (label, text) in [("snapshot grammar", snapshot::text::COMPONENT_GRAMMAR_SEMIO), ("mutations grammar", mutations::text::COMPONENT_GRAMMAR_SEMIO), ("diff grammar", diff::text::COMPONENT_GRAMMAR_SEMIO)] {
            let grammar = dsl::parse_grammar(text).unwrap_or_else(|e| panic!("{label}: parse_grammar failed: {e:?}"));
            assert_eq!(grammar.dialect, dsl::SemioDialect::Grammar, "{label}: expected grammar dialect");
        }
        for (label, text) in [("snapshot protocol", snapshot::binary::COMPONENT_PROTOCOL_SEMIO), ("mutations protocol", mutations::binary::COMPONENT_PROTOCOL_SEMIO), ("diff protocol", diff::binary::COMPONENT_PROTOCOL_SEMIO)] {
            dsl::parse_protocol(text).unwrap_or_else(|e| panic!("{label}: parse_protocol failed: {e:?}"));
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn grammar_conformance_law() {
        let grammar = dsl::parse_grammar(snapshot::text::COMPONENT_GRAMMAR_SEMIO).expect("parse snapshot grammar");
        let recognizer = dsl::Recognizer::compile(&grammar);
        let text = store::ArtifactDsl::print_dsl(&demo_las_snapshot());
        let (envelope, body) = store::semio_format::split_text_preamble(&text).expect("split preamble");
        let reconstructed = format!("{}\n{body}", envelope.envelope_id());
        assert!(recognizer.recognize(&reconstructed).expect("recognize"), "grammar did not recognize demo dsl body:\n{reconstructed}");

        let empty_text = store::ArtifactDsl::print_dsl(&empty_las_snapshot());
        let (empty_envelope, empty_body) = store::semio_format::split_text_preamble(&empty_text).expect("split preamble");
        let empty_reconstructed = format!("{}\n{empty_body}", empty_envelope.envelope_id());
        assert!(recognizer.recognize(&empty_reconstructed).expect("recognize"), "grammar did not recognize empty dsl body:\n{empty_reconstructed}");
    }

    #[semio_framework_async_macros::async_test]
    async fn ops_grammar_conformance_law() {
        let grammar = dsl::parse_grammar(mutations::text::COMPONENT_GRAMMAR_SEMIO).expect("parse mutations grammar");
        let recognizer = dsl::Recognizer::compile(&grammar);
        for mutation in mutations::demo_mutation_cases() {
            let printed = mutation.print_op();
            assert!(recognizer.recognize(&printed).unwrap_or(false), "mutations grammar did not recognize {printed:?} (from {mutation:?})");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn diff_grammar_conformance_law() {
        let grammar = dsl::parse_grammar(diff::text::COMPONENT_GRAMMAR_SEMIO).expect("parse diff grammar");
        let recognizer = dsl::Recognizer::compile(&grammar);
        for d in diff::demo_diff_cases() {
            let printed = d.print_diff();
            assert!(recognizer.recognize(&printed).unwrap_or(false), "diff grammar did not recognize {printed:?} (from {d:?})");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn protocol_walk_law() {
        let pack_spec = dsl::parse_protocol(snapshot::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse snapshot protocol");
        let packed = store::ArtifactPack::encode_pack(&demo_las_snapshot());
        let (_, inner) = store::semio_format::unwrap_binary(&packed).expect("unwrap semio envelope");
        let trace = dsl::walk_protocol(&pack_spec, &inner).unwrap_or_else(|e| panic!("walk_protocol(pack) failed @{}: {}", e.offset, e.message));
        assert_eq!(trace.consumed, inner.len(), "pack walk did not consume every byte");

        let op_spec = dsl::parse_protocol(mutations::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse mutations protocol");
        for mutation in mutations::demo_mutation_cases() {
            let bytes = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op failed for {mutation:?}: {e:?}"));
            let trace = dsl::walk_protocol(&op_spec, &bytes).unwrap_or_else(|e| panic!("walk_protocol(op) failed for {mutation:?} @{}: {}", e.offset, e.message));
            assert_eq!(trace.consumed, bytes.len(), "op walk did not consume every byte for {mutation:?}");
        }

        let diff_spec = dsl::parse_protocol(diff::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse diff protocol");
        for d in diff::demo_diff_cases() {
            let bytes = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed for {d:?}: {e:?}"));
            let trace = dsl::walk_protocol(&diff_spec, &bytes).unwrap_or_else(|e| panic!("walk_protocol(diff) failed for {d:?} @{}: {}", e.offset, e.message));
            assert_eq!(trace.consumed, bytes.len(), "diff walk did not consume every byte for {d:?}");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn fixture_honesty_law() {
        const FIXTURE_DSL: &str = include_str!("../../../📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio");
        const FIXTURE_PACK: &[u8] = include_bytes!("../../../📚️examples/🎬️demo/🖼️assets/🎒️.pack.semio");

        let demo = demo_las_snapshot();

        let parsed = <LasSnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE_DSL).expect("parse shipped .dsl.semio fixture");
        assert_eq!(parsed, demo, "shipped .dsl.semio fixture does not parse back to demo_las_snapshot()");
        assert_eq!(store::ArtifactDsl::print_dsl(&demo), FIXTURE_DSL, "print_dsl(demo_las_snapshot()) drifted from the shipped .dsl.semio fixture");

        let decoded = <LasSnapshot as store::ArtifactPack>::decode_pack(FIXTURE_PACK).expect("decode shipped .pack.semio fixture");
        assert_eq!(decoded, demo, "shipped .pack.semio fixture does not decode back to demo_las_snapshot()");
        assert_eq!(store::ArtifactPack::encode_pack(&demo), FIXTURE_PACK, "encode_pack(demo_las_snapshot()) drifted from the shipped .pack.semio fixture");
    }
}
//#endregion 🔖️ConformanceLaws
