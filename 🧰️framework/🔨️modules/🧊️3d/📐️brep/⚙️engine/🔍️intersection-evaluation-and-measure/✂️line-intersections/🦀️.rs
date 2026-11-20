//! ✂️ Bounded intersections of analytic supporting lines and planes.

use super::*;
use semio_framework_3d_brep_intersect::{intersect_line_line_unique, intersect_line_plane_unique, LineIntersectionRefusal};

impl Brep {
    fn original_line(&self, handle: &str) -> Result<(Pnt3, NativeVec3), ValueError> {
        match self.original_curve(handle)? {
            Curve3::Line { origin, dir } => Ok((*origin, *dir)),
            _ => Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "retained linear intersection requires a line")),
        }
    }

    /// 📍️ Plans one native vertex for the unique intersection of two supporting lines.
    pub fn plan_original_line_line(&mut self, a: &str, b: &str, tolerance: f64) -> Result<VertexPublication, ValueError> {
        let (o1, d1) = self.original_line(a)?;
        let (o2, d2) = self.original_line(b)?;
        let hit = intersect_line_line_unique(o1, d1, o2, d2, tolerance).map_err(refusal)?.ok_or_else(no_intersection)?;
        self.plan_vertex_publication(evec(hit.point))
    }

    /// 🪟️ Plans one native vertex for a unique supporting line and plane intersection.
    pub fn plan_original_line_plane(&mut self, curve: &str, surface: &str, tolerance: f64) -> Result<VertexPublication, ValueError> {
        let (origin, direction) = self.original_line(curve)?;
        let frame = match self.native.original_query_entity(surface)? {
            Entity::Surface(Surface::Plane { frame }, _) => frame,
            Entity::Surface(..) => return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "retained curve/surface intersection requires a plane")),
            _ => return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "curve/surface intersection requires a surface")),
        };
        let hit = intersect_line_plane_unique(origin, direction, frame, tolerance).map_err(refusal)?.ok_or_else(no_intersection)?;
        self.plan_vertex_publication(evec(hit.point))
    }
    /// 🧭️ Plans the exact continuous curve shared by two live analytic surfaces.
    pub fn plan_original_surface_intersection(&mut self, a: &str, b: &str, tolerance: f64) -> Result<AnalyticPublication, ValueError> {
        use semio_framework_3d_brep_intersect::{intersect_original_analytic_surfaces, ExactSurfaceRefusal};
        let surface = |key| match self.native.original_query_entity(key)? {
            Entity::Surface(surface, _) => Ok(surface),
            _ => Err(ValueError::literal(ValueRefusalKind::InvalidValue, "surface intersection requires surface inputs")),
        };
        let curve = intersect_original_analytic_surfaces(surface(a)?, surface(b)?, tolerance).map_err(|error| {
            let kind = if matches!(error, ExactSurfaceRefusal::Unsupported) { ValueRefusalKind::UnsupportedOwner } else { ValueRefusalKind::InvalidValue };
            ValueError::literal(kind, error.message())
        })?.ok_or_else(|| ValueError::literal(ValueRefusalKind::InvalidValue, "no intersection wire"))?;
        Ok(self.native.plan_prepared_analytic_curve(curve))
    }
}

fn no_intersection() -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, "no intersection")
}

fn refusal(error: LineIntersectionRefusal) -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, error.message())
}
