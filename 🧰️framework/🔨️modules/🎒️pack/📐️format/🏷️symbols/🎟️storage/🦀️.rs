//! 🎟️ Original symbol pages admit and retire through independent caller authority.
use super::{RetainedPackSymbolTable,RetainedPackSymbolSpans,RetainedPackSymbolScalars,RetainedPackSymbolSpan,RetainedPackCatalogFault,RetainedPackCatalogAllocationError};
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retirement::controlled::ControlledRetirement,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use std::mem::size_of;
pub(super) struct SymbolStorage{symbols:RetainedPackSymbolSpans,scalars:RetainedPackSymbolScalars}
semio_framework_value::artifact_retire_struct!(SymbolStorage{symbols,scalars});
semio_framework_value::artifact_retire_struct!(RetainedPackSymbolSpan{scalar_start,scalar_len,utf8_len});
fn funded(grant:RetainedCloneGrant,demand:RetirementDemand)->bool{grant.maximum_items!=0&&grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes&&grant.maximum_depth>=demand.depth}
impl RetainedPackSymbolTable{
 /// 📏️ Quotes the actual next span metadata or payload page independently of its backing extent.
 pub fn symbol_storage_admission_demands(&mut self,target:usize,offset:u64)->Result<RetirementDemand,RetainedPackCatalogFault>{
  Ok(match self.next_symbol_allocation_bytes(target,offset)?{Some(bytes)=>RetirementDemand{copy_bytes:self.symbols.next_capacity_copy_byte_demand(target).map_err(|error|RetainedPackCatalogFault::from_paged_refusal(error,"retained-pack.symbol-copy",offset))?,capacity_bytes:bytes,release_bytes:0,depth:1},None=>Default::default()})
 }
 /// 📏️ Quotes the actual next Unicode scalar page without conflating copy and capacity.
 pub fn scalar_storage_admission_demands(&mut self,target:usize,offset:u64)->Result<RetirementDemand,RetainedPackCatalogFault>{
  Ok(match self.next_scalar_allocation_bytes(target,offset)?{Some(bytes)=>RetirementDemand{copy_bytes:self.scalars.next_capacity_copy_byte_demand(target).map_err(|error|RetainedPackCatalogFault::from_paged_refusal(error,"retained-pack.scalar-copy",offset))?,capacity_bytes:bytes,release_bytes:0,depth:1},None=>Default::default()})
 }
 /// 📦️ Preserves every actual span page on denied work, copy, capacity, or depth.
 pub fn admit_symbol_storage(&mut self,target:usize,grant:RetainedCloneGrant,offset:u64)->Result<RetainedCloneProgress,RetainedPackCatalogAllocationError>{self.admit_original_storage(target,grant,offset,true)}
 /// 📦️ Preserves every actual scalar page on denied work, copy, capacity, or depth.
 pub fn admit_scalar_storage(&mut self,target:usize,grant:RetainedCloneGrant,offset:u64)->Result<RetainedCloneProgress,RetainedPackCatalogAllocationError>{self.admit_original_storage(target,grant,offset,false)}
 fn admit_original_storage(&mut self,target:usize,grant:RetainedCloneGrant,offset:u64,symbol:bool)->Result<RetainedCloneProgress,RetainedPackCatalogAllocationError>{
  let demand=if symbol{self.symbol_storage_admission_demands(target,offset)}else{self.scalar_storage_admission_demands(target,offset)}.map_err(|fault|RetainedPackCatalogAllocationError{allocated_bytes:0,fault})?;
  if demand.capacity_bytes==0||!funded(grant,demand){return Ok(Default::default())}
  let result=if symbol{self.symbols.reserve_capacity_one_funded(target,grant)}else{self.scalars.reserve_capacity_one_funded(target,grant)};
  match result{Ok(receipt)=>Ok(receipt),Err(error)=>{let fault=RetainedPackCatalogFault::from_paged_allocation(error,"retained-pack.original-symbol-storage",offset);self.fault.get_or_insert(fault);Err(RetainedPackCatalogAllocationError{allocated_bytes:error.allocated_bytes,fault})}}
 }
 /// 📏️ Quotes the same original controlled page owner with a separately declared frame ceiling.
 pub fn symbol_storage_retirement_demands(&self,maximum_body_bytes:usize)->Result<RetirementDemand,ValueError>{
  if let Some(owner)=&self.storage_retirement{if !owner.terminal_is_empty(){return Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?.checked_add(size_of::<usize>()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"symbol storage copy extent"))?,capacity_bytes:owner.next_capacity_byte_demand(maximum_body_bytes)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"symbol storage depth extent"))?})}return Ok(RetirementDemand{copy_bytes:size_of::<Option<ControlledRetirement<SymbolStorage>>>()+4*size_of::<usize>()+2*size_of::<bool>(),depth:1,..Default::default()})}
  if self.closed{return Ok(Default::default())}Ok(RetirementDemand{copy_bytes:size_of::<SymbolStorage>()+size_of::<Option<ControlledRetirement<SymbolStorage>>>()+size_of::<usize>()+size_of::<bool>(),depth:1,..Default::default()})
 }
 /// ♻️ Transfers actual pages into their typed owner, then closes only under the original grant.
 pub fn retire_symbol_storage(&mut self,maximum_body_bytes:usize,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}
  let demand=self.symbol_storage_retirement_demands(maximum_body_bytes)?;if !funded(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()))}
  if self.storage_retirement.is_none(){
   let bytes=self.allocated_bytes();let original=SymbolStorage{symbols:std::mem::replace(&mut self.symbols,RetainedPackSymbolSpans::empty()),scalars:std::mem::replace(&mut self.scalars,RetainedPackSymbolScalars::empty())};
   match ControlledRetirement::new(original){Ok(owner)=>{self.storage_retirement=Some(owner);self.storage_retiring_bytes=bytes;self.closing=true;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))},Err((error,original))=>{self.symbols=original.symbols;self.scalars=original.scalars;return Err(error)}}
  }
  let owner=self.storage_retirement.as_mut().unwrap();if !owner.terminal_is_empty(){let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-size_of::<usize>(),maximum_depth:grant.maximum_depth-1,..grant};let step=owner.step(child)?;let mut receipt=match step{RetainedCloneStep::Progress(receipt)|RetainedCloneStep::Complete(receipt)=>receipt};if receipt.retained_capacity_bytes!=0||receipt.released_bytes!=0{self.storage_retiring_bytes=self.storage_retiring_bytes.checked_add(receipt.retained_capacity_bytes).and_then(|bytes|bytes.checked_sub(receipt.released_bytes)).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"symbol storage physical conservation"))?;receipt.copied_bytes+=size_of::<usize>();}return Ok(RetainedCloneStep::Progress(receipt))}
  if self.storage_retiring_bytes!=0{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"terminal symbol storage retains physical bytes"))}
  self.storage_retirement.take();self.published_scalars=0;self.pending_utf8_bytes=0;self.utf8_bytes=0;self.closed=true;self.closing=true;
  Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))
 }
}
