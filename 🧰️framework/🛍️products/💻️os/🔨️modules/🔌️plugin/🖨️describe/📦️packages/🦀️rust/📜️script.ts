#!/usr/bin/env bun
/**
 * 🛂️ `@semio-tech/os-plugin-describe-rs` task router: `bun ./📜️script.ts <build|test|describe>`.
 * `describe <component.wasm> --core <core.wasm> --out <dir>` builds (if needed) and execs the
 * `semio-framework-plugin-describe` binary — the build-time-only descriptor emitter
 * (`📓️design-abi.md` §3). Called from the dev `📜️script.ts` right after the `wasm32-wasip2` build,
 * and from each plugin crate's own `📜️script.ts describe` (see that script's own doc for the exact
 * invocation convention every migrated plugin crate follows).
 */
import { createHash } from "node:crypto";
import { tmpdir } from "node:os";
import { closeSync, existsSync, fsyncSync, lstatSync, mkdirSync, mkdtempSync, openSync, readFileSync, readSync, realpathSync, readdirSync, renameSync, rmSync, statSync, writeFileSync, writeSync } from "node:fs";
import { isAbsolute, join, relative, resolve } from "node:path";
import {
  BundleScript,
  ScriptRouter,
  buildBudgetMs,
  devToolingEnv,
  parseExtensionCargoManifest,
  readStableBuildFile,
  resolveWorkspaceBin,
  runBundleScriptMain,
  runCargoTestBudgeted,
  runExactCargoLawProcess,
  runCmd,
  runCmdStatus,
  resolveTestLevel,
} from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { verifyDescriptorPairBytesV1, verifyFreshCatalogPackageV1 } from "../../../📇️registry/📜️script.ts";

const CRATE_NAME = "semio-framework-plugin-describe";
const DESCRIPTOR_PACK_FILENAME = "🛂️.descriptor.semio";
const DESCRIPTOR_JSON_FILENAME = "🔣️.json";
const FRESH_COMPONENT_MAX_BYTES = 64 * 1024 * 1024;
const FRESH_DESCRIPTOR_MAX_BYTES = 4 * 1024 * 1024;
const FRESH_IO_CHUNK_BYTES = 64 * 1024;

export type FreshComponentRequestV1 = Readonly<{
  pluginId: string;
  cargoPackage: string;
  componentPackageId: string;
  outputName: string;
  componentProfile: "wasm-release";
  rootCdylib: boolean;
}>;

export type FreshComponentReceiptV1 = Readonly<{
  pluginId: string;
  packageId: string;
  version: string;
  component: { readonly relativePath: "component.wasm"; readonly byteLength: number; readonly sha256: string; readonly blake3: string };
  descriptor: { readonly relativePath: "descriptor.semio"; readonly byteLength: number; readonly sha256: string };
  coreSha256: string;
  witExports: readonly string[];
}>;

export type FreshBuildControlV1 = Readonly<{
  diagnosticsRoot?: string;
  cancelled(): boolean;
  remainingMs(): number;
  checkpoint(stage: string, completed: number, total: number): void;
}>;

export type FreshComponentLeaseV1 = Readonly<{
  consume<T>(derive: (component: Uint8Array) => Promise<T>): Promise<T>;
}>;

export type FreshComponentProducedV1<T> = Readonly<{
  receipt: FreshComponentReceiptV1;
  derived: T;
}>;

class BuildScript extends BundleScript {
  run(): void {
    runCmd("cargo", ["build", "-p", CRATE_NAME, "--release"], { cwd: this.repoRoot, env: devToolingEnv() });
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestBudgeted([CRATE_NAME], this.repoRoot, rest);
  }
}

/** @emoji 🎯️ Resolves cargo's real target dir, honouring a `CARGO_TARGET_DIR` override (ticket-scoped
 * builds always set one — `important.md` binding rule 4) instead of assuming the repo-root `target/`. */
function cargoTargetRoot(repoRoot: string): string {
  return process.env.CARGO_TARGET_DIR ? resolve(repoRoot, process.env.CARGO_TARGET_DIR) : join(repoRoot, "target");
}

