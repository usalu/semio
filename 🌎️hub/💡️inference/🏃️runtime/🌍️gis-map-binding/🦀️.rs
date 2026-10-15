//! 🌍️ The GIS Map domain binding: GIS, drawing and value store types, the commit identity, the native GIS inference and the preview decoder. These items name the GIS crates inside the OS-level hub (wrong direction); moving this child into a hub-side GIS plugin is the follow-up.

use super::*;

pub(super) type GisMapParentSnapshotV1 = semio_s_artifact_gis_gismap::GisMapSnapshot;
pub(super) type GisMapParentMutationV1 = semio_s_artifact_gis_gismap::mutations::GisMapMutation;
pub(super) type GisMapDrawingSnapshotV1 = semio_s_artifact_stdio_semio_drawing::schema::snapshot::SemioDrawingSnapshot;
pub(super) type GisMapDrawingMutationV1 = semio_s_artifact_stdio_semio_drawing::schema::mutations::SemioDrawingMutation;
pub(super) type GisMapValueSnapshotV1 = semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValueSnapshot;
pub(super) type GisMapValueMutationV1 = semio_s_artifact_stdio_semio_value_mutations::schema::mutations::SemioValueMutation;
pub(super) type GisMapParentStoreV1 = directory::os_store::ArtifactStore<GisMapParentSnapshotV1, GisMapParentMutationV1>;
pub(super) type GisMapDrawingStoreV1 = directory::os_store::ArtifactStore<GisMapDrawingSnapshotV1, GisMapDrawingMutationV1>;
pub(super) type GisMapValueStoreV1 = directory::os_store::ArtifactStore<GisMapValueSnapshotV1, GisMapValueMutationV1>;
pub(super) type GisMapAssemblyV1 = directory::os_store::durable_group::DurableOwnedThreeStoreMapAssemblyV1<GisMapParentSnapshotV1, GisMapParentMutationV1, GisMapDrawingSnapshotV1, GisMapDrawingMutationV1, GisMapValueSnapshotV1, GisMapValueMutationV1>;
pub(super) type GisMapCommitHostV1 = directory::os_store::durable_group::DurableOwnedMapCommitHostV1<GisMapParentSnapshotV1, GisMapParentMutationV1, GisMapDrawingSnapshotV1, GisMapDrawingMutationV1, GisMapValueSnapshotV1, GisMapValueMutationV1>;
pub(super) type GisMapRecoveryOwnerV1 = db::document::ArtifactDurableGroupRecoveryOwnerV1<GisMapParentSnapshotV1, GisMapParentMutationV1, GisMapDrawingSnapshotV1, GisMapDrawingMutationV1, GisMapValueSnapshotV1, GisMapValueMutationV1>;
pub(super) type GisMapJournalReceiptV1 = directory::os_store::durable_group::DurableOwnedGroupJournalReceiptV1;

pub(super) struct GisMapDocumentStoresV1 {
    pub(super) parent: Option<GisMapParentStoreV1>,
    pub(super) drawing: Option<GisMapDrawingStoreV1>,
    pub(super) value: Option<GisMapValueStoreV1>,
    pub(super) handle: db::ArtifactHandle,
    pub(super) scope: DocumentScope,
    pub(super) generation: u64,
    pub(super) document_write: Arc<tokio::sync::Mutex<()>>,
    pub(super) fence: Arc<InferenceDocumentFenceV1>,
}

#[derive(Clone)]
pub(super) struct GisMapCommitIdentityV1 {
    pub(super) scope: DocumentScope,
    pub(super) actor: String,
    pub(super) mutation_id: String,
    pub(super) command_hash: String,
    pub(super) job_id: String,
    pub(super) proposal_hash: String,
    pub(super) base_frontier: ArtifactFrontier,
    pub(super) descriptor_digest: String,
    pub(super) base_digest: String,
    pub(super) timestamp: protocol::HybridLogicalTimestamp,
    pub(super) journal_now_ms: u64,
    pub(super) document_write: Arc<tokio::sync::Mutex<()>>,
    pub(super) ingress: Option<Arc<dyn GisMapApprovalIngressAuthorityV1>>,
    pub(super) operation: GisMapCommitOperationV1,
}

#[derive(Clone)]
pub(super) enum GisMapCommitOperationV1 {
    Approval,
    Undo { target_id: String, idempotency_key: String, original_job_id: String, original_command: Arc<InferencePrivateBytesV1> },
}

pub(super) struct GisMapClosingStoresV1 {
    pub(super) owners: GisMapDocumentStoresV1,
    pub(super) phase: u8,
    pub(super) document_write: Option<GisMapDocumentWriteLeaseV1>,
}

