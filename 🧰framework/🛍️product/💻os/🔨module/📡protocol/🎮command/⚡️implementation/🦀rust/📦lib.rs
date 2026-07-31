//! 🎞️ Protocol command/collaboration-semantics layer: the `Operation`/`OperationDiff`/`OpText`
//! trait family, `OperationMeta`/`Edit`, generic collection-operation plumbing, the runtime
//! `OperationDescriptor` registry, the upcast seam, and the five-channel `CommandOutcome`. Moved
//! (with two defaulted trait methods and a `reconcile` return-type change, both called out inline)
//! from `vcs/rs/lib.rs`'s `🔖Operation`/`🔖CollectionDiff`/`🔖CollectionOperation` regions and
//! `framework/core`. Frozen contract:
//! `.🦑repo/🎫tickets/26/07/27/PROTOCOL-BINARY-OP-LOG-LAYER/contract.md` `## Amendment` §`protocol_command`.
//!
//! Op payloads stay schema-opaque here exactly like `protocol_history`: this crate never parses or
//! interprets an `Op`'s fields, only threads it through the trait seams a technology implements.

//#region 🔖Operation
/// @emoji 📦 Centralized projection mutation — one `apply` per technology. Moved from
/// `store::OperationDiff` verbatim (only the parameter name `projection` → `base`).
pub trait OperationDiff<P>: Clone + Default + serde::Serialize + serde::de::DeserializeOwned {
    fn apply(&self, base: &P) -> P;
    fn absorb(&mut self, other: Self);
}

/// @emoji 🔁 Stored operation: emits a diff and computes backwards from pre-state. Moved from
/// `store::Operation` verbatim except: `operation_id`/`dependencies`/`author_id` now return the
/// `protocol_core` id newtypes (were bare `String`) and `base_version` now returns
/// `Option<protocol_core::DocumentVersion>` (was a bare `u64` defaulting to `0`, which conflated
/// "no base" with "based on version 0" — `None` fixes that); `conflict_rule`/`state_class` are new
/// defaulted methods so every existing `impl` recompiles unchanged; `reconcile` becomes an instance
/// method (`&self`) returning this crate's own `ReconcileReport` instead of `store::SpaceConflict`,
/// so `vcs` maps `ReconcileReport -> SpaceConflict` at its own edge instead of this crate knowing
/// about space types.
pub trait Operation<P>: Clone + serde::Serialize + serde::de::DeserializeOwned {
    type Diff: OperationDiff<P>;

    fn diff(&self, base: &P) -> Self::Diff;
    fn backwards(&self, base: &P) -> Vec<Self>;

    fn operation_id(&self) -> Option<protocol_core::OperationId> {
        None
    }
    fn dependencies(&self) -> Vec<protocol_core::OperationId> {
        Vec::new()
    }
    fn base_version(&self) -> Option<protocol_core::DocumentVersion> {
        None
    }
    fn author_id(&self) -> Option<protocol_core::ActorId> {
        None
    }
    fn timestamp(&self) -> Option<protocol_core::HybridLogicalTimestamp> {
        None
    }
    fn undo_policy(&self) -> protocol_core::UndoPolicy {
        protocol_core::UndoPolicy::ExactBaseOnly
    }
    fn merge_strategy(&self) -> protocol_core::MergeStrategyKind {
        protocol_core::MergeStrategyKind::LwwRegister
    }
    /// @emoji ⚖️ Per-operation conflict declaration; defaults to `Merge(self.merge_strategy())` so a
    /// technology that only overrode `merge_strategy` keeps its exact prior collapse-to-merge shape.
    fn conflict_rule(&self) -> protocol_core::ConflictRule {
        protocol_core::ConflictRule::Merge(self.merge_strategy())
    }
    /// @emoji 🗂️ Which durability/visibility class this operation's diffs belong to.
    fn state_class(&self) -> protocol_core::StateClass {
        protocol_core::StateClass::Persistent
    }
    /// @emoji 🤝 Post-materialization reconciliation pass (e.g. cross-document studio graph checks).
    /// Defaults to a no-op so every existing document kind keeps its exact prior behavior.
    fn reconcile(&self, projection: P) -> (P, Vec<ReconcileReport>) {
        (projection, Vec::new())
    }
    /// @emoji 🛂 Pre-apply validation against the current projection. Defaults to `Ok`.
    fn validate(&self, _projection: &P) -> Result<(), String> {
        Ok(())
    }
}

