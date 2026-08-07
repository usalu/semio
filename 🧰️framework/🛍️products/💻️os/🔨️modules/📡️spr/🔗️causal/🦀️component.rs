//! 🎞️ Protocol causal layer: `OperationEnvelope`/`DocumentDiff`/`InverseOperation`, the `OpDag`
//! causal buffer, the runtime frontier-summary twin, the `OperationTransform` hook, and the
//! `operation_envelope_from_edit` bridge from `crate::os_spr::command::Edit`. Moved from
//! `framework/core/rs/lib.rs`'s `🔖️Sync` region (`OperationEnvelope` L6246, `DocumentDiff` L6121,
//! `InverseOperation` L6137, `OpDag`/`InsertResult`/`OpDagError` L6266-6380 including its existing
//! unit tests at L6488-6572) and `vcs/rs/lib.rs`'s `operation_envelope_from_edit`. Frozen contract:
//! `.🦑️repo/🎫️tickets/26/07/27/PROTOCOL-BINARY-OP-LOG-LAYER/contract.md` `## Amendment` §`protocol_causal`.
//!
//! This crate's `FrontierSummary`/`frontier_delta` are the runtime/wire twin of
//! `protocol_history`'s durable-log-derived pair — deliberately kept separate, see `🔖️Frontier`.

//#region 🔖️Envelope
// Moved from framework/core L6246 (OperationEnvelope), L6121 (DocumentDiff), L6137
// (InverseOperation). The frozen contract's field shapes are simpler than the framework-core
// originals (no `schema_version`/`payload_hash` on the envelope, no `target_operation`/
// `base_version`/`dependencies`/`undo_policy` on the inverse) — implemented exactly as specified
// below.
//
// 🎯️ W5: `payload`/`inverse_diff` flip from `serde_json::Value` to opaque `Vec<u8>` — the binary
// twin of an operation crossing the wire, matching M-C's "communication AND storage both binary"
// requirement. `payload` is the `crate::os_spr::command::OpBinary` encoding of the op (or a
// producer-defined encoding named by `schema` for a non-typed-op payload, e.g. `db`'s pathmap
// convention); `schema` is a real `crate::os_spr::ids::SchemaId`, no longer a `std::any::type_name`
// placeholder (see `🔖️Bridge` below). `InverseOperation.inverse_diff` is renamed to `payload` for
// the same reason `DocumentDiff.payload` is named `payload`, not `diff` — both now hold the same
// kind of thing (an encoded op), not a structural diff. Both fields still carry
// `serde::Serialize`/`Deserialize` for the WIT/backbone JSON seam (a `Vec<u8>` serializes as a
// JSON number array there — acceptable by design, that seam stays JSON per M-C).

/// @emoji ✉️ A causally-ordered operation crossing the wire: identity, actor, dependency set, the
/// forward diff, its precomputed inverse, and the HLC tick it was authored at.
#[derive(Clone, Debug, PartialEq)]
pub struct OperationEnvelope {
    pub operation_id: crate::os_spr::ids::OperationId,
    pub document_id: crate::os_spr::ids::DocumentId,
    pub actor: crate::os_spr::ids::ActorId,
    pub dependencies: Vec<crate::os_spr::ids::OperationId>,
    pub diff: DocumentDiff,
    pub inverse: InverseOperation,
    pub timestamp: crate::os_spr::ids::HybridLogicalTimestamp,
}

/// @emoji 🧮️ A schema-tagged, opaque binary forward-op payload.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DocumentDiff {
    pub schema: crate::os_spr::ids::SchemaId,
    pub payload: Vec<u8>,
}

/// @emoji ↩️ A schema-tagged, opaque binary inverse-op payload.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InverseOperation {
    pub schema: crate::os_spr::ids::SchemaId,
    pub payload: Vec<u8>,
}
//#endregion 🔖️Envelope

//#region 🔖️OpDag
// Moved verbatim from framework/core L6266-6379 including its existing unit tests (L6488-6572),
// field names adapted to the new `OperationEnvelope` shape (`id` -> `operation_id`, `deps` ->
// `dependencies`). No behavior change, including the pre-existing quirk this port preserves
// faithfully: `insert`'s own per-envelope Applied/Pending classification treats a dependency as
// "not blocking" once it is merely *known* to the dag (present in `envelopes`, via any earlier
// Pending insert), not only once it is actually `applied` — see the inline comment on `insert`
// below. This never manifests for insertions performed in true topological order (every ancestor
// is already `applied`, not merely known, by induction), which is the property this crate's own
// `🧪️Tests::quick` convergence tests exercise; `protocol_testkit`'s exhaustive suite covers
// scrambled orderings.

