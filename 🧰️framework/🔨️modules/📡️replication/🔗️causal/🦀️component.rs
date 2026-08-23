//! 🎞️ Protocol causal layer: `MutationEnvelope`/`ArtifactDiff`/`InverseMutation`, the `MutationDag`
//! causal buffer, the runtime frontier-summary twin, the `MutationTransform` hook, and the
//! `mutation_envelope_from_edit` bridge from `crate::mutation::Edit`. Moved from
//! `framework/core/rs/lib.rs`'s `🔖️Sync` region (`MutationEnvelope` L6246, `ArtifactDiff` L6121,
//! `InverseMutation` L6137, `MutationDag`/`InsertResult`/`MutationDagError` L6266-6380 including its existing
//! unit tests at L6488-6572) and `vcs/rs/lib.rs`'s `mutation_envelope_from_edit`. Frozen contract:
//! `.🦑️repo/🎫️tickets/26/07/27/PROTOCOL-BINARY-OP-LOG-LAYER/contract.md` `## Amendment` §`protocol_causal`.
//!
//! This crate's `FrontierSummary`/`frontier_delta` are the runtime/wire twin of
//! `protocol_history`'s durable-log-derived pair — deliberately kept separate, see `🔖️Frontier`.

//#region 🔖️Envelope
// Moved from framework/core L6246 (MutationEnvelope), L6121 (ArtifactDiff), L6137
// (InverseMutation). The frozen contract's field shapes are simpler than the framework-core
// originals (no `schema_version`/`payload_hash` on the envelope, no `target_mutation`/
// `base_version`/`dependencies`/`undo_policy` on the inverse) — implemented exactly as specified
// below.
//
// 🎯️ W5: `payload`/`inverse_diff` flip from `serde_json::Value` to opaque `Vec<u8>` — the binary
// twin of an operation crossing the wire, matching M-C's "communication AND storage both binary"
// requirement. `payload` is the `crate::mutation::OpBinary` encoding of the op (or a
// producer-defined encoding named by `schema` for a non-typed-op payload, e.g. `db`'s pathmap
// convention); `schema` is a real `crate::ids::SchemaId`, no longer a `std::any::type_name`
// placeholder (see `🔖️Bridge` below). `InverseMutation.inverse_diff` is renamed to `payload` for
// the same reason `ArtifactDiff.payload` is named `payload`, not `diff` — both now hold the same
// kind of thing (an encoded op), not a structural diff. Both fields still carry
// `serde::Serialize`/`Deserialize` for the WIT/backbone JSON seam (a `Vec<u8>` serializes as a
// JSON number array there — acceptable by design, that seam stays JSON per M-C).

/// @emoji ✉️ A causally-ordered operation crossing the wire: identity, actor, dependency set, the
/// forward diff, its precomputed inverse, and the HLC tick it was authored at.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MutationEnvelope {
    pub mutation_id: crate::ids::MutationId,
    pub document_id: crate::ids::ArtifactId,
    pub actor: crate::ids::ActorId,
    pub dependencies: Vec<crate::ids::MutationId>,
    pub diff: ArtifactDiff,
    pub inverse: InverseMutation,
    pub timestamp: crate::ids::HybridLogicalTimestamp,
}

/// @emoji 🧮️ A schema-tagged, opaque binary forward-op payload.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ArtifactDiff {
    pub schema: crate::ids::SchemaId,
    pub payload: Vec<u8>,
}

/// @emoji ↩️ A schema-tagged, opaque binary inverse-op payload.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InverseMutation {
    pub schema: crate::ids::SchemaId,
    pub payload: Vec<u8>,
}
//#endregion 🔖️Envelope

//#region 🔖️MutationDag
// Moved verbatim from framework/core L6266-6379 including its existing unit tests (L6488-6572),
// field names adapted to the new `MutationEnvelope` shape (`id` -> `mutation_id`, `deps` ->
// `dependencies`). No behavior change, including the pre-existing quirk this port preserves
// faithfully: `insert`'s own per-envelope Applied/Pending classification treats a dependency as
// "not blocking" once it is merely *known* to the dag (present in `envelopes`, via any earlier
// Pending insert), not only once it is actually `applied` — see the inline comment on `insert`
// below. This never manifests for insertions performed in true topological order (every ancestor
// is already `applied`, not merely known, by induction), which is the property this crate's own
// `🧪️Tests::quick` convergence tests exercise; `protocol_testkit`'s exhaustive suite covers
// scrambled orderings.

/// @emoji 🕸️ Causal DAG of exchanged `MutationEnvelope`s: buffers envelopes until their
/// dependencies are applied.
pub const MUTATION_DAG_CAPACITY: usize = 8_192;
pub const MUTATION_DAG_IDENTIFIER_BYTES: usize = 256;

struct MutationDagFixedSlots<T> {
    slots: Box<[std::mem::MaybeUninit<T>]>,
    generations: Box<[u32]>,
    occupied: Box<[bool]>,
    next: Box<[u16]>,
    previous: Box<[u16]>,
    free: Box<[u16]>,
    free_len: usize,
    head: u16,
    tail: u16,
    len: usize,
}

const MUTATION_DAG_SLOT_NONE: u16 = u16::MAX;

struct MutationDagFixedSlotsIter<'a, T> {
    owner: &'a MutationDagFixedSlots<T>,
    next: u16,
}

impl<'a, T> Iterator for MutationDagFixedSlotsIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next == MUTATION_DAG_SLOT_NONE {
            return None;
        }
        let slot = usize::from(self.next);
        self.next = self.owner.next[slot];
        Some(unsafe { self.owner.slots[slot].assume_init_ref() })
    }
}

impl<T> MutationDagFixedSlots<T> {
    fn new() -> Self {
        let free = (0..MUTATION_DAG_CAPACITY).rev().map(|slot| slot as u16).collect::<Vec<_>>().into_boxed_slice();
        Self {
            slots: Box::<[T]>::new_uninit_slice(MUTATION_DAG_CAPACITY),
            generations: vec![0; MUTATION_DAG_CAPACITY].into_boxed_slice(),
            occupied: vec![false; MUTATION_DAG_CAPACITY].into_boxed_slice(),
            next: vec![MUTATION_DAG_SLOT_NONE; MUTATION_DAG_CAPACITY].into_boxed_slice(),
            previous: vec![MUTATION_DAG_SLOT_NONE; MUTATION_DAG_CAPACITY].into_boxed_slice(),
            free,
            free_len: MUTATION_DAG_CAPACITY,
            head: MUTATION_DAG_SLOT_NONE,
            tail: MUTATION_DAG_SLOT_NONE,
            len: 0,
        }
    }

    fn len(&self) -> usize {
        self.len
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn get(&self, index: usize) -> Option<&T> {
        self.iter().nth(index)
    }

    fn iter(&self) -> MutationDagFixedSlotsIter<'_, T> {
        MutationDagFixedSlotsIter { owner: self, next: self.head }
    }

    fn push(&mut self, value: T) -> Result<(), T> {
        if self.len == MUTATION_DAG_CAPACITY {
            return Err(value);
        }
        self.push_reserved(value);
        Ok(())
    }

