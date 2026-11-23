//! 🫴️ Actual original native caller for the Semio SQLite snapshot suites: independent body and close grants drive every erased conversion.
use semio_os_kernel::{self as store, sqlite_snapshot::{SqliteDatabaseLimits, SqliteSnapshotControl}};
use semio_framework_value::retained_clone::OriginalEffectReply;
use semio_framework_value::{AuthoringWallet, NativeEncodeControl, OriginalRetirementFailure, RetainedAllocationEffect, RetainedCloneGrant, RetainedCloneStep, RetainedWorkOutcome, RetainedWorkPoll};
use semio_framework_value_native_owned::native_decoding::{NativeAllocationAuthority, NativeAllocationAuthorityPort, NativeAllocationEvent, NativeAllocationRefusal, NativeAllocationReply, NativeAllocationReplyComplete, NativeSendAllocationLoan, OriginalConstructionAccept, OriginalConstructionAcceptComplete, OriginalConstructionAuthority, OriginalConstructionGrant, OriginalConstructionGrantComplete, OriginalConstructionLoan, OriginalConstructionRecipient, OriginalSourceTurnIdentity, OriginalWorkAuthority, OriginalWorkLoan, OriginalWorkPartialCheck, OriginalWorkPartialCheckComplete, OriginalWorkQuote, OriginalWorkQuoteComplete, OriginalWorkRecipient};
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;

pub fn native_caller_policy() -> serde_json::Value { serde_json::from_str(include_str!("🎟️original.json")).unwrap() }
pub trait OriginalNativeRefusalReceipt { fn original_native_receipt(&self) -> semio_framework_value::retained_clone::RetainedCloneProgress; }
impl OriginalNativeRefusalReceipt for semio_framework_value::ValueError { fn original_native_receipt(&self) -> semio_framework_value::retained_clone::RetainedCloneProgress { self.retained_progress() } }
impl OriginalNativeRefusalReceipt for store::io_schema::IoError { fn original_native_receipt(&self) -> semio_framework_value::retained_clone::RetainedCloneProgress { self.cause.retained_progress() } }
macro_rules! original_native_caller {
    ($name:ident,$root:ident,$module:ident,$control:ident,$owner:ident,$recipient:ident,$progress:ident,$close:path) => {
        pub fn $name<T, E: OriginalNativeRefusalReceipt>(law: &serde_json::Value, limits: SqliteDatabaseLimits, callback: &mut dyn FnMut(store::sqlite_snapshot::SqliteSnapshotProgress) -> bool, original_observer: &mut dyn FnMut($root::$module::$progress) -> bool, original_continuation: impl FnOnce(&mut semio_framework_value::$control<'_>), operation: impl FnOnce(&mut SqliteSnapshotControl<'_>, &mut store::$owner<'_, '_>) -> Result<T, E>) -> Result<T, E> {
            use $root::$module::{$recipient, NativeAllocationEvent};
            use std::sync::atomic::{AtomicUsize, Ordering};
            let grant = serde_json::from_value::<semio_framework_value::RetainedCloneGrant>(law["bodyGrant"].clone()).unwrap();
            let close = serde_json::from_value::<semio_framework_value::RetainedCloneGrant>(law["closeGrant"].clone()).unwrap();
            let accepted = AtomicUsize::new(0usize);
            let original_allocation = |event: NativeAllocationEvent| { match event { NativeAllocationEvent::Admit(request) => { assert_eq!(request.owned_bytes, accepted.load(Ordering::Relaxed)); accepted.store(request.next_owned_bytes, Ordering::Relaxed); } NativeAllocationEvent::Quote(request) => assert_eq!(request.owned_bytes, accepted.load(Ordering::Relaxed)), NativeAllocationEvent::Post { request, actual } => accepted.store(request.owned_bytes + actual, Ordering::Relaxed), NativeAllocationEvent::Cancel(_) => {} } Ok(()) };
            with_port(original_allocation, |allocation| with_original_authorities(grant, |work, construction| { let (mut work, mut construction) = (work.reborrow(), construction.reborrow());
            let mut recipient = $recipient::new();
            let mut native = semio_framework_value::$control::new_forwarded(law["nativeMaximumBytes"].as_u64().unwrap() as usize, original_observer, allocation);
            native.install_retirement_recipient(&mut recipient).unwrap();
            native.install_work_recipient(&mut work).unwrap();
            native.install_construction_recipient(&mut construction).unwrap();
            let mut sql = SqliteSnapshotControl::new(callback, limits);
            let mut owner = store::$owner::new(&mut native, grant);
            let result = operation(&mut sql, &mut owner);
            assert!(owner.progress().fits(grant));
            if let Err(error) = &result { assert!(error.original_native_receipt().fits(grant)); }
            drop(owner);
            assert_eq!(native.owned_bytes(), accepted.load(Ordering::Relaxed));
            let held = native.has_retirement_owner();
            let denied = serde_json::from_value::<semio_framework_value::RetainedCloneGrant>(law["deniedCloseGrant"].clone()).unwrap();
            assert_eq!($close(&mut native, denied).result.unwrap().progress(), Default::default());
            assert_eq!(native.has_retirement_owner(), held);
            original_continuation(&mut native);
            for _ in 0..law["maximumCloseTurns"].as_u64().unwrap() {
                if !native.has_retirement_owner() { return result; }
                assert!($close(&mut native, close).result.unwrap().progress().fits(close));
            }
            assert!(!native.has_retirement_owner(), "original native caller did not reach funded terminal");
            result
            }))
        }
    };
}
original_native_caller!(with_original_decode, semio_framework_value_native_owned, native_decoding, NativeDecodeControl, NativeSnapshotDecodeOwner, NativeDecodeRetirementRecipient, NativeDecodeProgress, close_decode);
original_native_caller!(with_original_encode, semio_framework_value, native_encoding, NativeEncodeControl, NativeSnapshotEncodeOwner, NativeEncodeRetirementRecipient, NativeEncodeProgress, close_encode);

