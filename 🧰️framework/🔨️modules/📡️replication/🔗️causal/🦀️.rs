//! 🎞️ Protocol causal layer: `MutationEnvelope`/`ArtifactDiff`/`InverseMutation`, the `MutationDag`
//! causal buffer, the runtime frontier-summary twin, the `MutationTransform` hook, and the
//! `mutation_envelope_from_edit` bridge from `crate::mutation::Edit`. Moved from
//! `framework/core/rs/lib.rs`'s `🔖️Sync` region (`MutationEnvelope` L6246, `ArtifactDiff` L6121,
//! `InverseMutation` L6137, `MutationDag`/`InsertResult`/`MutationDagError` L6266-6380 including its existing
//! unit tests at L6488-6572) and `vcs/rs/lib.rs`'s `mutation_envelope_from_edit`. Frozen contract:
//! `.🧬semio/🦑️repo/🎫️tickets/26/07/27/PROTOCOL-BINARY-OP-LOG-LAYER/contract.md` `## Amendment` §`protocol_causal`.
//!
//! This crate's `FrontierSummary`/`frontier_delta` are the runtime/wire twin of
//! `protocol_history`'s durable-log-derived pair — deliberately kept separate, see `🔖️Frontier`.

#[path = "🔀️transition/🦀️.rs"]
pub mod transition;
pub use transition::*;

//#region 🔖️Envelope
// Moved from framework/core L6246 (MutationEnvelope), L6121 (ArtifactDiff), L6137
// (InverseMutation). The frozen contract's field shapes are simpler than the framework-core
// originals (no `schema_version`/`payload_hash` on the envelope, no `target_mutation`/
// `base_version`/`dependencies`/`undo_policy` on the inverse) — implemented exactly as specified
// below.
//
// 🎯️ W5: `payload`/`inverse_diff` flip from `serde_json::Value` to opaque `Vec<u8>` — the binary
// twin of an operation crossing the wire, matching M-C's "communication AND storage both binary"
// requirement. `payload` is the `crate::io::OpBinary` encoding of the op (or a
// producer-defined encoding named by `schema` for a non-typed-op payload, e.g. `db`'s pathmap
// convention); `schema` is a real `crate::ids::SchemaId`, no longer a `std::any::type_name`
// placeholder (see `🔖️Bridge` below). `InverseMutation.inverse_diff` is renamed to `payload` for
// the same reason `ArtifactDiff.payload` is named `payload`, not `diff` — both now hold the same
// kind of thing (an encoded op), not a structural diff.
//
// 🌱️ RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS (26/09/01): the "both fields still
// carry serde for the WIT/backbone JSON seam" note above is stale — repo-wide grep found zero
// non-test call site that serializes `MutationEnvelope`/`ArtifactDiff`/`InverseMutation` via serde;
// every real host↔guest value crossing in this codebase now goes through `to_dsl_value` +
// `store::pack_rt::encode_wire_value` (first-party, not JSON). Converted to hand-written
// `ToValue`/`FromValue` below, mirroring the pre-existing wire shape byte-for-byte.

