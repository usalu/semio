/// 🎟️ The language-agnostic terminal vacancy vector: every admitted snapshot read slot is vacated exactly once on every terminal path.
const TERMINAL_VACANCY_FIXTURE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../🧫️fixtures/snapshot-read-terminal-vacancy-v1/🔣️.json"));
const TERMINAL_VACANCY_TEXT: &str = "terminal-vacancy-root";

type VacancyRegistry = semio_framework_value_snapshot_read::SnapshotReadRegistryHandle;

fn vacancy_pump(registry: &VacancyRegistry) -> usize {
    let mut pumped = 0;
    for _ in 0..4 * SNAPSHOT_READ_LEASE_CAPACITY {
        if !registry.has_returned() { break; }
        drop(take_one_returned::<String>(registry).expect("the pump admits a returned root").expect("a returned root waits"));
        pumped += 1;
    }
    pumped
}

fn vacancy_issue(registry: &VacancyRegistry, alias: Option<&Arc<String>>) -> (Arc<String>, semio_framework_value_snapshot_read::SnapshotReadLease) {
    let owner = alias.cloned().unwrap_or_else(|| Arc::new(TERMINAL_VACANCY_TEXT.to_string()));
    let lease = registry.try_issue(owner.clone()).expect("an empty or pumped table admits a read");
    (owner, lease)
}

fn vacancy_end(registry: &VacancyRegistry, terminal: &str, alias: Option<&Arc<String>>) {
    use semio_framework_value_snapshot_read::SnapshotRead;
    match terminal {
        "complete" => {
            let (owner, lease) = vacancy_issue(registry, alias);
            let read = ErasedSnapshotRead::new(owner, lease);
            drop(match read.into_typed::<String>(registry) { Ok(owner) => owner, Err(_) => panic!("the exact owner completes its read") });
        }
        "cancel" => {
            let (owner, lease) = vacancy_issue(registry, alias);
            drop(SnapshotRead::new(owner, lease));
        }
        "bare-lease" => {
            let (owner, lease) = vacancy_issue(registry, alias);
            drop((owner, lease));
        }
        "refuse" => {
            let guard = registry.try_lock_state().expect("hold the issuer busy");
            let (reason, original) = match registry.try_issue_original(Arc::new(TERMINAL_VACANCY_TEXT.to_string())) { Err(refusal) => refusal, Ok(_) => panic!("a busy issuer admits nothing") };
            assert!(matches!(reason, semio_framework_value_snapshot_read::SnapshotReadLeaseRefusal::Busy));
            drop((original, guard));
        }
        "fault" => {
            let (owner, lease) = vacancy_issue(registry, alias);
            let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
                let _read = SnapshotRead::new(owner, lease);
                panic!("injected fault while a read is held");
            }));
            assert!(outcome.is_err());
        }
        "supersede" => {
            let (first, first_lease) = vacancy_issue(registry, alias);
            let mut held = Some(SnapshotRead::new(first, first_lease));
            let (second, second_lease) = vacancy_issue(registry, alias);
            held = Some(SnapshotRead::new(second, second_lease));
            assert!(held.take().expect("the superseding read is held").return_to_registry());
        }
        other => panic!("unknown terminal {other}"),
    }
}

/// 🎟️ LAW: cycling read admission through every terminal for several times the table capacity never exhausts it; once the returned reads are pumped the table is empty and every slot admits again.
#[test]
fn every_terminal_of_a_read_vacates_its_slot_exactly_once() {
    let fixture: serde_json::Value = serde_json::from_str(TERMINAL_VACANCY_FIXTURE).expect("terminal vacancy fixture");
    assert_eq!(fixture["capacity"].as_u64().unwrap() as usize, SNAPSHOT_READ_LEASE_CAPACITY);
    let terminals: Vec<&str> = fixture["terminals"].as_array().unwrap().iter().map(|terminal| terminal.as_str().unwrap()).collect();
    let cycles = fixture["capacityMultiples"].as_u64().unwrap() as usize * SNAPSHOT_READ_LEASE_CAPACITY + terminals.len();
    let (pump_every, alias_every) = (fixture["pumpEvery"].as_u64().unwrap() as usize, fixture["aliasHeldEvery"].as_u64().unwrap() as usize);
    let registry = VacancyRegistry::new();
    let shared = Arc::new(TERMINAL_VACANCY_TEXT.to_string());
    for cycle in 0..cycles {
        let terminal = terminals[cycle % terminals.len()];
        vacancy_end(&registry, terminal, (cycle % alias_every == 0).then_some(&shared));
        assert!(registry.occupied_count() <= 2 * pump_every, "cycle {cycle} ({terminal}) left {} slots admitted", registry.occupied_count());
        if cycle % pump_every == pump_every - 1 { vacancy_pump(&registry); }
    }
    vacancy_pump(&registry);
    assert_eq!(registry.occupied_count(), fixture["expected"]["occupiedAfterPump"].as_u64().unwrap() as usize);
    assert_eq!(registry.returned_count(), fixture["expected"]["returnedAfterPump"].as_u64().unwrap() as usize);
    assert!(registry.terminal_is_empty());
    let leases: Vec<_> = (0..fixture["expected"]["nextAdmissions"].as_u64().unwrap()).map(|_| registry.try_issue(Arc::new(TERMINAL_VACANCY_TEXT.to_string())).expect("the next read admits")).collect();
    assert_eq!(leases.len(), SNAPSHOT_READ_LEASE_CAPACITY);
    assert!(registry.try_issue(Arc::new(TERMINAL_VACANCY_TEXT.to_string())).is_err(), "the table holds exactly its capacity");
    drop(leases);
    vacancy_pump(&registry);
    assert!(registry.terminal_is_empty());
}

