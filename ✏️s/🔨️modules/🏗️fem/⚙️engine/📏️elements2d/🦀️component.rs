//! 📐️ 2D structural elements: axial `Bar2` truss, Euler-Bernoulli `BeamEb2` frame member, the
//! Tri3/Tri6/Quad4/Quad8 plane-stress/plane-strain continuum family, and the `PlateDkt` Batoz
//! Discrete Kirchhoff Triangle thin-plate bending element.

use crate::formulation::{b_matrix_plane, d_matrix_plane_strain, d_matrix_plane_stress, gauss_quad, gauss_tri, jacobian_2d, shape_quad4, shape_quad8, shape_tri3, shape_tri6};
use crate::model::{Dof, Element, ElementContext, ElementResult, Elements, MemberUdl, PlaneStress, PlateMoments};
use crate::algebra::{MatD, VecD};

// #region 🔖️Geometry
async fn segment_geometry(ctx: &ElementContext) -> (f64, f64, f64) {
    let p1 = ctx.positions[0];
    let p2 = ctx.positions[1];
    let dx = p2[0] - p1[0];
    let dy = p2[1] - p1[1];
    let l = (dx * dx + dy * dy).sqrt();
    (l, dx / l, dy / l)
}
// #endregion 🔖️Geometry

// #region 🔖️Bar2
/// 🪢️ 2-node axial truss element — DOFs `[Tx, Ty]` per node.
pub struct Bar2 {
    pub id: String,
    pub start: String,
    pub end: String,
    pub e: f64,
    pub area: f64,
    pub density: f64,
}

impl Element for Bar2 {
    async fn id(&self) -> &str {
        &self.id
    }

    async fn node_ids(&self) -> Vec<String> {
        vec![self.start.clone(), self.end.clone()]
    }

    async fn dofs_per_node(&self) -> &[Dof] {
        &[Dof::Tx, Dof::Ty]
    }

    async fn stiffness_global(&self, ctx: &ElementContext) -> MatD {
        let (l, cx, cy) = segment_geometry(ctx);
        let k = self.e * self.area / l;
        let mut m = MatD::zeros(4, 4);
        let terms = [[cx * cx, cx * cy, -cx * cx, -cx * cy], [cx * cy, cy * cy, -cx * cy, -cy * cy]];
        for row in 0..2 {
            for col in 0..4 {
                m.set(row, col, k * terms[row][col]);
                m.set(row + 2, col, if col < 2 { -k * terms[row][col] } else { k * terms[row][col - 2] });
            }
        }
        m
    }

    async fn recover(&self, ctx: &ElementContext, u_local: &VecD, _udl: Option<&MemberUdl>) -> ElementResult {
        let (l, cx, cy) = segment_geometry(ctx);
        let k = self.e * self.area / l;
        let n = k * ((u_local.get(2) - u_local.get(0)) * cx + (u_local.get(3) - u_local.get(1)) * cy);
        ElementResult::Bar { n }
    }

    /// 🏋️ Isotropic lumped-consistent mass — same in both directions since a bar has no bending
    /// stiffness to give mass a preferred orientation. `m = ρAL/6`, block form `[[2m,0,m,0],[0,2m,0,m],
    /// [m,0,2m,0],[0,m,0,2m]]` (node-major `[u1,v1,u2,v2]`).
    async fn mass(&self, ctx: &ElementContext) -> Option<MatD> {
        let (l, _, _) = segment_geometry(ctx);
        let m = self.density * self.area * l / 6.0;
        let mut out = MatD::zeros(4, 4);
        for i in 0..4 {
            out.set(i, i, 2.0 * m);
        }
        out.set(0, 2, m);
        out.set(2, 0, m);
        out.set(1, 3, m);
        out.set(3, 1, m);
        Some(out)
    }

    /// 🌬️ Consistent end-load `wL/2` at each node from a global member UDL `(wx,wy)` — a 2-node
    /// linear axial element has no bending stiffness to redistribute the load unevenly, so the
    /// lumped-consistent split is exact.
    async fn equivalent_nodal_loads(&self, ctx: &ElementContext, udl: &MemberUdl) -> Option<VecD> {
        let (l, _, _) = segment_geometry(ctx);
        let half = l / 2.0;
        Some(VecD::from_vec(vec![udl.wx * half, udl.wy * half, udl.wx * half, udl.wy * half]))
    }

    /// 🌀️ Truss geometric ("stability") stiffness under the member's own axial force `n` (tension-
    /// positive, same convention as `recover`): `N/L·(I − ccᵀ)` on each 2x2 node block, `ccᵀ` the
    /// outer product of the unit axial direction — the transverse-projector form (Przemieniecki,
    /// "Theory of Matrix Structural Analysis") that only destabilizes displacement PERPENDICULAR to
    /// the bar's own axis, vanishing identically for a rigid translation (which the projector kills).
    async fn geometric_stiffness(&self, ctx: &ElementContext, u_element: &VecD) -> Option<MatD> {
        let (l, cx, cy) = segment_geometry(ctx);
        let k = self.e * self.area / l;
        let n = k * ((u_element.get(2) - u_element.get(0)) * cx + (u_element.get(3) - u_element.get(1)) * cy);
        let coeff = n / l;
        let proj = [[1.0 - cx * cx, -cx * cy], [-cx * cy, 1.0 - cy * cy]];
        let mut kg = MatD::zeros(4, 4);
        for row in 0..2 {
            for col in 0..2 {
                let v = coeff * proj[row][col];
                kg.set(row, col, v);
                kg.set(row, col + 2, -v);
                kg.set(row + 2, col, -v);
                kg.set(row + 2, col + 2, v);
            }
        }
        Some(kg)
    }
}
// #endregion 🔖️Bar2

// #region 🔖️BeamEb2
/// 🧭️ 2D frame transformation matrix — block-diagonal 3 copies of the planar rotation, mapping
/// GLOBAL `[u1,v1,θ1,u2,v2,θ2]` to LOCAL coordinates.
async fn beam_transform(c: f64, s: f64) -> MatD {
    let mut t = MatD::zeros(6, 6);
    for block in 0..2 {
        let o = block * 3;
        t.set(o, o, c);
        t.set(o, o + 1, s);
        t.set(o + 1, o, -s);
        t.set(o + 1, o + 1, c);
        t.set(o + 2, o + 2, 1.0);
    }
    t
}

/// 🧮️ Local 6x6 Euler-Bernoulli beam stiffness, dof order `[u1,v1,θ1,u2,v2,θ2]`.
async fn beam_local_stiffness(l: f64, axial_k: f64, bend_k: f64) -> MatD {
    let mut k = MatD::zeros(6, 6);
    k.set(0, 0, axial_k);
    k.set(0, 3, -axial_k);
    k.set(3, 0, -axial_k);
    k.set(3, 3, axial_k);

    let l2 = l * l;
    let bending = [
        (1, 1, 12.0 * bend_k / l2),
        (1, 2, 6.0 * bend_k / l),
        (1, 4, -12.0 * bend_k / l2),
        (1, 5, 6.0 * bend_k / l),
        (2, 1, 6.0 * bend_k / l),
        (2, 2, 4.0 * bend_k),
        (2, 4, -6.0 * bend_k / l),
        (2, 5, 2.0 * bend_k),
        (4, 1, -12.0 * bend_k / l2),
        (4, 2, -6.0 * bend_k / l),
        (4, 4, 12.0 * bend_k / l2),
        (4, 5, -6.0 * bend_k / l),
        (5, 1, 6.0 * bend_k / l),
        (5, 2, 2.0 * bend_k),
        (5, 4, -6.0 * bend_k / l),
        (5, 5, 4.0 * bend_k),
    ];
    for (row, col, value) in bending {
        k.set(row, col, value);
    }
    k
}

/// 🌬️ Local fixed-end load vector `[u1,v1,θ1,u2,v2,θ2]` for a local-frame UDL `(wx_local, wy_local)`.
async fn beam_local_udl(l: f64, wx_local: f64, wy_local: f64) -> VecD {
    VecD::from_vec(vec![wx_local * l / 2.0, wy_local * l / 2.0, wy_local * l * l / 12.0, wx_local * l / 2.0, wy_local * l / 2.0, -wy_local * l * l / 12.0])
}

/// 🏋️ Consistent local mass matrix, dof order `[u1,v1,θ1,u2,v2,θ2]` — axial `ρAL/6*[[2,1],[1,2]]` at
/// `(0,3)`, standard Euler-Bernoulli consistent bending mass at `[1,2,4,5]` (rotary inertia of the
/// cross-section neglected — see Cook/Malkus/Plesha "Concepts and Applications of Finite Element
/// Analysis" for the closed form).
async fn beam_local_mass(l: f64, area: f64, density: f64) -> MatD {
    let mut m = MatD::zeros(6, 6);
    let axial = density * area * l / 6.0;
    m.set(0, 0, 2.0 * axial);
    m.set(0, 3, axial);
    m.set(3, 0, axial);
    m.set(3, 3, 2.0 * axial);

    let l2 = l * l;
    let factor = density * area * l / 420.0;
    let idx = [1usize, 2, 4, 5];
    let block = [[156.0, 22.0 * l, 54.0, -13.0 * l], [22.0 * l, 4.0 * l2, 13.0 * l, -3.0 * l2], [54.0, 13.0 * l, 156.0, -22.0 * l], [-13.0 * l, -3.0 * l2, -22.0 * l, 4.0 * l2]];
    for (bi, &gi) in idx.iter().enumerate() {
        for (bj, &gj) in idx.iter().enumerate() {
            m.set(gi, gj, factor * block[bi][bj]);
        }
    }
    m
}

/// 🌀️ Local geometric ("stress") stiffness for a 2D Euler-Bernoulli beam-column under axial force `n`
/// (tension-positive, same convention `recover` reports), bending block `[v1,θ1,v2,θ2]` only — no
/// axial/geometric coupling at this scope. Standard textbook beam-column geometric stiffness.
async fn beam_local_geometric_stiffness(l: f64, n: f64) -> MatD {
    let mut kg = MatD::zeros(6, 6);
    let l2 = l * l;
    let coeff = n / l;
    let idx = [1usize, 2, 4, 5];
    let block = [[6.0 / 5.0, l / 10.0, -6.0 / 5.0, l / 10.0], [l / 10.0, 2.0 * l2 / 15.0, -l / 10.0, -l2 / 30.0], [-6.0 / 5.0, -l / 10.0, 6.0 / 5.0, -l / 10.0], [l / 10.0, -l2 / 30.0, -l / 10.0, 2.0 * l2 / 15.0]];
    for (bi, &gi) in idx.iter().enumerate() {
        for (bj, &gj) in idx.iter().enumerate() {
            kg.set(gi, gj, coeff * block[bi][bj]);
        }
    }
    kg
}

/// 🏗️ 2-node Euler-Bernoulli frame element — DOFs `[Tx, Ty, Rz]` per node.
pub struct BeamEb2 {
    pub id: String,
    pub start: String,
    pub end: String,
    pub e: f64,
    pub area: f64,
    pub iy: f64,
    pub density: f64,
}

impl Element for BeamEb2 {
    async fn id(&self) -> &str {
        &self.id
    }

    async fn node_ids(&self) -> Vec<String> {
        vec![self.start.clone(), self.end.clone()]
    }

    async fn dofs_per_node(&self) -> &[Dof] {
        &[Dof::Tx, Dof::Ty, Dof::Rz]
    }

    async fn stiffness_global(&self, ctx: &ElementContext) -> MatD {
        let (l, c, s) = segment_geometry(ctx);
        let axial_k = self.e * self.area / l;
        let bend_k = self.e * self.iy / l;
        let k_local = beam_local_stiffness(l, axial_k, bend_k);
        let t = beam_transform(c, s);
        t.transpose().matmul(&k_local).matmul(&t)
    }

