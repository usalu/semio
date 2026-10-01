#!/usr/bin/env bun
/** 🗄️ `@semio-tech/stdio-plugin` router: `bun ./📜️script.ts test`. */
import { spawn } from "node:child_process";
import { createHash } from "node:crypto";
import { closeSync, existsSync, fsyncSync, lstatSync, mkdirSync, mkdtempSync, openSync, readFileSync, readdirSync, readSync, realpathSync, renameSync, rmSync, statSync, truncateSync, writeFileSync, writeSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, isAbsolute, join, relative, resolve, sep } from "node:path";
import { isDeepStrictEqual } from "node:util";
import { decodePackValue, encodePackValue } from "../../../../../🧰️framework/🛍️products/💻️os/🟦️.ts";
import { BundleScript, ScriptRouter, buildBudgetMs, devToolingEnv, resolveTestLevel, resolveWorkspaceBin, runBundleScriptMain, runCargoTestBudgeted, runCmd, runExactCargoLaws, runTestBudgeted } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { prepareCargoWorkspaceInvocation } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🟦️.ts";
import { acquireCargoBuildLeaseV1 } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🏗️native-build/🔒️lease/🟦️.ts";
import { repoCacheDirectory } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🟦️.ts";
import { cargoTargetDirectory, cargoBuildDirectory } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🟦️.ts";
import { pluginModulesRootIn } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🟦️.ts";
import { terminateOwnedChildTree } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🟦️.ts";
import { prepareStdioComposition } from "../../🧩️composition/🟦️.ts";

const PACKAGE_NAME = "semio-hub-stdio";
const PLUGIN_ID = "stdio";
const ARTIFACT_OWNER = "✏️s/🔌️plugins/🗄️stdio";
const WASM_OUT = "semio_hub_stdio.wasm";
const DESCRIPTOR_PACK = "🛂️.descriptor.semio";
const DESCRIPTOR_JSON = "🔣️.json";
const ARTIFACT_MAX_BYTES = 64 * 1024 * 1024;
const IO_CHUNK_BYTES = 64 * 1024;
const CATALOG_DEADLINE_MS = 1_200_000;
const COMPONENT_FUNCTION_MAX = 1_000_000;
const COMPONENT_PROFILE = "wasm-release";

/** 🧬️ Compiles one PascalCase `$defs` export of the Stdio plugin-root draft-07 schema module.
 * @see 🌎️hub/🧩️compositions/🗄️stdio/🧬️schema/🔣️.json */
async function compileStdioScopeExport(stdioRoot: string, exportId: string) {
  const module = JSON.parse(readFileSync(join(stdioRoot, "🧬️schema", "🔣️.json"), "utf8"));
  const { default: Ajv } = await import("ajv");
  const ajv = new Ajv({ strict: true });
  ajv.addKeyword({ keyword: "x-semio-formats", metaSchema: { type: "array", items: { type: "string" } } });
  ajv.addSchema(module);
  return ajv.compile({ $ref: `${module.$id}#/$defs/${exportId}` });
}

type CatalogControl = { readonly cancelled: () => boolean; readonly remainingMs: () => number; readonly afterChunk?: (copied: number) => void };

function assertControlled(control: CatalogControl): void {
  if (control.cancelled()) throw new Error("stdio catalog-root cancelled");
  if (control.remainingMs() <= 0) throw new Error("stdio catalog-root exceeded its configured deadline");
}

