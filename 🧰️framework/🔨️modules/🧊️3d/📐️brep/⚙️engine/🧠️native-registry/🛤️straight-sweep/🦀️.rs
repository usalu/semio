//! 🛤️ Exact original sweeps along one directed line trim.

use super::*;

fn unsupported(message: &'static str) -> ValueError {
    ValueError::literal(ValueRefusalKind::UnsupportedOwner, message)
}

fn invalid(message: &'static str) -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, message)
}

impl BrepRegistry {
    fn straight_sweep_member(&self, path: &str) -> Result<(EdgeId, bool), ValueError> {
        let Entity::Wire(wire, _) = self.original_query_entity(path)? else {
            return Err(unsupported("straight Sweep and Pipe require open one-edge line wires"));
        };
        match wire.members.as_slice() {
            [entry] if !wire.closed => Ok(*entry),
            _ => Err(unsupported("straight Sweep and Pipe require open one-edge line wires")),
        }
    }

    /// 🎟️ Quotes the planar profile and each supplied rail's original trim lookup.
    pub fn straight_sweep_depth(&self, profile: &str, path: &str, guide: Option<&str>) -> Result<usize, ValueError> {
        let mut depth = self.planar_face_boundary_depth(profile)?;
        for rail in std::iter::once(path).chain(guide) {
            let (edge, _) = self.straight_sweep_member(rail)?;
            let record = self.body.edges.get(edge).ok_or_else(|| ValueError::invariant("live sweep rail edge is missing"))?;
            depth = depth.max(self.tessellation_source_depth_demand(rail)?)
                .max(self.body.edges.next_remove_depth_demand(edge)?)
                .max(self.body.curves3.next_remove_depth_demand(record.curve)?);
        }
        Ok(depth)
    }

    fn straight_sweep_trim(&self, rail: &str) -> Result<[Pnt3; 2], ValueError> {
        let (edge, forward) = self.straight_sweep_member(rail)?;
        let edge = self.body.edges.get(edge).ok_or_else(|| ValueError::invariant("live sweep rail edge is missing"))?;
        let Some(Curve3::Line { origin, dir }) = self.body.curves3.get(edge.curve) else {
            return Err(unsupported("straight Sweep and Pipe require line trims"));
        };
        let start = *origin + *dir * edge.range.0;
        let end = *origin + *dir * edge.range.1;
        let length = (end - start).norm();
        if !length.is_finite() || length <= Tol::DEFAULT.value() {
            return Err(invalid("sweep rail must have a finite nonzero length at kernel tolerance"));
        }
        Ok(if forward { [start, end] } else { [end, start] })
    }

    fn straight_guide_frame(&self, guide: &str, start: Pnt3, direction: NativeVec3) -> Result<Frame3, ValueError> {
        let [first, last] = self.straight_sweep_trim(guide)?.map(|point| {
            let offset = point - start;
            offset - direction * offset.dot(direction)
        });
        let tolerance = Tol::DEFAULT.value();
        let refusal = || unsupported("straight Pipe requires a nonzero constant guide direction");
        let length = first.norm();
        if !length.is_finite() || length <= tolerance { return Err(refusal()); }
        let x = first * (1.0 / length);
        if !(last.dot(x) > tolerance && last.cross(x).norm() <= tolerance) {
            return Err(refusal());
        }
        Ok(Frame3 { origin: start, x, y: direction.cross(x), z: direction })
    }

    /// 🧱️ Stages a straight solid with the profile's original or guide-aligned placement.
    pub fn plan_straight_sweep(&mut self, profile: &str, path: &str, guide: Option<&str>) -> Result<PlanarExtrusion, ValueError> {
        let [start, end] = self.straight_sweep_trim(path)?;
        let displacement = end - start;
        let direction = displacement * (1.0 / displacement.norm());
        let vector = [displacement.x, displacement.y, displacement.z];
        if let Some(guide) = guide {
            let frame = self.straight_guide_frame(guide, start, direction)?;
            return self.plan_prism_with_placement(profile, vector, Some(frame))?.map(PlanarExtrusion::Prism)
                .ok_or_else(|| unsupported("straight guided Pipe requires a planar line face"));
        }

        let unsupported_profile = || unsupported("original straight sweep requires a planar line polygon or an axial circular face");
        let Entity::Face(id) = self.original_query_entity(profile)? else { return Err(unsupported_profile()); };
        let face = self.body.faces.get(*id).ok_or_else(unsupported_profile)?;
        let Some(Surface::Plane { frame }) = self.body.surfaces.get(face.surface) else { return Err(unsupported_profile()); };
        if direction.dot(frame.z).abs() <= 1e-9 {
            return Err(invalid("sweep path direction lies in the profile plane"));
        }
        if let Some((curve, _, _)) = self.conic_face_boundary(*id) {
            if !matches!(curve, Curve3::Circle { .. }) || displacement.cross(frame.z).norm() > Tol::DEFAULT.value() {
                return Err(unsupported_profile());
            }
        }
        self.plan_planar_profile_extrusion(profile, vector)
    }
}
