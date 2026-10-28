//! 📋️ Canonical Drawing graph admission borrows every typed authored value without planning or rendering it.
use super::*;
use super::index::{Slots,Target,start};
use super::source::{PagedHash,PagedCompare};
use semio_framework_value::paged::Utf8Text;
use std::{future::Future,pin::Pin,task::{Context,Poll},sync::atomic::Ordering};
struct Tick(bool);
impl Future for Tick{type Output=();fn poll(mut self:Pin<&mut Self>,_:&mut Context<'_>)->Poll<()>{if self.0{Poll::Ready(())}else{self.0=true;Poll::Pending}}}
async fn tick(){Tick(false).await}
fn require(valid:bool,message:&'static str)->Result<(),&'static str>{if valid{Ok(())}else{Err(message)}}
fn positive(v:f64)->bool{v.is_finite()&&v>0.0}
fn nonnegative(v:f64)->bool{v.is_finite()&&v>=0.0}
fn unit(v:f64)->bool{v.is_finite()&&(0.0..=1.0).contains(&v)}
fn color(v:&[f64;4])->bool{v.iter().copied().all(unit)}
async fn same<A:Utf8Text+Sync+?Sized,B:Utf8Text+Sync+?Sized>(a:&A,b:&B)->bool{let mut cursor=PagedCompare::default();loop{tick().await;if let Some(equal)=cursor.step(a,b){return equal}}}
async fn member(text:&Text,values:&[&str])->bool{for value in values{if same(text,*value).await{return true}}false}
async fn lookup<'a>(slots:Slots<'a>,kind:u8,key:&Text)->Option<(usize,&'a index::Entry)>{if slots.len()==0{tick().await;return None}let mut hash=PagedHash::default();loop{tick().await;if hash.step(key){break}}let mut at=start(kind,hash.hash,slots.len());for _ in 0..slots.len(){tick().await;let Some((held,entry))=slots.get(at)else{return None};if *held==(kind,hash.hash)&&same(key,entry.key).await{return Some((at,entry))}at=(at+1)&(slots.len()-1);}None}
async fn attributes(attributes:&crate::DrawingAttributes)->Result<(),&'static str>{
 if let Some(fill)=&attributes.fill{tick().await;let stops=match fill{crate::FillStyle::Solid{color:c}=>{require(color(c),"Drawing paint channels must be finite unit values.")?;None},crate::FillStyle::LinearGradient{x1,y1,x2,y2,stops}=>{require([x1,y1,x2,y2].iter().all(|v|v.is_finite())&&!stops.is_empty(),"Drawing gradients need finite geometry and non-empty stops.")?;Some(stops)},crate::FillStyle::RadialGradient{cx,cy,r,stops}=>{require(cx.is_finite()&&cy.is_finite()&&positive(*r)&&!stops.is_empty(),"Drawing radial gradients need finite geometry, positive radius and non-empty stops.")?;Some(stops)}};if let Some(stops)=stops{for stop in stops.iter(){tick().await;require(unit(stop.offset)&&color(&stop.color),"Drawing gradient stops must have unit offsets and paint channels.")?}}}
 if let Some(stroke)=&attributes.stroke{tick().await;require(nonnegative(stroke.width)&&color(&stroke.color),"Drawing strokes need non-negative finite width and unit paint channels.")?;if let Some(dash)=&stroke.dash{for value in dash.iter(){tick().await;require(nonnegative(*value),"Drawing dash lengths must be finite and non-negative.")?}}}Ok(())
}
async fn shape(shape:&crate::DrawingShapeBody)->Result<(),&'static str>{tick().await;let count=usize::from(shape.rect.is_some())+usize::from(shape.ellipse.is_some())+usize::from(shape.circle.is_some())+usize::from(shape.line.is_some())+usize::from(shape.polygon.is_some());require(count==1,"Drawing shape kind must match exactly one geometry.")?;
 if same(&shape.shape_kind,"rect").await{let r=shape.rect.as_ref().ok_or("Drawing shape kind must match exactly one geometry.")?;require(r.x.is_finite()&&r.y.is_finite()&&nonnegative(r.width)&&nonnegative(r.height),"Drawing rectangles need finite coordinates and non-negative extents.")?}
 else if same(&shape.shape_kind,"ellipse").await{let r=shape.ellipse.as_ref().ok_or("Drawing shape kind must match exactly one geometry.")?;require(r.cx.is_finite()&&r.cy.is_finite()&&nonnegative(r.rx)&&nonnegative(r.ry),"Drawing ellipses need finite coordinates and non-negative radii.")?}
 else if same(&shape.shape_kind,"circle").await{let r=shape.circle.as_ref().ok_or("Drawing shape kind must match exactly one geometry.")?;require(r.cx.is_finite()&&r.cy.is_finite()&&nonnegative(r.r),"Drawing circles need finite coordinates and non-negative radius.")?}
 else if same(&shape.shape_kind,"line").await{let r=shape.line.as_ref().ok_or("Drawing shape kind must match exactly one geometry.")?;require([r.x1,r.y1,r.x2,r.y2].iter().all(|v|v.is_finite()),"Drawing line coordinates must be finite.")?}
 else if same(&shape.shape_kind,"polygon").await{let p=shape.polygon.as_ref().ok_or("Drawing shape kind must match exactly one geometry.")?;for point in p.points.iter(){tick().await;require(point.iter().all(|v|v.is_finite()),"Drawing polygon coordinates must be finite.")?}}
 else{return Err("Drawing shape kind must name an authored primitive.")}Ok(())
}
pub(super) async fn check(source:&DrawingSnapshot,slots:Slots<'_>)->Result<(),&'static str>{
 tick().await;require(same(&source.schema,crate::DRAWING_DOCUMENT_SCHEMA).await&&!source.id.is_empty(),"Drawing documents need their exact schema and non-empty identity.")?;
 for at in 0..slots.len(){tick().await;let Some((_,entry))=slots.get(at)else{continue};match entry.target{
 Target::Asset(asset)=>{let count=u64::from(asset.width)*u64::from(asset.height);require(count>0&&usize::try_from(count).is_ok_and(|count|count==asset.samples.len()),"Drawing asset dimensions and intrinsic RGBA sample count must agree.")?},
 Target::Layer(node)=>{let base=crate::layer_base(node);let t=&base.transform;require([t.x,t.y,t.scale_x,t.scale_y,t.shear,t.rotation].iter().all(|v|v.is_finite())&&unit(base.opacity)&&member(&base.blend_mode,crate::DRAWING_BLEND_MODES).await,"Drawing layers need finite transforms, unit opacity and an authored blend mode.")?;attributes(&base.attributes).await?;match node{
 crate::DrawingLayerNode::Path(path)=>for segment in path.segments.iter(){tick().await;require(crate::valid_path_segment(segment),"Drawing path geometry must be finite with non-negative arc radii.")?},
 crate::DrawingLayerNode::Shape(value)=>shape(value).await?,
 crate::DrawingLayerNode::Text(value)=>require(value.x.is_finite()&&value.y.is_finite()&&positive(value.size),"Drawing text needs finite coordinates and positive size.")?,
 crate::DrawingLayerNode::Image(value)=>{require(positive(value.width)&&positive(value.height),"Drawing images need positive finite extents.")?;require(lookup(slots,1,&value.image_key).await.is_some(),"Drawing image layers must reference an existing asset.")?},
 crate::DrawingLayerNode::Boolean(value)=>require(member(&value.operation,crate::DRAWING_BOOLEAN_OPERATIONS).await,"Drawing booleans must use an authored operation.")?,
 crate::DrawingLayerNode::Trace(value)=>{require(unit(value.params.threshold)&&nonnegative(value.params.simplify_epsilon),"Drawing trace settings need unit threshold and non-negative finite epsilon.")?;require(lookup(slots,1,&value.source_key).await.is_some(),"Drawing trace layers must reference an existing asset.")?},
 crate::DrawingLayerNode::Group(_)=>{}}}}
 }
 for root in 0..slots.len(){tick().await;let Some((_,entry))=slots.get(root)else{continue};if !matches!(entry.target,Target::Layer(_))||entry.mark.load(Ordering::Relaxed)==2{continue}entry.mark.store(1,Ordering::Relaxed);let mut at=root;while at!=usize::MAX{tick().await;let(_,current)=slots.get(at).expect("original active Draw layer slot");let Target::Layer(node)=current.target else{unreachable!()};let edge=current.edge.fetch_add(1,Ordering::Relaxed);let key=match node{crate::DrawingLayerNode::Group(group)=>group.children.get(edge).map(|node|&crate::layer_base(node).id),crate::DrawingLayerNode::Boolean(value)=>value.children.get(edge),_=>None};if let Some(key)=key{let(next,target)=lookup(slots,0,key).await.ok_or("Drawing operands must reference existing layers.")?;match target.mark.load(Ordering::Relaxed){1=>return Err("Drawing group and boolean dependencies must not contain cycles."),0=>{target.parent.store(at,Ordering::Relaxed);target.mark.store(1,Ordering::Relaxed);at=next},_=>{}}}else{current.mark.store(2,Ordering::Relaxed);at=current.parent.load(Ordering::Relaxed);}}}
 Ok(())
}
