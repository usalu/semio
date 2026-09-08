
use super::*;

#[derive(Debug)]
struct Owner {
    identity: u64,
    fixture_label: Option<&'static str>,
    bytes: Vec<u8>,
    cancelled: bool,
    closing: bool,
}

impl Owner {
    fn new(identity: u64, bytes: usize) -> Self {
        Self { identity, fixture_label: None, bytes: vec![0; bytes], cancelled: false, closing: false }
    }

    fn fixture(identity: u64, label: &'static str, bytes: usize) -> Self {
        Self { identity, fixture_label: Some(label), bytes: vec![0; bytes], cancelled: false, closing: false }
    }

    fn close_all(&mut self) {
        self.begin_close();
        for _ in 0..16 {
            let _ = self.close_step(1, 1);
            if self.terminal_is_empty() {
                return;
            }
        }
        panic!("fixture owner did not close within its declared fixed bound");
    }
}

impl FixedOperationOwner for Owner {
    fn retained_bytes(&self) -> usize {
        self.bytes.len()
    }

    fn cancel(&mut self) {
        self.cancelled = true;
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> InteractiveJobCloseStep {
        if !self.closing || maximum_items == 0 || maximum_bytes == 0 {
            return InteractiveJobCloseStep::Blocked;
        }
        if self.bytes.pop().is_some() {
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 1 };
        }
        InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.bytes.is_empty()
    }
}

impl Drop for Owner {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "fixture owner was dropped before terminal-empty");
    }
}

fn drain<const CAPACITY: usize>(registry: &mut FixedOperationRegistry<Owner, CAPACITY>) {
    for _ in 0..64 {
        let _ = registry.close_step(1, 1);
        if registry.is_empty() {
            return;
        }
    }
    panic!("fixed operation registry did not close within capacity × owner bound");
}

fn fixture_admit<const CAPACITY: usize>(registry: &mut FixedOperationRegistry<Owner, CAPACITY>, output: &mut Vec<String>, operation: OperationId, generation: Generation, bytes: usize, identity: u64, label: &'static str) {
    match registry.admit(FixedOperationKey::new(operation, generation), Owner::fixture(identity, label, bytes)) {
        Ok(()) => output.push(format!("admit:accepted:{label}")),
        Err(mut rejected) => {
            assert_eq!(rejected.owner.identity, identity);
            assert_eq!(rejected.owner.fixture_label, Some(label));
            output.push(format!("admit:rejected:{label}"));
            rejected.owner.close_all();
        }
    }
}

fn fixture_take<const CAPACITY: usize>(registry: &mut FixedOperationRegistry<Owner, CAPACITY>, output: &mut Vec<String>, operation: OperationId, generation: Generation) {
    match registry.take(FixedOperationKey::new(operation, generation)) {
        Some(mut owner) => {
            output.push(format!("take:{}", owner.fixture_label.expect("fixture owner label")));
            owner.close_all();
        }
        None => output.push("take:none".into()),
    }
}

fn fixture_cancel<const CAPACITY: usize>(registry: &mut FixedOperationRegistry<Owner, CAPACITY>, output: &mut Vec<String>, operation: OperationId, generation: Generation) {
    output.push(format!("cancel:{}", registry.cancel(FixedOperationKey::new(operation, generation))));
}

fn fixture_cancel_stale<const CAPACITY: usize>(registry: &mut FixedOperationRegistry<Owner, CAPACITY>, output: &mut Vec<String>, operation: OperationId, live_generation: Generation) {
    output.push(format!("stale:{}", registry.cancel_stale_step(operation, live_generation)));
}

fn fixture_close<const CAPACITY: usize>(registry: &mut FixedOperationRegistry<Owner, CAPACITY>, output: &mut Vec<String>, maximum_items: usize, maximum_bytes: usize) {
    let state = match registry.close_step(maximum_items, maximum_bytes) {
        InteractiveJobCloseStep::Blocked => "blocked",
        InteractiveJobCloseStep::Pending { .. } => "pending",
        InteractiveJobCloseStep::Complete => "complete",
    };
    output.push(format!("close:{state}"));
}

fn fixture_inspect<const CAPACITY: usize>(registry: &FixedOperationRegistry<Owner, CAPACITY>, output: &mut Vec<String>) {
    let remaining = registry.slots.iter().filter_map(Option::as_ref).map(|entry| entry.owner.bytes.len()).sum::<usize>();
    output.push(format!("state:{}:{}:{remaining}", registry.occupied, registry.retained_bytes));
}

fn fixture_assert<const CAPACITY: usize>(id: &str, registry: &mut FixedOperationRegistry<Owner, CAPACITY>, output: Vec<String>, expected: &[&str]) {
    assert_eq!(output, expected, "language-neutral fixed operation case {id}");
    assert!(registry.is_empty(), "language-neutral fixed operation case {id} retained an owner");
}

