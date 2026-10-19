import { withRepositoryArtifactV1, withRepositoryArtifactTaskV1, readNativeCallerAuthorityV1, withArtifactScratchV1, withArtifactDestinationTaskV1, withRepositoryArtifactDestinationV1, stageRepositoryArtifacts } from "@semio-tech/repo-lib/artifacts";
import {withArtifactDescriptorSlotV1} from "@semio-tech/process/artifacts/publication/sources";
import { artifactsStorageSchema as publicationSchema } from "@semio-tech/repo-lib/artifacts";
import {copyFile} from "node:fs/promises";
import type {ScriptInvocation} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { withRepositoryCacheStoreV1 } from "@semio-tech/repo-lib/storage";
import { checkResourceLeaseControl } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import { terminateOwnedChildTree } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { scriptCompilerAdmissionControlV1, compilerCapacityWorkerCountV1, compilerAdmissionBudgetV1, requestedCompilerAdmissionControlV1, type CompilerAdmissionControl } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { readBuildMode } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";
import { spawn } from "node:child_process";
import { createRequire } from "node:module";
import { fileURLToPath } from "node:url";
import { createHash } from "node:crypto";
import { copyFileSync, existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, renameSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join, relative } from "node:path";
import { BundleScript } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { componentModuleOwnerV1, MODULE_BRIDGE_FILE, MODULE_SHARD_DIRECTORY, moduleDirectoryName } from "@semio-tech/plugin-registry/deployment";
import type { ComponentSourceOwnerV1 } from "@semio-tech/plugin-registry/discovery";
import { builtPluginModulesRootIn } from "@semio-tech/framework-os-dev/activation";
import { COMPONENT_PROFILE_POLICIES, COMPONENT_RECEIPT_FILE,encodeComponentReceiptV1, readComponentReceiptV1, type ComponentDeliverableV1, type ComponentReceiptV1 } from "../../../🖨️describe/🏷️component-receipt/🟦️.ts";
import { CATALOG_DESCRIPTOR_MAX_BYTES } from "@semio-tech/plugin-registry/catalog-verification";
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
import { shardWorkerSource } from "../🧵️shard-worker-source/🟦️.ts";
import { captureOwnedCommandV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { STAGE_CACHE_POLICY_V1, stageCacheV1, type StageCacheV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🗃️stage-cache/🟦️.ts";

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
  const digests = await Promise.all(["../🟦️.ts", "../🧬️jco-module-rewrites/🟦️.ts", "../🧾️contract/🟦️.ts"].map((source) => fileDigest(fileURLToPath(new URL(source, import.meta.url)))));
  return createHash("sha256").update(digests.join("\n")).digest("hex");
}

/** 🧬️ The transpile stage: jco's location-independent module files of the component, cached by the component's digest, the jco version, this module's
 * implementation and the module name. */
export async function transpileComponentModulesV1(work: ComponentModuleWorkV1): Promise<string> {
  checkResourceLeaseControl(work.control);
  const base = componentBaseV1(work.target), output = join(work.work, "transpiled");
  const [component, jco, materialization] = await Promise.all([fileDigest(work.deliverable.wasm, work.control.signal), toolIdentityV1("jco", async () => JSON.parse(readFileSync(createRequire(join(work.repoRoot, "package.json")).resolve("@bytecodealliance/jco/package.json"), "utf8")).version), toolIdentityV1("materialization", materializationIdentityV1)]);
  await work.cache.run({ stage: "transpile", schemaVersion: 1, inputs: { component }, tools: { jco, materialization, base } }, output, (directory) => transpilePluginComponentModulesAsync(work.deliverable.wasm, directory, base, { repoRoot: work.repoRoot, preview2VendorDir: join(builtPluginModulesRootIn(work.repoRoot, work.deliverable.profile), PREVIEW2_VENDOR_RELATIVE), environment: work.environment, control: work.control }), work.control.signal);
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
async function assembleComponentModuleV1(work:ComponentModuleWorkV1,transpiled:string,optimized:string|null,temporary:string,vendor:string,maximumFiles:number,maximumBytes:number):Promise<string>{
 checkResourceLeaseControl(work.control);if(work.target.directoryName===undefined)throw Error("Authored deployment directory required");mkdirSync(temporary,{recursive:true});const shim=hostShimSource();let bytes=Buffer.byteLength(shim);if(bytes>maximumBytes)throw Error("Finite browser shim extent required");for(const[name,path]of artifactFiles(transpiled,maximumFiles-1,maximumBytes-bytes)){const source=optimized&&existsSync(join(optimized,name))?join(optimized,name):path,destination=join(temporary,name),facts=lstatSync(source),extent=facts.size;if(!facts.isFile()||facts.isSymbolicLink()||facts.nlink!==1||destination.length>publicationSchema.definitions.Protocol.const.maximumPathUnits||extent>maximumBytes-bytes)throw Error("Finite assembled browser extent required");bytes+=extent;mkdirSync(dirname(destination),{recursive:true});await withArtifactDescriptorSlotV1(()=>withArtifactDescriptorSlotV1(()=>copyFile(source,destination)));}writeFileSync(join(temporary,PLUGIN_HOST_SHIM_FILE),shim);rewritePreview2ShimImports(join(temporary,`${componentBaseV1(work.target)}.js`),vendor);checkResourceLeaseControl(work.control);return temporary;
}

/** 🔎️ The probe stage: the assembled module's own `describe()` under JSPI, cached by the digest of every module file and the node version. */
async function probeComponentModuleV1(work: ComponentModuleWorkV1, module: string,vendor:string): Promise<Uint8Array> {
  checkResourceLeaseControl(work.control);
  const inputs: Record<string, string> = {};
  for (const [name, path] of artifactFiles(module)) inputs[name] = await fileDigest(path, work.control.signal);
  for (const [name, path] of artifactFiles(vendor)) inputs[`vendor/${name}`] = await fileDigest(path, work.control.signal);
  const node = await toolIdentityV1("node", () => captureOwnedCommandV1("node", ["--version"], work.repoRoot, { control: work.control }).then((text) => text.trim())), output = join(work.work, "probe");
  await work.cache.run({ stage: "probe", schemaVersion: 1, inputs, tools: { node } }, output, async (directory) => { writeFileSync(join(directory, "semantic.bin"), await probeModuleDescriptor(join(module, `${componentBaseV1(work.target)}.js`), work.repoRoot, work.control, work.environment)); }, work.control.signal);
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
export async function publishComponentBrowserV1(work:ComponentModuleWorkV1,transpiled:string,optimized:string|null,receipt:ComponentReceiptV1):Promise<void>{
 const mode=readNativeCallerAuthorityV1(work.invocation.capabilities).original.buildMode,policy=publicationSchema.definitions.Protocol.const;if(receipt.profile!==mode)throw Error("Exact actual browser publication mode required");
 await withRepositoryCacheStoreV1(work.repoRoot,"products",work.invocation,work.control,(store,storePhase)=>withRepositoryArtifactV1({workspace:work.repoRoot,invocation:work.invocation,storage:store.storage,control:store.control,phaseControl:storePhase,request:{version:1,path:join(builtPluginModulesRootIn(work.repoRoot,mode),PREVIEW2_VENDOR_RELATIVE),owner:"browser-support:preview2",maximumBytes:policy.maximumOutputBytes+policy.maximumManifestBytes,maximumFiles:policy.maximumFiles}},reader=>withRepositoryArtifactTaskV1(reader,work.invocation,async artifact=>{
  await withArtifactScratchV1({workspace:work.repoRoot,invocation:work.invocation,storage:store.storage,control:store.control,phaseControl:artifact.control,request:{version:1,mode,owner:"browser/module/"+work.target.pluginId}},scratch=>withArtifactDestinationTaskV1(scratch,work.invocation,async sink=>{
   let total=0;const vendor=join(sink.directory,PREVIEW2_VENDOR_RELATIVE);for(const name of artifact.files){const source=join(artifact.path,name),destination=join(vendor,name),size=lstatSync(source).size;if(destination.length>policy.maximumPathUnits||size>policy.maximumOutputBytes-total)throw Error("Browser vendor scratch extent exceeded");total+=size;mkdirSync(dirname(destination),{recursive:true});await withArtifactDescriptorSlotV1(()=>withArtifactDescriptorSlotV1(()=>copyFile(source,destination)));}
   const module=await assembleComponentModuleV1({...work,control:sink.control},transpiled,optimized,join(sink.directory,"module"),vendor,policy.maximumFiles-artifact.files.length,policy.maximumOutputBytes-total);artifactFiles(sink.directory);const semantic=await probeComponentModuleV1({...work,control:sink.control},module,vendor);await artifact.recheck();await publishComponentModuleV1({...work,control:sink.control},module,semantic,receipt,policy.maximumFiles-artifact.files.length,policy.maximumOutputBytes-total);
  }));
 })));
}
