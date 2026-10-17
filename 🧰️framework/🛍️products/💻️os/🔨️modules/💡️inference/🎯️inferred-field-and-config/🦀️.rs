//! 🎯️ Inferred field contract of the inference cache: the pending and compute step enums, the inferred field trait, the cache configuration and the persistence mode. See [`super`].

use super::*;

//#region 🔖️InferredField
/// ⏳ A compute that spans several driver calls (a kernel job): the driver keeps it in the cursor between calls and cancels it with the run.
pub trait InferencePending: Send {
    fn cancel(&mut self);
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn close_original(&mut self,_cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"pending inference has no original payload closure producer").into())}
    fn original_box_release_bytes(&self)->Option<usize>{None}

}

/// 🪜 One slice of a compute: it finished (`Done`) or yielded for another driver call (`Working`). Either way it reports the fuel it consumed.
pub enum ComputeStep<V> {
    Done { value: V, fuel_used: usize },
    Working { fuel_used: usize, progress: f32 },
}

/// 🕸️ One inferred field family, computed entity-by-entity over a dependency DAG. This is the
/// trait real derivation math (e.g. a flatten engine) implements; an artifact's top-level
/// `Inference::infer` assembles its `XInference` struct from one or more `InferredField`s.
pub trait InferredField<P>: Send + Sync + 'static {
    type Key: RetireOwned + Clone + Eq + std::hash::Hash + Ord + Send + Sync + ToValue + FromValue + 'static;
    type Value: RetireOwned + Clone + Send + Sync + 'static;
    type Dependency: ToValue;

    const FIELD_ID: &'static str;
    const SCHEMA_VERSION: u32;

    /// 🗺️ Coarse tier-1 read-set — checked against a diff's [`semio_framework_os_command::DiffRegions::touches`]
    /// before this field's plan is even walked.
    fn reads() -> &'static [&'static str];

    /// 🎯 Whether the regions a diff touched may change the dependency of `key` (the per-key twin of [`reads`](Self::reads)): `false` lets an incremental update carry the previous value of an entity whose parents did not move
    /// without evaluating its dependency. `snapshot` is the snapshot after the diff, so the rows `key` reads are the ones it names now. Over-approximation costs a hash, under-approximation serves a stale value, so the default says yes.
    fn touched_by(snapshot: &P, key: &Self::Key, touched: &TouchedPaths) -> bool {
        let _ = (snapshot, key, touched);
        true
    }

    /// 🧭 Deterministic topological plan over `snapshot`'s entities (roots first — entries with no
    /// parents come before anything that depends on them).
    fn plan(snapshot: &P) -> Vec<InferenceStep<Self::Key>>;
    /// 🪪️ Borrows the original semantic snapshot identity across temporary host input views.
    fn source_identity(snapshot:&P)->usize{snapshot as*const P as usize}
    /// 📋️ Borrows the admitted graph's source extents without constructing an owned plan.
    fn plan_source_extent(_snapshot:&P)->Option<(usize,usize)>{None}
    /// 🪪️ Borrows one original node identity by its document ordinal.
    fn plan_source_node(_snapshot:&P,_ordinal:usize)->Option<&str>{None}
    /// 🔑️ Borrows the typed key from the same original document ordinal.
    fn plan_source_key(_snapshot:&P,_ordinal:usize)->Option<&Self::Key>{None}
    /// 🔗️ Borrows one original directed link by its document ordinal.
    fn plan_source_link(_snapshot:&P,_ordinal:usize)->Option<(&str,&str)>{None}
    /// 🪢️ Copies one original parent header into a retained donor under the actual caller.
    fn clone_original_parent(_original:&Self::Value,_output:&mut Option<Self::Value>,_cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,InferenceWorkFailure>{
        Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"inferred parent value has no original copy producer").into())
    }

    /// 🔑 Owned dependency values for `key` — EXACTLY the snapshot fields `compute` may
    /// read for this key (excluding parents' OWN upstream values, which are folded in separately
    /// via their already-computed [`DepHash`]es — but INCLUDING the specific edge/connector data
    /// tying `key` to each of `parents`, e.g. a compose-style attraction's params, since that lives
    /// on `key`'s own incoming edge, not on the parent's upstream chain). `parents` is `plan`'s
    /// `InferenceStep.parents` for this key, passed through so implementations don't need to
    /// re-derive "which edge connects to which parent" a second time. Honesty contract: this must
    /// cover everything `compute` reads, or a changed-but-uncovered input silently serves a stale
    /// cached value.
    fn dep_input(snapshot: &P, key: &Self::Key, parents: &[Self::Key]) -> Self::Dependency;

    /// 🧮 Pure per-entity compute, given parents' already-computed values in `plan`'s parent order.
    fn compute(snapshot: &P, key: &Self::Key, parents: &[Self::Value]) -> Self::Value;

    /// ⚖️ The bytes a cached `value` is accounted with against the cache budget; the default is the value's inline size.
    fn value_bytes(value: &Self::Value) -> usize {
        let _ = value;
        size_of::<Self::Value>()
    }

    /// 🪜 Resumable, fallible variant of [`compute`](Self::compute): consumes at most `fuel` units and either finishes or parks its progress in `pending`
    /// for the next call. The default finishes in one slice at the cost of one unit, so `compute` stays the single semantic source of every field that does not override this.
    fn compute_step(_snapshot:&P,_key:&Self::Key,_parents:&[Self::Value],_pending:&mut Option<Box<dyn InferencePending>>,_cx:&mut semio_framework_job::StepContext<'_>)->Result<ComputeStep<Self::Value>,InferenceWorkFailure>{
        Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original inferred field has no funded compute producer").into())
    }
    /// 🔄️ Produces the field's real cycle result for a row in the admitted plan's cyclic tail.
    fn compute_cyclic_step(_snapshot:&P,_key:&Self::Key,_pending:&mut Option<Box<dyn InferencePending>>,_cx:&mut semio_framework_job::StepContext<'_>)->Result<ComputeStep<Self::Value>,InferenceWorkFailure>{
        Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original inferred field has no funded cyclic result producer").into())
    }
}
//#endregion 🔖️InferredField

//#region 🔖️Config
/// ⚙️ Optionality + configuration surface. Default = DISABLED: with no cache every path degenerates
/// to plain recompute — caching is strictly opt-in per host.
#[derive(Clone, Debug)]
#[derive(semio_framework_value::RetireOwned)]
pub struct InferenceCacheConfig {
    pub enabled: bool,
    pub budget_bytes: usize,
    pub persistence: InferencePersistence,
    pub record_stats: bool,
}

impl Default for InferenceCacheConfig {
    fn default() -> Self {
        Self { enabled: false, budget_bytes: 16 * 1024 * 1024, persistence: InferencePersistence::None, record_stats: false }
    }
}

/// 💾 Whether cached inference results are also checkpointed durably (via a `db_projection`
/// adapter, kept out of this wasm-safe kernel crate — see the framework-level `db_artifact` module).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[derive(semio_framework_value::RetireOwned)]
pub enum InferencePersistence {
    None,
    Projection,
}
//#endregion 🔖️Config