    async fn equivalent_nodal_loads(&self, ctx: &ElementContext, udl: &MemberUdl) -> Option<VecD> {
        let (l, c, s) = segment_geometry(ctx);
        let wx_local = udl.wx * c + udl.wy * s;
        let wy_local = -udl.wx * s + udl.wy * c;
        let f_local = beam_local_udl(l, wx_local, wy_local);
        let t = beam_transform(c, s);
        Some(t.transpose().mul_vec(&f_local))
    }

    async fn recover(&self, ctx: &ElementContext, u_local: &VecD, udl: Option<&MemberUdl>) -> ElementResult {
        let (l, c, s) = segment_geometry(ctx);
        let axial_k = self.e * self.area / l;
        let bend_k = self.e * self.iy / l;
        let t = beam_transform(c, s);
        let u_loc = t.mul_vec(u_local);
        let k_local = beam_local_stiffness(l, axial_k, bend_k);

        let (wx_local, wy_local) = match udl {
            Some(u) => (u.wx * c + u.wy * s, -u.wx * s + u.wy * c),
            None => (0.0, 0.0),
        };
        let f_udl_local = beam_local_udl(l, wx_local, wy_local);
        let f_end = k_local.mul_vec(&u_loc).sub(&f_udl_local);

        let n1 = f_end.get(0);
        let v1 = f_end.get(1);
        let m1 = f_end.get(2);

        let stations = (0..11)
            .map(|i| {
                let x = l * (i as f64) / 10.0;
                crate::model::BeamStation { x, n: -n1, v: v1 + wy_local * x, m: m1 + v1 * x + wy_local * x * x / 2.0 }
            })
            .collect();
        ElementResult::Beam { stations }
    }

    async fn mass(&self, ctx: &ElementContext) -> Option<MatD> {
        let (l, c, s) = segment_geometry(ctx);
        let m_local = beam_local_mass(l, self.area, self.density);
        let t = beam_transform(c, s);
        Some(t.transpose().matmul(&m_local).matmul(&t))
    }

    /// 🌀️ Buckling geometric stiffness from the member's own axial force under `u_element` — same
    /// sign convention as `recover`'s `n` (tension-positive): `n = -k_local.mul_vec(u_loc).get(0)`.
    async fn geometric_stiffness(&self, ctx: &ElementContext, u_element: &VecD) -> Option<MatD> {
        let (l, c, s) = segment_geometry(ctx);
        let axial_k = self.e * self.area / l;
        let bend_k = self.e * self.iy / l;
        let t = beam_transform(c, s);
        let u_loc = t.mul_vec(u_element);
        let k_local = beam_local_stiffness(l, axial_k, bend_k);
        let f_end = k_local.mul_vec(&u_loc);
        let n = -f_end.get(0);
        let kg_local = beam_local_geometric_stiffness(l, n);
        Some(t.transpose().matmul(&kg_local).matmul(&t))
    }
}
// #endregion 🔖️BeamEb2

// #region 🔖️Continuum
/// 🧱️ Plane-stress vs plane-strain constitutive assumption, shared by the Tri3/Tri6/Quad4/Quad8
/// continuum elements.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlaneKind {
    Stress,
    Strain,
}

impl PlaneKind {
    async fn d_matrix(self, e: f64, nu: f64) -> MatD {
        match self {
            PlaneKind::Stress => d_matrix_plane_stress(e, nu),
            PlaneKind::Strain => d_matrix_plane_strain(e, nu),
        }
    }
}

async fn plane_coords(ctx: &ElementContext) -> Vec<[f64; 2]> {
    ctx.positions.iter().map(|p| [p[0], p[1]]).collect()
}

/// 🧮️ Physical B-matrix + `weight * det(J)` at every Gauss point of a rule, shared by
/// `stiffness_global` and `recover` so both walk the SAME Gauss points in the SAME order.
async fn plane_b_and_weights(coords: &[[f64; 2]], rule: &[(f64, f64, f64)], shape: impl Fn(f64, f64) -> Vec<[f64; 2]>) -> Vec<(MatD, f64)> {
    rule.iter()
        .map(|&(xi, eta, w)| {
            let d_n_param = shape(xi, eta);
            let (_, det_j, d_n_xy) = jacobian_2d(coords, &d_n_param);
            (b_matrix_plane(&d_n_xy), w * det_j)
        })
        .collect()
}

async fn plane_stiffness(coords: &[[f64; 2]], rule: &[(f64, f64, f64)], shape: impl Fn(f64, f64) -> Vec<[f64; 2]>, d: &MatD, thickness: f64, ndof: usize) -> MatD {
    let mut ke = MatD::zeros(ndof, ndof);
    for (b, w) in plane_b_and_weights(coords, rule, shape) {
        ke.add_triple_product(&b, d, w * thickness);
    }
    ke
}

async fn plane_recover(coords: &[[f64; 2]], rule: &[(f64, f64, f64)], shape: impl Fn(f64, f64) -> Vec<[f64; 2]>, d: &MatD, u_local: &VecD) -> ElementResult {
    let gauss = plane_b_and_weights(coords, rule, shape)
        .into_iter()
        .map(|(b, _)| {
            let eps = b.mul_vec(u_local);
            let sigma = d.mul_vec(&eps);
            let (sxx, syy, sxy) = (sigma.get(0), sigma.get(1), sigma.get(2));
            let von_mises = (sxx * sxx - sxx * syy + syy * syy + 3.0 * sxy * sxy).sqrt();
            PlaneStress { sxx, syy, sxy, von_mises }
        })
        .collect();
    ElementResult::Plane { gauss }
}

/// 🏋️ Consistent plane-continuum mass `ρ·t·∫Nᵀ·N·dA`, evaluated at the SAME Gauss rule as
/// `plane_stiffness` — `shape_full` returns BOTH shape values (for `Nᵀ·N`) and parametric
/// derivatives (for `jacobian_2d`'s `det(J)`), unlike `plane_b_and_weights`'s gradient-only closure.
async fn plane_mass(coords: &[[f64; 2]], rule: &[(f64, f64, f64)], shape_full: impl Fn(f64, f64) -> (Vec<f64>, Vec<[f64; 2]>), density: f64, thickness: f64, n_nodes: usize) -> MatD {
    let mut m = MatD::zeros(n_nodes * 2, n_nodes * 2);
    for (xi, eta, w) in rule.iter().copied() {
        let (n_vals, d_n_param) = shape_full(xi, eta);
        let (_, det_j, _) = jacobian_2d(coords, &d_n_param);
        let scale = density * thickness * w * det_j;
        for i in 0..n_nodes {
            for j in 0..n_nodes {
                let v = n_vals[i] * n_vals[j] * scale;
                m.add_at(2 * i, 2 * j, v);
                m.add_at(2 * i + 1, 2 * j + 1, v);
            }
        }
    }
    m
}

/// 🌀️ Plane-continuum initial-stress geometric stiffness `Kg = ∫Gᵀ(σ⊗I₂)G·t·dA` (Cook, Malkus,
/// Plesha & Witt, "Concepts and Applications of Finite Element Analysis") — recovers the Cauchy
/// stress `σ=Dε` from `u_local` at each Gauss point, then couples node `i`/`j`'s shape gradients
/// through `σ` identically in BOTH the `u` and `v` directions (no `u`-`v` cross-coupling, since `G`
/// is block-diagonal by direction).
async fn plane_geometric_stiffness(coords: &[[f64; 2]], rule: &[(f64, f64, f64)], shape: impl Fn(f64, f64) -> Vec<[f64; 2]>, d: &MatD, thickness: f64, u_local: &VecD, n_nodes: usize) -> MatD {
    let mut kg = MatD::zeros(n_nodes * 2, n_nodes * 2);
    for (xi, eta, w) in rule.iter().copied() {
        let d_n_param = shape(xi, eta);
        let (_, det_j, d_n_xy) = jacobian_2d(coords, &d_n_param);
        let b = b_matrix_plane(&d_n_xy);
        let eps = b.mul_vec(u_local);
        let sigma = d.mul_vec(&eps);
        let (sxx, syy, sxy) = (sigma.get(0), sigma.get(1), sigma.get(2));
        let scale = w * det_j * thickness;
        for i in 0..n_nodes {
            let (dix, diy) = (d_n_xy[i][0], d_n_xy[i][1]);
            for j in 0..n_nodes {
                let (djx, djy) = (d_n_xy[j][0], d_n_xy[j][1]);
                let s = dix * sxx * djx + dix * sxy * djy + diy * sxy * djx + diy * syy * djy;
                kg.add_at(2 * i, 2 * j, s * scale);
                kg.add_at(2 * i + 1, 2 * j + 1, s * scale);
            }
        }
    }
    kg
}

// #region 🔖️Tri3Cst
/// 🔺️ 3-node constant-strain triangle — DOFs `[Tx, Ty]` per node, 1-point Gauss-tri integration
/// (exact for constant strain).
pub struct Tri3Cst {
    pub id: String,
    pub nodes: [String; 3],
    pub e: f64,
    pub nu: f64,
    pub thickness: f64,
    pub kind: PlaneKind,
    pub density: f64,
}

impl Tri3Cst {
    async fn rule(&self) -> Vec<(f64, f64, f64)> {
        gauss_tri(1)
    }

    async fn shape(xi: f64, eta: f64) -> Vec<[f64; 2]> {
        shape_tri3(xi, eta).1.to_vec()
    }

    async fn shape_full(xi: f64, eta: f64) -> (Vec<f64>, Vec<[f64; 2]>) {
        let (n, dn) = shape_tri3(xi, eta);
        (n.to_vec(), dn.to_vec())
    }
}

impl Element for Tri3Cst {
    async fn id(&self) -> &str {
        &self.id
    }

    async fn node_ids(&self) -> Vec<String> {
        self.nodes.to_vec()
    }

    async fn dofs_per_node(&self) -> &[Dof] {
        &[Dof::Tx, Dof::Ty]
    }

    async fn stiffness_global(&self, ctx: &ElementContext) -> MatD {
        let coords = plane_coords(ctx);
        let d = self.kind.d_matrix(self.e, self.nu);
        plane_stiffness(&coords, &self.rule(), Self::shape, &d, self.thickness, 6)
    }

    async fn recover(&self, ctx: &ElementContext, u_local: &VecD, _udl: Option<&MemberUdl>) -> ElementResult {
        let coords = plane_coords(ctx);
        let d = self.kind.d_matrix(self.e, self.nu);
        plane_recover(&coords, &self.rule(), Self::shape, &d, u_local)
    }

    /// 🏋️ Consistent CST mass `ρtA/12·[[2,1,1],[1,2,1],[1,1,2]]` (both directions) — Tri3's shape
    /// functions ARE the area coordinates (`Ni=Li`), so `Ni·Nj` is a complete quadratic in area
    /// coordinates, integrated EXACTLY by the degree-2-precision 3-point rule (own stiffness rule
    /// `self.rule()` is only 1-point, adequate for the constant-strain stiffness but NOT exact here).
    async fn mass(&self, ctx: &ElementContext) -> Option<MatD> {
        let coords = plane_coords(ctx);
        Some(plane_mass(&coords, &gauss_tri(3), Self::shape_full, self.density, self.thickness, 3))
    }

    async fn geometric_stiffness(&self, ctx: &ElementContext, u_element: &VecD) -> Option<MatD> {
        let coords = plane_coords(ctx);
        let d = self.kind.d_matrix(self.e, self.nu);
        Some(plane_geometric_stiffness(&coords, &self.rule(), Self::shape, &d, self.thickness, u_element, 3))
    }
}
// #endregion 🔖️Tri3Cst

// #region 🔖️Tri6Lst
/// 🔺️ 6-node linear-strain triangle — DOFs `[Tx, Ty]` per node, 3-point Gauss-tri integration.
/// Node order `[n0,n1,n2,n01,n12,n20]` — see `formulation::shape_tri6` for the exact convention.
pub struct Tri6Lst {
    pub id: String,
    pub nodes: [String; 6],
    pub e: f64,
    pub nu: f64,
    pub thickness: f64,
    pub kind: PlaneKind,
    pub density: f64,
}

