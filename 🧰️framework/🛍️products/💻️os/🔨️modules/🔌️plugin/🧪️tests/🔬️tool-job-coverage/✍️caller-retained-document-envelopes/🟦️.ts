/** ✍️ The writer, jack, trinity rewrite and gis map caller retained document envelope checks. */
import { toolJobWriterEnvelopeCallerRetainedExact, toolJobJackEnvelopeCallerRetainedExact, toolJobTrinityRewriteEnvelopeCallerRetainedExact, toolJobGisMapEnvelopeCallerRetainedExact } from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/⚖️laws/🟦️.ts";

/** 📝 Checks the writer envelope caller retained route and returns the writer sources the other documents derive from. */
export function writerEnvelopeCallerRetainedChecks() {
  const retainedWriterInitializer = [
    "struct WriterStoreInitializationAuthority",
    "impl semio_framework_plugin::ArtifactStoreInitializationAuthority<WriterSnapshot, WriterMutation> for WriterStoreInitializationAuthority",
    "WriterStoreInitializationPhase::ValidateEdit { index }",
    "self.edit_index.admit(&envelope.vcs.edits, index,",
    "WriterStoreInitializationPhase::SeedHistory",
    "WriterStoreInitializationPhase::BuildCandidate",
    "ArtifactStoreInitializationRuntime::new",
    "ArtifactStore::from_initialized_runtime_with_owners",
    "self.generation.0.checked_add(1)",
    "Writer store initialization authority reached Drop before exact candidate handoff or retained rejection close",
    "writer_store_initializer_publishes_exact_next_generation_and_candidate_closes_incrementally",
    "writer_store_initializer_cancel_and_stale_generation_return_every_owner_terminal_empty",
  ].join("\n");
  const retainedWriterEditor = "fn build_document_store_initialization_job( writer_document_store_initialization_job(envelope, operation, generation) writer_live_envelope_submit_pump_swap_displaced_store_and_exact_ack_succeed writer_live_envelope_cancel_closes_retained_pages_without_publication";
  const retainedWriterWasm = [
    "pub struct WriterEnvelopeLoadHandle",
    "pub fn begin_envelope_load(",
    "begin_artifact_envelope_ingress(maximum_pages, maximum_bytes)",
    "source: &js_sys::Uint8Array",
    "pub fn admit_envelope_page(",
    "admit_artifact_envelope_ingress_page(handle.runtime_handle(), page)",
    "pub fn seal_envelope_load(",
    "seal_artifact_envelope_ingress(handle.runtime_handle())",
    "pub fn poll_envelope_load(",
    "app.maintenance_step(1, store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)",
    "advance_artifact_envelope_load(handle.runtime_handle())",
    "acknowledge_artifact_store_replacement(handle.runtime_handle())",
    "pub fn cancel_envelope_load(",
    "pub fn close_step(&self)",
  ].join("\n");
  const retainedWriterPlugin = [
    "envelope_ingress: ArtifactFixedRegistry<ActiveArtifactEnvelopeIngress>",
    "pub fn begin_artifact_envelope_ingress(",
    "pub fn preflight_artifact_envelope_ingress_page(",
    "pub fn construct_and_admit_artifact_envelope_ingress_page<Build>(",
    "pub fn seal_artifact_envelope_ingress(",
    "pub fn advance_artifact_envelope_load(",
    "fn drive_envelope_ingress(",
    "saturated decoder keeps the sealed ingress owner in its original fixed slot for retry",
    "session: std::mem::ManuallyDrop<Option<semio_framework_job::MountedWorkerJobSession<ArtifactStoreInitializationJob<P, Mutation>>>>",
    "session_rejected: std::mem::ManuallyDrop<Option<semio_framework_job::WorkerJobSessionAdmissionRejected<ArtifactStoreInitializationJob<P, Mutation>>>>",
    "MountedWorkerJobSession::try_new(job, params)",
    "self.jobs.insert_admitted(self.operation.0, ActiveArtifactStoreReplacement::new(self.operation, self.generation, job))",
    "artifact store replacement reached Drop before initializer/candidate/displaced-store ownership was terminal empty",
    "artifact store initialization job reached Drop before exact candidate handoff or terminal retained close",
    "artifact envelope ingress reached Drop before every admitted page was transferred or bounded-closed",
    "artifact_envelope_ingress_saturation_returns_exact_plus_one_owner_and_closes_fifo_slots",
    "artifact_envelope_ingress_cancel_and_interrupted_close_release_one_real_page_per_grant",
  ].join("\n");
  if (!toolJobWriterEnvelopeCallerRetainedExact(retainedWriterInitializer, retainedWriterEditor, retainedWriterWasm, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test retained-Writer-envelope-route was falsely rejected.");
  if (toolJobWriterEnvelopeCallerRetainedExact(retainedWriterInitializer, retainedWriterEditor, retainedWriterWasm.replace("source: &js_sys::Uint8Array", "source: &[u8]"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Writer-post-lift-dynamic-page was falsely accepted.");
  if (toolJobWriterEnvelopeCallerRetainedExact(retainedWriterInitializer, retainedWriterEditor, retainedWriterWasm.replace("acknowledge_artifact_store_replacement(handle.runtime_handle())", "return Ready"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Writer-completion-without-exact-ack was falsely accepted.");
  if (toolJobWriterEnvelopeCallerRetainedExact(retainedWriterInitializer, retainedWriterEditor, retainedWriterWasm, retainedWriterPlugin.replace("self.jobs.insert_admitted(self.operation.0, ActiveArtifactStoreReplacement::new(self.operation, self.generation, job))", "drop(job)"))) throw new Error("[verify interactivity tool-jobs] self-test Writer-false-terminal-initializer-drop was falsely accepted.");
  return { retainedWriterWasm, retainedWriterPlugin };
}

/** 🔌 Checks the jack envelope caller retained route. */
export function jackEnvelopeCallerRetainedChecks({ retainedWriterWasm, retainedWriterPlugin }: {
  readonly retainedWriterWasm: string;
  readonly retainedWriterPlugin: string;
}) {
  const retainedJackStore = [
    "pub fn artifact_owned_spr_edit_history_decoder",
    "struct ArtifactOwnedSprMutationArrayAuthority",
    "self.scalar_entry",
    "artifact-spr.mutation-array-cancelled",
    "id_digest: [u8; 32]",
    'Self::push_revision_record(&mut self.revision.applied, identity_digest, b"applied", &id, edit_digest)?',
    "SPR edit decode reached Drop before exact publication or bounded retirement",
    "pub fn preflight_page_bytes(&self, page_bytes: usize)",
    "pub fn admit_preflighted_page(&mut self, page: OwnedSchemaDecodePage)",
  ].join("\n");
  const retainedJackCodec = [
    "pub struct JackEnvelopeOwnedFieldCatalog",
    "artifact_owned_spr_edit_history_decoder",
    "struct JackMutationDecodeAuthority",
    "OwnedSchemaHexAuthority<JACK_OWNED_FIELD_BYTES>",
    "struct JackSnapshotCloneAuthority",
    "struct JackStoreInitializationAuthority",
    "impl semio_framework_plugin::ArtifactStoreInitializationAuthority<JackSnapshot, TrinityGraphMutation> for JackStoreInitializationAuthority",
    "JackStoreInitializationPhase::ValidateEdit { index }",
    "self.edit_index.admit(&envelope.vcs.edits, index,",
    "JackStoreInitializationPhase::SeedHistory",
    "JackStoreInitializationPhase::BuildCandidate",
    "ArtifactStoreInitializationRuntime::new",
    "ArtifactStore::from_initialized_runtime_with_owners",
    "self.generation.0.checked_add(1)",
    "Jack store initialization authority reached Drop before exact candidate handoff or retained rejection close",
    "jack_store_initializer_publishes_exact_next_generation_and_candidate_closes_incrementally",
    "jack_store_initializer_cancel_and_stale_generation_return_every_owner_terminal_empty",
    "jack_nested_mutation_and_child_snapshot_retire_one_exact_owner_per_grant",
  ].join("\n");
  const retainedJackEditor = "fn build_document_store_initialization_job( jack_document_store_initialization_job(envelope, operation, generation) jack_live_envelope_submit_pump_swap_displaced_store_and_exact_ack_succeed jack_live_envelope_cancel_closes_retained_pages_without_publication";
  const retainedJackWasm = retainedWriterWasm.replace("WriterEnvelopeLoadHandle", "JackEnvelopeLoadHandle");
  if (!toolJobJackEnvelopeCallerRetainedExact(retainedJackStore, retainedJackCodec, retainedJackEditor, retainedJackWasm, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test retained-Jack-envelope-route was falsely rejected.");
  if (toolJobJackEnvelopeCallerRetainedExact(retainedJackStore, retainedJackCodec.replace("artifact_owned_spr_edit_history_decoder", "artifact_bounded_history_entry_decoder"), retainedJackEditor, retainedJackWasm, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Jack-domain-only-whole-edit-decoder was falsely accepted.");
  if (toolJobJackEnvelopeCallerRetainedExact(retainedJackStore, retainedJackCodec, retainedJackEditor, retainedJackWasm.replace("source: &js_sys::Uint8Array", "source: &[u8]"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Jack-post-lift-dynamic-page was falsely accepted.");
  if (toolJobJackEnvelopeCallerRetainedExact(retainedJackStore, retainedJackCodec, retainedJackEditor, retainedJackWasm.replace("acknowledge_artifact_store_replacement(handle.runtime_handle())", "return Ready"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Jack-completion-without-exact-ack was falsely accepted.");
  if (toolJobJackEnvelopeCallerRetainedExact(retainedJackStore, retainedJackCodec, retainedJackEditor, retainedJackWasm, retainedWriterPlugin.replace("self.jobs.insert_admitted(self.operation.0, ActiveArtifactStoreReplacement::new(self.operation, self.generation, job))", "drop(job)"))) throw new Error("[verify interactivity tool-jobs] self-test Jack-false-terminal-initializer-drop was falsely accepted.");
  if (toolJobJackEnvelopeCallerRetainedExact(retainedJackStore.replace("self.scalar_entry", "let raw = serde_json::from_slice(bytes)"), retainedJackCodec, retainedJackEditor, retainedJackWasm, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Jack-string-mutation-whole-buffer-bypass was falsely accepted.");
  return { retainedJackStore, retainedJackCodec, retainedJackEditor };
}

/** 🔁 Checks the trinity rewrite envelope caller retained route. */
export function trinityRewriteEnvelopeCallerRetainedChecks({ retainedJackStore, retainedJackCodec, retainedJackEditor, retainedWriterPlugin }: {
  readonly retainedJackStore: string;
  readonly retainedJackCodec: string;
  readonly retainedJackEditor: string;
  readonly retainedWriterPlugin: string;
}) {
  const retainedTrinityRewrite = [
    "type TrinityRewriteApp = VcsArtifactApp<EditorApp<TrinityJackPlayApp>>",
    "pub struct TrinityRewriteEnvelopeLoadHandle",
    "fn runtime_handle(&self) -> ArtifactEnvelopeDecodeOperationHandle",
    "TRINITY_REWRITE_ENVELOPE_MAXIMUM_PAGES",
    "TRINITY_REWRITE_ENVELOPE_MAXIMUM_BYTES",
    "trinity_rewrite_envelope_credits_admit_exact_caps_and_reject_zero_or_plus_one",
    "pub async fn new()",
    "VcsArtifactApp::new(EditorApp::<TrinityJackPlayApp>::default()).await",
    "pub fn begin_envelope_load(",
    "begin_artifact_envelope_ingress(maximum_pages, maximum_bytes)",
    "source: &js_sys::Uint8Array",
    "pub enum TrinityRewriteEnvelopePageFault",
    "pub struct TrinityRewriteEnvelopePageAdmission",
    "TrinityRewriteCallerPageOwner::new(owner)",
    "source.clone()",
    "construct_and_admit_artifact_envelope_ingress_page(handle.runtime_handle(), len",
    "let mut bytes = [0; store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES]",
    "source.copy_to(&mut bytes[..len])",
    "ArtifactEnvelopeDecodePage::from_preflighted_array(bytes, len)",
    "pub fn retry_envelope_page(",
    "pub fn is_same_page(",
    "pub fn take_page(&mut self) -> Option<js_sys::Uint8Array>",
    "pub fn close_step(&mut self) -> bool",
    "pub fn terminal_is_empty(&self) -> bool",
    "trinity_rewrite_rejected_page_preserves_pointer_content_and_retry_owner",
    "trinity_rewrite_page_cap_plus_one_rejects_before_owner_construction",
    "trinity_rewrite_stale_generation_and_slot_aba_never_match",
    "trinity_rewrite_checked_out_drop_preserves_raw_caller_authority",
    "trinity_rewrite_rejected_page_close_retires_one_owner_per_grant",
    "seal_artifact_envelope_ingress(handle.runtime_handle())",
    "app.maintenance_step(1, store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)",
    "advance_artifact_envelope_load(handle.runtime_handle())",
    "acknowledge_artifact_store_replacement(handle.runtime_handle())",
    "cancel_artifact_envelope_load(handle.runtime_handle())",
    "close_step(1, store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)",
  ].join("\n");
  if (!toolJobTrinityRewriteEnvelopeCallerRetainedExact(retainedJackStore, retainedJackCodec, retainedJackEditor, retainedTrinityRewrite, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test retained-Trinity-Rewrite-envelope-route was falsely rejected.");
  if (toolJobTrinityRewriteEnvelopeCallerRetainedExact(retainedJackStore, retainedJackCodec, retainedJackEditor, `${retainedTrinityRewrite}\nenvelope_json: Option<String>`, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Trinity-Rewrite-whole-buffer-constructor-bypass was falsely accepted.");
  if (toolJobTrinityRewriteEnvelopeCallerRetainedExact(retainedJackStore, retainedJackCodec, retainedJackEditor, retainedTrinityRewrite.replace("source: &js_sys::Uint8Array", "source: &[u8]"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Trinity-Rewrite-post-lift-dynamic-page was falsely accepted.");
  if (toolJobTrinityRewriteEnvelopeCallerRetainedExact(retainedJackStore, retainedJackCodec, retainedJackEditor, retainedTrinityRewrite.replace("fn runtime_handle(&self) -> ArtifactEnvelopeDecodeOperationHandle", "fn operation_only(&self) -> u64"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Trinity-Rewrite-generation-handle-erasure was falsely accepted.");
  if (toolJobTrinityRewriteEnvelopeCallerRetainedExact(retainedJackStore, retainedJackCodec, retainedJackEditor, retainedTrinityRewrite.replace("let mut bytes = [0; store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES]", "let mut bytes = Vec::new()"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Trinity-Rewrite-fixed-page-owner-removal was falsely accepted.");
  if (toolJobTrinityRewriteEnvelopeCallerRetainedExact(retainedJackStore, retainedJackCodec, retainedJackEditor, retainedTrinityRewrite.replace("pub struct TrinityRewriteEnvelopePageAdmission", "pub struct ErasedEnvelopePageAdmission"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Trinity-Rewrite-typed-rejected-page-result-erasure was falsely accepted.");
  if (toolJobTrinityRewriteEnvelopeCallerRetainedExact(retainedJackStore, retainedJackCodec, retainedJackEditor, retainedTrinityRewrite.replace("TrinityRewriteCallerPageOwner::new(owner)", "drop(owner)"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Trinity-Rewrite-rejected-page-ordinary-drop was falsely accepted.");
  if (toolJobTrinityRewriteEnvelopeCallerRetainedExact(retainedJackStore, retainedJackCodec, retainedJackEditor, retainedTrinityRewrite.replace("source.clone()", "Uint8Array::new(source)"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Trinity-Rewrite-rejected-page-byte-clone was falsely accepted.");
  if (
    toolJobTrinityRewriteEnvelopeCallerRetainedExact(
      retainedJackStore,
      retainedJackCodec,
      retainedJackEditor,
      retainedTrinityRewrite.replace(
        "construct_and_admit_artifact_envelope_ingress_page(handle.runtime_handle(), len\nlet mut bytes = [0; store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES]\nsource.copy_to(&mut bytes[..len])",
        "let mut bytes = [0; store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES]\nsource.copy_to(&mut bytes[..len])\nconstruct_and_admit_artifact_envelope_ingress_page(handle.runtime_handle(), len",
      ),
      retainedWriterPlugin,
    )
  )
    throw new Error("[verify interactivity tool-jobs] self-test Trinity-Rewrite-page-copy-before-preflight was falsely accepted.");
  if (toolJobTrinityRewriteEnvelopeCallerRetainedExact(retainedJackStore, retainedJackCodec, retainedJackEditor, retainedTrinityRewrite.replace("pub fn take_page(&mut self) -> Option<js_sys::Uint8Array>", "fn discard_page(&mut self)"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Trinity-Rewrite-rejected-page-retrieval-removal was falsely accepted.");
  if (toolJobTrinityRewriteEnvelopeCallerRetainedExact(retainedJackStore, retainedJackCodec, retainedJackEditor, retainedTrinityRewrite.replace("pub fn retry_envelope_page(", "fn drop_retry_owner("), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Trinity-Rewrite-rejected-page-retry-removal was falsely accepted.");
  if (toolJobTrinityRewriteEnvelopeCallerRetainedExact(retainedJackStore, retainedJackCodec, retainedJackEditor, retainedTrinityRewrite.replace("pub fn close_step(&mut self) -> bool", "fn drop_rejected_page(&mut self)"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Trinity-Rewrite-rejected-page-close-removal was falsely accepted.");
  if (toolJobTrinityRewriteEnvelopeCallerRetainedExact(retainedJackStore, retainedJackCodec, retainedJackEditor, retainedTrinityRewrite.replace("seal_artifact_envelope_ingress(handle.runtime_handle())", "submit_unsealed(handle.runtime_handle())"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Trinity-Rewrite-unsealed-ingress was falsely accepted.");
  if (toolJobTrinityRewriteEnvelopeCallerRetainedExact(retainedJackStore, retainedJackCodec, retainedJackEditor, retainedTrinityRewrite.replace("app.maintenance_step(1, store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)", "app.run_to_completion()"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Trinity-Rewrite-unbounded-poll-turn was falsely accepted.");
  if (toolJobTrinityRewriteEnvelopeCallerRetainedExact(retainedJackStore, retainedJackCodec, retainedJackEditor, retainedTrinityRewrite.replace("acknowledge_artifact_store_replacement(handle.runtime_handle())", "return Ready"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Trinity-Rewrite-terminal-without-exact-ack was falsely accepted.");
  if (toolJobTrinityRewriteEnvelopeCallerRetainedExact(retainedJackStore, retainedJackCodec, retainedJackEditor, retainedTrinityRewrite.replace("cancel_artifact_envelope_load(handle.runtime_handle())", "drop(handle)"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Trinity-Rewrite-cancel-owner-drop was falsely accepted.");
  if (toolJobTrinityRewriteEnvelopeCallerRetainedExact(retainedJackStore, retainedJackCodec, retainedJackEditor, retainedTrinityRewrite.replace("close_step(1, store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)", "close_all()"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Trinity-Rewrite-bulk-close was falsely accepted.");
  if (toolJobTrinityRewriteEnvelopeCallerRetainedExact(retainedJackStore, retainedJackCodec, retainedJackEditor, retainedTrinityRewrite.replace("trinity_rewrite_envelope_credits_admit_exact_caps_and_reject_zero_or_plus_one", "trinity_rewrite_envelope_accepts_unbounded_credits"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Trinity-Rewrite-cap-plus-one-fixture-missing was falsely accepted.");
  if (toolJobTrinityRewriteEnvelopeCallerRetainedExact(retainedJackStore, retainedJackCodec, retainedJackEditor, retainedTrinityRewrite, retainedWriterPlugin.replace("artifact_envelope_ingress_saturation_returns_exact_plus_one_owner_and_closes_fifo_slots", "artifact_envelope_ingress_drops_plus_one"))) throw new Error("[verify interactivity tool-jobs] self-test Trinity-Rewrite-fixed-registry-exact-handback-fixture-missing was falsely accepted.");
  if (toolJobTrinityRewriteEnvelopeCallerRetainedExact(retainedJackStore, retainedJackCodec, retainedJackEditor, retainedTrinityRewrite, retainedWriterPlugin.replace("artifact_envelope_ingress_cancel_and_interrupted_close_release_one_real_page_per_grant", "artifact_envelope_ingress_bulk_close"))) throw new Error("[verify interactivity tool-jobs] self-test Trinity-Rewrite-one-page-close-fixture-missing was falsely accepted.");
}

/** 🗺 Checks the gis map envelope caller retained route. */
export function gisMapEnvelopeCallerRetainedChecks({ retainedWriterWasm, retainedJackStore, retainedWriterPlugin }: {
  readonly retainedWriterWasm: string;
  readonly retainedJackStore: string;
  readonly retainedWriterPlugin: string;
}) {
  const retainedGisMapCodec = [
    "pub struct GisMapEnvelopeOwnedFieldCatalog",
    "artifact_owned_spr_edit_history_decoder",
    "GisMapSnapshotDecodeAuthority",
    "GisMapMutationDecodeAuthority",
    "OwnedSchemaHexAuthority<GIS_MAP_OWNED_FIELD_BYTES>",
    "struct GisMapSnapshotCloneAuthority",
    "struct GisMapStoreInitializationAuthority",
    "impl semio_framework_plugin::ArtifactStoreInitializationAuthority<GisMapSnapshot, GisMapMutation> for GisMapStoreInitializationAuthority",
    "GisMapStoreInitializationPhase::ValidateEdit { index }",
    "self.edit_index.admit(&envelope.vcs.edits, index,",
    "GisMapStoreInitializationPhase::SeedHistory",
    "GisMapStoreInitializationPhase::BuildCandidate",
    "ArtifactStoreInitializationRuntime::new",
    "ArtifactStore::from_initialized_runtime_with_owners",
    "self.generation.0.checked_add(1)",
    "value.positions.pop()",
    "value.routes.pop()",
    "value.regions.pop()",
    "Self::child_step(&mut value.drawing",
    "Self::child_step(image",
    "Self::child_step(&mut value.value",
    "dsl::DslValue::Array(values)",
    "dsl::DslValue::Object(values)",
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
    "GIS store initialization authority reached Drop before exact candidate handoff or retained rejection close",
    "gis_map_store_initializer_publishes_next_generation_and_candidate_closes_incrementally",
    "gis_map_store_initializer_cancel_and_stale_generation_return_every_owner_terminal_empty",
    "gis_map_nested_value_mutation_and_all_child_handles_retire_one_owner_per_grant",
    "gis_map_all_twelve_mutation_variants_preserve_catalog_order_and_zero_grant_ownership",
  ].join("\n");
  const retainedGisMapEditor = "fn build_document_store_initialization_job( gis_map_document_store_initialization_job(envelope, operation, generation) gis_map_live_envelope_submit_pump_swap_displaced_store_and_exact_ack_succeed gis_map_live_envelope_cancel_closes_retained_pages_without_publication duplicate GIS load acknowledgement is a no-op";
  const retainedGisMapWasm = retainedWriterWasm.replace("WriterEnvelopeLoadHandle", "GisMapEnvelopeLoadHandle");
  if (!toolJobGisMapEnvelopeCallerRetainedExact(retainedJackStore, retainedGisMapCodec, retainedGisMapEditor, retainedGisMapWasm, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test retained-GIS-Map-envelope-route was falsely rejected.");
  if (toolJobGisMapEnvelopeCallerRetainedExact(retainedJackStore, retainedGisMapCodec.replace("artifact_owned_spr_edit_history_decoder", "artifact_bounded_history_entry_decoder"), retainedGisMapEditor, retainedGisMapWasm, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test GIS-Map-domain-only-whole-edit-decoder was falsely accepted.");
  if (toolJobGisMapEnvelopeCallerRetainedExact(retainedJackStore, retainedGisMapCodec, retainedGisMapEditor, retainedGisMapWasm.replace("source: &js_sys::Uint8Array", "source: &[u8]"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test GIS-Map-post-lift-dynamic-page was falsely accepted.");
  if (toolJobGisMapEnvelopeCallerRetainedExact(retainedJackStore, retainedGisMapCodec, retainedGisMapEditor, retainedGisMapWasm.replace("acknowledge_artifact_store_replacement(handle.runtime_handle())", "return Ready"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test GIS-Map-completion-without-exact-ack was falsely accepted.");
  if (toolJobGisMapEnvelopeCallerRetainedExact(retainedJackStore, retainedGisMapCodec, retainedGisMapEditor, retainedGisMapWasm, retainedWriterPlugin.replace("self.jobs.insert_admitted(self.operation.0, ActiveArtifactStoreReplacement::new(self.operation, self.generation, job))", "drop(job)"))) throw new Error("[verify interactivity tool-jobs] self-test GIS-Map-false-terminal-initializer-drop was falsely accepted.");
  if (toolJobGisMapEnvelopeCallerRetainedExact(retainedJackStore, retainedGisMapCodec.replace("self.generation.0.checked_add(1)", "self.generation.0 + 1"), retainedGisMapEditor, retainedGisMapWasm, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test GIS-Map-unchecked-generation-publication was falsely accepted.");
  if (toolJobGisMapEnvelopeCallerRetainedExact(retainedJackStore, retainedGisMapCodec.replace("Self::child_step(&mut value.drawing", "drop(value.drawing)"), retainedGisMapEditor, retainedGisMapWasm, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test GIS-Map-drawing-child-deep-drop was falsely accepted.");
  if (toolJobGisMapEnvelopeCallerRetainedExact(retainedJackStore, retainedGisMapCodec.replace("dsl::DslValue::Object(values)", "drop(value)"), retainedGisMapEditor, retainedGisMapWasm, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test GIS-Map-nested-value-deep-drop was falsely accepted.");
  if (toolJobGisMapEnvelopeCallerRetainedExact(retainedJackStore, retainedGisMapCodec.replace("ReplaceRegionData(payload)", "drop(payload)"), retainedGisMapEditor, retainedGisMapWasm, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test GIS-Map-mutation-catalog-hole was falsely accepted.");
  if (toolJobGisMapEnvelopeCallerRetainedExact(retainedJackStore, retainedGisMapCodec.replace("gis_map_all_twelve_mutation_variants_preserve_catalog_order_and_zero_grant_ownership", "gis_map_partial_mutation_catalog"), retainedGisMapEditor, retainedGisMapWasm, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test GIS-Map-zero-grant-catalog-fixture-missing was falsely accepted.");
  if (toolJobGisMapEnvelopeCallerRetainedExact(retainedJackStore, retainedGisMapCodec, retainedGisMapEditor, `${retainedGisMapWasm}\nenvelope_json: &str`, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test GIS-Map-whole-buffer-ingress-bypass was falsely accepted.");
}
