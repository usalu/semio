/// 🧠️ Laws of guest residency bounds, the verification memory, the residency ledger and off-worker guest codec calls.

/// 🧊️ The residency a hub applies unless configured is exactly the `default` and `const` values
/// `TrustedCatalogGuestResidencyV1` declares, an operator's `residentComponentBytes` is admitted only within the
/// declared bounds, a catalog load never verifies more guests at once than the declared bound, and the state the
/// admin route reports carries exactly the fields `TrustedCatalogGuestResidencyStateV1` requires.
#[test]
fn guest_residency_bounds_are_the_declared_schema_values() {
    let module: serde_json::Value = serde_json::from_str(semio_hub_trusted_catalog_contract::schema::TRUSTED_CATALOG_SCHEMA_JSON).unwrap();
    let residency = &module["$defs"]["TrustedCatalogGuestResidencyV1"];
    let declared = &residency["properties"];
    let bytes = &declared["residentComponentBytes"];
    assert_eq!(bytes["default"].as_u64(), Some(TRUSTED_CATALOG_GUEST_RESIDENCY.resident_component_bytes));
    assert_eq!(bytes["minimum"].as_u64(), Some(*semio_hub_trusted_catalog_contract::schema::TRUSTED_CATALOG_GUEST_RESIDENCY_BYTES_BOUNDS.start()));
    assert_eq!(bytes["maximum"].as_u64(), Some(*semio_hub_trusted_catalog_contract::schema::TRUSTED_CATALOG_GUEST_RESIDENCY_BYTES_BOUNDS.end()));
    assert_eq!(declared["concurrentVerifications"]["const"].as_u64(), Some(TRUSTED_CATALOG_GUEST_RESIDENCY.concurrent_verifications as u64));
    assert_eq!(declared["accessCountCeiling"]["const"].as_u64(), Some(u64::from(TRUSTED_CATALOG_GUEST_RESIDENCY.access_count_ceiling)));
    assert_eq!(declared["accessCountAgingPerGuest"]["const"].as_u64(), Some(TRUSTED_CATALOG_GUEST_RESIDENCY.access_count_aging_per_guest));
    assert_eq!(residency["required"].as_array().unwrap().len(), declared.as_object().unwrap().len(), "every declared bound is required");
    assert!(guest_verification_concurrency() >= 1 && guest_verification_concurrency() <= TRUSTED_CATALOG_GUEST_RESIDENCY.concurrent_verifications);
    let configured = |value: Option<&str>| TrustedCatalogGuestResidencyV1::configured(value).map(|residency| residency.resident_component_bytes);
    assert_eq!(configured(None), Ok(TRUSTED_CATALOG_GUEST_RESIDENCY.resident_component_bytes));
    assert_eq!(configured(Some(" ")), Ok(TRUSTED_CATALOG_GUEST_RESIDENCY.resident_component_bytes));
    assert_eq!(configured(Some("0")), Ok(0));
    assert_eq!(configured(Some("67108864")), Ok(67_108_864));
    assert_eq!(configured(Some("17179869184")), Ok(17_179_869_184));
    for refused in ["17179869185", "-1", "64MiB", "1e9", "0x10"] {
        assert!(configured(Some(refused)).is_err(), "{refused} is refused");
    }
    let state = serde_json::to_value(TrustedCatalogGuestResidencyStateV1::default()).unwrap();
    let required: BTreeSet<&str> = module["$defs"]["TrustedCatalogGuestResidencyStateV1"]["required"].as_array().unwrap().iter().map(|field| field.as_str().unwrap()).collect();
    assert_eq!(state.as_object().unwrap().keys().map(String::as_str).collect::<BTreeSet<_>>(), required, "the reported state is exactly the schema's fields");
}

