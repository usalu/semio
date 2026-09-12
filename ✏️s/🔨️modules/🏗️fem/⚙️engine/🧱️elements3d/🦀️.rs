//! 🧊️ 3D structural elements: axial `Bar3` truss, Euler-Bernoulli `Frame3` frame member (with
//! torsion and member-UDL support), the `Tet4`/`Hex8` solid continuum elements, and `ShellFacet3`
//! (flat facet shell: CST membrane + DKT bending + drilling stabilization).

use crate::algebra::{vec3d_cross, vec3d_length, vec3d_normalize, vec3d_sub, Mat3d, MatD, VecD};
use crate::formulation::{b_matrix_plane, d_matrix_plane_stress, gauss_tri, jacobian_2d, shape_tri3};
#[cfg(test)]
use crate::model::Elements;
use crate::model::{BeamStation, Dof, Element, ElementContext, ElementResult, MemberUdl, ShellState, SolidStress};

#[derive(Clone, Copy)]
struct Line3Geometry {
    length: f64,
    direction: [f64; 3],
}

/// 📐️ Resolves one finite nondegenerate 3D line without allocating an element matrix.
fn line3_geometry(ctx: &ElementContext) -> Option<Line3Geometry> {
    if ctx.positions.len() != 2 {
        return None;
    }
    let delta = vec3d_sub(ctx.positions[1], ctx.positions[0]);
    let length = vec3d_length(delta);
    if !length.is_finite() || length <= f64::EPSILON {
        return None;
    }
    let direction = [delta[0] / length, delta[1] / length, delta[2] / length];
    direction.iter().all(|component| component.is_finite()).then_some(Line3Geometry { length, direction })
}

/// 🧮️ Evaluates one Bar3 stiffness cell from its fixed line geometry.
fn bar3_stiffness_cell(e: f64, area: f64, geometry: Line3Geometry, row: usize, column: usize) -> f64 {
    let sign = if row / 3 == column / 3 { 1.0 } else { -1.0 };
    e * area / geometry.length * sign * geometry.direction[row % 3] * geometry.direction[column % 3]
}

// #region 🔖️Bar3
/// 🪵️ Two-node 3D axial truss element — carries only translational DOFs, stiffness `k = EA/L`
/// projected onto the member's unit direction.
pub struct Bar3 {
    pub id: String,
    pub node_a: String,
    pub node_b: String,
    pub e: f64,
    pub a: f64,
    pub density: f64,
}

impl Element for Bar3 {
    fn id(&self) -> &str {
        &self.id
    }

    fn node_ids(&self) -> Vec<String> {
        vec![self.node_a.clone(), self.node_b.clone()]
    }

    fn mounted_node_id(&self, index: usize) -> Option<&str> {
        [self.node_a.as_str(), self.node_b.as_str()].get(index).copied()
    }

    fn mounted_node_id_count(&self) -> Option<usize> {
        Some(2)
    }

    fn dofs_per_node(&self) -> &[Dof] {
        const DOFS: [Dof; 3] = [Dof::Tx, Dof::Ty, Dof::Tz];
        &DOFS
    }

    fn stiffness_global(&self, ctx: &ElementContext) -> MatD {
        let geometry = line3_geometry(ctx).expect("non-degenerate bar3");
        let mut ke = MatD::zeros(6, 6);
        for row in 0..6 {
            for column in 0..6 {
                ke.set(row, column, bar3_stiffness_cell(self.e, self.a, geometry, row, column));
            }
        }
        ke
    }

    fn mounted_stiffness_cell(&self, ctx: &ElementContext, row: usize, column: usize) -> Option<f64> {
        if row >= 6 || column >= 6 {
            return None;
        }
        Some(bar3_stiffness_cell(self.e, self.a, line3_geometry(ctx)?, row, column))
    }

    fn recover(&self, ctx: &ElementContext, u_elem: &VecD, _udl: Option<&MemberUdl>) -> ElementResult {
        let d = vec3d_sub(ctx.positions[1], ctx.positions[0]);
        let l = vec3d_length(d);
        let c = vec3d_normalize(d);
        let k = self.e * self.a / l;
        let du = [u_elem.get(3) - u_elem.get(0), u_elem.get(4) - u_elem.get(1), u_elem.get(5) - u_elem.get(2)];
        let n = k * (c[0] * du[0] + c[1] * du[1] + c[2] * du[2]);
        ElementResult::Bar { n }
    }

    /// 🏋️ Isotropic mass — same pattern as `Bar2` but with 3x3 identity blocks, no preferred direction.
    /// `m = ρAL/6`, block `(node_i, node_j) = (2m if i==j else m) * I3`.
    fn mass(&self, ctx: &ElementContext) -> Option<MatD> {
        let d = vec3d_sub(ctx.positions[1], ctx.positions[0]);
        let l = vec3d_length(d);
        let m = self.density * self.a * l / 6.0;
        let mut out = MatD::zeros(6, 6);
        for i in 0..3 {
            out.set(i, i, 2.0 * m);
            out.set(i + 3, i + 3, 2.0 * m);
            out.set(i, i + 3, m);
            out.set(i + 3, i, m);
        }
        Some(out)
    }

    /// 🌬️ Consistent end-load `wL/2` at each node from a global member UDL `(wx,wy,wz)` — same
    /// exact-split reasoning as `elements2d::Bar2::equivalent_nodal_loads`.
    fn equivalent_nodal_loads(&self, ctx: &ElementContext, udl: &MemberUdl) -> Option<VecD> {
        let d = vec3d_sub(ctx.positions[1], ctx.positions[0]);
        let l = vec3d_length(d);
        let half = l / 2.0;
        Some(VecD::from_vec(vec![udl.wx * half, udl.wy * half, udl.wz * half, udl.wx * half, udl.wy * half, udl.wz * half]))
    }

    /// 🌀️ 3D truss geometric stiffness under axial force `n` (tension-positive, `recover`'s convention):
    /// `N/L·(I₃ − ccᵀ)` per 3x3 node block — the 3D analogue of `elements2d::Bar2::geometric_stiffness`.
    fn geometric_stiffness(&self, ctx: &ElementContext, u_elem: &VecD) -> Option<MatD> {
        let d = vec3d_sub(ctx.positions[1], ctx.positions[0]);
        let l = vec3d_length(d);
        let c = vec3d_normalize(d);
        let k = self.e * self.a / l;
        let du = [u_elem.get(3) - u_elem.get(0), u_elem.get(4) - u_elem.get(1), u_elem.get(5) - u_elem.get(2)];
        let n = k * (c[0] * du[0] + c[1] * du[1] + c[2] * du[2]);
        let coeff = n / l;
        let mut kg = MatD::zeros(6, 6);
        for i in 0..3 {
            for j in 0..3 {
                let identity = if i == j { 1.0 } else { 0.0 };
                let v = coeff * (identity - c[i] * c[j]);
                kg.set(i, j, v);
                kg.set(i, j + 3, -v);
                kg.set(i + 3, j, -v);
                kg.set(i + 3, j + 3, v);
            }
        }
        Some(kg)
    }
}
// #endregion 🔖️Bar3

