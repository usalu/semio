//! 🚪️ IO stdio.las (1.0/✳️any) — registration now flows through 🎹️composer::register
//! (called once from 🔌️plugin/🔧️setup via ⚙️engine::register), not per-leaf register().
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::artifacts::las::standards::v1_0::subsets::any::schema::LasAnalyzer;
    use crate::artifacts::las::LasSnapshot;
    use semio_framework_plugin::{AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, StandardId, SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.las", standard: StandardId("1.0"), subset: SubsetId("*") };
    const DEP_BINARY: Dialect = Dialect { artifact_kind: "s.stdio.binary", standard: StandardId("raw"), subset: SubsetId("*") };

    pub struct LasComposerComposition;

    impl ArtifactComposition for LasComposerComposition {
        type Snapshot = LasSnapshot;
        const WRITES: Dialect = DIALECT;

        async fn reads() -> &'static [Dialect] {
            &[DIALECT, DEP_BINARY]
        }

        async fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            // 🌱 Every listed read dialect's payload is raw text/bytes that this artifact's own
            // analyzer already round-trips through `store::Document{Dsl,Pack}` -- including bytes
            // claiming a dependency's dialect, since (for a single-standard DAG-adjacent dependency
            // like binary) that payload IS the same byte/text shape `analyze` already accepts.
            let native: Vec<AnalyzeSource<'_>> = sources
                .iter()
                .filter(|s| s.dialect == DIALECT || s.dialect == DEP_BINARY)
                .map(|s| match &s.payload {
                    AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                    AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
                })
                .collect();
            if native.is_empty() {
                return Err(ComposeError { message: "LasComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = LasAnalyzer::analyze(&native).await;
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "LasComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🔖️Codec
// Real las codec. Decode reads the full LAS 1.0 public header block (§2.3: version, system
// identifier, generating software, creation date, header size, offset to point data, VLR count,
// point data format id + record length, point record count, points-by-return histogram,
// scale/offset, max/min bounds), walks `number_of_vlrs` Variable Length Records starting at
// `header_size` (payload retained byte-verbatim), then decodes point data record formats 0-3
// (§LAS 1.2). Trusts the header's own `offset_to_point_data`/`header_size` fields as ground
// truth for where VLRs/point data start (no hardcoded 227-byte clamp), and falls back to the
// LAS 1.4 extended point count (offset 247, u64) when the legacy count field (offset 107) is
// zero. Encode always emits a fixed 227-byte header — see 🚫️EncodeScopeNote below.
use crate::artifacts::las::schema::snapshot::{LasHeader, LasPoint, LasVlr};
use crate::artifacts::las::{LasSnapshot, STDIO_LAS_DOCUMENT_SCHEMA};

//#region 🔖️ByteHelpers
/// 🔍 Reads a null/space-padded fixed-width ASCII field, trimmed of trailing padding.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_fixed_str(bytes: &[u8]) -> String {
    let end = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
    String::from_utf8_lossy(&bytes[..end]).trim_end().to_string()
}

/// 🏗️ Writes `s` into a fixed-width field, truncated to `buf.len()` bytes, the rest left as
/// whatever `buf` already held (callers zero-init the output buffer up front).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_fixed_str(buf: &mut [u8], s: &str) {
    let bytes = s.as_bytes();
    let n = bytes.len().min(buf.len());
    buf[..n].copy_from_slice(&bytes[..n]);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_u16(bytes: &[u8], off: usize) -> Result<u16, String> {
    bytes.get(off..off + 2).and_then(|s| s.try_into().ok()).map(u16::from_le_bytes).ok_or_else(|| format!("las: truncated u16 at offset {off}"))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_u32(bytes: &[u8], off: usize) -> Result<u32, String> {
    bytes.get(off..off + 4).and_then(|s| s.try_into().ok()).map(u32::from_le_bytes).ok_or_else(|| format!("las: truncated u32 at offset {off}"))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_u64(bytes: &[u8], off: usize) -> Result<u64, String> {
    bytes.get(off..off + 8).and_then(|s| s.try_into().ok()).map(u64::from_le_bytes).ok_or_else(|| format!("las: truncated u64 at offset {off}"))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_f64(bytes: &[u8], off: usize) -> Result<f64, String> {
    bytes.get(off..off + 8).and_then(|s| s.try_into().ok()).map(f64::from_le_bytes).ok_or_else(|| format!("las: truncated f64 at offset {off}"))
}
//#endregion 🔖️ByteHelpers

//#region 🔖️RecordLayout
/// 📏 Fixed byte width of point data record formats 0-3 (§LAS 1.2). `0` marks an
/// unsupported format.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn point_record_min_len(fmt: u8) -> usize {
    match fmt {
        0 => 20,
        1 => 28,
        2 => 26,
        3 => 34,
        _ => 0,
    }
}
//#endregion 🔖️RecordLayout

//#region 🔖️HeaderLayout
/// 📐 Public header block byte offsets (§LAS 1.0/1.2, 227-byte header). The 20-byte span
/// [4..24) (file source id, global encoding, project id GUID) is spec-real but out of this
/// wave's contracted field list — skipped on decode (never indexed), left zero on encode.
mod off {
    pub const VERSION_MAJOR: usize = 24;
    pub const VERSION_MINOR: usize = 25;
    pub const SYSTEM_IDENTIFIER: std::ops::Range<usize> = 26..58;
    pub const GENERATING_SOFTWARE: std::ops::Range<usize> = 58..90;
    pub const CREATION_DAY: usize = 90;
    pub const CREATION_YEAR: usize = 92;
    pub const HEADER_SIZE: usize = 94;
    pub const OFFSET_TO_POINT_DATA: usize = 96;
    pub const NUMBER_OF_VLRS: usize = 100;
    pub const POINT_DATA_FORMAT_ID: usize = 104;
    pub const POINT_DATA_RECORD_LENGTH: usize = 105;
    pub const NUMBER_OF_POINT_RECORDS: usize = 107;
    pub const POINTS_BY_RETURN: usize = 111; // 5x u32, 111..131
    pub const X_SCALE: usize = 131;
    pub const Y_SCALE: usize = 139;
    pub const Z_SCALE: usize = 147;
    pub const X_OFFSET: usize = 155;
    pub const Y_OFFSET: usize = 163;
    pub const Z_OFFSET: usize = 171;
    pub const MAX_X: usize = 179;
    pub const MIN_X: usize = 187;
    pub const MAX_Y: usize = 195;
    pub const MIN_Y: usize = 203;
    pub const MAX_Z: usize = 211;
    pub const MIN_Z: usize = 219;
    pub const FIXED_HEADER_LEN: usize = 227;
    pub const EXTENDED_POINT_COUNT: usize = 247; // LAS 1.4 only, u64
}

/// 📐 VLR header byte layout (§2.4, 54-byte header + `record_length_after_header` data bytes).
mod vlr_off {
    pub const RESERVED: usize = 0;
    pub const USER_ID: std::ops::Range<usize> = 2..18;
    pub const RECORD_ID: usize = 18;
    pub const RECORD_LENGTH: usize = 20;
    pub const DESCRIPTION: std::ops::Range<usize> = 22..54;
    pub const HEADER_LEN: usize = 54;
}
//#endregion 🔖️HeaderLayout

//#region 🔖️Decode
/// 🔍 Decodes one point record at fixed byte offsets for the given point data format,
/// applying the header's scale/offset to reconstruct real-world `x/y/z`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_point(rec: &[u8], fmt: u8, scale: (f64, f64, f64), offset: (f64, f64, f64)) -> Result<LasPoint, String> {
    let min_len = point_record_min_len(fmt);
    if min_len == 0 {
        return Err(format!("las: unsupported point data format {fmt}"));
    }
    if rec.len() < min_len {
        return Err("las: truncated point record".into());
    }
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
        2 => (None, Some((u16::from_le_bytes(rec[20..22].try_into().unwrap()), u16::from_le_bytes(rec[22..24].try_into().unwrap()), u16::from_le_bytes(rec[24..26].try_into().unwrap())))),
        3 => (Some(f64::from_le_bytes(rec[20..28].try_into().unwrap())), Some((u16::from_le_bytes(rec[28..30].try_into().unwrap()), u16::from_le_bytes(rec[30..32].try_into().unwrap()), u16::from_le_bytes(rec[32..34].try_into().unwrap())))),
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

/// 🔍 Decodes `number_of_vlrs` Variable Length Records starting at `header_size`, bounded by
/// `point_offset` (graceful truncation — never reads past either boundary or `bytes.len()`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_vlrs(bytes: &[u8], header_size: usize, point_offset: usize, number_of_vlrs: u32) -> Vec<LasVlr> {
    let mut vlrs = Vec::with_capacity((number_of_vlrs as usize).min(10_000));
    let mut pos = header_size;
    for _ in 0..number_of_vlrs {
        if pos + vlr_off::HEADER_LEN > bytes.len() || pos + vlr_off::HEADER_LEN > point_offset {
            break;
        }
        let user_id = read_fixed_str(&bytes[pos + vlr_off::USER_ID.start..pos + vlr_off::USER_ID.end]);
        let record_id = match read_u16(bytes, pos + vlr_off::RECORD_ID) {
            Ok(v) => v,
            Err(_) => break,
        };
        let data_len = match read_u16(bytes, pos + vlr_off::RECORD_LENGTH) {
            Ok(v) => v as usize,
            Err(_) => break,
        };
        let description = read_fixed_str(&bytes[pos + vlr_off::DESCRIPTION.start..pos + vlr_off::DESCRIPTION.end]);
        let data_start = pos + vlr_off::HEADER_LEN;
        let data_end = data_start + data_len;
        if data_end > bytes.len() || data_end > point_offset {
            break;
        }
        let data = bytes[data_start..data_end].to_vec();
        vlrs.push(LasVlr { user_id, record_id, description, data });
        pos = data_end;
    }
    vlrs
}

/// 🔍 Decodes a full LAS binary buffer: header fields (trusting `offset_to_point_data` and
/// `header_size` rather than a hardcoded 227 constant) + VLRs + all point records for whichever
/// of formats 0-3 the header declares.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_las(bytes: &[u8]) -> Result<LasSnapshot, String> {
    if bytes.len() < 4 || &bytes[0..4] != b"LASF" {
        return Err("las: signature missing".into());
    }
    if bytes.len() < off::FIXED_HEADER_LEN {
        return Err("las: header too short".into());
    }

    let version_major = bytes[off::VERSION_MAJOR];
    let version_minor = bytes[off::VERSION_MINOR];
    let system_identifier = read_fixed_str(&bytes[off::SYSTEM_IDENTIFIER]);
    let generating_software = read_fixed_str(&bytes[off::GENERATING_SOFTWARE]);
    let creation_day_of_year = read_u16(bytes, off::CREATION_DAY)?;
    let creation_year = read_u16(bytes, off::CREATION_YEAR)?;
    let header_size = read_u16(bytes, off::HEADER_SIZE)? as usize;
    let offset_to_point_data = read_u32(bytes, off::OFFSET_TO_POINT_DATA)?;
    let point_offset = offset_to_point_data as usize;
    if point_offset < header_size {
        return Err(format!("las: offset_to_point_data {point_offset} precedes declared header_size {header_size}"));
    }
    let number_of_vlrs = read_u32(bytes, off::NUMBER_OF_VLRS)?;
    let point_format = bytes[off::POINT_DATA_FORMAT_ID] & 0x7F; // top bit flags waveform-packet storage, irrelevant to the record layout itself
    let record_len = read_u16(bytes, off::POINT_DATA_RECORD_LENGTH)? as usize;
    if record_len == 0 {
        return Err("las: point data record length is zero".into());
    }
    let legacy_count = read_u32(bytes, off::NUMBER_OF_POINT_RECORDS)?;
    let mut point_count = legacy_count as u64;
    if point_count == 0 && version_minor >= 4 && bytes.len() >= off::EXTENDED_POINT_COUNT + 8 {
        // 🔖 LAS 1.4: legacy count of 0 means "see the extended 64-bit count".
        let extended = read_u64(bytes, off::EXTENDED_POINT_COUNT)?;
        if extended != 0 {
            point_count = extended;
        }
    }
    let mut points_by_return = [0u32; 5];
    for (i, slot) in points_by_return.iter_mut().enumerate() {
        *slot = read_u32(bytes, off::POINTS_BY_RETURN + i * 4)?;
    }
    let x_scale = read_f64(bytes, off::X_SCALE)?;
    let y_scale = read_f64(bytes, off::Y_SCALE)?;
    let z_scale = read_f64(bytes, off::Z_SCALE)?;
    let x_offset = read_f64(bytes, off::X_OFFSET)?;
    let y_offset = read_f64(bytes, off::Y_OFFSET)?;
    let z_offset = read_f64(bytes, off::Z_OFFSET)?;
    let max_x = read_f64(bytes, off::MAX_X)?;
    let min_x = read_f64(bytes, off::MIN_X)?;
    let max_y = read_f64(bytes, off::MAX_Y)?;
    let min_y = read_f64(bytes, off::MIN_Y)?;
    let max_z = read_f64(bytes, off::MAX_Z)?;
    let min_z = read_f64(bytes, off::MIN_Z)?;

    let min_len = point_record_min_len(point_format);
    if min_len == 0 {
        return Err(format!("las: unsupported point data format {point_format}"));
    }
    if record_len < min_len {
        return Err(format!("las: record length {record_len} too small for point data format {point_format} (needs >= {min_len})"));
    }

    let vlrs = decode_vlrs(bytes, header_size, point_offset, number_of_vlrs);

    let mut points = Vec::with_capacity((point_count as usize).min(1_000_000));
    let mut pos = point_offset;
    for _ in 0..point_count {
        if pos + record_len > bytes.len() {
            break;
        }
        let rec = &bytes[pos..pos + record_len];
        points.push(decode_point(rec, point_format, (x_scale, y_scale, z_scale), (x_offset, y_offset, z_offset))?);
        pos += record_len;
    }

    let header = LasHeader {
        version_major,
        version_minor,
        system_identifier,
        generating_software,
        creation_day_of_year,
        creation_year,
        header_size: header_size as u16,
        offset_to_point_data,
        number_of_vlrs,
        point_data_format_id: point_format,
        point_data_record_length: record_len as u16,
        number_of_point_records: point_count.min(u32::MAX as u64) as u32,
        points_by_return,
        x_scale,
        y_scale,
        z_scale,
        x_offset,
        y_offset,
        z_offset,
        max_x,
        min_x,
        max_y,
        min_y,
        max_z,
        min_z,
    };

    Ok(LasSnapshot { schema: STDIO_LAS_DOCUMENT_SCHEMA.into(), header, vlrs, points })
}
//#endregion 🔖️Decode

//#region 🔖️Encode
/// 🚫 EncodeScopeNote: always emits a fixed 227-byte public header block (no LAS 1.3/1.4
/// extensions), with `header_size`, `offset_to_point_data` (`227 + Σ(54 + vlr.data.len())`),
/// `number_of_vlrs` (`== vlrs.len()`), `point_data_format_id` (chosen per-encode from which
/// optional point fields any point carries — see `choose_point_format`), `point_data_record_length`
/// and `number_of_point_records` (`== points.len()`) ALWAYS recomputed from the real `vlrs`/
/// `points` content — never trusted verbatim from `snap.header`, since these six fields are
/// structural. Every other header field is written verbatim from `snap.header`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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

/// 🏗️ Encodes `snap` into a real LAS binary buffer: header + VLRs + point records, point data
/// format 0-3 chosen automatically (see `choose_point_format`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_las(snap: &LasSnapshot) -> Result<Vec<u8>, String> {
    let format = choose_point_format(&snap.points);
    let record_len = point_record_min_len(format) as u16;
    let header_size = off::FIXED_HEADER_LEN;
    let point_count = snap.points.len();
    if point_count as u64 > u32::MAX as u64 {
        return Err("las: point count exceeds legacy u32 header field (LAS 1.4 extended count not implemented for encode)".into());
    }
    if snap.vlrs.len() as u64 > u32::MAX as u64 {
        return Err("las: vlr count exceeds u32 header field".into());
    }
    for v in &snap.vlrs {
        if v.data.len() > u16::MAX as usize {
            return Err("las: vlr data exceeds u16 record_length_after_header field".into());
        }
    }
    let vlr_bytes: usize = snap.vlrs.iter().map(|v| vlr_off::HEADER_LEN + v.data.len()).sum();
    let offset_to_point_data = header_size + vlr_bytes;
    let total_len = offset_to_point_data + point_count * record_len as usize;
    let mut out = vec![0u8; total_len];

    out[0..4].copy_from_slice(b"LASF");
    out[off::VERSION_MAJOR] = snap.header.version_major;
    out[off::VERSION_MINOR] = snap.header.version_minor;
    write_fixed_str(&mut out[off::SYSTEM_IDENTIFIER], &snap.header.system_identifier);
    write_fixed_str(&mut out[off::GENERATING_SOFTWARE], &snap.header.generating_software);
    out[off::CREATION_DAY..off::CREATION_DAY + 2].copy_from_slice(&snap.header.creation_day_of_year.to_le_bytes());
    out[off::CREATION_YEAR..off::CREATION_YEAR + 2].copy_from_slice(&snap.header.creation_year.to_le_bytes());
    out[off::HEADER_SIZE..off::HEADER_SIZE + 2].copy_from_slice(&(header_size as u16).to_le_bytes());
    out[off::OFFSET_TO_POINT_DATA..off::OFFSET_TO_POINT_DATA + 4].copy_from_slice(&(offset_to_point_data as u32).to_le_bytes());
    out[off::NUMBER_OF_VLRS..off::NUMBER_OF_VLRS + 4].copy_from_slice(&(snap.vlrs.len() as u32).to_le_bytes());
    out[off::POINT_DATA_FORMAT_ID] = format;
    out[off::POINT_DATA_RECORD_LENGTH..off::POINT_DATA_RECORD_LENGTH + 2].copy_from_slice(&record_len.to_le_bytes());
    out[off::NUMBER_OF_POINT_RECORDS..off::NUMBER_OF_POINT_RECORDS + 4].copy_from_slice(&(point_count as u32).to_le_bytes());
    for (i, count) in snap.header.points_by_return.iter().enumerate() {
        out[off::POINTS_BY_RETURN + i * 4..off::POINTS_BY_RETURN + i * 4 + 4].copy_from_slice(&count.to_le_bytes());
    }
    out[off::X_SCALE..off::X_SCALE + 8].copy_from_slice(&snap.header.x_scale.to_le_bytes());
    out[off::Y_SCALE..off::Y_SCALE + 8].copy_from_slice(&snap.header.y_scale.to_le_bytes());
    out[off::Z_SCALE..off::Z_SCALE + 8].copy_from_slice(&snap.header.z_scale.to_le_bytes());
    out[off::X_OFFSET..off::X_OFFSET + 8].copy_from_slice(&snap.header.x_offset.to_le_bytes());
    out[off::Y_OFFSET..off::Y_OFFSET + 8].copy_from_slice(&snap.header.y_offset.to_le_bytes());
    out[off::Z_OFFSET..off::Z_OFFSET + 8].copy_from_slice(&snap.header.z_offset.to_le_bytes());
    out[off::MAX_X..off::MAX_X + 8].copy_from_slice(&snap.header.max_x.to_le_bytes());
    out[off::MIN_X..off::MIN_X + 8].copy_from_slice(&snap.header.min_x.to_le_bytes());
    out[off::MAX_Y..off::MAX_Y + 8].copy_from_slice(&snap.header.max_y.to_le_bytes());
    out[off::MIN_Y..off::MIN_Y + 8].copy_from_slice(&snap.header.min_y.to_le_bytes());
    out[off::MAX_Z..off::MAX_Z + 8].copy_from_slice(&snap.header.max_z.to_le_bytes());
    out[off::MIN_Z..off::MIN_Z + 8].copy_from_slice(&snap.header.min_z.to_le_bytes());

    let mut pos = header_size;
    for v in &snap.vlrs {
        out[pos + vlr_off::RESERVED..pos + vlr_off::RESERVED + 2].copy_from_slice(&0u16.to_le_bytes());
        write_fixed_str(&mut out[pos + vlr_off::USER_ID.start..pos + vlr_off::USER_ID.end], &v.user_id);
        out[pos + vlr_off::RECORD_ID..pos + vlr_off::RECORD_ID + 2].copy_from_slice(&v.record_id.to_le_bytes());
        out[pos + vlr_off::RECORD_LENGTH..pos + vlr_off::RECORD_LENGTH + 2].copy_from_slice(&(v.data.len() as u16).to_le_bytes());
        write_fixed_str(&mut out[pos + vlr_off::DESCRIPTION.start..pos + vlr_off::DESCRIPTION.end], &v.description);
        let data_start = pos + vlr_off::HEADER_LEN;
        out[data_start..data_start + v.data.len()].copy_from_slice(&v.data);
        pos = data_start + v.data.len();
    }
    debug_assert_eq!(pos, offset_to_point_data, "vlr walk must land exactly on offset_to_point_data");

    let mut pos = offset_to_point_data;
    for p in &snap.points {
        let xi = ((p.x - snap.header.x_offset) / snap.header.x_scale).round() as i32;
        let yi = ((p.y - snap.header.y_offset) / snap.header.y_scale).round() as i32;
        let zi = ((p.z - snap.header.z_offset) / snap.header.z_scale).round() as i32;
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
//#endregion 🔖️Encode
//#endregion 🔖️Codec

//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::artifacts::las::standards::v1_0::subsets::any::schema::LasComposer as LasRawAnyComposer;
    use semio_framework_plugin::{composer_entry_of, ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<LasRawAnyComposer>()]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::las::schema::{demo_las_snapshot, empty_las_snapshot};

    #[semio_framework_async_macros::async_test]
    async fn empty_snapshot_matches_schema() {
        let snapshot = empty_las_snapshot();
        assert_eq!(snapshot.schema, STDIO_LAS_DOCUMENT_SCHEMA);
    }

    #[semio_framework_async_macros::async_test]
    async fn codec_round_trip() {
        let snap = empty_las_snapshot();
        let text = store::ArtifactDsl::print_dsl(&snap);
        let parsed = <LasSnapshot as store::ArtifactDsl>::parse_dsl(&text).await.expect("parse");
        assert_eq!(parsed.schema, snap.schema);
        let bytes = store::ArtifactPack::encode_pack(&snap);
        let decoded = <LasSnapshot as store::ArtifactPack>::decode_pack(&bytes).await.expect("decode");
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
        let parsed = <LasSnapshot as store::ArtifactDsl>::parse_dsl(&text).await.expect("parse");
        assert_eq!(parsed, snap);
        let bytes = store::ArtifactPack::encode_pack(&snap);
        let decoded = <LasSnapshot as store::ArtifactPack>::decode_pack(&bytes).await.expect("decode");
        assert_eq!(decoded, snap);
    }

    //#region 🔖️ConformanceLaws
    /// 🧪️ Per-artifact conformance laws — grammar/protocol parseability, `Recognizer` against
    /// real fixtures AND real `print_op`/`print_diff` output, `walk_protocol` against real
    /// `encode_pack`/`encode_op`/`encode_diff` bytes, and the fixture-honesty round-trip.
    mod conformance_laws {
        use super::*;
        use crate::artifacts::las::schema::{diff, mutations, snapshot};
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
                let bytes = mutation.encode_op().await.unwrap_or_else(|e| panic!("encode_op failed for {mutation:?}: {e:?}"));
                let trace = dsl::walk_protocol(&op_spec, &bytes).unwrap_or_else(|e| panic!("walk_protocol(op) failed for {mutation:?} @{}: {}", e.offset, e.message));
                assert_eq!(trace.consumed, bytes.len(), "op walk did not consume every byte for {mutation:?}");
            }

            let diff_spec = dsl::parse_protocol(diff::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse diff protocol");
            for d in diff::demo_diff_cases() {
                let bytes = d.encode_diff().await.unwrap_or_else(|e| panic!("encode_diff failed for {d:?}: {e:?}"));
                let trace = dsl::walk_protocol(&diff_spec, &bytes).unwrap_or_else(|e| panic!("walk_protocol(diff) failed for {d:?} @{}: {}", e.offset, e.message));
                assert_eq!(trace.consumed, bytes.len(), "diff walk did not consume every byte for {d:?}");
            }
        }

        #[semio_framework_async_macros::async_test]
        async fn fixture_honesty_law() {
            const FIXTURE_DSL: &str = include_str!("../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");
            const FIXTURE_PACK: &[u8] = include_bytes!("../📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio");

            let demo = demo_las_snapshot();

            let parsed = <LasSnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE_DSL).await.expect("parse shipped .dsl.semio fixture");
            assert_eq!(parsed, demo, "shipped .dsl.semio fixture does not parse back to demo_las_snapshot()");
            assert_eq!(store::ArtifactDsl::print_dsl(&demo), FIXTURE_DSL, "print_dsl(demo_las_snapshot()) drifted from the shipped .dsl.semio fixture");

            let decoded = <LasSnapshot as store::ArtifactPack>::decode_pack(FIXTURE_PACK).await.expect("decode shipped .pack.semio fixture");
            assert_eq!(decoded, demo, "shipped .pack.semio fixture does not decode back to demo_las_snapshot()");
            assert_eq!(store::ArtifactPack::encode_pack(&demo), FIXTURE_PACK, "encode_pack(demo_las_snapshot()) drifted from the shipped .pack.semio fixture");
        }
    }
    //#endregion 🔖️ConformanceLaws
}
//#endregion 🧪️Tests
