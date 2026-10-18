export function toolJobPresentationEnvelopeCallerRetainedExact(presentation: string, wasm: string, plugin: string): boolean {
  return (
    presentation.includes("pub struct PresentationEnvelopeOwnedFieldCatalog") &&
    presentation.includes("impl store::os_store::owned_schema_decode::ArtifactEnvelopeOwnedFieldCatalog<PresentationSnapshot, PresentationMutation> for PresentationEnvelopeOwnedFieldCatalog") &&
    presentation.includes("pub struct PresentationEnvelopeMaterializeHandle") &&
    presentation.includes("session: std::mem::ManuallyDrop<Option<semio_framework_job::WorkerJobSession<PresentationEnvelopeMaterializeJob>>>") &&
    presentation.includes("session.try_submit_step(pool, semio_framework_job::Lane::Interactive)") &&
    presentation.includes("pub struct PresentationEnvelopeMaterializeRegistry") &&
    presentation.includes("slots: [PresentationEnvelopeMaterializeSlot; PRESENTATION_ENVELOPE_MATERIALIZE_CAPACITY]") &&
    presentation.includes("handle: std::mem::MaybeUninit<PresentationEnvelopeMaterializeHandle>") &&
    presentation.includes("pub fn try_submit(") &&
    presentation.includes("pub fn maintenance_step(") &&
    presentation.includes("pub fn try_publish_to(") &&
    presentation.includes("pub fn close_step(") &&
    presentation.includes("Presentation envelope materialize handle reached Drop before worker, result, and fault owners were terminal empty") &&
    presentation.includes("Presentation envelope materialize registry reached Drop before every retained caller was closed and reclaimed") &&
    presentation.includes("retained_presentation_envelope_caller_faults_and_zero_grant_closes_malformed_pack") &&
    presentation.includes("retained_presentation_envelope_caller_cancels_and_zero_grant_closes_without_output") &&
    presentation.includes("retained_presentation_envelope_registry_preserves_collision_capacity_and_exact_rejected_pages") &&
    presentation.includes("retained_presentation_envelope_wrong_owner_abort_and_interrupted_close_preserve_live_slot") &&
    presentation.includes("retained_presentation_envelope_publication_retries_backpressure_exactly_once") &&
    presentation.includes("retained_presentation_envelope_stale_generation_cancels_and_unpublished_output_closes") &&
    presentation.includes("retained_presentation_envelope_materializes_populated_history_in_order") &&
    presentation.includes("PresentationEnvelopeMaterializeState::Materialize") &&
    presentation.includes("PresentationEnvelopeMaterializeState::RetireEnvelopeComplete") &&
    presentation.includes("presentation_envelope_decode_owner_bundle().retire_envelope(envelope)") &&
    presentation.includes("let (diff, messages) = mutation.diff(current).into_parts()") &&
    presentation.includes("match diff.apply(current)") &&
    !presentation.includes("envelope.try_into_fresh_snapshot()") &&
    plugin.includes("fn build_envelope_decode_owner_bundle() -> Option<store::os_store::owned_schema_decode::ArtifactEnvelopeDecodeOwnerBundle<Self::Snapshot, Self::Mutation>>") &&
    plugin.includes("envelope_decode_jobs: ArtifactFixedRegistry<ActiveArtifactEnvelopeDecode<A::Snapshot, A::Mutation>>") &&
    plugin.includes("pub fn submit_artifact_envelope_decode(") &&
    plugin.includes("pub fn try_publish_artifact_envelope_decode(") &&
    plugin.includes("fn drive_envelope_decode_jobs(") &&
    plugin.includes("close_envelope_decode_jobs_drained") &&
    plugin.includes("active envelope decode reached Drop before its worker and rejected owner were terminal empty") &&
    !presentation.includes("pub fn begin_materialize_presentation_projection(") &&
    !presentation.includes("materialize_presentation_projection_json") &&
    !wasm.includes("materializePresentationProjectionJson") &&
    !wasm.includes("materialize_presentation_projection_json") &&
    !wasm.includes("createPresentationEnvelopeJson") &&
    !wasm.includes("envelope_json") &&
    !wasm.includes("reject_whole_buffer_artifact_envelope_ingress")
  );
}

