//! 🗜️ Pure bounded topology recipe for a straight prism.

use crate::primitives::{PrimitiveEdge, PrimitiveTrim, planar_pcurve};
use semio_framework_3d_brep_curve::Curve3;
use semio_framework_3d_brep_surface::Surface;
use semio_framework_3d_brep_vector::matrix::Frame3;
use semio_framework_3d_brep_vector::{Pnt3, Vec3 as NativeVec3};

pub const MAX_PRISM_CORNERS: usize = 32;
const RANGE: (f64, f64) = (0., 1.);

/// 🧱️ An exact planar prism through the original solid topology recipe.
pub struct PrismBoundary {
    corners: [Pnt3; MAX_PRISM_CORNERS],
    sizes: [usize; MAX_PRISM_CORNERS + 2],
    count: usize,
    vector: NativeVec3,
    normal: NativeVec3,
    reverse: bool,
}

impl PrismBoundary {
    /// 🎟️ Builds a fixed topology recipe from the validated planar ring and transverse vector.
    pub fn from_planar_ring(corners: [Pnt3; MAX_PRISM_CORNERS], count: usize, vector: NativeVec3, area_normal: NativeVec3) -> Option<Self> {
        if !(3..=MAX_PRISM_CORNERS).contains(&count) { return None; }
        let mut normal = area_normal.normalized()?;
        let reverse = normal.dot(vector) < 0.0;
        if reverse { normal = -normal; }
        let mut sizes = [4; MAX_PRISM_CORNERS + 2];
        sizes[0] = count;
        sizes[1] = count;
        Some(Self { corners, sizes, count, vector, normal, reverse })
    }

    pub fn vertex_count(&self) -> usize { 2 * self.count }
    pub fn edge_count(&self) -> usize { 3 * self.count }
    pub fn ring_sizes(&self) -> &[usize] { &self.sizes[..self.count + 2] }

    pub fn vertex(&self, index: usize) -> Pnt3 {
        let corner = index % self.count;
        let corner = if self.reverse { (self.count - corner) % self.count } else { corner };
        let point = self.corners[corner];
        if index < self.count { point } else { point + self.vector }
    }

    pub fn edge(&self, index: usize) -> PrimitiveEdge {
        let n = self.count;
        let corner = index % n;
        let next = (corner + 1) % n;
        let [a, b] = if index < n { [corner, next] } else if index < 2*n { [n+corner, n+next] } else { [corner, n+corner] };
        let origin = self.vertex(a);
        PrimitiveEdge { vertices: [a, b], range: RANGE, curve: Curve3::Line { origin, dir: self.vertex(b) - origin } }
    }

    pub fn surface(&self, index: usize) -> Surface {
        let (origin, tangent, normal) = match index {
            0 => (self.vertex(0), self.vertex(1) - self.vertex(0), -self.normal),
            1 => (self.vertex(self.count), self.vertex(self.count + 1) - self.vertex(self.count), self.normal),
            _ => {
                let start = self.vertex(index - 2);
                let tangent = self.vertex((index - 1) % self.count) - start;
                (start, tangent, tangent.cross(self.vector))
            }
        };
        Surface::Plane { frame: Frame3::from_x_z(origin, tangent, normal).expect("validated prism has nondegenerate planar faces") }
    }

    pub fn trim(&self, face: usize, member: usize) -> PrimitiveTrim {
        let n = self.count;
        let (edge, forward) = match (face, member) {
            (0, _) => (n - member - 1, false),
            (1, _) => (n + member, true),
            (_, 0) => (face - 2, true),
            (_, 1) => (2*n + (face - 1) % n, true),
            (_, 2) => (n + face - 2, false),
            (_, _) => (2*n + face - 2, false),
        };
        let Surface::Plane { frame } = self.surface(face) else { unreachable!("prism faces are planar") };
        PrimitiveTrim { edge, forward, curve: planar_pcurve(&self.edge(edge).curve, frame), range: RANGE }
    }
}
