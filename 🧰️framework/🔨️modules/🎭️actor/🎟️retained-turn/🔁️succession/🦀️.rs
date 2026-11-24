//! 🔁️ A kernel-issued successor operation conveys fresh finite funds into one actor Source whose identity stays unchanged.
//! Schema: `🧬️schema/🔣️.json`; laws: `🧪️tests/🧫️fixtures/🔣️.json` (native `🎟️context/🧪️tests/🔁️succession/🦀️.rs`, TypeScript `🧪️tests/🟦️.ts`).
use super::{refusal,RetainedTurnInput};
use semio_framework_value::{RetainedCloneGrant,ValueError};
/// 🪪️ Names the one predecessor turn that pays for the succession and closes the predecessor's funds.
#[derive(Clone,Copy,Debug,PartialEq,Eq,serde::Serialize,serde::Deserialize)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
pub struct OriginalActorSuccession{pub operation:u64,pub generation:u64,pub epoch:u64,pub successor:u64,pub grant:RetainedCloneGrant,pub fuel:u64}
impl OriginalActorSuccession{
 /// ⚖️ Accepts only this exact predecessor turn and a kernel identity issued after every tenure this Source already had.
 pub fn validate_for(&self,input:RetainedTurnInput,tenure:u64)->Result<(),ValueError>{
  input.validate()?;
  if (self.operation,self.generation,self.epoch)!=(input.operation,input.generation,input.epoch){return Err(refusal("actor succession names another predecessor turn"))}
  if self.successor<=input.operation||self.successor<=tenure{return Err(refusal("actor succession repeats an earlier kernel operation"))}
  if self.fuel==0||self.grant.maximum_items==0||self.grant.maximum_depth==0{return Err(refusal("actor succession conveys no usable stock"))}
  Ok(())
 }
}
