//! 🧊️ 3D structural elements: axial `Bar3` truss, Euler-Bernoulli `Frame3` frame member (with
//! torsion and member-UDL support), the `Tet4`/`Hex8` solid continuum elements, and `ShellFacet3`
//! (flat facet shell: CST membrane + DKT bending + drilling stabilization).

use semio_s_fem_algebra::{vec3d_cross, vec3d_length, vec3d_normalize, vec3d_sub, Mat3d, MatD, VecD};
use semio_s_fem_formulation::{b_matrix_plane, d_matrix_plane_stress, gauss_tri, jacobian_2d, shape_tri3};
use semio_s_fem_model::{BeamStation, Dof, Element, ElementContext, ElementResult, MemberUdl, ShellState, SolidStress};

#[path = "🌉️bars-and-frames/🦀️.rs"]
mod bars_and_frames;
pub use bars_and_frames::*;

#[path = "🧱️solids/🦀️.rs"]
mod solids;
pub use solids::*;

#[path = "🐚️shell-facet/🦀️.rs"]
mod shell_facet;
pub use shell_facet::*;
