//! 📖️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (sdk-async): a streaming body abstraction wired into
//! `Host::http_fetch`/`Host::blob_read`. The poll world has no live channel — a poll-world
//! completion is one shot, and `🌐host/📮️requests::RequestRegistry::append_chunk`'s chunk-
//! reassembly fix already buffers the WHOLE body before `Host` ever sees it — so `BodyReader::Poll`
//! just replays that already-complete buffer in caller-sized slices.

use semio_framework::{Fault, FaultCode, FaultOrigin};

/// 📖️ See module doc. Never constructed directly outside `🌐host/🦀️.rs` — use
/// `BodyReader::poll_buffered`.
pub enum BodyReader {
    Poll {
        bytes: Vec<u8>,
        consumed: usize,
    },
}

impl BodyReader {
    /// 🌊️ Wraps an already-fully-buffered body as a single-"chunk" reader — the poll world's shape
    /// for BOTH `blob_read` (no chunked blob backend exists anywhere in this codebase — see
    /// `Host::blob_read`'s own doc) and `http_fetch` (the reactor's `append_chunk` reassembly hands
    /// `Host::http_fetch`'s Poll arm one complete buffer, never a live channel).
    pub(crate) async fn poll_buffered(bytes: Vec<u8>) -> Self {
        BodyReader::Poll { bytes, consumed: 0 }
    }

    /// ▶️ The next chunk, `None` once the body is exhausted: the whole remaining buffer in one shot
    /// (there is nothing left to actually await — see module doc).
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
