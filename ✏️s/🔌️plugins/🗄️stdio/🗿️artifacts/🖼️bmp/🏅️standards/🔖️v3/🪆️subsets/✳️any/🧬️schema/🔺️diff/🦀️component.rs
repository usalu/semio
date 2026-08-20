//! 🔺️ BmpDiff — handcrafted sparse structural diff. Header (BITMAPINFOHEADER) fields are
//! top-level `Option<T>` scalars (none are spec-nullable, so no tri-state is needed); `palette`
//! is an index-keyed removed/modified/added triple (BMP palette entries have no identity beyond
//! position); `pixels` is a whole-buffer replace (the format's payload literally IS decoded
//! canonical-RGBA bytes — the recipe's documented `Vec<u8>` exception).

use crate::artifacts::bmp::schema::snapshot::{BmpPaletteEntry, BmpRowOrder};
use crate::artifacts::bmp::BmpSnapshot;
use std::collections::HashSet;
// 🔗 `DiffAlgebra` (spine S-1) isn't in the `protocol` facade's curated re-export list yet —
// reach it via the same crate's directly-mounted `command` module (F1 precedent, see
// `f1-csv-report.md` `## Deviations`).
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

//#region 🔖️PaletteDiff
/// 🧩 One palette entry patched-in-place at a BASE index (whole-value replace — palette
/// entries are a weak/value entity, never sub-diffed).
/// 🧪️ F6: `dsl::DslRecord` — nested value type needs `DslField` for `BmpPaletteDiff`'s own
/// `#[derive(dsl::DslDiff)]`/`#[derive(dsl::DslRecord)]` field codegen (`f6-recon-report.md` §9
/// STEP-2a's cascading-derive requirement).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct BmpPaletteModified {
    pub index: usize,
    #[dsl(block)]
    pub entry: BmpPaletteEntry,
}

/// 🧩 One palette entry inserted at a FINAL index.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct BmpPaletteAdded {
    pub index: usize,
    #[dsl(block)]
    pub entry: BmpPaletteEntry,
}

/// 🔺️ Index-keyed removed/modified/added triple over `BmpSnapshot::palette`
/// (`~/.claude/plans/the-current-schemas-are-scalable-journal.md` `## Diff`).
/// 🧪️ F6: `dsl::DslRecord` — the collection-triple shape's own container binds directly (bare
/// `Vec<T>` fields have a blanket `DslField` impl in the `dsl` crate, `f6-recon-report.md` §3b).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct BmpPaletteDiff {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<BmpPaletteModified>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<BmpPaletteAdded>,
}

impl BmpPaletteDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }
}
//#endregion 🔖️PaletteDiff

//#region 🔖️IndexTransport
// 🧮 Base-free index transport for absorb — simulates the SAME removed(descending)/added
// (ascending, clamped) sequence `BmpPaletteDiff` apply performs, over a virtual index universe
// bounded tightly by what a diff's own removed/modified keys actually reference (matches the
// recipe's "structural, total, base-free" absorb contract; identical shape to csv's proven
// `absorb_records` index transport).

/// 🎰 One slot of a simulated post-removal/insertion array.
#[derive(Clone, Copy, Debug)]
enum Slot {
    /// A surviving item that was at this BASE index.
    Base(usize),
    /// An item inserted by this diff, identified by its position in the diff's own `added` vec.
    Added(usize),
}

/// 🧪 Simulates `removed`(descending)/`added`(ascending, clamped) against a virtual array of
/// `[0, len)` `Slot::Base(i)` markers, mirroring `BmpPaletteDiff` apply's own ordering exactly.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn simulate_slots(len: usize, removed: &[usize], added_indices: &[usize]) -> Vec<Slot> {
    let mut slots: Vec<Slot> = (0..len).map(Slot::Base).collect();
    let mut removed_desc = removed.to_vec();
    removed_desc.sort_unstable_by(|a, b| b.cmp(a));
    removed_desc.dedup();
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

