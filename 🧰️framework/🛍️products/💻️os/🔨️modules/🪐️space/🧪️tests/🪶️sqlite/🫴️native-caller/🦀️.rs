//! 🫴️ Actual original native caller for the Semio SQLite snapshot suites: independent body and close grants drive every erased conversion.
use semio_framework_os_kernel::{self as store, sqlite_snapshot::{SqliteDatabaseLimits, SqliteSnapshotControl}};
use semio_framework_value::{NativeDecodeControl, NativeEncodeControl};

pub fn native_caller_policy() -> serde_json::Value { serde_json::from_str(include_str!("🎟️original.json")).unwrap() }
pub trait OriginalNativeRefusalReceipt { fn original_native_receipt(&self) -> semio_framework_value::retained_clone::RetainedCloneProgress; }
impl OriginalNativeRefusalReceipt for semio_framework_value::ValueError { fn original_native_receipt(&self) -> semio_framework_value::retained_clone::RetainedCloneProgress { self.retained_progress() } }
impl OriginalNativeRefusalReceipt for store::io_schema::IoError { fn original_native_receipt(&self) -> semio_framework_value::retained_clone::RetainedCloneProgress { self.cause.retained_progress() } }
macro_rules! original_native_caller {
    ($name:ident,$module:ident,$control:ident,$owner:ident,$recipient:ident,$allocation:ident,$progress:ident) => {
        pub fn $name<T, E: OriginalNativeRefusalReceipt>(law: &serde_json::Value, limits: SqliteDatabaseLimits, callback: &mut dyn FnMut(store::sqlite_snapshot::SqliteSnapshotProgress) -> bool, original_observer: &mut dyn FnMut(semio_framework_value::$module::$progress) -> bool, original_continuation: impl FnOnce(&mut semio_framework_value::$control<'_>), operation: impl FnOnce(&mut SqliteSnapshotControl<'_>, &mut store::$owner<'_, '_>) -> Result<T, E>) -> Result<T, E> {
            use semio_framework_value::$module::{$recipient, $allocation};
            use std::sync::atomic::{AtomicUsize, Ordering};
            let grant = serde_json::from_value::<semio_framework_value::RetainedCloneGrant>(law["bodyGrant"].clone()).unwrap();
            let close = serde_json::from_value::<semio_framework_value::RetainedCloneGrant>(law["closeGrant"].clone()).unwrap();
            let accepted = AtomicUsize::new(0usize);
            let mut original_allocation = |request: $allocation| { assert_eq!(request.owned_bytes, accepted.load(Ordering::Relaxed)); accepted.store(request.next_owned_bytes, Ordering::Relaxed); Ok(()) };
            let mut recipient = $recipient::new();
            let mut native = semio_framework_value::$control::new_forwarded(law["nativeMaximumBytes"].as_u64().unwrap() as usize, original_observer, &mut original_allocation);
            native.install_retirement_recipient(&mut recipient).unwrap();
            let mut sql = SqliteSnapshotControl::new(callback, limits);
            let mut owner = store::$owner::new(&mut native, grant);
            let result = operation(&mut sql, &mut owner);
            assert!(owner.progress().fits(grant));
            if let Err(error) = &result { assert!(error.original_native_receipt().fits(grant)); }
            drop(owner);
            assert_eq!(native.owned_bytes(), accepted.load(Ordering::Relaxed));
            let held = native.has_retirement_owner();
            let denied = serde_json::from_value::<semio_framework_value::RetainedCloneGrant>(law["deniedCloseGrant"].clone()).unwrap();
            assert_eq!(native.close_retirement_recipient(denied).unwrap().progress(), Default::default());
            assert_eq!(native.has_retirement_owner(), held);
            original_continuation(&mut native);
            for _ in 0..law["maximumCloseTurns"].as_u64().unwrap() {
                if !native.has_retirement_owner() { return result; }
                assert!(native.close_retirement_recipient(close).unwrap().progress().fits(close));
            }
            assert!(!native.has_retirement_owner(), "original native caller did not reach funded terminal");
            result
        }
    };
}
original_native_caller!(with_original_decode, native_decoding, NativeDecodeControl, NativeSnapshotDecodeOwner, NativeDecodeRetirementRecipient, NativeDecodeAllocation, NativeDecodeProgress);
original_native_caller!(with_original_encode, native_encoding, NativeEncodeControl, NativeSnapshotEncodeOwner, NativeEncodeRetirementRecipient, NativeEncodeAllocation, NativeEncodeProgress);

pub fn retire_original_output<T: semio_framework_value::retirement::RetireOwned>(source: T, law: &serde_json::Value) {
    let close = serde_json::from_value::<semio_framework_value::RetainedCloneGrant>(law["closeGrant"].clone()).unwrap();
    let mut owner = semio_framework_value::retirement::controlled::ControlledRetirement::new(source).map_err(|(error, _)| error).unwrap();
    for _ in 0..law["maximumCloseTurns"].as_u64().unwrap() {
        if owner.terminal_is_empty() { return; }
        assert!(owner.step(close).unwrap().progress().fits(close));
    }
    panic!("original native output did not reach funded terminal");
}

pub fn diagnostic_capacity(error: &semio_framework_value::ValueError) -> usize { match &error.message { std::borrow::Cow::Owned(text) => text.capacity(), std::borrow::Cow::Borrowed(_) => 0 } }
