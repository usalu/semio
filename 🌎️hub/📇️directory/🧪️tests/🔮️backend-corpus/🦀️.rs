//! 🔮️ The backend-neutral directory corpora. One language-agnostic fixture
//! (`📇️directory/🧫️fixtures/🔑️share-token-vectors/🔣️.json`) drives the identical assertion body
//! against SQLite, PostgreSQL and Neo4j, so a grant that is honoured on the proven sqlite lane
//! cannot be honoured differently — or silently not at all — on either of the other two. Before this
//! module the fixture corpus was read by the sqlite lane alone and the other two backends shared no
//! fixture with it at all.

use crate::directory::model::*;
use crate::directory::{DirectoryError, HubClock, HubDirectory, NewDirectoryEvent};
use directory::os_directory::{DirectoryActor, DirectoryActorKind, DirectoryEventBody, DirectorySpaceKind, DirectorySpaceRole, DirectorySpaceVisibility, DocumentDescriptor, DocumentFrontier, DocumentOwner};

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct CorpusScope {
    space_id: String,
    document_id: String,
}

#[derive(serde::Deserialize)]
struct CorpusScopes {
    grant: CorpusScope,
    allowed: CorpusScope,
    denied: CorpusScope,
}

#[derive(serde::Deserialize)]
struct CorpusEncodingVector {
    bytes: Vec<u8>,
    hex: String,
}

#[derive(serde::Deserialize)]
struct ShareScopeCorpusV1 {
    encoding: Vec<CorpusEncodingVector>,
    scope: CorpusScopes,
}

fn corpus() -> ShareScopeCorpusV1 {
    serde_json::from_str(include_str!("../../🧫️fixtures/🔑️share-token-vectors/🔣️.json")).expect("share-token vectors")
}

fn share_descriptor(scope: &DocumentScope) -> DocumentDescriptor {
    DocumentDescriptor {
        space_id: scope.space_id.clone(),
        document_id: scope.document_id.clone(),
        artifact_kind: "s.gis.gismap".into(),
        artifact_schema: "s.gis.gismap@1/*".into(),
        owner: DocumentOwner { plugin_id: "gis".into(), package_id: "semio:gis".into(), version: "1.0.0".into(), package_hash: "11".repeat(32) },
        pack_schema_hash: "22".repeat(32),
        bootstrap_version: 1,
        bootstrap_frontier: DocumentFrontier { head_seq: 1, commit_seq: 1, epoch: 1 },
        bootstrap_snapshot_hash: "33".repeat(32),
    }
}

/// 🔑️ Runs the whole corpus against one seeded directory: the owned hex encoding matches the
/// fixture's vectors, a grant authorizes its own scope, the fixture's cross-space scope is refused,
/// revocation is durable and idempotent-by-refusal, and a zero-lifetime grant is rejected.
pub(crate) async fn assert_share_scope_corpus_v1<D: HubDirectory + ?Sized>(directory: &D) {
    let corpus = corpus();
    for vector in &corpus.encoding {
        assert_eq!(crate::directory::encode_capability_bytes(&vector.bytes), vector.hex, "owned hex encoding must match the language-neutral vector");
    }

    let grant_scope = DocumentScope::new(corpus.scope.grant.space_id.clone(), corpus.scope.grant.document_id.clone());
    let allowed_scope = DocumentScope::new(corpus.scope.allowed.space_id.clone(), corpus.scope.allowed.document_id.clone());
    let denied_scope = DocumentScope::new(corpus.scope.denied.space_id.clone(), corpus.scope.denied.document_id.clone());

    let mut clock = HubClock::new();
    directory
        .append_events(&[NewDirectoryEvent {
            hlc: clock.tick(),
            actor: DirectoryActor { kind: DirectoryActorKind::User, id: "seed".into() },
            space_id: Some(grant_scope.space_id.clone()),
            user_id: Some("seed".into()),
            body: DirectoryEventBody::DocumentAnnounced { descriptor: share_descriptor(&grant_scope) },
        }])
        .await
        .expect("announce share document");

    let grant = directory.issue_share_token(&grant_scope, 60, "backend-corpus").await.expect("mint token");
    assert!(grant.capability.expose_once().starts_with("share.v1."), "every backend mints the same capability dialect");
    assert!(directory.authenticate_share(&allowed_scope, &grant.capability).await.expect("authenticate allowed"), "the granted scope must authorize");
    assert!(!directory.authenticate_share(&denied_scope, &grant.capability).await.expect("authenticate denied"), "a grant must not cross the fixture's space boundary");

    directory.revoke_share_token(&grant_scope, &grant.record.id, "backend-corpus-revoke", "backend-corpus").await.expect("revoke token");
    assert!(!directory.authenticate_share(&allowed_scope, &grant.capability).await.expect("authenticate revoked"), "revocation is durable");
    assert!(matches!(directory.revoke_share_token(&grant_scope, &grant.record.id, "backend-corpus-revoke", "backend-corpus").await, Err(DirectoryError::NotFound(_))), "a second revoke reports NotFound on every backend");

    assert!(matches!(directory.issue_share_token(&grant_scope, 0, "backend-corpus-invalid").await, Err(DirectoryError::Conflict(_))), "a zero-lifetime grant is refused on every backend");
}


