//! 🧹️ Retained ownership of a detached local presence root and its complete peer roster.

use super::*;

//#region 🧹️StoreRetirement
pub(super) fn advance_returned_local<P: Send + Sync + 'static>(
    registry: &SnapshotReadLeaseRegistry,
    active: &mut Option<Box<dyn ErasedSnapshotRetirement>>,
    factory: Option<&Arc<dyn SnapshotRetirementFactory<P>>>,
    maximum_items: usize,
    maximum_bytes: usize,
) -> Result<SnapshotRetirementStep, String> {
    if maximum_items == 0 {
        return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
    }
    if let Some(owner) = active.as_mut() {
        return match owner.close_step(1, maximum_bytes)? {
            SnapshotRetirementStep::Complete if owner.terminal_is_empty() => {
                drop(active.take());
                Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
            }
            SnapshotRetirementStep::Complete => Err("presence returned local owner completed without its exact empty witness".into()),
            SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items > 1 || released_bytes > maximum_bytes => Err("presence returned local owner exceeded its exact grant".into()),
            step => Ok(step),
        };
    }
    if !registry.has_returned() {
        return Ok(SnapshotRetirementStep::Complete);
    }
    let factory = factory.ok_or_else(|| "presence returned local read has no exact retirement factory".to_string())?;
    match registry.try_take_one_returned::<P>() {
        Ok(Some(root)) => {
            *active = Some(factory.retire(root));
            Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
        }
        Ok(None) => Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 }),
        Err(reason) if reason == "snapshot read lease registry is busy" => Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }),
        Err(reason) => Err(reason),
    }
}

pub struct PresenceStoreRetirement<P> {
    base_root: std::mem::ManuallyDrop<Option<Arc<PresencePeersRoot<P>>>>,
    local: std::mem::ManuallyDrop<Option<Arc<P>>>,
    peers: std::mem::ManuallyDrop<Option<Arc<PresencePeersRoot<P>>>>,
    active_local: std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    active_peers: std::mem::ManuallyDrop<Option<PresencePeersRetirement<P>>>,
    reads: std::mem::ManuallyDrop<Option<Arc<SnapshotReadLeaseRegistry>>>,
    active_returned: std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    local_factory: Option<Arc<dyn SnapshotRetirementFactory<P>>>,
    peer_factory: Option<Arc<dyn SnapshotRetirementFactory<P>>>,
}

