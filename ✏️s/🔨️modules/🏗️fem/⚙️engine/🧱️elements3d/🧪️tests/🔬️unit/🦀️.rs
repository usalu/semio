
use super::*;
use crate::model::{Model, NodalLoad, Node, Support, solve_linear_static};

/// ↕️ The y-bending blocks of `local_mass` and `local_geometric_stiffness` must be `S·B·S` of the
/// z-bending blocks (`S = diag(1, −1, 1, −1)`), the same `θy = −∂w/∂x` flip `local_stiffness` carries.
#[test]
fn frame3_y_plane_mass_and_geometric_blocks_carry_the_theta_y_sign_flip() {
    let frame = Frame3 { id: "e1".into(), node_a: "a".into(), node_b: "b".into(), e: 210e9, g: 80.77e9, a: 0.005, iy: 1e-5, iz: 1e-5, j: 1e-6, roll: 0.0, density: 7850.0 };
    let l = 3.0;
    let z = [1usize, 5, 7, 11];
    let y = [2usize, 4, 8, 10];
    for matrix in [frame.local_mass(l), frame.local_geometric_stiffness(l, -1.0e4)] {
        for (bi, (&gz, &gy)) in z.iter().zip(y.iter()).enumerate() {
            for (bj, (&hz, &hy)) in z.iter().zip(y.iter()).enumerate() {
                let expected = Y_PLANE_SIGN[bi] * Y_PLANE_SIGN[bj] * matrix.get(gz, hz);
                assert!((matrix.get(gy, hy) - expected).abs() <= 1e-12 * expected.abs().max(1.0), "({gy},{hy}) = {} vs {expected}", matrix.get(gy, hy));
            }
        }
    }
}

/// 🪵️ Headless axial elongation check along an arbitrary (non-axis-aligned) 3D direction.
#[test]
fn bar3_axial_matches_hand_calc_on_skew_member() {
    // A free 3D joint needs ≥3 non-coplanar bars to be determinate (see `truss_fixture` below),
    // so `b` gets two extra fixed-node bars (to `d` and `c`) besides the member under test (`e1`).
    // Loading exactly along e1's own axis (0.6,0.8,0) makes e1 carry the full load by equilibrium
    // (hand-solved: N_e1 = p, N_bd = N_bc = 0) — a clean, unambiguous check on a genuinely skew direction.
    let (e, a) = (200e9, 0.001);
    let l = 5.0;
    let p = 2000.0;
    let model = Model {
        nodes: vec![
            Node { id: "a".into(), pos: [0.0, 0.0, 0.0] },
            Node { id: "b".into(), pos: [3.0, 4.0, 0.0] }, // length 5 from a, direction (0.6, 0.8, 0.0)
            Node { id: "c".into(), pos: [3.0, 4.0, 2.0] },
            Node { id: "d".into(), pos: [3.0, 0.0, 0.0] },
        ],
        elements: vec![
            Bar3 { id: "e1".into(), node_a: "a".into(), node_b: "b".into(), e, a, density: 0.0 }.into(),
            Bar3 { id: "bc".into(), node_a: "b".into(), node_b: "c".into(), e, a, density: 0.0 }.into(),
            Bar3 { id: "bd".into(), node_a: "b".into(), node_b: "d".into(), e, a, density: 0.0 }.into(),
        ],
        supports: vec![Support { node_id: "a".into(), fixed: vec![Dof::Tx, Dof::Ty, Dof::Tz] }, Support { node_id: "c".into(), fixed: vec![Dof::Tx, Dof::Ty, Dof::Tz] }, Support { node_id: "d".into(), fixed: vec![Dof::Tx, Dof::Ty, Dof::Tz] }],
        nodal_loads: vec![NodalLoad { node_id: "b".into(), dof: Dof::Tx, value: p * 0.6 }, NodalLoad { node_id: "b".into(), dof: Dof::Ty, value: p * 0.8 }],
        member_loads: vec![],
    };
    let result = solve_linear_static(&model).expect("solves");
    let n_e1 = result
        .elements
        .iter()
        .find(|(id, _)| id == "e1")
        .map(|(_, r)| match r {
            ElementResult::Bar { n } => *n,
            _ => panic!("expected bar"),
        })
        .unwrap();
    assert!((n_e1 - p).abs() / p < 1e-6, "axial force {n_e1} vs expected {p}");
    let expected_elongation = p * l / (e * a);
    let b = result.displacements.iter().find(|d| d.node_id == "b").unwrap();
    let actual_elongation = b.values[Dof::Tx.index()] * 0.6 + b.values[Dof::Ty.index()] * 0.8;
    assert!((actual_elongation - expected_elongation).abs() / expected_elongation < 1e-6);
}

