use super::*;
use crate::catalog::{ifc_attribute_declarations, ifc4x3_catalog};
use crate::details::Ifc4x3DetailsProvider;
use semio_framework_ui_locale::Locale;
use semio_framework_value::{DslValue, ToValue};
use semio_s_artifact_stdio_contract::editing::{SnapshotDetailPathSegment as P, SnapshotDetailsProvider};
use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::{IfcEntity, IfcValue};
#[path = "receive-tests.rs"]
mod original_value_receiving;

fn path(index: usize, suffix: &[P]) -> Vec<P> {
    [vec![P::Key("entities".into()), P::Index(0), P::Key("args".into()), P::Index(index)], suffix.to_vec()].concat()
}

#[test]
fn language_neutral_cases_resolve_official_inherited_attributes() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("tests.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let attributes = ifc_attribute_declarations(case["entity"].as_str().unwrap()).unwrap();
        assert_eq!(attributes[case["position"].as_u64().unwrap() as usize].name, case["attribute"].as_str().unwrap());
    }
    assert!(ifc4x3_catalog().entities.len() > 800);
    for vector in fixture["globalIds"].as_array().unwrap() {
        assert_eq!(encode_global_id(u128::from_str_radix(vector["hex"].as_str().unwrap(), 16).unwrap()), vector["compressed"].as_str().unwrap());
    }
}

#[test]
fn attributes_are_named_localized_and_discriminants_are_hidden() {
    let mut snapshot = IfcSnapshot::default();
    snapshot.entities.push(IfcEntity {
        id: 9_007_199_254_740_993,
        name: "IFCWALL".into(),
        args: vec![IfcValue::String("0abcdefghijklmnopqrstu".into()), IfcValue::Unset, IfcValue::String("Wall".into()), IfcValue::Unset, IfcValue::Unset, IfcValue::Unset, IfcValue::Unset, IfcValue::Unset, IfcValue::Enum("STANDARD".into())],
        complex: Vec::new(),
    });
    let provider = Ifc4x3DetailsProvider::new(&snapshot);
    assert!(provider.presentation(&path(0, &[]), Locale::En).unwrap().label.contains("GlobalId"));
    assert!(provider.presentation(&path(8, &[]), Locale::De).unwrap().description.unwrap().to_ascii_uppercase().contains("IFCWALL"));
    assert_eq!(provider.child_count(&path(8, &[])), 1);
    assert_eq!(provider.object_key(&path(8, &[]), 0).as_deref(), Some("value"));
    assert!(provider.enum_values(&path(8, &[P::Key("value".into())])).contains(&DslValue::String("STANDARD".into())));
    assert!(!provider.item_capabilities(&path(0, &[])).remove);
    assert!(provider.variants(&path(1, &[]), Locale::De).iter().any(|variant| variant.label.contains("Nicht gesetzt")));
}

#[test]
fn aggregate_limits_and_nested_mutations_match_the_host() {
    let snapshot = IfcSnapshot { entities: vec![IfcEntity { id: 1, name: "IFCCARTESIANPOINT".into(), args: vec![IfcValue::Aggregate(vec![IfcValue::Real(0.0)])], complex: Vec::new() }], ..Default::default() };
    let provider = Ifc4x3DetailsProvider::new(&snapshot);
    let aggregate = path(0, &[P::Key("value".into())]);
    assert!(provider.allows_collection_insert(&aggregate));
    assert!(!provider.item_capabilities(&path(0, &[P::Key("value".into()), P::Index(0)])).remove);
    let event = editing::SnapshotEditEvent::SetValue { path: "/entities/0/args/0/value/0/value".into(), value: DslValue::Number(semio_framework_value::Number::Int(4)) };
    let mutations = <Ifc4x3AnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_rules().resolve::<IfcSnapshot, IfcMutation>(&snapshot, &event).unwrap();
    assert!(matches!(mutations.as_slice(), [IfcMutation::SetEntityArg(_)]));
    let outcome = protocol::ColdMutation::diff(&mutations[0], &snapshot);
    let next = protocol::apply_diff_cold(outcome.diff(), &snapshot).unwrap();
    assert_eq!(next.entities[0].args[0], IfcValue::Aggregate(vec![IfcValue::Real(4.0)]));
}

#[test]
fn select_controls_reach_transitive_scalar_alternatives() {
    let snapshot = IfcSnapshot { entities: vec![IfcEntity { id: 1, name: "IFCPROPERTYSINGLEVALUE".into(), args: vec![IfcValue::String("Name".into()), IfcValue::Unset, IfcValue::Unset, IfcValue::Unset], complex: Vec::new() }], ..Default::default() };
    let provider = Ifc4x3DetailsProvider::new(&snapshot);
    let choices = provider.variants(&path(2, &[]), Locale::En);
    let label = choices.iter().find(|variant| variant.label == "IFCLABEL").unwrap();
    assert_eq!(label.value, IfcValue::TypedValue(semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcTypedValue { name: "IFCLABEL".into(), items: vec![IfcValue::String(String::new())] }).to_value());
}

#[test]
fn type_switches_preserve_identity_with_sparse_modifications_and_inverse() {
    use protocol::command::ColdDiffAlgebra;
    let cases: serde_json::Value = serde_json::from_str(include_str!("tests.json")).unwrap();
    let case = &cases["typeSwitch"];
    let id = case["entityId"].as_str().unwrap().parse::<u64>().unwrap();
    let mut snapshot = IfcSnapshot {
        entities: vec![
            IfcEntity {
                id,
                name: case["from"].as_str().unwrap().into(),
                args: vec![
                    IfcValue::String("0000000000000000000000".into()),
                    IfcValue::Unset,
                    IfcValue::String("Opening".into()),
                    IfcValue::Unset,
                    IfcValue::Unset,
                    IfcValue::Unset,
                    IfcValue::Unset,
                    IfcValue::String("Element 1".into()),
                    IfcValue::Enum(case["fromEnum"].as_str().unwrap().into()),
                ],
                complex: Vec::new(),
            },
            IfcEntity {
                id: 2,
                name: "IFCRELAGGREGATES".into(),
                args: vec![IfcValue::String("0000000000000000000001".into()), IfcValue::Unset, IfcValue::Unset, IfcValue::Unset, IfcValue::Reference(3), IfcValue::Aggregate(vec![IfcValue::Reference(id)])],
                complex: Vec::new(),
            },
            IfcEntity { id: 3, name: "IFCGROUP".into(), args: vec![IfcValue::String("0000000000000000000002".into()), IfcValue::Unset, IfcValue::String("Elements".into()), IfcValue::Unset, IfcValue::Unset], complex: Vec::new() },
        ],
        ..Default::default()
    };
    let original = snapshot.clone();
    let entity_path = vec![P::Key("entities".into()), P::Index(0)];
    let provider = Ifc4x3DetailsProvider::new(&snapshot);
    let choice = (0..provider.variant_count(&entity_path, Locale::En)).filter_map(|index| provider.variant(&entity_path, Locale::En, index)).find(|variant| variant.label == case["to"].as_str().unwrap()).unwrap();
    let event = editing::SnapshotEditEvent::SetValue { path: "/entities/0".into(), value: choice.value };
    assert!(<Ifc4x3AnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_is_admitted(&event, &snapshot));
    let mutations = <Ifc4x3AnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, &snapshot).unwrap().artifact_mutations;
    let mut inverses = Vec::new();
    for mutation in mutations {
        assert!(!matches!(mutation, IfcMutation::InsertEntity(_) | IfcMutation::RemoveEntity(_)));
        let outcome = protocol::ColdMutation::diff(&mutation, &snapshot);
        let entities = outcome.diff().entities.as_ref().unwrap();
        assert!(entities.added.is_empty() && entities.removed.is_empty());
        inverses.push(outcome.diff().inverse(&snapshot));
        snapshot = protocol::apply_diff_cold(outcome.diff(), &snapshot).unwrap();
        assert_eq!(snapshot.entities[0].id, id);
        assert_eq!(snapshot.entities[1], original.entities[1]);
    }
    assert_eq!(snapshot.entities[0].name, case["to"].as_str().unwrap());
    assert!(case["resetToUnset"].as_bool().unwrap());
    assert_eq!(snapshot.entities[0].args[8], IfcValue::Unset);
    assert_eq!(snapshot.entities[0].args.len(), case["toAttributes"].as_u64().unwrap() as usize);
    for attribute in case["preserved"].as_array().unwrap() {
        let old = ifc_attribute_declarations(case["from"].as_str().unwrap()).unwrap();
        let new = ifc_attribute_declarations(case["to"].as_str().unwrap()).unwrap();
        let name = attribute.as_str().unwrap();
        assert_eq!(snapshot.entities[0].args[new.iter().position(|attribute| attribute.name == name).unwrap()], original.entities[0].args[old.iter().position(|attribute| attribute.name == name).unwrap()]);
    }
    for inverse in inverses.iter().rev() {
        snapshot = protocol::apply_diff_cold(inverse, &snapshot).unwrap();
    }
    assert_eq!(snapshot, original);
}

