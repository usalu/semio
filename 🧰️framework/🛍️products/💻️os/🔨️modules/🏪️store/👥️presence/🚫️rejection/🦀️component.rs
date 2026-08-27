//! 🚫️ Exact ownership of both rejected peer identity bytes and its typed presence value.

use super::*;

//#region 🚫️AdmissionOwner
pub struct PresencePeerAdmissionRejected<P> {
    pub reason: &'static str,
    actor: std::mem::ManuallyDrop<String>,
    presence: std::mem::ManuallyDrop<Option<P>>,
    factory: std::mem::ManuallyDrop<Option<Arc<dyn SnapshotRetirementFactory<P>>>>,
}

impl<P> PresencePeerAdmissionRejected<P> {
    pub(super) fn new(reason: &'static str, actor: String, presence: P, factory: Arc<dyn SnapshotRetirementFactory<P>>) -> Self {
        Self { reason, actor: std::mem::ManuallyDrop::new(actor), presence: std::mem::ManuallyDrop::new(Some(presence)), factory: std::mem::ManuallyDrop::new(Some(factory)) }
    }

    pub fn actor(&self) -> &str { self.actor.as_str() }

    pub fn presence(&self) -> &P { self.presence.as_ref().expect("rejected admission retains its exact presence") }

    pub fn into_retirement(mut self) -> Box<dyn ErasedSnapshotRetirement>
    where P: Send + Sync + 'static,
    {
        let actor = std::mem::take(&mut *self.actor);
        let presence = self.presence.take();
        let factory = self.factory.take().expect("rejected admission retains its minting publication factory");
        Box::new(PresencePeerRejectionRetirement {
            actor: std::mem::ManuallyDrop::new(Some(actor.into_bytes())),
            presence: std::mem::ManuallyDrop::new(presence),
            active: std::mem::ManuallyDrop::new(None),
            factory,
        })
    }
}

impl<P> Drop for PresencePeerAdmissionRejected<P> {
    fn drop(&mut self) {
        let terminal = self.actor.is_empty() && self.actor.capacity() == 0 && self.presence.is_none() && self.factory.is_none();
        if !std::thread::panicking() {
            assert!(terminal, "rejected peer admission requires exact actor and presence owner transfer");
        }
    }
}
//#endregion 🚫️AdmissionOwner

//#region 🧹️RetainedRejection
pub(super) struct PresencePeerRejectionRetirement<P> {
    actor: std::mem::ManuallyDrop<Option<Vec<u8>>>,
    presence: std::mem::ManuallyDrop<Option<P>>,
    active: std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    factory: Arc<dyn SnapshotRetirementFactory<P>>,
}

impl<P: Send + Sync + 'static> ErasedSnapshotRetirement for PresencePeerRejectionRetirement<P> {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, String> {
        if maximum_items == 0 { return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
        if let Some(actor) = self.actor.as_mut() {
            if actor.is_empty() {
                drop(self.actor.take());
                return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            let released_bytes = actor.len().min(maximum_bytes);
            actor.truncate(actor.len() - released_bytes);
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes });
        }
        if let Some(active) = self.active.as_mut() {
            return match active.close_step(1, maximum_bytes)? {
                SnapshotRetirementStep::Complete if active.terminal_is_empty() => {
                    drop(self.active.take());
                    Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                }
                SnapshotRetirementStep::Complete => Err("rejected peer presence reported Complete without an exact empty witness".into()),
                SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items > 1 || released_bytes > maximum_bytes => Err("rejected peer presence exceeded its exact grant".into()),
                step => Ok(step),
            };
        }
        if let Some(presence) = self.presence.take() {
            *self.active = Some(self.factory.retire(Arc::new(presence)));
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool { self.actor.is_none() && self.presence.is_none() && self.active.is_none() }
}

