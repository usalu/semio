//! 🧹️ Retained ownership of a detached local presence root and its complete peer roster.

use super::*;

//#region 🧹️StoreRetirement
pub struct PresenceStoreRetirement<P> {
    local: std::mem::ManuallyDrop<Option<Arc<P>>>,
    peers: std::mem::ManuallyDrop<Option<Arc<PresencePeersRoot<P>>>>,
    active_local: std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    active_peers: std::mem::ManuallyDrop<Option<PresencePeersRetirement<P>>>,
    factory: Arc<dyn SnapshotRetirementFactory<P>>,
}

impl<P: Send + Sync + 'static> PresenceStoreRetirement<P> {
    fn new(local: Arc<P>, peers: Arc<PresencePeersRoot<P>>, factory: Arc<dyn SnapshotRetirementFactory<P>>) -> Self {
        Self { local: std::mem::ManuallyDrop::new(Some(local)), peers: std::mem::ManuallyDrop::new(Some(peers)), active_local: std::mem::ManuallyDrop::new(None), active_peers: std::mem::ManuallyDrop::new(None), factory }
    }

    pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, String> {
        if maximum_items == 0 {
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(active) = self.active_local.as_mut() {
            let step = active.close_step(1, maximum_bytes)?;
            return match step {
                SnapshotRetirementStep::Complete => {
                    if !active.terminal_is_empty() {
                        return Err("presence local close reported Complete without its terminal-empty witness".into());
                    }
                    drop(self.active_local.take());
                    Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                }
                SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items > 1 || released_bytes > maximum_bytes => Err("presence local close exceeded its exact grant".into()),
                step => Ok(step),
            };
        }
        if let Some(active) = self.active_peers.as_mut() {
            let step = active.close_step(1, maximum_bytes)?;
            if step != SnapshotRetirementStep::Complete {
                return Ok(step);
            }
            if !active.terminal_is_empty() {
                return Err("presence peer close reported Complete without its terminal-empty witness".into());
            }
            drop(self.active_peers.take());
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(local) = self.local.take() {
            *self.active_local = Some(self.factory.retire(local));
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(peers) = self.peers.take() {
            return match Arc::try_unwrap(peers) {
                Ok(peers) => {
                    let retired = PresencePeersRetiredEntries { entries: std::mem::ManuallyDrop::new(peers.entries), len: peers.len };
                    *self.active_peers = Some(PresencePeersRetirement::new(retired, self.factory.clone()));
                    Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                }
                Err(peers) => {
                    *self.peers = Some(peers);
                    Ok(SnapshotRetirementStep::Blocked)
                }
            };
        }
        Ok(SnapshotRetirementStep::Complete)
    }
}

impl<P> PresenceStoreRetirement<P> {
    pub fn terminal_is_empty(&self) -> bool {
        self.local.is_none() && self.peers.is_none() && self.active_local.is_none() && self.active_peers.is_none()
    }
}

impl<P> Drop for PresenceStoreRetirement<P> {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            assert!(self.terminal_is_empty(), "presence store retirement requires its exact terminal-empty witness");
        }
    }
}

impl<P: Send + Sync + 'static> ErasedSnapshotRetirement for PresenceStoreRetirement<P> {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, String> {
        PresenceStoreRetirement::close_step(self, maximum_items, maximum_bytes)
    }

    fn terminal_is_empty(&self) -> bool {
        PresenceStoreRetirement::terminal_is_empty(self)
    }
}

impl<P: Send + Sync + 'static> PresencePeersCommit<P> {
    pub fn into_retirement(self) -> PresenceStoreRetirement<P> {
        PresenceStoreRetirement {
            local: std::mem::ManuallyDrop::new(None),
            peers: std::mem::ManuallyDrop::new(Some(self.root)),
            active_local: std::mem::ManuallyDrop::new(None),
            active_peers: std::mem::ManuallyDrop::new(self.retirement),
            factory: self.factory,
        }
    }
}
//#endregion 🧹️StoreRetirement

