
use super::*;
use semio_s_artifact_stdio_step::engine::part21::{Part21Document, Part21Header};

fn snapshot() -> Ifc2x3Snapshot {
    let header = Part21Header {
        file_description: vec![Part21Value::List(vec![Part21Value::Str("ViewDefinition [CoordinationView_V2.0]".into())]), Part21Value::Str("2;1".into())],
        file_name: vec![],
        file_schema: vec![Part21Value::List(vec![Part21Value::Str("IFC2X3".into())])],
    };
    let project = simple_instance(
        1,
        "IFCPROJECT",
        vec![Part21Value::Str("guid".into()), Part21Value::Unset, Part21Value::Str("Project".into()), Part21Value::Unset, Part21Value::Unset, Part21Value::Unset, Part21Value::Unset, Part21Value::Unset, Part21Value::Ref(2)],
    );
    let units = simple_instance(2, "IFCUNITASSIGNMENT", vec![]);
    Ifc2x3Snapshot { schema: "stdio.ifc.2x3".into(), document: Part21Document { header, instances: vec![project, units] }, edm_preamble: None }
}

#[test]
fn view_definition_reads_and_restamps_the_header() {
    let mut snap = snapshot();
    assert_eq!(view_definition_name(&snap).as_deref(), Some("CoordinationView_V2.0"));
    set_view_definition(&mut snap, "FMHandOverView");
    assert_eq!(view_definition(&snap), Some("ViewDefinition [FMHandOverView]"));
    assert_eq!(view_definition_name(&snap).as_deref(), Some("FMHandOverView"));
}

#[test]
fn set_argument_guards_the_expected_type() {
    let mut snap = snapshot();
    assert_eq!(reference_argument(&snap, 1, 8), Some(2));
    set_argument(&mut snap, 1, &["IFCPROJECT"], 8, Part21Value::Unset).expect("the project accepts the edit");
    assert_eq!(reference_argument(&snap, 1, 8), None);
    assert!(set_argument(&mut snap, 2, &["IFCPROJECT"], 8, Part21Value::Unset).is_err(), "an IFCUNITASSIGNMENT is not an IFCPROJECT");
    assert!(set_argument(&mut snap, 99, &[], 0, Part21Value::Unset).is_err(), "an absent id is an error, never a silent no-op");
}

#[test]
fn set_argument_pads_a_short_record() {
    let mut snap = snapshot();
    set_argument(&mut snap, 2, &["IFCUNITASSIGNMENT"], 3, Part21Value::Str("padded".into())).expect("padding");
    assert_eq!(argument(&snap, 2, 0), Some(&Part21Value::Unset));
    assert_eq!(argument(&snap, 2, 3), Some(&Part21Value::Str("padded".into())));
}

#[test]
fn upsert_replaces_and_remove_guards() {
    let mut snap = snapshot();
    upsert_instance(&mut snap, simple_instance(3, "IFCSPACE", vec![Part21Value::Str("guid".into())]));
    assert_eq!(instance_type(&snap, 3), Some("IFCSPACE"));
    upsert_instance(&mut snap, simple_instance(3, "IFCSPACE", vec![Part21Value::Str("other".into())]));
    assert_eq!(snap.document.instances.len(), 3, "upsert replaces an existing id rather than appending a duplicate");
    assert!(remove_instance(&mut snap, 3, &["IFCPROJECT"]).is_err(), "the type guard refuses an unrelated concept");
    remove_instance(&mut snap, 3, &["IFCSPACE"]).expect("removing the space");
    assert!(remove_instance(&mut snap, 3, &["IFCSPACE"]).is_err(), "removing an absent id is an error");
}

#[test]
fn reference_lists_round_trip() {
    let value = reference_list(&[7, 9]);
    assert_eq!(reference_list_ids(Some(&value)), vec![7, 9]);
    assert_eq!(reference_list_ids(None), Vec::<u64>::new());
    assert_eq!(optional(None), Part21Value::Unset);
}

#[test]
fn canonical_compares_documents_as_exchange_structures() {
    let start = snapshot();
    let mut reordered = start.clone();
    reordered.document.instances.reverse();
    assert_ne!(reordered, start, "Part21Document equality is line-order sensitive");
    assert_eq!(canonical(&reordered), canonical(&start), "the exchange structure is unchanged");
}

#[test]
fn ids_of_types_finds_the_concept_population() {
    let snap = snapshot();
    assert_eq!(ids_of_types(&snap, &["IFCPROJECT"]), vec![1]);
    assert_eq!(ids_of_types(&snap, &["IFCSPACE"]), Vec::<u64>::new());
}
