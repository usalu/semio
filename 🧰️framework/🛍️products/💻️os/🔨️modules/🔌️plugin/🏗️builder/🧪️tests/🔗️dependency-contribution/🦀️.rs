//#region 🔗️DependencyContributionFixture
//! 🔗️ Concrete value contribution used only by PluginBuilder dependency tests.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DependencyTestSnapshot { pub value: i32 }

impl store::ArtifactPack for DependencyTestSnapshot {
    fn encode_pack_with(&self, _options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        serde_json::to_vec(self).map_err(|error| store::PackError::Schema(error.to_string()))
    }
    fn decode_pack_with(bytes: &[u8], _options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        serde_json::from_slice(bytes).map_err(|error| store::PackError::Schema(error.to_string()))
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DependencyTestDiff { pub deltas: Vec<i32> }

impl protocol::MutationDiff<DependencyTestSnapshot> for DependencyTestDiff {
    fn apply(&self, base: &DependencyTestSnapshot) -> protocol::MutationApplyResult<DependencyTestSnapshot> {
        let mut value = base.value;
        for delta in &self.deltas {
            value = value.checked_add(*delta).ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.overflow", "builder contribution exceeds i32").at(["value"]))?;
        }
        Ok(DependencyTestSnapshot { value })
    }
    fn absorb(&mut self, other: Self) { self.deltas.extend(other.deltas); }
}

#[path = "🧬️mutations/🦀️.rs"]
mod mutations;
pub use mutations::{AddValue, DependencyTestOp};

#[path = "🧪️tests/🦀️.rs"]
mod tests;
//#endregion 🔗️DependencyContributionFixture