/// 📏 Tight virtual-array bound: one past the highest base index this diff's own
/// removed/modified/added keys reference (0 if it references none).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn base_len_hint(removed: &[usize], modified_indices: impl Iterator<Item = usize>, added_indices: impl Iterator<Item = usize>) -> usize {
    removed.iter().copied().chain(modified_indices).chain(added_indices).max().map(|m| m + 1).unwrap_or(0)
}

/// ➕️ Structural, total, base-free absorb of two `palette` triples
/// (`~/.claude/plans/the-current-schemas-are-scalable-journal.md` `## Absorb`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_palette(d1: BmpPaletteDiff, d2: BmpPaletteDiff) -> BmpPaletteDiff {
    //#region 🔖️PhiBaseToMid
    let d1_added_indices: Vec<usize> = d1.added.iter().map(|a| a.index).collect();
    let removed_count = {
        let mut r = d1.removed.clone();
        r.sort_unstable();
        r.dedup();
        r.len()
    };
    let needed_mid_len = d2.removed.iter().copied().chain(d2.modified.iter().map(|m| m.index)).max().map(|m| m + 1).unwrap_or(0);
    let base_len = base_len_hint(&d1.removed, d1.modified.iter().map(|m| m.index), d1_added_indices.iter().copied()).max((needed_mid_len + removed_count).saturating_sub(d1.added.len()));
    let mid_slots = simulate_slots(base_len, &d1.removed, &d1_added_indices);
    //#endregion 🔖️PhiBaseToMid

    //#region 🔖️Seed
    let mut final_removed: Vec<usize> = d1.removed.clone();
    let mut modified_map: BTreeMap<usize, BmpPaletteEntry> = d1.modified.into_iter().map(|m| (m.index, m.entry)).collect();
    let mut added_alive: Vec<Option<BmpPaletteAdded>> = d1.added.into_iter().map(Some).collect();
    //#endregion 🔖️Seed

    //#region 🔖️ApplyD2
    for mid_idx in &d2.removed {
        match mid_slots.get(*mid_idx) {
            Some(Slot::Base(b)) => {
                final_removed.push(*b);
                modified_map.remove(b);
            }
            Some(Slot::Added(ai)) => {
                added_alive[*ai] = None;
            }
            None => {} // 🕳️ out-of-range: graceful no-op
        }
    }
    for m2 in &d2.modified {
        match mid_slots.get(m2.index) {
            Some(Slot::Base(b)) => {
                // ➕️ Weak entity: whole-value replace, no recursive absorb — `other`'s
                // populated entry wins (LWW at the entry level).
                modified_map.insert(*b, m2.entry.clone());
            }
            Some(Slot::Added(ai)) => {
                if let Some(added) = added_alive[*ai].as_mut() {
                    added.entry = m2.entry.clone();
                }
            }
            None => {} // 🕳️ out-of-range: graceful no-op
        }
    }
    //#endregion 🔖️ApplyD2

    //#region 🔖️FinalizeRemovedModified
    final_removed.sort_unstable();
    final_removed.dedup();
    for r in &final_removed {
        modified_map.remove(r);
    }
    let mut final_modified: Vec<BmpPaletteModified> = modified_map.into_iter().map(|(index, entry)| BmpPaletteModified { index, entry }).collect();
    final_modified.sort_by_key(|m| m.index);
    //#endregion 🔖️FinalizeRemovedModified

    //#region 🔖️PsiMidToAfter
    let alive_mid_positions: Vec<usize> = mid_slots
        .iter()
        .enumerate()
        .filter_map(|(pos, slot)| match slot {
            Slot::Added(ai) if added_alive[*ai].is_some() => Some(pos),
            _ => None,
        })
        .collect();
    let d2_added_indices: Vec<usize> = d2.added.iter().map(|a| a.index).collect();
    let mid_len = d2.removed.iter().copied().chain(d2.modified.iter().map(|m| m.index)).chain(alive_mid_positions.iter().copied()).chain(d2_added_indices.iter().copied()).max().map(|m| m + 1).unwrap_or(0);
    let after_slots = simulate_slots(mid_len, &d2.removed, &d2_added_indices);
    let mut mid_to_after: HashMap<usize, usize> = HashMap::new();
    for (pos, slot) in after_slots.iter().enumerate() {
        if let Slot::Base(m) = slot {
            mid_to_after.insert(*m, pos);
        }
    }
    //#endregion 🔖️PsiMidToAfter

    //#region 🔖️FinalizeAdded
    let mut final_added: Vec<BmpPaletteAdded> = Vec::new();
    for (ai, alive) in added_alive.into_iter().enumerate() {
        if let Some(added) = alive {
            let mid_pos = mid_slots.iter().position(|s| matches!(s, Slot::Added(idx) if *idx == ai)).expect("added_alive index always has a corresponding mid slot");
            if let Some(after_pos) = mid_to_after.get(&mid_pos) {
                final_added.push(BmpPaletteAdded { index: *after_pos, entry: added.entry });
            }
            // 🕳️ else: this mid slot was itself removed by d2 — always routes through
            // `added_alive[ai] = None` above, so this branch is unreachable.
        }
    }
    for a2 in d2.added {
        final_added.push(a2);
    }
    final_added.sort_by_key(|a| a.index);
    //#endregion 🔖️FinalizeAdded

    BmpPaletteDiff { removed: final_removed, modified: final_modified, added: final_added }
}
//#endregion 🔖️IndexTransport

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.bmp`. No `snapshot: Option<BmpSnapshot>` full-replace slot — even
/// `SetSnapshot`'s diff is `BmpDiff::between(base, next)`.
/// 🧪️ F6: `dsl::DslDiff` derive — every field here is a single-layer `Option<T>` (never
/// `Option<Option<T>>`, no tri-state anywhere in this struct or its nested types) and no
/// data-carrying enum is reachable (`BmpRowOrder` is unit-variant-only, `dsl::DslScalar`-bound),
/// so the derive compiles clean and emits `protocol::DiffCodec` in full — no hand-written impl
/// needed (`f6-recon-report.md` §4/§8 row 14 confirmed for real via `cargo check`). `pixels:
/// Option<Vec<u8>>` does NOT get `#[dsl(base64)]` — the derive's `classify_field` peels the
/// `Option` layer before checking the attribute, so it silently falls back to a verbose bracketed
/// byte list regardless (documented quirk, §3's "Known derive quirk found in passing" — not a
/// compile error, just a token-inefficiency accepted here rather than hand-rolling for it alone).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema, dsl::DslDiff)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.bmp.diff")]
pub struct BmpDiff {
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub header_size: Option<u32>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub row_order: Option<BmpRowOrder>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub planes: Option<u16>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bits_per_pixel: Option<u16>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compression: Option<u32>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_size: Option<u32>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x_pixels_per_meter: Option<i32>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y_pixels_per_meter: Option<i32>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub colors_used: Option<u32>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub colors_important: Option<u32>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[dsl(block)]
    pub palette: Option<BmpPaletteDiff>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pixels: Option<Vec<u8>>,
}

