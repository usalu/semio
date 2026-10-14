//! 🧭️ Original graph planning retains borrowed identities and funds each comparison and scalar page event.
use super::{InferenceWorkFailure,InferredField};
use semio_framework_job::{OriginalTurnFailure,StepContext};
use semio_framework_value::{list::PagedList,RetainedCloneProgress,RetirementDemand,ValueError,ValueRefusalKind};

#[derive(Default,semio_framework_value::RetireOwned)]
pub struct OriginalInferencePlan{
 nodes:PagedList<usize,{usize::MAX}>,
 degrees:PagedList<usize,{usize::MAX}>,
 edges:PagedList<(usize,usize),{usize::MAX}>,
 queue:PagedList<usize,{usize::MAX}>,
 order:PagedList<usize,{usize::MAX}>,
 phase:u8,source:usize,scan:usize,byte:usize,from:usize,to:usize,head:usize,child:usize,edge:usize,cyclic_start:usize,closing:bool,
}

impl OriginalInferencePlan{
 pub fn len(&self)->usize{self.order.len()}
 pub fn is_empty(&self)->bool{self.order.is_empty()}
 pub fn complete(&self)->bool{self.phase==13}
 pub fn source_ordinal(&self,order:usize)->Option<usize>{self.order.get(order).and_then(|node|self.nodes.get(*node)).copied()}
 pub fn edge_count(&self)->usize{self.edges.len()}
 pub fn cyclic_start(&self)->usize{self.cyclic_start}
 pub fn edge_source_ordinals(&self,index:usize)->Option<(usize,usize)>{let(from,to)=*self.edges.get(index)?;Some((*self.nodes.get(from)?,*self.nodes.get(to)?))}
 /// 🔎️ Quotes the same next borrowed probe or reserved scalar event without advancing its source.
 pub fn demands<P,F:InferredField<P>>(&self,snapshot:&P)->Result<RetirementDemand,ValueError>{
  let(nodes,links)=F::plan_source_extent(snapshot).ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original inferred field plan has no borrowed graph producer"))?;
  match self.phase{
   0 if self.source<nodes&&self.scan==self.nodes.len()=>append_demand(&self.nodes,nodes),
   1=>append_demand(&self.degrees,nodes),
   4 if self.scan==self.edges.len()=>append_demand(&self.edges,links),
   5=>append_demand(&self.edges,links),6|10=>Ok(RetirementDemand{copy_bytes:size_of::<usize>(),depth:1,..Default::default()}),
   7 if self.scan<self.nodes.len()&&self.degrees.get(self.scan)==Some(&0)=>append_demand(&self.queue,self.nodes.len()),
   8 if self.head<self.queue.len()=>append_demand(&self.order,self.nodes.len()),
   11 if self.child<self.nodes.len()&&self.scan==self.cyclic_start=>append_demand(&self.order,self.nodes.len()),
   12=>append_demand(&self.queue,self.nodes.len()),13=>Ok(RetirementDemand::default()),_=>Ok(RetirementDemand{depth:1,..Default::default()}),
  }
 }
 #[cfg(test)]
 pub(crate) fn position(&self)->[usize;16]{[self.phase as usize,self.source,self.scan,self.byte,self.from,self.to,self.head,self.child,self.edge,self.cyclic_start,self.nodes.len(),self.degrees.len(),self.edges.len(),self.queue.len(),self.order.len(),usize::from(self.closing)]}
 /// 📏️ Quotes one original close event without returning a page or discarding its scalar rows.
 pub fn close_demands(&self)->Result<RetirementDemand,ValueError>{
  if !self.closing{return Ok(RetirementDemand{depth:1,..Default::default()})}
  if !self.order.terminal_is_empty(){return close_demand(&self.order)}
  if !self.queue.terminal_is_empty(){return close_demand(&self.queue)}
  if !self.edges.terminal_is_empty(){return close_demand(&self.edges)}
  if !self.degrees.terminal_is_empty(){return close_demand(&self.degrees)}
  if !self.nodes.terminal_is_empty(){return close_demand(&self.nodes)}
  Ok(RetirementDemand::default())
 }
 /// ♻️ Retires at most one original scalar row or exact backing allocation, including cancelled callers.
 pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,InferenceWorkFailure>{
  if cx.retirement_should_yield(){return Ok(false)}
  if !self.closing{if !metadata(cx)?{return Ok(false)}self.closing=true;return Ok(false)}
  if !self.order.terminal_is_empty(){close(&mut self.order,cx)?;return Ok(false)}
  if !self.queue.terminal_is_empty(){close(&mut self.queue,cx)?;return Ok(false)}
  if !self.edges.terminal_is_empty(){close(&mut self.edges,cx)?;return Ok(false)}
  if !self.degrees.terminal_is_empty(){close(&mut self.degrees,cx)?;return Ok(false)}
  if !self.nodes.terminal_is_empty(){close(&mut self.nodes,cx)?;return Ok(false)}
  Ok(true)
 }
 pub fn terminal_is_empty(&self)->bool{self.nodes.terminal_is_empty()&&self.degrees.terminal_is_empty()&&self.edges.terminal_is_empty()&&self.queue.terminal_is_empty()&&self.order.terminal_is_empty()}
 /// 🎟️ Advances one defining graph event under the caller's original context.
 pub fn step<P,F:InferredField<P>>(&mut self,snapshot:&P,cx:&mut StepContext<'_>)->Result<bool,InferenceWorkFailure>{
  let before=cx.fuel_remaining();self.advance::<P,F>(snapshot,cx)?;Ok(cx.fuel_remaining()<before)
 }
 fn advance<P,F:InferredField<P>>(&mut self,snapshot:&P,cx:&mut StepContext<'_>)->Result<bool,InferenceWorkFailure>{
  if self.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original inferred graph planning is closing").into())}
  if cx.is_cancelled()||cx.should_yield(){return Ok(false)}
  let(nodes,links)=F::plan_source_extent(snapshot).ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original inferred field plan has no borrowed graph producer"))?;
  let node=|source|F::plan_source_node(snapshot,source).ok_or_else(source_changed);
  match self.phase{
   0=>{
    if self.source==nodes{if !metadata(cx)?{return Ok(false)}self.phase=2;self.source=0;self.scan=0;return Ok(true)}
    if self.scan==self.nodes.len(){if append(&mut self.nodes,self.source,nodes,cx)?{self.phase=1}return Ok(true)}
    let a=node(self.source)?;let b=node(*self.nodes.get(self.scan).ok_or_else(source_changed)?)?;
    match compare(a,b,&mut self.byte,cx)?{Some(true)=>{self.source+=1;self.scan=0;self.byte=0},Some(false)=>{self.scan+=1;self.byte=0},None=>{}}
    Ok(true)
   }
   1=>{
    if !append(&mut self.degrees,0,nodes,cx)?{return Ok(false)}self.source+=1;self.scan=0;self.phase=0;Ok(true)
   }
   2|3=>{
    if self.source==links{if !metadata(cx)?{return Ok(false)}self.phase=7;self.scan=0;return Ok(true)}
    if self.scan==self.nodes.len(){if !metadata(cx)?{return Ok(false)}self.source+=1;self.scan=0;self.byte=0;self.phase=2;return Ok(true)}
    let(from,to)=F::plan_source_link(snapshot,self.source).ok_or_else(source_changed)?;
    let target=if self.phase==2{from}else{to};let candidate=node(*self.nodes.get(self.scan).ok_or_else(source_changed)?)?;
    match compare(target,candidate,&mut self.byte,cx)?{
     Some(true)=>{if self.phase==2{self.from=self.scan;self.phase=3}else{self.to=self.scan;self.phase=4}self.scan=0;self.byte=0},
     Some(false)=>{self.scan+=1;self.byte=0},None=>{}
    }Ok(true)
   }
   4=>{
    if self.scan==self.edges.len(){if append(&mut self.edges,(self.from,self.to),links,cx)?{self.phase=6}return Ok(true)}
    if !metadata(cx)?{return Ok(false)}
    if self.edges.get(self.scan)==Some(&(self.from,self.to)){self.source+=1;self.scan=0;self.phase=2}else{self.scan+=1}Ok(true)
   }
   5=>{if !append(&mut self.edges,(self.from,self.to),links,cx)?{return Ok(false)}self.phase=6;Ok(true)}
   6=>{
    if !scalar(cx,size_of::<usize>())?{return Ok(false)}
    let degree=self.degrees.get_mut(self.to).ok_or_else(source_changed)?;*degree=degree.checked_add(1).ok_or_else(source_changed)?;
    self.source+=1;self.scan=0;self.phase=2;Ok(true)
   }
   7=>{
    if self.scan==self.nodes.len(){if !metadata(cx)?{return Ok(false)}self.phase=8;return Ok(true)}
    if self.degrees.get(self.scan)==Some(&0){if !append(&mut self.queue,self.scan,self.nodes.len(),cx)?{return Ok(false)}}else if !metadata(cx)?{return Ok(false)}
    self.scan+=1;Ok(true)
   }
   8=>{
    if self.head==self.queue.len(){if !metadata(cx)?{return Ok(false)}self.cyclic_start=self.order.len();self.phase=11;self.child=0;self.scan=0;return Ok(true)}
    let current=*self.queue.get(self.head).ok_or_else(source_changed)?;
    if !append(&mut self.order,current,self.nodes.len(),cx)?{return Ok(false)}self.head+=1;self.from=current;self.child=0;self.edge=0;self.phase=9;Ok(true)
   }
   9=>{
    if self.child==self.nodes.len(){if !metadata(cx)?{return Ok(false)}self.phase=8;return Ok(true)}
    if self.edge==self.edges.len(){if !metadata(cx)?{return Ok(false)}self.child+=1;self.edge=0;return Ok(true)}
    if !metadata(cx)?{return Ok(false)}
    if self.edges.get(self.edge)==Some(&(self.from,self.child)){self.phase=10}else{self.edge+=1}Ok(true)
   }
   10=>{
    if !scalar(cx,size_of::<usize>())?{return Ok(false)}let degree=self.degrees.get_mut(self.child).ok_or_else(source_changed)?;
    *degree=degree.checked_sub(1).ok_or_else(source_changed)?;self.phase=if *degree==0{12}else{9};self.edge+=1;Ok(true)
   }
   11=>{
    if self.child==self.nodes.len(){if !metadata(cx)?{return Ok(false)}self.phase=13;return Ok(true)}
    if self.scan==self.cyclic_start{if !append(&mut self.order,self.child,self.nodes.len(),cx)?{return Ok(false)}self.child+=1;self.scan=0;return Ok(true)}
    if !metadata(cx)?{return Ok(false)}
    if self.order.get(self.scan)==Some(&self.child){self.child+=1;self.scan=0}else{self.scan+=1}Ok(true)
   }
   12=>{if !append(&mut self.queue,self.child,self.nodes.len(),cx)?{return Ok(false)}self.phase=9;Ok(true)}
   13=>Ok(false),_=>Err(source_changed().into()),
  }
 }
}

