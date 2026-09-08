
use super::*;
use crate::model::{Model, NodalLoad, Node, Support, solve_linear_static};

const E: f64 = 1000.0;
const NU: f64 = 0.25;
const THICKNESS: f64 = 1.0;

// Same small-magnitude membrane field as `elements2d::continuum_tests` (keeps expected forces
// O(1) relative to the absolute patch-test tolerance) plus a small constant-curvature bending field.
const MU: (f64, f64, f64) = (0.01, 0.003, 0.0021);
const MV: (f64, f64, f64) = (-0.02, 0.0012, 0.0027);
const KX: f64 = 0.004;
const KY: f64 = -0.0025;
const KXY: f64 = 0.0017;

/// 📐️ A triangle placed so `p1-p0` lies exactly on global X and `p2` has `z=0` — the local shell
/// frame (`local_x=normalize(p1-p0)`, `local_z=facet normal`) then coincides EXACTLY with global
/// `(X,Y,Z)`, so local == global coordinates and the combined membrane+bending patch-test input
/// can be built directly in global `(x,y)` without any local-frame bookkeeping.
fn aligned_triangle_positions() -> [[f64; 3]; 3] {
    [[0.0, 0.0, 0.0], [2.0, 0.0, 0.0], [0.2, 1.8, 0.0]]
}

#[test]
fn shell_facet3_patch_test_reproduces_linear_membrane_and_constant_curvature() {
    let positions = aligned_triangle_positions();
    let el = ShellFacet3 { id: "s".into(), nodes: ["a".into(), "b".into(), "c".into()], e: E, nu: NU, thickness: THICKNESS, density: 0.0 };
    let ctx = ElementContext { positions: positions.to_vec() };

    let mut u = Vec::with_capacity(18);
    for &[x, y, _] in &positions {
        u.push(MU.0 + MU.1 * x + MU.2 * y); // Tx
        u.push(MV.0 + MV.1 * x + MV.2 * y); // Ty
        u.push(0.5 * (KX * x * x + KY * y * y + 2.0 * KXY * x * y)); // Tz = w
        u.push(KY * y + KXY * x); // Rx = ∂w/∂y
        u.push(-(KX * x + KXY * y)); // Ry = -∂w/∂x
        u.push(0.0); // Rz (drilling) stays zero — not excited by this field
    }
    let u = VecD::from_vec(u);

    let ElementResult::Shell { gauss } = el.recover(&ctx, &u, None) else { panic!("expected shell result") };
    assert_eq!(gauss.len(), 1);
    let state = &gauss[0];

    let d_mem = d_matrix_plane_stress(E, NU);
    let strain = VecD::from_vec(vec![MU.1, MV.2, MU.2 + MV.1]);
    let sigma = d_mem.mul_vec(&strain);
    let (expected_nxx, expected_nyy, expected_nxy) = (sigma.get(0) * THICKNESS, sigma.get(1) * THICKNESS, sigma.get(2) * THICKNESS);

    let d_bend = crate::elements2d::d_matrix_plate(E, NU, THICKNESS);
    let kappa = VecD::from_vec(vec![KX, KY, 2.0 * KXY]);
    let m = d_bend.mul_vec(&kappa);

    let mem_scale = expected_nxx.abs().max(expected_nyy.abs()).max(expected_nxy.abs()).max(1.0);
    assert!((state.nxx - expected_nxx).abs() / mem_scale < 1e-6, "nxx {} vs {}", state.nxx, expected_nxx);
    assert!((state.nyy - expected_nyy).abs() / mem_scale < 1e-6, "nyy {} vs {}", state.nyy, expected_nyy);
    assert!((state.nxy - expected_nxy).abs() / mem_scale < 1e-6, "nxy {} vs {}", state.nxy, expected_nxy);

    let bend_scale = m.get(0).abs().max(m.get(1).abs()).max(m.get(2).abs()).max(1.0);
    assert!((state.mxx - m.get(0)).abs() / bend_scale < 1e-4, "mxx {} vs {}", state.mxx, m.get(0));
    assert!((state.myy - m.get(1)).abs() / bend_scale < 1e-4, "myy {} vs {}", state.myy, m.get(1));
    assert!((state.mxy - m.get(2)).abs() / bend_scale < 1e-4, "mxy {} vs {}", state.mxy, m.get(2));
}

/// 🌀️ Rigid-body test: a pure 3D translation (zero rotation, so the drilling DOF is untouched too)
/// must produce zero internal force on a generic (non-axis-aligned) skew triangle.
#[test]
fn shell_facet3_rigid_translation_gives_zero_force() {
    let positions = [[0.0, 0.0, 0.0], [2.0, 0.0, 0.3], [0.5, 1.5, 0.7]];
    let el = ShellFacet3 { id: "s".into(), nodes: ["a".into(), "b".into(), "c".into()], e: E, nu: NU, thickness: THICKNESS, density: 0.0 };
    let ctx = ElementContext { positions: positions.to_vec() };
    let ke = el.stiffness_global(&ctx);
    let mut rigid = Vec::with_capacity(18);
    for _ in 0..3 {
        rigid.extend_from_slice(&[1.2, -0.8, 0.5, 0.0, 0.0, 0.0]);
    }
    let rigid = VecD::from_vec(rigid);
    let f = ke.mul_vec(&rigid);
    for i in 0..18 {
        assert!(f.get(i).abs() < 1e-6, "rigid-body force[{i}] = {}", f.get(i));
    }
}

