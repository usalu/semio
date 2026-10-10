//! 📋️ Original operation buffers retain paid collection backing until protocol handoff or closure.
use semio_framework_value::{ValueError,ValueRefusalKind,RetirementDemand,ErasedSnapshotRetirement};
use semio_framework_value::retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep};
use semio_framework_value::list::{PagedList,PagedListError};
use std::mem::{ManuallyDrop,size_of};

pub type ArtifactPreparedOperations=Vec<Vec<u8>>;

/// 📦️ The supplied embedding limits cover original rows and every collection backing.
pub struct ArtifactPreparedOperationsOwner {
 values:ManuallyDrop<PagedList<Option<Vec<u8>>,{isize::MAX as usize}>>,
 contiguous:ManuallyDrop<Option<ArtifactPreparedOperations>>,
 transferred:usize,
 payload_capacity_bytes:usize,
 maximum_operations:usize,
 maximum_capacity_bytes:usize,
 closing:bool,
 closed:bool,
}

fn refusal(error:PagedListError)->ValueError {
 let kind=match error.kind {semio_framework_value::list::PagedListRefusalKind::AllocationFailed=>ValueRefusalKind::AllocationFailed,semio_framework_value::list::PagedListRefusalKind::OwnershipLimit=>ValueRefusalKind::OwnershipLimit,_=>ValueRefusalKind::InvariantViolated};
 ValueError::literal(kind,error.reason)
}

