//! ♻️ Tool operation keys, document authority and tool cancellation scopes, handles and leases.
use semio_framework::{Fault, FaultCode, FaultOrigin};
use semio_framework_async::{PublicationClaimHandle, PublicationClaim, PublicationPermit};
use crate::bounded_ownership::ArtifactFixedRegistry;
use crate::sdk_fault::plugin_sdk_fault;
#[path="🎟️issuer/🦀️.rs"]
mod issuer;
pub(crate) use issuer::ToolCancellationIssuer;
#[cfg(any(test,feature="media-turn-testing"))]
pub(crate) use issuer::cancellation_issuer_original_creation_physical_birth_and_close;

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

pub(crate) struct ToolPublicationScopePermit {
    _app: PublicationPermit,
    _document: Option<PublicationPermit>,
    _operation: PublicationPermit,
}

pub(crate) struct ToolDocumentCancellationScope {
    pub(crate) document: ArtifactDocumentAuthority,
    pub(crate) generation: u64,
    pub(crate) token: semio_framework_job::CancelToken,
    pub(crate) publication_claim: PublicationClaimHandle,
    pub(crate) operation: ToolOperationKey,
}

pub(crate) struct ToolKeyedDocumentCancellationScope {
    pub(crate) token: semio_framework_job::CancelToken,
    pub(crate) publication_claim: PublicationClaimHandle,
    pub(crate) active: usize,
}

pub(crate) struct ToolCancellationState {
    slots: Box<[std::mem::MaybeUninit<ToolDocumentCancellationScope>]>,
    occupied: [u64; TOOL_CANCELLATION_WORDS],
    pub(crate) allocation_admitted: bool,
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

pub struct ToolCancellationHandle {
    pub(crate) state: std::sync::Arc<semio_framework_value::retirement::controlled::RetainedOwnerGate<ToolCancellationState>>,
    pub(crate) app_scope: semio_framework_job::CancelToken,
    pub(crate) publication_scope: PublicationClaimHandle,
    pub(crate) app_generation: std::sync::Arc<std::sync::atomic::AtomicU64>,
    pub(crate) active: std::sync::Arc<std::sync::atomic::AtomicUsize>,
}

impl ToolCancellationHandle {
    pub(crate) fn slot(document: ArtifactDocumentAuthority) -> usize {
        document.0 as usize % TOOL_CANCELLATION_SLOTS
    }

    fn try_state(&self) -> Result<semio_framework_value::retirement::controlled::RetainedOwnerGateGuard<'_, ToolCancellationState>, Fault> {
        self.state.try_lock().map_err(|_| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.cancellation-busy"), "cancellation authority is contended or poisoned"))
    }

    pub fn cancel(&self, key: &ToolOperationKey) -> Result<bool, Fault> {
        let state=self.try_state()?;
        let scope=state.keyed_operations.get(key.operation_id.0).filter(|scope|scope.operation==*key).or_else(||state.get(Self::slot(key.document)).filter(|scope|scope.document==key.document&&scope.operation==*key));
        if let Some(scope)=scope{scope.publication_claim.cancel();scope.token.cancel_now();Ok(true)}else{Ok(false)}
    }

    pub fn active_operation_count(&self) -> usize {
        self.active.load(std::sync::atomic::Ordering::Acquire)
    }

    pub fn scope_generation(&self) -> u64 {
        self.app_generation.load(std::sync::atomic::Ordering::Acquire)
    }

    pub fn cancel_document(&self, document: ArtifactDocumentAuthority) -> Result<bool, Fault> {
        let state=self.try_state()?;let mut cancelled=false;
        if let Some(scope)=state.keyed_documents.get(u64::from(document.0)){scope.publication_claim.cancel();scope.token.cancel_now();cancelled=true;}
        if let Some(scope)=state.get(Self::slot(document)).filter(|scope|scope.document==document){scope.publication_claim.cancel();scope.token.cancel_now();cancelled=true;}
        Ok(cancelled)
    }

