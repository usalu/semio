//! 🛬️ Original caller authorities for artifact tests: the native snapshot owners and SQLite recipient every snapshot codec receives from its caller.
//!
//! Mounted by `#[path]` into the test modules of artifact crates that depend on `semio_os_kernel` and `semio_framework_value`.
//!
//! @see 🏪️store `NativeSnapshotDecodeOwner` / `NativeSnapshotEncodeOwner`, `SqliteSnapshotControl::install_native_retirement_recipient`.

use semio_os_kernel::io::io_mechanism::{io_run_with_snapshot_control, IoRunControl};
use semio_os_kernel::io_schema::{IoError, IoPayload, IoResult, IoRoute};
use semio_os_kernel::os_store::serialization::{ArtifactSqliteSnapshotCodec, ArtifactSqliteSnapshotHost, ArtifactSqliteSnapshotTurn, SnapshotReceivingOutcome, SnapshotReceivingOutput};
use semio_framework_artifact_reference::ArtifactDialect;
use semio_framework_value::{ValueError, ValueRefusalKind};
use semio_os_kernel::sqlite_snapshot::{SnapshotEncoding, SqliteDatabase, SqliteDatabaseLimits, SqliteSnapshotControl, SqliteSnapshotProgress, SqliteStageFailure};
use semio_os_kernel::{NativeSnapshotDecodeOwner, NativeSnapshotEncodeOwner};
use semio_framework_value_native_owned::native_decoding::NativeDecodeRetirementRecipient;
use semio_framework_value::native_encoding::NativeEncodeRetirementRecipient;
use semio_framework_value::{ErasedSnapshotRetirement as _, NativeEncodeControl, OriginalRetirementFailure, RetainedAllocationEffect, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
use semio_framework_value_testing_admission::{bank_run, driven, finite_work, sourced_work, BankWallet, EffectBank, FiniteWork, OriginalConstructionAccept, OriginalConstructionAcceptComplete, OriginalConstructionAuthority, OriginalConstructionGrant, OriginalConstructionGrantComplete, OriginalConstructionRecipient, OriginalSourceTurnIdentity, OriginalWorkAuthority, RetainedWorkPoll};

/// 📏️ Native allocation ceiling the original test callers grant.
pub const NATIVE_MAXIMUM_BYTES: usize = 1 << 30;

/// 🎟️ The generous five-axis ownership grant of the original test caller.
pub const fn original_grant() -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 1 << 24, maximum_copy_bytes: 1 << 30, maximum_capacity_bytes: 1 << 30, maximum_release_bytes: 1 << 30, maximum_depth: 4_096 }
}

/// 📏️ Body bytes every original close turn may borrow.
const CLOSE_BODY: usize = 4_096;

/// ⚙️ Work units every original close turn may perform.
const CLOSE_WORK: u64 = 1 << 20;

/// 🏦️ The finite effect wallet the original test caller closes its owners from.
pub fn original_bank() -> EffectBank {
    EffectBank::new(BankWallet::new(original_grant(), 1 << 40))
}

/// 🏭️ The physical construction recipient of the original caller: it grants its full grant and totals every accepted receipt.
struct Physical { submitted: RetainedCloneProgress }

impl OriginalConstructionRecipient for Physical {
    type Context<'turn, 'source> = () where 'source: 'turn;
    fn validate_context<'turn, 'source>(&self, _: &()) -> Result<(), ValueError> where 'source: 'turn { Ok(()) }
    fn source_turn<'turn, 'source>(&self, _: &()) -> Option<OriginalSourceTurnIdentity> where 'source: 'turn { None }
    fn grant<'a, 'turn, 'source>(&mut self, _: &mut (), quote: OriginalConstructionGrant<'a>) -> OriginalConstructionGrantComplete<'a> where 'source: 'turn { quote.complete(Ok(original_grant())) }
    fn accept<'a, 'turn, 'source>(&mut self, _: &mut (), receipt: OriginalConstructionAccept<'a>) -> OriginalConstructionAcceptComplete<'a> where 'source: 'turn {
        let result = self.submitted.checked_add(receipt.progress()).map(|sum| self.submitted = sum);
        receipt.complete(result)
    }
    fn submitted<'turn, 'source>(&self, _: &()) -> RetainedCloneProgress where 'source: 'turn { self.submitted }
}