/// 🗃️ A remembered verification answers only for exactly its component, schema and engine; a record
/// edited to claim another hash, another engine or another component is a miss, never a verdict.
#[tokio::test]
async fn a_guest_codec_verification_is_recalled_only_for_its_exact_key() {
    let root = std::env::temp_dir().join(format!("guest-codec-verifications-{}-{}", std::process::id(), FIXTURE_SEQUENCE.fetch_add(1, Ordering::SeqCst)));
    let cache = GuestCodecVerificationCacheV1::at(root.clone(), "owned-engine-v1:aa:fuel-1");
    let component = [7u8; 32];
    let observed = [9u8; 32];
    assert_eq!(cache.recall(&component, "note.document").await, None, "a cold cache recalls nothing");
    assert!(cache.remember(&component, "note.document", &observed).await);
    assert_eq!(cache.recall(&component, "note.document").await, Some(observed));
    assert_eq!(cache.recall(&component, "draw.document").await, None, "another schema is another key");
    assert_eq!(cache.recall(&[8u8; 32], "note.document").await, None, "another component is another key");
    assert_eq!(GuestCodecVerificationCacheV1::at(root.clone(), "owned-engine-v1:ab:fuel-1").recall(&component, "note.document").await, None, "a changed engine verifies afresh");
    assert_eq!(GuestCodecVerificationCacheV1::disabled().recall(&component, "note.document").await, None);
    let file = std::fs::read_dir(&root).unwrap().next().unwrap().unwrap().path();
    let module: serde_json::Value = serde_json::from_str(semio_hub_trusted_catalog_contract::schema::TRUSTED_CATALOG_SCHEMA_JSON).unwrap();
    let stored: serde_json::Value = serde_json::from_slice(&std::fs::read(&file).unwrap()).unwrap();
    let required: Vec<&str> = module["$defs"]["GuestCodecVerificationV1"]["required"].as_array().unwrap().iter().map(|field| field.as_str().unwrap()).collect();
    assert_eq!(stored.as_object().unwrap().keys().map(String::as_str).collect::<BTreeSet<_>>(), required.iter().copied().collect(), "the record is exactly the schema's fields");
    let forged = String::from_utf8(std::fs::read(&file).unwrap()).unwrap().replace(&"09".repeat(32), &"0a".repeat(32));
    std::fs::write(&file, forged).unwrap();
    assert_eq!(cache.recall(&component, "note.document").await, Some([10u8; 32]), "a record is only a memory; the loader compares it with the trust record");
    std::fs::write(&file, b"{not json").unwrap();
    assert_eq!(cache.recall(&component, "note.document").await, None, "an unreadable record is a miss");
    std::fs::remove_dir_all(&root).unwrap();
}

/// 🪪️ A hub's verification memory is keyed by the owned engine and lives inside its trusted catalog:
/// any build of the same engine at the same data root — another path, a copy, a re-signed binary —
/// recalls what one verified, and copying `trusted-catalog/` carries the memory with it.
#[tokio::test]
async fn a_guest_codec_verification_is_keyed_by_the_engine_and_travels_with_the_catalog() {
    let data = std::env::temp_dir().join(format!("guest-codec-engine-{}-{}", std::process::id(), FIXTURE_SEQUENCE.fetch_add(1, Ordering::SeqCst)));
    let engine = guest_codec_engine_identity();
    assert!(engine.starts_with("owned-engine-v1:") && engine.ends_with(&format!(":fuel-{}", GUEST_CODEC_BUDGET.fuel)), "{engine}");
    assert_eq!(engine, guest_codec_engine_identity(), "the identity is a pure function of the engine");
    let component = [3u8; 32];
    let observed = [5u8; 32];
    assert!(GuestCodecVerificationCacheV1::beside(&data).remember(&component, "note.document", &observed).await);
    assert!(data.join("trusted-catalog").join("guest-codec-verifications").is_dir());
    assert_eq!(GuestCodecVerificationCacheV1::beside(&data).recall(&component, "note.document").await, Some(observed));
    let copied = data.with_extension("copy");
    std::fs::create_dir_all(copied.join("trusted-catalog")).unwrap();
    std::fs::rename(data.join("trusted-catalog").join("guest-codec-verifications"), copied.join("trusted-catalog").join("guest-codec-verifications")).unwrap();
    assert_eq!(GuestCodecVerificationCacheV1::beside(&copied).recall(&component, "note.document").await, Some(observed), "the memory travels with the catalog tree");
    assert_eq!(GuestCodecVerificationCacheV1::beside(&data).recall(&component, "note.document").await, None);
    std::fs::remove_dir_all(&data).unwrap();
    std::fs::remove_dir_all(&copied).unwrap();
}