impl ArtifactPreparedOperationsOwner {
 /// 🎟️ Installs actual limits without allocating a row or collection frame.
 pub fn empty(maximum_operations:usize,maximum_capacity_bytes:usize)->Result<Self,ValueError>{
  if maximum_operations>isize::MAX as usize/size_of::<Vec<u8>>()||maximum_capacity_bytes>isize::MAX as usize{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"operation collection limits exceed addressable ownership"));}
  Ok(Self{values:ManuallyDrop::new(PagedList::empty()),contiguous:ManuallyDrop::new(None),transferred:0,payload_capacity_bytes:0,maximum_operations,maximum_capacity_bytes,closing:false,closed:false})
 }
 pub fn begin_close(&mut self){self.closing=true;}
 pub fn len(&self)->usize{self.values.len()}
 pub fn row_at(&self,index:usize)->Option<&[u8]>{self.values.get(index).and_then(Option::as_ref).map(Vec::as_slice)}
 pub fn prepared_prefix(&self)->Option<&[Vec<u8>]>{self.contiguous.as_ref().map(Vec::as_slice)}
 pub fn allocated_bytes(&self)->usize{self.values.allocated_bytes()+self.payload_capacity_bytes+self.contiguous.as_ref().map_or(0,|value|value.capacity()*size_of::<Vec<u8>>())}

 /// 🤝️ Returns the original buffer on a denied transfer or a separately paid page birth.
 pub fn append_original(&mut self,original:Vec<u8>,grant:RetainedCloneGrant)->Result<(Option<Vec<u8>>,RetainedCloneProgress),(ValueError,Vec<u8>)>{
  if self.closing||self.closed||self.contiguous.is_some(){return Err((ValueError::literal(ValueRefusalKind::InvariantViolated,"operation collection is closing or transferring"),original));}
  if self.values.len()>=self.maximum_operations{return Err((ValueError::literal(ValueRefusalKind::OwnershipLimit,"operation collection row extent exhausted"),original));}
  if self.allocated_bytes().checked_add(original.capacity()).is_none_or(|total|total>self.maximum_capacity_bytes){return Err((ValueError::literal(ValueRefusalKind::OwnershipLimit,"operation collection original payload exceeds total authority"),original));}
  if grant.maximum_items==0{return Ok((Some(original),Default::default()));}
  if !self.values.has_reserved_slot(){
   let depth=match self.values.next_reserve_depth_demand().map_err(refusal).and_then(|depth|depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"operation collection admission depth overflow"))){Ok(depth)=>depth,Err(error)=>return Err((error,original))};
   let capacity=match self.values.next_exact_capacity_allocation_bytes(self.maximum_operations).map_err(refusal){Ok(Some(capacity))=>capacity,Ok(None)=>return Err((ValueError::literal(ValueRefusalKind::InvariantViolated,"operation collection lost its unfunded row slot"),original)),Err(error)=>return Err((error,original))};
   if self.allocated_bytes().checked_add(original.capacity()).and_then(|total|total.checked_add(capacity)).is_none_or(|total|total>self.maximum_capacity_bytes){return Err((ValueError::literal(ValueRefusalKind::OwnershipLimit,"operation collection page exceeds total authority"),original));}
   if grant.maximum_depth<depth||grant.maximum_capacity_bytes<capacity{return Ok((Some(original),Default::default()));}
   match self.values.reserve_exact_capacity_one(self.maximum_operations,grant.maximum_capacity_bytes){
    Ok(step)=>return Ok((Some(original),RetainedCloneProgress{copied_items:usize::from(step.progressed),retained_capacity_bytes:step.allocated_bytes,..Default::default()})),
    Err(error)=>return Err((refusal(error.refusal()).with_retained_progress(RetainedCloneProgress{copied_items:usize::from(error.allocated_bytes!=0),retained_capacity_bytes:error.allocated_bytes,..Default::default()}),original)),
   }
  }
  let depth=match self.values.next_push_depth_demand().map_err(refusal).and_then(|depth|depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"operation row transfer depth overflow"))){Ok(depth)=>depth,Err(error)=>return Err((error,original))};
  let copied_bytes=size_of::<Option<Vec<u8>>>()+size_of::<usize>();
  if grant.maximum_depth<depth||grant.maximum_copy_bytes<copied_bytes{return Ok((Some(original),Default::default()));}
  let capacity=original.capacity();
  if let Err(mut returned)=self.values.push_reserved(Some(original)){return Err((ValueError::literal(ValueRefusalKind::InvariantViolated,"operation collection reserved row refused its original owner"),returned.take().expect("rejected original row remains present")));}
  self.payload_capacity_bytes+=capacity;
  Ok((None,RetainedCloneProgress{copied_items:1,copied_bytes,..Default::default()}))
 }

 /// 📬️ Funds only the protocol row-header backing and moves each original buffer once.
 pub fn prepare_handoff(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if self.closing||self.closed{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"operation collection handoff requires its live original owner"));}
  if grant.maximum_items==0||grant.maximum_depth==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
  if self.contiguous.is_none(){
   let count=self.values.len();let capacity=count*size_of::<Vec<u8>>();let copied_bytes=size_of::<ArtifactPreparedOperations>();
   if self.allocated_bytes().checked_add(capacity).is_none_or(|total|total>self.maximum_capacity_bytes){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"operation protocol backing exceeds total authority"));}
   if grant.maximum_capacity_bytes<capacity||grant.maximum_copy_bytes<copied_bytes{return Ok(RetainedCloneStep::Progress(Default::default()));}
   let output=if count==0{Vec::new()}else{let layout=std::alloc::Layout::array::<Vec<u8>>(count).map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"operation protocol backing exceeds addressable ownership"))?;let pointer=std::ptr::NonNull::new(unsafe{std::alloc::alloc(layout)}).ok_or_else(||ValueError::literal(ValueRefusalKind::AllocationFailed,"operation protocol backing allocation refused"))?;unsafe{Vec::from_raw_parts(pointer.as_ptr().cast(),0,count)}};
   *self.contiguous=Some(output);
   return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes,retained_capacity_bytes:capacity,..Default::default()}));
  }
  if self.transferred<self.values.len(){
   let copied_bytes=size_of::<Vec<u8>>()*2+size_of::<usize>();
   let depth=self.values.next_pop_depth_demand().map_err(refusal)?.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"operation protocol transfer depth overflow"))?;
   if grant.maximum_copy_bytes<copied_bytes||grant.maximum_depth<depth{return Ok(RetainedCloneStep::Progress(Default::default()));}
   let row=self.values.get_mut(self.transferred).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"operation protocol lost its original row"))?;
   let original=row.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"operation protocol row was already transferred"))?;
   self.contiguous.as_mut().expect("protocol backing remains retained").push(original);self.transferred+=1;
   return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes,..Default::default()}));
  }
  self.close_paged_one(grant)
 }

 /// 📨️ Transfers the exact paid protocol sequence after its original pages are closed.
 pub fn take_prepared(&mut self,grant:RetainedCloneGrant)->Result<Option<(ArtifactPreparedOperations,RetainedCloneProgress)>,ValueError>{
  if self.closing||self.closed{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"operation protocol handoff requires its live original owner"));}
  let copied_bytes=size_of::<ArtifactPreparedOperations>()+size_of::<usize>();
  if !self.values.terminal_is_empty()||self.contiguous.is_none()||grant.maximum_items==0||grant.maximum_copy_bytes<copied_bytes||grant.maximum_depth==0{return Ok(None);}
  let original=self.contiguous.take().expect("protocol sequence remains present");self.payload_capacity_bytes=0;self.closed=true;
  Ok(Some((original,RetainedCloneProgress{copied_items:1,copied_bytes,..Default::default()})))
 }

 fn paged_demands(&self)->Result<RetirementDemand,ValueError>{
  if let Some(Some(row))=self.values.last(){return Ok(RetirementDemand{copy_bytes:if row.capacity()!=0{size_of::<Vec<u8>>()*2}else{size_of::<Option<Vec<u8>>>()},release_bytes:row.capacity(),depth:self.values.next_pop_depth_demand().map_err(refusal)?.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"operation collection close depth overflow"))?,..Default::default()});}
  let(copy_bytes,release_bytes,depth)=if !self.values.is_empty(){(size_of::<Option<Vec<u8>>>(),0,self.values.next_pop_depth_demand().map_err(refusal)?)}else{(0,self.values.next_release_allocation_bytes().map_err(refusal)?,self.values.next_release_depth_demand().map_err(refusal)?)};
  Ok(RetirementDemand{copy_bytes,release_bytes,depth:depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"operation collection close depth overflow"))?,..Default::default()})
 }
 fn close_paged_one(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if self.values.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}
  let demand=self.paged_demands()?;
  if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(RetainedCloneStep::Progress(Default::default()));}
  let mut progress=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};
  if let Some(Some(row))=self.values.last_mut(){if row.capacity()!=0{let original=std::mem::take(row);self.payload_capacity_bytes-=original.capacity();drop(original);progress.released_bytes=demand.release_bytes;}else{self.values.last_mut().expect("original last row remains present").take();}}
  else if !self.values.is_empty(){self.values.pop();}
  else{progress.released_bytes=self.values.release_empty_page(grant.maximum_release_bytes).map_err(refusal)?.released_allocation_bytes;}
  Ok(if self.values.terminal_is_empty(){RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})
 }
 pub fn retirement_demands(&self)->Result<RetirementDemand,ValueError>{
  if self.closed{return Ok(Default::default());}
  if let Some(output)=self.contiguous.as_ref(){if let Some(row)=output.last(){return Ok(RetirementDemand{copy_bytes:if row.capacity()!=0{size_of::<Vec<u8>>()*2}else{size_of::<Vec<u8>>()},release_bytes:row.capacity(),depth:1,..Default::default()});}return Ok(RetirementDemand{copy_bytes:size_of::<ArtifactPreparedOperations>(),release_bytes:output.capacity()*size_of::<Vec<u8>>(),depth:1,..Default::default()});}
  self.paged_demands()
 }
}

