//! 🩺️ Typed operation faults and raw input of the tool job contract: the bounded fault page, the completion value and result lane, the authoring effect, the completion and raw input records and the completion rejections. See [`super`].

use super::*;

pub const TYPED_OPERATION_FAULT_BYTES: usize = 480;

pub const TYPED_OPERATION_FAULT_PAGE_BYTES: usize = TYPED_OPERATION_FAULT_BYTES;

/// 🧾️ Decodes the canonical bounded Fault carried by one typed-operation result page.
pub fn decode_typed_operation_fault_page(bytes: &[u8]) -> Fault {
    semio_framework_diagnostic::try_decode_fault_bytes(bytes).unwrap_or_else(|| Fault::new(FaultOrigin::Framework, "interactive-job.app-owned-output", String::from_utf8_lossy(bytes)))
}

#[derive(Clone,Copy)]
pub struct ArtifactBoundedToolFault {
    pub bytes: [u8; TYPED_OPERATION_FAULT_BYTES],
    pub len: usize,
}

impl ArtifactBoundedToolFault {
    /// 🧾️ Retains a terminal job fault's detail bytes (bounded) so the publication that follows
    /// a cancelled lease reports the real cause instead of a generic cancellation.
    pub fn from_payload(detail: &semio_framework_job::RetainedJobPayload) -> Self {
        let mut bounded = Self { bytes: [0; TYPED_OPERATION_FAULT_BYTES], len: 0 };
        for page in 0..detail.page_count() {
            let Some(bytes) = detail.page(page) else { break };
            for byte in bytes {
                if bounded.len == bounded.bytes.len() {
                    return bounded;
                }
                bounded.bytes[bounded.len] = *byte;
                bounded.len += 1;
            }
        }
        bounded
    }

    /// 🧾️ Recovers the canonical reducer refusal before retaining its bounded publication frame.
    fn from_retained_fault(detail: &semio_framework_job::RetainedJobPayload) -> Self {
        let payload = Self::from_payload(detail);
        let fault = reducer_fault_of_detail(payload.as_bytes()).unwrap_or_else(|| Fault::new(FaultOrigin::Framework, "interactive-job.app-owned-output", String::from_utf8_lossy(payload.as_bytes())));
        Self::from_fault(&fault)
    }

    pub(crate) fn from_inline_report(bytes:[u8;480],len:usize)->Self{Self{bytes,len}}
    pub fn from_fault(fault: &Fault) -> Self {
        let encoded = semio_framework_diagnostic::encode_fault_bytes_bounded(fault, TYPED_OPERATION_FAULT_BYTES).unwrap_or_else(|| {
            semio_framework_diagnostic::encode_fault_bytes(&Fault::new(FaultOrigin::Framework, "interactive-job.fault-capacity", "the typed operation fault exceeded its bounded result page").with_retained_progress(fault.retained_progress()))
        });
        let mut bounded = Self { bytes: [0; TYPED_OPERATION_FAULT_BYTES], len: encoded.len() };
        bounded.bytes[..encoded.len()].copy_from_slice(&encoded);
        bounded
    }

    pub fn into_fault(self) -> Fault {
        decode_typed_operation_fault_page(&self.bytes[..self.len])
    }

    /// 🧾️ Copies the canonical Fault wire into the result page that crosses the app frame.
    pub fn framed_page_bytes(&self, buffer: &mut [u8; TYPED_OPERATION_FAULT_PAGE_BYTES]) -> usize {
        buffer[..self.len].copy_from_slice(&self.bytes[..self.len]);
        self.len
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes[..self.len]
    }
}

semio_framework_value::artifact_retire_leaf!(ArtifactBoundedToolFault);

pub enum ArtifactToolCompletionValue<A: ArtifactApp> {
    Emit(ArtifactMutationOutcome<<A as ArtifactApp>::Mutation, <A as ArtifactApp>::ConfigMutation, <A as ArtifactApp>::DraftMutation, ArtifactCompletionFault>, EphemeralEmit<A>),
    Download(Result<ArtifactDownloadOutput, ArtifactCompletionFault>, EphemeralEmit<A>),
}

pub use semio_framework_actor::typed_operation_page::TypedOperationResultLane;

pub(crate) struct OriginalAuthoringEffect {
    pub(super) receipt: Option<semio_framework_value::native_encoding::NativeEncodeContinuation>,
    pub token: semio_framework_job::CancelToken,
    pub(super) publication: PublicationClaimHandle,
    pub(super) progress: semio_framework_value::native_encoding::NativeEncodeProgress,
}