/// @emoji 📋 One `reconcile` finding.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ReconcileReport {
    pub id: String,
    pub message: String,
    pub severity: ReconcileSeverity,
}

/// @emoji 🚦 How serious a `ReconcileReport` is.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ReconcileSeverity {
    Info,
    Warning,
    Blocking,
}
//#endregion 🔖Operation

//#region 🔖OpText
/// @emoji ⚡ Handcrafted ONE-LINE textual representation of an operation, implemented once per
/// technology next to its `Operation` enum. Moved verbatim from `store::OpText` (method order
/// flipped to match the frozen contract; behavior unchanged). LAWS: `print_op` output never
/// contains `\n`; `Op::parse_op` recovers an equal operation from `op.print_op()`.
pub trait OpText: Sized {
    fn print_op(&self) -> String;
    fn parse_op(line: &str) -> Result<Self, dsl_core::TextError>;
}
//#endregion 🔖OpText

//#region 🔖OpBinary
/// @emoji 🎞️ Binary twin of [`OpText`]: the maximum-token-efficient one-line grammar and this
/// byte encoding are two renderings of the same operation, implemented per technology next to its
/// `Operation` enum (in practice emitted by `#[derive(dsl::DslOps)]` through `dsl::op_rt`, the
/// exact mirror of the `DocumentDsl`/`DocumentPack` pairing). Layout (owned by the runtime, not
/// by implementors): `format u8 (=1) | variant ordinal varint | record body`. LAWS:
/// `Op::decode_op(op.encode_op()) == op == Op::parse_op(op.print_op())`, and encoding is
/// deterministic — byte-identical output for equal operations.
pub trait OpBinary: Sized {
    fn encode_op(&self) -> Result<Vec<u8>, protocol_core::ProtocolError>;
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol_core::ProtocolError>;
}
//#endregion 🔖OpBinary

//#region 🔖Meta
/// @emoji 🧾 Per-operation causal/undo metadata attached to one `Edit` slot. Moved from
/// `store::OperationMeta` (was `vcs/rs/lib.rs` L59) with the id-flavored fields upgraded from bare
/// `String`/`Option<String>` to the `protocol_core` newtypes and `timestamp` upgraded from
/// `Option<HybridLogicalTimestamp>` to a required field (an edit's op always has a tick by the time
/// it is durable).
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OperationMeta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<protocol_core::OperationId>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dependencies: Vec<protocol_core::OperationId>,
    pub base_version: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_id: Option<protocol_core::ActorId>,
    pub timestamp: protocol_core::HybridLogicalTimestamp,
    pub undo_policy: protocol_core::UndoPolicy,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_hash: Option<protocol_core::PayloadHash>,
}

/// @emoji 📝 One coalesced batch of operations, forward and backward, plus their causal metadata.
/// Moved verbatim from `store::Edit` (was `vcs/rs/lib.rs` L73).
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Edit<Op> {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor: Option<String>,
    pub forwards: Vec<Op>,
    pub backwards: Vec<Op>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub operation_meta: Vec<OperationMeta>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coalesce_key: Option<String>,
    pub sequence_number: i32,
    pub started_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<String>,
}
//#endregion 🔖Meta

//#region 🔖Collection
/// @emoji 🏷️ Identifies an item within a `Vec` by a stable id, for generic collection operations.
/// Moved verbatim from `vcs::Identified`.
pub trait Identified<TId> {
    fn id(&self) -> &TId;
}

