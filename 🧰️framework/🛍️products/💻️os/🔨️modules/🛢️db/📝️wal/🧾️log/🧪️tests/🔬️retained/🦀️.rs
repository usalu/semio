use super::*;

#[semio_framework_async_macros::async_test]
async fn wal_bytes_exact_backing_handback_cancel_and_close_are_one_owner() {
    let _pool = db_io_driver::db_io_test_pool();
    let cancelled = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let mut control = WalCursorControl::new(cancelled.clone(), std::time::Instant::now() + std::time::Duration::from_secs(30), 64).unwrap();
    let mut source = Vec::with_capacity(32);
    source.push(0xA5);
    let mut retained = WalBytes::try_admit(source, 32, &mut control).await.unwrap();
    assert_eq!(retained.len(), 1);
    while retained.close_step().unwrap().is_some() {}
    assert!(retained.terminal_is_empty());

    let mut source = Vec::with_capacity(33);
    source.push(0x5A);
    let pointer = source.as_ptr();
    let rejected = WalBytes::try_admit(source, 32, &mut control).await.unwrap_err();
    let returned = rejected.into_source().unwrap();
    assert_eq!(returned.as_ptr(), pointer);
    assert_eq!(returned.capacity(), 33);

    cancelled.store(true, std::sync::atomic::Ordering::Release);
    let mut source = Vec::with_capacity(8);
    source.push(1);
    let pointer = source.as_ptr();
    let mut rejected = WalBytes::try_admit(source, 8, &mut control).await.unwrap_err();
    while rejected.close_step().unwrap() {}
    let returned = rejected.into_source().unwrap();
    assert_eq!(returned.as_ptr(), pointer);

    cancelled.store(false, std::sync::atomic::Ordering::Release);
    let mut deadline_control = WalCursorControl::new(cancelled, std::time::Instant::now(), 16).unwrap();
    let mut source = Vec::with_capacity(2);
    source.push(0x33);
    let pointer = source.as_ptr();
    let mut rejected = WalBytes::try_admit(source, 2, &mut deadline_control).await.unwrap_err();
    assert!(matches!(rejected.error(), DbError::Unavailable(message) if message == "wal cursor deadline reached"));
    while rejected.close_step().unwrap() {}
    assert_eq!(rejected.into_source().unwrap().as_ptr(), pointer);
}
