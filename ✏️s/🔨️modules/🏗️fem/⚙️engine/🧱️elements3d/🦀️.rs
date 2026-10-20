//! 👤️ torsion and member-UDL support), the `Tet4`/`Hex8` solid continuum elements, and `ShellFacet3`
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
