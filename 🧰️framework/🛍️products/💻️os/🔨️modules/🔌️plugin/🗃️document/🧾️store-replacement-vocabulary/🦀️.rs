//! 🧾️ Store replacement vocabulary: the replacement state, the refusal code and refusal, the replacement legs, the publication guard and the active store replacement record. See [`super`].

use super::*;

#[doc(hidden)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActiveArtifactStoreReplacementState {
    Initializing,
    AwaitingMembers,
    OpeningMembers,
    ClosingRejectedMember,
    ValidatingClosure,
    PreparingCandidateViews,
    CandidateReady,
    RetiringRejectedMembers,
    RetiringCommittedMembers,
    RetiringCommittedStore,
    RetiringRejectedCandidate,
    Complete,
}

/// 🧭️ Which leg of a document-archive replacement refused the candidate, and with what
/// evidence. An atomic replacement validates three of them in order — the owned-member set
/// (`members`), the recursive ownership `closure`, and the `retained-publication` guard plus the
/// app's own publication `authority` — and every one of them used to collapse into a single
/// `faulted: bool` whose only explanation was a `#[cfg(test)]` `eprintln!`. A headless host
/// therefore saw one opaque sentence for eleven distinct causes. This carries the cause to the
/// wire instead: {@link ArtifactStoreReplacementRefusal::into_fault} mints the leg-specific
/// {@link FaultCode} and detail that `Effect::LoadDocument` reports to the shell.
/// 🔖️ A bounded inline copy of a job fault's first detail page — the initializer's own
/// refusal slug (e.g. `bounded-store.initializer-envelope-invalid`), which the replacement used
/// to drop on the floor. Fixed capacity, no allocation, `Copy`: it rides the refusal record.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct ArtifactStoreRefusalCode {
    pub bytes: [u8; 96],
    pub len: u8,
}

impl ArtifactStoreRefusalCode {
    pub const EMPTY: Self = Self { bytes: [0; 96], len: 0 };

    pub(super) fn from_page(page: &[u8]) -> Self {
        let mut bytes = [0u8; 96];
        let len = page.len().min(bytes.len());
        bytes[..len].copy_from_slice(&page[..len]);
        Self { bytes, len: len as u8 }
    }

    pub fn text(&self) -> &str {
        std::str::from_utf8(&self.bytes[..self.len as usize]).unwrap_or("<non-utf8 refusal page>")
    }
}

#[doc(hidden)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArtifactStoreReplacementRefusal {
    Invariant { leg: ArtifactStoreReplacementLeg, detail: &'static str },
    InitializerAdmissionRejected,
    InitializerFailed(ArtifactStoreRefusalCode),
    Cancelled,
    MemberOpenRejected { ordinal: usize, diagnostic: store::MemberOpenDiagnostic },
    MemberIdentityMismatch { ordinal: usize, expected: usize },
    MemberFactoryRefused { ordinal: usize },
    ClosureRejected { diagnostic: store::OwnedDocumentClosureDiagnostic },
    ClosureMemberCount { members: usize, children: usize },
    PublicationGuard(ArtifactStoreReplacementPublicationGuard),
    PublicationAuthorityRejected,
}

/// 🦵️ The named leg a refusal belongs to — the same three the host message lists.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum ArtifactStoreReplacementLeg {
    Initializer,
    Members,
    Closure,
    CandidateViews,
    RetainedPublication,
}

impl ArtifactStoreReplacementLeg {
    pub fn slug(self) -> &'static str {
        match self {
            Self::Initializer => "initializer",
            Self::Members => "members",
            Self::Closure => "closure",
            Self::CandidateViews => "candidate-views",
            Self::RetainedPublication => "retained-publication",
        }
    }
}