/// @emoji 🩹 In-place patch application plus a structural diff between two states of `Self`. The
/// frozen contract splits `vcs::Patchable`'s single `apply_patch(&mut self, &TPatch) -> TPatch`
/// (mutate-and-return-inverse) into two methods: `apply_patch` (mutate only) and `diff_patch`
/// (compute a patch from `self` to `other`, direction chosen to match `Operation::diff(&self,
/// base)`'s "self relative to an argument" convention) — `invert_collection_operation` below
/// composes them (`after.diff_patch(&prior)`) to recover the same inverse-patch behavior.
pub trait Patchable<TPatch> {
    fn apply_patch(&mut self, patch: &TPatch);
    fn diff_patch(&self, other: &Self) -> Option<TPatch>;
}

/// @emoji 🧩 Sparse collection patch entry (mirrors compose `XModified`). Moved verbatim from
/// `vcs::ItemPatch`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemPatch<TId, TPatch> {
    pub id: TId,
    pub patch: TPatch,
}

/// @emoji 🧩 Sparse collection diff (mirrors compose `XCollectionDiff`). Moved verbatim from
/// `vcs::CollectionDiff`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionDiff<TId, TPatch, TAdded> {
    pub removed: Vec<TId>,
    pub modified: Vec<ItemPatch<TId, TPatch>>,
    pub added: Vec<TAdded>,
}

impl<TId, TPatch, TAdded> Default for CollectionDiff<TId, TPatch, TAdded> {
    fn default() -> Self {
        Self { removed: Vec::new(), modified: Vec::new(), added: Vec::new() }
    }
}

/// @emoji 🧺 Generic ordered-collection operation (add/remove/move/patch) with mechanical
/// pre-state inverses. Frozen-contract shape: `Add` now carries its own `id` (was implied via
/// `TItem: Identified`), and `Move`'s target field is named `to` (was `to_index`).
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum CollectionOperation<TId, TItem, TPatch> {
    Add { id: TId, item: TItem, at: usize },
    Remove { id: TId },
    Move { id: TId, to: usize },
    Patch { id: TId, patch: TPatch },
}

/// @emoji ▶️ Applies a `CollectionOperation` to a `Vec` in place. Moved from
/// `vcs::apply_collection_operation`, adapted to the `Add { at, .. }`/`Move { to, .. }` field names.
pub fn apply_collection_operation<TId, TItem, TPatch>(items: &mut Vec<TItem>, operation: &CollectionOperation<TId, TItem, TPatch>)
where
    TId: PartialEq + Clone,
    TItem: Identified<TId> + Clone + Patchable<TPatch>,
{
    match operation {
        CollectionOperation::Add { item, at, .. } => {
            let pos = (*at).min(items.len());
            items.insert(pos, item.clone());
        }
        CollectionOperation::Remove { id } => {
            items.retain(|item| item.id() != id);
        }
        CollectionOperation::Move { id, to } => {
            if let Some(from) = items.iter().position(|item| item.id() == id) {
                let item = items.remove(from);
                let pos = (*to).min(items.len());
                items.insert(pos, item);
            }
        }
        CollectionOperation::Patch { id, patch } => {
            if let Some(item) = items.iter_mut().find(|item| item.id() == id) {
                item.apply_patch(patch);
            }
        }
    }
}

