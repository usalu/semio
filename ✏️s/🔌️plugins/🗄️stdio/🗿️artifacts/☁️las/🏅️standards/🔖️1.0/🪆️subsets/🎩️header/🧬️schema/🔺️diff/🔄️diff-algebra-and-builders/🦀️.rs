//! 🔄️ Diff algebra and builders of the LAS diff: the header diff application, the mutation diff and diff algebra implementations and the typed diff constructors. See [`super`].

use super::*;

//#region 🔖️Diff
/// ▶️ Applies every header scalar patch onto `header` in place.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_header_diff(header: &mut LasHeader, d: &LasDiff) {
    if let Some(v) = d.version_major {
        header.version_major = v;
    }
    if let Some(v) = d.version_minor {
        header.version_minor = v;
    }
    if let Some(v) = &d.system_identifier {
        header.system_identifier = v.clone();
    }
    if let Some(v) = &d.generating_software {
        header.generating_software = v.clone();
    }
    if let Some(v) = d.creation_day_of_year {
        header.creation_day_of_year = v;
    }
    if let Some(v) = d.creation_year {
        header.creation_year = v;
    }
    if let Some(v) = d.header_size {
        header.header_size = v;
    }
    if let Some(v) = d.offset_to_point_data {
        header.offset_to_point_data = v;
    }
    if let Some(v) = d.number_of_vlrs {
        header.number_of_vlrs = v;
    }
    if let Some(v) = d.point_data_format_id {
        header.point_data_format_id = v;
    }
    if let Some(v) = d.point_data_record_length {
        header.point_data_record_length = v;
    }
    if let Some(v) = d.number_of_point_records {
        header.number_of_point_records = v;
    }
    if let Some(v) = d.points_by_return {
        header.points_by_return = v;
    }
    if let Some(v) = d.x_scale {
        header.x_scale = v;
    }
    if let Some(v) = d.y_scale {
        header.y_scale = v;
    }
    if let Some(v) = d.z_scale {
        header.z_scale = v;
    }
    if let Some(v) = d.x_offset {
        header.x_offset = v;
    }
    if let Some(v) = d.y_offset {
        header.y_offset = v;
    }
    if let Some(v) = d.z_offset {
        header.z_offset = v;
    }
    if let Some(v) = d.max_x {
        header.max_x = v;
    }
    if let Some(v) = d.min_x {
        header.min_x = v;
    }
    if let Some(v) = d.max_y {
        header.max_y = v;
    }
    if let Some(v) = d.min_y {
        header.min_y = v;
    }
    if let Some(v) = d.max_z {
        header.max_z = v;
    }
    if let Some(v) = d.min_z {
        header.min_z = v;
    }
}

impl MutationDiff<LasSnapshot> for LasDiff {}

