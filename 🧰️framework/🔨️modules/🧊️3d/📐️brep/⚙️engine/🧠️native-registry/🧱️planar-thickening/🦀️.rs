//! 🧱️ Exact signed planar thickening through the retained analytic solid and prism publishers.

use super::*;
use semio_framework_3d_brep_solid_transaction::perforated_extrusion;

fn unsupported() -> ValueError {
    ValueError::literal(ValueRefusalKind::UnsupportedOwner, "original Thicken requires a planar face with one circular or polygonal boundary")
}

/// 🪜️ The exact solid construction selected from the original planar profile.
pub enum PlanarExtrusion {
    Solid(SolidPublication),
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
        let Some(Surface::Plane { frame }) = self.body.surfaces.get(face.surface) else { return Err(unsupported()); };
        let distance = if face.flipped { -thickness } else { thickness };
        let normal = frame.z * distance;
        let extrusion = [normal.x, normal.y, normal.z];
        self.plan_planar_profile_extrusion(handle, extrusion)
    }

    pub(super) fn plan_perforated_profile_extrusion(&mut self,id:FaceId,vector:EVec3)->Result<Option<SolidPublication>,ValueError>{
        if self.body.faces.get(id).is_none_or(|face|face.inners.is_empty()){return Ok(None);}
        let boundary=perforated_extrusion(&self.body,id,vector)?;
        self.plan_solid_boundary(boundary).map(Some)
    }

    pub(super) fn plan_planar_profile_extrusion(&mut self, handle: &str, extrusion: EVec3) -> Result<PlanarExtrusion, ValueError> {
        if let Entity::Face(id)=self.original_query_entity(handle)? {
            if let Some(solid)=self.plan_perforated_profile_extrusion(*id,extrusion)? {
                return Ok(PlanarExtrusion::Solid(solid));
            }
        }
        if let Some(cylinder) = self.plan_circular_extrusion(handle, extrusion)? {
            return Ok(PlanarExtrusion::Solid(cylinder));
        }
        self.plan_prism(handle, extrusion)?.map(PlanarExtrusion::Prism).ok_or_else(unsupported)
    }
}
