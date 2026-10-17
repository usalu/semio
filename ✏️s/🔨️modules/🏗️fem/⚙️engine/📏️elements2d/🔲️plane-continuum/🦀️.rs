//! 🔲️ Plane-stress and plane-strain continuum family: `Tri3Cst`, `Tri6Lst`, `Quad4` and `Quad8`.

use super::*;

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
        semio_s_fem_model::close_mounted_strings(std::iter::once(&mut self.id).chain(self.nodes.iter_mut()))
    }

    fn mounted_next_string_bytes(&self) -> Option<usize> {
        semio_s_fem_model::mounted_string_bytes(std::iter::once(&self.id).chain(self.nodes.iter()))
    }

    fn mounted_strings_terminal_is_empty(&self) -> bool {
        self.mounted_next_string_bytes().is_none()
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