impl MutationDiff<BmpSnapshot> for BmpDiff {
    async fn apply(&self, base: &BmpSnapshot) -> MutationApplyResult<BmpSnapshot> {
        if let Some(palette) = &self.palette {
            validate_bmp_palette(base.palette.len(), palette)?;
        }
        let mut next = base.clone();
        if let Some(v) = self.header_size {
            next.header_size = v;
        }
        if let Some(v) = self.width {
            next.width = v;
        }
        if let Some(v) = self.height {
            next.height = v;
        }
        if let Some(v) = self.row_order {
            next.row_order = v;
        }
        if let Some(v) = self.planes {
            next.planes = v;
        }
        if let Some(v) = self.bits_per_pixel {
            next.bits_per_pixel = v;
        }
        if let Some(v) = self.compression {
            next.compression = v;
        }
        if let Some(v) = self.image_size {
            next.image_size = v;
        }
        if let Some(v) = self.x_pixels_per_meter {
            next.x_pixels_per_meter = v;
        }
        if let Some(v) = self.y_pixels_per_meter {
            next.y_pixels_per_meter = v;
        }
        if let Some(v) = self.colors_used {
            next.colors_used = v;
        }
        if let Some(v) = self.colors_important {
            next.colors_important = v;
        }
        if let Some(pdiff) = &self.palette {
            // 🥇 modified refers to BASE indices — apply before any removal shifts them.
            for m in &pdiff.modified {
                if let Some(entry) = next.palette.get_mut(m.index) {
                    *entry = m.entry.clone();
                }
            }
            // 🥈 removed refers to BASE indices — process descending.
            let mut removed_desc = pdiff.removed.clone();
            removed_desc.sort_unstable_by(|a, b| b.cmp(a));
            removed_desc.dedup();
            for idx in removed_desc {
                if idx < next.palette.len() {
                    next.palette.remove(idx);
                }
            }
            // 🥉 added refers to FINAL indices — process ascending, clamped.
            let mut added_asc = pdiff.added.clone();
            added_asc.sort_by_key(|a| a.index);
            for a in added_asc {
                let at = a.index.min(next.palette.len());
                next.palette.insert(at, a.entry);
            }
        }
        if let Some(v) = &self.pixels {
            next.pixels = v.clone();
        }
        Ok(next)
    }

