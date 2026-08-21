//! 🗄️ `db_projection` — the `db` family's projection engine: typed, versioned projection classes
//! wired into a dependency DAG, applied incrementally as commands commit, checkpointed via
//! `db_index::ProjectionIndex` for historical ("state as of frontier X") queries, and augmentable
//! with an ephemeral, never-persisted preview overlay. Frozen contract:
//! `.🦑️repo/🎫️tickets/26/07/27/INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING/contract.md`
//! (`## db crate family`, `db_projection` row) and Part 2 of the approved plan.
//!
//! 🎯️ Design choice: per the contract's "no db crate below `db_artifact` interprets operation
//! semantics" rule, this crate never looks inside a `protocol::MutationEnvelope`'s
//! `diff.schema`/`diff.payload` itself — every `ProjectionClass::apply` is supplied by a higher
//! layer (`db_artifact`, or an app-level program) that owns the actual domain interpretation. This
//! crate only owns the mechanism around that: dependency ordering, incremental application,
//! checkpoint persistence/versioning, historical lookup, and preview augmentation.
//!
//! 🎯️ Design choice (dependency addition): the contract's per-crate deps table lists this crate as
//! depending on `db_core, db_state, db_index, protocol` only. `db_index::ProjectionIndex` — the
//! exact `(projection_id, frontier_seq) -> state bytes` primitive this crate's checkpoint
//! persistence and historical queries are built on (see its own doc: "`db_projection`'s 'this
//! projection's state as of at or before frontier X' lookup") — is only constructible against a
//! `&dyn db_storage::IndexStorage`, so `db_storage` is added as a direct dependency here (it was
//! genuinely missing, not optional: without it, `ProjectionIndex` cannot be named or called at
//! all). `db_storage` itself only depends on `db_core`/`pack`/`blake3`, already present elsewhere
//! in the family, so this adds no new external dependency. Noted per the task's deviation-reporting
//! instruction.
//!
//! 🎯️ Design choice (versioning): `db_index::ProjectionIndex` stores opaque bytes with no side
//! channel for a schema version, so every checkpoint this crate persists is prefixed with its
//! `ProjectionClass::schema_version()` as 4 little-endian bytes (`🔖️State`'s `encode_versioned`/
//! `decode_versioned`). Resuming from a checkpoint whose stored version differs from the
//! currently-registered version returns `DbError::Conflict` rather than silently misinterpreting
//! stale bytes — the caller's recovery path is `ProjectionEngine::rebuild_and_persist`.
//!
//! 🎯️ Design choice (incremental triggering — appended alongside the two design choices above,
//! extending rather than replacing the implementation already in place here): the ticket requires
//! "a diff/touched-region triggers only the affected projections", which the original
//! `apply_envelope`/`rebuild_in_memory` did not yet implement (every registered projection ran
//! unconditionally on every envelope). `ProjectionClass` gains `reads()` (the region paths a
//! projection watches directly) and `affected_by()` (checked against a step's `db_state::TouchedSet`);
//! `apply_envelope`/`rebuild_in_memory` now share one `should_run` gate — direct `affected_by` match
//! OR cascade through a dependency that itself ran this same step — and skip
//! `ErasedProjection::apply_bytes`/the `ProjectionIndex::record` write entirely for anything
//! neither, carrying its unchanged prior state forward for `DepView`/the returned map instead. A
//! skipped projection's on-disk frontier simply stays wherever it last actually advanced
//! (`ProjectionIndex::latest_at_or_before` already floors correctly) — exactly the "frontier
//! tracking per projection" the ticket asks for. `should_run` is called identically from both the
//! persisted incremental path and the pure rebuild path, which is what keeps the rebuild ==
//! incremental-apply law holding by construction rather than by two independently-written rules
//! happening to agree.

use crate::*;
use db_index::ProjectionIndex;
use db_state::{PGraph, PMap, TouchedRegion, TouchedSet};
use db_storage::IndexStorage;
use protocol::MutationEnvelope;

//#region 🔖️State
/// @emoji 🧬️ What a projection's in-memory state must support to round-trip through
/// `db_index::ProjectionIndex`'s opaque byte storage: a caller-defined, schema-specific
/// encode/decode pair. `db_projection` never interprets the bytes it stores — only the
/// `ProjectionClass` that produced them does (see the module doc's "no operation semantics" note).
pub trait ProjectionState: Clone + Send + Sync + 'static {
    async fn encode(&self) -> Vec<u8>;
    async fn decode(bytes: &[u8]) -> Result<Self, DbError>
    where
        Self: Sized;
}

/// @emoji 🔢️ A ready-made `ProjectionState` for the common "just count/accumulate a `u64`" shape.
impl ProjectionState for u64 {
    async fn encode(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }

    async fn decode(bytes: &[u8]) -> Result<Self, DbError> {
        let array: [u8; 8] = bytes.try_into().map_err(|_| DbError::Corrupt("expected an 8-byte little-endian u64 projection state".to_string()))?;
        Ok(u64::from_le_bytes(array))
    }
}

/// @emoji 🔤️ A ready-made `ProjectionState` for a UTF-8 text projection.
impl ProjectionState for String {
    async fn encode(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }

    async fn decode(bytes: &[u8]) -> Result<Self, DbError> {
        String::from_utf8(bytes.to_vec()).map_err(|_| DbError::Corrupt("projection state is not valid utf-8".to_string()))
    }
}

/// @emoji 📦️ A ready-made `ProjectionState` for callers that already have their own opaque byte
/// encoding and just want the checkpoint machinery, not another codec layer.
impl ProjectionState for Vec<u8> {
    async fn encode(&self) -> Vec<u8> {
        self.clone()
    }

    async fn decode(bytes: &[u8]) -> Result<Self, DbError> {
        Ok(bytes.to_vec())
    }
}

/// @emoji 🏷️ How many bytes `encode_versioned` prepends — a fixed-width `u32` schema version tag
/// (see the module doc's versioning design-choice note).
const VERSION_PREFIX_LEN: usize = 4;

/// @emoji ✍️ Prefixes `state_bytes` with `schema_version` — the exact shape persisted via
/// `ProjectionIndex::record`.
async fn encode_versioned(schema_version: u32, state_bytes: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(VERSION_PREFIX_LEN + state_bytes.len());
    out.extend_from_slice(&schema_version.to_le_bytes());
    out.extend_from_slice(state_bytes);
    out
}

/// @emoji 📖️ Inverse of `encode_versioned`: splits the version prefix from the state bytes,
/// erroring `Corrupt` (never panicking) if `bytes` is shorter than the prefix itself.
async fn decode_versioned(bytes: &[u8]) -> Result<(u32, &[u8]), DbError> {
    if bytes.len() < VERSION_PREFIX_LEN {
        return Err(DbError::Corrupt("projection checkpoint is shorter than its version prefix".to_string()));
    }
    let mut version_bytes = [0u8; VERSION_PREFIX_LEN];
    version_bytes.copy_from_slice(&bytes[..VERSION_PREFIX_LEN]);
    Ok((u32::from_le_bytes(version_bytes), &bytes[VERSION_PREFIX_LEN..]))
}
//#endregion 🔖️State

