//! 🟰️ Complete links compare each original variable field in bounded fragments under the enclosing source fence.
use super::*;
use semio_framework_value::RetainedAllocationEffect;
use crate::retained::native_child;
use protocol::retained_mutation_demand_fits;
pub(super) struct RootLinkEquality{field:u8,text:RetainedSliceEquality,result:Option<bool>}
impl RootLinkEquality{
 pub fn new()->Self{Self{field:0,text:RetainedSliceEquality::new(),result:None}}
 pub fn result(&self)->Option<bool>{self.result}
 pub fn selection_work(&self)->u64{match self.field{0=>0,6|7=>2,_=>1}}
 pub fn owner_quote_work(&self)->u64{if self.field==0{0}else{4}}
 fn fixed_work(&self,l:&ArtifactLink,r:&ArtifactLink)->u64{if self.field==0{if matches!((&l.pin,&r.pin),(LinkPin::Snapshot{..},LinkPin::Snapshot{..})){3}else{2}}else{2*self.selection_work()+2}}
 fn text(link:&ArtifactLink,field:u8)->&str{match field{1=>&link.target.artifact_id,2=>&link.target.dialect.artifact_kind,3=>&link.target.dialect.standard,4=>&link.target.dialect.subset,5=>&link.role,6=>match &link.pin{LinkPin::Head=>"",LinkPin::Checkpoint{id}=>id,LinkPin::Snapshot{blob}=>&blob.hash},7=>match &link.pin{LinkPin::Snapshot{blob}=>&blob.media_type,_=>""},_=>""}}
 fn same_pin(left:&LinkPin,right:&LinkPin)->bool{match(left,right){(LinkPin::Head,LinkPin::Head)|(LinkPin::Checkpoint{..},LinkPin::Checkpoint{..})=>true,(LinkPin::Snapshot{blob:l},LinkPin::Snapshot{blob:r})=>l.size==r.size,_=>false}}
 pub fn demand(&self,l:&ArtifactLink,r:&ArtifactLink,b:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{if self.result.is_some(){return Err(invalid())}let minimum=if self.field==0{2*size_of::<LinkPin>()+3}else{3};if b<minimum||maximum_work_units==0||self.field!=0&&maximum_work_units<2*self.selection_work()+2{return Ok(Default::default())}if self.field==0{Ok(scalar(minimum))}else{nested(self.text.demand(Self::text(l,self.field).as_bytes(),Self::text(r,self.field).as_bytes(),b.saturating_sub(3),maximum_work_units.saturating_sub(self.fixed_work(l,r)))?,3)}}
 pub fn work_demand(&self,l:&ArtifactLink,r:&ArtifactLink,b:usize,maximum_work_units:u64)->Result<u64,ValueError>{self.demand(l,r,b,maximum_work_units)?;let minimum=if self.field==0{2*size_of::<LinkPin>()+3}else{3};if maximum_work_units==0||b<minimum{return Ok(0)}if self.field==0{return Ok(maximum_work_units.min(3))}let fixed=self.fixed_work(l,r);if maximum_work_units<fixed{return Ok(0)}let child=self.text.work_demand(Self::text(l,self.field).as_bytes(),Self::text(r,self.field).as_bytes(),b.saturating_sub(3),maximum_work_units-fixed)?;if child==0{return Ok(2*self.selection_work()+1)}child.checked_add(fixed).ok_or_else(invalid)}
 pub fn advance(&mut self,l:&ArtifactLink,r:&ArtifactLink,b:usize,maximum_work_units:u64,effect:&mut RetainedAllocationEffect<'_>,work:&mut u128)->Result<RetainedCloneStep,ValueError>{
  let g=effect.grant();if self.result.is_some(){return Err(invalid())}
  if maximum_work_units==0||g.maximum_items==0||g.maximum_depth==0{return Ok(RetainedCloneStep::Progress(Default::default()))}
  let minimum=if self.field==0{2*size_of::<LinkPin>()+3}else{3};if b<minimum||g.maximum_copy_bytes<minimum{return Ok(RetainedCloneStep::Progress(Default::default()))}
  let fixed=self.fixed_work(l,r);if maximum_work_units<fixed{return Ok(RetainedCloneStep::Progress(Default::default()))}
  let p=if self.field==0{let d=scalar(minimum);if Self::same_pin(&l.pin,&r.pin){self.field=1}else{self.result=Some(false)}receipt(d)}else{
   let left=Self::text(l,self.field).as_bytes();let right=Self::text(r,self.field).as_bytes();let body=b.saturating_sub(3);let budget=maximum_work_units-fixed;
   if self.text.work_demand(left,right,body,budget)?==0{return Ok(RetainedCloneStep::Progress(Default::default()))}
   let d=nested(self.text.demand(left,right,body,budget)?,3)?;if !retained_mutation_demand_fits(d,g){return Ok(RetainedCloneStep::Progress(Default::default()))}
   let text=&mut self.text;let step=native_child(effect,work,|e,rep|text.advance_into(left,right,body,budget,&mut e.nested(child(g,3)),rep))??;let p=add(step.progress(),3)?;
   if let Some(equal)=self.text.equal(){if !equal{self.result=Some(false)}else if self.field==7{self.result=Some(true)}else{self.field+=1;self.text=RetainedSliceEquality::new();}}p};
  Ok(if self.result.is_some(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})
 }
}
