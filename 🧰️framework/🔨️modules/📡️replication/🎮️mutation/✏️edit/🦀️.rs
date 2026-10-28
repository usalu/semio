//! ✏️ The recorded Edit: the transaction reference, the per-operation mutation meta, the provenance origin and the canonical field cursors that seal them.
//!
//! Sits between the mutation contract below and the causal envelopes above; the optimistic replica and the authority record the same bytes.

use semio_framework_replication_mutation::ForeignTarget;

//#region 🔖️Meta

/// 🧾️ The committed tool transaction that authored an operation: `id` is `tx-<hex16>`
/// ([`TransactionRef::mint`]), `tool` the authoring action/tool id `<appId>#<toolId>`. Every
/// operation one transaction produced carries the same ref, in the edit ledger and on the wire.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[derive(semio_framework_value::RetainedClone)]
pub struct TransactionRef {
    pub id: String,
    pub tool: String,
}

impl TransactionRef {
    /// 🪪️ `tx-{hex16(blake3(actor | hlc | tool))}`: deterministic per author, tick and tool.
    pub fn mint(actor: &semio_framework_replication_ids::ActorId, timestamp: &semio_framework_replication_ids::HybridLogicalTimestamp, tool: impl Into<String>) -> Self {
        let tool = tool.into();
        let mut material = Vec::with_capacity(actor.0.len() + tool.len() + 32);
        semio_framework_replication_wire_core::write_str(&mut material, &actor.0);
        semio_framework_replication_wire_core::write_varint_u64(&mut material, timestamp.actor);
        semio_framework_replication_wire_core::write_varint_u64(&mut material, timestamp.physical_ms);
        semio_framework_replication_wire_core::write_varint_u64(&mut material, timestamp.logical);
        semio_framework_replication_wire_core::write_str(&mut material, &tool);
        let digest = semio_framework_replication_crypto::RecordHasher::hash(&semio_framework_replication_wire_core::Blake3Hasher, &material);
        let mut id = String::with_capacity("tx-".len() + 16);
        id.push_str("tx-");
        for byte in &digest[..8] {
            id.push_str(&format!("{byte:02x}"));
        }
        Self { id, tool }
    }
}

impl semio_framework_value::ToValue for TransactionRef {
    fn to_value(&self) -> semio_framework_value::DslValue {
        semio_framework_value::DslValue::object(vec![("id".to_string(), semio_framework_value::ToValue::to_value(&self.id)), ("tool".to_string(), semio_framework_value::ToValue::to_value(&self.tool))])
    }
}
impl semio_framework_value::FromValue for TransactionRef {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let semio_framework_value::DslValue::Object(fields) = value else {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for TransactionRef, found {value:?}")));
        };
        let mut id = None;
        let mut tool = None;
        for (key, entry) in fields {
            match key.as_str() {
                "id" => id = Some(<String as semio_framework_value::FromValue>::from_value(entry).map_err(|error| error.under("id"))?),
                "tool" => tool = Some(<String as semio_framework_value::FromValue>::from_value(entry).map_err(|error| error.under("tool"))?),
                _ => {}
            }
        }
        Ok(TransactionRef { id: id.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "TransactionRef missing id"))?, tool: tool.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "TransactionRef missing tool"))? })
    }
}

