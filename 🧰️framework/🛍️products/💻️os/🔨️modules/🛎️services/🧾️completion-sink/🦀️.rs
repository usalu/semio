//! 🧾️ The completion sink: the only re-entry point from the services into the kernel, and its recording mock.

use std::sync::{Arc, Mutex};

//#region 🧾️CompletionSink
/// 🧾 The ONLY way any service in this crate re-enters the kernel: hand a completed operation's
/// result back as raw bytes tagged with the actor/generation/lane it belongs to. No type in this
/// crate holds or calls a `Kernel` directly — every completion flows out through this trait instead,
/// the same seam discipline `HostAsyncRuntime` itself uses to hide tokio.
// 🚫️async: E6 dyn-compat — `Arc<dyn CompletionSink>` is the load-bearing shape (`spawn_driver`'s
// `sink` parameter, an OPEN extension point implemented outside this crate). `complete` does no
// awaiting of its own (raw-bytes handoff, no I/O in this crate), so unlike `AsyncHttpTransport` it
// needs no `HostFuture` erasure either — plain sync is both correct and dyn-compatible. See the
// `HttpTransport` tag above for the coordinator note on this exemption class.
pub trait CompletionSink: Send + Sync {
    fn complete(&self, actor: u64, generation: u16, event_bytes: Vec<u8>, lane: u8);
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CompletionRecord {
    pub actor: u64,
    pub generation: u16,
    pub event_bytes: Vec<u8>,
    pub lane: u8,
}

/// 🧪️ Test double recording every [`CompletionSink::complete`] call in order, for asserting a
/// service actually reached the ONLY re-entry point rather than swallowing its result.
#[derive(Clone, Default)]
pub struct MockCompletionSink {
    completions: Arc<Mutex<Vec<CompletionRecord>>>,
}

impl MockCompletionSink {
    pub async fn new() -> MockCompletionSink {
        MockCompletionSink::default()
    }

    pub async fn recorded(&self) -> Vec<CompletionRecord> {
        self.completions.lock().expect("MockCompletionSink mutex poisoned").clone()
    }
}

impl CompletionSink for MockCompletionSink {
    // 🚫️async: E6 dyn-compat — see the trait declaration's tag.
    fn complete(&self, actor: u64, generation: u16, event_bytes: Vec<u8>, lane: u8) {
        self.completions.lock().expect("MockCompletionSink mutex poisoned").push(CompletionRecord { actor, generation, event_bytes, lane });
    }
}
//#endregion 🧾️CompletionSink
