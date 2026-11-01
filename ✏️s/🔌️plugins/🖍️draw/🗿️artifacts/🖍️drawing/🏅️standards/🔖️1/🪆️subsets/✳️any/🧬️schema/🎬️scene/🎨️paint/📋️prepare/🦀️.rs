//! 🎬️ Owned vector plans and indexed painted caches publish as one completed scene.
use super::{PaintedPathPrepareJob,PaintedPathPreparation,PaintedPathRegions,PaintedRegionLimits,PaintedPreparationRetirement};
use semio_s_artifact_draw_drawing_scene::preparation::{DocumentScenePlan,DocumentSceneContent,DocumentSceneError};
use crate::picking::PaintedPathStroke;
use semio_framework_value_persistent::work::RetainedWorkLedger;
use semio_framework_value::RetirementDemand;
use semio_framework_job::StepContext;

type Point=[f64;2];type Bounds=[f64;4];
#[derive(Clone,Debug)]
#[derive(semio_framework_value::RetireOwned)]
pub struct PaintedSceneLimits{pub max_nodes:usize,pub max_segments:usize,pub max_points:usize,pub max_contours:usize,pub max_work:u64}
#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]
pub struct PreparedSceneCounts{pub nodes:usize,pub segments:usize,pub points:usize,pub contours:usize}
#[derive(Debug)]
#[derive(semio_framework_value::RetireOwned)]
pub enum PreparedPaint{Empty,Path(PaintedPathRegions),Image([Point;4])}
#[derive(Debug)]
#[derive(semio_framework_value::RetireOwned)]
pub struct PreparedGeometry{pub geometry_bounds:Option<Bounds>,pub bounds:Option<Bounds>,pub paint:PreparedPaint}
#[derive(Debug)]
#[derive(semio_framework_value::RetireOwned)]
pub struct PreparedScene{pub plan:DocumentScenePlan,pub geometry:Vec<PreparedGeometry>,pub flatness:f64,pub counts:PreparedSceneCounts}
#[derive(Clone,Copy,Debug)]
pub struct ScenePaintProgress{pub phase:&'static str,pub work:u64,pub nodes:usize,pub segments:usize,pub points:usize,pub contours:usize,pub done:bool}
fn valid(v:f64)->bool{v.is_finite()&&v.abs()<=1e9}
fn union(a:Option<Bounds>,b:Bounds)->Bounds{a.map_or(b,|a|[a[0].min(b[0]),a[1].min(b[1]),a[2].max(b[2]),a[3].max(b[3])])}
fn rectangle(x:f64,y:f64,w:f64,h:f64,m:[f64;6])->Result<PreparedGeometry,DocumentSceneError>{let mut corners=[[x,y],[x+w,y],[x+w,y+h],[x,y+h]];let mut bounds=None;for p in &mut corners{*p=[m[0]*p[0]+m[2]*p[1]+m[4],m[1]*p[0]+m[3]*p[1]+m[5]];if !p.iter().copied().all(valid){return Err(DocumentSceneError::literal("Painted scene coordinate limit exceeded"));}bounds=Some(union(bounds,[p[0],p[1],p[0],p[1]]));}Ok(PreparedGeometry{geometry_bounds:bounds,bounds,paint:PreparedPaint::Image(corners)})}
semio_framework_2d::physical_work_retirement!(PreparedSceneCloseJob,PreparedScene);
/// 🧱️ Constructor adoption preserves every actual owner until funded publication or controlled retirement.
#[derive(semio_framework_value::RetireOwned)]
#[value(send_only)]
pub struct ScenePaintJob{
 pub plan:Option<DocumentScenePlan>,limits:PaintedSceneLimits,flatness:f64,geometry:Vec<PreparedGeometry>,counts:PreparedSceneCounts,pub phase:&'static str,work:u64,validated:bool,index:usize,segment:usize,pen:Point,origin:Point,bounds:Option<Bounds>,
 pub path:Option<PaintedPathPrepareJob>,retired_paths:Vec<PaintedPreparationRetirement>,path_retirement:Option<PaintedPreparationRetirement>,pending:Option<PreparedGeometry>,output:Option<PreparedScene>,dash:Vec<f64>,dash_at:usize,cancelled:bool,failure:Option<DocumentSceneError>,demand:RetirementDemand,last_turn_work:u64,last_turn_semantic_work:u64,step_semantic_work:u64,
}
impl ScenePaintJob{
 pub fn new(plan:DocumentScenePlan,flatness:f64,limits:PaintedSceneLimits)->Self{Self{plan:Some(plan),limits,flatness,geometry:Vec::new(),counts:Default::default(),phase:"nodes",work:0,validated:false,index:0,segment:0,pen:[0.;2],origin:[0.;2],bounds:None,path:None,retired_paths:Vec::new(),path_retirement:None,pending:None,output:None,dash:Vec::new(),dash_at:0,cancelled:false,failure:None,demand:RetirementDemand{depth:1,..Default::default()},last_turn_work:0,last_turn_semantic_work:0,step_semantic_work:0}}
 fn validate(&mut self)->Result<(),DocumentSceneError>{let l=&self.limits;if !self.flatness.is_finite()||!(1e-6..=16.).contains(&self.flatness)||l.max_nodes==0||l.max_nodes>1024||l.max_segments==0||l.max_segments>65536||l.max_points==0||l.max_points>262144||l.max_contours==0||l.max_contours>65536||l.max_work==0||l.max_work>1000000000{return Err(DocumentSceneError::literal("Invalid painted scene contract"));}if self.plan.as_ref().ok_or_else(DocumentSceneError::held)?.nodes.len()>l.max_nodes{return Err(DocumentSceneError::literal("Painted scene node limit exceeded"));}self.validated=true;Ok(())}
 fn admit(&mut self,work:&mut RetainedWorkLedger)->Result<(),DocumentSceneError>{
  let g=self.pending.as_ref().ok_or_else(DocumentSceneError::held)?;let(points,contours)=match &g.paint{PreparedPaint::Empty=>(0,0),PreparedPaint::Path(r)=>(r.points,r.contours),_=>(4,1)};if self.counts.points+points>self.limits.max_points||self.counts.contours+contours>self.limits.max_contours{return Err(DocumentSceneError::literal("Painted scene cache count limit exceeded"));}
  let needed=self.geometry.len()+1;let target=if self.geometry.capacity()>=needed{needed}else{needed.max(self.geometry.capacity().saturating_mul(2).min(self.limits.max_nodes))};if !work.reserve_vec(&mut self.geometry,target).map_err(DocumentSceneError::literal)?||!work.vec_slot(&mut self.geometry).map_err(DocumentSceneError::literal)?{return Ok(());}
  work.vec_push(&mut self.geometry,self.pending.take().ok_or_else(DocumentSceneError::held)?).map_err(|(message,value)|{self.pending=Some(value);DocumentSceneError::literal(message)})?;self.counts.points+=points;self.counts.contours+=contours;self.counts.nodes+=1;self.index+=1;self.phase="nodes";self.step_semantic_work=1;Ok(())
 }
 fn step(&mut self,work:&mut RetainedWorkLedger)->Result<(),DocumentSceneError>{
  if self.path_retirement.is_some(){let needed=self.retired_paths.len()+1;let target=if self.retired_paths.capacity()>=needed{needed}else{needed.max(self.retired_paths.capacity().saturating_mul(2).min(self.limits.max_nodes))};if !work.reserve_vec(&mut self.retired_paths,target).map_err(DocumentSceneError::literal)?||!work.vec_slot(&mut self.retired_paths).map_err(DocumentSceneError::literal)?{return Ok(());}work.vec_push(&mut self.retired_paths,self.path_retirement.take().ok_or_else(DocumentSceneError::held)?).map_err(|(message,value)|{self.path_retirement=Some(value);DocumentSceneError::literal(message)})?;return Ok(());}
  if !self.validated{self.step_semantic_work=1;return self.validate();}
  let node=self.plan.as_ref().ok_or_else(DocumentSceneError::held)?.nodes.get(self.index);match self.phase{
   "nodes"=>{
    let Some(node)=node else{if !work.copy_bytes(0){return Ok(());}self.output=Some(PreparedScene{plan:self.plan.take().ok_or_else(DocumentSceneError::held)?,geometry:std::mem::take(&mut self.geometry),flatness:self.flatness,counts:self.counts});self.phase="complete";return Ok(());};
    if !node.transform.into_iter().all(valid){return Err(DocumentSceneError::literal("Painted scene transform limit exceeded"));}
    if matches!(node.content,DocumentSceneContent::Boolean{..}|DocumentSceneContent::Trace{..}){return Err(DocumentSceneError::literal("Unresolved algorithm in painted scene"));}
    if (!node.visible||matches!(node.content,DocumentSceneContent::Group{..}|DocumentSceneContent::Image{..}))&&!work.copy_bytes(std::mem::size_of::<PreparedGeometry>()){return Ok(());}
    if let DocumentSceneContent::Path{segments,..}|DocumentSceneContent::Glyphs{segments,..}=&node.content{if self.counts.segments+segments.len()>self.limits.max_segments{return Err(DocumentSceneError::literal("Painted scene segment limit exceeded"));}self.counts.segments+=segments.len();}
    self.bounds=None;self.segment=0;self.dash_at=0;self.pen=[0.;2];self.origin=self.pen;
    if !node.visible||matches!(node.content,DocumentSceneContent::Group{..}){self.pending=Some(PreparedGeometry{geometry_bounds:None,bounds:None,paint:PreparedPaint::Empty});self.phase="pathCleanup";}
    else{match &node.content{
     DocumentSceneContent::Path{stroke,..}|DocumentSceneContent::Glyphs{stroke,..}=>{if stroke.as_ref().and_then(|s|s.dash.as_ref()).is_some_and(|d|d.len()>1024){return Err(DocumentSceneError::literal("Painted scene stroke limit exceeded"));}self.phase="bounds";self.step_semantic_work=1;}
     DocumentSceneContent::Image{width,height,..}=>{self.pending=Some(rectangle(0.,0.,*width,*height,node.transform)?);self.phase="pathCleanup";}
     DocumentSceneContent::Text{..}=>return Err(DocumentSceneError::literal("Unresolved semantic text in painted scene")),_=>unreachable!()
    }}
   }
   "bounds"=>{
    let node=node.ok_or_else(DocumentSceneError::held)?;let(DocumentSceneContent::Path{segments,..}|DocumentSceneContent::Glyphs{segments,..})=&node.content else{unreachable!()};
    if let Some(s)=segments.get(self.segment){
     if !matches!(s,semio_s_artifact_draw_drawing_model::PathSegment::Close)||self.bounds.is_some(){let b=semio_s_artifact_draw_drawing_geometry::segment_bounds(s,self.pen,self.origin,node.transform);let world=[b[0],b[1],b[0]+b[2],b[1]+b[3]];if !world.into_iter().all(valid){return Err(DocumentSceneError::literal("Painted scene bounds limit exceeded"));}self.bounds=Some(union(self.bounds,world));}
     match s{semio_s_artifact_draw_drawing_model::PathSegment::Move{to}=>{self.pen=*to;self.origin=*to},semio_s_artifact_draw_drawing_model::PathSegment::Close=>self.pen=self.origin,semio_s_artifact_draw_drawing_model::PathSegment::Line{to}|semio_s_artifact_draw_drawing_model::PathSegment::Quad{to,..}|semio_s_artifact_draw_drawing_model::PathSegment::Cubic{to,..}|semio_s_artifact_draw_drawing_model::PathSegment::Arc{to,..}=>self.pen=*to}self.segment+=1;self.step_semantic_work=1;
    }else{self.phase="pathInput";}
   }
   "pathInput"=>{
    let node=node.ok_or_else(DocumentSceneError::held)?;let(DocumentSceneContent::Path{fill,fill_rule,stroke,..}|DocumentSceneContent::Glyphs{fill,fill_rule,stroke,..})=&node.content else{unreachable!()};
    if let Some(dash)=stroke.as_ref().and_then(|s|s.dash.as_ref()){if self.dash_at<dash.len(){if !work.reserve_vec(&mut self.dash,dash.len()).map_err(DocumentSceneError::literal)?||!work.vec_slot(&mut self.dash).map_err(DocumentSceneError::literal)?{return Ok(());}work.vec_push(&mut self.dash,dash[self.dash_at]).map_err(|_|DocumentSceneError::literal("Painted scene stroke copy authority lost"))?;self.dash_at+=1;return Ok(());}}
    let bytes=std::mem::size_of::<[f64;6]>()+std::mem::size_of::<bool>()+std::mem::size_of::<semio_s_artifact_draw_drawing_model::fill_rule::FillRule>()+stroke.as_ref().map_or(0,|s|std::mem::size_of_val(&s.width)+std::mem::size_of_val(&s.cap)+std::mem::size_of_val(&s.join));if !work.copy_bytes(bytes){return Ok(());}
    let stroke=stroke.as_ref().map(|s|PaintedPathStroke{width:s.width,cap:s.cap.clone(),join:s.join.clone(),dash:std::mem::take(&mut self.dash)});
    self.path=Some(PaintedPathPrepareJob::new(PaintedPathPreparation{transform:node.transform,flatness:self.flatness,fill:fill.is_some(),fill_rule:fill_rule.clone(),stroke,limits:PaintedRegionLimits{max_segments:65536,max_points:262144,max_contours:65536,max_work:self.limits.max_work-self.work}}));self.phase="path";
   }
   "path"=>{
    let node=node.ok_or_else(DocumentSceneError::held)?;let(DocumentSceneContent::Path{segments,..}|DocumentSceneContent::Glyphs{segments,..})=&node.content else{unreachable!()};
    if self.path.as_ref().ok_or_else(DocumentSceneError::held)?.result().is_ok(){
     let child=self.path.as_ref().ok_or_else(DocumentSceneError::held)?;let bytes=usize::from(self.bounds.is_some())*std::mem::size_of::<Bounds>()+usize::from(child.result()?.bounds.is_some())*std::mem::size_of::<Bounds>();if !work.copy_bytes(bytes){return Ok(());}
     let(close,output)=self.path.take().ok_or_else(DocumentSceneError::held)?.into_retirement();self.path_retirement=Some(close);let regions=output.ok_or_else(DocumentSceneError::held)?;self.pending=Some(PreparedGeometry{geometry_bounds:self.bounds,bounds:regions.bounds,paint:PreparedPaint::Path(regions)});self.phase="pathCleanup";
    }else{let child=self.path.as_mut().ok_or_else(DocumentSceneError::held)?;let result=child.advance(1,|index|segments.get(index).cloned(),work);self.last_turn_work+=child.last_turn_work();self.step_semantic_work+=child.last_turn_semantic_work();result?;}
   }
   "pathCleanup"=>self.admit(work)?,"complete"=>{},_=>unreachable!()
  }Ok(())
 }
 fn semantic_step_pending(&self)->bool{if !self.validated{return true}if self.path_retirement.is_some(){return false}match self.phase{"nodes"=>self.plan.as_ref().is_some_and(|plan|self.index<plan.nodes.len()),"pathInput"=>false,"pathCleanup"=>true,"bounds"=>self.plan.as_ref().and_then(|plan|plan.nodes.get(self.index)).is_some_and(|node|match &node.content{DocumentSceneContent::Path{segments,..}|DocumentSceneContent::Glyphs{segments,..}=>self.segment<segments.len(),_=>false}),_=>false}}
 pub fn next_work_demand(&self)->RetirementDemand{self.demand}
 pub fn last_turn_work(&self)->u64{self.last_turn_work}
 pub fn last_turn_semantic_work(&self)->u64{self.last_turn_semantic_work}
 pub fn failure_pending(&self)->bool{self.failure.is_some()||self.path.as_ref().is_some_and(PaintedPathPrepareJob::failure_pending)}
 pub fn settlement_pending(&self)->bool{if self.cancelled||self.failure.is_some(){return false}if self.phase=="path"&&self.path_retirement.is_none(){return self.path.as_ref().is_some_and(|child|child.result().is_ok()||child.settlement_pending())}!self.semantic_step_pending()}
 pub fn advance(&mut self,budget:usize,work:&mut RetainedWorkLedger)->Result<ScenePaintProgress,DocumentSceneError>{
  self.last_turn_work=0;self.last_turn_semantic_work=0;if budget==0||budget as u128>9007199254740991{return Err(DocumentSceneError::literal("Invalid painted scene work grant"));}if self.cancelled{return Err(DocumentSceneError::Cancelled);}if let Some(error)=&self.failure{return Err(error.clone());}
  for _ in 0..budget{
   if self.phase=="complete"{break}let d=self.demand;if !work.admits_absolute(d){work.require_absolute(d);break}
   if self.validated&&self.semantic_step_pending()&&self.work>=self.limits.max_work&&!self.failure_pending(){let error=DocumentSceneError::literal("Painted scene work limit exceeded");self.failure=Some(error.clone());return Err(error)}
   let settling=self.failure_pending();let semantic_pending=self.semantic_step_pending();let structural=self.phase!="path"||self.path_retirement.is_some()||self.path.as_ref().is_some_and(|child|child.result().is_ok());let before=work.receipt();let before_cpu=self.last_turn_work;
   let Some(root)=work.enter()else{self.demand=work.next_demand();break};
   self.step_semantic_work=0;let result=self.step(work);if structural&&semantic_pending&&result.is_err()&&!work.blocked()&&self.step_semantic_work==0{self.step_semantic_work=1;}let changed=work.receipt()!=before;let actual=self.last_turn_work>before_cpu||changed||(structural&&!work.blocked());
   if actual{work.note_source(root);if self.last_turn_work==before_cpu{self.last_turn_work+=1;}if !settling{self.work+=self.step_semantic_work;self.last_turn_semantic_work+=self.step_semantic_work;}}work.exit();
   self.demand=work.next_demand();if let Err(error)=result{self.failure=Some(error.clone());return Err(error)}if work.blocked()||!actual{break}
  }Ok(ScenePaintProgress{phase:self.phase,work:self.work,nodes:self.counts.nodes,segments:self.counts.segments,points:self.counts.points,contours:self.counts.contours,done:self.phase=="complete"})
 }
 pub fn advance_original(&mut self,cx:&mut StepContext<'_>)->Result<ScenePaintProgress,semio_framework_job::OriginalWorkFailure<DocumentSceneError>>{
  self.last_turn_work=0;self.last_turn_semantic_work=0;semio_framework_job::OriginalWorkFailure::qualify(cx)?;let mut work=RetainedWorkLedger::new(cx.retained_grant());let result=if cx.is_cancelled(){Err(DocumentSceneError::Cancelled)}else if cx.should_yield(){Ok(ScenePaintProgress{phase:self.phase,work:self.work,nodes:self.counts.nodes,segments:self.counts.segments,points:self.counts.points,contours:self.counts.contours,done:self.phase=="complete"})}else{self.advance(1,&mut work)};
  match result{Ok(progress)=>{semio_framework_job::OriginalWorkFailure::<DocumentSceneError>::fold(cx,None,work.receipt(),self.last_turn_work)?;Ok(progress)},Err(error)=>Err(semio_framework_job::OriginalWorkFailure::fold(cx,Some(error),work.receipt(),self.last_turn_work).expect_err("an original body failure remains owned"))}
 }
 pub fn result(&self)->Result<&PreparedScene,DocumentSceneError>{if self.cancelled{return Err(DocumentSceneError::Cancelled)}if let Some(error)=&self.failure{return Err(error.clone())}if self.phase!="complete"{return Err(DocumentSceneError::Incomplete)}Ok(self.output.as_ref().ok_or_else(DocumentSceneError::held)?)}
 pub fn cancel(&mut self){self.cancelled=true;}
 pub fn into_retirement(mut self)->(ScenePaintRetirement,Option<PreparedScene>){let output=if self.result().is_ok(){self.output.take()}else{None};self.cancelled=true;(ScenePaintRetirement::new(self),output)}
}
semio_framework_2d::physical_work_retirement!(ScenePaintRetirement,ScenePaintJob);
semio_framework_value::artifact_retire_leaf!(PreparedSceneCounts,ScenePaintProgress);

#[path="🎯️query/🦀️.rs"]
pub mod query;