async function runControlled(command: string, args: string[], cwd: string, env: NodeJS.ProcessEnv, control: CatalogControl): Promise<void> {
  assertControlled(control);
  if (command === "cargo") prepareCargoWorkspaceInvocation(cwd, args, cwd);
  const child = spawn(command, args, { cwd, env, stdio: "inherit", windowsHide: true });
  const drained = new Promise<void>((resolve) => child.once("close", () => resolve()));
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
      terminateOwnedChildTree(child);
      await drained;
      throw new Error(control.cancelled() ? "stdio catalog-root cancelled" : "stdio catalog-root exceeded its configured deadline");
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

function publishCatalogCommitMarker(rowRoot: string, marker: unknown, markerFilename: string): void {
  const bytes = Buffer.from(`${JSON.stringify(marker)}\n`);
  if (bytes.byteLength > IO_CHUNK_BYTES) throw new Error("stdio catalog commit marker exceeds 64 KiB");
  const temporary = join(rowRoot, `.${markerFilename}.${process.pid}.new`);
  const destination = join(rowRoot, markerFilename);
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

async function runCatalogRootContractTests(root: string, repoRoot: string): Promise<void> {
  const fixtureRoot = join(root, "../..", "🧫️fixtures", "🌳️catalog-root");
  const fixture = JSON.parse(readFileSync(join(fixtureRoot, "🔣️.json"), "utf8")) as {
    packageId: string;
    vectors: { raw: string; core: string; distinct: boolean }[];
    wasmStructures: { name: string; core: string; definedFunctions: number; componentizable: boolean }[];
    compilerCaches: { id: string; ambient: NodeJS.ProcessEnv; target: string; build: string }[];
  };
  const validate = await compileStdioScopeExport(join(root, "../.."), "StdioCatalogRoot");
  if (!validate(fixture)) throw new Error(`catalog-root fixture schema failed: ${JSON.stringify(validate.errors)}`);
  const { default: Ajv } = await import("ajv");
  const catalogSchema = JSON.parse(readFileSync(join(root, "../../📇️catalog/🧬️schema/🔣️.json"), "utf8"));
  const validateCommitment = new Ajv({ strict: false }).compile({ ...catalogSchema, $ref: "#/$defs/NativeCatalogSurfaceCommitment" });
  if (!validateCommitment({ schema: "semio.stdio.artifact-catalog/v1", pluginId: "stdio", packageId: "semio:stdio", packageVersion: "0.1.0", definitions: [], codecs: [] })) throw new Error("authored empty catalog commitment is invalid");
  if (componentPackageId(join(root, "Cargo.toml")) !== fixture.packageId) throw new Error("Cargo and neutral fixture package identities disagree");
  for (const vector of fixture.vectors) {
    const raw = await webSha256(Buffer.from(vector.raw, "hex"));
    const core = await webSha256(Buffer.from(vector.core, "hex"));
    if ((raw !== core) !== vector.distinct) throw new Error("catalog-root identity vector failed");
  }
  const testBase = process.env.SEMIO_TEST_ARTIFACT_DIR ? resolve(repoRoot, process.env.SEMIO_TEST_ARTIFACT_DIR) : tmpdir();
  mkdirSync(testBase, { recursive: true });
  const scratch = mkdtempSync(join(testBase, "stdio-catalog-root-contract-"));
  const active: CatalogControl = { cancelled: () => false, remainingMs: () => CATALOG_DEADLINE_MS };
  try {
    mkdirSync(join(scratch, ".cargo"));
    const config = '[build]\ntarget-dir = "shared-target"\nbuild-dir = "shared-build"\n';
    writeFileSync(join(scratch, ".cargo/config.toml"), config);
    const { parse: independentToml } = await import("@iarna/toml");
    if (!isDeepStrictEqual(Bun.TOML.parse(config), independentToml(config))) throw new Error("compiler cache config oracle differs");
    for (const row of fixture.compilerCaches) {
      const env = catalogCargoEnvironment(join(scratch, row.target), row.ambient);
      if (cargoTargetDirectory(scratch, env) !== join(scratch, row.target) || cargoBuildDirectory(scratch, env) !== join(scratch, row.build)) throw new Error(`${row.id}: staged compiler cache escapes its private target`);
    }
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

class FlowRetainedDecodeScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { default: assert } = await import("node:assert/strict");
    const { default: Ajv } = await import("ajv");
    const leb = await import("@webassemblyjs/leb128");
    const base = join(this.repoRoot, ARTIFACT_OWNER, "🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🌊️flow/🧬️schema/📸️snapshot/💾️binary");
    const fixture = JSON.parse(readFileSync(join(base, "🧫️fixtures/🔣️.json"), "utf8"));
    const schema = JSON.parse(readFileSync(join(base, "🧬️schema/🔣️.json"), "utf8"));
    const ajv = new Ajv({ strict: true });
    ajv.addKeyword({ keyword: "x-semio-formats", metaSchema: { type: "array", items: { type: "string" } } });
    const validate = ajv.compile(schema.$defs.FlowRetainedSnapshot);
    assert(validate(fixture), ajv.errorsText(validate.errors));
    const header = Buffer.concat([Buffer.from([137,83,69,77,13,10,26,10,24,0,0,0]), Buffer.from("stdio.semio.flow.pack v1")]);
    const decode = (hex: string): unknown => {
      const bytes = Buffer.from(hex, "hex"); let offset = 0; let strings = 0;
      const fail = (reason: string): never => { throw new Error(reason); };
      const byte = (): number => offset < bytes.length ? bytes[offset++]! : fail("malformed");
      const uint = (): number => {
        const start = offset; let value = 0n;
        for (let index = 0; index < 10; index++) {
          const next = byte(); if (index === 9 && next > 1) fail("malformed");
          value |= BigInt(next & 127) << BigInt(index * 7);
          if (next < 128) {
            if (index && next === 0) fail("malformed");
            if (value <= 0xffffffffn) assert.deepEqual(bytes.subarray(start, offset), Buffer.from(leb.encodeU32(Number(value))));
            return value > BigInt(Number.MAX_SAFE_INTEGER) ? Number.MAX_SAFE_INTEGER : Number(value);
          }
        }
        return fail("malformed");
      };
      const text = (): string => {
        const size = uint(); if (size > fixture.limits.stringBytes || strings + size > fixture.limits.totalStringBytes) fail("capacity");
        strings += size; if (offset + size > bytes.length) fail("malformed");
        let value: string; try { value = new TextDecoder("utf-8", { fatal: true }).decode(bytes.subarray(offset, offset + size)); } catch { return fail("malformed"); }
        offset += size; return value;
      };
      const count = (maximum: number): number => { const value = uint(); return value <= maximum ? value : fail("capacity"); };
      const number = (): number => { if (offset + 8 > bytes.length) fail("malformed"); const value = bytes.readDoubleLE(offset); offset += 8; return value; };
      for (const expected of header) if (byte() !== expected) fail("malformed");
      if (byte() !== 1) fail("malformed");
      const identity = text(); if (identity !== "stdio.semio.flow") fail("identity");
      const nodes = []; const nodeCount = count(fixture.limits.nodes);
      for (let index = 0; index < nodeCount; index++) {
        const id = text(), kind = text(), label = text(), params = []; const parameterCount = count(fixture.limits.paramsPerNode);
        for (let parameter = 0; parameter < parameterCount; parameter++) params.push({ key: text(), value: text() });
        nodes.push({ id, kind, label, params, position: { x: number(), y: number() } });
      }
      const edges = []; const edgeCount = count(fixture.limits.edges);
      for (let index = 0; index < edgeCount; index++) edges.push({ id: text(), from: { node: text(), port: text() }, to: { node: text(), port: text() }, kind: text() });
      if (offset !== bytes.length) fail("malformed");
      return { schema: identity, nodes, edges };
    };
    const encode = (snapshot: any): Buffer => {
      const chunks = [header, Buffer.from([1])];
      const count = (value: number): void => { chunks.push(Buffer.from(leb.encodeU32(value))); };
      const text = (value: string): void => { const bytes = Buffer.from(value); count(bytes.length); chunks.push(bytes); };
      text(snapshot.schema); count(snapshot.nodes.length);
      for (const node of snapshot.nodes) {
        for (const value of [node.id, node.kind, node.label]) text(value);
        count(node.params.length); for (const parameter of node.params) { text(parameter.key); text(parameter.value); }
        const point = new ArrayBuffer(16), view = new DataView(point); view.setFloat64(0, node.position.x, true); view.setFloat64(8, node.position.y, true); chunks.push(Buffer.from(point));
      }
      count(snapshot.edges.length); for (const edge of snapshot.edges) for (const value of [edge.id, edge.from.node, edge.from.port, edge.to.node, edge.to.port, edge.kind]) text(value);
      return Buffer.concat(chunks);
    };
    for (const row of fixture.valid) { assert.deepEqual(decode(row.hex), row.snapshot, row.id); assert.equal(encode(row.snapshot).toString("hex"), row.hex, row.id); }
    for (const row of fixture.invalid) assert.throws(() => decode(row.hex), (error: Error) => error.message === row.reason, row.id);
    console.log(`Flow retained decoder independent oracle: ${fixture.valid.length} exact wire snapshots, ${fixture.invalid.length} hostile denials; third-party LEB128 and AJV agree`);
    const lifecycle = JSON.parse(readFileSync(join(base, "🧫️fixtures/♻️lifecycle/🔣️.json"), "utf8"));
    const validateLifecycle = ajv.compile(schema.$defs.FlowRetainedSnapshotLifecycle);
    assert(validateLifecycle(lifecycle), ajv.errorsText(validateLifecycle.errors));
    assert.equal(new Set(lifecycle.admission.map((row: any) => row.id)).size, 5);
    for (const row of lifecycle.admission) {
      const reason = row.state === "closing" || row.state === "retired" ? "stale" : row.state === "unadmitted" ? "unsealed" : row.subset !== "flow" ? "identity" : null;
      assert.equal(reason, row.reason, row.id);
    }
    const large = structuredClone(fixture.valid.find((row: any) => row.id === lifecycle.multiPage.source).snapshot);
    large.nodes[lifecycle.multiPage.nodeIndex].label = lifecycle.multiPage.labelScalar.repeat(lifecycle.multiPage.labelRepeats);
    const largeWire = encode(large);
    assert.deepEqual(decode(largeWire.toString("hex")), large);
    const stringBytes = (value: any): number => typeof value === "string" ? Buffer.byteLength(value) : value && typeof value === "object" ? Object.values(value).reduce<number>((sum, field) => sum + stringBytes(field), 0) : 0;
    const identityBytes = [lifecycle.request.artifactId, lifecycle.request.artifactKind, lifecycle.request.standard, lifecycle.request.subset].reduce((sum: number, value: string) => sum + Buffer.byteLength(value), 0);
    const inputBytes = Buffer.from(leb.encodeU32(largeWire.length)).length + largeWire.length + 1;
    const typedBytes = stringBytes(large) + large.nodes.length * 16;
    assert.equal(largeWire.length, lifecycle.multiPage.wireBytes);
    assert.equal(inputBytes, lifecycle.multiPage.inputBytes);
    assert.equal(Math.ceil(inputBytes / 4096), lifecycle.multiPage.inputPages);
    assert.equal(identityBytes, lifecycle.request.identityBytes);
    assert.equal(typedBytes, lifecycle.multiPage.snapshotRetiredBytes);
    assert.equal(inputBytes + identityBytes + typedBytes, lifecycle.multiPage.totalRetiredBytes);
    console.log(`Flow lifecycle independent oracle: ${lifecycle.admission.length} exact admission states, ${lifecycle.multiPage.inputPages} input pages, ${lifecycle.multiPage.totalRetiredBytes} retained bytes; third-party encoding and strict AJV agree`);
    const source = readFileSync(join(base, "🧪️tests/💾️binary/🦀️.rs"), "utf8");
    assert(source.includes("semio_flow_retained_snapshot_matches_neutral_wire_and_retains_failures"));
    assert(source.includes("semio_flow_retained_snapshot_rejects_retired_requests_and_closes_exact_bytes"), "retained Flow lifecycle native law is absent");
    if (segments.includes("--oracle-only")) return;
    const receipts = await runExactCargoLaws({ cwd: this.repoRoot, groups: [{ package: PACKAGE_NAME, target: { kind: "test", name: "flow_retained_decode" }, laws: ["semio_flow_retained_snapshot_matches_neutral_wire_and_retains_failures", "semio_flow_retained_snapshot_rejects_retired_requests_and_closes_exact_bytes"] }] });
    assert.equal(receipts[0]!.assertions, 2);
  }
}

type ArtifactDirectoryRow = { directory: string; id: string; kind: string; responsibility: string };

function artifactDirectorySemanticKey(segment: string): string {
  return segment.replace(/^[^A-Za-z0-9_.]+/u, "");
}

function resolveTaxonomyDirectoryReference(sourcePath: string, token: string, stdioRoot: string, repoRoot: string, accepts: (pieces: string[]) => boolean): string {
  const pieces = token.split("/");
  if (!accepts(pieces)) return token;
  let current: string;
  if (token.startsWith("✏️s/")) current = repoRoot;
  else if (token.startsWith("🗿️artifacts/")) current = stdioRoot;
  else if (token.startsWith("/../../🗿️artifacts/")) current = import.meta.dir;
  else if (token.startsWith(".") || token.startsWith("..")) current = dirname(sourcePath);
  else return token;
  const rendered: string[] = [];
  for (const piece of pieces) {
    if (!piece) { rendered.push(piece); continue; }
    if (piece === ".") { rendered.push(piece); continue; }
    if (piece === "..") { current = dirname(current); rendered.push(piece); continue; }
    const exact = join(current, piece);
    if (existsSync(exact)) { current = exact; rendered.push(piece); continue; }
    const key = artifactDirectorySemanticKey(piece);
    const matches = existsSync(current)
      ? readdirSync(current, { withFileTypes: true }).filter((entry) => artifactDirectorySemanticKey(entry.name) === key)
      : [];
    if (matches.length !== 1) return token;
    current = join(current, matches[0]!.name);
    rendered.push(matches[0]!.name);
  }
  return rendered.join("/");
}

function resolveArtifactDirectoryReference(sourcePath: string, token: string, stdioRoot: string, repoRoot: string): string {
  return resolveTaxonomyDirectoryReference(sourcePath, token, stdioRoot, repoRoot, (pieces) => {
    const artifactIndex = pieces.findIndex((piece) => artifactDirectorySemanticKey(piece) === "artifacts");
    return artifactIndex >= 0 && pieces.slice(artifactIndex + 1).some((piece) => artifactDirectorySemanticKey(piece) === "jpg");
  });
}

function renderArtifactDirectoryReferences(sourcePath: string, content: string, stdioRoot: string, repoRoot: string, directory: string): string {
  const canonicalRoot = content.replace(/(🗿️artifacts\/)([^/\s"'`]+)/gu, (match, prefix: string, candidate: string) => artifactDirectorySemanticKey(candidate) === "jpg" ? `${prefix}${directory}` : match);
  return canonicalRoot.replace(/[^\s"'`]+/gu, (token) => resolveArtifactDirectoryReference(sourcePath, token, stdioRoot, repoRoot));
}

class ArtifactDirectoryWiringScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const mode = segments[0];
    const artifact = segments[1];
    if ((mode !== "generate" && mode !== "check") || artifact !== "jpg") throw new Error("artifact-directory-wiring expects generate|check jpg");
    const repoRoot = this.repoRoot;
    const stdioRoot = resolve(repoRoot, ARTIFACT_OWNER);
    const artifactsRoot = join(stdioRoot, "🗿️artifacts");
    const definitions = readdirSync(artifactsRoot, { withFileTypes: true })
      .filter((entry) => entry.isDirectory())
      .map((entry) => ({ directory: entry.name, path: join(artifactsRoot, entry.name, "📜️artifact-definition.json") }))
      .filter((entry) => existsSync(entry.path))
      .map((entry) => ({ directory: entry.directory, definition: JSON.parse(readFileSync(entry.path, "utf8")) as { artifact?: unknown; directory?: unknown; id?: unknown } }));
    if (definitions.length !== 36) throw new Error(`artifact-directory-wiring expected 36 physical definitions, got ${definitions.length}`);
    const selected = definitions.find((entry) => entry.definition.artifact === artifact);
    if (!selected || selected.definition.id !== `s.stdio.${artifact}` || selected.definition.directory !== selected.directory) throw new Error(`artifact-directory-wiring ${artifact} definition does not own its physical directory`);
    const collectionPath = join(artifactsRoot, "🔣️.json");
    const collection = JSON.parse(readFileSync(collectionPath, "utf8")) as { "x-semio"?: { members?: ArtifactDirectoryRow[] } };
    const members = collection["x-semio"]?.members;
    if (!Array.isArray(members) || members.length !== 36) throw new Error("artifact-directory-wiring requires the complete 36-row artifact collection");
    const row = members.find((entry) => entry.id === `s.stdio.${artifact}`);
    if (!row || row.kind !== "artifact") throw new Error(`artifact-directory-wiring missing ${artifact} collection row`);
    row.directory = selected.directory;
    const expected = new Map<string, string>();
    expected.set(collectionPath, `${JSON.stringify(collection, null, 2)}\n`);
    const visit = (directory: string): void => {
      for (const entry of readdirSync(directory, { withFileTypes: true })) {
        if (entry.isDirectory() && ["target", "node_modules", "🗑️generated"].includes(entry.name)) continue;
        const path = join(directory, entry.name);
        if (entry.isDirectory()) { visit(path); continue; }
        if (path === import.meta.filename) continue;
        const original = readFileSync(path, "utf8");
        const rendered = renderArtifactDirectoryReferences(path, original, stdioRoot, repoRoot, selected.directory);
        if (rendered !== original) expected.set(path, rendered);
      }
    };
    visit(stdioRoot);
    const stale = [...expected].filter(([path, content]) => readFileSync(path, "utf8") !== content);
    if (mode === "check") {
      if (stale.length > 0) throw new Error(`artifact-directory-wiring ${artifact} has ${stale.length} stale files; first: ${relative(repoRoot, stale[0]![0])}`);
    } else {
      for (const [path, content] of stale) writeFileSync(path, content);
    }
    const remaining = stdioWalkText(stdioRoot).filter((path) => [...readFileSync(path, "utf8").matchAll(/🗿️artifacts\/([^/\s"'`]+)/gu)].some((match) => artifactDirectorySemanticKey(match[1]!) === "jpg" && match[1] !== selected.directory));
    if (remaining.length > 0) throw new Error(`artifact-directory-wiring ${artifact} left stale identity in ${relative(repoRoot, remaining[0]!)}`);
    console.log(`[stdio] artifact-directory-wiring ${mode}: ${artifact}=${selected.directory}; ${stale.length} files ${mode === "generate" ? "regenerated" : "stale"}`);
  }
}

type SubsetDirectoryOutcome = "accepted" | "rewritten" | "missing-directory" | "ambiguous-directory";
type SubsetDirectoryCase = { id: string; directories: string[]; reference: string; outcome: SubsetDirectoryOutcome };
type SubsetDirectoryFixture = { version: number; artifact: string; standard: string; subset: string; schema: string; canonicalDirectory: string; cases: SubsetDirectoryCase[] };

function selectSubsetDirectory(directories: string[], subset: string, reference: string): { outcome: SubsetDirectoryOutcome; directory?: string } {
  const matches = directories.filter((directory) => artifactDirectorySemanticKey(directory) === subset);
  if (matches.length === 0) return { outcome: "missing-directory" };
  if (matches.length !== 1) return { outcome: "ambiguous-directory" };
  return { outcome: matches[0] === reference ? "accepted" : "rewritten", directory: matches[0] };
}

function resolveSubsetDirectoryReference(sourcePath: string, token: string, stdioRoot: string, repoRoot: string, artifact: string, standard: string, subset: string): string {
  return resolveTaxonomyDirectoryReference(sourcePath, token, stdioRoot, repoRoot, (pieces) => {
    const keys = pieces.map(artifactDirectorySemanticKey);
    const artifacts = keys.indexOf("artifacts");
    const standards = keys.indexOf("standards", artifacts + 1);
    const subsets = keys.indexOf("subsets", standards + 1);
    return artifacts >= 0 && keys[artifacts + 1] === artifact && standards > artifacts && keys[standards + 1] === standard && subsets > standards && keys[subsets + 1] === subset;
  });
}

class SubsetDirectoryWiringScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const [mode, artifact, subset] = segments;
    if ((mode !== "generate" && mode !== "check") || artifact !== "semio" || subset !== "mesh") throw new Error("subset-directory-wiring expects generate|check semio mesh");
    const { default: assert } = await import("node:assert/strict");
    const fixtureRoot = join(import.meta.dir, "../../🧫️fixtures/🧭️wiring/🗂️subset-directory-wiring");
    const fixture = JSON.parse(readFileSync(join(fixtureRoot, "🔣️.json"), "utf8")) as SubsetDirectoryFixture;
    const schema = JSON.parse(readFileSync(join(import.meta.dir, "🧬️schema/🗂️subset-directory-wiring/🔣️.json"), "utf8"));
    const { default: Ajv2020 } = await import("ajv/dist/2020.js");
    const ajv = new Ajv2020({ strict: true });
    ajv.addKeyword({ keyword: "x-semio-formats", metaSchema: { type: "array", items: { type: "string" } } });
    const validate = ajv.compile(schema);
    assert(validate(fixture), ajv.errorsText(validate.errors));
    assert.equal(new Set(fixture.cases.map((row) => row.id)).size, fixture.cases.length);
    for (const row of fixture.cases) {
      const selected = selectSubsetDirectory(row.directories, fixture.subset, row.reference);
      assert.equal(selected.outcome, row.outcome, row.id);
      if (selected.directory) assert.equal(selected.directory, fixture.canonicalDirectory, row.id);
    }
    const repoRoot = this.repoRoot;
    const stdioRoot = resolve(repoRoot, ARTIFACT_OWNER);
    const subsetsRoot = join(stdioRoot, `🗿️artifacts/🧿️${fixture.artifact}/🏅️standards/🔖️${fixture.standard}/🪆️subsets`);
    const selection = selectSubsetDirectory(readdirSync(subsetsRoot, { withFileTypes: true }).filter((entry) => entry.isDirectory()).map((entry) => entry.name), fixture.subset, fixture.canonicalDirectory);
    if (!selection.directory || selection.outcome !== "accepted") throw new Error(`subset-directory-wiring ${fixture.subset} physical authority is ${selection.outcome}`);
    const manifestPath = join(subsetsRoot, "🔣️.json");
    const manifest = JSON.parse(readFileSync(manifestPath, "utf8")) as { artifact?: unknown; standard?: unknown; subsets?: Record<string, { schema?: unknown }> };
    if (manifest.artifact !== `s.stdio.${fixture.artifact}` || manifest.standard !== fixture.standard || manifest.subsets?.[fixture.subset]?.schema !== fixture.schema) throw new Error("subset-directory-wiring logical mesh manifest row is absent or detached from its schema");
    const expected = new Map<string, string>();
    for (const path of stdioWalkText(stdioRoot)) {
      if (path === import.meta.filename) continue;
      const original = readFileSync(path, "utf8");
      const rendered = original.replace(/[^\s"'`]+/gu, (token) => resolveSubsetDirectoryReference(path, token, stdioRoot, repoRoot, fixture.artifact, fixture.standard, fixture.subset));
      if (rendered !== original) expected.set(path, rendered);
    }
    const stale = [...expected].filter(([path, content]) => readFileSync(path, "utf8") !== content);
    if (mode === "check" && stale.length > 0) throw new Error(`subset-directory-wiring ${fixture.subset} has ${stale.length} stale files; first: ${relative(repoRoot, stale[0]![0])}`);
    if (mode === "generate") for (const [path, content] of stale) writeFileSync(path, content);
    const remaining = stdioWalkText(stdioRoot).filter((path) => [...readFileSync(path, "utf8").matchAll(/🗿️artifacts\/([^/\s"'`]+)\/🏅️standards\/([^/\s"'`]+)\/🪆️subsets\/([^/\s"'`]+)/gu)].some((match) => artifactDirectorySemanticKey(match[1]!) === fixture.artifact && artifactDirectorySemanticKey(match[2]!) === fixture.standard && artifactDirectorySemanticKey(match[3]!) === fixture.subset && match[3] !== selection.directory));
    if (remaining.length > 0) throw new Error(`subset-directory-wiring ${fixture.subset} left stale identity in ${relative(repoRoot, remaining[0]!)}`);
    console.log(`Stdio subset directory oracle: ${fixture.cases.length} schema-valid accepted/stale/missing/ambiguous cases`);
    console.log(`[stdio] subset-directory-wiring ${mode}: ${fixture.artifact}/${fixture.subset}=${selection.directory}; ${stale.length} files ${mode === "generate" ? "regenerated" : "stale"}`);
  }
}

function stdioWalkText(directory: string, files: string[] = []): string[] {
  for (const entry of readdirSync(directory, { withFileTypes: true })) {
    if (entry.isDirectory() && ["target", "node_modules", "🗑️generated"].includes(entry.name)) continue;
    const path = join(directory, entry.name);
    if (entry.isDirectory()) stdioWalkText(path, files);
    else if (/[.](?:rs|ts|js|json|feature|semio)$/u.test(entry.name) || ["Cargo.toml", "package.json"].includes(entry.name)) files.push(path);
  }
  return files;
}

function readScannedText(path: string): string | undefined {
  try {
    return readFileSync(path, "utf8");
  } catch (error) {
    if ((error as NodeJS.ErrnoException).code === "ENOENT") return undefined;
    throw error;
  }
}

/** 🖊️ Checks codec ownership against an independent JSON Schema oracle and the framework source tree. */
async function runDwgArtifactOwnership(root: string, repoRoot: string): Promise<void> {
  const { default: assert } = await import("node:assert/strict");
  const fixtureRoot = join(root, "../../🧫️fixtures/🖊️dwg-artifact-ownership");
  const fixture = JSON.parse(readFileSync(join(fixtureRoot, "🔣️.json"), "utf8")) as { cases: { path: string; allowed: boolean }[]; forbiddenFrameworkPatterns: string[] };
  const validate = await compileStdioScopeExport(join(root, "../.."), "StdioDwgArtifactOwnership");
  const ownsCodec = (path: string): boolean => {
    const parts = path.split("/");
    return parts[0] === "✏️s" && parts[1] === "🔌️plugins" && Boolean(parts[2]) && Boolean(parts[4]) && parts[3] === "🗿️artifacts" && (parts[4] === "🖊️dwg" || parts.slice(5).includes("🚪️io"));
  };
  for (const test of fixture.cases) {
    assert.equal(ownsCodec(test.path), test.allowed, test.path);
    assert.equal(validate(test.path), test.allowed, `AJV ownership oracle: ${test.path}`);
  }
  const forbidden = fixture.forbiddenFrameworkPatterns.map((pattern) => new RegExp(pattern, "u"));
  const sources = ["🧰️framework", "✏️s/🔨️modules"].flatMap((directory) => stdioWalkText(join(repoRoot, directory)));
  const failures = sources.filter((path) => /[.](?:rs|ts|js|json)$|Cargo[.]toml$/u.test(path) && !["/🧩️extension-modules/", "/🔌️plugin-modules/", "/📤️distribution/", "/dist/", "/🧫️fixtures/", "/🧪️tests/"].some((directory) => path.split(sep).join("/").includes(directory))).flatMap((path) => {
    const source = readScannedText(path)?.replace(/\/\*[\s\S]*?\*\/|^\s*\/\/.*$/gmu, "") ?? "";
    return forbidden.some((pattern) => pattern.test(source)) ? [relative(repoRoot, path)] : [];
  });
  assert.deepEqual(failures, [], "DWG codecs escaped artifact I/O ownership");
  const misplaced = stdioWalkText(join(repoRoot, "✏️s/🔌️plugins")).filter((path) => /[.](?:rs|ts)$/u.test(path) && path.split(sep).join("/").includes("/🗿️artifacts/") && !ownsCodec(relative(repoRoot, path).split(sep).join("/"))).filter((path) => {
    const source = readScannedText(path)?.replace(/\/\*[\s\S]*?\*\/|^\s*\/\/.*$/gmu, "") ?? "";
    return /(?:semio_hub_stdio|crate)::artifacts::dwg::|\bDwg(?:Drawing|Geometry|Entity|Color|Importer|Exporter)\b|\b(?:export|import)_dwg(?:_sync)?\b/u.test(source);
  }).map((path) => relative(repoRoot, path));
  assert.deepEqual(misplaced, [], "Other artifact facets contain DWG codecs");
  console.log(`stdio-dwg-artifact-ownership: cases=${fixture.cases.length} AJV=1 framework=clean modules=clean artifact-facets=clean`);
}

type HomeIoSurfaceFixture = {
  readonly schema: "semio.stdio.home-io-surface/v1";
  readonly features: { readonly homeIo: "home-io"; readonly fullArtifactCatalog: "full-artifact-catalog"; readonly componentAppAssembly: "component-app-assembly"; readonly spaceGuest: "space-guest" };
  readonly directArtifacts: readonly ["csv", "json", "xlsx", "zip"];
  readonly sharedCodecs: readonly ["binary", "deflate", "txt", "xml"];
  readonly fullArtifactCount: 36;
  readonly nativeCodecCount: 29;
  readonly surfaceCases: readonly { readonly id: string; readonly selected: readonly string[]; readonly catalog: number; readonly apps: boolean; readonly exports: boolean }[];
};

function cargoTomlFiles(directory: string, files: string[] = []): string[] {
  for (const entry of readdirSync(directory, { withFileTypes: true })) {
    if (entry.isDirectory() && [".git", ".nx", ".🧬semio", "node_modules", "target", "🗑️generated"].includes(entry.name)) continue;
    const path = join(directory, entry.name);
    if (entry.isDirectory()) cargoTomlFiles(path, files);
    else if (entry.name === "Cargo.toml") files.push(path);
  }
  return files;
}

/** 🏠️ Proves Space selects only the four Home I/O families plus their exact shared codec closure. */
class HomeIoSurfaceScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runDwgArtifactOwnership(this.root, this.repoRoot);
    const mode = segments[0] ?? "source";
    if (mode !== "source" && mode !== "native") throw new Error("home-io-surface expects source|native");
    const { default: assert } = await import("node:assert/strict");
    const fixtureRoot = join(this.root, "../../🧫️fixtures/🏠️home-io-surface");
    const fixture = JSON.parse(readFileSync(join(fixtureRoot, "🔣️.json"), "utf8")) as HomeIoSurfaceFixture;
    const validate = await compileStdioScopeExport(join(this.root, "../.."), "StdioHomeIoSurface");
    assert(validate(fixture), JSON.stringify(validate.errors));
    const manifest = readFileSync(join(this.root, "Cargo.toml"), "utf8");
    assert.match(manifest, /default\s*=\s*\["plugin-root"\]/u);
    const { parse: parseToml } = await import("@iarna/toml");
    const firstParty = Bun.TOML.parse(manifest) as { features: Record<string, string[]> };
    const independent = parseToml(manifest) as unknown as { features: Record<string, string[]> };
    assert.deepEqual(firstParty.features, independent.features);
    const artifactPrefix = "semio-s-artifact-stdio-";
    const featureClosure = (selected: readonly string[]): { features: Set<string>; artifacts: Set<string> } => {
      const closure = new Set<string>(), artifacts = new Set<string>();
      const pending = [...selected];
      while (pending.length) {
        const next = pending.pop()!;
        if (closure.has(next)) continue;
        assert(Object.hasOwn(firstParty.features, next), `unknown Stdio feature ${next}`);
        closure.add(next);
        for (const activation of firstParty.features[next]!) {
          if (Object.hasOwn(firstParty.features, activation)) pending.push(activation);
          const packageName = (activation.startsWith("dep:") ? activation.slice(4) : activation.split("/")[0]!)!;
          if (packageName.startsWith(artifactPrefix)) artifacts.add(packageName.slice(artifactPrefix.length));
        }
      }
      return { features: closure, artifacts };
    };
    for (const row of fixture.surfaceCases) {
      const closure = featureClosure(row.selected);
      assert.deepEqual({
        catalog: closure.artifacts.size,
        apps: closure.features.has(fixture.features.componentAppAssembly),
        exports: closure.features.has("plugin-root"),
      }, { catalog: row.catalog, apps: row.apps, exports: row.exports }, row.id);
    }
    const selected = [...fixture.directArtifacts, ...fixture.sharedCodecs].sort();
    const fullArtifacts = [...featureClosure([fixture.features.fullArtifactCatalog]).artifacts].sort();
    const homeArtifacts = [...featureClosure([fixture.features.homeIo]).artifacts].sort();
    const componentArtifacts = [...featureClosure([fixture.features.componentAppAssembly]).artifacts].sort();
    assert.equal(fullArtifacts.length, fixture.fullArtifactCount);
    assert.deepEqual(homeArtifacts, selected);
    assert.deepEqual(componentArtifacts, fullArtifacts);
    const root = readFileSync(join(this.root, "../../🦀️.rs"), "utf8");
    assert.match(root, /#\[cfg\(feature = "component-app-assembly"\)\]\s+#\[path = "🔌️plugin\/🦀️\.rs"\]\s+pub mod plugin;/u);
    assert.match(root, /#\[path = "🔌️plugin\/📇️catalog\/🦀️\.rs"\]\s+pub mod catalog;/u);
    assert(!root.includes("pub mod artifacts"), "Stdio composition remounted artifact implementations");
    for (const path of [
      resolve(this.repoRoot, ARTIFACT_OWNER, "🗿️artifacts/💾️binary/🦀️.rs"),
      resolve(this.repoRoot, ARTIFACT_OWNER, "🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🦀️.rs"),
      resolve(this.repoRoot, ARTIFACT_OWNER, "🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🦀️.rs"),
      resolve(this.repoRoot, ARTIFACT_OWNER, "🗿️artifacts/🔤️txt/🦀️.rs"),
      resolve(this.repoRoot, ARTIFACT_OWNER, "🗿️artifacts/🔤️txt/🏅️standards/🔖️utf-8/🦀️.rs"),
      resolve(this.repoRoot, ARTIFACT_OWNER, "🗿️artifacts/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🦀️.rs"),
    ]) assert.match(readFileSync(path, "utf8"), /#\[cfg\(feature = "component-app-assembly"\)\]\s+pub fn (?:artifact|standard|subset)\(/u, `${relative(this.repoRoot, path)} leaked its plugin app declaration into a catalog-only library`);
    const spaceManifestPath = resolve(this.repoRoot, "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/📦️packages/🦀️rust/Cargo.toml");
    const spaceManifest = readFileSync(spaceManifestPath, "utf8");
    assert(!spaceManifest.includes("semio-hub-stdio"), "Space retains the Stdio composition dependency");
    assert.match(spaceManifest, new RegExp(`semio-framework-os\\s*=\\s*\\{[^\\n]*features\\s*=\\s*\\["${fixture.features.spaceGuest}"\\][^\\n]*\\}`, "u"));
    const workspace = parseToml(readFileSync(join(this.repoRoot, "Cargo.toml"), "utf8")) as unknown as { workspace: { dependencies: Record<string, { package?: string } | string> } };
    const dependencyNames = (cargo: any): string[] => [cargo, ...Object.values(cargo.target ?? {})].flatMap((table: any) => Object.entries(table.dependencies ?? {}).map(([alias, value]: [string, any]) => {
      const declaration = value?.workspace === true ? workspace.workspace.dependencies[alias] : value;
      return declaration && typeof declaration === "object" && declaration.package ? declaration.package : alias;
    }));
    const artifactManifests = new Map<string, any>();
    for (const path of cargoTomlFiles(resolve(this.repoRoot, ARTIFACT_OWNER, "🗿️artifacts"))) {
      const cargo = parseToml(readFileSync(path, "utf8")) as any;
      if (cargo.package?.name?.startsWith(artifactPrefix)) artifactManifests.set(cargo.package.name, cargo);
    }
    const homeClosure = new Set<string>();
    const visitArtifact = (name: string): void => {
      if (name === `${artifactPrefix}contract`) return;
      if (!name.startsWith(artifactPrefix) || homeClosure.has(name)) return;
      homeClosure.add(name);
      const cargo = artifactManifests.get(name);
      assert(cargo, `Home I/O dependency has no artifact package ${name}`);
      for (const dependency of dependencyNames(cargo)) visitArtifact(dependency);
    };
    const spaceCargo = parseToml(spaceManifest) as any;
    const spaceDirect = dependencyNames(spaceCargo).filter((name) => name.startsWith(artifactPrefix));
    for (const name of spaceDirect) visitArtifact(name);
    assert.deepEqual([...homeClosure].map((name) => name.slice(artifactPrefix.length)).sort(), selected);
    for (const artifact of fixture.directArtifacts) assert(spaceDirect.includes(`${artifactPrefix}${artifact}`), `Space misses direct Home I/O artifact ${artifact}`);
    const consumers = cargoTomlFiles(this.repoRoot).filter((path) => path !== join(this.root, "Cargo.toml") && readFileSync(path, "utf8").includes("semio-hub-stdio = {"));
    assert(!consumers.includes(spaceManifestPath));
    const hubManifestPath = resolve(this.repoRoot, "🌎️hub/📦️packages/🦀️rust/Cargo.toml");
    const osHostManifestPath = resolve(this.repoRoot, "🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/Cargo.toml");
    assert(consumers.includes(hubManifestPath));
    const hubSource = readFileSync(hubManifestPath, "utf8");
    assert(hubSource.includes('"semio-hub-stdio/full-artifact-catalog"'), "native Hub provider must select its concrete Stdio deployment");
    const neutralManifest = Bun.TOML.parse(readFileSync(resolve(this.repoRoot, ARTIFACT_OWNER, "📦️packages/🦀️rust/Cargo.toml"), "utf8")) as any;
    assert(!neutralManifest.package.metadata.component, "general Stdio assembly advertises a concrete component");
    assert(!dependencyNames(neutralManifest).some((name) => name.startsWith(artifactPrefix) && name !== `${artifactPrefix}contract`), "general Stdio assembly statically depends on concrete artifacts");
    assert(!neutralManifest.features?.[fixture.features.fullArtifactCatalog], "general Stdio assembly contains a default concrete roster");
    const osHostManifest = readFileSync(osHostManifestPath, "utf8");
    const osHostCargo = parseToml(osHostManifest) as any;
    assert((osHostCargo.features[fixture.features.spaceGuest] as string[]).every((name) => !name.includes("stdio")), "neutral Space guest selects Stdio artifacts");
    assert.match(osHostManifest, new RegExp(`^os-host-full\\s*=\\s*\\["${fixture.features.spaceGuest}",\\s*"dep:semio-framework-deflate",\\s*"semio-framework-os-kernel/sync"\\]$`, "mu"));
    assert(!consumers.includes(osHostManifestPath), "OS host must not select domain artifact codecs");
    assert(!osHostManifest.includes("semio-hub-stdio"), "OS host depends on Stdio");
    const osHostRoot = readFileSync(resolve(this.repoRoot, "🧰️framework/🛍️products/💻️os/🖥️host/🦀️.rs"), "utf8");
    for (const module of ["host", "backbone", "instance", "workflow", "registry"]) assert.match(osHostRoot, new RegExp(`#\\[cfg\\(any\\(feature = "os-host-full", feature = "${fixture.features.spaceGuest}"\\)\\)\\]\\s+pub mod ${module} \\{`, "u"));
    const spaceRoot = resolve(this.repoRoot, "✏️s/🔌️plugins/🪐️space");
    const productionRefs = stdioWalkText(spaceRoot)
      .filter((path) => path.endsWith(".rs") && path.includes("/🗿️artifacts/🏠️home/") && !path.includes("/🧪️tests/"))
      .flatMap((path) => [...readFileSync(path, "utf8").matchAll(/semio_s_artifact_stdio_([a-z0-9_]+)/gu)].map((match) => match[1]!));
    assert(productionRefs.length > 0 && productionRefs.every((artifact) => selected.includes(artifact)), "Space Home I/O uses an artifact outside its selected Cargo closure");
    console.log(`stdio-home-io-surface-oracle: AJV=1 TOML=bun+iarna surfaces=${fixture.surfaceCases.length} direct=${fixture.directArtifacts.length} shared=${fixture.sharedCodecs.length} full=${fixture.fullArtifactCount} codecs=${fixture.nativeCodecCount} consumers=${consumers.length}`);
    if (mode === "native") {
      const options = { cwd: this.repoRoot, env: devToolingEnv(), budgetMs: buildBudgetMs() };
      runCmd("cargo", ["check", "--manifest-path", join(this.root, "Cargo.toml"), "-p", PACKAGE_NAME, "--lib", "--target", "wasm32-wasip2", "--no-default-features", "--features", fixture.features.homeIo, "--message-format=short"], options);
      runCmd("cargo", ["check", "--manifest-path", resolve(this.repoRoot, "🌎️hub/🧩️compositions/🪐️space/📦️packages/🦀️rust/Cargo.toml"), "-p", "semio-hub-space", "--lib", "--target", "wasm32-wasip2", "--message-format=short"], options);
    }
  }
}

/** 🧪️ Checks the neutral editor acceptance fixture with an independent JSON Schema validator: every stdio editor ships in
 * exactly one stdio package (the stdio component or one `🧩️extensions` family), the packages together ship every catalogue
 * format, and every editor owns exactly one launchable playground row across them. */
async function testEditorCatalogContract(packageRoot: string): Promise<void> {
  const root = resolve(packageRoot, "../..");
  const fixture = JSON.parse(readFileSync(join(root, "🧫️fixtures/✏️editor-catalog/🔣️.json"), "utf8")) as { editorCount: number; formatCount: number; editorApps: string[]; actions: { id: string }[]; deployedComponents: { id: string; package: string; path: string; role: string; componentKind: string }[] };
  const schema = JSON.parse(readFileSync(join(root, "🧬️schema/✏️editor-catalog/🔣️.json"), "utf8"));
  const { default: Ajv } = await import("ajv");
  const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
  if (!validate(fixture)) throw new Error(`editor catalogue fixture: ${JSON.stringify(validate.errors)}`);
  const { parse: parseToml } = await import("@iarna/toml");
  for (const row of fixture.deployedComponents) {
    const path = resolve(root, "../../..", row.path);
    const text = readFileSync(path, "utf8");
    const actual = Bun.TOML.parse(text) as any;
    if (!isDeepStrictEqual(actual, parseToml(text))) throw new Error(`${row.id}: independent manifest oracle differs`);
    if (actual.package.name !== row.package || actual.package.metadata.component.package !== `semio:${row.id}` || actual.package.metadata.semio.role !== row.role || actual.package.metadata.semio["component-kind"] !== row.componentKind) throw new Error(`${row.id}: deployed component ownership differs`);
    if (row.id !== "stdio" && existsSync(resolve(root, "../../..", row.path.replace("🌎️hub/🧩️compositions/🗄️stdio", ARTIFACT_OWNER)))) throw new Error(`${row.id}: prior concrete deployment owner remains`);
  }
  if (new Set(fixture.actions.map((action) => action.id)).size !== fixture.actions.length) throw new Error("editor catalogue repeats an edit action");
  const rust = readFileSync(join(root, "🧪️tests/✏️editor-catalog/🦀️.rs"), "utf8");
  const roots = [...rust.matchAll(/^    \([a-z0-9_]+, [a-z0-9_]+, semio_s_artifact_stdio_/gm)].length;
  if (roots !== fixture.editorCount) throw new Error(`editor catalogue expects ${fixture.editorCount} editors; native acceptance covers ${roots}`);
  const formats = new Map<string, string>();
  const apps = new Map<string, string>();
  for (const { id, manifest } of stdioPackageManifests(root)) {
    for (const format of stdioPackageEditorFormats(manifest)) {
      if (formats.has(format)) throw new Error(`${format} editors ship in both ${formats.get(format)} and ${id}`);
      formats.set(format, id);
    }
    for (const row of manifest.package.metadata.semio.playground ?? []) {
      if (apps.has(row.app)) throw new Error(`${row.app} has playground rows in both ${apps.get(row.app)} and ${id}`);
      apps.set(row.app, id);
    }
  }
  if (formats.size !== fixture.formatCount) throw new Error(`the stdio packages ship editors for ${formats.size}/${fixture.formatCount} formats`);
  if (fixture.editorApps.length !== fixture.editorCount) throw new Error("editor fixture count differs from its app identities");
  if (!isDeepStrictEqual([...apps.keys()].sort(), [...fixture.editorApps].sort())) throw new Error("each editor needs exactly one launchable playground across the stdio packages");
  console.log(`🧾️ editor catalogue fixture validated: ${roots} editors in ${new Set(apps.values()).size} packages, ${fixture.actions.length} edit operations`);
}

/** 📦️ Every stdio package manifest, parsed by Bun's TOML reader independently of the Rust census law's row scan: the
 * stdio component's own and one per family component under `🧩️extensions`. */
function stdioPackageManifests(root: string): { readonly id: string; readonly path: string; readonly manifest: StdioPackageManifest }[] {
  const families = readdirSync(resolve(root, "🧩️extensions"), { withFileTypes: true }).filter((entry) => entry.isDirectory()).map((entry) => resolve(root, "🧩️extensions", entry.name, "📦️packages/🦀️rust/Cargo.toml"));
  return [join(root, "📦️packages/🦀️rust/Cargo.toml"), ...families.sort()].map((path) => {
    const manifest = Bun.TOML.parse(readFileSync(path, "utf8")) as StdioPackageManifest;
    return { id: manifest.package.metadata.component.package.slice("semio:".length), path, manifest };
  });
}

/** 🧩️ The stdio formats whose editors one package's default build assembles: `component-app-assembly` reached through its
 * default feature closure (the stdio component) or requested on the artifact dependency itself (a family component). */
function stdioPackageEditorFormats(manifest: StdioPackageManifest): readonly string[] {
  const selected = new Set<string>();
  const pending = ["default"];
  while (pending.length) {
    const feature = pending.pop()!;
    if (selected.has(feature)) continue;
    selected.add(feature);
    pending.push(...(manifest.features?.[feature] ?? []));
  }
  const prefix = "semio-s-artifact-stdio-";
  const viaFeatures = [...selected].filter((feature) => feature.startsWith(prefix) && feature.endsWith("/component-app-assembly")).map((feature) => feature.slice(prefix.length, -"/component-app-assembly".length));
  const viaDependencies = Object.entries(manifest.dependencies ?? {}).filter(([name, spec]) => name.startsWith(prefix) && typeof spec === "object" && (spec.features ?? []).includes("component-app-assembly")).map(([name]) => name.slice(prefix.length));
  return [...new Set([...viaFeatures, ...viaDependencies])];
}

/** 📜️ The Cargo manifest fields the editor catalogue contract reads from a stdio package. */
type StdioPackageManifest = {
  readonly features?: Record<string, string[]>;
  readonly dependencies?: Record<string, string | { readonly features?: readonly string[] }>;
  readonly package: { readonly name: string; readonly metadata: { readonly component: { readonly package: string }; readonly semio: { readonly playground?: readonly { readonly app: string }[] } } };
};

class CompositionScript extends BundleScript {
  run(segments: string[]): void {
    if (segments.length !== 1 || segments[0] !== "prepare") throw new Error("Unknown Stdio composition command");
    const result = prepareStdioComposition(this.repoRoot, resolve(this.root, "../.."));
    console.log(`stdio composition: contributions=${result.contributions} apps=${result.apps} receipts=${result.receipts}`);
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments, "long");
    await testEditorCatalogContract(this.root);
    if (rest[0] === "editor-catalog-contract") return;
    await runCatalogRootContractTests(this.root, this.repoRoot);
    if (rest[0] === "catalog-root-contract") return;
    await runCargoTestBudgeted([PACKAGE_NAME], this.repoRoot, rest);
  }
}

/** 🧩️ Links every stdio package's own component (stdio and each `🧩️extensions` family) with the release component
 * profile, extracts its core module with JCO and validates it against the native WebAssembly parser and the component
 * function ceiling — the measured bound each family's bounded fleet must stay under. Each package's link runs within its
 * own build budget: one release link is the unit a deadline bounds. */
class EditorComponentCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await testEditorCatalogContract(this.root);
    const outputRoot = segments[0] ?? join(cargoTargetDirectory(this.repoRoot), "stdio-editor-components");
    if (segments.length > 1 || !isAbsolute(outputRoot)) throw new Error("editor-component-check accepts one absolute output directory");
    mkdirSync(outputRoot, { recursive: true });
    let interrupted = false;
    const interrupt = (): void => { interrupted = true; };
    process.on("SIGINT", interrupt);
    process.on("SIGTERM", interrupt);
    const env = devToolingEnv({});
    const jco = resolveWorkspaceBin("@bytecodealliance/jco", this.repoRoot);
    if (!jco) throw new Error("missing workspace component tooling");
    try {
      const packages = stdioPackageManifests(resolve(this.root, "../.."));
      for (const [index, { id, path, manifest }] of packages.entries()) {
        const started = Date.now();
        const control: CatalogControl = { cancelled: () => interrupted, remainingMs: () => Math.max(0, (buildBudgetMs() || CATALOG_DEADLINE_MS) - (Date.now() - started)) };
        const lib = manifest.package.name.replaceAll("-", "_");
        const args = ["rustc", "--manifest-path", path, "-p", manifest.package.name, "--profile", COMPONENT_PROFILE, "--lib", "--crate-type", "cdylib", "--target", "wasm32-wasip2"];
        const controller = new AbortController();
        const observe = (): void => { if (control.cancelled() || control.remainingMs() <= 0) controller.abort(); };
        observe();
        const watch = setInterval(observe, 100);
        let lease: Awaited<ReturnType<typeof acquireCargoBuildLeaseV1>> | undefined;
        try {
          lease = await acquireCargoBuildLeaseV1({ directory: repoCacheDirectory(this.repoRoot, "agents", "resource-leases"), buildDirectory: cargoBuildDirectory(this.repoRoot, env), args, signal: controller.signal });
          await runControlled("cargo", args, this.repoRoot, env, control);
        } finally {
          try { lease?.release(); } finally { clearInterval(watch); }
        }
        const out = join(outputRoot, id);
        mkdirSync(out, { recursive: true });
        await runControlled("node", [jco, "transpile", join(cargoTargetDirectory(this.repoRoot, env), "wasm32-wasip2", COMPONENT_PROFILE, `${lib}.wasm`), "-o", out, "--name", lib, "--map", "semio:framework/pure=./pure.js", "--map", "semio:framework/host-async=./host-async.js"], this.repoRoot, env, control);
        const core = readFileSync(join(out, `${lib}.core.wasm`));
        const structure = assertComponentizableCore(core);
        console.log(`🧩️ ${index + 1}/${packages.length} ${id} component validated: ${JSON.stringify({ ...structure, coreBytes: core.byteLength })}`);
      }
    } finally {
      process.off("SIGINT", interrupt);
      process.off("SIGTERM", interrupt);
    }
  }
}

/** 📈️ Runs the owned deterministic-iteration `Brep` kernel benchmark suite (`🏃️benches/⏱️brep-kernel.rs`) — moved here
 * from `semio-framework-3d` in ticket 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-
 * ARTIFACTS wave G5, alongside the `Brep` kernel itself. */
class BenchScript extends BundleScript {
  run(): void {
    runCmd("cargo", ["bench", "--manifest-path", join(this.root, "Cargo.toml"), "-p", "semio-hub-stdio"], { cwd: this.repoRoot, budgetMs: buildBudgetMs() });
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
  const ambientTarget = cargoTargetDirectory(repoRoot);
  const ambientBuild = cargoBuildDirectory(repoRoot);
  const developmentCache = pluginModulesRootIn(repoRoot, "dev");
  if (pathIsWithin(ambientTarget, exact)) throw new Error("catalog-root refuses the ambient shared target");
  if (pathIsWithin(ambientBuild, exact)) throw new Error("catalog-root refuses the ambient shared build directory");
  if (pathIsWithin(developmentCache, exact)) throw new Error("catalog-root refuses the development cache");
  if (exact === resolve(repoRoot)) throw new Error("catalog-root requires a dedicated fresh directory");
  return exact;
}

/** 🏗️ Keeps staged Cargo deliverables and intermediate compiler files in one private owner root. */
function catalogCargoEnvironment(target: string, inherited: NodeJS.ProcessEnv = process.env): NodeJS.ProcessEnv {
  return devToolingEnv({ ...inherited, CARGO_TARGET_DIR: target, CARGO_BUILD_BUILD_DIR: join(target, "build"), CARGO_INCREMENTAL: "0" });
}

/** 🌳 Builds and verifies the one strict stdio row from an empty caller-owned root. */
class CatalogRootScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { CATALOG_COMMIT_MARKER_FILENAME, auditPluginCatalogSources, createFreshCatalogBuildVerifier, createFreshCatalogCommitMarker } = await import("../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/✅️catalog-verification/🟦️.ts");
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
    const env = catalogCargoEnvironment(cargoTarget);
    try {
      mkdirSync(workRoot, { recursive: true });
      mkdirSync(stageRoot, { recursive: true });
      const packageId = componentPackageId(join(this.root, "Cargo.toml"));
      await runControlled("cargo", ["rustc", "--manifest-path", join(this.root, "Cargo.toml"), "-p", PACKAGE_NAME, "--profile", COMPONENT_PROFILE, "--lib", "--crate-type", "cdylib", "--target", "wasm32-wasip2"], this.repoRoot, env, control);
      const raw = join(cargoTarget, "wasm32-wasip2", COMPONENT_PROFILE, WASM_OUT);
      assertContainedBounded(buildRoot, raw, "raw component");
      const jco = resolveWorkspaceBin("@bytecodealliance/jco", this.repoRoot);
      if (!jco) throw new Error("missing @bytecodealliance/jco workspace binary; run bun install");
      const extractRoot = join(workRoot, "extract");
      mkdirSync(extractRoot, { recursive: true });
      await runControlled("node", [jco, "transpile", raw, "-o", extractRoot, "--name", "semio_hub_stdio", "--map", "semio:framework/pure=./pure.js", "--map", "semio:framework/host-async=./host-async.js"], this.repoRoot, env, control);
      const core = join(extractRoot, "semio_hub_stdio.core.wasm");
      assertContainedBounded(buildRoot, core, "jco-extracted core module");
      const witPath = join(workRoot, "stdio.wit");
      await runControlled("node", [jco, "wit", raw, "--output", witPath], this.repoRoot, env, control);
      const wit = readFileSync(witPath, "utf8");
      if (!/world\s+actor\s*\{/.test(wit) || !["reactor", "jobs", "checkpoint", "describe"].every((name) => new RegExp(`export\\s+${name}\\s*;`).test(wit))) {
        throw new Error("wasm-tools WIT oracle rejected the required actor exports");
      }
      await runControlled("cargo", ["build", "--manifest-path", resolve(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/Cargo.toml"), "-p", "semio-framework-plugin-describe"], this.repoRoot, env, control);
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
      publishCatalogCommitMarker(rowRoot, createFreshCatalogCommitMarker(source, buildRoot, { cancelled: control.cancelled }), CATALOG_COMMIT_MARKER_FILENAME);
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

/** 🧬️ Regenerates this plugin's committed native-codec projection `packSchemaHash` column from the live Rust
 * receipts (`os_pack::schema_hash`); the same test, run without the write mode, is the projection-equals-receipts law. */
class NativeCodecProjectionScript extends BundleScript {
  run(): void {
    runCmd("cargo", ["test", "--manifest-path", join(this.root, "Cargo.toml"), "-p", "semio-hub-stdio", "--features", "full-artifact-catalog", "--test", "native_openable_provider", "--", "native_codec_projection_pack_schema_hashes_equal_live_receipts", "--exact"], { cwd: this.repoRoot, env: devToolingEnv({ SEMIO_NATIVE_CODEC_PROJECTION: "write", CARGO_INCREMENTAL: "0" }), budgetMs: buildBudgetMs() });
  }
}

/** 🪶️ Concrete Stdio snapshot codec fleet with independent SQLite interoperability oracles. */
class SnapshotSqliteTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments[0] === "foundation") {
      await runCargoTestBudgeted(["semio-s-artifact-stdio-binary", "semio-s-artifact-stdio-txt", "semio-s-artifact-stdio-csv", "semio-s-artifact-stdio-tsv", "semio-s-artifact-stdio-json", "semio-s-artifact-stdio-xml"], this.repoRoot, ["--lib", "sqlite_snapshot_"]);
      const artifacts = join(this.repoRoot, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts");
      await runTestBudgeted(process.execPath, ["test", ...[
        "💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any",
        "🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any",
        "📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any",
        "📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any",
        "🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base",
        "📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base",
      ].map(root => join(artifacts, root, "🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts"))], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "catalog") {
      await runCargoTestBudgeted(["semio-hub-stdio"], this.repoRoot, ["--test", "editor_catalog", "sqlite_snapshot_"]);
      return;
    }
    if (segments[0] === "geometry") {
      await runCargoTestBudgeted(["semio-s-artifact-stdio-stl", "semio-s-artifact-stdio-obj", "semio-s-artifact-stdio-ply"], this.repoRoot, ["--lib", "sqlite_snapshot_"]);
      const artifacts = join(this.repoRoot, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts");
      await runTestBudgeted(process.execPath, ["test", ...[
        "🔺️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any",
        "🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry",
        "🧱️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any",
      ].map(root => join(artifacts, root, "🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts"))], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "images") {
      if (segments[1] !== "source") await runCargoTestBudgeted(["semio-s-artifact-stdio-bmp", "semio-s-artifact-stdio-png"], this.repoRoot, ["--lib", "sqlite_snapshot_"]);
      const artifacts = join(this.repoRoot, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts");
      await runTestBudgeted(process.execPath, ["test", ...[
        "🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any",
        "📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any",
      ].map(root => join(artifacts, root, "🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts"))], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "archives") {
      if (segments[1] !== "source") await runCargoTestBudgeted(["semio-s-artifact-stdio-deflate", "semio-s-artifact-stdio-zip"], this.repoRoot, ["--lib", "sqlite_snapshot_"]);
      const artifacts = join(this.repoRoot, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts");
      await runTestBudgeted(process.execPath, ["test", ...[
        "🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any",
        "🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base",
      ].map(root => join(artifacts, root, "🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts"))], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "gif") {
      if (segments[1] !== "source") await runCargoTestBudgeted(["semio-s-artifact-stdio-gif"], this.repoRoot, ["--lib", "sqlite_snapshot_"]);
      const artifacts = join(this.repoRoot, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards");
      await runTestBudgeted(process.execPath, ["test", ...[
        "7️⃣87a/🪆️subsets/✳️any",
        "9️⃣89a/🪆️subsets/🧱️base",
      ].map(root => join(artifacts, root, "🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts"))], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "jpg") {
      if (segments[1] !== "source") await runCargoTestBudgeted(["semio-s-artifact-stdio-jpg"], this.repoRoot, ["--lib", "sqlite_snapshot_"]);
      await runTestBudgeted(process.execPath, ["test", join(this.repoRoot, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts")], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "md") {
      if (segments[1] !== "source") await runCargoTestBudgeted(["semio-s-artifact-stdio-md"], this.repoRoot, ["--lib", "sqlite_snapshot_"]);
      await runTestBudgeted(process.execPath, ["test", join(this.repoRoot, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts")], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "mp3") {
      if (segments[1] !== "source") await runCargoTestBudgeted(["semio-s-artifact-stdio-mp3"], this.repoRoot, ["--lib", "sqlite_snapshot_"]);
      if (segments[1] !== "native") await runTestBudgeted(process.execPath, ["test", join(this.repoRoot, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts")], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "semio") {
      const artifacts = join(this.repoRoot, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets");
      const owners = ["✉️base", "🔤️text", "🔢️value", "🔊️audio", "🎬️video", "🖼️image", "🌊️flow", "📊️table", "🕸️graph", "🎞️animation", "📦️object", "🧰️kit", "🔺️mesh", "🏛️model", "📐️cad", "📑️document", "🖊️drawing", "📽️presentation", "🧊️brep"];
      const selected = segments[2] === undefined ? owners : owners.filter(owner => owner.endsWith(segments[2]!));
      if (selected.length === 0) throw new Error("Unknown Semio SQLite snapshot owner");
      if (segments[1] !== "source") await runCargoTestBudgeted(["semio-s-artifact-stdio-semio"], this.repoRoot, ["--lib", "sqlite_snapshot_"]);
      await runTestBudgeted(process.execPath, ["test", ...selected.map(owner => join(artifacts, owner, "🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts"))], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "tiff") {
      if (segments[1] !== "source") await runCargoTestBudgeted(["semio-s-artifact-stdio-tiff"], this.repoRoot, ["--lib", "sqlite_snapshot_"]);
      await runTestBudgeted(process.execPath, ["test", join(this.repoRoot, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts")], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "wav") {
      if (segments[1] !== "source") await runCargoTestBudgeted(["semio-s-artifact-stdio-wav"], this.repoRoot, ["--lib", "sqlite_snapshot_"]);
      await runTestBudgeted(process.execPath, ["test", join(this.repoRoot, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts")], { cwd: this.repoRoot });
      return;
    }
    throw new Error("Unknown authored Stdio SQLite snapshot group");
  }
}

const router = new ScriptRouter(import.meta.dir).register("composition", CompositionScript).register("snapshot-sqlite", SnapshotSqliteTestScript).register("editor-component-check", EditorComponentCheckScript).register("native-codec-projection", NativeCodecProjectionScript).register("test", TestScript).register("bench", BenchScript).register("catalog-root", CatalogRootScript).register("flow-retained-decode-check", FlowRetainedDecodeScript).register("artifact-directory-wiring", ArtifactDirectoryWiringScript).register("subset-directory-wiring", SubsetDirectoryWiringScript).register("home-io-surface", HomeIoSurfaceScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