/// 🔁️ LAW: returning a lease twice never vacates a slot twice, and a lease dropped after its slot already left is a no-op.
#[test]
fn a_lease_is_returned_exactly_once() {
    let registry = VacancyRegistry::new();
    let (owner, mut lease) = vacancy_issue(&registry, None);
    assert!(lease.return_now());
    assert!(!lease.return_now(), "a second return of the same lease is refused");
    drop(owner);
    drop(lease);
    vacancy_pump(&registry);
    let (fresh_owner, fresh) = vacancy_issue(&registry, None);
    assert_eq!(registry.occupied_count(), 1);
    drop((fresh_owner, fresh));
    vacancy_pump(&registry);
    assert!(registry.terminal_is_empty());
}

/// 🌡️ LAW: returned reads that wait for their pump raise the issuer's pressure exactly at an eighth of its table, and draining them clears it.
#[test]
fn returned_reads_raise_pressure_at_an_eighth_of_the_table_and_the_pump_clears_it() {
    let fixture: serde_json::Value = serde_json::from_str(TERMINAL_VACANCY_FIXTURE).expect("terminal vacancy fixture");
    let threshold = fixture["returnPressure"].as_u64().unwrap() as usize;
    assert_eq!(threshold, semio_framework_value_snapshot_read::SNAPSHOT_READ_RETURN_PRESSURE);
    let registry = VacancyRegistry::new();
    for _ in 0..threshold - 1 { vacancy_end(&registry, "cancel", None); }
    assert!(!registry.returned_under_pressure());
    vacancy_end(&registry, "cancel", None);
    assert_eq!(registry.returned_count(), threshold);
    assert!(registry.returned_under_pressure());
    assert_eq!(vacancy_pump(&registry), threshold);
    assert!(!registry.returned_under_pressure() && registry.terminal_is_empty());
}

/// 🧯️ LAW: an admission that consumes its returned original and then faults or refuses leaves its slot vacated exactly once, never occupied with nothing inside.
#[test]
fn a_faulted_returned_admission_that_consumed_its_original_still_vacates_the_slot() {
    for consumed_without_publishing in [false, true] {
        let registry = VacancyRegistry::new();
        let (owner, lease) = vacancy_issue(&registry, None);
        drop(semio_framework_value_snapshot_read::SnapshotRead::new(owner, lease));
        assert_eq!(registry.returned_count(), 1);
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 0, maximum_capacity_bytes: 0, maximum_release_bytes: 0, maximum_depth: 1 };
        let outcome = effect_tests::with_effect(grant, effect_tests::CPU, |effect| registry.try_admit_one_returned::<String, ()>(effect, |original, _| {
            drop(original.take());
            if consumed_without_publishing { semio_framework_value::RetainedWorkOutcome { result: Ok(None), work: 1 } } else { semio_framework_value::RetainedWorkOutcome { result: Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::AllocationFailed, "injected admission fault").into()), work: 1 } }
        })).0.result;
        assert!(outcome.is_err(), "the consumed admission reports its fault");
        assert_eq!((registry.returned_count(), registry.occupied_count()), (0, 0), "the faulted slot is vacated exactly once");
        assert!(registry.terminal_is_empty());
        assert!(registry.try_issue(Arc::new(TERMINAL_VACANCY_TEXT.to_string())).is_ok(), "the next read admits");
        vacancy_pump(&registry);
    }
}
