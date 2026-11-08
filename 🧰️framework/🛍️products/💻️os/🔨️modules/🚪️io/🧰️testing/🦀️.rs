//! 🧰️ Explicit original caller policy and funded close for snapshot file IO laws.
use semio_framework_io_control::{NativeSnapshotDecodeOwner, NativeSnapshotEncodeOwner};
use semio_framework_io_sqlite_snapshot::{SqliteDatabaseLimits, SqliteSnapshotControl, SqliteSnapshotProgress};
use semio_framework_value::{RetainedCloneGrant, ErasedSnapshotRetirement, NativeEncodeControl, native_encoding::NativeEncodeRetirementRecipient};
use semio_framework_value_native_owned::{native_decoding::NativeDecodeControl, native_decoding::NativeDecodeRetirementRecipient};
use semio_framework_value_native_owned::native_decoding::{OriginalWorkAuthority,OriginalWorkRecipient,OriginalWorkQuote,OriginalWorkQuoteComplete,OriginalWorkPartialCheck,OriginalWorkPartialCheckComplete,OriginalSourceTurnIdentity};

/// 🎟️ Independent finite authority supplied by each file law's caller.
#[derive(Clone,Copy)]
pub struct SnapshotFileCaller{pub grant:RetainedCloneGrant,pub native_maximum_bytes:usize,pub close_grant:RetainedCloneGrant,pub maximum_close_turns:usize}
/// 📏️ The authored general file law policy, independent of observed producer demand.
pub const ORIGINAL_FILE_CALLER:SnapshotFileCaller=SnapshotFileCaller{
 grant:RetainedCloneGrant{maximum_items:1<<24,maximum_copy_bytes:1<<30,maximum_capacity_bytes:1<<30,maximum_release_bytes:1<<30,maximum_depth:4096},
 native_maximum_bytes:1<<30,
 close_grant:RetainedCloneGrant{maximum_items:4096,maximum_copy_bytes:1<<20,maximum_capacity_bytes:1<<30,maximum_release_bytes:1<<30,maximum_depth:4096},
 maximum_close_turns:65536
};

