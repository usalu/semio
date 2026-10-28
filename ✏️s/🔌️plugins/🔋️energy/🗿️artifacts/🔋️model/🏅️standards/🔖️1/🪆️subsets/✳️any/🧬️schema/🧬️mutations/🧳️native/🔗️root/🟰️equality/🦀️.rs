//! 🟰️ Complete links compare each original variable field in bounded fragments under the enclosing source fence.
use super::*;
pub(super) struct RootLinkEquality{field:u8,text:RetainedSliceEquality,result:Option<bool>}
impl RootLinkEquality{
 pub fn new()->Self{Self{field:0,text:RetainedSliceEquality::new(),result:None}}
 pub fn result(&self)->Option<bool>{self.result}
 fn text(link:&ArtifactLink,field:u8)->&str{match field{1=>&link.target.artifact_id,2=>&link.target.dialect.artifact_kind,3=>&link.target.dialect.standard,4=>&link.target.dialect.subset,5=>&link.role,6=>match &link.pin{LinkPin::Head=>"",LinkPin::Checkpoint{id}=>id,LinkPin::Snapshot{blob}=>&blob.hash},7=>match &link.pin{LinkPin::Snapshot{blob}=>&blob.media_type,_=>""},_=>""}}
 fn same_pin(left:&LinkPin,right:&LinkPin)->bool{match(left,right){(LinkPin::Head,LinkPin::Head)|(LinkPin::Checkpoint{..},LinkPin::Checkpoint{..})=>true,(LinkPin::Snapshot{blob:l},LinkPin::Snapshot{blob:r})=>l.size==r.size,_=>false}}
 pub fn demand(&self,l:&ArtifactLink,r:&ArtifactLink,b:usize)->Result<RetirementDemand,ValueError>{if self.result.is_some(){return Err(invalid())}if self.field==0{Ok(scalar(2*size_of::<LinkPin>()+3))}else{nested(self.text.demand(Self::text(l,self.field).as_bytes(),Self::text(r,self.field).as_bytes(),b.saturating_sub(3))?,3)}}
 pub fn advance(&mut self,l:&ArtifactLink,r:&ArtifactLink,b:usize,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let d=self.demand(l,r,b)?;if !retained_mutation_demand_fits(d,g){return Ok(RetainedCloneStep::Progress(Default::default()))}let p=if self.field==0{if Self::same_pin(&l.pin,&r.pin){self.field=1}else{self.result=Some(false)}receipt(d)}else{let step=self.text.advance(Self::text(l,self.field).as_bytes(),Self::text(r,self.field).as_bytes(),child(g,3)).map_err(|e|tag(e,3))?;let p=add(step.progress(),3)?;if let Some(equal)=self.text.equal(){if !equal{self.result=Some(false)}else if self.field==7{self.result=Some(true)}else{self.field+=1;self.text=RetainedSliceEquality::new();}}p};Ok(if self.result.is_some(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})}
}
