//! 🗄️ `db_projection` — the `db` family's projection engine: typed, versioned projection classes
//! wired into a dependency DAG, applied incrementally as commands commit, checkpointed via
//! `db_index::ProjectionIndex` for historical ("state as of frontier X") queries, and augmentable
//! with an ephemeral, never-persisted preview overlay. Frozen contract:
//! `.🦑repo/🎫tickets/26/07/27/INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING/contract.md`
//! (`## db crate family`, `db_projection` row) and Part 2 of the approved plan.
//!
//! 🎯 Design choice: per the contract's "no db crate below `db_document` interprets operation
//! semantics" rule, this crate never looks inside a `protocol::OperationEnvelope`'s
//! `diff.schema`/`diff.payload` itself — every `ProjectionClass::apply` is supplied by a higher
//! layer (`db_document`, or an app-level program) that owns the actual domain interpretation. This
//! crate only owns the mechanism around that: dependency ordering, incremental application,
//! checkpoint persistence/versioning, historical lookup, and preview augmentation.
//!
//! 🎯 Design choice (dependency addition): the contract's per-crate deps table lists this crate as
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
//! 🎯 Design choice (versioning): `db_index::ProjectionIndex` stores opaque bytes with no side
//! channel for a schema version, so every checkpoint this crate persists is prefixed with its
//! `ProjectionClass::schema_version()` as 4 little-endian bytes (`🔖State`'s `encode_versioned`/
//! `decode_versioned`). Resuming from a checkpoint whose stored version differs from the
//! currently-registered version returns `DbError::Conflict` rather than silently misinterpreting
//! stale bytes — the caller's recovery path is `ProjectionEngine::rebuild_and_persist`.
//!
//! 🎯 Design choice (incremental triggering — appended alongside the two design choices above,
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

use db_core::{DbError, DocumentId};
use db_index::ProjectionIndex;
use db_state::{PGraph, PMap, TouchedRegion, TouchedSet};
use db_storage::IndexStorage;
use protocol::OperationEnvelope;

//#region 🔖State
/// @emoji 🧬 What a projection's in-memory state must support to round-trip through
/// `db_index::ProjectionIndex`'s opaque byte storage: a caller-defined, schema-specific
/// encode/decode pair. `db_projection` never interprets the bytes it stores — only the
/// `ProjectionClass` that produced them does (see the module doc's "no operation semantics" note).
pub trait ProjectionState: Clone + Send + Sync + 'static {
    fn encode(&self) -> Vec<u8>;
    fn decode(bytes: &[u8]) -> Result<Self, DbError>
    where
        Self: Sized;
}

/// @emoji 🔢 A ready-made `ProjectionState` for the common "just count/accumulate a `u64`" shape.
impl ProjectionState for u64 {
    fn encode(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }

    fn decode(bytes: &[u8]) -> Result<Self, DbError> {
        let array: [u8; 8] =
            bytes.try_into().map_err(|_| DbError::Corrupt("expected an 8-byte little-endian u64 projection state".to_string()))?;
        Ok(u64::from_le_bytes(array))
    }
}

/// @emoji 🔤 A ready-made `ProjectionState` for a UTF-8 text projection.
impl ProjectionState for String {
    fn encode(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }

    fn decode(bytes: &[u8]) -> Result<Self, DbError> {
        String::from_utf8(bytes.to_vec()).map_err(|_| DbError::Corrupt("projection state is not valid utf-8".to_string()))
    }
}

/// @emoji 📦 A ready-made `ProjectionState` for callers that already have their own opaque byte
/// encoding and just want the checkpoint machinery, not another codec layer.
impl ProjectionState for Vec<u8> {
    fn encode(&self) -> Vec<u8> {
        self.clone()
    }

    fn decode(bytes: &[u8]) -> Result<Self, DbError> {
        Ok(bytes.to_vec())
    }
}

/// @emoji 🏷️ How many bytes `encode_versioned` prepends — a fixed-width `u32` schema version tag
/// (see the module doc's versioning design-choice note).
const VERSION_PREFIX_LEN: usize = 4;

/// @emoji ✍️ Prefixes `state_bytes` with `schema_version` — the exact shape persisted via
/// `ProjectionIndex::record`.
fn encode_versioned(schema_version: u32, state_bytes: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(VERSION_PREFIX_LEN + state_bytes.len());
    out.extend_from_slice(&schema_version.to_le_bytes());
    out.extend_from_slice(state_bytes);
    out
}

/// @emoji 📖 Inverse of `encode_versioned`: splits the version prefix from the state bytes,
/// erroring `Corrupt` (never panicking) if `bytes` is shorter than the prefix itself.
fn decode_versioned(bytes: &[u8]) -> Result<(u32, &[u8]), DbError> {
    if bytes.len() < VERSION_PREFIX_LEN {
        return Err(DbError::Corrupt("projection checkpoint is shorter than its version prefix".to_string()));
    }
    let mut version_bytes = [0u8; VERSION_PREFIX_LEN];
    version_bytes.copy_from_slice(&bytes[..VERSION_PREFIX_LEN]);
    Ok((u32::from_le_bytes(version_bytes), &bytes[VERSION_PREFIX_LEN..]))
}
//#endregion 🔖State

//#region 🔖ProjectionClass
/// @emoji 👀 One step's view onto sibling projections' just-computed state, for a projection whose
/// `dependencies()` names them — `ProjectionGraph`'s topological order guarantees every dependency
/// has already run (and is present here) before a dependent's `apply` is called in the same step.
/// Scoped to raw bytes (rather than a typed map) so heterogeneous `ProjectionClass::State` types
/// can share one view without this crate needing a type-erased-but-downcastable value box.
#[derive(Clone, Default)]
pub struct DepView {
    states: PMap<String, Vec<u8>>,
}