/// 🧊️ A residency ledger over byte-charged test values, `budget` bytes.
fn residency_ledger(budget: u64) -> Arc<GuestResidencyLedgerV1<Vec<u8>>> {
    GuestResidencyLedgerV1::new(TrustedCatalogGuestResidencyV1 { resident_component_bytes: budget, ..TRUSTED_CATALOG_GUEST_RESIDENCY })
}

/// 🧪️ One operation's `calls` codec calls on `slot`; a compile answers the SHA-256 of the slot's own `source`.
async fn residency_operation(control: &TestControl, slot: &GuestResidencyV1<Vec<u8>>, source: &[u8], calls: usize, compiles: &AtomicUsize) -> Vec<u8> {
    let context = control.context();
    let mut answers = Vec::with_capacity(calls);
    for _ in 0..calls {
        let value = slot
            .acquire(&context, || async {
                compiles.fetch_add(1, Ordering::SeqCst);
                Ok::<_, AuthorityError>(Sha256::digest(source).to_vec())
            })
            .await
            .unwrap();
        answers.push(value.as_ref().clone());
    }
    assert!(answers.windows(2).all(|pair| pair[0] == pair[1]), "every call of one operation answers the same guest");
    answers.pop().unwrap()
}

/// 🧊️ A compiled guest that fits stays resident, a resident guest serves later operations without compiling
/// again, and idleness alone releases nothing.
#[tokio::test]
async fn a_compiled_guest_that_fits_stays_resident_and_idleness_releases_nothing() {
    let (control, compiles) = (TestControl::new(), AtomicUsize::new(0));
    let ledger = residency_ledger(2_000);
    let (a, b) = (ledger.register(1_000), ledger.register(1_000));
    residency_operation(&control, &a, b"a", 1, &compiles).await;
    residency_operation(&control, &b, b"b", 1, &compiles).await;
    residency_operation(&control, &a, b"a", 1, &compiles).await;
    tokio::time::sleep(std::time::Duration::from_millis(20)).await;
    assert!(a.is_resident() && b.is_resident(), "idleness alone releases nothing");
    assert_eq!(ledger.state(), TrustedCatalogGuestResidencyStateV1 { budget_bytes: 2_000, registered_guests: 2, resident_guests: 2, resident_bytes: 2_000, footprint_bytes: 0, hits: 1, compiles: 2, admitted: 2, bypassed: 0, released: 0, compile_micros: ledger.state().compile_micros });
}

/// 🧮️ An operation counts one use however many calls it makes, a guest that does not fit serves every call
/// of the operation that compiled it and is dropped with it, and it displaces the least recently used
/// resident guest only once it has been used in more operations than that guest.
#[tokio::test]
async fn one_operation_counts_one_use_and_an_unadmitted_guest_lives_exactly_as_long_as_its_operation() {
    let (control, compiles) = (TestControl::new(), AtomicUsize::new(0));
    let ledger = residency_ledger(1_000);
    let (a, b) = (ledger.register(1_000), ledger.register(1_000));
    residency_operation(&control, &a, b"a", 3, &compiles).await;
    assert_eq!(compiles.load(Ordering::SeqCst), 1);
    assert!(a.is_resident());
    residency_operation(&control, &b, b"b", 3, &compiles).await;
    assert_eq!(compiles.load(Ordering::SeqCst), 2, "the unadmitted guest served all three calls of its operation from one compile");
    assert!(!b.is_resident() && a.is_resident(), "a guest used in no more operations than the one it would release does not displace it");
    assert_eq!(ledger.state().resident_guests, 1, "the unadmitted guest was dropped with its operation");
    residency_operation(&control, &b, b"b", 3, &compiles).await;
    assert!(!b.is_resident(), "one use each: a tie keeps the resident guest");
    residency_operation(&control, &b, b"b", 1, &compiles).await;
    assert!(b.is_resident() && !a.is_resident(), "a guest used in more operations displaces the least recently used one");
    assert_eq!(compiles.load(Ordering::SeqCst), 4);
    let state = ledger.state();
    assert_eq!((state.admitted, state.bypassed, state.released, state.hits), (2, 2, 1, 6));
}