//#region 🔖️ProjectionClass
/// @emoji 👀️ One step's view onto sibling projections' just-computed state, for a projection whose
/// `dependencies()` names them — `ProjectionGraph`'s topological order guarantees every dependency
/// has already run (and is present here) before a dependent's `apply` is called in the same step.
/// Scoped to raw bytes (rather than a typed map) so heterogeneous `ProjectionClass::State` types
/// can share one view without this crate needing a type-erased-but-downcastable value box.
#[derive(Clone, Default)]
pub struct DepView {
    states: PMap<String, Vec<u8>>,
}

impl DepView {
    /// @emoji 🔍️ The raw encoded bytes a dependency computed this step, or `None` if `projection_id`
    /// isn't a registered dependency (or hasn't run yet — shouldn't happen given topological order).
    pub async fn get_raw(&self, projection_id: &str) -> Option<&Vec<u8>> {
        self.states.get(&projection_id.to_string())
    }

    /// @emoji 🔍️ `get_raw` decoded through `S::decode` — the typed convenience most `apply` impls
    /// reach for when they know a dependency's concrete `ProjectionState` shape by convention.
    pub async fn get<S: ProjectionState>(&self, projection_id: &str) -> Result<Option<S>, DbError> {
        match self.get_raw(projection_id).await {
            Some(bytes) => S::decode(bytes).await.map(Some),
            None => Ok(None),
        }
    }
}

/// @emoji 📽️ One projection's rules: identity, its schema version (bumping this invalidates every
/// previously-persisted checkpoint, see the module doc), which other projections it reads via
/// `DepView` during `apply`, its starting state, and the state-transition function itself. Never
/// interprets `envelope` beyond what the implementor chooses to (this crate stays semantics-free,
/// see the module doc).
pub trait ProjectionClass: Send + Sync {
    type State: ProjectionState;

    /// @emoji 🪪️ A stable, unique-within-one-`ProjectionEngine` identifier.
    async fn id(&self) -> &'static str;

    /// @emoji 🔢️ This projection's current schema version (see the module doc's versioning note).
    async fn schema_version(&self) -> u32;

    /// @emoji 🕸️ The ids of other registered projections this one reads via `DepView` — must all
    /// resolve to projections registered in the same `ProjectionEngine`, and must not form a cycle
    /// (`ProjectionGraph::build` validates both). Defaults to none.
    async fn dependencies(&self) -> &'static [&'static str] {
        &[]
    }

    /// @emoji 👀️ The document-region path prefixes this projection reads directly —
    /// `affected_by` checks these against a step's touched regions to decide whether this
    /// projection needs to run at all THIS step. Defaults to none, meaning "not directly triggered
    /// by anything" — a projection that should only ever run via a dependency's cascade (see
    /// `dependencies()`) leaves this empty; one that wants to see every step overrides
    /// `affected_by` directly rather than relying on any "empty means everything" implicit
    /// default (there is deliberately none, to avoid that footgun).
    async fn reads(&self) -> &'static [&'static str] {
        &[]
    }

    /// @emoji 🎯️ True iff `touched` should trigger this projection's `apply` on its own,
    /// independent of any dependency cascade (`ProjectionEngine`'s `should_run` layers that on
    /// top). Default: any declared `reads()` path intersects any touched region.
    async fn affected_by(&self, touched: &TouchedSet) -> bool {
        let reads = self.reads().await;
        touched.regions.iter().any(|region| reads.iter().any(|&path| region.path_intersects(&TouchedRegion::read(path))))
    }

    /// @emoji 🌱️ The state of a brand-new instance of this projection, before any envelope has
    /// been applied.
    async fn initial(&self) -> Self::State;

    /// @emoji ⏩️ Computes the next state from `state`, `envelope`, and this step's already-computed
    /// sibling states (`deps`).
    async fn apply(&self, state: &Self::State, envelope: &MutationEnvelope, deps: &DepView) -> Result<Self::State, DbError>;
}

/// @emoji 🎭️ The object-safe view of a `ProjectionClass` — every method operates on raw encoded
/// bytes instead of the associated `State` type, so a `ProjectionEngine<S, E>` can be generic over
/// one concrete `E: ErasedProjection` (MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/dedyn-fw-os-guestruntime,
/// O1/R1: no first-party `dyn` trait objects) instead of erasing into `Box<dyn ErasedProjection>`.
/// A call site that genuinely mixes several `ProjectionClass` types in one `Vec` (this file's own
/// `🧪️Tests` module is the only one that does, repo-wide) declares its own small closed enum over
/// exactly the `ErasedWrapper<P>` types it uses (R11: "closed set ⇒ `dyn_enum_close!`" — hand-written
/// here per the `GuestRuntimes` precedent, since the variant set is `#[cfg(test)]`-only); a call site
/// that uses exactly one `ProjectionClass` (`db_testkit`) just names `E = ErasedWrapper<ItsType>`
/// directly, no enum needed. Implemented automatically for any `ProjectionClass` via `erase`; not
/// meant to be implemented by hand except for `NoProjections` below.
pub trait ErasedProjection: Send + Sync {
    async fn id(&self) -> &'static str;
    async fn schema_version(&self) -> u32;
    async fn dependencies(&self) -> &'static [&'static str];
    async fn reads(&self) -> &'static [&'static str];
    async fn affected_by(&self, touched: &TouchedSet) -> bool;
    async fn initial_bytes(&self) -> Vec<u8>;
    async fn apply_bytes(&self, state_bytes: &[u8], envelope: &MutationEnvelope, deps: &DepView) -> Result<Vec<u8>, DbError>;
}

/// @emoji 🚫️ Zero-variant `ErasedProjection` — `db_artifact::ArtifactEngineConfig`'s `projections`
/// factory default type parameter. Nothing constructs `NoProjections` (uninhabited), so a
/// `Vec<NoProjections>` is always empty by construction and every method body's `match *self {}` is
/// exhaustive over zero arms. Repo-wide, as of this packet, NOTHING overrides that factory with a
/// real closure (every `ArtifactEngineConfig::default()`/`{ ..Default::default() }` call site
/// leaves it at the empty-`Vec`-returning default) — this type is the honest reflection of that: the
/// day a real caller wants to register a projection through `ArtifactEngineConfig`, it swaps this
/// type parameter for its own closed `ErasedProjection` enum (R11).
pub enum NoProjections {}

impl ErasedProjection for NoProjections {
    async fn id(&self) -> &'static str {
        match *self {}
    }

    async fn schema_version(&self) -> u32 {
        match *self {}
    }

    async fn dependencies(&self) -> &'static [&'static str] {
        match *self {}
    }

    async fn reads(&self) -> &'static [&'static str] {
        match *self {}
    }

    async fn affected_by(&self, _touched: &TouchedSet) -> bool {
        match *self {}
    }

    async fn initial_bytes(&self) -> Vec<u8> {
        match *self {}
    }

    async fn apply_bytes(&self, _state_bytes: &[u8], _envelope: &MutationEnvelope, _deps: &DepView) -> Result<Vec<u8>, DbError> {
        match *self {}
    }
}

