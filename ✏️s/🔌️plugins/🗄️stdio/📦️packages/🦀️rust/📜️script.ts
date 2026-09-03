#!/usr/bin/env bun
/** 🗄️ `@semio-tech/stdio-plugin` router: `bun ./📜️script.ts test`. */
import { spawn } from "node:child_process";
import { createHash } from "node:crypto";
import { closeSync, existsSync, fsyncSync, lstatSync, mkdirSync, mkdtempSync, openSync, readFileSync, readdirSync, readSync, realpathSync, renameSync, rmSync, statSync, truncateSync, writeFileSync, writeSync } from "node:fs";
import { tmpdir } from "node:os";
import { isAbsolute, join, relative, resolve } from "node:path";
import { isDeepStrictEqual } from "node:util";
import { decodePackValue, encodePackValue } from "../../../../../🧰️framework/🛍️products/💻️os/🟦️.ts";
import { BundleScript, ScriptRouter, buildBudgetMs, devToolingEnv, resolveTestLevel, resolveWorkspaceBin, runBundleScriptMain, runCargoTestBudgeted, runCmd } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { describePluginComponent } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️describe/📦️packages/🦀️rust/📜️script.ts";
import { CATALOG_COMMIT_MARKER_FILENAME, auditPluginCatalogSources, createFreshCatalogBuildVerifier, createFreshCatalogCommitMarker } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts";

const PACKAGE_NAME = "semio-s-plugin-stdio";
const PLUGIN_ID = "stdio";
const WASM_OUT = "semio_s_plugin_stdio.wasm";
const DESCRIPTOR_PACK = "🛂️.descriptor.semio";
const DESCRIPTOR_JSON = "🔣️.json";
const ARTIFACT_MAX_BYTES = 64 * 1024 * 1024;
const IO_CHUNK_BYTES = 64 * 1024;
const CATALOG_DEADLINE_MS = 1_200_000;
const COMPONENT_FUNCTION_MAX = 1_000_000;
const COMPONENT_PROFILE = "wasm-release";

type CatalogControl = { readonly cancelled: () => boolean; readonly remainingMs: () => number; readonly afterChunk?: (copied: number) => void };

function assertControlled(control: CatalogControl): void {
  if (control.cancelled()) throw new Error("stdio catalog-root cancelled");
  if (control.remainingMs() <= 0) throw new Error(`stdio catalog-root exceeded ${CATALOG_DEADLINE_MS}ms deadline`);
}

async function runControlled(command: string, args: string[], cwd: string, env: NodeJS.ProcessEnv, control: CatalogControl): Promise<void> {
  assertControlled(control);
  const child = spawn(command, args, { cwd, env, stdio: "inherit", windowsHide: true });
  let settled = false;
  let failure: Error | undefined;
  child.once("error", (error) => {
    failure = error;
    settled = true;
  });
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
      throw new Error(control.cancelled() ? "stdio catalog-root cancelled" : `stdio catalog-root exceeded ${CATALOG_DEADLINE_MS}ms deadline`);
    }
  }
  if (failure) throw failure;
}

function assertRegularBounded(path: string, label: string): number {
  const info = lstatSync(path);
  if (info.isSymbolicLink() || !info.isFile()) throw new Error(`${label} must be a regular non-symlink file`);
  if (info.size > ARTIFACT_MAX_BYTES) throw new Error(`${label} exceeds ${ARTIFACT_MAX_BYTES} bytes`);
  return info.size;
}

function assertContainedBounded(root: string, path: string, label: string): number {
  const size = assertRegularBounded(path, label);
  if (!pathIsWithin(realpathSync(root), realpathSync(path))) throw new Error(`${label} escapes the fresh build root`);
  return size;
}

function copyCatalogArtifact(source: string, destination: string, label: string, control: CatalogControl): string {
  const size = assertRegularBounded(source, label);
  mkdirSync(resolve(destination, ".."), { recursive: true });
  const input = openSync(source, "r");
  const output = openSync(destination, "wx");
  const chunk = Buffer.allocUnsafe(IO_CHUNK_BYTES);
  const hash = createHash("sha256");
  let copied = 0;
  try {
    while (copied < size) {
      assertControlled(control);
      const count = readSync(input, chunk, 0, Math.min(chunk.byteLength, size - copied), copied);
      if (count === 0) throw new Error(`${label} changed while copying`);
      let written = 0;
      while (written < count) written += writeSync(output, chunk, written, count - written);
      hash.update(chunk.subarray(0, count));
      copied += count;
      control.afterChunk?.(copied);
    }
    if (statSync(source).size !== size) throw new Error(`${label} changed while copying`);
    fsyncSync(output);
  } catch (error) {
    rmSync(destination, { force: true });
    throw error;
  } finally {
    closeSync(input);
    closeSync(output);
  }
  return hash.digest("hex");
}

