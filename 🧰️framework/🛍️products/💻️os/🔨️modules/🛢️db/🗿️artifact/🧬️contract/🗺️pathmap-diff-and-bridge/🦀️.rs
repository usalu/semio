//! 🗺️ Pathmap, diff and bridge: the pathmap schema and codecs, the diff, inverse and touched entries and the envelope bridge from an operation. See [`super`].

use super::*;

//#region 🔖️Ids
//#region 🔖️Diff
/// 🧬️ The schema tag `db_artifact` reserves for its own generic path-value diff
/// convention (see module doc) — the only `ArtifactDiff`/`InverseMutation` shape this crate
/// knows how to interpret. 🎯️ W5: `diff`/`inverse` payloads are opaque `Vec<u8>` on the wire now;
/// `db_artifact` still only understands ITS OWN JSON-object-of-paths convention, tagged with this
/// schema so `diff_entries`/`inverse_entries` can distinguish "our own pathmap bytes" from a
/// typed op's binary payload it has no business decoding (foreign schema -> empty `TouchedSet`,
/// not an error — the envelope is still persisted/relayed, just not interpreted at this layer;
/// that's `db_artifact`-and-above's future typed path).
pub const DB_PATHMAP_SCHEMA: &str = "db.pathmap.v1";

/// 🎯️ `DslValue` object pathmap -> `store::pack_rt::encode_wire_value` bytes.
pub async fn encode_pathmap(value: &DslValue) -> Vec<u8> {
    store::pack_rt::encode_wire_value(value)
}

/// 🎯️ Inverse of `encode_pathmap`.
async fn decode_pathmap(bytes: &[u8]) -> Result<DslValue, DbError> {
    store::pack_rt::decode_wire_value(bytes).map_err(wire_err)
}

/// 🧰️ Public convenience for every crate above this one that hand-builds a `DB_PATHMAP_SCHEMA`
/// `MutationEnvelope` (test fixtures, `db_cli`'s `profile`/`migrate` commands, `db_fault_testing`'s
/// workload generators) rather than going through `envelope_from_operation_cold`: encodes a
/// `serde_json::Value::Object` the same way `decode_pathmap`/`apply_one` decode it. Centralizing
/// this is the single source of truth for `DB_PATHMAP_SCHEMA`'s actual wire bytes — a caller that
/// instead hand-rolls `serde_json::to_vec` produces bytes `decode_pathmap` cannot read (it expects
/// `store::pack_rt`'s binary encoding, not raw JSON text), which is exactly the "wire error:
/// truncated" bug this function exists to make structurally impossible to repeat.
pub async fn encode_pathmap_json(value: &serde_json::Value) -> Result<Vec<u8>, DbError> {
    Ok(encode_pathmap(&semio_framework_value::DslValue::from(value)).await)
}

/// 🧰️ Inverse of `encode_pathmap_json` — also the general "read back one stored/queried
/// value's bytes as JSON" decode every caller above this crate needs: `ArtifactEngine::get`/
/// `preview_get` and `db_engine::ArtifactHandle::query` all hand back these same `store::pack_rt`
/// wire bytes (single value OR whole pathmap object, both are just a `DslValue` tree to this
/// codec), never raw JSON text — a caller reaching for `serde_json::from_slice` on them directly
/// hits exactly the "expected value" parse error this function exists to make impossible.
pub async fn decode_pathmap_json(bytes: &[u8]) -> Result<serde_json::Value, DbError> {
    Ok(serde_json::Value::from(decode_pathmap(bytes).await?))
}

/// 🧮️ Flattens a diff/inverse pathmap object into `(path, Some(value) | None)` pairs per this
/// module's generic path-value convention (see module doc). Errors if `value` is not an object —
/// this crate's own schema-erased documents have no other shape it can interpret.
async fn entries_from_value(value: &DslValue) -> Result<Vec<(String, Option<DslValue>)>, DbError> {
    let object = value.as_object().ok_or_else(|| DbError::InvalidArgument("diff/inverse payload must be a pathmap object".to_string()))?;
    Ok(object.iter().map(|(path, entry)| (path.clone(), if entry.is_null() { None } else { Some(entry.clone()) })).collect())
}

/// ➡️ Entries for an envelope's forward diff — empty (not an error) for any schema other
/// than `DB_PATHMAP_SCHEMA`, see its doc.
pub async fn diff_entries(diff: &protocol::ArtifactDiff) -> Result<Vec<(String, Option<DslValue>)>, DbError> {
    if diff.schema.0 != DB_PATHMAP_SCHEMA {
        return Ok(Vec::new());
    }
    entries_from_value(&decode_pathmap(&diff.payload).await?).await
}

/// ↩️ Entries for an envelope's inverse diff (the `undo` pipeline's source) — same
/// foreign-schema handling as `diff_entries`.
pub async fn inverse_entries(inverse: &protocol::InverseMutation) -> Result<Vec<(String, Option<DslValue>)>, DbError> {
    if inverse.schema.0 != DB_PATHMAP_SCHEMA {
        return Ok(Vec::new());
    }
    entries_from_value(&decode_pathmap(&inverse.payload).await?).await
}