impl Tri6Lst {
    async fn rule(&self) -> Vec<(f64, f64, f64)> {
        gauss_tri(3)
    }

    /// 🎯️ A 7-point rule (degree-5 precision) for mass — Tri6's quadratic shape functions make
    /// `Ni·Nj` a degree-4 polynomial, which the element's own 3-point (degree-2) stiffness rule
    /// under-integrates.
    async fn mass_rule() -> Vec<(f64, f64, f64)> {
        gauss_tri(7)
    }

    async fn shape(xi: f64, eta: f64) -> Vec<[f64; 2]> {
        shape_tri6(xi, eta).1.to_vec()
    }

    async fn shape_full(xi: f64, eta: f64) -> (Vec<f64>, Vec<[f64; 2]>) {
        let (n, dn) = shape_tri6(xi, eta);
        (n.to_vec(), dn.to_vec())
    }
}

impl Element for Tri6Lst {
    async fn id(&self) -> &str {
        &self.id
    }

    async fn node_ids(&self) -> Vec<String> {
        self.nodes.to_vec()
    }

    async fn dofs_per_node(&self) -> &[Dof] {
        &[Dof::Tx, Dof::Ty]
    }

    async fn stiffness_global(&self, ctx: &ElementContext) -> MatD {
        let coords = plane_coords(ctx);
        let d = self.kind.d_matrix(self.e, self.nu);
        plane_stiffness(&coords, &self.rule(), Self::shape, &d, self.thickness, 12)
    }

    async fn recover(&self, ctx: &ElementContext, u_local: &VecD, _udl: Option<&MemberUdl>) -> ElementResult {
        let coords = plane_coords(ctx);
        let d = self.kind.d_matrix(self.e, self.nu);
        plane_recover(&coords, &self.rule(), Self::shape, &d, u_local)
    }

    async fn mass(&self, ctx: &ElementContext) -> Option<MatD> {
        let coords = plane_coords(ctx);
        Some(plane_mass(&coords, &Self::mass_rule(), Self::shape_full, self.density, self.thickness, 6))
    }

    async fn geometric_stiffness(&self, ctx: &ElementContext, u_element: &VecD) -> Option<MatD> {
        let coords = plane_coords(ctx);
        let d = self.kind.d_matrix(self.e, self.nu);
        Some(plane_geometric_stiffness(&coords, &self.rule(), Self::shape, &d, self.thickness, u_element, 6))
    }
}
// #endregion 🔖️Tri6Lst

// #region 🔖️Quad4
/// ⬜️ 4-node bilinear quadrilateral — DOFs `[Tx, Ty]` per node, 2x2 Gauss-quad integration.
pub struct Quad4 {
    pub id: String,
    pub nodes: [String; 4],
    pub e: f64,
    pub nu: f64,
    pub thickness: f64,
    pub kind: PlaneKind,
    pub density: f64,
}

impl Quad4 {
    async fn rule(&self) -> Vec<(f64, f64, f64)> {
        gauss_quad(2)
    }

    async fn shape(xi: f64, eta: f64) -> Vec<[f64; 2]> {
        shape_quad4(xi, eta).1.to_vec()
    }

    async fn shape_full(xi: f64, eta: f64) -> (Vec<f64>, Vec<[f64; 2]>) {
        let (n, dn) = shape_quad4(xi, eta);
        (n.to_vec(), dn.to_vec())
    }
}

impl Element for Quad4 {
    async fn id(&self) -> &str {
        &self.id
    }

    async fn node_ids(&self) -> Vec<String> {
        self.nodes.to_vec()
    }

    async fn dofs_per_node(&self) -> &[Dof] {
        &[Dof::Tx, Dof::Ty]
    }

    async fn stiffness_global(&self, ctx: &ElementContext) -> MatD {
        let coords = plane_coords(ctx);
        let d = self.kind.d_matrix(self.e, self.nu);
        plane_stiffness(&coords, &self.rule(), Self::shape, &d, self.thickness, 8)
    }

    async fn recover(&self, ctx: &ElementContext, u_local: &VecD, _udl: Option<&MemberUdl>) -> ElementResult {
        let coords = plane_coords(ctx);
        let d = self.kind.d_matrix(self.e, self.nu);
        plane_recover(&coords, &self.rule(), Self::shape, &d, u_local)
    }

    /// 🏋️ Consistent bilinear mass — the same 2x2 rule as stiffness under-integrates the biquadratic
    /// `Ni·Nj` product for a non-rectangular quad, so mass uses the fuller 3x3 rule instead.
    async fn mass(&self, ctx: &ElementContext) -> Option<MatD> {
        let coords = plane_coords(ctx);
        Some(plane_mass(&coords, &gauss_quad(3), Self::shape_full, self.density, self.thickness, 4))
    }

    async fn geometric_stiffness(&self, ctx: &ElementContext, u_element: &VecD) -> Option<MatD> {
        let coords = plane_coords(ctx);
        let d = self.kind.d_matrix(self.e, self.nu);
        Some(plane_geometric_stiffness(&coords, &self.rule(), Self::shape, &d, self.thickness, u_element, 4))
    }
}
// #endregion 🔖️Quad4

// #region 🔖️Quad8
/// ⬜️ 8-node serendipity quadratic quadrilateral — DOFs `[Tx, Ty]` per node, 3x3 (deliberately
/// full-integrated, per standard FEM practice for serendipity elements) Gauss-quad integration.
pub struct Quad8 {
    pub id: String,
    pub nodes: [String; 8],
    pub e: f64,
    pub nu: f64,
    pub thickness: f64,
    pub kind: PlaneKind,
    pub density: f64,
}

impl Quad8 {
    async fn rule(&self) -> Vec<(f64, f64, f64)> {
        gauss_quad(3)
    }

    async fn shape(xi: f64, eta: f64) -> Vec<[f64; 2]> {
        shape_quad8(xi, eta).1.to_vec()
    }

    async fn shape_full(xi: f64, eta: f64) -> (Vec<f64>, Vec<[f64; 2]>) {
        let (n, dn) = shape_quad8(xi, eta);
        (n.to_vec(), dn.to_vec())
    }
}

impl Element for Quad8 {
    async fn id(&self) -> &str {
        &self.id
    }

    async fn node_ids(&self) -> Vec<String> {
        self.nodes.to_vec()
    }

    async fn dofs_per_node(&self) -> &[Dof] {
        &[Dof::Tx, Dof::Ty]
    }

    async fn stiffness_global(&self, ctx: &ElementContext) -> MatD {
        let coords = plane_coords(ctx);
        let d = self.kind.d_matrix(self.e, self.nu);
        plane_stiffness(&coords, &self.rule(), Self::shape, &d, self.thickness, 16)
    }

    async fn recover(&self, ctx: &ElementContext, u_local: &VecD, _udl: Option<&MemberUdl>) -> ElementResult {
        let coords = plane_coords(ctx);
        let d = self.kind.d_matrix(self.e, self.nu);
        plane_recover(&coords, &self.rule(), Self::shape, &d, u_local)
    }

    async fn mass(&self, ctx: &ElementContext) -> Option<MatD> {
        let coords = plane_coords(ctx);
        Some(plane_mass(&coords, &self.rule(), Self::shape_full, self.density, self.thickness, 8))
    }

    async fn geometric_stiffness(&self, ctx: &ElementContext, u_element: &VecD) -> Option<MatD> {
        let coords = plane_coords(ctx);
        let d = self.kind.d_matrix(self.e, self.nu);
        Some(plane_geometric_stiffness(&coords, &self.rule(), Self::shape, &d, self.thickness, u_element, 8))
    }
}
// #endregion 🔖️Quad8
// #endregion 🔖️Continuum

// #region 🔖️PlateDkt
/// 🧮️ Per-edge Batoz DKT geometric coefficients `a,b,c,d,e` (cross-checked against Batoz, Bathe & Ho
/// (1980) via the JuliaFEM `FEMPlates.jl` reference implementation — `e_k` is a DISTINCT coefficient
/// from `b_k`, used only in `Hy`'s `βx`-columns; it is NOT the `f_k` appearing in some other DKT
/// write-ups' `Hx`, which this formulation doesn't need).
struct DktEdge {
    a: f64,
    b: f64,
    c: f64,
    d: f64,
    e: f64,
}

async fn dkt_edge(pi: [f64; 2], pj: [f64; 2]) -> DktEdge {
    let x_ij = pi[0] - pj[0];
    let y_ij = pi[1] - pj[1];
    let l2 = x_ij * x_ij + y_ij * y_ij;
    DktEdge { a: -x_ij / l2, b: 0.75 * x_ij * y_ij / l2, c: (0.25 * x_ij * x_ij - 0.5 * y_ij * y_ij) / l2, d: -y_ij / l2, e: (0.25 * y_ij * y_ij - 0.5 * x_ij * x_ij) / l2 }
}

/// 🧱️ Bending constitutive matrix `(E t³)/(12(1-ν²)) [[1,ν,0],[ν,1,0],[0,0,(1-ν)/2]]`, shared by
/// `PlateDkt` and (via `crate::elements2d::d_matrix_plate`) `elements3d::ShellFacet3`'s bending part.
pub(crate) async fn d_matrix_plate(e: f64, nu: f64, thickness: f64) -> MatD {
    let factor = e * thickness.powi(3) / (12.0 * (1.0 - nu * nu));
    let mut d = MatD::zeros(3, 3);
    d.set(0, 0, factor);
    d.set(0, 1, factor * nu);
    d.set(1, 0, factor * nu);
    d.set(1, 1, factor);
    d.set(2, 2, factor * (1.0 - nu) / 2.0);
    d
}

