
use super::*;

fn assert_mounted_stiffness_cells_match_batch(element: &dyn Element, context: &ElementContext, tolerance: f64) {
    let batch = element.stiffness_global(context);
    for row in 0..batch.rows {
        for column in 0..batch.cols {
            let mounted = element.mounted_stiffness_cell(context, row, column).expect("mounted fixed-schema cell");
            let scale = batch.get(row, column).abs().max(1.0);
            assert!((mounted - batch.get(row, column)).abs() <= tolerance * scale, "cell ({row},{column}) mounted={mounted} batch={}", batch.get(row, column));
        }
    }
}

#[test]
fn p6h_mounted_element_fixed_schema_cells_match_batch_and_reject_maximum_plus_one() {
    let line = ElementContext { positions: vec![[0.0, 0.0, 0.0], [2.0, 1.0, 0.0]] };
    let bar = Bar2 { id: "bar".into(), start: "a".into(), end: "b".into(), e: 210e9, area: 0.01, density: 0.0 };
    assert_mounted_stiffness_cells_match_batch(&bar, &line, 1e-12);
    assert_eq!(bar.mounted_stiffness_cell(&line, 4, 0), None);

    let beam = BeamEb2 { id: "beam".into(), start: "a".into(), end: "b".into(), e: 210e9, area: 0.01, iy: 8.0e-6, density: 0.0 };
    assert_mounted_stiffness_cells_match_batch(&beam, &line, 1e-12);
    assert_eq!(beam.mounted_stiffness_cell(&line, 6, 0), None);

    let triangle_context = ElementContext { positions: vec![[0.0, 0.0, 0.0], [2.0, 0.0, 0.0], [0.25, 1.5, 0.0]] };
    let triangle = Tri3Cst { id: "triangle".into(), nodes: ["a".into(), "b".into(), "c".into()], e: 30e9, nu: 0.2, thickness: 0.3, kind: PlaneKind::Stress, density: 0.0 };
    assert_mounted_stiffness_cells_match_batch(&triangle, &triangle_context, 1e-12);
    assert_eq!(triangle.mounted_stiffness_cell(&triangle_context, 6, 0), None);
}
use crate::model::{Model, NodalLoad, Node, Support, solve_linear_static};

/// 🪢️ Headless (no document layer) axial elongation check: δ = FL/EA, N = F.
#[test]
fn bar2_axial_matches_hand_calc() {
    let (e, area, l, p) = (200e9, 0.001, 2.0, 5000.0);
    let model = Model {
        nodes: vec![Node { id: "a".into(), pos: [0.0, 0.0, 0.0] }, Node { id: "b".into(), pos: [l, 0.0, 0.0] }],
        elements: vec![Bar2 { id: "e1".into(), start: "a".into(), end: "b".into(), e, area, density: 0.0 }.into()],
        // A single bar only resists motion along its own axis, so `b`'s transverse (Ty) DOF must
        // also be restrained here — otherwise it's a mechanism (zero stiffness, singular system).
        supports: vec![Support { node_id: "a".into(), fixed: vec![Dof::Tx, Dof::Ty] }, Support { node_id: "b".into(), fixed: vec![Dof::Ty] }],
        nodal_loads: vec![NodalLoad { node_id: "b".into(), dof: Dof::Tx, value: p }],
        member_loads: vec![],
    };
    let result = solve_linear_static(&model).expect("solves");
    let expected = p * l / (e * area);
    let b = result.displacements.iter().find(|d| d.node_id == "b").unwrap();
    assert!((b.values[Dof::Tx.index()] - expected).abs() / expected < 1e-9);
    let ElementResult::Bar { n } = result.elements[0].1 else { panic!("expected bar") };
    assert!((n - p).abs() < 1e-6);
}

