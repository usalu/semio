import { createHash } from "node:crypto";
import { tmpdir } from "node:os";
import { closeSync, existsSync, fsyncSync, lstatSync, mkdirSync, mkdtempSync, openSync, readFileSync, realpathSync, readdirSync, renameSync, rmSync, writeFileSync, writeSync } from "node:fs";
import { isAbsolute, join, relative, resolve } from "node:path";
import { isGeneratedPath } from "../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/🟦️.ts";
import { devToolingEnv, parseExtensionCargoManifest, readStableBuildFile, resolveWorkspaceBin, runExactCargoLawProcess } from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { semanticOwnedInputFileSnapshot } from "../../../../../🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";
import { createFreshComponentTests } from "../🧪️tests/🆕️fresh-component/🟦️.ts";
import { verifyFreshCatalogPackageV1 } from "../../📇️registry/✅️catalog-verification/🟦️.ts";
import { DESCRIPTOR_JSON_FILENAME, DESCRIPTOR_PACK_FILENAME, FRESH_COMPONENT_MAX_BYTES, FRESH_DESCRIPTOR_MAX_BYTES, FRESH_IO_CHUNK_BYTES, CRATE_NAME, buildPluginComponent, cargoTargetRoot, extractPluginCore, freshWasmArtifactSize, pluginWasmArtifactPath } from "../🏗️component-build/🟦️.ts";
import { emitOwnerDescriptorPairV1, type DescriptorEmissionControlV1 } from "../🛂️descriptor-emission/🟦️.ts";
import { FRESH_SOURCE_EPOCH_LIMITS, captureFreshSourceEpochV1, freshCheckpoint, freshPathIsWithin, freshSourceEpochBytesV1, freshSourceOrderedJson, parseFreshRustDepInfoV1, type FreshBuildControlV1, type FreshComponentLeaseV1, type FreshComponentProducedV1, type FreshComponentReceiptV1, type FreshComponentRequestV1 } from "../🧾️source-epoch/🟦️.ts";

