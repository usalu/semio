import { policyReadFileSafe, policyReadModuleTreeSource, policyReadRustPolicySource, policyReadUiReconcileSource } from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📇️inventory/🟦️.ts";
import { INTERACTIVITY_AUDIT_UI_VALUE_FILE, INTERACTIVITY_AUDIT_KERNEL_FILE, INTERACTIVITY_AUDIT_PLUGIN_CENTRAL_FILE, INTERACTIVITY_AUDIT_UI_BUILDER_FILE, INTERACTIVITY_AUDIT_UI_COMPONENT_FILE, INTERACTIVITY_AUDIT_UI_ACCESSIBILITY_FILE, INTERACTIVITY_AUDIT_UI_SURFACE_FILE, INTERACTIVITY_AUDIT_UI_DOCUMENT_FILE, INTERACTIVITY_AUDIT_UI_LIMITS_FILE, INTERACTIVITY_AUDIT_UI_LAYOUT_FILE, INTERACTIVITY_AUDIT_UI_PRESENT_FILE, INTERACTIVITY_AUDIT_REACTOR_PATCHES_FILE, INTERACTIVITY_AUDIT_REACTOR_FILE, INTERACTIVITY_AUDIT_SHARD_FILE, INTERACTIVITY_AUDIT_RENDERER_GLUE_FILE, INTERACTIVITY_AUDIT_RUN_FILE, INTERACTIVITY_AUDIT_OS_ACTIVATION_FILE, INTERACTIVITY_AUDIT_RENDERER_RUNTIME_FILE, INTERACTIVITY_AUDIT_WINDOW_MEASURE_FILE, INTERACTIVITY_AUDIT_SHELL_FILE, interactivityLiveReconcileFailures } from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/⚖️laws/🟦️.ts";
import { policyReadRustPolicyFamily, POLICY_RUST_TEST_EVIDENCE_BOUNDARY } from "../../../../../../🦑️repo/🔨️modules/📚️library/🔍️discovery/📖️source-access/🦀️rust-evidence/🟦️.ts";
import { INTERACTIVITY_AUDIT_KERNEL_UI_FILE, INTERACTIVITY_AUDIT_REACTOR_TURN_FILE } from "../../../../../../🦑️repo/🔨️modules/📚️library/⚖️laws/🔬️interactivity-audit/🗂️files/🟦️.ts";
import { listSourceFamily } from "../../../../../../../🔨️modules/📚️compiler/📦️packages/🟦️typescript/📖️syntax/🌳️module-tree/🟦️.ts";
import { readdirSync } from "node:fs";
import { dirname, join, relative, sep } from "node:path";