export function toolJobStoreInitializerRetainedExact(plugin: string): boolean {
  return (
    plugin.includes("session: std::mem::ManuallyDrop<Option<semio_framework_job::MountedWorkerJobSession<ArtifactStoreInitializationJob<P, Mutation>>>>") &&
    plugin.includes("session_rejected: std::mem::ManuallyDrop<Option<semio_framework_job::WorkerJobSessionAdmissionRejected<ArtifactStoreInitializationJob<P, Mutation>>>>") &&
    plugin.includes("MountedWorkerJobSession::try_new(job, params)") &&
    plugin.includes("self.jobs.insert_admitted(self.operation.0, ActiveArtifactStoreReplacement::new(self.operation, self.generation, job))") &&
    plugin.includes("artifact store replacement reached Drop before initializer/candidate/displaced-store ownership was terminal empty")
  );
}

export function toolJobWriterEnvelopeCallerRetainedExact(writer: string, editor: string, wasm: string, plugin: string): boolean {
  return (
    writer.includes("struct WriterStoreInitializationAuthority") &&
    writer.includes("impl semio_framework_plugin::ArtifactStoreInitializationAuthority<WriterSnapshot, WriterMutation> for WriterStoreInitializationAuthority") &&
    writer.includes("WriterStoreInitializationPhase::ValidateEdit { index }") &&
    writer.includes("self.edit_index.admit(&envelope.vcs.edits, index,") &&
    writer.includes("WriterStoreInitializationPhase::SeedHistory") &&
    writer.includes("WriterStoreInitializationPhase::BuildCandidate") &&
    writer.includes("ArtifactStoreInitializationRuntime::new") &&
    writer.includes("ArtifactStore::from_initialized_runtime_with_owners") &&
    writer.includes("self.generation.0.checked_add(1)") &&
    writer.includes("Writer store initialization authority reached Drop before exact candidate handoff or retained rejection close") &&
    writer.includes("writer_store_initializer_publishes_exact_next_generation_and_candidate_closes_incrementally") &&
    writer.includes("writer_store_initializer_cancel_and_stale_generation_return_every_owner_terminal_empty") &&
    editor.includes("fn build_document_store_initialization_job(") &&
    editor.includes("writer_document_store_initialization_job(envelope, operation, generation)") &&
    wasm.includes("pub struct WriterEnvelopeLoadHandle") &&
    wasm.includes("pub fn begin_envelope_load(") &&
    wasm.includes("begin_artifact_envelope_ingress(maximum_pages, maximum_bytes)") &&
    wasm.includes("source: &js_sys::Uint8Array") &&
    wasm.includes("pub fn admit_envelope_page(") &&
    wasm.includes("admit_artifact_envelope_ingress_page(handle.runtime_handle(), page)") &&
    wasm.includes("pub fn seal_envelope_load(") &&
    wasm.includes("seal_artifact_envelope_ingress(handle.runtime_handle())") &&
    wasm.includes("pub fn poll_envelope_load(") &&
    wasm.includes("app.maintenance_step(1, store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)") &&
    wasm.includes("advance_artifact_envelope_load(handle.runtime_handle())") &&
    wasm.includes("acknowledge_artifact_store_replacement(handle.runtime_handle())") &&
    wasm.includes("pub fn cancel_envelope_load(") &&
    wasm.includes("pub fn close_step(&self)") &&
    editor.includes("writer_live_envelope_submit_pump_swap_displaced_store_and_exact_ack_succeed") &&
    editor.includes("writer_live_envelope_cancel_closes_retained_pages_without_publication") &&
    plugin.includes("envelope_ingress: ArtifactFixedRegistry<ActiveArtifactEnvelopeIngress>") &&
    plugin.includes("pub fn begin_artifact_envelope_ingress(") &&
    plugin.includes("pub fn seal_artifact_envelope_ingress(") &&
    plugin.includes("pub fn advance_artifact_envelope_load(") &&
    plugin.includes("fn drive_envelope_ingress(") &&
    plugin.includes("saturated decoder keeps the sealed ingress owner in its original fixed slot for retry") &&
    toolJobStoreInitializerRetainedExact(plugin) &&
    plugin.includes("artifact envelope ingress reached Drop before every admitted page was transferred or bounded-closed") &&
    plugin.includes("artifact_envelope_ingress_saturation_returns_exact_plus_one_owner_and_closes_fifo_slots") &&
    plugin.includes("artifact_envelope_ingress_cancel_and_interrupted_close_release_one_real_page_per_grant") &&
    !wasm.includes("envelope_json: &str") &&
    !wasm.includes("reject_whole_buffer_artifact_envelope_ingress") &&
    !wasm.includes("ArtifactStore::new") &&
    !wasm.includes("while let") &&
    !wasm.includes("loop {")
  );
}

