//! 🎟️ Original operation identity and retained currencies survive every native and component turn.
use protocol::value::{RetainedCloneGrant,RetainedCloneProgress,ValueError,ValueRefusalKind};
use semio_framework_value_derive::{FromValue,ToValue};
#[derive(Clone,Copy,Debug,PartialEq,Eq,serde::Serialize,serde::Deserialize,FromValue,ToValue)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
#[value(crate="::protocol::value")]
pub struct RetainedTurnInput{pub operation:u64,pub generation:u64,pub epoch:u64,pub grant:RetainedCloneGrant}
#[derive(Clone,Copy,Debug,PartialEq,Eq,serde::Serialize,serde::Deserialize,FromValue,ToValue)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
#[value(crate="::protocol::value")]
pub struct RetainedTurnReceipt{pub input:RetainedTurnInput,pub spent:RetainedCloneProgress,pub remaining:RetainedCloneGrant}
impl RetainedTurnInput{
 pub fn validate(&self)->Result<(),ValueError>{if self.operation==0||self.generation==0||self.epoch==0{return Err(refusal("original retained turn requires operation, generation and epoch"))}Ok(())}
 pub fn return_original(self,spent:RetainedCloneProgress)->Result<RetainedTurnReceipt,ValueError>{self.validate()?;if !spent.fits(self.grant){return Err(refusal("original retained receipt exceeds its supplied turn currencies"))}let remaining=RetainedCloneGrant{maximum_items:self.grant.maximum_items-spent.copied_items,maximum_copy_bytes:self.grant.maximum_copy_bytes-spent.copied_bytes,maximum_capacity_bytes:self.grant.maximum_capacity_bytes-spent.retained_capacity_bytes,maximum_release_bytes:self.grant.maximum_release_bytes-spent.released_bytes,maximum_depth:self.grant.maximum_depth};Ok(RetainedTurnReceipt{input:self,spent,remaining})}
}
impl RetainedTurnReceipt{pub fn validate_for(&self,input:RetainedTurnInput)->Result<(),ValueError>{if self.input!=input||input.return_original(self.spent)?!=*self{return Err(refusal("original retained receipt identity or conserved remaining currencies do not match"))}Ok(())}}
fn refusal(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}

impl RetainedTurnInput{
 pub async fn pack_encode(&self,out:&mut Vec<u8>)->Result<(),crate::pack::PackError>{self.validate().map_err(|_|crate::pack::PackError::InvalidRetainedTurn("operation identity is absent"))?;crate::pack::write_u64(out,self.operation).await;crate::pack::write_u64(out,self.generation).await;crate::pack::write_u64(out,self.epoch).await;write_grant(out,self.grant).await;Ok(())}
 pub async fn pack_decode(bytes:&[u8],pos:&mut usize)->Result<Self,crate::pack::PackError>{let value=Self{operation:crate::pack::read_u64(bytes,pos,"RetainedTurnInput::operation").await?,generation:crate::pack::read_u64(bytes,pos,"RetainedTurnInput::generation").await?,epoch:crate::pack::read_u64(bytes,pos,"RetainedTurnInput::epoch").await?,grant:read_grant(bytes,pos).await?};value.validate().map_err(|_|crate::pack::PackError::InvalidRetainedTurn("operation identity is absent"))?;Ok(value)}
}
impl RetainedTurnReceipt{
 pub async fn pack_encode(&self,out:&mut Vec<u8>)->Result<(),crate::pack::PackError>{self.validate_for(self.input).map_err(|_|crate::pack::PackError::InvalidRetainedTurn("receipt does not conserve original currencies"))?;self.input.pack_encode(out).await?;for value in[self.spent.copied_items,self.spent.copied_bytes,self.spent.retained_capacity_bytes,self.spent.released_bytes]{crate::pack::write_u64(out,value as u64).await;}write_grant(out,self.remaining).await;Ok(())}
 pub async fn pack_decode(bytes:&[u8],pos:&mut usize)->Result<Self,crate::pack::PackError>{let value=Self{input:RetainedTurnInput::pack_decode(bytes,pos).await?,spent:RetainedCloneProgress{copied_items:read_extent(bytes,pos).await?,copied_bytes:read_extent(bytes,pos).await?,retained_capacity_bytes:read_extent(bytes,pos).await?,released_bytes:read_extent(bytes,pos).await?},remaining:read_grant(bytes,pos).await?};value.validate_for(value.input).map_err(|_|crate::pack::PackError::InvalidRetainedTurn("receipt does not conserve original currencies"))?;Ok(value)}
}
async fn write_grant(out:&mut Vec<u8>,grant:RetainedCloneGrant){for value in[grant.maximum_items,grant.maximum_copy_bytes,grant.maximum_capacity_bytes,grant.maximum_release_bytes,grant.maximum_depth]{crate::pack::write_u64(out,value as u64).await;}}
async fn read_extent(bytes:&[u8],pos:&mut usize)->Result<usize,crate::pack::PackError>{usize::try_from(crate::pack::read_u64(bytes,pos,"RetainedTurn::extent").await?).map_err(|_|crate::pack::PackError::InvalidRetainedTurn("wire extent exceeds this native address space"))}
async fn read_grant(bytes:&[u8],pos:&mut usize)->Result<RetainedCloneGrant,crate::pack::PackError>{Ok(RetainedCloneGrant{maximum_items:read_extent(bytes,pos).await?,maximum_copy_bytes:read_extent(bytes,pos).await?,maximum_capacity_bytes:read_extent(bytes,pos).await?,maximum_release_bytes:read_extent(bytes,pos).await?,maximum_depth:read_extent(bytes,pos).await?})}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

