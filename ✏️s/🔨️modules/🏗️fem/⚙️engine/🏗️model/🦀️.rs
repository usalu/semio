//! 🧩️ implements, the result records and the error vocabulary. The closed element set, the assembled `Model` and the
//! 🏗️ FEM element contract: degrees of freedom, nodes, supports, loads, the `Element` trait every finite element
//! linear-static solve live in `semio-s-fem-static`; the element libraries in `semio-s-fem-elements2d`/`-elements3d`.

use semio_s_fem_algebra::MatD;

use semio_s_fem_algebra::VecD;
use semio_framework_dispatch_macros::dyn_enum;
use std::fmt;

/// 🪶 Retires one exact string backing from a mounted element's borrowed identity owners.
pub fn close_mounted_strings<'a>(owners: impl IntoIterator<Item = &'a mut String>) -> Option<usize> {
    let owner = owners.into_iter().find(|owner| owner.capacity() != 0)?;
    let bytes = owner.capacity();
    *owner = String::new();
    Some(bytes)
}

/// 🔭 Observes the next mounted identity allocation without changing its owner.
pub fn mounted_string_bytes<'a>(owners: impl IntoIterator<Item = &'a String>) -> Option<usize> {
    owners.into_iter().find(|owner| owner.capacity() != 0).map(String::capacity)
}


// #region 🔖️Dof
/// 🧭️ Nodal degree of freedom kind, shared by 2D (Tx, Ty, Rz) and 3D (all six) models.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Dof {
    Tx,
    Ty,
    Tz,
    Rx,
    Ry,
    Rz,
}

impl Dof {
    pub const ALL: [Dof; 6] = [Dof::Tx, Dof::Ty, Dof::Tz, Dof::Rx, Dof::Ry, Dof::Rz];

    pub fn index(self) -> usize {
        match self {
            Dof::Tx => 0,
            Dof::Ty => 1,
            Dof::Tz => 2,
            Dof::Rx => 3,
            Dof::Ry => 4,
            Dof::Rz => 5,
        }
    }
}
// #endregion 🔖️Dof

// #region 🔖️Model
/// 📍️ A structural node: a stable id and a global position (2D models keep `pos[2] == 0`).
#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    pub id: String,
    pub pos: [f64; 3],
}

/// 🔒️ A support: the subset of a node's active DOFs restrained to zero displacement.
#[derive(Clone, Debug, PartialEq)]
pub struct Support {
    pub node_id: String,
    pub fixed: Vec<Dof>,
}

/// 🏋️ A concentrated load applied directly to one node's global DOF.
#[derive(Clone, Debug, PartialEq)]
pub struct NodalLoad {
    pub node_id: String,
    pub dof: Dof,
    pub value: f64,
}

/// 🌬️ A uniformly distributed member load, components in global directions per unit length.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MemberUdl {
    pub wx: f64,
    pub wy: f64,
    pub wz: f64,
}

/// 📦️ Resolved geometry handed to an element when it builds its stiffness/loads/results —
/// node positions in the same order as `Element::node_ids`.
pub struct ElementContext {
    pub positions: Vec<[f64; 3]>,
}

