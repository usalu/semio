import { checkResourceLeaseControl } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import { terminateOwnedChildTree } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { compilerCapacityWorkerCountV1, compilerAdmissionBudgetV1, requestedCompilerAdmissionControlV1, type CompilerAdmissionControl } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { readBuildMode } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";
import { spawn } from "node:child_process";
import { createRequire } from "node:module";
import { fileURLToPath } from "node:url";
import { createHash } from "node:crypto";
import { copyFileSync, existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, renameSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join, relative } from "node:path";
import { getWorkspaceRoot } from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { stageRepositoryArtifacts } from "../../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🟦️.ts";
import { componentModuleOwnerV1, MODULE_BRIDGE_FILE, MODULE_SHARD_DIRECTORY, moduleDirectoryName } from "../../../📇️registry/📦️deployment/🟦️.ts";
import type { ComponentSourceOwnerV1 } from "../../../📇️registry/🔎️discovery/🟦️.ts";
import { pluginModulesRoot, pluginModulesRootIn, builtPluginModulesRootIn } from "../../../../🧑‍💻dev/♻️activation/🟦️.ts";
import { COMPONENT_PROFILE_POLICIES, COMPONENT_RECEIPT_FILE, readComponentReceiptV1, type ComponentDeliverableV1, type ComponentReceiptV1 } from "../../../🖨️describe/🏷️component-receipt/🟦️.ts";
import { PLUGIN_DESCRIPTOR_PROBE_SOURCE, finalizePluginDescriptor } from "../../🛂️descriptor/🟦️.ts";
import { artifactFiles, fileDigest } from "../../📦️distribution/📋️inventory/🟦️.ts";
import { ensureGuestSlimTypstFontsAt, ensurePreview2ShimVendorAt, hostShimSource, optimizePluginCoreModuleAsync, PLUGIN_HOST_SHIM_FILE, pluginCoreModuleFilesV1, pluginComponentBridgeSource, pluginWasmOptToolV1, PREVIEW2_VENDOR_RELATIVE, rewritePreview2ShimImports, SHARD_WORKER_FILE, shardWorkerSource, transpilePluginComponentModulesAsync } from "../🟦️.ts";
import { captureOwnedCommandV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { STAGE_CACHE_POLICY_V1, stageCacheV1, type StageCacheV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🗃️stage-cache/🟦️.ts";
import { repoCacheDirectory } from "../../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/🟦️.ts";

/** ⏱️ Upper bound of one guest `describe()` probe over a transpiled module. */
const DESCRIPTOR_PROBE_BUDGET_MS = 120_000;

/** 📏️ Upper bound of one probe's base64 descriptor answer. */
const DESCRIPTOR_PROBE_MAX_OUTPUT_BYTES = 12 * 1024 * 1024;

/** 📦️ `support`: stages the vendor shims, the shard worker and the packed fonts every module of the one plugin-modules root shares. */
export class SupportScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length !== 0) throw new Error("Usage: support");
    const root = pluginModulesRoot();
    mkdirSync(root, { recursive: true });
    const temporary = mkdtempSync(join(root, ".support-"));
    try {
      const vendor = join(temporary, "vendor"), shard = join(temporary, "shard");
      ensurePreview2ShimVendorAt(vendor, getWorkspaceRoot());
      mkdirSync(shard);
      writeFileSync(join(shard, SHARD_WORKER_FILE), shardWorkerSource());
      for (const destination of [builtPluginModulesRootIn(this.repoRoot, readBuildMode(process.env.SEMIO_BUILD_MODE)), root]) {
        await stageRepositoryArtifacts(join(destination, PREVIEW2_VENDOR_RELATIVE), "browser-support:preview2", artifactFiles(vendor));
        await stageRepositoryArtifacts(join(destination, MODULE_SHARD_DIRECTORY), "browser-support:shard", artifactFiles(shard));
      }
      if (!ensureGuestSlimTypstFontsAt(root, getWorkspaceRoot())) throw new Error("Missing typst font seed; run semio-framework-os-infinite:fonts through Nx");
      console.log("Browser support: vendor shims, typst fonts and shard worker staged");
    } finally { rmSync(temporary, { recursive: true, force: true }); }
  }
}


