//! 📶️ Original scalar columns prepare quantile bands through admitted pages and one shared sort frontier per turn.
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind, retained_clone::{RetainedCloneGrant as Grant, RetainedCloneProgress as Progress, RetainedCloneStep as Step}};
use semio_framework_value_persistent::list::PagedList;
use semio_framework_value_edit::{sort::HeapSortCursor, sort::HeapSortWork};
use std::mem::size_of;
const CONTROL:usize=8*size_of::<usize>()+5*size_of::<f64>()+2*size_of::<u8>()+2*size_of::<bool>();
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original scalar rank projection lost its paid frontier")}
fn fits(d:RetirementDemand,g:Grant)->bool{g.maximum_items>0&&g.maximum_copy_bytes>=d.copy_bytes&&g.maximum_capacity_bytes>=d.capacity_bytes&&g.maximum_release_bytes>=d.release_bytes&&g.maximum_depth>=d.depth}
fn scalar(bytes:usize,depth:usize)->RetirementDemand{RetirementDemand{copy_bytes:bytes+CONTROL,depth,..Default::default()}}
fn reserve<T>(list:&PagedList<T,{usize::MAX}>)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{copy_bytes:list.next_reserve_copy_byte_demand().map_err(|_|invalid())?+CONTROL,capacity_bytes:list.next_allocation_bytes().map_err(|_|invalid())?,depth:list.next_reserve_depth_demand().map_err(|_|invalid())?+1,..Default::default()})}
fn child(g:Grant)->Grant{Grant{maximum_copy_bytes:g.maximum_copy_bytes-CONTROL,maximum_depth:g.maximum_depth-1,..g}}
fn receipt(mut p:Progress)->Progress{p.copied_bytes+=CONTROL;p}

/// 🪪️ Read costs and immutable source identity come from the original first-party numeric owner.
pub trait ScalarRankSource{
 fn original_identity(&self)->(usize,u64);
 fn len(&self)->usize;
 fn value_demand(&self,index:usize)->Result<RetirementDemand,ValueError>;
 fn value(&self,index:usize)->Result<f64,ValueError>;
}
#[derive(Clone,Copy,semio_framework_value::RetireOwned)]
struct Entry{index:usize,value:f64,rank:usize}

/// 🎨️ Prepared bands retain an exact paged ordinal lookup and the actual finite extrema.
#[derive(semio_framework_value::RetireOwned)]
pub struct ScalarRankProjection{pub bands:PagedList<u8,{usize::MAX}>,pub minimum:f64,pub maximum:f64}
impl ScalarRankProjection{pub fn band(&self,index:usize)->Option<u8>{self.bands.get(index).copied().filter(|band|*band!=255)}}