/// 🔩️ One finite element: contributes a global-coordinate stiffness matrix, optional equivalent
/// nodal loads for a member UDL, and recovers internal forces from the solved displacement vector.
/// Every vector this trait produces or consumes is node-major, DOF-minor ordered, matching
/// `node_ids()` paired with `dofs_per_node()`.
#[dyn_enum]
pub trait Element {
    fn id(&self) -> &str;
    fn node_ids(&self) -> Vec<String>;
    /// 🧵 Borrows one fixed mounted-route node id without allocating a temporary vector.
    fn mounted_node_id(&self, _index: usize) -> Option<&str> {
        None
    }
    /// 📏 Returns the fixed mounted-route node-id count when this element is admitted there.
    fn mounted_node_id_count(&self) -> Option<usize> {
        None
    }
    fn dofs_per_node(&self) -> &[Dof];
    fn stiffness_global(&self, ctx: &ElementContext) -> MatD;
    /// 🧮 Returns one fixed-schema mounted stiffness cell without allocating a matrix owner.
    fn mounted_stiffness_cell(&self, _ctx: &ElementContext, _row: usize, _column: usize) -> Option<f64> {
        None
    }
    /// 🌀️ Borrows the original reference displacements to evaluate one geometric stiffness cell.
    fn mounted_geometric_stiffness_cell(&self, _ctx: &ElementContext, _displacement: &[f64], _row: usize, _column: usize) -> Option<f64> {
        None
    }
    /// 🌬️ Fixed-end nodal loads equivalent to a per-unit-length `MemberUdl` in GLOBAL coordinates.
    /// `None` (the default) means this element doesn't support member UDLs — meaningful for 2-node
    /// line members (`Bar2`/`Bar3`/`BeamEb2`/`Frame3`); continuum/plate/shell elements have no per-unit-
    /// length member concept, so distributed loading on them is the document layer's job (translated
    /// into ordinary nodal loads from an `Area`/pressure load — see `fem_2d`/`fem_3d`'s bridges).
    fn equivalent_nodal_loads(&self, _ctx: &ElementContext, _udl: &MemberUdl) -> Option<VecD> {
        None
    }
    fn recover(&self, ctx: &ElementContext, u_local: &VecD, udl: Option<&MemberUdl>) -> ElementResult;
    /// 🏋️ Consistent element mass matrix in GLOBAL coordinates, same DOF order as `stiffness_global`.
    /// `None` means this element contributes no mass (the default — self-weight/modal analysis skips
    /// it, so massless elements never spuriously restrain a modal shape).
    fn mass(&self, _ctx: &ElementContext) -> Option<MatD> {
        None
    }
    /// 🌀️ Geometric ("stress") stiffness in GLOBAL coordinates for the element's current axial/stress
    /// state at displacement `u_element` — used by linear buckling. `None` means unsupported.
    fn geometric_stiffness(&self, _ctx: &ElementContext, _u_element: &VecD) -> Option<MatD> {
        None
    }
    /// 🧹 Releases one string backing owned by a mounted-analysis element. Implementors
    /// admitted to that bounded route override this method and its terminal witness.
    fn close_mounted_string_step(&mut self) -> Option<usize> {
        None
    }
    /// 📏 Reports the next exact mounted string backing without changing ownership.
    fn mounted_next_string_bytes(&self) -> Option<usize> {
        Some(usize::MAX)
    }
    /// 🧪 Witnesses that a mounted-analysis element has no remaining string backing.
    fn mounted_strings_terminal_is_empty(&self) -> bool {
        false
    }
}

/// 🌱️ A minimal 2-node axial spring — `[Tx]`-only stiffness `k`, geometry-independent (no `ctx`
/// lookup, unlike every real 2D/3D element). Simple enough to hand-calc in closed form, so it anchors
/// `model`'s own solver tests without pulling in a full `Bar2`/`BeamEb2` geometry+material fixture.
pub struct AxialSpring {
    pub id: String,
    pub a: String,
    pub b: String,
    pub k: f64,
}

impl Element for AxialSpring {
    fn id(&self) -> &str {
        &self.id
    }

    fn node_ids(&self) -> Vec<String> {
        vec![self.a.clone(), self.b.clone()]
    }

    fn dofs_per_node(&self) -> &[Dof] {
        &[Dof::Tx]
    }

    fn stiffness_global(&self, _ctx: &ElementContext) -> MatD {
        let mut m = MatD::zeros(2, 2);
        m.set(0, 0, self.k);
        m.set(0, 1, -self.k);
        m.set(1, 0, -self.k);
        m.set(1, 1, self.k);
        m
    }

    fn recover(&self, _ctx: &ElementContext, u_local: &VecD, _udl: Option<&MemberUdl>) -> ElementResult {
        ElementResult::Bar { n: self.k * (u_local.get(1) - u_local.get(0)) }
    }
}


