use super::*;

#[semio_framework_async_macros::async_test]
async fn create_editor_builds_a_definition_for_the_editor_role() {
    let def = create_ifc2x3_sav_editor();
    assert_eq!(def.role, semio_plugin::AppRole::Editor);
    assert_eq!(def.dialect, IFC2X3_SAV_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<Ifc2x3SavEditor as ArtifactEditor>::DIALECT, IFC2X3_SAV_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn editor_and_viewer_share_one_dialect() {
    semio_plugin::artifact_app_laws::assert_editor_and_viewer_share_dialect::<Ifc2x3SavEditor, crate::viewer::ifc2x3_sav::Ifc2x3SavViewer>().await;
}

semio_plugin::history_edit_acceptance_law!("stdio", super::Ifc2x3SavEditor, || semio_plugin::App { definition: super::create_ifc2x3_sav_editor(), examples: Vec::new() }, "../../🏅️standards/🔖️2x3/🪆️subsets/🧮️sav");

#[semio_framework_async_macros::async_test]
async fn stable_selected_instance_uses_own_typed_mutation_route(){use semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor;use semio_s_artifact_stdio_contract::part21::{Part21Instance,Part21Value};let mut snapshot=Ifc2x3Snapshot::default();snapshot.document.instances.push(Part21Instance{id:1,entities:vec![("IFCWALL".into(),vec![Part21Value::Unset;8])]});let event=editing::SnapshotEditEvent::SetValue{path:"/ifc-1/entities/0/arguments/2".into(),value:semio_framework_value::ToValue::to_value(&Part21Value::Str("Selected wall".into()))};assert!(Ifc2x3SavEditor::snapshot_edit_is_admitted(&event,&snapshot));let emit=Ifc2x3SavEditor::snapshot_edit_emit(&event,&snapshot).unwrap();assert!(matches!(&emit.artifact_mutations[0],Ifc2x3Mutation::UpsertInstance(change)if change.instance.id==1&&change.instance.entities[0].1[2]==Part21Value::Str("Selected wall".into())));let definition=create_ifc2x3_sav_editor();assert!(definition.interactions.iter().any(|domain|domain.id==inspector::DOMAIN));}

#[test]
fn natural_file_round_trips_the_real_export_under_the_subset_qualified_kind() {
    const REAL: &[u8] = include_bytes!("../../../../🧱️base/🧪️tests/🧫️fixtures/🏥️wellness-center-sama-street-level/🏥️wellness-center-sama-street-level.ifc");
    let codec = <Ifc2x3SavEditor as ArtifactEditor>::natural_file_codec().expect("the subset declares its natural file");
    assert_eq!((codec.format_kind, codec.extension), ("s.stdio.ifc@2x3/sav", ".ifc"));
    let opened = <Ifc2x3SavEditor as ArtifactEditor>::decode_natural_file(REAL).expect("the real export opens");
    let saved = <Ifc2x3SavEditor as ArtifactEditor>::encode_natural_file(&opened).expect("save");
    assert_eq!(<Ifc2x3SavEditor as ArtifactEditor>::decode_natural_file(&saved).expect("reopen"), opened);
    assert!(<Ifc2x3SavEditor as ArtifactEditor>::decode_natural_file(b"not a step file").is_err());
}