impl DepView {
    /// @emoji 🔍 The raw encoded bytes a dependency computed this step, or `None` if `projection_id`
    /// isn't a registered dependency (or hasn't run yet — shouldn't happen given topological order).
    pub fn get_raw(&self, projection_id: &str) -> Option<&Vec<u8>> {
        self.states.get(&projection_id.to_string())
    }

    /// @emoji 🔍 `get_raw` decoded through `S::decode` — the typed convenience most `apply` impls
    /// reach for when they know a dependency's concrete `ProjectionState` shape by convention.
    pub fn get<S: ProjectionState>(&self, projection_id: &str) -> Result<Option<S>, DbError> {
        self.get_raw(projection_id).map(|bytes| S::decode(bytes)).transpose()
    }
}

/// @emoji 📽️ One projection's rules: identity, its schema version (bumping this invalidates every
/// previously-persisted checkpoint, see the module doc), which other projections it reads via
/// `DepView` during `apply`, its starting state, and the state-transition function itself. Never
/// interprets `envelope` beyond what the implementor chooses to (this crate stays semantics-free,
/// see the module doc).
pub trait ProjectionClass: Send + Sync {
    type State: ProjectionState;

    /// @emoji 🪪 A stable, unique-within-one-`ProjectionEngine` identifier.
    fn id(&self) -> &'static str;

    /// @emoji 🔢 This projection's current schema version (see the module doc's versioning note).
    fn schema_version(&self) -> u32;

    /// @emoji 🕸️ The ids of other registered projections this one reads via `DepView` — must all
    /// resolve to projections registered in the same `ProjectionEngine`, and must not form a cycle
    /// (`ProjectionGraph::build` validates both). Defaults to none.
    fn dependencies(&self) -> &'static [&'static str] {
        &[]
    }

    /// @emoji 👀 The document-region path prefixes this projection reads directly —
    /// `affected_by` checks these against a step's touched regions to decide whether this
    /// projection needs to run at all THIS step. Defaults to none, meaning "not directly triggered
    /// by anything" — a projection that should only ever run via a dependency's cascade (see
    /// `dependencies()`) leaves this empty; one that wants to see every step overrides
    /// `affected_by` directly rather than relying on any "empty means everything" implicit
    /// default (there is deliberately none, to avoid that footgun).
    fn reads(&self) -> &'static [&'static str] {
        &[]
    }

    /// @emoji 🎯 True iff `touched` should trigger this projection's `apply` on its own,
    /// independent of any dependency cascade (`ProjectionEngine`'s `should_run` layers that on
    /// top). Default: any declared `reads()` path intersects any touched region.
    fn affected_by(&self, touched: &TouchedSet) -> bool {
        let reads = self.reads();
        touched.regions.iter().any(|region| reads.iter().any(|&path| region.path_intersects(&TouchedRegion::read(path))))
    }

    /// @emoji 🌱 The state of a brand-new instance of this projection, before any envelope has
    /// been applied.
    fn initial(&self) -> Self::State;

    /// @emoji ⏩ Computes the next state from `state`, `envelope`, and this step's already-computed
    /// sibling states (`deps`).
    fn apply(&self, state: &Self::State, envelope: &OperationEnvelope, deps: &DepView) -> Result<Self::State, DbError>;
}

/// @emoji 🎭 The object-safe (dyn-compatible) view of a `ProjectionClass` — every method operates
/// on raw encoded bytes instead of the associated `State` type, so a `ProjectionEngine` can hold a
/// heterogeneous `Vec<Box<dyn ErasedProjection>>` of differently-typed projections. Implemented
/// automatically for any `ProjectionClass` via `erase`; not meant to be implemented by hand.
pub trait ErasedProjection: Send + Sync {
    fn id(&self) -> &'static str;
    fn schema_version(&self) -> u32;
    fn dependencies(&self) -> &'static [&'static str];
    fn reads(&self) -> &'static [&'static str];
    fn affected_by(&self, touched: &TouchedSet) -> bool;
    fn initial_bytes(&self) -> Vec<u8>;
    fn apply_bytes(&self, state_bytes: &[u8], envelope: &OperationEnvelope, deps: &DepView) -> Result<Vec<u8>, DbError>;
}

struct ErasedWrapper<P: ProjectionClass>(P);

impl<P: ProjectionClass> ErasedProjection for ErasedWrapper<P> {
    fn id(&self) -> &'static str {
        self.0.id()
    }

    fn schema_version(&self) -> u32 {
        self.0.schema_version()
    }

    fn dependencies(&self) -> &'static [&'static str] {
        self.0.dependencies()
    }

    fn reads(&self) -> &'static [&'static str] {
        self.0.reads()
    }

    fn affected_by(&self, touched: &TouchedSet) -> bool {
        self.0.affected_by(touched)
    }

    fn initial_bytes(&self) -> Vec<u8> {
        self.0.initial().encode()
    }

    fn apply_bytes(&self, state_bytes: &[u8], envelope: &OperationEnvelope, deps: &DepView) -> Result<Vec<u8>, DbError> {
        let state = P::State::decode(state_bytes)?;
        Ok(self.0.apply(&state, envelope, deps)?.encode())
    }
}

