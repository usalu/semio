use super::*;
use semio_framework_value::{NativeDecodeControl, RetainedCloneGrant};
use semio_framework_value_native_owned::native_decoding::NativeDecodeRetirementRecipient;
use std::cell::Cell;
fn grant() -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 1_000_000, maximum_copy_bytes: 64 * 1048576, maximum_capacity_bytes: 64 * 1048576, maximum_release_bytes: 64 * 1048576, maximum_depth: 128 }
}
fn enumeration(value: &str) -> IfcValue { IfcValue::Enum(value.into()) }
fn fixture(row: &serde_json::Value) -> IfcSnapshot {
    let named = |id, name: &str, args| IfcEntity { id, name: name.into(), args, complex: vec![] };
    let prefix = row["prefix"].as_str().map_or(IfcValue::Unset, enumeration);
    let mut rows = if row["form"] == "si" {
        vec![named(1, "IFCSIUNIT", vec![IfcValue::Derived, enumeration(row["tag"].as_str().unwrap()), prefix, enumeration(row["name"].as_str().unwrap())])]
    } else {
        vec![
            named(1, "IFCDERIVEDUNIT", vec![IfcValue::Aggregate(vec![IfcValue::Reference(4), IfcValue::Reference(5)]), enumeration(row["tag"].as_str().unwrap()), IfcValue::Unset, IfcValue::Unset]),
            named(2, "IFCSIUNIT", vec![IfcValue::Derived, enumeration("FORCEUNIT"), prefix, enumeration(row["name"].as_str().unwrap())]),
            named(3, "IFCSIUNIT", vec![IfcValue::Derived, enumeration("LENGTHUNIT"), IfcValue::Unset, enumeration("METRE")]),
            named(4, "IFCDERIVEDUNITELEMENT", vec![IfcValue::Reference(2), IfcValue::Integer(1)]),
            named(5, "IFCDERIVEDUNITELEMENT", vec![IfcValue::Reference(3), IfcValue::Integer(row["lengthExponent"].as_i64().unwrap())]),
        ]
    };
    if row["extraDerivedZero"].as_bool()==Some(true) {
        let IfcValue::Aggregate(elements)=&mut rows[0].args[0] else { unreachable!() };
        elements.push(IfcValue::Reference(7));
        rows.extend([
            named(6,"IFCDERIVEDUNIT",vec![IfcValue::Aggregate(vec![IfcValue::Reference(10),IfcValue::Reference(11)]),enumeration("MASSDENSITYUNIT"),IfcValue::Unset,IfcValue::Unset]),
            named(7,"IFCDERIVEDUNITELEMENT",vec![IfcValue::Reference(6),IfcValue::Integer(0)]),
            named(8,"IFCSIUNIT",vec![IfcValue::Derived,enumeration("MASSUNIT"),enumeration("KILO"),enumeration("GRAM")]),
            named(10,"IFCDERIVEDUNITELEMENT",vec![IfcValue::Reference(8),IfcValue::Integer(1)]),
            named(11,"IFCDERIVEDUNITELEMENT",vec![IfcValue::Reference(3),IfcValue::Integer(-3)]),
        ]);
    }
    rows.push(named(9, "IFCUNITASSIGNMENT", vec![IfcValue::Aggregate((0..row["assignments"].as_u64().unwrap()).map(|_| IfcValue::Reference(1)).collect())]));
    rows.sort_by_key(|row|row.id);
    IfcSnapshot { entities: rows, ..Default::default() }
}
fn role(row: &serde_json::Value) -> LoadUnit {
    match row["role"].as_str().unwrap() { "force" => LoadUnit::Force, "torque" => LoadUnit::Torque, _ => panic!("undeclared unit role") }
}
#[test]
fn actual_native_unit_resolver_consumes_the_closed_fifteen_role_cases() {
    let cases: serde_json::Value = serde_json::from_str(include_str!("units-cases.json")).unwrap();
    for row in cases["cases"].as_array().unwrap() {
        let source = fixture(row);
        let index: Vec<_> = source.entities.iter().enumerate().map(|(rank, value)| (value.id, rank)).collect();
        let graph = Graph { source: &source, index: &index, relations: &[] };
        let mut observe = |_| true;
        let mut recipient = NativeDecodeRetirementRecipient::new();
        let mut native = NativeDecodeControl::new(64 * 1048576, &mut observe);
        native.install_retirement_recipient(&mut recipient).unwrap();
        let mut original = Owner::new(&mut native, grant());
        let assignment=graph.entity(9,&mut original).unwrap();
        let result = graph.require_load_unit(assignment, role(row), &mut original);
        assert_eq!(result.is_ok(), row["valid"].as_bool().unwrap(), "{}", row["id"]);
        if result.is_ok() {
            let (dimensions, scale) = graph.unit(1, &mut [0; 64], 0, &mut original).unwrap();
            assert_eq!(dimensions, role(row).dimensions());
            assert_eq!(scale, 1.);
        }
        assert!(original.progress().copied_items > 0);
        drop(original);
        assert!(!native.has_retirement_owner());
    }
}
#[test]
fn borrowed_unit_work_uses_original_grants_and_preserves_finite_parent_stage() {
    let cases: serde_json::Value = serde_json::from_str(include_str!("units-cases.json")).unwrap();
    let source = fixture(&cases["cases"][7]);
    let index: Vec<_> = source.entities.iter().enumerate().map(|(rank, value)| (value.id, rank)).collect();
    let graph = Graph { source: &source, index: &index, relations: &[] };
    let last = Cell::new((0, 0));
    let mut observe = |p: semio_framework_value_native_owned::native_decoding::NativeDecodeProgress| { last.set((p.completed, p.total)); true };
    let mut recipient = NativeDecodeRetirementRecipient::new();
    let mut native = NativeDecodeControl::new(64 * 1048576, &mut observe);
    native.install_retirement_recipient(&mut recipient).unwrap();
    native.begin_stage(1).unwrap();
    let g = RetainedCloneGrant { maximum_capacity_bytes: 0, maximum_release_bytes: 0, ..grant() };
    let mut original = Owner::new(&mut native, g);
    graph.require_load_unit(source.entities.last().unwrap(), LoadUnit::Torque, &mut original).unwrap();
    assert_eq!(original.progress().retained_capacity_bytes, 0);
    assert_eq!(original.progress().released_bytes, 0);
    drop(original);
    native.step().unwrap();
    native.checkpoint().unwrap();
    assert_eq!(last.get(), (1, 1));
    for g in [RetainedCloneGrant { maximum_items: 0, ..grant() }, RetainedCloneGrant { maximum_copy_bytes: 0, ..grant() }, RetainedCloneGrant { maximum_depth: 0, ..grant() }] {
        assert!(graph.require_load_unit(source.entities.last().unwrap(), LoadUnit::Torque, &mut Owner::new(&mut native, g)).is_err());
    }
}
#[test]
fn every_observed_unit_prefix_cancels_without_owning_or_publishing_a_value() {
    let cases: serde_json::Value = serde_json::from_str(include_str!("units-cases.json")).unwrap();
    let source = fixture(&cases["cases"][7]);
    let index: Vec<_> = source.entities.iter().enumerate().map(|(rank, value)| (value.id, rank)).collect();
    let graph = Graph { source: &source, index: &index, relations: &[] };
    let total = Cell::new(0);
    for stop in std::iter::once(usize::MAX).chain(1..10000) {
        if stop != usize::MAX && stop > total.get() { break; }
        let calls = Cell::new(0);
        let mut observe = |_| { calls.set(calls.get() + 1); calls.get() != stop };
        let mut recipient = NativeDecodeRetirementRecipient::new();
        let mut native = NativeDecodeControl::new(64 * 1048576, &mut observe);
        native.install_retirement_recipient(&mut recipient).unwrap();
        let mut original = Owner::new(&mut native, grant());
        let result = graph.require_load_unit(source.entities.last().unwrap(), LoadUnit::Torque, &mut original);
        if stop == usize::MAX { result.unwrap(); total.set(calls.get()); assert!(total.get() < 10000); } else { assert_eq!(result.unwrap_err().kind, ValueRefusalKind::Canceled); }
        assert_eq!(original.progress().retained_capacity_bytes, 0);
        drop(original);
        assert!(!native.has_retirement_owner());
    }
}