/// 🏗️ Headless cantilever tip-load check: δ = PL³/3EI, θ = PL²/2EI — the classic beam-theory
/// benchmark, exercised here directly against `fem_core::Model` (no document layer involved).
#[test]
fn beam_eb2_cantilever_matches_hand_calc() {
    let (e, iy, area, l, p) = (200e9, 1e-5, 0.01, 2.0, 1000.0);
    let model = Model {
        nodes: vec![Node { id: "a".into(), pos: [0.0, 0.0, 0.0] }, Node { id: "b".into(), pos: [l, 0.0, 0.0] }],
        elements: vec![BeamEb2 { id: "e1".into(), start: "a".into(), end: "b".into(), e, area, iy, density: 0.0 }.into()],
        supports: vec![Support { node_id: "a".into(), fixed: vec![Dof::Tx, Dof::Ty, Dof::Rz] }],
        nodal_loads: vec![NodalLoad { node_id: "b".into(), dof: Dof::Ty, value: -p }],
        member_loads: vec![],
    };
    let result = solve_linear_static(&model).expect("solves");
    let expected_deflection = p * l.powi(3) / (3.0 * e * iy);
    let expected_rotation = p * l.powi(2) / (2.0 * e * iy);
    let b = result.displacements.iter().find(|d| d.node_id == "b").unwrap();
    assert!((b.values[Dof::Ty.index()].abs() - expected_deflection).abs() / expected_deflection < 1e-6);
    assert!((b.values[Dof::Rz.index()].abs() - expected_rotation).abs() / expected_rotation < 1e-6);
}

/// 🏗️ Straight `BeamEb2` chain of `n` equal elements spanning `l` (nodes `n0..nN`), carrying a
/// downward UDL `w` as a member load on every element, with the given end restraints.
fn beam_udl_span(n: usize, l: f64, w: f64, base: Vec<Dof>, tip: Vec<Dof>) -> crate::model::StaticResult {
    let (e, iy, area) = (200e9, 1e-5, 0.01);
    let id = |i: usize| format!("n{i}");
    let nodes: Vec<Node> = (0..=n).map(|i| Node { id: id(i), pos: [l * i as f64 / n as f64, 0.0, 0.0] }).collect();
    let elements: Vec<Elements> = (0..n).map(|i| BeamEb2 { id: format!("e{i}"), start: id(i), end: id(i + 1), e, area, iy, density: 0.0 }.into()).collect();
    let member_loads = (0..n).map(|i| (format!("e{i}"), MemberUdl { wx: 0.0, wy: -w, wz: 0.0 })).collect();
    let supports = vec![Support { node_id: id(0), fixed: base }, Support { node_id: id(n), fixed: tip }];
    let model = Model { nodes, elements, supports, nodal_loads: vec![], member_loads };
    solve_linear_static(&model).expect("udl span solves")
}

/// 🏗️ Simply supported beam under a UDL, four `BeamEb2` elements: midspan deflection
/// `5wL⁴/384EI`, end rotations `wL³/24EI` and support reactions `wL/2`. Consistent nodal loads
/// make cubic-Hermite nodal values EXACT for a UDL, so all three are asserted to 1e-9 — an
/// independent numpy direct-stiffness solve (`🔨️w7-kernel-references.py` section 6) reproduces
/// the closed form to 3.3e-15.
#[test]
fn beam_eb2_simply_supported_udl_matches_closed_form() {
    let (l, w) = (6.0_f64, 2000.0_f64);
    let (e, iy) = (200e9_f64, 1e-5_f64);
    let result = beam_udl_span(4, l, w, vec![Dof::Tx, Dof::Ty], vec![Dof::Ty]);

    let midspan = result.displacements.iter().find(|d| d.node_id == "n2").unwrap();
    let expected_deflection = -5.0 * w * l.powi(4) / (384.0 * e * iy);
    assert!((midspan.values[Dof::Ty.index()] - expected_deflection).abs() / expected_deflection.abs() < 1e-9, "midspan {} vs {expected_deflection}", midspan.values[Dof::Ty.index()]);

    let expected_rotation = w * l.powi(3) / (24.0 * e * iy);
    for (node_id, sign) in [("n0", -1.0), ("n4", 1.0)] {
        let rotation = result.displacements.iter().find(|d| d.node_id == node_id).unwrap().values[Dof::Rz.index()];
        assert!((rotation - sign * expected_rotation).abs() / expected_rotation < 1e-9, "{node_id} rotation {rotation} vs {}", sign * expected_rotation);
    }
    for node_id in ["n0", "n4"] {
        let reaction = result.reactions.iter().find(|r| r.node_id == node_id && r.dof == Dof::Ty).unwrap().value;
        assert!((reaction - w * l / 2.0).abs() / (w * l / 2.0) < 1e-9, "{node_id} reaction {reaction} vs {}", w * l / 2.0);
    }
}