/// @emoji 🎁 Wraps a concrete `ProjectionClass` into its object-safe `ErasedProjection` form, for
/// registering into a `ProjectionEngine` alongside differently-typed projections.
pub fn erase<P: ProjectionClass + 'static>(class: P) -> Box<dyn ErasedProjection> {
    Box::new(ErasedWrapper(class))
}
//#endregion 🔖ProjectionClass

//#region 🔖Graph
/// @emoji 🕸️ The dependency-validated, topologically-sorted view of a set of registered
/// projections — built once at `ProjectionEngine::new` and reused by every subsequent apply/
/// rebuild pass so per-step ordering work is O(1) instead of a fresh sort each time.
struct ProjectionGraph {
    /// @emoji 🔢 Indices into the owning `ProjectionEngine`'s `projections` vec, in an order where
    /// every dependency appears before every projection that depends on it.
    order: Vec<usize>,
}

impl ProjectionGraph {
    /// @emoji 🏗️ Validates that every id is unique, every `dependencies()` entry resolves to a
    /// registered projection, and the dependency relation is acyclic — then computes one
    /// deterministic topological order (ties broken lexicographically by id, so the same
    /// registration set always yields the same order across runs/processes).
    fn build(projections: &[Box<dyn ErasedProjection>]) -> Result<ProjectionGraph, DbError> {
        let mut graph: PGraph<String, (), ()> = PGraph::new();
        let mut ids: Vec<String> = Vec::with_capacity(projections.len());
        for projection in projections {
            let id = projection.id().to_string();
            if graph.contains_node(&id) {
                return Err(DbError::AlreadyExists(format!("projection id {id:?} is registered more than once")));
            }
            graph = graph.add_node(id.clone(), ());
            ids.push(id);
        }
        for projection in projections {
            let id = projection.id().to_string();
            for &dependency in projection.dependencies() {
                if !graph.contains_node(&dependency.to_string()) {
                    return Err(DbError::NotFound(format!(
                        "projection {id:?} depends on unregistered projection {dependency:?}"
                    )));
                }
                graph = graph
                    .add_edge(dependency.to_string(), id.clone(), ())
                    .expect("both endpoints were just validated present in the graph");
            }
        }

        let mut index_of_id: std::collections::HashMap<String, usize> = std::collections::HashMap::with_capacity(ids.len());
        for (index, projection) in projections.iter().enumerate() {
            index_of_id.insert(projection.id().to_string(), index);
        }

        let sorted_ids = topological_sort(&graph, &ids)?;
        let order = sorted_ids.into_iter().map(|id| index_of_id[&id]).collect();
        Ok(ProjectionGraph { order })
    }
}

/// @emoji 🌀 Kahn's algorithm over `graph` restricted to `ids`: repeatedly takes the
/// lexicographically-smallest zero-in-degree node, appends it, and decrements its successors'
/// in-degree. `ids.len() > order.len()` at the end means a cycle remains (some node's in-degree
/// never reached zero) — reported as `DbError::InvalidArgument` rather than silently truncating.
fn topological_sort(graph: &PGraph<String, (), ()>, ids: &[String]) -> Result<Vec<String>, DbError> {
    let mut in_degree: std::collections::HashMap<String, usize> =
        ids.iter().map(|id| (id.clone(), graph.predecessors(id).len())).collect();
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
//#endregion 🔖Graph

//#region 🔖Engine
/// @emoji 🎯 Whether `projection` should actually run `apply` for this step: directly touched
/// (`ErasedProjection::affected_by`) OR cascaded to because a declared dependency ran THIS SAME
/// step (`changed_this_step`, populated in topological order so a dependency is always decided
/// before its dependents are checked). Called identically from `apply_envelope`'s persisted,
/// checkpoint-resuming path and `rebuild_in_memory`'s pure replay path — see the module doc's
/// "incremental triggering" design-choice note for why that shared call site is what makes the
/// rebuild == incremental-apply law hold by construction.
fn should_run(projection: &dyn ErasedProjection, touched: &TouchedSet, changed_this_step: &std::collections::HashSet<&'static str>) -> bool {
    projection.affected_by(touched) || projection.dependencies().iter().any(|dependency| changed_this_step.contains(dependency))
}

/// @emoji 🚂 Drives a fixed set of registered `ErasedProjection`s for one document: incremental
/// per-command application with checkpoint persistence (`apply_envelope`), pure in-memory replay
/// (`rebuild_in_memory`, the ground truth `apply_envelope`'s persisted path is checked against),
/// checkpoint-recovery rebuild (`rebuild_and_persist`), historical lookup (`state_at`), and
/// preview augmentation (`preview_augmented`).
pub struct ProjectionEngine<'a> {
    storage: &'a dyn IndexStorage,
    document: DocumentId,
    projections: Vec<Box<dyn ErasedProjection>>,
    graph: ProjectionGraph,
}

impl<'a> ProjectionEngine<'a> {
    /// @emoji 🏗️ Registers `projections` for `document` against `storage`, validating the
    /// dependency DAG up front (see `ProjectionGraph::build`) so a misconfigured cycle/dangling
    /// dependency fails at construction rather than mid-apply.
    pub fn new(storage: &'a dyn IndexStorage, document: DocumentId, projections: Vec<Box<dyn ErasedProjection>>) -> Result<Self, DbError> {
        let graph = ProjectionGraph::build(&projections)?;
        Ok(ProjectionEngine { storage, document, projections, graph })
    }