/// ✉️ A causally-ordered operation crossing the wire: identity, actor, dependency set, the
/// forward diff, its precomputed inverse, and the HLC tick it was authored at. `dependencies` are
/// ordering constraints (a replica applies the operation only after every one of them); `observed`
/// is advisory authoring context and never orders anything: the newest operation of ANOTHER author
/// the author's replica had applied when it authored this one (`None`: none, or unknown). `target`
/// is the structured address the operation writes, as its mutation declares it (outermost segment
/// first; empty: the whole artifact). The hub grades a write against the other authors' writes
/// committed after `observed` whose targets overlap (ticket 26/09/23 LD item 2). `transaction` names
/// the committed tool transaction that authored the operation (`None` for every transition and for
/// operations authored outside a tool transaction). `verb` is the id of the action or command whose edit carried the
/// operation (`None` for every transition and for operations no verb authored), so a peer labels its history row
/// exactly as the author does.
#[derive(Clone, Debug, PartialEq)]
pub struct MutationEnvelope {
    pub mutation_id: crate::ids::MutationId,
    pub document_id: crate::ids::ArtifactId,
    pub actor: crate::ids::ActorId,
    pub dependencies: Vec<crate::ids::MutationId>,
    pub observed: Option<crate::ids::MutationId>,
    pub target: Vec<String>,
    pub diff: ArtifactDiff,
    pub inverse: InverseMutation,
    pub timestamp: crate::ids::HybridLogicalTimestamp,
    pub transaction: Option<crate::mutation::TransactionRef>,
    pub verb: Option<String>,
    /// 🌿️ The alternative the operation was authored on. `None` is the trunk.
    pub line: Option<String>,
}




/// 🧮️ A schema-tagged, opaque binary forward-op payload.
#[derive(Clone, Debug, PartialEq)]
pub struct ArtifactDiff {
    pub schema: crate::ids::SchemaId,
    pub payload: Vec<u8>,
}




/// ↩️ A schema-tagged, opaque binary inverse-op payload.
#[derive(Clone, Debug, PartialEq)]
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

/// 🕸️ Causal DAG of exchanged `MutationEnvelope`s: buffers envelopes until their
/// dependencies are applied.
pub const MUTATION_DAG_CAPACITY: usize = 8_192;
pub const MUTATION_DAG_IDENTIFIER_BYTES: usize = 256;

struct MutationDagFixedSlots<T> {
    slots: Box<[Option<Box<[std::mem::MaybeUninit<T>; MUTATION_DAG_PAGE_SLOTS]>>]>,
    close_page: usize,
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
const MUTATION_DAG_PAGE_SLOTS: usize = 64;
const MUTATION_DAG_PAGE_COUNT: usize = MUTATION_DAG_CAPACITY / MUTATION_DAG_PAGE_SLOTS;

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
        Some(unsafe { self.owner.slot(slot).assume_init_ref() })
    }
}

impl<T> MutationDagFixedSlots<T> {
    fn empty() -> Self {
        Self { slots: Vec::new().into_boxed_slice(), close_page: 0, generations: Vec::new().into_boxed_slice(), occupied: Vec::new().into_boxed_slice(), next: Vec::new().into_boxed_slice(), previous: Vec::new().into_boxed_slice(), free: Vec::new().into_boxed_slice(), free_len: 0, head: MUTATION_DAG_SLOT_NONE, tail: MUTATION_DAG_SLOT_NONE, len: 0 }
    }

