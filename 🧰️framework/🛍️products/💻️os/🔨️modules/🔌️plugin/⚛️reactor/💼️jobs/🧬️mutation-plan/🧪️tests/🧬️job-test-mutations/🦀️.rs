//#region 🧬️JobTestMutations
use store::ArtifactPack;

#[derive(Clone,Debug,PartialEq,serde::Serialize,serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct JobTestSnapshot { pub(crate) value:i32 }

impl ArtifactPack for JobTestSnapshot {
    fn encode_pack_with(&self,_:&store::PackEncodeOptions)->Result<Vec<u8>,store::PackError>{serde_json::to_vec(self).map_err(|error|store::PackError::Schema(error.to_string()))}
    fn decode_pack_with(bytes:&[u8],_:&store::PackDecodeOptions)->Result<Self,store::PackError>{serde_json::from_slice(bytes).map_err(|error|store::PackError::Schema(error.to_string()))}
}

/// 🧮️ Ordered checked additions preserve intermediate rejection during structural composition.
#[derive(Clone,Debug,Default,PartialEq,serde::Serialize,serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct JobTestDiff { pub(crate) deltas:Vec<i32> }

impl protocol::MutationDiff<JobTestSnapshot> for JobTestDiff {
    fn apply(&self,base:&JobTestSnapshot)->protocol::MutationApplyResult<JobTestSnapshot>{
        let value=self.deltas.iter().try_fold(base.value,|value,delta|value.checked_add(*delta).ok_or_else(||protocol::MutationApplyError::new("job-test.value-overflow","job fixture value addition exceeds i32").at(["value"])))?;
        Ok(JobTestSnapshot{value})
    }
    fn absorb(&mut self,other:Self){self.deltas.extend(other.deltas);}
}

#[path="🧬️mutations/🦀️.rs"]
mod mutations;
pub(crate) use mutations::{AddValue,JobTestOp};

#[path="🧪️tests/🦀️.rs"]
mod tests;
//#endregion 🧬️JobTestMutations