function writeSyncedNew(path: string, bytes: Uint8Array): void {
  const descriptor = openSync(path, "wx");
  try {
    let written = 0;
    while (written < bytes.byteLength) written += writeSync(descriptor, bytes, written, bytes.byteLength - written);
    fsyncSync(descriptor);
  } finally {
    closeSync(descriptor);
  }
}

function componentPackageId(cargoManifestPath: string): string {
  const info = lstatSync(cargoManifestPath);
  if (info.isSymbolicLink() || !info.isFile() || info.size > IO_CHUNK_BYTES) throw new Error("stdio Cargo component contract must be a regular file of at most 64 KiB");
  const cargoManifest = readFileSync(cargoManifestPath, "utf8");
  let component = false;
  let componentSeen = false;
  let packageId: string | undefined;
  for (const raw of cargoManifest.split(/\r?\n/u)) {
    const line = raw.trim();
    if (line.startsWith("[")) {
      component = line === "[package.metadata.component]";
      if (component && componentSeen) throw new Error("stdio Cargo component contract repeats its component section");
      componentSeen ||= component;
      continue;
    }
    if (!component) continue;
    const separator = line.indexOf("=");
    if (separator < 0 || line.slice(0, separator).trim() !== "package") continue;
    if (packageId !== undefined) throw new Error("stdio Cargo component contract repeats its package key");
    const quoted = line.slice(separator + 1).trim().match(/^"([^"]+)"$/u);
    if (!quoted) throw new Error("stdio Cargo component package must be one quoted string");
    packageId = quoted[1];
  }
  if (!packageId || !/^semio:[a-z0-9]+(?:-[a-z0-9]+)*$/u.test(packageId)) throw new Error("stdio Cargo component package must be canonical semio:<lowercase-alnum-hyphen>");
  return packageId;
}

function publishCatalogCommitMarker(rowRoot: string, marker: unknown): void {
  const bytes = Buffer.from(`${JSON.stringify(marker)}\n`);
  if (bytes.byteLength > IO_CHUNK_BYTES) throw new Error("stdio catalog commit marker exceeds 64 KiB");
  const temporary = join(rowRoot, `.${CATALOG_COMMIT_MARKER_FILENAME}.${process.pid}.new`);
  const destination = join(rowRoot, CATALOG_COMMIT_MARKER_FILENAME);
  try {
    writeSyncedNew(temporary, bytes);
    renameSync(temporary, destination);
  } finally {
    rmSync(temporary, { force: true });
  }
}

function atomicDescriptorPair(outDir: string, pack: Uint8Array, json: Uint8Array, failBeforeJson = false): void {
  mkdirSync(outDir, { recursive: true });
  const suffix = `${process.pid}-${Date.now()}`;
  const packPath = join(outDir, DESCRIPTOR_PACK);
  const jsonPath = join(outDir, DESCRIPTOR_JSON);
  const packNew = join(outDir, `.${DESCRIPTOR_PACK}.${suffix}.new`);
  const jsonNew = join(outDir, `.${DESCRIPTOR_JSON}.${suffix}.new`);
  const packOld = join(outDir, `.${DESCRIPTOR_PACK}.${suffix}.old`);
  const jsonOld = join(outDir, `.${DESCRIPTOR_JSON}.${suffix}.old`);
  try {
    writeSyncedNew(packNew, pack);
    writeSyncedNew(jsonNew, json);
  } catch (error) {
    rmSync(packNew, { force: true });
    rmSync(jsonNew, { force: true });
    throw error;
  }
  const hadPack = existsSync(packPath);
  const hadJson = existsSync(jsonPath);
  try {
    if (hadPack) renameSync(packPath, packOld);
    if (hadJson) renameSync(jsonPath, jsonOld);
    renameSync(packNew, packPath);
    if (failBeforeJson) throw new Error("injected descriptor pair publication failure");
    renameSync(jsonNew, jsonPath);
    rmSync(packOld, { force: true });
    rmSync(jsonOld, { force: true });
  } catch (error) {
    rmSync(packPath, { force: true });
    rmSync(jsonPath, { force: true });
    if (hadPack && existsSync(packOld)) renameSync(packOld, packPath);
    if (hadJson && existsSync(jsonOld)) renameSync(jsonOld, jsonPath);
    throw error;
  } finally {
    for (const path of [packNew, jsonNew, packOld, jsonOld]) rmSync(path, { force: true });
  }
}

