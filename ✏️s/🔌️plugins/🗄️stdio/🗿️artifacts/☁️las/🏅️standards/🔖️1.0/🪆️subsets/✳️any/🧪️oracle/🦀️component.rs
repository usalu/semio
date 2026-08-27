//! 🔮️ Mutation oracle for this subset — every mutation kind the subset declares, performed by the
//! registered `las` 0.11 reference implementation so the subject's own mutation has an independent
//! result to be compared against instead of being checked against its own reading.
//!
//! The vocabulary is per SUBSET, not per artifact: two standards of the same format declare
//! different mutations, and a subset that shares an implementation with another reaches it through
//! the shared family modules rather than by copying it. No shared "point cloud" family module
//! exists (LAS is currently the only artifact in that domain), so the independent reader/writer
//! below lives here rather than under `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/`.
//!
//! **Raw types, not the façade**: every dispatch arm below works against `las::raw::{Header, Vlr,
//! Point}` — the crate's byte-exact typed mirror of the LAS public header block / VLR / point-data-
//! format-0-3 records — never `las::{Reader, Writer, Builder}`. The friendlier façade auto-
//! recomputes `bounds`/`points_by_return` from whatever points are actually written, with no public
//! way to override either, while `../🧬️schema/📸️snapshot/🦀️component.rs`'s `LasHeader` treats both
//! as directly retained, non-structural content (its own doc comment: "real-world LAS files
//! frequently carry an inaccurate one; honest retention beats silent correction") — so `SetBounds`/
//! `SetPointsByReturn` need to set an arbitrary value independent of the real point distribution to
//! be a checked mutation rather than a no-op. The raw types give exactly that control while still
//! being the crate's own (de)serialization.
//!
//! **Version stays within 1.0-1.2**: `las::raw::Header::write_to` appends waveform/EVLR/large-file
//! extensions once `self.version.supports::<_>()` reports true for a 1.3+ minor, growing the header
//! past the fixed 227 bytes `../🚪️io/🦀️component.rs`'s `encode_las` always emits (its own
//! `EncodeScopeNote`: "no LAS 1.3/1.4 extensions"). `SetVersion` scenarios are kept to the 1.0-1.2
//! family so this is a real, in-scope version bump on both sides, not an accidental exercise of a
//! subject capability gap under a version-label mutation.
//!
//! **A real spec detail neither side needs**: LAS 1.0 files may carry a 2-byte `[0xCC, 0xDD]`
//! "point data start signature" between the VLRs and the point records (`las::raw::
//! POINT_DATA_START_SIGNATURE`, `Version::requires_point_data_start_signature`) — the real
//! committed fixture, itself written by this same crate's façade, carries it. Neither this oracle's
//! `raw_doc::write` nor the subject's `encode_las` emits it (both always place point data
//! immediately after the last VLR), which is why a genuine decode → re-encode never reproduces the
//! input byte-for-byte even with no mutation applied. This costs nothing semantically: the
//! signature carries no field value, and `offsetToPointData` (the only field it would shift) is
//! itself STRUCTURAL and excluded from the projection below.
//!
//! @see ../🧪️oracle/🔣️.json — the mutation catalog this module is measured against.
//! @see ../🧬️schema/🧬️mutations/🦀️component.rs — the mutation vocabulary itself.

use semio_repo_test_host::Json;

//#region 🔖️RawDoc
/// 🧬️ Independent read/write of the LAS public header block + VLRs + point records behind
/// `las::raw::{Header, Vlr, Point}`, shared by every dispatch arm below.
#[cfg(feature = "oracles")]
mod raw_doc {
    use las::point::Format;
    use las::raw::vlr::RecordLength;
    use las::raw::{Header, Point, Vlr};
    use std::io::Cursor;

    pub(super) struct RawDoc {
        pub header: Header,
        pub vlrs: Vec<Vlr>,
        pub points: Vec<Point>,
    }

    /// 🧬️ `las::raw::Vlr` does not derive `Clone` (unlike `Header`/`Point`); every field is `Copy`
    /// except `data`.
    pub(super) fn clone_vlr(vlr: &Vlr) -> Vlr {
        Vlr { reserved: vlr.reserved, user_id: vlr.user_id, record_id: vlr.record_id, record_length_after_header: vlr.record_length_after_header, description: vlr.description, data: vlr.data.clone() }
    }

