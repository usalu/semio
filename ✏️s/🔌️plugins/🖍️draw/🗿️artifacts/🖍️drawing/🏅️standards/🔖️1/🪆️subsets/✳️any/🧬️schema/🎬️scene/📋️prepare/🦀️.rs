//! 📋️ Complete authored documents become private typed plans under work grants.
use semio_framework_value::paged::Utf8Text;
use semio_framework_value_persistent::work::RetainedWorkLedger;
use semio_framework_value::RetirementDemand;
use semio_framework_job::{StepContext,OriginalWorkFailure};
use semio_s_artifact_draw_drawing_model::fill_rule::FillRule;
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;
use semio_s_artifact_draw_drawing_model::{DrawingLayerNode, FillStyle, StrokeStyle, PathSegment};
use semio_s_artifact_draw_drawing_model::{layer_base, DrawingSceneGroup};
use semio_s_artifact_draw_drawing_geometry::shape_path_segment;
use semio_s_artifact_draw_drawing_model::affine::drawing_transform_to_matrix;
use crate::raster::{RasterSceneAsset, RasterSceneNode, RasterSceneContent, RasterSceneInput};
use crate::retirement::ScenePlanCloseJob;
use semio_framework_2d::retirement::WorkRetirementProgress;
pub type DocumentSceneRetirementProgress=WorkRetirementProgress;
pub type DocumentVectorRetirementProgress=WorkRetirementProgress;
use std::{collections::VecDeque,sync::Arc};
use semio_framework_value::retirement::{RetireOwned,RetirementCursor,controlled::ControlledRetirement};
use semio_framework_value::{ValueError,retained_clone::{RetainedCloneGrant,RetainedCloneStep}};
#[derive(Clone,Debug,semio_framework_value::RetireOwned)]
pub struct OwnedTable<K:RetireOwned+Eq,V:RetireOwned>{values:Vec<(K,V)>,retired_keys:Vec<K>,retired_values:Vec<V>}
impl<K:RetireOwned+Eq,V:RetireOwned> OwnedTable<K,V>{
 pub fn new()->Self{Self{values:Vec::new(),retired_keys:Vec::new(),retired_values:Vec::new()}}
 pub fn get<Q:Eq+?Sized>(&self,key:&Q)->Option<&V>where K:std::borrow::Borrow<Q>{self.values.iter().find(|(id,_)|id.borrow()==key).map(|(_,value)|value)}
 pub fn contains_key<Q:Eq+?Sized>(&self,key:&Q)->bool where K:std::borrow::Borrow<Q>{self.get(key).is_some()}
 pub fn insert(&mut self,key:K,value:V){if let Some(at)=self.values.iter().position(|(id,_)|id==&key){self.retired_keys.push(key);self.retired_values.push(std::mem::replace(&mut self.values[at].1,value));}else{self.values.push((key,value));}}
 pub fn remove<Q:Eq+?Sized>(&mut self,key:&Q)->Option<V>where K:std::borrow::Borrow<Q>{let at=self.values.iter().position(|(id,_)|id.borrow()==key)?;let(key,value)=self.values.swap_remove(at);self.retired_keys.push(key);Some(value)}
}
impl<K:RetireOwned+Eq+std::borrow::Borrow<Q>,V:RetireOwned,Q:Eq+?Sized> std::ops::Index<&Q> for OwnedTable<K,V>{type Output=V;fn index(&self,key:&Q)->&V{self.get(key).expect("admitted scene key")}}
#[derive(Clone,Debug,semio_framework_value::RetireOwned)]
pub struct OwnedSet<T:RetireOwned+Eq>{values:Vec<T>,retired:Vec<T>,generations:Vec<Vec<T>>}
impl<T:RetireOwned+Eq> OwnedSet<T>{
 pub fn new()->Self{Self{values:Vec::new(),retired:Vec::new(),generations:Vec::new()}}
 pub fn contains(&self,value:&T)->bool{self.values.contains(value)}
 pub fn insert(&mut self,value:T)->bool{if self.contains(&value){self.retired.push(value);false}else{self.values.push(value);true}}
 pub fn remove(&mut self,value:&T)->bool{if let Some(at)=self.values.iter().position(|item|item==value){self.retired.push(self.values.swap_remove(at));true}else{false}}
 pub fn reset(&mut self){self.generations.push(std::mem::take(&mut self.values));}
}
#[path="🧭️selection/🦀️.rs"]
pub mod selection;
pub use selection::{scene_selection_relation,SceneSelectionRelation};
use crate::booleans::{DocumentBooleanInput, DocumentBooleanJob, DocumentBooleanProgress, DocumentBooleanRetirement};
use crate::text::{DocumentTextJob, DocumentTextRetirement};
use crate::trace::{DocumentTraceInput, DocumentTraceJob, DocumentTraceProgress, DocumentTraceRetirement};
use crate::booleans::DocumentBooleanLimits;
use crate::trace::DocumentTraceLimits;
#[derive(Clone,Copy,Debug)]
pub struct DocumentSceneLimits{pub max_nodes:usize,pub max_depth:usize,pub max_segments:usize,pub max_references:usize,pub max_source_bytes:usize}
impl Default for DocumentSceneLimits{fn default()->Self{Self{max_nodes:1024,max_depth:32,max_segments:65536,max_references:32768,max_source_bytes:268439552}}}
#[derive(Clone,Debug,semio_framework_value::RetireOwned)]
pub enum DocumentSceneContent{
 Path{segments:Vec<PathSegment>,fill_rule:FillRule,fill:Option<FillStyle>,stroke:Option<StrokeStyle>},
 Image{asset:String,width:f64,height:f64},Group{children:Vec<String>,isolation:bool},
 Text{content:String,x:f64,y:f64,size:f64,font_family:semio_s_artifact_draw_drawing_model::font_family::DrawingFontFamily,fill_rule:FillRule,fill:Option<FillStyle>,stroke:Option<StrokeStyle>},
 Glyphs{content:String,x:f64,y:f64,size:f64,font_family:semio_s_artifact_draw_drawing_model::font_family::DrawingFontFamily,segments:Vec<PathSegment>,fill_rule:FillRule,fill:Option<FillStyle>,stroke:Option<StrokeStyle>},
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
pub use semio_s_artifact_draw_drawing_scene_support::error::{DocumentSceneError, DocumentSceneMessage};

fn invalid(message:impl Into<DocumentSceneMessage>)->DocumentSceneError{DocumentSceneError::Invalid(message.into())}
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
fn matrix(a:[f64;6],b:[f64;6])->Result<[f64;6],DocumentSceneError>{let mut m=semio_s_artifact_draw_drawing_geometry::multiply(a,b);for n in &mut m{coordinate(*n)?;if *n==0.0{*n=0.0;}}Ok(m)}
fn validate_segment(s:&PathSegment)->Result<(),DocumentSceneError>{let p=|v:[f64;2]|{coordinate(v[0])?;coordinate(v[1])?;Ok::<(),DocumentSceneError>(())};match s{PathSegment::Move{to}|PathSegment::Line{to}=>p(*to)?,PathSegment::Quad{ctrl,to}=>{p(*ctrl)?;p(*to)?},PathSegment::Cubic{ctrl1,ctrl2,to}=>{p(*ctrl1)?;p(*ctrl2)?;p(*to)?},PathSegment::Arc{rx,ry,rotation,to,..}=>{coordinate(*rx)?;coordinate(*ry)?;coordinate(*rotation)?;p(*to)?},PathSegment::Close=>{}}Ok(())}
/// 👁️ Scene preparation borrows actual root and asset owners separately in each turn.
pub trait DrawingSceneSource{
 fn root_count(&self)->usize;
 fn root(&self,index:usize)->Option<&DrawingLayerNode>;
 fn asset_count(&self)->usize;
 fn asset_at(&self,index:usize)->Option<(&dyn Utf8Text,&semio_s_artifact_draw_drawing_model::DrawingImageAsset)>;
 fn asset(&self,key:&str)->Option<&semio_s_artifact_draw_drawing_model::DrawingImageAsset>;
 fn root_visible(&self,_index:usize)->bool{true}
}
impl DrawingSceneSource for DrawingSnapshot{
 fn root_count(&self)->usize{self.layers.len()}
 fn root(&self,index:usize)->Option<&DrawingLayerNode>{self.layers.get(index)}
 fn asset_count(&self)->usize{self.assets.len()}
 fn asset_at(&self,index:usize)->Option<(&dyn Utf8Text,&semio_s_artifact_draw_drawing_model::DrawingImageAsset)>{self.assets.retained_entries().get(index).map(|(key,asset)|(key as &dyn Utf8Text,asset))}
 fn asset(&self,key:&str)->Option<&semio_s_artifact_draw_drawing_model::DrawingImageAsset>{self.assets.get(key)}
}
fn scene_layer_at<'d>(document:&'d impl DrawingSceneSource,path:&[usize])->Result<&'d DrawingLayerNode,DocumentSceneError>{let mut node=document.root(*path.first().ok_or_else(||invalid("Missing scene cursor path"))?).ok_or_else(||invalid("Captured scene root disappeared"))?;for at in &path[1..]{let DrawingLayerNode::Group(group)=node else{return Err(invalid("Captured scene group changed"));};node=group.children.get(*at).ok_or_else(||invalid("Captured scene layer disappeared"))?;}Ok(node)}
fn scene_layer_count(document:&impl DrawingSceneSource,path:&[usize])->Result<usize,DocumentSceneError>{if path.is_empty(){return Ok(document.root_count());}let DrawingLayerNode::Group(group)=scene_layer_at(document,path)? else{return Err(invalid("Captured scene group changed"));};Ok(group.children.len())}
fn scene_child_at<'d>(document:&'d impl DrawingSceneSource,path:&[usize],index:usize)->Result<&'d DrawingLayerNode,DocumentSceneError>{if path.is_empty(){return document.root(index).ok_or_else(||invalid("Captured scene root disappeared"));}let DrawingLayerNode::Group(group)=scene_layer_at(document,path)? else{return Err(invalid("Captured scene group changed"));};group.children.get(index).ok_or_else(||invalid("Captured scene layer disappeared"))}
fn scene_source_key(key:&dyn Utf8Text)->Result<String,DocumentSceneError>{id(key)?;let mut result=String::with_capacity(key.text_bytes());for at in 0..key.text_chunk_count(){result.push_str(key.text_chunk(at).ok_or_else(||invalid("Captured asset key changed"))?);}Ok(result)}
#[derive(Clone,Copy,semio_framework_value::RetireOwned)]
struct Frame{path:[usize;32],depth:usize,at:usize,matrix:[f64;6],visible:bool,locks:u32,groups:[usize;32],group_count:usize}
impl Frame{const EMPTY:Self=Self{path:[0;32],depth:0,at:0,matrix:[1.0,0.0,0.0,1.0,0.0,0.0],visible:true,locks:0,groups:[0;32],group_count:0};}
/// 🧱️ Inline bounded traversal retains only the output payloads admitted by the original work ledger.
struct DocumentSceneCursor{
 limits:DocumentSceneLimits,frames:[Frame;33],frame_count:usize,path:[usize;32],depth:usize,parent:Frame,node:Option<DocumentSceneNode>,plan:DocumentScenePlan,
 asset_samples:Vec<u8>,asset_id:String,reference:String,group:Option<DrawingSceneGroup>,asset_char:usize,text_chunk:usize,
 phase:&'static str,layers:usize,assets:usize,segments:usize,references:usize,source_bytes:usize,work:u64,last_turn_work:u64,last_turn_semantic_work:u64,step_work:u64,pending:semio_framework_value::RetirementDemand,
 at:usize,stroke_at:usize,text_at:usize,text_chars:usize,text_chunk_offset:usize,group_at:usize,
 validate_at:usize,ref_at:usize,cycle_at:usize,graph:[(usize,usize);1024],graph_len:usize,visited:[bool;1024],visiting:[bool;1024],cancelled:bool,failure:Option<DocumentSceneError>,
}
fn preparation_invalid(message:&'static str)->DocumentSceneError{DocumentSceneError::literal(message)}
fn scene_vector_slot<T>(values:&mut Vec<T>,work:&mut semio_framework_2d_text_font::work::RetainedWorkLedger)->Result<bool,DocumentSceneError>{
 let target=if values.len()==values.capacity(){values.capacity().checked_mul(2).unwrap_or(usize::MAX).max(16)}else{values.len()+1};if !work.reserve_vec(values,target).map_err(preparation_invalid)?{return Ok(false);}work.vec_slot(values).map_err(preparation_invalid)
}
fn scene_text_copy(source:&(impl Utf8Text+?Sized),target:&mut String,chunk:&mut usize,work:&mut semio_framework_2d_text_font::work::RetainedWorkLedger)->Result<(bool,bool),DocumentSceneError>{
 if !work.reserve_string(target,source.text_bytes()).map_err(preparation_invalid)?{if target.capacity()>=source.text_bytes(){work.require_child(semio_framework_value::RetirementDemand{copy_bytes:source.text_chunk(*chunk).map_or(0,str::len),depth:1,..Default::default()}).map_err(preparation_invalid)?;}return Ok((false,false));}
 if let Some(part)=source.text_chunk(*chunk){if !work.string_push(target,part).map_err(preparation_invalid)?{return Ok((false,false));}*chunk+=1;return Ok((false,true));}Ok((true,false))
}
impl DocumentSceneCursor{
 pub fn new(document:&impl DrawingSceneSource,limits:DocumentSceneLimits)->Result<Self,DocumentSceneError>{
  if !(1..=1024).contains(&limits.max_nodes)||!(1..=32).contains(&limits.max_depth)||!(1..=65536).contains(&limits.max_segments)||!(1..=32768).contains(&limits.max_references)||!(1..=268439552).contains(&limits.max_source_bytes)||document.root_count()>limits.max_nodes||document.asset_count()>1024{return Err(preparation_invalid("Invalid scene preparation limits or document"));}
  Ok(Self{limits,frames:[Frame::EMPTY;33],frame_count:1,path:[0;32],depth:0,parent:Frame::EMPTY,node:None,plan:DocumentScenePlan::default(),asset_samples:Vec::new(),asset_id:String::new(),reference:String::new(),group:None,asset_char:0,text_chunk:0,phase:"assets",layers:0,assets:0,segments:0,references:0,source_bytes:0,work:0,last_turn_work:0,last_turn_semantic_work:0,step_work:0,pending:semio_framework_value::RetirementDemand{depth:1,..Default::default()},at:0,stroke_at:0,text_at:0,text_chars:0,text_chunk_offset:0,group_at:0,validate_at:0,ref_at:0,cycle_at:0,graph:[(0,0);1024],graph_len:0,visited:[false;1024],visiting:[false;1024],cancelled:false,failure:None})
 }
 fn layer<'d>(&self,document:&'d impl DrawingSceneSource)->Result<&'d DrawingLayerNode,DocumentSceneError>{scene_layer_at(document,&self.path[..self.depth])}
 fn transition(&mut self,phase:&'static str){self.phase=phase;self.text_chunk=0;}
 fn begin(&mut self,document:&impl DrawingSceneSource)->Result<(),DocumentSceneError>{
  let layer=self.layer(document)?;let b=layer_base(layer);id(&b.id)?;
  if self.layers>=self.limits.max_nodes||self.plan.nodes.iter().any(|node|b.id.eq_str(&node.id)){return Err(preparation_invalid("Duplicate scene identity or layer limit"));}
  for n in [b.transform.x,b.transform.y,b.transform.scale_x,b.transform.scale_y,b.transform.shear,b.transform.rotation]{coordinate(n)?;}
  let transform=matrix(self.parent.matrix,drawing_transform_to_matrix(&b.transform))?;let opacity=unit(b.opacity)?;
  if !semio_s_artifact_draw_drawing_model::DRAWING_BLEND_MODES.iter().any(|mode|b.blend_mode.eq_str(mode)){return Err(preparation_invalid("Invalid authored scene blend"));}
  let rule=b.attributes.fill_rule.clone();let content=match layer{
   DrawingLayerNode::Path(_)|DrawingLayerNode::Shape(_)=>DocumentSceneContent::Path{segments:Vec::new(),fill_rule:rule,fill:None,stroke:None},
   DrawingLayerNode::Image(v)=>{id(&v.image_key)?;DocumentSceneContent::Image{asset:String::new(),width:positive(v.width)?,height:positive(v.height)?}},
   DrawingLayerNode::Group(v)=>{if v.children.len()>1024{return Err(preparation_invalid("Invalid scene group"));}DocumentSceneContent::Group{children:Vec::new(),isolation:v.isolation}},
   DrawingLayerNode::Text(v)=>{if v.content.len()>262144{return Err(preparation_invalid("Scene text exceeds limit"));}DocumentSceneContent::Text{content:String::new(),x:coordinate(v.x)?,y:coordinate(v.y)?,size:positive(v.size)?,font_family:v.font_family,fill_rule:rule,fill:None,stroke:None}},
   DrawingLayerNode::Boolean(v)=>{if !semio_s_artifact_draw_drawing_model::DRAWING_BOOLEAN_OPERATIONS.iter().any(|operation|v.operation.eq_str(operation))||v.children.len()>1024{return Err(preparation_invalid("Invalid boolean scene work"));}DocumentSceneContent::Boolean{operation:String::new(),children:Vec::new(),reference_transform:self.parent.matrix,fill_rule:rule,fill:None,stroke:None}},
   DrawingLayerNode::Trace(v)=>{id(&v.source_key)?;let epsilon=coordinate(v.params.simplify_epsilon)?;if epsilon<0.0{return Err(preparation_invalid("Invalid trace epsilon"));}DocumentSceneContent::Trace{source:String::new(),threshold:unit(v.params.threshold)?,simplify_epsilon:epsilon,fill_rule:rule,fill:None,stroke:None}},
  };
  self.node=Some(DocumentSceneNode{source_path:Vec::new(),locked_ancestors:self.parent.locks|if b.locked{1u32<<(self.depth-1)}else{0},id:String::new(),groups:Vec::new(),transform,opacity,blend_mode:String::new(),visible:self.parent.visible&&b.visible,content});self.layers+=1;self.at=0;self.stroke_at=0;self.text_at=0;self.text_chars=0;self.text_chunk_offset=0;self.group_at=0;self.transition("nodeIdentity");Ok(())
 }
 fn step(&mut self,document:&impl DrawingSceneSource,work:&mut semio_framework_2d_text_font::work::RetainedWorkLedger)->Result<bool,DocumentSceneError>{
  match self.phase{
   "assets"=>{
    let Some((key,asset))=document.asset_at(self.assets)else{if work.can_step(){self.transition("layers");}return Ok(true);};
    id(key)?;let count=(asset.width as usize).checked_mul(asset.height as usize).ok_or_else(||preparation_invalid("Scene image extent overflow"))?;
    if count==0||count>16_777_216||asset.samples.len()!=count||count.checked_mul(4).is_none_or(|bytes|bytes>self.limits.max_source_bytes.saturating_sub(self.source_bytes)){return Err(preparation_invalid("Invalid intrinsic image extent or sample byte limit"));}
    let(complete,semantic)=scene_text_copy(key,&mut self.asset_id,&mut self.text_chunk,work)?;if !complete{return Ok(semantic);}self.asset_char=0;self.transition("samples");
   },
   "samples"=>{
    let (_,asset)=document.asset_at(self.assets).ok_or_else(||preparation_invalid("Captured scene asset disappeared"))?;
    if let Some(sample)=asset.samples.get(self.asset_char){if !work.reserve_vec(&mut self.asset_samples,(self.asset_char+1)*4).map_err(preparation_invalid)?{return Ok(false);}if !work.copy_bytes(4){return Ok(false);}self.asset_samples.extend_from_slice(sample);self.asset_char+=1;self.source_bytes+=4;return Ok(true);}
    if !scene_vector_slot(&mut self.plan.assets,work)?||!work.arc_ready::<semio_framework_pixels::RasterImage>().map_err(preparation_invalid)?{return Ok(false);}
    let image=work.arc_new(semio_framework_pixels::RasterImage{width:asset.width,height:asset.height,pixels:std::mem::take(&mut self.asset_samples)}).map_err(|(error,_)|preparation_invalid(error))?;
    work.vec_push(&mut self.plan.assets,RasterSceneAsset{id:std::mem::take(&mut self.asset_id),image}).map_err(|(error,_)|preparation_invalid(error))?;self.assets+=1;self.transition("assets");
   },
   "layers"=>{
    if self.frame_count==0{if work.can_step(){self.transition("validation");}return Ok(true);}
    let f=self.frames[self.frame_count-1];let count=scene_layer_count(document,&f.path[..f.depth])?;
    if f.at==count{if work.can_step(){self.frame_count-=1;}return Ok(true);}
    if f.depth>=self.limits.max_depth{return Err(preparation_invalid("Scene depth limit exceeded"));}
    if !work.can_step(){return Ok(false);}
    self.parent=f;self.path=f.path;self.path[f.depth]=f.at;self.depth=f.depth+1;self.frames[self.frame_count-1].at+=1;
    if self.depth==1{self.parent.visible&=document.root_visible(f.at);}
    self.begin(document)?;
   },
   "nodeIdentity"|"nodeBlend"|"nodeContent"=>{
    let layer=self.layer(document)?;let b=layer_base(layer);let node=self.node.as_mut().ok_or_else(DocumentSceneError::held)?;let phase=self.phase;
    let(complete,semantic)=match phase{
     "nodeIdentity"=>scene_text_copy(&b.id,&mut node.id,&mut self.text_chunk,work)?,
     "nodeBlend"=>scene_text_copy(&b.blend_mode,&mut node.blend_mode,&mut self.text_chunk,work)?,
     _=>match (layer,&mut node.content){(DrawingLayerNode::Image(v),DocumentSceneContent::Image{asset,..})=>scene_text_copy(&v.image_key,asset,&mut self.text_chunk,work)?,(DrawingLayerNode::Boolean(v),DocumentSceneContent::Boolean{operation,..})=>scene_text_copy(&v.operation,operation,&mut self.text_chunk,work)?,(DrawingLayerNode::Trace(v),DocumentSceneContent::Trace{source,..})=>scene_text_copy(&v.source_key,source,&mut self.text_chunk,work)?,_=>(true,false)}
    };if !complete{return Ok(semantic);}if complete&&work.can_step(){self.transition(match phase{"nodeIdentity"=>"nodeBlend","nodeBlend"=>"nodeContent",_=>"nodeAddress"});}
   },
   "nodeAddress"=>{
    let node=self.node.as_mut().ok_or_else(DocumentSceneError::held)?;if let Some(index)=self.path.get(node.source_path.len()).filter(|_|node.source_path.len()<self.depth){let value=u16::try_from(*index).map_err(|_|preparation_invalid("Scene source index exceeds limit"))?;if !scene_vector_slot(&mut node.source_path,work)?{return Ok(false);}work.vec_push(&mut node.source_path,value).map_err(|(error,_)|preparation_invalid(error))?;}else if work.can_step(){self.transition("nodeGroups");}
   },
   "nodeGroups"=>{
    if self.group_at==self.parent.group_count{if work.can_step(){self.transition("paint");}return Ok(true);}
    if self.group.is_none(){if !work.can_step(){return Ok(false);}let source=&self.plan.nodes[self.parent.groups[self.group_at]];self.group=Some(DrawingSceneGroup{id:String::new(),opacity:source.opacity,blend_mode:String::new()});self.transition("groupIdentity");}
   },
   "groupIdentity"|"groupBlend"=>{
    let source=&self.plan.nodes[self.parent.groups[self.group_at]];let group=self.group.as_mut().ok_or_else(DocumentSceneError::held)?;let phase=self.phase;
    let(complete,semantic)=if phase=="groupIdentity"{scene_text_copy(&source.id,&mut group.id,&mut self.text_chunk,work)?}else{scene_text_copy(&source.blend_mode,&mut group.blend_mode,&mut self.text_chunk,work)?};
    if !complete{return Ok(semantic);}if complete{if phase=="groupIdentity"{if work.can_step(){self.transition("groupBlend");}}else{let groups=&mut self.node.as_mut().ok_or_else(DocumentSceneError::held)?.groups;if !scene_vector_slot(groups,work)?{return Ok(false);}work.vec_push(groups,self.group.take().ok_or_else(DocumentSceneError::held)?).map_err(|(error,_)|preparation_invalid(error))?;self.group_at+=1;self.transition("nodeGroups");}}
   },
   "paint"=>{
    let b=layer_base(self.layer(document)?);let Some((out_fill,out_stroke))=self.node.as_mut().ok_or_else(DocumentSceneError::held)?.content.paint()else{if work.can_step(){self.transition(if matches!(self.layer(document)?,DrawingLayerNode::Group(_)|DrawingLayerNode::Boolean(_)){"references"}else{"geometry"});}return Ok(true);};
    let source=b.attributes.fill.as_ref();
    if self.at==0{
     if !work.can_step(){return Ok(false);}
     *out_fill=match source{None=>None,Some(FillStyle::Solid{color:c})=>Some(FillStyle::Solid{color:color(*c)?}),Some(FillStyle::LinearGradient{x1,y1,x2,y2,stops})=>{if stops.is_empty()||stops.len()>64{return Err(preparation_invalid("Invalid scene gradient stops"));}Some(FillStyle::LinearGradient{x1:coordinate(*x1)?,y1:coordinate(*y1)?,x2:coordinate(*x2)?,y2:coordinate(*y2)?,stops:Default::default()})},Some(FillStyle::RadialGradient{cx,cy,r,stops})=>{if stops.is_empty()||stops.len()>64{return Err(preparation_invalid("Invalid scene gradient stops"));}Some(FillStyle::RadialGradient{cx:coordinate(*cx)?,cy:coordinate(*cy)?,r:positive(*r)?,stops:Default::default()})}};self.at+=1;return Ok(true);
    }
    let stops=match source{Some(FillStyle::LinearGradient{stops,..})|Some(FillStyle::RadialGradient{stops,..})=>Some(stops),_=>None};
    if let Some(stop)=stops.and_then(|values|values.get(self.at-1)){let stop=semio_s_artifact_draw_drawing_model::GradientStop{offset:unit(stop.offset)?,color:color(stop.color)?};let target=match out_fill{Some(FillStyle::LinearGradient{stops,..})|Some(FillStyle::RadialGradient{stops,..})=>stops,_=>unreachable!()};if !work.slot(target).map_err(preparation_invalid)?{return Ok(false);}work.push(target,stop).map_err(|(error,_)|preparation_invalid(error))?;self.at+=1;return Ok(true);}
    let stroke=b.attributes.stroke.as_ref();
    if self.stroke_at==0{if let Some(s)=stroke{coordinate(s.width)?;if s.width<0.0||s.dash.as_ref().is_some_and(|d|d.len()>1024){return Err(preparation_invalid("Invalid scene stroke"));}if !work.can_step(){return Ok(false);}*out_stroke=Some(StrokeStyle{color:color(s.color)?,width:s.width,cap:s.cap.clone(),join:s.join.clone(),dash:s.dash.as_ref().map(|_|Default::default())});}else if !work.can_step(){return Ok(false);}self.stroke_at+=1;return Ok(true);}
    if let Some(value)=stroke.and_then(|s|s.dash.as_ref()).and_then(|v|v.get(self.stroke_at-1)){coordinate(*value)?;if *value<0.0{return Err(preparation_invalid("Invalid scene dash"));}let target=out_stroke.as_mut().ok_or_else(DocumentSceneError::held)?.dash.as_mut().ok_or_else(DocumentSceneError::held)?;if !work.slot(target).map_err(preparation_invalid)?{return Ok(false);}work.push(target,*value).map_err(|(error,_)|preparation_invalid(error))?;self.stroke_at+=1;return Ok(true);}
    if work.can_step(){self.at=0;self.transition(if matches!(self.layer(document)?,DrawingLayerNode::Group(_)|DrawingLayerNode::Boolean(_)){"references"}else{"geometry"});}
   },
   "geometry"=>{
    let next=match self.layer(document)?{DrawingLayerNode::Path(p)=>p.segments.get(self.at).cloned(),DrawingLayerNode::Shape(s)=>{let absent=if s.shape_kind.eq_str("rect"){s.rect.is_none()}else if s.shape_kind.eq_str("ellipse"){s.ellipse.is_none()}else if s.shape_kind.eq_str("circle"){s.circle.is_none()}else if s.shape_kind.eq_str("line"){s.line.is_none()}else if s.shape_kind.eq_str("polygon"){s.polygon.is_none()}else{true};if absent{return Err(preparation_invalid("Missing or unknown primitive geometry"));}shape_path_segment(s,self.at)},_=>None};
    if let Some(next)=next{validate_segment(&next)?;if self.segments>=self.limits.max_segments{return Err(preparation_invalid("Scene segment limit exceeded"));}let DocumentSceneContent::Path{segments,..}=&mut self.node.as_mut().ok_or_else(DocumentSceneError::held)?.content else{unreachable!()};if !scene_vector_slot(segments,work)?{return Ok(false);}work.vec_push(segments,next).map_err(|(error,_)|preparation_invalid(error))?;self.segments+=1;self.at+=1;return Ok(true);}
    if let DrawingLayerNode::Text(text)=self.layer(document)?{if self.text_at<text.content.len(){let mut chunk=self.text_chunk;let mut offset=self.text_chunk_offset;let next=text_scalar(&text.content,&mut chunk,&mut offset)?;let Some(ch)=next else{if work.can_step(){self.text_chunk=chunk;self.text_chunk_offset=offset;}return Ok(true);};if self.text_chars>=65536{return Err(preparation_invalid("Scene text exceeds limit"));}let DocumentSceneContent::Text{content,..}=&mut self.node.as_mut().ok_or_else(DocumentSceneError::held)?.content else{unreachable!()};if !work.reserve_string(content,text.content.len()).map_err(preparation_invalid)?{return Ok(false);}let mut bytes=[0;4];if !work.string_push(content,ch.encode_utf8(&mut bytes)).map_err(preparation_invalid)?{return Ok(false);}self.text_chunk=chunk;self.text_chunk_offset=offset;self.text_at+=ch.len_utf8();self.text_chars+=1;return Ok(true);}}
    if work.can_step(){self.transition("finish");}
   },
   "references"=>{
    let key=match self.layer(document)?{DrawingLayerNode::Group(g)=>g.children.get(self.at).map(|n|&layer_base(n).id),DrawingLayerNode::Boolean(b)=>b.children.get(self.at),_=>unreachable!()};
    let Some(key)=key else{if work.can_step(){self.transition("finish");}return Ok(true);};id(key)?;if self.references>=self.limits.max_references{return Err(preparation_invalid("Scene reference limit exceeded"));}
    let(complete,semantic)=scene_text_copy(key,&mut self.reference,&mut self.text_chunk,work)?;if !complete{return Ok(semantic);}let children=match &mut self.node.as_mut().ok_or_else(DocumentSceneError::held)?.content{DocumentSceneContent::Group{children,..}|DocumentSceneContent::Boolean{children,..}=>children,_=>unreachable!()};if !scene_vector_slot(children,work)?{return Ok(false);}work.vec_push(children,std::mem::take(&mut self.reference)).map_err(|(error,_)|preparation_invalid(error))?;self.references+=1;self.at+=1;self.text_chunk=0;
   },
   "finish"=>{
    if !scene_vector_slot(&mut self.plan.nodes,work)?{return Ok(false);}let node=self.node.take().ok_or_else(DocumentSceneError::held)?;let index=self.plan.nodes.len();
    if matches!(node.content,DocumentSceneContent::Group{..}){let mut frame=Frame{path:self.path,depth:self.depth,at:0,matrix:node.transform,visible:node.visible,locks:node.locked_ancestors,groups:self.parent.groups,group_count:self.parent.group_count};if matches!(node.content,DocumentSceneContent::Group{isolation:true,..})||node.opacity!=1.0||node.blend_mode!="normal"{frame.groups[frame.group_count]=index;frame.group_count+=1;}self.frames[self.frame_count]=frame;self.frame_count+=1;}
    work.vec_push(&mut self.plan.nodes,node).map_err(|(error,_)|preparation_invalid(error))?;self.transition("layers");
   },
   "validation"=>{
    let Some(node)=self.plan.nodes.get(self.validate_at)else{if work.can_step(){self.transition("cycles");}return Ok(true);};
    let asset=match &node.content{DocumentSceneContent::Image{asset,..}=>Some(asset),DocumentSceneContent::Trace{source,..}=>Some(source),_=>None};if asset.is_some_and(|key|!self.plan.assets.iter().any(|asset|&asset.id==key)){return Err(preparation_invalid("Missing scene source asset"));}
    if let Some(key)=node.content.children().get(self.ref_at){if !self.plan.nodes.iter().any(|node|&node.id==key){return Err(preparation_invalid("Missing scene operand"));}if work.can_step(){self.ref_at+=1;}}else if work.can_step(){self.validate_at+=1;self.ref_at=0;}
   },
   "cycles"=>{
    if self.graph_len>0{let(index,at)=self.graph[self.graph_len-1];if let Some(key)=self.plan.nodes[index].content.children().get(at){let next=self.plan.nodes.iter().position(|node|&node.id==key).ok_or_else(||preparation_invalid("Missing scene operand"))?;if self.visiting[next]{return Err(preparation_invalid("Cyclic scene operands"));}if !work.can_step(){return Ok(false);}self.graph[self.graph_len-1].1+=1;if !self.visited[next]{self.visiting[next]=true;self.graph[self.graph_len]=(next,0);self.graph_len+=1;}}else if work.can_step(){self.graph_len-=1;self.visiting[index]=false;self.visited[index]=true;}}
    else if self.cycle_at<self.plan.nodes.len(){if !work.can_step(){return Ok(false);}let index=self.cycle_at;self.cycle_at+=1;if !self.visited[index]{self.visiting[index]=true;self.graph[0]=(index,0);self.graph_len=1;}}
    else if work.can_step(){self.transition("complete");self.step_work=1;return Ok(false);}
   },
   _=>{}
  }Ok(true)
 }
 pub fn settlement_pending(&self)->bool{self.phase=="complete"||self.phase=="cycles"&&self.graph_len==0&&self.cycle_at>=self.plan.nodes.len()}
 pub fn failure_pending(&self)->bool{self.failure.is_some()}
 pub fn next_work_demand(&self)->semio_framework_value::RetirementDemand{self.pending}
 pub fn last_turn_work(&self)->u64{self.last_turn_work}
 pub fn last_turn_semantic_work(&self)->u64{self.last_turn_semantic_work}
 fn progress(&self)->DocumentSceneProgress{DocumentSceneProgress{phase:self.phase,layers:self.layers,assets:self.assets,segments:self.segments,references:self.references,source_bytes:self.source_bytes,work:self.work,done:self.phase=="complete"}}
 pub fn advance(&mut self,document:&impl DrawingSceneSource,budget:usize,work:&mut semio_framework_2d_text_font::work::RetainedWorkLedger)->Result<DocumentSceneProgress,DocumentSceneError>{
  self.last_turn_work=0;self.last_turn_semantic_work=0;if budget==0||budget as u128>9_007_199_254_740_991{return Err(preparation_invalid("Invalid scene preparation work grant"));}if self.cancelled{return Err(DocumentSceneError::Cancelled);}if let Some(error)=&self.failure{return Err(error.clone());}
  let d=self.pending;if !work.admits_absolute(d){work.require_absolute(d);return Ok(self.progress());}
  let Some(root)=work.enter()else{self.pending=work.next_demand();return Ok(self.progress());};
  let result=(||{for _ in 0..budget{if self.phase=="complete"{break;}if !work.can_step(){break;}let before=work.receipt();self.step_work=0;let result=self.step(document,work);let semantic=result.as_ref().copied().unwrap_or(true);let actual=semantic||self.step_work>0||work.receipt()!=before;if actual{work.note_source(root);self.last_turn_work+=1;}if semantic{self.work+=1;self.last_turn_semantic_work+=1;}self.pending=work.next_demand();if let Err(error)=result{self.failure=Some(error.clone());return Err(error);}if work.blocked(){break;}}Ok(self.progress())})();work.exit();result
 }
 fn into_retirement(mut self)->(DocumentSceneRetirement,Option<DocumentScenePlan>){let output=if self.phase=="complete"&&!self.cancelled&&self.failure.is_none(){Some(std::mem::take(&mut self.plan))}else{None};self.cancelled=true;(DocumentSceneRetirement::new(self),output)}
 pub fn cancel(&mut self){self.cancelled=true;}
 pub fn result(&mut self)->Result<DocumentScenePlan,DocumentSceneError>{if self.cancelled{return Err(DocumentSceneError::Cancelled);}if let Some(error)=&self.failure{return Err(error.clone());}if self.phase!="complete"{return Err(DocumentSceneError::Incomplete);}Ok(std::mem::take(&mut self.plan))}
}
#[derive(semio_framework_value::RetireOwned)]
struct DocumentSceneOwners{node:Option<DocumentSceneNode>,plan:DocumentScenePlan,asset_samples:Vec<u8>,asset_id:String,reference:String,group:Option<DrawingSceneGroup>,failure:Option<DocumentSceneError>}
impl RetireOwned for DocumentSceneCursor{
 fn retirement(self)->Box<dyn RetirementCursor>{DocumentSceneOwners{node:self.node,plan:self.plan,asset_samples:self.asset_samples,asset_id:self.asset_id,reference:self.reference,group:self.group,failure:self.failure}.retirement()}
 fn retirement_birth_bytes(&self)->Option<usize>{use semio_framework_value::retirement::{sequence_birth_bytes,deferred_birth_bytes_for};sequence_birth_bytes(&[deferred_birth_bytes_for(&self.node),deferred_birth_bytes_for(&self.plan),deferred_birth_bytes_for(&self.asset_samples),deferred_birth_bytes_for(&self.asset_id),deferred_birth_bytes_for(&self.reference),deferred_birth_bytes_for(&self.group),deferred_birth_bytes_for(&self.failure)])}
 fn controlled_retirement_supported()->bool{DocumentSceneOwners::controlled_retirement_supported()}
}
semio_framework_2d::physical_work_retirement!(DocumentSceneRetirement,DocumentSceneCursor);
/// 🧭️ Owns source positions so a retained caller supplies its immutable document per work grant.
pub struct DocumentScenePreparation{cursor:DocumentSceneCursor}
impl DocumentScenePreparation{
 pub fn new(document:&impl DrawingSceneSource,limits:DocumentSceneLimits)->Result<Self,DocumentSceneError>{Ok(Self{cursor:DocumentSceneCursor::new(document,limits)?})}
 pub fn advance(&mut self,document:&impl DrawingSceneSource,budget:usize,work:&mut semio_framework_2d_text_font::work::RetainedWorkLedger)->Result<DocumentSceneProgress,DocumentSceneError>{self.cursor.advance(document,budget,work)}
 pub fn settlement_pending(&self)->bool{self.cursor.settlement_pending()}
 pub fn failure_pending(&self)->bool{self.cursor.failure_pending()}
 pub fn next_work_demand(&self)->semio_framework_value::RetirementDemand{self.cursor.next_work_demand()}
 pub fn last_turn_work(&self)->u64{self.cursor.last_turn_work()}
 pub fn last_turn_semantic_work(&self)->u64{self.cursor.last_turn_semantic_work()}
 pub fn cancel(&mut self){self.cursor.cancel();}
 pub fn result(&mut self)->Result<DocumentScenePlan,DocumentSceneError>{self.cursor.result()}
 pub fn into_retirement(self)->(DocumentSceneRetirement,Option<DocumentScenePlan>){self.cursor.into_retirement()}
}
/// 🧱️ Borrows one stable document while retaining inline bounded source positions.
pub struct DocumentSceneJob<'a>{document:&'a DrawingSnapshot,pub cursor:DocumentSceneCursor}
impl<'a> DocumentSceneJob<'a>{
 pub fn new(document:&'a DrawingSnapshot,limits:DocumentSceneLimits)->Result<Self,DocumentSceneError>{Ok(Self{document,cursor:DocumentSceneCursor::new(document,limits)?})}
 pub fn advance(&mut self,budget:usize,work:&mut semio_framework_2d_text_font::work::RetainedWorkLedger)->Result<DocumentSceneProgress,DocumentSceneError>{self.cursor.advance(self.document,budget,work)}
 pub fn settlement_pending(&self)->bool{self.cursor.settlement_pending()}
 pub fn failure_pending(&self)->bool{self.cursor.failure_pending()}
 pub fn next_work_demand(&self)->semio_framework_value::RetirementDemand{self.cursor.next_work_demand()}
 pub fn last_turn_work(&self)->u64{self.cursor.last_turn_work()}
 pub fn last_turn_semantic_work(&self)->u64{self.cursor.last_turn_semantic_work()}
 pub fn cancel(&mut self){self.cursor.cancel();}
 pub fn result(&mut self)->Result<DocumentScenePlan,DocumentSceneError>{self.cursor.result()}
 pub fn into_retirement(self)->(DocumentSceneRetirement,Option<DocumentScenePlan>){self.cursor.into_retirement()}
}
/// 🚪️ Retains unresolved input and consumed source backing until funded physical retirement.
pub struct ResolvedSceneInputJob{
 source:DocumentScenePlan,viewport:DocumentSceneViewport,nodes:Vec<RasterSceneNode>,retired:Vec<DocumentSceneNode>,paths:Vec<Vec<u16>>,done:bool,cancelled:bool,reversed:bool,reverse_at:usize,failure:Option<DocumentSceneError>,work:u64,last_turn_work:u64,last_turn_semantic_work:u64,pending:semio_framework_value::RetirementDemand,message:String,message_part:usize,invalid:bool,
}
impl ResolvedSceneInputJob{
 pub fn new(source:DocumentScenePlan,viewport:DocumentSceneViewport)->Self{Self{source,viewport,nodes:Vec::new(),retired:Vec::new(),paths:Vec::new(),done:false,cancelled:false,reversed:false,reverse_at:0,failure:None,work:0,last_turn_work:0,last_turn_semantic_work:0,pending:semio_framework_value::RetirementDemand{depth:1,..Default::default()},message:String::new(),message_part:0,invalid:false}}
 pub fn settlement_pending(&self)->bool{self.source.nodes.is_empty()&&!self.invalid&&self.failure.is_none()}
 pub fn failure_pending(&self)->bool{self.invalid||self.failure.is_some()}
 pub fn next_work_demand(&self)->semio_framework_value::RetirementDemand{self.pending}
 pub fn last_turn_work(&self)->u64{self.last_turn_work}
 pub fn last_turn_semantic_work(&self)->u64{self.last_turn_semantic_work}
 pub fn cancel(&mut self){self.cancelled=true;}
 pub fn advance(&mut self,items:usize,work:&mut semio_framework_2d_text_font::work::RetainedWorkLedger)->Result<bool,DocumentSceneError>{
  self.last_turn_work=0;self.last_turn_semantic_work=0;if self.cancelled{return Err(DocumentSceneError::Cancelled);}if let Some(error)=&self.failure{return Err(error.clone());}if items==0{return Err(preparation_invalid("Invalid resolved scene work grant"));}
  let d=self.pending;if !work.admits_absolute(d){work.require_absolute(d);return Ok(self.done);}
  let Some(root)=work.enter()else{self.pending=work.next_demand();return Ok(self.done);};
  let result=(||{for _ in 0..items{if self.done||!work.can_step(){break;}let before=work.receipt();let settling=self.invalid;let result=self.step(work);let semantic=!settling&&matches!(&result,Ok((true,_))|Err(_));let actual=matches!(&result,Ok((_,true))|Err(_))||work.receipt()!=before;if actual{work.note_source(root);self.last_turn_work+=1;}if semantic{self.work+=1;self.last_turn_semantic_work+=1;}self.pending=work.next_demand();if let Err(error)=result{self.failure=Some(error.clone());return Err(error);}if work.blocked(){break;}}Ok(self.done)})();work.exit();result
 }
 fn step(&mut self,work:&mut semio_framework_2d_text_font::work::RetainedWorkLedger)->Result<(bool,bool),DocumentSceneError>{
  if self.invalid{
   let prefix="Unresolved scene layer: ";let node=self.source.nodes.last().ok_or_else(DocumentSceneError::held)?;let total=prefix.len().checked_add(node.id.len()).ok_or_else(||preparation_invalid("Resolved diagnostic extent overflow"))?;
   if !work.reserve_string(&mut self.message,total).map_err(preparation_invalid)?{return Ok((false,false));}
   if self.message_part<2{let part=if self.message_part==0{prefix}else{node.id.as_str()};if !work.string_push(&mut self.message,part).map_err(preparation_invalid)?{return Ok((false,false));}self.message_part+=1;return Ok((false,true));}
   if !work.arc_ready::<String>().map_err(preparation_invalid)?{return Ok((false,false));}let message=work.arc_new(std::mem::take(&mut self.message)).map_err(|(error,_)|preparation_invalid(error))?;return Err(DocumentSceneError::owned(message));
  }
  let Some(node)=self.source.nodes.last()else{
   if !self.reversed{let half=self.nodes.len()/2;if self.reverse_at<half{let right=self.nodes.len()-1-self.reverse_at;self.nodes.swap(self.reverse_at,right);self.reverse_at+=1;return Ok((false,true));}self.reversed=true;return Ok((false,true));}self.done=true;return Ok((false,true));
  };
  if matches!(node.content,DocumentSceneContent::Group{..}){if !scene_vector_slot(&mut self.retired,work)?{return Ok((false,false));}work.vec_push(&mut self.retired,self.source.nodes.pop().ok_or_else(DocumentSceneError::held)?).map_err(|(error,_)|preparation_invalid(error))?;return Ok((true,true));}
  if !matches!(node.content,DocumentSceneContent::Path{..}|DocumentSceneContent::Image{..}){self.invalid=true;return Ok((true,true));}
  if !scene_vector_slot(&mut self.nodes,work)?||!scene_vector_slot(&mut self.paths,work)?{return Ok((false,false));}
  let node=self.source.nodes.pop().ok_or_else(DocumentSceneError::held)?;let content=match node.content{DocumentSceneContent::Path{segments,fill_rule,fill,stroke}=>RasterSceneContent::Path{segments,fill_rule,fill,stroke},DocumentSceneContent::Image{asset,width,height}=>RasterSceneContent::Image{asset,width,height},_=>unreachable!()};
  work.vec_push(&mut self.paths,node.source_path).map_err(|(error,_)|preparation_invalid(error))?;
  work.vec_push(&mut self.nodes,RasterSceneNode{id:node.id,groups:node.groups,transform:node.transform,opacity:node.opacity,blend_mode:node.blend_mode,visible:node.visible,content}).map_err(|(error,_)|preparation_invalid(error))?;Ok((true,true))
 }
 pub fn result(&mut self)->Result<RasterSceneInput,DocumentSceneError>{if self.cancelled{return Err(DocumentSceneError::Cancelled);}if let Some(error)=&self.failure{return Err(error.clone());}if !self.done{return Err(DocumentSceneError::Incomplete);}Ok(self.viewport.input(std::mem::take(&mut self.source.assets),std::mem::take(&mut self.nodes)))}
}
#[derive(semio_framework_value::RetireOwned)]
struct ResolvedSceneInputOwners{source:DocumentScenePlan,nodes:Vec<RasterSceneNode>,retired:Vec<DocumentSceneNode>,paths:Vec<Vec<u16>>,failure:Option<DocumentSceneError>,message:String}
impl RetireOwned for ResolvedSceneInputJob{
 fn retirement(self)->Box<dyn RetirementCursor>{ResolvedSceneInputOwners{source:self.source,nodes:self.nodes,retired:self.retired,paths:self.paths,failure:self.failure,message:self.message}.retirement()}
 fn retirement_birth_bytes(&self)->Option<usize>{use semio_framework_value::retirement::{sequence_birth_bytes,deferred_birth_bytes_for};sequence_birth_bytes(&[deferred_birth_bytes_for(&self.source),deferred_birth_bytes_for(&self.nodes),deferred_birth_bytes_for(&self.retired),deferred_birth_bytes_for(&self.paths),deferred_birth_bytes_for(&self.failure),deferred_birth_bytes_for(&self.message)])}
 fn controlled_retirement_supported()->bool{ResolvedSceneInputOwners::controlled_retirement_supported()}
}
#[cfg(any(test, feature = "testkit"))]
#[path="🧰️testkit/🦀️.rs"]
pub mod testkit;
#[derive(Clone,Debug)]
pub struct DocumentSceneViewport{pub width:u32,pub height:u32,pub origin:[f64;2],pub tolerance:f64,pub max_pixels:usize,pub max_source_bytes:usize}
impl DocumentSceneViewport{fn input(&self,assets:Vec<RasterSceneAsset>,nodes:Vec<RasterSceneNode>)->RasterSceneInput{RasterSceneInput{width:self.width,height:self.height,origin:self.origin,tolerance:self.tolerance,max_pixels:self.max_pixels,max_source_bytes:self.max_source_bytes,assets,nodes}}}
#[derive(Clone,Debug)]
pub struct DocumentRasterProgress{pub phase:&'static str,pub preparation:DocumentSceneProgress,pub tracing:Option<DocumentTraceProgress>,pub resolution:Option<DocumentBooleanProgress>,pub raster:Option<crate::raster::RasterSceneProgress>,pub nodes:usize,pub work:u64,pub done:bool}
#[derive(Clone,Copy,Debug)]
pub struct DocumentAlgorithmLimits{pub booleans:DocumentBooleanLimits,pub trace:DocumentTraceLimits,pub max_work:u64}
impl Default for DocumentAlgorithmLimits{fn default()->Self{Self{max_work:1000000000,trace:DocumentTraceLimits{max_pixels:16777216,max_admitted_pixels:67108864,max_source_bytes:268439552,max_edges:65536,max_segments:65536,max_retained_segments:65536,max_work:1000000000},booleans:DocumentBooleanLimits{tolerance:0.05,epsilon:1e-8,max_depth:32,max_references:32768,max_edges:65536,max_parameters:262144,max_atomic_edges:65536,max_segments:65536,max_retained_segments:262144,max_work:1000000000}}}}
#[derive(Clone,Debug)]
pub struct DocumentVectorProgress{pub phase:&'static str,pub preparation:DocumentSceneProgress,pub tracing:Option<DocumentTraceProgress>,pub resolution:Option<DocumentBooleanProgress>,pub work:u64,pub done:bool}
impl Default for DocumentVectorProgress{fn default()->Self{Self{phase:"preparing",preparation:DocumentSceneProgress{phase:"assets",layers:0,assets:0,segments:0,references:0,source_bytes:0,work:0,done:false},tracing:None,resolution:None,work:0,done:false}}}
fn vector_slot<T>(values:&mut Vec<T>,work:&mut RetainedWorkLedger)->Result<bool,DocumentSceneError>{let needed=values.len().checked_add(1).ok_or_else(||DocumentSceneError::literal("Vector retained frontier overflow"))?;if values.capacity()<needed{let target=needed.checked_next_power_of_two().ok_or_else(||DocumentSceneError::literal("Vector retained capacity overflow"))?;if !work.reserve_vec(values,target).map_err(DocumentSceneError::literal)?{return Ok(false);}}work.vec_slot(values).map_err(DocumentSceneError::literal)}
enum DocumentVectorSource<'a>{Borrowed(&'a DrawingSnapshot),SnapshotRead(store::os_store::snapshot_read::SnapshotRead<DrawingSnapshot>)}
impl DocumentVectorSource<'_>{fn document(&self)->&DrawingSnapshot{match self{Self::Borrowed(document)=>document,Self::SnapshotRead(read)=>read.get()}}}
#[derive(semio_framework_value::RetireOwned)]
enum VectorChildRetirement{Preparation(DocumentSceneRetirement),Trace(DocumentTraceRetirement),Text(DocumentTextRetirement),Boolean(DocumentBooleanRetirement)}
/// 🎬️ Resolves one stable source while retaining text and authored paint metadata.
pub struct DocumentVectorJob<'a>{
 pub source:Option<DocumentVectorSource<'a>>,pub preparation:Option<DocumentSceneCursor>,pub traces:Option<DocumentTraceJob>,texts:Option<DocumentTextJob>,pub algorithms:Option<DocumentBooleanJob>,progress:DocumentVectorProgress,
 pub output:Option<DocumentScenePlan>,failure:Option<DocumentSceneError>,cancelled:bool,limits:DocumentAlgorithmLimits,pub closing:Option<VectorChildRetirement>,discard:Option<ScenePlanCloseJob>,handoff:Option<DocumentScenePlan>,retired_children:Vec<VectorChildRetirement>,retired_plans:Vec<ScenePlanCloseJob>,pending:RetirementDemand,last_turn_work:u64,last_turn_semantic_work:u64,step_semantic:bool,step_cpu:bool,
}
impl<'a> DocumentVectorJob<'a>{
 pub fn new(document:&'a DrawingSnapshot,limits:DocumentSceneLimits,algorithms:DocumentAlgorithmLimits)->Result<Self,DocumentSceneError>{Self::from_source(DocumentVectorSource::Borrowed(document),limits,algorithms)}
 fn from_source(source:DocumentVectorSource<'a>,limits:DocumentSceneLimits,algorithms:DocumentAlgorithmLimits)->Result<Self,DocumentSceneError>{let mut job=Self::from_scene_source(source.document(),limits,algorithms)?;job.source=Some(source);Ok(job)}
 /// 👁️ Stores only preparation positions while the caller retains real root and asset custody.
 pub fn from_scene_source(document:&impl DrawingSceneSource,limits:DocumentSceneLimits,algorithms:DocumentAlgorithmLimits)->Result<Self,DocumentSceneError>{
  if !(1..=1000000000).contains(&algorithms.max_work){return Err(invalid("Invalid document vector work limit"));}
  let preparation=DocumentSceneCursor::new(document,limits)?;
  DocumentBooleanJob::validate_limits(algorithms.booleans)?;DocumentTraceJob::validate_limits(algorithms.trace)?;
  Ok(Self{preparation:Some(preparation),source:None,traces:None,texts:None,algorithms:None,progress:DocumentVectorProgress::default(),output:None,failure:None,cancelled:false,limits:algorithms,closing:None,discard:None,handoff:None,retired_children:Vec::new(),retired_plans:Vec::new(),pending:RetirementDemand{depth:1,..Default::default()},last_turn_work:0,last_turn_semantic_work:0,step_semantic:false,step_cpu:false})
 }
 /// 🔐️ Owns the genuine immutable store read while preparation retains only source positions.
 pub fn from_snapshot_read(read:store::os_store::snapshot_read::SnapshotRead<DrawingSnapshot>,limits:DocumentSceneLimits,algorithms:DocumentAlgorithmLimits)->DocumentVectorJob<'static>{
  let mut job=DocumentVectorJob{preparation:None,source:Some(DocumentVectorSource::SnapshotRead(read)),traces:None,texts:None,algorithms:None,progress:DocumentVectorProgress::default(),output:None,failure:None,cancelled:false,limits:algorithms,closing:None,discard:None,handoff:None,retired_children:Vec::new(),retired_plans:Vec::new(),pending:RetirementDemand{depth:1,..Default::default()},last_turn_work:0,last_turn_semantic_work:0,step_semantic:false,step_cpu:false};
  let admitted=(||->Result<(),DocumentSceneError>{
   if !(1..=1000000000).contains(&algorithms.max_work){return Err(invalid("Invalid document vector work limit"));}
   DocumentBooleanJob::validate_limits(algorithms.booleans)?;DocumentTraceJob::validate_limits(algorithms.trace)?;
   job.preparation=Some(DocumentSceneCursor::new(job.source.as_ref().unwrap().document(),limits)?);Ok(())
  })();job.failure=admitted.err();job
 }
 /// 🛂️ Checks the captured generation and revision against the store's current event authority.
 pub fn snapshot_authority_matches(&self,generation:u64,revision:[u8;32])->bool{match &self.source{Some(DocumentVectorSource::SnapshotRead(read))=>read.commit_authority_matches(generation,revision),_=>false}}
 fn step(&mut self,document:&impl DrawingSceneSource,work:&mut RetainedWorkLedger)->Result<(),DocumentSceneError>{
  if self.closing.is_some(){if !vector_slot(&mut self.retired_children,work)?{return Ok(());}work.vec_push(&mut self.retired_children,self.closing.take().ok_or_else(DocumentSceneError::held)?).map_err(|_|DocumentSceneError::literal("Vector child archive authority lost"))?;self.step_cpu=true;return Ok(());}
  if self.discard.is_some(){if !vector_slot(&mut self.retired_plans,work)?{return Ok(());}work.vec_push(&mut self.retired_plans,self.discard.take().ok_or_else(DocumentSceneError::held)?).map_err(|_|DocumentSceneError::literal("Vector plan archive authority lost"))?;self.step_cpu=true;return Ok(());}
  if self.handoff.is_some(){if !work.can_step(){return Ok(());}let plan=self.handoff.take().ok_or_else(DocumentSceneError::held)?;match self.progress.phase{"preparing"=>{self.traces=Some(DocumentTraceJob::new(DocumentTraceInput{plan,limits:self.limits.trace}));self.progress.phase="tracing";},"tracing"=>{self.texts=Some(DocumentTextJob::new(plan,self.limits.max_work));self.progress.phase="text";},"text"=>{self.algorithms=Some(DocumentBooleanJob::new(DocumentBooleanInput{plan,limits:self.limits.booleans}));self.progress.phase="algorithms";},_=>{self.output=Some(plan);self.progress.phase="complete";self.progress.done=true;}}self.step_cpu=true;return Ok(());}
  match self.progress.phase{
   "preparing"=>{if self.progress.preparation.done{let(close,plan)=self.preparation.take().ok_or_else(DocumentSceneError::held)?.into_retirement();self.closing=Some(VectorChildRetirement::Preparation(close));self.handoff=plan;self.step_cpu=true;return Ok(());}let child=self.preparation.as_mut().ok_or_else(DocumentSceneError::held)?;let result=child.advance(document,1,work);self.last_turn_work+=child.last_turn_work();self.last_turn_semantic_work+=child.last_turn_semantic_work();self.progress.preparation=result?;}
   "tracing"=>{if self.traces.as_ref().ok_or_else(DocumentSceneError::held)?.result().is_ok(){let(close,input,plan)=self.traces.take().ok_or_else(DocumentSceneError::held)?.into_retirement();self.closing=Some(VectorChildRetirement::Trace(close));self.discard=Some(ScenePlanCloseJob::new(input));self.handoff=plan;self.step_cpu=true;return Ok(());}let child=self.traces.as_mut().ok_or_else(DocumentSceneError::held)?;let result=child.advance(1,work);self.last_turn_work+=child.last_turn_work();self.last_turn_semantic_work+=child.last_turn_semantic_work();self.progress.tracing=Some(result?);}
   "text"=>{if self.texts.as_ref().ok_or_else(DocumentSceneError::held)?.result().is_ok(){let(close,plan)=self.texts.take().ok_or_else(DocumentSceneError::held)?.into_retirement();self.closing=Some(VectorChildRetirement::Text(close));self.handoff=plan;self.step_cpu=true;return Ok(());}let child=self.texts.as_mut().ok_or_else(DocumentSceneError::held)?;let result=child.advance(1,work);self.last_turn_work+=child.last_turn_work();self.last_turn_semantic_work+=child.last_turn_semantic_work();result?;}
   "algorithms"=>{if self.algorithms.as_ref().ok_or_else(DocumentSceneError::held)?.result().is_ok(){let(close,input,plan)=self.algorithms.take().ok_or_else(DocumentSceneError::held)?.into_retirement();self.closing=Some(VectorChildRetirement::Boolean(close));self.discard=Some(ScenePlanCloseJob::new(input));self.handoff=plan;self.step_cpu=true;return Ok(());}let child=self.algorithms.as_mut().ok_or_else(DocumentSceneError::held)?;let result=child.advance(1,work);self.last_turn_work+=child.last_turn_work();self.last_turn_semantic_work+=child.last_turn_semantic_work();self.progress.resolution=Some(result?);}
   _=>{}
  }Ok(())
 }
 pub fn failure_pending(&self)->bool{self.failure.is_some()||self.preparation.as_ref().is_some_and(DocumentSceneCursor::failure_pending)||self.traces.as_ref().is_some_and(DocumentTraceJob::failure_pending)||self.texts.as_ref().is_some_and(DocumentTextJob::failure_pending)||self.algorithms.as_ref().is_some_and(DocumentBooleanJob::failure_pending)}
 pub fn settlement_pending(&self)->bool{if self.progress.done||self.closing.is_some()||self.discard.is_some(){return true;}if self.handoff.is_some(){return true;}match self.progress.phase{"preparing"=>self.preparation.as_ref().is_some_and(DocumentSceneCursor::settlement_pending),"tracing"=>self.traces.as_ref().is_some_and(DocumentTraceJob::settlement_pending),"text"=>self.texts.as_ref().is_some_and(DocumentTextJob::settlement_pending),"algorithms"=>self.algorithms.as_ref().is_some_and(DocumentBooleanJob::settlement_pending),_=>false}}
 pub fn next_work_demand(&self)->RetirementDemand{self.pending}
 pub fn last_turn_work(&self)->u64{self.last_turn_work}
 pub fn last_turn_semantic_work(&self)->u64{self.last_turn_semantic_work}
 pub fn advance_scene_source(&mut self,document:&impl DrawingSceneSource,budget:usize,work:&mut RetainedWorkLedger)->Result<DocumentVectorProgress,DocumentSceneError>{
  self.last_turn_work=0;self.last_turn_semantic_work=0;if budget==0||budget as u128>9_007_199_254_740_991{return Err(DocumentSceneError::literal("Invalid document work grant"));}if self.cancelled{return Err(DocumentSceneError::Cancelled);}if let Some(error)=&self.failure{return Err(error.clone());}
  for _ in 0..budget{
   if self.progress.done{break;}let d=self.pending;
   if !work.admits_absolute(d){work.require_absolute(d);break;}
   if self.progress.work>=self.limits.max_work&&!self.failure_pending(){let error=DocumentSceneError::literal("Document vector work limit exceeded");self.failure=Some(error.clone());return Err(error);}
   let settling=self.failure_pending();let before=work.receipt();let before_cpu=self.last_turn_work;let before_semantic=self.last_turn_semantic_work;self.step_semantic=false;self.step_cpu=false;let entered=work.enter();
   if entered.is_none(){self.pending=work.next_demand();break;}
   let result=self.step(document,work);let child_cpu=self.last_turn_work-before_cpu;let child_semantic=self.last_turn_semantic_work-before_semantic;let actual=child_cpu>0||work.receipt()!=before||self.step_cpu||result.is_err();
   if actual{work.note_source(entered==Some(true));if child_cpu==0{self.last_turn_work+=1;}}if !settling&&child_semantic==0&&(self.step_semantic||result.is_err()){self.last_turn_semantic_work+=1;}if !settling{self.progress.work+=self.last_turn_semantic_work-before_semantic;}work.exit();
   self.pending=work.next_demand();if let Err(error)=result{self.failure=Some(error.clone());return Err(error);}
   if work.blocked()||!actual{break;}
  }Ok(self.progress.clone())
 }
 pub fn advance(&mut self,budget:usize,work:&mut RetainedWorkLedger)->Result<DocumentVectorProgress,DocumentSceneError>{let source=self.source.take().ok_or_else(||DocumentSceneError::literal("Borrowed scene producer requires its real source each turn"))?;let result=self.advance_scene_source(source.document(),budget,work);self.source=Some(source);result}
 pub fn advance_original(&mut self,cx:&mut StepContext<'_>)->Result<DocumentVectorProgress,OriginalWorkFailure<DocumentSceneError>>{
  self.last_turn_work=0;self.last_turn_semantic_work=0;OriginalWorkFailure::qualify(cx)?;let mut work=RetainedWorkLedger::new(cx.retained_grant());let result=if cx.is_cancelled(){Err(DocumentSceneError::Cancelled)}else if cx.should_yield(){Ok(self.progress.clone())}else{self.advance(1,&mut work)};
  match result{Ok(progress)=>{OriginalWorkFailure::<DocumentSceneError>::fold(cx,None,work.receipt(),self.last_turn_work)?;Ok(progress)},Err(error)=>Err(OriginalWorkFailure::fold(cx,Some(error),work.receipt(),self.last_turn_work).expect_err("an original body failure remains owned"))}
 }
 /// 🧹️ Retain actual source authority while all child and intermediate plan owners drain.
 pub fn into_retirement(mut self)->(DocumentVectorRetirement<'a>,Option<DocumentScenePlan>){
  let output=if self.progress.done&&!self.cancelled&&self.failure.is_none(){self.output.take()}else{None};
  let owners=DocumentVectorOwners{children:self.retired_children,plans:self.retired_plans,preparation:self.preparation,traces:self.traces,texts:self.texts,algorithms:self.algorithms,closing:self.closing,discard:self.discard,handoff:self.handoff,output:self.output,failure:self.failure};
  (DocumentVectorRetirement{source:self.source.take(),owner:ControlledRetirement::retain(owners)},output)
 }
 pub fn cancel(&mut self){self.cancelled=true;}
 pub fn result(&mut self)->Result<DocumentScenePlan,DocumentSceneError>{if self.cancelled{return Err(DocumentSceneError::Cancelled);}if let Some(e)=&self.failure{return Err(e.clone());}if !self.progress.done{return Err(DocumentSceneError::Incomplete);}self.output.take().ok_or(DocumentSceneError::Incomplete)}
}
#[derive(semio_framework_value::RetireOwned)]
#[value(send_only)]
struct DocumentVectorOwners{children:Vec<VectorChildRetirement>,plans:Vec<ScenePlanCloseJob>,preparation:Option<DocumentSceneCursor>,traces:Option<DocumentTraceJob>,texts:Option<DocumentTextJob>,algorithms:Option<DocumentBooleanJob>,closing:Option<VectorChildRetirement>,discard:Option<ScenePlanCloseJob>,handoff:Option<DocumentScenePlan>,output:Option<DocumentScenePlan>,failure:Option<DocumentSceneError>}
/// 🔐️ Actual private byte ownership closes before the exact source read returns to its registry.
pub struct DocumentVectorRetirement<'a>{pub source:Option<DocumentVectorSource<'a>>,owner:ControlledRetirement<DocumentVectorOwners>}
impl DocumentVectorRetirement<'_>{
 pub fn private_is_empty(&self)->bool{self.owner.terminal_is_empty()}
 pub fn terminal_is_empty(&self)->bool{self.private_is_empty()&&self.source.is_none()}
 pub fn snapshot_authority_matches(&self,generation:u64,revision:[u8;32])->bool{match &self.source{Some(DocumentVectorSource::SnapshotRead(read))=>read.commit_authority_matches(generation,revision),_=>false}}
 pub fn take_snapshot_read(&mut self)->Option<store::os_store::snapshot_read::SnapshotRead<DrawingSnapshot>>{if !self.private_is_empty(){return None;}match self.source.take(){Some(DocumentVectorSource::SnapshotRead(read))=>Some(read),other=>{self.source=other;None}}}
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if !self.private_is_empty(){return self.owner.step(grant);}
  if grant.maximum_items>0&&grant.maximum_depth>0&&matches!(self.source,Some(DocumentVectorSource::Borrowed(_))){self.source=None;return Ok(RetainedCloneStep::Complete(semio_framework_value::retained_clone::RetainedCloneProgress{copied_items:1,..Default::default()}));}
  Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(Default::default())}else{RetainedCloneStep::Progress(Default::default())})
 }
 pub fn close_original(&mut self,grant:RetainedCloneGrant)->semio_framework_value::RetainedWorkOutcome{
  if !self.private_is_empty(){let result=self.owner.step(grant);return semio_framework_value::RetainedWorkOutcome{result,work:self.owner.last_turn_work()};}
  let moving=grant.maximum_items>0&&grant.maximum_depth>0&&matches!(self.source,Some(DocumentVectorSource::Borrowed(_)));
  let result=self.close_step(grant);semio_framework_value::RetainedWorkOutcome{result,work:u64::from(moving)}
 }
 pub fn next_copy_byte_demand(&self)->Result<usize,ValueError>{self.owner.next_copy_byte_demand()}
 pub fn next_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{self.owner.next_capacity_byte_demand(body)}
 pub fn next_release_byte_demand(&self)->Result<usize,ValueError>{self.owner.next_release_byte_demand()}
 pub fn next_depth_demand(&self)->Result<usize,ValueError>{if self.private_is_empty(){Ok(usize::from(self.source.is_some()))}else{self.owner.next_depth_demand()}}

}
#[derive(semio_framework_value::RetireOwned)]
#[value(send_only)]
struct BorrowedVectorOwners{preparation:Option<DocumentSceneCursor>,traces:Option<DocumentTraceJob>,texts:Option<DocumentTextJob>,algorithms:Option<DocumentBooleanJob>,output:Option<DocumentScenePlan>,failure:Option<DocumentSceneError>,closing:Option<VectorChildRetirement>,discard:Option<ScenePlanCloseJob>,handoff:Option<DocumentScenePlan>,retired_children:Vec<VectorChildRetirement>,retired_plans:Vec<ScenePlanCloseJob>}
/// 👁️ Actual scene parts are borrowed per turn while every private producer owner stays typed.
pub struct DocumentVectorPreparationJob{job:DocumentVectorJob<'static>}
impl DocumentVectorPreparationJob{
 pub fn new(source:&impl DrawingSceneSource,limits:DocumentSceneLimits,algorithms:DocumentAlgorithmLimits)->Result<Self,DocumentSceneError>{Ok(Self{job:DocumentVectorJob::from_scene_source(source,limits,algorithms)?})}
 pub fn advance(&mut self,source:&impl DrawingSceneSource,grant:usize,work:&mut RetainedWorkLedger)->Result<DocumentVectorProgress,DocumentSceneError>{self.job.advance_scene_source(source,grant,work)}
 pub fn next_work_demand(&self)->RetirementDemand{self.job.next_work_demand()}
 pub fn last_turn_work(&self)->u64{self.job.last_turn_work()}
 pub fn last_turn_semantic_work(&self)->u64{self.job.last_turn_semantic_work()}
 pub fn settlement_pending(&self)->bool{self.job.settlement_pending()}
 pub fn failure_pending(&self)->bool{self.job.failure_pending()}
 pub fn advance_original(&mut self,source:&impl DrawingSceneSource,cx:&mut StepContext<'_>)->Result<DocumentVectorProgress,OriginalWorkFailure<DocumentSceneError>>{
  self.job.last_turn_work=0;self.job.last_turn_semantic_work=0;OriginalWorkFailure::qualify(cx)?;let mut work=RetainedWorkLedger::new(cx.retained_grant());let result=if cx.is_cancelled(){Err(DocumentSceneError::Cancelled)}else if cx.should_yield(){Ok(self.job.progress.clone())}else{self.advance(source,1,&mut work)};
  match result{Ok(progress)=>{OriginalWorkFailure::<DocumentSceneError>::fold(cx,None,work.receipt(),self.last_turn_work())?;Ok(progress)},Err(error)=>Err(OriginalWorkFailure::fold(cx,Some(error),work.receipt(),self.last_turn_work()).expect_err("an original body failure remains owned"))}
 }
 pub fn cancel(&mut self){self.job.cancel();}
 pub fn result(&mut self)->Result<DocumentScenePlan,DocumentSceneError>{self.job.result()}
 pub fn into_retirement(mut self)->(DocumentVectorPreparationRetirement,Option<DocumentScenePlan>){let output=if self.job.progress.done&&!self.job.cancelled&&self.job.failure.is_none(){self.job.output.take()}else{None};self.cancel();(DocumentVectorPreparationRetirement::new(self),output)}
}
impl RetireOwned for DocumentVectorPreparationJob{
 fn retirement(self)->Box<dyn RetirementCursor>{let job=self.job;assert!(job.source.is_none(),"borrowed preparation cannot own a fabricated snapshot source");BorrowedVectorOwners{preparation:job.preparation,traces:job.traces,texts:job.texts,algorithms:job.algorithms,output:job.output,failure:job.failure,closing:job.closing,discard:job.discard,handoff:job.handoff,retired_children:job.retired_children,retired_plans:job.retired_plans}.retirement()}
 fn retirement_birth_bytes(&self)->Option<usize>{use semio_framework_value::retirement::{sequence_birth_bytes,deferred_birth_bytes_for};let job=&self.job;sequence_birth_bytes(&[deferred_birth_bytes_for(&job.preparation),deferred_birth_bytes_for(&job.traces),deferred_birth_bytes_for(&job.texts),deferred_birth_bytes_for(&job.algorithms),deferred_birth_bytes_for(&job.output),deferred_birth_bytes_for(&job.failure),deferred_birth_bytes_for(&job.closing),deferred_birth_bytes_for(&job.discard),deferred_birth_bytes_for(&job.handoff),deferred_birth_bytes_for(&job.retired_children),deferred_birth_bytes_for(&job.retired_plans)])}
 fn controlled_retirement_supported()->bool{BorrowedVectorOwners::controlled_retirement_supported()}
}
semio_framework_2d::physical_work_retirement!(DocumentVectorPreparationRetirement,DocumentVectorPreparationJob);
/// 📷️ Resolves each raster record after the common document vector producer completes.
pub struct DocumentRasterJob<'a>{
 vector:Option<DocumentVectorJob<'a>>,prepared:DocumentVectorProgress,source:VecDeque<DocumentSceneNode>,assets:Vec<RasterSceneAsset>,nodes:Vec<RasterSceneNode>,retired_nodes:Vec<DocumentSceneNode>,retired_paths:Vec<Vec<u16>>,retired_texts:Vec<String>,at:usize,work:u64,phase:&'static str,
 raster:Option<crate::raster::RasterSceneJob>,rendered:Option<crate::raster::RasterSceneProgress>,output:Option<semio_framework_pixels::RasterImage>,failure:Option<DocumentSceneError>,cancelled:bool,viewport:DocumentSceneViewport,max_work:u64,
 vector_retirement:Option<DocumentVectorRetirement<'a>>,raster_retirement:Option<crate::raster::RasterSceneRetirement>,handoff:Option<DocumentScenePlan>,
 pixel_scale:[f64;2],pending:RetirementDemand,last_turn_work:u64,last_turn_semantic_work:u64,step_semantic:bool,step_cpu:bool,
}
impl<'a> DocumentRasterJob<'a>{
 pub fn new(document:&'a DrawingSnapshot,limits:DocumentSceneLimits,viewport:DocumentSceneViewport,resolution_limits:DocumentAlgorithmLimits)->Result<Self,DocumentSceneError>{
  Self::from_vector(DocumentVectorJob::new(document,limits,resolution_limits)?,viewport,resolution_limits.max_work)
 }
 fn from_vector(vector:DocumentVectorJob<'a>,viewport:DocumentSceneViewport,max_work:u64)->Result<Self,DocumentSceneError>{
  crate::raster::validate_scene_input(&viewport.input(Vec::new(),Vec::new())).map_err(DocumentSceneError::from)?;
  Ok(Self{vector:Some(vector),prepared:DocumentVectorProgress::default(),source:VecDeque::new(),assets:Vec::new(),nodes:Vec::new(),retired_nodes:Vec::new(),retired_paths:Vec::new(),retired_texts:Vec::new(),at:0,work:0,phase:"preparing",raster:None,rendered:None,output:None,failure:None,cancelled:false,viewport,max_work,vector_retirement:None,raster_retirement:None,handoff:None,pixel_scale:[1.0;2],pending:RetirementDemand{depth:1,..Default::default()},last_turn_work:0,last_turn_semantic_work:0,step_semantic:false,step_cpu:false})
 }
 /// 🔐️ Retains the actual immutable store read through every raster and output handoff.
 pub fn from_snapshot_read(read:store::os_store::snapshot_read::SnapshotRead<DrawingSnapshot>,limits:DocumentSceneLimits,viewport:DocumentSceneViewport,algorithms:DocumentAlgorithmLimits)->DocumentRasterJob<'static>{
  let failure=crate::raster::validate_scene_input(&viewport.input(Vec::new(),Vec::new())).err().map(DocumentSceneError::from);
  DocumentRasterJob{vector:Some(DocumentVectorJob::from_snapshot_read(read,limits,algorithms)),prepared:DocumentVectorProgress::default(),source:VecDeque::new(),assets:Vec::new(),nodes:Vec::new(),retired_nodes:Vec::new(),retired_paths:Vec::new(),retired_texts:Vec::new(),at:0,work:0,phase:"preparing",raster:None,rendered:None,output:None,failure,cancelled:false,viewport,max_work:algorithms.max_work,vector_retirement:None,raster_retirement:None,handoff:None,pixel_scale:[1.0;2],pending:RetirementDemand{depth:1,..Default::default()},last_turn_work:0,last_turn_semantic_work:0,step_semantic:false,step_cpu:false}
 }
 /// 🔍️ Maps the immutable artboard extent to the exact requested pixel extent.
 pub fn set_pixel_scale(&mut self,x:f64,y:f64)->Result<(),DocumentSceneError>{if self.work!=0||![x,y].into_iter().all(|n|n.is_finite()&&n>0.0){return Err(invalid("Invalid document pixel scale"));}self.pixel_scale=[x,y];Ok(())}
 fn step(&mut self,work:&mut RetainedWorkLedger)->Result<(),DocumentSceneError>{
  if self.vector_retirement.is_some(){if self.handoff.is_some(){if !work.can_step(){return Ok(());}let plan=self.handoff.take().ok_or_else(DocumentSceneError::held)?;self.source=plan.nodes.into();self.assets=plan.assets;self.step_cpu=true;return Ok(());}}
  
  if self.vector.is_some(){if self.prepared.done{let(close,plan)=self.vector.take().ok_or_else(DocumentSceneError::held)?.into_retirement();self.vector_retirement=Some(close);self.handoff=plan;self.phase="resolving";self.step_cpu=true;return Ok(());}let vector=self.vector.as_mut().ok_or_else(DocumentSceneError::held)?;let result=vector.advance(1,work);self.last_turn_work+=vector.last_turn_work();self.last_turn_semantic_work+=vector.last_turn_semantic_work();self.prepared=result?;if !self.prepared.done{self.phase=self.prepared.phase;}return Ok(());}
  match self.phase{
   "resolving"=>{
    if let Some(n)=self.source.front(){
     let hidden=!n.visible||matches!(&n.content,DocumentSceneContent::Group{..});let glyph=matches!(&n.content,DocumentSceneContent::Glyphs{..});
     if hidden{if !vector_slot(&mut self.retired_nodes,work)?{return Ok(());}}
     else{
      if !matches!(&n.content,DocumentSceneContent::Path{..}|DocumentSceneContent::Glyphs{..}|DocumentSceneContent::Image{..}){return Err(DocumentSceneError::literal("Unresolved scene layer"));}
      if !vector_slot(&mut self.retired_paths,work)?{return Ok(());}
      if glyph{if !vector_slot(&mut self.retired_texts,work)?{return Ok(());}}
      if !vector_slot(&mut self.nodes,work)?{return Ok(());}
     }
     let n=self.source.pop_front().ok_or_else(DocumentSceneError::held)?;self.at+=1;
     if hidden{work.vec_push(&mut self.retired_nodes,n).map_err(|_|DocumentSceneError::literal("Raster hidden source authority lost"))?;self.step_semantic=true;self.step_cpu=true;return Ok(());}
     let content=match n.content{DocumentSceneContent::Path{segments,fill_rule,fill,stroke}=>RasterSceneContent::Path{segments,fill_rule,fill,stroke},DocumentSceneContent::Glyphs{content,segments,fill_rule,fill,stroke,..}=>{work.vec_push(&mut self.retired_texts,content).map_err(|_|DocumentSceneError::literal("Raster text archive authority lost"))?;RasterSceneContent::Path{segments,fill_rule,fill,stroke}},DocumentSceneContent::Image{asset,width,height}=>RasterSceneContent::Image{asset,width,height},_=>unreachable!()};
     work.vec_push(&mut self.retired_paths,n.source_path).map_err(|_|DocumentSceneError::literal("Raster source address authority lost"))?;let m=n.transform;let[x,y]=self.pixel_scale;work.vec_push(&mut self.nodes,RasterSceneNode{id:n.id,groups:n.groups,transform:[m[0]*x,m[1]*y,m[2]*x,m[3]*y,m[4]*x,m[5]*y],opacity:n.opacity,blend_mode:n.blend_mode,visible:n.visible,content}).map_err(|_|DocumentSceneError::literal("Raster node authority lost"))?;self.step_semantic=true;self.step_cpu=true;
    }else{if !work.can_step(){return Ok(());}self.raster=Some(crate::raster::RasterSceneJob::new(self.viewport.input(std::mem::take(&mut self.assets),std::mem::take(&mut self.nodes))));self.phase="raster";self.step_cpu=true;}
   }
   "raster"=>{if self.rendered.as_ref().is_some_and(|progress|progress.done){let(close,image)=self.raster.take().ok_or_else(DocumentSceneError::held)?.into_retirement();self.output=image;self.raster_retirement=Some(close);self.phase="complete";self.step_cpu=true;return Ok(());}let child=self.raster.as_mut().ok_or_else(DocumentSceneError::held)?;let result=child.advance(1,work);self.last_turn_work+=child.last_turn_work();self.last_turn_semantic_work+=child.last_turn_semantic_work();self.rendered=Some(result.map_err(DocumentSceneError::from)?);}
   _=>{}
  }Ok(())
 }
 pub fn failure_pending(&self)->bool{self.failure.is_some()||self.vector.as_ref().is_some_and(DocumentVectorJob::failure_pending)||self.raster.as_ref().is_some_and(crate::raster::RasterSceneJob::failure_pending)}
 pub fn settlement_pending(&self)->bool{self.phase=="complete"||self.handoff.is_some()||self.phase=="resolving"&&self.source.is_empty()||self.vector.as_ref().is_some_and(DocumentVectorJob::settlement_pending)||self.raster.as_ref().is_some_and(crate::raster::RasterSceneJob::settlement_pending)}
 pub fn next_work_demand(&self)->RetirementDemand{self.pending}
 pub fn last_turn_work(&self)->u64{self.last_turn_work}
 pub fn last_turn_semantic_work(&self)->u64{self.last_turn_semantic_work}
 pub fn progress(&self)->DocumentRasterProgress{DocumentRasterProgress{phase:self.phase,preparation:self.prepared.preparation.clone(),tracing:self.prepared.tracing,resolution:self.prepared.resolution,raster:self.rendered.clone(),nodes:self.at,work:self.work,done:self.phase=="complete"}}
 pub fn advance(&mut self,budget:usize,work:&mut RetainedWorkLedger)->Result<DocumentRasterProgress,DocumentSceneError>{
  self.last_turn_work=0;self.last_turn_semantic_work=0;if budget==0||budget as u128>9_007_199_254_740_991{return Err(DocumentSceneError::literal("Invalid document work grant"));}if self.cancelled{return Err(DocumentSceneError::Cancelled);}if let Some(error)=&self.failure{return Err(error.clone());}
  for _ in 0..budget{
   if self.phase=="complete"{break;}let d=self.pending;
   if !work.admits_absolute(d){work.require_absolute(d);break;}
   if self.work>=self.max_work&&!self.failure_pending(){let error=DocumentSceneError::literal("Document raster work limit exceeded");self.failure=Some(error.clone());return Err(error);}
   let settling=self.failure_pending();let before=work.receipt();let before_cpu=self.last_turn_work;let before_semantic=self.last_turn_semantic_work;self.step_semantic=false;self.step_cpu=false;let entered=work.enter();if entered.is_none(){self.pending=work.next_demand();break;}
   let exhausted=self.work>=self.max_work&&!settling&&!self.settlement_pending();let result=if exhausted{Err(DocumentSceneError::literal("Document raster work limit exceeded"))}else{self.step(work)};let child_cpu=self.last_turn_work-before_cpu;let child_semantic=self.last_turn_semantic_work-before_semantic;let actual=child_cpu>0||work.receipt()!=before||self.step_cpu||result.is_err();
   if actual{work.note_source(entered==Some(true));if child_cpu==0{self.last_turn_work+=1;}}if !settling&&!exhausted&&child_semantic==0&&(self.step_semantic||result.is_err()){self.last_turn_semantic_work+=1;}if !settling{self.work+=self.last_turn_semantic_work-before_semantic;}work.exit();self.pending=work.next_demand();if let Err(error)=result{self.failure=Some(error.clone());return Err(error);}if work.blocked()||!actual{break;}
  }Ok(self.progress())
 }
 pub fn advance_original(&mut self,cx:&mut StepContext<'_>)->Result<DocumentRasterProgress,OriginalWorkFailure<DocumentSceneError>>{
  self.last_turn_work=0;self.last_turn_semantic_work=0;OriginalWorkFailure::qualify(cx)?;let mut work=RetainedWorkLedger::new(cx.retained_grant());let result=if cx.is_cancelled(){Err(DocumentSceneError::Cancelled)}else if cx.should_yield(){Ok(self.progress())}else{self.advance(1,&mut work)};
  match result{Ok(progress)=>{OriginalWorkFailure::<DocumentSceneError>::fold(cx,None,work.receipt(),self.last_turn_work)?;Ok(progress)},Err(error)=>Err(OriginalWorkFailure::fold(cx,Some(error),work.receipt(),self.last_turn_work).expect_err("an original body failure remains owned"))}
 }
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
  (DocumentRasterRetirement{vector:self.vector_retirement,owner:ControlledRetirement::retain(owners)},output)
 }
}
/// 🧺️ Source handback follows actual vector, raster and private candidate retirement.
#[derive(semio_framework_value::RetireOwned)]
#[value(send_only)]
struct DocumentRasterOwners{source:VecDeque<DocumentSceneNode>,assets:Vec<RasterSceneAsset>,nodes:Vec<RasterSceneNode>,retired_nodes:Vec<DocumentSceneNode>,retired_paths:Vec<Vec<u16>>,retired_texts:Vec<String>,raster:Option<crate::raster::RasterSceneRetirement>,handoff:Option<DocumentScenePlan>,output:Option<semio_framework_pixels::RasterImage>,failure:Option<DocumentSceneError>}
pub struct DocumentRasterRetirement<'a>{vector:Option<DocumentVectorRetirement<'a>>,owner:ControlledRetirement<DocumentRasterOwners>}
impl DocumentRasterRetirement<'_>{
 pub fn terminal_is_empty(&self)->bool{self.owner.terminal_is_empty()&&self.vector.as_ref().is_none_or(DocumentVectorRetirement::terminal_is_empty)}
 pub fn take_snapshot_read(&mut self)->Option<store::os_store::snapshot_read::SnapshotRead<DrawingSnapshot>>{if !self.owner.terminal_is_empty(){return None;}self.vector.as_mut()?.take_snapshot_read()}
 pub fn snapshot_authority_matches(&self,generation:u64,revision:[u8;32])->bool{self.vector.as_ref().is_some_and(|close|close.snapshot_authority_matches(generation,revision))}
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  let step=if !self.owner.terminal_is_empty(){self.owner.step(grant)?}else if let Some(vector)=&mut self.vector{vector.close_step(grant)?}else{RetainedCloneStep::Complete(Default::default())};
  let progress=step.progress();Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})
 }
 pub fn close_original(&mut self,grant:RetainedCloneGrant)->semio_framework_value::RetainedWorkOutcome{
  let turn=if !self.owner.terminal_is_empty(){let result=self.owner.step(grant);semio_framework_value::RetainedWorkOutcome{result,work:self.owner.last_turn_work()}}else if let Some(vector)=&mut self.vector{vector.close_original(grant)}else{semio_framework_value::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}};
  semio_framework_value::RetainedWorkOutcome{work:turn.work,result:turn.result.map(|step|{let progress=step.progress();if self.terminal_is_empty(){RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)}})}
 }
 pub fn next_copy_byte_demand(&self)->Result<usize,ValueError>{if !self.owner.terminal_is_empty(){self.owner.next_copy_byte_demand()}else{self.vector.as_ref().map_or(Ok(0),DocumentVectorRetirement::next_copy_byte_demand)}}
 pub fn next_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{if !self.owner.terminal_is_empty(){self.owner.next_capacity_byte_demand(body)}else{self.vector.as_ref().map_or(Ok(0),|vector|vector.next_capacity_byte_demand(body))}}
 pub fn next_release_byte_demand(&self)->Result<usize,ValueError>{if !self.owner.terminal_is_empty(){self.owner.next_release_byte_demand()}else{self.vector.as_ref().map_or(Ok(0),DocumentVectorRetirement::next_release_byte_demand)}}
 pub fn next_depth_demand(&self)->Result<usize,ValueError>{if !self.owner.terminal_is_empty(){self.owner.next_depth_demand()}else{self.vector.as_ref().map_or(Ok(0),DocumentVectorRetirement::next_depth_demand)}}

}
