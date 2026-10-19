//! ✂️ Ranked insertion and removal copy only fixed pages on the changed immutable path.
use super::*;
use crate::retained_clone::{RetainedCloneBinding,RetainedCloneClose,RetainedCloneStep,admit_retained_clone_progress};
use std::mem::ManuallyDrop;
/// 🎮️ Each structural event owns one insertion or one removal at its original rank.
#[derive(Clone,Copy,PartialEq,Eq)]
pub enum PersistentSpliceKind{Insert,Remove}
#[derive(crate::RetireOwned)]
enum Scratch<T:Send+Sync+'static>{Leaf{rows:[Option<SealedShared<T>>;17],count:usize},Branch{children:[Option<SealedShared<Page<T>>>;17],count:usize,height:u8}}
/// 🧳️ Original pages, split candidates and authored row aliases retain typed cancellation custody.
pub struct PersistentSplice<T:Send+Sync+'static>{
 original:Option<RetainedCloneBinding>,authored:Option<RetainedCloneBinding>,scratch:ManuallyDrop<Option<Scratch<T>>>,candidate:ManuallyDrop<[Option<SealedShared<Page<T>>>;2]>,retirement:RetainedCloneClose,
 kind:PersistentSpliceKind,index:usize,len:usize,height:u8,level:u8,slot:usize,count:usize,rank:usize,changed:usize,pair_count:usize,phase:u8,root_split:bool,closing:bool,
}
impl<T:Send+Sync+'static> PersistentSplice<T>{
 /// 🌱️ Fixed metadata captures no original row, page or heap allocation.
 pub fn new(kind:PersistentSpliceKind,index:usize)->Self{Self{original:None,authored:None,scratch:ManuallyDrop::new(None),candidate:ManuallyDrop::new(std::array::from_fn(|_|None)),retirement:Default::default(),kind,index,len:0,height:0,level:0,slot:0,count:0,rank:0,changed:0,pair_count:0,phase:0,root_split:false,closing:false}}
 fn fixed(&self)->usize{size_of::<usize>()*(16*usize::from(self.height)+12)+size_of::<u8>()*4+size_of::<bool>()}
 fn page_at<'a>(&self,list:&'a PersistentList<T>)->Result<(Option<&'a Page<T>>,usize),ValueError>{let Some(root)=list.root.as_ref()else{return Ok((None,0));};let mut page=root.get();let mut height=list.height;let mut rank=self.index;while height>self.level{let Page::Branch{children,..}=page else{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"immutable splice lost its original branch"));};let(slot,relative)=child_rank(children,rank,self.kind==PersistentSpliceKind::Insert).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"immutable splice lost its original rank"))?;rank=relative;page=children[slot].as_ref().unwrap().get();height-=1;}Ok((Some(page),rank))}
 /// 📏️ Every ranked read, alias move and sealed page birth has a separate fixed quote.
 pub fn demand(&self,original:RetainedCloneRef<'_,PersistentList<T>>,authored:Option<RetainedCloneRef<'_,SealedShared<T>>>)->Result<RetirementDemand,ValueError>{
  if self.closing||self.phase>6{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"immutable splice is closing or transferred"));}
  if self.original.is_none(){return Ok(RetirementDemand{copy_bytes:original.binding_copy_bytes(),depth:1,..Default::default()});}
  if self.kind==PersistentSpliceKind::Insert&&self.authored.is_none(){let a=authored.ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"immutable insertion requires an original authored row source"))?;return Ok(RetirementDemand{copy_bytes:a.binding_copy_bytes(),depth:1,..Default::default()});}
  let born=matches!(self.phase,3|4|5)&&self.count>0;
  let copy=match self.phase{0=>self.fixed(),1=>size_of::<Scratch<T>>()+self.fixed(),2=>2*size_of::<SealedShared<Page<T>>>()+2*size_of::<SealedShared<T>>()+self.fixed(),3|4|5=>2*size_of::<Scratch<T>>()+2*size_of::<Page<T>>()+2*size_of::<SealedShared<Page<T>>>()+self.fixed(),6=>0,_=>unreachable!()};
  Ok(RetirementDemand{copy_bytes:copy,capacity_bytes:if born{SealedShared::<Page<T>>::birth_bytes()}else{0},depth:1,..Default::default()})
 }
 /// 🧵️ A split creates two fixed pages and propagates only through the original changed ancestors.
 pub fn advance(&mut self,original:RetainedCloneRef<'_,PersistentList<T>>,authored:Option<RetainedCloneRef<'_,SealedShared<T>>>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  let d=self.demand(original,authored)?;if !fits(d,grant){return Ok(RetainedCloneStep::Progress(Default::default()));}if let Some(p)=original.bind(&mut self.original,grant)?{return Ok(RetainedCloneStep::Progress(p));}if self.kind==PersistentSpliceKind::Insert{if let Some(p)=authored.unwrap().bind(&mut self.authored,grant)?{return Ok(RetainedCloneStep::Progress(p));}}
  let mut p=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};
  match self.phase{
   0=>{self.height=original.get().height;self.len=original.get().len;let valid=if self.kind==PersistentSpliceKind::Insert{self.index<=self.len}else{self.index<self.len};if !valid{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"immutable splice rank is outside its original collection").with_retained_progress(p));}self.len=if self.kind==PersistentSpliceKind::Insert{self.len.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"immutable insertion cardinality overflow"))?}else{self.len-1};self.phase=1;}
   1=>{
    let(page,rank)=self.page_at(original.get())?;self.rank=rank;self.slot=0;
    *self.scratch=Some(match page{
     None=>{self.count=1;Scratch::Leaf{rows:std::array::from_fn(|_|None),count:1}},
     Some(Page::Leaf{len,..})=>{self.count=if self.kind==PersistentSpliceKind::Insert{len+1}else{len-1};Scratch::Leaf{rows:std::array::from_fn(|_|None),count:self.count}},
     Some(Page::Branch{children,height,..})=>{self.changed=child_rank(children,rank,self.kind==PersistentSpliceKind::Insert).expect("original splice ancestor rank").0;let count=children.iter().take_while(|c|c.is_some()).count();self.count=count-1+self.pair_count;Scratch::Branch{children:std::array::from_fn(|_|None),count:self.count,height:*height}},
    });self.phase=if self.count==0{3}else{2};
   }
   2=>{
    let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-self.fixed(),..grant};let(page,_)=self.page_at(original.get())?;
    match (page,self.scratch.as_mut().unwrap()){
     (None,Scratch::Leaf{rows,..})=>rows[0]=Some(authored.unwrap().get().try_duplicate(child)?.0),
     (Some(Page::Leaf{rows:before,..}),Scratch::Leaf{rows,..})=>{rows[self.slot]=if self.kind==PersistentSpliceKind::Insert&&self.slot==self.rank{Some(authored.unwrap().get().try_duplicate(child)?.0)}else{let from=if self.kind==PersistentSpliceKind::Insert{self.slot-usize::from(self.slot>self.rank)}else{self.slot+usize::from(self.slot>=self.rank)};Some(before[from].as_ref().unwrap().try_duplicate(child)?.0)};},
     (Some(Page::Branch{children:before,..}),Scratch::Branch{children,..})=>{children[self.slot]=if self.slot>=self.changed&&self.slot<self.changed+self.pair_count{self.candidate[self.slot-self.changed].take()}else{let from=if self.slot<self.changed{self.slot}else{self.slot+1-self.pair_count};Some(before[from].as_ref().unwrap().try_duplicate(child)?.0)};},
     _=>unreachable!("original splice page shape"),
    }
    self.slot+=1;if self.slot==self.count{self.phase=3;}
   }
   3|4=>{
    let second=self.phase==4;let split=self.count>16;let start=if second{8}else{0};let end=if split&&!second{8}else{self.count};
    if self.count>0{let page=match self.scratch.as_mut().unwrap(){Scratch::Leaf{rows,..}=>Page::Leaf{rows:std::array::from_fn(|i|if start+i<end{rows[start+i].take()}else{None}),len:end-start},Scratch::Branch{children,height,..}=>{let children=std::array::from_fn(|i|if start+i<end{children[start+i].take()}else{None});let len=children.iter().flatten().map(|c|page_len(c.get())).sum();Page::Branch{children,len,height:*height}}};match SealedShared::admit(page,SharedIssuer::owned(),grant){Ok((root,receipt))=>{self.candidate[usize::from(second)]=Some(root);p.retained_capacity_bytes=receipt.retained_capacity_bytes;},Err((error,page))=>{match(self.scratch.as_mut().unwrap(),page){(Scratch::Leaf{rows,..},Page::Leaf{rows:mut returned,..})=>for i in start..end{rows[i]=returned[i-start].take();},(Scratch::Branch{children,..},Page::Branch{children:mut returned,..})=>for i in start..end{children[i]=returned[i-start].take();},_=>unreachable!()};return Err(error);}}}
    if split&&!second{self.phase=4;}else{drop(self.scratch.take());self.pair_count=if self.count==0{0}else if split{2}else{1};self.level+=1;if self.level>self.height{self.phase=if self.pair_count==2{5}else{6};}else{self.phase=1;}}
   }
   5=>{let children=std::array::from_fn(|i|if i<2{self.candidate[i].take()}else{None});let page=Page::Branch{children,len:self.len,height:self.height+1};match SealedShared::admit(page,SharedIssuer::owned(),grant){Ok((root,receipt))=>{self.candidate[0]=Some(root);self.root_split=true;self.pair_count=1;self.phase=6;p.retained_capacity_bytes=receipt.retained_capacity_bytes;},Err((error,Page::Branch{mut children,..}))=>{self.candidate[0]=children[0].take();self.candidate[1]=children[1].take();return Err(error);},Err(_)=>unreachable!()}}
   6=>return Ok(RetainedCloneStep::Complete(Default::default())),_=>unreachable!(),
  }
  let p=admit_retained_clone_progress(grant,p,"immutable ranked splice")?;Ok(if self.phase==6{RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})
 }
 /// 📤️ The exact immutable candidate root transfers once after every page birth is admitted.
 pub fn take_demand(&self)->RetirementDemand{RetirementDemand{copy_bytes:2*size_of::<PersistentList<T>>()+size_of::<SealedShared<Page<T>>>(),depth:1,..Default::default()}}
 pub fn take(&mut self,grant:RetainedCloneGrant)->Result<Option<(PersistentList<T>,RetainedCloneProgress)>,ValueError>{if self.closing||self.phase!=6{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"immutable splice candidate is not ready"));}let d=self.take_demand();if !fits(d,grant){return Ok(None);}let root=self.candidate[0].take();self.phase=7;Ok(Some((PersistentList{root,len:self.len,height:if self.len==0{0}else{self.height+u8::from(self.root_split)}},RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})))}
 pub fn begin_close(&mut self){self.closing=true;}
 /// ♻️ Staged split pages and genuine aliases retain their exact original typed issuers.
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<Scratch<T>>(false,&None,body);}if self.scratch.is_some(){return self.retirement.next_owner_demands::<Scratch<T>>(true,&None,body);}if self.candidate.iter().any(Option::is_some){return self.retirement.next_owner_demands::<SealedShared<Page<T>>>(true,&None,body);}let b=if self.authored.is_some(){&self.authored}else{&self.original};Ok(RetirementDemand{copy_bytes:RetainedCloneBinding::copy_demand(b)?,capacity_bytes:RetainedCloneBinding::capacity_demand(b,body)?,release_bytes:RetainedCloneBinding::release_demand(b)?,depth:RetainedCloneBinding::depth_demand(b)?})}
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{if !self.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"immutable splice close not started"));}let d=self.close_demand(grant.maximum_copy_bytes)?;if !fits(d,grant){return Ok(RetainedCloneStep::Progress(Default::default()));}if !self.retirement.is_empty(){return self.retirement.step_granted(grant).map(|s|RetainedCloneStep::Progress(s.progress()));}if let Some(s)=self.retirement.begin_granted(&mut self.scratch,grant)?{return Ok(RetainedCloneStep::Progress(s.progress()));}for c in self.candidate.iter_mut(){if let Some(s)=self.retirement.begin_granted(c,grant)?{return Ok(RetainedCloneStep::Progress(s.progress()));}}let s=RetainedCloneBinding::close_one(if self.authored.is_some(){&mut self.authored}else{&mut self.original},grant)?;Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(s.progress())}else{RetainedCloneStep::Progress(s.progress())})}
 pub fn terminal_is_empty(&self)->bool{self.scratch.is_none()&&self.candidate.iter().all(Option::is_none)&&self.retirement.is_empty()&&self.original.is_none()&&self.authored.is_none()}
}
impl<T:Send+Sync+'static> Drop for PersistentSplice<T>{fn drop(&mut self){let empty=self.terminal_is_empty();assert!(std::thread::panicking()||empty,"immutable splice requires funded original custody closure");if empty{unsafe{ManuallyDrop::drop(&mut self.scratch);ManuallyDrop::drop(&mut self.candidate);}}}}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"] mod tests;