export function toolJobJackEnvelopeCallerRetainedExact(store: string, jack: string, editor: string, wasm: string, plugin: string): boolean {
  return (
    store.includes("pub fn artifact_owned_spr_edit_history_decoder") &&
    store.includes("struct ArtifactOwnedSprMutationArrayAuthority") &&
    store.includes("self.scalar_entry") &&
    store.includes("artifact-spr.mutation-array-cancelled") &&
    store.includes("SPR edit decode reached Drop before exact publication or bounded retirement") &&
    store.includes("pub fn preflight_page_bytes(&self, page_bytes: usize)") &&
    store.includes("pub fn admit_preflighted_page(&mut self, page: OwnedSchemaDecodePage)") &&
    jack.includes("pub struct JackEnvelopeOwnedFieldCatalog") &&
    jack.includes("artifact_owned_spr_edit_history_decoder") &&
    jack.includes("struct JackMutationDecodeAuthority") &&
    jack.includes("OwnedSchemaHexAuthority<JACK_OWNED_FIELD_BYTES>") &&
    jack.includes("struct JackSnapshotCloneAuthority") &&
    jack.includes("struct JackStoreInitializationAuthority") &&
    jack.includes("impl semio_framework_plugin::ArtifactStoreInitializationAuthority<JackSnapshot, TrinityGraphMutation> for JackStoreInitializationAuthority") &&
    jack.includes("JackStoreInitializationPhase::ValidateEdit { index }") &&
    jack.includes("self.edit_index.admit(&envelope.vcs.edits, index,") &&
    jack.includes("JackStoreInitializationPhase::SeedHistory") &&
    jack.includes("JackStoreInitializationPhase::BuildCandidate") &&
    jack.includes("ArtifactStoreInitializationRuntime::new") &&
    jack.includes("ArtifactStore::from_initialized_runtime_with_owners") &&
    jack.includes("self.generation.0.checked_add(1)") &&
    jack.includes("Jack store initialization authority reached Drop before exact candidate handoff or retained rejection close") &&
    jack.includes("jack_store_initializer_publishes_exact_next_generation_and_candidate_closes_incrementally") &&
    jack.includes("jack_store_initializer_cancel_and_stale_generation_return_every_owner_terminal_empty") &&
    jack.includes("jack_nested_mutation_and_child_snapshot_retire_one_exact_owner_per_grant") &&
    !jack.includes("artifact_bounded_history_entry_decoder") &&
    editor.includes("fn build_document_store_initialization_job(") &&
    editor.includes("jack_document_store_initialization_job(envelope, operation, generation)") &&
    editor.includes("jack_live_envelope_submit_pump_swap_displaced_store_and_exact_ack_succeed") &&
    editor.includes("jack_live_envelope_cancel_closes_retained_pages_without_publication") &&
    wasm.includes("pub struct JackEnvelopeLoadHandle") &&
    wasm.includes("pub fn begin_envelope_load(") &&
    wasm.includes("begin_artifact_envelope_ingress(maximum_pages, maximum_bytes)") &&
    wasm.includes("source: &js_sys::Uint8Array") &&
    wasm.includes("pub fn admit_envelope_page(") &&
    wasm.includes("admit_artifact_envelope_ingress_page(handle.runtime_handle(), page)") &&
    wasm.includes("pub fn seal_envelope_load(") &&
    wasm.includes("seal_artifact_envelope_ingress(handle.runtime_handle())") &&
    wasm.includes("pub fn poll_envelope_load(") &&
    wasm.includes("app.maintenance_step(1, store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)") &&
    wasm.includes("advance_artifact_envelope_load(handle.runtime_handle())") &&
    wasm.includes("acknowledge_artifact_store_replacement(handle.runtime_handle())") &&
    wasm.includes("pub fn cancel_envelope_load(") &&
    wasm.includes("pub fn close_step(&self)") &&
    plugin.includes("envelope_ingress: ArtifactFixedRegistry<ActiveArtifactEnvelopeIngress>") &&
    toolJobStoreInitializerRetainedExact(plugin) &&
    !wasm.includes("envelope_json: &str") &&
    !wasm.includes("reject_whole_buffer_artifact_envelope_ingress") &&
    !wasm.includes("ArtifactStore::new") &&
    !wasm.includes("while let") &&
    !wasm.includes("loop {")
  );
}

