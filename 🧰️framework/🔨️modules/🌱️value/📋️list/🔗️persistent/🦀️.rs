//! 🔗️ Immutable shared fixed-fanout pages retain original readers while candidates replace only changed paths.
use crate::{RetirementDemand,ValueError,ValueRefusalKind,retirement::RetireOwned,shared_issuance::{SealedShared,SharedIssuer},retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneRef}};
/// 🌳️ Every branch and payload leaf has a fixed bounded page footprint.
pub const PERSISTENT_PAGE_FANOUT:usize=16;
#[derive(crate::RetireOwned)]#[value(crate="crate")]
enum Page<T:RetireOwned+Send+Sync+'static>{
 Leaf{rows:[Option<SealedShared<T>>;PERSISTENT_PAGE_FANOUT],len:usize},
 Branch{children:[Option<SealedShared<Page<T>>>;PERSISTENT_PAGE_FANOUT],len:usize,height:u8},
}
fn page_len<T:RetireOwned+Send+Sync+'static>(page:&Page<T>)->usize{match page{Page::Leaf{len,..}|Page::Branch{len,..}=>*len}}
fn child_rank_paid<T:RetireOwned+Send+Sync+'static>(children:&[Option<SealedShared<Page<T>>>;PERSISTENT_PAGE_FANOUT],mut index:usize,insertion:bool,effect:&mut crate::RetainedAllocationEffect<'_>)->Result<Option<(usize,usize)>,ValueError>{let mut last=None;for(slot,child)in children.iter().enumerate(){if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"persistent rank probe exceeded Work"))}let Some(child)=child else{break};let len=page_len(child.get());if index<len{return Ok(Some((slot,index)))}index=index.checked_sub(len).ok_or_else(||ValueError::invariant("persistent rank underflow"))?;last=Some((slot,len))}Ok(if insertion&&index==0{last}else{None})}
#[path="🎮️splice/🦀️.rs"] pub mod splice;
pub use splice::{PersistentSplice,PersistentSpliceKind};
mod traversal {
//! 🔎️ Borrowed ranked page traversal enters each page once and skips complete subtrees by their original cardinalities.
use super::{Page,PersistentList,page_len};
const MAXIMUM_PAGE_DEPTH:usize=usize::BITS as usize/4;
struct Frame<'a,T:crate::retirement::RetireOwned+Send+Sync+'static>{page:&'a Page<T>,next:usize}
impl<T:crate::retirement::RetireOwned+Send+Sync+'static> Copy for Frame<'_,T>{}
impl<T:crate::retirement::RetireOwned+Send+Sync+'static> Clone for Frame<'_,T>{fn clone(&self)->Self{*self}}
/// 🧭️ A fixed-depth borrowed stack never duplicates row owners or allocates traversal storage.
pub struct PersistentListIter<'a,T:crate::retirement::RetireOwned+Send+Sync+'static>{
 stack:[Option<Frame<'a,T>>;MAXIMUM_PAGE_DEPTH],depth:usize,remaining:usize,
 #[cfg(test)] pub(super) visited_pages:usize,
}
impl<'a,T:crate::retirement::RetireOwned+Send+Sync+'static> PersistentListIter<'a,T>{
 fn push(&mut self,page:&'a Page<T>){
  assert!(self.depth<MAXIMUM_PAGE_DEPTH,"persistent page depth exceeded its admitted ranked address space");
  self.stack[self.depth]=Some(Frame{page,next:0});self.depth+=1;
  #[cfg(test)]{self.visited_pages+=1;}
 }
 fn exhaust(&mut self){self.stack=[None;MAXIMUM_PAGE_DEPTH];self.depth=0;self.remaining=0;}
 fn consume(&mut self,mut skip:usize)->Option<&'a T>{
  if skip>=self.remaining{self.exhaust();return None;}
  loop{
   let position=self.depth-1;let frame=self.stack[position].as_mut().expect("remaining immutable rows have an original page");
   match frame.page{
    Page::Leaf{rows,len}=>{
     let available=len-frame.next;
     if skip>=available{skip-=available;self.remaining-=available;self.stack[position]=None;self.depth-=1;continue;}
     frame.next+=skip;self.remaining-=skip+1;let row=rows[frame.next].as_ref().expect("original immutable leaf has a dense occupied prefix");frame.next+=1;return Some(row.get());
    },
    Page::Branch{children,..}=>{
     let child=children.get(frame.next).and_then(Option::as_ref);
     let Some(child)=child else{self.stack[position]=None;self.depth-=1;continue;};
     frame.next+=1;let length=page_len(child.get());
     if skip>=length{skip-=length;self.remaining-=length;continue;}
     self.push(child.get());
    },
   }
  }
 }
}
impl<T:crate::retirement::RetireOwned+Send+Sync+'static> PersistentList<T>{
 /// 👁️ Scans original rows in ranked order with one fixed-depth stack and no payload cloning.
 pub fn iter(&self)->PersistentListIter<'_,T>{
  let mut cursor=PersistentListIter{stack:[None;MAXIMUM_PAGE_DEPTH],depth:0,remaining:self.len,#[cfg(test)]visited_pages:0};
  if let Some(root)=self.root.as_ref(){cursor.push(root.get());}cursor
 }
}
impl<'a,T:crate::retirement::RetireOwned+Send+Sync+'static> Iterator for PersistentListIter<'a,T>{
 type Item=&'a T;
 fn next(&mut self)->Option<Self::Item>{self.consume(0)}
 fn nth(&mut self,n:usize)->Option<Self::Item>{self.consume(n)}
 fn size_hint(&self)->(usize,Option<usize>){(self.remaining,Some(self.remaining))}
 fn count(self)->usize{self.remaining}
 fn last(mut self)->Option<Self::Item>{self.remaining.checked_sub(1).and_then(|n|self.consume(n))}
}
impl<T:crate::retirement::RetireOwned+Send+Sync+'static> ExactSizeIterator for PersistentListIter<'_,T>{fn len(&self)->usize{self.remaining}}
impl<T:crate::retirement::RetireOwned+Send+Sync+'static> std::iter::FusedIterator for PersistentListIter<'_,T>{}
impl<'a,T:crate::retirement::RetireOwned+Send+Sync+'static> IntoIterator for &'a PersistentList<T>{
 type Item=&'a T;type IntoIter=PersistentListIter<'a,T>;
 fn into_iter(self)->Self::IntoIter{self.iter()}
}
}
pub use traversal::PersistentListIter;
#[path="🧳️read/🦀️.rs"] mod retained_read;
pub use retained_read::{PersistentListRead,PersistentListReadStep};
#[path="📥️build/🦀️.rs"] mod retained_build;
pub use retained_build::PersistentListBuilder;
/// 🪪️ Immutable page roots preserve original typed row issuers and every old reader.
#[derive(crate::RetireOwned)]#[value(crate="crate")]
pub struct PersistentList<T:RetireOwned+Send+Sync+'static>{root:Option<SealedShared<Page<T>>>,len:usize,height:u8}
impl<T:RetireOwned+std::fmt::Debug+Send+Sync+'static> std::fmt::Debug for PersistentList<T>{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{f.debug_list().entries(self.iter()).finish()}}
impl<T:RetireOwned+PartialEq+Send+Sync+'static> PartialEq for PersistentList<T>{fn eq(&self,other:&Self)->bool{self.len==other.len&&self.iter().eq(other.iter())}}
impl<T:RetireOwned+Eq+Send+Sync+'static> Eq for PersistentList<T>{}
impl<T:RetireOwned+Send+Sync+'static> std::ops::Index<usize> for PersistentList<T>{type Output=T;fn index(&self,index:usize)->&T{self.get(index).expect("persistent list index exceeds its cardinality")}}

