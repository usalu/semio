//! 🪢️ The original validation loop frontier pays source rows, diagnostic text and finding pages separately.
use super::*;
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress}};
use std::fmt::{self,Write};

#[derive(Clone,Copy)]
pub(super) enum LoopFinding {
 Ring{loop_id:LoopId,index:usize,next:usize,mismatch:bool},
 Valence{edge:EdgeId,uses:usize},
 MissingPcurve{coedge:CoedgeId},
 SameParameter{coedge:CoedgeId,face:FaceId,edge:EdgeId,deviation:f64,parameter:f64,tolerance:f64,prange:(f64,f64),range:(f64,f64)},
 Tolerance{finer_kind:&'static str,finer:u32,coarser_kind:&'static str,coarser:u32},
}
semio_framework_value::artifact_retire_leaf!(LoopFinding);
impl LoopFinding {
 fn code(self)->&'static str{match self{Self::Ring{mismatch:true,..}=>"next-prev-mismatch",Self::Ring{..}=>"loop-not-closed",Self::Valence{..}=>"non-manifold-edge",Self::MissingPcurve{..}=>"missing-pcurve",Self::Tolerance{..}=>"tolerance-containment-violated",Self::SameParameter{..}=>"same-parameter-violated"}}
 fn text(self,entity:bool,target:&mut impl Write)->fmt::Result{match self{
  Self::Ring{loop_id,index,next,mismatch}=>if entity{write!(target,"loop-{}",loop_id.raw_index())}else if mismatch{write!(target,"coedge {index}'s next/prev pointers are not symmetric with its ring neighbor")}else{write!(target,"coedge {index} ends at a different vertex than coedge {next} starts at")},
  Self::Valence{edge,uses}=>if entity{write!(target,"edge-{}",edge.raw_index())}else{write!(target,"edge is used by {uses} coedges (2-manifold shapes use at most 2)")},
  Self::MissingPcurve{coedge}=>if entity{write!(target,"coedge-{}",coedge.raw_index())}else{target.write_str("coedge has no p-curve — every coedge must carry one for trim/same-parameter validation")},
  Self::SameParameter{coedge,face,edge,deviation,parameter,tolerance,prange,range}=>if entity{write!(target,"coedge-{}",coedge.raw_index())}else{write!(target,"pcurve and 3D curve disagree by {deviation} at s={parameter} (tol {tolerance}; face-{} edge-{} prange {prange:?} range {range:?})",face.raw_index(),edge.raw_index())},
  Self::Tolerance{finer_kind,finer,coarser_kind,coarser}=>if entity{write!(target,"{finer_kind}-{finer}")}else{write!(target,"{finer_kind}-{finer}'s tolerance exceeds its containing {coarser_kind}-{coarser}'s")},
 }}
 fn length(self,entity:bool)->Result<usize,ValueError>{let mut count=TextLength(0);self.text(entity,&mut count).map_err(|_|refusal())?;Ok(count.0)}
 fn scalar_bytes(self,entity:bool,offset:usize)->Result<usize,ValueError>{let mut cursor=TextScalar{offset,visited:0,bytes:0};self.text(entity,&mut cursor).map_err(|_|refusal())?;Ok(cursor.bytes)}
}
struct TextScalar{offset:usize,visited:usize,bytes:usize}
impl Write for TextScalar{fn write_str(&mut self,text:&str)->fmt::Result{if self.bytes==0&&self.offset>=self.visited&&self.offset<self.visited+text.len(){self.bytes=text.get(self.offset-self.visited..).ok_or(fmt::Error)?.chars().next().ok_or(fmt::Error)?.len_utf8();}self.visited=self.visited.checked_add(text.len()).ok_or(fmt::Error)?;Ok(())}}
struct TextLength(usize);
impl Write for TextLength{fn write_str(&mut self,text:&str)->fmt::Result{self.0=self.0.checked_add(text.len()).ok_or(fmt::Error)?;Ok(())}}
struct TextSlice<'a>{target:&'a mut String,start:usize,end:usize,visited:usize}
impl Write for TextSlice<'_>{fn write_str(&mut self,text:&str)->fmt::Result{let end=self.visited.checked_add(text.len()).ok_or(fmt::Error)?;let from=self.start.saturating_sub(self.visited).min(text.len());let to=self.end.saturating_sub(self.visited).min(text.len());if to>from{let mut boundary=to;while boundary>from&&!text.is_char_boundary(boundary){boundary-=1;}if boundary>from{self.target.push_str(text.get(from..boundary).ok_or(fmt::Error)?);}}self.visited=end;Ok(())}}
fn refusal()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original validation loop source or frontier changed")}
fn allocation()->ValueError{ValueError::literal(ValueRefusalKind::AllocationFailed,"original validation loop backing allocation failed")}
fn metadata(depth:usize)->RetirementDemand{RetirementDemand{depth,..Default::default()}}

