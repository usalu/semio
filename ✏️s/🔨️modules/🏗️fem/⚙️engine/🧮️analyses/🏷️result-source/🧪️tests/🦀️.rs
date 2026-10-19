use super::*;

/// 🪪️ Portable result identities distinguish identical case and combination names.
#[test]
fn result_source_identities_are_typed_canonical_and_borrowed() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).expect("independent fixture parser");
    for row in fixture["sources"].as_array().expect("sources") {
        let kind = row["kind"].as_str().expect("kind");
        let id = row["id"].as_str().expect("id");
        let identity = row["identity"].as_str().expect("identity");
        let source = ResultSource::from_parts(kind, id.to_owned()).expect("source kind");
        assert_eq!(encode_result_source_identity(&source).expect("identity"), identity);
        assert_eq!(decode_result_source_identity(identity).expect("source"), source);
        let borrowed = borrow_result_source_identity(identity).expect("borrowed source");
        assert_eq!((borrowed.kind, borrowed.id), (kind, id));
        assert_eq!(validate_result_source_identity(identity).expect("admission"), kind);
        assert!(borrowed.id.as_ptr() >= identity.as_ptr());
        assert!((borrowed.id.as_ptr() as usize + borrowed.id.len()) <= identity.as_ptr() as usize + identity.len());
    }
    for identity in fixture["invalid"].as_array().expect("invalid") {
        assert!(validate_result_source_identity(identity.as_str().expect("identity")).is_err());
    }
    println!("[DEBUG] FEM result source identities preserve case/combination namespaces, Unicode and borrowed validation; raw and wrong-kind inputs refuse");
}

/// 🧮️ The physical solver retains both same-named sources and exact superposition.
#[test]
fn result_source_namespaces_preserve_same_named_case_and_combination() {
    use crate::{analyses::{AnalysisModel, Combination, LoadCase, solve_multi_case}, elements2d::BeamEb2, model::{Node, Support, Dof, NodalLoad}};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).expect("independent fixture parser");
    let beam = &fixture["cantilever"];
    let scalar = |key: &str| beam[key].as_f64().expect("scalar");
    let model = AnalysisModel {
        nodes: vec![Node { id: "a".into(), pos: [0.0, 0.0, 0.0] }, Node { id: "b".into(), pos: [scalar("length"), 0.0, 0.0] }],
        elements: vec![BeamEb2 { id: "e".into(), start: "a".into(), end: "b".into(), e: scalar("e"), area: scalar("area"), iy: scalar("iy"), density: 0.0 }.into()],
        supports: vec![Support { node_id: "a".into(), fixed: vec![Dof::Tx, Dof::Ty, Dof::Rz] }],
    };
    let case = LoadCase { id: "same".into(), nodal_loads: vec![NodalLoad { node_id: "b".into(), dof: Dof::Ty, value: scalar("force") }], member_loads: vec![], self_weight: false };
    let combination = Combination { id: "same".into(), terms: vec![("same".into(), scalar("factor"))] };
    let results = solve_multi_case(&model, &[case], &[combination], [0.0; 3]).expect("solve");
    assert_eq!(results.len(), fixture["expected"]["count"].as_u64().expect("count") as usize);
    for (source, expected) in [(ResultSource::LoadCase("same".into()), "loadCaseTy"), (ResultSource::Combination("same".into()), "combinationTy")] {
        let result = results.get(&source).expect("typed result");
        let actual = result.displacements.iter().find(|row| row.node_id == "b").expect("tip").values[Dof::Ty.index()];
        assert!((actual - fixture["expected"][expected].as_f64().expect("oracle")).abs() < 1e-12);
    }
    println!("[DEBUG] FEM physical solver returns two independent same-named typed result sources and their exact scaled displacement");
}