async function webSha256(bytes: Uint8Array): Promise<string> {
  return Buffer.from(await globalThis.crypto.subtle.digest("SHA-256", bytes)).toString("hex");
}

type WasmCoreStructure = { readonly definedFunctions: number; readonly codeBodies: number };

function readWasmU32(bytes: Uint8Array, offset: number): { readonly value: number; readonly next: number } {
  let value = 0;
  let shift = 0;
  for (let index = 0; index < 5; index += 1) {
    if (offset >= bytes.byteLength) throw new Error("truncated wasm u32");
    const byte = bytes[offset++];
    value += (byte & 0x7f) * 2 ** shift;
    if ((byte & 0x80) === 0) {
      if (value > 0xffff_ffff) throw new Error("wasm u32 overflow");
      return { value, next: offset };
    }
    shift += 7;
  }
  throw new Error("wasm u32 exceeds five bytes");
}

function inspectWasmCoreStructure(bytes: Uint8Array): WasmCoreStructure {
  if (bytes.byteLength > ARTIFACT_MAX_BYTES) throw new Error(`core module exceeds ${ARTIFACT_MAX_BYTES} bytes`);
  if (bytes.byteLength < 8 || !Buffer.from(bytes.subarray(0, 8)).equals(Buffer.from("0061736d01000000", "hex"))) throw new Error("not a version-1 core wasm module");
  let offset = 8;
  let definedFunctions: number | undefined;
  let codeBodies: number | undefined;
  while (offset < bytes.byteLength) {
    const sectionId = bytes[offset++];
    const size = readWasmU32(bytes, offset);
    const sectionStart = size.next;
    const sectionEnd = sectionStart + size.value;
    if (sectionEnd > bytes.byteLength) throw new Error("wasm section exceeds input");
    if (sectionId === 3) {
      if (definedFunctions !== undefined) throw new Error("duplicate wasm function section");
      definedFunctions = readWasmU32(bytes, sectionStart).value;
    } else if (sectionId === 10) {
      if (codeBodies !== undefined) throw new Error("duplicate wasm code section");
      codeBodies = readWasmU32(bytes, sectionStart).value;
    }
    offset = sectionEnd;
  }
  return { definedFunctions: definedFunctions ?? 0, codeBodies: codeBodies ?? 0 };
}

function assertComponentizableCore(bytes: Uint8Array): WasmCoreStructure {
  const structure = inspectWasmCoreStructure(bytes);
  if (structure.definedFunctions > COMPONENT_FUNCTION_MAX) throw new Error(`core module has ${structure.definedFunctions} defined functions; component limit is ${COMPONENT_FUNCTION_MAX}`);
  if (structure.definedFunctions !== structure.codeBodies) throw new Error("core function and code section counts disagree");
  if (!WebAssembly.validate(bytes)) throw new Error("WebAssembly parser rejected core module");
  return structure;
}

