//! 🧵️ Ordered Flow structural changes corresponding to the adjacent JSON schema.
use super::{FlowCollectionDelta, FlowFixture, FlowLayoutEntry, FlowOwner, FlowRetirement, MutationApplyResult, MutationDiff, SynapseSpec, Widget};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🧬️Schema
/// 🗂️ Ordered structural fragments; only the explicit import leaf emits Fixture.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(tag = "delta", content = "value", rename_all = "camelCase", deny_unknown_fields)]
pub enum FlowDelta {
    Widgets(FlowCollectionDelta<Widget>),
    Synapses(FlowCollectionDelta<SynapseSpec>),
    Layout(Vec<FlowLayoutEntry>),
    Fixture(FlowFixture),
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
    /// 🧊️ Hands one structural fragment's owners to `frontier`; `Fixture` carries the fail-closed
    /// `OrderedMap<WidgetLayout>` root that aborts the process on a bare drop
    /// (`🌱️value/🗂️ordered/🦀️.rs`'s `Drop`), and both collection deltas own whole `Widget`s.
    fn handoff(self, frontier: &mut FlowRetirement) {
        match self {
            Self::Widgets(delta) => {
                frontier.push(FlowOwner::Strings(delta.removed));
                frontier.push(FlowOwner::Widgets(delta.inserted.into_iter().map(|(_, widget)| widget).collect()));
                let (ids, widgets): (Vec<String>, Vec<Widget>) = delta.replaced.into_iter().unzip();
                frontier.push(FlowOwner::Strings(ids));
                frontier.push(FlowOwner::Widgets(widgets));
            }
            Self::Synapses(delta) => {
                frontier.push(FlowOwner::Strings(delta.removed));
                frontier.push(FlowOwner::Specs(delta.inserted.into_iter().map(|(_, spec)| spec).collect()));
                let (ids, specs): (Vec<String>, Vec<SynapseSpec>) = delta.replaced.into_iter().unzip();
                frontier.push(FlowOwner::Strings(ids));
                frontier.push(FlowOwner::Specs(specs));
            }
            Self::Layout(entries) => frontier.push(FlowOwner::Layout(entries)),
            Self::Fixture(fixture) => frontier.push(FlowOwner::Fixture(fixture)),
        }
    }
}
//#endregion 🧬️Schema

//#region ▶️Application
#[path = "📑️projection/🦀️.rs"]
mod projection;
use projection::FlowProjection;

impl MutationDiff<FlowFixture> for FlowDiff {
    fn apply(&self, snapshot: &FlowFixture) -> MutationApplyResult<FlowFixture> {
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

    fn retire_projection(projection: FlowFixture) { projection.retire_cold(); }
}
//#endregion ▶️Application

//#region 🧪️Ownership
#[cfg(test)]
#[path = "🧪️tests/🧾️ownership/🦀️.rs"]
mod ownership_tests;
//#endregion 🧪️Ownership
