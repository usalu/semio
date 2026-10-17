//! 🌉️ Line geometry, the axial `Bar3` truss and the `Frame3` frame member with torsion and member loads.

use super::*;

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
    fn close_mounted_string_step(&mut self) -> Option<usize> {
        semio_s_fem_model::close_mounted_strings([&mut self.id, &mut self.node_a, &mut self.node_b])
    }

    fn mounted_next_string_bytes(&self) -> Option<usize> {
        semio_s_fem_model::mounted_string_bytes([&self.id, &self.node_a, &self.node_b])
    }

    fn mounted_strings_terminal_is_empty(&self) -> bool {
        self.mounted_next_string_bytes().is_none()
    }

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
pub const Y_PLANE_SIGN: [f64; 4] = [1.0, -1.0, 1.0, -1.0];

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
    pub(super) fn local_stiffness(&self, l: f64) -> MatD {
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
    pub fn local_mass(&self, l: f64) -> MatD {
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
    pub fn local_geometric_stiffness(&self, l: f64, n: f64) -> MatD {
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
    fn close_mounted_string_step(&mut self) -> Option<usize> {
        semio_s_fem_model::close_mounted_strings([&mut self.id, &mut self.node_a, &mut self.node_b])
    }

    fn mounted_next_string_bytes(&self) -> Option<usize> {
        semio_s_fem_model::mounted_string_bytes([&self.id, &self.node_a, &self.node_b])
    }

    fn mounted_strings_terminal_is_empty(&self) -> bool {
        self.mounted_next_string_bytes().is_none()
    }

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
