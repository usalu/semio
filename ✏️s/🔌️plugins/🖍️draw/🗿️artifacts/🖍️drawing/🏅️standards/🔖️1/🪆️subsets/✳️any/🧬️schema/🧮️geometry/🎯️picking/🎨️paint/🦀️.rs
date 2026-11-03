//! 🎨️ Borrowed path admission, painted winding and real geometry retirement under work grants.
use semio_s_artifact_draw_drawing_model::PathSegment;
use semio_s_artifact_draw_drawing_model::fill_rule::FillRule;
use semio_s_artifact_draw_drawing_model::stroke::{StrokeCap, StrokeJoin};
use semio_framework_2d::{flatten::{PathFlattenJob,PathFlattenInput,PathFlattenRetirement,FlatContour},stroke::{StrokeOutlineJob,StrokeOutlineInput,StrokeOutlineRetirement,StrokeGeometryStyle,StrokeGeometryCap,StrokeGeometryJoin,StrokeContour},retirement::WorkRetirementProgress};
use semio_framework_value::{work::RetainedWorkLedger,RetirementDemand};
use semio_s_artifact_draw_drawing_geometry::raster::close_one;
type Point=[f64;2];
#[derive(Clone,Debug)]
#[derive(semio_framework_value::RetireOwned)]
pub struct PaintedPathStroke{pub width:f64,pub cap:StrokeCap,pub join:StrokeJoin,pub dash:Vec<f64>}
#[derive(Clone,Debug)]
pub struct PaintedPathQuery{pub point:Point,pub transform:[f64;6],pub tolerance:f64,pub flatness:f64,pub fill:bool,pub fill_rule:FillRule,pub stroke:Option<PaintedPathStroke>}
#[derive(Clone,Copy,Debug,PartialEq)]
pub struct PaintedPathHit{pub contains:bool,pub bounds:Option<[f64;4]>}
#[derive(Clone,Copy,Debug)]
pub struct PaintedPathProgress{pub phase:&'static str,pub work:u64,pub done:bool}
fn valid(v:f64)->bool{v.is_finite()&&v.abs()<=1e9}
fn map(p:Point,m:[f64;6])->Point{[m[0]*p[0]+m[2]*p[1]+m[4],m[1]*p[0]+m[3]*p[1]+m[5]]}
fn retire_flat(flat:&mut Vec<FlatContour>)->bool{if flat.pop().is_some(){false}else{*flat=Vec::new();true}}
fn retire_polygons(polygons:&mut Vec<Vec<Point>>)->bool{if polygons.pop().is_some(){false}else{*polygons=Vec::new();true}}
/// 🧱️ Stores only admitted fixed-shape segments; completed results contain no source references.
pub struct PaintedPathHitJob{
 query:PaintedPathQuery,style:Option<StrokeGeometryStyle>,segments:Vec<semio_framework_2d::PathSegment>,pub flat:Vec<FlatContour>,contours:Vec<StrokeContour>,pub polygons:Vec<Vec<Point>>,
 pub flatten:Option<PathFlattenJob>,flatten_retirement:Option<PathFlattenRetirement>,pub outline:Option<StrokeOutlineJob>,outline_retirement:Option<StrokeOutlineRetirement>,
 pub phase:&'static str,next:usize,at:usize,edge:usize,work:u64,demand:RetirementDemand,last_turn_work:u64,last_turn_semantic_work:u64,nested:bool,winding:i64,boundary:bool,stroke_hit:bool,bounds:[f64;4],output:Option<PaintedPathHit>,cancelled:bool,failure:Option<String>,cleanup:u8,
}
impl PaintedPathHitJob{
 /// 🎬️ Uses the resolved leaf's actual geometry transform and renderer stroke semantics.
 pub fn from_prepared(node:&semio_s_artifact_draw_drawing_scene::preparation::DocumentSceneNode,point:Point,tolerance:f64,flatness:f64)->Result<Self,String>{
  let semio_s_artifact_draw_drawing_scene::preparation::DocumentSceneContent::Path{fill,fill_rule,stroke,..}=&node.content else{return Err("Painted query requires a resolved path leaf".into());};
  Self::new(PaintedPathQuery{point,transform:node.transform,tolerance,flatness,fill:fill.is_some(),fill_rule:fill_rule.clone(),stroke:stroke.as_ref().map(|s|PaintedPathStroke{width:s.width,cap:s.cap.clone(),join:s.join.clone(),dash:s.dash.as_ref().map(|dash|dash.iter().copied().collect()).unwrap_or_default()})})
 }
 pub fn new(query:PaintedPathQuery)->Result<Self,String>{
  if !query.point.into_iter().chain(query.transform).all(valid)||!valid(query.tolerance)||query.tolerance<0.0||!query.flatness.is_finite()||!(1e-6..=16.0).contains(&query.flatness){return Err("Invalid painted query contract".into());}
  let style=query.stroke.as_ref().map(|s|{
   if !valid(s.width)||s.width<0.0||s.dash.len()>1024||!s.dash.iter().all(|v|valid(*v)&&*v>=0.0){return Err("Invalid painted query stroke".to_string());}
   Ok(StrokeGeometryStyle{width:s.width,cap:match s.cap{StrokeCap::Butt=>StrokeGeometryCap::Butt,StrokeCap::Round=>StrokeGeometryCap::Round,StrokeCap::Square=>StrokeGeometryCap::Square},join:match s.join{StrokeJoin::Miter=>StrokeGeometryJoin::Miter,StrokeJoin::Round=>StrokeGeometryJoin::Round,StrokeJoin::Bevel=>StrokeGeometryJoin::Bevel},miter_limit:4.0,dash:s.dash.clone(),dash_offset:0.0})
  }).transpose()?;
  Ok(Self{query,style,segments:Vec::new(),flat:Vec::new(),contours:Vec::new(),polygons:Vec::new(),flatten:None,flatten_retirement:None,outline:None,outline_retirement:None,phase:"admitting",next:0,at:0,edge:0,work:0,demand:RetirementDemand{depth:1,..Default::default()},last_turn_work:0,last_turn_semantic_work:0,nested:false,winding:0,boundary:false,stroke_hit:false,bounds:[f64::INFINITY,f64::INFINITY,f64::NEG_INFINITY,f64::NEG_INFINITY],output:None,cancelled:false,failure:None,cleanup:0})
 }
 fn line(&mut self,a:Point,b:Point)->Result<(),String>{
  if !a.into_iter().chain(b).all(valid){return Err("Painted query exceeds coordinate limit".into());}
  for p in [a,b]{self.bounds[0]=self.bounds[0].min(p[0]);self.bounds[1]=self.bounds[1].min(p[1]);self.bounds[2]=self.bounds[2].max(p[0]);self.bounds[3]=self.bounds[3].max(p[1]);}
  let rounding=a.into_iter().chain(b).chain(self.query.point).fold(1.0_f64,|s,v|s.max(v.abs()))*f64::EPSILON*8.0;
  self.boundary|=semio_s_artifact_draw_drawing_geometry::picking::distance(self.query.point,a,b)<=self.query.tolerance+rounding;
  let[x,y]=self.query.point;if (a[1]<=y&&b[1]>y)||(b[1]<=y&&a[1]>y){let t=(y-a[1])/(b[1]-a[1]);if a[0]*(1.0-t)+b[0]*t>x{self.winding+=if b[1]>a[1]{1}else{-1};}}
  Ok(())
 }
 fn retire_one(&mut self,ledger:&mut RetainedWorkLedger)->Result<bool,String>{
  if self.cleanup==8{return Ok(true);}
  let complete=match self.cleanup{
   0=>{if let Some(child)=&mut self.flatten_retirement{if !child.terminal_is_empty(){self.close_child(0,ledger)?;return Ok(false);}self.flatten_retirement=None;}true},
   1=>{if let Some(child)=&mut self.outline_retirement{if !child.terminal_is_empty(){self.close_child(1,ledger)?;return Ok(false);}self.outline_retirement=None;}true},
   2=>retire_flat(&mut self.flat),3=>{if self.contours.pop().is_some(){false}else{self.contours=Vec::new();true}},4=>retire_polygons(&mut self.polygons),5=>{self.segments=Vec::new();true},
   6=>{self.style=None;self.query.stroke=None;true},7=>{self.failure=None;true},_=>unreachable!(),
  };if complete{self.cleanup+=1;}Ok(self.cleanup==8)
 }
 fn close_child(&mut self,which:u8,ledger:&mut RetainedWorkLedger)->Result<(),String>{
  let turn=match which{0=>close_one(&mut self.flatten_retirement.as_mut().ok_or("Painted query lost its working state")?.owner,ledger),_=>close_one(&mut self.outline_retirement.as_mut().ok_or("Painted query lost its working state")?.owner,ledger)};
  self.nested=true;self.last_turn_work+=turn.cpu;if let Some(error)=turn.error{return Err(error.message.to_string());}Ok(())
 }
 fn step(&mut self,source:&mut impl FnMut(usize)->Option<PathSegment>,ledger:&mut RetainedWorkLedger)->Result<bool,String>{
  match self.phase{
   "admitting"=>{if let Some(segment)=source(self.next){if self.next>=65536{return Err("Painted query exceeds segment limit".into());}if self.segments.capacity()<=self.segments.len(){let wanted=(self.segments.len()+1).max(self.segments.capacity().saturating_mul(2)).min(65536);if !ledger.reserve_vec(&mut self.segments,wanted).map_err(|e|e.to_string())?{return Ok(false);}}if !ledger.copy_bytes(std::mem::size_of::<semio_framework_2d::PathSegment>()){return Ok(false);}self.segments.push(semio_s_artifact_draw_drawing_geometry::to_kernel_segment(&segment));self.next+=1;}else{self.flatten=Some(PathFlattenJob::new(PathFlattenInput{segments:std::mem::take(&mut self.segments),transform:self.query.transform,tolerance:self.query.flatness}));self.phase="flattening";}},
   "flattening"=>{let child=self.flatten.as_mut().ok_or("Painted query lost its working state")?;if child.result().is_ok(){let(retired,output)=self.flatten.take().ok_or("Painted query lost its working state")?.into_retirement();self.flatten_retirement=Some(retired);self.flat=output.ok_or("Painted query lost its flat contours")?;self.phase="flattenCleanup";}else{self.nested=true;let result=child.advance(1,ledger);self.last_turn_work+=child.last_turn_work();self.last_turn_semantic_work+=child.last_turn_semantic_work();result.map_err(|e|e.to_string())?;}return Ok(false);},
   "flattenCleanup"=>{if let Some(owner)=&mut self.flatten_retirement{if !owner.terminal_is_empty(){self.close_child(0,ledger)?;return Ok(false);}self.flatten_retirement=None;}self.phase="fill";return Ok(false);},
   "fill"=>{if let Some(contour)=self.flat.get(self.at){if !self.query.fill||contour.points.len()<2{self.at+=1;self.edge=0;}else if self.edge<contour.points.len(){let a=map(contour.points[self.edge],self.query.transform);let b=map(contour.points[(self.edge+1)%contour.points.len()],self.query.transform);self.edge+=1;self.line(a,b)?;}else{self.at+=1;self.edge=0;}}else{self.phase="contours";}},
   "contours"=>{if let Some(c)=self.flat.pop(){if self.style.is_some(){self.contours.push(StrokeContour{points:c.points,closed:c.closed});}}else{self.flat=Vec::new();if let Some(style)=self.style.take(){self.outline=Some(StrokeOutlineJob::new(StrokeOutlineInput{contours:std::mem::take(&mut self.contours),transform:self.query.transform,tolerance:self.query.flatness,style}));self.phase="stroke";}else{self.publish();self.phase="cleanup";}}},
   "stroke"=>{let child=self.outline.as_mut().ok_or("Painted query lost its working state")?;if child.result().is_ok(){let(retired,output)=self.outline.take().ok_or("Painted query lost its working state")?.into_retirement();self.outline_retirement=Some(retired);self.polygons=output.ok_or("Painted query lost its stroke polygons")?;self.phase="strokeCleanup";}else{self.nested=true;let result=child.advance(1,ledger);self.last_turn_work+=child.last_turn_work();self.last_turn_semantic_work+=child.last_turn_semantic_work();result.map_err(|e|e.to_string())?;}return Ok(false);},
   "strokeCleanup"=>{if let Some(owner)=&mut self.outline_retirement{if !owner.terminal_is_empty(){self.close_child(1,ledger)?;return Ok(false);}self.outline_retirement=None;}self.publish();self.winding=0;self.boundary=false;self.at=0;self.edge=0;self.phase="polygons";return Ok(false);},
   "polygons"=>{if let Some(polygon)=self.polygons.get(self.at){if self.edge<polygon.len(){let a=map(polygon[self.edge],self.query.transform);let b=map(polygon[(self.edge+1)%polygon.len()],self.query.transform);self.edge+=1;self.line(a,b)?;}else{self.stroke_hit|=self.boundary||self.winding!=0;self.winding=0;self.boundary=false;self.at+=1;self.edge=0;}}else{let fill=self.output.unwrap().contains;self.output=Some(PaintedPathHit{contains:fill||self.stroke_hit,bounds:self.bounds[0].is_finite().then_some(self.bounds)});self.phase="cleanup";}},
   "cleanup"=>{if self.retire_one(ledger)?{self.phase="complete";}return Ok(false);},"complete"=>{},_=>return Err("Painted query phase is invalid".into()),
  }Ok(true)
 }
 fn publish(&mut self){self.output=Some(PaintedPathHit{contains:self.query.fill&&(self.boundary||if self.query.fill_rule==FillRule::Evenodd{self.winding%2!=0}else{self.winding!=0}),bounds:self.bounds[0].is_finite().then_some(self.bounds)});}
 pub fn next_work_demand(&self)->RetirementDemand{self.demand}
 pub fn last_turn_work(&self)->u64{self.last_turn_work}
 pub fn last_turn_semantic_work(&self)->u64{self.last_turn_semantic_work}
 pub fn advance(&mut self,grant:usize,ledger:&mut RetainedWorkLedger,mut source:impl FnMut(usize)->Option<PathSegment>)->Result<PaintedPathProgress,String>{
  self.last_turn_work=0;self.last_turn_semantic_work=0;
  if grant==0||grant as u128>9_007_199_254_740_991{return Err("Invalid painted query work grant".into());}if self.cancelled{return Err("Painted query cancelled".into());}if let Some(error)=&self.failure{return Err(error.clone());}
  for _ in 0..grant{
   if self.phase=="complete"{break;}
   if !ledger.admits_absolute(self.demand){ledger.require_absolute(self.demand);break;}
   let Some(root)=ledger.enter()else{self.demand=ledger.next_demand();break;};
   self.nested=false;let before=ledger.receipt();let cpu=self.last_turn_work;let semantic=self.last_turn_semantic_work;
   let result=self.step(&mut source,ledger);
   if !self.nested&&(ledger.receipt()!=before||!ledger.blocked()){self.last_turn_work+=1;self.last_turn_semantic_work+=match &result{Ok(true)|Err(_)=>1,_=>0};}
   if self.last_turn_work>cpu{ledger.note_source(root);}
   self.work+=self.last_turn_semantic_work-semantic;ledger.exit();self.demand=ledger.next_demand();
   if let Err(error)=result{self.failure=Some(error.clone());return Err(error);}
   if ledger.blocked()||self.last_turn_work==cpu{break;}
  }
  Ok(PaintedPathProgress{phase:self.phase,work:self.work,done:self.phase=="complete"})
 }
 pub fn result(&self)->Result<PaintedPathHit,String>{if self.cancelled{return Err("Painted query cancelled".into());}if let Some(error)=&self.failure{return Err(error.clone());}if self.phase!="complete"{return Err("Painted query incomplete".into());}Ok(self.output.unwrap())}
 pub fn cancel(&mut self){self.cancelled=true;}
 pub fn into_retirement(mut self)->(PaintedPathRetirement,Option<PaintedPathHit>){
  let output=self.result().ok();self.cancelled=true;
  if let Some(mut child)=self.flatten.take(){child.cancel();self.flatten_retirement=Some(child.into_retirement().0);}if let Some(mut child)=self.outline.take(){child.cancel();self.outline_retirement=Some(child.into_retirement().0);}
  (PaintedPathRetirement{job:Some(self),work:0,last_turn_work:0},output)
 }
}
/// 🧹️ Actual child owners reach empty retirement before private query state is released.
pub struct PaintedPathRetirement{job:Option<PaintedPathHitJob>,work:u64,last_turn_work:u64}
impl PaintedPathRetirement{
 pub fn terminal_is_empty(&self)->bool{self.job.is_none()}
 pub fn last_turn_work(&self)->u64{self.last_turn_work}
 pub fn advance(&mut self,grant:usize,ledger:&mut RetainedWorkLedger)->Result<WorkRetirementProgress,&'static str>{
  self.last_turn_work=0;if grant==0||grant as u128>9_007_199_254_740_991{return Err("Invalid painted query cleanup grant");}
  for _ in 0..grant{
   let Some(job)=self.job.as_mut()else{break};
   let Some(root)=ledger.enter()else{break};job.nested=false;
   let before=job.last_turn_work;let result=job.retire_one(ledger);if !job.nested{self.last_turn_work+=1;}else{self.last_turn_work+=job.last_turn_work-before;}ledger.note_source(root);ledger.exit();
   match result{Ok(true)=>{self.job=None;}Ok(false)=>{}Err(_)=>return Err("Painted query cleanup refused")}
   self.work+=1;if ledger.blocked(){break;}
  }
  Ok(WorkRetirementProgress{phase:"cleanup",work:self.work,done:self.job.is_none()})
 }
}
