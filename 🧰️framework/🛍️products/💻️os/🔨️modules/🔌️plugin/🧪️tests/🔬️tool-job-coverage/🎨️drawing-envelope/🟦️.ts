/** 🎨️ The drawing envelope caller retained sources and their route, semantic digest and mutation checks. */
import { toolJobDrawingEnvelopeCallerRetainedExact } from "../../../../../../🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🗄️artifact-store/🖼️caller-retained-media/🟦️.ts";

/** 🏗 Builds the drawing codec and editor sources. */
export function drawingEnvelopeSourceFixtures() {
  const retainedDrawingCodec = [
    "pub struct DrawingEnvelopeOwnedFieldCatalog",
    "artifact_owned_spr_edit_history_decoder",
    "DrawingSnapshotDecodeAuthority",
    "DrawingMutationDecodeAuthority",
    "OwnedSchemaHexAuthority<DRAWING_OWNED_FIELD_BYTES>",
    "const DRAWING_MAXIMUM_NESTED_ITEMS: usize = 4_096",
    "const DRAWING_MAXIMUM_LAYER_DEPTH: usize = 64",
    "struct DrawingSnapshotBoundsAuthority",
    "struct DrawingFixedOwnerCensus",
    "slots: [DrawingOwnerCreditSlot; DRAWING_MAXIMUM_NESTED_ITEMS]",
    "const DRAWING_MUTATION_OVERLAY_PAGE_CAPACITY: usize = 16",
    "const DRAWING_MUTATION_CONTAINER_SLOT_CAPACITY: usize = 64",
    "const DRAWING_MUTATION_ARENA_POOL_CAPACITY: usize = 4",
    "struct DrawingMutationArenaOwner",
    "struct DrawingMutationArenaPool {",
    "DrawingMutationArenaProcessState::Building",
    "#[cfg(test)]\n    fn try_new()",
    "struct DrawingMutationArenaOwnerBuilder",
    "struct DrawingMutationArenaPoolBootstrap",
    "struct DrawingMutationArenaBootstrapJob",
    "fn step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> DrawingMutationArenaBootstrapStep",
    "DrawingMutationArenaProcessState::Inert",
    "DrawingMutationArenaBootstrapAdmission::fixed()",
    "DrawingMutationArenaBootstrapStep::Blocked",
    "pub fn request_drawing_mutation_arena_pool() {",
    "DRAWING_MUTATION_ARENA_BOOTSTRAP_REQUESTED.store(true",
    "DrawingMutationArenaProcessState::Inert",
    "}",
    "pub fn drawing_mutation_arena_pool_fault",
    "fn borrow_drawing_mutation_arena() {",
    "request_drawing_mutation_arena_pool()",
    "DrawingMutationArenaBorrowError::NotReady",
    "}",
    "//#endregion",
    "DrawingStoreInitializationPhase::InitializeArena",
    "DrawingStoreInitializationPhase::InitializeArena => {",
    "match self.arena_bootstrap_job.step(cx)",
    "if let Some(undo) = self.source_undo {",
    "self.start_rebuild(source, undo.parent, None, Some(undo.index), DrawingContainerRebuildRole::CloseSourceUndo)?;",
    "DrawingMutationArenaProcessTransition::Retire",
    "drawing_mutation_arena_pool_fault",
    "borrow_drawing_mutation_arena_from",
    "drawing-store.mutation-arena-pool-saturated",
    "drawing-store.mutation-arena-stale-generation",
    "owner.terminal_is_empty()",
    "arena_return_phase: u8",
    "Ok(Some(false)) => return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })",
    "reverse.try_reserve_exact(DRAWING_MUTATION_CONTAINER_SLOT_CAPACITY)",
    "pages.try_reserve_exact(DRAWING_MUTATION_OVERLAY_PAGE_CAPACITY)",
    "page.try_reserve_exact(DRAWING_MUTATION_RETAINED_PAGE_BYTES)",
    "total.checked_add(page.capacity())",
    "fn write_overlay_string",
    "target.capacity() < source.len()",
    "struct DrawingMutationOverlayPatch",
    "drawing-store.mutation-overlay-owner-changed",
    "struct DrawingAssetBoundsCursor",
    "assets.range::<str, _>",
    "drawing-store.preflight-depth-capacity",
    "drawing-store.preflight-item-capacity",
    "drawing-store.preflight-byte-capacity",
    "struct DrawingLayerCloneAuthority",
    "self.bounds.step(source, cx)",
    "layer.step(source.layers.get(self.index)",
    "target.children.push(Self::skeleton(child)?)",
    "struct DrawingStoreInitializationAuthority",
    "struct DrawingSemanticDigestCredit",
    "semantic: Option<semio_framework_hash::Sha256>",
    'digest.observe(b"drawing.semantic.sha256")',
    "struct DrawingFillDigestAuthority",
    "struct DrawingStrokeDigestAuthority",
    "struct DrawingPathSegmentDigestAuthority",
    "struct DrawingLayerVariantDigestAuthority",
    "struct DrawingLayerDigestAuthority",
    "struct DrawingMutationDigestAuthority",
    "struct DrawingMutationAggregateReservation",
    "mutation_source_items",
    "mutation_derived_items",
    "duplicate_candidate_items",
    "source_owner_bytes",
    "derived_owner_bytes",
    "DRAWING_MUTATION_AGGREGATE_ITEMS",
    "DRAWING_MUTATION_AGGREGATE_BYTES",
    "DRAWING_MUTATION_RETAINED_PAGE_ITEMS",
    "DRAWING_MUTATION_RETAINED_PAGE_BYTES",
    "drawing-store.mutation-aggregate-item-capacity",
    "drawing-store.mutation-aggregate-byte-capacity",
    "struct DrawingMutationCandidateAuthority",
    "drawing-store.mutation-candidate-stale-authority",
    "drawing-store.mutation-candidate-cancelled",
    "struct DrawingDuplicateRewriteAuthority",
    "material: [u8; DRAWING_DUPLICATE_MATERIAL_BYTES]",
    "self.hash_cursor + DRAWING_OWNED_FIELD_BYTES",
    'update(b"semio.drawing.duplicate-id.v1")',
    "update(&(self.id_len as u64).to_be_bytes())",
    "update(&(self.name_len as u64).to_be_bytes())",
    "pending_name: std::mem::ManuallyDrop<Option<String>>",
    "drawing-store.duplicate-destination-capacity",
    "DrawingMutationCandidatePhase::PreflightSource",
    "DrawingMutationCandidatePhase::PreflightMutation",
    "struct DrawingContainerRebuildAuthority",
    "const DRAWING_CONTAINER_REBUILD_MOVE_CAPACITY",
    "enum DrawingContainerRebuildMove",
    "rebuild.rollback_step()?",
    "DrawingContainerRebuildRole::CloseSourceUndo",
    "source_undo: Option<DrawingContainerSourceUndo>",
    "source.capacity() < output_capacity",
    'self.source.as_mut().ok_or("drawing-store.container-source")?.push(value)',
    "DrawingStoreInitializationPhase::ValidateEditId",
    "DrawingStoreInitializationPhase::ValidateEditMeta",
    "DrawingStoreInitializationPhase::PrepareApplied",
    "DrawingStoreInitializationPhase::PrepareRedo",
    "impl semio_framework_plugin::ArtifactStoreInitializationAuthority<DrawingSnapshot, DrawingMutation> for DrawingStoreInitializationAuthority",
    "DrawingStoreInitializationPhase::ValidateEdit { index }",
    "self.edit_index.admit(&envelope.vcs.edits, index,",
    "DrawingStoreInitializationPhase::SeedHistory",
    "DrawingStoreInitializationPhase::BuildCandidate",
    "ArtifactStore::from_initialized_runtime_with_owners",
    "ArtifactStoreInitializationOwnerCatalog::try_new()",
    "ArtifactStoreInitializationRuntime::new_with_owner_catalog",
    "DrawingStoreInitializationPhase::MoveInitialOwner",
    "self.generation.0.checked_add(1)",
    "DrawingRetirementOwner::Layer(value)",
    "DrawingRetirementOwner::Fill(fill)",
    "DrawingRetirementOwner::Stroke(stroke)",
    "DrawingRetirementOwner::Segments(values)",
    "DrawingRetirementOwner::Stops(values)",
    "DrawingRetirementOwner::Points(values)",
    "DrawingRetirementOwner::AssetEntry",
    "FillStyle::Solid { color }",
    "FillStyle::LinearGradient { x1, y1, x2, y2, stops }",
    "FillStyle::RadialGradient { cx: center_x, cy: center_y, r, stops }",
    "stop.offset",
    "stop.color",
    "value.color[(self.phase - 1) as usize]",
    "value.width",
    "observe_owned_string(digest, 246, &value.cap",
    "observe_owned_string(digest, 247, &value.join",
    "value.dash.is_some()",
    "base.visible",
    "base.locked",
    "base.opacity",
    "observe_owned_string(digest, 106, &base.blend_mode",
    "base.transform.rotation",
    "observe_owned_string(digest, 340, &value.shape_kind",
    "value.rect.is_some()",
    "value.ellipse.is_some()",
    "value.circle.is_some()",
    "value.line.is_some()",
    "value.polygon.is_some()",
    "PathSegment::Move",
    "PathSegment::Line",
    "PathSegment::Quad",
    "PathSegment::Cubic",
    "PathSegment::Arc",
    "PathSegment::Close",
    "observe_owned_string(digest, 382, &value.content",
    "observe_owned_string(digest, 390, &value.image_key",
    "observe_owned_string(digest, 410, &value.operation",
    "value.children.len()",
    "observe_owned_string(digest, 420, &value.source_key",
    "value.params.threshold",
    "Self::variant(mutation)",
    "value.visible",
    "value.locked",
    "value.opacity",
    "observe_owned_string(digest, 3, &value.blend_mode",
    "observe_owned_string(digest, 3, &value.new_name",
    "value.transform.rotation",
    "value.fill.as_ref()",
    "value.stroke.as_ref()",
    "observe_owned_string(digest, 3, &value.boolean_operation",
    "value.params.simplify_epsilon",
    "value.parent_id.is_some()",
    "value.index",
    "value.layer",
    "SetLayerVisible(payload)",
    "SetLayerLocked(payload)",
    "SetLayerOpacity(payload)",
    "SetLayerBlendMode(payload)",
    "RenameLayer(payload)",
    "UpdateLayerTransform(payload)",
    "ReplaceLayerFill(payload)",
    "ReplaceLayerStroke(payload)",
    "SetLayerBooleanOperation(payload)",
    "UpdateLayerTraceParams(payload)",
    "CreateLayer(payload)",
    "DuplicateLayer(payload)",
    "DeleteLayer(payload)",
    "ReorderLayer(payload)",
    "retained_drawing_mutation_candidate_covers_all_fourteen_variants_and_returns_exact_owners",
    "retained_drawing_depth_plus_one_and_hostile_fields_fault_then_close_terminal_empty",
    "retained_drawing_container_false_terminal_saturation_and_interrupted_close_preserve_exact_owner",
    "retained_drawing_schema_digest_distinguishes_every_nested_semantic_field",
    "retained_drawing_aggregate_credit_admits_exact_4096_rejects_plus_one_with_owner_handback",
    "retained_drawing_duplicate_hash_frames_domain_id_and_name_lengths_without_concatenation_collision",
    "retained_drawing_process_arena_pool_cap_plus_one_returns_exact_slots_and_rejects_stale_aba",
    "retained_drawing_duplicate_name_uses_preadmitted_page_and_returns_exact_rejection_owner",
    "retained_drawing_cancel_stale_each_replay_candidate_container_stage_preserves_last_valid",
    "retained_drawing_rebuild_fault_after_every_phase_rolls_back_exact_container_and_reuses_pool_slot",
    "retained_drawing_reorder_fault_after_source_handoff_restores_exact_nested_fifo_and_pool_roots",
    "retained_drawing_arena_bootstrap_failure_at_each_allocation_retires_one_exact_root_per_grant",
    "retained_drawing_arena_bootstrap_failure_after_each_bundle_keeps_every_root_until_terminal_close",
    "retained_drawing_arena_bootstrap_advances_one_allocation_per_turn_and_withholds_incomplete_pool",
    "retained_drawing_arena_bootstrap_exact_cap_and_plus_one_rejection_preserve_every_owner_until_close",
    "retained_drawing_arena_default_second_app_and_borrow_only_request_without_allocation",
    "retained_drawing_arena_bootstrap_job_cancel_budget_contention_and_saturation_are_governed",
  ].join("\n");
  const retainedDrawingEditor = [
    "drawing_document_store_initialization_job(envelope, operation, generation)",
    "impl Default for DrawingPlayApp {",
    "request_drawing_mutation_arena_pool()",
    "}",
    "impl ArtifactEditor for DrawingPlayApp",
    "arena_boot_fault: Option<&'static str>",
    "drawing_mutation_arena_pool_fault",
    "drawing_live_envelope_submit_recursive_clone_swap_displaced_store_and_exact_ack_succeed",
    "drawing_live_envelope_cancel_closes_retained_pages_without_publication",
    "drawing_live_envelope_rejects_single_and_final_edit_id_plus_one_before_mutation_candidate",
    "drawing_live_initializer_candidate_container_commit_ack_cancel_stale_preserve_last_valid_and_exact_handle",
    '"forwards": [crate::artifacts::drawing::mutations::DrawingMutation::RenameLayer',
    "pub struct DrawingEnvelopeLoadHandle",
    "source: &js_sys::Uint8Array",
    "begin_artifact_envelope_ingress(maximum_pages, maximum_bytes)",
    "admit_artifact_envelope_ingress_page(handle.runtime_handle(), page)",
    "seal_artifact_envelope_ingress(handle.runtime_handle())",
    "app.maintenance_step(1, store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)",
    "advance_artifact_envelope_load(handle.runtime_handle())",
    "acknowledge_artifact_store_replacement(handle.runtime_handle())",
    "cancel_artifact_envelope_load(handle.runtime_handle())",
  ].join("\n");
  return { retainedDrawingCodec, retainedDrawingEditor };
}

