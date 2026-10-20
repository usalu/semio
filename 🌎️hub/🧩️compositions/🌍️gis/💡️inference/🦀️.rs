//! 💡️ The GIS Map side of the hub inference binding: the native GIS Map inference, the proposal preview decoder, the fixed-three member Stores and their exact work.
//!
//! [`GisMapInferenceBindingV1`] implements both neutral ports of `semio-hub-inference-binding`; the
//! hub bootstrap registers it and the hub inference runtime never names a GIS crate. See
//! `🪓️monolith-split/RESULT-RELOCATE-HUB-GIS-BINDING.md`.

use std::sync::Arc;

use pack::snapshot::ArtifactPack;
use semio_framework_os_kernel::os_spr::HybridLogicalTimestamp;
use semio_framework_os_kernel::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory;
use semio_framework_os_kernel::os_store::ArtifactStore;
use semio_framework_plugin::{ArtifactInferenceExecutionError, ArtifactInferenceExecutionRequest, ArtifactInferenceService, WireArtifactInferenceBudget, WireArtifactInferenceCacheMode};
use semio_hub_inference_binding::{ComposedMapCommitBindingV1, ComposedMapMemberStoresV1, ComposedMapUndoWorkV1, ComposedMapWorkV1, MapBindingRefusalV1, MapInferenceProposalBindingV1, MapInferenceRunV1, MapProposalEffectV1, MapRegionPreviewV1, MapUndoEffectV1};
use semio_s_artifact_gis_gismap::mutations::GisMapMutation;
use semio_s_artifact_gis_gismap::standards::v1::subsets::any::schema::inferences::{GisMapCreateRegionGroupWorkV1, GisMapInference};
use semio_s_artifact_gis_gismap::GisMapSnapshot;
use semio_s_artifact_stdio_semio_drawing::schema::mutations::SemioDrawingMutation;
use semio_s_artifact_stdio_semio_drawing::schema::snapshot::SemioDrawingSnapshot;
use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValueSnapshot;
use semio_s_artifact_stdio_semio_value_mutations::schema::mutations::SemioValueMutation;

/// ↩️ The owner-authored GIS Map approval undo wire contracts the hub inference scope publishes.
pub use semio_s_artifact_gis_gismap::inference_schema::{GisMapApprovalUndoHandleV1, GisMapApprovalUndoReceiptV1, GisMapApprovalUndoRequestV1};

const GIS_MAP_INFERENCE_POLICY: &[u8] = b"gis-map-v1";
const GIS_MAP_COMMIT_MEMBERS: [&str; 2] = ["gismap-drawing", "gismap-value"];

/// 🌍️ The GIS Map registration of both hub inference binding ports.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct GisMapInferenceBindingV1;

/// 🧮️ Runs the frozen native GIS executable once and decodes exactly its own canonical result.
pub fn deterministic_map_inference(pack: &[u8], job_id: &str, budget: &WireArtifactInferenceBudget) -> Result<(GisMapSnapshot, GisMapInference), MapBindingRefusalV1> {
    use semio_framework_value::FromValue;
    let execution = semio_s_artifact_gis_gismap::infer_gis_map_controlled(&execution_request(pack, job_id, budget), &mut |_| Ok(())).map_err(|_| MapBindingRefusalV1::Execution)?;
    let snapshot = decode_gis_map(pack)?;
    let inference = GisMapInference::from_value(semio_framework_os_kernel::os_pack::pack_rt::decode_wire_value(&execution.canonical_payload).map_err(|_| MapBindingRefusalV1::Undecodable)?).map_err(|_| MapBindingRefusalV1::Undecodable)?;
    Ok((snapshot, inference))
}

fn execution_request<'a>(pack: &'a [u8], job_id: &'a str, budget: &'a WireArtifactInferenceBudget) -> ArtifactInferenceExecutionRequest<'a> {
    ArtifactInferenceExecutionRequest {
        policy: GIS_MAP_INFERENCE_POLICY,
        budgets: budget,
        cancellation_id: job_id,
        previous_state: None,
        requested_cache_mode: WireArtifactInferenceCacheMode::Cold,
        canonical_payload: pack,
        dependencies: &[],
    }
}

