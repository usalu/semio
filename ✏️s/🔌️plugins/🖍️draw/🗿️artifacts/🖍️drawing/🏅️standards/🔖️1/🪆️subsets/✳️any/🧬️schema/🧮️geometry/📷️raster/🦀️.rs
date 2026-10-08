//! 📷️ Resumable authored path coverage and local paint into straight RGBA.
use crate::{FillStyle,StrokeStyle,PathSegment,FillRule,StrokeCap,StrokeJoin};
use crate::schema::fill::sampling::{PreparedFill,PreparedFillRetirement};
use semio_framework_2d::retirement::{WorkRetirementCounter,WorkRetirementProgress};
use semio_framework_2d::{flatten::{PathFlattenJob,PathFlattenInput,PathFlattenRetirement,FlatContour},stroke::{StrokeOutlineJob,StrokeOutlineInput,StrokeOutlineRetirement,StrokeGeometryStyle,StrokeGeometryCap,StrokeGeometryJoin,StrokeContour}};
use semio_framework_pixels::{RasterImage,coverage::{CoverageJob,CoverageInput,CoverageMask,CoverageRule,CoverageRetirement},editing::validate_extent};
#[derive(Clone,Debug)]
pub struct PathRasterInput {pub width:u32,pub height:u32,pub origin:[f64;2],pub segments:Vec<PathSegment>,pub transform:[f64;6],pub tolerance:f64,pub fill_rule:FillRule,pub fill:Option<FillStyle>,pub stroke:Option<StrokeStyle>}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum PathRasterPhase {Preparing,Flattening,FlattenCleanup,Contours,ContoursCleanup,Stroke,StrokeCleanup,FillCoverage,FillCoverageCleanup,StrokeCoverage,StrokeCoverageCleanup,Painting,Complete}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct PathRasterProgress {pub phase:PathRasterPhase,pub completed:usize,pub total:usize,pub work:u64,pub done:bool}
#[derive(Clone,Debug,PartialEq,Eq)]
pub enum PathRasterError {Invalid(String),Incomplete,Cancelled}
impl std::fmt::Display for PathRasterError {
 fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result {match self {Self::Invalid(message)=>f.write_str(message),Self::Incomplete=>f.write_str("Path raster is incomplete"),Self::Cancelled=>f.write_str("Path raster cancelled")}}
}
impl std::error::Error for PathRasterError {}
fn invalid(error:impl std::fmt::Display)->PathRasterError {PathRasterError::Invalid(error.to_string())}
fn coordinate(v:f64)->bool {v.is_finite()&&v.abs()<=1e9}
fn style(stroke:&StrokeStyle)->Result<StrokeGeometryStyle,PathRasterError> {
 PreparedFill::new(&FillStyle::Solid {color:stroke.color}).map_err(invalid)?;
 if !coordinate(stroke.width)||stroke.width<0.0||stroke.dash.as_ref().is_some_and(|d|d.len()>1024||d.iter().any(|v|!coordinate(*v)||*v<0.0)) {return Err(invalid("Invalid raster stroke"));}
 Ok(StrokeGeometryStyle {width:stroke.width,cap:match stroke.cap {StrokeCap::Butt=>StrokeGeometryCap::Butt,StrokeCap::Round=>StrokeGeometryCap::Round,StrokeCap::Square=>StrokeGeometryCap::Square},join:match stroke.join {StrokeJoin::Miter=>StrokeGeometryJoin::Miter,StrokeJoin::Round=>StrokeGeometryJoin::Round,StrokeJoin::Bevel=>StrokeGeometryJoin::Bevel},miter_limit:4.0,dash:stroke.dash.as_ref().map(|dash|dash.iter().copied().collect()).unwrap_or_default(),dash_offset:0.0})
}
/// 🧱️ Private raster candidate with per-segment/point conversion and bounded geometry/paint grants.
pub struct PathRasterJob {
 width:u32,height:u32,count:usize,transform:[f64;6],tolerance:f64,rule:CoverageRule,inverse:Option<[f64;6]>,
 fill:Option<PreparedFill>,stroke_color:Option<[f64;4]>,style:Option<StrokeGeometryStyle>,source:Vec<PathSegment>,segments:Vec<semio_framework_2d::PathSegment>,flat:Vec<FlatContour>,fill_contours:Vec<Vec<[f64;2]>>,stroke_contours:Vec<StrokeContour>,
 flatten:Option<PathFlattenJob>,outline:Option<StrokeOutlineJob>,coverage:Option<CoverageJob>,fill_mask:Option<CoverageMask>,stroke_mask:Option<CoverageMask>,candidate:Option<RasterImage>,phase:PathRasterPhase,at:usize,point:usize,work:u64,completed:usize,total:usize,cancelled:bool,failed:Option<PathRasterError>,
 outline_retirement:Option<StrokeOutlineRetirement>,stroke_polygons:Vec<Vec<[f64;2]>>,
 coverage_retirement:Option<CoverageRetirement>,
 flatten_retirement:Option<PathFlattenRetirement>,
 fill_retirement:Option<PreparedFillRetirement>,
}
impl PathRasterJob {
 pub fn new(input:PathRasterInput)->Result<Self,PathRasterError> {
  let count=validate_extent(input.width,input.height).map_err(invalid)?;
  if !input.origin.into_iter().all(coordinate)||!input.transform.into_iter().all(coordinate)||input.segments.len()>65536||!input.tolerance.is_finite()||!(1e-6..=16.0).contains(&input.tolerance) {return Err(invalid("Invalid painted path contract"));}
  let mut transform=input.transform;transform[4]-=input.origin[0];transform[5]-=input.origin[1];if !transform.into_iter().all(coordinate) {return Err(invalid("Path raster origin exceeds coordinate budget"));}
  if let Some(fill)=&input.fill {let valid=match fill {FillStyle::Solid {..}=>true,FillStyle::LinearGradient {x1,y1,x2,y2,..}=>[*x1,*y1,*x2,*y2].into_iter().all(coordinate),FillStyle::RadialGradient {cx,cy,r,..}=>[*cx,*cy,*r].into_iter().all(coordinate)};if !valid {return Err(invalid("Paint coordinates exceed raster budget"));}}
  let fill=input.fill.as_ref().map(PreparedFill::new).transpose().map_err(invalid)?;let style=input.stroke.as_ref().map(style).transpose()?;let stroke_color=input.stroke.as_ref().map(|s|s.color);
  let [a,b,c,d,e,f]=transform;let det=a*d-b*c;if !det.is_finite() {return Err(invalid("Path raster transform exceeds numeric limits"));}
  let inverse=(det!=0.0).then(||[d/det,-b/det,-c/det,a/det,(c*f-d*e)/det,(b*e-a*f)/det]);if inverse.is_some_and(|m|!m.into_iter().all(f64::is_finite)) {return Err(invalid("Path raster inverse exceeds numeric limits"));}
  let total=input.segments.len();
  Ok(Self {width:input.width,height:input.height,count,transform,tolerance:input.tolerance,rule:match input.fill_rule {FillRule::Nonzero=>CoverageRule::NonZero,FillRule::Evenodd=>CoverageRule::EvenOdd},inverse,fill,stroke_color,style,source:input.segments,segments:Vec::new(),flat:Vec::new(),fill_contours:Vec::new(),stroke_contours:Vec::new(),flatten:None,flatten_retirement:None,outline:None,outline_retirement:None,stroke_polygons:Vec::new(),coverage:None,coverage_retirement:None,fill_retirement:None,fill_mask:None,stroke_mask:None,candidate:Some(RasterImage {width:input.width,height:input.height,pixels:vec![0;count*4]}),phase:PathRasterPhase::Preparing,at:0,point:0,work:0,completed:0,total,cancelled:false,failed:None})
 }
 fn enter(&mut self,phase:PathRasterPhase,total:usize) {self.phase=phase;self.completed=0;self.total=total;}
 fn start_fill(&mut self)->Result<(),PathRasterError> {
  if self.fill.is_some()&&self.inverse.is_some() {self.coverage=Some(CoverageJob::new(CoverageInput {width:self.width,height:self.height,transform:self.transform,rule:self.rule,contours:std::mem::take(&mut self.fill_contours)}).map_err(invalid)?);self.enter(PathRasterPhase::FillCoverage,0);}
  else {self.start_stroke()?;}Ok(())
 }
 fn start_stroke(&mut self)->Result<(),PathRasterError> {
  if self.style.is_some()&&self.inverse.is_some() {let contours=std::mem::take(&mut self.stroke_polygons);self.coverage=Some(CoverageJob::new(CoverageInput {width:self.width,height:self.height,transform:self.transform,rule:CoverageRule::NonZero,contours}).map_err(invalid)?);self.enter(PathRasterPhase::StrokeCoverage,0);}
  else {self.at=0;self.enter(PathRasterPhase::Painting,self.count);}Ok(())
 }
 fn step(&mut self)->Result<(),PathRasterError> {
  match self.phase {
   PathRasterPhase::Preparing=>{
    if let Some(segment)=self.source.get(self.at) {self.segments.push(super::super::to_kernel_segment(segment));self.at+=1;self.completed=self.at;}
    else {self.flatten=Some(PathFlattenJob::new(PathFlattenInput {segments:std::mem::take(&mut self.segments),transform:self.transform,tolerance:self.tolerance}).map_err(invalid)?);self.source=Vec::new();self.at=0;self.enter(PathRasterPhase::Flattening,0);}
   }
   PathRasterPhase::Flattening=>{
    let p=self.flatten.as_mut().unwrap().advance(1).map_err(invalid)?;self.completed=p.completed;self.total=p.total;
    if p.done {let(retired,output)=self.flatten.take().unwrap().into_retirement();self.flat=output.expect("complete flatten output");self.flatten_retirement=Some(retired);self.enter(PathRasterPhase::FlattenCleanup,p.points+self.flat.len());}
   }
   PathRasterPhase::FlattenCleanup=>{
    if !self.flatten_retirement.as_ref().unwrap().terminal_is_empty(){self.flatten_retirement.as_mut().unwrap().advance(1).map_err(invalid)?;}
    else {self.flatten_retirement=None;self.enter(PathRasterPhase::Contours,self.total);}
   }
   PathRasterPhase::Contours=>{
    if let Some(c)=self.flat.get(self.at) {
     if self.point==0 {if self.fill.is_some() {self.fill_contours.push(Vec::new());}if self.style.is_some() {self.stroke_contours.push(StrokeContour {points:Vec::new(),closed:c.closed});}}
     if let Some(p)=c.points.get(self.point) {if self.fill.is_some() {self.fill_contours[self.at].push(*p);}if self.style.is_some() {self.stroke_contours[self.at].points.push(*p);}self.point+=1;}
     else {self.at+=1;self.point=0;}self.completed+=1;
    } else {self.enter(PathRasterPhase::ContoursCleanup,0);}
   }
   PathRasterPhase::ContoursCleanup=>{
    if retire_contour(&mut self.flat){if let Some(style)=self.style.clone(){self.outline=Some(StrokeOutlineJob::new(StrokeOutlineInput{contours:std::mem::take(&mut self.stroke_contours),transform:self.transform,tolerance:self.tolerance,style}).map_err(invalid)?);self.enter(PathRasterPhase::Stroke,0);}else{self.start_fill()?;}}
   }
   PathRasterPhase::Stroke=>{
    let p=self.outline.as_mut().unwrap().advance(1).map_err(invalid)?;self.completed=p.completed;self.total=p.total;
    if p.done {let(retired,output)=self.outline.take().unwrap().into_retirement();self.stroke_polygons=output.expect("complete stroke output");self.outline_retirement=Some(retired);self.enter(PathRasterPhase::StrokeCleanup,0);}
   }
   PathRasterPhase::StrokeCleanup=>{
    if self.outline_retirement.as_ref().unwrap().terminal_is_empty(){self.outline_retirement=None;self.start_fill()?;}
    else {self.outline_retirement.as_mut().unwrap().advance(1).map_err(invalid)?;}
   }
   PathRasterPhase::FillCoverage|PathRasterPhase::StrokeCoverage=>{
    let p=self.coverage.as_mut().unwrap().advance(1).map_err(invalid)?;self.completed=p.completed;self.total=p.total;
    if p.done {let fill=self.phase==PathRasterPhase::FillCoverage;let(retired,output)=self.coverage.take().unwrap().into_retirement();self.coverage_retirement=Some(retired);let mask=output.expect("complete coverage output");if fill {self.fill_mask=Some(mask);} else {self.stroke_mask=Some(mask);}self.enter(if fill {PathRasterPhase::FillCoverageCleanup} else {PathRasterPhase::StrokeCoverageCleanup},0);}
   }
   PathRasterPhase::FillCoverageCleanup|PathRasterPhase::StrokeCoverageCleanup=>{
    if !self.coverage_retirement.as_ref().unwrap().terminal_is_empty(){self.coverage_retirement.as_mut().unwrap().advance(1).map_err(invalid)?;}
    else {self.coverage_retirement=None;if self.phase==PathRasterPhase::FillCoverageCleanup {self.start_stroke()?;} else {self.at=0;self.enter(PathRasterPhase::Painting,self.count);}}
   }
   PathRasterPhase::Painting=>{
    if self.at==self.count {self.enter(PathRasterPhase::Complete,self.count);self.completed=self.count;return Ok(());}
    let at=self.at;self.at+=1;let fa=self.fill_mask.as_ref().map_or(0,|m|m.coverage[at]);let sa=self.stroke_mask.as_ref().map_or(0,|m|m.coverage[at]);let mut color=[0.0;4];
    if fa>0 {if let (Some(fill),Some(m))=(&self.fill,self.inverse) {let x=(at%self.width as usize) as f64+0.5;let y=(at/self.width as usize) as f64+0.5;color=fill.sample([m[0]*x+m[2]*y+m[4],m[1]*x+m[3]*y+m[5]]).map_err(invalid)?;color[3]*=f64::from(fa)/255.0;}}
    let stroke=self.stroke_color.unwrap_or([0.0;4]);let alpha=stroke[3]*f64::from(sa)/255.0;let output_alpha=alpha+color[3]*(1.0-alpha);
    if output_alpha>0.0 {let pixels=&mut self.candidate.as_mut().unwrap().pixels;for c in 0..3 {pixels[at*4+c]=(255.0*(alpha*stroke[c]+(1.0-alpha)*color[3]*color[c])/output_alpha).round() as u8;}pixels[at*4+3]=(output_alpha*255.0).round() as u8;}
    self.completed=self.at;
   }
   PathRasterPhase::Complete=>{}
  }Ok(())
 }
 pub fn advance(&mut self,budget:usize)->Result<PathRasterProgress,PathRasterError> {
  if budget==0||budget as u128>9_007_199_254_740_991 {return Err(invalid("Path raster work grant must be a positive integer"));}
  if self.cancelled {return Err(PathRasterError::Cancelled);}if let Some(error)=&self.failed {return Err(error.clone());}
  for _ in 0..budget {if self.phase==PathRasterPhase::Complete {break;}if let Err(error)=self.step() {self.failed=Some(error.clone());return Err(error);}self.work+=1;}
  Ok(PathRasterProgress {phase:self.phase,completed:self.completed,total:self.total,work:self.work,done:self.phase==PathRasterPhase::Complete})
 }
 pub fn cancel(&mut self) {self.cancelled=true;}
 pub fn result(&self)->Result<&RasterImage,PathRasterError> {if self.cancelled {return Err(PathRasterError::Cancelled);}if let Some(error)=&self.failed {return Err(error.clone());}if self.phase!=PathRasterPhase::Complete {return Err(PathRasterError::Incomplete);}Ok(self.candidate.as_ref().unwrap())}
 /// 🧹️ Transfers complete pixels and adopts the actual active or closing child owners before private retirement.
 pub fn into_retirement(mut self)->(PathRasterRetirement,Option<RasterImage>){
  let output=if self.phase==PathRasterPhase::Complete&&!self.cancelled&&self.failed.is_none(){self.candidate.take()}else{None};self.cancelled=true;
  if let Some(mut child)=self.flatten.take(){child.cancel();self.flatten_retirement=Some(child.into_retirement().0);}
  if let Some(mut child)=self.outline.take(){child.cancel();self.outline_retirement=Some(child.into_retirement().0);}
  if let Some(mut child)=self.coverage.take(){child.cancel();self.coverage_retirement=Some(child.into_retirement().0);}
  self.fill_retirement=self.fill.take().map(PreparedFill::into_retirement);(PathRasterRetirement{job:Some(self),slot:0,counter:WorkRetirementCounter::default()},output)
 }
}
fn retire_contour<T>(contours:&mut Vec<T>)->bool{if contours.pop().is_some(){false}else{*contours=Vec::new();true}}
/// 🧽️ Whole path private owners with exact structural grants and an empty terminal destructor.
pub struct PathRasterRetirement{job:Option<PathRasterJob>,slot:u8,counter:WorkRetirementCounter}
impl PathRasterRetirement{
 pub fn terminal_is_empty(&self)->bool{self.job.is_none()}
 fn step(&mut self){let job=self.job.as_mut().unwrap();
  macro_rules! child{($field:ident)=>{if let Some(owner)=&mut job.$field{if !owner.terminal_is_empty(){owner.advance(1).expect("valid child unit grant");false}else{job.$field=None;true}}else{true}}}
  let complete=match self.slot{0=>child!(flatten_retirement),1=>child!(outline_retirement),2=>child!(coverage_retirement),3=>child!(fill_retirement),4=>{job.source=Vec::new();true},5=>{job.segments=Vec::new();true},6=>retire_contour(&mut job.flat),7=>retire_contour(&mut job.fill_contours),8=>retire_contour(&mut job.stroke_contours),9=>retire_contour(&mut job.stroke_polygons),10=>{job.fill_mask=None;true},11=>{job.stroke_mask=None;true},12=>{if let Some(style)=&mut job.style{style.dash=Vec::new();}true},13=>{job.style=None;job.stroke_color=None;true},14=>{job.candidate=None;true},15=>{job.inverse=None;job.failed=None;true},_=>unreachable!()};if complete{self.slot+=1;}if self.slot==16{self.job=None;}
 }
 pub fn advance(&mut self,grant:usize)->Result<WorkRetirementProgress,PathRasterError>{let mut counter=self.counter;let result=counter.advance(grant,||{self.step();self.terminal_is_empty()});self.counter=counter;result.map_err(invalid)}
}
#[cfg(test)]
#[path="🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