/// 🌀️ Rigid-body test: a pure 3D translation must produce zero internal force on a `Frame3`.
#[test]
fn frame3_rigid_translation_gives_zero_force() {
    let frame = Frame3 { id: "e1".into(), node_a: "a".into(), node_b: "b".into(), e: 210e9, g: 80.77e9, a: 0.005, iy: 1e-5, iz: 1e-5, j: 1e-6, roll: 0.0, density: 0.0 };
    let ctx = ElementContext { positions: vec![[0.0, 0.0, 0.0], [2.0, 1.0, 0.5]] };
    let ke = frame.stiffness_global(&ctx);
    let rigid = VecD::from_vec(vec![1.0, 2.0, 3.0, 0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 0.0, 0.0, 0.0]);
    let f = ke.mul_vec(&rigid);
    for i in 0..12 {
        assert!(f.get(i).abs() < 1e-6, "rigid-body force[{i}] = {}", f.get(i));
    }
}

/// 🌬️ Cantilever `Frame3` under a member UDL, checked against the classical cantilever-under-UDL
/// formulas: base moment `wL²/2`, base shear `wL`, ~0 at the free tip. The member runs along
/// global X with `roll: 0.0`, for which `local_system`'s reference-vector logic aligns local y/z
/// with global Y/Z exactly — so a UDL in global `wz` lands directly in the local z-bending plane
/// that `recover` already reports via `v1 = f.get(2)`/`m1 = f.get(4)`.
#[test]
fn frame3_udl_cantilever_matches_hand_calc() {
    let (e, g, a, iy, iz, j) = (200e9, 80e9, 0.01, 8e-5, 8e-5, 1e-6);
    let l = 4.0;
    let w = 1000.0;
    let model = Model {
        nodes: vec![Node { id: "a".into(), pos: [0.0, 0.0, 0.0] }, Node { id: "b".into(), pos: [l, 0.0, 0.0] }],
        elements: vec![Frame3 { id: "e1".into(), node_a: "a".into(), node_b: "b".into(), e, g, a, iy, iz, j, roll: 0.0, density: 0.0 }.into()],
        supports: vec![Support { node_id: "a".into(), fixed: vec![Dof::Tx, Dof::Ty, Dof::Tz, Dof::Rx, Dof::Ry, Dof::Rz] }],
        nodal_loads: vec![],
        member_loads: vec![("e1".into(), MemberUdl { wx: 0.0, wy: 0.0, wz: -w })],
    };
    let result = solve_linear_static(&model).expect("solves");
    let (_, e1_result) = result.elements.iter().find(|(id, _)| id == "e1").unwrap();
    let stations = match e1_result {
        ElementResult::Beam { stations } => stations,
        _ => panic!("expected beam"),
    };
    let base = stations.first().unwrap();
    let tip = stations.last().unwrap();
    let expected_m = w * l * l / 2.0;
    let expected_v = w * l;
    assert!((base.m.abs() - expected_m).abs() / expected_m < 1e-6, "base moment {} vs expected {}", base.m, expected_m);
    assert!((base.v.abs() - expected_v).abs() / expected_v < 1e-6, "base shear {} vs expected {}", base.v, expected_v);
    assert!(tip.m.abs() < expected_m * 1e-6, "tip moment {} should be ~0", tip.m);
    assert!(tip.v.abs() < expected_v * 1e-6, "tip shear {} should be ~0", tip.v);
}

