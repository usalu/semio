//! 🏋️ Exact case gravity vectors traverse the authentic owned FEM exchange entries.
use super::nodal_support::*;
use super::*;
use semio_framework_value::ValueRefusalKind;
use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcTypedValue;

#[test]
fn original_owned_self_weight_round_trips_shared_boolean_cases_without_extra_actions() {
    let flags: Vec<bool> = serde_json::from_str(include_str!("commands.json")).unwrap();
    for flag in flags {
        let mut source = input("Tz", (-1000f64).to_bits());
        source.load_cases[0].self_weight = flag;
        let graph = export(&source, wallet(), usize::MAX).0.unwrap();
        let case = graph.entities.iter().find(|row| row.name == "IFCSTRUCTURALLOADCASE").unwrap();
        assert_eq!(case.args[10], if flag { IfcValue::Aggregate(vec![IfcValue::Real(0.), IfcValue::Real(0.), IfcValue::Real(-1.)]) } else { IfcValue::Unset });
        assert_eq!(graph.entities.iter().filter(|row| row.name == "IFCSTRUCTURALPOINTACTION").count(), 1);
        let target = import(&graph, wallet(), usize::MAX).0.unwrap();
        assert_eq!(target.load_cases[0].self_weight, flag);
        assert_eq!(records(&target), vec![(2, (-1000f64).to_bits())]);
        retire(target);
        retire(graph);
    }
}

#[test]
fn original_owned_implicit_weight_case_remains_distinct_from_empty_false_case() {
    let mut source = input("Tz", 0);
    source.load_cases[0].self_weight = true;
    source.load_cases[0].loads.clear();
    let mut other = source.load_cases[0].clone();
    other.id = "empty".into();
    other.name = "Empty".into();
    other.self_weight = false;
    source.load_cases.push(other);
    let graph = export(&source, wallet(), usize::MAX).0.unwrap();
    assert!(!graph.entities.iter().any(|row| row.name == "IFCSTRUCTURALPOINTACTION"));
    let target = import(&graph, wallet(), usize::MAX).0.unwrap();
    assert_eq!(target.load_cases.iter().map(|row| (row.self_weight, row.loads.len())).collect::<Vec<_>>(), vec![(true, 0), (false, 0)]);
    retire(target);
    retire(graph);
}

#[test]
fn original_owned_foreign_self_weight_vectors_consume_the_same_neutral_contract() {
    let data: serde_json::Value = serde_json::from_str(include_str!("cases.json")).unwrap();
    for row in data["cases"].as_array().unwrap() {
        let graph = export(&input("Tx", 0), wallet(), usize::MAX).0.unwrap();
        let mut graph = graph;
        let case = graph.entities.iter_mut().find(|entity| entity.name == "IFCSTRUCTURALLOADCASE").unwrap();
        let kind = row["kind"].as_str().unwrap();
        if kind == "missing" {
            case.args.pop();
        } else {
            case.args[10] = match kind {
                "unset" => IfcValue::Unset,
                "derived" => IfcValue::Derived,
                "scalar" => IfcValue::Real(-1.),
                _ => IfcValue::Aggregate(
                    row["values"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .enumerate()
                        .map(|(index, value)| {
                            let number = value.as_f64().unwrap();
                            match kind {
                                "integer" => IfcValue::Integer(number as i64),
                                "typed" | "wrong-typed" => IfcValue::TypedValue(IfcTypedValue { name: if kind == "typed" { "IFCRATIOMEASURE" } else { "IFCLENGTHMEASURE" }.into(), items: vec![IfcValue::Real(number)] }),
                                "component-unset" if index == 1 => IfcValue::Unset,
                                _ => IfcValue::Real(number),
                            }
                        })
                        .collect(),
                ),
            };
        }
        match import(&graph, wallet(), usize::MAX).0 {
            Ok(target) => {
                assert_eq!(Some(target.load_cases[0].self_weight), row["expected"].as_bool());
                retire(target);
            }
            Err(_) => assert!(row.get("expected").is_none(), "{}", row["id"]),
        }
        retire(graph);
    }
}

#[test]
fn original_self_weight_denial_cancellation_and_paid_recipient_close_use_real_routes() {
    let mut source = input("Tz", 0);
    source.load_cases[0].self_weight = true;
    let (result, turns) = export(&source, wallet(), usize::MAX);
    let graph = result.unwrap();
    let (result, import_turns) = import(&graph, wallet(), usize::MAX);
    retire(result.unwrap());
    let mut denied = wallet();
    denied.maximum_items = 0;
    assert_eq!(export(&source, denied, usize::MAX).0.unwrap_err(), ValueRefusalKind::OwnershipLimit);
    assert_eq!(import(&graph, denied, usize::MAX).0.unwrap_err(), ValueRefusalKind::OwnershipLimit);
    for stop in [1, turns / 2, turns] {
        assert!(export(&source, wallet(), stop).0.is_err());
    }
    for stop in [1, import_turns / 2, import_turns] {
        assert!(import(&graph, wallet(), stop).0.is_err());
    }
    retire(graph);
}
