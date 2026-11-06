//! 📦️ Original native octet receivers keep exact backing and every copied prefix.
use super::{NativeEncodeControl,NativeDecodeControl,NativeSnapshotBodyWallet,NativeSnapshotEffectProgress,NativeSnapshotFailure,NativeSnapshotDecodeEffectQuote,RetainedCloneGrant,ValueError,ValueRefusalKind};
use semio_framework_value::RetainedCloneProgress;
use std::mem::size_of;
macro_rules! copy_octets{
 ($method:ident,$native:ident,$quote:ident,$quote_method:ident,$allocation:ident)=>{
  impl NativeSnapshotBodyWallet{
   /// 🧊️ Adopts exact original octet backing before any fallible work or physical post.
   pub fn $method(&mut self,native:&mut $native<'_>,bytes:&[u8],output:&mut Vec<u8>,marker:&mut NativeSnapshotEffectProgress,unwind:&mut Option<NativeSnapshotFailure>,allocation:&mut super::$allocation,depth:usize)->semio_framework_value::RetainedWorkOutcome<(),NativeSnapshotFailure>{
    let mut work=0;let result=(||{
     if allocation.actual.is_some()||allocation.expected.is_some()||allocation.unwind.is_some()||allocation.pending.is_some()||allocation.collision.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"snapshot octet allocation receipt is unsettled").into())}
     if !output.is_empty()||output.capacity()!=0{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"snapshot octet receiving slot must be empty").into())}
     native.scoped_stage(|native|{
      native.begin_stage(bytes.len())?;if bytes.is_empty(){return Ok(())}
      let layout=std::alloc::Layout::array::<u8>(bytes.len()).map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"snapshot octet extent overflow"))?;
      let quote=$quote::$quote_method(self,native,RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:size_of::<Vec<u8>>(),maximum_capacity_bytes:bytes.len(),maximum_depth:depth,..Default::default()})?;
      let turn=quote.run_native(marker,unwind,&mut |native,performed,physical|{
       let outcome=native.allocate_effect(bytes.len(),&mut allocation.actual,&mut allocation.unwind,&mut |actual|{
        let pointer=unsafe{std::alloc::alloc(layout)};if pointer.is_null(){return Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"snapshot octet allocation failed"))}
        *output=unsafe{Vec::from_raw_parts(pointer,0,bytes.len())};*actual=Some(bytes.len());*performed=true;*physical=RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Vec<u8>>(),retained_capacity_bytes:bytes.len(),..Default::default()};Ok(())
       });
       if let Err(outcome)=allocation.capture(outcome,bytes.len()){allocation.collision=Some(outcome);return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"snapshot octet allocation custody is occupied"))}Ok(())
      });work+=turn.work;allocation.finish(turn.result).map_err(|error|error.with_retained_progress(self.progress()))?;marker.reset_completed()?;
      for chunk in bytes.chunks(65536){
       let quote=$quote::$quote_method(self,native,RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:chunk.len(),maximum_depth:depth,..Default::default()})?;
       let turn=quote.run(marker,unwind,&mut |performed,physical|{
        output.extend_from_slice(chunk);*performed=true;*physical=RetainedCloneProgress{copied_items:1,copied_bytes:chunk.len(),..Default::default()};Ok(())
       });work+=turn.work;turn.result?;marker.reset_completed()?;native.advance(chunk.len())?;
      }Ok(())
     })
    })();semio_framework_value::RetainedWorkOutcome{result,work}
   }
  }
 };
}
impl NativeSnapshotBodyWallet{
 /// 🎟️ Runs one original encoder effect through the shared lower transaction.
 pub(super) fn encode_effect(&mut self,native:&mut NativeEncodeControl<'_>,demand:RetainedCloneGrant,bytes:Option<usize>,marker:&mut NativeSnapshotEffectProgress,unwind:&mut Option<NativeSnapshotFailure>,allocation:&mut super::NativeSnapshotEncodeAllocation,effect:&mut impl FnMut(&mut bool,&mut RetainedCloneProgress,&mut Option<usize>)->Result<(),ValueError>)->semio_framework_value::RetainedWorkOutcome<(),NativeSnapshotFailure>{
  use semio_framework_value::RetainedWorkOutcome;
  if allocation.is_unsettled()||unwind.is_some(){return RetainedWorkOutcome{work:0,result:Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"snapshot encoder effect requires empty retained receipt").into())}}
  let bytes=bytes.filter(|bytes|*bytes!=0);let mut view=semio_framework_value::native_encoding::NativeEncodeEffectProgress{performed:&mut marker.performed,physical:&mut marker.physical,actual:&mut allocation.actual,allocation:&mut allocation.unwind,outcome:&mut allocation.outcome,settlement:&mut marker.settlement};
  let turn=native.effect_allocated(self.grant,&mut self.progress,demand,bytes,&mut view,&mut allocation.effect_unwind,effect);let progress=self.progress;
  if turn.result.is_ok(){allocation.actual=None}else if allocation.actual.is_some()||allocation.unwind.is_some(){allocation.expected=bytes}
  RetainedWorkOutcome{result:turn.result.map_err(|error|NativeSnapshotFailure::from_encode_effect(error,progress)),work:turn.work}
 }
 /// 📦️ Copies octets through the same original encoder and retained allocation carrier.
 pub fn copy_bytes_encoded(&mut self,native:&mut NativeEncodeControl<'_>,bytes:&[u8],output:&mut Vec<u8>,marker:&mut NativeSnapshotEffectProgress,unwind:&mut Option<NativeSnapshotFailure>,allocation:&mut super::NativeSnapshotEncodeAllocation,depth:usize)->semio_framework_value::RetainedWorkOutcome<(),NativeSnapshotFailure>{
  super::native_snapshot_turn(|work|{if allocation.is_unsettled()||unwind.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"snapshot octet allocation receipt is unsettled").into())}if !output.is_empty()||output.capacity()!=0{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"snapshot octet receiving slot must be empty").into())}native.scoped_stage(|native|{native.begin_stage(bytes.len())?;if bytes.is_empty(){return Ok(())}let layout=std::alloc::Layout::array::<u8>(bytes.len()).map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"snapshot octet extent overflow"))?;let demand=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:size_of::<Vec<u8>>(),maximum_capacity_bytes:bytes.len(),maximum_depth:depth,..Default::default()};work.take(self.encode_effect(native,demand,Some(bytes.len()),marker,unwind,allocation,&mut |performed,physical,actual|{*performed=true;let pointer=unsafe{std::alloc::alloc(layout)};if pointer.is_null(){return Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"snapshot octet allocation failed"))}*output=unsafe{Vec::from_raw_parts(pointer,0,bytes.len())};*actual=Some(bytes.len());*physical=RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Vec<u8>>(),retained_capacity_bytes:bytes.len(),..Default::default()};Ok(())}))?;marker.reset_completed()?;for chunk in bytes.chunks(65536){let demand=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:chunk.len(),maximum_depth:depth,..Default::default()};work.take(self.encode_effect(native,demand,None,marker,unwind,allocation,&mut |performed,physical,_|{*performed=true;output.extend_from_slice(chunk);*physical=RetainedCloneProgress{copied_items:1,copied_bytes:chunk.len(),..Default::default()};Ok(())}))?;marker.reset_completed()?;native.advance(chunk.len())?}Ok(())})})
 }
}
copy_octets!(copy_bytes_decoded,NativeDecodeControl,NativeSnapshotDecodeEffectQuote,decode,NativeSnapshotDecodeAllocation);
impl super::NativeSnapshotDecodeOwner<'_,'_>{
 /// 🏗️ Admits capacity inside a quoted original effect before invoking its caller-retained allocator.
 pub fn effect_allocated(&mut self,bytes:usize,demand:RetainedCloneGrant,marker:&mut crate::NativeSnapshotEffectProgress,unwind:&mut Option<NativeSnapshotFailure>,allocation:&mut super::NativeSnapshotDecodeAllocation,effect:&mut impl FnMut(&mut bool,&mut RetainedCloneProgress,&mut Option<usize>)->Result<(),ValueError>)->semio_framework_value::RetainedWorkOutcome<(),NativeSnapshotFailure>{
  use semio_framework_value::RetainedWorkOutcome;
  if bytes>demand.maximum_capacity_bytes||allocation.actual.is_some()||allocation.expected.is_some()||allocation.unwind.is_some()||allocation.pending.is_some()||allocation.collision.is_some(){return RetainedWorkOutcome{work:0,result:Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"snapshot allocation requires its admitted extent and empty original receipt").into())}}
  let mut scope=super::SnapshotBodyScope{wallet:NativeSnapshotBodyWallet{grant:self.grant,progress:self.progress},progress:&mut self.progress};
  let quote=match NativeSnapshotDecodeEffectQuote::decode(&mut scope.wallet,self.native,demand){Ok(quote)=>quote,Err(error)=>return RetainedWorkOutcome{work:0,result:Err(error)}};
  let turn=quote.run_native(marker,unwind,&mut |native,performed,physical|{
   let outcome=native.allocate_effect(bytes,&mut allocation.actual,&mut allocation.unwind,&mut |actual|effect(performed,physical,actual));
   if let Err(outcome)=allocation.capture(outcome,bytes){allocation.collision=Some(outcome);return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"snapshot allocation custody is occupied"))}Ok(())
  });RetainedWorkOutcome{result:allocation.finish(turn.result).map_err(|error|error.with_retained_progress(scope.wallet.progress())),work:turn.work}
 }

 /// 📦️ Copies octets through this same original owner and keeps refused prefixes in the caller slot.
 pub fn copy_bytes_into(&mut self,bytes:&[u8],output:&mut Vec<u8>,marker:&mut NativeSnapshotEffectProgress,unwind:&mut Option<NativeSnapshotFailure>,allocation:&mut super::NativeSnapshotDecodeAllocation,depth:usize)->semio_framework_value::RetainedWorkOutcome<(),NativeSnapshotFailure>{
  let mut scope=super::SnapshotBodyScope{wallet:NativeSnapshotBodyWallet{grant:self.grant,progress:self.progress},progress:&mut self.progress};scope.wallet.copy_bytes_decoded(self.native,bytes,output,marker,unwind,allocation,depth)
 }
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