pub fn retire_original_output<T: semio_framework_value::retirement::RetireOwned>(source: T, law: &serde_json::Value) {
    let close = serde_json::from_value::<semio_framework_value::RetainedCloneGrant>(law["closeGrant"].clone()).unwrap();
    let mut owner = semio_framework_value::retirement::controlled::ControlledRetirement::new(source).map_err(|(error, _)| error).unwrap();
    for _ in 0..law["maximumCloseTurns"].as_u64().unwrap() {
        if owner.terminal_is_empty() { return; }
        let (turn, _) = AuthoringWallet::new(close, 1 << 24).run_settled(close, 1 << 10, |effect| owner.step(close.maximum_copy_bytes, effect)).expect("authoring retire effect").expect("retire work quote admitted");
        assert!(turn.result.unwrap().progress().fits(close));
    }
    panic!("original native output did not reach funded terminal");
}

pub fn diagnostic_capacity(error: &semio_framework_value::ValueError) -> usize { match &error.message { std::borrow::Cow::Owned(text) => text.capacity(), std::borrow::Cow::Borrowed(_) => 0 } }

/// 🌐️ Runs one erased `io_run_with_snapshot_control` route under explicit original native decode and encode authority, then closes both retirement recipients.
/// [`store::io::io_mechanism::io_run_with_snapshot_control`] returns an already-completed future, so `with_native_control(|native| io_run_with_snapshot_control(..)).await` is the whole call.
pub fn with_native_control<T>(operation: impl FnOnce(&mut store::io::io_mechanism::IoRunControl<'_, '_>) -> T) -> T {
    let maximum = 16usize << 20;
    let grant = semio_framework_value::RetainedCloneGrant { maximum_items: 1 << 20, maximum_copy_bytes: maximum, maximum_capacity_bytes: maximum, maximum_release_bytes: maximum, maximum_depth: 512 };
    with_original_authorities(grant, |decode_work, decode_construction| with_original_authorities(grant, |encode_work, encode_construction| {
        let mut decode_recipient = semio_framework_value_native_owned::native_decoding::NativeDecodeRetirementRecipient::new();
        let mut encode_recipient = semio_framework_value::native_encoding::NativeEncodeRetirementRecipient::new();
        let mut receive = |_| true;
        let mut publish = |_| true;
        let (mut decode_work, mut decode_construction) = (decode_work.reborrow(), decode_construction.reborrow());
        let (mut encode_work, mut encode_construction) = (encode_work.reborrow(), encode_construction.reborrow());
        let mut decode = NativeDecodeControl::new(maximum, &mut receive);
        let mut encode = NativeEncodeControl::new(maximum, &mut publish);
        decode.install_retirement_recipient(&mut decode_recipient).expect("the run owns an explicit decode retirement slot");
        encode.install_retirement_recipient(&mut encode_recipient).expect("the run owns an explicit encode retirement slot");
        decode.install_work_recipient(&mut decode_work).expect("the run owns its decode work loan");
        decode.install_construction_recipient(&mut decode_construction).expect("the run owns its decode construction loan");
        encode.install_work_recipient(&mut encode_work).expect("the run owns its encode work loan");
        encode.install_construction_recipient(&mut encode_construction).expect("the run owns its encode construction loan");
        let result = {
            let mut control = store::io::io_mechanism::IoRunControl::new(&mut decode, &mut encode, grant);
            operation(&mut control)
        };
        while decode.has_retirement_owner() {
            close_decode(&mut decode, grant).result.expect("the decode recipient closes under the run grant");
        }
        while encode.has_retirement_owner() {
            close_encode(&mut encode, grant).result.expect("the encode recipient closes under the run grant");
        }
        result
    }))
}

