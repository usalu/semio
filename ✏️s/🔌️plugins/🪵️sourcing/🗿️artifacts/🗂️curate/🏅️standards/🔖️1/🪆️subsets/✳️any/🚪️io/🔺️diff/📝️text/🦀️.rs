//! 🔺️ Sourcing curate artifact — sparse field-delta diff codec and apply/absorb.

use crate::artifacts::curate::schema::diff::{CurateCuratedDelta, CurateDiff, CurateStockExtraDelta};
use crate::artifacts::curate::schema::CurateArtifact;
use crate::artifacts::curate::{CurateSnapshot, CuratedItem, ObjectKindExtra};
use protocol::MutationDiff;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️Apply
pub fn apply_stock_extra_delta(stock_extra: &[ObjectKindExtra], delta: &CurateStockExtraDelta) -> protocol::MutationApplyResult<Vec<ObjectKindExtra>> {
    let mut removed = std::collections::BTreeSet::new();
    for (index, id) in delta.removed.iter().enumerate() {
        if !removed.insert(id.as_str()) {
            return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "stock entry is removed more than once").at(["removed".to_string(), index.to_string()]));
        }
        if !stock_extra.iter().any(|extra| &extra.id == id) {
            return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "removed stock entry does not exist").at(["removed".to_string(), index.to_string()]));
        }
    }
    let mut identities: std::collections::BTreeSet<_> = stock_extra.iter().map(|extra| extra.id.clone()).collect();
    for id in &delta.removed {
        identities.remove(id);
    }
    for (index, extra) in delta.added.iter().enumerate() {
        if !identities.insert(extra.id.clone()) {
            return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "added stock entry identity already exists").at(["added".to_string(), index.to_string()]));
        }
    }
    let mut patched = std::collections::BTreeSet::new();
    for (index, entry) in delta.patched.iter().enumerate() {
        if !patched.insert(entry.id.as_str()) {
            return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "stock entry is patched more than once").at(["patched".to_string(), index.to_string()]));
        }
        if removed.contains(entry.id.as_str()) || !identities.contains(&entry.id) {
            return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "patched stock entry does not exist").at(["patched".to_string(), index.to_string()]));
        }
        if entry.extra.id != entry.id {
            return Err(protocol::MutationApplyError::new("mutation.apply.invalid-target", "stock entry patch cannot change its identity").at(["patched".to_string(), index.to_string()]));
        }
    }
    let mut next: Vec<_> = stock_extra.iter().filter(|extra| !removed.contains(extra.id.as_str())).cloned().collect();
    next.extend(delta.added.iter().cloned());
    for entry in &delta.patched {
        let target = next
            .iter_mut()
            .find(|extra| extra.id == entry.id)
            .ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.missing-target", "patched stock entry does not exist after structural edits").at(["patched".to_string(), entry.id.clone()]))?;
        *target = entry.extra.clone();
    }
    reorder_named(next, delta.reordered.as_deref(), |extra| extra.id.as_str())
}

pub fn apply_curated_delta(curated: &[CuratedItem], delta: &CurateCuratedDelta) -> protocol::MutationApplyResult<Vec<CuratedItem>> {
    let mut removed = std::collections::BTreeSet::new();
    for (index, id) in delta.removed.iter().enumerate() {
        if !removed.insert(id.as_str()) {
            return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "curated item is removed more than once").at(["removed".to_string(), index.to_string()]));
        }
        if !curated.iter().any(|item| &item.object_id == id) {
            return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "removed curated item does not exist").at(["removed".to_string(), index.to_string()]));
        }
    }
    let mut identities: std::collections::BTreeSet<_> = curated.iter().map(|item| item.object_id.clone()).collect();
    for id in &delta.removed {
        identities.remove(id);
    }
    for (index, item) in delta.added.iter().enumerate() {
        if !identities.insert(item.object_id.clone()) {
            return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "added curated item identity already exists").at(["added".to_string(), index.to_string()]));
        }
    }
    let mut patched = std::collections::BTreeSet::new();
    for (index, entry) in delta.patched.iter().enumerate() {
        if !patched.insert(entry.object_id.as_str()) {
            return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "curated item is patched more than once").at(["patched".to_string(), index.to_string()]));
        }
        if removed.contains(entry.object_id.as_str()) || !identities.contains(&entry.object_id) {
            return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "patched curated item does not exist").at(["patched".to_string(), index.to_string()]));
        }
    }
    let mut next: Vec<_> = curated.iter().filter(|item| !removed.contains(item.object_id.as_str())).cloned().collect();
    next.extend(delta.added.iter().cloned());
    for entry in &delta.patched {
        let target = next
            .iter_mut()
            .find(|item| item.object_id == entry.object_id)
            .ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.missing-target", "patched curated item does not exist after structural edits").at(["patched".to_string(), entry.object_id.clone()]))?;
        if let Some(count) = entry.count {
            target.count = count;
        }
    }
    reorder_named(next, delta.reordered.as_deref(), |item| item.object_id.as_str())
}