#[test]
fn whole_entity_edits_preserve_binary64_sign_bits_and_inverse() {
    use protocol::command::ColdDiffAlgebra;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("tests.json")).unwrap();
    for case in fixture["realEdits"].as_array().unwrap() {
        let before = u64::from_str_radix(case["before"].as_str().unwrap(), 16).unwrap();
        let after = u64::from_str_radix(case["after"].as_str().unwrap(), 16).unwrap();
        let snapshot = IfcSnapshot { entities: vec![IfcEntity { id: 1, name: "IFCCARTESIANPOINT".into(), args: vec![IfcValue::Aggregate(vec![IfcValue::Real(f64::from_bits(before))])], complex: Vec::new() }], ..Default::default() };
        let mut entity = snapshot.entities[0].clone();
        entity.args = vec![IfcValue::Aggregate(vec![IfcValue::Real(f64::from_bits(after))])];
        let event = editing::SnapshotEditEvent::SetValue { path: "/entities/0".into(), value: entity.to_value() };
        assert!(<Ifc4x3AnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_is_admitted(&event, &snapshot));
        let mutations = <Ifc4x3AnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, &snapshot).unwrap().artifact_mutations;
        let [IfcMutation::SetEntityArg(_)] = mutations.as_slice() else { panic!("a sign bit change requires one sparse argument mutation") };
        let outcome = protocol::ColdMutation::diff(&mutations[0], &snapshot);
        let next = protocol::apply_diff_cold(outcome.diff(), &snapshot).unwrap();
        let bits = |snapshot: &IfcSnapshot| match &snapshot.entities[0].args[0] {
            IfcValue::Aggregate(items) => match items[0] {
                IfcValue::Real(value) => value.to_bits(),
                _ => panic!("real required"),
            },
            _ => panic!("aggregate required"),
        };
        assert_eq!(bits(&next), after);
        assert_eq!(bits(&protocol::apply_diff_cold(&outcome.diff().inverse(&snapshot), &next).unwrap()), before);
    }
}

#[test]
fn complex_field_edits_preserve_binary64_sign_bits_and_inverse() {
    use protocol::command::ColdDiffAlgebra;
    use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcComplexType;
    let snapshot = IfcSnapshot {
        entities: vec![IfcEntity { id: 1, name: "IFCREPRESENTATIONITEM".into(), args: Vec::new(), complex: vec![IfcComplexType { name: "IFCCARTESIANPOINT".into(), args: vec![IfcValue::Aggregate(vec![IfcValue::Real(0.0)])] }] }],
        ..Default::default()
    };
    let event = editing::SnapshotEditEvent::SetValue { path: "/entities/0/complex/0/args/0/value/0/value".into(), value: DslValue::float(-0.0) };
    assert!(<Ifc4x3AnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_is_admitted(&event, &snapshot));
    let mutations = <Ifc4x3AnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, &snapshot).unwrap().artifact_mutations;
    let [IfcMutation::SetEntityComplex(_)] = mutations.as_slice() else { panic!("a complex sign bit change requires a sparse complex mutation") };
    let outcome = protocol::ColdMutation::diff(&mutations[0], &snapshot);
    let next = protocol::apply_diff_cold(outcome.diff(), &snapshot).unwrap();
    let bits = |snapshot: &IfcSnapshot| match &snapshot.entities[0].complex[0].args[0] {
        IfcValue::Aggregate(items) => match items[0] {
            IfcValue::Real(value) => value.to_bits(),
            _ => panic!("real required"),
        },
        _ => panic!("aggregate required"),
    };
    assert_eq!(bits(&next), (-0.0_f64).to_bits());
    assert_eq!(bits(&protocol::apply_diff_cold(&outcome.diff().inverse(&snapshot), &next).unwrap()), 0.0_f64.to_bits());
}

#[test]
fn editor_uses_the_distinct_4x3_dialect() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("tests.json")).unwrap();
    assert_eq!(<Ifc4x3AnyEditor as ArtifactEditor>::DIALECT.standard.0, "4x3");
    assert_eq!(IFC4X3_DOCUMENT_SCHEMA, fixture["identities"]["document"].as_str().unwrap());
    assert_eq!(Ifc4x3AnyEditor::initial_snapshot().schema, fixture["identities"]["graph"].as_str().unwrap());
    assert_eq!(create_ifc4x3_any_editor().role, semio_framework_plugin::AppRole::Editor);
}

