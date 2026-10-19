/** 🎨️ The drawing envelope caller retained sources and their route, semantic digest and mutation checks. */
import { toolJobDrawingEnvelopeCallerRetainedExact } from "@semio-tech/repo-lib/laws";

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
    "struct DrawingNativeCloneAuthority",
    "struct DrawingLayerCloneWorkAuthority",
    "struct DrawingLayerLocator",
  ].join("\n");
  const retainedDrawingEditor = [
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
    "drawing_live_envelope_replay_equals_the_central_apply_fold_of_every_forward",
    "protocol::apply_diff_cold(outcome.diff(), &expected)",
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
  const retainedDrawingDefaultInitializer = "Ok(bounded_document_store_initialization_job(envelope, Self::DOCUMENT_SCHEMA, operation, generation, actor))";
  const retainedDrawingPlugin = [retainedWriterPlugin, retainedDrawingDefaultInitializer].join("\n");
  if (!toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec, retainedDrawingEditor, retainedDrawingPlugin)) throw new Error("[verify interactivity tool-jobs] self-test retained-Drawing-envelope-route was falsely rejected.");
  const secondDrawingApplyLanes = [
    "struct DrawingStoreInitializationAuthority",
    "struct DrawingMutationCandidateAuthority",
    "struct DrawingContainerRebuildAuthority",
    "struct DrawingDuplicateRewriteAuthority",
    "struct DrawingMutationArenaPool",
    "fn node_at_mut",
    "fn container_mut",
    "fn apply_selection_transform",
    "fn start_rebuild(&mut self, source: &mut DrawingSnapshot)",
    "snapshot_mut",
    "operation.diff(",
    "diff.apply(",
    "source.assets.remove(",
  ];
  for (const lane of secondDrawingApplyLanes) {
    if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, `${retainedDrawingCodec}\n${lane}`, retainedDrawingEditor, retainedDrawingPlugin)) throw new Error(`[verify interactivity tool-jobs] self-test Drawing-second-apply-lane-${lane} was falsely accepted.`);
  }
  const ownedDrawingInitializerHooks = [
    "fn build_document_store_initialization_job",
    "drawing_document_store_initialization_job(envelope, operation, generation, actor)",
    "request_drawing_mutation_arena_pool()",
    "drawing_mutation_arena_pool_fault",
    "arena_boot_fault: Option<&'static str>",
  ];
  for (const hook of ownedDrawingInitializerHooks) {
    if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec, `${retainedDrawingEditor}\n${hook}`, retainedDrawingPlugin)) throw new Error(`[verify interactivity tool-jobs] self-test Drawing-owned-initializer-hook-${hook} was falsely accepted.`);
    if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, `${retainedDrawingCodec}\n${hook}`, retainedDrawingEditor, retainedDrawingPlugin)) throw new Error(`[verify interactivity tool-jobs] self-test Drawing-host-initializer-hook-${hook} was falsely accepted.`);
  }
}

/** 🧪 Checks the drawing single-edit mutations of the codec, the editor and the plugin sources. */
export function drawingEnvelopeMutationChecks({ retainedJackStore, retainedDrawingCodec, retainedDrawingEditor, retainedWriterPlugin }: {
  readonly retainedJackStore: string;
  readonly retainedDrawingCodec: string;
  readonly retainedDrawingEditor: string;
  readonly retainedWriterPlugin: string;
}) {
  for (const anchor of retainedDrawingCodec.split("\n")) {
    if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec.replace(anchor, "omitted-host-anchor"), retainedDrawingEditor, retainedDrawingPlugin)) throw new Error(`[verify interactivity tool-jobs] self-test Drawing-host-anchor-omission-${anchor} was falsely accepted.`);
  }
  for (const law of retainedDrawingEditor.split("\n")) {
    if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec, retainedDrawingEditor.replace(law, "omitted-law"), retainedDrawingPlugin)) throw new Error(`[verify interactivity tool-jobs] self-test Drawing-law-omission-${law} was falsely accepted.`);
  }
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec, retainedDrawingEditor, retainedWriterPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-without-framework-default-initializer was falsely accepted.");
  if (toolJobDrawingEnvelopeCallerRetainedExact(retainedJackStore, retainedDrawingCodec, `${retainedDrawingEditor}\nenvelope_json: Option<String>`, retainedDrawingPlugin)) throw new Error("[verify interactivity tool-jobs] self-test Drawing-whole-string-envelope-ingress was falsely accepted.");
}