export function toolJobTrinityRewriteEnvelopeCallerRetainedExact(store: string, jack: string, editor: string, rewrite: string, plugin: string): boolean {
  return (
    store.includes("pub fn artifact_owned_spr_edit_history_decoder") &&
    store.includes("struct ArtifactOwnedSprMutationArrayAuthority") &&
    store.includes("self.scalar_entry") &&
    store.includes("artifact-spr.mutation-array-cancelled") &&
    store.includes("SPR edit decode reached Drop before exact publication or bounded retirement") &&
    jack.includes("pub struct JackEnvelopeOwnedFieldCatalog") &&
    jack.includes("artifact_owned_spr_edit_history_decoder") &&
    jack.includes("struct JackMutationDecodeAuthority") &&
    jack.includes("OwnedSchemaHexAuthority<JACK_OWNED_FIELD_BYTES>") &&
    jack.includes("struct JackSnapshotCloneAuthority") &&
    jack.includes("struct JackStoreInitializationAuthority") &&
    jack.includes("impl semio_framework_plugin::ArtifactStoreInitializationAuthority<JackSnapshot, TrinityGraphMutation> for JackStoreInitializationAuthority") &&
    jack.includes("JackStoreInitializationPhase::ValidateEdit { index }") &&
    jack.includes("self.edit_index.admit(&envelope.vcs.edits, index,") &&
    jack.includes("JackStoreInitializationPhase::SeedHistory") &&
    jack.includes("JackStoreInitializationPhase::BuildCandidate") &&
    jack.includes("ArtifactStoreInitializationRuntime::new") &&
    jack.includes("ArtifactStore::from_initialized_runtime_with_owners") &&
    jack.includes("self.generation.0.checked_add(1)") &&
    jack.includes("Jack store initialization authority reached Drop before exact candidate handoff or retained rejection close") &&
    jack.includes("jack_store_initializer_publishes_exact_next_generation_and_candidate_closes_incrementally") &&
    jack.includes("jack_store_initializer_cancel_and_stale_generation_return_every_owner_terminal_empty") &&
    jack.includes("jack_nested_mutation_and_child_snapshot_retire_one_exact_owner_per_grant") &&
    editor.includes("fn build_document_store_initialization_job(") &&
    editor.includes("jack_document_store_initialization_job(envelope, operation, generation)") &&
    editor.includes("jack_live_envelope_submit_pump_swap_displaced_store_and_exact_ack_succeed") &&
    editor.includes("jack_live_envelope_cancel_closes_retained_pages_without_publication") &&
    rewrite.includes("type TrinityRewriteApp = VcsArtifactApp<EditorApp<TrinityJackPlayApp>>") &&
    rewrite.includes("pub struct TrinityRewriteEnvelopeLoadHandle") &&
    rewrite.includes("fn runtime_handle(&self) -> ArtifactEnvelopeDecodeOperationHandle") &&
    rewrite.includes("TRINITY_REWRITE_ENVELOPE_MAXIMUM_PAGES") &&
    rewrite.includes("TRINITY_REWRITE_ENVELOPE_MAXIMUM_BYTES") &&
    rewrite.includes("trinity_rewrite_envelope_credits_admit_exact_caps_and_reject_zero_or_plus_one") &&
    rewrite.includes("pub async fn new()") &&
    rewrite.includes("VcsArtifactApp::new(EditorApp::<TrinityJackPlayApp>::default()).await") &&
    rewrite.includes("pub fn begin_envelope_load(") &&
    rewrite.includes("begin_artifact_envelope_ingress(maximum_pages, maximum_bytes)") &&
    rewrite.includes("source: &js_sys::Uint8Array") &&
    rewrite.includes("pub enum TrinityRewriteEnvelopePageFault") &&
    rewrite.includes("pub struct TrinityRewriteEnvelopePageAdmission") &&
    rewrite.includes("TrinityRewriteCallerPageOwner::new(owner)") &&
    rewrite.includes("source.clone()") &&
    rewrite.includes("source.copy_to(&mut bytes[..len])") &&
    rewrite.includes("let mut bytes = [0; store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES]") &&
    rewrite.includes("ArtifactEnvelopeDecodePage::from_preflighted_array(bytes, len)") &&
    rewrite.includes("construct_and_admit_artifact_envelope_ingress_page(handle.runtime_handle(), len") &&
    rewrite.includes("pub fn retry_envelope_page(") &&
    rewrite.includes("pub fn is_same_page(") &&
    rewrite.includes("pub fn take_page(&mut self) -> Option<js_sys::Uint8Array>") &&
    rewrite.includes("pub fn close_step(&mut self) -> bool") &&
    rewrite.includes("pub fn terminal_is_empty(&self) -> bool") &&
    rewrite.includes("trinity_rewrite_rejected_page_preserves_pointer_content_and_retry_owner") &&
    rewrite.includes("trinity_rewrite_page_cap_plus_one_rejects_before_owner_construction") &&
    rewrite.includes("trinity_rewrite_stale_generation_and_slot_aba_never_match") &&
    rewrite.includes("trinity_rewrite_checked_out_drop_preserves_raw_caller_authority") &&
    rewrite.includes("trinity_rewrite_rejected_page_close_retires_one_owner_per_grant") &&
    rewrite.indexOf("construct_and_admit_artifact_envelope_ingress_page(handle.runtime_handle(), len") < rewrite.indexOf("source.copy_to(&mut bytes[..len])") &&
    rewrite.includes("seal_artifact_envelope_ingress(handle.runtime_handle())") &&
    rewrite.includes("app.maintenance_step(1, store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)") &&
    rewrite.includes("advance_artifact_envelope_load(handle.runtime_handle())") &&
    rewrite.includes("acknowledge_artifact_store_replacement(handle.runtime_handle())") &&
    rewrite.includes("cancel_artifact_envelope_load(handle.runtime_handle())") &&
    rewrite.includes("close_step(1, store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)") &&
    plugin.includes("envelope_ingress: ArtifactFixedRegistry<ActiveArtifactEnvelopeIngress>") &&
    plugin.includes("pub fn preflight_artifact_envelope_ingress_page(") &&
    plugin.includes("pub fn construct_and_admit_artifact_envelope_ingress_page<Build>(") &&
    toolJobStoreInitializerRetainedExact(plugin) &&
    plugin.includes("artifact_envelope_ingress_saturation_returns_exact_plus_one_owner_and_closes_fifo_slots") &&
    plugin.includes("artifact_envelope_ingress_cancel_and_interrupted_close_release_one_real_page_per_grant") &&
    !rewrite.includes("envelope_json: Option<String>") &&
    !rewrite.includes("RefCell<TrinityGraphStore>") &&
    !rewrite.includes("reject_whole_buffer_artifact_envelope_ingress") &&
    !rewrite.includes("dispatch_text(command_text)") &&
    !rewrite.includes("dispatch_binary(command_bytes)") &&
    !rewrite.includes("map_err(|(fault, _page)|") &&
    !rewrite.includes("source.to_vec()") &&
    !rewrite.includes("Uint8Array::new(source)") &&
    !rewrite.includes("while let") &&
    !rewrite.includes("loop {")
  );
}

