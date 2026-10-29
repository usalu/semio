//! 🌳️ Checkpoint ancestry, merge base, alternative reconciliation and the by-id merge over an envelope's history ledgers.

use semio_framework_os_store_engine_core::lifted_document_materialize::now_iso;
use semio_framework_os_store_engine_live::lifted_store::transition_retraction_closure;
use semio_framework_os_store_space_schema::ArtifactEnvelope;
use semio_framework_os_vcs::io::binary::checkpoint_identity::content_addressed_checkpoint_id;
use semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority;
use semio_framework_os_vcs::io::binary::entity_identity::{mint_alternative_id, mint_change_id};
use semio_framework_os_vcs::{Alternative, ArtifactHistoryLedger, Author, Change, Checkpoint, VcsError};
use semio_framework_value::{FromValue, ToValue};
use std::collections::{HashMap, HashSet};

/// 🌳️ Walks `checkpoint_id`'s ancestor chain via `parent_id` back to the root, nearest-first
/// (`checkpoint_id` itself is the first entry). Cycle-guarded (a malformed/adversarial parent chain
/// stops instead of looping forever) — every well-formed chain built by `reconcile_alternative`/
/// `CommitCheckpoint` is already acyclic, this is defense in depth, not a documented invariant break.
pub async fn checkpoint_ancestors<P, Mutation>(envelope: &ArtifactEnvelope<P, Mutation>, checkpoint_id: &str) -> Vec<String> {
    let mut chain = Vec::new();
    let mut seen = HashSet::new();
    let mut current = Some(checkpoint_id.to_string());
    while let Some(id) = current {
        if !seen.insert(id.clone()) {
            break;
        }
        let parent = envelope.vcs.checkpoints.iter().find(|checkpoint| checkpoint.id == id).and_then(|checkpoint| checkpoint.parent_id.clone());
        chain.push(id);
        current = parent;
    }
    chain
}

/// 🌳️ The merge-base of checkpoints `a` and `b`: the nearest checkpoint common to both
/// ancestor chains (via `parent_id`), or `None` if their histories share no common ancestor.
/// Supports branch-merge tooling that needs to know "everything since the fork point" on either
/// side. `b`'s chain is walked nearest-to-farthest so the FIRST hit in `a`'s ancestor set is the
/// nearest (not merely *a*) common ancestor.
pub async fn merge_base<P, Mutation>(envelope: &ArtifactEnvelope<P, Mutation>, a: &str, b: &str) -> Option<String> {
    let ancestors_a: HashSet<String> = checkpoint_ancestors(envelope, a).await.into_iter().collect();
    checkpoint_ancestors(envelope, b).await.into_iter().find(|id| ancestors_a.contains(id))
}

