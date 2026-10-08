//! 🧹️ Consumes complete scene plans one shallow owner or container entry per grant.
use crate::{FillStyle,StrokeStyle,PathSegment,GradientStop};
use semio_framework_value::list::PagedList;
use crate::schema::DrawingSceneGroup;
use crate::schema::scene_preparation::{DocumentScenePlan,DocumentSceneNode,DocumentSceneContent,DocumentSceneError};
use crate::schema::scene_raster::RasterSceneAsset;
use std::{sync::Arc,vec::IntoIter};
enum Owner{
 Plan(DocumentScenePlan),Assets(IntoIter<RasterSceneAsset>),Asset(RasterSceneAsset),Nodes(IntoIter<DocumentSceneNode>),Node(DocumentSceneNode),
 Groups(IntoIter<DrawingSceneGroup>),Group(DrawingSceneGroup),Content(DocumentSceneContent),Strings(IntoIter<String>),String(String),Source(Arc<semio_framework_pixels::RasterImage>),
 Address(Vec<u16>),Segments(Vec<PathSegment>),Fill(FillStyle),Stroke(StrokeStyle),Stops(PagedList<GradientStop,{usize::MAX}>),Dash(PagedList<f64,{usize::MAX}>),
}
#[derive(Clone,Copy,Debug)]
pub struct ScenePlanCloseProgress{pub phase:&'static str,pub owners:u64,pub work:u64,pub done:bool}
/// 🧹️ Owns the transferred plan until every deep container reaches an empty shell.
pub struct ScenePlanCloseJob{stack:Vec<Owner>,owners:u64,work:u64}
impl ScenePlanCloseJob{
 pub fn new(plan:DocumentScenePlan)->Self{let mut stack=Vec::with_capacity(12);stack.push(Owner::Plan(plan));Self{stack,owners:0,work:0}}
 fn paint(&mut self,fill:Option<FillStyle>,stroke:Option<StrokeStyle>){if let Some(value)=fill{self.stack.push(Owner::Fill(value));}if let Some(value)=stroke{self.stack.push(Owner::Stroke(value));}}
 fn step(&mut self)->bool{
  match self.stack.pop().unwrap(){
   Owner::Plan(p)=>{self.stack.push(Owner::Assets(p.assets.into_iter()));self.stack.push(Owner::Nodes(p.nodes.into_iter()));},
   Owner::Assets(mut entries)=>{if let Some(value)=entries.next_back(){self.stack.push(Owner::Assets(entries));self.stack.push(Owner::Asset(value));return false;}},
   Owner::Nodes(mut entries)=>{if let Some(value)=entries.next_back(){self.stack.push(Owner::Nodes(entries));self.stack.push(Owner::Node(value));return false;}},
   Owner::Groups(mut entries)=>{if let Some(value)=entries.next_back(){self.stack.push(Owner::Groups(entries));self.stack.push(Owner::Group(value));return false;}},
   Owner::Strings(mut entries)=>{if let Some(value)=entries.next_back(){self.stack.push(Owner::Strings(entries));self.stack.push(Owner::String(value));return false;}},
   Owner::Asset(a)=>{self.stack.push(Owner::String(a.id));self.stack.push(Owner::Source(a.image));},
   Owner::Node(n)=>{self.stack.push(Owner::Address(n.source_path));self.stack.push(Owner::String(n.id));self.stack.push(Owner::String(n.blend_mode));self.stack.push(Owner::Groups(n.groups.into_iter()));self.stack.push(Owner::Content(n.content));},
   Owner::Group(g)=>{self.stack.push(Owner::String(g.id));self.stack.push(Owner::String(g.blend_mode));},
   Owner::Content(c)=>match c{
    DocumentSceneContent::Path{segments,fill,stroke,..}=>{self.stack.push(Owner::Segments(segments));self.paint(fill,stroke);},
    DocumentSceneContent::Image{asset,..}=>self.stack.push(Owner::String(asset)),
    DocumentSceneContent::Group{children,..}=>self.stack.push(Owner::Strings(children.into_iter())),
    DocumentSceneContent::Text{content,fill,stroke,..}=>{self.stack.push(Owner::String(content));self.paint(fill,stroke);},
    DocumentSceneContent::Boolean{operation,children,fill,stroke,..}=>{self.stack.push(Owner::String(operation));self.stack.push(Owner::Strings(children.into_iter()));self.paint(fill,stroke);},
    DocumentSceneContent::Trace{source,fill,stroke,..}=>{self.stack.push(Owner::String(source));self.paint(fill,stroke);},
   },
   Owner::Fill(FillStyle::LinearGradient{stops,..}|FillStyle::RadialGradient{stops,..})=>self.stack.push(Owner::Stops(stops)),
   Owner::Stroke(s)=>{if let Some(dash)=s.dash{self.stack.push(Owner::Dash(dash));}},
   Owner::Address(value)=>drop(value),Owner::String(value)=>drop(value),Owner::Source(value)=>drop(value),Owner::Segments(value)=>drop(value),Owner::Stops(value)=>drop(value),Owner::Dash(value)=>drop(value),Owner::Fill(_)=>{},
  }true
 }
 pub fn advance(&mut self,grant:usize)->Result<ScenePlanCloseProgress,DocumentSceneError>{
  if grant==0||grant as u128>9_007_199_254_740_991{return Err(DocumentSceneError::Invalid("Invalid scene retirement work grant".into()));}
  for _ in 0..grant{if self.stack.is_empty(){break;}if self.step(){self.owners+=1;}self.work+=1;}
  let done=self.terminal_is_empty();Ok(ScenePlanCloseProgress{phase:if done{"complete"}else{"closing"},owners:self.owners,work:self.work,done})
 }
 pub fn terminal_is_empty(&self)->bool{self.stack.is_empty()}
}
#[cfg(test)]
#[path="🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
