//! 🪪️ Exercises the private mint through actual retained EventQueue roots.

use super::{EventQueue, InputGeneration};
use super::input_root::{InputRootFault, InputRootSequence};
use std::sync::{atomic::{AtomicU64, Ordering}, Barrier};

//#region 🔎️AllocationObservation
struct ObservedAllocator;
thread_local! {
    static OBSERVE: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    static ALLOCATIONS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    static INTERFERE: std::cell::Cell<Option<u64>> = const { std::cell::Cell::new(None) };
}
unsafe impl std::alloc::GlobalAlloc for ObservedAllocator {
    unsafe fn alloc(&self, layout: std::alloc::Layout) -> *mut u8 {
        OBSERVE.with(|enabled| if enabled.get() { ALLOCATIONS.with(|count| count.set(count.get() + 1)); });
        unsafe { std::alloc::GlobalAlloc::alloc(&std::alloc::System, layout) }
    }
    unsafe fn dealloc(&self, pointer: *mut u8, layout: std::alloc::Layout) {
        unsafe { std::alloc::GlobalAlloc::dealloc(&std::alloc::System, pointer, layout) }
    }
}
#[global_allocator]
static ALLOCATOR: ObservedAllocator = ObservedAllocator;

pub(super) fn interfere_after_load(counter: &AtomicU64) {
    INTERFERE.with(|pending| if let Some(value) = pending.take() { counter.store(value, Ordering::SeqCst); });
}
fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("🔣️.json")).unwrap() }
fn number(value: &serde_json::Value) -> u64 { value.as_str().unwrap().parse().unwrap() }
fn grant() -> usize { std::mem::size_of::<EventQueue>() }
pub(super) fn allocations_start() {
    ALLOCATIONS.with(|value| value.set(0));
    OBSERVE.with(|value| value.set(true));
}
pub(super) fn allocations_end() -> usize {
    OBSERVE.with(|value| value.set(false));
    ALLOCATIONS.with(std::cell::Cell::get)
}
//#endregion 🔎️AllocationObservation

//#region 🧪️InputRoots
#[test]
fn input_root_native_vectors_refuse_before_mint_or_allocation() {
    let fixture = fixture();
    for row in fixture["cases"].as_array().unwrap() {
        let before = number(&row["before"]);
        let at_cas = number(&row["atCas"]);
        let sequence = InputRootSequence { last: AtomicU64::new(before) };
        let bytes = match row["grant"].as_str().unwrap() { "zero" => 0, "short" => grant() - 1, "admitted" => grant(), _ => unreachable!() };
        INTERFERE.with(|pending| pending.set((before != at_cas).then_some(at_cas)));
        allocations_start();
        let mut queue = EventQueue::new();
        let result = queue.try_admit_root_with(&sequence, bytes);
        let allocations = allocations_end();
        let outcome = match result { Ok(false) => "blocked", Ok(true) => "admitted", Err(InputRootFault::Busy) => "busy", Err(InputRootFault::Exhausted) => "exhausted" };
        assert_eq!(outcome, row["outcome"].as_str().unwrap(), "{}", row["name"]);
        assert_eq!(sequence.last.load(Ordering::SeqCst), number(&row["after"]));
        assert_eq!(queue.root.map(|root| root.get()), row["root"].as_str().map(|root| root.parse::<u64>().unwrap()));
        assert_eq!(allocations, 0, "root admission must not allocate queue backing");
        assert_eq!(queue.discrete.capacity(), 0);
        assert_eq!(queue.current_generation(), InputGeneration(0));
        if let Some(root) = queue.root {
            let hex: String = root.get().to_le_bytes().iter().map(|byte| format!("{byte:02x}")).collect();
            assert_eq!(hex, row["rootLeHex"].as_str().unwrap());
        }
        INTERFERE.with(|pending| pending.set(None));
    }
}

#[test]
fn input_root_native_equal_counters_moves_and_address_reuse_never_reuse_identity() {
    let fixture = fixture();
    let sequence = InputRootSequence::new();
    let mut first = EventQueue::new();
    let mut second = EventQueue::new();
    assert!(first.try_admit_root_with(&sequence, grant()).unwrap());
    assert!(second.try_admit_root_with(&sequence, grant()).unwrap());
    assert_eq!(first.current_generation(), second.current_generation());
    assert_eq!(first.root.unwrap().get(), number(&fixture["reuse"]["firstRoot"]));
    assert_eq!(second.root.unwrap().get(), number(&fixture["reuse"]["secondRoot"]));
    assert_ne!(first.root, second.root);
    let original = first.root;
    let moved = (17u8, first);
    assert_eq!(moved.1.root, original);
    let address = &second as *const EventQueue;
    let old = std::mem::replace(&mut second, EventQueue::new());
    assert_eq!(&second as *const EventQueue, address);
    assert!(second.try_admit_root_with(&sequence, grant()).unwrap());
    assert_ne!(second.root, old.root);
    assert_ne!(second.root, moved.1.root);
    assert_eq!(sequence.last.load(Ordering::SeqCst), 3);
}

