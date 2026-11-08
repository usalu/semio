//! 👁️ The cache lends one immutable dictionary root after a bounded original numeric index probe.
use super::*;
use semio_framework_replication_history_index::HistoryFoldIndexRead;
use semio_framework_value::{RetainedWorkOutcome,retained_clone::retained_clone_grant_admits};
use std::mem::size_of;
#[derive(Default)]
pub struct NeuralCacheRead{probe:HistoryFoldIndexRead,binding:Option<(usize,u64)>,ready:bool,done:bool}
pub struct NeuralCacheReadAdvance{pub value:Option<Option<Dictionary>>,pub progress:RetainedCloneProgress}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original cached output probe changed its cache, hash or recipient")}
fn compare_quote(_: &u64,_: &u64,_:usize,_:usize)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{copy_bytes:2*size_of::<u64>(),depth:1,..Default::default()})}
fn compare(left:&u64,right:&u64,_:usize,g:RetainedCloneGrant)->Result<(Option<std::cmp::Ordering>,usize,RetainedCloneProgress),ValueError>{let d=compare_quote(left,right,0,0)?;if !retained_clone_grant_admits(g,d){return Ok((None,0,Default::default()))}Ok((Some(left.cmp(right)),0,RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}
impl NeuralCacheRead{
 pub fn new()->Self{Self::default()}
 pub fn next_demands(&self,cache:&NeuralCache,hash:u64,copy:usize)->Result<RetirementDemand,ValueError>{
  if self.done||self.binding.is_some_and(|binding|binding!=(cache as*const _ as usize,hash)){return Err(invalid())}let Ok(state)=cache.state.try_lock()else{return Ok(RetirementDemand{depth:1,..Default::default()})};
  if self.ready&&self.probe.matched(&state.entries).is_ok(){return Ok(RetirementDemand{copy_bytes:2*size_of::<Option<Dictionary>>()+2*size_of::<Self>(),depth:1,..Default::default()})}let mut d=self.probe.next_demands(&state.entries,&hash,copy,compare_quote)?;d.copy_bytes=d.copy_bytes.checked_add(2*size_of::<Self>()).ok_or_else(invalid)?;Ok(d)
 }
 pub fn advance(&mut self,cache:&NeuralCache,hash:u64,g:RetainedCloneGrant)->RetainedWorkOutcome<NeuralCacheReadAdvance>{
  let mut progress=RetainedCloneProgress::default();if g.maximum_items==0||g.maximum_depth==0{return RetainedWorkOutcome{result:Ok(NeuralCacheReadAdvance{value:None,progress}),work:0}}let result=(||{if self.done||self.binding.is_some_and(|binding|binding!=(cache as*const _ as usize,hash)){return Err(invalid())}let Ok(mut state)=cache.state.try_lock()else{return Ok(NeuralCacheReadAdvance{value:None,progress})};
   if self.ready{if let Ok(matched)=self.probe.matched(&state.entries){let d=RetirementDemand{copy_bytes:2*size_of::<Option<Dictionary>>()+2*size_of::<Self>(),depth:1,..Default::default()};if !retained_clone_grant_admits(g,d){return Ok(NeuralCacheReadAdvance{value:None,progress})}let value=if let Some(matched)=matched{let entry=matched.value_mut(&mut state.entries)?;entry.0=cache.current_epoch();Some(entry.1.clone())}else{None};self.done=true;progress=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};return Ok(NeuralCacheReadAdvance{value:Some(value),progress})}}
   let header=2*size_of::<Self>();if g.maximum_copy_bytes<header{return Ok(NeuralCacheReadAdvance{value:None,progress})}let(found,mut p)=match self.probe.step(&state.entries,&hash,RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-header,..g},compare){Ok(result)=>result,Err(error)=>{progress=error.retained_progress();if progress.copied_items!=0{progress.copied_bytes=progress.copied_bytes.checked_add(header).ok_or_else(invalid)?}return Err(error)}};if p.copied_items!=0{p.copied_bytes+=header}progress=p;if p.copied_items!=0{self.binding=Some((cache as*const _ as usize,hash));self.ready=found.is_some()}Ok(NeuralCacheReadAdvance{value:None,progress})
  })();RetainedWorkOutcome{work:u128::from(progress.copied_items!=0),result:result.map_err(|error|error.with_retained_progress(progress))}
 }
}

#[cfg(test)]
#[path="🧪️read.rs"]
mod tests;