/// 🔒️ A resident guest a running operation holds is never released, even for a guest used in more
/// operations, and a zero budget keeps nothing resident while operations that reach a guest at once still
/// share its one compile.
#[tokio::test]
async fn held_guests_are_never_released_and_a_zero_budget_keeps_nothing_resident() {
    let (control, compiles) = (TestControl::new(), AtomicUsize::new(0));
    let ledger = residency_ledger(1_000);
    let (a, b) = (ledger.register(1_000), ledger.register(1_000));
    residency_operation(&control, &b, b"b", 1, &compiles).await;
    let holding = control.context();
    let held = b.acquire(&holding, || async { Ok::<_, AuthorityError>(b"never".to_vec()) }).await.unwrap();
    for _ in 0..4 {
        residency_operation(&control, &a, b"a", 1, &compiles).await;
    }
    assert!(b.is_resident() && !a.is_resident(), "the held guest stays although the other was used in more operations");
    assert_eq!(compiles.load(Ordering::SeqCst), 5);
    drop((held, holding));
    residency_operation(&control, &a, b"a", 1, &compiles).await;
    assert!(a.is_resident() && !b.is_resident(), "once released by its operation, the less used guest makes room");
    let zero = residency_ledger(0);
    let (c, zero_compiles) = (zero.register(1), AtomicUsize::new(0));
    let first = control.context();
    let shared = c.acquire(&first, || async { Ok::<_, AuthorityError>(b"c".to_vec()) }).await.unwrap();
    let second = control.context();
    let again = c.acquire(&second, || async { Ok::<_, AuthorityError>(b"never".to_vec()) }).await.unwrap();
    assert!(Arc::ptr_eq(&shared, &again), "an operation that reaches a held guest shares its compile");
    drop((shared, again, first, second));
    residency_operation(&control, &c, b"c", 2, &zero_compiles).await;
    assert_eq!(zero_compiles.load(Ordering::SeqCst), 1);
    assert!(!c.is_resident() && zero.state().resident_guests == 0 && zero.state().admitted == 0, "a zero budget keeps nothing resident");
}

