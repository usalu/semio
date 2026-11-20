//! 🪚️ Exact one-edge chamfer boundary for an axis-aligned native box.

use semio_framework_3d_brep_construction::{primitives::SolidPrimitive, prism_boundary::{MAX_PRISM_CORNERS, PrismBoundary}};
use semio_framework_3d_brep_tolerance::Tol;
use semio_framework_3d_brep_vector::{Pnt3, Vec3};

/// 🔶️ Clips the selected profile corner and extrudes it along the full source edge.
pub fn box_edge_chamfer(primitive: SolidPrimitive, edge: usize, distance: f64) -> Option<PrismBoundary> {
    let SolidPrimitive::Box { origin, width, depth, height } = primitive else { return None; };
    if edge >= primitive.edge_count() || !distance.is_finite() || distance <= Tol::DEFAULT.value() { return None; }
    let [a,b] = primitive.edge(edge).vertices.map(|i|primitive.vertex(i).to_array());
    let axis = (0..3).find(|&i| a[i] != b[i])?;
    let (u,v) = ((axis+1)%3,(axis+2)%3);
    let lo = origin.to_array();
    let size = [width,depth,height];
    let hi: [f64;3] = std::array::from_fn(|i| lo[i]+size[i]);
    let trims = [[lo[u]+distance,hi[u]-distance],[lo[v]+distance,hi[v]-distance]];
    if [u,v].into_iter().zip(trims).any(|(i,cut)| cut.into_iter().any(|x| !(lo[i]<x && x<hi[i]))) { return None; }
    let ring = [[lo[u],lo[v]],[hi[u],lo[v]],[hi[u],hi[v]],[lo[u],hi[v]]];
    let target = [a[u],a[v]];
    let selected = ring.iter().position(|&corner| corner==target)?;
    let mut points = [origin;MAX_PRISM_CORNERS];
    let place = |coords:[f64;2]| {
        let mut p=lo;
        p[u]=coords[0];p[v]=coords[1];
        Pnt3::from_array(p)
    };
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
    let mut normal=[0.;3];normal[axis]=1.;
    let normal=Vec3::from_array(normal);
    PrismBoundary::from_planar_ring(points,count,normal*size[axis],normal)
}
