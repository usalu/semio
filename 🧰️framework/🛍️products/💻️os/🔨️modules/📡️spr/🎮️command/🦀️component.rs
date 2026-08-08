//! 🎞️ Protocol command/collaboration-semantics layer: the `Mutation`/`MutationDiff`/`OpText`
//! trait family, `MutationMeta`/`Edit`, generic collection-operation plumbing, the runtime
//! `MutationDescriptor` registry, the upcast seam, and the five-channel `CommandOutcome`. Moved
//! (with two defaulted trait methods and a `reconcile` return-type change, both called out inline)
//! from `vcs/rs/lib.rs`'s `🔖️Mutation`/`🔖️CollectionDiff`/`🔖️CollectionMutation` regions and
//! `framework/core`. Frozen contract:
//! `.🦑️repo/🎫️tickets/26/07/27/PROTOCOL-BINARY-OP-LOG-LAYER/contract.md` `## Amendment` §`protocol_command`.
//!
//! Op payloads stay schema-opaque here exactly like `protocol_history`: this crate never parses or
//! interprets an `Op`'s fields, only threads it through the trait seams a technology implements.

//#region 🔖️Mutation
/// @emoji 📦️ Centralized projection mutation — one `apply` per technology. Moved from
/// `crate::os_store::MutationDiff` verbatim (only the parameter name `projection` → `base`).
pub trait MutationDiff<P>: Clone + Default + serde::Serialize + serde::de::DeserializeOwned {
    fn apply(&self, base: &P) -> P;
    fn absorb(&mut self, other: Self);
}

/// @emoji 🔁️ Stored operation: emits a diff and computes inverse from pre-state. Moved from
/// `crate::os_store::Mutation` verbatim except: `mutation_id`/`dependencies`/`author_id` now return the
/// `protocol_core` id newtypes (were bare `String`) and `base_version` now returns
/// `Option<crate::os_spr::ids::DocumentVersion>` (was a bare `u64` defaulting to `0`, which conflated
/// "no base" with "based on version 0" — `None` fixes that); `conflict_rule`/`state_class` are new
/// defaulted methods so every existing `impl` recompiles unchanged; `reconcile` becomes an instance
/// method (`&self`) returning this crate's own `ReconcileReport` instead of `crate::os_store::SpaceConflict`,
/// so `vcs` maps `ReconcileReport -> SpaceConflict` at its own edge instead of this crate knowing
/// about space types.
pub trait Mutation<P>: Clone + serde::Serialize + serde::de::DeserializeOwned {
    type Diff: MutationDiff<P>;

    fn diff(&self, base: &P) -> Self::Diff;
    fn inverse(&self, base: &P) -> Vec<Self>;

    fn mutation_id(&self) -> Option<crate::os_spr::ids::MutationId> {
        None
    }
    fn dependencies(&self) -> Vec<crate::os_spr::ids::MutationId> {
        Vec::new()
    }
    fn base_version(&self) -> Option<crate::os_spr::ids::DocumentVersion> {
        None
    }
    fn author_id(&self) -> Option<crate::os_spr::ids::ActorId> {
        None
    }
    fn timestamp(&self) -> Option<crate::os_spr::ids::HybridLogicalTimestamp> {
        None
    }
    fn undo_policy(&self) -> crate::os_spr::UndoPolicy {
        crate::os_spr::UndoPolicy::ExactBaseOnly
    }
    fn merge_strategy(&self) -> crate::os_spr::MergeStrategyKind {
        crate::os_spr::MergeStrategyKind::LwwRegister
    }
    /// @emoji ⚖️ Per-operation conflict declaration; defaults to `Merge(self.merge_strategy())` so a
    /// technology that only overrode `merge_strategy` keeps its exact prior collapse-to-merge shape.
    fn conflict_rule(&self) -> crate::os_spr::ConflictRule {
        crate::os_spr::ConflictRule::Merge(self.merge_strategy())
    }
    /// @emoji 🗂️ Which durability/visibility class this operation's diffs belong to.
    fn state_class(&self) -> crate::os_spr::StateClass {
        crate::os_spr::StateClass::Persistent
    }
    /// @emoji 🤝️ Post-materialization reconciliation pass (e.g. cross-document studio graph checks).
    /// Defaults to a no-op so every existing document kind keeps its exact prior behavior.
    fn reconcile(&self, projection: P) -> (P, Vec<ReconcileReport>) {
        (projection, Vec::new())
    }
    /// @emoji 🛂️ Pre-apply validation against the current projection. Defaults to `Ok`.
    fn validate(&self, _projection: &P) -> Result<(), String> {
        Ok(())
    }
}

