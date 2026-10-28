//! 📣️ Original cursor catalogs and indexes admit each backing page and authored identity separately.
use crate::lifted_store::ArtifactStoreInitializationRuntime;
use semio_framework_os_spr::{Edit,Mutation,MutationId};
use semio_framework_os_vcs::ArtifactHistoryKey;
use semio_framework_os_store_ledger::{CursorRevisionAccumulator,CursorRevisionRecord};
use semio_framework_value::{ValueError,ValueRefusalKind,RetirementDemand,RetainedCloneGrant,RetainedCloneProgress};
use std::mem::size_of;
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"cursor commit requires its original edit, ledger key and digest")}
fn fixed(copy_bytes:usize,capacity_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,capacity_bytes,depth:1,release_bytes:0}}
fn identity<P,M:Mutation<P>,R>(edit:&Edit<M>,operation:usize,use_text:impl FnOnce(&str)->Result<R,ValueError>)->Result<R,ValueError>{
 if let Some(id)=edit.mutation_meta.get(operation).and_then(|meta|meta.mutation_id.as_ref()).or_else(||edit.forwards.get(operation).and_then(M::mutation_id)){return use_text(&id.0)}
 let mut bytes=[0u8;256];let mut digits=[0u8;20];let(mut at,mut value)=(digits.len(),operation);loop{at-=1;digits[at]=b'0'+(value%10)as u8;value/=10;if value==0{break}}let length=edit.id.len()+1+digits.len()-at;if length>bytes.len(){return Err(invalid())}bytes[..edit.id.len()].copy_from_slice(edit.id.as_bytes());bytes[edit.id.len()]=b'#';bytes[edit.id.len()+1..length].copy_from_slice(&digits[at..]);use_text(std::str::from_utf8(&bytes[..length]).unwrap())
}
impl<P> ArtifactStoreInitializationRuntime<P>{
 pub fn commit_metadata_demands<M:Mutation<P>>(&self,edit:&Edit<M>,key:ArtifactHistoryKey,digest:[u8;32],redo:bool)->Result<RetirementDemand,ValueError>{
  if self.commit_source.is_some_and(|bound|bound!=(edit as*const _ as usize,key,digest,redo)){return Err(invalid())}let control=size_of::<u8>()+size_of::<usize>()+size_of::<Option<(usize,ArtifactHistoryKey,[u8;32],bool)>>();
  Ok(match self.commit_stage{
   0|1=>{let ids=match(self.commit_stage,redo){(0,false)=>&self.applied_edit_ids,(0,true)=>&self.redo_edit_ids,(1,false)=>&self.cursor_applied_edit_ids,_=>&self.cursor_redo_edit_ids};fixed(control+size_of::<String>()+edit.id.len(),ids.next_push_allocation_bytes()+edit.id.len())},
   2=>{let rows=if redo{&self.revision.redo}else{&self.revision.applied};fixed(control+size_of::<CursorRevisionRecord>(),rows.next_push_allocation_bytes())},
   3 if self.commit_operation<edit.forwards.len()=>identity::<P,M,_>(edit,self.commit_operation,|id|{let index=&self.revision.mutation_positions;let capacity=index.next_insert_capacity_byte_demand(id,0)?;Ok(RetirementDemand{copy_bytes:control+if capacity==0{size_of::<(MutationId,(usize,usize,[u8;32]))>()+2*id.len()}else{0},capacity_bytes:if capacity==0{id.len()}else{capacity},depth:index.next_insert_depth_demand(id)?.max(1),release_bytes:0})})?,
   4 if self.commit_operation<edit.forwards.len()&&edit.forwards[self.commit_operation].may_emit_foreign_steps()=>{let id=CursorRevisionAccumulator::hash_record(b"edit-id",&[edit.id.as_bytes()]);let index=&self.revision.unit_flags;if index.contains_key(&(id,self.commit_operation)){return Ok(fixed(control,0))}RetirementDemand{depth:index.next_insert_depth_demand(&(id,self.commit_operation))?,..fixed(control+size_of::<(([u8;32],usize),bool)>(),index.next_insert_capacity_byte_demand(&(id,self.commit_operation),0)?)}},
   5=>{let id=CursorRevisionAccumulator::hash_record(b"edit-id",&[edit.id.as_bytes()]);let index=&self.revision.indexed_edits;RetirementDemand{depth:index.next_insert_depth_demand(&id)?,..fixed(control+size_of::<([u8;32],(usize,usize))>(),index.next_insert_capacity_byte_demand(&id,0)?)}},
   _=>fixed(control,0),
  })
 }
 pub fn advance_commit_metadata<M:Mutation<P>>(&mut self,edit:&Edit<M>,key:ArtifactHistoryKey,digest:[u8;32],redo:bool,grant:RetainedCloneGrant)->Result<(bool,RetainedCloneProgress),ValueError>{
  let d=self.commit_metadata_demands(edit,key,digest,redo)?;if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Ok((false,Default::default()))}self.commit_source=Some((edit as*const _ as usize,key,digest,redo));let mut progress=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,..Default::default()};let id=CursorRevisionAccumulator::hash_record(b"edit-id",&[edit.id.as_bytes()]);
  match self.commit_stage{
   0|1=>{let ids=match(self.commit_stage,redo){(0,false)=>&mut self.applied_edit_ids,(0,true)=>&mut self.redo_edit_ids,(1,false)=>&mut self.cursor_applied_edit_ids,_=>&mut self.cursor_redo_edit_ids};ids.push(edit.id.clone());self.commit_stage+=1;},
   2=>{let initial=self.revision.identity_digest;let rows=if redo{&mut self.revision.redo}else{&mut self.revision.applied};let previous=rows.last().map_or(initial,|row|row.prefix_digest);let domain=if redo{b"redo".as_slice()}else{b"applied".as_slice()};let prefix=CursorRevisionAccumulator::hash_record(domain,&[&previous,&digest]);rows.push(CursorRevisionRecord::new(Some(key),id,digest,prefix));self.commit_stage=if redo{6}else{3};},
   3 if self.commit_operation<edit.forwards.len()=>{let position=self.applied_edit_ids.len()-1;let index=&mut self.revision.mutation_positions;let operation=self.commit_operation;let reserved=identity::<P,M,_>(edit,operation,|text|{let capacity=index.next_insert_capacity_byte_demand(text,grant.maximum_copy_bytes)?;if capacity!=0{return index.reserve_insert_step(text,grant).map(Some).map_err(|(error,p)|error.with_retained_progress(p))}let original=MutationId(text.into());index.insert_reserved(original,(position,operation,id),grant).unwrap_or_else(|_|unreachable!("original identity index backing and depth admitted"));Ok(None)})?;if let Some(p)=reserved{progress=p;progress.copied_bytes=d.copy_bytes;}else{self.commit_stage=4;}},
   3=>self.commit_stage=5,
   4 if self.commit_operation<edit.forwards.len()&&edit.forwards[self.commit_operation].may_emit_foreign_steps()=>{let index=&mut self.revision.unit_flags;let key=(id,self.commit_operation);if index.contains_key(&key){self.commit_operation+=1;self.commit_stage=3;}else if index.next_insert_capacity_byte_demand(&key,grant.maximum_copy_bytes)?!=0{progress=index.reserve_insert_step(&key,grant).map_err(|(error,p)|error.with_retained_progress(p))?;}else{index.insert_reserved(key,false,grant).unwrap_or_else(|_|unreachable!("original unit flag backing admitted"));self.commit_operation+=1;self.commit_stage=3;}},
   4=>{self.commit_operation+=1;self.commit_stage=3;},
   5=>{let index=&mut self.revision.indexed_edits;if index.next_insert_capacity_byte_demand(&id,grant.maximum_copy_bytes)?!=0{progress=index.reserve_insert_step(&id,grant).map_err(|(error,p)|error.with_retained_progress(p))?;}else{index.insert_reserved(id,(self.applied_edit_ids.len()-1,edit.forwards.len()),grant).unwrap_or_else(|_|unreachable!("original edit index backing admitted"));self.commit_stage=6;}},
   _=>{self.commit_stage=0;self.commit_operation=0;self.commit_source=None;return Ok((true,progress))},
  }Ok((false,progress))
 }
}

