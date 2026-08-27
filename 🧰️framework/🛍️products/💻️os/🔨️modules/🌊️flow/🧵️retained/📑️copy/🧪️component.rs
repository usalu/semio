//! 🧪️ Selected typed Flow copies preserve canonical payloads and exact rooted ownership.

use super::*;
use std::sync::atomic::{AtomicUsize, Ordering};

//#region 🧪️RootAuthority
fn allocation() -> FlowCopyAllocationBudget { FlowCopyAllocationBudget::new(16 * 1024 * 1024, 32 * 1024 * 1024) }
#[derive(Debug)]
struct Root { fixture: Option<FlowFixture>, drops: Arc<AtomicUsize> }
impl Drop for Root { fn drop(&mut self) { assert!(self.fixture.is_none()); self.drops.fetch_add(1, Ordering::SeqCst); } }
struct RootFactory;
struct RootRetirement { root: Option<Arc<Root>>, retirement: Retirement }
impl SnapshotRetirementFactory<Root> for RootFactory {
    fn retire(&self, root: Arc<Root>) -> Box<dyn ErasedSnapshotRetirement> {
        assert_eq!(Arc::strong_count(&root), 1, "borrowed frames must release before the root");
        Box::new(RootRetirement { root: Some(root), retirement: Retirement::default() })
    }
}
impl ErasedSnapshotRetirement for RootRetirement {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, String> {
        if maximum_items == 0 || maximum_bytes == 0 { return Ok(SnapshotRetirementStep::Blocked); }
        if !self.retirement.is_empty() { return self.retirement.close_step(maximum_items, maximum_bytes); }
        if let Some(root) = self.root.take() {
            let mut root = Arc::into_inner(root).expect("final selected copy source");
            self.retirement.push(Owner::Fixture(root.fixture.take().unwrap()));
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(SnapshotRetirementStep::Complete)
    }
    fn terminal_is_empty(&self) -> bool { self.root.is_none() && self.retirement.is_empty() }
}
fn source() -> (Arc<Root>, Arc<AtomicUsize>) {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧪️fixtures/🔣️retirement.json")).unwrap();
    let fixture = serde_json::from_value(fixture["fixture"].clone()).unwrap();
    let drops = Arc::new(AtomicUsize::new(0));
    (Arc::new(Root { fixture: Some(fixture), drops: drops.clone() }), drops)
}
fn close<R: Send + Sync + 'static, T: Copy>(cursor: &mut CopyCursor<R, T>, grant: usize) {
    cursor.begin_close();
    for _ in 0..200_000 {
        match cursor.close_step(1, grant).unwrap() {
            SnapshotRetirementStep::Complete => break,
            SnapshotRetirementStep::Pending { released_items, released_bytes } => assert!(released_items <= 1 && released_bytes <= grant),
            SnapshotRetirementStep::Blocked => panic!("positive copy close grant blocked"),
        }
    }
    assert!(cursor.terminal_is_empty());
}
//#endregion 🧪️RootAuthority