/// @emoji 📋️ One `reconcile` finding.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ReconcileReport {
    pub id: String,
    pub message: String,
    pub severity: ReconcileSeverity,
}

/// @emoji 🚦️ How serious a `ReconcileReport` is.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ReconcileSeverity {
    Info,
    Warning,
    Blocking,
}
//#endregion 🔖️Mutation

//#region 🔖️OpText
/// @emoji ⚡️ Handcrafted ONE-LINE textual representation of an operation, implemented once per
/// technology next to its `Mutation` enum. Moved verbatim from `crate::os_store::OpText` (method order
/// flipped to match the frozen contract; behavior unchanged). LAWS: `print_op` output never
/// contains `\n`; `Op::parse_op` recovers an equal operation from `op.print_op()`.
pub trait OpText: Sized {
    fn print_op(&self) -> String;
    fn parse_op(line: &str) -> Result<Self, crate::os_dsl::TextError>;
}
//#endregion 🔖️OpText

//#region 🔖️OpBinary
/// @emoji 🎞️ Binary twin of [`OpText`]: the maximum-token-efficient one-line grammar and this
/// byte encoding are two renderings of the same operation, implemented per technology next to its
/// `Mutation` enum (in practice emitted by `#[derive(crate::os_dsl::DslOps)]` through `crate::os_dsl::op_rt`, the
/// exact mirror of the `DocumentDsl`/`DocumentPack` pairing). Layout (owned by the runtime, not
/// by implementors): `format u8 (=1) | variant ordinal varint | record body`. LAWS:
/// `Op::decode_op(op.encode_op()) == op == Op::parse_op(op.print_op())`, and encoding is
/// deterministic — byte-identical output for equal operations.
pub trait OpBinary: Sized {
    fn encode_op(&self) -> Result<Vec<u8>, crate::os_spr::ProtocolError>;
    fn decode_op(bytes: &[u8]) -> Result<Self, crate::os_spr::ProtocolError>;
}
//#endregion 🔖️OpBinary

//#region 🔖️DiffCodec
/// @emoji 🧬️ Grammared twin of [`OpText`]/[`OpBinary`], but for a technology's `MutationDiff::Diff`
/// value rather than its `Mutation`: the W1 foundation of the `handcrafted-grammar-for-every-artifact`
/// program's diff track (design ruling B-R4 at `.claude/plans/the-final-goal-for-jolly-spindle.md`) —
/// today every `*Diff` type is serde-only, this trait promotes a diff to a first-class grammared value
/// exactly like `OpText`/`OpBinary` already did for operations. In practice emitted by
/// `#[derive(crate::os_dsl::DslDiff)]` through the same `RecordSpec`-generation machinery `DslRecord`/
/// `DslDocument` already use (a diff is structurally just another record). Schema id convention:
/// `"<doc-schema>#diff"`. Deliberately NOT (yet) a supertrait bound of [`MutationDiff`] — W1 only
/// proves the mechanism on a handful of real diff types (tracked in `script.ts`'s
/// `POLICY_DIFF_COMPLETENESS_ALLOWLIST`); wiring it as a hard bound across all diff types is deferred
/// to wave 6 (`## Master wave plan` `W6 — Lane C (B5)`), once every type is covered.
/// LAWS: `Diff::parse_diff(&d.print_diff()) == d`, `Diff::decode_diff(&d.encode_diff()?)? == d`,
/// `print_diff` output never contains `\n`, and `encode_diff` is deterministic.
pub trait DiffCodec: Sized {
    fn print_diff(&self) -> String;
    fn parse_diff(line: &str) -> Result<Self, crate::os_dsl::TextError>;
    fn encode_diff(&self) -> Result<Vec<u8>, crate::os_spr::ProtocolError>;
    fn decode_diff(bytes: &[u8]) -> Result<Self, crate::os_spr::ProtocolError>;
}
//#endregion 🔖️DiffCodec

