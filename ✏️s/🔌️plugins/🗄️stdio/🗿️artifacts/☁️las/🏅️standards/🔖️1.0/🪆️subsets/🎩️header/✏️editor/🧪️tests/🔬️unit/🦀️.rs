use crate::apply_mutation;
use super::*;

#[semio_framework_async_macros::async_test]
async fn create_editor_builds_a_definition_for_the_editor_role() {
    let def = create_las_any_editor();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
    assert_eq!(def.dialect, LAS_ANY_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<LasAnyEditor as ArtifactEditor>::DIALECT, LAS_ANY_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn editor_and_viewer_share_one_dialect() {
    semio_framework_plugin::artifact_app_laws::assert_editor_and_viewer_share_dialect::<LasAnyEditor, crate::viewer::las::LasAnyViewer>().await;
}

semio_framework_plugin::history_edit_acceptance_law!("stdio", super::LasAnyEditor, || semio_framework_plugin::App { definition: super::create_las_any_editor(), examples: Vec::new() }, "../../🏅️standards/🔖️1.0/🪆️subsets/🎩️header");

#[semio_framework_async_macros::async_test]
async fn details_edits_resolve_to_the_kind_of_the_addressed_field() {
    
    use crate::standards::v1_0::subsets::any::schema::snapshot::{LasPoint, LasVlr};
    let base = LasSnapshot {
        vlrs: vec![LasVlr { user_id: "u".into(), record_id: 1, description: "d".into(), data: vec![1, 2] }],
        points: vec![LasPoint::default(), LasPoint::default()],
        ..LasSnapshot::default()
    };
    let emit = |event: editing::SnapshotEditEvent| <LasAnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, &base);
    let renamed = emit(editing::SnapshotEditEvent::SetValue { path: "/header/systemIdentifier".into(), value: semio_framework_value::DslValue::String("scanner".into()) }).expect("a header edit resolves");
    let [mutation @ LasMutation::SetSystemIdentifier(_)] = renamed.artifact_mutations.as_slice() else { panic!("a header field edit raises the kind of that group") };
    let mut state = base.clone();
    apply_mutation(&mut state, mutation);
    assert_eq!(state.header.system_identifier, "scanner");
    let scaled = emit(editing::SnapshotEditEvent::SetValue { path: "/header/yScale".into(), value: semio_framework_value::DslValue::Number(semio_framework_value::Number::Float(0.5)) }).expect("a scale edit resolves");
    assert!(matches!(scaled.artifact_mutations.as_slice(), [LasMutation::SetScaleAndOffset(_)]));
    let structural = emit(editing::SnapshotEditEvent::SetValue { path: "/header/headerSize".into(), value: semio_framework_value::DslValue::uint(300) });
    assert!(structural.is_err(), "a structural header field has no kind");
    let moved = emit(editing::SnapshotEditEvent::SetValue { path: "/points/1/classification".into(), value: semio_framework_value::DslValue::uint(2) }).expect("a point field edit resolves");
    assert!(matches!(moved.artifact_mutations.as_slice(), [LasMutation::SetPoint(_)]));
    let data = emit(editing::SnapshotEditEvent::SetValue { path: "/vlrs/0/data".into(), value: semio_framework_value::ToValue::to_value(&vec![9u8]) }).expect("a VLR data edit resolves");
    assert!(matches!(data.artifact_mutations.as_slice(), [LasMutation::SetVlrData(_)]));
    let identity = emit(editing::SnapshotEditEvent::SetValue { path: "/vlrs/0/recordId".into(), value: semio_framework_value::DslValue::uint(7) }).expect("a VLR identity edit resolves");
    assert!(matches!(identity.artifact_mutations.as_slice(), [LasMutation::RemoveVlr(_), LasMutation::InsertVlr(_)]));
}