// #region 🔖️Frame3
/// 🏗️ Two-node 3D Euler-Bernoulli frame element with torsion — full 6-DOF-per-node member. Local x
/// runs node-a to node-b; local y/z are built from a reference "up" vector and rotated by `roll`
/// (radians) about local x. `stiffness_global`/`recover` rotate the decoupled axial/torsion/biaxial
/// bending local stiffness into global coordinates via the block-diagonal transform `T`.
pub struct Frame3 {
    pub id: String,
    pub node_a: String,
    pub node_b: String,
    pub e: f64,
    pub g: f64,
    pub a: f64,
    pub iy: f64,
    pub iz: f64,
    pub j: f64,
    pub roll: f64,
    pub density: f64,
}

/// ↕️ `θy = −∂w/∂x` sign flip of the odd (`L`-carrying) rows/columns in the y-bending block.
const Y_PLANE_SIGN: [f64; 4] = [1.0, -1.0, 1.0, -1.0];

#[derive(Clone, Copy)]
struct Frame3Geometry {
    length: f64,
    axes: [[f64; 3]; 3],
}

/// 🧭️ Resolves Frame3's rolled orthonormal local basis as fixed-size scalar state.
fn frame3_geometry(ctx: &ElementContext, roll: f64) -> Option<Frame3Geometry> {
    if !roll.is_finite() {
        return None;
    }
    let line = line3_geometry(ctx)?;
    let reference = if line.direction[2].abs() > 0.99 { [1.0, 0.0, 0.0] } else { [0.0, 0.0, 1.0] };
    let y_cross = vec3d_cross(reference, line.direction);
    let y_length = vec3d_length(y_cross);
    if !y_length.is_finite() || y_length <= f64::EPSILON {
        return None;
    }
    let y_unrotated = [y_cross[0] / y_length, y_cross[1] / y_length, y_cross[2] / y_length];
    let z_unrotated = vec3d_cross(line.direction, y_unrotated);
    let (sin_roll, cos_roll) = roll.sin_cos();
    let local_y = [y_unrotated[0] * cos_roll + z_unrotated[0] * sin_roll, y_unrotated[1] * cos_roll + z_unrotated[1] * sin_roll, y_unrotated[2] * cos_roll + z_unrotated[2] * sin_roll];
    let local_z = [z_unrotated[0] * cos_roll - y_unrotated[0] * sin_roll, z_unrotated[1] * cos_roll - y_unrotated[1] * sin_roll, z_unrotated[2] * cos_roll - y_unrotated[2] * sin_roll];
    local_y.iter().chain(local_z.iter()).all(|component| component.is_finite()).then_some(Frame3Geometry { length: line.length, axes: [line.direction, local_y, local_z] })
}

/// 🏛️ Evaluates one Euler-Bernoulli bending-block cell before optional plane sign changes.
fn frame3_bending_cell(length: f64, rigidity: f64, row: usize, column: usize) -> f64 {
    let base = rigidity / length;
    let length_squared = length * length;
    [
        [12.0 * base / length_squared, 6.0 * base / length, -12.0 * base / length_squared, 6.0 * base / length],
        [6.0 * base / length, 4.0 * base, -6.0 * base / length, 2.0 * base],
        [-12.0 * base / length_squared, -6.0 * base / length, 12.0 * base / length_squared, -6.0 * base / length],
        [6.0 * base / length, 2.0 * base, -6.0 * base / length, 4.0 * base],
    ][row][column]
}

impl Frame3 {
    /// 🧭️ Builds the member length, local 12x12 stiffness, and the 12x12 global<->local block-diagonal
    /// rotation `T` (four `R^T` 3x3 blocks) shared by `stiffness_global` and `recover`.
    fn local_system(&self, ctx: &ElementContext) -> (f64, MatD, MatD) {
        let geometry = frame3_geometry(ctx, self.roll).expect("non-degenerate frame3");
        let mut t = MatD::zeros(12, 12);
        for offset in [0usize, 3, 6, 9] {
            for row in 0..3 {
                for col in 0..3 {
                    t.set(offset + row, offset + col, geometry.axes[row][col]);
                }
            }
        }
        (geometry.length, self.local_stiffness(geometry.length), t)
    }

    /// 🧮️ Decoupled local 12x12 stiffness: axial, torsion, and biaxial (y/z) Euler-Bernoulli bending.
    fn local_stiffness(&self, l: f64) -> MatD {
        let mut k = MatD::zeros(12, 12);
        for row in 0..12 {
            for column in 0..12 {
                k.set(row, column, self.local_stiffness_cell(l, row, column));
            }
        }
        k
    }

    /// 🔩️ Evaluates one local axial, torsion, or bending cell without allocating a matrix.
    fn local_stiffness_cell(&self, length: f64, row: usize, column: usize) -> f64 {
        if [0usize, 6].contains(&row) && [0usize, 6].contains(&column) {
            return self.e * self.a / length * if row == column { 1.0 } else { -1.0 };
        }
        if [3usize, 9].contains(&row) && [3usize, 9].contains(&column) {
            return self.g * self.j / length * if row == column { 1.0 } else { -1.0 };
        }
        const Z_PLANE: [usize; 4] = [1, 5, 7, 11];
        if let (Some(local_row), Some(local_column)) = (Z_PLANE.iter().position(|index| *index == row), Z_PLANE.iter().position(|index| *index == column)) {
            return frame3_bending_cell(length, self.e * self.iz, local_row, local_column);
        }
        const Y_PLANE: [usize; 4] = [2, 4, 8, 10];
        if let (Some(local_row), Some(local_column)) = (Y_PLANE.iter().position(|index| *index == row), Y_PLANE.iter().position(|index| *index == column)) {
            return Y_PLANE_SIGN[local_row] * Y_PLANE_SIGN[local_column] * frame3_bending_cell(length, self.e * self.iy, local_row, local_column);
        }
        0.0
    }

    /// 🌐️ Evaluates one transformed global cell from fixed local geometry.
    fn global_stiffness_cell(&self, geometry: Frame3Geometry, row: usize, column: usize) -> f64 {
        let mut value = 0.0;
        for local_row_component in 0..3 {
            let local_row = row / 3 * 3 + local_row_component;
            let left = geometry.axes[local_row_component][row % 3];
            for local_column_component in 0..3 {
                let local_column = column / 3 * 3 + local_column_component;
                value += left * self.local_stiffness_cell(geometry.length, local_row, local_column) * geometry.axes[local_column_component][column % 3];
            }
        }
        value
    }