fn source_changed()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original inference graph ordinal changed")}
fn metadata(cx:&mut StepContext<'_>)->Result<bool,InferenceWorkFailure>{scalar(cx,0)}
fn scalar(cx:&mut StepContext<'_>,bytes:usize)->Result<bool,InferenceWorkFailure>{
 let grant=cx.retained_grant();if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<bytes{return Ok(false)}
 cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);Ok(true)
}
fn compare(a:&str,b:&str,byte:&mut usize,cx:&mut StepContext<'_>)->Result<Option<bool>,InferenceWorkFailure>{
 if !metadata(cx)?{return Ok(None)}
 if a.len()!=b.len(){return Ok(Some(false))}if *byte==a.len(){return Ok(Some(true))}
 if a.as_bytes()[*byte]!=b.as_bytes()[*byte]{return Ok(Some(false))}*byte+=1;Ok(None)
}
fn append<T:Copy>(target:&mut PagedList<T,{usize::MAX}>,value:T,extent:usize,cx:&mut StepContext<'_>)->Result<bool,InferenceWorkFailure>{
 if !target.has_reserved_slot(){
  match target.reserve_capacity_one_funded(extent,cx.retained_grant()){
   Ok(receipt)=>{if receipt.copied_items==0{return Ok(false)}cx.consume_fuel(1);cx.consume_retained(receipt)?;return Ok(false)},
   Err(error)=>{cx.consume_fuel(1);let original=ValueError::from(error.refusal()).with_retained_progress(RetainedCloneProgress{retained_capacity_bytes:error.allocated_bytes,..Default::default()});return Err(OriginalTurnFailure::debit_original(original,cx).into())}
  }
 }
 if cx.retained_grant().maximum_depth<target.next_push_depth_demand().map_err(ValueError::from)?||!scalar(cx,size_of::<T>())?{return Ok(false)}
 target.push_reserved(value).map_err(|_|source_changed())?;Ok(true)
}

