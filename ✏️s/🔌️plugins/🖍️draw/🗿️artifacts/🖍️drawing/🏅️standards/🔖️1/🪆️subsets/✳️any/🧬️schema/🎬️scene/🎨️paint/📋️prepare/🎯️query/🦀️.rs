//! 🖱️ Pointer selection borrows one complete cache and retains only bounded query metadata.
use super::{PreparedScene,PreparedPaint};
use super::super::{PreparedRegionStamp,PreparedRegionQueryJob};
use crate::schema::{scene_identity::SceneIdentity,scene_preparation::DocumentSceneNode};
#[path="🪢️lasso/🦀️.rs"]
mod lasso;
use lasso::{SegmentEnclosureCursor,LassoInteriorCursor};
#[derive(Clone,Copy,Debug,PartialEq)]
pub struct PreparedSceneIdentity{pub source:SceneIdentity,pub build:u64,pub flatness:f64}
impl PreparedSceneIdentity{
 fn stamp(self,entry:usize)->PreparedRegionStamp{PreparedRegionStamp{source:self.source,build:self.build,entry,flatness:self.flatness}}
 fn valid(self)->bool{self.stamp(0).valid()}
 fn matches(self,live:Self)->bool{self.valid()&&live.valid()&&self==live}
}
#[derive(Clone,Debug)]
pub enum PreparedScenePick{Point{point:[f64;2],tolerance:f64,required_flatness:f64},Rectangle{start:[f64;2],end:[f64;2],crossing:bool},Lasso{points:Vec<[f64;2]>}}
#[derive(Clone,Copy,Debug)]
pub struct PreparedScenePickProgress{pub phase:&'static str,pub work:u64,pub done:bool}
fn visible(node:&DocumentSceneNode)->bool{node.visible&&node.opacity>0.0&&node.groups.iter().all(|group|group.opacity>0.0)}
fn coordinate(n:f64)->bool{n.is_finite()&&n.abs()<=1e9}
fn point(p:[f64;2])->bool{p.into_iter().all(coordinate)}
fn segment_crosses_rectangle(a:[f64;2],b:[f64;2],min:[f64;2],max:[f64;2])->bool{let(mut lo,mut hi)=(0.0_f64,1.0_f64);for axis in 0..2{let d=b[axis]-a[axis];if d==0.0{if a[axis]<min[axis]||a[axis]>max[axis]{return false;}}else{let t0=(min[axis]-a[axis])/d;let t1=(max[axis]-a[axis])/d;lo=lo.max(t0.min(t1));hi=hi.min(t0.max(t1));if lo>hi{return false;}}}true}
fn quad_contains(quad:[[f64;2];4],point:[f64;2],tolerance:f64)->bool{let mut winding=0;for i in 0..4{let a=quad[i];let b=quad[(i+1)%4];let rounding=a.into_iter().chain(b).chain(point).fold(1.0_f64,|n,v|n.max(v.abs()))*f64::EPSILON*8.0;if super::super::distance(point,a,b)<=tolerance+rounding{return true;}if(a[1]<=point[1]&&b[1]>point[1])||(b[1]<=point[1]&&a[1]>point[1]){let t=(point[1]-a[1])/(b[1]-a[1]);if a[0]*(1.0-t)+b[0]*t>point[0]{winding+=1;}}}winding%2!=0}
struct RectangleCursor{min:[f64;2],max:[f64;2],slot:usize,contour:usize,edge:usize}
struct LassoCursor{slot:usize,contour:usize,edge:usize,seen:bool,segment:Option<SegmentEnclosureCursor>,interior:Option<LassoInteriorCursor>}
/// 🎯️ Partial candidate indices stay private until the exact cache query is complete.
pub struct PreparedScenePickJob{identity:PreparedSceneIdentity,query:PreparedScenePick,max_hits:usize,next:Option<usize>,active:usize,path:Option<PreparedRegionQueryJob>,rectangle:Option<RectangleCursor>,lasso_bounds:Option<[f64;4]>,lasso:Option<LassoCursor>,hits:Vec<usize>,phase:&'static str,work:u64,cancelled:bool,failure:Option<String>}
impl PreparedScenePickJob{
 pub fn new(identity:PreparedSceneIdentity,query:PreparedScenePick,max_hits:usize)->Result<Self,String>{
  if !identity.valid()||max_hits==0||max_hits>256{return Err("Invalid prepared scene query contract".into());}
  match &query{PreparedScenePick::Point{point:p,tolerance,required_flatness}=>{PreparedRegionQueryJob::new(identity.stamp(0),*p,*tolerance,*required_flatness)?;},PreparedScenePick::Rectangle{start,end,..}=>{if !point(*start)||!point(*end){return Err("Invalid prepared scene rectangle".into());}},PreparedScenePick::Lasso{points}=>{if !(3..=256).contains(&points.len())||!points.iter().copied().all(point){return Err("Invalid prepared scene lasso".into());}}}
  let lasso_bounds=if let PreparedScenePick::Lasso{points}=&query{Some(points.iter().fold([f64::INFINITY,f64::INFINITY,f64::NEG_INFINITY,f64::NEG_INFINITY],|[x,y,r,b],p|[x.min(p[0]),y.min(p[1]),r.max(p[0]),b.max(p[1])]))}else{None};
  Ok(Self{identity,query,max_hits,next:None,active:0,path:None,rectangle:None,lasso_bounds,lasso:None,hits:Vec::new(),phase:"nodes",work:0,cancelled:false,failure:None})
 }
 fn authority(&self,live:PreparedSceneIdentity)->Result<(),String>{if self.cancelled{return Err("Prepared scene query cancelled".into());}if let Some(error)=&self.failure{return Err(error.clone());}if !self.identity.matches(live){return Err("Prepared scene query authority changed".into());}Ok(())}
 fn admit(&mut self,index:usize)->Result<(),String>{if self.hits.len()>=self.max_hits{return Err("Prepared scene query exceeds result capacity".into());}self.hits.push(index);Ok(())}
 fn step(&mut self,scene:&PreparedScene,live:PreparedSceneIdentity)->Result<(),String>{
  if self.phase=="paint"{
   if let Some(cursor)=self.lasso.as_mut(){let PreparedScenePick::Lasso{points}=&self.query else{return Err("Prepared scene lasso owner changed".into());};let paint=&scene.geometry[self.active].paint;
    if let PreparedPaint::Path(regions)=paint{if regions.flatness!=live.flatness{return Err("Prepared scene cache precision changed".into());}}
    if let Some(interior)=cursor.interior.as_mut(){if let Some(enclosed)=interior.advance(points,paint){self.lasso=None;self.phase="nodes";if enclosed{self.admit(self.active)?;}}return Ok(());}
    if let Some(segment)=cursor.segment.as_mut(){match segment.advance(points){Some(false)=>{self.lasso=None;self.phase="nodes";},Some(true)=>{cursor.segment=None;cursor.edge+=1;},None=>{}}return Ok(());}
    if cursor.slot==if matches!(paint,PreparedPaint::Path(_)){2}else{1}{if cursor.seen{cursor.interior=Some(LassoInteriorCursor::new(scene.geometry[self.active].bounds.unwrap()));}else{self.lasso=None;self.phase="nodes";}return Ok(());}
    let contour=match paint{PreparedPaint::Path(regions)=>(if cursor.slot==0{&regions.fill}else{&regions.stroke}).get(cursor.contour).map(Vec::as_slice),PreparedPaint::Image(quad)=>if cursor.contour==0{Some(quad.as_slice())}else{None},PreparedPaint::Empty=>None};
    let Some(contour)=contour else{cursor.slot+=1;cursor.contour=0;cursor.edge=0;return Ok(());};if cursor.edge==contour.len(){cursor.contour+=1;cursor.edge=0;return Ok(());}cursor.segment=Some(SegmentEnclosureCursor::new(contour[cursor.edge],contour[(cursor.edge+1)%contour.len()]));cursor.seen=true;return Ok(());
   }
   let stamp=live.stamp(self.active);let PreparedPaint::Path(regions)=&scene.geometry[self.active].paint else{return Err("Prepared scene cache entry changed".into());};if regions.flatness!=live.flatness{return Err("Prepared scene cache precision changed".into());}
   if let Some(query)=self.path.as_mut(){if query.advance(regions,stamp,1)?.done{let hit=query.result(stamp)?;self.path=None;if hit{self.admit(self.active)?;self.phase=if self.rectangle.is_some(){"nodes"}else{"complete"};self.rectangle=None;}else if self.rectangle.is_none(){self.phase="nodes";}}return Ok(());}
   let q=self.rectangle.as_mut().unwrap();if q.slot==2{self.rectangle=None;self.phase="nodes";return Ok(());}let contours=if q.slot==0{&regions.fill}else{&regions.stroke};let Some(c)=contours.get(q.contour)else{q.slot+=1;q.contour=0;q.edge=0;return Ok(());};if q.edge>=c.len(){q.contour+=1;q.edge=0;return Ok(());}let hit=segment_crosses_rectangle(c[q.edge],c[(q.edge+1)%c.len()],q.min,q.max);q.edge+=1;if hit{self.admit(self.active)?;self.rectangle=None;self.phase="nodes";}return Ok(());
  }
  let Some(next)=self.next else{self.next=Some(scene.plan.nodes.len());return Ok(())};
  if next==0{self.phase="complete";return Ok(());}let index=next-1;self.next=Some(index);let node=&scene.plan.nodes[index];if !visible(node)||node.locked_ancestors!=0{return Ok(());}let geometry=&scene.geometry[index];
  match &self.query{
   PreparedScenePick::Point{point,tolerance,required_flatness}=>match &geometry.paint{PreparedPaint::Empty=>{},PreparedPaint::Path(_)=>{self.path=Some(PreparedRegionQueryJob::new(live.stamp(index),*point,*tolerance,*required_flatness)?);self.active=index;self.phase="paint";},PreparedPaint::Image(quad)=>{if quad_contains(*quad,*point,*tolerance){self.admit(index)?;self.phase="complete";}}},
   PreparedScenePick::Rectangle{start,end,crossing}=>{if let Some([x,y,r,b])=geometry.bounds{let min=[start[0].min(end[0]),start[1].min(end[1])];let max=[start[0].max(end[0]),start[1].max(end[1])];if !crossing{if x>=min[0]&&y>=min[1]&&r<=max[0]&&b<=max[1]{self.admit(index)?;}}else if min[0]<=r&&max[0]>=x&&min[1]<=b&&max[1]>=y{match &geometry.paint{PreparedPaint::Empty=>{},PreparedPaint::Path(_)=>{self.rectangle=Some(RectangleCursor{min,max,slot:0,contour:0,edge:0});self.path=Some(PreparedRegionQueryJob::new(live.stamp(index),min,0.0,live.flatness)?);self.active=index;self.phase="paint";},PreparedPaint::Image(quad)=>{if quad_contains(*quad,min,0.0)||(0..4).any(|i|segment_crosses_rectangle(quad[i],quad[(i+1)%4],min,max)){self.admit(index)?;}}}}}},
   PreparedScenePick::Lasso{..}=>{if let Some([x,y,r,b])=geometry.bounds{let[min_x,min_y,max_x,max_y]=self.lasso_bounds.unwrap();if x>=min_x&&y>=min_y&&r<=max_x&&b<=max_y{self.lasso=Some(LassoCursor{slot:0,contour:0,edge:0,seen:false,segment:None,interior:None});self.active=index;self.phase="paint";}}}
  }Ok(())
 }
 pub fn advance(&mut self,scene:&PreparedScene,live:PreparedSceneIdentity,grant:usize)->Result<PreparedScenePickProgress,String>{
  if grant==0||grant as u128>9007199254740991{return Err("Invalid prepared scene query work grant".into());}self.authority(live).map_err(|error|{if !self.cancelled&&self.failure.is_none(){self.failure=Some(error.clone());}error})?;
  if scene.flatness!=self.identity.flatness||scene.geometry.len()!=scene.plan.nodes.len()||scene.geometry.len()>1024{let error="Prepared scene cache precision or structure changed".to_string();self.failure=Some(error.clone());return Err(error);}
  for _ in 0..grant{if self.phase=="complete"{break;}if let Err(error)=self.step(scene,live){self.failure=Some(error.clone());return Err(error);}self.work+=1;}Ok(PreparedScenePickProgress{phase:self.phase,work:self.work,done:self.phase=="complete"})
 }
 pub fn result(&self,live:PreparedSceneIdentity)->Result<&[usize],String>{self.authority(live)?;if self.phase!="complete"{return Err("Prepared scene query incomplete".into());}Ok(&self.hits)}
 pub fn cancel(&mut self){self.cancelled=true;}
}
/// 📐️ Selection presentation and pointer handles share cached bounds and unlocked ancestry.
pub fn prepared_selection_bounds(scene:&PreparedScene,ids:&[String])->Result<Option<[f64;4]>,String>{
 if ids.len()>256||ids.iter().map(String::len).sum::<usize>()>16384{return Err("Scene selection exceeds identity capacity".into());}
 let selected=ids.iter().filter_map(|id|scene.plan.nodes.iter().find(|node|node.id==*id).map(|node|node.source_path.as_slice())).collect::<Vec<_>>();let mut bounds=None;
 for(index,node)in scene.plan.nodes.iter().enumerate(){if !visible(node)||crate::schema::scene_preparation::scene_selection_relation(&node.source_path,node.locked_ancestors,&selected).map_err(|error|error.to_string())?.bounds_selection.is_none(){continue;}if let Some(next)=scene.geometry[index].bounds.or(scene.geometry[index].geometry_bounds){bounds=Some(crate::schema::scene_view::union(bounds,next));}}
 Ok(bounds)
}
#[cfg(test)]
#[path="🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