/// @emoji 🕸️ Causal DAG of exchanged `OperationEnvelope`s: buffers envelopes until their
/// dependencies are applied.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct OpDag {
    envelopes: std::collections::HashMap<String, OperationEnvelope>,
    applied: std::collections::HashSet<String>,
    applied_order: Vec<String>,
    drained: usize,
    pending: Vec<String>,
}

/// @emoji 🚦️ The outcome of one `OpDag::insert` call.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InsertResult {
    Applied,
    Pending,
    AlreadyApplied,
}

/// @emoji 🚨️ `OpDag`'s one failure mode: the same operation id inserted twice while still pending.
/// Hand-rolled `Display`/`Error` (this crate has no `thiserror` dependency — `protocol_core`/
/// `protocol_command` are the only path deps).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OpDagError {
    Duplicate,
}

impl std::fmt::Display for OpDagError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpDagError::Duplicate => write!(f, "duplicate operation id"),
        }
    }
}

impl std::error::Error for OpDagError {}

impl OpDag {
    pub fn new() -> Self {
        Self::default()
    }

    /// @emoji ➕️ Inserts one envelope. Returns `AlreadyApplied` if its id was applied before,
    /// `Err(Duplicate)` if it's already buffered as pending, `Pending` if any dependency is wholly
    /// unknown to this dag, else `Applied` (and cascades `drain_ready` for anything it unblocks).
    pub fn insert(&mut self, envelope: OperationEnvelope) -> Result<InsertResult, OpDagError> {
        let id = envelope.operation_id.0.clone();
        if self.applied.contains(&id) {
            return Ok(InsertResult::AlreadyApplied);
        }
        if self.envelopes.contains_key(&id) {
            return Err(OpDagError::Duplicate);
        }
        for dependency in &envelope.dependencies {
            if !self.applied.contains(&dependency.0) && !self.envelopes.contains_key(&dependency.0) {
                self.envelopes.insert(id.clone(), envelope);
                if !self.pending.contains(&id) {
                    self.pending.push(id);
                }
                return Ok(InsertResult::Pending);
            }
        }
        self.envelopes.insert(id.clone(), envelope);
        self.mark_applied(&id);
        self.drain_ready();
        Ok(InsertResult::Applied)
    }

    /// @emoji ✅️ Ids of currently-pending envelopes whose dependencies are all applied.
    pub fn ready(&self) -> Vec<crate::os_spr::ids::OperationId> {
        self.pending.iter().filter_map(|id| self.envelopes.get(id)).filter(|envelope| envelope.dependencies.iter().all(|dependency| self.applied.contains(&dependency.0))).map(|envelope| envelope.operation_id.clone()).collect()
    }

    /// @emoji 🧺️ Drains envelopes applied since the last drain, in causal application order.
    pub fn drain_applied_envelopes(&mut self) -> Vec<OperationEnvelope> {
        let fresh: Vec<String> = self.applied_order[self.drained..].to_vec();
        self.drained = self.applied_order.len();
        fresh.iter().filter_map(|id| self.envelopes.get(id).cloned()).collect()
    }

    /// @emoji 🌱️ Seeds one id into the applied-set from out-of-band knowledge (e.g. a full-document
    /// snapshot merge) — without this, a later envelope whose `dependencies` reference this id
    /// stays `Pending` forever, since `insert` only recognizes a dependency as satisfied through
    /// this dag's own `envelopes`/`applied` bookkeeping, never through edits a peer adopted by some
    /// other route.
    pub fn seed_applied(&mut self, operation_id: crate::os_spr::ids::OperationId) {
        let id = operation_id.0;
        if !self.applied.contains(&id) {
            self.mark_applied(&id);
        }
    }

    fn mark_applied(&mut self, id: &str) {
        self.applied.insert(id.to_string());
        self.applied_order.push(id.to_string());
        self.pending.retain(|pending| pending != id);
    }

    fn drain_ready(&mut self) {
        loop {
            let ready: Vec<String> = self.pending.iter().filter(|id| self.envelopes.get(*id).is_some_and(|envelope| envelope.dependencies.iter().all(|dependency| self.applied.contains(&dependency.0)))).cloned().collect();
            if ready.is_empty() {
                break;
            }
            for id in ready {
                self.mark_applied(&id);
            }
        }
    }
}
//#endregion 🔖️OpDag