/// 📎️ Per-operation causal/undo metadata attached to one `Edit` slot. Moved from
/// `crate::os_store::MutationMeta` (was `vcs/rs/lib.rs` L59) with the id-flavored fields upgraded from bare
/// `String`/`Option<String>` to the `protocol_core` newtypes and `timestamp` upgraded from
/// `Option<HybridLogicalTimestamp>` to a required field (an edit's op always has a tick by the time
/// it is durable).
#[derive(Clone, Debug, PartialEq)]
pub struct MutationMeta {
    pub mutation_id: Option<semio_framework_replication_ids::MutationId>,
    pub dependencies: Vec<semio_framework_replication_ids::MutationId>,
    pub base_version: u64,
    pub author_id: Option<semio_framework_replication_ids::ActorId>,
    pub timestamp: semio_framework_replication_ids::HybridLogicalTimestamp,
    pub undo_policy: semio_framework_replication_wire_core::UndoPolicy,
    pub payload_hash: Option<semio_framework_replication_ids::PayloadHash>,
    /// 🗣️ `"<doc-schema>#<kind>"` once the authoring mutation implements [`SemanticMutation`]
    /// — additive, `None` for mutations still on generic vocabulary. Populated by callers (store
    /// replay tightens this once a store's `Mutation: SemanticMutation<P>`, at the final ratchet);
    /// this crate never derives it implicitly to avoid a premature trait-bound change here.
    pub semantic_kind: Option<semio_framework_replication_ids::SchemaId>,
    /// 🏷️ Human undo/history label captured at authoring time (`MutationKind::label`/
    /// `SemanticMutation::label`), so history UI stops reverse-engineering one from `print_op`.
    pub label: Option<String>,
    /// 🧑‍🤝‍🧑️ Composite-gesture stamp: `Some(id)` when this edit was authored as one member
    /// of a multi-document composite gesture (the future `CompositionCoordinator`'s atomic
    /// parent+child dispatch across several `ArtifactEnvelope`s), so group undo can find and
    /// reverse every sibling member together; `None` for a solitary, single-document edit.
    /// Additive, mirrors `semantic_kind`/`label` above. 🎞️ Wire representation is the bare id
    /// string `semio_framework::kernel::InvocationId` wraps, not that newtype itself: this crate
    /// (`semio-framework-os-kernel`) is a dependency `semio-framework` builds on (see
    /// `📦️packages/🦀️rust/Cargo.toml`), so importing `InvocationId` here would invert that edge —
    /// see `UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM/📓️wave1-reports/b1-spr-vcs-report.md` for the
    /// decision record. Callers holding a real `InvocationId` pass `.0`; this field round-trips
    /// through `Edit.mutation_meta` into the `.spr` history log (`HistoryOpMeta.group_id`,
    /// `📡️spr/📜️history/🦀️.rs`) so it survives persistence and sync.
    pub group_id: Option<String>,
    /// 🔀️ Provenance of this operation: `Owner` (default, omitted from wire — the
    /// overwhelming common case, an edit authored by the artifact's own owner logic), `Contributed`
    /// (synthesized by a contributor plugin's `contributor.artifact-mutation-plan` response), or
    /// `Transaction` (applied as one member of a cross-artifact composite gesture initiated
    /// elsewhere). Additive, mirrors `group_id`/`semantic_kind`/`label` above — see
    /// `.🧬semio/🦑️repo/🎫️tickets/26/08/16/PLUGIN-DEPENDENCIES-ARTIFACT-CONTRIBUTIONS-AND-COMPOSITE-MUTATIONS/📋️contract-freeze.md`
    /// §1.
    pub origin: MutationOrigin,
    /// 🔧️ The committed tool transaction that authored this operation (`None`: authored outside
    /// any tool transaction). Wire-omitted when `None`, like `group_id`.
    pub transaction: Option<TransactionRef>,
}

