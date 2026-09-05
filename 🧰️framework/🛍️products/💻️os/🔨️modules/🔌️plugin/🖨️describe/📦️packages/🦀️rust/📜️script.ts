#!/usr/bin/env bun
/**
 * 🛂️ `@semio-tech/os-plugin-describe-rs` task router: `bun ./📜️script.ts <build|test|describe>`.
 * `describe <component.wasm> --core <core.wasm> --out <dir>` builds (if needed) and execs the
 * `semio-framework-plugin-describe` binary — the build-time-only descriptor emitter
 * (`📓️design-abi.md` §3). Called from the dev `📜️script.ts` right after the `wasm32-wasip2` build,
 * and from each plugin crate's own `📜️script.ts describe` (see that script's own doc for the exact
 * invocation convention every migrated plugin crate follows).
 */
import { spawn } from "node:child_process";
import { createHash } from "node:crypto";
import { closeSync, existsSync, fsyncSync, lstatSync, mkdirSync, mkdtempSync, openSync, readFileSync, readSync, realpathSync, readdirSync, renameSync, rmSync, statSync, writeSync } from "node:fs";
import { isAbsolute, join, relative, resolve } from "node:path";
import { BundleScript, ScriptRouter, buildBudgetMs, devToolingEnv, parseExtensionCargoManifest, resolveWorkspaceBin, runBundleScriptMain, runCargoTestBudgeted, runCmd, runCmdStatus, resolveTestLevel } from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
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
  cancelled(): boolean;
  remainingMs(): number;
  checkpoint(stage: string, completed: number, total: number): void;
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
  const buildArgs = rootCdylib
    ? ["rustc", "-p", packageName, "--lib", "--crate-type", "cdylib", "--target", "wasm32-wasip2", "--profile", "wasm-dev"]
    : ["build", "-p", packageName, "--target", "wasm32-wasip2", "--profile", "wasm-dev"];
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
  const child = spawn(command, args, { cwd, env, stdio: "inherit", windowsHide: true });
  let settled = false;
  let failure: Error | undefined;
  child.once("error", (error) => { failure = error; settled = true; });
  child.once("close", (code, signal) => {
    if (code !== 0) failure = new Error(`${command} ${args.join(" ")} exited with ${signal ?? code}`);
    settled = true;
  });
  while (!settled) {
    await new Promise((wake) => setTimeout(wake, Math.min(100, Math.max(1, control.remainingMs()))));
    if (control.cancelled() || control.remainingMs() <= 0) {
      child.kill("SIGTERM");
      await new Promise((wake) => setTimeout(wake, 100));
      if (!settled) child.kill("SIGKILL");
      throw new Error(control.cancelled() ? `fresh component cancelled at ${stage}` : `fresh component deadline exceeded at ${stage}`);
    }
  }
  if (failure) throw failure;
  freshCheckpoint(control, stage, completed + 1, total);
}

function freshRoot(path: string, label: string): string {
  if (!isAbsolute(path)) throw new Error(`${label} must be absolute`);
  const exact = resolve(path);
  const info = lstatSync(exact);
  if (info.isSymbolicLink() || !info.isDirectory() || readdirSync(exact).length !== 0) throw new Error(`${label} must be an empty regular directory`);
  return realpathSync(exact);
}

function freshFile(path: string, maximum: number, label: string): number {
  const info = lstatSync(path);
  if (info.isSymbolicLink() || !info.isFile() || info.size === 0 || info.size > maximum) throw new Error(`${label} is empty, non-regular, or exceeds ${maximum} bytes`);
  return info.size;
}

function freshCopy(source: string, destination: string, maximum: number, control: FreshBuildControlV1, stage: string, completed: number, total: number): { byteLength: number; sha256: string } {
  const size = freshFile(source, maximum, stage);
  const input = openSync(source, "r");
  const output = openSync(destination, "wx", 0o600);
  const chunk = Buffer.allocUnsafe(FRESH_IO_CHUNK_BYTES);
  const hash = createHash("sha256");
  let copied = 0;
  let complete = false;
  try {
    while (copied < size) {
      freshCheckpoint(control, stage, completed, total);
      const count = readSync(input, chunk, 0, Math.min(chunk.byteLength, size - copied), copied);
      if (count === 0) throw new Error(`${stage} changed during bounded copy`);
      let written = 0;
      while (written < count) written += writeSync(output, chunk, written, count - written);
      hash.update(chunk.subarray(0, count));
      copied += count;
    }
    if (statSync(source).size !== size) throw new Error(`${stage} changed during bounded copy`);
    fsyncSync(output);
    freshCheckpoint(control, stage, completed + 1, total);
    complete = true;
    return { byteLength: copied, sha256: hash.digest("hex") };
  } finally {
    chunk.fill(0);
    closeSync(output);
    closeSync(input);
    if (!complete) rmSync(destination, { force: true });
  }
}

