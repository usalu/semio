//! ⏱️ Only the active original native direction observes a physical IO phase.
use semio_framework_value::{NativeDecodeControl,NativeEncodeControl,RetainedCloneGrant,retained_clone::RetainedCloneProgress,ValueError,ValueRefusalKind};
#[path="🛫️snapshot/🦀️.rs"]mod snapshot;
pub use snapshot::NativeSnapshotEncodeOwner;
/// 🧾️ Debits actual producer spans from one admitted receiving frame's original authority.
pub struct NativeSnapshotBodyWallet{grant:RetainedCloneGrant,progress:RetainedCloneProgress}
impl crate::sqlite_snapshot::artifact::receiving::ReceivingWallet for NativeSnapshotBodyWallet{
 fn admit_frontier(&self,demand:RetainedCloneGrant)->Result<(),ValueError>{NativeSnapshotBodyWallet::admit_frontier(self,demand)}
 fn record_progress(&mut self,progress:RetainedCloneProgress)->Result<(),ValueError>{NativeSnapshotBodyWallet::record_progress(self,progress)}
 fn allocate_vec_into<T>(&mut self,native:&mut NativeDecodeControl<'_>,count:usize,output:&mut Vec<T>,depth:usize)->Result<(),ValueError>{NativeSnapshotBodyWallet::allocate_vec_into(self,native,count,output,depth)}
 fn allocate_encode_vec_into<T>(&mut self,native:&mut NativeEncodeControl<'_>,count:usize,output:&mut Vec<T>,depth:usize)->Result<(),ValueError>{NativeSnapshotBodyWallet::allocate_encode_vec_into(self,native,count,output,depth)}
 fn copy_text_into(&mut self,native:&mut NativeDecodeControl<'_>,text:&str,output:&mut String,depth:usize)->Result<(),ValueError>{NativeSnapshotBodyWallet::copy_text_into(self,native,text,output,depth)}
 fn copy_encode_text_into(&mut self,native:&mut NativeEncodeControl<'_>,text:&str,output:&mut String,depth:usize)->Result<(),ValueError>{NativeSnapshotBodyWallet::copy_encode_text_into(self,native,text,output,depth)}
}
impl NativeSnapshotBodyWallet{
 /// 🎟️ Binds explicitly supplied original producer authority without deriving it from storage demands.
 pub fn new(grant:RetainedCloneGrant)->Self{Self{grant,progress:Default::default()}}
 /// 🧮️ Returns only currencies unspent by earlier accepted producer spans.
 pub fn remaining_grant(&self)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:self.grant.maximum_items.saturating_sub(self.progress.copied_items),maximum_copy_bytes:self.grant.maximum_copy_bytes.saturating_sub(self.progress.copied_bytes),maximum_capacity_bytes:self.grant.maximum_capacity_bytes.saturating_sub(self.progress.retained_capacity_bytes),maximum_release_bytes:self.grant.maximum_release_bytes.saturating_sub(self.progress.released_bytes),maximum_depth:self.grant.maximum_depth}}
 /// 🛂️ Checks all independently authored currencies before the producer can create an owned frontier.
 pub fn admit_frontier(&self,demand:RetainedCloneGrant)->Result<(),ValueError>{let remaining=self.remaining_grant();if demand.maximum_items>remaining.maximum_items{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"snapshot producer exceeds remaining work grant"))}if demand.maximum_depth>remaining.maximum_depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"snapshot producer exceeds original parent depth"))}if demand.maximum_copy_bytes>remaining.maximum_copy_bytes||demand.maximum_capacity_bytes>remaining.maximum_capacity_bytes||demand.maximum_release_bytes>remaining.maximum_release_bytes{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"snapshot producer exceeds remaining physical grant"))}Ok(())}
 /// 🔐️ Records actual performed effects before refusing exceeded original authority.
 pub fn record_progress(&mut self,progress:RetainedCloneProgress)->Result<(),ValueError>{let admitted=self.progress.fits(self.grant)&&progress.fits(self.remaining_grant());self.progress=self.progress.checked_add(progress).map_err(|error|error.with_retained_progress(progress))?;if !admitted{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"snapshot producer receipt exceeds original grant").with_retained_progress(progress))}Ok(())}
 /// 📊️ Reports accepted actual producer receipts independently of native allocation counters.
 pub fn progress(&self)->RetainedCloneProgress{self.progress}
 /// 🛫️ Loans the original encoder while settling accepted spans into this same wallet.
 pub fn snapshot_encode<'owner,'control>(&'owner mut self,native:&'owner mut NativeEncodeControl<'control>)->NativeSnapshotEncodeOwner<'owner,'control>{NativeSnapshotEncodeOwner::borrowed(native,self.grant,&mut self.progress)}
 /// 🛬️ Loans the original decoder while preserving earlier accepted wallet receipts.
 pub fn snapshot_decode<'owner,'control>(&'owner mut self,native:&'owner mut NativeDecodeControl<'control>)->NativeSnapshotDecodeOwner<'owner,'control>{NativeSnapshotDecodeOwner::borrowed(native,self.grant,&mut self.progress)}
 /// 🧵️ Preserves an original UTF8 destination and settles every copied prefix before propagating refusal.
 pub fn copy_text_into(&mut self,native:&mut NativeDecodeControl<'_>,text:&str,output:&mut String,depth:usize)->Result<(),ValueError>{if !output.is_empty()||output.capacity()!=0{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"snapshot text receiving slot must be empty"))}self.admit_frontier(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:text.len(),maximum_capacity_bytes:text.len(),maximum_release_bytes:0,maximum_depth:depth})?;let result=native.copy_text_into(text,output);let performed=RetainedCloneProgress{copied_items:usize::from(result.is_ok()||output.capacity()>0),copied_bytes:output.len(),retained_capacity_bytes:output.capacity(),released_bytes:0};self.record_progress(performed)?;result.map_err(|error|error.with_retained_progress(performed))}
 /// 📦️ Keeps original intrinsic octet backing and settles accepted prefixes before propagating cancellation.
 pub fn copy_bytes_into(&mut self,native:&mut NativeDecodeControl<'_>,bytes:&[u8],output:&mut Vec<u8>,depth:usize)->Result<(),ValueError>{if !output.is_empty()||output.capacity()!=0{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"snapshot octet receiving slot must be empty"))}self.admit_frontier(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:bytes.len(),maximum_capacity_bytes:bytes.len(),maximum_release_bytes:0,maximum_depth:depth})?;let result=native.copy_bytes_into(bytes,output);let performed=RetainedCloneProgress{copied_items:usize::from(result.is_ok()||output.capacity()>0),copied_bytes:output.len(),retained_capacity_bytes:output.capacity(),released_bytes:0};self.record_progress(performed)?;result.map_err(|error|error.with_retained_progress(performed))}
 /// 🪑️ Admits and moves actual typed vector backing into its original receiving slot.
 pub fn allocate_vec_into<T>(&mut self,native:&mut NativeDecodeControl<'_>,count:usize,output:&mut Vec<T>,depth:usize)->Result<(),ValueError>{if !output.is_empty()||output.capacity()!=0{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"snapshot vector receiving slot must be empty"))}let bytes=count.checked_mul(std::mem::size_of::<T>()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"snapshot producer vector extent overflow"))?;self.admit_frontier(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:std::mem::size_of::<Vec<T>>(),maximum_capacity_bytes:bytes,maximum_release_bytes:0,maximum_depth:depth})?;*output=native.allocate_vec(count)?;self.record_progress(RetainedCloneProgress{copied_items:1,copied_bytes:std::mem::size_of::<Vec<T>>(),retained_capacity_bytes:output.capacity()*std::mem::size_of::<T>(),released_bytes:0})}
 /// 🛫️ Keeps an import reconstruction's actual UTF8 prefix under its original encoder custody.
 pub fn copy_encode_text_into(&mut self,native:&mut NativeEncodeControl<'_>,text:&str,output:&mut String,depth:usize)->Result<(),ValueError>{
  if !output.is_empty()||output.capacity()!=0{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"snapshot encoding text receiving slot must be empty"))}
  self.admit_frontier(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:text.len(),maximum_capacity_bytes:text.len(),maximum_release_bytes:0,maximum_depth:depth})?;
  let result:Result<(),ValueError>=native.scoped_stage(|native|{native.begin_stage(text.len())?;native.charge(text.len())?;output.try_reserve_exact(text.len()).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"snapshot encoding text allocation failed"))?;let mut copied=0;while copied<text.len(){let mut end=copied.saturating_add(65536).min(text.len());while !text.is_char_boundary(end){end-=1;}output.push_str(&text[copied..end]);native.advance(end-copied)?;copied=end;}Ok(())});
  let progress=RetainedCloneProgress{copied_items:usize::from(result.is_ok()||output.capacity()>0),copied_bytes:output.len(),retained_capacity_bytes:output.capacity(),released_bytes:0};self.record_progress(progress)?;result.map_err(|error|error.with_retained_progress(progress))
 }
 /// 🪑️ Admits actual typed reconstruction backing through the original encoder allocation port.
 pub fn allocate_encode_vec_into<T>(&mut self,native:&mut NativeEncodeControl<'_>,count:usize,output:&mut Vec<T>,depth:usize)->Result<(),ValueError>{if !output.is_empty()||output.capacity()!=0{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"snapshot encoding vector receiving slot must be empty"))}let bytes=count.checked_mul(std::mem::size_of::<T>()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"snapshot encoding vector extent overflow"))?;self.admit_frontier(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:std::mem::size_of::<Vec<T>>(),maximum_capacity_bytes:bytes,maximum_release_bytes:0,maximum_depth:depth})?;*output=native.allocate_vec(count)?;self.record_progress(RetainedCloneProgress{copied_items:1,copied_bytes:std::mem::size_of::<Vec<T>>(),retained_capacity_bytes:output.capacity()*std::mem::size_of::<T>(),released_bytes:0})}
}
/// 🛬️ Carries the original decoder and independently supplied physical ownership grant.
pub struct NativeSnapshotDecodeOwner<'owner,'control>{native:&'owner mut NativeDecodeControl<'control>,grant:RetainedCloneGrant,progress:RetainedCloneProgress,wallet:Option<&'owner mut RetainedCloneProgress>,pending:Option<&'owner mut Option<Box<dyn semio_framework_value::ErasedSnapshotRetirement>>>}
impl<'owner,'control> NativeSnapshotDecodeOwner<'owner,'control>{
 fn with_io<T>(&mut self,operation:impl FnOnce(&mut IoRunControl<'_,'control>)->Result<T,ValueError>)->Result<T,ValueError>{let mut run=IoRunControl::borrowed_decoding(self.native,self.grant,&mut self.progress,self.pending.as_deref_mut());operation(&mut run)}
 /// 🫴️ Binds authentic caller authority without reconstructing any grant currency.
 pub fn new(native:&'owner mut NativeDecodeControl<'control>,grant:RetainedCloneGrant)->Self{Self{native,grant,progress:Default::default(),wallet:None,pending:None}}
 /// 🔗️ Carries prior accepted receipts from the same original IO operation.
 fn borrowed(native:&'owner mut NativeDecodeControl<'control>,grant:RetainedCloneGrant,wallet:&'owner mut RetainedCloneProgress)->Self{let progress=*wallet;Self{native,grant,progress,wallet:Some(wallet),pending:None}}
 /// 🧮️ Returns only authority unspent by actual earlier turns.
 pub fn remaining_grant(&self)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:self.grant.maximum_items.saturating_sub(self.progress.copied_items),maximum_copy_bytes:self.grant.maximum_copy_bytes.saturating_sub(self.progress.copied_bytes),maximum_capacity_bytes:self.grant.maximum_capacity_bytes.saturating_sub(self.progress.retained_capacity_bytes),maximum_release_bytes:self.grant.maximum_release_bytes.saturating_sub(self.progress.released_bytes),maximum_depth:self.grant.maximum_depth}}
 /// 🧾️ Returns the original cumulative accepted physical receipts.
 pub fn progress(&self)->RetainedCloneProgress{self.progress}
 /// 🔐️ Keeps the actual performed span in the original wallet before typed refusal.
 pub fn record_progress(&mut self,progress:RetainedCloneProgress)->Result<(),ValueError>{let admitted=self.progress.fits(self.grant)&&progress.fits(self.remaining_grant());self.progress=self.progress.checked_add(progress).map_err(|error|error.with_retained_progress(progress))?;if !admitted{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"snapshot decoding receipt exceeds original grant").with_retained_progress(progress))}Ok(())}
 /// 🎟️ Returns the original five-axis grant unchanged.
 pub fn grant(&self)->RetainedCloneGrant{self.grant}
 /// 🔭️ Reborrows the original callback, allocation port and cumulative decode receipt.
 pub fn native(&mut self)->&mut NativeDecodeControl<'control>{self.native}
 /// 🔗️ Keeps actual parent custody and accepted receipts through original native scopes.
 pub fn scoped_native<O>(&mut self,maximum:usize,observer:&mut dyn FnMut(semio_framework_value::native_decoding::NativeDecodeProgress)->bool,operation:impl FnOnce(&mut NativeSnapshotDecodeOwner<'_,'_>)->Result<O,ValueError>)->Result<O,ValueError>{let grant=self.remaining_grant();let pending=self.pending.as_deref_mut();let mut performed=RetainedCloneProgress::default();let result=self.native.scoped_maximum(maximum,|native|native.scoped_observer(observer,|native|{let mut original=NativeSnapshotDecodeOwner{native,grant,progress:Default::default(),wallet:None,pending};let result=operation(&mut original);performed=original.progress();result}));self.record_progress(performed).map_err(|error|error.with_retained_progress(performed))?;result.map_err(|error|error.with_retained_progress(performed))}

 /// 🪑️ Retains a typed decoder cursor and its accepted output in the original recipient.
 pub fn drive_cursor<T:semio_framework_value::retirement::RetireOwned,O:semio_framework_value::retirement::RetireOwned>(&mut self,cursor:&mut Option<T>,advance:impl FnMut(&mut T,&mut NativeDecodeControl<'_>,RetainedCloneGrant)->(Result<Option<O>,ValueError>,RetainedCloneProgress))->Result<O,ValueError>{
  let remaining=self.remaining_grant();let mut performed=RetainedCloneProgress::default();
  let result=snapshot::drive_cursor(self.native,remaining,&mut performed,self.pending.as_deref_mut(),cursor,advance);
  self.record_progress(performed).map_err(|error|error.with_retained_progress(performed))?;
  result.map_err(|error|error.with_retained_progress(performed))
 }

 /// 🫴️ Returns every partial decoded owner to its original recipient before publication refusal.
 pub fn receive<T:semio_framework_value::retirement::RetireOwned,O:semio_framework_value::retirement::RetireOwned>(&mut self,operation:impl FnOnce(&mut Option<T>,&mut NativeDecodeControl<'_>,&mut NativeSnapshotBodyWallet)->Result<O,ValueError>)->Result<O,ValueError>{let remaining=self.remaining_grant();let mut performed=RetainedCloneProgress::default();let result=snapshot::receive(self.native,remaining,&mut performed,self.pending.as_deref_mut(),|slot,native,body,_pending|operation(slot,native,body));self.record_progress(performed).map_err(|error|error.with_retained_progress(performed))?;result.map_err(|error|error.with_retained_progress(performed))}
 /// 🪆️ Keeps child construction and its exact receipts inside the original admitted parent frame.
 pub fn receive_nested<T:semio_framework_value::retirement::RetireOwned,O:semio_framework_value::retirement::RetireOwned>(&mut self,operation:impl FnOnce(&mut Option<T>,&mut NativeSnapshotDecodeOwner<'_, 'control>)->Result<O,ValueError>)->Result<O,ValueError>{let remaining=self.remaining_grant();let mut performed=RetainedCloneProgress::default();let result=snapshot::receive(self.native,remaining,&mut performed,self.pending.as_deref_mut(),|slot,native,body,pending|{let mut original=NativeSnapshotDecodeOwner{native,grant:body.remaining_grant(),progress:Default::default(),wallet:None,pending:Some(pending)};let result=operation(slot,&mut original);let progress=original.progress();drop(original);body.record_progress(progress)?;result});self.record_progress(performed).map_err(|error|error.with_retained_progress(performed))?;result.map_err(|error|error.with_retained_progress(performed))}

}
impl Drop for NativeSnapshotDecodeOwner<'_, '_>{fn drop(&mut self){if let Some(wallet)=self.wallet.as_deref_mut(){*wallet=self.progress;}}}
/// 🧭️ Names the one active native receiving ledger.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum IoNativeDirection{Decode,Encode}
enum Owner<'a,'b>{Native{decode:&'a mut NativeDecodeControl<'b>,encode:&'a mut NativeEncodeControl<'b>,grant:RetainedCloneGrant},Decode{native:&'a mut NativeDecodeControl<'b>,grant:RetainedCloneGrant},Encode{native:&'a mut NativeEncodeControl<'b>,grant:RetainedCloneGrant}}
/// 🫴️ Borrows original authority; foreign phases bind only their selected native direction.
pub struct IoRunControl<'a,'b>{owner:Owner<'a,'b>,active:IoNativeDirection,progress:RetainedCloneProgress,wallet:Option<&'a mut RetainedCloneProgress>,pending:Option<&'a mut Option<Box<dyn semio_framework_value::ErasedSnapshotRetirement>>>}
impl<'a,'b> IoRunControl<'a,'b>{
 fn borrowed_native(decode:&'a mut NativeDecodeControl<'b>,encode:&'a mut NativeEncodeControl<'b>,grant:RetainedCloneGrant,wallet:&'a mut RetainedCloneProgress,pending:Option<&'a mut Option<Box<dyn semio_framework_value::ErasedSnapshotRetirement>>>,active:IoNativeDirection)->Self{let progress=*wallet;Self{owner:Owner::Native{decode,encode,grant},active,progress,wallet:Some(wallet),pending}}
 fn borrowed_decoding(native:&'a mut NativeDecodeControl<'b>,grant:RetainedCloneGrant,wallet:&'a mut RetainedCloneProgress,pending:Option<&'a mut Option<Box<dyn semio_framework_value::ErasedSnapshotRetirement>>>)->Self{let progress=*wallet;Self{owner:Owner::Decode{native,grant},active:IoNativeDirection::Decode,progress,wallet:Some(wallet),pending}}
 fn borrowed_encoding(native:&'a mut NativeEncodeControl<'b>,grant:RetainedCloneGrant,wallet:&'a mut RetainedCloneProgress,pending:Option<&'a mut Option<Box<dyn semio_framework_value::ErasedSnapshotRetirement>>>)->Self{let progress=*wallet;Self{owner:Owner::Encode{native,grant},active:IoNativeDirection::Encode,progress,wallet:Some(wallet),pending}}
 /// 🪆️ Admits a real child frame while retaining its pending ticket and all prior accepted physical spans.
 pub fn receive_nested<T:semio_framework_value::retirement::RetireOwned,O:semio_framework_value::retirement::RetireOwned>(&mut self,operation:impl FnOnce(&mut Option<T>,&mut IoRunControl<'_,'b>)->Result<O,ValueError>)->Result<O,ValueError>{
  if let Owner::Native{decode,encode,grant}=&mut self.owner{return match self.active{
   IoNativeDirection::Decode=>{let mut original=NativeSnapshotDecodeOwner::borrowed(decode,*grant,&mut self.progress);original.pending=self.pending.as_deref_mut();original.receive_nested(|slot,child|{let mut run=IoRunControl::borrowed_native(child.native,encode,child.grant,&mut child.progress,child.pending.as_deref_mut(),IoNativeDirection::Decode);operation(slot,&mut run)})},
   IoNativeDirection::Encode=>{let mut original=NativeSnapshotEncodeOwner::borrowed_pending(encode,*grant,&mut self.progress,self.pending.as_deref_mut());original.receive_nested(|slot,child|child.with_io_companion(decode,|run|operation(slot,run)))},
  }}
  match self.active{IoNativeDirection::Decode=>self.snapshot_decode()?.receive_nested(|slot,owner|owner.with_io(|run|operation(slot,run))),IoNativeDirection::Encode=>self.snapshot_encode()?.receive_nested(|slot,owner|owner.with_io(|run|operation(slot,run)))}
 }
 /// 🫴️ Preserves both original native caller controls across explicit phase transitions.
 pub fn new(decode:&'a mut NativeDecodeControl<'b>,encode:&'a mut NativeEncodeControl<'b>,grant:RetainedCloneGrant)->Self{Self{owner:Owner::Native{decode,encode,grant},active:IoNativeDirection::Decode,progress:Default::default(),wallet:None,pending:None}}
 /// 🛬️ Binds only the current foreign decode phase without creating an inactive encoder.
 pub fn decoding(decode:&'a mut NativeDecodeControl<'b>,grant:RetainedCloneGrant)->Self{Self{owner:Owner::Decode{native:decode,grant},active:IoNativeDirection::Decode,progress:Default::default(),wallet:None,pending:None}}
 /// 🛫️ Binds only the current foreign encode phase without creating an inactive decoder.
 pub fn encoding(encode:&'a mut NativeEncodeControl<'b>,grant:RetainedCloneGrant)->Self{Self{owner:Owner::Encode{native:encode,grant},active:IoNativeDirection::Encode,progress:Default::default(),wallet:None,pending:None}}
 /// 🧭️ Reports the active original ledger for receiving phase completion.
 pub fn direction(&self)->IoNativeDirection{self.active}
 /// 📊️ Reports accepted original physical spans, including those preceding a refused publication.
 pub fn progress(&self)->RetainedCloneProgress{self.progress}
 /// 🛑️ Observes only the active original lifetime and ownership ledger.
 pub fn checkpoint(&mut self)->Result<(),ValueError>{match (&mut self.owner,self.active){(Owner::Native{decode,..},IoNativeDirection::Decode)|(Owner::Decode{native:decode,..},IoNativeDirection::Decode)=>decode.checkpoint(),(Owner::Native{encode,..},IoNativeDirection::Encode)|(Owner::Encode{native:encode,..},IoNativeDirection::Encode)=>encode.checkpoint(),_=>Err(Self::opposite())}}
 /// 🎟️ Finishes an original native phase before selecting another available caller ledger.
 pub fn select(&mut self,direction:IoNativeDirection)->Result<(),ValueError>{if direction==self.active{return Ok(())}if !matches!(&self.owner,Owner::Native{..})||self.pending.is_some(){return Err(Self::opposite())}self.checkpoint()?;self.active=direction;self.checkpoint()}
 /// 🛬️ Reborrows the original decoder only after its receiving phase is selected.
 pub fn decode(&mut self)->Result<&mut NativeDecodeControl<'b>,ValueError>{self.select(IoNativeDirection::Decode)?;match &mut self.owner{Owner::Native{decode,..}|Owner::Decode{native:decode,..}=>Ok(decode),_=>Err(Self::opposite())}}
 /// 🛫️ Reborrows the original encoder only after its receiving phase is selected.
 pub fn encode(&mut self)->Result<&mut NativeEncodeControl<'b>,ValueError>{self.select(IoNativeDirection::Encode)?;match &mut self.owner{Owner::Native{encode,..}|Owner::Encode{native:encode,..}=>Ok(encode),_=>Err(Self::opposite())}}
 /// 🎟️ Reborrows the actual decoder beside its independent original ownership grant.
 pub fn snapshot_decode(&mut self)->Result<NativeSnapshotDecodeOwner<'_,'b>,ValueError>{self.select(IoNativeDirection::Decode)?;match &mut self.owner{Owner::Native{decode,grant,..}|Owner::Decode{native:decode,grant}=>{let mut owner=NativeSnapshotDecodeOwner::borrowed(decode,*grant,&mut self.progress);owner.pending=self.pending.as_deref_mut();Ok(owner)},_=>Err(Self::opposite())}}
 /// 🎟️ Reborrows an actual encoder with its original independent snapshot ownership grant.
 pub fn snapshot_encode(&mut self)->Result<NativeSnapshotEncodeOwner<'_,'b>,ValueError>{self.select(IoNativeDirection::Encode)?;match &mut self.owner{Owner::Native{encode,grant,..}|Owner::Encode{native:encode,grant}=>Ok(NativeSnapshotEncodeOwner::borrowed_pending(encode,*grant,&mut self.progress,self.pending.as_deref_mut())),_=>Err(Self::opposite())}}
 fn opposite()->ValueError{ValueError::literal(ValueRefusalKind::UnsupportedOwner,"IO foreign receiving phase requires completion before the opposite direction begins")}
}
impl Drop for IoRunControl<'_, '_>{fn drop(&mut self){if let Some(wallet)=self.wallet.as_deref_mut(){*wallet=self.progress;}}}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
#[cfg(test)]
#[path="🧪️tests/nested-io.rs"]
mod nested_io_tests;