/// 🔁️ The scale law: twelve guests of different sizes, budget for a third of their bytes, sixty rounds of a
/// round-robin where every operation makes three codec calls (a creation's genesis and two validations). After
/// the first round the resident set never changes, every round compiles exactly the guests that do not fit —
/// once per operation — every answer is its own guest's, and the resident charge never exceeds the budget once
/// an operation ends. A pure least-recently-used ledger compiles every guest in every round of the same trace.
#[tokio::test]
async fn a_round_robin_over_more_guests_than_fit_keeps_a_stable_resident_set() {
    const SIZES: [u64; 12] = [500, 300, 800, 200, 600, 400, 700, 100, 900, 300, 500, 200];
    const BUDGET: u64 = 2_000;
    const ROUNDS: usize = 60;
    let (control, compiles) = (TestControl::new(), AtomicUsize::new(0));
    let ledger = residency_ledger(BUDGET);
    let slots: Vec<_> = SIZES.iter().map(|size| ledger.register(*size)).collect();
    let sources: Vec<Vec<u8>> = (0..SIZES.len()).map(|index| format!("guest-{index}").into_bytes()).collect();
    let (mut resident_sets, mut compiled_per_round, mut released_per_round) = (Vec::new(), Vec::new(), Vec::new());
    for _ in 0..ROUNDS {
        let before = compiles.load(Ordering::SeqCst);
        for (slot, source) in slots.iter().zip(&sources) {
            assert_eq!(residency_operation(&control, slot, source, 3, &compiles).await, Sha256::digest(source).to_vec(), "a guest answers from its own source, compiled again or not");
            assert!(ledger.state().resident_bytes <= BUDGET, "the resident charge fits the budget once the operation ended");
        }
        compiled_per_round.push(compiles.load(Ordering::SeqCst) - before);
        released_per_round.push(ledger.state().released);
        resident_sets.push(slots.iter().map(GuestResidencyV1::is_resident).collect::<Vec<_>>());
    }
    let resident = resident_sets[1].iter().filter(|resident| **resident).count();
    assert!(resident >= 2, "the budget holds several guests: {resident_sets:?}");
    assert!(resident_sets[1..].windows(2).all(|pair| pair[0] == pair[1]), "the resident set is stable after the first round");
    assert!(compiled_per_round[1..].iter().all(|compiled| *compiled == SIZES.len() - resident), "each round compiles exactly the guests that do not fit: {compiled_per_round:?}");
    assert!(released_per_round.windows(2).skip(1).all(|pair| pair[0] == pair[1]), "no resident guest is released after the first round: {released_per_round:?}");
    let mut lru: Vec<usize> = Vec::new();
    let mut lru_compiles_per_round = Vec::new();
    for _ in 0..ROUNDS {
        let mut compiled = 0;
        for (index, size) in SIZES.iter().enumerate() {
            if let Some(at) = lru.iter().position(|resident| *resident == index) {
                lru.remove(at);
            } else {
                compiled += 1;
                while lru.iter().map(|resident| SIZES[*resident]).sum::<u64>() + size > BUDGET && !lru.is_empty() {
                    lru.remove(0);
                }
            }
            lru.push(index);
        }
        lru_compiles_per_round.push(compiled);
    }
    assert!(lru_compiles_per_round[1..].iter().all(|compiled| *compiled == SIZES.len()), "the same trace thrashes a pure LRU ledger: {lru_compiles_per_round:?}");
}

/// 🌊️ A workload that moved on displaces the guests it left: after a long round-robin, two guests that were
/// never resident become resident within a bounded number of operations (use counts age), and from then on
/// their operations compile nothing.
#[tokio::test]
async fn a_workload_that_moved_on_displaces_the_guests_it_left() {
    const SIZES: [u64; 8] = [400, 400, 400, 400, 400, 400, 400, 400];
    let (control, compiles) = (TestControl::new(), AtomicUsize::new(0));
    let ledger = residency_ledger(1_600);
    let slots: Vec<_> = SIZES.iter().map(|size| ledger.register(*size)).collect();
    for _ in 0..40 {
        for (index, slot) in slots.iter().enumerate() {
            residency_operation(&control, slot, &[index as u8], 1, &compiles).await;
        }
    }
    let cold: Vec<usize> = (0..SIZES.len()).filter(|index| !slots[*index].is_resident()).take(2).collect();
    assert_eq!(cold.len(), 2);
    let aging = TRUSTED_CATALOG_GUEST_RESIDENCY.access_count_aging_per_guest as usize * SIZES.len();
    let mut operations = 0;
    while !(slots[cold[0]].is_resident() && slots[cold[1]].is_resident()) {
        assert!(operations < 2 * aging, "the new workload became resident within two aging windows");
        residency_operation(&control, &slots[cold[operations % 2]], &[cold[operations % 2] as u8], 1, &compiles).await;
        operations += 1;
    }
    let settled = compiles.load(Ordering::SeqCst);
    for operation in 0..20 {
        residency_operation(&control, &slots[cold[operation % 2]], &[cold[operation % 2] as u8], 1, &compiles).await;
    }
    assert_eq!(compiles.load(Ordering::SeqCst), settled, "the new workload's operations compile nothing once resident");
}