/** @emoji 🛠️ Resolves the debug-profile binary path for the current platform, after ensuring it is built (cargo's incremental cache makes a no-op rebuild fast — never exec a possibly-stale binary). */
function ensureBuiltBin(repoRoot: string, budgetMs = buildBudgetMs()): string {
  runCmd("cargo", ["build", "-p", CRATE_NAME], { cwd: repoRoot, env: devToolingEnv(), budgetMs });
  const binName = process.platform === "win32" ? `${CRATE_NAME}.exe` : CRATE_NAME;
  return join(cargoTargetRoot(repoRoot), "debug", binName);
}

/** @emoji 🛂️ `describe <component.wasm> --core <core.wasm> --out <dir>` — builds then execs the emitter with forwarded argv and inherited stdio. */
class DescribeScript extends BundleScript {
  run(segments: string[]): void {
    const bin = ensureBuiltBin(this.repoRoot);
    const status = runCmdStatus(bin, ["describe", ...segments], { cwd: this.repoRoot, env: devToolingEnv() });
    process.exit(status);
  }
}

/** @emoji 🎯️ WASI-development artifact path cargo just built for `packageName`, honouring
 * the same `CARGO_TARGET_DIR` override as {@link ensureBuiltBin}. */
export function pluginWasmArtifactPath(repoRoot: string, packageName: string, profile = "wasm-dev", targetRoot = cargoTargetRoot(repoRoot)): string {
  return join(targetRoot, "wasm32-wasip2", profile, `${packageName.replace(/-/g, "_")}.wasm`);
}

/** @emoji 🧩 Builds one exact plugin component and returns cargo's fresh output path. */
export function buildPluginComponent(repoRoot: string, packageName: string, rootCdylib = false, budgetMs = buildBudgetMs()): string {
  const buildArgs = rootCdylib ? ["rustc", "-p", packageName, "--lib", "--crate-type", "cdylib", "--target", "wasm32-wasip2", "--profile", "wasm-dev"] : ["build", "-p", packageName, "--target", "wasm32-wasip2", "--profile", "wasm-dev"];
  runCmd("cargo", buildArgs, { cwd: repoRoot, env: devToolingEnv(), budgetMs });
  const component = pluginWasmArtifactPath(repoRoot, packageName);
  if (!existsSync(component)) throw new Error(`cargo did not produce ${component}`);
  return component;
}

/** @emoji 🧬 Extracts the first core module from the exact component with jco's independent parser. */
export function extractPluginCore(repoRoot: string, component: string, outDir: string, baseName: string, budgetMs = buildBudgetMs()): string {
  const jco = resolveWorkspaceBin("@bytecodealliance/jco", repoRoot);
  if (!jco) throw new Error("missing @bytecodealliance/jco workspace binary; run bun install");
  runCmd("node", [jco, "transpile", component, "-o", outDir, "--name", baseName, "--map", "semio:framework/pure=./pure.js", "--map", "semio:framework/host-async=./host-async.js"], {
    cwd: repoRoot,
    env: devToolingEnv(),
    budgetMs,
  });
  const core = join(outDir, `${baseName}.core.wasm`);
  if (!existsSync(core)) throw new Error(`jco did not extract ${core}`);
  return core;
}

export type DescriptorEmissionRequestV1 = Readonly<{
  rawComponentPath: string;
  extractedCorePath: string;
  ownerRoot: string;
  artifactRoot?: string;
}>;

export type DescriptorEmissionReceiptV1 = Readonly<{
  pluginId: string;
  packageId: string;
  role: "plugin" | "extension";
  version: string;
  ownerRoot: string;
  jsonPath: string;
  packPath: string;
  rawSha256: string;
  coreSha256: string;
  descriptorSha256: string;
  jsonByteLength: number;
  packByteLength: number;
}>;

export type DescriptorEmissionControlV1 = Readonly<{
  cancelled?: () => boolean;
  deadlineMs?: number;
  checkpoint?: (stage: string) => void;
}>;

function emissionGuard(control: DescriptorEmissionControlV1, startedAt: number, budgetMs: number, stage: string): void {
  if (control.cancelled?.()) throw new Error(`descriptor emission cancelled at ${stage}`);
  if (Date.now() - startedAt > budgetMs) throw new Error(`descriptor emission deadline of ${budgetMs}ms exceeded at ${stage}`);
  control.checkpoint?.(stage);
}