/// 🧮️ Batoz DKT curvature-displacement B-matrix (3x9) at parametric `(xi, eta)` on a flat triangle
/// with physical `coords`. Dof order `[w1,Rx1,Ry1,w2,Rx2,Ry2,w3,Rx3,Ry3]`, where `Rx = ∂w/∂y` and
/// `Ry = -∂w/∂x` (the physically standard rotation-about-local-axis convention — see the module docs
/// on `PlateDkt`). Builds Batoz's `Hx`/`Hy` shape combinations over the standard mid-edge quadratic
/// "bubble" functions `N4=4L2L3, N5=4L3L1, N6=4L1L2` and the QUADRATIC Tri6 corner functions
/// `Ni=Li(2Li-1)` (cross-checked against the JuliaFEM `FEMPlates.jl` reference implementation of
/// Batoz, Bathe & Ho 1980). Batoz's own `Hx`/`Hy` columns land directly on our `(Rx,Ry)` dof slots
/// with NO permutation — empirically confirmed via the patch test below — but the curvature signs
/// `κx=-∂Hx/∂x, κy=-∂Hy/∂y, κxy=-(∂Hx/∂y+∂Hy/∂x)` carry an overall minus relative to the raw `Hx`/`Hy`
/// derivatives (this formulation's `Hx`/`Hy` represent the NEGATIVE of the physical rotation fields).
/// Shared by `PlateDkt` and `elements3d::ShellFacet3`.
pub(crate) async fn dkt_b_matrix(coords: &[[f64; 2]; 3], xi: f64, eta: f64) -> MatD {
    let edge4 = dkt_edge(coords[1], coords[2]);
    let edge5 = dkt_edge(coords[2], coords[0]);
    let edge6 = dkt_edge(coords[0], coords[1]);

    let (_, dn_param) = shape_tri3(xi, eta);
    let (_, _, d_n_xy) = jacobian_2d(coords, &dn_param);
    let gx = [d_n_xy[0][0], d_n_xy[1][0], d_n_xy[2][0]];
    let gy = [d_n_xy[0][1], d_n_xy[1][1], d_n_xy[2][1]];

    let (l1, l2v, l3) = (1.0 - xi - eta, xi, eta);
    let dn4dx = 4.0 * (gx[1] * l3 + l2v * gx[2]);
    let dn4dy = 4.0 * (gy[1] * l3 + l2v * gy[2]);
    let dn5dx = 4.0 * (gx[2] * l1 + l3 * gx[0]);
    let dn5dy = 4.0 * (gy[2] * l1 + l3 * gy[0]);
    let dn6dx = 4.0 * (gx[0] * l2v + l1 * gx[1]);
    let dn6dy = 4.0 * (gy[0] * l2v + l1 * gy[1]);

    // The Hx3/Hx6/Hx9 and Hy2/Hy5/Hy8 "direct" terms use the QUADRATIC Tri6 corner shape functions
    // `Ni = Li*(2Li-1)` (matching `formulation::shape_tri6`'s convention), NOT the linear area
    // coordinates `Li` themselves — `dNi/dx = (4Li-1) * dLi/dx`.
    let dn1qdx = (4.0 * l1 - 1.0) * gx[0];
    let dn1qdy = (4.0 * l1 - 1.0) * gy[0];
    let dn2qdx = (4.0 * l2v - 1.0) * gx[1];
    let dn2qdy = (4.0 * l2v - 1.0) * gy[1];
    let dn3qdx = (4.0 * l3 - 1.0) * gx[2];
    let dn3qdy = (4.0 * l3 - 1.0) * gy[2];

    // Batoz-order (w1,βx1,βy1,w2,βx2,βy2,w3,βx3,βy3) partial derivatives of the Hx/Hy shape combinations.
    let dhx_dx = [
        1.5 * (edge6.a * dn6dx - edge5.a * dn5dx),
        edge5.b * dn5dx + edge6.b * dn6dx,
        dn1qdx - edge5.c * dn5dx - edge6.c * dn6dx,
        1.5 * (edge4.a * dn4dx - edge6.a * dn6dx),
        edge6.b * dn6dx + edge4.b * dn4dx,
        dn2qdx - edge6.c * dn6dx - edge4.c * dn4dx,
        1.5 * (edge5.a * dn5dx - edge4.a * dn4dx),
        edge4.b * dn4dx + edge5.b * dn5dx,
        dn3qdx - edge4.c * dn4dx - edge5.c * dn5dx,
    ];
    let dhx_dy = [
        1.5 * (edge6.a * dn6dy - edge5.a * dn5dy),
        edge5.b * dn5dy + edge6.b * dn6dy,
        dn1qdy - edge5.c * dn5dy - edge6.c * dn6dy,
        1.5 * (edge4.a * dn4dy - edge6.a * dn6dy),
        edge6.b * dn6dy + edge4.b * dn4dy,
        dn2qdy - edge6.c * dn6dy - edge4.c * dn4dy,
        1.5 * (edge5.a * dn5dy - edge4.a * dn4dy),
        edge4.b * dn4dy + edge5.b * dn5dy,
        dn3qdy - edge4.c * dn4dy - edge5.c * dn5dy,
    ];
    let dhy_dy = [
        1.5 * (edge6.d * dn6dy - edge5.d * dn5dy),
        -dn1qdy + edge5.e * dn5dy + edge6.e * dn6dy,
        -edge5.b * dn5dy - edge6.b * dn6dy,
        1.5 * (edge4.d * dn4dy - edge6.d * dn6dy),
        -dn2qdy + edge4.e * dn4dy + edge6.e * dn6dy,
        -edge4.b * dn4dy - edge6.b * dn6dy,
        1.5 * (edge5.d * dn5dy - edge4.d * dn4dy),
        -dn3qdy + edge4.e * dn4dy + edge5.e * dn5dy,
        -edge4.b * dn4dy - edge5.b * dn5dy,
    ];
    let dhy_dx = [
        1.5 * (edge6.d * dn6dx - edge5.d * dn5dx),
        -dn1qdx + edge5.e * dn5dx + edge6.e * dn6dx,
        -edge5.b * dn5dx - edge6.b * dn6dx,
        1.5 * (edge4.d * dn4dx - edge6.d * dn6dx),
        -dn2qdx + edge4.e * dn4dx + edge6.e * dn6dx,
        -edge4.b * dn4dx - edge6.b * dn6dx,
        1.5 * (edge5.d * dn5dx - edge4.d * dn4dx),
        -dn3qdx + edge4.e * dn4dx + edge5.e * dn5dx,
        -edge4.b * dn4dx - edge5.b * dn5dx,
    ];

    // Batoz's Hx/Hy columns land directly on our (w,Rx,Ry) triple with no permutation, but with an
    // overall sign flip (see the doc comment above): κx=-∂Hx/∂x, κy=-∂Hy/∂y, κxy=-(∂Hx/∂y+∂Hy/∂x).
    let mut b = MatD::zeros(3, 9);
    for i in 0..3 {
        let (bw, bbx, bby) = (3 * i, 3 * i + 1, 3 * i + 2);
        b.set(0, bw, -dhx_dx[bw]);
        b.set(0, bbx, -dhx_dx[bbx]);
        b.set(0, bby, -dhx_dx[bby]);
        b.set(1, bw, -dhy_dy[bw]);
        b.set(1, bbx, -dhy_dy[bbx]);
        b.set(1, bby, -dhy_dy[bby]);
        b.set(2, bw, -dhx_dy[bw] - dhy_dx[bw]);
        b.set(2, bbx, -dhx_dy[bbx] - dhy_dx[bbx]);
        b.set(2, bby, -dhx_dy[bby] - dhy_dx[bby]);
    }
    b
}

/// 🧊️ Batoz Discrete Kirchhoff Triangle (DKT) — 3-node thin-plate bending element, DOFs `[Tz,Rx,Ry]`
/// per node (`Rx = ∂w/∂y`, `Ry = -∂w/∂x`, the physically standard rotation-about-axis convention: a
/// positive rotation about the local x-axis tilts the plate normal the same way a positive `∂w/∂y`
/// slope does). 3-point Gauss-tri integration of the (non-constant, unlike CST) curvature field. See
/// Batoz, Bathe & Ho (1980) "A study of three-node triangular plate bending elements".
///
/// 🌀️ Reports NO `geometric_stiffness` (stays the trait default `None`) — a pure bending element
/// carries no membrane stress state to destabilize its own transverse deflection; plate/shell
/// buckling under in-plane compression needs the membrane-bending coupling `elements3d::ShellFacet3`
/// provides, not `PlateDkt` alone.
pub struct PlateDkt {
    pub id: String,
    pub nodes: [String; 3],
    pub e: f64,
    pub nu: f64,
    pub thickness: f64,
    pub density: f64,
}

impl PlateDkt {
    async fn coords(ctx: &ElementContext) -> [[f64; 2]; 3] {
        [[ctx.positions[0][0], ctx.positions[0][1]], [ctx.positions[1][0], ctx.positions[1][1]], [ctx.positions[2][0], ctx.positions[2][1]]]
    }
}

impl Element for PlateDkt {
    async fn id(&self) -> &str {
        &self.id
    }

    async fn node_ids(&self) -> Vec<String> {
        self.nodes.to_vec()
    }

    async fn dofs_per_node(&self) -> &[Dof] {
        &[Dof::Tz, Dof::Rx, Dof::Ry]
    }

    async fn stiffness_global(&self, ctx: &ElementContext) -> MatD {
        let coords = Self::coords(ctx);
        let (_, det_j, _) = jacobian_2d(&coords, &shape_tri3(0.0, 0.0).1);
        let d = d_matrix_plate(self.e, self.nu, self.thickness);
        let mut ke = MatD::zeros(9, 9);
        for (xi, eta, w) in gauss_tri(3) {
            let b = dkt_b_matrix(&coords, xi, eta);
            ke.add_triple_product(&b, &d, w * det_j);
        }
        ke
    }

    async fn recover(&self, ctx: &ElementContext, u_local: &VecD, _udl: Option<&MemberUdl>) -> ElementResult {
        let coords = Self::coords(ctx);
        let d = d_matrix_plate(self.e, self.nu, self.thickness);
        let gauss = gauss_tri(3)
            .into_iter()
            .map(|(xi, eta, _)| {
                let b = dkt_b_matrix(&coords, xi, eta);
                let kappa = b.mul_vec(u_local);
                let m = d.mul_vec(&kappa);
                PlateMoments { mx: m.get(0), my: m.get(1), mxy: m.get(2) }
            })
            .collect();
        ElementResult::Plate { gauss }
    }

