//#region 📡️ContributedMutationWireFixture
//! 📡️ Direct test domain for contributed mutation wire planning.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct WireTestSnapshot {
    pub(crate) value: i32,
}

impl crate::store::ArtifactPack for WireTestSnapshot {
    fn encode_pack_with(&self, _options: &crate::store::PackEncodeOptions) -> Result<Vec<u8>, crate::store::PackError> {
        serde_json::to_vec(self).map_err(|error| crate::store::PackError::Schema(error.to_string()))
    }

    fn decode_pack_with(bytes: &[u8], _options: &crate::store::PackDecodeOptions) -> Result<Self, crate::store::PackError> {
        serde_json::from_slice(bytes).map_err(|error| crate::store::PackError::Schema(error.to_string()))
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
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

#[path = "🧬️mutations/🦀️.rs"]
mod mutations;
pub(crate) use mutations::{AddValue, WireTestMutation};

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
//#endregion 📡️ContributedMutationWireFixture