fn decode_gis_map(pack: &[u8]) -> Result<GisMapSnapshot, MapBindingRefusalV1> {
    <GisMapSnapshot as ArtifactPack>::decode_pack(pack).map_err(|_| MapBindingRefusalV1::Undecodable)
}

fn json(value: &impl semio_framework_pack_json::ToValue) -> Vec<u8> {
    semio_framework_pack_json::to_json_string(value).into_bytes()
}

fn decode_inverses(payload: &[u8]) -> Result<Vec<GisMapMutation>, MapBindingRefusalV1> {
    let text = std::str::from_utf8(payload).map_err(|_| MapBindingRefusalV1::Undecodable)?;
    let inverses = semio_framework_pack_json::from_json_str::<Vec<GisMapMutation>>(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|_| MapBindingRefusalV1::Undecodable)?;
    if inverses.len() != 1 {
        return Err(MapBindingRefusalV1::Arity);
    }
    Ok(inverses)
}

fn rebuilt_original(current: &GisMapSnapshot, inverse: &GisMapMutation, original_job_id: &str) -> Result<GisMapCreateRegionGroupWorkV1, MapBindingRefusalV1> {
    use semio_framework_os_kernel::Inference as _;
    use semio_s_artifact_gis_gismap::standards::v1::subsets::any::io::text::mutations::apply_gis_map_mutation;
    let mut before = current.clone();
    apply_gis_map_mutation(&mut before, inverse).map_err(|_| MapBindingRefusalV1::Inapplicable)?;
    GisMapInference::infer(&before).create_region_group_work(&before, original_job_id).map_err(|_| MapBindingRefusalV1::Composition)
}

fn composed_work(work: GisMapCreateRegionGroupWorkV1) -> ComposedMapWorkV1<GisMapInferenceBindingV1> {
    ComposedMapWorkV1 {
        parent_json: json(&work.parent),
        parent_inverse_json: json(&work.parent_inverse),
        parent: work.parent,
        parent_inverse: work.parent_inverse,
        drawing_child_id: work.drawing_child.child_id,
        drawing: work.drawing,
        drawing_inverse: work.drawing_inverse,
        value_child_id: work.value_child.child_id,
        value: work.value,
        value_inverse: work.value_inverse,
    }
}

fn stamp(mutation_id: String, timestamp: HybridLogicalTimestamp) -> semio_s_artifact_gis_gismap::editor::gis2d::GisMapOneItemStampV1 {
    semio_s_artifact_gis_gismap::editor::gis2d::GisMapOneItemStampV1 { mutation_id: protocol::MutationId(mutation_id), timestamp }
}

fn parent_store(id: &str, snapshot: GisMapSnapshot, actor: protocol::ActorId) -> ArtifactStore<GisMapSnapshot, GisMapMutation> {
    let mut envelope = semio_framework_os_kernel::os_store::document_materialize::create_document_envelope::<GisMapSnapshot, GisMapMutation>(semio_s_artifact_gis_gismap::GIS_MAP_SCHEMA, id, snapshot, None);
    envelope.dialect = Some(semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.gis.gismap".into(), standard: "1".into(), subset: "*".into() });
    let runtime = semio_framework_os_kernel::os_store::ArtifactStoreInitializationRuntime::new(id, semio_s_artifact_gis_gismap::GIS_MAP_SCHEMA, envelope.vcs.genesis.share_snapshot(), envelope.vcs.genesis.digest(), actor);
    semio_framework_os_kernel::os_store::ArtifactStore::from_initialized_runtime_with_owners(envelope, runtime, 0, semio_s_artifact_gis_gismap::standards::v1::subsets::any::io::binary::mutations::gis_map_document_store_owners())
}

fn drawing_store(id: &str, parent: semio_framework_artifact_reference::ArtifactRef, snapshot: SemioDrawingSnapshot, actor: protocol::ActorId) -> ArtifactStore<SemioDrawingSnapshot, SemioDrawingMutation> {
    use semio_framework_os_kernel::os_store::document_store_owners::MemberStoreOwner;
    let schema = semio_s_artifact_stdio_semio_drawing::schema::snapshot::STDIO_SEMIODRAWING_DOCUMENT_SCHEMA;
    let mut envelope = semio_framework_os_kernel::os_store::document_materialize::create_document_envelope::<SemioDrawingSnapshot, SemioDrawingMutation>(schema, id, snapshot, None);
    envelope.dialect = Some(semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "drawing".into() });
    envelope.owner = Some(semio_framework_os_kernel::os_store::OwnerRef { parent, slot: "drawing".into(), child_id: id.into() });
    let runtime = semio_framework_os_kernel::os_store::ArtifactStoreInitializationRuntime::new(id, schema, envelope.vcs.genesis.share_snapshot(), envelope.vcs.genesis.digest(), actor);
    semio_framework_os_kernel::os_store::ArtifactStore::from_initialized_runtime_with_owners(envelope, runtime, 0, <SemioDrawingSnapshot as MemberStoreOwner<SemioDrawingMutation>>::member_store_owners())
}

