//! 🛤️ Exact original sweeps along one directed line trim.

use super::*;

fn unsupported(message: &'static str) -> ValueError {
    ValueError::literal(ValueRefusalKind::UnsupportedOwner, message)
}

fn invalid(message: &'static str) -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, message)
}

impl BrepRegistry {
    fn straight_sweep_path(&self, path: &str) -> Result<(EdgeId, bool), ValueError> {
        let Entity::Wire(wire, _) = self.original_query_entity(path)? else {
            return Err(unsupported("original straight sweep requires an open one-edge line wire"));
        };
        match wire.members.as_slice() {
            [entry] if !wire.closed => Ok(*entry),
            _ => Err(unsupported("original straight sweep requires an open one-edge line wire")),
        }
    }

    /// 🎟️ Quotes the original planar profile and directed path edge lookup.
    pub fn straight_sweep_depth(&self, profile: &str, path: &str) -> Result<usize, ValueError> {
        let depth = self.planar_face_boundary_depth(profile)?.max(self.tessellation_source_depth_demand(path)?);
        let (edge, _) = self.straight_sweep_path(path)?;
        let record = self.body.edges.get(edge).ok_or_else(|| ValueError::invariant("live sweep path edge is missing"))?;
        Ok(depth.max(self.body.edges.next_remove_depth_demand(edge)?)
            .max(self.body.curves3.next_remove_depth_demand(record.curve)?))
    }

    /// 🧱️ Stages a cylinder or planar prism in the profile frame using the directed path displacement.
    pub fn plan_straight_sweep(&mut self, profile: &str, path: &str) -> Result<PlanarExtrusion, ValueError> {
        let (edge, forward) = self.straight_sweep_path(path)?;
        let edge = self.body.edges.get(edge).ok_or_else(|| ValueError::invariant("live sweep path edge is missing"))?;
        let Some(Curve3::Line { origin, dir }) = self.body.curves3.get(edge.curve) else {
            return Err(unsupported("original straight sweep requires a line path trim"));
        };
        let start = *origin + *dir * edge.range.0;
        let end = *origin + *dir * edge.range.1;
        let displacement = if forward { end - start } else { start - end };
        let length = displacement.norm();
        if !length.is_finite() || length <= Tol::DEFAULT.value() {
            return Err(invalid("sweep path must have a finite nonzero length at kernel tolerance"));
        }

        let unsupported_profile = || unsupported("original straight sweep requires a planar line polygon or an axial circular face");
        let Entity::Face(id) = self.original_query_entity(profile)? else { return Err(unsupported_profile()); };
        let face = self.body.faces.get(*id).ok_or_else(unsupported_profile)?;
        let Some(Surface::Plane { frame }) = self.body.surfaces.get(face.surface) else { return Err(unsupported_profile()); };
        let direction = displacement * (1.0 / length);
        if direction.dot(frame.z).abs() <= 1e-9 {
            return Err(invalid("sweep path direction lies in the profile plane"));
        }
        if let Some((curve, _, _)) = self.conic_face_boundary(*id) {
            if !matches!(curve, Curve3::Circle { .. }) || displacement.cross(frame.z).norm() > Tol::DEFAULT.value() {
                return Err(unsupported_profile());
            }
        }
        self.plan_planar_profile_extrusion(profile, [displacement.x, displacement.y, displacement.z])
    }
}