    async fn absorb(&mut self, other: Self) {
        if other.header_size.is_some() {
            self.header_size = other.header_size;
        }
        if other.width.is_some() {
            self.width = other.width;
        }
        if other.height.is_some() {
            self.height = other.height;
        }
        if other.row_order.is_some() {
            self.row_order = other.row_order;
        }
        if other.planes.is_some() {
            self.planes = other.planes;
        }
        if other.bits_per_pixel.is_some() {
            self.bits_per_pixel = other.bits_per_pixel;
        }
        if other.compression.is_some() {
            self.compression = other.compression;
        }
        if other.image_size.is_some() {
            self.image_size = other.image_size;
        }
        if other.x_pixels_per_meter.is_some() {
            self.x_pixels_per_meter = other.x_pixels_per_meter;
        }
        if other.y_pixels_per_meter.is_some() {
            self.y_pixels_per_meter = other.y_pixels_per_meter;
        }
        if other.colors_used.is_some() {
            self.colors_used = other.colors_used;
        }
        if other.colors_important.is_some() {
            self.colors_important = other.colors_important;
        }
        if let Some(pd2) = other.palette {
            match self.palette.take() {
                None => self.palette = Some(pd2),
                Some(pd1) => self.palette = Some(absorb_palette(pd1, pd2)),
            }
        }
        if other.pixels.is_some() {
            self.pixels = other.pixels;
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_bmp_palette(base_len: usize, diff: &BmpPaletteDiff) -> MutationApplyResult<()> {
    let mut removed = HashSet::new();
    for &index in &diff.removed {
        if index >= base_len || !removed.insert(index) {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "palette removal is missing or duplicated").await.at(["palette", "removed"]));
        }
    }
    let mut modified = HashSet::new();
    for entry in &diff.modified {
        if entry.index >= base_len || !modified.insert(entry.index) || removed.contains(&entry.index) {
            return Err(MutationApplyError::new("mutation.apply.conflicting-target", "palette modification is missing, duplicated, or removed").await.at(["palette", "modified"]));
        }
    }
    let final_len = base_len.saturating_sub(diff.removed.len()).saturating_add(diff.added.len());
    let mut added = HashSet::new();
    for entry in &diff.added {
        if entry.index > final_len || !added.insert(entry.index) {
            return Err(MutationApplyError::new("mutation.apply.invalid-index", "palette addition index is invalid or duplicated").await.at(["palette", "added"]));
        }
    }
    Ok(())
}

impl DiffAlgebra<BmpSnapshot> for BmpDiff {
    async fn inverse(&self, base: &BmpSnapshot) -> Self {
        let applied = self.apply(base).await.unwrap();
        Self::between(&applied, base).await
    }

