use super::*;

#[semio_framework_async_macros::async_test]
async fn create_viewer_builds_a_definition_for_the_viewer_role() {
    let def = create_ifc2x3_cv20_viewer();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Viewer);
    assert_eq!(def.dialect, IFC2X3_CV20_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn viewer_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<Ifc2x3Cv20Viewer as ArtifactViewer>::DIALECT, IFC2X3_CV20_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn viewer_never_mutates_the_document_or_draft_store() {
    semio_framework_plugin::with_authoring_identity!(|identity| semio_framework_plugin::artifact_app_laws::assert_viewer_never_mutates::<Ifc2x3Cv20Viewer>(semio_s_artifact_stdio_contract::testing::mounted_policy(), &mut identity).await);
}

#[semio_framework_async_macros::async_test]
async fn own_selected_inspector_is_readonly_and_topology_backed(){let definition=create_ifc2x3_cv20_viewer();assert!(definition.interactions.iter().any(|domain|domain.id==inspector::DOMAIN));let window=definition.window_kinds.iter().find(|window|window.body_key==inspector::BODY_KEY).unwrap();assert!(window.actions.is_empty());}
