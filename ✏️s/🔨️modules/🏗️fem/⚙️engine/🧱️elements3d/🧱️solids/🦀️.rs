//! 🧱️ Solid constitutive helpers and the `Tet4` and `Hex8` solid continuum elements.

use super::*;

// #region 🔖️Solid
/// 🧮️ Isotropic 3D solid-mechanics constitutive matrix (6x6), strain order `[εxx,εyy,εzz,γxy,γyz,γxz]`.
pub fn d_matrix_solid(e: f64, nu: f64) -> MatD {
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
pub fn solid_b_matrix(grads: &[[f64; 3]]) -> MatD {
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
pub fn von_mises_solid(sxx: f64, syy: f64, szz: f64, sxy: f64, syz: f64, sxz: f64) -> f64 {
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
    tet4_geometry_of_positions(&ctx.positions)
}

fn tet4_geometry_of_positions(positions: &[[f64; 3]]) -> Option<Tet4Geometry> {
    if positions.len() != 4 {
        return None;
    }
    let first = vec3d_sub(positions[1], positions[0]);
    let second = vec3d_sub(positions[2], positions[0]);
    let third = vec3d_sub(positions[3], positions[0]);
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

/// 🧊️ Contracts one tetrahedral geometric cell through the original full signed stress tensor.
fn tet4_geometric_cell(geometry: Tet4Geometry, stress: &SolidStress, row: usize, column: usize) -> f64 {
    if row % 3 != column % 3 { return 0.0; }
    let left = geometry.gradients[row / 3];
    let right = geometry.gradients[column / 3];
    geometry.volume * (
        left[0] * (stress.sxx * right[0] + stress.sxy * right[1] + stress.sxz * right[2])
        + left[1] * (stress.sxy * right[0] + stress.syy * right[1] + stress.syz * right[2])
        + left[2] * (stress.sxz * right[0] + stress.syz * right[1] + stress.szz * right[2])
    )
}

impl Tet4 {
    /// 🪨️ Recovers the constant tetrahedral stress without allocating scratch matrices or vectors.
    pub fn mounted_recover_stress(&self, positions: &[[f64; 3]; 4], displacement: &[f64; 12]) -> Option<SolidStress> {
        let geometry = tet4_geometry_of_positions(positions)?;
        Some(self.recover_stress(geometry, displacement))
    }

    /// 🪨️ Resolves the signed stress tensor using fixed-size strain and constitutive contractions.
    fn recover_stress(&self, geometry: Tet4Geometry, displacement: &[f64; 12]) -> SolidStress {
        let mut strain = [0.0; 6];
        for local in 0..12 {
            let column = solid_b_column(geometry.gradients[local / 3], local % 3);
            for axis in 0..6 { strain[axis] += column[axis] * displacement[local]; }
        }
        let stress: [f64; 6] = std::array::from_fn(|axis| solid_constitutive_bilinear(self.e, self.nu, std::array::from_fn(|component| f64::from(u8::from(component == axis))), strain));
        let [sxx, syy, szz, sxy, syz, sxz] = stress;
        SolidStress { sxx, syy, szz, sxy, syz, sxz, von_mises: von_mises_solid(sxx, syy, szz, sxy, syz, sxz) }
    }
    /// 🧭️ Signed volume via the scalar triple product of edge vectors from node 0.
    pub fn volume(ctx: &ElementContext) -> f64 {
        tet4_geometry(ctx).expect("non-degenerate tet4").volume
    }

    /// 🧭️ Constant per-node shape-function gradients `[∂Li/∂x, ∂Li/∂y, ∂Li/∂z]`. `Li(x,y,z) = a+bx+cy+dz`
    /// with `Li(node_j) = δij` for all j — solving `R·[a,b,c,d]ᵀ = e_i` per node (`R`'s row j is
    /// `[1,xj,yj,zj]`) gives node i's coefficients directly, gradient in components 1..4.
    pub fn gradients(ctx: &ElementContext) -> [[f64; 3]; 4] {
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
        let geometry = tet4_geometry(ctx)?;
        let displacement = std::array::from_fn(|index| u_elem.get(index));
        let stress = self.recover_stress(geometry, &displacement);
        let mut kg = MatD::zeros(12, 12);
        for row in 0..12 {
            for column in 0..12 {
                kg.set(row, column, tet4_geometric_cell(geometry, &stress, row, column));
            }
        }
        Some(kg)
    }

    fn mounted_geometric_stiffness_cell(&self, ctx: &ElementContext, displacement: &[f64], row: usize, column: usize) -> Option<f64> {
        let displacement: &[f64; 12] = displacement.try_into().ok()?;
        if row >= 12 || column >= 12 || !displacement.iter().all(|value| value.is_finite()) { return None; }
        let geometry = tet4_geometry(ctx)?;
        let stress = self.recover_stress(geometry, displacement);
        let value = tet4_geometric_cell(geometry, &stress, row, column);
        value.is_finite().then_some(value)
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