/** 🔎️ Runs the transpiled module's own `describe()` under JSPI and returns the semantic descriptor bytes it answers. */
async function probeModuleDescriptor(module: string, repoRoot: string, control: CompilerAdmissionControl, environment: Readonly<Record<string, string | undefined>> = process.env): Promise<Uint8Array> {
  const phase = requestedCompilerAdmissionControlV1(control, DESCRIPTOR_PROBE_BUDGET_MS);
  const budget = compilerAdmissionBudgetV1(phase);
  const child = spawn("node", [`--v8-pool-size=${compilerCapacityWorkerCountV1(environment)}`, "--experimental-wasm-jspi", "--input-type=module", "--eval", PLUGIN_DESCRIPTOR_PROBE_SOURCE, module], { cwd: repoRoot, env: { ...environment, UV_THREADPOOL_SIZE: String(compilerCapacityWorkerCountV1(environment)) }, signal: control.signal, timeout: budget, killSignal: "SIGTERM", stdio: ["ignore", "pipe", "pipe"] });
  const terminate = (): void => terminateOwnedChildTree(child);
  control.signal.addEventListener("abort", terminate, { once: true });
  const deadline = setTimeout(terminate, budget);
  let stdout = "", stderr = "";
  child.stdout.setEncoding("utf8").on("data", (chunk: string) => { stdout += chunk; if (stdout.length > DESCRIPTOR_PROBE_MAX_OUTPUT_BYTES) child.kill(); });
  child.stderr.setEncoding("utf8").on("data", (chunk: string) => { stderr = (stderr + chunk).slice(-8192); });
  let status: number | null, failure: unknown;
  try { status = await new Promise<number | null>(accept => { child.once("error", error => { failure = error; }); child.once("close", accept); }); } finally { clearTimeout(deadline); control.signal.removeEventListener("abort", terminate); }
  checkResourceLeaseControl(phase);
  if (failure !== undefined) throw failure;
  if (status !== 0) throw new Error(`module describe() probe failed (${status}): ${stderr}`);
  const base64 = stdout.trim();
  if (!/^[A-Za-z0-9+/]+={0,2}$/u.test(base64)) throw new Error("module describe() probe answered no base64 descriptor");
  return Buffer.from(base64, "base64");
}

/** 🧺️ The private state one component module is materialized in: its source identity, the deliverable the module is built from, the stage cache and a
 * private work directory. */
export type ComponentModuleWorkV1 = Readonly<{ repoRoot: string; target: ComponentSourceOwnerV1; deliverable: ComponentDeliverableV1; cache: StageCacheV1; work: string; environment?: Readonly<Record<string, string | undefined>>; control: CompilerAdmissionControl }>;

/** 🏷️ The jco module name of a component crate. */
function componentBaseV1(target: ComponentSourceOwnerV1): string {
  return target.packageName.replaceAll("-", "_");
}

const identities = new Map<string, Promise<string>>();
/** 🪪️ One tool identity per process: a file's SHA-256 or a command's version answer. */
function toolIdentityV1(key: string, compute: () => Promise<string>): Promise<string> {
  let known = identities.get(key);
  if (!known) { known = compute(); identities.set(key, known); }
  return known;
}

/** 🧬️ The transpile stage: jco's location-independent module files of the component, cached by the component's digest, the jco version, this module's
 * implementation and the module name. */
export async function transpileComponentModulesV1(work: ComponentModuleWorkV1): Promise<string> {
  checkResourceLeaseControl(work.control);
  const base = componentBaseV1(work.target), output = join(work.work, "transpiled");
  const [component, jco, materialization] = await Promise.all([fileDigest(work.deliverable.wasm, work.control.signal), toolIdentityV1("jco", async () => JSON.parse(readFileSync(createRequire(join(work.repoRoot, "package.json")).resolve("@bytecodealliance/jco/package.json"), "utf8")).version), toolIdentityV1("materialization", () => fileDigest(fileURLToPath(new URL("../🟦️.ts", import.meta.url))))]);
  await work.cache.run({ stage: "transpile", schemaVersion: 1, inputs: { component }, tools: { jco, materialization, base } }, output, (directory) => transpilePluginComponentModulesAsync(work.deliverable.wasm, directory, base, { repoRoot: work.repoRoot, preview2VendorDir: join(builtPluginModulesRootIn(work.repoRoot, work.deliverable.profile === "wasm-release" ? "release" : "prototype"), PREVIEW2_VENDOR_RELATIVE), environment: work.environment, control: work.control }), work.control.signal);
  checkResourceLeaseControl(work.control);
  return output;
}

/** 🪶️ The optimize stage: `wasm-opt -Oz` of every extracted core of a shipped component on `cores` threads, each cached by the core's digest, the optimizer's
 * version and its arguments (the thread count never changes Binaryen's output). Resolves with the directory of the optimized cores, or `null` for a local
 * profile that ships unoptimized cores. */