/// 🤝️ Operations that reach one guest at once share its single compile, and each of them is a use: eight
/// concurrent operations outnumber the two of a resident guest, so the shared compile is admitted in its place.
#[tokio::test]
async fn concurrent_operations_share_one_compile() {
    let control = TestControl::new();
    let compiles = AtomicUsize::new(0);
    let ledger = residency_ledger(1_000);
    let (hot, cold) = (ledger.register(1_000), ledger.register(1_000));
    for _ in 0..2 {
        residency_operation(&control, &hot, b"hot", 1, &compiles).await;
    }
    let barrier = tokio::sync::Barrier::new(8);
    let operations = (0..8).map(|_| async {
        let context = control.context();
        let value = cold
            .acquire(&context, || async {
                compiles.fetch_add(1, Ordering::SeqCst);
                Ok::<_, AuthorityError>(b"cold".to_vec())
            })
            .await
            .unwrap();
        barrier.wait().await;
        value
    });
    let answers = futures::future::join_all(operations).await;
    assert!(answers.windows(2).all(|pair| Arc::ptr_eq(&pair[0], &pair[1])), "every concurrent operation holds the same compile");
    assert_eq!(compiles.load(Ordering::SeqCst), 2, "one compile for the resident guest, one shared by eight operations");
    assert!(cold.is_resident() && !hot.is_resident(), "the guest eight operations used displaced the one two used");
}

/// 📐️ A test guest whose state grows in a call, as a compiled guest's does when its first codec call assembles its origin.
struct GrowingGuest(std::sync::atomic::AtomicU64);

impl GuestResidentFootprintV1 for GrowingGuest {
    /// 📐️ What the call left the guest holding.
    fn footprint_bytes(&self) -> u64 {
        self.0.load(Ordering::SeqCst)
    }
}

/// 🧪️ One operation's single call on `slot`, after which its guest holds `footprint` bytes beyond its registration.
async fn growing_operation(control: &TestControl, slot: &GuestResidencyV1<GrowingGuest>, footprint: u64) {
    let context = control.context();
    let guest = slot.acquire(&context, || async { Ok::<_, AuthorityError>(GrowingGuest(std::sync::atomic::AtomicU64::new(0))) }).await.unwrap();
    guest.0.store(footprint, Ordering::SeqCst);
    slot.remeasure(&guest, &context).await;
}

/// 📐️ A guest is charged what it holds after each call, not only what it was registered with: one that grew past what the
/// budget has left stays by releasing the least recently used unheld guests when it was used in more operations than they
/// were, a guest that then does not fit is served without staying, and one that grew past the whole budget serves the
/// operation holding it and is dropped with it.
#[tokio::test]
async fn a_guest_that_grows_in_a_call_is_charged_what_it_holds() {
    let control = TestControl::new();
    let ledger = GuestResidencyLedgerV1::<GrowingGuest>::new(TrustedCatalogGuestResidencyV1 { resident_component_bytes: 3_000, ..TRUSTED_CATALOG_GUEST_RESIDENCY });
    let (a, b) = (ledger.register(1_000), ledger.register(1_000));
    growing_operation(&control, &b, 0).await;
    growing_operation(&control, &a, 0).await;
    growing_operation(&control, &a, 0).await;
    assert!(a.is_resident() && b.is_resident());
    assert_eq!((ledger.state().resident_bytes, ledger.state().footprint_bytes), (2_000, 0));
    growing_operation(&control, &a, 1_500).await;
    let grown = ledger.state();
    assert!(a.is_resident() && !b.is_resident(), "the grown guest, used in more operations, displaced the least recently used one");
    assert_eq!((grown.resident_guests, grown.resident_bytes, grown.footprint_bytes, grown.released), (1, 2_500, 1_500, 1));
    growing_operation(&control, &b, 0).await;
    assert!(a.is_resident() && !b.is_resident() && ledger.state().bypassed == 1, "a guest used less often than the grown one is served without staying");
    let context = control.context();
    let guest = a.acquire(&context, || async { Ok::<_, AuthorityError>(GrowingGuest(std::sync::atomic::AtomicU64::new(0))) }).await.unwrap();
    guest.0.store(2_500, Ordering::SeqCst);
    a.remeasure(&guest, &context).await;
    assert!(!a.is_resident() && ledger.state().resident_guests == 1, "a guest grown past the whole budget stays only with the operation holding it");
    drop((guest, context));
    let released = ledger.state();
    assert_eq!((released.resident_guests, released.resident_bytes, released.footprint_bytes, released.released), (0, 0, 0, 2), "and is dropped with it");
}