fn value_store(id: &str, parent: semio_framework_artifact_reference::ArtifactRef, snapshot: SemioValueSnapshot, actor: protocol::ActorId) -> ArtifactStore<SemioValueSnapshot, SemioValueMutation> {
    use semio_framework_os_kernel::os_store::document_store_owners::MemberStoreOwner;
    let schema = semio_s_artifact_stdio_semio_value_model::schema::snapshot::STDIO_SEMIOVALUE_DOCUMENT_SCHEMA;
    let mut envelope = semio_framework_os_kernel::os_store::document_materialize::create_document_envelope::<SemioValueSnapshot, SemioValueMutation>(schema, id, snapshot, None);
    envelope.dialect = Some(semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "value".into() });
    envelope.owner = Some(semio_framework_os_kernel::os_store::OwnerRef { parent, slot: "value".into(), child_id: id.into() });
    let runtime = semio_framework_os_kernel::os_store::ArtifactStoreInitializationRuntime::new(id, schema, envelope.vcs.genesis.share_snapshot(), envelope.vcs.genesis.digest(), actor);
    semio_framework_os_kernel::os_store::ArtifactStore::from_initialized_runtime_with_owners(envelope, runtime, 0, <SemioValueSnapshot as MemberStoreOwner<SemioValueMutation>>::member_store_owners())
}

/// 🖼️ Decodes one canonical `CreateRegion` proposal into the exact bounded region it would create.
pub fn gis_map_region_preview(job_id: &str, proposal: &[u8]) -> Result<MapRegionPreviewV1, MapBindingRefusalV1> {
    let text = std::str::from_utf8(proposal).map_err(|_| MapBindingRefusalV1::Undecodable)?;
    let mutation = semio_framework_pack_json::from_json_str::<GisMapMutation>(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|_| MapBindingRefusalV1::Undecodable)?;
    let GisMapMutation::CreateRegion(created) = mutation else {
        return Err(MapBindingRefusalV1::Composition);
    };
    let region_id = format!("inference-{job_id}");
    let entries = created.item.data.as_object().ok_or(MapBindingRefusalV1::Undecodable)?;
    if created.item.id != region_id || entries.len() != 3 || created.item.data.get("id").and_then(|value| value.as_str()) != Some(region_id.as_str()) || created.item.data.get("kind").and_then(|value| value.as_str()) != Some("inference-bounds") {
        return Err(MapBindingRefusalV1::Composition);
    }
    let points = created.item.data.get("ring").and_then(|value| value.as_array()).ok_or(MapBindingRefusalV1::Undecodable)?;
    if points.len() != 5 {
        return Err(MapBindingRefusalV1::Bounds);
    }
    let mut ring = [[0.0_f64; 2]; 5];
    for (index, point) in points.iter().enumerate() {
        let coordinates = point.as_array().ok_or(MapBindingRefusalV1::Undecodable)?;
        if coordinates.len() != 2 {
            return Err(MapBindingRefusalV1::Bounds);
        }
        ring[index] = [coordinates[0].as_f64().ok_or(MapBindingRefusalV1::Undecodable)?, coordinates[1].as_f64().ok_or(MapBindingRefusalV1::Undecodable)?];
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
        return Err(MapBindingRefusalV1::Composition);
    }
    Ok(MapRegionPreviewV1 { region_id, ring })
}

