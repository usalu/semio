/** 🖼️ The raster envelope caller retained sources and their route, mutation, loop restoration and rejected output identity checks. */
import { toolJobRasterEnvelopeCallerRetainedExact } from "../../../../../../🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🗄️artifact-store/🖼️caller-retained-media/🟦️.ts";

/** 🧬 Builds the raster codec and editor sources. */
export function rasterEnvelopeSourceFixtures() {
  const retainedRasterCodec = [
    "pub struct RasterEnvelopeOwnedFieldCatalog",
    "artifact_owned_spr_edit_history_decoder",
    "RasterSnapshotDecodeAuthority",
    "RasterMutationDecodeAuthority",
    "OwnedSchemaHexAuthority<RASTER_OWNED_FIELD_BYTES>",
    "struct RasterSnapshotBoundsAuthority",
    "struct RasterSnapshotCloneAuthority",
    "struct RasterMutationCandidateAuthority",
    "RASTER_RETIREMENT_STACK_CAPACITY",
    "pages: std::mem::ManuallyDrop<[Option<Box<RasterRetirementFramePage>>; RASTER_RETIREMENT_STACK_PAGE_COUNT]>",
    "pending_push: std::mem::ManuallyDrop<Option<RasterRetirementOwner>>",
    "struct RasterStoreInitializationAuthority",
    "impl semio_framework_plugin::ArtifactStoreInitializationAuthority<RasterSnapshot, RasterMutation> for RasterStoreInitializationAuthority",
    "RasterStoreInitializationPhase::ValidateEdit { index }",
    "self.edit_index.admit(&envelope.vcs.edits, index,",
    "RasterStoreInitializationPhase::SeedHistory",
    "RasterStoreInitializationPhase::BuildCandidate",
    "if cx.should_yield()",
    "if cx.should_yield()",
    "if cx.should_yield()",
    "fn raster_reserve_unit(cx: &mut semio_framework_job::StepContext<'_>) -> bool",
    "cx.consume_fuel(1)",
    "self.cancel_requested || cx.is_cancelled()",
    "ArtifactStoreInitializationRuntime::new",
    "ArtifactStore::from_initialized_runtime_with_owners",
    "self.generation.0.checked_add(1)",
    "value.layers.pop()",
    "value.assets.take_last_entry()",
    "dsl::DslValue::Array(values)",
    "dsl::DslValue::Object(values)",
    "CreateLayer(payload)",
    "DeleteLayer(payload)",
    "ReorderLayers(payload)",
    "RenameLayer(payload)",
    "ChangeLayerVisible(payload)",
    "ChangeLayerOpacity(payload)",
    "ChangeLayerBlendMode(payload)",
    "MoveLayer(payload)",
    "ResizeLayer(payload)",
    "ChangeLayerAdjustmentKind(payload)",
    "AddLayerAsset(payload)",
    "RemoveLayerAsset(payload)",
    "Raster store initialization authority reached Drop before exact candidate handoff or retained rejection close",
    "raster_store_initializer_publishes_next_generation_and_candidate_closes_incrementally",
    "raster_store_initializer_cancel_and_stale_generation_return_every_owner_terminal_empty",
    "raster_store_initializer_zero_budget_advances_no_owner_or_phase",
    "raster_nested_snapshot_and_child_handles_retire_one_owner_per_grant",
    "raster_owner_caps_and_all_mutation_variants_retire_one_owner_per_grant",
    "raster_envelope_caps_and_plus_one_page_return_the_exact_fixed_owner",
    "raster_snapshot_bounds_and_clone_advance_one_pre_admitted_unit_with_low_nonzero_fuel",
    "raster_expired_deadline_advances_no_bounds_clone_or_mutation_owner",
    "raster_small_mutation_against_deep_snapshot_is_cursorized_and_atomic",
    "raster_cancel_after_complete_retires_the_unclaimed_candidate_before_terminal",
    "raster_retirement_uses_allocation_capacity_and_fixed_iterative_depth",
    "raster_nested_owner_item_and_byte_capacity_plus_one_reject_before_clone",
    "raster_empty_bounds_and_mounted_sixty_four_fuel_progress_across_second_map_page",
    "raster_retirement_page_credit_is_claimed_before_allocation_and_returned_with_backing",
    "raster_owned_map_removal_returns_exact_pair_and_populated_drop_refuses",
    "const RASTER_MAXIMUM_CONTROL_BYTES: usize = RASTER_MAXIMUM_CONTROL_BACKINGS * RASTER_CONTROL_BACKING_BYTES",
    "source_control_bytes: usize",
    "self.source_control_bytes = RASTER_MAXIMUM_CONTROL_BYTES",
    "raster-store.control-backing-double-reservation",
    "static RASTER_RETIREMENT_PROCESS_PAGES: std::sync::atomic::AtomicUsize",
    "static RASTER_INITIALIZATION_PROCESS_CONTROLS: std::sync::atomic::AtomicUsize",
    "static RASTER_STANDALONE_PROCESS_CONTROLS: std::sync::atomic::AtomicUsize",
    "held_items: usize",
    "held_bytes: usize",
    "control: std::mem::ManuallyDrop<Option<RasterStandaloneControlCredit>>",
    "control: std::mem::ManuallyDrop<Option<RasterStandaloneControlCredit>>",
    "let control = RasterStandaloneControlCredit::try_claim().ok();",
    "let control = RasterStandaloneControlCredit::try_claim().ok();",
    "fn claim_control_if_available(&mut self) -> Result<bool, String>",
    "Err(\"raster-store.standalone-control-capacity\") => Ok(false)",
    "control_returned: bool",
    "remaining: RASTER_NON_STACK_CONTROL_BACKINGS",
    "control_reservation: std::mem::ManuallyDrop<Option<RasterInitializationControlReservation>>",
    "normal completion returns every non-stack process control credit",
    "standalone Box control credit is returned before terminal-empty",
    "standalone Arc and inner Box control credits return before terminal-empty",
    "fn reserve_page_credit(&mut self, page_index: usize)",
    "compare_exchange(current, next",
    "self.return_page_credit(page_index)?",
    "pages: std::mem::ManuallyDrop<[Option<Box<RasterOwnedMapPage<V>>>; RASTER_OWNED_MAP_PAGE_COUNT]>",
    "Raster owned map reached Drop before every entry and page backing was explicitly retired",
    "Populated Raster owned maps require the retained page clone authority",
    "Raster maps require the retained page decoder",
    "Populated Raster owned map DSL materialization is forbidden; interactive production routes require the retained page output authority",
    "dsl::FieldValue::Map(Vec::new())",
    "pub fn remove_entry(&mut self, key: &str) -> Option<RasterOwnedMapEntry<V>>",
    "RasterOwnedMapInsert::Replaced",
    "raster_empty_asset_map_retirement_has_no_hidden_allocation_release",
    "value.assets.take_last_entry()",
    "value.assets.take_empty_page_backing()",
    "RasterRetirementOwner::AssetMapPage(page)",
    "RasterRetirementOwner::ValueMapPage(page)",
    "RasterOwnedMap::<V>::conservative_page_credit_bytes()",
    "fn observe_candidate_capacity(",
    "fn raster_exact_string_from_parts(",
    "const RASTER_CONTROL_BACKING_BYTES: usize = RASTER_OWNED_FIELD_BYTES",
    "const RASTER_MAXIMUM_CONTROL_BACKINGS: usize = RASTER_RETIREMENT_STACK_PAGE_COUNT + RASTER_NON_STACK_CONTROL_BACKINGS",
    "RasterRetirementOwner::BoxedLayer(Some(layer))",
    "const RASTER_RETIREMENT_ADMITTED_FRAME_CAPACITY: usize = RASTER_RETIREMENT_LAYER_FRAMES + RASTER_RETIREMENT_VALUE_FRAMES + RASTER_RETIREMENT_WRAPPER_FRAMES",
    "const RASTER_RETIREMENT_STACK_CAPACITY: usize = RASTER_RETIREMENT_ADMITTED_FRAME_CAPACITY + RASTER_RETIREMENT_REJECTED_OWNER_MARGIN",
    "raster_owned_map_cap_plus_one_returns_exact_owner_and_populated_pages_retire_explicitly",
    "raster_observed_capacity_and_combined_retirement_depth_are_exact",
    "raster_box_and_arc_control_backings_require_and_report_fixed_credit",
    "raster_standalone_control_max_plus_one_returns_exact_owner_and_resumes_after_full_saturation",
    "raster_arc_factory_full_saturation_preserves_exact_producer_through_every_control_phase",
    "raster_populated_dsl_materialization_max_plus_one_nested_cancel_fault_panic_and_close_are_exact",
    "impl<V> dsl::ToValue for RasterOwnedMap<V>",
    "impl<V> dsl::FromValue for RasterOwnedMap<V>",
    "Populated Raster owned map serialization is forbidden; interactive production routes require the retained page output authority",
    "    pub assets: RasterOwnedMap<RasterAssetChild>,",
    "    pub assets: RasterOwnedMap<RasterAssetChild>,",
    "        params: RasterOwnedMap<dsl::DslValue>,",
    "raster_populated_serde_output_max_plus_one_nested_cancel_fault_panic_and_close_are_exact",
    "pub(crate) fn require_empty_output_shell(&self) -> Result<(), &'static str>",
    "self.layers.is_empty() && self.assets.is_empty()",
    "Populated Raster snapshot output is forbidden; interactive production routes require the retained page output authority",
    "pub(crate) fn enc_asset_map(map: &RasterOwnedMap<RasterAssetChild>) -> String",
    "pub(crate) fn enc_params(params: &RasterOwnedMap<dsl::DslValue>) -> String",
    "fn write_asset_map(out: &mut Vec<u8>, map: &RasterOwnedMap<RasterAssetChild>)",
    "fn write_params(out: &mut Vec<u8>, params: &RasterOwnedMap<dsl::DslValue>)",
    "fn print_raster_snapshot_body(s: &RasterSnapshot) -> String",
    "fn encode_raster_snapshot_binary(s: &RasterSnapshot) -> Vec<u8>",
    'assert!(map.is_empty(), "{RASTER_POPULATED_OUTPUT_ERROR}");',
    'assert!(params.is_empty(), "{RASTER_POPULATED_OUTPUT_ERROR}");',
    'assert!(map.is_empty(), "{RASTER_POPULATED_OUTPUT_ERROR}");',
    'assert!(params.is_empty(), "{RASTER_POPULATED_OUTPUT_ERROR}");',
    'assert!(list.is_empty(), "{RASTER_POPULATED_OUTPUT_ERROR}");',
    'assert!(list.is_empty(), "{RASTER_POPULATED_OUTPUT_ERROR}");',
    "s.require_empty_output_shell().expect(RASTER_POPULATED_OUTPUT_ERROR);",
    "s.require_empty_output_shell().expect(RASTER_POPULATED_OUTPUT_ERROR);",
    "self.require_empty_output_shell().expect(RASTER_POPULATED_OUTPUT_ERROR);",
    "self.require_empty_output_shell().map_err(|error| store::PackError::Schema(error.to_owned()))?;",
    "fn raster_populated_snapshot_output_max_plus_one_nested_cancel_fault_panic_and_close_are_exact()",
    "let plus_one_param_value_pointer = plus_one_param_value.as_ptr();",
    "let rejected_param = params.insert",
    "params.insert(plus_one_param_key, dsl::DslValue::String(plus_one_param_value))",
    "let rejected_param_value = match &rejected_param.value",
    'assert_eq!(rejected_param_value.as_ptr(), plus_one_param_value_pointer, "rejected output parameter returns the exact value allocation");',
    "RasterOwnedRetirement::new(RasterRetirementOwner::ValueEntry { key: rejected_param.key, value: Some(rejected_param.value) })",
    "let plus_one_asset_child_pointer = plus_one_asset_child.child_id.as_ptr();",
    "let rejected_asset = assets",
    "assets.insert(plus_one_asset_key, plus_one_asset_child)",
    'assert_eq!(rejected_asset.value.child_id.as_ptr(), plus_one_asset_child_pointer, "rejected output asset returns the exact child allocation");',
    "RasterOwnedRetirement::new(RasterRetirementOwner::AssetEntry { key: rejected_asset.key, child: Some(rejected_asset.value) })",
    ...["bmp", "png", "tiff", "jpg"].map(
      (format) =>
        `pub fn serialize_bytes(snapshot: &RasterSnapshot) -> Result<Vec<u8>, String> { let image = raster_composite_image(snapshot)?; let target = semio_image_to_format(&image, ${format.toUpperCase()}_DIALECT)?; semio_s_artifact_stdio_${format}::io::encode_${format}(&target) }`,
    ),
    "pub fn serialize_bytes(snapshot: &RasterSnapshot) -> Result<Vec<u8>, String> { let image = raster_composite_image(snapshot)?; let gif89a = semio_image_to_format(&image, GIF89A_DIALECT)?; semio_s_artifact_stdio_gif::standards::v87a::subsets::any::io::encode_gif(&gif87a::from_89a(&gif89a)) }",
    "pub fn serialize_bytes(snapshot: &RasterSnapshot) -> Result<Vec<u8>, String> { let (svg, _width, _height) = crate::artifacts::raster::io::raster_document_json_to_svg(snapshot)?; Ok(svg.into_bytes()) }",
    'pub const RASTER_PDF_EXPORT_UNSUPPORTED: &str = "pdf export not supported for a raster document";',
    "pub fn serialize_bytes(snapshot: &RasterSnapshot) -> Result<Vec<u8>, String> { let _ = snapshot; Err(RASTER_PDF_EXPORT_UNSUPPORTED.to_string()) }",
    'pub const RASTER_DWG_EXPORT_UNSUPPORTED: &str = "dwg export not supported for a raster document";',
    "pub fn serialize_bytes(snapshot: &RasterSnapshot) -> Result<Vec<u8>, String> { let _ = snapshot; Err(RASTER_DWG_EXPORT_UNSUPPORTED.to_string()) }",
    "std::mem::size_of::<RasterOwnedRetirement>() <= RASTER_CONTROL_BACKING_BYTES",
    "std::mem::size_of::<RasterRetirementFramePage>() <= RASTER_CONTROL_BACKING_BYTES",
    "fn raster_maximum_combined_layer_and_value_depth_retires_to_terminal()",
    "Ok(Some((previous_key, previous)))",
    "Ok(Some((previous_key, previous)))",
    "Ok(Some((previous_key, previous)))",
    "replacement returns the exact displaced pair",
    "assert_eq!(previous_key.as_ptr(), old_key_pointer)",
    "candidate_disposer",
    "let released_bytes = value.capacity();",
    "let bytes = value.capacity();",
  ].join("\n");
  const retainedRasterEditor = [
    "fn build_envelope_decode_owner_bundle() raster_envelope_decode_owner_bundle() fn build_document_store_initialization_job( raster_document_store_initialization_job(envelope, operation, generation)",
    "fn admit_raster_envelope(app: &mut VcsArtifactApp<EditorApp<RasterPlayApp>>, wire: &[u8]) -> semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle {",
    "let handle = app.begin_artifact_envelope_ingress(pages, wire.len().max(1)).expect(\"Raster live envelope ingress credits\");",
    "for chunk in wire.chunks(store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES) {",
    "let mut bytes = [0; store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES];",
    "let page = store::os_store::owned_schema_decode::ArtifactEnvelopeDecodePage::try_from_array(bytes, chunk.len()).expect(\"bounded Raster live envelope page\");",
    "app.admit_artifact_envelope_ingress_page(handle, page).unwrap_or_else(|(fault, _page)| panic!(\"Raster live envelope page admission failed: {fault}\"));",
    "assert!(app.seal_artifact_envelope_ingress(handle).expect(\"Raster live envelope seal/submit\"));",
    "app.maintenance_step(1, store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect(\"one Raster live maintenance turn\");",
    "let poll = app.advance_artifact_envelope_load(handle).expect(\"Raster live load advancement\");",
    "match retirement.close_step(1, store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect(\"Raster fixture envelope retirement\") {",
    "async fn raster_live_envelope_submit_pump_swap_displaced_store_and_exact_ack_succeed() {",
    "assert_eq!(handle.generation, base_generation);",
    "assert!(app.acknowledge_artifact_store_replacement(handle).expect(\"first exact Raster load acknowledgement\"));",
    "assert!(!app.acknowledge_artifact_store_replacement(handle).expect(\"duplicate Raster load acknowledgement is a no-op\"));",
    "async fn raster_live_envelope_cancel_closes_retained_pages_without_publication() {",
    "app.cancel_artifact_envelope_load(handle).expect(\"cancel exact Raster ingress\");",
  ].join("\n");
  return { retainedRasterCodec, retainedRasterEditor };
}

