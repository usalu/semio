//! 🖼️ Resolved world-space scene inputs with isolated scopes and cropped private paint.
use crate::{PathSegment,FillRule,FillStyle,StrokeStyle};
use crate::schema::DrawingSceneGroup;
use semio_framework_pixels::{RasterImage,image_decoding::{ImageDecodeInput,ImageDecodeJob,ImageDecodeProgress}};
use semio_framework_pixels::affine_sampling::{AffineImageJob,AffineImageInput,AffineSampling};
use std::sync::Arc;
#[derive(Clone,Debug)]
pub struct RasterSceneAsset{pub id:String,pub mime:String,pub data:Arc<String>}
#[derive(Clone,Debug)]
pub enum RasterSceneContent {Path {segments:Vec<PathSegment>,fill_rule:FillRule,fill:Option<FillStyle>,stroke:Option<StrokeStyle>},Pixels(Arc<RasterImage>),Image{asset:String,width:f64,height:f64}}
#[derive(Clone,Debug)]
pub struct RasterSceneNode {pub id:String,pub groups:Vec<DrawingSceneGroup>,pub transform:[f64;6],pub opacity:f64,pub blend_mode:String,pub visible:bool,pub content:RasterSceneContent}
#[derive(Clone,Debug)]
pub struct RasterSceneInput {pub width:u32,pub height:u32,pub origin:[f64;2],pub tolerance:f64,pub max_pixels:usize,pub max_source_bytes:usize,pub max_bytes:usize,pub max_chunks:usize,pub assets:Vec<RasterSceneAsset>,pub nodes:Vec<RasterSceneNode>}
#[derive(Clone,Debug)]
pub struct RasterSceneProgress {pub phase:&'static str,pub nodes:usize,pub total_nodes:usize,pub pixels:usize,pub assets:usize,pub total_assets:usize,pub source_bytes:usize,pub decodes:usize,pub decoding:Option<ImageDecodeProgress>,pub work:u64,pub done:bool}
use crate::schema::geometry::{arc_geometry,raster::{PathRasterJob,PathRasterInput}};
use semio_framework_pixels::{editing::{validate_extent,validate_image},compositing::{CompositeJob,CompositeInput,CompositeLayer,CompositeContent,CompositeBlend}};
use std::collections::{BTreeMap,BTreeSet,VecDeque};
const IDENTITY:[f64;6]=[1.0,0.0,0.0,1.0,0.0,0.0];
#[derive(Clone,Debug,PartialEq,Eq)]
pub enum RasterSceneError {Invalid(String),Incomplete,Cancelled}
impl std::fmt::Display for RasterSceneError {
 fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{match self{Self::Invalid(message)=>f.write_str(message),Self::Incomplete=>f.write_str("Scene raster incomplete"),Self::Cancelled=>f.write_str("Scene raster cancelled")}}
}
impl std::error::Error for RasterSceneError {}
fn invalid(error:impl std::fmt::Display)->RasterSceneError{RasterSceneError::Invalid(error.to_string())}
fn coordinate(v:f64)->bool{v.is_finite()&&v.abs()<=1e9}
fn valid_id(id:&str)->bool{!id.is_empty()&&id.len()<=4096}
fn style(opacity:f64,blend:&str)->Result<CompositeBlend,RasterSceneError>{
 if !opacity.is_finite()||!(0.0..=1.0).contains(&opacity){return Err(invalid("Invalid scene opacity"));}
 blend.parse().map_err(invalid)
}
fn map(p:[f64;2],m:[f64;6])->[f64;2]{[m[0]*p[0]+m[2]*p[1]+m[4],m[1]*p[0]+m[3]*p[1]+m[5]]}
struct Scope {group:DrawingSceneGroup,layers:Vec<CompositeLayer>}
/// 🧱️ Incremental cropped paint and isolated group compilation into a private tiled candidate.
pub struct RasterSceneJob {
 width:u32,height:u32,origin:[f64;2],tolerance:f64,max_pixels:usize,total_nodes:usize,source:VecDeque<RasterSceneNode>,
 max_source_bytes:usize,max_bytes:usize,max_chunks:usize,total_assets:usize,asset_source:VecDeque<RasterSceneAsset>,assets:usize,source_bytes:usize,decodes:usize,
 catalog:BTreeMap<String,RasterSceneAsset>,decoded:BTreeMap<String,Arc<RasterImage>>,decoder:Option<ImageDecodeJob>,decoder_key:String,decoder_charged:bool,decoding:Option<ImageDecodeProgress>,
 nodes:usize,group_at:usize,pixels:usize,work:u64,entries:usize,phase:&'static str,current:Option<RasterSceneNode>,
 stack:Vec<Scope>,root:Vec<CompositeLayer>,closed:BTreeSet<String>,ids:BTreeSet<String>,images:BTreeMap<String,Arc<RasterImage>>,
 segment:usize,from:[f64;2],start:[f64;2],contour:bool,bounds:[f64;4],crop:[f64;6],painter:Option<PathRasterJob>,sampler:Option<AffineImageJob>,compositor:Option<CompositeJob>,output:Option<RasterImage>,cancelled:bool,failure:Option<RasterSceneError>,
}
impl RasterSceneJob {
 pub fn new(input:RasterSceneInput)->Result<Self,RasterSceneError>{
  validate_extent(input.width,input.height).map_err(invalid)?;
  if !input.origin.into_iter().all(coordinate)||!input.tolerance.is_finite()||!(1e-6..=16.0).contains(&input.tolerance)||!(1..=67_108_864).contains(&input.max_pixels)||input.nodes.len()>1024||input.assets.len()>1024||!(1..=268439552).contains(&input.max_source_bytes)||!(8..=67108864).contains(&input.max_bytes)||!(1..=65536).contains(&input.max_chunks){return Err(invalid("Invalid scene raster contract"));}
  let total_nodes=input.nodes.len();let total_assets=input.assets.len();
  Ok(Self{width:input.width,height:input.height,origin:input.origin,tolerance:input.tolerance,max_pixels:input.max_pixels,total_nodes,source:input.nodes.into(),max_source_bytes:input.max_source_bytes,max_bytes:input.max_bytes,max_chunks:input.max_chunks,total_assets,asset_source:input.assets.into(),assets:0,source_bytes:0,decodes:0,catalog:BTreeMap::new(),decoded:BTreeMap::new(),decoder:None,decoder_key:String::new(),decoder_charged:false,decoding:None,nodes:0,group_at:0,pixels:0,work:0,entries:0,phase:"assets",current:None,stack:Vec::new(),root:Vec::new(),closed:BTreeSet::new(),ids:BTreeSet::new(),images:BTreeMap::new(),segment:0,from:[0.0;2],start:[0.0;2],contour:false,bounds:[f64::INFINITY,f64::INFINITY,f64::NEG_INFINITY,f64::NEG_INFINITY],crop:IDENTITY,painter:None,sampler:None,compositor:None,output:None,cancelled:false,failure:None})
 }
 fn layers(&mut self)->&mut Vec<CompositeLayer>{self.stack.last_mut().map_or(&mut self.root,|s|&mut s.layers)}
 fn active(&self)->bool{let n=self.current.as_ref().unwrap();n.visible&&n.opacity>0.0&&self.stack.iter().all(|s|s.group.opacity>0.0)}
 fn entry(&mut self)->Result<(),RasterSceneError>{self.entries+=1;if self.entries>1024{return Err(invalid("Scene node count exceeds compositor budget"));}Ok(())}
 fn close(&mut self)->Result<(),RasterSceneError>{
  let scope=self.stack.pop().unwrap();self.closed.insert(scope.group.id);
  let layer=CompositeLayer{opacity:scope.group.opacity,blend:style(scope.group.opacity,&scope.group.blend_mode)?,visible:true,transform:IDENTITY,mask:None,content:CompositeContent::Group(scope.layers)};
  self.layers().push(layer);Ok(())
 }
 fn reserve(&mut self,count:usize)->Result<(),RasterSceneError>{if count>self.max_pixels.saturating_sub(self.pixels){return Err(invalid("Scene aggregate pixel budget exceeded"));}self.pixels+=count;Ok(())}
 fn add(&mut self,image:Arc<RasterImage>,matrix:[f64;6])->Result<(),RasterSceneError>{
  let n=self.current.take().unwrap();let key=self.nodes.to_string();self.images.insert(key.clone(),image);
  let layer=CompositeLayer{opacity:n.opacity,blend:style(n.opacity,&n.blend_mode)?,visible:true,transform:matrix,mask:None,content:CompositeContent::Pixels(key)};
  self.layers().push(layer);self.nodes+=1;self.phase="nodes";Ok(())
 }
 fn include(&mut self,p:[f64;2])->Result<(),RasterSceneError>{
  if !p.into_iter().all(f64::is_finite){return Err(invalid("Scene path bounds exceed numeric limits"));}
  self.bounds[0]=self.bounds[0].min(p[0]);self.bounds[1]=self.bounds[1].min(p[1]);self.bounds[2]=self.bounds[2].max(p[0]);self.bounds[3]=self.bounds[3].max(p[1]);Ok(())
 }
 fn point(p:[f64;2])->Result<[f64;2],RasterSceneError>{if !p.into_iter().all(coordinate){return Err(invalid("Invalid scene path coordinate"));}Ok(p)}
 fn bound(&mut self)->Result<(),RasterSceneError>{
  let n=self.current.as_ref().unwrap();let RasterSceneContent::Path{segments,..}=&n.content else{return Err(invalid("Expected scene path"));};
  let m=n.transform;let segment=segments.get(self.segment).cloned();self.segment+=1;let Some(segment)=segment else{return self.paint();};
  if let PathSegment::Move{to}=segment{self.from=Self::point(to)?;self.start=self.from;self.contour=true;return self.include(map(self.from,m));}
  if !self.contour{return Err(invalid("Scene drawing segment requires a moveto"));}
  self.include(map(self.from,m))?;
  let to=match segment{
   PathSegment::Close=>self.start,
   PathSegment::Line{to}=>to,
   PathSegment::Quad{ctrl,to}=>{self.include(map(Self::point(ctrl)?,m))?;to},
   PathSegment::Cubic{ctrl1,ctrl2,to}=>{self.include(map(Self::point(ctrl1)?,m))?;self.include(map(Self::point(ctrl2)?,m))?;to},
   PathSegment::Arc{rx,ry,rotation,large_arc,sweep,to}=>{
    Self::point(to)?;if ![rx,ry,rotation].into_iter().all(coordinate){return Err(invalid("Invalid scene arc"));}
    if rx!=0.0&&ry!=0.0&&self.from!=to{
     if let Some(arc)=arc_geometry(self.from,[rx.abs(),ry.abs()],rotation%360.0,large_arc,sweep,to){
      let [rx,ry]=arc.radii;let(a,b)=(arc.rotation.cos(),arc.rotation.sin());let center=map(arc.center,m);let ex=(m[0]*rx*a+m[2]*rx*b).hypot(-m[0]*ry*b+m[2]*ry*a);let ey=(m[1]*rx*a+m[3]*rx*b).hypot(-m[1]*ry*b+m[3]*ry*a);
      self.include([center[0]-ex,center[1]-ey])?;self.include([center[0]+ex,center[1]+ey])?;
     }else{self.include(self.origin)?;self.include([self.origin[0]+f64::from(self.width),self.origin[1]+f64::from(self.height)])?;}
    }to
   },
   PathSegment::Move{..}=>unreachable!(),
  };
  self.from=Self::point(to)?;self.include(map(to,m))
 }
 fn paint(&mut self)->Result<(),RasterSceneError>{
  let n=self.current.as_ref().unwrap();let RasterSceneContent::Path{fill,stroke,..}=&n.content else{return Err(invalid("Expected scene path"));};let m=n.transform;
  if !self.active()||fill.is_none()&&stroke.is_none()||m[0]*m[3]-m[1]*m[2]==0.0||self.bounds[0]==f64::INFINITY{self.current=None;self.nodes+=1;self.phase="nodes";return Ok(());}
  let pad=stroke.as_ref().map_or(0.0,|s|s.width*2.0);let px=pad*m[0].hypot(m[2]);let py=pad*m[1].hypot(m[3]);
  let left=(self.bounds[0]-px-self.origin[0]).floor().max(0.0);let top=(self.bounds[1]-py-self.origin[1]).floor().max(0.0);let right=(self.bounds[2]+px-self.origin[0]).ceil().min(f64::from(self.width));let bottom=(self.bounds[3]+py-self.origin[1]).ceil().min(f64::from(self.height));
  if right<=left||bottom<=top{self.current=None;self.nodes+=1;self.phase="nodes";return Ok(());}
  let width=(right-left)as u32;let height=(bottom-top)as u32;self.reserve(width as usize*height as usize)?;let origin=[self.origin[0]+left,self.origin[1]+top];self.crop=[1.0,0.0,0.0,1.0,origin[0],origin[1]];
  let n=self.current.as_mut().unwrap();let RasterSceneContent::Path{segments,fill_rule,fill,stroke}=&mut n.content else{unreachable!()};
  self.painter=Some(PathRasterJob::new(PathRasterInput{width,height,origin,segments:std::mem::take(segments),transform:m,tolerance:self.tolerance,fill_rule:*fill_rule,fill:fill.take(),stroke:stroke.take()}).map_err(invalid)?);self.phase="path";Ok(())
 }
 fn skip(&mut self){self.current=None;self.nodes+=1;self.phase="nodes";}
 fn asset_step(&mut self)->Result<(),RasterSceneError>{
  let Some(asset)=self.asset_source.pop_front()else{self.phase="nodes";return Ok(());};
  if !valid_id(&asset.id)||self.catalog.contains_key(&asset.id)||asset.mime.is_empty()||asset.mime.len()>128||asset.data.is_empty(){return Err(invalid("Invalid scene asset catalog"));}
  if asset.data.len()>self.max_source_bytes-self.source_bytes{return Err(invalid("Scene aggregate source byte budget exceeded"));}
  self.source_bytes+=asset.data.len();self.catalog.insert(asset.id.clone(),asset);self.assets+=1;Ok(())
 }
 fn image_crop(&self,width:f64,height:f64)->Result<Option<[f64;4]>,RasterSceneError>{
  let m=self.current.as_ref().unwrap().transform;if !self.active()||m[0]*m[3]-m[1]*m[2]==0.0{return Ok(None);}
  let points=[[0.0,0.0],[width,0.0],[width,height],[0.0,height]].map(|p|map(p,m));let mut min=[f64::INFINITY;2];let mut max=[f64::NEG_INFINITY;2];
  for p in points{if !p.into_iter().all(f64::is_finite){return Err(invalid("Scene image bounds exceed numeric limits"));}for axis in 0..2{min[axis]=min[axis].min(p[axis]);max[axis]=max[axis].max(p[axis]);}}
  let left=(min[0]-self.origin[0]).floor().max(0.0);let top=(min[1]-self.origin[1]).floor().max(0.0);let right=(max[0]-self.origin[0]).ceil().min(f64::from(self.width));let bottom=(max[1]-self.origin[1]).ceil().min(f64::from(self.height));
  Ok((right>left&&bottom>top).then_some([left,top,right,bottom]))
 }
 fn asset_image(&mut self,width:f64,height:f64,image:Arc<RasterImage>)->Result<(),RasterSceneError>{
  let n=self.current.as_mut().unwrap();let m=n.transform;let sx=width/f64::from(image.width);let sy=height/f64::from(image.height);
  n.transform=[m[0]*sx,m[1]*sx,m[2]*sy,m[3]*sy,m[4],m[5]];self.image(image,true)
 }
 fn prepare_image(&mut self,key:&str,width:f64,height:f64)->Result<(),RasterSceneError>{
  if !valid_id(key)||!coordinate(width)||width<=0.0||!coordinate(height)||height<=0.0{return Err(invalid("Invalid authored scene image"));}
  if !self.decoded.contains_key(key)&&!self.catalog.contains_key(key){return Err(invalid(format!("Missing scene image asset: {key}")));}
  if self.image_crop(width,height)?.is_none(){self.skip();return Ok(());}
  if let Some(cached)=self.decoded.get(key){return self.asset_image(width,height,Arc::clone(cached));}
  let available=(self.max_pixels-self.pixels).min(16777216);if available==0{return Err(invalid("Scene aggregate pixel budget exceeded"));}
  let asset=self.catalog.remove(key).unwrap();
  self.decoder=Some(ImageDecodeJob::new(ImageDecodeInput{mime:asset.mime,data:asset.data,max_source_bytes:self.max_source_bytes,max_bytes:self.max_bytes,max_pixels:available,max_chunks:self.max_chunks}).map_err(invalid)?);
  self.decoding=None;self.decoder_key=asset.id;self.decoder_charged=false;self.decodes+=1;self.phase="source";Ok(())
 }
 fn source_step(&mut self)->Result<(),RasterSceneError>{
  let p=self.decoder.as_mut().unwrap().advance(1).map_err(invalid)?;self.decoding=Some(p);
  if p.total_pixels!=0&&!self.decoder_charged{self.reserve(p.total_pixels)?;self.decoder_charged=true;}
  if !p.done{return Ok(());}
  let RasterSceneContent::Image{width,height,..}=self.current.as_ref().unwrap().content else{return Err(invalid("Expected scene image source"));};
  let image=Arc::new(self.decoder.take().unwrap().into_result().map_err(invalid)?);self.decoded.insert(std::mem::take(&mut self.decoder_key),Arc::clone(&image));self.asset_image(width,height,image)
 }
 fn image(&mut self,image:Arc<RasterImage>,charged:bool)->Result<(),RasterSceneError>{
  validate_image(&image).map_err(invalid)?;let count=validate_extent(image.width,image.height).map_err(invalid)?;let m=self.current.as_ref().unwrap().transform;
  let Some([left,top,right,bottom])=self.image_crop(f64::from(image.width),f64::from(image.height))?else{self.skip();return Ok(());};if !charged{self.reserve(count)?;}
  if m[..4].iter().all(|v|[-1.0,0.0,1.0].contains(v))&&m[0]*m[0]+m[1]*m[1]==1.0&&m[2]*m[2]+m[3]*m[3]==1.0&&m[0]*m[2]+m[1]*m[3]==0.0&&(m[4]-self.origin[0]).fract()==0.0&&(m[5]-self.origin[1]).fract()==0.0{return self.add(image,m);}
  let width=(right-left)as u32;let height=(bottom-top)as u32;self.reserve(width as usize*height as usize)?;let origin=[self.origin[0]+left,self.origin[1]+top];self.crop=[1.0,0.0,0.0,1.0,origin[0],origin[1]];
  self.sampler=Some(AffineImageJob::new(AffineImageInput{source:image,width,height,origin,transform:m,sampling:AffineSampling::Auto}).map_err(invalid)?);self.phase="image";Ok(())
 }
 fn step(&mut self)->Result<(),RasterSceneError>{
  if self.phase=="assets"{return self.asset_step();}
  if self.phase=="source"{return self.source_step();}
  if self.phase=="nodes"{
   if self.current.is_none(){
    let Some(n)=self.source.pop_front()else{
     if !self.stack.is_empty(){return self.close();}
     self.catalog.clear();self.decoded.clear();self.compositor=Some(CompositeJob::new(CompositeInput{width:self.width,height:self.height,origin:self.origin,images:std::mem::take(&mut self.images),layers:std::mem::take(&mut self.root)}).map_err(invalid)?);self.closed.clear();self.ids.clear();self.phase="compositing";return Ok(());
    };
    if !valid_id(&n.id)||self.ids.contains(&n.id)||!n.transform.into_iter().all(coordinate)||n.groups.len()>32{return Err(invalid("Invalid resolved scene node"));}
    style(n.opacity,&n.blend_mode)?;self.ids.insert(n.id.clone());self.entry()?;self.current=Some(n);self.group_at=0;return Ok(());
   }
   let n=self.current.as_ref().unwrap();let group=n.groups.get(self.group_at).cloned();let scope=self.stack.get(self.group_at);
   if let Some(g)=&group{if !valid_id(&g.id){return Err(invalid("Invalid scene group"));}style(g.opacity,&g.blend_mode)?;}
   if scope.is_some_and(|s|group.as_ref().is_none_or(|g|s.group.id!=g.id)){return self.close();}
   if let Some(g)=group{
    if let Some(s)=scope{if s.group.opacity!=g.opacity||s.group.blend_mode!=g.blend_mode{return Err(invalid("Inconsistent scene group style"));}}
    else{if self.closed.contains(&g.id)||self.stack.iter().any(|s|s.group.id==g.id){return Err(invalid("Scene group scope cannot reopen"));}self.entry()?;self.stack.push(Scope{group:g,layers:Vec::new()});}
    self.group_at+=1;return Ok(());
   }
   let n=self.current.as_ref().unwrap();match &n.content{
    RasterSceneContent::Pixels(image)=>{
     let image=Arc::clone(image);self.image(image,false)?;
    },
    RasterSceneContent::Image{asset,width,height}=>{let key=asset.clone();let (width,height)=(*width,*height);self.prepare_image(&key,width,height)?;},
    RasterSceneContent::Path{segments,fill_rule,fill,stroke}=>{
     if segments.len()>65536{return Err(invalid("Scene path exceeds segment budget"));}
     PathRasterJob::new(PathRasterInput{width:1,height:1,origin:[0.0;2],segments:Vec::new(),transform:n.transform,tolerance:self.tolerance,fill_rule:*fill_rule,fill:fill.clone(),stroke:stroke.clone()}).map_err(invalid)?;
     self.segment=0;self.from=[0.0;2];self.start=[0.0;2];self.contour=false;self.bounds=[f64::INFINITY,f64::INFINITY,f64::NEG_INFINITY,f64::NEG_INFINITY];self.phase="bounds";
    },
   }
  }else if self.phase=="bounds"{self.bound()?;}
  else if self.phase=="path"{if self.painter.as_mut().unwrap().advance(1).map_err(invalid)?.done{let image=self.painter.take().unwrap().into_result().map_err(invalid)?;self.add(Arc::new(image),self.crop)?;}}
  else if self.phase=="image"{if self.sampler.as_mut().unwrap().advance(1).map_err(invalid)?.done{let image=self.sampler.take().unwrap().into_result().map_err(invalid)?;self.add(Arc::new(image),self.crop)?;}}
  else if self.phase=="compositing"{if self.compositor.as_mut().unwrap().advance(1).map_err(invalid)?.done{self.output=Some(self.compositor.take().unwrap().into_result().map_err(invalid)?);self.current=None;self.phase="complete";}}
  Ok(())
 }
 pub fn advance(&mut self,budget:usize)->Result<RasterSceneProgress,RasterSceneError>{
  if budget==0||budget as u128>9_007_199_254_740_991{return Err(invalid("Scene work grant must be a positive integer"));}
  if self.cancelled{return Err(RasterSceneError::Cancelled);}if let Some(error)=&self.failure{return Err(error.clone());}
  for _ in 0..budget{if self.phase=="complete"{break;}if let Err(error)=self.step(){self.failure=Some(error.clone());self.clear();return Err(error);}self.work+=1;}
  Ok(RasterSceneProgress{phase:self.phase,nodes:self.nodes,total_nodes:self.total_nodes,pixels:self.pixels,assets:self.assets,total_assets:self.total_assets,source_bytes:self.source_bytes,decodes:self.decodes,decoding:self.decoding,work:self.work,done:self.phase=="complete"})
 }
 fn clear(&mut self){if let Some(job)=&mut self.decoder{job.cancel();}self.decoder=None;self.decoder_key=String::new();self.decoding=None;self.asset_source=VecDeque::new();self.catalog=BTreeMap::new();self.decoded=BTreeMap::new();if let Some(job)=&mut self.painter{job.cancel();}if let Some(job)=&mut self.sampler{job.cancel();}if let Some(job)=&mut self.compositor{job.cancel();}self.painter=None;self.sampler=None;self.compositor=None;self.output=None;self.current=None;self.source=VecDeque::new();self.stack=Vec::new();self.root=Vec::new();self.images=BTreeMap::new();self.ids.clear();self.closed.clear();}
 pub fn cancel(&mut self){self.cancelled=true;self.clear();}
 pub fn result(&self)->Result<&RasterImage,RasterSceneError>{if self.cancelled{return Err(RasterSceneError::Cancelled);}if let Some(error)=&self.failure{return Err(error.clone());}if self.phase!="complete"{return Err(RasterSceneError::Incomplete);}Ok(self.output.as_ref().unwrap())}
 pub fn into_result(mut self)->Result<RasterImage,RasterSceneError>{self.result()?;Ok(self.output.take().unwrap())}
}
#[cfg(test)]
#[path="🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
