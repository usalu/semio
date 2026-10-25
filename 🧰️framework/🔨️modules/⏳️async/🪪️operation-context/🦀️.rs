//! 🪪️ `OperationContext`: the per-operation value carrying cancellation, deadline, trace, lane and capability.

use super::CancelToken;

//#region 🪪️OperationContext
/// 🔖️ Opaque per-operation trace correlation id — carried end to end through logs/metrics, never
/// interpreted by this crate. No `Serialize`/`Deserialize`: same repo-wide zero-call-site finding
/// as [`CancelState`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TraceId(pub u64);

/// 🔑️ Opaque handle to a capability grant held elsewhere (the concrete grant type is an
/// application-layer concern this framework-tier crate must not depend on — same seam discipline
/// `🎭️actor`'s `CapabilityGrant` doc records). Revoking the grant this id names is how a capability
/// revocation propagates into an in-flight [`OperationContext`]: the holder checks the id is still
/// live, it does not itself carry rights. No `Serialize`/`Deserialize`: same repo-wide
/// zero-call-site finding as [`CancelState`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CapabilityTokenId(pub u64);

/// ☁️ Carried by every async host operation so cancellation, deadlines, tracing, scheduling
/// priority (`lane`, mirroring `🎭️actor::Lane`'s discriminant order — kept as a bare `u8` here so
/// this crate never depends on the actor crate; [`Lane::from_context_lane`] converts it into this
/// crate's own [`Lane`] for [`WorkerPool::submit`]) and capability revocation all propagate through
/// the whole operation with one value. Deliberately NOT `Serialize`/`Deserialize`: [`CancelToken`] is
/// a live in-process handle (an `Arc`), not wire data — a context is passed by value within one host
/// process, never encoded to bytes.
#[derive(Clone)]
pub struct OperationContext {
    pub actor: u64,
    pub generation: u16,
    pub trace: TraceId,
    pub lane: u8,
    pub deadline_ms: Option<u64>,
    pub cancel: CancelToken,
    pub capability: Option<CapabilityTokenId>,
}

impl std::fmt::Debug for OperationContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OperationContext")
            .field("actor", &self.actor)
            .field("generation", &self.generation)
            .field("trace", &self.trace)
            .field("lane", &self.lane)
            .field("deadline_ms", &self.deadline_ms)
            .field("cancel", &self.cancel)
            .field("capability", &self.capability)
            .finish()
    }
}
//#endregion 🪪️OperationContext