//#region 🔖️Meta
/// @emoji 🧾️ Per-operation causal/undo metadata attached to one `Edit` slot. Moved from
/// `crate::os_store::MutationMeta` (was `vcs/rs/lib.rs` L59) with the id-flavored fields upgraded from bare
/// `String`/`Option<String>` to the `protocol_core` newtypes and `timestamp` upgraded from
/// `Option<HybridLogicalTimestamp>` to a required field (an edit's op always has a tick by the time
/// it is durable).
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MutationMeta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mutation_id: Option<crate::os_spr::ids::MutationId>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dependencies: Vec<crate::os_spr::ids::MutationId>,
    pub base_version: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_id: Option<crate::os_spr::ids::ActorId>,
    pub timestamp: crate::os_spr::ids::HybridLogicalTimestamp,
    pub undo_policy: crate::os_spr::UndoPolicy,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_hash: Option<crate::os_spr::ids::PayloadHash>,
}

/// @emoji 📝️ One coalesced batch of operations, forward and backward, plus their causal metadata.
/// Moved verbatim from `crate::os_store::Edit` (was `vcs/rs/lib.rs` L73).
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Edit<Op> {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor: Option<String>,
    pub forwards: Vec<Op>,
    pub inverse: Vec<Op>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mutation_meta: Vec<MutationMeta>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coalesce_key: Option<String>,
    pub sequence_number: i32,
    pub started_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<String>,
}
//#endregion 🔖️Meta

//#region 🔖️Collection
/// 🧬️ Collection identity/patch/diff/ops — single source of truth in VCS (`crate::os_vcs`).
pub use crate::os_vcs::{
    apply_collection_mutation, collection_diff_from_mutation, inverse_collection_mutation, CollectionDiff,
    CollectionMutation, Identified, ItemPatch, Patchable,
};

//#endregion 🔖️Collection

//#region 🔖️Descriptor
/// @emoji 🪪️ A registered operation kind's runtime descriptor: schema identity/version, its
/// `StateClass`/`ConflictRule`, and a content-addressed `fingerprint` over those four fields.
#[derive(Clone, Debug, PartialEq)]
pub struct MutationDescriptor {
    pub id: crate::os_spr::ids::SchemaId,
    pub schema_version: crate::os_spr::ids::SchemaVersion,
    pub state_class: crate::os_spr::StateClass,
    pub conflict_rule: crate::os_spr::ConflictRule,
    pub fingerprint: [u8; 32],
}

impl MutationDescriptor {
    /// @emoji 🏗️ Constructs a descriptor, computing `fingerprint` deterministically from the other
    /// four fields. The contract fixes the struct's shape but not how `fingerprint` is derived; our
    /// choice is a canonical-JSON encoding of `(id, schema_version, state_class, conflict_rule)`
    /// hashed with blake3 — stable across process runs and platforms, pinned by a golden test below.
    pub fn new(id: crate::os_spr::ids::SchemaId, schema_version: crate::os_spr::ids::SchemaVersion, state_class: crate::os_spr::StateClass, conflict_rule: crate::os_spr::ConflictRule) -> Self {
        let fingerprint = descriptor_fingerprint(&id, schema_version, state_class, conflict_rule);
        Self { id, schema_version, state_class, conflict_rule, fingerprint }
    }
}

fn descriptor_fingerprint(id: &crate::os_spr::ids::SchemaId, schema_version: crate::os_spr::ids::SchemaVersion, state_class: crate::os_spr::StateClass, conflict_rule: crate::os_spr::ConflictRule) -> [u8; 32] {
    #[derive(serde::Serialize)]
    struct Canonical<'a> {
        id: &'a str,
        schema_version: u32,
        state_class: crate::os_spr::StateClass,
        conflict_rule: crate::os_spr::ConflictRule,
    }
    let canonical = Canonical { id: &id.0, schema_version: schema_version.0, state_class, conflict_rule };
    let bytes = serde_json::to_vec(&canonical).expect("descriptor canonical encoding never fails");
    *blake3::hash(&bytes).as_bytes()
}

static MUTATION_DESCRIPTOR_REGISTRY: std::sync::OnceLock<std::sync::RwLock<std::collections::HashMap<String, MutationDescriptor>>> = std::sync::OnceLock::new();

fn mutation_descriptor_registry() -> &'static std::sync::RwLock<std::collections::HashMap<String, MutationDescriptor>> {
    MUTATION_DESCRIPTOR_REGISTRY.get_or_init(|| std::sync::RwLock::new(std::collections::HashMap::new()))
}