#[test]
fn entity_ids_use_an_available_positive_u64_even_when_unsorted_or_exhausted_at_the_top() {
    let entity = |id| IfcEntity { id, name: "IFCCARTESIANPOINT".into(), args: vec![IfcValue::Aggregate(vec![IfcValue::Real(0.0)])], complex: Vec::new() };
    let mut snapshot = IfcSnapshot { entities: vec![entity(2), entity(1)], ..Default::default() };
    assert_eq!(next_entity_id(&snapshot).unwrap(), 3);
    snapshot.entities.push(entity(u64::MAX));
    assert_eq!(next_entity_id(&snapshot).unwrap(), 3);
    snapshot.entities.push(entity(3));
    assert_eq!(next_entity_id(&snapshot).unwrap(), 4);
    assert_eq!(next_entity_id(&IfcSnapshot::default()).unwrap(), 1);
    let first = fresh_global_id(&snapshot, 4);
    snapshot.entities.push(IfcEntity { id: 4, name: "IFCWALL".into(), args: vec![IfcValue::String(first.clone())], complex: Vec::new() });
    let second = fresh_global_id(&snapshot, 5);
    assert_ne!(first, second);
    for value in [first, second] {
        assert_eq!(value.len(), 22);
        assert!(matches!(value.as_bytes()[0], b'0'..=b'3'));
        assert!(value.bytes().all(|byte| b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz_$".contains(&byte)));
    }
}

#[test]
fn exact_reference_field_reaches_targets_beyond_the_suggestion_window() {
    let mut snapshot = IfcSnapshot::default();
    snapshot.entities = (1..=300).map(|id| IfcEntity { id, name: "IFCLOCALPLACEMENT".into(), args: vec![IfcValue::Unset, IfcValue::Unset], complex: Vec::new() }).collect();
    snapshot.entities[0].args[0] = IfcValue::Reference(9_007_199_254_740_993);
    let provider = Ifc4x3DetailsProvider::new(&snapshot);
    assert_eq!(provider.value(&path(0, &[P::Key("value".into())])), Some(editing::SnapshotDetailValue::String("9007199254740993".into())));
    assert!(provider.item_capabilities(&path(0, &[P::Key("value".into())])).edit);
    assert_eq!(provider.value(&path(0, &[P::Key("reference".into())])), Some(editing::SnapshotDetailValue::String("9007199254740993".into())));
    snapshot.entities[0].args[0] = IfcValue::Unset;
    let event = editing::SnapshotEditEvent::SetValue { path: "/entities/0/args/0/reference".into(), value: DslValue::String("300".into()) };
    assert!(<Ifc4x3AnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_is_admitted(&event, &snapshot));
    let mutations = <Ifc4x3AnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, &snapshot).unwrap().artifact_mutations;
    let [IfcMutation::SetEntityArg(argument)] = mutations.as_slice() else { panic!("reference field uses an argument mutation") };
    assert_eq!(argument.value, IfcValue::Reference(300));
    let missing = editing::SnapshotEditEvent::SetValue { path: "/entities/0/args/0/reference".into(), value: DslValue::String("301".into()) };
    assert!(<Ifc4x3AnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_special(&missing, &snapshot).is_err());
}

#[test]
fn revisioned_host_render_has_localized_named_controls_and_controller_bindings() {
    let snapshot = IfcSnapshot { entities: vec![IfcEntity { id: 1, name: "IFCCARTESIANPOINT".into(), args: vec![IfcValue::Aggregate(vec![IfcValue::Real(0.0)])], complex: Vec::new() }], ..Default::default() };
    for (locale, attribute_label) in [(Locale::En, "Attributes"), (Locale::De, "Attribute")] {
        let rendered =
            editing::render_snapshot_details_provider_revisioned(&Ifc4x3DetailsProvider::new(&snapshot), semio_framework_ui_contract::UiPublicationRevision(42), locale, IFC4X3_CONTROLLER, &semio_framework_plugin::TreeWindows::unhosted()).unwrap();
        let independent_serialization = serde_json::to_string(&rendered).unwrap();
        assert!(independent_serialization.contains("Coordinates"));
        assert!(independent_serialization.contains(attribute_label));
        assert!(independent_serialization.contains(IFC4X3_CONTROLLER));
        assert!(independent_serialization.contains(editing::SET_SNAPSHOT_VALUE_ACTION_ID));
    }
}

#[test]
fn complex_nested_field_uses_a_sparse_complex_diff_and_inverse() {
    use protocol::command::ColdDiffAlgebra;
    use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcComplexType;
    let snapshot = IfcSnapshot {
        entities: vec![IfcEntity { id: 1, name: "IFCREPRESENTATIONITEM".into(), args: Vec::new(), complex: vec![IfcComplexType { name: "IFCCARTESIANPOINT".into(), args: vec![IfcValue::Aggregate(vec![IfcValue::Real(0.0)])] }] }],
        ..Default::default()
    };
    let event = editing::SnapshotEditEvent::SetValue { path: "/entities/0/complex/0/args/0/value/0/value".into(), value: DslValue::Number(semio_framework_value::Number::Float(7.0)) };
    let mutations = <Ifc4x3AnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_special(&event, &snapshot).unwrap().unwrap();
    assert!(matches!(mutations.as_slice(), [IfcMutation::SetEntityComplex(_)]));
    let outcome = protocol::ColdMutation::diff(&mutations[0], &snapshot);
    let sparse = outcome.diff().entities.as_ref().unwrap();
    assert_eq!(sparse.modified.len(), 1);
    assert!(sparse.added.is_empty() && sparse.removed.is_empty());
    assert!(sparse.modified[0].diff.args.is_none());
    let next = protocol::apply_diff_cold(outcome.diff(), &snapshot).unwrap();
    assert_eq!(protocol::apply_diff_cold(&outcome.diff().inverse(&snapshot), &next).unwrap(), snapshot);
}

#[test]
fn inserting_the_first_complex_constituent_uses_the_optional_field_authority() {
    use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcComplexType;
    let snapshot = IfcSnapshot { entities: vec![IfcEntity { id: 1, name: "IFCREPRESENTATIONITEM".into(), args: Vec::new(), complex: Vec::new() }], ..Default::default() };
    let part = IfcComplexType { name: "IFCCARTESIANPOINT".into(), args: vec![IfcValue::Aggregate(vec![IfcValue::Real(0.0)])] };
    let event = editing::SnapshotEditEvent::InsertValue { path: "/entities/0/complex/0".into(), value: part.to_value() };
    assert!(<Ifc4x3AnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_is_admitted(&event, &snapshot));
    let mutations = <Ifc4x3AnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_special(&event, &snapshot).unwrap().unwrap();
    let [IfcMutation::SetEntityComplex(_)] = mutations.as_slice() else { panic!("first constituent uses the complex mutation") };
    let outcome = protocol::ColdMutation::diff(&mutations[0], &snapshot);
    let next = protocol::apply_diff_cold(outcome.diff(), &snapshot).unwrap();
    assert_eq!(next.entities[0].complex, vec![part]);
    assert!(snapshot.entities[0].complex.is_empty());
}

#[test]
fn the_first_complex_constituent_uses_partial_attributes_too() {
    use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcComplexType;
    let snapshot = IfcSnapshot {
        entities: vec![IfcEntity {
            id: 1,
            name: "IFCWALL".into(),
            args: vec![IfcValue::Enum("STANDARD".into())],
            complex: vec![IfcComplexType { name: "IFCROOT".into(), args: vec![IfcValue::String("0000000000000000000000".into()), IfcValue::Unset, IfcValue::Unset, IfcValue::Unset] }],
        }],
        ..Default::default()
    };
    let provider = Ifc4x3DetailsProvider::new(&snapshot);
    assert!(provider.presentation(&path(0, &[]), Locale::En).unwrap().label.contains("PredefinedType"));
    assert!(provider.enum_values(&path(0, &[P::Key("value".into())])).contains(&DslValue::String("STANDARD".into())));
}

#[test]
fn empty_complex_collections_remain_visible_and_creatable() {
    let snapshot = IfcSnapshot { entities: vec![IfcEntity { id: 1, name: "IFCREPRESENTATIONITEM".into(), args: Vec::new(), complex: Vec::new() }], ..Default::default() };
    let provider = Ifc4x3DetailsProvider::new(&snapshot);
    let entity = vec![P::Key("entities".into()), P::Index(0)];
    let complex = [entity.clone(), vec![P::Key("complex".into())]].concat();
    assert_eq!(provider.child_count(&entity), 4);
    assert_eq!(provider.object_key(&entity, 3).as_deref(), Some("complex"));
    assert_eq!(provider.value(&complex), Some(editing::SnapshotDetailValue::Array));
    assert_eq!(provider.child_count(&complex), 0);
    assert!(provider.allows_collection_insert(&complex));
    assert!(provider.creation_template(&complex, true).is_some());
}

#[test]
fn an_optional_aggregate_does_not_make_its_mandatory_items_optional() {
    let snapshot = IfcSnapshot {
        entities: vec![IfcEntity {
            id: 1,
            name: "IFCCARTESIANPOINTLIST3D".into(),
            args: vec![IfcValue::Aggregate(vec![IfcValue::Aggregate(vec![IfcValue::Real(0.0), IfcValue::Real(0.0), IfcValue::Real(0.0)])]), IfcValue::Aggregate(vec![IfcValue::String("A".into())])],
            complex: Vec::new(),
        }],
        ..Default::default()
    };
    let provider = Ifc4x3DetailsProvider::new(&snapshot);
    assert!(provider.variants(&path(1, &[]), Locale::En).iter().any(|variant| variant.value == IfcValue::Unset.to_value()));
    assert!(!provider.variants(&path(1, &[P::Key("value".into()), P::Index(0)]), Locale::En).iter().any(|variant| variant.value == IfcValue::Unset.to_value()));
}

#[test]
fn unique_reference_collections_choose_an_unused_admitted_target() {
    let unit = |id| IfcEntity { id, name: "IFCSIUNIT".into(), args: vec![IfcValue::Derived, IfcValue::Enum("LENGTHUNIT".into()), IfcValue::Unset, IfcValue::Enum("METRE".into())], complex: Vec::new() };
    let snapshot = IfcSnapshot { entities: vec![IfcEntity { id: 1, name: "IFCUNITASSIGNMENT".into(), args: vec![IfcValue::Aggregate(vec![IfcValue::Reference(2)])], complex: Vec::new() }, unit(2), unit(3)], ..Default::default() };
    let provider = Ifc4x3DetailsProvider::new(&snapshot);
    let collection = path(0, &[P::Key("value".into())]);
    assert!(provider.allows_collection_insert(&collection));
    assert_eq!(provider.creation_template(&collection, true), Some(IfcValue::Reference(3).to_value()));
}

#[test]
fn binary_select_values_have_a_control_and_invalid_bits_never_publish() {
    use semio_s_artifact_stdio_ifc_4_mutations::standards::v4::subsets::any::schema::mutations::set_entity_arg::SetEntityArg;
    let snapshot = IfcSnapshot { entities: vec![IfcEntity { id: 1, name: "IFCPROPERTYSINGLEVALUE".into(), args: vec![IfcValue::String("Bits".into()), IfcValue::Unset, IfcValue::Unset, IfcValue::Unset], complex: Vec::new() }], ..Default::default() };
    let provider = Ifc4x3DetailsProvider::new(&snapshot);
    assert!(provider.variants(&path(2, &[]), Locale::En).iter().any(|variant| variant.label == "IFCBINARY"));
    let mutation = IfcMutation::SetEntityArg(SetEntityArg { id: 1, index: 2, value: IfcValue::Binary("3F".into()) });
    let outcome = protocol::ColdMutation::diff(&mutation, &snapshot);
    assert!(protocol::apply_diff_cold(outcome.diff(), &snapshot).is_err());
    assert_eq!(snapshot.entities[0].args[2], IfcValue::Unset);
}

#[test]
fn model_and_details_windows_share_the_ifc4x3_editor() {
    use semio_framework_plugin::{MeshWindowKit, WindowKit};
    let definition = create_ifc4x3_any_editor();
    let independent = serde_json::to_value(&definition).unwrap();
    let kinds = independent["windowKinds"].as_array().unwrap();
    for kind in [MeshWindowKit::KIND_ID, editing::SNAPSHOT_DETAILS_WINDOW_KIND_ID] {
        assert!(kinds.iter().any(|entry| entry["id"] == kind));
        assert!(independent["defaultLayout"].to_string().contains(kind));
    }
}

#[test]
fn published_view_scope_is_localized_and_does_not_disable_entity_editing() {
    let snapshot = IfcSnapshot { entities: vec![IfcEntity { id: 1, name: "IFCWALL".into(), args: Vec::new(), complex: Vec::new() }], ..Default::default() };
    let provider = Ifc4x3DetailsProvider::new(&snapshot);
    let path = [P::Key("entities".into()), P::Index(0)];
    let english = provider.presentation(&path, Locale::En).unwrap().description.unwrap();
    let german = provider.presentation(&path, Locale::De).unwrap().description.unwrap();
    assert!(english.contains("Reference View") && english.contains("Alignment Based View"));
    assert!(german.contains("Referenzansicht") && german.contains("Trassierungsbasierte Ansicht"));
    assert!(provider.item_capabilities(&path).remove);
    assert!(provider.variant_count(&path, Locale::En) > 700);
}

#[test]
fn reference_collection_append_reaches_every_exact_id_after_suggestions_are_exhausted() {
    use protocol::command::ColdDiffAlgebra;
    let property = |id| IfcEntity { id, name: "IFCPROPERTYSINGLEVALUE".into(), args: vec![IfcValue::String(format!("Property {id}")), IfcValue::Unset, IfcValue::Unset, IfcValue::Unset], complex: Vec::new() };
    let mut snapshot = IfcSnapshot {
        entities: vec![IfcEntity {
            id: 1,
            name: "IFCPROPERTYSET".into(),
            args: vec![IfcValue::String("0000000000000000000000".into()), IfcValue::Unset, IfcValue::String("Reference Set".into()), IfcValue::Unset, IfcValue::Aggregate((2..=257).map(IfcValue::Reference).collect())],
            complex: Vec::new(),
        }],
        ..Default::default()
    };
    snapshot.entities.extend((2..=301).map(property));
    snapshot.entities.push(IfcEntity { id: 302, name: "IFCCARTESIANPOINT".into(), args: vec![IfcValue::Aggregate(vec![IfcValue::Real(0.0)])], complex: Vec::new() });
    let provider = Ifc4x3DetailsProvider::new(&snapshot);
    let field = path(4, &[P::Key("appendReference".into())]);
    assert_eq!(provider.value(&field), Some(editing::SnapshotDetailValue::String(String::new())));
    assert!(provider.item_capabilities(&field).edit);
    assert_eq!(provider.object_key(&path(4, &[]), 1).as_deref(), Some("appendReference"));
    for (locale, label) in [(Locale::En, "Add reference by entity ID"), (Locale::De, "Referenz per Entitäts-ID hinzufügen")] {
        assert_eq!(provider.presentation(&field, locale).unwrap().label, label);
    }
    let event = |value: &str| editing::SnapshotEditEvent::SetValue { path: "/entities/0/args/4/appendReference".into(), value: DslValue::String(value.into()) };
    assert!(<Ifc4x3AnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_is_admitted(&event("301"), &snapshot));
    let mutations = <Ifc4x3AnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&event("301"), &snapshot).unwrap().artifact_mutations;
    assert!(matches!(mutations.as_slice(), [IfcMutation::SetEntityArg(_)]));
    let outcome = protocol::ColdMutation::diff(&mutations[0], &snapshot);
    let next = protocol::apply_diff_cold(outcome.diff(), &snapshot).unwrap();
    assert_eq!(next.entities[0].args[4].as_aggregate().unwrap().last(), Some(&IfcValue::Reference(301)));
    assert_eq!(protocol::apply_diff_cold(&outcome.diff().inverse(&snapshot), &next).unwrap(), snapshot);
    for value in ["2", "302", "303", "0", "", "9007199254740993", "18446744073709551616", "3e2"] {
        assert!(<Ifc4x3AnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_special(&event(value), &snapshot).is_err(), "{value}");
    }
    snapshot.entities.push(property(9_007_199_254_740_993));
    let exact = <Ifc4x3AnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_special(&event("9007199254740993"), &snapshot).unwrap().unwrap();
    let exact = protocol::apply_diff_cold(protocol::ColdMutation::diff(&exact[0], &snapshot).diff(), &snapshot).unwrap();
    assert_eq!(exact.entities[0].args[4].as_aggregate().unwrap().last(), Some(&IfcValue::Reference(9_007_199_254_740_993)));
}