impl ErasedSnapshotRetirement for ArtifactPreparedOperationsOwner {
 fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.retirement_demands()?.copy_bytes)}
 fn next_capacity_byte_demand(&self,_:usize)->Result<usize,ValueError>{Ok(0)}
 fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.retirement_demands()?.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.retirement_demands()?.depth)}
 fn terminal_is_empty(&self)->bool{self.closed&&self.values.terminal_is_empty()&&self.contiguous.is_none()}
 fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if self.closed{return Ok(RetainedCloneStep::Complete(Default::default()));}
  let demand=self.retirement_demands()?;
  if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(RetainedCloneStep::Progress(Default::default()));}
  self.closing=true;
  if let Some(output)=self.contiguous.as_mut(){let progress=if let Some(row)=output.last_mut(){if row.capacity()!=0{let original=std::mem::take(row);self.payload_capacity_bytes-=original.capacity();drop(original);}else{output.pop();}RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()}}else{drop(self.contiguous.take());RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()}};return Ok(RetainedCloneStep::Progress(progress));}
  let step=self.close_paged_one(grant)?;if self.values.terminal_is_empty(){self.closed=true;}
  Ok(if self.closed{RetainedCloneStep::Complete(step.progress())}else{step})
 }
}
impl Drop for ArtifactPreparedOperationsOwner {
 fn drop(&mut self){assert!(self.values.terminal_is_empty()&&self.contiguous.is_none(),"original operation rows require paid handoff or terminal closure");unsafe{ManuallyDrop::drop(&mut self.values);ManuallyDrop::drop(&mut self.contiguous)};}
}