/// 🏛️ Runs the exact directory read surface the hub's `/directory/spaces/{id}` administration page
/// performs, against one seeded directory.
///
/// Every one of these five reads is a *page* read that no other law touched, and a backend can fail
/// them wholesale without any write law noticing — which is precisely what happened: the Neo4j
/// `list_admin_space_summaries_page` Cypher was invalid (`Aggregation column contains implicit
/// grouping expressions`), so the route answered a bare `500` on every Neo4j hub ever booted, while
/// every write law and the whole `directory::` suite stayed green (ticket 26/09/18, slice DB3).
/// The route reads these five and nothing else, so running them here is running the page.
pub(crate) async fn assert_space_administration_read_surface_v1<D: HubDirectory + ?Sized>(directory: &D) {
    let author = directory.create_user("surface-author@example.com", "Surface Author", None, None, None).await.expect("create the space author");
    let space_id = format!("surface-{}", &author.id);
    let scope = DocumentScope::new(space_id.clone(), "surface-document".to_string());

    let mut clock = HubClock::new();
    let actor = DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{}", author.id) };
    directory
        .append_events(&[
            NewDirectoryEvent {
                hlc: clock.tick(),
                actor: actor.clone(),
                space_id: Some(space_id.clone()),
                user_id: Some(author.id.clone()),
                body: DirectoryEventBody::SpaceCreated {
                    space_id: space_id.clone(),
                    name: "Surface Atelier".into(),
                    space_kind: DirectorySpaceKind::Atelier,
                    visibility: DirectorySpaceVisibility::Private,
                    owner_user_id: author.id.clone(),
                },
            },
            NewDirectoryEvent {
                hlc: clock.tick(),
                actor: actor.clone(),
                space_id: Some(space_id.clone()),
                user_id: Some(author.id.clone()),
                body: DirectoryEventBody::MemberUpserted { space_id: space_id.clone(), user_id: author.id.clone(), role: DirectorySpaceRole::Author },
            },
            NewDirectoryEvent {
                hlc: clock.tick(),
                actor,
                space_id: Some(space_id.clone()),
                user_id: Some(author.id.clone()),
                body: DirectoryEventBody::DocumentAnnounced { descriptor: share_descriptor(&scope) },
            },
        ])
        .await
        .expect("create the surface space");

    let issued = directory.issue_invite(&space_id, SpaceRole::Spectator, 3600, "backend-corpus-surface").await.expect("issue the surface invite");

    let summaries = directory.list_admin_space_summaries_page(Some(&space_id), 0, 1).await.expect("the space summary page must answer on every backend");
    let summary = summaries.into_iter().next().expect("the space the corpus just created must be summarised");
    assert_eq!(summary.space.id, space_id, "the summary page must answer for the space it was asked about");
    assert_eq!(summary.member_count, 1, "one member was upserted");
    assert_eq!(summary.document_count, 1, "one document was announced");
    assert_eq!(summary.active_connections, 0, "no sync session was opened");
    assert!(summary.updated_at > 0, "updatedAt folds the space's own events, never a null");

    assert_eq!(directory.get_role(&space_id, &author.id).await.expect("read the caller role"), Some(SpaceRole::Author), "the page resolves the caller's role before it renders");

    let members = directory.list_space_administration_members_page(&space_id, None, crate::directory::SPACE_ADMINISTRATION_PAGE_FETCH_MAX).await.expect("the member page must answer on every backend");
    assert_eq!(members.len(), 1, "exactly the one upserted member");
    assert_eq!(members[0].user_id, author.id);
    assert_eq!(members[0].email, "surface-author@example.com", "the member row carries the identity the page prints");
    assert_eq!(members[0].role, SpaceRole::Author);

    let invites = directory.list_space_administration_invites_page(&space_id, None, crate::directory::SPACE_ADMINISTRATION_PAGE_FETCH_MAX).await.expect("the invite page must answer on every backend");
    assert_eq!(invites.len(), 1, "exactly the one issued invite");
    assert_eq!(invites[0].invite_id, issued.record.id);
    assert!(!invites[0].revoked && !invites[0].accepted, "a freshly issued invite is neither revoked nor accepted on any backend");

    let descriptors = directory.list_document_descriptors_page(Some(&space_id), 0, crate::directory::ADMIN_PAGE_FETCH_MAX).await.expect("the document page must answer on every backend");
    assert_eq!(descriptors.len(), 1, "exactly the one announced document");
    assert_eq!(descriptors[0].document_id, scope.document_id);
}