    async fn between(base: &BmpSnapshot, other: &BmpSnapshot) -> Self {
        let mut d = BmpDiff {
            header_size: (base.header_size != other.header_size).then_some(other.header_size),
            width: (base.width != other.width).then_some(other.width),
            height: (base.height != other.height).then_some(other.height),
            row_order: (base.row_order != other.row_order).then_some(other.row_order),
            planes: (base.planes != other.planes).then_some(other.planes),
            bits_per_pixel: (base.bits_per_pixel != other.bits_per_pixel).then_some(other.bits_per_pixel),
            compression: (base.compression != other.compression).then_some(other.compression),
            image_size: (base.image_size != other.image_size).then_some(other.image_size),
            x_pixels_per_meter: (base.x_pixels_per_meter != other.x_pixels_per_meter).then_some(other.x_pixels_per_meter),
            y_pixels_per_meter: (base.y_pixels_per_meter != other.y_pixels_per_meter).then_some(other.y_pixels_per_meter),
            colors_used: (base.colors_used != other.colors_used).then_some(other.colors_used),
            colors_important: (base.colors_important != other.colors_important).then_some(other.colors_important),
            palette: None,
            pixels: (base.pixels != other.pixels).then(|| other.pixels.clone()),
        };

        let mut removed = Vec::new();
        let mut modified = Vec::new();
        let mut added = Vec::new();
        let min_len = base.palette.len().min(other.palette.len());
        for i in 0..min_len {
            if base.palette[i] != other.palette[i] {
                modified.push(BmpPaletteModified { index: i, entry: other.palette[i].clone() });
            }
        }
        for i in min_len..base.palette.len() {
            removed.push(i);
        }
        for i in min_len..other.palette.len() {
            added.push(BmpPaletteAdded { index: i, entry: other.palette[i].clone() });
        }
        d.palette = if removed.is_empty() && modified.is_empty() && added.is_empty() { None } else { Some(BmpPaletteDiff { removed, modified, added }) };

        d
    }

    async fn is_empty(&self) -> bool {
        self.header_size.is_none()
            && self.width.is_none()
            && self.height.is_none()
            && self.row_order.is_none()
            && self.planes.is_none()
            && self.bits_per_pixel.is_none()
            && self.compression.is_none()
            && self.image_size.is_none()
            && self.x_pixels_per_meter.is_none()
            && self.y_pixels_per_meter.is_none()
            && self.colors_used.is_none()
            && self.colors_important.is_none()
            && self.palette.as_ref().map_or(true, BmpPaletteDiff::is_empty)
            && self.pixels.is_none()
    }
}

/// 🧩 Builds a set-snapshot diff (sparse field-by-field delta, never a full-replace slot).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_snapshot(base: &BmpSnapshot, next: &BmpSnapshot) -> BmpDiff {
    BmpDiff::between(base, next)
}
//#endregion 🔖️Diff

//#region 🔖️DemoDiffCases
/// 🧬️ Module-level (not nested in `mod tests`, mirroring `stdio.png`'s own
/// `demo_snap_a`/`demo_diff_cases` placement) so `⚙️engine/🦀️component.rs`'s
/// `conformance_laws` module can reach these too.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn entry(b: u8, g: u8, r: u8, reserved: u8) -> BmpPaletteEntry {
    BmpPaletteEntry { b, g, r, reserved }
}

#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_snap_a() -> BmpSnapshot {
    BmpSnapshot {
        schema: "stdio.bmp".into(),
        header_size: 40,
        width: 4,
        height: 3,
        row_order: BmpRowOrder::BottomUp,
        planes: 1,
        bits_per_pixel: 8,
        compression: 0,
        image_size: 48,
        x_pixels_per_meter: 2835,
        y_pixels_per_meter: 2835,
        colors_used: 3,
        colors_important: 0,
        palette: vec![entry(0, 0, 255, 0), entry(0, 255, 0, 0)],
        pixels: vec![0u8; 4 * 3 * 4],
    }
}

