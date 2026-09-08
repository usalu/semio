use super::*;

#[derive(serde::Deserialize)]
struct ShareTokenVectors {
    encoding: Vec<ShareTokenEncodingVector>,
    scope: ShareTokenScopeVector,
}

#[derive(serde::Deserialize)]
struct ShareTokenEncodingVector {
    bytes: Vec<u8>,
    hex: String,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct ShareTokenScope {
    space_id: String,
    document_id: String,
}

#[derive(serde::Deserialize)]
struct ShareTokenScopeVector {
    grant: ShareTokenScope,
    allowed: ShareTokenScope,
    denied: ShareTokenScope,
}

fn actor(id: &str) -> DirectoryActor {
    DirectoryActor { kind: DirectoryActorKind::User, id: id.to_string() }
}

#[tokio::test]
async fn document_index_neutral_transactions_survive_projection_rebuild() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/📇️document-index-v1/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let directory = SqliteDirectory::connect(":memory:").await.unwrap();
        let mut clock = HubClock::new();
        directory
            .append_events(&[NewDirectoryEvent {
                hlc: clock.tick(),
                actor: actor("user-fixture"),
                space_id: None,
                user_id: Some("user-fixture".into()),
                body: DirectoryEventBody::UserCreated { user_id: "user-fixture".into(), email: "fixture@example.test".into(), display_name: "Fixture".into() },
            }])
            .await
            .unwrap();
        let events: Vec<DirectoryEvent> = directory::os_pack::json::from_json_str(&row["events"].to_string()).unwrap();
        let mut ids = std::collections::BTreeSet::new();
        let events: Vec<_> = events.into_iter().filter(|event| ids.insert(event.id.clone())).map(|event| NewDirectoryEvent { hlc: event.hlc, actor: event.actor, space_id: event.space_id, user_id: event.user_id, body: event.body }).collect();
        let result = directory.append_events(&events).await;
        assert_eq!(result.is_ok(), row["backendAccepted"].as_bool().unwrap(), "{}: {result:?}", row["id"]);
        let payloads = || {
            let conn = directory.lock().unwrap();
            let mut query = conn.prepare("SELECT payload FROM hub_document_index ORDER BY space_id, document_id").unwrap();
            query.query_map([], |row| row.get::<_, String>(0)).unwrap().collect::<Result<Vec<_>, _>>().unwrap()
        };
        let before = payloads();
        if let Ok(persisted) = result {
            assert_eq!(before.len(), 1);
            let indexed = persisted.iter().find(|event| matches!(event.body, DirectoryEventBody::DocumentIndexed { .. })).unwrap();
            let independent: serde_json::Value = serde_json::from_str(&before[0]).unwrap();
            assert_eq!(independent["createdAtMs"], indexed.recorded_at_ms);
            assert_eq!(independent["createdBy"], "user-fixture");
            assert_eq!(directory.head_seq().await.unwrap(), 1 + events.len() as u64);
            assert_eq!(directory.rebuild_projections().await.unwrap(), directory.head_seq().await.unwrap());
            assert_eq!(payloads(), before, "{}: immutable row survives replay", row["id"]);
            let index = events.iter().find(|event| matches!(event.body, DirectoryEventBody::DocumentIndexed { .. })).unwrap().clone();
            for body in [DirectoryEventBody::SpaceArchived { space_id: "space-fixture".into() }, DirectoryEventBody::SpaceDeleted { space_id: "space-fixture".into() }] {
                directory.append_events(&[NewDirectoryEvent { hlc: clock.tick(), actor: actor("user-fixture"), space_id: Some("space-fixture".into()), user_id: Some("user-fixture".into()), body }]).await.unwrap();
                let head = directory.head_seq().await.unwrap();
                assert!(directory.append_events(std::slice::from_ref(&index)).await.is_err());
                assert_eq!(directory.head_seq().await.unwrap(), head);
            }
            assert!(payloads().is_empty());
        } else {
            assert!(before.is_empty());
            assert_eq!(directory.head_seq().await.unwrap(), 1, "{}: whole batch rolls back", row["id"]);
            assert!(directory.get_space("space-fixture").await.unwrap().is_none());
        }
    }
    println!("[DEBUG] SQLite document index: neutral transactions=10 rollback=8 rebuild=2 archived-refusal=2 deleted-refusal=2");
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