#[test]
fn complex_component_type_controls_create_direct_attributes_and_preserve_identity() {
    use protocol::command::ColdDiffAlgebra;
    use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcComplexType;
    let snapshot = IfcSnapshot { entities: vec![IfcEntity { id: 1, name: "IFCREPRESENTATIONITEM".into(), args: Vec::new(), complex: vec![IfcComplexType { name: "IFCGEOMETRICREPRESENTATIONITEM".into(), args: Vec::new() }] }], ..Default::default() };
    let provider = Ifc4x3DetailsProvider::new(&snapshot);
    let component_path = [P::Key("entities".into()), P::Index(0), P::Key("complex".into()), P::Index(0)];
    assert_eq!(provider.variant_count(&component_path, Locale::En), ifc4x3_catalog().entities.len());
    let fixture: serde_json::Value = serde_json::from_str(include_str!("tests.json")).unwrap();
    for case in fixture["complexTypes"].as_array().unwrap() {
        let name = case["entity"].as_str().unwrap();
        let variant = (0..provider.variant_count(&component_path, Locale::En)).filter_map(|index| provider.variant(&component_path, Locale::En, index)).find(|variant| variant.label == name).unwrap();
        let event = editing::SnapshotEditEvent::SetValue { path: "/entities/0/complex/0".into(), value: variant.value };
        assert!(<Ifc4x3AnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_is_admitted(&event, &snapshot));
        let emit = <Ifc4x3AnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, &snapshot).unwrap();
        assert!(matches!(emit.artifact_mutations.as_slice(), [IfcMutation::SetEntityComplex(_)]));
        let outcome = protocol::ColdMutation::diff(&emit.artifact_mutations[0], &snapshot);
        let next = protocol::apply_diff_cold(outcome.diff(), &snapshot).unwrap();
        assert_eq!(next.entities[0].id, 1);
        assert_eq!(next.entities[0].complex[0].args.len(), case["attributes"].as_u64().unwrap() as usize);
        assert_eq!(protocol::apply_diff_cold(&outcome.diff().inverse(&snapshot), &next).unwrap(), snapshot);
        if name == "IFCROOT" {
            assert_eq!(next.entities[0].complex[0].args[0].as_str().unwrap().len(), 22);
        }
    }
    let rename = editing::SnapshotEditEvent::SetValue { path: "/entities/0/complex/0/name".into(), value: DslValue::String("IFCCARTESIANPOINT".into()) };
    let emit = <Ifc4x3AnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&rename, &snapshot).unwrap();
    let next = protocol::apply_diff_cold(protocol::ColdMutation::diff(&emit.artifact_mutations[0], &snapshot).diff(), &snapshot).unwrap();
    assert_eq!(next.entities[0].complex[0].args.len(), 1);
    let coordinate_path = [component_path.to_vec(), vec![P::Key("args".into()), P::Index(0)]].concat();
    assert!(Ifc4x3DetailsProvider::new(&next).presentation(&coordinate_path, Locale::En).unwrap().label.contains("Coordinates"));
    let primary = [P::Key("entities".into()), P::Index(0)];
    assert_eq!(provider.variant_count(&primary, Locale::En), ifc4x3_catalog().entities.len());
    assert!((0..provider.variant_count(&primary, Locale::En)).filter_map(|index| provider.variant(&primary, Locale::En, index)).any(|variant| variant.label == "IFCROOT"));
}

#[test]
fn complex_derived_redeclarations_remain_read_only_in_the_ancestor_component() {
    use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcComplexType;
    let snapshot = IfcSnapshot {
        entities: vec![IfcEntity {
            id: 1,
            name: "IFCNAMEDUNIT".into(),
            args: vec![IfcValue::Derived, IfcValue::Enum("LENGTHUNIT".into())],
            complex: vec![IfcComplexType { name: "IFCSIUNIT".into(), args: vec![IfcValue::Unset, IfcValue::Enum("METRE".into())] }],
        }],
        ..Default::default()
    };
    let provider = Ifc4x3DetailsProvider::new(&snapshot);
    assert_eq!(provider.child_count(&path(0, &[])), 0);
    assert!(provider.variants(&path(0, &[]), Locale::En).is_empty());
    assert!(provider.value(&path(0, &[P::Key("reference".into())])).is_none());
    assert!(!provider.item_capabilities(&path(0, &[P::Key("reference".into())])).edit);
}