async function verifyIndependentOracles(rawPath: string, corePath: string, descriptorPackPath: string, descriptorJsonPath: string): Promise<{ rawSha256: string; coreSha256: string; descriptorSha256: string }> {
  for (const [path, label] of [[rawPath, "raw component"], [corePath, "core module"], [descriptorPackPath, "descriptor pack"], [descriptorJsonPath, "descriptor JSON"]] as const) assertRegularBounded(path, label);
  const raw = readFileSync(rawPath);
  const core = readFileSync(corePath);
  const pack = readFileSync(descriptorPackPath);
  assertComponentizableCore(core);
  const decoded = decodePackValue(pack) as Record<string, unknown>;
  if (!Buffer.from(encodePackValue(decoded)).equals(pack)) throw new Error("independent Pack oracle rejected non-canonical descriptor bytes");
  const json = JSON.parse(readFileSync(descriptorJsonPath, "utf8")) as Record<string, unknown>;
  const decodedManifest = decoded.manifest as Record<string, unknown>;
  const jsonManifest = json.manifest as Record<string, unknown>;
  if (decodedManifest?.pluginId !== PLUGIN_ID || jsonManifest?.pluginId !== PLUGIN_ID) throw new Error("stdio descriptor identity mismatch");
  const decodedHashes = decoded.hashes as Record<string, string>;
  const jsonHashes = json.hashes as Record<string, string>;
  if (!isDeepStrictEqual(decodedHashes, jsonHashes)) throw new Error("stdio descriptor JSON/Pack hash records disagree");
  const rawSha256 = await webSha256(raw);
  const coreSha256 = await webSha256(core);
  if (rawSha256 === coreSha256) throw new Error("stdio descriptor substituted raw component bytes for independently extracted core bytes");
  if (decodedHashes.wasmSha256 !== rawSha256 || decodedHashes.coreWasmSha256 !== coreSha256) throw new Error("WebCrypto raw/core hashes disagree with descriptor");
  const blanked = structuredClone(decoded);
  (blanked.hashes as Record<string, string>).descriptorSha256 = "";
  const descriptorSha256 = await webSha256(encodePackValue(blanked));
  if (decodedHashes.descriptorSha256 !== descriptorSha256) throw new Error("WebCrypto descriptor self-hash mismatch");
  return { rawSha256, coreSha256, descriptorSha256 };
}