/// 🪪️ The Work and construction authorities the original caller lends to every native control of one run.
struct Lent { work: OriginalWorkAuthority<FiniteWork>, construction: OriginalConstructionAuthority<Physical> }

macro_rules! lend {
    ($lent:expr, $control:expr, $operation:expr) => {{
        let lent: &mut Lent = $lent;
        let mut output = None;
        lent.construction
            .with_loan(&mut (), |construction| {
                lent.work.with_loan(&mut (), |work| {
                    $control.with_work_recipient(work, |control| control.with_construction_recipient(construction, |control| { output = Some($operation(control)); Ok::<(), ValueError>(()) }))
                })
            })
            .expect("original construction loan")
            .expect("original work loan")
            .expect("original authorities lent");
        output.expect("lent operation ran")
    }};
}

impl Lent {
    fn new() -> Self { Self::sourced(None) }
    fn sourced(source: Option<OriginalSourceTurnIdentity>) -> Self { Self { work: source.map_or_else(|| finite_work(u64::MAX, None), sourced_work), construction: OriginalConstructionAuthority::new(Physical { submitted: RetainedCloneProgress::default() }) } }
}

/// 🧾️ Takes the progress a settled close turn paid; a pending turn paid nothing.
fn settled(poll: RetainedWorkPoll<RetainedCloneStep, OriginalRetirementFailure>, what: &str) -> RetainedCloneProgress {
    match poll {
        RetainedWorkPoll::Complete(turn) => turn.result.expect(what).progress(),
        RetainedWorkPoll::Pending { .. } => RetainedCloneProgress::default(),
    }
}