/// 🏗️ Propped cantilever under a UDL, four `BeamEb2` elements — the classic statically
/// indeterminate reaction set `5wL/8` (fixed end), `3wL/8` (prop) and `wL²/8` (fixed-end moment),
/// exact for consistent nodal loads and reproduced to machine precision by the independent numpy
/// direct-stiffness solve in `🔨️w7-kernel-references.py` section 6 (7500 / 4500 / 9000 N, N·m).
#[test]
fn beam_eb2_propped_cantilever_reactions_match_closed_form() {
    let (l, w) = (6.0_f64, 2000.0_f64);
    let result = beam_udl_span(4, l, w, vec![Dof::Tx, Dof::Ty, Dof::Rz], vec![Dof::Ty]);
    let reaction = |node_id: &str, dof: Dof| result.reactions.iter().find(|r| r.node_id == node_id && r.dof == dof).unwrap().value;
    let expectations = [("n0", Dof::Ty, 5.0 * w * l / 8.0), ("n4", Dof::Ty, 3.0 * w * l / 8.0), ("n0", Dof::Rz, w * l * l / 8.0)];
    for (node_id, dof, expected) in expectations {
        let actual = reaction(node_id, dof);
        assert!((actual - expected).abs() / expected < 1e-9, "{node_id} {dof:?} reaction {actual} vs {expected}");
    }
}