    /// 🏋️ Local 12x12 consistent mass: axial `ρAL/6*[[2,1],[1,2]]` at `(0,6)`, torsion `ρJL/6*[[2,1],[1,2]]`
    /// at `(3,9)` — a simplified polar-inertia proxy (not rigorously exact rotary inertia, but the
    /// accepted simplification at this scope — see `mass`'s doc), and both bending planes ([1,5,7,11]
    /// z-plane, [2,4,8,10] y-plane) using the same 156/22L/54/-13L consistent-beam-mass pattern. The
    /// y-plane carries `θy = −∂w/∂x`, exactly as `local_stiffness` and `local_udl` do, so its block is
    /// `S·M·S` with `S = diag(1, −1, 1, −1)` (`Y_PLANE_SIGN`) — without it self-weight fixed-end
    /// moments and the y-bending modal family come out with the wrong sign (ticket
    /// 26/09/06/FEM-PLUGIN-END-TO-END, W6 PyNite/scipy oracle: −66.7 % on a one-element cantilever).
    fn local_mass(&self, l: f64) -> MatD {
        let mut m = MatD::zeros(12, 12);
        let axial = self.density * self.a * l / 6.0;
        m.set(0, 0, 2.0 * axial);
        m.set(0, 6, axial);
        m.set(6, 0, axial);
        m.set(6, 6, 2.0 * axial);

        let torsion = self.density * self.j * l / 6.0;
        m.set(3, 3, 2.0 * torsion);
        m.set(3, 9, torsion);
        m.set(9, 3, torsion);
        m.set(9, 9, 2.0 * torsion);

        let l2 = l * l;
        let factor = self.density * self.a * l / 420.0;
        let block = [[156.0, 22.0 * l, 54.0, -13.0 * l], [22.0 * l, 4.0 * l2, 13.0 * l, -3.0 * l2], [54.0, 13.0 * l, 156.0, -22.0 * l], [-13.0 * l, -3.0 * l2, -22.0 * l, 4.0 * l2]];
        for (bi, &gi) in [1usize, 5, 7, 11].iter().enumerate() {
            for (bj, &gj) in [1usize, 5, 7, 11].iter().enumerate() {
                m.set(gi, gj, factor * block[bi][bj]);
            }
        }
        for (bi, &gi) in [2usize, 4, 8, 10].iter().enumerate() {
            for (bj, &gj) in [2usize, 4, 8, 10].iter().enumerate() {
                m.set(gi, gj, Y_PLANE_SIGN[bi] * Y_PLANE_SIGN[bj] * factor * block[bi][bj]);
            }
        }
        m
    }

    /// 🌀️ Local 12x12 geometric stiffness under axial force `n` (tension-positive, matches `recover`'s
    /// convention), applied independently to both bending planes via the same `Kg_bend` beam-column
    /// formula `beam_local_geometric_stiffness` in `elements2d` uses.
    fn local_geometric_stiffness(&self, l: f64, n: f64) -> MatD {
        let mut kg = MatD::zeros(12, 12);
        let l2 = l * l;
        let coeff = n / l;
        let block = [[6.0 / 5.0, l / 10.0, -6.0 / 5.0, l / 10.0], [l / 10.0, 2.0 * l2 / 15.0, -l / 10.0, -l2 / 30.0], [-6.0 / 5.0, -l / 10.0, 6.0 / 5.0, -l / 10.0], [l / 10.0, -l2 / 30.0, -l / 10.0, 2.0 * l2 / 15.0]];
        for (bi, &gi) in [1usize, 5, 7, 11].iter().enumerate() {
            for (bj, &gj) in [1usize, 5, 7, 11].iter().enumerate() {
                kg.set(gi, gj, coeff * block[bi][bj]);
            }
        }
        for (bi, &gi) in [2usize, 4, 8, 10].iter().enumerate() {
            for (bj, &gj) in [2usize, 4, 8, 10].iter().enumerate() {
                kg.set(gi, gj, Y_PLANE_SIGN[bi] * Y_PLANE_SIGN[bj] * coeff * block[bi][bj]);
            }
        }
        kg
    }
}

/// 🌬️ Rotates a GLOBAL member UDL `(wx,wy,wz)` into LOCAL `(wx_l,wy_l,wz_l)` via the same 12x12
/// global->local block-diagonal rotation `T` used for stiffness/displacement (`local_system`'s `t`).
fn local_udl_components(t: &MatD, udl: &MemberUdl) -> (f64, f64, f64) {
    let global_w = VecD::from_vec(vec![udl.wx, udl.wy, udl.wz, 0.0, 0.0, 0.0, udl.wx, udl.wy, udl.wz, 0.0, 0.0, 0.0]);
    let local_w = t.mul_vec(&global_w);
    (local_w.get(0), local_w.get(1), local_w.get(2))
}

/// 🌬️ Local 12-vector fixed-end load for a member UDL, dof order `[u,v,w,θx,θy,θz]` per node.
/// The z-bending plane (`v`,`θz`, indices 1/5/7/11, using `iz`) uses the standard beam fixed-end-load
/// formula (identical in form to `elements2d`'s `beam_local_udl`). The y-bending plane (`w`,`θy`,
/// indices 2/4/8/10, using `iy`) carries the same off-diagonal sign flip `local_stiffness`'s `by`
/// block has relative to its `bz` block — hand-verified against a cantilever-under-UDL benchmark
/// (base moment `wL²/2`, base shear `wL`, zero at the tip) in `solid_tests`.
fn local_udl(l: f64, t: &MatD, udl: &MemberUdl) -> VecD {
    let (wx_l, wy_l, wz_l) = local_udl_components(t, udl);
    let l2 = l * l;
    let mut f = VecD::zeros(12);
    f.set(0, wx_l * l / 2.0);
    f.set(6, wx_l * l / 2.0);
    f.set(1, wy_l * l / 2.0);
    f.set(5, wy_l * l2 / 12.0);
    f.set(7, wy_l * l / 2.0);
    f.set(11, -wy_l * l2 / 12.0);
    f.set(2, wz_l * l / 2.0);
    f.set(4, -wz_l * l2 / 12.0);
    f.set(8, wz_l * l / 2.0);
    f.set(10, wz_l * l2 / 12.0);
    f
}

impl Element for Frame3 {
    fn id(&self) -> &str {
        &self.id
    }

    fn node_ids(&self) -> Vec<String> {
        vec![self.node_a.clone(), self.node_b.clone()]
    }

    fn mounted_node_id(&self, index: usize) -> Option<&str> {
        [self.node_a.as_str(), self.node_b.as_str()].get(index).copied()
    }

    fn mounted_node_id_count(&self) -> Option<usize> {
        Some(2)
    }

    fn dofs_per_node(&self) -> &[Dof] {
        const DOFS: [Dof; 6] = [Dof::Tx, Dof::Ty, Dof::Tz, Dof::Rx, Dof::Ry, Dof::Rz];
        &DOFS
    }

    fn stiffness_global(&self, ctx: &ElementContext) -> MatD {
        let geometry = frame3_geometry(ctx, self.roll).expect("non-degenerate frame3");
        let mut stiffness = MatD::zeros(12, 12);
        for row in 0..12 {
            for column in 0..12 {
                stiffness.set(row, column, self.global_stiffness_cell(geometry, row, column));
            }
        }
        stiffness
    }

    fn mounted_stiffness_cell(&self, ctx: &ElementContext, row: usize, column: usize) -> Option<f64> {
        if row >= 12 || column >= 12 {
            return None;
        }
        Some(self.global_stiffness_cell(frame3_geometry(ctx, self.roll)?, row, column))
    }

    fn equivalent_nodal_loads(&self, ctx: &ElementContext, udl: &MemberUdl) -> Option<VecD> {
        let (l, _k_local, t) = self.local_system(ctx);
        let f_local = local_udl(l, &t, udl);
        Some(t.transpose().mul_vec(&f_local))
    }

