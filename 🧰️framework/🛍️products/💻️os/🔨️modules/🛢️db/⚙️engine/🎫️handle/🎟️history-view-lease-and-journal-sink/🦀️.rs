//! 🎟️ History view, lease and journal sink: the history view, the handle lease, the compaction future and the journal sink. See [`super`].

use super::*;

//#region 🔖️History
pub struct HistoryView {
    pub inner: Option<db_artifact_history::ArtifactHistoryView>,
    pub admission: Option<ArtifactHistoryAdmission>,
    pub terminal_state: Option<Arc<ArtifactHistoryState>>,
    return_on_drop: bool,
}

impl HistoryView {
    pub fn new(inner: db_artifact_history::ArtifactHistoryView, admission: Option<ArtifactHistoryAdmission>, terminal_state: Arc<ArtifactHistoryState>) -> Self {
        Self { inner: Some(inner), admission, terminal_state: Some(terminal_state), return_on_drop: true }
    }

    pub fn entries(&self) -> &[HistoryEntry] {
        self.inner.as_ref().map_or(&[], |inner| inner.entries.as_slice())
    }

    pub fn operation_id_eq(&self, entry: usize, operation: usize, expected: &str) -> bool {
        self.inner.as_ref().is_some_and(|inner| inner.operation_id_eq(entry, operation, expected))
    }

    pub fn close_step(&mut self) -> bool {
        if self.inner.as_mut().is_some_and(db_artifact_history::ArtifactHistoryView::close_step) {
            return true;
        }
        if self.inner.take().is_some() {
            return true;
        }
        if self.admission.take().is_some() {
            return true;
        }
        if self.terminal_state.take().is_some() {
            return true;
        }
        false
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.inner.is_none() && self.admission.is_none()
    }

    pub(super) fn rearm_terminal_return(&mut self) {
        self.return_on_drop = true;
    }
}

impl Drop for HistoryView {
    fn drop(&mut self) {
        if !self.terminal_is_empty() && self.return_on_drop {
            if let Some(state) = self.terminal_state.as_ref().cloned() {
                let owner = HistoryView { inner: self.inner.take(), admission: self.admission.take(), terminal_state: Some(state.clone()), return_on_drop: false };
                state.finished.store(false, std::sync::atomic::Ordering::Release);
                register_artifact_history(&state);
                *state.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Ok(owner));
            }
        }
        assert!(self.terminal_is_empty(), "history result reached Drop before page/range/admission retirement");
    }
}
//#endregion 🔖️History

/// 🎫️ Counts the caller-held `ArtifactHandle`s of one mounted document. The last one to drop
/// unmounts the document: its `Ready` slot becomes `Closing` and its authority retires, so a
/// document holds its WAL writer and retained state only while someone holds it, and
/// the mount capacity bounds concurrently open documents instead of every document a process ever
/// opened. A mount that finds the slot `Closing` waits for that retirement before it replays the
/// WAL. Leases are only ever created under the registry lock. Database shutdown blocks on these
/// only; internal retained owners that still share the authority retire on their own.
pub struct ArtifactHandleLease {
    pub handles: Arc<std::sync::atomic::AtomicUsize>,
    pub registry: std::sync::Weak<Mutex<DatabaseDocumentMountRegistry>>,
    pub document: String,
}

impl ArtifactHandleLease {
    pub fn new(handles: Arc<std::sync::atomic::AtomicUsize>, registry: std::sync::Weak<Mutex<DatabaseDocumentMountRegistry>>, document: String) -> Self {
        handles.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        Self { handles, registry, document }
    }

    fn unmount_idle(&self) {
        let Some(registry) = self.registry.upgrade() else { return };
        let retired = {
            let mut registry = registry.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let idle = matches!(registry.slots.get(&self.document), Some(DatabaseDocumentMountSlot::Ready(reply)) if Arc::ptr_eq(&reply.handles, &self.handles));
            if !idle || self.handles.load(std::sync::atomic::Ordering::Acquire) != 0 {
                return;
            }
            let Some(DatabaseDocumentMountSlot::Ready(reply)) = registry.slots.remove(&self.document) else { return };
            if let Some(terminal) = reply.authority.take_terminal_signal() {
                registry.slots.insert(self.document.clone(), DatabaseDocumentMountSlot::Closing { terminal });
            }
            reply
        };
        drop(retired);
    }
}

impl Clone for ArtifactHandleLease {
    fn clone(&self) -> Self {
        Self::new(self.handles.clone(), self.registry.clone(), self.document.clone())
    }
}

impl Drop for ArtifactHandleLease {
    fn drop(&mut self) {
        if self.handles.fetch_sub(1, std::sync::atomic::Ordering::AcqRel) == 1 {
            self.unmount_idle();
        }
    }
}

/// 🧹️ One queued compaction turn that keeps its document mounted until the turn answers: the
/// actor mailbox refuses every queued message once the last handle unmounts the document.
pub struct ArtifactCompactionFuture {
    pub(super) ask: db_actor::AskFuture<db_artifact_contract::ArtifactMessage, Result<db_compact::CompactionReport, DbError>>,
    pub _lease: ArtifactHandleLease,
}

impl Future for ArtifactCompactionFuture {
    type Output = Result<Result<db_compact::CompactionReport, DbError>, DbError>;

    fn poll(self: std::pin::Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        std::pin::Pin::new(&mut self.get_mut().ask).poll(context)
    }
}

/// 📓️ A durable group journal sink that keeps its document mounted for as long as it lives, like
/// the `ArtifactHandle` it came from.
pub(super) struct ArtifactHandleJournalSinkV1 {
    pub(super) sink: Box<dyn store::durable_group::DurableOwnedGroupJournalSinkV1>,
    pub _lease: ArtifactHandleLease,
}

impl store::durable_group::DurableOwnedGroupJournalSinkV1 for ArtifactHandleJournalSinkV1 {
    fn begin_commit(&mut self, decision_pack: Vec<u8>, decision_sha256: String) -> Box<dyn store::durable_group::DurableOwnedGroupJournalCommitV1> {
        self.sink.begin_commit(decision_pack, decision_sha256)
    }
}