    fn new() -> Self {
        let free = (0..MUTATION_DAG_CAPACITY).rev().map(|slot| slot as u16).collect::<Vec<_>>().into_boxed_slice();
        Self {
            slots: (0..MUTATION_DAG_PAGE_COUNT).map(|_| Some(Box::new([const { std::mem::MaybeUninit::<T>::uninit() }; MUTATION_DAG_PAGE_SLOTS]))).collect::<Vec<_>>().into_boxed_slice(),
            close_page: 0,
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

    fn slot(&self, slot: usize) -> &std::mem::MaybeUninit<T> {
        &self.slots[slot / MUTATION_DAG_PAGE_SLOTS].as_ref().expect("causal page remains retained")[slot % MUTATION_DAG_PAGE_SLOTS]
    }

    fn slot_mut(&mut self, slot: usize) -> &mut std::mem::MaybeUninit<T> {
        &mut self.slots[slot / MUTATION_DAG_PAGE_SLOTS].as_mut().expect("causal page remains retained")[slot % MUTATION_DAG_PAGE_SLOTS]
    }

    fn backing_is_empty(&self) -> bool {
        self.slots.is_empty() && self.generations.is_empty() && self.occupied.is_empty() && self.next.is_empty() && self.previous.is_empty() && self.free.is_empty()
    }

    fn next_backing_release_byte_demand(&self) -> usize {
        if !self.is_empty() { return 0; }
        if !self.slots.is_empty() {
            return if self.close_page < self.slots.len() { std::mem::size_of::<[std::mem::MaybeUninit<T>; MUTATION_DAG_PAGE_SLOTS]>() } else { std::mem::size_of_val(&*self.slots) };
        }
        if !self.generations.is_empty() { return std::mem::size_of_val(&*self.generations); }
        if !self.occupied.is_empty() { return std::mem::size_of_val(&*self.occupied); }
        if !self.next.is_empty() { return std::mem::size_of_val(&*self.next); }
        if !self.previous.is_empty() { return std::mem::size_of_val(&*self.previous); }
        std::mem::size_of_val(&*self.free)
    }

    fn close_backing_step(&mut self, maximum_items: usize, maximum_release_bytes: usize) -> (usize, usize, bool) {
        if self.backing_is_empty() { return (0, 0, true); }
        let demand = self.next_backing_release_byte_demand();
        if !self.is_empty() || maximum_items == 0 || maximum_release_bytes < demand { return (0, 0, false); }
        if !self.slots.is_empty() {
            if self.close_page < self.slots.len() {
                drop(self.slots[self.close_page].take());
                self.close_page += 1;
            } else { self.slots = Vec::new().into_boxed_slice(); }
        } else if !self.generations.is_empty() { self.generations = Vec::new().into_boxed_slice(); }
        else if !self.occupied.is_empty() { self.occupied = Vec::new().into_boxed_slice(); }
        else if !self.next.is_empty() { self.next = Vec::new().into_boxed_slice(); }
        else if !self.previous.is_empty() { self.previous = Vec::new().into_boxed_slice(); }
        else { self.free = Vec::new().into_boxed_slice(); self.free_len = 0; }
        (1, demand, self.backing_is_empty())
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

    #[cfg(test)]
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
        self.slot_mut(slot).write(value);
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
        unsafe { self.slot_mut(slot).assume_init_read() }
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
        // 🧯️ A thread that is ALREADY unwinding is not holding this contract wrong — it is being
        // torn down. Panicking a second time here turns the first, diagnosable panic into
        // `panic in a destructor during cleanup` + `SIGABRT`, which takes the whole test binary
        // down and hides every other test's result. The occupied slots are `MaybeUninit`, so
        // returning early leaks them rather than dropping them — safe, and the process is dying
        // anyway. Same guard as `ValueRetirement`, `ordered::Retirement` and `Dictionary`.
        assert!(self.is_empty() || std::thread::panicking(), "fixed causal slots reached Drop before every exact nested owner was detached");
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
        // 🧯️ Panicking-aware for the same reason as `MutationDagFixedSlots::drop` above: a second
        // panic while the thread already unwinds aborts the whole test binary and erases the first
        // panic's diagnosis.
        assert!(self.terminal_is_empty() || std::thread::panicking(), "mutation dag reached Drop before every exact envelope and identity owner was cursor-retired");
    }
}

/// 🚦️ The outcome of one `MutationDag::insert` call.
#[derive(Debug, PartialEq)]
#[cfg_attr(target_pointer_width = "64", expect(clippy::large_enum_variant, reason = "Duplicate admission returns the exact envelope owner without allocating on refusal."))]
pub enum InsertResult {
    Applied,
    Pending,
    AlreadyApplied(MutationEnvelope),
}

/// 🚨️ `MutationDag`'s one failure mode: the same operation id inserted twice while still pending.
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

#[cfg_attr(target_pointer_width = "64", expect(clippy::large_enum_variant, reason = "Each bounded drain step transfers an existing envelope owner without allocating a wrapper."))]
pub enum MutationDagAppliedStep {
    Envelope(MutationEnvelope),
    SeededIdentity,
    Complete,
}

impl MutationDag {
    /// 🪹️ An allocation-free terminal shell for granted transfer of the original causal owner.
    pub fn empty() -> Self {
        Self { envelopes: MutationDagFixedSlots::empty(), applied: MutationDagFixedSlots::empty(), drained: 0, pending: MutationDagFixedSlots::empty() }
    }

    pub fn new() -> Self {
        Self::default()
    }

    /// 🧹 Proves every original mutation and identity owner has been detached; backing remains separately retained.
    pub fn terminal_is_empty(&self) -> bool {
        self.envelopes.is_empty() && self.applied.is_empty() && self.pending.is_empty()
    }

    /// 🪹️ Proves all causal payload pages and fixed metadata allocations have been funded and released.
    pub fn backing_is_empty(&self) -> bool {
        self.envelopes.backing_is_empty() && self.applied.backing_is_empty() && self.pending.backing_is_empty()
    }

    /// 📏️ Queries the next indivisible original page or metadata allocation without work or ownership transfer.
    pub fn next_backing_release_byte_demand(&self) -> usize {
        if !self.terminal_is_empty() { return 0; }
        if !self.envelopes.backing_is_empty() { self.envelopes.next_backing_release_byte_demand() }
        else if !self.applied.backing_is_empty() { self.applied.next_backing_release_byte_demand() }
        else { self.pending.next_backing_release_byte_demand() }
    }

    /// 🧱️ Releases at most one original backing allocation after every exact payload owner has detached.
    pub fn close_backing_step(&mut self, maximum_items: usize, maximum_release_bytes: usize) -> (usize, usize, bool) {
        if !self.terminal_is_empty() { return (0, 0, false); }
        if self.backing_is_empty() { return (0, 0, true); }
        let (items, bytes, _) = if !self.envelopes.backing_is_empty() { self.envelopes.close_backing_step(maximum_items, maximum_release_bytes) }
        else if !self.applied.backing_is_empty() { self.applied.close_backing_step(maximum_items, maximum_release_bytes) }
        else { self.pending.close_backing_step(maximum_items, maximum_release_bytes) };
        (items, bytes, self.backing_is_empty())
    }

    /// ⏳️ No inserted envelope still waits on a dependency this dag has never seen.
    pub fn pending_is_empty(&self) -> bool {
        self.pending.is_empty()
    }

    /// ➕️ Inserts one envelope. Returns `AlreadyApplied` if its id was applied before,
    /// `Err(Duplicate)` if it's already buffered as pending, `Pending` while any dependency is not
    /// applied — unknown to this dag, or buffered and itself still pending — else `Applied`.
    ///
    /// ⛓️ The unblocking is a CASCADE, not one step. An accepted envelope can release a whole
    /// pending chain — `c` depends on `b` depends on `a`, all three arriving in reverse order — and a
    /// single {@link advance_ready_one} released only `b`, leaving `c` pending forever. That is the
    /// exact shape `assert_op_dag_convergence` measured: the same closed dependency set converged to
    /// DIFFERENT applied sets depending on arrival order. The loop terminates because every
    /// successful step moves one id out of `pending`, which is fixed-capacity.
    #[expect(clippy::result_large_err, reason = "Fixed-capacity admission returns the original envelope without allocating on rejection.")]
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
        let pending = envelope.dependencies.iter().any(|dependency| !self.applied.iter().any(|applied| applied == &dependency.0));
        let id = envelope.mutation_id.0.clone();
        self.envelopes.push_reserved(envelope);
        if pending {
            self.pending.push_reserved(id);
            return Ok(InsertResult::Pending);
        }
        self.mark_applied(&id);
        while self.advance_ready_one() {}
        Ok(InsertResult::Applied)
    }

    /// ✅️ Borrows one ready identity at a caller-owned cursor without materializing a list.
    pub fn ready_identity_at(&self, cursor: usize) -> Option<&str> {
        let id = self.pending.get(cursor)?;
        self.envelopes.iter().find(|envelope| envelope.mutation_id.0 == *id).filter(|envelope| envelope.dependencies.iter().all(|dependency| self.applied.iter().any(|applied| applied == &dependency.0))).map(|envelope| envelope.mutation_id.0.as_str())
    }

    /// 🧺️ Transfers at most one exact applied owner at the retained drain cursor.
    pub fn take_next_applied(&mut self) -> MutationDagAppliedStep {
        let Some(id) = self.applied.get(self.drained) else { return MutationDagAppliedStep::Complete };
        self.drained += 1;
        let Some(index) = self.envelopes.iter().position(|envelope| envelope.mutation_id.0 == *id) else {
            return MutationDagAppliedStep::SeededIdentity;
        };
        MutationDagAppliedStep::Envelope(self.envelopes.swap_remove(index).expect("validated applied envelope slot remains occupied"))
    }

    /// 🌱️ Seeds one id into the applied-set from out-of-band knowledge (e.g. a full-document
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
/// 🏔️ Runtime/wire twin of `os_spr::history::FrontierSummary` — the shape `db` and
/// `framework/sync` exchange without a full history-log decode. Deliberately NOT unified with the
/// durable-log-derived version: they serve different layers (live runtime state vs on-disk log).
/// 🌱️ Carries serde's derives alongside the hand-written `ToValue`/`FromValue` twin below, the same
/// transitional shape every sibling wire type in `📡️wire` holds: `server`'s CQRS contract embeds this
/// struct inside serde-derived envelopes (`CommandEnvelope.causal_frontier`, `CommandOutcome`,
/// `QueryConsistency::AtFrontier`, `QueryResult`), a bound only a real derive can satisfy. `serde` is
/// an unconditional dependency of this crate, so the derive adds nothing to any target, wasm included.
/// No `#[serde(rename_all = …)]`, matching the `ToValue` twin's snake_case field names byte-for-byte.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FrontierSummary {
    pub document_id: crate::ids::ArtifactId,
    pub head_edit_ordinal: u64,
    pub head_edit_id: String,
    pub last_commit_seq: u64,
    pub chain_hash: [u8; 32],
}

/// 🌱️ Hand-written, not derived — same DAG reason `MutationEnvelope`'s hand-written twin above
/// documents. No `#[serde(rename_all = …)]` on the original, so field names stay snake_case.



/// ⚖️ How a `local` frontier relates to a `remote` one.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FrontierComparison {
    Equal,
    Ahead,
    Behind,
    Diverged { common_edit_count: u64 },
}

