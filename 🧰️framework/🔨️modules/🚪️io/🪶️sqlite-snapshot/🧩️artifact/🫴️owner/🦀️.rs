//! 🫴️ SQL fields borrow the initiating original snapshot direction and its receipts.
use semio_framework_io_control::{NativeSnapshotEncodeOwner,NativeSnapshotDecodeOwner,NativeSnapshotEffectProgress,NativeSnapshotFailure};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetainedWorkOutcome,ValueError};
/// 🧭️ A borrowed original owner preserves the active native direction across each SQL field.
pub enum SnapshotOwner<'a,'owner,'control>{Decode(&'a mut NativeSnapshotDecodeOwner<'owner,'control>),Encode(&'a mut NativeSnapshotEncodeOwner<'owner,'control>)}
impl SnapshotOwner<'_,'_,'_>{
 /// 📐️ Reports the original native allocation ledger without resetting its scope.
 pub fn owned_bytes(&mut self)->usize{match self{Self::Decode(owner)=>owner.native().owned_bytes(),Self::Encode(owner)=>owner.native().owned_bytes()}}
 /// 💻️ Reports cumulative original concrete actions independently of physical receipts.
 pub fn actual_work(&mut self)->u64{match self{Self::Decode(owner)=>owner.native().actual_work(),Self::Encode(owner)=>owner.native().actual_work()}}
 /// 🛑️ Observes the initiating direction before the next expensive effect.
 pub fn checkpoint(&mut self)->Result<(),ValueError>{match self{Self::Decode(owner)=>owner.native().checkpoint(),Self::Encode(owner)=>owner.native().checkpoint()}}
 /// 📏️ Reserves the same original native capacity before allocation.
 pub fn charge(&mut self,bytes:usize)->Result<(),ValueError>{match self{Self::Decode(owner)=>owner.native().charge(bytes),Self::Encode(owner)=>owner.native().charge(bytes)}}
 /// 🎟️ Runs one quoted native effect while the caller retains every progress and failure field.
 pub fn effect(&mut self,demand:RetainedCloneGrant,marker:&mut NativeSnapshotEffectProgress,unwind:&mut Option<NativeSnapshotFailure>,effect:&mut impl FnMut(&mut bool,&mut RetainedCloneProgress)->Result<(),ValueError>)->RetainedWorkOutcome<(),NativeSnapshotFailure>{match self{Self::Decode(owner)=>owner.effect(demand,marker,unwind,effect),Self::Encode(owner)=>owner.effect(demand,marker,unwind,effect)}}
 /// 🏗️ Charges native capacity only after the original work and physical quote admit the effect.
 pub fn effect_allocated(&mut self,bytes:usize,demand:RetainedCloneGrant,marker:&mut NativeSnapshotEffectProgress,unwind:&mut Option<NativeSnapshotFailure>,allocation:&mut semio_framework_io_control::NativeSnapshotAllocation,effect:&mut impl FnMut(&mut bool,&mut RetainedCloneProgress,&mut Option<usize>)->Result<(),ValueError>)->RetainedWorkOutcome<(),NativeSnapshotFailure>{
  match self{
   Self::Decode(owner)=>match allocation.decode_mut(){Ok(allocation)=>owner.effect_allocated(bytes,demand,marker,unwind,allocation,effect),Err(error)=>RetainedWorkOutcome{work:0,result:Err(error.into())}},
   Self::Encode(owner)=>match allocation.encode_mut(){Ok(allocation)=>owner.effect_allocated(bytes,demand,marker,unwind,allocation,effect),Err(error)=>RetainedWorkOutcome{work:0,result:Err(error.into())}},
  }
 }
 /// 📦️ Copies into the caller's original octet slot under the same work and physical recipients.
 pub fn copy_blob(&mut self,bytes:&[u8],output:&mut Vec<u8>,marker:&mut NativeSnapshotEffectProgress,unwind:&mut Option<NativeSnapshotFailure>,allocation:&mut semio_framework_io_control::NativeSnapshotAllocation,depth:usize,progress:&mut dyn FnMut(usize,usize)->bool)->RetainedWorkOutcome<(),NativeSnapshotFailure>{
  match self{
   Self::Decode(owner)=>{let allocation=match allocation.decode_mut(){Ok(value)=>value,Err(error)=>return RetainedWorkOutcome{work:0,result:Err(error.into())}};let maximum=owner.native().maximum_bytes();let mut observe=|event:semio_framework_value_native_owned::native_decoding::NativeDecodeProgress|progress(event.completed,event.total);owner.scoped_native(maximum,&mut observe,|owner|owner.copy_bytes_into(bytes,output,marker,unwind,allocation,depth))},
   Self::Encode(owner)=>{let allocation=match allocation.encode_mut(){Ok(value)=>value,Err(error)=>return RetainedWorkOutcome{work:0,result:Err(error.into())}};let maximum=owner.native().maximum_bytes();let mut observe=|event:semio_framework_value::native_encoding::NativeEncodeProgress|progress(event.completed,event.total);owner.scoped_native(maximum,&mut observe,|owner|owner.copy_bytes_into(bytes,output,marker,unwind,allocation,depth))},
  }
 }
}