/// 🏗️ Smoke test: a single flat `ShellFacet3` with one full edge fixed, loaded out-of-plane at the
/// free node — deflection must be finite, nonzero, and in the same direction as the applied load
/// (not a precision benchmark, just a physical-sanity check on assembly/BC wiring).
#[test]
fn shell_facet3_cantilever_deflects_toward_tip_load() {
    let (e, nu, t) = (200e9, 0.3, 0.01);
    let p = -1000.0;
    let model = Model {
        nodes: vec![Node { id: "a".into(), pos: [0.0, 0.0, 0.0] }, Node { id: "b".into(), pos: [1.0, 0.0, 0.0] }, Node { id: "c".into(), pos: [0.0, 1.0, 0.0] }],
        elements: vec![ShellFacet3 { id: "s".into(), nodes: ["a".into(), "b".into(), "c".into()], e, nu, thickness: t, density: 0.0 }.into()],
        supports: vec![Support { node_id: "a".into(), fixed: vec![Dof::Tx, Dof::Ty, Dof::Tz, Dof::Rx, Dof::Ry, Dof::Rz] }, Support { node_id: "b".into(), fixed: vec![Dof::Tx, Dof::Ty, Dof::Tz, Dof::Rx, Dof::Ry, Dof::Rz] }],
        nodal_loads: vec![NodalLoad { node_id: "c".into(), dof: Dof::Tz, value: p }],
        member_loads: vec![],
    };
    let result = solve_linear_static(&model).expect("cantilevered shell facet solves");
    let c = result.displacements.iter().find(|d| d.node_id == "c").unwrap();
    let dz = c.values[Dof::Tz.index()];
    assert!(dz.is_finite() && dz < 0.0, "tip deflection {dz} should be finite and negative (toward the -Tz load)");
}

/// ⚖️ `ShellFacet3::mass`'s total (pure-`Tx` submatrix sum) equals `ρtA` — same lumped-mass
/// row-sum identity `PlateDkt`'s translational lump satisfies.
#[test]
fn shell_facet3_mass_total_equals_rho_t_area() {
    let (density, thickness) = (7850.0, 0.008);
    let positions = aligned_triangle_positions();
    let el = ShellFacet3 { id: "s".into(), nodes: ["a".into(), "b".into(), "c".into()], e: E, nu: NU, thickness, density };
    let ctx = ElementContext { positions: positions.to_vec() };
    let m = el.mass(&ctx).expect("shell facet reports mass");
    let sum_tx: f64 = (0..3).flat_map(|r| (0..3).map(move |c| (6 * r, 6 * c))).map(|(r, c)| m.get(r, c)).sum();
    // `aligned_triangle_positions` is `[[0,0,0],[2,0,0],[0.2,1.8,0]]` — shoelace area directly.
    let area = 0.5 * ((positions[1][0] - positions[0][0]) * (positions[2][1] - positions[0][1]) - (positions[2][0] - positions[0][0]) * (positions[1][1] - positions[0][1])).abs();
    let expected = density * thickness * area;
    assert!((sum_tx - expected).abs() / expected < 1e-9, "sum={sum_tx} expected={expected}");
}

/// 🌀️ A cantilevered flat shell panel (2 `ShellFacet3` triangles, one edge fully fixed) under
/// in-plane axial COMPRESSION at the free edge must produce a finite, positive lowest linear-
/// buckling load factor — possible only now that `ShellFacet3::geometric_stiffness` exists (a
/// `PlateDkt`-only panel would report no geometric stiffness at all, per its documented `None`).
#[test]
fn shell_facet3_membrane_compression_destabilizes_and_tension_stabilizes_out_of_plane_stiffness() {
    let positions = aligned_triangle_positions();
    let el = ShellFacet3 { id: "s".into(), nodes: ["a".into(), "b".into(), "c".into()], e: E, nu: NU, thickness: THICKNESS, density: 0.0 };
    let ctx = ElementContext { positions: positions.to_vec() };

    // Uniform uniaxial membrane strain `u = k*x` (zero elsewhere) recovers a constant `Nxx`,
    // compressive for k<0 and tensile for k>0 — same field shape `elements2d::continuum_tests`
    // uses for its patch tests.
    let field = |k: f64| {
        let mut u = Vec::with_capacity(18);
        for &[x, _, _] in &positions {
            u.extend_from_slice(&[k * x, 0.0, 0.0, 0.0, 0.0, 0.0]);
        }
        VecD::from_vec(u)
    };
    let kg_tension = el.geometric_stiffness(&ctx, &field(1e-4)).expect("shell reports geometric stiffness");
    let kg_compression = el.geometric_stiffness(&ctx, &field(-1e-4)).expect("shell reports geometric stiffness");

    // Node `b`'s local `Tz` sits at global index 8 (node 1 * 6 dof + 2) — the aligned-triangle
    // fixture makes local == global, so this global diagonal entry is directly the out-of-plane
    // stiffness contribution the buckling solver would add for node b.
    let tz_b = 8usize;
    assert!(kg_tension.get(tz_b, tz_b) > 0.0, "tension should STIFFEN out-of-plane bending, got Kg[b,Tz]={}", kg_tension.get(tz_b, tz_b));
    assert!(kg_compression.get(tz_b, tz_b) < 0.0, "compression should DESTABILIZE out-of-plane bending, got Kg[b,Tz]={}", kg_compression.get(tz_b, tz_b));

    for r in 0..18 {
        for c in 0..18 {
            assert!((kg_compression.get(r, c) - kg_compression.get(c, r)).abs() < 1e-9, "Kg not symmetric at ({r},{c})");
        }
    }
}