/// 🌱️ Hand-written, not derived — same reason as `FrontierSummary` above. No `#[serde(tag = …)]`
/// on the original and one data-carrying variant, so serde's default representation applies:
/// externally tagged, unit variants as bare strings, `Diverged` as a one-key object — and no
/// `rename_all`, so both the outer key and `common_edit_count` stay exactly as declared.



/// 🔎️ Compares two frontier summaries. Design choice (the contract fixes the enum shape,
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
/// 🔀️ The result of transforming one operation against a concurrent one.
#[derive(Clone, Debug, PartialEq)]
pub enum TransformOutcome<Op> {
    Unchanged(Op),
    Transformed(Op),
    Conflict(String),
}

/// 🧮️ Operational-transform hook: rewrites `self` so it applies cleanly after `against`
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
/// 🪪️ The wire `MutationId` each of `edit.forwards` would get if fanned out through
/// `mutation_envelope_from_edit` — same fallback chain (`mutation_meta[i]` field, else the `Op`
/// trait method, else `{edit.id}#{i}`), extracted so callers that only need identity (e.g.
/// snapshot-vs-operations-message dedup) don't have to pay for `encode_op`/`inverse` work, and so
/// there is exactly one place this chain is spelled out.
pub fn mutation_ids_for_edit<P, Op: crate::mutation::Mutation<P>>(edit: &crate::mutation::Edit<Op>) -> Vec<crate::ids::MutationId> {
    edit.forwards.iter().enumerate().map(|(index, op)| edit_operation_mutation_id(edit, index, op)).collect()
}

