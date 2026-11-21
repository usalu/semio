//! 🛤️ Geometry and admission for original polygonal and sampled helix wires.

use semio_framework_3d_brep_representation::contract::Vec3 as EVec3;
use semio_framework_3d_brep_tolerance::Tol;
use semio_framework_3d_brep_vector::{Pnt3, Vec3, matrix::Frame3};
use semio_framework_value::{ValueError, ValueRefusalKind};

/// 🌀️ Samples the helix of the original kernel wire constructor.
#[derive(Clone, Copy)]
pub struct HelixWireRecipe {
    frame: Frame3,
    axis: Vec3,
    radius: f64,
    pitch: f64,
    turns: f64,
    segments: usize,
}

impl HelixWireRecipe {
    /// 📐️ Binds the dimensions, stock axis fallback and original chord density.
    pub fn new(origin: EVec3, axis: EVec3, radius: f64, pitch: f64, turns: f64) -> Result<Self, ValueError> {
        let invalid = || ValueError::literal(ValueRefusalKind::InvalidValue, "helix requires finite parameters and a representable sample count");
        if !origin.into_iter().chain(axis).chain([radius, pitch, turns]).all(f64::is_finite) { return Err(invalid()); }
        let axis = Vec3::new(axis[0], axis[1], axis[2]).normalized().unwrap_or(Vec3::Z);
        let frame = Frame3::from_normal(Pnt3::new(origin[0], origin[1], origin[2]), axis).ok_or_else(invalid)?;
        let segments = ((turns.abs() * 32.0).ceil() as usize).max(8);
        if segments.checked_add(1).is_none() { return Err(invalid()); }
        Ok(Self { frame, axis, radius, pitch, turns, segments })
    }

    /// 🔢️ Counts the samples, including both ends of the original parameter interval.
    pub fn sample_count(self) -> usize { self.segments + 1 }

    /// 📍️ Computes one authored helix position in its local axis frame.
    pub fn sample(self, index: usize) -> EVec3 {
        let turn = index as f64 / self.segments as f64 * self.turns;
        let angle = turn * std::f64::consts::TAU;
        let frame = self.frame;
        let point = frame.origin + frame.x * (self.radius * angle.cos()) + frame.y * (self.radius * angle.sin()) + self.axis * (self.pitch * turn);
        [point.x, point.y, point.z]
    }
}

/// 🔖️ Construction source for one funded polygonal profile.
#[derive(Clone, Copy)]
pub enum PolygonWireRecipe {
    Rectangle { width: f64, height: f64 },
    Regular { radius: f64, sides: usize },
    Helix(HelixWireRecipe),
    Polyline { vertices: usize, closed: bool },
    Contacts { points: [EVec3; 4], count: usize },
}

impl PolygonWireRecipe {
    fn counts(self) -> (usize, bool) {
        match self {
            Self::Rectangle { .. } => (4, true),
            Self::Regular { sides, .. } => (sides, true),
            Self::Helix(helix) => {
                let closed = helix.sample(0) == helix.sample(helix.segments);
                (helix.sample_count() - usize::from(closed), closed)
            }
            Self::Polyline { vertices, closed } => (vertices, closed),
            Self::Contacts { count, .. } => (count, false),
        }
    }

    /// 🎟️ Admits the vertex count and closure with the tolerance used for publication.
    pub fn admit(self) -> Result<(usize, bool, Tol), ValueError> {
        let positive = |value: f64| value.is_finite() && value > Tol::DEFAULT.value();
        let (vertex_count, closed) = self.counts();
        let valid = match self {
            Self::Rectangle { width, height } => positive(width) && positive(height),
            Self::Regular { radius, .. } => positive(radius),
            Self::Helix(_) | Self::Polyline { .. } => true,
            Self::Contacts { points, count } => points.get(..count).is_some_and(|points| {
                points.iter().flatten().all(|coordinate| coordinate.is_finite())
                    && points.windows(2).all(|pair| pair[0] != pair[1])
            }),
        };
        if !valid || vertex_count < (if closed { 3 } else { 2 }) || vertex_count > 65_536 {
            return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "polygonal wire has invalid dimensions or point count"));
        }
        let mut tolerance = Tol::DEFAULT;
        if let Self::Contacts { points, count } = self {
            for pair in points[..count].windows(2) {
                let [x, y, z] = std::array::from_fn(|axis| pair[1][axis] - pair[0][axis]);
                let length = x.hypot(y).hypot(z);
                tolerance = Tol::new(tolerance.value().min(length * 0.25));
            }
        }
        Ok((vertex_count, closed, tolerance))
    }

    /// 📍️ Samples an admitted polygon vertex. Polyline vertices are supplied by the caller.
    pub fn vertex(self, index: usize) -> Option<EVec3> {
        match self {
            Self::Rectangle { width, height } => Some([[0., 0., 0.], [width, 0., 0.], [width, height, 0.], [0., height, 0.]][index]),
            Self::Regular { radius, sides } => {
                let angle = std::f64::consts::TAU * index as f64 / sides as f64;
                let (sin, cos) = angle.sin_cos();
                Some([radius * cos, radius * sin, 0.])
            }
            Self::Helix(helix) => Some(helix.sample(index)),
            Self::Polyline { .. } => None,
            Self::Contacts { points, .. } => Some(points[index]),
        }
    }
}