pub type Close = RetainedWorkOutcome<RetainedCloneStep, OriginalRetirementFailure>;
pub type Step = RetainedWorkPoll<RetainedCloneStep, OriginalRetirementFailure>;

/// 🏁️ Settles one close poll: a pending turn is progress-free work.
pub fn settle(poll: Step) -> Close {
    match poll {
        RetainedWorkPoll::Complete(turn) => turn,
        RetainedWorkPoll::Pending { work } => RetainedWorkOutcome { result: Ok(RetainedCloneStep::Progress(Default::default())), work },
    }
}

/// 🪙️ Runs one closure inside one original effect funded by an explicit authoring wallet; a zero grant runs nothing.
pub fn authored_invoked(grant: RetainedCloneGrant, operation: impl FnOnce(&mut RetainedAllocationEffect<'_>) -> Step) -> Step {
    if grant.maximum_items == 0 || grant.maximum_depth == 0 { return RetainedWorkPoll::Pending { work: 0 }; }
    AuthoringWallet::new(grant, 1 << 20).run_invoked(grant, 1 << 10, operation).expect("authoring close effect").expect("close work quote admitted").output
}

/// ♻️ Closes one turn of the encoder's selected retirement recipient into a local reply slot.
pub fn encode_step(native: &mut NativeEncodeControl<'_>, body: usize, effect: &mut RetainedAllocationEffect<'_>) -> Step {
    let mut slot = None;
    { let reply = OriginalEffectReply::borrow(&mut slot).unwrap(); drop(native.close_retirement_recipient_into(body, effect, reply).unwrap()); }
    RetainedWorkPoll::Complete(RetainedWorkOutcome { result: slot.expect("encode close recorded its reply"), work: effect.performed_work() })
}

/// ♻️ Closes one turn of the decoder's selected retirement recipient.
pub fn decode_step(native: &mut NativeDecodeControl<'_>, body: usize, effect: &mut RetainedAllocationEffect<'_>) -> Step { native.close_retirement_recipient(body, effect) }

/// ♻️ Closes one encoder turn under its own wallet.
pub fn close_encode(native: &mut NativeEncodeControl<'_>, grant: RetainedCloneGrant) -> Close { settle(authored_invoked(grant, |effect| encode_step(native, grant.maximum_copy_bytes, effect))) }

/// ♻️ Closes one decoder turn under its own wallet.
pub fn close_decode(native: &mut NativeDecodeControl<'_>, grant: RetainedCloneGrant) -> Close {
    if grant.maximum_items != 0 && grant.maximum_depth != 0 && birth_required(native, grant) { return settle(native.with_retirement::<_, semio_framework_value::ValueError>(grant, |original| Ok(original.close_original_birth_step(grant))).expect("original birth retirement")); }
    settle(authored_invoked(grant, |effect| decode_step(native, grant.maximum_copy_bytes, effect)))
}

struct EventPort<F>(F);
impl<F: FnMut(NativeAllocationEvent) -> Result<(), NativeAllocationRefusal>> NativeAllocationAuthorityPort for EventPort<F> {
    type Context<'turn, 'source> = () where 'source: 'turn;
    fn validate_context<'turn, 'source>(&self, _: &()) -> Result<(), semio_framework_value::ValueError> where 'source: 'turn { Ok(()) }
    fn source_turn<'turn, 'source>(&self, _: &()) -> Option<OriginalSourceTurnIdentity> where 'source: 'turn { None }
    fn receive<'reply, 'turn, 'source>(&mut self, _: &mut (), event: NativeAllocationEvent, reply: NativeAllocationReply<'reply>) -> Result<NativeAllocationReplyComplete<'reply>, semio_framework_value::ValueError> where 'source: 'turn { Ok(reply.complete((self.0)(event))) }
    fn recover<'reply, 'turn, 'source>(&mut self, _: &mut (), _: NativeAllocationEvent, reply: NativeAllocationReply<'reply>) -> Result<NativeAllocationReplyComplete<'reply>, semio_framework_value::ValueError> where 'source: 'turn { Ok(reply.pending()) }
}

