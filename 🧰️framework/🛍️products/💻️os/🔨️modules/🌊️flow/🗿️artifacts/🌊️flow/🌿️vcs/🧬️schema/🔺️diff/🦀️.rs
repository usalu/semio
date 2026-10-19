//! 🧵️ Ordered Flow structural changes corresponding to the adjacent JSON schema.
use super::{ApplyCapability, DiffAlgebra, ColdDiffAlgebra, FlowCollectionDelta, FlowHostSnapshot, FlowMove, FlowLayoutEntry, FlowOwner, FlowRetirement, Identified, MutationApplyResult, MutationDiff, ColdMutationDiff, SynapseSpec, Widget};
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

mod projection {
//! 📑️ Borrowed ordered validation precedes owned Flow snapshot materialization.
use super::FlowDelta;
use super::super::{apply_flow_collection_delta, FlowHostSnapshot, FlowLayoutEntry, Identified, MutationApplyError, MutationApplyResult, SynapseSpec, Widget, WidgetLayout};
use std::collections::BTreeMap;

//#region 📑️Projection
pub(super) struct FlowProjection<'a> {
    host_snapshot: &'a FlowHostSnapshot,
    widgets: Vec<&'a Widget>,
    synapses: Vec<&'a SynapseSpec>,
    layout: BTreeMap<&'a str, &'a WidgetLayout>,
}

impl<'a> FlowProjection<'a> {
    pub(super) fn new(host_snapshot: &'a FlowHostSnapshot) -> Self {
        Self {
            host_snapshot,
            widgets: host_snapshot.widgets.iter().collect(),
            synapses: host_snapshot.synapses.iter().collect(),
            layout: host_snapshot.layout.iter().map(|(id, layout)| (id.as_str(), layout)).collect(),
        }
    }

    pub(super) fn apply(&mut self, delta: &'a FlowDelta) -> MutationApplyResult<()> {
        match delta {
            FlowDelta::Widgets(delta) => apply_flow_collection_delta(&mut self.widgets, delta).map_err(|error| error.under(["widgets"])),
            FlowDelta::Synapses(delta) => apply_flow_collection_delta(&mut self.synapses, delta).map_err(|error| error.under(["synapses"])),
            FlowDelta::Layout(entries) => self.apply_layout(entries),
        }
    }

    fn apply_layout(&mut self, entries: &'a [FlowLayoutEntry]) -> MutationApplyResult<()> {
        for entry in entries {
            if !self.widgets.iter().any(|widget| widget.id() == &entry.id) {
                return Err(MutationApplyError::new("mutation.apply.missing-target", format!("layout widget {} does not exist", entry.id)).at(["layout", entry.id.as_str()]));
            }
            match &entry.layout {
                Some(layout) => { self.layout.insert(entry.id.as_str(), layout); }
                None => {
                    if self.layout.remove(entry.id.as_str()).is_none() {
                        return Err(MutationApplyError::new("mutation.apply.missing-target", format!("layout entry {} does not exist", entry.id)).at(["layout", entry.id.as_str()]));
                    }
                }
            }
        }
        Ok(())
    }

    pub(super) fn materialize(self) -> FlowHostSnapshot {
        FlowHostSnapshot {
            schema: self.host_snapshot.schema.clone(),
            camera: self.host_snapshot.camera.clone(),
            widgets: self.widgets.into_iter().cloned().collect(),
            synapses: self.synapses.into_iter().cloned().collect(),
            layout: self.layout.into_iter().map(|(id, layout)| (id.to_owned(), layout.clone())).collect(),
        }
    }
}

//#endregion 📑️Projection
}

//#region ▶️Application
use projection::FlowProjection;

impl DiffAlgebra<FlowHostSnapshot> for FlowDiff {


    fn is_empty(&self) -> bool {
        self.deltas.iter().all(|delta| match delta {
            FlowDelta::Widgets(delta) => delta.is_idle(),
            FlowDelta::Synapses(delta) => delta.is_idle(),
            FlowDelta::Layout(entries) => entries.is_empty(),
        })
    }
}

