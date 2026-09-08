
use super::*;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::testkit::{meta, new_app as framework_new_app};

pub type SpaceIndexApp = semio_framework_plugin::VcsArtifactApp<EditorApp<SpaceIndexEditor>>;

pub async fn new_app() -> SpaceIndexApp {
    framework_new_app::<EditorApp<SpaceIndexEditor>>().await
}

pub async fn new_app_with_artifact() -> (SpaceIndexApp, String) {
    use crate::standards::v1::subsets::any::schema::snapshot::{SpaceArtifactDialect, SpaceArtifactRow, empty_space_index_snapshot};
    use semio_framework_plugin::PluginApp;
    use store::ArtifactDsl;
    let mut app = new_app().await;
    let id = "artifact-1".to_string();
    let mut snapshot = empty_space_index_snapshot("space-1");
    snapshot.artifacts.push(SpaceArtifactRow {
        id: id.clone(),
        name: "First".into(),
        kind_id: "s.draw.draw".into(),
        schema: "s.draw.draw".into(),
        dialect: SpaceArtifactDialect { artifact_kind: "s.draw.draw".into(), standard: "1".into(), subset: "*".into() },
        created_at_ms: 1,
        created_by: "user:1".into(),
        updated_at_ms: 1,
        updated_by: "user:1".into(),
    });
    app.load_document_text(&store::ArtifactTextFiles { dsl: snapshot.print_dsl(), ops: String::new() }).await.expect("load test artifact");
    (app, id)
}

pub async fn new_app_with_indexed_artifact() -> (SpaceIndexApp, String) {
    use crate::editor::space_index::commands::fold_directory_events::FoldDirectoryEvents;
    use crate::standards::v1::subsets::any::schema::snapshot::empty_space_index_snapshot;
    use semio_framework_os_kernel::os_directory::{
        ArtifactHash, DirectoryActor, DirectoryActorKind, DirectoryEvent, DirectoryEventBody, DirectorySpaceKind, DirectorySpaceVisibility, DocumentDescriptor, DocumentFrontier, DocumentIndexEntryV1, DocumentOwner, DocumentScope, Hlc,
    };
    use semio_framework_plugin::{ArtifactDialect, PluginApp};
    use store::ArtifactDsl;

    let mut app = new_app().await;
    let id = "artifact-0123456789abcdef0123456789abcdef".to_string();
    let snapshot = empty_space_index_snapshot("space-1");
    app.load_document_text(&store::ArtifactTextFiles { dsl: snapshot.print_dsl(), ops: String::new() }).await.expect("load empty Space index");
    let descriptor = DocumentDescriptor {
        space_id: "space-1".into(),
        document_id: id.clone(),
        artifact_kind: "s.draw.draw".into(),
        artifact_schema: "s.draw.draw".into(),
        owner: DocumentOwner { plugin_id: "draw".into(), package_id: "draw".into(), version: "1".into(), package_hash: "a".repeat(64) },
        pack_schema_hash: "b".repeat(64),
        bootstrap_version: 1,
        bootstrap_frontier: DocumentFrontier { head_seq: 1, commit_seq: 1, epoch: 1 },
        bootstrap_snapshot_hash: "c".repeat(64),
    };
    let event = |seq: u64, user_id: Option<&str>, body: DirectoryEventBody| DirectoryEvent {
        seq,
        id: format!("evt-{seq}"),
        hlc: Hlc { physical_ms: seq as i64, logical: 0 },
        actor: DirectoryActor { kind: DirectoryActorKind::System, id: "system:test".into() },
        space_id: Some("space-1".into()),
        user_id: user_id.map(Into::into),
        body,
        recorded_at_ms: seq as i64,
    };
    let events = vec![
        event(1, None, DirectoryEventBody::SpaceCreated { space_id: "space-1".into(), name: "Space 1".into(), space_kind: DirectorySpaceKind::Atelier, visibility: DirectorySpaceVisibility::Private, owner_user_id: "u-1".into() }),
        event(2, None, DirectoryEventBody::DocumentAnnounced { descriptor }),
        event(
            3,
            Some("u-1"),
            DirectoryEventBody::DocumentIndexed {
                scope: DocumentScope { space_id: "space-1".into(), document_id: id.clone() },
                descriptor_digest_v1: ArtifactHash([7; 32]),
                entry: DocumentIndexEntryV1 { name: "First".into(), dialect: ArtifactDialect { artifact_kind: "s.draw.draw".into(), standard: "1".into(), subset: "*".into() } },
            },
        ),
    ];
    app.dispatch_typed(SpaceIndexCommand::FoldDirectoryEvents(FoldDirectoryEvents { events_json: pack::to_json_string(&events) }), &meta("local")).await.expect("fold indexed artifact");
    let files = app.config_pack().await.expect("indexed config pack");
    let config = store::parse_document_pack::<SpaceIndexConfig, SpaceIndexConfigMutation>(&files.pack, &files.spr).await.expect("indexed config projection").snapshot;
    assert_eq!(config.indexed_artifacts.len(), 1);
    (app, id)
}

#[allow(dead_code)]
pub async fn dispatch(app: &mut SpaceIndexApp, command: SpaceIndexCommand) -> semio_framework_plugin::InvocationResult {
    app.dispatch_typed(command, &meta("local")).await.expect("dispatch")
}