/// 🎟️ Admits every observed decode effect: the file laws bound work through their grants, not through a job loan.
struct AdmittedLoan;
impl OriginalWorkRecipient for AdmittedLoan{
 type Context<'t,'s>=() where 's:'t;
 fn validate_context<'t,'s>(&self,_:&())->Result<(),semio_framework_value::ValueError> where 's:'t{Ok(())}
 fn source_turn<'t,'s>(&self,_:&())->Option<OriginalSourceTurnIdentity> where 's:'t{None}
 fn admits<'r,'t,'s>(&mut self,_:&mut (),quote:OriginalWorkQuote<'r>,observed:u64,additional:u64)->OriginalWorkQuoteComplete<'r> where 's:'t{quote.complete(Ok(observed.checked_add(additional).is_some()))}
 fn settle_partial<'r,'t,'s>(&mut self,_:&mut (),check:OriginalWorkPartialCheck<'r>)->OriginalWorkPartialCheckComplete<'r> where 's:'t{check.complete(Ok(()))}
 fn recover_partial<'r,'t,'s>(&mut self,_:&mut (),check:OriginalWorkPartialCheck<'r>)->OriginalWorkPartialCheckComplete<'r> where 's:'t{check.pending()}
}
/// 🏁️ Drives one funded close turn to its outcome; a suspended poll is a fixture fault.
fn completed<T,E:std::fmt::Debug>(poll:semio_framework_value::RetainedWorkPoll<T,E>,label:&str)->T{match poll{semio_framework_value::RetainedWorkPoll::Complete(turn)=>turn.result.expect(label),semio_framework_value::RetainedWorkPoll::Pending{..}=>panic!("original file close stayed pending")}}
/// 🧭️ Drives a file law future that completes without external wakeups to its output on the calling thread.
fn drive_ready<T>(future:impl std::future::Future<Output=T>)->T{let mut future=std::pin::pin!(future);let mut context=std::task::Context::from_waker(std::task::Waker::noop());loop{if let std::task::Poll::Ready(value)=future.as_mut().poll(&mut context){return value}std::thread::yield_now();}}
/// 🛬️ Loans original decode and SQL controls, preserving the law's actual SQL policy and progress callback.
pub async fn with_snapshot_file_decoding<T>(policy:SnapshotFileCaller,limits:SqliteDatabaseLimits,progress:&mut dyn FnMut(SqliteSnapshotProgress)->bool,operation:impl AsyncFnOnce(&mut SqliteSnapshotControl<'_>,&mut NativeSnapshotDecodeOwner<'_,'_>)->T)->T{
 let mut workspace=NativeDecodeRetirementRecipient::new();let mut recipient=NativeDecodeRetirementRecipient::new();let mut observe=|_|true;let mut native=NativeDecodeControl::new(policy.native_maximum_bytes,&mut observe);native.install_retirement_recipient(&mut recipient).expect("original file decode recipient installs");
 let result={let mut control=SqliteSnapshotControl::new(progress,limits);control.install_native_retirement_recipient(&mut workspace).expect("original file SQL recipient installs");OriginalWorkAuthority::new(AdmittedLoan).with_loan(&mut (),|loan|native.with_work_recipient(loan,|native|{let mut owner=NativeSnapshotDecodeOwner::new(native,policy.grant);Ok::<_,semio_framework_value::ValueError>(drive_ready(operation(&mut control,&mut owner)))})).expect("original file decode work loan issues").expect("original file decode work loan installs")};
 for _ in 0..policy.maximum_close_turns{if !native.has_retirement_owner(){break}OriginalWorkAuthority::new(AdmittedLoan).with_loan(&mut (),|loan|native.with_work_recipient(loan,|native|Ok::<_,semio_framework_value::ValueError>(native.with_effect(policy.close_grant,u64::MAX,|native,effect|completed(native.close_retirement_recipient(effect),"original file decode close is funded"))))).expect("original file close work loan issues").expect("original file close work loan installs").expect("original file decode close effect opens").expect("original file decode close work is admitted");}assert!(!native.has_retirement_owner(),"original file decode close exceeds authored turn ceiling");
 for _ in 0..policy.maximum_close_turns{if workspace.terminal_is_empty(){break}OriginalWorkAuthority::new(AdmittedLoan).with_loan(&mut (),|loan|native.with_work_recipient(loan,|native|Ok::<_,semio_framework_value::ValueError>(native.with_effect(policy.close_grant,u64::MAX,|_,effect|completed(workspace.close_step(effect),"original file SQL close is funded"))))).expect("original file close work loan issues").expect("original file close work loan installs").expect("original file SQL close effect opens").expect("original file SQL close work is admitted");}assert!(workspace.terminal_is_empty(),"original file SQL close exceeds authored turn ceiling");result
}

/// 🛫️ Loans original encode and SQL controls without deriving native authority from SQLite demand.
pub async fn with_snapshot_file_encoding<T>(policy:SnapshotFileCaller,limits:SqliteDatabaseLimits,progress:&mut dyn FnMut(SqliteSnapshotProgress)->bool,operation:impl AsyncFnOnce(&mut SqliteSnapshotControl<'_>,&mut NativeSnapshotEncodeOwner<'_,'_>)->T)->T{
 let mut workspace=NativeDecodeRetirementRecipient::new();let mut recipient=NativeEncodeRetirementRecipient::new();let mut observe=|_|true;let mut native=NativeEncodeControl::new(policy.native_maximum_bytes,&mut observe);native.install_retirement_recipient(&mut recipient).expect("original file encode recipient installs");
 let result={let mut control=SqliteSnapshotControl::new(progress,limits);control.install_native_retirement_recipient(&mut workspace).expect("original file SQL recipient installs");OriginalWorkAuthority::new(AdmittedLoan).with_loan(&mut (),|loan|native.with_work_recipient(loan,|native|{let mut owner=NativeSnapshotEncodeOwner::new(native,policy.grant);Ok::<_,semio_framework_value::ValueError>(drive_ready(operation(&mut control,&mut owner)))})).expect("original file encode work loan issues").expect("original file encode work loan installs")};
 for _ in 0..policy.maximum_close_turns{if !native.has_retirement_owner(){break}OriginalWorkAuthority::new(AdmittedLoan).with_loan(&mut (),|loan|native.with_work_recipient(loan,|native|Ok::<_,semio_framework_value::ValueError>(native.with_effect(policy.close_grant,u64::MAX,|native,effect|completed(native.close_retirement_recipient(effect),"original file encode close is funded"))))).expect("original file close work loan issues").expect("original file close work loan installs").expect("original file encode close effect opens").expect("original file encode close work is admitted");}assert!(!native.has_retirement_owner(),"original file encode close exceeds authored turn ceiling");
 for _ in 0..policy.maximum_close_turns{if workspace.terminal_is_empty(){break}OriginalWorkAuthority::new(AdmittedLoan).with_loan(&mut (),|loan|native.with_work_recipient(loan,|native|Ok::<_,semio_framework_value::ValueError>(native.with_effect(policy.close_grant,u64::MAX,|_,effect|completed(workspace.close_step(effect),"original file SQL close is funded"))))).expect("original file close work loan issues").expect("original file close work loan installs").expect("original file SQL close effect opens").expect("original file SQL close work is admitted");}assert!(workspace.terminal_is_empty(),"original file SQL close exceeds authored turn ceiling");result
}

