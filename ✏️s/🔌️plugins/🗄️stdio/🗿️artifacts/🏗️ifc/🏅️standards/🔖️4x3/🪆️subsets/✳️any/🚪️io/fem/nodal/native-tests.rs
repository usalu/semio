//! 🧪️ Native loaded-FEM laws target actual owned Pack routes and shared nodal vectors.
use super::*;
use semio_framework_os_kernel::{ArtifactPackReceiving, NativeSnapshotDecodeOwner};
use semio_framework_value::native_encoding::NativeEncodeRetirementRecipient;
use semio_framework_value::{NativeDecodeControl, NativeEncodeControl, RetainedCloneGrant, RetireOwned, ValueRefusalKind};
use semio_framework_value_native_owned::native_decoding::NativeDecodeRetirementRecipient;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::{
    FemMaterialValue, FemNodeValue,
    snapshot::{Fem3dSnapshot, Fem3dSnapshotValue},
    structure::{FemDof, FemElement, FemLoad, FemLoadCase, FemSection},
};
use std::cell::Cell;
use super::nodal_support::*;
#[test]
fn native_owned_nodal_routes_consume_the_same_eighteen_component_vectors() {
    let vectors = cases();
    let mut count = 0;
    for name in vectors["dofs"].as_array().unwrap() {
        for value in vectors["values"].as_array().unwrap() {
            let name = name.as_str().unwrap();
            let bits = value["bits"].as_str().unwrap().parse::<u64>().unwrap();
            let source = input(name, bits);
            let (graph, _) = export(&source, wallet(), usize::MAX);
            let graph = graph.expect("actual owned loaded FEM export must succeed");
            let row = &graph.entities[force(&graph)];
            let selected = rank(dof(name));
            for k in 0..6 {
                if k == selected {
                    assert!(matches!(row.args[k+1],IfcValue::Real(number)if number.to_bits()==bits));
                } else {
                    assert_eq!(row.args[k + 1], IfcValue::Unset);
                }
            }
            let (restored, _) = import(&graph, wallet(), usize::MAX);
            let restored = restored.expect("actual owned loaded FEM import must succeed");
            assert_eq!(records(&restored), vec![(selected, bits)]);
            let FemLoad::Nodal { node_id, .. } = &restored.load_cases[0].loads[0] else { unreachable!() };
            assert_eq!(node_id, restored.nodes.get(1).unwrap().id.as_str());
            if let Ok(path) = std::env::var("SEMIO_IFC_NATIVE_ORACLE_OUTPUT") {
                std::fs::create_dir_all(&path).unwrap();
                std::fs::write(std::path::Path::new(&path).join(format!("native-nodal-{name}-{}.ifc", value["id"].as_str().unwrap())), crate::io::write_ifc4x3_step(&graph).unwrap()).unwrap();
            }
            retire(restored);
            retire(graph);
            count += 1;
        }
    }
    assert_eq!(count, contract()["selectedCases"].as_u64().unwrap() as usize);
}
#[test]
fn native_original_query_locates_the_actual_force_component_and_canonical_inverse() {
    use semio_s_artifact_stdio_ifc_4_mutations::standards::v4::subsets::any::schema::mutations::{IfcMutation, set_entity_arg::SetEntityArg};
    let (graph, _) = export(&input("Tz", 0), wallet(), usize::MAX);
    let graph = graph.unwrap();
    let ordinal = force(&graph);
    let mut observe = |_| true;
    let mut recipient = NativeDecodeRetirementRecipient::new();
    let mut native = NativeDecodeControl::new(256 * 1024 * 1024, &mut observe);
    native.install_retirement_recipient(&mut recipient).unwrap();
    let index = {
        let mut original = NativeSnapshotDecodeOwner::new(&mut native, wallet());
        crate::catalog::query::with_ifc4x3_native_attribute(&graph, ordinal, "ForceZ", Some("IFCSTRUCTURALLOADSINGLEFORCE"), 71, &mut original, |query, _| {
            assert_eq!(query.status, crate::catalog::query::IfcAttributeQueryStatus::Unique);
            let found = query.matches[0].as_ref().unwrap();
            assert!(std::ptr::eq(found.value.unwrap(), &graph.entities[ordinal].args[found.address.index]));
            Ok(found.address.index)
        })
        .unwrap()
    };
    assert!(!native.has_retirement_owner());
    let mutation = IfcMutation::SetEntityArg(SetEntityArg { id: graph.entities[ordinal].id, index, value: IfcValue::Real(5.) });
    let change = protocol::ColdMutation::diff(&mutation, &graph);
    let next = protocol::apply_diff_cold(change.diff(), &graph).unwrap();
    let restored = protocol::apply_diff_cold(&change.diff().inverse(&graph), &next).unwrap();
    assert_eq!(restored.encode_pack(),graph.encode_pack());
    assert!(matches!(restored.entities[ordinal].args[index],IfcValue::Real(value)if value.to_bits()==0));
    let (a, _) = import(&graph, wallet(), usize::MAX);
    let (b, _) = import(&next, wallet(), usize::MAX);
    let (c, _) = import(&restored, wallet(), usize::MAX);
    let (a, b, c) = (a.unwrap(), b.unwrap(), c.unwrap());
    assert_eq!(records(&a), vec![(2, 0)]);
    assert_eq!(records(&b), vec![(2, 5f64.to_bits())]);
    assert_eq!(records(&a), records(&c));
    retire((a, b, c));
    retire((graph, next, restored));
}
#[test]
fn native_foreign_explicit_components_preserve_zero_presence_and_order() {
    let data = cases();
    for vector in data["vectors"].as_array().unwrap() {
        let (graph, _) = export(&input("Tz", 0), wallet(), usize::MAX);
        let mut graph = graph.unwrap();
        let ordinal = force(&graph);
        for k in 0..6 {
            graph.entities[ordinal].args[k + 1] = IfcValue::Unset;
        }
        let mut expected = Vec::new();
        for item in vector["components"].as_array().unwrap() {
            let k = rank(dof(item["dof"].as_str().unwrap()));
            let bits = item["bits"].as_str().unwrap().parse::<u64>().unwrap();
            graph.entities[ordinal].args[k + 1] = IfcValue::Real(f64::from_bits(bits));
            expected.push((k, bits));
        }
        let (restored, _) = import(&graph, wallet(), usize::MAX);
        let restored = restored.unwrap();
        assert_eq!(records(&restored), expected);
        retire(restored);
        retire(graph);
    }
}
#[test]
fn native_named_empty_cases_and_case_scoped_load_identities_are_not_dropped() {
    let mut source = input("Tx", 0);
    source.load_cases.push(FemLoadCase { id: "other".into(), name: "Other".into(), loads: source.load_cases[0].loads.clone(), self_weight: false });
    source.load_cases.push(FemLoadCase { id: "empty".into(), name: "Empty".into(), loads: Vec::new(), self_weight: false });
    let (graph, _) = export(&source, wallet(), usize::MAX);
    let graph = graph.unwrap();
    let (restored, _) = import(&graph, wallet(), usize::MAX);
    let restored = restored.unwrap();
    assert_eq!(restored.load_cases.len(), 3);
    assert_eq!(restored.load_cases.iter().map(|row| row.loads.len()).collect::<Vec<_>>(), vec![1, 1, 0]);
    assert_eq!(restored.load_cases[2].name, "Empty");
    retire(restored);
    retire(graph);
}
#[test]
fn native_original_all_axis_denial_never_publishes_a_loaded_projection() {
    let source = input("Tz", 0);
    let (graph, _) = export(&source, wallet(), usize::MAX);
    let graph = graph.unwrap();
    for axis in 0..5 {
        let mut grant = wallet();
        match axis {
            0 => grant.maximum_items = 0,
            1 => grant.maximum_copy_bytes = 0,
            2 => grant.maximum_capacity_bytes = 0,
            3 => grant.maximum_release_bytes = 0,
            _ => grant.maximum_depth = 0,
        }
        assert!(export(&source, grant, usize::MAX).0.is_err());
        assert!(import(&graph, grant, usize::MAX).0.is_err());
    }
    retire(graph);
}
#[test]
fn native_every_actual_export_import_observer_prefix_retains_paid_close() {
    let source = input("Rz", (-0f64).to_bits());
    let (graph, export_events) = export(&source, wallet(), usize::MAX);
    let graph = graph.unwrap();
    let (restored, import_events) = import(&graph, wallet(), usize::MAX);
    retire(restored.unwrap());
    for stop in 1..=export_events {
        let result = export(&source, wallet(), stop).0;
        assert_eq!(result.unwrap_err(), ValueRefusalKind::Canceled);
    }
    for stop in 1..=import_events {
        let result = import(&graph, wallet(), stop).0;
        assert_eq!(result.unwrap_err(), ValueRefusalKind::Canceled);
    }
    retire(graph);
}
#[test]
fn native_explicit_zero_components_require_actual_force_and_torque_units() {
    let data = cases();
    for fault in data["refusals"].as_array().unwrap() {
        let fault = fault.as_str().unwrap();
        let name = if fault.contains("torque") { "Rx" } else { "Tx" };
        let (graph, _) = export(&input(name, 0), wallet(), usize::MAX);
        let mut graph = graph.unwrap();
        let ordinal = force(&graph);
        let component = rank(dof(name)) + 1;
        match fault {
            "no-components" => {
                for k in 1..7 {
                    graph.entities[ordinal].args[k] = IfcValue::Unset;
                }
            }
            "invalid-component" => graph.entities[ordinal].args[component] = IfcValue::Enum("T".into()),
            "derived-component" => graph.entities[ordinal].args[component] = IfcValue::Derived,
            "wrong-force-unit-name" => {
                let unit = graph.entities.iter_mut().find(|row| row.name == "IFCSIUNIT" && row.args.get(1).and_then(IfcValue::as_enum) == Some("FORCEUNIT")).unwrap();
                unit.args[3] = IfcValue::Enum("PASCAL".into());
            }
            "scaled-force-unit" => {
                let unit = graph.entities.iter_mut().find(|row| row.name == "IFCSIUNIT" && row.args.get(1).and_then(IfcValue::as_enum) == Some("FORCEUNIT")).unwrap();
                unit.args[2] = IfcValue::Enum("KILO".into());
            }
            "wrong-torque-dimensions" => {
                let unit = graph.entities.iter().find(|row| row.name == "IFCDERIVEDUNIT" && row.args.get(1).and_then(IfcValue::as_enum) == Some("TORQUEUNIT")).unwrap();
                let IfcValue::Aggregate(parts) = &unit.args[0] else { panic!("torque elements") };
                let id = parts[0].as_reference().unwrap();
                graph.entities.iter_mut().find(|row| row.id == id).unwrap().args[1] = IfcValue::Integer(2);
            }
            "missing-force-unit" | "missing-torque-unit" => {
                let tag = if name == "Rx" { "TORQUEUNIT" } else { "FORCEUNIT" };
                let ids = graph.entities.iter().filter(|row| row.args.get(1).and_then(IfcValue::as_enum) == Some(tag)).map(|row| row.id).collect::<Vec<_>>();
                let unit = graph.entities.iter_mut().find(|row| row.name == "IFCUNITASSIGNMENT").unwrap();
                let IfcValue::Aggregate(values) = &mut unit.args[0] else { panic!("units") };
                values.retain(|value| value.as_reference().is_none_or(|id| !ids.contains(&id)));
            }
            "duplicate-force-unit" | "duplicate-torque-unit" => {
                let tag = if name == "Rx" { "TORQUEUNIT" } else { "FORCEUNIT" };
                let mut extra = graph.entities.iter().find(|row| row.args.get(1).and_then(IfcValue::as_enum) == Some(tag)).unwrap().clone();
                extra.id = graph.entities.iter().map(|row| row.id).max().unwrap() + 1;
                let id = extra.id;
                graph.entities.push(extra);
                let unit = graph.entities.iter_mut().find(|row| row.name == "IFCUNITASSIGNMENT").unwrap();
                let IfcValue::Aggregate(values) = &mut unit.args[0] else { panic!("units") };
                values.push(IfcValue::Reference(id));
            }
            _ => panic!("unconsumed neutral refusal"),
        }
        let before = graph.encode_pack();
        let (result, _) = import(&graph, wallet(), usize::MAX);
        assert!(result.is_err(), "{fault}");
        assert_eq!(graph.encode_pack(), before);
        retire(graph);
    }
}
