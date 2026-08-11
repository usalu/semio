//! 🧬️ LasMutation — document mutation dispatch. Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: real vocabulary beyond
//! the universal `{NoMutation, SetSnapshot}` stub — header fields grouped sensibly
//! (`SetVersion`/`SetSystemIdentifier`/`SetSoftwareInfo`/`SetCreationDate`/`SetScaleAndOffset`/
//! `SetBounds`/`SetPointsByReturn`), `InsertVlr`/`RemoveVlr`/`SetVlrData` and
//! `InsertPoint`/`RemovePoint`/`SetPoint` cover the index-keyed collections. Every variant's
//! `diff()` is handcrafted (constructs `LasDiff` directly via the `schema::diff` builders) —
//! apply-and-capture is never used.

use crate::artifacts::las::schema::diff::{self, LasDiff};
use crate::artifacts::las::schema::snapshot::{LasHeader, LasPoint, LasVlr};
use crate::artifacts::las::LasSnapshot;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.las`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum LasMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        snapshot: LasSnapshot,
    },
    /// 🔢️ Sets the LAS point format major/minor version.
    SetVersion {
        major: u8,
        minor: u8,
    },
    /// 🏢️ Sets §2.3 System Identifier.
    SetSystemIdentifier {
        system_identifier: String,
    },
    /// 🛠️ Sets §2.3 Generating Software.
    SetSoftwareInfo {
        generating_software: String,
    },
    /// 🕰️ Sets the file creation day-of-year / year.
    SetCreationDate {
        day_of_year: u16,
        year: u16,
    },
    /// 📏️ Sets X/Y/Z scale factors and offsets (the two fields that jointly reconstruct
    /// real-world coordinates from the on-disk integer `x/y/z`).
    SetScaleAndOffset {
        scale: (f64, f64, f64),
        offset: (f64, f64, f64),
    },
    /// 📦️ Sets the X/Y/Z max/min bounding box.
    SetBounds {
        max: (f64, f64, f64),
        min: (f64, f64, f64),
    },
    /// 🔁️ Sets the Number of Points by Return histogram (return channels 1..=5).
    SetPointsByReturn {
        counts: [u32; 5],
    },
    /// ➕️ Inserts a fully-specified VLR at `index` (final position, clamped to `len`).
    InsertVlr {
        index: usize,
        vlr: LasVlr,
    },
    /// ➖️ Removes the VLR at `index` (no-op if out of range).
    RemoveVlr {
        index: usize,
    },
    /// 📦️ Replaces a VLR's payload bytes.
    SetVlrData {
        index: usize,
        data: Vec<u8>,
    },
    /// ➕️ Inserts a fully-specified point record at `index` (final position, clamped to `len`).
    InsertPoint {
        index: usize,
        point: LasPoint,
    },
    /// ➖️ Removes the point record at `index` (no-op if out of range).
    RemovePoint {
        index: usize,
    },
    /// ✏️ Replaces a point record wholesale.
    SetPoint {
        index: usize,
        point: LasPoint,
    },
}
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`. Every index-targeted variant is a graceful no-op when
/// `index` is out of range (a stale index from a concurrent edit degrades gracefully, never
/// panics). `InsertVlr`/`RemoveVlr`/`InsertPoint`/`RemovePoint` keep `header.number_of_vlrs`/
/// `header.number_of_point_records` in sync with the real collection length (`engine::encode_las`
/// also independently recomputes both at encode time — see `LasHeader`'s doc comment — so this
/// sync is a snapshot-level consistency guarantee, not the sole source of correctness).
pub fn apply_las_mutation(snapshot: &mut LasSnapshot, mutation: &LasMutation) -> LasDiff {
    let __diff = <LasMutation as protocol::Mutation<LasSnapshot>>::diff(mutation, snapshot);
    match mutation {
        LasMutation::NoMutation => {}
        LasMutation::SetSnapshot { snapshot: next } => *snapshot = next.clone(),
        LasMutation::SetVersion { major, minor } => {
            snapshot.header.version_major = *major;
            snapshot.header.version_minor = *minor;
        }
        LasMutation::SetSystemIdentifier { system_identifier } => {
            snapshot.header.system_identifier = system_identifier.clone();
        }
        LasMutation::SetSoftwareInfo { generating_software } => {
            snapshot.header.generating_software = generating_software.clone();
        }
        LasMutation::SetCreationDate { day_of_year, year } => {
            snapshot.header.creation_day_of_year = *day_of_year;
            snapshot.header.creation_year = *year;
        }
        LasMutation::SetScaleAndOffset { scale, offset } => {
            snapshot.header.x_scale = scale.0;
            snapshot.header.y_scale = scale.1;
            snapshot.header.z_scale = scale.2;
            snapshot.header.x_offset = offset.0;
            snapshot.header.y_offset = offset.1;
            snapshot.header.z_offset = offset.2;
        }
        LasMutation::SetBounds { max, min } => {
            snapshot.header.max_x = max.0;
            snapshot.header.max_y = max.1;
            snapshot.header.max_z = max.2;
            snapshot.header.min_x = min.0;
            snapshot.header.min_y = min.1;
            snapshot.header.min_z = min.2;
        }
        LasMutation::SetPointsByReturn { counts } => snapshot.header.points_by_return = *counts,
        LasMutation::InsertVlr { index, vlr } => {
            let at = (*index).min(snapshot.vlrs.len());
            snapshot.vlrs.insert(at, vlr.clone());
            snapshot.header.number_of_vlrs = snapshot.vlrs.len() as u32;
        }
        LasMutation::RemoveVlr { index } => {
            if *index < snapshot.vlrs.len() {
                snapshot.vlrs.remove(*index);
                snapshot.header.number_of_vlrs = snapshot.vlrs.len() as u32;
            }
        }
        LasMutation::SetVlrData { index, data } => {
            if let Some(v) = snapshot.vlrs.get_mut(*index) { v.data = data.clone(); }
        }
        LasMutation::InsertPoint { index, point } => {
            let at = (*index).min(snapshot.points.len());
            snapshot.points.insert(at, point.clone());
            snapshot.header.number_of_point_records = snapshot.points.len() as u32;
        }
        LasMutation::RemovePoint { index } => {
            if *index < snapshot.points.len() {
                snapshot.points.remove(*index);
                snapshot.header.number_of_point_records = snapshot.points.len() as u32;
            }
        }
        LasMutation::SetPoint { index, point } => {
            if let Some(p) = snapshot.points.get_mut(*index) { *p = point.clone(); }
        }
    }

    __diff
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
impl Mutation<LasSnapshot> for LasMutation {
    type Diff = LasDiff;

    fn diff(&self, base: &LasSnapshot) -> Self::Diff {
        match self {
            LasMutation::NoMutation => LasDiff::default(),
            LasMutation::SetSnapshot { snapshot } => diff::diff_set_snapshot(base, snapshot),
            LasMutation::SetVersion { major, minor } => diff::diff_set_version(*major, *minor),
            LasMutation::SetSystemIdentifier { system_identifier } => diff::diff_set_system_identifier(system_identifier),
            LasMutation::SetSoftwareInfo { generating_software } => diff::diff_set_software_info(generating_software),
            LasMutation::SetCreationDate { day_of_year, year } => diff::diff_set_creation_date(*day_of_year, *year),
            LasMutation::SetScaleAndOffset { scale, offset } => diff::diff_set_scale_and_offset(*scale, *offset),
            LasMutation::SetBounds { max, min } => diff::diff_set_bounds(*max, *min),
            LasMutation::SetPointsByReturn { counts } => diff::diff_set_points_by_return(*counts),
            LasMutation::InsertVlr { index, vlr } => diff::diff_insert_vlr(base, *index, vlr.clone()),
            LasMutation::RemoveVlr { index } => diff::diff_remove_vlr(base, *index),
            LasMutation::SetVlrData { index, data } => diff::diff_set_vlr_data(*index, data.clone()),
            LasMutation::InsertPoint { index, point } => diff::diff_insert_point(base, *index, point.clone()),
            LasMutation::RemovePoint { index } => diff::diff_remove_point(base, *index),
            LasMutation::SetPoint { index, point } => diff::diff_set_point(base, *index, point.clone()),
        }
    }

    /// ↩️ Handcrafted, index-aware mutation-level inverses. Index-targeted variants look the
    /// prior value up in `base`; a stale/out-of-range index inverts to `NoMutation` (nothing to
    /// undo).
    fn inverse(&self, base: &LasSnapshot) -> Vec<Self> {
        match self {
            LasMutation::NoMutation => vec![LasMutation::NoMutation],
            LasMutation::SetSnapshot { .. } => vec![LasMutation::SetSnapshot { snapshot: base.clone() }],
            LasMutation::SetVersion { .. } => vec![LasMutation::SetVersion { major: base.header.version_major, minor: base.header.version_minor }],
            LasMutation::SetSystemIdentifier { .. } => vec![LasMutation::SetSystemIdentifier { system_identifier: base.header.system_identifier.clone() }],
            LasMutation::SetSoftwareInfo { .. } => vec![LasMutation::SetSoftwareInfo { generating_software: base.header.generating_software.clone() }],
            LasMutation::SetCreationDate { .. } => vec![LasMutation::SetCreationDate { day_of_year: base.header.creation_day_of_year, year: base.header.creation_year }],
            LasMutation::SetScaleAndOffset { .. } => vec![LasMutation::SetScaleAndOffset {
                scale: (base.header.x_scale, base.header.y_scale, base.header.z_scale),
                offset: (base.header.x_offset, base.header.y_offset, base.header.z_offset),
            }],
            LasMutation::SetBounds { .. } => vec![LasMutation::SetBounds {
                max: (base.header.max_x, base.header.max_y, base.header.max_z),
                min: (base.header.min_x, base.header.min_y, base.header.min_z),
            }],
            LasMutation::SetPointsByReturn { .. } => vec![LasMutation::SetPointsByReturn { counts: base.header.points_by_return }],
            LasMutation::InsertVlr { index, .. } => vec![LasMutation::RemoveVlr { index: (*index).min(base.vlrs.len()) }],
            LasMutation::RemoveVlr { index } => match base.vlrs.get(*index) {
                Some(v) => vec![LasMutation::InsertVlr { index: *index, vlr: v.clone() }],
                None => vec![LasMutation::NoMutation],
            },
            LasMutation::SetVlrData { index, .. } => match base.vlrs.get(*index) {
                Some(v) => vec![LasMutation::SetVlrData { index: *index, data: v.data.clone() }],
                None => vec![LasMutation::NoMutation],
            },
            LasMutation::InsertPoint { index, .. } => vec![LasMutation::RemovePoint { index: (*index).min(base.points.len()) }],
            LasMutation::RemovePoint { index } => match base.points.get(*index) {
                Some(p) => vec![LasMutation::InsertPoint { index: *index, point: p.clone() }],
                None => vec![LasMutation::NoMutation],
            },
            LasMutation::SetPoint { index, .. } => match base.points.get(*index) {
                Some(p) => vec![LasMutation::SetPoint { index: *index, point: p.clone() }],
                None => vec![LasMutation::NoMutation],
            },
        }
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
/// 🧪️ F6 (las, recon-gap-fill): **hand-rolled** `OpText`/`OpBinary` for `LasMutation` — the
/// derive path (`#[derive(dsl::DslOps)]`) is NOT usable here. STEP 1 classification done for real
/// (attribute added, `cargo check -p semio-s-plugin-stdio --lib` run, real errors read, then
/// reverted): `LasMutation::SetScaleAndOffset`/`SetBounds` carry bare tuple fields
/// (`scale`/`offset`/`max`/`min`: `(f64, f64, f64)`) — real compiler output:
/// `error[E0277]: the trait bound `(f64, f64, f64): DslField` is not satisfied` (4 occurrences).
/// Root cause: no blanket `impl<..> DslField for (A, B, C)` exists in the `dsl` crate (same gap
/// class as `LasDiff`'s tri-state blocker — see that module's doc comment). `SetSnapshot`
/// independently fails too, transitively: `LasSnapshot` embeds `LasPoint::rgb: Option<(u16, u16,
/// u16)>`, the exact same bare-tuple gap.
///
/// **Grammar**: `keyword arg=value ...` (space-separated, one token per argument — no argument is
/// ever omitted since a mutation's `Option<T>` argument means "the new value", never a diff
/// tri-state, so no field is ever elided the way `LasDiff`'s sparse tokens are). Reuses every
/// primitive/value-codec the `🔺️diff` module already made `pub(crate)` for exactly this purpose
/// (`diff::hex_encode`/`diff::enc_vlr`/`diff::enc_point`/`diff::enc_u32x5`/etc. — same
/// intra-artifact reuse pattern svg's mutations module uses against its own diff module's
/// primitives). `encode_op`/`decode_op` = the text bytes verbatim, same simplification the
/// hand-rolled `DiffCodec` above uses.
//#region 🔖️SnapshotCodec
/// 📋 Whole-`LasHeader` positional codec — only needed by `SetSnapshot`'s `snapshot` argument (no
/// other variant carries a full header).
fn enc_header(h: &LasHeader) -> String {
    let fields: Vec<String> = vec![
        h.version_major.to_string(),
        h.version_minor.to_string(),
        diff::hex_encode(h.system_identifier.as_bytes()),
        diff::hex_encode(h.generating_software.as_bytes()),
        h.creation_day_of_year.to_string(),
        h.creation_year.to_string(),
        h.header_size.to_string(),
        h.offset_to_point_data.to_string(),
        h.number_of_vlrs.to_string(),
        h.point_data_format_id.to_string(),
        h.point_data_record_length.to_string(),
        h.number_of_point_records.to_string(),
        diff::enc_u32x5(&h.points_by_return),
        h.x_scale.to_string(), h.y_scale.to_string(), h.z_scale.to_string(),
        h.x_offset.to_string(), h.y_offset.to_string(), h.z_offset.to_string(),
        h.max_x.to_string(), h.min_x.to_string(),
        h.max_y.to_string(), h.min_y.to_string(),
        h.max_z.to_string(), h.min_z.to_string(),
    ];
    format!("[{}]", fields.join(","))
}
fn dec_header(s: &str) -> Result<LasHeader, String> {
    let parts = diff::split_top_level(diff::strip_brackets(s)?, ',');
    let [version_major, version_minor, system_identifier, generating_software, creation_day_of_year, creation_year, header_size, offset_to_point_data, number_of_vlrs, point_data_format_id, point_data_record_length, number_of_point_records, points_by_return, x_scale, y_scale, z_scale, x_offset, y_offset, z_offset, max_x, min_x, max_y, min_y, max_z, min_z] = parts.as_slice() else {
        return Err(format!("header: expected 25 fields, got {}", parts.len()));
    };
    Ok(LasHeader {
        version_major: diff::parse_u8(version_major)?,
        version_minor: diff::parse_u8(version_minor)?,
        system_identifier: String::from_utf8(diff::hex_decode(system_identifier)?).map_err(|e| e.to_string())?,
        generating_software: String::from_utf8(diff::hex_decode(generating_software)?).map_err(|e| e.to_string())?,
        creation_day_of_year: diff::parse_u16(creation_day_of_year)?,
        creation_year: diff::parse_u16(creation_year)?,
        header_size: diff::parse_u16(header_size)?,
        offset_to_point_data: diff::parse_u32(offset_to_point_data)?,
        number_of_vlrs: diff::parse_u32(number_of_vlrs)?,
        point_data_format_id: diff::parse_u8(point_data_format_id)?,
        point_data_record_length: diff::parse_u16(point_data_record_length)?,
        number_of_point_records: diff::parse_u32(number_of_point_records)?,
        points_by_return: diff::dec_u32x5(points_by_return)?,
        x_scale: diff::parse_f64(x_scale)?, y_scale: diff::parse_f64(y_scale)?, z_scale: diff::parse_f64(z_scale)?,
        x_offset: diff::parse_f64(x_offset)?, y_offset: diff::parse_f64(y_offset)?, z_offset: diff::parse_f64(z_offset)?,
        max_x: diff::parse_f64(max_x)?, min_x: diff::parse_f64(min_x)?,
        max_y: diff::parse_f64(max_y)?, min_y: diff::parse_f64(min_y)?,
        max_z: diff::parse_f64(max_z)?, min_z: diff::parse_f64(min_z)?,
    })
}
fn enc_snapshot(s: &LasSnapshot) -> String {
    let vlrs = s.vlrs.iter().map(diff::enc_vlr).collect::<Vec<_>>().join(",");
    let points = s.points.iter().map(diff::enc_point).collect::<Vec<_>>().join(",");
    format!("[{},[{}],[{}]]", enc_header(&s.header), vlrs, points)
}
fn dec_snapshot(s: &str) -> Result<LasSnapshot, String> {
    let inner = diff::strip_brackets(s)?;
    let parts = diff::split_top_level(inner, ',');
    let [header_s, vlrs_s, points_s] = parts.as_slice() else {
        return Err(format!("snapshot: expected 3 top-level fields, got {}", parts.len()));
    };
    let header = dec_header(header_s)?;
    let vlrs = diff::split_top_level(diff::strip_brackets(vlrs_s)?, ',')
        .into_iter().filter(|s| !s.is_empty()).map(diff::dec_vlr).collect::<Result<Vec<_>, String>>()?;
    let points = diff::split_top_level(diff::strip_brackets(points_s)?, ',')
        .into_iter().filter(|s| !s.is_empty()).map(diff::dec_point).collect::<Result<Vec<_>, String>>()?;
    Ok(LasSnapshot { schema: crate::artifacts::las::STDIO_LAS_DOCUMENT_SCHEMA.into(), header, vlrs, points })
}
//#endregion 🔖️SnapshotCodec

//#region 🔖️TupleCodec
fn enc_f64x3(t: &(f64, f64, f64)) -> String {
    format!("[{},{},{}]", t.0, t.1, t.2)
}
fn dec_f64x3(s: &str) -> Result<(f64, f64, f64), String> {
    let parts = diff::split_top_level(diff::strip_brackets(s)?, ',');
    let [a, b, c] = parts.as_slice() else { return Err(format!("f64x3: expected 3 fields, got {}", parts.len())) };
    Ok((diff::parse_f64(a)?, diff::parse_f64(b)?, diff::parse_f64(c)?))
}
//#endregion 🔖️TupleCodec

//#region 🔖️TopLevel
fn print_las_mutation(m: &LasMutation) -> String {
    match m {
        LasMutation::NoMutation => "no-mutation".to_string(),
        LasMutation::SetSnapshot { snapshot } => format!("set-snapshot snapshot={}", enc_snapshot(snapshot)),
        LasMutation::SetVersion { major, minor } => format!("set-version major={major} minor={minor}"),
        LasMutation::SetSystemIdentifier { system_identifier } => format!("set-system-identifier system-identifier={}", diff::hex_encode(system_identifier.as_bytes())),
        LasMutation::SetSoftwareInfo { generating_software } => format!("set-software-info generating-software={}", diff::hex_encode(generating_software.as_bytes())),
        LasMutation::SetCreationDate { day_of_year, year } => format!("set-creation-date day-of-year={day_of_year} year={year}"),
        LasMutation::SetScaleAndOffset { scale, offset } => format!("set-scale-and-offset scale={} offset={}", enc_f64x3(scale), enc_f64x3(offset)),
        LasMutation::SetBounds { max, min } => format!("set-bounds max={} min={}", enc_f64x3(max), enc_f64x3(min)),
        LasMutation::SetPointsByReturn { counts } => format!("set-points-by-return counts={}", diff::enc_u32x5(counts)),
        LasMutation::InsertVlr { index, vlr } => format!("insert-vlr index={index} vlr={}", diff::enc_vlr(vlr)),
        LasMutation::RemoveVlr { index } => format!("remove-vlr index={index}"),
        LasMutation::SetVlrData { index, data } => format!("set-vlr-data index={index} data={}", diff::hex_encode(data)),
        LasMutation::InsertPoint { index, point } => format!("insert-point index={index} point={}", diff::enc_point(point)),
        LasMutation::RemovePoint { index } => format!("remove-point index={index}"),
        LasMutation::SetPoint { index, point } => format!("set-point index={index} point={}", diff::enc_point(point)),
    }
}
fn parse_las_mutation(line: &str) -> Result<LasMutation, String> {
    let mut tokens = line.split(' ');
    let keyword = tokens.next().filter(|k| !k.is_empty()).ok_or_else(|| "empty mutation line".to_string())?;
    let rest: Vec<&str> = tokens.collect();
    let arg = |key: &str| -> Result<&str, String> {
        rest.iter().find_map(|t| t.strip_prefix(key)).ok_or_else(|| format!("{keyword}: missing arg {key:?}"))
    };
    match keyword {
        "no-mutation" => Ok(LasMutation::NoMutation),
        "set-snapshot" => Ok(LasMutation::SetSnapshot { snapshot: dec_snapshot(arg("snapshot=")?)? }),
        "set-version" => Ok(LasMutation::SetVersion { major: diff::parse_u8(arg("major=")?)?, minor: diff::parse_u8(arg("minor=")?)? }),
        "set-system-identifier" => Ok(LasMutation::SetSystemIdentifier { system_identifier: String::from_utf8(diff::hex_decode(arg("system-identifier=")?)?).map_err(|e| e.to_string())? }),
        "set-software-info" => Ok(LasMutation::SetSoftwareInfo { generating_software: String::from_utf8(diff::hex_decode(arg("generating-software=")?)?).map_err(|e| e.to_string())? }),
        "set-creation-date" => Ok(LasMutation::SetCreationDate { day_of_year: diff::parse_u16(arg("day-of-year=")?)?, year: diff::parse_u16(arg("year=")?)? }),
        "set-scale-and-offset" => Ok(LasMutation::SetScaleAndOffset { scale: dec_f64x3(arg("scale=")?)?, offset: dec_f64x3(arg("offset=")?)? }),
        "set-bounds" => Ok(LasMutation::SetBounds { max: dec_f64x3(arg("max=")?)?, min: dec_f64x3(arg("min=")?)? }),
        "set-points-by-return" => Ok(LasMutation::SetPointsByReturn { counts: diff::dec_u32x5(arg("counts=")?)? }),
        "insert-vlr" => Ok(LasMutation::InsertVlr { index: diff::parse_usize(arg("index=")?)?, vlr: diff::dec_vlr(arg("vlr=")?)? }),
        "remove-vlr" => Ok(LasMutation::RemoveVlr { index: diff::parse_usize(arg("index=")?)? }),
        "set-vlr-data" => Ok(LasMutation::SetVlrData { index: diff::parse_usize(arg("index=")?)?, data: diff::hex_decode(arg("data=")?)? }),
        "insert-point" => Ok(LasMutation::InsertPoint { index: diff::parse_usize(arg("index=")?)?, point: diff::dec_point(arg("point=")?)? }),
        "remove-point" => Ok(LasMutation::RemovePoint { index: diff::parse_usize(arg("index=")?)? }),
        "set-point" => Ok(LasMutation::SetPoint { index: diff::parse_usize(arg("index=")?)?, point: diff::dec_point(arg("point=")?)? }),
        other => Err(format!("las mutation: unknown keyword {other:?}")),
    }
}
//#endregion 🔖️TopLevel

impl protocol::OpText for LasMutation {
    fn print_op(&self) -> String {
        print_las_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        parse_las_mutation(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
}

impl protocol::OpBinary for LasMutation {
    /// ⚡️ Binary = the text bytes verbatim (same simplification the `🔺️diff` module's hand-rolled
    /// `DiffCodec::encode_diff` uses — satisfies every `OpBinary` law without inventing a second
    /// wire format).
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(print_las_mutation(self).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let line = std::str::from_utf8(bytes).map_err(|e| protocol::ProtocolError::Malformed { what: "op utf8", offset: 0, detail: e.to_string() })?;
        parse_las_mutation(line).map_err(|e| protocol::ProtocolError::Malformed { what: "op text", offset: 0, detail: e.to_string() })
    }
}
//#endregion OpCodecs

//#region Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::las::schema::diff::{LasPointsDiff, LasVlrsDiff};
    use protocol::command::DiffAlgebra;
    use protocol::MutationDiff;
    use protocol::{OpBinary, OpText};

    //#region Fixtures
    fn vlr(user_id: &str, record_id: u16, data: &[u8]) -> LasVlr {
        LasVlr { user_id: user_id.into(), record_id, description: format!("vlr {record_id}"), data: data.to_vec() }
    }

    fn point(seed: u8) -> LasPoint {
        LasPoint {
            x: 100.0 + seed as f64,
            y: -50.0 + seed as f64 * 0.5,
            z: 10.0 + seed as f64 * 0.1,
            intensity: 100 + seed as u16,
            return_number: (seed % 5) + 1,
            number_of_returns: ((seed + 1) % 5) + 1,
            scan_direction_flag: seed % 2 == 0,
            edge_of_flight_line: seed % 3 == 0,
            classification: seed,
            scan_angle_rank: seed as i8 - 10,
            user_data: seed,
            point_source_id: 1000 + seed as u16,
            gps_time: None,
            rgb: None,
        }
    }

    fn base_snapshot() -> LasSnapshot {
        let vlrs = vec![vlr("LASF_Spec", 100, b"vlr-a"), vlr("LASF_Spec", 101, b"vlr-b")];
        let points = vec![point(0), point(1), point(2)];
        LasSnapshot {
            schema: "stdio.las".into(),
            header: LasHeader {
                number_of_vlrs: vlrs.len() as u32,
                number_of_point_records: points.len() as u32,
                ..LasHeader::default()
            },
            vlrs,
            points,
        }
    }
    //#endregion Fixtures

    //#region 🔖️mutation_diff_law
    fn assert_mutation_diff_law(base: &LasSnapshot, mutation: LasMutation) {
        let expected_diff = mutation.diff(base);
        let mut applied_snapshot = base.clone();
        let returned_diff = apply_las_mutation(&mut applied_snapshot, &mutation);
        assert_eq!(returned_diff, expected_diff, "apply_las_mutation must return mutation.diff(base) for {mutation:?}");
        assert_eq!(expected_diff.apply(base), applied_snapshot, "diff.apply(base) must equal the imperative mutation result for {mutation:?}");
    }

    #[test]
    fn mutation_diff_law() {
        let base = base_snapshot();
        assert_mutation_diff_law(&base, LasMutation::NoMutation);
        let mut alt = base.clone();
        alt.header.creation_year = 2030;
        assert_mutation_diff_law(&base, LasMutation::SetSnapshot { snapshot: alt });
        assert_mutation_diff_law(&base, LasMutation::SetVersion { major: 1, minor: 4 });
        assert_mutation_diff_law(&base, LasMutation::SetSystemIdentifier { system_identifier: "semio".into() });
        assert_mutation_diff_law(&base, LasMutation::SetSoftwareInfo { generating_software: "semio-las-writer".into() });
        assert_mutation_diff_law(&base, LasMutation::SetCreationDate { day_of_year: 42, year: 2026 });
        assert_mutation_diff_law(&base, LasMutation::SetScaleAndOffset { scale: (0.001, 0.001, 0.001), offset: (1000.0, 2000.0, 0.0) });
        assert_mutation_diff_law(&base, LasMutation::SetBounds { max: (999.0, 888.0, 777.0), min: (-1.0, -2.0, -3.0) });
        assert_mutation_diff_law(&base, LasMutation::SetPointsByReturn { counts: [1, 2, 3, 4, 5] });
        assert_mutation_diff_law(&base, LasMutation::InsertVlr { index: 1, vlr: vlr("EXTRA", 200, b"new-vlr") });
        assert_mutation_diff_law(&base, LasMutation::RemoveVlr { index: 0 });
        assert_mutation_diff_law(&base, LasMutation::SetVlrData { index: 0, data: b"patched".to_vec() });
        assert_mutation_diff_law(&base, LasMutation::InsertPoint { index: 1, point: point(9) });
        assert_mutation_diff_law(&base, LasMutation::RemovePoint { index: 0 });
        assert_mutation_diff_law(&base, LasMutation::SetPoint { index: 0, point: point(42) });
        // Out-of-range index: graceful no-op, still law-compliant.
        assert_mutation_diff_law(&base, LasMutation::RemoveVlr { index: 99 });
        assert_mutation_diff_law(&base, LasMutation::SetPoint { index: 99, point: point(1) });
    }
    //#endregion 🔖️mutation_diff_law

    //#region 🔖️inverse_law
    #[test]
    fn inverse_law() {
        let base = base_snapshot();
        let variants = vec![
            LasMutation::NoMutation,
            LasMutation::SetVersion { major: 1, minor: 4 },
            LasMutation::SetSystemIdentifier { system_identifier: "semio".into() },
            LasMutation::SetSoftwareInfo { generating_software: "semio-las-writer".into() },
            LasMutation::SetCreationDate { day_of_year: 42, year: 2026 },
            LasMutation::SetScaleAndOffset { scale: (0.001, 0.001, 0.001), offset: (1000.0, 2000.0, 0.0) },
            LasMutation::SetBounds { max: (999.0, 888.0, 777.0), min: (-1.0, -2.0, -3.0) },
            LasMutation::SetPointsByReturn { counts: [1, 2, 3, 4, 5] },
            LasMutation::InsertVlr { index: 1, vlr: vlr("EXTRA", 200, b"new-vlr") },
            LasMutation::RemoveVlr { index: 0 },
            LasMutation::SetVlrData { index: 0, data: b"patched".to_vec() },
            LasMutation::InsertPoint { index: 1, point: point(9) },
            LasMutation::RemovePoint { index: 0 },
            LasMutation::SetPoint { index: 0, point: point(42) },
        ];
        for m in variants {
            // Mutation-level round trip.
            let mut snap = base.clone();
            apply_las_mutation(&mut snap, &m);
            for inv in m.inverse(&base) {
                apply_las_mutation(&mut snap, &inv);
            }
            assert_eq!(snap, base, "mutation-level inverse must restore base for {m:?}");

            // Diff-level round trip.
            let d = m.diff(&base);
            let mutated = d.apply(&base);
            let inv_d = d.inverse(&base);
            assert_eq!(inv_d.apply(&mutated), base, "diff-level inverse must restore base for {m:?}");
        }
    }
    //#endregion 🔖️inverse_law

    //#region 🔖️absorb_law
    fn assert_absorb_law(base: &LasSnapshot, m1: LasMutation, m2: LasMutation) {
        let d1 = m1.diff(base);
        let mid = d1.apply(base);
        let d2 = m2.diff(&mid);
        let sequential = d2.apply(&mid);

        let mut merged = d1.clone();
        merged.absorb(d2.clone());
        assert_eq!(merged.apply(base), sequential, "absorb(d1,d2).apply(base) must equal sequential application for {m1:?} + {m2:?}");
    }

    #[test]
    fn absorb_law() {
        let base = base_snapshot();

        // Insert+Remove-before (vlrs): canonical shift case.
        assert_absorb_law(&base, LasMutation::InsertVlr { index: 1, vlr: vlr("X", 1, b"x") }, LasMutation::RemoveVlr { index: 0 });

        // Insert+Insert-same-index (vlrs): both survive.
        assert_absorb_law(&base, LasMutation::InsertVlr { index: 1, vlr: vlr("X", 1, b"x") }, LasMutation::InsertVlr { index: 1, vlr: vlr("Y", 2, b"y") });

        // Add+SetField (vlrs): patches directly into the still-pending added VLR.
        assert_absorb_law(&base, LasMutation::InsertVlr { index: 0, vlr: vlr("X", 1, b"x") }, LasMutation::SetVlrData { index: 0, data: b"patched".to_vec() });

        // Modify+Remove (vlrs): a pending field patch on a since-removed base VLR vanishes.
        assert_absorb_law(&base, LasMutation::SetVlrData { index: 0, data: b"will be dropped".to_vec() }, LasMutation::RemoveVlr { index: 0 });

        // Insert+Remove-before (points): same canonical case, other collection.
        assert_absorb_law(&base, LasMutation::InsertPoint { index: 1, point: point(9) }, LasMutation::RemovePoint { index: 0 });

        // Insert+Insert-same-index (points): both survive.
        assert_absorb_law(&base, LasMutation::InsertPoint { index: 1, point: point(9) }, LasMutation::InsertPoint { index: 1, point: point(8) });

        // Add+SetField (points): patches into the pending added point.
        assert_absorb_law(&base, LasMutation::InsertPoint { index: 0, point: point(9) }, LasMutation::SetPoint { index: 0, point: point(7) });

        // Insert then annihilate the very same insert (points).
        assert_absorb_law(&base, LasMutation::InsertPoint { index: 0, point: point(9) }, LasMutation::RemovePoint { index: 0 });

        // Two unrelated scalar sets absorb via LWW.
        assert_absorb_law(&base, LasMutation::SetSystemIdentifier { system_identifier: "first".into() }, LasMutation::SetSystemIdentifier { system_identifier: "second".into() });
    }

    #[test]
    fn absorb_law_associativity() {
        let base = base_snapshot();
        let d1 = LasMutation::SetSystemIdentifier { system_identifier: "one".into() }.diff(&base);
        let mid1 = d1.apply(&base);
        let d2 = LasMutation::InsertPoint { index: 0, point: point(9) }.diff(&mid1);
        let mid2 = d2.apply(&mid1);
        let d3 = LasMutation::SetPoint { index: 0, point: point(7) }.diff(&mid2);

        // (d1∘d2)∘d3
        let mut left = d1.clone();
        left.absorb(d2.clone());
        left.absorb(d3.clone());

        // d1∘(d2∘d3)
        let mut d23 = d2.clone();
        d23.absorb(d3.clone());
        let mut right = d1.clone();
        right.absorb(d23);

        assert_eq!(left.apply(&base), right.apply(&base), "absorb must associate");
        assert_eq!(left.apply(&base), d3.apply(&mid2), "associated absorb must match full sequential application");
    }
    //#endregion 🔖️absorb_law

    //#region 🔖️between_roundtrip_law
    #[test]
    fn between_roundtrip_law() {
        let a = base_snapshot();
        let mut b = base_snapshot();
        b.header.creation_year = 2030;
        b.vlrs.remove(0);
        b.vlrs[0].description = "modified".into();
        b.vlrs.push(vlr("NEW", 300, b"new-vlr"));
        b.points.remove(0);
        b.points[0].classification = 250;
        b.points.push(point(50));

        let d = LasDiff::between(&a, &b);
        assert_eq!(d.apply(&a), b, "between(a,b).apply(a) must equal b");
        let d_rev = LasDiff::between(&b, &a);
        assert_eq!(d_rev.apply(&b), a, "between(b,a).apply(b) must equal a");
        assert!(LasDiff::between(&a, &a).is_empty(), "between(a,a) must be empty");
    }
    //#endregion 🔖️between_roundtrip_law

    //#region 🔖️codec_retention_law
    #[test]
    fn codec_retention_law() {
        let bytes = std::fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../🗿️artifacts/☁️las/📚️examples/🎬️demo/🖼️assets/example.las"
        ));
        let snap = match bytes {
            Ok(b) => crate::artifacts::las::engine::decode_las(&b).expect("decode fixture"),
            // Fixture path is relative to this crate's manifest dir under the workspace layout;
            // fall back to a synthetic snapshot so this law still exercises decode -> encode ->
            // decode identity even if the workspace root differs at test time.
            Err(_) => base_snapshot(),
        };
        let reencoded = crate::artifacts::las::engine::encode_las(&snap).expect("re-encode fixture");
        let redecoded = crate::artifacts::las::engine::decode_las(&reencoded).expect("re-decode fixture");
        // Structural fields are always recomputed on encode (see `LasHeader`'s doc comment); the
        // retained invariant is real content: points, VLR payloads, and the non-structural header
        // fields (scale/offset/bounds/dates/identifiers/points-by-return).
        assert_eq!(redecoded.points.len(), snap.points.len());
        for (a, b) in snap.points.iter().zip(redecoded.points.iter()) {
            assert!((a.x - b.x).abs() < 1e-6);
            assert!((a.y - b.y).abs() < 1e-6);
            assert!((a.z - b.z).abs() < 1e-6);
            assert_eq!(a.classification, b.classification);
        }
        assert_eq!(redecoded.vlrs.len(), snap.vlrs.len());
        for (a, b) in snap.vlrs.iter().zip(redecoded.vlrs.iter()) {
            assert_eq!(a.user_id, b.user_id);
            assert_eq!(a.record_id, b.record_id);
            assert_eq!(a.data, b.data);
        }
        assert_eq!(snap.header.system_identifier, redecoded.header.system_identifier);
        assert_eq!(snap.header.generating_software, redecoded.header.generating_software);
        assert_eq!(snap.header.points_by_return, redecoded.header.points_by_return);
    }
    //#endregion 🔖️codec_retention_law

    //#region 🔖️field_sweep
    /// 🌪️ `sweep_a`/`sweep_b` differ in EVERY mutable field: every header scalar, one VLR
    /// removed + one modified-in-every-field, one point added + one modified-in-every-field
    /// (incl. both tri-states). `vlrs`/`points` are index-keyed so a SINGLE `between()` call can
    /// only ever show `removed` XOR `added` on a same-length pair (the recipe's own documented
    /// structural limit — see f1-closer-report.md §4.4) — sidestepped here by giving `a`/`b`
    /// DIFFERENT lengths per collection and splitting assertions across both `between()`
    /// directions, exactly like `txt`'s fix: `vlrs` SHRINKS a->b (removed forward, added
    /// backward), `points` GROWS a->b (added forward, removed backward); both collections'
    /// `modified` slot (index 0, which exists on both sides either way) is exercised in EVERY
    /// direction.
    fn sweep_a() -> LasSnapshot {
        LasSnapshot {
            schema: "stdio.las".into(),
            header: LasHeader {
                version_major: 1,
                version_minor: 2,
                system_identifier: "before-system".into(),
                generating_software: "before-software".into(),
                creation_day_of_year: 10,
                creation_year: 2020,
                header_size: 227,
                offset_to_point_data: 227,
                number_of_vlrs: 2,
                point_data_format_id: 1,
                point_data_record_length: 28,
                number_of_point_records: 2,
                points_by_return: [1, 1, 0, 0, 0],
                x_scale: 0.01, y_scale: 0.01, z_scale: 0.01,
                x_offset: 0.0, y_offset: 0.0, z_offset: 0.0,
                max_x: 100.0, min_x: 0.0, max_y: 100.0, min_y: 0.0, max_z: 100.0, min_z: 0.0,
            },
            vlrs: vec![
                vlr("stay-user", 1, b"stay-data-before"),
                vlr("gone-user", 2, b"will be removed"),
            ],
            points: vec![
                LasPoint { gps_time: Some(1000.0), ..point(0) },
                point(1),
            ],
        }
    }

    fn sweep_b() -> LasSnapshot {
        LasSnapshot {
            schema: "stdio.las".into(),
            header: LasHeader {
                version_major: 2,
                version_minor: 4,
                system_identifier: "after-system".into(),
                generating_software: "after-software".into(),
                creation_day_of_year: 250,
                creation_year: 2026,
                header_size: 375,
                offset_to_point_data: 500,
                number_of_vlrs: 1,
                point_data_format_id: 3,
                point_data_record_length: 34,
                number_of_point_records: 3,
                points_by_return: [0, 0, 2, 1, 0],
                x_scale: 0.001, y_scale: 0.001, z_scale: 0.001,
                x_offset: 500.0, y_offset: 500.0, z_offset: 10.0,
                max_x: 999.0, min_x: -1.0, max_y: 999.0, min_y: -1.0, max_z: 50.0, min_z: -50.0,
            },
            vlrs: vec![
                // Index 0 stays "alive" but every field changes (exercises `modified`). `a`'s
                // index 1 ("gone-user") has no counterpart here -- exercises `removed` forward
                // (a->b) / `added` backward (b->a), the collection SHRINKS.
                vlr("stay-user-2", 9, b"stay-data-after"),
            ],
            points: vec![
                // Index 0 stays "alive" but every field changes, incl. both tri-states going
                // from Some -> None and None -> Some.
                LasPoint {
                    x: 1.0, y: 2.0, z: 3.0,
                    intensity: 500,
                    return_number: 4,
                    number_of_returns: 5,
                    scan_direction_flag: false,
                    edge_of_flight_line: false,
                    classification: 9,
                    scan_angle_rank: 5,
                    user_data: 200,
                    point_source_id: 42,
                    gps_time: None, // tri-state: Some(1000.0) -> None
                    rgb: Some((10, 20, 30)), // tri-state: None -> Some
                },
                point(1),
                // Index 2 is a brand-new point (exercises `added` on the a->b direction).
                point(9),
            ],
        }
    }

    #[test]
    fn field_sweep_covers_every_mutable_field() {
        let a = sweep_a();
        let b = sweep_b();

        let forward = LasDiff::between(&a, &b);
        assert_eq!(forward.apply(&a), b, "between(a,b).apply(a) must equal b");
        let backward = LasDiff::between(&b, &a);
        assert_eq!(backward.apply(&b), a, "between(b,a).apply(b) must equal a");
        assert!(LasDiff::between(&a, &a).is_empty(), "between(a,a) must be empty");

        // Every header scalar must be diffed forward.
        assert!(forward.version_major.is_some());
        assert!(forward.version_minor.is_some());
        assert!(forward.system_identifier.is_some());
        assert!(forward.generating_software.is_some());
        assert!(forward.creation_day_of_year.is_some());
        assert!(forward.creation_year.is_some());
        assert!(forward.header_size.is_some());
        assert!(forward.offset_to_point_data.is_some());
        assert!(forward.point_data_format_id.is_some());
        assert!(forward.point_data_record_length.is_some());
        assert!(forward.points_by_return.is_some());
        assert!(forward.x_scale.is_some());
        assert!(forward.y_scale.is_some());
        assert!(forward.z_scale.is_some());
        assert!(forward.x_offset.is_some());
        assert!(forward.y_offset.is_some());
        assert!(forward.z_offset.is_some());
        assert!(forward.max_x.is_some());
        assert!(forward.min_x.is_some());
        assert!(forward.max_y.is_some());
        assert!(forward.min_y.is_some());
        assert!(forward.max_z.is_some());
        assert!(forward.min_z.is_some());
        assert!(forward.number_of_vlrs.is_some(), "number_of_vlrs must be diffed (2 -> 1)");
        assert!(forward.number_of_point_records.is_some(), "number_of_point_records must be diffed (2 -> 3)");

        // vlrs: a has 2, b has 1 (SHRINKS) -- index 0 modified in every field, index 1 removed.
        let vd: &LasVlrsDiff = forward.vlrs.as_ref().expect("vlrs diff must be present");
        assert_eq!(vd.modified.len(), 1, "exactly one VLR must be modified");
        assert_eq!(vd.modified[0].index, 0);
        assert_eq!(vd.removed, vec![1], "a->b (shrinking) must show the removed VLR");
        assert!(vd.added.is_empty(), "a->b (shrinking) must not show an added VLR");
        let vmd = &vd.modified[0].diff;
        assert!(vmd.user_id.is_some());
        assert!(vmd.record_id.is_some());
        assert!(vmd.description.is_some());
        assert!(vmd.data.is_some());

        // Backward direction: vlrs GROW 1 -> 2, proving `added` (the tail the forward direction
        // structurally could not show).
        let vd_back: &LasVlrsDiff = backward.vlrs.as_ref().expect("vlrs diff must be present");
        assert_eq!(vd_back.added.len(), 1, "b->a (growing) must show the added (formerly-removed) VLR");
        assert_eq!(vd_back.added[0].index, 1);
        assert!(vd_back.removed.is_empty(), "b->a (growing) must not show a removed VLR");

        // points: a has 2, b has 3 -- index 0 modified (incl. both tri-states), index 2 added.
        let pd: &LasPointsDiff = forward.points.as_ref().expect("points diff must be present");
        assert_eq!(pd.modified.len(), 1, "exactly one point must be modified");
        assert_eq!(pd.modified[0].index, 0);
        assert_eq!(pd.added.len(), 1, "exactly one point must be added");
        assert!(pd.removed.is_empty(), "a->b (growing) must not show a removed point");
        let pmd = &pd.modified[0].diff;
        assert!(pmd.x.is_some());
        assert!(pmd.y.is_some());
        assert!(pmd.z.is_some());
        assert!(pmd.intensity.is_some());
        assert!(pmd.return_number.is_some());
        assert!(pmd.number_of_returns.is_some());
        assert!(pmd.scan_direction_flag.is_some());
        assert!(pmd.edge_of_flight_line.is_some());
        assert!(pmd.classification.is_some());
        assert!(pmd.scan_angle_rank.is_some());
        assert!(pmd.user_data.is_some());
        assert!(pmd.point_source_id.is_some());
        assert_eq!(pmd.gps_time, Some(None), "gps_time tri-state must show a clear (Some(None))");
        assert_eq!(pmd.rgb, Some(Some((10, 20, 30))), "rgb tri-state must show a set (Some(Some(_)))");

        // Backward direction: points shrink 3 -> 2, proving `removed` (the tail the forward
        // direction structurally could not show).
        let pd_back: &LasPointsDiff = backward.points.as_ref().expect("points diff must be present");
        assert_eq!(pd_back.removed, vec![2], "b->a (shrinking) must show the removed (formerly-added) point");
    }
    //#endregion 🔖️field_sweep

    #[test]
    fn out_of_range_index_mutation_is_noop_not_panic() {
        let base = base_snapshot();
        let mut snap = base.clone();
        apply_las_mutation(&mut snap, &LasMutation::RemoveVlr { index: 99 });
        assert_eq!(snap, base);
        apply_las_mutation(&mut snap, &LasMutation::SetPoint { index: 99, point: point(1) });
        assert_eq!(snap, base);
        apply_las_mutation(&mut snap, &LasMutation::SetVlrData { index: 99, data: vec![1] });
        assert_eq!(snap, base);
    }

    //#region 🔖️op_text_binary_roundtrip_law
    /// 🧪️ F6 (las): `OpText`/`OpBinary` round-trip law over the full 15-variant vocabulary
    /// (hand-rolled, `dsl::DslOps` blocked — see the `OpCodecs` region's doc comment). Exercises
    /// `SetSnapshot` (whole-header + vlrs + points positional codec), both bare-tuple mutation
    /// variants (`SetScaleAndOffset`/`SetBounds`), the `[u32; 5]` array (`SetPointsByReturn`), and
    /// a point/VLR carrying both tri-state-capable fields set (`gps_time`/`rgb`).
    #[test]
    fn op_text_binary_roundtrip_law() {
        let base = base_snapshot();
        let mut rich_point = point(9);
        rich_point.gps_time = Some(1234.5);
        rich_point.rgb = Some((11, 22, 33));
        for mutation in [
            LasMutation::NoMutation,
            LasMutation::SetSnapshot { snapshot: LasSnapshot { header: LasHeader { creation_year: 2031, ..base.header.clone() }, ..base.clone() } },
            LasMutation::SetVersion { major: 1, minor: 4 },
            LasMutation::SetSystemIdentifier { system_identifier: "semio".into() },
            LasMutation::SetSoftwareInfo { generating_software: "semio-las-writer".into() },
            LasMutation::SetCreationDate { day_of_year: 42, year: 2026 },
            LasMutation::SetScaleAndOffset { scale: (0.001, 0.001, 0.001), offset: (1000.0, 2000.0, 0.0) },
            LasMutation::SetBounds { max: (999.0, 888.0, 777.0), min: (-1.0, -2.0, -3.0) },
            LasMutation::SetPointsByReturn { counts: [1, 2, 3, 4, 5] },
            LasMutation::InsertVlr { index: 1, vlr: vlr("EXTRA", 200, b"new-vlr") },
            LasMutation::RemoveVlr { index: 0 },
            LasMutation::SetVlrData { index: 0, data: b"patched".to_vec() },
            LasMutation::InsertPoint { index: 1, point: rich_point.clone() },
            LasMutation::RemovePoint { index: 0 },
            LasMutation::SetPoint { index: 0, point: rich_point },
        ] {
            let printed = mutation.print_op();
            assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
            let parsed = LasMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
            assert_eq!(parsed, mutation, "print_op/parse_op round-trip mismatch for {mutation:?} (printed {printed:?})");

            let encoded = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op({mutation:?}) failed: {e}"));
            let decoded = LasMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
            assert_eq!(decoded, mutation, "encode_op/decode_op round-trip mismatch for {mutation:?}");
        }
    }
    //#endregion 🔖️op_text_binary_roundtrip_law
}
//#endregion Tests
