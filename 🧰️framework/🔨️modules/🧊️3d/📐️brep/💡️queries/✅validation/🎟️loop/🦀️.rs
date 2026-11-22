//! 🪢️ The original validation loop frontier pays source rows, diagnostic text and finding pages separately.
use super::*;
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress}};
use std::fmt::{self,Write};

#[derive(Clone,Copy)]
pub(super) enum LoopFinding {
 Ring{loop_id:LoopId,index:usize,next:usize,mismatch:bool},
 Valence{edge:EdgeId,uses:usize},
 Shell{shell:u32,edge:EdgeId,uses:usize},
 Intersection{a:FaceId,b:FaceId},
 Winding{face:FaceId,inner:Option<LoopId>,area:f64},
 Inward{solid:u32,volume:f64},
 VoidOrientation{solid:u32,shell:u32},
 ShortEdge{edge:EdgeId,length:f64,tolerance:f64},
 Sliver{face:FaceId,area:f64,tolerance2:f64},
 MissingPcurve{coedge:CoedgeId},
 SameParameter{coedge:CoedgeId,face:FaceId,edge:EdgeId,deviation:f64,parameter:f64,tolerance:f64,prange:(f64,f64),range:(f64,f64)},
 Tolerance{finer_kind:&'static str,finer:u32,coarser_kind:&'static str,coarser:u32},
}
semio_framework_value::artifact_retire_leaf!(LoopFinding);
impl LoopFinding {
 pub(super) fn code(self)->&'static str{match self{Self::Ring{mismatch:true,..}=>"next-prev-mismatch",Self::Ring{..}=>"loop-not-closed",Self::Valence{..}=>"non-manifold-edge",Self::Inward{..}=>"shell-orientation-inward",Self::Intersection{..}=>"face-self-intersection",Self::VoidOrientation{..}=>"void-shell-not-inverted",Self::ShortEdge{..}=>"degenerate-edge",Self::Sliver{..}=>"sliver-face",Self::Winding{inner:None,..}=>"outer-loop-winding-inverted",Self::Winding{..}=>"hole-loop-winding-inverted",Self::Shell{uses:2,..}=>"orientation-inconsistent",Self::Shell{..}=>"shell-not-closed",Self::MissingPcurve{..}=>"missing-pcurve",Self::Tolerance{..}=>"tolerance-containment-violated",Self::SameParameter{..}=>"same-parameter-violated"}}
 fn text(self,entity:bool,target:&mut impl Write)->fmt::Result{match self{
  Self::Ring{loop_id,index,next,mismatch}=>if entity{write!(target,"loop-{}",loop_id.raw_index())}else if mismatch{write!(target,"coedge {index}'s next/prev pointers are not symmetric with its ring neighbor")}else{write!(target,"coedge {index} ends at a different vertex than coedge {next} starts at")},
  Self::Inward{solid,volume}=>if entity{write!(target,"solid-{solid}")}else{write!(target,"outer shell's signed volume is negative ({volume}); face normals appear to point inward")},
  Self::Intersection{a,b}=>if entity{write!(target,"face-{}-face-{}",a.raw_index(),b.raw_index())}else{target.write_str("planar triangular faces intersect through their interiors")},
  Self::VoidOrientation{solid,shell}=>if entity{write!(target,"solid-{solid}-void-shell-{shell}")}else{target.write_str("void (inner) shell's signed volume has the same sign as the outer shell — it should be inverted relative to the solid's exterior")},
  Self::ShortEdge{edge,length,tolerance}=>if entity{write!(target,"edge-{}",edge.raw_index())}else{write!(target,"edge length {length} is below its own tolerance {tolerance}")},
  Self::Sliver{face,area,tolerance2}=>if entity{write!(target,"face-{}",face.raw_index())}else{write!(target,"face area {area} is below tol² ({tolerance2})")},
  Self::Winding{face,inner,area}=>match (entity,inner) {
    (true,None)=>write!(target,"face-{}",face.raw_index()),
    (true,Some(id))=>write!(target,"face-{}-loop-{}",face.raw_index(),id.raw_index()),
    (false,None)=>write!(target,"outer loop's (u, v) signed area is negative ({area}) — the loop is clockwise in its own surface, so the face trims to the COMPLEMENT of its region"),
    (false,Some(_))=>write!(target,"hole loop's (u, v) signed area is positive ({area}) — a hole must run counter to its outer loop"),
  },
  Self::Shell{shell,edge,uses}=>if entity{write!(target,"shell-{shell}-edge-{}",edge.raw_index())}else if uses==2{target.write_str("both faces sharing this edge traverse it in the same direction — adjacent face orientations disagree")}else{write!(target,"edge is used {uses} time(s) within this shell (a closed shell needs exactly 2)")},
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
 pub fn funded_demands(&self,body:&Body)->Result<RetirementDemand,ValueError>{self.funded_demands_for(body,None)}

 /// 🔎️ Quotes the next validation event within the selected original shape.
 pub fn funded_demands_for(&self,body:&Body,scope:Option<&ReachSet>)->Result<RetirementDemand,ValueError>{
  if self.source_body!=body as*const Body as usize{return Err(refusal())}
  match self.phase{BodyValidationPhase::CollectingTopology=>self.planning_demands_for(body,scope),BodyValidationPhase::LoopRings|BodyValidationPhase::EdgeValence|BodyValidationPhase::ToleranceContainment|BodyValidationPhase::MissingPcurves|BodyValidationPhase::FaceLoopWinding|BodyValidationPhase::SolidOrientation|BodyValidationPhase::DegenerateEdges|BodyValidationPhase::DegenerateFaces=>self.loop_demands(body,scope),BodyValidationPhase::SameParameter=>if self.loop_findings.iter().any(Option::is_some){self.loop_demands(body,scope)}else{self.same_parameter_demands(body)},BodyValidationPhase::ShellClosure=>if self.loop_findings.iter().any(Option::is_some){self.loop_demands(body,scope)}else{self.shell_closure_demand(body,scope)},BodyValidationPhase::SelfIntersection=>if self.loop_findings.iter().any(Option::is_some){self.loop_demands(body,scope)}else{self.triangle_pair_demand(body)},BodyValidationPhase::Complete=>Ok(Default::default())}
 }
 fn loop_demands(&self,body:&Body,scope:Option<&ReachSet>)->Result<RetirementDemand,ValueError>{
  if let Some(finding)=self.loop_findings.get(self.loop_finding_cursor).copied().flatten(){
   let Some(output)=&self.loop_output else{return Ok(metadata(1))};
   if self.loop_text_stage<2{let entity=self.loop_text_stage==0;let text=if entity{&output.entity}else{&output.message};let length=finding.length(entity)?;return Ok(if text.len()==length{metadata(1)}else if text.capacity()==0{RetirementDemand{capacity_bytes:length,depth:1,..Default::default()}}else{RetirementDemand{copy_bytes:finding.scalar_bytes(entity,text.len())?,depth:1,..Default::default()}})}
   return self.issues.append_demand();
  }
  if self.phase==BodyValidationPhase::FaceLoopWinding && scoped_count!(body,scope,faces)!=0 {
   let count=scoped_count!(body,scope,faces);
   if self.winding.face_areas.capacity()==0 { return Ok(RetirementDemand{capacity_bytes:count.checked_mul(size_of::<f64>()).ok_or_else(refusal)?,depth:1,..Default::default()}); }
   if self.winding.face_areas.len()<count || self.winding.pair.is_some() && self.winding.sample+1==self.winding.count && self.winding.faces.next.is_none() { return Ok(RetirementDemand{copy_bytes:size_of::<f64>(),depth:1,..Default::default()}); }
  }
  if self.phase==BodyValidationPhase::LoopRings&&self.loop_id.is_some()&&self.ring_next.is_some(){if self.ring.capacity()==0{let extent=self.live_coedges.checked_add(1).ok_or_else(refusal)?;Ok(RetirementDemand{capacity_bytes:extent.checked_mul(std::mem::size_of::<CoedgeId>()).ok_or_else(refusal)?,depth:1,..Default::default()})}else if self.ring.len()<self.ring.capacity(){Ok(RetirementDemand{copy_bytes:std::mem::size_of::<CoedgeId>(),depth:1,..Default::default()})}else{Err(refusal())}}else{Ok(metadata(1))}
 }
 /// ⏱️ Advances one original planning or loop event under its full physical authority.
 pub fn step_funded(&mut self,body:&Body,units:usize,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->Result<(RetainedCloneProgress,usize),ValueError>{
  self.step_funded_for(body,None,units,effect)
 }

 /// 📍️ Advances validation only for the original topology selected by the caller.
 pub fn step_funded_for(&mut self,body:&Body,scope:Option<&ReachSet>,units:usize,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->Result<(RetainedCloneProgress,usize),ValueError>{
  self.step_funded_with_positions(body,scope,units,effect,&|_,position|position)
 }

 pub(super) fn step_funded_with_positions(&mut self,body:&Body,scope:Option<&ReachSet>,units:usize,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>,positions:&impl Fn(VertexId,Pnt3)->Pnt3)->Result<(RetainedCloneProgress,usize),ValueError>{
  let grant=effect.grant();
  let demand=self.funded_demands_for(body,scope)?;
  if self.is_complete()||units==0||grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok((Default::default(),0))}
  if self.phase==BodyValidationPhase::CollectingTopology{return self.step_planning_for(body,scope,units,effect)}
  if !effect.performed(1)? { return Ok((Default::default(),0)); }
  let mut receipt=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};
  if let Some(finding)=self.loop_findings.get(self.loop_finding_cursor).copied().flatten(){
   if self.loop_output.is_none(){self.loop_output=Some(ValidationIssue{entity:String::new(),code:finding.code(),message:String::new()});}
   else if self.loop_text_stage<2{
    let entity=self.loop_text_stage==0;let output=self.loop_output.as_mut().unwrap();let text=if entity{&mut output.entity}else{&mut output.message};let length=finding.length(entity)?;
    if text.len()==length{self.loop_text_stage+=1;}
    else if text.capacity()==0{text.try_reserve_exact(length).map_err(|_|allocation())?;receipt.retained_capacity_bytes=text.capacity();}
    else{let from=text.len();let end=from.saturating_add(grant.maximum_copy_bytes).min(length);finding.text(entity,&mut TextSlice{target:text,start:from,end,visited:0}).map_err(|_|refusal())?;receipt.copied_bytes=text.len()-from;}
   }else if demand.capacity_bytes!=0{receipt=self.issues.reserve_append()?;}
   else{let output=self.loop_output.take().unwrap();if let Err(output)=self.issues.push_reserved(output){self.loop_output=Some(output);return Err(refusal())}self.loop_findings[self.loop_finding_cursor]=None;self.loop_finding_cursor+=1;self.loop_text_stage=0;}
  }else if self.loop_finding_cursor!=0{self.loop_finding_cursor=0;self.loop_findings=[None,None];if self.phase==BodyValidationPhase::LoopRings{self.ring_probe+=1;}}
  else if self.phase==BodyValidationPhase::FaceLoopWinding{
   let faces=scoped_count!(body,scope,faces);
   if demand.capacity_bytes!=0 { self.winding.face_areas.try_reserve_exact(faces).map_err(|_|allocation())?; receipt.retained_capacity_bytes=self.winding.face_areas.capacity()*size_of::<f64>(); }
   else if self.winding.face_areas.len()<faces{self.winding.face_areas.push(0.0); }
   else{self.probe_winding_funded(body,scope)?;}
  }
  else if self.phase==BodyValidationPhase::SolidOrientation{self.probe_orientation_funded(body,scope)?;}
  else if self.phase==BodyValidationPhase::DegenerateEdges{self.probe_degenerate_edge_funded(body)?;}
  else if self.phase==BodyValidationPhase::DegenerateFaces{self.probe_degenerate_face_funded(body,scope)?;}
  else if self.phase==BodyValidationPhase::SelfIntersection{self.probe_triangle_pair(body,positions);}
  else if self.phase==BodyValidationPhase::ShellClosure{receipt=self.probe_shell_closure_funded(body,scope)?;}
  else if self.phase==BodyValidationPhase::SameParameter{receipt=self.probe_same_parameter_funded(body,scope,grant)?;}
  else if self.phase==BodyValidationPhase::EdgeValence{self.probe_valence_funded(body,scope);}
  else if self.phase==BodyValidationPhase::ToleranceContainment{self.probe_tolerance_funded(body,scope);}
  else if self.phase==BodyValidationPhase::MissingPcurves{self.probe_pcurve_presence_funded(body,scope);}
  else if self.phase==BodyValidationPhase::LoopRings&&self.loop_id.is_some()&&self.ring_next.is_some()&&self.ring.capacity()==0{let extent=self.live_coedges.checked_add(1).ok_or_else(refusal)?;self.ring.try_reserve_exact(extent).map_err(|_|allocation())?;receipt.retained_capacity_bytes=self.ring.capacity()*std::mem::size_of::<CoedgeId>();}
  else if let Some(loop_id)=self.loop_id{
   if let Some(current)=self.ring_next.take(){self.ring.push(current);if let Some(coedge)=body.coedges.get(current){if self.ring.len()<=self.live_coedges&&self.ring.first().copied()!=Some(coedge.next){self.ring_next=Some(coedge.next);}}}
   else if self.ring_probe<self.ring.len(){
    let index=self.ring_probe;let next=(index+1)%self.ring.len();let pair=body.coedge_endpoints(self.ring[index]).zip(body.coedge_endpoints(self.ring[next]));
    if let Some(((_,end),(start,_)))=pair{let a=body.coedges.get(self.ring[index]).ok_or_else(refusal)?;let b=body.coedges.get(self.ring[next]).ok_or_else(refusal)?;let mut found=0;if end!=start{self.loop_findings[found]=Some(LoopFinding::Ring{loop_id,index,next,mismatch:false});found+=1;}if a.next!=self.ring[next]||b.prev!=self.ring[index]{self.loop_findings[found]=Some(LoopFinding::Ring{loop_id,index,next,mismatch:true});found+=1;}if found==0{self.ring_probe+=1;}}
    else{self.ring_probe+=1;}
   }else{self.ring.clear();self.loop_id=None;}
  }else if self.loop_slot>=scoped_count!(body,scope,loops){self.phase=BodyValidationPhase::EdgeValence;}
  else{if let Some((id,lp))=scoped_slot!(body,scope,loops,self.loop_slot){self.loop_id=Some(id);self.ring_next=Some(lp.first);self.ring_probe=0;}self.loop_slot+=1;}
  self.units_done+=1;self.units_total=self.units_total.max(self.units_done.saturating_add(1));
  if !receipt.fits(grant){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original validation loop allocator exceeded admitted backing").with_retained_progress(receipt))}Ok((receipt,1))
 }
 pub(super) fn hold_topology_finding(&mut self,finding:LoopFinding){self.loop_findings=[Some(finding),None];self.loop_finding_cursor=0;self.loop_text_stage=0;}
 fn probe_valence_funded(&mut self,body:&Body,scope:Option<&ReachSet>){
  let Some(edge)=self.valence_edge else{if self.valence_edge_slot>=scoped_count!(body,scope,edges){self.phase=BodyValidationPhase::ToleranceContainment;return}self.valence_edge=scoped_slot!(body,scope,edges,self.valence_edge_slot).map(|(id,_)|id);self.valence_edge_slot+=1;self.valence_coedge_slot=0;self.valence_uses=0;return};
  if self.valence_coedge_slot<scoped_count!(body,scope,coedges){if let Some((_,coedge))=scoped_slot!(body,scope,coedges,self.valence_coedge_slot){self.valence_uses+=usize::from(semio_framework_3d_brep_representation::topology::coedge_uses_edge(coedge,edge));}self.valence_coedge_slot+=1;}
  else{if self.valence_uses>2{self.hold_topology_finding(LoopFinding::Valence{edge,uses:self.valence_uses});}self.valence_edge=None;}
 }
 fn probe_pcurve_presence_funded(&mut self,body:&Body,scope:Option<&ReachSet>){
  if self.pcurve_slot>=scoped_count!(body,scope,coedges){self.phase=BodyValidationPhase::SameParameter;return}
  if let Some((coedge,value))=scoped_slot!(body,scope,coedges,self.pcurve_slot){if value.pcurve.is_none(){self.hold_topology_finding(LoopFinding::MissingPcurve{coedge});}}
  self.pcurve_slot+=1;
 }
 fn probe_tolerance_funded(&mut self,body:&Body,scope:Option<&ReachSet>){
  if self.tolerance_stage==0{
   let Some(&edge_id)=self.edges.get(self.tolerance_edge_cursor)else{self.tolerance_stage=1;return};
   if self.tolerance_vertex>=2{self.tolerance_edge_cursor+=1;self.tolerance_vertex=0;return}
   let endpoint=self.tolerance_vertex;self.tolerance_vertex+=1;let Some(edge)=body.edges.get(edge_id)else{return};let vertex_id=[edge.v0,edge.v1][endpoint];
   if body.vertices.get(vertex_id).is_some_and(|vertex|vertex.tol.value()>edge.tol.value()){self.hold_topology_finding(LoopFinding::Tolerance{finer_kind:"vertex",finer:vertex_id.raw_index(),coarser_kind:"edge",coarser:edge_id.raw_index()});}return;
  }
  match self.tolerance_faces.step(body,self.live_coedges,scope){FaceCoedgeTurn::Pending=>{},FaceCoedgeTurn::Complete=>self.phase=BodyValidationPhase::MissingPcurves,FaceCoedgeTurn::Coedge(face_id,coedge_id)=>{
   let Some(face)=body.faces.get(face_id)else{return};let Some(coedge)=body.coedges.get(coedge_id)else{return};
   if body.edges.get(coedge.edge).is_some_and(|edge|edge.tol.value()>face.tol.value()){self.hold_topology_finding(LoopFinding::Tolerance{finer_kind:"edge",finer:coedge.edge.raw_index(),coarser_kind:"face",coarser:face_id.raw_index()});}
  }}
 }

}

#[derive(Default)]
pub(super) struct ShellClosureCursor {
    shell_slot: usize,
    face_index: usize,
    face: FaceCoedgeCursor,
    testing_edges: bool,
    edge_slot: usize,
    uses: Vec<(usize, usize)>,
}
semio_framework_value::artifact_retire_struct!(ShellClosureCursor {shell_slot, face_index, face, testing_edges, edge_slot, uses});

impl BodyValidationJob {
    fn shell_closure_demand(&self,body:&Body,scope:Option<&ReachSet>) -> Result<RetirementDemand, ValueError> {
        let mut demand = metadata(1);
        let count=scoped_count!(body,scope,edges);
        let width = std::mem::size_of::<(usize, usize)>();
        if self.closure.shell_slot < scoped_count!(body,scope,shells) {
            if count != 0 && self.closure.uses.capacity() == 0 {
                demand.capacity_bytes = count.checked_mul(width).ok_or_else(refusal)?;
            } else if self.closure.uses.len() < count || self.closure.face.next.is_some() || (self.closure.testing_edges && self.closure.edge_slot < scoped_count!(body,scope,edges)) {
                demand.copy_bytes = width;
            }
        }
        Ok(demand)
    }

    pub(super) fn probe_shell_closure_funded(&mut self,body:&Body,scope:Option<&ReachSet>) -> Result<RetainedCloneProgress, ValueError> {
        let demand=self.shell_closure_demand(body,scope)?;
        let mut receipt = RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() };
        let cursor = &mut self.closure;
        let count=scoped_count!(body,scope,edges);
        if cursor.shell_slot >= scoped_count!(body,scope,shells) {
            self.phase = BodyValidationPhase::FaceLoopWinding;
            return Ok(receipt);
        }
        if demand.capacity_bytes != 0 {
            cursor.uses.try_reserve_exact(count).map_err(|_| allocation())?;
            receipt.retained_capacity_bytes = cursor.uses.capacity() * std::mem::size_of::<(usize, usize)>();
        } else if cursor.uses.len() < count {
            cursor.uses.push((0, 0));
        } else if let Some((shell_id, shell)) = scoped_slot!(body,scope,shells,cursor.shell_slot) {
            if cursor.testing_edges {
                if cursor.edge_slot==scoped_count!(body,scope,edges){
                    cursor.shell_slot += 1;
                    cursor.edge_slot = 0;
                    cursor.face_index = 0;
                    cursor.testing_edges = false;
                } else {
                    if let Some((edge,_))=scoped_slot!(body,scope,edges,cursor.edge_slot){
                        let index=scoped_index!(scope,edges,edge).ok_or_else(refusal)?;
                        let (uses,forward)=cursor.uses[index];
                        let permits_boundary=scope.is_some_and(|selection|selection.solids.0.is_empty());
                        let inconsistent=match uses{0=>false,1=>!permits_boundary,2=>forward!=1,_=>true};
                        if inconsistent&&!funded_point_edge(body,edge)?{
                            self.loop_findings[0]=Some(LoopFinding::Shell{shell:shell_id.raw_index(),edge,uses});
                        }
                        cursor.uses[index]=(0,0);
                    }
                    cursor.edge_slot += 1;
                }
            } else if cursor.face.face.is_some() {
                if let FaceCoedgeTurn::Coedge(face, id) = cursor.face.step(body,self.live_coedges,scope) {
                    if let Some(coedge) = body.coedges.get(id) {
                        let tally = cursor.uses.get_mut(scoped_index!(scope,edges,coedge.edge).ok_or_else(refusal)?).ok_or_else(refusal)?;
                        tally.0 += 1;
                        tally.1 += usize::from(coedge.forward != body.faces.get(face).is_some_and(|row| row.flipped));
                    }
                }
            } else if let Some(&face) = shell.faces.get(cursor.face_index) {
                cursor.face = FaceCoedgeCursor { face: Some(face), ..Default::default() };
                cursor.face_index += 1;
            } else {
                cursor.testing_edges = true;
            }
        } else {
            cursor.shell_slot += 1;
        }
        Ok(receipt)
    }
}

fn funded_point_edge(body: &Body, id: EdgeId) -> Result<bool, ValueError> {
    use semio_framework_3d_brep_curve::Curve3;
    let Some(edge) = body.edges.get(id) else { return Ok(false) };
    if edge.v0 != edge.v1 { return Ok(false); }
    let Some(curve) = body.curves3.get(edge.curve) else { return Ok(false) };
    let span = (edge.range.1 - edge.range.0).abs();
    if span == 0.0 { return Ok(true); }
    let (low, high) = match curve {
        Curve3::Line { dir, .. } => { let speed = dir.x.hypot(dir.y).hypot(dir.z); (speed, speed) }
        Curve3::Circle { radius, .. } => (radius.abs(), radius.abs()),
        Curve3::Ellipse { major_radius, minor_radius, .. } => (major_radius.abs().min(minor_radius.abs()), major_radius.abs().max(minor_radius.abs())),
        Curve3::Nurbs { .. } => return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "point-edge spline length needs funded integration")),
    };
    if span * high <= edge.tol.value() { return Ok(true); }
    if span * low > edge.tol.value() { return Ok(false); }
    Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "point-edge ellipse length needs funded integration"))
}