/** 🎞 Checks the raster envelope route, signature suspension and single-edit mutations. */
export function rasterEnvelopeRouteAndMutationChecks({ retainedJackStore, retainedRasterCodec, retainedRasterEditor, retainedWriterPlugin }: {
  readonly retainedJackStore: string;
  readonly retainedRasterCodec: string;
  readonly retainedRasterEditor: string;
  readonly retainedWriterPlugin: string;
}) {
  if (!toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec, retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test retained-Raster-envelope-route was falsely rejected.");
  for (const signature of [
    "pub(crate) fn enc_asset_map(map: &RasterOwnedMap<RasterAssetChild>) -> String",
    "pub(crate) fn enc_params(params: &RasterOwnedMap<dsl::DslValue>) -> String",
    "fn write_asset_map(out: &mut Vec<u8>, map: &RasterOwnedMap<RasterAssetChild>)",
    "fn write_params(out: &mut Vec<u8>, params: &RasterOwnedMap<dsl::DslValue>)",
    "fn print_raster_snapshot_body(s: &RasterSnapshot) -> String",
    "fn encode_raster_snapshot_binary(s: &RasterSnapshot) -> Vec<u8>",
  ]) {
    const suspended = signature.replace("fn ", "async fn ");
    if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace(signature, suspended), retainedRasterEditor, retainedWriterPlugin))
      throw new Error(`[verify interactivity tool-jobs] self-test Raster-immediate-signature ${signature} was falsely accepted as async.`);
  }
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec, retainedRasterEditor.replace('let handle = app.begin_artifact_envelope_ingress(pages, wire.len().max(1)).expect("Raster live envelope ingress credits");\n', ""), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-page-admitted-without-ingress-credits was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec, retainedRasterEditor.replace('let page = store::os_store::owned_schema_decode::ArtifactEnvelopeDecodePage::try_from_array(bytes, chunk.len()).expect("bounded Raster live envelope page");\napp.admit_artifact_envelope_ingress_page(handle, page).unwrap_or_else(|(fault, _page)| panic!("Raster live envelope page admission failed: {fault}"));', 'app.admit_artifact_envelope_ingress_page(handle, page).unwrap_or_else(|(fault, _page)| panic!("Raster live envelope page admission failed: {fault}"));\nlet page = store::os_store::owned_schema_decode::ArtifactEnvelopeDecodePage::try_from_array(bytes, chunk.len()).expect("bounded Raster live envelope page");'), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-page-admitted-before-bounded-construction was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec, retainedRasterEditor.replace("let mut bytes = [0; store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES]", "let mut bytes = Vec::new()"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-fixed-page-owner-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec, retainedRasterEditor.replace("assert_eq!(handle.generation, base_generation);", "let _ = handle;"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-generation-handle-erasure was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec, retainedRasterEditor.replace("-> semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle {", "{"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-untyped-ingress-handle was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec, retainedRasterEditor.replace("for chunk in wire.chunks(store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES) {", "let chunk = wire;"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-single-whole-wire-page was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec, retainedRasterEditor.replace("app.seal_artifact_envelope_ingress(handle)", "app.submit_all()"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-unsealed-ingress-submit was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec, retainedRasterEditor.replace("app.maintenance_step(1, store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)", "app.run_to_completion()"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-unpumped-load-progress was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec, retainedRasterEditor.replace("app.advance_artifact_envelope_load(handle)", "app.await_load()"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-unpolled-load-terminal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec, retainedRasterEditor.replace("duplicate Raster load acknowledgement is a no-op", "second Raster acknowledgement replaces the store again"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-repeatable-load-acknowledgement was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec, retainedRasterEditor.replace("async fn raster_live_envelope_submit_pump_swap_displaced_store_and_exact_ack_succeed()", "async fn raster_envelope_loads()"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-live-submit-fixture-missing was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec, retainedRasterEditor.replace("async fn raster_live_envelope_cancel_closes_retained_pages_without_publication()", "async fn raster_envelope_cancels()"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-live-cancel-fixture-missing was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("self.generation.0.checked_add(1)", "self.generation.0 + 1"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-unchecked-generation-publication was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replaceAll("if cx.should_yield()", "if false"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-deadline-fuel-guard-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("self.cancel_requested || cx.is_cancelled()", "self.cancel_requested"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-worker-cancel-guard-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("raster_store_initializer_zero_budget_advances_no_owner_or_phase", "raster_zero_budget_advances_work"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-zero-budget-fixture-missing was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("RemoveLayerAsset(payload)", "drop(payload)"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-mutation-catalog-hole was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("dsl::DslValue::Object(values)", "drop(value)"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-nested-value-deep-drop was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("raster_nested_snapshot_and_child_handles_retire_one_owner_per_grant", "raster_shallow_snapshot_fixture"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-nested-close-fixture-missing was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("raster_owner_caps_and_all_mutation_variants_retire_one_owner_per_grant", "raster_partial_owner_fixture"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-cap-terminal-fixture-missing was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("raster_envelope_caps_and_plus_one_page_return_the_exact_fixed_owner", "raster_unbounded_page_fixture"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-page-plus-one-fixture-missing was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec, retainedRasterEditor.replace("store::os_store::owned_schema_decode::ArtifactEnvelopeDecodePage::try_from_array(bytes, chunk.len())", "store::os_store::owned_schema_decode::ArtifactEnvelopeDecodePage::try_from_vec(bytes.to_vec(), chunk.len())"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-dynamic-page-route was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec, `${retainedRasterEditor}\nenvelope_json: &str`, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-whole-buffer-bypass was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec, retainedRasterEditor.replaceAll("acknowledge_artifact_store_replacement(handle)", "return Ready"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-terminal-without-exact-ack was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec, retainedRasterEditor.replace("app.cancel_artifact_envelope_load(handle)", "drop(handle)"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-cancel-owner-drop was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec, retainedRasterEditor.replace("close_step(1, store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)", "close_all()"), retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-bulk-close was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec, retainedRasterEditor, retainedWriterPlugin.replace("artifact_envelope_ingress_saturation_returns_exact_plus_one_owner_and_closes_fifo_slots", "artifact_envelope_ingress_drops_plus_one"))) throw new Error("[verify interactivity tool-jobs] self-test Raster-fixed-registry-exact-handback-fixture-missing was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec, retainedRasterEditor, retainedWriterPlugin.replace("artifact_envelope_ingress_cancel_and_interrupted_close_release_one_real_page_per_grant", "artifact_envelope_ingress_bulk_close"))) throw new Error("[verify interactivity tool-jobs] self-test Raster-one-page-close-fixture-missing was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("struct RasterSnapshotBoundsAuthority", "struct RasterPostCloneBoundsAuthority"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-recursive-preflight-authority-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("fn raster_reserve_unit(cx: &mut semio_framework_job::StepContext<'_>) -> bool", "fn raster_post_work_fuel(cx: &mut semio_framework_job::StepContext<'_>) -> bool"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-pre-work-fuel-authority-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("cx.consume_fuel(1)", "cx.consume_fuel(RASTER_CONTROL_BACKING_BYTES as u64)"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-mounted-64-fuel-byte-charge was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("self.source_control_bytes = RASTER_MAXIMUM_CONTROL_BYTES", "self.source_bytes = RASTER_MAXIMUM_NESTED_BYTES"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-control-payload-double-counting was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("static RASTER_INITIALIZATION_PROCESS_CONTROLS: std::sync::atomic::AtomicUsize", "static RASTER_UNTRACKED_CONTROLS: std::sync::atomic::AtomicUsize"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-non-stack-process-control-reservation-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("static RASTER_STANDALONE_PROCESS_CONTROLS: std::sync::atomic::AtomicUsize", "static RASTER_UNTRACKED_STANDALONE_CONTROLS: std::sync::atomic::AtomicUsize"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-standalone-Box-Arc-control-reservation-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replaceAll("control: std::mem::ManuallyDrop<Option<RasterStandaloneControlCredit>>", "control: Option<RasterStandaloneControlCredit>"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-standalone-control-terminal-shell-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replaceAll("let control = RasterStandaloneControlCredit::try_claim().ok();", "let control = RasterStandaloneControlCredit::try_claim().expect(\"saturation panic\");"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-saturated-constructor-panic-restoration was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("Err(\"raster-store.standalone-control-capacity\") => Ok(false)", "Err(code) => panic!(\"{code}\")"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-saturated-retirement-resume-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("control_returned: bool", "control_returned_without_witness: ()"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-Arc-control-return-witness-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("normal completion returns every non-stack process control credit", "normal completion leaks process control credit"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-process-control-zero-fixture-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("fn reserve_page_credit(&mut self, page_index: usize)", "fn allocate_page_without_credit(&mut self, page_index: usize)"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-retirement-page-credit-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("pages: std::mem::ManuallyDrop<[Option<Box<RasterOwnedMapPage<V>>>; RASTER_OWNED_MAP_PAGE_COUNT]>", "pages: [Option<Box<RasterOwnedMapPage<V>>>; RASTER_OWNED_MAP_PAGE_COUNT]"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-populated-map-fail-closed-shell-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, `${retainedRasterCodec}\npub fn remove(&mut self, key: &str)`, retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-key-discarding-remove-restoration was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("Populated Raster owned maps require the retained page clone authority", "Raster clone allocates map pages"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-populated-map-whole-clone-restoration was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("Raster maps require the retained page decoder", "Raster serde allocates map pages"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-uncredited-map-serde-restoration was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("Populated Raster owned map DSL materialization is forbidden; interactive production routes require the retained page output authority", "ordinary populated DSL materialization is allowed"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-populated-map-DSL-fail-closure-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("dsl::FieldValue::Map(Vec::new())", "let mut entries = Vec::with_capacity(self.length); for (key, value) in self { entries.push((key.clone(), value.to_value())); } dsl::FieldValue::Map(entries)"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-populated-map-uncredited-DSL-loop-restoration was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, `${retainedRasterCodec}\nserde_json::to_vec(source)`, retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-whole-recursive-encode-reintroduction was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, `${retainedRasterCodec}\nsource.clone()`, retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-whole-recursive-clone-reintroduction was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, `${retainedRasterCodec}\noperation.diff(current)\ndiff.apply(current)`, retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-monolithic-history-apply-reintroduction was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("candidate_disposer", "discarded_candidate"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-complete-candidate-retirement-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("let released_bytes = value.capacity();", "let released_bytes = value.len();"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-string-capacity-retirement-erasure was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("let bytes = value.capacity();", "let bytes = value.len();"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-byte-capacity-retirement-erasure was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("pages: std::mem::ManuallyDrop<[Option<Box<RasterRetirementFramePage>>; RASTER_RETIREMENT_STACK_PAGE_COUNT]>", "active: Box<RasterOwnedRetirement>"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-fixed-iterative-retirement-erasure was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("raster_small_mutation_against_deep_snapshot_is_cursorized_and_atomic", "raster_small_mutation_clones_deep_snapshot"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-low-fuel-deep-mutation-fixture-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("raster_cancel_after_complete_retires_the_unclaimed_candidate_before_terminal", "raster_cancel_drops_completed_candidate"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-cancel-after-complete-fixture-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("raster_nested_owner_item_and_byte_capacity_plus_one_reject_before_clone", "raster_nested_owner_capacity_without_plus_one"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-nested-owner-capacity-plus-one-fixture-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("raster_empty_asset_map_retirement_has_no_hidden_allocation_release", "raster_empty_asset_map_bulk_drop"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-empty-map-allocation-release-fixture-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("value.assets.take_empty_page_backing()", "drop(std::mem::take(&mut value.assets))"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-populated-map-page-retirement-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("fn observe_candidate_capacity(", "fn trust_requested_candidate_capacity("), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-observed-capacity-admission-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("const RASTER_CONTROL_BACKING_BYTES: usize = RASTER_OWNED_FIELD_BYTES", "const RASTER_CONTROL_BACKING_BYTES: usize = 0"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-control-backing-credit-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("const RASTER_MAXIMUM_CONTROL_BACKINGS: usize = RASTER_RETIREMENT_STACK_PAGE_COUNT + RASTER_NON_STACK_CONTROL_BACKINGS", "const RASTER_MAXIMUM_CONTROL_BACKINGS: usize = 16"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-control-backing-count-regression was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("RASTER_RETIREMENT_LAYER_FRAMES + RASTER_RETIREMENT_VALUE_FRAMES + RASTER_RETIREMENT_WRAPPER_FRAMES", "RASTER_MAXIMUM_NESTED_DEPTH * 2 + 8"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-combined-retirement-depth-regression was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("RASTER_RETIREMENT_ADMITTED_FRAME_CAPACITY + RASTER_RETIREMENT_REJECTED_OWNER_MARGIN", "RASTER_RETIREMENT_ADMITTED_FRAME_CAPACITY"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-rejected-owner-retirement-margin-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("raster_owned_map_cap_plus_one_returns_exact_owner_and_populated_pages_retire_explicitly", "raster_empty_map_only"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-populated-map-cap-fixture-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("raster_observed_capacity_and_combined_retirement_depth_are_exact", "raster_requested_capacity_only"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-observed-capacity-fixture-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("raster_box_and_arc_control_backings_require_and_report_fixed_credit", "raster_unreported_control_drop"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-control-owner-fixture-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("raster_standalone_control_max_plus_one_returns_exact_owner_and_resumes_after_full_saturation", "raster_standalone_saturation_panics"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-standalone-max-plus-one-fixture-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("raster_arc_factory_full_saturation_preserves_exact_producer_through_every_control_phase", "raster_arc_saturation_drops_producer"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-Arc-every-control-phase-saturation-fixture-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("raster_populated_dsl_materialization_max_plus_one_nested_cancel_fault_panic_and_close_are_exact", "raster_empty_dsl_map_only"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-populated-DSL-hostile-fixture-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("raster_populated_serde_output_max_plus_one_nested_cancel_fault_panic_and_close_are_exact", "raster_empty_serde_map_only"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-populated-serde-hostile-fixture-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("    pub assets: RasterOwnedMap<RasterAssetChild>,", "    pub assets: std::collections::HashMap<String, RasterAssetChild>,"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-unguarded-map-field-escape was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("impl<V> dsl::ToValue for RasterOwnedMap<V>", "impl<V: dsl::ToValue> dsl::ToValue for RasterOwnedMapEntry<V>"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-owned-map-output-authority-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("impl<V> dsl::FromValue for RasterOwnedMap<V>", "impl<V: dsl::FromValue> dsl::FromValue for RasterOwnedMapEntry<V>"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-owned-map-decode-authority-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, `${retainedRasterCodec}\n#[cfg_attr(test, serde(serialize_with = "crate::artifacts::raster::serialize_empty_owned_map"))]`, retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-dangling-serde-oracle-route was falsely accepted.");
}

/** ♻ Checks that restoring a whole loop in the raster serializers is refused. */
export function rasterEnvelopeLoopRestorationChecks({ retainedJackStore, retainedRasterCodec, retainedRasterEditor, retainedWriterPlugin }: {
  readonly retainedJackStore: string;
  readonly retainedRasterCodec: string;
  readonly retainedRasterEditor: string;
  readonly retainedWriterPlugin: string;
}) {
  const restoredRasterOwnedMapSerializeLoop = `impl<V: serde::Serialize> serde::Serialize for RasterOwnedMap<V> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(self.length))?;
        for (key, value) in self {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}`;
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, `${retainedRasterCodec}\n${restoredRasterOwnedMapSerializeLoop}`, retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-public-populated-serde-loop-bound-restoration was falsely accepted.");
  const restoredRasterEncAssetMapLoop = `pub(crate) fn enc_asset_map(map: &RasterOwnedMap<RasterAssetChild>) -> String {
    format!("[{}]", map.iter().map(|(k, v)| format!("[{},{}]", enc_str(k), enc_child(v))).collect::<Vec<_>>().join(","))
}`;
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, `${retainedRasterCodec}\n${restoredRasterEncAssetMapLoop}`, retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-snapshot-text-asset-map-whole-loop-restoration was falsely accepted.");
  const restoredRasterEncParamsLoop = `pub(crate) fn enc_params(params: &RasterOwnedMap<dsl::DslValue>) -> String {
    format!("[{}]", params.iter().map(|(k, v)| format!("[{},{}]", enc_str(k), hex_encode(&serde_json::to_vec(v).unwrap_or_default()))).collect::<Vec<_>>().join(","))
}`;
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, `${retainedRasterCodec}\n${restoredRasterEncParamsLoop}`, retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-snapshot-text-parameter-map-whole-loop-restoration was falsely accepted.");
  const restoredRasterWriteAssetMapLoop = `fn write_asset_map(out: &mut Vec<u8>, map: &RasterOwnedMap<RasterAssetChild>) {
    store::pack_rt::write_varint_u64(out, map.len() as u64);
    for (k, v) in map {
        write_str_lp(out, k);
        write_child(out, v);
    }
}`;
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, `${retainedRasterCodec}\n${restoredRasterWriteAssetMapLoop}`, retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-snapshot-pack-asset-map-whole-loop-restoration was falsely accepted.");
  const restoredRasterWriteParamsLoop = `fn write_params(out: &mut Vec<u8>, params: &RasterOwnedMap<dsl::DslValue>) {
    store::pack_rt::write_varint_u64(out, params.len() as u64);
    for (k, v) in params {
        write_str_lp(out, k);
        write_bytes_lp(out, &serde_json::to_vec(v).unwrap_or_default());
    }
}`;
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, `${retainedRasterCodec}\n${restoredRasterWriteParamsLoop}`, retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-snapshot-pack-parameter-map-whole-loop-restoration was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("semio_s_artifact_stdio_png::io::encode_png(&target)", "Ok(<RasterSnapshot as pack::snapshot::ArtifactDsl>::print_dsl(snapshot).into_bytes())"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-mounted-exporter-DSL-print-under-foreign-extension-restoration was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("let image = raster_composite_image(snapshot)?;", "let image = SemioImage::default();"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-mounted-exporter-document-composite-bypass was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("crate::artifacts::raster::io::raster_document_json_to_svg(snapshot)?", 'Ok::<_, String>((String::from("<svg/>"), 0u32, 0u32))?'), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-mounted-vector-exporter-composite-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("Err(RASTER_PDF_EXPORT_UNSUPPORTED.to_string())", "Ok(Vec::new())"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-mounted-exporter-silent-empty-output-instead-of-typed-decline was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("pub const RASTER_DWG_EXPORT_UNSUPPORTED: &str =", "const DWG_UNSUPPORTED: &str ="), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-mounted-exporter-decline-reason-constant-erasure was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("pub fn serialize_bytes(snapshot: &RasterSnapshot) -> Result<Vec<u8>, String> {", "fn unmounted_serialize_bytes(snapshot: &RasterSnapshot) -> Result<Vec<u8>, String> {"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-mounted-exporter-entry-point-unmounting was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("self.require_empty_output_shell().expect(RASTER_POPULATED_OUTPUT_ERROR);", "let _ = self;"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-public-DSL-output-preflight-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("s.require_empty_output_shell().expect(RASTER_POPULATED_OUTPUT_ERROR);\n", ""), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-snapshot-body-output-preflight-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("self.require_empty_output_shell().map_err(|error| store::PackError::Schema(error.to_owned()))?;", "let _ = self;"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-public-pack-output-preflight-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("raster_populated_snapshot_output_max_plus_one_nested_cancel_fault_panic_and_close_are_exact", "raster_empty_snapshot_output_only"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-populated-snapshot-output-hostile-fixture-removal was falsely accepted.");
}

/** 🪪 Checks the rejected output identity, binding and retirement order of the raster parameter and asset maps. */
export function rasterRejectedOutputIdentityChecks({ retainedJackStore, retainedRasterCodec, retainedRasterEditor, retainedWriterPlugin }: {
  readonly retainedJackStore: string;
  readonly retainedRasterCodec: string;
  readonly retainedRasterEditor: string;
  readonly retainedWriterPlugin: string;
}) {
  const rasterRejectedParamValueIdentity = 'assert_eq!(rejected_param_value.as_ptr(), plus_one_param_value_pointer, "rejected output parameter returns the exact value allocation");';
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace(rasterRejectedParamValueIdentity, ""), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-rejected-output-parameter-value-identity-removal was falsely accepted.");
  if (
    toolJobRasterEnvelopeCallerRetainedExact(
      retainedJackStore,
      retainedRasterCodec.replace(rasterRejectedParamValueIdentity, 'assert_eq!(rejected_param.key.as_ptr(), plus_one_param_pointer, "rejected output parameter returns only the key allocation");'),
      retainedRasterEditor,
      retainedWriterPlugin,
    )
  )
    throw new Error("[verify interactivity tool-jobs] self-test Raster-rejected-output-parameter-value-identity-substitution was falsely accepted.");
  const rasterRejectedParamMovedInsert = "params.insert(plus_one_param_key, dsl::DslValue::String(plus_one_param_value))";
  const rasterRejectedParamValueBinding = "let rejected_param_value = match &rejected_param.value";
  const rasterParamBindingBeforeInsertion = retainedRasterCodec
    .replace(rasterRejectedParamValueBinding, "")
    .replace(rasterRejectedParamMovedInsert, `${rasterRejectedParamValueBinding}\n${rasterRejectedParamMovedInsert}`);
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, rasterParamBindingBeforeInsertion, retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-rejected-output-parameter-binding-before-insertion was falsely accepted.");
  const rasterRejectedParamRetirement = "RasterOwnedRetirement::new(RasterRetirementOwner::ValueEntry { key: rejected_param.key, value: Some(rejected_param.value) })";
  const rasterParamRetirementBeforeAssertion = retainedRasterCodec
    .replace(rasterRejectedParamRetirement, "")
    .replace(rasterRejectedParamValueIdentity, `${rasterRejectedParamRetirement}\n${rasterRejectedParamValueIdentity}`);
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, rasterParamRetirementBeforeAssertion, retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-rejected-output-parameter-retirement-before-identity was falsely accepted.");
  const rasterRejectedAssetChildIdentity = 'assert_eq!(rejected_asset.value.child_id.as_ptr(), plus_one_asset_child_pointer, "rejected output asset returns the exact child allocation");';
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace(rasterRejectedAssetChildIdentity, ""), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-rejected-output-asset-child-identity-removal was falsely accepted.");
  if (
    toolJobRasterEnvelopeCallerRetainedExact(
      retainedJackStore,
      retainedRasterCodec.replace(rasterRejectedAssetChildIdentity, 'assert_eq!(rejected_asset.key.as_ptr(), plus_one_asset_pointer, "rejected output asset returns only the key allocation");'),
      retainedRasterEditor,
      retainedWriterPlugin,
    )
  )
    throw new Error("[verify interactivity tool-jobs] self-test Raster-rejected-output-asset-child-identity-substitution was falsely accepted.");
  const rasterRejectedAssetMovedInsert = "assets.insert(plus_one_asset_key, plus_one_asset_child)";
  const rasterAssetBindingBeforeInsertion = retainedRasterCodec
    .replace(rasterRejectedAssetChildIdentity, "")
    .replace(rasterRejectedAssetMovedInsert, `${rasterRejectedAssetChildIdentity}\n${rasterRejectedAssetMovedInsert}`);
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, rasterAssetBindingBeforeInsertion, retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-rejected-output-asset-binding-before-insertion was falsely accepted.");
  const rasterRejectedAssetRetirement = "RasterOwnedRetirement::new(RasterRetirementOwner::AssetEntry { key: rejected_asset.key, child: Some(rejected_asset.value) })";
  const rasterAssetRetirementBeforeAssertion = retainedRasterCodec
    .replace(rasterRejectedAssetRetirement, "")
    .replace(rasterRejectedAssetChildIdentity, `${rasterRejectedAssetRetirement}\n${rasterRejectedAssetChildIdentity}`);
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, rasterAssetRetirementBeforeAssertion, retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-rejected-output-asset-retirement-before-identity was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("std::mem::size_of::<RasterRetirementFramePage>() <= RASTER_CONTROL_BACKING_BYTES", "true"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-control-page-size-proof-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("raster_maximum_combined_layer_and_value_depth_retires_to_terminal", "raster_separate_depth_only"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-combined-depth-fixture-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replaceAll("RasterOwnedMapInsert::Replaced", "RasterOwnedMapInsert::DiscardedKey"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-replacement-key-owner-handback-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec.replace("replacement returns the exact displaced pair", "replacement drops incoming key"), retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-replacement-pointer-fixture-removal was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, `${retainedRasterCodec}\nsource.assets.clone()`, retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-retained-map-whole-clone-reintroduction was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, `${retainedRasterCodec}\nserde_json::to_value(&source.assets)\nsource.assets.to_value()`, retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-retained-map-serde-dsl-loop-reintroduction was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, `${retainedRasterCodec}\ndrop(populated_map)`, retainedRasterEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Raster-populated-map-ordinary-drop-reintroduction was falsely accepted.");
  if (toolJobRasterEnvelopeCallerRetainedExact(retainedJackStore, retainedRasterCodec, retainedRasterEditor, retainedWriterPlugin.replace("artifact store initialization job reached Drop before exact candidate handoff or terminal retained close", "artifact initializer job silently drops retained candidate"))) throw new Error("[verify interactivity tool-jobs] self-test Raster-public-initializer-drop-refusal-removal was falsely accepted.");
}