// #region 🔖️Results
/// 📐️ Per-node displacement, indexed by `Dof::index()`; inactive DOFs stay `0.0`.
#[derive(Clone, Debug, PartialEq)]
pub struct NodeDisplacement {
    pub node_id: String,
    pub values: [f64; 6],
}

/// ⚖️ A support reaction at one restrained, active DOF.
#[derive(Clone, Debug, PartialEq)]
pub struct NodeReaction {
    pub node_id: String,
    pub dof: Dof,
    pub value: f64,
}

/// 📊️ A station along a beam's length: internal axial/shear/moment at `x` from the start node.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BeamStation {
    pub x: f64,
    pub n: f64,
    pub v: f64,
    pub m: f64,
}

/// 🧮️ In-plane stress state at one Gauss point of a plane-stress/plane-strain continuum element.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlaneStress {
    pub sxx: f64,
    pub syy: f64,
    pub sxy: f64,
    pub von_mises: f64,
}

/// 🧊️ Full 3D stress state at one Gauss point of a solid element.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SolidStress {
    pub sxx: f64,
    pub syy: f64,
    pub szz: f64,
    pub sxy: f64,
    pub syz: f64,
    pub sxz: f64,
    pub von_mises: f64,
}

/// 🧊️ Bending moments per unit width at one Gauss point of a plate element.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlateMoments {
    pub mx: f64,
    pub my: f64,
    pub mxy: f64,
}

/// 🐚️ Membrane forces + bending moments per unit width at one Gauss point of a facet shell element.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ShellState {
    pub nxx: f64,
    pub nyy: f64,
    pub nxy: f64,
    pub mxx: f64,
    pub myy: f64,
    pub mxy: f64,
    pub von_mises_top: f64,
    pub von_mises_bottom: f64,
}

/// 📤️ Element-kind-specific internal-force recovery. Only Gauss-point values live here — nodal
/// extrapolation/averaging across elements (for contour rendering) is a mesh-wide post-processing
/// step owned by `analyses`, not by any single element.
#[derive(Clone, Debug, PartialEq)]
pub enum ElementResult {
    Bar { n: f64 },
    Beam { stations: Vec<BeamStation> },
    Plane { gauss: Vec<PlaneStress> },
    Plate { gauss: Vec<PlateMoments> },
    Solid { gauss: Vec<SolidStress> },
    Shell { gauss: Vec<ShellState> },
}

/// ✅️ Global sanity checks on the solved system.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SolutionChecks {
    pub residual_norm: f64,
    pub reaction_sum: [f64; 6],
}

/// 📈️ The full result of a linear-static solve.
#[derive(Clone, Debug, PartialEq)]
pub struct StaticResult {
    pub displacements: Vec<NodeDisplacement>,
    pub reactions: Vec<NodeReaction>,
    pub elements: Vec<(String, ElementResult)>,
    pub checks: SolutionChecks,
}
// #endregion 🔖️Results

//#region ⚠️ Errors
/// ⚠️ Everything that can go wrong building or solving a [`Model`].
#[derive(Clone, Debug, PartialEq)]
pub enum FemError {
    EmptyModel,
    DuplicateNodeId(String),
    DanglingNodeRef(String),
    ResourceAdmission,
    Singular,
}

impl fmt::Display for FemError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyModel => formatter.write_str("model has no nodes"),
            Self::DuplicateNodeId(id) => write!(formatter, "duplicate node id: {id}"),
            Self::DanglingNodeRef(id) => write!(formatter, "reference to unknown node id: {id}"),
            Self::ResourceAdmission => formatter.write_str("numerical owner resource admission was refused"),
            Self::Singular => formatter.write_str("stiffness matrix is singular — model is a mechanism or under-constrained"),
        }
    }
}

impl std::error::Error for FemError {}
//#endregion ⚠️ Errors