#[derive(Default)]
pub(super) struct WindingCursor {
    faces: FaceCoedgeCursor,
    pair: Option<(FaceId, CoedgeId)>,
    sample: usize,
    count: usize,
    first: Option<(f64, f64)>,
    previous: Option<(f64, f64)>,
    before_last: Option<(f64, f64)>,
    previous_u: Option<f64>,
    pole: bool,
    twice_area: f64,
    face_areas: Vec<f64>,
    outer_volume: Option<f64>,
}
semio_framework_value::artifact_retire_struct!(WindingCursor {faces,pair,sample,count,first,previous,before_last,previous_u,pole,twice_area,face_areas,outer_volume});

impl WindingCursor {
    fn visit(&mut self, point: (f64, f64)) {
        if let Some(previous) = self.previous {
            if (previous.0 - point.0).abs() <= 1e-12 && (previous.1 - point.1).abs() <= 1e-12 { return; }
            self.twice_area += previous.0 * point.1 - point.0 * previous.1;
        }
        self.first.get_or_insert(point);
        self.before_last = self.previous;
        self.previous = Some(point);
    }

    fn finish(&mut self) -> f64 {
        let mut twice = self.twice_area;
        if let (Some(a), Some(b)) = (self.previous, self.first) {
            if (a.0 - b.0).abs() <= 1e-12 && (a.1 - b.1).abs() <= 1e-12 {
                if let Some(p) = self.before_last { twice += p.0 * (b.1 - a.1) - (b.0 - a.0) * p.1; }
            } else {
                twice += a.0 * b.1 - b.0 * a.1;
            }
        }
        self.twice_area = 0.0;
        self.first = None;
        self.previous = None;
        self.before_last = None;
        self.previous_u = None;
        self.pole = false;
        twice * 0.5
    }
}

