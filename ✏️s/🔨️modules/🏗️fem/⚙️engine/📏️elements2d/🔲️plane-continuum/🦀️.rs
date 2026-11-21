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

fn stress_gradient_product([dix, diy]: [f64; 2], [sxx, syy, sxy]: [f64; 3], [djx, djy]: [f64; 2]) -> f64 {
    dix * sxx * djx + dix * sxy * djy + diy * sxy * djx + diy * syy * djy
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
        let stress = [sigma.get(0), sigma.get(1), sigma.get(2)];
        let scale = w * det_j * thickness;
        for (i, &left) in d_n_xy[..n_nodes].iter().enumerate() {
            for (j, &right) in d_n_xy[..n_nodes].iter().enumerate() {
                let s = stress_gradient_product(left, stress, right);
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

struct TriangleJacobian {
    dx_dxi: f64,
    dx_deta: f64,
    dy_dxi: f64,
    dy_deta: f64,
    determinant: f64,
}

impl TriangleJacobian {
    fn new(positions: &[[f64; 3]]) -> Self {
        let dx_dxi = positions[1][0] - positions[0][0];
        let dx_deta = positions[2][0] - positions[0][0];
        let dy_dxi = positions[1][1] - positions[0][1];
        let dy_deta = positions[2][1] - positions[0][1];
        let determinant = dx_dxi * dy_deta - dx_deta * dy_dxi;
        Self { dx_dxi, dx_deta, dy_dxi, dy_deta, determinant }
    }

    fn checked(positions: &[[f64; 3]]) -> Option<Self> {
        if positions.len() != 3 {
            return None;
        }
        let jacobian = Self::new(positions);
        (jacobian.determinant.is_finite() && jacobian.determinant.abs() > f64::EPSILON).then_some(jacobian)
    }

    fn gradients(&self) -> [[f64; 2]; 3] {
        shape_tri3(0.0, 0.0).1.map(|[dxi, deta]| {
            [
                (self.dy_deta * dxi - self.dy_dxi * deta) / self.determinant,
                (-self.dx_deta * dxi + self.dx_dxi * deta) / self.determinant,
            ]
        })
    }
}

fn plane_strain_column([dx, dy]: [f64; 2], axis: usize) -> [f64; 3] {
    if axis == 0 { [dx, 0.0, dy] } else { [0.0, dy, dx] }
}

impl Tri3Cst {
    /// 🌧️ Integrates one CST area-pressure and consistent translational gravity row.
    pub fn mounted_load_row(&self, positions: &[[f64; 3]], row: usize, pressure: f64, gravity: [f64; 2]) -> Option<f64> {
        if row >= 6 {
            return None;
        }
        let determinant = TriangleJacobian::checked(positions)?.determinant;
        Some((self.density * self.thickness * gravity[row % 2] - if row % 2 == 1 { pressure } else { 0.0 }) * determinant.abs() / 6.0)
    }

    /// 🧷 Evaluates one CST initial-stress cell from the fixed triangle and its displacements.
    pub fn mounted_geometric_cell(&self, positions: &[[f64; 3]], displacement: &[f64], row: usize, column: usize) -> Option<f64> {
        if row >= 6 || column >= 6 || displacement.len() != 6 || displacement.iter().any(|value| !value.is_finite()) {
            return None;
        }
        let jacobian = TriangleJacobian::checked(positions)?;
        if row % 2 != column % 2 {
            return Some(0.0);
        }
        let value = self.geometric_cells(&jacobian, displacement)(row, column);
        value.is_finite().then_some(value)
    }

    fn constitutive_coefficients(&self) -> (f64, f64, f64) {
        match self.kind {
            PlaneKind::Stress => {
                let factor = self.e / (1.0 - self.nu * self.nu);
                (factor, factor * self.nu, factor * (1.0 - self.nu) / 2.0)
            }
            PlaneKind::Strain => {
                let factor = self.e / ((1.0 + self.nu) * (1.0 - 2.0 * self.nu));
                (factor * (1.0 - self.nu), factor * self.nu, factor * (1.0 - 2.0 * self.nu) / 2.0)
            }
        }
    }

    fn mass_cell(&self, determinant: f64, row: usize, column: usize) -> f64 {
        if row % 2 != column % 2 {
            return 0.0;
        }
        let coefficient = self.density * self.thickness * determinant / 24.0;
        if row == column { 2.0 * coefficient } else { coefficient }
    }

    fn geometric_cells(&self, jacobian: &TriangleJacobian, displacement: &[f64]) -> impl Fn(usize, usize) -> f64 {
        let gradients = jacobian.gradients();
        let mut strain = [0.0; 3];
        for ([dx, dy], dofs) in gradients.iter().copied().zip(displacement.chunks_exact(2)) {
            strain[0] += dx * dofs[0];
            strain[1] += dy * dofs[1];
            strain[2] += dy * dofs[0];
            strain[2] += dx * dofs[1];
        }
        let (normal, coupling, shear) = self.constitutive_coefficients();
        let [exx, eyy, exy] = strain;
        let stress = [normal * exx + coupling * eyy, coupling * exx + normal * eyy, shear * exy];
        let scale = 0.5 * jacobian.determinant * self.thickness;
        move |row, column| {
            if row % 2 != column % 2 {
                return 0.0;
            }
            let left = gradients[row.max(column) / 2];
            let right = gradients[row.min(column) / 2];
            stress_gradient_product(left, stress, right) * scale
        }
    }

    fn rule(&self) -> Vec<(f64, f64, f64)> {
        gauss_tri(1)
    }

    fn shape(xi: f64, eta: f64) -> Vec<[f64; 2]> {
        shape_tri3(xi, eta).1.to_vec()
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
        if row >= 6 || column >= 6 {
            return None;
        }
        let jacobian = TriangleJacobian::checked(&ctx.positions)?;
        let derivatives = jacobian.gradients();
        let left = plane_strain_column(derivatives[row / 2], row % 2);
        let right = plane_strain_column(derivatives[column / 2], column % 2);
        let (normal, coupling, shear) = self.constitutive_coefficients();
        let constitutive = [[normal, coupling, 0.0], [coupling, normal, 0.0], [0.0, 0.0, shear]];
        let mut value = 0.0;
        for i in 0..3 {
            for j in 0..3 {
                value += left[i] * constitutive[i][j] * right[j];
            }
        }
        Some(value * 0.5 * jacobian.determinant * self.thickness)
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
        let determinant = TriangleJacobian::new(&ctx.positions).determinant;
        let mut mass = MatD::zeros(6, 6);
        for row in 0..6 {
            for column in 0..6 {
                mass.set(row, column, self.mass_cell(determinant, row, column));
            }
        }
        Some(mass)
    }

    fn mounted_mass_cell(&self, positions: &[[f64; 3]], row: usize, column: usize) -> Option<f64> {
        if row >= 6 || column >= 6 {
            return None;
        }
        let jacobian = TriangleJacobian::checked(positions)?;
        let value = self.mass_cell(jacobian.determinant, row, column);
        value.is_finite().then_some(value)
    }

    fn geometric_stiffness(&self, ctx: &ElementContext, u_element: &VecD) -> Option<MatD> {
        assert_eq!(u_element.len(), 6, "triangle displacement count");
        let jacobian = TriangleJacobian::new(&ctx.positions);
        let cell = self.geometric_cells(&jacobian, &u_element.0);
        let mut geometric = MatD::zeros(6, 6);
        for row in 0..6 {
            for column in 0..6 {
                geometric.set(row, column, cell(row, column));
            }
        }
        Some(geometric)
    }

    fn mounted_geometric_stiffness_cell(&self, ctx: &ElementContext, displacement: &[f64], row: usize, column: usize) -> Option<f64> {
        self.mounted_geometric_cell(&ctx.positions, displacement, row, column)
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
