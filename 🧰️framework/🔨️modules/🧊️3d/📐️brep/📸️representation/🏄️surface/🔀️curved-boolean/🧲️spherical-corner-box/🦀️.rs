//! 🧲️ Exact union of a sphere and a box sharing one corner.

use semio_framework_3d_brep_curve::Curve3;
use semio_framework_3d_brep_vector::{Pnt3, Vec3, matrix::Frame3};
use std::f64::consts::{FRAC_PI_2, PI, TAU};

/// 📦️ Seven spherical octants and six cuboid faces with their common quarter-circle trims.
#[derive(Clone, Copy)]
pub struct SphericalCornerBox {
    pub frame: Frame3,
    pub radius: f64,
    pub dimensions: [f64; 3],
}

impl SphericalCornerBox {
    pub fn new(center: Pnt3, radius: f64, origin: Pnt3, dimensions: [f64; 3], tolerance: f64) -> Option<Self> {
        if !radius.is_finite() || radius <= tolerance
            || dimensions.into_iter().any(|length| !length.is_finite() || length - radius <= tolerance)
        {
            return None;
        }
        let mut signs = [1.; 3];
        let mut axes = [Vec3::X, Vec3::Y, Vec3::Z];
        let mut dimensions = dimensions;
        let lower = origin.to_array();
        for (axis, coordinate) in center.to_array().into_iter().enumerate() {
            if coordinate == lower[axis] + dimensions[axis] { signs[axis] = -1.; }
            else if coordinate != lower[axis] { return None; }
            axes[axis] = axes[axis] * signs[axis];
        }
        if signs.into_iter().product::<f64>() < 0. {
            axes.swap(0, 1);
            dimensions.swap(0, 1);
        }
        let [x, y, z] = axes;
        Some(Self { frame: Frame3 { origin: center, x, y, z }, radius, dimensions })
    }

    pub const SPHERE_FACES: usize = 7;
    pub const RINGS: [usize; 13] = [4,4,4,4,4,4,4,5,4,5,4,5,4];
    pub const VERTICES: usize = 13;
    pub const EDGES: usize = 31;
    const BOX_EDGES: usize = 19;

    pub fn volume(self) -> f64 {
        self.dimensions.into_iter().product::<f64>() + 7. * PI * self.radius.powi(3) / 6.
    }

    pub fn area(self) -> f64 {
        let [x,y,z] = self.dimensions;
        2. * (x*y + y*z + z*x) + 11. * PI * self.radius * self.radius / 4.
    }

    pub fn center_of_mass(self) -> Pnt3 {
        let box_volume = self.dimensions.into_iter().product::<f64>();
        let removed_moment = PI * self.radius.powi(4) / 16.;
        self.frame.to_world(Pnt3::from_array(self.dimensions.map(|length|
            (box_volume * length / 2. - removed_moment) / self.volume())))
    }

    pub fn bounds(self) -> [Pnt3; 2] {
        let center = self.frame.origin;
        let far = self.frame.to_world(Pnt3::from_array(self.dimensions));
        let center = center.to_array();
        let far = far.to_array();
        [Pnt3::from_array(std::array::from_fn(|axis| (center[axis] - self.radius).min(far[axis]))),
         Pnt3::from_array(std::array::from_fn(|axis| (center[axis] + self.radius).max(far[axis])))]
    }

    pub fn signed_boundary_distance(self, point: Pnt3) -> f64 {
        let local = self.frame.to_local(point).to_array();
        let point = Vec3::from_array(local);
        let mut candidate = local;
        if local.into_iter().all(|value| value > 0.) {
            let nearest_plane = (0..3).min_by(|&a,&b| local[a].total_cmp(&local[b])).unwrap();
            candidate[nearest_plane] = 0.;
        }
        let nearest = Vec3::from_array(candidate).normalized().unwrap_or(Vec3::X) * self.radius;
        let mut distance = (nearest - point).norm();
        for face in 0..6 {
            let (axis, u, v, upper) = Self::plane_axes(face);
            let mut a = local[u].clamp(0., self.dimensions[u]);
            let mut b = local[v].clamp(0., self.dimensions[v]);
            if !upper && a.hypot(b) < self.radius {
                let angle = if a == 0. && b == 0. && local[v] > local[u] { FRAC_PI_2 }
                            else { b.atan2(a) };
                a = self.radius * angle.cos();
                b = self.radius * angle.sin();
            }
            let h = if upper {self.dimensions[axis]} else {0.};
            distance = distance.min((local[u]-a).hypot(local[v]-b).hypot(local[axis]-h));
        }
        if point.norm() < self.radius || (0..3).all(|axis| (0. ..=self.dimensions[axis]).contains(&local[axis])) {
            -distance
        } else { distance }
    }

    pub fn sphere_chart(face: usize) -> (usize, bool) {
        if face < 4 { (face, false) } else { (face - 3, true) }
    }

    fn axis(self, index: usize) -> Vec3 { [self.frame.x, self.frame.y, self.frame.z][index] }

    fn radial(self, meridian: usize) -> Vec3 {
        [self.frame.x, self.frame.y, -self.frame.x, -self.frame.y][meridian % 4]
    }

    fn sphere_axis(axis: usize) -> usize { [0, 1, 5][axis] }