function emissionDirectory(root: string, path: string, label: string): string {
  const exact = resolve(path);
  const info = lstatSync(exact);
  if (info.isSymbolicLink() || !info.isDirectory()) throw new Error(`${label} ${exact} must be a regular non-symlink directory`);
  const real = realpathSync(exact);
  if (!freshPathIsWithin(realpathSync(root), real)) throw new Error(`${label} ${exact} resolves outside ${root}`);
  return real;
}

function emissionArtifact(root: string, path: string, label: string, maximum: number): { readonly path: string; readonly sha256: string } {
  const exact = resolve(path);
  const info = lstatSync(exact);
  if (info.isSymbolicLink() || !info.isFile() || info.size === 0 || info.size > maximum) throw new Error(`${label} ${exact} must be a regular non-symlink file of 1..${maximum} bytes`);
  const real = realpathSync(exact);
  if (!freshPathIsWithin(realpathSync(root), real)) throw new Error(`${label} ${exact} resolves outside the declared artifact root ${root}`);
  const handle = openSync(real, "r");
  const chunk = Buffer.allocUnsafe(FRESH_IO_CHUNK_BYTES);
  const hash = createHash("sha256");
  try {
    let read = 0;
    while (read < info.size) {
      const count = readSync(handle, chunk, 0, Math.min(chunk.byteLength, info.size - read), read);
      if (count === 0) throw new Error(`${label} ${exact} shrank while being hashed`);
      hash.update(chunk.subarray(0, count));
      read += count;
    }
  } finally {
    chunk.fill(0);
    closeSync(handle);
  }
  if (statSync(real).size !== info.size) throw new Error(`${label} ${exact} changed while being hashed`);
  return { path: real, sha256: hash.digest("hex") };
}

/** @emoji 🛂️ The owner descriptor receipt contract: emits `🛂️.descriptor.semio` + `🔣️.json` for
 * `ownerRoot` from ONE decoded descriptor, out of two independently supplied artifacts — the raw
 * `wasm32-wasip2` component and the separately extracted core module. Both are hashed here, in this
 * process, from the exact bytes on disk; the emitter blanks exactly `hashes.descriptorSha256` for its
 * two-pass self hash; both forms are strict-decoded, compared semantically, re-encoded canonically and
 * checked against those two hashes BEFORE a single owner byte moves. Inputs must be regular
 * non-symlink files inside `artifactRoot` (cargo's target root by default, never the source tree) and
 * `ownerRoot` a regular directory inside the repository; every stage is cancellable and deadline
 * bounded, and a failure at any stage leaves the previous owner pair exactly as it was. */
export function emitOwnerDescriptorPairV1(repoRoot: string, request: DescriptorEmissionRequestV1, control: DescriptorEmissionControlV1 = {}): DescriptorEmissionReceiptV1 {
  const startedAt = Date.now();
  const budgetMs = control.deadlineMs ?? buildBudgetMs();
  emissionGuard(control, startedAt, budgetMs, "validate");
  const artifactRoot = emissionDirectory(repoRoot, request.artifactRoot ?? cargoTargetRoot(repoRoot), "artifact root");
  const ownerRoot = emissionDirectory(repoRoot, request.ownerRoot, "owner root");
  const raw = emissionArtifact(artifactRoot, request.rawComponentPath, "raw component", FRESH_COMPONENT_MAX_BYTES);
  const core = emissionArtifact(artifactRoot, request.extractedCorePath, "extracted core module", FRESH_COMPONENT_MAX_BYTES);
  if (raw.path === core.path) throw new Error("raw component and extracted core module are the same file");
  if (raw.sha256 === core.sha256) throw new Error("raw component and extracted core module have the same SHA-256");
  emissionGuard(control, startedAt, budgetMs, "emit");
  const staging = mkdtempSync(join(ownerRoot, ".🛂️descriptor-staging-"));
  try {
    const emitter = ensureBuiltBin(repoRoot, Math.max(1, budgetMs - (Date.now() - startedAt)));
    emissionGuard(control, startedAt, budgetMs, "describe");
    const status = runCmdStatus(emitter, ["describe", raw.path, "--core", core.path, "--out", staging], { cwd: repoRoot, env: devToolingEnv(), budgetMs: Math.max(1, budgetMs - (Date.now() - startedAt)) });
    if (status !== 0) throw new Error(`descriptor emitter exited with ${status}`);
    emissionGuard(control, startedAt, budgetMs, "verify");
    const packPath = join(staging, DESCRIPTOR_PACK_FILENAME);
    const jsonPath = join(staging, DESCRIPTOR_JSON_FILENAME);
    const packBytes = readFileSync(packPath);
    const jsonBytes = readFileSync(jsonPath);
    const pair = verifyDescriptorPairBytesV1(jsonBytes, packBytes, { wasmSha256: raw.sha256, coreWasmSha256: core.sha256 });
    emissionGuard(control, startedAt, budgetMs, "publish");
    renameSync(packPath, join(ownerRoot, DESCRIPTOR_PACK_FILENAME));
    renameSync(jsonPath, join(ownerRoot, DESCRIPTOR_JSON_FILENAME));
    return {
      pluginId: pair.pluginId,
      packageId: pair.packageId,
      role: pair.role,
      version: pair.version,
      ownerRoot,
      jsonPath: join(ownerRoot, DESCRIPTOR_JSON_FILENAME),
      packPath: join(ownerRoot, DESCRIPTOR_PACK_FILENAME),
      rawSha256: raw.sha256,
      coreSha256: core.sha256,
      descriptorSha256: pair.hashes.descriptorSha256,
      jsonByteLength: jsonBytes.byteLength,
      packByteLength: packBytes.byteLength,
    };
  } finally {
    rmSync(staging, { recursive: true, force: true });
  }
}