/// @emoji 📝️ Registers (or overwrites) a descriptor by `descriptor.id`. Mirrors
/// `crate::os_store::CodecRegistry`'s `OnceLock<RwLock<HashMap>>` pattern; idempotent, safe to call repeatedly.
pub fn register_mutation_descriptor(descriptor: MutationDescriptor) {
    let mut registry = mutation_descriptor_registry().write().unwrap_or_else(|poisoned| poisoned.into_inner());
    registry.insert(descriptor.id.0.clone(), descriptor);
}

/// @emoji 🔎️ Looks up the descriptor registered for `schema`, if any.
pub fn mutation_descriptor(schema: &str) -> Option<MutationDescriptor> {
    let registry = mutation_descriptor_registry().read().unwrap_or_else(|poisoned| poisoned.into_inner());
    registry.get(schema).cloned()
}
//#endregion 🔖️Descriptor

//#region 🔖️Upcast
/// @emoji ⬆️ Rewrites an operation authored at an older schema version into today's shape.
/// LAW: `upcast(upcast(x)) == upcast(x)` — idempotence at the target version.
pub trait MutationUpcaster<Op> {
    fn upcast(&self, from_version: crate::os_spr::ids::SchemaVersion, op: Op) -> Op;
}
//#endregion 🔖️Upcast

//#region 🔖️Events
/// @emoji 📡️ One side-effect-channel event emitted alongside a persistent/UI diff.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MutationEvent {
    pub mutation_id: crate::os_spr::ids::MutationId,
    pub state_class: crate::os_spr::StateClass,
    pub payload: serde_json::Value,
}
//#endregion 🔖️Events