    /// 🏋️ Lumped translational mass `ρtA/3` on each node's `Tz` only — zero rotary inertia. DKT has
    /// no independent transverse-displacement interpolation to derive a consistent mass from (its
    /// curvature field comes from `w`+rotations jointly), so lumping the plate's own weight evenly
    /// across its 3 corners is the standard practical simplification (Cook, Malkus, Plesha & Witt).
    async fn mass(&self, ctx: &ElementContext) -> Option<MatD> {
        let coords = Self::coords(ctx);
        let (_, det_j, _) = jacobian_2d(&coords, &shape_tri3(0.0, 0.0).1);
        let area = 0.5 * det_j;
        let share = self.density * self.thickness * area / 3.0;
        let mut m = MatD::zeros(9, 9);
        for i in 0..3 {
            m.set(3 * i, 3 * i, share);
        }
        Some(m)
    }
}
// #endregion 🔖️PlateDkt

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{solve_linear_static, Model, NodalLoad, Node, Support};

    /// 🪢️ Headless (no document layer) axial elongation check: δ = FL/EA, N = F.
    #[semio_framework_async_macros::async_test]
    async fn bar2_axial_matches_hand_calc() {
        let (e, area, l, p) = (200e9, 0.001, 2.0, 5000.0);
        let model = Model {
            nodes: vec![Node { id: "a".into(), pos: [0.0, 0.0, 0.0] }, Node { id: "b".into(), pos: [l, 0.0, 0.0] }],
            elements: vec![Bar2 { id: "e1".into(), start: "a".into(), end: "b".into(), e, area, density: 0.0 }.into()],
            // A single bar only resists motion along its own axis, so `b`'s transverse (Ty) DOF must
            // also be restrained here — otherwise it's a mechanism (zero stiffness, singular system).
            supports: vec![Support { node_id: "a".into(), fixed: vec![Dof::Tx, Dof::Ty] }, Support { node_id: "b".into(), fixed: vec![Dof::Ty] }],
            nodal_loads: vec![NodalLoad { node_id: "b".into(), dof: Dof::Tx, value: p }],
            member_loads: vec![],
        };
        let result = solve_linear_static(&model).expect("solves");
        let expected = p * l / (e * area);
        let b = result.displacements.iter().find(|d| d.node_id == "b").unwrap();
        assert!((b.values[Dof::Tx.index()] - expected).abs() / expected < 1e-9);
        let ElementResult::Bar { n } = result.elements[0].1 else { panic!("expected bar") };
        assert!((n - p).abs() < 1e-6);
    }

    /// 🏗️ Headless cantilever tip-load check: δ = PL³/3EI, θ = PL²/2EI — the classic beam-theory
    /// benchmark, exercised here directly against `fem_core::Model` (no document layer involved).
    #[semio_framework_async_macros::async_test]
    async fn beam_eb2_cantilever_matches_hand_calc() {
        let (e, iy, area, l, p) = (200e9, 1e-5, 0.01, 2.0, 1000.0);
        let model = Model {
            nodes: vec![Node { id: "a".into(), pos: [0.0, 0.0, 0.0] }, Node { id: "b".into(), pos: [l, 0.0, 0.0] }],
            elements: vec![BeamEb2 { id: "e1".into(), start: "a".into(), end: "b".into(), e, area, iy, density: 0.0 }.into()],
            supports: vec![Support { node_id: "a".into(), fixed: vec![Dof::Tx, Dof::Ty, Dof::Rz] }],
            nodal_loads: vec![NodalLoad { node_id: "b".into(), dof: Dof::Ty, value: -p }],
            member_loads: vec![],
        };
        let result = solve_linear_static(&model).expect("solves");
        let expected_deflection = p * l.powi(3) / (3.0 * e * iy);
        let expected_rotation = p * l.powi(2) / (2.0 * e * iy);
        let b = result.displacements.iter().find(|d| d.node_id == "b").unwrap();
        assert!((b.values[Dof::Ty.index()].abs() - expected_deflection).abs() / expected_deflection < 1e-6);
        assert!((b.values[Dof::Rz.index()].abs() - expected_rotation).abs() / expected_rotation < 1e-6);
    }

    /// 🌀️ Rigid-body test: a pure translation (no relative deformation) must produce zero internal
    /// force — `Ke * rigid_translation ≈ 0`. Catches sign/assembly bugs that a single load case might not.
    #[semio_framework_async_macros::async_test]
    async fn beam_eb2_rigid_translation_gives_zero_force() {
        let (e, iy, area, l) = (200e9, 1e-5, 0.01, 2.0);
        let beam = BeamEb2 { id: "e1".into(), start: "a".into(), end: "b".into(), e, area, iy, density: 0.0 };
        let ctx = ElementContext { positions: vec![[0.0, 0.0, 0.0], [l, 0.0, 0.0]] };
        let ke = beam.stiffness_global(&ctx);
        let rigid = VecD::from_vec(vec![3.0, 4.0, 0.0, 3.0, 4.0, 0.0]);
        let f = ke.mul_vec(&rigid);
        for i in 0..6 {
            assert!(f.get(i).abs() < 1e-6, "rigid-body force[{i}] = {}", f.get(i));
        }
    }

    /// 🏋️ `Bar2::mass` matches the hand-derived isotropic `m = ρAL/6` block form directly.
    #[semio_framework_async_macros::async_test]
    async fn bar2_mass_matches_hand_calc() {
        let (density, area, l) = (7850.0, 0.001, 2.0);
        let bar = Bar2 { id: "e1".into(), start: "a".into(), end: "b".into(), e: 200e9, area, density };
        let ctx = ElementContext { positions: vec![[0.0, 0.0, 0.0], [l, 0.0, 0.0]] };
        let m = bar.mass(&ctx).expect("bar2 reports mass");
        let expected = density * area * l / 6.0;
        assert!((m.get(0, 0) - 2.0 * expected).abs() < 1e-9);
        assert!((m.get(1, 1) - 2.0 * expected).abs() < 1e-9);
        assert!((m.get(0, 2) - expected).abs() < 1e-9);
        assert!((m.get(1, 3) - expected).abs() < 1e-9);
        assert!((m.get(0, 1)).abs() < 1e-12, "no coupling between Tx and Ty");
    }

    /// ⚖️ Consistent-mass physical sanity check: the sum of ALL entries in a pure-translational
    /// submatrix (no rotational DOFs involved) must equal the element's total mass `ρAL` — a
    /// consequence of the shape functions partitioning unity.
    #[semio_framework_async_macros::async_test]
    async fn bar2_mass_total_equals_rho_a_l() {
        let (density, area, l) = (7850.0, 0.001, 2.0);
        let bar = Bar2 { id: "e1".into(), start: "a".into(), end: "b".into(), e: 200e9, area, density };
        let ctx = ElementContext { positions: vec![[0.0, 0.0, 0.0], [l, 0.0, 0.0]] };
        let m = bar.mass(&ctx).expect("bar2 reports mass");
        let sum_tx: f64 = [0, 2].iter().flat_map(|&r| [0, 2].iter().map(move |&c| (r, c))).map(|(r, c)| m.get(r, c)).sum();
        assert!((sum_tx - density * area * l).abs() / (density * area * l) < 1e-9);
    }

    /// 🏋️ `BeamEb2::mass`'s axial 2x2 submatrix sums to the total member mass `ρAL` (same identity as
    /// `Bar2`'s, since the axial DOFs carry no rotational coupling) — checked on a horizontal member so
    /// global == local (rotation is identity) and hand-derived indices apply directly.
    #[semio_framework_async_macros::async_test]
    async fn beam_eb2_mass_axial_block_sums_to_total_mass() {
        let (e, iy, area, l, density) = (200e9, 1e-5, 0.01, 2.0, 7850.0);
        let beam = BeamEb2 { id: "e1".into(), start: "a".into(), end: "b".into(), e, area, iy, density };
        let ctx = ElementContext { positions: vec![[0.0, 0.0, 0.0], [l, 0.0, 0.0]] };
        let m = beam.mass(&ctx).expect("beam_eb2 reports mass");
        let sum_axial = m.get(0, 0) + m.get(0, 3) + m.get(3, 0) + m.get(3, 3);
        let expected = density * area * l;
        assert!((sum_axial - expected).abs() / expected < 1e-9);
    }

    /// 🌀️ Geometric stiffness must vanish under a pure rigid translation, same as ordinary stiffness —
    /// a non-zero axial force alone shouldn't invent a force from rigid motion.
    #[semio_framework_async_macros::async_test]
    async fn beam_eb2_geometric_stiffness_rigid_translation_gives_zero_force() {
        let (e, iy, area, l) = (200e9, 1e-5, 0.01, 2.0);
        let beam = BeamEb2 { id: "e1".into(), start: "a".into(), end: "b".into(), e, area, iy, density: 0.0 };
        let ctx = ElementContext { positions: vec![[0.0, 0.0, 0.0], [l, 0.0, 0.0]] };
        // A pure translation along +x produces a nonzero axial force n = EA/L * dx; geometric
        // stiffness only touches the bending block, so a pure translation still gives zero force there.
        let u = VecD::from_vec(vec![0.0, 0.0, 0.0, 0.001, 0.0, 0.0]);
        let kg = beam.geometric_stiffness(&ctx, &u).expect("beam_eb2 reports geometric stiffness");
        let rigid = VecD::from_vec(vec![3.0, 4.0, 0.0, 3.0, 4.0, 0.0]);
        let f = kg.mul_vec(&rigid);
        for i in 0..6 {
            assert!(f.get(i).abs() < 1e-6, "rigid-body geometric force[{i}] = {}", f.get(i));
        }
    }

    /// 🌀️ Geometric stiffness is symmetric and scales linearly with the recovered axial force.
    #[semio_framework_async_macros::async_test]
    async fn beam_eb2_geometric_stiffness_is_symmetric_and_scales_with_axial_force() {
        let (e, iy, area, l) = (200e9, 1e-5, 0.01, 2.0);
        let beam = BeamEb2 { id: "e1".into(), start: "a".into(), end: "b".into(), e, area, iy, density: 0.0 };
        let ctx = ElementContext { positions: vec![[0.0, 0.0, 0.0], [l, 0.0, 0.0]] };
        let u1 = VecD::from_vec(vec![0.0, 0.0, 0.0, 0.001, 0.0, 0.0]);
        let u2 = VecD::from_vec(vec![0.0, 0.0, 0.0, 0.002, 0.0, 0.0]);
        let kg1 = beam.geometric_stiffness(&ctx, &u1).unwrap();
        let kg2 = beam.geometric_stiffness(&ctx, &u2).unwrap();
        for r in 0..6 {
            for c in 0..6 {
                assert!((kg1.get(r, c) - kg1.get(c, r)).abs() < 1e-9, "Kg not symmetric at ({r},{c})");
                assert!((kg2.get(r, c) - 2.0 * kg1.get(r, c)).abs() < 1e-6, "Kg should scale linearly with axial force at ({r},{c})");
            }
        }
    }

    /// 🌬️ `Bar2::equivalent_nodal_loads` splits a global UDL `wL/2` exactly evenly at both nodes.
    #[semio_framework_async_macros::async_test]
    async fn bar2_equivalent_nodal_loads_matches_wl_over_2() {
        let (e, area, l) = (200e9, 0.001, 2.0);
        let bar = Bar2 { id: "e1".into(), start: "a".into(), end: "b".into(), e, area, density: 0.0 };
        let ctx = ElementContext { positions: vec![[0.0, 0.0, 0.0], [l, 0.0, 0.0]] };
        let udl = MemberUdl { wx: 100.0, wy: -50.0, wz: 0.0 };
        let f = bar.equivalent_nodal_loads(&ctx, &udl).expect("bar2 reports equivalent nodal loads");
        let half = l / 2.0;
        assert!((f.get(0) - udl.wx * half).abs() < 1e-9);
        assert!((f.get(1) - udl.wy * half).abs() < 1e-9);
        assert!((f.get(2) - udl.wx * half).abs() < 1e-9);
        assert!((f.get(3) - udl.wy * half).abs() < 1e-9);
    }

    /// 🌀️ `Bar2::geometric_stiffness`: zero under rigid translation, symmetric, and destabilizes only
    /// the direction PERPENDICULAR to the bar's own axis (an axially-aligned bar with axial force `n`
    /// should have ZERO transverse stiffness contribution along its own axis).
    #[semio_framework_async_macros::async_test]
    async fn bar2_geometric_stiffness_rigid_translation_gives_zero_force_and_is_symmetric() {
        let (e, area, l) = (200e9, 0.001, 2.0);
        let bar = Bar2 { id: "e1".into(), start: "a".into(), end: "b".into(), e, area, density: 0.0 };
        let ctx = ElementContext { positions: vec![[0.0, 0.0, 0.0], [l, 0.0, 0.0]] };
        let u = VecD::from_vec(vec![0.0, 0.0, 0.001, 0.0]);
        let kg = bar.geometric_stiffness(&ctx, &u).expect("bar2 reports geometric stiffness");
        for r in 0..4 {
            for c in 0..4 {
                assert!((kg.get(r, c) - kg.get(c, r)).abs() < 1e-9, "Kg not symmetric at ({r},{c})");
            }
        }
        let rigid = VecD::from_vec(vec![3.0, 4.0, 3.0, 4.0]);
        let f = kg.mul_vec(&rigid);
        for i in 0..4 {
            assert!(f.get(i).abs() < 1e-6, "rigid-body geometric force[{i}] = {}", f.get(i));
        }
        // Axial member here runs along global X, so `Kg`'s axial (Tx) rows/columns must be zero.
        for i in [0usize, 2] {
            for j in 0..4 {
                assert!(kg.get(i, j).abs() < 1e-6, "Kg({i},{j}) should be zero along the bar's own axis");
            }
        }
    }
}
// #endregion 🔖️Tests

// #region 🔖️ContinuumTests
#[cfg(test)]
mod continuum_tests {
    use super::*;
    use crate::model::{solve_linear_static, Model, NodalLoad, Node, Support};

    /// 📐️ Builds a node-major `[u_i,v_i]` displacement vector by sampling the linear field
    /// `u = a.0 + a.1*x + a.2*y`, `v = b.0 + b.1*x + b.2*y` at every node coordinate — the standard
    /// FEM patch-test input, guaranteed to be reproduced EXACTLY by any complete element basis.
    async fn linear_field_u_local(coords: &[[f64; 2]], a: (f64, f64, f64), b: (f64, f64, f64)) -> VecD {
        let mut v = Vec::with_capacity(coords.len() * 2);
        for &[x, y] in coords {
            v.push(a.0 + a.1 * x + a.2 * y);
            v.push(b.0 + b.1 * x + b.2 * y);
        }
        VecD::from_vec(v)
    }

    async fn rigid_translation_u_local(n_nodes: usize, dx: f64, dy: f64) -> VecD {
        let mut v = Vec::with_capacity(n_nodes * 2);
        for _ in 0..n_nodes {
            v.push(dx);
            v.push(dy);
        }
        VecD::from_vec(v)
    }

    async fn assert_plane_gauss_matches(gauss: &[PlaneStress], expected: (f64, f64, f64), tol: f64) {
        for gp in gauss {
            assert!((gp.sxx - expected.0).abs() < tol, "sxx {} vs {}", gp.sxx, expected.0);
            assert!((gp.syy - expected.1).abs() < tol, "syy {} vs {}", gp.syy, expected.1);
            assert!((gp.sxy - expected.2).abs() < tol, "sxy {} vs {}", gp.sxy, expected.2);
        }
    }

