//! 🪟️ Fixed straight prism with two rectangular cap loops and no internal faces.

use semio_framework_3d_brep_curve::Curve3;
use semio_framework_3d_brep_vector::{Pnt3,Vec3,matrix::Frame3};

const CORNERS: usize = 8;

pub const PERFORATED_RING_SIZES: [usize;12] = [4;12];

/// 🔗️ Outward cap and wall placements for one certified rectangular through-hole.
pub struct PerforatedPrism {
    points: [Pnt3;CORNERS],
    span: Vec3,
    normal: Vec3,
}

impl PerforatedPrism {
    pub fn new(rings: [[[f64;2];4];2], origin: Pnt3, [x,y,z]: [Vec3;3], height: f64, tol: f64) -> Option<Self> {
        let [outer,hole] = rings;
        if !(height > tol && height.is_finite()) { return None; }
        let mut inside = hole;
        inside.reverse();
        let rebuilt = super::PolygonRegion::rectangles(outer,inside,super::RectangleOperation::Difference,tol)?;
        if rebuilt.rings().len()!=2 || !rebuilt.rings().any(|ring| super::PolygonRegion::ring_area(ring)<0.) { return None; }
        let span = z*height;
        let normal = z.normalized()?;
        let orientation = x.cross(y).dot(normal);
        if !((orientation.abs()-1.).abs()*height <= tol/4.) { return None; }
        let mut points = [origin;CORNERS];
        for (slot,polygon) in points.chunks_exact_mut(4).zip(rings) {
            let mut polygon = polygon;
            if orientation<0. { polygon.reverse(); }
            for (point,[u,v]) in slot.iter_mut().zip(polygon) {
                let local = x*u+y*v;
                *point = origin+local;
                let top = *point+span;
                if !point.to_array().into_iter().chain(top.to_array()).all(f64::is_finite)
                    || ((*point-origin)-local).norm()>tol/4.
                    || ((top-*point)-span).norm()>tol/4. { return None; }
            }
        }
        for i in 0..CORNERS {
            let tangent=points[Self::next(i)]-points[i];
            if !(tangent.norm()>tol && tangent.cross(span).norm()>f64::EPSILON) { return None; }
        }
        Some(Self { points,span,normal })
    }

    pub fn vertex(&self, id: usize) -> Pnt3 {
        let point = self.points[id % CORNERS];
        if id<CORNERS { point } else { point+self.span }
    }

    fn next(corner: usize) -> usize { 4*(corner/4)+(corner+1)%4 }

    pub fn edge(&self, id: usize) -> ([usize;2],Curve3) {
        let i = id%CORNERS;
        let next = Self::next(i);
        let ids = match id/CORNERS {
            0 => [i,next],
            1 => [CORNERS+i,CORNERS+next],
            _ => [i,CORNERS+i],
        };
        let [origin,end]=ids.map(|id|self.vertex(id));
        (ids,Curve3::Line{origin,dir:end-origin})
    }

    pub fn frame(&self, face: usize) -> Frame3 {
        if face<2 {
            return Frame3::from_normal(self.vertex(face*CORNERS),if face==0{-self.normal}else{self.normal})
                .expect("certified cap normal");
        }
        let corner = face-2;
        let origin = self.vertex(corner);
        let tangent = self.vertex(Self::next(corner))-origin;
        Frame3::from_x_z(origin,tangent,tangent.cross(self.span)).expect("certified wall normal")
    }

    pub fn member(ring: usize, index: usize) -> (usize,bool) {
        if ring<2 || ring>=10 {
            let offset = if ring<2 {0}else{4};
            return if ring%2==0 {(offset+3-index,false)} else {(CORNERS+offset+index,true)};
        }
        let corner=ring-2;
        [(corner,true),(2*CORNERS+Self::next(corner),true),(CORNERS+corner,false),(2*CORNERS+corner,false)][index]
    }
}