/** 🧭 Reads the live reconcile, patch, reactor, value, and schema texts the law measures. */
export function interactivityLiveReconcileBoundSources(repoRoot: string): { readonly reconcile: string; readonly patches: string; readonly reactor: string; readonly value: string; readonly schema: string } {
  const boundary = POLICY_RUST_TEST_EVIDENCE_BOUNDARY;
  const rel = (absolute: string): string => relative(repoRoot, absolute).split(sep).join("/");
  const testsUnder = (directory: string): string => listSourceFamily(directory, { tests: true }).map(rel).filter((file) => file.split("/").some((segment) => segment.includes("tests") || segment.includes("testing"))).map((file) => policyReadFileSafe(repoRoot, file)).join("\n");
  const withCorpus = (source: string, corpus: string): string => `${source}${boundary}${corpus}`;
  const family = (files: readonly string[]): string => policyReadRustPolicyFamily(repoRoot, files.filter((file) => policyReadFileSafe(repoRoot, file).length > 0));
  const tree = (file: string): string => policyReadModuleTreeSource(repoRoot, file);
  const rustIn = (directory: string, needle: string): string => {
    const folder = readdirSync(directory).find((name) => name.includes(needle));
    if (!folder) throw new Error(`[verify interactivity] missing ${needle}`);
    const rust = readdirSync(join(directory, folder)).find((name) => name.endsWith(".rs"));
    if (!rust) throw new Error(`[verify interactivity] missing rust root ${needle}`);
    return join(directory, folder, rust).slice(repoRoot.length + 1);
  };
  const contractDir = dirname(dirname(join(repoRoot, INTERACTIVITY_AUDIT_UI_DOCUMENT_FILE)));
  const resident = rustIn(contractDir, "resident");
  const valuesRoot = rustIn(contractDir, "values");
  const conformance = rustIn(contractDir, "conformance");
  const contractCorpus = testsUnder(contractDir);
  const kernelCorpus = testsUnder(dirname(dirname(join(repoRoot, INTERACTIVITY_AUDIT_KERNEL_UI_FILE))));
  const engineCorpus = testsUnder(dirname(dirname(dirname(dirname(join(repoRoot, INTERACTIVITY_AUDIT_RENDERER_GLUE_FILE))))));
  const patchesCorpus = testsUnder(dirname(join(repoRoot, INTERACTIVITY_AUDIT_REACTOR_PATCHES_FILE)));
  const windowKits = rustIn(dirname(join(repoRoot, INTERACTIVITY_AUDIT_PLUGIN_CENTRAL_FILE)), "window-kits");
  const reconcileBase = policyReadUiReconcileSource(repoRoot);
  const reconcileAt = reconcileBase.indexOf(boundary);
  const reconcile = (reconcileAt < 0 ? reconcileBase : reconcileBase.slice(0, reconcileAt)) + "\n" + tree(resident) + (reconcileAt < 0 ? "" : reconcileBase.slice(reconcileAt));
  const textRoot = (() => {
    const folder = readdirSync(contractDir).find((name) => name.endsWith("text") && !name.includes("edit"));
    if (!folder) throw new Error("[verify interactivity] missing text");
    const rust = readdirSync(join(contractDir, folder)).find((name) => name.endsWith(".rs"));
    if (!rust) throw new Error("[verify interactivity] missing text root");
    return join(contractDir, folder, rust).slice(repoRoot.length + 1);
  })();
  const presentation = rustIn(contractDir, "presentation");
  const value = withCorpus(tree(valuesRoot) + "\n" + tree(textRoot), contractCorpus);
  const kernelUi = tree(INTERACTIVITY_AUDIT_KERNEL_UI_FILE);
  const kernelTurn = kernelUi.slice(kernelUi.indexOf("//#region 🔖️TurnResult"), kernelUi.indexOf("//#endregion 🔖️TurnResult"));
  const kits = tree(windowKits);
  const commandAt = kits.indexOf("pub const UI_COMMAND_VALUE_DEPTH");
  const commandEnd = kits.indexOf("//#endregion 🔖️ActionFactory", Math.max(commandAt, 0));
  const commandBridge = commandAt < 0 ? "" : kits.slice(commandAt, commandEnd < 0 ? undefined : commandEnd);
  const schemaFiles = [INTERACTIVITY_AUDIT_UI_BUILDER_FILE, INTERACTIVITY_AUDIT_UI_COMPONENT_FILE, INTERACTIVITY_AUDIT_UI_ACCESSIBILITY_FILE, INTERACTIVITY_AUDIT_UI_SURFACE_FILE, INTERACTIVITY_AUDIT_UI_DOCUMENT_FILE, INTERACTIVITY_AUDIT_UI_LIMITS_FILE, INTERACTIVITY_AUDIT_UI_LAYOUT_FILE, INTERACTIVITY_AUDIT_UI_PRESENT_FILE];
  const schemaTree = schemaFiles.filter((file) => policyReadFileSafe(repoRoot, file).length > 0).map(tree).join("\n");
  const schemaAll = family(schemaFiles);
  const schemaBoundary = schemaAll.indexOf(boundary);
  const pluginAll = family([INTERACTIVITY_AUDIT_PLUGIN_CENTRAL_FILE, windowKits]);
  const pluginDir = dirname(join(repoRoot, INTERACTIVITY_AUDIT_PLUGIN_CENTRAL_FILE));
  const tableKit = policyReadFileSafe(repoRoot, rel(join(pluginDir, "🪟️window-kits", "📊️table", "🦀️.rs")));
  const tableKitTests = policyReadFileSafe(repoRoot, rel(join(pluginDir, "🧪️tests", "🔬️app-window-kits", "🦀️.rs")));
  const tableKitFixtures = ["table_kit_first_paint_stays_inside_the_body_node_budget_at_any_row_count", "table_kit_ends_a_window_of_heavy_rows_inside_the_body_item_budget", "table_kit_serves_a_whole_window_of_light_rows"].filter((fixture) => tableKitTests.includes(`fn ${fixture}`)).join("\n");
  const commandBridgeFixtures = ["retained_ui_value_bridge_cancel_and_deadline_preserve_the_original_owner", "retained_ui_value_bridge_rejects_depth_plus_one_without_consuming_the_original"].filter((fixture) => pluginAll.includes(`fn ${fixture}`)).join("\n");
  const kernelFacade = policyReadFileSafe(repoRoot, INTERACTIVITY_AUDIT_KERNEL_FILE);
  const fieldAt = kernelFacade.indexOf("pub ui_patches: UiTurnPatches");
  const uiPatchField = fieldAt < 0 ? "" : kernelFacade.slice(fieldAt, fieldAt + "pub ui_patches: UiTurnPatches".length);
  const schema = withCorpus(schemaTree + "\n" + uiPatchField + "\n" + tree(INTERACTIVITY_AUDIT_KERNEL_UI_FILE) + "\n" + tree(presentation) + "\n" + tree(conformance) + "\n" + kernelTurn + tableKit + commandBridge + (schemaBoundary < 0 ? "" : schemaAll.slice(schemaBoundary)) + "\n" + tableKitFixtures + "\n" + commandBridgeFixtures, `${contractCorpus}\n${kernelCorpus}`);
  const reactorBase = family([INTERACTIVITY_AUDIT_REACTOR_FILE, INTERACTIVITY_AUDIT_SHARD_FILE, INTERACTIVITY_AUDIT_RUN_FILE, INTERACTIVITY_AUDIT_OS_ACTIVATION_FILE, INTERACTIVITY_AUDIT_RENDERER_RUNTIME_FILE, INTERACTIVITY_AUDIT_WINDOW_MEASURE_FILE, INTERACTIVITY_AUDIT_SHELL_FILE]);
  const reactorAt = reactorBase.indexOf(boundary);
  const glueTree = policyReadModuleTreeSource(repoRoot, INTERACTIVITY_AUDIT_RENDERER_GLUE_FILE, { crates: true });
  const rendererDir = dirname(join(repoRoot, INTERACTIVITY_AUDIT_RENDERER_GLUE_FILE));
  const kernelRuntimeName = readdirSync(rendererDir).find((name) => name.includes("kernel-runtime"));
  if (!kernelRuntimeName) throw new Error("[verify interactivity] missing kernel runtime");
  const kernelRuntimeDir = join(rendererDir, kernelRuntimeName);
  const kernelOwners = tree(rustIn(kernelRuntimeDir, "kernel-retained-state")) + "\n" + tree(rustIn(kernelRuntimeDir, "kernel-pool"));
  const uiDir = dirname(contractDir);
  const declarationName = readdirSync(uiDir).find((name) => name.includes("declaration"));
  if (!declarationName) throw new Error("[verify interactivity] missing declaration");
  const measureFile = listSourceFamily(join(uiDir, declarationName)).map(rel).find((file) => file.includes("window-measure") && file.endsWith(".rs"));
  if (!measureFile) throw new Error("[verify interactivity] missing window measure");
  const measureFixture = listSourceFamily(uiDir, { tests: true }).map(rel).find((file) => file.includes("layout-wire-format") && file.endsWith(".rs")) ?? "";
  const pending = rustIn(dirname(join(repoRoot, INTERACTIVITY_AUDIT_REACTOR_FILE)), "pending");
  const reactorDir = dirname(join(repoRoot, INTERACTIVITY_AUDIT_REACTOR_FILE));
  const witDirName = readdirSync(reactorDir).find((name) => name.includes("wit"));
  const witFile = witDirName ? rel(join(reactorDir, witDirName, "🦀️.rs")) : "";
  const driveFile = listSourceFamily(dirname(join(repoRoot, INTERACTIVITY_AUDIT_REACTOR_PATCHES_FILE))).map(rel).find((file) => file.includes("tracker-drive") && file.endsWith(".rs"));
  if (!driveFile) throw new Error("[verify interactivity] missing tracker drive");
  const reactor = withCorpus((reactorAt < 0 ? reactorBase : reactorBase.slice(0, reactorAt)) + "\n" + glueTree + "\n" + kernelOwners + "\n" + tree(INTERACTIVITY_AUDIT_REACTOR_TURN_FILE) + "\n" + tree(measureFile) + "\n" + tree(pending) + "\n" + tree(INTERACTIVITY_AUDIT_KERNEL_UI_FILE) + "\n" + (witFile.length > 0 ? tree(witFile) : "") + "\n" + policyReadFileSafe(repoRoot, measureFixture) + (reactorAt < 0 ? "" : reactorBase.slice(reactorAt)), engineCorpus);
  const patchesFamily = family([INTERACTIVITY_AUDIT_REACTOR_PATCHES_FILE]);
  const patchesAt = patchesFamily.indexOf(boundary);
  const patches = withCorpus((patchesAt < 0 ? patchesFamily : patchesFamily.slice(0, patchesAt)) + "\n" + tree(driveFile) + (patchesAt < 0 ? "" : patchesFamily.slice(patchesAt)), patchesCorpus);
  return { reconcile, patches, reactor, value, schema };
}