/// @emoji ↩️ Computes the inverse `CollectionOperation` from the pre-state `items`. Panics if
/// `operation` targets an id absent from `items` (`Remove`/`Move`/`Patch` always target an existing
/// item by construction) or if a `Patch`'s `apply_patch` is a genuine no-op that `diff_patch`
/// cannot express as an inverse (a technology whose `Patchable` impl can produce a no-op patch
/// should have `diff_patch` return `Some(unchanged-patch)`, not `None`, in that case).
pub fn invert_collection_operation<TId, TItem, TPatch>(items: &[TItem], operation: &CollectionOperation<TId, TItem, TPatch>) -> CollectionOperation<TId, TItem, TPatch>
where
    TId: PartialEq + Clone,
    TItem: Identified<TId> + Clone + Patchable<TPatch>,
    TPatch: Clone,
{
    match operation {
        CollectionOperation::Add { id, .. } => CollectionOperation::Remove { id: id.clone() },
        CollectionOperation::Remove { id } => {
            let index = items.iter().position(|item| item.id() == id).expect("remove target must exist in pre-state");
            CollectionOperation::Add { id: id.clone(), item: items[index].clone(), at: index }
        }
        CollectionOperation::Move { id, .. } => {
            let index = items.iter().position(|item| item.id() == id).expect("move target must exist in pre-state");
            CollectionOperation::Move { id: id.clone(), to: index }
        }
        CollectionOperation::Patch { id, patch } => {
            let prior = items.iter().find(|item| item.id() == id).cloned().expect("patch target must exist in pre-state");
            let mut after = prior.clone();
            after.apply_patch(patch);
            let inverse_patch = after.diff_patch(&prior).expect("a patch that changed state must yield a computable inverse");
            CollectionOperation::Patch { id: id.clone(), patch: inverse_patch }
        }
    }
}

/// @emoji 🧮 Projects a `CollectionOperation` onto a sparse `CollectionDiff`. Moved from
/// `vcs::collection_diff_from_operation`, adapted to the new field names. `Add` → `added`,
/// `Remove` → `removed`, `Patch` → `modified`. `CollectionDiff` has no positional-move channel, so
/// `Move` is encoded as `removed` + `added` (delete then re-add by identity).
pub fn collection_diff_from_operation<TId, TItem, TPatch>(items: &[TItem], operation: &CollectionOperation<TId, TItem, TPatch>) -> CollectionDiff<TId, TPatch, TItem>
where
    TId: PartialEq + Clone,
    TItem: Identified<TId> + Clone,
    TPatch: Clone,
{
    let mut diff = CollectionDiff::default();
    match operation {
        CollectionOperation::Add { item, .. } => diff.added.push(item.clone()),
        CollectionOperation::Remove { id } => diff.removed.push(id.clone()),
        CollectionOperation::Patch { id, patch } => diff.modified.push(ItemPatch { id: id.clone(), patch: patch.clone() }),
        CollectionOperation::Move { id, .. } => {
            if let Some(item) = items.iter().find(|item| item.id() == id) {
                diff.removed.push(id.clone());
                diff.added.push(item.clone());
            }
        }
    }
    diff
}
//#endregion 🔖Collection

//#region 🔖Descriptor
/// @emoji 🪪 A registered operation kind's runtime descriptor: schema identity/version, its
/// `StateClass`/`ConflictRule`, and a content-addressed `fingerprint` over those four fields.
#[derive(Clone, Debug, PartialEq)]
pub struct OperationDescriptor {
    pub id: protocol_core::SchemaId,
    pub schema_version: protocol_core::SchemaVersion,
    pub state_class: protocol_core::StateClass,
    pub conflict_rule: protocol_core::ConflictRule,
    pub fingerprint: [u8; 32],
}

impl OperationDescriptor {
    /// @emoji 🏗️ Constructs a descriptor, computing `fingerprint` deterministically from the other
    /// four fields. The contract fixes the struct's shape but not how `fingerprint` is derived; our
    /// choice is a canonical-JSON encoding of `(id, schema_version, state_class, conflict_rule)`
    /// hashed with blake3 — stable across process runs and platforms, pinned by a golden test below.
    pub fn new(id: protocol_core::SchemaId, schema_version: protocol_core::SchemaVersion, state_class: protocol_core::StateClass, conflict_rule: protocol_core::ConflictRule) -> Self {
        let fingerprint = descriptor_fingerprint(&id, schema_version, state_class, conflict_rule);
        Self { id, schema_version, state_class, conflict_rule, fingerprint }
    }
}

