//! ⚙️ LasEngine — real las codec.
//!
//! Decode supports LAS point data record formats 0-3 (§LAS 1.2 point data record formats),
//! trusts the header's own `point_data_offset`/`header_size` fields as the ground truth for
//! where point data starts (no hardcoded 227-byte clamp), and falls back to the LAS 1.4
//! extended point count (offset 247, u64) when the legacy count field (offset 107) is zero.
//! Encode always emits a fixed 227-byte LAS 1.2 header — see 🚫️EncodeScopeNote below.

use crate::artifacts::las::schema::snapshot::LasPoint;
use crate::artifacts::las::{LasArtifact, LasDiff, LasMutation, LasSnapshot, STDIO_LAS_DOCUMENT_SCHEMA};

//#region 🔖️RecordLayout
/// 📏 Fixed byte width of point data record formats 0-3 (§LAS 1.2). `0` marks an
/// unsupported format.
fn point_record_min_len(fmt: u8) -> usize {
    match fmt { 0 => 20, 1 => 28, 2 => 26, 3 => 34, _ => 0 }
}
//#endregion 🔖️RecordLayout

//#region 🔖️Decode
/// 🔍 Decodes one point record at fixed byte offsets for the given point data format,
/// applying the header's scale/offset to reconstruct real-world `x/y/z`.
fn decode_point(rec: &[u8], fmt: u8, scale: (f64, f64, f64), offset: (f64, f64, f64)) -> Result<LasPoint, String> {
    let min_len = point_record_min_len(fmt);
    if min_len == 0 { return Err(format!("las: unsupported point data format {fmt}")); }
    if rec.len() < min_len { return Err("las: truncated point record".into()); }
    let xi = i32::from_le_bytes(rec[0..4].try_into().unwrap());
    let yi = i32::from_le_bytes(rec[4..8].try_into().unwrap());
    let zi = i32::from_le_bytes(rec[8..12].try_into().unwrap());
    let intensity = u16::from_le_bytes(rec[12..14].try_into().unwrap());
    let flags = rec[14];
    let classification = rec[15];
    let scan_angle_rank = rec[16] as i8;
    let user_data = rec[17];
    let point_source_id = u16::from_le_bytes(rec[18..20].try_into().unwrap());
    let (gps_time, rgb) = match fmt {
        0 => (None, None),
        1 => (Some(f64::from_le_bytes(rec[20..28].try_into().unwrap())), None),
        2 => (None, Some((
            u16::from_le_bytes(rec[20..22].try_into().unwrap()),
            u16::from_le_bytes(rec[22..24].try_into().unwrap()),
            u16::from_le_bytes(rec[24..26].try_into().unwrap()),
        ))),
        3 => (
            Some(f64::from_le_bytes(rec[20..28].try_into().unwrap())),
            Some((
                u16::from_le_bytes(rec[28..30].try_into().unwrap()),
                u16::from_le_bytes(rec[30..32].try_into().unwrap()),
                u16::from_le_bytes(rec[32..34].try_into().unwrap()),
            )),
        ),
        _ => unreachable!("validated by point_record_min_len"),
    };
    Ok(LasPoint {
        x: xi as f64 * scale.0 + offset.0,
        y: yi as f64 * scale.1 + offset.1,
        z: zi as f64 * scale.2 + offset.2,
        intensity,
        return_number: flags & 0x07,
        number_of_returns: (flags >> 3) & 0x07,
        scan_direction_flag: (flags >> 6) & 0x01 != 0,
        edge_of_flight_line: (flags >> 7) & 0x01 != 0,
        classification,
        scan_angle_rank,
        user_data,
        point_source_id,
        gps_time,
        rgb,
    })
}

