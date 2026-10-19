//! 📥️ Original owned rows become immutable fixed-fanout pages through separately funded carry turns.
use super::{PersistentList,PERSISTENT_PAGE_FANOUT,fits};
use crate::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneClose,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep},shared_issuance::SealedShared};
use std::mem::{ManuallyDrop,size_of};
const LEVELS:usize=usize::BITS as usize/4;
type Rows<T>=[Option<SealedShared<T>>;PERSISTENT_PAGE_FANOUT];
type Roots<T>=[Option<PersistentList<T>>;PERSISTENT_PAGE_FANOUT];
#[derive(Clone,Copy,PartialEq,Eq)]
enum Phase{Wait,Leaf,Carry,Branch,Scan,Ready,Spent}
/// 🌳️ A native address-space bound limits original construction custody independently of stream length.
pub struct PersistentListBuilder<T:Send+Sync+'static>{
 rows:ManuallyDrop<Rows<T>>,levels:ManuallyDrop<[Option<Box<Roots<T>>>;LEVELS]>,carry:ManuallyDrop<Option<PersistentList<T>>>,output:ManuallyDrop<Option<PersistentList<T>>>,
 retirement:RetainedCloneClose,counts:[u8;LEVELS],expected:usize,adopted:usize,row_count:usize,level:usize,close_index:usize,phase:Phase,finishing:bool,closing:bool,
}
fn refusal(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
fn receipt(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}}
impl<T:Send+Sync+'static> PersistentListBuilder<T>{
 /// 🌱️ The fixed owned header has no heap birth and requires its original movement authority.
 pub fn constructor_demand()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Self>()*2,depth:1,..Default::default()}}
 /// 🪪️ Construction captures no payload or root until an explicitly owned receiving slot is admitted.
 pub fn new(expected:usize,grant:RetainedCloneGrant)->Result<Option<Self>,ValueError>{if !fits(Self::constructor_demand(),grant){return Ok(None);}Ok(Some(Self{rows:ManuallyDrop::new(std::array::from_fn(|_|None)),levels:ManuallyDrop::new(std::array::from_fn(|_|None)),carry:ManuallyDrop::new(None),output:ManuallyDrop::new(None),retirement:Default::default(),counts:[0;LEVELS],expected,adopted:0,row_count:0,level:0,close_index:0,phase:Phase::Wait,finishing:false,closing:false}))}
 /// 🚪️ Only the current original stream position may supply another owned row header.
 pub fn accepts_row(&self)->bool{!self.closing&&!self.finishing&&self.phase==Phase::Wait}
 /// 📏️ Adoption prices one original sealed header and the touched stream counters.
 pub fn adopt_demand(&self)->Result<RetirementDemand,ValueError>{if !self.accepts_row()||self.adopted>=self.expected{return Err(refusal("original persistent row is outside its admitted stream"));}Ok(RetirementDemand{copy_bytes:size_of::<Option<SealedShared<T>>>()*2+size_of::<usize>()*3+size_of::<Phase>(),depth:1,..Default::default()})}
 /// 📦️ A denied adoption preserves the exact original receiving slot without producing another issuer.
 pub fn adopt(&mut self,row:&mut Option<SealedShared<T>>,grant:RetainedCloneGrant)->Result<Option<RetainedCloneProgress>,ValueError>{let d=self.adopt_demand()?;if !fits(d,grant){return Ok(None);}if row.is_none(){return Err(refusal("original persistent row receiving slot is empty"));}self.rows[self.row_count]=row.take();self.row_count+=1;self.adopted+=1;if self.row_count==PERSISTENT_PAGE_FANOUT{self.phase=Phase::Leaf;}Ok(Some(receipt(d)))}
 /// 🏁️ Finishing requires the exact authored cardinality before any incomplete page may be sealed.
 pub fn finish_demand(&self)->Result<RetirementDemand,ValueError>{if self.closing||self.finishing||self.adopted!=self.expected||self.phase==Phase::Spent{return Err(refusal("original persistent stream has not supplied its exact cardinality"));}Ok(RetirementDemand{copy_bytes:size_of::<usize>()*3+size_of::<Phase>()+size_of::<bool>(),depth:1,..Default::default()})}
 /// 🧵️ Finishing folds lower partial levels before joining higher original page roots.
 pub fn finish(&mut self,grant:RetainedCloneGrant)->Result<Option<RetainedCloneProgress>,ValueError>{let d=self.finish_demand()?;if !fits(d,grant){return Ok(None);}self.finishing=true;if self.phase==Phase::Wait{self.level=0;self.phase=if self.row_count>0{Phase::Leaf}else{Phase::Scan};}Ok(Some(receipt(d)))}
 fn metadata()->usize{size_of::<usize>()*3+size_of::<u8>()+size_of::<Phase>()}
 /// 🎟️ Each fixed page birth, header carry or rank scan exposes its complete indivisible quote.
 pub fn demand(&self)->Result<RetirementDemand,ValueError>{if self.closing{return Err(refusal("original persistent construction is closing"));}let level_birth=self.phase==Phase::Carry&&self.level<LEVELS&&self.levels[self.level].is_none();let extra=match self.phase{Phase::Leaf=>size_of::<Rows<T>>()*2+Self::metadata(),Phase::Branch=>size_of::<Roots<T>>()*2+Self::metadata(),Phase::Carry=>size_of::<Option<PersistentList<T>>>()*2+Self::metadata()+if level_birth{size_of::<Roots<T>>()*2+size_of::<Option<Box<Roots<T>>>>()*2}else{0},Phase::Scan=>size_of::<Option<PersistentList<T>>>()*2+size_of::<u8>()*LEVELS+Self::metadata(),_=>return Err(refusal("original persistent builder has no pending preparation turn"))};if matches!(self.phase,Phase::Leaf|Phase::Branch){let mut d=PersistentList::<T>::page_birth_demand();d.copy_bytes=d.copy_bytes.checked_add(extra).ok_or_else(||refusal("original persistent page quote overflows native address space"))?;Ok(d)}else{Ok(RetirementDemand{copy_bytes:extra,capacity_bytes:if level_birth{size_of::<Roots<T>>()}else{0},depth:if level_birth{2}else{1},..Default::default()})}}
 /// 🪜️ Preparation moves original roots through one fixed carry or one actual sealed page birth per turn.
 pub fn advance(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  let d=self.demand()?;if !fits(d,grant){return Ok(RetainedCloneStep::Progress(Default::default()));}let mut p=receipt(d);
  match self.phase{
   Phase::Leaf=>{let rows=std::mem::replace(&mut*self.rows,std::array::from_fn(|_|None));match PersistentList::admit_leaf(rows,self.row_count,grant){Ok((root,birth))=>{*self.carry=Some(root);p.retained_capacity_bytes=birth.retained_capacity_bytes;self.row_count=0;self.level=0;self.phase=Phase::Carry;},Err((error,rows))=>{*self.rows=rows;return Err(error);}}},
   Phase::Carry=>{let Some(count)=self.counts.get_mut(self.level)else{return Err(refusal("original persistent carry exceeds ranked address space"));};let slot=usize::from(*count);if slot>=PERSISTENT_PAGE_FANOUT||self.carry.is_none(){return Err(refusal("original persistent carry has no vacant original page slot"));}if self.levels[self.level].is_none(){self.levels[self.level]=Some(Box::new(std::array::from_fn(|_|None)));p.retained_capacity_bytes=size_of::<Roots<T>>();}self.levels[self.level].as_mut().expect("original funded level table")[slot]=self.carry.take();*count+=1;self.phase=if usize::from(*count)==PERSISTENT_PAGE_FANOUT{Phase::Branch}else if self.finishing{self.level=0;Phase::Scan}else{Phase::Wait};},
   Phase::Branch=>{if self.level>=LEVELS||self.counts[self.level]==0{return Err(refusal("original persistent branch has no children"));}let table=self.levels[self.level].as_mut().ok_or_else(||refusal("original persistent branch has no owned table"))?;let roots=std::mem::replace(&mut**table,std::array::from_fn(|_|None));match PersistentList::admit_branch(roots,grant){Ok((root,birth))=>{*self.carry=Some(root);p.retained_capacity_bytes=birth.retained_capacity_bytes;self.counts[self.level]=0;self.level+=1;self.phase=Phase::Carry;},Err((error,roots))=>{**table=roots;return Err(error);}}},
   Phase::Scan=>{if self.expected==0{*self.output=Some(PersistentList::default());self.phase=Phase::Ready;}else if self.level>=LEVELS{return Err(refusal("original persistent stream lost its complete page roots"));}else if self.counts[self.level]==0{self.level+=1;}else if self.counts[self.level]==1&&self.counts[self.level+1..].iter().all(|count|*count==0){*self.output=self.levels[self.level].as_mut().ok_or_else(||refusal("original final root has no owned table"))?[0].take();self.counts[self.level]=0;self.phase=Phase::Ready;}else{self.phase=Phase::Branch;}},
   _=>unreachable!("quoted original persistent preparation phase"),
  }
  Ok(if self.output_ready(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})
 }
 /// 👁️ Readiness requires the genuine final owned root, including an authored empty collection.
 pub fn output_ready(&self)->bool{!self.closing&&self.phase==Phase::Ready&&self.output.is_some()}
 /// 📤️ Final publication transfers only a fixed root header and leaves no duplicate source custody.
 pub fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.output_ready(){return Err(refusal("original persistent candidate is not ready"));}Ok(RetirementDemand{copy_bytes:size_of::<Option<PersistentList<T>>>()*2+size_of::<Phase>(),depth:1,..Default::default()})}
 /// 🔑️ A denied final transfer retains the original final root for another funded turn or cancellation.
 pub fn take(&mut self,grant:RetainedCloneGrant)->Result<Option<(PersistentList<T>,RetainedCloneProgress)>,ValueError>{let d=self.take_demand()?;if !fits(d,grant){return Ok(None);}let output=self.output.take().expect("ready original persistent root");self.phase=Phase::Spent;Ok(Some((output,receipt(d))))}
 /// 🛑️ Every receiving frontier keeps its genuine row and page issuers until funded closure.
 pub fn begin_close(&mut self){self.closing=true;}
 fn close_metadata()->usize{size_of::<usize>()*2+size_of::<Option<PersistentList<T>>>().max(size_of::<Option<SealedShared<T>>>())}
 /// ♻️ Cleanup quotes one original owned slot or the active native child without recursive repeated queries.
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<PersistentList<T>>(false,&None,body);}
  let mut d=if self.close_index<PERSISTENT_PAGE_FANOUT{self.retirement.next_owner_demands::<SealedShared<T>>(self.rows[self.close_index].is_some(),&None,body)?}else if self.close_index<PERSISTENT_PAGE_FANOUT+LEVELS{let index=self.close_index-PERSISTENT_PAGE_FANOUT;self.retirement.next_owner_demands::<Box<Roots<T>>>(self.levels[index].is_some(),&None,body)?}else if self.close_index==PERSISTENT_PAGE_FANOUT+LEVELS{self.retirement.next_owner_demands::<PersistentList<T>>(self.carry.is_some(),&None,body)?}else if self.close_index==PERSISTENT_PAGE_FANOUT+LEVELS+1{self.retirement.next_owner_demands::<PersistentList<T>>(self.output.is_some(),&None,body)?}else{return Ok(RetirementDemand::default());};d.copy_bytes=d.copy_bytes.checked_add(Self::close_metadata()).ok_or_else(||refusal("original persistent close quote overflow"))?;d.depth=d.depth.max(1);Ok(d)
 }
 /// 🚚️ Cancellation hands each original child into the existing caller-funded retirement frontier.
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if !self.closing{return Err(refusal("original persistent closure has not started"));}let d=self.close_demand(grant.maximum_copy_bytes)?;if !fits(d,grant){return Ok(RetainedCloneStep::Progress(Default::default()));}if !self.retirement.is_empty(){return self.retirement.step_granted(grant).map(|s|RetainedCloneStep::Progress(s.progress()));}if self.close_index>=PERSISTENT_PAGE_FANOUT+LEVELS+2{return Ok(RetainedCloneStep::Complete(Default::default()));}
  let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-Self::close_metadata(),..grant};let step=if self.close_index<PERSISTENT_PAGE_FANOUT{self.retirement.begin_granted(&mut self.rows[self.close_index],child)?}else if self.close_index<PERSISTENT_PAGE_FANOUT+LEVELS{let index=self.close_index-PERSISTENT_PAGE_FANOUT;self.retirement.begin_granted(&mut self.levels[index],child)?}else if self.close_index==PERSISTENT_PAGE_FANOUT+LEVELS{self.retirement.begin_granted(&mut self.carry,child)?}else{self.retirement.begin_granted(&mut self.output,child)?};let mut p=step.map(|s|s.progress()).unwrap_or(RetainedCloneProgress{copied_items:1,..Default::default()});p.copied_bytes=p.copied_bytes.checked_add(Self::close_metadata()).ok_or_else(||refusal("original persistent close receipt overflow"))?;self.close_index+=1;Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})
 }
 /// 🧹️ Terminal owned custody is determined from real slots and the actual native cleanup frontier.
 pub fn terminal_is_empty(&self)->bool{self.rows.iter().all(Option::is_none)&&self.levels.iter().all(Option::is_none)&&self.carry.is_none()&&self.output.is_none()&&self.retirement.is_empty()}
}
impl<T:Send+Sync+'static> Drop for PersistentListBuilder<T>{fn drop(&mut self){let empty=self.terminal_is_empty();assert!(empty||std::thread::panicking(),"original persistent builder retains owned rows and roots until funded closure");if empty{unsafe{ManuallyDrop::drop(&mut self.rows);ManuallyDrop::drop(&mut self.levels);ManuallyDrop::drop(&mut self.carry);ManuallyDrop::drop(&mut self.output);}}}}