/// ♻️ Runs one funded close turn of the original caller and returns its paid progress.
fn close_turn(bank: &mut EffectBank, what: &str, close: impl FnOnce(&mut RetainedAllocationEffect<'_>) -> RetainedWorkPoll<RetainedCloneStep, OriginalRetirementFailure>) -> RetainedCloneProgress {
    settled(bank_run(bank, original_grant(), CLOSE_WORK, close).0, what)
}

/// ♻️ Runs one funded reply-shaped close turn of the original caller and returns its paid progress.
fn encode_turn(bank: &mut EffectBank, what: &str, native: &mut NativeEncodeControl<'_>) -> RetainedCloneProgress {
    settled(driven::<OriginalRetirementFailure>(bank, original_grant(), CLOSE_WORK, |effect, reply, recover| if recover { native.recover_close_retirement_recipient_into(CLOSE_BODY, effect, reply) } else { native.close_retirement_recipient_into(CLOSE_BODY, effect, reply) }).0, what)
}

/// 🚮️ Runs one funded close turn of the original decode recipient whose control already holds its Work and construction loans; a selected failure birth runs first through its separate driver.
fn close_decode_installed(native: &mut NativeDecodeControl<'_>) -> RetainedCloneProgress {
    let birth = native.with_retirement::<_, ValueError>(original_grant(), |loan| Ok(loan.requires_original_birth().then(|| loan.close_original_birth_step(original_grant())))).expect("original decode failure birth selects");
    match birth {
        Some(poll) => settled(poll, "original decode failure birth closes"),
        None => settled(native.with_effect(original_grant(), CLOSE_WORK, |native, effect| native.close_retirement_recipient(CLOSE_BODY, effect)).expect("original decode close effect opens").expect("original decode close work is admitted"), "original decode recipient closes"),
    }
}

/// 🚮️ Lends the original Work and construction authorities to the decode control for one funded close turn.
fn close_decode(lent: &mut Lent, native: &mut NativeDecodeControl<'_>) -> RetainedCloneProgress {
    lend!(lent, native, |native: &mut NativeDecodeControl<'_>| close_decode_installed(native))
}

/// 🚮️ Closes every retired owner of a caller-built decode control through the original failure-birth and funded close turns.
pub fn close_retired(native: &mut NativeDecodeControl<'_>) {
    let mut lent = Lent::new();
    while native.has_retirement_owner() {
        close_decode(&mut lent, native);
    }
}

/// 🧲️ Runs `operation` under an original native decode owner and closes every retired owner afterwards.
pub fn with_decoding<T>(operation: impl FnOnce(&mut NativeSnapshotDecodeOwner<'_, '_>) -> T) -> T {
    with_decoding_observed(&mut |_| true, operation)
}

/// 🧲️ Runs `operation` under an original native decode owner whose caller-owned progress callback is `callback`, the authority that cancels interior native decoding.
pub fn with_decoding_observed<T>(callback: &mut dyn FnMut(semio_framework_value_native_owned::native_decoding::NativeDecodeProgress) -> bool, operation: impl FnOnce(&mut NativeSnapshotDecodeOwner<'_, '_>) -> T) -> T {
    with_decoding_granted(original_grant(), callback, operation)
}

/// 🧲️ Runs `operation` under an original native decode owner that holds exactly `grant` as its ownership authority.
pub fn with_decoding_granted<T>(grant: RetainedCloneGrant, callback: &mut dyn FnMut(semio_framework_value_native_owned::native_decoding::NativeDecodeProgress) -> bool, operation: impl FnOnce(&mut NativeSnapshotDecodeOwner<'_, '_>) -> T) -> T {
    let source = original_bank();
    let mut recipient = NativeDecodeRetirementRecipient::new();
    let mut native = NativeDecodeControl::new(NATIVE_MAXIMUM_BYTES, callback);
    native.install_retirement_recipient(&mut recipient).expect("original decode recipient installs");
    let mut lent = Lent::sourced(Some(source.source_witness().identity()));
    let result = native.with_source_witness(Some(source.source_witness()), |native| lend!(&mut lent, native, |native: &mut NativeDecodeControl<'_>| operation(&mut NativeSnapshotDecodeOwner::new(native, grant))));
    while native.has_retirement_owner() {
        close_decode(&mut lent, &mut native);
    }
    result
}

/// 🛫️ Runs `operation` under an original native encode owner and closes every retired owner afterwards.
pub fn with_encoding<T>(operation: impl FnOnce(&mut NativeSnapshotEncodeOwner<'_, '_>) -> T) -> T {
    with_encoding_observed(&mut |_| true, operation)
}

/// 🛫️ Runs `operation` under an original native encode owner whose caller-owned progress callback is `callback`, the authority that cancels interior native encoding.
pub fn with_encoding_observed<T>(callback: &mut dyn FnMut(semio_framework_value::native_encoding::NativeEncodeProgress) -> bool, operation: impl FnOnce(&mut NativeSnapshotEncodeOwner<'_, '_>) -> T) -> T {
    let grant = original_grant();
    let mut recipient = NativeEncodeRetirementRecipient::new();
    let mut native = NativeEncodeControl::new(NATIVE_MAXIMUM_BYTES, callback);
    native.install_retirement_recipient(&mut recipient).expect("original encode recipient installs");
    let result = lend!(&mut Lent::new(), native, |native: &mut NativeEncodeControl<'_>| operation(&mut NativeSnapshotEncodeOwner::new(native, grant)));
    let mut bank = original_bank();
    while native.has_retirement_owner() {
        encode_turn(&mut bank, "original encode recipient closes", &mut native);
    }
    result
}

/// 🕑️ Drives `future` to completion on the calling thread, which keeps its synchronous original loans lent for every await point.
fn drive<F: std::future::Future>(future: F) -> F::Output {
    let mut future = std::pin::pin!(future);
    let mut context = std::task::Context::from_waker(std::task::Waker::noop());
    loop {
        match future.as_mut().poll(&mut context) {
            std::task::Poll::Ready(output) => return output,
            std::task::Poll::Pending => std::thread::yield_now(),
        }
    }
}

/// 🕑️ Runs an async `operation` under an original native encode owner that holds the lent Work and construction authorities across its awaits and closes every retired owner afterwards.
pub async fn with_encoding_async<T>(operation: impl AsyncFnOnce(&mut NativeSnapshotEncodeOwner<'_, '_>) -> T) -> T {
    let grant = original_grant();
    let mut callback = |_| true;
    let mut recipient = NativeEncodeRetirementRecipient::new();
    let mut native = NativeEncodeControl::new(NATIVE_MAXIMUM_BYTES, &mut callback);
    native.install_retirement_recipient(&mut recipient).expect("original encode recipient installs");
    let result = lend!(&mut Lent::new(), native, |native: &mut NativeEncodeControl<'_>| drive(operation(&mut NativeSnapshotEncodeOwner::new(native, grant))));
    let mut bank = original_bank();
    while native.has_retirement_owner() {
        encode_turn(&mut bank, "original encode recipient closes", &mut native);
    }
    result
}

/// 🗄️ Runs `operation` under a SQLite snapshot control whose caller installed the original retirement recipient, then closes it.
pub fn with_sqlite_recipient<T>(limits: SqliteDatabaseLimits, accept: &mut dyn FnMut(SqliteSnapshotProgress) -> bool, operation: impl FnOnce(&mut SqliteSnapshotControl<'_>) -> T) -> T {
    let mut recipient = NativeDecodeRetirementRecipient::new();
    let result = {
        let mut control = SqliteSnapshotControl::new(accept, limits);
        control.install_native_retirement_recipient(&mut recipient).expect("original SQLite recipient installs");
        operation(&mut control)
    };
    let mut bank = original_bank();
    while !recipient.terminal_is_empty() {
        close_turn(&mut bank, "original SQLite recipient closes", |effect| recipient.close_step(CLOSE_BODY, effect));
    }
    result
}

/// 🚏️ Executes a routed IO payload under original native and SQLite authorities, then closes both native directions. The decode and encode controls install their own Work and construction loans for the whole run, so both share the one lifetime the run's `IoRunControl` demands.
pub async fn io_run(route: &IoRoute, payload: IoPayload, limits: SqliteDatabaseLimits, progress: &mut dyn FnMut(SqliteSnapshotProgress) -> bool) -> IoResult<IoPayload> {
    let grant = original_grant();
    let mut receive = |_| true;
    let mut emit = |_| true;
    let mut receive_recipient = NativeDecodeRetirementRecipient::new();
    let mut emit_recipient = NativeEncodeRetirementRecipient::new();
    let (mut decoding, mut encoding) = (Lent::new(), Lent::new());
    let mut result = None;
    decoding.construction
        .with_loan(&mut (), |decode_construction| {
            decoding.work.with_loan(&mut (), |decode_work| {
                encoding.construction.with_loan(&mut (), |encode_construction| {
                    encoding.work.with_loan(&mut (), |encode_work| {
                        let (mut decode_construction, mut decode_work) = (decode_construction.reborrow(), decode_work.reborrow());
                        let (mut encode_construction, mut encode_work) = (encode_construction.reborrow(), encode_work.reborrow());
                        let mut receive_control = NativeDecodeControl::new(NATIVE_MAXIMUM_BYTES, &mut receive);
                        let mut emit_control = NativeEncodeControl::new(NATIVE_MAXIMUM_BYTES, &mut emit);
                        receive_control.install_retirement_recipient(&mut receive_recipient).expect("original decode recipient installs");
                        emit_control.install_retirement_recipient(&mut emit_recipient).expect("original encode recipient installs");
                        receive_control.install_work_recipient(&mut decode_work).expect("original decode work loan installs");
                        receive_control.install_construction_recipient(&mut decode_construction).expect("original decode construction loan installs");
                        emit_control.install_work_recipient(&mut encode_work).expect("original encode work loan installs");
                        emit_control.install_construction_recipient(&mut encode_construction).expect("original encode construction loan installs");
                        let mut native = IoRunControl::new(&mut receive_control, &mut emit_control, grant);
                        let mut snapshot = SqliteSnapshotControl::new(progress, limits);
                        result = Some(io_run_with_snapshot_control(route, &mut Some(payload), &mut native, &mut snapshot).into_inner());
                        close_original(&mut native);
                        drop(native);
                        Ok::<(), ValueError>(())
                    })
                })
            })
        })
        .expect("original decode construction loan")
        .expect("original decode work loan")
        .expect("original encode construction loan")
        .expect("original encode work loan")
        .expect("original io run ran");
    result.expect("original io run produced its result")
}

/// 🧹️ Closes both native directions of one routed run through host-funded turns charged to their original grants; both controls already hold their installed loans.
fn close_original(native: &mut IoRunControl<'_, '_>) {
    let mut bank = original_bank();
    {
        let mut owner = native.snapshot_decode().expect("original decode direction");
        while owner.native().has_retirement_owner() {
            let progress = close_decode_installed(owner.native());
            owner.record_progress(progress).expect("original decode close is funded");
        }
    }
    let mut owner = native.snapshot_encode().expect("original encode direction");
    while owner.native().has_retirement_owner() {
        let progress = encode_turn(&mut bank, "original encode recipient closes", owner.native());
        owner.record_progress(progress).expect("original encode close is funded");
    }
}

/// 🚫️ Why an original snapshot turn produced no value, for tests that only assert refusal.
#[derive(Debug)]
pub struct OriginalSnapshotRefusal(pub String);

fn stage_label<T, E: std::fmt::Debug>(failure: &SqliteStageFailure<T, E>) -> String {
    match failure {
        SqliteStageFailure::WorkSettlement { .. } => "work settlement".to_owned(),
        SqliteStageFailure::NotStarted(error) => format!("not started: {error:?}"),
        SqliteStageFailure::Operation(error) => format!("operation: {error:?}"),
        SqliteStageFailure::Settlement { refusal, .. } => format!("settlement: {refusal:?}"),
    }
}

fn received<T>(output: Option<SnapshotReceivingOutput<T>>, outcome: SnapshotReceivingOutcome) -> Result<T, OriginalSnapshotRefusal> {
    match (output, outcome.result) {
        (Some(Ok(value)), Ok(())) => Ok(value),
        (Some(Err(failure)), _) => Err(OriginalSnapshotRefusal(stage_label(&failure))),
        (_, Err(failure)) => Err(OriginalSnapshotRefusal(stage_label(&failure))),
        (None, Ok(())) => Err(OriginalSnapshotRefusal("no output".to_owned())),
    }
}

/// 🛬️ Decodes a native snapshot through its owner under original authorities and returns the received value.
pub fn decode_native<T: ArtifactSqliteSnapshotHost>(payload: &IoPayload, control: &mut SqliteSnapshotControl<'_>) -> Result<T, OriginalSnapshotRefusal> {
    with_decoding(|owner| {
        let mut output = None;
        let outcome = T::decode_sqlite_snapshot_native(payload, control, owner, &mut output);
        received(output, outcome)
    })
}

/// 🛫️ Encodes a native snapshot through its owner under original authorities and returns the received payload.
pub fn encode_native<T: ArtifactSqliteSnapshotHost>(value: &T, encoding: SnapshotEncoding, control: &mut SqliteSnapshotControl<'_>) -> Result<IoPayload, OriginalSnapshotRefusal> {
    with_encoding(|owner| {
        let mut output = None;
        let outcome = value.encode_sqlite_snapshot_native(encoding, control, owner, &mut output);
        received(output, outcome)
    })
}

fn settled_io<T>(output: Option<IoResult<T>>, turn: ArtifactSqliteSnapshotTurn) -> IoResult<T> {
    match (output, turn.result) {
        (Some(result), Ok(())) => result,
        (_, Err(error)) => Err(IoError::from_value_error(ValueError::new(ValueRefusalKind::InvalidValue, format!("original snapshot turn failed: {error:?}")))),
        (None, Ok(())) => Err(IoError::from_value_error(ValueError::literal(ValueRefusalKind::InvalidValue, "original snapshot turn produced no output"))),
    }
}

/// 📤️ Runs a codec's relational export of a native payload under original authorities.
pub fn export_sqlite(provider: &ArtifactSqliteSnapshotCodec, schema: &str, dialect: &ArtifactDialect, payload: &IoPayload, control: &mut SqliteSnapshotControl<'_>) -> IoResult<SqliteDatabase> {
    with_decoding(|owner| {
        let mut output = None;
        let turn = (provider.export)(schema, dialect, payload, &mut output, control, owner);
        settled_io(output, turn)
    })
}

/// 📥️ Runs a codec's relational import into a native payload under original authorities.
pub fn import_sqlite(provider: &ArtifactSqliteSnapshotCodec, schema: &str, dialect: &ArtifactDialect, database: SqliteDatabase, encoding: SnapshotEncoding, control: &mut SqliteSnapshotControl<'_>) -> IoResult<IoPayload> {
    with_encoding(|owner| {
        let mut output = None;
        let turn = (provider.import)(schema, dialect, &mut Some(database), encoding, &mut output, control, owner);
        settled_io(output, turn)
    })
}
