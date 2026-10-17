//! 📍️ Point diff of the LAS diff: the sparse point diff, its rows and the absorb and inverse of the point collection. See [`super`].

use super::*;

//#region 🔖️PointDiff
/// 📍️ Sparse per-field patch for one `LasPoint`. `gps_time`/`rgb` are tri-state:
/// `None` = unchanged, `Some(None)` = cleared (point demoted out of a format that carries it),
/// `Some(Some(v))` = set.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct LasPointDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub z: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub intensity: Option<u16>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub return_number: Option<u8>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub number_of_returns: Option<u8>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub scan_direction_flag: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub edge_of_flight_line: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub classification: Option<u8>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub scan_angle_rank: Option<i8>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub user_data: Option<u8>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub point_source_id: Option<u16>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub gps_time: Option<Option<f64>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub rgb: Option<Option<(u16, u16, u16)>>,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_point_diff(p: &mut LasPoint, diff: &LasPointDiff) {
    if let Some(v) = diff.x {
        p.x = v;
    }
    if let Some(v) = diff.y {
        p.y = v;
    }
    if let Some(v) = diff.z {
        p.z = v;
    }
    if let Some(v) = diff.intensity {
        p.intensity = v;
    }
    if let Some(v) = diff.return_number {
        p.return_number = v;
    }
    if let Some(v) = diff.number_of_returns {
        p.number_of_returns = v;
    }
    if let Some(v) = diff.scan_direction_flag {
        p.scan_direction_flag = v;
    }
    if let Some(v) = diff.edge_of_flight_line {
        p.edge_of_flight_line = v;
    }
    if let Some(v) = diff.classification {
        p.classification = v;
    }
    if let Some(v) = diff.scan_angle_rank {
        p.scan_angle_rank = v;
    }
    if let Some(v) = diff.user_data {
        p.user_data = v;
    }
    if let Some(v) = diff.point_source_id {
        p.point_source_id = v;
    }
    if let Some(v) = diff.gps_time {
        p.gps_time = v;
    }
    if let Some(v) = diff.rgb {
        p.rgb = v;
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn point_inverse(diff: &LasPointDiff, base: &LasPoint) -> LasPointDiff {
    LasPointDiff {
        x: diff.x.map(|_| base.x),
        y: diff.y.map(|_| base.y),
        z: diff.z.map(|_| base.z),
        intensity: diff.intensity.map(|_| base.intensity),
        return_number: diff.return_number.map(|_| base.return_number),
        number_of_returns: diff.number_of_returns.map(|_| base.number_of_returns),
        scan_direction_flag: diff.scan_direction_flag.map(|_| base.scan_direction_flag),
        edge_of_flight_line: diff.edge_of_flight_line.map(|_| base.edge_of_flight_line),
        classification: diff.classification.map(|_| base.classification),
        scan_angle_rank: diff.scan_angle_rank.map(|_| base.scan_angle_rank),
        user_data: diff.user_data.map(|_| base.user_data),
        point_source_id: diff.point_source_id.map(|_| base.point_source_id),
        gps_time: diff.gps_time.map(|_| base.gps_time),
        rgb: diff.rgb.map(|_| base.rgb),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn changed_point_fields(a: &LasPoint, b: &LasPoint) -> LasPointDiff {
    LasPointDiff {
        x: (a.x != b.x).then_some(b.x),
        y: (a.y != b.y).then_some(b.y),
        z: (a.z != b.z).then_some(b.z),
        intensity: (a.intensity != b.intensity).then_some(b.intensity),
        return_number: (a.return_number != b.return_number).then_some(b.return_number),
        number_of_returns: (a.number_of_returns != b.number_of_returns).then_some(b.number_of_returns),
        scan_direction_flag: (a.scan_direction_flag != b.scan_direction_flag).then_some(b.scan_direction_flag),
        edge_of_flight_line: (a.edge_of_flight_line != b.edge_of_flight_line).then_some(b.edge_of_flight_line),
        classification: (a.classification != b.classification).then_some(b.classification),
        scan_angle_rank: (a.scan_angle_rank != b.scan_angle_rank).then_some(b.scan_angle_rank),
        user_data: (a.user_data != b.user_data).then_some(b.user_data),
        point_source_id: (a.point_source_id != b.point_source_id).then_some(b.point_source_id),
        gps_time: (a.gps_time != b.gps_time).then_some(b.gps_time),
        rgb: (a.rgb != b.rgb).then_some(b.rgb),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_point_rows(base: &mut LasPointDiff, other: &LasPointDiff) {
    if other.x.is_some() {
        base.x = other.x;
    }
    if other.y.is_some() {
        base.y = other.y;
    }
    if other.z.is_some() {
        base.z = other.z;
    }
    if other.intensity.is_some() {
        base.intensity = other.intensity;
    }
    if other.return_number.is_some() {
        base.return_number = other.return_number;
    }
    if other.number_of_returns.is_some() {
        base.number_of_returns = other.number_of_returns;
    }
    if other.scan_direction_flag.is_some() {
        base.scan_direction_flag = other.scan_direction_flag;
    }
    if other.edge_of_flight_line.is_some() {
        base.edge_of_flight_line = other.edge_of_flight_line;
    }
    if other.classification.is_some() {
        base.classification = other.classification;
    }
    if other.scan_angle_rank.is_some() {
        base.scan_angle_rank = other.scan_angle_rank;
    }
    if other.user_data.is_some() {
        base.user_data = other.user_data;
    }
    if other.point_source_id.is_some() {
        base.point_source_id = other.point_source_id;
    }
    if other.gps_time.is_some() {
        base.gps_time = other.gps_time;
    }
    if other.rgb.is_some() {
        base.rgb = other.rgb;
    }
}

/// 📍️ One `points.modified[]` entity — `index` is the point's position in BASE.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct LasPointModified {
    pub index: usize,
    pub diff: LasPointDiff,
}

/// 📍️ One `points.added[]` entity — `index` is the point's position in the FINAL sequence.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct LasPointAdded {
    pub index: usize,
    pub point: LasPoint,
}

/// 📍️ Sparse index-keyed `points` triple.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct LasPointsDiff {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<LasPointModified>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<LasPointAdded>,
}

impl LasPointsDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(super) fn apply_unchecked(&self, base: &[LasPoint]) -> Vec<LasPoint> {
        let mut items = base.to_vec();
        for m in &self.modified {
            apply_point_diff(&mut items[m.index], &m.diff);
        }
        let mut removed = self.removed.clone();
        removed.sort_unstable_by(|a, b| b.cmp(a));
        for index in removed {
            items.remove(index);
        }
        let mut added = self.added.clone();
        added.sort_by_key(|a| a.index);
        for a in added {
            items.insert(a.index, a.point);
        }
        items
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn apply(&self, base: &[LasPoint]) -> MutationApplyResult<Vec<LasPoint>> {
        validate_indexed_targets(base.len(), &self.removed, self.modified.iter().map(|value| value.index), self.added.iter().map(|value| value.index), "points")?;
        Ok(self.apply_unchecked(base))
    }

    /// ↩️ The negative triple against the base points.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(super) fn inverse(&self, base: &[LasPoint]) -> Self {
        let modified: Vec<(usize, LasPointDiff)> = self.modified.iter().map(|m| (m.index, m.diff.clone())).collect();
        let added: Vec<(usize, LasPoint)> = self.added.iter().map(|a| (a.index, a.point.clone())).collect();
        let (removed, modified, added) = inverse_indexed_triple(&self.removed, &modified, &added, base, point_inverse);
        Self { removed, modified: modified.into_iter().map(|(index, diff)| LasPointModified { index, diff }).collect(), added: added.into_iter().map(|(index, point)| LasPointAdded { index, point }).collect() }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn absorb_points(d1: Option<LasPointsDiff>, d2: Option<LasPointsDiff>) -> Option<LasPointsDiff> {
    let (d1, d2) = match (d1, d2) {
        (None, None) => return None,
        (Some(d1), None) => return Some(d1),
        (None, Some(d2)) => return Some(d2),
        (Some(d1), Some(d2)) => (d1, d2),
    };
    let d1m: Vec<(usize, LasPointDiff)> = d1.modified.iter().map(|m| (m.index, m.diff.clone())).collect();
    let d1a: Vec<(usize, LasPoint)> = d1.added.iter().map(|a| (a.index, a.point.clone())).collect();
    let d2m: Vec<(usize, LasPointDiff)> = d2.modified.iter().map(|m| (m.index, m.diff.clone())).collect();
    let d2a: Vec<(usize, LasPoint)> = d2.added.iter().map(|a| (a.index, a.point.clone())).collect();
    let (removed, modified, added) = absorb_indexed_triple((&d1.removed, &d1m, &d1a), (&d2.removed, &d2m, &d2a), |base, other| absorb_point_rows(base, &other), apply_point_diff);
    let merged = LasPointsDiff { removed, modified: modified.into_iter().map(|(index, diff)| LasPointModified { index, diff }).collect(), added: added.into_iter().map(|(index, point)| LasPointAdded { index, point }).collect() };
    if merged.is_empty() {
        None
    } else {
        Some(merged)
    }
}
//#endregion 🔖️PointDiff