    fn push_reserved(&mut self, value: T) {
        assert!(self.free_len > 0, "fixed causal slot reservation was not established");
        self.free_len -= 1;
        let slot = usize::from(self.free[self.free_len]);
        self.slots[slot].write(value);
        self.generations[slot] = self.generations[slot].wrapping_add(1).max(1);
        self.occupied[slot] = true;
        self.previous[slot] = self.tail;
        self.next[slot] = MUTATION_DAG_SLOT_NONE;
        if self.tail == MUTATION_DAG_SLOT_NONE {
            self.head = slot as u16;
        } else {
            self.next[usize::from(self.tail)] = slot as u16;
        }
        self.tail = slot as u16;
        self.len += 1;
    }

    fn pop(&mut self) -> Option<T> {
        (self.tail != MUTATION_DAG_SLOT_NONE).then(|| self.remove_slot(self.tail))
    }

    fn swap_remove(&mut self, index: usize) -> Option<T> {
        let mut ticket = self.head;
        for _ in 0..index {
            if ticket == MUTATION_DAG_SLOT_NONE {
                return None;
            }
            ticket = self.next[usize::from(ticket)];
        }
        (ticket != MUTATION_DAG_SLOT_NONE).then(|| self.remove_slot(ticket))
    }

    fn remove_slot(&mut self, ticket: u16) -> T {
        let slot = usize::from(ticket);
        assert!(self.occupied[slot], "fixed causal generation ticket addressed a vacant slot");
        let previous = self.previous[slot];
        let next = self.next[slot];
        if previous == MUTATION_DAG_SLOT_NONE {
            self.head = next;
        } else {
            self.next[usize::from(previous)] = next;
        }
        if next == MUTATION_DAG_SLOT_NONE {
            self.tail = previous;
        } else {
            self.previous[usize::from(next)] = previous;
        }
        self.occupied[slot] = false;
        self.next[slot] = MUTATION_DAG_SLOT_NONE;
        self.previous[slot] = MUTATION_DAG_SLOT_NONE;
        self.generations[slot] = self.generations[slot].wrapping_add(1).max(1);
        self.free[self.free_len] = ticket;
        self.free_len += 1;
        self.len -= 1;
        unsafe { self.slots[slot].assume_init_read() }
    }
}

impl<T: Clone> Clone for MutationDagFixedSlots<T> {
    fn clone(&self) -> Self {
        let mut clone = Self::new();
        for value in self.iter() {
            clone.push_reserved(value.clone());
        }
        clone
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for MutationDagFixedSlots<T> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_list().entries(self.iter()).finish()
    }
}

impl<T: PartialEq> PartialEq for MutationDagFixedSlots<T> {
    fn eq(&self, other: &Self) -> bool {
        self.iter().eq(other.iter())
    }
}

impl<T> Drop for MutationDagFixedSlots<T> {
    fn drop(&mut self) {
        assert!(self.is_empty(), "fixed causal slots reached Drop before every exact nested owner was detached");
    }
}

#[derive(Debug, PartialEq)]
pub struct MutationDag {
    envelopes: MutationDagFixedSlots<MutationEnvelope>,
    applied: MutationDagFixedSlots<String>,
    drained: usize,
    pending: MutationDagFixedSlots<String>,
}

impl Default for MutationDag {
    fn default() -> Self {
        Self { envelopes: MutationDagFixedSlots::new(), applied: MutationDagFixedSlots::new(), drained: 0, pending: MutationDagFixedSlots::new() }
    }
}

impl Clone for MutationDag {
    fn clone(&self) -> Self {
        Self { envelopes: self.envelopes.clone(), applied: self.applied.clone(), drained: self.drained, pending: self.pending.clone() }
    }
}

impl Drop for MutationDag {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "mutation dag reached Drop before every exact envelope and identity owner was cursor-retired");
    }
}

/// @emoji 🚦️ The outcome of one `MutationDag::insert` call.
#[derive(Debug, PartialEq)]
pub enum InsertResult {
    Applied,
    Pending,
    AlreadyApplied(MutationEnvelope),
}

/// @emoji 🚨️ `MutationDag`'s one failure mode: the same operation id inserted twice while still pending.
/// Hand-rolled `Display`/`Error` (this crate has no `thiserror` dependency — `protocol_core`/
/// `protocol_command` are the only path deps).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MutationDagError {
    Duplicate,
    Capacity,
    IdentifierTooLong,
}

impl std::fmt::Display for MutationDagError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MutationDagError::Duplicate => write!(f, "duplicate mutation id"),
            MutationDagError::Capacity => write!(f, "mutation dag fixed capacity exhausted"),
            MutationDagError::IdentifierTooLong => write!(f, "mutation dag identifier exceeds its fixed byte authority"),
        }
    }
}

impl std::error::Error for MutationDagError {}

#[derive(Debug, PartialEq)]
pub struct MutationDagInsertRejected {
    pub error: MutationDagError,
    pub envelope: MutationEnvelope,
}

impl std::fmt::Display for MutationDagInsertRejected {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.error, formatter)
    }
}

#[derive(Debug, PartialEq)]
pub struct MutationDagSeedRejected {
    pub error: MutationDagError,
    pub mutation_id: crate::ids::MutationId,
}

impl std::fmt::Display for MutationDagSeedRejected {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.error, formatter)
    }
}

pub enum MutationDagCloseOwner {
    Envelope(MutationEnvelope),
    Identity(String),
}

pub enum MutationDagAppliedStep {
    Envelope(MutationEnvelope),
    SeededIdentity,
    Complete,
}

impl MutationDag {
    pub fn new() -> Self {
        Self::default()
    }

    /// 🧹 Proves the causal owner shell contains no mutation, identity, or ordering allocation.
    pub fn terminal_is_empty(&self) -> bool {
        self.envelopes.is_empty() && self.applied.is_empty() && self.pending.is_empty()
    }

    /// @emoji ➕️ Inserts one envelope. Returns `AlreadyApplied` if its id was applied before,
    /// `Err(Duplicate)` if it's already buffered as pending, `Pending` if any dependency is wholly
    /// unknown to this dag, else `Applied` (and cascades `drain_ready` for anything it unblocks).
    pub fn insert(&mut self, envelope: MutationEnvelope) -> Result<InsertResult, MutationDagInsertRejected> {
        let id = envelope.mutation_id.0.as_str();
        if id.len() > MUTATION_DAG_IDENTIFIER_BYTES || envelope.dependencies.iter().any(|dependency| dependency.0.len() > MUTATION_DAG_IDENTIFIER_BYTES) {
            return Err(MutationDagInsertRejected { error: MutationDagError::IdentifierTooLong, envelope });
        }
        if self.applied.iter().any(|applied| applied == id) {
            return Ok(InsertResult::AlreadyApplied(envelope));
        }
        if self.envelopes.iter().any(|known| known.mutation_id.0 == id) {
            return Err(MutationDagInsertRejected { error: MutationDagError::Duplicate, envelope });
        }
        let seeded_only = self.applied.iter().filter(|applied| !self.envelopes.iter().any(|known| known.mutation_id.0.as_str() == applied.as_str())).count();
        if self.envelopes.len() + seeded_only == MUTATION_DAG_CAPACITY {
            return Err(MutationDagInsertRejected { error: MutationDagError::Capacity, envelope });
        }
        let pending = envelope.dependencies.iter().any(|dependency| !self.applied.iter().any(|applied| applied == &dependency.0) && !self.envelopes.iter().any(|known| known.mutation_id.0 == dependency.0));
        let id = envelope.mutation_id.0.clone();
        self.envelopes.push_reserved(envelope);
        if pending {
            self.pending.push_reserved(id);
            return Ok(InsertResult::Pending);
        }
        self.mark_applied(&id);
        self.advance_ready_one();
        Ok(InsertResult::Applied)
    }

