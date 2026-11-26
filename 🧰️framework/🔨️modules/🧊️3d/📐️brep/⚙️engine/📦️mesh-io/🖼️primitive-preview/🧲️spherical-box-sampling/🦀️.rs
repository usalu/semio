//! 🧩️ Samples both sides of every shared spherical corner trim.

use super::*;

pub fn slots(face: usize, segments: usize) -> usize {
    if face < SphericalCornerBox::SPHERE_FACES { 2 * segments * segments }
    else if (face - SphericalCornerBox::SPHERE_FACES) % 2 == 0 { segments + 2 }
    else { 2 }
}

pub fn triangle(shape: SphericalCornerBox, face: usize, slot: usize, segments: usize) -> Option<[Vertex; 3]> {
    if face < SphericalCornerBox::SPHERE_FACES {
        let side = slot % 2;
        let column = (slot / 2) % segments;
        let row = slot / (2 * segments);
        let (_, upper) = SphericalCornerBox::sphere_chart(face);
        if (!upper && row == 0 && side == 0) || (upper && row + 1 == segments && side == 1) {
            return None;
        }
        let chart = if side == 0 { [(column,row),(column+1,row),(column+1,row+1)] }
                    else { [(column,row),(column+1,row+1),(column,row+1)] };
        return Some(chart.map(|(u,v)| {
            let (point,normal)=shape.sphere_point(face,u as f64/segments as f64,v as f64/segments as f64);
            Vertex{point,normal}
        }));
    }
    let face = face - SphericalCornerBox::SPHERE_FACES;
    let normal = shape.plane(face).z;
    let corner = |member|shape.vertex(shape.plane_corner(face,member));
    let points = if face % 2 == 1 {
        if slot == 0 { [corner(0),corner(1),corner(2)] } else { [corner(0),corner(2),corner(3)] }
    } else if slot == 0 {
        [corner(1),corner(2),corner(3)]
    } else if slot == segments + 1 {
        [corner(1),corner(4),corner(0)]
    } else {
        let (index,forward)=shape.plane_edge(face,3);
        let (curve,range,_)=shape.edge(index);
        let sample = |i| {
            if i==0{return corner(3);}
            if i==segments{return corner(4);}
            let fraction=if forward {i} else {segments-i} as f64/segments as f64;
            curve.eval(range.0+(range.1-range.0)*fraction)
        };
        [corner(1),sample(slot-1),sample(slot)]
    };
    Some(points.map(|point|Vertex{point,normal}))
}