impl ColdDiffAlgebra<FlowHostSnapshot> for FlowDiff {
    fn inverse(&self, base: &FlowHostSnapshot) -> Self {
        let mut groups: Vec<Vec<FlowDelta>> = Vec::with_capacity(self.deltas.len());
        for (index, delta) in self.deltas.iter().enumerate() {
            let earlier = &self.deltas[..index];
            groups.push(match delta {
                FlowDelta::Widgets(delta) => inverse_collection_deltas(&base.widgets, earlier.iter().filter_map(|delta| match delta { FlowDelta::Widgets(rows) => Some(rows), _ => None }), delta).into_iter().map(FlowDelta::Widgets).collect(),
                FlowDelta::Synapses(delta) => inverse_collection_deltas(&base.synapses, earlier.iter().filter_map(|delta| match delta { FlowDelta::Synapses(rows) => Some(rows), _ => None }), delta).into_iter().map(FlowDelta::Synapses).collect(),
                FlowDelta::Layout(entries) => vec![FlowDelta::Layout(inverse_layout(base, earlier.iter().filter_map(|delta| match delta { FlowDelta::Layout(rows) => Some(rows.as_slice()), _ => None }), entries))],
            });
        }
        Self { deltas: groups.into_iter().rev().flatten().collect() }
    }
}

impl MutationDiff<FlowHostSnapshot> for FlowDiff {




    fn retire_cold(self) {
        let mut frontier = FlowRetirement::default();
        for delta in self.deltas {
            delta.handoff(&mut frontier);
        }
        frontier.retire_cold();
    }

    fn retire_projection(projection: FlowHostSnapshot) { projection.retire_cold(); }
}

impl ColdMutationDiff<FlowHostSnapshot> for FlowDiff {
    fn absorb(&mut self, other: Self) {
        for delta in other.deltas {
            match (self.deltas.last_mut(), delta) {
                (Some(FlowDelta::Layout(rows)), FlowDelta::Layout(more)) => rows.extend(more),
                (_, delta) => self.deltas.push(delta),
            }
        }
    }

    fn apply(&self, snapshot: &mut FlowHostSnapshot, _capability: ApplyCapability) -> MutationApplyResult<()> {
        let mut projection = FlowProjection::new(snapshot);
        for delta in &self.deltas {
            projection.apply(delta)?;
        }
        protocol::PreparedField::new(projection.materialize(), <Self as protocol::MutationDiff<FlowHostSnapshot>>::retire_projection, _capability).install(snapshot);
        Ok(())
    }
}
//#endregion ▶️Application

//#region ↩️Rows
impl<T> FlowCollectionDelta<T> {
    fn is_idle(&self) -> bool { self.removed.is_empty() && self.inserted.is_empty() && self.replaced.is_empty() && self.moved.is_empty() }
}

/// ↩️ The collection deltas that undo `delta`, in application order, read row by row: the move rows come back first, reversed with
/// `from` and `to` swapped (positions the rows themselves carry); then inserted identities leave, replaced items return under
/// their replacement identity with the base item's value, and removed items re-enter at their base position ascending.
/// A row's prior value is looked up in `base`, else in the rows of `earlier` deltas of the same diff; nothing is applied.
fn inverse_collection_deltas<'a, T: Identified<String> + Clone + 'a>(base: &'a [T], earlier: impl Iterator<Item = &'a FlowCollectionDelta<T>> + Clone, delta: &'a FlowCollectionDelta<T>) -> Vec<FlowCollectionDelta<T>> {
    let unmoved = FlowCollectionDelta { moved: delta.moved.iter().rev().map(|row| FlowMove { id: row.id.clone(), from: row.to, to: row.from }).collect(), ..FlowCollectionDelta::default() };
    let rest = inverse_structural_rows(base, earlier, delta);
    [unmoved, rest].into_iter().filter(|part| !part.is_idle()).collect()
}

/// ↩️ The removed/inserted/replaced half of [`inverse_collection_deltas`].
fn inverse_structural_rows<'a, T: Identified<String> + Clone + 'a>(base: &'a [T], earlier: impl Iterator<Item = &'a FlowCollectionDelta<T>> + Clone, delta: &'a FlowCollectionDelta<T>) -> FlowCollectionDelta<T> {
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
        moved: Vec::new(),
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

//#endregion ↩️Rows