impl protocol::ColdMutationDiff<LasSnapshot> for LasDiff {
    /// ➕️ Structural, total, base-free sequential-coalesce (`## Absorb` contract). Header
    /// scalars: LWW. `vlrs`/`points`: index-transport via [`absorb_indexed_triple`].
    fn absorb(&mut self, other: Self) {
        if other.version_major.is_some() {
            self.version_major = other.version_major;
        }
        if other.version_minor.is_some() {
            self.version_minor = other.version_minor;
        }
        if other.system_identifier.is_some() {
            self.system_identifier = other.system_identifier;
        }
        if other.generating_software.is_some() {
            self.generating_software = other.generating_software;
        }
        if other.creation_day_of_year.is_some() {
            self.creation_day_of_year = other.creation_day_of_year;
        }
        if other.creation_year.is_some() {
            self.creation_year = other.creation_year;
        }
        if other.header_size.is_some() {
            self.header_size = other.header_size;
        }
        if other.offset_to_point_data.is_some() {
            self.offset_to_point_data = other.offset_to_point_data;
        }
        if other.number_of_vlrs.is_some() {
            self.number_of_vlrs = other.number_of_vlrs;
        }
        if other.point_data_format_id.is_some() {
            self.point_data_format_id = other.point_data_format_id;
        }
        if other.point_data_record_length.is_some() {
            self.point_data_record_length = other.point_data_record_length;
        }
        if other.number_of_point_records.is_some() {
            self.number_of_point_records = other.number_of_point_records;
        }
        if other.points_by_return.is_some() {
            self.points_by_return = other.points_by_return;
        }
        if other.x_scale.is_some() {
            self.x_scale = other.x_scale;
        }
        if other.y_scale.is_some() {
            self.y_scale = other.y_scale;
        }
        if other.z_scale.is_some() {
            self.z_scale = other.z_scale;
        }
        if other.x_offset.is_some() {
            self.x_offset = other.x_offset;
        }
        if other.y_offset.is_some() {
            self.y_offset = other.y_offset;
        }
        if other.z_offset.is_some() {
            self.z_offset = other.z_offset;
        }
        if other.max_x.is_some() {
            self.max_x = other.max_x;
        }
        if other.min_x.is_some() {
            self.min_x = other.min_x;
        }
        if other.max_y.is_some() {
            self.max_y = other.max_y;
        }
        if other.min_y.is_some() {
            self.min_y = other.min_y;
        }
        if other.max_z.is_some() {
            self.max_z = other.max_z;
        }
        if other.min_z.is_some() {
            self.min_z = other.min_z;
        }
        self.vlrs = absorb_vlrs(self.vlrs.take(), other.vlrs);
        self.points = absorb_points(self.points.take(), other.points);
    }

    fn apply(&self, unpublished: &mut LasSnapshot, _capability: protocol::ApplyCapability) -> MutationApplyResult<()> {
        let base = &*unpublished;
        if let Some(diff) = &self.vlrs {
            validate_indexed_targets(base.vlrs.len(), &diff.removed, diff.modified.iter().map(|value| value.index), diff.added.iter().map(|value| value.index), "vlrs")?;
        }
        if let Some(diff) = &self.points {
            validate_indexed_targets(base.points.len(), &diff.removed, diff.modified.iter().map(|value| value.index), diff.added.iter().map(|value| value.index), "points")?;
        }
        let mut header = base.header.clone();
        apply_header_diff(&mut header, self);
        let vlrs = match &self.vlrs {
            Some(vd) => vd.apply_unchecked(&base.vlrs),
            None => base.vlrs.clone(),
        };
        let points = match &self.points {
            Some(pd) => pd.apply_unchecked(&base.points),
            None => base.points.clone(),
        };
        *unpublished = LasSnapshot { schema: base.schema.clone(), header, vlrs, points };
        Ok(())
    }
}

impl DiffAlgebra<LasSnapshot> for LasDiff {
    fn is_empty(&self) -> bool {
        self == &LasDiff::default()
    }
}

