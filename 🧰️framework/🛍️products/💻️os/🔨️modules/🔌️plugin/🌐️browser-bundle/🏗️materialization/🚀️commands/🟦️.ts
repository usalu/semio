import { stripWasmDebugSectionsV1 } from "../🪶️strip-debug/🟦️.ts";
import { withRepositoryArtifactV1, withRepositoryArtifactTaskV1, readNativeCallerAuthorityV1, withArtifactScratchV1, withArtifactDestinationTaskV1, withRepositoryArtifactDestinationV1, stageRepositoryArtifacts } from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️artifacts/🟦️.ts";
import {withArtifactDescriptorSlotV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/📤️publication/📥️sources/🟦️.ts";
import { artifactsStorageSchema as publicationSchema } from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️artifacts/🟦️.ts";
import {copyFile} from "node:fs/promises";
import type {ScriptInvocation} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { withRepositoryCacheStoreV1 } from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/💾️storage/🟦️.ts";
import { checkResourceLeaseControl } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import { terminateOwnedChildTree } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { scriptCompilerAdmissionControlV1, compilerCapacityWorkerCountV1, compilerAdmissionBudgetV1, requestedCompilerAdmissionControlV1, type CompilerAdmissionControl } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { readBuildMode } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";

import { fileURLToPath } from "node:url";
import { createHash } from "node:crypto";
import { copyFileSync, existsSync, lstatSync, mkdirSync, mkdtempSync, readdirSync, readFileSync, renameSync, rmSync, writeFileSync } from "node:fs";
import { basename, dirname, join, posix, relative, sep } from "node:path";
import { BundleScript } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { componentModuleOwnerV1, MODULE_BRIDGE_FILE, MODULE_SHARD_DIRECTORY, moduleDirectoryName } from "../../../📇️registry/📦️deployment/🟦️.ts";
import type { ComponentSourceOwnerV1 } from "@semio-tech/plugin-registry/discovery";
import { builtPluginModulesRootIn } from "../../../../🧑‍💻dev/📦️packages/🟦️typescript/♻️activation/🟦️.ts";
import { COMPONENT_PROFILE_POLICIES, COMPONENT_RECEIPT_FILE,encodeComponentReceiptV1, readComponentReceiptV1, type ComponentDeliverableV1, type ComponentReceiptV1 } from "../../../🖨️describe/🏷️component-receipt/🟦️.ts";
import { CATALOG_DESCRIPTOR_MAX_BYTES } from "../../../📇️registry/✅️catalog-verification/🟦️.ts";
import { PLUGIN_DESCRIPTOR_PROBE_SOURCE, finalizePluginDescriptor } from "../../🛂️descriptor/🟦️.ts";
import { artifactFiles, fileDigest,readArtifactBytesV1 } from "../../📦️distribution/📋️inventory/🟦️.ts";
import { ensureGuestSlimTypstFontsAt } from "../🔤️guestslim-fonts/🟦️.ts";
import { ensurePreview2ShimVendorAt } from "../🩹️preview2-shim-patches/🟦️.ts";
import { hostShimSource } from "../🪢️host-shim-source/🟦️.ts";
import { optimizePluginCoreModuleAsync, pluginCoreModuleFilesV1, pluginWasmOptToolV1 } from "../🪶️wasm-opt/🟦️.ts";
import { PLUGIN_HOST_SHIM_FILE, SHARD_WORKER_FILE } from "../🧾️contract/🟦️.ts";
import { pluginComponentBridgeSource } from "../🌉️component-bridge-source/🟦️.ts";
import { PREVIEW2_VENDOR_RELATIVE } from "../../🕸️imports/🟦️.ts";
import { rewritePreview2ShimImports, transpilePluginComponentModulesAsync } from "../🟦️.ts";
import { nativeTranspileArgsV1, nativeTranspileBinaryV1, PLUGIN_COMPONENT_IMPORT_MAPS_V1 } from "../🦀️native-transpile/🟦️.ts";
import { rewriteJcoAsyncResultLiftingAt, rewriteJcoComponentAssetUrlsAt } from "../🧬️jco-module-rewrites/🟦️.ts";
import { shardWorkerSource } from "../🧵️shard-worker-source/🟦️.ts";
import { captureOwnedCommandV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { STAGE_CACHE_POLICY_V1, stageCacheV1, type StageCacheV1, type StageRemoteV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🗃️stage-cache/🟦️.ts";
import { startToolV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧰️tools/🟦️.ts";

/** ⏱️ Upper bound of one guest `describe()` probe over a transpiled module. */
const DESCRIPTOR_PROBE_BUDGET_MS = 120_000;

/** 📏️ Upper bound of one probe's base64 descriptor answer. */
const DESCRIPTOR_PROBE_MAX_OUTPUT_BYTES = 12 * 1024 * 1024;

/** 📦️ `support`: stages the vendor shims, the shard worker and the packed fonts every module of the one plugin-modules root shares. */
export class SupportScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length !== 0) throw new Error("Usage: support");
    const mode=readNativeCallerAuthorityV1(this.invocation.capabilities).original.buildMode,phase=scriptCompilerAdmissionControlV1(this.invocation,"browser:support",0);
    await withRepositoryCacheStoreV1(this.repoRoot,"dependencies",this.invocation,phase,(_store,storePhase)=>withRepositoryCacheStoreV1(this.repoRoot,"products",this.invocation,storePhase,(store,storePhase)=>withArtifactScratchV1({workspace:this.repoRoot,invocation:this.invocation,storage:store.storage,control:store.control,phaseControl:storePhase,request:{version:1,mode,owner:"browser/support"}},token=>withArtifactDestinationTaskV1(token,this.invocation,async sink=>{
      const vendor=join(sink.directory,"vendor"),shard=join(sink.directory,"shard"),root=builtPluginModulesRootIn(this.repoRoot,mode);
      ensurePreview2ShimVendorAt(vendor,this.repoRoot);mkdirSync(shard);writeFileSync(join(shard,SHARD_WORKER_FILE),shardWorkerSource());
      for(const[directory,owner,source]of[[PREVIEW2_VENDOR_RELATIVE,"browser-support:preview2",vendor],[MODULE_SHARD_DIRECTORY,"browser-support:shard",shard]]as const)await withRepositoryArtifactDestinationV1({workspace:this.repoRoot,invocation:this.invocation,storage:store.storage,control:store.control,phaseControl:sink.control,directory:join(root,directory)},destination=>stageRepositoryArtifacts(destination,owner,artifactFiles(source)));
      console.log("Browser support: canonical selected-mode vendor shims and shard worker published");
    }))));
  }
}


/** 🔎️ Runs the transpiled module's own `describe()` under JSPI and returns the semantic descriptor bytes it answers. */
async function probeModuleDescriptor(module: string, repoRoot: string, control: CompilerAdmissionControl, environment: Readonly<Record<string, string | undefined>> = process.env): Promise<Uint8Array> {
  const phase = requestedCompilerAdmissionControlV1(control, DESCRIPTOR_PROBE_BUDGET_MS);
  const budget = compilerAdmissionBudgetV1(phase);
  const child = startToolV1("node", [`--v8-pool-size=${compilerCapacityWorkerCountV1(environment)}`, "--experimental-wasm-jspi", "--input-type=module", "--eval", PLUGIN_DESCRIPTOR_PROBE_SOURCE, module], { cwd: repoRoot, env: { ...environment, UV_THREADPOOL_SIZE: String(compilerCapacityWorkerCountV1(environment)) }, signal: control.signal, timeout: budget, killSignal: "SIGTERM", stdio: ["ignore", "pipe", "pipe"] });
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
export type ComponentModuleWorkV1 = Readonly<{ invocation:ScriptInvocation; repoRoot: string; target: ComponentSourceOwnerV1; deliverable: ComponentDeliverableV1; cache: StageCacheV1; work: string; environment?: Readonly<Record<string, string | undefined>>; control: CompilerAdmissionControl }>;

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

/** 🧬️ The identity of the sources the transpile stage runs: the transpile orchestration, the jco module rewrites and the contract they share. */
async function materializationIdentityV1(): Promise<string> {
  const digests = await Promise.all(["../🟦️.ts", "../🧬️jco-module-rewrites/🟦️.ts", "../🧾️contract/🟦️.ts", "../🦀️native-transpile/🟦️.ts"].map((source) => fileDigest(fileURLToPath(new URL(source, import.meta.url)))));
  return createHash("sha256").update(digests.join("\n")).digest("hex");
}

/** 🧬️ Keys native transpilation by the stripped component bytes, the content digest of the native executable (which embeds jco and its mode), the materializer
 * sources, the import maps and the module name; no checkout path, vendor tree or build profile enters the key because the closure reads none of them. */
export function transpileStageRequestV1(component: string, materialization: string, base: string, native: string, maps: readonly string[] = PLUGIN_COMPONENT_IMPORT_MAPS_V1) {
  return { stage: "transpile", schemaVersion: 3, inputs: { component }, tools: { materialization, base, "native-transpile": native, maps: createHash("sha256").update(maps.join("\n")).digest("hex") } };
}

/** 🪶️ The optimize stage cache request of one extracted core: its digest, the optimizer's version and content digest, and its arguments (never the thread count). */
export function optimizeStageRequestV1(core: string, version: string, args: readonly string[], binary: string) {
  return { stage: "optimize", schemaVersion: 2, inputs: { core }, tools: { "wasm-opt": version, binary, args: args.join(" ") } };
}

/** 🧬️ Transpiles the debug-stripped component through the staged native executable and caches the location-independent module files. */
export async function transpileComponentModulesV1(work: ComponentModuleWorkV1): Promise<string> {
  checkResourceLeaseControl(work.control);
  const base = componentBaseV1(work.target), output = join(work.work, "transpiled");
  const nativeBinary = nativeTranspileBinaryV1(work.repoRoot, work.deliverable.profile, work.environment);
  const component = createHash("sha256").update(stripWasmDebugSectionsV1(readFileSync(work.deliverable.wasm))).digest("hex");
  const [materialization, native] = await Promise.all([
    toolIdentityV1("materialization", materializationIdentityV1),
    fileDigest(nativeBinary, work.control.signal),
  ]);
  await work.cache.run(transpileStageRequestV1(component, materialization, base, native), output, (directory) => transpilePluginComponentModulesAsync(work.deliverable.wasm, directory, base, { repoRoot: work.repoRoot, mode: work.deliverable.profile, preview2VendorDir: join(builtPluginModulesRootIn(work.repoRoot, work.deliverable.profile), PREVIEW2_VENDOR_RELATIVE), environment: work.environment, control: work.control }), work.control.signal, base, transpileStageRemoteV1(work, nativeBinary, native));
  checkResourceLeaseControl(work.control);
  return output;
}

const SHA256_V1 = /^[0-9a-f]{64}$/u;

/** 🔌️ The command offload of this checkout, `undefined` when offloading is switched off (`SEMIO_OFFLOAD=never`) so the stage runs here.
 * @see ../../../../../../🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🚀️offload/🟦️.ts */
async function stageOffloadV1(work: ComponentModuleWorkV1) {
  const environment = work.environment ?? process.env, offload = await import("../../../../../../🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🚀️offload/🟦️.ts");
  return offload.offloadModeV1(environment) === "never" ? undefined : { offload, environment, root: environment.SEMIO_UNIT_CACHE_ROOT ?? join(work.cache.root, ".offload") };
}

/** 🛟️ Runs one remote producer so that it never fails a stage: a fault, a refusal or a local answer empties the stage directory and answers `false` (the stage runs here). */
async function remoteStageV1(directory: string, body: () => Promise<boolean>): Promise<boolean> {
  const emptied = (): void => { for (const entry of readdirSync(directory)) rmSync(join(directory, entry), { recursive: true, force: true }); };
  try { if (await body()) return true; } catch {}
  try { emptied(); } catch {}
  return false;
}

/** ☁️ The remote producer of one optimize stage: on a fleet miss, `wasm-opt` over the extracted core runs on an idle machine of the fleet through the command offload (the tool pinned by the content digest of its binary, the core as the only input, `core.wasm` as the only output; the arguments are the shipped ones, never the thread count) and the optimized bytes land in the stage directory exactly as a local run writes them. It answers `false` (the stage runs here) when offloading is switched off (`SEMIO_OFFLOAD=never`), the binary digest is no SHA-256, or the fleet cannot take it.
 * @see ../../../../../../🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🚀️offload/🟦️.ts */
export function optimizeStageRemoteV1(work: ComponentModuleWorkV1, core: string, binary: string, args: readonly string[]): StageRemoteV1 {
  return async (directory, signal) => remoteStageV1(directory, async () => {
    const port = await stageOffloadV1(work);
    if (port === undefined || !SHA256_V1.test(binary)) return false;
    const outcome = await port.offload.offloadCommandV1({ tool: { name: "wasm-opt", digest: binary }, arguments: ["@root/in/core.wasm", ...args, "-o", "@root/out/core.wasm"], inputs: new Map([["in/core.wasm", core]]), outputs: new Map([["out/core.wasm", join(directory, "core.wasm")]]), root: port.root, environment: port.environment, label: `wasm-opt ${componentBaseV1(work.target)}/${basename(core)}`, key: null, signal });
    return outcome.kind === "done";
  });
}

/** ☁️ The remote producer of one transpile stage: the native executable is shipped as an input pinned by the digest in the key (the worker checks it after placing the inputs), the debug-stripped component is the only other input and every file the executable writes below `out/transpiled` lands in the stage directory; then the same two module rewrites a local run applies are applied here, so the stage holds exactly the files a local run leaves. It answers `false` (the stage runs here) when offloading is switched off, the digest is no SHA-256 or the fleet cannot take it.
 * @see ../../../../../../🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🚀️offload/🟦️.ts */
export function transpileStageRemoteV1(work: ComponentModuleWorkV1, binary: string, native: string): StageRemoteV1 {
  return async (directory, signal) => remoteStageV1(directory, async () => {
    const port = await stageOffloadV1(work), base = componentBaseV1(work.target);
    if (port === undefined || !SHA256_V1.test(native)) return false;
    mkdirSync(work.work, { recursive: true });
    const scratch = mkdtempSync(join(work.work, ".transpile-remote-")), component = join(scratch, `${base}.nodebug.wasm`);
    try {
      writeFileSync(component, stripWasmDebugSectionsV1(readFileSync(work.deliverable.wasm)));
      const outcome = await port.offload.offloadCommandV1({ tool: { name: "native-transpile", digest: native, file: binary }, arguments: nativeTranspileArgsV1(`@root/in/${base}.nodebug.wasm`, "@root/out/transpiled", base, PLUGIN_COMPONENT_IMPORT_MAPS_V1), inputs: new Map([[`in/${base}.nodebug.wasm`, component]]), outputs: new Map(), directories: new Map([["out/transpiled", directory]]), root: port.root, environment: port.environment, label: `transpile ${base}`, key: null, signal });
      if (outcome.kind !== "done") return false;
    } finally { rmSync(scratch, { recursive: true, force: true }); }
    rewriteJcoAsyncResultLiftingAt(join(directory, `${base}.js`));
    rewriteJcoComponentAssetUrlsAt(join(directory, `${base}.js`));
    return true;
  });
}

/** 🪶️ The optimize stage: `wasm-opt -Oz` of every extracted core of a shipped component on `cores` threads, each cached by the core's digest, the optimizer's
 * version and its arguments (the thread count never changes Binaryen's output). Resolves with the directory of the optimized cores, or `null` for a local
 * profile that ships unoptimized cores. */
export async function optimizeComponentCoresV1(work: ComponentModuleWorkV1, transpiled: string, cores?: number): Promise<string | null> {
  checkResourceLeaseControl(work.control);
  if (!COMPONENT_PROFILE_POLICIES[work.deliverable.profile].shipped) return null;
  const output = join(work.work, "optimized"), tool = pluginWasmOptToolV1({ repoRoot: work.repoRoot }), version = await toolIdentityV1(`wasm-opt:${tool.command}`, () => captureOwnedCommandV1(tool.command, ["--version"], work.repoRoot, { control: work.control }).then((text) => text.trim())), binary = await toolIdentityV1(`wasm-opt-binary:${tool.command}`, () => fileDigest(tool.command, work.control.signal));
  mkdirSync(output, { recursive: true });
  for (const file of pluginCoreModuleFilesV1(transpiled, componentBaseV1(work.target))) {
    const core = join(transpiled, file), stage = join(work.work, `optimize-${file}`);
    await work.cache.run(optimizeStageRequestV1(await fileDigest(core, work.control.signal), version, tool.args, binary), stage, (directory) => optimizePluginCoreModuleAsync(core, join(directory, "core.wasm"), { repoRoot: work.repoRoot, preview2VendorDir: "", environment: work.environment, ...(cores === undefined ? {} : { cores }), control: work.control }), work.control.signal, `${componentBaseV1(work.target)}/${file}`, optimizeStageRemoteV1(work, core, binary, tool.args));
    renameSync(join(stage, "core.wasm"), join(output, file));
    rmSync(stage, { recursive: true, force: true });
  }
  checkResourceLeaseControl(work.control);
  return output;
}

/** 🧩️ Assembles the module next to its final directory (so relative imports resolve exactly as published): the transpiled files with the optimized cores
 * swapped in, the host shim, and the preview2 shim imports pointed at the plugin-modules vendor directory. */
async function assembleComponentModuleV1(work:ComponentModuleWorkV1,transpiled:string,optimized:string|null,temporary:string,vendor:string,maximumFiles:number,maximumBytes:number):Promise<string>{
 checkResourceLeaseControl(work.control);if(work.target.directoryName===undefined)throw Error("Authored deployment directory required");mkdirSync(temporary,{recursive:true});const shim=hostShimSource();let bytes=Buffer.byteLength(shim);if(bytes>maximumBytes)throw Error("Finite browser shim extent required");for(const[name,path]of artifactFiles(transpiled,maximumFiles-1,maximumBytes-bytes)){const source=optimized&&existsSync(join(optimized,name))?join(optimized,name):path,destination=join(temporary,name),facts=lstatSync(source),extent=facts.size;if(!facts.isFile()||facts.isSymbolicLink()||facts.nlink!==1||destination.length>publicationSchema.definitions.Protocol.const.maximumPathUnits||extent>maximumBytes-bytes)throw Error("Finite assembled browser extent required");bytes+=extent;mkdirSync(dirname(destination),{recursive:true});await withArtifactDescriptorSlotV1(()=>withArtifactDescriptorSlotV1(()=>copyFile(source,destination)));}writeFileSync(join(temporary,PLUGIN_HOST_SHIM_FILE),shim);rewritePreview2ShimImports(join(temporary,`${componentBaseV1(work.target)}.js`),vendor);checkResourceLeaseControl(work.control);return temporary;
}

/** 🔎️ The probe stage: the assembled module's own `describe()` under JSPI, cached by the digest of every module file and the node version. */
/** 🧪️ The probe stage cache request: the probe source is part of the key, so a changed probe can never reuse the semantic bytes of an older one. */
export function probeStageRequestV1(inputs: Record<string, string>, node: string, source: string = PLUGIN_DESCRIPTOR_PROBE_SOURCE) {
  return { stage: "probe", schemaVersion: 2, inputs, tools: { node, probe: createHash("sha256").update(source).digest("hex") } };
}

/** 🪄️ The two files a remote probe runs from `in/`: the unchanged probe source and a fixed entry that points `process.argv[1]` at the module and writes what the probe prints to the declared output file (a command declares output files, never a stream). */
const PROBE_ENTRY_SOURCE_V1 = `import { writeFileSync } from "node:fs";
const [, , out, module] = process.argv;
process.argv.splice(1, 2, module);
process.stdout.write = (chunk) => { writeFileSync(out, chunk); return true; };
await import("./probe.mjs");
`;

/** ☁️ The remote producer of one probe stage: the module's `describe()` runs under the `node` of the fleet that has exactly this machine's `node` file (the worker offers it by digest; nothing is shipped), over the module files and the preview2 vendor files placed at the relative layout the module's imports expect; the base64 answer is decoded into `semantic.bin` as a local run does. It answers `false` (the stage runs here) when offloading is switched off, the layout does not fit below the input root or the fleet cannot take it.
 * @see ../../../../../../🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🚀️offload/🟦️.ts */
export function probeStageRemoteV1(work: ComponentModuleWorkV1, module: string, vendor: string, source: string = PLUGIN_DESCRIPTOR_PROBE_SOURCE): StageRemoteV1 {
  return async (directory, signal) => remoteStageV1(directory, async () => {
    const port = await stageOffloadV1(work), base = componentBaseV1(work.target);
    if (port === undefined) return false;
    const { commandToolDigestV1, defaultCommandToolFindersV1 } = await import("../../../../../../🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🚀️offload/🧰️command/🛠️worker-tools/🟦️.ts"), path = await defaultCommandToolFindersV1(work.repoRoot, port.environment).node?.();
    if (path === undefined) return false;
    const node = await toolIdentityV1(`node-binary:${path}`, async () => commandToolDigestV1(path));
    const vendorAt = posix.normalize(posix.join("in/m", relative(module, vendor).split(sep).join("/")));
    if (!SHA256_V1.test(node) || !vendorAt.startsWith("in/")) return false;
    const inputs = new Map<string, string>();
    for (const [name, file] of artifactFiles(module)) inputs.set(`in/m/${name}`, file);
    for (const [name, file] of artifactFiles(vendor)) if (inputs.has(`${vendorAt}/${name}`)) return false; else inputs.set(`${vendorAt}/${name}`, file);
    mkdirSync(work.work, { recursive: true });
    const scratch = mkdtempSync(join(work.work, ".probe-remote-")), answer = join(scratch, "semantic.b64");
    try {
      writeFileSync(join(scratch, "probe.mjs"), source);
      writeFileSync(join(scratch, "run.mjs"), PROBE_ENTRY_SOURCE_V1);
      inputs.set("in/probe.mjs", join(scratch, "probe.mjs")).set("in/run.mjs", join(scratch, "run.mjs"));
      const outcome = await port.offload.offloadCommandV1({ tool: { name: "node", digest: node }, arguments: ["--experimental-wasm-jspi", "@root/in/run.mjs", "@root/out/semantic.b64", `@root/in/m/${base}.js`], inputs, outputs: new Map([["out/semantic.b64", answer]]), root: port.root, environment: port.environment, label: `probe ${base}`, key: null, signal });
      if (outcome.kind !== "done") return false;
      const text = readFileSync(answer, "utf8").trim();
      if (!/^[A-Za-z0-9+/]+={0,2}$/u.test(text)) return false;
      writeFileSync(join(directory, "semantic.bin"), Buffer.from(text, "base64"));
      return true;
    } finally { rmSync(scratch, { recursive: true, force: true }); }
  });
}

async function probeComponentModuleV1(work: ComponentModuleWorkV1, module: string,vendor:string): Promise<Uint8Array> {
  checkResourceLeaseControl(work.control);
  const inputs: Record<string, string> = {};
  for (const [name, path] of artifactFiles(module)) inputs[name] = await fileDigest(path, work.control.signal);
  for (const [name, path] of artifactFiles(vendor)) inputs[`vendor/${name}`] = await fileDigest(path, work.control.signal);
  const node = await toolIdentityV1("node", () => captureOwnedCommandV1("node", ["--version"], work.repoRoot, { control: work.control }).then((text) => text.trim())), output = join(work.work, "probe");
  await work.cache.run(probeStageRequestV1(inputs, node), output, async (directory) => { writeFileSync(join(directory, "semantic.bin"), await probeModuleDescriptor(join(module, `${componentBaseV1(work.target)}.js`), work.repoRoot, work.control, work.environment)); }, work.control.signal, componentBaseV1(work.target), probeStageRemoteV1(work, module, vendor));
  checkResourceLeaseControl(work.control);
  return readArtifactBytesV1(join(output,"semantic.bin"),CATALOG_DESCRIPTOR_MAX_BYTES,work.control.signal);
}

/** 📦️ Publishes the assembled module: its semantic descriptor, the bridge and a copy of the receipt that binds the two, staged atomically under the
 * component's one ownership key. The module's `describe()` must hash to the receipt's semantic descriptor. */
async function publishComponentModuleV1(work: ComponentModuleWorkV1, module: string, semantic: Uint8Array, receipt: ComponentReceiptV1,maximumFiles:number,maximumBytes:number): Promise<string> {
  checkResourceLeaseControl(work.control);
  const mode=readNativeCallerAuthorityV1(work.invocation.capabilities).original.buildMode,output=join(builtPluginModulesRootIn(work.repoRoot,mode),work.target.directoryName!),base=componentBaseV1(work.target);
  if(receipt.profile!==mode)throw Error("Component receipt must match its actual Original mode");
    if (createHash("sha256").update(semantic).digest("hex") !== receipt.semanticDescriptorSha256) throw new Error(`${work.target.pluginId}: the module's describe() is not the semantic descriptor its receipt names`);
    const descriptor=finalizePluginDescriptor(semantic,work.target.pluginId),payload=new Map<string,Uint8Array|string>([["🛂️.descriptor.semio",descriptor.pack],["🔣️.json",descriptor.json],[MODULE_BRIDGE_FILE,pluginComponentBridgeSource(base,work.target.wasmOut)],[COMPONENT_RECEIPT_FILE,encodeComponentReceiptV1(receipt)]]),existing=artifactFiles(module,maximumFiles,maximumBytes);let bytes=0,count=existing.size;
    for(const[name,path]of existing)if(!payload.has(name))bytes+=lstatSync(path).size;for(const[name,value]of payload){if(!existing.has(name))count++;bytes+=typeof value==="string"?Buffer.byteLength(value):value.byteLength;}if(count>maximumFiles||bytes>maximumBytes)throw Error("Final browser module exceeds combined scratch grant");for(const[name,value]of payload)writeFileSync(join(module,name),value);
    work.control.signal?.throwIfAborted();
    const files=artifactFiles(module);
    await withRepositoryCacheStoreV1(work.repoRoot,"products",work.invocation,work.control,(store,storePhase)=>withRepositoryArtifactDestinationV1({workspace:work.repoRoot,invocation:work.invocation,storage:store.storage,control:store.control,phaseControl:storePhase,directory:output},destination=>stageRepositoryArtifacts(destination,componentModuleOwnerV1(work.target.cratePath),files)));
    return output;
}

/** 🌐️ Retains one genuine vendor reader and funded assembled module through actual probe and final publication. */
export async function publishComponentBrowserV1(work:ComponentModuleWorkV1,transpiled:string,optimized:string|null,receipt:ComponentReceiptV1,withCompilerFiles:<T>(phase:CompilerAdmissionControl,body:(writer:CompilerAdmissionControl)=>Promise<T>)=>Promise<T>):Promise<void>{
 const mode=readNativeCallerAuthorityV1(work.invocation.capabilities).original.buildMode,policy=publicationSchema.definitions.Protocol.const;if(receipt.profile!==mode)throw Error("Exact actual browser publication mode required");
 await withRepositoryCacheStoreV1(work.repoRoot,"products",work.invocation,work.control,(store,storePhase)=>withRepositoryArtifactV1({workspace:work.repoRoot,invocation:work.invocation,storage:store.storage,control:store.control,phaseControl:storePhase,request:{version:1,path:join(builtPluginModulesRootIn(work.repoRoot,mode),PREVIEW2_VENDOR_RELATIVE),owner:"browser-support:preview2",maximumBytes:policy.maximumOutputBytes,maximumFiles:policy.maximumFiles}},reader=>withRepositoryArtifactTaskV1(reader,work.invocation,async artifact=>{
  await withArtifactScratchV1({workspace:work.repoRoot,invocation:work.invocation,storage:store.storage,control:store.control,phaseControl:artifact.control,request:{version:1,mode,owner:"browser/module/"+work.target.pluginId}},scratch=>withArtifactDestinationTaskV1(scratch,work.invocation,async sink=>{
   let total=0;const vendor=join(sink.directory,PREVIEW2_VENDOR_RELATIVE);for(const name of artifact.files){const source=join(artifact.path,name),destination=join(vendor,name),size=lstatSync(source).size;if(destination.length>policy.maximumPathUnits||size>policy.maximumOutputBytes-total)throw Error("Browser vendor scratch extent exceeded");total+=size;mkdirSync(dirname(destination),{recursive:true});await withArtifactDescriptorSlotV1(()=>withArtifactDescriptorSlotV1(()=>copyFile(source,destination)));}
   const maximumFiles=policy.maximumFiles-artifact.files.length,maximumBytes=policy.maximumOutputBytes-total;
   const {module,semantic}=await withCompilerFiles(sink.control,async writer=>{
    const compilerWork={...work,control:writer};
    const module=await assembleComponentModuleV1(compilerWork,transpiled,optimized,join(sink.directory,"module"),vendor,maximumFiles,maximumBytes);
    artifactFiles(sink.directory);
    return {module,semantic:await probeComponentModuleV1(compilerWork,module,vendor)};
   });
   await artifact.recheck();
   await publishComponentModuleV1({...work,control:sink.control},module,semantic,receipt,maximumFiles,maximumBytes);
  }));
 })));
}
