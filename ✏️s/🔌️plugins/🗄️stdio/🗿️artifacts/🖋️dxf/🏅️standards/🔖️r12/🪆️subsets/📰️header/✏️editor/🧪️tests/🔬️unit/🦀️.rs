use crate::apply_mutation;
use super::*;

#[semio_framework_async_macros::async_test]
async fn create_editor_builds_a_definition_for_the_editor_role() {
    let def = create_dxf_any_editor();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
    assert_eq!(def.dialect, DXF_ANY_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<DxfAnyEditor as ArtifactEditor>::DIALECT, DXF_ANY_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn editor_and_viewer_share_one_dialect() {
    semio_framework_plugin::artifact_app_laws::assert_editor_and_viewer_share_dialect::<DxfAnyEditor, crate::viewer::dxf::DxfAnyViewer>().await;
}

semio_framework_plugin::history_edit_acceptance_law!("stdio", super::DxfAnyEditor, || semio_framework_plugin::App { definition: super::create_dxf_any_editor(), examples: Vec::new() }, "../../🏅️standards/🔖️r12/🪆️subsets/📰️header");

#[semio_framework_async_macros::async_test]
async fn details_edits_resolve_to_the_kind_of_the_addressed_row() {
    use crate::schema::snapshot::{DxfEntity, DxfLine, DxfHeaderVar, DxfLayer, DxfValue};
    use crate::standards::v_r12::subsets::any::schema::mutations::{DxfMutation};
    let mut base = DxfSnapshot::default();
    base.tables.layers.push(DxfLayer { name: "0".into(), color: 7, linetype: "CONTINUOUS".into(), flags: 0, unknown_group_codes: vec![] });
    base.entities.push(DxfEntity::Line(DxfLine { start: [0.0, 0.0, 0.0], end: [1.0, 1.0, 0.0], layer: "0".into(), unknown_group_codes: vec![] }));
    base.header_vars.push(DxfHeaderVar { name: "$ACADVER".into(), group_code: 1, value: DxfValue::Str { value: "AC1009".into() }, extra_group_codes: vec![] });
    let emit = |event: editing::SnapshotEditEvent| <DxfAnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, &base);
    let recolor = emit(editing::SnapshotEditEvent::SetValue { path: "/tables/layers/0/color".into(), value: semio_framework_value::DslValue::uint(3) }).expect("a layer field edit resolves");
    let [mutation @ DxfMutation::SetLayer(_)] = recolor.artifact_mutations.as_slice() else { panic!("a layer field edit raises the layer kind") };
    let mut state = base.clone();
    apply_mutation(&mut state, mutation);
    assert_eq!(state.tables.layers[0].color, 3);
    let removal = emit(editing::SnapshotEditEvent::RemoveValue { path: "/entities/0".into() }).expect("an entity removal resolves");
    assert!(matches!(removal.artifact_mutations.as_slice(), [DxfMutation::RemoveEntity(_)]));
    let dropped = emit(editing::SnapshotEditEvent::RemoveValue { path: "/headerVars/0".into() }).expect("a header variable removal resolves");
    assert!(matches!(dropped.artifact_mutations.as_slice(), [DxfMutation::RemoveHeaderVar(_)]));
    let unnamed = emit(editing::SnapshotEditEvent::SetValue { path: "/schema".into(), value: semio_framework_value::DslValue::String("other".into()) }).expect_err("the schema stamp has no kind");
    assert_eq!(unnamed.code.0, "snapshot-edit.unsupported-path");
}