/// 🧮️ Runs the frozen native GIS executable once and decodes exactly its own canonical result.
pub(super) fn deterministic_map_inference(base: &InferenceMapBaseV1, job_id: &str) -> Result<(semio_s_artifact_gis_gismap::GisMapSnapshot, semio_s_artifact_gis_gismap::standards::v1::subsets::any::schema::inferences::GisMapInference), InferenceRouteErrorV1> {
    use directory::FromValue;
    use semio_s_artifact_gis_gismap::standards::v1::subsets::any::schema::inferences::GisMapInference;
    let budgets = semio_framework_plugin::WireArtifactInferenceBudget { allocation_bytes: ALLOCATION_BYTES, work_units: WORK_UNIT_LIMIT, recursion_depth: RECURSION_DEPTH };
    let execution = semio_s_artifact_gis_gismap::infer_gis_map_controlled(
        &semio_framework_plugin::ArtifactInferenceExecutionRequest {
            policy: b"gis-map-v1",
            budgets: &budgets,
            cancellation_id: job_id,
            previous_state: None,
            requested_cache_mode: semio_framework_plugin::WireArtifactInferenceCacheMode::Cold,
            canonical_payload: base.pack.as_slice(),
            dependencies: &[],
        },
        &mut |_| Ok(()),
    )
    .map_err(|_| InferenceRouteErrorV1::Invalid)?;
    let snapshot = <semio_s_artifact_gis_gismap::GisMapSnapshot as pack::snapshot::ArtifactPack>::decode_pack(base.pack.as_slice()).map_err(|_| InferenceRouteErrorV1::Invalid)?;
    let inference = GisMapInference::from_value(directory::pack_rt::decode_wire_value(&execution.canonical_payload).map_err(|_| InferenceRouteErrorV1::Invalid)?).map_err(|_| InferenceRouteErrorV1::Invalid)?;
    Ok((snapshot, inference))
}

//#region 🛣️Routes
pub(super) fn gis_map_inference_preview(job_id: &str, proposal_hash: &str, proposal: &[u8]) -> Result<GisMapInferencePreviewV1, InferenceRouteErrorV1> {
    use semio_s_artifact_gis_gismap::mutations::GisMapMutation;

    if sha256(proposal) != proposal_hash {
        return Err(InferenceRouteErrorV1::Conflict);
    }
    let text = std::str::from_utf8(proposal).map_err(|_| InferenceRouteErrorV1::Invalid)?;
    let mutation = semio_framework_pack_json::from_json_str::<GisMapMutation>(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|_| InferenceRouteErrorV1::Invalid)?;
    let GisMapMutation::CreateRegion(created) = mutation else {
        return Err(InferenceRouteErrorV1::Conflict);
    };
    let region_id = format!("inference-{job_id}");
    let entries = created.item.data.as_object().ok_or(InferenceRouteErrorV1::Invalid)?;
    if created.item.id != region_id || entries.len() != 3 || created.item.data.get("id").and_then(|value| value.as_str()) != Some(region_id.as_str()) || created.item.data.get("kind").and_then(|value| value.as_str()) != Some("inference-bounds") {
        return Err(InferenceRouteErrorV1::Conflict);
    }
    let points = created.item.data.get("ring").and_then(|value| value.as_array()).ok_or(InferenceRouteErrorV1::Invalid)?;
    if points.len() != 5 {
        return Err(InferenceRouteErrorV1::Bounds);
    }
    let mut ring = [[0.0_f64; 2]; 5];
    for (index, point) in points.iter().enumerate() {
        let coordinates = point.as_array().ok_or(InferenceRouteErrorV1::Invalid)?;
        if coordinates.len() != 2 {
            return Err(InferenceRouteErrorV1::Bounds);
        }
        ring[index] = [coordinates[0].as_f64().ok_or(InferenceRouteErrorV1::Invalid)?, coordinates[1].as_f64().ok_or(InferenceRouteErrorV1::Invalid)?];
    }
    let [lon_min, lat_min] = ring[0];
    let [lon_max, lat_max] = ring[2];
    if !ring.iter().flatten().all(|value| value.is_finite())
        || !(-180.0..=180.0).contains(&lon_min)
        || !(-180.0..=180.0).contains(&lon_max)
        || !(-90.0..=90.0).contains(&lat_min)
        || !(-90.0..=90.0).contains(&lat_max)
        || lon_min > lon_max
        || lat_min > lat_max
        || ring != [[lon_min, lat_min], [lon_max, lat_min], [lon_max, lat_max], [lon_min, lat_max], [lon_min, lat_min]]
    {
        return Err(InferenceRouteErrorV1::Conflict);
    }
    Ok(GisMapInferencePreviewV1 { schema: "semio.hub.gis-map-inference-preview/v1", job_id: job_id.to_owned(), proposal_hash: proposal_hash.to_owned(), region_id, ring })
}
//#endregion 🛣️Routes