    /// 🔍 Reads a real LAS buffer into its typed parts, trusting the header's own
    /// `offsetToPointData` (not sequential continuation after the VLRs) to locate the point
    /// records — the same "ground truth, not a hardcoded clamp" rule `../../🚪️io/🦀️component.rs`'s
    /// own `decode_las` follows, and what makes the point-data-start-signature gap (this file's top
    /// doc comment) a non-issue for reading.
    pub(super) fn read(bytes: &[u8]) -> Result<RawDoc, String> {
        let mut cursor = Cursor::new(bytes);
        let header = Header::read_from(&mut cursor).map_err(|error| format!("las header read: {error}"))?;
        let mut vlrs = Vec::with_capacity((header.number_of_variable_length_records as usize).min(10_000));
        for _ in 0..header.number_of_variable_length_records {
            vlrs.push(Vlr::read_from(&mut cursor, false).map_err(|error| format!("las vlr read: {error}"))?);
        }
        let format = Format::new(header.point_data_record_format).map_err(|error| format!("las point format: {error}"))?;
        let point_offset = header.offset_to_point_data as usize;
        let point_bytes = bytes.get(point_offset..).ok_or_else(|| "las: offsetToPointData past end of file".to_string())?;
        let mut point_cursor = Cursor::new(point_bytes);
        let count = (header.number_of_point_records as usize).min(1_000_000);
        let mut points = Vec::with_capacity(count);
        for _ in 0..header.number_of_point_records {
            points.push(Point::read_from(&mut point_cursor, &format).map_err(|error| format!("las point read: {error}"))?);
        }
        Ok(RawDoc { header, vlrs, points })
    }

    /// 🎯️ Mirrors `../../🚪️io/🦀️component.rs`'s `choose_point_format` exactly: point format chosen
    /// from whichever of gps-time/rgb the real points actually carry.
    fn choose_format(points: &[Point]) -> u8 {
        let has_gps_time = points.iter().any(|point| point.gps_time.is_some());
        let has_color = points.iter().any(|point| point.color.is_some());
        match (has_gps_time, has_color) {
            (true, true) => 3,
            (false, true) => 2,
            (true, false) => 1,
            (false, false) => 0,
        }
    }

    /// 🏗️ Always recomputes the six STRUCTURAL fields from the real `vlrs`/`points` content —
    /// `headerSize` fixed at 227 (this file's top doc comment: no LAS 1.3/1.4 extensions),
    /// `offsetToPointData` (`227 + Σ(54 + vlr.data.len())`, no point-data-start-signature padding —
    /// see this file's top doc comment), `numberOfVariableLengthRecords`, `pointDataRecordFormat`/
    /// `pointDataRecordLength` (`choose_format`), `numberOfPointRecords` — mirroring `encode_las`'s
    /// own structural/non-structural split field-for-field. Every other header field is written
    /// verbatim from `doc.header`.
    pub(super) fn write(doc: &RawDoc) -> Result<Vec<u8>, String> {
        let mut header = doc.header.clone();
        let format_id = choose_format(&doc.points);
        let format = Format::new(format_id).map_err(|error| format!("las point format: {error}"))?;
        header.point_data_record_format = format_id;
        header.point_data_record_length = format.len();
        header.number_of_variable_length_records = doc.vlrs.len() as u32;
        header.number_of_point_records = doc.points.len() as u32;
        header.header_size = 227;
        let vlr_bytes: u32 = doc.vlrs.iter().map(|vlr| 54 + vlr.data.len() as u32).sum();
        header.offset_to_point_data = 227 + vlr_bytes;

        let mut out = Vec::new();
        header.write_to(&mut out).map_err(|error| format!("las header write: {error}"))?;
        for vlr in &doc.vlrs {
            let mut vlr = clone_vlr(vlr);
            vlr.record_length_after_header = RecordLength::Vlr(vlr.data.len() as u16);
            vlr.write_to(&mut out).map_err(|error| format!("las vlr write: {error}"))?;
        }
        for point in &doc.points {
            point.write_to(&mut out, &format).map_err(|error| format!("las point write: {error}"))?;
        }
        Ok(out)
    }
}
//#endregion 🔖️RawDoc