    pub fn vertex(self, index: usize) -> Pnt3 {
        match index {
            0..=3 => self.frame.origin + self.radial(index) * self.radius,
            4 | 5 => self.frame.origin + self.frame.z * if index == 4 {-self.radius} else {self.radius},
            6..=12 => self.frame.to_world(Pnt3::from_array(std::array::from_fn(|axis|
                if (index - 5) & (1 << axis) != 0 { self.dimensions[axis] } else { 0. }))),
            _ => unreachable!("spherical corner box vertex"),
        }
    }

    fn box_edge(self, index: usize) -> [usize; 2] {
        let axis = index / 4;
        let pair = index % 4;
        let u = (axis + 1) % 3;
        let v = (axis + 2) % 3;
        let bits = ((pair & 1) << u) | ((pair >> 1) << v);
        [if bits != 0 { 5 + bits } else { Self::sphere_axis(axis) }, 5 + (bits | (1 << axis))]
    }

    pub fn edge(self, index: usize) -> (Curve3, (f64, f64), [usize; 2]) {
        let origin = self.frame.origin;
        match index {
            0..=3 => {
                let x = self.radial(index);
                let y = self.radial(index+1);
                let frame = Frame3 { origin, x, y, z: self.frame.z };
                (Curve3::Circle { frame, radius: self.radius }, (0., FRAC_PI_2), [index,(index+1)%4])
            }
            4..=11 => {
                let longitude = index % 4;
                let x = self.radial(longitude);
                let y = self.frame.z;
                let frame = Frame3 { origin, x, y, z: x.cross(y) };
                let (range, vertices) = if index < 8 { ((-FRAC_PI_2,0.), [4,longitude]) }
                    else { ((0.,FRAC_PI_2), [longitude,5]) };
                (Curve3::Circle { frame, radius: self.radius }, range, vertices)
            }
            12..=18 => {
                let pole = if index < 16 {4} else {5};
                (Curve3::Line { origin: self.vertex(pole), dir: Vec3::ZERO }, (0.,FRAC_PI_2), [pole;2])
            }
            Self::BOX_EDGES..=30 => {
                let vertices = self.box_edge(index - Self::BOX_EDGES);
                let [a,b] = vertices.map(|vertex|self.vertex(vertex));
                (Curve3::Line { origin:a,dir:b-a }, (0.,1.), vertices)
            }
            _=>unreachable!("spherical corner box edge"),
        }
    }

    fn plane_axes(face: usize) -> (usize, usize, usize, bool) {
        let axis = face / 2;
        let upper = face % 2 == 1;
        let [u,v] = if upper {[(axis+1)%3,(axis+2)%3]} else {[(axis+2)%3,(axis+1)%3]};
        (axis,u,v,upper)
    }

    pub fn plane(self, face: usize) -> Frame3 {
        let (axis,u,v,upper) = Self::plane_axes(face);
        let origin = if upper {self.frame.origin+self.axis(axis)*self.dimensions[axis]} else {self.frame.origin};
        let (x,y) = (self.axis(u), self.axis(v));
        Frame3 { origin, x, y, z:x.cross(y) }
    }

    pub fn plane_corner(self, face: usize, member: usize) -> usize {
        let (axis,u,v,upper) = Self::plane_axes(face);
        let base = if upper {1<<axis} else {0};
        let box_corner = |bits: usize| 5 + (base | bits);
        if upper { [box_corner(0),box_corner(1<<u),box_corner((1<<u)|(1<<v)),box_corner(1<<v)][member] }
        else { [box_corner(1<<u),box_corner((1<<u)|(1<<v)),box_corner(1<<v),Self::sphere_axis(v),Self::sphere_axis(u)][member] }
    }

    pub fn plane_edge(self, face: usize, member: usize) -> (usize,bool) {
        let count = Self::RINGS[Self::SPHERE_FACES+face];
        let from = self.plane_corner(face,member);
        let to = self.plane_corner(face,(member+1)%count);
        let (first,end) = if face%2==0 && member==3 {
            let arc = [9,8,0][face/2];
            (arc,arc+1)
        } else {(Self::BOX_EDGES,Self::EDGES)};
        (first..end).find_map(|edge| {
            let (_,_,[a,b]) = self.edge(edge);
            if [a,b]==[from,to] {Some((edge,true))}
            else if [b,a]==[from,to] {Some((edge,false))}
            else {None}
        }).expect("spherical corner box connected face")
    }

    pub fn sphere_point(self, face: usize, u: f64, fraction: f64) -> (Pnt3,Vec3) {
        let (quadrant, upper) = Self::sphere_chart(face);
        let angle = u * FRAC_PI_2;
        let (sin,cos) = angle.sin_cos();
        let direction = if u == 0. {self.radial(quadrant)} else if u == 1. {self.radial(quadrant+1)}
                        else {self.radial(quadrant)*cos + self.radial(quadrant+1)*sin};
        let v = if upper {fraction*FRAC_PI_2} else {fraction*FRAC_PI_2-FRAC_PI_2};
        let normal = if fraction == 0. && !upper {-self.frame.z} else if fraction == 1. && upper {self.frame.z}
                        else {direction*v.cos()+self.frame.z*v.sin()};
        (self.frame.origin+normal*self.radius,normal)
    }
}
