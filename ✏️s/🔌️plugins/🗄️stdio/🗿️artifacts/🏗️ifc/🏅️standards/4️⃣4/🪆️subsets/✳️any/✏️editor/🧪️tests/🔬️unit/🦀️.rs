use super::*;

#[semio_framework_async_macros::async_test]
async fn create_editor_builds_a_definition_for_the_editor_role() {
    let def = create_ifc4_any_editor();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
    assert_eq!(def.dialect, IFC4_ANY_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<Ifc4AnyEditor as ArtifactEditor>::DIALECT, IFC4_ANY_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn editor_and_viewer_share_one_dialect() {
    semio_framework_plugin::artifact_app_laws::assert_editor_and_viewer_share_dialect::<Ifc4AnyEditor, crate::viewer::ifc4_any::Ifc4AnyViewer>().await;
}

semio_framework_plugin::history_edit_acceptance_law!("stdio", super::Ifc4AnyEditor, || semio_framework_plugin::App { definition: super::create_ifc4_any_editor(), examples: Vec::new() }, "../../🏅️standards/4️⃣4/🪆️subsets/✳️any");

#[semio_framework_async_macros::async_test]
async fn selected_semantic_paths_use_retained_admission_and_argument_mutations() {
    use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::{from_part21_document,IfcValue};
    let fixture: semio_framework_pack_json::Value=semio_framework_pack_json::parse(include_str!("../../../🧬️schema/🔎️semantics/🧪️tests/🔣️.json"),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let source=format!("ISO-10303-21;HEADER;FILE_DESCRIPTION((''),'2;1');FILE_NAME('','',(''),(''),'','','');FILE_SCHEMA(('IFC4'));ENDSEC;DATA;{}ENDSEC;END-ISO-10303-21;",fixture["data"].as_str().unwrap());
    let snapshot=from_part21_document("stdio.ifc",&semio_s_artifact_stdio_contract::part21::parse_part21(&source).unwrap());
    let event=editing::SnapshotEditEvent::SetValue{path:"/ifc-10/name".into(),value:semio_framework_value::DslValue::String("Selected Wall".into())};
    assert!(<Ifc4AnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_is_admitted(&event,&snapshot));
    let emission=<Ifc4AnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&event,&snapshot).unwrap();
    assert_eq!(emission.artifact_mutations.len(),1);
    match &emission.artifact_mutations[0]{IfcMutation::SetEntityArg(edit)=>{assert_eq!((edit.id,edit.index),(10,2));assert_eq!(edit.value,IfcValue::String("Selected Wall".into()));},_=>panic!("semantic edit must use existing argument mutation")}
    let clear=editing::SnapshotEditEvent::RemoveValue{path:"/ifc-10/name".into()};
    let emission=<Ifc4AnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&clear,&snapshot).unwrap();
    assert!(matches!(&emission.artifact_mutations[0],IfcMutation::SetEntityArg(edit) if edit.value==IfcValue::Unset));
    for path in ["/ifc-01/name","/ifc-10/globalId/extra","/ifc-10/objectPlacement"]{
        let invalid=editing::SnapshotEditEvent::SetValue{path:path.into(),value:semio_framework_value::DslValue::String("bad".into())};
        assert!(!<Ifc4AnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_is_admitted(&invalid,&snapshot));
    }
    let topology=crate::standards::v4::subsets::any::io::inspector::topology(&snapshot);
    assert!(topology.domains["ifc-entities"].ordered.iter().any(|node|node.id=="ifc-10"&&node.granularity=="entity"));
    let definition=create_ifc4_any_editor();
    assert!(definition.interactions.iter().any(|domain|domain.id=="ifc-entities"));
    assert!(definition.window_kinds.iter().any(|window|window.body_key==crate::standards::v4::subsets::any::io::inspector::BODY_KEY));
}

#[semio_framework_async_macros::async_test]
async fn semantic_retained_hook_uses_original_cancellation_and_fuel(){
    use semio_framework_job::{StepContext,StepBudget,OperationId,Generation,root_cancel_token};
    use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::from_part21_document;
    fn now()->Option<u64>{Some(1)}
    let fixture:semio_framework_pack_json::Value=semio_framework_pack_json::parse(include_str!("../../../🧬️schema/🔎️semantics/🧪️tests/🔣️.json"),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();let source=format!("ISO-10303-21;HEADER;FILE_DESCRIPTION((''),'2;1');FILE_NAME('','',(''),(''),'','','');FILE_SCHEMA(('IFC4'));ENDSEC;DATA;{}ENDSEC;END-ISO-10303-21;",fixture["data"].as_str().unwrap());let snapshot=from_part21_document("stdio.ifc",&semio_s_artifact_stdio_contract::part21::parse_part21(&source).unwrap());
    let event=editing::SnapshotEditEvent::SetValue{path:"/ifc-10/name".into(),value:semio_framework_value::DslValue::String("Changed".into())};
    for (fuel,cancelled,expected) in [(0,false,false),(1,false,false),(100,true,false),(100,false,true)]{let cancel=root_cancel_token();if cancelled{cancel.cancel_now();}let mut sequence=0;let mut progress=Default::default();let mut cx=StepContext::new(OperationId(714),Generation(3),StepBudget::new(fuel,1000,Default::default()),cancel,now,&mut sequence,&mut progress);let result=<Ifc4AnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit_in_context(&event,&snapshot,&mut cx);assert_eq!(result.is_ok(),expected);if let Ok(emission)=result{assert_eq!(emission.artifact_mutations.len(),1);}assert!(cx.fuel_remaining()<=fuel);}
}

const NATURAL_OFFICIAL_IFC: &str = include_str!("../../../🧪️tests/🧫️fixtures/🏛️official/column-straight-rectangle-tessellation.ifc");

fn instance_ids(text: &str) -> Vec<u64> {
    text.lines().filter_map(|line| line.strip_prefix('#')?.split('=').next()?.trim().parse().ok()).collect()
}

#[test]
fn natural_file_entity_edit_keeps_every_id_and_every_unedited_entity() {
    let opened = <Ifc4AnyEditor as ArtifactEditor>::decode_natural_file(NATURAL_OFFICIAL_IFC.as_bytes()).expect("an official IFC4 file opens");
    let source_ids = instance_ids(NATURAL_OFFICIAL_IFC);
    assert_eq!(opened.entities.iter().map(|entity| entity.id).collect::<Vec<_>>(), source_ids, "ids and order come from the file text");
    let project = opened.entities.iter().find(|entity| entity.name.eq_ignore_ascii_case("IFCPROJECT")).expect("the file declares a project").id;
    let mut edited = opened.clone();
    semio_s_artifact_stdio_contract::apply_mutation(&mut edited, &IfcMutation::SetEntityArg(semio_s_artifact_stdio_ifc_4_mutations::standards::v4::subsets::any::schema::mutations::set_entity_arg::SetEntityArg { id: project, index: 2, value: semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcValue::String("Renamed Project".into()) }));
    let saved = <Ifc4AnyEditor as ArtifactEditor>::encode_natural_file(&edited).expect("save");
    let saved_text = std::str::from_utf8(&saved).expect("IFC is text");
    assert!(saved_text.contains("'Renamed Project'"));
    assert_eq!(instance_ids(saved_text), source_ids, "no id was renumbered, dropped or added");
    let reopened = <Ifc4AnyEditor as ArtifactEditor>::decode_natural_file(&saved).expect("reopen");
    assert_eq!(reopened, edited);
    for (before, after) in opened.entities.iter().zip(&reopened.entities) {
        if before.id != project {
            assert_eq!(before, after, "entity #{} was not edited", before.id);
        }
    }
}

#[test]
fn natural_file_refuses_a_dangling_reference_and_the_wrong_schema() {
    let dangling = NATURAL_OFFICIAL_IFC.replacen("#37= IFCPROJECT('0CxDbxzA1B4eLeOw9eIjQx',$,'Project',$,$,$,$,(#40),#36);", "#37= IFCPROJECT('0CxDbxzA1B4eLeOw9eIjQx',$,'Project',$,$,$,$,(#40),#99999);", 1);
    let error = <Ifc4AnyEditor as ArtifactEditor>::decode_natural_file(dangling.as_bytes()).expect_err("a missing target is refused");
    assert!(error.to_string().contains("#99999"), "{error}");
    let wrong = NATURAL_OFFICIAL_IFC.replacen("FILE_SCHEMA(('IFC4'))", "FILE_SCHEMA(('IFC2X3'))", 1);
    assert!(<Ifc4AnyEditor as ArtifactEditor>::decode_natural_file(wrong.as_bytes()).is_err());
}