export async function optimizeComponentCoresV1(work: ComponentModuleWorkV1, transpiled: string, cores?: number): Promise<string | null> {
  checkResourceLeaseControl(work.control);
  if (!COMPONENT_PROFILE_POLICIES[work.deliverable.profile].shipped) return null;
  const output = join(work.work, "optimized"), tool = pluginWasmOptToolV1({ repoRoot: work.repoRoot }), version = await toolIdentityV1(`wasm-opt:${tool.command}`, () => captureOwnedCommandV1(tool.command, ["--version"], work.repoRoot, { control: work.control }).then((text) => text.trim()));
  mkdirSync(output, { recursive: true });
  for (const file of pluginCoreModuleFilesV1(transpiled, componentBaseV1(work.target))) {
    const core = join(transpiled, file), stage = join(work.work, `optimize-${file}`);
    await work.cache.run({ stage: "optimize", schemaVersion: 1, inputs: { core: await fileDigest(core, work.control.signal) }, tools: { "wasm-opt": version, args: tool.args.join(" ") } }, stage, (directory) => optimizePluginCoreModuleAsync(core, join(directory, "core.wasm"), { repoRoot: work.repoRoot, preview2VendorDir: "", environment: work.environment, ...(cores === undefined ? {} : { cores }), control: work.control }), work.control.signal);
    renameSync(join(stage, "core.wasm"), join(output, file));
    rmSync(stage, { recursive: true, force: true });
  }
  checkResourceLeaseControl(work.control);
  return output;
}

/** 🧩️ Assembles the module next to its final directory (so relative imports resolve exactly as published): the transpiled files with the optimized cores
 * swapped in, the host shim, and the preview2 shim imports pointed at the plugin-modules vendor directory. */
export function assembleComponentModuleV1(work: ComponentModuleWorkV1, transpiled: string, optimized: string | null): string {
  checkResourceLeaseControl(work.control);
  if (work.target.directoryName === undefined) throw new Error(`${work.target.pluginId}: materialization requires an authored deployment directory`);
  const root = builtPluginModulesRootIn(work.repoRoot, work.deliverable.profile === "wasm-release" ? "release" : "prototype"), output = join(root, moduleDirectoryName(work.target.pluginId, [{ pluginId: work.target.pluginId, directoryName: work.target.directoryName }]));
  mkdirSync(root, { recursive: true });
  const temporary = mkdtempSync(`${output}.materialize-`);
  for (const [name, path] of artifactFiles(transpiled)) { mkdirSync(dirname(join(temporary, name)), { recursive: true }); copyFileSync(optimized && existsSync(join(optimized, name)) ? join(optimized, name) : path, join(temporary, name)); }
  writeFileSync(join(temporary, PLUGIN_HOST_SHIM_FILE), hostShimSource());
  rewritePreview2ShimImports(join(temporary, `${componentBaseV1(work.target)}.js`), join(root, PREVIEW2_VENDOR_RELATIVE));
  checkResourceLeaseControl(work.control);
  return temporary;
}

/** 🔎️ The probe stage: the assembled module's own `describe()` under JSPI, cached by the digest of every module file and the node version. */
export async function probeComponentModuleV1(work: ComponentModuleWorkV1, module: string): Promise<Uint8Array> {
  checkResourceLeaseControl(work.control);
  const vendor = join(builtPluginModulesRootIn(work.repoRoot, work.deliverable.profile === "wasm-release" ? "release" : "prototype"), PREVIEW2_VENDOR_RELATIVE);
  if (!existsSync(join(vendor, ".nx-artifact.json"))) throw new Error("Missing browser support prerequisite; run @semio-tech/framework-plugin-web:support");
  const inputs: Record<string, string> = {};
  for (const [name, path] of artifactFiles(module)) inputs[name] = await fileDigest(path, work.control.signal);
  for (const [name, path] of artifactFiles(vendor)) inputs[`vendor/${name}`] = await fileDigest(path, work.control.signal);
  const node = await toolIdentityV1("node", () => captureOwnedCommandV1("node", ["--version"], work.repoRoot, { control: work.control }).then((text) => text.trim())), output = join(work.work, "probe");
  await work.cache.run({ stage: "probe", schemaVersion: 1, inputs, tools: { node } }, output, async (directory) => { writeFileSync(join(directory, "semantic.bin"), await probeModuleDescriptor(join(module, `${componentBaseV1(work.target)}.js`), work.repoRoot, work.control, work.environment)); }, work.control.signal);
  checkResourceLeaseControl(work.control);
  return readFileSync(join(output, "semantic.bin"));
}