fn descriptor_fingerprint(id: &protocol_core::SchemaId, schema_version: protocol_core::SchemaVersion, state_class: protocol_core::StateClass, conflict_rule: protocol_core::ConflictRule) -> [u8; 32] {
    #[derive(serde::Serialize)]
    struct Canonical<'a> {
        id: &'a str,
        schema_version: u32,
        state_class: protocol_core::StateClass,
        conflict_rule: protocol_core::ConflictRule,
    }
    let canonical = Canonical { id: &id.0, schema_version: schema_version.0, state_class, conflict_rule };
    let bytes = serde_json::to_vec(&canonical).expect("descriptor canonical encoding never fails");
    *blake3::hash(&bytes).as_bytes()
}

static OPERATION_DESCRIPTOR_REGISTRY: std::sync::OnceLock<std::sync::RwLock<std::collections::HashMap<String, OperationDescriptor>>> = std::sync::OnceLock::new();

fn operation_descriptor_registry() -> &'static std::sync::RwLock<std::collections::HashMap<String, OperationDescriptor>> {
    OPERATION_DESCRIPTOR_REGISTRY.get_or_init(|| std::sync::RwLock::new(std::collections::HashMap::new()))
}

/// @emoji 📝 Registers (or overwrites) a descriptor by `descriptor.id`. Mirrors
/// `store::CodecRegistry`'s `OnceLock<RwLock<HashMap>>` pattern; idempotent, safe to call repeatedly.
pub fn register_operation_descriptor(descriptor: OperationDescriptor) {
    let mut registry = operation_descriptor_registry().write().unwrap_or_else(|poisoned| poisoned.into_inner());
    registry.insert(descriptor.id.0.clone(), descriptor);
}

/// @emoji 🔎 Looks up the descriptor registered for `schema`, if any.
pub fn operation_descriptor(schema: &str) -> Option<OperationDescriptor> {
    let registry = operation_descriptor_registry().read().unwrap_or_else(|poisoned| poisoned.into_inner());
    registry.get(schema).cloned()
}
//#endregion 🔖Descriptor

//#region 🔖Upcast
/// @emoji ⬆️ Rewrites an operation authored at an older schema version into today's shape.
/// LAW: `upcast(upcast(x)) == upcast(x)` — idempotence at the target version.
pub trait OperationUpcaster<Op> {
    fn upcast(&self, from_version: protocol_core::SchemaVersion, op: Op) -> Op;
}
//#endregion 🔖Upcast

//#region 🔖Events
/// @emoji 📡 One side-effect-channel event emitted alongside a persistent/UI diff.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OperationEvent {
    pub operation_id: protocol_core::OperationId,
    pub state_class: protocol_core::StateClass,
    pub payload: serde_json::Value,
}
//#endregion 🔖Events