async fn announce_share_document(directory: &SqliteDirectory, clock: &mut HubClock, scope: &DocumentScope, actor_user_id: &str) {
    directory
        .append_events(&[NewDirectoryEvent {
            hlc: clock.tick(),
            actor: actor(actor_user_id),
            space_id: Some(scope.space_id.clone()),
            user_id: Some(actor_user_id.to_string()),
            body: DirectoryEventBody::DocumentAnnounced { descriptor: share_descriptor(scope) },
        }])
        .await
        .expect("announce share document");
}

/// 🌱️ `create_space`/`upsert_membership` were removed (writes now go through
/// `append_events` — see the module root's `//#region 🔖️Decider`); this recreates just enough
/// of a `create-space` decision by hand so backend tests do not need a full `DirectoryService`.
async fn seed_space(dir: &SqliteDirectory, clock: &mut HubClock, owner_user_id: &str, kind: DirectorySpaceKind) -> String {
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

// 🔬️ Users, spaces, and role-based membership round-trip over the event log.
#[tokio::test]
async fn user_space_membership_round_trip() {
    let directory = SqliteDirectory::connect(":memory:").await.expect("connect");
    let mut clock = HubClock::new();
    let user = directory.create_user("a@example.com", "Ada", None, None, None).await.expect("create user");
    let space_id = seed_space(&directory, &mut clock, &user.id, DirectorySpaceKind::Studio).await;
    assert_eq!(directory.get_role(&space_id, &user.id).await.unwrap(), Some(SpaceRole::Author));

    let member = directory.create_user("b@example.com", "Bob", None, None, None).await.expect("create user 2");
    directory
        .append_events(&[NewDirectoryEvent {
            hlc: clock.tick(),
            actor: actor(&user.id),
            space_id: Some(space_id.clone()),
            user_id: Some(member.id.clone()),
            body: DirectoryEventBody::MemberUpserted { space_id: space_id.clone(), user_id: member.id.clone(), role: DirectorySpaceRole::Spectator },
        }])
        .await
        .expect("add member");
    assert_eq!(directory.get_role(&space_id, &member.id).await.unwrap(), Some(SpaceRole::Spectator));
    let spaces = directory.list_spaces_for_user(&member.id).await.unwrap();
    assert_eq!(spaces.len(), 1);

    directory
        .append_events(&[NewDirectoryEvent {
            hlc: clock.tick(),
            actor: actor(&user.id),
            space_id: Some(space_id.clone()),
            user_id: Some(member.id.clone()),
            body: DirectoryEventBody::MemberRemoved { space_id: space_id.clone(), user_id: member.id.clone() },
        }])
        .await
        .expect("remove member");
    assert_eq!(directory.get_role(&space_id, &member.id).await.unwrap(), None);
}

// 🔬️ SyncSession open/close is durable, listable, filterable by space, and boot-time cleanup
// closes every still-open session at once.
#[tokio::test]
async fn sync_session_lifecycle() {
    let directory = SqliteDirectory::connect(":memory:").await.expect("connect");
    directory.seed().await.expect("seed");
    let session = directory.record_sync_session_open(None, 0, "s.space@1/*#editor", "default", "default", "default", None, None, None, "test-client").await.expect("open");
    assert!(session.disconnected_at.is_none());
    assert_eq!(directory.list_active_sync_sessions(Some("default"), ACTIVE_SYNC_SESSION_READ_MAX).await.unwrap().len(), 1);

    directory.record_sync_session_close(&session.id).await.expect("close");
    let sessions = directory.list_sync_sessions_for_document("default").await.unwrap();
    assert_eq!(sessions.len(), 1);
    assert!(sessions[0].disconnected_at.is_some());
    assert!(directory.list_active_sync_sessions(Some("default"), ACTIVE_SYNC_SESSION_READ_MAX).await.unwrap().is_empty());

    directory.record_sync_session_open(None, 0, "s.space@1/*#viewer", "default", "default", "default", None, None, None, "test-client-2").await.expect("open 2");
    directory.close_all_sync_sessions().await.expect("close all");
    assert!(directory.list_active_sync_sessions(None, ACTIVE_SYNC_SESSION_READ_MAX).await.unwrap().is_empty());
}

// 🔬️ Language-neutral vectors validate owned hex encoding against SQLite's independent
// `hex()` oracle and describe the cross-space authorization boundary shared by every backend.
#[test]
fn share_token_vectors_match_sqlite_hex_oracle() {
    let vectors: ShareTokenVectors = serde_json::from_str(include_str!("../../../🧪️tests/🔑️share-token-vectors.json")).expect("share-token vectors");
    let oracle = Connection::open_in_memory().expect("sqlite oracle");
    for vector in vectors.encoding {
        let actual = crate::directory::encode_capability_bytes(&vector.bytes);
        let sqlite_hex: String = oracle.query_row("SELECT lower(hex(?1))", rusqlite::params![vector.bytes], |row| row.get(0)).expect("sqlite hex");
        assert_eq!(actual, vector.hex);
        assert_eq!(actual, sqlite_hex);
    }
}

// 🔬️ Share grants are private by default, space/document scoped, revocable, and expiring.
#[tokio::test]
async fn share_token_lifecycle_and_scope() {
    let directory = SqliteDirectory::connect(":memory:").await.expect("connect");
    directory.seed().await.expect("seed");
    let mut clock = HubClock::new();
    let vectors: ShareTokenVectors = serde_json::from_str(include_str!("../../../🧪️tests/🔑️share-token-vectors.json")).expect("share-token vectors");
    let grant_scope = DocumentScope::new(vectors.scope.grant.space_id, vectors.scope.grant.document_id);
    let allowed_scope = DocumentScope::new(vectors.scope.allowed.space_id, vectors.scope.allowed.document_id);
    let denied_scope = DocumentScope::new(vectors.scope.denied.space_id, vectors.scope.denied.document_id);
    announce_share_document(&directory, &mut clock, &grant_scope, "seed").await;

    let grant = directory.issue_share_token(&grant_scope, 60, "share-lifecycle").await.expect("mint token");
    assert!(grant.capability.expose_once().starts_with("share.v1."));
    assert!(directory.authenticate_share(&allowed_scope, &grant.capability).await.unwrap());
    assert!(!directory.authenticate_share(&denied_scope, &grant.capability).await.unwrap());

    directory.revoke_share_token(&grant_scope, &grant.record.id, "test-revoke", "share-lifecycle").await.expect("revoke token");
    assert!(!directory.authenticate_share(&allowed_scope, &grant.capability).await.unwrap());
    assert!(matches!(directory.revoke_share_token(&grant_scope, &grant.record.id, "test-revoke", "share-lifecycle").await, Err(DirectoryError::NotFound(_))));

    let expiring_scope = DocumentScope::new("default", "expiring");
    announce_share_document(&directory, &mut clock, &expiring_scope, "seed").await;
    let expiring = directory.issue_share_token(&expiring_scope, 60, "share-expiry").await.expect("mint expiring token");
    directory.lock().unwrap().execute("UPDATE hub_share_grant SET expires_at = ?2 WHERE id = ?1", rusqlite::params![expiring.record.id, now_ms() - 1]).unwrap();
    assert!(!directory.authenticate_share(&expiring_scope, &expiring.capability).await.unwrap());
    assert!(matches!(directory.issue_share_token(&expiring_scope, 0, "share-invalid").await, Err(DirectoryError::Conflict(_))));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn share_issuance_atomically_requires_the_persisted_scope_and_preserves_archived_spectator_read() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/🔐️share-issuance-atomicity/🔣️.json")).expect("share issuance fixture");
    assert_eq!(fixture["cases"].as_array().expect("share cases").len(), 7);
    assert_eq!(fixture["sourceHostiles"].as_array().expect("share hostiles").len(), 7);
    let directory = SqliteDirectory::connect(":memory:").await.expect("connect");
    directory.seed().await.expect("seed");
    let mut clock = HubClock::new();
    let live = DocumentScope::new("default", "share-atomic-live");
    announce_share_document(&directory, &mut clock, &live, "seed").await;
    let issued = directory.issue_share_token_as(&live, 60, Some("seed"), "share-atomic-live").await.expect("live share");
    assert!(directory.authenticate_share(&live, &issued.capability).await.expect("live auth"));

    let archived_space = seed_space(&directory, &mut clock, "seed", DirectorySpaceKind::Archive).await;
    let archived = DocumentScope::new(&archived_space, "share-atomic-archive");
    announce_share_document(&directory, &mut clock, &archived, "seed").await;
    assert_eq!(directory.get_role(&archived_space, "seed").await.expect("archived role"), Some(SpaceRole::Spectator));
    let archived_share = directory.issue_share_token_as(&archived, 60, Some("seed"), "share-atomic-archive").await.expect("archived spectator read share");
    assert!(directory.authenticate_share(&archived, &archived_share.capability).await.expect("archived share auth"));

    let before: (i64, i64) = directory
        .lock()
        .expect("precondition inspection")
        .query_row("SELECT (SELECT count(*) FROM hub_share_grant), (SELECT count(*) FROM hub_auth_audit WHERE event_kind = 'share-issued')", [], |row| Ok((row.get(0)?, row.get(1)?)))
        .expect("precondition counts");
    for scope in [DocumentScope::new("missing-space", "missing-document"), DocumentScope::new("default", "missing-document"), DocumentScope::new(&archived_space, "foreign-document")] {
        assert!(matches!(directory.issue_share_token_as(&scope, 60, Some("seed"), "share-atomic-denied").await, Err(DirectoryError::NotFound(_))));
    }
    let after: (i64, i64) = directory
        .lock()
        .expect("postcondition inspection")
        .query_row("SELECT (SELECT count(*) FROM hub_share_grant), (SELECT count(*) FROM hub_auth_audit WHERE event_kind = 'share-issued')", [], |row| Ok((row.get(0)?, row.get(1)?)))
        .expect("postcondition counts");
    assert_eq!(after, before);

    let root = std::env::var_os("SEMIO_TEST_ARTIFACT_DIR").map_or_else(std::env::temp_dir, std::path::PathBuf::from);
    std::fs::create_dir_all(&root).expect("share race artifact root");
    let path = root.join(format!("share-issuance-race-{}.sqlite", time_ordered_id()));
    let issuer = SqliteDirectory::connect(path.to_str().expect("share race path")).await.expect("race issuer");
    issuer.seed().await.expect("race seed");
    let race = DocumentScope::new("default", "share-race-document");
    announce_share_document(&issuer, &mut clock, &race, "seed").await;
    let deletion = Connection::open(&path).expect("independent deletion writer");
    deletion.busy_timeout(std::time::Duration::from_secs(2)).expect("deletion timeout");
    deletion.execute_batch("PRAGMA foreign_keys = ON; BEGIN IMMEDIATE; DELETE FROM hub_space WHERE id = 'default';").expect("hold independent deletion");
    let (started_tx, started_rx) = std::sync::mpsc::sync_channel(0);
    let issue = std::thread::spawn(move || {
        started_tx.send(()).expect("signal issue start");
        tokio::runtime::Builder::new_current_thread().enable_all().build().expect("issue runtime").block_on(issuer.issue_share_token_as(&race, 60, Some("seed"), "share-race"))
    });
    started_rx.recv().expect("issue started");
    std::thread::sleep(std::time::Duration::from_millis(25));
    deletion.execute_batch("COMMIT;").expect("commit independent deletion");
    assert!(matches!(issue.join().expect("join issue"), Err(DirectoryError::NotFound(_))));
    let counts: (i64, i64) = deletion.query_row("SELECT (SELECT count(*) FROM hub_share_grant), (SELECT count(*) FROM hub_auth_audit WHERE correlation_id = 'share-race')", [], |row| Ok((row.get(0)?, row.get(1)?))).expect("race counts");
    assert_eq!(counts, (0, 0));
    drop(deletion);
    std::fs::remove_file(path).expect("remove share race database");
}

#[tokio::test]
async fn socket_binding_reads_are_exact_id_generation_selector_scope_and_status() {
    let directory = SqliteDirectory::connect(":memory:").await.expect("connect");
    directory.seed().await.expect("seed");
    let issue = AuthSessionIssue {
        user_id: "seed".into(),
        identity_provider: "socket-test".into(),
        identity_subject_digest: crate::directory::identity_subject_digest("socket-test", "seed").expect("subject digest"),
        ttl_secs: 60,
        device_instance_id: "socket-device".into(),
        session_kind: AuthSessionKind::DevelopmentLocal,
        correlation_id: "socket-session".into(),
        peer_class: "loopback-test".into(),
    };
    let issued = directory.issue_auth_session(&issue).await.expect("issue session");
    let at_ms = issued.record.issued_at;
    assert_eq!(
        directory.socket_session_binding(&issued.record.id, "seed", issued.record.authorization_generation, None, at_ms).await.expect("session status"),
        SocketSessionBindingStatus::Active { role: None, expires_at_ms: issued.record.expires_at },
    );
    assert_eq!(
        directory.socket_session_binding(&issued.record.id, "seed", issued.record.authorization_generation, Some("default"), at_ms).await.expect("membership status"),
        SocketSessionBindingStatus::Active { role: Some(SpaceRole::Author), expires_at_ms: issued.record.expires_at },
    );
    assert_eq!(directory.socket_session_binding(&issued.record.id, "seed", issued.record.authorization_generation + 1, None, at_ms).await.expect("generation status"), SocketSessionBindingStatus::Revoked,);
    assert_eq!(directory.socket_session_binding(&issued.record.id, "other", issued.record.authorization_generation, None, at_ms).await.expect("user status"), SocketSessionBindingStatus::Unavailable);
    assert_eq!(directory.socket_session_binding(&issued.record.id, "seed", issued.record.authorization_generation, Some("missing"), at_ms).await.expect("lost membership status"), SocketSessionBindingStatus::MembershipLost,);

    let scope = DocumentScope::new("default", "socket-share");
    let mut clock = HubClock::new();
    announce_share_document(&directory, &mut clock, &scope, "seed").await;
    let share = directory.issue_share_token(&scope, 60, "socket-share").await.expect("issue share");
    assert_eq!(directory.socket_share_binding(&share.record.id, share.capability.selector(), &scope, share.record.created_at).await.expect("share status"), SocketShareBindingStatus::Active { expires_at_ms: share.record.expires_at },);
    assert_eq!(directory.socket_share_binding("wrong", share.capability.selector(), &scope, share.record.created_at).await.expect("share id status"), SocketShareBindingStatus::Unavailable);
    assert_eq!(directory.socket_share_binding(&share.record.id, "00", &scope, share.record.created_at).await.expect("share selector status"), SocketShareBindingStatus::Unavailable);
    assert_eq!(directory.socket_share_binding(&share.record.id, share.capability.selector(), &DocumentScope::new("default", "other"), share.record.created_at).await.expect("share scope status"), SocketShareBindingStatus::Unavailable,);
    directory.revoke_share_token(&scope, &share.record.id, "socket-revoke", "socket-share").await.expect("revoke share");
    assert_eq!(directory.socket_share_binding(&share.record.id, share.capability.selector(), &scope, share.record.created_at).await.expect("revoked share status"), SocketShareBindingStatus::Revoked);

    directory.lock().expect("sqlite lock").execute("UPDATE hub_auth_session SET expires_at = ?2 WHERE id = ?1", rusqlite::params![issued.record.id, at_ms]).expect("expire session");
    assert_eq!(directory.socket_session_binding(&issued.record.id, "seed", issued.record.authorization_generation, None, at_ms).await.expect("expired session status"), SocketSessionBindingStatus::Expired);
    let expired_share = directory.issue_share_token(&scope, 60, "socket-expiry").await.expect("issue expiring share");
    directory.lock().expect("sqlite lock").execute("UPDATE hub_share_grant SET expires_at = ?2 WHERE id = ?1", rusqlite::params![expired_share.record.id, at_ms]).expect("expire share");
    assert_eq!(directory.socket_share_binding(&expired_share.record.id, expired_share.capability.selector(), &scope, at_ms).await.expect("expired share status"), SocketShareBindingStatus::Expired);
}

#[tokio::test]
async fn auth_session_storage_is_digest_only_and_revoke_returns_generation() {
    let directory = SqliteDirectory::connect(":memory:").await.expect("connect");
    directory.seed().await.expect("seed");
    let issue = AuthSessionIssue {
        user_id: "seed".into(),
        identity_provider: "oidc.example".into(),
        identity_subject_digest: crate::directory::identity_subject_digest("oidc.example", "sub-123").expect("subject digest"),
        ttl_secs: 60,
        device_instance_id: "device-a".into(),
        session_kind: AuthSessionKind::External,
        correlation_id: "issue-correlation".into(),
        peer_class: "loopback-test".into(),
    };
    let issued = directory.issue_auth_session(&issue).await.expect("issue session");
    let share_scope = DocumentScope::new("default", "auth-storage-test");
    let mut clock = HubClock::new();
    announce_share_document(&directory, &mut clock, &share_scope, "seed").await;
    let share = directory.issue_share_token(&share_scope, 60, "share-storage").await.expect("issue share");
    let invite = directory.issue_invite("default", SpaceRole::Spectator, 60, "invite-storage").await.expect("issue invite");
    let raw = issued.capability.expose_once();
    let secret_hex = raw.rsplit('.').next().expect("raw secret");
    let (selector, digest_hex): (String, String) =
        directory.lock().expect("sqlite lock").query_row("SELECT selector, lower(hex(secret_digest)) FROM hub_auth_session WHERE id = ?1", [&issued.record.id], |row| Ok((row.get(0)?, row.get(1)?))).expect("stored session authority");
    assert_eq!(selector, issued.capability.selector());
    assert_eq!(digest_hex, crate::directory::encode_capability_bytes(&issued.capability.secret_digest()));
    assert_ne!(digest_hex, secret_hex);
    assert!(!format!("{:?}", issued.capability).contains(secret_hex));
    let share_raw = share.capability.expose_once();
    let invite_raw = invite.capability.expose_once();
    let conn = directory.lock().expect("sqlite storage inspection");
    for (table, id, raw_capability) in [("hub_share_grant", &share.record.id, &share_raw), ("hub_space_invite", &invite.record.id, &invite_raw)] {
        let sql = format!("SELECT selector, lower(hex(secret_digest)) FROM {table} WHERE id = ?1");
        let (stored_selector, stored_digest): (String, String) = conn.query_row(&sql, [id], |row| Ok((row.get(0)?, row.get(1)?))).expect("stored scoped capability");
        assert!(raw_capability.contains(&stored_selector));
        assert!(!raw_capability.ends_with(&stored_digest));
    }
    drop(conn);
    assert_eq!(directory.authenticate_session(&issued.capability).await.expect("authenticate").expect("active").authorization_generation, 1);

    let revoked = directory.revoke_auth_sessions_for_user("seed", "security-test", Some("seed"), "revoke-correlation").await.expect("revoke user sessions");
    assert_eq!(revoked.len(), 1);
    assert_eq!(revoked[0].id, issued.record.id);
    assert_eq!(revoked[0].authorization_generation, 2);
    assert!(directory.authenticate_session(&issued.capability).await.expect("authenticate revoked").is_none());
    let audit = directory.list_auth_audit(AUTH_AUDIT_PAGE_MAX, 0).await.expect("auth audit");
    let audit_text = format!("{audit:?}");
    assert!(!audit_text.contains(&raw));
    assert!(!audit_text.contains(secret_hex));
    assert!(audit.iter().any(|entry| entry.event_kind == "session-issued"));
    assert!(audit.iter().any(|entry| entry.event_kind == "session-revoked" && entry.reason_code.as_deref() == Some("security-test")));
}

#[tokio::test]
async fn projection_rebuild_preserves_live_credential_invite_and_session_bindings() {
    let directory = SqliteDirectory::connect(":memory:").await.expect("connect");
    directory.seed().await.expect("seed");
    let issued = directory
        .issue_auth_session(&AuthSessionIssue {
            user_id: "seed".into(),
            identity_provider: "rebuild-test".into(),
            identity_subject_digest: crate::directory::identity_subject_digest("rebuild-test", "seed").expect("subject digest"),
            ttl_secs: 60,
            device_instance_id: "rebuild-device".into(),
            session_kind: AuthSessionKind::DevelopmentLocal,
            correlation_id: "rebuild-session".into(),
            peer_class: "loopback-test".into(),
        })
        .await
        .expect("issue session");
    let invite = directory.issue_invite("default", SpaceRole::Spectator, 60, "rebuild-invite").await.expect("issue invite");
    let sync = directory
        .record_sync_session_open(Some(&issued.record.id), issued.record.authorization_generation, "s.default@1/default#admin", "default", "default", "admin", Some("seed"), Some("seed@localhost"), Some(SpaceRole::Author), "rebuild-client")
        .await
        .expect("open sync session");

    assert_eq!(directory.rebuild_projections().await.expect("rebuild"), 3);
    assert_eq!(directory.authenticate_session(&issued.capability).await.expect("authenticate").expect("live session").id, issued.record.id);
    assert!(directory.list_invites("default").await.expect("invites").iter().any(|record| record.id == invite.record.id));
    let sessions = directory.list_active_sync_sessions(Some("default"), ACTIVE_SYNC_SESSION_READ_MAX).await.expect("sync sessions");
    let restored = sessions.iter().find(|record| record.id == sync.id).expect("restored sync session");
    assert_eq!(restored.auth_session_id.as_deref(), Some(issued.record.id.as_str()));
    assert_eq!(restored.user_id.as_deref(), Some("seed"));
}

// 🔬️ `seed()` (now event-sourced) still leaves a dense, replayable log.
#[tokio::test]
async fn seed_is_replayable() {
    let directory = SqliteDirectory::connect(":memory:").await.expect("connect");
    directory.seed().await.expect("seed");
    assert_eq!(directory.head_seq().await.unwrap(), 3);
    let before = directory.get_space("default").await.unwrap();
    let replayed = directory.rebuild_projections().await.expect("rebuild");
    assert_eq!(replayed, 3);
    assert_eq!(directory.get_space("default").await.unwrap(), before);
}

#[tokio::test]
async fn directory_event_page_v1_append_admission_is_transactional_sqlite() {
    let directory = SqliteDirectory::connect(":memory:").await.expect("connect");
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
