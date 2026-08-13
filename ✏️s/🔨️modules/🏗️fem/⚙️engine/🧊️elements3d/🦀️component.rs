//! 🧊️ 3D structural elements: axial `Bar3` truss, Euler-Bernoulli `Frame3` frame member (with
//! torsion and member-UDL support), the `Tet4`/`Hex8` solid continuum elements, and `ShellFacet3`
//! (flat facet shell: CST membrane + DKT bending + drilling stabilization).

use crate::formulation::{b_matrix_plane, d_matrix_plane_stress, gauss_tri, jacobian_2d, shape_tri3};
use crate::model::{BeamStation, Dof, Element, ElementContext, ElementResult, MemberUdl, ShellState, SolidStress};
use crate::algebra::{vec3d_cross, vec3d_length, vec3d_normalize, vec3d_sub, Mat3d, MatD, VecD};

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

    fn dofs_per_node(&self) -> &[Dof] {
        const DOFS: [Dof; 3] = [Dof::Tx, Dof::Ty, Dof::Tz];
        &DOFS
    }

    fn stiffness_global(&self, ctx: &ElementContext) -> MatD {
        let d = vec3d_sub(ctx.positions[1], ctx.positions[0]);
        let l = vec3d_length(d);
        let c = vec3d_normalize(d);
        let k = self.e * self.a / l;
        let mut ke = MatD::zeros(6, 6);
        for i in 0..3 {
            for j in 0..3 {
                let v = k * c[i] * c[j];
                ke.set(i, j, v);
                ke.set(i, j + 3, -v);
                ke.set(i + 3, j, -v);
                ke.set(i + 3, j + 3, v);
            }
        }
        ke
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
/// 🧮️ Places a 4x4 bending block into `k` at the given DOF indices (used for both the y- and z-bending
/// planes, which are decoupled from each other and from axial/torsion).
fn set_bend_block(k: &mut MatD, idx: [usize; 4], block: [[f64; 4]; 4]) {
    for (bi, &gi) in idx.iter().enumerate() {
        for (bj, &gj) in idx.iter().enumerate() {
            k.set(gi, gj, block[bi][bj]);
        }
    }
}

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

impl Frame3 {
    /// 🧭️ Builds the member length, local 12x12 stiffness, and the 12x12 global<->local block-diagonal
    /// rotation `T` (four `R^T` 3x3 blocks) shared by `stiffness_global` and `recover`.
    fn local_system(&self, ctx: &ElementContext) -> (f64, MatD, MatD) {
        let d = vec3d_sub(ctx.positions[1], ctx.positions[0]);
        let l = vec3d_length(d);
        let cx = vec3d_normalize(d);
        let reference = if cx[2].abs() > 0.99 { [1.0, 0.0, 0.0] } else { [0.0, 0.0, 1.0] };
        let y_unrot = vec3d_normalize(vec3d_cross(reference, cx));
        let z_unrot = vec3d_cross(cx, y_unrot);
        let (sin_r, cos_r) = self.roll.sin_cos();
        let local_y = [y_unrot[0] * cos_r + z_unrot[0] * sin_r, y_unrot[1] * cos_r + z_unrot[1] * sin_r, y_unrot[2] * cos_r + z_unrot[2] * sin_r];
        let local_z = [z_unrot[0] * cos_r - y_unrot[0] * sin_r, z_unrot[1] * cos_r - y_unrot[1] * sin_r, z_unrot[2] * cos_r - y_unrot[2] * sin_r];
        let rt = Mat3d::from_axes(cx, local_y, local_z).transpose();
        let mut t = MatD::zeros(12, 12);
        for offset in [0usize, 3, 6, 9] {
            for row in 0..3 {
                for col in 0..3 {
                    t.set(offset + row, offset + col, rt.cols[col][row]);
                }
            }
        }
        (l, self.local_stiffness(l), t)
    }

    /// 🧮️ Decoupled local 12x12 stiffness: axial, torsion, and biaxial (y/z) Euler-Bernoulli bending.
    fn local_stiffness(&self, l: f64) -> MatD {
        let mut k = MatD::zeros(12, 12);
        let l2 = l * l;
        let ax = self.e * self.a / l;
        k.set(0, 0, ax);
        k.set(0, 6, -ax);
        k.set(6, 0, -ax);
        k.set(6, 6, ax);
        let tor = self.g * self.j / l;
        k.set(3, 3, tor);
        k.set(3, 9, -tor);
        k.set(9, 3, -tor);
        k.set(9, 9, tor);
        let bz = self.e * self.iz / l;
        set_bend_block(
            &mut k,
            [1, 5, 7, 11],
            [[12.0 * bz / l2, 6.0 * bz / l, -12.0 * bz / l2, 6.0 * bz / l], [6.0 * bz / l, 4.0 * bz, -6.0 * bz / l, 2.0 * bz], [-12.0 * bz / l2, -6.0 * bz / l, 12.0 * bz / l2, -6.0 * bz / l], [6.0 * bz / l, 2.0 * bz, -6.0 * bz / l, 4.0 * bz]],
        );
        let by = self.e * self.iy / l;
        set_bend_block(
            &mut k,
            [2, 4, 8, 10],
            [[12.0 * by / l2, -6.0 * by / l, -12.0 * by / l2, -6.0 * by / l], [-6.0 * by / l, 4.0 * by, 6.0 * by / l, 2.0 * by], [-12.0 * by / l2, 6.0 * by / l, 12.0 * by / l2, 6.0 * by / l], [-6.0 * by / l, 2.0 * by, 6.0 * by / l, 4.0 * by]],
        );
        k
    }

    /// 🏋️ Local 12x12 consistent mass: axial `ρAL/6*[[2,1],[1,2]]` at `(0,6)`, torsion `ρJL/6*[[2,1],[1,2]]`
    /// at `(3,9)` — a simplified polar-inertia proxy (not rigorously exact rotary inertia, but the
    /// accepted simplification at this scope — see `mass`'s doc), and both bending planes ([1,5,7,11]
    /// z-plane, [2,4,8,10] y-plane) using the same 156/22L/54/-13L consistent-beam-mass pattern.
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
                m.set(gi, gj, factor * block[bi][bj]);
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
                kg.set(gi, gj, coeff * block[bi][bj]);
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

    fn dofs_per_node(&self) -> &[Dof] {
        const DOFS: [Dof; 6] = [Dof::Tx, Dof::Ty, Dof::Tz, Dof::Rx, Dof::Ry, Dof::Rz];
        &DOFS
    }

    fn stiffness_global(&self, ctx: &ElementContext) -> MatD {
        let (_l, k_local, t) = self.local_system(ctx);
        t.transpose().matmul(&k_local).matmul(&t)
    }

    fn equivalent_nodal_loads(&self, ctx: &ElementContext, udl: &MemberUdl) -> Option<VecD> {
        let (l, _k_local, t) = self.local_system(ctx);
        let f_local = local_udl(l, &t, udl);
        Some(t.transpose().mul_vec(&f_local))
    }

    fn recover(&self, ctx: &ElementContext, u_elem: &VecD, udl: Option<&MemberUdl>) -> ElementResult {
        let (l, k_local, t) = self.local_system(ctx);
        let u_loc = t.mul_vec(u_elem);
        let f_udl_local = udl.map(|u| local_udl(l, &t, u)).unwrap_or_else(|| VecD::zeros(12));
        let f = k_local.mul_vec(&u_loc).sub(&f_udl_local);
        let n = -f.get(0);
        let v1 = f.get(2);
        let m1 = f.get(4);
        let wz_l = udl.map(|u| local_udl_components(&t, u).2).unwrap_or(0.0);
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

impl Tet4 {
    /// 🧭️ Signed volume via the scalar triple product of edge vectors from node 0.
    fn volume(ctx: &ElementContext) -> f64 {
        let p = &ctx.positions;
        let e1 = vec3d_sub(p[1], p[0]);
        let e2 = vec3d_sub(p[2], p[0]);
        let e3 = vec3d_sub(p[3], p[0]);
        let cross = vec3d_cross(e1, e2);
        (cross[0] * e3[0] + cross[1] * e3[1] + cross[2] * e3[2]).abs() / 6.0
    }

    /// 🧭️ Constant per-node shape-function gradients `[∂Li/∂x, ∂Li/∂y, ∂Li/∂z]`. `Li(x,y,z) = a+bx+cy+dz`
    /// with `Li(node_j) = δij` for all j — solving `R·[a,b,c,d]ᵀ = e_i` per node (`R`'s row j is
    /// `[1,xj,yj,zj]`) gives node i's coefficients directly, gradient in components 1..4.
    fn gradients(ctx: &ElementContext) -> [[f64; 3]; 4] {
        let p = &ctx.positions;
        let mut r = MatD::zeros(4, 4);
        for (j, pj) in p.iter().enumerate() {
            r.set(j, 0, 1.0);
            r.set(j, 1, pj[0]);
            r.set(j, 2, pj[1]);
            r.set(j, 3, pj[2]);
        }
        let mut grads = [[0.0; 3]; 4];
        for (i, slot) in grads.iter_mut().enumerate() {
            let mut e = VecD::zeros(4);
            e.set(i, 1.0);
            let coeffs = r.lu_solve(&e).expect("non-degenerate tet4");
            *slot = [coeffs.get(1), coeffs.get(2), coeffs.get(3)];
        }
        grads
    }
}

impl Element for Tet4 {
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
        let v = Self::volume(ctx);
        let grads = Self::gradients(ctx);
        let b = solid_b_matrix(&grads);
        let d = d_matrix_solid(self.e, self.nu);
        let mut ke = MatD::zeros(12, 12);
        ke.add_triple_product(&b, &d, v);
        ke
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
        for i in 0..4 {
            let gi = grads[i];
            for j in 0..4 {
                let gj = grads[j];
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
            for a in 0..3 {
                for b in 0..3 {
                    j.add_at(a, b, pd[a] * ctx.positions[i][b]);
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
            for i in 0..8 {
                let gi = grads[i];
                for j in 0..8 {
                    let gj = grads[j];
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
mod tests {
    use super::*;
    use crate::model::{solve_linear_static, Model, NodalLoad, Node, Support};

    /// 🪵️ Headless axial elongation check along an arbitrary (non-axis-aligned) 3D direction.
    #[test]
    fn bar3_axial_matches_hand_calc_on_skew_member() {
        // A free 3D joint needs ≥3 non-coplanar bars to be determinate (see `truss_fixture` below),
        // so `b` gets two extra fixed-node bars (to `d` and `c`) besides the member under test (`e1`).
        // Loading exactly along e1's own axis (0.6,0.8,0) makes e1 carry the full load by equilibrium
        // (hand-solved: N_e1 = p, N_bd = N_bc = 0) — a clean, unambiguous check on a genuinely skew direction.
        let (e, a) = (200e9, 0.001);
        let l = 5.0;
        let p = 2000.0;
        let model = Model {
            nodes: vec![
                Node { id: "a".into(), pos: [0.0, 0.0, 0.0] },
                Node { id: "b".into(), pos: [3.0, 4.0, 0.0] }, // length 5 from a, direction (0.6, 0.8, 0.0)
                Node { id: "c".into(), pos: [3.0, 4.0, 2.0] },
                Node { id: "d".into(), pos: [3.0, 0.0, 0.0] },
            ],
            elements: vec![
                Box::new(Bar3 { id: "e1".into(), node_a: "a".into(), node_b: "b".into(), e, a, density: 0.0 }),
                Box::new(Bar3 { id: "bc".into(), node_a: "b".into(), node_b: "c".into(), e, a, density: 0.0 }),
                Box::new(Bar3 { id: "bd".into(), node_a: "b".into(), node_b: "d".into(), e, a, density: 0.0 }),
            ],
            supports: vec![Support { node_id: "a".into(), fixed: vec![Dof::Tx, Dof::Ty, Dof::Tz] }, Support { node_id: "c".into(), fixed: vec![Dof::Tx, Dof::Ty, Dof::Tz] }, Support { node_id: "d".into(), fixed: vec![Dof::Tx, Dof::Ty, Dof::Tz] }],
            nodal_loads: vec![NodalLoad { node_id: "b".into(), dof: Dof::Tx, value: p * 0.6 }, NodalLoad { node_id: "b".into(), dof: Dof::Ty, value: p * 0.8 }],
            member_loads: vec![],
        };
        let result = solve_linear_static(&model).expect("solves");
        let n_e1 = result
            .elements
            .iter()
            .find(|(id, _)| id == "e1")
            .map(|(_, r)| match r {
                ElementResult::Bar { n } => *n,
                _ => panic!("expected bar"),
            })
            .unwrap();
        assert!((n_e1 - p).abs() / p < 1e-6, "axial force {n_e1} vs expected {p}");
        let expected_elongation = p * l / (e * a);
        let b = result.displacements.iter().find(|d| d.node_id == "b").unwrap();
        let actual_elongation = b.values[Dof::Tx.index()] * 0.6 + b.values[Dof::Ty.index()] * 0.8;
        assert!((actual_elongation - expected_elongation).abs() / expected_elongation < 1e-6);
    }

    /// 🌀️ Rigid-body test: a pure 3D translation must produce zero internal force on a `Frame3`.
    #[test]
    fn frame3_rigid_translation_gives_zero_force() {
        let frame = Frame3 { id: "e1".into(), node_a: "a".into(), node_b: "b".into(), e: 210e9, g: 80.77e9, a: 0.005, iy: 1e-5, iz: 1e-5, j: 1e-6, roll: 0.0, density: 0.0 };
        let ctx = ElementContext { positions: vec![[0.0, 0.0, 0.0], [2.0, 1.0, 0.5]] };
        let ke = frame.stiffness_global(&ctx);
        let rigid = VecD::from_vec(vec![1.0, 2.0, 3.0, 0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 0.0, 0.0, 0.0]);
        let f = ke.mul_vec(&rigid);
        for i in 0..12 {
            assert!(f.get(i).abs() < 1e-6, "rigid-body force[{i}] = {}", f.get(i));
        }
    }

    /// 🌬️ Cantilever `Frame3` under a member UDL, checked against the classical cantilever-under-UDL
    /// formulas: base moment `wL²/2`, base shear `wL`, ~0 at the free tip. The member runs along
    /// global X with `roll: 0.0`, for which `local_system`'s reference-vector logic aligns local y/z
    /// with global Y/Z exactly — so a UDL in global `wz` lands directly in the local z-bending plane
    /// that `recover` already reports via `v1 = f.get(2)`/`m1 = f.get(4)`.
    #[test]
    fn frame3_udl_cantilever_matches_hand_calc() {
        let (e, g, a, iy, iz, j) = (200e9, 80e9, 0.01, 8e-5, 8e-5, 1e-6);
        let l = 4.0;
        let w = 1000.0;
        let model = Model {
            nodes: vec![Node { id: "a".into(), pos: [0.0, 0.0, 0.0] }, Node { id: "b".into(), pos: [l, 0.0, 0.0] }],
            elements: vec![Box::new(Frame3 { id: "e1".into(), node_a: "a".into(), node_b: "b".into(), e, g, a, iy, iz, j, roll: 0.0, density: 0.0 })],
            supports: vec![Support { node_id: "a".into(), fixed: vec![Dof::Tx, Dof::Ty, Dof::Tz, Dof::Rx, Dof::Ry, Dof::Rz] }],
            nodal_loads: vec![],
            member_loads: vec![("e1".into(), MemberUdl { wx: 0.0, wy: 0.0, wz: -w })],
        };
        let result = solve_linear_static(&model).expect("solves");
        let (_, e1_result) = result.elements.iter().find(|(id, _)| id == "e1").unwrap();
        let stations = match e1_result {
            ElementResult::Beam { stations } => stations,
            _ => panic!("expected beam"),
        };
        let base = stations.first().unwrap();
        let tip = stations.last().unwrap();
        let expected_m = w * l * l / 2.0;
        let expected_v = w * l;
        assert!((base.m.abs() - expected_m).abs() / expected_m < 1e-6, "base moment {} vs expected {}", base.m, expected_m);
        assert!((base.v.abs() - expected_v).abs() / expected_v < 1e-6, "base shear {} vs expected {}", base.v, expected_v);
        assert!(tip.m.abs() < expected_m * 1e-6, "tip moment {} should be ~0", tip.m);
        assert!(tip.v.abs() < expected_v * 1e-6, "tip shear {} should be ~0", tip.v);
    }

    /// 🏋️ `Bar3::mass` matches the hand-derived isotropic `m = ρAL/6` block form (3x3 identity blocks).
    #[test]
    fn bar3_mass_matches_hand_calc() {
        let (density, a, l) = (7850.0, 0.001, 5.0);
        let bar = Bar3 { id: "e1".into(), node_a: "a".into(), node_b: "b".into(), e: 200e9, a, density };
        let ctx = ElementContext { positions: vec![[0.0, 0.0, 0.0], [3.0, 4.0, 0.0]] };
        let m = bar.mass(&ctx).expect("bar3 reports mass");
        let expected = density * a * l / 6.0;
        for i in 0..3 {
            assert!((m.get(i, i) - 2.0 * expected).abs() < 1e-9);
            assert!((m.get(i + 3, i + 3) - 2.0 * expected).abs() < 1e-9);
            assert!((m.get(i, i + 3) - expected).abs() < 1e-9);
        }
        assert!(m.get(0, 1).abs() < 1e-12, "no coupling across directions");
    }

    /// ⚖️ Sum of ALL entries of `Bar3::mass` (a pure translational, no-rotation element) equals the
    /// total member mass `ρAL` — same partition-of-unity identity as `Bar2`'s.
    #[test]
    fn bar3_mass_total_equals_rho_a_l() {
        let (density, a, l) = (7850.0, 0.001, 5.0);
        let bar = Bar3 { id: "e1".into(), node_a: "a".into(), node_b: "b".into(), e: 200e9, a, density };
        let ctx = ElementContext { positions: vec![[0.0, 0.0, 0.0], [3.0, 4.0, 0.0]] };
        let m = bar.mass(&ctx).expect("bar3 reports mass");
        let mut sum_x = 0.0;
        for &r in &[0usize, 3] {
            for &c in &[0usize, 3] {
                sum_x += m.get(r, c);
            }
        }
        assert!((sum_x - density * a * l).abs() / (density * a * l) < 1e-9);
    }

    /// 🏋️ `Frame3::mass`'s axial and torsion 2x2 submatrices each sum to their own hand-derived total
    /// (`ρAL` axial, `ρJL` torsion) — checked on a member along global X with `roll: 0.0`, for which
    /// `local_system` aligns local axes with global ones exactly (rotation is identity).
    #[test]
    fn frame3_mass_axial_and_torsion_blocks_sum_to_total() {
        let (e, g, a, iy, iz, j, density) = (200e9, 80e9, 0.01, 8e-5, 8e-5, 1e-6, 7850.0);
        let l = 4.0;
        let frame = Frame3 { id: "e1".into(), node_a: "a".into(), node_b: "b".into(), e, g, a, iy, iz, j, roll: 0.0, density };
        let ctx = ElementContext { positions: vec![[0.0, 0.0, 0.0], [l, 0.0, 0.0]] };
        let m = frame.mass(&ctx).expect("frame3 reports mass");
        let sum_axial = m.get(0, 0) + m.get(0, 6) + m.get(6, 0) + m.get(6, 6);
        assert!((sum_axial - density * a * l).abs() / (density * a * l) < 1e-9);
        let sum_torsion = m.get(3, 3) + m.get(3, 9) + m.get(9, 3) + m.get(9, 9);
        assert!((sum_torsion - density * j * l).abs() / (density * j * l) < 1e-9);
    }

    /// 🌀️ `Frame3` geometric stiffness must vanish under a pure rigid translation.
    #[test]
    fn frame3_geometric_stiffness_rigid_translation_gives_zero_force() {
        let frame = Frame3 { id: "e1".into(), node_a: "a".into(), node_b: "b".into(), e: 210e9, g: 80.77e9, a: 0.005, iy: 1e-5, iz: 1e-5, j: 1e-6, roll: 0.0, density: 0.0 };
        let ctx = ElementContext { positions: vec![[0.0, 0.0, 0.0], [2.0, 1.0, 0.5]] };
        let mut u = VecD::zeros(12);
        u.set(6, 0.001);
        let kg = frame.geometric_stiffness(&ctx, &u).expect("frame3 reports geometric stiffness");
        let rigid = VecD::from_vec(vec![1.0, 2.0, 3.0, 0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 0.0, 0.0, 0.0]);
        let f = kg.mul_vec(&rigid);
        for i in 0..12 {
            assert!(f.get(i).abs() < 1e-6, "rigid-body geometric force[{i}] = {}", f.get(i));
        }
    }

    /// 🌀️ `Frame3` geometric stiffness is symmetric.
    #[test]
    fn frame3_geometric_stiffness_is_symmetric() {
        let frame = Frame3 { id: "e1".into(), node_a: "a".into(), node_b: "b".into(), e: 210e9, g: 80.77e9, a: 0.005, iy: 1e-5, iz: 1e-5, j: 1e-6, roll: 0.0, density: 0.0 };
        let ctx = ElementContext { positions: vec![[0.0, 0.0, 0.0], [2.0, 1.0, 0.5]] };
        let mut u = VecD::zeros(12);
        u.set(6, 0.001);
        let kg = frame.geometric_stiffness(&ctx, &u).unwrap();
        for r in 0..12 {
            for c in 0..12 {
                assert!((kg.get(r, c) - kg.get(c, r)).abs() < 1e-9, "Kg not symmetric at ({r},{c})");
            }
        }
    }

    /// 🌬️ `Bar3::equivalent_nodal_loads` splits a global UDL `wL/2` exactly evenly at both nodes —
    /// the 3D analogue of `elements2d::bar2_equivalent_nodal_loads_matches_wl_over_2`.
    #[test]
    fn bar3_equivalent_nodal_loads_matches_wl_over_2() {
        let (e, a, l) = (200e9, 0.001, 5.0);
        let bar = Bar3 { id: "e1".into(), node_a: "a".into(), node_b: "b".into(), e, a, density: 0.0 };
        let ctx = ElementContext { positions: vec![[0.0, 0.0, 0.0], [3.0, 4.0, 0.0]] };
        let udl = MemberUdl { wx: 100.0, wy: -50.0, wz: 20.0 };
        let f = bar.equivalent_nodal_loads(&ctx, &udl).expect("bar3 reports equivalent nodal loads");
        let half = l / 2.0;
        assert!((f.get(0) - udl.wx * half).abs() < 1e-9);
        assert!((f.get(1) - udl.wy * half).abs() < 1e-9);
        assert!((f.get(2) - udl.wz * half).abs() < 1e-9);
        assert!((f.get(3) - udl.wx * half).abs() < 1e-9);
        assert!((f.get(4) - udl.wy * half).abs() < 1e-9);
        assert!((f.get(5) - udl.wz * half).abs() < 1e-9);
    }

    /// 🌀️ `Bar3::geometric_stiffness`: zero under rigid translation, symmetric, and (same reasoning
    /// as `elements2d::bar2_geometric_stiffness_rigid_translation_gives_zero_force_and_is_symmetric`)
    /// zero along the bar's own axis.
    #[test]
    fn bar3_geometric_stiffness_rigid_translation_gives_zero_force_and_is_symmetric() {
        let (e, a) = (200e9, 0.001);
        let bar = Bar3 { id: "e1".into(), node_a: "a".into(), node_b: "b".into(), e, a, density: 0.0 };
        let ctx = ElementContext { positions: vec![[0.0, 0.0, 0.0], [3.0, 4.0, 0.0]] };
        let u = VecD::from_vec(vec![0.0, 0.0, 0.0, 0.001, 0.0, 0.0]);
        let kg = bar.geometric_stiffness(&ctx, &u).expect("bar3 reports geometric stiffness");
        for r in 0..6 {
            for c in 0..6 {
                assert!((kg.get(r, c) - kg.get(c, r)).abs() < 1e-9, "Kg not symmetric at ({r},{c})");
            }
        }
        let rigid = VecD::from_vec(vec![3.0, 4.0, 1.0, 3.0, 4.0, 1.0]);
        let f = kg.mul_vec(&rigid);
        for i in 0..6 {
            assert!(f.get(i).abs() < 1e-6, "rigid-body geometric force[{i}] = {}", f.get(i));
        }
    }
}
// #endregion 🔖️Tests

// #region 🔖️SolidTests
#[cfg(test)]
mod solid_tests {
    use super::*;
    use crate::model::{solve_linear_static, Model, NodalLoad, Node, Support};

    /// 🧮️ Linear displacement field `u=ux·x+uy·y+uz·z` (and analogous `v`,`w`) shared by the Tet4/Hex8
    /// patch tests — its gradient (hence strain) is constant everywhere, so a direct
    /// `stiffness_global`/`recover` call can be checked against the closed-form `σ=Dε` exactly.
    struct LinearField {
        ux: f64,
        uy: f64,
        uz: f64,
        vx: f64,
        vy: f64,
        vz: f64,
        wx: f64,
        wy: f64,
        wz: f64,
    }

    impl LinearField {
        fn sample() -> Self {
            Self { ux: 0.0011, uy: 0.0007, uz: -0.0004, vx: -0.0006, vy: 0.0012, vz: 0.0003, wx: 0.0002, wy: -0.0005, wz: 0.0009 }
        }

        fn displacement_at(&self, p: [f64; 3]) -> [f64; 3] {
            [self.ux * p[0] + self.uy * p[1] + self.uz * p[2], self.vx * p[0] + self.vy * p[1] + self.vz * p[2], self.wx * p[0] + self.wy * p[1] + self.wz * p[2]]
        }

        /// 🧮️ Analytical `σ = Dε` for this field's (constant) strain, hand-expanded independently of
        /// `d_matrix_solid` as a cross-check of the whole B/D recovery pipeline.
        fn expected_stress(&self, e: f64, nu: f64) -> SolidStress {
            let (exx, eyy, ezz) = (self.ux, self.vy, self.wz);
            let (gxy, gyz, gxz) = (self.uy + self.vx, self.vz + self.wy, self.uz + self.wx);
            let s = e / ((1.0 + nu) * (1.0 - 2.0 * nu));
            let sxx = s * ((1.0 - nu) * exx + nu * eyy + nu * ezz);
            let syy = s * (nu * exx + (1.0 - nu) * eyy + nu * ezz);
            let szz = s * (nu * exx + nu * eyy + (1.0 - nu) * ezz);
            let g = s * (1.0 - 2.0 * nu) / 2.0;
            let (sxy, syz, sxz) = (g * gxy, g * gyz, g * gxz);
            let von_mises = von_mises_solid(sxx, syy, szz, sxy, syz, sxz);
            SolidStress { sxx, syy, szz, sxy, syz, sxz, von_mises }
        }

        fn nodal_vector(&self, positions: &[[f64; 3]]) -> VecD {
            let mut data = Vec::with_capacity(positions.len() * 3);
            for &p in positions {
                data.extend_from_slice(&self.displacement_at(p));
            }
            VecD::from_vec(data)
        }
    }

    /// 🔍️ Component-wise relative comparison (scaled by `max(|expected|, 1.0)` so a near-zero
    /// expected component doesn't demand an absurdly tight absolute match).
    fn assert_stress_close(actual: &SolidStress, expected: &SolidStress, rel_tol: f64) {
        let check = |name: &str, a: f64, ex: f64| {
            let scale = ex.abs().max(1.0);
            assert!((a - ex).abs() / scale < rel_tol, "{name}: {a} vs expected {ex}");
        };
        check("sxx", actual.sxx, expected.sxx);
        check("syy", actual.syy, expected.syy);
        check("szz", actual.szz, expected.szz);
        check("sxy", actual.sxy, expected.sxy);
        check("syz", actual.syz, expected.syz);
        check("sxz", actual.sxz, expected.sxz);
    }

    // #region 🔖️Tet4
    fn skew_tet_positions() -> [[f64; 3]; 4] {
        [[0.0, 0.0, 0.0], [1.0, 0.1, 0.05], [0.2, 1.0, 0.1], [0.1, 0.15, 1.0]]
    }

    /// 🧮️ Constant-strain patch test: an exact linear field imposed at every node must recover the
    /// exact analytical `σ=Dε` at the (single, exact) integration point.
    #[test]
    fn tet4_patch_test_recovers_exact_constant_stress() {
        let (e, nu) = (200e9, 0.3);
        let positions = skew_tet_positions();
        let field = LinearField::sample();
        let ctx = ElementContext { positions: positions.to_vec() };
        let tet = Tet4 { id: "t1".into(), nodes: ["n0".into(), "n1".into(), "n2".into(), "n3".into()], e, nu, density: 0.0 };
        let ke = tet.stiffness_global(&ctx);
        assert_eq!(ke.rows, 12);
        let u = field.nodal_vector(&positions);
        let result = tet.recover(&ctx, &u, None);
        let ElementResult::Solid { gauss } = result else { panic!("expected solid") };
        assert_eq!(gauss.len(), 1);
        assert_stress_close(&gauss[0], &field.expected_stress(e, nu), 1e-6);
    }

    /// 🌀️ Rigid-body test: a pure translation of all 4 nodes must produce zero internal force.
    #[test]
    fn tet4_rigid_translation_gives_zero_force() {
        let (e, nu) = (200e9, 0.3);
        let positions = skew_tet_positions();
        let ctx = ElementContext { positions: positions.to_vec() };
        let tet = Tet4 { id: "t1".into(), nodes: ["n0".into(), "n1".into(), "n2".into(), "n3".into()], e, nu, density: 0.0 };
        let ke = tet.stiffness_global(&ctx);
        let rigid = VecD::from_vec((0..4).flat_map(|_| [1.0, 2.0, 3.0]).collect());
        let f = ke.mul_vec(&rigid);
        for i in 0..12 {
            assert!(f.get(i).abs() < 1e-3, "rigid-body force[{i}] = {}", f.get(i));
        }
    }

    fn tet_volume(positions: &[[f64; 3]; 4]) -> f64 {
        let e1 = [positions[1][0] - positions[0][0], positions[1][1] - positions[0][1], positions[1][2] - positions[0][2]];
        let e2 = [positions[2][0] - positions[0][0], positions[2][1] - positions[0][1], positions[2][2] - positions[0][2]];
        let e3 = [positions[3][0] - positions[0][0], positions[3][1] - positions[0][1], positions[3][2] - positions[0][2]];
        let cross = [e1[1] * e2[2] - e1[2] * e2[1], e1[2] * e2[0] - e1[0] * e2[2], e1[0] * e2[1] - e1[1] * e2[0]];
        (cross[0] * e3[0] + cross[1] * e3[1] + cross[2] * e3[2]).abs() / 6.0
    }

    /// ⚖️ `Tet4::mass`'s total (the pure-`Tx` submatrix's sum) equals `ρV` — same partition-of-unity
    /// identity as `Bar3`'s.
    #[test]
    fn tet4_mass_total_equals_rho_v() {
        let (density, e, nu) = (2400.0, 200e9, 0.3);
        let positions = skew_tet_positions();
        let tet = Tet4 { id: "t1".into(), nodes: ["n0".into(), "n1".into(), "n2".into(), "n3".into()], e, nu, density };
        let ctx = ElementContext { positions: positions.to_vec() };
        let m = tet.mass(&ctx).expect("tet4 reports mass");
        let sum_tx: f64 = (0..4).flat_map(|r| (0..4).map(move |c| (3 * r, 3 * c))).map(|(r, c)| m.get(r, c)).sum();
        let expected = density * tet_volume(&positions);
        assert!((sum_tx - expected).abs() / expected < 1e-9, "sum={sum_tx} expected={expected}");
    }

    /// ⚖️ A single `Tet4` under self-weight only: the vertical reaction sum must equal `ρVg` — the
    /// same strong equilibrium check `analyses`'s beam self-weight test uses, now exercised on a
    /// continuum solid element (only possible once `Tet4::mass` exists).
    #[test]
    fn tet4_self_weight_matches_total_mass_times_gravity() {
        let (density, e, nu, g) = (2400.0, 30e9, 0.2, 9.81);
        let positions = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
        let nodes: Vec<Node> = (0..4).map(|i| Node { id: format!("n{i}"), pos: positions[i] }).collect();
        let model = crate::analyses::AnalysisModel {
            nodes,
            elements: vec![Box::new(Tet4 { id: "t1".into(), nodes: ["n0".into(), "n1".into(), "n2".into(), "n3".into()], e, nu, density })],
            supports: vec![Support { node_id: "n0".into(), fixed: vec![Dof::Tx, Dof::Ty, Dof::Tz] }, Support { node_id: "n1".into(), fixed: vec![Dof::Ty, Dof::Tz] }, Support { node_id: "n2".into(), fixed: vec![Dof::Tz] }],
        };
        let case = crate::analyses::LoadCase { id: "self_weight".into(), nodal_loads: vec![], member_loads: vec![], self_weight: true };
        let results = crate::analyses::solve_multi_case(&model, &[case], &[], [0.0, 0.0, -g]).expect("solves");
        let result = results.get("self_weight").unwrap();
        let total_tz_reaction: f64 = result.reactions.iter().filter(|r| r.dof == Dof::Tz).map(|r| r.value).sum();
        let expected = density * tet_volume(&positions) * g;
        assert!((total_tz_reaction - expected).abs() / expected < 1e-9, "reaction sum {total_tz_reaction} vs expected {expected}");
    }

    /// 🌀️ `Tet4::geometric_stiffness`: zero under rigid translation and symmetric.
    #[test]
    fn tet4_geometric_stiffness_rigid_translation_gives_zero_force_and_is_symmetric() {
        let (e, nu) = (200e9, 0.3);
        let positions = skew_tet_positions();
        let field = LinearField::sample();
        let ctx = ElementContext { positions: positions.to_vec() };
        let tet = Tet4 { id: "t1".into(), nodes: ["n0".into(), "n1".into(), "n2".into(), "n3".into()], e, nu, density: 0.0 };
        let u = field.nodal_vector(&positions);
        let kg = tet.geometric_stiffness(&ctx, &u).expect("tet4 reports geometric stiffness");
        for r in 0..12 {
            for c in 0..12 {
                assert!((kg.get(r, c) - kg.get(c, r)).abs() < 1e-6, "Kg not symmetric at ({r},{c})");
            }
        }
        let rigid = VecD::from_vec((0..4).flat_map(|_| [1.0, 2.0, 3.0]).collect());
        let f = kg.mul_vec(&rigid);
        for i in 0..12 {
            assert!(f.get(i).abs() < 1e-3, "rigid-body geometric force[{i}] = {}", f.get(i));
        }
    }
    // #endregion 🔖️Tet4

    // #region 🔖️Hex8
    fn skew_hex_positions() -> [[f64; 3]; 8] {
        [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.05, 1.0, 0.02], [-0.03, 0.98, 0.01], [0.02, 0.01, 1.0], [1.02, -0.01, 0.97], [0.99, 1.03, 1.05], [0.01, 1.0, 0.98]]
    }

    /// 🧮️ Constant-strain patch test, same field as `Tet4`'s — checked at all 8 Gauss points (a
    /// skewed-but-non-degenerate hex still reproduces an exact linear field everywhere, a fundamental
    /// isoparametric-interpolation property, not something specific to parallelepiped geometry).
    #[test]
    fn hex8_patch_test_recovers_exact_constant_stress() {
        let (e, nu) = (200e9, 0.3);
        let positions = skew_hex_positions();
        let field = LinearField::sample();
        let ctx = ElementContext { positions: positions.to_vec() };
        let nodes: [String; 8] = std::array::from_fn(|i| format!("n{i}"));
        let hex = Hex8 { id: "h1".into(), nodes, e, nu, density: 0.0 };
        let ke = hex.stiffness_global(&ctx);
        assert_eq!(ke.rows, 24);
        let u = field.nodal_vector(&positions);
        let result = hex.recover(&ctx, &u, None);
        let ElementResult::Solid { gauss } = result else { panic!("expected solid") };
        assert_eq!(gauss.len(), 8);
        let expected = field.expected_stress(e, nu);
        for g in &gauss {
            assert_stress_close(g, &expected, 1e-6);
        }
    }

    /// 🌀️ Rigid-body test: a pure translation of all 8 nodes must produce zero internal force.
    #[test]
    fn hex8_rigid_translation_gives_zero_force() {
        let (e, nu) = (200e9, 0.3);
        let positions = skew_hex_positions();
        let ctx = ElementContext { positions: positions.to_vec() };
        let nodes: [String; 8] = std::array::from_fn(|i| format!("n{i}"));
        let hex = Hex8 { id: "h1".into(), nodes, e, nu, density: 0.0 };
        let ke = hex.stiffness_global(&ctx);
        let rigid = VecD::from_vec((0..8).flat_map(|_| [1.0, 2.0, 3.0]).collect());
        let f = ke.mul_vec(&rigid);
        for i in 0..24 {
            assert!(f.get(i).abs() < 1e-3, "rigid-body force[{i}] = {}", f.get(i));
        }
    }

    /// 🏗️ Coarse hex-meshed cantilever (4 elements along the span) vs classical beam theory
    /// `δ = PL³/3EI` — a sanity check on assembly/BC wiring, not on element accuracy (low-order hex
    /// without incompatible modes is known to lock somewhat stiff in bending), so the tolerance is
    /// wide: just confirm the deflection is negative (toward the load), finite, and the right order
    /// of magnitude.
    #[test]
    fn hex8_meshed_cantilever_deflection_is_right_order_of_magnitude() {
        let (e, nu) = (200e9, 0.3);
        let (b, h, l, nx) = (0.2, 0.3, 4.0, 4usize);
        let dx = l / nx as f64;
        let corner_id = |ix: usize, iy: usize, iz: usize| format!("n{ix}_{iy}_{iz}");
        let corners = [(0usize, 0usize), (1, 0), (1, 1), (0, 1)];

        let mut nodes = Vec::new();
        for ix in 0..=nx {
            let x = dx * ix as f64;
            for &(iy, iz) in &corners {
                let y = if iy == 0 { 0.0 } else { b };
                let z = if iz == 0 { 0.0 } else { h };
                nodes.push(Node { id: corner_id(ix, iy, iz), pos: [x, y, z] });
            }
        }

        let mut elements: Vec<Box<dyn Element>> = Vec::new();
        for ix in 0..nx {
            elements.push(Box::new(Hex8 {
                id: format!("hex{ix}"),
                nodes: [corner_id(ix, 0, 0), corner_id(ix + 1, 0, 0), corner_id(ix + 1, 1, 0), corner_id(ix, 1, 0), corner_id(ix, 0, 1), corner_id(ix + 1, 0, 1), corner_id(ix + 1, 1, 1), corner_id(ix, 1, 1)],
                e,
                nu,
                density: 0.0,
            }));
        }

        let supports = corners.iter().map(|&(iy, iz)| Support { node_id: corner_id(0, iy, iz), fixed: vec![Dof::Tx, Dof::Ty, Dof::Tz] }).collect();
        let p_total = 1e4;
        let nodal_loads = corners.iter().map(|&(iy, iz)| NodalLoad { node_id: corner_id(nx, iy, iz), dof: Dof::Tz, value: -p_total / 4.0 }).collect();

        let model = Model { nodes, elements, supports, nodal_loads, member_loads: vec![] };
        let result = solve_linear_static(&model).expect("solves");

        let tip_dz: f64 = corners
            .iter()
            .map(|&(iy, iz)| {
                let id = corner_id(nx, iy, iz);
                result.displacements.iter().find(|d| d.node_id == id).unwrap().values[Dof::Tz.index()]
            })
            .sum::<f64>()
            / corners.len() as f64;

        let i_area = b * h.powi(3) / 12.0;
        let expected = p_total * l.powi(3) / (3.0 * e * i_area);
        assert!(tip_dz.is_finite());
        assert!(tip_dz < 0.0, "tip should deflect toward -Z, got {tip_dz}");
        let ratio = tip_dz.abs() / expected;
        assert!(ratio > 0.02 && ratio < 3.0, "deflection ratio {ratio} (actual {tip_dz} vs beam-theory {expected}) out of order-of-magnitude range");
    }

    /// ⚖️ `Hex8::mass`'s total (pure-`Tx` submatrix sum) equals `ρV` on the UNIT cube (skewed hex
    /// positions make an independent volume oracle fiddly — the axis-aligned unit cube's volume is
    /// trivially `1.0`, isolating the mass identity from any volume-computation risk).
    #[test]
    fn hex8_mass_total_equals_rho_v() {
        let density = 2400.0;
        let positions = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 1.0], [1.0, 1.0, 1.0], [0.0, 1.0, 1.0]];
        let nodes: [String; 8] = std::array::from_fn(|i| format!("n{i}"));
        let hex = Hex8 { id: "h1".into(), nodes, e: 200e9, nu: 0.3, density };
        let ctx = ElementContext { positions: positions.to_vec() };
        let m = hex.mass(&ctx).expect("hex8 reports mass");
        let sum_tx: f64 = (0..8).flat_map(|r| (0..8).map(move |c| (3 * r, 3 * c))).map(|(r, c)| m.get(r, c)).sum();
        assert!((sum_tx - density).abs() / density < 1e-9, "sum={sum_tx} expected={density}");
    }

    /// 🌀️ `Hex8::geometric_stiffness`: zero under rigid translation and symmetric.
    #[test]
    fn hex8_geometric_stiffness_rigid_translation_gives_zero_force_and_is_symmetric() {
        let (e, nu) = (200e9, 0.3);
        let positions = skew_hex_positions();
        let field = LinearField::sample();
        let ctx = ElementContext { positions: positions.to_vec() };
        let nodes: [String; 8] = std::array::from_fn(|i| format!("n{i}"));
        let hex = Hex8 { id: "h1".into(), nodes, e, nu, density: 0.0 };
        let u = field.nodal_vector(&positions);
        let kg = hex.geometric_stiffness(&ctx, &u).expect("hex8 reports geometric stiffness");
        for r in 0..24 {
            for c in 0..24 {
                assert!((kg.get(r, c) - kg.get(c, r)).abs() < 1e-6, "Kg not symmetric at ({r},{c})");
            }
        }
        let rigid = VecD::from_vec((0..8).flat_map(|_| [1.0, 2.0, 3.0]).collect());
        let f = kg.mul_vec(&rigid);
        for i in 0..24 {
            assert!(f.get(i).abs() < 1e-3, "rigid-body geometric force[{i}] = {}", f.get(i));
        }
    }
    // #endregion 🔖️Hex8
}
// #endregion 🔖️SolidTests

// #region 🔖️ShellTests
#[cfg(test)]
mod shell_tests {
    use super::*;
    use crate::model::{solve_linear_static, Model, NodalLoad, Node, Support};

    const E: f64 = 1000.0;
    const NU: f64 = 0.25;
    const THICKNESS: f64 = 1.0;

    // Same small-magnitude membrane field as `elements2d::continuum_tests` (keeps expected forces
    // O(1) relative to the absolute patch-test tolerance) plus a small constant-curvature bending field.
    const MU: (f64, f64, f64) = (0.01, 0.003, 0.0021);
    const MV: (f64, f64, f64) = (-0.02, 0.0012, 0.0027);
    const KX: f64 = 0.004;
    const KY: f64 = -0.0025;
    const KXY: f64 = 0.0017;

    /// 📐️ A triangle placed so `p1-p0` lies exactly on global X and `p2` has `z=0` — the local shell
    /// frame (`local_x=normalize(p1-p0)`, `local_z=facet normal`) then coincides EXACTLY with global
    /// `(X,Y,Z)`, so local == global coordinates and the combined membrane+bending patch-test input
    /// can be built directly in global `(x,y)` without any local-frame bookkeeping.
    fn aligned_triangle_positions() -> [[f64; 3]; 3] {
        [[0.0, 0.0, 0.0], [2.0, 0.0, 0.0], [0.2, 1.8, 0.0]]
    }

    #[test]
    fn shell_facet3_patch_test_reproduces_linear_membrane_and_constant_curvature() {
        let positions = aligned_triangle_positions();
        let el = ShellFacet3 { id: "s".into(), nodes: ["a".into(), "b".into(), "c".into()], e: E, nu: NU, thickness: THICKNESS, density: 0.0 };
        let ctx = ElementContext { positions: positions.to_vec() };

        let mut u = Vec::with_capacity(18);
        for &[x, y, _] in &positions {
            u.push(MU.0 + MU.1 * x + MU.2 * y); // Tx
            u.push(MV.0 + MV.1 * x + MV.2 * y); // Ty
            u.push(0.5 * (KX * x * x + KY * y * y + 2.0 * KXY * x * y)); // Tz = w
            u.push(KY * y + KXY * x); // Rx = ∂w/∂y
            u.push(-(KX * x + KXY * y)); // Ry = -∂w/∂x
            u.push(0.0); // Rz (drilling) stays zero — not excited by this field
        }
        let u = VecD::from_vec(u);

        let ElementResult::Shell { gauss } = el.recover(&ctx, &u, None) else { panic!("expected shell result") };
        assert_eq!(gauss.len(), 1);
        let state = &gauss[0];

        let d_mem = d_matrix_plane_stress(E, NU);
        let strain = VecD::from_vec(vec![MU.1, MV.2, MU.2 + MV.1]);
        let sigma = d_mem.mul_vec(&strain);
        let (expected_nxx, expected_nyy, expected_nxy) = (sigma.get(0) * THICKNESS, sigma.get(1) * THICKNESS, sigma.get(2) * THICKNESS);

        let d_bend = crate::elements2d::d_matrix_plate(E, NU, THICKNESS);
        let kappa = VecD::from_vec(vec![KX, KY, 2.0 * KXY]);
        let m = d_bend.mul_vec(&kappa);

        let mem_scale = expected_nxx.abs().max(expected_nyy.abs()).max(expected_nxy.abs()).max(1.0);
        assert!((state.nxx - expected_nxx).abs() / mem_scale < 1e-6, "nxx {} vs {}", state.nxx, expected_nxx);
        assert!((state.nyy - expected_nyy).abs() / mem_scale < 1e-6, "nyy {} vs {}", state.nyy, expected_nyy);
        assert!((state.nxy - expected_nxy).abs() / mem_scale < 1e-6, "nxy {} vs {}", state.nxy, expected_nxy);

        let bend_scale = m.get(0).abs().max(m.get(1).abs()).max(m.get(2).abs()).max(1.0);
        assert!((state.mxx - m.get(0)).abs() / bend_scale < 1e-4, "mxx {} vs {}", state.mxx, m.get(0));
        assert!((state.myy - m.get(1)).abs() / bend_scale < 1e-4, "myy {} vs {}", state.myy, m.get(1));
        assert!((state.mxy - m.get(2)).abs() / bend_scale < 1e-4, "mxy {} vs {}", state.mxy, m.get(2));
    }

    /// 🌀️ Rigid-body test: a pure 3D translation (zero rotation, so the drilling DOF is untouched too)
    /// must produce zero internal force on a generic (non-axis-aligned) skew triangle.
    #[test]
    fn shell_facet3_rigid_translation_gives_zero_force() {
        let positions = [[0.0, 0.0, 0.0], [2.0, 0.0, 0.3], [0.5, 1.5, 0.7]];
        let el = ShellFacet3 { id: "s".into(), nodes: ["a".into(), "b".into(), "c".into()], e: E, nu: NU, thickness: THICKNESS, density: 0.0 };
        let ctx = ElementContext { positions: positions.to_vec() };
        let ke = el.stiffness_global(&ctx);
        let mut rigid = Vec::with_capacity(18);
        for _ in 0..3 {
            rigid.extend_from_slice(&[1.2, -0.8, 0.5, 0.0, 0.0, 0.0]);
        }
        let rigid = VecD::from_vec(rigid);
        let f = ke.mul_vec(&rigid);
        for i in 0..18 {
            assert!(f.get(i).abs() < 1e-6, "rigid-body force[{i}] = {}", f.get(i));
        }
    }

    /// 🏗️ Smoke test: a single flat `ShellFacet3` with one full edge fixed, loaded out-of-plane at the
    /// free node — deflection must be finite, nonzero, and in the same direction as the applied load
    /// (not a precision benchmark, just a physical-sanity check on assembly/BC wiring).
    #[test]
    fn shell_facet3_cantilever_deflects_toward_tip_load() {
        let (e, nu, t) = (200e9, 0.3, 0.01);
        let p = -1000.0;
        let model = Model {
            nodes: vec![Node { id: "a".into(), pos: [0.0, 0.0, 0.0] }, Node { id: "b".into(), pos: [1.0, 0.0, 0.0] }, Node { id: "c".into(), pos: [0.0, 1.0, 0.0] }],
            elements: vec![Box::new(ShellFacet3 { id: "s".into(), nodes: ["a".into(), "b".into(), "c".into()], e, nu, thickness: t, density: 0.0 })],
            supports: vec![Support { node_id: "a".into(), fixed: vec![Dof::Tx, Dof::Ty, Dof::Tz, Dof::Rx, Dof::Ry, Dof::Rz] }, Support { node_id: "b".into(), fixed: vec![Dof::Tx, Dof::Ty, Dof::Tz, Dof::Rx, Dof::Ry, Dof::Rz] }],
            nodal_loads: vec![NodalLoad { node_id: "c".into(), dof: Dof::Tz, value: p }],
            member_loads: vec![],
        };
        let result = solve_linear_static(&model).expect("cantilevered shell facet solves");
        let c = result.displacements.iter().find(|d| d.node_id == "c").unwrap();
        let dz = c.values[Dof::Tz.index()];
        assert!(dz.is_finite() && dz < 0.0, "tip deflection {dz} should be finite and negative (toward the -Tz load)");
    }

    /// ⚖️ `ShellFacet3::mass`'s total (pure-`Tx` submatrix sum) equals `ρtA` — same lumped-mass
    /// row-sum identity `PlateDkt`'s translational lump satisfies.
    #[test]
    fn shell_facet3_mass_total_equals_rho_t_area() {
        let (density, thickness) = (7850.0, 0.008);
        let positions = aligned_triangle_positions();
        let el = ShellFacet3 { id: "s".into(), nodes: ["a".into(), "b".into(), "c".into()], e: E, nu: NU, thickness, density };
        let ctx = ElementContext { positions: positions.to_vec() };
        let m = el.mass(&ctx).expect("shell facet reports mass");
        let sum_tx: f64 = (0..3).flat_map(|r| (0..3).map(move |c| (6 * r, 6 * c))).map(|(r, c)| m.get(r, c)).sum();
        // `aligned_triangle_positions` is `[[0,0,0],[2,0,0],[0.2,1.8,0]]` — shoelace area directly.
        let area = 0.5 * ((positions[1][0] - positions[0][0]) * (positions[2][1] - positions[0][1]) - (positions[2][0] - positions[0][0]) * (positions[1][1] - positions[0][1])).abs();
        let expected = density * thickness * area;
        assert!((sum_tx - expected).abs() / expected < 1e-9, "sum={sum_tx} expected={expected}");
    }

    /// 🌀️ A cantilevered flat shell panel (2 `ShellFacet3` triangles, one edge fully fixed) under
    /// in-plane axial COMPRESSION at the free edge must produce a finite, positive lowest linear-
    /// buckling load factor — possible only now that `ShellFacet3::geometric_stiffness` exists (a
    /// `PlateDkt`-only panel would report no geometric stiffness at all, per its documented `None`).
    #[test]
    fn shell_facet3_membrane_compression_destabilizes_and_tension_stabilizes_out_of_plane_stiffness() {
        let positions = aligned_triangle_positions();
        let el = ShellFacet3 { id: "s".into(), nodes: ["a".into(), "b".into(), "c".into()], e: E, nu: NU, thickness: THICKNESS, density: 0.0 };
        let ctx = ElementContext { positions: positions.to_vec() };

        // Uniform uniaxial membrane strain `u = k*x` (zero elsewhere) recovers a constant `Nxx`,
        // compressive for k<0 and tensile for k>0 — same field shape `elements2d::continuum_tests`
        // uses for its patch tests.
        let field = |k: f64| {
            let mut u = Vec::with_capacity(18);
            for &[x, _, _] in &positions {
                u.extend_from_slice(&[k * x, 0.0, 0.0, 0.0, 0.0, 0.0]);
            }
            VecD::from_vec(u)
        };
        let kg_tension = el.geometric_stiffness(&ctx, &field(1e-4)).expect("shell reports geometric stiffness");
        let kg_compression = el.geometric_stiffness(&ctx, &field(-1e-4)).expect("shell reports geometric stiffness");

        // Node `b`'s local `Tz` sits at global index 8 (node 1 * 6 dof + 2) — the aligned-triangle
        // fixture makes local == global, so this global diagonal entry is directly the out-of-plane
        // stiffness contribution the buckling solver would add for node b.
        let tz_b = 8usize;
        assert!(kg_tension.get(tz_b, tz_b) > 0.0, "tension should STIFFEN out-of-plane bending, got Kg[b,Tz]={}", kg_tension.get(tz_b, tz_b));
        assert!(kg_compression.get(tz_b, tz_b) < 0.0, "compression should DESTABILIZE out-of-plane bending, got Kg[b,Tz]={}", kg_compression.get(tz_b, tz_b));

        for r in 0..18 {
            for c in 0..18 {
                assert!((kg_compression.get(r, c) - kg_compression.get(c, r)).abs() < 1e-9, "Kg not symmetric at ({r},{c})");
            }
        }
    }
}
// #endregion 🔖️ShellTests