export function toolJobGisMapEnvelopeCallerRetainedExact(store: string, gisMap: string, editor: string, wasm: string, plugin: string): boolean {
  const variants = [
    "CreatePosition(payload)",
    "DeletePosition(payload)",
    "ReorderPositions(payload)",
    "ReplacePositionData(payload)",
    "CreateRoute(payload)",
    "DeleteRoute(payload)",
    "ReorderRoutes(payload)",
    "ReplaceRouteData(payload)",
    "CreateRegion(payload)",
    "DeleteRegion(payload)",
    "ReorderRegions(payload)",
    "ReplaceRegionData(payload)",
  ];
  return (
    store.includes("pub fn artifact_owned_spr_edit_history_decoder") &&
    store.includes("struct ArtifactOwnedSprMutationArrayAuthority") &&
    store.includes("self.scalar_entry") &&
    store.includes("artifact-spr.mutation-array-cancelled") &&
    store.includes("SPR edit decode reached Drop before exact publication or bounded retirement") &&
    gisMap.includes("pub struct GisMapEnvelopeOwnedFieldCatalog") &&
    gisMap.includes("artifact_owned_spr_edit_history_decoder") &&
    gisMap.includes("GisMapSnapshotDecodeAuthority") &&
    gisMap.includes("GisMapMutationDecodeAuthority") &&
    gisMap.includes("OwnedSchemaHexAuthority<GIS_MAP_OWNED_FIELD_BYTES>") &&
    gisMap.includes("struct GisMapSnapshotCloneAuthority") &&
    gisMap.includes("struct GisMapStoreInitializationAuthority") &&
    gisMap.includes("impl semio_framework_plugin::ArtifactStoreInitializationAuthority<GisMapSnapshot, GisMapMutation> for GisMapStoreInitializationAuthority") &&
    gisMap.includes("GisMapStoreInitializationPhase::ValidateEdit { index }") &&
    gisMap.includes("self.edit_index.admit(&envelope.vcs.edits, index,") &&
    gisMap.includes("GisMapStoreInitializationPhase::SeedHistory") &&
    gisMap.includes("GisMapStoreInitializationPhase::BuildCandidate") &&
    gisMap.includes("ArtifactStoreInitializationRuntime::new") &&
    gisMap.includes("ArtifactStore::from_initialized_runtime_with_owners") &&
    gisMap.includes("self.generation.0.checked_add(1)") &&
    gisMap.includes("value.positions.pop()") &&
    gisMap.includes("value.routes.pop()") &&
    gisMap.includes("value.regions.pop()") &&
    gisMap.includes("Self::child_step(&mut value.drawing") &&
    gisMap.includes("Self::child_step(image") &&
    gisMap.includes("Self::child_step(&mut value.value") &&
    gisMap.includes("dsl::DslValue::Array(values)") &&
    gisMap.includes("dsl::DslValue::Object(values)") &&
    variants.every((variant) => gisMap.includes(variant)) &&
    gisMap.includes("GIS store initialization authority reached Drop before exact candidate handoff or retained rejection close") &&
    gisMap.includes("gis_map_store_initializer_publishes_next_generation_and_candidate_closes_incrementally") &&
    gisMap.includes("gis_map_store_initializer_cancel_and_stale_generation_return_every_owner_terminal_empty") &&
    gisMap.includes("gis_map_nested_value_mutation_and_all_child_handles_retire_one_owner_per_grant") &&
    gisMap.includes("gis_map_all_twelve_mutation_variants_preserve_catalog_order_and_zero_grant_ownership") &&
    !gisMap.includes("artifact_bounded_history_entry_decoder") &&
    editor.includes("fn build_document_store_initialization_job(") &&
    editor.includes("gis_map_document_store_initialization_job(envelope, operation, generation)") &&
    editor.includes("gis_map_live_envelope_submit_pump_swap_displaced_store_and_exact_ack_succeed") &&
    editor.includes("gis_map_live_envelope_cancel_closes_retained_pages_without_publication") &&
    editor.includes("duplicate GIS load acknowledgement is a no-op") &&
    wasm.includes("pub struct GisMapEnvelopeLoadHandle") &&
    wasm.includes("pub fn begin_envelope_load(") &&
    wasm.includes("begin_artifact_envelope_ingress(maximum_pages, maximum_bytes)") &&
    wasm.includes("source: &js_sys::Uint8Array") &&
    wasm.includes("pub fn admit_envelope_page(") &&
    wasm.includes("admit_artifact_envelope_ingress_page(handle.runtime_handle(), page)") &&
    wasm.includes("pub fn seal_envelope_load(") &&
    wasm.includes("seal_artifact_envelope_ingress(handle.runtime_handle())") &&
    wasm.includes("pub fn poll_envelope_load(") &&
    wasm.includes("app.maintenance_step(1, store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)") &&
    wasm.includes("advance_artifact_envelope_load(handle.runtime_handle())") &&
    wasm.includes("acknowledge_artifact_store_replacement(handle.runtime_handle())") &&
    wasm.includes("pub fn cancel_envelope_load(") &&
    wasm.includes("pub fn close_step(&self)") &&
    plugin.includes("envelope_ingress: ArtifactFixedRegistry<ActiveArtifactEnvelopeIngress>") &&
    toolJobStoreInitializerRetainedExact(plugin) &&
    !wasm.includes("envelope_json: &str") &&
    !wasm.includes("reject_whole_buffer_artifact_envelope_ingress") &&
    !wasm.includes("ArtifactStore::new") &&
    !wasm.includes("while let") &&
    !wasm.includes("loop {")
  );
}
