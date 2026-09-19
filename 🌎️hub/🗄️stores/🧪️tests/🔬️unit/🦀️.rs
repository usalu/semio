
use super::*;
use server::conformance;
use server::gateway::Server;

/// 📁️ A fresh, empty directory under the OS temp dir, named after the calling law.
fn scratch(name: &str) -> PathBuf {
    let stamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |elapsed| elapsed.as_nanos());
    let dir = std::env::temp_dir().join(format!("semio-hub-stores-{name}-{stamp}"));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("a scratch directory");
    dir
}

/// 🗄️ The four stores of a durable hub instance opened under `dir`.
async fn durable(dir: &Path) -> InstanceStores<HubInstance> {
    HubInstance::open(&StorageProfile::Embedded { data_dir: dir.display().to_string() }).await.expect("the hub instance opens its stores")
}

/// 🫧️ The four stores of an ephemeral hub instance.
async fn ephemeral() -> InstanceStores<HubInstance> {
    HubInstance::open(&StorageProfile::Ephemeral).await.expect("the hub instance opens its stores")
}

//#region 🔖️Conformance
#[tokio::test]
async fn receipt_round_trips_and_is_idempotent() {
    conformance::receipt_round_trips_and_is_idempotent(&mut ephemeral().await.authority).await;
    let dir = scratch("receipt");
    conformance::receipt_round_trips_and_is_idempotent(&mut durable(&dir).await.authority).await;
}

#[tokio::test]
async fn append_events_rejects_a_sequence_gap_and_writes_nothing() {
    conformance::append_events_rejects_a_sequence_gap_and_writes_nothing(&mut ephemeral().await.authority).await;
    let dir = scratch("gap");
    conformance::append_events_rejects_a_sequence_gap_and_writes_nothing(&mut durable(&dir).await.authority).await;
}

#[tokio::test]
async fn events_since_returns_only_later_events_of_that_actor() {
    conformance::events_since_returns_only_later_events_of_that_actor(&mut ephemeral().await.authority).await;
    let dir = scratch("since");
    conformance::events_since_returns_only_later_events_of_that_actor(&mut durable(&dir).await.authority).await;
}

#[tokio::test]
async fn snapshots_only_move_forward() {
    conformance::snapshots_only_move_forward(&mut ephemeral().await.authority).await;
    let dir = scratch("snapshot");
    conformance::snapshots_only_move_forward(&mut durable(&dir).await.authority).await;
}

#[tokio::test]
async fn outbox_delivers_each_entry_exactly_once() {
    conformance::outbox_delivers_each_entry_exactly_once(&mut ephemeral().await.authority).await;
    let dir = scratch("outbox");
    conformance::outbox_delivers_each_entry_exactly_once(&mut durable(&dir).await.authority).await;
}

#[tokio::test]
async fn lease_epoch_bump_fences_out_the_previous_holder() {
    conformance::lease_epoch_bump_fences_out_the_previous_holder(&mut ephemeral().await.authority).await;
    let dir = scratch("lease");
    conformance::lease_epoch_bump_fences_out_the_previous_holder(&mut durable(&dir).await.authority).await;
}

#[tokio::test]
async fn projection_list_is_prefix_scoped_and_key_ordered() {
    conformance::projection_list_is_prefix_scoped_and_key_ordered(&mut ephemeral().await.projections).await;
    let dir = scratch("projection-list");
    conformance::projection_list_is_prefix_scoped_and_key_ordered(&mut durable(&dir).await.projections).await;
}

#[tokio::test]
async fn clearing_a_projection_resets_it_for_rebuild() {
    conformance::clearing_a_projection_resets_it_for_rebuild(&mut ephemeral().await.projections).await;
    let dir = scratch("projection-clear");
    conformance::clearing_a_projection_resets_it_for_rebuild(&mut durable(&dir).await.projections).await;
}

#[tokio::test]
async fn blob_put_get_and_has_are_content_addressed() {
    conformance::blob_put_get_and_has_are_content_addressed(&mut ephemeral().await.blobs).await;
    let dir = scratch("blob");
    conformance::blob_put_get_and_has_are_content_addressed(&mut durable(&dir).await.blobs).await;
}

#[tokio::test]
async fn revoke_principal_removes_every_session_of_that_principal() {
    conformance::revoke_principal_removes_every_session_of_that_principal(&mut ephemeral().await.sessions).await;
    let dir = scratch("session");
    conformance::revoke_principal_removes_every_session_of_that_principal(&mut durable(&dir).await.sessions).await;
}

