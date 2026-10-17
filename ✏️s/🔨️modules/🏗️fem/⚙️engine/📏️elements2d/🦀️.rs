//! 📐️ 2D structural elements: axial `Bar2` truss, Euler-Bernoulli `BeamEb2` frame member, the
//! Tri3/Tri6/Quad4/Quad8 plane-stress/plane-strain continuum family, and the `PlateDkt` Batoz
//! Discrete Kirchhoff Triangle thin-plate bending element.

use semio_s_fem_algebra::{MatD, VecD};
use semio_s_fem_formulation::{b_matrix_plane, d_matrix_plane_strain, d_matrix_plane_stress, gauss_quad, gauss_tri, jacobian_2d, shape_quad4, shape_quad8, shape_tri3, shape_tri6};
use semio_s_fem_model::{Dof, Element, ElementContext, ElementResult, MemberUdl, PlaneStress, PlateMoments};

#[path = "🪢️bars-and-beams/🦀️.rs"]
mod bars_and_beams;
pub use bars_and_beams::*;

#[path = "🔲️plane-continuum/🦀️.rs"]
mod plane_continuum;
pub use plane_continuum::*;

#[path = "🍽️plate-dkt/🦀️.rs"]
mod plate_dkt;
pub use plate_dkt::*;