impl BodyValidationJob {
    pub(super) fn probe_winding_funded(&mut self,body:&Body,scope:Option<&ReachSet>) -> Result<(), ValueError> {
        let w = &mut self.winding;
        let Some((face_id, id)) = w.pair else {
            match w.faces.step(body,self.live_coedges,scope) {
                FaceCoedgeTurn::Complete => self.phase = BodyValidationPhase::SolidOrientation,
                FaceCoedgeTurn::Pending => {}
                FaceCoedgeTurn::Coedge(face, coedge) => {
                    let co = body.coedges.get(coedge).ok_or_else(refusal)?;
                    w.count = mass_properties::coedge_sample_count(body, co, 1e-3, false).max(2);
                    w.sample = 0;
                    w.pair = Some((face, coedge));
                }
            }
            return Ok(());
        };
        let face = body.faces.get(face_id).ok_or_else(refusal)?;
        let surface = body.surfaces.get(face.surface).ok_or_else(refusal)?;
        let co = body.coedges.get(id).ok_or_else(refusal)?;
        if matches!(surface,Surface::Nurbs{..}) || matches!(co.pcurve.and_then(|pc| body.curves2.get(pc)),Some(semio_framework_3d_brep_curve::Curve2::Nurbs{..})) || (co.pcurve.is_none() && matches!(body.edges.get(co.edge).and_then(|e| body.curves3.get(e.curve)),Some(semio_framework_3d_brep_curve::Curve3::Nurbs{..}))) {
            return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "original spline winding needs funded evaluation"));
        }
        let fraction = w.sample as f64 / (w.count - 1) as f64;
        let uv = mass_properties::coedge_uv_sample(body, co, surface, fraction).map_err(|_| refusal())?;
        let at_pole = surface.is_degenerate_uv(uv.x, uv.y);
        let last = w.sample + 1 == w.count;
        let end_of_ring = w.faces.next.is_none();
        if !last || end_of_ring || at_pole {
            let mut u = uv.x;
            if surface.is_u_periodic() {
                if !w.pole {
                    if let Some(previous) = w.previous_u { u = mass_properties::unwrap_u(previous, u); }
                }
                w.previous_u = Some(u);
            }
            w.pole = at_pole;
            w.visit((u, uv.y));
        }
        w.sample += 1;
        if last {
            w.pair = None;
            if end_of_ring {
                let area = w.finish();
                let inner = if w.faces.loop_slot == 1 { None } else { w.faces.start.and_then(|_| face.inners.get(w.faces.loop_slot - 2).copied()) };
                let face_area = w.face_areas.get_mut(scoped_index!(scope,faces,face_id).ok_or_else(refusal)?).ok_or_else(refusal)?;
                *face_area += if inner.is_none() { area.abs() } else { -area.abs() };
                if (inner.is_none() && area < 0.0) || (inner.is_some() && area > 0.0) {
                    self.loop_findings[0] = Some(LoopFinding::Winding { face: face_id, inner, area });
                }
            }
        }
        Ok(())
    }
}