#[tokio::test]
async fn a_saga_reacts_to_a_committed_event_exactly_once_across_a_restart() {
    conformance::a_saga_reacts_to_a_committed_event_exactly_once_across_a_restart(&mut ephemeral().await.authority, || async { ephemeral().await.authority }).await;
    let dir = scratch("saga-outbox");
    conformance::a_saga_reacts_to_a_committed_event_exactly_once_across_a_restart(&mut durable(&dir).await.authority, || async { durable(&dir).await.authority }).await;
}
//#endregion 🔖️Conformance

//#region 🔖️Fault
/// 🧨️ A journal sink that refuses every write: the store's own log file, reopened **read-only**.
///
/// A real handle failing at the OS boundary rather than a flag, because the contract under test is
/// exactly "what a durable backend does when its disk says no". The path must exist; `create` makes
/// it if it does not, so an ephemeral store can be given a failing sink too — a store with no sink
/// cannot fail a write, and the law is about the failure, not about the absence.
fn failing_sink(path: &Path) -> tokio::fs::File {
    let _ = std::fs::OpenOptions::new().create(true).append(true).open(path);
    tokio::fs::File::from_std(std::fs::File::open(path).expect("a read-only handle on the journal"))
}

/// 📄️ A path that is a regular file, so every attempt to write a child path under it fails at the
/// filesystem — how a session store whose directory is not a directory is expressed.
fn blocked_dir(dir: &Path) -> PathBuf {
    let path = dir.join("not-a-directory");
    std::fs::write(&path, b"").expect("a regular file where a directory is expected");
    path
}

#[tokio::test]
async fn a_projection_write_reports_a_failing_sink() {
    let dir = scratch("projection-fault");
    let mut volatile = ephemeral().await.projections;
    conformance::seed_projection_fault_fixture(&mut volatile).await;
    volatile.journal.sink = Some(failing_sink(&dir.join("volatile.jsonl")));
    conformance::a_projection_write_reports_a_failing_sink(&mut volatile).await;

    let mut persisted = durable(&dir).await.projections;
    conformance::seed_projection_fault_fixture(&mut persisted).await;
    persisted.journal.sink = Some(failing_sink(&dir.join("projections/log.jsonl")));
    conformance::a_projection_write_reports_a_failing_sink(&mut persisted).await;
    drop(persisted);
    assert_eq!(durable(&dir).await.projections.get("members", "space/a").await, Some(vec![1]), "the refused write must be absent from the reopened journal too");
}

#[tokio::test]
async fn a_session_write_reports_a_failing_sink() {
    let dir = scratch("session-fault");
    let mut volatile = ephemeral().await.sessions;
    conformance::seed_session_fault_fixture(&mut volatile).await;
    volatile.dir = Some(blocked_dir(&dir));
    conformance::a_session_write_reports_a_failing_sink(&mut volatile).await;

    let mut persisted = durable(&dir).await.sessions;
    conformance::seed_session_fault_fixture(&mut persisted).await;
    persisted.dir = Some(blocked_dir(&dir));
    conformance::a_session_write_reports_a_failing_sink(&mut persisted).await;
    drop(persisted);
    assert!(durable(&dir).await.sessions.get(&SessionId("s1".into())).await.is_some(), "a session the store could not remove is still on disk, which is why the caller was told");
}
//#endregion 🔖️Fault

//#region 🔖️Profile
#[tokio::test]
async fn the_profile_decides_whether_a_store_is_durable_at_all() {
    let volatile = ephemeral().await;
    assert!(!volatile.authority.is_durable());
    assert!(!volatile.projections.is_durable());
    assert!(!volatile.blobs.is_durable());
    assert!(!volatile.sessions.is_durable());
    let dir = scratch("profile");
    let persisted = durable(&dir).await;
    assert!(persisted.authority.is_durable());
    assert!(persisted.projections.is_durable());
    assert!(persisted.blobs.is_durable());
    assert!(persisted.sessions.is_durable());
    assert!(dir.join("authority/log.jsonl").exists());
    assert!(dir.join("projections/log.jsonl").exists());
    assert!(dir.join("blobs").is_dir());
    assert!(dir.join("sessions").is_dir());
}

#[tokio::test]
async fn an_ephemeral_instance_keeps_nothing_for_the_next_open() {
    let actor = conformance::actor_key("doc-1");
    let mut first = ephemeral().await;
    first.authority.append_events(&actor, &[conformance::event_record(&actor, 1)], &[]).await.expect("appended");
    assert_eq!(first.authority.last_seq(&actor).await.unwrap(), 1);
    let second = ephemeral().await;
    assert_eq!(second.authority.last_seq(&actor).await.unwrap(), 0);
}
//#endregion 🔖️Profile

