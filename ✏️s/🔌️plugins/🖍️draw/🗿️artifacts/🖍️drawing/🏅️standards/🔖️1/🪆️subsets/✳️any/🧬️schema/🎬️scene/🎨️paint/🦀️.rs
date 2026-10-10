//! 🎨️ Reusable world-space painted contours carry their preparation precision explicitly.
use crate::{PathSegment,FillRule,StrokeCap,StrokeJoin};
use crate::schema::geometry::picking::paint::PaintedPathStroke;
use semio_framework_2d::{flatten::{PathFlattenJob,PathFlattenInput,PathFlattenRetirement,FlatContour},stroke::{StrokeOutlineJob,StrokeOutlineInput,StrokeOutlineRetirement,StrokeGeometryStyle,StrokeGeometryCap,StrokeGeometryJoin,StrokeContour},booleans::{BooleanJob,BooleanInput,BooleanOperand,BooleanOperation,BooleanFillRule,BooleanRetirement}};
type Point=[f64;2];
#[derive(Clone,Debug)]
#[derive(semio_framework_value::RetireOwned)]
pub struct PaintedRegionLimits{pub max_segments:usize,pub max_points:usize,pub max_contours:usize,pub max_work:u64}
#[derive(Clone,Debug)]
#[derive(semio_framework_value::RetireOwned)]
pub struct PaintedPathPreparation{pub transform:[f64;6],pub flatness:f64,pub fill:bool,pub fill_rule:FillRule,pub stroke:Option<PaintedPathStroke>,pub limits:PaintedRegionLimits}
#[derive(Clone,Debug,PartialEq)]
#[derive(semio_framework_value::RetireOwned)]
pub struct PaintedPathRegions{pub flatness:f64,pub fill_rule:FillRule,pub fill:Vec<Vec<Point>>,pub stroke:Vec<Vec<Point>>,pub bounds:Option<[f64;4]>,pub points:usize,pub contours:usize}
#[derive(Clone,Copy,Debug)]
pub struct PaintedPreparationProgress{pub phase:&'static str,pub work:u64,pub done:bool}
#[derive(Clone,Copy,Debug)]
pub struct PreparedQueryProgress{pub phase:&'static str,pub work:u64,pub done:bool}
fn valid(v:f64)->bool{v.is_finite()&&v.abs()<=1e9}
fn precision(v:f64)->bool{v.is_finite()&&(1e-6..=16.0).contains(&v)}
fn map(p:Point,m:[f64;6])->Point{[m[0]*p[0]+m[2]*p[1]+m[4],m[1]*p[0]+m[3]*p[1]+m[5]]}
fn union_job(contours:Vec<Vec<Point>>,rule:BooleanFillRule,epsilon:f64,max_work:u64)->Result<BooleanJob,String>{BooleanJob::new(BooleanInput{operation:BooleanOperation::Union,operands:vec![BooleanOperand{contours,fill_rule:rule}],epsilon,max_edges:262144,max_parameters:1048576,max_atomic_edges:262144,max_segments:327680,max_work}).map_err(|e|e.to_string())}
#[derive(semio_framework_value::RetireOwned)]
struct StrokeRegion{contours:Vec<Vec<Point>>,points:usize}
impl Default for StrokeRegion{fn default()->Self{Self{contours:Vec::new(),points:0}}}
/// 🪵️ Uniform primitive winding and canonical holes admit bounded positive union waves.
#[derive(semio_framework_value::RetireOwned)]
struct StrokeCanonicalJob{
 retired_children:Vec<BooleanRetirement>,retired_operands:Vec<Vec<BooleanOperand>>,retired_segments:Vec<Vec<semio_framework_2d::PathSegment>>,retired_groups:Vec<Vec<Option<StrokeRegion>>>,source:Vec<Vec<Point>>,current:Vec<Option<StrokeRegion>>,next:Vec<Option<StrokeRegion>>,active:Vec<Option<StrokeRegion>>,combined:Vec<Vec<Point>>,combined_group:usize,
 region:StrokeRegion,child:Option<BooleanJob>,child_retirement:Option<BooleanRetirement>,owned_operands:Vec<BooleanOperand>,segments:Vec<semio_framework_2d::PathSegment>,
 phase:&'static str,first:bool,at:usize,emitted:usize,wave_points:usize,wave_contours:usize,work:u64,max_work:u64,epsilon:f64,cancelled:bool,
}
impl StrokeCanonicalJob{
 fn new(source:Vec<Vec<Point>>,epsilon:f64,max_work:u64)->Self{Self{retired_children:Vec::new(),retired_operands:Vec::new(),retired_segments:Vec::new(),retired_groups:Vec::new(),source,current:Vec::new(),next:Vec::new(),active:Vec::new(),combined:Vec::new(),combined_group:0,region:Default::default(),child:None,child_retirement:None,owned_operands:Vec::new(),segments:Vec::new(),phase:"grouping",first:true,at:0,emitted:0,wave_points:0,wave_contours:0,work:0,max_work,epsilon,cancelled:false}}
 fn reserve(&mut self,points:usize,contours:usize)->Result<(),String>{if self.wave_points+points>262144||self.wave_contours+contours>65536{return Err("Painted stroke union exceeds frontier limit".into());}self.wave_points+=points;self.wave_contours+=contours;Ok(())}
 fn step(&mut self)->Result<(),String>{match self.phase{
  "grouping"=>{if let Some(c)=self.source.pop(){let points=c.len();self.current.push(Some(StrokeRegion{contours:vec![c],points}));}else{self.phase="merging";}},
  "merging"=>{if self.at==self.current.len(){self.phase="waveCleanup";return Ok(());}let count=(if self.first{4}else{16}).min(self.current.len()-self.at);for index in 0..count{self.active.push(self.current[self.at+index].take());}if count==1&&!self.first{let group=self.active.last().unwrap().as_ref().unwrap();let(points,contours)=(group.points,group.contours.len());self.reserve(points,contours)?;self.next.push(self.active.pop().unwrap());self.at+=1;}else{self.combined_group=0;self.phase="operands";}},
  "waveCleanup"=>{if self.current.pop().is_none(){self.retired_groups.push(std::mem::replace(&mut self.current,std::mem::take(&mut self.next)));self.at=0;self.wave_points=0;self.wave_contours=0;self.first=false;if self.current.len()<=1{self.region=self.current.pop().flatten().unwrap_or_default();self.phase="complete";}else{self.phase="merging";}}},
  "operands"=>{if let Some(group)=self.active.get_mut(self.combined_group){if let Some(c)=group.as_mut().unwrap().contours.pop(){self.combined.push(c);}else{self.combined_group+=1;}}else{self.child=Some(union_job(std::mem::take(&mut self.combined),BooleanFillRule::Nonzero,self.epsilon,self.max_work)?);self.phase="canonicalizing";}},
  "canonicalizing"=>{if self.child.as_mut().unwrap().advance_work(1).map_err(|e|e.to_string())?{let(retirement,operands,output)=self.child.take().unwrap().into_retirement();if let Some(previous)=self.child_retirement.replace(retirement){self.retired_children.push(previous);}self.owned_operands=operands;self.segments=output.unwrap();self.phase="canonicalCleanup";}},
  "canonicalCleanup"=>{self.emitted=0;self.phase="canonicalContours";},
  "canonicalContours"=>{if let Some(s)=self.segments.get(self.emitted).cloned(){self.emitted+=1;if let semio_framework_2d::PathSegment::Move{..}=s{self.reserve(0,1)?;self.region.contours.push(Vec::new());}if let semio_framework_2d::PathSegment::Move{to}|semio_framework_2d::PathSegment::Line{to}=s{self.reserve(1,0)?;self.region.contours.last_mut().unwrap().push(to);self.region.points+=1;}}else{self.phase="canonicalSourceCleanup";}},
  "canonicalSourceCleanup"=>{self.retired_operands.push(std::mem::take(&mut self.owned_operands));self.retired_segments.push(std::mem::take(&mut self.segments));self.retired_groups.push(std::mem::take(&mut self.active));self.next.push(Some(std::mem::take(&mut self.region)));self.at+=(if self.first{4}else{16}).min(self.current.len()-self.at);self.phase="merging";},
  "complete"=>{},_=>unreachable!(),
 }Ok(())}
 fn advance(&mut self)->Result<bool,String>{if self.cancelled{return Err("Painted stroke union cancelled".into());}if self.work>=self.max_work{return Err("Painted stroke union exceeds work limit".into());}if self.phase!="complete"{self.step()?;self.work+=1;}Ok(self.phase=="complete")}
 fn into_retirement(mut self)->(StrokeCanonicalRetirement,Option<Vec<Vec<Point>>>){let output=if !self.cancelled&&self.phase=="complete"{Some(std::mem::take(&mut self.region.contours))}else{None};if let Some(child)=self.child.take(){let(retirement,operands,output)=child.into_retirement();if let Some(previous)=self.child_retirement.replace(retirement){self.retired_children.push(previous);}self.owned_operands=operands;self.segments=output.unwrap_or_default();}self.cancelled=true;(StrokeCanonicalRetirement::new(self),output)}
}
semio_framework_2d::physical_work_retirement!(StrokeCanonicalRetirement,StrokeCanonicalJob,String,|message:&str|message.to_string());
semio_framework_2d::physical_work_retirement!(PaintedRegionsCloseJob,PaintedPathRegions,&'static str,|_:&str|"Painted region physical close refused");
/// 🧱️ Preparation admits borrowed source segments and retains completed children until explicit retirement.
#[derive(semio_framework_value::RetireOwned)]
pub struct PaintedPathPrepareJob{
 input:PaintedPathPreparation,style:Option<StrokeGeometryStyle>,segments:Vec<semio_framework_2d::PathSegment>,flat:Vec<FlatContour>,contours:Vec<StrokeContour>,polygons:Vec<Vec<Point>>,
 flatten:Option<PathFlattenJob>,flatten_retirement:Option<PathFlattenRetirement>,outline:Option<StrokeOutlineJob>,outline_retirement:Option<StrokeOutlineRetirement>,
 canonical:Option<BooleanJob>,canonical_retirement:Option<BooleanRetirement>,canonical_operands:Vec<BooleanOperand>,canonical_segments:Vec<semio_framework_2d::PathSegment>,stroke_canonical:Option<StrokeCanonicalJob>,stroke_canonical_retirement:Option<StrokeCanonicalRetirement>,raw_points:usize,raw_contours:usize,magnitude:f64,
 phase:&'static str,work:u64,next:usize,at:usize,point:usize,regions:Option<PaintedPathRegions>,failure:Option<String>,cancelled:bool,
}
impl PaintedPathPrepareJob{
 /// 🎬️ Borrows the resolved leaf separately while copying bounded preparation attributes.
 pub fn from_prepared(node:&crate::schema::scene_preparation::DocumentSceneNode,flatness:f64,limits:PaintedRegionLimits)->Result<Self,String>{let (crate::schema::scene_preparation::DocumentSceneContent::Path{fill,fill_rule,stroke,..}|crate::schema::scene_preparation::DocumentSceneContent::Glyphs{fill,fill_rule,stroke,..})=&node.content else{return Err("Painted preparation requires a resolved path leaf".into());};Self::new(PaintedPathPreparation{transform:node.transform,flatness,fill:fill.is_some(),fill_rule:fill_rule.clone(),stroke:stroke.as_ref().map(|s|PaintedPathStroke{width:s.width,cap:s.cap.clone(),join:s.join.clone(),dash:s.dash.as_ref().map(|dash|dash.iter().copied().collect()).unwrap_or_default()}),limits})}
 pub fn new(input:PaintedPathPreparation)->Result<Self,String>{let l=&input.limits;if !input.transform.into_iter().all(valid)||!precision(input.flatness)||l.max_segments==0||l.max_segments>65536||l.max_points==0||l.max_points>262144||l.max_contours==0||l.max_contours>65536||l.max_work==0||l.max_work>1000000000{return Err("Invalid painted preparation contract".into());}
  let style=input.stroke.as_ref().map(|s|{if !valid(s.width)||s.width<0.0||s.dash.len()>1024||!s.dash.iter().all(|v|valid(*v)&&*v>=0.0){return Err("Invalid painted preparation stroke".to_string());}Ok(StrokeGeometryStyle{width:s.width,cap:match s.cap{StrokeCap::Butt=>StrokeGeometryCap::Butt,StrokeCap::Round=>StrokeGeometryCap::Round,StrokeCap::Square=>StrokeGeometryCap::Square},join:match s.join{StrokeJoin::Miter=>StrokeGeometryJoin::Miter,StrokeJoin::Round=>StrokeGeometryJoin::Round,StrokeJoin::Bevel=>StrokeGeometryJoin::Bevel},miter_limit:4.0,dash:s.dash.clone(),dash_offset:0.0})}).transpose()?;
  let regions=PaintedPathRegions{flatness:input.flatness,fill_rule:input.fill_rule.clone(),fill:Vec::new(),stroke:Vec::new(),bounds:None,points:0,contours:0};Ok(Self{input,style,segments:Vec::new(),flat:Vec::new(),contours:Vec::new(),polygons:Vec::new(),flatten:None,flatten_retirement:None,outline:None,outline_retirement:None,canonical:None,canonical_retirement:None,canonical_operands:Vec::new(),canonical_segments:Vec::new(),stroke_canonical:None,stroke_canonical_retirement:None,raw_points:0,raw_contours:0,magnitude:1.0,phase:"admitting",work:0,next:0,at:0,point:0,regions:Some(regions),failure:None,cancelled:false})
 }
 fn begin(&mut self,stroke:bool)->Result<(),String>{if self.raw_contours>=65536{return Err("Painted preparation exceeds raw contour limit".into());}let r=self.regions.as_mut().unwrap();if stroke{r.stroke.push(Vec::new());}else{r.fill.push(Vec::new());}self.raw_contours+=1;Ok(())}
 fn append(&mut self,stroke:bool,local:Point)->Result<(),String>{let p=map(local,self.input.transform);if !p.into_iter().all(valid){return Err("Painted preparation exceeds coordinate limit".into());}if self.raw_points>=262144{return Err("Painted preparation exceeds raw point limit".into());}let r=self.regions.as_mut().unwrap();if stroke{r.stroke.last_mut().unwrap().push(p);}else{r.fill.last_mut().unwrap().push(p);}self.raw_points+=1;self.magnitude=self.magnitude.max(p[0].abs()).max(p[1].abs());Ok(())}
 fn topology_epsilon(&self)->Result<f64,String>{let epsilon=1e-12_f64.max(self.magnitude*f64::EPSILON*32.0);if epsilon>self.input.flatness/16.0{return Err("Painted topology exceeds coordinate precision".into());}Ok(epsilon)}
 fn account_point(&mut self,p:Point)->Result<(),String>{let r=self.regions.as_mut().unwrap();if r.points>=self.input.limits.max_points{return Err("Painted preparation exceeds point limit".into());}r.points+=1;if let Some(b)=&mut r.bounds{b[0]=b[0].min(p[0]);b[1]=b[1].min(p[1]);b[2]=b[2].max(p[0]);b[3]=b[3].max(p[1]);}else{r.bounds=Some([p[0],p[1],p[0],p[1]]);}Ok(())}
 fn account_contour(&mut self)->Result<(),String>{let r=self.regions.as_mut().unwrap();if r.contours>=self.input.limits.max_contours{return Err("Painted preparation exceeds contour limit".into());}r.contours+=1;Ok(())}
 fn begin_canonical(&mut self)->Result<(),String>{let epsilon=self.topology_epsilon()?;let rule=if self.input.fill_rule==FillRule::Evenodd{BooleanFillRule::Evenodd}else{BooleanFillRule::Nonzero};let source=std::mem::take(&mut self.regions.as_mut().unwrap().fill);self.canonical=Some(union_job(source,rule,epsilon,self.input.limits.max_work)?);self.at=0;self.point=0;self.phase="canonicalizing";Ok(())}
 fn step(&mut self,source:&mut impl FnMut(usize)->Option<PathSegment>)->Result<(),String>{match self.phase{
  "admitting"=>{if let Some(s)=source(self.next){if self.next>=self.input.limits.max_segments{return Err("Painted preparation exceeds segment limit".into());}self.segments.push(crate::schema::to_kernel_segment(&s));self.next+=1;}else{self.flatten=Some(PathFlattenJob::new(PathFlattenInput{segments:std::mem::take(&mut self.segments),transform:self.input.transform,tolerance:self.input.flatness}).map_err(|e|e.to_string())?);self.phase="flattening";}},
  "flattening"=>{if self.flatten.as_mut().unwrap().advance(1).map_err(|e|e.to_string())?.done{let(c,output)=self.flatten.take().unwrap().into_retirement();self.flatten_retirement=Some(c);self.flat=output.unwrap();self.phase="flattenCleanup";}},
  "flattenCleanup"=>{self.phase="fill";},
  "fill"=>{if let Some(c)=self.flat.get(self.at){if !self.input.fill||c.points.len()<2{self.at+=1;self.point=0;}else if self.point==0{self.begin(false)?;self.point+=1;}else if self.point<=c.points.len(){let p=c.points[self.point-1];self.append(false,p)?;self.point+=1;}else{self.at+=1;self.point=0;}}else{self.begin_canonical()?;}},
  "contours"=>{if self.style.is_none(){self.phase="cleanup";}else if let Some(c)=self.flat.get(self.at){if self.point==0{self.contours.push(StrokeContour{points:Vec::new(),closed:c.closed});self.point+=1;}else if self.point<=c.points.len(){self.contours.last_mut().unwrap().points.push(c.points[self.point-1]);self.point+=1;}else{self.at+=1;self.point=0;}}else{self.outline=Some(StrokeOutlineJob::new(StrokeOutlineInput{contours:std::mem::take(&mut self.contours),transform:self.input.transform,tolerance:self.input.flatness,style:self.style.take().unwrap()}).map_err(|e|e.to_string())?);self.phase="stroke";}},
  "stroke"=>{if self.outline.as_mut().unwrap().advance(1).map_err(|e|e.to_string())?.done{let(c,output)=self.outline.take().unwrap().into_retirement();self.outline_retirement=Some(c);self.polygons=output.unwrap();self.phase="strokeCleanup";}},
  "strokeCleanup"=>{self.at=0;self.point=0;self.phase="polygons";},
  "polygons"=>{if let Some(p)=self.polygons.get(self.at){if self.point==0{self.begin(true)?;self.point+=1;}else if self.point<=p.len(){let p=p[self.point-1];self.append(true,p)?;self.point+=1;}else{self.at+=1;self.point=0;}}else{let epsilon=self.topology_epsilon()?;let source=std::mem::take(&mut self.regions.as_mut().unwrap().stroke);self.stroke_canonical=Some(StrokeCanonicalJob::new(source,epsilon,self.input.limits.max_work));self.phase="strokeCanonical";}},
  "strokeCanonical"=>{if self.stroke_canonical.as_mut().unwrap().advance()?{let(retirement,output)=self.stroke_canonical.take().unwrap().into_retirement();self.stroke_canonical_retirement=Some(retirement);self.regions.as_mut().unwrap().stroke=output.unwrap();self.at=0;self.point=0;self.phase="strokeAccount";}},
  "strokeAccount"=>{if let Some(c)=self.regions.as_ref().unwrap().stroke.get(self.at){if self.point==0{self.account_contour()?;self.point+=1;}else if self.point<=c.len(){let p=c[self.point-1];self.account_point(p)?;self.point+=1;}else{self.at+=1;self.point=0;}}else{self.phase="cleanup";}},
  "canonicalizing"=>{if self.canonical.as_mut().unwrap().advance_work(1).map_err(|e|e.to_string())?{let(retirement,operands,output)=self.canonical.take().unwrap().into_retirement();self.canonical_retirement=Some(retirement);self.canonical_operands=operands;self.canonical_segments=output.unwrap();self.phase="canonicalCleanup";}},
  "canonicalCleanup"=>{self.at=0;self.phase="canonicalContours";},
  "canonicalContours"=>{if let Some(s)=self.canonical_segments.get(self.at).cloned(){self.at+=1;if let semio_framework_2d::PathSegment::Move{..}=s{self.account_contour()?;self.regions.as_mut().unwrap().fill.push(Vec::new());}if let semio_framework_2d::PathSegment::Move{to}|semio_framework_2d::PathSegment::Line{to}=s{self.account_point(to)?;self.regions.as_mut().unwrap().fill.last_mut().unwrap().push(to);}}else{self.phase="canonicalSourceCleanup";}},
  "canonicalSourceCleanup"=>{self.raw_points=0;self.raw_contours=0;self.at=0;self.point=0;self.phase="contours";},
  "cleanup"=>{self.phase="complete";},"complete"=>{},_=>unreachable!(),}Ok(())}
 pub fn advance_work(&mut self,grant:usize,mut source:impl FnMut(usize)->Option<PathSegment>)->Result<bool,String>{if grant==0||grant as u128>9007199254740991{return Err("Invalid painted preparation work grant".into());}if self.cancelled{return Err("Painted preparation cancelled".into());}if let Some(error)=&self.failure{return Err(error.clone());}for _ in 0..grant{if self.phase=="complete"{break;}let step=if self.work>=self.input.limits.max_work{Err("Painted preparation exceeds work limit".into())}else{self.step(&mut source)};if let Err(error)=step{self.failure=Some(error.clone());return Err(error);}self.work+=1;}Ok(self.phase=="complete")}
 pub fn advance(&mut self,grant:usize,source:impl FnMut(usize)->Option<PathSegment>)->Result<PaintedPreparationProgress,String>{self.advance_work(grant,source)?;Ok(PaintedPreparationProgress{phase:self.phase,work:self.work,done:self.phase=="complete"})}
 pub fn result(&self)->Result<&PaintedPathRegions,String>{if self.cancelled{return Err("Painted preparation cancelled".into());}if let Some(error)=&self.failure{return Err(error.clone());}if self.phase!="complete"{return Err("Painted preparation incomplete".into());}Ok(self.regions.as_ref().unwrap())}
 pub fn cancel(&mut self){self.cancelled=true;}
 pub fn into_retirement(mut self)->(PaintedPreparationRetirement,Option<PaintedPathRegions>){let success=self.result().is_ok();let output=if success{self.regions.take()}else{None};self.cancelled=true;if let Some(mut c)=self.flatten.take(){c.cancel();self.flatten_retirement=Some(c.into_retirement().0);}if let Some(mut c)=self.outline.take(){c.cancel();self.outline_retirement=Some(c.into_retirement().0);}if let Some(c)=self.canonical.take(){let(retirement,operands,output)=c.into_retirement();self.canonical_retirement=Some(retirement);self.canonical_operands=operands;self.canonical_segments=output.unwrap_or_default();}if let Some(mut c)=self.stroke_canonical.take(){c.cancelled=true;self.stroke_canonical_retirement=Some(c.into_retirement().0);}(PaintedPreparationRetirement::new(self),output)}
}
semio_framework_2d::physical_work_retirement!(PaintedPreparationRetirement,PaintedPathPrepareJob,&'static str,|_:&str|"Painted preparation physical close refused");
fn distance(p:Point,a:Point,b:Point)->f64{let dx=b[0]-a[0];let dy=b[1]-a[1];let length=dx.hypot(dy);let t=if length>0.0{(((p[0]-a[0])*dx+(p[1]-a[1])*dy)/length).clamp(0.0,length)}else{0.0};(p[0]-a[0]-if length>0.0{dx*t/length}else{0.0}).hypot(p[1]-a[1]-if length>0.0{dy*t/length}else{0.0})}
/// 🪪️ A fixed stamp identifies the exact immutable cache entry and its preparation precision.
#[derive(Clone,Copy,Debug,PartialEq)]
pub struct PreparedRegionStamp{pub source:crate::schema::scene_identity::SceneIdentity,pub build:u64,pub entry:usize,pub flatness:f64}
impl PreparedRegionStamp{
 fn valid(self)->bool{self.source.matches(self.source)&&self.build!=0&&self.entry<1024&&precision(self.flatness)}
 fn matches(self,live:Self)->bool{self.valid()&&live.valid()&&self==live}
}
/// 🎯️ Only cursor metadata survives a turn; completed geometry is borrowed separately per grant.
pub struct PreparedRegionQueryJob{stamp:PreparedRegionStamp,point:Point,tolerance:f64,phase:&'static str,at:usize,edge:usize,work:u64,winding:i64,boundary:bool,contains:bool,cancelled:bool,failure:Option<&'static str>}
impl PreparedRegionQueryJob{
 pub fn new(stamp:PreparedRegionStamp,point:Point,tolerance:f64,required_flatness:f64)->Result<Self,String>{if !stamp.valid()||!point.into_iter().all(valid)||!valid(tolerance)||tolerance<0.0||!precision(required_flatness){return Err("Invalid prepared region query contract".into());}if stamp.flatness>required_flatness{return Err("Prepared region precision is insufficient".into());}Ok(Self{stamp,point,tolerance,phase:"fill",at:0,edge:0,work:0,winding:0,boundary:false,contains:false,cancelled:false,failure:None})}
 fn authority(&self,live:PreparedRegionStamp)->Result<(),String>{if self.cancelled{return Err("Prepared region query cancelled".into());}if let Some(error)=self.failure{return Err(error.into());}if !self.stamp.matches(live){return Err("Prepared region query authority changed".into());}Ok(())}
 fn line(&mut self,a:Point,b:Point){let p=self.point;let rounding=a.into_iter().chain(b).chain(p).fold(1.0_f64,|s,v|s.max(v.abs()))*f64::EPSILON*8.0;self.boundary|=distance(p,a,b)<=self.tolerance+rounding;if (a[1]<=p[1]&&b[1]>p[1])||(b[1]<=p[1]&&a[1]>p[1]){let t=(p[1]-a[1])/(b[1]-a[1]);if a[0]*(1.0-t)+b[0]*t>p[0]{self.winding+=if b[1]>a[1]{1}else{-1};}}}
 fn step(&mut self,regions:&PaintedPathRegions){let contours=if self.phase=="fill"{&regions.fill}else{&regions.stroke};if let Some(c)=contours.get(self.at){if self.edge<c.len(){self.line(c[self.edge],c[(self.edge+1)%c.len()]);self.edge+=1;}else{self.at+=1;self.edge=0;}}else if self.phase=="fill"{self.contains=self.boundary||if regions.fill_rule==FillRule::Evenodd{self.winding%2!=0}else{self.winding!=0};self.winding=0;self.boundary=false;self.at=0;self.edge=0;self.phase="stroke";}else{self.contains|=self.boundary||self.winding!=0;self.phase="complete";}}
 pub fn advance(&mut self,regions:&PaintedPathRegions,live:PreparedRegionStamp,grant:usize)->Result<PreparedQueryProgress,String>{if grant==0||grant as u128>9007199254740991{return Err("Invalid prepared query work grant".into());}self.authority(live).map_err(|error|{if !self.cancelled&&self.failure.is_none(){self.failure=Some("Prepared region query authority changed");}error})?;if regions.flatness!=self.stamp.flatness{self.failure=Some("Prepared region cache precision changed");return Err(self.failure.unwrap().into());}for _ in 0..grant{if self.phase=="complete"{break;}self.step(regions);self.work+=1;}Ok(PreparedQueryProgress{phase:self.phase,work:self.work,done:self.phase=="complete"})}
 pub fn result(&self,live:PreparedRegionStamp)->Result<bool,String>{self.authority(live)?;if self.phase!="complete"{return Err("Prepared region query incomplete".into());}Ok(self.contains)}
 pub fn cancel(&mut self){self.cancelled=true;}
}
#[cfg(test)]
#[path="🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[path="📋️prepare/🦀️.rs"]
pub mod scene;