/// 🛂️ Lends one genuine Send allocation loan of an original port that answers every event through the recording closure.
pub fn with_port<R>(port: impl FnMut(NativeAllocationEvent) -> Result<(), NativeAllocationRefusal> + Send, operation: impl FnOnce(&mut dyn NativeSendAllocationLoan) -> R) -> R {
    let mut authority = NativeAllocationAuthority::new(EventPort(port));
    let result = authority.with_send_loan(&mut (), operation).expect("fixture allocation port validates");
    assert!(authority.into_port().is_ok(), "fixture allocation port settles every transaction");
    result
}

/// 🧾️ Extracts the primary value refusal an original native failure retains.
pub fn primary(failure: &store::io::control::NativeSnapshotFailure) -> semio_framework_value::ValueError {
    let error = match failure.cause() {
        Some(store::io::control::NativeSnapshotCause::Value(error)) => error.clone(),
        Some(store::io::control::NativeSnapshotCause::Receiving(error)) => error.primary.clone().expect("receiving failure carries its primary"),
        _ => panic!("native failure carries a value cause"),
    };
    error.with_retained_progress(failure.retained_progress())
}

/// 🧾️ Collapses one staged SQL failure to its primary value refusal.
pub fn staged<T>(stage: store::sqlite_snapshot::SqliteStageFailure<T, store::io::control::NativeSnapshotFailure>) -> semio_framework_value::ValueError {
    use store::sqlite_snapshot::SqliteStageFailure as Stage;
    match stage {
        Stage::Operation(failure) => primary(&failure),
        Stage::NotStarted(error) => error,
        Stage::Settlement { operation, refusal, .. } => match operation { Err(failure) => { let operation = primary(&failure); if operation.kind == semio_framework_value::ValueRefusalKind::Canceled && refusal.kind == semio_framework_value::ValueRefusalKind::Canceled { refusal.with_retained_progress(operation.retained_progress()) } else { operation } }, Ok(_) => refusal },
        Stage::WorkSettlement { operation, .. } => match operation { Err(failure) => primary(&failure), Ok(_) => semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "work settlement refused") },
    }
}

/// 📥️ Runs one output-slot receiving call and returns its published owner or primary refusal.
pub fn slot<T>(call: impl FnOnce(&mut Option<store::os_store::serialization::SnapshotReceivingOutput<T>>) -> store::os_store::serialization::SnapshotReceivingOutcome) -> Result<T, semio_framework_value::ValueError> {
    let mut output = None;
    let turn = call(&mut output);
    match (turn.result, output.take()) {
        (Err(stage), leftover) => { std::mem::forget(leftover); Err(staged(stage)) }
        (Ok(()), Some(Ok(value))) => Ok(value),
        (Ok(()), Some(Err(stage))) => Err(staged(stage)),
        (Ok(()), None) => panic!("original receiving published no output"),
    }
}

