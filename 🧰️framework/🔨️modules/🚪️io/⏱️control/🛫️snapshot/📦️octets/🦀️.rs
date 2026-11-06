//! 📦️ Encoder octet copying keeps the initiating owner and original wallet.
use super::*;
impl NativeSnapshotEncodeOwner<'_,'_>{
 /// 🏗️ Admits capacity inside a quoted original effect before invoking its caller-retained allocator.
 pub fn effect_allocated(&mut self,bytes:usize,demand:RetainedCloneGrant,marker:&mut crate::NativeSnapshotEffectProgress,unwind:&mut Option<NativeSnapshotFailure>,allocation:&mut crate::NativeSnapshotEncodeAllocation,effect:&mut impl FnMut(&mut bool,&mut RetainedCloneProgress,&mut Option<usize>)->Result<(),ValueError>)->semio_framework_value::RetainedWorkOutcome<(),NativeSnapshotFailure>{
  let mut scope=super::super::SnapshotBodyScope{wallet:NativeSnapshotBodyWallet{grant:self.grant,progress:self.progress},progress:&mut self.progress};scope.wallet.encode_effect(self.native,demand,Some(bytes),marker,unwind,allocation,effect)
 }

 /// 🧊️ Adopts original octet backing before posting any fallible effect.
 pub fn copy_bytes_into(&mut self,bytes:&[u8],output:&mut Vec<u8>,marker:&mut crate::NativeSnapshotEffectProgress,unwind:&mut Option<NativeSnapshotFailure>,allocation:&mut crate::NativeSnapshotEncodeAllocation,depth:usize)->RetainedWorkOutcome<(),NativeSnapshotFailure>{
  let mut scope=super::super::SnapshotBodyScope{wallet:NativeSnapshotBodyWallet{grant:self.grant,progress:self.progress},progress:&mut self.progress};scope.wallet.copy_bytes_encoded(self.native,bytes,output,marker,unwind,allocation,depth)
 }
}