//#region 🧪️CanonicalCopy
#[test]
fn flow_selected_copy_matches_serde_and_shares_unchanged_ordered_roots() {
    let vectors: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🔣️typed-copy.json")).unwrap();
    for grant in [1, 4096] {
        for case in vectors["cases"].as_array().unwrap() {
            let (root, drops) = source();
            let before = serde_json::to_value(root.fixture.as_ref().unwrap()).unwrap();
            let source_pointer = Arc::as_ptr(&root);
            let kind = case["kind"].as_str().unwrap();
            let index = case["index"].as_u64().unwrap() as usize;
            macro_rules! run {
                ($type:ty, $project:expr) => {{
                    let mut cursor = CopyCursor::<Root, $type>::new(root, index, $project, Arc::new(RootFactory), allocation());
                    assert_eq!(cursor.advance(0, grant).unwrap(), None);
                    assert_eq!(cursor.advance(1, 0).unwrap(), None);
                    for _ in 0..200_000 {
                        assert_eq!(Arc::as_ptr(cursor.owned.source.as_ref().unwrap()), source_pointer);
                        let bytes = cursor.advance(1, grant).unwrap().unwrap();
                        assert!(bytes <= grant);
                        if cursor.complete() { break; }
                    }
                    assert!(cursor.complete());
                    let copied = cursor.take().unwrap();
                    assert_eq!(serde_json::to_value(&copied).unwrap(), *before.pointer(case["pointer"].as_str().unwrap()).unwrap());
                    assert_eq!(serde_json::to_value(cursor.owned.source.as_ref().unwrap().fixture.as_ref().unwrap()).unwrap(), before);
                    copied.retire(&mut cursor.owned.retirement);
                    std::thread::spawn(move || { close(&mut cursor, grant); }).join().unwrap();
                }};
            }
            match kind {
                "widget" => run!(Widget, |root, index| root.fixture.as_ref()?.widgets.get(index)),
                "synapse" => run!(SynapseSpec, |root, index| root.fixture.as_ref()?.synapses.get(index)),
                "fixture" => {
                    let mut cursor = FlowFixtureCopy::new(root, index, |root, index| (index == 0).then_some(root.fixture.as_ref()?), Arc::new(RootFactory), allocation());
                    while !cursor.complete() { assert!(cursor.advance(1, grant).unwrap().unwrap() <= grant); }
                    let copied = cursor.take().unwrap();
                    let original = cursor.cursor.owned.source.as_ref().unwrap().fixture.as_ref().unwrap();
                    assert_eq!(serde_json::to_value(&copied).unwrap(), before);
                    for ((left_key, left), (right_key, right)) in copied.layout.iter().zip(original.layout.iter()) {
                        assert!(std::ptr::eq(left_key, right_key) && std::ptr::eq(left, right));
                    }
                    copied.retire(&mut cursor.cursor.owned.retirement);
                    std::thread::spawn(move || close(&mut cursor.cursor, grant)).join().unwrap();
                }
                _ => panic!("unknown shared copy fixture kind"),
            }
            assert_eq!(drops.load(Ordering::SeqCst), 1);
        }
    }
}

#[test]
fn flow_selected_copy_cancellation_and_invalid_projection_preserve_root_until_close() {
    for polls in [0, 1, 4, 25, 4097] {
        let (root, drops) = source();
        let weak = Arc::downgrade(&root);
        let mut cursor = FlowFixtureCopy::new(root, 0, |root, _| root.fixture.as_ref(), Arc::new(RootFactory), allocation());
        for _ in 0..polls { cursor.advance(1, 1).unwrap(); }
        cursor.begin_close();
        assert!(!cursor.complete());
        assert!(cursor.take().is_none());
        assert!(matches!(cursor.close_step(0, 1).unwrap(), SnapshotRetirementStep::Blocked));
        assert!(matches!(cursor.close_step(1, 0).unwrap(), SnapshotRetirementStep::Blocked));
        assert!(weak.upgrade().is_some());
        std::thread::spawn(move || close(&mut cursor.cursor, 1)).join().unwrap();
        assert!(weak.upgrade().is_none());
        assert_eq!(drops.load(Ordering::SeqCst), 1);
    }
    let (root, drops) = source();
    let mut cursor = FlowWidgetCopy::new(root, usize::MAX, |root, index| root.fixture.as_ref()?.widgets.get(index), Arc::new(RootFactory), allocation());
    assert!(cursor.advance(1, 1).is_err());
    assert!(cursor.advance(1, 1).unwrap().is_none());
    assert_eq!(drops.load(Ordering::SeqCst), 0);
    close(&mut cursor.cursor, 1);
    assert_eq!(drops.load(Ordering::SeqCst), 1);
}

#[test]
fn flow_selected_copy_nonterminal_drop_is_guarded_without_destroying_root() {
    let (root, drops) = source();
    let mut cursor = FlowWidgetCopy::new(root, 0, |root, index| root.fixture.as_ref()?.widgets.get(index), Arc::new(RootFactory), allocation());
    cursor.advance(1, 1).unwrap();
    assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| drop(cursor))).is_err());
    assert_eq!(drops.load(Ordering::SeqCst), 0);
    assert!(std::thread::spawn(|| {
        let (root, _) = source();
        let _cursor = FlowFixtureCopy::new(root, 0, |root, _| root.fixture.as_ref(), Arc::new(RootFactory), allocation());
        panic!("primary selected Flow fault");
    }).join().is_err());
}

