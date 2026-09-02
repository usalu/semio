//! 🧵️ Ordered Flow structural changes corresponding to the adjacent JSON schema.
use super::{apply_flow_collection_delta, FlowCollectionDelta, FlowFixture, FlowLayoutEntry, MutationApplyResult, MutationDiff, SynapseSpec, Widget};
#[cfg(test)]
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🧬️Schema
/// 🗂️ Ordered structural fragments; only the explicit import leaf emits Fixture. `serde` is
/// TEST-ONLY (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS, 26/09/01, tenth-seam
/// pass): `Widget`/`FlowFixture` both lost their own unconditional `Serialize`/`Deserialize` this
/// pass — see `📓️orderedmap-tenth-seam.md`. `MutationDiff<FlowFixture>` below is `ToValue`/
/// `FromValue`-bound already (seam 1), unaffected.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(tag = "delta", content = "value", rename_all = "camelCase", deny_unknown_fields))]
#[value(tag = "delta", content = "value", rename_all = "camelCase", deny_unknown_fields)]
pub enum FlowDelta {
    Widgets(FlowCollectionDelta<Widget>),
    Synapses(FlowCollectionDelta<SynapseSpec>),
    Layout(Vec<FlowLayoutEntry>),
    Fixture(FlowFixture),
}

/// 🧶️ Sequential structural changes compose by concatenation, never by semantic replay. `serde` is
/// TEST-ONLY — see `FlowDelta` above.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", deny_unknown_fields))]
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