//#region 🔖️FixedStr
/// 🔤️ Null/space-padded fixed-width ASCII field helpers, matching `../../🚪️io/🦀️component.rs`'s
/// own `read_fixed_str`/`write_fixed_str` exactly (same LAS §2.3 convention, independently applied
/// here to `las::raw::Header`'s `[u8; 16]`/`[u8; 32]` arrays rather than to this subset's own
/// snapshot).
#[cfg(feature = "oracles")]
fn read_fixed_str(bytes: &[u8]) -> String {
    let end = bytes.iter().position(|&byte| byte == 0).unwrap_or(bytes.len());
    String::from_utf8_lossy(&bytes[..end]).trim_end().to_string()
}
#[cfg(feature = "oracles")]
fn write_fixed<const N: usize>(text: &str) -> [u8; N] {
    let mut buf = [0u8; N];
    let bytes = text.as_bytes();
    let n = bytes.len().min(N);
    buf[..n].copy_from_slice(&bytes[..n]);
    buf
}
//#endregion 🔖️FixedStr

//#region 🔖️JsonReaders
#[cfg(feature = "oracles")]
fn mutation_params(spec: &Json) -> Json {
    spec.get("params").cloned().unwrap_or(Json::Null)
}
#[cfg(feature = "oracles")]
fn number(value: &Json, key: &str) -> Option<f64> {
    match value.get(key) {
        Some(Json::Number(number)) => Some(*number),
        _ => None,
    }
}
#[cfg(feature = "oracles")]
fn string(value: &Json, key: &str) -> Option<String> {
    match value.get(key) {
        Some(Json::String(text)) => Some(text.clone()),
        _ => None,
    }
}
#[cfg(feature = "oracles")]
fn bool_of(value: &Json, key: &str) -> Option<bool> {
    match value.get(key) {
        Some(Json::Bool(flag)) => Some(*flag),
        _ => None,
    }
}
#[cfg(feature = "oracles")]
fn f64x3(value: &Json, key: &str) -> Option<(f64, f64, f64)> {
    match value.get(key) {
        Some(Json::Array(items)) if items.len() == 3 => {
            let at = |index: usize| match &items[index] {
                Json::Number(number) => Some(*number),
                _ => None,
            };
            Some((at(0)?, at(1)?, at(2)?))
        }
        _ => None,
    }
}
#[cfg(feature = "oracles")]
fn u32x5(value: &Json, key: &str) -> Option<[u32; 5]> {
    match value.get(key) {
        Some(Json::Array(items)) if items.len() == 5 => {
            let mut out = [0u32; 5];
            for (slot, item) in out.iter_mut().zip(items.iter()) {
                *slot = match item {
                    Json::Number(number) => *number as u32,
                    _ => return None,
                };
            }
            Some(out)
        }
        _ => None,
    }
}
#[cfg(feature = "oracles")]
fn spec_of(kind: &str, params: Json) -> Json {
    Json::Object(vec![("kind".to_string(), Json::String(kind.to_string())), ("params".to_string(), params)])
}
//#endregion 🔖️JsonReaders

//#region 🔖️VlrJson
#[cfg(feature = "oracles")]
fn vlr_to_json(vlr: &las::raw::Vlr) -> Json {
    Json::Object(vec![
        ("userId".to_string(), Json::String(read_fixed_str(&vlr.user_id))),
        ("recordId".to_string(), Json::Number(vlr.record_id as f64)),
        ("description".to_string(), Json::String(read_fixed_str(&vlr.description))),
        ("data".to_string(), Json::String(String::from_utf8_lossy(&vlr.data).to_string())),
    ])
}
#[cfg(feature = "oracles")]
fn vlr_of_json(value: &Json) -> Option<las::raw::Vlr> {
    let data = string(value, "data")?.into_bytes();
    Some(las::raw::Vlr {
        reserved: 0,
        user_id: write_fixed(&string(value, "userId")?),
        record_id: number(value, "recordId")? as u16,
        record_length_after_header: las::raw::vlr::RecordLength::Vlr(data.len() as u16),
        description: write_fixed(&string(value, "description")?),
        data,
    })
}
//#endregion 🔖️VlrJson

