
use super::*;
use db_storage::MemoryStorage;

#[semio_framework_async_macros::async_test]
async fn snapshot_cursor_cancel_fuel_interrupted_close_and_terminal_empty_are_exact() {
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let manager = SnapshotManager::new(&storage).await;
    let document = ArtifactId::from("retained-snapshot");
    let body = SnapshotBody { head_seq: 0, commit_seq: 0, epoch: 0, chain_hash: [0; 32], protocol_version: 1, vcs_head: None, base_pack_hash: None, roots: Vec::new(), created_at_ms: 0 };
    manager.publish(&document, SnapshotOrigin::FullBaseline, &[], body).await.unwrap();

    let cancelled = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let control = SnapshotCursorControl::new(cancelled.clone(), std::time::Instant::now() + std::time::Duration::from_secs(30), 1).unwrap();
    let mut cursor = manager.chain_cursor(&document, 0, control);
    assert!(cursor.latest_descriptor().await.is_err());
    cursor.control.replenish(64, std::time::Instant::now() + std::time::Duration::from_secs(30)).unwrap();
    cancelled.store(true, std::sync::atomic::Ordering::Release);
    assert!(matches!(cursor.latest_descriptor().await, Err(DbError::Unavailable(_))));
    cancelled.store(false, std::sync::atomic::Ordering::Release);
    assert!(cursor.close_step().unwrap());
    assert!(cursor.terminal_is_empty());
    assert!(!cursor.close_step().unwrap());
}