/** 🧬️ Builds one component in a caller-owned fresh target and stages only verified immutable loader inputs. */
export async function produceFreshComponentV1(repoRoot: string, request: FreshComponentRequestV1, freshTargetRoot: string, packageStageRoot: string, control: FreshBuildControlV1): Promise<FreshComponentReceiptV1> {
  if (!/^[a-z0-9]+(?:-[a-z0-9]+)*$/u.test(request.pluginId) || !/^semio:[a-z0-9]+(?:-[a-z0-9]+)*$/u.test(request.componentPackageId) || !/^[a-z0-9]+(?:-[a-z0-9]+)*$/u.test(request.cargoPackage) || !/^[a-z0-9_]+\.wasm$/u.test(request.outputName)) throw new Error("fresh component request identity is not canonical");
  const targetRoot = freshRoot(freshTargetRoot, "fresh component target root");
  const stageRoot = freshRoot(packageStageRoot, "fresh component stage root");
  if (freshPathIsWithin(targetRoot, stageRoot) || freshPathIsWithin(stageRoot, targetRoot)) throw new Error("fresh component target and stage roots must be disjoint");
  const workRoot = join(targetRoot, ".semio-fresh-component-work");
  mkdirSync(workRoot, { mode: 0o700 });
  const env = devToolingEnv({ CARGO_TARGET_DIR: targetRoot, CARGO_INCREMENTAL: "0", RUSTC_WRAPPER: "", SCCACHE_DISABLE: "1" });
  const total = 8;
  try {
    const cargo = request.rootCdylib
      ? ["--config", 'build.rustc-wrapper=""', "rustc", "-p", request.cargoPackage, "--lib", "--crate-type", "cdylib", "--target", "wasm32-wasip2", "--profile", request.componentProfile]
      : ["--config", 'build.rustc-wrapper=""', "build", "-p", request.cargoPackage, "--target", "wasm32-wasip2", "--profile", request.componentProfile];
    await freshRun("cargo", cargo, repoRoot, env, control, "build", 0, total);
    const component = pluginWasmArtifactPath(repoRoot, request.cargoPackage, request.componentProfile, targetRoot);
    if (component !== join(targetRoot, "wasm32-wasip2", request.componentProfile, request.outputName)) throw new Error("fresh component output identity differs from the shared Cargo artifact path");
    freshFile(component, FRESH_COMPONENT_MAX_BYTES, "fresh component");
    const jco = resolveWorkspaceBin("@bytecodealliance/jco", repoRoot);
    if (!jco) throw new Error("missing @bytecodealliance/jco workspace binary; run bun install");
    const extractRoot = join(workRoot, "extract");
    mkdirSync(extractRoot, { mode: 0o700 });
    const baseName = request.outputName.slice(0, -".wasm".length);
    await freshRun("node", [jco, "transpile", component, "-o", extractRoot, "--name", baseName, "--map", "semio:framework/pure=./pure.js", "--map", "semio:framework/host-async=./host-async.js"], repoRoot, env, control, "extract-core", 1, total);
    const core = join(extractRoot, `${baseName}.core.wasm`);
    freshFile(core, FRESH_COMPONENT_MAX_BYTES, "fresh core module");
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
    freshFile(descriptorPack, FRESH_DESCRIPTOR_MAX_BYTES, "fresh descriptor pack");
    freshFile(descriptorJson, FRESH_DESCRIPTOR_MAX_BYTES, "fresh descriptor JSON");
    const descriptorJsonBytes = readFileSync(descriptorJson);
    const descriptorPackBytes = readFileSync(descriptorPack);
    const projected = JSON.parse(new TextDecoder("utf-8", { fatal: true }).decode(descriptorJsonBytes)) as Record<string, any>;
    const componentSha256 = createHash("sha256").update(readFileSync(component)).digest("hex");
    const coreSha256 = createHash("sha256").update(readFileSync(core)).digest("hex");
    if (projected.packageId !== request.componentPackageId || projected.manifest?.pluginId !== request.pluginId || typeof projected.manifest?.version !== "string" || projected.manifest.version.length === 0 || projected.role !== "plugin") throw new Error("fresh descriptor identity differs from the exact component request");
    try {
      verifyFreshCatalogPackageV1(descriptorJsonBytes, descriptorPackBytes, { pluginId: request.pluginId, packageId: request.componentPackageId, version: projected.manifest.version, role: "plugin", execution: "isolated", wasmSha256: componentSha256, coreWasmSha256: coreSha256 });
    } finally {
      descriptorJsonBytes.fill(0);
      descriptorPackBytes.fill(0);
    }
    const stagedComponent = freshCopy(component, join(stageRoot, "component.wasm"), FRESH_COMPONENT_MAX_BYTES, control, "stage-component", 5, total);
    const stagedDescriptor = freshCopy(descriptorPack, join(stageRoot, "descriptor.semio"), FRESH_DESCRIPTOR_MAX_BYTES, control, "stage-descriptor", 6, total);
    if (stagedComponent.sha256 !== componentSha256) throw new Error("fresh component changed between descriptor verification and staging");
    freshCheckpoint(control, "hash", 7, total);
    const { blake3Hex } = await import(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts"));
    const componentBlake3 = blake3Hex(readFileSync(join(stageRoot, "component.wasm")));
    freshCheckpoint(control, "complete", total, total);
    return {
      pluginId: request.pluginId,
      packageId: request.componentPackageId,
      version: projected.manifest.version,
      component: { relativePath: "component.wasm", ...stagedComponent, blake3: componentBlake3 },
      descriptor: { relativePath: "descriptor.semio", ...stagedDescriptor },
      coreSha256,
      witExports,
    };
  } catch (error) {
    for (const name of ["component.wasm", "descriptor.semio"]) rmSync(join(stageRoot, name), { force: true });
    throw error;
  } finally {
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

if (import.meta.main) {
  const router = new ScriptRouter(import.meta.dir).register("build", BuildScript).register("test", TestScript).register("describe", DescribeScript);
  await runBundleScriptMain(router, import.meta.url);
}