impl protocol::ColdDiffAlgebra<LasSnapshot> for LasDiff {
    /// 🔁️ The negative diff: every touched header scalar restores its base value and the `vlrs`/`points` triples invert against the
    /// base rows.
    fn inverse(&self, base: &LasSnapshot) -> Self {
        let header = &base.header;
        LasDiff {
            version_major: self.version_major.map(|_| header.version_major),
            version_minor: self.version_minor.map(|_| header.version_minor),
            system_identifier: self.system_identifier.as_ref().map(|_| header.system_identifier.clone()),
            generating_software: self.generating_software.as_ref().map(|_| header.generating_software.clone()),
            creation_day_of_year: self.creation_day_of_year.map(|_| header.creation_day_of_year),
            creation_year: self.creation_year.map(|_| header.creation_year),
            header_size: self.header_size.map(|_| header.header_size),
            offset_to_point_data: self.offset_to_point_data.map(|_| header.offset_to_point_data),
            number_of_vlrs: self.number_of_vlrs.map(|_| header.number_of_vlrs),
            point_data_format_id: self.point_data_format_id.map(|_| header.point_data_format_id),
            point_data_record_length: self.point_data_record_length.map(|_| header.point_data_record_length),
            number_of_point_records: self.number_of_point_records.map(|_| header.number_of_point_records),
            points_by_return: self.points_by_return.map(|_| header.points_by_return),
            x_scale: self.x_scale.map(|_| header.x_scale),
            y_scale: self.y_scale.map(|_| header.y_scale),
            z_scale: self.z_scale.map(|_| header.z_scale),
            x_offset: self.x_offset.map(|_| header.x_offset),
            y_offset: self.y_offset.map(|_| header.y_offset),
            z_offset: self.z_offset.map(|_| header.z_offset),
            max_x: self.max_x.map(|_| header.max_x),
            min_x: self.min_x.map(|_| header.min_x),
            max_y: self.max_y.map(|_| header.max_y),
            min_y: self.min_y.map(|_| header.min_y),
            max_z: self.max_z.map(|_| header.max_z),
            min_z: self.min_z.map(|_| header.min_z),
            vlrs: self.vlrs.as_ref().map(|vlrs| vlrs.inverse(&base.vlrs)),
            points: self.points.as_ref().map(|points| points.inverse(&base.points)),
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_version(major: u8, minor: u8) -> LasDiff {
    LasDiff { version_major: Some(major), version_minor: Some(minor), ..Default::default() }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_system_identifier(system_identifier: &str) -> LasDiff {
    LasDiff { system_identifier: Some(system_identifier.to_string()), ..Default::default() }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_software_info(generating_software: &str) -> LasDiff {
    LasDiff { generating_software: Some(generating_software.to_string()), ..Default::default() }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_creation_date(day_of_year: u16, year: u16) -> LasDiff {
    LasDiff { creation_day_of_year: Some(day_of_year), creation_year: Some(year), ..Default::default() }
}

/// 📏️ One axis of [`diff_set_scale_and_offset`]: the point RECORD this coordinate was decoded
/// from, re-read under the new scale/offset. `record = round((value - offset) / scale)` is the
/// exact inverse of the LAS public-header rule `coordinate = record * scale + offset`, so the
/// integer this returns to is the one on disk. A zero `from_scale` is not a legal LAS header and
/// carries no record to preserve, so the coordinate is left alone rather than turned into NaN.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn coordinate_under(value: f64, from_scale: f64, from_offset: f64, to_scale: f64, to_offset: f64) -> f64 {
    if from_scale == 0.0 {
        return value;
    }
    ((value - from_offset) / from_scale).round() * to_scale + to_offset
}

/// 📏️ Sets the six public-header-block fields that state HOW the on-disk integer point records
/// are turned into real-world coordinates — and changes nothing else about the file's contents.
///
/// ⚠️ This snapshot stores each point in REAL-WORLD coordinates and re-derives its integer record
/// on encode, which is the opposite primacy from the file itself: LAS §"Public Header Block"
/// defines `coordinate = record * scale + offset`, so the record is what the document carries and
/// the coordinate is what is computed from it. Writing only the six header fields therefore used
/// to hold the COORDINATES fixed and silently rewrite all 8,448 point records — a re-quantization,
/// not the header edit this kind is named for. That reading is also lossy in one direction: moving
/// to a COARSER scale rounds every coordinate away and `LasMutation::inverse`'s "put the old scale
/// and offset back" cannot bring it back, so the inverse law held only for rows that happen to
/// refine. The mutation now does what its name says — the records stay exactly where they are and
/// every coordinate is re-read from its own record under the new parameters, which is lossless and
/// exactly invertible for any scale in either direction. Reproduced by
/// `mutate-las-1-0::mutate-set-scale-and-offset` in the parity phase, where the reference
/// (`las::raw`, whose model IS the record) reported `$.points[1].x` 583000.246 against our
/// 583000.491 across 24,320 differences (ticket `26/08/23/END-TO-END-TESTING-REFACTOR`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_scale_and_offset(base: &LasSnapshot, scale: (f64, f64, f64), offset: (f64, f64, f64)) -> LasDiff {
    let modified: Vec<LasPointModified> = base
        .points
        .iter()
        .enumerate()
        .filter_map(|(index, point)| {
            let held = LasPoint {
                x: coordinate_under(point.x, base.header.x_scale, base.header.x_offset, scale.0, offset.0),
                y: coordinate_under(point.y, base.header.y_scale, base.header.y_offset, scale.1, offset.1),
                z: coordinate_under(point.z, base.header.z_scale, base.header.z_offset, scale.2, offset.2),
                ..point.clone()
            };
            let d = changed_point_fields(point, &held);
            (d != LasPointDiff::default()).then_some(LasPointModified { index, diff: d })
        })
        .collect();
    LasDiff {
        x_scale: Some(scale.0),
        y_scale: Some(scale.1),
        z_scale: Some(scale.2),
        x_offset: Some(offset.0),
        y_offset: Some(offset.1),
        z_offset: Some(offset.2),
        points: (!modified.is_empty()).then_some(LasPointsDiff { removed: vec![], modified, added: vec![] }),
        ..Default::default()
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_bounds(max: (f64, f64, f64), min: (f64, f64, f64)) -> LasDiff {
    LasDiff { max_x: Some(max.0), max_y: Some(max.1), max_z: Some(max.2), min_x: Some(min.0), min_y: Some(min.1), min_z: Some(min.2), ..Default::default() }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_points_by_return(counts: [u32; 5]) -> LasDiff {
    LasDiff { points_by_return: Some(counts), ..Default::default() }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_insert_vlr(base: &LasSnapshot, index: usize, vlr: LasVlr) -> LasDiff {
    LasDiff { number_of_vlrs: Some((base.vlrs.len() + 1) as u32), vlrs: Some(LasVlrsDiff { removed: vec![], modified: vec![], added: vec![LasVlrAdded { index, vlr }] }), ..Default::default() }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_vlr(base: &LasSnapshot, index: usize) -> LasDiff {
    LasDiff { number_of_vlrs: Some(base.vlrs.len().saturating_sub(1) as u32), vlrs: Some(LasVlrsDiff { removed: vec![index], modified: vec![], added: vec![] }), ..Default::default() }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_vlr_data(index: usize, data: Vec<u8>) -> LasDiff {
    LasDiff { vlrs: Some(LasVlrsDiff { removed: vec![], modified: vec![LasVlrModified { index, diff: LasVlrDiff { data: Some(data), ..Default::default() } }], added: vec![] }), ..Default::default() }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_insert_point(base: &LasSnapshot, index: usize, point: LasPoint) -> LasDiff {
    LasDiff { number_of_point_records: Some((base.points.len() + 1) as u32), points: Some(LasPointsDiff { removed: vec![], modified: vec![], added: vec![LasPointAdded { index, point }] }), ..Default::default() }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_point(base: &LasSnapshot, index: usize) -> LasDiff {
    LasDiff { number_of_point_records: Some(base.points.len().saturating_sub(1) as u32), points: Some(LasPointsDiff { removed: vec![index], modified: vec![], added: vec![] }), ..Default::default() }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_point(base: &LasSnapshot, index: usize, point: &LasPoint) -> LasDiff {
    let d = changed_point_fields(base.points.get(index).unwrap_or(&LasPoint::default()), point);
    if base.points.get(index).is_some() && d == LasPointDiff::default() {
        return LasDiff::default();
    }
    LasDiff { points: Some(LasPointsDiff { removed: vec![], modified: vec![LasPointModified { index, diff: d }], added: vec![] }), ..Default::default() }
}
//#endregion 🔖️Diff
