//! 🪙️ A sealed snapshot comes only from one actual native recipient and its frozen guest header.
use super::{GuestInstance,GuestRuntimes};
use semio_framework_job::{OriginalNativeBorrowed,OriginalNativeRecipientIdentity,OriginalNativeRetirement,OriginalSourceBinding,OriginalTurnFailure,StepContext};
use semio_framework_value::{RetainedCloneProgress,ValueError};
/// 🪪️ Identity of the actual frozen header and escrow extent, never a caller-supplied pointer.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct GuestFrozenEscrowIdentity{recipient:OriginalNativeRecipientIdentity,source:OriginalSourceBinding,header_address:usize,header_length:usize,header_epoch:u64,escrow_address:usize,escrow_length:usize,revision:u64}
impl GuestFrozenEscrowIdentity{
 /// 📏️ The exact target-specific copy extent for one preborn optional identity slot.
 pub const fn retained_copy_bytes()->usize{std::mem::size_of::<Option<Self>>()}
 pub fn matches(&self,borrow:&GuestFrozenEscrowBorrow<'_>)->bool{*self==borrow.identity}
}
/// 🧾️ Exact Deallocate export return for the header of a frozen snapshot.
pub struct GuestHeaderDeallocateReturned{identity:GuestFrozenEscrowIdentity}
impl GuestHeaderDeallocateReturned{
 /// 📏️ The exact target-specific copy extent for one preborn witness slot.
 pub const fn retained_copy_bytes()->usize{std::mem::size_of::<Option<Self>>()}
 pub fn proves(&self,identity:&GuestFrozenEscrowIdentity,cx:&StepContext<'_>)->bool{self.identity==*identity&&self.identity.source.matches(cx)&&self.identity.recipient.validate_context(cx).is_ok()}
}
/// 📖️ A word view exists only within the original native Source callback.
pub struct GuestFrozenEscrowBorrow<'a>{identity:GuestFrozenEscrowIdentity,header:&'a[u8],escrow:&'a[u8]}
impl GuestFrozenEscrowBorrow<'_>{
 pub fn recipient(&self)->OriginalNativeRecipientIdentity{self.identity.recipient}
 pub fn identity(&self)->GuestFrozenEscrowIdentity{self.identity}
 pub fn validate_context(&self,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{self.identity.recipient.validate_context(cx)?;if !self.identity.source.matches(cx){return Err(ValueError::invariant("frozen escrow changed its original Source").into())}Ok(())}
 pub fn header_len(&self)->usize{self.header.len()}
 pub fn escrow_len(&self)->usize{self.escrow.len()}
 pub fn header_word(&self,offset:usize)->Option<&[u8]>{if offset>=self.header.len(){return None}self.header.get(offset..offset.checked_add(8)?.min(self.header.len()))}
 pub fn escrow_word(&self,offset:usize)->Option<&[u8]>{if offset>=self.escrow.len(){return None}self.escrow.get(offset..offset.checked_add(8)?.min(self.escrow.len()))}
}
fn frozen<'a>(recipient:OriginalNativeRecipientIdentity,header:&'a[u8],escrow:&'a[u8],cx:&StepContext<'_>)->Result<GuestFrozenEscrowBorrow<'a>,OriginalTurnFailure>{if header.len()<40||escrow.len()<48||&escrow[..4]!=b"SRE1"{return Err(ValueError::invariant("frozen escrow header is invalid").into())}let epoch=if &header[..4]==b"SJH2"{32}else{24};let identity=GuestFrozenEscrowIdentity{recipient,source:OriginalSourceBinding::borrow(cx),header_address:header.as_ptr()as usize,header_length:header.len(),header_epoch:u64::from_le_bytes(header[epoch..epoch+8].try_into().unwrap()),escrow_address:escrow.as_ptr()as usize,escrow_length:escrow.len(),revision:u64::from_le_bytes(escrow[32..40].try_into().unwrap())};Ok(GuestFrozenEscrowBorrow{identity,header,escrow})}
impl GuestRuntimes{
 /// 🔍️ Lends the actual frozen unit header and its escrow without retaining either view.
 pub fn with_original_unit_frozen_escrow<T:OriginalNativeRetirement>(&self,mut loan:OriginalNativeBorrowed<'_,T>,project:impl for<'a> FnOnce(&'a mut T)->Result<&'a mut GuestInstance,OriginalTurnFailure>,cx:&mut StepContext<'_>,receive:impl for<'a> FnOnce(GuestFrozenEscrowBorrow<'a>,&mut StepContext<'_>)->Result<(),OriginalTurnFailure>)->Result<bool,OriginalTurnFailure>{
  loan.validate_context(cx)?;let recipient=loan.recipient_identity();let instance=project(loan.value_mut())?;let(Self::Owned(runtime),super::GuestInstanceState::Owned(state))=(self,&instance.state)else{return Err(ValueError::invariant("frozen escrow requires the actual owned instance").into())};let Some(header)=runtime.owned_original_unit_output(state,cx)?else{return Ok(false)};let Some(escrow)=runtime.owned_original_unit_escrow(state,cx)?else{return Ok(false)};receive(frozen(recipient,header,escrow,cx)?,cx)?;Ok(true)
 }
 /// 🔍️ Lends the actual frozen scalar header and its escrow without taking its result.
 pub fn with_original_scalar_frozen_escrow<T:OriginalNativeRetirement>(&self,mut loan:OriginalNativeBorrowed<'_,T>,project:impl for<'a> FnOnce(&'a mut T)->Result<&'a mut GuestInstance,OriginalTurnFailure>,cx:&mut StepContext<'_>,receive:impl for<'a> FnOnce(GuestFrozenEscrowBorrow<'a>,&mut StepContext<'_>)->Result<(),OriginalTurnFailure>)->Result<bool,OriginalTurnFailure>{loan.validate_context(cx)?;let recipient=loan.recipient_identity();let instance=project(loan.value_mut())?;self.with_original_scalar_escrow(instance,cx,|header,escrow,cx|receive(frozen(recipient,header,escrow,cx)?,cx))}
 /// ✅️ Mints a unit-header deallocator-return witness, without asserting guest Source acceptance.
 pub fn attest_original_unit_header_deallocate<T:OriginalNativeRetirement>(&self,mut loan:OriginalNativeBorrowed<'_,T>,project:impl for<'a> FnOnce(&'a mut T)->Result<(&'a mut GuestInstance,&'a mut Option<GuestHeaderDeallocateReturned>),OriginalTurnFailure>,identity:GuestFrozenEscrowIdentity,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  loan.validate_context(cx)?;let recipient=loan.recipient_identity();if recipient!=identity.recipient||!identity.source.matches(cx){return Err(ValueError::invariant("header handback changed its original Source").into())}let(instance,destination)=project(loan.value_mut())?;if destination.is_some(){return Err(ValueError::invariant("header handback destination is occupied").into())}let(Self::Owned(runtime),super::GuestInstanceState::Owned(state))=(self,&instance.state)else{return Err(ValueError::invariant("header handback requires its owned instance").into())};let Some((address,length,epoch))=runtime.owned_original_unit_returned(state,cx)?else{return Ok(false)};if address!=identity.header_address||length!=identity.header_length||epoch!=identity.header_epoch{return Err(ValueError::invariant("header handback differs from its frozen receipt").into())}let bytes=std::mem::size_of::<Option<GuestHeaderDeallocateReturned>>();let grant=cx.retained_grant();if (if cx.original_retirement_active(){cx.retirement_should_yield()}else{cx.should_yield()||cx.is_cancelled()})||grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0||cx.fuel_remaining()==0{return Ok(false)}cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);*destination=Some(GuestHeaderDeallocateReturned{identity});Ok(true)
 }
 /// ✅️ Mints a scalar deallocator-return witness, without asserting guest Source acceptance.
 pub fn attest_original_scalar_header_deallocate<T:OriginalNativeRetirement>(&self,mut loan:OriginalNativeBorrowed<'_,T>,project:impl for<'a> FnOnce(&'a mut T)->Result<(&'a mut GuestInstance,&'a mut Option<GuestHeaderDeallocateReturned>),OriginalTurnFailure>,identity:GuestFrozenEscrowIdentity,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  loan.validate_context(cx)?;let recipient=loan.recipient_identity();if recipient!=identity.recipient||!identity.source.matches(cx){return Err(ValueError::invariant("header handback changed its original Source").into())}let(instance,destination)=project(loan.value_mut())?;if destination.is_some(){return Err(ValueError::invariant("header handback destination is occupied").into())}let(Self::Owned(runtime),super::GuestInstanceState::Owned(state))=(self,&instance.state)else{return Err(ValueError::invariant("header handback requires its owned instance").into())};let Some((address,length))=runtime.original_deallocate_returned(state,cx)?else{return Ok(false)};if address!=identity.header_address||length!=identity.header_length{return Err(ValueError::invariant("header handback differs from its frozen receipt").into())}let bytes=std::mem::size_of::<Option<GuestHeaderDeallocateReturned>>();let grant=cx.retained_grant();if (if cx.original_retirement_active(){cx.retirement_should_yield()}else{cx.should_yield()||cx.is_cancelled()})||grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0||cx.fuel_remaining()==0{return Ok(false)}cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);*destination=Some(GuestHeaderDeallocateReturned{identity});Ok(true)
 }

}
