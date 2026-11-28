import { lstatSync, readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { BundleScript, type ScriptCommand, type ScriptInvocation } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { advanceScriptInvocation } from "../../../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🟦️.ts";
import { scriptCompilerAdmissionControlV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { readOwnedBuildInvocationV1 } from "../../../🏗️build/📥️caller/🟦️.ts";
import { readBuildOwnerRegistryV1 } from "../../../🏗️build/🕸️graph/🚪️admission/🟦️.ts";
import { issueRepositoryScriptProvidersV1 } from "../../../🏃️process/🔌️providers/🟦️.ts";
import { loadCatalogBuildProvidersV1 } from "../../../../../../💻️os/🔨️modules/🔌️plugin/📇️registry/📣️publication/🏗️build/🟦️.ts";
import { descriptorEmitterBinaryNameV1, descriptorEmitterPinArgumentsV1, descriptorEmitterRequestV1, type DescriptorEmitterPinV1 } from "../../../../../../💻️os/🔨️modules/🔌️plugin/🖨️describe/🛠️emitter/📋️contract/🟦️.ts";
import { GENERATED_PLAYGROUNDS_PROJECTION, GENERATED_PLUGINS_PROJECTION, readGeneratedCatalogProjection, runtimeCatalogProjectionRootV1 } from "../../../../../../💻️os/🔨️modules/🔌️plugin/📇️registry/📖️catalog-view/🟦️.ts";
import { ACTIVATION_RECEIPT_FILE, builtPluginModulesRootIn, developmentRuntimeRoot, playgroundSessionsRootV1, preparedComponentVerdict, type PreparedComponentFacts } from "../../../../../../💻️os/🔨️modules/🧑‍💻dev/♻️activation/🟦️.ts";
import { frameworkOsEngineDeliveriesV1, frameworkOsEngineFacetV1 } from "../../../../../../💻️os/🔨️modules/🧑‍💻dev/🏗️builder/🌐️vite/📥️artifacts/🕸️engines/🟦️.ts";
import { loadDevContribution } from "../../../../../../💻️os/🔨️modules/🧑‍💻dev/🧩️contribution/📥️loading/🟦️.ts";
import { playgroundCatalog, resolvePlaygroundFilter } from "../../../../../../💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/🏗️build/📋️plan/🟦️.ts";
import { MODULE_SHARD_DIRECTORY } from "../../../../../../💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment/🟦️.ts";
import { SHARD_WORKER_FILE } from "../../../../../../💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/🌐️browser-bundle/🏗️materialization/🧾️contract/🟦️.ts";
import { shardWorkerPublicationV1 } from "../../../../../../💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/🌐️browser-bundle/🏗️materialization/🧵️shard-worker-source/🟦️.ts";
import { PREVIEW2_VENDOR_RELATIVE } from "../../../../../../💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/🌐️browser-bundle/🕸️imports/🟦️.ts";
import { FONT_ASSET } from "../../../../../../../🔨️modules/🖼️canvas/📦️packages/🟦️typescript/🔤️fonts/🟦️.ts";
import { fontAssetPublicationRootV1 } from "../../../../../../../🔨️modules/🖼️canvas/📦️packages/🟦️typescript/🔤️fonts/📦️publication/🟦️.ts";
import { publishedCargoArtifactDirectoryV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/📤️published/🟦️.ts";
import { repositoryCachePartitionRootDataV1 } from "../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📂️paths/🟦️.ts";
import { rootDevPlaygroundRequestV1 } from "../🌱️birth/🟦️.ts";
import { nativeHostFilesystemViewV1, nativeHostTaskV1 } from "../../../🎮️playground/🖥️native-host/🟦️.ts";
import { createNativeHostSourceEditReceiverV1 } from "../../../🎮️playground/🖥️native-host/🔁️churn/🟦️.ts";
import { issueStoreReceiptV1, settleStoreReceiptV1 } from "../../../🏗️build/🧾️store-receipt/🟨️.mjs";
import { nativeCallerPhaseV1 } from "../../../⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts";
import { runOwnedCommand } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️owned-execution/🟦️.ts";

function physicalKind(path: string): "directory" | "file" | "other" | null {
  try {
    const stat = lstatSync(path);
    return stat.isFile() ? "file" : stat.isDirectory() ? "directory" : "other";
  } catch (error) {
    if ((error as NodeJS.ErrnoException).code === "ENOENT") return null;
    throw error;
  }
}

function publicationMissing(root: string, files: readonly string[]): boolean {
  const kinds = files.map(file => physicalKind(join(root, file)));
  if (kinds.some(kind => kind !== null && kind !== "file")) throw Error("Expected a physical playground publication file");
  return kinds.includes(null);
}

async function ensureCatalog(workspace: string, invocation: ScriptInvocation): Promise<string> {
  const files = [GENERATED_PLUGINS_PROJECTION, GENERATED_PLAYGROUNDS_PROJECTION, "📣️publication.json", "🦀️catalog.json"];
  const configured = runtimeCatalogProjectionRootV1();
  const selected = configured === undefined ? undefined : resolve(configured);
  if (selected !== undefined && !publicationMissing(selected, files)) return selected;
  const readText = (path: string) => readFileSync(join(workspace, path), "utf8");
  const owners = readBuildOwnerRegistryV1(JSON.parse(readText("📋️build-owners.json")));
  const providers = await loadCatalogBuildProvidersV1(owners.catalogProviders.map(({ ownerRoot, sourcePath }) => ({ ownerRoot, sourcePath })), {
    readText,
    kind: path => physicalKind(join(workspace, path)),
    load: path => import(pathToFileURL(join(workspace, path)).href),
  });
  const matches = providers.filter(owner => selected === undefined || join(workspace, owner.publication.outputRoot) === selected);
  if (matches.length !== 1) throw Error("Select one declared playground catalog publication through SEMIO_PLUGIN_CATALOG");
  const owner = matches[0]!;
  const root = join(workspace, owner.publication.outputRoot);
  if (publicationMissing(root, files)) {
    const { GenerateScript } = await import("../../../../../../💻️os/🔨️modules/🔌️plugin/📇️registry/📽️projection/🟦️.ts");
    await new GenerateScript(join(workspace, owner.projectRoot), workspace, invocation, owner.publication).run([]);
  }
  return root;
}

async function ensureBrowserSupport(workspace: string, invocation: ScriptInvocation, mode: "prototype" | "release"): Promise<void> {
  const moduleRoot = builtPluginModulesRootIn(workspace, mode);
  if (publicationMissing(moduleRoot, [join(PREVIEW2_VENDOR_RELATIVE, ".semio-artifact.json"), join(MODULE_SHARD_DIRECTORY, SHARD_WORKER_FILE)]) || shardWorkerPublicationV1(readFileSync(join(moduleRoot, MODULE_SHARD_DIRECTORY, SHARD_WORKER_FILE), "utf8")) !== "current") {
    const { SupportScript } = await import("../../../../../../💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🚀️commands/🟦️.ts");
    const support = join(workspace, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript");
    await new SupportScript(support, workspace, invocation).run([]);
  }
  const products = repositoryCachePartitionRootDataV1(workspace, "products");
  if (!publicationMissing(fontAssetPublicationRootV1(products, mode), [".semio-artifact.json", FONT_ASSET])) return;
  const compiler = repositoryCachePartitionRootDataV1(workspace, "rust");
  const tool = join(workspace, "🧰️framework/🔨️modules/🖼️canvas/🔤️fonts/📦️packages/🦀️rust");
  if (publicationMissing(publishedCargoArtifactDirectoryV1(compiler, join(tool, "Cargo.toml"), mode), [".semio-artifact.json"])) {
    const { FontAssetToolBuildScript } = await import("../../../../../../../🔨️modules/🖼️canvas/🔤️fonts/📦️packages/🦀️rust/📜️script.ts");
    await new FontAssetToolBuildScript(tool, workspace, invocation).run([]);
  }
  const { FontsScript, ROOT } = await import("../../../../../../💻️os/🔨️modules/♾️infinite/🧪️tests/🔬️rust-package-commands/🟦️.ts");
  await new FontsScript(ROOT, workspace, invocation).run([]);
}

async function ensureRendererEngines(workspace: string, invocation: ScriptInvocation, variant: string, mode: "prototype" | "release"): Promise<void> {
  const contribution = loadDevContribution(workspace, resolvePlaygroundFilter(variant));
  const facet = frameworkOsEngineFacetV1(contribution?.browserSessionFactories ?? []);
  const products = repositoryCachePartitionRootDataV1(workspace, "products");
  for (const delivery of frameworkOsEngineDeliveriesV1(products, { version: 1, mode, facet })) {
    if (!publicationMissing(delivery.root, [".semio-artifact.json", ...delivery.files])) continue;
    const owner = join(workspace, delivery.sourceOwner);
    const { WasmScript }: { WasmScript?: unknown } = await import(pathToFileURL(join(owner, "📜️script.ts")).href);
    if (typeof WasmScript !== "function" || !(WasmScript.prototype instanceof BundleScript)) {
      throw Error(`Declared browser engine requires an owned wasm producer for ${delivery.packageName}`);
    }
    const Producer = WasmScript as ScriptCommand;
    await new Producer(owner, workspace, invocation).run([]);
  }
}

/** 🧩️ Selects only the components needed to boot whose browser publication is missing or incomplete. */
export function missingPlaygroundBootComponentsV1(boot: ReadonlySet<string>, factsFor: (pluginId: string) => PreparedComponentFacts): ReadonlySet<string> {
  return new Set([...boot].filter(pluginId => preparedComponentVerdict(factsFor(pluginId)).kind !== "prepared"));
}

/** 🛠️ Checks whether the selected compiler prerequisite is physically published for this host. */
export function playgroundDescriptorEmitterMissingV1(products: string, mode: "prototype" | "release", platform: string, missing = publicationMissing): boolean {
  const intent = { version: 1, mode, platform };
  const { path } = descriptorEmitterRequestV1(products, intent);
  return missing(path, [".semio-artifact.json", descriptorEmitterBinaryNameV1(intent)]);
}

/** 🏭️ Runs the declared `generate` or generator requests of the mounted Rust modules that were missing at the scan, as neutral children of this launch. */
export async function ensurePlaygroundGeneratedInputsV1(workspace: string, invocation: ScriptInvocation, requests: readonly (readonly string[])[]): Promise<void> {
  if (requests.length === 0) return;
  const { runOwnedBuildChildInvocationV1 } = await import("../../../🏗️build/📥️caller/📤️child/🟦️.ts");
  const control = scriptCompilerAdmissionControlV1(invocation, "playground:generated-inputs", 0);
  for (const args of requests) await runOwnedBuildChildInvocationV1(workspace, args, invocation, control);
}

async function preparePlaygroundModels(workspace: string, invocation: ScriptInvocation, missing: ReadonlySet<string>, mode: "prototype" | "release", facet: "browser" | "model", generated: readonly (readonly string[])[], emitterPin?: DescriptorEmitterPinV1): Promise<void> {
  if (missing.size === 0) return;
  await ensurePlaygroundGeneratedInputsV1(workspace, invocation, generated);
  const { buildComponentClosureV1 } = await import("../../../../../../💻️os/🔨️modules/🔌️plugin/🏗️build/🧩️driver-stages/🟦️.ts");
  if (facet === "browser") {
    const { ensureNativeTranspileV1 } = await import("../../../../../../💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🦀️native-transpile/📜️script.ts");
    await ensureNativeTranspileV1(workspace, invocation, scriptCompilerAdmissionControlV1(invocation, "playground:native-transpile", 0));
  }
  const products = repositoryCachePartitionRootDataV1(workspace, "products");
  if (emitterPin === undefined && playgroundDescriptorEmitterMissingV1(products, mode, process.platform)) {
    const { buildDescriptorEmitterV1 } = await import("../../../../../../💻️os/🔨️modules/🔌️plugin/🖨️describe/🛠️emitter/🟦️.ts");
    await buildDescriptorEmitterV1(workspace, invocation,
      scriptCompilerAdmissionControlV1(invocation, "playground:descriptor-emitter", 0));
  }
  const { repositoryCargoNetworkV1 } = await import("../../../📦️packages/🟦️typescript/🦀️cargo/🟦️.ts");
  const control = scriptCompilerAdmissionControlV1(invocation, "playground:components", 0);
  const compile = () => buildComponentClosureV1(workspace, missing, {
    facet, invocation, ...(emitterPin === undefined ? {} : { emitterPin }),
    network: repositoryCargoNetworkV1(invocation, workspace),
    control,
  });
  try {
    await compile();
  } catch (error) {
    const { NativeBuildStorageShortfallV1 } = await import("../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/💾️storage-gate/🟦️.ts");
    if (!(error instanceof NativeBuildStorageShortfallV1) || facet !== "browser" || mode !== "prototype") throw error;
    const { recoverRepositoryRustPassStorageV1 } = await import("../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/♻️reset/🟦️.ts");
    if (await recoverRepositoryRustPassStorageV1(invocation, control, error.minimumFreeBytes) !== true) throw error;
    await compile();
  }
}

/** 📦️ Builds missing browser boot components through their managed driver using the original launch. */
async function ensureBootComponents(workspace: string, invocation: ScriptInvocation, variant: string, mode: "prototype" | "release", generated: readonly (readonly string[])[], emitterPin?: DescriptorEmitterPinV1): Promise<boolean> {
  const { playgroundComponentClosureV1 } = await import("../../../../../../💻️os/🔨️modules/🔌️plugin/🏗️build/🧩️driver-stages/🟦️.ts");
  const { stagedComponentFacts } = await import("../../../../../../💻️os/🔨️modules/🧑‍💻dev/♻️activation/🔍️freshness/🟦️.ts");
  const modules = builtPluginModulesRootIn(workspace, mode);
  const boot = playgroundComponentClosureV1(variant, "boot");
  const selected = emitterPin === undefined ? missingPlaygroundBootComponentsV1(boot, id => stagedComponentFacts(modules, id)) : boot;
  await preparePlaygroundModels(workspace, invocation, selected, mode, "browser", generated, emitterPin);
  return selected.size > 0;
}

const WGPU_RENDERER = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu";

async function ensureWgpuRenderer(workspace: string, invocation: ScriptInvocation, mode: "prototype" | "release", generated: readonly (readonly string[])[]): Promise<void> {
  await ensurePlaygroundGeneratedInputsV1(workspace, invocation, generated);
  const renderer = join(workspace, WGPU_RENDERER);
  const rustOwner = join(renderer, "📦️packages/🦀️rust");
  const bootFiles = ["🚀️browser-boot", "🎞️frame-worker", "🎬️renderer-boot"].map(part => join(part, "🤖️generated", "🟨️.js"));
  if (publicationMissing(renderer, bootFiles)) {
    const { createUiNativeRouter } = await import("../../../../../../../🔨️modules/🖱️ui/📦️packages/🦀️rust/📜️script.ts");
    await createUiNativeRouter(join(workspace, "🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust"), workspace).run(["generate"], invocation);
    const { GenerateBrowserBootScript, GenerateRendererBootScript } = await import("../../../../../../💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⚙️browser-build/📜️script.ts");
    const { generateFrameWorker } = await import("../../../../../../💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎞️frame-worker/🏗️builder/🟦️.ts");
    await new GenerateBrowserBootScript(rustOwner, workspace, invocation).run([]);
    await generateFrameWorker(join(renderer, "📦️packages/🟦️typescript"));
    await new GenerateRendererBootScript(rustOwner, workspace, invocation).run([]);
  }
  const products = repositoryCachePartitionRootDataV1(workspace, "products");
  const { frameworkOsViteArtifactPathsV1 } = await import("../../../../../../💻️os/🔨️modules/🧑‍💻dev/🏗️builder/🌐️vite/📥️artifacts/🟦️.ts");
  const publication = frameworkOsViteArtifactPathsV1(products, mode).renderer;
  if (publicationMissing(publication.directory, [".semio-artifact.json", "semio-renderer-wgpu-component.js", "semio-renderer-wgpu-component_bg.wasm"])) {
    const { buildWasmRenderer } = await import("../../../../../../💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🏗️compiler/🌐️wasm/📜️script.ts");
    const control = scriptCompilerAdmissionControlV1(invocation, "playground:wgpu-wasm", 0);
    await buildWasmRenderer({ rustPackageRoot: rustOwner, componentRoot: join(renderer, "🧩️component/📦️packages/🦀️rust"), workspace, profile: mode, invocation, control });
  }
}

async function serveWgpuPlayground(workspace: string, invocation: ScriptInvocation, variant: string, mode: "prototype" | "release", restaged: boolean): Promise<void> {
  const playground = playgroundCatalog().find(row => row.variant === variant);
  if (!playground) throw Error(`Missing generated playground ${variant}`);
  const runtime = developmentRuntimeRoot(workspace, variant, mode, "wgpu");
  if (restaged || publicationMissing(join(runtime, "activation"), [ACTIVATION_RECEIPT_FILE])) {
    const bundle = join(workspace, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript");
    const lane = [variant, "wgpu", mode];
    const { PreparationScript } = await import("../../../../../../💻️os/🔨️modules/🧑‍💻dev/♻️activation/🧰️preparation/🟦️.ts");
    const { ActivationScript } = await import("../../../../../../💻️os/🔨️modules/🧑‍💻dev/♻️activation/🏃️execution/🟦️.ts");
    await new PreparationScript(bundle, workspace, invocation).run(lane);
    await new ActivationScript(bundle, workspace, invocation).run(lane);
  }
  const { ServeScript } = await import("../../../../../../💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️server/📜️script.ts");
  const args = [variant, mode, "--port", String(playground.ports.wgpu)];
  if (playground.compositionConfigPath !== undefined) args.push("--composition", playground.compositionConfigPath);
  await new ServeScript(join(workspace, WGPU_RENDERER, "🌐️server"), workspace, invocation).run(args);
}

async function buildPlaygroundNativeHost(workspace: string, invocation: ScriptInvocation, mode: "prototype" | "release", host: unknown): Promise<void> {
  const { repositoryCargoNetworkV1 } = await import("../../../📦️packages/🟦️typescript/🦀️cargo/🟦️.ts");
  if (host === undefined) {
    const { buildNativeRenderer } = await import("../../../../../../💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🏗️compiler/🦀️native/📜️script.ts");
    const rust = join(workspace, WGPU_RENDERER, "📦️packages/🦀️rust");
    await buildNativeRenderer(rust, mode, workspace, scriptCompilerAdmissionControlV1(invocation, "playground:native-renderer", 0), repositoryCargoNetworkV1(invocation, workspace), invocation);
    return;
  }
  const recipe = nativeHostTaskV1(host, mode, nativeHostFilesystemViewV1(workspace));
  const declaration = { workspaceRoot: workspace, projectRoot: recipe.host.cratePath, target: recipe.host.target, outputs: recipe.outputs };
  const issued = issueStoreReceiptV1(declaration), startedAtMs = Date.now();
  const label = "playground:native-host";
  const phase = nativeCallerPhaseV1(invocation, workspace, scriptCompilerAdmissionControlV1(invocation, label, 0));
  const sourceEdit = createNativeHostSourceEditReceiverV1(workspace);
  const environment = { ...process.env, ...issued, ...sourceEdit.environment, SEMIO_WORKSPACE_ROOT: workspace, SEMIO_BUILD_MODE: mode, SEMIO_TASK_PROJECT: recipe.host.project, SEMIO_TASK_TARGET: recipe.host.target };
  try {
    await phase.advance(label);
    await runOwnedCommand(process.execPath, [join(workspace, recipe.script), ...recipe.arguments], join(workspace, recipe.host.cratePath), label, 0, { ...phase.options(environment), deadline: phase.leaseControl, onLine: sourceEdit.observe });
    phase.budget();
    await settleStoreReceiptV1(declaration, issued, { succeeded: true, startedAtMs });
  } catch (error) {
    await settleStoreReceiptV1(declaration, issued, { succeeded: false, startedAtMs }).catch(secondary => {
      throw new AggregateError([error, secondary], "Native host build and receipt retirement failed");
    });
    if (!phase.leaseControl.signal.aborted) throw sourceEdit.restore(error) ?? error;
    throw error;
  }
}

async function prepareNativeRuntimeAssets(workspace: string, invocation: ScriptInvocation): Promise<void> {
  const { RUNTIME_ASSET_CHANNELS } = await import("../../../../../../../🔨️modules/🖼️canvas/🔌️runtime-assets/🟦️.ts");
  for (const { id, directory, files } of RUNTIME_ASSET_CHANNELS) {
    const path = join(workspace, directory);
    if (!publicationMissing(path, files)) continue;
    if (id === "icon-pack") {
      const { runGenerateCatalogScript } = await import("../../../../../../../🔨️modules/🖼️assets/🧪️tests/🔬️typescript-package-commands/🟦️.ts");
      const root = join(workspace, "🧰️framework/🔨️modules/🖼️assets/📦️packages/🟦️typescript");
      runGenerateCatalogScript({ root, repoRoot: workspace, invocation }, ["rust"]);
    }
    if (publicationMissing(path, files)) throw Error(`Missing native runtime assets in ${directory}`);
  }
}

async function runNativePlayground(workspace: string, invocation: ScriptInvocation, variant: string, mode: "prototype" | "release", generated: readonly (readonly string[])[]): Promise<void> {
  const { withPlaygroundSessionV1 } = await import("../../../../../../💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🧭️session/👓️reading/🟦️.ts");
  const { componentModelLayoutV1, componentModelGenerationPathV1 } = await import("../../../../../../💻️os/🔨️modules/🔌️plugin/📦️model/📋️contract/🟦️.ts");
  const { readComponentModelHeadFileV1 } = await import("../../../../../../💻️os/🔨️modules/🔌️plugin/📦️model/🧭️head/🟦️.ts");
  const { default: modelSchema } = await import("../../../../../../💻️os/🔨️modules/🔌️plugin/📦️model/🧬️schema/🔣️.json", { with: { type: "json" } });
  const products = repositoryCachePartitionRootDataV1(workspace, "products");
  const { missing, host } = await withPlaygroundSessionV1(workspace, invocation, variant, async (session, projection) => {
    const row = projection.playgrounds.find(playground => playground.variant === variant);
    if (!row) throw Error(`Missing native playground ${variant}`);
    const missing = new Set<string>();
    for (const { pluginId } of session.plugins) {
      const entry = projection.entries.find(candidate => candidate.pluginId === pluginId);
      if (!entry) throw Error(`Missing Native component declaration: ${pluginId}`);
      const layout = componentModelLayoutV1(products, { version: 1, mode, pluginId, cratePath: entry.cratePath });
      const head = await readComponentModelHeadFileV1(layout.head);
      if (!head || publicationMissing(componentModelGenerationPathV1(layout, head.generation), [".semio-artifact.json", ...Object.values(modelSchema.definitions.Protocol.const)])) missing.add(pluginId);
    }
    return { missing, host: row.nativeHost };
  });
  await preparePlaygroundModels(workspace, invocation, missing, mode, "model", generated);
  const { publishNativePlaygroundRuntimeV1 } = await import("../../../../../../💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/📦️modules/🟦️.ts");
  await publishNativePlaygroundRuntimeV1(workspace, invocation, variant, invocation.control.signal);
  await buildPlaygroundNativeHost(workspace, invocation, mode, host);
  const { runRunScript } = await import("../../../../../../💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/🟦️.ts");
  await runRunScript({ root: join(workspace, WGPU_RENDERER, "⌨️native-entrypoint"), repoRoot: workspace, invocation }, [variant, mode]);
}

/** 🎮️ Prepares and runs the publications required by the original playground request. */
export async function serveRootPlaygroundV1(workspace: string, invocation: ScriptInvocation, argv: readonly string[]): Promise<void> {
  const request = rootDevPlaygroundRequestV1(argv);
  if (!request) throw Error("usage: dev playground <variant> <react|wgpu|native> <prototype|release> [--published-emitter <binary:receipt:index> for React prototype]");
  const original = readOwnedBuildInvocationV1(workspace, invocation, argv);
  if (original.buildMode !== request.profile) throw Error("Playground profile must match the original native request");
  issueRepositoryScriptProvidersV1(process.env);
  const previousCatalog = process.env.SEMIO_PLUGIN_CATALOG;
  process.env.SEMIO_PLUGIN_CATALOG = await ensureCatalog(workspace, invocation);
  try {
    const sessionRoot = join(playgroundSessionsRootV1(workspace, request.profile), request.variant);
    const { refreshPlaygroundSessionPublicationV1, stagePlaygroundSession } = await import("../../../../../../💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🧭️session/🟦️.ts");
    const projection = readGeneratedCatalogProjection();
    if (!projection.playgrounds.some(row => row.variant === request.variant)) throw Error(`Selected playground requires current compiled descriptors: ${request.variant}`);
    const sessionRefreshed = await refreshPlaygroundSessionPublicationV1(projection, async () => {
      if (publicationMissing(sessionRoot, [".semio-artifact.json"])) return undefined;
      const { retainedPlaygroundSessionProjectionV1 } = await import("../../../../../../💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🧭️session/👓️reading/🟦️.ts");
      return retainedPlaygroundSessionProjectionV1(workspace, invocation, request.variant);
    }, async current => {
      const { session } = await stagePlaygroundSession(workspace, invocation, request.variant, current);
      console.log(`Playground session ${request.variant}: ${session.plugins.length} plugins staged`);
    });
    const { generatedPrerequisiteArgumentsV1 } = await import("../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🏭️generated-owner/🟦️.ts");
    const { generatedScanPortsV1 } = await import("../📡️scan-feed/🟦️.ts");
    const generated = await generatedPrerequisiteArgumentsV1(workspace, {
      ...generatedScanPortsV1(workspace),
      signal: invocation.control.signal,
      report: stage => advanceScriptInvocation(invocation, "playground:generated-inputs", stage),
    });
    if (request.renderer === "native") {
      await prepareNativeRuntimeAssets(workspace, invocation);
      await runNativePlayground(workspace, invocation, request.variant, request.profile, generated);
      return;
    }
    await ensureBrowserSupport(workspace, invocation, request.profile);
    if (request.renderer === "wgpu") await ensureWgpuRenderer(workspace, invocation, request.profile, generated);
    const emitterPin = request.publishedEmitter === undefined ? undefined : descriptorEmitterPinArgumentsV1(request.publishedEmitter, request.profile, process.platform);
    const restaged = await ensureBootComponents(workspace, invocation, request.variant, request.profile, generated, emitterPin);
    const activationRequired = restaged || sessionRefreshed;
    if (request.renderer === "wgpu") {
      await serveWgpuPlayground(workspace, invocation, request.variant, request.profile, activationRequired);
      return;
    }
    await ensureRendererEngines(workspace, invocation, request.variant, request.profile);
    const bundle = join(workspace, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript");
    const lane = [request.variant, request.renderer, request.profile];
    if (activationRequired) {
      const { ActivationScript } = await import("../../../../../../💻️os/🔨️modules/🧑‍💻dev/♻️activation/🏃️execution/🟦️.ts");
      await new ActivationScript(bundle, workspace, invocation).run(lane);
    }
    const { ServeScript } = await import("../../../../../../💻️os/🔨️modules/🧑‍💻dev/♻️activation/🌐️serve/🟦️.ts");
    await new ServeScript(bundle, workspace, invocation).run(lane);
  } finally {
    if (previousCatalog === undefined) delete process.env.SEMIO_PLUGIN_CATALOG;
    else process.env.SEMIO_PLUGIN_CATALOG = previousCatalog;
  }
}
