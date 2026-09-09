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
/// (`🔌️plugin/🖥️host/🦀️.rs`): wiring a brand-new proc-macro dependency into this
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
        timestamp: protocol::HybridLogicalTimestamp::new(1, seq),
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
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
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
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    assert!(matches!(ProjectionEngine::new(&storage, "doc-1".into(), projections).await, Err(DbError::AlreadyExists(_))));
}

#[semio_framework_async_macros::async_test]
async fn build_rejects_unknown_dependency() {
    let projections = vec![erase(CounterProjection { id: "a", schema_version: 1, dependencies: &["ghost"], reads: &[] })];
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    assert!(matches!(ProjectionEngine::new(&storage, "doc-1".into(), projections).await, Err(DbError::NotFound(_))));
}

#[semio_framework_async_macros::async_test]
async fn build_rejects_a_dependency_cycle() {
    let projections = vec![erase(CounterProjection { id: "a", schema_version: 1, dependencies: &["b"], reads: &[] }), erase(CounterProjection { id: "b", schema_version: 1, dependencies: &["a"], reads: &[] })];
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    assert!(matches!(ProjectionEngine::new(&storage, "doc-1".into(), projections).await, Err(DbError::InvalidArgument(_))));
}
//#endregion 🔖️Graph

//#region 🔖️Engine
#[semio_framework_async_macros::async_test]
async fn apply_envelope_advances_and_persists_incrementally() {
    let projections = vec![erase(CounterProjection { id: "count", schema_version: 1, dependencies: &[], reads: &["doc"] })];
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
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
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let engine = ProjectionEngine::new(&storage, "doc-1".into(), projections).await.unwrap();
    assert!(matches!(db_actor::block_on(engine.apply_envelope(1, &envelope("doc-OTHER", "op-1", 1).await, &touch(&["doc"]).await)), Err(DbError::InvalidArgument(_))));
}

#[semio_framework_async_macros::async_test]
async fn dependent_projection_sees_its_dependencys_state_from_the_same_step() {
    let projections: Vec<AnyTestProjection> =
        vec![erase(SumWithDependencyProjection { id: "sum", dependency_id: "count", dependencies: &["count"], reads: &[] }).into(), erase(CounterProjection { id: "count", schema_version: 1, dependencies: &[], reads: &["doc"] }).into()];
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
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
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
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
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
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
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
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
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
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
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
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