#[test]
fn contextual_derived_transitions_pass_retained_admission_and_restore_exactly() {
    use protocol::command::ColdDiffAlgebra;
    use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcComplexType;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("derived.json")).unwrap();
    let part = |name: &str| IfcComplexType { name: name.into(), args: if name == "IFCSIUNIT" { vec![IfcValue::Unset, IfcValue::Enum("METRE".into())] } else { vec![IfcValue::String("Custom unit".into()), IfcValue::Unset] } };
    for case in fixture["cases"].as_array().unwrap() {
        let before = case["before"].as_str();
        let snapshot = IfcSnapshot {
            entities: vec![
                IfcEntity {
                    id: 1,
                    name: "IFCNAMEDUNIT".into(),
                    args: vec![if before == Some("IFCSIUNIT") { IfcValue::Derived } else { IfcValue::Reference(2) }, IfcValue::Enum("LENGTHUNIT".into())],
                    complex: before.map_or_else(Vec::new, |name| vec![part(name)]),
                },
                IfcEntity {
                    id: 2,
                    name: "IFCDIMENSIONALEXPONENTS".into(),
                    args: vec![IfcValue::Integer(1), IfcValue::Integer(0), IfcValue::Integer(0), IfcValue::Integer(0), IfcValue::Integer(0), IfcValue::Integer(0), IfcValue::Integer(0)],
                    complex: Vec::new(),
                },
            ],
            ..Default::default()
        };
        let provider = Ifc4x3DetailsProvider::new(&snapshot);
        let root = [P::Key("entities".into()), P::Index(0)];
        let component = [root.to_vec(), vec![P::Key("complex".into()), P::Index(0)]].concat();
        let event = match case["event"].as_str().unwrap() {
            "primary" => editing::SnapshotEditEvent::SetValue {
                path: "/entities/0".into(),
                value: (0..provider.variant_count(&root, Locale::En)).filter_map(|index| provider.variant(&root, Locale::En, index)).find(|variant| variant.label.eq_ignore_ascii_case("IFCNAMEDUNIT")).unwrap().value,
            },
            "variant" => editing::SnapshotEditEvent::SetValue {
                path: "/entities/0/complex/0".into(),
                value: (0..provider.variant_count(&component, Locale::En)).filter_map(|index| provider.variant(&component, Locale::En, index)).find(|variant| variant.label.eq_ignore_ascii_case(case["after"].as_str().unwrap())).unwrap().value,
            },
            "rename" => editing::SnapshotEditEvent::SetValue { path: "/entities/0/complex/0/name".into(), value: DslValue::String(case["after"].as_str().unwrap().into()) },
            "insert" => editing::SnapshotEditEvent::InsertValue { path: "/entities/0/complex/0".into(), value: part(case["after"].as_str().unwrap()).to_value() },
            "remove" => editing::SnapshotEditEvent::RemoveValue { path: "/entities/0/complex/0".into() },
            _ => unreachable!(),
        };
        assert!(<Ifc4x3AnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_is_admitted(&event, &snapshot), "{} admission", case["name"]);
        let emit = <Ifc4x3AnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, &snapshot).unwrap();
        let retained = prepare_retained_edit(&event, &snapshot);
        let capacity = <Ifc4x3AnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_work_capacity();
        assert!(capacity.work_items() >= retained.artifact_mutations.len() * 2);
        let mut next = snapshot.clone();
        let mut inverse = Vec::new();
        for mutation in emit.artifact_mutations {
            let outcome = protocol::ColdMutation::diff(&mutation, &next);
            inverse.push(outcome.diff().inverse(&next));
            next = protocol::apply_diff_cold(outcome.diff(), &next).unwrap();
        }
        let mut retained_next = snapshot.clone();
        for mutation in retained.artifact_mutations {
            retained_next = protocol::apply_diff_cold(protocol::ColdMutation::diff(&mutation, &retained_next).diff(), &retained_next).unwrap();
        }
        assert_eq!(retained_next, next, "{} retained preparation", case["name"]);
        let derived = case["derived"].as_bool().unwrap();
        assert_eq!(matches!(next.entities[0].args[0], IfcValue::Derived), derived, "{} marker", case["name"]);
        let next_provider = Ifc4x3DetailsProvider::new(&next);
        let reference = path(0, &[P::Key("reference".into())]);
        assert_eq!(next_provider.item_capabilities(&reference).edit, !derived, "{} editability", case["name"]);
        if !derived {
            assert_eq!(next.entities[0].args[0], IfcValue::Reference(2));
        }
        for inverse in inverse.into_iter().rev() {
            next = protocol::apply_diff_cold(&inverse, &next).unwrap();
        }
        assert_eq!(next, snapshot, "{} inverse", case["name"]);
    }
}

fn prepare_retained_edit(event: &editing::SnapshotEditEvent, snapshot: &IfcSnapshot) -> Emit<IfcMutation, NoConfigMutation, NoDraftMutation> {
    retained_edit_case(event, snapshot, None).0.unwrap()
}

fn reference_operation_wire(source: semio_framework_plugin::plugin_app_close_prelude::store::ArtifactPreparedOperationSource<'_>, cx: &mut semio_framework_job::StepContext<'_>) -> Vec<u8> {
    use semio_framework_plugin::plugin_app_close_prelude::store;
    let mut cursor = store::ArtifactPreparedOperationCursor::default();
    let mut result = Vec::new();
    let mut chunk = [0; 64];
    for _ in 0..2_000_000 {
        let mut grant = editing::fixture_mounted_owner_policy().preparation;
        let demand = cursor.canonical_json_work_demands().unwrap();
        grant.maximum_copy_bytes = demand.copy_bytes;
        grant.maximum_capacity_bytes = demand.capacity_bytes;
        grant.maximum_release_bytes = demand.release_bytes;
        grant.maximum_depth = demand.depth;
        let step = cx.with_retained_loan(grant, |cx| {
            let step = cursor.advance(source, &mut chunk, cx.retained_grant()).unwrap();
            cx.consume_retained(semio_framework_value::retained_clone::RetainedCloneProgress {
                copied_items: step.processed_items,
                copied_bytes: step.copied_bytes,
                retained_capacity_bytes: step.retained_capacity_bytes,
                released_bytes: step.released_bytes,
            })
            .unwrap();
            cx.consume_fuel(step.processed_items as u64);
            step
        });
        result.extend_from_slice(&chunk[..step.written_bytes]);
        if step.complete {
            let demand = cursor.retirement_demands().unwrap();
            let mut grant = editing::fixture_mounted_owner_policy().preparation;
            grant.maximum_copy_bytes = demand.copy_bytes;
            grant.maximum_capacity_bytes = demand.capacity_bytes;
            grant.maximum_release_bytes = demand.release_bytes;
            grant.maximum_depth = demand.depth;
            cx.with_retained_loan(grant, |cx| {
                let progress = cursor.close(cx.retained_grant()).unwrap();
                assert!(progress.complete);
                cx.consume_retained(semio_framework_value::retained_clone::RetainedCloneProgress {
                    copied_items: progress.processed_items,
                    copied_bytes: progress.copied_bytes,
                    retained_capacity_bytes: progress.retained_capacity_bytes,
                    released_bytes: progress.released_bytes,
                })
                .unwrap();
            });
            return result;
        }
    }
    panic!("reference operation wire did not finish")
}

fn retained_edit_case(event: &editing::SnapshotEditEvent, snapshot: &IfcSnapshot, cancel_at: Option<usize>) -> (Option<Emit<IfcMutation, NoConfigMutation, NoDraftMutation>>, usize) {
    let (emit, ticks, _) = retained_edit_control_case(event, snapshot, cancel_at, None, None);
    (emit, ticks)
}

fn retained_contiguous_owner_loan() -> semio_framework_value::retained_clone::RetainedCloneGrant {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../🔎️details/🧪️tests/source-owners.json")).unwrap();
    serde_json::from_value(fixture["contiguousOwnerLoan"].clone()).unwrap()
}

