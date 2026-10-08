//! 🛞️ Mass properties of any face-bearing shape: area and surface centroid always, and volume,
//! centroid, inertia tensor about the centroid with principal moments and axes whenever the shape
//! encloses volume (a solid, a compound of solids, or a closed shell).

use super::{manifold_report, ShapeKind, ShapeScope, Watertightness};
use crate::brep::queries::mass_properties::{area_centroid_from_moments, area_from_moments, faces_moments, mass_from_moments, solid_mass_with_area_centroid, MassProperties};
use crate::brep::representation::error::KernelError;
use crate::brep::representation::topology::Body;
use crate::inertia::{combine, principal_inertia, InertiaPiece, PrincipalInertia};

/// 🛞️ Volume, centroid and inertia about the centroid at unit density, with the principal frame.
#[derive(Clone, Copy, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::protocol::value")]
pub struct VolumetricMass {
    pub volume: f64,
    pub centroid: [f64; 3],
    pub inertia: [[f64; 3]; 3],
    pub principal: PrincipalInertia,
    pub error_estimate: f64,
}

/// ⚖️ What a shape weighs: its boundary area, and its enclosed mass when it encloses volume.
#[derive(Clone, Copy, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::protocol::value")]
pub struct ShapeMassProperties {
    pub area: f64,
    pub area_centroid: Option<[f64; 3]>,
    pub volumetric: Option<VolumetricMass>,
}

fn volumetric(mass: &MassProperties) -> VolumetricMass {
    let centroid = [mass.centroid.x, mass.centroid.y, mass.centroid.z];
    VolumetricMass { volume: mass.volume, centroid, inertia: mass.inertia, principal: principal_inertia(mass.inertia), error_estimate: mass.error_estimate }
}

/// 🏋️ Mass properties of `scope` integrated over its trimmed faces with relative tolerance `tolerance`.
///
/// Vertices, edges and wires have no area and are refused. A compound sums its solids (overlaps are not
/// merged); an open shell reports area only.
pub fn mass_properties(body: &Body, scope: &ShapeScope, tolerance: f64) -> Result<ShapeMassProperties, KernelError> {
    let members = scope.members(body)?;
    match scope.kind {
        ShapeKind::Vertex | ShapeKind::Edge | ShapeKind::Wire => Err(KernelError::InvalidInput("mass properties need a face, shell, solid or compound".into())),
        ShapeKind::Solid | ShapeKind::Compound => {
            let mut pieces = Vec::new();
            let (mut area, mut area_moment) = (0.0, [0.0; 3]);
            let mut error = 0.0;
            for &solid in &members.solids {
                let (mass, centroid) = solid_mass_with_area_centroid(body, solid, tolerance)?;
                area += mass.area;
                area_moment = [area_moment[0] + mass.area * centroid.x, area_moment[1] + mass.area * centroid.y, area_moment[2] + mass.area * centroid.z];
                error += mass.error_estimate * mass.volume;
                pieces.push(InertiaPiece { mass: mass.volume, centroid: [mass.centroid.x, mass.centroid.y, mass.centroid.z], about_centroid: mass.inertia });
            }
            let whole = combine(&pieces).ok_or_else(|| KernelError::InvalidInput("shape has no volume".into()))?;
            let area_centroid = (area > 0.0).then(|| [area_moment[0] / area, area_moment[1] / area, area_moment[2] / area]);
            let merged = VolumetricMass { volume: whole.mass, centroid: whole.centroid, inertia: whole.about_centroid, principal: principal_inertia(whole.about_centroid), error_estimate: error / whole.mass };
            Ok(ShapeMassProperties { area, area_centroid, volumetric: Some(merged) })
        }
        ShapeKind::Shell | ShapeKind::Face => {
            let (totals, error) = faces_moments(body, &members.faces, tolerance)?;
            let area_centroid = area_centroid_from_moments(&totals).map(|c| [c.x, c.y, c.z]);
            let enclosed = scope.kind == ShapeKind::Shell && manifold_report(body, scope)?.verdict == Watertightness::Watertight;
            let volumetric = if enclosed { mass_from_moments(&totals, error).ok().as_ref().map(volumetric) } else { None };
            Ok(ShapeMassProperties { area: area_from_moments(&totals), area_centroid, volumetric })
        }
    }
}