    fn recover(&self, ctx: &ElementContext, u_elem: &VecD, udl: Option<&MemberUdl>) -> ElementResult {
        let (l, k_local, t) = self.local_system(ctx);
        let u_loc = t.mul_vec(u_elem);
        let f_udl_local = udl.map_or_else(|| VecD::zeros(12), |u| local_udl(l, &t, u));
        let f = k_local.mul_vec(&u_loc).sub(&f_udl_local);
        let n = -f.get(0);
        let v1 = f.get(2);
        let m1 = f.get(4);
        let wz_l = udl.map_or(0.0, |u| local_udl_components(&t, u).2);
        let stations = (0..11)
            .map(|i| {
                let x = l * (i as f64) / 10.0;
                BeamStation { x, n, v: v1 + wz_l * x, m: m1 + v1 * x + wz_l * x * x / 2.0 }
            })
            .collect();
        ElementResult::Beam { stations }
    }

    fn mass(&self, ctx: &ElementContext) -> Option<MatD> {
        let (l, _k_local, t) = self.local_system(ctx);
        let m_local = self.local_mass(l);
        Some(t.transpose().matmul(&m_local).matmul(&t))
    }

    /// 🌀️ Buckling geometric stiffness from the member's own axial force under `u_element` — same
    /// sign convention as `recover`'s `n` (tension-positive): `n = -k_local.mul_vec(u_loc).get(0)`.
    fn geometric_stiffness(&self, ctx: &ElementContext, u_element: &VecD) -> Option<MatD> {
        let (l, k_local, t) = self.local_system(ctx);
        let u_loc = t.mul_vec(u_element);
        let f = k_local.mul_vec(&u_loc);
        let n = -f.get(0);
        let kg_local = self.local_geometric_stiffness(l, n);
        Some(t.transpose().matmul(&kg_local).matmul(&t))
    }
}
// #endregion 🔖️Frame3

// #region 🔖️Solid
/// 🧮️ Isotropic 3D solid-mechanics constitutive matrix (6x6), strain order `[εxx,εyy,εzz,γxy,γyz,γxz]`.
fn d_matrix_solid(e: f64, nu: f64) -> MatD {
    let s = e / ((1.0 + nu) * (1.0 - 2.0 * nu));
    let mut d = MatD::zeros(6, 6);
    d.set(0, 0, s * (1.0 - nu));
    d.set(0, 1, s * nu);
    d.set(0, 2, s * nu);
    d.set(1, 0, s * nu);
    d.set(1, 1, s * (1.0 - nu));
    d.set(1, 2, s * nu);
    d.set(2, 0, s * nu);
    d.set(2, 1, s * nu);
    d.set(2, 2, s * (1.0 - nu));
    let g = s * (1.0 - 2.0 * nu) / 2.0;
    d.set(3, 3, g);
    d.set(4, 4, g);
    d.set(5, 5, g);
    d
}

/// 🧮️ Standard solid-mechanics B-matrix (6x3n) from per-node physical shape-function gradients —
/// shared by `Tet4` (constant gradients, one row of blocks) and `Hex8` (per-Gauss-point gradients).
fn solid_b_matrix(grads: &[[f64; 3]]) -> MatD {
    let mut b = MatD::zeros(6, grads.len() * 3);
    for (i, g) in grads.iter().enumerate() {
        let (bx, by, bz) = (g[0], g[1], g[2]);
        let c = i * 3;
        b.set(0, c, bx);
        b.set(1, c + 1, by);
        b.set(2, c + 2, bz);
        b.set(3, c, by);
        b.set(3, c + 1, bx);
        b.set(4, c + 1, bz);
        b.set(4, c + 2, by);
        b.set(5, c, bz);
        b.set(5, c + 2, bx);
    }
    b
}

/// 🧬️ Resolves one displacement column of the 3D strain-displacement matrix.
fn solid_b_column(gradient: [f64; 3], component: usize) -> [f64; 6] {
    match component {
        0 => [gradient[0], 0.0, 0.0, gradient[1], 0.0, gradient[2]],
        1 => [0.0, gradient[1], 0.0, gradient[0], gradient[2], 0.0],
        2 => [0.0, 0.0, gradient[2], 0.0, gradient[1], gradient[0]],
        _ => [0.0; 6],
    }
}

/// 🪨️ Contracts two fixed strain columns through the isotropic solid constitutive tensor.
fn solid_constitutive_bilinear(e: f64, nu: f64, left: [f64; 6], right: [f64; 6]) -> f64 {
    let scale = e / ((1.0 + nu) * (1.0 - 2.0 * nu));
    let mut value = 0.0;
    for row in 0..3 {
        for column in 0..3 {
            value += left[row] * scale * if row == column { 1.0 - nu } else { nu } * right[column];
        }
    }
    let shear = scale * (1.0 - 2.0 * nu) / 2.0;
    for index in 3..6 {
        value += left[index] * shear * right[index];
    }
    value
}

/// 🧮️ Von Mises equivalent stress from the full 3D stress state.
fn von_mises_solid(sxx: f64, syy: f64, szz: f64, sxy: f64, syz: f64, sxz: f64) -> f64 {
    (0.5 * ((sxx - syy).powi(2) + (syy - szz).powi(2) + (szz - sxx).powi(2) + 6.0 * (sxy * sxy + syz * syz + sxz * sxz))).sqrt()
}
// #endregion 🔖️Solid

// #region 🔖️Tet4
/// 🧊️ Four-node linear tetrahedron — constant-strain solid element, DOFs `[Tx,Ty,Tz]` per node.
/// Exact under a single "integration point" (no quadrature loop needed: a linear tet has constant
/// strain over its volume).
pub struct Tet4 {
    pub id: String,
    pub nodes: [String; 4],
    pub e: f64,
    pub nu: f64,
    pub density: f64,
}

#[derive(Clone, Copy)]
struct Tet4Geometry {
    volume: f64,
    gradients: [[f64; 3]; 4],
}

/// 🔺️ Resolves tetrahedral volume and barycentric gradients with fixed-size vector products.
fn tet4_geometry(ctx: &ElementContext) -> Option<Tet4Geometry> {
    if ctx.positions.len() != 4 {
        return None;
    }
    let first = vec3d_sub(ctx.positions[1], ctx.positions[0]);
    let second = vec3d_sub(ctx.positions[2], ctx.positions[0]);
    let third = vec3d_sub(ctx.positions[3], ctx.positions[0]);
    let first_gradient_numerator = vec3d_cross(second, third);
    let determinant = first[0] * first_gradient_numerator[0] + first[1] * first_gradient_numerator[1] + first[2] * first_gradient_numerator[2];
    if !determinant.is_finite() || determinant.abs() <= f64::EPSILON {
        return None;
    }
    let second_gradient_numerator = vec3d_cross(third, first);
    let third_gradient_numerator = vec3d_cross(first, second);
    let inverse = 1.0 / determinant;
    let gradient_one = [first_gradient_numerator[0] * inverse, first_gradient_numerator[1] * inverse, first_gradient_numerator[2] * inverse];
    let gradient_two = [second_gradient_numerator[0] * inverse, second_gradient_numerator[1] * inverse, second_gradient_numerator[2] * inverse];
    let gradient_three = [third_gradient_numerator[0] * inverse, third_gradient_numerator[1] * inverse, third_gradient_numerator[2] * inverse];
    let gradient_zero = [-(gradient_one[0] + gradient_two[0] + gradient_three[0]), -(gradient_one[1] + gradient_two[1] + gradient_three[1]), -(gradient_one[2] + gradient_two[2] + gradient_three[2])];
    let gradients = [gradient_zero, gradient_one, gradient_two, gradient_three];
    gradients.iter().flatten().all(|component| component.is_finite()).then_some(Tet4Geometry { volume: determinant.abs() / 6.0, gradients })
}

