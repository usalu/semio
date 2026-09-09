#!/usr/bin/env bun
import { createFreshComponentTests } from "../../🧪️tests/🆕️fresh-component/🟦️.ts";
import { buildCargoArtifacts } from "../../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📜️script.ts";
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
import { semanticOwnedInputFileSnapshot } from "../../../../../../🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";

const CRATE_NAME = "semio-framework-plugin-describe";
const DESCRIPTOR_PACK_FILENAME = "🛂️.descriptor.semio";
const DESCRIPTOR_JSON_FILENAME = "🔣️.json";
/** 🧱️ Admission bound for the two build artifacts the descriptor emitter reads — the raw
 * `wasm32-wasip2` component and jco's extracted core. {@link buildPluginComponent} builds the
 * UNOPTIMIZED `wasm-dev` profile, so this bounds a build-time input and is deliberately NOT the
 * runtime bound on a shipped component (`DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES`, which
 * applies to the optimized artifact). The procedural plugin's dev component is ~80 MB. */
const FRESH_COMPONENT_MAX_BYTES = 128 * 1024 * 1024;
const FRESH_DESCRIPTOR_MAX_BYTES = 4 * 1024 * 1024;
const FRESH_IO_CHUNK_BYTES = 64 * 1024;

function freshWasmArtifactSize(path: string, maximum: number, label: string): number {
  const info = lstatSync(path);
  if (!info.isFile() || info.isSymbolicLink() || !Number.isSafeInteger(info.size) || info.size < 1 || info.size > maximum)
    throw new Error(`${label} file bound: byteLength=${info.size} maximum=${maximum}`);
  return info.size;
}

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

export type FreshSourceEpochLegV1 = Readonly<{ id: string; package: string; args: readonly string[] }>;
export type FreshSourceEpochPlanV1 = Readonly<{
  toolchain: Readonly<{ cargo: string; rustc: string }>;
  environment: readonly (readonly [string, string | null])[];
  legs: readonly FreshSourceEpochLegV1[];
  files: readonly string[];
}>;
export type FreshSourceEpochFileV1 = Readonly<{ path: string; sha256: string; byteLength: number; mode: number }>;
export type FreshSourceEpochRecordV1 = Readonly<{
  schema: "semio.plugin.fresh-source-epoch/v1";
  toolchain: FreshSourceEpochPlanV1["toolchain"];
  environment: FreshSourceEpochPlanV1["environment"];
  legs: readonly FreshSourceEpochLegV1[];
  files: readonly FreshSourceEpochFileV1[];
}>;
export type FreshSourceEpochOwnerV1 = Readonly<{
  digest: string;
  record: FreshSourceEpochRecordV1;
  start(leg: FreshSourceEpochLegV1, environment: FreshSourceEpochPlanV1["environment"]): void;
  complete(id: string, compilerInputs: readonly string[]): void;
  finish(): string;
  abort(): void;
}>;

const FRESH_SOURCE_EPOCH_LIMITS = Object.freeze({ fileBytes: 4 * 1024 * 1024, totalBytes: 128 * 1024 * 1024, files: 32768, legs: 16 });

function freshSourceOrderedJson(value: unknown): string {
  if (value === null || typeof value === "string" || typeof value === "boolean" || (typeof value === "number" && Number.isFinite(value))) return JSON.stringify(value);
  if (Array.isArray(value)) return "[" + value.map(freshSourceOrderedJson).join(",") + "]";
  if (value && typeof value === "object") return "{" + Object.keys(value).sort().map((key) => JSON.stringify(key) + ":" + freshSourceOrderedJson((value as Record<string, unknown>)[key])).join(",") + "}";
  throw new Error("fresh source epoch contains a non-JSON value");
}

type FreshRustDepInfoV1 = Readonly<{ targets: readonly string[]; inputs: readonly string[]; environment: readonly (readonly [string, string | null])[] }>;