/// 🧵️ A guest codec call interprets on the blocking pool, never on the worker awaiting it: another
/// task on a one-thread runtime keeps running for the whole call, every fuel observation reaches the
/// caller's context in order, and a cancelled caller is released at its next observation — or within
/// the poll between observations — with the call itself cancelled, as it is when the caller is dropped.
#[test]
fn guest_codec_calls_run_off_the_async_worker_and_relay_their_fuel() {
    let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
    runtime.block_on(async {
        let control = TestControl::new();
        let context = control.context();
        let ticks = Arc::new(AtomicUsize::new(0));
        let ticker = {
            let ticks = ticks.clone();
            tokio::spawn(async move {
                loop {
                    ticks.fetch_add(1, Ordering::SeqCst);
                    tokio::time::sleep(std::time::Duration::from_millis(5)).await;
                }
            })
        };
        let answer = interpret_off_worker(&context, |_handle, progress, _cancellation| {
            for fuel in [10, 20, 30] {
                std::thread::sleep(std::time::Duration::from_millis(100));
                progress(fuel);
            }
            Ok(42u32)
        })
        .await;
        assert!(matches!(answer, Ok(Ok(42))), "{answer:?}");
        assert!(ticks.load(Ordering::SeqCst) >= 20, "the awaiting worker kept running other tasks: {} ticks", ticks.load(Ordering::SeqCst));
        let relayed: Vec<u64> = control.progress.lock().unwrap().iter().filter(|progress| progress.stage == AuthorityProgressStage::GuestCodecExecuting).map(|progress| progress.completed_units).collect();
        assert_eq!(relayed, vec![10, 20, 30]);
        let observed = |observes: Arc<AtomicBool>| {
            move |_handle: &tokio::runtime::Handle, progress: &mut dyn FnMut(u64), cancellation: &semio_framework_plugin_guest_contract::GuestCallCancellation| -> Result<u32, semio_framework_plugin_guest_contract::TurnFault> {
                progress(1);
                let started = std::time::Instant::now();
                while !cancellation.is_cancelled() && started.elapsed() < std::time::Duration::from_secs(10) {
                    std::thread::sleep(std::time::Duration::from_millis(1));
                }
                observes.store(cancellation.is_cancelled(), Ordering::SeqCst);
                Err(semio_framework_plugin_guest_contract::TurnFault::Cancelled)
            }
        };
        let settled = |observes: Arc<AtomicBool>| async move {
            let started = std::time::Instant::now();
            while !observes.load(Ordering::SeqCst) && started.elapsed() < std::time::Duration::from_secs(1) {
                tokio::time::sleep(std::time::Duration::from_millis(5)).await;
            }
            observes.load(Ordering::SeqCst)
        };
        control.cancelled.store(true, Ordering::SeqCst);
        let refused = Arc::new(AtomicBool::new(false));
        let started = std::time::Instant::now();
        let cancelled = interpret_off_worker(&context, observed(refused.clone())).await;
        assert!(matches!(cancelled, Err(AuthorityError::Cancelled)), "{cancelled:?}");
        assert!(started.elapsed() < std::time::Duration::from_millis(300), "a cancelled caller is released at its next observation, not when the call ends");
        assert!(settled(refused).await, "the released caller cancelled its call");
        let quiet = Arc::new(AtomicBool::new(false));
        let started = std::time::Instant::now();
        let silent = interpret_off_worker(&context, {
            let quiet = quiet.clone();
            move |_handle, _progress, cancellation| {
                while !cancellation.is_cancelled() && started.elapsed() < std::time::Duration::from_secs(10) {
                    std::thread::sleep(std::time::Duration::from_millis(1));
                }
                quiet.store(cancellation.is_cancelled(), Ordering::SeqCst);
                Ok(0u32)
            }
        })
        .await;
        assert!(matches!(silent, Err(AuthorityError::Cancelled)), "{silent:?}");
        assert!(started.elapsed() < GUEST_CALL_CANCELLATION_POLL * 6, "a cancelled caller that sees no observation is released within its poll: {:?}", started.elapsed());
        assert!(settled(quiet).await, "the call that reported nothing was cancelled too");
        control.cancelled.store(false, Ordering::SeqCst);
        let dropped = Arc::new(AtomicBool::new(false));
        let abandoned = tokio::time::timeout(std::time::Duration::from_millis(100), interpret_off_worker(&context, observed(dropped.clone()))).await;
        assert!(abandoned.is_err(), "an uncancelled caller keeps awaiting its call");
        assert!(settled(dropped).await, "dropping the awaiting caller cancelled its call");
        ticker.abort();
    });
}