pub struct ErasedWrapper<P: ProjectionClass>(P);

impl<P: ProjectionClass> ErasedProjection for ErasedWrapper<P> {
    async fn id(&self) -> &'static str {
        self.0.id().await
    }

    async fn schema_version(&self) -> u32 {
        self.0.schema_version().await
    }

    async fn dependencies(&self) -> &'static [&'static str] {
        self.0.dependencies().await
    }

    async fn reads(&self) -> &'static [&'static str] {
        self.0.reads().await
    }

    async fn affected_by(&self, touched: &TouchedSet) -> bool {
        self.0.affected_by(touched).await
    }

    async fn initial_bytes(&self) -> Vec<u8> {
        self.0.initial().await.encode().await
    }

    async fn apply_bytes(&self, state_bytes: &[u8], envelope: &MutationEnvelope, deps: &DepView) -> Result<Vec<u8>, DbError> {
        let state = P::State::decode(state_bytes).await?;
        Ok(self.0.apply(&state, envelope, deps).await?.encode().await)
    }
}

/// @emoji 🎁️ Wraps a concrete `ProjectionClass` into its `ErasedProjection` form for registering
/// into a `ProjectionEngine`. Returns the concrete `ErasedWrapper<P>` (not boxed/erased) — a caller
/// mixing several `ProjectionClass` types converts each into its own small closed enum instead (see
/// `ErasedProjection`'s own doc).
// 🚫️async: E1 pure constructor, used inside a sync `|| vec![...]` factory closure slot — see R9
pub fn erase<P: ProjectionClass + 'static>(class: P) -> ErasedWrapper<P> {
    ErasedWrapper(class)
}
//#endregion 🔖️ProjectionClass

//#region 🔖️Graph
/// @emoji 🕸️ The dependency-validated, topologically-sorted view of a set of registered
/// projections — built once at `ProjectionEngine::new` and reused by every subsequent apply/
/// rebuild pass so per-step ordering work is O(1) instead of a fresh sort each time.
struct ProjectionGraph {
    /// @emoji 🔢️ Indices into the owning `ProjectionEngine`'s `projections` vec, in an order where
    /// every dependency appears before every projection that depends on it.
    order: Vec<usize>,
}

impl ProjectionGraph {
    /// @emoji 🏗️ Validates that every id is unique, every `dependencies()` entry resolves to a
    /// registered projection, and the dependency relation is acyclic — then computes one
    /// deterministic topological order (ties broken lexicographically by id, so the same
    /// registration set always yields the same order across runs/processes).
    async fn build<E: ErasedProjection>(projections: &[E]) -> Result<ProjectionGraph, DbError> {
        let mut graph: PGraph<String, (), ()> = PGraph::new();
        let mut ids: Vec<String> = Vec::with_capacity(projections.len());
        for projection in projections {
            let id = projection.id().await.to_string();
            if graph.contains_node(&id) {
                return Err(DbError::AlreadyExists(format!("projection id {id:?} is registered more than once")));
            }
            graph = graph.add_node(id.clone(), ());
            ids.push(id);
        }
        for projection in projections {
            let id = projection.id().await.to_string();
            for &dependency in projection.dependencies().await {
                if !graph.contains_node(&dependency.to_string()) {
                    return Err(DbError::NotFound(format!("projection {id:?} depends on unregistered projection {dependency:?}")));
                }
                graph = graph.add_edge(dependency.to_string(), id.clone(), ()).expect("both endpoints were just validated present in the graph");
            }
        }

        let mut index_of_id: std::collections::HashMap<String, usize> = std::collections::HashMap::with_capacity(ids.len());
        for (index, projection) in projections.iter().enumerate() {
            index_of_id.insert(projection.id().await.to_string(), index);
        }

        let sorted_ids = topological_sort(&graph, &ids).await?;
        let order = sorted_ids.into_iter().map(|id| index_of_id[&id]).collect();
        Ok(ProjectionGraph { order })
    }
}

/// @emoji 🌀️ Kahn's algorithm over `graph` restricted to `ids`: repeatedly takes the
/// lexicographically-smallest zero-in-degree node, appends it, and decrements its successors'
/// in-degree. `ids.len() > order.len()` at the end means a cycle remains (some node's in-degree
/// never reached zero) — reported as `DbError::InvalidArgument` rather than silently truncating.
async fn topological_sort(graph: &PGraph<String, (), ()>, ids: &[String]) -> Result<Vec<String>, DbError> {
    let mut in_degree: std::collections::HashMap<String, usize> = ids.iter().map(|id| (id.clone(), graph.predecessors(id).len())).collect();
    let mut ready: Vec<String> = in_degree.iter().filter(|(_, &degree)| degree == 0).map(|(id, _)| id.clone()).collect();
    ready.sort();

    let mut order = Vec::with_capacity(ids.len());
    while !ready.is_empty() {
        let next = ready.remove(0);
        order.push(next.clone());
        let mut newly_ready = Vec::new();
        for successor in graph.neighbors(&next) {
            let degree = in_degree.get_mut(successor).expect("successor must be one of the graph's registered ids");
            *degree -= 1;
            if *degree == 0 {
                newly_ready.push(successor.clone());
            }
        }
        newly_ready.sort();
        for id in newly_ready {
            let position = ready.binary_search(&id).unwrap_or_else(|position| position);
            ready.insert(position, id);
        }
    }

    if order.len() != ids.len() {
        let stuck: Vec<&String> = ids.iter().filter(|id| !order.contains(id)).collect();
        return Err(DbError::InvalidArgument(format!("projection dependency graph has a cycle among {stuck:?}")));
    }
    Ok(order)
}
//#endregion 🔖️Graph

//#region 🔖️Engine
/// @emoji 🎯️ Whether `projection` should actually run `apply` for this step: directly touched
/// (`ErasedProjection::affected_by`) OR cascaded to because a declared dependency ran THIS SAME
/// step (`changed_this_step`, populated in topological order so a dependency is always decided
/// before its dependents are checked). Called identically from `apply_envelope`'s persisted,
/// checkpoint-resuming path and `rebuild_in_memory`'s pure replay path — see the module doc's
/// "incremental triggering" design-choice note for why that shared call site is what makes the
/// rebuild == incremental-apply law hold by construction.
async fn should_run<E: ErasedProjection>(projection: &E, touched: &TouchedSet, changed_this_step: &std::collections::HashSet<&'static str>) -> bool {
    projection.affected_by(touched).await || projection.dependencies().await.iter().any(|dependency| changed_this_step.contains(dependency))
}

