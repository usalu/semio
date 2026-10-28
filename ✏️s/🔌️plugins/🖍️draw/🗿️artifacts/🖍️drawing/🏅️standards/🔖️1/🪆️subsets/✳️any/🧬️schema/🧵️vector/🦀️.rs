//! 🧵️ Genuine Draw snapshots lend complete typed vector semantics without raw conversion or prepared-scene copies.
use crate::*;
use crate::snapshot::DrawingSnapshot;
use semio_framework_value::{ValueError,ValueRefusalKind,RetirementDemand,paged::Utf8Text,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,retained_clone_grant_admits}};
use semio_framework_io_ports::{ChildAdmissionSource,VectorCanvas,VectorPoint2,VectorSegment,vector::*};
fn point(value:[f64;2])->VectorPoint2{VectorPoint2{x:value[0],y:value[1]}}
impl VectorPaintSourceV1 for DrawingAttributes{
 fn fill_rule(&self)->&'static str{self.fill_rule.as_str()}
 fn fill(&self)->Option<VectorFillHead>{self.fill.as_ref().map(|fill|match fill{FillStyle::Solid{color}=>VectorFillHead::Solid{color:*color},FillStyle::LinearGradient{x1,y1,x2,y2,..}=>VectorFillHead::Linear{x1:*x1,y1:*y1,x2:*x2,y2:*y2},FillStyle::RadialGradient{cx,cy,r,..}=>VectorFillHead::Radial{cx:*cx,cy:*cy,r:*r}})}
 fn stop_count(&self)->usize{match &self.fill{Some(FillStyle::LinearGradient{stops,..}|FillStyle::RadialGradient{stops,..})=>stops.len(),_=>0}}
 fn stop(&self,index:usize)->Option<VectorGradientStop>{match &self.fill{Some(FillStyle::LinearGradient{stops,..}|FillStyle::RadialGradient{stops,..})=>stops.get(index).map(|stop|VectorGradientStop{offset:stop.offset,color:stop.color}),_=>None}}
 fn stroke(&self)->Option<VectorStrokeHead>{self.stroke.as_ref().map(|stroke|VectorStrokeHead{color:stroke.color,width:stroke.width,cap:stroke.cap.as_str(),join:stroke.join.as_str()})}
 fn dash_count(&self)->usize{self.stroke.as_ref().and_then(|stroke|stroke.dash.as_ref()).map_or(0,|dash|dash.len())}
 fn dash(&self,index:usize)->Option<f64>{self.stroke.as_ref().and_then(|stroke|stroke.dash.as_ref()).and_then(|dash|dash.get(index)).copied()}
}
impl VectorPathSourceV1 for DrawingPathBody{
 fn segment_count(&self)->usize{self.segments.len()}
 fn segment(&self,index:usize)->Option<VectorSegment>{self.segments.get(index).map(|segment|match segment{PathSegment::Move{to}=>VectorSegment::MoveTo{to:point(*to)},PathSegment::Line{to}=>VectorSegment::LineTo{to:point(*to)},PathSegment::Quad{ctrl,to}=>VectorSegment::QuadTo{c:point(*ctrl),to:point(*to)},PathSegment::Cubic{ctrl1,ctrl2,to}=>VectorSegment::CubicTo{c1:point(*ctrl1),c2:point(*ctrl2),to:point(*to)},PathSegment::Arc{rx,ry,rotation,large_arc,sweep,to}=>VectorSegment::ArcTo{rx:*rx,ry:*ry,x_rotation:*rotation,large_arc:*large_arc,sweep:*sweep,to:point(*to)},PathSegment::Close=>VectorSegment::Close})}
}
impl VectorPolygonSourceV1 for DrawingPolygon{fn point_count(&self)->usize{self.points.len()}fn point(&self,index:usize)->Option<[f64;2]>{self.points.get(index).copied()}}
impl VectorShapeSourceV1 for DrawingShapeBody{
 fn primitive(&self)->Option<VectorShapeRef<'_>>{if self.shape_kind.eq_str("rect"){self.rect.as_ref().map(|v|VectorShapeRef::Rect{x:v.x,y:v.y,width:v.width,height:v.height})}else if self.shape_kind.eq_str("ellipse"){self.ellipse.as_ref().map(|v|VectorShapeRef::Ellipse{cx:v.cx,cy:v.cy,rx:v.rx,ry:v.ry})}else if self.shape_kind.eq_str("circle"){self.circle.as_ref().map(|v|VectorShapeRef::Circle{cx:v.cx,cy:v.cy,r:v.r})}else if self.shape_kind.eq_str("line"){self.line.as_ref().map(|v|VectorShapeRef::Line{x1:v.x1,y1:v.y1,x2:v.x2,y2:v.y2})}else if self.shape_kind.eq_str("polygon"){self.polygon.as_ref().map(|v|VectorShapeRef::Polygon(v))}else{None}}
}
impl VectorRgbaSourceV1 for DrawingImageAsset{fn width(&self)->u32{self.width}fn height(&self)->u32{self.height}fn sample_count(&self)->usize{self.samples.len()}fn sample(&self,index:usize)->Option<[u8;4]>{self.samples.get(index).copied()}}
impl VectorNodeSourceV1 for DrawingLayerNode{
 fn descriptor(&self)->VectorNodeRef<'_>{let base=layer_base(self);let content=match self{Self::Path(v)=>VectorContentRef::Path(v),Self::Shape(v)=>VectorContentRef::Shape(v),Self::Text(v)=>VectorContentRef::Text{content:&v.content,x:v.x,y:v.y,size:v.size,font_family:v.font_family.as_str()},Self::Image(v)=>VectorContentRef::Image{key:&v.image_key,width:v.width,height:v.height},Self::Group(v)=>VectorContentRef::Group{isolation:v.isolation},Self::Boolean(v)=>VectorContentRef::Boolean{operation:&v.operation},Self::Trace(v)=>VectorContentRef::Trace{key:&v.source_key,threshold:v.params.threshold,simplify_epsilon:v.params.simplify_epsilon}};VectorNodeRef{identity:self as*const Self as usize,id:&base.id,name:&base.name,transform:drawing_transform_to_matrix(&base.transform),opacity:base.opacity,blend_mode:&base.blend_mode,visible:base.visible,locked:base.locked,paint:&base.attributes,content}}
 fn child_count(&self)->usize{match self{Self::Group(v)=>v.children.len(),_=>0}}
 fn child(&self,index:usize)->Option<&dyn VectorNodeSourceV1>{match self{Self::Group(v)=>v.children.get(index).map(|node|node as &dyn VectorNodeSourceV1),_=>None}}
 fn reference_count(&self)->usize{match self{Self::Boolean(v)=>v.children.len(),_=>0}}
 fn reference(&self,index:usize)->Option<&(dyn Utf8Text+Sync)>{match self{Self::Boolean(v)=>v.children.get(index).map(|key|key as &(dyn Utf8Text+Sync)),_=>None}}
}
impl VectorGraphSourceV1 for DrawingSnapshot{
 fn schema(&self)->&(dyn Utf8Text+Sync){&self.schema}fn id(&self)->&(dyn Utf8Text+Sync){&self.id}
 fn canvas(&self)->Option<VectorCanvas>{self.artboard.as_ref().map(|v|VectorCanvas{width:v.width,height:v.height,background:None})}
 fn root_count(&self)->usize{self.layers.len()}fn root(&self,index:usize)->Option<&dyn VectorNodeSourceV1>{self.layers.get(index).map(|node|node as &dyn VectorNodeSourceV1)}
 fn asset_count(&self)->usize{self.assets.len()}fn asset(&self,index:usize)->Option<VectorAssetRef<'_>>{self.assets.entry_at(index).map(|(key,samples)|VectorAssetRef{identity:samples as*const DrawingImageAsset as usize,key,samples})}
}
pub struct DrawingVectorSource;
pub static DRAWING_VECTOR_SOURCE:DrawingVectorSource=DrawingVectorSource;
impl EmbeddedChildVectorSourceV1 for DrawingVectorSource{
 fn read_demand(&self)->RetirementDemand{RetirementDemand{copy_bytes:std::mem::size_of::<VectorGraphAdvance<'static>>()*2+std::mem::size_of::<ChildAdmissionSource<'static>>(),depth:1,..Default::default()}}
 fn read<'a>(&self,source:ChildAdmissionSource<'a>,g:RetainedCloneGrant)->Result<VectorGraphAdvance<'a>,ValueError>{let d=self.read_demand();if !retained_clone_grant_admits(g,d){return Ok(VectorGraphAdvance{progress:Default::default(),done:false,graph:None})}let progress=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};let graph=source.get().downcast_ref::<DrawingSnapshot>().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"Drawing vector source requires its genuine native snapshot").with_retained_progress(progress))?;Ok(VectorGraphAdvance{progress,done:true,graph:Some(graph)})}
}
#[cfg(test)]#[path="🧪️tests.rs"]mod tests;

#[cfg(test)]#[path="🧪️preparation.rs"]mod preparation_tests;
