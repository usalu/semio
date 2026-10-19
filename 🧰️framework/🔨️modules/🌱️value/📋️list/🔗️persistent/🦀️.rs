//! 🔗️ Immutable shared fixed-fanout pages retain original readers while candidates replace only changed paths.
use crate::{RetirementDemand,ValueError,ValueRefusalKind,retirement::RetireOwned,shared_issuance::{SealedShared,SharedIssuer},retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneRef}};
use std::mem::size_of;
/// 🌳️ Every branch and payload leaf has a fixed bounded page footprint.
pub const PERSISTENT_PAGE_FANOUT:usize=16;
#[derive(crate::RetireOwned)]
enum Page<T:Send+Sync+'static>{
 Leaf{rows:[Option<SealedShared<T>>;PERSISTENT_PAGE_FANOUT],len:usize},
 Branch{children:[Option<SealedShared<Page<T>>>;PERSISTENT_PAGE_FANOUT],len:usize,height:u8},
}
fn page_len<T:Send+Sync+'static>(page:&Page<T>)->usize{match page{Page::Leaf{len,..}|Page::Branch{len,..}=>*len}}
fn child_rank<T:Send+Sync+'static>(children:&[Option<SealedShared<Page<T>>>;PERSISTENT_PAGE_FANOUT],mut index:usize,insertion:bool)->Option<(usize,usize)>{let mut last=None;for(slot,child)in children.iter().enumerate(){let Some(child)=child else{break;};let len=page_len(child.get());if index<len{return Some((slot,index));}index=index.checked_sub(len)?;last=Some((slot,len));}if insertion&&index==0{last}else{None}}
#[path="🎮️splice/🦀️.rs"] pub mod splice;
pub use splice::{PersistentSplice,PersistentSpliceKind};
#[path="🔎️traversal/🦀️.rs"] mod traversal;
pub use traversal::PersistentListIter;
#[path="🧳️read/🦀️.rs"] mod retained_read;
pub use retained_read::{PersistentListRead,PersistentListReadStep};
#[path="📥️build/🦀️.rs"] mod retained_build;
pub use retained_build::PersistentListBuilder;
/// 🪪️ Immutable page roots preserve original typed row issuers and every old reader.
#[derive(crate::RetireOwned)]
pub struct PersistentList<T:Send+Sync+'static>{root:Option<SealedShared<Page<T>>>,len:usize,height:u8}
impl<T:std::fmt::Debug+Send+Sync+'static> std::fmt::Debug for PersistentList<T>{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{f.debug_list().entries(self.iter()).finish()}}
impl<T:PartialEq+Send+Sync+'static> PartialEq for PersistentList<T>{fn eq(&self,other:&Self)->bool{self.len==other.len&&self.iter().eq(other.iter())}}
impl<T:Eq+Send+Sync+'static> Eq for PersistentList<T>{}

