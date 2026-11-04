use super::*;

#[semio_framework_async_macros::async_test]
async fn create_editor_builds_a_definition_for_the_editor_role() {
    let def = create_ifc2x3_any_editor();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
    assert_eq!(def.dialect, IFC2X3_ANY_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<Ifc2x3AnyEditor as ArtifactEditor>::DIALECT, IFC2X3_ANY_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn editor_and_viewer_share_one_dialect() {
    semio_framework_plugin::artifact_app_laws::assert_editor_and_viewer_share_dialect::<Ifc2x3AnyEditor, crate::viewer::ifc2x3_any::Ifc2x3AnyViewer>().await;
}

semio_framework_plugin::history_edit_acceptance_law!("stdio", super::Ifc2x3AnyEditor, || semio_framework_plugin::App { definition: super::create_ifc2x3_any_editor(), examples: Vec::new() }, "../../🏅️standards/🔖️2x3/🪆️subsets/🧱️base");

const NATURAL_REVIT_IFC: &[u8] = include_bytes!("../../../🧪️tests/🧫️fixtures/🏥️wellness-center-sama-street-level/🏥️wellness-center-sama-street-level.ifc");

#[test]
fn natural_file_attribute_edit_keeps_every_id_reference_and_unedited_instance() {
    use semio_s_artifact_stdio_contract::part21::Part21Value;
    let opened = <Ifc2x3AnyEditor as ArtifactEditor>::decode_natural_file(NATURAL_REVIT_IFC).expect("a real Revit IFC2X3 export opens");
    let source_ids: Vec<u64> = std::str::from_utf8(NATURAL_REVIT_IFC).expect("IFC is text").lines().filter_map(|line| line.strip_prefix('#')?.split('=').next()?.trim().parse().ok()).collect();
    assert_eq!(opened.document.instances.iter().map(|instance| instance.id).collect::<Vec<_>>(), source_ids);
    let mut project = opened.document.instances.iter().find(|instance| instance.entity("IFCPROJECT").is_some()).expect("the export declares a project").clone();
    let owner_history = project.entity("IFCPROJECT").expect("project attributes")[1].clone();
    project.entities[0].1[2] = Part21Value::Str("Renamed Project".into());
    let mut edited = opened.clone();
    semio_s_artifact_stdio_contract::apply_mutation(&mut edited, &Ifc2x3Mutation::UpsertInstance(semio_s_artifact_stdio_ifc_2x3_base_mutations::standards::v2x3::subsets::base::schema::mutations::upsert_instance::UpsertInstance { instance: project.clone(), index: None }));
    let saved = <Ifc2x3AnyEditor as ArtifactEditor>::encode_natural_file(&edited).expect("save");
    let reopened = <Ifc2x3AnyEditor as ArtifactEditor>::decode_natural_file(&saved).expect("reopen");
    assert_eq!(reopened.document.instances.iter().map(|instance| instance.id).collect::<Vec<_>>(), source_ids, "no #id was renumbered, dropped or added");
    assert_eq!(reopened.document.instance(project.id).expect("project").entity("IFCPROJECT").expect("attributes")[1], owner_history, "the reference attribute is untouched");
    for (before, after) in opened.document.instances.iter().zip(&reopened.document.instances) {
        if before.id != project.id {
            assert_eq!(before, after, "instance #{} was not edited", before.id);
        }
    }
    assert!(std::str::from_utf8(&saved).expect("text").contains("'Renamed Project'"));
}

#[test]
fn natural_file_refuses_an_ifc4_schema_declaration() {
    let text = std::str::from_utf8(NATURAL_REVIT_IFC).expect("IFC is text").replacen("FILE_SCHEMA(('IFC2X3'))", "FILE_SCHEMA(('IFC4'))", 1);
    let error = <Ifc2x3AnyEditor as ArtifactEditor>::decode_natural_file(text.as_bytes()).expect_err("the wrong schema is refused");
    assert!(error.to_string().to_lowercase().contains("ifc2x3"), "{error}");
}