/** 📃️ Reads the Rust dep-info writer's closed rule and environment grammar without evaluating Make syntax. */
function parseFreshRustDepInfoV1(bytes: Uint8Array, check: () => void): FreshRustDepInfoV1 {
  check();
  if (!bytes.byteLength || bytes.byteLength > 8 * 1024 * 1024) throw new Error("fresh Rust dep-info byte boundary");
  const text = new TextDecoder("utf-8", { fatal: true }).decode(bytes).replaceAll("\r\n", "\n");
  if (!text.endsWith("\n") || text.includes("\0")) throw new Error("fresh Rust dep-info is truncated or contains NUL");
  const targets: string[] = [], fake = new Set<string>(), environment: (readonly [string, string | null])[] = [], names = new Set<string>();
  let inputs: string[] | undefined, inputSet = new Set<string>(), phase: "outputs" | "inputs" | "environment" = "outputs";
  const path = (value: string): string => {
    if (!value || Buffer.byteLength(value) > 4096 || /[\x00-\x1f\x7f]/u.test(value)) throw new Error("fresh Rust dep-info path boundary");
    return value;
  };
  const paths = (value: string): string[] => {
    const result: string[] = [];
    let token = "";
    for (let index = 0; index < value.length; index++) {
      if ((index & 65535) === 0) check();
      const character = value[index]!;
      if (character === "\\" && value[index + 1] === " ") { token += " "; index++; }
      else if (character === " ") { result.push(path(token)); token = ""; }
      else token += character;
      if (token.length > 4096 || result.length > FRESH_SOURCE_EPOCH_LIMITS.files) throw new Error("fresh Rust dep-info token boundary");
    }
    result.push(path(token));
    if (result.length > FRESH_SOURCE_EPOCH_LIMITS.files || new Set(result).size !== result.length) throw new Error("fresh Rust dep-info input count or duplicate");
    return result;
  };
  const unescapeEnvironment = (value: string): string => {
    if (Buffer.byteLength(value) > 32768) throw new Error("fresh Rust dep-info environment boundary");
    let output = "";
    for (let index = 0; index < value.length; index++) {
      const character = value[index]!;
      if (character !== "\\") { output += character; continue; }
      const escaped = value[++index];
      if (escaped === "n") output += "\n";
      else if (escaped === "r") output += "\r";
      else if (escaped === "\\") output += "\\";
      else throw new Error("fresh Rust dep-info environment escape is unknown");
    }
    if (Buffer.byteLength(output) > 16384) throw new Error("fresh Rust dep-info environment boundary");
    return output;
  };
  const lines = text.split("\n");
  if (lines.length > 4 * FRESH_SOURCE_EPOCH_LIMITS.files + 1024) throw new Error("fresh Rust dep-info line boundary");
  for (const line of lines) {
    check();
    if (!line) continue;
    if (line.startsWith("# env-dep:")) {
      if (!inputs || fake.size !== inputs.length) throw new Error("fresh Rust dep-info environment precedes complete input rules");
      phase = "environment";
      const row = line.slice(10), equal = row.indexOf("="), name = equal < 0 ? row : row.slice(0, equal);
      if (!/^[A-Za-z_][A-Za-z0-9_]{0,255}$/u.test(name) || names.has(name) || environment.length >= 256) throw new Error("fresh Rust dep-info environment name or count");
      names.add(name);
      environment.push(Object.freeze([name, equal < 0 ? null : unescapeEnvironment(row.slice(equal + 1))] as const));
      continue;
    }
    if (line.startsWith("#") || phase === "environment") throw new Error("fresh Rust dep-info contains an unknown or out-of-order record");
    const separator = line.indexOf(": ");
    if (separator >= 0) {
      if (phase !== "outputs" || targets.length >= 128) throw new Error("fresh Rust dep-info output order or count");
      const target = path(line.slice(0, separator)), selected = paths(line.slice(separator + 2));
      if (targets.includes(target) || (inputs && freshSourceOrderedJson(inputs) !== freshSourceOrderedJson(selected))) throw new Error("fresh Rust dep-info output dependency lists disagree");
      targets.push(target);
      inputs = selected;
      inputSet = new Set(selected);
    } else {
      if (!line.endsWith(":") || !inputs) throw new Error("fresh Rust dep-info rule is unknown");
      phase = "inputs";
      const selected = paths(line.slice(0, -1));
      if (selected.length !== 1 || !inputSet.has(selected[0]!) || fake.has(selected[0]!)) throw new Error("fresh Rust dep-info fake input rule disagrees");
      fake.add(selected[0]!);
    }
  }
  if (!targets.length || !inputs?.length || fake.size !== inputs.length) throw new Error("fresh Rust dep-info input closure is incomplete");
  return Object.freeze({ targets: Object.freeze(targets), inputs: Object.freeze(inputs), environment: Object.freeze(environment) });
}