/// 🧮️ The inverse of `entries_from_value` — rebuilds a JSON object from path-value pairs,
/// `None` becoming an explicit `null` tombstone. Used by `undo` to construct a compensating
/// envelope's diff/inverse payloads.
// 🚫️async: E1 pure accessor, always used as `&entries_to_value(...)` inline into another call — see R9
pub fn entries_to_value(entries: &[(String, Option<DslValue>)]) -> DslValue {
    semio_framework_value::DslValue::Object(entries.iter().map(|(path, value)| (path.clone(), value.clone().unwrap_or(semio_framework_value::DslValue::Null))).collect())
}

/// 👣️ The `TouchedSet` a set of entries would write — shared by `DocumentState::
/// apply_entries` and preview publishing.
// 🚫️async: E1 pure accessor, `db_state::TouchedSet`'s own methods are sync — see R9
pub fn entries_touched(entries: &[(String, Option<DslValue>)]) -> db_state::TouchedSet {
    let mut touched = db_state::TouchedSet::new();
    for (path, _) in entries {
        touched.record(db_state::TouchedRegion::write(path.clone()));
    }
    touched
}
//#endregion 🔖️Diff

//#region 🔖️Bridge
/// ❄️ Builds a cold reference `MutationEnvelope` in this crate's path-value diff convention
/// from explicit `protocol::ColdMutation<P>` arithmetic and `protocol::Mutation<P>` metadata against a serializable
/// projection `P`. The operation's diff is applied once through [`protocol::apply_diff_cold`]; the envelope then persists only what
/// that diff changed — one `path/field` entry per top-level field of `P` whose value differs (a removed field is a tombstone),
/// the inverse carrying the base value of exactly those fields — never the whole post- or base projection. A projection that is
/// not a record is one entity: its single entry sits at `path`. Uses `ColdMutation::diff` and strict `Mutation`
/// metadata (`mutation_id`/`dependencies`/`author_id`/`timestamp`) — see the module doc's design-choice note on why this crate is
/// allowed to.
pub async fn envelope_from_operation_cold<P, Op>(
    document: protocol::ArtifactId,
    path: &str,
    op: &Op,
    base: &P,
    default_actor: protocol::ActorId,
    default_mutation_id: protocol::MutationId,
    default_timestamp: protocol::HybridLogicalTimestamp,
) -> Result<protocol::MutationEnvelope, DbError>
where
    P: semio_framework_value::ToValue + Clone,
    Op: protocol::Mutation<P> + protocol::ColdMutation<P>,
{
    let diff = <Op as protocol::ColdMutation<P>>::diff(op, base);
    let post = protocol::apply_diff_cold(diff.diff(), base).map_err(|error| DbError::InvalidArgument(error.to_string()))?;
    let (forward, backward) = changed_entries(path, &semio_framework_value::ToValue::to_value(base), &semio_framework_value::ToValue::to_value(&post));
    let schema = protocol::SchemaId(DB_PATHMAP_SCHEMA.to_string());
    Ok(protocol::MutationEnvelope {
        mutation_id: op.mutation_id().unwrap_or(default_mutation_id),
        document_id: document,
        actor: op.author_id().unwrap_or(default_actor),
        dependencies: op.dependencies(),
        observed: None,
        target: op.conflict_target(),
        diff: protocol::ArtifactDiff { schema: schema.clone(), payload: encode_pathmap(&forward).await },
        inverse: protocol::InverseMutation { schema, payload: encode_pathmap(&backward).await },
        timestamp: op.timestamp().unwrap_or(default_timestamp),
        transaction: None, verb: None, line: None,
    })
}

/// 🔺️ The pathmap entries a base→post change writes, as `(forward, inverse)`: per differing top-level field of two record
/// values `path/field` (absent after = `Null` tombstone, absent before = `Null` in the inverse), else the one entity at `path`.
fn changed_entries(path: &str, base: &DslValue, post: &DslValue) -> (DslValue, DslValue) {
    let (DslValue::Object(before), DslValue::Object(after)) = (base, post) else {
        return (DslValue::Object(vec![(path.to_string(), post.clone())]), DslValue::Object(vec![(path.to_string(), base.clone())]));
    };
    let mut forward = Vec::new();
    let mut backward = Vec::new();
    for (key, value) in after {
        let prior = before.iter().find(|(name, _)| name == key).map(|(_, prior)| prior);
        if prior != Some(value) {
            forward.push((format!("{path}/{key}"), value.clone()));
            backward.push((format!("{path}/{key}"), prior.cloned().unwrap_or(DslValue::Null)));
        }
    }
    for (key, prior) in before {
        if !after.iter().any(|(name, _)| name == key) {
            forward.push((format!("{path}/{key}"), DslValue::Null));
            backward.push((format!("{path}/{key}"), prior.clone()));
        }
    }
    (DslValue::Object(forward), DslValue::Object(backward))
}
//#endregion 🔖️Bridge
//#endregion 🔖️Ids
