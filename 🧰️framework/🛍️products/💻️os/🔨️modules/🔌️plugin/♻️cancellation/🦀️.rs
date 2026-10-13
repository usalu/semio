//! ♻️ Tool operation keys, document authority and tool cancellation scopes, handles and leases.
use semio_framework::{Fault, FaultCode, FaultOrigin};
use semio_framework_async::{PublicationClaimHandle, PublicationClaim, PublicationPermit};
use crate::bounded_ownership::ArtifactFixedRegistry;
use crate::sdk_fault::plugin_sdk_fault;



#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ArtifactDocumentAuthority(pub u32);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ToolOperationKey {
    pub app_instance_id: u32,
    pub document: ArtifactDocumentAuthority,
    pub operation_id: semio_framework_job::OperationId,
    pub base_revision: semio_framework_job::RevisionId,
    pub generation: semio_framework_job::Generation,
}

pub(crate) const TOOL_CANCELLATION_SLOTS: usize = 1_024;

const TOOL_CANCELLATION_WORDS: usize = TOOL_CANCELLATION_SLOTS / u64::BITS as usize;

struct ToolPublicationScopePermit {
    _app: PublicationPermit,
    _document: Option<PublicationPermit>,
    _operation: PublicationPermit,
}

pub(crate) struct ToolDocumentCancellationScope {
    pub(crate) document: ArtifactDocumentAuthority,
    pub(crate) generation: u64,
    token: semio_framework_job::CancelToken,
    pub(crate) publication_claim: PublicationClaimHandle,
    pub(crate) operation: ToolOperationKey,
}

pub(crate) struct ToolKeyedDocumentCancellationScope {
    pub(crate) token: semio_framework_job::CancelToken,
    publication_claim: PublicationClaimHandle,
    pub(crate) active: usize,
}

pub(crate) struct ToolCancellationState {
    slots: Box<[std::mem::MaybeUninit<ToolDocumentCancellationScope>]>,
    occupied: [u64; TOOL_CANCELLATION_WORDS],
    allocation_admitted: bool,
    pub(crate) keyed_operations: ArtifactFixedRegistry<ToolDocumentCancellationScope>,
    pub(crate) keyed_documents: ArtifactFixedRegistry<ToolKeyedDocumentCancellationScope>,
}

impl ToolCancellationState {
    fn occupied(&self, index: usize) -> bool {
        self.occupied[index / u64::BITS as usize] & (1u64 << (index % u64::BITS as usize)) != 0
    }

    fn set_occupied(&mut self, index: usize, occupied: bool) {
        let word = &mut self.occupied[index / u64::BITS as usize];
        let mask = 1u64 << (index % u64::BITS as usize);
        if occupied {
            *word |= mask;
        } else {
            *word &= !mask;
        }
    }

    /// 🗝️ SAFETY: occupancy is set only after `write` and cleared before `assume_init_read`.
    pub(crate) fn get(&self, index: usize) -> Option<&ToolDocumentCancellationScope> {
        if !self.occupied(index) {
            return None;
        }
        self.slots.get(index).map(|slot| unsafe { slot.assume_init_ref() })
    }

    fn insert_admitted(&mut self, index: usize, value: ToolDocumentCancellationScope) {
        debug_assert!(self.allocation_admitted && !self.occupied(index));
        self.slots[index].write(value);
        self.set_occupied(index, true);
    }

    /// 🫳️ SAFETY: occupancy was checked and is now cleared before the exact initialized read.
    pub(crate) fn take(&mut self, index: usize) -> Option<ToolDocumentCancellationScope> {
        if !self.occupied(index) {
            return None;
        }
        self.set_occupied(index, false);
        Some(unsafe { self.slots[index].assume_init_read() })
    }
}

impl Drop for ToolCancellationState {
    fn drop(&mut self) {}
}

#[derive(Clone)]
pub struct ToolCancellationHandle {
    pub(crate) state: std::sync::Arc<std::sync::Mutex<ToolCancellationState>>,
    pub(crate) app_scope: semio_framework_job::CancelToken,
    pub(crate) publication_scope: PublicationClaimHandle,
    pub(crate) app_generation: std::sync::Arc<std::sync::atomic::AtomicU64>,
    pub(crate) active: std::sync::Arc<std::sync::atomic::AtomicUsize>,
}

