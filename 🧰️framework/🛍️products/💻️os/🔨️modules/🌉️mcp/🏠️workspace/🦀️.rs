//! 🏠️ `HeadlessWorkspace` — packet `P7-headless-workspace`: the real `GatewayBackend` that lets the
//! gateway open, read and (best-effort) drive real artifacts with no UI attached. Follows
//! `🏃️run/🦀️.rs`'s headless-driving precedent (`📓️luna-channel-audit.md` §8) rather than
//! inventing a second way to open a space: `--folder` binds `store::sync::ArtifactHost` to
//! `PersistenceBinding::Folder{watch_external: true}` (so a native shell on the same folder sees
//! agent edits and vice versa — `📋️master.md` §2.1); `--hub` binds `PersistenceBinding::Hub`.
//!
//! 🚧️ **Known, documented gap**, same class as `🏃️run/🦀️.rs`'s own `WasmtimeNodeHost` (see
//! that struct's doc comment): a plugin's real document schema (e.g. note's `"note.document"`) is
//! host-opaque by design — only the wasm guest understands it. This crate never decodes plugin
//! document content structurally; it passes pack+spr bytes through unchanged (`read_resource`) and,
//! for the one place it DOES need a concrete `Snapshot`/`Mutation` pair to prove the real VCS +
//! backbone propagation pipeline end to end (§`🔖️ProbeDocument` below), it uses a generic
//! JSON-valued document of its own — never a note-specific type this crate has no business knowing.

use std::sync::Arc;

pub(crate) fn workspace_worker_pool() -> Arc<semio_framework_async::WorkerPool> {
    let cores = std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get);
    Arc::new(semio_framework_async::process_worker_pool(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::InteractiveNative, cores)))
}

//#region 🔖️ArtifactChannel
//#endregion 🔖️ArtifactChannel

//#region 🔖️HeadlessWorkspace
//#endregion 🔖️HeadlessWorkspace

//#region 🧪️Tests
//#endregion 🧪️Tests