//#region 🔖️PointJson
/// 🧭️ `flags`/`classification` are stored as `Flags::TwoByte(a, b)` for point formats 0-5 — `a` the
/// return/direction/edge bitfield byte, `b` the classification byte VERBATIM (LAS §point data record
/// formats 0-3 never split synthetic/key-point/withheld out of it), the exact same union
/// `../../🚪️io/🦀️component.rs`'s `LasPoint.classification: u8` already models — so no decomposition
/// is needed on either side of this conversion.
#[cfg(feature = "oracles")]
fn point_to_json(point: &las::raw::Point, header: &las::raw::Header) -> Json {
    let (a, b) = point.flags.to_two_bytes().unwrap_or((0, 0));
    let scan_angle_rank = match point.scan_angle {
        las::raw::point::ScanAngle::Rank(rank) => rank as f64,
        las::raw::point::ScanAngle::Scaled(scaled) => scaled as f64,
    };
    Json::Object(vec![
        ("x".to_string(), Json::Number(point.x as f64 * header.x_scale_factor + header.x_offset)),
        ("y".to_string(), Json::Number(point.y as f64 * header.y_scale_factor + header.y_offset)),
        ("z".to_string(), Json::Number(point.z as f64 * header.z_scale_factor + header.z_offset)),
        ("intensity".to_string(), Json::Number(point.intensity as f64)),
        ("returnNumber".to_string(), Json::Number((a & 0x07) as f64)),
        ("numberOfReturns".to_string(), Json::Number(((a >> 3) & 0x07) as f64)),
        ("scanDirectionFlag".to_string(), Json::Bool((a >> 6) & 0x01 != 0)),
        ("edgeOfFlightLine".to_string(), Json::Bool((a >> 7) & 0x01 != 0)),
        ("classification".to_string(), Json::Number(b as f64)),
        ("scanAngleRank".to_string(), Json::Number(scan_angle_rank)),
        ("userData".to_string(), Json::Number(point.user_data as f64)),
        ("pointSourceId".to_string(), Json::Number(point.point_source_id as f64)),
        ("gpsTime".to_string(), point.gps_time.map(Json::Number).unwrap_or(Json::Null)),
        ("rgb".to_string(), point.color.map(|color| Json::Array(vec![Json::Number(color.red as f64), Json::Number(color.green as f64), Json::Number(color.blue as f64)])).unwrap_or(Json::Null)),
    ])
}
#[cfg(feature = "oracles")]
fn point_of_json(value: &Json, header: &las::raw::Header) -> Option<las::raw::Point> {
    let x = number(value, "x")?;
    let y = number(value, "y")?;
    let z = number(value, "z")?;
    let return_number = number(value, "returnNumber")? as u8 & 0x07;
    let number_of_returns = number(value, "numberOfReturns")? as u8 & 0x07;
    let scan_direction_flag = bool_of(value, "scanDirectionFlag")?;
    let edge_of_flight_line = bool_of(value, "edgeOfFlightLine")?;
    let classification = number(value, "classification")? as u8;
    let a = return_number | (number_of_returns << 3) | ((scan_direction_flag as u8) << 6) | ((edge_of_flight_line as u8) << 7);
    let gps_time = match value.get("gpsTime") {
        Some(Json::Number(time)) => Some(*time),
        _ => None,
    };
    let color = match value.get("rgb") {
        Some(Json::Array(items)) if items.len() == 3 => {
            let at = |index: usize| match &items[index] {
                Json::Number(channel) => Some(*channel as u16),
                _ => None,
            };
            Some(las::Color { red: at(0)?, green: at(1)?, blue: at(2)? })
        }
        _ => None,
    };
    Some(las::raw::Point {
        x: ((x - header.x_offset) / header.x_scale_factor).round() as i32,
        y: ((y - header.y_offset) / header.y_scale_factor).round() as i32,
        z: ((z - header.z_offset) / header.z_scale_factor).round() as i32,
        intensity: number(value, "intensity")? as u16,
        flags: las::raw::point::Flags::TwoByte(a, classification),
        scan_angle: las::raw::point::ScanAngle::Rank(number(value, "scanAngleRank")? as i8),
        user_data: number(value, "userData")? as u8,
        point_source_id: number(value, "pointSourceId")? as u16,
        gps_time,
        color,
        waveform: None,
        nir: None,
        extra_bytes: Vec::new(),
    })
}
//#endregion 🔖️PointJson

