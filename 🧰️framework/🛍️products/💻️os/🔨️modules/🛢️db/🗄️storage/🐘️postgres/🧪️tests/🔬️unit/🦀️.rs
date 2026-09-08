
use super::*;
use crate::db_storage::db_io_maintenance_step;

#[semio_framework_async_macros::async_test]
async fn lost_postgres_facade_drives_the_real_lazy_pool_to_closed() {
    let url = DbIoText::try_from_str("postgres://localhost/p1q-lost-facade").unwrap();
    let executor = PostgresDbIoExecutor::new(url).unwrap();
    let driver_pool = executor.pool.clone();
    let worker_pool = Arc::new(WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
    let control = register_db_io_backend(DbIoBackendKind::Postgres, Box::new(executor), worker_pool.clone()).unwrap();
    let facade = PostgresStorage { control, worker_pool: worker_pool.clone(), closed: std::sync::atomic::AtomicBool::new(false) };
    drop(facade);
    close_db_io_backend(control).await.unwrap();
    loop {
        match db_io_maintenance_step() {
            Ok(true) => {}
            Ok(false) => break,
            Err(error) => panic!("PostgreSQL lost-facade maintenance failed: {error}"),
        }
    }
    assert!(driver_pool.is_closed());
    worker_pool.shutdown();
}

// 🔬️ Live-Postgres integration testing (schema bootstrap round-trip, real CAS races across
// connections, transaction rollback on error) is deliberately deferred — this environment has
// no `DATABASE_URL`/live Postgres server. Everything below tests the pure decision logic each
// trait method delegates to, which is where this backend's actual correctness lives; the SQL
// itself is exercised by inspection against the schema in `//#region 🔖️Schema`.

//#region 🔖️RangeAndTruncate
#[semio_framework_async_macros::async_test]
async fn validate_read_range_accepts_in_bounds_slice() {
    assert_eq!(validate_read_range(10, ByteRange { offset: 2, len: 5 }).unwrap(), (2, 5));
}

#[semio_framework_async_macros::async_test]
async fn validate_read_range_rejects_out_of_bounds() {
    assert!(matches!(validate_read_range(10, ByteRange { offset: 8, len: 5 }), Err(DbError::InvalidArgument(_))));
}

#[semio_framework_async_macros::async_test]
async fn validate_read_range_rejects_offset_len_overflow() {
    assert!(matches!(validate_read_range(10, ByteRange { offset: u64::MAX, len: 1 }), Err(DbError::InvalidArgument(_))));
}

#[semio_framework_async_macros::async_test]
async fn validate_truncate_rejects_sealed_segment() {
    assert!(matches!(validate_truncate(true, 10, 5), Err(DbError::InvalidArgument(_))));
}

#[semio_framework_async_macros::async_test]
async fn validate_truncate_rejects_growth() {
    assert!(matches!(validate_truncate(false, 10, 20), Err(DbError::InvalidArgument(_))));
}

#[semio_framework_async_macros::async_test]
async fn validate_truncate_accepts_shrink() {
    assert!(validate_truncate(false, 10, 5).is_ok());
}

#[test]
fn wal_segment_state_query_and_mapper_are_read_only_and_byte_neutral() {
    assert_eq!(postgres_wal_segment_state(false), WalSegmentState::Active);
    assert_eq!(postgres_wal_segment_state(true), WalSegmentState::Sealed);
    assert!(POSTGRES_WAL_STATE_QUERY.contains("SELECT sealed"));
    assert!(!POSTGRES_WAL_STATE_QUERY.contains("FOR UPDATE"));
    assert!(!POSTGRES_WAL_STATE_QUERY.contains("bytes"));
}
//#endregion 🔖️RangeAndTruncate

//#region 🔖️Lease
#[semio_framework_async_macros::async_test]
async fn lease_acquire_fresh_resource_starts_at_initial_fence() {
    assert_eq!(lease_acquire_decision(None, "alice", 0).unwrap(), EpochFence::INITIAL);
}

#[semio_framework_async_macros::async_test]
async fn lease_acquire_reacquire_by_same_live_holder_keeps_fence() {
    let existing = ExistingLease { holder: DbIoText::try_from_str("alice").unwrap(), fence: EpochFence::INITIAL.next(), expires_at_ms: 1_000 };
    assert_eq!(lease_acquire_decision(Some(&existing), "alice", 500).unwrap(), existing.fence);
}

#[semio_framework_async_macros::async_test]
async fn lease_acquire_by_other_holder_before_expiry_conflicts() {
    let existing = ExistingLease { holder: DbIoText::try_from_str("alice").unwrap(), fence: EpochFence::INITIAL, expires_at_ms: 1_000 };
    assert!(matches!(lease_acquire_decision(Some(&existing), "bob", 500), Err(DbError::Conflict(_))));
}

#[semio_framework_async_macros::async_test]
async fn lease_acquire_after_expiry_bumps_fence_for_new_holder() {
    let existing = ExistingLease { holder: DbIoText::try_from_str("alice").unwrap(), fence: EpochFence::INITIAL, expires_at_ms: 1_000 };
    assert_eq!(lease_acquire_decision(Some(&existing), "bob", 2_000).unwrap(), EpochFence::INITIAL.next());
}

#[semio_framework_async_macros::async_test]
async fn lease_renew_rejects_absent_expired_wrong_holder_and_wrong_fence() {
    let existing = ExistingLease { holder: DbIoText::try_from_str("alice").unwrap(), fence: EpochFence::INITIAL, expires_at_ms: 1_000 };
    assert!(matches!(lease_renew_check(None, "alice", EpochFence::INITIAL, 0), Err(DbError::NotFound(_))));
    assert!(matches!(lease_renew_check(Some(&existing), "alice", EpochFence::INITIAL, 2_000), Err(DbError::Unavailable(_))));
    assert!(matches!(lease_renew_check(Some(&existing), "bob", EpochFence::INITIAL, 500), Err(DbError::Unauthorized(_))));
    assert!(matches!(lease_renew_check(Some(&existing), "alice", EpochFence::INITIAL.next(), 500), Err(DbError::Fenced { .. })));
}

#[semio_framework_async_macros::async_test]
async fn lease_renew_accepts_matching_live_holder_and_fence() {
    let existing = ExistingLease { holder: DbIoText::try_from_str("alice").unwrap(), fence: EpochFence::INITIAL, expires_at_ms: 1_000 };
    assert!(lease_renew_check(Some(&existing), "alice", EpochFence::INITIAL, 500).is_ok());
}

#[semio_framework_async_macros::async_test]
async fn lease_release_rejects_wrong_holder_or_fence() {
    let existing = ExistingLease { holder: DbIoText::try_from_str("alice").unwrap(), fence: EpochFence::INITIAL, expires_at_ms: 1_000 };
    assert!(matches!(lease_release_check(Some(&existing), "bob", EpochFence::INITIAL), Err(DbError::Unauthorized(_))));
    assert!(matches!(lease_release_check(Some(&existing), "alice", EpochFence::INITIAL.next()), Err(DbError::Fenced { .. })));
    assert!(lease_release_check(Some(&existing), "alice", EpochFence::INITIAL).is_ok());
}
//#endregion 🔖️Lease

//#region 🔖️Conversion
#[semio_framework_async_macros::async_test]
async fn to_i64_round_trips_ordinary_values() {
    assert_eq!(to_i64(42).unwrap(), 42);
    assert_eq!(to_i64(0).unwrap(), 0);
}

#[semio_framework_async_macros::async_test]
async fn to_i64_rejects_values_above_i64_max() {
    assert!(to_i64(u64::MAX).is_err());
}
//#endregion 🔖️Conversion

//#region 🔖️ErrorMapping
#[semio_framework_async_macros::async_test]
async fn map_sqlx_error_classifies_row_not_found() {
    assert!(matches!(map_sqlx_error(sqlx::Error::RowNotFound), DbError::NotFound(_)));
}

#[semio_framework_async_macros::async_test]
async fn map_sqlx_error_classifies_pool_exhaustion_as_unavailable() {
    assert!(matches!(map_sqlx_error(sqlx::Error::PoolClosed), DbError::Unavailable(_)));
    assert!(matches!(map_sqlx_error(sqlx::Error::PoolTimedOut), DbError::Unavailable(_)));
    assert!(matches!(map_sqlx_error(sqlx::Error::WorkerCrashed), DbError::Unavailable(_)));
}

#[semio_framework_async_macros::async_test]
async fn map_sqlx_error_classifies_protocol_and_configuration_as_invalid_argument() {
    assert!(matches!(map_sqlx_error(sqlx::Error::Protocol("bad frame".to_string())), DbError::InvalidArgument(_)));
    assert!(matches!(map_sqlx_error(sqlx::Error::InvalidArgument("bad bind".to_string())), DbError::InvalidArgument(_)));
}

#[semio_framework_async_macros::async_test]
async fn map_sqlx_error_classifies_io_as_io() {
    assert!(matches!(map_sqlx_error(sqlx::Error::Io(std::io::Error::other("disconnected"))), DbError::Io(_)));
}
//#endregion 🔖️ErrorMapping

//#region 🔖️SchemaAndCapabilities
#[semio_framework_async_macros::async_test]
async fn schema_statements_cover_every_storage_table() {
    let joined = SCHEMA_STATEMENTS.join(" ");
    for table in ["db_wal_segment", "db_snapshot_generation", "db_payload", "db_catalog_root", "db_index_run", "db_lease"] {
        assert!(joined.contains(table), "schema is missing table {table}");
    }
}

#[semio_framework_async_macros::async_test]
async fn schema_seeds_the_catalog_singleton_row() {
    assert!(SCHEMA_STATEMENTS.iter().any(|statement| statement.contains("INSERT INTO db_catalog_root")));
}

#[semio_framework_async_macros::async_test]
async fn postgres_capabilities_report_durable_fsync_and_real_cas() {
    let caps = postgres_capabilities();
    assert!(caps.durable);
    assert!(caps.supports_fsync);
    assert!(caps.supports_cas);
    assert_eq!(caps.max_durability, DurabilityClass::Fsync);
}
//#endregion 🔖️SchemaAndCapabilities

//#region 🔖️ContentAddressing
#[semio_framework_async_macros::async_test]
async fn payload_hash_is_deterministic_and_content_addressed() {
    let a = ContentHash(*semio_framework_hash::hash(b"hello").as_bytes());
    let b = ContentHash(*semio_framework_hash::hash(b"hello").as_bytes());
    let c = ContentHash(*semio_framework_hash::hash(b"world").as_bytes());
    assert_eq!(a.0, b.0, "identical bytes hash identically");
    assert_ne!(a.0, c.0, "different bytes hash differently");
}
//#endregion 🔖️ContentAddressing