//#region 🔖Outcome
/// @emoji 🗂️ The five-channel separation `framework/core`'s `InvocationResult` later maps onto:
/// durable diffs, two UI-visibility tiers, a speculative preview tier, and side-effect events.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CommandOutcome<Diff> {
    pub persistent: Vec<Diff>,
    pub shared_ui: Vec<Diff>,
    pub local_ui: Vec<Diff>,
    pub preview: Vec<Diff>,
    pub effects: Vec<OperationEvent>,
}
//#endregion 🔖Outcome

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;

    //#region 🧸Fixtures
    // Dummy (P=i64, Op=AddOp) pair: the smallest possible Operation/OperationDiff/OpText impl,
    // used across every law test below instead of a real technology's op set.
    #[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
    struct AddDiff {
        delta: i64,
    }
    impl OperationDiff<i64> for AddDiff {
        fn apply(&self, base: &i64) -> i64 {
            base + self.delta
        }
        fn absorb(&mut self, other: Self) {
            self.delta += other.delta;
        }
    }

    #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    struct AddOp {
        delta: i64,
    }
    impl Operation<i64> for AddOp {
        type Diff = AddDiff;
        fn diff(&self, _base: &i64) -> AddDiff {
            AddDiff { delta: self.delta }
        }
        fn backwards(&self, _base: &i64) -> Vec<Self> {
            vec![AddOp { delta: -self.delta }]
        }
    }
    impl OpText for AddOp {
        fn print_op(&self) -> String {
            format!("add {}", self.delta)
        }
        fn parse_op(line: &str) -> Result<Self, dsl_core::TextError> {
            let rest = line
                .strip_prefix("add ")
                .ok_or_else(|| dsl_core::TextError::new("expected 'add <n>'", dsl_core::TextSpan::at(1, 1)))?;
            let delta: i64 = rest
                .trim()
                .parse()
                .map_err(|_| dsl_core::TextError::new("invalid integer", dsl_core::TextSpan::at(1, 1)))?;
            Ok(AddOp { delta })
        }
    }

    #[derive(Clone, Debug, PartialEq)]
    struct Item {
        id: String,
        value: i64,
    }
    impl Identified<String> for Item {
        fn id(&self) -> &String {
            &self.id
        }
    }
    impl Patchable<i64> for Item {
        fn apply_patch(&mut self, patch: &i64) {
            self.value += patch;
        }
        fn diff_patch(&self, other: &Self) -> Option<i64> {
            let delta = other.value - self.value;
            if delta == 0 {
                None
            } else {
                Some(delta)
            }
        }
    }
    //#endregion 🧸Fixtures

    //#region 🧪OperationLaws
    #[test]
    fn operation_diff_apply_matches_backwards_inverse() {
        let base: i64 = 10;
        let op = AddOp { delta: 5 };
        let forward = op.diff(&base).apply(&base);
        assert_eq!(forward, 15);
        let [undo] = <[AddOp; 1]>::try_from(op.backwards(&base)).unwrap();
        let restored = undo.diff(&forward).apply(&forward);
        assert_eq!(restored, base);
    }

    #[test]
    fn operation_diff_absorb_accumulates() {
        let mut a = AddDiff { delta: 3 };
        a.absorb(AddDiff { delta: 4 });
        assert_eq!(a.delta, 7);
    }

    #[test]
    fn operation_defaults_are_stable() {
        let op = AddOp { delta: 1 };
        assert_eq!(op.operation_id(), None);
        assert!(op.dependencies().is_empty());
        assert_eq!(op.base_version(), None);
        assert_eq!(op.author_id(), None);
        assert_eq!(op.timestamp(), None);
        assert_eq!(op.undo_policy(), protocol_core::UndoPolicy::ExactBaseOnly);
        assert_eq!(op.merge_strategy(), protocol_core::MergeStrategyKind::LwwRegister);
        assert_eq!(op.conflict_rule(), protocol_core::ConflictRule::Merge(protocol_core::MergeStrategyKind::LwwRegister));
        assert_eq!(op.state_class(), protocol_core::StateClass::Persistent);
        assert!(op.validate(&0).is_ok());
        let (projection, reports) = op.reconcile(42);
        assert_eq!(projection, 42);
        assert!(reports.is_empty());
    }
    //#endregion 🧪OperationLaws

    //#region 🧪OpTextLaws
    #[test]
    fn op_text_round_trip() {
        let op = AddOp { delta: -7 };
        let line = op.print_op();
        assert!(!line.contains('\n'));
        let parsed = AddOp::parse_op(&line).expect("round trip parse");
        assert_eq!(parsed, op);
    }

    #[test]
    fn op_text_parse_error_carries_message() {
        let error = AddOp::parse_op("nope").unwrap_err();
        assert!(!error.message.is_empty());
    }
    //#endregion 🧪OpTextLaws

    //#region 🧪MetaSerde
    #[test]
    fn operation_meta_serde_round_trip() {
        let meta = OperationMeta {
            operation_id: Some(protocol_core::OperationId("op-1".into())),
            dependencies: vec![protocol_core::OperationId("op-0".into())],
            base_version: 3,
            author_id: Some(protocol_core::ActorId("actor-1".into())),
            timestamp: protocol_core::HybridLogicalTimestamp::new(1, 1000),
            undo_policy: protocol_core::UndoPolicy::TransformAgainstConcurrent,
            payload_hash: Some(protocol_core::PayloadHash([7u8; 32])),
        };
        let json = serde_json::to_string(&meta).expect("serialize");
        let round_tripped: OperationMeta = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(round_tripped, meta);
    }

    #[test]
    fn edit_serde_round_trip() {
        let edit = Edit::<AddOp> {
            id: "edit-1".into(),
            actor: Some("actor-1".into()),
            forwards: vec![AddOp { delta: 1 }, AddOp { delta: 2 }],
            backwards: vec![AddOp { delta: -2 }, AddOp { delta: -1 }],
            operation_meta: vec![OperationMeta {
                operation_id: None,
                dependencies: Vec::new(),
                base_version: 0,
                author_id: None,
                timestamp: protocol_core::HybridLogicalTimestamp::new(1, 0),
                undo_policy: protocol_core::UndoPolicy::ExactBaseOnly,
                payload_hash: None,
            }],
            description: Some("two adds".into()),
            coalesce_key: None,
            sequence_number: 1,
            started_at: "2026-07-27T00:00:00Z".into(),
            finished_at: None,
        };
        let json = serde_json::to_string(&edit).expect("serialize");
        let round_tripped: Edit<AddOp> = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(round_tripped, edit);
    }
    //#endregion 🧪MetaSerde

    //#region 🧪CollectionLaws
    #[test]
    fn apply_add_remove_move_patch() {
        let mut items = vec![Item { id: "a".into(), value: 1 }, Item { id: "b".into(), value: 2 }];

        apply_collection_operation(&mut items, &CollectionOperation::Add { id: "c".into(), item: Item { id: "c".into(), value: 3 }, at: 1 });
        assert_eq!(items.iter().map(|i| i.id.clone()).collect::<Vec<_>>(), vec!["a", "c", "b"]);

        apply_collection_operation(&mut items, &CollectionOperation::Move { id: "c".into(), to: 2 });
        assert_eq!(items.iter().map(|i| i.id.clone()).collect::<Vec<_>>(), vec!["a", "b", "c"]);

        apply_collection_operation::<String, Item, i64>(&mut items, &CollectionOperation::Patch { id: "b".into(), patch: 10 });
        assert_eq!(items.iter().find(|i| i.id == "b").unwrap().value, 12);

        apply_collection_operation(&mut items, &CollectionOperation::Remove { id: "a".into() });
        assert_eq!(items.iter().map(|i| i.id.clone()).collect::<Vec<_>>(), vec!["b", "c"]);
    }

    #[test]
    fn invert_collection_operation_round_trips_every_kind() {
        let original = vec![Item { id: "a".into(), value: 1 }, Item { id: "b".into(), value: 2 }];

        let add = CollectionOperation::Add { id: "c".into(), item: Item { id: "c".into(), value: 3 }, at: 2 };
        let mut items = original.clone();
        apply_collection_operation(&mut items, &add);
        let inverse = invert_collection_operation(&original, &add);
        apply_collection_operation(&mut items, &inverse);
        assert_eq!(items, original);

        let mov = CollectionOperation::<String, Item, i64>::Move { id: "b".into(), to: 0 };
        let mut items = original.clone();
        apply_collection_operation(&mut items, &mov);
        let inverse = invert_collection_operation(&original, &mov);
        apply_collection_operation(&mut items, &inverse);
        assert_eq!(items, original);

        let patch = CollectionOperation::Patch { id: "a".into(), patch: 9i64 };
        let mut items = original.clone();
        apply_collection_operation(&mut items, &patch);
        let inverse = invert_collection_operation(&original, &patch);
        apply_collection_operation(&mut items, &inverse);
        assert_eq!(items, original);

        let remove = CollectionOperation::<String, Item, i64>::Remove { id: "a".into() };
        let mut items = original.clone();
        apply_collection_operation(&mut items, &remove);
        let inverse = invert_collection_operation(&original, &remove);
        apply_collection_operation(&mut items, &inverse);
        assert_eq!(items, original);
    }

    #[test]
    fn collection_diff_from_operation_projects_each_kind() {
        let items = vec![Item { id: "a".into(), value: 1 }, Item { id: "b".into(), value: 2 }];

        let add = CollectionOperation::<String, Item, i64>::Add { id: "c".into(), item: Item { id: "c".into(), value: 3 }, at: 0 };
        let diff = collection_diff_from_operation(&items, &add);
        assert_eq!(diff.added, vec![Item { id: "c".into(), value: 3 }]);
        assert!(diff.removed.is_empty() && diff.modified.is_empty());

        let remove = CollectionOperation::<String, Item, i64>::Remove { id: "a".into() };
        let diff = collection_diff_from_operation(&items, &remove);
        assert_eq!(diff.removed, vec!["a".to_string()]);

        let patch = CollectionOperation::Patch { id: "b".into(), patch: 5i64 };
        let diff = collection_diff_from_operation(&items, &patch);
        assert_eq!(diff.modified, vec![ItemPatch { id: "b".into(), patch: 5i64 }]);

        let mov = CollectionOperation::<String, Item, i64>::Move { id: "a".into(), to: 1 };
        let diff = collection_diff_from_operation(&items, &mov);
        assert_eq!(diff.removed, vec!["a".to_string()]);
        assert_eq!(diff.added, vec![Item { id: "a".into(), value: 1 }]);
    }
    //#endregion 🧪CollectionLaws

    //#region 🧪DescriptorLaws
    #[test]
    fn operation_descriptor_fingerprint_is_golden_pinned() {
        let descriptor = OperationDescriptor::new(
            protocol_core::SchemaId("note.append".into()),
            protocol_core::SchemaVersion(1),
            protocol_core::StateClass::Persistent,
            protocol_core::ConflictRule::Merge(protocol_core::MergeStrategyKind::TextSequence),
        );
        let hex: String = descriptor.fingerprint.iter().map(|b| format!("{b:02x}")).collect();
        // Golden pin computed once from `descriptor_fingerprint`'s canonical-JSON+blake3 encoding;
        // any change to that encoding (or to serde's field order/derives on the id/enum types it
        // hashes) is a breaking change to every persisted `OperationDescriptor` and must update this.
        assert_eq!(hex, "8c6c0b22512540811343d8caa8d147f48e936728105861c4360bb202f626d517");
    }
    //#endregion 🧪DescriptorLaws

    //#region 🧪UpcastLaws
    // Clamp-to-floor is the simplest genuinely idempotent upcaster: `upcast(upcast(x)) ==
    // upcast(x)` holds because `max(max(x, 10), 10) == max(x, 10)` for every `x`.
    struct ClampToFloor;
    impl OperationUpcaster<i64> for ClampToFloor {
        fn upcast(&self, _from_version: protocol_core::SchemaVersion, op: i64) -> i64 {
            op.max(10)
        }
    }

    #[test]
    fn upcaster_is_idempotent_at_target_version() {
        let upcaster = ClampToFloor;
        let version = protocol_core::SchemaVersion(1);
        for start in [0i64, 3, 7, 10, 40] {
            let once = upcaster.upcast(version, start);
            let twice = upcaster.upcast(version, once);
            assert_eq!(once, twice);
        }
    }
    //#endregion 🧪UpcastLaws

    //#region 🧪OutcomeLaws
    #[test]
    fn command_outcome_default_is_empty() {
        let outcome: CommandOutcome<AddDiff> = CommandOutcome::default();
        assert!(outcome.persistent.is_empty());
        assert!(outcome.shared_ui.is_empty());
        assert!(outcome.local_ui.is_empty());
        assert!(outcome.preview.is_empty());
        assert!(outcome.effects.is_empty());
    }

    #[test]
    fn operation_event_serde_round_trip() {
        let event = OperationEvent {
            operation_id: protocol_core::OperationId("op-1".into()),
            state_class: protocol_core::StateClass::Effect,
            payload: serde_json::json!({ "kind": "toast", "text": "saved" }),
        };
        let json = serde_json::to_string(&event).expect("serialize");
        let round_tripped: OperationEvent = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(round_tripped, event);
    }
    //#endregion 🧪OutcomeLaws
}
//#endregion 🧪Tests