/// 🔍 Decodes a full LAS binary buffer: header fields (trusting `point_data_offset` and
/// `header_size` rather than a hardcoded 227 constant) + all point records for whichever
/// of formats 0-3 the header declares.
pub fn decode_las(bytes: &[u8]) -> Result<LasSnapshot, String> {
    if bytes.len() < 111 { return Err("las: header too short".into()); }
    if &bytes[0..4] != b"LASF" { return Err("las: signature missing".into()); }
    let version_minor = bytes[25];
    let header_size = u16::from_le_bytes(bytes[94..96].try_into().map_err(|_| "las: header_size field")?) as usize;
    let point_offset = u32::from_le_bytes(bytes[96..100].try_into().map_err(|_| "las: point_data_offset field")?) as usize;
    if point_offset < header_size {
        return Err(format!("las: point_data_offset {point_offset} precedes declared header_size {header_size}"));
    }
    let point_format = bytes[104] & 0x7F; // top bit flags waveform-packet storage, irrelevant to the record layout itself
    let record_len = u16::from_le_bytes(bytes[105..107].try_into().map_err(|_| "las: point_data_record_length field")?) as usize;
    if record_len == 0 { return Err("las: point data record length is zero".into()); }
    let mut point_count = u32::from_le_bytes(bytes[107..111].try_into().map_err(|_| "las: legacy point count field")?) as u64;
    if point_count == 0 && version_minor >= 4 && bytes.len() >= 255 {
        // 🔖 LAS 1.4: legacy count of 0 means "see the extended 64-bit count" at offset 247.
        let extended = u64::from_le_bytes(bytes[247..255].try_into().map_err(|_| "las: extended point count field")?);
        if extended != 0 { point_count = extended; }
    }
    let x_scale = f64::from_le_bytes(bytes[131..139].try_into().map_err(|_| "las: x scale field")?);
    let y_scale = f64::from_le_bytes(bytes[139..147].try_into().map_err(|_| "las: y scale field")?);
    let z_scale = f64::from_le_bytes(bytes[147..155].try_into().map_err(|_| "las: z scale field")?);
    let x_off = f64::from_le_bytes(bytes[155..163].try_into().map_err(|_| "las: x offset field")?);
    let y_off = f64::from_le_bytes(bytes[163..171].try_into().map_err(|_| "las: y offset field")?);
    let z_off = f64::from_le_bytes(bytes[171..179].try_into().map_err(|_| "las: z offset field")?);
    let min_len = point_record_min_len(point_format);
    if min_len == 0 { return Err(format!("las: unsupported point data format {point_format}")); }
    if record_len < min_len {
        return Err(format!("las: record length {record_len} too small for point data format {point_format} (needs >= {min_len})"));
    }
    let mut points = Vec::with_capacity((point_count as usize).min(1_000_000));
    let mut pos = point_offset;
    for _ in 0..point_count {
        if pos + record_len > bytes.len() { break; }
        let rec = &bytes[pos..pos + record_len];
        points.push(decode_point(rec, point_format, (x_scale, y_scale, z_scale), (x_off, y_off, z_off))?);
        pos += record_len;
    }
    Ok(LasSnapshot { schema: STDIO_LAS_DOCUMENT_SCHEMA.into(), points })
}
//#endregion 🔖️Decode

//#region 🔖️Encode
/// 🚫 EncodeScopeNote: always emits a fixed 227-byte LAS 1.2 public header block (no VLRs,
/// no LAS 1.3/1.4 extensions) with `point_data_offset == header_size == 227`. The point data
/// format (0/1/2/3) is chosen per-encode based on which optional fields any point carries —
/// see `choose_point_format` — so a re-encoded LAS 1.4 source will not byte-for-byte
/// round-trip its original header, only its point content (within the modeled fields).
fn choose_point_format(points: &[LasPoint]) -> u8 {
    let has_gps = points.iter().any(|p| p.gps_time.is_some());
    let has_rgb = points.iter().any(|p| p.rgb.is_some());
    match (has_gps, has_rgb) {
        (true, true) => 3,
        (false, true) => 2,
        (true, false) => 1,
        (false, false) => 0,
    }
}