//#region 🔖️Outcome
/// @emoji 🗂️ The five-channel separation `framework/core`'s `InvocationResult` later maps onto:
/// durable diffs, two UI-visibility tiers, a speculative preview tier, and side-effect events.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CommandOutcome<Diff> {
    pub persistent: Vec<Diff>,
    pub shared_ui: Vec<Diff>,
    pub local_ui: Vec<Diff>,
    pub preview: Vec<Diff>,
    pub effects: Vec<MutationEvent>,
}
//#endregion 🔖️Outcome

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    //#region 🧸️Fixtures
    // Dummy (P=i64, Op=AddOp) pair: the smallest possible Mutation/MutationDiff/OpText impl,
    // used across every law test below instead of a real technology's op set.
    #[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
    struct AddDiff {
        delta: i64,
    }
    impl MutationDiff<i64> for AddDiff {
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
    impl Mutation<i64> for AddOp {
        type Diff = AddDiff;
        fn diff(&self, _base: &i64) -> AddDiff {
            AddDiff { delta: self.delta }
        }
        fn inverse(&self, _base: &i64) -> Vec<Self> {
            vec![AddOp { delta: -self.delta }]
        }
    }
    impl OpText for AddOp {
        fn print_op(&self) -> String {
            format!("add {}", self.delta)
        }
        fn parse_op(line: &str) -> Result<Self, crate::os_dsl::TextError> {
            let rest = line.strip_prefix("add ").ok_or_else(|| crate::os_dsl::TextError::new("expected 'add <n>'", crate::os_dsl::TextSpan::at(1, 1)))?;
            let delta: i64 = rest.trim().parse().map_err(|_| crate::os_dsl::TextError::new("invalid integer", crate::os_dsl::TextSpan::at(1, 1)))?;
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
    //#endregion 🧸️Fixtures

    //#region 🧪️MutationLaws
    #[test]
    fn operation_diff_apply_matches_backwards_inverse() {
        let base: i64 = 10;
        let op = AddOp { delta: 5 };
        let forward = op.diff(&base).apply(&base);
        assert_eq!(forward, 15);
        let [undo] = <[AddOp; 1]>::try_from(op.inverse(&base)).unwrap();
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
        assert_eq!(op.mutation_id(), None);
        assert!(op.dependencies().is_empty());
        assert_eq!(op.base_version(), None);
        assert_eq!(op.author_id(), None);
        assert_eq!(op.timestamp(), None);
        assert_eq!(op.undo_policy(), crate::os_spr::UndoPolicy::ExactBaseOnly);
        assert_eq!(op.merge_strategy(), crate::os_spr::MergeStrategyKind::LwwRegister);
        assert_eq!(op.conflict_rule(), crate::os_spr::ConflictRule::Merge(crate::os_spr::MergeStrategyKind::LwwRegister));
        assert_eq!(op.state_class(), crate::os_spr::StateClass::Persistent);
        assert!(op.validate(&0).is_ok());
        let (projection, reports) = op.reconcile(42);
        assert_eq!(projection, 42);
        assert!(reports.is_empty());
    }
    //#endregion 🧪️MutationLaws

    //#region 🧪️OpTextLaws
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
    //#endregion 🧪️OpTextLaws

    //#region 🧪️MetaSerde
    #[test]
    fn operation_meta_serde_round_trip() {
        let meta = MutationMeta {
            mutation_id: Some(crate::os_spr::ids::MutationId("op-1".into())),
            dependencies: vec![crate::os_spr::ids::MutationId("op-0".into())],
            base_version: 3,
            author_id: Some(crate::os_spr::ids::ActorId("actor-1".into())),
            timestamp: crate::os_spr::ids::HybridLogicalTimestamp::new(1, 1000),
            undo_policy: crate::os_spr::UndoPolicy::TransformAgainstConcurrent,
            payload_hash: Some(crate::os_spr::ids::PayloadHash([7u8; 32])),
        };
        let json = serde_json::to_string(&meta).expect("serialize");
        let round_tripped: MutationMeta = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(round_tripped, meta);
    }

    #[test]
    fn edit_serde_round_trip() {
        let edit = Edit::<AddOp> {
            id: "edit-1".into(),
            actor: Some("actor-1".into()),
            forwards: vec![AddOp { delta: 1 }, AddOp { delta: 2 }],
            inverse: vec![AddOp { delta: -2 }, AddOp { delta: -1 }],
            mutation_meta: vec![MutationMeta {
                mutation_id: None,
                dependencies: Vec::new(),
                base_version: 0,
                author_id: None,
                timestamp: crate::os_spr::ids::HybridLogicalTimestamp::new(1, 0),
                undo_policy: crate::os_spr::UndoPolicy::ExactBaseOnly,
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
    //#endregion 🧪️MetaSerde

    //#region 🧪️CollectionLaws
    #[test]
    fn apply_add_remove_move_patch() {
        let mut items = vec![Item { id: "a".into(), value: 1 }, Item { id: "b".into(), value: 2 }];

        apply_collection_mutation(&mut items, &CollectionMutation::Add { index: 1, item: Item { id: "c".into(), value: 3 } });
        assert_eq!(items.iter().map(|i| i.id.clone()).collect::<Vec<_>>(), vec!["a", "c", "b"]);

        apply_collection_mutation(&mut items, &CollectionMutation::Move { id: "c".into(), to_index: 2 });
        assert_eq!(items.iter().map(|i| i.id.clone()).collect::<Vec<_>>(), vec!["a", "b", "c"]);

        apply_collection_mutation::<String, Item, i64>(&mut items, &CollectionMutation::Patch { id: "b".into(), patch: 10 });
        assert_eq!(items.iter().find(|i| i.id == "b").unwrap().value, 12);

        apply_collection_mutation(&mut items, &CollectionMutation::Remove { id: "a".into() });
        assert_eq!(items.iter().map(|i| i.id.clone()).collect::<Vec<_>>(), vec!["b", "c"]);
    }

    #[test]
    fn invert_collection_operation_round_trips_every_kind() {
        let original = vec![Item { id: "a".into(), value: 1 }, Item { id: "b".into(), value: 2 }];

        let add = CollectionMutation::Add { index: 2, item: Item { id: "c".into(), value: 3 } };
        let mut items = original.clone();
        apply_collection_mutation(&mut items, &add);
        let inverse = inverse_collection_mutation(&original, &add);
        apply_collection_mutation(&mut items, &inverse);
        assert_eq!(items, original);

        let mov = CollectionMutation::<String, Item, i64>::Move { id: "b".into(), to: 0 };
        let mut items = original.clone();
        apply_collection_mutation(&mut items, &mov);
        let inverse = inverse_collection_mutation(&original, &mov);
        apply_collection_mutation(&mut items, &inverse);
        assert_eq!(items, original);

        let patch = CollectionMutation::Patch { id: "a".into(), patch: 9i64 };
        let mut items = original.clone();
        apply_collection_mutation(&mut items, &patch);
        let inverse = inverse_collection_mutation(&original, &patch);
        apply_collection_mutation(&mut items, &inverse);
        assert_eq!(items, original);

        let remove = CollectionMutation::<String, Item, i64>::Remove { id: "a".into() };
        let mut items = original.clone();
        apply_collection_mutation(&mut items, &remove);
        let inverse = inverse_collection_mutation(&original, &remove);
        apply_collection_mutation(&mut items, &inverse);
        assert_eq!(items, original);
    }

    #[test]
    fn collection_diff_from_operation_projects_each_kind() {
        let items = vec![Item { id: "a".into(), value: 1 }, Item { id: "b".into(), value: 2 }];

        let add = CollectionMutation::<String, Item, i64>::Add { id: "c".into(), item: Item { id: "c".into(), value: 3 }, at: 0 };
        let diff = collection_diff_from_mutation(&items, &add);
        assert_eq!(diff.added, vec![Item { id: "c".into(), value: 3 }]);
        assert!(diff.removed.is_empty() && diff.modified.is_empty());

        let remove = CollectionMutation::<String, Item, i64>::Remove { id: "a".into() };
        let diff = collection_diff_from_mutation(&items, &remove);
        assert_eq!(diff.removed, vec!["a".to_string()]);

        let patch = CollectionMutation::Patch { id: "b".into(), patch: 5i64 };
        let diff = collection_diff_from_mutation(&items, &patch);
        assert_eq!(diff.modified, vec![ItemPatch { id: "b".into(), patch: 5i64 }]);

        let mov = CollectionMutation::<String, Item, i64>::Move { id: "a".into(), to: 1 };
        let diff = collection_diff_from_mutation(&items, &mov);
        assert_eq!(diff.removed, vec!["a".to_string()]);
        assert_eq!(diff.added, vec![Item { id: "a".into(), value: 1 }]);
    }
    //#endregion 🧪️CollectionLaws

    //#region 🧪️DescriptorLaws
    #[test]
    fn operation_descriptor_fingerprint_is_golden_pinned() {
        let descriptor =
            MutationDescriptor::new(crate::os_spr::ids::SchemaId("note.append".into()), crate::os_spr::ids::SchemaVersion(1), crate::os_spr::StateClass::Persistent, crate::os_spr::ConflictRule::Merge(crate::os_spr::MergeStrategyKind::TextSequence));
        let hex: String = descriptor.fingerprint.iter().map(|b| format!("{b:02x}")).collect();
        // Golden pin computed once from `descriptor_fingerprint`'s canonical-JSON+blake3 encoding;
        // any change to that encoding (or to serde's field order/derives on the id/enum types it
        // hashes) is a breaking change to every persisted `MutationDescriptor` and must update this.
        assert_eq!(hex, "8c6c0b22512540811343d8caa8d147f48e936728105861c4360bb202f626d517");
    }
    //#endregion 🧪️DescriptorLaws

    //#region 🧪️UpcastLaws
    // Clamp-to-floor is the simplest genuinely idempotent upcaster: `upcast(upcast(x)) ==
    // upcast(x)` holds because `max(max(x, 10), 10) == max(x, 10)` for every `x`.
    struct ClampToFloor;
    impl MutationUpcaster<i64> for ClampToFloor {
        fn upcast(&self, _from_version: crate::os_spr::ids::SchemaVersion, op: i64) -> i64 {
            op.max(10)
        }
    }

    #[test]
    fn upcaster_is_idempotent_at_target_version() {
        let upcaster = ClampToFloor;
        let version = crate::os_spr::ids::SchemaVersion(1);
        for start in [0i64, 3, 7, 10, 40] {
            let once = upcaster.upcast(version, start);
            let twice = upcaster.upcast(version, once);
            assert_eq!(once, twice);
        }
    }
    //#endregion 🧪️UpcastLaws

    //#region 🧪️OutcomeLaws
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
        let event = MutationEvent { mutation_id: crate::os_spr::ids::MutationId("op-1".into()), state_class: crate::os_spr::StateClass::Effect, payload: serde_json::json!({ "kind": "toast", "text": "saved" }) };
        let json = serde_json::to_string(&event).expect("serialize");
        let round_tripped: MutationEvent = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(round_tripped, event);
    }
    //#endregion 🧪️OutcomeLaws
}
//#endregion 🧪️Tests