/// 🏋️ `Bar3::mass` matches the hand-derived isotropic `m = ρAL/6` block form (3x3 identity blocks).
#[test]
fn bar3_mass_matches_hand_calc() {
    let (density, a, l) = (7850.0, 0.001, 5.0);
    let bar = Bar3 { id: "e1".into(), node_a: "a".into(), node_b: "b".into(), e: 200e9, a, density };
    let ctx = ElementContext { positions: vec![[0.0, 0.0, 0.0], [3.0, 4.0, 0.0]] };
    let m = bar.mass(&ctx).expect("bar3 reports mass");
    let expected = density * a * l / 6.0;
    for i in 0..3 {
        assert!((m.get(i, i) - 2.0 * expected).abs() < 1e-9);
        assert!((m.get(i + 3, i + 3) - 2.0 * expected).abs() < 1e-9);
        assert!((m.get(i, i + 3) - expected).abs() < 1e-9);
    }
    assert!(m.get(0, 1).abs() < 1e-12, "no coupling across directions");
}

/// ⚖️ Sum of ALL entries of `Bar3::mass` (a pure translational, no-rotation element) equals the
/// total member mass `ρAL` — same partition-of-unity identity as `Bar2`'s.
#[test]
fn bar3_mass_total_equals_rho_a_l() {
    let (density, a, l) = (7850.0, 0.001, 5.0);
    let bar = Bar3 { id: "e1".into(), node_a: "a".into(), node_b: "b".into(), e: 200e9, a, density };
    let ctx = ElementContext { positions: vec![[0.0, 0.0, 0.0], [3.0, 4.0, 0.0]] };
    let m = bar.mass(&ctx).expect("bar3 reports mass");
    let mut sum_x = 0.0;
    for &r in &[0usize, 3] {
        for &c in &[0usize, 3] {
            sum_x += m.get(r, c);
        }
    }
    assert!((sum_x - density * a * l).abs() / (density * a * l) < 1e-9);
}

/// 🏋️ `Frame3::mass`'s axial and torsion 2x2 submatrices each sum to their own hand-derived total
/// (`ρAL` axial, `ρJL` torsion) — checked on a member along global X with `roll: 0.0`, for which
/// `local_system` aligns local axes with global ones exactly (rotation is identity).
#[test]
fn frame3_mass_axial_and_torsion_blocks_sum_to_total() {
    let (e, g, a, iy, iz, j, density) = (200e9, 80e9, 0.01, 8e-5, 8e-5, 1e-6, 7850.0);
    let l = 4.0;
    let frame = Frame3 { id: "e1".into(), node_a: "a".into(), node_b: "b".into(), e, g, a, iy, iz, j, roll: 0.0, density };
    let ctx = ElementContext { positions: vec![[0.0, 0.0, 0.0], [l, 0.0, 0.0]] };
    let m = frame.mass(&ctx).expect("frame3 reports mass");
    let sum_axial = m.get(0, 0) + m.get(0, 6) + m.get(6, 0) + m.get(6, 6);
    assert!((sum_axial - density * a * l).abs() / (density * a * l) < 1e-9);
    let sum_torsion = m.get(3, 3) + m.get(3, 9) + m.get(9, 3) + m.get(9, 9);
    assert!((sum_torsion - density * j * l).abs() / (density * j * l) < 1e-9);
}

/// 🌀️ `Frame3` geometric stiffness must vanish under a pure rigid translation.
#[test]
fn frame3_geometric_stiffness_rigid_translation_gives_zero_force() {
    let frame = Frame3 { id: "e1".into(), node_a: "a".into(), node_b: "b".into(), e: 210e9, g: 80.77e9, a: 0.005, iy: 1e-5, iz: 1e-5, j: 1e-6, roll: 0.0, density: 0.0 };
    let ctx = ElementContext { positions: vec![[0.0, 0.0, 0.0], [2.0, 1.0, 0.5]] };
    let mut u = VecD::zeros(12);
    u.set(6, 0.001);
    let kg = frame.geometric_stiffness(&ctx, &u).expect("frame3 reports geometric stiffness");
    let rigid = VecD::from_vec(vec![1.0, 2.0, 3.0, 0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 0.0, 0.0, 0.0]);
    let f = kg.mul_vec(&rigid);
    for i in 0..12 {
        assert!(f.get(i).abs() < 1e-6, "rigid-body geometric force[{i}] = {}", f.get(i));
    }
}