fn retained_edit_control_case(
    event: &editing::SnapshotEditEvent,
    snapshot: &IfcSnapshot,
    cancel_at: Option<usize>,
    wire_cancel: Option<usize>,
    expected_fault: Option<&str>,
) -> (Option<Emit<IfcMutation, NoConfigMutation, NoDraftMutation>>, usize, usize) {
    use semio_framework_plugin::retained_command::ArtifactCommandWork;
    use semio_framework_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandSnapshotRead, ArtifactCommandWorkStep};
    let snapshot = std::sync::Arc::new(snapshot.clone());
    let mut command = Ifc4x3EditCommand::EditSnapshot { event: event.clone() };
    let mut work = crate::details::IfcDetailsWork::<Ifc4x3AnyEditor, crate::details::Ifc4x3DetailsProfile>::new(event.action_id());
    if let Some(tick) = cancel_at {
        work.cancel_at_reduction_tick(tick);
    }
    if let Some(turn) = wire_cancel {
        work.cancel_at_publication_turn(turn);
    }
    fn maximum_text(value: &IfcValue) -> usize {
        match value {
            IfcValue::String(value) => value.len(),
            IfcValue::Aggregate(values) => values.iter().map(maximum_text).max().unwrap_or(0),
            IfcValue::TypedValue(value) => value.items.iter().map(maximum_text).max().unwrap_or(0),
            _ => 0,
        }
    }
    let retained_bytes = snapshot
        .entities
        .iter()
        .flat_map(|entity| entity.args.iter().chain(entity.complex.iter().flat_map(|part| &part.args)))
        .chain(snapshot.header.file_description.iter())
        .chain(snapshot.header.file_name.iter())
        .chain(snapshot.header.file_schema.iter())
        .map(maximum_text)
        .max()
        .unwrap_or(0);
    let mut caller = semio_framework_plugin::app_testing::artifact_app_laws::OriginalFixtureCaller::admit().unwrap();
    let mut decoder = semio_framework_job::OriginalJobDecodeOwner::empty();
    let mut cx = caller.context().with_original_decoder(&mut decoder);
    let grant = retained_contiguous_owner_loan();
    assert!(cx.with_retained_loan(grant, |cx| work.receive_original_command(&mut command, cx)).unwrap());
    assert_eq!(<Ifc4x3AnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_event(&command).unwrap().action_id(), event.action_id());
    let operation = semio_framework_plugin::AppOperationContext {
        app_instance_id: 1,
        parent_document_id: "ifc-retained-derived".into(),
        operation_id: cx.operation().0,
        generation: cx.generation().0,
        canonical_base_revision: [0; 32],
        retained: grant,
        authoring_seed: "ifc-retained-derived".into(),
    };
    let config = NoConfig::default();
    let history = semio_framework_plugin::HistoryView::empty();
    let interaction = protocol::InteractionState::default();
    let hover = semio_framework_plugin::app::InteractionHoverState::default();
    let input = ArtifactCommandInputs::<semio_framework_plugin::EditorApp<Ifc4x3AnyEditor>> {
        command: &command,
        snapshot: &snapshot,
        snapshot_owner: Some(&snapshot),
        snapshot_read: ArtifactCommandSnapshotRead::unavailable(&operation),
        config: &config,
        history: &history,
        interaction: &interaction,
        hover: &hover,
        context: None,
        operation: &operation,
    };
    let mut emit = None;
    for _ in 0..100_000 {
        match cx.with_retained_loan(grant, |cx| work.step(&input, cx)) {
            Ok(ArtifactCommandWorkStep::Complete(output)) => {
                emit = Some(output);
                break;
            }
            Err(error) => {
                if let Some(expected_fault) = expected_fault {
                    assert!(error.primary.message().contains(expected_fault), "unexpected retained failure: {error:?}");
                } else {
                    assert!(cancel_at.is_some() || wire_cancel.is_some(), "unexpected retained failure: {error:?}");
                    assert!(cx.is_cancelled());
                }
                if !work.staged_native_value() {
                    assert!(work.retained_text_bytes() >= retained_bytes);
                }
                break;
            }
            _ => {}
        }
    }
    assert_eq!(emit.is_none(), cancel_at.is_some() || wire_cancel.is_some() || expected_fault.is_some());
    if let Some(last) = emit.as_ref().and_then(|emit| emit.artifact_mutations.last()) {
        let inverse = protocol::ColdMutation::inverse(last, snapshot.as_ref()).unwrap();
        if let Some(inverse) = inverse.first() {
            use semio_framework_plugin::plugin_app_close_prelude::store;
            let expected = reference_operation_wire(store::ArtifactPreparedOperationSource::CanonicalJson { header: b"op", body: store::ArtifactCanonicalTreeSource::view(inverse) }, &mut cx);
            let inverse_view = work.borrowed_inverse_wire(&emit.as_ref().unwrap().artifact_mutations);
            let actual = reference_operation_wire(store::ArtifactPreparedOperationSource::CanonicalJson { header: b"op", body: &inverse_view }, &mut cx);
            assert_eq!(actual, expected);
        }
    }
    let ticks = work.reduction_ticks();
    let wire_turns = work.publication_turns();
    work.begin_close();
    for _ in 0..100_000 {
        let close_grant = semio_framework_value::retained_clone::RetainedCloneGrant {
            maximum_items: 16,
            maximum_copy_bytes: work.next_close_copy_byte_demand().unwrap(),
            maximum_capacity_bytes: work.next_close_capacity_byte_demand(4096).unwrap(),
            maximum_release_bytes: work.next_close_release_byte_demand().unwrap(),
            maximum_depth: work.next_close_depth_demand().unwrap(),
        };
        let _ = cx.with_retained_loan(close_grant, |cx| work.close_step(cx)).unwrap();
        if work.terminal_is_empty() {
            break;
        }
    }
    assert!(work.terminal_is_empty());
    let fuel = cx.fuel_remaining();
    drop(cx);
    let fuel = caller.retire_decoder(&mut decoder, fuel).unwrap();
    caller.retire(fuel).unwrap();
    (emit, ticks, wire_turns)
}

#[test]
fn same_primary_empty_global_id_matches_direct_retained_and_inverse() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("retained.json")).unwrap();
    let case = &fixture["identityRepair"];
    let mut args = vec![IfcValue::Unset; ifc_attribute_declarations(case["entity"].as_str().unwrap()).unwrap().len()];
    args[0] = IfcValue::String(case["globalId"].as_str().unwrap().into());
    let snapshot = IfcSnapshot { entities: vec![IfcEntity { id: 1, name: case["entity"].as_str().unwrap().into(), args, complex: Vec::new() }], ..Default::default() };
    let event = editing::SnapshotEditEvent::SetValue { path: "/entities/0".into(), value: DslValue::Object(vec![("$ifcType".into(), DslValue::String(case["entity"].as_str().unwrap().into()))]) };
    let direct = <Ifc4x3AnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, &snapshot).unwrap();
    let retained = prepare_retained_edit(&event, &snapshot);
    let apply = |mutations: &[IfcMutation]| mutations.iter().fold(snapshot.clone(), |base, mutation| protocol::apply_diff_cold(protocol::ColdMutation::diff(mutation, &base).diff(), &base).unwrap());
    let mut direct_next = apply(&direct.artifact_mutations);
    let retained_next = apply(&retained.artifact_mutations);
    assert_eq!(direct_next.entities[0].args[0].as_str().unwrap().len(), 22);
    assert!(matches!(retained.artifact_mutations.iter().find(|mutation| matches!(mutation, IfcMutation::SetEntityArg(value) if value.index == 0)), Some(IfcMutation::SetEntityArg(value)) if value.value.as_str().is_some_and(|text| text.len() == 22)));
    direct_next.entities[0].args[0] = retained_next.entities[0].args[0].clone();
    assert_eq!(direct_next, retained_next);
    assert_eq!(retained_next.entities[0].args[0].as_str().unwrap().len(), 22);
    let mut next = snapshot.clone();
    let mut inverses = Vec::new();
    for mutation in retained.artifact_mutations {
        inverses.splice(0..0, protocol::ColdMutation::inverse(&mutation, &next).unwrap());
        next = protocol::apply_diff_cold(protocol::ColdMutation::diff(&mutation, &next).diff(), &next).unwrap();
    }
    for inverse in inverses {
        next = protocol::apply_diff_cold(protocol::ColdMutation::diff(&inverse, &next).diff(), &next).unwrap();
    }
    assert_eq!(next, snapshot);
}

#[test]
fn actual_publication_wire_matches_neutral_cases_and_json_escaping() {
    use semio_framework_plugin::plugin_app_close_prelude::store;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../🔎️details/🧪️tests/wire.json")).unwrap();
    let snapshot = IfcSnapshot { entities: vec![IfcEntity { id: 1, name: "IFCPROPERTYSINGLEVALUE".into(), args: vec![IfcValue::String("old".into()), IfcValue::Unset, IfcValue::Unset, IfcValue::Unset], complex: Vec::new() }], ..Default::default() };
    let mut caller = semio_framework_plugin::app_testing::artifact_app_laws::OriginalFixtureCaller::admit().unwrap();
    let mut decoder = semio_framework_job::OriginalJobDecodeOwner::empty();
    let mut cx = caller.context().with_original_decoder(&mut decoder);
    for case in fixture["cases"].as_array().unwrap() {
        let parsed = semio_framework_pack_json::parse(&case["operation"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let mutation = IfcMutation::from_value(semio_framework_pack_json::to_dsl_value(&parsed)).unwrap();
        let event = match &mutation {
            IfcMutation::SetEntityArg(value) => editing::SnapshotEditEvent::SetValue { path: "/entities/0/args/0".into(), value: value.value.to_value() },
            IfcMutation::InsertEntityArg(value) => editing::SnapshotEditEvent::InsertValue { path: "/entities/0/args/0".into(), value: value.value.to_value() },
            IfcMutation::RemoveEntityArg(_) => editing::SnapshotEditEvent::RemoveValue { path: "/entities/0/args/0".into() },
            _ => panic!("wire case requires an argument mutation"),
        };
        let emit = prepare_retained_edit(&event, &snapshot);
        assert_eq!(emit.artifact_mutations, vec![mutation.clone()]);
        let encoded = reference_operation_wire(store::ArtifactPreparedOperationSource::CanonicalJson { header: b"op", body: store::ArtifactCanonicalTreeSource::view(&mutation) }, &mut cx);
        assert_eq!(&encoded[..2], b"op");
        assert_eq!(serde_json::from_slice::<serde_json::Value>(&encoded[2..]).unwrap(), case["operation"]);
        let inverse = protocol::ColdMutation::inverse(&mutation, &snapshot).unwrap();
        let encoded = reference_operation_wire(store::ArtifactPreparedOperationSource::CanonicalJson { header: b"op", body: store::ArtifactCanonicalTreeSource::view(&inverse[0]) }, &mut cx);
        assert_eq!(serde_json::from_slice::<serde_json::Value>(&encoded[2..]).unwrap(), case["inverse"]);
    }
    for text in fixture["textRuns"].as_array().unwrap() {
        let text = text.as_str().unwrap().to_owned();
        let encoded = reference_operation_wire(store::ArtifactPreparedOperationSource::CanonicalJson { header: b"op", body: store::ArtifactCanonicalTreeSource::view(&text) }, &mut cx);
        assert_eq!(&encoded[2..], serde_json::to_vec(&text).unwrap());
    }
    let fuel = cx.fuel_remaining();
    drop(cx);
    let fuel = caller.retire_decoder(&mut decoder, fuel).unwrap();
    caller.retire(fuel).unwrap();
}

#[test]
fn every_publication_turn_cancels_with_borrowed_inverse_and_owned_close() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../🔎️details/🧪️tests/wire.json")).unwrap();
    let case = &fixture["cancellation"];
    let snapshot = IfcSnapshot {
        entities: vec![IfcEntity {
            id: 1,
            name: "IFCPROPERTYSINGLEVALUE".into(),
            args: vec![IfcValue::String("Name".into()), IfcValue::String("x".repeat(case["textBytes"].as_u64().unwrap() as usize)), IfcValue::Unset, IfcValue::Unset],
            complex: Vec::new(),
        }],
        ..Default::default()
    };
    let event = editing::SnapshotEditEvent::SetValue { path: case["path"].as_str().unwrap().into(), value: DslValue::String("changed".into()) };
    let (emit, _, turns) = retained_edit_control_case(&event, &snapshot, None, None, None);
    assert_eq!(emit.unwrap().artifact_mutations.len(), 1);
    assert!(turns > 64);
    for turn in 1..=turns {
        assert!(retained_edit_control_case(&event, &snapshot, None, Some(turn), None).0.is_none());
    }
    assert_eq!(snapshot.entities[0].args[1].as_str().unwrap().len(), case["textBytes"].as_u64().unwrap() as usize);
}

#[test]
fn header_tuple_uses_retained_native_preparation_and_borrowed_inverse() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../🔎️details/🧪️tests/wire.json")).unwrap();
    let case = &fixture["headerTuple"];
    let comment = "x".repeat(case["commentBytes"].as_u64().unwrap() as usize);
    let mut snapshot = IfcSnapshot::default();
    snapshot.header.file_description = vec![IfcValue::Aggregate(vec![IfcValue::String(case["before"].as_str().unwrap().into()), IfcValue::String(comment.clone())]), IfcValue::String("2;1".into())];
    let event = editing::SnapshotEditEvent::SetValue { path: case["path"].as_str().unwrap().into(), value: IfcValue::String(case["after"].as_str().unwrap().into()).to_value() };
    let (emit, ticks) = retained_edit_case(&event, &snapshot, None);
    assert!(ticks > 0);
    let [IfcMutation::SetFileDescription(mutation)] = emit.as_ref().unwrap().artifact_mutations.as_slice() else { panic!("header requires its existing primitive mutation") };
    let IfcValue::Aggregate(values) = &mutation.values[0] else { panic!("header description tuple remains aggregate") };
    assert_eq!(values[0].as_str(), case["after"].as_str());
    assert_eq!(values[1].as_str(), Some(comment.as_str()));
    for tick in 1..=ticks {
        assert!(retained_edit_case(&event, &snapshot, Some(tick)).0.is_none());
    }
    assert_eq!(snapshot.header.file_description[0].as_aggregate().unwrap()[0].as_str(), case["before"].as_str());
}

