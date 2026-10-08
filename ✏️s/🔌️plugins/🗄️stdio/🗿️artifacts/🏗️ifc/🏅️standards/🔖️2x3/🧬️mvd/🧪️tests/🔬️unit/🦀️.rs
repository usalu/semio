use super::*;
use protocol::os_spr::command::DiffAlgebra;
use semio_s_artifact_stdio_contract::part21::{Part21Document, Part21Header};

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

fn applied(base: &Ifc2x3Snapshot, diff: &Ifc2x3Diff) -> Ifc2x3Snapshot {
    protocol::apply_diff(diff, base).expect("the diff applies")
}

#[test]
fn view_definition_reads_and_restamps_the_header() {
    let snap = snapshot();
    assert_eq!(view_definition_name(&snap).as_deref(), Some("CoordinationView_V2.0"));
    let next = applied(&snap, &view_definition_diff(&snap, "FMHandOverView"));
    assert_eq!(view_definition(&next), Some("ViewDefinition [FMHandOverView]"));
    assert_eq!(view_definition_name(&next).as_deref(), Some("FMHandOverView"));
    assert!(view_definition_diff(&next, "FMHandOverView").is_empty(), "restamping the same view is the empty diff");
}

#[test]
fn argument_diff_guards_the_expected_type() {
    let snap = snapshot();
    assert_eq!(reference_argument(&snap, 1, 8), Some(2));
    let next = applied(&snap, &argument_diff(&snap, 1, &["IFCPROJECT"], 8, Part21Value::Unset).expect("the project accepts the edit"));
    assert_eq!(reference_argument(&next, 1, 8), None);
    assert!(argument_diff(&snap, 2, &["IFCPROJECT"], 8, Part21Value::Unset).is_err(), "an IFCUNITASSIGNMENT is not an IFCPROJECT");
    assert!(argument_diff(&snap, 99, &[], 0, Part21Value::Unset).is_err(), "an absent id is an error, never a silent no-op");
}

#[test]
fn argument_diff_pads_a_short_record() {
    let snap = snapshot();
    let next = applied(&snap, &argument_diff(&snap, 2, &["IFCUNITASSIGNMENT"], 3, Part21Value::Str("padded".into())).expect("padding"));
    assert_eq!(argument(&next, 2, 0), Some(&Part21Value::Unset));
    assert_eq!(argument(&next, 2, 3), Some(&Part21Value::Str("padded".into())));
}

#[test]
fn upsert_replaces_and_remove_guards() {
    let snap = snapshot();
    let inserted = applied(&snap, &upsert_diff(&snap, simple_instance(3, "IFCSPACE", vec![Part21Value::Str("guid".into())]), None));
    assert_eq!(instance_type(&inserted, 3), Some("IFCSPACE"));
    let replaced = applied(&inserted, &upsert_diff(&inserted, simple_instance(3, "IFCSPACE", vec![Part21Value::Str("other".into())]), None));
    assert_eq!(replaced.document.instances.len(), 3, "upsert replaces an existing id rather than appending a duplicate");
    assert!(remove_diff(&replaced, 3, &["IFCPROJECT"]).is_err(), "the type guard refuses an unrelated concept");
    let removed = applied(&replaced, &remove_diff(&replaced, 3, &["IFCSPACE"]).expect("removing the space"));
    assert!(remove_diff(&removed, 3, &["IFCSPACE"]).is_err(), "removing an absent id is an error");
}

#[test]
fn upsert_at_an_index_inserts_at_that_position() {
    let snap = snapshot();
    let inserted = applied(&snap, &upsert_diff(&snap, simple_instance(3, "IFCSPACE", vec![]), Some(0)));
    assert_eq!(position(&inserted, 3), Some(0));
    assert_eq!(standing(&inserted, 3, &["IFCSPACE"]), Standing::Present { index: 0 });
    assert_eq!(standing(&inserted, 1, &["IFCSPACE"]), Standing::Foreign);
    assert_eq!(standing(&snap, 3, &["IFCSPACE"]), Standing::Absent);
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