/// 🌉️ Hand-written, not derived — same DAG reason as the other replication-crate types above.
/// No `#[serde(rename_all = ...)]` on this struct, so field names stay snake_case (unlike
/// `ForeignTarget`/`MutationOrigin` above); mirrors every `skip_serializing_if` by hand.
impl semio_framework_value::ToValue for MutationMeta {
    fn to_value(&self) -> semio_framework_value::DslValue {
        let mut entries = Vec::new();
        if self.mutation_id.is_some() {
            entries.push(("mutation_id".to_string(), semio_framework_value::ToValue::to_value(&self.mutation_id)));
        }
        if !self.dependencies.is_empty() {
            entries.push(("dependencies".to_string(), semio_framework_value::ToValue::to_value(&self.dependencies)));
        }
        entries.push(("base_version".to_string(), semio_framework_value::ToValue::to_value(&self.base_version)));
        if self.author_id.is_some() {
            entries.push(("author_id".to_string(), semio_framework_value::ToValue::to_value(&self.author_id)));
        }
        entries.push(("timestamp".to_string(), semio_framework_value::ToValue::to_value(&self.timestamp)));
        entries.push(("undo_policy".to_string(), semio_framework_value::ToValue::to_value(&self.undo_policy)));
        if self.payload_hash.is_some() {
            entries.push(("payload_hash".to_string(), semio_framework_value::ToValue::to_value(&self.payload_hash)));
        }
        if self.semantic_kind.is_some() {
            entries.push(("semantic_kind".to_string(), semio_framework_value::ToValue::to_value(&self.semantic_kind)));
        }
        if self.label.is_some() {
            entries.push(("label".to_string(), semio_framework_value::ToValue::to_value(&self.label)));
        }
        if self.group_id.is_some() {
            entries.push(("group_id".to_string(), semio_framework_value::ToValue::to_value(&self.group_id)));
        }
        if !self.origin.is_owner() {
            entries.push(("origin".to_string(), semio_framework_value::ToValue::to_value(&self.origin)));
        }
        if self.transaction.is_some() {
            entries.push(("transaction".to_string(), semio_framework_value::ToValue::to_value(&self.transaction)));
        }
        semio_framework_value::DslValue::object(entries)
    }
}
impl semio_framework_value::FromValue for MutationMeta {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let semio_framework_value::DslValue::Object(fields) = value else {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for MutationMeta, found {value:?}")));
        };
        let mut mutation_id = None;
        let mut dependencies = Vec::new();
        let mut base_version = None;
        let mut author_id = None;
        let mut timestamp = None;
        let mut undo_policy = None;
        let mut payload_hash = None;
        let mut semantic_kind = None;
        let mut label = None;
        let mut group_id = None;
        let mut origin = MutationOrigin::Owner;
        let mut transaction = None;
        for (key, entry) in fields {
            match key.as_str() {
                "mutation_id" => mutation_id = <Option<semio_framework_replication_ids::MutationId> as semio_framework_value::FromValue>::from_value(entry).map_err(|error| error.under("mutation_id"))?,
                "dependencies" => dependencies = <Vec<semio_framework_replication_ids::MutationId> as semio_framework_value::FromValue>::from_value(entry).map_err(|error| error.under("dependencies"))?,
                "base_version" => base_version = Some(<u64 as semio_framework_value::FromValue>::from_value(entry).map_err(|error| error.under("base_version"))?),
                "author_id" => author_id = <Option<semio_framework_replication_ids::ActorId> as semio_framework_value::FromValue>::from_value(entry).map_err(|error| error.under("author_id"))?,
                "timestamp" => timestamp = Some(<semio_framework_replication_ids::HybridLogicalTimestamp as semio_framework_value::FromValue>::from_value(entry).map_err(|error| error.under("timestamp"))?),
                "undo_policy" => undo_policy = Some(<semio_framework_replication_wire_core::UndoPolicy as semio_framework_value::FromValue>::from_value(entry).map_err(|error| error.under("undo_policy"))?),
                "payload_hash" => payload_hash = <Option<semio_framework_replication_ids::PayloadHash> as semio_framework_value::FromValue>::from_value(entry).map_err(|error| error.under("payload_hash"))?,
                "semantic_kind" => semantic_kind = <Option<semio_framework_replication_ids::SchemaId> as semio_framework_value::FromValue>::from_value(entry).map_err(|error| error.under("semantic_kind"))?,
                "label" => label = <Option<String> as semio_framework_value::FromValue>::from_value(entry).map_err(|error| error.under("label"))?,
                "group_id" => group_id = <Option<String> as semio_framework_value::FromValue>::from_value(entry).map_err(|error| error.under("group_id"))?,
                "origin" => origin = <MutationOrigin as semio_framework_value::FromValue>::from_value(entry).map_err(|error| error.under("origin"))?,
                "transaction" => transaction = <Option<TransactionRef> as semio_framework_value::FromValue>::from_value(entry).map_err(|error| error.under("transaction"))?,
                _ => {}
            }
        }
        Ok(MutationMeta {
            mutation_id,
            dependencies,
            base_version: base_version.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "MutationMeta missing base_version"))?,
            author_id,
            timestamp: timestamp.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "MutationMeta missing timestamp"))?,
            undo_policy: undo_policy.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "MutationMeta missing undo_policy"))?,
            payload_hash,
            semantic_kind,
            label,
            group_id,
            origin,
            transaction,
        })
    }
}

