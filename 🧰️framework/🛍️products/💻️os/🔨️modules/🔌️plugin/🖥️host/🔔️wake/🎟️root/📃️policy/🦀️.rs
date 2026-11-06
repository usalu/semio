//! 🎚️ Independently authored Host root and driver policies keep their canonical data contract.
use semio_framework_value::{RetainedCloneGrant,ValueError,ValueRefusalKind};
use semio_framework_value_derive::{FromValue,ToValue};
#[derive(Clone,Copy,Debug,PartialEq,Eq,serde::Serialize,serde::Deserialize,semio_framework_value_derive::FromValue,semio_framework_value_derive::ToValue)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
#[value(rename_all="camelCase")]
pub struct GuestRelayRootPolicy{pub operation:u64,pub generation:u64,pub turn_fuel:u64,pub operation_fuel:u64,pub turn_execution_fuel:u64,pub operation_execution_fuel:u64,pub maximum_turn_microseconds:u64,pub maximum_lifetime_microseconds:u64,pub max_effects:u32,pub max_patch_bytes:u32,pub max_frames:u32}
impl GuestRelayRootPolicy{
 pub fn validate(self)->Result<(),ValueError>{if self.operation==0||self.generation==0||self.turn_fuel==0||self.operation_fuel==0||self.turn_execution_fuel==0||self.operation_execution_fuel==0||self.maximum_turn_microseconds==0||self.maximum_lifetime_microseconds<self.maximum_turn_microseconds{return Err(refusal("original Host root policy requires explicit positive identity, fuel and callback duration"))}Ok(())}
}
#[derive(Clone,Copy,Debug,PartialEq,Eq,serde::Serialize,serde::Deserialize,FromValue,ToValue)]
#[serde(deny_unknown_fields)]
pub struct GuestRelayDriverPolicy{pub root:GuestRelayRootPolicy,pub admission:RetainedCloneGrant,pub drive:RetainedCloneGrant,pub wake:RetainedCloneGrant,pub operation:RetainedCloneGrant}

fn refusal(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
