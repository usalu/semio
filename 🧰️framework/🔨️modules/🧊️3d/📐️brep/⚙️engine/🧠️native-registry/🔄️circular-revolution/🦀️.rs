//! 🔄️ Original complete revolutions of certified meridional circular faces.

use super::*;
use std::f64::consts::TAU;

fn unsupported(message: &'static str) -> ValueError {
    ValueError::literal(ValueRefusalKind::UnsupportedOwner, message)
}

impl BrepRegistry {
    /// 🎟️ Quotes the live profile and original arena paths needed to certify its planar trim.
    pub fn planar_face_boundary_depth(&self, handle: &str) -> Result<usize, ValueError> {
        let mut depth = self.live.next_contains_depth_demand(handle)?.max(1);
        let Entity::Face(id) = self.original_query_entity(handle)? else { return Ok(depth); };
        depth = depth.max(self.body.faces.next_remove_depth_demand(*id)?);
        let Some(face) = self.body.faces.get(*id) else { return Ok(depth); };
        depth = depth.max(self.body.surfaces.next_remove_depth_demand(face.surface)?);
        let Some(ring) = face.outer else { return Ok(depth); };
        depth = depth.max(self.body.loops.next_remove_depth_demand(ring)?);
        let Some(ring) = self.body.loops.get(ring) else { return Ok(depth); };
        depth = depth.max(self.body.coedges.next_remove_depth_demand(ring.first)?);
        let Some(coedge) = self.body.coedges.get(ring.first) else { return Ok(depth); };
        if let Some(trim) = coedge.pcurve {
            depth = depth.max(self.body.curves2.next_remove_depth_demand(trim)?);
        }
        depth = depth.max(self.body.edges.next_remove_depth_demand(coedge.edge)?);
        if let Some(edge) = self.body.edges.get(coedge.edge) {
            depth = depth.max(self.body.curves3.next_remove_depth_demand(edge.curve)?);
        }
        Ok(depth)
    }

    /// 🧭️ Plans the exact ring torus swept by a circle around a coplanar external axis.
    pub fn plan_circular_revolution(&mut self, handle: &str, origin: EVec3, direction: EVec3, angle: f64)
        -> Result<SolidPublication, ValueError> {
        let invalid_axis = || ValueError::literal(ValueRefusalKind::InvalidValue, "revolution requires a finite axis with nonzero direction");
        let axis_length = direction[0].hypot(direction[1]).hypot(direction[2]);
        if !axis_length.is_finite() || axis_length <= Tol::DEFAULT.value() || origin.iter().any(|coordinate| !coordinate.is_finite()) {
            return Err(invalid_axis());
        }
        if !angle.is_finite() || (angle.abs() - TAU).abs() > 16.0 * f64::EPSILON * TAU {
            return Err(unsupported("original circular revolution requires one full turn"));
        }
        let Entity::Face(id) = self.original_query_entity(handle)? else {
            return Err(unsupported("original circular revolution requires a live circular face"));
        };
        let Some((Curve3::Circle { frame, radius }, _, _)) = self.conic_face_boundary(*id) else {
            return Err(unsupported("original circular revolution requires one full circular face boundary"));
        };
        let normal = frame.z;
        let center = frame.origin;
        let minor = *radius;
        let axis = vec3(direction.map(|value| value / axis_length));
        let origin = pnt(origin);
        let along_axis = (center - origin).dot(axis);
        let axis_center = origin + axis * along_axis;
        let radial = center - axis_center;
        let major = radial.x.hypot(radial.y).hypot(radial.z);
        if !major.is_finite() || major - minor <= Tol::DEFAULT.value() {
            return Err(unsupported("original circular revolution requires clearance between profile and axis"));
        }
        let radial = radial * major.recip();
        let plane_tol = 1e-10;
        if normal.dot(axis).abs() > plane_tol || normal.dot(radial).abs() > plane_tol {
            return Err(unsupported("original circular revolution requires the axis in the profile plane"));
        }
        let y = axis.cross(radial).normalized().ok_or_else(invalid_axis)?;
        let frame = Frame3 { origin: axis_center, x: y.cross(axis), y, z: axis };
        self.plan_solid_primitive(SolidPrimitive::Torus { frame, major, minor })
    }
}