/** 🔍 Checks the drawing envelope route, the production editor and the semantic digest field omissions. */
export function drawingEnvelopeRouteAndSemanticChecks({ retainedJackStore, retainedDrawingCodec, retainedDrawingEditor, retainedWriterPlugin }: {
  readonly retainedJackStore: string;
  readonly retainedDrawingCodec: string;
  readonly retainedDrawingEditor: string;
  readonly retainedWriterPlugin: string;
}) {
  if (!toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec, retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test retained-Drawing-envelope-route was falsely rejected.");
  const retainedDrawingProductionEditor = retainedDrawingEditor
    .replace("drawing_live_envelope_submit_recursive_clone_swap_displaced_store_and_exact_ack_succeed", "")
    .replace("drawing_live_envelope_cancel_closes_retained_pages_without_publication", "")
    .replace("drawing_live_envelope_rejects_single_and_final_edit_id_plus_one_before_mutation_candidate", "")
    .replace("drawing_live_initializer_candidate_container_commit_ack_cancel_stale_preserve_last_valid_and_exact_handle", "")
    .replace('"forwards": [crate::artifacts::drawing::mutations::DrawingMutation::RenameLayer', "");
  if (!toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec, retainedDrawingProductionEditor, retainedWriterPlugin, retainedDrawingEditor)) {
    throw new Error("[verify interactivity tool-jobs] self-test production-stripped Drawing editor with raw cfg(test) law witnesses was falsely rejected.");
  }
  const retainedDrawingSemanticFields = [
    "FillStyle::Solid { color }",
    "FillStyle::LinearGradient { x1, y1, x2, y2, stops }",
    "FillStyle::RadialGradient { cx: center_x, cy: center_y, r, stops }",
    "stop.offset",
    "stop.color",
    "value.color[(self.phase - 1) as usize]",
    "value.width",
    "observe_owned_string(digest, 246, &value.cap",
    "observe_owned_string(digest, 247, &value.join",
    "value.dash.is_some()",
    "base.visible",
    "base.locked",
    "base.opacity",
    "observe_owned_string(digest, 106, &base.blend_mode",
    "base.transform.rotation",
    "observe_owned_string(digest, 340, &value.shape_kind",
    "value.rect.is_some()",
    "value.ellipse.is_some()",
    "value.circle.is_some()",
    "value.line.is_some()",
    "value.polygon.is_some()",
    "PathSegment::Move",
    "PathSegment::Line",
    "PathSegment::Quad",
    "PathSegment::Cubic",
    "PathSegment::Arc",
    "PathSegment::Close",
    "observe_owned_string(digest, 382, &value.content",
    "observe_owned_string(digest, 390, &value.image_key",
    "observe_owned_string(digest, 410, &value.operation",
    "value.children.len()",
    "observe_owned_string(digest, 420, &value.source_key",
    "value.params.threshold",
    "Self::variant(mutation)",
    "value.visible",
    "value.locked",
    "value.opacity",
    "observe_owned_string(digest, 3, &value.blend_mode",
    "observe_owned_string(digest, 3, &value.new_name",
    "value.transform.rotation",
    "value.fill.as_ref()",
    "value.stroke.as_ref()",
    "observe_owned_string(digest, 3, &value.boolean_operation",
    "value.params.simplify_epsilon",
    "value.parent_id.is_some()",
    "value.index",
    "value.layer",
  ];
  for (const field of retainedDrawingSemanticFields) {
    if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace(field, "omitted-semantic-field"), retainedDrawingEditor, retainedWriterPlugin)) {
      throw new Error(`[verify interactivity tool-jobs] self-test Drawing-semantic-digest-omission-${field} was falsely accepted.`);
    }
  }
}

