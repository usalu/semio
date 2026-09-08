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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests

// #region 🔖️ContinuumTests
#[cfg(test)]
#[path = "🧪️tests/🔬️continuum/🦀️.rs"]
mod continuum_tests;
// #endregion 🔖️ContinuumTests

// #region 🔖️PlateTests
#[cfg(test)]
#[path = "🧪️tests/🔬️plate/🦀️.rs"]
mod plate_tests;
// #endregion 🔖️PlateTests
