import {captureOwnedProcess} from "../../../../../../🔨️modules/🏃️process/📥️capture/🟦️.ts";
import { writeCompletedCargoInvocationProvenanceV1 } from "../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🟦️.ts";
import { createHash } from "node:crypto";
import { tmpdir } from "node:os";
import { closeSync, existsSync, fsyncSync, lstatSync, mkdirSync, mkdtempSync, openSync, readFileSync, realpathSync, readdirSync, renameSync, rmSync, writeFileSync, writeSync } from "node:fs";
import { dirname, isAbsolute, join, relative, resolve } from "node:path";
import { createRequire } from "node:module";
import { acquireCargoBuildLeaseV1 } from "../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🔒️lease/🟦️.ts";
import { cargoWorkspacePreparationInvocationV1, selectedCargoArguments } from "../../../../../🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🟦️.ts";
import { cargoDirectories } from "../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🟦️.ts";
import { repoCacheDirectory } from "../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/🟦️.ts";
import { isGeneratedPath } from "../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/🟦️.ts";
import { devToolingEnv, readStableBuildFile, resolveWorkspaceBin } from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { semanticOwnedInputFileSnapshot } from "../../../../../🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";
import { blake3Hex } from "../../../../../../🔨️modules/🔏️hash/🟦️.ts";
import { verifyFreshCatalogPackageV1 } from "../../📇️registry/✅️catalog-verification/🟦️.ts";
import { DESCRIPTOR_JSON_FILENAME, DESCRIPTOR_PACK_FILENAME, FRESH_COMPONENT_MAX_BYTES, FRESH_DESCRIPTOR_MAX_BYTES, FRESH_IO_CHUNK_BYTES, CRATE_NAME, extractPluginCore, freshWasmArtifactSize, pluginWasmArtifactPath } from "../🏗️component-build/🟦️.ts";
import { emitOwnerDescriptorPairV1, type DescriptorEmissionControlV1 } from "../🛂️descriptor-emission/🟦️.ts";
import { FRESH_SOURCE_EPOCH_LIMITS, captureFreshSourceEpochV1, freshCheckpoint, freshPathIsWithin, freshSourceEpochBytesV1, freshSourceOrderedJson, parseFreshRustDepInfoV1, type FreshBuildControlV1, type FreshComponentLeaseV1, type FreshComponentProducedV1, type FreshComponentReceiptV1, type FreshComponentRequestV1 } from "../🧾️source-epoch/🟦️.ts";

