import { toolJobStoreInitializerRetainedExact } from "../📄️caller-retained-documents/🟦️.ts";
import { toolJobRustBlock } from "../../📋️inventory/🟦️.ts";

export function toolJobRasterEnvelopeCallerRetainedExact(store: string, raster: string, editor: string, plugin: string): boolean {
  const variants = [
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
  ];
  const credits = editor.indexOf("begin_artifact_envelope_ingress(pages, ");
  const page = editor.indexOf("let mut bytes = [0; store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES]");
  const construct = editor.indexOf("store::os_store::owned_schema_decode::ArtifactEnvelopeDecodePage::try_from_array(bytes, ");
  const admit = editor.indexOf("admit_artifact_envelope_ingress_page(handle, page)");
  const rasterOwnedMapSerializeBound = /impl(?:\s*<[^>]*>)?\s*(?:serde::)?Serialize\s+for\s+RasterOwnedMap\b/s.test(raster);
  const rasterOwnedMapWholeSerializeMap = /serialize_map\s*\(\s*Some\s*\(\s*self\.length\s*\)\s*\)/s.test(raster);
  const rasterOwnedMapWholeSerializeEntry = /\.serialize_entry\s*\(/s.test(raster);
  const rasterOwnedMapSerdeGuards = (raster.match(/serde\(serialize_with = "crate::artifacts::raster::serialize_empty_owned_map"\)/g) ?? []).length;
  const rasterOwnedMapFieldSites = (raster.match(/^\s*(?:pub )?[a-z_]+: RasterOwnedMap</gm) ?? []).length;
  const rasterWholeJoinedOutput = /\.collect\s*::\s*<\s*Vec\s*<\s*_\s*>\s*>\s*\(\s*\)\s*\.join\s*\(/s.test(raster);
  const rasterPerValueJsonOutput = /serde_json::to_vec\s*\(\s*v\s*\)/s.test(raster);
  const rasterDirectMapOutput = /for\s*\(\s*k\s*,\s*v\s*\)\s*in\s*(?:map|params)\b/s.test(raster);
  const rasterEmptyMapOutputGuards = (raster.match(/assert!\((?:map|params)\.is_empty\(\), "\{RASTER_POPULATED_OUTPUT_ERROR\}"\);/g) ?? []).length;
  const rasterEmptyLayerOutputGuards = (raster.match(/assert!\(list\.is_empty\(\), "\{RASTER_POPULATED_OUTPUT_ERROR\}"\);/g) ?? []).length;
  const rasterSnapshotOutputPreflights = (raster.match(/s\.require_empty_output_shell\(\)\.expect\(RASTER_POPULATED_OUTPUT_ERROR\);/g) ?? []).length;
  const rasterMountedOutputEntryPoints = (raster.match(/pub fn serialize_bytes\(snapshot: &RasterSnapshot\) -> Result<Vec<u8>, String> \{/g) ?? []).length;
  const rasterMountedPixelComposites = (raster.match(/raster_composite_image\(snapshot\)/g) ?? []).length;
  const rasterMountedPixelDialects = (raster.match(/semio_image_to_format\(&image, /g) ?? []).length;
  const rasterMountedStdioEncoders = (raster.match(/semio_s_artifact_stdio_[a-z0-9_]*::[A-Za-z0-9_:]*io::encode_[a-z0-9]+\(/g) ?? []).length;
  const rasterMountedVectorComposites = (raster.match(/raster_document_json_to_svg\(snapshot\)/g) ?? []).length;
  const rasterMountedTypedDeclines = (raster.match(/Err\(RASTER_(?:PDF|DWG)_EXPORT_UNSUPPORTED\.to_string\(\)\)/g) ?? []).length;
  const rasterMountedOutputGuards = (raster.match(/snapshot\.require_empty_output_shell\(\)\.map_err\(str::to_owned\)\?;/g) ?? []).length;
  const rasterMountedOutputCallers = (raster.match(/Ok\(<RasterSnapshot as store::os_store::serialization::ArtifactDsl>::print_dsl\(snapshot\)\.into_bytes\(\)\)/g) ?? []).length;
  const rasterOutputFixtureStart = raster.indexOf("fn raster_populated_snapshot_output_max_plus_one_nested_cancel_fault_panic_and_close_are_exact()");
  const rasterOutputFixtureEnd = raster.indexOf("fn raster_maximum_combined_layer_and_value_depth_retires_to_terminal()", rasterOutputFixtureStart);
  const rasterOutputFixture = rasterOutputFixtureStart >= 0 && rasterOutputFixtureEnd > rasterOutputFixtureStart ? raster.slice(rasterOutputFixtureStart, rasterOutputFixtureEnd) : "";
  const rasterRejectedParamValueCapture = rasterOutputFixture.indexOf("let plus_one_param_value_pointer = plus_one_param_value.as_ptr();");
  const rasterRejectedParamInsert = rasterOutputFixture.indexOf("params.insert(plus_one_param_key, dsl::DslValue::String(plus_one_param_value))");
  const rasterRejectedParamValueBinding = rasterOutputFixture.indexOf("let rejected_param_value = match &rejected_param.value");
  const rasterRejectedParamValueAssertion = rasterOutputFixture.indexOf(
    'assert_eq!(rejected_param_value.as_ptr(), plus_one_param_value_pointer, "rejected output parameter returns the exact value allocation");',
  );
  const rasterRejectedParamRetirement = rasterOutputFixture.indexOf(
    "RasterOwnedRetirement::new(RasterRetirementOwner::ValueEntry { key: rejected_param.key, value: Some(rejected_param.value) })",
  );
  const rasterRejectedAssetChildCapture = rasterOutputFixture.indexOf("let plus_one_asset_child_pointer = plus_one_asset_child.child_id.as_ptr();");
  const rasterRejectedAssetInsert = rasterOutputFixture.indexOf("assets.insert(plus_one_asset_key, plus_one_asset_child)");
  const rasterRejectedAssetChildBinding = rasterOutputFixture.indexOf("rejected_asset.value.child_id.as_ptr()");
  const rasterRejectedAssetChildAssertion = rasterOutputFixture.indexOf(
    ', plus_one_asset_child_pointer, "rejected output asset returns the exact child allocation");',
    rasterRejectedAssetChildBinding,
  );
  const rasterRejectedAssetRetirement = rasterOutputFixture.indexOf(
    "RasterOwnedRetirement::new(RasterRetirementOwner::AssetEntry { key: rejected_asset.key, child: Some(rejected_asset.value) })",
  );
  return (
    store.includes("pub fn artifact_owned_spr_edit_history_decoder") &&
    store.includes("struct ArtifactOwnedSprMutationArrayAuthority") &&
    raster.includes("pub struct RasterEnvelopeOwnedFieldCatalog") &&
    raster.includes("artifact_owned_spr_edit_history_decoder") &&
    raster.includes("RasterSnapshotDecodeAuthority") &&
    raster.includes("RasterMutationDecodeAuthority") &&
    raster.includes("OwnedSchemaHexAuthority<RASTER_OWNED_FIELD_BYTES>") &&
    raster.includes("struct RasterSnapshotBoundsAuthority") &&
    raster.includes("struct RasterSnapshotCloneAuthority") &&
    raster.includes("struct RasterMutationCandidateAuthority") &&
    raster.includes("RASTER_RETIREMENT_STACK_CAPACITY") &&
    raster.includes("pages: std::mem::ManuallyDrop<[Option<Box<RasterRetirementFramePage>>; RASTER_RETIREMENT_STACK_PAGE_COUNT]>") &&
    raster.includes("pending_push: std::mem::ManuallyDrop<Option<RasterRetirementOwner>>") &&
    raster.includes("struct RasterStoreInitializationAuthority") &&
    raster.includes("impl semio_framework_plugin::ArtifactStoreInitializationAuthority<RasterSnapshot, RasterMutation> for RasterStoreInitializationAuthority") &&
    raster.includes("RasterStoreInitializationPhase::ValidateEdit { index }") &&
    raster.includes("self.edit_index.admit(&envelope.vcs.edits, index,") &&
    raster.includes("RasterStoreInitializationPhase::SeedHistory") &&
    raster.includes("RasterStoreInitializationPhase::BuildCandidate") &&
    (raster.match(/if cx\.should_yield\(\)/g) ?? []).length === 3 &&
    raster.includes("fn raster_reserve_unit(cx: &mut semio_framework_job::StepContext<'_>) -> bool") &&
    raster.includes("cx.consume_fuel(1)") &&
    raster.includes("self.cancel_requested || cx.is_cancelled()") &&
    raster.includes("ArtifactStoreInitializationRuntime::new") &&
    raster.includes("ArtifactStore::from_initialized_runtime_with_owners") &&
    raster.includes("self.generation.0.checked_add(1)") &&
    raster.includes("value.layers.pop()") &&
    raster.includes("value.assets.take_last_entry()") &&
    raster.includes("value.assets.take_empty_page_backing()") &&
    raster.includes("RasterRetirementOwner::AssetMapPage(page)") &&
    raster.includes("RasterRetirementOwner::ValueMapPage(page)") &&
    raster.includes("RasterOwnedMap::<V>::conservative_page_credit_bytes()") &&
    raster.includes("fn observe_candidate_capacity(") &&
    raster.includes("fn raster_exact_string_from_parts(") &&
    raster.includes("const RASTER_CONTROL_BACKING_BYTES: usize = RASTER_OWNED_FIELD_BYTES") &&
    raster.includes("const RASTER_MAXIMUM_CONTROL_BACKINGS: usize = RASTER_RETIREMENT_STACK_PAGE_COUNT + RASTER_NON_STACK_CONTROL_BACKINGS") &&
    raster.includes("const RASTER_MAXIMUM_CONTROL_BYTES: usize = RASTER_MAXIMUM_CONTROL_BACKINGS * RASTER_CONTROL_BACKING_BYTES") &&
    raster.includes("source_control_bytes: usize") &&
    raster.includes("self.source_control_bytes = RASTER_MAXIMUM_CONTROL_BYTES") &&
    raster.includes("raster-store.control-backing-double-reservation") &&
    raster.includes("static RASTER_RETIREMENT_PROCESS_PAGES: std::sync::atomic::AtomicUsize") &&
    raster.includes("static RASTER_INITIALIZATION_PROCESS_CONTROLS: std::sync::atomic::AtomicUsize") &&
    raster.includes("static RASTER_STANDALONE_PROCESS_CONTROLS: std::sync::atomic::AtomicUsize") &&
    raster.includes("held_items: usize") &&
    raster.includes("held_bytes: usize") &&
    (raster.match(/control: std::mem::ManuallyDrop<Option<RasterStandaloneControlCredit>>/g) ?? []).length === 2 &&
    (raster.match(/let control = RasterStandaloneControlCredit::try_claim\(\)\.ok\(\);/g) ?? []).length === 2 &&
    raster.includes("fn claim_control_if_available(&mut self) -> Result<bool, String>") &&
    raster.includes("Err(\"raster-store.standalone-control-capacity\") => Ok(false)") &&
    raster.includes("control_returned: bool") &&
    raster.includes("remaining: RASTER_NON_STACK_CONTROL_BACKINGS") &&
    raster.includes("control_reservation: std::mem::ManuallyDrop<Option<RasterInitializationControlReservation>>") &&
    raster.includes("normal completion returns every non-stack process control credit") &&
    raster.includes("standalone Box control credit is returned before terminal-empty") &&
    raster.includes("standalone Arc and inner Box control credits return before terminal-empty") &&
    raster.includes("fn reserve_page_credit(&mut self, page_index: usize)") &&
    raster.includes("compare_exchange(current, next") &&
    raster.includes("self.return_page_credit(page_index)?") &&
    raster.includes("pages: std::mem::ManuallyDrop<[Option<Box<RasterOwnedMapPage<V>>>; RASTER_OWNED_MAP_PAGE_COUNT]>") &&
    raster.includes("Raster owned map reached Drop before every entry and page backing was explicitly retired") &&
    raster.includes("Populated Raster owned maps require the retained page clone authority") &&
    raster.includes("Raster maps require the retained page decoder") &&
    raster.includes("impl<V> dsl::ToValue for RasterOwnedMap<V>") &&
    raster.includes("impl<V> dsl::FromValue for RasterOwnedMap<V>") &&
    raster.includes("Populated Raster owned map serialization is forbidden; interactive production routes require the retained page output authority") &&
    rasterOwnedMapFieldSites === 3 &&
    (rasterOwnedMapSerdeGuards === 0 || raster.includes("fn serialize_empty_owned_map<S: serde::Serializer, V>")) &&
    !rasterOwnedMapSerializeBound &&
    !rasterOwnedMapWholeSerializeMap &&
    !rasterOwnedMapWholeSerializeEntry &&
    raster.includes("pub(crate) fn require_empty_output_shell(&self) -> Result<(), &'static str>") &&
    raster.includes("self.layers.is_empty() && self.assets.is_empty()") &&
    raster.includes("Populated Raster snapshot output is forbidden; interactive production routes require the retained page output authority") &&
    raster.includes("pub(crate) fn enc_asset_map(map: &RasterOwnedMap<RasterAssetChild>) -> String") &&
    raster.includes("pub(crate) fn enc_params(params: &RasterOwnedMap<dsl::DslValue>) -> String") &&
    raster.includes("fn write_asset_map(out: &mut Vec<u8>, map: &RasterOwnedMap<RasterAssetChild>)") &&
    raster.includes("fn write_params(out: &mut Vec<u8>, params: &RasterOwnedMap<dsl::DslValue>)") &&
    raster.includes("fn print_raster_snapshot_body(s: &RasterSnapshot) -> String") &&
    raster.includes("fn encode_raster_snapshot_binary(s: &RasterSnapshot) -> Vec<u8>") &&
    !raster.includes("async fn write_asset_map(out: &mut Vec<u8>, map: &RasterOwnedMap<RasterAssetChild>)") &&
    !raster.includes("async fn write_params(out: &mut Vec<u8>, params: &RasterOwnedMap<dsl::DslValue>)") &&
    !raster.includes("async fn print_raster_snapshot_body(s: &RasterSnapshot) -> String") &&
    !raster.includes("async fn encode_raster_snapshot_binary(s: &RasterSnapshot) -> Vec<u8>") &&
    raster.includes("self.require_empty_output_shell().expect(RASTER_POPULATED_OUTPUT_ERROR);") &&
    raster.includes("self.require_empty_output_shell().map_err(|error| store::PackError::Schema(error.to_owned()))?;") &&
    rasterEmptyMapOutputGuards === 4 &&
    rasterEmptyLayerOutputGuards === 2 &&
    rasterSnapshotOutputPreflights === 2 &&
    rasterMountedOutputEntryPoints === 8 &&
    rasterMountedPixelComposites === 5 &&
    rasterMountedPixelDialects === 5 &&
    rasterMountedStdioEncoders === 5 &&
    rasterMountedVectorComposites === 1 &&
    rasterMountedTypedDeclines === 2 &&
    raster.includes("pub const RASTER_PDF_EXPORT_UNSUPPORTED: &str =") &&
    raster.includes("pub const RASTER_DWG_EXPORT_UNSUPPORTED: &str =") &&
    rasterMountedOutputGuards === 0 &&
    rasterMountedOutputCallers === 0 &&
    rasterRejectedParamValueCapture >= 0 &&
    rasterRejectedParamInsert > rasterRejectedParamValueCapture &&
    rasterRejectedParamValueBinding > rasterRejectedParamInsert &&
    rasterRejectedParamValueAssertion > rasterRejectedParamValueBinding &&
    rasterRejectedParamRetirement > rasterRejectedParamValueAssertion &&
    rasterRejectedAssetChildCapture >= 0 &&
    rasterRejectedAssetInsert > rasterRejectedAssetChildCapture &&
    rasterRejectedAssetChildBinding > rasterRejectedAssetInsert &&
    rasterRejectedAssetChildAssertion > rasterRejectedAssetChildBinding &&
    rasterRejectedAssetRetirement > rasterRejectedAssetChildAssertion &&
    !rasterWholeJoinedOutput &&
    !rasterPerValueJsonOutput &&
    !rasterDirectMapOutput &&
    raster.includes("Populated Raster owned map DSL materialization is forbidden; interactive production routes require the retained page output authority") &&
    raster.includes("dsl::FieldValue::Map(Vec::new())") &&
    raster.includes("pub fn remove_entry(&mut self, key: &str) -> Option<RasterOwnedMapEntry<V>>") &&
    !raster.includes("pub fn remove(&mut self, key: &str)") &&
    !raster.includes("result.insert(key.clone(), value.clone())") &&
    !raster.includes("while let Some((key, value)) = source.next_entry") &&
    raster.includes("RasterRetirementOwner::BoxedLayer(Some(layer))") &&
    raster.includes("const RASTER_RETIREMENT_ADMITTED_FRAME_CAPACITY: usize = RASTER_RETIREMENT_LAYER_FRAMES + RASTER_RETIREMENT_VALUE_FRAMES + RASTER_RETIREMENT_WRAPPER_FRAMES") &&
    raster.includes("const RASTER_RETIREMENT_STACK_CAPACITY: usize = RASTER_RETIREMENT_ADMITTED_FRAME_CAPACITY + RASTER_RETIREMENT_REJECTED_OWNER_MARGIN") &&
    raster.includes("dsl::DslValue::Array(values)") &&
    raster.includes("dsl::DslValue::Object(values)") &&
    variants.every((variant) => raster.includes(variant)) &&
    raster.includes("Raster store initialization authority reached Drop before exact candidate handoff or retained rejection close") &&
    raster.includes("raster_store_initializer_publishes_next_generation_and_candidate_closes_incrementally") &&
    raster.includes("raster_store_initializer_cancel_and_stale_generation_return_every_owner_terminal_empty") &&
    raster.includes("raster_store_initializer_zero_budget_advances_no_owner_or_phase") &&
    raster.includes("raster_nested_snapshot_and_child_handles_retire_one_owner_per_grant") &&
    raster.includes("raster_owner_caps_and_all_mutation_variants_retire_one_owner_per_grant") &&
    raster.includes("raster_envelope_caps_and_plus_one_page_return_the_exact_fixed_owner") &&
    raster.includes("raster_snapshot_bounds_and_clone_advance_one_pre_admitted_unit_with_low_nonzero_fuel") &&
    raster.includes("raster_empty_bounds_and_mounted_sixty_four_fuel_progress_across_second_map_page") &&
    raster.includes("raster_expired_deadline_advances_no_bounds_clone_or_mutation_owner") &&
    raster.includes("raster_small_mutation_against_deep_snapshot_is_cursorized_and_atomic") &&
    raster.includes("raster_cancel_after_complete_retires_the_unclaimed_candidate_before_terminal") &&
    raster.includes("raster_retirement_uses_allocation_capacity_and_fixed_iterative_depth") &&
    raster.includes("raster_nested_owner_item_and_byte_capacity_plus_one_reject_before_clone") &&
    raster.includes("raster_retirement_page_credit_is_claimed_before_allocation_and_returned_with_backing") &&
    raster.includes("raster_owned_map_removal_returns_exact_pair_and_populated_drop_refuses") &&
    raster.includes("raster_empty_asset_map_retirement_has_no_hidden_allocation_release") &&
    raster.includes("raster_owned_map_cap_plus_one_returns_exact_owner_and_populated_pages_retire_explicitly") &&
    raster.includes("raster_observed_capacity_and_combined_retirement_depth_are_exact") &&
    raster.includes("raster_box_and_arc_control_backings_require_and_report_fixed_credit") &&
    raster.includes("raster_standalone_control_max_plus_one_returns_exact_owner_and_resumes_after_full_saturation") &&
    raster.includes("raster_arc_factory_full_saturation_preserves_exact_producer_through_every_control_phase") &&
    raster.includes("raster_populated_dsl_materialization_max_plus_one_nested_cancel_fault_panic_and_close_are_exact") &&
    raster.includes("raster_populated_serde_output_max_plus_one_nested_cancel_fault_panic_and_close_are_exact") &&
    raster.includes("raster_populated_snapshot_output_max_plus_one_nested_cancel_fault_panic_and_close_are_exact") &&
    raster.includes("std::mem::size_of::<RasterOwnedRetirement>() <= RASTER_CONTROL_BACKING_BYTES") &&
    raster.includes("std::mem::size_of::<RasterRetirementFramePage>() <= RASTER_CONTROL_BACKING_BYTES") &&
    raster.includes("raster_maximum_combined_layer_and_value_depth_retires_to_terminal") &&
    (raster.match(/RasterOwnedMapInsert::Replaced/g) ?? []).length >= 1 &&
    raster.includes("replacement returns the exact displaced pair") &&
    raster.includes("assert_eq!(previous_key.as_ptr(), old_key_pointer)") &&
    raster.includes("candidate_disposer") &&
    raster.includes("let released_bytes = value.capacity();") &&
    raster.includes("let bytes = value.capacity();") &&
    !raster.includes("serde_json::to_vec(source)") &&
    !raster.includes("source.clone()") &&
    !raster.includes("source.assets.clone()") &&
    !raster.includes("serde_json::to_value(&source.assets)") &&
    !raster.includes("source.assets.to_value()") &&
    !raster.includes("entries.push((key.clone(), value.to_value()))") &&
    !raster.includes("drop(populated_map)") &&
    !raster.includes("operation.diff(current)") &&
    !raster.includes("diff.apply(current)") &&
    !raster.includes("active: Box<RasterOwnedRetirement>") &&
    !raster.includes("artifact_bounded_history_entry_decoder") &&
    editor.includes("fn build_envelope_decode_owner_bundle()") &&
    editor.includes("raster_envelope_decode_owner_bundle()") &&
    editor.includes("fn build_document_store_initialization_job(") &&
    editor.includes("raster_document_store_initialization_job(envelope, operation, generation)") &&
    editor.includes("-> semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle") &&
    editor.includes("assert_eq!(handle.generation, base_generation)") &&
    editor.includes("chunks(store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)") &&
    credits >= 0 &&
    page > credits &&
    construct > page &&
    admit > construct &&
    editor.includes("seal_artifact_envelope_ingress(handle)") &&
    editor.includes("app.maintenance_step(1, store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)") &&
    editor.includes("advance_artifact_envelope_load(handle)") &&
    editor.includes("acknowledge_artifact_store_replacement(handle)") &&
    editor.includes("duplicate Raster load acknowledgement is a no-op") &&
    editor.includes("cancel_artifact_envelope_load(handle)") &&
    editor.includes("close_step(1, store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)") &&
    editor.includes("raster_live_envelope_submit_pump_swap_displaced_store_and_exact_ack_succeed") &&
    editor.includes("raster_live_envelope_cancel_closes_retained_pages_without_publication") &&
    plugin.includes("envelope_ingress: ArtifactFixedRegistry<ActiveArtifactEnvelopeIngress>") &&
    toolJobStoreInitializerRetainedExact(plugin) &&
    plugin.includes("artifact store initialization job reached Drop before exact candidate handoff or terminal retained close") &&
    plugin.includes("artifact_envelope_ingress_saturation_returns_exact_plus_one_owner_and_closes_fifo_slots") &&
    plugin.includes("artifact_envelope_ingress_cancel_and_interrupted_close_release_one_real_page_per_grant") &&
    !editor.includes("envelope_json: &str") &&
    !editor.includes("reject_whole_buffer_artifact_envelope_ingress") &&
    !editor.includes("ArtifactStore::new") &&
    !editor.includes("source.to_vec()") &&
    !editor.includes("while let") &&
    !editor.includes("loop {")
  );
}

export function toolJobDrawingEnvelopeCallerRetainedExact(store: string, drawing: string, editor: string, plugin: string, editorTestLaws: string = editor): boolean {
  const variants = [
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
  ];
  const owners = [
    "DrawingRetirementOwner::Layer(value)",
    "DrawingRetirementOwner::Fill(fill)",
    "DrawingRetirementOwner::Stroke(stroke)",
    "DrawingRetirementOwner::Segments(values)",
    "DrawingRetirementOwner::Stops(values)",
    "DrawingRetirementOwner::Points(values)",
    "DrawingRetirementOwner::AssetEntry",
  ];
  const digestFields = [
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
  const requestStart = drawing.indexOf("pub fn request_drawing_mutation_arena_pool()");
  const requestAuthority = requestStart >= 0 ? toolJobRustBlock(drawing, drawing.indexOf("{", requestStart)) : undefined;
  const requestBlock = requestAuthority?.body ?? "";
  const borrowStart = drawing.indexOf("fn borrow_drawing_mutation_arena()");
  const borrowAuthority = borrowStart >= 0 ? toolJobRustBlock(drawing, drawing.indexOf("{", borrowStart)) : undefined;
  const borrowBlock = borrowAuthority?.body ?? "";
  const defaultStart = editor.indexOf("impl Default for DrawingPlayApp");
  const defaultAuthority = defaultStart >= 0 ? toolJobRustBlock(editor, editor.indexOf("{", defaultStart)) : undefined;
  const defaultBlock = defaultAuthority?.body ?? "";
  const governedBootstrapArm = /DrawingStoreInitializationPhase::InitializeArena\s*=>\s*\{\s*match\s+self\.arena_bootstrap_job\.step\(cx\)/s.test(drawing);
  const exactSourceUndo = /if let Some\(undo\) = self\.source_undo\s*\{[\s\S]*?self\.start_rebuild\(source, undo\.parent, None, Some\(undo\.index\), DrawingContainerRebuildRole::CloseSourceUndo\)\?;/s.test(drawing);
  return (
    store.includes("pub fn artifact_owned_spr_edit_history_decoder") &&
    store.includes("struct ArtifactOwnedSprMutationArrayAuthority") &&
    store.includes("self.scalar_entry") &&
    store.includes("artifact-spr.mutation-array-cancelled") &&
    store.includes("id_digest: [u8; 32]") &&
    store.includes('Self::push_revision_record(&mut self.revision.applied, identity_digest, b"applied", &id, edit_digest)?') &&
    drawing.includes("pub struct DrawingEnvelopeOwnedFieldCatalog") &&
    drawing.includes("artifact_owned_spr_edit_history_decoder") &&
    drawing.includes("DrawingSnapshotDecodeAuthority") &&
    drawing.includes("DrawingMutationDecodeAuthority") &&
    drawing.includes("OwnedSchemaHexAuthority<DRAWING_OWNED_FIELD_BYTES>") &&
    drawing.includes("const DRAWING_MAXIMUM_NESTED_ITEMS: usize = 4_096") &&
    drawing.includes("const DRAWING_MAXIMUM_LAYER_DEPTH: usize = 64") &&
    drawing.includes("struct DrawingSnapshotBoundsAuthority") &&
    drawing.includes("struct DrawingFixedOwnerCensus") &&
    drawing.includes("slots: [DrawingOwnerCreditSlot; DRAWING_MAXIMUM_NESTED_ITEMS]") &&
    drawing.includes("const DRAWING_MUTATION_OVERLAY_PAGE_CAPACITY: usize = 16") &&
    drawing.includes("const DRAWING_MUTATION_CONTAINER_SLOT_CAPACITY: usize = 64") &&
    drawing.includes("const DRAWING_MUTATION_ARENA_POOL_CAPACITY: usize = 4") &&
    drawing.includes("struct DrawingMutationArenaOwner") &&
    drawing.includes("struct DrawingMutationArenaPool {") &&
    drawing.includes("DrawingMutationArenaProcessState::Building") &&
    drawing.includes("#[cfg(test)]\n    fn try_new()") &&
    drawing.includes("struct DrawingMutationArenaOwnerBuilder") &&
    drawing.includes("struct DrawingMutationArenaPoolBootstrap") &&
    drawing.includes("struct DrawingMutationArenaBootstrapJob") &&
    drawing.includes("fn step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> DrawingMutationArenaBootstrapStep") &&
    drawing.includes("DrawingMutationArenaProcessState::Inert") &&
    drawing.includes("DrawingMutationArenaBootstrapAdmission::fixed()") &&
    drawing.includes("DrawingMutationArenaBootstrapStep::Blocked") &&
    governedBootstrapArm &&
    exactSourceUndo &&
    requestBlock.includes("DrawingMutationArenaProcessState::Inert") &&
    requestBlock.includes("DRAWING_MUTATION_ARENA_BOOTSTRAP_REQUESTED.store(true") &&
    !requestBlock.includes("bootstrap.step(") &&
    !requestBlock.includes("try_reserve_exact") &&
    !requestBlock.includes("DrawingMutationArenaPoolBootstrap::production") &&
    borrowBlock.includes("DrawingMutationArenaBorrowError::NotReady") &&
    borrowBlock.includes("request_drawing_mutation_arena_pool()") &&
    !borrowBlock.includes("bootstrap.step(") &&
    !borrowBlock.includes("try_reserve_exact") &&
    !borrowBlock.includes("DrawingMutationArenaPoolBootstrap::production") &&
    drawing.includes("DrawingStoreInitializationPhase::InitializeArena") &&
    drawing.includes("DrawingMutationArenaProcessTransition::Retire") &&
    drawing.includes("drawing_mutation_arena_pool_fault") &&
    drawing.includes("borrow_drawing_mutation_arena_from") &&
    drawing.includes("drawing-store.mutation-arena-pool-saturated") &&
    drawing.includes("drawing-store.mutation-arena-stale-generation") &&
    drawing.includes("owner.terminal_is_empty()") &&
    drawing.includes("arena_return_phase: u8") &&
    drawing.includes("Ok(Some(false)) => return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })") &&
    drawing.includes("reverse.try_reserve_exact(DRAWING_MUTATION_CONTAINER_SLOT_CAPACITY)") &&
    drawing.includes("pages.try_reserve_exact(DRAWING_MUTATION_OVERLAY_PAGE_CAPACITY)") &&
    drawing.includes("page.try_reserve_exact(DRAWING_MUTATION_RETAINED_PAGE_BYTES)") &&
    drawing.includes("total.checked_add(page.capacity())") &&
    drawing.includes("fn write_overlay_string") &&
    drawing.includes("target.capacity() < source.len()") &&
    drawing.includes("struct DrawingMutationOverlayPatch") &&
    drawing.includes("drawing-store.mutation-overlay-owner-changed") &&
    drawing.includes("struct DrawingAssetBoundsCursor") &&
    drawing.includes("assets.range::<str, _>") &&
    drawing.includes("drawing-store.preflight-depth-capacity") &&
    drawing.includes("drawing-store.preflight-item-capacity") &&
    drawing.includes("drawing-store.preflight-byte-capacity") &&
    drawing.includes("struct DrawingLayerCloneAuthority") &&
    drawing.includes("self.bounds.step(source, cx)") &&
    drawing.includes("layer.step(source.layers.get(self.index)") &&
    drawing.includes("target.children.push(Self::skeleton(child)?)") &&
    drawing.includes("struct DrawingStoreInitializationAuthority") &&
    drawing.includes("struct DrawingSemanticDigestCredit") &&
    drawing.includes("semantic: Option<semio_framework_hash::Sha256>") &&
    drawing.includes('digest.observe(b"drawing.semantic.sha256")') &&
    drawing.includes("struct DrawingFillDigestAuthority") &&
    drawing.includes("struct DrawingStrokeDigestAuthority") &&
    drawing.includes("struct DrawingPathSegmentDigestAuthority") &&
    drawing.includes("struct DrawingLayerVariantDigestAuthority") &&
    drawing.includes("struct DrawingLayerDigestAuthority") &&
    drawing.includes("struct DrawingMutationDigestAuthority") &&
    drawing.includes("struct DrawingMutationAggregateReservation") &&
    drawing.includes("mutation_source_items") &&
    drawing.includes("mutation_derived_items") &&
    drawing.includes("duplicate_candidate_items") &&
    drawing.includes("source_owner_bytes") &&
    drawing.includes("derived_owner_bytes") &&
    drawing.includes("DRAWING_MUTATION_AGGREGATE_ITEMS") &&
    drawing.includes("DRAWING_MUTATION_AGGREGATE_BYTES") &&
    drawing.includes("DRAWING_MUTATION_RETAINED_PAGE_ITEMS") &&
    drawing.includes("DRAWING_MUTATION_RETAINED_PAGE_BYTES") &&
    drawing.includes("drawing-store.mutation-aggregate-item-capacity") &&
    drawing.includes("drawing-store.mutation-aggregate-byte-capacity") &&
    drawing.includes("struct DrawingMutationCandidateAuthority") &&
    drawing.includes("drawing-store.mutation-candidate-stale-authority") &&
    drawing.includes("drawing-store.mutation-candidate-cancelled") &&
    drawing.includes("struct DrawingDuplicateRewriteAuthority") &&
    drawing.includes("material: [u8; DRAWING_DUPLICATE_MATERIAL_BYTES]") &&
    drawing.includes("self.hash_cursor + DRAWING_OWNED_FIELD_BYTES") &&
    drawing.includes('update(b"semio.drawing.duplicate-id.v1")') &&
    drawing.includes("update(&(self.id_len as u64).to_be_bytes())") &&
    drawing.includes("update(&(self.name_len as u64).to_be_bytes())") &&
    drawing.includes("pending_name: std::mem::ManuallyDrop<Option<String>>") &&
    drawing.includes("drawing-store.duplicate-destination-capacity") &&
    drawing.includes("DrawingMutationCandidatePhase::PreflightSource") &&
    drawing.includes("DrawingMutationCandidatePhase::PreflightMutation") &&
    drawing.includes("struct DrawingContainerRebuildAuthority") &&
    drawing.includes("const DRAWING_CONTAINER_REBUILD_MOVE_CAPACITY") &&
    drawing.includes("enum DrawingContainerRebuildMove") &&
    drawing.includes("rebuild.rollback_step()?") &&
    drawing.includes("DrawingContainerRebuildRole::CloseSourceUndo") &&
    drawing.includes("source_undo: Option<DrawingContainerSourceUndo>") &&
    drawing.includes("source.capacity() < output_capacity") &&
    drawing.includes("self.source.as_mut().ok_or(\"drawing-store.container-source\")?.push(value)") &&
    drawing.includes("DrawingStoreInitializationPhase::ValidateEditId") &&
    drawing.includes("DrawingStoreInitializationPhase::ValidateEditMeta") &&
    drawing.includes("DrawingStoreInitializationPhase::PrepareApplied") &&
    drawing.includes("DrawingStoreInitializationPhase::PrepareRedo") &&
    drawing.includes("impl semio_framework_plugin::ArtifactStoreInitializationAuthority<DrawingSnapshot, DrawingMutation> for DrawingStoreInitializationAuthority") &&
    drawing.includes("DrawingStoreInitializationPhase::ValidateEdit { index }") &&
    drawing.includes("self.edit_index.admit(&envelope.vcs.edits, index,") &&
    drawing.includes("DrawingStoreInitializationPhase::SeedHistory") &&
    drawing.includes("DrawingStoreInitializationPhase::BuildCandidate") &&
    drawing.includes("ArtifactStore::from_initialized_runtime_with_owners") &&
    drawing.includes("ArtifactStoreInitializationOwnerCatalog::try_new()") &&
    drawing.includes("ArtifactStoreInitializationRuntime::new_with_owner_catalog") &&
    drawing.includes("DrawingStoreInitializationPhase::MoveInitialOwner") &&
    drawing.includes("self.generation.0.checked_add(1)") &&
    owners.every((owner) => drawing.includes(owner)) &&
    digestFields.every((field) => drawing.includes(field)) &&
    variants.every((variant) => drawing.includes(variant)) &&
    drawing.includes("retained_drawing_mutation_candidate_covers_all_fourteen_variants_and_returns_exact_owners") &&
    drawing.includes("retained_drawing_depth_plus_one_and_hostile_fields_fault_then_close_terminal_empty") &&
    drawing.includes("retained_drawing_container_false_terminal_saturation_and_interrupted_close_preserve_exact_owner") &&
    drawing.includes("retained_drawing_schema_digest_distinguishes_every_nested_semantic_field") &&
    drawing.includes("retained_drawing_aggregate_credit_admits_exact_4096_rejects_plus_one_with_owner_handback") &&
    drawing.includes("retained_drawing_duplicate_hash_frames_domain_id_and_name_lengths_without_concatenation_collision") &&
    drawing.includes("retained_drawing_process_arena_pool_cap_plus_one_returns_exact_slots_and_rejects_stale_aba") &&
    drawing.includes("retained_drawing_duplicate_name_uses_preadmitted_page_and_returns_exact_rejection_owner") &&
    drawing.includes("retained_drawing_cancel_stale_each_replay_candidate_container_stage_preserves_last_valid") &&
    drawing.includes("retained_drawing_rebuild_fault_after_every_phase_rolls_back_exact_container_and_reuses_pool_slot") &&
    drawing.includes("retained_drawing_reorder_fault_after_source_handoff_restores_exact_nested_fifo_and_pool_roots") &&
    drawing.includes("retained_drawing_arena_bootstrap_failure_at_each_allocation_retires_one_exact_root_per_grant") &&
    drawing.includes("retained_drawing_arena_bootstrap_failure_after_each_bundle_keeps_every_root_until_terminal_close") &&
    drawing.includes("retained_drawing_arena_bootstrap_advances_one_allocation_per_turn_and_withholds_incomplete_pool") &&
    drawing.includes("retained_drawing_arena_bootstrap_exact_cap_and_plus_one_rejection_preserve_every_owner_until_close") &&
    drawing.includes("retained_drawing_arena_default_second_app_and_borrow_only_request_without_allocation") &&
    drawing.includes("retained_drawing_arena_bootstrap_job_cancel_budget_contention_and_saturation_are_governed") &&
    !drawing.includes("source.clone()") &&
    !drawing.includes("snapshot.clone()") &&
    !drawing.includes("operation.encode_op()") &&
    !drawing.includes("operation.diff(") &&
    !drawing.includes("diff.apply(") &&
    !drawing.includes("serde_json::from_value") &&
    !drawing.includes("serde_json::from_str") &&
    !drawing.includes("serde_json::to_vec") &&
    !drawing.includes("mutation_meta.iter().any") &&
    !drawing.includes("exact_for_test") &&
    !drawing.includes("iter().nth") &&
    !drawing.includes("structural_copies") &&
    !drawing.includes("DRAWING_ASSET_MAP_NODE_POINTERS") &&
    !drawing.includes("DrawingStoreInitializationPhase::CloneInitial") &&
    !drawing.includes("base.name.try_reserve_exact") &&
    !drawing.includes("DrawingMutationArenaOwner::try_new()?") &&
    !drawing.includes("rposition(Option::is_some)") &&
    !store.includes("id.clone(), edit_digest") &&
    !drawing.includes("drop(value.layers)") &&
    editor.includes("drawing_document_store_initialization_job(envelope, operation, generation)") &&
    editorTestLaws.includes("drawing_live_envelope_submit_recursive_clone_swap_displaced_store_and_exact_ack_succeed") &&
    editorTestLaws.includes("drawing_live_envelope_cancel_closes_retained_pages_without_publication") &&
    editorTestLaws.includes("drawing_live_envelope_rejects_single_and_final_edit_id_plus_one_before_mutation_candidate") &&
    editorTestLaws.includes("drawing_live_initializer_candidate_container_commit_ack_cancel_stale_preserve_last_valid_and_exact_handle") &&
    editor.includes("arena_boot_fault: Option<&'static str>") &&
    editor.includes("drawing_mutation_arena_pool_fault") &&
    editorTestLaws.includes('"forwards": [crate::artifacts::drawing::mutations::DrawingMutation::RenameLayer') &&
    editor.includes("pub struct DrawingEnvelopeLoadHandle") &&
    editor.includes("source: &js_sys::Uint8Array") &&
    editor.includes("begin_artifact_envelope_ingress(maximum_pages, maximum_bytes)") &&
    editor.includes("admit_artifact_envelope_ingress_page(handle.runtime_handle(), page)") &&
    editor.includes("seal_artifact_envelope_ingress(handle.runtime_handle())") &&
    editor.includes("app.maintenance_step(1, store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)") &&
    editor.includes("advance_artifact_envelope_load(handle.runtime_handle())") &&
    editor.includes("acknowledge_artifact_store_replacement(handle.runtime_handle())") &&
    editor.includes("cancel_artifact_envelope_load(handle.runtime_handle())") &&
    editor.includes("request_drawing_mutation_arena_pool()") &&
    defaultBlock.includes("request_drawing_mutation_arena_pool()") &&
    !defaultBlock.includes("bootstrap.step(") &&
    !defaultBlock.includes("try_reserve_exact") &&
    !defaultBlock.includes("initialize_drawing_mutation_arena_pool") &&
    plugin.includes("envelope_ingress: ArtifactFixedRegistry<ActiveArtifactEnvelopeIngress>") &&
    toolJobStoreInitializerRetainedExact(plugin) &&
    !editor.includes("reject_whole_buffer_artifact_envelope_ingress") &&
    !editor.includes("envelope_json: Option<String>") &&
    !editor.includes("while let") &&
    !editor.includes("loop {")
  );
}
