
use super::*;
use semio_framework_os::{ArtifactBody, S_WORKFLOW_SCHEMA};
use semio_s_artifact_space_space::standards::v1::subsets::any::schema::snapshot::{empty_space_index_snapshot, SpaceArtifactDialect, SpaceArtifactRow};

#[semio_framework_async_macros::async_test]
async fn projects_every_row_into_a_root_level_collection_entry() {
    let mut index = empty_space_index_snapshot("space-1");
    index.artifacts.push(SpaceArtifactRow {
        id: "artifact-1".into(),
        name: "First".into(),
        kind_id: "space.sdraw".into(),
        schema: S_WORKFLOW_SCHEMA.into(),
        dialect: SpaceArtifactDialect { artifact_kind: "s.workflow".into(), standard: "1".into(), subset: "*".into() },
        created_at_ms: 1,
        created_by: "user:1".into(),
        updated_at_ms: 1,
        updated_by: "user:1".into(),
    });
    let collection = project_space_index_to_collection(&index).await;
    assert_eq!(collection.name, "space-1");
    assert_eq!(collection.entries.len(), 1);
    let entry = &collection.entries[0];
    assert_eq!(entry.id, "artifact-1");
    assert!(entry.folder_id.is_none());
    let ArtifactBody::Document { schema, document_id } = entry.body.as_ref() else { panic!("expected a document body") };
    assert_eq!(schema, S_WORKFLOW_SCHEMA);
    assert_eq!(document_id, "artifact-1");
}