//#region 🔌️OwnerTransfer
impl<P: Clone + Send + Sync + 'static, M: self::Mutation<P>> PresenceStore<P, M> {
    /// 🧹️ Detaches exact roots once after the concrete domain validates its empty terminal value.
    pub fn begin_retirement(
        &mut self,
        terminal_local: Arc<P>,
        terminal_is_empty: fn(&P) -> bool,
        factory: Arc<dyn SnapshotRetirementFactory<P>>,
    ) -> Result<PresenceStoreRetirement<P>, (&'static str, Arc<P>)> {
        if self.close_started || !terminal_is_empty(terminal_local.as_ref()) {
            return Err(("presence close requires a fresh store and an exact empty domain terminal", terminal_local));
        }
        self.close_started = true;
        let local = std::mem::replace(&mut self.local, terminal_local);
        let peers = std::mem::replace(&mut self.peers, Arc::new(PresencePeersRoot::empty()));
        Ok(PresenceStoreRetirement::new(local, peers, factory))
    }

    pub fn retirement_started(&self) -> bool {
        self.close_started
    }
}
//#endregion 🔌️OwnerTransfer

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
    struct Value(i32);

    impl MutationDiff<Value> for Value {
        fn apply(&self, _base: &Value) -> crate::os_spr::MutationApplyResult<Value> { Ok(self.clone()) }
        fn absorb(&mut self, other: Self) { *self = other; }
    }

    #[derive(Clone, serde::Serialize, serde::Deserialize)]
    struct Noop;

    impl Mutation<Value> for Noop {
        type Diff = Value;
        fn diff(&self, base: &Value) -> crate::os_spr::MutationOutcome<Value> { crate::os_spr::MutationOutcome::new(base.clone()) }
        fn inverse(&self, _base: &Value) -> Vec<Self> { vec![Noop] }
    }

    struct Factory(Arc<std::sync::atomic::AtomicUsize>);
    struct Retirement { root: std::mem::ManuallyDrop<Option<Arc<Value>>>, count: Arc<std::sync::atomic::AtomicUsize> }

    impl SnapshotRetirementFactory<Value> for Factory {
        fn retire(&self, root: Arc<Value>) -> Box<dyn ErasedSnapshotRetirement> {
            Box::new(Retirement { root: std::mem::ManuallyDrop::new(Some(root)), count: self.0.clone() })
        }
    }

    impl ErasedSnapshotRetirement for Retirement {
        fn close_step(&mut self, items: usize, _bytes: usize) -> Result<SnapshotRetirementStep, String> {
            if items == 0 { return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
            let Some(root) = self.root.take() else { return Ok(SnapshotRetirementStep::Complete) };
            match Arc::try_unwrap(root) {
                Ok(_) => { self.count.fetch_add(1, std::sync::atomic::Ordering::Relaxed); Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 }) }
                Err(root) => { *self.root = Some(root); Ok(SnapshotRetirementStep::Blocked) }
            }
        }
        fn terminal_is_empty(&self) -> bool { self.root.is_none() }
    }

    fn close_peer_root(mut retirement: PresencePeersRetirement<Value>) -> usize {
        let mut bytes = 0;
        for _ in 0..512 {
            match retirement.close_step(1, 4096).unwrap() {
                SnapshotRetirementStep::Pending { released_items, released_bytes } => { assert!(released_items <= 1 && released_bytes <= 4096); bytes += released_bytes; }
                SnapshotRetirementStep::Complete => { assert!(retirement.terminal_is_empty()); return bytes; }
                SnapshotRetirementStep::Blocked => panic!("peer root has no remaining captured reader"),
            }
        }
        panic!("exact peer root failed bounded terminal progress")
    }

    #[test]
    fn retained_presence_overlapping_rosters_retire_shared_entries_once_across_workers() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧪️retirement.json")).unwrap();
        let law = &fixture["overlap"];
        for race in [false, true] {
            let count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
            let factory = Arc::new(Factory(count.clone()));
            let mut owner = PresenceStore::<Value, Noop>::new(Value(23));
            owner.install_peer_retirement_factory(factory.clone()).unwrap();
            let mut publication = owner.begin_peer_publication().unwrap();
            while publication.prune_one(|_| true).unwrap() {}
            for peer in law["first"].as_array().unwrap() { publication.adopt(peer["actor"].as_str().unwrap().into(), Value(peer["value"].as_i64().unwrap() as i32), 0).ok().unwrap(); }
            while publication.release_created_one() {}
            let commit = publication.take_commit().unwrap();
            assert_eq!(close_peer_root(owner.publish_peer_commit(commit).ok().unwrap().unwrap()), 0);
            let reader = owner.peers_root();
            let mut publication = owner.begin_peer_publication().unwrap();
            while publication.prune_one(|_| true).unwrap() {}
            for peer in law["second"].as_array().unwrap().iter().skip(law["first"].as_array().unwrap().len()) { publication.adopt(peer["actor"].as_str().unwrap().into(), Value(peer["value"].as_i64().unwrap() as i32), 0).ok().unwrap(); }
            while publication.release_created_one() {}
            let mut first = owner.publish_peer_commit(publication.take_commit().unwrap()).ok().unwrap().unwrap();
            let mut publication = owner.begin_peer_publication().unwrap();
            while publication.prune_one(|_| false).unwrap() {}
            let second = owner.publish_peer_commit(publication.take_commit().unwrap()).ok().unwrap().unwrap();
            assert!(!owner.retirement_started());
            assert!(owner.peers_root().is_empty());
            let bytes = if race {
                drop(reader);
                let barrier = Arc::new(std::sync::Barrier::new(2));
                let first_barrier = barrier.clone();
                let first = std::thread::spawn(move || { first_barrier.wait(); close_peer_root(first) });
                let second = std::thread::spawn(move || { barrier.wait(); close_peer_root(second) });
                first.join().unwrap() + second.join().unwrap()
            } else {
                assert_eq!(first.close_step(1, 4096).unwrap(), SnapshotRetirementStep::Blocked);
                let second_bytes = std::thread::spawn(move || close_peer_root(second)).join().unwrap();
                assert_eq!(count.load(std::sync::atomic::Ordering::Relaxed), 1);
                let observed = reader.peers().map(|(actor, value)| serde_json::json!({ "actor": actor, "value": value.0 })).collect::<Vec<_>>();
                assert_eq!(serde_json::to_value(observed).unwrap(), law["first"]);
                drop(reader);
                second_bytes + std::thread::spawn(move || close_peer_root(first)).join().unwrap()
            };
            assert_eq!(serde_json::json!(bytes), law["expectedActorBytes"]);
            assert_eq!(serde_json::json!(count.load(std::sync::atomic::Ordering::Relaxed)), law["expectedPeerSnapshots"]);
            let mut close = owner.begin_retirement(Arc::new(Value(0)), |value| value.0 == 0, factory).ok().unwrap();
            for _ in 0..256 { if close.close_step(1, 4096).unwrap() == SnapshotRetirementStep::Complete { break; } }
            assert!(close.terminal_is_empty());
            assert_eq!(count.load(std::sync::atomic::Ordering::Relaxed), 3);
            eprintln!("[DEBUG] overlapping immutable peer roots retired each payload once across workers race={race}, actor_bytes={bytes}");
        }
    }

    #[test]
    fn retained_presence_read_return_releases_alias_before_cross_worker_reclamation() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧪️retirement.json")).unwrap();
        for variant in fixture["readerReturn"]["variants"].as_array().unwrap() {
            let registry = Arc::new(SnapshotReadLeaseRegistry::new());
            let root = Arc::new(fixture["readerReturn"]["text"].as_str().unwrap().to_string());
            let lease = registry.try_issue(root.clone()).unwrap();
            let variant = variant.as_str().unwrap().to_string();
            let barrier = Arc::new(std::sync::Barrier::new(2));
            let worker_barrier = barrier.clone();
            let worker = std::thread::spawn(move || {
                worker_barrier.wait();
                match variant.as_str() {
                    "typed-drop" => drop(SnapshotRead::new(root, lease)),
                    "erased-drop" => drop(ErasedSnapshotRead::new(root, lease)),
                    "explicit-return" => assert!(SnapshotRead::new(root, lease).return_to_registry()),
                    "witness-return" => { assert!(SnapshotRead::new(root, lease).return_to_registry_witness().is_some()); }
                    _ => unreachable!(),
                }
            });
            barrier.wait();
            let mut final_value = None;
            for _ in 0..100_000 {
                if let Ok(Some(root)) = registry.try_take_one_returned::<String>() {
                    final_value = Arc::into_inner(root);
                    break;
                }
                std::thread::yield_now();
            }
            worker.join().unwrap();
            assert_eq!(serde_json::to_value(final_value).unwrap(), fixture["readerReturn"]["text"]);
            assert!(registry.terminal_is_empty());
        }
        eprintln!("[DEBUG] all four opaque read returns left unique final payload authority for the cross-worker retirement cursor");
    }

    #[test]
    fn retained_presence_read_return_injected_alias_barrier_preserves_unreturned_guard() {
        let registry = Arc::new(SnapshotReadLeaseRegistry::new());
        let root = Arc::new(String::from("aä🧵"));
        let lease = registry.try_issue(root.clone()).unwrap();
        let barrier = Arc::new(std::sync::Barrier::new(2));
        let worker_barrier = barrier.clone();
        let worker = std::thread::spawn(move || {
            let mut owner = Some(root);
            let mut lease = Some(lease);
            assert!(return_snapshot_read(&mut owner, &mut lease, || {
                worker_barrier.wait();
                worker_barrier.wait();
            }));
        });
        barrier.wait();
        assert!(!registry.has_returned());
        for _ in 0..SNAPSHOT_READ_LEASE_CAPACITY { assert!(registry.try_take_one_returned::<String>().unwrap().is_none()); }
        assert!(!registry.terminal_is_empty());
        barrier.wait();
        worker.join().unwrap();
        let mut final_value = None;
        for _ in 0..SNAPSHOT_READ_LEASE_CAPACITY {
            if let Some(root) = registry.try_take_one_returned::<String>().unwrap() { final_value = Arc::into_inner(root); break; }
        }
        assert_eq!(final_value.as_deref(), Some("aä🧵"));
        assert!(registry.terminal_is_empty());
        eprintln!("[DEBUG] injected alias-release barrier retained the unreturned registry owner until publication");
    }

    #[test]
    fn retained_presence_read_transfer_contention_preserves_unreturned_capability() {
        let registry = Arc::new(SnapshotReadLeaseRegistry::new());
        let root = Arc::new(String::from("aä🧵"));
        let lease = registry.try_issue(root.clone()).unwrap();
        let read = ErasedSnapshotRead::new(root, lease);
        let held = registry.state.lock().unwrap();
        let worker_registry = registry.clone();
        let read = match std::thread::spawn(move || read.into_typed::<String>(&worker_registry)).join().unwrap() {
            Err(read) => read,
            Ok(_) => panic!("contended transfer cannot detach its unreturned guard"),
        };
        assert!(!registry.has_returned());
        assert!(!read.lease.as_ref().unwrap().returned.load(std::sync::atomic::Ordering::Acquire));
        drop(held);
        let barrier = Arc::new(std::sync::Barrier::new(2));
        let worker_barrier = barrier.clone();
        let worker_registry = registry.clone();
        let worker = std::thread::spawn(move || {
            worker_barrier.wait();
            for _ in 0..SNAPSHOT_READ_LEASE_CAPACITY {
                if let Ok(owner) = worker_registry.try_take_one_returned::<String>() { assert!(owner.is_none()); }
            }
        });
        barrier.wait();
        let mut read = read;
        let mut owner = None;
        for _ in 0..100_000 {
            match read.into_typed::<String>(&registry) {
                Ok(root) => { owner = Some(root); break; }
                Err(retained) => read = retained,
            }
            std::thread::yield_now();
        }
        worker.join().unwrap();
        assert!(!registry.has_returned());
        assert_eq!(Arc::into_inner(owner.expect("exact transfer retries after bounded lock contention")).as_deref(), Some("aä🧵"));
        assert!(registry.terminal_is_empty());
        eprintln!("[DEBUG] contended erased transfer preserved its unreturned lease and atomically detached only its exact final owner");
    }

    #[test]
    fn retained_presence_store_close_keeps_captured_readers_and_retires_nonempty_peers() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧪️retirement.json")).unwrap();
        for case in fixture["cases"].as_array().unwrap() {
            let count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
            let factory = Arc::new(Factory(count.clone()));
            let mut owner = PresenceStore::<Value, Noop>::new(Value(case["local"].as_i64().unwrap() as i32));
            owner.install_peer_retirement_factory(factory.clone()).unwrap();
            let mut publication = owner.begin_peer_publication().unwrap();
            assert!(!publication.prune_one(|_| true).unwrap());
            for peer in case["peers"].as_array().unwrap() {
                assert!(publication.adopt(peer["actor"].as_str().unwrap().into(), Value(peer["value"].as_i64().unwrap() as i32), 0).is_ok());
            }
            while publication.release_created_one() {}
            let commit = publication.take_commit().unwrap();
            assert_eq!(close_peer_root(owner.publish_peer_commit(commit).ok().unwrap().unwrap()), 0);
            assert!(publication.terminal_is_empty());
            let shared = case["sharedReaders"].as_bool().unwrap();
            let mut local_reader = shared.then(|| owner.local_root());
            let mut peer_reader = shared.then(|| owner.peers_root());
            let mut close = owner.begin_retirement(Arc::new(Value(0)), |value| value.0 == 0, factory).ok().unwrap();
            assert!(owner.retirement_started());
            assert_eq!(owner.local_root().0, 0);
            assert!(owner.peers_root().is_empty());
            assert!(owner.apply_one(0, Noop).is_err());
            assert!(owner.begin_peer_publication().is_err());
            assert_eq!(close.close_step(0, 4096).unwrap(), SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            let mut bytes = 0;
            let mut blocked_local = false;
            let mut blocked_peers = false;
            for turn in 0..256 {
                match close.close_step(1, 4096).unwrap() {
                    SnapshotRetirementStep::Pending { released_items, released_bytes } => { assert!(released_items <= 1 && released_bytes <= 4096); bytes += released_bytes; }
                    SnapshotRetirementStep::Blocked => {
                        if let Some(reader) = local_reader.take() {
                            assert_eq!(reader.0, case["local"].as_i64().unwrap() as i32);
                            blocked_local = true;
                        } else if let Some(reader) = peer_reader.take() {
                            let actual = serde_json::to_value(reader.peers().map(|(actor, value)| serde_json::json!({ "actor": actor, "value": value.0 })).collect::<Vec<_>>()).unwrap();
                            assert_eq!(actual, case["peers"]);
                            blocked_peers = true;
                        } else { panic!("unshared presence root failed to progress"); }
                    }
                    SnapshotRetirementStep::Complete => break,
                }
                assert!(turn < 255);
            }
            assert!(close.terminal_is_empty());
            assert_eq!(count.load(std::sync::atomic::Ordering::Relaxed), case["expectedSnapshots"].as_u64().unwrap() as usize);
            assert_eq!(bytes, case["expectedActorBytes"].as_u64().unwrap() as usize);
            assert_eq!((blocked_local, blocked_peers), (shared, shared));
            eprintln!("[DEBUG] presence close case={} snapshots={} actor_bytes={bytes}", case["name"], count.load(std::sync::atomic::Ordering::Relaxed));
        }
    }

    #[test]
    fn retained_presence_store_close_rejects_nonempty_terminal_and_late_commit_without_drop() {
        let count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let factory = Arc::new(Factory(count.clone()));
        let mut owner = PresenceStore::<Value, Noop>::new(Value(7));
        owner.install_peer_retirement_factory(factory.clone()).unwrap();
        let terminal = Arc::new(Value(9));
        let rejected = owner.begin_retirement(terminal.clone(), |value| value.0 == 0, factory.clone()).err().unwrap();
        assert!(Arc::ptr_eq(&terminal, &rejected.1));
        assert!(!owner.retirement_started());
        drop((terminal, rejected));
        let mut publication = owner.begin_peer_publication().unwrap();
        assert!(!publication.prune_one(|_| true).unwrap());
        assert!(publication.adopt("late".into(), Value(11), 0).is_ok());
        assert!(publication.release_created_one());
        let commit = publication.take_commit().unwrap();
        let mut close = owner.begin_retirement(Arc::new(Value(0)), |value| value.0 == 0, factory).ok().unwrap();
        let rejected = owner.publish_peer_commit(commit).err().expect("closed store preserves the exact rejected commit");
        assert_eq!(count.load(std::sync::atomic::Ordering::Relaxed), 0);
        let mut rejected = rejected.into_retirement();
        for cursor in [&mut close, &mut rejected] {
            for turn in 0..128 {
                if cursor.close_step(1, 4096).unwrap() == SnapshotRetirementStep::Complete { break; }
                assert!(turn < 127);
            }
            assert!(cursor.terminal_is_empty());
        }
        assert_eq!(count.load(std::sync::atomic::Ordering::Relaxed), 2);
    }
}
//#endregion 🧪️Tests
