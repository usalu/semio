//! 📏️ Original analytic same-parameter samples retain scratch, adaptive passes and sort work.
use super::*;
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress}};
const SAMPLE_LIMIT:usize=129;
const MIDPOINT_LIMIT:usize=64;
fn refusal()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original same-parameter frontier changed")}
fn allocation()->ValueError{ValueError::literal(ValueRefusalKind::AllocationFailed,"original same-parameter scratch allocation failed")}

impl BodyValidationJob {
 pub(super) fn same_parameter_demands(&self,body:&Body)->Result<RetirementDemand,ValueError>{
  if self.same_pair.is_some(){
   let Some((surface,_,pcurve,_,curve))=self.same_parameter_source(body)else{return Ok(RetirementDemand{depth:1,..Default::default()})};
   {
    if matches!(surface,Surface::Nurbs{..})||matches!(pcurve,semio_framework_3d_brep_representation::curve::Curve2::Nurbs{..})||matches!(curve,semio_framework_3d_brep_representation::curve::Curve3::Nurbs{..}){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original NURBS same-parameter evaluator has no funded scratch producer"))}
   }
   if self.same_samples.capacity()==0{return Ok(RetirementDemand{capacity_bytes:SAMPLE_LIMIT*size_of::<(f64,f64)>(),depth:1,..Default::default()})}
   if self.same_midpoints.capacity()==0{return Ok(RetirementDemand{capacity_bytes:MIDPOINT_LIMIT*size_of::<f64>(),depth:1,..Default::default()})}
   if self.same_samples.capacity()<SAMPLE_LIMIT||self.same_midpoints.capacity()<MIDPOINT_LIMIT{return Err(refusal())}
  }
  Ok(RetirementDemand{depth:1,..Default::default()})
 }
 fn same_parameter_source<'a>(&self,body:&'a Body)->Option<(&'a Surface,&'a semio_framework_3d_brep_representation::topology::Coedge,&'a semio_framework_3d_brep_representation::curve::Curve2,&'a semio_framework_3d_brep_representation::topology::Edge,&'a semio_framework_3d_brep_representation::curve::Curve3)>{
  let(face,coedge)=self.same_pair?;let surface=body.surfaces.get(body.faces.get(face)?.surface)?;let coedge=body.coedges.get(coedge)?;let pcurve=body.curves2.get(coedge.pcurve?)?;let edge=body.edges.get(coedge.edge)?;Some((surface,coedge,pcurve,edge,body.curves3.get(edge.curve)?))
 }
 pub(super) fn probe_same_parameter_funded(&mut self,body:&Body,_grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
  let mut receipt=RetainedCloneProgress{copied_items:1,..Default::default()};
  let Some((face_id,coedge_id))=self.same_pair else{
   match self.same_faces.step(body,self.live_coedges){FaceCoedgeTurn::Pending=>{},FaceCoedgeTurn::Complete=>self.phase=BodyValidationPhase::ShellClosure,FaceCoedgeTurn::Coedge(face,coedge)=>{self.same_pair=Some((face,coedge));self.same_stage=0;self.same_cursor=0;self.same_pass=0;self.same_sort_cursor=0;self.same_worst=None;self.same_samples.clear();self.same_midpoints.clear();}}return Ok(receipt)
  };
  let Some((surface,coedge,pcurve,edge,curve))=self.same_parameter_source(body)else{self.same_pair=None;return Ok(receipt)};
  if self.same_samples.capacity()==0{self.same_samples.try_reserve_exact(SAMPLE_LIMIT).map_err(|_|allocation())?;receipt.retained_capacity_bytes=self.same_samples.capacity()*size_of::<(f64,f64)>();return Ok(receipt)}
  if self.same_midpoints.capacity()==0{self.same_midpoints.try_reserve_exact(MIDPOINT_LIMIT).map_err(|_|allocation())?;receipt.retained_capacity_bytes=self.same_midpoints.capacity()*size_of::<f64>();return Ok(receipt)}
  match self.same_stage{
   0=>{if self.same_cursor>16{self.same_stage=1;self.same_cursor=0;}else{let s=self.same_cursor as f64/16.0;self.same_samples.push((s,same_parameter_deviation_at(surface,pcurve,curve,coedge.prange,edge.range,s)));self.same_cursor+=1;}},
   1=>{if self.same_cursor+1>=self.same_samples.len(){self.same_stage=if self.same_midpoints.is_empty(){3}else{2};self.same_cursor=0;self.same_worst=None;}else{let(s0,d0)=self.same_samples[self.same_cursor];let(s1,d1)=self.same_samples[self.same_cursor+1];if(d1-d0).abs()>d0.max(d1).max(1e-12)*0.5{if self.same_midpoints.len()==MIDPOINT_LIMIT{return Err(refusal())}self.same_midpoints.push(0.5*(s0+s1));}self.same_cursor+=1;}},
   2=>{if let Some(&s)=self.same_midpoints.get(self.same_cursor){if self.same_samples.len()==SAMPLE_LIMIT{return Err(refusal())}self.same_samples.push((s,same_parameter_deviation_at(surface,pcurve,curve,coedge.prange,edge.range,s)));self.same_cursor+=1;}else{self.same_stage=4;self.same_cursor=1;self.same_sort_cursor=1;}},
   3=>{if let Some(&(s,deviation))=self.same_samples.get(self.same_cursor){if self.same_worst.is_none_or(|(_,worst)|deviation.partial_cmp(&worst).unwrap_or(std::cmp::Ordering::Equal)!=std::cmp::Ordering::Less){self.same_worst=Some((s,deviation));}self.same_cursor+=1;}else{self.same_stage=5;}},
   4=>{if self.same_cursor>=self.same_samples.len(){self.same_midpoints.clear();self.same_pass+=1;self.same_stage=if self.same_pass==3{3}else{1};self.same_cursor=0;self.same_worst=None;}else if self.same_sort_cursor!=0&&self.same_samples[self.same_sort_cursor-1].0>self.same_samples[self.same_sort_cursor].0{self.same_samples.swap(self.same_sort_cursor-1,self.same_sort_cursor);self.same_sort_cursor-=1;}else{self.same_cursor+=1;self.same_sort_cursor=self.same_cursor;}},
   5=>{if let Some((parameter,deviation))=self.same_worst{if deviation>edge.tol.value(){self.hold_topology_finding(retained_loop::LoopFinding::SameParameter{coedge:coedge_id,face:face_id,edge:coedge.edge,deviation,parameter,tolerance:edge.tol.value(),prange:coedge.prange,range:edge.range});}}self.same_pair=None;},
   _=>return Err(refusal()),
  }
  Ok(receipt)
 }
}