//#region 🔖️HeaderJson
/// 📋 Only the NON-STRUCTURAL header fields (`raw_doc::write`'s doc comment lists the six
/// STRUCTURAL ones this never carries): the same field set `../🧬️schema/📸️snapshot/🦀️component.rs`'s
/// `LasHeader` models, `file_source_id`/`global_encoding`/`guid` excluded as out of this wave's
/// contracted field list, exactly as `decode_las`/`encode_las` already exclude them.
#[cfg(feature = "oracles")]
fn header_to_json(header: &las::raw::Header) -> Json {
    Json::Object(vec![
        ("versionMajor".to_string(), Json::Number(header.version.major as f64)),
        ("versionMinor".to_string(), Json::Number(header.version.minor as f64)),
        ("systemIdentifier".to_string(), Json::String(read_fixed_str(&header.system_identifier))),
        ("generatingSoftware".to_string(), Json::String(read_fixed_str(&header.generating_software))),
        ("dayOfYear".to_string(), Json::Number(header.file_creation_day_of_year as f64)),
        ("year".to_string(), Json::Number(header.file_creation_year as f64)),
        ("scale".to_string(), Json::Array(vec![Json::Number(header.x_scale_factor), Json::Number(header.y_scale_factor), Json::Number(header.z_scale_factor)])),
        ("offset".to_string(), Json::Array(vec![Json::Number(header.x_offset), Json::Number(header.y_offset), Json::Number(header.z_offset)])),
        ("max".to_string(), Json::Array(vec![Json::Number(header.max_x), Json::Number(header.max_y), Json::Number(header.max_z)])),
        ("min".to_string(), Json::Array(vec![Json::Number(header.min_x), Json::Number(header.min_y), Json::Number(header.min_z)])),
        ("counts".to_string(), Json::Array(header.number_of_points_by_return.iter().map(|count| Json::Number(*count as f64)).collect())),
    ])
}
#[cfg(feature = "oracles")]
fn header_of_json(value: &Json) -> Option<las::raw::Header> {
    let scale = f64x3(value, "scale")?;
    let offset = f64x3(value, "offset")?;
    let max = f64x3(value, "max")?;
    let min = f64x3(value, "min")?;
    let counts = u32x5(value, "counts")?;
    Some(las::raw::Header {
        file_signature: las::raw::LASF,
        version: las::Version::new(number(value, "versionMajor")? as u8, number(value, "versionMinor")? as u8),
        system_identifier: write_fixed(&string(value, "systemIdentifier")?),
        generating_software: write_fixed(&string(value, "generatingSoftware")?),
        file_creation_day_of_year: number(value, "dayOfYear")? as u16,
        file_creation_year: number(value, "year")? as u16,
        number_of_points_by_return: counts,
        x_scale_factor: scale.0,
        y_scale_factor: scale.1,
        z_scale_factor: scale.2,
        x_offset: offset.0,
        y_offset: offset.1,
        z_offset: offset.2,
        max_x: max.0,
        max_y: max.1,
        max_z: max.2,
        min_x: min.0,
        min_y: min.1,
        min_z: min.2,
        ..Default::default()
    })
}
//#endregion 🔖️HeaderJson

//#region 🔖️SnapshotJson
/// 🧭️ The whole document as JSON — `{header, vlrs, points}` — reused for THREE roles: `set-snapshot`
/// params, `set-snapshot`'s inverse payload (the base document in this same shape), and
/// `project_las`'s comparison projection. One shape, three call sites, per CLAUDE.md.
#[cfg(feature = "oracles")]
fn snapshot_to_json(doc: &raw_doc::RawDoc) -> Json {
    Json::Object(vec![
        ("header".to_string(), header_to_json(&doc.header)),
        ("vlrs".to_string(), Json::Array(doc.vlrs.iter().map(vlr_to_json).collect())),
        ("points".to_string(), Json::Array(doc.points.iter().map(|point| point_to_json(point, &doc.header)).collect())),
    ])
}
#[cfg(feature = "oracles")]
fn snapshot_of_json(value: &Json) -> Option<raw_doc::RawDoc> {
    let header = header_of_json(value.get("header")?)?;
    let vlrs: Vec<las::raw::Vlr> = match value.get("vlrs")? {
        Json::Array(items) => items.iter().map(vlr_of_json).collect::<Option<Vec<_>>>()?,
        _ => return None,
    };
    let points: Vec<las::raw::Point> = match value.get("points")? {
        Json::Array(items) => items.iter().map(|item| point_of_json(item, &header)).collect::<Option<Vec<_>>>()?,
        _ => return None,
    };
    Some(raw_doc::RawDoc { header, vlrs, points })
}

