//! 🧵️ Ordered Flow structural changes corresponding to the adjacent JSON schema.
use super::{ApplyCapability, DiffAlgebra, FlowCollectionDelta, FlowHostSnapshot, FlowLayoutEntry, FlowOwner, FlowRetirement, Identified, MutationApplyResult, MutationDiff, SynapseSpec, Widget};
use std::collections::BTreeSet;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🧬️Schema
/// 🗂️ Ordered structural fragments; a whole document is never a delta (a load goes through `Effect::LoadDocument`).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(tag = "delta", content = "value", rename_all = "camelCase", deny_unknown_fields)]
pub enum FlowDelta {
    Widgets(FlowCollectionDelta<Widget>),
    Synapses(FlowCollectionDelta<SynapseSpec>),
    Layout(Vec<FlowLayoutEntry>),
}

/// 🧶️ Sequential structural changes compose by concatenation, never by semantic replay.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct FlowDiff {
    pub deltas: Vec<FlowDelta>,
}

impl From<FlowDelta> for FlowDiff {
    fn from(delta: FlowDelta) -> Self { Self { deltas: vec![delta] } }
}

impl FlowDelta {
    /// 🧊️ Hands one structural fragment's owners to `frontier`; both collection deltas own whole `Widget`s whose
    /// fail-closed roots abort the process on a bare drop (`🌱️value/🗂️ordered/🦀️.rs`'s `Drop`).
    fn handoff(self, frontier: &mut FlowRetirement) {
        match self {
            Self::Widgets(delta) => {
                frontier.push_cold(FlowOwner::Strings(delta.removed));
                frontier.push_cold(FlowOwner::Widgets(delta.inserted.into_iter().map(|(_, widget)| widget).collect()));
                let (ids, widgets): (Vec<String>, Vec<Widget>) = delta.replaced.into_iter().unzip();
                frontier.push_cold(FlowOwner::Strings(ids));
                frontier.push_cold(FlowOwner::Widgets(widgets));
            }
            Self::Synapses(delta) => {
                frontier.push_cold(FlowOwner::Strings(delta.removed));
                frontier.push_cold(FlowOwner::Specs(delta.inserted.into_iter().map(|(_, spec)| spec).collect()));
                let (ids, specs): (Vec<String>, Vec<SynapseSpec>) = delta.replaced.into_iter().unzip();
                frontier.push_cold(FlowOwner::Strings(ids));
                frontier.push_cold(FlowOwner::Specs(specs));
            }
            Self::Layout(entries) => frontier.push_cold(FlowOwner::Layout(entries)),
        }
    }
}
//#endregion 🧬️Schema

//#region ▶️Application
#[path = "📑️projection/🦀️.rs"]
mod projection;
use projection::FlowProjection;

impl DiffAlgebra<FlowHostSnapshot> for FlowDiff {
    fn inverse(&self, base: &FlowHostSnapshot) -> Self {
        let mut inverse = Vec::with_capacity(self.deltas.len());
        for (index, delta) in self.deltas.iter().enumerate() {
            let earlier = &self.deltas[..index];
            inverse.push(match delta {
                FlowDelta::Widgets(delta) => FlowDelta::Widgets(inverse_collection_delta(&base.widgets, earlier.iter().filter_map(|delta| match delta { FlowDelta::Widgets(rows) => Some(rows), _ => None }), delta)),
                FlowDelta::Synapses(delta) => FlowDelta::Synapses(inverse_collection_delta(&base.synapses, earlier.iter().filter_map(|delta| match delta { FlowDelta::Synapses(rows) => Some(rows), _ => None }), delta)),
                FlowDelta::Layout(entries) => FlowDelta::Layout(inverse_layout(base, earlier.iter().filter_map(|delta| match delta { FlowDelta::Layout(rows) => Some(rows.as_slice()), _ => None }), entries)),
            });
        }
        inverse.reverse();
        Self { deltas: inverse }
    }

    fn is_empty(&self) -> bool {
        self.deltas.iter().all(|delta| match delta {
            FlowDelta::Widgets(delta) => delta.is_idle(),
            FlowDelta::Synapses(delta) => delta.is_idle(),
            FlowDelta::Layout(entries) => entries.is_empty(),
        })
    }
}

impl MutationDiff<FlowHostSnapshot> for FlowDiff {
    fn apply(&self, snapshot: &FlowHostSnapshot, _capability: ApplyCapability) -> MutationApplyResult<FlowHostSnapshot> {
        let mut projection = FlowProjection::new(snapshot);
        for delta in &self.deltas {
            projection.apply(delta)?;
        }
        Ok(projection.materialize())
    }