impl<P: Send + Sync + 'static> PresenceStoreRetirement<P> {
    fn new(
        local: Arc<P>,
        peers: Arc<PresencePeersRoot<P>>,
        reads: Arc<SnapshotReadLeaseRegistry>,
        active_returned: Option<Box<dyn ErasedSnapshotRetirement>>,
        local_factory: Arc<dyn SnapshotRetirementFactory<P>>,
        peer_factory: Option<Arc<dyn SnapshotRetirementFactory<P>>>,
    ) -> Self {
        Self {
            base_root: std::mem::ManuallyDrop::new(None),
            local: std::mem::ManuallyDrop::new(Some(local)),
            peers: std::mem::ManuallyDrop::new(Some(peers)),
            active_local: std::mem::ManuallyDrop::new(None),
            active_peers: std::mem::ManuallyDrop::new(None),
            reads: std::mem::ManuallyDrop::new(Some(reads)),
            active_returned: std::mem::ManuallyDrop::new(active_returned),
            local_factory: Some(local_factory),
            peer_factory,
        }
    }

    pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, String> {
        if maximum_items == 0 {
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.base_root.take().is_some() {
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(reads) = self.reads.as_ref() {
            if self.active_returned.is_some() || reads.has_returned() {
                return advance_returned_local(reads, &mut self.active_returned, self.local_factory.as_ref(), 1, maximum_bytes);
            }
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
            *self.active_local = Some(self.local_factory.as_ref().expect("detached local root retains its installed factory").retire(local));
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(peers) = self.peers.take() {
            return match Arc::try_unwrap(peers) {
                Ok(peers) => {
                    if peers.is_empty() {
                        return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
                    }
                    let retired = PresencePeersRetiredEntries { entries: std::mem::ManuallyDrop::new(peers.entries), len: peers.len };
                    *self.active_peers = Some(PresencePeersRetirement::new(retired, self.peer_factory.as_ref().expect("detached nonempty peer root retains its installed factory").clone()));
                    Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                }
                Err(peers) => {
                    *self.peers = Some(peers);
                    Ok(SnapshotRetirementStep::Blocked)
                }
            };
        }
        if self.reads.as_ref().is_some_and(|reads| !reads.terminal_is_empty()) {
            return Ok(SnapshotRetirementStep::Blocked);
        }
        if self.reads.take().is_some() {
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.local_factory.take().is_some() || self.peer_factory.take().is_some() {
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(SnapshotRetirementStep::Complete)
    }
}

impl<P> PresenceStoreRetirement<P> {
    pub fn terminal_is_empty(&self) -> bool {
        self.base_root.is_none()
            && self.local.is_none()
            && self.peers.is_none()
            && self.active_local.is_none()
            && self.active_peers.is_none()
            && self.active_returned.is_none()
            && self.reads.is_none()
            && self.local_factory.is_none()
            && self.peer_factory.is_none()
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
            base_root: std::mem::ManuallyDrop::new(Some(self.base_root)),
            local: std::mem::ManuallyDrop::new(None),
            peers: std::mem::ManuallyDrop::new(Some(self.root)),
            active_local: std::mem::ManuallyDrop::new(None),
            active_peers: std::mem::ManuallyDrop::new(self.retirement),
            reads: std::mem::ManuallyDrop::new(None),
            active_returned: std::mem::ManuallyDrop::new(None),
            local_factory: None,
            peer_factory: Some(self.factory),
        }
    }
}
//#endregion 🧹️StoreRetirement

//#region 🔌️OwnerTransfer
impl<P: Clone + Send + Sync + 'static, M: self::Mutation<P>> PresenceStore<P, M> {
    /// 🧹️ Detaches exact roots once after the concrete domain validates its empty terminal value.
    pub fn begin_retirement(&mut self, terminal_local: Arc<P>, terminal_is_empty: fn(&P) -> bool) -> Result<PresenceStoreRetirement<P>, (&'static str, Arc<P>)> {
        if self.close_started || !terminal_is_empty(terminal_local.as_ref()) {
            return Err(("presence close requires a fresh store and an exact empty domain terminal", terminal_local));
        }
        let Some(local_factory) = self.local_retirement_factory.as_ref() else {
            return Err(("presence close requires its installed local-root retirement factory", terminal_local));
        };
        if !self.peers.is_empty() && self.peer_retirement_factory.is_none() {
            return Err(("presence close requires its installed peer retirement factory", terminal_local));
        }
        let local_factory = local_factory.clone();
        let peer_factory = self.peer_retirement_factory.clone();
        self.close_started = true;
        let local = std::mem::replace(&mut *self.local, terminal_local);
        let peers = std::mem::replace(&mut *self.peers, Arc::new(PresencePeersRoot::empty()));
        Ok(PresenceStoreRetirement::new(local, peers, Arc::clone(&self.local_reads), self.active_returned_local.take(), local_factory, peer_factory))
    }

    pub fn retirement_started(&self) -> bool {
        self.close_started
    }
}

impl<P, M> Drop for PresenceStore<P, M> {
    fn drop(&mut self) {
        let terminal = self.close_started && self.local_reads.terminal_is_empty() && self.active_returned_local.is_none();
        if !std::thread::panicking() {
            assert!(terminal, "presence store requires its exact detached terminal-empty owner before Drop");
        }
        if terminal {
            unsafe {
                std::mem::ManuallyDrop::drop(&mut self.local);
                std::mem::ManuallyDrop::drop(&mut self.peers);
                std::mem::ManuallyDrop::drop(&mut self.local_reads);
            }
        }
    }
}
//#endregion 🔌️OwnerTransfer

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️fixtures/🧬️mutations/🦀️.rs"]
mod fixture_mutations;

#[cfg(test)]
mod tests {
    use super::fixture_mutations::{SetValue, ValueMutation};
    use super::*;

    pub(super) fn assert_fixture_descriptor<T: crate::os_spr::MutationLeaf>(descriptor: &str) {
        assert_eq!(serde_json::to_value(T::DESCRIPTOR).unwrap(), serde_json::from_str::<serde_json::Value>(descriptor).unwrap());
        assert!(T::DESCRIPTOR.validate().is_ok());
    }

    #[test]
    fn direct_presence_fixture_value_inverse() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧪️fixtures/🔣️.json")).unwrap();
        let row = fixture["cases"].as_array().unwrap().iter().find(|row| row["family"] == "presence").unwrap();
        let before = Value(serde_json::from_value(row["before"].clone()).unwrap());
        let mut wire = row["payload"].clone();
        wire["operation"] = row["operation"].clone();
        let op = serde_json::from_value::<ValueMutation>(wire).unwrap();
        let after = op.diff(&before).diff().apply(&before).unwrap();
        assert_eq!(after.0, serde_json::from_value::<i32>(row["after"].clone()).unwrap());
        assert_eq!(op.inverse(&before)[0].diff(&after).diff().apply(&after).unwrap().0, before.0);
        assert_eq!(serde_json::from_value::<ValueMutation>(serde_json::to_value(&op).unwrap()).unwrap(), op);
        for json in ["{\"operation\":\"setValue\"}", "{\"operation\":\"setValue\",\"n\":null}", "{\"operation\":\"setValue\",\"n\":2147483648}", "{\"operation\":\"setValue\",\"n\":0.5}", "{\"operation\":\"setValue\",\"n\":7,\"unknown\":true}"] {
            assert!(serde_json::from_str::<ValueMutation>(json).is_err());
        }
    }

    #[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
    pub(crate) struct Value(pub(super) i32);

    /// 🔀️ Hand-written, not derived: tuple struct, not one of `#[derive(ToValue, FromValue)]`'s
    /// supported shapes.
    impl ::semio_framework_os_kernel::ToValue for Value {
        fn to_value(&self) -> ::semio_framework_os_kernel::DslValue {
            ::semio_framework_os_kernel::ToValue::to_value(&self.0)
        }
    }
    impl ::semio_framework_os_kernel::FromValue for Value {
        fn from_value(value: ::semio_framework_os_kernel::DslValue) -> Result<Self, ::semio_framework_os_kernel::ValueError> {
            Ok(Self(::semio_framework_os_kernel::FromValue::from_value(value)?))
        }
    }

    impl MutationDiff<Value> for Value {
        fn apply(&self, _base: &Value) -> crate::os_spr::MutationApplyResult<Value> {
            Ok(self.clone())
        }
        fn absorb(&mut self, other: Self) {
            *self = other;
        }
    }

    struct Factory(Arc<std::sync::atomic::AtomicUsize>);
    struct Retirement {
        root: std::mem::ManuallyDrop<Option<Arc<Value>>>,
        count: Arc<std::sync::atomic::AtomicUsize>,
    }

    impl SnapshotRetirementFactory<Value> for Factory {
        fn retire(&self, root: Arc<Value>) -> Box<dyn ErasedSnapshotRetirement> {
            Box::new(Retirement { root: std::mem::ManuallyDrop::new(Some(root)), count: self.0.clone() })
        }
    }

    impl ErasedSnapshotRetirement for Retirement {
        fn close_step(&mut self, items: usize, _bytes: usize) -> Result<SnapshotRetirementStep, String> {
            if items == 0 {
                return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            }
            let Some(root) = self.root.take() else { return Ok(SnapshotRetirementStep::Complete) };
            if Arc::into_inner(root).is_some() {
                self.count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            }
            Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
        }
        fn terminal_is_empty(&self) -> bool {
            self.root.is_none()
        }
    }

    struct CapturedLocalJob {
        read: Option<SnapshotRead<Value>>,
        returned: Option<SnapshotReadReturn>,
        closing: bool,
    }

    impl semio_framework_job::InteractiveJob for CapturedLocalJob {
        fn step(&mut self, _cx: &mut semio_framework_job::StepContext<'_>) -> semio_framework_job::StepOutcome {
            assert_eq!(self.read.as_ref().unwrap().get().0, 23);
            semio_framework_job::StepOutcome::Yield
        }
        fn begin_close(&mut self) {
            self.closing = true;
        }
        fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
            if maximum_items == 0 {
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            if let Some(read) = self.read.take() {
                self.returned = read.return_to_registry_witness();
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
            }
            if let Some(returned) = self.returned.as_ref() {
                if !returned.terminal_is_empty() {
                    return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
                }
                drop(self.returned.take());
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
            }
            semio_framework_job::InteractiveJobCloseStep::Complete
        }
        fn terminal_is_empty(&self) -> bool {
            self.closing && self.read.is_none() && self.returned.is_none()
        }
    }

    #[test]
    fn retained_presence_local_capture_cancel_closes_mounted_worker_while_store_remains_open() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧪️retirement.json")).unwrap();
        let law = &fixture["localCapture"];
        let count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let factory = Arc::new(Factory(count.clone()));
        let mut owner = PresenceStore::<Value, ValueMutation>::new(Value(law["value"].as_i64().unwrap() as i32));
        assert!(owner.local_read().is_err());
        owner.install_local_retirement_factory(factory.clone()).unwrap();
        let job = CapturedLocalJob { read: Some(owner.local_read().unwrap()), returned: None, closing: false };
        let cancel = semio_framework_job::root_cancel_token();
        let params = semio_framework_job::BatchJobParams {
            operation: semio_framework_job::allocate_operation_id(),
            generation: semio_framework_job::Generation(0),
            cancel: cancel.clone(),
            config: semio_framework_job::BatchDriveConfig { site: "presence.local.capture", stage: semio_framework_job::InteractiveStage::InteractiveStep, fuel_per_step: 1, step_budget_us: 8000 },
            now_us: semio_framework_job::default_now_us,
        };
        let mut session = semio_framework_job::MountedWorkerJobSession::try_new(job, params).unwrap_or_else(|_| panic!("exact mounted capture slot"));
        let pool = semio_framework_async::process_worker_pool(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::InteractiveNative, std::thread::available_parallelism().map(std::num::NonZeroUsize::get).unwrap_or(1)));
        for _ in 0..100_000 {
            session.pump_one(&pool, semio_framework_async::Lane::Interactive).unwrap_or_else(|_| panic!("mounted Presence capture must pump its exact worker"));
            if session.checked_out_outcome().is_some() {
                break;
            }
            std::thread::yield_now();
        }
        assert!(session.checked_out_outcome().is_some());
        cancel.cancel_now();
        session.begin_close();
        for _ in 0..4096 {
            let _ = session.close_step(1, 4096);
            owner.maintenance_local_reads_step(1, 4096).unwrap();
            if session.terminal_is_empty() && owner.local_read_maintenance_is_idle() {
                break;
            }
        }
        assert_eq!(session.terminal_is_empty(), law["expectedWorkerTerminal"].as_bool().unwrap());
        assert!(!owner.retirement_started());
        assert_eq!(serde_json::to_value(owner.local()).unwrap(), law["expectedValueWhileOpen"]);
        assert_eq!(count.load(std::sync::atomic::Ordering::Relaxed), 0);
        let mut close = owner.begin_retirement(Arc::new(Value(0)), |value| value.0 == 0).ok().unwrap();
        for _ in 0..2048 {
            if close.close_step(1, 4096).unwrap() == SnapshotRetirementStep::Complete {
                break;
            }
        }
        assert!(close.terminal_is_empty());
        assert_eq!(serde_json::json!(count.load(std::sync::atomic::Ordering::Relaxed)), law["expectedFinalSnapshots"]);
        eprintln!("[DEBUG] mounted captured Presence cancelled and closed while live Store preserved value23; final domain retirement occurred once after Store close");
    }

    #[test]
    fn retained_presence_local_replacements_release_shared_aliases_and_retire_exact_final_owners() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧪️retirement.json")).unwrap();
        let law = &fixture["localReplacements"];
        let count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let factory = Arc::new(Factory(count.clone()));
        let mut owner = PresenceStore::<Value, ValueMutation>::new(Value(23));
        owner.install_local_retirement_factory(factory.clone()).unwrap();
        let first = owner.local_read().unwrap();
        owner.apply_one(0, ValueMutation::SetValue(SetValue { n: 31 })).ok().unwrap();
        let second = owner.local_read().unwrap();
        owner.apply_one(1, ValueMutation::SetValue(SetValue { n: 47 })).ok().unwrap();
        assert_eq!(serde_json::json!([first.get().0, second.get().0]), law["capturedValues"]);
        let barrier = Arc::new(std::sync::Barrier::new(2));
        let other = barrier.clone();
        let first = std::thread::spawn(move || {
            other.wait();
            drop(first);
        });
        let second = std::thread::spawn(move || {
            barrier.wait();
            drop(second);
        });
        first.join().unwrap();
        second.join().unwrap();
        let guard = owner.local_reads.state.lock().unwrap();
        assert_eq!(advance_returned_local(&owner.local_reads, &mut owner.active_returned_local, owner.local_retirement_factory.as_ref(), 1, 4096).unwrap(), SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        drop(guard);
        for _ in 0..4096 {
            if owner.maintenance_local_reads_step(1, 4096).unwrap() == SnapshotRetirementStep::Complete {
                break;
            }
        }
        assert!(owner.local_read_maintenance_is_idle());
        assert!(!owner.retirement_started());
        assert_eq!(owner.local().0, 47);
        assert_eq!(serde_json::json!(count.load(std::sync::atomic::Ordering::Relaxed)), law["expectedRetiredWhileOpen"]);
        let mut close = owner.begin_retirement(Arc::new(Value(0)), |value| value.0 == 0).ok().unwrap();
        for _ in 0..2048 {
            if close.close_step(1, 4096).unwrap() == SnapshotRetirementStep::Complete {
                break;
            }
        }
        assert!(close.terminal_is_empty());
        assert_eq!(serde_json::json!(count.load(std::sync::atomic::Ordering::Relaxed)), law["expectedFinalSnapshots"]);
        eprintln!("[DEBUG] two overlapping local replacements and cross-worker readers retired exactly two old roots while Store47 remained live; final total3");
    }

    fn close_peer_root(mut retirement: PresencePeersRetirement<Value>) -> usize {
        let mut bytes = 0;
        for _ in 0..512 {
            match retirement.close_step(1, 4096).unwrap() {
                SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                    assert!(released_items <= 1 && released_bytes <= 4096);
                    bytes += released_bytes;
                }
                SnapshotRetirementStep::Complete => {
                    assert!(retirement.terminal_is_empty());
                    return bytes;
                }
                SnapshotRetirementStep::Blocked => panic!("peer root has no remaining captured reader"),
            }
        }
        panic!("exact peer root failed bounded terminal progress")
    }

    fn peer_commit(owner: &PresenceStore<Value, ValueMutation>, peer: &serde_json::Value) -> PresencePeersCommit<Value> {
        let mut publication = owner.begin_peer_publication().unwrap();
        while publication.prune_one(|_| true).unwrap() {}
        publication.adopt(peer["actor"].as_str().unwrap().into(), Value(peer["value"].as_i64().unwrap() as i32), 0).ok().unwrap();
        while publication.release_created_one() {}
        let commit = publication.take_commit().unwrap();
        assert!(publication.terminal_is_empty());
        commit
    }

    #[test]
    fn retained_presence_peer_commit_rejects_foreign_and_stale_roots_without_losing_exact_owners() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧪️peer-commit.json")).unwrap();
        for law in fixture["cases"].as_array().unwrap() {
            let count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
            let factory: Arc<dyn SnapshotRetirementFactory<Value>> = Arc::new(Factory(count.clone()));
            let other_factory: Arc<dyn SnapshotRetirementFactory<Value>> = if law["sameFactory"] == true { factory.clone() } else { Arc::new(Factory(count.clone())) };
            let mut first = PresenceStore::<Value, ValueMutation>::new(Value(17));
            first.install_local_retirement_factory(factory.clone()).unwrap();
            first.install_peer_retirement_factory(factory.clone()).unwrap();
            let mut other = PresenceStore::<Value, ValueMutation>::new(Value(23));
            other.install_local_retirement_factory(other_factory.clone()).unwrap();
            other.install_peer_retirement_factory(other_factory).unwrap();
            let candidate = peer_commit(&first, &fixture["candidate"]);
            let mut displaced = if law["stale"] == true { Some(first.publish_peer_commit(peer_commit(&first, &fixture["winner"])).ok().unwrap().unwrap()) } else { None };
            let blocked = displaced.as_mut().map(|owner| owner.close_step(1, 4096).unwrap() == SnapshotRetirementStep::Blocked);
            let target = if law["sameStore"] == true { &mut first } else { &mut other };
            let before = target.peers_root();
            let result = target.publish_peer_commit(candidate);
            let accepted = result.is_ok();
            let unchanged = Arc::ptr_eq(&before, &target.peers_root());
            drop(before);
            match result {
                Ok(Some(retirement)) => {
                    close_peer_root(retirement);
                }
                Ok(None) => panic!("peer commit must hand back its exact displaced root"),
                Err(commit) => {
                    let mut retirement = commit.into_retirement();
                    for _ in 0..2048 {
                        if retirement.close_step(1, 4096).unwrap() == SnapshotRetirementStep::Complete {
                            break;
                        }
                    }
                    assert!(retirement.terminal_is_empty());
                }
            }
            if let Some(retirement) = displaced {
                close_peer_root(retirement);
            }
            let mut first = first.begin_retirement(Arc::new(Value(0)), |value| value.0 == 0).ok().unwrap();
            let mut other = other.begin_retirement(Arc::new(Value(0)), |value| value.0 == 0).ok().unwrap();
            for owner in [&mut first, &mut other] {
                for _ in 0..2048 {
                    if owner.close_step(1, 4096).unwrap() == SnapshotRetirementStep::Complete {
                        break;
                    }
                }
                assert!(owner.terminal_is_empty());
            }
            assert_eq!(accepted, law["accepted"].as_bool().unwrap(), "{}", law["name"]);
            assert_eq!(unchanged, !accepted);
            if law["stale"] == true {
                assert_eq!(blocked, Some(true));
            }
            assert_eq!(count.load(std::sync::atomic::Ordering::SeqCst) as u64, law["expectedSnapshots"].as_u64().unwrap());
            eprintln!("[DEBUG] exact peer commit case={} accepted={accepted}, unchanged={unchanged}, base-retirement-blocked={blocked:?}, snapshots={}", law["name"], count.load(std::sync::atomic::Ordering::SeqCst));
        }
    }

    #[test]
    fn retained_presence_store_close_preserves_distinct_original_local_and_peer_factories() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧪️retirement.json")).unwrap();
        let law = &fixture["closeFactoryBinding"];
        let local = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let peer = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let foreign = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let mut owner = PresenceStore::<Value, ValueMutation>::new(Value(law["local"].as_i64().unwrap() as i32));
        owner.install_local_retirement_factory(Arc::new(Factory(local.clone()))).unwrap();
        owner.install_peer_retirement_factory(Arc::new(Factory(peer.clone()))).unwrap();
        let read = owner.local_read().unwrap();
        let mut publication = owner.begin_peer_publication().unwrap();
        while publication.prune_one(|_| true).unwrap() {}
        publication.adopt(law["peer"]["actor"].as_str().unwrap().into(), Value(law["peer"]["value"].as_i64().unwrap() as i32), 0).ok().unwrap();
        while publication.release_created_one() {}
        close_peer_root(owner.publish_peer_commit(publication.take_commit().unwrap()).ok().unwrap().unwrap());
        let foreign_factory = Arc::new(Factory(foreign.clone()));
        let mut close = owner.begin_retirement(Arc::new(Value(0)), |value| value.0 == 0).ok().unwrap();
        drop(read);
        for _ in 0..2048 {
            if close.close_step(1, 4096).unwrap() == SnapshotRetirementStep::Complete {
                break;
            }
        }
        assert!(close.terminal_is_empty());
        let actual = serde_json::json!({ "local": local.load(std::sync::atomic::Ordering::SeqCst), "peer": peer.load(std::sync::atomic::Ordering::SeqCst), "foreign": foreign.load(std::sync::atomic::Ordering::SeqCst) });
        assert_eq!(actual, serde_json::json!({ "local": law["expectedLocal"], "peer": law["expectedPeer"], "foreign": law["expectedForeign"] }));
        assert_eq!(Arc::strong_count(&foreign_factory), 1);
        eprintln!("[DEBUG] Presence detach preserved original local/peer factories and returned-read authority: {actual}");
    }

    #[test]
    fn retained_presence_overlapping_rosters_retire_shared_entries_once_across_workers() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧪️retirement.json")).unwrap();
        let law = &fixture["overlap"];
        for race in [false, true] {
            let count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
            let factory = Arc::new(Factory(count.clone()));
            let mut owner = PresenceStore::<Value, ValueMutation>::new(Value(23));
            owner.install_local_retirement_factory(factory.clone()).unwrap();
            owner.install_peer_retirement_factory(factory.clone()).unwrap();
            let mut publication = owner.begin_peer_publication().unwrap();
            while publication.prune_one(|_| true).unwrap() {}
            for peer in law["first"].as_array().unwrap() {
                publication.adopt(peer["actor"].as_str().unwrap().into(), Value(peer["value"].as_i64().unwrap() as i32), 0).ok().unwrap();
            }
            while publication.release_created_one() {}
            let commit = publication.take_commit().unwrap();
            assert_eq!(close_peer_root(owner.publish_peer_commit(commit).ok().unwrap().unwrap()), 0);
            let reader = owner.peers_root();
            let mut publication = owner.begin_peer_publication().unwrap();
            while publication.prune_one(|_| true).unwrap() {}
            for peer in law["second"].as_array().unwrap().iter().skip(law["first"].as_array().unwrap().len()) {
                publication.adopt(peer["actor"].as_str().unwrap().into(), Value(peer["value"].as_i64().unwrap() as i32), 0).ok().unwrap();
            }
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
                let first = std::thread::spawn(move || {
                    first_barrier.wait();
                    close_peer_root(first)
                });
                let second = std::thread::spawn(move || {
                    barrier.wait();
                    close_peer_root(second)
                });
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
            let mut close = owner.begin_retirement(Arc::new(Value(0)), |value| value.0 == 0).ok().unwrap();
            for _ in 0..256 {
                if close.close_step(1, 4096).unwrap() == SnapshotRetirementStep::Complete {
                    break;
                }
            }
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
                    "witness-return" => {
                        assert!(SnapshotRead::new(root, lease).return_to_registry_witness().is_some());
                    }
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
        for _ in 0..SNAPSHOT_READ_LEASE_CAPACITY {
            assert!(registry.try_take_one_returned::<String>().unwrap().is_none());
        }
        assert!(!registry.terminal_is_empty());
        barrier.wait();
        worker.join().unwrap();
        let mut final_value = None;
        for _ in 0..SNAPSHOT_READ_LEASE_CAPACITY {
            if let Some(root) = registry.try_take_one_returned::<String>().unwrap() {
                final_value = Arc::into_inner(root);
                break;
            }
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
                if let Ok(owner) = worker_registry.try_take_one_returned::<String>() {
                    assert!(owner.is_none());
                }
            }
        });
        barrier.wait();
        let mut read = read;
        let mut owner = None;
        for _ in 0..100_000 {
            match read.into_typed::<String>(&registry) {
                Ok(root) => {
                    owner = Some(root);
                    break;
                }
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
            let mut owner = PresenceStore::<Value, ValueMutation>::new(Value(case["local"].as_i64().unwrap() as i32));
            owner.install_local_retirement_factory(factory.clone()).unwrap();
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
            let mut local_reader = shared.then(|| owner.local_read().unwrap());
            let mut peer_reader = shared.then(|| owner.peers_root());
            let mut close = owner.begin_retirement(Arc::new(Value(0)), |value| value.0 == 0).ok().unwrap();
            assert!(owner.retirement_started());
            assert_eq!(owner.local().0, 0);
            assert!(owner.peers_root().is_empty());
            assert!(owner.apply_one(0, ValueMutation::SetValue(SetValue { n: 1 })).is_err());
            assert!(owner.begin_peer_publication().is_err());
            assert_eq!(close.close_step(0, 4096).unwrap(), SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            let mut bytes = 0;
            let mut blocked_local = false;
            let mut blocked_peers = false;
            for turn in 0..256 {
                match close.close_step(1, 4096).unwrap() {
                    SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                        assert!(released_items <= 1 && released_bytes <= 4096);
                        bytes += released_bytes;
                    }
                    SnapshotRetirementStep::Blocked => {
                        if let Some(reader) = local_reader.take() {
                            assert_eq!(reader.0, case["local"].as_i64().unwrap() as i32);
                            blocked_local = true;
                        } else if let Some(reader) = peer_reader.take() {
                            let actual = serde_json::to_value(reader.peers().map(|(actor, value)| serde_json::json!({ "actor": actor, "value": value.0 })).collect::<Vec<_>>()).unwrap();
                            assert_eq!(actual, case["peers"]);
                            blocked_peers = true;
                        } else {
                            panic!("unshared presence root failed to progress");
                        }
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
        let mut owner = PresenceStore::<Value, ValueMutation>::new(Value(7));
        let empty = Arc::new(Value(0));
        let missing = owner.begin_retirement(empty.clone(), |value| value.0 == 0).err().unwrap();
        assert_eq!(missing.0, "presence close requires its installed local-root retirement factory");
        assert!(Arc::ptr_eq(&missing.1, &empty));
        assert!(!owner.retirement_started());
        drop((missing, empty));
        owner.install_local_retirement_factory(factory.clone()).unwrap();
        owner.install_peer_retirement_factory(factory.clone()).unwrap();
        let terminal = Arc::new(Value(9));
        let rejected = owner.begin_retirement(terminal.clone(), |value| value.0 == 0).err().unwrap();
        assert!(Arc::ptr_eq(&terminal, &rejected.1));
        assert!(!owner.retirement_started());
        drop((terminal, rejected));
        let mut publication = owner.begin_peer_publication().unwrap();
        assert!(!publication.prune_one(|_| true).unwrap());
        assert!(publication.adopt("late".into(), Value(11), 0).is_ok());
        assert!(publication.release_created_one());
        let commit = publication.take_commit().unwrap();
        let mut close = owner.begin_retirement(Arc::new(Value(0)), |value| value.0 == 0).ok().unwrap();
        let rejected = owner.publish_peer_commit(commit).err().expect("closed store preserves the exact rejected commit");
        assert_eq!(count.load(std::sync::atomic::Ordering::Relaxed), 0);
        let mut rejected = rejected.into_retirement();
        for turn in 0..128 {
            for cursor in [&mut close, &mut rejected] {
                match cursor.close_step(1, 4096).unwrap() {
                    SnapshotRetirementStep::Pending { released_items, released_bytes } => assert!(released_items <= 1 && released_bytes <= 4096),
                    SnapshotRetirementStep::Blocked | SnapshotRetirementStep::Complete => {}
                }
            }
            if close.terminal_is_empty() && rejected.terminal_is_empty() {
                break;
            }
            assert!(turn < 127);
        }
        assert!(close.terminal_is_empty() && rejected.terminal_is_empty());
        assert_eq!(count.load(std::sync::atomic::Ordering::Relaxed), 2);
        eprintln!("[DEBUG] closed Store and rejected late peer commit co-retired their exact base alias and two snapshots");
    }
}
//#endregion 🧪️Tests