/// 🪪️ One operation's wire identity without materializing its edit's other identities.
pub fn mutation_id_for_edit_operation<P, Op: crate::mutation::Mutation<P>>(edit: &crate::mutation::Edit<Op>, index: usize) -> Option<crate::ids::MutationId> {
    edit.forwards.get(index).map(|op| edit_operation_mutation_id(edit, index, op))
}

fn edit_operation_mutation_id<P, Op: crate::mutation::Mutation<P>>(edit: &crate::mutation::Edit<Op>, index: usize, op: &Op) -> crate::ids::MutationId {
    match edit.mutation_meta.get(index).and_then(|m| m.mutation_id.clone()) {
        Some(id) => id,
        None => match op.mutation_id() {
            Some(id) => id,
            None => crate::ids::MutationId(format!("{}#{index}", edit.id)),
        },
    }
}

pub fn mutation_envelope_from_edit<P, Op: crate::mutation::Mutation<P> + crate::io::OpBinary>(
    edit: &crate::mutation::Edit<Op>,
    document_id: &crate::ids::ArtifactId,
    schema: &crate::ids::SchemaId,
) -> Result<Vec<MutationEnvelope>, crate::ProtocolError> {
    mutation_envelopes_from_edit_since(edit, 0, document_id, schema)
}