    /// @emoji ✅️ Borrows one ready identity at a caller-owned cursor without materializing a list.
    pub fn ready_identity_at(&self, cursor: usize) -> Option<&str> {
        let id = self.pending.get(cursor)?;
        self.envelopes.iter().find(|envelope| envelope.mutation_id.0 == *id).filter(|envelope| envelope.dependencies.iter().all(|dependency| self.applied.iter().any(|applied| applied == &dependency.0))).map(|envelope| envelope.mutation_id.0.as_str())
    }

    /// @emoji 🧺️ Transfers at most one exact applied owner at the retained drain cursor.
    pub fn take_next_applied(&mut self) -> MutationDagAppliedStep {
        let Some(id) = self.applied.get(self.drained) else { return MutationDagAppliedStep::Complete };
        self.drained += 1;
        let Some(index) = self.envelopes.iter().position(|envelope| envelope.mutation_id.0 == *id) else {
            return MutationDagAppliedStep::SeededIdentity;
        };
        MutationDagAppliedStep::Envelope(self.envelopes.swap_remove(index).expect("validated applied envelope slot remains occupied"))
    }

    /// @emoji 🌱️ Seeds one id into the applied-set from out-of-band knowledge (e.g. a full-document
    /// snapshot merge) — without this, a later envelope whose `dependencies` reference this id
    /// stays `Pending` forever, since `insert` only recognizes a dependency as satisfied through
    /// this dag's own `envelopes`/`applied` bookkeeping, never through edits a peer adopted by some
    /// other route.
    pub fn seed_applied(&mut self, mutation_id: crate::ids::MutationId) -> Result<(), MutationDagSeedRejected> {
        if mutation_id.0.len() > MUTATION_DAG_IDENTIFIER_BYTES {
            return Err(MutationDagSeedRejected { error: MutationDagError::IdentifierTooLong, mutation_id });
        }
        if self.applied.iter().any(|applied| applied == &mutation_id.0) {
            return Err(MutationDagSeedRejected { error: MutationDagError::Duplicate, mutation_id });
        }
        let unique_seed = !self.envelopes.iter().any(|envelope| envelope.mutation_id == mutation_id);
        let seeded_only = self.applied.iter().filter(|id| !self.envelopes.iter().any(|envelope| envelope.mutation_id.0.as_str() == id.as_str())).count();
        if unique_seed && self.envelopes.len() + seeded_only == MUTATION_DAG_CAPACITY {
            return Err(MutationDagSeedRejected { error: MutationDagError::Capacity, mutation_id });
        }
        self.mark_applied(&mutation_id.0);
        Ok(())
    }

    pub fn take_one_close_owner(&mut self) -> Option<MutationDagCloseOwner> {
        if let Some(id) = self.pending.pop() {
            return Some(MutationDagCloseOwner::Identity(id));
        }
        if let Some(id) = self.applied.pop() {
            self.drained = self.drained.min(self.applied.len());
            return Some(MutationDagCloseOwner::Identity(id));
        }
        self.envelopes.pop().map(MutationDagCloseOwner::Envelope)
    }

    fn mark_applied(&mut self, id: &str) {
        if let Some(index) = self.pending.iter().position(|pending| pending == id) {
            let pending = self.pending.swap_remove(index).expect("validated pending causal identity remains occupied");
            self.applied.push_reserved(pending);
        } else {
            self.applied.push_reserved(id.to_string());
        }
    }

    pub fn advance_ready_one(&mut self) -> bool {
        let ready = self
            .pending
            .iter()
            .find(|id| self.envelopes.iter().find(|envelope| envelope.mutation_id.0.as_str() == id.as_str()).is_some_and(|envelope| envelope.dependencies.iter().all(|dependency| self.applied.iter().any(|applied| applied == &dependency.0))))
            .cloned();
        let Some(id) = ready else { return false };
        self.mark_applied(&id);
        true
    }
}
//#endregion 🔖️MutationDag

//#region 🔖️Frontier
/// @emoji 🏔️ Runtime/wire twin of `os_spr::history::FrontierSummary` — the shape `db` and
/// `framework/sync` exchange without a full history-log decode. Deliberately NOT unified with the
/// durable-log-derived version: they serve different layers (live runtime state vs on-disk log).
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FrontierSummary {
    pub document_id: crate::ids::ArtifactId,
    pub head_edit_ordinal: u64,
    pub head_edit_id: String,
    pub last_commit_seq: u64,
    pub chain_hash: [u8; 32],
}

/// @emoji ⚖️ How a `local` frontier relates to a `remote` one.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum FrontierComparison {
    Equal,
    Ahead,
    Behind,
    Diverged { common_edit_count: u64 },
}

/// @emoji 🔎️ Compares two frontier summaries. Design choice (the contract fixes the enum shape,
/// not the comparison algorithm): identical `(head_edit_ordinal, head_edit_id, chain_hash)` is
/// `Equal`; a strictly greater/lesser `head_edit_ordinal` alone is `Ahead`/`Behind` (a summary
/// carries no ancestry chain to verify beyond its tip, so ordinal order is the only signal
/// available at this layer); equal ordinal with a differing `head_edit_id`/`chain_hash` is
/// `Diverged`, with `common_edit_count` conservatively reported as the shared ordinal floor
/// (`min` of both ordinals) since this summary-only comparison cannot walk history to find the
/// true common ancestor — callers wanting an exact count must consult the durable log via
/// `protocol_history`.
pub fn frontier_delta(local: &FrontierSummary, remote: &FrontierSummary) -> FrontierComparison {
    if local.head_edit_ordinal == remote.head_edit_ordinal && local.head_edit_id == remote.head_edit_id && local.chain_hash == remote.chain_hash {
        return FrontierComparison::Equal;
    }
    if local.head_edit_ordinal > remote.head_edit_ordinal {
        return FrontierComparison::Ahead;
    }
    if local.head_edit_ordinal < remote.head_edit_ordinal {
        return FrontierComparison::Behind;
    }
    FrontierComparison::Diverged { common_edit_count: local.head_edit_ordinal.min(remote.head_edit_ordinal) }
}
//#endregion 🔖️Frontier

//#region 🔖️Transform
/// @emoji 🔀️ The result of transforming one operation against a concurrent one.
#[derive(Clone, Debug, PartialEq)]
pub enum TransformOutcome<Op> {
    Unchanged(Op),
    Transformed(Op),
    Conflict(String),
}

/// @emoji 🧮️ Operational-transform hook: rewrites `self` so it applies cleanly after `against`
/// (both assumed concurrent, same base). New trait — no prior `vcs`/`framework-core` equivalent.
pub trait MutationTransform<P>: crate::mutation::Mutation<P> {
    fn transform(&self, against: &Self) -> TransformOutcome<Self>
    where
        Self: Sized;
}
//#endregion 🔖️Transform

