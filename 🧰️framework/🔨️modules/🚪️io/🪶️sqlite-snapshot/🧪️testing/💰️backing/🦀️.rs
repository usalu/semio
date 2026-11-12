//! 🔬️ Test-only request observation reads each actual backing request from the one testing allocator.
use semio_framework_value_testing_allocator as _;

#[derive(Clone, Copy, Default)]
pub struct Observation {
    pub bytes: usize,
    pub released_bytes: usize,
}
/// 📏️ Measures the operation only; nested observers add their own requests to the enclosing scope.
pub fn measure<T>(operation: impl FnOnce() -> T) -> (T, Observation) {
    let (output, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(operation);
    assert!(!heap.overflowed, "actual allocator observation overflow");
    (output, Observation { bytes: heap.requested_bytes, released_bytes: heap.released_bytes })
}

/// 🧭️ Reads the current thread's real request total without owning another buffer.
pub fn active_request_bytes() -> usize {
    semio_framework_trace::heap_allocation_observation_on_this_thread().map_or(0, |heap| heap.requested_bytes)
}

use semio_os_kernel::{
    os_store::serialization::ArtifactSqliteSnapshotHost,
    sqlite_snapshot::{SqliteDatabase, SqliteDatabaseLimits, SqliteSnapshotControl, SqliteSnapshotPhase, ValueError, ValueRefusalKind},
};
enum Owned<T: ArtifactSqliteSnapshotHost> {
    Database(SqliteDatabase),
    Snapshot(T),
}
impl<T: ArtifactSqliteSnapshotHost> Owned<T> {
    fn verify(self, snapshot: &T, database: &SqliteDatabase, verify: &impl Fn(&T, &T)) {
        match self {
            Self::Database(value) => assert_eq!(&value, database),
            Self::Snapshot(value) => {
                verify(&value, snapshot);
                value.retire_sqlite_snapshot();
            }
        }
    }
}
fn construct<T: ArtifactSqliteSnapshotHost>(snapshot: &T, database: &SqliteDatabase, phase: SqliteSnapshotPhase, control: &mut SqliteSnapshotControl<'_>) -> Result<Owned<T>, ValueError> {
    match phase {
        SqliteSnapshotPhase::ProjectSnapshot => snapshot.to_sqlite_database(control).map(Owned::Database),
        SqliteSnapshotPhase::ReconstructSnapshot => T::from_sqlite_database(database, control).map(Owned::Snapshot),
        _ => unreachable!(),
    }
}
/// 💰️ Direct typed SQL producers settle the concrete allocator requests and retain cumulative admission.
pub fn verify_snapshot_backing<T: ArtifactSqliteSnapshotHost + PartialEq + std::fmt::Debug>(snapshot: &T, database: &SqliteDatabase) {
    verify_snapshot_backing_by(snapshot, database, |actual, expected| assert_eq!(actual, expected));
}
/// 📸️ Observes unchanged physical requests with explicit domain literal equality outside observation.
pub fn verify_snapshot_backing_by<T: ArtifactSqliteSnapshotHost>(snapshot: &T, database: &SqliteDatabase, verify: impl Fn(&T, &T)) {
    let defaults = SqliteDatabaseLimits::default();
    for phase in [SqliteSnapshotPhase::ProjectSnapshot, SqliteSnapshotPhase::ReconstructSnapshot] {
        let mut callback = |_| true;
        let mut control = SqliteSnapshotControl::new(&mut callback, defaults);
        let (result, observed) = measure(|| construct(snapshot, database, phase, &mut control));
        let output = result.expect("actual complete typed SQL producer");
        assert!(observed.bytes > 0, "{phase:?} must own concrete backing");
        let admitted = defaults.max_allocation_bytes - control.allocation_remaining_bytes();
        output.verify(snapshot, database, &verify);
        assert_eq!(admitted, observed.bytes, "{phase:?} must settle full concrete allocator requests");
        let exact = SqliteDatabaseLimits { max_allocation_bytes: observed.bytes, ..defaults };
        let mut callback = |_| true;
        let mut control = SqliteSnapshotControl::new(&mut callback, exact);
        let (result, replayed) = measure(|| construct(snapshot, database, phase, &mut control));
        result.expect("exact observed ownership allowance").verify(snapshot, database, &verify);
        assert_eq!(replayed.bytes, observed.bytes, "{phase:?} exact replay preserves physical requests");
        assert_eq!(control.allocation_remaining_bytes(), 0, "{phase:?} exact ownership ledger");
        for maximum in [0, observed.bytes - 1] {
            let mut callback = |_| true;
            let mut control = SqliteSnapshotControl::new(&mut callback, SqliteDatabaseLimits { max_allocation_bytes: maximum, ..defaults });
            match construct(snapshot, database, phase, &mut control) {
                Err(error) => assert_eq!(error.kind, ValueRefusalKind::OwnershipLimit, "{phase:?} physical caller ceiling"),
                Ok(value) => {
                    value.verify(snapshot, database, &verify);
                    panic!("{phase:?} must refuse below concrete ownership");
                }
            }
        }
        let maximum = observed.bytes.checked_mul(2).expect("two observed ownership extents");
        let mut callback = |_| true;
        let mut control = SqliteSnapshotControl::new(&mut callback, SqliteDatabaseLimits { max_allocation_bytes: maximum, ..defaults });
        for index in 1..=2 {
            let (result, replayed) = measure(|| construct(snapshot, database, phase, &mut control));
            result.expect("cumulative concrete ownership allowance").verify(snapshot, database, &verify);
            assert_eq!(replayed.bytes, observed.bytes, "{phase:?} cumulative physical requests");
            assert_eq!(control.allocation_remaining_bytes(), maximum - index * observed.bytes, "{phase:?} retirement cannot refund admission");
        }
        match construct(snapshot, database, phase, &mut control) {
            Err(error) => assert_eq!(error.kind, ValueRefusalKind::OwnershipLimit, "{phase:?} exhausted cumulative ownership"),
            Ok(value) => {
                value.verify(snapshot, database, &verify);
                panic!("{phase:?} must retain cumulative retirement admission");
            }
        }
    }
}

/// 🧯️ Observes the complete direct producer's retained failure and cancellation owners.
pub fn verify_snapshot_failure_backing<T: ArtifactSqliteSnapshotHost>(snapshot: &T, database: &SqliteDatabase, cancel_at: usize) {
    let defaults = SqliteDatabaseLimits::default();
    for phase in [SqliteSnapshotPhase::ProjectSnapshot, SqliteSnapshotPhase::ReconstructSnapshot] {
        let mut callback = |_| true;
        let mut control = SqliteSnapshotControl::new(&mut callback, defaults);
        let ((), success) = measure(|| match construct(snapshot, database, phase, &mut control).expect("complete owned producer") {
            Owned::Database(value) => drop(value),
            Owned::Snapshot(value) => value.retire_sqlite_snapshot(),
        });
        assert_eq!(success.bytes, success.released_bytes, "{phase:?} releases every successful backing request");
        let exact = defaults.max_allocation_bytes - control.allocation_remaining_bytes();
        assert!(exact > 0, "{phase:?} positive observed ownership allowance");
        for maximum in [0, exact - 1] {
            let mut callback = |_| true;
            let mut control = SqliteSnapshotControl::new(&mut callback, SqliteDatabaseLimits { max_allocation_bytes: maximum, ..defaults });
            let ((kind, diagnostic), observed) = measure(|| {
                let error = match construct(snapshot, database, phase, &mut control) {
                    Err(error) => error,
                    Ok(value) => { drop(value); panic!("physical ceiling must refuse"); }
                };
                let result = (error.kind, match &error.message { std::borrow::Cow::Owned(message) => message.capacity(), std::borrow::Cow::Borrowed(_) => 0 });
                drop(error);
                result
            });
            let admitted = maximum - control.allocation_remaining_bytes();
            assert_eq!(kind, ValueRefusalKind::OwnershipLimit, "{phase:?} retained physical refusal");
            assert_eq!(observed.bytes, observed.released_bytes, "{phase:?} releases refused owner and diagnostic");
            assert!(observed.bytes <= admitted.checked_add(diagnostic).expect("scratch and diagnostic extent"), "{phase:?} full requests include the actual retained diagnostic capacity");
            if maximum == 0 { assert_eq!(admitted, 0); assert_eq!(observed.bytes, diagnostic); }
        }
        for interior in [false, true] {
            let mut reached = false;
            let mut callback = |event: semio_os_kernel::sqlite_snapshot::SqliteSnapshotProgress| {
                if !interior || (event.phase == phase && event.completed >= cancel_at && event.completed < event.total && active_request_bytes() > 0) {
                    reached = true;
                    false
                } else { true }
            };
            let mut control = SqliteSnapshotControl::new(&mut callback, defaults);
            let ((kind, diagnostic), observed) = measure(|| {
                let error = match construct(snapshot, database, phase, &mut control) {
                    Err(error) => error,
                    Ok(value) => { drop(value); panic!("real producer cancellation must refuse"); }
                };
                let result = (error.kind, match &error.message { std::borrow::Cow::Owned(message) => message.capacity(), std::borrow::Cow::Borrowed(_) => 0 });
                drop(error);
                result
            });
            let admitted = defaults.max_allocation_bytes - control.allocation_remaining_bytes();
            drop(control);
            assert!(reached, "{phase:?} reaches the authored cancellation boundary");
            assert_eq!(kind, ValueRefusalKind::Canceled);
            assert_eq!(observed.bytes, observed.released_bytes, "{phase:?} releases canceled owner and diagnostic");
            assert!(observed.bytes <= admitted.checked_add(diagnostic).expect("scratch and diagnostic extent"), "{phase:?} complete canceled requests retain monotonic admission");
            if interior { assert!(observed.bytes > diagnostic, "{phase:?} cancellation follows actual backing materialization"); }
            else { assert_eq!(admitted, 0); assert_eq!(observed.bytes, diagnostic); }
        }
    }
}
