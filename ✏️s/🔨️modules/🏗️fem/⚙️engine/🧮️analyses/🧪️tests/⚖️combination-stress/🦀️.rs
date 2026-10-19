use super::*;
use semio_s_fem_elements2d::BeamEb2;
use semio_s_fem_elements2d::PlaneKind;
use semio_s_fem_elements2d::Tri3Cst;
use crate::elements3d::ShellFacet3;
use crate::elements3d::Tet4;

fn model(kind: &str) -> AnalysisModel {
    let mut nodes = vec![Node { id: "a".into(), pos: [0.0, 0.0, 0.0] }, Node { id: "b".into(), pos: [1.0, 0.0, 0.0] }, Node { id: "c".into(), pos: [0.0, 1.0, 0.0] }];
    let element: Elements = match kind {
        "plane" => Tri3Cst { id: "e".into(), nodes: ["a".into(), "b".into(), "c".into()], e: 1000.0, nu: 0.25, thickness: 0.2, kind: PlaneKind::Stress, density: 0.0 }.into(),
        "solid" => {
            nodes.push(Node { id: "d".into(), pos: [0.0, 0.0, 1.0] });
            Tet4 { id: "e".into(), nodes: ["a".into(), "b".into(), "c".into(), "d".into()], e: 1000.0, nu: 0.25, density: 0.0 }.into()
        }
        "shell" => ShellFacet3 { id: "e".into(), nodes: ["a".into(), "b".into(), "c".into()], e: 1000.0, nu: 0.25, thickness: 0.2, density: 0.0 }.into(),
        _ => unreachable!(),
    };
    let supports = nodes.iter().map(|node| {
        let free = match node.id.as_str() { "b" => Some(Dof::Tx), "c" => Some(Dof::Ty), "d" => Some(Dof::Tz), _ => None };
        Support { node_id: node.id.clone(), fixed: Dof::ALL.into_iter().filter(|dof| Some(*dof) != free).collect() }
    }).collect();
    AnalysisModel { nodes, elements: vec![element], supports }
}

fn close(actual: f64, expected: f64, id: &str) {
    assert!((actual - expected).abs() <= 1e-9 * expected.abs().max(1.0), "{id}: {actual} != {expected}");
}

/// ⚖️ Portable vectors require equivalent stresses from the combined tensor, including reversed factors.
#[test]
fn combination_stress_matches_portable_tensor_invariants() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/⚖️combination-stress/🔣️.json")).unwrap();
    for vector in fixture["cases"].as_array().unwrap() {
        let id = vector["id"].as_str().unwrap();
        let kind = vector["kind"].as_str().unwrap();
        let model = model(kind);
        let cases: Vec<LoadCase> = vector["loads"].as_array().unwrap().iter().enumerate().map(|(index, loads)| LoadCase {
            id: format!("case-{index}"),
            nodal_loads: [("b", Dof::Tx), ("c", Dof::Ty), ("d", Dof::Tz)].into_iter().enumerate().filter(|(slot, _)| kind == "solid" || *slot < 2).map(|(slot, (node, dof))| NodalLoad { node_id: node.into(), dof, value: loads[slot].as_f64().unwrap() }).collect(),
            member_loads: vec![], self_weight: false,
        }).collect();
        let combo = Combination { id: id.into(), terms: vector["factors"].as_array().unwrap().iter().enumerate().map(|(index, factor)| (format!("case-{index}"), factor.as_f64().unwrap())).collect() };
        let results = solve_multi_case(&model, &cases, &[combo], [0.0; 3]).unwrap();
        let result = &results[&ResultSource::Combination(id.into())];
        let (stress, equivalent) = match &result.elements[0].1 {
            ElementResult::Plane { gauss } => ([gauss[0].sxx, gauss[0].syy, 0.0], vec![gauss[0].von_mises]),
            ElementResult::Solid { gauss } => ([gauss[0].sxx, gauss[0].syy, gauss[0].szz], vec![gauss[0].von_mises]),
            ElementResult::Shell { gauss } => ([gauss[0].nxx / 0.2, gauss[0].nyy / 0.2, 0.0], vec![gauss[0].von_mises_top, gauss[0].von_mises_bottom]),
            other => panic!("{id}: unexpected {other:?}"),
        };
        for (slot, actual) in stress.into_iter().enumerate() { close(actual, vector["expectedStress"][slot].as_f64().unwrap(), id); }
        for actual in equivalent { close(actual, vector["expectedVonMises"].as_f64().unwrap(), id); assert!(actual >= 0.0); }
        println!("[DEBUG] {id}: combined stress {stress:?}; equivalent stress matches independent principal-stress fixture");
    }
}

/// 🌬️ Recovered combined beam forces retain the weighted fixed-end member load contribution.
#[test]
fn combination_member_load_recovery_matches_direct_solve() {
    let model = AnalysisModel {
        nodes: vec![Node { id: "a".into(), pos: [0.0; 3] }, Node { id: "b".into(), pos: [2.0, 0.0, 0.0] }],
        elements: vec![BeamEb2 { id: "e1".into(), start: "a".into(), end: "b".into(), e: 1000.0, area: 0.2, iy: 0.01, density: 0.0 }.into()],
        supports: vec![Support { node_id: "a".into(), fixed: vec![Dof::Tx, Dof::Ty, Dof::Rz] }],
    };
    let cases = vec![LoadCase { id: "a".into(), nodal_loads: vec![], member_loads: vec![("e1".into(), MemberUdl { wy: -3.0, ..MemberUdl::default() })], self_weight: false }, LoadCase { id: "b".into(), nodal_loads: vec![], member_loads: vec![("e1".into(), MemberUdl { wy: 2.0, ..MemberUdl::default() })], self_weight: false }, LoadCase { id: "direct".into(), nodal_loads: vec![], member_loads: vec![("e1".into(), MemberUdl { wy: -7.0, ..MemberUdl::default() })], self_weight: false }];
    let results = solve_multi_case(&model, &cases, &[Combination { id: "combined".into(), terms: vec![("a".into(), 2.0), ("b".into(), -0.5)] }], [0.0; 3]).unwrap();
    let combined = &results[&ResultSource::Combination("combined".into())];
    let direct = &results[&ResultSource::LoadCase("direct".into())];
    match (&combined.elements[0].1, &direct.elements[0].1) {
        (ElementResult::Beam { stations: combined }, ElementResult::Beam { stations: direct }) => {
            assert_eq!(combined.len(), direct.len());
            for (combined, direct) in combined.iter().zip(direct) { close(combined.x, direct.x, "beam x"); close(combined.n, direct.n, "beam axial"); close(combined.v, direct.v, "beam shear"); close(combined.m, direct.m, "beam moment"); }
        }
        _ => panic!("expected beam station recovery"),
    }
    println!("[DEBUG] combined signed member loads preserve direct-solve beam station forces");
}
