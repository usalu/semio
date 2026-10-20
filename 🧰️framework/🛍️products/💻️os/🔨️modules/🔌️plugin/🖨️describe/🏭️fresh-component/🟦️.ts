import {checkResourceLeaseControl,type ResourceLeaseControl} from "@semio-tech/process/leases";
import type {CompletedCargoCompilationViewV1} from "@semio-tech/process/artifacts/native-build/receipt/retention/snapshot";
import {withNativeBuildCompilationTaskV1} from "@semio-tech/process/native-release";
import compilerRetentionPolicy from "@semio-tech/process/artifacts/publication/sources/schema.json";
import {requireIssuedResourceControlV1,requireIssuedResourceControlInvocationV1,type ScriptInvocation} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {type CompilerAdmissionControl} from "@semio-tech/process/compiler-capacity";
import {captureOwnedCommandV1} from "@semio-tech/process/execution";
import {buildModePolicy} from "@semio-tech/process/build-mode";
import { readNativeCallerAuthorityV1, withArtifactScratchV1, withArtifactDestinationTaskV1 } from "@semio-tech/repo-lib/artifacts";
import { repositoryCargoNetworkV1, selectedCargoArguments, cargoCommandPreparationV1, CargoCliOperationOwner, CargoCommandFailure, runCargoCliCommand } from "@semio-tech/repo-lib/cargo";
import { withRepositoryCacheStoreV1 } from "@semio-tech/repo-lib/storage";
import {buildComponentClosureV1} from "../../🏗️build/🧩️driver-stages/🟦️.ts";
import {withComponentModelV1,type ComponentModelV1} from "../../📦️model/👓️reading/🟦️.ts";
import {readArtifactBytesV1} from "../../🌐️browser-bundle/📦️distribution/📋️inventory/🟦️.ts";
import {componentStageTargetV1} from "../🧩️component-stage/🟦️.ts";
import {buildDescriptorEmitterV1,withDescriptorEmitterV1,withDescriptorEmitterTaskV1} from "../🛠️emitter/🟦️.ts";
import {captureOwnedProcess} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import {scriptInvocationEnvironment} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { cargoProvenancePhysicalControlV1, writeCompletedCargoInvocationProvenanceV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🟦️.ts";
import { createHash } from "node:crypto";
import { tmpdir,homedir } from "node:os";
import {CurrentPhysicalOwnerV1} from "@semio-tech/framework-filesystem/observation/current";
import { closeSync, existsSync, fsyncSync, lstatSync, mkdirSync, mkdtempSync, openSync, readFileSync, realpathSync, readdirSync, renameSync, rmSync, writeFileSync, writeSync } from "node:fs";
import { dirname, isAbsolute, join, relative, resolve } from "node:path";
import { acquireCargoBuildLeaseV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🟦️.ts";
import {parseFreshProcessPolicyV1,type FreshProcessControlV1} from "./🎛️control/🟦️.ts";
import { cargoDirectories, resolveWorkspaceBin } from "@semio-tech/process/execution";
import { isGeneratedPath } from "@semio-tech/repo-lib/process";
import { readStableBuildFile } from "@semio-tech/repo-lib/build";
import { blake3Hex } from "@semio-tech/framework";
import { verifySemanticDescriptorPairBytesV1 } from "@semio-tech/plugin-registry/catalog-verification/descriptor-pair";
import { findPluginCargoFiles, parseComponentSourceOwnerV1, DESCRIPTOR_JSON_REL_PATH } from "@semio-tech/plugin-registry/discovery";
import { COMPONENT_RECEIPT_SCHEMA_ID, COMPONENT_TARGET, SHIPPED_COMPONENT_MAX_BYTES, componentProvenanceDigestsV1, componentReceiptValueV1, rustToolchainIdentityV1, type ComponentReceiptV1 } from "../🏷️component-receipt/🟦️.ts";
import { decodeCargoProvenanceV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🧾️receipt/🟦️.ts";
import { DESCRIPTOR_JSON_FILENAME, DESCRIPTOR_PACK_FILENAME, FRESH_DESCRIPTOR_MAX_BYTES, FRESH_IO_CHUNK_BYTES, CRATE_NAME, freshWasmArtifactSize, pluginWasmArtifactPath } from "../🏗️component-build/🟦️.ts";
import { FRESH_SOURCE_EPOCH_LIMITS, captureFreshSourceEpochV1, freshCheckpoint, freshPathIsWithin, freshSourceEpochBytesV1, freshSourceOrderedJson, parseFreshRustDepInfoV1, type FreshBuildControlV1, type FreshComponentLeaseV1, type FreshComponentProducedV1, type FreshComponentReceiptV1, type FreshComponentRequestV1 } from "../🧾️source-epoch/🟦️.ts";

export async function freshRun(command: string, args: string[], cwd: string, env: NodeJS.ProcessEnv, control: FreshProcessControlV1, stage: string, completed: number, total: number): Promise<string | undefined> {
  const policy=parseFreshProcessPolicyV1(control.process),started=performance.now(),remaining=()=>Math.floor(Math.min(control.remainingMs(),policy.command.maximumElapsedMilliseconds-(performance.now()-started)));
  freshCheckpoint(control, stage, completed, total);
  const budgetMs = remaining();
  if (!Number.isSafeInteger(budgetMs) || budgetMs <= 0 || budgetMs > 86_400_000) throw new Error("fresh process budget must be 1..86400000ms");
  const retained = control.diagnosticsRoot !== undefined;
  const root = control.diagnosticsRoot ?? tmpdir();
  if (!isAbsolute(root) || (retained && !isGeneratedPath(root))) throw new Error("fresh process evidence root must be an absolute directory inside a generated directory");
  const info = lstatSync(root);
  if (!info.isDirectory() || info.isSymbolicLink()) throw new Error("fresh process evidence root must be a regular directory");
  if (command === "cargo" && args.some((arg) => arg === "--message-format" || arg.startsWith("--message-format="))) throw new Error("fresh Cargo diagnostics format is producer-owned");
  let selectedArgs=args,preparation:Awaited<ReturnType<typeof cargoCommandPreparationV1>>;
  if(command==="cargo"){
    const owner=new CargoCliOperationOwner(policy.command.control,{remainingMilliseconds:remaining,publish:row=>control.checkpoint("select-cargo:"+row.stage,row.completed,policy.command.control.discovery.maximumUnits),yieldContinuation:()=>new Promise<void>(accept=>setImmediate(accept))}),watch=setInterval(()=>{if(control.cancelled()||remaining()<=0)owner.cancelDiscovery();},25);
    try{if(control.cancelled())owner.cancelDiscovery();const selected=await runCargoCliCommand(owner,async()=>{const selectedArgs=await selectedCargoArguments(cwd,args,policy.command.scope,owner.operation),argv=[...selectedArgs,"--message-format=json"],preparation=await cargoCommandPreparationV1({root:cwd,cwd,environment:env,storage:policy.storage},argv,owner.operation);return{selectedArgs,preparation};});selectedArgs=selected.selectedArgs;preparation=selected.preparation;}
    catch(error){if(error instanceof CargoCommandFailure)throw new Error("fresh Cargo selection refused: "+(error.commandCause instanceof Error?error.commandCause.message:String(error.commandCause))+"; retirement="+(error.retirementCause instanceof Error?error.retirementCause.message:String(error.retirementCause)),{cause:error});throw error;}
    finally{clearInterval(watch);}
  }
  const argv = command === "cargo" ? [...selectedArgs, "--message-format=json"] : selectedArgs;
  const trace = mkdtempSync(join(root, "fresh-process-"));
  const evidence = retained ? trace : "ephemeral";
  if (retained) console.log("fresh-component-process: stage=" + stage + " evidence=" + evidence);
  const controller = new AbortController();
  let preparing = false;
  const observe = () => { if (control.cancelled() || remaining() <= 0) controller.abort(); else if (preparing) control.checkpoint("prepare-cargo", completed, total); };
  const watch = setInterval(observe, 100);
  let lease: Awaited<ReturnType<typeof acquireCargoBuildLeaseV1>> | undefined;
  try {
    observe();
    const directories = command === "cargo" ? cargoDirectories(cwd, env) : undefined;
    const observedEnv = directories ? { ...env, SEMIO_COMPILER_RESOURCE_ROOT: join(directories.build, "semio-compiler-resources") } : env;
    if(preparation&&directories)preparation={...preparation,environment:{...preparation.environment,SEMIO_COMPILER_RESOURCE_ROOT:join(directories.build,"semio-compiler-resources")}};
    if (preparation) {
      freshCheckpoint(control, "prepare-cargo", completed, total);
      const preparationTrace = join(trace, "preparation");
      mkdirSync(preparationTrace);
      preparing = true;
      const result = await captureOwnedProcess(preparation.command, [...preparation.args], {
        invocation: control.invocation,
        cwd: preparation.cwd,
        env: scriptInvocationEnvironment(control.invocation, preparation.environment),
        budgetMs: Math.min(budgetMs, remaining()),
        maxOutputBytes: 64 * 1024 * 1024,
        stdoutPath: join(preparationTrace, "stdout.jsonl"),
        stderrPath: join(preparationTrace, "stderr.txt"),
      });
      preparing = false;
      const reason = result.reason ?? "exit";
      writeFileSync(join(preparationTrace, "outcome.json"), JSON.stringify({ schema: "semio.plugin.fresh-process/v1", stage: "prepare-cargo", command: preparation.command, args: preparation.args, status: result.status, signal: result.signal, reason }) + "\n", { flag: "wx", mode: 0o600 });
      if (result.status !== 0 || result.signal !== null || reason !== "exit") throw new Error("fresh component preparation " + reason + " at " + stage + "; evidence=" + evidence + "\n" + (result.stderr || result.stdout).slice(-6000));
      freshCheckpoint(control, stage, completed, total);
    }
    if (command === "cargo") lease = await acquireCargoBuildLeaseV1({ directory: policy.storage.directory, buildDirectory: directories!.build, targetDirectory: directories!.target, args: selectedArgs, signal: controller.signal, onWait: () => freshCheckpoint(control, "wait-build-lease", completed, total) });
    if (command === "cargo") freshCheckpoint(control, stage, completed, total);
    const builtAtMs = Date.now();
    const result = await captureOwnedProcess(command, argv, {
      invocation: control.invocation,
      cwd,
      env: observedEnv,
      budgetMs: Math.min(budgetMs, remaining()),
      maxOutputBytes: 64 * 1024 * 1024,
      stdoutPath: join(trace, "stdout.jsonl"),
      stderrPath: join(trace, "stderr.txt"),
    });
    const reason = result.reason ?? "exit";
    let provenance: string | undefined;
    if (directories) {
      const messages = result.stdout.split(/\r?\n/u).flatMap(line => { try { return [JSON.parse(line)]; } catch { return []; } });
      const manifestIndex = selectedArgs.indexOf("--manifest-path");
      const manifest = resolve(cwd, manifestIndex >= 0 ? selectedArgs[manifestIndex + 1]! : selectedArgs.find(arg => arg.startsWith("--manifest-path="))?.slice("--manifest-path=".length) ?? "Cargo.toml");
      provenance = join(directories.build, "semio-cargo-provenance", trace.split(/[\\/]/u).at(-1) + ".json");
      const physical=new CurrentPhysicalOwnerV1(cwd,cargoProvenancePhysicalControlV1({cancelled:()=>control.cancelled(),remainingMs:()=>remaining(),onProgress:()=>freshCheckpoint(control,"compiler-provenance",completed,total)}));
      await writeCompletedCargoInvocationProvenanceV1(provenance, { manifest, cwd, command, args: argv, buildDirectory: directories.build, builtAtMs, status: result.status ?? -1, cancelled: reason !== "exit", units: messages.filter(message => message.reason === "compiler-artifact").map(message => ({message,evidence:{version:1,kind:"discovery",paths:[],producer:null}})), buildScripts: messages.filter(message => message.reason === "build-script-executed") },new Map(),resolve(env.CARGO_HOME??join(homedir(),".cargo")),physical);
    }
    writeFileSync(join(trace, "outcome.json"), JSON.stringify({ schema: "semio.plugin.fresh-process/v1", stage, command, args: argv, cargoTargetDir: env.CARGO_TARGET_DIR ?? null, status: result.status, signal: result.signal, reason }) + "\n", {
      flag: "wx",
      mode: 0o600,
    });
    if (result.status !== 0 || result.signal !== null || reason !== "exit") {
      const errors: string[] = [];
      for (const line of result.stdout.split("\n")) {
        try {
          const record = JSON.parse(line);
          if (record?.reason === "compiler-message" && record.message?.level === "error") {
            const rendered = record.message.rendered ?? record.message.message;
            if (typeof rendered === "string") errors.push(rendered.slice(0, 6000));
          }
        } catch {}
        if (errors.length >= 3) break;
      }
      const detail = (errors.length ? errors.join("\n") : result.stderr || result.stdout).slice(-6000);
      throw new Error("fresh component " + reason + " at " + stage + " (status=" + result.status + ", signal=" + result.signal + "); evidence=" + evidence + "\n" + detail);
    }
    freshCheckpoint(control, stage, completed + 1, total);
    return provenance;
  } finally {
    try { lease?.release(); }
    finally { clearInterval(watch); if (!retained) rmSync(trace, { recursive: true, force: true }); }
  }
}

function freshRoot(path: string, label: string): string {
  if (!isAbsolute(path)) throw new Error(`${label} must be absolute`);
  const exact = resolve(path);
  const info = lstatSync(exact);
  if (info.isSymbolicLink() || !info.isDirectory() || readdirSync(exact).length !== 0) throw new Error(`${label} must be an empty regular directory`);
  return realpathSync(exact);
}

export function freshStage(bytes: Uint8Array, destination: string, control: FreshBuildControlV1, stage: string, completed: number, total: number): { byteLength: number; sha256: string } {
  freshCheckpoint(control, stage, completed, total);
  const output = openSync(destination, "wx", 0o600);
  const hash = createHash("sha256");
  let copied = 0;
  let complete = false;
  try {
    while (copied < bytes.byteLength) {
      freshCheckpoint(control, stage, completed, total);
      const chunk = bytes.subarray(copied, Math.min(copied + FRESH_IO_CHUNK_BYTES, bytes.byteLength));
      let written = 0;
      while (written < chunk.byteLength) {
        const count = writeSync(output, chunk, written, chunk.byteLength - written);
        if (!count) throw new Error(`${stage} could not advance snapshot write`);
        written += count;
      }
      hash.update(chunk);
      copied += chunk.byteLength;
    }
    fsyncSync(output);
    freshCheckpoint(control, stage, completed + 1, total);
    complete = true;
    return { byteLength: copied, sha256: hash.digest("hex") };
  } finally {
    closeSync(output);
    if (!complete) rmSync(destination, { force: true });
  }
}

/** 📍️ The committed semantic descriptor pack of the component crate named by its Cargo package, found through registry
 * discovery — the descriptor a fresh build's own `describe()` answer must equal byte for byte. */
export function committedSemanticDescriptorPackV1(repoRoot: string, cargoPackage: string): Uint8Array {
  const owner = findPluginCargoFiles(repoRoot).map((path) => parseComponentSourceOwnerV1(path, repoRoot)).find((row) => row.packageName === cargoPackage);
  if (!owner) throw new Error(`fresh component ${cargoPackage} is no discovered plugin or extension component`);
  return readFileSync(join(dirname(resolve(repoRoot, owner.cratePath, ...DESCRIPTOR_JSON_REL_PATH)), DESCRIPTOR_PACK_FILENAME));
}

/** 🪪️ Verifies the emitted SEMANTIC descriptor of the fresh bytes and refuses it unless it equals the committed semantic
 * descriptor: that pack is what the trusted catalog serves, bound to the bytes only by the component receipt. */
export async function captureFreshComponentInputs(
  repoRoot: string,
  request: Pick<FreshComponentRequestV1, "pluginId" | "componentPackageId" | "cargoPackage">,
  componentBytes: Uint8Array,
  paths: { descriptorPack: string; descriptorJson: string },
  control: FreshBuildControlV1,
  committedPack: Uint8Array = committedSemanticDescriptorPackV1(repoRoot, request.cargoPackage),
) {
  const admission = { remaining: 2 * FRESH_DESCRIPTOR_MAX_BYTES };
  const check = () => freshCheckpoint(control, "verify", 5, 8);
  let descriptorJsonBytes: Uint8Array | undefined, descriptorBytes: Uint8Array | undefined;
  let complete = false;
  try {
    descriptorJsonBytes = readStableBuildFile(paths.descriptorJson, FRESH_DESCRIPTOR_MAX_BYTES, admission, check);
    descriptorBytes = readStableBuildFile(paths.descriptorPack, FRESH_DESCRIPTOR_MAX_BYTES, admission, check);
    const semantic = verifySemanticDescriptorPairBytesV1(descriptorJsonBytes, descriptorBytes);
    if (semantic.packageId !== request.componentPackageId || semantic.pluginId !== request.pluginId || semantic.version.length === 0 || semantic.role !== "plugin")
      throw new Error("fresh descriptor identity differs from the exact component request");
    if (!Buffer.from(descriptorBytes).equals(Buffer.from(committedPack))) throw new Error(`fresh ${request.pluginId} bytes describe a different semantic descriptor than the committed pair; commit it with \`bun repo run workspace:components describe ${request.pluginId}\``);
    check();
    const result = {
      componentBytes,
      descriptorBytes,
      componentSha256: createHash("sha256").update(componentBytes).digest("hex"),
      componentBlake3: blake3Hex(componentBytes),
      descriptorSha256: semantic.semanticSha256,
      version: semantic.version,
    };
    complete = true;
    return result;
  } finally {
    descriptorJsonBytes?.fill(0);
    if (!complete) descriptorBytes?.fill(0);
  }
}

/** 🧾️ The shipped receipt of one fresh production: its bytes, its semantic descriptor, the pinned toolchain and the provenance
 * digests of exactly the units its component build observed (`provenance`, that build's cargo provenance receipt). */
export function freshComponentReceiptV1(repoRoot: string, request: Pick<FreshComponentRequestV1, "pluginId" | "componentPackageId" | "componentProfile">, snapshot: Readonly<{ componentSha256: string; descriptorSha256: string; componentBytes: Uint8Array }>, provenance: string, buildDirectory: string, cargoHome: string): ComponentReceiptV1 {
  const digests = componentProvenanceDigestsV1(decodeCargoProvenanceV1(readFileSync(provenance, "utf8")), { repository: repoRoot, build: buildDirectory, cargoHome });
  return componentReceiptValueV1({
    schema: COMPONENT_RECEIPT_SCHEMA_ID,
    pluginId: request.pluginId,
    packageId: request.componentPackageId,
    profile: request.componentProfile,
    target: COMPONENT_TARGET,
    wasmSha256: snapshot.componentSha256,
    wasmBytes: snapshot.componentBytes.byteLength,
    semanticDescriptorSha256: snapshot.descriptorSha256,
    toolchain: rustToolchainIdentityV1(repoRoot),
    sourceTreeDigest: digests.sourceTreeDigest,
    provenanceSha256: digests.provenanceSha256,
  });
}

/** 🫴️ Loans a private bounded copy once and drains its consumer before retiring the capability. */
async function withFreshComponentLease<T>(component: Uint8Array, control: FreshBuildControlV1, phaseControl: ResourceLeaseControl, derive: (lease: FreshComponentLeaseV1) => Promise<T>): Promise<T> {
  requireIssuedResourceControlV1(phaseControl);checkResourceLeaseControl(phaseControl);
  let open = true,
    used = false,
    pending: Promise<unknown> | undefined;
  const lease: FreshComponentLeaseV1 = Object.freeze({
    consume<R>(consumer: (bytes: Uint8Array<ArrayBuffer>, phaseControl: ResourceLeaseControl) => Promise<R>): Promise<R> {
      if (!open) return Promise.reject(new Error("fresh component lease expired"));
      if (used) return Promise.reject(new Error("fresh component lease already consumed"));
      used = true;
      const operation = (async () => {
        freshCheckpoint(control, "derive", 7, 8);
        const loan = new Uint8Array(component.byteLength);
        try {
          for (let offset = 0; offset < component.byteLength; offset += FRESH_IO_CHUNK_BYTES) {
            freshCheckpoint(control, "derive-copy", offset, component.byteLength);
            loan.set(component.subarray(offset, Math.min(offset + FRESH_IO_CHUNK_BYTES, component.byteLength)), offset);
          }
          freshCheckpoint(control, "derive", 7, 8);
          checkResourceLeaseControl(phaseControl);
          const result = await consumer(loan, phaseControl);
          checkResourceLeaseControl(phaseControl);
          freshCheckpoint(control, "derive", 8, 8);
          return result;
        } finally {
          loan.fill(0);
        }
      })();
      pending = operation;
      void operation.catch(() => {});
      return operation;
    },
  });
  try {
    freshCheckpoint(control, "derive", 7, 8);
    const result = await derive(lease);
    open = false;
    if (!used) throw new Error("fresh component lease was not consumed");
    await pending;
    freshCheckpoint(control, "derive", 8, 8);
    return result;
  } finally {
    open = false;
    await pending?.catch(() => {});
  }
}

/** 🧊️ Stages verified inputs, settles their required derivation and retires every source owner. */
export async function stageFreshComponentInputs<T>(
  request: Pick<FreshComponentRequestV1, "pluginId" | "componentPackageId">,
  snapshot: Awaited<ReturnType<typeof captureFreshComponentInputs>>,
  stageRoot: string,
  witExports: readonly string[],
  componentReceipt: ComponentReceiptV1,
  control: FreshBuildControlV1,
  phaseControl: ResourceLeaseControl,
  derive: (lease: FreshComponentLeaseV1) => Promise<T>,
): Promise<FreshComponentProducedV1<T>> {
  requireIssuedResourceControlV1(phaseControl);checkResourceLeaseControl(phaseControl);const original=control,{cancelled,remainingMs,checkpoint,diagnosticsRoot}=original;control=Object.freeze({diagnosticsRoot,cancelled:()=>phaseControl.signal.aborted||cancelled.call(original),remainingMs:()=>Math.min(checkResourceLeaseControl(phaseControl)??Infinity,remainingMs.call(original)),checkpoint:(stage:string,completed:number,total:number)=>{checkResourceLeaseControl(phaseControl);checkpoint.call(original,stage,completed,total);checkResourceLeaseControl(phaseControl);}});
  const ownedFiles: string[] = [];
  try {
    const componentPath = join(stageRoot, "component.wasm"),
      descriptorPath = join(stageRoot, "descriptor.semio");
    const stagedComponent = freshStage(snapshot.componentBytes, componentPath, control, "stage-component", 5, 8);
    ownedFiles.push(componentPath);
    const stagedDescriptor = freshStage(snapshot.descriptorBytes, descriptorPath, control, "stage-descriptor", 6, 8);
    ownedFiles.push(descriptorPath);
    if (stagedComponent.sha256 !== snapshot.componentSha256 || stagedDescriptor.sha256 !== snapshot.descriptorSha256) throw new Error("fresh staged bytes differ from the verified snapshot");
    if (componentReceipt.wasmSha256 !== snapshot.componentSha256 || componentReceipt.wasmBytes !== stagedComponent.byteLength || componentReceipt.semanticDescriptorSha256 !== snapshot.descriptorSha256 || componentReceipt.pluginId !== request.pluginId || componentReceipt.packageId !== request.componentPackageId) throw new Error("fresh component receipt does not name the staged bytes");
    const receipt: FreshComponentReceiptV1 = Object.freeze({
      pluginId: request.pluginId,
      packageId: request.componentPackageId,
      version: snapshot.version,
      component: Object.freeze({ relativePath: "component.wasm", ...stagedComponent, blake3: snapshot.componentBlake3 }),
      descriptor: Object.freeze({ relativePath: "descriptor.semio", ...stagedDescriptor }),
      componentReceipt,
      witExports: Object.freeze([...witExports]),
    });
    const derived = await withFreshComponentLease(snapshot.componentBytes, control, phaseControl, derive);
    freshCheckpoint(control, "complete", 8, 8);
    return Object.freeze({ receipt, derived });
  } catch (error) {
    for (const path of ownedFiles) rmSync(path, { force: true });
    throw error;
  } finally {
    snapshot.componentBytes.fill(0);
    snapshot.descriptorBytes.fill(0);
  }
}

export interface FreshCompilerObservationV1{readonly kind:"component"|"emitter";readonly pass:string;readonly bytes:Uint8Array;readonly sha256:string;readonly originalArtifact:Readonly<{path:string;sha256:string;byteLength:number}>|null}
export interface FreshComponentEvidenceV1{withCompilation<T>(kind:"component"|"emitter",pass:string,body:(view:CompletedCargoCompilationViewV1)=>Promise<T>):Promise<T>;readonly control:CompilerAdmissionControl;readonly observations:readonly FreshCompilerObservationV1[];withEmitter<T>(body:(tool:Readonly<{path:string;sha256:string;control:CompilerAdmissionControl;recheck():Promise<void>}>)=>Promise<T>):Promise<T>}
export type FreshComponentProductionControlV1=FreshBuildControlV1&Readonly<{invocation:ScriptInvocation;compiler:CompilerAdmissionControl}>;
/** 🧬️ Consumes a genuinely fresh canonical model and real compiler observations before the driver cleans its generation. */
export async function produceFreshComponentV1<T>(repoRoot:string,request:FreshComponentRequestV1,packageStageRoot:string,control:FreshComponentProductionControlV1,derive:(lease:FreshComponentLeaseV1,evidence:FreshComponentEvidenceV1)=>Promise<T>):Promise<FreshComponentProducedV1<T>>{
 requireIssuedResourceControlInvocationV1(control.compiler,control.invocation);const authority=readNativeCallerAuthorityV1(control.invocation.capabilities),mode=authority.original.buildMode,target=componentStageTargetV1(repoRoot,join(repoRoot,componentSourceManifestV1(repoRoot,request.cargoPackage)));
 if(repoRoot!==authority.repositoryRoot||request.componentProfile!==buildModePolicy(mode).cargoProfile||target.pluginId!==request.pluginId||target.packageId!==request.componentPackageId||target.packageName!==request.cargoPackage||request.outputName!==request.cargoPackage.replaceAll("-","_")+".wasm"||typeof derive!=="function")throw Error("Exact actual Original fresh component selection required");const stage=freshRoot(packageStageRoot,"fresh component stage root");await buildDescriptorEmitterV1(repoRoot,control.invocation,control.compiler);let completed=false,produced:FreshComponentProducedV1<T>|undefined;
 await withDescriptorEmitterV1(repoRoot,control.invocation,control.compiler,(emitter,emitterPhase)=>buildComponentClosureV1(repoRoot,new Set([request.pluginId]),{invocation:control.invocation,facet:"model",network:repositoryCargoNetworkV1(control.invocation,repoRoot),control:emitterPhase,generation:"fresh",retainCompilationBytes:compilerRetentionPolicy.properties.maximumBytes.const,maximumObservationBytes:authority.transport.maximumBytes,consume:async(outcome,phase)=>withDescriptorEmitterTaskV1(emitter,control.invocation,async tool=>{
  const observations:FreshCompilerObservationV1[]=[],maximum=authority.transport.maximumBytes;let retained=0;
  try{for(const row of outcome.observations){const bytes=await readArtifactBytesV1(row.path,maximum-retained,phase.signal);retained+=bytes.length;const record=decodeCargoProvenanceV1(new TextDecoder("utf-8",{fatal:true}).decode(bytes));if(record.status!==0||record.cancelled)throw Error("Actual successful fresh compiler observation required");observations.push(Object.freeze({kind:"component",pass:row.pass,bytes,sha256:createHash("sha256").update(bytes).digest("hex"),originalArtifact:null}));}const emitterBytes=new TextEncoder().encode(tool.observation);if(emitterBytes.length>maximum-retained||!observations.length)throw Error("Finite actual component and emitter observations required");observations.push(Object.freeze({kind:"emitter",pass:"descriptor-emitter",bytes:emitterBytes,sha256:createHash("sha256").update(emitterBytes).digest("hex"),originalArtifact:null}));
   await withComponentModelV1(repoRoot,control.invocation,{version:1,mode,pluginId:request.pluginId,cratePath:target.cratePath},phase,async model=>{const componentBytes=await readArtifactBytesV1(model.component,SHIPPED_COMPONENT_MAX_BYTES,model.control.signal);let snapshot:Awaited<ReturnType<typeof captureFreshComponentInputs>>|undefined;try{snapshot=await captureFreshComponentInputs(repoRoot,request,componentBytes,{descriptorPack:model.descriptorPack,descriptorJson:model.descriptor},control);await model.recheck();const withCompilation=<R>(kind:"component"|"emitter",pass:string,body:(view:CompletedCargoCompilationViewV1)=>Promise<R>):Promise<R>=>{if(kind==="emitter"&&pass==="descriptor-emitter")return withDescriptorEmitterTaskV1(emitter,control.invocation,actual=>body(actual.compilation));const row=kind==="component"?outcome.compilations.find(row=>row.pass===pass):undefined;if(!row)throw Error("Exact actual completed fresh compiler pass required");return withNativeBuildCompilationTaskV1(row.capture,body);},bound:FreshCompilerObservationV1[]=[];let selectedComponents=0;
   for(const row of observations)await withCompilation(row.kind,row.pass,async actual=>{if(actual.observationSha256!==row.sha256||createHash("sha256").update(actual.observation).digest("hex")!==row.sha256)throw Error("Exact unchanged actual completed compiler receipt required");const record=decodeCargoProvenanceV1(actual.observation),expected=row.kind==="component"?model.receipt.wasmSha256:tool.sha256,matches=record.units.filter(unit=>row.kind==="emitter"||unit.message.manifest_path===join(repoRoot,target.cratePath,"Cargo.toml")).flatMap(unit=>unit.artifacts.filter(artifact=>artifact.sha256===expected));if(matches.length>1||row.kind==="emitter"&&matches.length!==1)throw Error("One actual selected component or emitter compiler output required");const match=matches[0];let originalArtifact:FreshCompilerObservationV1["originalArtifact"]=null;if(match){const file=actual.files.get("blobs/"+match.sha256);if(!file||file.sha256!==expected||row.kind==="component"&&(file.byteLength!==model.receipt.wasmBytes||createHash("sha256").update(componentBytes).digest("hex")!==expected))throw Error("Real model/emitter bytes differ from its exact compiler output");originalArtifact=Object.freeze({path:match.path,sha256:file.sha256,byteLength:file.byteLength});if(row.kind==="component")selectedComponents++;}bound.push(Object.freeze({...row,originalArtifact}));await actual.recheck();});if(selectedComponents!==1)throw Error("Exactly one actual compiler output must bind the selected model");
   const evidence:FreshComponentEvidenceV1=Object.freeze({control:model.control,observations:Object.freeze(bound),withCompilation,withEmitter:<R>(body:(actual:Readonly<{path:string;sha256:string;control:CompilerAdmissionControl;recheck():Promise<void>}>)=>Promise<R>)=>withDescriptorEmitterTaskV1(emitter,control.invocation,body)});produced=await stageFreshComponentInputs(request,snapshot,stage,await freshComponentWitExportsV1(repoRoot,control.invocation,model,control),model.receipt,control,model.control,lease=>derive(lease,evidence));completed=true;await model.recheck();}finally{componentBytes.fill(0);snapshot?.descriptorBytes.fill(0);}});await tool.recheck();
  }finally{for(const row of observations)row.bytes.fill(0);}
 })}));
 if(!completed||!produced)throw Error("Actual fresh model consumer did not complete");return produced;
}

function componentSourceManifestV1(repoRoot:string,packageName:string):string{const rows=findPluginCargoFiles(repoRoot).map(path=>parseComponentSourceOwnerV1(path,repoRoot)).filter(row=>row.packageName===packageName);if(rows.length!==1)throw Error("One declared component source owner required");return join(rows[0]!.cratePath,"Cargo.toml");}

async function freshComponentWitExportsV1(repoRoot:string,invocation:ScriptInvocation,model:ComponentModelV1,control:FreshBuildControlV1):Promise<readonly string[]>{
 const jco=resolveWorkspaceBin("@bytecodealliance/jco",repoRoot);if(!jco)throw Error("Declared component inspector required");return withRepositoryCacheStoreV1(repoRoot,"products",invocation,model.control,(store,storePhase)=>withArtifactScratchV1({workspace:repoRoot,invocation,storage:store.storage,control:store.control,phaseControl:storePhase,request:{version:1,mode:model.intent.mode,owner:"browser/fresh-wit/"+model.intent.pluginId}},scratch=>withArtifactDestinationTaskV1(scratch,invocation,async sink=>{const path=join(sink.directory,"component.wit");await captureOwnedCommandV1(process.execPath,[jco,"wit",model.component,"--output",path],repoRoot,{control:sink.control,env:scriptInvocationEnvironment(invocation,process.env)});const bytes=await readArtifactBytesV1(path,SHIPPED_COMPONENT_MAX_BYTES,sink.control.signal),wit=new TextDecoder("utf-8",{fatal:true}).decode(bytes),exports=[...wit.matchAll(/\bexport\s+([^;]+);/gu)].map(match=>match[1]!.trim().match(/\/([a-z][a-z0-9-]*)@/u)?.[1]??match[1]!.trim()).sort();if(!["checkpoint","codec","describe","jobs","reactor"].every(name=>exports.includes(name)))throw Error("Fresh component omits a required actual actor export");freshCheckpoint(control,"inspect-wit",6,8);return Object.freeze(exports);})));}