async function runCatalogRootContractTests(root: string): Promise<void> {
  const fixtureRoot = join(root, "🧪️fixtures", "catalog-root");
  const schema = JSON.parse(readFileSync(join(fixtureRoot, "🧬️.schema.json"), "utf8"));
  const fixture = JSON.parse(readFileSync(join(fixtureRoot, "🔣️.json"), "utf8")) as {
    packageId: string;
    vectors: { raw: string; core: string; distinct: boolean }[];
    wasmStructures: { name: string; core: string; definedFunctions: number; componentizable: boolean }[];
  };
  const { default: Ajv2020 } = await import("ajv/dist/2020.js");
  const validate = new Ajv2020({ strict: true }).compile(schema);
  if (!validate(fixture)) throw new Error(`catalog-root fixture schema failed: ${JSON.stringify(validate.errors)}`);
  if (componentPackageId(join(root, "Cargo.toml")) !== fixture.packageId) throw new Error("Cargo and neutral fixture package identities disagree");
  for (const vector of fixture.vectors) {
    const raw = await webSha256(Buffer.from(vector.raw, "hex"));
    const core = await webSha256(Buffer.from(vector.core, "hex"));
    if ((raw !== core) !== vector.distinct) throw new Error("catalog-root identity vector failed");
  }
  const testBase = process.env.SEMIO_TEST_ARTIFACT_DIR ? resolve(process.env.SEMIO_TEST_ARTIFACT_DIR) : tmpdir();
  mkdirSync(testBase, { recursive: true });
  const scratch = mkdtempSync(join(testBase, "stdio-catalog-root-contract-"));
  const active: CatalogControl = { cancelled: () => false, remainingMs: () => CATALOG_DEADLINE_MS };
  try {
    const wasmOpt = resolveWorkspaceBin("wasm-opt", root);
    if (!wasmOpt) throw new Error("missing Binaryen wasm-opt workspace binary");
    for (const vector of fixture.wasmStructures) {
      const bytes = Buffer.from(vector.core, "hex");
      const structure = inspectWasmCoreStructure(bytes);
      if (structure.definedFunctions !== vector.definedFunctions) throw new Error(`${vector.name} function-section count disagrees`);
      let componentizable = true;
      try { assertComponentizableCore(bytes); } catch { componentizable = false; }
      if (componentizable !== vector.componentizable || WebAssembly.validate(bytes) !== vector.componentizable) throw new Error(`${vector.name} parser agreement failed`);
      if (vector.componentizable) {
        const input = join(scratch, `${vector.name}.wasm`);
        const output = join(scratch, `${vector.name}.optimized.wasm`);
        writeFileSync(input, bytes);
        runCmd(wasmOpt, [input, "-o", output], { cwd: root, budgetMs: 10_000 });
        if (!WebAssembly.validate(readFileSync(output))) throw new Error(`${vector.name} Binaryen oracle produced invalid wasm`);
      }
    }
    for (const [name, cargo] of [
      ["duplicate-section", '[package.metadata.component]\npackage = "semio:stdio"\n[package.metadata.component]\npackage = "semio:stdio"\n'],
      ["duplicate-key", '[package.metadata.component]\npackage = "semio:stdio"\npackage = "semio:stdio"\n'],
      ["noncanonical", '[package.metadata.component]\npackage = "semio:Stdio"\n'],
    ] as const) {
      const path = join(scratch, `${name}.toml`);
      writeFileSync(path, cargo);
      let invalid = false;
      try { componentPackageId(path); } catch { invalid = true; }
      if (!invalid) throw new Error(`${name} Cargo component identity was accepted`);
    }
    const oversized = join(scratch, "oversized.wasm");
    writeFileSync(oversized, "");
    truncateSync(oversized, ARTIFACT_MAX_BYTES + 1);
    let rejected = false;
    try { copyCatalogArtifact(oversized, join(scratch, "oversized-copy.wasm"), "fixture", active); } catch { rejected = true; }
    if (!rejected || existsSync(join(scratch, "oversized-copy.wasm"))) throw new Error("oversized artifact did not fail without publication");
    const pairRoot = join(scratch, "pair");
    atomicDescriptorPair(pairRoot, Buffer.from("old-pack"), Buffer.from("old-json"));
    rejected = false;
    try { atomicDescriptorPair(pairRoot, Buffer.from("new-pack"), Buffer.from("new-json"), true); } catch { rejected = true; }
    if (!rejected || readFileSync(join(pairRoot, DESCRIPTOR_PACK), "utf8") !== "old-pack" || readFileSync(join(pairRoot, DESCRIPTOR_JSON), "utf8") !== "old-json") throw new Error("descriptor pair rollback failed");
    const rawOnly = join(scratch, "raw-only.wasm");
    writeFileSync(rawOnly, "raw");
    rejected = false;
    try { copyCatalogArtifact(rawOnly, join(scratch, "cancelled.wasm"), "fixture", { cancelled: () => true, remainingMs: () => CATALOG_DEADLINE_MS }); } catch { rejected = true; }
    if (!rejected || existsSync(join(scratch, "cancelled.wasm"))) throw new Error("cancelled artifact copy left a publication");
    rejected = false;
    try { copyCatalogArtifact(rawOnly, join(scratch, "deadline.wasm"), "fixture", { cancelled: () => false, remainingMs: () => 0 }); } catch { rejected = true; }
    if (!rejected || existsSync(join(scratch, "deadline.wasm"))) throw new Error("expired artifact copy left a publication");
    rejected = false;
    try { assertRegularBounded(join(scratch, "missing-core.wasm"), "core module"); } catch { rejected = true; }
    if (!rejected) throw new Error("missing core input was accepted");
    const staleRoot = join(scratch, "stale-root");
    mkdirSync(staleRoot);
    writeFileSync(join(staleRoot, "stale-raw.wasm"), "stale");
    rejected = false;
    try { requireEmptyFreshRoot(root, staleRoot); } catch { rejected = true; }
    if (!rejected) throw new Error("stale raw component satisfied the fresh-root contract");
    const changing = join(scratch, "changing.wasm");
    writeFileSync(changing, Buffer.alloc(IO_CHUNK_BYTES * 2, 7));
    rejected = false;
    try {
      copyCatalogArtifact(changing, join(scratch, "changing-copy.wasm"), "fixture", {
        cancelled: () => false,
        remainingMs: () => CATALOG_DEADLINE_MS,
        afterChunk(copied) { if (copied === IO_CHUNK_BYTES) truncateSync(changing, IO_CHUNK_BYTES); },
      });
    } catch { rejected = true; }
    if (!rejected || existsSync(join(scratch, "changing-copy.wasm"))) throw new Error("changing artifact did not fail without publication");
    const packed = encodePackValue(fixture);
    if (!isDeepStrictEqual(decodePackValue(packed), fixture)) throw new Error("independent Pack fixture round-trip failed");
  } finally {
    rmSync(scratch, { recursive: true, force: true });
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runCatalogRootContractTests(this.root);
    const { rest } = resolveTestLevel(segments);
    if (rest[0] === "catalog-root-contract") return;
    await runCargoTestBudgeted([PACKAGE_NAME], this.repoRoot, rest);
  }
}

