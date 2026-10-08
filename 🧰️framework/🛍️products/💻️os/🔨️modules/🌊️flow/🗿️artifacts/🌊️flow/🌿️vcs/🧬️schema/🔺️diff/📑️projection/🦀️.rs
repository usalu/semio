//! 📑️ Borrowed ordered validation precedes owned Flow snapshot materialization.
use super::FlowDelta;
use super::super::{apply_flow_collection_delta, FlowCollectionDelta, FlowHostSnapshot, FlowLayoutEntry, Identified, MutationApplyError, MutationApplyResult, SynapseSpec, Widget, WidgetLayout};
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
            FlowDelta::HostSnapshot(fixture) => { *self = Self::new(fixture); Ok(()) }
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

    pub(super) fn inverse_of(&self, delta: &FlowDelta) -> FlowDelta {
        match delta {
            FlowDelta::Widgets(delta) => FlowDelta::Widgets(inverse_collection_delta(&self.widgets, delta)),
            FlowDelta::Synapses(delta) => FlowDelta::Synapses(inverse_collection_delta(&self.synapses, delta)),
            FlowDelta::Layout(entries) => FlowDelta::Layout(self.inverse_layout(entries)),
            FlowDelta::HostSnapshot(_) => FlowDelta::HostSnapshot(self.snapshot()),
        }
    }

    fn inverse_layout(&self, entries: &[FlowLayoutEntry]) -> Vec<FlowLayoutEntry> {
        let mut overlay: BTreeMap<&str, Option<&WidgetLayout>> = BTreeMap::new();
        let mut inverse = Vec::with_capacity(entries.len());
        for entry in entries {
            let prior = overlay.get(entry.id.as_str()).copied().unwrap_or_else(|| self.layout.get(entry.id.as_str()).copied());
            inverse.push(FlowLayoutEntry { id: entry.id.clone(), layout: prior.cloned() });
            overlay.insert(entry.id.as_str(), entry.layout.as_ref());
        }
        inverse.reverse();
        inverse
    }

    fn snapshot(&self) -> FlowHostSnapshot {
        FlowHostSnapshot {
            schema: self.host_snapshot.schema.clone(),
            camera: self.host_snapshot.camera.clone(),
            widgets: self.widgets.iter().map(|widget| (*widget).clone()).collect(),
            synapses: self.synapses.iter().map(|synapse| (*synapse).clone()).collect(),
            layout: self.layout.iter().map(|(id, layout)| ((*id).to_owned(), (*layout).clone())).collect(),
        }
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

/// ↩️ The collection delta that turns the post-state back into `items`: the inserted identities leave, replaced items return
/// under their replacement identity, and removed items are re-inserted at their original positions in ascending order.
fn inverse_collection_delta<T: Identified<String> + Clone>(items: &[&T], delta: &FlowCollectionDelta<T>) -> FlowCollectionDelta<T> {
    let mut restored: Vec<(u32, T)> = items
        .iter()
        .enumerate()
        .filter(|(_, item)| delta.removed.contains(item.id()))
        .map(|(index, item)| (index as u32, (*item).clone()))
        .collect();
    restored.sort_by_key(|(index, _)| *index);
    FlowCollectionDelta {
        removed: delta.inserted.iter().map(|(_, item)| item.id().clone()).collect(),
        replaced: delta
            .replaced
            .iter()
            .filter_map(|(id, replacement)| items.iter().find(|item| item.id() == id).map(|prior| (replacement.id().clone(), (*prior).clone())))
            .collect(),
        inserted: restored,
    }
}
//#endregion 📑️Projection