/// 📝️ One coalesced batch of operations, forward and backward, plus their causal metadata.
/// Moved verbatim from `crate::os_store::Edit` (was `vcs/rs/lib.rs` L73).
#[derive(Clone, Debug, PartialEq)]
pub struct Edit<Op> {
    pub id: String,
    pub actor: Option<semio_framework_value::SharedUtf8>,
    /// 🌿️ Authored branch identity; explicit null identifies trunk or unknown provenance.
    pub line: Option<String>,
    pub forwards: Vec<Op>,
    pub inverse: semio_framework_value::RetainedPublicationRoot<semio_framework_value_persistent::list::PagedList<Op, {usize::MAX}>>,
    pub mutation_meta: Vec<MutationMeta>,
    /// 🖊️ The id of the action or command that authored this edit — never display text: history resolves it through
    /// the authoring app's registry to its label in every locale at projection time, so a reload or a peer keeps it.
    pub verb: Option<String>,
    pub sequence_number: i32,
    pub started_at: String,
    pub finished_at: Option<String>,
}

/// 🧰️ Hand-written, not derived — same DAG reason as the other replication-crate types above (and
/// the actual trigger of the os-kernel cascade: `CursorRevisionAccumulator::edit_digest`/
/// `reconcile_stack` already bound `Mutation: ToValue` and hash via `to_json_string`, which needs
/// `Edit<Mutation>: ToValue` itself, not just its generic parameter). Mirrors
/// `#[serde(rename_all = "camelCase")]` field naming and every `skip_serializing_if`; `Option<T>`
/// fields with no explicit `#[serde(default)]` (`finished_at`) still default to
/// `None` when absent — serde's built-in behavior for `Option<T>` fields, mirrored here too.
impl<Op: semio_framework_value::ToValue> semio_framework_value::ToValue for Edit<Op> {
    fn to_value(&self) -> semio_framework_value::DslValue {
        let mut entries = vec![("id".to_string(), semio_framework_value::ToValue::to_value(&self.id))];
        if self.actor.is_some() {
            entries.push(("actor".to_string(), semio_framework_value::ToValue::to_value(&self.actor)));
        }
        entries.push(("forwards".to_string(), semio_framework_value::ToValue::to_value(&self.forwards)));
        entries.push(("inverse".to_string(), semio_framework_value::ToValue::to_value(&self.inverse)));
        if !self.mutation_meta.is_empty() {
            entries.push(("mutationMeta".to_string(), semio_framework_value::ToValue::to_value(&self.mutation_meta)));
        }
        if self.verb.is_some() {
            entries.push(("verb".to_string(), semio_framework_value::ToValue::to_value(&self.verb)));
        }
        entries.push(("sequenceNumber".to_string(), semio_framework_value::ToValue::to_value(&self.sequence_number)));
        entries.push(("startedAt".to_string(), semio_framework_value::ToValue::to_value(&self.started_at)));
        if self.finished_at.is_some() {
            entries.push(("finishedAt".to_string(), semio_framework_value::ToValue::to_value(&self.finished_at)));
        }
        entries.push(("line".to_string(), semio_framework_value::ToValue::to_value(&self.line)));
        semio_framework_value::DslValue::object(entries)
    }
}
impl<Op: semio_framework_value::FromValue> semio_framework_value::FromValue for Edit<Op> {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let semio_framework_value::DslValue::Object(fields) = value else {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for Edit, found {value:?}")));
        };
        let mut id = None;
        let mut actor = None;
        let mut line = None;
        let mut forwards = None;
        let mut inverse = None;
        let mut mutation_meta = Vec::new();
        let mut verb = None;
        let mut sequence_number = None;
        let mut started_at = None;
        let mut finished_at = None;
        for (key, entry) in fields {
            match key.as_str() {
                "id" => id = Some(<String as semio_framework_value::FromValue>::from_value(entry).map_err(|error| error.under("id"))?),
                "line" => {
                    if line.is_some() { return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "Edit repeats line")); }
                    line = Some(<Option<String> as semio_framework_value::FromValue>::from_value(entry).map_err(|error| error.under("line"))?);
                }
                "actor" => actor = <Option<semio_framework_value::SharedUtf8> as semio_framework_value::FromValue>::from_value(entry).map_err(|error| error.under("actor"))?,
                "forwards" => forwards = Some(<Vec<Op> as semio_framework_value::FromValue>::from_value(entry).map_err(|error| error.under("forwards"))?),
                "inverse" => inverse = Some(<semio_framework_value_persistent::list::PagedList<Op, {usize::MAX}> as semio_framework_value::FromValue>::from_value(entry).map_err(|error| error.under("inverse"))?),
                "mutationMeta" => mutation_meta = <Vec<MutationMeta> as semio_framework_value::FromValue>::from_value(entry).map_err(|error| error.under("mutationMeta"))?,
                "verb" => verb = <Option<String> as semio_framework_value::FromValue>::from_value(entry).map_err(|error| error.under("verb"))?,
                "sequenceNumber" => sequence_number = Some(<i32 as semio_framework_value::FromValue>::from_value(entry).map_err(|error| error.under("sequenceNumber"))?),
                "startedAt" => started_at = Some(<String as semio_framework_value::FromValue>::from_value(entry).map_err(|error| error.under("startedAt"))?),
                "finishedAt" => finished_at = <Option<String> as semio_framework_value::FromValue>::from_value(entry).map_err(|error| error.under("finishedAt"))?,
                _ => {}
            }
        }
        Ok(Edit {
            id: id.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "Edit missing id"))?,
            actor,
            line: line.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "Edit missing line"))?,
            forwards: forwards.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "Edit missing forwards"))?,
            inverse: inverse.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "Edit missing inverse"))?.into(),
            mutation_meta,
            verb,
            sequence_number: sequence_number.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "Edit missing sequenceNumber"))?,
            started_at: started_at.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "Edit missing startedAt"))?,
            finished_at,
        })
    }
}