impl OriginalAuthoringEffect {
    pub(super) fn run<T>(&mut self, operation: impl FnOnce(&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<T, Fault>) -> Result<T, Fault> {
        let Self { receipt, token, publication, progress } = self;
        let original = receipt.take().ok_or_else(|| plugin_sdk_fault("app-owned effect lacks its original authoring receipt"))?;
        let mut observed = *progress;
        let mut observer = |next: semio_framework_value::native_encoding::NativeEncodeProgress| { observed = next; !token.is_cancelled_now() && !publication.is_cancelled() };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::resume(original, &mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>);
        let result = operation(&mut identity);
        *receipt = Some(identity.pause().map_err(|error| plugin_sdk_fault(error.to_string()))?);
        drop(observer);
        *progress = observed;
        result
    }
}

/// 📬️ Operation-owned completion cell shared by an app factory's job and the exact framework
/// commit path. It is single-assignment and consumed exactly once after freshness validation.
pub struct ArtifactToolCompletion<A: ArtifactApp> {
    #[doc(hidden)]
    pub inner: crate::completion_retirement::CompletionCell<A>,

}

/// 🧬️ Shared retained raw-page lease. Application jobs may inspect one page per step;
/// the mounted operation keeps the sole close authority until every job clone is returned.
#[derive(Clone)]
pub struct ArtifactToolRawInput {
    pub(super) inner: std::sync::Arc<std::sync::Mutex<Option<semio_framework::action_bus::RetainedToolWireInput>>>,
}

impl ArtifactToolRawInput {
    pub fn new(input: semio_framework::action_bus::RetainedToolWireInput) -> Self {
        Self { inner: std::sync::Arc::new(std::sync::Mutex::new(Some(input))) }
    }

    pub fn transferred_to_factory() -> Self {
        Self { inner: std::sync::Arc::new(std::sync::Mutex::new(None)) }
    }

    pub fn page<R>(&self, index: usize, read: impl FnOnce(Option<&[u8]>) -> R) -> Result<R, Fault> {
        let input = self.inner.try_lock().map_err(|_| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.raw-page-busy"), "typed-operation raw page owner is busy or poisoned"))?;
        Ok(read(input.as_ref().and_then(|input| input.page(index))))
    }

    pub fn page_count(&self) -> Result<usize, Fault> {
        let input = self.inner.try_lock().map_err(|_| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.raw-page-busy"), "typed-operation raw page owner is busy or poisoned"))?;
        Ok(input.as_ref().map_or(0, semio_framework::action_bus::RetainedToolWireInput::page_count))
    }

    /// 📏️ Quotes releasing the retained raw page backing whole; a held or busy lease quotes nothing.
    pub fn retirement_demands(&self) -> Result<RetirementDemand, ValueError> {
        if std::sync::Arc::strong_count(&self.inner) != 1 {
            return Ok(RetirementDemand::default());
        }
        let Ok(input) = self.inner.try_lock() else { return Ok(RetirementDemand::default()) };
        let Some(owner) = input.as_ref() else { return Ok(RetirementDemand::default()) };
        Ok(RetirementDemand { copy_bytes: owner.next_close_copy_byte_demand()?, capacity_bytes: owner.next_close_capacity_byte_demand(0)?, release_bytes: owner.next_close_release_byte_demand()?, depth: owner.next_close_depth_demand()? })
    }

    pub fn close_step(&self, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, Fault> {
        if std::sync::Arc::strong_count(&self.inner) != 1 {
            return Ok(PluginLifecycleStep::Blocked { reason: "typed-operation raw page lease remains held by its app-owned decoder job" });
        }
        let mut input = self.inner.try_lock().map_err(|_| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.raw-page-busy"), "typed-operation raw page owner is busy or poisoned"))?;
        let Some(owner) = input.as_mut() else { return Ok(PluginLifecycleStep::Complete(Default::default())) };
        let step = owner.close_step(grant).map_err(semio_framework_plugin_ownership::bounded_ownership::plugin_retirement_fault)?;
        if owner.terminal_is_empty() {
            *input = None;
        }
        match step {
            InteractiveJobCloseStep::Pending { progress } => Ok(PluginLifecycleStep::Progress(progress)),
            InteractiveJobCloseStep::Blocked => Ok(PluginLifecycleStep::Blocked { reason: "typed-operation raw page close is blocked" }),
            InteractiveJobCloseStep::Complete { progress } => Ok(PluginLifecycleStep::Complete(progress)),
        }
    }

    pub fn terminal_is_empty(&self) -> bool {
        std::sync::Arc::strong_count(&self.inner) == 1 && std::sync::Arc::weak_count(&self.inner) == 0 && self.inner.try_lock().is_ok_and(|input| input.is_none())
    }
}

impl<A: ArtifactApp> Clone for ArtifactToolCompletion<A> {
    fn clone(&self) -> Self {
        Self { inner: self.inner.clone() }
    }
}

pub struct ArtifactDownloadCompletionRejection<A:ArtifactApp>{pub download:Result<ArtifactDownloadOutput,Fault>,pub ephemeral:EphemeralEmit<A>,pub fault:Fault}

impl<A:ArtifactApp> std::fmt::Debug for ArtifactDownloadCompletionRejection<A>{fn fmt(&self,formatter:&mut std::fmt::Formatter<'_>)->std::fmt::Result{formatter.debug_struct("ArtifactDownloadCompletionRejection").field("fault",&self.fault).finish_non_exhaustive()}}

pub struct ArtifactToolCompletionRejection<A: ArtifactApp> {
    pub emit: ArtifactMutationOutcome<A::Mutation, A::ConfigMutation, A::DraftMutation>,
    pub ephemeral: EphemeralEmit<A>,
    pub fault: Fault,
}

impl<A: ArtifactApp> std::fmt::Debug for ArtifactToolCompletionRejection<A> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("ArtifactToolCompletionRejection").field("fault", &self.fault).finish_non_exhaustive()
    }
}