/// 🏗️ Single-bay portal frame (6 m span, 4 m columns, both bases fully fixed) under a 15 kN
/// lateral load at the windward eaves — one `BeamEb2` per member, so the whole global transform
/// path (vertical columns) is exercised. Every asserted number comes from the independent
/// numpy/scipy direct-stiffness solve in `🔨️w7-kernel-references.py` section 6, whose own global
/// moment equilibrium about the origin closes to 9.7e-10 N·m.
#[test]
fn beam_eb2_portal_frame_sway_matches_stiffness_method() {
    let (h, span, load) = (4.0_f64, 6.0_f64, 15000.0_f64);
    let (ec, ac, ic) = (210e9, 0.008, 8.0e-5);
    let (eb, ab, ib) = (210e9, 0.012, 2.0e-4);
    let model = Model {
        nodes: vec![Node { id: "base_left".into(), pos: [0.0, 0.0, 0.0] }, Node { id: "eaves_left".into(), pos: [0.0, h, 0.0] }, Node { id: "eaves_right".into(), pos: [span, h, 0.0] }, Node { id: "base_right".into(), pos: [span, 0.0, 0.0] }],
        elements: vec![
            BeamEb2 { id: "col_left".into(), start: "base_left".into(), end: "eaves_left".into(), e: ec, area: ac, iy: ic, density: 0.0 }.into(),
            BeamEb2 { id: "beam".into(), start: "eaves_left".into(), end: "eaves_right".into(), e: eb, area: ab, iy: ib, density: 0.0 }.into(),
            BeamEb2 { id: "col_right".into(), start: "base_right".into(), end: "eaves_right".into(), e: ec, area: ac, iy: ic, density: 0.0 }.into(),
        ],
        supports: vec![Support { node_id: "base_left".into(), fixed: vec![Dof::Tx, Dof::Ty, Dof::Rz] }, Support { node_id: "base_right".into(), fixed: vec![Dof::Tx, Dof::Ty, Dof::Rz] }],
        nodal_loads: vec![NodalLoad { node_id: "eaves_left".into(), dof: Dof::Tx, value: load }],
        member_loads: vec![],
    };
    let result = solve_linear_static(&model).expect("portal frame solves");

    let displacement = |node_id: &str, dof: Dof| result.displacements.iter().find(|d| d.node_id == node_id).unwrap().values[dof.index()];
    let reaction = |node_id: &str, dof: Dof| result.reactions.iter().find(|r| r.node_id == node_id && r.dof == dof).unwrap().value;
    let references = [
        ("sway left", displacement("eaves_left", Dof::Tx), 3.045764339376e-3),
        ("sway right", displacement("eaves_right", Dof::Tx), 3.027946678835e-3),
        ("eaves rotation left", displacement("eaves_left", Dof::Rz), -3.297738248139e-4),
        ("eaves rotation right", displacement("eaves_right", Dof::Rz), -3.261293033395e-4),
        ("base shear left", reaction("base_left", Dof::Tx), -7516.582572708),
        ("base shear right", reaction("base_right", Dof::Tx), -7483.417427292),
        ("base uplift left", reaction("base_left", Dof::Ty), -4540.867810293),
        ("base uplift right", reaction("base_right", Dof::Ty), 4540.867810293),
        ("base moment left", reaction("base_left", Dof::Rz), 16418.215209634),
        ("base moment right", reaction("base_right", Dof::Rz), 16336.577928609),
    ];
    for (label, actual, expected) in references {
        assert!((actual - expected).abs() / expected.abs() < 1e-9, "{label}: {actual} vs scipy {expected}");
    }
    let base_shear_sum = reaction("base_left", Dof::Tx) + reaction("base_right", Dof::Tx);
    assert!((base_shear_sum + load).abs() / load < 1e-9, "base shears {base_shear_sum} must balance the applied {load}");
    let global_moment = reaction("base_left", Dof::Rz) + reaction("base_right", Dof::Rz) + reaction("base_right", Dof::Ty) * span - load * h;
    assert!(global_moment.abs() / (load * h) < 1e-9, "global moment residual {global_moment}");
}

/// 🌀️ Rigid-body test: a pure translation (no relative deformation) must produce zero internal
/// force — `Ke * rigid_translation ≈ 0`. Catches sign/assembly bugs that a single load case might not.
#[test]
fn beam_eb2_rigid_translation_gives_zero_force() {
    let (e, iy, area, l) = (200e9, 1e-5, 0.01, 2.0);
    let beam = BeamEb2 { id: "e1".into(), start: "a".into(), end: "b".into(), e, area, iy, density: 0.0 };
    let ctx = ElementContext { positions: vec![[0.0, 0.0, 0.0], [l, 0.0, 0.0]] };
    let ke = beam.stiffness_global(&ctx);
    let rigid = VecD::from_vec(vec![3.0, 4.0, 0.0, 3.0, 4.0, 0.0]);
    let f = ke.mul_vec(&rigid);
    for i in 0..6 {
        assert!(f.get(i).abs() < 1e-6, "rigid-body force[{i}] = {}", f.get(i));
    }
}

/// 🏋️ `Bar2::mass` matches the hand-derived isotropic `m = ρAL/6` block form directly.
#[test]
fn bar2_mass_matches_hand_calc() {
    let (density, area, l) = (7850.0, 0.001, 2.0);
    let bar = Bar2 { id: "e1".into(), start: "a".into(), end: "b".into(), e: 200e9, area, density };
    let ctx = ElementContext { positions: vec![[0.0, 0.0, 0.0], [l, 0.0, 0.0]] };
    let m = bar.mass(&ctx).expect("bar2 reports mass");
    let expected = density * area * l / 6.0;
    assert!((m.get(0, 0) - 2.0 * expected).abs() < 1e-9);
    assert!((m.get(1, 1) - 2.0 * expected).abs() < 1e-9);
    assert!((m.get(0, 2) - expected).abs() < 1e-9);
    assert!((m.get(1, 3) - expected).abs() < 1e-9);
    assert!((m.get(0, 1)).abs() < 1e-12, "no coupling between Tx and Ty");
}