impl MapInferenceProposalBindingV1 for GisMapInferenceBindingV1 {
    fn inference_service(&self) -> ArtifactInferenceService {
        semio_s_artifact_gis_gismap::gis_map_inference_service()
    }

    fn commit_members(&self) -> [&'static str; 2] {
        GIS_MAP_COMMIT_MEMBERS
    }

    fn composed_children(&self, pack: &[u8]) -> Result<Vec<String>, MapBindingRefusalV1> {
        let snapshot = decode_gis_map(pack)?;
        let mut members = Vec::with_capacity(3);
        if !snapshot.drawing.child_id.is_empty() {
            members.push(snapshot.drawing.child_id.clone());
        }
        if let Some(image) = snapshot.image.as_ref() {
            members.push(image.child_id.clone());
        }
        if !snapshot.value.child_id.is_empty() {
            members.push(snapshot.value.child_id.clone());
        }
        Ok(members)
    }

    fn infer(&self, pack: &[u8], job_id: &str, budget: &WireArtifactInferenceBudget, checkpoint: &mut dyn FnMut(u64) -> Result<(), ArtifactInferenceExecutionError>) -> Result<MapInferenceRunV1, MapBindingRefusalV1> {
        use semio_framework_value::FromValue;
        let execution = semio_s_artifact_gis_gismap::infer_gis_map_controlled(&execution_request(pack, job_id, budget), checkpoint).map_err(|_| MapBindingRefusalV1::Execution)?;
        let snapshot = decode_gis_map(pack)?;
        let inference = GisMapInference::from_value(semio_framework_os_kernel::os_pack::pack_rt::decode_wire_value(&execution.canonical_payload).map_err(|_| MapBindingRefusalV1::Undecodable)?).map_err(|_| MapBindingRefusalV1::Undecodable)?;
        let mutation = inference.bounds_proposal(&snapshot, job_id).map_err(|_| MapBindingRefusalV1::Composition)?;
        Ok(MapInferenceRunV1 { result: execution.canonical_payload, proposal: json(&mutation) })
    }

    fn approval_effect(&self, pack: &[u8], job_id: &str, budget: &WireArtifactInferenceBudget) -> Result<MapProposalEffectV1, MapBindingRefusalV1> {
        use semio_s_artifact_gis_gismap::mutations::inverse_gis_map_mutation;
        let (snapshot, inference) = deterministic_map_inference(pack, job_id, budget)?;
        let mutation = inference.bounds_proposal(&snapshot, job_id).map_err(|_| MapBindingRefusalV1::Composition)?;
        let inverse = inverse_gis_map_mutation(&snapshot, &mutation).map_err(|_| MapBindingRefusalV1::Composition)?;
        Ok(MapProposalEffectV1 { forward: json(&mutation), inverse: json(&inverse) })
    }

    fn undo_effect(&self, pack: &[u8], original_inverse_payload: &[u8], original_job_id: &str) -> Result<MapUndoEffectV1, MapBindingRefusalV1> {
        let inverses = decode_inverses(original_inverse_payload)?;
        let current = decode_gis_map(pack)?;
        let work = rebuilt_original(&current, &inverses[0], original_job_id)?;
        Ok(MapUndoEffectV1 { original_forward: json(&work.parent), original_inverse: json(&work.parent_inverse), forward: json(&inverses[0]), inverse: json(&vec![work.parent]) })
    }

    fn preview(&self, job_id: &str, proposal: &[u8]) -> Result<MapRegionPreviewV1, MapBindingRefusalV1> {
        gis_map_region_preview(job_id, proposal)
    }
}

impl ComposedMapCommitBindingV1 for GisMapInferenceBindingV1 {
    type ParentSnapshot = GisMapSnapshot;
    type ParentMutation = GisMapMutation;
    type DrawingSnapshot = SemioDrawingSnapshot;
    type DrawingMutation = SemioDrawingMutation;
    type ValueSnapshot = SemioValueSnapshot;
    type ValueMutation = SemioValueMutation;