#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_snap_b() -> BmpSnapshot {
    BmpSnapshot {
        schema: "stdio.bmp".into(),
        header_size: 56,
        width: 8,
        height: 6,
        row_order: BmpRowOrder::TopDown,
        planes: 2,
        bits_per_pixel: 24,
        compression: 3,
        image_size: 200,
        x_pixels_per_meter: 3000,
        y_pixels_per_meter: 4000,
        colors_used: 5,
        colors_important: 2,
        // 🔺 index 0 stable, index 1 modified vs. `demo_snap_a`, index 2 brand-new (asymmetric
        // length on purpose, mirrors gif89a/csv's own "one direction can't show both removed AND
        // added" fixture design) so `a->b` exercises `modified`+`added`.
        palette: vec![entry(0, 0, 255, 0), entry(99, 88, 77, 1), entry(200, 201, 202, 0)],
        pixels: (0..(8 * 6 * 4)).map(|i| ((i * 7 + 3) % 256) as u8).collect(),
    }
}

/// ✅️ P2-FG2: representative `BmpDiff` cases (incl. the empty/default diff) — exercises every
/// scalar field plus all three sections (`removed`/`modified`/`added`) of the `palette`
/// collection triple, via a real `between()` result. Single case list
/// `diff_codec_text_binary_roundtrip_law` (`mod tests` below) AND
/// `diff_grammar_conformance_law`/`protocol_walk_law` (`⚙️engine/🦀️component.rs`'s
/// `conformance_laws` module) all exercise — same consolidation `stdio.png`'s own
/// `demo_diff_cases()` already made (single source of truth, per this repo's own CLAUDE.md).
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_diff_cases() -> Vec<BmpDiff> {
    let a = demo_snap_a();
    let b = demo_snap_b();
    vec![BmpDiff::default(), BmpDiff::between(&a, &b), BmpDiff::between(&b, &a)]
}
//#endregion 🔖️DemoDiffCases

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    /// 🧪️ F6: `diff_codec_text_binary_roundtrip_law` — exercises every scalar field plus all
    /// three sections (`removed`/`modified`/`added`) of the `palette` collection triple, via a
    /// real `between()` result (`f6-recon-report.md` §9 STEP-3's mandated shape).
    #[semio_framework_async_macros::async_test]
    async fn diff_codec_text_binary_roundtrip_law() {
        use protocol::DiffCodec;

        for d in demo_diff_cases() {
            let printed = d.print_diff();
            assert!(!printed.await.contains('\n'), "print_diff must be one line, got {printed:?}");
            let parsed = BmpDiff::parse_diff(&printed).await.unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
            assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch for {d:?} (printed {printed:?})");

            let encoded = d.encode_diff().await.unwrap_or_else(|e| panic!("encode_diff({d:?}) failed: {e}"));
            let decoded = BmpDiff::decode_diff(&encoded).await.unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
            assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch for {d:?}");
        }

        // 🔍 Sanity: `a->b` must actually populate both `modified` and `added` (the collection
        // triple's own coverage, not just the codec round-trip).
        let a = demo_snap_a();
        let b = demo_snap_b();
        let ab = BmpDiff::between(&a, &b);
        let pd = ab.await.palette.as_ref().expect("palette diff must be populated a->b");
        assert!(pd.removed.is_empty(), "a->b must not need a removal (palette grows)");
        assert!(!pd.modified.is_empty(), "a->b must show the modified entry");
        assert!(!pd.added.is_empty(), "a->b must show the added entry");

        let ba = BmpDiff::between(&b, &a);
        let pd_ba = ba.await.palette.as_ref().expect("palette diff must be populated b->a");
        assert!(!pd_ba.removed.is_empty(), "b->a must show the removed entry");
    }
}
//#endregion 🧪️Tests