    /// @emoji 📋 Every registered projection's id, in topological (dependency-respecting) order —
    /// exposed for callers/tests that want to observe or assert on the resolved DAG order.
    pub fn topological_order(&self) -> Vec<&'static str> {
        self.graph.order.iter().map(|&index| self.projections[index].id()).collect()
    }

    fn projection_by_id(&self, projection_id: &str) -> Result<&dyn ErasedProjection, DbError> {
        self.projections
            .iter()
            .find(|projection| projection.id() == projection_id)
            .map(|projection| projection.as_ref())
            .ok_or_else(|| DbError::NotFound(format!("projection {projection_id:?} is not registered on this engine")))
    }

    fn index_for(&self, projection_id: &str) -> ProjectionIndex<'a> {
        let _ = projection_id; // ProjectionIndex is per-document, not per-projection-id; kept as a parameter for call-site clarity.
        ProjectionIndex::new(self.storage, self.document.clone())
    }

    /// @emoji 🔓 Decodes a raw `ProjectionIndex` value (version prefix + state bytes), rejecting a
    /// stale schema version with `DbError::Conflict` rather than misinterpreting incompatible
    /// bytes — the shared guard behind both `load_checkpoint` and `state_at`.
    fn decode_checkpoint(&self, projection: &dyn ErasedProjection, versioned_bytes: &[u8]) -> Result<Vec<u8>, DbError> {
        let (stored_version, state_bytes) = decode_versioned(versioned_bytes)?;
        if stored_version != projection.schema_version() {
            return Err(DbError::Conflict(format!(
                "projection {:?} checkpoint schema version {stored_version} does not match registered version {} — rebuild required",
                projection.id(),
                projection.schema_version()
            )));
        }
        Ok(state_bytes.to_vec())
    }

    /// @emoji 📥 `projection`'s persisted state at or before `at_or_before`, or its `initial()`
    /// bytes if nothing has been persisted at or before that point yet.
    fn load_checkpoint(&self, projection: &dyn ErasedProjection, at_or_before: u64) -> Result<Vec<u8>, DbError> {
        let index = self.index_for(projection.id());
        match index.latest_at_or_before(projection.id(), at_or_before)? {
            Some((_, versioned_bytes)) => self.decode_checkpoint(projection, &versioned_bytes),
            None => Ok(projection.initial_bytes()),
        }
    }

    fn require_matching_document(&self, envelope: &OperationEnvelope) -> Result<(), DbError> {
        if envelope.document_id.0 != self.document.0 {
            return Err(DbError::InvalidArgument(format!(
                "envelope document {:?} does not match this engine's document {:?}",
                envelope.document_id.0, self.document.0
            )));
        }
        Ok(())
    }

    /// @emoji ⏩ Applies one committed envelope at `command_seq` across every registered
    /// projection in dependency order: for each, `should_run` decides whether it is directly
    /// affected by `touched` or cascaded to via a dependency that ran this same step; if so, loads
    /// its checkpoint at `command_seq - 1` (or `initial()` if none), computes the next state via
    /// `ErasedProjection::apply_bytes`, and durably persists it under `command_seq` via
    /// `ProjectionIndex::record` before the next projection in the order runs (so a dependent sees
    /// a durably-recorded — not merely in-flight — dependency state). A projection `should_run`
    /// says no to is left exactly as it was (no write, no frontier advance) and its prior state is
    /// carried forward unchanged for `DepView`/the returned map. Returns every projection's
    /// (possibly carried-forward) state, keyed by id.
    pub fn apply_envelope(&self, command_seq: u64, envelope: &OperationEnvelope, touched: &TouchedSet) -> Result<PMap<String, Vec<u8>>, DbError> {
        self.require_matching_document(envelope)?;
        let mut deps = DepView::default();
        let mut out: PMap<String, Vec<u8>> = PMap::new();
        let mut changed_this_step: std::collections::HashSet<&'static str> = std::collections::HashSet::new();
        for &index in &self.graph.order {
            let projection = self.projections[index].as_ref();
            let prior = self.load_checkpoint(projection, command_seq.saturating_sub(1))?;
            let new_state = if should_run(projection, touched, &changed_this_step) {
                let computed = projection.apply_bytes(&prior, envelope, &deps)?;
                let index_handle = self.index_for(projection.id());
                index_handle.record(projection.id(), command_seq, encode_versioned(projection.schema_version(), &computed))?;
                changed_this_step.insert(projection.id());
                computed
            } else {
                prior
            };
            deps.states = deps.states.insert(projection.id().to_string(), new_state.clone());
            out = out.insert(projection.id().to_string(), new_state);
        }
        Ok(out)
    }

    /// @emoji 🧮 Pure, storage-independent replay: recomputes every registered projection from
    /// `ProjectionClass::initial()` through `events` (each an envelope paired with its touched
    /// regions, ordered oldest-first) without ever touching `IndexStorage`, gating every step with
    /// the exact same `should_run` call `apply_envelope` uses. This is the ground truth
    /// `apply_envelope`'s persisted, checkpoint-resuming path is checked against by the
    /// rebuild==incremental law (see `🧪Tests::rebuild_equals_incremental_after_checkpoint_resume`).
    pub fn rebuild_in_memory(&self, events: &[(u64, OperationEnvelope, TouchedSet)]) -> Result<PMap<String, Vec<u8>>, DbError> {
        let mut states: Vec<Vec<u8>> = self.projections.iter().map(|projection| projection.initial_bytes()).collect();
        for (_, envelope, touched) in events {
            self.require_matching_document(envelope)?;
            let mut deps = DepView::default();
            let mut changed_this_step: std::collections::HashSet<&'static str> = std::collections::HashSet::new();
            for &index in &self.graph.order {
                let projection = self.projections[index].as_ref();
                let new_state = if should_run(projection, touched, &changed_this_step) {
                    let computed = projection.apply_bytes(&states[index], envelope, &deps)?;
                    changed_this_step.insert(projection.id());
                    computed
                } else {
                    states[index].clone()
                };
                deps.states = deps.states.insert(projection.id().to_string(), new_state.clone());
                states[index] = new_state;
            }
        }
        let mut out = PMap::new();
        for (index, projection) in self.projections.iter().enumerate() {
            out = out.insert(projection.id().to_string(), states[index].clone());
        }
        Ok(out)
    }

    /// @emoji 🛠️ `rebuild_in_memory` followed by durably re-persisting each projection's final
    /// state under `final_command_seq`, unconditionally overwriting any stale/incompatible
    /// checkpoint. This is the recovery path from a `DbError::Conflict` schema-version mismatch
    /// surfaced by `apply_envelope`/`state_at` — a projection whose `schema_version()` was bumped
    /// gets a fresh, current-version checkpoint by replaying its full history once.
    pub fn rebuild_and_persist(&self, events: &[(u64, OperationEnvelope, TouchedSet)], final_command_seq: u64) -> Result<PMap<String, Vec<u8>>, DbError> {
        let final_states = self.rebuild_in_memory(events)?;
        for projection in &self.projections {
            let id = projection.id().to_string();
            let bytes = final_states.get(&id).expect("rebuild_in_memory populates every registered projection id");
            let index_handle = self.index_for(projection.id());
            index_handle.record(projection.id(), final_command_seq, encode_versioned(projection.schema_version(), bytes))?;
        }
        Ok(final_states)
    }

    /// @emoji 🏔️ Historical query: `projection_id`'s persisted state at or before `frontier_seq`
    /// (past its version prefix), or `Ok(None)` if nothing was ever persisted at or before that
    /// frontier. Errors `DbError::Conflict` if the nearest checkpoint's schema version is stale.
    pub fn state_at(&self, projection_id: &str, frontier_seq: u64) -> Result<Option<Vec<u8>>, DbError> {
        let projection = self.projection_by_id(projection_id)?;
        let index_handle = self.index_for(projection_id);
        match index_handle.latest_at_or_before(projection_id, frontier_seq)? {
            None => Ok(None),
            Some((_, versioned_bytes)) => Ok(Some(self.decode_checkpoint(projection, &versioned_bytes)?)),
        }
    }

    /// @emoji 🌫️ Preview-augmented query: `projection_id`'s canonical state at or before
    /// `base_frontier_seq` (its `initial()` bytes if nothing persisted yet), with
    /// `preview_envelope` applied on top — computed entirely in memory and returned, NEVER
    /// persisted (the contract's "previews are never durable" law: this method never calls
    /// `ProjectionIndex::record`/any `IndexStorage` write). Dependency states for the preview step
    /// are the DEPENDENCIES' own canonical state at `base_frontier_seq` — a preview augments one
    /// projection, it does not cascade a preview through the whole DAG.
    pub fn preview_augmented(&self, projection_id: &str, base_frontier_seq: u64, preview_envelope: &OperationEnvelope) -> Result<Vec<u8>, DbError> {
        self.require_matching_document(preview_envelope)?;
        let projection = self.projection_by_id(projection_id)?;
        let base = match self.state_at(projection_id, base_frontier_seq)? {
            Some(bytes) => bytes,
            None => projection.initial_bytes(),
        };
        let mut deps = DepView::default();
        for &dependency_id in projection.dependencies() {
            if let Some(dependency_bytes) = self.state_at(dependency_id, base_frontier_seq)? {
                deps.states = deps.states.insert(dependency_id.to_string(), dependency_bytes);
            }
        }
        projection.apply_bytes(&base, preview_envelope, &deps)
    }
}
//#endregion 🔖Engine

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use db_storage::MemoryStorage;

    //#region 🔖Fixtures
    /// @emoji 🔢 A trivial counting projection: state is "how many times I've actually run" —
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

        fn id(&self) -> &'static str {
            self.id
        }

        fn schema_version(&self) -> u32 {
            self.schema_version
        }

        fn dependencies(&self) -> &'static [&'static str] {
            self.dependencies
        }

        fn reads(&self) -> &'static [&'static str] {
            self.reads
        }

        fn initial(&self) -> u64 {
            0
        }

        fn apply(&self, state: &u64, _envelope: &OperationEnvelope, _deps: &DepView) -> Result<u64, DbError> {
            Ok(state + 1)
        }
    }

    /// @emoji ➕ A projection that sums its own counter with a named dependency's counter each
    /// step it actually runs — exercises `DepView`/DAG ordering, not just a standalone projection.
    struct SumWithDependencyProjection {
        id: &'static str,
        dependency_id: &'static str,
        dependencies: &'static [&'static str],
        reads: &'static [&'static str],
    }

    impl ProjectionClass for SumWithDependencyProjection {
        type State = u64;

        fn id(&self) -> &'static str {
            self.id
        }

        fn schema_version(&self) -> u32 {
            1
        }

        fn dependencies(&self) -> &'static [&'static str] {
            self.dependencies
        }

        fn reads(&self) -> &'static [&'static str] {
            self.reads
        }

        fn initial(&self) -> u64 {
            0
        }

        fn apply(&self, state: &u64, _envelope: &OperationEnvelope, deps: &DepView) -> Result<u64, DbError> {
            let dependency_value: u64 = deps.get(self.dependency_id)?.unwrap_or(0);
            Ok(state + 1 + dependency_value)
        }
    }

    fn envelope(document: &str, operation: &str, seq: u64) -> OperationEnvelope {
        OperationEnvelope {
            operation_id: protocol::OperationId(operation.to_string()),
            document_id: protocol::DocumentId(document.to_string()),
            actor: protocol::ActorId("actor-1".to_string()),
            dependencies: Vec::new(),
            diff: protocol::DocumentDiff { schema: protocol::SchemaId("test".to_string()), payload: Default::default() },
            inverse: protocol::InverseOperation { schema: protocol::SchemaId("test".to_string()), payload: Default::default() },
            timestamp: protocol::HybridLogicalTimestamp::new(1, seq),
        }
    }

    /// @emoji 👆 Builds a `TouchedSet` recording a write against every one of `paths`.
    fn touch(paths: &[&str]) -> TouchedSet {
        let mut touched = TouchedSet::new();
        for path in paths {
            touched.record(TouchedRegion::write(*path));
        }
        touched
    }
    //#endregion 🔖Fixtures

    //#region 🔖State
    #[test]
    fn versioned_round_trips_and_rejects_short_input() {
        let bytes = encode_versioned(7, &[1, 2, 3]);
        let (version, state) = decode_versioned(&bytes).unwrap();
        assert_eq!(version, 7);
        assert_eq!(state, &[1, 2, 3]);

        assert!(matches!(decode_versioned(&[0u8, 1, 2]), Err(DbError::Corrupt(_))));
    }

    #[test]
    fn u64_and_string_and_bytes_projection_states_round_trip() {
        assert_eq!(u64::decode(&42u64.encode()).unwrap(), 42u64);
        assert_eq!(String::decode(&"hello".to_string().encode()).unwrap(), "hello".to_string());
        assert_eq!(Vec::<u8>::decode(&vec![9u8, 8, 7].encode()).unwrap(), vec![9u8, 8, 7]);
        assert!(matches!(u64::decode(&[1, 2, 3]), Err(DbError::Corrupt(_))));
    }
    //#endregion 🔖State

    //#region 🔖Graph
    #[test]
    fn topological_order_respects_dependency_edges() {
        let projections: Vec<Box<dyn ErasedProjection>> = vec![
            erase(CounterProjection { id: "b", schema_version: 1, dependencies: &["a"], reads: &[] }),
            erase(CounterProjection { id: "a", schema_version: 1, dependencies: &[], reads: &[] }),
            erase(CounterProjection { id: "c", schema_version: 1, dependencies: &["a", "b"], reads: &[] }),
        ];
        let storage = MemoryStorage::new();
        let engine = ProjectionEngine::new(&storage, "doc-1".into(), projections).unwrap();
        let order = engine.topological_order();
        let position = |id: &str| order.iter().position(|candidate| *candidate == id).unwrap();
        assert!(position("a") < position("b"));
        assert!(position("a") < position("c"));
        assert!(position("b") < position("c"));
    }

    #[test]
    fn build_rejects_duplicate_ids() {
        let projections: Vec<Box<dyn ErasedProjection>> = vec![
            erase(CounterProjection { id: "a", schema_version: 1, dependencies: &[], reads: &[] }),
            erase(CounterProjection { id: "a", schema_version: 1, dependencies: &[], reads: &[] }),
        ];
        let storage = MemoryStorage::new();
        assert!(matches!(ProjectionEngine::new(&storage, "doc-1".into(), projections), Err(DbError::AlreadyExists(_))));
    }

    #[test]
    fn build_rejects_unknown_dependency() {
        let projections: Vec<Box<dyn ErasedProjection>> =
            vec![erase(CounterProjection { id: "a", schema_version: 1, dependencies: &["ghost"], reads: &[] })];
        let storage = MemoryStorage::new();
        assert!(matches!(ProjectionEngine::new(&storage, "doc-1".into(), projections), Err(DbError::NotFound(_))));
    }

    #[test]
    fn build_rejects_a_dependency_cycle() {
        let projections: Vec<Box<dyn ErasedProjection>> = vec![
            erase(CounterProjection { id: "a", schema_version: 1, dependencies: &["b"], reads: &[] }),
            erase(CounterProjection { id: "b", schema_version: 1, dependencies: &["a"], reads: &[] }),
        ];
        let storage = MemoryStorage::new();
        assert!(matches!(ProjectionEngine::new(&storage, "doc-1".into(), projections), Err(DbError::InvalidArgument(_))));
    }
    //#endregion 🔖Graph

    //#region 🔖Engine
    #[test]
    fn apply_envelope_advances_and_persists_incrementally() {
        let projections: Vec<Box<dyn ErasedProjection>> =
            vec![erase(CounterProjection { id: "count", schema_version: 1, dependencies: &[], reads: &["doc"] })];
        let storage = MemoryStorage::new();
        let engine = ProjectionEngine::new(&storage, "doc-1".into(), projections).unwrap();

        for seq in 1..=3u64 {
            let result = engine.apply_envelope(seq, &envelope("doc-1", &format!("op-{seq}"), seq), &touch(&["doc"])).unwrap();
            assert_eq!(u64::decode(result.get(&"count".to_string()).unwrap()).unwrap(), seq);
        }

        let persisted = engine.state_at("count", 3).unwrap().unwrap();
        assert_eq!(u64::decode(&persisted).unwrap(), 3);
        assert_eq!(engine.state_at("count", 1).unwrap().map(|bytes| u64::decode(&bytes).unwrap()), Some(1));
        assert_eq!(engine.state_at("count", 0).unwrap(), None);
    }

    #[test]
    fn apply_envelope_rejects_a_mismatched_document() {
        let projections: Vec<Box<dyn ErasedProjection>> =
            vec![erase(CounterProjection { id: "count", schema_version: 1, dependencies: &[], reads: &["doc"] })];
        let storage = MemoryStorage::new();
        let engine = ProjectionEngine::new(&storage, "doc-1".into(), projections).unwrap();
        assert!(matches!(engine.apply_envelope(1, &envelope("doc-OTHER", "op-1", 1), &touch(&["doc"])), Err(DbError::InvalidArgument(_))));
    }

    #[test]
    fn dependent_projection_sees_its_dependencys_state_from_the_same_step() {
        let projections: Vec<Box<dyn ErasedProjection>> = vec![
            erase(SumWithDependencyProjection { id: "sum", dependency_id: "count", dependencies: &["count"], reads: &[] }),
            erase(CounterProjection { id: "count", schema_version: 1, dependencies: &[], reads: &["doc"] }),
        ];
        let storage = MemoryStorage::new();
        let engine = ProjectionEngine::new(&storage, "doc-1".into(), projections).unwrap();

        // Step 1: count -> 1, sum sees count's *this-step* value (1): sum = 1 + 1 = 2.
        let result = engine.apply_envelope(1, &envelope("doc-1", "op-1", 1), &touch(&["doc"])).unwrap();
        assert_eq!(u64::decode(result.get(&"count".to_string()).unwrap()).unwrap(), 1);
        assert_eq!(u64::decode(result.get(&"sum".to_string()).unwrap()).unwrap(), 2);

        // Step 2: count -> 2, sum = (prior sum 2) + 1 + (this-step count 2) = 5.
        let result = engine.apply_envelope(2, &envelope("doc-1", "op-2", 2), &touch(&["doc"])).unwrap();
        assert_eq!(u64::decode(result.get(&"count".to_string()).unwrap()).unwrap(), 2);
        assert_eq!(u64::decode(result.get(&"sum".to_string()).unwrap()).unwrap(), 5);
    }

    #[test]
    fn stale_schema_version_checkpoint_is_reported_as_conflict_not_misread() {
        let storage = MemoryStorage::new();
        {
            let projections: Vec<Box<dyn ErasedProjection>> =
                vec![erase(CounterProjection { id: "count", schema_version: 1, dependencies: &[], reads: &["doc"] })];
            let engine = ProjectionEngine::new(&storage, "doc-1".into(), projections).unwrap();
            engine.apply_envelope(1, &envelope("doc-1", "op-1", 1), &touch(&["doc"])).unwrap();
        }
        // A fresh engine registers the SAME projection id at a bumped schema version.
        let projections: Vec<Box<dyn ErasedProjection>> =
            vec![erase(CounterProjection { id: "count", schema_version: 2, dependencies: &[], reads: &["doc"] })];
        let engine = ProjectionEngine::new(&storage, "doc-1".into(), projections).unwrap();
        assert!(matches!(engine.state_at("count", 1), Err(DbError::Conflict(_))));
        assert!(matches!(engine.apply_envelope(2, &envelope("doc-1", "op-2", 2), &touch(&["doc"])), Err(DbError::Conflict(_))));

        // rebuild_and_persist recovers: replays from scratch and re-persists at the current version.
        let events = vec![(1u64, envelope("doc-1", "op-1", 1), touch(&["doc"]))];
        engine.rebuild_and_persist(&events, 1).unwrap();
        assert_eq!(engine.state_at("count", 1).unwrap().map(|bytes| u64::decode(&bytes).unwrap()), Some(1));
    }

    #[test]
    fn preview_augmented_never_persists_and_does_not_affect_canonical_state() {
        let projections: Vec<Box<dyn ErasedProjection>> =
            vec![erase(CounterProjection { id: "count", schema_version: 1, dependencies: &[], reads: &["doc"] })];
        let storage = MemoryStorage::new();
        let engine = ProjectionEngine::new(&storage, "doc-1".into(), projections).unwrap();
        engine.apply_envelope(1, &envelope("doc-1", "op-1", 1), &touch(&["doc"])).unwrap();

        let previewed = engine.preview_augmented("count", 1, &envelope("doc-1", "preview-op", 2)).unwrap();
        assert_eq!(u64::decode(&previewed).unwrap(), 2);

        // Canonical state at the same frontier is untouched by the preview.
        assert_eq!(engine.state_at("count", 1).unwrap().map(|bytes| u64::decode(&bytes).unwrap()), Some(1));
        // And no checkpoint was ever recorded past seq 1 (the preview never persisted anything).
        assert_eq!(engine.state_at("count", 2).unwrap().map(|bytes| u64::decode(&bytes).unwrap()), Some(1));
    }
    //#endregion 🔖Engine

    //#region 🔖IncrementalTriggering
    #[test]
    fn apply_envelope_skips_a_projection_whose_reads_dont_intersect_the_touched_set() {
        let projections: Vec<Box<dyn ErasedProjection>> =
            vec![erase(CounterProjection { id: "counter", schema_version: 1, dependencies: &[], reads: &["counter"] })];
        let storage = MemoryStorage::new();
        let engine = ProjectionEngine::new(&storage, "doc-1".into(), projections).unwrap();

        // Untouched: the projection must not run, and nothing must be persisted for it.
        let result = engine.apply_envelope(1, &envelope("doc-1", "op-1", 1), &touch(&["unrelated"])).unwrap();
        assert_eq!(u64::decode(result.get(&"counter".to_string()).unwrap()).unwrap(), 0, "carried-forward initial state, not incremented");
        assert_eq!(engine.state_at("counter", 1).unwrap(), None, "an unaffected projection must not advance its frontier");

        // Directly touched: now it runs and persists.
        engine.apply_envelope(2, &envelope("doc-1", "op-2", 2), &touch(&["counter"])).unwrap();
        assert_eq!(engine.state_at("counter", 1).unwrap(), None, "still nothing at seq 1 — the skip was never retroactively persisted");
        assert_eq!(engine.state_at("counter", 2).unwrap().map(|bytes| u64::decode(&bytes).unwrap()), Some(1));
    }

    #[test]
    fn apply_envelope_cascades_to_a_dependent_with_no_reads_of_its_own() {
        let projections: Vec<Box<dyn ErasedProjection>> = vec![
            erase(CounterProjection { id: "counter", schema_version: 1, dependencies: &[], reads: &["counter"] }),
            // "cascade" has an EMPTY reads() — per the module doc's design note, empty reads means
            // "not directly triggered by anything"; it must only ever run via the dependency cascade.
            erase(SumWithDependencyProjection { id: "cascade", dependency_id: "counter", dependencies: &["counter"], reads: &[] }),
        ];
        let storage = MemoryStorage::new();
        let engine = ProjectionEngine::new(&storage, "doc-1".into(), projections).unwrap();

        // "counter" is untouched -> "cascade" has nothing to cascade from -> neither runs.
        engine.apply_envelope(1, &envelope("doc-1", "op-1", 1), &touch(&["unrelated"])).unwrap();
        assert_eq!(engine.state_at("counter", 1).unwrap(), None);
        assert_eq!(engine.state_at("cascade", 1).unwrap(), None);

        // "counter" is touched -> runs -> "cascade" cascades even though "unrelated" (not "counter")
        // is the only path in this step's touched set that "cascade" itself would ever have read.
        engine.apply_envelope(2, &envelope("doc-1", "op-2", 2), &touch(&["counter"])).unwrap();
        assert_eq!(engine.state_at("counter", 2).unwrap().map(|bytes| u64::decode(&bytes).unwrap()), Some(1));
        assert_eq!(
            engine.state_at("cascade", 2).unwrap().map(|bytes| u64::decode(&bytes).unwrap()),
            Some(2),
            "cascade = prior(0) + 1 + counter's this-step value(1) = 2"
        );
    }
    //#endregion 🔖IncrementalTriggering

    //#region 🔖RebuildEqualsIncremental
    /// 🧪 The core law: applying every event incrementally (each step reading its checkpoint via
    /// `load_checkpoint`, resuming from whatever was durably persisted, and gated by `should_run`
    /// against that step's touched set) must land on the exact same final state as a pure in-memory
    /// `rebuild_in_memory` replay of the same event sequence — including after an engine is dropped
    /// and reconstructed mid-stream so `apply_envelope` genuinely resumes from a persisted
    /// checkpoint rather than in-process memory, and including projections that are skipped on some
    /// steps and cascaded-to on others.
    #[test]
    fn rebuild_equals_incremental_after_checkpoint_resume() {
        let storage = MemoryStorage::new();
        let make_projections = || -> Vec<Box<dyn ErasedProjection>> {
            vec![
                erase(CounterProjection { id: "count", schema_version: 1, dependencies: &[], reads: &["doc"] }),
                erase(SumWithDependencyProjection { id: "sum", dependency_id: "count", dependencies: &["count"], reads: &[] }),
                erase(CounterProjection { id: "never", schema_version: 1, dependencies: &[], reads: &["never-touched"] }),
            ]
        };

        // Alternating touched paths so "count"/"sum" run on some steps and are skipped on others —
        // "never" is never touched and has no dependents, so it should stay at its initial state.
        let touched_paths: [&[&str]; 5] = [&["doc"], &["unrelated"], &["doc"], &["doc", "unrelated"], &["unrelated"]];
        let events: Vec<(u64, OperationEnvelope, TouchedSet)> = (1..=5u64)
            .map(|seq| (seq, envelope("doc-1", &format!("op-{seq}"), seq), touch(touched_paths[(seq - 1) as usize])))
            .collect();

        // Incremental path: apply seqs 1-3 against one engine instance, drop it, then resume with a
        // FRESH engine instance (forcing seqs 4-5 to load their checkpoint from `storage`, not from
        // any in-memory state the first engine instance happened to hold).
        {
            let engine = ProjectionEngine::new(&storage, "doc-1".into(), make_projections()).unwrap();
            for (seq, env, touched) in &events[..3] {
                engine.apply_envelope(*seq, env, touched).unwrap();
            }
        }
        let incremental_final = {
            let engine = ProjectionEngine::new(&storage, "doc-1".into(), make_projections()).unwrap();
            let mut last = PMap::new();
            for (seq, env, touched) in &events[3..] {
                last = engine.apply_envelope(*seq, env, touched).unwrap();
            }
            last
        };

        // Ground-truth path: one pure in-memory replay of the whole history, touching no storage.
        let rebuild_engine = ProjectionEngine::new(&storage, "doc-1".into(), make_projections()).unwrap();
        let rebuilt_final = rebuild_engine.rebuild_in_memory(&events).unwrap();

        for id in ["count", "sum", "never"] {
            assert_eq!(
                incremental_final.get(&id.to_string()),
                rebuilt_final.get(&id.to_string()),
                "projection {id} diverged between checkpoint-resumed incremental application and full in-memory rebuild"
            );
        }
        assert_eq!(u64::decode(rebuilt_final.get(&"never".to_string()).unwrap()).unwrap(), 0, "sanity: 'never' truly never ran");
    }
    //#endregion 🔖RebuildEqualsIncremental
}
//#endregion 🧪Tests
