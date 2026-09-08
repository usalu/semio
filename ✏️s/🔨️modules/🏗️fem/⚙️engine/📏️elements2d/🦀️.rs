//! 📐️ 2D structural elements: axial `Bar2` truss, Euler-Bernoulli `BeamEb2` frame member, the
//! Tri3/Tri6/Quad4/Quad8 plane-stress/plane-strain continuum family, and the `PlateDkt` Batoz
//! Discrete Kirchhoff Triangle thin-plate bending element.

use crate::algebra::{MatD, VecD};
use crate::formulation::{b_matrix_plane, d_matrix_plane_strain, d_matrix_plane_stress, gauss_quad, gauss_tri, jacobian_2d, shape_quad4, shape_quad8, shape_tri3, shape_tri6};
#[cfg(test)]
use crate::model::Elements;
use crate::model::{Dof, Element, ElementContext, ElementResult, MemberUdl, PlaneStress, PlateMoments};

// #region 🔖️Geometry
fn segment_geometry(ctx: &ElementContext) -> (f64, f64, f64) {
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
    fn id(&self) -> &str {
        &self.id
    }

    fn node_ids(&self) -> Vec<String> {
        vec![self.start.clone(), self.end.clone()]
    }

    fn mounted_node_id(&self, index: usize) -> Option<&str> {
        [self.start.as_str(), self.end.as_str()].get(index).copied()
    }

    fn mounted_node_id_count(&self) -> Option<usize> {
        Some(2)
    }

    fn dofs_per_node(&self) -> &[Dof] {
        &[Dof::Tx, Dof::Ty]
    }

    fn close_mounted_string_step(&mut self) -> Option<usize> {
        for owner in [&mut self.id, &mut self.start, &mut self.end] {
            if !owner.is_empty() || owner.capacity() != 0 {
                let bytes = owner.capacity();
                *owner = String::new();
                return Some(bytes);
            }
        }
        None
    }

    fn mounted_next_string_bytes(&self) -> Option<usize> {
        [&self.id, &self.start, &self.end].into_iter().find(|owner| !owner.is_empty() || owner.capacity() != 0).map(|owner| owner.capacity())
    }

    fn mounted_strings_terminal_is_empty(&self) -> bool {
        self.id.capacity() == 0 && self.start.capacity() == 0 && self.end.capacity() == 0
    }

    fn stiffness_global(&self, ctx: &ElementContext) -> MatD {
        let (l, cx, cy) = segment_geometry(ctx);
        let k = self.e * self.area / l;
        let mut m = MatD::zeros(4, 4);
        let terms = [[cx * cx, cx * cy, -cx * cx, -cx * cy], [cx * cy, cy * cy, -cx * cy, -cy * cy]];
        for (row, terms) in terms.iter().enumerate() {
            for (col, &term) in terms.iter().enumerate() {
                m.set(row, col, k * term);
                m.set(row + 2, col, if col < 2 { -k * term } else { k * terms[col - 2] });
            }
        }
        m
    }

    fn mounted_stiffness_cell(&self, ctx: &ElementContext, row: usize, column: usize) -> Option<f64> {
        if row >= 4 || column >= 4 || ctx.positions.len() != 2 {
            return None;
        }
        let (length, cosine, sine) = segment_geometry(ctx);
        if !length.is_finite() || length <= 0.0 {
            return None;
        }
        let direction = [cosine, sine];
        let sign = if row / 2 == column / 2 { 1.0 } else { -1.0 };
        Some(self.e * self.area / length * sign * direction[row % 2] * direction[column % 2])
    }

    fn recover(&self, ctx: &ElementContext, u_local: &VecD, _udl: Option<&MemberUdl>) -> ElementResult {
        let (l, cx, cy) = segment_geometry(ctx);
        let k = self.e * self.area / l;
        let n = k * ((u_local.get(2) - u_local.get(0)) * cx + (u_local.get(3) - u_local.get(1)) * cy);
        ElementResult::Bar { n }
    }

    /// 🏋️ Isotropic lumped-consistent mass — same in both directions since a bar has no bending
    /// stiffness to give mass a preferred orientation. `m = ρAL/6`, block form `[[2m,0,m,0],[0,2m,0,m],
    /// [m,0,2m,0],[0,m,0,2m]]` (node-major `[u1,v1,u2,v2]`).
    fn mass(&self, ctx: &ElementContext) -> Option<MatD> {
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
    fn equivalent_nodal_loads(&self, ctx: &ElementContext, udl: &MemberUdl) -> Option<VecD> {
        let (l, _, _) = segment_geometry(ctx);
        let half = l / 2.0;
        Some(VecD::from_vec(vec![udl.wx * half, udl.wy * half, udl.wx * half, udl.wy * half]))
    }

    /// 🌀️ Truss geometric ("stability") stiffness under the member's own axial force `n` (tension-
    /// positive, same convention as `recover`): `N/L·(I − ccᵀ)` on each 2x2 node block, `ccᵀ` the
    /// outer product of the unit axial direction — the transverse-projector form (Przemieniecki,
    /// "Theory of Matrix Structural Analysis") that only destabilizes displacement PERPENDICULAR to
    /// the bar's own axis, vanishing identically for a rigid translation (which the projector kills).
    fn geometric_stiffness(&self, ctx: &ElementContext, u_element: &VecD) -> Option<MatD> {
        let (l, cx, cy) = segment_geometry(ctx);
        let k = self.e * self.area / l;
        let n = k * ((u_element.get(2) - u_element.get(0)) * cx + (u_element.get(3) - u_element.get(1)) * cy);
        let coeff = n / l;
        let proj = [[1.0 - cx * cx, -cx * cy], [-cx * cy, 1.0 - cy * cy]];
        let mut kg = MatD::zeros(4, 4);
        for (row, projection) in proj.iter().enumerate() {
            for (col, &projection) in projection.iter().enumerate() {
                let v = coeff * projection;
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
fn beam_transform(c: f64, s: f64) -> MatD {
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
fn beam_local_stiffness(l: f64, axial_k: f64, bend_k: f64) -> MatD {
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

fn beam_local_stiffness_cell(length: f64, axial: f64, bending: f64, row: usize, column: usize) -> f64 {
    let length_squared = length * length;
    match (row, column) {
        (0, 0) | (3, 3) => axial,
        (0, 3) | (3, 0) => -axial,
        (1, 1) | (4, 4) => 12.0 * bending / length_squared,
        (1, 2) | (1, 5) | (2, 1) | (5, 1) => 6.0 * bending / length,
        (1, 4) | (4, 1) => -12.0 * bending / length_squared,
        (2, 2) | (5, 5) => 4.0 * bending,
        (2, 4) | (4, 2) | (4, 5) | (5, 4) => -6.0 * bending / length,
        (2, 5) | (5, 2) => 2.0 * bending,
        _ => 0.0,
    }
}

fn beam_transform_cell(cosine: f64, sine: f64, row: usize, column: usize) -> f64 {
    let block = row / 3;
    if column / 3 != block {
        return 0.0;
    }
    match (row % 3, column % 3) {
        (0, 0) | (1, 1) => cosine,
        (2, 2) => 1.0,
        (0, 1) => sine,
        (1, 0) => -sine,
        _ => 0.0,
    }
}

/// 🌬️ Local fixed-end load vector `[u1,v1,θ1,u2,v2,θ2]` for a local-frame UDL `(wx_local, wy_local)`.
fn beam_local_udl(l: f64, wx_local: f64, wy_local: f64) -> VecD {
    VecD::from_vec(vec![wx_local * l / 2.0, wy_local * l / 2.0, wy_local * l * l / 12.0, wx_local * l / 2.0, wy_local * l / 2.0, -wy_local * l * l / 12.0])
}

/// 🏋️ Consistent local mass matrix, dof order `[u1,v1,θ1,u2,v2,θ2]` — axial `ρAL/6*[[2,1],[1,2]]` at
/// `(0,3)`, standard Euler-Bernoulli consistent bending mass at `[1,2,4,5]` (rotary inertia of the
/// cross-section neglected — see Cook/Malkus/Plesha "Concepts and Applications of Finite Element
/// Analysis" for the closed form).
fn beam_local_mass(l: f64, area: f64, density: f64) -> MatD {
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
fn beam_local_geometric_stiffness(l: f64, n: f64) -> MatD {
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
    fn id(&self) -> &str {
        &self.id
    }

    fn node_ids(&self) -> Vec<String> {
        vec![self.start.clone(), self.end.clone()]
    }

    fn mounted_node_id(&self, index: usize) -> Option<&str> {
        [self.start.as_str(), self.end.as_str()].get(index).copied()
    }

    fn mounted_node_id_count(&self) -> Option<usize> {
        Some(2)
    }

    fn dofs_per_node(&self) -> &[Dof] {
        &[Dof::Tx, Dof::Ty, Dof::Rz]
    }

    fn close_mounted_string_step(&mut self) -> Option<usize> {
        for owner in [&mut self.id, &mut self.start, &mut self.end] {
            if !owner.is_empty() || owner.capacity() != 0 {
                let bytes = owner.capacity();
                *owner = String::new();
                return Some(bytes);
            }
        }
        None
    }

    fn mounted_next_string_bytes(&self) -> Option<usize> {
        [&self.id, &self.start, &self.end].into_iter().find(|owner| !owner.is_empty() || owner.capacity() != 0).map(|owner| owner.capacity())
    }

    fn mounted_strings_terminal_is_empty(&self) -> bool {
        self.id.capacity() == 0 && self.start.capacity() == 0 && self.end.capacity() == 0
    }

    fn stiffness_global(&self, ctx: &ElementContext) -> MatD {
        let (l, c, s) = segment_geometry(ctx);
        let axial_k = self.e * self.area / l;
        let bend_k = self.e * self.iy / l;
        let k_local = beam_local_stiffness(l, axial_k, bend_k);
        let t = beam_transform(c, s);
        t.transpose().matmul(&k_local).matmul(&t)
    }

    fn mounted_stiffness_cell(&self, ctx: &ElementContext, row: usize, column: usize) -> Option<f64> {
        if row >= 6 || column >= 6 || ctx.positions.len() != 2 {
            return None;
        }
        let (length, cosine, sine) = segment_geometry(ctx);
        if !length.is_finite() || length <= 0.0 {
            return None;
        }
        let axial = self.e * self.area / length;
        let bending = self.e * self.iy / length;
        let mut value = 0.0;
        for local_row in 0..6 {
            let left = beam_transform_cell(cosine, sine, local_row, row);
            if left == 0.0 {
                continue;
            }
            for local_column in 0..6 {
                let right = beam_transform_cell(cosine, sine, local_column, column);
                if right != 0.0 {
                    value += left * beam_local_stiffness_cell(length, axial, bending, local_row, local_column) * right;
                }
            }
        }
        Some(value)
    }

    fn equivalent_nodal_loads(&self, ctx: &ElementContext, udl: &MemberUdl) -> Option<VecD> {
        let (l, c, s) = segment_geometry(ctx);
        let wx_local = udl.wx * c + udl.wy * s;
        let wy_local = -udl.wx * s + udl.wy * c;
        let f_local = beam_local_udl(l, wx_local, wy_local);
        let t = beam_transform(c, s);
        Some(t.transpose().mul_vec(&f_local))
    }

    fn recover(&self, ctx: &ElementContext, u_local: &VecD, udl: Option<&MemberUdl>) -> ElementResult {
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
                crate::model::BeamStation { x, n: -n1, v: v1 + wy_local * x, m: -m1 + v1 * x + wy_local * x * x / 2.0 }
            })
            .collect();
        ElementResult::Beam { stations }
    }

    fn mass(&self, ctx: &ElementContext) -> Option<MatD> {
        let (l, c, s) = segment_geometry(ctx);
        let m_local = beam_local_mass(l, self.area, self.density);
        let t = beam_transform(c, s);
        Some(t.transpose().matmul(&m_local).matmul(&t))
    }

    /// 🌀️ Buckling geometric stiffness from the member's own axial force under `u_element` — same
    /// sign convention as `recover`'s `n` (tension-positive): `n = -k_local.mul_vec(u_loc).get(0)`.
    fn geometric_stiffness(&self, ctx: &ElementContext, u_element: &VecD) -> Option<MatD> {
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
    fn d_matrix(self, e: f64, nu: f64) -> MatD {
        match self {
            PlaneKind::Stress => d_matrix_plane_stress(e, nu),
            PlaneKind::Strain => d_matrix_plane_strain(e, nu),
        }
    }
}

fn plane_coords(ctx: &ElementContext) -> Vec<[f64; 2]> {
    ctx.positions.iter().map(|p| [p[0], p[1]]).collect()
}

/// 🧮️ Physical B-matrix + `weight * det(J)` at every Gauss point of a rule, shared by
/// `stiffness_global` and `recover` so both walk the SAME Gauss points in the SAME order.
fn plane_b_and_weights(coords: &[[f64; 2]], rule: &[(f64, f64, f64)], shape: impl Fn(f64, f64) -> Vec<[f64; 2]>) -> Vec<(MatD, f64)> {
    rule.iter()
        .map(|&(xi, eta, w)| {
            let d_n_param = shape(xi, eta);
            let (_, det_j, d_n_xy) = jacobian_2d(coords, &d_n_param);
            (b_matrix_plane(&d_n_xy), w * det_j)
        })
        .collect()
}

fn plane_stiffness(coords: &[[f64; 2]], rule: &[(f64, f64, f64)], shape: impl Fn(f64, f64) -> Vec<[f64; 2]>, d: &MatD, thickness: f64, ndof: usize) -> MatD {
    let mut ke = MatD::zeros(ndof, ndof);
    for (b, w) in plane_b_and_weights(coords, rule, shape) {
        ke.add_triple_product(&b, d, w * thickness);
    }
    ke
}

fn plane_recover(coords: &[[f64; 2]], rule: &[(f64, f64, f64)], shape: impl Fn(f64, f64) -> Vec<[f64; 2]>, d: &MatD, u_local: &VecD) -> ElementResult {
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
fn plane_mass(coords: &[[f64; 2]], rule: &[(f64, f64, f64)], shape_full: impl Fn(f64, f64) -> (Vec<f64>, Vec<[f64; 2]>), density: f64, thickness: f64, n_nodes: usize) -> MatD {
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
fn plane_geometric_stiffness(coords: &[[f64; 2]], rule: &[(f64, f64, f64)], shape: impl Fn(f64, f64) -> Vec<[f64; 2]>, d: &MatD, thickness: f64, u_local: &VecD, n_nodes: usize) -> MatD {
    let mut kg = MatD::zeros(n_nodes * 2, n_nodes * 2);
    for (xi, eta, w) in rule.iter().copied() {
        let d_n_param = shape(xi, eta);
        let (_, det_j, d_n_xy) = jacobian_2d(coords, &d_n_param);
        let b = b_matrix_plane(&d_n_xy);
        let eps = b.mul_vec(u_local);
        let sigma = d.mul_vec(&eps);
        let (sxx, syy, sxy) = (sigma.get(0), sigma.get(1), sigma.get(2));
        let scale = w * det_j * thickness;
        for (i, &[dix, diy]) in d_n_xy[..n_nodes].iter().enumerate() {
            for (j, &[djx, djy]) in d_n_xy[..n_nodes].iter().enumerate() {
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
    fn rule(&self) -> Vec<(f64, f64, f64)> {
        gauss_tri(1)
    }

    fn shape(xi: f64, eta: f64) -> Vec<[f64; 2]> {
        shape_tri3(xi, eta).1.to_vec()
    }

    fn shape_full(xi: f64, eta: f64) -> (Vec<f64>, Vec<[f64; 2]>) {
        let (n, dn) = shape_tri3(xi, eta);
        (n.to_vec(), dn.to_vec())
    }
}

impl Element for Tri3Cst {
    fn close_mounted_string_step(&mut self) -> Option<usize> {
        if !self.id.is_empty() || self.id.capacity() != 0 {
            let bytes = self.id.capacity();
            self.id = String::new();
            return Some(bytes);
        }
        for owner in &mut self.nodes {
            if !owner.is_empty() || owner.capacity() != 0 {
                let bytes = owner.capacity();
                *owner = String::new();
                return Some(bytes);
            }
        }
        None
    }

    fn mounted_next_string_bytes(&self) -> Option<usize> {
        std::iter::once(&self.id).chain(self.nodes.iter()).find(|owner| !owner.is_empty() || owner.capacity() != 0).map(|owner| owner.capacity())
    }

    fn mounted_strings_terminal_is_empty(&self) -> bool {
        self.id.capacity() == 0 && self.nodes.iter().all(|owner| owner.capacity() == 0)
    }

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
        Some(3)
    }

    fn dofs_per_node(&self) -> &[Dof] {
        &[Dof::Tx, Dof::Ty]
    }

    fn stiffness_global(&self, ctx: &ElementContext) -> MatD {
        let coords = plane_coords(ctx);
        let d = self.kind.d_matrix(self.e, self.nu);
        plane_stiffness(&coords, &self.rule(), Self::shape, &d, self.thickness, 6)
    }

    fn mounted_stiffness_cell(&self, ctx: &ElementContext, row: usize, column: usize) -> Option<f64> {
        if row >= 6 || column >= 6 || ctx.positions.len() != 3 {
            return None;
        }
        let coordinates = [[ctx.positions[0][0], ctx.positions[0][1]], [ctx.positions[1][0], ctx.positions[1][1]], [ctx.positions[2][0], ctx.positions[2][1]]];
        let dx_dxi = coordinates[1][0] - coordinates[0][0];
        let dx_deta = coordinates[2][0] - coordinates[0][0];
        let dy_dxi = coordinates[1][1] - coordinates[0][1];
        let dy_deta = coordinates[2][1] - coordinates[0][1];
        let determinant = dx_dxi * dy_deta - dx_deta * dy_dxi;
        if !determinant.is_finite() || determinant.abs() <= f64::EPSILON {
            return None;
        }
        let parametric = [[-1.0, -1.0], [1.0, 0.0], [0.0, 1.0]];
        let derivatives = std::array::from_fn::<_, 3, _>(|index| {
            let derivative = parametric[index];
            [(dy_deta * derivative[0] - dy_dxi * derivative[1]) / determinant, (-dx_deta * derivative[0] + dx_dxi * derivative[1]) / determinant]
        });
        let b = |strain: usize, dof: usize| {
            let derivative = derivatives[dof / 2];
            match (strain, dof % 2) {
                (0, 0) => derivative[0],
                (1, 1) => derivative[1],
                (2, 0) => derivative[1],
                (2, 1) => derivative[0],
                _ => 0.0,
            }
        };
        let constitutive = |left: usize, right: usize| match self.kind {
            PlaneKind::Stress => {
                let factor = self.e / (1.0 - self.nu * self.nu);
                match (left, right) {
                    (0, 0) | (1, 1) => factor,
                    (0, 1) | (1, 0) => factor * self.nu,
                    (2, 2) => factor * (1.0 - self.nu) / 2.0,
                    _ => 0.0,
                }
            }
            PlaneKind::Strain => {
                let factor = self.e / ((1.0 + self.nu) * (1.0 - 2.0 * self.nu));
                match (left, right) {
                    (0, 0) | (1, 1) => factor * (1.0 - self.nu),
                    (0, 1) | (1, 0) => factor * self.nu,
                    (2, 2) => factor * (1.0 - 2.0 * self.nu) / 2.0,
                    _ => 0.0,
                }
            }
        };
        let mut value = 0.0;
        for left in 0..3 {
            for right in 0..3 {
                value += b(left, row) * constitutive(left, right) * b(right, column);
            }
        }
        Some(value * 0.5 * determinant * self.thickness)
    }

    fn recover(&self, ctx: &ElementContext, u_local: &VecD, _udl: Option<&MemberUdl>) -> ElementResult {
        let coords = plane_coords(ctx);
        let d = self.kind.d_matrix(self.e, self.nu);
        plane_recover(&coords, &self.rule(), Self::shape, &d, u_local)
    }

    /// 🏋️ Consistent CST mass `ρtA/12·[[2,1,1],[1,2,1],[1,1,2]]` (both directions) — Tri3's shape
    /// functions ARE the area coordinates (`Ni=Li`), so `Ni·Nj` is a complete quadratic in area
    /// coordinates, integrated EXACTLY by the degree-2-precision 3-point rule (own stiffness rule
    /// `self.rule()` is only 1-point, adequate for the constant-strain stiffness but NOT exact here).
    fn mass(&self, ctx: &ElementContext) -> Option<MatD> {
        let coords = plane_coords(ctx);
        Some(plane_mass(&coords, &gauss_tri(3), Self::shape_full, self.density, self.thickness, 3))
    }

    fn geometric_stiffness(&self, ctx: &ElementContext, u_element: &VecD) -> Option<MatD> {
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
    fn rule(&self) -> Vec<(f64, f64, f64)> {
        gauss_tri(3)
    }

    /// 🎯️ A 7-point rule (degree-5 precision) for mass — Tri6's quadratic shape functions make
    /// `Ni·Nj` a degree-4 polynomial, which the element's own 3-point (degree-2) stiffness rule
    /// under-integrates.
    fn mass_rule() -> Vec<(f64, f64, f64)> {
        gauss_tri(7)
    }

    fn shape(xi: f64, eta: f64) -> Vec<[f64; 2]> {
        shape_tri6(xi, eta).1.to_vec()
    }

    fn shape_full(xi: f64, eta: f64) -> (Vec<f64>, Vec<[f64; 2]>) {
        let (n, dn) = shape_tri6(xi, eta);
        (n.to_vec(), dn.to_vec())
    }
}

impl Element for Tri6Lst {
    fn id(&self) -> &str {
        &self.id
    }

    fn node_ids(&self) -> Vec<String> {
        self.nodes.to_vec()
    }

    fn dofs_per_node(&self) -> &[Dof] {
        &[Dof::Tx, Dof::Ty]
    }

    fn stiffness_global(&self, ctx: &ElementContext) -> MatD {
        let coords = plane_coords(ctx);
        let d = self.kind.d_matrix(self.e, self.nu);
        plane_stiffness(&coords, &self.rule(), Self::shape, &d, self.thickness, 12)
    }

    fn recover(&self, ctx: &ElementContext, u_local: &VecD, _udl: Option<&MemberUdl>) -> ElementResult {
        let coords = plane_coords(ctx);
        let d = self.kind.d_matrix(self.e, self.nu);
        plane_recover(&coords, &self.rule(), Self::shape, &d, u_local)
    }

    fn mass(&self, ctx: &ElementContext) -> Option<MatD> {
        let coords = plane_coords(ctx);
        Some(plane_mass(&coords, &Self::mass_rule(), Self::shape_full, self.density, self.thickness, 6))
    }

    fn geometric_stiffness(&self, ctx: &ElementContext, u_element: &VecD) -> Option<MatD> {
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
    fn rule(&self) -> Vec<(f64, f64, f64)> {
        gauss_quad(2)
    }

    fn shape(xi: f64, eta: f64) -> Vec<[f64; 2]> {
        shape_quad4(xi, eta).1.to_vec()
    }

    fn shape_full(xi: f64, eta: f64) -> (Vec<f64>, Vec<[f64; 2]>) {
        let (n, dn) = shape_quad4(xi, eta);
        (n.to_vec(), dn.to_vec())
    }
}

impl Element for Quad4 {
    fn id(&self) -> &str {
        &self.id
    }

    fn node_ids(&self) -> Vec<String> {
        self.nodes.to_vec()
    }

    fn dofs_per_node(&self) -> &[Dof] {
        &[Dof::Tx, Dof::Ty]
    }

    fn stiffness_global(&self, ctx: &ElementContext) -> MatD {
        let coords = plane_coords(ctx);
        let d = self.kind.d_matrix(self.e, self.nu);
        plane_stiffness(&coords, &self.rule(), Self::shape, &d, self.thickness, 8)
    }

    fn recover(&self, ctx: &ElementContext, u_local: &VecD, _udl: Option<&MemberUdl>) -> ElementResult {
        let coords = plane_coords(ctx);
        let d = self.kind.d_matrix(self.e, self.nu);
        plane_recover(&coords, &self.rule(), Self::shape, &d, u_local)
    }

    /// 🏋️ Consistent bilinear mass — the same 2x2 rule as stiffness under-integrates the biquadratic
    /// `Ni·Nj` product for a non-rectangular quad, so mass uses the fuller 3x3 rule instead.
    fn mass(&self, ctx: &ElementContext) -> Option<MatD> {
        let coords = plane_coords(ctx);
        Some(plane_mass(&coords, &gauss_quad(3), Self::shape_full, self.density, self.thickness, 4))
    }

    fn geometric_stiffness(&self, ctx: &ElementContext, u_element: &VecD) -> Option<MatD> {
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
    fn rule(&self) -> Vec<(f64, f64, f64)> {
        gauss_quad(3)
    }

    fn shape(xi: f64, eta: f64) -> Vec<[f64; 2]> {
        shape_quad8(xi, eta).1.to_vec()
    }

    fn shape_full(xi: f64, eta: f64) -> (Vec<f64>, Vec<[f64; 2]>) {
        let (n, dn) = shape_quad8(xi, eta);
        (n.to_vec(), dn.to_vec())
    }
}

impl Element for Quad8 {
    fn id(&self) -> &str {
        &self.id
    }

    fn node_ids(&self) -> Vec<String> {
        self.nodes.to_vec()
    }

    fn dofs_per_node(&self) -> &[Dof] {
        &[Dof::Tx, Dof::Ty]
    }

    fn stiffness_global(&self, ctx: &ElementContext) -> MatD {
        let coords = plane_coords(ctx);
        let d = self.kind.d_matrix(self.e, self.nu);
        plane_stiffness(&coords, &self.rule(), Self::shape, &d, self.thickness, 16)
    }

    fn recover(&self, ctx: &ElementContext, u_local: &VecD, _udl: Option<&MemberUdl>) -> ElementResult {
        let coords = plane_coords(ctx);
        let d = self.kind.d_matrix(self.e, self.nu);
        plane_recover(&coords, &self.rule(), Self::shape, &d, u_local)
    }

    fn mass(&self, ctx: &ElementContext) -> Option<MatD> {
        let coords = plane_coords(ctx);
        Some(plane_mass(&coords, &self.rule(), Self::shape_full, self.density, self.thickness, 8))
    }

    fn geometric_stiffness(&self, ctx: &ElementContext, u_element: &VecD) -> Option<MatD> {
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

fn dkt_edge(pi: [f64; 2], pj: [f64; 2]) -> DktEdge {
    let x_ij = pi[0] - pj[0];
    let y_ij = pi[1] - pj[1];
    let l2 = x_ij * x_ij + y_ij * y_ij;
    DktEdge { a: -x_ij / l2, b: 0.75 * x_ij * y_ij / l2, c: (0.25 * x_ij * x_ij - 0.5 * y_ij * y_ij) / l2, d: -y_ij / l2, e: (0.25 * y_ij * y_ij - 0.5 * x_ij * x_ij) / l2 }
}

/// 🧱️ Bending constitutive matrix `(E t³)/(12(1-ν²)) [[1,ν,0],[ν,1,0],[0,0,(1-ν)/2]]`, shared by
/// `PlateDkt` and (via `crate::elements2d::d_matrix_plate`) `elements3d::ShellFacet3`'s bending part.
pub(crate) fn d_matrix_plate(e: f64, nu: f64, thickness: f64) -> MatD {
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
pub(crate) fn dkt_b_matrix(coords: &[[f64; 2]; 3], xi: f64, eta: f64) -> MatD {
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
    fn coords(ctx: &ElementContext) -> [[f64; 2]; 3] {
        [[ctx.positions[0][0], ctx.positions[0][1]], [ctx.positions[1][0], ctx.positions[1][1]], [ctx.positions[2][0], ctx.positions[2][1]]]
    }
}

impl Element for PlateDkt {
    fn id(&self) -> &str {
        &self.id
    }

    fn node_ids(&self) -> Vec<String> {
        self.nodes.to_vec()
    }

    fn dofs_per_node(&self) -> &[Dof] {
        &[Dof::Tz, Dof::Rx, Dof::Ry]
    }

    fn stiffness_global(&self, ctx: &ElementContext) -> MatD {
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

    fn recover(&self, ctx: &ElementContext, u_local: &VecD, _udl: Option<&MemberUdl>) -> ElementResult {
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
    fn mass(&self, ctx: &ElementContext) -> Option<MatD> {
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

    fn assert_mounted_stiffness_cells_match_batch(element: &dyn Element, context: &ElementContext, tolerance: f64) {
        let batch = element.stiffness_global(context);
        for row in 0..batch.rows {
            for column in 0..batch.cols {
                let mounted = element.mounted_stiffness_cell(context, row, column).expect("mounted fixed-schema cell");
                let scale = batch.get(row, column).abs().max(1.0);
                assert!((mounted - batch.get(row, column)).abs() <= tolerance * scale, "cell ({row},{column}) mounted={mounted} batch={}", batch.get(row, column));
            }
        }
    }

    #[test]
    fn p6h_mounted_element_fixed_schema_cells_match_batch_and_reject_maximum_plus_one() {
        let line = ElementContext { positions: vec![[0.0, 0.0, 0.0], [2.0, 1.0, 0.0]] };
        let bar = Bar2 { id: "bar".into(), start: "a".into(), end: "b".into(), e: 210e9, area: 0.01, density: 0.0 };
        assert_mounted_stiffness_cells_match_batch(&bar, &line, 1e-12);
        assert_eq!(bar.mounted_stiffness_cell(&line, 4, 0), None);

        let beam = BeamEb2 { id: "beam".into(), start: "a".into(), end: "b".into(), e: 210e9, area: 0.01, iy: 8.0e-6, density: 0.0 };
        assert_mounted_stiffness_cells_match_batch(&beam, &line, 1e-12);
        assert_eq!(beam.mounted_stiffness_cell(&line, 6, 0), None);

        let triangle_context = ElementContext { positions: vec![[0.0, 0.0, 0.0], [2.0, 0.0, 0.0], [0.25, 1.5, 0.0]] };
        let triangle = Tri3Cst { id: "triangle".into(), nodes: ["a".into(), "b".into(), "c".into()], e: 30e9, nu: 0.2, thickness: 0.3, kind: PlaneKind::Stress, density: 0.0 };
        assert_mounted_stiffness_cells_match_batch(&triangle, &triangle_context, 1e-12);
        assert_eq!(triangle.mounted_stiffness_cell(&triangle_context, 6, 0), None);
    }
    use crate::model::{solve_linear_static, Model, NodalLoad, Node, Support};

    /// 🪢️ Headless (no document layer) axial elongation check: δ = FL/EA, N = F.
    #[test]
    fn bar2_axial_matches_hand_calc() {
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
    #[test]
    fn beam_eb2_cantilever_matches_hand_calc() {
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

    /// 🏗️ Straight `BeamEb2` chain of `n` equal elements spanning `l` (nodes `n0..nN`), carrying a
    /// downward UDL `w` as a member load on every element, with the given end restraints.
    fn beam_udl_span(n: usize, l: f64, w: f64, base: Vec<Dof>, tip: Vec<Dof>) -> crate::model::StaticResult {
        let (e, iy, area) = (200e9, 1e-5, 0.01);
        let id = |i: usize| format!("n{i}");
        let nodes: Vec<Node> = (0..=n).map(|i| Node { id: id(i), pos: [l * i as f64 / n as f64, 0.0, 0.0] }).collect();
        let elements: Vec<Elements> = (0..n).map(|i| BeamEb2 { id: format!("e{i}"), start: id(i), end: id(i + 1), e, area, iy, density: 0.0 }.into()).collect();
        let member_loads = (0..n).map(|i| (format!("e{i}"), MemberUdl { wx: 0.0, wy: -w, wz: 0.0 })).collect();
        let supports = vec![Support { node_id: id(0), fixed: base }, Support { node_id: id(n), fixed: tip }];
        let model = Model { nodes, elements, supports, nodal_loads: vec![], member_loads };
        solve_linear_static(&model).expect("udl span solves")
    }

    /// 🏗️ Simply supported beam under a UDL, four `BeamEb2` elements: midspan deflection
    /// `5wL⁴/384EI`, end rotations `wL³/24EI` and support reactions `wL/2`. Consistent nodal loads
    /// make cubic-Hermite nodal values EXACT for a UDL, so all three are asserted to 1e-9 — an
    /// independent numpy direct-stiffness solve (`🔨️w7-kernel-references.py` section 6) reproduces
    /// the closed form to 3.3e-15.
    #[test]
    fn beam_eb2_simply_supported_udl_matches_closed_form() {
        let (l, w) = (6.0_f64, 2000.0_f64);
        let (e, iy) = (200e9_f64, 1e-5_f64);
        let result = beam_udl_span(4, l, w, vec![Dof::Tx, Dof::Ty], vec![Dof::Ty]);

        let midspan = result.displacements.iter().find(|d| d.node_id == "n2").unwrap();
        let expected_deflection = -5.0 * w * l.powi(4) / (384.0 * e * iy);
        assert!((midspan.values[Dof::Ty.index()] - expected_deflection).abs() / expected_deflection.abs() < 1e-9, "midspan {} vs {expected_deflection}", midspan.values[Dof::Ty.index()]);

        let expected_rotation = w * l.powi(3) / (24.0 * e * iy);
        for (node_id, sign) in [("n0", -1.0), ("n4", 1.0)] {
            let rotation = result.displacements.iter().find(|d| d.node_id == node_id).unwrap().values[Dof::Rz.index()];
            assert!((rotation - sign * expected_rotation).abs() / expected_rotation < 1e-9, "{node_id} rotation {rotation} vs {}", sign * expected_rotation);
        }
        for node_id in ["n0", "n4"] {
            let reaction = result.reactions.iter().find(|r| r.node_id == node_id && r.dof == Dof::Ty).unwrap().value;
            assert!((reaction - w * l / 2.0).abs() / (w * l / 2.0) < 1e-9, "{node_id} reaction {reaction} vs {}", w * l / 2.0);
        }
    }

    /// 🏗️ Propped cantilever under a UDL, four `BeamEb2` elements — the classic statically
    /// indeterminate reaction set `5wL/8` (fixed end), `3wL/8` (prop) and `wL²/8` (fixed-end moment),
    /// exact for consistent nodal loads and reproduced to machine precision by the independent numpy
    /// direct-stiffness solve in `🔨️w7-kernel-references.py` section 6 (7500 / 4500 / 9000 N, N·m).
    #[test]
    fn beam_eb2_propped_cantilever_reactions_match_closed_form() {
        let (l, w) = (6.0_f64, 2000.0_f64);
        let result = beam_udl_span(4, l, w, vec![Dof::Tx, Dof::Ty, Dof::Rz], vec![Dof::Ty]);
        let reaction = |node_id: &str, dof: Dof| result.reactions.iter().find(|r| r.node_id == node_id && r.dof == dof).unwrap().value;
        let expectations = [("n0", Dof::Ty, 5.0 * w * l / 8.0), ("n4", Dof::Ty, 3.0 * w * l / 8.0), ("n0", Dof::Rz, w * l * l / 8.0)];
        for (node_id, dof, expected) in expectations {
            let actual = reaction(node_id, dof);
            assert!((actual - expected).abs() / expected < 1e-9, "{node_id} {dof:?} reaction {actual} vs {expected}");
        }
    }

    /// 🏗️ Single-bay portal frame (6 m span, 4 m columns, both bases fully fixed) under a 15 kN
    /// lateral load at the windward eaves — one `BeamEb2` per member, so the whole global transform
    /// path (vertical columns) is exercised. Every asserted number comes from the independent
    /// numpy/scipy direct-stiffness solve in `🔨️w7-kernel-references.py` section 6, whose own global
    /// moment equilibrium about the origin closes to 9.7e-10 N·m.
    #[test]
    fn beam_eb2_portal_frame_sway_matches_stiffness_method() {
        let (h, span, load) = (4.0_f64, 6.0_f64, 15000.0_f64);
        let (ec, ac, ic) = (210e9, 0.008, 8.0e-5);
        let (eb, ab, ib) = (210e9, 0.012, 2.0e-4);
        let model = Model {
            nodes: vec![
                Node { id: "base_left".into(), pos: [0.0, 0.0, 0.0] },
                Node { id: "eaves_left".into(), pos: [0.0, h, 0.0] },
                Node { id: "eaves_right".into(), pos: [span, h, 0.0] },
                Node { id: "base_right".into(), pos: [span, 0.0, 0.0] },
            ],
            elements: vec![
                BeamEb2 { id: "col_left".into(), start: "base_left".into(), end: "eaves_left".into(), e: ec, area: ac, iy: ic, density: 0.0 }.into(),
                BeamEb2 { id: "beam".into(), start: "eaves_left".into(), end: "eaves_right".into(), e: eb, area: ab, iy: ib, density: 0.0 }.into(),
                BeamEb2 { id: "col_right".into(), start: "base_right".into(), end: "eaves_right".into(), e: ec, area: ac, iy: ic, density: 0.0 }.into(),
            ],
            supports: vec![
                Support { node_id: "base_left".into(), fixed: vec![Dof::Tx, Dof::Ty, Dof::Rz] },
                Support { node_id: "base_right".into(), fixed: vec![Dof::Tx, Dof::Ty, Dof::Rz] },
            ],
            nodal_loads: vec![NodalLoad { node_id: "eaves_left".into(), dof: Dof::Tx, value: load }],
            member_loads: vec![],
        };
        let result = solve_linear_static(&model).expect("portal frame solves");

        let displacement = |node_id: &str, dof: Dof| result.displacements.iter().find(|d| d.node_id == node_id).unwrap().values[dof.index()];
        let reaction = |node_id: &str, dof: Dof| result.reactions.iter().find(|r| r.node_id == node_id && r.dof == dof).unwrap().value;
        let references = [
            ("sway left", displacement("eaves_left", Dof::Tx), 3.045764339376e-3),
            ("sway right", displacement("eaves_right", Dof::Tx), 3.027946678835e-3),
            ("eaves rotation left", displacement("eaves_left", Dof::Rz), -3.297738248139e-4),
            ("eaves rotation right", displacement("eaves_right", Dof::Rz), -3.261293033395e-4),
            ("base shear left", reaction("base_left", Dof::Tx), -7516.582572708),
            ("base shear right", reaction("base_right", Dof::Tx), -7483.417427292),
            ("base uplift left", reaction("base_left", Dof::Ty), -4540.867810293),
            ("base uplift right", reaction("base_right", Dof::Ty), 4540.867810293),
            ("base moment left", reaction("base_left", Dof::Rz), 16418.215209634),
            ("base moment right", reaction("base_right", Dof::Rz), 16336.577928609),
        ];
        for (label, actual, expected) in references {
            assert!((actual - expected).abs() / expected.abs() < 1e-9, "{label}: {actual} vs scipy {expected}");
        }
        let base_shear_sum = reaction("base_left", Dof::Tx) + reaction("base_right", Dof::Tx);
        assert!((base_shear_sum + load).abs() / load < 1e-9, "base shears {base_shear_sum} must balance the applied {load}");
        let global_moment = reaction("base_left", Dof::Rz) + reaction("base_right", Dof::Rz) + reaction("base_right", Dof::Ty) * span - load * h;
        assert!(global_moment.abs() / (load * h) < 1e-9, "global moment residual {global_moment}");
    }

    /// 🌀️ Rigid-body test: a pure translation (no relative deformation) must produce zero internal
    /// force — `Ke * rigid_translation ≈ 0`. Catches sign/assembly bugs that a single load case might not.
    #[test]
    fn beam_eb2_rigid_translation_gives_zero_force() {
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
    #[test]
    fn bar2_mass_matches_hand_calc() {
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
    #[test]
    fn bar2_mass_total_equals_rho_a_l() {
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
    #[test]
    fn beam_eb2_mass_axial_block_sums_to_total_mass() {
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
    #[test]
    fn beam_eb2_geometric_stiffness_rigid_translation_gives_zero_force() {
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
    #[test]
    fn beam_eb2_geometric_stiffness_is_symmetric_and_scales_with_axial_force() {
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
    #[test]
    fn bar2_equivalent_nodal_loads_matches_wl_over_2() {
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
    #[test]
    fn bar2_geometric_stiffness_rigid_translation_gives_zero_force_and_is_symmetric() {
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
    fn linear_field_u_local(coords: &[[f64; 2]], a: (f64, f64, f64), b: (f64, f64, f64)) -> VecD {
        let mut v = Vec::with_capacity(coords.len() * 2);
        for &[x, y] in coords {
            v.push(a.0 + a.1 * x + a.2 * y);
            v.push(b.0 + b.1 * x + b.2 * y);
        }
        VecD::from_vec(v)
    }

    fn rigid_translation_u_local(n_nodes: usize, dx: f64, dy: f64) -> VecD {
        let mut v = Vec::with_capacity(n_nodes * 2);
        for _ in 0..n_nodes {
            v.push(dx);
            v.push(dy);
        }
        VecD::from_vec(v)
    }

    fn assert_plane_gauss_matches(gauss: &[PlaneStress], expected: (f64, f64, f64), tol: f64) {
        for gp in gauss {
            assert!((gp.sxx - expected.0).abs() < tol, "sxx {} vs {}", gp.sxx, expected.0);
            assert!((gp.syy - expected.1).abs() < tol, "syy {} vs {}", gp.syy, expected.1);
            assert!((gp.sxy - expected.2).abs() < tol, "sxy {} vs {}", gp.sxy, expected.2);
        }
    }

    fn assert_rigid_body_gives_zero_force(ke: &MatD, u_local: &VecD) {
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

    fn expected_stress(kind: PlaneKind) -> (f64, f64, f64) {
        let d = match kind {
            PlaneKind::Stress => d_matrix_plane_stress(E, NU),
            PlaneKind::Strain => d_matrix_plane_strain(E, NU),
        };
        let strain = VecD::from_vec(vec![A.1, B.2, A.2 + B.1]);
        let sigma = d.mul_vec(&strain);
        (sigma.get(0), sigma.get(1), sigma.get(2))
    }

    fn ctx_of(coords: &[[f64; 2]]) -> ElementContext {
        ElementContext { positions: coords.iter().map(|&[x, y]| [x, y, 0.0]).collect() }
    }

    #[test]
    fn tri3_cst_patch_test_reproduces_linear_field() {
        let coords = [[0.0, 0.0], [2.0, 0.1], [0.2, 1.8]];
        let el = Tri3Cst { id: "t".into(), nodes: ["a".into(), "b".into(), "c".into()], e: E, nu: NU, thickness: 1.0, kind: PlaneKind::Stress, density: 0.0 };
        let ctx = ctx_of(&coords);
        let u = linear_field_u_local(&coords, A, B);
        let ElementResult::Plane { gauss } = el.recover(&ctx, &u, None) else { panic!("expected plane result") };
        assert_eq!(gauss.len(), 1);
        assert_plane_gauss_matches(&gauss, expected_stress(PlaneKind::Stress), 1e-8);
    }

    #[test]
    fn tri3_cst_rigid_translation_gives_zero_force() {
        let coords = [[0.0, 0.0], [2.0, 0.1], [0.2, 1.8]];
        let el = Tri3Cst { id: "t".into(), nodes: ["a".into(), "b".into(), "c".into()], e: E, nu: NU, thickness: 1.0, kind: PlaneKind::Stress, density: 0.0 };
        let ctx = ctx_of(&coords);
        let ke = el.stiffness_global(&ctx);
        assert_rigid_body_gives_zero_force(&ke, &rigid_translation_u_local(3, 1.5, -2.3));
    }

    #[test]
    fn tri6_lst_patch_test_reproduces_linear_field() {
        let coords = [[0.0, 0.0], [2.0, 0.1], [0.2, 1.8], [1.0, 0.05], [1.1, 0.95], [0.1, 0.9]];
        let el = Tri6Lst { id: "t".into(), nodes: ["a".into(), "b".into(), "c".into(), "d".into(), "e".into(), "f".into()], e: E, nu: NU, thickness: 1.0, kind: PlaneKind::Stress, density: 0.0 };
        let ctx = ctx_of(&coords);
        let u = linear_field_u_local(&coords, A, B);
        let ElementResult::Plane { gauss } = el.recover(&ctx, &u, None) else { panic!("expected plane result") };
        assert_eq!(gauss.len(), 3);
        assert_plane_gauss_matches(&gauss, expected_stress(PlaneKind::Stress), 1e-8);
    }

    #[test]
    fn tri6_lst_rigid_translation_gives_zero_force() {
        let coords = [[0.0, 0.0], [2.0, 0.1], [0.2, 1.8], [1.0, 0.05], [1.1, 0.95], [0.1, 0.9]];
        let el = Tri6Lst { id: "t".into(), nodes: ["a".into(), "b".into(), "c".into(), "d".into(), "e".into(), "f".into()], e: E, nu: NU, thickness: 1.0, kind: PlaneKind::Stress, density: 0.0 };
        let ctx = ctx_of(&coords);
        let ke = el.stiffness_global(&ctx);
        assert_rigid_body_gives_zero_force(&ke, &rigid_translation_u_local(6, 1.5, -2.3));
    }

    #[test]
    fn quad4_patch_test_reproduces_linear_field() {
        let coords = [[0.0, 0.0], [3.0, 0.2], [3.3, 2.5], [0.2, 2.3]];
        let el = Quad4 { id: "q".into(), nodes: ["a".into(), "b".into(), "c".into(), "d".into()], e: E, nu: NU, thickness: 1.0, kind: PlaneKind::Strain, density: 0.0 };
        let ctx = ctx_of(&coords);
        let u = linear_field_u_local(&coords, A, B);
        let ElementResult::Plane { gauss } = el.recover(&ctx, &u, None) else { panic!("expected plane result") };
        assert_eq!(gauss.len(), 4);
        assert_plane_gauss_matches(&gauss, expected_stress(PlaneKind::Strain), 1e-8);
    }

    #[test]
    fn quad4_rigid_translation_gives_zero_force() {
        let coords = [[0.0, 0.0], [3.0, 0.2], [3.3, 2.5], [0.2, 2.3]];
        let el = Quad4 { id: "q".into(), nodes: ["a".into(), "b".into(), "c".into(), "d".into()], e: E, nu: NU, thickness: 1.0, kind: PlaneKind::Strain, density: 0.0 };
        let ctx = ctx_of(&coords);
        let ke = el.stiffness_global(&ctx);
        assert_rigid_body_gives_zero_force(&ke, &rigid_translation_u_local(4, 1.5, -2.3));
    }

    #[test]
    fn quad8_patch_test_reproduces_linear_field() {
        let coords = [[0.0, 0.0], [3.0, 0.2], [3.3, 2.5], [0.2, 2.3], [1.5, 0.1], [3.15, 1.35], [1.75, 2.4], [0.1, 1.15]];
        let el = Quad8 { id: "q8".into(), nodes: ["a".into(), "b".into(), "c".into(), "d".into(), "e".into(), "f".into(), "g".into(), "h".into()], e: E, nu: NU, thickness: 1.0, kind: PlaneKind::Stress, density: 0.0 };
        let ctx = ctx_of(&coords);
        let u = linear_field_u_local(&coords, A, B);
        let ElementResult::Plane { gauss } = el.recover(&ctx, &u, None) else { panic!("expected plane result") };
        assert_eq!(gauss.len(), 9, "quad8 must use the full 3x3 rule, not 2x2");
        assert_plane_gauss_matches(&gauss, expected_stress(PlaneKind::Stress), 1e-8);
    }

    #[test]
    fn quad8_rigid_translation_gives_zero_force() {
        let coords = [[0.0, 0.0], [3.0, 0.2], [3.3, 2.5], [0.2, 2.3], [1.5, 0.1], [3.15, 1.35], [1.75, 2.4], [0.1, 1.15]];
        let el = Quad8 { id: "q8".into(), nodes: ["a".into(), "b".into(), "c".into(), "d".into(), "e".into(), "f".into(), "g".into(), "h".into()], e: E, nu: NU, thickness: 1.0, kind: PlaneKind::Stress, density: 0.0 };
        let ctx = ctx_of(&coords);
        let ke = el.stiffness_global(&ctx);
        assert_rigid_body_gives_zero_force(&ke, &rigid_translation_u_local(8, 1.5, -2.3));
    }

    /// 🌀️ Cook's membrane mesh — the classic tapered cantilever panel `(0,0)-(48,44)-(48,60)-(0,44)`
    /// bilinearly blended into an `n x n` grid of `Quad4` (or, with `quadratic`, `Quad8`) elements,
    /// clamped along `x=0`, carrying a uniform edge traction of unit TOTAL shear on `x=48` applied as
    /// the consistent nodal loads for the element's own edge order (`h/2,h/2` linear, `h/6,2h/3,h/6`
    /// quadratic). Returns the vertical deflection at the loaded edge's midpoint `(48,52)`.
    fn cooks_membrane_tip_deflection(n: usize, quadratic: bool) -> f64 {
        let steps = if quadratic { 2 * n } else { n };
        let corners = [(0.0_f64, 0.0_f64), (48.0, 44.0), (48.0, 60.0), (0.0, 44.0)];
        let blend = |i: usize, j: usize| {
            let (r, s) = (i as f64 / steps as f64, j as f64 / steps as f64);
            let w = [(1.0 - r) * (1.0 - s), r * (1.0 - s), r * s, (1.0 - r) * s];
            [(0..4).map(|k| w[k] * corners[k].0).sum::<f64>(), (0..4).map(|k| w[k] * corners[k].1).sum::<f64>(), 0.0]
        };
        let id = |i: usize, j: usize| format!("n{i}_{j}");

        let mut nodes = Vec::new();
        for i in 0..=steps {
            for j in 0..=steps {
                if !(quadratic && i % 2 == 1 && j % 2 == 1) {
                    nodes.push(Node { id: id(i, j), pos: blend(i, j) });
                }
            }
        }

        let (e, nu, t) = (1.0, 1.0 / 3.0, 1.0);
        let step = if quadratic { 2 } else { 1 };
        let mut elements: Vec<Elements> = Vec::new();
        for a in 0..n {
            for b in 0..n {
                let (i, j) = (step * a, step * b);
                if quadratic {
                    elements.push(Quad8 { id: format!("q{a}_{b}"), nodes: [id(i, j), id(i + 2, j), id(i + 2, j + 2), id(i, j + 2), id(i + 1, j), id(i + 2, j + 1), id(i + 1, j + 2), id(i, j + 1)], e, nu, thickness: t, kind: PlaneKind::Stress, density: 0.0 }.into());
                } else {
                    elements.push(Quad4 { id: format!("q{a}_{b}"), nodes: [id(i, j), id(i + 1, j), id(i + 1, j + 1), id(i, j + 1)], e, nu, thickness: t, kind: PlaneKind::Stress, density: 0.0 }.into());
                }
            }
        }

        let supports = (0..=steps).map(|j| Support { node_id: id(0, j), fixed: vec![Dof::Tx, Dof::Ty] }).collect();
        let (h, traction) = (16.0 / n as f64, 1.0 / 16.0);
        let mut lumped: std::collections::HashMap<String, f64> = std::collections::HashMap::new();
        for b in 0..n {
            let j = step * b;
            if quadratic {
                *lumped.entry(id(steps, j)).or_insert(0.0) += traction * h / 6.0;
                *lumped.entry(id(steps, j + 2)).or_insert(0.0) += traction * h / 6.0;
                *lumped.entry(id(steps, j + 1)).or_insert(0.0) += traction * h * 2.0 / 3.0;
            } else {
                *lumped.entry(id(steps, j)).or_insert(0.0) += traction * h / 2.0;
                *lumped.entry(id(steps, j + 1)).or_insert(0.0) += traction * h / 2.0;
            }
        }
        let nodal_loads = lumped.into_iter().map(|(node_id, value)| NodalLoad { node_id, dof: Dof::Ty, value }).collect();

        let model = Model { nodes, elements, supports, nodal_loads, member_loads: vec![] };
        let result = solve_linear_static(&model).expect("cook's membrane mesh solves");
        result.displacements.iter().find(|d| d.node_id == id(steps, steps / 2)).unwrap().values[Dof::Ty.index()]
    }

    /// 🌀️ Cook's membrane (E=1, ν=1/3, t=1, unit total tip shear) against the tip deflection
    /// scikit-fem 12.0.2 computes on the IDENTICAL mesh — `ElementQuad1` at `intorder=2` (2x2 Gauss,
    /// matching `Quad4::rule`) and `ElementQuadS2` at `intorder=4` (3x3, matching `Quad8::rule`) —
    /// and against the published converged reference 23.96. `🔨️w7-kernel-references.py` section 3
    /// reports scikit-fem and an independent hand-assembled numpy kernel agreeing to <1e-11 on every
    /// mesh, and the Quad4 sequence 11.845 / 18.299 / 22.079 / 23.430 / 23.818 (2x2 … 32x32)
    /// reproducing the classical Cook convergence table.
    #[test]
    fn quad4_and_quad8_cooks_membrane_match_reference_tip_deflection() {
        let quad4_coarse = cooks_membrane_tip_deflection(4, false);
        let quad4_fine = cooks_membrane_tip_deflection(8, false);
        let quad8_coarse = cooks_membrane_tip_deflection(4, true);
        let references = [("quad4 4x4", quad4_coarse, 18.299165832569), ("quad4 8x8", quad4_fine, 22.079183389482), ("quad8 4x4", quad8_coarse, 23.708288809430)];
        for (label, actual, reference) in references {
            assert!((actual - reference).abs() / reference < 1e-6, "{label}: {actual} vs scikit-fem {reference}");
        }
        assert!(quad4_coarse < quad4_fine && quad4_fine < 23.96, "Quad4 must converge to 23.96 from below, got {quad4_coarse} then {quad4_fine}");
        assert!((quad8_coarse - 23.96).abs() / 23.96 < 0.015, "Quad8 4x4 {quad8_coarse} must be within 1.5 % of the converged 23.96 (it lands at 1.05 %)");
    }

    /// 📏️ MacNeal-Harder straight-cantilever distortion sensitivity for `Quad4`: the standard
    /// `L=6, h=0.2, t=0.1, E=1e7, ν=0.3` strip meshed as SIX elements in three shapes — rectangular,
    /// 45° parallelogram, and 45° alternating trapezoid — under a unit tip shear, against the
    /// beam-theory tip deflection 0.1081. Reference tip values from `🔨️w7-kernel-references.py`
    /// section 7, where scikit-fem's `ElementQuad1` and an independent numpy kernel agree to <1e-13.
    /// A plain fully-integrated bilinear quad shear-locks hard here (MacNeal-Harder's published
    /// 0.904/0.080/0.071 row is for a QUAD4 WITH incompatible modes, which this kernel does not
    /// have), so the gate is the exact same-mesh value plus the distortion ORDERING: both distorted
    /// meshes must lose at least a further factor of three against the rectangular one.
    #[test]
    fn quad4_macneal_harder_distorted_cantilever_matches_reference_sensitivity() {
        let cases = [("rectangular", 0.010088000000_f64), ("parallelogram", 0.002613057737), ("trapezoidal", 0.002908744060)];
        let mut deflections = Vec::new();
        for (shape, reference) in cases {
            let tip = macneal_harder_tip_deflection(shape);
            assert!((tip - reference).abs() / reference < 1e-6, "{shape}: {tip} vs scikit-fem {reference}");
            deflections.push(tip);
        }
        let theory = 0.1081;
        assert!(deflections[0] / theory < 0.12, "rectangular Quad4 must lock, got {}", deflections[0] / theory);
        for distorted in &deflections[1..] {
            assert!(distorted * 3.0 < deflections[0], "distortion must cost at least a factor of three, got {distorted} vs {}", deflections[0]);
        }
    }

    /// 📏️ MacNeal-Harder cantilever mesh in one of the three published shapes; returns the mean tip
    /// vertical deflection under a unit shear split over the two tip nodes.
    fn macneal_harder_tip_deflection(shape: &str) -> f64 {
        let (e, nu, t, length, height, n) = (1e7, 0.3, 0.1, 6.0, 0.2, 6usize);
        let id = |i: usize, j: usize| format!("n{i}_{j}");
        let mut nodes = Vec::new();
        for i in 0..=n {
            let x = length * i as f64 / n as f64;
            for j in 0..2 {
                let offset = match shape {
                    "parallelogram" => height * j as f64,
                    "trapezoidal" if i > 0 && i < n => (if i % 2 == 0 { 0.5 } else { -0.5 }) * height * (if j == 1 { 1.0 } else { -1.0 }),
                    _ => 0.0,
                };
                nodes.push(Node { id: id(i, j), pos: [x + offset, height * j as f64, 0.0] });
            }
        }
        let elements: Vec<Elements> = (0..n).map(|i| Quad4 { id: format!("q{i}"), nodes: [id(i, 0), id(i + 1, 0), id(i + 1, 1), id(i, 1)], e, nu, thickness: t, kind: PlaneKind::Stress, density: 0.0 }.into()).collect();
        let supports = (0..2).map(|j| Support { node_id: id(0, j), fixed: vec![Dof::Tx, Dof::Ty] }).collect();
        let nodal_loads = (0..2).map(|j| NodalLoad { node_id: id(n, j), dof: Dof::Ty, value: 0.5 }).collect();

        let model = Model { nodes, elements, supports, nodal_loads, member_loads: vec![] };
        let result = solve_linear_static(&model).expect("macneal-harder cantilever solves");
        (0..2).map(|j| result.displacements.iter().find(|d| d.node_id == id(n, j)).unwrap().values[Dof::Ty.index()]).sum::<f64>() / 2.0
    }

    /// ⚖️ Consistent-mass physical sanity check (same identity `bar2_mass_total_equals_rho_a_l` uses):
    /// the sum of the pure-`Tx` submatrix must equal the element's total mass `ρtA`.
    #[test]
    fn tri3_cst_mass_total_equals_rho_t_area() {
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

    fn triangle_signed_area(coords: &[[f64; 2]]) -> f64 {
        0.5 * ((coords[1][0] - coords[0][0]) * (coords[2][1] - coords[0][1]) - (coords[2][0] - coords[0][0]) * (coords[1][1] - coords[0][1]))
    }

    #[test]
    fn quad4_mass_total_equals_rho_t_area() {
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
    #[test]
    fn tri3_cst_geometric_stiffness_rigid_translation_gives_zero_force_and_is_symmetric() {
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
    #[test]
    fn quad4_geometric_stiffness_rigid_translation_gives_zero_force_and_is_symmetric() {
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
    #[test]
    fn tri6_lst_mass_total_equals_rho_t_area() {
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
    #[test]
    fn tri6_lst_geometric_stiffness_rigid_translation_gives_zero_force_and_is_symmetric() {
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
    #[test]
    fn quad8_mass_total_equals_rho_t_area() {
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
    #[test]
    fn quad8_geometric_stiffness_rigid_translation_gives_zero_force_and_is_symmetric() {
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
    #[test]
    fn continuum_elements_solve_correctly_via_enum_dispatch() {
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

    fn ctx_of(coords: &[[f64; 2]; 3]) -> ElementContext {
        ElementContext { positions: coords.iter().map(|&[x, y]| [x, y, 0.0]).collect() }
    }

    /// 📐️ Constant-curvature field `w = 0.5*(kx*x² + ky*y² + 2*kxy*x*y)` with matching nodal rotations
    /// `Rx = ∂w/∂y = ky*y + kxy*x`, `Ry = -∂w/∂x = -(kx*x + kxy*y)` — the DKT patch-test input.
    fn constant_curvature_u_local(coords: &[[f64; 2]; 3]) -> VecD {
        let mut v = Vec::with_capacity(9);
        for &[x, y] in coords {
            v.push(0.5 * (KX * x * x + KY * y * y + 2.0 * KXY * x * y));
            v.push(KY * y + KXY * x);
            v.push(-(KX * x + KXY * y));
        }
        VecD::from_vec(v)
    }

    #[test]
    fn plate_dkt_patch_test_reproduces_constant_curvature() {
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

    #[test]
    fn plate_dkt_rigid_translation_gives_zero_force() {
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
    #[test]
    fn plate_dkt_mass_lumps_rho_t_area_over_3_onto_each_tz_only() {
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

    /// 🏗️ Simply-supported square plate (side `a=2`, `t=0.01`, E=2e11, ν=0.3) under a uniform
    /// pressure `q=1000`, meshed `n x n` into `2n²` `PlateDkt` triangles (cells split along the
    /// `(i,j)-(i+1,j+1)` diagonal), `Tz=0` at every boundary node with rotations free (soft simple
    /// support), load lumped `q*Area_i/3` to each triangle's 3 nodes. Returns the centre deflection.
    fn plate_dkt_simply_supported_centre_deflection(n: usize) -> f64 {
        let (e, nu, t, a, q) = (2e11, 0.3, 0.01, 2.0, 1000.0);
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
                elements.push(PlateDkt { id: format!("t{i}_{j}a"), nodes: [node_id(i, j), node_id(i + 1, j), node_id(i + 1, j + 1)], e, nu, thickness: t, density: 0.0 }.into());
                elements.push(PlateDkt { id: format!("t{i}_{j}b"), nodes: [node_id(i, j), node_id(i + 1, j + 1), node_id(i, j + 1)], e, nu, thickness: t, density: 0.0 }.into());
            }
        }

        let supports = (0..=n).flat_map(|i| (0..=n).map(move |j| (i, j))).filter(|&(i, j)| i == 0 || i == n || j == 0 || j == n).map(|(i, j)| Support { node_id: node_id(i, j), fixed: vec![Dof::Tz] }).collect();

        let mut lumped: std::collections::HashMap<String, f64> = std::collections::HashMap::new();
        for i in 0..n {
            for j in 0..n {
                let share = q * (0.5 * dx * dx) / 3.0;
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
        -result.displacements.iter().find(|d| d.node_id == node_id(n / 2, n / 2)).unwrap().values[Dof::Tz.index()]
    }

    /// 🏗️ Simply-supported square plate under UDL vs the classical thin-plate closed form
    /// `w_max = 0.00406 q a⁴/D` (ν=0.3). `🔨️w7-kernel-references.py` section 4 confirms that
    /// coefficient two ways: the Navier double sine series gives `w_max D/(q a⁴) = 0.00406235`, and
    /// scikit-fem's `ElementTriMorley` Kirchhoff plate converges to that series value (0.12 % at
    /// 16641 DOFs). The same script's independent numpy Batoz-DKT assembly on the IDENTICAL meshes
    /// gives 3.406677456811e-3 m (4x4) and 3.514848272890e-3 m (8x8, 128 triangles, 243 DOFs),
    /// i.e. 3.95 % then 0.90 % below the closed form — a monotone convergence this test also gates.
    #[test]
    fn plate_dkt_simply_supported_square_center_deflection_matches_closed_form() {
        let (e, nu, t, a, q) = (2e11_f64, 0.3_f64, 0.01_f64, 2.0_f64, 1000.0_f64);
        let closed_form = 0.00406 * q * a.powi(4) / (e * t.powi(3) / (12.0 * (1.0 - nu * nu)));

        let coarse = plate_dkt_simply_supported_centre_deflection(4);
        let fine = plate_dkt_simply_supported_centre_deflection(8);
        for (label, actual, reference) in [("4x4", coarse, 3.406677456811e-3), ("8x8", fine, 3.514848272890e-3)] {
            assert!((actual - reference).abs() / reference < 1e-6, "{label}: {actual} vs numpy DKT {reference}");
        }
        assert!(coarse < fine && fine < closed_form, "DKT must converge to {closed_form} from below, got {coarse} then {fine}");
        assert!((fine - closed_form).abs() / closed_form < 0.02, "8x8 centre deflection {fine} vs closed form {closed_form}");
    }
}
// #endregion 🔖️PlateTests
