//! 📑️ Borrowed ordered validation precedes owned Flow snapshot materialization.
use super::FlowDelta;
use super::super::{apply_flow_collection_delta, FlowFixture, FlowLayoutEntry, Identified, MutationApplyError, MutationApplyResult, SynapseSpec, Widget, WidgetLayout};
use std::collections::BTreeMap;

//#region 📑️Projection
pub(super) struct FlowProjection<'a> {
    fixture: &'a FlowFixture,
    widgets: Vec<&'a Widget>,
    synapses: Vec<&'a SynapseSpec>,
    layout: BTreeMap<&'a str, &'a WidgetLayout>,
}

impl<'a> FlowProjection<'a> {
    pub(super) fn new(fixture: &'a FlowFixture) -> Self {
        Self {
            fixture,
            widgets: fixture.widgets.iter().collect(),
            synapses: fixture.synapses.iter().collect(),
            layout: fixture.layout.iter().map(|(id, layout)| (id.as_str(), layout)).collect(),
        }
    }

    pub(super) fn apply(&mut self, delta: &'a FlowDelta) -> MutationApplyResult<()> {
        match delta {
            FlowDelta::Widgets(delta) => apply_flow_collection_delta(&mut self.widgets, delta).map_err(|error| error.under(["widgets"])),
            FlowDelta::Synapses(delta) => apply_flow_collection_delta(&mut self.synapses, delta).map_err(|error| error.under(["synapses"])),
            FlowDelta::Layout(entries) => self.apply_layout(entries),
            FlowDelta::Fixture(fixture) => { *self = Self::new(fixture); Ok(()) }
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

    pub(super) fn materialize(self) -> FlowFixture {
        FlowFixture {
            schema: self.fixture.schema.clone(),
            camera: self.fixture.camera.clone(),
            widgets: self.widgets.into_iter().cloned().collect(),
            synapses: self.synapses.into_iter().cloned().collect(),
            layout: self.layout.into_iter().map(|(id, layout)| (id.to_owned(), layout.clone())).collect(),
        }
    }
}
//#endregion 📑️Projection