impl Default for ToolCancellationHandle {
    fn default() -> Self {
        let mut slots = Vec::new();
        let allocation_admitted = slots.try_reserve_exact(TOOL_CANCELLATION_SLOTS).is_ok();
        if allocation_admitted {
            slots.resize_with(TOOL_CANCELLATION_SLOTS, std::mem::MaybeUninit::uninit);
        }
        Self {
            state: std::sync::Arc::new(std::sync::Mutex::new(ToolCancellationState {
                slots: slots.into_boxed_slice(),
                occupied: [0; TOOL_CANCELLATION_WORDS],
                allocation_admitted,
                keyed_operations: ArtifactFixedRegistry::new(),
                keyed_documents: ArtifactFixedRegistry::new(),
            })),
            app_scope: semio_framework_job::CancelToken::root_now(),
            publication_scope: PublicationClaim::new(),
            app_generation: std::sync::Arc::new(std::sync::atomic::AtomicU64::new(0)),
            active: std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0)),
        }
    }
}

impl ToolCancellationHandle {
    pub(crate) fn slot(document: ArtifactDocumentAuthority) -> usize {
        document.0 as usize % TOOL_CANCELLATION_SLOTS
    }

    fn try_state(&self) -> Result<std::sync::MutexGuard<'_, ToolCancellationState>, Fault> {
        self.state.try_lock().map_err(|_| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.cancellation-busy"), "cancellation authority is contended or poisoned"))
    }

    pub(crate) fn begin(&self, key: ToolOperationKey) -> Result<ToolCancellationLease, Fault> {
        let document_token = self.app_scope.child_now();
        let token = document_token.child_now();
        let publication_claim = PublicationClaim::new();
        let index = Self::slot(key.document);
        let (superseded, generation) = {
            let mut state = self.try_state()?;
            if !state.allocation_admitted {
                return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.cancellation-capacity"), "fixed cancellation slots were not admitted"));
            }
            if state.get(index).is_some_and(|scope| scope.document != key.document) {
                return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.cancellation-collision"), "fixed cancellation authority slot collision"));
            }
            let superseded = state.take(index);
            let generation = superseded.as_ref().map_or(0, |scope| scope.generation.saturating_add(1));
            state.insert_admitted(index, ToolDocumentCancellationScope { document: key.document, generation, token: document_token, publication_claim: publication_claim.clone(), operation: key.clone() });
            drop(state);
            (superseded, generation)
        };
        if let Some(scope) = superseded {
            scope.publication_claim.cancel();
            scope.token.cancel_now();
        } else {
            self.active.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        }
        Ok(ToolCancellationLease { handle: self.clone(), document: key.document, document_generation: generation, key, token, publication_claim, document_claim: None, keyed: false, finished: false })
    }

    pub(crate) fn begin_keyed(&self, key: ToolOperationKey) -> Result<ToolCancellationLease, Fault> {
        let mut state = self.try_state()?;
        if !state.keyed_operations.can_insert(key.operation_id.0) {
            return Err(plugin_sdk_fault("keyed cancellation operation has no exact pre-admitted slot"));
        }
        let document_id = u64::from(key.document.0);
        if state.keyed_documents.get(document_id).is_none() {
            if !state.keyed_documents.can_insert(document_id) {
                return Err(plugin_sdk_fault("keyed cancellation document has no exact pre-admitted slot"));
            }
            state.keyed_documents.insert_admitted(document_id, ToolKeyedDocumentCancellationScope { token: self.app_scope.child_now(), publication_claim: PublicationClaim::new(), active: 0 });
        }
        let document = state.keyed_documents.get_mut(document_id).expect("exact keyed document was admitted");
        if document.token.is_cancelled_now() {
            document.token = self.app_scope.child_now();
            document.publication_claim = PublicationClaim::new();
        }
        let token = document.token.child_now();
        let document_claim = document.publication_claim.clone();
        document.active += 1;
        let publication_claim = PublicationClaim::new();
        state
            .keyed_operations
            .insert_admitted(key.operation_id.0, ToolDocumentCancellationScope { document: key.document, generation: key.generation.0, token: token.clone(), publication_claim: publication_claim.clone(), operation: key.clone() });
        self.active.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        Ok(ToolCancellationLease { handle: self.clone(), document: key.document, document_generation: key.generation.0, key, token, publication_claim, document_claim: Some(document_claim), keyed: true, finished: false })
    }

    #[cfg(test)]
    fn test_begin(&self, key: ToolOperationKey) -> Result<TestToolCancellationLease, Fault> {
        self.begin(key).map(TestToolCancellationLease)
    }