export async function freshRun(command: string, args: string[], cwd: string, env: NodeJS.ProcessEnv, control: FreshBuildControlV1, stage: string, completed: number, total: number): Promise<void> {
  freshCheckpoint(control, stage, completed, total);
  const budgetMs = control.remainingMs();
  if (!Number.isSafeInteger(budgetMs) || budgetMs <= 0 || budgetMs > 86_400_000) throw new Error("fresh process budget must be 1..86400000ms");
  const retained = control.diagnosticsRoot !== undefined;
  const root = control.diagnosticsRoot ?? tmpdir();
  if (!isAbsolute(root) || (retained && !isGeneratedPath(root))) throw new Error("fresh process evidence root must be an absolute directory inside a generated directory");
  const info = lstatSync(root);
  if (!info.isDirectory() || info.isSymbolicLink()) throw new Error("fresh process evidence root must be a regular directory");
  if (command === "cargo" && args.some((arg) => arg === "--message-format" || arg.startsWith("--message-format="))) throw new Error("fresh Cargo diagnostics format is producer-owned");
  const argv = command === "cargo" ? [...args, "--message-format=json"] : args;
  const trace = mkdtempSync(join(root, "fresh-process-"));
  const evidence = retained ? trace : "ephemeral";
  if (retained) console.log("fresh-component-process: stage=" + stage + " evidence=" + evidence);
  try {
    const result = await runExactCargoLawProcess(command, argv, {
      cwd,
      env,
      budgetMs,
      maxOutputBytes: 64 * 1024 * 1024,
      stdoutPath: join(trace, "stdout.jsonl"),
      stderrPath: join(trace, "stderr.txt"),
      cancelled: () => control.cancelled(),
    });
    const reason = result.reason ?? "exit";
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
  } finally {
    if (!retained) rmSync(trace, { recursive: true, force: true });
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
    const { blake3Hex } = await import(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts"));
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
    consume<R>(consumer: (bytes: Uint8Array) => Promise<R>): Promise<R> {
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
    await freshRun("cargo", cargo, repoRoot, env, control, "build", 0, total);
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
    const witExports = [...wit.matchAll(/\bexport\s+([a-z][a-z0-9-]*)\s*;/gu)].map((match) => match[1]!).sort();
    if (!["checkpoint", "describe", "jobs", "reactor"].every((name) => witExports.includes(name))) throw new Error("fresh component omits a required actor export");
    await freshRun("cargo", ["build", "-p", CRATE_NAME], repoRoot, env, control, "build-descriptor-emitter", 3, total);
    const emitter = join(targetRoot, "debug", process.platform === "win32" ? `${CRATE_NAME}.exe` : CRATE_NAME);
    const descriptorRoot = join(workRoot, "descriptor");
    mkdirSync(descriptorRoot, { mode: 0o700 });
    await freshRun(emitter, ["describe", component, "--core", core, "--out", descriptorRoot], repoRoot, env, control, "emit-descriptor", 4, total);
    const descriptorPack = join(descriptorRoot, DESCRIPTOR_PACK_FILENAME);
    const descriptorJson = join(descriptorRoot, DESCRIPTOR_JSON_FILENAME);
    snapshot = await captureFreshComponentInputs(repoRoot, request, componentBytes, coreBytes, { descriptorPack, descriptorJson }, control);
    return await stageFreshComponentInputs(request, snapshot, stageRoot, witExports, control, derive);
  } finally {
    componentBytes?.fill(0);
    coreBytes?.fill(0);
    snapshot?.descriptorBytes.fill(0);
    rmSync(workRoot, { recursive: true, force: true });
  }
}

/** @emoji 🛂️ Shared implementation for a plugin/extension crate's own `📜️script.ts describe` command
 * (D0-descriptor-plumbing, `📌️important.md`): builds `packageName`'s `wasm32-wasip2` component — no
 * extra `--features component-guest` flag needed, every plugin crate's own `Cargo.toml` already
 * enables it unconditionally on its `semio-framework-plugin` dependency, confirmed empirically (no
 * plugin crate exposes a feature literally named `component-guest` of its own; passing that flag to
 * `cargo build -p <plugin>` fails with "does not contain this feature") — then runs the real emitter
 * (`describe_component`, `🖨️describe/📦️packages/🦀️rust/🦀️.rs`) against the built wasm, writing
 * `🛂️.descriptor.semio` + `🔣️.json` straight into `ownerRoot` (the plugin/extension owner
 * root, sibling of the tracked `🛂️manifest.json` — NOT `🤖️generated/`, which is gitignored). One
 * shared function so every migrated plugin crate's own `describe` command stays a thin two-line
 * wrapper around it rather than duplicating the build+emit sequence 33 times. */
export function describePluginComponent(repoRoot: string, packageName: string, ownerRoot: string, rootCdylib = false, control: DescriptorEmissionControlV1 = {}): number {
  const artifactRoot = cargoTargetRoot(repoRoot);
  const component = buildPluginComponent(repoRoot, packageName, rootCdylib);
  const scratch = mkdtempSync(join(artifactRoot, ".semio-describe-core-"));
  try {
    const core = extractPluginCore(repoRoot, component, scratch, packageName.replace(/-/g, "_"));
    const receipt = emitOwnerDescriptorPairV1(repoRoot, { rawComponentPath: component, extractedCorePath: core, ownerRoot, artifactRoot }, control);
    console.log(`described ${receipt.pluginId} (${receipt.role} ${receipt.packageId}@${receipt.version}) -> ${relative(repoRoot, receipt.ownerRoot)} (wasm=${receipt.rawSha256} core=${receipt.coreSha256} descriptor=${receipt.descriptorSha256})`);
    return 0;
  } catch (error) {
    console.error(`describe ${packageName} failed: ${(error as Error).message}`);
    return 1;
  } finally {
    rmSync(scratch, { recursive: true, force: true });
  }
}

/** @emoji 🧩 The shared extension `describe` route: the same owner receipt contract as a plugin's, with
 * the crate identity read from the extension's own `[package]`/`[package.metadata.component]` block and
 * the owner root taken as the extension root (the `.sxt` runtime package that `package` builds is a
 * separate artifact and never a descriptor source). */
export function describeExtensionComponent(repoRoot: string, rsDir: string, control: DescriptorEmissionControlV1 = {}): number {
  const manifest = parseExtensionCargoManifest(join(resolve(rsDir), "Cargo.toml"), repoRoot);
  return describePluginComponent(repoRoot, manifest.packageName, resolve(rsDir, "..", ".."), false, control);
}
const createFreshComponentTestsInstance = createFreshComponentTests({ captureFreshComponentInputs, captureFreshSourceEpochV1, closeSync, createHash, existsSync, FRESH_COMPONENT_MAX_BYTES, FRESH_IO_CHUNK_BYTES, FRESH_SOURCE_EPOCH_LIMITS, freshRun, freshSourceEpochBytesV1, freshSourceOrderedJson, freshStage, freshWasmArtifactSize, isAbsolute, join, mkdirSync, mkdtempSync, openSync, parseFreshRustDepInfoV1, readdirSync, readFileSync, readStableBuildFile, renameSync, resolve, rmSync, semanticOwnedInputFileSnapshot, stageFreshComponentInputs, writeFileSync }, { directory: resolve(import.meta.dir, "../📦️packages/🦀️rust"), url: import.meta.url });
export const testFreshComponentSourceEpochV1 = createFreshComponentTestsInstance.testFreshComponentSourceEpochV1;
export const testFreshComponentStagingV1 = createFreshComponentTestsInstance.testFreshComponentStagingV1;
export const testFreshComponentProcessV1 = createFreshComponentTestsInstance.testFreshComponentProcessV1;
