//! 📋️ Complete authored documents become private typed plans under work grants.
use semio_framework_value::{list::PagedList, paged::Utf8Text};
use crate::{DrawingSnapshot,DrawingLayerNode,FillStyle,StrokeStyle,PathSegment,FillRule};
use crate::schema::{layer_base,shape_path_segment,drawing_transform_to_matrix,DrawingSceneGroup};
use crate::schema::scene_raster::{RasterSceneAsset,RasterSceneNode,RasterSceneContent,RasterSceneInput};
use crate::schema::scene_retirement::ScenePlanCloseJob;
use semio_framework_2d::retirement::{WorkRetirementCounter,WorkRetirementProgress};
pub type DocumentSceneRetirementProgress=WorkRetirementProgress;
pub type DocumentVectorRetirementProgress=WorkRetirementProgress;
use std::{collections::{BTreeMap,BTreeSet},sync::Arc};
#[path="🧭️selection/🦀️.rs"]
pub mod selection;
pub use selection::{scene_selection_relation,SceneSelectionRelation};
use crate::schema::scene_booleans::{DocumentBooleanInput,DocumentBooleanJob,DocumentBooleanLimits,DocumentBooleanProgress,DocumentBooleanRetirement};
use crate::schema::scene_trace::{DocumentTraceInput,DocumentTraceJob,DocumentTraceLimits,DocumentTraceProgress,DocumentTraceRetirement};
#[derive(Clone,Copy,Debug)]
pub struct DocumentSceneLimits{pub max_nodes:usize,pub max_depth:usize,pub max_segments:usize,pub max_references:usize,pub max_source_bytes:usize}
#[derive(Clone,Debug)]
pub enum DocumentSceneContent{
 Path{segments:Vec<PathSegment>,fill_rule:FillRule,fill:Option<FillStyle>,stroke:Option<StrokeStyle>},
 Image{asset:String,width:f64,height:f64},Group{children:Vec<String>,isolation:bool},
 Text{content:String,x:f64,y:f64,size:f64,fill_rule:FillRule,fill:Option<FillStyle>,stroke:Option<StrokeStyle>},
 Boolean{operation:String,children:Vec<String>,reference_transform:[f64;6],fill_rule:FillRule,fill:Option<FillStyle>,stroke:Option<StrokeStyle>},
 Trace{source:String,threshold:f64,simplify_epsilon:f64,fill_rule:FillRule,fill:Option<FillStyle>,stroke:Option<StrokeStyle>},
}
impl DocumentSceneContent{
 fn paint(&mut self)->Option<(&mut Option<FillStyle>,&mut Option<StrokeStyle>)>{match self{Self::Path{fill,stroke,..}|Self::Text{fill,stroke,..}|Self::Boolean{fill,stroke,..}|Self::Trace{fill,stroke,..}=>Some((fill,stroke)),_=>None}}
 fn children(&self)->&[String]{match self{Self::Group{children,..}|Self::Boolean{children,..}=>children,_=>&[]}}
}
#[derive(Clone,Debug)]
pub struct DocumentSceneNode{pub source_path:Vec<u16>,pub locked_ancestors:u32,pub id:String,pub groups:Vec<DrawingSceneGroup>,pub transform:[f64;6],pub opacity:f64,pub blend_mode:String,pub visible:bool,pub content:DocumentSceneContent}
#[derive(Clone,Debug,Default)]
pub struct DocumentScenePlan{pub assets:Vec<RasterSceneAsset>,pub nodes:Vec<DocumentSceneNode>}
/// 🧭️ Validate the bounded authored address and the exact unsigned lock-mask width.
pub fn validate_scene_source_address(path:&[u16],locks:u32)->Result<(),DocumentSceneError>{if path.is_empty()||path.len()>32||locks.checked_shr(path.len()as u32).is_some_and(|bits|bits!=0){return Err(invalid("Invalid scene source ancestry"));}Ok(())}
#[derive(Clone,Debug)]
pub struct DocumentSceneProgress{pub phase:&'static str,pub layers:usize,pub assets:usize,pub segments:usize,pub references:usize,pub source_bytes:usize,pub work:u64,pub done:bool}
#[derive(Clone,Debug,PartialEq,Eq)]
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
fn scene_layer_at<'d>(document:&'d DrawingSnapshot,path:&[usize])->Result<&'d DrawingLayerNode,DocumentSceneError>{let mut layers=&document.layers;let mut node=None;for(index,at)in path.iter().enumerate(){let next=layers.get(*at).ok_or_else(||invalid("Captured scene layer disappeared"))?;node=Some(next);if index+1<path.len(){let DrawingLayerNode::Group(group)=next else{return Err(invalid("Captured scene group changed"));};layers=&group.children;}}node.ok_or_else(||invalid("Missing scene cursor path"))}
fn scene_layers_at<'d>(document:&'d DrawingSnapshot,path:&[usize])->Result<&'d PagedList<DrawingLayerNode, {usize::MAX}>,DocumentSceneError>{if path.is_empty(){return Ok(&document.layers);}let DrawingLayerNode::Group(group)=scene_layer_at(document,path)? else{return Err(invalid("Captured scene group changed"));};Ok(&group.children)}
struct Frame{locked_ancestors:u32,path:Vec<usize>,at:usize,matrix:[f64;6],visible:bool,groups:Vec<DrawingSceneGroup>}
/// 🧱️ Retains source positions and copies each character or geometry entry under a grant.
struct DocumentSceneCursor{
 limits:DocumentSceneLimits,asset_key:Option<String>,asset_active:bool,asset_text:String,asset_char:usize,asset_chunk:usize,asset_chunk_offset:usize,
 frames:Vec<Frame>,current:Option<Vec<usize>>,node:Option<DocumentSceneNode>,plan:DocumentScenePlan,ids:BTreeMap<String,usize>,asset_ids:BTreeSet<String>,
 phase:&'static str,layers:usize,assets:usize,segments:usize,references:usize,source_bytes:usize,work:u64,at:usize,stroke_at:usize,text_at:usize,text_chars:usize,text_chunk:usize,text_chunk_offset:usize,
 validate_at:usize,ref_at:usize,cycle_at:usize,graph:Vec<(usize,usize)>,visited:BTreeSet<usize>,visiting:BTreeSet<usize>,cancelled:bool,failure:Option<DocumentSceneError>,cleanup_slot:u8,
}
impl DocumentSceneCursor{
 pub fn new(document:&DrawingSnapshot,limits:DocumentSceneLimits)->Result<Self,DocumentSceneError>{
  if !(1..=1024).contains(&limits.max_nodes)||!(1..=32).contains(&limits.max_depth)||!(1..=65536).contains(&limits.max_segments)||!(1..=32768).contains(&limits.max_references)||!(1..=268439552).contains(&limits.max_source_bytes)||document.layers.len()>limits.max_nodes||document.assets.len()>1024{return Err(invalid("Invalid scene preparation limits or document"));}
  Ok(Self{limits,asset_key:None,asset_active:false,asset_text:String::new(),asset_char:0,asset_chunk:0,asset_chunk_offset:0,frames:vec![Frame{locked_ancestors:0,path:Vec::new(),at:0,matrix:[1.0,0.0,0.0,1.0,0.0,0.0],visible:true,groups:Vec::new()}],current:None,node:None,plan:DocumentScenePlan::default(),ids:BTreeMap::new(),asset_ids:BTreeSet::new(),phase:"assets",layers:0,assets:0,segments:0,references:0,source_bytes:0,work:0,at:0,stroke_at:0,text_at:0,text_chars:0,text_chunk:0,text_chunk_offset:0,validate_at:0,ref_at:0,cycle_at:0,graph:Vec::new(),visited:BTreeSet::new(),visiting:BTreeSet::new(),cancelled:false,failure:None,cleanup_slot:0})
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
   DrawingLayerNode::Text(v)=>{if v.content.len()>262144{return Err(invalid("Scene text exceeds limit"));}DocumentSceneContent::Text{content:String::new(),x:coordinate(v.x)?,y:coordinate(v.y)?,size:positive(v.size)?,fill_rule:rule,fill:None,stroke:None}},
   DrawingLayerNode::Boolean(v)=>{if !crate::DRAWING_BOOLEAN_OPERATIONS.iter().any(|operation| v.operation.eq_str(operation))||v.children.len()>1024{return Err(invalid("Invalid boolean scene work"));}DocumentSceneContent::Boolean{operation:v.operation.to_string_owner(),children:Vec::new(),reference_transform:parent,fill_rule:rule,fill:None,stroke:None}},
   DrawingLayerNode::Trace(v)=>{id(&v.source_key)?;let epsilon=coordinate(v.params.simplify_epsilon)?;if epsilon<0.0{return Err(invalid("Invalid trace epsilon"));}DocumentSceneContent::Trace{source:v.source_key.to_string_owner(),threshold:unit(v.params.threshold)?,simplify_epsilon:epsilon,fill_rule:rule,fill:None,stroke:None}},
  };
  self.ids.insert(authored_id.clone(),self.plan.nodes.len());self.layers+=1;self.node=Some(DocumentSceneNode{source_path,locked_ancestors,id:authored_id,groups,transform,opacity,blend_mode:b.blend_mode.to_string_owner(),visible:visible&&b.visible,content});self.current=Some(path);self.at=0;self.stroke_at=0;self.text_at=0;self.text_chars=0;self.text_chunk=0;self.text_chunk_offset=0;self.phase="paint";Ok(())
 }
 fn paint(&mut self,document:&DrawingSnapshot)->Result<(),DocumentSceneError>{
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
 fn finish(&mut self,document:&DrawingSnapshot)->Result<(),DocumentSceneError>{
  if matches!(scene_layer_at(document,self.current.as_deref().unwrap())?,DrawingLayerNode::Group(group) if self.frames.len()>=self.limits.max_depth&&!group.children.is_empty()){return Err(invalid("Scene depth limit exceeded"));}
  let n=self.node.take().unwrap();
  if let DrawingLayerNode::Group(group)=scene_layer_at(document,self.current.as_deref().unwrap())?{
   let mut groups=n.groups.clone();if group.isolation||n.opacity!=1.0||n.blend_mode!="normal"{groups.push(DrawingSceneGroup{id:n.id.clone(),opacity:n.opacity,blend_mode:n.blend_mode.clone()});}
   self.frames.push(Frame{locked_ancestors:n.locked_ancestors,path:self.current.as_ref().unwrap().clone(),at:0,matrix:n.transform,visible:n.visible,groups});
  }
  self.plan.nodes.push(n);self.current=None;self.phase="layers";Ok(())
 }
 fn step(&mut self,document:&DrawingSnapshot)->Result<(),DocumentSceneError>{
  match self.phase{
   "assets"=>{
    if self.asset_active{
     let key=self.asset_key.as_ref().unwrap();let asset=document.assets.get(key).ok_or_else(||invalid("Captured scene asset disappeared"))?;
     if self.asset_char==asset.data.len(){self.plan.assets.push(RasterSceneAsset{id:key.clone(),mime:asset.mime.to_string_owner(),data:Arc::new(std::mem::take(&mut self.asset_text))});self.asset_ids.insert(key.clone());self.asset_active=false;self.assets+=1;return Ok(());}
     let Some(ch)=text_scalar(&asset.data,&mut self.asset_chunk,&mut self.asset_chunk_offset)? else{return Ok(());};self.asset_text.try_reserve(ch.len_utf8()).map_err(|_|invalid("Scene source allocation failed"))?;self.asset_text.push(ch);self.asset_char+=ch.len_utf8();self.source_bytes+=ch.len_utf8();return Ok(());
    }
    let next=document.assets.retained_entries().get(self.assets).map(|(key,asset)|(key,asset));
    if let Some((key,asset))=next{id(key)?;if asset.mime.is_empty()||asset.mime.len()>128||asset.data.is_empty()||asset.data.len()>self.limits.max_source_bytes.saturating_sub(self.source_bytes){return Err(invalid("Invalid source asset catalog or source byte limit"));}self.asset_key=Some(key.to_string_owner());self.asset_active=true;self.asset_char=0;self.asset_chunk=0;self.asset_chunk_offset=0;}else{self.phase="layers";}
   }
   "layers"=>{let Some(f)=self.frames.last_mut()else{self.phase="validation";return Ok(());};let layers=scene_layers_at(document,&f.path)?;if f.at==layers.len(){if f.groups.pop().is_none(){self.frames.pop();}return Ok(());}let layer=layers.get(f.at).ok_or_else(||invalid("Captured scene layer disappeared"))?;let mut path=f.path.clone();path.push(f.at);f.at+=1;let(parent,visible,groups,locks)=(f.matrix,f.visible,f.groups.clone(),f.locked_ancestors);self.start(layer,path,parent,visible,groups,locks)?;}
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
 pub fn advance(&mut self,document:&DrawingSnapshot,budget:usize)->Result<DocumentSceneProgress,DocumentSceneError>{if budget==0||budget as u128>9_007_199_254_740_991{return Err(invalid("Invalid scene preparation work grant"));}if self.cancelled{return Err(DocumentSceneError::Cancelled);}if let Some(error)=&self.failure{return Err(error.clone());}for _ in 0..budget{if self.phase=="complete"{break;}if let Err(error)=self.step(document){self.failure=Some(error.clone());return Err(error);}self.work+=1;}Ok(DocumentSceneProgress{phase:self.phase,layers:self.layers,assets:self.assets,segments:self.segments,references:self.references,source_bytes:self.source_bytes,work:self.work,done:self.phase=="complete"})}
 fn cleanup(&mut self)->bool{let complete=match self.cleanup_slot{0=>self.ids.pop_first().is_none(),1=>self.asset_ids.pop_first().is_none(),2=>self.visited.pop_first().is_none(),3=>self.visiting.pop_first().is_none(),4=>{self.graph=Vec::new();true},5=>{self.asset_key=None;true},6=>{self.asset_text=String::new();true},_=>unreachable!()};if complete{self.cleanup_slot+=1;}self.cleanup_slot==7}
 fn into_retirement(mut self)->(DocumentSceneRetirement,Option<DocumentScenePlan>){let output=if self.phase=="complete"&&!self.cancelled&&self.failure.is_none(){Some(std::mem::take(&mut self.plan))}else{None};let draft=if output.is_some(){None}else{Some(ScenePlanCloseJob::new(std::mem::take(&mut self.plan)))};let node=self.node.take().map(|node|ScenePlanCloseJob::new(DocumentScenePlan{assets:Vec::new(),nodes:vec![node]}));self.cancelled=true;(DocumentSceneRetirement{cursor:Some(self),draft,node,slot:0,counter:Default::default()},output)}
 fn clear(&mut self){self.asset_key=None;self.asset_active=false;self.asset_text=String::new();self.frames.clear();self.current=None;self.node=None;self.plan=DocumentScenePlan::default();self.ids.clear();self.asset_ids.clear();self.graph.clear();self.visited.clear();self.visiting.clear();}
 pub fn cancel(&mut self){self.cancelled=true;self.clear();}
 pub fn result(&mut self)->Result<DocumentScenePlan,DocumentSceneError>{if self.cancelled{return Err(DocumentSceneError::Cancelled);}if let Some(error)=&self.failure{return Err(error.clone());}if self.phase!="complete"{return Err(DocumentSceneError::Incomplete);}Ok(std::mem::take(&mut self.plan))}
}
/// 🧹️ Retains private preparation ownership until frames, scenes and indexes are empty.
pub struct DocumentSceneRetirement{cursor:Option<DocumentSceneCursor>,draft:Option<ScenePlanCloseJob>,node:Option<ScenePlanCloseJob>,slot:u8,counter:WorkRetirementCounter}
impl DocumentSceneRetirement{
 pub fn terminal_is_empty(&self)->bool{self.cursor.is_none()&&self.draft.is_none()&&self.node.is_none()}
 fn step(&mut self)->bool{let cursor=self.cursor.as_mut().unwrap();match self.slot{
  0=>if let Some(close)=&mut self.draft{if !close.advance(1).expect("positive scene cleanup grant").done{return false;}self.draft=None;},
  1=>if let Some(close)=&mut self.node{if !close.advance(1).expect("positive node cleanup grant").done{return false;}self.node=None;},
  2=>{if let Some(frame)=cursor.frames.last_mut(){if frame.groups.pop().is_none(){cursor.frames.pop();}return false;}cursor.frames=Vec::new();},
  3=>if cursor.ids.pop_first().is_some(){return false;},4=>if cursor.asset_ids.pop_first().is_some(){return false;},5=>if cursor.visited.pop_first().is_some(){return false;},6=>if cursor.visiting.pop_first().is_some(){return false;},
  7=>cursor.graph=Vec::new(),8=>cursor.current=None,9=>cursor.asset_key=None,10=>cursor.asset_text=String::new(),11=>cursor.plan=DocumentScenePlan::default(),_=>unreachable!()
 }self.slot+=1;if self.slot==12{self.cursor=None;}self.terminal_is_empty()}
 pub fn advance(&mut self,grant:usize)->Result<DocumentSceneRetirementProgress,DocumentSceneError>{let mut counter=self.counter;let result=counter.advance(grant,||self.step());self.counter=counter;result.map_err(invalid)}
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
mod tests;
#[derive(Clone,Debug)]
pub struct DocumentSceneViewport{pub width:u32,pub height:u32,pub origin:[f64;2],pub tolerance:f64,pub max_pixels:usize,pub max_source_bytes:usize,pub max_bytes:usize,pub max_chunks:usize}
impl DocumentSceneViewport{fn input(&self,assets:Vec<RasterSceneAsset>,nodes:Vec<RasterSceneNode>)->RasterSceneInput{RasterSceneInput{width:self.width,height:self.height,origin:self.origin,tolerance:self.tolerance,max_pixels:self.max_pixels,max_source_bytes:self.max_source_bytes,max_bytes:self.max_bytes,max_chunks:self.max_chunks,assets,nodes}}}
#[derive(Clone,Debug)]
pub struct DocumentRasterProgress{pub phase:&'static str,pub preparation:DocumentSceneProgress,pub tracing:Option<DocumentTraceProgress>,pub resolution:Option<DocumentBooleanProgress>,pub raster:Option<crate::schema::scene_raster::RasterSceneProgress>,pub nodes:usize,pub work:u64,pub done:bool}
#[derive(Clone,Copy,Debug)]
pub struct DocumentAlgorithmLimits{pub booleans:DocumentBooleanLimits,pub trace:DocumentTraceLimits,pub max_work:u64}
#[derive(Clone,Debug)]
pub struct DocumentVectorProgress{pub phase:&'static str,pub preparation:DocumentSceneProgress,pub tracing:Option<DocumentTraceProgress>,pub resolution:Option<DocumentBooleanProgress>,pub work:u64,pub done:bool}
impl Default for DocumentVectorProgress{fn default()->Self{Self{phase:"preparing",preparation:DocumentSceneProgress{phase:"assets",layers:0,assets:0,segments:0,references:0,source_bytes:0,work:0,done:false},tracing:None,resolution:None,work:0,done:false}}}
enum DocumentVectorSource<'a>{Borrowed(&'a DrawingSnapshot),SnapshotRead(store::SnapshotRead<DrawingSnapshot>)}
impl DocumentVectorSource<'_>{fn document(&self)->&DrawingSnapshot{match self{Self::Borrowed(document)=>document,Self::SnapshotRead(read)=>read.get()}}}
enum VectorChildRetirement{Preparation(DocumentSceneRetirement),Trace(DocumentTraceRetirement),Boolean(DocumentBooleanRetirement)}
impl VectorChildRetirement{fn advance(&mut self,grant:usize)->Result<WorkRetirementProgress,DocumentSceneError>{match self{Self::Preparation(job)=>job.advance(grant),Self::Trace(job)=>job.advance(grant),Self::Boolean(job)=>job.advance(grant)}}}
/// 🎬️ Resolves one stable source while retaining text and authored paint metadata.
pub struct DocumentVectorJob<'a>{
 source:Option<DocumentVectorSource<'a>>,preparation:Option<DocumentSceneCursor>,traces:Option<DocumentTraceJob>,algorithms:Option<DocumentBooleanJob>,progress:DocumentVectorProgress,
 output:Option<DocumentScenePlan>,failure:Option<DocumentSceneError>,cancelled:bool,limits:DocumentAlgorithmLimits,closing:Option<VectorChildRetirement>,discard:Option<ScenePlanCloseJob>,handoff:Option<DocumentScenePlan>,
}
impl<'a> DocumentVectorJob<'a>{
 pub fn new(document:&'a DrawingSnapshot,limits:DocumentSceneLimits,algorithms:DocumentAlgorithmLimits)->Result<Self,DocumentSceneError>{Self::from_source(DocumentVectorSource::Borrowed(document),limits,algorithms)}
 fn from_source(source:DocumentVectorSource<'a>,limits:DocumentSceneLimits,algorithms:DocumentAlgorithmLimits)->Result<Self,DocumentSceneError>{
  if !(1..=1000000000).contains(&algorithms.max_work){return Err(invalid("Invalid document vector work limit"));}
  let mut check=DocumentBooleanJob::new(DocumentBooleanInput{plan:DocumentScenePlan::default(),limits:algorithms.booleans})?;check.cancel();
  let mut check=DocumentTraceJob::new(DocumentTraceInput{plan:DocumentScenePlan::default(),limits:algorithms.trace})?;check.cancel();
  Ok(Self{preparation:Some(DocumentSceneCursor::new(source.document(),limits)?),source:Some(source),traces:None,algorithms:None,progress:DocumentVectorProgress::default(),output:None,failure:None,cancelled:false,limits:algorithms,closing:None,discard:None,handoff:None})
 }

 /// 🔐️ Owns the genuine immutable store read while preparation retains only source positions.
 pub fn from_snapshot_read(read:store::SnapshotRead<DrawingSnapshot>,limits:DocumentSceneLimits,algorithms:DocumentAlgorithmLimits)->Result<DocumentVectorJob<'static>,DocumentSceneError>{DocumentVectorJob::from_source(DocumentVectorSource::SnapshotRead(read),limits,algorithms)}
 /// 🧾️ Returns the exact source lease after all source traversal reaches a terminal state.
 pub fn take_snapshot_read(&mut self)->Option<store::SnapshotRead<DrawingSnapshot>>{if self.preparation.is_some()||self.traces.is_some()||self.algorithms.is_some()||self.closing.is_some()||self.discard.is_some()||self.handoff.is_some(){return None;}match self.source.take(){Some(DocumentVectorSource::SnapshotRead(read))=>Some(read),other=>{self.source=other;None}}}
 /// 🛂️ Checks the captured generation and revision against the store's current event authority.
 pub fn snapshot_authority_matches(&self,generation:u64,revision:[u8;32])->bool{match &self.source{Some(DocumentVectorSource::SnapshotRead(read))=>read.commit_authority_matches(generation,revision),_=>false}}
 fn step(&mut self)->Result<(),DocumentSceneError>{
  if let Some(close)=&mut self.closing{if close.advance(1)?.done{self.closing=None;}return Ok(());}
  if let Some(close)=&mut self.discard{if close.advance(1)?.done{self.discard=None;}return Ok(());}
  if let Some(plan)=self.handoff.take(){match self.progress.phase{"preparing"=>{self.traces=Some(DocumentTraceJob::new(DocumentTraceInput{plan,limits:self.limits.trace})?);self.progress.phase="tracing";},"tracing"=>{self.algorithms=Some(DocumentBooleanJob::new(DocumentBooleanInput{plan,limits:self.limits.booleans})?);self.progress.phase="algorithms";},_=>{self.output=Some(plan);self.progress.phase="complete";self.progress.done=true;}}return Ok(());}
  match self.progress.phase{
   "preparing"=>{self.progress.preparation=self.preparation.as_mut().unwrap().advance(self.source.as_ref().unwrap().document(),1)?;if self.progress.preparation.done{let(close,plan)=self.preparation.take().unwrap().into_retirement();self.closing=Some(VectorChildRetirement::Preparation(close));self.handoff=plan;}}
   "tracing"=>{let p=self.traces.as_mut().unwrap().advance(1)?;self.progress.tracing=Some(p);if p.done{let(close,input,plan)=self.traces.take().unwrap().into_retirement();self.closing=Some(VectorChildRetirement::Trace(close));self.discard=Some(ScenePlanCloseJob::new(input));self.handoff=plan;}}
   "algorithms"=>{let p=self.algorithms.as_mut().unwrap().advance(1)?;self.progress.resolution=Some(p);if p.done{let(close,input,plan)=self.algorithms.take().unwrap().into_retirement();self.closing=Some(VectorChildRetirement::Boolean(close));self.discard=Some(ScenePlanCloseJob::new(input));self.handoff=plan;}}
   _=>{}
  }Ok(())
 }
 pub fn advance(&mut self,budget:usize)->Result<DocumentVectorProgress,DocumentSceneError>{
  if budget==0||budget as u128>9_007_199_254_740_991{return Err(invalid("Invalid document vector work grant"));}if self.cancelled{return Err(DocumentSceneError::Cancelled);}if let Some(e)=&self.failure{return Err(e.clone());}
  for _ in 0..budget{if self.progress.done{break;}let result=if self.progress.work>=self.limits.max_work{Err(invalid("Document vector work limit exceeded"))}else{self.step()};if let Err(e)=result{self.failure=Some(e.clone());return Err(e);}self.progress.work+=1;}Ok(self.progress.clone())
 }
 /// 🧹️ Retain actual source authority while all child and intermediate plan owners drain.
 pub fn into_retirement(mut self)->(DocumentVectorRetirement<'a>,Option<DocumentScenePlan>){
  let mut child=self.closing.take();let mut plans=Vec::new();if let Some(discard)=self.discard.take(){plans.push(discard);}
  if let Some(preparation)=self.preparation.take(){let(close,output)=preparation.into_retirement();child=Some(VectorChildRetirement::Preparation(close));if let Some(plan)=output{plans.push(ScenePlanCloseJob::new(plan));}}
  if let Some(trace)=self.traces.take(){let(close,input,output)=trace.into_retirement();child=Some(VectorChildRetirement::Trace(close));plans.push(ScenePlanCloseJob::new(input));if let Some(plan)=output{plans.push(ScenePlanCloseJob::new(plan));}}
  if let Some(algorithms)=self.algorithms.take(){let(close,input,output)=algorithms.into_retirement();child=Some(VectorChildRetirement::Boolean(close));plans.push(ScenePlanCloseJob::new(input));if let Some(plan)=output{plans.push(ScenePlanCloseJob::new(plan));}}
  if let Some(plan)=self.handoff.take(){plans.push(ScenePlanCloseJob::new(plan));}let output=if self.progress.done&&!self.cancelled&&self.failure.is_none(){self.output.take()}else{None};if let Some(plan)=self.output.take(){plans.push(ScenePlanCloseJob::new(plan));}
  (DocumentVectorRetirement{source:self.source.take(),child,plans,slot:0,counter:Default::default()},output)
 }
 fn clear(&mut self){if let Some(p)=&mut self.preparation{p.cancel();}if let Some(t)=&mut self.traces{t.cancel();}if let Some(a)=&mut self.algorithms{a.cancel();}self.preparation=None;self.traces=None;self.algorithms=None;self.closing=None;self.discard=None;self.handoff=None;self.output=None;}
 pub fn cancel(&mut self){self.cancelled=true;self.clear();}
 pub fn result(&mut self)->Result<DocumentScenePlan,DocumentSceneError>{if self.cancelled{return Err(DocumentSceneError::Cancelled);}if let Some(e)=&self.failure{return Err(e.clone());}if !self.progress.done{return Err(DocumentSceneError::Incomplete);}self.output.take().ok_or(DocumentSceneError::Incomplete)}
}
/// 🔐️ Private work completes before the genuine source read may return; terminal state includes handback.
pub struct DocumentVectorRetirement<'a>{source:Option<DocumentVectorSource<'a>>,child:Option<VectorChildRetirement>,plans:Vec<ScenePlanCloseJob>,slot:u8,counter:WorkRetirementCounter}
impl DocumentVectorRetirement<'_>{
 fn private_is_empty(&self)->bool{self.slot==3&&self.child.is_none()&&self.plans.is_empty()}
 pub fn terminal_is_empty(&self)->bool{self.private_is_empty()&&self.source.is_none()}
 /// 🧾️ Release the actual source lease only after every private owner reaches terminal state.
 pub fn take_snapshot_read(&mut self)->Option<store::SnapshotRead<DrawingSnapshot>>{if !self.private_is_empty(){return None;}match self.source.take(){Some(DocumentVectorSource::SnapshotRead(read))=>Some(read),other=>{self.source=other;None}}}
 fn step(&mut self)->bool{match self.slot{0=>if let Some(child)=&mut self.child{if !child.advance(1).expect("positive vector child cleanup grant").done{return false;}self.child=None;},1=>{if let Some(plan)=self.plans.last_mut(){if plan.advance(1).expect("positive vector plan cleanup grant").done{self.plans.pop();}return false;}self.plans=Vec::new();},2=>{if matches!(self.source,Some(DocumentVectorSource::Borrowed(_))){self.source=None;}},_=>unreachable!()}self.slot+=1;self.private_is_empty()}
 pub fn advance(&mut self,grant:usize)->Result<DocumentVectorRetirementProgress,DocumentSceneError>{let mut counter=self.counter;let result=counter.advance(grant,||self.step());self.counter=counter;result.map_err(invalid)}
}
/// 📷️ Resolves each raster record after the common document vector producer completes.
pub struct DocumentRasterJob<'a>{
 vector:Option<DocumentVectorJob<'a>>,prepared:DocumentVectorProgress,source:std::vec::IntoIter<DocumentSceneNode>,assets:Vec<RasterSceneAsset>,nodes:Vec<RasterSceneNode>,at:usize,work:u64,phase:&'static str,
 raster:Option<crate::schema::scene_raster::RasterSceneJob>,rendered:Option<crate::schema::scene_raster::RasterSceneProgress>,output:Option<semio_framework_pixels::RasterImage>,failure:Option<DocumentSceneError>,cancelled:bool,viewport:DocumentSceneViewport,max_work:u64,
}
impl<'a> DocumentRasterJob<'a>{
 pub fn new(document:&'a DrawingSnapshot,limits:DocumentSceneLimits,viewport:DocumentSceneViewport,resolution_limits:DocumentAlgorithmLimits)->Result<Self,DocumentSceneError>{
  let mut check=crate::schema::scene_raster::RasterSceneJob::new(viewport.input(Vec::new(),Vec::new())).map_err(|e|invalid(e.to_string()))?;check.cancel();
  Ok(Self{vector:Some(DocumentVectorJob::new(document,limits,resolution_limits)?),prepared:DocumentVectorProgress::default(),source:Vec::new().into_iter(),assets:Vec::new(),nodes:Vec::new(),at:0,work:0,phase:"preparing",raster:None,rendered:None,output:None,failure:None,cancelled:false,viewport,max_work:resolution_limits.max_work})
 }
 fn step(&mut self)->Result<(),DocumentSceneError>{
  if let Some(vector)=&mut self.vector{self.prepared=vector.advance(1)?;if self.prepared.done{let plan=vector.result()?;self.source=plan.nodes.into_iter();self.assets=plan.assets;self.vector=None;self.phase="resolving";}else{self.phase=self.prepared.phase;}return Ok(());}
  match self.phase{
   "resolving"=>{if let Some(n)=self.source.next(){self.at+=1;let content=match n.content{DocumentSceneContent::Group{..}=>return Ok(()),DocumentSceneContent::Path{segments,fill_rule,fill,stroke}=>RasterSceneContent::Path{segments,fill_rule,fill,stroke},DocumentSceneContent::Image{asset,width,height}=>RasterSceneContent::Image{asset,width,height},_=>return Err(invalid(format!("Unresolved scene layer: {}",n.id)))};self.nodes.push(RasterSceneNode{id:n.id,groups:n.groups,transform:n.transform,opacity:n.opacity,blend_mode:n.blend_mode,visible:n.visible,content});}else{self.raster=Some(crate::schema::scene_raster::RasterSceneJob::new(self.viewport.input(std::mem::take(&mut self.assets),std::mem::take(&mut self.nodes))).map_err(|e|invalid(e.to_string()))?);self.phase="raster";}}
   "raster"=>{let p=self.raster.as_mut().unwrap().advance(1).map_err(|e|invalid(e.to_string()))?;let done=p.done;self.rendered=Some(p);if done{self.output=Some(self.raster.take().unwrap().into_result().map_err(|e|invalid(e.to_string()))?);self.phase="complete";}}
   _=>{}
  }Ok(())
 }
 pub fn advance(&mut self,budget:usize)->Result<DocumentRasterProgress,DocumentSceneError>{if budget==0||budget as u128>9_007_199_254_740_991{return Err(invalid("Invalid document raster work grant"));}if self.cancelled{return Err(DocumentSceneError::Cancelled);}if let Some(e)=&self.failure{return Err(e.clone());}for _ in 0..budget{if self.phase=="complete"{break;}let result=if self.work>=self.max_work{Err(invalid("Document raster work limit exceeded"))}else{self.step()};if let Err(e)=result{self.clear();self.failure=Some(e.clone());return Err(e);}self.work+=1;}Ok(DocumentRasterProgress{phase:self.phase,preparation:self.prepared.preparation.clone(),tracing:self.prepared.tracing,resolution:self.prepared.resolution,raster:self.rendered.clone(),nodes:self.at,work:self.work,done:self.phase=="complete"})}
 fn clear(&mut self){if let Some(v)=&mut self.vector{v.cancel();}if let Some(r)=&mut self.raster{r.cancel();}self.vector=None;self.raster=None;self.source=Vec::new().into_iter();self.assets.clear();self.nodes.clear();self.output=None;}
 pub fn cancel(&mut self){self.cancelled=true;self.clear();}
 pub fn result(&mut self)->Result<semio_framework_pixels::RasterImage,DocumentSceneError>{if self.cancelled{return Err(DocumentSceneError::Cancelled);}if let Some(e)=&self.failure{return Err(e.clone());}if self.phase!="complete"{return Err(DocumentSceneError::Incomplete);}self.output.take().ok_or(DocumentSceneError::Incomplete)}
}
