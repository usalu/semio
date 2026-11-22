//! 📦️ Exact rounded rectangular stock as an inset box swept by a ball.

use super::box_finish_layout::{EDGES, FACES, TOPOLOGY, VERTICES, face, sign};
use semio_framework_3d_brep_curve::Curve3;
use semio_framework_3d_brep_vector::{Pnt3, Vec3, matrix::Frame3};
use std::f64::consts::{FRAC_1_SQRT_2, FRAC_PI_2, PI};

/// 🟠️ Exact surface and topology certificate for one uniformly finished box.
#[derive(Clone, Copy)]
pub struct RoundedBox {
    inset: [[f64; 2]; 3],
    radius: f64,
}

impl RoundedBox {
    pub fn new(origin: Pnt3, size: [f64; 3], radius: f64, tolerance: f64) -> Option<Self> {
        let low = origin.to_array();
        let high: [f64; 3] = std::array::from_fn(|i| low[i] + size[i]);
        let shape = Self::from_inset(std::array::from_fn(|i| [low[i] + radius, high[i] - radius]), radius, tolerance)?;
        let within = |error: f64| error.abs() <= tolerance / 4.;
        shape.inset.iter().enumerate().all(|(i, &[a, b])|
            within(a - radius - low[i]) && within(b + radius - high[i])
            && within(a - low[i] - radius) && within(high[i] - b - radius)).then_some(shape)
    }

    /// 🪨️ Reconstructs a rounded boundary recipe from its original inset intervals.
    pub fn from_inset(inset: [[f64; 2]; 3], radius: f64, tolerance: f64) -> Option<Self> {
        let shape = Self { inset, radius };
        (radius.is_finite() && radius > tolerance && tolerance >= 0.
            && inset.iter().all(|&[a, b]| b - a > tolerance && (a - radius).is_finite() && (b + radius).is_finite())
            && shape.volume().is_finite() && shape.area().is_finite()).then_some(shape)
    }

    pub const fn vertex_count(self) -> usize { VERTICES }
    pub const fn edge_count(self) -> usize { EDGES }
    pub const fn face_count(self) -> usize { FACES }
    pub const fn ring_sizes(self) -> &'static [usize] { &TOPOLOGY.ring_sizes }
    pub const fn radius(self) -> f64 { self.radius }

    pub fn corner(self, corner: usize) -> Pnt3 { Pnt3::from_array(std::array::from_fn(|i| self.inset[i][(corner >> i) & 1])) }

    fn axis(corner: usize, axis: usize) -> Vec3 {
        Vec3::from_array(std::array::from_fn(|i| if i == axis { sign((corner >> i) & 1) } else { 0. }))
    }

    pub fn vertex(self, id: usize) -> Pnt3 {
        self.corner(id / 3) + Self::axis(id / 3, id % 3) * self.radius
    }

    pub fn edge_is_arc(self, index: usize) -> bool {
        let [a,b] = TOPOLOGY.edges[index];
        a / 3 == b / 3
    }

    pub fn edge(self, index: usize) -> ([usize; 2], Curve3, (f64, f64)) {
        let ids @ [a,b] = TOPOLOGY.edges[index];
        if self.edge_is_arc(index) {
            let origin = self.corner(a / 3);
            let x = Self::axis(a / 3, a % 3);
            let y = Self::axis(b / 3, b % 3);
            let frame = Frame3 { origin, x, y, z: x.cross(y) };
            (ids, Curve3::Circle { frame, radius: self.radius }, (0., FRAC_PI_2))
        } else {
            let [origin,end] = ids.map(|id|self.vertex(id));
            (ids, Curve3::Line { origin, dir: end - origin }, (0., 1.))
        }
    }

    pub fn face_frame(self, index: usize) -> Frame3 {
        if index < 6 {
            let ring = face(index);
            return Frame3::from_normal(self.vertex(ring.vertices[0]), Vec3::from_array(ring.normal))
                .expect("fixed rounded-box plane normal");
        }
        if index < 18 {
            let (axis, sides) = ((index-6) / 4, (index-6) % 4);
            let (u,v) = ((axis+1) % 3, (axis+2) % 3);
            let corner = ((sides % 2) << u) | ((sides / 2) << v);
            let (x,y) = (Self::axis(corner,u), Self::axis(corner,v));
            let z = x.cross(y);
            let start = corner | (usize::from(z.to_array()[axis] < 0.) << axis);
            return Frame3 { origin: self.corner(start), x, y, z };
        }
        let corner = index-18;
        let (mut x,mut y,z) = (Self::axis(corner,0), Self::axis(corner,1), Self::axis(corner,2));
        if x.cross(y).dot(z) < 0. { std::mem::swap(&mut x,&mut y); }
        Frame3 { origin: self.corner(corner), x, y, z }
    }

    pub fn cylinder_end(self, face: usize) -> Pnt3 {
        let frame = self.face_frame(face);
        let axis = (face-6)/4;
        let mut end = frame.origin.to_array();
        end[axis] = self.inset[axis][usize::from(frame.z.to_array()[axis]>0.)];
        Pnt3::from_array(end)
    }

    pub fn trim_uv(self, face: usize, ids: [usize; 2]) -> [[f64; 2]; 2] {
        let frame = self.face_frame(face);
        let normals = ids.map(|id| Self::axis(id / 3, id % 3));
        let azimuth = |n: Vec3| if n.dot(frame.x) > 0. { 0. } else { FRAC_PI_2 };
        std::array::from_fn(|i| {
            let normal = normals[i];
            if face < 18 {
                [azimuth(normal), (self.corner(ids[i] / 3)-frame.origin).dot(frame.z)]
            } else if normal.dot(frame.z) > 0. {
                [azimuth(normals[1-i]), FRAC_PI_2]
            } else {
                [azimuth(normal), 0.]
            }
        })
    }

    pub fn bounds(self) -> [[f64; 3]; 2] {
        std::array::from_fn(|i| std::array::from_fn(|axis|self.inset[axis][i]+sign(i)*self.radius))
    }

    pub fn center(self) -> [f64; 3] { self.inset.map(|[a,b]|a+(b-a)*0.5) }

    pub fn signed_boundary_distance(self, point: [f64; 3]) -> f64 {
        let [x,y,z] = std::array::from_fn(|i| {
            let [low,high] = self.inset[i];
            (low-point[i]).max(point[i]-high)
        });
        x.max(0.).hypot(y.max(0.)).hypot(z.max(0.)) + x.max(y).max(z).min(0.) - self.radius
    }

    pub fn volume(self) -> f64 {
        let [a,b,c] = self.inset.map(|[a,b]|b-a);
        let r = self.radius;
        a*b*c + 2.*r*(a*b+b*c+c*a) + PI*r*r*(a+b+c) + 4.*PI*r*r*r/3.
    }

    pub fn area(self) -> f64 {
        let [a,b,c] = self.inset.map(|[a,b]|b-a);
        let r = self.radius;
        2.*(a*b+b*c+c*a) + 2.*PI*r*(a+b+c) + 4.*PI*r*r
    }
}

/// 🧭️ Samples a quarter-circle with identical coordinates under endpoint reversal.
pub fn quadrant(step: usize, segments: usize) -> [f64; 2] {
    if step > segments/2 {
        let [a,b] = quadrant(segments-step,segments);
        [b,a]
    } else if step == 0 {
        [1.,0.]
    } else if step == segments-step {
        [FRAC_1_SQRT_2; 2]
    } else {
        let (s,c) = (FRAC_PI_2*step as f64/segments as f64).sin_cos();
        [c,s]
    }
}
