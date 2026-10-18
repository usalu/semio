//! 🚨️ Sync errors, actor identity and envelope serde: the repository root check, the sync error, the actor identity admission and the envelope serde adapter. See [`super`].

use super::*;

/// 🏠️ Admits only a declared package workspace with regular build authority.
pub fn is_repository_root(path: &std::path::Path) -> Result<bool, String> {
    workspace_identity::is_workspace_root(path, |source| semio_framework_pack_json::parse(source, semio_framework_pack_json::JsonMemberPolicy::Reject).map(|value| matches!(value.get("workspaces"), Some(semio_framework_pack_json::Value::Array(_) | semio_framework_pack_json::Value::Object(_)))).map_err(|error| error.to_string()))
}

//#region 🔖️Errors
#[derive(Debug, PartialEq, Eq)]
pub enum SyncError {
    Vcs(String),
    Actor(String),
}

impl std::fmt::Display for SyncError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Vcs(detail) => write!(formatter, "vcs error: {detail}"),
            Self::Actor(detail) => write!(formatter, "actor error: {detail}"),
        }
    }
}

impl std::error::Error for SyncError {}
//#endregion 🔖️Errors

pub fn actor_identity_admission_admits(grant:semio_framework_value::RetainedCloneGrant)->bool{let demand=semio_framework_value::SharedUtf8::admission_demand();grant.maximum_items>=1&&grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_depth>=demand.depth}

pub(super) fn actor_identity_batch_admits(count:usize,grant:semio_framework_value::RetainedCloneGrant)->bool{let demand=semio_framework_value::SharedUtf8::lease_demand();count<=grant.maximum_items&&count.checked_mul(demand.copy_bytes).is_some_and(|bytes|bytes<=grant.maximum_copy_bytes)&&grant.maximum_depth>=demand.depth}

pub fn add_actor_identity_progress(total:&mut semio_framework_value::RetainedCloneProgress,receipt:semio_framework_value::RetainedCloneProgress){total.copied_items=total.copied_items.checked_add(receipt.copied_items).expect("original identity item conservation overflow");total.copied_bytes=total.copied_bytes.checked_add(receipt.copied_bytes).expect("original identity copy conservation overflow");total.retained_capacity_bytes=total.retained_capacity_bytes.checked_add(receipt.retained_capacity_bytes).expect("original identity capacity conservation overflow");total.released_bytes=total.released_bytes.checked_add(receipt.released_bytes).expect("original identity release conservation overflow");}

pub fn wire_envelopes_with_original_actor(envelopes:&[MutationEnvelope],actor:&semio_framework_value::SharedUtf8,grant:semio_framework_value::RetainedCloneGrant)->Result<(Vec<MutationEnvelope>,semio_framework_value::RetainedCloneProgress),ValueError>{if !actor_identity_batch_admits(envelopes.len(),grant){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"original socket identity batch exceeds caller lease authority"));}let mut total=semio_framework_value::RetainedCloneProgress::default();let mut rows=Vec::with_capacity(envelopes.len());for original in envelopes{let(actor,receipt)=actor.admit_clone(semio_framework_value::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:grant.maximum_copy_bytes-total.copied_bytes,..grant})?;add_actor_identity_progress(&mut total,receipt);rows.push(MutationEnvelope{mutation_id:original.mutation_id.clone(),document_id:original.document_id.clone(),actor:ActorId(actor),dependencies:original.dependencies.clone(),observed:original.observed.clone(),target:original.target.clone(),diff:original.diff.clone(),inverse:original.inverse.clone(),timestamp:original.timestamp,transaction:original.transaction.clone(),verb:original.verb.clone(),line:original.line.clone()});}Ok((rows,total))}

//#region 🔖️EnvelopeSerde
/// 🧵️ JSON worker seam: `MutationEnvelope` vectors as `encode_envelopes` bytes (not struct JSON).
pub(super) mod envelope_serde {
    use semio_framework_os_spr::{decode_envelopes, encode_envelopes, MutationEnvelope};

    /// 🧵️ Byte framing for the `ArtifactActorMsg::LocalMutations`/`ArtifactEvent::
    /// RemoteMutations` `#[value(serialize_with = ..., deserialize_with = ...)]` field bridge —
    /// wire shape is a `DslValue::Array` of `DslValue::Number` (one per byte), matching what the
    /// former serde `serialize_seq` path produced.
    pub fn to_value(envelopes: &[MutationEnvelope]) -> semio_framework_value::DslValue {
        let bytes = encode_envelopes(envelopes);
        semio_framework_value::DslValue::Array(bytes.into_iter().map(|byte| semio_framework_value::DslValue::uint(byte as u64)).collect())
    }

    /// 🧵️ `FromValue` twin of `deserialize` above.
    pub fn from_value(value: semio_framework_value::DslValue) -> Result<Vec<MutationEnvelope>, semio_framework_value::ValueError> {
        let semio_framework_value::DslValue::Array(items) = value else {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected array of bytes"));
        };
        let mut bytes = Vec::with_capacity(items.len());
        for item in items {
            let semio_framework_value::DslValue::Number(n) = item else {
                return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected byte number"));
            };
            let Some(n) = n.as_u64() else {
                return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected byte number"));
            };
            bytes.push(n as u8);
        }
        decode_envelopes(&bytes).map_err(|error| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string()))
    }
}
//#endregion 🔖️EnvelopeSerde
