//#region 📡️ContributedMutationWireFixture
//! 📡️ Direct test domain for contributed mutation wire planning.

use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, ToValue, Deserialize, FromValue)]
#[serde(deny_unknown_fields)]
#[value(deny_unknown_fields)]
pub(crate) struct WireTestSnapshot {
    pub(crate) value: i32,
}

impl store::ArtifactPack for WireTestSnapshot {
    fn encode_pack_with(&self, _options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        serde_json::to_vec(self).map_err(|error| store::PackError::Schema(error.to_string()))
    }

    fn decode_pack_with(bytes: &[u8], _options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        serde_json::from_slice(bytes).map_err(|error| store::PackError::Schema(error.to_string()))
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(deny_unknown_fields)]
#[value(deny_unknown_fields)]
pub(crate) struct WireTestDiff {
    pub(crate) deltas: Vec<i32>,
}

impl protocol::MutationDiff<WireTestSnapshot> for WireTestDiff {
    fn apply(&self, base: &WireTestSnapshot) -> protocol::MutationApplyResult<WireTestSnapshot> {
        let mut value = base.value;
        for delta in &self.deltas {
            value = value.checked_add(*delta).ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.overflow", "contributed wire value exceeds i32").at(["value"]))?;
        }
        Ok(WireTestSnapshot { value })
    }

    fn absorb(&mut self, other: Self) {
        self.deltas.extend(other.deltas);
    }
}

#[path = "../../🧪️testkit/📡️contributed-mutation-wire/🧬️mutations/🦀️.rs"]
mod mutations;
pub(crate) use mutations::{AddValue, WireTestMutation};

#[cfg(test)]
#[path = "../📡️contributed-mutation-wire-unit/🦀️.rs"]
mod tests;
//#endregion 📡️ContributedMutationWireFixture