/// 🚦️ Every input the `CandidateReady` publication guard reads, as it read them.
/// A bare "guard rejected" is unactionable; which of the seven conditions tripped is the answer.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ArtifactStoreReplacementPublicationGuard {
    pub closing: bool,
    pub cancelled: bool,
    pub parent_generation: u64,
    pub live_generation: u64,
    pub base_child_generation: u64,
    pub child_generation: u64,
    pub complete_candidate: bool,
    pub retirement_admitted: bool,
    pub next_generation: bool,
}

impl ArtifactStoreReplacementRefusal {
    pub(super) fn leg(self) -> ArtifactStoreReplacementLeg {
        match self {
            Self::Invariant { leg, .. } => leg,
            Self::InitializerAdmissionRejected | Self::InitializerFailed(_) => ArtifactStoreReplacementLeg::Initializer,
            Self::Cancelled | Self::MemberOpenRejected { .. } | Self::MemberIdentityMismatch { .. } | Self::MemberFactoryRefused { .. } => ArtifactStoreReplacementLeg::Members,
            Self::ClosureRejected { .. } | Self::ClosureMemberCount { .. } => ArtifactStoreReplacementLeg::Closure,
            Self::PublicationGuard(_) | Self::PublicationAuthorityRejected => ArtifactStoreReplacementLeg::RetainedPublication,
        }
    }

    pub fn slug(self) -> &'static str {
        match self {
            Self::Invariant { .. } => "invariant",
            Self::InitializerAdmissionRejected => "initializer-admission-rejected",
            Self::InitializerFailed(_) => "initializer-failed",
            Self::Cancelled => "cancelled",
            Self::MemberOpenRejected { .. } => "member-open-rejected",
            Self::MemberIdentityMismatch { .. } => "member-identity-mismatch",
            Self::MemberFactoryRefused { .. } => "member-factory-refused",
            Self::ClosureRejected { .. } => "closure-rejected",
            Self::ClosureMemberCount { .. } => "closure-member-count",
            Self::PublicationGuard(_) => "publication-guard",
            Self::PublicationAuthorityRejected => "publication-authority-rejected",
        }
    }

    pub fn detail(self) -> String {
        match self {
            Self::Invariant { detail, .. } => detail.to_string(),
            Self::InitializerAdmissionRejected => "the app's document-store initialization job was refused admission to the interactive worker lane".to_string(),
            Self::InitializerFailed(code) => format!("the app's document-store initialization job faulted or was cancelled before it produced a candidate, reporting `{}`; its `build_document_store_initialization_job` refused this envelope, or the envelope's schema/id/history failed that job's own validation", code.text()),
            Self::Cancelled => "the replacement was cancelled before its member set reached candidate readiness".to_string(),
            Self::MemberOpenRejected { ordinal, diagnostic } => format!("owned member {ordinal} was rejected by its typed open ({diagnostic:?})"),
            Self::MemberIdentityMismatch { ordinal, expected } => format!("owned member open returned ordinal {ordinal} while the sealed set expected {expected}, or its artifact/owner reference changed across the open"),
            Self::MemberFactoryRefused { ordinal } => format!("the app's member factory refused the admitted open request for ordinal {ordinal}"),
            Self::ClosureRejected { diagnostic } => format!("recursive ownership closure validation rejected the candidate ({diagnostic:?})"),
            Self::ClosureMemberCount { members, children } => format!("closure validation reached {members} members while the candidate registry owns {children}; the archive's member roster is not the parent's exact owned closure"),
            Self::PublicationGuard(guard) => format!(
                "the retained publication guard refused the ready candidate: closing={} cancelled={} parent-generation={} live-generation={} base-child-generation={} child-generation={} complete-candidate={} retirement-admitted={} next-generation={}",
                guard.closing,
                guard.cancelled,
                guard.parent_generation,
                guard.live_generation,
                guard.base_child_generation,
                guard.child_generation,
                guard.complete_candidate,
                guard.retirement_admitted,
                guard.next_generation,
            ),
            Self::PublicationAuthorityRejected => "the app's `validate_document_store_publication` refused the exact candidate".to_string(),
        }
    }

    /// 🧯️ The typed fault the host reports instead of the one-sentence collapse.
    pub fn into_fault(self) -> Fault {
        let leg = self.leg();
        Fault::new(FaultOrigin::Plugin, FaultCode::new(format!("plugin.internal.document-archive-replacement.{}", self.slug())), format!("document archive replacement failed its {} leg: {}", leg.slug(), self.detail()))
    }
}

