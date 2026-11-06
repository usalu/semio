//! 🏠️ `HeadlessWorkspace` — packet `P7-headless-workspace`: the real `GatewayBackend` that lets the [Design notes](README.md#source-notes-1).

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
