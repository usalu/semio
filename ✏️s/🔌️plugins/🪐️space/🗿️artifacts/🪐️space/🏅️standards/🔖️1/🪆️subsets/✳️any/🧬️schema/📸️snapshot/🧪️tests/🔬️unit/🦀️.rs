
use super::*;

#[semio_framework_async_macros::async_test]
async fn empty_snapshot_uses_the_space_index_schema() {
    let snapshot = empty_space_index_snapshot("space-1");
    assert_eq!(snapshot.schema, S_SPACE_INDEX_DOCUMENT_SCHEMA);
    assert_eq!(snapshot.space_id, "space-1");
    assert!(snapshot.artifacts.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn mint_artifact_id_probes_past_a_collision() {
    let existing = vec![SpaceArtifactRow { id: "artifact-1-0".into(), ..Default::default() }];
    assert_eq!(mint_artifact_id(&existing, 1), "artifact-1-1");
    assert_eq!(mint_artifact_id(&[], 1), "artifact-1-0");
}

#[semio_framework_async_macros::async_test]
async fn table_row_projects_the_seven_worker_brief_columns() {
    assert_eq!(SPACE_INDEX_TABLE_COLUMNS.len(), 7);
    let row = SpaceArtifactRow {
        id: "artifact-1".into(),
        name: "First".into(),
        kind_id: "s.draw.draw".into(),
        schema: "draw.document".into(),
        dialect: SpaceArtifactDialect { artifact_kind: "s.draw.draw".into(), standard: "1".into(), subset: "*".into() },
        created_at_ms: 1,
        created_by: "user:1".into(),
        updated_at_ms: 42,
        updated_by: "user:2".into(),
    };
    assert_eq!(space_index_table_row(&row, "user:9"), vec!["artifact-1", "First", "s.draw.draw", "*", "42", "user:2", "user:9"]);
}

#[semio_framework_async_macros::async_test]
async fn dsl_round_trips_default_and_populated_documents() {
    store::os_store::test_support::assert_dsl_round_trip(&SSpaceSnapshot::default());
    let mut populated = empty_space_index_snapshot("space-2");
    populated.artifacts.push(SpaceArtifactRow {
        id: "artifact-1".into(),
        name: "First".into(),
        kind_id: "space.sdraw".into(),
        schema: "s.draw".into(),
        dialect: SpaceArtifactDialect { artifact_kind: "s.draw".into(), standard: "1".into(), subset: "*".into() },
        created_at_ms: 1,
        created_by: "user:1".into(),
        updated_at_ms: 1,
        updated_by: "user:1".into(),
    });
    store::os_store::test_support::assert_dsl_round_trip(&populated);
}