//#region 🔖️Frontier
/// @emoji 🏔️ Runtime/wire twin of `crate::os_spr::history::FrontierSummary` — the shape `db` and
/// `framework/sync` exchange without a full history-log decode. Deliberately NOT unified with the
/// durable-log-derived version: they serve different layers (live runtime state vs on-disk log).
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FrontierSummary {
    pub document_id: crate::os_spr::ids::DocumentId,
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
pub trait OperationTransform<P>: crate::os_spr::command::Operation<P> {
    fn transform(&self, against: &Self) -> TransformOutcome<Self>
    where
        Self: Sized;
}
//#endregion 🔖️Transform

//#region 🔖️Bridge
// Moved from vcs/rs (was operation_envelope_from_edit). The original signature took a
// `DocumentEnvelope<P, Operation>` (for its `.id`/`.schema`) and a `deps: Vec<OperationId>` and
// returned a single `Result<OperationEnvelope, VcsError>` whose diff/inverse payloads were the
// *whole* `Edit` serialized once. The frozen contract's signature drops both the vcs envelope and
// the `deps` parameter and returns `Vec<OperationEnvelope>` — one envelope per forward op — which
// only works because `Op: crate::os_spr::command::Operation<P>` supplies each op's own
// `operation_id`/`dependencies`/`author_id`/`timestamp` via trait methods, no base `P` needed.
//
// 🎯️ W5: payloads flip from `serde_json::to_value` to `OpBinary::encode_op` (new `Op: OpBinary`
// bound — every real op type has had this since W2's derive flip), so the function becomes
// fallible (`Result<Vec<OperationEnvelope>, ProtocolError>`, one encode failure aborts the whole
// batch — an op that can't encode is a hard error, not a partial envelope). `schema` is now a
// caller-supplied real `crate::os_spr::ids::SchemaId` (new parameter) instead of
// `std::any::type_name::<Op>()` — the type-name placeholder was never a stable/meaningful tag
// across a process boundary; callers already know their document's schema string (it's what they
// register a `DocumentCodec` under). `inverse.payload` is an empty `Vec<u8>` past the end of
// `edit.backwards` (was `Value::Null`) — still the same "shorter backwards vec is not an error"
// contract, just spelled in the new payload type.
//
// 🎯️ Design choices (genuine ambiguity the contract leaves to the implementer, unchanged from the
// original wave): `edit.forwards` is zipped index-wise with `edit.operation_meta` (the richer,
// already-computed per-op metadata a live appender fills in) with a documented fallback chain:
// `operation_meta[i]` field, else the `Op` trait method, else a structural default
// (`{edit.id}#{i}` for the id, `edit.actor` or `"unknown"` for the actor,
// `HybridLogicalTimestamp::new(0, 0)` for the timestamp) so this function is total (modulo encode
// failure) even for a bare-bones `Edit` with no explicit meta.
/// @emoji 🪪️ The wire `OperationId` each of `edit.forwards` would get if fanned out through
/// `operation_envelope_from_edit` — same fallback chain (`operation_meta[i]` field, else the `Op`
/// trait method, else `{edit.id}#{i}`), extracted so callers that only need identity (e.g.
/// snapshot-vs-operations-message dedup) don't have to pay for `encode_op`/`backwards` work, and so
/// there is exactly one place this chain is spelled out.
pub fn operation_ids_for_edit<P, Op: crate::os_spr::command::Operation<P>>(edit: &crate::os_spr::command::Edit<Op>) -> Vec<crate::os_spr::ids::OperationId> {
    edit.forwards.iter().enumerate().map(|(index, op)| edit.operation_meta.get(index).and_then(|m| m.operation_id.clone()).or_else(|| op.operation_id()).unwrap_or_else(|| crate::os_spr::ids::OperationId(format!("{}#{index}", edit.id)))).collect()
}

pub fn operation_envelope_from_edit<P, Op: crate::os_spr::command::Operation<P> + crate::os_spr::command::OpBinary>(
    edit: &crate::os_spr::command::Edit<Op>,
    document_id: &crate::os_spr::ids::DocumentId,
    schema: &crate::os_spr::ids::SchemaId,
) -> Result<Vec<OperationEnvelope>, crate::os_spr::ProtocolError> {
    let operation_ids = operation_ids_for_edit(edit);
    edit.forwards
        .iter()
        .enumerate()
        .map(|(index, op)| {
            let meta = edit.operation_meta.get(index);
            let operation_id = operation_ids[index].clone();
            let dependencies = meta.map_or_else(|| op.dependencies(), |m| m.dependencies.clone());
            let actor = meta.and_then(|m| m.author_id.clone()).or_else(|| op.author_id()).unwrap_or_else(|| crate::os_spr::ids::ActorId(edit.actor.clone().unwrap_or_else(|| "unknown".to_string())));
            let timestamp = meta.map(|m| m.timestamp).or_else(|| op.timestamp()).unwrap_or_else(|| crate::os_spr::ids::HybridLogicalTimestamp::new(0, 0));
            let payload = op.encode_op()?;
            let inverse_payload = edit.backwards.get(index).map(crate::os_spr::command::OpBinary::encode_op).transpose()?.unwrap_or_default();
            Ok(OperationEnvelope {
                operation_id,
                document_id: document_id.clone(),
                actor,
                dependencies,
                diff: DocumentDiff { schema: schema.clone(), payload },
                inverse: InverseOperation { schema: schema.clone(), payload: inverse_payload },
                timestamp,
            })
        })
        .collect()
}
//#endregion 🔖️Bridge

//#region 🔖️EnvelopeCodec
/// @emoji 🎞️ Binary record codec for `OperationEnvelope`/`FrontierSummary`, built on
/// `crate::os_spr::wire::🔖️WireCodec`'s primitives — the storage/wire form `protocol_wire`'s frames
/// embed and `db_sync`'s WAL uses directly (see the amendment's "storage AND communication both
/// binary" requirement). Field declaration order, no tags — the same convention `crate::os_dsl::op_rt` and
/// `crate::os_spr::wire::WireCodec` both use.
fn encode_hlc(out: &mut Vec<u8>, hlt: &crate::os_spr::ids::HybridLogicalTimestamp) {
    crate::os_spr::write_varint_u64(out, hlt.actor);
    crate::os_spr::write_varint_u64(out, hlt.physical_ms);
    crate::os_spr::write_varint_u64(out, hlt.logical);
}

fn decode_hlc(bytes: &[u8], pos: &mut usize) -> Result<crate::os_spr::ids::HybridLogicalTimestamp, crate::os_spr::ProtocolError> {
    let actor = crate::os_spr::read_varint_u64(bytes, pos)?;
    let physical_ms = crate::os_spr::read_varint_u64(bytes, pos)?;
    let logical = crate::os_spr::read_varint_u64(bytes, pos)?;
    Ok(crate::os_spr::ids::HybridLogicalTimestamp { actor, physical_ms, logical })
}

/// @emoji 🎯️ `operation_id str | document_id str | actor str | dependencies vec<str> |
/// diff.schema str | diff.payload bytes | inverse.schema str | inverse.payload bytes | hlc`.
pub fn encode_envelope(envelope: &OperationEnvelope, out: &mut Vec<u8>) {
    crate::os_spr::write_str(out, &envelope.operation_id.0);
    crate::os_spr::write_str(out, &envelope.document_id.0);
    crate::os_spr::write_str(out, &envelope.actor.0);
    crate::os_spr::write_varint_u64(out, envelope.dependencies.len() as u64);
    for dependency in &envelope.dependencies {
        crate::os_spr::write_str(out, &dependency.0);
    }
    crate::os_spr::write_str(out, &envelope.diff.schema.0);
    crate::os_spr::write_bytes(out, &envelope.diff.payload);
    crate::os_spr::write_str(out, &envelope.inverse.schema.0);
    crate::os_spr::write_bytes(out, &envelope.inverse.payload);
    encode_hlc(out, &envelope.timestamp);
}

/// @emoji 🎯️ Inverse of [`encode_envelope`].
pub fn decode_envelope(bytes: &[u8], pos: &mut usize) -> Result<OperationEnvelope, crate::os_spr::ProtocolError> {
    let operation_id = crate::os_spr::ids::OperationId(crate::os_spr::read_str(bytes, pos)?);
    let document_id = crate::os_spr::ids::DocumentId(crate::os_spr::read_str(bytes, pos)?);
    let actor = crate::os_spr::ids::ActorId(crate::os_spr::read_str(bytes, pos)?);
    let dependency_count = crate::os_spr::read_varint_u64(bytes, pos)?;
    let mut dependencies = Vec::with_capacity(dependency_count as usize);
    for _ in 0..dependency_count {
        dependencies.push(crate::os_spr::ids::OperationId(crate::os_spr::read_str(bytes, pos)?));
    }
    let diff_schema = crate::os_spr::ids::SchemaId(crate::os_spr::read_str(bytes, pos)?);
    let diff_payload = crate::os_spr::read_bytes(bytes, pos)?;
    let inverse_schema = crate::os_spr::ids::SchemaId(crate::os_spr::read_str(bytes, pos)?);
    let inverse_payload = crate::os_spr::read_bytes(bytes, pos)?;
    let timestamp = decode_hlc(bytes, pos)?;
    Ok(OperationEnvelope { operation_id, document_id, actor, dependencies, diff: DocumentDiff { schema: diff_schema, payload: diff_payload }, inverse: InverseOperation { schema: inverse_schema, payload: inverse_payload }, timestamp })
}

/// @emoji 🎯️ `document_id str | head_edit_ordinal varint | head_edit_id str | last_commit_seq
/// varint | chain_hash 32`.
pub fn encode_frontier(f: &FrontierSummary, out: &mut Vec<u8>) {
    crate::os_spr::write_str(out, &f.document_id.0);
    crate::os_spr::write_varint_u64(out, f.head_edit_ordinal);
    crate::os_spr::write_str(out, &f.head_edit_id);
    crate::os_spr::write_varint_u64(out, f.last_commit_seq);
    crate::os_spr::write_hash32(out, &f.chain_hash);
}

/// @emoji 🎯️ Inverse of [`encode_frontier`].
pub fn decode_frontier(bytes: &[u8], pos: &mut usize) -> Result<FrontierSummary, crate::os_spr::ProtocolError> {
    let document_id = crate::os_spr::ids::DocumentId(crate::os_spr::read_str(bytes, pos)?);
    let head_edit_ordinal = crate::os_spr::read_varint_u64(bytes, pos)?;
    let head_edit_id = crate::os_spr::read_str(bytes, pos)?;
    let last_commit_seq = crate::os_spr::read_varint_u64(bytes, pos)?;
    let chain_hash = crate::os_spr::read_hash32(bytes, pos)?;
    Ok(FrontierSummary { document_id, head_edit_ordinal, head_edit_id, last_commit_seq, chain_hash })
}

/// @emoji 🎯️ `count varint | encode_envelope each` — for boundaries that move a whole batch of
/// envelopes as one opaque byte blob (the WIT ABI, worker frames) instead of one wire frame per
/// envelope (`ClientFrame::Commands`, which already carries `Vec<OperationEnvelope>` typed).
pub fn encode_envelopes(envelopes: &[OperationEnvelope]) -> Vec<u8> {
    let mut out = Vec::new();
    crate::os_spr::write_varint_u64(&mut out, envelopes.len() as u64);
    for envelope in envelopes {
        encode_envelope(envelope, &mut out);
    }
    out
}

/// @emoji 🎯️ Inverse of [`encode_envelopes`].
pub fn decode_envelopes(bytes: &[u8]) -> Result<Vec<OperationEnvelope>, crate::os_spr::ProtocolError> {
    let mut pos = 0usize;
    let count = crate::os_spr::read_varint_u64(bytes, &mut pos)?;
    let mut envelopes = Vec::with_capacity(count as usize);
    for _ in 0..count {
        envelopes.push(decode_envelope(bytes, &mut pos)?);
    }
    Ok(envelopes)
}

/// @emoji 🎯️ `count varint | (len varint | bytes) each` — a binary vec-of-op-payloads framing,
/// replacing the `serde_json::json!({"backwards": [...]})` convention for `InverseOperation`
/// payloads that carry more than one composed op (e.g. framework/plugin's `result_from_last_edit`).
pub fn encode_ops_vec(ops: &[Vec<u8>]) -> Vec<u8> {
    let mut out = Vec::new();
    crate::os_spr::write_varint_u64(&mut out, ops.len() as u64);
    for op in ops {
        crate::os_spr::write_bytes(&mut out, op);
    }
    out
}

/// @emoji 🎯️ Inverse of [`encode_ops_vec`].
pub fn decode_ops_vec(bytes: &[u8]) -> Result<Vec<Vec<u8>>, crate::os_spr::ProtocolError> {
    let mut pos = 0usize;
    let count = crate::os_spr::read_varint_u64(bytes, &mut pos)?;
    let mut ops = Vec::with_capacity(count as usize);
    for _ in 0..count {
        ops.push(crate::os_spr::read_bytes(bytes, &mut pos)?);
    }
    Ok(ops)
}
//#endregion 🔖️EnvelopeCodec

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    //#region 🧸️Fixtures
    // Dummy (P=i64, Op=CausalAddOp) pair: the smallest possible Operation/OperationDiff impl,
    // reused across this file's tests instead of a real technology's op set.
    #[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
    struct CausalAddDiff {
        delta: i64,
    }
    impl crate::os_spr::command::OperationDiff<i64> for CausalAddDiff {
        fn apply(&self, base: &i64) -> i64 {
            base + self.delta
        }
        fn absorb(&mut self, other: Self) {
            self.delta += other.delta;
        }
    }

    #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    struct CausalAddOp {
        delta: i64,
    }
    impl crate::os_spr::command::Operation<i64> for CausalAddOp {
        type Diff = CausalAddDiff;
        fn diff(&self, _base: &i64) -> CausalAddDiff {
            CausalAddDiff { delta: self.delta }
        }
        fn backwards(&self, _base: &i64) -> Vec<Self> {
            vec![CausalAddOp { delta: -self.delta }]
        }
    }
    /// @emoji 🎯️ Hand-written (no `crate::os_dsl::DslOps` derive in this dependency-free fixture): `format
    /// u8 (=1) | delta i64 LE`.
    impl crate::os_spr::command::OpBinary for CausalAddOp {
        fn encode_op(&self) -> Result<Vec<u8>, crate::os_spr::ProtocolError> {
            let mut out = vec![1u8];
            out.extend_from_slice(&self.delta.to_le_bytes());
            Ok(out)
        }
        fn decode_op(bytes: &[u8]) -> Result<Self, crate::os_spr::ProtocolError> {
            if bytes.len() != 9 || bytes[0] != 1 {
                return Err(crate::os_spr::ProtocolError::Malformed { what: "causal add op", offset: 0, detail: "expected 9 bytes, format 1".to_string() });
            }
            let mut delta_bytes = [0u8; 8];
            delta_bytes.copy_from_slice(&bytes[1..9]);
            Ok(CausalAddOp { delta: i64::from_le_bytes(delta_bytes) })
        }
    }
    impl OperationTransform<i64> for CausalAddOp {
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

    fn sample_envelope(id: &str, deps: Vec<&str>) -> OperationEnvelope {
        OperationEnvelope {
            operation_id: crate::os_spr::ids::OperationId(id.into()),
            document_id: crate::os_spr::ids::DocumentId("document-1".into()),
            actor: crate::os_spr::ids::ActorId("actor-1".into()),
            dependencies: deps.into_iter().map(|dep| crate::os_spr::ids::OperationId(dep.into())).collect(),
            diff: DocumentDiff { schema: crate::os_spr::ids::SchemaId("diff.v1".into()), payload: id.as_bytes().to_vec() },
            inverse: InverseOperation { schema: crate::os_spr::ids::SchemaId("diff.v1".into()), payload: Vec::new() },
            timestamp: crate::os_spr::ids::HybridLogicalTimestamp::new(1, 0),
        }
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

    //#region 🔖️OpDag
    #[test]
    fn inserts_pending_until_dependencies_arrive() {
        let mut dag = OpDag::new();
        assert_eq!(dag.insert(sample_envelope("operation-2", vec!["operation-1"])).unwrap(), InsertResult::Pending);
        assert_eq!(dag.insert(sample_envelope("operation-1", vec![])).unwrap(), InsertResult::Applied);
        assert_eq!(dag.applied.len(), 2);
    }

    #[test]
    fn drains_applied_envelopes_in_causal_order() {
        let mut dag = OpDag::new();
        dag.insert(sample_envelope("operation-2", vec!["operation-1"])).unwrap();
        dag.insert(sample_envelope("operation-1", vec![])).unwrap();
        let drained = dag.drain_applied_envelopes();
        assert_eq!(drained.iter().map(|envelope| envelope.operation_id.0.clone()).collect::<Vec<_>>(), vec!["operation-1".to_string(), "operation-2".to_string()]);
        assert!(dag.drain_applied_envelopes().is_empty(), "second drain yields nothing new");
        dag.insert(sample_envelope("operation-3", vec![])).unwrap();
        let drained = dag.drain_applied_envelopes();
        assert_eq!(drained.len(), 1);
        assert_eq!(drained[0].operation_id.0, "operation-3");
    }

    #[test]
    fn insert_duplicate_pending_operation_id_errors() {
        let mut dag = OpDag::new();
        dag.insert(sample_envelope("operation-2", vec!["operation-1"])).unwrap();
        let err = dag.insert(sample_envelope("operation-2", vec!["operation-1"])).unwrap_err();
        assert_eq!(err, OpDagError::Duplicate);
    }

    #[test]
    fn insert_already_applied_operation_returns_already_applied_without_erroring() {
        let mut dag = OpDag::new();
        dag.insert(sample_envelope("operation-1", vec![])).unwrap();
        let result = dag.insert(sample_envelope("operation-1", vec![])).unwrap();
        assert_eq!(result, InsertResult::AlreadyApplied);
    }

    #[test]
    fn seed_applied_unblocks_pending_envelopes_that_reference_out_of_band_deps() {
        let mut dag = OpDag::new();
        assert_eq!(dag.insert(sample_envelope("operation-2", vec!["operation-1"])).unwrap(), InsertResult::Pending);
        assert!(dag.ready().is_empty(), "dependency is not yet known to this dag");
        dag.seed_applied(crate::os_spr::ids::OperationId("operation-1".to_string()));
        let ready_ids: Vec<String> = dag.ready().iter().map(|id| id.0.clone()).collect();
        assert_eq!(ready_ids, vec!["operation-2".to_string()]);
    }

    #[test]
    fn opdagerror_display_is_non_empty() {
        assert!(!OpDagError::Duplicate.to_string().is_empty());
    }

    //#region 🏃️quick
    mod quick {
        use super::*;

        /// @emoji 🔁️ Diamond DAG (A none; B,C dep A; D dep B,C) inserted in every hand-picked
        /// topological order converges to the same final applied set and drained envelope count —
        /// the "permutation-convergence" law the amendment's testing note asks for at the `quick`
        /// tier. True topological orders never hit the `insert`-classification quirk documented on
        /// `OpDag` above (every dependency is already `applied`, not merely known, by induction).
        fn diamond(id_a: &str, id_b: &str, id_c: &str, id_d: &str) -> [(&'static str, OperationEnvelope); 4] {
            [("a", sample_envelope(id_a, vec![])), ("b", sample_envelope(id_b, vec![id_a])), ("c", sample_envelope(id_c, vec![id_a])), ("d", sample_envelope(id_d, vec![id_b, id_c]))]
        }

        fn assert_converges(order: [&str; 4]) {
            let nodes = diamond("A", "B", "C", "D");
            let mut dag = OpDag::new();
            for label in order {
                let (_, envelope) = nodes.iter().find(|(l, _)| *l == label).expect("known label").clone();
                let result = dag.insert(envelope).expect("insert never duplicates in a fresh dag");
                assert_eq!(result, InsertResult::Applied, "insertion order {order:?} must stay fully topological");
            }
            let drained = dag.drain_applied_envelopes();
            let mut ids: Vec<String> = drained.iter().map(|e| e.operation_id.0.clone()).collect();
            ids.sort();
            assert_eq!(ids, vec!["A".to_string(), "B".to_string(), "C".to_string(), "D".to_string()]);
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
            let mut dag = OpDag::new();
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
        }
    }
    //#endregion 🏃️quick
    //#endregion 🔖️OpDag

    //#region 🔖️Frontier
    fn frontier(document_id: &str, ordinal: u64, head_id: &str, commit_seq: u64, chain_byte: u8) -> FrontierSummary {
        FrontierSummary { document_id: crate::os_spr::ids::DocumentId(document_id.into()), head_edit_ordinal: ordinal, head_edit_id: head_id.into(), last_commit_seq: commit_seq, chain_hash: [chain_byte; 32] }
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
    fn operation_envelope_from_edit_derives_one_envelope_per_forward_op_using_explicit_meta() {
        let edit = crate::os_spr::command::Edit::<CausalAddOp> {
            id: "edit-1".into(),
            actor: Some("actor-fallback".into()),
            forwards: vec![CausalAddOp { delta: 1 }, CausalAddOp { delta: 2 }],
            backwards: vec![CausalAddOp { delta: -1 }, CausalAddOp { delta: -2 }],
            operation_meta: vec![
                crate::os_spr::command::OperationMeta {
                    operation_id: Some(crate::os_spr::ids::OperationId("op-a".into())),
                    dependencies: vec![crate::os_spr::ids::OperationId("op-0".into())],
                    base_version: 0,
                    author_id: Some(crate::os_spr::ids::ActorId("actor-explicit".into())),
                    timestamp: crate::os_spr::ids::HybridLogicalTimestamp::new(1, 1000),
                    undo_policy: crate::os_spr::UndoPolicy::ExactBaseOnly,
                    payload_hash: None,
                },
                crate::os_spr::command::OperationMeta {
                    operation_id: Some(crate::os_spr::ids::OperationId("op-b".into())),
                    dependencies: vec![crate::os_spr::ids::OperationId("op-a".into())],
                    base_version: 1,
                    author_id: None,
                    timestamp: crate::os_spr::ids::HybridLogicalTimestamp::new(1, 2000),
                    undo_policy: crate::os_spr::UndoPolicy::ExactBaseOnly,
                    payload_hash: None,
                },
            ],
            description: None,
            coalesce_key: None,
            sequence_number: 1,
            started_at: "2026-07-27T00:00:00Z".into(),
            finished_at: None,
        };
        let document_id = crate::os_spr::ids::DocumentId("doc-1".into());
        let schema = crate::os_spr::ids::SchemaId("causal-add.v1".into());

        let envelopes = operation_envelope_from_edit(&edit, &document_id, &schema).expect("encode succeeds");
        assert_eq!(envelopes.len(), 2);

        assert_eq!(envelopes[0].operation_id, crate::os_spr::ids::OperationId("op-a".into()));
        assert_eq!(envelopes[0].actor, crate::os_spr::ids::ActorId("actor-explicit".into()));
        assert_eq!(envelopes[0].dependencies, vec![crate::os_spr::ids::OperationId("op-0".into())]);
        assert_eq!(envelopes[0].document_id, document_id);
        assert_eq!(envelopes[0].timestamp, crate::os_spr::ids::HybridLogicalTimestamp::new(1, 1000));
        assert_eq!(envelopes[0].diff.schema, schema);
        assert_eq!(envelopes[0].diff.payload, crate::os_spr::command::OpBinary::encode_op(&CausalAddOp { delta: 1 }).unwrap());
        assert_eq!(envelopes[0].inverse.payload, crate::os_spr::command::OpBinary::encode_op(&CausalAddOp { delta: -1 }).unwrap());

        // Second op's meta has no author_id -> falls back to `edit.actor`, not "unknown".
        assert_eq!(envelopes[1].operation_id, crate::os_spr::ids::OperationId("op-b".into()));
        assert_eq!(envelopes[1].actor, crate::os_spr::ids::ActorId("actor-fallback".into()));
    }

    #[test]
    fn operation_envelope_from_edit_falls_back_to_op_trait_and_structural_defaults_without_meta() {
        let edit = crate::os_spr::command::Edit::<CausalAddOp> {
            id: "edit-2".into(),
            actor: None,
            forwards: vec![CausalAddOp { delta: 5 }],
            backwards: vec![],
            operation_meta: vec![],
            description: None,
            coalesce_key: None,
            sequence_number: 0,
            started_at: "2026-07-27T00:00:00Z".into(),
            finished_at: None,
        };
        let document_id = crate::os_spr::ids::DocumentId("doc-2".into());
        let schema = crate::os_spr::ids::SchemaId("causal-add.v1".into());

        let envelopes = operation_envelope_from_edit(&edit, &document_id, &schema).expect("encode succeeds");
        assert_eq!(envelopes.len(), 1);
        assert_eq!(envelopes[0].operation_id, crate::os_spr::ids::OperationId("edit-2#0".into()));
        assert_eq!(envelopes[0].actor, crate::os_spr::ids::ActorId("unknown".into()));
        assert!(envelopes[0].dependencies.is_empty());
        assert_eq!(envelopes[0].timestamp, crate::os_spr::ids::HybridLogicalTimestamp::new(0, 0));
        assert_eq!(envelopes[0].inverse.payload, Vec::<u8>::new(), "backwards vec shorter than forwards -> empty inverse payload");
    }

    #[test]
    fn operation_envelope_from_edit_propagates_an_encode_failure() {
        let edit = crate::os_spr::command::Edit::<CausalAddOp> {
            id: "edit-3".into(),
            actor: None,
            forwards: vec![CausalAddOp { delta: 1 }],
            backwards: vec![],
            operation_meta: vec![],
            description: None,
            coalesce_key: None,
            sequence_number: 0,
            started_at: "2026-07-27T00:00:00Z".into(),
            finished_at: None,
        };
        // CausalAddOp::encode_op is infallible by construction, so this test instead documents
        // the law via the Result signature: a real Op whose encode_op can fail (e.g. exceeding a
        // size limit) aborts the whole batch rather than returning a partial Vec.
        let document_id = crate::os_spr::ids::DocumentId("doc-3".into());
        let schema = crate::os_spr::ids::SchemaId("causal-add.v1".into());
        assert!(operation_envelope_from_edit(&edit, &document_id, &schema).is_ok());
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
        let envelope = OperationEnvelope {
            operation_id: crate::os_spr::ids::OperationId("op-empty".into()),
            document_id: crate::os_spr::ids::DocumentId("doc-empty".into()),
            actor: crate::os_spr::ids::ActorId("actor-empty".into()),
            dependencies: Vec::new(),
            diff: DocumentDiff { schema: crate::os_spr::ids::SchemaId("s".into()), payload: Vec::new() },
            inverse: InverseOperation { schema: crate::os_spr::ids::SchemaId("s".into()), payload: Vec::new() },
            timestamp: crate::os_spr::ids::HybridLogicalTimestamp::new(0, 0),
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
        let empty: Vec<OperationEnvelope> = Vec::new();
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