//#region 🔖️Restart
#[tokio::test]
async fn a_durable_authority_reopens_at_the_frontier_it_was_left_at() {
    let dir = scratch("restart-authority");
    let actor = conformance::actor_key("doc-1");
    let other = conformance::actor_key("doc-2");
    {
        let mut stores = durable(&dir).await;
        stores.authority.append_events(&actor, &[conformance::event_record(&actor, 1), conformance::event_record(&actor, 2)], &[OutboxEntry::pending(actor.clone(), conformance::event_record(&actor, 2))]).await.expect("appended");
        stores.authority.append_events(&other, &[conformance::event_record(&other, 1)], &[]).await.expect("appended");
        stores.authority.record_receipt(&IdempotencyKey("k1".into()), &conformance::receipt_for(&actor, 2)).await.expect("recorded");
        stores.authority.put_snapshot(&actor, Revision(2), vec![9, 9]).await.expect("snapshotted");
        stores.authority.acquire_lease(&actor, "node-a").await.expect("leased");
        assert_eq!(stores.authority.last_seq(&actor).await.unwrap(), 2);
    }

    let mut stores = durable(&dir).await;
    assert_eq!(stores.authority.last_seq(&actor).await.unwrap(), 2);
    assert_eq!(stores.authority.last_seq(&other).await.unwrap(), 1);
    assert_eq!(stores.authority.events_since(&actor, 0).await.unwrap(), vec![conformance::event_record(&actor, 1), conformance::event_record(&actor, 2)]);
    assert_eq!(stores.authority.receipt(&IdempotencyKey("k1".into())).await.unwrap(), Some(conformance::receipt_for(&actor, 2)));
    assert_eq!(stores.authority.snapshot(&actor).await.unwrap(), Some((Revision(2), vec![9, 9])));
    assert_eq!(stores.authority.pending_outbox(10).await.unwrap().iter().map(|entry| entry.id).collect::<Vec<_>>(), vec![1]);
    assert!(stores.authority.validate_lease(&actor, &Lease { epoch: 1, holder: "node-a".into() }).await);
    assert_eq!(stores.authority.append_events(&actor, &[conformance::event_record(&actor, 2)], &[]).await, Err(StorageError::SequenceGap { expected: 3, got: 2 }));
    stores.authority.append_events(&actor, &[conformance::event_record(&actor, 3)], &[]).await.expect("the reopened stream continues");
    assert_eq!(stores.authority.last_seq(&actor).await.unwrap(), 3);
}

#[tokio::test]
async fn a_durable_outbox_reopens_with_its_acknowledgements_intact() {
    let dir = scratch("restart-outbox");
    let actor = conformance::actor_key("doc-1");
    {
        let mut stores = durable(&dir).await;
        stores.authority.enqueue_outbox(vec![OutboxEntry::pending(actor.clone(), conformance::event_record(&actor, 1)), OutboxEntry::pending(actor.clone(), conformance::event_record(&actor, 2))]).await.expect("enqueued");
        stores.authority.mark_outbox_delivered(&[1]).await.expect("acknowledged");
    }
    let mut stores = durable(&dir).await;
    assert_eq!(stores.authority.pending_outbox(10).await.unwrap().iter().map(|entry| entry.id).collect::<Vec<_>>(), vec![2]);
    stores.authority.enqueue_outbox(vec![OutboxEntry::pending(actor.clone(), conformance::event_record(&actor, 3))]).await.expect("enqueued");
    assert_eq!(stores.authority.pending_outbox(10).await.unwrap().iter().map(|entry| entry.id).collect::<Vec<_>>(), vec![2, 3]);
}