/// 🔎️ The independent reader used to project BOTH the oracle's and the subject's output onto
/// `semantic-las-v1` before comparison.
#[cfg(feature = "oracles")]
pub fn project_las(bytes: &[u8]) -> Result<Json, String> {
    Ok(snapshot_to_json(&raw_doc::read(bytes)?))
}
#[cfg(not(feature = "oracles"))]
pub fn project_las(_bytes: &[u8]) -> Result<Json, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️SnapshotJson

//#region 🔖️Dispatch
/// 🦠️ Applies one declared mutation kind to a real artifact and returns the re-serialized bytes.
/// An unrecognised kind is an error, never a silent no-op: a mutation that is quietly skipped
/// reports as a passing test.
#[cfg(feature = "oracles")]
pub fn oracle_apply_mutation(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
    let params = mutation_params(spec);
    match spec.str("kind").as_str() {
        "" => Err("mutation spec carries no `kind`".to_string()),
        "no-mutation" => Ok(input.to_vec()),
        "set-snapshot" => raw_doc::write(&snapshot_of_json(&params).ok_or("set-snapshot: malformed snapshot")?),
        "set-version" => {
            let mut doc = raw_doc::read(input)?;
            doc.header.version = las::Version::new(number(&params, "major").ok_or("set-version: missing `major`")? as u8, number(&params, "minor").ok_or("set-version: missing `minor`")? as u8);
            raw_doc::write(&doc)
        }
        "set-system-identifier" => {
            let mut doc = raw_doc::read(input)?;
            doc.header.system_identifier = write_fixed(&string(&params, "systemIdentifier").ok_or("set-system-identifier: missing `systemIdentifier`")?);
            raw_doc::write(&doc)
        }
        "set-software-info" => {
            let mut doc = raw_doc::read(input)?;
            doc.header.generating_software = write_fixed(&string(&params, "generatingSoftware").ok_or("set-software-info: missing `generatingSoftware`")?);
            raw_doc::write(&doc)
        }
        "set-creation-date" => {
            let mut doc = raw_doc::read(input)?;
            doc.header.file_creation_day_of_year = number(&params, "dayOfYear").ok_or("set-creation-date: missing `dayOfYear`")? as u16;
            doc.header.file_creation_year = number(&params, "year").ok_or("set-creation-date: missing `year`")? as u16;
            raw_doc::write(&doc)
        }
        "set-scale-and-offset" => {
            let mut doc = raw_doc::read(input)?;
            let scale = f64x3(&params, "scale").ok_or("set-scale-and-offset: missing `scale`")?;
            let offset = f64x3(&params, "offset").ok_or("set-scale-and-offset: missing `offset`")?;
            (doc.header.x_scale_factor, doc.header.y_scale_factor, doc.header.z_scale_factor) = scale;
            (doc.header.x_offset, doc.header.y_offset, doc.header.z_offset) = offset;
            raw_doc::write(&doc)
        }
        "set-bounds" => {
            let mut doc = raw_doc::read(input)?;
            let max = f64x3(&params, "max").ok_or("set-bounds: missing `max`")?;
            let min = f64x3(&params, "min").ok_or("set-bounds: missing `min`")?;
            (doc.header.max_x, doc.header.max_y, doc.header.max_z) = max;
            (doc.header.min_x, doc.header.min_y, doc.header.min_z) = min;
            raw_doc::write(&doc)
        }
        "set-points-by-return" => {
            let mut doc = raw_doc::read(input)?;
            doc.header.number_of_points_by_return = u32x5(&params, "counts").ok_or("set-points-by-return: missing `counts`")?;
            raw_doc::write(&doc)
        }
        "insert-vlr" => {
            let mut doc = raw_doc::read(input)?;
            let index = number(&params, "index").ok_or("insert-vlr: missing `index`")? as usize;
            let vlr = vlr_of_json(params.get("vlr").ok_or("insert-vlr: missing `vlr`")?).ok_or("insert-vlr: malformed `vlr`")?;
            doc.vlrs.insert(index.min(doc.vlrs.len()), vlr);
            raw_doc::write(&doc)
        }
        "remove-vlr" => {
            let mut doc = raw_doc::read(input)?;
            let index = number(&params, "index").ok_or("remove-vlr: missing `index`")? as usize;
            if index < doc.vlrs.len() {
                let _ = doc.vlrs.remove(index);
            }
            raw_doc::write(&doc)
        }
        "set-vlr-data" => {
            let mut doc = raw_doc::read(input)?;
            let index = number(&params, "index").ok_or("set-vlr-data: missing `index`")? as usize;
            let data = string(&params, "data").ok_or("set-vlr-data: missing `data`")?.into_bytes();
            if let Some(vlr) = doc.vlrs.get_mut(index) {
                vlr.record_length_after_header = las::raw::vlr::RecordLength::Vlr(data.len() as u16);
                vlr.data = data;
            }
            raw_doc::write(&doc)
        }
        "insert-point" => {
            let mut doc = raw_doc::read(input)?;
            let index = number(&params, "index").ok_or("insert-point: missing `index`")? as usize;
            let point = point_of_json(params.get("point").ok_or("insert-point: missing `point`")?, &doc.header).ok_or("insert-point: malformed `point`")?;
            doc.points.insert(index.min(doc.points.len()), point);
            raw_doc::write(&doc)
        }
        "remove-point" => {
            let mut doc = raw_doc::read(input)?;
            let index = number(&params, "index").ok_or("remove-point: missing `index`")? as usize;
            if index < doc.points.len() {
                let _ = doc.points.remove(index);
            }
            raw_doc::write(&doc)
        }
        "set-point" => {
            let mut doc = raw_doc::read(input)?;
            let index = number(&params, "index").ok_or("set-point: missing `index`")? as usize;
            let point = point_of_json(params.get("point").ok_or("set-point: missing `point`")?, &doc.header).ok_or("set-point: malformed `point`")?;
            if index < doc.points.len() {
                doc.points[index] = point;
            }
            raw_doc::write(&doc)
        }
        kind => Err(format!("mutation kind {kind:?} has no oracle implementation ({} input byte(s))", input.len())),
    }
}