/// @emoji 🚂️ Drives a fixed set of registered `ErasedProjection`s for one document: incremental
/// per-command application with checkpoint persistence (`apply_envelope`), pure in-memory replay
/// (`rebuild_in_memory`, the ground truth `apply_envelope`'s persisted path is checked against),
/// checkpoint-recovery rebuild (`rebuild_and_persist`), historical lookup (`state_at`), and
/// preview augmentation (`preview_augmented`). Generic over `E: ErasedProjection` (dedyn-fw-os-
/// guestruntime, O1/R1) instead of `Vec<Box<dyn ErasedProjection>>` — see `ErasedProjection`'s own
/// doc for how a caller picks `E` (a single concrete `ErasedWrapper<P>`, or its own closed enum over
/// several).
pub struct ProjectionEngine<'a, S: IndexStorage, E: ErasedProjection> {
    storage: &'a S,
    document: ArtifactId,
    projections: Vec<E>,
    graph: ProjectionGraph,
}

impl<'a, S: IndexStorage, E: ErasedProjection> ProjectionEngine<'a, S, E> {
    /// @emoji 🏗️ Registers `projections` for `document` against `storage`, validating the
    /// dependency DAG up front (see `ProjectionGraph::build`) so a misconfigured cycle/dangling
    /// dependency fails at construction rather than mid-apply.
    pub async fn new(storage: &'a S, document: ArtifactId, projections: Vec<E>) -> Result<Self, DbError> {
        let graph = ProjectionGraph::build(&projections).await?;
        Ok(ProjectionEngine { storage, document, projections, graph })
    }