#[doc(hidden)]
pub struct ActiveArtifactStoreReplacement<P, Mutation, M>
where
    P: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + semio_framework_schema_composition::ArtifactCompositionFields + Send + Sync + 'static,
    Mutation: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + store::Mutation<P> + OpBinary + OpText + Send + 'static,
    M: SpaceMember + MemberFactory,
{
    pub operation: semio_framework_job::OperationId,
    pub generation: semio_framework_job::Generation,
    pub base_child_content_generation: u64,
    pub cancel: ManuallyDrop<Option<semio_framework_job::CancelToken>>,
    pub cancel_signal: ManuallyDrop<Option<std::sync::Arc<std::sync::atomic::AtomicBool>>>,
    pub cancel_retirement: ManuallyDrop<Option<semio_framework_async::CancelTokenRetirement>>,
    pub session: ManuallyDrop<Option<semio_framework_job_worker::MountedWorkerJobSession<ArtifactStoreInitializationJob<P, Mutation>>>>,
    pub session_rejected: ManuallyDrop<Option<semio_framework_plugin_ownership::reserved_job_session::RefusedWorkerSource<ArtifactStoreInitializationJob<P, Mutation>>>>,
    pub outcome_pending: bool,
    pub(super) outcome_terminal: Option<(u8, ArtifactStoreRefusalCode)>,
    pub terminal_target: Option<ActiveArtifactStoreReplacementState>,
    pub resume_pending: bool,
    pub retained_store: ManuallyDrop<Option<Box<ArtifactStore<P, Mutation>>>>,
    pub retained_disposer: ManuallyDrop<ArtifactDisposal<ArtifactStore<P, Mutation>>>,
    pub member_ingress: ManuallyDrop<Option<OwnedDocumentMemberIngressRegistry>>,
    pub(super) active_member_ingress: ManuallyDrop<Option<OwnedDocumentMemberIngress>>,
    pub active_member_open: ManuallyDrop<Option<M::Open>>,
    pub(super) member_open_admission: ManuallyDrop<Option<store::MemberOpenSourceAdmission>>,
    pub(super) opened_member: ManuallyDrop<Option<M>>,
    pub candidate_children: ManuallyDrop<Option<ChildMemberRegistry<M>>>,
    pub candidate_content: ManuallyDrop<Option<ChildContentView>>,
    pub(super) candidate_content_retirement: ManuallyDrop<Option<ChildContentRetirement>>,
    pub displaced_content_retirements: ManuallyDrop<Option<ArtifactFixedRegistry<ChildContentRetirement>>>,
    pub(super) displaced_content_retirement_cursor: usize,
    pub candidate_composition: ManuallyDrop<Option<CompositionCoordinator>>,
    pub displaced_pins: ManuallyDrop<Option<CompositionPinsRetirement>>,
    pub(super) retiring_child: ManuallyDrop<Option<ChildMemberRetirement<M>>>,
    pub(super) retiring_child_cursor: usize,
    pub(super) next_member_ordinal: usize,
    pub view_member_ordinal: usize,
    pub candidate_source_generation: u64,
    pub(super) member_step_sequence: u64,
    pub expires_at_us: u64,
    pub closure: Option<store::OwnedDocumentClosure>,
    pub committed: bool,
    pub refusal: Option<ArtifactStoreReplacementRefusal>,
    /// 📈️ The initializer's last reported `(operations folded, operations to fold)` (gap N17), kept after its handoff.
    pub fold_progress: (u64, u64),
    pub state: ActiveArtifactStoreReplacementState,
}
