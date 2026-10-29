//! 🧵️ Backbone worker wire and history envelopes: the backbone worker wire, the operation ids, the history edit and envelope conversions and the hub socket url. See [`super`].

use super::*;

//#region 🔖️BackboneWorkerWire
/// 🧵️ Binary worker seam: `MAGIC` + `crate::os_pack::pack_rt::encode_wire_value` over a `DslValue`
/// tree (serde-shaped), shared by the wasm `store_worker` and `🏪️store/👷️worker/🟦️.ts`.
pub mod backbone_worker_wire {
    use crate::{ArtifactActorConfig, ArtifactActorMsg, ArtifactEvent, PersistenceBinding};
    
    use semio_framework_value_derive::{FromValue, ToValue};

    pub const MAGIC: u8 = 0x01;

    #[derive(Clone, Debug, ToValue, FromValue)]
    #[value(tag = "kind", rename_all = "camelCase")]
    pub enum BackboneWorkerRequest {
        Open {
            document_id: String,
            #[value(default, skip_serializing_if = "Option::is_none")]
            client_instance_id: Option<String>,
            schema: String,
            bindings: Vec<PersistenceBinding>,
            #[value(default, skip_serializing_if = "Option::is_none")]
            watch_external: Option<bool>,
            actor: String,
            actor_identity_grant: semio_framework_value::RetainedCloneGrant,
        },
        Close {
            document_id: String,
            #[value(default, skip_serializing_if = "Option::is_none")]
            client_instance_id: Option<String>,
        },
        Send {
            document_id: String,
            #[value(default, skip_serializing_if = "Option::is_none")]
            client_instance_id: Option<String>,
            message: Box<ArtifactActorMsg>,
        },
    }

    #[derive(Clone, Debug, ToValue, FromValue)]
    #[value(tag = "kind", rename_all = "camelCase")]
    pub enum BackboneWorkerResponse {
        Event { document_id: String, client_instance_id: String, event: ArtifactEvent },
        Ready,
    }

    impl BackboneWorkerRequest {
        pub async fn actor_config(&self) -> Option<ArtifactActorConfig> {
            let Self::Open { document_id, schema, bindings, watch_external, actor, actor_identity_grant, .. } = self else {
                return None;
            };
            Some(ArtifactActorConfig { document_id: document_id.clone(), schema: schema.clone(), bindings: bindings.clone(), watch_external: watch_external.unwrap_or(true), actor: actor.clone(), actor_identity_grant: *actor_identity_grant })
        }
    }

    pub fn encode_request(request: &BackboneWorkerRequest) -> Vec<u8> {
        let dsl = semio_framework_value::ToValue::to_value(request);
        let mut bytes = vec![MAGIC];
        bytes.extend(semio_framework_os_pack::pack_rt::encode_wire_value(&dsl));
        bytes
    }

    pub async fn decode_request(bytes: &[u8]) -> Result<BackboneWorkerRequest, String> {
        let (magic, payload) = bytes.split_first().ok_or_else(|| "backbone worker wire: empty".to_string())?;
        if *magic != MAGIC {
            return Err(format!("backbone worker wire: unknown magic {magic}"));
        }
        let dsl = semio_framework_os_pack::pack_rt::decode_wire_value(payload).map_err(|error| error.to_string())?;
        semio_framework_value::FromValue::from_value(dsl).map_err(|error| error.to_string())
    }

    pub fn encode_response(response: &BackboneWorkerResponse) -> Vec<u8> {
        let dsl = semio_framework_value::ToValue::to_value(response);
        let mut bytes = vec![MAGIC];
        bytes.extend(semio_framework_os_pack::pack_rt::encode_wire_value(&dsl));
        bytes
    }

    pub async fn decode_response(bytes: &[u8]) -> Result<BackboneWorkerResponse, String> {
        let (magic, payload) = bytes.split_first().ok_or_else(|| "backbone worker wire: empty".to_string())?;
        if *magic != MAGIC {
            return Err(format!("backbone worker wire: unknown magic {magic}"));
        }
        let dsl = semio_framework_os_pack::pack_rt::decode_wire_value(payload).map_err(|error| error.to_string())?;
        semio_framework_value::FromValue::from_value(dsl).map_err(|error| error.to_string())
    }
}
//#endregion 🔖️BackboneWorkerWire