#[path = "../🧾️transaction/🎟️admission/🦀️.rs"]
pub mod transaction_admission;

#[cfg(test)]
#[path = "../🧪️tests/🧪️transaction-ref/🦀️.rs"]
mod transaction_ref_tests;

//#endregion 🔖️Meta

//#region 🔖️Origin
/// 📍️ Provenance of one [`MutationMeta`]-described operation. `Owner` is the default (and
/// the overwhelming common case), omitted from wire by `MutationMeta.origin`'s
/// `skip_serializing_if`.
#[derive(Clone, Debug, Default, PartialEq,semio_framework_value::NativeReceive,semio_framework_value::RetainedClone)]
#[value(tag="kind",rename_all="camelCase",rename_all_fields="camelCase")]
pub enum MutationOrigin {
    #[default]
    Owner,
    Contributed {
        plugin_id: String,
        mutation_id: semio_framework_replication_ids::SchemaId,
        payload_hash: semio_framework_replication_ids::PayloadHash,
    },
    Transaction {
        initiator: ForeignTarget,
    },
}
impl MutationOrigin {
    /// 🕳️ Whether this is the (wire-omitted) default owner origin.
    pub fn is_owner(&self) -> bool {
        matches!(self, Self::Owner)
    }
}

/// 🌱️ Hand-written, not derived — same DAG reason as the other replication-crate types above.
/// `#[serde(tag = "kind")]` internal tagging has no adjacently-tagged (`tag` + `content`) shape the
/// derive macro supports anyway, so this was always going to be hand-written; mirrors the wire
/// shape (`kind` sibling to each struct variant's own fields) exactly.
impl semio_framework_value::ToValue for MutationOrigin {
    fn to_value(&self) -> semio_framework_value::DslValue {
        match self {
            MutationOrigin::Owner => semio_framework_value::DslValue::object([("kind".to_string(), semio_framework_value::DslValue::String("owner".to_string()))]),
            MutationOrigin::Contributed { plugin_id, mutation_id, payload_hash } => semio_framework_value::DslValue::object([
                ("kind".to_string(), semio_framework_value::DslValue::String("contributed".to_string())),
                ("pluginId".to_string(), semio_framework_value::ToValue::to_value(plugin_id)),
                ("mutationId".to_string(), semio_framework_value::ToValue::to_value(mutation_id)),
                ("payloadHash".to_string(), semio_framework_value::ToValue::to_value(payload_hash)),
            ]),
            MutationOrigin::Transaction { initiator } => semio_framework_value::DslValue::object([
                ("kind".to_string(), semio_framework_value::DslValue::String("transaction".to_string())),
                ("initiator".to_string(), semio_framework_value::ToValue::to_value(initiator)),
            ]),
        }
    }
}
impl semio_framework_value::FromValue for MutationOrigin {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let semio_framework_value::DslValue::Object(fields) = value else {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for MutationOrigin, found {value:?}")));
        };
        let mut kind = None;
        let mut plugin_id = None;
        let mut mutation_id = None;
        let mut payload_hash = None;
        let mut initiator = None;
        for (key, entry) in fields {
            match key.as_str() {
                "kind" => kind = Some(<String as semio_framework_value::FromValue>::from_value(entry).map_err(|error| error.under("kind"))?),
                "pluginId" => plugin_id = Some(<String as semio_framework_value::FromValue>::from_value(entry).map_err(|error| error.under("pluginId"))?),
                "mutationId" => mutation_id = Some(<semio_framework_replication_ids::SchemaId as semio_framework_value::FromValue>::from_value(entry).map_err(|error| error.under("mutationId"))?),
                "payloadHash" => payload_hash = Some(<semio_framework_replication_ids::PayloadHash as semio_framework_value::FromValue>::from_value(entry).map_err(|error| error.under("payloadHash"))?),
                "initiator" => initiator = Some(<ForeignTarget as semio_framework_value::FromValue>::from_value(entry).map_err(|error| error.under("initiator"))?),
                _ => {}
            }
        }
        match kind.as_deref() {
            Some("owner") => Ok(MutationOrigin::Owner),
            Some("contributed") => Ok(MutationOrigin::Contributed {
                plugin_id: plugin_id.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "MutationOrigin::Contributed missing pluginId"))?,
                mutation_id: mutation_id.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "MutationOrigin::Contributed missing mutationId"))?,
                payload_hash: payload_hash.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "MutationOrigin::Contributed missing payloadHash"))?,
            }),
            Some("transaction") => Ok(MutationOrigin::Transaction {
                initiator: initiator.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "MutationOrigin::Transaction missing initiator"))?,
            }),
            Some(other) => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("unknown MutationOrigin kind `{other}`"))),
            None => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "MutationOrigin missing kind")),
        }
    }
}
//#endregion 🔖️Origin