function freshPathIsWithin(root: string, candidate: string): boolean {
  const path = relative(root, candidate);
  return path === "" || (!path.startsWith("..") && !isAbsolute(path));
}

function freshCheckpoint(control: FreshBuildControlV1, stage: string, completed: number, total: number): void {
  if (control.cancelled()) throw new Error(`fresh component cancelled at ${stage}`);
  if (control.remainingMs() <= 0) throw new Error(`fresh component deadline exceeded at ${stage}`);
  control.checkpoint(stage, completed, total);
}

async function freshRun(command: string, args: string[], cwd: string, env: NodeJS.ProcessEnv, control: FreshBuildControlV1, stage: string, completed: number, total: number): Promise<void> {
  freshCheckpoint(control, stage, completed, total);
  const budgetMs = control.remainingMs();
  if (!Number.isSafeInteger(budgetMs) || budgetMs <= 0 || budgetMs > 86_400_000) throw new Error("fresh process budget must be 1..86400000ms");
  const retained = control.diagnosticsRoot !== undefined;
  const root = control.diagnosticsRoot ?? tmpdir();
  if (!isAbsolute(root) || (retained && !root.split(/[\\/]/u).includes("🗑️generated"))) throw new Error("fresh process evidence root must be an absolute ticket-generated directory");
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

function freshStage(bytes: Uint8Array, destination: string, control: FreshBuildControlV1, stage: string, completed: number, total: number): { byteLength: number; sha256: string } {
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
async function captureFreshComponentInputs(
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
async function stageFreshComponentInputs<T>(
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
  const env = devToolingEnv({ CARGO_TARGET_DIR: targetRoot, CARGO_INCREMENTAL: "0", RUSTC_WRAPPER: "", SCCACHE_DISABLE: "1" });
  const total = 8;
  let componentBytes: Uint8Array | undefined, coreBytes: Uint8Array | undefined, snapshot: Awaited<ReturnType<typeof captureFreshComponentInputs>> | undefined;
  try {
    const cargo = request.rootCdylib
      ? ["--config", 'build.rustc-wrapper=""', "rustc", "-p", request.cargoPackage, "--lib", "--crate-type", "cdylib", "--target", "wasm32-wasip2", "--profile", request.componentProfile]
      : ["--config", 'build.rustc-wrapper=""', "build", "-p", request.cargoPackage, "--target", "wasm32-wasip2", "--profile", request.componentProfile];
    await freshRun("cargo", cargo, repoRoot, env, control, "build", 0, total);
    const cargoComponent = pluginWasmArtifactPath(repoRoot, request.cargoPackage, request.componentProfile, targetRoot);
    if (cargoComponent !== join(targetRoot, "wasm32-wasip2", request.componentProfile, request.outputName)) throw new Error("fresh component output identity differs from the shared Cargo artifact path");
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
    coreBytes = readStableBuildFile(join(extractRoot, `${baseName}.core.wasm`), FRESH_COMPONENT_MAX_BYTES, { remaining: FRESH_COMPONENT_MAX_BYTES }, () => freshCheckpoint(control, "snapshot-core", 2, total));
    if (!coreBytes.byteLength) throw new Error("fresh core module is empty");
    const core = join(workRoot, "core.wasm");
    freshStage(coreBytes, core, control, "snapshot-core", 2, total);
    const witPath = join(workRoot, "component.wit");
    await freshRun("node", [jco, "wit", component, "--output", witPath], repoRoot, env, control, "inspect-wit", 2, total);
    const wit = readFileSync(witPath, "utf8");
    if (Buffer.byteLength(wit) > FRESH_COMPONENT_MAX_BYTES) throw new Error("fresh component WIT exceeds its fixed boundary");
    const witExports = [...wit.matchAll(/\bexport\s+([a-z][a-z0-9-]*)\s*;/gu)].map((match) => match[1]!).sort();
    if (!["checkpoint", "describe", "jobs", "reactor"].every((name) => witExports.includes(name))) throw new Error("fresh component omits a required actor export");
    await freshRun("cargo", ["--config", 'build.rustc-wrapper=""', "build", "-p", CRATE_NAME], repoRoot, env, control, "build-descriptor-emitter", 3, total);
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

/** 🧪️ Qualifies retained verified inputs and staging independently of Cargo or descriptor execution. */
export async function testFreshComponentStagingV1(repoRoot: string): Promise<void> {
  const { default: assert } = await import("node:assert/strict");
  const { default: Ajv2020 } = await import("ajv/dist/2020.js");
  const { encodePackValue } = await import("../../../../../🟦️.ts");
  const fixtureRoot = resolve(import.meta.dir, "../../🧪️fixtures/🧊️fresh-staging");
  const fixture = JSON.parse(readFileSync(join(fixtureRoot, "🔣️.json"), "utf8"));
  const validate = new Ajv2020({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(fixtureRoot, "🧬️.schema.json"), "utf8")));
  assert(validate(fixture), JSON.stringify(validate.errors));
  const artifactRoot = process.env.SEMIO_TEST_ARTIFACT_DIR;
  assert(artifactRoot?.includes("🗑️generated"));
  mkdirSync(artifactRoot, { recursive: true });
  const evidence = mkdtempSync(join(artifactRoot, "fresh-component-staging-"));
  const component = Buffer.from(fixture.componentHex, "hex"),
    core = Buffer.from(fixture.coreHex, "hex");
  const descriptor = {
    descriptorVersion: 1,
    packageId: "semio:gis",
    role: "plugin",
    manifest: { pluginId: "gis", label: "GIS", version: "0.1.0", apps: [], examples: [], capabilities: [], topicContributions: [], commands: [], artifactKinds: [], dependencies: [], contributions: [] },
    activationEvents: [],
    capabilityRequests: [],
    extensionPoints: [],
    execution: "isolated",
    quotas: {},
    contributions: {},
    assets: [],
    hashes: { wasmSha256: fixture.componentSha256, coreWasmSha256: createHash("sha256").update(core).digest("hex"), descriptorSha256: "" },
  };
  descriptor.hashes.descriptorSha256 = createHash("sha256").update(encodePackValue(descriptor)).digest("hex");
  const descriptorBytes = encodePackValue(descriptor);
  const paths = { component: join(evidence, "component.wasm"), core: join(evidence, "core.wasm"), descriptorPack: join(evidence, "descriptor.semio"), descriptorJson: join(evidence, "descriptor.json") };
  for (const [key, bytes] of [
    ["component", component],
    ["core", core],
    ["descriptorPack", descriptorBytes],
    ["descriptorJson", Buffer.from(JSON.stringify(descriptor))],
  ] as const)
    writeFileSync(paths[key], bytes);
  const control: FreshBuildControlV1 = { cancelled: () => false, remainingMs: () => 60_000, checkpoint() {} };
  const capturedComponent = readStableBuildFile(paths.component, FRESH_COMPONENT_MAX_BYTES, { remaining: FRESH_COMPONENT_MAX_BYTES }, () => {});
  const capturedCore = readStableBuildFile(paths.core, FRESH_COMPONENT_MAX_BYTES, { remaining: FRESH_COMPONENT_MAX_BYTES }, () => {});
  writeFileSync(paths.core, "replaced-core");
  const snapshot = await captureFreshComponentInputs(repoRoot, { pluginId: "gis", componentPackageId: "semio:gis" }, capturedComponent, capturedCore, paths, control);
  assert.equal(snapshot.coreSha256, descriptor.hashes.coreWasmSha256);
  assert.equal(snapshot.componentSha256, fixture.componentSha256);
  assert.equal(snapshot.componentBlake3, fixture.componentBlake3);
  assert.equal(snapshot.componentSha256, Buffer.from(await crypto.subtle.digest("SHA-256", component)).toString("hex"));
  assert.deepEqual(Buffer.from(snapshot.descriptorBytes), Buffer.from(descriptorBytes));
  for (const path of Object.values(paths)) {
    renameSync(path, path + ".retained");
    writeFileSync(path, "replaced-source");
  }
  const destination = join(evidence, "staged.semio");
  const staged = freshStage(snapshot.descriptorBytes, destination, control, "stage-descriptor", 6, 8);
  assert.equal(staged.sha256, Buffer.from(await crypto.subtle.digest("SHA-256", descriptorBytes)).toString("hex"));
  assert.deepEqual(readFileSync(destination), Buffer.from(descriptorBytes));
  renameSync(destination, destination + ".retained");
  writeFileSync(destination, "replaced-stage");
  assert.deepEqual(Buffer.from(snapshot.descriptorBytes), Buffer.from(descriptorBytes));
  assert.equal(createHash("sha256").update(snapshot.componentBytes).digest("hex"), fixture.componentSha256);
  const cancelled = join(evidence, "cancelled.semio");
  let checkpoints = 0;
  assert.throws(
    () =>
      freshStage(
        snapshot.descriptorBytes,
        cancelled,
        {
          ...control,
          cancelled: () => checkpoints >= 2,
          checkpoint() {
            checkpoints++;
          },
        },
        "stage-descriptor",
        6,
        8,
      ),
    /cancelled/,
  );
  assert.equal(checkpoints, 2);
  assert.equal(existsSync(cancelled), false);
  writeFileSync(paths.component, component);
  writeFileSync(paths.core, core);
  writeFileSync(paths.descriptorPack, descriptorBytes);
  writeFileSync(paths.descriptorJson, JSON.stringify({ ...descriptor, packageId: "semio:foreign" }));
  await assert.rejects(captureFreshComponentInputs(repoRoot, { pluginId: "gis", componentPackageId: "semio:gis" }, capturedComponent, capturedCore, paths, control), /identity/);
  const cloneSnapshot = () => ({ ...snapshot, componentBytes: Uint8Array.from(component), descriptorBytes: Uint8Array.from(descriptorBytes) });
  const handoff = async <T>(name: string, input: ReturnType<typeof cloneSnapshot>, buildControl: FreshBuildControlV1, derive: (lease: FreshComponentLeaseV1) => Promise<T>) => {
    const stage = join(evidence, name);
    mkdirSync(stage);
    return await stageFreshComponentInputs({ pluginId: "gis", componentPackageId: "semio:gis" }, input, stage, ["checkpoint", "describe", "jobs", "reactor"], buildControl, derive);
  };
  let retainedLease: FreshComponentLeaseV1 | undefined, retainedLoan: Uint8Array | undefined;
  const source = cloneSnapshot();
  const produced = await handoff("loan-success", source, control, async (lease) => {
    retainedLease = lease;
    assert.deepEqual(Object.keys(lease), ["consume"]);
    assert(Object.isFrozen(lease));
    const digest = await lease.consume(async (bytes) => {
      retainedLoan = bytes;
      assert.notEqual(bytes.buffer, source.componentBytes.buffer);
      const sha256 = Buffer.from(await crypto.subtle.digest("SHA-256", bytes)).toString("hex");
      bytes.fill(9);
      assert.deepEqual(Buffer.from(source.componentBytes), component);
      return sha256;
    });
    await assert.rejects(
      lease.consume(async () => "second"),
      /already consumed/,
    );
    return digest;
  });
  assert.equal(produced.derived, produced.receipt.component.sha256);
  assert.equal(produced.receipt.component.sha256, fixture.componentSha256);
  assert.deepEqual(readFileSync(join(evidence, "loan-success/component.wasm")), component);
  assert.deepEqual(Object.keys(produced.receipt).sort(), ["component", "coreSha256", "descriptor", "packageId", "pluginId", "version", "witExports"]);
  assert(retainedLoan!.every((byte) => byte === 0));
  assert(source.componentBytes.every((byte) => byte === 0));
  assert(source.descriptorBytes.every((byte) => byte === 0));
  await assert.rejects(
    retainedLease!.consume(async () => "late"),
    /expired/,
  );
  const rejectedSource = cloneSnapshot();
  let rejectedLoan: Uint8Array | undefined;
  await assert.rejects(
    handoff("loan-rejected", rejectedSource, control, (lease) =>
      lease.consume(async (bytes) => {
        rejectedLoan = bytes;
        throw new Error("derive sentinel");
      }),
    ),
    /^Error: derive sentinel$/,
  );
  assert(rejectedLoan!.every((byte) => byte === 0));
  assert(rejectedSource.componentBytes.every((byte) => byte === 0));
  assert(rejectedSource.descriptorBytes.every((byte) => byte === 0));
  assert.deepEqual(readdirSync(join(evidence, "loan-rejected")), []);
  for (const when of ["before-consume", "after-consume"] as const) {
    let stop = false,
      invoked = false;
    const cancelledSource = cloneSnapshot();
    await assert.rejects(
      handoff(`loan-cancel-${when}`, cancelledSource, { ...control, cancelled: () => stop }, async (lease) => {
        if (when === "before-consume") stop = true;
        return await lease.consume(async () => {
          invoked = true;
          stop = true;
          return "must not publish";
        });
      }),
      /cancelled/,
    );
    assert.equal(invoked, when === "after-consume");
    assert(cancelledSource.componentBytes.every((byte) => byte === 0));
    assert(cancelledSource.descriptorBytes.every((byte) => byte === 0));
    assert.deepEqual(readdirSync(join(evidence, `loan-cancel-${when}`)), []);
  }
  for (const failure of [false, true]) {
    const unawaitedSource = cloneSnapshot();
    let release!: () => void,
      entered!: () => void,
      settled = false,
      loan: Uint8Array | undefined;
    const gate = new Promise<void>((resolve) => {
      release = resolve;
    });
    const running = new Promise<void>((resolve) => {
      entered = resolve;
    });
    const operation = handoff(`loan-unawaited-${failure}`, unawaitedSource, control, async (lease) => {
      void lease.consume(async (bytes) => {
        loan = bytes;
        entered();
        await gate;
        if (failure) throw new Error("unawaited sentinel");
        return "done";
      });
      return "callback finished";
    });
    void operation.then(
      () => {
        settled = true;
      },
      () => {
        settled = true;
      },
    );
    await running;
    await Promise.resolve();
    assert.equal(settled, false);
    assert.deepEqual(Buffer.from(loan!), component);
    release();
    if (failure) await assert.rejects(operation, /^Error: unawaited sentinel$/);
    else assert.equal((await operation).derived, "callback finished");
    assert(loan!.every((byte) => byte === 0));
    assert(unawaitedSource.componentBytes.every((byte) => byte === 0));
    if (failure) assert.deepEqual(readdirSync(join(evidence, `loan-unawaited-${failure}`)), []);
  }
  snapshot.componentBytes.fill(0);
  capturedCore.fill(0);
  snapshot.descriptorBytes.fill(0);
  console.log(`fresh-component-staging: AJV=1 WebCrypto=1 Pack=1 BLAKE3=1 laws=${fixture.laws.length} evidence=${evidence}`);
}

/** 🧪️ Qualifies fresh producer diagnostics and bounded real process retirement without Cargo. */
export async function testFreshComponentProcessV1(repoRoot: string): Promise<void> {
  const { default: assert } = await import("node:assert/strict");
  const { default: Ajv2020 } = await import("ajv/dist/2020.js");
  const { default: deepEqual } = await import("fast-deep-equal");
  const fixtureRoot = resolve(import.meta.dir, "../../🧪️fixtures/🧵️fresh-process");
  const fixture = JSON.parse(readFileSync(join(fixtureRoot, "🔣️.json"), "utf8"));
  const validate = new Ajv2020({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(fixtureRoot, "🧬️.schema.json"), "utf8")));
  assert(validate(fixture), JSON.stringify(validate.errors));
  const artifactRoot = process.env.SEMIO_TEST_ARTIFACT_DIR;
  assert(artifactRoot && isAbsolute(artifactRoot) && artifactRoot.split(/[\\/]/u).includes("🗑️generated"));
  const evidence = mkdtempSync(join(artifactRoot, "fresh-process-laws-"));
  for (const row of fixture.cases) {
    const root = join(evidence, row.name);
    mkdirSync(root);
    const started = Date.now(),
      checkpoints: number[] = [];
    const deadline = started + (row.mode === "timeout" ? 150 : 10_000);
    const control: FreshBuildControlV1 = {
      diagnosticsRoot: root,
      cancelled: () => row.mode === "pre-cancel" || (row.mode === "cancel" && Date.now() - started >= 150),
      remainingMs: () => deadline - Date.now(),
      checkpoint: (_stage, completed) => {
        checkpoints.push(completed);
      },
    };
    const script =
      row.mode === "exit"
        ? "process.stdout.write(" + JSON.stringify(row.stdout) + "); process.stderr.write(" + JSON.stringify(row.stderr) + "); process.exitCode=" + row.exitCode
        : row.mode === "flood"
          ? "const {writeSync}=require('node:fs'); const b=Buffer.alloc(65536,120); for(let n=0;n<=1024;n++) writeSync(1,b); setInterval(()=>{},1000)"
          : "setInterval(()=>{},1000)";
    let failure: Error | undefined;
    try {
      await freshRun(
        row.mode === "missing" ? join(root, "absent-executable") : process.execPath,
        ["-e", script],
        repoRoot,
        { ...process.env, SEMIO_TEST_ARTIFACT_DIR: join(root, "must-not-use-ambient-evidence"), CARGO_TARGET_DIR: root },
        control,
        row.name,
        0,
        1,
      );
    } catch (error) {
      failure = error as Error;
    }
    assert(Date.now() - started < 6000, row.name + " bounded retirement");
    if (row.mode === "pre-cancel") {
      assert(failure?.message.includes(row.diagnostic));
      assert.deepEqual(readdirSync(root), []);
      continue;
    }
    const traces = readdirSync(root);
    assert.equal(traces.length, 1, row.name + " retained process trace");
    const trace = join(root, traces[0]!);
    const outcome = JSON.parse(readFileSync(join(trace, "outcome.json"), "utf8"));
    assert.equal(outcome.reason, row.reason);
    assert.equal(outcome.stage, row.name);
    assert.equal(outcome.cargoTargetDir, root);
    const stdout = readFileSync(join(trace, "stdout.jsonl"), "utf8"),
      stderr = readFileSync(join(trace, "stderr.txt"), "utf8");
    assert(Buffer.byteLength(stdout) + Buffer.byteLength(stderr) <= fixture.maxOutputBytes);
    if (row.mode === "exit") {
      assert(deepEqual({ stdout, stderr, status: outcome.status }, { stdout: row.stdout, stderr: row.stderr, status: row.exitCode }), row.name + " independent transcript oracle");
      assert.equal(outcome.signal, null);
    }
    if (row.name === "success") {
      assert.equal(failure, undefined);
      assert.deepEqual(checkpoints, [0, 1]);
    } else {
      assert(failure?.message.includes(row.diagnostic), row.name + " surfaced cause: " + failure?.message);
      assert(failure.message.includes(trace), row.name + " exact trace location");
      assert(failure.message.length <= fixture.diagnosticChars + trace.length + 500);
      assert.deepEqual(checkpoints, [0]);
    }
  }
  console.log("fresh-component-process: AJV=1 fast-deep-equal=3 runtime-laws=" + fixture.cases.length + " evidence=" + evidence);
}

if (import.meta.main) {
  const router = new ScriptRouter(import.meta.dir).register("build", BuildScript).register("test", TestScript).register("describe", DescribeScript);
  await runBundleScriptMain(router, import.meta.url);
}