#[tokio::test]
async fn durable_projections_blobs_and_sessions_survive_a_restart() {
    let dir = scratch("restart-roles");
    let hash = server::storage::content_hash(b"durable bytes");
    let alice = Principal::User { id: "alice".into() };
    {
        let mut stores = durable(&dir).await;
        stores.projections.put("members", "space/a", vec![1]).await.expect("written");
        stores.projections.put("members", "space/b", vec![2]).await.expect("written");
        stores.projections.put("members", "space/a", vec![3]).await.expect("written");
        stores.projections.set_checkpoint("members", 42).await.expect("checkpointed");
        stores.projections.put("stale", "x", vec![0]).await.expect("written");
        stores.projections.clear("stale").await.expect("cleared");
        stores.blobs.put(hash, b"durable bytes").await.expect("stored");
        stores.sessions.create(conformance::session_record("s1", alice.clone())).await.expect("created");
        stores.sessions.create(conformance::session_record("s2", Principal::User { id: "bob".into() })).await.expect("created");
        stores.sessions.delete(&SessionId("s2".into())).await.expect("deleted");
    }

    let mut stores = durable(&dir).await;
    assert_eq!(stores.projections.get("members", "space/a").await, Some(vec![3]));
    assert_eq!(stores.projections.list("members", "space/").await.into_iter().map(|(key, _)| key).collect::<Vec<_>>(), vec!["space/a".to_string(), "space/b".to_string()]);
    assert_eq!(stores.projections.checkpoint("members").await, 42);
    assert!(stores.projections.list("stale", "").await.is_empty());
    assert_eq!(stores.blobs.get(&hash).await, Some(b"durable bytes".to_vec()));
    assert!(stores.blobs.has(&hash).await);
    assert_eq!(stores.sessions.get(&SessionId("s1".into())).await.map(|record| record.principal), Some(alice.clone()));
    assert_eq!(stores.sessions.get(&SessionId("s2".into())).await, None);
    assert_eq!(stores.sessions.revoke_principal(&alice).await.expect("revoked"), 1);

    let stores = durable(&dir).await;
    assert_eq!(stores.sessions.get(&SessionId("s1".into())).await, None);
}

#[tokio::test]
async fn a_projection_journal_is_compacted_to_its_current_state_on_open() {
    let dir = scratch("compaction");
    {
        let mut stores = durable(&dir).await;
        for value in 0..16u8 {
            stores.projections.put("members", "space/a", vec![value]).await.expect("written");
        }
        stores.projections.set_checkpoint("members", 7).await.expect("checkpointed");
    }
    let path = dir.join("projections/log.jsonl");
    assert_eq!(std::fs::read_to_string(&path).expect("readable").lines().count(), 17);
    let stores = durable(&dir).await;
    assert_eq!(std::fs::read_to_string(&path).expect("readable").lines().count(), 2);
    assert_eq!(stores.projections.get("members", "space/a").await, Some(vec![15]));
    assert_eq!(stores.projections.checkpoint("members").await, 7);
}

#[tokio::test]
async fn a_torn_final_record_is_dropped_rather_than_failing_the_open() {
    let dir = scratch("torn-tail");
    let actor = conformance::actor_key("doc-1");
    {
        let mut stores = durable(&dir).await;
        stores.authority.append_events(&actor, &[conformance::event_record(&actor, 1)], &[]).await.expect("appended");
        stores.authority.append_events(&actor, &[conformance::event_record(&actor, 2)], &[]).await.expect("appended");
    }
    let path = dir.join("authority/log.jsonl");
    let intact = std::fs::read_to_string(&path).expect("readable");
    let torn = format!("{intact}{}", &intact.lines().next().expect("a first line")[..20]);
    std::fs::write(&path, torn).expect("written");

    let mut stores = durable(&dir).await;
    assert_eq!(stores.authority.last_seq(&actor).await.unwrap(), 2);
    assert_eq!(std::fs::read_to_string(&path).expect("readable"), intact);
    stores.authority.append_events(&actor, &[conformance::event_record(&actor, 3)], &[]).await.expect("the truncated log keeps accepting");
    assert_eq!(std::fs::read_to_string(&path).expect("readable").lines().count(), 3);
}
//#endregion 🔖️Restart

//#region 🔖️Server
#[tokio::test]
async fn the_hub_instance_builds_a_server_over_its_durable_stores() {
    let dir = scratch("server-build");
    let server = Server::<HubInstance>::builder(StorageProfile::Embedded { data_dir: dir.display().to_string() }).identity("hub", "0.1.0").build().await.expect("the hub instance builds");
    assert_eq!(server.definition().id, "hub");
    assert!(server.definition().modules.is_empty());
    assert_eq!(server.state().profile.data_dir(), Some(dir.display().to_string().as_str()));
    assert!(server.state().blobs.lock().await.is_durable());
    server.state().sessions.lock().await.create(conformance::session_record("s1", Principal::User { id: "alice".into() })).await.expect("created");
    assert_eq!(server.state().sessions.lock().await.get(&SessionId("s1".into())).await.map(|record| record.principal), Some(Principal::User { id: "alice".into() }));
    assert!(dir.join("sessions").read_dir().expect("readable").count() == 1);
}
//#endregion 🔖️Server
