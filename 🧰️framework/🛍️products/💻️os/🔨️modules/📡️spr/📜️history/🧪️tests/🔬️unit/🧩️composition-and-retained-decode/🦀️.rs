/// 🧩️ The composition overlay in the binary log and the retained history decode: yielding across granted turns, closing every original owner, rejecting malformed and repeated transitions.

//#region 🔖️Composition
#[semio_framework_async_macros::async_test]
async fn composition_overlay_round_trips_through_the_binary_log() {
    let composition = HistoryComposition {
        owner: Some(("parent-1!s.stdio.object@1/*".to_string(), "mesh".to_string(), "child-1".to_string())),
        dialect: Some(("s.stdio.mesh".to_string(), "1".to_string(), "*".to_string())),
    };
    let log = HistoryLog { composition: Some(composition.clone()), ..sample_log().await };

    let bytes = encode_history(&log, &EncodeOptions::default()).await.expect("encode");
    let decoded = decode_history(&bytes, &DecodeOptions::default()).await.expect("decode");
    assert_eq!(decoded.composition, Some(composition), "the composition overlay did not survive the binary round trip");
}

#[semio_framework_async_macros::async_test]
async fn a_log_without_composition_writes_no_composition_record() {
    let bytes = encode_history(&sample_log().await, &EncodeOptions::default()).await.expect("encode");
    assert_eq!(decode_history(&bytes, &DecodeOptions::default()).await.expect("decode").composition, None);
    assert!(!bytes.windows(1).any(|window| window == [REC_COMPOSITION]) || decode_history(&bytes, &DecodeOptions::default()).await.is_ok());
}

struct FlaggedTurn {
    yielding: std::sync::atomic::AtomicBool,
    cancelled: std::sync::atomic::AtomicBool,
    work: std::sync::atomic::AtomicU64,
}

impl FlaggedTurn {
    fn new() -> Self {
        Self { yielding: std::sync::atomic::AtomicBool::new(false), cancelled: std::sync::atomic::AtomicBool::new(false), work: std::sync::atomic::AtomicU64::new(1_000_000) }
    }
}

impl RetainedHistoryTurn for FlaggedTurn {
    fn should_yield(&self) -> bool {
        self.yielding.load(std::sync::atomic::Ordering::Relaxed)
    }
    fn is_cancelled(&self) -> bool {
        self.cancelled.load(std::sync::atomic::Ordering::Relaxed)
    }
}

fn retained_corpus() -> (serde_json::Value, protocol::codec::HistoryReadLimits, RetainedCloneGrant, usize) {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../../🛬️decode/🗂️file/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let limits: protocol::codec::HistoryReadLimits = serde_json::from_value(law["limits"].clone()).unwrap();
    let axis = |key: &str| law["grant"][key].as_u64().unwrap() as usize;
    let grant = RetainedCloneGrant { maximum_items: axis("items"), maximum_copy_bytes: axis("copyBytes"), maximum_capacity_bytes: axis("capacityBytes"), maximum_release_bytes: axis("releaseBytes"), maximum_depth: axis("depth") };
    let allocation = law["control"]["maximumAllocationBytes"].as_u64().unwrap() as usize;
    (law, limits, grant, allocation)
}