    async fn assert_rigid_body_gives_zero_force(ke: &MatD, u_local: &VecD) {
        let f = ke.mul_vec(u_local);
        for i in 0..f.len() {
            assert!(f.get(i).abs() < 1e-6, "rigid-body force[{i}] = {}", f.get(i));
        }
    }

    // Shared "test material" — small-magnitude E keeps expected stresses O(1) so the 1e-8 absolute
    // patch-test tolerance is meaningful relative to f64 precision, not swamped by it.
    const E: f64 = 1000.0;
    const NU: f64 = 0.25;
    const A: (f64, f64, f64) = (0.01, 0.003, 0.0021);
    const B: (f64, f64, f64) = (-0.02, 0.0012, 0.0027);

    async fn expected_stress(kind: PlaneKind) -> (f64, f64, f64) {
        let d = match kind {
            PlaneKind::Stress => d_matrix_plane_stress(E, NU),
            PlaneKind::Strain => d_matrix_plane_strain(E, NU),
        };
        let strain = VecD::from_vec(vec![A.1, B.2, A.2 + B.1]);
        let sigma = d.mul_vec(&strain);
        (sigma.get(0), sigma.get(1), sigma.get(2))
    }

    async fn ctx_of(coords: &[[f64; 2]]) -> ElementContext {
        ElementContext { positions: coords.iter().map(|&[x, y]| [x, y, 0.0]).collect() }
    }

    #[semio_framework_async_macros::async_test]
    async fn tri3_cst_patch_test_reproduces_linear_field() {
        let coords = [[0.0, 0.0], [2.0, 0.1], [0.2, 1.8]];
        let el = Tri3Cst { id: "t".into(), nodes: ["a".into(), "b".into(), "c".into()], e: E, nu: NU, thickness: 1.0, kind: PlaneKind::Stress, density: 0.0 };
        let ctx = ctx_of(&coords);
        let u = linear_field_u_local(&coords, A, B);
        let ElementResult::Plane { gauss } = el.recover(&ctx, &u, None) else { panic!("expected plane result") };
        assert_eq!(gauss.len(), 1);
        assert_plane_gauss_matches(&gauss, expected_stress(PlaneKind::Stress), 1e-8);
    }

    #[semio_framework_async_macros::async_test]
    async fn tri3_cst_rigid_translation_gives_zero_force() {
        let coords = [[0.0, 0.0], [2.0, 0.1], [0.2, 1.8]];
        let el = Tri3Cst { id: "t".into(), nodes: ["a".into(), "b".into(), "c".into()], e: E, nu: NU, thickness: 1.0, kind: PlaneKind::Stress, density: 0.0 };
        let ctx = ctx_of(&coords);
        let ke = el.stiffness_global(&ctx);
        assert_rigid_body_gives_zero_force(&ke, &rigid_translation_u_local(3, 1.5, -2.3));
    }

    #[semio_framework_async_macros::async_test]
    async fn tri6_lst_patch_test_reproduces_linear_field() {
        let coords = [[0.0, 0.0], [2.0, 0.1], [0.2, 1.8], [1.0, 0.05], [1.1, 0.95], [0.1, 0.9]];
        let el = Tri6Lst { id: "t".into(), nodes: ["a".into(), "b".into(), "c".into(), "d".into(), "e".into(), "f".into()], e: E, nu: NU, thickness: 1.0, kind: PlaneKind::Stress, density: 0.0 };
        let ctx = ctx_of(&coords);
        let u = linear_field_u_local(&coords, A, B);
        let ElementResult::Plane { gauss } = el.recover(&ctx, &u, None) else { panic!("expected plane result") };
        assert_eq!(gauss.len(), 3);
        assert_plane_gauss_matches(&gauss, expected_stress(PlaneKind::Stress), 1e-8);
    }

    #[semio_framework_async_macros::async_test]
    async fn tri6_lst_rigid_translation_gives_zero_force() {
        let coords = [[0.0, 0.0], [2.0, 0.1], [0.2, 1.8], [1.0, 0.05], [1.1, 0.95], [0.1, 0.9]];
        let el = Tri6Lst { id: "t".into(), nodes: ["a".into(), "b".into(), "c".into(), "d".into(), "e".into(), "f".into()], e: E, nu: NU, thickness: 1.0, kind: PlaneKind::Stress, density: 0.0 };
        let ctx = ctx_of(&coords);
        let ke = el.stiffness_global(&ctx);
        assert_rigid_body_gives_zero_force(&ke, &rigid_translation_u_local(6, 1.5, -2.3));
    }

    #[semio_framework_async_macros::async_test]
    async fn quad4_patch_test_reproduces_linear_field() {
        let coords = [[0.0, 0.0], [3.0, 0.2], [3.3, 2.5], [0.2, 2.3]];
        let el = Quad4 { id: "q".into(), nodes: ["a".into(), "b".into(), "c".into(), "d".into()], e: E, nu: NU, thickness: 1.0, kind: PlaneKind::Strain, density: 0.0 };
        let ctx = ctx_of(&coords);
        let u = linear_field_u_local(&coords, A, B);
        let ElementResult::Plane { gauss } = el.recover(&ctx, &u, None) else { panic!("expected plane result") };
        assert_eq!(gauss.len(), 4);
        assert_plane_gauss_matches(&gauss, expected_stress(PlaneKind::Strain), 1e-8);
    }

    #[semio_framework_async_macros::async_test]
    async fn quad4_rigid_translation_gives_zero_force() {
        let coords = [[0.0, 0.0], [3.0, 0.2], [3.3, 2.5], [0.2, 2.3]];
        let el = Quad4 { id: "q".into(), nodes: ["a".into(), "b".into(), "c".into(), "d".into()], e: E, nu: NU, thickness: 1.0, kind: PlaneKind::Strain, density: 0.0 };
        let ctx = ctx_of(&coords);
        let ke = el.stiffness_global(&ctx);
        assert_rigid_body_gives_zero_force(&ke, &rigid_translation_u_local(4, 1.5, -2.3));
    }

    #[semio_framework_async_macros::async_test]
    async fn quad8_patch_test_reproduces_linear_field() {
        let coords = [[0.0, 0.0], [3.0, 0.2], [3.3, 2.5], [0.2, 2.3], [1.5, 0.1], [3.15, 1.35], [1.75, 2.4], [0.1, 1.15]];
        let el = Quad8 { id: "q8".into(), nodes: ["a".into(), "b".into(), "c".into(), "d".into(), "e".into(), "f".into(), "g".into(), "h".into()], e: E, nu: NU, thickness: 1.0, kind: PlaneKind::Stress, density: 0.0 };
        let ctx = ctx_of(&coords);
        let u = linear_field_u_local(&coords, A, B);
        let ElementResult::Plane { gauss } = el.recover(&ctx, &u, None) else { panic!("expected plane result") };
        assert_eq!(gauss.len(), 9, "quad8 must use the full 3x3 rule, not 2x2");
        assert_plane_gauss_matches(&gauss, expected_stress(PlaneKind::Stress), 1e-8);
    }

    #[semio_framework_async_macros::async_test]
    async fn quad8_rigid_translation_gives_zero_force() {
        let coords = [[0.0, 0.0], [3.0, 0.2], [3.3, 2.5], [0.2, 2.3], [1.5, 0.1], [3.15, 1.35], [1.75, 2.4], [0.1, 1.15]];
        let el = Quad8 { id: "q8".into(), nodes: ["a".into(), "b".into(), "c".into(), "d".into(), "e".into(), "f".into(), "g".into(), "h".into()], e: E, nu: NU, thickness: 1.0, kind: PlaneKind::Stress, density: 0.0 };
        let ctx = ctx_of(&coords);
        let ke = el.stiffness_global(&ctx);
        assert_rigid_body_gives_zero_force(&ke, &rigid_translation_u_local(8, 1.5, -2.3));
    }

    /// 🌀️ Cook's membrane: the classic tapered/skewed cantilever panel, meshed on a 4x4 grid of
    /// `Quad4` elements via bilinear blending of the four corner points. A coarse-mesh sanity check
    /// (not a fine-mesh convergence study) — the tip deflection must be positive and finite.
    #[semio_framework_async_macros::async_test]
    async fn quad4_cooks_membrane_tip_deflection_is_positive_and_finite() {
        let n = 4usize;
        let (p00, p10, p11, p01) = ((0.0, 0.0), (48.0, 44.0), (48.0, 60.0), (0.0, 44.0));
        let blend = |r: f64, s: f64| {
            let x = (1.0 - r) * (1.0 - s) * p00.0 + r * (1.0 - s) * p10.0 + r * s * p11.0 + (1.0 - r) * s * p01.0;
            let y = (1.0 - r) * (1.0 - s) * p00.1 + r * (1.0 - s) * p10.1 + r * s * p11.1 + (1.0 - r) * s * p01.1;
            (x, y)
        };
        let node_id = |i: usize, j: usize| format!("n{i}_{j}");

        let mut nodes = Vec::new();
        for i in 0..=n {
            for j in 0..=n {
                let (x, y) = blend(i as f64 / n as f64, j as f64 / n as f64);
                nodes.push(Node { id: node_id(i, j), pos: [x, y, 0.0] });
            }
        }
        let mut elements: Vec<Elements> = Vec::new();
        for i in 0..n {
            for j in 0..n {
                elements.push(Quad4 { id: format!("e{i}_{j}"), nodes: [node_id(i, j), node_id(i + 1, j), node_id(i + 1, j + 1), node_id(i, j + 1)], e: 1.0, nu: 1.0 / 3.0, thickness: 1.0, kind: PlaneKind::Stress, density: 0.0 }.into());
            }
        }
        let supports = (0..=n).map(|j| Support { node_id: node_id(0, j), fixed: vec![Dof::Tx, Dof::Ty] }).collect();
        let per_node = 1.0 / (n as f64 + 1.0);
        let nodal_loads = (0..=n).map(|j| NodalLoad { node_id: node_id(n, j), dof: Dof::Ty, value: per_node }).collect();

        let model = Model { nodes, elements, supports, nodal_loads, member_loads: vec![] };
        let result = solve_linear_static(&model).expect("cook's membrane mesh solves");
        let tip: f64 = (0..=n).map(|j| result.displacements.iter().find(|d| d.node_id == node_id(n, j)).unwrap().values[Dof::Ty.index()]).sum::<f64>() / (n as f64 + 1.0);
        assert!(tip > 0.0 && tip.is_finite(), "tip deflection = {tip}");
    }

    /// ⚖️ Consistent-mass physical sanity check (same identity `bar2_mass_total_equals_rho_a_l` uses):
    /// the sum of the pure-`Tx` submatrix must equal the element's total mass `ρtA`.
    #[semio_framework_async_macros::async_test]
    async fn tri3_cst_mass_total_equals_rho_t_area() {
        let (density, thickness) = (7850.0, 0.02);
        let coords = [[0.0, 0.0], [2.0, 0.1], [0.2, 1.8]];
        let el = Tri3Cst { id: "t".into(), nodes: ["a".into(), "b".into(), "c".into()], e: E, nu: NU, thickness, kind: PlaneKind::Stress, density };
        let ctx = ctx_of(&coords);
        let m = el.mass(&ctx).expect("tri3cst reports mass");
        let area = triangle_signed_area(&coords).abs();
        let sum_tx: f64 = (0..3).flat_map(|r| (0..3).map(move |c| (2 * r, 2 * c))).map(|(r, c)| m.get(r, c)).sum();
        let expected = density * thickness * area;
        assert!((sum_tx - expected).abs() / expected < 1e-9, "sum={sum_tx} expected={expected}");
    }

    async fn triangle_signed_area(coords: &[[f64; 2]]) -> f64 {
        0.5 * ((coords[1][0] - coords[0][0]) * (coords[2][1] - coords[0][1]) - (coords[2][0] - coords[0][0]) * (coords[1][1] - coords[0][1]))
    }