impl<P> Drop for PresencePeerRejectionRetirement<P> {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            assert!(self.actor.is_none() && self.presence.is_none() && self.active.is_none(), "rejected peer cursor requires its exact empty terminal witness");
        }
    }
}
//#endregion 🧹️RetainedRejection

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    struct Factory(Arc<std::sync::atomic::AtomicUsize>);
    struct Retirement(std::mem::ManuallyDrop<Option<Arc<i32>>>, Arc<std::sync::atomic::AtomicUsize>);

    impl SnapshotRetirementFactory<i32> for Factory {
        fn retire(&self, value: Arc<i32>) -> Box<dyn ErasedSnapshotRetirement> {
            Box::new(Retirement(std::mem::ManuallyDrop::new(Some(value)), self.0.clone()))
        }
    }

    impl ErasedSnapshotRetirement for Retirement {
        fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> Result<SnapshotRetirementStep, String> {
            if maximum_items == 0 { return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
            if let Some(value) = self.0.take() {
                if Arc::into_inner(value) == Some(41) { self.1.fetch_add(1, std::sync::atomic::Ordering::Relaxed); }
                return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            Ok(SnapshotRetirementStep::Complete)
        }
        fn terminal_is_empty(&self) -> bool { self.0.is_none() }
    }

    #[test]
    fn retained_presence_peer_rejection_keeps_its_minting_factory_after_source_close() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧪️peer-admission.json")).unwrap();
        let law = &fixture["factoryBinding"];
        let original = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let foreign = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let original_factory: Arc<dyn SnapshotRetirementFactory<i32>> = Arc::new(Factory(original.clone()));
        let foreign_factory: Arc<dyn SnapshotRetirementFactory<i32>> = Arc::new(Factory(foreign.clone()));
        let mut source = PresencePeersPublication::<i32>::new(&Arc::new(PresencePeersRoot::empty()), original_factory.clone());
        let mut other = PresencePeersPublication::<i32>::new(&Arc::new(PresencePeersRoot::empty()), foreign_factory.clone());
        let rejected = source.adopt("exact-owner".into(), 41, 0).err().unwrap();
        assert!(Arc::ptr_eq(rejected.factory.as_ref().unwrap(), &original_factory));
        assert!(!Arc::ptr_eq(rejected.factory.as_ref().unwrap(), &foreign_factory));
        for _ in 0..16 { if source.close_step(1, 4096).unwrap() == SnapshotRetirementStep::Complete { break; } }
        assert!(source.terminal_is_empty());
        drop(source);
        drop(original_factory);
        let mut rejected = rejected.into_retirement();
        for _ in 0..128 { if rejected.close_step(1, 1).unwrap() == SnapshotRetirementStep::Complete { break; } }
        assert!(rejected.terminal_is_empty());
        assert_eq!(serde_json::json!(original.load(std::sync::atomic::Ordering::Relaxed)), law["expectedOriginalRetirements"]);
        assert_eq!(serde_json::json!(foreign.load(std::sync::atomic::Ordering::Relaxed)), law["expectedForeignRetirements"]);
        for _ in 0..16 { if other.close_step(1, 4096).unwrap() == SnapshotRetirementStep::Complete { break; } }
        assert!(other.terminal_is_empty());
        eprintln!("[DEBUG] rejected peer kept its original exact factory after source publication closed; original=1 foreign=0");
    }

    #[test]
    fn retained_presence_peer_admission_preserves_rejected_actor_allocation_and_payload() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧪️peer-admission.json")).unwrap();
        for case in fixture["cases"].as_array().unwrap() {
            let count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
            let factory = Arc::new(Factory(count.clone()));
            let mut publication = PresencePeersPublication::<i32>::new(&Arc::new(PresencePeersRoot::empty()), factory.clone());
            let state = case["state"].as_str().unwrap();
            if state != "pruning" { while publication.prune_one(|_| true).unwrap() {} }
            if state == "transferred" {
                let mut transferred = publication.take_commit().unwrap().into_retirement();
                for _ in 0..16 { if transferred.close_step(1, 4096).unwrap() == SnapshotRetirementStep::Complete { break; } }
                assert!(transferred.terminal_is_empty() && publication.terminal_is_empty());
            }
            let mut seeded_bytes = 0;
            if matches!(state, "full" | "created-full") {
                for index in 0..PRESENCE_PEER_SLOTS {
                    let actor = format!("seed-{index:02}");
                    seeded_bytes += actor.len();
                    assert!(publication.adopt(actor, 0, 0).is_ok());
                }
            }
            let text = case["actor"]["unit"].as_str().unwrap().repeat(case["actor"]["repeat"].as_u64().unwrap() as usize);
            let minimum_capacity = case["actor"]["minimumCapacity"].as_u64().unwrap() as usize;
            let mut actor = String::with_capacity(minimum_capacity);
            actor.push_str(&text);
            let pointer = actor.as_ptr();
            let capacity = actor.capacity();
            assert!(capacity >= minimum_capacity && capacity > fixture["maximumBytes"].as_u64().unwrap() as usize);
            let expected_bytes = case["expectedActorBytes"].as_u64().unwrap() as usize;
            assert_eq!(serde_json::from_str::<String>(&serde_json::to_string(&text).unwrap()).unwrap().len(), expected_bytes);
            let mut bytes = 0;
            match publication.adopt(actor, 41, 0) {
                Ok(()) => assert!(case["accepted"].as_bool().unwrap()),
                Err(rejected) => {
                    assert!(!case["accepted"].as_bool().unwrap());
                    assert_eq!(rejected.actor(), text);
                    assert_eq!(rejected.actor.as_ptr(), pointer);
                    assert_eq!(rejected.actor.capacity(), capacity);
                    assert_eq!(*rejected.presence(), 41);
                    assert_eq!(count.load(std::sync::atomic::Ordering::Relaxed), 0);
                    let mut rejected = rejected.into_retirement();
                    assert_eq!(rejected.close_step(0, 4096).unwrap(), SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
                    for _ in 0..4096 {
                        match rejected.close_step(1, 1).unwrap() {
                            SnapshotRetirementStep::Pending { released_items, released_bytes } => { assert!(released_items <= 1 && released_bytes <= 1); bytes += released_bytes; }
                            SnapshotRetirementStep::Complete => break,
                            SnapshotRetirementStep::Blocked => panic!("exact rejected owners require no external alias"),
                        }
                    }
                    assert!(rejected.terminal_is_empty());
                    assert_eq!(bytes, expected_bytes);
                }
            }
            for _ in 0..4096 {
                match publication.close_step(1, 4096).unwrap() {
                    SnapshotRetirementStep::Pending { released_items, released_bytes } => { assert!(released_items <= 1 && released_bytes <= 4096); bytes += released_bytes; }
                    SnapshotRetirementStep::Complete => break,
                    SnapshotRetirementStep::Blocked => panic!("candidate owns all rejected fixture aliases"),
                }
            }
            assert!(publication.terminal_is_empty());
            assert_eq!(bytes, expected_bytes + seeded_bytes);
            assert_eq!(count.load(std::sync::atomic::Ordering::Relaxed), 1);
            eprintln!("[DEBUG] peer admission case={} retained actor capacity={capacity}, retired initialized bytes={}, exact target snapshots=1", case["name"], expected_bytes);
        }
    }
}
//#endregion 🧪️Tests
