//! torsion and member-UDL support), the `Tet4`/`Hex8` solid continuum elements, and `ShellFacet3`
//! 🧊️ 3D structural elements: axial `Bar3` truss, Euler-Bernoulli `Frame3` frame member (with
//! (flat facet shell: CST membrane + DKT bending + drilling stabilization).

use semio_s_fem_algebra::vec3d_cross;
use semio_s_fem_model::Dof;
use semio_s_fem_model::Element;
use semio_s_fem_model::ElementContext;
use semio_s_fem_model::ElementResult;
use semio_s_fem_model::MemberUdl;
use semio_s_fem_model::ShellState;
use semio_s_fem_model::SolidStress;

use semio_s_fem_algebra::vec3d_length;

use semio_s_fem_algebra::vec3d_normalize;

use semio_s_fem_algebra::vec3d_sub;

/// 🧲️ Contracts the original signed axial strain with the transverse geometric projector.
fn bar3_geometric_cell(e: f64, area: f64, geometry: Line3Geometry, displacement: &[f64; 6], row: usize, column: usize) -> f64 {
    let extension: f64 = (0..3).map(|axis| geometry.direction[axis] * (displacement[axis + 3] - displacement[axis])).sum();
    let sign = if row / 3 == column / 3 { 1.0 } else { -1.0 };
    let identity = f64::from(u8::from(row % 3 == column % 3));
    e * area * extension / geometry.length.powi(2) * sign * (identity - geometry.direction[row % 3] * geometry.direction[column % 3])
}

use semio_s_fem_algebra::Mat3d;

use semio_s_fem_algebra::MatD;

use semio_s_fem_algebra::VecD;
use semio_s_fem_formulation::b_matrix_plane;
use semio_s_fem_formulation::d_matrix_plane_stress;
use semio_s_fem_formulation::gauss_tri;
use semio_s_fem_formulation::jacobian_2d;
use semio_s_fem_formulation::shape_tri3;
use semio_s_fem_model::BeamStation;








#[path = "🌉️bars-and-frames/🦀️.rs"]
mod bars_and_frames;
pub use bars_and_frames::*;

#[path = "🧱️solids/🦀️.rs"]
mod solids;
pub use solids::*;

#[path = "🐚️shell-facet/🦀️.rs"]
mod shell_facet;
pub use shell_facet::*;
