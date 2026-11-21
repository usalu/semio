//! 🔳️ Original exact axis intervals and funded box construction.

use super::*;
use semio_framework_3d_brep_analysis::{analytic_curve_bounds, nearest_analytic};

impl BrepRegistry {
    /// 🎟️ Quotes all original lookup paths for the selected analytic bound.
    pub fn analytic_bounds_depth(&self, handle: &str) -> Result<usize, ValueError> {
        let depth = self.tessellation_source_depth_demand(handle)?.max(1);
        match self.original_query_entity(handle)? {
            Entity::Solid(id, Some(_)) => Ok(depth.max(self.body.solids.next_remove_depth_demand(*id)?)),
            Entity::Face(_) => self.planar_face_boundary_depth(handle),
            Entity::Edge(_) => Ok(depth.max(self.original_intersection_curve_depth(handle)?)),
            _ => Ok(depth),
        }
    }

    /// 📦️ Reads exact world extrema of one original finite analytic geometry.
    pub fn original_axis_bounds(&self, handle: &str) -> Result<Option<[Pnt3; 2]>, ValueError> {
        let bounds = |curve: &Curve3, range| analytic_curve_bounds(curve, range).map(|aabb| [pnt(aabb.min), pnt(aabb.max)]);
        Ok(match self.original_query_entity(handle)? {
            Entity::Solid(id, Some(provenance)) if self.body.solids.contains(*id) => match provenance {
                SolidProvenance::Primitive(source) => Some(source.primitive.axis_aligned_bounds()),
                SolidProvenance::RoundedBox(source,_) => Some(source.bounds().map(pnt)),
                SolidProvenance::ClippedBox(source) => source.bounds(),
                SolidProvenance::Bore(source) => {
                    let origin = pnt(source.origin);
                    Some([origin, origin + vec3(source.dimensions)])
                }
                SolidProvenance::SphericalShell(shell,_) => Some(nearest_analytic::spherical_shell_bounds(*shell)),
                SolidProvenance::SphericalCaps(caps,_) => Some(nearest_analytic::spherical_caps_bounds(*caps)),
                SolidProvenance::Frustum(shape,_) => Some(nearest_analytic::frustum_bounds(shape.frame,shape.radii,shape.height)),
                SolidProvenance::PlanarBoundary(..) | SolidProvenance::Orthogonal(_) | SolidProvenance::PlacedOrthogonal(..) | SolidProvenance::CircularPrism(..) | SolidProvenance::TorusSector(..) | SolidProvenance::Annular(..) => None,
            },
            Entity::Curve(curve, _) => bounds(curve, curve.domain()),
            Entity::Edge(id) => {
                let edge = self.body.edges.get(*id).ok_or_else(|| ValueError::invariant("live bounds edge is missing"))?;
                let curve = self.body.curves3.get(edge.curve).ok_or_else(|| ValueError::invariant("live bounds curve is missing"))?;
                bounds(curve, edge.range)
            }
            Entity::Face(id) => self.conic_face_boundary(*id).and_then(|(curve, _, _)| bounds(curve, curve.domain())),
            Entity::Surface(Surface::Sphere { frame, radius }, _) => {
                let extent = NativeVec3::new(*radius, *radius, *radius);
                Some([frame.origin + -extent, frame.origin + extent])
            }
            Entity::Surface(Surface::Torus { frame, major_radius, minor_radius }, _) =>
                Some(SolidPrimitive::Torus { frame: *frame, major: *major_radius, minor: *minor_radius }.axis_aligned_bounds()),
            _ => None,
        })
    }

    /// 🧱️ Plans a solid covering a bounded analytic source's exact world-axis interval.
    pub fn plan_analytic_bounds(&mut self, handle: &str) -> Result<SolidPublication, ValueError> {
        let [min, max] = self.original_axis_bounds(handle)?.ok_or_else(|| ValueError::literal(
            ValueRefusalKind::UnsupportedOwner, "original Bounding Box requires finite analytic geometry",
        ))?;
        let span = max - min;
        if [span.x, span.y, span.z].into_iter().any(|axis| !axis.is_finite() || axis <= Tol::DEFAULT.value()) {
            return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "bounding box has a collapsed or nonfinite extent"));
        }
        self.plan_solid_primitive(SolidPrimitive::Box { origin: min, width: span.x, depth: span.y, height: span.z })
    }
}