#[path = "../🧵️canonical/🦀️.rs"]
mod canonical_fields;
pub use canonical_fields::ArtifactCanonicalEditIdentityCursor;
pub use canonical_fields::{ArtifactCanonicalEditAuthority,ArtifactCanonicalEditAuthorityCursor};

#[path = "../../🚪️io/📝️text/🧵️canonical/🦀️.rs"]
pub mod canonical;

mod owned_retirement {
    use super::*;
    use semio_framework_value::retirement::{RetireOwned, RetirementCursor, sequence, deferred, deferred_birth_bytes_for, sequence_birth_bytes};
    semio_framework_value::artifact_retire_struct!(TransactionRef { id, tool });
    impl RetireOwned for MutationOrigin {
        fn retirement(self) -> Box<dyn RetirementCursor> {
            match self {
                Self::Owner => sequence(Vec::new()),
                Self::Contributed { plugin_id, mutation_id, payload_hash } => sequence(vec![deferred(plugin_id), deferred(mutation_id.0), deferred(payload_hash.0)]),
                Self::Transaction { initiator } => sequence(vec![deferred(initiator.artifact_id), deferred(initiator.artifact_kind), deferred(initiator.dialect)]),
            }
        }
        fn retirement_birth_bytes(&self) -> Option<usize> { match self { Self::Owner => sequence_birth_bytes(&[]), Self::Contributed { plugin_id, mutation_id, payload_hash } => sequence_birth_bytes(&[deferred_birth_bytes_for(plugin_id), deferred_birth_bytes_for(&mutation_id.0), deferred_birth_bytes_for(&payload_hash.0)]), Self::Transaction { initiator } => sequence_birth_bytes(&[deferred_birth_bytes_for(&initiator.artifact_id), deferred_birth_bytes_for(&initiator.artifact_kind), deferred_birth_bytes_for(&initiator.dialect)]) } }
        fn controlled_retirement_supported() -> bool { true }
    }
}

#[cfg(test)]
#[global_allocator]
static TEST_ALLOCATION_OBSERVER: semio_framework_trace_memory_testing::RequestedAllocator = semio_framework_trace_memory_testing::RequestedAllocator;