include!("../🧪️fixed-operation-registry-cases/🦀️.rs");

#[test]
fn maximum_plus_one_and_saturation_return_the_exact_owner() {
    let mut registry = FixedOperationRegistry::<Owner, 2>::new(4);
    let key = FixedOperationKey::new(OperationId(1), Generation(7));
    registry.admit(key, Owner::new(11, 4)).expect("exact maximum");
    let mut byte_rejected = registry.admit(FixedOperationKey::new(OperationId(2), Generation(7)), Owner::new(12, 1)).expect_err("maximum plus one");
    assert_eq!(byte_rejected.owner.identity, 12);
    assert_eq!(byte_rejected.owner.bytes.len(), 1);
    byte_rejected.owner.close_all();
    let mut collision_rejected = registry.admit(FixedOperationKey::new(OperationId(3), Generation(7)), Owner::new(13, 0)).expect_err("fixed-slot collision");
    assert_eq!(collision_rejected.owner.identity, 13);
    collision_rejected.owner.close_all();
    assert!(registry.cancel(key));
    drain(&mut registry);
    assert_eq!(registry.retained_bytes(), 0);

    let mut full = FixedOperationRegistry::<Owner, 2>::new(8);
    let first = FixedOperationKey::new(OperationId(0), Generation(0));
    let second = FixedOperationKey::new(OperationId(1), Generation(0));
    full.admit(first, Owner::new(14, 1)).expect("first distinct slot");
    full.admit(second, Owner::new(15, 1)).expect("exact fixed capacity");
    let mut capacity_rejected = full.admit(FixedOperationKey::new(OperationId(2), Generation(0)), Owner::new(16, 1)).expect_err("fixed capacity plus one");
    assert_eq!(capacity_rejected.owner.identity, 16);
    capacity_rejected.owner.close_all();
    assert!(full.cancel(first));
    assert!(full.cancel(second));
    drain(&mut full);
}

#[test]
fn stale_generation_interrupted_close_and_aba_preserve_exact_authority() {
    let mut registry = FixedOperationRegistry::<Owner, 4>::new(8);
    let stale = FixedOperationKey::new(OperationId(9), Generation(1));
    registry.admit(stale, Owner::new(21, 2)).expect("stale owner");
    let (observed_key, observed) = registry.get_operation(OperationId(9)).expect("operation-owned lookup");
    assert_eq!(observed_key, stale);
    assert_eq!(observed.identity, 21);
    let (_, observed_mut) = registry.get_operation_mut(OperationId(9)).expect("mutable operation-owned lookup");
    observed_mut.identity = 23;
    assert_eq!(registry.get(stale).expect("same exact owner").identity, 23);
    assert!(registry.get_operation(OperationId(10)).is_none(), "another operation cannot observe the retained owner");
    assert!(registry.take(FixedOperationKey::new(OperationId(9), Generation(2))).is_none());
    for _ in 0..4 {
        let _ = registry.cancel_stale_step(OperationId(9), Generation(2));
    }
    let _ = registry.close_step(1, 1);
    assert!(!registry.is_empty(), "interrupted close must retain the exact owner");
    drain(&mut registry);
    let fresh = FixedOperationKey::new(OperationId(9), Generation(2));
    registry.admit(fresh, Owner::new(22, 0)).expect("fresh ABA generation");
    assert!(registry.take(stale).is_none());
    let mut owner = registry.take(fresh).expect("exact accepted owner handback");
    assert_eq!(owner.identity, 22);
    owner.close_all();
    assert!(matches!(registry.close_step(1, 1), InteractiveJobCloseStep::Complete));
    assert!(matches!(registry.close_step(1, 1), InteractiveJobCloseStep::Complete));
}

#[test]
fn maximum_registry_backing_initializes_inside_one_interactive_ceiling_under_concurrent_load() {
    const WORKERS: usize = 4;
    const SAMPLES: usize = 31;
    let barrier = Arc::new(std::sync::Barrier::new(WORKERS));
    let mut workers = Vec::with_capacity(WORKERS);
    for _ in 0..WORKERS {
        let barrier = barrier.clone();
        workers.push(std::thread::spawn(move || {
            barrier.wait();
            let mut elapsed = [0_u128; SAMPLES];
            for sample in &mut elapsed {
                let started = Instant::now();
                let registry = FixedOperationRegistry::<Owner, 64>::new(4_096);
                *sample = started.elapsed().as_micros();
                assert!(registry.allocation_admitted);
                drop(registry);
            }
            elapsed.sort_unstable();
            elapsed[SAMPLES / 2]
        }));
    }
    for worker in workers {
        let median = worker.join().expect("concurrent fixed registry initialization worker");
        assert!(median < u128::from(semio_framework_trace::INTERACTIVE_STEP_CEILING_US), "fixed registry median backing initialization exceeded the interactive ceiling under concurrent load: {median}us");
    }
}
