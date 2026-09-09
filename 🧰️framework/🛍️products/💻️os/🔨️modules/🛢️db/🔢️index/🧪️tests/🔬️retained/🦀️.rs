use super::*;

#[semio_framework_async_macros::async_test]
async fn exact_backing_handback_cancel_close_and_fragment_order_are_deterministic() {
    let _pool = db_storage::db_io_test_pool();
    let cancelled = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let mut control = IndexCursorControl::new(cancelled.clone(), std::time::Instant::now() + std::time::Duration::from_secs(30), 128).unwrap();
    let mut one = Vec::with_capacity(db_storage::DB_IO_PAGE_BYTES + 1);
    one.push(b'a');
    let mut retained = IndexBytes::try_admit(one, (db_storage::DB_IO_PAGE_BYTES + 1) as u64, &mut control).await.unwrap();
    let mut second_source = Vec::with_capacity(db_storage::DB_IO_PAGE_BYTES + 1);
    second_source.push(b'b');
    let mut second = IndexBytes::try_admit(second_source, (db_storage::DB_IO_PAGE_BYTES + 1) as u64, &mut control).await.unwrap();
    assert_eq!(index_bytes_cmp(&retained, &second), std::cmp::Ordering::Less);
    while retained.close_step().unwrap().is_some() {}
    while second.close_step().unwrap().is_some() {}
    assert!(retained.terminal_is_empty());
    assert!(second.terminal_is_empty());

    let mut source = Vec::with_capacity(65);
    source.push(1);
    let pointer = source.as_ptr();
    let rejected = IndexBytes::try_admit(source, 64, &mut control).await.unwrap_err();
    let returned = rejected.into_source().unwrap();
    assert_eq!(returned.as_ptr(), pointer);
    assert_eq!(returned.capacity(), 65);

    cancelled.store(true, std::sync::atomic::Ordering::Release);
    let mut source = Vec::with_capacity(8);
    source.push(1);
    let pointer = source.as_ptr();
    let mut rejected = IndexBytes::try_admit(source, 8, &mut control).await.unwrap_err();
    while rejected.close_step().unwrap() {}
    assert_eq!(rejected.into_source().unwrap().as_ptr(), pointer);

    cancelled.store(false, std::sync::atomic::Ordering::Release);
    let mut deadline_control = IndexCursorControl::new(cancelled, std::time::Instant::now(), 16).unwrap();
    let mut source = Vec::with_capacity(2);
    source.push(0x33);
    let pointer = source.as_ptr();
    let mut rejected = IndexBytes::try_admit(source, 2, &mut deadline_control).await.unwrap_err();
    assert!(matches!(rejected.error(), DbError::Unavailable(message) if message == "index cursor deadline reached"));
    while rejected.close_step().unwrap() {}
    assert_eq!(rejected.into_source().unwrap().as_ptr(), pointer);
}
