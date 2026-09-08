use super::*;
use std::net::TcpListener;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

//#region 🔖️PostgresFixture
static NEXT_CONTAINER: AtomicU64 = AtomicU64::new(1);

pub(super) struct PostgresContainer {
    name: String,
    url: String,
}

impl Drop for PostgresContainer {
    fn drop(&mut self) {
        let _ = Command::new("docker").args(["rm", "--force", &self.name]).output();
    }
}

impl PostgresContainer {
    pub(super) async fn connect(&self) -> PostgresDirectory {
        PostgresDirectory::connect(&self.url).await.expect("connect second postgres directory")
    }
}

/// 🐘️ Starts a disposable real Postgres behind a private fixture boundary, without a Rust
/// container-orchestration dependency.
pub(super) async fn test_directory() -> (PostgresDirectory, PostgresContainer) {
    let port = TcpListener::bind(("127.0.0.1", 0)).expect("reserve postgres fixture port").local_addr().expect("postgres fixture address").port();
    let sequence = NEXT_CONTAINER.fetch_add(1, Ordering::Relaxed);
    let name = format!("semio-hub-postgres-{}-{sequence}", std::process::id());
    let mapping = format!("127.0.0.1:{port}:5432");
    let output = Command::new("docker").args(["run", "--detach", "--rm", "--name", &name, "--env", "POSTGRES_PASSWORD=postgres", "--publish", &mapping, "postgres:16-alpine"]).output().expect("start docker for postgres fixture");
    assert!(output.status.success(), "start postgres fixture: {}", String::from_utf8_lossy(&output.stderr));
    let url = format!("postgres://postgres:postgres@127.0.0.1:{port}/postgres");
    let container = PostgresContainer { name, url: url.clone() };
    let mut last_error = None;
    for _ in 0..300 {
        match PostgresDirectory::connect(&url).await {
            Ok(directory) => return (directory, container),
            Err(error) => last_error = Some(error),
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    panic!("connect to postgres fixture: {}", last_error.expect("postgres fixture must report a connection error"));
}
//#endregion 🔖️PostgresFixture

fn actor(id: &str) -> DirectoryActor {
    DirectoryActor { kind: DirectoryActorKind::User, id: id.to_string() }
}

fn claim_actor(id: &str) -> DirectoryActor {
    DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{id}#postgres-invite") }
}

/// 🌱️ `create_space`/`upsert_membership` were removed (writes now go through
/// `append_events` — see the module root's `//#region 🔖️Decider`); rebuilds just enough of a
/// `create-space` decision by hand so these backend tests do not need a full `DirectoryService`.
async fn seed_space(dir: &PostgresDirectory, clock: &mut HubClock, owner_user_id: &str, kind: DirectorySpaceKind) -> String {
    let space_id = time_ordered_id();
    let owner_role = if kind == DirectorySpaceKind::Archive { DirectorySpaceRole::Spectator } else { DirectorySpaceRole::Author };
    let events = vec![
        NewDirectoryEvent {
            hlc: clock.tick(),
            actor: actor(owner_user_id),
            space_id: Some(space_id.clone()),
            user_id: Some(owner_user_id.to_string()),
            body: DirectoryEventBody::SpaceCreated { space_id: space_id.clone(), name: "Space".into(), space_kind: kind, visibility: DirectorySpaceVisibility::Private, owner_user_id: owner_user_id.to_string() },
        },
        NewDirectoryEvent {
            hlc: clock.tick(),
            actor: actor(owner_user_id),
            space_id: Some(space_id.clone()),
            user_id: Some(owner_user_id.to_string()),
            body: DirectoryEventBody::MemberUpserted { space_id: space_id.clone(), user_id: owner_user_id.to_string(), role: owner_role },
        },
    ];
    dir.append_events(&events).await.expect("seed space");
    space_id
}

fn admin_audit_fact(phase: &str, outcome_code: &str) -> NewAdminOperationAuditRecord {
    NewAdminOperationAuditRecord {
        request_id: "request:postgres-race".into(),
        intent_digest: "1".repeat(64),
        operation_id: "operation:postgres-race".into(),
        occurred_at: now_ms(),
        phase: phase.into(),
        intent_kind: "delete-space".into(),
        target_kind: "space".into(),
        target_id: "space:one".into(),
        principal_user_id: "user:admin".into(),
        principal_session_id: "session:admin".into(),
        principal_generation: 7,
        correlation_id: "correlation:postgres-race".into(),
        event_seq_first: None,
        event_seq_last: None,
        outcome_code: outcome_code.into(),
        reason_code: None,
    }
}

#[tokio::test]
async fn admin_operation_audit_concurrent_absent_request_rereads_established_receipt() {
    let (directory, _container) = test_directory().await;
    let directory = Arc::new(directory);
    let barrier = Arc::new(tokio::sync::Barrier::new(17));
    let accepted = admin_audit_fact("accepted", "accepted");
    let mut writers = Vec::new();
    for _ in 0..16 {
        let directory = directory.clone();
        let barrier = barrier.clone();
        let accepted = accepted.clone();
        writers.push(tokio::spawn(async move {
            barrier.wait().await;
            directory.append_admin_operation_audit(&accepted).await
        }));
    }
    barrier.wait().await;
    let mut sequence = None;
    for writer in writers {
        let established = writer.await.expect("postgres audit writer").expect("race loser rereads receipt");
        if let Some(first) = sequence {
            assert_eq!(first, established.sequence);
        } else {
            sequence = Some(established.sequence);
        }
    }
    assert_eq!(directory.admin_operation_audit_for_request(&accepted.request_id).await.expect("postgres audit").len(), 1);
    let terminal = admin_audit_fact("succeeded", "space-deleted");
    assert_eq!(directory.append_admin_operation_audit(&terminal).await.expect("postgres terminal").fact.intent_digest, accepted.intent_digest);
}

/// 🎟️ A real PostgreSQL row lock yields one immutable redemption, rolls faults back, and preserves direct invite decisions across rebuild.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn invite_redemption_claim_matches_neutral_contract() {
    let (primary, container) = test_directory().await;
    primary.seed().await.expect("seed postgres invite fixture");
    let invited = primary.create_user("postgres-invite@example.com", "Postgres Invite", None, None, None).await.expect("create invited user");
    let issued = primary.issue_invite("default", SpaceRole::Spectator, 3600, "postgres-invite-race").await.expect("issue invite");
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
    let commits = [first.await.expect("first postgres claim").expect("first postgres result"), second.await.expect("second postgres claim").expect("second postgres result")];
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
    let invite = directory.list_invites("default").await.expect("postgres claimed invite").into_iter().find(|record| record.id == issued.record.id).expect("claimed invite row");
    assert!(invite.accepted_at.is_some());
    assert_eq!(invite.accepted_event_id.as_deref(), event_ids.first().copied());
    assert_eq!(directory.get_role("default", &invited.id).await.expect("postgres invite membership"), Some(SpaceRole::Spectator));
    let head = directory.head_seq().await.expect("head before forged redemption");
    let forged = NewDirectoryEvent {
        hlc: Hlc { physical_ms: 3, logical: 0 },
        actor: claim_actor(&invited.id),
        space_id: Some("default".into()),
        user_id: Some(invited.id.clone()),
        body: DirectoryEventBody::InviteRedeemed { space_id: "default".into(), user_id: invited.id.clone(), invite_id: issued.record.id.clone(), role: DirectorySpaceRole::Spectator },
    };
    assert!(matches!(directory.append_events(&[forged]).await, Err(DirectoryError::Conflict(_))));
    assert_eq!(directory.head_seq().await.expect("head after forged redemption"), head);

    let fault_user = directory.create_user("postgres-invite-fault@example.com", "Postgres Invite Fault", None, None, None).await.expect("create fault user");
    let fault_invite = directory.issue_invite("default", SpaceRole::Spectator, 3600, "postgres-invite-fault").await.expect("issue fault invite");
    sqlx_core::query::query("CREATE OR REPLACE FUNCTION hub_test_fail_invite_projection() RETURNS trigger AS $$ BEGIN RAISE EXCEPTION 'injected invite projection failure'; END; $$ LANGUAGE plpgsql")
        .execute(&directory.pool)
        .await
        .expect("create postgres invite failure function");
    sqlx_core::query::query("CREATE TRIGGER hub_test_fail_invite_projection BEFORE INSERT ON hub_space_membership FOR EACH ROW EXECUTE FUNCTION hub_test_fail_invite_projection()")
        .execute(&directory.pool)
        .await
        .expect("create postgres invite failure trigger");
    let fault_head = directory.head_seq().await.expect("head before projection fault");
    assert!(matches!(directory.redeem_invite_atomic(&fault_invite.capability, &claim_actor(&fault_user.id), &fault_user.id, Hlc { physical_ms: 4, logical: 0 }).await, Err(DirectoryError::Backend(_))));
    assert_eq!(directory.head_seq().await.expect("head after projection fault"), fault_head);
    assert_eq!(directory.list_invites("default").await.expect("fault invite row").into_iter().find(|record| record.id == fault_invite.record.id).expect("fault invite").accepted_at, None);
    assert_eq!(directory.get_role("default", &fault_user.id).await.expect("fault membership"), None);
    sqlx_core::query::query("DROP TRIGGER hub_test_fail_invite_projection ON hub_space_membership").execute(&directory.pool).await.expect("drop postgres invite failure trigger");
    sqlx_core::query::query("DROP FUNCTION hub_test_fail_invite_projection()").execute(&directory.pool).await.expect("drop postgres invite failure function");
    directory.redeem_invite_atomic(&fault_invite.capability, &claim_actor(&fault_user.id), &fault_user.id, Hlc { physical_ms: 5, logical: 0 }).await.expect("retry rolled back postgres invite");
    let before = directory.list_invites("default").await.expect("postgres invites before rebuild");
    directory.rebuild_projections().await.expect("postgres rebuild");
    let after = directory.list_invites("default").await.expect("postgres invites after rebuild");
    assert_eq!(after, before);
    assert_eq!(directory.get_role("default", &invited.id).await.expect("rebuilt postgres membership"), Some(SpaceRole::Spectator));
}

// 🔬️ Users, spaces, and role-based membership round-trip against a real Postgres.
#[tokio::test]
async fn user_space_membership_round_trip() {
    let (directory, _container) = test_directory().await;
    let mut clock = HubClock::new();
    let user = directory.create_user("a@example.com", "Ada", None, None, None).await.expect("create user");
    let space_id = seed_space(&directory, &mut clock, &user.id, DirectorySpaceKind::Studio).await;
    assert_eq!(directory.get_role(&space_id, &user.id).await.unwrap(), Some(SpaceRole::Author));
}

// 🔬️ Schema bootstrap + seed grow a default space the seed user authors — proves the DDL
// (`hub_space`/`hub_space_membership`, `author`/`spectator` role CHECK) matches this crate's
// own queries against a real Postgres instance.
#[tokio::test]
async fn seed_creates_default_space_and_membership() {
    let (directory, _container) = test_directory().await;
    directory.seed().await.expect("seed");
    let space = directory.get_space("default").await.unwrap().expect("default space");
    assert_eq!(space.kind, "studio");
    assert_eq!(space.visibility, "private");
    assert_eq!(directory.get_role("default", "seed").await.unwrap(), Some(SpaceRole::Author));
}

// 🔬️ Event log replay reproduces the same projections against a real Postgres, mirroring the
// sqlite backend's `event_log_replay_matches_projections`.
#[tokio::test]
async fn event_log_replay_matches_projections() {
    let (directory, _container) = test_directory().await;
    directory.seed().await.expect("seed");
    let head = directory.head_seq().await.expect("head seq");
    let before = directory.get_space("default").await.unwrap();
    let replayed = directory.rebuild_projections().await.expect("rebuild");
    assert_eq!(replayed, head);
    assert_eq!(directory.get_space("default").await.unwrap(), before);
}

#[tokio::test]
async fn directory_event_page_v1_append_admission_is_transactional_postgres() {
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