/** 📦️ Publishes the assembled module: its semantic descriptor, the bridge and a copy of the receipt that binds the two, staged atomically under the
 * component's one ownership key. The module's `describe()` must hash to the receipt's semantic descriptor. */
export async function publishComponentModuleV1(work: ComponentModuleWorkV1, module: string, semantic: Uint8Array, receipt: ComponentReceiptV1): Promise<string> {
  checkResourceLeaseControl(work.control);
  const output = join(pluginModulesRootIn(work.repoRoot), work.target.directoryName!), base = componentBaseV1(work.target);
  try {
    if (createHash("sha256").update(semantic).digest("hex") !== receipt.semanticDescriptorSha256) throw new Error(`${work.target.pluginId}: the module's describe() is not the semantic descriptor its receipt names`);
    const descriptor = finalizePluginDescriptor(semantic, work.target.pluginId);
    writeFileSync(join(module, "🛂️.descriptor.semio"), descriptor.pack);
    writeFileSync(join(module, "🔣️.json"), descriptor.json);
    writeFileSync(join(module, MODULE_BRIDGE_FILE), pluginComponentBridgeSource(base, work.target.wasmOut));
    writeFileSync(join(module, COMPONENT_RECEIPT_FILE), readFileSync(work.deliverable.receipt));
    work.control.signal?.throwIfAborted();
    const mode = receipt.profile === "wasm-release" ? "release" : "prototype", canonical = join(builtPluginModulesRootIn(work.repoRoot, mode), work.target.directoryName!);
    const files = artifactFiles(module), options = { control: work.control, signal: work.control.signal };
    await stageRepositoryArtifacts(canonical, componentModuleOwnerV1(work.target.cratePath), files, options);
    checkResourceLeaseControl(work.control);
    await stageRepositoryArtifacts(output, componentModuleOwnerV1(work.target.cratePath), files, options);
  } finally {
    rmSync(module, { recursive: true, force: true });
  }
  checkResourceLeaseControl(work.control);
  return output;
}

/** 🧩️ Materializes one component deliverable into its module directory of the one plugin-modules root through the cached stages: transpile, `wasm-opt -Oz`
 * only for shipped bytes, the module's own `describe()` checked against the receipt's semantic descriptor, then that semantic descriptor,
 * the bridge, the host shim and a copy of the receipt — so every consumer can tell a shipped module from a local `wasm-prototype` one. */
export async function materializeComponentModuleV1(repoRoot: string, target: ComponentSourceOwnerV1, deliverable: ComponentDeliverableV1, control: CompilerAdmissionControl): Promise<Readonly<{ output: string; receipt: ComponentReceiptV1 }>> {
  if (target.directoryName === undefined) throw new Error(`${target.pluginId}: materialization requires an authored deployment directory`);
  const receipt = readComponentReceiptV1(deliverable.receipt);
  if (receipt.pluginId !== target.pluginId || receipt.packageId !== target.packageId || receipt.profile !== deliverable.profile) throw new Error(`${target.pluginId}: ${relative(repoRoot, deliverable.receipt)} names ${receipt.pluginId} ${receipt.profile}, not this ${deliverable.profile} deliverable`);
  if (lstatSync(deliverable.wasm).size !== receipt.wasmBytes || await fileDigest(deliverable.wasm, control.signal) !== receipt.wasmSha256) throw new Error(`${target.pluginId}: ${relative(repoRoot, deliverable.wasm)} is not the component its receipt names; rebuild it`);
  const work: ComponentModuleWorkV1 = { repoRoot, target, deliverable, cache: stageCacheV1(repoCacheDirectory(repoRoot, STAGE_CACHE_POLICY_V1.warmRoot)), work: mkdtempSync(join(dirname(deliverable.directory), ".materialize-")), control };
  try {
    const transpiled = await transpileComponentModulesV1(work), module = assembleComponentModuleV1(work, transpiled, await optimizeComponentCoresV1(work, transpiled));
    const output = await publishComponentModuleV1(work, module, await probeComponentModuleV1(work, module).catch((error: unknown) => { rmSync(module, { recursive: true, force: true }); throw error; }), receipt);
    return Object.freeze({ output, receipt });
  } finally {
    rmSync(work.work, { recursive: true, force: true });
  }
}