//#region 🔖️Bridge
// Moved from vcs/rs (was mutation_envelope_from_edit). The original signature took a
// `ArtifactEnvelope<P, Mutation>` (for its `.id`/`.schema`) and a `deps: Vec<MutationId>` and
// returned a single `Result<MutationEnvelope, VcsError>` whose diff/inverse payloads were the
// *whole* `Edit` serialized once. The frozen contract's signature drops both the vcs envelope and
// the `deps` parameter and returns `Vec<MutationEnvelope>` — one envelope per forward op — which
// only works because `Op: crate::mutation::Mutation<P>` supplies each op's own
// `mutation_id`/`dependencies`/`author_id`/`timestamp` via trait methods, no base `P` needed.
//
// 🎯️ W5: payloads flip from `serde_json::to_value` to `OpBinary::encode_op` (new `Op: OpBinary`
// bound — every real op type has had this since W2's derive flip), so the function becomes
// fallible (`Result<Vec<MutationEnvelope>, ProtocolError>`, one encode failure aborts the whole
// batch — an op that can't encode is a hard error, not a partial envelope). `schema` is now a
// caller-supplied real `crate::ids::SchemaId` (new parameter) instead of
// `std::any::type_name::<Op>()` — the type-name placeholder was never a stable/meaningful tag
// across a process boundary; callers already know their document's schema string (it's what they
// register a `ArtifactCodec` under). `inverse.payload` is an empty `Vec<u8>` past the end of
// `edit.inverse` (was `Value::Null`) — still the same "shorter inverse vec is not an error"
// contract, just spelled in the new payload type.
//
// 🎯️ Design choices (genuine ambiguity the contract leaves to the implementer, unchanged from the
// original wave): `edit.forwards` is zipped index-wise with `edit.mutation_meta` (the richer,
// already-computed per-op metadata a live appender fills in) with a documented fallback chain:
// `mutation_meta[i]` field, else the `Op` trait method, else a structural default
// (`{edit.id}#{i}` for the id, `edit.actor` or `"unknown"` for the actor,
// `HybridLogicalTimestamp::new(0, 0).await` for the timestamp) so this function is total (modulo encode
// failure) even for a bare-bones `Edit` with no explicit meta.
/// @emoji 🪪️ The wire `MutationId` each of `edit.forwards` would get if fanned out through
/// `mutation_envelope_from_edit` — same fallback chain (`mutation_meta[i]` field, else the `Op`
/// trait method, else `{edit.id}#{i}`), extracted so callers that only need identity (e.g.
/// snapshot-vs-operations-message dedup) don't have to pay for `encode_op`/`inverse` work, and so
/// there is exactly one place this chain is spelled out.
pub fn mutation_ids_for_edit<P, Op: crate::mutation::Mutation<P>>(edit: &crate::mutation::Edit<Op>) -> Vec<crate::ids::MutationId> {
    let mut out = Vec::with_capacity(edit.forwards.len());
    for (index, op) in edit.forwards.iter().enumerate() {
        let id = match edit.mutation_meta.get(index).and_then(|m| m.mutation_id.clone()) {
            Some(id) => id,
            None => match op.mutation_id() {
                Some(id) => id,
                None => crate::ids::MutationId(format!("{}#{index}", edit.id)),
            },
        };
        out.push(id);
    }
    out
}

pub fn mutation_envelope_from_edit<P, Op: crate::mutation::Mutation<P> + crate::mutation::OpBinary>(
    edit: &crate::mutation::Edit<Op>,
    document_id: &crate::ids::ArtifactId,
    schema: &crate::ids::SchemaId,
) -> Result<Vec<MutationEnvelope>, crate::ProtocolError> {
    let operation_ids = mutation_ids_for_edit(edit);
    let mut out = Vec::with_capacity(edit.forwards.len());
    for (index, op) in edit.forwards.iter().enumerate() {
        let meta = edit.mutation_meta.get(index);
        let mutation_id = operation_ids[index].clone();
        let dependencies = match meta {
            Some(m) => m.dependencies.clone(),
            None => op.dependencies(),
        };
        let actor = match meta.and_then(|m| m.author_id.clone()) {
            Some(actor) => actor,
            None => match op.author_id() {
                Some(actor) => actor,
                None => crate::ids::ActorId(edit.actor.clone().unwrap_or_else(|| "unknown".to_string())),
            },
        };
        let timestamp = match meta.map(|m| m.timestamp) {
            Some(ts) => ts,
            None => match op.timestamp() {
                Some(ts) => ts,
                None => crate::ids::HybridLogicalTimestamp::new(0, 0),
            },
        };
        let payload = op.encode_op()?;
        let inverse_payload = match edit.inverse.get(index) {
            Some(inv) => crate::mutation::OpBinary::encode_op(inv)?,
            None => Vec::new(),
        };
        out.push(MutationEnvelope {
            mutation_id,
            document_id: document_id.clone(),
            actor,
            dependencies,
            diff: ArtifactDiff { schema: schema.clone(), payload },
            inverse: InverseMutation { schema: schema.clone(), payload: inverse_payload },
            timestamp,
        });
    }
    Ok(out)
}
//#endregion 🔖️Bridge

//#region 🔖️EnvelopeCodec
/// @emoji 🎞️ Binary record codec for `MutationEnvelope`/`FrontierSummary`, built on
/// `crate::wire::🔖️WireCodec`'s primitives — the storage/wire form `protocol_wire`'s frames
/// embed and `db_sync`'s WAL uses directly (see the amendment's "storage AND communication both
/// binary" requirement). Field declaration order, no tags — the same convention `os_dsl::op_rt` and
/// `crate::wire::WireCodec` both use.
fn encode_hlc(out: &mut Vec<u8>, hlt: &crate::ids::HybridLogicalTimestamp) {
    crate::wire::write_varint_u64(out, hlt.actor);
    crate::wire::write_varint_u64(out, hlt.physical_ms);
    crate::wire::write_varint_u64(out, hlt.logical);
}

fn decode_hlc(bytes: &[u8], pos: &mut usize) -> Result<crate::ids::HybridLogicalTimestamp, crate::ProtocolError> {
    let actor = crate::wire::read_varint_u64(bytes, pos)?;
    let physical_ms = crate::wire::read_varint_u64(bytes, pos)?;
    let logical = crate::wire::read_varint_u64(bytes, pos)?;
    Ok(crate::ids::HybridLogicalTimestamp { actor, physical_ms, logical })
}

/// @emoji 🎯️ `mutation_id str | document_id str | actor str | dependencies vec<str> |
/// diff.schema str | diff.payload bytes | inverse.schema str | inverse.payload bytes | hlc`.
pub fn encode_envelope(envelope: &MutationEnvelope, out: &mut Vec<u8>) {
    crate::write_str(out, &envelope.mutation_id.0);
    crate::write_str(out, &envelope.document_id.0);
    crate::write_str(out, &envelope.actor.0);
    crate::wire::write_varint_u64(out, envelope.dependencies.len() as u64);
    for dependency in &envelope.dependencies {
        crate::write_str(out, &dependency.0);
    }
    crate::write_str(out, &envelope.diff.schema.0);
    crate::write_bytes(out, &envelope.diff.payload);
    crate::write_str(out, &envelope.inverse.schema.0);
    crate::write_bytes(out, &envelope.inverse.payload);
    encode_hlc(out, &envelope.timestamp);
}

/// @emoji 🎯️ Inverse of [`encode_envelope`].
pub fn decode_envelope(bytes: &[u8], pos: &mut usize) -> Result<MutationEnvelope, crate::ProtocolError> {
    let mutation_id = crate::ids::MutationId(crate::read_str(bytes, pos)?);
    let document_id = crate::ids::ArtifactId(crate::read_str(bytes, pos)?);
    let actor = crate::ids::ActorId(crate::read_str(bytes, pos)?);
    let dependency_count = crate::wire::read_varint_u64(bytes, pos)?;
    let mut dependencies = Vec::with_capacity(dependency_count as usize);
    for _ in 0..dependency_count {
        dependencies.push(crate::ids::MutationId(crate::read_str(bytes, pos)?));
    }
    let diff_schema = crate::ids::SchemaId(crate::read_str(bytes, pos)?);
    let diff_payload = crate::read_bytes(bytes, pos)?;
    let inverse_schema = crate::ids::SchemaId(crate::read_str(bytes, pos)?);
    let inverse_payload = crate::read_bytes(bytes, pos)?;
    let timestamp = decode_hlc(bytes, pos)?;
    Ok(MutationEnvelope { mutation_id, document_id, actor, dependencies, diff: ArtifactDiff { schema: diff_schema, payload: diff_payload }, inverse: InverseMutation { schema: inverse_schema, payload: inverse_payload }, timestamp })
}