    pub fn cancel(&self, key: &ToolOperationKey) -> Result<bool, Fault> {
        let detached = {
            let mut state = self.try_state()?;
            if let Some(scope) = state.keyed_operations.get(key.operation_id.0).filter(|scope| scope.operation == *key) {
                scope.publication_claim.cancel();
                scope.token.cancel_now();
                return Ok(true);
            }
            let index = Self::slot(key.document);
            let detached = if state.get(index).is_some_and(|scope| scope.document == key.document && scope.operation == *key) { state.take(index) } else { None };
            drop(state);
            detached
        };
        if let Some(scope) = detached {
            self.active.fetch_sub(1, std::sync::atomic::Ordering::AcqRel);
            scope.publication_claim.cancel();
            scope.token.cancel_now();
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn active_operation_count(&self) -> usize {
        self.active.load(std::sync::atomic::Ordering::Acquire)
    }

    pub fn scope_generation(&self) -> u64 {
        self.app_generation.load(std::sync::atomic::Ordering::Acquire)
    }

    pub fn cancel_document(&self, document: ArtifactDocumentAuthority) -> Result<bool, Fault> {
        let (detached, keyed) = {
            let mut state = self.try_state()?;
            let keyed = if let Some(scope) = state.keyed_documents.get(u64::from(document.0)) {
                scope.publication_claim.cancel();
                scope.token.cancel_now();
                true
            } else {
                false
            };
            let index = Self::slot(document);
            let detached = if state.get(index).is_some_and(|scope| scope.document == document) { state.take(index) } else { None };
            drop(state);
            (detached, keyed)
        };
        if let Some(scope) = detached {
            self.active.fetch_sub(1, std::sync::atomic::Ordering::AcqRel);
            scope.publication_claim.cancel();
            scope.token.cancel_now();
            Ok(true)
        } else {
            Ok(keyed)
        }
    }

    pub(crate) fn renew_scope_generation(&mut self) {
        self.cancel_scope_generation();
        self.app_scope = semio_framework_job::CancelToken::root_now();
        self.publication_scope = PublicationClaim::new();
    }

    pub fn cancel_scope_generation(&self) {
        self.publication_scope.cancel();
        self.app_scope.cancel_now();
        let _ = self.app_generation.try_update(std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire, |generation| Some(generation.saturating_add(1)));
    }

    pub(crate) fn cleanup_slot(&self, index: usize) -> Result<bool, Fault> {
        if index >= TOOL_CANCELLATION_SLOTS {
            let mut state = self.try_state()?;
            let Some(operation) = state.keyed_operations.id_at(index - TOOL_CANCELLATION_SLOTS) else {
                return Ok(false);
            };
            let scope = state.keyed_operations.remove(operation).expect("exact keyed cancellation cleanup slot");
            scope.publication_claim.cancel();
            scope.token.cancel_now();
            let document_id = u64::from(scope.document.0);
            let document = state.keyed_documents.get_mut(document_id).expect("keyed cancellation cleanup retains document authority");
            document.active -= 1;
            if document.active == 0 {
                drop(state.keyed_documents.remove(document_id));
            }
            self.active.fetch_sub(1, std::sync::atomic::Ordering::AcqRel);
            return Ok(true);
        }
        let detached = {
            let mut state = self.try_state()?;
            let detached = state.take(index);
            drop(state);
            detached
        };
        if let Some(scope) = detached {
            self.active.fetch_sub(1, std::sync::atomic::Ordering::AcqRel);
            scope.publication_claim.cancel();
            scope.token.cancel_now();
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub(crate) fn cleanup_finished_slot(&self, index: usize) -> Result<Option<bool>, Fault> {
        let mut state = match self.state.try_lock() {
            Ok(state) => state,
            Err(std::sync::TryLockError::WouldBlock) => return Ok(None),
            Err(std::sync::TryLockError::Poisoned(_)) => return Err(plugin_sdk_fault("finished cancellation authority is poisoned")),
        };
        let detached = if index < TOOL_CANCELLATION_SLOTS {
            if !state.get(index).is_some_and(|scope| scope.publication_claim.is_finished()) {
                return Ok(Some(false));
            }
            state.take(index)
        } else {
            let Some(operation) = state.keyed_operations.id_at(index - TOOL_CANCELLATION_SLOTS) else {
                return Ok(Some(false));
            };
            if !state.keyed_operations.get(operation).is_some_and(|scope| scope.publication_claim.is_finished()) {
                return Ok(Some(false));
            }
            let scope = state.keyed_operations.remove(operation).expect("finished exact keyed cancellation owner");
            let document_id = u64::from(scope.document.0);
            let document = state.keyed_documents.get_mut(document_id).expect("finished keyed operation retains document count");
            document.active -= 1;
            if document.active == 0 {
                drop(state.keyed_documents.remove(document_id));
            }
            Some(scope)
        };
        if detached.is_some() {
            self.active.fetch_sub(1, std::sync::atomic::Ordering::AcqRel);
        }
        Ok(Some(detached.is_some()))
    }
}

pub(crate) struct ToolCancellationLease {
    pub(crate) handle: ToolCancellationHandle,
    pub(crate) document: ArtifactDocumentAuthority,
    pub(crate) document_generation: u64,
    pub(crate) key: ToolOperationKey,
    pub(crate) token: semio_framework_job::CancelToken,
    pub(crate) publication_claim: PublicationClaimHandle,
    pub(crate) document_claim: Option<PublicationClaimHandle>,
    pub(crate) keyed: bool,
    pub(crate) finished: bool,
}

impl ToolCancellationLease {
    pub(crate) async fn is_cancelled(&self) -> bool {
        self.token.is_cancelled().await
    }

    pub(crate) fn cancel_token(&self) -> semio_framework_job::CancelToken {
        self.token.child_now()
    }

    pub(crate) fn cancel(&self) {
        self.publication_claim.cancel();
        self.token.cancel_now();
    }

    pub(crate) fn rebind_keyed(&mut self, base_revision: semio_framework_job::RevisionId, generation: semio_framework_job::Generation) -> Result<bool, Fault> {
        let mut state = match self.handle.state.try_lock() {
            Ok(state) => state,
            Err(std::sync::TryLockError::WouldBlock) => return Ok(false),
            Err(std::sync::TryLockError::Poisoned(_)) => return Err(plugin_sdk_fault("keyed cancellation authority is poisoned")),
        };
        if !self.keyed || self.finished || self.publication_claim.is_cancelled() || self.token.is_cancelled_now() {
            return Err(plugin_sdk_fault("only a live exact keyed lease may rebind its revision"));
        }
        let scope = state.keyed_operations.get_mut(self.key.operation_id.0).ok_or_else(|| plugin_sdk_fault("keyed rebase lost its registered cancellation authority"))?;
        if scope.operation != self.key || !PublicationClaimHandle::same_original(&scope.publication_claim, &self.publication_claim) {
            return Err(plugin_sdk_fault("keyed rebase does not own the registered cancellation authority"));
        }
        scope.operation.base_revision = base_revision;
        scope.operation.generation = generation;
        scope.generation = generation.0;
        self.key.base_revision = base_revision;
        self.key.generation = generation;
        self.document_generation = generation.0;
        Ok(true)
    }

    pub(crate) fn try_claim_publication(&self) -> Option<ToolPublicationScopePermit> {
        let app = self.handle.publication_scope.try_claim()?;
        let document = match self.document_claim.as_ref() {
            Some(claim) => Some(claim.try_claim()?),
            None => None,
        };
        let operation = self.publication_claim.try_claim()?;
        Some(ToolPublicationScopePermit { _app: app, _document: document, _operation: operation })
    }

    fn release_current(&self) {
        let Ok(mut state) = self.handle.state.try_lock() else { return };
        if self.keyed {
            if !state.keyed_operations.get(self.key.operation_id.0).is_some_and(|scope| scope.operation == self.key && PublicationClaimHandle::same_original(&scope.publication_claim, &self.publication_claim)) {
                return;
            }
            let detached = state.keyed_operations.remove(self.key.operation_id.0);
            if detached.is_some() {
                let document_id = u64::from(self.document.0);
                let document = state.keyed_documents.get_mut(document_id).expect("keyed operation retains its exact document scope");
                document.active -= 1;
                if document.active == 0 {
                    drop(state.keyed_documents.remove(document_id));
                }
                self.handle.active.fetch_sub(1, std::sync::atomic::Ordering::AcqRel);
            }
            return;
        }
        let index = ToolCancellationHandle::slot(self.document);
        let detached = if state.get(index).is_some_and(|scope| scope.document == self.document && scope.generation == self.document_generation && scope.operation == self.key) { state.take(index) } else { None };
        drop(state);
        if detached.is_some() {
            self.handle.active.fetch_sub(1, std::sync::atomic::Ordering::AcqRel);
        }
    }

    pub(crate) fn finish(mut self) {
        self.publication_claim.finish();
        self.release_current();
        self.finished = true;
    }

    pub(crate) fn finish_in_place(&mut self) {
        self.publication_claim.finish();
        self.release_current();
        self.finished = true;
    }
}

#[cfg(test)]
struct TestToolCancellationLease(ToolCancellationLease);

#[cfg(test)]
impl TestToolCancellationLease {
    fn cancel_token(&self) -> semio_framework_job::CancelToken {
        self.0.cancel_token()
    }

    fn finish(self) {
        self.0.finish();
    }
}

impl Drop for ToolCancellationLease {
    fn drop(&mut self) {
        if !self.finished {
            self.cancel();
        }
        self.publication_claim.finish();
        self.release_current();
    }
}
