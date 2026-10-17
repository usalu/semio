//! 🪢️ Segment geometry, the axial `Bar2` truss and the Euler-Bernoulli `BeamEb2` frame member.

use super::*;

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
        semio_s_fem_model::close_mounted_strings([&mut self.id, &mut self.start, &mut self.end])
    }

    fn mounted_next_string_bytes(&self) -> Option<usize> {
        semio_s_fem_model::mounted_string_bytes([&self.id, &self.start, &self.end])
    }

    fn mounted_strings_terminal_is_empty(&self) -> bool {
        self.mounted_next_string_bytes().is_none()
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
        semio_s_fem_model::close_mounted_strings([&mut self.id, &mut self.start, &mut self.end])
    }

    fn mounted_next_string_bytes(&self) -> Option<usize> {
        semio_s_fem_model::mounted_string_bytes([&self.id, &self.start, &self.end])
    }

    fn mounted_strings_terminal_is_empty(&self) -> bool {
        self.mounted_next_string_bytes().is_none()
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
                semio_s_fem_model::BeamStation { x, n: -n1, v: v1 + wy_local * x, m: -m1 + v1 * x + wy_local * x * x / 2.0 }
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