/// 🚫️ Without the `oracles` feature the reference implementation is not linked at all.
#[cfg(not(feature = "oracles"))]
pub fn oracle_apply_mutation(_input: &[u8], _spec: &Json) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️Dispatch

//#region 🔖️Inverse
/// ↩️ The spec that undoes `spec` when applied AFTER `oracle_apply_mutation(base, spec)`'s own
/// result — index-aware and computed from `base` (the pre-mutation document), mirroring
/// `LasMutation::inverse()` (`../🧬️schema/🧬️mutations/🦀️component.rs`) independently: an
/// out-of-range index that the forward mutation would have rejected inverts to `no-mutation`,
/// exactly as that hand-rolled method does.
#[cfg(feature = "oracles")]
pub fn oracle_inverse_spec(base: &[u8], spec: &Json) -> Result<Json, String> {
    let params = mutation_params(spec);
    let doc = raw_doc::read(base)?;
    Ok(match spec.str("kind").as_str() {
        "" => return Err("mutation spec carries no `kind`".to_string()),
        "no-mutation" => spec_of("no-mutation", Json::Object(vec![])),
        "set-snapshot" => spec_of("set-snapshot", snapshot_to_json(&doc)),
        "set-version" => spec_of("set-version", Json::Object(vec![("major".to_string(), Json::Number(doc.header.version.major as f64)), ("minor".to_string(), Json::Number(doc.header.version.minor as f64))])),
        "set-system-identifier" => spec_of("set-system-identifier", Json::Object(vec![("systemIdentifier".to_string(), Json::String(read_fixed_str(&doc.header.system_identifier)))])),
        "set-software-info" => spec_of("set-software-info", Json::Object(vec![("generatingSoftware".to_string(), Json::String(read_fixed_str(&doc.header.generating_software)))])),
        "set-creation-date" => spec_of("set-creation-date", Json::Object(vec![("dayOfYear".to_string(), Json::Number(doc.header.file_creation_day_of_year as f64)), ("year".to_string(), Json::Number(doc.header.file_creation_year as f64))])),
        "set-scale-and-offset" => spec_of(
            "set-scale-and-offset",
            Json::Object(vec![
                ("scale".to_string(), Json::Array(vec![Json::Number(doc.header.x_scale_factor), Json::Number(doc.header.y_scale_factor), Json::Number(doc.header.z_scale_factor)])),
                ("offset".to_string(), Json::Array(vec![Json::Number(doc.header.x_offset), Json::Number(doc.header.y_offset), Json::Number(doc.header.z_offset)])),
            ]),
        ),
        "set-bounds" => spec_of(
            "set-bounds",
            Json::Object(vec![
                ("max".to_string(), Json::Array(vec![Json::Number(doc.header.max_x), Json::Number(doc.header.max_y), Json::Number(doc.header.max_z)])),
                ("min".to_string(), Json::Array(vec![Json::Number(doc.header.min_x), Json::Number(doc.header.min_y), Json::Number(doc.header.min_z)])),
            ]),
        ),
        "set-points-by-return" => spec_of("set-points-by-return", Json::Object(vec![("counts".to_string(), Json::Array(doc.header.number_of_points_by_return.iter().map(|count| Json::Number(*count as f64)).collect()))])),
        "insert-vlr" => {
            let index = number(&params, "index").ok_or("insert-vlr: missing `index`")? as usize;
            spec_of("remove-vlr", Json::Object(vec![("index".to_string(), Json::Number(index.min(doc.vlrs.len()) as f64))]))
        }
        "remove-vlr" => {
            let index = number(&params, "index").ok_or("remove-vlr: missing `index`")? as usize;
            match doc.vlrs.get(index) {
                Some(vlr) => spec_of("insert-vlr", Json::Object(vec![("index".to_string(), Json::Number(index as f64)), ("vlr".to_string(), vlr_to_json(vlr))])),
                None => spec_of("no-mutation", Json::Object(vec![])),
            }
        }
        "set-vlr-data" => {
            let index = number(&params, "index").ok_or("set-vlr-data: missing `index`")? as usize;
            match doc.vlrs.get(index) {
                Some(vlr) => spec_of("set-vlr-data", Json::Object(vec![("index".to_string(), Json::Number(index as f64)), ("data".to_string(), Json::String(String::from_utf8_lossy(&vlr.data).to_string()))])),
                None => spec_of("no-mutation", Json::Object(vec![])),
            }
        }
        "insert-point" => {
            let index = number(&params, "index").ok_or("insert-point: missing `index`")? as usize;
            spec_of("remove-point", Json::Object(vec![("index".to_string(), Json::Number(index.min(doc.points.len()) as f64))]))
        }
        "remove-point" => {
            let index = number(&params, "index").ok_or("remove-point: missing `index`")? as usize;
            match doc.points.get(index) {
                Some(point) => spec_of("insert-point", Json::Object(vec![("index".to_string(), Json::Number(index as f64)), ("point".to_string(), point_to_json(point, &doc.header))])),
                None => spec_of("no-mutation", Json::Object(vec![])),
            }
        }
        "set-point" => {
            let index = number(&params, "index").ok_or("set-point: missing `index`")? as usize;
            match doc.points.get(index) {
                Some(point) => spec_of("set-point", Json::Object(vec![("index".to_string(), Json::Number(index as f64)), ("point".to_string(), point_to_json(point, &doc.header))])),
                None => spec_of("no-mutation", Json::Object(vec![])),
            }
        }
        kind => return Err(format!("mutation kind {kind:?} has no oracle implementation ({} base byte(s))", base.len())),
    })
}

