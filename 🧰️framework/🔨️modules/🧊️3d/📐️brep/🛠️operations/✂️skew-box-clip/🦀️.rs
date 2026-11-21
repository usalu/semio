//! ✂️ Exact bounded intersections of nonparallel rectangular stock.

use crate::point_hull::HullBoundary;
use semio_framework_3d_brep_intersect::convex_box::{BoxPlacement,box_cavity_faces,box_recess_faces,intersect_box_faces,support_interval,union_box_faces};
use semio_framework_3d_brep_representation::contract::{ClippedBox,ClippedBoxBoundary};
use semio_framework_3d_brep_tolerance::Tol;
pub use semio_framework_3d_brep_intersect::convex_box::{BoxSupportPieces,box_recess_pieces,boxes_separated,contains_box};

pub enum SkewBoxBoundary{
    Clipped(ClippedBoxBoundary,ClippedBox),
    Planar(HullBoundary),
}

impl SkewBoxBoundary{
    pub fn map<T>(self,clipped:impl FnOnce(ClippedBoxBoundary,ClippedBox)->T,planar:impl FnOnce(HullBoundary)->T)->T{
        match self{
            Self::Clipped(boundary,proof)=>clipped(boundary,proof),
            Self::Planar(boundary)=>planar(boundary),
        }
    }
}

/// 📦️ Separates empty support contact from geometry outside the finite boundary certificate.
pub enum SkewBoxClipRefusal{ Empty, Unsupported }

fn single_clip(stock:BoxPlacement,tool:BoxPlacement,inside:bool)->Option<(ClippedBoxBoundary,ClippedBox)>{
    let (origin,axes,size)=tool;
    let mut plane=None;
    for (axis,length) in axes.into_iter().zip(size){
        let [lo,hi]=support_interval(stock,origin,axis);
        for (cuts,point,normal) in [(lo<0.,origin,axis),(hi>length,origin+axis*length,-axis)]{
            if cuts&&plane.replace((point,normal)).is_some(){return None;}
        }
    }
    let (point,normal)=plane?;
    let [positive,_]=ClippedBox::split(stock.0,stock.1,stock.2,point,if inside{normal}else{-normal})?;
    Some(positive)
}

/// 🧭️ Resolves separating axes, then publishes one exact clipped or convex boundary.
pub fn skew_box_intersection(first:BoxPlacement,second:BoxPlacement)->Result<SkewBoxBoundary,SkewBoxClipRefusal>{
    let tolerance=Tol::DEFAULT.value();
    if boxes_separated(first,second,tolerance/16.).ok_or(SkewBoxClipRefusal::Unsupported)?{
        return Err(SkewBoxClipRefusal::Empty);
    }
    if let Some((boundary,proof))=single_clip(first,second,true).or_else(||single_clip(second,first,true)){
        return Ok(SkewBoxBoundary::Clipped(boundary,proof));
    }
    intersect_box_faces(first,second,tolerance).and_then(HullBoundary::from_box_faces)
        .map(SkewBoxBoundary::Planar).ok_or(SkewBoxClipRefusal::Unsupported)
}

/// 🪚️ Publishes the bounded native boundary for a complementary cut, cavity or sewn recess.
pub fn skew_box_difference(stock:BoxPlacement,tool:BoxPlacement)->Result<Option<SkewBoxBoundary>,SkewBoxClipRefusal>{
    let tolerance=Tol::DEFAULT.value();
    if boxes_separated(stock,tool,tolerance/16.).ok_or(SkewBoxClipRefusal::Unsupported)?{return Ok(None);}
    if let Some((boundary,proof))=single_clip(stock,tool,false){return Ok(Some(SkewBoxBoundary::Clipped(boundary,proof)));}
    box_cavity_faces(stock,tool,tolerance).or_else(||box_recess_faces(stock,tool,tolerance)).and_then(HullBoundary::from_box_faces)
        .map(|boundary|Some(SkewBoxBoundary::Planar(boundary))).ok_or(SkewBoxClipRefusal::Unsupported)
}

/// 🤝️ Publishes the sewn boundary for placed boxes with positive overlap.
pub fn placed_box_union(first:BoxPlacement,second:BoxPlacement)->Option<HullBoundary>{
    let tolerance=Tol::DEFAULT.value();
    if boxes_separated(first,second,tolerance/16.)?{return None;}
    union_box_faces(first,second,tolerance).and_then(HullBoundary::from_box_faces)
}