/** 📈️ Runs the owned deterministic-iteration `Brep` kernel benchmark suite (`benches/brep_kernel.rs`) — moved here
 * from `semio-framework-3d` in ticket 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-
 * ARTIFACTS wave G5, alongside the `Brep` kernel itself. */
class BenchScript extends BundleScript {
  run(): void {
    runCmd("cargo", ["bench", "-p", "semio-s-plugin-stdio"], { cwd: this.repoRoot, budgetMs: buildBudgetMs() });
  }
}

/** 🧩 Builds the root plugin's optimized WASI-P2 component without making stdio a cdylib dependency. */
class BuildWasmReleaseScript extends BundleScript {
  run(): void {
    runCmd("cargo", ["rustc", "-p", "semio-s-plugin-stdio", "--profile", COMPONENT_PROFILE, "--lib", "--crate-type", "cdylib", "--target", "wasm32-wasip2"], {
      cwd: this.repoRoot,
      budgetMs: buildBudgetMs(),
    });
  }
}

/** @emoji 🛂️ Builds this crate's `wasm32-wasip2` component and re-emits `🛂️.descriptor.semio` +
 * `🔣️.json` at this plugin's own owner root (D0-descriptor-plumbing) — the command
 * `📇️registry:check`'s own descriptor-gate warning tells a developer to run. */
class DescribeScript extends BundleScript {
  run(): void {
    process.exit(describePluginComponent(this.repoRoot, "semio-s-plugin-stdio", join(this.root, "..", ".."), true));
  }
}

type DescriptorSnapshot = { readonly pack?: Buffer; readonly json?: Buffer };

function snapshotDescriptor(ownerRoot: string): DescriptorSnapshot {
  const read = (name: string): Buffer | undefined => {
    const path = join(ownerRoot, name);
    if (!existsSync(path)) return undefined;
    assertRegularBounded(path, `owner ${name}`);
    return readFileSync(path);
  };
  return { pack: read(DESCRIPTOR_PACK), json: read(DESCRIPTOR_JSON) };
}

function restoreDescriptor(ownerRoot: string, snapshot: DescriptorSnapshot): void {
  const packPath = join(ownerRoot, DESCRIPTOR_PACK);
  const jsonPath = join(ownerRoot, DESCRIPTOR_JSON);
  rmSync(packPath, { force: true });
  rmSync(jsonPath, { force: true });
  if (snapshot.pack !== undefined && snapshot.json !== undefined) {
    atomicDescriptorPair(ownerRoot, snapshot.pack, snapshot.json);
    return;
  }
  if (snapshot.pack !== undefined) writeFileSync(packPath, snapshot.pack, { flag: "wx" });
  if (snapshot.json !== undefined) writeFileSync(jsonPath, snapshot.json, { flag: "wx" });
}

function pathIsWithin(root: string, candidate: string): boolean {
  const rel = relative(root, candidate);
  return rel === "" || (!rel.startsWith("..") && !isAbsolute(rel));
}

function requireEmptyFreshRoot(repoRoot: string, value: string): string {
  if (!isAbsolute(value)) throw new Error("catalog-root --build-root must be absolute");
  const root = resolve(value);
  const info = lstatSync(root);
  if (info.isSymbolicLink() || !info.isDirectory()) throw new Error("catalog-root build root must be a regular non-symlink directory");
  if (readdirSync(root).length !== 0) throw new Error("catalog-root build root must be empty");
  const exact = realpathSync(root);
  const ambientTarget = resolve(repoRoot, "target");
  const developmentCache = resolve(repoRoot, "🧰️framework", "🛍️products", "💻️os", "🔨️modules", "🧑️‍💻️dev", "🔌️plugin-modules");
  if (pathIsWithin(ambientTarget, exact)) throw new Error("catalog-root refuses the ambient shared target");
  if (pathIsWithin(developmentCache, exact)) throw new Error("catalog-root refuses the development cache");
  if (exact === resolve(repoRoot)) throw new Error("catalog-root requires a dedicated fresh directory");
  return exact;
}