    /// @emoji 📋️ Every registered projection's id, in topological (dependency-respecting) order —
    /// exposed for callers/tests that want to observe or assert on the resolved DAG order.
    pub async fn topological_order(&self) -> Vec<&'static str> {
        {
            let mut ids = Vec::with_capacity(self.graph.order.len());
            for &index in &self.graph.order {
                ids.push(self.projections[index].id().await);
            }
            ids
        }
    }

    async fn projection_by_id(&self, projection_id: &str) -> Result<&E, DbError> {
        // 🔀️ `ErasedProjection::id` genuinely awaits (an open trait, R9 does not apply) — hoisted
        // out of `Iterator::find`'s sync closure (R10 residue shape 1) into an explicit loop.
        for projection in &self.projections {
            if projection.id().await == projection_id {
                return Ok(projection);
            }
        }
        Err(DbError::NotFound(format!("projection {projection_id:?} is not registered on this engine")))
    }

    async fn index_for(&self, projection_id: &str) -> ProjectionIndex<'a, S> {
        let _ = projection_id; // ProjectionIndex is per-document, not per-projection-id; kept as a parameter for call-site clarity.
        ProjectionIndex::new(self.storage, self.document.clone()).await
    }

    /// @emoji 🔓️ Decodes a raw `ProjectionIndex` value (version prefix + state bytes), rejecting a
    /// stale schema version with `DbError::Conflict` rather than misinterpreting incompatible
    /// bytes — the shared guard behind both `load_checkpoint` and `state_at`.
    async fn decode_checkpoint(&self, projection: &E, versioned_bytes: &[u8]) -> Result<Vec<u8>, DbError> {
        let (stored_version, state_bytes) = decode_versioned(versioned_bytes).await?;
        if stored_version != projection.schema_version().await {
            return Err(DbError::Conflict(format!("projection {:?} checkpoint schema version {stored_version} does not match registered version {} — rebuild required", projection.id().await, projection.schema_version().await)));
        }
        Ok(state_bytes.to_vec())
    }

    /// @emoji 📥️ `projection`'s persisted state at or before `at_or_before`, or its `initial()`
    /// bytes if nothing has been persisted at or before that point yet.
    async fn load_checkpoint(&self, projection: &E, at_or_before: u64) -> Result<Vec<u8>, DbError> {
        let index = self.index_for(projection.id().await).await;
        match index.latest_at_or_before(projection.id().await, at_or_before).await? {
            Some((_, versioned_bytes)) => self.decode_checkpoint(projection, &versioned_bytes).await,
            None => Ok(projection.initial_bytes().await),
        }
    }

    async fn require_matching_document(&self, envelope: &MutationEnvelope) -> Result<(), DbError> {
        if envelope.document_id.0 != self.document.0 {
            return Err(DbError::InvalidArgument(format!("envelope document {:?} does not match this engine's document {:?}", envelope.document_id.0, self.document.0)));
        }
        Ok(())
    }

    /// @emoji ⏩️ Applies one committed envelope at `command_seq` across every registered
    /// projection in dependency order: for each, `should_run` decides whether it is directly
    /// affected by `touched` or cascaded to via a dependency that ran this same step; if so, loads
    /// its checkpoint at `command_seq - 1` (or `initial()` if none), computes the next state via
    /// `ErasedProjection::apply_bytes`, and durably persists it under `command_seq` via
    /// `ProjectionIndex::record` before the next projection in the order runs (so a dependent sees
    /// a durably-recorded — not merely in-flight — dependency state). A projection `should_run`
    /// says no to is left exactly as it was (no write, no frontier advance) and its prior state is
    /// carried forward unchanged for `DepView`/the returned map. Returns every projection's
    /// (possibly carried-forward) state, keyed by id.
    pub async fn apply_envelope(&self, command_seq: u64, envelope: &MutationEnvelope, touched: &TouchedSet) -> Result<PMap<String, Vec<u8>>, DbError> {
        self.require_matching_document(envelope).await?;
        let mut deps = DepView::default();
        let mut out: PMap<String, Vec<u8>> = PMap::new();
        let mut changed_this_step: std::collections::HashSet<&'static str> = std::collections::HashSet::new();
        for &index in &self.graph.order {
            let projection = &self.projections[index];
            let prior = self.load_checkpoint(projection, command_seq.saturating_sub(1)).await?;
            let new_state = if should_run(projection, touched, &changed_this_step).await {
                let computed = projection.apply_bytes(&prior, envelope, &deps).await?;
                let index_handle = self.index_for(projection.id().await).await;
                index_handle.record(projection.id().await, command_seq, encode_versioned(projection.schema_version().await, &computed).await).await?;
                changed_this_step.insert(projection.id().await);
                computed
            } else {
                prior
            };
            deps.states = deps.states.insert(projection.id().await.to_string(), new_state.clone());
            out = out.insert(projection.id().await.to_string(), new_state);
        }
        Ok(out)
    }

    /// @emoji 🧮️ Pure, storage-independent replay: recomputes every registered projection from
    /// `ProjectionClass::initial()` through `events` (each an envelope paired with its touched
    /// regions, ordered oldest-first) without ever touching `IndexStorage`, gating every step with
    /// the exact same `should_run` call `apply_envelope` uses. This is the ground truth
    /// `apply_envelope`'s persisted, checkpoint-resuming path is checked against by the
    /// rebuild==incremental law (see `🧪️Tests::rebuild_equals_incremental_after_checkpoint_resume`).
    pub async fn rebuild_in_memory(&self, events: &[(u64, MutationEnvelope, TouchedSet)]) -> Result<PMap<String, Vec<u8>>, DbError> {
        let mut states: Vec<Vec<u8>> = {
            let mut v = Vec::with_capacity(self.projections.len());
            for projection in &self.projections {
                v.push(projection.initial_bytes().await);
            }
            v
        };
        for (_, envelope, touched) in events {
            self.require_matching_document(envelope).await?;
            let mut deps = DepView::default();
            let mut changed_this_step: std::collections::HashSet<&'static str> = std::collections::HashSet::new();
            for &index in &self.graph.order {
                let projection = &self.projections[index];
                let new_state = if should_run(projection, touched, &changed_this_step).await {
                    let computed = projection.apply_bytes(&states[index], envelope, &deps).await?;
                    changed_this_step.insert(projection.id().await);
                    computed
                } else {
                    states[index].clone()
                };
                deps.states = deps.states.insert(projection.id().await.to_string(), new_state.clone());
                states[index] = new_state;
            }
        }
        let mut out = PMap::new();
        for (index, projection) in self.projections.iter().enumerate() {
            out = out.insert(projection.id().await.to_string(), states[index].clone());
        }
        Ok(out)
    }

    /// @emoji 🛠️ `rebuild_in_memory` followed by durably re-persisting each projection's final
    /// state under `final_command_seq`, unconditionally overwriting any stale/incompatible
    /// checkpoint. This is the recovery path from a `DbError::Conflict` schema-version mismatch
    /// surfaced by `apply_envelope`/`state_at` — a projection whose `schema_version()` was bumped
    /// gets a fresh, current-version checkpoint by replaying its full history once.
    pub async fn rebuild_and_persist(&self, events: &[(u64, MutationEnvelope, TouchedSet)], final_command_seq: u64) -> Result<PMap<String, Vec<u8>>, DbError> {
        let final_states = self.rebuild_in_memory(events).await?;
        for projection in &self.projections {
            let id = projection.id().await.to_string();
            let bytes = final_states.get(&id).expect("rebuild_in_memory populates every registered projection id");
            let index_handle = self.index_for(projection.id().await).await;
            index_handle.record(projection.id().await, final_command_seq, encode_versioned(projection.schema_version().await, bytes).await).await?;
        }
        Ok(final_states)
    }

    /// @emoji 🏔️ Historical query: `projection_id`'s persisted state at or before `frontier_seq`
    /// (past its version prefix), or `Ok(None)` if nothing was ever persisted at or before that
    /// frontier. Errors `DbError::Conflict` if the nearest checkpoint's schema version is stale.
    pub async fn state_at(&self, projection_id: &str, frontier_seq: u64) -> Result<Option<Vec<u8>>, DbError> {
        let projection = self.projection_by_id(projection_id).await?;
        let index_handle = self.index_for(projection_id).await;
        match index_handle.latest_at_or_before(projection_id, frontier_seq).await? {
            None => Ok(None),
            Some((_, versioned_bytes)) => Ok(Some(self.decode_checkpoint(projection, &versioned_bytes).await?)),
        }
    }

    /// @emoji 🌫️ Preview-augmented query: `projection_id`'s canonical state at or before
    /// `base_frontier_seq` (its `initial()` bytes if nothing persisted yet), with
    /// `preview_envelope` applied on top — computed entirely in memory and returned, NEVER
    /// persisted (the contract's "previews are never durable" law: this method never calls
    /// `ProjectionIndex::record`/any `IndexStorage` write). Dependency states for the preview step
    /// are the DEPENDENCIES' own canonical state at `base_frontier_seq` — a preview augments one
    /// projection, it does not cascade a preview through the whole DAG.
    pub async fn preview_augmented(&self, projection_id: &str, base_frontier_seq: u64, preview_envelope: &MutationEnvelope) -> Result<Vec<u8>, DbError> {
        self.require_matching_document(preview_envelope).await?;
        let projection = self.projection_by_id(projection_id).await?;
        let base = match self.state_at(projection_id, base_frontier_seq).await? {
            Some(bytes) => bytes,
            None => projection.initial_bytes().await,
        };
        let mut deps = DepView::default();
        for &dependency_id in projection.dependencies().await {
            if let Some(dependency_bytes) = self.state_at(dependency_id, base_frontier_seq).await? {
                deps.states = deps.states.insert(dependency_id.to_string(), dependency_bytes);
            }
        }
        projection.apply_bytes(&base, preview_envelope, &deps).await
    }
}
//#endregion 🔖️Engine

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use db_storage::MemoryStorage;

    //#region 🔖️Fixtures
    /// @emoji 🔢️ A trivial counting projection: state is "how many times I've actually run" —
    /// ignores the envelope's content entirely (this crate is semantics-free, see the module doc),
    /// so a test can tell "did this projection run this step" apart from "did it not" purely from
    /// its state, independent of `reads()`/`dependencies()` gating.
    struct CounterProjection {
        id: &'static str,
        schema_version: u32,
        dependencies: &'static [&'static str],
        reads: &'static [&'static str],
    }

    impl ProjectionClass for CounterProjection {
        type State = u64;

        async fn id(&self) -> &'static str {
            self.id
        }

        async fn schema_version(&self) -> u32 {
            self.schema_version
        }

        async fn dependencies(&self) -> &'static [&'static str] {
            self.dependencies
        }

        async fn reads(&self) -> &'static [&'static str] {
            self.reads
        }

        async fn initial(&self) -> u64 {
            0
        }

        async fn apply(&self, state: &u64, _envelope: &MutationEnvelope, _deps: &DepView) -> Result<u64, DbError> {
            Ok(state + 1)
        }
    }

    /// @emoji ➕️ A projection that sums its own counter with a named dependency's counter each
    /// step it actually runs — exercises `DepView`/DAG ordering, not just a standalone projection.
    struct SumWithDependencyProjection {
        id: &'static str,
        dependency_id: &'static str,
        dependencies: &'static [&'static str],
        reads: &'static [&'static str],
    }

    impl ProjectionClass for SumWithDependencyProjection {
        type State = u64;

        async fn id(&self) -> &'static str {
            self.id
        }

        async fn schema_version(&self) -> u32 {
            1
        }

        async fn dependencies(&self) -> &'static [&'static str] {
            self.dependencies
        }

        async fn reads(&self) -> &'static [&'static str] {
            self.reads
        }

        async fn initial(&self) -> u64 {
            0
        }

        async fn apply(&self, state: &u64, _envelope: &MutationEnvelope, deps: &DepView) -> Result<u64, DbError> {
            let dependency_value: u64 = deps.get(self.dependency_id).await?.unwrap_or(0);
            Ok(state + 1 + dependency_value)
        }
    }

    /// @emoji 🎛️ dedyn-fw-os-guestruntime (O1/R1): the closed-set enum letting a `Vec` in this
    /// module's own tests mix `CounterProjection` AND `SumWithDependencyProjection` — the ONLY
    /// place, repo-wide, that genuinely needs an `ErasedProjection` erased into more than one
    /// concrete shape (see `ErasedProjection`'s own doc). Hand-written, not `#[dyn_enum]`/
    /// `dyn_enum_close!` (`semio-framework-dispatch-macros`), same call as `GuestRuntimes`
    /// (`🔌️plugin/🖥️host/🦀️component.rs`): wiring a brand-new proc-macro dependency into this
    /// crate's manifest for a 2-variant, 7-method, all-sync trait is more risk than the mechanical
    /// match-delegation below.
    enum AnyTestProjection {
        Counter(ErasedWrapper<CounterProjection>),
        SumWithDependency(ErasedWrapper<SumWithDependencyProjection>),
    }

    impl ErasedProjection for AnyTestProjection {
        async fn id(&self) -> &'static str {
            match self {
                Self::Counter(p) => p.id().await,
                Self::SumWithDependency(p) => p.id().await,
            }
        }

        async fn schema_version(&self) -> u32 {
            match self {
                Self::Counter(p) => p.schema_version().await,
                Self::SumWithDependency(p) => p.schema_version().await,
            }
        }

        async fn dependencies(&self) -> &'static [&'static str] {
            match self {
                Self::Counter(p) => p.dependencies().await,
                Self::SumWithDependency(p) => p.dependencies().await,
            }
        }

        async fn reads(&self) -> &'static [&'static str] {
            match self {
                Self::Counter(p) => p.reads().await,
                Self::SumWithDependency(p) => p.reads().await,
            }
        }

        async fn affected_by(&self, touched: &TouchedSet) -> bool {
            match self {
                Self::Counter(p) => p.affected_by(touched).await,
                Self::SumWithDependency(p) => p.affected_by(touched).await,
            }
        }

        async fn initial_bytes(&self) -> Vec<u8> {
            match self {
                Self::Counter(p) => p.initial_bytes().await,
                Self::SumWithDependency(p) => p.initial_bytes().await,
            }
        }

        async fn apply_bytes(&self, state_bytes: &[u8], envelope: &MutationEnvelope, deps: &DepView) -> Result<Vec<u8>, DbError> {
            match self {
                Self::Counter(p) => p.apply_bytes(state_bytes, envelope, deps).await,
                Self::SumWithDependency(p) => p.apply_bytes(state_bytes, envelope, deps).await,
            }
        }
    }

    impl From<ErasedWrapper<CounterProjection>> for AnyTestProjection {
        // 🚫️async: E1 impl of std::convert::From — signature fixed outside this repo; body is a
        // pure zero-suspension enum-variant wrap, see R9.
        fn from(wrapper: ErasedWrapper<CounterProjection>) -> Self {
            Self::Counter(wrapper)
        }
    }

    impl From<ErasedWrapper<SumWithDependencyProjection>> for AnyTestProjection {
        // 🚫️async: E1 impl of std::convert::From — signature fixed outside this repo; body is a
        // pure zero-suspension enum-variant wrap, see R9.
        fn from(wrapper: ErasedWrapper<SumWithDependencyProjection>) -> Self {
            Self::SumWithDependency(wrapper)
        }
    }

    async fn envelope(document: &str, operation: &str, seq: u64) -> MutationEnvelope {
        MutationEnvelope {
            mutation_id: protocol::MutationId(operation.to_string()),
            document_id: protocol::ArtifactId(document.to_string()),
            actor: protocol::ActorId("actor-1".to_string()),
            dependencies: Vec::new(),
            diff: protocol::ArtifactDiff { schema: protocol::SchemaId("test".to_string()), payload: Default::default() },
            inverse: protocol::InverseMutation { schema: protocol::SchemaId("test".to_string()), payload: Default::default() },
            timestamp: protocol::HybridLogicalTimestamp::new(1, seq).await,
        }
    }

    /// @emoji 👆️ Builds a `TouchedSet` recording a write against every one of `paths`.
    async fn touch(paths: &[&str]) -> TouchedSet {
        let mut touched = TouchedSet::new();
        for path in paths {
            touched.record(TouchedRegion::write(*path));
        }
        touched
    }
    //#endregion 🔖️Fixtures

    //#region 🔖️State
    #[semio_framework_async_macros::async_test]
    async fn versioned_round_trips_and_rejects_short_input() {
        let bytes = encode_versioned(7, &[1, 2, 3]).await;
        let (version, state) = decode_versioned(&bytes).await.unwrap();
        assert_eq!(version, 7);
        assert_eq!(state, &[1, 2, 3]);

        assert!(matches!(decode_versioned(&[0u8, 1, 2]).await, Err(DbError::Corrupt(_))));
    }

    #[semio_framework_async_macros::async_test]
    async fn u64_and_string_and_bytes_projection_states_round_trip() {
        assert_eq!(u64::decode(&42u64.encode().await).await.unwrap(), 42u64);
        assert_eq!(String::decode(&"hello".to_string().encode().await).await.unwrap(), "hello".to_string());
        assert_eq!(Vec::<u8>::decode(&vec![9u8, 8, 7].encode().await).await.unwrap(), vec![9u8, 8, 7]);
        assert!(matches!(u64::decode(&[1, 2, 3]).await, Err(DbError::Corrupt(_))));
    }
    //#endregion 🔖️State

    //#region 🔖️Graph
    #[semio_framework_async_macros::async_test]
    async fn topological_order_respects_dependency_edges() {
        let projections = vec![
            erase(CounterProjection { id: "b", schema_version: 1, dependencies: &["a"], reads: &[] }),
            erase(CounterProjection { id: "a", schema_version: 1, dependencies: &[], reads: &[] }),
            erase(CounterProjection { id: "c", schema_version: 1, dependencies: &["a", "b"], reads: &[] }),
        ];
        let storage = MemoryStorage::new().await;
        let engine = ProjectionEngine::new(&storage, "doc-1".into(), projections).await.unwrap();
        let order = engine.topological_order().await;
        let position = |id: &str| order.iter().position(|candidate| *candidate == id).unwrap();
        assert!(position("a") < position("b"));
        assert!(position("a") < position("c"));
        assert!(position("b") < position("c"));
    }

    #[semio_framework_async_macros::async_test]
    async fn build_rejects_duplicate_ids() {
        let projections = vec![erase(CounterProjection { id: "a", schema_version: 1, dependencies: &[], reads: &[] }), erase(CounterProjection { id: "a", schema_version: 1, dependencies: &[], reads: &[] })];
        let storage = MemoryStorage::new().await;
        assert!(matches!(ProjectionEngine::new(&storage, "doc-1".into(), projections).await, Err(DbError::AlreadyExists(_))));
    }

    #[semio_framework_async_macros::async_test]
    async fn build_rejects_unknown_dependency() {
        let projections = vec![erase(CounterProjection { id: "a", schema_version: 1, dependencies: &["ghost"], reads: &[] })];
        let storage = MemoryStorage::new().await;
        assert!(matches!(ProjectionEngine::new(&storage, "doc-1".into(), projections).await, Err(DbError::NotFound(_))));
    }

    #[semio_framework_async_macros::async_test]
    async fn build_rejects_a_dependency_cycle() {
        let projections = vec![erase(CounterProjection { id: "a", schema_version: 1, dependencies: &["b"], reads: &[] }), erase(CounterProjection { id: "b", schema_version: 1, dependencies: &["a"], reads: &[] })];
        let storage = MemoryStorage::new().await;
        assert!(matches!(ProjectionEngine::new(&storage, "doc-1".into(), projections).await, Err(DbError::InvalidArgument(_))));
    }
    //#endregion 🔖️Graph

    //#region 🔖️Engine
    #[semio_framework_async_macros::async_test]
    async fn apply_envelope_advances_and_persists_incrementally() {
        let projections = vec![erase(CounterProjection { id: "count", schema_version: 1, dependencies: &[], reads: &["doc"] })];
        let storage = MemoryStorage::new().await;
        let engine = ProjectionEngine::new(&storage, "doc-1".into(), projections).await.unwrap();

        for seq in 1..=3u64 {
            let result = db_actor::block_on(engine.apply_envelope(seq, &envelope("doc-1", &format!("op-{seq}"), seq).await, &touch(&["doc"]).await)).unwrap();
            assert_eq!(u64::decode(result.get(&"count".to_string()).unwrap()).await.unwrap(), seq);
        }

        let persisted = db_actor::block_on(engine.state_at("count", 3)).unwrap().unwrap();
        assert_eq!(u64::decode(&persisted).await.unwrap(), 3);
        assert_eq!(db_actor::block_on(engine.state_at("count", 1)).unwrap().map(|bytes| db_actor::block_on(u64::decode(&bytes)).unwrap()), Some(1));
        assert_eq!(db_actor::block_on(engine.state_at("count", 0)).unwrap(), None);
    }

    #[semio_framework_async_macros::async_test]
    async fn apply_envelope_rejects_a_mismatched_document() {
        let projections = vec![erase(CounterProjection { id: "count", schema_version: 1, dependencies: &[], reads: &["doc"] })];
        let storage = MemoryStorage::new().await;
        let engine = ProjectionEngine::new(&storage, "doc-1".into(), projections).await.unwrap();
        assert!(matches!(db_actor::block_on(engine.apply_envelope(1, &envelope("doc-OTHER", "op-1", 1).await, &touch(&["doc"]).await)), Err(DbError::InvalidArgument(_))));
    }

    #[semio_framework_async_macros::async_test]
    async fn dependent_projection_sees_its_dependencys_state_from_the_same_step() {
        let projections: Vec<AnyTestProjection> =
            vec![erase(SumWithDependencyProjection { id: "sum", dependency_id: "count", dependencies: &["count"], reads: &[] }).into(), erase(CounterProjection { id: "count", schema_version: 1, dependencies: &[], reads: &["doc"] }).into()];
        let storage = MemoryStorage::new().await;
        let engine = ProjectionEngine::new(&storage, "doc-1".into(), projections).await.unwrap();

        // Step 1: count -> 1, sum sees count's *this-step* value (1): sum = 1 + 1 = 2.
        let result = db_actor::block_on(engine.apply_envelope(1, &envelope("doc-1", "op-1", 1).await, &touch(&["doc"]).await)).unwrap();
        assert_eq!(u64::decode(result.get(&"count".to_string()).unwrap()).await.unwrap(), 1);
        assert_eq!(u64::decode(result.get(&"sum".to_string()).unwrap()).await.unwrap(), 2);

        // Step 2: count -> 2, sum = (prior sum 2) + 1 + (this-step count 2) = 5.
        let result = db_actor::block_on(engine.apply_envelope(2, &envelope("doc-1", "op-2", 2).await, &touch(&["doc"]).await)).unwrap();
        assert_eq!(u64::decode(result.get(&"count".to_string()).unwrap()).await.unwrap(), 2);
        assert_eq!(u64::decode(result.get(&"sum".to_string()).unwrap()).await.unwrap(), 5);
    }

    #[semio_framework_async_macros::async_test]
    async fn stale_schema_version_checkpoint_is_reported_as_conflict_not_misread() {
        let storage = MemoryStorage::new().await;
        {
            let projections = vec![erase(CounterProjection { id: "count", schema_version: 1, dependencies: &[], reads: &["doc"] })];
            let engine = ProjectionEngine::new(&storage, "doc-1".into(), projections).await.unwrap();
            db_actor::block_on(engine.apply_envelope(1, &envelope("doc-1", "op-1", 1).await, &touch(&["doc"]).await)).unwrap();
        }
        // A fresh engine registers the SAME projection id at a bumped schema version.
        let projections = vec![erase(CounterProjection { id: "count", schema_version: 2, dependencies: &[], reads: &["doc"] })];
        let engine = ProjectionEngine::new(&storage, "doc-1".into(), projections).await.unwrap();
        assert!(matches!(db_actor::block_on(engine.state_at("count", 1)), Err(DbError::Conflict(_))));
        assert!(matches!(db_actor::block_on(engine.apply_envelope(2, &envelope("doc-1", "op-2", 2).await, &touch(&["doc"]).await)), Err(DbError::Conflict(_))));

        // rebuild_and_persist recovers: replays from scratch and re-persists at the current version.
        let events = vec![(1u64, envelope("doc-1", "op-1", 1).await, touch(&["doc"]).await)];
        db_actor::block_on(engine.rebuild_and_persist(&events, 1)).unwrap();
        assert_eq!(db_actor::block_on(engine.state_at("count", 1)).unwrap().map(|bytes| db_actor::block_on(u64::decode(&bytes)).unwrap()), Some(1));
    }

    #[semio_framework_async_macros::async_test]
    async fn preview_augmented_never_persists_and_does_not_affect_canonical_state() {
        let projections = vec![erase(CounterProjection { id: "count", schema_version: 1, dependencies: &[], reads: &["doc"] })];
        let storage = MemoryStorage::new().await;
        let engine = ProjectionEngine::new(&storage, "doc-1".into(), projections).await.unwrap();
        db_actor::block_on(engine.apply_envelope(1, &envelope("doc-1", "op-1", 1).await, &touch(&["doc"]).await)).unwrap();

        let previewed = db_actor::block_on(engine.preview_augmented("count", 1, &envelope("doc-1", "preview-op", 2).await)).unwrap();
        assert_eq!(u64::decode(&previewed).await.unwrap(), 2);

        // Canonical state at the same frontier is untouched by the preview.
        assert_eq!(db_actor::block_on(engine.state_at("count", 1)).unwrap().map(|bytes| db_actor::block_on(u64::decode(&bytes)).unwrap()), Some(1));
        // And no checkpoint was ever recorded past seq 1 (the preview never persisted anything).
        assert_eq!(db_actor::block_on(engine.state_at("count", 2)).unwrap().map(|bytes| db_actor::block_on(u64::decode(&bytes)).unwrap()), Some(1));
    }
    //#endregion 🔖️Engine

    //#region 🔖️IncrementalTriggering
    #[semio_framework_async_macros::async_test]
    async fn apply_envelope_skips_a_projection_whose_reads_dont_intersect_the_touched_set() {
        let projections = vec![erase(CounterProjection { id: "counter", schema_version: 1, dependencies: &[], reads: &["counter"] })];
        let storage = MemoryStorage::new().await;
        let engine = ProjectionEngine::new(&storage, "doc-1".into(), projections).await.unwrap();

        // Untouched: the projection must not run, and nothing must be persisted for it.
        let result = db_actor::block_on(engine.apply_envelope(1, &envelope("doc-1", "op-1", 1).await, &touch(&["unrelated"]).await)).unwrap();
        assert_eq!(u64::decode(result.get(&"counter".to_string()).unwrap()).await.unwrap(), 0, "carried-forward initial state, not incremented");
        assert_eq!(db_actor::block_on(engine.state_at("counter", 1)).unwrap(), None, "an unaffected projection must not advance its frontier");

        // Directly touched: now it runs and persists.
        db_actor::block_on(engine.apply_envelope(2, &envelope("doc-1", "op-2", 2).await, &touch(&["counter"]).await)).unwrap();
        assert_eq!(db_actor::block_on(engine.state_at("counter", 1)).unwrap(), None, "still nothing at seq 1 — the skip was never retroactively persisted");
        assert_eq!(db_actor::block_on(engine.state_at("counter", 2)).unwrap().map(|bytes| db_actor::block_on(u64::decode(&bytes)).unwrap()), Some(1));
    }

    #[semio_framework_async_macros::async_test]
    async fn apply_envelope_cascades_to_a_dependent_with_no_reads_of_its_own() {
        let projections: Vec<AnyTestProjection> = vec![
            erase(CounterProjection { id: "counter", schema_version: 1, dependencies: &[], reads: &["counter"] }).into(),
            // "cascade" has an EMPTY reads() — per the module doc's design note, empty reads means
            // "not directly triggered by anything"; it must only ever run via the dependency cascade.
            erase(SumWithDependencyProjection { id: "cascade", dependency_id: "counter", dependencies: &["counter"], reads: &[] }).into(),
        ];
        let storage = MemoryStorage::new().await;
        let engine = ProjectionEngine::new(&storage, "doc-1".into(), projections).await.unwrap();

        // "counter" is untouched -> "cascade" has nothing to cascade from -> neither runs.
        db_actor::block_on(engine.apply_envelope(1, &envelope("doc-1", "op-1", 1).await, &touch(&["unrelated"]).await)).unwrap();
        assert_eq!(db_actor::block_on(engine.state_at("counter", 1)).unwrap(), None);
        assert_eq!(db_actor::block_on(engine.state_at("cascade", 1)).unwrap(), None);

        // "counter" is touched -> runs -> "cascade" cascades even though "unrelated" (not "counter")
        // is the only path in this step's touched set that "cascade" itself would ever have read.
        db_actor::block_on(engine.apply_envelope(2, &envelope("doc-1", "op-2", 2).await, &touch(&["counter"]).await)).unwrap();
        assert_eq!(db_actor::block_on(engine.state_at("counter", 2)).unwrap().map(|bytes| db_actor::block_on(u64::decode(&bytes)).unwrap()), Some(1));
        assert_eq!(db_actor::block_on(engine.state_at("cascade", 2)).unwrap().map(|bytes| db_actor::block_on(u64::decode(&bytes)).unwrap()), Some(2), "cascade = prior(0) + 1 + counter's this-step value(1) = 2");
    }
    //#endregion 🔖️IncrementalTriggering

    //#region 🔖️RebuildEqualsIncremental
    /// 🧪️ The core law: applying every event incrementally (each step reading its checkpoint via
    /// `load_checkpoint`, resuming from whatever was durably persisted, and gated by `should_run`
    /// against that step's touched set) must land on the exact same final state as a pure in-memory
    /// `rebuild_in_memory` replay of the same event sequence — including after an engine is dropped
    /// and reconstructed mid-stream so `apply_envelope` genuinely resumes from a persisted
    /// checkpoint rather than in-process memory, and including projections that are skipped on some
    /// steps and cascaded-to on others.
    #[semio_framework_async_macros::async_test]
    async fn rebuild_equals_incremental_after_checkpoint_resume() {
        let storage = MemoryStorage::new().await;
        let make_projections = || -> Vec<AnyTestProjection> {
            vec![
                erase(CounterProjection { id: "count", schema_version: 1, dependencies: &[], reads: &["doc"] }).into(),
                erase(SumWithDependencyProjection { id: "sum", dependency_id: "count", dependencies: &["count"], reads: &[] }).into(),
                erase(CounterProjection { id: "never", schema_version: 1, dependencies: &[], reads: &["never-touched"] }).into(),
            ]
        };

        // Alternating touched paths so "count"/"sum" run on some steps and are skipped on others —
        // "never" is never touched and has no dependents, so it should stay at its initial state.
        let touched_paths: [&[&str]; 5] = [&["doc"], &["unrelated"], &["doc"], &["doc", "unrelated"], &["unrelated"]];
        // 🪡 `.map` takes a sync closure, so the per-step async envelope/touch construction is
        // hoisted into an explicit loop instead of the original closure-chain shape.
        let mut events: Vec<(u64, MutationEnvelope, TouchedSet)> = Vec::new();
        for seq in 1..=5u64 {
            events.push((seq, envelope("doc-1", &format!("op-{seq}"), seq).await, touch(touched_paths[(seq - 1) as usize]).await));
        }

        // Incremental path: apply seqs 1-3 against one engine instance, drop it, then resume with a
        // FRESH engine instance (forcing seqs 4-5 to load their checkpoint from `storage`, not from
        // any in-memory state the first engine instance happened to hold).
        {
            let engine = ProjectionEngine::new(&storage, "doc-1".into(), make_projections()).await.unwrap();
            for (seq, env, touched) in &events[..3] {
                db_actor::block_on(engine.apply_envelope(*seq, env, touched)).unwrap();
            }
        }
        let incremental_final = {
            let engine = ProjectionEngine::new(&storage, "doc-1".into(), make_projections()).await.unwrap();
            let mut last = PMap::new();
            for (seq, env, touched) in &events[3..] {
                last = db_actor::block_on(engine.apply_envelope(*seq, env, touched)).unwrap();
            }
            last
        };

        // Ground-truth path: one pure in-memory replay of the whole history, touching no storage.
        let rebuild_engine = ProjectionEngine::new(&storage, "doc-1".into(), make_projections()).await.unwrap();
        let rebuilt_final = rebuild_engine.rebuild_in_memory(&events).await.unwrap();

        for id in ["count", "sum", "never"] {
            assert_eq!(incremental_final.get(&id.to_string()), rebuilt_final.get(&id.to_string()), "projection {id} diverged between checkpoint-resumed incremental application and full in-memory rebuild");
        }
        assert_eq!(u64::decode(rebuilt_final.get(&"never".to_string()).unwrap()).await.unwrap(), 0, "sanity: 'never' truly never ran");
    }
    //#endregion 🔖️RebuildEqualsIncremental
}
//#endregion 🧪️Tests