#[test]
fn oversized_exact_inverse_refuses_before_publication_without_inverse_copy() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../🔎️details/🧪️tests/wire.json")).unwrap();
    let bytes = fixture["largeInverse"]["textBytes"].as_u64().unwrap() as usize;
    let snapshot = IfcSnapshot {
        entities: vec![IfcEntity { id: 1, name: "IFCPROPERTYSINGLEVALUE".into(), args: vec![IfcValue::String("Name".into()), IfcValue::String("x".repeat(bytes)), IfcValue::Unset, IfcValue::Unset], complex: Vec::new() }],
        ..Default::default()
    };
    let event = editing::SnapshotEditEvent::SetValue { path: "/entities/0/args/1/value".into(), value: DslValue::String("changed".into()) };
    assert!(retained_edit_control_case(&event, &snapshot, None, None, Some("native publication item limit")).0.is_none());
    assert_eq!(snapshot.entities[0].args[1].as_str().unwrap().len(), bytes);
}

#[test]
fn every_reduction_cancellation_prefix_keeps_large_native_owners_for_close() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("retained.json")).unwrap();
    let case = &fixture["reductionCustody"];
    let mut args = vec![IfcValue::Unset; ifc_attribute_declarations(case["from"].as_str().unwrap()).unwrap().len()];
    args[0] = IfcValue::String("0123456789012345678901".into());
    args[2] = IfcValue::String("Wall".into());
    args[3] = IfcValue::String("x".repeat(case["textBytes"].as_u64().unwrap() as usize));
    let snapshot = IfcSnapshot { entities: vec![IfcEntity { id: 1, name: case["from"].as_str().unwrap().into(), args, complex: Vec::new() }], ..Default::default() };
    let event = editing::SnapshotEditEvent::SetValue { path: "/entities/0".into(), value: DslValue::Object(vec![("$ifcType".into(), DslValue::String(case["to"].as_str().unwrap().into()))]) };
    let (emit, ticks) = retained_edit_case(&event, &snapshot, None);
    assert!(emit.unwrap().artifact_mutations.iter().any(|mutation| matches!(mutation, IfcMutation::SetEntityArg(value) if value.index == 2 && value.value.as_str().is_some_and(|text| text.len() == case["textBytes"].as_u64().unwrap() as usize))));
    assert!(ticks > 2);
    for prefix in 1..=ticks {
        assert!(retained_edit_case(&event, &snapshot, Some(prefix)).0.is_none());
    }
    assert_eq!(snapshot.entities[0].args[3].as_str().unwrap().len(), case["textBytes"].as_u64().unwrap() as usize);
}

#[test]
fn contextual_derivation_considers_every_retained_component() {
    use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcComplexType;
    let mut complex = vec![IfcComplexType { name: "IFCREPRESENTATIONITEM".into(), args: Vec::new() }; 80];
    complex.push(IfcComplexType { name: "IFCSIUNIT".into(), args: vec![IfcValue::Unset, IfcValue::Enum("METRE".into())] });
    let snapshot = IfcSnapshot { entities: vec![IfcEntity { id: 1, name: "IFCNAMEDUNIT".into(), args: vec![IfcValue::Derived, IfcValue::Enum("LENGTHUNIT".into())], complex }], ..Default::default() };
    let provider = Ifc4x3DetailsProvider::new(&snapshot);
    assert!(!provider.item_capabilities(&path(0, &[P::Key("reference".into())])).edit);
}

#[test]
fn type_rows_keep_constant_payload_size_with_large_existing_values() {
    let snapshot = IfcSnapshot {
        entities: vec![IfcEntity {
            id: 1,
            name: "IFCPROPERTYSINGLEVALUE".into(),
            args: vec![
                IfcValue::String("Large".into()),
                IfcValue::Unset,
                IfcValue::TypedValue(semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcTypedValue { name: "IFCTEXT".into(), items: vec![IfcValue::String("x".repeat(1_000_000))] }),
                IfcValue::Unset,
            ],
            complex: Vec::new(),
        }],
        ..Default::default()
    };
    let provider = Ifc4x3DetailsProvider::new(&snapshot);
    let root = [P::Key("entities".into()), P::Index(0)];
    for index in 0..provider.variant_count(&root, Locale::En) {
        let row = provider.variant(&root, Locale::En, index).unwrap();
        let event = editing::SnapshotEditEvent::SetValue { path: "/entities/0".into(), value: row.value };
        assert!(protocol::OpBinary::encode_op(&event).unwrap().len() < 256);
    }
}

