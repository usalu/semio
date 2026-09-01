//! 🧵️ Ordered Flow structural changes corresponding to the adjacent JSON schema.
use super::{apply_flow_collection_delta, FlowCollectionDelta, FlowFixture, FlowLayoutEntry, MutationApplyResult, MutationDiff, SynapseSpec, Widget};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🧬️Schema
/// 🗂️ Ordered structural fragments; only the explicit import leaf emits Fixture.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(tag = "delta", content = "value", rename_all = "camelCase", deny_unknown_fields)]
#[value(tag = "delta", content = "value", rename_all = "camelCase", deny_unknown_fields)]
pub enum FlowDelta {
    Widgets(FlowCollectionDelta<Widget>),
    Synapses(FlowCollectionDelta<SynapseSpec>),
    Layout(Vec<FlowLayoutEntry>),
    Fixture(FlowFixture),
}

/// 🧶️ Sequential structural changes compose by concatenation, never by semantic replay.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct FlowDiff {
    pub deltas: Vec<FlowDelta>,
}

impl From<FlowDelta> for FlowDiff {
    fn from(delta: FlowDelta) -> Self { Self { deltas: vec![delta] } }
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
}
//#endregion ▶️Application

//#region 🧪️Ownership
#[cfg(test)]
#[path = "🧪️tests/🧾️ownership/🦀️.rs"]
mod ownership_tests;
//#endregion 🧪️Ownership
