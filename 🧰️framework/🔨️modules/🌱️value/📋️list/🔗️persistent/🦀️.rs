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
fn child_rank<T:RetireOwned+Send+Sync+'static>(children:&[Option<SealedShared<Page<T>>>;PERSISTENT_PAGE_FANOUT],mut index:usize,insertion:bool)->Option<(usize,usize)>{let mut last=None;for(slot,child)in children.iter().enumerate(){let Some(child)=child else{break;};let len=page_len(child.get());if index<len{return Some((slot,index));}index=index.checked_sub(len)?;last=Some((slot,len));}if insertion&&index==0{last}else{None}}
#[path="🎮️splice/🦀️.rs"] pub mod splice;
pub use splice::{PersistentSplice,PersistentSpliceKind};
#[path="🔎️traversal/🦀️.rs"] mod traversal;
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

/// 🔗️ Cloning shares the immutable page root as one more alias; every live alias must be explicitly retired, exactly like `OrderedMap`.
impl<T:RetireOwned+Send+Sync+'static> Clone for PersistentList<T>{fn clone(&self)->Self{Self{root:self.root.clone(),len:self.len,height:self.height}}}
impl<T:RetireOwned+Send+Sync+'static> Default for PersistentList<T>{fn default()->Self{Self{root:None,len:0,height:0}}}
fn fits(d:RetirementDemand,g:RetainedCloneGrant)->bool{g.maximum_items>0&&g.maximum_copy_bytes>=d.copy_bytes&&g.maximum_capacity_bytes>=d.capacity_bytes&&g.maximum_release_bytes>=d.release_bytes&&g.maximum_depth>=d.depth}
impl<T:RetireOwned+Send+Sync+'static> PersistentList<T>{
 /// 📏️ Root projection duplicates one fixed original authority without inspecting or cloning rows.
 pub fn projection_demand()->RetirementDemand{RetirementDemand{depth:1,..Default::default()}}
 /// 🧷️ A genuine original list source lends an independent immutable candidate root.
 pub fn project(original:RetainedCloneRef<'_,Self>,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<Option<(Self,RetainedCloneProgress)>,ValueError>{let mut work=0;let result=(||{
  let demand=Self::projection_demand();if !fits(demand,grant){return Ok(None);}
  let original=original.get();
  let(root,progress)=if let Some(root)=original.root.as_ref(){let turn=root.try_duplicate(grant);work=turn.work;let(root,p)=turn.result?;(Some(root),p)}else{work=1;(None,RetainedCloneProgress{copied_items:1,..Default::default()})};
  Ok(Some((Self{root,len:original.len,height:original.height},progress)))
 })();crate::RetainedWorkOutcome{result,work}}
 /// 📦️ A page birth quotes its actual Arc backing; owned row and root headers move without payload copying.
 pub fn page_birth_demand()->RetirementDemand{RetirementDemand{capacity_bytes:SealedShared::<Page<T>>::birth_bytes(),depth:1,..Default::default()}}
 /// 🌱️ Leaf admission preserves every owned original row on denial or real allocation refusal.
 pub fn admit_leaf(rows:[Option<SealedShared<T>>;PERSISTENT_PAGE_FANOUT],len:usize,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<(Self,RetainedCloneProgress),(ValueError,[Option<SealedShared<T>>;PERSISTENT_PAGE_FANOUT])>{
  let d=Self::page_birth_demand();if !fits(d,grant){return crate::RetainedWorkOutcome{result:Err((ValueError::literal(ValueRefusalKind::OwnershipLimit,"immutable leaf page birth is not funded"),rows)),work:0};}
  if len==0||len>PERSISTENT_PAGE_FANOUT||rows.iter().enumerate().any(|(i,row)|row.is_some()!=(i<len)){return crate::RetainedWorkOutcome{result:Err((ValueError::literal(ValueRefusalKind::InvariantViolated,"immutable leaf requires an exact occupied prefix").with_retained_progress(RetainedCloneProgress{copied_items:1,..Default::default()}),rows)),work:1};}
  let turn=SealedShared::admit(Page::Leaf{rows,len},SharedIssuer::owned(),grant);let result=match turn.result{Ok((root,p))=>Ok((Self{root:Some(root),len,height:0},p)),Err((error,Page::Leaf{rows,..}))=>Err((error,rows)),Err(_)=>unreachable!("immutable leaf retains original declared page")};crate::RetainedWorkOutcome{result,work:turn.work}
 }
 /// 🌿️ Branch admission retains the exact original dense child prefix through actual shared allocation failure.
 pub fn admit_branch(mut children:[Option<Self>;PERSISTENT_PAGE_FANOUT],grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<(Self,RetainedCloneProgress),(ValueError,[Option<Self>;PERSISTENT_PAGE_FANOUT])>{
  let d=Self::page_birth_demand();if !fits(d,grant){return crate::RetainedWorkOutcome{result:Err((ValueError::literal(ValueRefusalKind::OwnershipLimit,"immutable branch page birth is not funded"),children)),work:0};}
  let validated:Result<(usize,u8),ValueError>=(||{let first=children[0].as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"immutable branch requires at least one child"))?;let child_height=first.height;let height=child_height.checked_add(1).filter(|height|usize::from(*height)*4<usize::BITS as usize).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"immutable page height exceeds ranked address space"))?;let full=1usize<<(usize::from(height)*4);let mut len=0usize;let mut gap=false;for child in &children{if let Some(child)=child{if gap||child.height!=child_height||child.len==0||child.len>full||child.root.is_none(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"immutable branch requires a dense prefix of matching-height nonempty children"))}len=len.checked_add(child.len).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"immutable branch cardinality overflow"))?;}else{gap=true;}}Ok((len,height))})();
  let(len,height)=match validated{Ok(value)=>value,Err(error)=>return crate::RetainedWorkOutcome{result:Err((error.with_retained_progress(RetainedCloneProgress{copied_items:1,..Default::default()}),children)),work:1}};
  let roots=std::array::from_fn(|i|children[i].as_mut().and_then(|child|child.root.take()));let turn=SealedShared::admit(Page::Branch{children:roots,len,height},SharedIssuer::owned(),grant);let result=match turn.result{Ok((root,p))=>Ok((Self{root:Some(root),len,height},p)),Err((error,Page::Branch{children:roots,..}))=>{for(i,root)in roots.into_iter().enumerate(){if let Some(root)=root{let Some(child)=children[i].as_mut()else{return crate::RetainedWorkOutcome{result:Err((ValueError::invariant("original branch child wrapper"),children)),work:0}};child.root=Some(root);}}Err((error,children))},Err(_)=>return crate::RetainedWorkOutcome{result:Err((ValueError::invariant("immutable branch retains original declared page"),children)),work:turn.work}};crate::RetainedWorkOutcome{result,work:turn.work}
 }
 /// 👁️ Ranked reads borrow immutable original rows through a logarithmic fixed-fanout path.
 pub fn get(&self,mut index:usize)->Option<&T>{if index>=self.len{return None;}let mut page=self.root.as_ref()?.get();loop{match page{Page::Leaf{rows,..}=>return rows[index].as_ref().map(SealedShared::get),Page::Branch{children,..}=>{let(slot,relative)=child_rank(children,index,false)?;index=relative;page=children[slot].as_ref()?.get();}}}} /// 🧮️ Cardinality is fixed metadata, independent of page count and payload size.
 /// 🎟️ Ranked observation prices every bounded original branch inspection and final row projection.
 pub fn rank_observation_demand(&self,index:usize)->Result<RetirementDemand,ValueError>{
  if index>=self.len{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"rank is outside the original immutable list"));}
  Ok(RetirementDemand{depth:usize::from(self.height)+1,..Default::default()})
 }

 /// 🧭️ A funded private page traversal lends the exact original row through its genuine root source.
 pub fn observe_rank(original:RetainedCloneRef<'_,Self>,index:usize,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<Option<(RetainedCloneRef<'_,T>,RetainedCloneProgress)>>{let mut work=0;let result=(||{
  let d=original.get().rank_observation_demand(index)?;if !fits(d,grant){return Ok(None);}
  let mut relative=index;let mut page=original.get().root.as_ref().expect("admitted original rank root").get();
  let row=loop{match page{
   Page::Leaf{rows,..}=>{work+=1;break rows[relative].as_ref().ok_or_else(||ValueError::invariant("admitted original rank row"))?.get()},
   Page::Branch{children,..}=>{let mut selected=None;for child in children{work+=1;let Some(child)=child else{break};let len=page_len(child.get());if relative<len{selected=Some(child);break}relative=relative.checked_sub(len).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original ranked page extent underflow"))?;}page=selected.ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original ranked page lost its child"))?.get();}
  }};
  let pointer=row as*const T;let row=original.project(index,|_|unsafe{&*pointer});work+=1;
  Ok(Some((row,RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})))
 })();crate::RetainedWorkOutcome{result,work}}

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
 fn page_at<'a>(&self,list:&'a PersistentList<T>)->Result<(&'a Page<T>,usize),ValueError>{
  let mut page=list.root.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"immutable candidate lost its original root"))?.get();let mut height=list.height;let mut rank=self.index;
  while height>self.level{let Page::Branch{children,..}=page else{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"immutable original branch shape changed"));};let(slot,relative)=child_rank(children,rank,false).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"immutable original path has no child"))?;rank=relative;page=children[slot].as_ref().unwrap().get();height-=1;}Ok((page,rank))
 } /// 📏️ Ranked probes and owned header movement copy no payload; actual page birth quotes its Arc backing.
 pub fn demand(&self,original:RetainedCloneRef<'_,PersistentList<T>>,authored:RetainedCloneRef<'_,SealedShared<T>>)->Result<RetirementDemand,ValueError>{if self.closing||self.phase>4{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"immutable page preparation is closing or transferred"))}if self.original.is_none(){return Ok(RetirementDemand{copy_bytes:original.binding_copy_bytes(),depth:1,..Default::default()})}if self.authored.is_none(){return Ok(RetirementDemand{copy_bytes:authored.binding_copy_bytes(),depth:1,..Default::default()})}Ok(RetirementDemand{capacity_bytes:if self.phase==3&&self.page.is_some(){SealedShared::<Page<T>>::birth_bytes()}else{0},depth:1,..Default::default()})}
 /// 🧵️ Each actual source alias, fixed ranked probe and changed page allocation exposes its own outcome.
 pub fn advance(&mut self,original:RetainedCloneRef<'_,PersistentList<T>>,authored:RetainedCloneRef<'_,SealedShared<T>>,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome{let mut work=0;let result=(||{use crate::retained_clone::RetainedCloneStep;if self.output_ready(){return Ok(RetainedCloneStep::Complete(Default::default()))}let d=self.demand(original,authored)?;if !fits(d,grant){return Ok(RetainedCloneStep::Progress(Default::default()))}let turn=original.bind(&mut self.original,grant);work=turn.work;if let Some(p)=turn.result?{return Ok(RetainedCloneStep::Progress(p))}let turn=authored.bind(&mut self.authored,grant);work=turn.work;if let Some(p)=turn.result?{return Ok(RetainedCloneStep::Progress(p))}work=1;let mut p=RetainedCloneProgress{copied_items:1,..Default::default()};match self.phase{
 0=>{self.len=original.get().len;self.height=original.get().height;if self.index>=self.len{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"immutable replacement index is outside the original collection").with_retained_progress(p))}self.phase=1;},
 1=>{let page=match self.page_at(original.get())?.0{Page::Leaf{len,..}=>Page::Leaf{rows:std::array::from_fn(|_|None),len:*len},Page::Branch{len,height,..}=>Page::Branch{children:std::array::from_fn(|_|None),len:*len,height:*height}};*self.page=Some(page);self.slot=0;self.phase=2;},
 2=>{let(original_page,rank)=self.page_at(original.get())?;let target=match original_page{Page::Leaf{..}=>rank,Page::Branch{children,..}=>child_rank(children,rank,false).expect("original changed child rank").0};match original_page{
 Page::Leaf{rows,..}=>{let source=if self.slot==target{Some(authored.get())}else{rows[self.slot].as_ref()};let alias=if let Some(source)=source{let turn=source.try_duplicate(grant);work=turn.work;let(alias,receipt)=turn.result?;p=receipt;Some(alias)}else{None};let Some(Page::Leaf{rows,..})=self.page.as_mut()else{unreachable!("original immutable leaf plan")};rows[self.slot]=alias;},
 Page::Branch{children,..}=>{let alias=if self.slot==target{Some(self.candidate.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"immutable changed child is absent").with_retained_progress(p))?)}else if let Some(source)=children[self.slot].as_ref(){let turn=source.try_duplicate(grant);work=turn.work;let(alias,receipt)=turn.result?;p=receipt;Some(alias)}else{None};let Some(Page::Branch{children,..})=self.page.as_mut()else{unreachable!("original immutable branch plan")};children[self.slot]=alias;}}
 self.slot+=1;if self.slot==PERSISTENT_PAGE_FANOUT{self.phase=3;}},
 3=>{if let Some(page)=self.page.take(){let turn=SealedShared::admit(page,SharedIssuer::owned(),grant);work=turn.work;return match turn.result{Ok((candidate,p))=>{*self.candidate=Some(candidate);Ok(RetainedCloneStep::Progress(p))},Err((error,page))=>{*self.page=Some(page);Err(error)}}}self.level+=1;self.phase=if self.level>self.height{4}else{1};},
 4=>return Ok(RetainedCloneStep::Complete(Default::default())),_=>unreachable!("quoted immutable path phase")}
 Ok(if self.output_ready(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})})();crate::RetainedWorkOutcome{result,work}}
 /// 📗️ Preparation readiness is independent of zero-valued numeric work quotes.
 pub fn output_ready(&self)->bool{!self.closing&&self.phase==4&&self.candidate.is_some()}
 /// 📤️ A funded fixed-header transfer yields the immutable prepared candidate root once.
 pub fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.output_ready(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"immutable candidate root is not ready"));}Ok(RetirementDemand{depth:1,..Default::default()})}
 pub fn take(&mut self,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<Option<(PersistentList<T>,RetainedCloneProgress)>>{let mut work=0;let result=(||{
  if !self.output_ready(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"immutable candidate root is not ready"));}
  let d=self.take_demand()?;if !fits(d,grant){return Ok(None);}work=1;
  let root=self.candidate.take();self.phase=5;Ok(Some((PersistentList{root,len:self.len,height:self.height},RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})))
 })();crate::RetainedWorkOutcome{result,work}}
 /// 🛑️ Cancellation never changes original published roots.
 pub fn begin_close(&mut self){self.closing=true;}
 /// ♻️ Every staged page and genuine source alias keeps its original typed retirement authority.
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<Page<T>>(false,&None,body);}
  if self.page.is_some(){return self.retirement.next_owner_demands::<Page<T>>(true,&None,body);}
  if self.candidate.is_some(){return self.retirement.next_owner_demands::<SealedShared<Page<T>>>(true,&None,body);}
  let binding=if self.authored.is_some(){&self.authored}else{&self.original};crate::retained_clone::RetainedCloneBinding::close_demand(binding,body)
 }
 /// 🪜️ Only completely settled pages and source bindings permit terminal cursor release.
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome{let mut work=0;let result=(||{use crate::retained_clone::{RetainedCloneStep,RetainedCloneBinding};if !self.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"immutable candidate closure not started"))}if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}let d=self.close_demand(grant.maximum_copy_bytes)?;if !fits(d,grant){return Ok(RetainedCloneStep::Progress(Default::default()))}if !self.retirement.is_empty(){let turn=self.retirement.step_granted(grant);work=turn.work;return turn.result.map(|s|RetainedCloneStep::Progress(s.progress()))}if self.page.is_some(){let turn=self.retirement.begin_granted(&mut self.page,grant);work=turn.work;return turn.result.map(|s|s.unwrap_or(RetainedCloneStep::Progress(Default::default())))}if self.candidate.is_some(){let turn=self.retirement.begin_granted(&mut self.candidate,grant);work=turn.work;return turn.result.map(|s|s.unwrap_or(RetainedCloneStep::Progress(Default::default())))}let turn=RetainedCloneBinding::close_one(if self.authored.is_some(){&mut self.authored}else{&mut self.original},grant);work=turn.work;let step=turn.result?;Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())})})();crate::RetainedWorkOutcome{result,work}}
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
#[path="🎟️update/🧪️tests/🦀️.rs"]
mod update_work_tests;