/// ✂️ The envelopes of `edit`'s operations from position `from` on — exactly the tail of
/// [`mutation_envelope_from_edit`]'s answer, encoding only that tail. An edit that absorbs later
/// operations (a coalesced typing run) is announced one appended range at a time, so re-encoding its
/// whole history per keystroke would make a long run quadratic (ticket 26/09/23 LD item 1).
pub fn mutation_envelopes_from_edit_since<P, Op: crate::mutation::Mutation<P> + crate::io::OpBinary>(
    edit: &crate::mutation::Edit<Op>,
    from: usize,
    document_id: &crate::ids::ArtifactId,
    schema: &crate::ids::SchemaId,
) -> Result<Vec<MutationEnvelope>, crate::ProtocolError> {
    let mut out = Vec::with_capacity(edit.forwards.len().saturating_sub(from));
    for (index, op) in edit.forwards.iter().enumerate().skip(from) {
        let meta = edit.mutation_meta.get(index);
        let mutation_id = edit_operation_mutation_id(edit, index, op);
        let dependencies = match meta {
            Some(m) => m.dependencies.clone(),
            None => op.dependencies(),
        };
        let actor = match meta.and_then(|m| m.author_id.clone()) {
            Some(actor) => actor,
            None => match op.author_id() {
                Some(actor) => actor,
                None => crate::ids::ActorId(edit.actor.clone().unwrap_or_else(|| "unknown".into())),
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
            Some(inv) => crate::io::OpBinary::encode_op(inv)?,
            None => Vec::new(),
        };
        out.push(MutationEnvelope {
            mutation_id,
            document_id: document_id.clone(),
            actor,
            dependencies,
            observed: None,
            target: op.conflict_target(),
            diff: ArtifactDiff { schema: schema.clone(), payload },
            inverse: InverseMutation { schema: schema.clone(), payload: inverse_payload },
            timestamp,
            transaction: meta.and_then(|m| m.transaction.clone()), verb: edit.verb.clone(), line: edit.line.clone(),
        });
    }
    Ok(out)
}
//#endregion 🔖️Bridge

pub const DOCUMENT_BACKBONE_PENDING_MAXIMUM_BYTES: usize = 1_048_576;
pub const DOCUMENT_BACKBONE_PENDING_MAXIMUM_MESSAGES: usize = 64;


//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