impl Tet4 {
    /// 🧭️ Signed volume via the scalar triple product of edge vectors from node 0.
    fn volume(ctx: &ElementContext) -> f64 {
        tet4_geometry(ctx).expect("non-degenerate tet4").volume
    }

    /// 🧭️ Constant per-node shape-function gradients `[∂Li/∂x, ∂Li/∂y, ∂Li/∂z]`. `Li(x,y,z) = a+bx+cy+dz`
    /// with `Li(node_j) = δij` for all j — solving `R·[a,b,c,d]ᵀ = e_i` per node (`R`'s row j is
    /// `[1,xj,yj,zj]`) gives node i's coefficients directly, gradient in components 1..4.
    fn gradients(ctx: &ElementContext) -> [[f64; 3]; 4] {
        tet4_geometry(ctx).expect("non-degenerate tet4").gradients
    }

    /// 🧊️ Evaluates one constant-strain tetrahedral stiffness cell from fixed geometry.
    fn stiffness_cell(&self, geometry: Tet4Geometry, row: usize, column: usize) -> f64 {
        let left = solid_b_column(geometry.gradients[row / 3], row % 3);
        let right = solid_b_column(geometry.gradients[column / 3], column % 3);
        geometry.volume * solid_constitutive_bilinear(self.e, self.nu, left, right)
    }
}

impl Element for Tet4 {
    fn id(&self) -> &str {
        &self.id
    }

    fn node_ids(&self) -> Vec<String> {
        self.nodes.to_vec()
    }

    fn mounted_node_id(&self, index: usize) -> Option<&str> {
        self.nodes.get(index).map(String::as_str)
    }

    fn mounted_node_id_count(&self) -> Option<usize> {
        Some(4)
    }

    fn dofs_per_node(&self) -> &[Dof] {
        const DOFS: [Dof; 3] = [Dof::Tx, Dof::Ty, Dof::Tz];
        &DOFS
    }

    fn stiffness_global(&self, ctx: &ElementContext) -> MatD {
        let geometry = tet4_geometry(ctx).expect("non-degenerate tet4");
        let mut stiffness = MatD::zeros(12, 12);
        for row in 0..12 {
            for column in 0..12 {
                stiffness.set(row, column, self.stiffness_cell(geometry, row, column));
            }
        }
        stiffness
    }

    fn mounted_stiffness_cell(&self, ctx: &ElementContext, row: usize, column: usize) -> Option<f64> {
        if row >= 12 || column >= 12 {
            return None;
        }
        Some(self.stiffness_cell(tet4_geometry(ctx)?, row, column))
    }

    fn recover(&self, ctx: &ElementContext, u_elem: &VecD, _udl: Option<&MemberUdl>) -> ElementResult {
        let grads = Self::gradients(ctx);
        let b = solid_b_matrix(&grads);
        let d = d_matrix_solid(self.e, self.nu);
        let strain = b.mul_vec(u_elem);
        let stress = d.mul_vec(&strain);
        let (sxx, syy, szz, sxy, syz, sxz) = (stress.get(0), stress.get(1), stress.get(2), stress.get(3), stress.get(4), stress.get(5));
        let von_mises = von_mises_solid(sxx, syy, szz, sxy, syz, sxz);
        ElementResult::Solid { gauss: vec![SolidStress { sxx, syy, szz, sxy, syz, sxz, von_mises }] }
    }

    /// 🏋️ Consistent tet mass `ρV/20 * (2 on the diagonal, 1 off-diagonal)` per direction — the
    /// standard closed-form linear-tetrahedron consistent mass (Cook, Malkus, Plesha & Witt), exact
    /// since `Ni=Li` are the tet's own barycentric coordinates.
    fn mass(&self, ctx: &ElementContext) -> Option<MatD> {
        let v = Self::volume(ctx);
        let mut m = MatD::zeros(12, 12);
        for i in 0..4 {
            for j in 0..4 {
                let scalar = self.density * v / 20.0 * if i == j { 2.0 } else { 1.0 };
                for a in 0..3 {
                    m.set(3 * i + a, 3 * j + a, scalar);
                }
            }
        }
        Some(m)
    }

    /// 🌀️ Initial-stress geometric stiffness `Kg = V·Gᵀ·(σ̂⊗I₃)·G` from the element's own (constant)
    /// stress state under `u_elem` — the 3D analogue of `elements2d::plane_geometric_stiffness`,
    /// `σ̂` the full 3x3 stress tensor built from the recovered `[sxx,syy,szz,sxy,syz,sxz]`.
    fn geometric_stiffness(&self, ctx: &ElementContext, u_elem: &VecD) -> Option<MatD> {
        let v = Self::volume(ctx);
        let grads = Self::gradients(ctx);
        let b = solid_b_matrix(&grads);
        let d = d_matrix_solid(self.e, self.nu);
        let strain = b.mul_vec(u_elem);
        let stress = d.mul_vec(&strain);
        let (sxx, syy, szz, sxy, syz, sxz) = (stress.get(0), stress.get(1), stress.get(2), stress.get(3), stress.get(4), stress.get(5));
        let mut kg = MatD::zeros(12, 12);
        for (i, gi) in grads.iter().enumerate() {
            for (j, gj) in grads.iter().enumerate() {
                let s = gi[0] * (sxx * gj[0] + sxy * gj[1] + sxz * gj[2]) + gi[1] * (sxy * gj[0] + syy * gj[1] + syz * gj[2]) + gi[2] * (sxz * gj[0] + syz * gj[1] + szz * gj[2]);
                let val = s * v;
                for a in 0..3 {
                    kg.add_at(3 * i + a, 3 * j + a, val);
                }
            }
        }
        Some(kg)
    }
}
// #endregion 🔖️Tet4

// #region 🔖️Hex8
/// 🧭️ Reference-cube corner sign vectors `(ξi,ηi,ζi)`, node order: bottom face (ζ=-1) CCW from
/// `(-1,-1,-1)` [0-3], top face (ζ=1) CCW from `(-1,-1,1)` [4-7] — node `i+4` sits above node `i`.
const HEX8_CORNERS: [[f64; 3]; 8] = [[-1.0, -1.0, -1.0], [1.0, -1.0, -1.0], [1.0, 1.0, -1.0], [-1.0, 1.0, -1.0], [-1.0, -1.0, 1.0], [1.0, -1.0, 1.0], [1.0, 1.0, 1.0], [-1.0, 1.0, 1.0]];

