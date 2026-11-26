//! 🍩️ Exact boundary of a sphere cut by a concentric ring torus.

use semio_framework_3d_brep_curve::Curve3;
use semio_framework_3d_brep_vector::{Pnt3, Vec3, matrix::Frame3};
use std::f64::consts::{FRAC_PI_2, TAU};

/// 🌐️ Two spherical caps and the inward torus patch between their common circles.
#[derive(Clone, Copy)]
pub struct SphericalTorusCut {
    pub frame: Frame3,
    pub radius: f64,
    pub major: f64,
    pub minor: f64,
    rim_radius: f64,
    rim_height: f64,
    latitude: f64,
    tube_start: f64,
}

impl SphericalTorusCut {
    pub fn new(frame: Frame3, radius: f64, major: f64, minor: f64, tolerance: f64) -> Option<Self> {
        if [radius, major, minor].into_iter().any(|n| !n.is_finite() || n <= tolerance)
            || major - minor <= tolerance
            || radius - (major - minor) <= tolerance
            || major + minor - radius <= tolerance
        {
            return None;
        }
        let rim_radius = (radius * radius + (major - minor) * (major + minor)) / (2. * major);
        let rim_height = ((radius - rim_radius) * (radius + rim_radius)).sqrt();
        if !rim_radius.is_finite() || !rim_height.is_finite() || rim_height <= tolerance {
            return None;
        }
        Some(Self {
            frame, radius, major, minor, rim_radius, rim_height,
            latitude: rim_height.atan2(rim_radius),
            tube_start: rim_height.atan2(rim_radius - major),
        })
    }

    pub fn band(self, face: usize) -> [f64; 2] {
        match face {
            0 => [-FRAC_PI_2, -self.latitude],
            1 => [self.latitude, FRAC_PI_2],
            _ => [self.tube_start, TAU - self.tube_start],
        }
    }

    pub fn rim(self, upper: bool, u: f64) -> Pnt3 {
        self.frame.to_world(Pnt3::new(self.rim_radius * u.cos(), self.rim_radius * u.sin(),
                                     if upper { self.rim_height } else { -self.rim_height }))
    }

    pub fn volume(self) -> f64 {
        let [start, end] = self.band(2);
        let span = end - start;
        let cosine = end.sin() - start.sin();
        let cosine_squared = span / 2. + ((2. * end).sin() - (2. * start).sin()) / 4.;
        let tube = self.minor * (self.major * self.minor * (span + cosine_squared)
            + (self.major * self.major + self.minor * self.minor) * cosine);
        (2. * self.radius * self.radius * (self.radius - self.rim_height) - tube) * TAU / 3.
    }

    pub fn area(self) -> f64 {
        let [start, end] = self.band(2);
        TAU * (2. * self.radius * (self.radius - self.rim_height)
            + self.minor * (self.major * (end - start) + self.minor * (end.sin() - start.sin())))
    }

    pub fn signed_boundary_distance(self, point: Pnt3) -> f64 {
        let p = self.frame.to_local(point);
        let radial = p.x.hypot(p.y);
        let z = p.z.abs();
        let tube = radial - self.major;
        let latitude = z.atan2(radial).max(self.latitude);
        let angle = z.atan2(tube).max(self.tube_start);
        let cap_distance = (radial - self.radius * latitude.cos()).hypot(z - self.radius * latitude.sin());
        let tube_distance = (tube - self.minor * angle.cos()).hypot(z - self.minor * angle.sin());
        let distance = cap_distance.min(tube_distance);
        if radial.hypot(z) < self.radius && tube.hypot(z) > self.minor { -distance } else { distance }
    }

    pub fn bounds(self) -> [Pnt3; 2] {
        let extent = self.frame.z.to_array().map(|axis| {
            let axial = axis.abs().min(1.);
            if self.radius * axial >= self.rim_height { self.radius }
            else { self.rim_radius * (1. - axial * axial).sqrt() + self.rim_height * axial }
        });
        let extent = Vec3::from_array(extent);
        [self.frame.origin - extent, self.frame.origin + extent]
    }

    pub const RINGS: [usize; 3] = [4; 3];
    pub const VERTICES: usize = 4;
    pub const EDGES: usize = 7;

    pub fn vertex(self, index: usize) -> Pnt3 {
        match index {
            0 => self.frame.origin - self.frame.z * self.radius,
            1 => self.rim(false, 0.),
            2 => self.rim(true, 0.),
            3 => self.frame.origin + self.frame.z * self.radius,
            _ => unreachable!("spherical torus cut vertex"),
        }
    }

    pub fn edge(self, index: usize) -> (Curve3, (f64, f64), [usize; 2]) {
        match index {
            0 | 3 => {
                let pole = if index == 0 { 0 } else { 3 };
                (Curve3::Line { origin: self.vertex(pole), dir: Vec3::ZERO }, (0., TAU), [pole; 2])
            }
            1 | 2 => {
                let center = self.frame.origin + self.frame.z *
                    if index == 1 { -self.rim_height } else { self.rim_height };
                (Curve3::Circle { frame: Frame3 { origin: center, ..self.frame }, radius: self.rim_radius },
                    (0., TAU), [index; 2])
            }
            4..=6 => {
                let (origin, radius, vertices) = match index {
                    4 => (self.frame.origin, self.radius, [0, 1]),
                    5 => (self.frame.origin, self.radius, [2, 3]),
                    _ => (self.frame.origin + self.frame.x * self.major, self.minor, [2, 1]),
                };
                let frame = Frame3 { origin, x: self.frame.x, y: self.frame.z, z: -self.frame.y };
                let [start, end] = self.band(index - 4);
                (Curve3::Circle { frame, radius }, (start, end), vertices)
            }
            _ => unreachable!("spherical torus cut edge"),
        }
    }
}