/// ⚖️ Consistent-mass physical sanity check: the sum of ALL entries in a pure-translational
/// submatrix (no rotational DOFs involved) must equal the element's total mass `ρAL` — a
/// consequence of the shape functions partitioning unity.
#[test]
fn bar2_mass_total_equals_rho_a_l() {
    let (density, area, l) = (7850.0, 0.001, 2.0);
    let bar = Bar2 { id: "e1".into(), start: "a".into(), end: "b".into(), e: 200e9, area, density };
    let ctx = ElementContext { positions: vec![[0.0, 0.0, 0.0], [l, 0.0, 0.0]] };
    let m = bar.mass(&ctx).expect("bar2 reports mass");
    let sum_tx: f64 = [0, 2].iter().flat_map(|&r| [0, 2].iter().map(move |&c| (r, c))).map(|(r, c)| m.get(r, c)).sum();
    assert!((sum_tx - density * area * l).abs() / (density * area * l) < 1e-9);
}

/// 🏋️ `BeamEb2::mass`'s axial 2x2 submatrix sums to the total member mass `ρAL` (same identity as
/// `Bar2`'s, since the axial DOFs carry no rotational coupling) — checked on a horizontal member so
/// global == local (rotation is identity) and hand-derived indices apply directly.
#[test]
fn beam_eb2_mass_axial_block_sums_to_total_mass() {
    let (e, iy, area, l, density) = (200e9, 1e-5, 0.01, 2.0, 7850.0);
    let beam = BeamEb2 { id: "e1".into(), start: "a".into(), end: "b".into(), e, area, iy, density };
    let ctx = ElementContext { positions: vec![[0.0, 0.0, 0.0], [l, 0.0, 0.0]] };
    let m = beam.mass(&ctx).expect("beam_eb2 reports mass");
    let sum_axial = m.get(0, 0) + m.get(0, 3) + m.get(3, 0) + m.get(3, 3);
    let expected = density * area * l;
    assert!((sum_axial - expected).abs() / expected < 1e-9);
}

/// 🌀️ Geometric stiffness must vanish under a pure rigid translation, same as ordinary stiffness —
/// a non-zero axial force alone shouldn't invent a force from rigid motion.
#[test]
fn beam_eb2_geometric_stiffness_rigid_translation_gives_zero_force() {
    let (e, iy, area, l) = (200e9, 1e-5, 0.01, 2.0);
    let beam = BeamEb2 { id: "e1".into(), start: "a".into(), end: "b".into(), e, area, iy, density: 0.0 };
    let ctx = ElementContext { positions: vec![[0.0, 0.0, 0.0], [l, 0.0, 0.0]] };
    // A pure translation along +x produces a nonzero axial force n = EA/L * dx; geometric
    // stiffness only touches the bending block, so a pure translation still gives zero force there.
    let u = VecD::from_vec(vec![0.0, 0.0, 0.0, 0.001, 0.0, 0.0]);
    let kg = beam.geometric_stiffness(&ctx, &u).expect("beam_eb2 reports geometric stiffness");
    let rigid = VecD::from_vec(vec![3.0, 4.0, 0.0, 3.0, 4.0, 0.0]);
    let f = kg.mul_vec(&rigid);
    for i in 0..6 {
        assert!(f.get(i).abs() < 1e-6, "rigid-body geometric force[{i}] = {}", f.get(i));
    }
}