/// @emoji 🎯️ `document_id str | head_edit_ordinal varint | head_edit_id str | last_commit_seq
/// varint | chain_hash 32`.
pub fn encode_frontier(f: &FrontierSummary, out: &mut Vec<u8>) {
    crate::write_str(out, &f.document_id.0);
    crate::wire::write_varint_u64(out, f.head_edit_ordinal);
    crate::write_str(out, &f.head_edit_id);
    crate::wire::write_varint_u64(out, f.last_commit_seq);
    crate::write_hash32(out, &f.chain_hash);
}

/// @emoji 🎯️ Inverse of [`encode_frontier`].
pub fn decode_frontier(bytes: &[u8], pos: &mut usize) -> Result<FrontierSummary, crate::ProtocolError> {
    let document_id = crate::ids::ArtifactId(crate::read_str(bytes, pos)?);
    let head_edit_ordinal = crate::wire::read_varint_u64(bytes, pos)?;
    let head_edit_id = crate::read_str(bytes, pos)?;
    let last_commit_seq = crate::wire::read_varint_u64(bytes, pos)?;
    let chain_hash = crate::read_hash32(bytes, pos)?;
    Ok(FrontierSummary { document_id, head_edit_ordinal, head_edit_id, last_commit_seq, chain_hash })
}

/// @emoji 🎯️ `count varint | encode_envelope each` — for boundaries that move a whole batch of
/// envelopes as one opaque byte blob (the WIT ABI, worker frames) instead of one wire frame per
/// envelope (`ClientFrame::Commands`, which already carries `Vec<MutationEnvelope>` typed).
pub fn encode_envelopes(envelopes: &[MutationEnvelope]) -> Vec<u8> {
    let mut out = Vec::new();
    crate::wire::write_varint_u64(&mut out, envelopes.len() as u64);
    for envelope in envelopes {
        encode_envelope(envelope, &mut out);
    }
    out
}

/// @emoji 🎯️ Inverse of [`encode_envelopes`].
pub fn decode_envelopes(bytes: &[u8]) -> Result<Vec<MutationEnvelope>, crate::ProtocolError> {
    let mut pos = 0usize;
    let count = crate::wire::read_varint_u64(bytes, &mut pos)?;
    let mut envelopes = Vec::with_capacity(count as usize);
    for _ in 0..count {
        envelopes.push(decode_envelope(bytes, &mut pos)?);
    }
    Ok(envelopes)
}

/// @emoji 🎯️ `count varint | (len varint | bytes) each` — a binary vec-of-op-payloads framing,
/// replacing the `serde_json::json!({"inverse": [...]})` convention for `InverseMutation`
/// payloads that carry more than one composed op (e.g. framework/plugin's `result_from_last_edit`).
pub fn encode_ops_vec(ops: &[Vec<u8>]) -> Vec<u8> {
    let mut out = Vec::new();
    crate::wire::write_varint_u64(&mut out, ops.len() as u64);
    for op in ops {
        crate::write_bytes(&mut out, op);
    }
    out
}

