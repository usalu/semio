//! 🪙️ A sealed snapshot comes only from one actual native recipient and its frozen guest header.
use super::{GuestInstance,GuestRuntimes,OwnedSemioExport};
use semio_framework_job::{OriginalNativeBorrowed,OriginalNativeRecipientIdentity,OriginalNativeRetirement,OriginalSourceBinding,OriginalTurnFailure,StepContext};
use semio_framework_value::{RetainedCloneProgress,ValueError};

/// 🪪️ Runtime-sealed causal origin.
#[derive(Clone,Copy)]
pub struct GuestActorFailureCaptureOrigin{_private:()}
pub type GuestActorFailureCallReference=semio_framework_actor::retained_context::OriginalActorFailureCall<GuestActorFailureCaptureOrigin>;

/// 🪪️ Identity of the actual frozen header and escrow extent, never a caller-supplied pointer.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct GuestFrozenEscrowIdentity{recipient:OriginalNativeRecipientIdentity,source:OriginalSourceBinding,header_address:usize,header_length:usize,header_epoch:u64,escrow_address:usize,escrow_length:usize,revision:u64,export:OwnedSemioExport}
impl GuestFrozenEscrowIdentity{
 pub fn publication_revision(&self)->u64{self.revision}
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
 /// 🧭️ Reveals the actual runtime-frozen export, sealed with this borrowed header.
 pub fn export(&self)->OwnedSemioExport{self.identity.export}
 pub fn recipient(&self)->OriginalNativeRecipientIdentity{self.identity.recipient}
 pub fn identity(&self)->GuestFrozenEscrowIdentity{self.identity}
 pub fn validate_context(&self,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{self.identity.recipient.validate_context(cx)?;if !self.identity.source.matches(cx){return Err(ValueError::invariant("frozen escrow changed its original Source").into())}Ok(())}
 pub fn header_len(&self)->usize{self.header.len()}
 pub fn escrow_len(&self)->usize{self.escrow.len()}
 pub fn header_word(&self,offset:usize)->Option<&[u8]>{if offset>=self.header.len(){return None}self.header.get(offset..offset.checked_add(8)?.min(self.header.len()))}
 pub fn escrow_word(&self,offset:usize)->Option<&[u8]>{if offset>=self.escrow.len(){return None}self.escrow.get(offset..offset.checked_add(8)?.min(self.escrow.len()))}
}
fn frozen<'a>(recipient:OriginalNativeRecipientIdentity,export:OwnedSemioExport,header:&'a[u8],escrow:&'a[u8],cx:&StepContext<'_>)->Result<GuestFrozenEscrowBorrow<'a>,OriginalTurnFailure>{if header.len()<40||escrow.len()<48||&escrow[..4]!=b"SRE1"{return Err(ValueError::invariant("frozen escrow header is invalid").into())}let epoch=if matches!(&header[..4],b"SJH2"|b"SAF1"|b"SBT1"){32}else{24};let identity=GuestFrozenEscrowIdentity{recipient,export,source:OriginalSourceBinding::borrow(cx),header_address:header.as_ptr()as usize,header_length:header.len(),header_epoch:u64::from_le_bytes(header[epoch..epoch+8].try_into().unwrap()),escrow_address:escrow.as_ptr()as usize,escrow_length:escrow.len(),revision:u64::from_le_bytes(escrow[32..40].try_into().unwrap())};Ok(GuestFrozenEscrowBorrow{identity,header,escrow})}
fn failure_capture_refusal()->ValueError{ValueError::invariant("actor failure capture requires its exact actual frozen source-scoped cause")}
fn capture_header_return<T>(destination:&mut Option<T>,make:impl FnOnce()->T,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{let bytes=std::mem::size_of::<Option<T>>();let grant=cx.retained_grant();if (if cx.original_retirement_active(){cx.retirement_should_yield()}else{cx.should_yield()||cx.is_cancelled()})||grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0||cx.fuel_remaining()==0{return Ok(false)}cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);*destination=Some(make());Ok(true)}
impl GuestRuntimes{
 /// 🔎️ Borrows frozen bytes and the disjoint retained controller from the same actual native recipient.
 pub fn with_original_frozen_escrow_parts<T:OriginalNativeRetirement,U>(&self,scalar:bool,mut loan:OriginalNativeBorrowed<'_,T>,project:impl for<'a> FnOnce(&'a mut T)->Result<(&'a mut GuestInstance,&'a mut U),OriginalTurnFailure>,cx:&mut StepContext<'_>,receive:impl for<'a> FnOnce(GuestFrozenEscrowBorrow<'a>,&mut U,&mut StepContext<'_>)->Result<(),OriginalTurnFailure>)->Result<bool,OriginalTurnFailure>{loan.validate_context(cx)?;let recipient=loan.recipient_identity();let(instance,payload)=project(loan.value_mut())?;let Some(export)=(if scalar{self.original_scalar_export(instance,cx)?}else{self.original_unit_export(instance,cx)?})else{return Ok(false)};if scalar{return self.with_original_scalar_escrow(instance,cx,|header,escrow,cx|receive(frozen(recipient,export,header,escrow,cx)?,payload,cx))}let(Self::Owned(runtime),super::GuestInstanceState::Owned(state))=(self,&instance.state)else{return Err(ValueError::invariant("frozen escrow requires its owned instance").into())};let Some(header)=runtime.owned_original_unit_output(state,cx)?else{return Ok(false)};let Some(escrow)=runtime.owned_original_unit_escrow(state,cx)?else{return Ok(false)};receive(frozen(recipient,export,header,escrow,cx)?,payload,cx)?;Ok(true)}

 /// ⚠️ Captures causal DATA only from the actual failed frozen export and the same native recipient.
 pub fn attest_original_actor_failure<T:OriginalNativeRetirement>(&self,scalar:bool,mut loan:OriginalNativeBorrowed<'_,T>,project:impl for<'a> FnOnce(&'a mut T)->Result<(&'a mut GuestInstance,&'a mut Option<GuestActorFailureCallReference>),OriginalTurnFailure>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  loan.validate_context(cx)?;let recipient=loan.recipient_identity();let(instance,out)=project(loan.value_mut())?;let Some(export)=(if scalar{self.original_scalar_export(instance,cx)?}else{self.original_unit_export(instance,cx)?})else{return Ok(false)};let tag=super::OwnedSemioExport::ALL.iter().position(|value|*value==export).ok_or_else(||failure_capture_refusal())?as u64+1;let call_kind=semio_framework_actor::ActorFailureCallKind::from_tag(tag)?;let mut reference=None;
  let mut receive=|header:&[u8],_:&[u8],cx:&mut StepContext<'_>|{if header.len()==184&&(&header[..4]!=b"SAF1"||header[4]!=0){return Err(failure_capture_refusal().into())}let bytes=if matches!(header.len(),184|200){&header[8..152]}else{header};let(call,_)=semio_plugin_guest_contract::original_actor_context::OriginalActorFixedReceipt::read(bytes)?;if !call.kind.is_failure_for(call_kind){return Err(failure_capture_refusal().into())}if header.len()==184&&(u64::from_le_bytes(header[152..160].try_into().unwrap())!=call.input.epoch||u64::from_le_bytes(header[160..168].try_into().unwrap())!=tag){return Err(failure_capture_refusal().into())}let actual=semio_framework_actor::RetainedTurnInput::borrow_original(cx)?;if call.input.operation!=actual.operation||call.input.generation!=actual.generation||call.input.work.clock_binding!=actual.work.clock_binding{return Err(failure_capture_refusal().into())}reference=Some(semio_framework_actor::ActorFailureCallReference{failed_epoch:call.input.epoch,call_kind});Ok(())};
  let available=if scalar{self.with_original_scalar_escrow(instance,cx,receive)?}else{let(Self::Owned(runtime),super::GuestInstanceState::Owned(state))=(self,&instance.state)else{return Err(failure_capture_refusal().into())};match(runtime.owned_original_unit_output(state,cx)?,runtime.owned_original_unit_escrow(state,cx)?){(Some(header),Some(escrow))=>{receive(header,escrow,cx)?;true},_=>false}};if !available{return Ok(false)}let reference=reference.ok_or_else(||failure_capture_refusal())?;reference.validate()?;if let Some(saved)=out.as_ref(){return Ok(saved.reference_for(recipient,cx)?==reference)}let bytes=std::mem::size_of::<Option<GuestActorFailureCallReference>>();let grant=cx.retained_grant();if (if cx.original_retirement_active(){cx.retirement_should_yield()}else{cx.should_yield()||cx.is_cancelled()})||grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0||cx.fuel_remaining()==0{return Ok(false)}let captured=GuestActorFailureCallReference::captured_by(GuestActorFailureCaptureOrigin{_private:()},reference,recipient,cx)?;cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);*out=Some(captured);Ok(true)
 }
 /// ✅️ Mints a unit-header deallocator-return witness, without asserting guest Source acceptance.
 pub fn attest_original_unit_header_deallocate<T:OriginalNativeRetirement>(&self,mut loan:OriginalNativeBorrowed<'_,T>,project:impl for<'a> FnOnce(&'a mut T)->Result<(&'a mut GuestInstance,&'a mut Option<GuestHeaderDeallocateReturned>),OriginalTurnFailure>,identity:GuestFrozenEscrowIdentity,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  loan.validate_context(cx)?;let recipient=loan.recipient_identity();if recipient!=identity.recipient||!identity.source.matches(cx){return Err(ValueError::invariant("header handback changed its original Source").into())}let(instance,destination)=project(loan.value_mut())?;if destination.is_some(){return Err(ValueError::invariant("header handback destination is occupied").into())}let(Self::Owned(runtime),super::GuestInstanceState::Owned(state))=(self,&instance.state)else{return Err(ValueError::invariant("header handback requires its owned instance").into())};let Some((address,length,epoch))=runtime.owned_original_unit_returned(state,cx)?else{return Ok(false)};if address!=identity.header_address||length!=identity.header_length||epoch!=identity.header_epoch{return Err(ValueError::invariant("header handback differs from its frozen receipt").into())}capture_header_return(destination,||GuestHeaderDeallocateReturned{identity},cx)
 }
 /// ✅️ Mints a scalar deallocator-return witness, without asserting guest Source acceptance.
 pub fn attest_original_scalar_header_deallocate<T:OriginalNativeRetirement>(&self,mut loan:OriginalNativeBorrowed<'_,T>,project:impl for<'a> FnOnce(&'a mut T)->Result<(&'a mut GuestInstance,&'a mut Option<GuestHeaderDeallocateReturned>),OriginalTurnFailure>,identity:GuestFrozenEscrowIdentity,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  loan.validate_context(cx)?;let recipient=loan.recipient_identity();if recipient!=identity.recipient||!identity.source.matches(cx){return Err(ValueError::invariant("header handback changed its original Source").into())}let(instance,destination)=project(loan.value_mut())?;if destination.is_some(){return Err(ValueError::invariant("header handback destination is occupied").into())}let(Self::Owned(runtime),super::GuestInstanceState::Owned(state))=(self,&instance.state)else{return Err(ValueError::invariant("header handback requires its owned instance").into())};let Some((address,length))=runtime.original_deallocate_returned(state,cx)?else{return Ok(false)};if address!=identity.header_address||length!=identity.header_length{return Err(ValueError::invariant("header handback differs from its frozen receipt").into())}capture_header_return(destination,||GuestHeaderDeallocateReturned{identity},cx)
 }

}

/// 🪪️ Bootstrap identity names only the actual header, export and native origin, without escrow fields.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct GuestBootstrapHeaderIdentity{recipient:OriginalNativeRecipientIdentity,source:OriginalSourceBinding,header_address:usize,header_length:usize,header_epoch:u64,sequence:u64,export:OwnedSemioExport}
impl GuestBootstrapHeaderIdentity{pub const fn retained_copy_bytes()->usize{std::mem::size_of::<Option<Self>>()}}
/// ✅️ Exact physical Deallocate return for the actual pre-Source bootstrap header.
pub struct GuestBootstrapHeaderDeallocateReturned{identity:GuestBootstrapHeaderIdentity}
impl GuestBootstrapHeaderDeallocateReturned{pub const fn retained_copy_bytes()->usize{std::mem::size_of::<Option<Self>>()}pub fn proves(&self,identity:&GuestBootstrapHeaderIdentity,cx:&StepContext<'_>)->bool{self.identity==*identity&&self.identity.source.matches(cx)&&self.identity.recipient.validate_context(cx).is_ok()}}
/// 🌱️ A pre-Source bootstrap header is borrowed from the actual frozen ActorContext export.
pub struct GuestFrozenBootstrapBorrow<'a>{identity:GuestBootstrapHeaderIdentity,header:&'a[u8]}
impl GuestFrozenBootstrapBorrow<'_>{
 pub fn identity(&self)->GuestBootstrapHeaderIdentity{self.identity}
 pub fn recipient(&self)->OriginalNativeRecipientIdentity{self.identity.recipient}
 pub fn export(&self)->OwnedSemioExport{self.identity.export}
 pub fn validate_context(&self,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{self.identity.recipient.validate_context(cx)?;if !self.identity.source.matches(cx){return Err(ValueError::invariant("bootstrap header changed its actual Source").into())}Ok(())}
 pub fn header_word(&self,offset:usize)->Option<&[u8]>{if offset>=self.header.len(){return None}self.header.get(offset..offset.checked_add(8)?.min(self.header.len()))}
}
impl GuestRuntimes{
 /// 🔎️ Lends only actual SAB1 backing; a Source-ready header selects the separate escrow path.
 pub fn with_original_bootstrap_parts<T:OriginalNativeRetirement,U>(&self,mut loan:OriginalNativeBorrowed<'_,T>,project:impl for<'a> FnOnce(&'a mut T)->Result<(&'a mut GuestInstance,&'a mut U),OriginalTurnFailure>,cx:&mut StepContext<'_>,receive:impl for<'a> FnOnce(GuestFrozenBootstrapBorrow<'a>,&mut U,&mut StepContext<'_>)->Result<(),OriginalTurnFailure>)->Result<Option<bool>,OriginalTurnFailure>{
  loan.validate_context(cx)?;let recipient=loan.recipient_identity();let(instance,payload)=project(loan.value_mut())?;let mut delivered=false;
  let ready=self.with_original_scalar_header(instance,cx,|export,header,cx|{
   if !matches!(export,OwnedSemioExport::AdmitActorContext|OwnedSemioExport::CloseActorContext){return Err(ValueError::invariant("bootstrap header requires its actual ActorContext export").into())}
   if header.len()!=304{return Ok(())}
   if header[..8]!=[83,65,66,49,1,0,48,1]{return Err(ValueError::invariant("bootstrap header changed its closed namespace").into())}
   let identity=GuestBootstrapHeaderIdentity{recipient,source:OriginalSourceBinding::borrow(cx),header_address:header.as_ptr()as usize,header_length:header.len(),header_epoch:u64::from_le_bytes(header[24..32].try_into().unwrap()),sequence:u64::from_le_bytes(header[280..288].try_into().unwrap()),export};
   receive(GuestFrozenBootstrapBorrow{identity,header},payload,cx)?;delivered=true;Ok(())
  })?;Ok(ready.then_some(delivered))
 }
}

impl GuestRuntimes{
 /// 🧾️ Mints bootstrap handback only from the actual same-recipient Core Deallocate result.
 pub fn attest_original_bootstrap_header_deallocate<T:OriginalNativeRetirement>(&self,mut loan:OriginalNativeBorrowed<'_,T>,project:impl for<'a> FnOnce(&'a mut T)->Result<(&'a mut GuestInstance,&'a mut Option<GuestBootstrapHeaderDeallocateReturned>),OriginalTurnFailure>,identity:GuestBootstrapHeaderIdentity,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{loan.validate_context(cx)?;if loan.recipient_identity()!=identity.recipient||!identity.source.matches(cx){return Err(ValueError::invariant("bootstrap handback changed its actual Source").into())}let(instance,out)=project(loan.value_mut())?;if out.is_some(){return Err(ValueError::invariant("bootstrap handback slot is occupied").into())}let(Self::Owned(runtime),super::GuestInstanceState::Owned(state))=(self,&instance.state)else{return Err(ValueError::invariant("bootstrap handback requires its actual owned instance").into())};let Some((address,length))=runtime.original_deallocate_returned(state,cx)?else{return Ok(false)};if address!=identity.header_address||length!=identity.header_length{return Err(ValueError::invariant("bootstrap handback differs from its frozen header").into())}capture_header_return(out,||GuestBootstrapHeaderDeallocateReturned{identity},cx)}
}

/// 🔐️ Native recipient-bound acknowledgment retains the lower interpreter proof without opening a constructor.
pub struct GuestActorFailureAcknowledged{proof:semio_plugin_interpreter_runtime::OwnedActorFailureAcknowledged,recipient:OriginalNativeRecipientIdentity,source:OriginalSourceBinding}
impl GuestActorFailureAcknowledged{pub fn reference_for(&self,recipient:OriginalNativeRecipientIdentity,cx:&StepContext<'_>)->Result<semio_framework_actor::ActorFailureCallReference,OriginalTurnFailure>{recipient.validate_context(cx)?;if recipient!=self.recipient||!self.source.matches(cx){return Err(failure_capture_refusal().into())}Ok(self.proof.reference())}pub fn runtime_proof(&self)->&semio_plugin_interpreter_runtime::OwnedActorFailureAcknowledged{&self.proof}}
impl GuestRuntimes{
 /// 📷️ Captures only the actual acknowledged SAF1 while its original guest bytes are still frozen.
 pub fn capture_original_actor_failure_ack(&self,instance:&GuestInstance,out:&mut Option<semio_plugin_interpreter_runtime::OwnedActorFailureAckFrame>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{let(Self::Owned(runtime),super::GuestInstanceState::Owned(state))=(self,&instance.state)else{return Err(failure_capture_refusal().into())};runtime.capture_actor_failure_ack_step(state,out,cx)}
 /// 🧾️ Preserves the lower proof until its actual native producer can seal it.
 pub fn finish_original_actor_failure_ack(&self,instance:&GuestInstance,frame:&mut Option<semio_plugin_interpreter_runtime::OwnedActorFailureAckFrame>,out:&mut Option<semio_plugin_interpreter_runtime::OwnedActorFailureAcknowledged>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{let(Self::Owned(runtime),super::GuestInstanceState::Owned(state))=(self,&instance.state)else{return Err(failure_capture_refusal().into())};runtime.finish_actor_failure_ack_step(state,frame,out,cx)}
 /// 🪢️ Only the recipient-bound genuine acknowledgment can unlock its retained failed wire.
 pub fn authorize_original_input_cleanup(&self,instance:&mut GuestInstance,ack:&GuestActorFailureAcknowledged,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{let(Self::Owned(runtime),super::GuestInstanceState::Owned(state))=(self,&mut instance.state)else{return Err(failure_capture_refusal().into())};runtime.authorize_failed_input_cleanup_step(state,ack.runtime_proof(),cx)}
 /// ✅️ Binds only the same saved causal reference and native producer after its real Ack header handback.
 pub fn seal_original_actor_failure_ack<T:OriginalNativeRetirement>(&self,mut loan:OriginalNativeBorrowed<'_,T>,project:impl for<'a> FnOnce(&'a mut T)->Result<(&'a mut GuestInstance,&'a GuestActorFailureCallReference,&'a GuestHeaderDeallocateReturned,&'a mut Option<semio_plugin_interpreter_runtime::OwnedActorFailureAcknowledged>,&'a mut Option<GuestActorFailureAcknowledged>),OriginalTurnFailure>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{loan.validate_context(cx)?;let recipient=loan.recipient_identity();let(instance,reference,returned,proof,out)=project(loan.value_mut())?;let expected=reference.reference_for(recipient,cx)?;if returned.identity.recipient!=recipient||returned.identity.export!=OwnedSemioExport::AcknowledgeActorFailure||!returned.proves(&returned.identity,cx){return Err(failure_capture_refusal().into())}let(Self::Owned(_),super::GuestInstanceState::Owned(state))=(self,&instance.state)else{return Err(failure_capture_refusal().into())};let value=proof.as_ref().ok_or_else(failure_capture_refusal)?;if out.is_some()||value.reference()!=expected||!value.proves(state,cx)||!value.matches_header(returned.identity.header_address,returned.identity.header_length){return Err(failure_capture_refusal().into())}let bytes=std::mem::size_of_val(proof)+std::mem::size_of_val(out);let grant=cx.retained_grant();if cx.retirement_should_yield()||grant.maximum_items<2||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0||cx.fuel_remaining()<2{return Ok(false)}cx.consume_retained(RetainedCloneProgress{copied_items:2,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(2);*out=Some(GuestActorFailureAcknowledged{proof:proof.take().unwrap(),recipient,source:OriginalSourceBinding::borrow(cx)});Ok(true)}
}