/// 🏗️ Encodes `snap` into a real LAS 1.2 binary buffer, picking point data format 0-3
/// automatically (see `choose_point_format`).
pub fn encode_las(snap: &LasSnapshot) -> Result<Vec<u8>, String> {
    let format = choose_point_format(&snap.points);
    let record_len = point_record_min_len(format) as u16;
    let header_size = 227usize;
    let count = snap.points.len();
    if count as u64 > u32::MAX as u64 {
        return Err("las: point count exceeds legacy u32 header field (LAS 1.4 extended count not implemented for encode)".into());
    }
    let mut out = vec![0u8; header_size + count * record_len as usize];
    out[0..4].copy_from_slice(b"LASF");
    out[24] = 1; // version major
    out[25] = 2; // version minor (1.2)
    out[94..96].copy_from_slice(&(header_size as u16).to_le_bytes());
    out[96..100].copy_from_slice(&(header_size as u32).to_le_bytes());
    out[104] = format;
    out[105..107].copy_from_slice(&record_len.to_le_bytes());
    out[107..111].copy_from_slice(&(count as u32).to_le_bytes());
    let scale = (0.01f64, 0.01f64, 0.01f64);
    out[131..139].copy_from_slice(&scale.0.to_le_bytes());
    out[139..147].copy_from_slice(&scale.1.to_le_bytes());
    out[147..155].copy_from_slice(&scale.2.to_le_bytes());
    // offsets (155..179) stay 0.0 — vec is zero-initialized.
    let mut pos = header_size;
    for p in &snap.points {
        let xi = (p.x / scale.0).round() as i32;
        let yi = (p.y / scale.1).round() as i32;
        let zi = (p.z / scale.2).round() as i32;
        out[pos..pos + 4].copy_from_slice(&xi.to_le_bytes());
        out[pos + 4..pos + 8].copy_from_slice(&yi.to_le_bytes());
        out[pos + 8..pos + 12].copy_from_slice(&zi.to_le_bytes());
        out[pos + 12..pos + 14].copy_from_slice(&p.intensity.to_le_bytes());
        let flags = (p.return_number & 0x07) | ((p.number_of_returns & 0x07) << 3) | ((p.scan_direction_flag as u8) << 6) | ((p.edge_of_flight_line as u8) << 7);
        out[pos + 14] = flags;
        out[pos + 15] = p.classification;
        out[pos + 16] = p.scan_angle_rank as u8;
        out[pos + 17] = p.user_data;
        out[pos + 18..pos + 20].copy_from_slice(&p.point_source_id.to_le_bytes());
        match format {
            0 => {}
            1 => out[pos + 20..pos + 28].copy_from_slice(&p.gps_time.unwrap_or(0.0).to_le_bytes()),
            2 => {
                let (r, g, b) = p.rgb.unwrap_or((0, 0, 0));
                out[pos + 20..pos + 22].copy_from_slice(&r.to_le_bytes());
                out[pos + 22..pos + 24].copy_from_slice(&g.to_le_bytes());
                out[pos + 24..pos + 26].copy_from_slice(&b.to_le_bytes());
            }
            3 => {
                out[pos + 20..pos + 28].copy_from_slice(&p.gps_time.unwrap_or(0.0).to_le_bytes());
                let (r, g, b) = p.rgb.unwrap_or((0, 0, 0));
                out[pos + 28..pos + 30].copy_from_slice(&r.to_le_bytes());
                out[pos + 30..pos + 32].copy_from_slice(&g.to_le_bytes());
                out[pos + 32..pos + 34].copy_from_slice(&b.to_le_bytes());
            }
            _ => unreachable!("choose_point_format only returns 0..=3"),
        }
        pos += record_len as usize;
    }
    Ok(out)
}

/// 🌱 Empty persisted snapshot.
pub fn empty_las_snapshot() -> LasSnapshot {
    LasSnapshot::default()
}
//#endregion 🔖️Encode

//#region 🔖️Register
/// 🗂️ Registers codecs and the artifact schema descriptor.
pub fn register() {
    crate::artifacts::las::composer::register();
    register_artifact_schema();
    register_pilot_languages();
    store::register_document_codec(store::ArtifactCodec::of::<LasSnapshot, LasMutation>(STDIO_LAS_DOCUMENT_SCHEMA));
}

/// 📌️ Registers handcrafted facet grammars (text) and protocols (las).
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.las",
        extension: Some("bin"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::las::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::las::schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::las::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::las::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.las"),
    });
}

/// 📌️ Registers schema leaves for `s.stdio.las`.
pub fn register_artifact_schema() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::las::schema::las_artifact_schema_descriptor());
}
//#endregion 🔖️Register

//#region 🔖️ArtifactEngine
/// ⚙️ `stdio.las` artifact engine.
pub struct LasEngine {
    artifact_state: LasArtifact,
    snapshot_state: LasSnapshot,
}

