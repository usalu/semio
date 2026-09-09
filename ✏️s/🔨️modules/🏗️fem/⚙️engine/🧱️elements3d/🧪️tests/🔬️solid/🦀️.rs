use super::*;
use crate::model::{NodalLoad, Node, Support};

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
        elements: vec![Tet4 { id: "t1".into(), nodes: ["n0".into(), "n1".into(), "n2".into(), "n3".into()], e, nu, density }.into()],
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

/// 🏗️ Structured `Hex8` cantilever mesh over the `4.0 x 1.0 x 2.0` box (E=200 GPa, ν=0.3), fully
/// clamped on `x=0`, a total 10 kN `-Z` tip load split evenly over the `x=L` face nodes. Solved
/// through the sparse `analyses::solve_multi_case` pipeline (the refined mesh has 702 DOFs, well
/// past what the dense `solve_linear_static` path is meant for). Returns the mean tip `Tz`.
fn hex8_cantilever_tip_deflection(nx: usize, ny: usize, nz: usize) -> f64 {
    use crate::analyses::{solve_multi_case, AnalysisModel, LoadCase};
    let (e, nu) = (200e9, 0.3);
    let (length, width, height) = (4.0_f64, 1.0_f64, 2.0_f64);
    let id = |i: usize, j: usize, k: usize| format!("n{i}_{j}_{k}");

    let mut nodes = Vec::new();
    for i in 0..=nx {
        for j in 0..=ny {
            for k in 0..=nz {
                nodes.push(Node { id: id(i, j, k), pos: [length * i as f64 / nx as f64, width * j as f64 / ny as f64, height * k as f64 / nz as f64] });
            }
        }
    }

    let mut elements: Vec<Elements> = Vec::new();
    for i in 0..nx {
        for j in 0..ny {
            for k in 0..nz {
                elements.push(
                    Hex8 { id: format!("h{i}_{j}_{k}"), nodes: [id(i, j, k), id(i + 1, j, k), id(i + 1, j + 1, k), id(i, j + 1, k), id(i, j, k + 1), id(i + 1, j, k + 1), id(i + 1, j + 1, k + 1), id(i, j + 1, k + 1)], e, nu, density: 0.0 }.into(),
                );
            }
        }
    }

    let face: Vec<(usize, usize)> = (0..=ny).flat_map(|j| (0..=nz).map(move |k| (j, k))).collect();
    let supports = face.iter().map(|&(j, k)| Support { node_id: id(0, j, k), fixed: vec![Dof::Tx, Dof::Ty, Dof::Tz] }).collect();
    let share = -1e4 / face.len() as f64;
    let nodal_loads = face.iter().map(|&(j, k)| NodalLoad { node_id: id(nx, j, k), dof: Dof::Tz, value: share }).collect();

    let model = AnalysisModel { nodes, elements, supports };
    let case = LoadCase { id: "tip".into(), nodal_loads, member_loads: vec![], self_weight: false };
    let results = solve_multi_case(&model, std::slice::from_ref(&case), &[], [0.0, 0.0, 0.0]).expect("hex cantilever solves");
    let result = results.get("tip").unwrap();
    face.iter().map(|&(j, k)| result.displacements.iter().find(|d| d.node_id == id(nx, j, k)).unwrap().values[Dof::Tz.index()]).sum::<f64>() / face.len() as f64
}

/// 🏗️ Hex-meshed cantilever against BOTH an exact same-mesh reference and the Timoshenko closed
/// form `δ = PL³/3EI + PL/(κGA)`, `κ=5/6` (1.600e-6 + 3.120e-7 = 1.912e-6 m for this box).
/// `🔨️w7-kernel-references.py` section 5 computes the same-mesh values two independent ways —
/// scikit-fem 12.0.2's `ElementHex1` at `intorder=2` and a hand-assembled numpy trilinear hex —
/// which agree to twelve digits: -1.722214505218e-6 m (8x2x2, 243 DOFs) and -1.822205573460e-6 m
/// (12x2x5, 702 DOFs). A trilinear hex without incompatible modes shear-locks, so beam theory is
/// only approached from below (90.07 % then 95.30 %) — hence the 5 % bound on the refined mesh.
#[test]
fn hex8_meshed_cantilever_matches_reference_and_beam_theory() {
    let coarse = hex8_cantilever_tip_deflection(8, 2, 2);
    let fine = hex8_cantilever_tip_deflection(12, 2, 5);
    for (label, actual, reference) in [("8x2x2", coarse, -1.722214505218e-6), ("12x2x5", fine, -1.822205573460e-6)] {
        assert!((actual - reference).abs() / reference.abs() < 1e-6, "{label}: {actual} vs scikit-fem {reference}");
    }

    let (e, nu, length, width, height, p_total) = (200e9_f64, 0.3_f64, 4.0_f64, 1.0_f64, 2.0_f64, 1e4_f64);
    let shear_modulus = e / (2.0 * (1.0 + nu));
    let inertia = width * height.powi(3) / 12.0;
    let closed_form = p_total * length.powi(3) / (3.0 * e * inertia) + p_total * length / ((5.0 / 6.0) * shear_modulus * width * height);
    assert!(fine < coarse && coarse < 0.0, "hex must stiffen-lock and converge upward in magnitude, got {coarse} then {fine}");
    assert!((fine.abs() - closed_form).abs() / closed_form < 0.05, "refined tip {fine} vs Timoshenko closed form {closed_form}");
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