/// 🌀️ `Frame3` geometric stiffness is symmetric.
#[test]
fn frame3_geometric_stiffness_is_symmetric() {
    let frame = Frame3 { id: "e1".into(), node_a: "a".into(), node_b: "b".into(), e: 210e9, g: 80.77e9, a: 0.005, iy: 1e-5, iz: 1e-5, j: 1e-6, roll: 0.0, density: 0.0 };
    let ctx = ElementContext { positions: vec![[0.0, 0.0, 0.0], [2.0, 1.0, 0.5]] };
    let mut u = VecD::zeros(12);
    u.set(6, 0.001);
    let kg = frame.geometric_stiffness(&ctx, &u).unwrap();
    for r in 0..12 {
        for c in 0..12 {
            assert!((kg.get(r, c) - kg.get(c, r)).abs() < 1e-9, "Kg not symmetric at ({r},{c})");
        }
    }
}

/// 🌬️ `Bar3::equivalent_nodal_loads` splits a global UDL `wL/2` exactly evenly at both nodes —
/// the 3D analogue of `elements2d::bar2_equivalent_nodal_loads_matches_wl_over_2`.
#[test]
fn bar3_equivalent_nodal_loads_matches_wl_over_2() {
    let (e, a, l) = (200e9, 0.001, 5.0);
    let bar = Bar3 { id: "e1".into(), node_a: "a".into(), node_b: "b".into(), e, a, density: 0.0 };
    let ctx = ElementContext { positions: vec![[0.0, 0.0, 0.0], [3.0, 4.0, 0.0]] };
    let udl = MemberUdl { wx: 100.0, wy: -50.0, wz: 20.0 };
    let f = bar.equivalent_nodal_loads(&ctx, &udl).expect("bar3 reports equivalent nodal loads");
    let half = l / 2.0;
    assert!((f.get(0) - udl.wx * half).abs() < 1e-9);
    assert!((f.get(1) - udl.wy * half).abs() < 1e-9);
    assert!((f.get(2) - udl.wz * half).abs() < 1e-9);
    assert!((f.get(3) - udl.wx * half).abs() < 1e-9);
    assert!((f.get(4) - udl.wy * half).abs() < 1e-9);
    assert!((f.get(5) - udl.wz * half).abs() < 1e-9);
}

/// 🌀️ `Bar3::geometric_stiffness`: zero under rigid translation, symmetric, and (same reasoning
/// as `elements2d::bar2_geometric_stiffness_rigid_translation_gives_zero_force_and_is_symmetric`)
/// zero along the bar's own axis.
#[test]
fn bar3_geometric_stiffness_rigid_translation_gives_zero_force_and_is_symmetric() {
    let (e, a) = (200e9, 0.001);
    let bar = Bar3 { id: "e1".into(), node_a: "a".into(), node_b: "b".into(), e, a, density: 0.0 };
    let ctx = ElementContext { positions: vec![[0.0, 0.0, 0.0], [3.0, 4.0, 0.0]] };
    let u = VecD::from_vec(vec![0.0, 0.0, 0.0, 0.001, 0.0, 0.0]);
    let kg = bar.geometric_stiffness(&ctx, &u).expect("bar3 reports geometric stiffness");
    for r in 0..6 {
        for c in 0..6 {
            assert!((kg.get(r, c) - kg.get(c, r)).abs() < 1e-9, "Kg not symmetric at ({r},{c})");
        }
    }
    let rigid = VecD::from_vec(vec![3.0, 4.0, 1.0, 3.0, 4.0, 1.0]);
    let f = kg.mul_vec(&rigid);
    for i in 0..6 {
        assert!(f.get(i).abs() < 1e-6, "rigid-body geometric force[{i}] = {}", f.get(i));
    }
}