impl<T:RetireOwned+Send+Sync+'static> Default for PersistentList<T>{fn default()->Self{Self{root:None,len:0,height:0}}}
fn cold_unique<T:crate::shared_issuance::ColdRetire>(page:&SealedShared<Page<T>>)->bool{
 if !page.original_is_exclusive(){return false}
 match page.get(){
  Page::Leaf{rows,..}=>rows.iter().flatten().all(|value|value.original_is_exclusive()&&value.get().cold_exclusive()),
  Page::Branch{children,..}=>children.iter().flatten().all(cold_unique),
 }
}
fn cold_take<T:RetireOwned+Send+Sync+'static>(value:SealedShared<T>)->T{crate::shared_issuance::cold_take_exclusive(value)}
fn cold_drain<T:crate::shared_issuance::ColdRetire>(page:SealedShared<Page<T>>){
 match cold_take(page){
  Page::Leaf{rows,..}=>for value in rows.into_iter().flatten(){cold_take(value).cold_release();},
  Page::Branch{children,..}=>for child in children.into_iter().flatten(){cold_drain(child);},
 }
}
fn fits(d:RetirementDemand,g:RetainedCloneGrant)->bool{g.maximum_items>0&&g.maximum_copy_bytes>=d.copy_bytes&&g.maximum_capacity_bytes>=d.capacity_bytes&&g.maximum_release_bytes>=d.release_bytes&&g.maximum_depth>=d.depth}
impl<T:RetireOwned+Send+Sync+'static> PersistentList<T>{
 /// ❄️ Checks all original page and row handles without changing custody.
 pub fn cold_is_exclusive(&self)->bool where T:crate::shared_issuance::ColdRetire{self.root.as_ref().is_none_or(cold_unique)}
 /// ♻️ Dismantles an exclusive cold tree without allocating; refusal leaves it unchanged.
 pub fn cold_dismantle(&mut self)->bool where T:crate::shared_issuance::ColdRetire{
  if !self.cold_is_exclusive(){return false}
  self.cold_dismantle_prechecked();true
 }
 fn cold_dismantle_prechecked(&mut self) where T:crate::shared_issuance::ColdRetire{
  if let Some(root)=self.root.take(){cold_drain(root)}
  self.len=0;self.height=0;
 }
 /// 🧷️ A genuine original list source lends an independent immutable candidate root.
 pub fn project(original:RetainedCloneRef<'_,Self>,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<Option<(Self,RetainedCloneProgress)>,ValueError>{let grant=effect.grant();let mut work=0;let result=(||{
  let demand=Self::projection_demand();if !fits(demand,grant){return Ok(None);}
  let original=original.get();
  let(root,progress)=if let Some(root)=original.root.as_ref(){let turn=root.try_duplicate(effect);work=turn.work;let(root,p)=turn.result?;(Some(root),p)}else{work=1;(None,RetainedCloneProgress{copied_items:1,..Default::default()})};
  Ok(Some((Self{root,len:original.len,height:original.height},progress)))
 })();crate::RetainedWorkOutcome{result,work}}
 /// 🌱️ Leaf admission preserves every owned original row on denial or real allocation refusal.
 /// ⏱️ Bounds the fixed fanout validation and actual sealed page birth without probing retained children.
 pub fn page_birth_work_demand()->u64{1+PERSISTENT_PAGE_FANOUT as u64+SealedShared::<Page<T>>::birth_work_demand()}
 pub fn admit_leaf(rows:[Option<SealedShared<T>>;PERSISTENT_PAGE_FANOUT],len:usize,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<(Self,RetainedCloneProgress),(ValueError,[Option<SealedShared<T>>;PERSISTENT_PAGE_FANOUT])>{
  let grant=effect.grant();let d=Self::page_birth_demand();if !fits(d,grant){return crate::RetainedWorkOutcome{result:Err((ValueError::literal(ValueRefusalKind::OwnershipLimit,"immutable leaf page birth is not funded"),rows)),work:0};}
  let mut validation_work=1;let invalid_prefix=if len==0||len>PERSISTENT_PAGE_FANOUT{true}else{let mut invalid=false;for(index,row)in rows.iter().enumerate(){validation_work+=1;if row.is_some()!=(index<len){invalid=true;break}}invalid};if invalid_prefix{return crate::RetainedWorkOutcome{result:Err((ValueError::literal(ValueRefusalKind::InvariantViolated,"immutable leaf requires an exact occupied prefix").with_retained_progress(RetainedCloneProgress{copied_items:1,..Default::default()}),rows)),work:validation_work};}
  let mut input=Some(Page::Leaf{rows,len});let mut output=None;let turn=SealedShared::admit_from(&mut input,SharedIssuer::owned(),&mut output,effect);let result=match(turn.result,output.take()){(Ok(step),Some(root))=>Ok((Self{root:Some(root),len,height:0},step.progress())),(outcome,_)=>{let error=outcome.err().unwrap_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"immutable leaf page birth is not funded"));match input.take(){Some(Page::Leaf{rows,..})=>Err((error,rows)),_=>unreachable!("immutable leaf retains original declared page")}}};crate::RetainedWorkOutcome{result,work:validation_work+turn.work}
 }
 /// 🌿️ Branch admission retains the exact original dense child prefix through actual shared allocation failure.
 pub fn admit_branch(mut children:[Option<Self>;PERSISTENT_PAGE_FANOUT],effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<(Self,RetainedCloneProgress),(ValueError,[Option<Self>;PERSISTENT_PAGE_FANOUT])>{
  let grant=effect.grant();let d=Self::page_birth_demand();if !fits(d,grant){return crate::RetainedWorkOutcome{result:Err((ValueError::literal(ValueRefusalKind::OwnershipLimit,"immutable branch page birth is not funded"),children)),work:0};}
  let mut validation_work=1;let validated:Result<(usize,u8),ValueError>=(||{let first=children[0].as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"immutable branch requires at least one child"))?;let child_height=first.height;let height=child_height.checked_add(1).filter(|height|usize::from(*height)*4<usize::BITS as usize).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"immutable page height exceeds ranked address space"))?;let full=1usize<<(usize::from(height)*4);let mut len=0usize;let mut gap=false;for child in &children{validation_work+=1;if let Some(child)=child{if gap||child.height!=child_height||child.len==0||child.len>full||child.root.is_none(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"immutable branch requires a dense prefix of matching-height nonempty children"))}len=len.checked_add(child.len).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"immutable branch cardinality overflow"))?;}else{gap=true;}}Ok((len,height))})();
  let(len,height)=match validated{Ok(value)=>value,Err(error)=>return crate::RetainedWorkOutcome{result:Err((error.with_retained_progress(RetainedCloneProgress{copied_items:1,..Default::default()}),children)),work:validation_work}};
  let roots=std::array::from_fn(|i|children[i].as_mut().and_then(|child|child.root.take()));let mut input=Some(Page::Branch{children:roots,len,height});let mut output=None;let turn=SealedShared::admit_from(&mut input,SharedIssuer::owned(),&mut output,effect);let result=match(turn.result,output.take()){(Ok(step),Some(root))=>Ok((Self{root:Some(root),len,height},step.progress())),(outcome,_)=>{let error=outcome.err().unwrap_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"immutable branch page birth is not funded"));let Some(Page::Branch{children:roots,..})=input.take()else{return crate::RetainedWorkOutcome{result:Err((ValueError::invariant("immutable branch retains original declared page"),children)),work:turn.work}};for(i,root)in roots.into_iter().enumerate(){if let Some(root)=root{let Some(child)=children[i].as_mut()else{return crate::RetainedWorkOutcome{result:Err((ValueError::invariant("original branch child wrapper"),children)),work:0}};child.root=Some(root);}}Err((error,children))}};crate::RetainedWorkOutcome{result,work:validation_work+turn.work}
 }
 /// 📏️ Quotes the native list header and its original root alias.
 pub fn projection_demand()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Option<Self>>().checked_mul(2).and_then(|copy|copy.checked_add(SealedShared::<Page<T>>::duplicate_copy_bytes())).expect("persistent projection header fits"),depth:1,..Default::default()}}
 /// ⛽️ Quotes the actual original root alias and fixed publication header.
 pub fn projection_work_demand(original:RetainedCloneRef<'_,Self>)->Result<u64,ValueError>{match original.get().root.as_ref(){Some(root)=>root.duplicate_work_demand()?.checked_add(1).ok_or_else(||ValueError::invariant("persistent projection work overflow")),None=>Ok(1)}}
 /// 🧷️ Publishes one resident immutable root under the original effect and reply.
 pub fn project_into<'r>(original:RetainedCloneRef<'_,Self>,output:&mut Option<Self>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,crate::RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,crate::RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{
  if output.is_some(){return Err(ValueError::invariant("persistent projection output occupied"))}let demand=Self::projection_demand();let work=Self::projection_work_demand(original)?;if !fits(demand,effect.grant())||effect.remaining_work_units()?<work{return Ok(reply.complete(Ok(crate::RetainedCloneStep::Progress(Default::default()))))}match effect.performed(1){Ok(true)=>{},Ok(false)=>return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::WorkLimit,"persistent projection publication exceeded Work").into()))),Err(error)=>return Ok(reply.complete(Err(error.into())))}let list=original.get();*output=Some(Self{root:None,len:list.len,height:list.height});if let Some(root)=list.root.as_ref(){let turn=root.duplicate_into(&mut output.as_mut().expect("persistent projection resident shell").root,effect);if let Err(error)=turn.result{*output=None;let mut progress=error.retained_progress();progress.copied_items=progress.copied_items.max(1);progress.copied_bytes=progress.copied_bytes.checked_add(2*size_of::<Option<Self>>()).expect("bounded persistent projection receipt");return Ok(reply.complete(Err(error.with_retained_progress(progress).into())))}if output.as_ref().is_some_and(|list|list.root.is_none()){*output=None;return Ok(reply.complete(Ok(crate::RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:2*size_of::<Option<Self>>(),..Default::default()}))))}}let copied=size_of::<Option<Self>>()+if list.root.is_some(){SealedShared::<Page<T>>::duplicate_copy_bytes()}else{0};Ok(reply.complete(Ok(crate::RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:copied,..Default::default()}))))
 }
 /// 📦️ A page birth quotes its actual Arc backing; owned row and root headers move without payload copying.
 pub fn page_birth_demand()->RetirementDemand{SealedShared::<Page<T>>::source_admission_demand().expect("persistent page admission demand fits")}
 /// 👁️ Ranked reads borrow immutable original rows through a logarithmic fixed-fanout path.
  pub fn get(&self,index:usize)->Option<&T>{self.get_with_work(index).0}
  /// 🪪️ Root identity compares immutable ownership aliases without walking their rows.
  pub fn same_root(&self,other:&Self)->bool{self.len==other.len&&self.height==other.height&&match(&self.root,&other.root){(Some(left),Some(right))=>left.identity()==right.identity(),(None,None)=>true,_=>false}}
  /// 🧮️ Ranked reads report every inspected child and the final leaf projection through one canonical walk.
  pub fn get_with_work(&self,mut index:usize)->(Option<&T>,u64){
   let mut work=0;let row=(||{if index>=self.len{return None;}let mut page=self.root.as_ref()?.get();loop{match page{
    Page::Leaf{rows,..}=>{work+=1;return rows[index].as_ref().map(SealedShared::get)},
    Page::Branch{children,..}=>{let mut selected=None;for child in children{work+=1;let child=child.as_ref()?;let len=page_len(child.get());if index<len{selected=Some(child);break;}index=index.checked_sub(len)?;}page=selected?.get();}
   }}})();(row,work)
  }
 /// 🎟️ Ranked observation prices every bounded original branch inspection and final row projection.
 pub fn rank_observation_demand(&self,index:usize)->Result<RetirementDemand,ValueError>{
  if index>=self.len{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"rank is outside the original immutable list"));}
  Ok(RetirementDemand{copy_bytes:size_of::<Option<RetainedCloneRef<'_,T>>>(),depth:usize::from(self.height)+1,..Default::default()})
 }

 /// 🧭️ A funded private page traversal lends the exact original row through its genuine root source.
 pub fn observe_rank(original:RetainedCloneRef<'_,Self>,index:usize,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<Option<(RetainedCloneRef<'_,T>,RetainedCloneProgress)>>{let mut work=0;let result=(||{
  let d=original.get().rank_observation_demand(index)?;if !fits(d,grant){return Ok(None);}
   let(row,observed)=original.get().get_with_work(index);work=u128::from(observed);let row=row.ok_or_else(||ValueError::invariant("admitted original rank row"))?;
  let pointer=row as*const T;let row=original.project(index,|_|unsafe{&*pointer});work+=1;
  Ok(Some((row,RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})))
 })();crate::RetainedWorkOutcome{result,work}}
 /// ⛽️ Bounds all original branch probes and the projected reference writes.
 pub fn rank_observation_work_demand(&self,index:usize)->Result<u64,ValueError>{self.rank_observation_demand(index)?;u64::from(self.height).checked_mul(PERSISTENT_PAGE_FANOUT as u64).and_then(|work|work.checked_add(6)).ok_or_else(||ValueError::invariant("persistent rank work overflow"))}
 /// 🧭️ Publishes one original row reference directly under its genuine effect.
 pub fn observe_rank_into<'a,'r>(original:RetainedCloneRef<'a,Self>,mut index:usize,output:&mut Option<RetainedCloneRef<'a,T>>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,crate::RetainedCloneStep,ValueError>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,crate::RetainedCloneStep,ValueError>,ValueError>{if output.is_some(){return Err(ValueError::invariant("persistent rank output occupied"))}let list=original.get();let mut d=list.rank_observation_demand(index)?;d.copy_bytes=std::mem::size_of::<Option<RetainedCloneRef<'a,T>>>();let work=list.rank_observation_work_demand(index)?;if !fits(d,effect.grant())||effect.remaining_work_units()?<work{return Ok(reply.complete(Ok(crate::RetainedCloneStep::Progress(Default::default()))))}let identity=index;let result=(||->Result<crate::RetainedCloneStep,ValueError>{let mut page=list.root.as_ref().ok_or_else(||ValueError::invariant("persistent rank root absent"))?.get();loop{match page{Page::Leaf{rows,..}=>{if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"persistent rank leaf exceeded Work"))}let row=rows[index].as_ref().ok_or_else(||ValueError::invariant("persistent rank row absent"))?.get()as*const T;if !effect.performed(5)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"persistent rank projection exceeded Work"))}*output=Some(original.project(identity,|_|unsafe{&*row}));return Ok(crate::RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))},Page::Branch{children,..}=>{let mut selected=None;for child in children{if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"persistent rank branch exceeded Work"))}let child=child.as_ref().ok_or_else(||ValueError::invariant("persistent rank dense child absent"))?;let len=page_len(child.get());if index<len{selected=Some(child);break}index=index.checked_sub(len).ok_or_else(||ValueError::invariant("persistent rank underflow"))?}page=selected.ok_or_else(||ValueError::invariant("persistent rank child absent"))?.get()}}}})();Ok(reply.complete(result))}

 /// 🫙️ True when the original root pages already moved to a funded retirement owner.
 /// ⛽️ Quotes the sealed alias work of projecting this root; an empty list projects with one unit.
 pub(crate) fn root_projection_work(&self)->Result<u64,ValueError>{match self.root.as_ref(){Some(root)=>root.duplicate_work_demand(),None=>Ok(1)}}
 pub(crate) fn root_transferred(&self)->bool{self.root.as_ref().is_some_and(|root|root.terminal_is_empty())}
 pub fn len(&self)->usize{self.len}
 pub fn is_empty(&self)->bool{self.len==0}
}

/// 🧳️ One immutable replacement stages fixed page paths while keeping exact original list and row sources.
pub struct PersistentSet<T:RetireOwned+Send+Sync+'static>{
 original:Option<crate::retained_clone::RetainedCloneBinding>,
 authored:Option<crate::retained_clone::RetainedCloneBinding>,
 page:std::mem::ManuallyDrop<Option<Page<T>>>,
 candidate:std::mem::ManuallyDrop<Option<SealedShared<Page<T>>>>,
 retirement:crate::retained_clone::RetainedCloneClose,
 index:usize,len:usize,height:u8,level:u8,slot:usize,phase:u8,closing:bool,
}
impl<T:RetireOwned+Send+Sync+'static> PersistentSet<T>{
 /// 🖱️ The fixed cursor has no heap birth and captures neither row nor original root.
 pub fn new(index:usize)->Self{Self{original:None,authored:None,page:std::mem::ManuallyDrop::new(None),candidate:std::mem::ManuallyDrop::new(None),retirement:Default::default(),index,len:0,height:0,level:0,slot:0,phase:0,closing:false}}
 fn page_at_paid<'a>(&self,list:&'a PersistentList<T>,effect:&mut crate::RetainedAllocationEffect<'_>)->Result<(&'a Page<T>,usize),ValueError>{if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"persistent set root probe exceeded Work"))}let mut page=list.root.as_ref().ok_or_else(||ValueError::invariant("persistent set root absent"))?.get();let mut height=list.height;let mut rank=self.index;while height>self.level{let Page::Branch{children,..}=page else{return Err(ValueError::invariant("persistent set branch changed"))};let(slot,relative)=child_rank_paid(children,rank,false,effect)?.ok_or_else(||ValueError::invariant("persistent set rank absent"))?;rank=relative;page=children[slot].as_ref().ok_or_else(||ValueError::invariant("persistent set child absent"))?.get();height-=1}Ok((page,rank))}
 /// 📏️ Quotes both bound inputs and the actual selected native page frontier.
 pub fn demand(&self,original:RetainedCloneRef<'_,PersistentList<T>>,authored:RetainedCloneRef<'_,SealedShared<T>>)->Result<RetirementDemand,ValueError>{if self.closing||self.phase>4{return Err(ValueError::invariant("persistent set is closing or transferred"))}if let Some(d)=original.binding_demand(&self.original)?{return Ok(d)}if let Some(d)=authored.binding_demand(&self.authored)?{return Ok(d)}if self.output_ready(){return Ok(Default::default())}if self.phase==3&&self.page.is_some(){return SealedShared::<Page<T>>::source_admission_demand()}let mut copy=std::mem::size_of::<Option<Page<T>>>()+std::mem::size_of::<Option<SealedShared<Page<T>>>>()+2*std::mem::size_of::<usize>()+2*std::mem::size_of::<u8>();if self.phase==2{copy=copy.checked_add(SealedShared::<Page<T>>::duplicate_copy_bytes().max(SealedShared::<T>::duplicate_copy_bytes())).ok_or_else(||ValueError::invariant("persistent set quote overflow"))?}Ok(RetirementDemand{copy_bytes:copy,depth:1,..Default::default()})}
 pub fn advance_work_demand(&self,original:RetainedCloneRef<'_,PersistentList<T>>,authored:RetainedCloneRef<'_,SealedShared<T>>)->Result<u64,ValueError>{original.binding_demand(&self.original)?;authored.binding_demand(&self.authored)?;if self.original.is_none(){return original.binding_work_demand(&self.original)}if self.authored.is_none(){return authored.binding_work_demand(&self.authored)}if self.output_ready(){return Ok(0)}let child=if self.phase==3&&self.page.is_some(){SealedShared::<Page<T>>::source_admission_work_demand()}else if self.phase==1{2+PERSISTENT_PAGE_FANOUT as u64*u64::from(original.get().height)}else if self.phase==2{2+PERSISTENT_PAGE_FANOUT as u64*(u64::from(original.get().height)+1)+SealedShared::<Page<T>>::DUPLICATE_WORK}else{1};RetainedCloneRef::<PersistentList<T>>::check_binding_work(&self.original).checked_add(RetainedCloneRef::<SealedShared<T>>::check_binding_work(&self.authored)).and_then(|work|work.checked_add(child)).ok_or_else(||ValueError::invariant("persistent set work quote overflow"))}
 /// 🧵️ Each selected page or alias mutation records into the original borrowed reply.
 pub fn advance_into<'r>(&mut self,original:RetainedCloneRef<'_,PersistentList<T>>,authored:RetainedCloneRef<'_,SealedShared<T>>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,crate::RetainedCloneStep,ValueError>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,crate::RetainedCloneStep,ValueError>,ValueError>{use crate::RetainedCloneStep;let d=self.demand(original,authored)?;if self.output_ready(){return Ok(reply.complete(Ok(RetainedCloneStep::Complete(Default::default()))))}let work=self.advance_work_demand(original,authored)?;if !fits(d,effect.grant())||effect.remaining_work_units()?<work{return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}if self.original.is_none(){return Ok(reply.complete(original.bind(&mut self.original,effect).result.map(|p|RetainedCloneStep::Progress(p.unwrap_or_default()))))}if self.authored.is_none(){return Ok(reply.complete(authored.bind(&mut self.authored,effect).result.map(|p|RetainedCloneStep::Progress(p.unwrap_or_default()))))}for checked in [0,1]{let checked=if checked==0{original.check_binding_paid(&self.original,effect)}else{authored.check_binding_paid(&self.authored,effect)};match checked.result{Ok(Some(true))=>{},Ok(_)=>return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default())))),Err(error)=>return Ok(reply.complete(Err(error)))}}if self.phase==3&&self.page.is_some(){return Ok(reply.complete(SealedShared::admit_from(&mut self.page,SharedIssuer::owned(),&mut self.candidate,effect).result.map(|step|RetainedCloneStep::Progress(step.progress()))))}let mut copied=0usize;let result=(||->Result<RetainedCloneStep,ValueError>{match self.phase{0=>{if self.index>=original.get().len{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"persistent replacement index is outside the original"))}if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"persistent set metadata exceeded Work"))}self.len=original.get().len;self.height=original.get().height;self.phase=1;copied=std::mem::size_of::<usize>()+2*std::mem::size_of::<u8>();},1=>{let page=match self.page_at_paid(original.get(),effect)?.0{Page::Leaf{len,..}=>Page::Leaf{rows:std::array::from_fn(|_|None),len:*len},Page::Branch{len,height,..}=>Page::Branch{children:std::array::from_fn(|_|None),len:*len,height:*height}};if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"persistent page plan exceeded Work"))}*self.page=Some(page);self.slot=0;self.phase=2;copied=std::mem::size_of::<Option<Page<T>>>()+std::mem::size_of::<usize>()+std::mem::size_of::<u8>();},2=>{let mut metadata_done=false;let(original_page,rank)=self.page_at_paid(original.get(),effect)?;let target=match original_page{Page::Leaf{..}=>rank,Page::Branch{children,..}=>child_rank_paid(children,rank,false,effect)?.ok_or_else(||ValueError::invariant("persistent changed child rank absent"))?.0};match original_page{Page::Leaf{rows,..}=>{let source=if self.slot==target{Some(authored.get())}else{rows[self.slot].as_ref()};let Some(Page::Leaf{rows:destination,..})=self.page.as_mut()else{return Err(ValueError::invariant("persistent leaf plan absent"))};if let Some(source)=source{let turn=source.duplicate_into(&mut destination[self.slot],effect);let step=turn.result?;if destination[self.slot].is_none(){return Ok(RetainedCloneStep::Progress(step.progress()))}copied+=step.progress().copied_bytes}},Page::Branch{children,..}=>{let Some(Page::Branch{children:destination,..})=self.page.as_mut()else{return Err(ValueError::invariant("persistent branch plan absent"))};if self.slot==target{if self.candidate.is_none(){return Err(ValueError::invariant("persistent changed child absent"))}if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"persistent changed child exceeded Work"))}metadata_done=true;destination[self.slot]=self.candidate.take();copied+=2*std::mem::size_of::<Option<SealedShared<Page<T>>>>()}else if let Some(source)=children[self.slot].as_ref(){let turn=source.duplicate_into(&mut destination[self.slot],effect);let step=turn.result?;if destination[self.slot].is_none(){return Ok(RetainedCloneStep::Progress(step.progress()))}copied+=step.progress().copied_bytes}}}if !metadata_done&&!effect.performed(1)?{self.slot+=1;if self.slot==PERSISTENT_PAGE_FANOUT{self.phase=3}return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"persistent page slot exceeded Work"))}self.slot+=1;copied+=std::mem::size_of::<usize>();if self.slot==PERSISTENT_PAGE_FANOUT{self.phase=3;copied+=std::mem::size_of::<u8>()}},3=>{if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"persistent page level exceeded Work"))}self.level+=1;self.phase=if self.level>self.height{4}else{1};copied=2*std::mem::size_of::<u8>()},4=>return Ok(RetainedCloneStep::Complete(Default::default())),_=>return Err(ValueError::invariant("persistent set selected phase changed"))}let p=RetainedCloneProgress{copied_items:1,copied_bytes:copied,..Default::default()};Ok(if self.output_ready(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})})();Ok(reply.complete(result))}
 /// 📗️ Preparation readiness is independent of zero-valued numeric work quotes.
 pub fn output_ready(&self)->bool{!self.closing&&self.phase==4&&self.candidate.is_some()}
 /// 📤️ Quotes the actual resident candidate and caller output headers.
 pub fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.output_ready(){return Err(ValueError::invariant("persistent set candidate is not ready"))}Ok(RetirementDemand{copy_bytes:std::mem::size_of::<Option<SealedShared<Page<T>>>>()+std::mem::size_of::<Option<PersistentList<T>>>()+std::mem::size_of::<u8>(),depth:1,..Default::default()})}
 pub fn take_work_demand(&self)->Result<u64,ValueError>{self.take_demand()?;Ok(1)}
 /// 🧳️ Publishes the original candidate directly into the caller-resident slot.
 pub fn take_into<'r>(&mut self,output:&mut Option<PersistentList<T>>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,crate::RetainedCloneStep,ValueError>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,crate::RetainedCloneStep,ValueError>,ValueError>{if output.is_some(){return Err(ValueError::invariant("persistent set output occupied"))}let d=self.take_demand()?;if !fits(d,effect.grant())||effect.remaining_work_units()?<1{return Ok(reply.complete(Ok(crate::RetainedCloneStep::Progress(Default::default()))))}match effect.performed(1){Ok(true)=>{},Ok(false)=>return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::WorkLimit,"persistent set publication exceeded Work")))),Err(error)=>return Ok(reply.complete(Err(error)))}*output=Some(PersistentList{root:self.candidate.take(),len:self.len,height:self.height});self.phase=5;Ok(reply.complete(Ok(crate::RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))))}
 /// 🛑️ Closure starts under the actual original effect.
 pub fn is_closing(&self)->bool{self.closing}
 pub fn begin_close_demand(&self)->RetirementDemand{if self.closing{Default::default()}else{RetirementDemand{copy_bytes:std::mem::size_of::<bool>(),depth:1,..Default::default()}}}
 pub fn begin_close_work_demand(&self)->u64{u64::from(!self.closing)}
 pub fn begin_close_paid(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome{if self.closing{return crate::RetainedWorkOutcome{result:Ok(crate::RetainedCloneStep::Complete(Default::default())),work:0}}let d=self.begin_close_demand();let remaining=match effect.remaining_work_units(){Ok(value)=>value,Err(error)=>return crate::RetainedWorkOutcome{result:Err(error),work:0}};if remaining<1||!fits(d,effect.grant()){return crate::RetainedWorkOutcome{result:Ok(crate::RetainedCloneStep::Progress(Default::default())),work:0}}let before=effect.performed_work();let result=match effect.performed(1){Ok(true)=>{self.closing=true;Ok(crate::RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))},Ok(false)=>Err(ValueError::literal(ValueRefusalKind::WorkLimit,"persistent set close start exceeded Work")),Err(error)=>Err(error)};crate::RetainedWorkOutcome{result,work:effect.performed_work()-before}}
 /// ♻️ Quotes the actual selected retained owner or source binding.
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<Page<T>>(false,&None,body)}if self.page.is_some(){return self.retirement.next_owner_demands::<Page<T>>(true,&None,body)}if self.candidate.is_some(){return self.retirement.next_owner_demands::<SealedShared<Page<T>>>(true,&None,body)}crate::retained_clone::RetainedCloneBinding::close_demand(if self.authored.is_some(){&self.authored}else{&self.original},body)}
 pub fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if !self.closing{return Err(ValueError::invariant("persistent set close not started"))}if !self.retirement.is_empty(){return self.retirement.next_owner_work::<Page<T>>(false,&None,body)}if self.page.is_some(){return self.retirement.next_owner_work::<Page<T>>(true,&None,body)}if self.candidate.is_some(){return self.retirement.next_owner_work::<SealedShared<Page<T>>>(true,&None,body)}crate::retained_clone::RetainedCloneBinding::close_work_demand(if self.authored.is_some(){&self.authored}else{&self.original},body)}
 pub fn requires_original_birth(&self)->bool{if !self.retirement.is_empty(){self.retirement.requires_original_birth()}else if self.page.is_some()||self.candidate.is_some(){false}else{crate::retained_clone::RetainedCloneBinding::requires_original_birth(if self.authored.is_some(){&self.authored}else{&self.original})}}
 pub fn close_original_birth_step(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->semio_framework_value_retirement::OriginalRetirementPoll{if !self.requires_original_birth(){return semio_framework_value_control_contract::RetainedWorkPoll::Complete(crate::RetainedWorkOutcome{result:Err(ValueError::invariant("persistent set original Birth has no selected child").into()),work:0})}if self.retirement.requires_original_birth(){self.retirement.step_original_birth(original,grant)}else{crate::retained_clone::RetainedCloneBinding::close_original_birth_step(if self.authored.is_some(){&mut self.authored}else{&mut self.original},original,grant)}}
 pub fn requires_original_control(&self)->bool{if !self.retirement.is_empty(){self.retirement.requires_original_control()}else if self.page.is_some()||self.candidate.is_some(){false}else{crate::retained_clone::RetainedCloneBinding::requires_original_control(if self.authored.is_some(){&self.authored}else{&self.original})}}
 /// 🪜️ The selected child records directly into the same borrowed reply.
 pub fn close_into<'r>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,crate::RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,crate::RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{if !self.closing{return Err(ValueError::invariant("persistent set close not started"))}if self.terminal_is_empty(){return Ok(reply.complete(Ok(crate::RetainedCloneStep::Complete(Default::default()))))}if !self.retirement.is_empty(){return self.retirement.step_into(body,effect,reply.progress())}let d=self.close_demand(body)?;let work=self.close_work_demand(body)?;if !fits(d,effect.grant())||effect.remaining_work_units()?<work{return Ok(reply.complete(Ok(crate::RetainedCloneStep::Progress(Default::default()))))}if self.page.is_some(){let turn=self.retirement.begin_granted(&mut self.page,effect);return Ok(reply.complete(turn.result.map(|step|step.unwrap_or(crate::RetainedCloneStep::Progress(Default::default()))).map_err(Into::into)))}if self.candidate.is_some(){let turn=self.retirement.begin_granted(&mut self.candidate,effect);return Ok(reply.complete(turn.result.map(|step|step.unwrap_or(crate::RetainedCloneStep::Progress(Default::default()))).map_err(Into::into)))}crate::retained_clone::RetainedCloneBinding::close_into(if self.authored.is_some(){&mut self.authored}else{&mut self.original},body,effect,reply.progress())}
 pub fn recover_close_into<'r>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,crate::RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,crate::RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{if !self.closing{return Err(ValueError::invariant("persistent set recovery not started"))}if !self.retirement.is_empty(){self.retirement.recover_step_into(body,effect,reply.progress())}else{crate::retained_clone::RetainedCloneBinding::recover_close_into(if self.authored.is_some(){&mut self.authored}else{&mut self.original},body,effect,reply.progress())}}
 pub fn close_original_into<'r>(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,crate::RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,crate::RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{if !self.closing{return Err(ValueError::invariant("persistent set original close not started"))}if !self.retirement.is_empty(){self.retirement.step_original_into(original,body,effect,reply.progress())}else if self.page.is_some()||self.candidate.is_some(){Err(ValueError::invariant("persistent set original close has no selected child"))}else{crate::retained_clone::RetainedCloneBinding::close_original_into(if self.authored.is_some(){&mut self.authored}else{&mut self.original},original,body,effect,reply.progress())}}
 pub fn recover_close_original_into<'r>(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,crate::RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,crate::RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{if !self.closing{return Err(ValueError::invariant("persistent set original recovery not started"))}if !self.retirement.is_empty(){self.retirement.recover_step_original_into(original,body,effect,reply.progress())}else if self.page.is_some()||self.candidate.is_some(){Err(ValueError::invariant("persistent set original recovery has no selected child"))}else{crate::retained_clone::RetainedCloneBinding::recover_close_original_into(if self.authored.is_some(){&mut self.authored}else{&mut self.original},original,body,effect,reply.progress())}}
 pub fn terminal_is_empty(&self)->bool{self.page.is_none()&&self.candidate.is_none()&&self.retirement.is_empty()&&self.original.is_none()&&self.authored.is_none()}
}
impl<T:RetireOwned+Send+Sync+'static> Drop for PersistentSet<T>{fn drop(&mut self){let empty=self.terminal_is_empty();assert!(empty||std::thread::panicking(),"immutable candidate retains original pages and issuer bindings until funded closure");if empty{unsafe{std::mem::ManuallyDrop::drop(&mut self.page);std::mem::ManuallyDrop::drop(&mut self.candidate);}}}}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
pub(crate) mod tests;

#[cfg(test)]
#[path="📥️build/🧪️tests/🦀️.rs"]
mod build_tests;



#[cfg(test)]
#[path="🔁️codec/🧪️tests/🦀️.rs"]
mod codec_tests;