    #[semio_framework_async_macros::async_test]
    async fn quad4_mass_total_equals_rho_t_area() {
        let (density, thickness) = (2400.0, 0.15);
        let coords = [[0.0, 0.0], [3.0, 0.2], [3.3, 2.5], [0.2, 2.3]];
        let el = Quad4 { id: "q".into(), nodes: ["a".into(), "b".into(), "c".into(), "d".into()], e: E, nu: NU, thickness, kind: PlaneKind::Strain, density };
        let ctx = ctx_of(&coords);
        let m = el.mass(&ctx).expect("quad4 reports mass");
        // Shoelace area of the (convex) quad, split as two triangles from vertex 0.
        let area = triangle_signed_area(&[coords[0], coords[1], coords[2]]).abs() + triangle_signed_area(&[coords[0], coords[2], coords[3]]).abs();
        let sum_tx: f64 = (0..4).flat_map(|r| (0..4).map(move |c| (2 * r, 2 * c))).map(|(r, c)| m.get(r, c)).sum();
        let expected = density * thickness * area;
        assert!((sum_tx - expected).abs() / expected < 1e-6, "sum={sum_tx} expected={expected}");
    }

    /// 🌀️ `Tri3Cst::geometric_stiffness` must vanish under a pure rigid translation (zero stress ⇒
    /// zero `Kg`, same reasoning `beam_eb2_geometric_stiffness_rigid_translation_gives_zero_force` uses)
    /// and be symmetric under a genuinely deforming field.
    #[semio_framework_async_macros::async_test]
    async fn tri3_cst_geometric_stiffness_rigid_translation_gives_zero_force_and_is_symmetric() {
        let coords = [[0.0, 0.0], [2.0, 0.1], [0.2, 1.8]];
        let el = Tri3Cst { id: "t".into(), nodes: ["a".into(), "b".into(), "c".into()], e: E, nu: NU, thickness: 1.0, kind: PlaneKind::Stress, density: 0.0 };
        let ctx = ctx_of(&coords);
        let u = linear_field_u_local(&coords, A, B);
        let kg = el.geometric_stiffness(&ctx, &u).expect("tri3cst reports geometric stiffness");
        for r in 0..6 {
            for c in 0..6 {
                assert!((kg.get(r, c) - kg.get(c, r)).abs() < 1e-9, "Kg not symmetric at ({r},{c})");
            }
        }
        let kg_rigid = el.geometric_stiffness(&ctx, &rigid_translation_u_local(3, 1.5, -2.3)).unwrap();
        let f = kg_rigid.mul_vec(&rigid_translation_u_local(3, 0.4, 0.6));
        for i in 0..6 {
            assert!(f.get(i).abs() < 1e-9, "rigid-body geometric force[{i}] = {}", f.get(i));
        }
    }

    /// 🌀️ `Quad4::geometric_stiffness` must vanish under a pure rigid translation and be symmetric —
    /// the last `Quad4` method not already exercised by `quad4_mass_total_equals_rho_t_area`/the patch
    /// and rigid-translation stiffness tests above.
    #[semio_framework_async_macros::async_test]
    async fn quad4_geometric_stiffness_rigid_translation_gives_zero_force_and_is_symmetric() {
        let coords = [[0.0, 0.0], [3.0, 0.2], [3.3, 2.5], [0.2, 2.3]];
        let el = Quad4 { id: "q".into(), nodes: ["a".into(), "b".into(), "c".into(), "d".into()], e: E, nu: NU, thickness: 1.0, kind: PlaneKind::Strain, density: 0.0 };
        let ctx = ctx_of(&coords);
        let u = linear_field_u_local(&coords, A, B);
        let kg = el.geometric_stiffness(&ctx, &u).expect("quad4 reports geometric stiffness");
        for r in 0..8 {
            for c in 0..8 {
                assert!((kg.get(r, c) - kg.get(c, r)).abs() < 1e-9, "Kg not symmetric at ({r},{c})");
            }
        }
        let kg_rigid = el.geometric_stiffness(&ctx, &rigid_translation_u_local(4, 1.5, -2.3)).unwrap();
        let f = kg_rigid.mul_vec(&rigid_translation_u_local(4, 0.4, 0.6));
        for i in 0..8 {
            assert!(f.get(i).abs() < 1e-9, "rigid-body geometric force[{i}] = {}", f.get(i));
        }
    }

    /// ⚖️ `Tri6Lst::mass` total (same partition-of-unity identity `tri3_cst_mass_total_equals_rho_t_area`
    /// uses) — `Tri6Lst`'s `mass`/`mass_rule`/`shape_full` are otherwise never exercised.
    #[semio_framework_async_macros::async_test]
    async fn tri6_lst_mass_total_equals_rho_t_area() {
        let (density, thickness) = (7850.0, 0.02);
        let coords = [[0.0, 0.0], [2.0, 0.1], [0.2, 1.8], [1.0, 0.05], [1.1, 0.95], [0.1, 0.9]];
        let el = Tri6Lst { id: "t".into(), nodes: ["a".into(), "b".into(), "c".into(), "d".into(), "e".into(), "f".into()], e: E, nu: NU, thickness, kind: PlaneKind::Stress, density };
        let ctx = ctx_of(&coords);
        let m = el.mass(&ctx).expect("tri6lst reports mass");
        let area = triangle_signed_area(&[coords[0], coords[1], coords[2]]).abs();
        let sum_tx: f64 = (0..6).flat_map(|r| (0..6).map(move |c| (2 * r, 2 * c))).map(|(r, c)| m.get(r, c)).sum();
        let expected = density * thickness * area;
        assert!((sum_tx - expected).abs() / expected < 1e-6, "sum={sum_tx} expected={expected}");
    }

    /// 🌀️ `Tri6Lst::geometric_stiffness` must vanish under a pure rigid translation and be symmetric.
    #[semio_framework_async_macros::async_test]
    async fn tri6_lst_geometric_stiffness_rigid_translation_gives_zero_force_and_is_symmetric() {
        let coords = [[0.0, 0.0], [2.0, 0.1], [0.2, 1.8], [1.0, 0.05], [1.1, 0.95], [0.1, 0.9]];
        let el = Tri6Lst { id: "t".into(), nodes: ["a".into(), "b".into(), "c".into(), "d".into(), "e".into(), "f".into()], e: E, nu: NU, thickness: 1.0, kind: PlaneKind::Stress, density: 0.0 };
        let ctx = ctx_of(&coords);
        let u = linear_field_u_local(&coords, A, B);
        let kg = el.geometric_stiffness(&ctx, &u).expect("tri6lst reports geometric stiffness");
        for r in 0..12 {
            for c in 0..12 {
                assert!((kg.get(r, c) - kg.get(c, r)).abs() < 1e-9, "Kg not symmetric at ({r},{c})");
            }
        }
        let kg_rigid = el.geometric_stiffness(&ctx, &rigid_translation_u_local(6, 1.5, -2.3)).unwrap();
        let f = kg_rigid.mul_vec(&rigid_translation_u_local(6, 0.4, 0.6));
        for i in 0..12 {
            assert!(f.get(i).abs() < 1e-9, "rigid-body geometric force[{i}] = {}", f.get(i));
        }
    }

    /// ⚖️ `Quad8::mass` total (same identity as `quad4_mass_total_equals_rho_t_area`) — `Quad8`'s
    /// `mass`/`shape_full` are otherwise never exercised.
    #[semio_framework_async_macros::async_test]
    async fn quad8_mass_total_equals_rho_t_area() {
        let (density, thickness) = (2400.0, 0.15);
        let coords = [[0.0, 0.0], [3.0, 0.2], [3.3, 2.5], [0.2, 2.3], [1.5, 0.1], [3.15, 1.35], [1.75, 2.4], [0.1, 1.15]];
        let el = Quad8 { id: "q8".into(), nodes: ["a".into(), "b".into(), "c".into(), "d".into(), "e".into(), "f".into(), "g".into(), "h".into()], e: E, nu: NU, thickness, kind: PlaneKind::Stress, density };
        let ctx = ctx_of(&coords);
        let m = el.mass(&ctx).expect("quad8 reports mass");
        let area = triangle_signed_area(&[coords[0], coords[1], coords[2]]).abs() + triangle_signed_area(&[coords[0], coords[2], coords[3]]).abs();
        let sum_tx: f64 = (0..8).flat_map(|r| (0..8).map(move |c| (2 * r, 2 * c))).map(|(r, c)| m.get(r, c)).sum();
        let expected = density * thickness * area;
        assert!((sum_tx - expected).abs() / expected < 1e-6, "sum={sum_tx} expected={expected}");
    }

    /// 🌀️ `Quad8::geometric_stiffness` must vanish under a pure rigid translation and be symmetric.
    #[semio_framework_async_macros::async_test]
    async fn quad8_geometric_stiffness_rigid_translation_gives_zero_force_and_is_symmetric() {
        let coords = [[0.0, 0.0], [3.0, 0.2], [3.3, 2.5], [0.2, 2.3], [1.5, 0.1], [3.15, 1.35], [1.75, 2.4], [0.1, 1.15]];
        let el = Quad8 { id: "q8".into(), nodes: ["a".into(), "b".into(), "c".into(), "d".into(), "e".into(), "f".into(), "g".into(), "h".into()], e: E, nu: NU, thickness: 1.0, kind: PlaneKind::Stress, density: 0.0 };
        let ctx = ctx_of(&coords);
        let u = linear_field_u_local(&coords, A, B);
        let kg = el.geometric_stiffness(&ctx, &u).expect("quad8 reports geometric stiffness");
        for r in 0..16 {
            for c in 0..16 {
                assert!((kg.get(r, c) - kg.get(c, r)).abs() < 1e-9, "Kg not symmetric at ({r},{c})");
            }
        }
        let kg_rigid = el.geometric_stiffness(&ctx, &rigid_translation_u_local(8, 1.5, -2.3)).unwrap();
        let f = kg_rigid.mul_vec(&rigid_translation_u_local(8, 0.4, 0.6));
        for i in 0..16 {
            assert!(f.get(i).abs() < 1e-9, "rigid-body geometric force[{i}] = {}", f.get(i));
        }
    }

