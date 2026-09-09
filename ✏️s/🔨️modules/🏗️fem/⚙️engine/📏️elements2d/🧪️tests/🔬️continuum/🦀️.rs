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
                elements.push(
                    Quad8 { id: format!("q{a}_{b}"), nodes: [id(i, j), id(i + 2, j), id(i + 2, j + 2), id(i, j + 2), id(i + 1, j), id(i + 2, j + 1), id(i + 1, j + 2), id(i, j + 1)], e, nu, thickness: t, kind: PlaneKind::Stress, density: 0.0 }.into(),
                );
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
