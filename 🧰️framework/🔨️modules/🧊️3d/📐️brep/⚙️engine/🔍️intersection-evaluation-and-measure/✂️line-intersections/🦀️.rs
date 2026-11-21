//! ✂️ Bounded intersections of analytic supporting lines and planes.

use super::*;
use semio_framework_3d_brep_intersect::{intersect_original_curve_curve, intersect_original_curve_plane, LineIntersectionRefusal};

impl Brep {
    /// ✂️ Resolves an original supporting curve or an edge's bounded supporting geometry.
    pub fn original_curve_curve_contacts(&self, a: &str, b: &str, tolerance: f64) -> Result<[Option<EVec3>; 4], ValueError> {
        let left = self.native.original_intersection_curve(a)?;
        let right = self.native.original_intersection_curve(b)?;
        let hits = intersect_original_curve_curve(left, right, tolerance).map_err(refusal)?;
        Ok(hits.ok_or_else(|| ValueError::literal(ValueRefusalKind::UnsupportedOwner, "original curve intersection requires analytic lines, circles or ellipses"))?
            .map(|hit| hit.map(|hit| evec(hit.point))))
    }

    /// 🪜️ Quotes the original edge and curve arena traversal in addition to the live handle.
    pub fn original_intersection_curve_source_depth(&self, handle: &str) -> Result<usize, ValueError> {
        self.native.original_intersection_curve_depth(handle)
    }

    /// 🪟️ Resolves up to four exact contacts with a live analytic plane.
    pub fn original_curve_plane_contacts(&self, curve:&str, surface:&str,tolerance:f64)->Result<[Option<EVec3>;4],ValueError> {
        let frame = match self.native.original_query_entity(surface)? {
            Entity::Surface(Surface::Plane { frame }, _) => frame,
            Entity::Surface(..) => return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "retained curve/surface intersection requires a plane")),
            _ => return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "curve/surface intersection requires a surface")),
        };
        let contacts=intersect_original_curve_plane(self.native.original_intersection_curve(curve)?,frame,tolerance).map_err(refusal)?;
        contacts.map(|hits|hits.map(|hit|hit.map(|h|evec(h.point)))).ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"retained plane intersection requires a supported line, circle, ellipse or bounded spline"))
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

fn refusal(error: LineIntersectionRefusal) -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, error.message())
}