/// 🧭️ 2x2x2 Gauss points (`±1/√3`, weight 1 each — 8 points, tensor product of the 1D 2-point rule).
fn hex8_gauss_points() -> [([f64; 3], f64); 8] {
    let g = 1.0 / 3.0_f64.sqrt();
    let mut pts = [([0.0; 3], 1.0); 8];
    let mut idx = 0;
    for &xi in &[-g, g] {
        for &eta in &[-g, g] {
            for &zeta in &[-g, g] {
                pts[idx] = ([xi, eta, zeta], 1.0);
                idx += 1;
            }
        }
    }
    pts
}

/// 🧭️ Per-node trilinear shape values `Ni = 0.125*(1+ξξi)(1+ηηi)(1+ζζi)` at one point — shared by
/// `mass`'s `Nᵀ·N` (the stiffness/recover Gauss loop only needed `hex8_param_derivs`, not values).
fn hex8_shape(xi: f64, eta: f64, zeta: f64) -> [f64; 8] {
    let mut n = [0.0; 8];
    for (i, c) in HEX8_CORNERS.iter().enumerate() {
        n[i] = 0.125 * (1.0 + xi * c[0]) * (1.0 + eta * c[1]) * (1.0 + zeta * c[2]);
    }
    n
}

/// 🧭️ Per-node parametric shape-function derivatives `[∂Ni/∂ξ, ∂Ni/∂η, ∂Ni/∂ζ]` at one Gauss point.
fn hex8_param_derivs(xi: f64, eta: f64, zeta: f64) -> [[f64; 3]; 8] {
    let mut out = [[0.0; 3]; 8];
    for (i, c) in HEX8_CORNERS.iter().enumerate() {
        let (xi_i, eta_i, zeta_i) = (c[0], c[1], c[2]);
        out[i] = [0.125 * xi_i * (1.0 + eta * eta_i) * (1.0 + zeta * zeta_i), 0.125 * eta_i * (1.0 + xi * xi_i) * (1.0 + zeta * zeta_i), 0.125 * zeta_i * (1.0 + xi * xi_i) * (1.0 + eta * eta_i)];
    }
    out
}

/// 🧭️ 3x3 determinant via cofactor expansion (Jacobians are always 3x3, no need for general-`n` logic).
fn mat3_det(j: &MatD) -> f64 {
    j.get(0, 0) * (j.get(1, 1) * j.get(2, 2) - j.get(1, 2) * j.get(2, 1)) - j.get(0, 1) * (j.get(1, 0) * j.get(2, 2) - j.get(1, 2) * j.get(2, 0)) + j.get(0, 2) * (j.get(1, 0) * j.get(2, 1) - j.get(1, 1) * j.get(2, 0))
}

/// 🧊️ Eight-node trilinear hexahedron ("brick") — DOFs `[Tx,Ty,Tz]` per node, 2x2x2 Gauss integration.
pub struct Hex8 {
    pub id: String,
    pub nodes: [String; 8],
    pub e: f64,
    pub nu: f64,
    pub density: f64,
}

impl Hex8 {
    /// 🧭️ Jacobian `J[a][b] = Σi ∂Ni/∂param_a · coord_i[b]`, its determinant, and the physical
    /// shape-function gradients `∂Ni/∂[x,y,z] = J⁻¹ · ∂Ni/∂[ξ,η,ζ]` (solved via `lu_solve`, one
    /// right-hand side per node, rather than a hand-derived closed-form 3x3 inverse).
    fn gradients_at(ctx: &ElementContext, xi: f64, eta: f64, zeta: f64) -> (f64, [[f64; 3]; 8]) {
        let param = hex8_param_derivs(xi, eta, zeta);
        let mut j = MatD::zeros(3, 3);
        for (i, pd) in param.iter().enumerate() {
            for (a, derivative) in pd.iter().enumerate() {
                for (b, coordinate) in ctx.positions[i].iter().enumerate() {
                    j.add_at(a, b, derivative * coordinate);
                }
            }
        }
        let det_j = mat3_det(&j);
        let mut grads = [[0.0; 3]; 8];
        for (i, slot) in grads.iter_mut().enumerate() {
            let rhs = VecD::from_vec(param[i].to_vec());
            let phys = j.lu_solve(&rhs).expect("non-degenerate hex8");
            *slot = [phys.get(0), phys.get(1), phys.get(2)];
        }
        (det_j, grads)
    }
}

impl Element for Hex8 {
    fn id(&self) -> &str {
        &self.id
    }

    fn node_ids(&self) -> Vec<String> {
        self.nodes.to_vec()
    }

    fn dofs_per_node(&self) -> &[Dof] {
        const DOFS: [Dof; 3] = [Dof::Tx, Dof::Ty, Dof::Tz];
        &DOFS
    }

    fn stiffness_global(&self, ctx: &ElementContext) -> MatD {
        let d = d_matrix_solid(self.e, self.nu);
        let mut ke = MatD::zeros(24, 24);
        for (p, weight) in hex8_gauss_points() {
            let (det_j, grads) = Self::gradients_at(ctx, p[0], p[1], p[2]);
            let b = solid_b_matrix(&grads);
            ke.add_triple_product(&b, &d, det_j * weight);
        }
        ke
    }

    fn recover(&self, ctx: &ElementContext, u_elem: &VecD, _udl: Option<&MemberUdl>) -> ElementResult {
        let d = d_matrix_solid(self.e, self.nu);
        let gauss = hex8_gauss_points()
            .iter()
            .map(|(p, _)| {
                let (_det_j, grads) = Self::gradients_at(ctx, p[0], p[1], p[2]);
                let b = solid_b_matrix(&grads);
                let strain = b.mul_vec(u_elem);
                let stress = d.mul_vec(&strain);
                let (sxx, syy, szz, sxy, syz, sxz) = (stress.get(0), stress.get(1), stress.get(2), stress.get(3), stress.get(4), stress.get(5));
                let von_mises = von_mises_solid(sxx, syy, szz, sxy, syz, sxz);
                SolidStress { sxx, syy, szz, sxy, syz, sxz, von_mises }
            })
            .collect();
        ElementResult::Solid { gauss }
    }

    /// 🏋️ Consistent trilinear mass `ρ∫Nᵀ·N·dV` over the same 2x2x2 Gauss rule as stiffness — exact,
    /// since `Ni·Nj` (biquadratic-per-axis) is within that rule's precision.
    fn mass(&self, ctx: &ElementContext) -> Option<MatD> {
        let mut m = MatD::zeros(24, 24);
        for (p, weight) in hex8_gauss_points() {
            let (det_j, _) = Self::gradients_at(ctx, p[0], p[1], p[2]);
            let n_vals = hex8_shape(p[0], p[1], p[2]);
            let scale = self.density * det_j * weight;
            for i in 0..8 {
                for j in 0..8 {
                    let v = n_vals[i] * n_vals[j] * scale;
                    for a in 0..3 {
                        m.add_at(3 * i + a, 3 * j + a, v);
                    }
                }
            }
        }
        Some(m)
    }