//#region 🔖️Endpoints
/// 🆔️ One `HistoryEdit`'s op ids, matching `crate::os_spr::mutation_envelope_from_edit`'s own
/// fallback convention (`meta[i].op_id` when present, else `"{edit.id}#{i}"`) so this is the SAME
/// id a live-dispatched envelope for this edit already carries. This is the ONE id domain the
/// actor's dedup set uses — computed here and ONLY here, so a locally-flushed edit's spr entry
/// and the envelope built from re-reading that same spr always agree, fixing the actor's old
/// self-re-ingest bug (mixing envelope op ids with raw JSON edit ids) by construction.
async fn op_ids_of(edit: &semio_framework_os_spr::HistoryEdit) -> Vec<String> {
    match &edit.meta {
        Some(metas) if metas.len() == edit.ops.len() => metas.iter().enumerate().map(|(index, meta)| meta.op_id.clone().unwrap_or_else(|| format!("{}#{index}", edit.id))).collect(),
        _ => (0..edit.ops.len()).map(|index| format!("{}#{index}", edit.id)).collect(),
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn document_archive_hash(archive: &[u8]) -> String {
    semio_framework_hash::hash_bytes(archive)
}

/// 🆔️ Every event id in an spr byte log — each edit's op ids and each transition's id — the
/// actor's dedup/known-ids set, read directly off the binary history (NEVER via
/// `parse_document_spr`, whose meta-absent branch mints fresh random ids on every read).
#[cfg(not(target_arch = "wasm32"))]
pub async fn spr_op_ids(spr: &[u8]) -> Result<std::collections::HashSet<String>, String> {
    let log = semio_framework_os_spr::decode_history(spr, &semio_framework_os_spr::DecodeOptions::default()).await.map_err(|error| error.to_string())?;
    let mut ids = std::collections::HashSet::new();
    for edit in &log.edits {
        ids.extend(op_ids_of(edit).await);
    }
    ids.extend(log.transitions.iter().map(|transition| transition.id.clone()));
    Ok(ids)
}

/// 📜️ Every event an spr byte log persists, as the causal envelopes a store ingests: each
/// edit's operations in log order, then every history transition — a persisted document IS its
/// genesis pack plus exactly these events.
pub async fn spr_events(spr: &[u8], document_id: &str, schema: &str, actor_identity_grant: semio_framework_value::RetainedCloneGrant) -> Result<Vec<MutationEnvelope>, String> {
    let log = semio_framework_os_spr::decode_history(spr, &semio_framework_os_spr::DecodeOptions::default()).await.map_err(|error| error.to_string())?;
    let mut events = Vec::new();
    for edit in &log.edits {
        events.extend(envelopes_from_history_edit(edit, document_id, schema, actor_identity_grant).await?);
    }
    events.extend(log.transitions.iter().map(|transition| transition.to_envelope(document_id)));
    Ok(events)
}

/// 📦️ Rebuilds real {@link MutationEnvelope}s (one per forward op, genuine `OpBinary`
/// payloads straight from the edit's own binary `OpPayload`s — no codec, no JSON) from one
/// `HistoryEdit` decoded off the spr bytes, so an appended external edit can flow through the
/// store's causal DAG (`ingest_remote` → `edit_from_operation_envelope_cold`). A binary-less op payload
/// is a hard error — `.spr` is binary-only since B1, so every real op has one.
pub async fn envelopes_from_history_edit(edit: &semio_framework_os_spr::HistoryEdit, document_id: &str, schema: &str, actor_identity_grant: semio_framework_value::RetainedCloneGrant) -> Result<Vec<MutationEnvelope>, String> {
    if !actor_identity_batch_admits(edit.ops.len(),actor_identity_grant){return Err("original history identity batch exceeds caller lease authority".into());}
    for index in 0..edit.ops.len(){if edit.meta.as_ref().and_then(|metas|metas.get(index)).and_then(|meta|meta.author_id.as_ref()).or(edit.actor.as_ref()).is_none(){return Err(format!("edit {} op {index} has no authentic actor identity",edit.id));}}
    let op_ids = op_ids_of(edit).await;
    let mut identity_copied=0;
    let mut envelopes = Vec::with_capacity(edit.ops.len());
    for (index, op) in edit.ops.iter().enumerate() {
        let payload = op.binary.as_deref().ok_or_else(|| format!("edit {} op {index} has no binary payload", edit.id)).and_then(|bytes| semio_framework_os_spr::CausalPayload::try_from_slice(bytes).map_err(ValueError::into_message))?;
        let meta = edit.meta.as_ref().and_then(|metas| metas.get(index));
        let dependencies = meta.map(|m| m.dependencies.iter().cloned().map(MutationId).collect()).unwrap_or_default();
        let original_actor=meta.and_then(|meta|meta.author_id.as_ref()).or(edit.actor.as_ref()).expect("original actor presence validated before envelope birth");
        let(actor,receipt)=original_actor.admit_clone(semio_framework_value::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:actor_identity_grant.maximum_copy_bytes-identity_copied,..actor_identity_grant}).map_err(ValueError::into_message)?;identity_copied+=receipt.copied_bytes;
        let timestamp = match meta.and_then(|meta| meta.hlt) {
            Some((actor, physical_ms, logical)) => semio_framework_os_spr::HybridLogicalTimestamp { actor, physical_ms: u64::try_from(physical_ms).map_err(|_| format!("edit {} op {index} has a negative hybrid-clock physical time", edit.id))?, logical },
            None => semio_framework_os_spr::HybridLogicalTimestamp::new(0, 0),
        };
        let inverse_payload = semio_framework_os_spr::CausalPayload::try_from_slice(edit.inverse.get(index).and_then(|p| p.binary.as_deref()).unwrap_or_default()).map_err(ValueError::into_message)?;
        envelopes.push(MutationEnvelope {
            mutation_id: MutationId(op_ids[index].clone()),
            document_id: ArtifactId(document_id.to_string()),
            actor: ActorId(actor),
            dependencies,
            observed: None,
            target: Vec::new(),
            diff: semio_framework_os_spr::ArtifactDiff { schema: semio_framework_os_spr::SchemaId(schema.to_string()), payload },
            inverse: semio_framework_os_spr::InverseMutation { schema: semio_framework_os_spr::SchemaId(schema.to_string()), payload: inverse_payload },
            timestamp,
            transaction: None, verb: edit.verb.clone(), line: edit.line.clone(),
        });
    }
    Ok(envelopes)
}

/// 📦️ The inverse of {@link envelopes_from_history_edit}: one `MutationEnvelope` -> one
/// `HistoryEdit` with a real binary `OpPayload` and populated meta (`op_id` == the envelope's own
/// id, so a later `op_ids_of` re-read agrees) — the byte-level twin of
/// `crate::os_store::ledger::edit_from_operation_envelope_cold`, for appending a locally-flushed envelope to the spr log.
///
/// 🎞️ `crate::os_spr::causal::MutationEnvelope` (this fn's input) carries no group_id —
/// same precedent as its already-absent `semantic_kind`/`label`/`undo_policy`, see
/// `command::MutationMeta.group_id`'s doc comment. A remote-ingested edit is therefore
/// never a recognized composite-gesture member; group undo degrades it to a foreign/
/// solitary edit, matching how this whole envelope is already `undo_policy: 0`-flattened.
///
/// 🔀️ A remote-ingested envelope carries no provenance of its own: the authoring peer
/// already resolved any contribution or transaction locally and shipped the resulting
/// OWNER ops, so this side records exactly what it receives — an owner edit.
#[cfg(not(target_arch = "wasm32"))]
pub async fn history_edit_from_envelope(envelope: &MutationEnvelope) -> semio_framework_os_spr::HistoryEdit {
    semio_framework_os_spr::HistoryEdit {
        id: envelope.mutation_id.0.clone(),
        actor: Some(envelope.actor.0.clone()),
        started_at: now_ms().await.to_string(),
        finished_at: None, verb: envelope.verb.clone(), line: envelope.line.clone(),
        ops: vec![semio_framework_os_spr::OpPayload { text: None, binary: Some(envelope.diff.payload.to_vec_owner()) }],
        inverse: if envelope.inverse.payload.is_empty() { Vec::new() } else { vec![semio_framework_os_spr::OpPayload { text: None, binary: Some(envelope.inverse.payload.to_vec_owner()) }] },
        meta: Some(vec![semio_framework_os_spr::HistoryOpMeta {
            op_id: Some(envelope.mutation_id.0.clone()),
            dependencies: envelope.dependencies.iter().map(|dependency| dependency.0.clone()).collect(),
            base_version: 0,
            author_id: Some(envelope.actor.0.clone()),
            hlt: Some((envelope.timestamp.actor, envelope.timestamp.physical_ms as i64, envelope.timestamp.logical)),
            undo_policy: 0,
            payload_hash: None,
            group_id: None,
            origin: semio_framework_os_spr::command::MutationOrigin::Owner,
            messages: Vec::new(),
            transaction: envelope.transaction.clone(),
        }]),
        lane: None,
    }
}

/// 🔗️ Derives a semio_hub WebSocket URL: `remote://host:port` (or `http(s)://`, `ws(s)://`) →
/// `ws(s)://host:port/scopes/{space_id}%2F{document_id}/document/ws`, with an out-of-band
/// `?surface=` appended when the binding carries one (contract §C0's presence scope, ticket
/// 26/08/16/HUB-SPACES-…: `(space_id, document_id, surface)` — `surface` rides outside the wire
/// protocol rather than widening `PresencePeer`'s already-full flag byte). Actor identity is never
/// a query parameter — the server binds it from the session credential.
pub async fn hub_ws_url(base_url: &str, space_id: &str, document_id: &str, surface: Option<&str>) -> String {
    let secure = base_url.starts_with("https://") || base_url.starts_with("wss://");
    let authority = base_url.split_once("://").map(|(_, rest)| rest).unwrap_or(base_url).split('/').next().unwrap_or(base_url);
    let scheme = if secure { "wss" } else { "ws" };
    let scope = semio_framework_os_directory_client::encode_url_component(&format!("{space_id}/{document_id}"));
    match surface {
        Some(surface) => format!("{scheme}://{authority}/scopes/{scope}/document/ws?surface={}", semio_framework_os_directory_client::encode_url_component(surface)),
        None => format!("{scheme}://{authority}/scopes/{scope}/document/ws"),
    }
}
//#endregion 🔖️Endpoints
