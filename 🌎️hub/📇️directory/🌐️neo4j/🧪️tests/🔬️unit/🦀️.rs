use super::*;
use std::net::TcpListener;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

static NEXT_CONTAINER: AtomicU64 = AtomicU64::new(1);

pub(super) struct Neo4jContainer {
    name: String,
    uri: String,
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