/// 🌀️ `alternative_id` is both cloned and returned below — a future can only be awaited once
/// (R10 shape 2), so it is resolved to a plain `String` here.
///
/// 🌀️ Same reasoning as `alternative_id` above — resolved once, before it is borrowed AND moved.
///
/// 🎯️ `&[]`: reconcile-alternative checkpoints carry no composition pins yet — the
/// `CompositionCoordinator` that populates real `CompositionPin`s on commit is a later wave.
pub async fn reconcile_alternative<P, Mutation>(envelope: &mut ArtifactEnvelope<P, Mutation>, alternative_name: &str, checkpoint_message: Option<String>, authors: Vec<Author>, identity: &mut EntityIdentityAuthority<'_>) -> Result<String, VcsError>
where
    P:  ToValue ,
    Mutation: Clone + ToValue + FromValue,
{
    if envelope.vcs.checkpoints.is_empty() {
        return Err(VcsError::NoCheckpoint);
    }
    let checkpoint_id = envelope.vcs.checkpoints.last().map(|checkpoint| checkpoint.id.clone()).ok_or(VcsError::NoCheckpoint)?;
    let alternative_id = identity.encode(|control|mint_alternative_id(alternative_name, std::slice::from_ref(&checkpoint_id), control))?;
    envelope.vcs.alternatives.try_push(Alternative { id: alternative_id.clone(), name: alternative_name.to_string(), checkpoint_ids: vec![checkpoint_id] }).map_err(|_| VcsError::ValidationFailed("alternative history ledger is saturated".into()))?;
    if let Some(message) = checkpoint_message {
        let change = Change { id: identity.encode(|control|mint_change_id(&[], Some(&message), control))?, edit_ids: Vec::new(), description: Some(message), saved_at: now_iso() };
        let parent = envelope.vcs.checkpoints.last();
        let parent_id = parent.map(|checkpoint| checkpoint.id.clone());
        let mut change_ids = parent.map(|checkpoint| checkpoint.change_ids.clone()).unwrap_or_default();
        change_ids.push(change.id.clone());
        envelope.vcs.changes.try_push(change).map_err(|_| VcsError::ValidationFailed("change history ledger is saturated".into()))?;
        let timestamp = now_iso();
        let checkpoint_message = Some("reconciled".to_string());
        let id = content_addressed_checkpoint_id(parent_id.as_deref(), &change_ids, &envelope.vcs.changes, checkpoint_message.as_deref(), &authors, &timestamp, &[]).await;
        envelope
            .vcs
            .checkpoints
            .try_push(Checkpoint { id, change_ids, parent_id, authors, message: checkpoint_message, timestamp, composition_pins: Vec::new() })
            .map_err(|_| VcsError::ValidationFailed("checkpoint history ledger is saturated".into()))?;
    }
    Ok(alternative_id)
}

pub async fn checkpoint_identity(checkpoint: &Checkpoint, changes: &ArtifactHistoryLedger<Change>) -> String {
    content_addressed_checkpoint_id(checkpoint.parent_id.as_deref(), &checkpoint.change_ids, changes, checkpoint.message.as_deref(), &checkpoint.authors, &checkpoint.timestamp, &checkpoint.composition_pins).await
}

pub async fn preflight_merge_by_id<T: PartialEq>(local: &[T], remote: &[T], id_of: impl Fn(&T) -> &String) -> Result<(), VcsError> {
    let existing: HashMap<&str, &T> = local.iter().map(|item| (id_of(item).as_str(), item)).collect();
    for item in remote {
        let id = id_of(item).clone();
        match existing.get(id.as_str()) {
            Some(established) if *established == item => {}
            Some(_) => return Err(VcsError::ValidationFailed(format!("remote history conflicts with established record {id}"))),
            None => {}
        }
    }
    Ok(())
}

pub async fn merge_by_id<T: Clone + PartialEq>(local: &mut Vec<T>, remote: Vec<T>, id_of: impl Fn(&T) -> &String) -> Result<(), VcsError> {
    preflight_merge_by_id(local, &remote, &id_of).await?;
    let mut existing: HashSet<String> = local.iter().map(|item| id_of(item).clone()).collect();
    for item in remote {
        if existing.insert(id_of(&item).clone()) {
            local.push(item);
        }
    }
    Ok(())
}

/// 🪒️ The `.spr` byte log without the transitions `refused` retracts ([`transition_retraction_closure`]) — what a sync
/// actor persists after the hub refused them — and the retracted ids.
pub async fn retract_history_transitions_from_spr(spr: &[u8], refused: &[String]) -> Result<(Vec<u8>, Vec<String>), VcsError> {
    let mut log = semio_framework_os_spr::decode_history(spr, &semio_framework_os_spr::DecodeOptions::default()).await.map_err(|error| VcsError::Deserialize(error.to_string()))?;
    let retracted = transition_retraction_closure(&log.transitions, |record| record.id.as_str(), |record| record.dependencies.iter().map(String::as_str).collect(), refused);
    log.transitions.retain(|record| !retracted.contains(&record.id));
    let options = semio_framework_os_spr::EncodeOptions { write_backwards_section: true, ..semio_framework_os_spr::EncodeOptions::default() };
    let bytes = semio_framework_os_spr::encode_history(&log, &options).await.map_err(|error| VcsError::Serialize(error.to_string()))?;
    Ok((bytes, retracted))
}