impl BodyValidationJob {
    fn probe_orientation_funded(&mut self,body:&Body,scope:Option<&ReachSet>) -> Result<(), ValueError> {
        let Some(unit) = self.shells.get_mut(self.shell_cursor) else {
            self.phase = BodyValidationPhase::DegenerateEdges;
            return Ok(());
        };
        if let Some(&id) = unit.faces.get(self.shell_face_cursor) {
            let face = body.faces.get(id).ok_or_else(refusal)?;
            let Some(Surface::Plane { frame }) = body.surfaces.get(face.surface) else {
                return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"curved shell orientation requires funded surface quadrature"));
            };
            let area = *self.winding.face_areas.get(scoped_index!(scope,faces,id).ok_or_else(refusal)?).ok_or_else(refusal)?;
            let height = frame.origin.x * frame.z.x + frame.origin.y * frame.z.y + frame.origin.z * frame.z.z;
            if let Some(total) = unit.total.as_mut() {
                *total += if face.flipped { -height } else { height } * area / 3.0;
            }
            self.shell_face_cursor += 1;
        } else {
            if unit.faces.is_empty() { unit.total = None; }
            if unit.outer {
                self.winding.outer_volume = unit.total;
                if let Some(volume) = unit.total.filter(|v| *v < 0.0) {
                    self.loop_findings[0] = Some(LoopFinding::Inward { solid: unit.solid.raw_index(), volume });
                }
            } else if let Some((outer, inner)) = self.winding.outer_volume.zip(unit.total) {
                if outer.signum() == inner.signum() {
                    self.loop_findings[0] = Some(LoopFinding::VoidOrientation { solid: unit.solid.raw_index(), shell: unit.shell.raw_index() });
                }
            }
            self.shell_cursor += 1;
            self.shell_face_cursor = 0;
        }
        Ok(())
    }

    fn probe_degenerate_edge_funded(&mut self, body: &Body) -> Result<(), ValueError> {
        use semio_framework_3d_brep_curve::Curve3;
        let Some(&id) = self.edges.get(self.edge_cursor) else {
            self.phase = BodyValidationPhase::DegenerateFaces;
            return Ok(());
        };
        let edge = body.edges.get(id).ok_or_else(refusal)?;
        let curve = body.curves3.get(edge.curve).ok_or_else(refusal)?;
        let span = (edge.range.1 - edge.range.0).abs();
        let tolerance = edge.tol.value();
        let length = match curve {
            Curve3::Line { dir, .. } => dir.x.hypot(dir.y).hypot(dir.z) * span,
            Curve3::Circle { radius, .. } => radius.abs() * span,
            Curve3::Ellipse { major_radius, minor_radius, .. } if major_radius.abs().min(minor_radius.abs()) * span >= tolerance => tolerance,
            _ if span == 0.0 => 0.0,
            _ => return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"curved edge degeneracy requires funded length integration")),
        };
        if length < tolerance && edge.v0 != edge.v1 {
            self.loop_findings[0] = Some(LoopFinding::ShortEdge { edge:id, length, tolerance });
        }
        self.edge_cursor += 1;
        Ok(())
    }

    fn probe_degenerate_face_funded(&mut self,body:&Body,scope:Option<&ReachSet>) -> Result<(), ValueError> {
        let Some(&id) = self.faces.get(self.face_cursor) else {
            self.phase = BodyValidationPhase::SelfIntersection;
            return Ok(());
        };
        let face = body.faces.get(id).ok_or_else(refusal)?;
        if !matches!(body.surfaces.get(face.surface),Some(Surface::Plane{..})) {
            return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"curved face area requires funded surface quadrature"));
        }
        let area = *self.winding.face_areas.get(scoped_index!(scope,faces,id).ok_or_else(refusal)?).ok_or_else(refusal)?;
        let tolerance2 = face.tol.value().powi(2);
        if area < tolerance2 { self.loop_findings[0] = Some(LoopFinding::Sliver { face:id, area, tolerance2 }); }
        self.face_cursor += 1;
        Ok(())
    }
}
