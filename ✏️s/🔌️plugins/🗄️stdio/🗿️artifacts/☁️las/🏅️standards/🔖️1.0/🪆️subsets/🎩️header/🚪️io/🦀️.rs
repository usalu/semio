//! 🚪️ IO stdio.las (1.0/🎩️header) — registration now flows through 🎹️composer::register
//! (called once from 🔌️plugin/🔧️setup via ⚙️engine::register), not per-leaf register().
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v1_0::subsets::any::schema::LasAnalyzer;
    use crate::LasSnapshot;
    use semio_framework_plugin::{AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, StandardId, SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.las", standard: StandardId("1.0"), subset: SubsetId("*") };
    const DEP_BINARY: Dialect = Dialect { artifact_kind: "s.stdio.binary", standard: StandardId("raw"), subset: SubsetId("*") };

    pub struct LasComposerComposition;

    impl ArtifactComposition for LasComposerComposition {
        type Snapshot = LasSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT, DEP_BINARY]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
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
            let analysis = LasAnalyzer::analyze(&native);
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
use crate::schema::snapshot::{LasHeader, LasPoint, LasVlr};
use crate::{LasSnapshot, STDIO_LAS_DOCUMENT_SCHEMA};

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
    use crate::standards::v1_0::subsets::any::schema::LasComposer as LasRawAnyComposer;
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