/** 🧪 Checks the drawing single-edit mutations of the codec, the editor and the plugin sources. */
export function drawingEnvelopeMutationChecks({ retainedJackStore, retainedDrawingCodec, retainedDrawingEditor, retainedWriterPlugin }: {
  readonly retainedJackStore: string;
  readonly retainedDrawingCodec: string;
  readonly retainedDrawingEditor: string;
  readonly retainedWriterPlugin: string;
}) {
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("semantic: Option<semio_framework_hash::Sha256>", "semantic: u64"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-non-collision-resistant-digest was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace('digest.observe(b"drawing.semantic.sha256")', "digest.observe(&field)"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-unsealed-semantic-digest was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("struct DrawingMutationAggregateReservation", "struct DrawingPerAllocationGuess"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-missing-aggregate-reservation was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("struct DrawingFixedOwnerCensus", "struct DrawingResizableOwnerGuess"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-fixed-owner-census-missing was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("const DRAWING_MUTATION_OVERLAY_PAGE_CAPACITY: usize = 16", "let overlay_pages = Vec::new()"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-fixed-overlay-page-arena-missing was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("page.try_reserve_exact(DRAWING_MUTATION_RETAINED_PAGE_BYTES)", "page.reserve(source.len())"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-overlay-page-not-preadmitted was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("total.checked_add(page.capacity())", "total.checked_add(page.len())"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-overlay-allocator-capacity-not-reconciled was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("fn write_overlay_string", "fn clone_overlay_string"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-overlay-owner-not-moved was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("reverse.try_reserve_exact(DRAWING_MUTATION_CONTAINER_SLOT_CAPACITY)", "let reverse = Vec::new()"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-fixed-container-owner-catalog-missing was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("struct DrawingMutationArenaPool {", "struct PerCandidateArenaAllocation {"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-process-arena-pool-missing was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("DrawingMutationArenaProcessState::Building", "DrawingMutationArenaOwner::try_new()?"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-arena-allocation-before-process-admission was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, `${retainedDrawingCodec}\nDrawingMutationArenaOwner::try_new()?;`, retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-question-mark-partial-bootstrap-drop was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("#[cfg(test)]\n    fn try_new()", "fn try_new()"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-synchronous-bootstrap-loop-was-production-reachable was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("borrow_drawing_mutation_arena_from", "allocate_drawing_mutation_arena_per_candidate"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-candidate-did-not-borrow-fixed-arena was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("owner.terminal_is_empty()", "true"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-nonterminal-arena-return-was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("arena_return_phase: u8", "return_whole_arena_bundle: bool"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-arena-return-was-not-one-root-per-grant was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("drawing-store.mutation-arena-stale-generation", "accept-stale-arena-return"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-arena-ABA-generation-check-missing was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, `${retainedDrawingCodec}\nbase.name.try_reserve_exact(suffix.len());`, retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-post-admission-duplicate-name-reserve was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("pending_name: std::mem::ManuallyDrop<Option<String>>", "pending_name: String"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-duplicate-name-owner-not-retained was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("source.capacity() < output_capacity", "false"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-live-container-owner-capacity-not-preflighted was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace('self.source.as_mut().ok_or("drawing-store.container-source")?.push(value)', 'self.output.as_mut().unwrap().push(value)'), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-container-scratch-owner-published-instead-of-returned was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("retained_drawing_process_arena_pool_cap_plus_one_returns_exact_slots_and_rejects_stale_aba", "drawing-string-only-pool-predicate"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-live-process-pool-handback-fixture-missing was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("retained_drawing_duplicate_name_uses_preadmitted_page_and_returns_exact_rejection_owner", "drawing-string-only-name-predicate"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-live-duplicate-name-owner-fixture-missing was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec, retainedDrawingEditor.replace("request_drawing_mutation_arena_pool()", "initialize_drawing_mutation_arena_pool()"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-app-default-advanced-bootstrap was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("DRAWING_MUTATION_ARENA_BOOTSTRAP_REQUESTED.store(true", "Vec::<u8>::new().try_reserve_exact(64); DRAWING_MUTATION_ARENA_BOOTSTRAP_REQUESTED.store(true"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-request-path-allocated-before-governed-turn was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("fn borrow_drawing_mutation_arena() {\nrequest_drawing_mutation_arena_pool()", "fn borrow_drawing_mutation_arena() {\nbootstrap.step(cx); request_drawing_mutation_arena_pool()"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-borrow-path-advanced-bootstrap was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("struct DrawingMutationOverlayPatch", "struct DrawingWholeCandidateRebuild"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-retained-overlay-patch-missing was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("mutation_derived_items", "semantic_totals_times_copies"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-derived-owner-credit-missing was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, `${retainedDrawingCodec}\nfn exact_for_test() {}`, retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-fabricated-boundary-authority was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, `${retainedDrawingCodec}\nsource.assets.iter().nth(index)`, retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-asset-rescan-cursor was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore.replace("id_digest: [u8; 32]", "id: String"), retainedDrawingCodec, retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-shared-history-id-clone-owner was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("struct DrawingDuplicateRewriteAuthority", "fn rewrite_duplicate_in_one_step"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-staged-duplicate-authority-missing was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace('update(b"semio.drawing.duplicate-id.v1")', "update(&material_len.to_be_bytes())"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-duplicate-domain-frame-missing was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("update(&(self.id_len as u64).to_be_bytes())", "update(&(self.material_len as u64).to_be_bytes())"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-duplicate-id-length-frame-missing was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("update(&(self.name_len as u64).to_be_bytes())", "update(&[])"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-duplicate-name-length-frame-missing was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("drawing-store.mutation-candidate-cancelled", "close-only-cancel"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-live-candidate-cancellation-missing was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("drawing-store.mutation-aggregate-item-capacity", "unchecked-aggregate-items"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-aggregate-item-plus-one was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("drawing-store.mutation-aggregate-byte-capacity", "unchecked-aggregate-bytes"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-aggregate-byte-plus-one was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("DRAWING_MUTATION_RETAINED_PAGE_ITEMS", "uncredited-retained-pages"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-retained-page-item-credit-missing was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("DRAWING_MUTATION_RETAINED_PAGE_BYTES", "uncredited-retained-page-bytes"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-retained-page-byte-credit-missing was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("DrawingMutationCandidatePhase::PreflightSource", "DrawingMutationCandidatePhase::Clone"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-clone-before-source-preflight was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("DrawingMutationCandidatePhase::PreflightMutation", "DrawingMutationCandidatePhase::Clone"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-clone-before-mutation-preflight was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("DrawingStoreInitializationPhase::ValidateEditMeta", "mutation_meta.iter().any"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-whole-metadata-scan was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("DrawingStoreInitializationPhase::PrepareApplied", "DrawingStoreInitializationPhase::CommitApplied"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-applied-id-actor-multi-clone-grant was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("DrawingStoreInitializationPhase::PrepareRedo", "DrawingStoreInitializationPhase::CommitRedo"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-redo-id-multi-clone-grant was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("retained_drawing_schema_digest_distinguishes_every_nested_semantic_field", "drawing-string-name-predicate"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-semantic-authority-fixture-missing was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("retained_drawing_aggregate_credit_admits_exact_4096_rejects_plus_one_with_owner_handback", "drawing-aggregate-string-predicate"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-real-aggregate-saturation-fixture-missing was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("retained_drawing_duplicate_hash_frames_domain_id_and_name_lengths_without_concatenation_collision", "drawing-duplicate-name-only-predicate"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-duplicate-split-boundary-fixture-missing was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("retained_drawing_cancel_stale_each_replay_candidate_container_stage_preserves_last_valid", "drawing-only-cancels-before-start"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-stage-cancel-stale-fixture-missing was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("rebuild.rollback_step()?", "rebuild.close_forward_step()?"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-active-rebuild-close-did-not-rollback was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("self.start_rebuild(source, undo.parent, None, Some(undo.index), DrawingContainerRebuildRole::CloseSourceUndo)?;", "self.start_rebuild(source, undo.parent, None, Some(undo.index), DrawingContainerRebuildRole::Destination)?;"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-live-source-undo-close-authority-missing was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("retained_drawing_rebuild_fault_after_every_phase_rolls_back_exact_container_and_reuses_pool_slot", "drawing-only-cancels-before-owner-move"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-every-rebuild-move-fault-fixture-missing was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("retained_drawing_reorder_fault_after_source_handoff_restores_exact_nested_fifo_and_pool_roots", "drawing-does-not-restore-source-handoff"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-reorder-source-undo-fixture-missing was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("struct DrawingMutationArenaOwnerBuilder", "fn try_new_arena_owner()"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-retained-bootstrap-owner-builder-missing was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("struct DrawingMutationArenaPoolBootstrap", "fn build_pool_with_question_mark()"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-retained-bootstrap-pool-owner-missing was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("match self.arena_bootstrap_job.step(cx)", "match Ok::<bool, &'static str>(true)"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-live-bootstrap-not-driven-by-governed-store-turn was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("retained_drawing_arena_bootstrap_failure_at_each_allocation_retires_one_exact_root_per_grant", "drawing-bootstrap-only-tests-success"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-bootstrap-allocation-fault-matrix-missing was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("retained_drawing_arena_bootstrap_failure_after_each_bundle_keeps_every_root_until_terminal_close", "drawing-bootstrap-drops-completed-bundles"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-bootstrap-bundle-fault-matrix-missing was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("retained_drawing_arena_bootstrap_advances_one_allocation_per_turn_and_withholds_incomplete_pool", "drawing-bootstrap-loops-to-completion"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-bootstrap-governed-progress-fixture-missing was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("retained_drawing_arena_bootstrap_exact_cap_and_plus_one_rejection_preserve_every_owner_until_close", "drawing-bootstrap-skips-aggregate-plus-one"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-bootstrap-aggregate-rejection-fixture-missing was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec, retainedDrawingEditor.replace("drawing_mutation_arena_pool_fault", "ignore_arena_boot_fault"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-app-bootstrap-fault-was-not-surfaced was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("retained_drawing_arena_default_second_app_and_borrow_only_request_without_allocation", "drawing-default-allocates-on-second-app"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-default-borrow-no-allocation-fixture-missing was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("retained_drawing_arena_bootstrap_job_cancel_budget_contention_and_saturation_are_governed", "drawing-bootstrap-ignores-governed-cancellation"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-governed-bootstrap-adversarial-fixture-missing was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, `${retainedDrawingCodec}\nmutation_meta.iter().any(|meta| true)`, retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-resurrected-whole-metadata-scan was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("artifact_owned_spr_edit_history_decoder", "artifact_bounded_history_entry_decoder"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-domain-only-whole-edit-decoder was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("const DRAWING_MAXIMUM_LAYER_DEPTH: usize = 64", "let depth = Vec::new()"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-resizable-recursion-stack was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("drawing-store.preflight-byte-capacity", "unchecked_nested_bytes"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-nested-byte-preflight-hole was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("target.children.push(Self::skeleton(child)?)", "target.children.push(child.clone())"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-whole-recursive-layer-clone was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("DrawingRetirementOwner::AssetEntry", "drop(asset)"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-asset-owner-drop was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("ReorderLayer(payload)", "drop(payload)"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-mutation-catalog-hole was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, `${retainedDrawingCodec}\noperation.encode_op()`, retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-whole-operation-encode-before-credit was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, `${retainedDrawingCodec}\nlet next = snapshot.clone();`, retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-whole-snapshot-clone-replay was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, `${retainedDrawingCodec}\nserde_json::from_value(value)`, retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-serde-mutation-reconstruction was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("DrawingStoreInitializationPhase::ValidateEditId", "skip_final_edit_id"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-single-final-edit-id-preflight-hole was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("ArtifactStoreInitializationOwnerCatalog::try_new()", "ArtifactStoreInitializationRuntime::new()"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-store-owner-catalog-not-preadmitted was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("DrawingStoreInitializationPhase::MoveInitialOwner", "DrawingStoreInitializationPhase::CloneInitial"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-initial-owner-clone-resurrected was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("struct DrawingMutationCandidateAuthority", "struct SynchronousDrawDiffApply"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-retained-mutation-candidate-missing was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace("retained_drawing_container_false_terminal_saturation_and_interrupted_close_preserve_exact_owner", "drawing_string_predicate_only"), retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-authority-close-fixture-missing was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec, retainedDrawingEditor.replace("drawing_live_envelope_rejects_single_and_final_edit_id_plus_one_before_mutation_candidate", "drawing_skips_final_edit_id"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-live-final-edit-id-fixture-missing was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec, retainedDrawingEditor.replace("drawing_live_initializer_candidate_container_commit_ack_cancel_stale_preserve_last_valid_and_exact_handle", "drawing-close-only-after-success"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-live-staged-cancel-stale-ack-fixture-missing was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec, retainedDrawingEditor.replace('"forwards": [crate::artifacts::drawing::mutations::DrawingMutation::RenameLayer', '"forwards": []'), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-live-populated-history-route-missing was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec, retainedDrawingEditor.replace("source: &js_sys::Uint8Array", "source: &[u8]"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-post-lift-dynamic-page was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec, retainedDrawingEditor.replace("acknowledge_artifact_store_replacement(handle.runtime_handle())", "return Ready"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-completion-without-exact-ack was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec, `${retainedDrawingEditor}\nenvelope_json: Option<String>`, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-whole-buffer-constructor-bypass was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec, retainedDrawingEditor, retainedWriterPlugin.replace("self.jobs.insert_admitted(self.operation.0, ActiveArtifactStoreReplacement::new(self.operation, self.generation, job))", "drop(job)"))) throw new Error("[verify interactivity tool-jobs] self-test Drawing-false-terminal-initializer-drop was falsely accepted.");
}
