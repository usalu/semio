//! 📋️ Complete authored documents become private typed plans under work grants.
use semio_framework_value::paged::Utf8Text;
use crate::{DrawingSnapshot,DrawingLayerNode,FillStyle,StrokeStyle,PathSegment,FillRule};
use crate::schema::{layer_base,shape_path_segment,drawing_transform_to_matrix,DrawingSceneGroup};
use crate::schema::scene_raster::{RasterSceneAsset,RasterSceneNode,RasterSceneContent,RasterSceneInput};
use crate::schema::scene_retirement::ScenePlanCloseJob;
use semio_framework_2d::retirement::{WorkRetirementCounter,WorkRetirementProgress};
pub type DocumentSceneRetirementProgress=WorkRetirementProgress;
pub type DocumentVectorRetirementProgress=WorkRetirementProgress;
use std::{collections::VecDeque,sync::Arc};
use semio_framework_value::retirement::{RetireOwned,RetirementCursor,controlled::ControlledRetirement};
use semio_framework_value::{ValueError,retained_clone::{RetainedCloneGrant,RetainedCloneStep}};
#[derive(Clone,Debug,semio_framework_value::RetireOwned)]
pub(crate) struct OwnedTable<K:RetireOwned+Eq,V:RetireOwned>{values:Vec<(K,V)>,retired_keys:Vec<K>,retired_values:Vec<V>}
impl<K:RetireOwned+Eq,V:RetireOwned> OwnedTable<K,V>{
 pub(crate) fn new()->Self{Self{values:Vec::new(),retired_keys:Vec::new(),retired_values:Vec::new()}}
 pub(crate) fn get<Q:Eq+?Sized>(&self,key:&Q)->Option<&V>where K:std::borrow::Borrow<Q>{self.values.iter().find(|(id,_)|id.borrow()==key).map(|(_,value)|value)}
 pub(crate) fn contains_key<Q:Eq+?Sized>(&self,key:&Q)->bool where K:std::borrow::Borrow<Q>{self.get(key).is_some()}
 pub(crate) fn insert(&mut self,key:K,value:V){if let Some(at)=self.values.iter().position(|(id,_)|id==&key){self.retired_keys.push(key);self.retired_values.push(std::mem::replace(&mut self.values[at].1,value));}else{self.values.push((key,value));}}
 pub(crate) fn remove<Q:Eq+?Sized>(&mut self,key:&Q)->Option<V>where K:std::borrow::Borrow<Q>{let at=self.values.iter().position(|(id,_)|id.borrow()==key)?;let(key,value)=self.values.swap_remove(at);self.retired_keys.push(key);Some(value)}
}
impl<K:RetireOwned+Eq+std::borrow::Borrow<Q>,V:RetireOwned,Q:Eq+?Sized> std::ops::Index<&Q> for OwnedTable<K,V>{type Output=V;fn index(&self,key:&Q)->&V{self.get(key).expect("admitted scene key")}}
#[derive(Clone,Debug,semio_framework_value::RetireOwned)]
pub(crate) struct OwnedSet<T:RetireOwned+Eq>{values:Vec<T>,retired:Vec<T>,generations:Vec<Vec<T>>}
impl<T:RetireOwned+Eq> OwnedSet<T>{
 pub(crate) fn new()->Self{Self{values:Vec::new(),retired:Vec::new(),generations:Vec::new()}}
 pub(crate) fn contains(&self,value:&T)->bool{self.values.contains(value)}
 pub(crate) fn insert(&mut self,value:T)->bool{if self.contains(&value){self.retired.push(value);false}else{self.values.push(value);true}}
 pub(crate) fn remove(&mut self,value:&T)->bool{if let Some(at)=self.values.iter().position(|item|item==value){self.retired.push(self.values.swap_remove(at));true}else{false}}
 pub(crate) fn reset(&mut self){self.generations.push(std::mem::take(&mut self.values));}
}
#[path="🧭️selection/🦀️.rs"]
pub mod selection;
pub use selection::{scene_selection_relation,SceneSelectionRelation};
use crate::schema::scene_booleans::{DocumentBooleanInput,DocumentBooleanJob,DocumentBooleanLimits,DocumentBooleanProgress,DocumentBooleanRetirement};
use crate::schema::scene_text::{DocumentTextJob,DocumentTextRetirement};
use crate::schema::scene_trace::{DocumentTraceInput,DocumentTraceJob,DocumentTraceLimits,DocumentTraceProgress,DocumentTraceRetirement};
#[derive(Clone,Copy,Debug)]
pub struct DocumentSceneLimits{pub max_nodes:usize,pub max_depth:usize,pub max_segments:usize,pub max_references:usize,pub max_source_bytes:usize}
impl Default for DocumentSceneLimits{fn default()->Self{Self{max_nodes:1024,max_depth:32,max_segments:65536,max_references:32768,max_source_bytes:268439552}}}
#[derive(Clone,Debug,semio_framework_value::RetireOwned)]
pub enum DocumentSceneContent{
 Path{segments:Vec<PathSegment>,fill_rule:FillRule,fill:Option<FillStyle>,stroke:Option<StrokeStyle>},
 Image{asset:String,width:f64,height:f64},Group{children:Vec<String>,isolation:bool},
 Text{content:String,x:f64,y:f64,size:f64,font_family:crate::DrawingFontFamily,fill_rule:FillRule,fill:Option<FillStyle>,stroke:Option<StrokeStyle>},
 Glyphs{content:String,x:f64,y:f64,size:f64,font_family:crate::DrawingFontFamily,segments:Vec<PathSegment>,fill_rule:FillRule,fill:Option<FillStyle>,stroke:Option<StrokeStyle>},
 Boolean{operation:String,children:Vec<String>,reference_transform:[f64;6],fill_rule:FillRule,fill:Option<FillStyle>,stroke:Option<StrokeStyle>},
 Trace{source:String,threshold:f64,simplify_epsilon:f64,fill_rule:FillRule,fill:Option<FillStyle>,stroke:Option<StrokeStyle>},
}
impl DocumentSceneContent{
 fn paint(&mut self)->Option<(&mut Option<FillStyle>,&mut Option<StrokeStyle>)>{match self{Self::Path{fill,stroke,..}|Self::Glyphs{fill,stroke,..}|Self::Text{fill,stroke,..}|Self::Boolean{fill,stroke,..}|Self::Trace{fill,stroke,..}=>Some((fill,stroke)),_=>None}}
 fn children(&self)->&[String]{match self{Self::Group{children,..}|Self::Boolean{children,..}=>children,_=>&[]}}
}
#[derive(Clone,Debug,semio_framework_value::RetireOwned)]
pub struct DocumentSceneNode{pub source_path:Vec<u16>,pub locked_ancestors:u32,pub id:String,pub groups:Vec<DrawingSceneGroup>,pub transform:[f64;6],pub opacity:f64,pub blend_mode:String,pub visible:bool,pub content:DocumentSceneContent}
#[derive(Clone,Debug,Default,semio_framework_value::RetireOwned)]
pub struct DocumentScenePlan{pub assets:Vec<RasterSceneAsset>,pub nodes:Vec<DocumentSceneNode>}
/// 🧭️ Validate the bounded authored address and the exact unsigned lock-mask width.
pub fn validate_scene_source_address(path:&[u16],locks:u32)->Result<(),DocumentSceneError>{if path.is_empty()||path.len()>32||locks.checked_shr(path.len()as u32).is_some_and(|bits|bits!=0){return Err(invalid("Invalid scene source ancestry"));}Ok(())}
#[derive(Clone,Debug)]
pub struct DocumentSceneProgress{pub phase:&'static str,pub layers:usize,pub assets:usize,pub segments:usize,pub references:usize,pub source_bytes:usize,pub work:u64,pub done:bool}
#[derive(Clone,Debug,PartialEq,Eq,semio_framework_value::RetireOwned)]
pub enum DocumentSceneError{Invalid(String),Incomplete,Cancelled}
impl std::fmt::Display for DocumentSceneError{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{match self{Self::Invalid(message)=>f.write_str(message),Self::Incomplete=>f.write_str("Document scene preparation incomplete"),Self::Cancelled=>f.write_str("Document scene preparation cancelled")}}}
impl std::error::Error for DocumentSceneError{}
fn invalid(message:impl Into<String>)->DocumentSceneError{DocumentSceneError::Invalid(message.into())}
fn coordinate(n:f64)->Result<f64,DocumentSceneError>{if n.is_finite()&&n.abs()<=1e9{Ok(n)}else{Err(invalid("Invalid authored scene coordinate"))}}
fn positive(n:f64)->Result<f64,DocumentSceneError>{coordinate(n)?;if n>0.0{Ok(n)}else{Err(invalid("Expected positive authored scene dimension"))}}
fn unit(n:f64)->Result<f64,DocumentSceneError>{if n.is_finite()&&(0.0..=1.0).contains(&n){Ok(n)}else{Err(invalid("Expected unit paint or opacity"))}}
fn id(s:&(impl Utf8Text + ?Sized))->Result<(),DocumentSceneError>{if s.text_bytes()>0&&s.text_bytes()<=4096{Ok(())}else{Err(invalid("Invalid scene identity"))}}
fn text_scalar(source:&(impl Utf8Text + ?Sized),chunk:&mut usize,offset:&mut usize)->Result<Option<char>,DocumentSceneError>{
 let value=source.text_chunk(*chunk).ok_or_else(||invalid("Captured scene source changed"))?;
 if *offset==value.len(){*chunk+=1;*offset=0;return Ok(None);}
 let character=value.get(*offset..).and_then(|tail|tail.chars().next()).ok_or_else(||invalid("Captured scene UTF8 cursor changed"))?;
 *offset+=character.len_utf8();Ok(Some(character))
}
fn color(c:[f64;4])->Result<[f64;4],DocumentSceneError>{for n in c{unit(n)?;}Ok(c)}
fn matrix(a:[f64;6],b:[f64;6])->Result<[f64;6],DocumentSceneError>{let mut m=crate::schema::geometry::multiply(a,b);for n in &mut m{coordinate(*n)?;if *n==0.0{*n=0.0;}}Ok(m)}
fn validate_segment(s:&PathSegment)->Result<(),DocumentSceneError>{let p=|v:[f64;2]|{coordinate(v[0])?;coordinate(v[1])?;Ok::<(),DocumentSceneError>(())};match s{PathSegment::Move{to}|PathSegment::Line{to}=>p(*to)?,PathSegment::Quad{ctrl,to}=>{p(*ctrl)?;p(*to)?},PathSegment::Cubic{ctrl1,ctrl2,to}=>{p(*ctrl1)?;p(*ctrl2)?;p(*to)?},PathSegment::Arc{rx,ry,rotation,to,..}=>{coordinate(*rx)?;coordinate(*ry)?;coordinate(*rotation)?;p(*to)?},PathSegment::Close=>{}}Ok(())}
/// 👁️ Scene preparation borrows actual root and asset owners separately in each turn.
pub trait DrawingSceneSource{
 fn root_count(&self)->usize;
 fn root(&self,index:usize)->Option<&DrawingLayerNode>;
 fn asset_count(&self)->usize;
 fn asset_at(&self,index:usize)->Option<(&dyn Utf8Text,&crate::DrawingImageAsset)>;
 fn asset(&self,key:&str)->Option<&crate::DrawingImageAsset>;
 fn root_visible(&self,_index:usize)->bool{true}
}
impl DrawingSceneSource for DrawingSnapshot{
 fn root_count(&self)->usize{self.layers.len()}
 fn root(&self,index:usize)->Option<&DrawingLayerNode>{self.layers.get(index)}
 fn asset_count(&self)->usize{self.assets.len()}
 fn asset_at(&self,index:usize)->Option<(&dyn Utf8Text,&crate::DrawingImageAsset)>{self.assets.retained_entries().get(index).map(|(key,asset)|(key as &dyn Utf8Text,asset))}
 fn asset(&self,key:&str)->Option<&crate::DrawingImageAsset>{self.assets.get(key)}
}
fn scene_layer_at<'d>(document:&'d impl DrawingSceneSource,path:&[usize])->Result<&'d DrawingLayerNode,DocumentSceneError>{let mut node=document.root(*path.first().ok_or_else(||invalid("Missing scene cursor path"))?).ok_or_else(||invalid("Captured scene root disappeared"))?;for at in &path[1..]{let DrawingLayerNode::Group(group)=node else{return Err(invalid("Captured scene group changed"));};node=group.children.get(*at).ok_or_else(||invalid("Captured scene layer disappeared"))?;}Ok(node)}
fn scene_layer_count(document:&impl DrawingSceneSource,path:&[usize])->Result<usize,DocumentSceneError>{if path.is_empty(){return Ok(document.root_count());}let DrawingLayerNode::Group(group)=scene_layer_at(document,path)? else{return Err(invalid("Captured scene group changed"));};Ok(group.children.len())}
fn scene_child_at<'d>(document:&'d impl DrawingSceneSource,path:&[usize],index:usize)->Result<&'d DrawingLayerNode,DocumentSceneError>{if path.is_empty(){return document.root(index).ok_or_else(||invalid("Captured scene root disappeared"));}let DrawingLayerNode::Group(group)=scene_layer_at(document,path)? else{return Err(invalid("Captured scene group changed"));};group.children.get(index).ok_or_else(||invalid("Captured scene layer disappeared"))}
fn scene_source_key(key:&dyn Utf8Text)->Result<String,DocumentSceneError>{id(key)?;let mut result=String::with_capacity(key.text_bytes());for at in 0..key.text_chunk_count(){result.push_str(key.text_chunk(at).ok_or_else(||invalid("Captured asset key changed"))?);}Ok(result)}
#[derive(semio_framework_value::RetireOwned)]
struct Frame{locked_ancestors:u32,path:Vec<usize>,at:usize,matrix:[f64;6],visible:bool,groups:Vec<DrawingSceneGroup>}
/// 🧱️ Retains source positions and copies each character or geometry entry under a grant.
struct DocumentSceneCursor{
 limits:DocumentSceneLimits,asset_key:Option<String>,asset_active:bool,asset_samples:Vec<u8>,asset_char:usize,
 frames:Vec<Frame>,retired_frames:Vec<Frame>,retired_groups:Vec<DrawingSceneGroup>,retired_paths:Vec<Vec<usize>>,current:Option<Vec<usize>>,node:Option<DocumentSceneNode>,plan:DocumentScenePlan,ids:OwnedTable<String,usize>,asset_ids:OwnedSet<String>,
 phase:&'static str,layers:usize,assets:usize,segments:usize,references:usize,source_bytes:usize,work:u64,at:usize,stroke_at:usize,text_at:usize,text_chars:usize,text_chunk:usize,text_chunk_offset:usize,
 validate_at:usize,ref_at:usize,cycle_at:usize,graph:Vec<(usize,usize)>,visited:OwnedSet<usize>,visiting:OwnedSet<usize>,cancelled:bool,failure:Option<DocumentSceneError>,cleanup_slot:u8,
}
impl DocumentSceneCursor{
 pub fn new(document:&impl DrawingSceneSource,limits:DocumentSceneLimits)->Result<Self,DocumentSceneError>{
  if !(1..=1024).contains(&limits.max_nodes)||!(1..=32).contains(&limits.max_depth)||!(1..=65536).contains(&limits.max_segments)||!(1..=32768).contains(&limits.max_references)||!(1..=268439552).contains(&limits.max_source_bytes)||document.root_count()>limits.max_nodes||document.asset_count()>1024{return Err(invalid("Invalid scene preparation limits or document"));}
  Ok(Self{limits,asset_key:None,asset_active:false,asset_samples:Vec::new(),asset_char:0,frames:vec![Frame{locked_ancestors:0,path:Vec::new(),at:0,matrix:[1.0,0.0,0.0,1.0,0.0,0.0],visible:true,groups:Vec::new()}],retired_frames:Vec::new(),retired_groups:Vec::new(),retired_paths:Vec::new(),retired_texts:Vec::new(),current:None,node:None,plan:DocumentScenePlan::default(),ids:OwnedTable::new(),asset_ids:OwnedSet::new(),phase:"assets",layers:0,assets:0,segments:0,references:0,source_bytes:0,work:0,at:0,stroke_at:0,text_at:0,text_chars:0,text_chunk:0,text_chunk_offset:0,validate_at:0,ref_at:0,cycle_at:0,graph:Vec::new(),visited:OwnedSet::new(),visiting:OwnedSet::new(),cancelled:false,failure:None,cleanup_slot:0})
 }
 fn start(&mut self,layer:&DrawingLayerNode,path:Vec<usize>,parent:[f64;6],visible:bool,groups:Vec<DrawingSceneGroup>,locks:u32)->Result<(),DocumentSceneError>{
  if path.is_empty()||path.len()>self.limits.max_depth{return Err(invalid("Scene source path exceeds depth limit"));}
  let source_path=path.iter().map(|index|u16::try_from(*index).map_err(|_|invalid("Scene source index exceeds limit"))).collect::<Result<Vec<_>,_>>()?;
  let b=layer_base(layer);let locked_ancestors=locks|if b.locked{1u32<<(path.len()-1)}else{0};id(&b.id)?;let authored_id=b.id.to_string_owner();if self.layers>=self.limits.max_nodes||self.ids.contains_key(&authored_id){return Err(invalid("Duplicate scene identity or layer limit"));}
  for n in [b.transform.x,b.transform.y,b.transform.scale_x,b.transform.scale_y,b.transform.shear,b.transform.rotation]{coordinate(n)?;}
  let transform=matrix(parent,drawing_transform_to_matrix(&b.transform))?;let opacity=unit(b.opacity)?;if !crate::DRAWING_BLEND_MODES.iter().any(|mode| b.blend_mode.eq_str(mode)){return Err(invalid("Invalid authored scene blend"));}
  let rule=b.attributes.fill_rule.clone();
  let content=match layer{
   DrawingLayerNode::Path(_)|DrawingLayerNode::Shape(_)=>DocumentSceneContent::Path{segments:Vec::new(),fill_rule:rule,fill:None,stroke:None},
   DrawingLayerNode::Image(v)=>{id(&v.image_key)?;DocumentSceneContent::Image{asset:v.image_key.to_string_owner(),width:positive(v.width)?,height:positive(v.height)?}},
   DrawingLayerNode::Group(v)=>{if v.children.len()>1024{return Err(invalid("Invalid scene group"));}DocumentSceneContent::Group{children:Vec::new(),isolation:v.isolation}},
   DrawingLayerNode::Text(v)=>{if v.content.len()>262144{return Err(invalid("Scene text exceeds limit"));}DocumentSceneContent::Text{content:String::new(),x:coordinate(v.x)?,y:coordinate(v.y)?,size:positive(v.size)?,font_family:v.font_family,fill_rule:rule,fill:None,stroke:None}},
   DrawingLayerNode::Boolean(v)=>{if !crate::DRAWING_BOOLEAN_OPERATIONS.iter().any(|operation| v.operation.eq_str(operation))||v.children.len()>1024{return Err(invalid("Invalid boolean scene work"));}DocumentSceneContent::Boolean{operation:v.operation.to_string_owner(),children:Vec::new(),reference_transform:parent,fill_rule:rule,fill:None,stroke:None}},
   DrawingLayerNode::Trace(v)=>{id(&v.source_key)?;let epsilon=coordinate(v.params.simplify_epsilon)?;if epsilon<0.0{return Err(invalid("Invalid trace epsilon"));}DocumentSceneContent::Trace{source:v.source_key.to_string_owner(),threshold:unit(v.params.threshold)?,simplify_epsilon:epsilon,fill_rule:rule,fill:None,stroke:None}},
  };
  self.ids.insert(authored_id.clone(),self.plan.nodes.len());self.layers+=1;self.node=Some(DocumentSceneNode{source_path,locked_ancestors,id:authored_id,groups,transform,opacity,blend_mode:b.blend_mode.to_string_owner(),visible:visible&&b.visible,content});self.current=Some(path);self.at=0;self.stroke_at=0;self.text_at=0;self.text_chars=0;self.text_chunk=0;self.text_chunk_offset=0;self.phase="paint";Ok(())
 }
 fn paint(&mut self,document:&impl DrawingSceneSource)->Result<(),DocumentSceneError>{
  let b=layer_base(scene_layer_at(document,self.current.as_deref().unwrap())?);
  if let Some((out_fill,out_stroke))=self.node.as_mut().unwrap().content.paint(){
   let source=b.attributes.fill.as_ref();
   if self.at==0{
    *out_fill=match source{None=>None,Some(FillStyle::Solid{color:c})=>Some(FillStyle::Solid{color:color(*c)?}),Some(FillStyle::LinearGradient{x1,y1,x2,y2,stops})=>{if stops.is_empty()||stops.len()>64{return Err(invalid("Invalid scene gradient stops"));}Some(FillStyle::LinearGradient{x1:coordinate(*x1)?,y1:coordinate(*y1)?,x2:coordinate(*x2)?,y2:coordinate(*y2)?,stops:Default::default()})},Some(FillStyle::RadialGradient{cx,cy,r,stops})=>{if stops.is_empty()||stops.len()>64{return Err(invalid("Invalid scene gradient stops"));}Some(FillStyle::RadialGradient{cx:coordinate(*cx)?,cy:coordinate(*cy)?,r:positive(*r)?,stops:Vec::new().into()})}};self.at+=1;return Ok(());
   }
   let stops=match source{Some(FillStyle::LinearGradient{stops,..})|Some(FillStyle::RadialGradient{stops,..})=>Some(stops),_=>None};
   if let Some(stop)=stops.and_then(|v|v.get(self.at-1)){let stop=crate::GradientStop{offset:unit(stop.offset)?,color:color(stop.color)?};match out_fill{Some(FillStyle::LinearGradient{stops,..})|Some(FillStyle::RadialGradient{stops,..})=>stops.push(stop),_=>unreachable!()}self.at+=1;return Ok(());}
   let stroke=b.attributes.stroke.as_ref();
   if self.stroke_at==0{if let Some(s)=stroke{coordinate(s.width)?;if s.width<0.0||s.dash.as_ref().is_some_and(|d|d.len()>1024){return Err(invalid("Invalid scene stroke"));}*out_stroke=Some(StrokeStyle{color:color(s.color)?,width:s.width,cap:s.cap.clone(),join:s.join.clone(),dash:s.dash.as_ref().map(|_|Default::default())});}self.stroke_at+=1;return Ok(());}
   if let Some(value)=stroke.and_then(|s|s.dash.as_ref()).and_then(|v|v.get(self.stroke_at-1)){coordinate(*value)?;if *value<0.0{return Err(invalid("Invalid scene dash"));}out_stroke.as_mut().unwrap().dash.as_mut().unwrap().push(*value);self.stroke_at+=1;return Ok(());}
  }
  self.at=0;self.phase=if matches!(scene_layer_at(document,self.current.as_deref().unwrap())?,DrawingLayerNode::Group(_)|DrawingLayerNode::Boolean(_)){"references"}else{"geometry"};Ok(())
 }
 fn finish(&mut self,document:&impl DrawingSceneSource)->Result<(),DocumentSceneError>{
  if matches!(scene_layer_at(document,self.current.as_deref().unwrap())?,DrawingLayerNode::Group(group) if self.frames.len()>=self.limits.max_depth&&!group.children.is_empty()){return Err(invalid("Scene depth limit exceeded"));}
  let n=self.node.take().unwrap();
  if let DrawingLayerNode::Group(group)=scene_layer_at(document,self.current.as_deref().unwrap())?{
   let mut groups=n.groups.clone();if group.isolation||n.opacity!=1.0||n.blend_mode!="normal"{groups.push(DrawingSceneGroup{id:n.id.clone(),opacity:n.opacity,blend_mode:n.blend_mode.clone()});}
   self.frames.push(Frame{locked_ancestors:n.locked_ancestors,path:self.current.as_ref().unwrap().clone(),at:0,matrix:n.transform,visible:n.visible,groups});
  }
  self.plan.nodes.push(n);if let Some(path)=self.current.take(){self.retired_paths.push(path);}self.phase="layers";Ok(())
 }
 fn step(&mut self,document:&impl DrawingSceneSource)->Result<(),DocumentSceneError>{
  match self.phase{
   "assets"=>{
    if self.asset_active{
     let key=self.asset_key.as_ref().unwrap();let asset=document.asset(key).ok_or_else(||invalid("Captured scene asset disappeared"))?;
     if self.asset_char==asset.samples.len(){self.plan.assets.push(RasterSceneAsset{id:key.clone(),image:Arc::new(semio_framework_pixels::RasterImage{width:asset.width,height:asset.height,pixels:std::mem::take(&mut self.asset_samples)})});self.asset_ids.insert(key.clone());self.asset_active=false;self.assets+=1;return Ok(());}
     let sample=asset.samples.get(self.asset_char).ok_or_else(||invalid("Captured scene image sample changed"))?;self.asset_samples.try_reserve(4).map_err(|_|invalid("Scene sample allocation failed"))?;self.asset_samples.extend_from_slice(sample);self.asset_char+=1;self.source_bytes+=4;return Ok(());
    }
    let next=document.asset_at(self.assets);
    if let Some((key,asset))=next{id(key)?;let count=(asset.width as usize).checked_mul(asset.height as usize).ok_or_else(||invalid("Scene image extent overflow"))?;if count==0||count>16_777_216||asset.samples.len()!=count||count.checked_mul(4).is_none_or(|bytes|bytes>self.limits.max_source_bytes.saturating_sub(self.source_bytes)){return Err(invalid("Invalid intrinsic image extent or sample byte limit"));}self.asset_key=Some(scene_source_key(key)?);self.asset_active=true;self.asset_char=0;}else{self.phase="layers";}
   }
   "layers"=>{let Some(f)=self.frames.last_mut()else{self.phase="validation";return Ok(());};let count=scene_layer_count(document,&f.path)?;if f.at==count{if let Some(group)=f.groups.pop(){self.retired_groups.push(group);}else{self.retired_frames.push(self.frames.pop().unwrap());}return Ok(());}let layer=scene_child_at(document,&f.path,f.at)?;let mut path=f.path.clone();path.push(f.at);f.at+=1;let(parent,visible,groups,locks)=(f.matrix,f.visible&&(path.len()!=1||document.root_visible(path[0])),f.groups.clone(),f.locked_ancestors);self.start(layer,path,parent,visible,groups,locks)?;}
   "paint"=>self.paint(document)?,
   "geometry"=>{
    let next=match scene_layer_at(document,self.current.as_deref().unwrap())?{DrawingLayerNode::Path(p)=>p.segments.get(self.at).cloned(),DrawingLayerNode::Shape(s)=>{let absent=if s.shape_kind.eq_str("rect"){s.rect.is_none()}else if s.shape_kind.eq_str("ellipse"){s.ellipse.is_none()}else if s.shape_kind.eq_str("circle"){s.circle.is_none()}else if s.shape_kind.eq_str("line"){s.line.is_none()}else if s.shape_kind.eq_str("polygon"){s.polygon.is_none()}else{true};if absent{return Err(invalid("Missing or unknown primitive geometry"));}shape_path_segment(s,self.at)},_=>None};
    if let Some(next)=next{validate_segment(&next)?;if self.segments>=self.limits.max_segments{return Err(invalid("Scene segment limit exceeded"));}let DocumentSceneContent::Path{segments,..}=&mut self.node.as_mut().unwrap().content else{unreachable!()};segments.push(next);self.segments+=1;self.at+=1;return Ok(());}
    if let DrawingLayerNode::Text(text)=scene_layer_at(document,self.current.as_deref().unwrap())?{if self.text_at<text.content.len(){let Some(ch)=text_scalar(&text.content,&mut self.text_chunk,&mut self.text_chunk_offset)? else{return Ok(());};self.text_at+=ch.len_utf8();self.text_chars+=1;if self.text_chars>65536{return Err(invalid("Scene text exceeds limit"));}let DocumentSceneContent::Text{content,..}=&mut self.node.as_mut().unwrap().content else{unreachable!()};content.try_reserve(ch.len_utf8()).map_err(|_|invalid("Scene text allocation failed"))?;content.push(ch);return Ok(());}}
    self.finish(document)?;
   }
   "references"=>{let key=match scene_layer_at(document,self.current.as_deref().unwrap())?{DrawingLayerNode::Group(g)=>g.children.get(self.at).map(|n|&layer_base(n).id),DrawingLayerNode::Boolean(b)=>b.children.get(self.at),_=>unreachable!()};if let Some(key)=key{id(key)?;if self.references>=self.limits.max_references{return Err(invalid("Scene reference limit exceeded"));}match &mut self.node.as_mut().unwrap().content{DocumentSceneContent::Group{children,..}|DocumentSceneContent::Boolean{children,..}=>children.push(key.to_string_owner()),_=>unreachable!()}self.references+=1;self.at+=1;}else{self.finish(document)?;}}
   "validation"=>{let Some(n)=self.plan.nodes.get(self.validate_at)else{self.phase="cycles";return Ok(());};let asset=match &n.content{DocumentSceneContent::Image{asset,..}=>Some(asset),DocumentSceneContent::Trace{source,..}=>Some(source),_=>None};if asset.is_some_and(|key|!self.asset_ids.contains(key)){return Err(invalid("Missing scene source asset"));}if let Some(key)=n.content.children().get(self.ref_at){if !self.ids.contains_key(key){return Err(invalid("Missing scene operand"));}self.ref_at+=1;}else{self.validate_at+=1;self.ref_at=0;}}
   "cycles"=>{if let Some((index,at))=self.graph.last_mut(){if let Some(key)=self.plan.nodes[*index].content.children().get(*at){*at+=1;let next=self.ids[key];if self.visiting.contains(&next){return Err(invalid("Cyclic scene operands"));}if !self.visited.contains(&next){self.visiting.insert(next);self.graph.push((next,0));}}else{let(index,_)=self.graph.pop().unwrap();self.visiting.remove(&index);self.visited.insert(index);}}else if self.cycle_at<self.plan.nodes.len(){let index=self.cycle_at;self.cycle_at+=1;if !self.visited.contains(&index){self.visiting.insert(index);self.graph.push((index,0));}}else{if self.cleanup(){self.phase="complete";}}}
   _=>{}
  }Ok(())
 }
 pub fn advance(&mut self,document:&impl DrawingSceneSource,budget:usize)->Result<DocumentSceneProgress,DocumentSceneError>{if budget==0||budget as u128>9_007_199_254_740_991{return Err(invalid("Invalid scene preparation work grant"));}if self.cancelled{return Err(DocumentSceneError::Cancelled);}if let Some(error)=&self.failure{return Err(error.clone());}for _ in 0..budget{if self.phase=="complete"{break;}if let Err(error)=self.step(document){self.failure=Some(error.clone());return Err(error);}self.work+=1;}Ok(DocumentSceneProgress{phase:self.phase,layers:self.layers,assets:self.assets,segments:self.segments,references:self.references,source_bytes:self.source_bytes,work:self.work,done:self.phase=="complete"})}
 fn cleanup(&mut self)->bool{true}
 fn into_retirement(mut self)->(DocumentSceneRetirement,Option<DocumentScenePlan>){let output=if self.phase=="complete"&&!self.cancelled&&self.failure.is_none(){Some(std::mem::take(&mut self.plan))}else{None};self.cancelled=true;(DocumentSceneRetirement::new(self),output)}
 pub fn cancel(&mut self){self.cancelled=true;}
 pub fn result(&mut self)->Result<DocumentScenePlan,DocumentSceneError>{if self.cancelled{return Err(DocumentSceneError::Cancelled);}if let Some(error)=&self.failure{return Err(error.clone());}if self.phase!="complete"{return Err(DocumentSceneError::Incomplete);}Ok(std::mem::take(&mut self.plan))}
}
#[derive(semio_framework_value::RetireOwned)]
struct DocumentSceneOwners{asset_key:Option<String>,asset_samples:Vec<u8>,frames:Vec<Frame>,retired_frames:Vec<Frame>,retired_groups:Vec<DrawingSceneGroup>,retired_paths:Vec<Vec<usize>>,current:Option<Vec<usize>>,node:Option<DocumentSceneNode>,plan:DocumentScenePlan,ids:OwnedTable<String,usize>,asset_ids:OwnedSet<String>,graph:Vec<(usize,usize)>,visited:OwnedSet<usize>,visiting:OwnedSet<usize>,failure:Option<DocumentSceneError>}
impl RetireOwned for DocumentSceneCursor{fn retirement(self)->Box<dyn RetirementCursor>{DocumentSceneOwners{asset_key:self.asset_key,asset_samples:self.asset_samples,frames:self.frames,retired_frames:self.retired_frames,retired_groups:self.retired_groups,retired_paths:self.retired_paths,retired_texts:self.retired_texts,current:self.current,node:self.node,plan:self.plan,ids:self.ids,asset_ids:self.asset_ids,graph:self.graph,visited:self.visited,visiting:self.visiting,failure:self.failure}.retirement()} fn retirement_birth_bytes(&self)->Option<usize>{use semio_framework_value::retirement::{sequence_birth_bytes,deferred_birth_bytes_for};sequence_birth_bytes(&[deferred_birth_bytes_for(&self.asset_key),deferred_birth_bytes_for(&self.asset_samples),deferred_birth_bytes_for(&self.frames),deferred_birth_bytes_for(&self.retired_frames),deferred_birth_bytes_for(&self.retired_groups),deferred_birth_bytes_for(&self.retired_paths),deferred_birth_bytes_for(&self.current),deferred_birth_bytes_for(&self.node),deferred_birth_bytes_for(&self.plan),deferred_birth_bytes_for(&self.ids),deferred_birth_bytes_for(&self.asset_ids),deferred_birth_bytes_for(&self.graph),deferred_birth_bytes_for(&self.visited),deferred_birth_bytes_for(&self.visiting),deferred_birth_bytes_for(&self.failure)])} fn controlled_retirement_supported()->bool{DocumentSceneOwners::controlled_retirement_supported()}}
semio_framework_2d::physical_work_retirement!(DocumentSceneRetirement,DocumentSceneCursor,DocumentSceneError,|error:&str|invalid(error));
/// 🧭️ Owns source positions so a retained caller supplies its immutable document per work grant.
pub struct DocumentScenePreparation{cursor:DocumentSceneCursor}
impl DocumentScenePreparation{
 pub fn new(document:&impl DrawingSceneSource,limits:DocumentSceneLimits)->Result<Self,DocumentSceneError>{Ok(Self{cursor:DocumentSceneCursor::new(document,limits)?})}
 pub fn advance(&mut self,document:&impl DrawingSceneSource,budget:usize)->Result<DocumentSceneProgress,DocumentSceneError>{self.cursor.advance(document,budget)}
 pub fn cancel(&mut self){self.cursor.cancelled=true;}
 pub fn result(&mut self)->Result<DocumentScenePlan,DocumentSceneError>{self.cursor.result()}
 pub fn into_retirement(self)->(DocumentSceneRetirement,Option<DocumentScenePlan>){self.cursor.into_retirement()}
}
/// 🧱️ Borrows one stable document while retaining only source positions in its traversal cursor.
pub struct DocumentSceneJob<'a>{document:&'a DrawingSnapshot,cursor:DocumentSceneCursor}
impl<'a> DocumentSceneJob<'a>{
 pub fn new(document:&'a DrawingSnapshot,limits:DocumentSceneLimits)->Result<Self,DocumentSceneError>{Ok(Self{document,cursor:DocumentSceneCursor::new(document,limits)?})}
 pub fn advance(&mut self,budget:usize)->Result<DocumentSceneProgress,DocumentSceneError>{self.cursor.advance(self.document,budget)}
 pub fn cancel(&mut self){self.cursor.cancel();}
 pub fn result(&mut self)->Result<DocumentScenePlan,DocumentSceneError>{self.cursor.result()}
 /// 🧹️ Preserve complete output while transferring partial preparation into its retirement cursor.
 pub fn into_retirement(self)->(DocumentSceneRetirement,Option<DocumentScenePlan>){self.cursor.into_retirement()}
}
/// 🚪️ Complete prepared path/image plans enter raster; unresolved work is an explicit refusal.
pub fn resolved_scene_input(plan:DocumentScenePlan,viewport:DocumentSceneViewport)->Result<RasterSceneInput,DocumentSceneError>{
 let mut nodes=Vec::new();for n in plan.nodes{let content=match n.content{DocumentSceneContent::Group{..}=>continue,DocumentSceneContent::Path{segments,fill_rule,fill,stroke}=>RasterSceneContent::Path{segments,fill_rule,fill,stroke},DocumentSceneContent::Image{asset,width,height}=>RasterSceneContent::Image{asset,width,height},_=>return Err(invalid(format!("Unresolved scene layer: {}",n.id)))};nodes.push(RasterSceneNode{id:n.id,groups:n.groups,transform:n.transform,opacity:n.opacity,blend_mode:n.blend_mode,visible:n.visible,content});}Ok(viewport.input(plan.assets,nodes))
}
#[cfg(test)]
#[path="🧪️tests/🔬️unit/🦀️.rs"]
pub(crate) mod tests;
#[derive(Clone,Debug)]
pub struct DocumentSceneViewport{pub width:u32,pub height:u32,pub origin:[f64;2],pub tolerance:f64,pub max_pixels:usize,pub max_source_bytes:usize}
impl DocumentSceneViewport{fn input(&self,assets:Vec<RasterSceneAsset>,nodes:Vec<RasterSceneNode>)->RasterSceneInput{RasterSceneInput{width:self.width,height:self.height,origin:self.origin,tolerance:self.tolerance,max_pixels:self.max_pixels,max_source_bytes:self.max_source_bytes,assets,nodes}}}
#[derive(Clone,Debug)]
pub struct DocumentRasterProgress{pub phase:&'static str,pub preparation:DocumentSceneProgress,pub tracing:Option<DocumentTraceProgress>,pub resolution:Option<DocumentBooleanProgress>,pub raster:Option<crate::schema::scene_raster::RasterSceneProgress>,pub nodes:usize,pub work:u64,pub done:bool}
#[derive(Clone,Copy,Debug)]
pub struct DocumentAlgorithmLimits{pub booleans:DocumentBooleanLimits,pub trace:DocumentTraceLimits,pub max_work:u64}
impl Default for DocumentAlgorithmLimits{fn default()->Self{Self{max_work:1000000000,trace:DocumentTraceLimits{max_pixels:16777216,max_admitted_pixels:67108864,max_source_bytes:268439552,max_edges:65536,max_segments:65536,max_retained_segments:65536,max_work:1000000000},booleans:DocumentBooleanLimits{tolerance:0.05,epsilon:1e-8,max_depth:32,max_references:32768,max_edges:65536,max_parameters:262144,max_atomic_edges:65536,max_segments:65536,max_retained_segments:262144,max_work:1000000000}}}}
#[derive(Clone,Debug)]
pub struct DocumentVectorProgress{pub phase:&'static str,pub preparation:DocumentSceneProgress,pub tracing:Option<DocumentTraceProgress>,pub resolution:Option<DocumentBooleanProgress>,pub work:u64,pub done:bool}
impl Default for DocumentVectorProgress{fn default()->Self{Self{phase:"preparing",preparation:DocumentSceneProgress{phase:"assets",layers:0,assets:0,segments:0,references:0,source_bytes:0,work:0,done:false},tracing:None,resolution:None,work:0,done:false}}}
enum DocumentVectorSource<'a>{Borrowed(&'a DrawingSnapshot),SnapshotRead(store::SnapshotRead<DrawingSnapshot>)}
impl DocumentVectorSource<'_>{fn document(&self)->&DrawingSnapshot{match self{Self::Borrowed(document)=>document,Self::SnapshotRead(read)=>read.get()}}}
#[derive(semio_framework_value::RetireOwned)]
enum VectorChildRetirement{Preparation(DocumentSceneRetirement),Trace(DocumentTraceRetirement),Text(DocumentTextRetirement),Boolean(DocumentBooleanRetirement)}
impl VectorChildRetirement{fn advance(&mut self,grant:usize)->Result<WorkRetirementProgress,DocumentSceneError>{match self{Self::Preparation(job)=>job.advance(grant),Self::Trace(job)=>job.advance(grant),Self::Text(job)=>job.advance(grant),Self::Boolean(job)=>job.advance(grant)}}}
/// 🎬️ Resolves one stable source while retaining text and authored paint metadata.
pub struct DocumentVectorJob<'a>{
 source:Option<DocumentVectorSource<'a>>,preparation:Option<DocumentSceneCursor>,traces:Option<DocumentTraceJob>,texts:Option<DocumentTextJob>,algorithms:Option<DocumentBooleanJob>,progress:DocumentVectorProgress,
 output:Option<DocumentScenePlan>,failure:Option<DocumentSceneError>,cancelled:bool,limits:DocumentAlgorithmLimits,closing:Option<VectorChildRetirement>,discard:Option<ScenePlanCloseJob>,handoff:Option<DocumentScenePlan>,retired_children:Vec<VectorChildRetirement>,retired_plans:Vec<ScenePlanCloseJob>,
}
impl<'a> DocumentVectorJob<'a>{
 pub fn new(document:&'a DrawingSnapshot,limits:DocumentSceneLimits,algorithms:DocumentAlgorithmLimits)->Result<Self,DocumentSceneError>{Self::from_source(DocumentVectorSource::Borrowed(document),limits,algorithms)}
 fn from_source(source:DocumentVectorSource<'a>,limits:DocumentSceneLimits,algorithms:DocumentAlgorithmLimits)->Result<Self,DocumentSceneError>{let mut job=Self::from_scene_source(source.document(),limits,algorithms)?;job.source=Some(source);Ok(job)}
 /// 👁️ Stores only preparation positions while the caller retains real root and asset custody.
 pub fn from_scene_source(document:&impl DrawingSceneSource,limits:DocumentSceneLimits,algorithms:DocumentAlgorithmLimits)->Result<Self,DocumentSceneError>{
  if !(1..=1000000000).contains(&algorithms.max_work){return Err(invalid("Invalid document vector work limit"));}
  let preparation=DocumentSceneCursor::new(document,limits)?;
  let mut boolean=DocumentBooleanJob::new(DocumentBooleanInput{plan:DocumentScenePlan::default(),limits:algorithms.booleans})?;boolean.cancel();
  let mut trace=DocumentTraceJob::new(DocumentTraceInput{plan:DocumentScenePlan::default(),limits:algorithms.trace})?;trace.cancel();
  let(boolean_close,boolean_input,boolean_output)=boolean.into_retirement();let(trace_close,trace_input,trace_output)=trace.into_retirement();
  let mut retired_plans=vec![ScenePlanCloseJob::new(boolean_input),ScenePlanCloseJob::new(trace_input)];for plan in [boolean_output,trace_output].into_iter().flatten(){retired_plans.push(ScenePlanCloseJob::new(plan));}
  Ok(Self{preparation:Some(preparation),source:None,traces:None,texts:None,algorithms:None,progress:DocumentVectorProgress::default(),output:None,failure:None,cancelled:false,limits:algorithms,closing:None,discard:None,handoff:None,retired_children:vec![VectorChildRetirement::Boolean(boolean_close),VectorChildRetirement::Trace(trace_close)],retired_plans})
 }
 /// 🔐️ Owns the genuine immutable store read while preparation retains only source positions.
 pub fn from_snapshot_read(read:store::SnapshotRead<DrawingSnapshot>,limits:DocumentSceneLimits,algorithms:DocumentAlgorithmLimits)->DocumentVectorJob<'static>{
  let mut job=DocumentVectorJob{preparation:None,source:Some(DocumentVectorSource::SnapshotRead(read)),traces:None,texts:None,algorithms:None,progress:DocumentVectorProgress::default(),output:None,failure:None,cancelled:false,limits:algorithms,closing:None,discard:None,handoff:None,retired_children:Vec::new(),retired_plans:Vec::new()};
  let admitted=(||->Result<(),DocumentSceneError>{
   if !(1..=1000000000).contains(&algorithms.max_work){return Err(invalid("Invalid document vector work limit"));}
   let mut boolean=DocumentBooleanJob::new(DocumentBooleanInput{plan:DocumentScenePlan::default(),limits:algorithms.booleans})?;boolean.cancel();let(close,input,output)=boolean.into_retirement();job.retired_children.push(VectorChildRetirement::Boolean(close));job.retired_plans.push(ScenePlanCloseJob::new(input));if let Some(plan)=output{job.retired_plans.push(ScenePlanCloseJob::new(plan));}
   let mut trace=DocumentTraceJob::new(DocumentTraceInput{plan:DocumentScenePlan::default(),limits:algorithms.trace})?;trace.cancel();let(close,input,output)=trace.into_retirement();job.retired_children.push(VectorChildRetirement::Trace(close));job.retired_plans.push(ScenePlanCloseJob::new(input));if let Some(plan)=output{job.retired_plans.push(ScenePlanCloseJob::new(plan));}
   job.preparation=Some(DocumentSceneCursor::new(job.source.as_ref().unwrap().document(),limits)?);Ok(())
  })();job.failure=admitted.err();job
 }
 /// 🛂️ Checks the captured generation and revision against the store's current event authority.
 pub fn snapshot_authority_matches(&self,generation:u64,revision:[u8;32])->bool{match &self.source{Some(DocumentVectorSource::SnapshotRead(read))=>read.commit_authority_matches(generation,revision),_=>false}}
 fn step(&mut self,document:&impl DrawingSceneSource)->Result<(),DocumentSceneError>{
  if let Some(close)=self.closing.take(){self.retired_children.push(close);return Ok(());}
  if let Some(close)=self.discard.take(){self.retired_plans.push(close);return Ok(());}
  if let Some(plan)=self.handoff.take(){match self.progress.phase{"preparing"=>{self.traces=Some(DocumentTraceJob::new(DocumentTraceInput{plan,limits:self.limits.trace})?);self.progress.phase="tracing";},"tracing"=>{self.texts=Some(DocumentTextJob::new(plan,self.limits.max_work));self.progress.phase="text";},"text"=>{self.algorithms=Some(DocumentBooleanJob::new(DocumentBooleanInput{plan,limits:self.limits.booleans})?);self.progress.phase="algorithms";},_=>{self.output=Some(plan);self.progress.phase="complete";self.progress.done=true;}}return Ok(());}
  match self.progress.phase{
   "preparing"=>{self.progress.preparation=self.preparation.as_mut().unwrap().advance(document,1)?;if self.progress.preparation.done{let(close,plan)=self.preparation.take().unwrap().into_retirement();self.closing=Some(VectorChildRetirement::Preparation(close));self.handoff=plan;}}
   "tracing"=>{let p=self.traces.as_mut().unwrap().advance(1)?;self.progress.tracing=Some(p);if p.done{let(close,input,plan)=self.traces.take().unwrap().into_retirement();self.closing=Some(VectorChildRetirement::Trace(close));self.discard=Some(ScenePlanCloseJob::new(input));self.handoff=plan;}}
   "text"=>{let p=self.texts.as_mut().unwrap().advance(1)?;if p.done{let(close,plan)=self.texts.take().unwrap().into_retirement();self.closing=Some(VectorChildRetirement::Text(close));self.handoff=plan;}}
   "algorithms"=>{let p=self.algorithms.as_mut().unwrap().advance(1)?;self.progress.resolution=Some(p);if p.done{let(close,input,plan)=self.algorithms.take().unwrap().into_retirement();self.closing=Some(VectorChildRetirement::Boolean(close));self.discard=Some(ScenePlanCloseJob::new(input));self.handoff=plan;}}
   _=>{}
  }Ok(())
 }
 pub fn advance_scene_source(&mut self,document:&impl DrawingSceneSource,budget:usize)->Result<DocumentVectorProgress,DocumentSceneError>{
  if budget==0||budget as u128>9_007_199_254_740_991{return Err(invalid("Invalid document vector work grant"));}if self.cancelled{return Err(DocumentSceneError::Cancelled);}if let Some(e)=&self.failure{return Err(e.clone());}
  for _ in 0..budget{if self.progress.done{break;}let result=if self.progress.work>=self.limits.max_work{Err(invalid("Document vector work limit exceeded"))}else{self.step(document)};if let Err(e)=result{self.failure=Some(e.clone());return Err(e);}self.progress.work+=1;}Ok(self.progress.clone())
 }
 pub fn advance(&mut self,budget:usize)->Result<DocumentVectorProgress,DocumentSceneError>{let source=self.source.take().ok_or_else(||invalid("Borrowed scene producer requires its real source each turn"))?;let result=self.advance_scene_source(source.document(),budget);self.source=Some(source);result}
 /// 🧹️ Retain actual source authority while all child and intermediate plan owners drain.
 pub fn into_retirement(mut self)->(DocumentVectorRetirement<'a>,Option<DocumentScenePlan>){
  let mut children=self.retired_children;let mut child=self.closing.take();let mut plans=self.retired_plans;if let Some(discard)=self.discard.take(){plans.push(discard);}
  if let Some(preparation)=self.preparation.take(){let(close,output)=preparation.into_retirement();child=Some(VectorChildRetirement::Preparation(close));if let Some(plan)=output{plans.push(ScenePlanCloseJob::new(plan));}}
  if let Some(trace)=self.traces.take(){let(close,input,output)=trace.into_retirement();child=Some(VectorChildRetirement::Trace(close));plans.push(ScenePlanCloseJob::new(input));if let Some(plan)=output{plans.push(ScenePlanCloseJob::new(plan));}}
  if let Some(text)=self.texts.take(){let(close,output)=text.into_retirement();child=Some(VectorChildRetirement::Text(close));if let Some(plan)=output{plans.push(ScenePlanCloseJob::new(plan));}}
  if let Some(algorithms)=self.algorithms.take(){let(close,input,output)=algorithms.into_retirement();child=Some(VectorChildRetirement::Boolean(close));plans.push(ScenePlanCloseJob::new(input));if let Some(plan)=output{plans.push(ScenePlanCloseJob::new(plan));}}
  if let Some(plan)=self.handoff.take(){plans.push(ScenePlanCloseJob::new(plan));}let output=if self.progress.done&&!self.cancelled&&self.failure.is_none(){self.output.take()}else{None};if let Some(plan)=self.output.take(){plans.push(ScenePlanCloseJob::new(plan));}
  if let Some(child)=child.take(){children.push(child);}(DocumentVectorRetirement{source:self.source.take(),owner:ControlledRetirement::new(DocumentVectorOwners{children,plans,failure:self.failure}).unwrap_or_else(|(error,_)|panic!("physical document vector owner refused: {error}")),work:0},output)
 }
 pub fn cancel(&mut self){self.cancelled=true;}
 pub fn result(&mut self)->Result<DocumentScenePlan,DocumentSceneError>{if self.cancelled{return Err(DocumentSceneError::Cancelled);}if let Some(e)=&self.failure{return Err(e.clone());}if !self.progress.done{return Err(DocumentSceneError::Incomplete);}self.output.take().ok_or(DocumentSceneError::Incomplete)}
}
#[derive(semio_framework_value::RetireOwned)]
struct DocumentVectorOwners{children:Vec<VectorChildRetirement>,plans:Vec<ScenePlanCloseJob>,failure:Option<DocumentSceneError>}
/// 🔐️ Actual private byte ownership closes before the exact source read returns to its registry.
pub struct DocumentVectorRetirement<'a>{source:Option<DocumentVectorSource<'a>>,owner:ControlledRetirement<DocumentVectorOwners>,work:u64}
impl DocumentVectorRetirement<'_>{
 fn private_is_empty(&self)->bool{self.owner.terminal_is_empty()}
 pub fn terminal_is_empty(&self)->bool{self.private_is_empty()&&self.source.is_none()}
 pub fn snapshot_authority_matches(&self,generation:u64,revision:[u8;32])->bool{match &self.source{Some(DocumentVectorSource::SnapshotRead(read))=>read.commit_authority_matches(generation,revision),_=>false}}
 pub fn take_snapshot_read(&mut self)->Option<store::SnapshotRead<DrawingSnapshot>>{if !self.private_is_empty(){return None;}match self.source.take(){Some(DocumentVectorSource::SnapshotRead(read))=>Some(read),other=>{self.source=other;None}}}
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if !self.private_is_empty(){return self.owner.step(grant);}
  if grant.maximum_items>0&&grant.maximum_depth>0&&matches!(self.source,Some(DocumentVectorSource::Borrowed(_))){self.source=None;return Ok(RetainedCloneStep::Complete(semio_framework_value::retained_clone::RetainedCloneProgress{copied_items:1,..Default::default()}));}
  Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(Default::default())}else{RetainedCloneStep::Progress(Default::default())})
 }
 pub fn next_copy_byte_demand(&self)->Result<usize,ValueError>{self.owner.next_copy_byte_demand()}
 pub fn next_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{self.owner.next_capacity_byte_demand(body)}
 pub fn next_release_byte_demand(&self)->Result<usize,ValueError>{self.owner.next_release_byte_demand()}
 pub fn next_depth_demand(&self)->Result<usize,ValueError>{if self.private_is_empty(){Ok(usize::from(self.source.is_some()))}else{self.owner.next_depth_demand()}}
 pub fn advance(&mut self,items:usize)->Result<DocumentVectorRetirementProgress,DocumentSceneError>{
  if items==0||items as u128>9_007_199_254_740_991{return Err(invalid("Invalid vector retirement work grant"));}
  for _ in 0..items{if self.private_is_empty(){break;}let copy=self.next_copy_byte_demand().map_err(|error|invalid(error.to_string()))?;let release=self.next_release_byte_demand().map_err(|error|invalid(error.to_string()))?;let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:self.next_capacity_byte_demand(if copy>0{copy}else{release}).map_err(|error|invalid(error.to_string()))?,maximum_release_bytes:release,maximum_depth:self.next_depth_demand().map_err(|error|invalid(error.to_string()))?};let step=self.close_step(grant).map_err(|error|invalid(error.to_string()))?;self.work+=step.progress().copied_items as u64;}
  Ok(WorkRetirementProgress{phase:if self.private_is_empty(){"complete"}else{"closing"},work:self.work,done:self.private_is_empty()})
 }
}
#[derive(semio_framework_value::RetireOwned)]
struct BorrowedVectorOwners{preparation:Option<DocumentSceneCursor>,traces:Option<DocumentTraceJob>,texts:Option<DocumentTextJob>,algorithms:Option<DocumentBooleanJob>,output:Option<DocumentScenePlan>,failure:Option<DocumentSceneError>,closing:Option<VectorChildRetirement>,discard:Option<ScenePlanCloseJob>,handoff:Option<DocumentScenePlan>,retired_children:Vec<VectorChildRetirement>,retired_plans:Vec<ScenePlanCloseJob>}
/// 👁️ Actual scene parts are borrowed per turn while every private producer owner stays typed.
pub struct DocumentVectorPreparationJob{job:DocumentVectorJob<'static>}
impl DocumentVectorPreparationJob{
 pub fn new(source:&impl DrawingSceneSource,limits:DocumentSceneLimits,algorithms:DocumentAlgorithmLimits)->Result<Self,DocumentSceneError>{Ok(Self{job:DocumentVectorJob::from_scene_source(source,limits,algorithms)?})}
 pub fn advance(&mut self,source:&impl DrawingSceneSource,grant:usize)->Result<DocumentVectorProgress,DocumentSceneError>{self.job.advance_scene_source(source,grant)}
 pub fn cancel(&mut self){self.job.cancel();}
 pub fn result(&mut self)->Result<DocumentScenePlan,DocumentSceneError>{self.job.result()}
 pub fn into_retirement(mut self)->(DocumentVectorPreparationRetirement,Option<DocumentScenePlan>){let output=if self.job.progress.done&&!self.job.cancelled&&self.job.failure.is_none(){self.job.output.take()}else{None};self.cancel();(DocumentVectorPreparationRetirement::new(self),output)}
}
impl RetireOwned for DocumentVectorPreparationJob{
 fn retirement(self)->Box<dyn RetirementCursor>{let job=self.job;assert!(job.source.is_none(),"borrowed preparation cannot own a fabricated snapshot source");BorrowedVectorOwners{preparation:job.preparation,traces:job.traces,texts:job.texts,algorithms:job.algorithms,output:job.output,failure:job.failure,closing:job.closing,discard:job.discard,handoff:job.handoff,retired_children:job.retired_children,retired_plans:job.retired_plans}.retirement()}
 fn retirement_birth_bytes(&self)->Option<usize>{use semio_framework_value::retirement::{sequence_birth_bytes,deferred_birth_bytes_for};let job=&self.job;sequence_birth_bytes(&[deferred_birth_bytes_for(&job.preparation),deferred_birth_bytes_for(&job.traces),deferred_birth_bytes_for(&job.texts),deferred_birth_bytes_for(&job.algorithms),deferred_birth_bytes_for(&job.output),deferred_birth_bytes_for(&job.failure),deferred_birth_bytes_for(&job.closing),deferred_birth_bytes_for(&job.discard),deferred_birth_bytes_for(&job.handoff),deferred_birth_bytes_for(&job.retired_children),deferred_birth_bytes_for(&job.retired_plans)])}
 fn controlled_retirement_supported()->bool{BorrowedVectorOwners::controlled_retirement_supported()}
}
semio_framework_2d::physical_work_retirement!(DocumentVectorPreparationRetirement,DocumentVectorPreparationJob,DocumentSceneError,|error:&str|invalid(error));
/// 📷️ Resolves each raster record after the common document vector producer completes.
pub struct DocumentRasterJob<'a>{
 vector:Option<DocumentVectorJob<'a>>,prepared:DocumentVectorProgress,source:VecDeque<DocumentSceneNode>,assets:Vec<RasterSceneAsset>,nodes:Vec<RasterSceneNode>,retired_nodes:Vec<DocumentSceneNode>,retired_paths:Vec<Vec<u16>>,retired_texts:Vec<String>,at:usize,work:u64,phase:&'static str,
 raster:Option<crate::schema::scene_raster::RasterSceneJob>,rendered:Option<crate::schema::scene_raster::RasterSceneProgress>,output:Option<semio_framework_pixels::RasterImage>,failure:Option<DocumentSceneError>,cancelled:bool,viewport:DocumentSceneViewport,max_work:u64,
 vector_retirement:Option<DocumentVectorRetirement<'a>>,raster_retirement:Option<crate::schema::scene_raster::RasterSceneRetirement>,handoff:Option<DocumentScenePlan>,
 pixel_scale:[f64;2],
}
impl<'a> DocumentRasterJob<'a>{
 pub fn new(document:&'a DrawingSnapshot,limits:DocumentSceneLimits,viewport:DocumentSceneViewport,resolution_limits:DocumentAlgorithmLimits)->Result<Self,DocumentSceneError>{
  Self::from_vector(DocumentVectorJob::new(document,limits,resolution_limits)?,viewport,resolution_limits.max_work)
 }
 fn from_vector(vector:DocumentVectorJob<'a>,viewport:DocumentSceneViewport,max_work:u64)->Result<Self,DocumentSceneError>{
  let mut check=crate::schema::scene_raster::RasterSceneJob::new(viewport.input(Vec::new(),Vec::new())).map_err(|e|invalid(e.to_string()))?;check.cancel();
  Ok(Self{vector:Some(vector),prepared:DocumentVectorProgress::default(),source:VecDeque::new(),assets:Vec::new(),nodes:Vec::new(),retired_nodes:Vec::new(),retired_paths:Vec::new(),retired_texts:Vec::new(),at:0,work:0,phase:"preparing",raster:None,rendered:None,output:None,failure:None,cancelled:false,viewport,max_work,vector_retirement:None,raster_retirement:None,handoff:None,pixel_scale:[1.0;2]})
 }
 /// 🔐️ Retains the actual immutable store read through every raster and output handoff.
 pub fn from_snapshot_read(read:store::SnapshotRead<DrawingSnapshot>,limits:DocumentSceneLimits,viewport:DocumentSceneViewport,algorithms:DocumentAlgorithmLimits)->DocumentRasterJob<'static>{
  let failure=crate::schema::scene_raster::validate_scene_input(&viewport.input(Vec::new(),Vec::new())).err().map(|error|invalid(error.to_string()));
  DocumentRasterJob{vector:Some(DocumentVectorJob::from_snapshot_read(read,limits,algorithms)),prepared:DocumentVectorProgress::default(),source:VecDeque::new(),assets:Vec::new(),nodes:Vec::new(),retired_nodes:Vec::new(),retired_paths:Vec::new(),retired_texts:Vec::new(),at:0,work:0,phase:"preparing",raster:None,rendered:None,output:None,failure,cancelled:false,viewport,max_work:algorithms.max_work,vector_retirement:None,raster_retirement:None,handoff:None,pixel_scale:[1.0;2]}
 }
 /// 🔍️ Maps the immutable artboard extent to the exact requested pixel extent.
 pub fn set_pixel_scale(&mut self,x:f64,y:f64)->Result<(),DocumentSceneError>{if self.work!=0||![x,y].into_iter().all(|n|n.is_finite()&&n>0.0){return Err(invalid("Invalid document pixel scale"));}self.pixel_scale=[x,y];Ok(())}
 fn step(&mut self)->Result<(),DocumentSceneError>{
  if self.vector_retirement.is_some(){if let Some(plan)=self.handoff.take(){self.source=plan.nodes.into();self.assets=plan.assets;return Ok(());}}
  
  if let Some(vector)=&mut self.vector{self.prepared=vector.advance(1)?;if self.prepared.done{let(close,plan)=self.vector.take().unwrap().into_retirement();self.vector_retirement=Some(close);self.handoff=plan;self.phase="resolving";}else{self.phase=self.prepared.phase;}return Ok(());}
  match self.phase{
   "resolving"=>{if let Some(n)=self.source.pop_front(){self.at+=1;if !n.visible||matches!(&n.content,DocumentSceneContent::Group{..}){self.retired_nodes.push(n);return Ok(());}if !matches!(&n.content,DocumentSceneContent::Path{..}|DocumentSceneContent::Glyphs{..}|DocumentSceneContent::Image{..}){let error=invalid(format!("Unresolved scene layer: {}",n.id));self.retired_nodes.push(n);return Err(error);}let content=match n.content{DocumentSceneContent::Path{segments,fill_rule,fill,stroke}=>RasterSceneContent::Path{segments,fill_rule,fill,stroke},DocumentSceneContent::Glyphs{content,segments,fill_rule,fill,stroke,..}=>{self.retired_texts.push(content);RasterSceneContent::Path{segments,fill_rule,fill,stroke}},DocumentSceneContent::Image{asset,width,height}=>RasterSceneContent::Image{asset,width,height},_=>unreachable!()};self.retired_paths.push(n.source_path);let m=n.transform;let [x,y]=self.pixel_scale;self.nodes.push(RasterSceneNode{id:n.id,groups:n.groups,transform:[m[0]*x,m[1]*y,m[2]*x,m[3]*y,m[4]*x,m[5]*y],opacity:n.opacity,blend_mode:n.blend_mode,visible:n.visible,content});}else{self.raster=Some(crate::schema::scene_raster::RasterSceneJob::new(self.viewport.input(std::mem::take(&mut self.assets),std::mem::take(&mut self.nodes))).map_err(|e|invalid(e.to_string()))?);self.phase="raster";}}
   "raster"=>{let p=self.raster.as_mut().unwrap().advance(1).map_err(|e|invalid(e.to_string()))?;let done=p.done;self.rendered=Some(p);if done{let(close,image)=self.raster.take().unwrap().into_retirement();self.output=image;self.raster_retirement=Some(close);self.phase="complete";}}
   _=>{}
  }Ok(())
 }
 pub fn advance(&mut self,budget:usize)->Result<DocumentRasterProgress,DocumentSceneError>{if budget==0||budget as u128>9_007_199_254_740_991{return Err(invalid("Invalid document raster work grant"));}if self.cancelled{return Err(DocumentSceneError::Cancelled);}if let Some(e)=&self.failure{return Err(e.clone());}for _ in 0..budget{if self.phase=="complete"{break;}let result=if self.work>=self.max_work{Err(invalid("Document raster work limit exceeded"))}else{self.step()};if let Err(e)=result{self.failure=Some(e.clone());return Err(e);}self.work+=1;}Ok(DocumentRasterProgress{phase:self.phase,preparation:self.prepared.preparation.clone(),tracing:self.prepared.tracing,resolution:self.prepared.resolution,raster:self.rendered.clone(),nodes:self.at,work:self.work,done:self.phase=="complete"})}
 pub fn cancel(&mut self){self.cancelled=true;}
 pub fn result(&mut self)->Result<semio_framework_pixels::RasterImage,DocumentSceneError>{if self.cancelled{return Err(DocumentSceneError::Cancelled);}if let Some(e)=&self.failure{return Err(e.clone());}if self.phase!="complete"{return Err(DocumentSceneError::Incomplete);}self.output.take().ok_or(DocumentSceneError::Incomplete)}
 /// 🛂️ Verifies the actual retained source against the current store event authority.
 pub fn snapshot_authority_matches(&self,generation:u64,revision:[u8;32])->bool{if let Some(vector)=&self.vector{return vector.snapshot_authority_matches(generation,revision);}self.vector_retirement.as_ref().is_some_and(|close|close.snapshot_authority_matches(generation,revision))}
 /// 🧹️ Adopts every real child and source lease before relinquishing private raster ownership.
 pub fn into_retirement(mut self)->(DocumentRasterRetirement<'a>,Option<semio_framework_pixels::RasterImage>){
  let output=if self.phase=="complete"&&!self.cancelled&&self.failure.is_none(){self.output.take()}else{None};self.cancelled=true;
  if let Some(vector)=self.vector.take(){let(close,plan)=vector.into_retirement();self.vector_retirement=Some(close);self.handoff=plan;}
  if let Some(raster)=self.raster.take(){let(close,image)=raster.into_retirement();self.raster_retirement=Some(close);if let Some(image)=image{self.output=Some(image);}}
  let owners=DocumentRasterOwners{source:self.source,assets:self.assets,nodes:self.nodes,retired_nodes:self.retired_nodes,retired_paths:self.retired_paths,retired_texts:self.retired_texts,raster:self.raster_retirement,handoff:self.handoff,output:self.output,failure:self.failure};
  (DocumentRasterRetirement{vector:self.vector_retirement,owner:ControlledRetirement::new(owners).unwrap_or_else(|(error,_)|panic!("physical document raster owner refused: {error}")),work:0},output)
 }
}
/// 🧺️ Source handback follows actual vector, raster and private candidate retirement.
#[derive(semio_framework_value::RetireOwned)]
struct DocumentRasterOwners{source:VecDeque<DocumentSceneNode>,assets:Vec<RasterSceneAsset>,nodes:Vec<RasterSceneNode>,retired_nodes:Vec<DocumentSceneNode>,retired_paths:Vec<Vec<u16>>,retired_texts:Vec<String>,raster:Option<crate::schema::scene_raster::RasterSceneRetirement>,handoff:Option<DocumentScenePlan>,output:Option<semio_framework_pixels::RasterImage>,failure:Option<DocumentSceneError>}
pub struct DocumentRasterRetirement<'a>{vector:Option<DocumentVectorRetirement<'a>>,owner:ControlledRetirement<DocumentRasterOwners>,work:u64}
impl DocumentRasterRetirement<'_>{
 pub fn terminal_is_empty(&self)->bool{self.owner.terminal_is_empty()&&self.vector.as_ref().is_none_or(DocumentVectorRetirement::terminal_is_empty)}
 pub fn take_snapshot_read(&mut self)->Option<store::SnapshotRead<DrawingSnapshot>>{if !self.owner.terminal_is_empty(){return None;}self.vector.as_mut()?.take_snapshot_read()}
 pub fn snapshot_authority_matches(&self,generation:u64,revision:[u8;32])->bool{self.vector.as_ref().is_some_and(|close|close.snapshot_authority_matches(generation,revision))}
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  let step=if !self.owner.terminal_is_empty(){self.owner.step(grant)?}else if let Some(vector)=&mut self.vector{vector.close_step(grant)?}else{RetainedCloneStep::Complete(Default::default())};
  let progress=step.progress();Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})
 }
 pub fn next_copy_byte_demand(&self)->Result<usize,ValueError>{if !self.owner.terminal_is_empty(){self.owner.next_copy_byte_demand()}else{self.vector.as_ref().map_or(Ok(0),DocumentVectorRetirement::next_copy_byte_demand)}}
 pub fn next_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{if !self.owner.terminal_is_empty(){self.owner.next_capacity_byte_demand(body)}else{self.vector.as_ref().map_or(Ok(0),|vector|vector.next_capacity_byte_demand(body))}}
 pub fn next_release_byte_demand(&self)->Result<usize,ValueError>{if !self.owner.terminal_is_empty(){self.owner.next_release_byte_demand()}else{self.vector.as_ref().map_or(Ok(0),DocumentVectorRetirement::next_release_byte_demand)}}
 pub fn next_depth_demand(&self)->Result<usize,ValueError>{if !self.owner.terminal_is_empty(){self.owner.next_depth_demand()}else{self.vector.as_ref().map_or(Ok(0),DocumentVectorRetirement::next_depth_demand)}}
 pub fn advance(&mut self,items:usize)->Result<WorkRetirementProgress,DocumentSceneError>{
  if items==0||items as u128>9_007_199_254_740_991{return Err(invalid("Invalid document raster retirement work grant"));}
  for _ in 0..items{if self.terminal_is_empty(){break;}let copy=self.next_copy_byte_demand().map_err(|error|invalid(error.to_string()))?;let release=self.next_release_byte_demand().map_err(|error|invalid(error.to_string()))?;let step=self.close_step(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:self.next_capacity_byte_demand(if copy>0{copy}else{release}).map_err(|error|invalid(error.to_string()))?,maximum_release_bytes:release,maximum_depth:self.next_depth_demand().map_err(|error|invalid(error.to_string()))?}).map_err(|error|invalid(error.to_string()))?;self.work+=step.progress().copied_items as u64;}
  Ok(WorkRetirementProgress{phase:if self.terminal_is_empty(){"complete"}else{"closing"},work:self.work,done:self.terminal_is_empty()})
 }
}
