//! 🔲️ Tri3/Tri6/Quad4/Quad8 plane-stress/plane-strain continuum family, and the `PlateDkt` Batoz
//! 📐️ 2D structural elements: axial `Bar2` truss, Euler-Bernoulli `BeamEb2` frame member, the
//! Discrete Kirchhoff Triangle thin-plate bending element.

use semio_s_fem_algebra::MatD;
use semio_s_fem_model::Element;
use semio_s_fem_model::ElementContext;
use semio_s_fem_model::ElementResult;
use semio_s_fem_model::MemberUdl;
use semio_s_fem_model::PlaneStress;
use semio_s_fem_model::PlateMoments;

use semio_s_fem_algebra::VecD;
use semio_s_fem_formulation::b_matrix_plane;
use semio_s_fem_formulation::d_matrix_plane_strain;
use semio_s_fem_formulation::d_matrix_plane_stress;
use semio_s_fem_formulation::gauss_quad;
use semio_s_fem_formulation::gauss_tri;
use semio_s_fem_formulation::jacobian_2d;
use semio_s_fem_formulation::shape_quad4;
use semio_s_fem_formulation::shape_quad8;
use semio_s_fem_formulation::shape_tri3;
use semio_s_fem_formulation::shape_tri6;
use semio_s_fem_model::Dof;





fn mounted_segment_geometry(positions: &[[f64; 3]]) -> Option<(f64, f64, f64)> {
    if positions.len() != 2 { return None; }
    let dx = positions[1][0] - positions[0][0];
    let dy = positions[1][1] - positions[0][1];
    let length = dx.hypot(dy);
    (length.is_finite() && length > 0.0).then_some((length, dx / length, dy / length))
}



#[path = "🪢️bars-and-beams/🦀️.rs"]
mod bars_and_beams;
pub use bars_and_beams::*;


impl Bar2 {
    /// 🌬️ Borrows one consistent member-load and gravity row without scratch allocations.
    pub fn mounted_load_row(&self, positions: &[[f64; 3]], row: usize, udl: &MemberUdl, gravity: [f64; 2]) -> Option<f64> {
        if row >= 4 { return None; }
        let (length, _, _) = mounted_segment_geometry(positions)?;
        Some(([udl.wx, udl.wy][row % 2] + self.density * self.area * gravity[row % 2]) * length / 2.0)
    }
}

#[path = "🔲️plane-continuum/🦀️.rs"]
mod plane_continuum;
pub use plane_continuum::*;

#[path = "🍽️plate-dkt/🦀️.rs"]
mod plate_dkt;
pub use plate_dkt::*;
