//! 🌐️ The owned port to the process's document-socket I/O reactor: the one place a caller outside this crate meets the reactor, with no Tokio type in any signature. See [`super`].

use super::actor_runner_scheduling_terminal_and_spawn::document_socket_io_reactor;
use semio_framework_os_store_sync::ArtifactEvent;
use std::future::Future;
use tokio::sync::broadcast;

//#region 🔌️DocumentSocketReactor
/// 🔌️ The process-wide I/O reactor every native document socket is driven by: one dedicated thread that parks on the platform I/O and time drivers, created on first use and never shut down.
///
/// 🧭️ A caller that dials or awaits a hub document socket from a thread no runtime owns enters it with [`DocumentSocketReactor::enter`] for as long as it polls socket futures, or hands it a task with [`DocumentSocketReactor::spawn`]; nothing outside this module names the runtime behind it.
pub struct DocumentSocketReactor {
    handle: tokio::runtime::Handle,
}

/// 🧯️ The operating system refused to start the document-socket reactor thread.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DocumentSocketReactorError;

impl std::fmt::Display for DocumentSocketReactorError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("the operating system refused to start the document-socket reactor thread")
    }
}

impl std::error::Error for DocumentSocketReactorError {}

/// 🚪️ Proof that the current thread is inside the reactor; leaving the scope leaves the reactor.
#[must_use = "the reactor is entered only while this scope is alive"]
pub struct DocumentSocketReactorScope<'reactor> {
    _entered: tokio::runtime::EnterGuard<'reactor>,
}

impl DocumentSocketReactor {
    /// 🏛️ The one reactor of this process, started on first use.
    pub fn shared() -> Result<&'static Self, DocumentSocketReactorError> {
        static SHARED: std::sync::OnceLock<Option<DocumentSocketReactor>> = std::sync::OnceLock::new();
        SHARED.get_or_init(|| document_socket_io_reactor().map(|handle| Self { handle })).as_ref().ok_or(DocumentSocketReactorError)
    }

    /// 🎟️ Makes the calling thread part of the reactor until the returned scope is dropped.
    pub fn enter(&self) -> DocumentSocketReactorScope<'_> {
        DocumentSocketReactorScope { _entered: self.handle.enter() }
    }

    /// 🚀️ Runs `task` to completion on the reactor thread, detached: nobody awaits its result.
    pub fn spawn(&self, task: impl Future<Output = ()> + Send + 'static) {
        drop(self.handle.spawn(task));
    }
}
//#endregion 🔌️DocumentSocketReactor

//#region 📡️EventPump
/// 📡️ Hands every event of one document to `on_event` in order until the document closes; a receiver that fell behind skips the lost events and keeps going.
pub(super) async fn pump_events(mut events: broadcast::Receiver<ArtifactEvent>, mut on_event: impl FnMut(ArtifactEvent)) {
    loop {
        match events.recv().await {
            Ok(event) => on_event(event),
            Err(broadcast::error::RecvError::Lagged(_)) => {}
            Err(broadcast::error::RecvError::Closed) => break,
        }
    }
}
//#endregion 📡️EventPump