    pub(crate) fn admit_alias(&self,grant:semio_framework_value::RetainedCloneGrant)->Result<Option<(Self,semio_framework_value::RetainedCloneProgress)>,semio_framework_value::ValueError>{
        let bytes=size_of::<Self>();if grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return Ok(None)}
        let alias=Self{state:std::sync::Arc::clone(&self.state),app_scope:self.app_scope.clone(),publication_scope:self.publication_scope.clone(),app_generation:std::sync::Arc::clone(&self.app_generation),active:std::sync::Arc::clone(&self.active)};
        Ok(Some((alias,semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})))
    }

    pub fn cancel_scope_generation(&self) {
        self.publication_scope.cancel();
        self.app_scope.cancel_now();
        let _ = self.app_generation.try_update(std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire, |generation| Some(generation.saturating_add(1)));
    }


}

pub(crate) struct ToolCancellationLeaseOriginal {
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

pub(crate) struct ToolCancellationLease{original:std::mem::ManuallyDrop<ToolCancellationLeaseOriginal>}
impl ToolCancellationLease{
    pub(crate) fn from_original(original:ToolCancellationLeaseOriginal)->Self{Self{original:std::mem::ManuallyDrop::new(original)}}
    pub(crate) fn into_retirement_source(self)->ToolCancellationLeaseOriginal{
        let original=std::mem::ManuallyDrop::new(self);
        unsafe{std::mem::ManuallyDrop::into_inner(std::ptr::read(&original.original))}
    }
}
impl std::ops::Deref for ToolCancellationLease{type Target=ToolCancellationLeaseOriginal;fn deref(&self)->&Self::Target{&self.original}}
impl std::ops::DerefMut for ToolCancellationLease{fn deref_mut(&mut self)->&mut Self::Target{&mut self.original}}

impl ToolCancellationLease {
    pub(crate) async fn is_cancelled(&self) -> bool {
        self.token.is_cancelled().await
    }

    pub(crate) fn cancel_token(&self) -> &semio_framework_job::CancelToken {
        &self.token
    }

    pub(crate) fn cancel(&self) {
        self.publication_claim.cancel();
        self.token.cancel_now();
    }

    pub(crate) fn rebind_keyed(&mut self, base_revision: semio_framework_job::RevisionId, generation: semio_framework_job::Generation) -> Result<bool, Fault> {
        let original=&mut*self.original;
        let mut state = match original.handle.state.try_lock() {
            Ok(state) => state,
            Err(_) => return Ok(false),
        };
        if !original.keyed || original.finished || original.publication_claim.is_cancelled() || original.token.is_cancelled_now() {
            return Err(plugin_sdk_fault("only a live exact keyed lease may rebind its revision"));
        }
        let scope = state.keyed_operations.get_mut(original.key.operation_id.0).ok_or_else(|| plugin_sdk_fault("keyed rebase lost its registered cancellation authority"))?;
        if scope.operation != original.key || !PublicationClaimHandle::same_original(&scope.publication_claim, &original.publication_claim) {
            return Err(plugin_sdk_fault("keyed rebase does not own the registered cancellation authority"));
        }
        scope.operation.base_revision = base_revision;
        scope.operation.generation = generation;
        scope.generation = generation.0;
        original.key.base_revision = base_revision;
        original.key.generation = generation;
        original.document_generation = generation.0;
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

    pub(crate) fn finish(&mut self) {
        self.publication_claim.finish();
        self.finished = true;
    }

    pub(crate) fn finish_in_place(&mut self) {
        self.publication_claim.finish();
        self.finished = true;
    }
}

impl Drop for ToolCancellationLease {
    fn drop(&mut self) {
        assert!(std::thread::panicking(),"original cancellation lease requires its paid terminal cursor");
    }
}

include!("📦️lease/🦀️.rs");