#[test]
fn input_root_native_installed_owner_survives_actual_admission_unwind() {
    let fixture = fixture();
    let sequence = InputRootSequence::new();
    let mut queue = EventQueue::new();
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        assert!(queue.try_admit_root_with(&sequence, grant()).unwrap());
        panic!("[DEBUG] after actual queue root installation");
    }));
    assert!(result.is_err());
    drop(result);
    let original = queue.root;
    assert_eq!(original.unwrap().get(), number(&fixture["failureAfterInstall"]["retainedRoot"]));
    assert!(queue.try_admit_root_with(&sequence, grant()).unwrap());
    assert_eq!(queue.root, original);
    assert_eq!(sequence.last.load(Ordering::SeqCst), original.unwrap().get());
    let mut next = EventQueue::new();
    assert!(next.try_admit_root_with(&sequence, grant()).unwrap());
    assert_eq!(next.root.unwrap().get(), number(&fixture["failureAfterInstall"]["nextRoot"]));
    assert_eq!(queue.discrete.capacity(), 0);
}

#[test]
fn input_root_native_concurrent_single_attempts_preserve_busy_roots() {
    let fixture = fixture();
    let workers = fixture["concurrent"]["workers"].as_u64().unwrap() as usize;
    let attempts = fixture["concurrent"]["attemptsPerWorker"].as_u64().unwrap() as usize;
    let sequence = InputRootSequence::new();
    let barrier = Barrier::new(workers);
    let results = std::thread::scope(|scope| {
        let mut handles = Vec::new();
        for _ in 0..workers {
            let sequence = &sequence;
            let barrier = &barrier;
            handles.push(scope.spawn(move || {
                let mut results = Vec::new();
                for _ in 0..attempts {
                    let mut queue = EventQueue::new();
                    barrier.wait();
                    let result = queue.try_admit_root_with(sequence, grant());
                    results.push((result, queue.root, queue.discrete.capacity()));
                }
                results
            }));
        }
        handles.into_iter().flat_map(|handle| handle.join().unwrap()).collect::<Vec<_>>()
    });
    let attempted = results.len();
    let mut successful = Vec::new();
    for (result, root, capacity) in results {
        assert_eq!(capacity, 0);
        match result {
            Ok(true) => successful.push(root.unwrap().get()),
            Err(InputRootFault::Busy) => assert!(root.is_none()),
            other => panic!("unexpected single-attempt result {other:?}"),
        }
    }
    let accepted = successful.len();
    successful.sort_unstable();
    successful.dedup();
    assert_eq!(attempted as u64, fixture["concurrent"]["attempts"].as_u64().unwrap());
    assert_eq!(accepted, successful.len());
    assert!(accepted as u64 >= fixture["concurrent"]["minimumSuccesses"].as_u64().unwrap());
    assert!(accepted as u64 <= fixture["concurrent"]["maximumSuccesses"].as_u64().unwrap());
    assert_eq!(sequence.last.load(Ordering::SeqCst), accepted as u64);
    assert_eq!(successful, (1..=accepted as u64).collect::<Vec<_>>());
    eprintln!("[DEBUG] input root single-attempt concurrency attempts={attempted} accepted={accepted} busy={}", attempted - accepted);
}

#[test]
fn input_root_native_permanent_exhaustion_and_exact_fixed_layout() {
    let fixture = fixture();
    let sequence = InputRootSequence { last: AtomicU64::new(u64::MAX - 1) };
    let mut last = EventQueue::new();
    assert!(last.try_admit_root_with(&sequence, grant()).unwrap());
    assert_eq!(last.root.unwrap().get(), u64::MAX);
    for _ in 0..3 {
        let mut refused = EventQueue::new();
        assert_eq!(refused.try_admit_root_with(&sequence, grant()), Err(InputRootFault::Exhausted));
        assert!(refused.root.is_none());
        assert_eq!(refused.discrete.capacity(), 0);
        assert_eq!(sequence.last.load(Ordering::SeqCst), u64::MAX);
    }
    let atomic = std::mem::size_of::<InputRootSequence>();
    assert_eq!(atomic as u64, fixture["storage"]["nativeAtomicBytes"].as_u64().unwrap());
    assert_eq!(atomic, std::mem::size_of::<AtomicU64>());
    assert!(!std::mem::needs_drop::<InputRootSequence>());
    assert_eq!(std::mem::size_of_val(&last.root), std::mem::size_of::<u64>());
    eprintln!("[DEBUG] input root static={atomic} queue={} rootField={} queueBacking={}", grant(), std::mem::size_of_val(&last.root), last.discrete.capacity());
}
//#endregion 🧪️InputRoots
