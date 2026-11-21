//! 🪚️ Exact one-edge chamfer boundary for a certified rectangular stock.

use semio_framework_3d_brep_construction::{primitives::{BoxGeometry,SolidPrimitive}, prism_boundary::{MAX_PRISM_CORNERS, PrismBoundary}};
use semio_framework_3d_brep_tolerance::Tol;
use semio_framework_3d_brep_vector::{Pnt3, Vec3};

/// 🔶️ Clips the selected profile corner and extrudes it along the full source edge.
pub fn box_edge_chamfer(primitive: SolidPrimitive, edge: usize, distance: f64) -> Option<PrismBoundary> {
    let SolidPrimitive::Box { origin, width, depth, height } = primitive else { return None; };
    chamfer_placed(origin,[Vec3::X,Vec3::Y,Vec3::Z],[width,depth,height],edge,distance)
}

/// 📐️ Builds the exact clipped profile on two authored stock axes.
pub(super) fn chamfer_placed(origin:Pnt3,axes:[Vec3;3],size:[f64;3],edge:usize,distance:f64)->Option<PrismBoundary>{
    let tol=Tol::DEFAULT.value();
    if !distance.is_finite()||distance<=tol{return None;}
    let corners=BoxGeometry::corners(size[0],size[1],size[2]);
    let [a,b]=BoxGeometry::EDGES.get(edge)?.map(|i|corners[i].to_array());
    let axis=(0..3).find(|&i|a[i]!=b[i])?;
    let (u,v)=((axis+1)%3,(axis+2)%3);
    let [w,h]=[size[u],size[v]];
    let trims=[[distance,w-distance],[distance,h-distance]];
    if w-distance<=tol||h-distance<=tol{return None;}
    let ring=[[0.,0.],[w,0.],[w,h],[0.,h]];
    let selected=ring.iter().position(|&corner|corner==[a[u],a[v]])?;
    let mut points=[origin;MAX_PRISM_CORNERS];
    let place=|[x,y]:[f64;2]|origin+(axes[u]*x+axes[v]*y);
    let mut count=0;
    for (i,corner) in ring.into_iter().enumerate() {
        if i==selected {
            for neighbor in [ring[(i+3)%4],ring[(i+1)%4]] {
                let coords=std::array::from_fn(|j| if corner[j]==neighbor[j] {corner[j]}
                    else {trims[j][usize::from(corner[j]>neighbor[j])]});
                points[count]=place(coords);
                count+=1;
            }
        } else {
            points[count]=place(corner);
            count+=1;
        }
    }
    let span=axes[axis]*size[axis];
    for (i,&point) in points[..count].iter().enumerate(){
        let next=points[(i+1)%count];
        let top=point+span;
        let length=(next-point).norm();
        if !length.is_finite()||!point.to_array().into_iter().chain(top.to_array()).all(f64::is_finite)
            || length<=tol||((top-point)-span).norm()>tol{return None;}
    }
    PrismBoundary::from_planar_ring(points,count,span,axes[u].cross(axes[v]))
}
