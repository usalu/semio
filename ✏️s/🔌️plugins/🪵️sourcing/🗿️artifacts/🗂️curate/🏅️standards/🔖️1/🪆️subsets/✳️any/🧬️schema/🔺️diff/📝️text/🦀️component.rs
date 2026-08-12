//! 🔺️ Sourcing curate artifact — sparse field-delta diff codec and apply/absorb.

use crate::artifacts::curate::schema::diff::{
    CurateCuratedDelta, CurateCuratedPatchEntry, CurateDiff, CurateObjectKindPatchEntry, CurateStockDelta,
};
use crate::artifacts::curate::schema::CurateArtifact;
use crate::artifacts::curate::{CuratedItem, CurateSnapshot, ObjectKind};
use protocol::MutationDiff;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::artifacts::curate::schema::diff::*;


//#region 🔖️Apply
pub fn apply_stock_delta(stock: &[ObjectKind], delta: &CurateStockDelta) -> Vec<ObjectKind> {
    let mut by_id: std::collections::BTreeMap<String, ObjectKind> =
        stock.iter().map(|kind| (kind.id.clone(), kind.clone())).collect();
    for id in &delta.removed {
        by_id.remove(id);
    }
    for kind in &delta.added {
        by_id.insert(kind.id.clone(), kind.clone());
    }
    for entry in &delta.patched {
        if by_id.contains_key(&entry.id) {
            by_id.insert(entry.id.clone(), entry.kind.clone());
        }
    }
    if let Some(order) = &delta.reordered {
        let mut ordered = Vec::with_capacity(order.len());
        for id in order {
            if let Some(kind) = by_id.remove(id) {
                ordered.push(kind);
            }
        }
        ordered.extend(by_id.into_values());
        return ordered;
    }
    by_id.into_values().collect()
}

pub fn apply_curated_delta(curated: &[CuratedItem], delta: &CurateCuratedDelta) -> Vec<CuratedItem> {
    let mut by_id: std::collections::BTreeMap<String, CuratedItem> =
        curated.iter().map(|item| (item.object_id.clone(), item.clone())).collect();
    for id in &delta.removed {
        by_id.remove(id);
    }
    for item in &delta.added {
        by_id.insert(item.object_id.clone(), item.clone());
    }
    for patch in &delta.patched {
        if let Some(entry) = by_id.get_mut(&patch.object_id) {
            if let Some(count) = patch.count {
                entry.count = count;
            }
        }
    }
    if let Some(order) = &delta.reordered {
        let mut ordered = Vec::with_capacity(order.len());
        for id in order {
            if let Some(item) = by_id.remove(id) {
                ordered.push(item);
            }
        }
        ordered.extend(by_id.into_values());
        return ordered;
    }
    by_id.into_values().collect()
}

impl CurateDiff {
    /// 🧬️ Applies every sparse entry (all state classes) onto a full artifact.
    pub fn apply_to_artifact(&self, artifact: &CurateArtifact) -> CurateArtifact {
        if let Some(replacement) = &self.artifact {
            return (**replacement).clone();
        }
        let mut next = artifact.clone();
        if let Some(delta) = &self.stock {
            next.stock = apply_stock_delta(&next.stock, delta);
        }
        if let Some(delta) = &self.curated {
            next.curated = apply_curated_delta(&next.curated, delta);
        }
        if let Some(filters) = &self.filters {
            next.filters = filters.clone();
        }
        if let Some(value) = &self.selected_object_id {
            next.selected_object_id = value.clone();
        }
        if let Some(value) = &self.locale {
            next.locale = value.clone();
        }
        if let Some(value) = &self.contributions_json {
            next.contributions_json = value.clone();
        }
        next
    }
}

/// 🖼️ Whole-artifact replacement from a snapshot (UI fields defaulted).
pub fn diff_set_snapshot(snapshot: &CurateSnapshot) -> CurateDiff {
    CurateDiff {
        artifact: Some(Box::new(CurateArtifact::from_snapshot(snapshot.clone()))),
        ..Default::default()
    }
}

impl MutationDiff<CurateSnapshot> for CurateDiff {
    fn apply(&self, snapshot: &CurateSnapshot) -> CurateSnapshot {
        if let Some(replacement) = &self.artifact {
            return replacement.to_snapshot();
        }
        let mut next = snapshot.clone();
        if let Some(delta) = &self.stock {
            next.stock = apply_stock_delta(&next.stock, delta);
        }
        if let Some(delta) = &self.curated {
            next.curated = apply_curated_delta(&next.curated, delta);
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        if other.artifact.is_some() {
            *self = other;
            return;
        }
        macro_rules! take {
            ($field:ident) => {
                if other.$field.is_some() {
                    self.$field = other.$field;
                }
            };
        }
        take!(filters);
        take!(selected_object_id);
        take!(locale);
        take!(contributions_json);
        match (&mut self.stock, other.stock) {
            (Some(dst), Some(src)) => {
                dst.added.extend(src.added);
                dst.removed.extend(src.removed);
                dst.patched.extend(src.patched);
                if src.reordered.is_some() {
                    dst.reordered = src.reordered;
                }
            }
            (None, Some(src)) => self.stock = Some(src),
            _ => {}
        }
        match (&mut self.curated, other.curated) {
            (Some(dst), Some(src)) => {
                dst.added.extend(src.added);
                dst.removed.extend(src.removed);
                dst.patched.extend(src.patched);
                if src.reordered.is_some() {
                    dst.reordered = src.reordered;
                }
            }
            (None, Some(src)) => self.curated = Some(src),
            _ => {}
        }
    }
}
//#endregion 🔖️Apply

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    /// 🧬️ `diff_set_snapshot`/`CurateDiff.artifact` are a generic whole-artifact-replacement escape
    /// hatch retained for `apply_to_artifact`'s own callers — no `SourcingMutation` variant reaches
    /// it any more (the former whole-snapshot-replace variant is banned outright, see `📓️taxonomy.md`), so this exercises the
    /// function directly rather than through a mutation's `diff()`.
    #[test]
    fn diff_set_snapshot_carries_whole_replacement() {
        let base = CurateSnapshot::default();
        let next = CurateSnapshot { stock: vec![], curated: vec![] };
        let diff = diff_set_snapshot(&next);
        assert_eq!(diff.apply(&base), next);
    }

    #[test]
    fn absorb_keeps_later_artifact_replacement() {
        let mut first = CurateDiff {
            artifact: Some(Box::new(CurateArtifact {
                stock: vec![],
                curated: vec![],
                ..Default::default()
            })),
            ..Default::default()
        };
        let second = CurateDiff::default();
        first.absorb(second);
        assert!(first.artifact.is_some());
    }
}
//#endregion 🧪️Tests