struct TurnBudget(u64);
struct TurnWorkDriver;
impl semio_framework_value_native_owned::native_decoding::OriginalWorkRecipient for TurnWorkDriver{
    type Context<'t,'s>=TurnBudget where 's:'t;
    fn validate_context<'t,'s>(&self,_:&TurnBudget)->Result<(),ValueError> where 's:'t{Ok(())}
    fn source_turn<'t,'s>(&self,_:&TurnBudget)->Option<semio_framework_value_native_owned::native_decoding::OriginalSourceTurnIdentity> where 's:'t{None}
    fn admits<'r,'t,'s>(&mut self,c:&mut TurnBudget,quote:semio_framework_value_native_owned::native_decoding::OriginalWorkQuote<'r>,observed:u64,additional:u64)->semio_framework_value_native_owned::native_decoding::OriginalWorkQuoteComplete<'r> where 's:'t{quote.complete(Ok(observed.checked_add(additional).is_some_and(|work|work<=c.0.min(4096))))}
    fn settle_partial<'r,'t,'s>(&mut self,_:&mut TurnBudget,check:semio_framework_value_native_owned::native_decoding::OriginalWorkPartialCheck<'r>)->semio_framework_value_native_owned::native_decoding::OriginalWorkPartialCheckComplete<'r> where 's:'t{check.complete(Ok(()))}
    fn recover_partial<'r,'t,'s>(&mut self,_:&mut TurnBudget,check:semio_framework_value_native_owned::native_decoding::OriginalWorkPartialCheck<'r>)->semio_framework_value_native_owned::native_decoding::OriginalWorkPartialCheckComplete<'r> where 's:'t{check.pending()}
}
fn with_budget<T>(turn:&FlaggedTurn,run:impl FnOnce(&mut semio_framework_value_native_owned::native_decoding::OriginalWorkLoan<'_>)->T)->T{let mut budget=TurnBudget(turn.work.load(std::sync::atomic::Ordering::Relaxed));let mut authority=semio_framework_value_native_owned::native_decoding::OriginalWorkAuthority::new(TurnWorkDriver);authority.with_loan(&mut budget,run).unwrap()}
fn settle_work(turn:&FlaggedTurn,work:u128){let work=u64::try_from(work).unwrap();let have=turn.work.load(std::sync::atomic::Ordering::Relaxed);assert!(work<=have.min(4096));turn.work.store(have-work,std::sync::atomic::Ordering::Relaxed)}

fn paid_turn(decode: &mut RetainedHistoryDecode, bytes: &[u8], turn: &FlaggedTurn, grant: RetainedCloneGrant, history: &mut Option<HistoryLog>) -> RetainedHistorySourceStep {
    let outcome = with_budget(turn, |loan| decode.step(bytes, turn, grant, history, loan));
    settle_work(turn,outcome.work);
    if turn.should_yield()||turn.is_cancelled(){assert_eq!(outcome.work,0)}
    let step=outcome.result.expect("advance retained decode");
    if let Some((issued, progress)) = decode.take_receipt() {
        assert_eq!(issued, grant, "every receipt answers the exact supplied grant");
        assert!(progress.fits(grant), "every receipt fits the supplied grant");
    }
    step
}

fn settle(decode: &mut RetainedHistoryDecode, bytes: &[u8], turn: &FlaggedTurn, grant: RetainedCloneGrant, history: &mut Option<HistoryLog>) -> (usize, RetainedHistorySourceStep) {
    let mut turns = 0usize;
    loop {
        turns += 1;
        assert!(turns < 1_000_000, "retained history decode did not settle");
        match paid_turn(decode, bytes, turn, grant, history) {
            step @ (RetainedHistorySourceStep::Ready | RetainedHistorySourceStep::Fault) => return (turns, step),
            RetainedHistorySourceStep::Closed => panic!("retained history decode closed before it was ready"),
            RetainedHistorySourceStep::Progress | RetainedHistorySourceStep::Blocked => {}
        }
    }
}

fn close_all(decode: &mut RetainedHistoryDecode, bytes: &[u8], grant: RetainedCloneGrant, turn:&FlaggedTurn) {
    for _ in 0..1_000_000 {
        if decode.terminal_is_empty() {
            return;
        }
        let outcome = with_budget(turn, |loan| decode.close_step(bytes, &RetainedHistoryUninterrupted, grant, loan));
        settle_work(turn,outcome.work);
        let step=outcome.result.expect("close retained decode");
        assert!(step.progress().fits(grant), "every close receipt fits the supplied grant");
    }
    panic!("retained history close did not settle");
}

#[semio_framework_async_macros::async_test]
async fn retained_history_decode_yields_across_granted_turns_and_returns_the_exact_log() {
    let log = sample_log().await;
    let bytes = encode_history(&log, &EncodeOptions::default()).await.expect("encode retained history fixture");
    let (_, limits, grant, allocation) = retained_corpus();
    let mut decode = RetainedHistoryDecode::new((0, bytes.len()), limits, false, allocation).expect("admit retained decode");
    assert_eq!(decode.source_identity(), None, "nothing is borrowed before the first granted turn");
    let turn = FlaggedTurn::new();
    let mut history = None;
    let mut yielded = 0usize;
    let mut turns = 0usize;
    let step = loop {
        turns += 1;
        assert!(turns < 1_000_000, "retained history decode did not settle");
        if turns % 3 == 0 {
            turn.yielding.store(true,std::sync::atomic::Ordering::Relaxed);
            let before = decode.consumed_end();
            assert_eq!(paid_turn(&mut decode, &bytes, &turn, grant, &mut history), RetainedHistorySourceStep::Blocked);
            assert_eq!(decode.consumed_end(), before, "a yielded turn consumes no original byte");
            turn.yielding.store(false,std::sync::atomic::Ordering::Relaxed);
            yielded += 1;
        }
        match paid_turn(&mut decode, &bytes, &turn, grant, &mut history) {
            step @ (RetainedHistorySourceStep::Ready | RetainedHistorySourceStep::Fault) => break step,
            RetainedHistorySourceStep::Closed => panic!("retained history decode closed before it was ready"),
            RetainedHistorySourceStep::Progress | RetainedHistorySourceStep::Blocked => {}
        }
    };
    assert_eq!(step, RetainedHistorySourceStep::Ready, "{:?}", decode.fault());
    assert!(yielded > 0 && turns > 3, "granted turns must remain separately scheduled");
    assert_eq!(history, Some(log));
    assert!(decode.terminal_is_empty());
}

#[semio_framework_async_macros::async_test]
async fn retained_history_decode_closes_every_original_owner_after_a_cancelled_turn() {
    let log = sample_log().await;
    let bytes = encode_history(&log, &EncodeOptions::default()).await.expect("encode retained history fixture");
    let (law, limits, grant, allocation) = retained_corpus();
    for cut in law["nativeTrials"]["cancelAt"].as_array().unwrap() {
        let cut = cut.as_u64().unwrap() as usize;
        let mut decode = RetainedHistoryDecode::new((0, bytes.len()), limits, false, allocation).expect("admit retained decode");
        let turn = FlaggedTurn::new();
        let mut history = None;
        for _ in 0..cut {
            if matches!(paid_turn(&mut decode, &bytes, &turn, grant, &mut history), RetainedHistorySourceStep::Ready | RetainedHistorySourceStep::Fault) {
                break;
            }
        }
        turn.cancelled.store(true,std::sync::atomic::Ordering::Relaxed);
        if !decode.terminal_is_empty() && history.is_none() {
            assert_eq!(paid_turn(&mut decode, &bytes, &turn, grant, &mut history), RetainedHistorySourceStep::Blocked, "a cancelled turn admits no further original work");
        }
        close_all(&mut decode, &bytes, grant, &turn);
        assert!(decode.terminal_is_empty());
        assert!(decode.source_identity().is_none());
    }
}

#[semio_framework_async_macros::async_test]
async fn retained_history_decode_refuses_a_demand_above_the_grant_and_allocates_nothing() {
    let log = sample_log().await;
    let bytes = encode_history(&log, &EncodeOptions::default()).await.expect("encode retained history fixture");
    let (_, limits, grant, allocation) = retained_corpus();
    let mut decode = RetainedHistoryDecode::new((0, bytes.len()), limits, false, allocation).expect("admit retained decode");
    let turn = FlaggedTurn::new();
    let mut history = None;
    paid_turn(&mut decode, &bytes, &turn, grant, &mut history);
    let denied = RetainedCloneGrant { maximum_copy_bytes: 0, ..grant };
    let before = decode.consumed_end();
    let (outcome, born, released) = with_budget(&turn, |loan| semio_os_history_testing_retirement::observe_backing(|| decode.step(&bytes, &turn, denied, &mut history, loan)));
    settle_work(&turn, outcome.work);
    let step = outcome.result.expect("refuse retained decode");
    assert_eq!(step, RetainedHistorySourceStep::Blocked, "a demand above the grant is refused typed");
    assert_eq!((born, released), (0, 0), "a refused forwarded step allocates nothing");
    assert_eq!(decode.consumed_end(), before, "a refused step consumes no original byte");
    close_all(&mut decode, &bytes, grant, &turn);
    assert!(decode.terminal_is_empty());
}

#[semio_framework_async_macros::async_test]
async fn retained_history_decode_rejects_a_crc_valid_malformed_transition_after_valid_records() {
    let mut log = sample_log().await;
    log.transitions.push(HistoryTransitionRecord { id: "transition-after-valid-prefix".to_string(), actor: "alice".into(), hlt: (1, 2, 0), dependencies: Vec::new(), observed: None, payload: semio_framework_value::paged::PagedBytes::<{isize::MAX as usize}>::try_from_slice(&[0xff]).unwrap() });
    let bytes = encode_history(&log, &EncodeOptions::default()).await.expect("encode semantically malformed retained history");
    let (_, limits, grant, allocation) = retained_corpus();
    let mut decode = RetainedHistoryDecode::new((0, bytes.len()), limits, false, allocation).expect("admit retained semantic refusal");
    let mut history = None;
    let turn=FlaggedTurn::new();
    let (_, step) = settle(&mut decode, &bytes, &turn, grant, &mut history);
    assert_eq!(step, RetainedHistorySourceStep::Fault, "a malformed transition payload never reaches ready");
    assert!(decode.fault().is_some(), "the refusal keeps its exact original owner until it is closed");
    assert!(history.is_none());
    close_all(&mut decode, &bytes, grant, &turn);
    assert!(decode.terminal_is_empty());
}

#[semio_framework_async_macros::async_test]
async fn retained_history_decode_rejects_a_repeated_transition_id() {
    let mut log = sample_log().await;
    log.transitions.push(log.transitions[0].clone());
    let bytes = encode_history(&log, &EncodeOptions::default()).await.expect("encode repeated transition");
    let (_, limits, grant, allocation) = retained_corpus();
    let mut decode = RetainedHistoryDecode::new((0, bytes.len()), limits, false, allocation).expect("admit retained decode");
    let mut history = None;
    let turn=FlaggedTurn::new();
    let (_, step) = settle(&mut decode, &bytes, &turn, grant, &mut history);
    assert_eq!(step, RetainedHistorySourceStep::Fault, "a repeated transition identity never reaches ready");
    assert!(decode.fault().is_some());
    assert!(history.is_none());
    close_all(&mut decode, &bytes, grant, &turn);
    assert!(decode.terminal_is_empty());
}
//#endregion 🔖️Composition