impl<T:Send+Sync+'static> Default for PersistentList<T>{fn default()->Self{Self{root:None,len:0,height:0}}}
fn fits(d:RetirementDemand,g:RetainedCloneGrant)->bool{g.maximum_items>0&&g.maximum_copy_bytes>=d.copy_bytes&&g.maximum_capacity_bytes>=d.capacity_bytes&&g.maximum_release_bytes>=d.release_bytes&&g.maximum_depth>=d.depth}
impl<T:Send+Sync+'static> PersistentList<T>{
 /// 📏️ Root projection duplicates one fixed original authority without inspecting or cloning rows.
 pub fn projection_demand()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Self>()*2+size_of::<SealedShared<Page<T>>>(),depth:1,..Default::default()}}
 /// 🧷️ A genuine original list source lends an independent immutable candidate root.
 pub fn project(original:RetainedCloneRef<'_,Self>,grant:RetainedCloneGrant)->Result<Option<(Self,RetainedCloneProgress)>,ValueError>{
  let demand=Self::projection_demand();if !fits(demand,grant){return Ok(None);}
  let original=original.get();let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-size_of::<Self>()*2,..grant};
  let root=original.root.as_ref().map(|root|root.try_duplicate(child).map(|(root,_)|root)).transpose()?;
  Ok(Some((Self{root,len:original.len,height:original.height},RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})))
 }
 /// 📦️ Each fixed page birth includes owned header movement independently of its Arc backing.
 pub fn page_birth_demand()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Page<T>>()*2+size_of::<Self>(),capacity_bytes:SealedShared::<Page<T>>::birth_bytes(),depth:1,..Default::default()}}
 /// 🌱️ A leaf adopts only explicitly owned original row authorities under its fixed page grant.
 pub fn admit_leaf(rows:[Option<SealedShared<T>>;PERSISTENT_PAGE_FANOUT],len:usize,grant:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),(ValueError,[Option<SealedShared<T>>;PERSISTENT_PAGE_FANOUT])>{
  let d=Self::page_birth_demand();if !fits(d,grant){return Err((ValueError::literal(ValueRefusalKind::OwnershipLimit,"immutable leaf page birth is not funded"),rows));}
  if len==0||len>PERSISTENT_PAGE_FANOUT||rows.iter().enumerate().any(|(i,row)|row.is_some()!=(i<len)){return Err((ValueError::literal(ValueRefusalKind::InvariantViolated,"immutable leaf requires an exact occupied prefix"),rows));}
  match SealedShared::admit(Page::Leaf{rows,len},SharedIssuer::owned(),grant){Ok((root,p))=>Ok((Self{root:Some(root),len,height:0},RetainedCloneProgress{copied_bytes:d.copy_bytes,..p})),Err((error,Page::Leaf{rows,..}))=>Err((error,rows)),Err(_)=>unreachable!("immutable leaf retains original declared page")}
 }
 /// 🌿️ Branch admission adopts a dense immutable prefix and preserves every exact child root.
 pub fn admit_branch(mut children:[Option<Self>;PERSISTENT_PAGE_FANOUT],grant:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),(ValueError,[Option<Self>;PERSISTENT_PAGE_FANOUT])>{
  let d=Self::page_birth_demand();if !fits(d,grant){return Err((ValueError::literal(ValueRefusalKind::OwnershipLimit,"immutable branch page birth is not funded"),children));}
  let Some(first)=children[0].as_ref()else{return Err((ValueError::literal(ValueRefusalKind::InvariantViolated,"immutable branch requires at least one child"),children));};
  let child_height=first.height;let Some(height)=child_height.checked_add(1).filter(|height|usize::from(*height)*4<usize::BITS as usize)else{return Err((ValueError::literal(ValueRefusalKind::OwnershipLimit,"immutable page height exceeds ranked address space"),children));};
  let full=1usize<<(usize::from(height)*4);let mut len=0usize;let mut gap=false;
  for child in &children{if let Some(child)=child{if gap||child.height!=child_height||child.len==0||child.len>full||child.root.is_none(){return Err((ValueError::literal(ValueRefusalKind::InvariantViolated,"immutable branch requires a dense prefix of matching-height nonempty children"),children));}len=match len.checked_add(child.len){Some(len)=>len,None=>return Err((ValueError::literal(ValueRefusalKind::OwnershipLimit,"immutable branch cardinality overflow"),children))};}else{gap=true;}}
  let roots=std::array::from_fn(|i|children[i].as_mut().and_then(|child|child.root.take()));
  match SealedShared::admit(Page::Branch{children:roots,len,height},SharedIssuer::owned(),grant){Ok((root,p))=>Ok((Self{root:Some(root),len,height},RetainedCloneProgress{copied_bytes:d.copy_bytes,..p})),Err((error,Page::Branch{children:roots,..}))=>{for(i,root)in roots.into_iter().enumerate(){if let Some(root)=root{children[i].as_mut().expect("original branch child wrapper").root=Some(root);}}Err((error,children))},Err(_)=>unreachable!("immutable branch retains original declared page")}
 }
 /// 👁️ Ranked reads borrow immutable original rows through a logarithmic fixed-fanout path.
 pub fn get(&self,mut index:usize)->Option<&T>{if index>=self.len{return None;}let mut page=self.root.as_ref()?.get();loop{match page{Page::Leaf{rows,..}=>return rows[index].as_ref().map(SealedShared::get),Page::Branch{children,..}=>{let(slot,relative)=child_rank(children,index,false)?;index=relative;page=children[slot].as_ref()?.get();}}}} /// 🧮️ Cardinality is fixed metadata, independent of page count and payload size.
 /// 🎟️ Ranked observation prices every bounded original branch inspection and final row projection.
 pub fn rank_observation_demand(&self,index:usize)->Result<RetirementDemand,ValueError>{
  if index>=self.len{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"rank is outside the original immutable list"));}
  let levels=usize::from(self.height)+1;
  let frame=size_of::<Option<SealedShared<Page<T>>>>()*PERSISTENT_PAGE_FANOUT+size_of::<usize>()*(PERSISTENT_PAGE_FANOUT+3);
  let copy_bytes=levels.checked_mul(frame).and_then(|n|n.checked_add(size_of::<RetainedCloneRef<'_,T>>()*2)).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"ranked observation footprint overflow"))?;
  Ok(RetirementDemand{copy_bytes,depth:levels,..Default::default()})
 }
 /// 🧭️ A funded private page traversal lends the exact original row through its genuine root source.
 pub fn observe_rank(original:RetainedCloneRef<'_,Self>,index:usize,grant:RetainedCloneGrant)->Result<Option<(RetainedCloneRef<'_,T>,RetainedCloneProgress)>,ValueError>{
  let d=original.get().rank_observation_demand(index)?;if !fits(d,grant){return Ok(None);}
  let mut relative=index;let mut page=original.get().root.as_ref().expect("admitted original rank root").get();
  let row=loop{match page{Page::Leaf{rows,..}=>break rows[relative].as_ref().expect("admitted original rank row").get(),Page::Branch{children,..}=>{let(slot,rank)=child_rank(children,relative,false).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original ranked page lost its child"))?;relative=rank;page=children[slot].as_ref().expect("admitted original rank child").get();}}};
  let pointer=row as*const T;let row=original.project(index,|_|unsafe{&*pointer});
  Ok(Some((row,RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})))
 }
 pub fn len(&self)->usize{self.len}
 pub fn is_empty(&self)->bool{self.len==0}
}

/// 🧳️ One immutable replacement stages fixed page paths while keeping exact original list and row sources.
pub struct PersistentSet<T:Send+Sync+'static>{
 original:Option<crate::retained_clone::RetainedCloneBinding>,
 authored:Option<crate::retained_clone::RetainedCloneBinding>,
 page:std::mem::ManuallyDrop<Option<Page<T>>>,
 candidate:std::mem::ManuallyDrop<Option<SealedShared<Page<T>>>>,
 retirement:crate::retained_clone::RetainedCloneClose,
 index:usize,len:usize,height:u8,level:u8,slot:usize,phase:u8,closing:bool,
}
impl<T:Send+Sync+'static> PersistentSet<T>{
 /// 🌱️ The fixed cursor has no heap birth and captures neither row nor original root.
 pub fn new(index:usize)->Self{Self{original:None,authored:None,page:std::mem::ManuallyDrop::new(None),candidate:std::mem::ManuallyDrop::new(None),retirement:Default::default(),index,len:0,height:0,level:0,slot:0,phase:0,closing:false}}
 fn metadata(&self)->usize{size_of::<usize>()*(usize::from(self.height)*2+5)+size_of::<u8>()*3}
 fn page_at<'a>(&self,list:&'a PersistentList<T>)->Result<(&'a Page<T>,usize),ValueError>{
  let mut page=list.root.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"immutable candidate lost its original root"))?.get();let mut height=list.height;let mut rank=self.index;
  while height>self.level{let Page::Branch{children,..}=page else{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"immutable original branch shape changed"));};let(slot,relative)=child_rank(children,rank,false).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"immutable original path has no child"))?;rank=relative;page=children[slot].as_ref().unwrap().get();height-=1;}Ok((page,rank))
 } /// 📏️ Every path inspection, original alias duplication and fixed page birth has an indivisible quote.
 pub fn demand(&self,original:RetainedCloneRef<'_,PersistentList<T>>,authored:RetainedCloneRef<'_,SealedShared<T>>)->Result<RetirementDemand,ValueError>{
  if self.closing||self.phase>4{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"immutable page preparation is closing or transferred"));}
  if self.original.is_none(){return Ok(RetirementDemand{copy_bytes:original.binding_copy_bytes(),depth:1,..Default::default()});}
  if self.authored.is_none(){return Ok(RetirementDemand{copy_bytes:authored.binding_copy_bytes(),depth:1,..Default::default()});}
  let copy_bytes=match self.phase{0=>size_of::<usize>()*3+size_of::<u8>()*3,1=>size_of::<Page<T>>()+self.metadata(),2=>if self.level==0{size_of::<SealedShared<T>>()*2+self.metadata()}else{size_of::<SealedShared<Page<T>>>()*2+self.metadata()},3=>size_of::<Page<T>>()*2+size_of::<SealedShared<Page<T>>>()+self.metadata(),4=>0,_=>unreachable!()};
  Ok(RetirementDemand{copy_bytes,capacity_bytes:if self.phase==3{SealedShared::<Page<T>>::birth_bytes()}else{0},depth:1,..Default::default()})
 }
 /// 🧵️ Only changed ancestors are born; sibling rows and pages duplicate their sealed original authorities.
 pub fn advance(&mut self,original:RetainedCloneRef<'_,PersistentList<T>>,authored:RetainedCloneRef<'_,SealedShared<T>>,grant:RetainedCloneGrant)->Result<crate::retained_clone::RetainedCloneStep,ValueError>{
  use crate::retained_clone::{RetainedCloneStep,admit_retained_clone_progress};
  let d=self.demand(original,authored)?;if !fits(d,grant){return Ok(RetainedCloneStep::Progress(Default::default()));}
  if let Some(p)=original.bind(&mut self.original,grant)?{return Ok(RetainedCloneStep::Progress(p));}
  if let Some(p)=authored.bind(&mut self.authored,grant)?{return Ok(RetainedCloneStep::Progress(p));}
  let mut p=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};
  match self.phase{
   0=>{self.len=original.get().len;self.height=original.get().height;if self.index>=self.len{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"immutable replacement index is outside the original collection").with_retained_progress(p));}self.phase=1;},
   1=>{let page=match self.page_at(original.get())?.0{Page::Leaf{len,..}=>Page::Leaf{rows:std::array::from_fn(|_|None),len:*len},Page::Branch{len,height,..}=>Page::Branch{children:std::array::from_fn(|_|None),len:*len,height:*height}};*self.page=Some(page);self.slot=0;self.phase=2;},
   2=>{
    let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-self.metadata(),..grant};let (original_page,rank)=self.page_at(original.get())?;let target=match original_page{Page::Leaf{..}=>rank,Page::Branch{children,..}=>child_rank(children,rank,false).expect("original changed child rank").0};
    match original_page{
     Page::Leaf{rows,..}=>{let alias=if self.slot==target{Some(authored.get().try_duplicate(child)?.0)}else{rows[self.slot].as_ref().map(|row|row.try_duplicate(child).map(|(row,_)|row)).transpose()?};let Some(Page::Leaf{rows,..})=self.page.as_mut()else{unreachable!("original immutable leaf plan")};rows[self.slot]=alias;},
     Page::Branch{children,..}=>{let alias=if self.slot==target{Some(self.candidate.take().expect("changed child page"))}else{children[self.slot].as_ref().map(|page|page.try_duplicate(child).map(|(page,_)|page)).transpose()?};let Some(Page::Branch{children,..})=self.page.as_mut()else{unreachable!("original immutable branch plan")};children[self.slot]=alias;}
    }
    self.slot+=1;if self.slot==PERSISTENT_PAGE_FANOUT{self.phase=3;}
   },
   3=>{let page=self.page.take().expect("complete owned immutable page");match SealedShared::admit(page,SharedIssuer::owned(),grant){Ok((candidate,receipt))=>{*self.candidate=Some(candidate);p.retained_capacity_bytes=receipt.retained_capacity_bytes;},Err((error,page))=>{*self.page=Some(page);return Err(error);}}self.level+=1;self.phase=if self.level>self.height{4}else{1};},
   4=>return Ok(RetainedCloneStep::Complete(Default::default())),
   _=>unreachable!("quoted immutable path phase"),
  }
  let p=admit_retained_clone_progress(grant,p,"immutable shared changed page")?;Ok(if self.phase==4{RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})
 }
 /// 🪪️ Preparation readiness is independent of zero-valued numeric work quotes.
 pub fn output_ready(&self)->bool{!self.closing&&self.phase==4&&self.candidate.is_some()}
 /// 📤️ A funded fixed-header transfer yields the immutable prepared candidate root once.
 pub fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.output_ready(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"immutable candidate root is not ready"));}Ok(RetirementDemand{copy_bytes:size_of::<PersistentList<T>>()*2+size_of::<SealedShared<Page<T>>>(),depth:1,..Default::default()})}
 pub fn take(&mut self,grant:RetainedCloneGrant)->Result<Option<(PersistentList<T>,RetainedCloneProgress)>,ValueError>{
  if !self.output_ready(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"immutable candidate root is not ready"));}
  let d=self.take_demand()?;if !fits(d,grant){return Ok(None);}
  let root=self.candidate.take();self.phase=5;Ok(Some((PersistentList{root,len:self.len,height:self.height},RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})))
 }
 /// 🛑️ Cancellation never changes original published roots.
 pub fn begin_close(&mut self){self.closing=true;}
 /// ♻️ Every staged page and genuine source alias keeps its original typed retirement authority.
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<Page<T>>(false,&None,body);}
  if self.page.is_some(){return self.retirement.next_owner_demands::<Page<T>>(true,&None,body);}
  if self.candidate.is_some(){return self.retirement.next_owner_demands::<SealedShared<Page<T>>>(true,&None,body);}
  let binding=if self.authored.is_some(){&self.authored}else{&self.original};Ok(RetirementDemand{copy_bytes:crate::retained_clone::RetainedCloneBinding::copy_demand(binding)?,capacity_bytes:crate::retained_clone::RetainedCloneBinding::capacity_demand(binding,body)?,release_bytes:crate::retained_clone::RetainedCloneBinding::release_demand(binding)?,depth:crate::retained_clone::RetainedCloneBinding::depth_demand(binding)?})
 }
 /// 🪜️ Only completely settled pages and source bindings permit terminal cursor release.
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<crate::retained_clone::RetainedCloneStep,ValueError>{
  use crate::retained_clone::{RetainedCloneStep,RetainedCloneBinding};if !self.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"immutable candidate closure not started"));}
  let d=self.close_demand(grant.maximum_copy_bytes)?;if !fits(d,grant){return Ok(RetainedCloneStep::Progress(Default::default()));}
  if !self.retirement.is_empty(){return self.retirement.step_granted(grant).map(|step|RetainedCloneStep::Progress(step.progress()));}
  if let Some(step)=self.retirement.begin_granted(&mut self.page,grant)?{return Ok(RetainedCloneStep::Progress(step.progress()));}
  if let Some(step)=self.retirement.begin_granted(&mut self.candidate,grant)?{return Ok(RetainedCloneStep::Progress(step.progress()));}
  let step=RetainedCloneBinding::close_one(if self.authored.is_some(){&mut self.authored}else{&mut self.original},grant)?;Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())})
 }
 pub fn terminal_is_empty(&self)->bool{self.page.is_none()&&self.candidate.is_none()&&self.retirement.is_empty()&&self.original.is_none()&&self.authored.is_none()}
}
impl<T:Send+Sync+'static> Drop for PersistentSet<T>{fn drop(&mut self){let empty=self.terminal_is_empty();assert!(empty||std::thread::panicking(),"immutable candidate retains original pages and issuer bindings until funded closure");if empty{unsafe{std::mem::ManuallyDrop::drop(&mut self.page);std::mem::ManuallyDrop::drop(&mut self.candidate);}}}}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path="📥️build/🧪️tests/🦀️.rs"]
mod build_tests;