fn reorder_named<T>(items: Vec<T>, order: Option<&[String]>, id: impl for<'a> Fn(&'a T) -> &'a str) -> protocol::MutationApplyResult<Vec<T>> {
    let Some(order) = order else {
        return Ok(items);
    };
    if order.len() != items.len() || order.iter().enumerate().any(|(index, target)| order[..index].contains(target) || !items.iter().any(|item| id(item) == target)) {
        return Err(protocol::MutationApplyError::new("mutation.apply.invalid-order", "reorder must be a complete unique permutation").at(["reordered"]));
    }
    let mut remaining = items;
    let mut ordered = Vec::with_capacity(order.len());
    for target in order {
        let index = remaining.iter().position(|item| id(item) == target).ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.missing-target", "reordered item does not exist").at(["reordered".to_string(), target.clone()]))?;
        ordered.push(remaining.remove(index));
    }
    Ok(ordered)
}

impl CurateDiff {
    /// 🧬️ Applies every sparse entry (all state classes) onto a full artifact.
    pub fn apply_to_artifact(&self, artifact: &CurateArtifact) -> protocol::MutationApplyResult<CurateArtifact> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok((**replacement).clone());
            }
            let mut next = artifact.clone();
            if let Some(handle) = &self.catalog {
                next.catalog = handle.clone();
            }
            if let Some(delta) = &self.stock_extra {
                next.stock_extra = apply_stock_extra_delta(&next.stock_extra, delta).map_err(|error| error.under(["stockExtra"]))?;
            }
            if let Some(delta) = &self.curated {
                next.curated = apply_curated_delta(&next.curated, delta).map_err(|error| error.under(["curated"]))?;
            }
            if let Some(filters) = &self.filters {
                next.filters = filters.clone();
            }
            if let Some(value) = &self.locale {
                next.locale = value.clone();
            }
            if let Some(value) = &self.contributions_json {
                next.contributions_json = value.clone();
            }
            next
        })
    }
}

/// 🖼️ Whole-artifact replacement from a snapshot (UI fields defaulted).
pub fn diff_set_snapshot(snapshot: &CurateSnapshot) -> CurateDiff {
    CurateDiff { artifact: Some(Box::new(CurateArtifact::from_snapshot(snapshot.clone()))), ..Default::default() }
}

impl MutationDiff<CurateSnapshot> for CurateDiff {
    fn apply(&self, snapshot: &CurateSnapshot) -> protocol::MutationApplyResult<CurateSnapshot> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok(replacement.to_snapshot());
            }
            let mut next = snapshot.clone();
            if let Some(handle) = &self.catalog {
                next.catalog = handle.clone();
            }
            if let Some(delta) = &self.stock_extra {
                next.stock_extra = apply_stock_extra_delta(&next.stock_extra, delta).map_err(|error| error.under(["stockExtra"]))?;
            }
            if let Some(delta) = &self.curated {
                next.curated = apply_curated_delta(&next.curated, delta).map_err(|error| error.under(["curated"]))?;
            }
            next
        })
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
        take!(catalog);
        take!(filters);
        take!(locale);
        take!(contributions_json);
        match (&mut self.stock_extra, other.stock_extra) {
            (Some(dst), Some(src)) => {
                dst.added.extend(src.added);
                dst.removed.extend(src.removed);
                dst.patched.extend(src.patched);
                // 🐛️ Pre-existing absorb-law bug, fixed in passing (same class as the
                // sequence plugin's `DisconnectSteps`-after-`DeleteStep` fix): a `patched`
                // entry absorbed alongside a LATER `removed` for the same id survived the
                // merge, so `apply_stock_extra_delta`'s own consistency check (which rejects
                // a `patched` id that is also `removed`) then rejected the whole absorbed
                // diff — violating `absorb(d1, d2).apply(base) == d2.apply(d1.apply(base))`
                // whenever d2 deletes something d1 patched. Dropping the stale patch entry
                // once its id lands in the merged `removed` set restores the law.
                let removed_ids: std::collections::BTreeSet<&str> = dst.removed.iter().map(String::as_str).collect();
                dst.patched.retain(|entry| !removed_ids.contains(entry.id.as_str()));
                if src.reordered.is_some() {
                    dst.reordered = src.reordered;
                }
            }
            (None, Some(src)) => self.stock_extra = Some(src),
            _ => {}
        }
        match (&mut self.curated, other.curated) {
            (Some(dst), Some(src)) => {
                dst.added.extend(src.added);
                dst.removed.extend(src.removed);
                dst.patched.extend(src.patched);
                // 🐛️ Same fix as `stock_extra` above — see that arm's doc comment.
                dst.patched.retain(|entry| !dst.removed.iter().any(|id| id == &entry.object_id));
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
    #[semio_framework_async_macros::async_test]
    async fn diff_set_snapshot_carries_whole_replacement() {
        let base = CurateSnapshot::default();
        let next = CurateSnapshot::default();
        let diff = diff_set_snapshot(&next);
        assert_eq!(diff.apply(&base).expect("valid mutation diff"), next);
    }

    #[semio_framework_async_macros::async_test]
    async fn absorb_keeps_later_artifact_replacement() {
        let mut first = CurateDiff { artifact: Some(Box::new(CurateArtifact::default())), ..Default::default() };
        let second = CurateDiff::default();
        first.absorb(second);
        assert!(first.artifact.is_some());
    }
}
//#endregion 🧪️Tests