    /// 🔌️ `Tri3Cst`/`Tri6Lst`/`Quad8` used as `crate::model::Elements` variants inside a solved `Model`
    /// — unlike every other test in this module (which calls their methods directly), this exercises
    /// `id`/`node_ids`/`dofs_per_node` via the SAME `#[dyn_enum]`-generated dispatch path
    /// `solve_linear_static` uses for every element kind, on three disjoint single-element-type patches
    /// sharing one solve.
    #[semio_framework_async_macros::async_test]
    async fn continuum_elements_solve_correctly_via_enum_dispatch() {
        let p = 1000.0;
        let mut nodes = vec![Node { id: "t3_a".into(), pos: [0.0, 0.0, 0.0] }, Node { id: "t3_b".into(), pos: [2.0, 0.0, 0.0] }, Node { id: "t3_c".into(), pos: [0.0, 2.0, 0.0] }];
        let mut elements: Vec<Elements> = vec![Tri3Cst { id: "t3".into(), nodes: ["t3_a".into(), "t3_b".into(), "t3_c".into()], e: E, nu: NU, thickness: 1.0, kind: PlaneKind::Stress, density: 0.0 }.into()];
        let mut supports = vec![Support { node_id: "t3_a".into(), fixed: vec![Dof::Tx, Dof::Ty] }, Support { node_id: "t3_b".into(), fixed: vec![Dof::Tx, Dof::Ty] }];
        let mut nodal_loads = vec![NodalLoad { node_id: "t3_c".into(), dof: Dof::Tx, value: p }];

        let tri6_ids = ["t6_n0", "t6_n1", "t6_n2", "t6_n01", "t6_n12", "t6_n20"];
        let tri6_coords: [[f64; 2]; 6] = [[10.0, 0.0], [12.0, 0.0], [10.0, 2.0], [11.0, 0.0], [11.0, 1.0], [10.0, 1.0]];
        for i in 0..6 {
            nodes.push(Node { id: tri6_ids[i].into(), pos: [tri6_coords[i][0], tri6_coords[i][1], 0.0] });
        }
        elements.push(Tri6Lst { id: "t6".into(), nodes: std::array::from_fn(|i| tri6_ids[i].to_string()), e: E, nu: NU, thickness: 1.0, kind: PlaneKind::Stress, density: 0.0 }.into());
        for &id in &tri6_ids[..5] {
            supports.push(Support { node_id: id.into(), fixed: vec![Dof::Tx, Dof::Ty] });
        }
        nodal_loads.push(NodalLoad { node_id: "t6_n20".into(), dof: Dof::Tx, value: p });

        let quad8_ids = ["q8_c0", "q8_c1", "q8_c2", "q8_c3", "q8_m01", "q8_m12", "q8_m23", "q8_m30"];
        let quad8_coords: [[f64; 2]; 8] = [[20.0, 0.0], [22.0, 0.0], [22.0, 2.0], [20.0, 2.0], [21.0, 0.0], [22.0, 1.0], [21.0, 2.0], [20.0, 1.0]];
        for i in 0..8 {
            nodes.push(Node { id: quad8_ids[i].into(), pos: [quad8_coords[i][0], quad8_coords[i][1], 0.0] });
        }
        elements.push(Quad8 { id: "q8".into(), nodes: std::array::from_fn(|i| quad8_ids[i].to_string()), e: E, nu: NU, thickness: 1.0, kind: PlaneKind::Stress, density: 0.0 }.into());
        for &id in &quad8_ids[..7] {
            supports.push(Support { node_id: id.into(), fixed: vec![Dof::Tx, Dof::Ty] });
        }
        nodal_loads.push(NodalLoad { node_id: "q8_m30".into(), dof: Dof::Tx, value: p });

        let model = Model { nodes, elements, supports, nodal_loads, member_loads: vec![] };
        let result = solve_linear_static(&model).expect("mixed continuum patches solve");

        assert_eq!(result.elements.len(), 3);
        for (free_node, element_id) in [("t3_c", "t3"), ("t6_n20", "t6"), ("q8_m30", "q8")] {
            let d = result.displacements.iter().find(|d| d.node_id == free_node).unwrap();
            assert!(d.values[Dof::Tx.index()] > 0.0 && d.values[Dof::Tx.index()].is_finite(), "{free_node}: {}", d.values[Dof::Tx.index()]);
            assert!(result.elements.iter().any(|(id, _)| id == element_id), "missing element result for {element_id}");
        }
    }
}
// #endregion 🔖️ContinuumTests

// #region 🔖️PlateTests
#[cfg(test)]
mod plate_tests {
    use super::*;
    use crate::model::{solve_linear_static, Model, NodalLoad, Node, Support};

    const E: f64 = 1000.0;
    const NU: f64 = 0.25;
    const THICKNESS: f64 = 1.0;
    // Small constant curvatures so the resulting moments stay O(1), matching `continuum_tests`'s
    // rationale for keeping the absolute patch-test tolerance meaningful.
    const KX: f64 = 0.004;
    const KY: f64 = -0.0025;
    const KXY: f64 = 0.0017;

    async fn ctx_of(coords: &[[f64; 2]; 3]) -> ElementContext {
        ElementContext { positions: coords.iter().map(|&[x, y]| [x, y, 0.0]).collect() }
    }

    /// 📐️ Constant-curvature field `w = 0.5*(kx*x² + ky*y² + 2*kxy*x*y)` with matching nodal rotations
    /// `Rx = ∂w/∂y = ky*y + kxy*x`, `Ry = -∂w/∂x = -(kx*x + kxy*y)` — the DKT patch-test input.
    async fn constant_curvature_u_local(coords: &[[f64; 2]; 3]) -> VecD {
        let mut v = Vec::with_capacity(9);
        for &[x, y] in coords {
            v.push(0.5 * (KX * x * x + KY * y * y + 2.0 * KXY * x * y));
            v.push(KY * y + KXY * x);
            v.push(-(KX * x + KXY * y));
        }
        VecD::from_vec(v)
    }

    #[semio_framework_async_macros::async_test]
    async fn plate_dkt_patch_test_reproduces_constant_curvature() {
        let coords = [[0.0, 0.0], [2.0, 0.1], [0.2, 1.8]];
        let el = PlateDkt { id: "p".into(), nodes: ["a".into(), "b".into(), "c".into()], e: E, nu: NU, thickness: THICKNESS, density: 0.0 };
        let ctx = ctx_of(&coords);
        let u = constant_curvature_u_local(&coords);
        let d = d_matrix_plate(E, NU, THICKNESS);
        let kappa = VecD::from_vec(vec![KX, KY, 2.0 * KXY]);
        let expected = d.mul_vec(&kappa);
        let ElementResult::Plate { gauss } = el.recover(&ctx, &u, None) else { panic!("expected plate result") };
        assert_eq!(gauss.len(), 3);
        for gp in &gauss {
            let scale = expected.get(0).abs().max(expected.get(1).abs()).max(expected.get(2).abs()).max(1.0);
            assert!((gp.mx - expected.get(0)).abs() / scale < 1e-4, "mx {} vs {}", gp.mx, expected.get(0));
            assert!((gp.my - expected.get(1)).abs() / scale < 1e-4, "my {} vs {}", gp.my, expected.get(1));
            assert!((gp.mxy - expected.get(2)).abs() / scale < 1e-4, "mxy {} vs {}", gp.mxy, expected.get(2));
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn plate_dkt_rigid_translation_gives_zero_force() {
        let coords = [[0.0, 0.0], [2.0, 0.1], [0.2, 1.8]];
        let el = PlateDkt { id: "p".into(), nodes: ["a".into(), "b".into(), "c".into()], e: E, nu: NU, thickness: THICKNESS, density: 0.0 };
        let ctx = ctx_of(&coords);
        let ke = el.stiffness_global(&ctx);
        let rigid = VecD::from_vec(vec![0.7, 0.0, 0.0, 0.7, 0.0, 0.0, 0.7, 0.0, 0.0]);
        let f = ke.mul_vec(&rigid);
        for i in 0..9 {
            assert!(f.get(i).abs() < 1e-6, "rigid-body force[{i}] = {}", f.get(i));
        }
    }

    /// 🏋️ `PlateDkt::mass` lumps `ρtA/3` onto each node's `Tz` only — zero rotary inertia, zero
    /// coupling to `Rx`/`Ry` — `mass` is otherwise never exercised (`stiffness_global`/`recover` are
    /// covered by the patch/rigid-translation/simply-supported tests above and below).
    #[semio_framework_async_macros::async_test]
    async fn plate_dkt_mass_lumps_rho_t_area_over_3_onto_each_tz_only() {
        let (density, thickness) = (2500.0, 0.02);
        let coords = [[0.0, 0.0], [2.0, 0.1], [0.2, 1.8]];
        let el = PlateDkt { id: "p".into(), nodes: ["a".into(), "b".into(), "c".into()], e: E, nu: NU, thickness, density };
        let ctx = ctx_of(&coords);
        let m = el.mass(&ctx).expect("plate_dkt reports mass");

        let area = 0.5 * ((coords[1][0] - coords[0][0]) * (coords[2][1] - coords[0][1]) - (coords[2][0] - coords[0][0]) * (coords[1][1] - coords[0][1])).abs();
        let expected_share = density * thickness * area / 3.0;
        for i in 0..3 {
            assert!((m.get(3 * i, 3 * i) - expected_share).abs() / expected_share < 1e-9, "node {i} Tz mass");
        }
        for r in 0..9 {
            for c in 0..9 {
                if r != c {
                    assert!(m.get(r, c).abs() < 1e-12, "unexpected coupling at ({r},{c})");
                }
            }
        }
        for i in 0..3 {
            assert!(m.get(3 * i + 1, 3 * i + 1).abs() < 1e-12, "node {i} Rx should carry no mass");
            assert!(m.get(3 * i + 2, 3 * i + 2).abs() < 1e-12, "node {i} Ry should carry no mass");
        }
    }

    /// 🏗️ Simply-supported square plate (side `a`) under a uniform pressure `q`, meshed as a coarse
    /// 2x2 grid (8 `PlateDkt` triangles), `Tz=0` at every boundary node (rotations free everywhere),
    /// load lumped `q*Area_i/3` to each triangle's 3 nodes — checked against the classical thin-plate
    /// centerpoint deflection `w = 0.00406*q*a⁴/D` within an order-of-magnitude (coarse mesh, crude
    /// load lumping, so this is a sanity check, not a convergence study).
    #[semio_framework_async_macros::async_test]
    async fn plate_dkt_simply_supported_square_center_deflection_right_order_of_magnitude() {
        let (e, nu, t, a) = (2e11, 0.3, 0.01, 2.0);
        let q = 1000.0;
        let n = 2usize;
        let dx = a / n as f64;
        let node_id = |i: usize, j: usize| format!("n{i}_{j}");

        let mut nodes = Vec::new();
        for i in 0..=n {
            for j in 0..=n {
                nodes.push(Node { id: node_id(i, j), pos: [dx * i as f64, dx * j as f64, 0.0] });
            }
        }

        let mut elements: Vec<Elements> = Vec::new();
        for i in 0..n {
            for j in 0..n {
                // Each grid cell split into 2 triangles along the (i,j)-(i+1,j+1) diagonal.
                elements.push(PlateDkt { id: format!("t{i}_{j}a"), nodes: [node_id(i, j), node_id(i + 1, j), node_id(i + 1, j + 1)], e, nu, thickness: t, density: 0.0 }.into());
                elements.push(PlateDkt { id: format!("t{i}_{j}b"), nodes: [node_id(i, j), node_id(i + 1, j + 1), node_id(i, j + 1)], e, nu, thickness: t, density: 0.0 }.into());
            }
        }

        let supports = (0..=n).flat_map(|i| (0..=n).map(move |j| (i, j))).filter(|&(i, j)| i == 0 || i == n || j == 0 || j == n).map(|(i, j)| Support { node_id: node_id(i, j), fixed: vec![Dof::Tz] }).collect();

        // Lump `q*Area/3` per triangle onto its 3 nodes, summed across all triangles sharing a node.
        let mut lumped: std::collections::HashMap<String, f64> = std::collections::HashMap::new();
        for i in 0..n {
            for j in 0..n {
                let area = 0.5 * dx * dx;
                let share = q * area / 3.0;
                for id in [node_id(i, j), node_id(i + 1, j), node_id(i + 1, j + 1)] {
                    *lumped.entry(id).or_insert(0.0) += share;
                }
                for id in [node_id(i, j), node_id(i + 1, j + 1), node_id(i, j + 1)] {
                    *lumped.entry(id).or_insert(0.0) += share;
                }
            }
        }
        let nodal_loads = lumped.into_iter().map(|(node_id, value)| NodalLoad { node_id, dof: Dof::Tz, value: -value }).collect();

        let model = Model { nodes, elements, supports, nodal_loads, member_loads: vec![] };
        let result = solve_linear_static(&model).expect("ss plate mesh solves");
        let center = result.displacements.iter().find(|d| d.node_id == node_id(n / 2, n / 2)).unwrap();
        let w_center = -center.values[Dof::Tz.index()];

        let d = e * t.powi(3) / (12.0 * (1.0 - nu * nu));
        let expected = 0.00406 * q * a.powi(4) / d;
        assert!(w_center.is_finite() && w_center > 0.0, "center deflection = {w_center}");
        let ratio = w_center / expected;
        assert!(ratio > 0.5 && ratio < 2.0, "deflection ratio {ratio} (actual {w_center} vs analytical {expected}) out of order-of-magnitude range");
    }
}
// #endregion 🔖️PlateTests
