//! 🪞️ Original ordered lookup can borrow a query retained by its real immutable caller owner.
use super::*;
fn invalid()->ValueError{ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"ordered lookup requires its original immutable query")}
fn select_query<'a>(owned:Option<&'a SharedOwner<String>>,binding:Option<(usize,usize)>,borrowed:Option<&'a str>)->Result<&'a str,ValueError>{
 match(owned,binding,borrowed){(Some(key),None,None)=>Ok(key.as_str()),(None,Some(identity),Some(query))if identity==(query.as_ptr()as usize,query.len())=>Ok(query),_=>Err(invalid())}
}
impl<V> LookupState<V>{
 pub(super) fn selected_query<'a>(&'a self,query:Option<&'a str>)->Result<&'a str,ValueError>{select_query(self.key.as_ref(),self.borrowed_query,query)}
 pub(super) fn advance_selected(&mut self,query:Option<&str>,grant:Grant)->Result<Step,ValueError>{
  if self.closing{return Ok(Step::Blocked)}
  let query=select_query(self.key.as_ref(),self.borrowed_query,query)?;
  if grant.maximum_items==0||grant.maximum_bytes==0{return Ok(Step::Blocked)}if self.complete{return Ok(Step::Complete)}
  let mut bytes=0;
  if self.current.is_none(){self.complete=true}
  else if self.ordering.is_none(){bytes=compare_bytes(query.as_bytes(),self.current.as_ref().unwrap().entry.key.as_bytes(),&mut self.offset,&mut self.left_byte,&mut self.ordering,grant.maximum_bytes)}
  else if self.ordering==Some(Ordering::Equal){self.rank+=len(&self.current.as_ref().unwrap().left);self.complete=true}
  else{let current=self.current.take().unwrap();self.current=if self.ordering==Some(Ordering::Less){current.left.clone()}else{self.rank+=len(&current.left)+1;current.right.clone()};self.offset=0;self.left_byte=None;self.ordering=None}
  Ok(Step::Progress{completed_items:1,completed_bytes:bytes})
 }
}
impl<V> OrderedMap<V>{
 /// 📏️ Quotes the actual two retained roots and borrowed query identity words.
 pub fn lookup_borrowed_constructor_demands(&self)->crate::RetirementDemand{self.lookup_constructor_demands()}
 /// 🧷️ The caller retains its real immutable query owner and supplies this same borrow each turn.
 pub fn admit_lookup_borrowed_original(&self,query:&str,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<(LookupCursor<V>,RetainedCloneProgress)>{
  let demand=self.lookup_borrowed_constructor_demands();if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_depth<demand.depth{return crate::RetainedWorkOutcome{result:Err(ValueError::literal(crate::ValueRefusalKind::WorkLimit,"original borrowed ordered lookup requires its metadata quote")),work:0}}
  let base=self.clone();let current=(*base.root).clone();
  let cursor=LookupCursor{state:ManuallyDrop::new(LookupState{base,key:None,borrowed_query:Some((query.as_ptr()as usize,query.len())),current,rank:0,offset:0,left_byte:None,ordering:None,complete:false,closing:false,retirement:Retirement::default(),last_close_work:0})};
  crate::RetainedWorkOutcome{result:Ok((cursor,RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})),work:1}
 }
}
impl<V> LookupCursor<V>{
 /// 📍️ Reads the matched rank retained by the actual completed navigation event.
 pub fn result_rank(&self)->Option<usize>{(self.state.complete&&self.state.ordering==Some(Ordering::Equal)).then_some(self.state.rank)}
 /// 🎟️ Requires the exact borrowed query still held by the original immutable caller.
 pub fn next_borrowed_original_demands(&self,query:&str)->Result<crate::RetirementDemand,ValueError>{self.next_original_demands_selected(Some(query))}
 /// 🔎️ Drives the same defining ordered comparison and roots without cloning the query.
 pub fn advance_borrowed_original(&mut self,query:&str,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome{self.advance_original_selected(Some(query),grant)}
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

impl<V> LookupCursor<V>{
 pub(super) fn next_original_demands_selected(&self,query:Option<&str>)->Result<crate::RetirementDemand,ValueError>{
  if self.state.closing{return Err(ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"original ordered lookup is closing"))}
  let query=self.state.selected_query(query)?;if self.state.complete{return Ok(Default::default())}
  let bytes=usize::from(self.state.current.as_ref().is_some_and(|node|self.state.ordering.is_none()&&self.state.offset<query.len().min(node.entry.key.len())));
  Ok(crate::RetirementDemand{copy_bytes:bytes,depth:self.state.retained_depth().max(1),..Default::default()})
 }
 pub(super) fn advance_original_selected(&mut self,query:Option<&str>,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome{
  let demand=match self.next_original_demands_selected(query){Ok(demand)=>demand,Err(error)=>return crate::RetainedWorkOutcome{result:Err(error),work:0}};
  if self.state.complete{return crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}
  if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_depth<demand.depth{return crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
  let step=match self.state.advance_selected(query,Grant{maximum_items:1,maximum_bytes:1}){Ok(step)=>step,Err(error)=>return crate::RetainedWorkOutcome{result:Err(error),work:0}};
  match step{
   Step::Blocked=>crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0},
   Step::Complete=>crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0},
   Step::Progress{completed_items,completed_bytes}=>{let progress=RetainedCloneProgress{copied_items:completed_items,copied_bytes:completed_bytes,..Default::default()};crate::RetainedWorkOutcome{result:Ok(if self.state.complete{RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)}),work:completed_items as u128}},
  }
 }
}