#[test]
fn flow_selected_copy_rejects_root_retirement_overgrant_and_closes_factory_owner() {
    struct Factory { drops: Arc<AtomicUsize> }
    impl Drop for Factory { fn drop(&mut self) { self.drops.fetch_add(1, Ordering::SeqCst); } }
    struct Adversary { inner: RootRetirement, overgrant: bool }
    impl ErasedSnapshotRetirement for Adversary {
        fn close_step(&mut self, items: usize, bytes: usize) -> Result<SnapshotRetirementStep, String> {
            if self.overgrant { self.overgrant = false; return Ok(SnapshotRetirementStep::Pending { released_items: 2, released_bytes: bytes + 1 }); }
            self.inner.close_step(items, bytes)
        }
        fn terminal_is_empty(&self) -> bool { self.inner.terminal_is_empty() }
    }
    impl SnapshotRetirementFactory<Root> for Factory {
        fn retire(&self, root: Arc<Root>) -> Box<dyn ErasedSnapshotRetirement> {
            Box::new(Adversary { inner: RootRetirement { root: Some(root), retirement: Retirement::default() }, overgrant: true })
        }
    }
    let (root, root_drops) = source();
    let factory_drops = Arc::new(AtomicUsize::new(0));
    let mut cursor = FlowWidgetCopy::new(root, 0, |root, index| root.fixture.as_ref()?.widgets.get(index), Arc::new(Factory { drops: factory_drops.clone() }), allocation());
    cursor.begin_close();
    assert!(matches!(cursor.close_step(1, 1).unwrap(), SnapshotRetirementStep::Pending { .. }));
    assert!(cursor.close_step(1, 1).unwrap_err().contains("exceeded its grant"));
    assert_eq!(root_drops.load(Ordering::SeqCst), 0);
    assert_eq!(factory_drops.load(Ordering::SeqCst), 0);
    close(&mut cursor.cursor, 1);
    assert_eq!(root_drops.load(Ordering::SeqCst), 1);
    assert_eq!(factory_drops.load(Ordering::SeqCst), 1);
}

#[test]
fn flow_selected_copy_allocation_admission_is_separate_and_never_reallocates_payload_pages() {
    for (single, total) in [(1, 1_000_000), (1_000_000, 1)] {
        let (root, drops) = source();
        let mut cursor = FlowWidgetCopy::new(root, 0, |root, index| root.fixture.as_ref()?.widgets.get(index), Arc::new(RootFactory), FlowCopyAllocationBudget::new(single, total));
        let mut failed = false;
        for _ in 0..100 {
            if cursor.advance(1, 1).is_err() { failed = true; break; }
        }
        assert!(failed && !cursor.complete());
        assert!(cursor.allocation().reserved_bytes() <= total);
        close(&mut cursor.cursor, 1);
        assert_eq!(drops.load(Ordering::SeqCst), 1);
    }
    let (root, drops) = source();
    let mut cursor = FlowFixtureCopy::new(root, 0, |root, _| root.fixture.as_ref(), Arc::new(RootFactory), allocation());
    assert_eq!(cursor.allocation().reservation_count(), 0);
    let mut maximum_reservation = std::time::Duration::ZERO;
    while !cursor.complete() {
        let previous = cursor.allocation().reservation_count();
        let started = std::time::Instant::now();
        let copied = cursor.advance(1, 4096).unwrap().unwrap();
        let elapsed = started.elapsed();
        if cursor.allocation().reservation_count() != previous {
            assert_eq!(copied, 0, "uninitialized reservation must not copy source bytes");
            maximum_reservation = maximum_reservation.max(elapsed);
        }
    }
    assert!(cursor.allocation().reserved_bytes() <= 32 * 1024 * 1024);
    eprintln!("[DEBUG] selected Flow fixture allocation count={} admitted-bytes={} maximum-reserve-ns={}", cursor.allocation().reservation_count(), cursor.allocation().reserved_bytes(), maximum_reservation.as_nanos());
    close(&mut cursor.cursor, 4096);
    assert_eq!(drops.load(Ordering::SeqCst), 1);
    let (root, _) = source();
    let source = Rooted { root: root.clone(), pointer: &root.fixture.as_ref().unwrap().schema as *const String };
    let mut task = TextTask { source, bytes: Vec::new(), reserved: false };
    let mut admission = allocation();
    assert!(matches!(task.advance(1, &mut admission), Advance::Bytes(0)));
    let address = task.bytes.as_ptr();
    while task.bytes.len() < task.source.get().len() {
        assert!(matches!(task.advance(1, &mut admission), Advance::Bytes(1)));
        assert_eq!(address, task.bytes.as_ptr());
    }
    let mut retirement = Retirement::default();
    Box::new(task).retire(&mut retirement);
    retirement.retire_cold();
    let mut root_retirement = RootFactory.retire(root);
    while !matches!(root_retirement.close_step(1, 4096).unwrap(), SnapshotRetirementStep::Complete) {}
}
//#endregion 🧪️CanonicalCopy
