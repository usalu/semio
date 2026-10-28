//! 🧵️ Borrowed complete vector graph semantics are supplied by the genuine typed child owner.
use super::*;
use semio_framework_value::paged::Utf8Text;
#[derive(Clone,Copy,Debug)]pub enum VectorFillHead{Solid{color:[f64;4]},Linear{x1:f64,y1:f64,x2:f64,y2:f64},Radial{cx:f64,cy:f64,r:f64}}
#[derive(Clone,Copy,Debug)]pub struct VectorGradientStop{pub offset:f64,pub color:[f64;4]}
#[derive(Clone,Copy,Debug)]pub struct VectorStrokeHead{pub color:[f64;4],pub width:f64,pub cap:&'static str,pub join:&'static str}
pub trait VectorPaintSourceV1:Sync{fn fill_rule(&self)->&'static str;fn fill(&self)->Option<VectorFillHead>;fn stop_count(&self)->usize;fn stop(&self,index:usize)->Option<VectorGradientStop>;fn stroke(&self)->Option<VectorStrokeHead>;fn dash_count(&self)->usize;fn dash(&self,index:usize)->Option<f64>;}
pub trait VectorPathSourceV1:Sync{fn segment_count(&self)->usize;fn segment(&self,index:usize)->Option<super::super::VectorSegment>;}
pub trait VectorPolygonSourceV1:Sync{fn point_count(&self)->usize;fn point(&self,index:usize)->Option<[f64;2]>;}
pub enum VectorShapeRef<'a>{Rect{x:f64,y:f64,width:f64,height:f64},Ellipse{cx:f64,cy:f64,rx:f64,ry:f64},Circle{cx:f64,cy:f64,r:f64},Line{x1:f64,y1:f64,x2:f64,y2:f64},Polygon(&'a dyn VectorPolygonSourceV1)}
pub trait VectorShapeSourceV1:Sync{fn primitive(&self)->Option<VectorShapeRef<'_>>;}
pub trait VectorRgbaSourceV1:Sync{fn width(&self)->u32;fn height(&self)->u32;fn sample_count(&self)->usize;fn sample(&self,index:usize)->Option<[u8;4]>;}
pub struct VectorAssetRef<'a>{pub identity:usize,pub key:&'a (dyn Utf8Text+Sync),pub samples:&'a dyn VectorRgbaSourceV1}
pub enum VectorContentRef<'a>{Path(&'a dyn VectorPathSourceV1),Shape(&'a dyn VectorShapeSourceV1),Text{content:&'a (dyn Utf8Text+Sync),x:f64,y:f64,size:f64,font_family:&'static str},Image{key:&'a (dyn Utf8Text+Sync),width:f64,height:f64},Group{isolation:bool},Boolean{operation:&'a (dyn Utf8Text+Sync)},Trace{key:&'a (dyn Utf8Text+Sync),threshold:f64,simplify_epsilon:f64}}
pub struct VectorNodeRef<'a>{pub identity:usize,pub id:&'a (dyn Utf8Text+Sync),pub name:&'a (dyn Utf8Text+Sync),pub transform:[f64;6],pub opacity:f64,pub blend_mode:&'a (dyn Utf8Text+Sync),pub visible:bool,pub locked:bool,pub paint:&'a dyn VectorPaintSourceV1,pub content:VectorContentRef<'a>}
pub trait VectorNodeSourceV1:Sync{fn descriptor(&self)->VectorNodeRef<'_>;fn child_count(&self)->usize;fn child(&self,index:usize)->Option<&dyn VectorNodeSourceV1>;fn reference_count(&self)->usize;fn reference(&self,index:usize)->Option<&(dyn Utf8Text+Sync)>;}
pub trait VectorGraphSourceV1:Sync{fn schema(&self)->&(dyn Utf8Text+Sync);fn id(&self)->&(dyn Utf8Text+Sync);fn canvas(&self)->Option<VectorCanvas>;fn root_count(&self)->usize;fn root(&self,index:usize)->Option<&dyn VectorNodeSourceV1>;fn asset_count(&self)->usize;fn asset(&self,index:usize)->Option<VectorAssetRef<'_>>;}
pub struct VectorGraphAdvance<'a>{pub progress:RetainedCloneProgress,pub done:bool,pub graph:Option<&'a dyn VectorGraphSourceV1>}
/// 📖️ The caller retains the issued original loan while borrowing each fixed graph descriptor; this facet never allocates a scene.
pub trait EmbeddedChildVectorSourceV1:Send+Sync{fn read_demand(&self)->RetirementDemand;fn read<'a>(&self,source:ChildAdmissionSource<'a>,grant:RetainedCloneGrant)->Result<VectorGraphAdvance<'a>,ValueError>;}

#[path="🧳️prepare/🦀️.rs"]pub mod preparation;
pub use preparation::{OriginalChildVectorPreparation,OriginalChildVectorAdvance};
