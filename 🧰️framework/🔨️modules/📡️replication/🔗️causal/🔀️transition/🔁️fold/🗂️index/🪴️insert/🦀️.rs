//! 🪴️ Original row custody survives bounded comparison, backing admission and exact slot transfer.
use super::*;

/// 🧳️ Owns the exact incoming row while its vacant AVL rank is admitted under caller grants.
pub struct HistoryFoldIndexInsert<K,V>{source:Option<(K,V)>,epoch:Option<u64>,next:Option<usize>,rank:usize,position:usize,vacant:bool,slot:Option<usize>}
impl<K,V> HistoryFoldIndexInsert<K,V>{
 pub fn new(key:K,value:V)->Self{Self{source:Some((key,value)),epoch:None,next:None,rank:0,position:0,vacant:false,slot:None}}
 pub fn original(&self)->Option<(&K,&V)>{self.source.as_ref().map(|(key,value)|(key,value))}
 pub fn terminal_is_empty(&self)->bool{self.source.is_none()}
 /// 📤️ Returns the exact uncommitted row headers without a payload copy or heap effect.
 pub fn return_original(&mut self,grant:RetainedCloneGrant)->Option<((K,V),RetainedCloneProgress)>{if grant.maximum_items==0||grant.maximum_depth==0{return None}self.source.take().map(|source|(source,RetainedCloneProgress{copied_items:1,..Default::default()}))}
}
impl<K:Ord,V> HistoryFoldIndexInsert<K,V>{
 /// 🪙️ Quotes the same retained comparison, page admission or vacant-rank commit selected by the incoming body.
 pub fn next_demands<W>(&self,index:&HistoryFoldIndex<K,W>,body:usize,compare:impl FnOnce(&K,&K,usize,usize)->Result<semio_framework_value::RetirementDemand,ValueError>)->Result<semio_framework_value::RetirementDemand,ValueError>{
  use semio_framework_value::RetirementDemand;
  if self.source.is_none(){return Ok(Default::default())}
  if self.epoch!=Some(index.mutation_epoch){return Ok(RetirementDemand{depth:1,..Default::default()})}
  if !self.vacant{let Some(slot)=self.next else{return Ok(RetirementDemand{depth:1,..Default::default()})};let(candidate,_)=index.nodes[slot].entry.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original insertion quote lost its live row"))?;let mut d=compare(&self.source.as_ref().unwrap().0,candidate,self.position,body)?;d.depth=d.depth.max(index.next_slot_depth_demand(slot)?);return Ok(d)}
  Ok(RetirementDemand{capacity_bytes:self.capacity_demand(index)?,depth:self.depth_demand(index)?,..Default::default()})
 }
 /// 🔎️ Examines one paid key chunk or admits one real backing page while retaining the whole source.
 pub fn step<W>(&mut self,index:&mut HistoryFoldIndex<K,W>,grant:RetainedCloneGrant,compare:impl FnOnce(&K,&K,usize,RetainedCloneGrant)->Result<(Option<Ordering>,usize,RetainedCloneProgress),ValueError>)->Result<(bool,RetainedCloneProgress),ValueError>{
  let empty=RetainedCloneProgress::default();if self.source.is_none(){return Ok((true,empty))}if grant.maximum_items==0||grant.maximum_depth==0{return Ok((false,empty))}let item=RetainedCloneProgress{copied_items:1,..empty};
  if self.epoch!=Some(index.mutation_epoch){self.epoch=Some(index.mutation_epoch);self.next=index.root;self.rank=0;self.position=0;self.vacant=false;return Ok((false,item))}
  if !self.vacant{
   let Some(slot)=self.next else{self.vacant=true;return Ok((false,item))};if grant.maximum_depth<index.next_slot_depth_demand(slot)?{return Ok((false,empty))}
   let node=&index.nodes[slot];let(candidate,_)=node.entry.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original insertion search lost its live row"))?;
   let(order,position,progress)=compare(&self.source.as_ref().unwrap().0,candidate,self.position,grant)?;if !progress.fits(grant){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original insertion comparison exceeds caller grant").with_retained_progress(progress))}self.position=position;
   if let Some(order)=order{if progress.copied_items==0{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original insertion branch requires its structural item"))}match order{Ordering::Less=>self.next=node.left,Ordering::Greater=>{self.rank=self.rank.checked_add(index.count(node.left)+1).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original insertion rank overflow"))?;self.next=node.right},Ordering::Equal=>return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original insertion row already exists").with_retained_progress(progress))}self.position=0;}
   return Ok((false,progress))
  }
  let bytes=self.capacity_demand(index)?;if bytes==0{return Ok((true,empty))}let depth=self.depth_demand(index)?;if grant.maximum_depth<depth||grant.maximum_capacity_bytes<bytes{return Ok((false,empty))}
  match index.nodes.reserve_one(grant.maximum_capacity_bytes){Ok(progress)=>Ok((false,RetainedCloneProgress{copied_items:usize::from(progress.progressed),retained_capacity_bytes:progress.allocated_bytes,..empty})),Err(error)=>{let progress=RetainedCloneProgress{copied_items:usize::from(error.allocated_bytes!=0),retained_capacity_bytes:error.allocated_bytes,..empty};Err(index_page_refusal(error.kind,error.reason).with_retained_progress(progress))}}
 }
 fn capacity_demand<W>(&self,index:&HistoryFoldIndex<K,W>)->Result<usize,ValueError>{if index.free_head.is_some(){Ok(0)}else{index.nodes.next_allocation_bytes().map_err(|error|index_page_refusal(error.kind,error.reason))}}
 fn depth_demand<W>(&self,index:&HistoryFoldIndex<K,W>)->Result<usize,ValueError>{let page=if index.free_head.is_some(){0}else{index.nodes.next_reserve_depth_demand().map_err(|error|index_page_refusal(error.kind,error.reason))?};index.height(index.root).checked_add(2).and_then(|tree|page.checked_add(1).map(|page|tree.max(page))).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"original insertion depth overflow"))}
 /// 🌱️ Commits the proven vacant rank without another key comparison or borrowed pointer scan.
 pub fn commit_with<W>(&mut self,index:&mut HistoryFoldIndex<K,W>,grant:RetainedCloneGrant,create:impl FnOnce(V)->W)->Result<(Option<usize>,RetainedCloneProgress),ValueError>{
  let empty=RetainedCloneProgress::default();if self.source.is_none(){return Ok((self.slot,empty))}if grant.maximum_items==0||grant.maximum_depth==0{return Ok((None,empty))}if self.epoch!=Some(index.mutation_epoch)||!self.vacant{return Ok((None,empty))}if grant.maximum_depth<self.depth_demand(index)?||self.capacity_demand(index)?!=0{return Ok((None,empty))}
  let slot=index.free_head.unwrap_or(index.nodes.len());let(key,value)=self.source.take().unwrap();let root=index.insert_vacant_rank(index.root,self.rank,key,create(value));index.root=Some(root);index.mark_mutation();self.slot=Some(slot);Ok((Some(slot),RetainedCloneProgress{copied_items:1,..empty}))
 }
}
impl<K:Ord,V> HistoryFoldIndex<K,V>{
 fn insert_vacant_rank(&mut self,root:Option<usize>,rank:usize,key:K,value:V)->usize{
  let Some(root)=root else{let node=FoldIndexNode{entry:Some((key,value)),retired_entry:None,left:None,right:None,height:1,count:1};return if let Some(slot)=self.free_head{self.free_head=self.nodes[slot].left;self.nodes[slot]=node;slot}else{let slot=self.nodes.len();self.nodes.push(node);slot}};
  let left=self.count(self.nodes[root].left);if rank<=left{self.nodes[root].left=Some(self.insert_vacant_rank(self.nodes[root].left,rank,key,value))}else{self.nodes[root].right=Some(self.insert_vacant_rank(self.nodes[root].right,rank-left-1,key,value))}self.balance(root)
 }
}
impl<K:RetireOwned,V:RetireOwned> RetireOwned for HistoryFoldIndexInsert<K,V>{
 fn retirement(self)->Box<dyn RetirementCursor>{sequence(vec![deferred(self.source),deferred(self.epoch),deferred(self.next),deferred(self.rank),deferred(self.position),deferred(self.vacant),deferred(self.slot)])}
 fn retirement_birth_bytes(&self)->Option<usize>{sequence_birth_bytes(&[deferred_birth_bytes_for(&self.source),deferred_birth_bytes_for(&self.epoch),deferred_birth_bytes_for(&self.next),deferred_birth_bytes_for(&self.rank),deferred_birth_bytes_for(&self.position),deferred_birth_bytes_for(&self.vacant),deferred_birth_bytes_for(&self.slot)])}
 fn controlled_retirement_supported()->bool{K::controlled_retirement_supported()&&V::controlled_retirement_supported()}
}