impl LasEngine {
    /// 🏗️ Builds an engine from a persisted snapshot.
    pub fn new(snapshot: LasSnapshot) -> Self {
        let artifact_state = LasArtifact::from_snapshot(snapshot.clone());
        Self { artifact_state, snapshot_state: snapshot }
    }
}
//#endregion 🔖️ArtifactEngine

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_snapshot_matches_schema() {
        let snapshot = empty_las_snapshot();
        assert_eq!(snapshot.schema, STDIO_LAS_DOCUMENT_SCHEMA);
    }

    #[test]
    fn codec_round_trip() {
        let snap = empty_las_snapshot();
        let text = store::ArtifactDsl::print_dsl(&snap);
        let parsed = <LasSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(parsed.schema, snap.schema);
        let bytes = store::ArtifactPack::encode_pack(&snap);
        let decoded = <LasSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(decoded, snap);
    }

    //#region 🔖️PointFixtures
    /// 🧪 7 points with varied per-field values (not all zero/default) so a naive stub that
    /// only reads x/y/z would fail these assertions on intensity/classification/flags/etc.
    fn sample_points(fmt: u8) -> Vec<LasPoint> {
        (0..7).map(|i| {
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
        }).collect()
    }

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
    //#endregion 🔖️PointFixtures

    #[test]
    fn format0_round_trip_all_fields() {
        let snap = LasSnapshot { schema: STDIO_LAS_DOCUMENT_SCHEMA.into(), points: sample_points(0) };
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
    }

    #[test]
    fn format1_round_trip_gps_time() {
        let snap = LasSnapshot { schema: STDIO_LAS_DOCUMENT_SCHEMA.into(), points: sample_points(1) };
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

    #[test]
    fn format2_round_trip_rgb() {
        let snap = LasSnapshot { schema: STDIO_LAS_DOCUMENT_SCHEMA.into(), points: sample_points(2) };
        let bytes = encode_las(&snap).expect("encode fmt2");
        assert_eq!(bytes[104], 2, "point data format byte must be 2");
        assert_eq!(u16::from_le_bytes(bytes[105..107].try_into().unwrap()), 26);
        let decoded = decode_las(&bytes).expect("decode fmt2");
        for (a, b) in snap.points.iter().zip(decoded.points.iter()) {
            assert_points_match(a, b);
            assert!(b.rgb.is_some(), "format 2 must decode rgb");
            assert_eq!(b.gps_time, None);
        }
    }

    #[test]
    fn format3_round_trip_gps_time_and_rgb() {
        let snap = LasSnapshot { schema: STDIO_LAS_DOCUMENT_SCHEMA.into(), points: sample_points(3) };
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

    #[test]
    fn point_offset_is_trusted_not_hardcoded_to_227() {
        // A header whose declared point_data_offset/header_size (200) differs from the old
        // hardcoded 227 fallback must still be honored exactly — proves decode_las trusts the
        // header fields as ground truth instead of clamping to a constant.
        let snap = LasSnapshot { schema: STDIO_LAS_DOCUMENT_SCHEMA.into(), points: sample_points(0) };
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

    #[test]
    fn las_1_4_extended_point_count_fallback() {
        // LAS 1.4: legacy count field (offset 107) is 0, real count lives in the extended
        // 64-bit field at offset 247. Build a 375-byte (LAS 1.4) header by hand.
        let snap = LasSnapshot { schema: STDIO_LAS_DOCUMENT_SCHEMA.into(), points: sample_points(0) };
        let bytes = encode_las(&snap).expect("encode");
        let header_size = 375usize;
        let mut out = vec![0u8; header_size];
        out[0..4].copy_from_slice(b"LASF");
        out[24] = 1; out[25] = 4; // version 1.4
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

    #[test]
    fn unsupported_point_format_is_rejected() {
        let snap = LasSnapshot { schema: STDIO_LAS_DOCUMENT_SCHEMA.into(), points: sample_points(0) };
        let mut bytes = encode_las(&snap).expect("encode");
        bytes[104] = 99;
        let err = decode_las(&bytes).unwrap_err();
        assert!(err.contains("unsupported point data format"), "unexpected error: {err}");
    }

    #[test]
    fn bad_signature_is_rejected() {
        let mut bytes = vec![0u8; 200];
        bytes[0..4].copy_from_slice(b"NOPE");
        let err = decode_las(&bytes).unwrap_err();
        assert!(err.contains("signature"));
    }
}
//#endregion 🧪️Tests