fn close<T:Copy>(target:&mut PagedList<T,{usize::MAX}>,cx:&mut StepContext<'_>)->Result<(),InferenceWorkFailure>{
 let grant=cx.retained_grant();
 if !target.is_empty(){if grant.maximum_depth<target.next_pop_depth_demand().map_err(ValueError::from)?||!scalar(cx,size_of::<T>())?{return Ok(())}target.pop();return Ok(())}
 let bytes=target.next_release_allocation_bytes().map_err(ValueError::from)?;let depth=target.next_release_depth_demand().map_err(ValueError::from)?;
 if grant.maximum_items==0||grant.maximum_depth<depth||grant.maximum_release_bytes<bytes{return Ok(())}
 let released=target.release_empty_page(grant.maximum_release_bytes).map_err(ValueError::from)?;
 if released.progressed{cx.consume_fuel(1);cx.consume_retained(RetainedCloneProgress{copied_items:1,released_bytes:released.released_allocation_bytes,..Default::default()})?}Ok(())
}

fn append_demand<T>(target:&PagedList<T,{usize::MAX}>,extent:usize)->Result<RetirementDemand,ValueError>{
 if target.has_reserved_slot(){return Ok(RetirementDemand{copy_bytes:size_of::<T>(),depth:target.next_push_depth_demand().map_err(ValueError::from)?,..Default::default()})}
 Ok(RetirementDemand{copy_bytes:target.next_capacity_copy_byte_demand(extent).map_err(ValueError::from)?,capacity_bytes:target.next_capacity_allocation_bytes(extent).map_err(ValueError::from)?.unwrap_or(0),depth:target.next_reserve_depth_demand().map_err(ValueError::from)?,..Default::default()})
}

fn close_demand<T>(target:&PagedList<T,{usize::MAX}>)->Result<RetirementDemand,ValueError>{
 if !target.is_empty(){return Ok(RetirementDemand{copy_bytes:size_of::<T>(),depth:target.next_pop_depth_demand().map_err(ValueError::from)?,..Default::default()})}
 Ok(RetirementDemand{release_bytes:target.next_release_allocation_bytes().map_err(ValueError::from)?,depth:target.next_release_depth_demand().map_err(ValueError::from)?,..Default::default()})
}