/// 🌀️ Geometric stiffness is symmetric and scales linearly with the recovered axial force.
#[test]
fn beam_eb2_geometric_stiffness_is_symmetric_and_scales_with_axial_force() {
    let (e, iy, area, l) = (200e9, 1e-5, 0.01, 2.0);
    let beam = BeamEb2 { id: "e1".into(), start: "a".into(), end: "b".into(), e, area, iy, density: 0.0 };
    let ctx = ElementContext { positions: vec![[0.0, 0.0, 0.0], [l, 0.0, 0.0]] };
    let u1 = VecD::from_vec(vec![0.0, 0.0, 0.0, 0.001, 0.0, 0.0]);
    let u2 = VecD::from_vec(vec![0.0, 0.0, 0.0, 0.002, 0.0, 0.0]);
    let kg1 = beam.geometric_stiffness(&ctx, &u1).unwrap();
    let kg2 = beam.geometric_stiffness(&ctx, &u2).unwrap();
    for r in 0..6 {
        for c in 0..6 {
            assert!((kg1.get(r, c) - kg1.get(c, r)).abs() < 1e-9, "Kg not symmetric at ({r},{c})");
            assert!((kg2.get(r, c) - 2.0 * kg1.get(r, c)).abs() < 1e-6, "Kg should scale linearly with axial force at ({r},{c})");
        }
    }
}

/// 🌬️ `Bar2::equivalent_nodal_loads` splits a global UDL `wL/2` exactly evenly at both nodes.
#[test]
fn bar2_equivalent_nodal_loads_matches_wl_over_2() {
    let (e, area, l) = (200e9, 0.001, 2.0);
    let bar = Bar2 { id: "e1".into(), start: "a".into(), end: "b".into(), e, area, density: 0.0 };
    let ctx = ElementContext { positions: vec![[0.0, 0.0, 0.0], [l, 0.0, 0.0]] };
    let udl = MemberUdl { wx: 100.0, wy: -50.0, wz: 0.0 };
    let f = bar.equivalent_nodal_loads(&ctx, &udl).expect("bar2 reports equivalent nodal loads");
    let half = l / 2.0;
    assert!((f.get(0) - udl.wx * half).abs() < 1e-9);
    assert!((f.get(1) - udl.wy * half).abs() < 1e-9);
    assert!((f.get(2) - udl.wx * half).abs() < 1e-9);
    assert!((f.get(3) - udl.wy * half).abs() < 1e-9);
}

/// 🌀️ `Bar2::geometric_stiffness`: zero under rigid translation, symmetric, and destabilizes only
/// the direction PERPENDICULAR to the bar's own axis (an axially-aligned bar with axial force `n`
/// should have ZERO transverse stiffness contribution along its own axis).
#[test]
fn bar2_geometric_stiffness_rigid_translation_gives_zero_force_and_is_symmetric() {
    let (e, area, l) = (200e9, 0.001, 2.0);
    let bar = Bar2 { id: "e1".into(), start: "a".into(), end: "b".into(), e, area, density: 0.0 };
    let ctx = ElementContext { positions: vec![[0.0, 0.0, 0.0], [l, 0.0, 0.0]] };
    let u = VecD::from_vec(vec![0.0, 0.0, 0.001, 0.0]);
    let kg = bar.geometric_stiffness(&ctx, &u).expect("bar2 reports geometric stiffness");
    for r in 0..4 {
        for c in 0..4 {
            assert!((kg.get(r, c) - kg.get(c, r)).abs() < 1e-9, "Kg not symmetric at ({r},{c})");
        }
    }
    let rigid = VecD::from_vec(vec![3.0, 4.0, 3.0, 4.0]);
    let f = kg.mul_vec(&rigid);
    for i in 0..4 {
        assert!(f.get(i).abs() < 1e-6, "rigid-body geometric force[{i}] = {}", f.get(i));
    }
    // Axial member here runs along global X, so `Kg`'s axial (Tx) rows/columns must be zero.
    for i in [0usize, 2] {
        for j in 0..4 {
            assert!(kg.get(i, j).abs() < 1e-6, "Kg({i},{j}) should be zero along the bar's own axis");
        }
    }
}