/// 🛑️ LAW (H11, session 13): a hub asked to stop exits within about a second while its background catalog
/// verification interprets. The shutdown aborts the verification; the aborted verification cancels its guest call,
/// which the interpreter ends at its next step; so dropping the runtime — what the hub's `main` does once `serve`
/// returned — waits for no interpretation. Before the interpreter observed its caller the drop waited for the call's
/// own fuel bound: SIGTERM → exit 17.9–25.2 s on catalog B3 (`📓️wp-h11.md`). The guest's `pack-schema-hash` never
/// returns, so only the cancellation can end it.
#[test]
fn an_aborted_catalog_verification_leaves_no_interpretation_for_the_runtime_to_wait_on() {
    let runtime = tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap();
    let mut fixture = prepared_fixture();
    install_owned_guest(&mut fixture, OwnedTestHash::Spins);
    let catalog = Arc::new(runtime.block_on(async { load_fixture(&fixture, &[], &TestControl::new().context()).await }).expect("a catalog whose row waits for its guest"));
    let control = Arc::new(TestControl::new());
    let verification = runtime.spawn({
        let (catalog, control) = (catalog.clone(), control.clone());
        async move { catalog.verify_pending_guests(&control.context()).await }
    });
    let interpreting = |control: &TestControl| control.progress.lock().unwrap().iter().any(|progress| progress.stage == AuthorityProgressStage::GuestCompiling && progress.completed_units == 1);
    let waited = std::time::Instant::now();
    while !interpreting(&*control) && waited.elapsed() < std::time::Duration::from_secs(30) {
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
    assert!(interpreting(&*control), "the verification compiled its guest and interprets it");
    std::thread::sleep(std::time::Duration::from_millis(200));
    assert!(!verification.is_finished(), "the spinning guest keeps the verification in flight");
    let started = std::time::Instant::now();
    verification.abort();
    let aborted = runtime.block_on(verification);
    drop(runtime);
    assert!(aborted.is_err_and(|error| error.is_cancelled()), "the shutdown aborted the verification");
    assert!(started.elapsed() < std::time::Duration::from_secs(1), "the runtime was dropped with no interpretation left to wait on: {:?}", started.elapsed());
}