function freshSourceCoordinate(path: string): string {
  if (typeof path !== "string" || Buffer.byteLength(path) > 4096 || /[\\\\\x00-\x1f\x7f:*?"<>|]/u.test(path) || Buffer.from(path).toString("utf8") !== path || path.split("/").some((part) => !part || part === "." || part === "..")) throw new Error("fresh source epoch coordinate is unsafe");
  if (path.split("/").some((part) => [".git", "node_modules", "target", "🗑️generated"].includes(part))) throw new Error("fresh source epoch refuses compiler, dependency, or ticket output inputs");
  return path;
}

function freshSourceEpochEnvironment(value: FreshSourceEpochPlanV1["environment"]): FreshSourceEpochPlanV1["environment"] {
  if (!Array.isArray(value) || value.length > 256) throw new Error("fresh source epoch environment exceeds its boundary");
  const names = new Set<string>();
  return Object.freeze(value.map((row) => {
    if (!Array.isArray(row) || row.length !== 2 || !/^[A-Za-z_][A-Za-z0-9_]{0,255}$/u.test(row[0]) || names.has(row[0]) || (row[1] !== null && (typeof row[1] !== "string" || Buffer.byteLength(row[1]) > 16384 || row[1].includes("\0")))) throw new Error("fresh source epoch environment is not a closed unique projection");
    names.add(row[0]);
    return Object.freeze([row[0], row[1]] as const);
  }).sort((left, right) => Buffer.from(left[0]).compare(Buffer.from(right[0]))));
}

function freshSourceEpochLeg(leg: FreshSourceEpochLegV1): FreshSourceEpochLegV1 {
  if (!leg || Object.keys(leg).sort().join(",") !== "args,id,package" || !/^[a-z][a-z0-9-]*$/u.test(leg.id) || !/^[a-z][a-z0-9-]*$/u.test(leg.package) || !Array.isArray(leg.args) || !leg.args.length || leg.args.length > 128 || leg.args.some((arg) => typeof arg !== "string" || Buffer.byteLength(arg) > 4096 || arg.includes("\0"))) throw new Error("fresh source epoch leg is not a bounded exact command");
  return Object.freeze({ id: leg.id, package: leg.package, args: Object.freeze([...leg.args]) });
}

/** 🧾️ Encodes a bounded source epoch independently of object insertion order. */
export function freshSourceEpochBytesV1(record: FreshSourceEpochRecordV1): Uint8Array {
  const pieces: Buffer[] = [];
  const field = (value: string): void => {
    const bytes = Buffer.from(value), length = Buffer.alloc(4);
    length.writeUInt32BE(bytes.byteLength);
    pieces.push(length, bytes);
  };
  field(record.schema);
  field(freshSourceOrderedJson(record.toolchain));
  field(freshSourceOrderedJson(record.environment));
  field(freshSourceOrderedJson(record.legs));
  field(freshSourceOrderedJson(record.files));
  return Buffer.concat(pieces);
}

/** 🧊️ Retains one no-follow source epoch across exact ordered compiler legs; it does not freeze collaborators' files. */
export function captureFreshSourceEpochV1(repoRoot: string, plan: FreshSourceEpochPlanV1, control: FreshBuildControlV1): FreshSourceEpochOwnerV1 {
  const check = (stage: string, index = 0, total = plan.files.length): void => freshCheckpoint(control, `source-epoch-${stage}`, index, total);
  check("capture");
  if (!plan.toolchain || Object.keys(plan.toolchain).sort().join(",") !== "cargo,rustc" || Object.values(plan.toolchain).some((value) => typeof value !== "string" || !value.length || Buffer.byteLength(value) > 8192 || value.includes("\0"))) throw new Error("fresh source epoch toolchain identity is invalid");
  if (!Array.isArray(plan.legs) || !plan.legs.length || plan.legs.length > FRESH_SOURCE_EPOCH_LIMITS.legs || !Array.isArray(plan.files) || !plan.files.length || plan.files.length > FRESH_SOURCE_EPOCH_LIMITS.files) throw new Error("fresh source epoch count exceeds its boundary");
  const environment = freshSourceEpochEnvironment(plan.environment), legs = plan.legs.map(freshSourceEpochLeg);
  if (new Set(legs.map((leg) => leg.id)).size !== legs.length) throw new Error("fresh source epoch repeats a compiler leg");
  const paths = plan.files.map(freshSourceCoordinate).sort((left, right) => Buffer.from(left).compare(Buffer.from(right)));
  if (new Set(paths).size !== paths.length) throw new Error("fresh source epoch repeats an input");
  const physical = new Map<string, string>();
  let totalBytes = 0;
  const identity = (path: string): string => {
    const node = lstatSync(join(repoRoot, path));
    if (!node.isFile() || node.isSymbolicLink() || node.size > FRESH_SOURCE_EPOCH_LIMITS.fileBytes) throw new Error("fresh source epoch input is non-regular or oversized: " + path);
    return [node.dev, node.ino, node.mode, node.size, node.mtimeMs, node.ctimeMs].join(":");
  };
  const read = (path: string): FreshSourceEpochFileV1 => {
    const before = identity(path), input = semanticOwnedInputFileSnapshot(repoRoot, path, { maximumBytes: FRESH_SOURCE_EPOCH_LIMITS.fileBytes, checkpoint: () => check("read") });
    if (!input) throw new Error("fresh source epoch input is missing: " + path);
    try {
      if (input.size > FRESH_SOURCE_EPOCH_LIMITS.fileBytes || before !== identity(path)) throw new Error("fresh source epoch input changed during capture: " + path);
      if (physical.has(path) && physical.get(path) !== before) throw new Error("fresh source epoch input identity changed: " + path);
      physical.set(path, before);
      return Object.freeze({ path, sha256: input.contentHash, byteLength: input.size, mode: input.mode });
    } finally { input.bytes.fill(0); }
  };
  const files = paths.map((path, index) => {
    check("capture", index);
    const input = read(path);
    totalBytes += input.byteLength;
    if (totalBytes > FRESH_SOURCE_EPOCH_LIMITS.totalBytes) throw new Error("fresh source epoch aggregate exceeds its byte boundary");
    return input;
  });
  const record: FreshSourceEpochRecordV1 = Object.freeze({ schema: "semio.plugin.fresh-source-epoch/v1", toolchain: Object.freeze({ ...plan.toolchain }), environment, legs: Object.freeze(legs), files: Object.freeze(files) });
  const encoded = freshSourceEpochBytesV1(record), digest = createHash("sha256").update(encoded).digest("hex");
  encoded.fill(0);
  const known = new Set(paths);
  let next = 0, phase: "ready" | "running" | "closed" = "ready";
  const verify = (): void => {
    for (const [index, expected] of files.entries()) {
      check("verify", index);
      if (freshSourceOrderedJson(read(expected.path)) !== freshSourceOrderedJson(expected)) throw new Error("fresh source epoch input bytes changed: " + expected.path);
    }
    check("verified", files.length);
  };
  const transition = <T>(operation: () => T): T => {
    try {
      if (phase === "closed") throw new Error("fresh source epoch owner is closed");
      check("transition");
      return operation();
    } catch (error) { phase = "closed"; throw error; }
  };
  return Object.freeze({
    digest, record,
    start(leg, actualEnvironment) {
      transition(() => {
        if (phase !== "ready" || next >= legs.length || freshSourceOrderedJson(freshSourceEpochLeg(leg)) !== freshSourceOrderedJson(legs[next])) throw new Error("fresh source epoch compiler leg is out of order or changed");
        if (freshSourceOrderedJson(freshSourceEpochEnvironment(actualEnvironment)) !== freshSourceOrderedJson(environment)) throw new Error("fresh source epoch compiler environment changed");
        verify();
        phase = "running";
      });
    },
    complete(id, compilerInputs) {
      transition(() => {
        if (phase !== "running" || legs[next]?.id !== id) throw new Error("fresh source epoch completion has no exact running leg");
        if (!Array.isArray(compilerInputs) || !compilerInputs.length || compilerInputs.length > FRESH_SOURCE_EPOCH_LIMITS.files || new Set(compilerInputs).size !== compilerInputs.length) throw new Error("fresh source epoch compiler input report is empty, duplicated, or oversized");
        for (const [index, input] of compilerInputs.entries()) {
          check("compiler-input", index, compilerInputs.length);
          if (!known.has(freshSourceCoordinate(input))) throw new Error("fresh source epoch has an unknown compiler input: " + input);
        }
        verify();
        next++;
        phase = "ready";
      });
    },
    finish() {
      return transition(() => {
        if (phase !== "ready" || next !== legs.length) throw new Error("fresh source epoch cannot finish incomplete compiler legs");
        verify();
        phase = "closed";
        return digest;
      });
    },
    abort() { phase = "closed"; },
  });
}

class BuildScript extends BundleScript {
  async run(): Promise<void> {
    await buildCargoArtifacts(join(this.root, "Cargo.toml"), ["--release", "--bin", CRATE_NAME], this.repoRoot);
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

/** 🛂️ Runs the restored descriptor emitter with forwarded argv and inherited stdio. */
class DescribeScript extends BundleScript {
  run(segments: string[]): void {
    const bin = join(this.root, "dist", "build", process.platform === "win32" ? `${CRATE_NAME}.exe` : CRATE_NAME);
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

/** ⏱️ Carries the descriptor emission budget into each child process. */
export function remainingDescriptorEmissionBudgetMs(budgetMs: number, elapsedMs: number): number {
  return budgetMs === 0 ? 0 : Math.max(1, budgetMs - elapsedMs);
}

function emissionGuard(control: DescriptorEmissionControlV1, startedAt: number, budgetMs: number, stage: string): void {
  if (control.cancelled?.()) throw new Error(`descriptor emission cancelled at ${stage}`);
  if (budgetMs !== 0 && Date.now() - startedAt > budgetMs) throw new Error(`descriptor emission deadline of ${budgetMs}ms exceeded at ${stage}`);
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
    const emitter = ensureBuiltBin(repoRoot, remainingDescriptorEmissionBudgetMs(budgetMs, Date.now() - startedAt));
    emissionGuard(control, startedAt, budgetMs, "describe");
    const status = runCmdStatus(emitter, ["describe", raw.path, "--core", core.path, "--out", staging], { cwd: repoRoot, env: devToolingEnv(), budgetMs: remainingDescriptorEmissionBudgetMs(budgetMs, Date.now() - startedAt) });
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
const createFreshComponentTestsInstance = createFreshComponentTests({ captureFreshComponentInputs, captureFreshSourceEpochV1, closeSync, createHash, existsSync, FRESH_COMPONENT_MAX_BYTES, FRESH_IO_CHUNK_BYTES, FRESH_SOURCE_EPOCH_LIMITS, freshRun, freshSourceEpochBytesV1, freshSourceOrderedJson, freshStage, freshWasmArtifactSize, isAbsolute, join, mkdirSync, mkdtempSync, openSync, parseFreshRustDepInfoV1, readdirSync, readFileSync, readStableBuildFile, renameSync, resolve, rmSync, semanticOwnedInputFileSnapshot, stageFreshComponentInputs, writeFileSync }, { directory: import.meta.dir, url: import.meta.url });
export const testFreshComponentSourceEpochV1 = createFreshComponentTestsInstance.testFreshComponentSourceEpochV1;
export const testFreshComponentStagingV1 = createFreshComponentTestsInstance.testFreshComponentStagingV1;
export const testFreshComponentProcessV1 = createFreshComponentTestsInstance.testFreshComponentProcessV1;



if (import.meta.main) {
  const router = new ScriptRouter(import.meta.dir).register("build", BuildScript).register("test", TestScript).register("describe", DescribeScript);
  await runBundleScriptMain(router, import.meta.url);
}