/** 🧪️ Executes interactivity live reconcile policy assertions. */
export function interactivityLiveReconcileSelfTests(repoRoot: string): void {
  const { reconcile, patches, reactor, value, schema } = interactivityLiveReconcileBoundSources(repoRoot);
  const mutations: [string, string, string, string][] = [
    ["per-surface-credit-cap", reconcile.replace("UI_RESIDENT_SURFACE_BYTES: usize = 8 * 1024 * 1024", "UI_RESIDENT_SURFACE_BYTES: usize = 16 * 1024 * 1024"), patches, reactor],
    ["aggregate-credit-cap", reconcile.replaceAll("UI_RESIDENT_AGGREGATE_BYTES: usize = UI_RESIDENT_SLOTS * UI_RESIDENT_DOCUMENT_BYTES", "UI_RESIDENT_AGGREGATE_BYTES: usize = UI_RESIDENT_SLOTS * UI_RESIDENT_DOCUMENT_BYTES * 2"), patches, reactor],
    ["default-credit-bypass", reconcile.replace("max_bytes: SURFACE_RECONCILE_SURFACE_BYTES", "max_bytes: usize::MAX"), patches, reactor],
    ["render-fault-cancel-omitted", reconcile, patches, reactor.replace("grant.cancel();", "let _ = &grant;")],
    ["render-fault-owner-erased", reconcile, patches, reactor.replace("Err(failure)=>{\n                    grant.cancel();", "Err(_)=>{\n                    grant.cancel();")],
    ["render-fault-wire-erased", reconcile, patches, reactor.replace("grant.cancel();\n                    return Err(failure);", "grant.cancel();\n                    drop(failure); return Err(\"rendered fault dropped\".to_string());")],
    ["dynamic-poll-surfaces", reconcile, patches, reactor.replace("surfaces: ui_contract::UiFixedList<(u32, ui_contract::SurfaceId), DIRTY_RENDER_CAPACITY>", "surfaces: Vec<(u32, ui_contract::SurfaceId)>")],
    ["dynamic-poll-intents", reconcile, patches, reactor.replace("intents: ui_contract::UiFixedList<DirtyIntentBatch, DIRTY_INTENT_INSTANCE_CAPACITY>", "intents: Vec<DirtyIntentBatch>")],
    ["tracker-heap-surface", reconcile, patches.replace("surface: ui_contract::SurfaceId", "surface: String"), reactor],
    ["dynamic-renderer-registry", reconcile, patches, reactor.replace("slots: Box<[Option<RetainedSurfaceSlot>; RETAINED_SURFACE_CAPACITY]>", "slots: Vec<Option<RetainedSurfaceSlot>>")],
    ["lossy-renderer-refusal", reconcile, patches, reactor.replace("result.ui_patches.try_transfer_one(|patch| self.apply_ui_patch(instance, patch, patch_receipt))", "result.ui_patches.into_iter().map(|patch| self.apply_ui_patch(instance, patch))")],
    ["nonterminal-closer-erasure", reconcile, patches, reactor.replaceAll("if closing.close_step() && closing.terminal_is_empty()", "let _ = closing.close_step(); if true")],
    ["shell-nonterminal-document-erasure", reconcile, patches, reactor.replace("let terminal = slot.document.close_step_with_grant(SHELL_DOCUMENT_RETIREMENT_ITEMS, SHELL_DOCUMENT_RETIREMENT_BYTES).is_ok_and(|step| step.complete) && slot.document.terminal_is_empty()", "let _ = slot.document.close_step(); let terminal = true")],
    ["shell-saturation-owner-drop", reconcile, patches, reactor.replace("return Err(\"shell: spawned document retirement registry refused the exact prior owner\".to_string());", "refused.extend(owners);\n                drop(refused); return Ok(());")],
    ["dynamic-shell-document-retirement", reconcile, patches, reactor.replace("slots: Box<[Option<ShellDocumentRetirementSlot>; SHELL_DOCUMENT_RETIREMENT_CAPACITY]>", "slots: Vec<Option<ShellDocumentRetirementSlot>>")],
    ["missing-command-document-preflight", reconcile, patches, reactor.replace("self.command_document_closes.try_reserve_page(key, generation)", "Err(())")],
    ["command-ninth-document-one-step-drop", reconcile, patches, reactor.replace("self.command_document_closes.admit(&mut destinations, surface);", "let mut rejected = surface; let _ = rejected.document.close_step();")],
    ["command-nonterminal-document-erasure", reconcile, patches, reactor.replace("let terminal = document.close_step() && document.terminal_is_empty()", "let _ = document.close_step(); let terminal = true")],
    ["missing-shell-document-saturation-law", reconcile, patches, reactor.replace("shell_ninth_document_and_nonterminal_first_close_remain_in_qualified_retirement", "shell_document_smoke")],
    ["missing-shell-document-refusal-law", reconcile, patches, reactor.replace("shell_absolute_refusal_returns_the_exact_max_plus_one_owner_before_mutation", "shell_refusal_smoke")],
    ["missing-command-document-saturation-law", reconcile, patches, reactor.replace("command_batch_slots_plus_one_document_is_retained_after_nonterminal_close_and_exactly_returned_or_retired", "command_document_smoke")],
    ["missing-command-document-preflight-law", reconcile, patches, reactor.replace("command_document_page_saturation_refuses_before_turn_owner_production", "command_preflight_smoke")],
    ["wrapping-renderer-sequence", reconcile, patches, reactor.replace("self.committed.checked_add(1).filter(|generation| *generation != 0)", "Some(self.committed.wrapping_add(1))")],
    ["dynamic-window-partition", reconcile, patches, reactor.replaceAll("UiFixedList<&'a WindowMeasure, WINDOW_MEASURE_PARTITION_CAPACITY>", "Vec<&'a WindowMeasure>")],
    ["recursive-window-measure", reconcile, patches, reactor.replace("partition.general.try_push(measure)", "Vec<WindowMeasureRenderFrame<'_>>")],
    ["cloned-window-action", reconcile, patches, reactor.replace("partition.general.try_push(measure)", "{ let _dynamic = on_change.clone(); partition.general.try_push(measure) }")],
    ["slot-cap", reconcile.replace("SURFACE_RECONCILE_ADMISSION_SLOTS: usize = ui_contract::UI_RESIDENT_SLOTS", "SURFACE_RECONCILE_ADMISSION_SLOTS: usize = 65"), patches, reactor],
    ["page-cap", reconcile.replace("SURFACE_RECONCILE_PAGE_BYTES: usize = 32 * 1_024", "SURFACE_RECONCILE_PAGE_BYTES: usize = 64 * 1_024"), patches, reactor],
    ["missing-byte-preflight", reconcile.replace("limits.max_bytes <= SurfaceReconcileLimits::default().max_bytes", "false"), patches, reactor],
    ["cursor-before-reserve", reconcile.replace("let credit = if surface_bytes <= limits.max_identifier_bytes { reserve_surface_reconcile(limits) } else { None };", "let cursor = SurfaceReconcileCursor::new_admitted(tree, &current, limits);\n        let credit = if surface_bytes <= limits.max_identifier_bytes { reserve_surface_reconcile(limits) } else { None };").replace("        let cursor = SurfaceReconcileCursor::new_admitted(tree, &current, limits);\n        Ok(Self", "        Ok(Self"), patches, reactor],
    ["missing-generation", reconcile.replace("generation != state.generation", "false"), patches, reactor],
    ["missing-cancel", reconcile.replaceAll("if cx.is_cancelled()", "if false"), patches, reactor],
    ["double-step", reconcile.replace("match cursor.step(&current.shadow) {", "let _extra = cursor.step(&current.shadow);\n                match cursor.step(&current.shadow) {"), patches, reactor],
    ["missing-terminal-drop", reconcile.replace("impl Drop for SurfaceReconcileTerminal", "impl SurfaceReconcileTerminal"), patches, reactor],
    ["dynamic-tracker", reconcile, patches.replace("slots: Box<[Option<SurfaceSlot>]>", "slots: Vec<Option<SurfaceSlot>>"), reactor],
    ["effect-reorder", reconcile, patches.replace("generation < ready_generation", "false"), reactor],
    ["instance-close-erasure", reconcile, patches.replaceAll("surface_instance(surface) == Some(closing.instance)", "false"), reactor],
    ["tree-clone", reconcile, patches, reactor.replace("grant.commit_source(source,cx)", "{ let alias = unsafe { std::ptr::read(&*source) }; grant.commit_source(&mut Some(alias),cx) }")],
    ["missing-drive", reconcile, patches, reactor.replace("patches.drive_one(cx)", "more = false;")],
    ["missing-close", reconcile, patches, reactor.replace("command_document_closes.begin_close_instance(u64::from(instance))", "patches.defer(numeric_instance.to_string())")],
    ["old-diff", reconcile, patches.replace("pub fn reserve_mounted(&self, surface: ui_contract::SurfaceId, key: NativeCloseKey)", "pub fn diff_mounted(&self, surface: ui_contract::SurfaceId)"), reactor],
    ["missing-cap-fixture", reconcile, patches.replace("cap_plus_one_returns_the_exact_tree_owner", "cap_smoke"), reactor],
    ["recursive-semantic-helper", reconcile.replace("struct SurfaceSemanticCensusCursor", "fn tree_node_semantic_usage(_node: &crate::TreeNode) {}\nstruct SurfaceSemanticCensusCursor"), patches, reactor],
    ["missing-fixed-value-stack", reconcile.replace("value_stack.resize_with(SURFACE_RECONCILE_VALUE_DEPTH, || None)", "value_stack.resize_with(SURFACE_RECONCILE_VALUE_DEPTH + 1, || None)"), patches, reactor],
    ["whole-list-one-grant", reconcile.replace("let Some(value) = cursor.next() else", "let _whole: Vec<_> = cursor.by_ref().collect();\n                let Some(value) = cursor.next() else"), patches, reactor],
    ["missing-zero-fuel-guard", reconcile.replaceAll("if cx.should_yield()", "if false"), patches, reactor],
    ["release-credit-in-take-ready", reconcile.replace("state.credit.is_none()", "release_surface_reconcile(credit); let replacement_credit = reserve_surface_reconcile(SurfaceReconcileLimits::default()).unwrap(); let (candidate_credit, patch_credit) = match split_surface_reconcile(replacement_credit) {"), patches, reactor],
    ["missing-pre-materialization-credit", reconcile, patches.replaceAll("SurfaceReconcileReservation::try_new(generation)", "None"), reactor],
    ["dynamic-ready-owner", reconcile, patches.replace("ready: Box<[Option<ReadySlot>]>", "ready: Vec<Option<ReadySlot>>"), reactor],
    ["variable-fixed-owner-constructor", reconcile, patches.replace(".take(capacity).collect::<Vec<_>>().into_boxed_slice()", ".collect::<Vec<_>>().into_boxed_slice()"), reactor],
    ["unadmitted-owner-drop", reconcile, patches.replace("if self.generation==0||source.is_none()||cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_capacity_bytes<capacity{return Ok(false);}", "drop(root); return Ok(());"), reactor],
    ["mounted-source-retention", reconcile, patches.replace("reservation: SurfaceReconcileReservation", "tree: Option<ComponentTree>"), reactor],
    ["unmounted-render", reconcile, patches, reactor.replace("patches.reserve_mounted(surface, key)", "patches.begin(surface, tree)")],
    ["blocked-before-producing-terminal", reconcile, patches.replace("if terminal_target.is_none()", "if false"), reactor],
    ["saturating-generation", reconcile, patches.replace("state.next_generation.checked_add(1)", "state.next_generation.checked_add(1).or(Some(u64::MAX))"), reactor],
    ["lossy-public-handback", reconcile.replace("slot.state = Some(state);", "if slot.state.is_none() { slot.state = Some(state); } else { drop(state); }"), patches, reactor],
    ["close-ready-omission", reconcile, patches.replaceAll("state.ready.iter().all(Option::is_none)", "true"), reactor],
    ["missing-terminal-saturation-fixture", reconcile, patches.replace("terminal_full_plus_matching_unadmitted_advances_capacity_before_conversion", "terminal_unadmitted_smoke"), reactor],
    ["missing-exhaustion-fixture", reconcile, patches.replace("generation_max_is_issued_once_and_repeated_exhaustion_returns_exact_owners_without_mutation", "generation_smoke"), reactor],
    ["missing-public-drop-fixture", reconcile.replace("public_drop_handback_is_lossless_at_terminal_cap_and_plus_one", "public_drop_smoke"), patches, reactor],
    ["page-index-rewalk", reconcile.replace("cursor: ui_contract::UiMapCursor", "values: ui_contract::UiMap, entry: usize").replace("page.cursor.advance()", "page.values.iter().nth(page.entry)"), patches, reactor],
    ["raw-map-pointer", reconcile.replace("cursor: ui_contract::UiMapCursor", "values: *const ui_contract::UiMap, entry: usize").replace("page.cursor.advance()", "unsafe { (&*page.values).iter().nth(page.entry) }"), patches, reactor],
    ["dynamic-reconcile-ops", reconcile.replace("    pub(super) ops: ui_contract::UiPatchOps,\n    pub(super) pending_op: ui_contract::UiPendingPatchOp", "    ops: Vec<ui_contract::UiPatchOp>,\n    pending_op: Option<ui_contract::UiPatchOp>"), patches, reactor],
    ["dynamic-retire-forest", reconcile.replace("retire_tree: SurfaceTreeRetireCursor", "retire_forest: Vec<std::vec::IntoIter<crate::TreeNode>>"), patches, reactor],
    ["eager-variable-diff", reconcile.replace("fn diff_record_field", "fn diff_record"), patches, reactor],
    ["dynamic-reactor-pending", reconcile, patches, reactor.replace("slots: [Option<PendingPatchSlot>; PENDING_PATCH_CAPACITY]", "slots: Vec<PendingPatchSlot>")],
    ["missing-published-ack", reconcile, patches, reactor.replace("apply_issued_ack", "drop((&surface, revision))")],
    ["missing-unforgeable-ack", reconcile.replace("pub struct SurfaceReconcilePublishedAck", "struct SurfaceReconcilePublishedAck"), patches, reactor],
    ["panic-after-published-transfer", reconcile, patches, reactor.replace("let Some(patch) = handbacks[cell].patch.source_mut()?.take() else {\n            return Err(\"pending publication reported bytes and handed out no patch\");\n        };", 'let slot = self.slots[index].take().expect("matching published patch authority");')],
    ["drop-ready-credit", reconcile.replace("handback_surface_reconcile(Box::new(SurfaceReconcileRetained", "if let Some(credit) = self.credit.take() { release_surface_reconcile(credit); } handback_surface_reconcile(Box::new(SurfaceReconcileRetained"), patches, reactor],
    ["deep-retire-overflow", reconcile.replaceAll("if self.depth == SURFACE_RECONCILE_VALUE_DEPTH", "if false"), patches, reactor],
    ["generation-commit-before-reservation", reconcile, patches.replace("        let Some(reservation) = SurfaceReconcileReservation::try_new(generation)", "        commit_generation(&mut state, generation);\n        let Some(reservation) = SurfaceReconcileReservation::try_new(generation)").replace("        commit_generation(&mut state, generation);\n        state.rejected_reserved", "        state.rejected_reserved"), reactor],
    ["rejection-generation-before-terminal", reconcile, patches.replace("let Some(target_index) = state.terminals.iter().position(Option::is_none) else { return false };", "let target_index = 0;").replace("let Some(generation) = next_generation(&state) else { return false };\n        let Some(reconciler)", "let Some(generation) = next_generation(&state) else { return false };\n        let Some(target_index) = state.terminals.iter().position(Option::is_none) else { return false };\n        let Some(reconciler)"), reactor],
    ["linear-public-take", reconcile.replace("let Some(slot) = registry.slots.get_mut(key.slot) else { return Ok(None); };", "let slot = registry.slots.iter_mut().find(|slot| slot.generation == key.generation)?;"), patches, reactor],
    ["uncredited-terminal-constructor", reconcile.replace("pub fn try_from_reconciler", "pub fn from_reconciler"), patches, reactor],
    ["missing-map-page-fixture", reconcile.replace("retained_map_page_advances_each_key_once_without_rewalking_prior_entries", "map_page_smoke"), patches, reactor],
    ["missing-transactional-generation-fixture", reconcile, patches.replace("terminal_saturation_does_not_consume_maximum_generation_before_exact_owner_reservation", "generation_reservation_smoke"), reactor],
    ["saturating-command-cursor", reconcile, patches, reactor.replace("cursor.page_index.checked_add(1)", "cursor.page_index.saturating_add(1)")],
    ["panicking-command-page", reconcile, patches, reactor.replace("match semio_framework::kernel::PagedCommand::try_from_pages(pages){", "match Ok::<_, (semio_framework::Fault, semio_framework::kernel::CommandPageSet)>(semio_framework::kernel::PagedCommand::try_from_pages(pages).expect(\"host-validated generic pages preserve their fixed shape\")){")],
    ["panicking-command-ingress", reconcile, patches, reactor.replace("advance_close_one()", 'let previous = ingress.borrow_mut()[retained_slot].replace(retained); assert!(previous.is_none(), "command ingress has one exact owner");')],
    ["panicking-timer-owner", reconcile, patches, reactor.replace("let Some(tail) = self.slots.get_mut(previous) else {\n                let _ = self.slots.take(index);\n                return Err(id);\n            };", "let tail = self.slots.get_mut(previous).expect(\"fixed timer tail authority\");")],
  ];
  const schemaMutations: [string, string][] = [
    ["btree-schema", value.replace("Map(UiMap)", "Map(std::collections::BTreeMap<String, UiValue>)")],
    ["dynamic-list-schema", value.replace("List(UiList)", "List(Vec<UiValue>)")],
    ["dynamic-page-backing", value.replace("pages: Box<[UiPageSlot]>", "pages: Vec<UiPageSlot>")],
    ["dynamic-fixed-list-taxonomy", value.replace("storage: fixed_list_storage::PagedList<T, N>", "items: Vec<Option<T>>")],
    ["dynamic-fixed-map-taxonomy", value.replace("entries: UiFixedList<(UiText, V)>", "entries: Box<UiFixedList<(UiText, V)>>").replace("Self { entries: UiFixedList::default() }", "Self { entries: Box::new(UiFixedList::default()) }")],
    ["dynamic-fixed-bytes-taxonomy", value.replace("bytes: Box<[u8]>", "bytes: Vec<u8>")],
    ["arc-page-alias", value.replace("handle: Option<UiCollectionHandle>", "backing: Option<Arc<UiPageSlot>>")],
    ["missing-map-max-refusal", value.replace("if entries.is_empty() {\n            Ok(Self::default())\n        } else {\n            Err(entries)\n        }", "Ok(Self::default())")],
    ["whole-list-constructor", value.replace("if values.is_empty() {\n            Ok(Self::default())\n        } else {\n            Err(values)\n        }", "for value in values { builder.push(value).unwrap(); } Ok(builder.finish())")],
    ["whole-map-sort", value.replace("if entries.is_empty() {\n            Ok(Self::default())\n        } else {\n            Err(entries)\n        }", "entries.sort_by(|a, b| a.0.cmp(&b.0)); Ok(Self::default())")],
    ["ordinary-list-drop", value.replace("impl Drop for UiList {", "impl UiList {")],
    ["ordinary-map-drop", value.replace("impl Drop for UiMap {", "impl UiMap {")],
    ["missing-value-credit", value.replace("aliases: u64", "aliases: ()")],
    ["clone-abort", value.replace("pub fn credited_clone(&self) -> Option<Self> {", "pub fn credited_clone(&self) -> Option<Self> { if false { std::process::abort(); }")],
    ["whole-serde-list", value.replace("let mut sequence = serializer.serialize_seq(Some(self.len))?;", "return serializer.collect_seq(self.iter()); let mut sequence = serializer.serialize_seq(Some(self.len))?;")],
    ["whole-serde-source", value.replace("let Some(mut builder) = UiListBuilder::try_new()", "let _whole = Vec::<UiValue>::deserialize(deserializer); let Some(mut builder) = UiListBuilder::try_new()")],
    ["missing-value-retirement-driver", value.replace("pub fn close_ui_value_page_one()", "fn close_ui_value_page_one()")],
    ["semantic-byte-fiction", value.replaceAll("let bytes = size_of::<UiPageSlot>();", "let bytes = self.pages.len().saturating_mul(size_of::<UiPageSlot>());")],
    ["missing-arena-max-fixture", value.replace("fixed_page_max_plus_one_returns_the_exact_untransferred_owner", "fixed_page_max_smoke")],
    ["missing-arena-poison-fixture", value.replace("poisoned_arena_lock_recovers_without_losing_fixed_authority", "arena_poison_smoke")],
  ];
  const documentMutations: [string, string, string][] = [
    ["dynamic-document-slots", schema.replace("slots: [UiDocumentSlot; UI_DOCUMENT_LEASE_SLOTS]", "slots: Vec<UiDocumentSlot>"), reactor],
    ["missing-document-generation", schema.replace("impl UiDocumentLease {\n    pub fn generation(&self) -> u64", "impl UiDocumentLease {\n    fn generation(&self) -> u64"), reactor],
    ["whole-document-read", schema.replace("pub fn read_node_page", "pub fn read_whole_document"), reactor],
    ["missing-document-close-driver", schema, reactor.replaceAll("ui_contract::close_ui_document_page_one()", "true")],
    ["sync-whole-patch-apply", schema.replace("#[cfg(test)]\npub fn apply_patch(state: &mut crate::UiSnapshotState", "pub fn apply_patch(state: &mut crate::UiSnapshotState"), reactor],
    ["sync-whole-snapshot-validation", schema.replace("#[cfg(test)]\npub fn validate_snapshot", "pub fn validate_snapshot"), reactor],
    ["missing-retained-patch-census", schema.replace("fn census_patch_one(&mut self)", "fn census_patch_all(&mut self)"), reactor],
    ["missing-exact-patch-rejection", schema.replace("pub fn take_rejected(mut self) -> Result<UiPatchApplyRejected, Self>", "fn take_rejected(mut self) -> Result<UiPatchApplyRejected, Self>"), reactor],
    ["bulk-patch-retirement", schema.replace("fn retire_patch_one", "fn retire_all_patch_ops"), reactor],
    ["bulk-remove-subtree", schema.replace("fn remove_subtree_one(&mut self)", "fn remove_subtree_all(&mut self)"), reactor],
    ["whole-patch-validation", schema.replace("fn validate_candidate_one(&mut self)", "fn validate_candidate_all(&mut self)"), reactor],
    ["dynamic-patch-validation-stack", schema.replace("validation_stack: crate::UiFixedList<UiPatchValidateFrame", "validation_stack: Vec<UiPatchValidateFrame"), reactor],
    ["missing-patch-max-fixture", schema.replaceAll("retained_patch_max_plus_one_returns_exact_state_and_patch", "retained_patch_max_smoke"), reactor],
    ["missing-patch-stale-fixture", schema.replaceAll("retained_patch_cancel_stale_and_deadline_preserve_owner", "retained_patch_stale_smoke"), reactor],
    ["missing-patch-remove-fixture", schema.replaceAll("retained_patch_remove_advances_one_node_or_child_per_opportunity", "retained_patch_remove_smoke"), reactor],
    ["dynamic-patch-handback-slots", schema.replace("slots: [UiPatchApplySlot; UI_PATCH_APPLY_SLOTS]", "slots: Vec<UiPatchApplySlot>"), reactor],
    ["missing-patch-drop-handback", schema.replace("impl Drop for UiPatchApplyProducer", "impl UiPatchApplyProducer"), reactor],
    ["missing-rejected-remove-owner", schema.replaceAll("remove_record: Option<crate::UiNodeRecord>", "remove_record: ()"), reactor],
    ["missing-patch-handback-driver", schema, reactor.replaceAll("semio_ui_contract_validation::close_ui_patch_owner_one()", "true")],
    ["missing-patch-handback-max-fixture", schema.replaceAll("patch_handback_arena_max_plus_one_refuses_without_reusing_a_live_generation", "patch_handback_arena_smoke"), reactor],
    ["missing-patch-abandon-fixture", schema.replaceAll("abandoned_patch_owner_moves_to_incremental_handback_and_reopens_capacity", "abandoned_patch_owner_smoke"), reactor],
    ["panicking-built-child-owner", schema.replace("let Some(backing) = self.backing.as_mut() else { return Err(node) };", 'let backing = self.backing.as_mut().expect("fixed child backing follows its reservation");'), reactor],
    ["panicking-image-typestate", schema.replace("let HasAlt(alt) = alt;", 'let alt = Some(alt).expect("🚫️ ImageBuilder<HasAlt> invariant").0;'), reactor],
    ["panicking-built-child-cursor", schema.replace("owner.cursor = next;", "owner.cursor += 1;"), reactor],
    ["panicking-document-retire-cursor", schema.replace("self.retire_scalar = 1;", "self.retire_scalar += 1;"), reactor],
    ["dynamic-turn-patch-owner", schema.replace("pub ui_patches: UiTurnPatches", "pub ui_patches: Vec<UiPatch>"), reactor],
    ["missing-turn-patch-exact-push", schema.replaceAll("pub fn try_push_ui_patch(&mut self, patch: UiPatch) -> Result<(), UiPatch>", "pub fn push_ui_patch(&mut self, patch: UiPatch)"), reactor],
    ["missing-turn-patch-handback", schema.replaceAll("impl Drop for UiTurnPatches", "impl UiTurnPatches"), reactor],
    ["dynamic-turn-patch-retirement", schema.replaceAll("slots: [UiTurnPatchRetireSlot; UI_TURN_PATCH_RETIRE_SLOTS]", "slots: Vec<UiTurnPatchRetireSlot>"), reactor],
    ["missing-turn-patch-close-driver", schema, reactor.replaceAll("close_ui_turn_patch_owner_one()", "false")],
    ["borrowed-shard-turn-patch-owner", schema, reactor.replace("mut result: TurnResult", "result: &TurnResult").replace("let mut owner = std::mem::take(&mut result.ui_patches);", "let owner = UiTurnPatches::default();")],
    ["missing-shard-turn-patch-producer", schema, reactor.replace("UiTurnPatchTransportProducer::try_new(session, owner)", "Err(owner)")],
    ["whole-shard-turn-patch-serde", schema, reactor.replace("let mut owner = std::mem::take(&mut result.ui_patches);", "let _whole = serde_json::to_vec(&result.ui_patches); let mut owner = std::mem::take(&mut result.ui_patches);")],
    ["missing-turn-patch-token-publication", schema, reactor.replace("match patch_transport.take_ready()", "let ui_patches = patch_transport\n        .removed_take_ready()")],
    ["duplicate-turn-patch-token-publication", schema.replaceAll("if !self.ready || self.transferred", "if !self.ready"), reactor],
    ["turn-patch-transfer-before-fallible-metadata", schema, reactor.replace("let effects = serde_json::to_vec(&result.effects)", "let _premature = patch_transport.take_ready();\n    let effects = serde_json::to_vec(&result.effects)")],
    ["missing-renderer-turn-patch-consumer", schema, reactor.replace("UiTurnPatchTransportLease::try_from_token(&result.ui_patches, session)", "Err(\"renderer transport consumer removed\")")],
    ["missing-run-turn-patch-consumer", schema, reactor.replace("UiTurnPatchTransportLease::try_from_token(&result.ui_patches, *reported)", "Err(\"run transport consumer removed\")")],
    ["whole-turn-patch-consumer-serde", schema, reactor.replace("UiTurnPatchTransportLease::try_from_token(&result.ui_patches, session)", "serde_json::from_slice(&result.ui_patches)")],
    ["missing-abandoned-session-close", schema, reactor.replaceAll("close_ui_turn_patch_transport_session_one(", "missing_turn_patch_session_close(")],
    ["missing-mounted-turn-patch-close", schema, reactor.replace("semio_framework::kernel::close_ui_turn_patch_transport_one()", "false")],
    ["infallible-raw-turn-patch-complete", schema, reactor.replaceAll(".await.map_err(|_| KernelError::InvalidTransition)?", ".await")],
    ["missing-in-place-shard-turn-patch-bridge", schema, reactor.replace("let bridged = to_actor_turn_result_in_place(&mut result, actor_id, 0, 0).await", "let bridged = Err(semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new(\"missing-turn-patch-bridge\"), \"missing\"))")],
    ["dynamic-grid-tracks", schema.replace("pub columns: UiGridTracks", "pub columns: Vec<GridTrack>"), reactor],
    ["missing-grid-max-fixture", schema.replace("grid_tracks_max_plus_one_returns_the_exact_track", "grid_tracks_smoke"), reactor],
    ["dynamic-table-row-owner", schema.replaceAll("let mut row_cells = UiFixedList::default();", "let mut row_cells: Vec<UiText> = Vec::new();"), reactor],
    ["dynamic-table-row-actions", schema.replace("builder.try_row_action(action).map_err(|_| ui_assembly_error(\"table-window.row-actions\"))?", "builder.row_actions.push(action)"), reactor],
    ["renderer-table-row-action", schema.replace("let mut row_cells = UiFixedList::default();", "let _action: Vec<ui_wgpu::wgpu::ActionDescriptor> = Vec::new(); let mut row_cells = UiFixedList::default();"), reactor],
    ["whole-table-row-collect", schema.replace("tree_window_rows(builder, windows, Self::KIND_ID, entries, &slice, row)?", "tree_window_rows(builder, windows, Self::KIND_ID, &entries.iter().collect::<Vec<_>>(), &slice, row)?"), reactor],
    ["missing-table-window-claim", schema.replace("windows.claim_window(&path, entries.len())?;", ""), reactor],
    ["missing-table-window-slice", schema.replace("let slice = windows.sliced(&path, true, entries.len());", "let slice = windows.sliced(&path, true, usize::MAX);"), reactor],
    ["whole-table-row-json", schema.replace("let mut row_cells = UiFixedList::default();", "let _whole = serde_json::to_string(&cells); let mut row_cells = UiFixedList::default();"), reactor],
    ["missing-table-window-first-paint-fixture", schema.replace("table_kit_first_paint_stays_inside_the_body_node_budget_at_any_row_count", "table_first_paint_smoke"), reactor],
    ["missing-table-window-heavy-rows-fixture", schema.replace("table_kit_ends_a_window_of_heavy_rows_inside_the_body_item_budget", "table_heavy_rows_smoke"), reactor],
    ["recursive-command-value-list", schema.replace("UiCommandJsonFrame::List { cursor", "values.iter().map(|value| ui_value_to_json(value)); UiCommandJsonFrame::List { cursor"), reactor],
    ["dynamic-command-value-stack", schema.replace("stack: UiFixedList<UiCommandJsonFrame, UI_COMMAND_VALUE_DEPTH>", "stack: Vec<UiCommandJsonFrame>"), reactor],
    ["missing-command-value-cancel", schema.replace("if cancelled {\n            self.pending = None", "if false {\n                self.pending = None"), reactor],
    ["missing-command-value-deadline", schema.replace("if deadline_expired {\n            return UiCommandJsonStep::MoreWork", "if false {\n                return UiCommandJsonStep::MoreWork"), reactor],
    ["whole-command-value-no-yield", schema.replace("semio_framework_async::yield_once().await", "std::future::ready(()).await"), reactor],
    ["missing-command-value-depth-fixture", schema.replace("retained_ui_value_bridge_rejects_depth_plus_one_without_consuming_the_original", "command_depth_smoke"), reactor],
    ["missing-command-value-owner-fixture", schema.replace("retained_ui_value_bridge_cancel_and_deadline_preserve_the_original_owner", "command_owner_smoke"), reactor],
    ["missing-turn-patch-max-fixture", schema.replace("ui_turn_patches_max_plus_one_returns_the_exact_patch_owner", "ui_turn_patches_max_smoke"), reactor],
  ];
  const producerMutations: [string, string][] = [
    ["missing-document-producer", reconcile.replace("pub struct SurfaceDocumentProducer", "struct SurfaceDocumentProducer")],
    ["missing-document-stale-generation", reconcile.replace("cx.generation().0 != self.generation", "false")],
    ["missing-document-stale-revision", reconcile.replace("SurfaceDocumentFault::StaleRevision { expected: self.revision, actual: current.revision() }", "false")],
    ["whole-document-producer", reconcile.replace("current.shadow.document_root_matches(self.lease.as_ref())", "current.retained.values().cloned().collect::<Vec<_>>().get(self.next_node)")],
    ["missing-document-outcome", reconcile.replace("pub struct SurfaceDocumentOutcome", "struct SurfaceDocumentOutcome")],
  ];
  const baseline = interactivityLiveReconcileFailures(reconcile, patches, reactor, value, schema);
  const known = new Set(baseline);
  const problems: string[] = baseline.length === 0 ? [] : [`live source rejected before mutations: ${baseline.join("; ")}`];
  const check = (name: string, bound: boolean, mutated: () => string[]): void => {
    if (!bound) problems.push(`mutation ${name} made no source mutation`);
    else if (!mutated().some((failure) => !known.has(failure))) problems.push(`mutation ${name} was falsely accepted`);
  };
  for (const [name, mutatedReconcile, mutatedPatches, mutatedReactor] of mutations) check(name, mutatedReconcile !== reconcile || mutatedPatches !== patches || mutatedReactor !== reactor, () => interactivityLiveReconcileFailures(mutatedReconcile, mutatedPatches, mutatedReactor, value, schema));
  for (const [name, mutatedValue] of schemaMutations) check(name, mutatedValue !== value, () => interactivityLiveReconcileFailures(reconcile, patches, reactor, mutatedValue, schema));
  for (const [name, mutatedSchema, mutatedReactor] of documentMutations) check(name, mutatedSchema !== schema || mutatedReactor !== reactor, () => interactivityLiveReconcileFailures(reconcile, patches, mutatedReactor, value, mutatedSchema));
  for (const [name, mutatedReconcile] of producerMutations) check(name, mutatedReconcile !== reconcile, () => interactivityLiveReconcileFailures(mutatedReconcile, patches, reactor, value, schema));
  if (problems.length !== 0) throw new Error(`[verify interactivity] live reconcile self-test: ${problems.join("; ")}`);
}

try {
  const { test } = await import("bun:test");
  test("live reconcile interactivity", () => {
    interactivityLiveReconcileSelfTests(process.cwd());
  }, 180_000);
} catch (error) {
  if (!(error instanceof Error) || !error.message.includes("outside of the test runner")) throw error;
}