/// 🚫️ Without the `oracles` feature the reference implementation is not linked at all.
#[cfg(not(feature = "oracles"))]
pub fn oracle_inverse_spec(_base: &[u8], _spec: &Json) -> Result<Json, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️Inverse

//#region 🔖️RoundTrip
/// 🔁️ A genuine independent decode + re-encode with no shortcut. Never bit-identical to the input —
/// see this file's top doc comment on the point-data-start-signature gap.
#[cfg(feature = "oracles")]
pub fn oracle_round_trip(input: &[u8]) -> Result<Vec<u8>, String> {
    raw_doc::write(&raw_doc::read(input)?)
}

/// 🚫️ Without the `oracles` feature the reference implementation is not linked at all.
#[cfg(not(feature = "oracles"))]
pub fn oracle_round_trip(_input: &[u8]) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️RoundTrip

//#region 🧪️Tests
#[cfg(all(test, feature = "oracles"))]
mod tests {
    use super::*;

    /// 🧪️ A small in-memory LAS 1.0 buffer (2 VLRs, 3 points), built straight from `raw_doc::write`
    /// itself so these dispatch-logic unit tests need no committed file on disk — the real
    /// 8,448-point `🧫️fixtures/🧊️pattern-sphere.las` fixture is exercised by the gherkin-driven case
    /// at `../../../../../../🧪️tests/mutate-las-1-0/🦀️component.rs` through `ctx.copy_fixture`.
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
    /// `../../../../../../🧪️tests/mutate-las-1-0/🦀️component.rs` exercises, not of this module in
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
}
//#endregion 🧪️Tests