export async function freshRun(command: string, args: string[], cwd: string, env: NodeJS.ProcessEnv, control: FreshBuildControlV1, stage: string, completed: number, total: number): Promise<string | undefined> {
  freshCheckpoint(control, stage, completed, total);
  const budgetMs = control.remainingMs();
  if (!Number.isSafeInteger(budgetMs) || budgetMs <= 0 || budgetMs > 86_400_000) throw new Error("fresh process budget must be 1..86400000ms");
  const retained = control.diagnosticsRoot !== undefined;
  const root = control.diagnosticsRoot ?? tmpdir();
  if (!isAbsolute(root) || (retained && !isGeneratedPath(root))) throw new Error("fresh process evidence root must be an absolute directory inside a generated directory");
  const info = lstatSync(root);
  if (!info.isDirectory() || info.isSymbolicLink()) throw new Error("fresh process evidence root must be a regular directory");
  if (command === "cargo" && args.some((arg) => arg === "--message-format" || arg.startsWith("--message-format="))) throw new Error("fresh Cargo diagnostics format is producer-owned");
  const selectedArgs = command === "cargo" ? selectedCargoArguments(cwd, args) : args;
  const argv = command === "cargo" ? [...selectedArgs, "--message-format=json"] : selectedArgs;
  const trace = mkdtempSync(join(root, "fresh-process-"));
  const evidence = retained ? trace : "ephemeral";
  if (retained) console.log("fresh-component-process: stage=" + stage + " evidence=" + evidence);
  const controller = new AbortController();
  let preparing = false;
  const observe = () => { if (control.cancelled() || control.remainingMs() <= 0) controller.abort(); else if (preparing) control.checkpoint("prepare-cargo", completed, total); };
  const watch = setInterval(observe, 100);
  let lease: Awaited<ReturnType<typeof acquireCargoBuildLeaseV1>> | undefined;
  try {
    observe();
    const directories = command === "cargo" ? cargoDirectories(cwd, env) : undefined;
    const observedEnv = directories ? { ...env, SEMIO_COMPILER_RESOURCE_ROOT: join(directories.build, "semio-compiler-resources") } : env;
    const preparation = command === "cargo" ? cargoWorkspacePreparationInvocationV1(cwd, argv, cwd, observedEnv) : undefined;
    if (preparation) {
      freshCheckpoint(control, "prepare-cargo", completed, total);
      const preparationTrace = join(trace, "preparation");
      mkdirSync(preparationTrace);
      preparing = true;
      const result = await captureOwnedProcess(preparation.command, [...preparation.args], {
        cwd: preparation.cwd,
        env: preparation.environment,
        budgetMs: Math.min(budgetMs, control.remainingMs()),
        maxOutputBytes: 64 * 1024 * 1024,
        stdoutPath: join(preparationTrace, "stdout.jsonl"),
        stderrPath: join(preparationTrace, "stderr.txt"),
        cancelled: () => control.cancelled(),
      });
      preparing = false;
      const reason = result.reason ?? "exit";
      writeFileSync(join(preparationTrace, "outcome.json"), JSON.stringify({ schema: "semio.plugin.fresh-process/v1", stage: "prepare-cargo", command: preparation.command, args: preparation.args, status: result.status, signal: result.signal, reason }) + "\n", { flag: "wx", mode: 0o600 });
      if (result.status !== 0 || result.signal !== null || reason !== "exit") throw new Error("fresh component preparation " + reason + " at " + stage + "; evidence=" + evidence + "\n" + (result.stderr || result.stdout).slice(-6000));
      freshCheckpoint(control, stage, completed, total);
    }
    if (command === "cargo") lease = await acquireCargoBuildLeaseV1({ directory: repoCacheDirectory(cwd, "agents", "resource-leases"), buildDirectory: directories!.build, args: selectedArgs, signal: controller.signal, onWait: () => freshCheckpoint(control, "wait-build-lease", completed, total) });
    if (command === "cargo") freshCheckpoint(control, stage, completed, total);
    const builtAtMs = Date.now();
    const result = await captureOwnedProcess(command, argv, {
      cwd,
      env: observedEnv,
      budgetMs: Math.min(budgetMs, control.remainingMs()),
      maxOutputBytes: 64 * 1024 * 1024,
      stdoutPath: join(trace, "stdout.jsonl"),
      stderrPath: join(trace, "stderr.txt"),
      cancelled: () => control.cancelled(),
    });
    const reason = result.reason ?? "exit";
    let provenance: string | undefined;
    if (directories) {
      const messages = result.stdout.split(/\r?\n/u).flatMap(line => { try { return [JSON.parse(line)]; } catch { return []; } });
      const manifestIndex = selectedArgs.indexOf("--manifest-path");
      const manifest = resolve(cwd, manifestIndex >= 0 ? selectedArgs[manifestIndex + 1]! : selectedArgs.find(arg => arg.startsWith("--manifest-path="))?.slice("--manifest-path=".length) ?? "Cargo.toml");
      provenance = join(directories.build, "semio-cargo-provenance", "cargo-unit-provenance-" + trace.split(/[\\/]/u).at(-1) + ".json");
      writeCompletedCargoInvocationProvenanceV1(provenance, { manifest, cwd, command, args: argv, buildDirectory: directories.build, builtAtMs, status: result.status ?? -1, cancelled: reason !== "exit", units: messages.filter(message => message.reason === "compiler-artifact").map(message => ({message})), buildScripts: messages.filter(message => message.reason === "build-script-executed") }, new Map(), env.CARGO_HOME);
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

/** 🪪️ Verifies descriptor outputs against the raw snapshot retained before extraction. */
export async function captureFreshComponentInputs(
  repoRoot: string,
  request: Pick<FreshComponentRequestV1, "pluginId" | "componentPackageId">,
  componentBytes: Uint8Array,
  coreBytes: Uint8Array,
  paths: { descriptorPack: string; descriptorJson: string },
  control: FreshBuildControlV1,
) {
  const admission = { remaining: 2 * FRESH_DESCRIPTOR_MAX_BYTES };
  const check = () => freshCheckpoint(control, "verify", 5, 8);
  let descriptorJsonBytes: Uint8Array | undefined, descriptorBytes: Uint8Array | undefined;
  let complete = false;
  try {
    descriptorJsonBytes = readStableBuildFile(paths.descriptorJson, FRESH_DESCRIPTOR_MAX_BYTES, admission, check);
    descriptorBytes = readStableBuildFile(paths.descriptorPack, FRESH_DESCRIPTOR_MAX_BYTES, admission, check);
    const projected = JSON.parse(new TextDecoder("utf-8", { fatal: true }).decode(descriptorJsonBytes)) as Record<string, any>;
    const componentSha256 = createHash("sha256").update(componentBytes).digest("hex");
    const coreSha256 = createHash("sha256").update(coreBytes).digest("hex");
    if (projected.packageId !== request.componentPackageId || projected.manifest?.pluginId !== request.pluginId || typeof projected.manifest?.version !== "string" || projected.manifest.version.length === 0 || projected.role !== "plugin")
      throw new Error("fresh descriptor identity differs from the exact component request");
    verifyFreshCatalogPackageV1(descriptorJsonBytes, descriptorBytes, {
      pluginId: request.pluginId,
      packageId: request.componentPackageId,
      version: projected.manifest.version,
      role: "plugin",
      execution: "isolated",
      wasmSha256: componentSha256,
      coreWasmSha256: coreSha256,
    });
    check();
    const result = {
      componentBytes,
      descriptorBytes,
      componentSha256,
      componentBlake3: blake3Hex(componentBytes),
      descriptorSha256: createHash("sha256").update(descriptorBytes).digest("hex"),
      coreSha256,
      version: projected.manifest.version as string,
    };
    complete = true;
    return result;
  } finally {
    descriptorJsonBytes?.fill(0);
    if (!complete) descriptorBytes?.fill(0);
  }
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
    const receipt: FreshComponentReceiptV1 = Object.freeze({
      pluginId: request.pluginId,
      packageId: request.componentPackageId,
      version: snapshot.version,
      component: Object.freeze({ relativePath: "component.wasm", ...stagedComponent, blake3: snapshot.componentBlake3 }),
      descriptor: Object.freeze({ relativePath: "descriptor.semio", ...stagedDescriptor }),
      coreSha256: snapshot.coreSha256,
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
  control: FreshBuildControlV1,
  derive: (lease: FreshComponentLeaseV1) => Promise<T>,
): Promise<FreshComponentProducedV1<T>> {
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
  let componentBytes: Uint8Array | undefined, coreBytes: Uint8Array | undefined, snapshot: Awaited<ReturnType<typeof captureFreshComponentInputs>> | undefined;
  try {
    const cargo = request.rootCdylib
      ? ["rustc", "-p", request.cargoPackage, "--lib", "--crate-type", "cdylib", "--target", "wasm32-wasip2", "--profile", request.componentProfile]
      : ["build", "-p", request.cargoPackage, "--target", "wasm32-wasip2", "--profile", request.componentProfile];
    const componentInvocation = await freshRun("cargo", cargo, repoRoot, env, control, "build", 0, total);
    const cargoComponent = pluginWasmArtifactPath(repoRoot, request.cargoPackage, request.componentProfile, targetRoot);
    if (cargoComponent !== join(targetRoot, "wasm32-wasip2", request.componentProfile, request.outputName)) throw new Error("fresh component output identity differs from the shared Cargo artifact path");
    freshWasmArtifactSize(cargoComponent, FRESH_COMPONENT_MAX_BYTES, "fresh WASIp2 component");
    componentBytes = readStableBuildFile(cargoComponent, FRESH_COMPONENT_MAX_BYTES, { remaining: FRESH_COMPONENT_MAX_BYTES }, () => freshCheckpoint(control, "snapshot", 1, total));
    if (!componentBytes.byteLength) throw new Error("fresh component is empty");
    const component = join(workRoot, "component.wasm");
    freshStage(componentBytes, component, control, "snapshot", 1, total);
    const jco = resolveWorkspaceBin("@bytecodealliance/jco", repoRoot);
    if (!jco) throw new Error("missing @bytecodealliance/jco workspace binary; run bun install");
    const extractRoot = join(workRoot, "extract");
    mkdirSync(extractRoot, { mode: 0o700 });
    const baseName = request.outputName.slice(0, -".wasm".length);
    await freshRun("node", [jco, "transpile", component, "-o", extractRoot, "--name", baseName, "--map", "semio:framework/pure=./pure.js", "--map", "semio:framework/host-async=./host-async.js"], repoRoot, env, control, "extract-core", 1, total);
    const corePath = join(extractRoot, `${baseName}.core.wasm`);
    freshWasmArtifactSize(corePath, FRESH_COMPONENT_MAX_BYTES, "fresh extracted core Wasm module");
    coreBytes = readStableBuildFile(corePath, FRESH_COMPONENT_MAX_BYTES, { remaining: FRESH_COMPONENT_MAX_BYTES }, () => freshCheckpoint(control, "snapshot-core", 2, total));
    if (!coreBytes.byteLength) throw new Error("fresh core module is empty");
    const core = join(workRoot, "core.wasm");
    freshStage(coreBytes, core, control, "snapshot-core", 2, total);
    const witPath = join(workRoot, "component.wit");
    await freshRun("node", [jco, "wit", component, "--output", witPath], repoRoot, env, control, "inspect-wit", 2, total);
    const wit = readFileSync(witPath, "utf8");
    if (Buffer.byteLength(wit) > FRESH_COMPONENT_MAX_BYTES) throw new Error("fresh component WIT exceeds its fixed boundary");
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
    await freshRun(emitter, ["describe", component, "--core", core, "--out", descriptorRoot], repoRoot, env, control, "emit-descriptor", 4, total);
    const descriptorPack = join(descriptorRoot, DESCRIPTOR_PACK_FILENAME);
    const descriptorJson = join(descriptorRoot, DESCRIPTOR_JSON_FILENAME);
    snapshot = await captureFreshComponentInputs(repoRoot, request, componentBytes, coreBytes, { descriptorPack, descriptorJson }, control);
    const produced = await stageFreshComponentInputs(request, snapshot, stageRoot, witExports, control, derive);
    if (!componentInvocation || !descriptorInvocation) throw new Error("fresh component compiler observations are missing");
    return { ...produced, compilerInvocations: [componentInvocation, descriptorInvocation] };
  } finally {
    componentBytes?.fill(0);
    coreBytes?.fill(0);
    snapshot?.descriptorBytes.fill(0);
    rmSync(workRoot, { recursive: true, force: true });
  }
}

/** 🛂️ The ONE describe route of every plugin and extension component (the inferred Nx `describe` target, which
 * `dependsOn` `component-dev`): reads the exact bytes `component-dev` staged at `<crate>/dist/component-dev/<crate>.wasm`,
 * extracts its core with jco and re-emits `🛂️.descriptor.semio` + `🔣️.json` at the owner root (`<owner>/📦️packages/🦀️rust`
 * is the crate). No build happens here, so the committed descriptor, the `materialize-dev` staging (which reads the same
 * deliverable) and every consumer that resolves `dist/component-dev` describe one build by construction. */
export function describeComponentDeliverable(repoRoot: string, manifest: string, control: DescriptorEmissionControlV1 = {}): number {
  const manifestPath = resolve(repoRoot, manifest);
  try {
    const cargo = createRequire(import.meta.url)("@iarna/toml").parse(readFileSync(manifestPath, "utf8")) as { package?: { name?: string; metadata?: { component?: { package?: string }; semio?: { "component-kind"?: string } } } };
    const packageName = cargo.package?.name;
    if (!packageName || !cargo.package?.metadata?.component?.package || !["plugin", "extension"].includes(cargo.package.metadata.semio?.["component-kind"] ?? "")) throw new Error(`${manifest} has no authored plugin or extension component kind`);
    const crateRoot = dirname(manifestPath);
    const deliverableRoot = join(crateRoot, "dist");
    const component = join(deliverableRoot, "component-dev", `${packageName.replace(/-/g, "_")}.wasm`);
    if (!existsSync(component)) throw new Error(`no component-dev deliverable at ${relative(repoRoot, component)}; describe runs through Nx, which builds component-dev first`);
    const scratch = mkdtempSync(join(deliverableRoot, ".semio-describe-core-"));
    try {
      const core = extractPluginCore(repoRoot, component, scratch, packageName.replace(/-/g, "_"));
      const receipt = emitOwnerDescriptorPairV1(repoRoot, { rawComponentPath: component, extractedCorePath: core, ownerRoot: resolve(crateRoot, "..", ".."), artifactRoot: deliverableRoot }, control);
      console.log(`described ${receipt.pluginId} (${receipt.role} ${receipt.packageId}@${receipt.version}) from ${relative(repoRoot, component)} -> ${relative(repoRoot, receipt.ownerRoot)} (wasm=${receipt.rawSha256} core=${receipt.coreSha256} descriptor=${receipt.descriptorSha256})`);
      return 0;
    } finally {
      rmSync(scratch, { recursive: true, force: true });
    }
  } catch (error) {
    console.error(`describe ${manifest} failed: ${(error as Error).message}`);
    return 1;
  }
}

/** 🛂️ `describe component --manifest <Cargo.toml>`: the command the inferred Nx `describe` target of every component runs. */
export class DescribeComponentScript extends BundleScript {
  run(segments: string[]): void {
    if (segments.length !== 2 || segments[0] !== "--manifest") throw new Error("usage: component --manifest <Cargo.toml>");
    process.exit(describeComponentDeliverable(this.repoRoot, segments[1]!));
  }
}
