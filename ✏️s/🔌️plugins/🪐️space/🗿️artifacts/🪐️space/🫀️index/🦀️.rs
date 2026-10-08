//! 🪐️ Space artifact index decoding and workflow projection.
use crate::{SSpaceSnapshot, SSpaceMutation, S_SPACE_INDEX_DOCUMENT_SCHEMA};
use semio_s_space_core::resolve_backbone_bytes;
use semio_framework_artifact_space_collection::{artifact_backbone_uri, collection_backbone_uri, ArtifactBody, CollectionEntry, CollectionMutation, CollectionSnapshot, S_COLLECTION_SCHEMA};
use semio_framework_os::{decode_backbone_payload, materialize_backbone_snapshot, OsSpaceDocument, OsWorkflowArtifactDocument, WorkflowMutation, WorkflowSnapshot, S_WORKFLOW_SCHEMA};

/// 🪆️ Reads a space's `s.space` artifact index (document id `index`, contract §C4) and decodes it —
/// `None` when no index document has been written yet (older spaces / test fixtures seeded before this
/// ticket), which is exactly the case `resolve_workflow_artifact_document` falls back on below.
async fn resolve_space_index_snapshot(space_id: &str) -> Option<SSpaceSnapshot> {
    let index_uri = artifact_backbone_uri(space_id, "index");
    let payload = resolve_backbone_bytes(&index_uri).await?;
    let index_document = decode_backbone_payload::<SSpaceSnapshot, SSpaceMutation>(&payload, S_SPACE_INDEX_DOCUMENT_SCHEMA).ok()?;
    materialize_backbone_snapshot(&index_document, &index_document.applied_edit_ids().ok()?).ok()
}

/// 🕸️ "Space session -> active workflow artifact" resolution. Index-first (contract §C4: the space's
/// own `s.space` artifact index is the single source of truth for which artifacts live in a space) —
/// projects the index onto the framework's `os.collection` shape via `project_space_index_to_collection`
/// and walks ITS entries first. Falls back to the legacy direct `projection.collections` walk only when
/// no index document exists yet, so existing `⚙️engine` fixtures that seed a collection directly (never
/// an index) keep resolving exactly as before — never a silent behavior loss.
pub async fn resolve_workflow_artifact_document(space_id: &str, space_document: &OsSpaceDocument) -> Option<OsWorkflowArtifactDocument> {
    if let Some(index_snapshot) = resolve_space_index_snapshot(space_id).await {
        let collection_projection = project_space_index_to_collection(&index_snapshot).await;
        if let Some(workflow_snapshot) = find_workflow_snapshot_in_collection(space_id, &collection_projection).await {
            return Some(workflow_snapshot);
        }
    }
    let projection = materialize_backbone_snapshot(space_document, &space_document.applied_edit_ids().ok()?).ok()?;
    for collection_ref in &projection.collections {
        let collection_uri = collection_backbone_uri(space_id, &collection_ref.id);
        let Some(collection_payload) = resolve_backbone_bytes(&collection_uri).await else { continue };
        let Ok(collection_document) = decode_backbone_payload::<CollectionSnapshot, CollectionMutation>(&collection_payload, S_COLLECTION_SCHEMA) else { continue };
        let Ok(applied_edit_ids) = collection_document.applied_edit_ids() else { continue };
        let Ok(collection_projection) = materialize_backbone_snapshot(&collection_document, &applied_edit_ids) else { continue };
        if let Some(workflow_snapshot) = find_workflow_snapshot_in_collection(space_id, &collection_projection).await {
            return Some(workflow_snapshot);
        }
    }
    None
}

/// 🔎️ Shared entry-walk: the first `s.workflow`-schema'd entry whose backbone bytes decode cleanly.
async fn find_workflow_snapshot_in_collection(space_id: &str, collection_projection: &CollectionSnapshot) -> Option<OsWorkflowArtifactDocument> {
    for entry in &collection_projection.entries {
        let ArtifactBody::Document { schema, document_id } = entry.body.as_ref() else { continue };
        if schema != S_WORKFLOW_SCHEMA {
            continue;
        }
        let artifact_uri = artifact_backbone_uri(space_id, document_id);
        let Some(artifact_payload) = resolve_backbone_bytes(&artifact_uri).await else { continue };
        if let Ok(workflow_snapshot) = decode_backbone_payload::<WorkflowSnapshot, WorkflowMutation>(&artifact_payload, S_WORKFLOW_SCHEMA) {
            return Some(workflow_snapshot);
        }
    }
    None
}

//#region 🔖️SpaceIndexProjection
/// 🪞️ Projects the space's `s.space` artifact index onto the framework's `os.collection` shape — the
/// SAME `CollectionSnapshot` type `resolve_workflow_artifact_document`'s legacy walk already understood,
/// so the index becomes a drop-in single source of truth without widening either reader's contract.
/// Ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS §C4.
pub async fn project_space_index_to_collection(index: &SSpaceSnapshot) -> CollectionSnapshot {
    let entries = index
        .artifacts
        .iter()
        .map(|row| CollectionEntry { id: row.id.clone(), folder_id: None, name: row.name.clone(), kind_id: row.kind_id.clone(), body: Box::new(ArtifactBody::Document { schema: row.schema.clone(), document_id: row.id.clone() }) })
        .collect();
    CollectionSnapshot { schema: S_COLLECTION_SCHEMA.into(), name: index.space_id.clone(), folders: Vec::new(), entries }
}
//#endregion 🔖️SpaceIndexProjection
