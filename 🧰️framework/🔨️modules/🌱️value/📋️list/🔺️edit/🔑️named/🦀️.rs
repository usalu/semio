//! 🔑️ Named sparse edits map exact immutable UTF-8 keys to the shared strict indexed planner.
use crate::{HeapSortCursor,HeapSortWork,IndexedEditShape,IndexedEditPlan,IndexedEditKind,IndexedEditFault,IndexedEditStep,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneClose,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,retained_clone_grant_admits,RETAINED_CLONE_BULK_PAGE_BYTES}};
use std::{cmp::Ordering,mem::{ManuallyDrop,size_of}};
#[path="🧳️ranked/🦀️.rs"] pub mod ranked;
#[path="📇️index/🦀️.rs"] pub mod index;
pub use ranked::{NamedRankedSource,VecRankedKeys,PersistentRankedKeys};
pub use index::{NamedKeyIndexPreparation,AdmittedNamedKeyIndex,IndexedNamedSource,KeyPreservingPersistentSet,NamedPersistentRoot,NamedPersistentRootPreparation,NamedRecordKey,NamedPersistentRootReceiver};

/// 🗂️ Original key lanes preserve authored ordinals independently of ordering.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum NamedEditLane{Base,Removed,Modified,Added}
/// 📎️ Every key is borrowed from the caller's original immutable source throughout admission.
pub trait NamedEditSource{
 fn shape(&self)->IndexedEditShape;
 fn key_demand(&self,lane:NamedEditLane,ordinal:usize)->Result<RetirementDemand,ValueError>;
 fn observe_key(&self,lane:NamedEditLane,ordinal:usize,grant:RetainedCloneGrant)->Result<Option<(&str,RetainedCloneProgress)>,ValueError>;
 fn added_index(&self,ordinal:usize)->usize;
 fn admitted_base(&self)->Option<&AdmittedNamedKeyIndex>{None}
}
/// 🎟️ A contiguous borrowed source prices its actual fixed key header and original ranked coordinate.
pub fn contiguous_named_key_demand<T:?Sized>(source:&T)->RetirementDemand{RetirementDemand{copy_bytes:std::mem::size_of_val(source)+size_of::<usize>()*2+size_of::<&str>()*2,depth:1,..Default::default()}}
/// 👁️ The borrowed key selector executes only after its original contiguous header quote is admitted.
pub fn observe_contiguous_named_key<'a>(d:RetirementDemand,grant:RetainedCloneGrant,key:impl FnOnce()->&'a str)->Option<(&'a str,RetainedCloneProgress)>{if !retained_clone_grant_admits(grant,d){return None;}Some((key(),RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}
/// 🚫️ Semantic key conflicts settle before any candidate collection changes.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum NamedEditFault{DuplicateBase,MissingRemoved,MissingModified,DuplicateAdded,RetainedAdded,Coordinates(IndexedEditFault)}
/// 🚦️ One funded work page, admitted plan or settled key refusal.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum NamedEditStep{Progress(RetainedCloneProgress),Admitted(RetainedCloneProgress),Refused(NamedEditFault,RetainedCloneProgress)}
impl NamedEditStep{pub fn progress(self)->RetainedCloneProgress{match self{Self::Progress(p)|Self::Admitted(p)|Self::Refused(_,p)=>p}}}
#[derive(Default)]
struct Compare{offset:usize}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"named edit plan lost its original immutable source or admitted phase")}
impl Compare{
 fn fixed()->usize{size_of::<Self>()*2+size_of::<usize>()*4+size_of::<Ordering>()*2}
 fn demand(&self,a:&str,b:&str,body:usize)->Result<RetirementDemand,ValueError>{
  let remaining=a.len().min(b.len()).checked_sub(self.offset).ok_or_else(invalid)?;
  let page=remaining.min(RETAINED_CLONE_BULK_PAGE_BYTES).min(body.saturating_sub(Self::fixed()).max(2)/2);
  Ok(RetirementDemand{copy_bytes:Self::fixed()+page*2,depth:1,..Default::default()})
 }
 fn advance(&mut self,a:&str,b:&str,grant:RetainedCloneGrant)->Result<(Option<Ordering>,RetainedCloneProgress),ValueError>{
  let demand=self.demand(a,b,grant.maximum_copy_bytes)?;
  if !retained_clone_grant_admits(grant,demand){return Ok((None,Default::default()));}
  let end=self.offset+(demand.copy_bytes-Self::fixed())/2;
  let order=a.as_bytes()[self.offset..end].cmp(&b.as_bytes()[self.offset..end]);self.offset=end;
  let settled=order!=Ordering::Equal||end==a.len().min(b.len());
  Ok((settled.then(||if order==Ordering::Equal{a.len().cmp(&b.len())}else{order}),RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))
 }
}
/// 🧭️ Paid key directories and paged comparisons preserve original payloads and authored coordinate custody.
pub struct NamedEditPlan{shape:IndexedEditShape,base:ManuallyDrop<Option<Vec<usize>>>,added:ManuallyDrop<Option<Vec<usize>>>,removed:ManuallyDrop<Option<Vec<usize>>>,modified:ManuallyDrop<Option<Vec<usize>>>,base_index:Option<AdmittedNamedKeyIndex>,plan:Option<IndexedEditPlan>,retirement:RetainedCloneClose,sort:HeapSortCursor,compare:Compare,phase:u8,cursor:usize,low:usize,high:usize,searching:bool,closing:bool,fault:Option<NamedEditFault>}
impl NamedEditPlan{
 pub fn new(shape:IndexedEditShape)->Self{Self{shape,base:ManuallyDrop::new(None),added:ManuallyDrop::new(None),removed:ManuallyDrop::new(None),modified:ManuallyDrop::new(None),base_index:None,plan:None,retirement:Default::default(),sort:HeapSortCursor::new(0),compare:Default::default(),phase:0,cursor:0,low:0,high:0,searching:false,closing:false,fault:None}}
 /// 📦️ An already admitted original index transfers under one quoted header action without another base sort.
 pub fn indexed_constructor_demand()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Self>()*2+size_of::<Option<AdmittedNamedKeyIndex>>()*2,depth:1,..Default::default()}}
 pub fn new_indexed(shape:IndexedEditShape,index:&mut Option<AdmittedNamedKeyIndex>,grant:RetainedCloneGrant)->Result<Option<Self>,ValueError>{let d=Self::indexed_constructor_demand();if !retained_clone_grant_admits(grant,d){return Ok(None);}let original=index.as_ref().ok_or_else(invalid)?;if original.len()!=shape.base_len||!original.ready(){return Err(invalid());}let mut owner=Self::new(shape);owner.base_index=index.take();owner.phase=4;Ok(Some(owner))}
 fn base_order(&self)->&[usize]{if let Some(index)=self.base_index.as_ref(){index.order()}else{self.base.as_deref().expect("original admitted named base directory")}}
 pub(super) fn base_take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.ready()||self.shape.removed+self.shape.modified+self.shape.added!=0||self.base_index.is_some(){return Err(invalid());}Ok(Self::scalar())}
 pub(super) fn take_base_order(&mut self,grant:RetainedCloneGrant)->Result<Option<(Vec<usize>,RetainedCloneProgress)>,ValueError>{let d=self.base_take_demand()?;if !retained_clone_grant_admits(grant,d){return Ok(None);}Ok(Some((self.base.take().ok_or_else(invalid)?,RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})))}
 pub fn settled(&self)->bool{self.phase==15}
 pub fn ready(&self)->bool{self.settled()&&self.fault.is_none()&&!self.closing}
 pub fn fault(&self)->Option<NamedEditFault>{self.fault}
 pub fn plan(&self)->Result<&IndexedEditPlan,ValueError>{if !self.ready(){return Err(invalid());}self.plan.as_ref().filter(|plan|plan.ready()).ok_or_else(invalid)}
 pub fn modified_index(&self,ordinal:usize)->Result<usize,ValueError>{if !self.ready(){return Err(invalid());}self.modified.as_ref().and_then(|values|values.get(ordinal)).copied().ok_or_else(invalid)}
 /// 📐️ Quotes the original authored ordinal and fixed coordinate directory observation.
 pub fn modified_index_demand(&self,ordinal:usize)->Result<RetirementDemand,ValueError>{if !self.ready()||ordinal>=self.shape.modified||self.modified.as_ref().is_none_or(|rows|ordinal>=rows.len()){return Err(invalid());}Ok(RetirementDemand{copy_bytes:size_of::<Vec<usize>>()+size_of::<Option<Vec<usize>>>()+size_of::<usize>()*3+size_of::<u8>()*2,depth:1,..Default::default()})}
 /// 📇️ Reads an admitted original coordinate only under its actual observation currencies.
 pub fn observe_modified_index(&self,ordinal:usize,grant:RetainedCloneGrant)->Result<Option<(usize,RetainedCloneProgress)>,ValueError>{let demand=self.modified_index_demand(ordinal)?;if !retained_clone_grant_admits(grant,demand){return Ok(None);}Ok(Some((self.modified_index(ordinal)?,RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})))}
 pub fn fixed_copy_bytes()->usize{size_of::<Self>()*2+size_of::<usize>()*6}
 fn scalar()->RetirementDemand{RetirementDemand{copy_bytes:Self::fixed_copy_bytes(),depth:1,..Default::default()}}
 fn nested(mut demand:RetirementDemand)->Result<RetirementDemand,ValueError>{demand.copy_bytes=demand.copy_bytes.checked_add(Self::fixed_copy_bytes()).ok_or_else(invalid)?;demand.depth=demand.depth.checked_add(1).ok_or_else(invalid)?;Ok(demand)}
 fn child(grant:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-Self::fixed_copy_bytes(),maximum_depth:grant.maximum_depth-1,..grant}}
 fn receipt(mut progress:RetainedCloneProgress)->RetainedCloneProgress{progress.copied_bytes+=Self::fixed_copy_bytes();progress}
 fn refuse(&mut self,fault:NamedEditFault,progress:RetainedCloneProgress)->NamedEditStep{self.fault=Some(fault);self.phase=15;NamedEditStep::Refused(fault,progress)}
 fn key_coordinates(&self)->Option<((NamedEditLane,usize),(NamedEditLane,usize))>{
  match self.phase{
   2|12=>if let HeapSortWork::Compare{left,right}=self.sort.next(){let(lane,order)=if self.phase==2{(NamedEditLane::Base,self.base_order())}else{(NamedEditLane::Added,self.added.as_ref().unwrap().as_slice())};Some(((lane,order[left]),(lane,order[right])))}else{None},
   3|13=>{let(lane,count,order)=if self.phase==3{(NamedEditLane::Base,self.shape.base_len,self.base_order())}else{(NamedEditLane::Added,self.shape.added,self.added.as_ref().unwrap().as_slice())};if self.cursor<count{Some(((lane,order[self.cursor-1]),(lane,order[self.cursor])))}else{None}},
   5|7|14 if self.searching&&self.low<self.high=>{let lane=match self.phase{5=>NamedEditLane::Removed,7=>NamedEditLane::Modified,_=>NamedEditLane::Added};let middle=self.low+(self.high-self.low)/2;Some(((lane,self.cursor),(NamedEditLane::Base,self.base_order()[middle])))},
   _=>None,
  }
 }
 pub fn demand(&self,source:&impl NamedEditSource,body:usize)->Result<RetirementDemand,ValueError>{
  if self.closing||self.settled()||source.shape()!=self.shape{return Err(invalid());}
  if let Some(index)=self.base_index.as_ref(){if !source.admitted_base().is_some_and(|actual|index.same_original(actual)){return Err(invalid());}}
  if self.phase==9{return Self::nested(self.plan.as_ref().ok_or_else(invalid)?.demand()?);}
  let mut demand=Self::scalar();let count=match self.phase{0=>self.shape.base_len,4=>self.shape.removed,6=>self.shape.modified,10=>self.shape.added,_=>0};
  demand.capacity_bytes=count.checked_mul(size_of::<usize>()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"named coordinate extent overflow"))?;
  if let Some((a,b))=self.key_coordinates(){let a=source.key_demand(a.0,a.1)?;let b=source.key_demand(b.0,b.1)?;let metadata=Self::fixed_copy_bytes()+a.copy_bytes+b.copy_bytes+Compare::fixed();let page=RETAINED_CLONE_BULK_PAGE_BYTES.min(body.saturating_sub(metadata).max(2)/2);demand.copy_bytes=metadata.checked_add(page*2).ok_or_else(invalid)?;demand.capacity_bytes=a.capacity_bytes.checked_add(b.capacity_bytes).ok_or_else(invalid)?;demand.release_bytes=a.release_bytes.checked_add(b.release_bytes).ok_or_else(invalid)?;demand.depth=a.depth.max(b.depth).checked_add(1).ok_or_else(invalid)?;}Ok(demand)
 }
 pub fn advance(&mut self,source:&impl NamedEditSource,grant:RetainedCloneGrant)->Result<NamedEditStep,ValueError>{
  let demand=self.demand(source,grant.maximum_copy_bytes)?;if !retained_clone_grant_admits(grant,demand){return Ok(NamedEditStep::Progress(Default::default()));}
  if self.phase==9{
   let removed=self.removed.as_ref().unwrap();let modified=self.modified.as_ref().unwrap();
   let step=self.plan.as_mut().unwrap().advance(Self::child(grant),|kind,ordinal|match kind{IndexedEditKind::Removed=>removed[ordinal],IndexedEditKind::Modified=>modified[ordinal],IndexedEditKind::Added=>source.added_index(ordinal)})?;
   let progress=Self::receipt(step.progress());
   return Ok(match step{IndexedEditStep::Refused(fault,_)=>self.refuse(NamedEditFault::Coordinates(fault),progress),IndexedEditStep::Admitted(_)=>{self.phase=10;NamedEditStep::Progress(progress)},IndexedEditStep::Progress(_)=>NamedEditStep::Progress(progress)});
  }
  let mut progress=RetainedCloneProgress{copied_items:1,copied_bytes:Self::fixed_copy_bytes(),..Default::default()};
  let observation=if let Some((a,b))=self.key_coordinates(){let ad=source.key_demand(a.0,a.1)?;let bd=source.key_demand(b.0,b.1)?;let(a,ap)=source.observe_key(a.0,a.1,grant)?.ok_or_else(invalid)?;let(b,bp)=source.observe_key(b.0,b.1,grant)?.ok_or_else(invalid)?;let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-Self::fixed_copy_bytes()-ad.copy_bytes-bd.copy_bytes,..grant};let(order,receipt)=self.compare.advance(a,b,child)?;progress.copied_bytes+=ap.copied_bytes+bp.copied_bytes+receipt.copied_bytes;progress.retained_capacity_bytes=ap.retained_capacity_bytes+bp.retained_capacity_bytes;progress.released_bytes=ap.released_bytes+bp.released_bytes;if order.is_some(){self.compare=Default::default();}order}else{None};
  match self.phase{
   0|4|6|10=>{let count=match self.phase{0=>self.shape.base_len,4=>self.shape.removed,6=>self.shape.modified,_=>self.shape.added};let mut output=Vec::new();output.try_reserve_exact(count).map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"named coordinate reservation refused"))?;progress.retained_capacity_bytes=output.capacity()*size_of::<usize>();match self.phase{0=>*self.base=Some(output),4=>*self.removed=Some(output),6=>*self.modified=Some(output),_=>*self.added=Some(output)}self.cursor=0;self.searching=false;self.phase+=1;},
   1|11=>{let count=if self.phase==1{self.shape.base_len}else{self.shape.added};if self.cursor==count{self.sort=HeapSortCursor::new(count);self.phase+=1;}else{if self.phase==1{self.base.as_mut().unwrap().push(self.cursor);}else{self.added.as_mut().unwrap().push(self.cursor);}self.cursor+=1;}},
   2|12=>match self.sort.next(){HeapSortWork::Compare{..}=>if let Some(order)=observation{self.sort.advance(Some(order)).map_err(|_|invalid())?;},HeapSortWork::Swap{left,right}=>{if self.phase==2{self.base.as_mut().unwrap().swap(left,right);}else{self.added.as_mut().unwrap().swap(left,right);}self.sort.advance(None).map_err(|_|invalid())?;},HeapSortWork::Advance=>self.sort.advance(None).map_err(|_|invalid())?,HeapSortWork::Complete=>{self.cursor=1;self.phase+=1;}},
   3|13=>{let count=if self.phase==3{self.shape.base_len}else{self.shape.added};if self.cursor>=count{self.cursor=0;self.searching=false;self.phase+=1;}else if let Some(order)=observation{if order==Ordering::Equal{return Ok(self.refuse(if self.phase==3{NamedEditFault::DuplicateBase}else{NamedEditFault::DuplicateAdded},progress));}self.cursor+=1;}},
   5|7|14=>{let count=match self.phase{5=>self.shape.removed,7=>self.shape.modified,_=>self.shape.added};if self.cursor==count{self.cursor=0;self.phase+=1;}else if !self.searching{self.low=0;self.high=self.shape.base_len;self.searching=true;}else if self.low==self.high{if self.phase!=14{return Ok(self.refuse(if self.phase==5{NamedEditFault::MissingRemoved}else{NamedEditFault::MissingModified},progress));}self.cursor+=1;self.searching=false;}else if let Some(order)=observation{let middle=self.low+(self.high-self.low)/2;match order{Ordering::Less=>self.high=middle,Ordering::Greater=>self.low=middle+1,Ordering::Equal=>{let index=self.base_order()[middle];if self.phase==5{self.removed.as_mut().unwrap().push(index);}else if self.phase==7{self.modified.as_mut().unwrap().push(index);}else if self.plan.as_ref().unwrap().entries(IndexedEditKind::Removed)?.binary_search_by_key(&index,|row|row.index).is_err(){return Ok(self.refuse(NamedEditFault::RetainedAdded,progress));}self.cursor+=1;self.searching=false;}}}},
   8=>{self.plan=Some(IndexedEditPlan::new(self.shape));self.phase=9;},
   _=>return Err(invalid()),
  }
  Ok(if self.ready(){NamedEditStep::Admitted(progress)}else{NamedEditStep::Progress(progress)})
 }
 pub fn begin_close(&mut self){self.closing=true;if let Some(plan)=self.plan.as_mut(){plan.begin_close();}if let Some(index)=self.base_index.as_mut(){index.begin_close();}}
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if !self.closing{return Err(invalid());}if self.terminal_is_empty(){return Ok(Default::default());}
  if let Some(plan)=self.plan.as_ref(){return if plan.terminal_is_empty(){Ok(Self::scalar())}else{Self::nested(plan.close_demand(body.saturating_sub(Self::fixed_copy_bytes()))?)};}
  if let Some(index)=self.base_index.as_ref(){return if index.terminal_is_empty(){Ok(Self::scalar())}else{Self::nested(index.close_demand(body.saturating_sub(Self::fixed_copy_bytes()))?)};}
  Self::nested(self.retirement.next_owner_demands::<Vec<usize>>(self.base.is_some()||self.added.is_some()||self.removed.is_some()||self.modified.is_some(),&None,body.saturating_sub(Self::fixed_copy_bytes()))?)
 }
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if !self.closing{return Err(invalid());}if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}
  let demand=self.close_demand(grant.maximum_copy_bytes)?;if !retained_clone_grant_admits(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()));}
  if let Some(plan)=self.plan.as_mut(){if !plan.terminal_is_empty(){return plan.close_step(Self::child(grant)).map(|step|RetainedCloneStep::Progress(Self::receipt(step.progress())));}self.plan=None;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:Self::fixed_copy_bytes(),..Default::default()}));}
  if let Some(index)=self.base_index.as_mut(){if !index.terminal_is_empty(){return index.close_step(Self::child(grant)).map(|step|RetainedCloneStep::Progress(Self::receipt(step.progress())));}self.base_index=None;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:Self::fixed_copy_bytes(),..Default::default()}));}
  if !self.retirement.is_empty(){return self.retirement.step_granted(Self::child(grant)).map(|step|RetainedCloneStep::Progress(Self::receipt(step.progress())));}
  for owner in [&mut *self.base,&mut *self.added,&mut *self.removed,&mut *self.modified]{if let Some(step)=self.retirement.begin_granted(owner,Self::child(grant))?{return Ok(RetainedCloneStep::Progress(Self::receipt(step.progress())));}}
  Err(invalid())
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.base_index.is_none()&&self.plan.is_none()&&self.base.is_none()&&self.added.is_none()&&self.removed.is_none()&&self.modified.is_none()&&self.retirement.is_empty()}
}
impl Drop for NamedEditPlan{fn drop(&mut self){let empty=self.base_index.is_none()&&self.plan.is_none()&&self.base.is_none()&&self.added.is_none()&&self.removed.is_none()&&self.modified.is_none()&&self.retirement.is_empty();assert!(std::thread::panicking()||empty,"named coordinate directories require original funded closure");if empty{unsafe{ManuallyDrop::drop(&mut self.base);ManuallyDrop::drop(&mut self.added);ManuallyDrop::drop(&mut self.removed);ManuallyDrop::drop(&mut self.modified);}}}}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