impl BodyValidationJob {
 /// 🧮️ Quotes the next original validation event without allocating or mutating the source.
 pub fn funded_demands(&self,body:&Body)->Result<RetirementDemand,ValueError>{
  if self.source_body!=body as*const Body as usize{return Err(refusal())}
  match self.phase{BodyValidationPhase::CollectingTopology=>self.planning_demands(body),BodyValidationPhase::LoopRings|BodyValidationPhase::EdgeValence|BodyValidationPhase::ToleranceContainment|BodyValidationPhase::MissingPcurves=>self.loop_demands(body),BodyValidationPhase::SameParameter=>if self.loop_findings.iter().any(Option::is_some){self.loop_demands(body)}else{self.same_parameter_demands(body)},BodyValidationPhase::Complete=>Ok(Default::default()),_=>Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original validation phase has no funded producer yet"))}
 }
 fn loop_demands(&self,_body:&Body)->Result<RetirementDemand,ValueError>{
  if let Some(finding)=self.loop_findings.get(self.loop_finding_cursor).copied().flatten(){
   let Some(output)=&self.loop_output else{return Ok(metadata(1))};
   if self.loop_text_stage<2{let entity=self.loop_text_stage==0;let text=if entity{&output.entity}else{&output.message};let length=finding.length(entity)?;return Ok(if text.len()==length{metadata(1)}else if text.capacity()==0{RetirementDemand{capacity_bytes:length,depth:1,..Default::default()}}else{RetirementDemand{copy_bytes:finding.scalar_bytes(entity,text.len())?,depth:1,..Default::default()}})}
   let capacity=self.issues.next_allocation_bytes().map_err(|_|refusal())?;
   return Ok(if capacity!=0{RetirementDemand{copy_bytes:self.issues.next_reserve_copy_byte_demand().map_err(|_|refusal())?,capacity_bytes:capacity,depth:self.issues.next_reserve_depth_demand().map_err(|_|refusal())?,..Default::default()}}else{metadata(self.issues.next_push_depth_demand().map_err(|_|refusal())?.max(1))})
  }
  if self.phase==BodyValidationPhase::LoopRings&&self.loop_id.is_some()&&self.ring_next.is_some(){if self.ring.capacity()==0{let extent=self.live_coedges.checked_add(1).ok_or_else(refusal)?;Ok(RetirementDemand{capacity_bytes:extent.checked_mul(std::mem::size_of::<CoedgeId>()).ok_or_else(refusal)?,depth:1,..Default::default()})}else if self.ring.len()<self.ring.capacity(){Ok(RetirementDemand{copy_bytes:std::mem::size_of::<CoedgeId>(),depth:1,..Default::default()})}else{Err(refusal())}}else{Ok(metadata(1))}
 }
 /// ⏱️ Advances one original planning or loop event under its full physical authority.
 pub fn step_funded(&mut self,body:&Body,units:usize,grant:RetainedCloneGrant)->Result<(RetainedCloneProgress,usize),ValueError>{
  let demand=self.funded_demands(body)?;
  if self.is_complete()||units==0||grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok((Default::default(),0))}
  if self.phase==BodyValidationPhase::CollectingTopology{return self.step_planning_funded(body,units,grant)}
  let mut receipt=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};
  if let Some(finding)=self.loop_findings.get(self.loop_finding_cursor).copied().flatten(){
   if self.loop_output.is_none(){self.loop_output=Some(ValidationIssue{entity:String::new(),code:finding.code(),message:String::new()});}
   else if self.loop_text_stage<2{
    let entity=self.loop_text_stage==0;let output=self.loop_output.as_mut().unwrap();let text=if entity{&mut output.entity}else{&mut output.message};let length=finding.length(entity)?;
    if text.len()==length{self.loop_text_stage+=1;}
    else if text.capacity()==0{text.try_reserve_exact(length).map_err(|_|allocation())?;receipt.retained_capacity_bytes=text.capacity();}
    else{let from=text.len();let end=from.saturating_add(grant.maximum_copy_bytes).min(length);finding.text(entity,&mut TextSlice{target:text,start:from,end,visited:0}).map_err(|_|refusal())?;receipt.copied_bytes=text.len()-from;}
   }else if demand.capacity_bytes!=0{receipt=self.issues.reserve_one_funded(grant).map_err(|error|allocation().with_retained_progress(RetainedCloneProgress{retained_capacity_bytes:error.allocated_bytes,..Default::default()}))?;}
   else{let output=self.loop_output.take().unwrap();if let Err(output)=self.issues.push_reserved(output){self.loop_output=Some(output);return Err(refusal())}self.loop_findings[self.loop_finding_cursor]=None;self.loop_finding_cursor+=1;self.loop_text_stage=0;}
  }else if self.loop_finding_cursor!=0{self.loop_finding_cursor=0;self.loop_findings=[None,None];if self.phase==BodyValidationPhase::LoopRings{self.ring_probe+=1;}}
  else if self.phase==BodyValidationPhase::SameParameter{receipt=self.probe_same_parameter_funded(body,grant)?;}
  else if self.phase==BodyValidationPhase::EdgeValence{self.probe_valence_funded(body);}
  else if self.phase==BodyValidationPhase::ToleranceContainment{self.probe_tolerance_funded(body);}
  else if self.phase==BodyValidationPhase::MissingPcurves{self.probe_pcurve_presence_funded(body);}
  else if self.phase==BodyValidationPhase::LoopRings&&self.loop_id.is_some()&&self.ring_next.is_some()&&self.ring.capacity()==0{let extent=self.live_coedges.checked_add(1).ok_or_else(refusal)?;self.ring.try_reserve_exact(extent).map_err(|_|allocation())?;receipt.retained_capacity_bytes=self.ring.capacity()*std::mem::size_of::<CoedgeId>();}
  else if let Some(loop_id)=self.loop_id{
   if let Some(current)=self.ring_next.take(){self.ring.push(current);if let Some(coedge)=body.coedges.get(current){if self.ring.len()<=self.live_coedges&&self.ring.first().copied()!=Some(coedge.next){self.ring_next=Some(coedge.next);}}}
   else if self.ring_probe<self.ring.len(){
    let index=self.ring_probe;let next=(index+1)%self.ring.len();let pair=body.coedge_endpoints(self.ring[index]).zip(body.coedge_endpoints(self.ring[next]));
    if let Some(((_,end),(start,_)))=pair{let a=body.coedges.get(self.ring[index]).ok_or_else(refusal)?;let b=body.coedges.get(self.ring[next]).ok_or_else(refusal)?;let mut found=0;if end!=start{self.loop_findings[found]=Some(LoopFinding::Ring{loop_id,index,next,mismatch:false});found+=1;}if a.next!=self.ring[next]||b.prev!=self.ring[index]{self.loop_findings[found]=Some(LoopFinding::Ring{loop_id,index,next,mismatch:true});found+=1;}if found==0{self.ring_probe+=1;}}
    else{self.ring_probe+=1;}
   }else{self.ring.clear();self.loop_id=None;}
  }else if self.loop_slot>=body.loops.slot_count(){self.phase=BodyValidationPhase::EdgeValence;}
  else{if let Some((id,lp))=body.loops.slot_at(self.loop_slot){self.loop_id=Some(id);self.ring_next=Some(lp.first);self.ring_probe=0;}self.loop_slot+=1;}
  self.units_done+=1;self.units_total=self.units_total.max(self.units_done.saturating_add(1));
  if !receipt.fits(grant){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original validation loop allocator exceeded admitted backing").with_retained_progress(receipt))}Ok((receipt,1))
 }
 pub(super) fn hold_topology_finding(&mut self,finding:LoopFinding){self.loop_findings=[Some(finding),None];self.loop_finding_cursor=0;self.loop_text_stage=0;}
 fn probe_valence_funded(&mut self,body:&Body){
  let Some(edge)=self.valence_edge else{if self.valence_edge_slot>=body.edges.slot_count(){self.phase=BodyValidationPhase::ToleranceContainment;return}self.valence_edge=body.edges.slot_at(self.valence_edge_slot).map(|(id,_)|id);self.valence_edge_slot+=1;self.valence_coedge_slot=0;self.valence_uses=0;return};
  if self.valence_coedge_slot<body.coedges.slot_count(){if let Some((_,coedge))=body.coedges.slot_at(self.valence_coedge_slot){self.valence_uses+=usize::from(semio_framework_3d_brep_representation::topology::coedge_uses_edge(coedge,edge));}self.valence_coedge_slot+=1;}
  else{if self.valence_uses>2{self.hold_topology_finding(LoopFinding::Valence{edge,uses:self.valence_uses});}self.valence_edge=None;}
 }
 fn probe_pcurve_presence_funded(&mut self,body:&Body){
  if self.pcurve_slot>=body.coedges.slot_count(){self.phase=BodyValidationPhase::SameParameter;return}
  if let Some((coedge,value))=body.coedges.slot_at(self.pcurve_slot){if value.pcurve.is_none(){self.hold_topology_finding(LoopFinding::MissingPcurve{coedge});}}
  self.pcurve_slot+=1;
 }
 fn probe_tolerance_funded(&mut self,body:&Body){
  if self.tolerance_stage==0{
   let Some(&edge_id)=self.edges.get(self.tolerance_edge_cursor)else{self.tolerance_stage=1;return};
   if self.tolerance_vertex>=2{self.tolerance_edge_cursor+=1;self.tolerance_vertex=0;return}
   let endpoint=self.tolerance_vertex;self.tolerance_vertex+=1;let Some(edge)=body.edges.get(edge_id)else{return};let vertex_id=[edge.v0,edge.v1][endpoint];
   if body.vertices.get(vertex_id).is_some_and(|vertex|vertex.tol.value()>edge.tol.value()){self.hold_topology_finding(LoopFinding::Tolerance{finer_kind:"vertex",finer:vertex_id.raw_index(),coarser_kind:"edge",coarser:edge_id.raw_index()});}return;
  }
  match self.tolerance_faces.step(body,self.live_coedges){FaceCoedgeTurn::Pending=>{},FaceCoedgeTurn::Complete=>self.phase=BodyValidationPhase::MissingPcurves,FaceCoedgeTurn::Coedge(face_id,coedge_id)=>{
   let Some(face)=body.faces.get(face_id)else{return};let Some(coedge)=body.coedges.get(coedge_id)else{return};
   if body.edges.get(coedge.edge).is_some_and(|edge|edge.tol.value()>face.tol.value()){self.hold_topology_finding(LoopFinding::Tolerance{finer_kind:"edge",finer:coedge.edge.raw_index(),coarser_kind:"face",coarser:face_id.raw_index()});}
  }}
 }

}