    const COMMIT_MEMBERS: [&'static str; 2] = GIS_MAP_COMMIT_MEMBERS;

    fn decode_parent(pack: &[u8]) -> Result<GisMapSnapshot, MapBindingRefusalV1> {
        decode_gis_map(pack)
    }

    fn derived_members(parent: &GisMapSnapshot) -> (SemioDrawingSnapshot, SemioValueSnapshot) {
        use semio_s_artifact_gis_gismap::{schema::gis_map_snapshot_to_drawing, standards::v1::subsets::any::io::text::snapshot::gis_map_descriptor_json};
        (gis_map_snapshot_to_drawing(parent), semio_s_artifact_gis_gismap::gis_map_value_from_descriptor_json(&gis_map_descriptor_json(parent)))
    }

    fn member_stores(parent_id: &str, snapshot: GisMapSnapshot, actor: &str) -> ComposedMapMemberStoresV1<Self> {
        let (drawing, value) = Self::derived_members(&snapshot);
        let drawing_id = snapshot.drawing.child_id.clone();
        let value_id = snapshot.value.child_id.clone();
        let actor = protocol::ActorId(actor.to_owned());
        let parent = parent_store(parent_id, snapshot, actor.clone());
        let parent_reference = semio_framework_artifact_reference::ArtifactRef { artifact_id: parent_id.to_owned(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.gis.gismap".into(), standard: "1".into(), subset: "*".into() } };
        ComposedMapMemberStoresV1 { parent, drawing: drawing_store(&drawing_id, parent_reference.clone(), drawing, actor.clone()), value: value_store(&value_id, parent_reference, value, actor) }
    }

    fn executed_approval_work(pack: &[u8], job_id: &str, budget: &WireArtifactInferenceBudget) -> Result<(GisMapSnapshot, ComposedMapWorkV1<Self>), MapBindingRefusalV1> {
        let (snapshot, inference) = deterministic_map_inference(pack, job_id, budget)?;
        let work = inference.create_region_group_work(&snapshot, job_id).map_err(|_| MapBindingRefusalV1::Composition)?;
        Ok((snapshot, composed_work(work)))
    }

    fn approval_work(parent: &GisMapSnapshot, job_id: &str) -> Result<ComposedMapWorkV1<Self>, MapBindingRefusalV1> {
        use semio_framework_os_kernel::Inference as _;
        GisMapInference::infer(parent).create_region_group_work(parent, job_id).map(composed_work).map_err(|_| MapBindingRefusalV1::Composition)
    }

    fn undo_work(parent: &GisMapSnapshot, inverse_payload: &[u8], original_job_id: &str) -> Result<ComposedMapUndoWorkV1<Self>, MapBindingRefusalV1> {
        let inverses = decode_inverses(inverse_payload)?;
        let work = rebuilt_original(parent, &inverses[0], original_job_id)?;
        Ok(ComposedMapUndoWorkV1 { forward_json: json(&inverses[0]), inverse_json: json(&vec![work.parent.clone()]), work: composed_work(work) })
    }

    fn parent_preparation_factory(mutation_id: String, timestamp: HybridLogicalTimestamp) -> Arc<dyn ArtifactStoreOneItemPreparationFactory<GisMapSnapshot, GisMapMutation>> {
        semio_s_artifact_gis_gismap::editor::gis2d::gis_map_parent_stamped_one_item_preparation_factory(stamp(mutation_id, timestamp))
    }

    fn drawing_preparation_factory(mutation_id: String, timestamp: HybridLogicalTimestamp) -> Arc<dyn ArtifactStoreOneItemPreparationFactory<SemioDrawingSnapshot, SemioDrawingMutation>> {
        semio_s_artifact_gis_gismap::editor::gis2d::gis_map_drawing_stamped_one_item_preparation_factory(stamp(mutation_id, timestamp))
    }

    fn value_preparation_factory(mutation_id: String, timestamp: HybridLogicalTimestamp) -> Arc<dyn ArtifactStoreOneItemPreparationFactory<SemioValueSnapshot, SemioValueMutation>> {
        semio_s_artifact_gis_gismap::editor::gis2d::gis_map_value_stamped_one_item_preparation_factory(stamp(mutation_id, timestamp))
    }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