/// 🧳️ Allocation, source reads, ordering, tie grouping and result handoff each own a separately rejectable turn.
pub struct ScalarRankCursor{
 values:PagedList<Entry,{usize::MAX}>,bands:PagedList<u8,{usize::MAX}>,sort:HeapSortCursor,
 identity:Option<(usize,u64)>,length:usize,index:usize,distinct:usize,last:f64,minimum:f64,maximum:f64,epsilon:f64,epsilon_factor:f64,band_count:u8,phase:u8,nonzero:bool,closing:bool,
}
impl ScalarRankCursor{
 fn new(epsilon_factor:f64,band_count:u8)->Result<Self,ValueError>{if !epsilon_factor.is_finite()||!(0.0..=1.0).contains(&epsilon_factor)||band_count==0||band_count==255{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"scalar rank policy is outside its finite declared bounds"));}Ok(Self{values:PagedList::new(),bands:PagedList::new(),sort:HeapSortCursor::new(0),identity:None,length:0,index:0,distinct:0,last:0.,minimum:0.,maximum:0.,epsilon:0.,epsilon_factor,band_count,phase:0,nonzero:false,closing:false})}
 pub fn birth_demand()->RetirementDemand{RetirementDemand{copy_bytes:2*size_of::<Self>(),depth:1,..Default::default()}}
 pub fn admit(epsilon_factor:f64,band_count:u8,g:Grant)->Result<Option<(Self,Progress)>,ValueError>{let d=Self::birth_demand();if !fits(d,g){return Ok(None);}Ok(Some((Self::new(epsilon_factor,band_count)?,Progress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})))}
 pub fn complete(&self)->bool{!self.closing&&self.phase==7}
 fn path<T>(&self,list:&PagedList<T,{usize::MAX}>,index:usize)->Result<usize,ValueError>{list.next_get_depth_demand(index).map_err(|_|invalid())}
 pub fn demand(&self,source:&impl ScalarRankSource)->Result<RetirementDemand,ValueError>{
  if self.closing||self.phase>7||self.identity.is_some_and(|id|id!=source.original_identity())||self.phase>0&&self.length!=source.len(){return Err(invalid());}
  Ok(match self.phase{
   0=>scalar(size_of::<(usize,u64)>()+size_of::<usize>(),1),
   1 if self.index==self.length=>scalar(0,1),
   1 if self.bands.len()==self.bands.capacity()=>return reserve(&self.bands),
   1=>scalar(1,self.bands.next_push_depth_demand().map_err(|_|invalid())?+1),
   2 if self.index==self.length=>scalar(size_of::<HeapSortCursor>(),1),
   2 if self.values.len()==self.values.capacity()=>return reserve(&self.values),
   2=>{let mut d=source.value_demand(self.index)?;d.copy_bytes+=CONTROL+2*size_of::<Entry>();d.depth+=self.values.next_push_depth_demand().map_err(|_|invalid())?+1;d},
   3=>match self.sort.next(){HeapSortWork::Compare{left,right}|HeapSortWork::Swap{left,right}=>scalar(4*size_of::<Entry>()+size_of::<HeapSortCursor>(),self.path(&self.values,left)?+self.path(&self.values,right)?+1),_=>scalar(size_of::<HeapSortCursor>(),1)},
   4 if self.values.is_empty()=>scalar(0,1),
   4=>scalar(2*size_of::<Entry>(),self.path(&self.values,0)?+self.path(&self.values,self.values.len()-1)?+1),
   5 if self.index==self.values.len()=>scalar(0,1),
   5=>scalar(3*size_of::<Entry>(),2*self.path(&self.values,self.index)?+1),
   6 if self.index==self.values.len()=>scalar(0,1),
   6=>{let entry=self.values.get(self.index).ok_or_else(invalid)?;scalar(size_of::<Entry>()+1,self.path(&self.values,self.index)?+self.path(&self.bands,entry.index)?+1)},
   7=>Default::default(),_=>return Err(invalid()),
  })
 }
 pub fn advance(&mut self,source:&impl ScalarRankSource,g:Grant)->Result<Step,ValueError>{
  let d=self.demand(source)?;if !fits(d,g){return Ok(Step::Progress(Default::default()));}if self.complete(){return Ok(Step::Complete(Default::default()));}
  let p=Progress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};
  match self.phase{
   0=>{self.identity=Some(source.original_identity());self.length=source.len();self.phase=1;},
   1 if self.index==self.length=>{self.index=0;self.phase=2;},
   1 if self.bands.len()==self.bands.capacity()=>return self.bands.reserve_one_funded(child(g)).map(|p|Step::Progress(receipt(p))).map_err(|_|invalid()),
   1=>{self.bands.push_reserved(255).map_err(|_|invalid())?;self.index+=1;},
   2 if self.index==self.length=>{if !self.nonzero{self.phase=7;}else{self.sort=HeapSortCursor::new(self.values.len());self.phase=3;}},
   2 if self.values.len()==self.values.capacity()=>return self.values.reserve_one_funded(child(g)).map(|p|Step::Progress(receipt(p))).map_err(|_|invalid()),
   2=>{let value=source.value(self.index)?;if value.is_finite(){self.nonzero|=value!=0.;self.values.push_reserved(Entry{index:self.index,value,rank:0}).map_err(|_|invalid())?;}self.index+=1;},
   3=>match self.sort.next(){HeapSortWork::Compare{left,right}=>{let a=self.values.get(left).ok_or_else(invalid)?;let b=self.values.get(right).ok_or_else(invalid)?;self.sort.advance(Some(a.value.total_cmp(&b.value).then(a.index.cmp(&b.index)))).map_err(|_|invalid())?;},HeapSortWork::Swap{left,right}=>{self.values.swap(left,right);self.sort.advance(None).map_err(|_|invalid())?;},HeapSortWork::Advance=>self.sort.advance(None).map_err(|_|invalid())?,HeapSortWork::Complete=>{self.phase=4;}},
   4=>{if !self.values.is_empty(){self.minimum=self.values.get(0).ok_or_else(invalid)?.value;self.maximum=self.values.get(self.values.len()-1).ok_or_else(invalid)?.value;self.epsilon=(self.maximum*self.epsilon_factor-self.minimum*self.epsilon_factor).abs();}self.index=0;self.phase=5;},
   5 if self.index==self.values.len()=>{self.index=0;self.phase=6;},
   5=>{let entry=self.values.get_mut(self.index).ok_or_else(invalid)?;if self.distinct==0||(entry.value-self.last).abs()>self.epsilon{self.last=entry.value;self.distinct+=1;}entry.rank=self.distinct-1;self.index+=1;},
   6 if self.index==self.values.len()=>{self.phase=7;},
   6=>{let entry=*self.values.get(self.index).ok_or_else(invalid)?;if self.minimum!=0.||self.maximum!=0.{let last=self.band_count-1;let band=if self.distinct<2{0}else{((entry.rank as f64*last as f64/(self.distinct-1)as f64).round()as u8).min(last)};*self.bands.get_mut(entry.index).ok_or_else(invalid)?=band;}self.index+=1;},
   _=>return Err(invalid()),
  }
  Ok(if self.complete(){Step::Complete(p)}else{Step::Progress(p)})
 }
 pub fn take_demand(&self)->RetirementDemand{scalar(2*size_of::<ScalarRankProjection>(),1)}
 pub fn take(&mut self,g:Grant)->Result<Option<(ScalarRankProjection,Progress)>,ValueError>{if !self.complete(){return Err(invalid());}let d=self.take_demand();if !fits(d,g){return Ok(None);}self.phase=8;let bands=std::mem::take(&mut self.bands);Ok(Some((ScalarRankProjection{bands,minimum:self.minimum,maximum:self.maximum},Progress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})))}
 pub fn begin_close(&mut self){self.closing=true;}
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.values.terminal_is_empty()&&self.bands.terminal_is_empty()}
 pub fn close_demand(&self)->Result<RetirementDemand,ValueError>{
  if !self.values.is_empty(){return Ok(scalar(size_of::<Entry>(),self.values.next_pop_depth_demand().map_err(|_|invalid())?+1));}
  if !self.values.terminal_is_empty(){return Ok(RetirementDemand{copy_bytes:CONTROL,release_bytes:self.values.next_release_allocation_bytes().map_err(|_|invalid())?,depth:self.values.next_release_depth_demand().map_err(|_|invalid())?+1,..Default::default()});}
  if !self.bands.is_empty(){return Ok(scalar(1,self.bands.next_pop_depth_demand().map_err(|_|invalid())?+1));}
  if !self.bands.terminal_is_empty(){return Ok(RetirementDemand{copy_bytes:CONTROL,release_bytes:self.bands.next_release_allocation_bytes().map_err(|_|invalid())?,depth:self.bands.next_release_depth_demand().map_err(|_|invalid())?+1,..Default::default()});}
  Ok(Default::default())
 }
 pub fn close_step(&mut self,g:Grant)->Result<Step,ValueError>{
  if !self.closing{return Err(invalid());}let d=self.close_demand()?;if !fits(d,g){return Ok(Step::Progress(Default::default()));}if self.terminal_is_empty(){return Ok(Step::Complete(Default::default()));}
  let mut p=Progress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};
  if !self.values.is_empty(){self.values.pop();}else if !self.values.terminal_is_empty(){p.released_bytes=self.values.release_empty_page(g.maximum_release_bytes).map_err(|_|invalid())?.released_bytes;}else if !self.bands.is_empty(){self.bands.pop();}else{p.released_bytes=self.bands.release_empty_page(g.maximum_release_bytes).map_err(|_|invalid())?.released_bytes;}
  Ok(if self.terminal_is_empty(){Step::Complete(p)}else{Step::Progress(p)})
 }
}
impl Drop for ScalarRankCursor{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"scalar rank preparation must close all paid original pages before drop");}}
#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;

#[cfg(test)]
#[path="../../⏱️trace/🧮️memory/🧪️testing/📥️requests/🦀️.rs"]
mod rank_allocation;
#[cfg(test)]
#[global_allocator]
static RANK_ALLOCATOR:rank_allocation::RequestedAllocator=rank_allocation::RequestedAllocator;