/// 🛡️ Runs one validation call and returns its primary refusal.
pub fn validated(call: impl FnOnce(&mut Option<store::os_store::serialization::SnapshotReceivingOutput<store::io_schema::IoResult<()>>>) -> store::os_store::serialization::SnapshotReceivingOutcome) -> Result<(), semio_framework_value::ValueError> {
    match slot(call)? { Ok(_) => Ok(()), Err(error) => Err(primary(&error.cause)) }
}

/// 🧾️ Collapses one received native outcome to its published value or primary refusal.
pub fn received<O>(turn: RetainedWorkOutcome<O, store::io::control::NativeSnapshotFailure>) -> Result<O, semio_framework_value::ValueError> { turn.result.map_err(|failure| primary(&failure)) }

struct WorkScope;
impl OriginalWorkRecipient for WorkScope {
    type Context<'turn, 'source> = () where 'source: 'turn;
    fn validate_context<'turn, 'source>(&self, _: &()) -> Result<(), semio_framework_value::ValueError> where 'source: 'turn { Ok(()) }
    fn source_turn<'turn, 'source>(&self, _: &()) -> Option<OriginalSourceTurnIdentity> where 'source: 'turn { None }
    fn admits<'reply, 'turn, 'source>(&mut self, _: &mut (), quote: OriginalWorkQuote<'reply>, _: u64, _: u64) -> OriginalWorkQuoteComplete<'reply> where 'source: 'turn { quote.complete(Ok(true)) }
    fn settle_partial<'reply, 'turn, 'source>(&mut self, _: &mut (), check: OriginalWorkPartialCheck<'reply>) -> OriginalWorkPartialCheckComplete<'reply> where 'source: 'turn { check.complete(Ok(())) }
    fn recover_partial<'reply, 'turn, 'source>(&mut self, _: &mut (), check: OriginalWorkPartialCheck<'reply>) -> OriginalWorkPartialCheckComplete<'reply> where 'source: 'turn { check.complete(Ok(())) }
}

struct ConstructionScope { grant: RetainedCloneGrant, submitted: semio_framework_value::RetainedCloneProgress }
impl OriginalConstructionRecipient for ConstructionScope {
    type Context<'turn, 'source> = () where 'source: 'turn;
    fn validate_context<'turn, 'source>(&self, _: &()) -> Result<(), semio_framework_value::ValueError> where 'source: 'turn { Ok(()) }
    fn source_turn<'turn, 'source>(&self, _: &()) -> Option<OriginalSourceTurnIdentity> where 'source: 'turn { None }
    fn grant<'reply, 'turn, 'source>(&mut self, _: &mut (), quote: OriginalConstructionGrant<'reply>) -> OriginalConstructionGrantComplete<'reply> where 'source: 'turn { quote.complete(Ok(self.grant)) }
    fn accept<'reply, 'turn, 'source>(&mut self, _: &mut (), receipt: OriginalConstructionAccept<'reply>) -> OriginalConstructionAcceptComplete<'reply> where 'source: 'turn { let progress = receipt.progress(); let posted = self.submitted.checked_add(progress).map(|total| self.submitted = total); receipt.complete(posted) }
    fn submitted<'turn, 'source>(&self, _: &()) -> semio_framework_value::RetainedCloneProgress where 'source: 'turn { self.submitted }
}

/// 🧰️ Lends the genuine Work and construction loans an original caller gives its snapshot owner for the duration of one operation.
pub fn with_original_authorities<R>(grant: RetainedCloneGrant, operation: impl FnOnce(&mut OriginalWorkLoan<'_>, &mut OriginalConstructionLoan<'_>) -> R) -> R {
    let mut construction = OriginalConstructionAuthority::new(ConstructionScope { grant, submitted: Default::default() });
    let mut work = OriginalWorkAuthority::new(WorkScope);
    construction.with_loan(&mut (), |construction| work.with_loan(&mut (), |work| operation(work, construction))).expect("fixture construction authority").expect("fixture work authority")
}

/// 🎂️ Reports whether the next decoder close must first pay an original failure birth.
pub fn birth_required(native: &mut NativeDecodeControl<'_>, grant: RetainedCloneGrant) -> bool {
    native.with_retirement::<_, semio_framework_value::ValueError>(grant, |original| Ok(original.requires_original_birth())).unwrap_or(false)
}