    /// 🌀️ Initial-stress geometric stiffness, same `Gᵀ(σ̂⊗I₃)G` pattern as `Tet4::geometric_stiffness`
    /// but Gauss-integrated over the element's own 2x2x2 rule (stress varies point-to-point).
    fn geometric_stiffness(&self, ctx: &ElementContext, u_elem: &VecD) -> Option<MatD> {
        let d = d_matrix_solid(self.e, self.nu);
        let mut kg = MatD::zeros(24, 24);
        for (p, weight) in hex8_gauss_points() {
            let (det_j, grads) = Self::gradients_at(ctx, p[0], p[1], p[2]);
            let b = solid_b_matrix(&grads);
            let strain = b.mul_vec(u_elem);
            let stress = d.mul_vec(&strain);
            let (sxx, syy, szz, sxy, syz, sxz) = (stress.get(0), stress.get(1), stress.get(2), stress.get(3), stress.get(4), stress.get(5));
            let scale = det_j * weight;
            for (i, gi) in grads.iter().enumerate() {
                for (j, gj) in grads.iter().enumerate() {
                    let s = gi[0] * (sxx * gj[0] + sxy * gj[1] + sxz * gj[2]) + gi[1] * (sxy * gj[0] + syy * gj[1] + syz * gj[2]) + gi[2] * (sxz * gj[0] + syz * gj[1] + szz * gj[2]);
                    let val = s * scale;
                    for a in 0..3 {
                        kg.add_at(3 * i + a, 3 * j + a, val);
                    }
                }
            }
        }
        Some(kg)
    }
}
// #endregion 🔖️Hex8

// #region 🔖️ShellFacet3
/// 🧭️ Local in-plane axes for a flat triangular facet, built directly from 3 non-collinear 3D
/// points (no roll angle or reference-vector edge case needed — 3 points unambiguously define a
/// plane, unlike `Frame3::local_system`'s 1D-member case): local x along edge `p0->p1`, local z the
/// facet normal (`cross(p1-p0, p2-p0)`, right-hand rule), local y completing the right-handed frame.
fn shell_local_axes(p0: [f64; 3], p1: [f64; 3], p2: [f64; 3]) -> Mat3d {
    let d1 = vec3d_sub(p1, p0);
    let d2 = vec3d_sub(p2, p0);
    let local_x = vec3d_normalize(d1);
    let local_z = vec3d_normalize(vec3d_cross(d1, d2));
    let local_y = vec3d_cross(local_z, local_x);
    Mat3d::from_axes(local_x, local_y, local_z)
}

/// 🧭️ 18x18 global<->local block-diagonal rotation `T` — six `R^T` 3x3 blocks (one per node's
/// translation triple `[Tx,Ty,Tz]`, one per node's rotation triple `[Rx,Ry,Rz]`), the same pattern
/// `Frame3::local_system` uses for its 12x12 `T`, extended to 3 nodes x 2 triples.
fn shell_transform(r: &Mat3d) -> MatD {
    let rt = r.transpose();
    let mut t = MatD::zeros(18, 18);
    for offset in [0usize, 3, 6, 9, 12, 15] {
        for row in 0..3 {
            for col in 0..3 {
                t.set(offset + row, offset + col, rt.cols[col][row]);
            }
        }
    }
    t
}

/// 🐚️ Flat facet shell — 3-node, 6-DOF-per-node (`[Tx,Ty,Tz,Rx,Ry,Rz]`) element combining an in-plane
/// `Tri3Cst`-style CST membrane, `PlateDkt`-style DKT bending, and a small artificial "drilling"
/// stiffness on the local `Rz` (in-plane rotation) DOF — flat shells have no natural stiffness
/// resisting drilling rotation, so a small diagonal stabilization avoids a singular system where
/// coplanar/near-coplanar facets meet. Membrane and bending are exactly decoupled at this (flat,
/// linear) scope, so the local 18x18 stiffness is block-diagonal by construction.
pub struct ShellFacet3 {
    pub id: String,
    pub nodes: [String; 3],
    pub e: f64,
    pub nu: f64,
    pub thickness: f64,
    pub density: f64,
}

/// 🎯️ Small dimensionless drilling-stabilization factor — standard "just enough to avoid
/// singularity, small enough not to distort real behavior" scaling on `k_drill = α·E·t·Area`.
const SHELL_DRILL_ALPHA: f64 = 1e-3;

impl ShellFacet3 {
    /// 🧭️ Local in-plane 2D triangle coordinates (`p0_local=(0,0)`, `p1_local` on the local x-axis,
    /// `p2_local` completing the triangle) plus the local-axes rotation `Mat3d` shared by
    /// `local_stiffness`'s membrane/bending quadrature and `shell_transform`'s `T`.
    fn local_coords(ctx: &ElementContext) -> ([[f64; 2]; 3], Mat3d) {
        let (p0, p1, p2) = (ctx.positions[0], ctx.positions[1], ctx.positions[2]);
        let r = shell_local_axes(p0, p1, p2);
        let (local_x, local_y) = (r.cols[0], r.cols[1]);
        let d1 = vec3d_sub(p1, p0);
        let d2 = vec3d_sub(p2, p0);
        let dot = |a: [f64; 3], b: [f64; 3]| a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
        let coords = [[0.0, 0.0], [dot(d1, local_x), 0.0], [dot(d2, local_x), dot(d2, local_y)]];
        (coords, r)
    }

    /// 🧮️ Local 18x18 stiffness, dof order `[Tx1,Ty1,Tz1,Rx1,Ry1,Rz1, ... x3]`: 6x6 CST membrane
    /// block at each node's `(Tx,Ty)` indices, 9x9 DKT bending block at each node's `(Tz,Rx,Ry)`
    /// indices, drilling diagonal at each node's `Rz` index — all cross-terms zero by construction.
    fn local_stiffness(&self, coords: &[[f64; 2]; 3]) -> MatD {
        let mut k = MatD::zeros(18, 18);

        let d_mem = d_matrix_plane_stress(self.e, self.nu);
        let mut k_mem = MatD::zeros(6, 6);
        for (xi, eta, w) in gauss_tri(1) {
            let (_, dn) = shape_tri3(xi, eta);
            let (_, det_j, d_n_xy) = jacobian_2d(coords, &dn);
            let b = b_matrix_plane(&d_n_xy);
            k_mem.add_triple_product(&b, &d_mem, w * det_j * self.thickness);
        }
        let mem_idx = [0usize, 1, 6, 7, 12, 13];
        for (i, &gi) in mem_idx.iter().enumerate() {
            for (j, &gj) in mem_idx.iter().enumerate() {
                k.set(gi, gj, k_mem.get(i, j));
            }
        }

        let (_, det_j, _) = jacobian_2d(coords, &shape_tri3(0.0, 0.0).1);
        let d_bend = crate::elements2d::d_matrix_plate(self.e, self.nu, self.thickness);
        let mut k_bend = MatD::zeros(9, 9);
        for (xi, eta, w) in gauss_tri(3) {
            let b = crate::elements2d::dkt_b_matrix(coords, xi, eta);
            k_bend.add_triple_product(&b, &d_bend, w * det_j);
        }
        let bend_idx = [2usize, 3, 4, 8, 9, 10, 14, 15, 16];
        for (i, &gi) in bend_idx.iter().enumerate() {
            for (j, &gj) in bend_idx.iter().enumerate() {
                k.set(gi, gj, k_bend.get(i, j));
            }
        }

        let area = 0.5 * det_j;
        let k_drill = SHELL_DRILL_ALPHA * self.e * self.thickness * area;
        for i in 0..3 {
            k.set(6 * i + 5, 6 * i + 5, k_drill);
        }

        k
    }
}

