//! 📤️ One inline original actor receipt preserves every scalar without result allocation.
use super::super::{OriginalActorAdmissionReply,OriginalActorCloseReply,RetainedTurnInput,RetainedCloneProgress,ValueError,ValueRefusalKind};
use semio_framework_value::RetainedCloneGrant;

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum OriginalActorFixedKind{Admission{ready:bool,refusal:Option<ValueRefusalKind>},Close{complete:bool,blocked:bool,refusal:Option<ValueRefusalKind>},Failure{authority:Option<ValueRefusalKind>}}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct OriginalActorFixedCall{pub input:RetainedTurnInput,pub spent:RetainedCloneProgress,pub kind:OriginalActorFixedKind}
/// 🧱️ Caller receipt storage is inline and reusable only after the original returned view is released.
pub struct OriginalActorFixedReceipt{bytes:[u8;104]}
impl OriginalActorFixedReceipt{
 pub const fn empty()->Self{Self{bytes:[0;104]}}
 pub fn bytes(&self)->&[u8;104]{&self.bytes}
 pub fn write_admission(&mut self,original:&OriginalActorAdmissionReply){self.write(OriginalActorFixedCall{input:original.input,spent:original.spent,kind:OriginalActorFixedKind::Admission{ready:original.ready,refusal:original.refusal}})}
 pub fn write_close(&mut self,original:&OriginalActorCloseReply){self.write(OriginalActorFixedCall{input:original.input,spent:original.spent,kind:OriginalActorFixedKind::Close{complete:original.complete,blocked:original.blocked,refusal:original.refusal}})}
 /// 🧾️ Raw actual spend is preserved even when it cannot conserve the supplied grant.
 pub fn write(&mut self,original:OriginalActorFixedCall){
  self.bytes[..4].copy_from_slice(b"SAC1");let(kind,flags,refusal)=match original.kind{OriginalActorFixedKind::Admission{ready,refusal}=>(1,u8::from(ready),refusal),OriginalActorFixedKind::Close{complete,blocked,refusal}=>(2,u8::from(complete)|(u8::from(blocked)<<1),refusal),OriginalActorFixedKind::Failure{authority}=>(3,0,authority)};self.bytes[4..8].copy_from_slice(&[kind,flags,code(refusal),0]);
  let input=original.input;let grant=input.grant;let spent=original.spent;for(index,value)in[input.operation,input.generation,input.epoch,grant.maximum_items as u64,grant.maximum_copy_bytes as u64,grant.maximum_capacity_bytes as u64,grant.maximum_release_bytes as u64,grant.maximum_depth as u64,spent.copied_items as u64,spent.copied_bytes as u64,spent.retained_capacity_bytes as u64,spent.released_bytes as u64].into_iter().enumerate(){self.bytes[8+index*8..16+index*8].copy_from_slice(&value.to_le_bytes());}
 }
 pub fn read(bytes:&[u8])->Result<OriginalActorFixedCall,ValueError>{
  if bytes.len()!=104||&bytes[..4]!=b"SAC1"||bytes[7]!=0{return Err(invalid())}let refusal=kind(bytes[6])?;let flags=bytes[5];let kind=match bytes[4]{1 if flags<=1=>OriginalActorFixedKind::Admission{ready:flags==1,refusal},2 if flags<=2=>OriginalActorFixedKind::Close{complete:flags&1!=0,blocked:flags&2!=0,refusal},3 if flags==0=>OriginalActorFixedKind::Failure{authority:refusal},_=>return Err(invalid())};let mut values=[0u64;12];for(index,value)in values.iter_mut().enumerate(){*value=u64::from_le_bytes(bytes[8+index*8..16+index*8].try_into().unwrap());}let extent=|index:usize|usize::try_from(values[index]).map_err(|_|invalid());Ok(OriginalActorFixedCall{input:RetainedTurnInput{operation:values[0],generation:values[1],epoch:values[2],grant:RetainedCloneGrant{maximum_items:extent(3)?,maximum_copy_bytes:extent(4)?,maximum_capacity_bytes:extent(5)?,maximum_release_bytes:extent(6)?,maximum_depth:extent(7)?}},spent:RetainedCloneProgress{copied_items:extent(8)?,copied_bytes:extent(9)?,retained_capacity_bytes:extent(10)?,released_bytes:extent(11)?},kind})
 }
}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"original fixed actor receipt has an invalid scalar header or native extent")}
fn code(kind:Option<ValueRefusalKind>)->u8{match kind{None=>0,Some(ValueRefusalKind::InvalidValue)=>1,Some(ValueRefusalKind::Canceled)=>2,Some(ValueRefusalKind::OwnershipLimit)=>3,Some(ValueRefusalKind::AllocationFailed)=>4,Some(ValueRefusalKind::WorkLimit)=>5,Some(ValueRefusalKind::DepthLimit)=>6,Some(ValueRefusalKind::UnsupportedOwner)=>7,Some(ValueRefusalKind::InvariantViolated)=>8}}
fn kind(code:u8)->Result<Option<ValueRefusalKind>,ValueError>{Ok(match code{0=>None,1=>Some(ValueRefusalKind::InvalidValue),2=>Some(ValueRefusalKind::Canceled),3=>Some(ValueRefusalKind::OwnershipLimit),4=>Some(ValueRefusalKind::AllocationFailed),5=>Some(ValueRefusalKind::WorkLimit),6=>Some(ValueRefusalKind::DepthLimit),7=>Some(ValueRefusalKind::UnsupportedOwner),8=>Some(ValueRefusalKind::InvariantViolated),_=>return Err(invalid())})}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
