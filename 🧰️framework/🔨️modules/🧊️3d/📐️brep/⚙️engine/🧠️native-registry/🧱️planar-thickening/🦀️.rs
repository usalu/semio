//! 🧱️ Exact signed planar thickening through the retained cylinder and prism publishers.

use super::*;

fn unsupported() -> ValueError {
    ValueError::literal(ValueRefusalKind::UnsupportedOwner, "original Thicken requires a planar face with one circular or polygonal boundary")
}

/// 🪜️ The exact solid construction selected from the original planar profile.
pub enum PlanarExtrusion {
    Cylinder(SolidPublication),
    Prism(PrismDraft),
}

impl BrepRegistry {
    /// 📐️ Plans a signed extrusion along the original face's outward normal.
    pub fn plan_planar_thickening(&mut self, handle: &str, thickness: f64) -> Result<PlanarExtrusion, ValueError> {
        if !thickness.is_finite() || thickness.abs() <= Tol::DEFAULT.value() {
            return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "thickness must be finite and nonzero at the kernel tolerance"));
        }
        let Entity::Face(id) = self.original_query_entity(handle)? else { return Err(unsupported()); };
        let face = self.body.faces.get(*id).ok_or_else(unsupported)?;
        if !face.inners.is_empty() { return Err(unsupported()); }
        let Some(Surface::Plane { frame }) = self.body.surfaces.get(face.surface) else { return Err(unsupported()); };
        let distance = if face.flipped { -thickness } else { thickness };
        let normal = frame.z * distance;
        let extrusion = [normal.x, normal.y, normal.z];
        self.plan_planar_profile_extrusion(handle, extrusion)
    }

    pub(super) fn plan_planar_profile_extrusion(&mut self, handle: &str, extrusion: EVec3) -> Result<PlanarExtrusion, ValueError> {
        if let Some(cylinder) = self.plan_circular_extrusion(handle, extrusion)? {
            return Ok(PlanarExtrusion::Cylinder(cylinder));
        }
        self.plan_prism(handle, extrusion)?.map(PlanarExtrusion::Prism).ok_or_else(unsupported)
    }
}
