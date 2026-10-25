/** 🔶️ The macro files phase of toolJobCoverageRun. */
import { readMcpGatewaySource } from "../../../../../../../💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/🟦️.ts";
import { policyReadFileSafe, policyReadModuleTreeSource, policyReadWitPackageSource, POLICY_SPLIT_SOURCE_ROOTS } from "../../../../🔍️discovery/📖️source-access/🟦️.ts";
import { relative } from "node:path";
import { readSourceFamilySource } from "../../../../../../../../🔨️modules/📚️compiler/📦️packages/🟦️typescript/📖️syntax/🌳️module-tree/🟦️.ts";
import { policyAllRustFiles } from "../../../🧩️plugin-consistency/🏛️foundation/🟦️.ts";
import { policyReadOsStoreSource } from "../../../../🔍️discovery/📖️source-access/🦀️rust-evidence/🟦️.ts";
import { toolJobRasterEnvelopeSourceEvidence } from "../../🗄️artifact-store/🖼️caller-retained-media/🟦️.ts";
import { toolJobPluginFamilySource, toolJobPluginProductionSources } from "../../🏭️factory/🟦️.ts";
import { type FixedOperationFixtureOutput, toolJobDrawingGestureOperationOwnerExact, toolJobFixedOperationFixtureRun, toolJobFixedOperationRegistryExact, toolJobFixedOperationRustFixtureSource, toolJobSharedFrameworkActionFixtureRun } from "../../⚙️fixed-operation/🟦️.ts";
import { toolJobAppOwnedRows, toolJobDispositions, toolJobGlobalPayloadStoreInventory, toolJobGlobalPayloadStoreIsSharedFixedAuthority, toolJobProductionSource, toolJobProofIdentity, toolJobProofs, toolJobRustBlock, toolJobScanThenMonolithRows, toolJobStaticRows } from "../../📋️inventory/🟦️.ts";
import type { SelfTestLanes } from "../../../🧪️self-test-lanes/🟦️.ts";
import { toolJobArtifactRetainedCommandExact, toolJobArtifactRetainedCommandRuntimeLawExact } from "../🟠️harness/🟦️.ts";

