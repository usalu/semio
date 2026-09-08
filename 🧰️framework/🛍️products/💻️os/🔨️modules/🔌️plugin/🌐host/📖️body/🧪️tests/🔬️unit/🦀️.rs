use super::*;

/// ⏳️ A busy-loop `block_on` for the small, always-immediately-ready futures `BodyReader`'s
/// `Poll` variant produces — mirrors `📮️requests/🦀️.rs`'s own `futures_test_waker`
/// idiom (`Waker::noop()`), since nothing here ever actually parks.
// 🚫️async: E5 executor bridge (test-only, R4 clause 5) — a bare `fn` is the whole point: this
// IS the sync/async bridge test bodies call into, so it cannot itself be `async fn`.
fn block_on<T>(future: impl std::future::Future<Output = T>) -> T {
    use std::task::{Context, Waker};
    let waker = Waker::noop();
    let mut cx = Context::from_waker(waker);
    let mut future = Box::pin(future);
    loop {
        if let std::task::Poll::Ready(value) = future.as_mut().poll(&mut cx) {
            return value;
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn poll_backed_reader_yields_the_whole_buffer_then_ends() {
    let mut reader = BodyReader::poll_buffered(b"hello world".to_vec()).await;
    let chunk = block_on(reader.next_chunk());
    assert_eq!(chunk, Some(b"hello world".to_vec()));
    let end = block_on(reader.next_chunk());
    assert_eq!(end, None, "a second next_chunk() call must observe end-of-body, not repeat the buffer");
}

#[semio_framework_async_macros::async_test]
async fn collect_reassembles_the_full_poll_buffer() {
    let reader = BodyReader::poll_buffered(vec![7u8; 1000]).await;
    let collected = block_on(reader.collect(10_000)).expect("under cap");
    assert_eq!(collected.len(), 1000);
    assert!(collected.iter().all(|byte| *byte == 7));
}

#[semio_framework_async_macros::async_test]
async fn collect_faults_over_cap_instead_of_truncating() {
    let reader = BodyReader::poll_buffered(vec![1u8; 100]).await;
    let result = block_on(reader.collect(50));
    let fault = result.expect_err("100 bytes over a 50-byte cap must fault");
    assert_eq!(fault.code.0, "plugin.host.body-too-large");
}

#[semio_framework_async_macros::async_test]
async fn an_empty_poll_body_yields_no_chunks() {
    let mut reader = BodyReader::poll_buffered(Vec::new()).await;
    assert_eq!(block_on(reader.next_chunk()), None);
}
