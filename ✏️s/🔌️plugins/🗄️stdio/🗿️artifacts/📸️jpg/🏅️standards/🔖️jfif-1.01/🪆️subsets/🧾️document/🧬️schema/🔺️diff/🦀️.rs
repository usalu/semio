//! 🔺️ Sparse owned JPEG content edits and position-aware metadata composition.

use crate::schema::snapshot::{JfifDensityUnits, JfifThumbnail, JpgSegment};
use crate::JpgSnapshot;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use std::collections::{BTreeMap, HashMap};

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct JpgSegmentDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<u8>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<u8>>,
}
impl JpgSegmentDiff {
    fn is_empty(&self) -> bool {
        self == &Self::default()
    }
    fn apply_patch(&self, base: &JpgSegment) -> JpgSegment {
        JpgSegment { marker: self.marker.unwrap_or(base.marker), data: self.data.clone().unwrap_or_else(|| base.data.clone()) }
    }
    fn fields_changed(a: &JpgSegment, b: &JpgSegment) -> Self {
        Self { marker: (a.marker != b.marker).then_some(b.marker), data: (a.data != b.data).then(|| b.data.clone()) }
    }
    fn absorb(&mut self, other: Self) {
        if other.marker.is_some() {
            self.marker = other.marker;
        }
        if other.data.is_some() {
            self.data = other.data;
        }
    }
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct JpgSegmentModified {
    pub index: usize,
    pub diff: JpgSegmentDiff,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct JpgSegmentAdded {
    pub index: usize,
    pub item: JpgSegment,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct JpgOtherSegmentsDiff {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<JpgSegmentModified>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<JpgSegmentAdded>,
}

#[derive(Clone, Copy, Debug)]
enum Slot {
    Base(usize),
    Added(usize),
}

fn simulate_slots(len: usize, removed: &[usize], added_indices: &[usize]) -> Vec<Slot> {
    let mut slots: Vec<Slot> = (0..len).map(Slot::Base).collect();
    let removed_desc = semio_s_artifact_stdio_contract::ordered_unique_descending(&removed);
    for r in removed_desc {
        if r < slots.len() {
            slots.remove(r);
        }
    }
    let mut order: Vec<usize> = (0..added_indices.len()).collect();
    order.sort_by_key(|&i| added_indices[i]);
    for i in order {
        let at = added_indices[i].min(slots.len());
        slots.insert(at, Slot::Added(i));
    }
    slots
}

fn base_len_hint(removed: &[usize], modified_indices: impl Iterator<Item = usize>, added_indices: impl Iterator<Item = usize>) -> usize {
    removed.iter().copied().chain(modified_indices).chain(added_indices).max().map_or(0, |m| m + 1)
}

fn absorb_other_segments(d1: JpgOtherSegmentsDiff, d2: JpgOtherSegmentsDiff) -> JpgOtherSegmentsDiff {
    let d1_added_indices: Vec<usize> = d1.added.iter().map(|a| a.index).collect();
    let removed_count = {
        let r = semio_s_artifact_stdio_contract::ordered_unique(&d1.removed);
        r.len()
    };
    let needed_mid_len = d2.removed.iter().copied().chain(d2.modified.iter().map(|m| m.index)).max().map_or(0, |m| m + 1);
    let base_len = base_len_hint(&d1.removed, d1.modified.iter().map(|m| m.index), d1_added_indices.iter().copied()).max((needed_mid_len + removed_count).saturating_sub(d1.added.len()));
    let mid_slots = simulate_slots(base_len, &d1.removed, &d1_added_indices);

    let mut final_removed: Vec<usize> = d1.removed;
    let mut modified_map: BTreeMap<usize, JpgSegmentDiff> = d1.modified.into_iter().map(|m| (m.index, m.diff)).collect();
    let mut added_alive: Vec<Option<JpgSegmentAdded>> = d1.added.into_iter().map(Some).collect();

    for mid_idx in &d2.removed {
        match mid_slots.get(*mid_idx) {
            Some(Slot::Base(b)) => {
                final_removed.push(*b);
                modified_map.remove(b);
            }
            Some(Slot::Added(ai)) => {
                added_alive[*ai] = None;
            }
            None => {}
        }
    }
    for m2 in &d2.modified {
        match mid_slots.get(m2.index) {
            Some(Slot::Base(b)) => {
                modified_map.entry(*b).or_default().absorb(m2.diff.clone());
            }
            Some(Slot::Added(ai)) => {
                if let Some(a) = added_alive[*ai].as_mut() {
                    a.item = m2.diff.apply_patch(&a.item);
                }
            }
            None => {}
        }
    }

    final_removed.sort_unstable();
    final_removed.dedup();
    for r in &final_removed {
        modified_map.remove(r);
    }
    let mut final_modified: Vec<JpgSegmentModified> = modified_map.into_iter().filter(|(_, d)| !d.is_empty()).map(|(index, diff)| JpgSegmentModified { index, diff }).collect();
    final_modified.sort_by_key(|m| m.index);

    let alive_mid_positions: Vec<usize> = mid_slots
        .iter()
        .enumerate()
        .filter_map(|(pos, slot)| match slot {
            Slot::Added(ai) if added_alive[*ai].is_some() => Some(pos),
            _ => None,
        })
        .collect();
    let d2_added_indices: Vec<usize> = d2.added.iter().map(|a| a.index).collect();
    let mid_len = d2.removed.iter().copied().chain(d2.modified.iter().map(|m| m.index)).chain(alive_mid_positions.iter().copied()).chain(d2_added_indices.iter().copied()).max().map_or(0, |m| m + 1);
    let after_slots = simulate_slots(mid_len, &d2.removed, &d2_added_indices);
    let mut mid_to_after: HashMap<usize, usize> = HashMap::new();
    for (pos, slot) in after_slots.iter().enumerate() {
        if let Slot::Base(m) = slot {
            mid_to_after.insert(*m, pos);
        }
    }

    let mut final_added: Vec<JpgSegmentAdded> = Vec::new();
    for (ai, alive) in added_alive.into_iter().enumerate() {
        if let Some(added) = alive {
            let mid_pos = mid_slots.iter().position(|s| matches!(s, Slot::Added(idx) if *idx == ai)).expect("added_alive index always has a corresponding mid slot");
            if let Some(after_pos) = mid_to_after.get(&mid_pos) {
                final_added.push(JpgSegmentAdded { index: *after_pos, item: added.item });
            }
        }
    }
    for a2 in d2.added {
        final_added.push(a2);
    }
    final_added.sort_by_key(|a| a.index);

    JpgOtherSegmentsDiff { removed: final_removed, modified: final_modified, added: final_added }
}

fn absorb_other_segments_opt(base: &mut Option<JpgOtherSegmentsDiff>, other: Option<JpgOtherSegmentsDiff>) {
    match (base.take(), other) {
        (None, o) => *base = o,
        (Some(b), None) => *base = Some(b),
        (Some(b), Some(o)) => *base = Some(absorb_other_segments(b, o)),
    }
}

fn apply_other_segments(base: &[JpgSegment], d: &JpgOtherSegmentsDiff) -> Vec<JpgSegment> {
    let mut items = base.to_vec();
    for m in &d.modified {
        if let Some(it) = items.get_mut(m.index) {
            *it = m.diff.apply_patch(it);
        }
    }
    let removed_desc = semio_s_artifact_stdio_contract::ordered_unique_descending(&d.removed);
    for idx in removed_desc {
        if idx < items.len() {
            items.remove(idx);
        }
    }
    let mut adds = d.added.clone();
    adds.sort_by_key(|a| a.index);
    for a in adds {
        let at = a.index.min(items.len());
        items.insert(at, a.item);
    }
    items
}

fn changed_other_segments(a: &[JpgSegment], b: &[JpgSegment]) -> Option<JpgOtherSegmentsDiff> {
    let min = a.len().min(b.len());
    let mut modified = Vec::new();
    for i in 0..min {
        if a[i] != b[i] {
            let d = JpgSegmentDiff::fields_changed(&a[i], &b[i]);
            if !d.is_empty() {
                modified.push(JpgSegmentModified { index: i, diff: d });
            }
        }
    }
    let removed: Vec<usize> = (min..a.len()).collect();
    let added: Vec<JpgSegmentAdded> = (min..b.len()).map(|i| JpgSegmentAdded { index: i, item: b[i].clone() }).collect();
    if removed.is_empty() && modified.is_empty() && added.is_empty() {
        None
    } else {
        Some(JpgOtherSegmentsDiff { removed, modified, added })
    }
}

/// ↩️ Negative rows for the other-segment triple against its BASE segments: added rows become removals at their final index, removed rows return at their
/// base index and each modified row restores its base marker and data at the index the row has after the diff. Every list comes back ascending.
fn inverse_other_segments(diff: &JpgOtherSegmentsDiff, base: &[JpgSegment]) -> JpgOtherSegmentsDiff {
    let removed_sorted = semio_s_artifact_stdio_contract::ordered_unique(&diff.removed);
    let mut added_final: Vec<usize> = diff.added.iter().map(|added| added.index).collect();
    added_final.sort_unstable();
    let after_index = |index: usize| {
        let survivor = index - removed_sorted.iter().filter(|dropped| **dropped < index).count();
        added_final.iter().fold(survivor, |position, inserted| if *inserted <= position { position + 1 } else { position })
    };
    let mut modified: Vec<JpgSegmentModified> = diff
        .modified
        .iter()
        .filter_map(|row| {
            let segment = base.get(row.index)?;
            let restore = JpgSegmentDiff { marker: row.diff.marker.filter(|marker| *marker != segment.marker).map(|_| segment.marker), data: row.diff.data.as_ref().filter(|data| **data != segment.data).map(|_| segment.data.clone()) };
            (!restore.is_empty()).then_some(JpgSegmentModified { index: after_index(row.index), diff: restore })
        })
        .collect();
    modified.sort_by_key(|row| row.index);
    let added = removed_sorted.iter().filter_map(|index| base.get(*index).map(|segment| JpgSegmentAdded { index: *index, item: segment.clone() })).collect();
    JpgOtherSegmentsDiff { removed: added_final, modified, added }
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.jpg.diff")]
pub struct JpgDiff {
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub pixels: Option<Vec<u8>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub jfif_version: Option<(u8, u8)>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub jfif_density_units: Option<JfifDensityUnits>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub jfif_x_density: Option<u16>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub jfif_y_density: Option<u16>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub jfif_thumbnail: Option<Option<JfifThumbnail>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub other_segments: Option<JpgOtherSegmentsDiff>,
}

impl MutationDiff<JpgSnapshot> for JpgDiff {
    fn apply(&self, base: &JpgSnapshot, _capability: protocol::ApplyCapability) -> MutationApplyResult<JpgSnapshot> {
        if let Some(segments) = &self.other_segments {
            validate_jpg_indexed(base.image.other_segments.len(), &segments.removed, segments.modified.iter().map(|entry| entry.index), segments.added.iter().map(|entry| entry.index), ["otherSegments"])?;
        }
        let mut next = base.clone();
        if let Some(v) = self.width {
            next.image.width = v;
        }
        if let Some(v) = self.height {
            next.image.height = v;
        }
        if let Some(v) = &self.pixels {
            next.image.pixels = v.clone();
        }
        if let Some(v) = self.jfif_version {
            next.image.jfif_version = v;
        }
        if let Some(v) = self.jfif_density_units {
            next.image.jfif_density_units = v;
        }
        if let Some(v) = self.jfif_x_density {
            next.image.jfif_x_density = v;
        }
        if let Some(v) = self.jfif_y_density {
            next.image.jfif_y_density = v;
        }
        if let Some(v) = &self.jfif_thumbnail {
            next.image.jfif_thumbnail = v.clone();
        }
        if let Some(od) = &self.other_segments {
            next.image.other_segments = apply_other_segments(&next.image.other_segments, od);
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        if other.width.is_some() {
            self.width = other.width;
        }
        if other.height.is_some() {
            self.height = other.height;
        }
        if other.pixels.is_some() {
            self.pixels = other.pixels;
        }
        if other.jfif_version.is_some() {
            self.jfif_version = other.jfif_version;
        }
        if other.jfif_density_units.is_some() {
            self.jfif_density_units = other.jfif_density_units;
        }
        if other.jfif_x_density.is_some() {
            self.jfif_x_density = other.jfif_x_density;
        }
        if other.jfif_y_density.is_some() {
            self.jfif_y_density = other.jfif_y_density;
        }
        if other.jfif_thumbnail.is_some() {
            self.jfif_thumbnail = other.jfif_thumbnail;
        }
        absorb_other_segments_opt(&mut self.other_segments, other.other_segments);
    }
}

fn validate_jpg_indexed<I, J, K>(base_len: usize, removed: &[usize], modified: I, added: J, path: K) -> MutationApplyResult<()>
where
    I: IntoIterator<Item = usize>,
    J: IntoIterator<Item = usize>,
    K: IntoIterator,
    K::Item: AsRef<str>,
{
    let path: Vec<String> = path.into_iter().map(|part| part.as_ref().to_owned()).collect();
    let mut removed_set = std::collections::HashSet::new();
    for &index in removed {
        if index >= base_len || !removed_set.insert(index) {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "JPEG collection removal is missing or duplicated").at(path.iter().map(String::as_str)));
        }
    }
    let mut modified_set = std::collections::HashSet::new();
    for index in modified {
        if index >= base_len || !modified_set.insert(index) || removed_set.contains(&index) {
            return Err(MutationApplyError::new("mutation.apply.conflicting-target", "JPEG collection modification is missing, duplicated, or removed").at(path.iter().map(String::as_str)));
        }
    }
    let added: Vec<usize> = added.into_iter().collect();
    let final_len = base_len.saturating_sub(removed.len()).saturating_add(added.len());
    let mut added_set = std::collections::HashSet::new();
    for index in added {
        if index > final_len || !added_set.insert(index) {
            return Err(MutationApplyError::new("mutation.apply.invalid-index", "JPEG collection addition index is invalid or duplicated").at(path.iter().map(String::as_str)));
        }
    }
    Ok(())
}

/// 🧮️ The sparse diff that carries `base` to `other`: only the fields that differ, and the segment list as a keyed triple.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn jpg_image_diff(base: &crate::schema::snapshot::JpgImage, other: &crate::schema::snapshot::JpgImage) -> JpgDiff {
    JpgDiff {
        width: (base.width != other.width).then_some(other.width),
        height: (base.height != other.height).then_some(other.height),
        pixels: (base.pixels != other.pixels).then(|| other.pixels.clone()),
        jfif_version: (base.jfif_version != other.jfif_version).then_some(other.jfif_version),
        jfif_density_units: (base.jfif_density_units != other.jfif_density_units).then_some(other.jfif_density_units),
        jfif_x_density: (base.jfif_x_density != other.jfif_x_density).then_some(other.jfif_x_density),
        jfif_y_density: (base.jfif_y_density != other.jfif_y_density).then_some(other.jfif_y_density),
        jfif_thumbnail: (base.jfif_thumbnail != other.jfif_thumbnail).then(|| other.jfif_thumbnail.clone()),
        other_segments: changed_other_segments(&base.other_segments, &other.other_segments),
    }
}

impl DiffAlgebra<JpgSnapshot> for JpgDiff {
    /// 🔁️ Concrete diff-level undo: every replaced field takes the value `base` carries, removed segments return at their base index, added ones go away
    /// again and modified ones restore their base marker and data at the index they have after the diff.
    fn inverse(&self, base: &JpgSnapshot) -> Self {
        let image = &base.image;
        Self {
            width: self.width.filter(|width| *width != image.width).map(|_| image.width),
            height: self.height.filter(|height| *height != image.height).map(|_| image.height),
            pixels: self.pixels.as_ref().filter(|pixels| **pixels != image.pixels).map(|_| image.pixels.clone()),
            jfif_version: self.jfif_version.filter(|version| *version != image.jfif_version).map(|_| image.jfif_version),
            jfif_density_units: self.jfif_density_units.filter(|units| *units != image.jfif_density_units).map(|_| image.jfif_density_units),
            jfif_x_density: self.jfif_x_density.filter(|density| *density != image.jfif_x_density).map(|_| image.jfif_x_density),
            jfif_y_density: self.jfif_y_density.filter(|density| *density != image.jfif_y_density).map(|_| image.jfif_y_density),
            jfif_thumbnail: self.jfif_thumbnail.as_ref().filter(|thumbnail| **thumbnail != image.jfif_thumbnail).map(|_| image.jfif_thumbnail.clone()),
            other_segments: self.other_segments.as_ref().map(|segments| inverse_other_segments(segments, &image.other_segments)).filter(|segments| !(segments.removed.is_empty() && segments.modified.is_empty() && segments.added.is_empty())),
        }
    }

    fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}

#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_diff_cases() -> Vec<JpgDiff> {
    let segments = JpgOtherSegmentsDiff {
        removed: vec![1],
        modified: vec![JpgSegmentModified { index: 0, diff: JpgSegmentDiff { data: Some(vec![4, 5, 6]), ..Default::default() } }],
        added: vec![JpgSegmentAdded { index: 1, item: JpgSegment { marker: 0xE1, data: vec![9, 9] } }],
    };
    vec![
        JpgDiff::default(),
        JpgDiff {
            width: Some(8),
            height: Some(6),
            pixels: Some(vec![9u8; 12]),
            jfif_version: Some((1, 2)),
            jfif_density_units: Some(JfifDensityUnits::Aspect),
            jfif_x_density: Some(1),
            jfif_y_density: Some(1),
            jfif_thumbnail: Some(None),
            other_segments: Some(segments),
        },
        JpgDiff { jfif_thumbnail: Some(Some(JfifThumbnail { width: 2, height: 1, rgb_data: vec![1, 2, 3, 4, 5, 6] })), ..Default::default() },
    ]
}

#[cfg(test)]
#[path = "🧪️tests/🔬️handcrafted-diff-codec/🦀️.rs"]
mod handcrafted_diff_codec_tests;
