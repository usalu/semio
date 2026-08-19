//! 📖️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (sdk-async): a streaming body abstraction wired into
//! `Host::http_fetch`/`Host::blob_read`. The poll world has no live channel — a poll-world
//! completion is one shot, and `⚛️reactor/📮️requests::RequestRegistry::append_chunk`'s chunk-
//! reassembly fix already buffers the WHOLE body before `Host` ever sees it — so `BodyReader::Poll`
//! just replays that already-complete buffer in caller-sized slices. `BodyReader::Direct` wraps a
//! REAL `wit_bindgen::StreamReader<u8>`, so a `component-guest-async` guest on a genuine
//! wasm32-wasip2 build can start consuming a response before the host finishes sending it — see
//! `🌐host/🦀️component.rs`'s `direct` module doc for why the concrete stream only exists there.

use semio_framework::{Fault, FaultCode, FaultOrigin};

/// 📖️ How many bytes one `Direct`-backend `read()` call asks the host for at a time. Not the
/// safety cap (`cap` in `next_chunk`'s caller / `collect` is that); a throughput knob only, picked
/// to keep a `StreamReader<u8>::read` call's per-read allocation modest without forcing a host
/// round-trip per BYTE the way the crate's own built-in `.next()` convenience method would (see
/// `wit-bindgen-0.57.1`'s `RawStreamReader::next`, which always reads with capacity 1).
const DIRECT_READ_CHUNK: usize = 64 * 1024;

/// 📖️ See module doc. Never constructed directly outside `🌐host/🦀️component.rs` — use
/// `BodyReader::poll_buffered`/`BodyReader::direct`.
pub enum BodyReader {
    Poll { bytes: Vec<u8>, consumed: usize },
    #[cfg(all(feature = "component-guest-async", target_arch = "wasm32", target_env = "p2"))]
    Direct(wit_bindgen::StreamReader<u8>),
}

impl BodyReader {
    /// 🌊️ Wraps an already-fully-buffered body as a single-"chunk" reader — the poll world's shape
    /// for BOTH `blob_read` (no chunked blob backend exists anywhere in this codebase — see
    /// `Host::blob_read`'s own doc) and `http_fetch` (the reactor's `append_chunk` reassembly hands
    /// `Host::http_fetch`'s Poll arm one complete buffer, never a live channel).
    pub(crate) async fn poll_buffered(bytes: Vec<u8>) -> Self {
        BodyReader::Poll { bytes, consumed: 0 }
    }

    /// 🌊️ Wraps a real host-async `stream<u8>` — see `Host::http_fetch`/`Host::blob_read`'s
    /// `Direct` arms, the only callers.
    #[cfg(all(feature = "component-guest-async", target_arch = "wasm32", target_env = "p2"))]
    pub(crate) async fn direct(stream: wit_bindgen::StreamReader<u8>) -> Self {
        BodyReader::Direct(stream)
    }

    /// ▶️ The next chunk, `None` once the body is exhausted. Poll world: the whole remaining buffer
    /// in one shot (there is nothing left to actually await — see module doc). Direct world: one
    /// real `StreamReader::read`, up to `DIRECT_READ_CHUNK` bytes.
    pub async fn next_chunk(&mut self) -> Option<Vec<u8>> {
        match self {
            BodyReader::Poll { bytes, consumed } => {
                if *consumed >= bytes.len() {
                    return None;
                }
                let chunk = bytes[*consumed..].to_vec();
                *consumed = bytes.len();
                Some(chunk)
            }
            #[cfg(all(feature = "component-guest-async", target_arch = "wasm32", target_env = "p2"))]
            BodyReader::Direct(stream) => {
                let (status, buf) = stream.read(Vec::with_capacity(DIRECT_READ_CHUNK)).await;
                if buf.is_empty() && matches!(status, wit_bindgen::StreamResult::Dropped) {
                    None
                } else {
                    Some(buf)
                }
            }
        }
    }

    /// 📦️ Drains every remaining chunk into one buffer, faulting (never silently truncating) once
    /// the running total exceeds `cap` — the SAME "typed fault over cap, not truncate" contract
    /// `RequestRegistry::append_chunk` enforces for the poll world's own chunk-reassembly, so a
    /// caller gets the identical failure shape regardless of which backend is live underneath.
    pub async fn collect(mut self, cap: usize) -> Result<Vec<u8>, Fault> {
        let mut out = Vec::new();
        while let Some(chunk) = self.next_chunk().await {
            if out.len() + chunk.len() > cap {
                return Err(Fault::new(FaultOrigin::Plugin, FaultCode::new("plugin.host.body-too-large"), format!("body exceeded the {cap}-byte cap while collecting")));
            }
            out.extend_from_slice(&chunk);
        }
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// ⏳️ A busy-loop `block_on` for the small, always-immediately-ready futures `BodyReader`'s
    /// `Poll` variant produces — mirrors `📮️requests/🦀️component.rs`'s own `futures_test_waker`
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
        let mut reader = BodyReader::poll_buffered(b"hello world".to_vec());
        let chunk = block_on(reader.next_chunk());
        assert_eq!(chunk, Some(b"hello world".to_vec()));
        let end = block_on(reader.next_chunk());
        assert_eq!(end, None, "a second next_chunk() call must observe end-of-body, not repeat the buffer");
    }

    #[semio_framework_async_macros::async_test]
    async fn collect_reassembles_the_full_poll_buffer() {
        let reader = BodyReader::poll_buffered(vec![7u8; 1000]);
        let collected = block_on(reader.collect(10_000)).expect("under cap");
        assert_eq!(collected.len(), 1000);
        assert!(collected.iter().all(|byte| *byte == 7));
    }

    #[semio_framework_async_macros::async_test]
    async fn collect_faults_over_cap_instead_of_truncating() {
        let reader = BodyReader::poll_buffered(vec![1u8; 100]);
        let result = block_on(reader.collect(50));
        let fault = result.expect_err("100 bytes over a 50-byte cap must fault");
        assert_eq!(fault.code.0, "plugin.host.body-too-large");
    }

    #[semio_framework_async_macros::async_test]
    async fn an_empty_poll_body_yields_no_chunks() {
        let mut reader = BodyReader::poll_buffered(Vec::new());
        assert_eq!(block_on(reader.next_chunk()), None);
    }
}