impl Element for ShellFacet3 {
    fn id(&self) -> &str {
        &self.id
    }

    fn node_ids(&self) -> Vec<String> {
        self.nodes.to_vec()
    }

    fn dofs_per_node(&self) -> &[Dof] {
        const DOFS: [Dof; 6] = [Dof::Tx, Dof::Ty, Dof::Tz, Dof::Rx, Dof::Ry, Dof::Rz];
        &DOFS
    }

    fn stiffness_global(&self, ctx: &ElementContext) -> MatD {
        let (coords, r) = Self::local_coords(ctx);
        let t = shell_transform(&r);
        let k_local = self.local_stiffness(&coords);
        t.transpose().matmul(&k_local).matmul(&t)
    }

    fn recover(&self, ctx: &ElementContext, u_elem: &VecD, _udl: Option<&MemberUdl>) -> ElementResult {
        let (coords, r) = Self::local_coords(ctx);
        let t = shell_transform(&r);
        let u_loc = t.mul_vec(u_elem);

        let mem_idx = [0usize, 1, 6, 7, 12, 13];
        let u_mem = VecD::from_vec(mem_idx.iter().map(|&i| u_loc.get(i)).collect());
        let d_mem = d_matrix_plane_stress(self.e, self.nu);
        let (_, dn) = shape_tri3(1.0 / 3.0, 1.0 / 3.0);
        let (_, _, d_n_xy) = jacobian_2d(&coords, &dn);
        let b_mem = b_matrix_plane(&d_n_xy);
        let eps = b_mem.mul_vec(&u_mem);
        let sigma = d_mem.mul_vec(&eps);
        let (nxx, nyy, nxy) = (sigma.get(0) * self.thickness, sigma.get(1) * self.thickness, sigma.get(2) * self.thickness);

        let bend_idx = [2usize, 3, 4, 8, 9, 10, 14, 15, 16];
        let u_bend = VecD::from_vec(bend_idx.iter().map(|&i| u_loc.get(i)).collect());
        let d_bend = crate::elements2d::d_matrix_plate(self.e, self.nu, self.thickness);
        let b_bend = crate::elements2d::dkt_b_matrix(&coords, 1.0 / 3.0, 1.0 / 3.0);
        let kappa = b_bend.mul_vec(&u_bend);
        let m = d_bend.mul_vec(&kappa);
        let (mxx, myy, mxy) = (m.get(0), m.get(1), m.get(2));

        let t_th = self.thickness;
        let surface = |sign: f64| {
            let sxx = nxx / t_th + sign * 6.0 * mxx / (t_th * t_th);
            let syy = nyy / t_th + sign * 6.0 * myy / (t_th * t_th);
            let sxy = nxy / t_th + sign * 6.0 * mxy / (t_th * t_th);
            (sxx * sxx - sxx * syy + syy * syy + 3.0 * sxy * sxy).sqrt()
        };
        let von_mises_top = surface(1.0);
        let von_mises_bottom = surface(-1.0);

        ElementResult::Shell { gauss: vec![ShellState { nxx, nyy, nxy, mxx, myy, mxy, von_mises_top, von_mises_bottom }] }
    }

    /// 🏋️ Lumped translational mass `ρtA/3` on each node's `[Tx,Ty,Tz]` — diagonal and isotropic
    /// (equal in all 3 local translation directions), so it needs no local->global rotation, unlike
    /// `local_stiffness`. Zero rotational inertia, same lumping rationale as `PlateDkt::mass`.
    fn mass(&self, ctx: &ElementContext) -> Option<MatD> {
        let (coords, _) = Self::local_coords(ctx);
        let (_, det_j, _) = jacobian_2d(&coords, &shape_tri3(0.0, 0.0).1);
        let area = 0.5 * det_j;
        let share = self.density * self.thickness * area / 3.0;
        let mut m = MatD::zeros(18, 18);
        for i in 0..3 {
            for a in 0..3 {
                m.set(6 * i + a, 6 * i + a, share);
            }
        }
        Some(m)
    }

    /// 🌀️ Geometric stiffness from the facet's own (constant) CST membrane forces `Nxx,Nyy,Nxy`
    /// acting on the LINEAR CST-interpolated out-of-plane `w` gradient (the standard flat-facet
    /// simplification — the DKT bending field's rotation-driven curvature correction is neglected
    /// for this coupling, following common practice for flat shell buckling), local `Tz` dof per
    /// node using the SAME constant gradient `local_stiffness`'s membrane block computes.
    fn geometric_stiffness(&self, ctx: &ElementContext, u_element: &VecD) -> Option<MatD> {
        let (coords, r) = Self::local_coords(ctx);
        let t = shell_transform(&r);
        let u_loc = t.mul_vec(u_element);

        let mem_idx = [0usize, 1, 6, 7, 12, 13];
        let u_mem = VecD::from_vec(mem_idx.iter().map(|&i| u_loc.get(i)).collect());
        let d_mem = d_matrix_plane_stress(self.e, self.nu);
        let (_, dn) = shape_tri3(1.0 / 3.0, 1.0 / 3.0);
        let (_, det_j, d_n_xy) = jacobian_2d(&coords, &dn);
        let b_mem = b_matrix_plane(&d_n_xy);
        let eps = b_mem.mul_vec(&u_mem);
        let sigma = d_mem.mul_vec(&eps);
        let (nxx, nyy, nxy) = (sigma.get(0) * self.thickness, sigma.get(1) * self.thickness, sigma.get(2) * self.thickness);

        let area = 0.5 * det_j;
        let w_idx = [2usize, 8, 14];
        let mut kg_local = MatD::zeros(18, 18);
        for i in 0..3 {
            let (gix, giy) = (d_n_xy[i][0], d_n_xy[i][1]);
            for j in 0..3 {
                let (gjx, gjy) = (d_n_xy[j][0], d_n_xy[j][1]);
                let s = gix * nxx * gjx + gix * nxy * gjy + giy * nxy * gjx + giy * nyy * gjy;
                kg_local.add_at(w_idx[i], w_idx[j], s * area);
            }
        }
        Some(t.transpose().matmul(&kg_local).matmul(&t))
    }
}
// #endregion 🔖️ShellFacet3

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests

// #region 🔖️SolidTests
#[cfg(test)]
#[path = "🧪️tests/🔬️solid/🦀️.rs"]
mod solid_tests;
// #endregion 🔖️SolidTests

// #region 🔖️ShellTests
#[cfg(test)]
#[path = "🧪️tests/🔬️shell/🦀️.rs"]
mod shell_tests;
// #endregion 🔖️ShellTests