#[test]
fn large_existing_argument_preparation_yields_and_cancels_before_emission() {
    use semio_framework_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandSnapshotRead, ArtifactCommandWorkStep};
    let snapshot = std::sync::Arc::new(IfcSnapshot {
        entities: vec![IfcEntity {
            id: 1,
            name: "IFCPROPERTYLISTVALUE".into(),
            args: vec![
                IfcValue::String("Large".into()),
                IfcValue::Unset,
                IfcValue::Aggregate(vec![IfcValue::TypedValue(semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcTypedValue { name: "IFCTEXT".into(), items: vec![IfcValue::String("x".repeat(256))] }); 2_000]),
                IfcValue::Unset,
            ],
            complex: Vec::new(),
        }],
        ..Default::default()
    });
    let event = editing::SnapshotEditEvent::SetValue { path: "/entities/0/args/2/value/1999/value/items/0/value".into(), value: DslValue::String("changed".into()) };
    assert!(<Ifc4x3AnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_is_admitted(&event, &snapshot));
    let command = Ifc4x3EditCommand::EditSnapshot { event };
    let mut work = <Ifc4x3AnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_work(editing::SET_SNAPSHOT_VALUE_ACTION_ID);
    let mut caller = semio_framework_plugin::app_testing::artifact_app_laws::OriginalFixtureCaller::admit().unwrap();
    let mut decoder = semio_framework_job::OriginalJobDecodeOwner::empty();
    let mut cx = caller.context().with_original_decoder(&mut decoder);
    let grant = editing::fixture_mounted_owner_policy().preparation;
    let operation = semio_framework_plugin::AppOperationContext {
        app_instance_id: 1,
        parent_document_id: "ifc-retained-argument".into(),
        operation_id: cx.operation().0,
        generation: cx.generation().0,
        canonical_base_revision: [0; 32],
        retained: grant,
        authoring_seed: "ifc-retained-argument".into(),
    };
    let config = NoConfig::default();
    let history = semio_framework_plugin::HistoryView::empty();
    let interaction = protocol::InteractionState::default();
    let hover = semio_framework_plugin::app::InteractionHoverState::default();
    let input = ArtifactCommandInputs::<semio_framework_plugin::EditorApp<Ifc4x3AnyEditor>> {
        command: &command,
        snapshot: &snapshot,
        snapshot_owner: Some(&snapshot),
        snapshot_read: ArtifactCommandSnapshotRead::unavailable(&operation),
        config: &config,
        history: &history,
        interaction: &interaction,
        hover: &hover,
        context: None,
        operation: &operation,
    };
    assert!(work.extent(&command, &snapshot, &interaction, None).is_some());
    for _ in 0..8 {
        let step = cx.with_retained_loan(grant, |cx| work.step(&input, cx)).unwrap();
        assert!(!matches!(step, ArtifactCommandWorkStep::Complete(_)));
    }
    cx.original_cancel_token().cancel_now();
    assert!(cx.with_retained_loan(grant, |cx| work.step(&input, cx)).is_err());
    assert_eq!(snapshot.entities[0].args[2].as_aggregate().unwrap()[1999].as_typed().unwrap().1[0].as_str(), Some("x".repeat(256).as_str()));
    work.begin_close();
    for _ in 0..100_000 {
        let _ = cx.with_retained_loan(grant, |cx| work.close_step(cx)).unwrap();
        if work.terminal_is_empty() {
            break;
        }
    }
    assert!(work.terminal_is_empty());
    let fuel = cx.fuel_remaining();
    drop(cx);
    let fuel = caller.retire_decoder(&mut decoder, fuel).unwrap();
    caller.retire(fuel).unwrap();
}

#[test]
fn official_4x3_tessellation_projects_real_geometry_and_cancellation_has_no_visible_mesh() {
    use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::geometry;
    let snapshot = crate::io::parse_ifc4x3_step(include_str!("../🚪️io/🧪️tests/triangulated.ifc")).unwrap();
    let scene = geometry::project(&snapshot);
    assert_eq!(scene.status, geometry::GeometryStatus::Complete);
    assert_eq!(scene.meshes.len(), 1);
    assert_eq!(scene.meshes[0].vertices.len(), 8);
    assert_eq!(scene.meshes[0].triangles.len(), 12);
    assert!(scene.issues.is_empty());
    let cancelled = semio_s_artifact_stdio_ifc_4::standards::v4::subsets::any::io::geometry::render_with(&snapshot, geometry::GeometryLimits::default(), &mut |_| false).unwrap();
    let independent = serde_json::to_string(&cancelled).unwrap();
    assert!(independent.contains("cancelled"));
    assert!(!independent.contains("positions"));
}

#[test]
fn domain_insertion_uses_retained_forward_inverse_admission_and_original_cancel() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../🔎️details/🧪️tests/wire.json")).unwrap();
    let case = &fixture["domainInsertion"];
    let parsed = semio_framework_pack_json::parse(&case["entity"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let event = editing::SnapshotEditEvent::InsertValue { path: case["path"].as_str().unwrap().into(), value: semio_framework_pack_json::to_dsl_value(&parsed) };
    let snapshot = IfcSnapshot::default();
    let (emit, _, turns) = retained_edit_control_case(&event, &snapshot, None, None, None);
    let emit = emit.unwrap();
    assert!(matches!(emit.artifact_mutations.as_slice(), [IfcMutation::InsertEntity(value)] if value.entity.name == "IFCORGANIZATION" && value.entity.id == 1));
    let mutation = &emit.artifact_mutations[0];
    let next = protocol::apply_diff_cold(protocol::ColdMutation::diff(mutation, &snapshot).diff(), &snapshot).unwrap();
    assert_eq!(next.entities.len(), 1);
    let inverse = protocol::ColdMutation::inverse(mutation, &snapshot).unwrap();
    assert_eq!(inverse[0].to_value(), semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(&case["inverse"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()));
    assert_eq!(protocol::apply_diff_cold(protocol::ColdMutation::diff(&inverse[0], &next).diff(), &next).unwrap(), snapshot);
    assert!(turns > 1);
    for turn in 1..=turns {
        assert!(retained_edit_control_case(&event, &snapshot, None, Some(turn), None).0.is_none());
    }
    let parsed = semio_framework_pack_json::parse(&case["entity"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let append = editing::SnapshotEditEvent::InsertValue { path: case["appendPath"].as_str().unwrap().into(), value: semio_framework_pack_json::to_dsl_value(&parsed) };
    assert!(matches!(prepare_retained_edit(&append,&snapshot).artifact_mutations.as_slice(),[IfcMutation::InsertEntity(value)] if value.index==0&&value.entity.id==1));
    for arity in [false, true] {
        let mut invalid = case["entity"].clone();
        if arity {
            invalid["args"].as_array_mut().unwrap().truncate(case["invalidArgumentCount"].as_u64().unwrap() as usize);
        } else {
            invalid["args"][1] = case["invalidName"].clone();
        }
        let parsed = semio_framework_pack_json::parse(&invalid.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let event = editing::SnapshotEditEvent::InsertValue { path: case["path"].as_str().unwrap().into(), value: semio_framework_pack_json::to_dsl_value(&parsed) };
        assert!(<Ifc4x3AnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, &snapshot).unwrap_err().message().contains("ifc-domain-"));
        assert!(retained_edit_control_case(&event, &snapshot, None, None, Some("ifc-domain-")).0.is_none());
    }
}

#[test]
fn constituent_domains_validate_prepared_context_without_rejecting_structural_repairs() {
    use protocol::command::ColdDiffAlgebra;
    use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcComplexType;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../🔎️details/🧪️tests/domain-structure.json")).unwrap();
    let native = |value: &serde_json::Value| semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(&value.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap());
    for case in fixture["cases"].as_array().unwrap() {
        let complex: Vec<_> = case["before"].as_array().unwrap().iter().map(|part| IfcComplexType::from_value(native(part)).unwrap()).collect();
        let snapshot = IfcSnapshot {
            entities: vec![
                IfcEntity { id: 1, name: "IFCNAMEDUNIT".into(), args: vec![if complex.iter().any(|part| part.name == "IFCSIUNIT") { IfcValue::Derived } else { IfcValue::Reference(2) }, IfcValue::Enum("LENGTHUNIT".into())], complex },
                IfcEntity { id: 2, name: "IFCDIMENSIONALEXPONENTS".into(), args: vec![IfcValue::Integer(0); 7], complex: Vec::new() },
            ],
            ..Default::default()
        };
        let path = format!("/entities/0/complex/{}", case["index"].as_u64().unwrap());
        let event = match case["operation"].as_str().unwrap() {
            "insert" => editing::SnapshotEditEvent::InsertValue { path, value: native(&case["part"]) },
            "remove" => editing::SnapshotEditEvent::RemoveValue { path },
            "move" => editing::SnapshotEditEvent::MoveValue { from: format!("/entities/0/complex/{}", case["from"].as_u64().unwrap()), path },
            _ => unreachable!(),
        };
        let direct = <Ifc4x3AnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, &snapshot);
        if !case["admitted"].as_bool().unwrap() {
            assert!(direct.unwrap_err().message().contains("ifc-domain-"), "{}", case["name"]);
            assert!(retained_edit_control_case(&event, &snapshot, None, None, Some("ifc-domain-")).0.is_none(), "{}", case["name"]);
            continue;
        }
        let direct = direct.unwrap();
        let (retained, _, turns) = retained_edit_control_case(&event, &snapshot, None, None, None);
        let retained = retained.unwrap();
        let mut direct_next = snapshot.clone();
        for mutation in direct.artifact_mutations {
            direct_next = protocol::apply_diff_cold(protocol::ColdMutation::diff(&mutation, &direct_next).diff(), &direct_next).unwrap();
        }
        let mut next = snapshot.clone();
        let mut inverse = Vec::new();
        for mutation in retained.artifact_mutations {
            let outcome = protocol::ColdMutation::diff(&mutation, &next);
            inverse.push(outcome.diff().inverse(&next));
            next = protocol::apply_diff_cold(outcome.diff(), &next).unwrap();
        }
        assert_eq!(next, direct_next, "{} retained", case["name"]);
        assert_eq!(matches!(next.entities[0].args[0], IfcValue::Derived), case["derived"].as_bool().unwrap(), "{} context", case["name"]);
        for inverse in inverse.into_iter().rev() {
            next = protocol::apply_diff_cold(&inverse, &next).unwrap();
        }
        assert_eq!(next, snapshot, "{} undo", case["name"]);
        for turn in 1..=turns {
            assert!(retained_edit_control_case(&event, &snapshot, None, Some(turn), None).0.is_none(), "{} cancellation {turn}", case["name"]);
        }
    }
}
