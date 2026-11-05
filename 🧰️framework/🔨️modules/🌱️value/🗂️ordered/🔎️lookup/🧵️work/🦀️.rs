//! 🔎️ Original lookup construction and byte comparisons expose their selected receipts.
use super::*;
use std::mem::size_of;
impl<V> OrderedMap<V>{
 pub fn lookup_constructor_demands(&self)->crate::RetirementDemand{crate::RetirementDemand{copy_bytes:4*size_of::<Self>(),depth:1+2*usize::from(!self.is_empty()),..Default::default()}}
 pub fn admit_lookup_original(&self,key:SharedOwner<String>,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<(LookupCursor<V>,RetainedCloneProgress),(ValueError,SharedOwner<String>)>{
  let demand=self.lookup_constructor_demands();if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_depth<demand.depth{return crate::RetainedWorkOutcome{result:Err((ValueError::literal(crate::ValueRefusalKind::WorkLimit,"original ordered lookup requires its quoted metadata transfer"),key)),work:0}}
  crate::RetainedWorkOutcome{result:Ok((self.begin_lookup_shared(key),RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})),work:1}
 }
}
impl<V> LookupCursor<V>{
 pub fn next_original_demands(&self)->Result<crate::RetirementDemand,ValueError>{self.next_original_demands_selected(None)}
 pub fn advance_original(&mut self,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome{self.advance_original_selected(None,grant)}
}
