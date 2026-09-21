use super::*;
use std::net::TcpListener;
use std::process::Command;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

static NEXT_CONTAINER: AtomicU64 = AtomicU64::new(1);

pub(super) struct Neo4jContainer {
    name: String,
    pub(super) uri: String,
}

impl Drop for Neo4jContainer {
    fn drop(&mut self) {
        let _ = Command::new("docker").args(["rm", "--force", &self.name]).output();
    }
}

impl Neo4jContainer {
    pub(super) async fn connect(&self) -> Neo4jDirectory {
        Neo4jDirectory::connect(&self.uri, "neo4j", "semio-test").await.expect("connect second neo4j directory")
    }
}

pub(super) async fn test_directory() -> (Neo4jDirectory, Neo4jContainer) {
    let port = TcpListener::bind(("127.0.0.1", 0)).expect("reserve neo4j fixture port").local_addr().expect("neo4j fixture address").port();
    let sequence = NEXT_CONTAINER.fetch_add(1, Ordering::Relaxed);
    let name = format!("semio-hub-neo4j-{}-{sequence}", std::process::id());
    let mapping = format!("127.0.0.1:{port}:7687");
    let output = Command::new("docker").args(["run", "--detach", "--rm", "--name", &name, "--env", "NEO4J_AUTH=neo4j/semio-test", "--publish", &mapping, "neo4j:5-community"]).output().expect("start docker for neo4j fixture");
    assert!(output.status.success(), "start neo4j fixture: {}", String::from_utf8_lossy(&output.stderr));
    let uri = format!("127.0.0.1:{port}");
    let container = Neo4jContainer { name, uri: uri.clone() };
    let mut last_error = None;
    for _ in 0..600 {
        match Neo4jDirectory::connect(&uri, "neo4j", "semio-test").await {
            Ok(directory) => return (directory, container),
            Err(error) => last_error = Some(error),
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    panic!("connect to neo4j fixture: {}", last_error.expect("neo4j fixture must report a connection error"));
}

fn claim_actor(id: &str) -> DirectoryActor {
    DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{id}#neo4j-invite") }
}

/// 🎟️ A real Neo4j transaction yields one immutable claim and retains accepted active invites through its projection rebuild policy.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn invite_redemption_claim_matches_neutral_contract() {
    let (primary, container) = test_directory().await;
    primary.seed().await.expect("seed neo4j invite fixture");
    let invited = primary.create_user("neo4j-invite@example.com", "Neo4j Invite", None, None, None).await.expect("create neo4j invited user");
    let issued = primary.issue_invite("default", SpaceRole::Spectator, 3600, "neo4j-invite-race").await.expect("issue neo4j invite");
    let secondary = container.connect().await;
    let barrier = Arc::new(tokio::sync::Barrier::new(3));
    let first = {
        let barrier = barrier.clone();
        let capability = issued.capability.clone();
        let user_id = invited.id.clone();
        tokio::spawn(async move {
            barrier.wait().await;
            primary.redeem_invite_atomic(&capability, &claim_actor(&user_id), &user_id, Hlc { physical_ms: 1, logical: 0 }).await
        })
    };
    let second = {
        let barrier = barrier.clone();
        let capability = issued.capability.clone();
        let user_id = invited.id.clone();
        tokio::spawn(async move {
            barrier.wait().await;
            secondary.redeem_invite_atomic(&capability, &claim_actor(&user_id), &user_id, Hlc { physical_ms: 2, logical: 0 }).await
        })
    };
    barrier.wait().await;
    let commits = [first.await.expect("first neo4j claim").expect("first neo4j result"), second.await.expect("second neo4j claim").expect("second neo4j result")];
    assert_eq!(commits.iter().filter(|commit| matches!(commit, InviteRedemptionCommit::NewlyCommitted { .. })).count(), 1);
    assert_eq!(commits.iter().filter(|commit| matches!(commit, InviteRedemptionCommit::AlreadyCommitted { .. })).count(), 1);
    let event_ids: std::collections::BTreeSet<_> = commits
        .iter()
        .map(|commit| match commit {
            InviteRedemptionCommit::NewlyCommitted { event } | InviteRedemptionCommit::AlreadyCommitted { event } => event.id.as_str(),
        })
        .collect();
    assert_eq!(event_ids.len(), 1);

    let directory = container.connect().await;
    let invite = directory.list_invites("default").await.expect("neo4j claimed invite").into_iter().find(|record| record.id == issued.record.id).expect("claimed neo4j invite row");
    assert!(invite.accepted_at.is_some());
    assert_eq!(invite.accepted_event_id.as_deref(), event_ids.first().copied());
    assert_eq!(directory.get_role("default", &invited.id).await.expect("neo4j invite membership"), Some(SpaceRole::Spectator));
    let head = directory.head_seq().await.expect("neo4j head before forged redemption");
    let forged = NewDirectoryEvent {
        hlc: Hlc { physical_ms: 3, logical: 0 },
        actor: claim_actor(&invited.id),
        space_id: Some("default".into()),
        user_id: Some(invited.id.clone()),
        body: DirectoryEventBody::InviteRedeemed { space_id: "default".into(), user_id: invited.id.clone(), invite_id: issued.record.id.clone(), role: DirectorySpaceRole::Spectator },
    };
    assert!(matches!(directory.append_events(&[forged]).await, Err(DirectoryError::Conflict(_))));
    assert_eq!(directory.head_seq().await.expect("neo4j head after forged redemption"), head);

    let rollback_invite = directory.issue_invite("default", SpaceRole::Spectator, 3600, "neo4j-invite-rollback").await.expect("issue rollback invite");
    let mut txn = directory.graph.start_txn().await.expect("begin neo4j rollback fixture");
    txn.run(query("MATCH (i:SpaceInvite {id: $id}) SET i.acceptedAt = $accepted_at, i.acceptedEventId = $event_id").param("id", rollback_invite.record.id.clone()).param("accepted_at", 101i64).param("event_id", "rolled-back-event"))
        .await
        .expect("write uncommitted neo4j marker");
    txn.rollback().await.expect("rollback neo4j marker");
    assert_eq!(directory.list_invites("default").await.expect("neo4j invites after rollback").into_iter().find(|record| record.id == rollback_invite.record.id).expect("rollback invite row").accepted_at, None);
    let before = directory.list_invites("default").await.expect("neo4j invites before rebuild");
    directory.rebuild_projections().await.expect("neo4j rebuild");
    let after = directory.list_invites("default").await.expect("neo4j invites after rebuild");
    assert_eq!(after, before);
    assert_eq!(directory.get_role("default", &invited.id).await.expect("rebuilt neo4j membership"), Some(SpaceRole::Spectator));
}

#[tokio::test]
async fn directory_event_page_v1_append_admission_is_transactional_neo4j() {
    let (directory, _container) = test_directory().await;
    directory.seed().await.expect("seed");
    let head = directory.head_seq().await.expect("head before boundary event");
    let mut event = NewDirectoryEvent {
        hlc: Hlc { physical_ms: 1, logical: 0 },
        actor: DirectoryActor { kind: DirectoryActorKind::System, id: "system:event-page-admission".into() },
        space_id: Some("default".into()),
        user_id: None,
        body: DirectoryEventBody::SpaceRenamed { space_id: "default".into(), name: String::new() },
    };
    let candidate = DirectoryEvent { seq: head + 1, id: time_ordered_id(), hlc: event.hlc, actor: event.actor.clone(), space_id: event.space_id.clone(), user_id: None, body: event.body.clone(), recorded_at_ms: now_ms() };
    let base = directory::os_pack::json::to_json_string(&candidate).len();
    let DirectoryEventBody::SpaceRenamed { name, .. } = &mut event.body else { unreachable!() };
    *name = "x".repeat(directory::os_directory::DIRECTORY_EVENT_PAGE_MAX_EVENT_BYTES - base);
    let exact = directory.append_events(&[event.clone()]).await.expect("append exact event-page boundary");
    assert_eq!(directory::os_pack::json::to_json_string(&exact[0]).len(), directory::os_directory::DIRECTORY_EVENT_PAGE_MAX_EVENT_BYTES);
    let head = directory.head_seq().await.expect("head before oversized event");
    let before = directory.get_space("default").await.expect("space before oversized event");
    let DirectoryEventBody::SpaceRenamed { name, .. } = &mut event.body else { unreachable!() };
    name.push('x');
    assert!(matches!(directory.append_events(&[event]).await, Err(DirectoryError::Conflict(_))));
    assert_eq!(directory.head_seq().await.expect("head after oversized event"), head);
    assert_eq!(directory.get_space("default").await.expect("space after oversized event"), before);
}

// 🔬️ ticket 26/09/18 slice AU3 — the credential-lifecycle pair against a real neo4j, mirroring
// the sqlite law `credential_writes_and_facts_stay_in_one_transaction`: the projection column and
// the `credential-changed` fact move together, an unknown target writes neither, and a refused
// sign-in's fact carries its public reason code and no credential material.
#[tokio::test]
async fn credential_writes_and_facts_stay_in_one_transaction() {
    let (directory, _container) = test_directory().await;
    directory.seed().await.expect("seed");
    let user = directory.create_user("neo4j-credential@example.com", "Credential", None, None, None).await.expect("create user");
    assert_eq!(directory.get_user(&user.id).await.expect("read user").expect("user").password_hash, None);

    let encoded = "pbkdf2-sha256$210000$00112233445566778899aabbccddeeff$0000000000000000000000000000000000000000000000000000000000000001";
    directory.set_password_credential(&user.id, encoded, Some(&user.id), "correlation-one").await.expect("set credential");
    assert_eq!(directory.get_user(&user.id).await.expect("read user").expect("user").password_hash.as_deref(), Some(encoded));
    let after_set = directory.list_auth_audit(64, 0).await.expect("audit after set");
    let changed: Vec<_> = after_set.iter().filter(|record| record.event_kind == "credential-changed").collect();
    assert_eq!(changed.len(), 1);
    assert_eq!(changed[0].target_user_id.as_deref(), Some(user.id.as_str()));
    assert_eq!(changed[0].outcome_code, "success");
    assert_eq!(changed[0].correlation_id, "correlation-one");
    assert!(!after_set.iter().any(|record| format!("{record:?}").contains(encoded)), "no audit fact may carry credential material");

    assert!(directory.set_password_credential("usr_absent", encoded, None, "correlation-two").await.is_err());
    assert_eq!(directory.list_auth_audit(64, 0).await.expect("audit after refusal").len(), after_set.len(), "a refused credential write appends nothing");

    let fact = CredentialAuditFactV1 {
        event_kind: "credential-sign-in".into(),
        target_user_id: Some(user.id.clone()),
        actor_user_id: None,
        outcome_code: "failure".into(),
        reason_code: Some("invalid-credentials".into()),
        correlation_id: "correlation-three".into(),
        peer_class: "browser".into(),
    };
    let appended = directory.append_credential_audit(&fact).await.expect("append sign-in fact");
    assert_eq!(appended.reason_code.as_deref(), Some("invalid-credentials"));
    assert_eq!(appended.peer_class, "browser");
    assert_eq!(directory.list_auth_audit(64, 0).await.expect("audit after sign-in fact").len(), after_set.len() + 1);
}

// 🔮️ The backend-neutral share-scope corpus (`🧪️tests/🔮️backend-corpus/`) over a real Neo4j.
#[tokio::test]
async fn share_scope_corpus_v1_holds_on_neo4j() {
    let (directory, _container) = test_directory().await;
    directory.seed().await.expect("seed");
    crate::directory::backend_corpus::assert_share_scope_corpus_v1(&directory).await;
}

// 🏛️ ticket 26/09/18 slice DB3 — the five directory reads `/directory/spaces/{id}` performs, over a real Neo4j.
#[tokio::test]
async fn space_administration_read_surface_v1_holds_on_neo4j() {
    let (directory, _container) = test_directory().await;
    directory.seed().await.expect("seed");
    crate::directory::backend_corpus::assert_space_administration_read_surface_v1(&directory).await;
}

// 🔬️ ticket 26/09/18 slice DB2 — the neo4j twin of the directory format-stamp law.
#[tokio::test]
async fn directory_format_stamp_is_written_and_a_foreign_format_is_refused_neo4j() {
    let (directory, container) = test_directory().await;
    let mut result = directory.graph.execute(query("MATCH (f:DirectoryFormat {id: 'singleton'}) RETURN f.schema AS schema, f.version AS version")).await.expect("read stamp");
    let row = result.next().await.expect("stamp row").expect("stamp written on creation");
    assert_eq!(row.get::<String>("schema").expect("schema"), crate::directory::DIRECTORY_FORMAT_SCHEMA);
    assert_eq!(row.get::<i64>("version").expect("version"), crate::directory::DIRECTORY_FORMAT_VERSION);
    drop(result);

    directory.graph.run(query("MATCH (f:DirectoryFormat {id: 'singleton'}) SET f.version = $version").param("version", crate::directory::DIRECTORY_FORMAT_VERSION + 1)).await.expect("forge a newer format");
    let refused = Neo4jDirectory::connect(&container.uri, "neo4j", "semio-test").await.err().map(|error| error.to_string()).unwrap_or_default();
    assert!(refused.contains("no migration framework"), "a newer format must be refused by name, got {refused:?}");

    directory.graph.run(query("MATCH (f:DirectoryFormat {id: 'singleton'}) SET f.schema = 'semio/hub/directory-format/v9', f.version = $version").param("version", crate::directory::DIRECTORY_FORMAT_VERSION)).await.expect("forge an unknown schema");
    let unknown = Neo4jDirectory::connect(&container.uri, "neo4j", "semio-test").await.err().map(|error| error.to_string()).unwrap_or_default();
    assert!(unknown.contains("unknown format stamp"), "an unknown schema must be refused by name, got {unknown:?}");

    directory.graph.run(query("MATCH (f:DirectoryFormat {id: 'singleton'}) SET f.schema = $schema, f.version = $version").param("schema", crate::directory::DIRECTORY_FORMAT_SCHEMA).param("version", crate::directory::DIRECTORY_FORMAT_VERSION)).await.expect("restore the stamp");
    container.connect().await;
}
