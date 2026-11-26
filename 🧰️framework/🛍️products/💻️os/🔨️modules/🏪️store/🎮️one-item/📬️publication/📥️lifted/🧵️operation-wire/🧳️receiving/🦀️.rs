//! 🧳️ Installed native operation codecs erase concrete receiver layout while preserving original typed sources.
use super::ArtifactOperationWire;
use semio_framework_value::{ArtifactOwnedValueRetirementFactory,DslValue,OriginalRetirementPoll,RetirementDemand,ValueError,ValueRefusalKind,RetainedAllocationEffect,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedOwnedProjection,RetainedWorkOutcome,RetainedWorkPoll};
use semio_framework_value_native_owned::native_decoding::{NativeDecodeControl,NativeRetirementDecodeControl,NativeReceive,NativeReceivingFailure,OriginalReceivingPrimary};
use semio_framework_pack_json::{OriginalJsonReceiving,ArtifactCanonicalJsonTree};
use semio_framework_dsl_record::native_encoding::FieldProjectionSource;
use std::{mem::size_of,sync::Arc};
#[path="pack-value.rs"]
mod pack_value;
/// 📬️ The concrete vocabulary keeps its native parser, typed receiver and issuer until explicit closure.
pub trait ArtifactOriginalOperationReceiving<M>:Send{
 fn demands(&self,body:usize)->Result<RetirementDemand,ValueError>;
 fn advance(&mut self,control:&mut NativeDecodeControl<'_>,body:usize,maximum_work_units:u64,grant:RetainedCloneGrant)->RetainedWorkPoll<RetainedCloneStep,NativeReceivingFailure>;
 fn ready(&self)->bool;
 /// 📤️ Indicates when the decoder can transfer the mutation to an owning caller.
 fn ready_for_owned(&self)->bool{self.ready()}
 fn owned_handoff_pending(&self)->bool;
 fn take(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome<Option<(RetainedOwnedProjection<M>,RetainedCloneProgress)>> where M:Sync;
 fn take_owned_demands(&self,body:usize)->Result<RetirementDemand,ValueError>;
 fn take_owned_work_demand(&self,body:usize,ceiling:u64)->Result<u64,ValueError>;
 fn take_owned_into(&mut self,output:&mut Option<M>,control:&mut NativeDecodeControl<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome;
 fn begin_close(&mut self);
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>;
 fn close_step(&mut self,control:&mut dyn semio_framework_value::OriginalRetirementControl,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll;
 fn terminal_is_empty(&self)->bool;
 fn allocation_bytes(&self)->usize;
}
pub struct OriginalOperationReceivingAdmission<M>{pub batch_retirement:crate::lifted_member_owned_batch::BatchRetirementIssuer<M>,pub demands:fn()->RetirementDemand,pub admit:fn(&mut Option<RetainedOwnedProjection<Vec<u8>>>,&mut Option<Box<dyn ArtifactOriginalOperationReceiving<M>>>,&Arc<dyn ArtifactOwnedValueRetirementFactory<M>>,&mut NativeDecodeControl<'_>,RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>,pack_value:Option<for<'a> fn(&'a M)->&'a dyn FieldProjectionSource>}
impl<M> Copy for OriginalOperationReceivingAdmission<M>{}
impl<M> Clone for OriginalOperationReceivingAdmission<M>{fn clone(&self)->Self{*self}}
impl<M:NativeReceive<DslValue>+ArtifactCanonicalJsonTree+Send+Sync+'static> ArtifactOriginalOperationReceiving<M> for OriginalJsonReceiving<M>{
 fn demands(&self,body:usize)->Result<RetirementDemand,ValueError>{self.demands(body)}
 fn advance(&mut self,control:&mut NativeDecodeControl<'_>,body:usize,maximum_work_units:u64,grant:RetainedCloneGrant)->RetainedWorkPoll<RetainedCloneStep,NativeReceivingFailure>{let witness=match control.source_witness(){Ok(witness)=>witness,Err(error)=>return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work:0})};self.advance(control,witness,body,maximum_work_units,grant)}
 fn ready(&self)->bool{self.ready()}
 fn owned_handoff_pending(&self)->bool{self.owned_handoff_pending()}
 fn take(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome<Option<(RetainedOwnedProjection<M>,RetainedCloneProgress)>>{self.take(grant)}
 fn take_owned_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{self.take_owned_demands(body)}
 fn take_owned_work_demand(&self,body:usize,ceiling:u64)->Result<u64,ValueError>{self.take_owned_work_demand(body,ceiling)}
 fn take_owned_into(&mut self,output:&mut Option<M>,control:&mut NativeDecodeControl<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome{self.take_owned_into(output,control,body,effect)}
 fn begin_close(&mut self){self.begin_close()}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{self.close_demands(body)}
 fn close_step(&mut self,control:&mut dyn semio_framework_value::OriginalRetirementControl,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{self.close_step(control,effect)}
 fn terminal_is_empty(&self)->bool{self.terminal_is_empty()}
 fn allocation_bytes(&self)->usize{size_of::<Self>()}
}
fn demands<M:NativeReceive<DslValue>+Sync>()->RetirementDemand{let d=OriginalJsonReceiving::<M>::constructor_demand();RetirementDemand{copy_bytes:d.copy_bytes+size_of::<Box<dyn ArtifactOriginalOperationReceiving<M>>>(),capacity_bytes:size_of::<OriginalJsonReceiving<M>>(),..d}}
fn admit<M:NativeReceive<DslValue>+ArtifactCanonicalJsonTree+Send+Sync+'static>(wire:&mut Option<RetainedOwnedProjection<Vec<u8>>>,output:&mut Option<Box<dyn ArtifactOriginalOperationReceiving<M>>>,factory:&Arc<dyn ArtifactOwnedValueRetirementFactory<M>>,control:&mut NativeDecodeControl<'_>,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
 if output.is_some()||wire.is_none(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original operation receiver requires its retained wire and empty output"))}let d=demands::<M>();if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Ok(Default::default())}control.admit_turn_capacity(grant.maximum_capacity_bytes)?;control.charge(d.capacity_bytes)?;
 match OriginalJsonReceiving::admit(wire.take().unwrap(),factory,b"op",|original:&M|original,control,grant){Ok((receiver,_))=>{*output=Some(Box::new(receiver));Ok(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,..Default::default()})},Err((error,original))=>{*wire=Some(original);Err(error)}}
}
impl<M:NativeReceive<DslValue>+ArtifactCanonicalJsonTree+Send+Sync+'static> ArtifactOperationWire<M>{
 /// 🧬️ Registration binds one actual typed receiving implementation to this wire vocabulary.
 pub fn with_original_receiving(mut self)->Self{self.original_receiving=Some(OriginalOperationReceivingAdmission{batch_retirement:crate::lifted_member_owned_batch::BatchRetirementIssuer::typed(),demands:demands::<M>,admit:admit::<M>,pack_value:None});self}
}

#[path="🔢️binary/🦀️.rs"]
pub mod binary;
pub use binary::OriginalBinaryReceiving;
impl<M:protocol::OpBinaryReceiving+Send+Sync+'static> ArtifactOperationWire<M> where M::Receiving:Send{
 /// 🔢️ Registration binds the vocabulary's own binary receiving cursor for persisted operations; the installer names the encoding, the receiver never sniffs bytes.
 pub fn with_original_binary_receiving(mut self)->Self{self.original_receiving=Some(OriginalOperationReceivingAdmission::binary(crate::lifted_member_owned_batch::BatchRetirementIssuer::typed()));self}
}
#[repr(transparent)]
struct OrdinalVariant<M>(M);
impl<M:semio_framework_dsl_record::BorrowedDslVariants> FieldProjectionSource for OrdinalVariant<M>{
 fn projection_view(&self,path:&[usize])->Result<semio_framework_dsl_record::native_encoding::FieldProjectionView<'_>,ValueError>{self.0.projected_borrowed_variant_view(path)}
 fn projection_key(&self,path:&[usize],index:usize)->Result<&str,ValueError>{self.0.projected_borrowed_variant_key(path,index)}
}
fn ordinal_pack_source<M:semio_framework_dsl_record::BorrowedDslVariants>(mutation:&M)->Option<super::ArtifactPreparedOperationSource<'_>>{
 let(_,ordinal,spec)=mutation.projected_borrowed_variant_identity();
 let body=unsafe{&*(std::ptr::from_ref(mutation).cast::<OrdinalVariant<M>>())};
 Some(super::ArtifactPreparedOperationSource::Pack{tag:ordinal as u64,body,spec})
}
impl<M:protocol::OpBinaryReceiving+semio_framework_dsl_record::BorrowedDslVariants+Send+Sync+'static> ArtifactOperationWire<M> where M::Receiving:Send{
 /// 🔢️ Registers a vocabulary whose persisted operations are `format | ordinal | record body`: the same borrowed variant is the original encoding source, so replacement and edit comparison re-encode the persisted bytes exactly.
 pub fn with_original_ordinal_binary_receiving(mut self)->Self{self.source=ordinal_pack_source::<M>;self.with_original_binary_receiving()}
}
#[repr(transparent)]
struct EncodedOperation<M>(M);
impl<M:protocol::OpBinary+Sync> super::ArtifactOperationEncoded for EncodedOperation<M>{
 fn encode_once(&self)->Result<Vec<u8>,ValueError>{self.0.encode_op().map_err(|_|ValueError::literal(ValueRefusalKind::InvalidValue,"encoded operation refused its authored binary codec"))}
}
/// 🔢️ Borrows the authored `OpBinary::encode_op` bytes of one operation as its original encoding source.
pub fn encoded_operation_source<M:protocol::OpBinary+Sync>(mutation:&M)->Option<super::ArtifactPreparedOperationSource<'_>>{
 let body=unsafe{&*(std::ptr::from_ref(mutation).cast::<EncodedOperation<M>>())};
 Some(super::ArtifactPreparedOperationSource::Encoded{body})
}
impl<M:protocol::OpBinaryReceiving+Send+Sync+'static> ArtifactOperationWire<M> where M::Receiving:Send{
 /// 🔢️ Registers a vocabulary whose persisted operations are exactly its authored `OpBinary::encode_op` bytes: that same codec is the original encoding source (encoded once by the cursor, which pays and releases the frame's capacity and copies sixty-four bytes per turn), so replacement and edit comparison re-encode the persisted bytes exactly; the quoted extent is the authored length itself.
 pub fn with_original_encoded_binary_receiving(mut self)->Self{self.source=encoded_operation_source::<M>;self.with_original_binary_receiving()}
}
impl<M:protocol::OpBinaryReceiving+ArtifactCanonicalJsonTree+Send+Sync+'static> ArtifactOperationWire<M> where M::Receiving:Send{
 /// 🎒️ Pairs a vocabulary's full Pack Value OpBinary decoder with its original typed comparison projection.
 pub fn with_original_pack_value_receiving(self)->Self{
  let mut wire=self.with_original_binary_receiving();
  wire.original_receiving.as_mut().unwrap().pack_value=Some(pack_value::source::<M>);
  wire
 }
}
impl<M> ArtifactOperationWire<M>{
 /// 🔍️ Borrows the registered Pack Value comparison projection without changing the Store publication codec.
 pub fn original_pack_value<'a>(&self,mutation:&'a M)->Option<&'a dyn FieldProjectionSource>{self.original_receiving.as_ref()?.pack_value.map(|project|project(mutation))}
}
/// 🧬️ A mutation vocabulary declares how its persisted operations are encoded and installs the matching original receiving on its operation wire; the Store installs it once for every vocabulary, so persisted history with supersessions replays.
pub trait ArtifactPersistedOperation:Sized+Send+Sync+'static{
 fn install_persisted_receiving(wire:ArtifactOperationWire<Self>)->ArtifactOperationWire<Self>;
}
/// 🔢️ Declares a vocabulary whose persisted operations are the binary encoding its own `OpBinaryReceiving` cursor receives.
#[macro_export]
macro_rules! artifact_persisted_binary_operation{
 ($operation:ty)=>{impl $crate::lifted_prepared_operation_wire::ArtifactPersistedOperation for $operation{
  fn install_persisted_receiving(wire:$crate::lifted_prepared_operation_wire::ArtifactOperationWire<Self>)->$crate::lifted_prepared_operation_wire::ArtifactOperationWire<Self>{wire.with_original_binary_receiving()}
 }};
}
/// 🔢️ Declares a vocabulary whose persisted operations are exactly its authored `OpBinary::encode_op` bytes, which are also the original encoding source of replacement and edit comparison.
#[macro_export]
macro_rules! artifact_persisted_encoded_binary_operation{
 ($operation:ty)=>{impl $crate::lifted_prepared_operation_wire::ArtifactPersistedOperation for $operation{
  fn install_persisted_receiving(wire:$crate::lifted_prepared_operation_wire::ArtifactOperationWire<Self>)->$crate::lifted_prepared_operation_wire::ArtifactOperationWire<Self>{wire.with_original_encoded_binary_receiving()}
 }};
}
