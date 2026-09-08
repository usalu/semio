
use super::*;

//#region 🔖️Source
#[semio_framework_async_macros::async_test]
async fn pack_source_over_slice_reads_at_offset() {
    let data: &[u8] = b"hello world";
    let mut buf = [0u8; 5];
    let n = data.read_at(6, &mut buf).await.unwrap();
    assert_eq!(n, 5);
    assert_eq!(&buf, b"world");
    assert_eq!(PackSource::len(&data).await, 11);
    assert!(!data.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn pack_source_over_slice_short_read_at_end_never_panics() {
    let data: &[u8] = b"hi";
    let mut buf = [0u8; 10];
    let n = data.read_at(0, &mut buf).await.unwrap();
    assert_eq!(n, 2);
    assert_eq!(&buf[..2], b"hi");
}

#[semio_framework_async_macros::async_test]
async fn pack_source_read_at_offset_past_end_errors_never_panics() {
    let data: &[u8] = b"hi";
    let mut buf = [0u8; 4];
    let result = data.read_at(100, &mut buf).await;
    assert_eq!(result, Err(PackError::Truncated(100)));
}

#[semio_framework_async_macros::async_test]
async fn pack_source_read_exact_at_errors_on_truncated_input() {
    let data: &[u8] = b"hi";
    let mut buf = [0u8; 5];
    let result = data.read_exact_at(0, &mut buf).await;
    assert!(matches!(result, Err(PackError::Truncated(_))));
}

#[semio_framework_async_macros::async_test]
async fn pack_source_over_vec_matches_slice_behavior() {
    let data: Vec<u8> = b"hello world".to_vec();
    let mut buf = [0u8; 5];
    let n = data.read_at(0, &mut buf).await.unwrap();
    assert_eq!(n, 5);
    assert_eq!(&buf, b"hello");
    assert_eq!(PackSource::len(&data).await, 11);
}

#[semio_framework_async_macros::async_test]
async fn pack_sink_over_vec_appends_and_tracks_position() {
    let mut sink: Vec<u8> = Vec::new();
    assert_eq!(sink.position().await, 0);
    sink.write_all(b"abc").await.unwrap();
    assert_eq!(sink.position().await, 3);
    sink.write_all(b"def").await.unwrap();
    assert_eq!(sink.position().await, 6);
    assert_eq!(sink.as_slice(), b"abcdef");
    assert!(sink.flush().await.is_ok());
}
