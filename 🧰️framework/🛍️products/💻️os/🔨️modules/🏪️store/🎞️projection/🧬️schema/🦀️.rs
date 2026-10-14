//! 🎞️ Versioned projection contracts independent of Store execution.
/// 🧬️ History identity for a projection input, independent of the process-local generation.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ArtifactRevision {
    pub artifact_id: String,
    pub schema: String,
    pub applied_edit_ids: semio_framework_os_vcs::HistoryPageStack<String>,
    pub redo_edit_ids: semio_framework_os_vcs::HistoryPageStack<String>,
    pub checkpoint_id: Option<String>,
}

/// 🎯️ Exact state identity a projection or inference result must match before it is accepted.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ArtifactProjectionStamp {
    pub revision: ArtifactRevision,
    pub generation: u64,
}

/// 🔄️ Store transition that produced a projection input.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ArtifactProjectionCause {
    Apply,
    RemoteIngest,
    Replay,
    Undo,
    Redo,
    Reset,
    Checkout,
    PolicyChange,
    ExternalResourceChange,
    Checkpoint,
    PruneDrafts,
}

/// 🗄️ Explicit prior-result handling; projections never infer cache authority from state alone.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ArtifactProjectionCacheMode {
    Rebuild,
    ReusePrevious,
    ValidatePrevious,
}

/// 📬️ Typed projection/inference input carrying the exact artifact version it observed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArtifactProjectionEvent<State, Previous, Policy> {
    pub stamp: ArtifactProjectionStamp,
    pub cause: ArtifactProjectionCause,
    pub state: State,
    pub previous: Option<Previous>,
    pub cache_mode: ArtifactProjectionCacheMode,
    pub policy: Policy,
}

/// 🧩️ Projection output plus an optional semantic-diff candidate. This layer validates freshness;
/// strict `ArtifactDiff` application stays owned by the semantic-mutation boundary.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArtifactProjectionResult<Output, Diff = ()> {
    pub stamp: ArtifactProjectionStamp,
    pub output: Output,
    pub proposed_diff: Option<Diff>,
}

impl<State, Previous, Policy> ArtifactProjectionEvent<State, Previous, Policy> {
    /// 🧪️ Couples an asynchronously computed result to this event's immutable stamp.
    pub async fn result<Output, Diff>(&self, output: Output, proposed_diff: Option<Diff>) -> ArtifactProjectionResult<Output, Diff> {
        ArtifactProjectionResult { stamp: self.stamp.clone(), output, proposed_diff }
    }
}

/// ✅️ A current projection result, intentionally separated from semantic-diff application.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AcceptedArtifactProjection<Output, Diff = ()> {
    pub output: Output,
    pub proposed_diff: Option<Diff>,
}

/// 🚫️ A result computed against an older state and therefore forbidden from changing current state.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StaleArtifactProjection {
    pub computed_for: ArtifactProjectionStamp,
    pub current: ArtifactProjectionStamp,
}

/// 📣️ One state or dependency transition that invalidates derived artifact work.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArtifactProjectionInvalidation {
    pub cause: ArtifactProjectionCause,
    pub stamp: ArtifactProjectionStamp,
}


/// 🧊️ Retires one SCRATCH projection a history fold displaced, through the technology's own diff
/// vocabulary. Every replay seam below walks `base → mid₁ → … → head` and throws each intermediate
/// away; an artifact whose projection owns a fail-closed root (an `OrderedMap`, a neural
/// `Dictionary`) aborts the process on that bare drop, which is what made `undo`/`redo` and every
/// `.pack`/`.spr` reload kill the app (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub fn retire_replayed_projection<P, Mutation>(projection: P)
where
    Mutation: protocol::Mutation<P>,
{
    <Mutation::Diff as protocol::MutationDiff<P>>::retire_projection(projection);
}

/// 🧊️ Releases one alias of a scratch projection: the last owner retires it ([`retire_replayed_projection`]), any
/// other only drops its count — the owner that outlives it retires it.
pub fn retire_shared_projection<P, Mutation>(projection: std::sync::Arc<P>)
where
    Mutation: protocol::Mutation<P>,
{
    if let Ok(projection) = std::sync::Arc::try_unwrap(projection) {
        retire_replayed_projection::<P, Mutation>(projection);
    }
}

