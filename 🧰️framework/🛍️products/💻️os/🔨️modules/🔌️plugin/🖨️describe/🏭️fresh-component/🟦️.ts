import {captureOwnedProcess} from "../../../../../../🔨️modules/🏃️process/📥️capture/🟦️.ts";
import {scriptInvocationEnvironment} from "../../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import { cargoProvenancePhysicalControlV1, writeCompletedCargoInvocationProvenanceV1 } from "../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🟦️.ts";
import { createHash } from "node:crypto";
import { tmpdir,homedir } from "node:os";
import {CurrentPhysicalOwnerV1} from "../../../../../../🔨️modules/📁️filesystem/🧾️observation/📁️current/🟦️.ts";
import { closeSync, existsSync, fsyncSync, lstatSync, mkdirSync, mkdtempSync, openSync, readFileSync, realpathSync, readdirSync, renameSync, rmSync, writeFileSync, writeSync } from "node:fs";
import { dirname, isAbsolute, join, relative, resolve } from "node:path";
import { acquireCargoBuildLeaseV1 } from "../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🔒️lease/🟦️.ts";
import {selectedCargoArguments} from "../../../../../🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/📤️arguments/🟦️.ts";
import {cargoCommandPreparationV1} from "../../../../../🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🏃️command/🟦️.ts";
import {CargoCliOperationOwner,CargoCommandFailure,runCargoCliCommand} from "../../../../../🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🚪️cli/🟦️.ts";
import {parseFreshProcessPolicyV1,type FreshProcessControlV1} from "./🎛️control/🟦️.ts";
import { cargoDirectories } from "../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🟦️.ts";
import { repoCacheDirectory } from "../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/🟦️.ts";
import { isGeneratedPath } from "../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/🟦️.ts";
import { devToolingEnv, readStableBuildFile, resolveWorkspaceBin } from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { semanticOwnedInputFileSnapshot } from "../../../../../🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";
import { blake3Hex } from "../../../../../../🔨️modules/🔏️hash/🟦️.ts";
import { verifySemanticDescriptorPairBytesV1 } from "../../📇️registry/✅️catalog-verification/🟦️.ts";
import { findPluginCargoFiles, parseComponentSourceOwnerV1, DESCRIPTOR_JSON_REL_PATH } from "../../📇️registry/🔎️discovery/🟦️.ts";
import { COMPONENT_RECEIPT_SCHEMA_ID, COMPONENT_TARGET, SHIPPED_COMPONENT_MAX_BYTES, componentProvenanceDigestsV1, componentReceiptValueV1, rustToolchainIdentityV1, type ComponentReceiptV1 } from "../🏷️component-receipt/🟦️.ts";
import { decodeCargoProvenanceV1 } from "../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧾️receipt/🟦️.ts";
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
    if(preparation)preparation={...preparation,environment:{...preparation.environment,SEMIO_COMPILER_RESOURCE_ROOT:observedEnv.SEMIO_COMPILER_RESOURCE_ROOT}};
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
    if (command === "cargo") lease = await acquireCargoBuildLeaseV1({ directory: policy.storage.directory, buildDirectory: directories!.build, args: selectedArgs, signal: controller.signal, onWait: () => freshCheckpoint(control, "wait-build-lease", completed, total) });
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
    if (!Buffer.from(descriptorBytes).equals(Buffer.from(committedPack))) throw new Error(`fresh ${request.pluginId} bytes describe a different semantic descriptor than the committed pair; commit it with \`bun nx run workspace:components describe ${request.pluginId}\``);
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
async function withFreshComponentLease<T>(component: Uint8Array, control: FreshBuildControlV1, derive: (lease: FreshComponentLeaseV1) => Promise<T>): Promise<T> {
  let open = true,
    used = false,
    pending: Promise<unknown> | undefined;
  const lease: FreshComponentLeaseV1 = Object.freeze({
    consume<R>(consumer: (bytes: Uint8Array<ArrayBuffer>) => Promise<R>): Promise<R> {
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
          const result = await consumer(loan);
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
  derive: (lease: FreshComponentLeaseV1) => Promise<T>,
): Promise<FreshComponentProducedV1<T>> {
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
    const derived = await withFreshComponentLease(snapshot.componentBytes, control, derive);
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

/** 🧬️ Builds and stages verified inputs, requiring derivation before its private raw owner retires. */
export async function produceFreshComponentV1<T>(
  repoRoot: string,
  request: FreshComponentRequestV1,
  freshTargetRoot: string,
  packageStageRoot: string,
  control: FreshProcessControlV1,
  derive: (lease: FreshComponentLeaseV1) => Promise<T>,
): Promise<FreshComponentProducedV1<T>> {
  parseFreshProcessPolicyV1(control.process);
  if (typeof derive !== "function") throw new Error("fresh component derivation callback is required");
  if (!/^[a-z0-9]+(?:-[a-z0-9]+)*$/u.test(request.pluginId) || !/^semio:[a-z0-9]+(?:-[a-z0-9]+)*$/u.test(request.componentPackageId) || !/^[a-z0-9]+(?:-[a-z0-9]+)*$/u.test(request.cargoPackage) || !/^[a-z0-9_]+\.wasm$/u.test(request.outputName))
    throw new Error("fresh component request identity is not canonical");
  const targetRoot = freshRoot(freshTargetRoot, "fresh component target root");
  const stageRoot = freshRoot(packageStageRoot, "fresh component stage root");
  if (freshPathIsWithin(targetRoot, stageRoot) || freshPathIsWithin(stageRoot, targetRoot)) throw new Error("fresh component target and stage roots must be disjoint");
  if (control.diagnosticsRoot && (freshPathIsWithin(targetRoot, resolve(control.diagnosticsRoot)) || freshPathIsWithin(stageRoot, resolve(control.diagnosticsRoot)))) throw new Error("fresh process evidence must outlive target and stage cleanup");
  const workRoot = join(targetRoot, ".semio-fresh-component-work");
  mkdirSync(workRoot, { mode: 0o700 });
  const env = devToolingEnv({ CARGO_TARGET_DIR: targetRoot, CARGO_INCREMENTAL: "0" });
  const total = 8;
  let componentBytes: Uint8Array | undefined, snapshot: Awaited<ReturnType<typeof captureFreshComponentInputs>> | undefined;
  try {
    const cargo = ["build", "-p", request.cargoPackage, "--lib", "--target", "wasm32-wasip2", "--profile", request.componentProfile];
    const componentInvocation = await freshRun("cargo", cargo, repoRoot, env, control, "build", 0, total);
    const cargoComponent = pluginWasmArtifactPath(repoRoot, request.cargoPackage, request.componentProfile, targetRoot);
    if (cargoComponent !== join(targetRoot, "wasm32-wasip2", request.componentProfile, request.outputName)) throw new Error("fresh component output identity differs from the shared Cargo artifact path");
    freshWasmArtifactSize(cargoComponent, SHIPPED_COMPONENT_MAX_BYTES, "fresh WASIp2 component");
    componentBytes = readStableBuildFile(cargoComponent, SHIPPED_COMPONENT_MAX_BYTES, { remaining: SHIPPED_COMPONENT_MAX_BYTES }, () => freshCheckpoint(control, "snapshot", 1, total));
    if (!componentBytes.byteLength) throw new Error("fresh component is empty");
    const component = join(workRoot, "component.wasm");
    freshStage(componentBytes, component, control, "snapshot", 1, total);
    const jco = resolveWorkspaceBin("@bytecodealliance/jco", repoRoot);
    if (!jco) throw new Error("missing @bytecodealliance/jco workspace binary; run bun install");
    const witPath = join(workRoot, "component.wit");
    await freshRun("node", [jco, "wit", component, "--output", witPath], repoRoot, env, control, "inspect-wit", 1, total);
    const wit = readFileSync(witPath, "utf8");
    if (Buffer.byteLength(wit) > SHIPPED_COMPONENT_MAX_BYTES) throw new Error("fresh component WIT exceeds its fixed boundary");
    const witExports = [...wit.matchAll(/\bexport\s+([^;]+);/gu)]
      .map((match) => {
        const token = match[1]!.trim();
        const iface = token.match(/\/([a-z][a-z0-9-]*)@/u)?.[1];
        return iface ?? token;
      })
      .sort();
    // 🧬️ `codec` joined `world actor`'s exports with the creation-path codec surface (ticket
    // 26/09/18): a component without it cannot answer `pack-schema-hash`/`genesis`/`print-mirror`/
    // `apply-ops`, so a hub that links no Rust codec for its package can neither create nor edit its
    // documents. Requiring it here is the earliest point the absence is visible — at the built
    // artifact, before a descriptor or a catalog generation is derived from it.
    if (!["checkpoint", "codec", "describe", "jobs", "reactor"].every((name) => witExports.includes(name))) throw new Error("fresh component omits a required actor export");
    const descriptorInvocation = await freshRun("cargo", ["build", "-p", CRATE_NAME], repoRoot, env, control, "build-descriptor-emitter", 3, total);
    const emitter = join(targetRoot, "debug", process.platform === "win32" ? `${CRATE_NAME}.exe` : CRATE_NAME);
    const descriptorRoot = join(workRoot, "descriptor");
    mkdirSync(descriptorRoot, { mode: 0o700 });
    await freshRun(emitter, ["describe", component, "--out", descriptorRoot], repoRoot, env, control, "emit-descriptor", 4, total);
    const descriptorPack = join(descriptorRoot, DESCRIPTOR_PACK_FILENAME);
    const descriptorJson = join(descriptorRoot, DESCRIPTOR_JSON_FILENAME);
    snapshot = await captureFreshComponentInputs(repoRoot, request, componentBytes, { descriptorPack, descriptorJson }, control);
    if (!componentInvocation || !descriptorInvocation) throw new Error("fresh component compiler observations are missing");
    const componentReceipt = freshComponentReceiptV1(repoRoot, request, snapshot, componentInvocation, cargoDirectories(repoRoot, env).build, resolve(env.CARGO_HOME ?? join(homedir(), ".cargo")));
    const produced = await stageFreshComponentInputs(request, snapshot, stageRoot, witExports, componentReceipt, control, derive);
    return { ...produced, compilerInvocations: [componentInvocation, descriptorInvocation] };
  } finally {
    componentBytes?.fill(0);
    snapshot?.descriptorBytes.fill(0);
    rmSync(workRoot, { recursive: true, force: true });
  }
}