/// @emoji 🎯️ Inverse of [`encode_ops_vec`].
pub fn decode_ops_vec(bytes: &[u8]) -> Result<Vec<Vec<u8>>, crate::ProtocolError> {
    let mut pos = 0usize;
    let count = crate::wire::read_varint_u64(bytes, &mut pos)?;
    let mut ops = Vec::with_capacity(count as usize);
    for _ in 0..count {
        ops.push(crate::read_bytes(bytes, &mut pos)?);
    }
    Ok(ops)
}
//#endregion 🔖️EnvelopeCodec

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    //#region 🧸️Fixtures
    // Dummy (P=i64, Op=CausalAddOp) pair: the smallest possible Mutation/MutationDiff impl,
    // reused across this file's tests instead of a real technology's op set.
    #[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
    struct CausalAddDiff {
        delta: i64,
    }
    impl crate::mutation::MutationDiff<i64> for CausalAddDiff {
        fn apply(&self, base: &i64) -> crate::mutation::MutationApplyResult<i64> {
            Ok(base + self.delta)
        }
        fn absorb(&mut self, other: Self) {
            self.delta += other.delta;
        }
    }

    #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    struct CausalAddOp {
        delta: i64,
    }
    impl crate::mutation::Mutation<i64> for CausalAddOp {
        type Diff = CausalAddDiff;
        fn diff(&self, _base: &i64) -> crate::mutation::MutationOutcome<CausalAddDiff> {
            crate::mutation::MutationOutcome::new(CausalAddDiff { delta: self.delta })
        }
        fn inverse(&self, _base: &i64) -> Vec<Self> {
            vec![CausalAddOp { delta: -self.delta }]
        }
    }
    /// @emoji 🎯️ Hand-written (no `os_dsl::DslOps` derive in this dependency-free fixture): `format
    /// u8 (=1) | delta i64 LE`.
    impl crate::mutation::OpBinary for CausalAddOp {
        fn encode_op(&self) -> Result<Vec<u8>, crate::ProtocolError> {
            let mut out = vec![1u8];
            out.extend_from_slice(&self.delta.to_le_bytes());
            Ok(out)
        }
        fn decode_op(bytes: &[u8]) -> Result<Self, crate::ProtocolError> {
            if bytes.len() != 9 || bytes[0] != 1 {
                return Err(crate::ProtocolError::Malformed { what: "causal add op", offset: 0, detail: "expected 9 bytes, format 1".to_string() });
            }
            let mut delta_bytes = [0u8; 8];
            delta_bytes.copy_from_slice(&bytes[1..9]);
            Ok(CausalAddOp { delta: i64::from_le_bytes(delta_bytes) })
        }
    }
    impl MutationTransform<i64> for CausalAddOp {
        fn transform(&self, against: &Self) -> TransformOutcome<Self> {
            if self.delta == against.delta {
                TransformOutcome::Unchanged(self.clone())
            } else if self.delta == 0 {
                TransformOutcome::Conflict("zero delta cannot transform".to_string())
            } else {
                TransformOutcome::Transformed(CausalAddOp { delta: self.delta + against.delta })
            }
        }
    }

    fn sample_envelope(id: &str, deps: Vec<&str>) -> MutationEnvelope {
        MutationEnvelope {
            mutation_id: crate::ids::MutationId(id.into()),
            document_id: crate::ids::ArtifactId("document-1".into()),
            actor: crate::ids::ActorId("actor-1".into()),
            dependencies: deps.into_iter().map(|dep| crate::ids::MutationId(dep.into())).collect(),
            diff: ArtifactDiff { schema: crate::ids::SchemaId("diff.v1".into()), payload: id.as_bytes().to_vec() },
            inverse: InverseMutation { schema: crate::ids::SchemaId("diff.v1".into()), payload: Vec::new() },
            timestamp: crate::ids::HybridLogicalTimestamp::new(1, 0),
        }
    }

    fn take_applied(dag: &mut MutationDag) -> Vec<MutationEnvelope> {
        let mut envelopes = Vec::new();
        loop {
            match dag.take_next_applied() {
                MutationDagAppliedStep::Envelope(envelope) => envelopes.push(envelope),
                MutationDagAppliedStep::SeededIdentity => {}
                MutationDagAppliedStep::Complete => return envelopes,
            }
        }
    }

    fn retire_dag_shell(dag: &mut MutationDag) {
        while dag.take_one_close_owner().is_some() {}
        assert!(dag.terminal_is_empty());
    }
    //#endregion 🧸️Fixtures

    //#region 🔖️Envelope
    #[test]
    fn operation_envelope_binary_round_trips() {
        let envelope = sample_envelope("operation-1", vec!["operation-0"]);
        let mut out = Vec::new();
        encode_envelope(&envelope, &mut out);
        let mut pos = 0;
        let round_tripped = decode_envelope(&out, &mut pos).expect("decode");
        assert_eq!(round_tripped, envelope);
    }
    //#endregion 🔖️Envelope

    //#region 🔖️MutationDag
    #[test]
    fn inserts_pending_until_dependencies_arrive() {
        let mut dag = MutationDag::new();
        assert_eq!(dag.insert(sample_envelope("operation-2", vec!["operation-1"])).unwrap(), InsertResult::Pending);
        assert_eq!(dag.insert(sample_envelope("operation-1", vec![])).unwrap(), InsertResult::Applied);
        assert_eq!(dag.applied.len(), 2);
        retire_dag_shell(&mut dag);
    }

    #[test]
    fn drains_applied_envelopes_in_causal_order() {
        let mut dag = MutationDag::new();
        dag.insert(sample_envelope("operation-2", vec!["operation-1"])).unwrap();
        dag.insert(sample_envelope("operation-1", vec![])).unwrap();
        let drained = take_applied(&mut dag);
        assert_eq!(drained.iter().map(|envelope| envelope.mutation_id.0.clone()).collect::<Vec<_>>(), vec!["operation-1".to_string(), "operation-2".to_string()]);
        assert!(take_applied(&mut dag).is_empty(), "second drain yields nothing new");
        dag.insert(sample_envelope("operation-3", vec![])).unwrap();
        let drained = take_applied(&mut dag);
        assert_eq!(drained.len(), 1);
        assert_eq!(drained[0].mutation_id.0, "operation-3");
        retire_dag_shell(&mut dag);
    }

    #[test]
    fn insert_duplicate_pending_operation_id_errors() {
        let mut dag = MutationDag::new();
        dag.insert(sample_envelope("operation-2", vec!["operation-1"])).unwrap();
        let err = dag.insert(sample_envelope("operation-2", vec!["operation-1"])).unwrap_err();
        assert_eq!(err.error, MutationDagError::Duplicate);
        assert_eq!(err.envelope.mutation_id.0, "operation-2");
        retire_dag_shell(&mut dag);
    }

    #[test]
    fn insert_already_applied_operation_returns_already_applied_without_erroring() {
        let mut dag = MutationDag::new();
        dag.insert(sample_envelope("operation-1", vec![])).unwrap();
        let result = dag.insert(sample_envelope("operation-1", vec![])).unwrap();
        assert!(matches!(result, InsertResult::AlreadyApplied(envelope) if envelope.mutation_id.0 == "operation-1"));
        retire_dag_shell(&mut dag);
    }

    #[test]
    fn seed_applied_unblocks_pending_envelopes_that_reference_out_of_band_deps() {
        let mut dag = MutationDag::new();
        assert_eq!(dag.insert(sample_envelope("operation-2", vec!["operation-1"])).unwrap(), InsertResult::Pending);
        assert!(dag.ready_identity_at(0).is_none(), "dependency is not yet known to this dag");
        dag.seed_applied(crate::ids::MutationId("operation-1".to_string())).unwrap();
        assert_eq!(dag.ready_identity_at(0), Some("operation-2"));
        retire_dag_shell(&mut dag);
    }

    #[test]
    fn duplicate_seed_returns_the_exact_unadopted_identity_owner() {
        let mut dag = MutationDag::new();
        dag.seed_applied(crate::ids::MutationId("operation-1".to_string())).unwrap();
        let rejected = dag.seed_applied(crate::ids::MutationId("operation-1".to_string())).expect_err("duplicate seed must not consume its exact owner");
        assert_eq!(rejected.error, MutationDagError::Duplicate);
        assert_eq!(rejected.mutation_id.0, "operation-1");
        retire_dag_shell(&mut dag);
    }

    #[test]
    fn fixed_slot_free_ring_reuses_generation_without_reordering_live_owners() {
        let mut slots = MutationDagFixedSlots::new();
        slots.push("first".to_string()).unwrap();
        slots.push("second".to_string()).unwrap();
        let first_slot = usize::from(slots.head);
        let first_generation = slots.generations[first_slot];
        assert_eq!(slots.swap_remove(0).as_deref(), Some("first"));
        slots.push("third".to_string()).unwrap();
        assert_eq!(usize::from(slots.tail), first_slot, "LIFO free ring reuses the exact detached slot");
        assert_ne!(slots.generations[first_slot], first_generation, "reused slot advances its ABA generation");
        assert_eq!(slots.iter().map(String::as_str).collect::<Vec<_>>(), vec!["second", "third"], "live traversal remains deterministic insertion order");
        assert_eq!(slots.pop().as_deref(), Some("third"));
        assert_eq!(slots.pop().as_deref(), Some("second"));
        assert!(slots.is_empty());
    }

    #[test]
    fn opdagerror_display_is_non_empty() {
        assert!(!MutationDagError::Duplicate.to_string().is_empty());
    }

    #[test]
    fn fixed_causal_authority_rejects_capacity_plus_one_with_exact_identity_and_closes_one_owner_at_a_time() {
        let mut dag = MutationDag::new();
        for index in 0..MUTATION_DAG_CAPACITY {
            dag.seed_applied(crate::ids::MutationId(format!("seed-{index:04}"))).expect("exact fixed capacity is admitted");
        }
        let rejected = dag.seed_applied(crate::ids::MutationId("capacity-plus-one".into())).expect_err("capacity plus one must retain its exact identity owner");
        assert_eq!(rejected.error, MutationDagError::Capacity);
        assert_eq!(rejected.mutation_id.0, "capacity-plus-one");
        let mut released = 0;
        while let Some(owner) = dag.take_one_close_owner() {
            assert!(matches!(owner, MutationDagCloseOwner::Identity(_)));
            released += 1;
        }
        assert_eq!(released, MUTATION_DAG_CAPACITY);
        assert!(dag.terminal_is_empty());
    }

    #[test]
    fn causal_insert_rejects_oversized_identity_without_losing_the_envelope_owner() {
        let mut dag = MutationDag::new();
        let envelope = sample_envelope(&"x".repeat(MUTATION_DAG_IDENTIFIER_BYTES + 1), vec![]);
        let rejected = dag.insert(envelope).expect_err("oversized identity is rejected before fixed-slot adoption");
        assert_eq!(rejected.error, MutationDagError::IdentifierTooLong);
        assert_eq!(rejected.envelope.mutation_id.0.len(), MUTATION_DAG_IDENTIFIER_BYTES + 1);
        assert!(dag.terminal_is_empty());
    }

    //#region 🏃️quick
    mod quick {
        use super::*;

        /// @emoji 🔁️ Diamond DAG (A none; B,C dep A; D dep B,C) inserted in every hand-picked
        /// topological order converges to the same final applied set and drained envelope count —
        /// the "permutation-convergence" law the amendment's testing note asks for at the `quick`
        /// tier. True topological orders never hit the `insert`-classification quirk documented on
        /// `MutationDag` above (every dependency is already `applied`, not merely known, by induction).
        fn diamond(id_a: &str, id_b: &str, id_c: &str, id_d: &str) -> [(&'static str, MutationEnvelope); 4] {
            [("a", sample_envelope(id_a, vec![])), ("b", sample_envelope(id_b, vec![id_a])), ("c", sample_envelope(id_c, vec![id_a])), ("d", sample_envelope(id_d, vec![id_b, id_c]))]
        }

        fn assert_converges(order: [&str; 4]) {
            let nodes = diamond("A", "B", "C", "D");
            let mut dag = MutationDag::new();
            for label in order {
                let (_, envelope) = nodes.iter().find(|(l, _)| *l == label).expect("known label").clone();
                let result = dag.insert(envelope).expect("insert never duplicates in a fresh dag");
                assert_eq!(result, InsertResult::Applied, "insertion order {order:?} must stay fully topological");
            }
            let drained = take_applied(&mut dag);
            let mut ids: Vec<String> = drained.iter().map(|e| e.mutation_id.0.clone()).collect();
            ids.sort();
            assert_eq!(ids, vec!["A".to_string(), "B".to_string(), "C".to_string(), "D".to_string()]);
            retire_dag_shell(&mut dag);
        }

        #[test]
        fn topological_order_a_b_c_d_converges() {
            assert_converges(["a", "b", "c", "d"]);
        }

        #[test]
        fn topological_order_a_c_b_d_converges() {
            assert_converges(["a", "c", "b", "d"]);
        }

        #[test]
        fn topological_order_a_b_d_c_is_rejected_as_non_topological() {
            // "d" before "c" is NOT a valid topological order (d depends on c) — insert must not
            // silently accept it as Applied; it must classify as Pending instead, proving this
            // test suite actually distinguishes topological from non-topological orderings rather
            // than accepting anything.
            let nodes = diamond("A", "B", "C", "D");
            let mut dag = MutationDag::new();
            for label in ["a", "b", "d"] {
                let (_, envelope) = nodes.iter().find(|(l, _)| *l == label).expect("known label").clone();
                let result = dag.insert(envelope).unwrap();
                if label == "d" {
                    assert_eq!(result, InsertResult::Pending, "d must not apply before its dependency c arrives");
                }
            }
            dag.insert(nodes.into_iter().find(|(l, _)| *l == "c").unwrap().1).unwrap();
            let mut ids: Vec<String> = dag.applied.iter().cloned().collect();
            ids.sort();
            assert_eq!(ids, vec!["A".to_string(), "B".to_string(), "C".to_string(), "D".to_string()], "once c arrives, d converges too");
            retire_dag_shell(&mut dag);
        }
    }
    //#endregion 🏃️quick
    //#endregion 🔖️MutationDag

    //#region 🔖️Frontier
    fn frontier(document_id: &str, ordinal: u64, head_id: &str, commit_seq: u64, chain_byte: u8) -> FrontierSummary {
        FrontierSummary { document_id: crate::ids::ArtifactId(document_id.into()), head_edit_ordinal: ordinal, head_edit_id: head_id.into(), last_commit_seq: commit_seq, chain_hash: [chain_byte; 32] }
    }

    #[test]
    fn frontier_delta_identical_summaries_are_equal() {
        let a = frontier("doc-1", 5, "edit-5", 3, 9);
        let b = a.clone();
        assert_eq!(frontier_delta(&a, &b), FrontierComparison::Equal);
    }

    #[test]
    fn frontier_delta_greater_ordinal_is_ahead() {
        let local = frontier("doc-1", 10, "edit-10", 4, 1);
        let remote = frontier("doc-1", 5, "edit-5", 3, 2);
        assert_eq!(frontier_delta(&local, &remote), FrontierComparison::Ahead);
    }

    #[test]
    fn frontier_delta_lesser_ordinal_is_behind() {
        let local = frontier("doc-1", 5, "edit-5", 3, 1);
        let remote = frontier("doc-1", 10, "edit-10", 4, 2);
        assert_eq!(frontier_delta(&local, &remote), FrontierComparison::Behind);
    }

    #[test]
    fn frontier_delta_same_ordinal_different_head_is_diverged() {
        let local = frontier("doc-1", 5, "edit-5a", 3, 1);
        let remote = frontier("doc-1", 5, "edit-5b", 3, 2);
        assert_eq!(frontier_delta(&local, &remote), FrontierComparison::Diverged { common_edit_count: 5 });
    }

    #[test]
    fn frontier_summary_serde_round_trips() {
        let summary = frontier("doc-1", 7, "edit-7", 2, 5);
        let json = serde_json::to_string(&summary).expect("serialize");
        let round_tripped: FrontierSummary = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(round_tripped, summary);
    }
    //#endregion 🔖️Frontier

    //#region 🔖️Transform
    #[test]
    fn transform_unchanged_when_deltas_match() {
        let a = CausalAddOp { delta: 3 };
        let b = CausalAddOp { delta: 3 };
        assert_eq!(a.transform(&b), TransformOutcome::Unchanged(CausalAddOp { delta: 3 }));
    }

    #[test]
    fn transform_transformed_when_deltas_differ() {
        let a = CausalAddOp { delta: 2 };
        let b = CausalAddOp { delta: 5 };
        assert_eq!(a.transform(&b), TransformOutcome::Transformed(CausalAddOp { delta: 7 }));
    }

    #[test]
    fn transform_conflict_case_carries_message() {
        let a = CausalAddOp { delta: 0 };
        let b = CausalAddOp { delta: 9 };
        match a.transform(&b) {
            TransformOutcome::Conflict(message) => assert!(!message.is_empty()),
            other => panic!("expected Conflict, got {other:?}"),
        }
    }
    //#endregion 🔖️Transform

    //#region 🔖️Bridge
    #[test]
    fn mutation_envelope_from_edit_derives_one_envelope_per_forward_op_using_explicit_meta() {
        let edit = crate::mutation::Edit::<CausalAddOp> {
            id: "edit-1".into(),
            actor: Some("actor-fallback".into()),
            forwards: vec![CausalAddOp { delta: 1 }, CausalAddOp { delta: 2 }],
            inverse: vec![CausalAddOp { delta: -1 }, CausalAddOp { delta: -2 }],
            mutation_meta: vec![
                crate::mutation::MutationMeta {
                    mutation_id: Some(crate::ids::MutationId("op-a".into())),
                    dependencies: vec![crate::ids::MutationId("op-0".into())],
                    base_version: 0,
                    author_id: Some(crate::ids::ActorId("actor-explicit".into())),
                    timestamp: crate::ids::HybridLogicalTimestamp::new(1, 1000),
                    undo_policy: crate::UndoPolicy::ExactBaseOnly,
                    payload_hash: None,
                    semantic_kind: None,
                    label: None,
                    group_id: None,
                    origin: crate::mutation::MutationOrigin::Owner,
                },
                crate::mutation::MutationMeta {
                    mutation_id: Some(crate::ids::MutationId("op-b".into())),
                    dependencies: vec![crate::ids::MutationId("op-a".into())],
                    base_version: 1,
                    author_id: None,
                    timestamp: crate::ids::HybridLogicalTimestamp::new(1, 2000),
                    undo_policy: crate::UndoPolicy::ExactBaseOnly,
                    payload_hash: None,
                    semantic_kind: None,
                    label: None,
                    group_id: None,
                    origin: crate::mutation::MutationOrigin::Owner,
                },
            ],
            description: None,
            coalesce_key: None,
            sequence_number: 1,
            started_at: "2026-07-27T00:00:00Z".into(),
            finished_at: None,
        };
        let document_id = crate::ids::ArtifactId("doc-1".into());
        let schema = crate::ids::SchemaId("causal-add.v1".into());

        let envelopes = mutation_envelope_from_edit(&edit, &document_id, &schema).expect("encode succeeds");
        assert_eq!(envelopes.len(), 2);

        assert_eq!(envelopes[0].mutation_id, crate::ids::MutationId("op-a".into()));
        assert_eq!(envelopes[0].actor, crate::ids::ActorId("actor-explicit".into()));
        assert_eq!(envelopes[0].dependencies, vec![crate::ids::MutationId("op-0".into())]);
        assert_eq!(envelopes[0].document_id, document_id);
        assert_eq!(envelopes[0].timestamp, crate::ids::HybridLogicalTimestamp::new(1, 1000));
        assert_eq!(envelopes[0].diff.schema, schema);
        assert_eq!(envelopes[0].diff.payload, crate::mutation::OpBinary::encode_op(&CausalAddOp { delta: 1 }).unwrap());
        assert_eq!(envelopes[0].inverse.payload, crate::mutation::OpBinary::encode_op(&CausalAddOp { delta: -1 }).unwrap());

        // Second op's meta has no author_id -> falls back to `edit.actor`, not "unknown".
        assert_eq!(envelopes[1].mutation_id, crate::ids::MutationId("op-b".into()));
        assert_eq!(envelopes[1].actor, crate::ids::ActorId("actor-fallback".into()));
    }

    #[test]
    fn mutation_envelope_from_edit_falls_back_to_op_trait_and_structural_defaults_without_meta() {
        let edit = crate::mutation::Edit::<CausalAddOp> {
            id: "edit-2".into(),
            actor: None,
            forwards: vec![CausalAddOp { delta: 5 }],
            inverse: vec![],
            mutation_meta: vec![],
            description: None,
            coalesce_key: None,
            sequence_number: 0,
            started_at: "2026-07-27T00:00:00Z".into(),
            finished_at: None,
        };
        let document_id = crate::ids::ArtifactId("doc-2".into());
        let schema = crate::ids::SchemaId("causal-add.v1".into());

        let envelopes = mutation_envelope_from_edit(&edit, &document_id, &schema).expect("encode succeeds");
        assert_eq!(envelopes.len(), 1);
        assert_eq!(envelopes[0].mutation_id, crate::ids::MutationId("edit-2#0".into()));
        assert_eq!(envelopes[0].actor, crate::ids::ActorId("unknown".into()));
        assert!(envelopes[0].dependencies.is_empty());
        assert_eq!(envelopes[0].timestamp, crate::ids::HybridLogicalTimestamp::new(0, 0));
        assert_eq!(envelopes[0].inverse.payload, Vec::<u8>::new(), "inverse vec shorter than forwards -> empty inverse payload");
    }

    #[test]
    fn mutation_envelope_from_edit_propagates_an_encode_failure() {
        let edit = crate::mutation::Edit::<CausalAddOp> {
            id: "edit-3".into(),
            actor: None,
            forwards: vec![CausalAddOp { delta: 1 }],
            inverse: vec![],
            mutation_meta: vec![],
            description: None,
            coalesce_key: None,
            sequence_number: 0,
            started_at: "2026-07-27T00:00:00Z".into(),
            finished_at: None,
        };
        // CausalAddOp::encode_op is infallible by construction, so this test instead documents
        // the law via the Result signature: a real Op whose encode_op can fail (e.g. exceeding a
        // size limit) aborts the whole batch rather than returning a partial Vec.
        let document_id = crate::ids::ArtifactId("doc-3".into());
        let schema = crate::ids::SchemaId("causal-add.v1".into());
        assert!(mutation_envelope_from_edit(&edit, &document_id, &schema).is_ok());
    }
    //#endregion 🔖️Bridge

    //#region 🔖️EnvelopeCodec
    #[test]
    fn envelope_binary_round_trips() {
        let envelope = sample_envelope("operation-1", vec!["operation-0", "operation-x"]);
        let mut out = Vec::new();
        encode_envelope(&envelope, &mut out);
        let mut pos = 0;
        let decoded = decode_envelope(&out, &mut pos).expect("decode");
        assert_eq!(decoded, envelope);
        assert_eq!(pos, out.len(), "decode must consume exactly the encoded bytes");
    }

    #[test]
    fn envelope_binary_encoding_is_deterministic() {
        let envelope = sample_envelope("operation-1", vec!["operation-0"]);
        let mut a = Vec::new();
        let mut b = Vec::new();
        encode_envelope(&envelope, &mut a);
        encode_envelope(&envelope, &mut b);
        assert_eq!(a, b);
    }

    #[test]
    fn envelope_binary_round_trips_with_empty_dependencies_and_payloads() {
        let envelope = MutationEnvelope {
            mutation_id: crate::ids::MutationId("op-empty".into()),
            document_id: crate::ids::ArtifactId("doc-empty".into()),
            actor: crate::ids::ActorId("actor-empty".into()),
            dependencies: Vec::new(),
            diff: ArtifactDiff { schema: crate::ids::SchemaId("s".into()), payload: Vec::new() },
            inverse: InverseMutation { schema: crate::ids::SchemaId("s".into()), payload: Vec::new() },
            timestamp: crate::ids::HybridLogicalTimestamp::new(0, 0),
        };
        let mut out = Vec::new();
        encode_envelope(&envelope, &mut out);
        let mut pos = 0;
        assert_eq!(decode_envelope(&out, &mut pos).unwrap(), envelope);
    }

    #[test]
    fn frontier_binary_round_trips() {
        let f = frontier("doc-1", 7, "edit-7", 3, 9);
        let mut out = Vec::new();
        encode_frontier(&f, &mut out);
        let mut pos = 0;
        assert_eq!(decode_frontier(&out, &mut pos).unwrap(), f);
        assert_eq!(pos, out.len());
    }

    #[test]
    fn envelopes_batch_binary_round_trips_including_empty() {
        let empty: Vec<MutationEnvelope> = Vec::new();
        assert_eq!(decode_envelopes(&encode_envelopes(&empty)).unwrap(), empty);

        let batch = vec![sample_envelope("operation-1", vec!["operation-0"]), sample_envelope("operation-2", Vec::new())];
        assert_eq!(decode_envelopes(&encode_envelopes(&batch)).unwrap(), batch);
    }

    #[test]
    fn ops_vec_binary_round_trips_including_empty() {
        let empty: Vec<Vec<u8>> = Vec::new();
        assert_eq!(decode_ops_vec(&encode_ops_vec(&empty)).unwrap(), empty);

        let ops = vec![vec![1u8, 2, 3], Vec::new(), vec![9u8; 5]];
        assert_eq!(decode_ops_vec(&encode_ops_vec(&ops)).unwrap(), ops);
    }
    //#endregion 🔖️EnvelopeCodec
}
//#endregion 🧪️Tests