/// 🤝️ The exact Host input remains held through separate funded guest admission turns.
#[derive(Clone,Copy,Debug,PartialEq,Eq,serde::Serialize,serde::Deserialize)]
#[serde(rename_all="camelCase")]
pub enum ActorGuestPhase{Pending,Ready,Dispatched}

/// 🧳️ Owns one original input and one checked-out guest grant under the current issued Actor turn.
pub struct OriginalActorInput<T>{input:std::mem::ManuallyDrop<Option<T>>,issued:Option<RetainedTurnInput>,returned:Option<RetainedTurnReceipt>,guest:Option<RetainedTurnInput>,spent:RetainedCloneProgress,phase:ActorGuestPhase,refused:Option<RetainedTurnReceipt>}
impl<T> OriginalActorInput<T>{
 pub fn new(input:T,issued:RetainedTurnInput)->Result<Self,(ValueError,T)>{if let Err(error)=issued.validate(){return Err((error,input))}Ok(Self{input:std::mem::ManuallyDrop::new(Some(input)),issued:Some(issued),returned:None,guest:None,spent:Default::default(),phase:ActorGuestPhase::Pending,refused:None})}
 pub fn original(&self)->Option<&T>{self.input.as_ref()}
 pub fn phase(&self)->ActorGuestPhase{self.phase}
 pub fn refused_receipt(&self)->Option<&RetainedTurnReceipt>{self.refused.as_ref()}
 pub fn spent(&self)->RetainedCloneProgress{self.spent}
 /// 🎟️ Lends the unchanged identity and the five-axis minimum from the original remaining currencies.
 pub fn checkout_guest(&mut self,policy:RetainedCloneGrant)->Result<Option<RetainedTurnInput>,ValueError>{if self.guest.is_some()||self.refused.is_some(){return Err(refusal("original guest checkout is still held"))}let original=self.issued.ok_or_else(||refusal("original Actor turn has already returned"))?;let remaining=original.return_original(self.spent)?.remaining;let grant=RetainedCloneGrant{maximum_items:remaining.maximum_items.min(policy.maximum_items),maximum_copy_bytes:remaining.maximum_copy_bytes.min(policy.maximum_copy_bytes),maximum_capacity_bytes:remaining.maximum_capacity_bytes.min(policy.maximum_capacity_bytes),maximum_release_bytes:remaining.maximum_release_bytes.min(policy.maximum_release_bytes),maximum_depth:remaining.maximum_depth.min(policy.maximum_depth)};if grant.maximum_items==0||grant.maximum_depth==0{return Ok(None)}let guest=RetainedTurnInput{grant,..original};self.guest=Some(guest);Ok(Some(guest))}
 /// 🧾️ Collects the exact original child delta once before publishing readiness or dispatch.
 pub fn receive_guest(&mut self,receipt:RetainedTurnReceipt,phase:ActorGuestPhase)->Result<(),ValueError>{let Some(guest)=self.guest.take()else{return Err(refusal("original guest turn was not checked out"))};let valid=receipt.validate_for(guest);let added=(||Ok::<_,ValueError>(RetainedCloneProgress{copied_items:self.spent.copied_items.checked_add(receipt.spent.copied_items).ok_or_else(||refusal("original guest items overflow"))?,copied_bytes:self.spent.copied_bytes.checked_add(receipt.spent.copied_bytes).ok_or_else(||refusal("original guest copy overflow"))?,retained_capacity_bytes:self.spent.retained_capacity_bytes.checked_add(receipt.spent.retained_capacity_bytes).ok_or_else(||refusal("original guest capacity overflow"))?,released_bytes:self.spent.released_bytes.checked_add(receipt.spent.released_bytes).ok_or_else(||refusal("original guest release overflow"))?}))();let spent=match added{Ok(spent)=>spent,Err(error)=>{self.refused=Some(receipt);return Err(error)}};self.spent=spent;if let Err(error)=valid{self.refused=Some(receipt);return Err(error)}if let Err(error)=self.issued.unwrap().return_original(spent){self.refused=Some(receipt);return Err(error)}if (self.phase==ActorGuestPhase::Pending&&phase==ActorGuestPhase::Dispatched)||(self.phase==ActorGuestPhase::Ready&&phase==ActorGuestPhase::Pending)||(self.phase==ActorGuestPhase::Dispatched&&phase!=ActorGuestPhase::Dispatched){self.refused=Some(receipt);return Err(refusal("original guest semantic phase does not follow paid readiness"))}self.phase=phase;Ok(())}
 /// 📤️ Returns one conserved parent receipt while retaining the same Host input for the next issued turn.
 pub fn return_actor_turn(&mut self)->Result<RetainedTurnReceipt,ValueError>{if self.guest.is_some()||self.refused.is_some(){return Err(refusal("original guest receipt remains held"))}let issued=self.issued.ok_or_else(||refusal("original Actor turn has already returned"))?;let receipt=issued.return_original(self.spent)?;self.issued=None;self.returned=Some(receipt);Ok(receipt)}
 /// 🫴️ Accepts only the scheduler's exact next epoch and the previous original conserved remaining grant.
 pub fn receive_next_issued(&mut self,issued:RetainedTurnInput)->Result<(),ValueError>{issued.validate()?;if self.issued.is_some()||self.guest.is_some()||self.refused.is_some(){return Err(refusal("original Actor turn still has a receiving owner"))}let previous=self.returned.ok_or_else(||refusal("original Actor turn has no prior return"))?;if issued.operation!=previous.input.operation||issued.generation!=previous.input.generation||Some(issued.epoch)!=previous.input.epoch.checked_add(1)||issued.grant!=previous.remaining{return Err(refusal("next issued Actor input does not preserve original remaining authority"))}self.issued=Some(issued);self.returned=None;self.spent=Default::default();Ok(())}
 /// 📬️ Transfers the original input only after paid readiness and successful semantic dispatch.
 pub fn take_dispatched_input(&mut self)->Result<Option<T>,ValueError>{if self.phase!=ActorGuestPhase::Dispatched||self.guest.is_some()||self.refused.is_some(){return Err(refusal("original Host input still requires its receiving owner"))}Ok(self.input.take())}
}
impl<T> Drop for OriginalActorInput<T>{fn drop(&mut self){assert!(std::thread::panicking()||(self.input.is_none()&&self.guest.is_none()&&self.refused.is_none()),"original Host input reached Drop before its receiving owner transferred it")}}