/** 🔷 Builds macroFiles, fixtureMacroFiles, commandRows, fixtureCommandRows of toolJobCoverageRun. */
export async function buildMacroFilesState({ root, lanes }: { readonly root: string; readonly lanes: SelfTestLanes }) {
  toolJobSharedFrameworkActionFixtureRun(root);
  const macroFiles = new Set<string>();
  const fixtureMacroFiles = new Set<string>();
  const commandRows: { file: string; id: string }[] = [];
  const fixtureCommandRows: { file: string; id: string }[] = [];
  let macroInvocations = 0;
  let fixtureMacroInvocations = 0;
  const fixtureHost = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs";
  for (const file of policyAllRustFiles(root)) {
    const source = policyReadFileSafe(root, file);
    const invocation = /^\s*(?:[A-Za-z_][A-Za-z0-9_]*::)*app_commands!\s*\{/gm;
    let match: RegExpExecArray | null;
    while ((match = invocation.exec(source))) {
      const open = source.indexOf("{", match.index);
      const block = toolJobRustBlock(source, open);
      if (!block) break;
      const fixture = file === fixtureHost;
      (fixture ? fixtureMacroFiles : macroFiles).add(file);
      if (fixture) fixtureMacroInvocations += 1;
      else macroInvocations += 1;
      const rows = /^\s*"([^"]+)"(?:\s+as\s+"[^"]+")?\s*=>/gm;
      let row: RegExpExecArray | null;
      while ((row = rows.exec(block.body))) (fixture ? fixtureCommandRows : commandRows).push({ file, id: row[1]! });
      invocation.lastIndex = block.end;
    }
  }
  const duplicateRows: { file: string; id: string }[] = [];
  const seenRows = new Set<string>();
  for (const row of commandRows) {
    const key = `${row.file}\0${row.id}`;
    if (seenRows.has(key)) duplicateRows.push(row);
    seenRows.add(key);
  }
  const intentionalWriterDuplicates = duplicateRows.filter((row) => row.file.includes("✏️s/🔌️plugins/✒️writer/") && row.id === "setEditorSetting");
  const productionFiles = toolJobPluginProductionSources(root);
  const staticRows = toolJobStaticRows(productionFiles);
  const appOwnedRows = toolJobAppOwnedRows(productionFiles);
  const registeredAppOwnedKeys = new Set(appOwnedRows.map((row) => `${row.ownerFile}\0${row.toolId}`));
  const appOwnedKeys = new Set(appOwnedRows.filter((row) => row.publicationReady).map((row) => `${row.ownerFile}\0${row.toolId}`));
  const dispositions = toolJobDispositions(productionFiles);
  const batchOnlyRows = commandRows.filter((row) => dispositions.get(`${row.file}\0${row.id}`) === "BatchOnlyPendingRewrite").length;
  const forbiddenRows = commandRows.filter((row) => dispositions.get(`${row.file}\0${row.id}`) === "ForbiddenFromUi").length;
  const deletedRows = commandRows.filter((row) => dispositions.get(`${row.file}\0${row.id}`) === "Deleted").length;
  const actionBus = readSourceFamilySource(`${root}/🧰️framework/🔨️modules/🎯️action-bus`, { tests: true });
  const platform = policyReadFileSafe(root, "🧰️framework/🔨️modules/🖥️platform/🦀️.rs");
  const glue = policyReadFileSafe(root, "🧰️framework/📦️packages/🦀️rust/🦀️.rs");
  const plugin = toolJobPluginFamilySource(root);
  const artifactRetainedCommand = policyReadModuleTreeSource(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🦀️.rs");
  const store = policyReadOsStoreSource(root);
  const presenceRetirement = policyReadFileSafe(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👥️presence/♻️retirement/🦀️.rs");
  const vcs = policyReadModuleTreeSource(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🦀️.rs");
  const causal = policyReadFileSafe(root, "🧰️framework/🔨️modules/📡️replication/🔗️causal/🦀️.rs");
  const channel = readSourceFamilySource(`${root}/🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel`);
  const actor = policyReadModuleTreeSource(root, "🧰️framework/🔨️modules/🎭️actor/🦀️.rs");
  const kernel = policyReadFileSafe(root, "🧰️framework/🔨️modules/🎠️kernel/🦀️.rs");
  const pluginHost = policyReadModuleTreeSource(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs");
  const mcpWorkspace = readMcpGatewaySource(root, (file) => policyReadFileSafe(root, relative(root, file)));
  const runHost = policyReadModuleTreeSource(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️.rs");
  const wgpuHost = policyReadModuleTreeSource(root, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs");
  const nativeIo = policyReadFileSafe(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🚪️native-io/🦀️.rs");
  const shardHost = policyReadModuleTreeSource(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️.rs");
  const jobRuntime = policyReadModuleTreeSource(root, "🧰️framework/🔨️modules/🧵️job/🦀️.rs");
  const puzzle5d = policyReadModuleTreeSource(root, "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs");
  const puzzle2d = policyReadModuleTreeSource(root, "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs");
  const semio = policyReadFileSafe(root, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🦀️.rs");
  const manifest = policyReadModuleTreeSource(root, "🧰️framework/🔨️modules/🛂️manifest/🕹️actions/🦀️.rs");
  const componentWit = policyReadWitPackageSource(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema");
  const reactor = policyReadModuleTreeSource(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs");
  const family = (relative: string): string => readSourceFamilySource(`${root}/${relative}`, { tests: true });
  const reactorFamily = family("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor");
  const valueSnapshotRead = policyReadFileSafe(root, "🧰️framework/🔨️modules/🌱️value/📸️snapshot/👁️read/🦀️.rs");
  const reactorRequests = policyReadFileSafe(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐host/📮️requests/🦀️.rs");
  const reactorExecutor = policyReadFileSafe(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🧵️executor/🦀️.rs");
  const reactorJobs = policyReadFileSafe(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🦀️.rs");
  const fem2dEditorFolder = "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor";
  const fem2dSession = family(`${fem2dEditorFolder}/🧵️session`);
  const fem2dEditor = family(fem2dEditorFolder);
  const fem2dModel = family(`${fem2dEditorFolder}/🎭️modes/✏️edit/🪟️windows/🧱️model`);
  const femPluginRoot = policyReadFileSafe(root, "🌎️hub/🧩️compositions/🏗️fem/🦀️.rs");
  const femGlue = policyReadFileSafe(root, "🌎️hub/🧩️compositions/🏗️fem/📦️packages/🦀️rust/Cargo.toml");
  const femAnalyses = family("✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses");
  const femMesh = family("✏️s/🔨️modules/🏗️fem/⚙️engine/🕸️mesh");
  const femSparse = policyReadModuleTreeSource(root, "✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🦀️.rs");
  const femEngineModel = policyReadFileSafe(root, "✏️s/🔨️modules/🏗️fem/⚙️engine/🏗️model/🦀️.rs");
  const femElements2d = policyReadModuleTreeSource(root, "✏️s/🔨️modules/🏗️fem/⚙️engine/📏️elements2d/🦀️.rs");
  const presentationEnvelopeCodec = policyReadFileSafe(root, "✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs");
  const presentationWasm = policyReadFileSafe(root, "✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️.rs");
  const writerEnvelopeCodec = policyReadFileSafe(root, "✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs");
  const writerEditor = policyReadModuleTreeSource(root, "✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs");
  const writerWasm = policyReadFileSafe(root, "✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️.rs");
  const jackEnvelopeCodec = policyReadFileSafe(root, "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs");
  const jackEditor = policyReadModuleTreeSource(root, "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs");
  const jackWasm = policyReadFileSafe(root, "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️.rs");
  const trinityRewrite = policyReadFileSafe(root, "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌍️world/🦀️.rs");
  const gisMapEnvelopeCodec = policyReadFileSafe(root, "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs");
  const gisMapEditor = policyReadModuleTreeSource(root, "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs");
  const gisMapWasm = policyReadFileSafe(root, "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️.rs");
  const rasterEvidence = toolJobRasterEnvelopeSourceEvidence(root);
  const drawingEnvelopeCodec = policyReadFileSafe(root, "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🔨️modules/🏠️host/🧰️owned/🦀️.rs");
  const drawingEditorSources = [
    "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
    "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/✏️editor/🪆️1-any/🎮️commands/🖱️canvas-pointer-down/🦀️.rs",
    "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️canvas/🦀️.rs",
  ].map((file) => POLICY_SPLIT_SOURCE_ROOTS.includes(file) ? policyReadModuleTreeSource(root, file) : policyReadFileSafe(root, file));
  const drawingEditorTestLaws = policyReadFileSafe(root, "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs");
  const drawingEditor = drawingEditorSources
    .map(toolJobProductionSource)
    .join("\n");
  const drawingConfig = policyReadFileSafe(root, "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs");
  const drawingConfigProto = policyReadFileSafe(root, "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🛰️.proto");
  const allRustFiles = new Map(policyAllRustFiles(root).map((file) => [file, policyReadFileSafe(root, file)]));
  const proofs = toolJobProofs(productionFiles);
  const proofIdentities = new Set(proofs.map(toolJobProofIdentity));
  const scanThenMonolithRows = toolJobScanThenMonolithRows(productionFiles, proofs);
  const scanThenMonolithKeys = new Set(scanThenMonolithRows.map((row) => `${row.file}\0${row.id}`));
  let fixedOperationFixture: FixedOperationFixtureOutput | undefined;
  let fixedOperationRustFixtureFresh = false;
  try {
    fixedOperationFixture = toolJobFixedOperationFixtureRun(root);
    fixedOperationRustFixtureFresh =
      policyReadFileSafe(root, "🧰️framework/🔨️modules/🧵️job/🧪️tests/🧪️fixed-operation-registry-cases/🦀️.rs") === toolJobFixedOperationRustFixtureSource(root);
  } catch {
    fixedOperationFixture = undefined;
  }
  const selfTests = (await lanes.toolJobCoverageSelfTests()) + lanes.toolJobFixedOperationRegistrySelfTests(jobRuntime) + lanes.toolJobDrawingGestureOperationOwnerSelfTests() + lanes.toolJobArtifactRetainedCommandSelfTests(artifactRetainedCommand, plugin) + (fixedOperationFixture?.results.length ?? 0) + Number(fixedOperationRustFixtureFresh);
  const factoryContracts: { file: string; line: number; factory: string; status: "explicit" | "missing" }[] = [];
  for (const [file, source] of allRustFiles) {
    const implementation = /impl(?:<[^>]+>)?\s+(?:semio_framework::)?ToolJobFactory\s+for\s+([^\s<{]+(?:<[^>{]+>)?)\s*\{/g;
    let match: RegExpExecArray | null;
    while ((match = implementation.exec(source))) {
      const block = toolJobRustBlock(source, source.indexOf("{", match.index));
      if (!block) break;
      factoryContracts.push({ file, line: source.slice(0, match.index).split("\n").length, factory: match[1]!, status: /fn\s+execution_contract\s*\(/.test(block.body) ? "explicit" : "missing" });
      implementation.lastIndex = block.end;
    }
  }
  const globalPayloadStoreInventory = toolJobGlobalPayloadStoreInventory(productionFiles);
  const globalPayloadStoreExemptions = globalPayloadStoreInventory.filter(toolJobGlobalPayloadStoreIsSharedFixedAuthority);
  const globalPayloadStores = globalPayloadStoreInventory.filter((row) => !toolJobGlobalPayloadStoreIsSharedFixedAuthority(row));
  const failures: string[] = [];
  if (!toolJobArtifactRetainedCommandExact(artifactRetainedCommand) || !toolJobArtifactRetainedCommandRuntimeLawExact(plugin)) failures.push("shared app-owned retained command shell lacks fixed ARC1 checkpoints, bounded restore/replay, incremental close, executable runtime replay/cancel laws, or hostile laws");
  if (!toolJobFixedOperationRegistryExact(jobRuntime)) failures.push("fixed operation scheduler authority lacks exact operation/generation admission, owner handback, bounded close, or hostile laws");
  if (!fixedOperationFixture) failures.push("fixed operation scheduler authority lacks its schema-first empty/single/max/max+1/collision/cancel/stale/ABA/interrupted-close executable fixture");
  if (!fixedOperationRustFixtureFresh) failures.push("production FixedOperationRegistry does not execute the exact generated language-neutral law stream");
  if (!toolJobDrawingGestureOperationOwnerExact(drawingEditor, drawingConfig, drawingConfigProto, plugin)) failures.push("Drawing gesture operation state remains process-global or persisted through Config/Draft/document lanes instead of one instance-retained worker owner");
  if (staticRows.length < commandRows.length) failures.push(`literal registration census regressed below macro census (${staticRows.length} < ${commandRows.length})`);
  if (!plugin.includes("pub const TOOL_JOB_IDS: &'static [&'static str] = &[$($id),*];")) failures.push("app_commands! does not generate the exact ToolCommandCatalog row list");
  if ((plugin.match(/const TOOL_JOB_IDS: &'static \[&'static str\] = Self::TOOL_JOB_IDS;/g) ?? []).length !== 8) failures.push("app_commands! does not publish every generated row list through both typed catalog traits");
  if (!actionBus.includes("pub struct ActionBus {") || !actionBus.includes("trait ErasedToolJobFactory")) failures.push("ActionBus is not a heterogeneous erased-factory registry");
  if (actionBus.includes("NoToolJobFactories") || actionBus.includes("NoInteractiveJobs") || platform.includes("NoToolJobFactories") || glue.includes("NoToolJobFactories")) failures.push("a hard-coded no-factory sentinel remains in production framework wiring");
  return { platform, failures, plugin, actionBus, puzzle2d, store, semio, causal, vcs, allRustFiles, presentationEnvelopeCodec, presentationWasm, writerEnvelopeCodec, writerEditor, writerWasm, jackEnvelopeCodec, jackEditor, jackWasm, trinityRewrite, gisMapEnvelopeCodec, gisMapEditor, gisMapWasm, rasterEvidence, drawingEnvelopeCodec, drawingEditor, drawingEditorTestLaws, channel, presenceRetirement, kernel, reactor, pluginHost, componentWit, mcpWorkspace, runHost, wgpuHost, actor, shardHost, jobRuntime, nativeIo, fem2dSession, fem2dEditor, fem2dModel, femMesh, femPluginRoot, femGlue, reactorJobs, femAnalyses, reactorFamily, valueSnapshotRead, femSparse, femEngineModel, femElements2d, reactorRequests, reactorExecutor, productionFiles, staticRows, dispositions, proofs, proofIdentities, manifest, factoryContracts, globalPayloadStores, scanThenMonolithRows, appOwnedRows, appOwnedKeys, scanThenMonolithKeys, registeredAppOwnedKeys, macroFiles, macroInvocations, commandRows, seenRows, duplicateRows, fixtureMacroFiles, fixtureMacroInvocations, fixtureCommandRows, batchOnlyRows, forbiddenRows, deletedRows, globalPayloadStoreExemptions, selfTests };
}
