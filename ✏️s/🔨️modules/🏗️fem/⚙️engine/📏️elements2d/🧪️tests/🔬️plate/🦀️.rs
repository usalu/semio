
use super::*;
use crate::model::{Model, NodalLoad, Node, Support, solve_linear_static};

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