    fn absorb(&mut self, other: Self) { self.deltas.extend(other.deltas); }

    fn retire_cold(self) {
        let mut frontier = FlowRetirement::default();
        for delta in self.deltas {
            delta.handoff(&mut frontier);
        }
        frontier.retire_cold();
    }

    fn retire_projection(projection: FlowHostSnapshot) { projection.retire_cold(); }
}
//#endregion ▶️Application

//#region ↩️Rows
impl<T> FlowCollectionDelta<T> {
    fn is_idle(&self) -> bool { self.removed.is_empty() && self.inserted.is_empty() && self.replaced.is_empty() }
}

/// ↩️ The collection delta that undoes `delta`, read row by row: inserted identities leave, replaced items return under
/// their replacement identity with the base item's value, and removed items re-enter at their base position ascending.
/// A row's prior value is looked up in `base`, else in the rows of `earlier` deltas of the same diff; nothing is applied.
fn inverse_collection_delta<'a, T: Identified<String> + Clone + 'a>(base: &'a [T], earlier: impl Iterator<Item = &'a FlowCollectionDelta<T>> + Clone, delta: &'a FlowCollectionDelta<T>) -> FlowCollectionDelta<T> {
    let prior = |id: &String| -> Option<(u32, &'a T)> {
        base.iter().position(|item| item.id() == id).map(|index| (index as u32, &base[index])).or_else(|| {
            earlier.clone().find_map(|rows| {
                rows.inserted.iter().find(|(_, item)| item.id() == id).map(|(index, item)| (*index, item)).or_else(|| rows.replaced.iter().find(|(_, item)| item.id() == id).map(|(_, item)| (0, item)))
            })
        })
    };
    let mut restored: Vec<(u32, T)> = delta.removed.iter().filter_map(|id| prior(id).map(|(index, item)| (index, item.clone()))).collect();
    restored.sort_by_key(|(index, _)| *index);
    FlowCollectionDelta {
        removed: delta.inserted.iter().map(|(_, item)| item.id().clone()).collect(),
        replaced: delta.replaced.iter().filter_map(|(id, replacement)| prior(id).map(|(_, previous)| (replacement.id().clone(), previous.clone()))).collect(),
        inserted: restored,
    }
}

/// ↩️ Layout rows restore each id's previous assignment: the last earlier entry for that id in this delta, else in an earlier layout
/// delta of the same diff, else the base's.
fn inverse_layout<'a>(base: &'a FlowHostSnapshot, earlier: impl Iterator<Item = &'a [FlowLayoutEntry]> + Clone, entries: &'a [FlowLayoutEntry]) -> Vec<FlowLayoutEntry> {
    let mut inverse = Vec::with_capacity(entries.len());
    for (index, entry) in entries.iter().enumerate() {
        let within = entries[..index].iter().rev().find(|row| row.id == entry.id).map(|row| row.layout.as_ref());
        let before = || earlier.clone().flat_map(|rows| rows.iter()).filter(|row| row.id == entry.id).last().map(|row| row.layout.as_ref());
        let prior = within.or_else(before).unwrap_or_else(|| base.layout.get(entry.id.as_str()));
        inverse.push(FlowLayoutEntry { id: entry.id.clone(), layout: prior.cloned() });
    }
    inverse.reverse();
    inverse
}

/// 🔀️ The survivors of `other` whose position differs from `base` after every other row is settled: each is a remove plus an
/// insert (the same pair `MoveWidget` emits), chosen greedily by fixing positions ascending.
fn moved_identities<'a, T: Identified<String>>(base: &'a [T], other: &'a [T]) -> BTreeSet<&'a str> {
    let target_ids: BTreeSet<&str> = other.iter().map(|item| item.id().as_str()).collect();
    let base_ids: BTreeSet<&str> = base.iter().map(|item| item.id().as_str()).collect();
    let mut current: Vec<&str> = base.iter().map(|item| item.id().as_str()).filter(|id| target_ids.contains(id)).collect();
    let target: Vec<&str> = other.iter().map(|item| item.id().as_str()).filter(|id| base_ids.contains(id)).collect();
    let mut moved = BTreeSet::new();
    for (index, id) in target.iter().enumerate() {
        if current.get(index) != Some(id) {
            if let Some(from) = current.iter().position(|candidate| candidate == id) {
                let item = current.remove(from);
                current.insert(index, item);
                moved.insert(*id);
            }
        }
    }
    moved
}

//#endregion ↩️Rows

//#region 🧪️Ownership
#[cfg(test)]
#[path = "🧪️tests/🧾️ownership/🦀️.rs"]
mod ownership_tests;
//#endregion 🧪️Ownership