/** 🌳 Builds and verifies the one strict stdio row from an empty caller-owned root. */
class CatalogRootScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const option = (name: string): string | undefined => {
      const index = segments.indexOf(name);
      return index < 0 ? undefined : segments[index + 1];
    };
    const suppliedRoot = option("--build-root") ?? process.env.SEMIO_CATALOG_FRESH_BUILD_ROOT;
    const cancelFile = option("--cancel-file");
    if (!suppliedRoot) throw new Error("usage: catalog-root --build-root <absolute empty fresh root> [--cancel-file <path>]");
    const buildRoot = requireEmptyFreshRoot(this.repoRoot, suppliedRoot);
    const started = Date.now();
    let interrupted = false;
    const interrupt = (): void => { interrupted = true; };
    process.on("SIGINT", interrupt);
    process.on("SIGTERM", interrupt);
    const control: CatalogControl = {
      cancelled: () => interrupted || (cancelFile !== undefined && existsSync(resolve(cancelFile))),
      remainingMs: () => Math.max(0, CATALOG_DEADLINE_MS - (Date.now() - started)),
    };
    const cargoTarget = join(buildRoot, `.stdio-cargo-target-${process.pid}`);
    const workRoot = join(buildRoot, `.stdio-work-${process.pid}`);
    const stageRoot = join(buildRoot, `.stdio-stage-${process.pid}`);
    const rowRoot = join(buildRoot, PLUGIN_ID);
    const ownerRoot = resolve(this.root, "..", "..");
    const ownerSnapshot = snapshotDescriptor(ownerRoot);
    let ownerPublished = false;
    let rowPublished = false;
    const env = devToolingEnv({ CARGO_TARGET_DIR: cargoTarget, CARGO_INCREMENTAL: "0", RUSTC_WRAPPER: "", SCCACHE_DISABLE: "1" });
    try {
      mkdirSync(workRoot, { recursive: true });
      mkdirSync(stageRoot, { recursive: true });
      const packageId = componentPackageId(join(this.root, "Cargo.toml"));
      await runControlled("cargo", ["--config", 'build.rustc-wrapper=""', "rustc", "-p", PACKAGE_NAME, "--profile", COMPONENT_PROFILE, "--lib", "--crate-type", "cdylib", "--target", "wasm32-wasip2"], this.repoRoot, env, control);
      const raw = join(cargoTarget, "wasm32-wasip2", COMPONENT_PROFILE, WASM_OUT);
      assertContainedBounded(buildRoot, raw, "raw component");
      const jco = resolveWorkspaceBin("@bytecodealliance/jco", this.repoRoot);
      if (!jco) throw new Error("missing @bytecodealliance/jco workspace binary; run bun install");
      const extractRoot = join(workRoot, "extract");
      mkdirSync(extractRoot, { recursive: true });
      await runControlled("node", [jco, "transpile", raw, "-o", extractRoot, "--name", "semio_s_plugin_stdio", "--map", "semio:framework/pure=./pure.js", "--map", "semio:framework/host-async=./host-async.js"], this.repoRoot, env, control);
      const core = join(extractRoot, "semio_s_plugin_stdio.core.wasm");
      assertContainedBounded(buildRoot, core, "jco-extracted core module");
      const witPath = join(workRoot, "stdio.wit");
      await runControlled("node", [jco, "wit", raw, "--output", witPath], this.repoRoot, env, control);
      const wit = readFileSync(witPath, "utf8");
      if (!/world\s+actor\s*\{/.test(wit) || !["reactor", "jobs", "checkpoint", "describe"].every((name) => new RegExp(`export\\s+${name}\\s*;`).test(wit))) {
        throw new Error("wasm-tools WIT oracle rejected the required actor exports");
      }
      await runControlled("cargo", ["--config", 'build.rustc-wrapper=""', "build", "-p", "semio-framework-plugin-describe"], this.repoRoot, env, control);
      const emitter = join(cargoTarget, "debug", process.platform === "win32" ? "semio-framework-plugin-describe.exe" : "semio-framework-plugin-describe");
      const descriptorRoot = join(workRoot, "descriptor");
      await runControlled(emitter, ["describe", raw, "--core", core, "--out", descriptorRoot], this.repoRoot, env, control);
      const descriptorPackPath = join(descriptorRoot, DESCRIPTOR_PACK);
      const descriptorJsonPath = join(descriptorRoot, DESCRIPTOR_JSON);
      assertContainedBounded(buildRoot, descriptorPackPath, "descriptor pack");
      assertContainedBounded(buildRoot, descriptorJsonPath, "descriptor JSON");
      const oracle = await verifyIndependentOracles(raw, core, descriptorPackPath, descriptorJsonPath);
      const stagedRaw = join(stageRoot, "raw", WASM_OUT);
      const stagedCore = join(stageRoot, "core", WASM_OUT);
      const stagedDescriptor = join(stageRoot, "descriptor", DESCRIPTOR_PACK);
      const copiedRawHash = copyCatalogArtifact(raw, stagedRaw, "raw component", control);
      const copiedCoreHash = copyCatalogArtifact(core, stagedCore, "core module", control);
      copyCatalogArtifact(descriptorPackPath, stagedDescriptor, "descriptor pack", control);
      if (copiedRawHash !== oracle.rawSha256 || copiedCoreHash !== oracle.coreSha256) throw new Error("bounded-copy hashes disagree with WebCrypto oracle");
      assertControlled(control);
      renameSync(stageRoot, rowRoot);
      rowPublished = true;
      atomicDescriptorPair(ownerRoot, readFileSync(descriptorPackPath), readFileSync(descriptorJsonPath));
      ownerPublished = true;
      await runControlled("bun", ["nx", "run", "@semio-tech/plugin-registry:generate"], this.repoRoot, devToolingEnv(), control);
      const audit = auditPluginCatalogSources(this.repoRoot, { cancelled: control.cancelled });
      const stdioIssues = audit.issues.filter((issue) => issue.pluginId === PLUGIN_ID);
      if (stdioIssues.length > 0) throw new Error(`stdio source audit failed: ${stdioIssues.map((issue) => issue.diagnostic).join("; ")}`);
      const source = audit.sources.find(({ entry }) => entry.pluginId === PLUGIN_ID);
      if (!source) throw new Error("stdio strict descriptor source was not discovered");
      if (source.entry.packageName !== PACKAGE_NAME || source.entry.wasmOut !== WASM_OUT) throw new Error("stdio source identity is not bijective with Cargo component identity");
      await runControlled("bun", ["nx", "run", "@semio-tech/plugin-registry:check-generated"], this.repoRoot, devToolingEnv(), control);
      if (source.descriptor.packageId !== packageId) throw new Error("stdio descriptor packageId does not match the Cargo component contract");
      publishCatalogCommitMarker(rowRoot, createFreshCatalogCommitMarker(source, buildRoot, { cancelled: control.cancelled }));
      const strictReceipt = createFreshCatalogBuildVerifier(this.repoRoot, buildRoot).verify(source.entry, { cancelled: control.cancelled });
      const strictHashes = { pluginId: strictReceipt.pluginId, rawSha256: strictReceipt.rawSha256, coreSha256: strictReceipt.coreSha256, descriptorSha256: strictReceipt.descriptorSha256 };
      if (!isDeepStrictEqual(strictHashes, { pluginId: PLUGIN_ID, ...oracle })) throw new Error("strict verifier and independent oracle receipts disagree");
      if (await webSha256(strictReceipt.rawBytes) !== oracle.rawSha256 || await webSha256(strictReceipt.coreBytes) !== oracle.coreSha256 || !Buffer.from(strictReceipt.descriptorBytes).equals(Buffer.from(source.packBytes))) throw new Error("strict verifier did not retain the exact admitted bytes");
      rmSync(cargoTarget, { recursive: true, force: true });
      rmSync(workRoot, { recursive: true, force: true });
      console.log(JSON.stringify({ schemaVersion: 1, packageId, wasmOut: WASM_OUT, limits: { artifactBytes: ARTIFACT_MAX_BYTES, ioChunkBytes: IO_CHUNK_BYTES, deadlineMs: CATALOG_DEADLINE_MS }, receipt: strictHashes }));
    } catch (error) {
      if (rowPublished) rmSync(rowRoot, { recursive: true, force: true });
      if (ownerPublished) {
        restoreDescriptor(ownerRoot, ownerSnapshot);
        try { runCmd("bun", ["nx", "run", "@semio-tech/plugin-registry:generate"], { cwd: this.repoRoot, env: devToolingEnv(), budgetMs: Math.max(1, control.remainingMs()) }); } catch { }
      }
      for (const path of [stageRoot, workRoot, cargoTarget]) rmSync(path, { recursive: true, force: true });
      throw error;
    } finally {
      process.off("SIGINT", interrupt);
      process.off("SIGTERM", interrupt);
    }
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("bench", BenchScript).register("build-wasm-release", BuildWasmReleaseScript).register("describe", DescribeScript).register("catalog-root", CatalogRootScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
