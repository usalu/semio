/** 🧾️ The catalog artifact io of the hub stdio package: the scope export compiler, the controlled and bounded readers, the release component preparation, the synced writes, the commit marker, the descriptor pair and the wasm core inspection with its independent oracles. */
import { runStdioOwnedCommandV1 } from "../../../📥️execution/🟦️.ts";
import { requireIssuedResourceControlInvocationV1, type ScriptInvocation } from "@semio-tech/process/routing";
import type { ResourceLeaseControl } from "@semio-tech/process/leases";
import { createHash } from "node:crypto";
import { closeSync, existsSync, fsyncSync, lstatSync, mkdirSync, mkdtempSync, openSync, readFileSync, readSync, realpathSync, renameSync, rmSync, statSync, writeSync } from "node:fs";
import { dirname, isAbsolute, join, relative, resolve } from "node:path";
import { decodePackValue, encodePackValue } from "@semio-tech/framework-os";
import { resolveWorkspaceBin } from "@semio-tech/process/execution";
import { checkResourceLeaseControl } from "@semio-tech/process/leases";
import { SHIPPED_COMPONENT_MAX_BYTES, SHIPPED_COMPONENT_PROFILE } from "@semio-tech/framework-plugin/describe/component-receipt";
import { ARTIFACT_MAX_BYTES, COMPONENT_FUNCTION_MAX, DESCRIPTOR_JSON, DESCRIPTOR_PACK, IO_CHUNK_BYTES, PLUGIN_ID } from "../🧮️component-constants/🟦️.ts";

/** 🧬️ Compiles one PascalCase `$defs` export of the Stdio plugin-root draft-07 schema module.
 * @see 🌎️hub/🧩️compositions/🗄️stdio/🧬️schema/🔣️.json */
export async function compileStdioScopeExport(stdioRoot: string, exportId: string) {
  const module = JSON.parse(readFileSync(join(stdioRoot, "🧬️schema", "🔣️.json"), "utf8"));
  const { default: Ajv } = await import("ajv");
  const ajv = new Ajv({ strict: true });
  ajv.addKeyword({ keyword: "x-semio-formats", metaSchema: { type: "array", items: { type: "string" } } });
  ajv.addSchema(module);
  return ajv.compile({ $ref: `${module.$id}#/$defs/${exportId}` });
}

type CatalogControl = { readonly cancelled: () => boolean; readonly remainingMs: () => number; readonly afterChunk?: (copied: number) => void };

export function assertControlled(control: CatalogControl): void {
  if (control.cancelled()) throw new Error("stdio catalog-root cancelled");
  if (control.remainingMs() <= 0) throw new Error("stdio catalog-root exceeded its configured deadline");
}

export interface CatalogExecution{readonly invocation:ScriptInvocation;readonly phase:ResourceLeaseControl}
export function catalogControlV1(execution:CatalogExecution):CatalogControl{requireIssuedResourceControlInvocationV1(execution.phase,execution.invocation);return{cancelled:()=>execution.phase.signal.aborted,remainingMs:()=>checkResourceLeaseControl(execution.phase)??Number.MAX_SAFE_INTEGER};}
export async function runControlled(execution:CatalogExecution,command:string,args:readonly string[],cwd:string,env:NodeJS.ProcessEnv,control:CatalogControl):Promise<void>{assertControlled(control);await runStdioOwnedCommandV1(execution.invocation,execution.phase,command,args,cwd,env);assertControlled(control);}

export function assertRegularBounded(path: string, label: string, maxBytes = ARTIFACT_MAX_BYTES): number {
  const info = lstatSync(path);
  if (info.isSymbolicLink() || !info.isFile()) throw new Error(`${label} must be a regular non-symlink file`);
  if (info.size > maxBytes) throw new Error(`${label} exceeds ${maxBytes} bytes`);
  return info.size;
}

export function assertContainedBounded(root: string, path: string, label: string, maxBytes = ARTIFACT_MAX_BYTES): number {
  const size = assertRegularBounded(path, label, maxBytes);
  if (!pathIsWithin(realpathSync(root), realpathSync(path))) throw new Error(`${label} escapes the fresh build root`);
  return size;
}

/** 🗜️ Produces a cancellable release component before applying the distribution admission limit. */
export async function prepareReleaseComponent(input: string, output: string, repoRoot: string, env: NodeJS.ProcessEnv, control: CatalogControl, execution:CatalogExecution): Promise<void> {
  requireIssuedResourceControlInvocationV1(execution.phase,execution.invocation);
  assertControlled(control);
  const inputBytes = assertRegularBounded(input, "compiler component", SHIPPED_COMPONENT_MAX_BYTES);
  if (resolve(input) === resolve(output)) throw new Error("release output cannot replace its compiler input");
  if (existsSync(output)) assertRegularBounded(output, "previous release component");
  const jco = resolveWorkspaceBin("@bytecodealliance/jco", repoRoot);
  if (!jco) throw new Error("missing workspace component tooling");
  mkdirSync(dirname(output), { recursive: true });
  const stage = mkdtempSync(join(dirname(output), ".stdio-release-"));
  const stagedOutput = join(stage, "component.wasm");
  try {
    await runControlled(execution,"node", [jco, "opt", input, "-o", stagedOutput, "--", "-Oz", "--low-memory-unused", "--enable-bulk-memory", "--enable-nontrapping-float-to-int", "--enable-sign-ext", "--strip-debug"], repoRoot, env, control);
    assertControlled(control);
    const outputBytes = assertRegularBounded(stagedOutput, "release component");
    renameSync(stagedOutput, output);
    console.log(`🗜️ release component optimized: ${JSON.stringify({ inputBytes, outputBytes })}`);
  } finally { rmSync(stage, { recursive: true, force: true }); }
}

export function copyCatalogArtifact(source: string, destination: string, label: string, control: CatalogControl): string {
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

export function componentPackageId(cargoManifestPath: string): string {
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

export function publishCatalogCommitMarker(rowRoot: string, marker: unknown, markerFilename: string): void {
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

export function atomicDescriptorPair(outDir: string, pack: Uint8Array, json: Uint8Array, failBeforeJson = false): void {
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

export async function webSha256(bytes: Uint8Array): Promise<string> {
  return Buffer.from(await globalThis.crypto.subtle.digest("SHA-256", new Uint8Array(bytes))).toString("hex");
}

type WasmCoreStructure = { readonly definedFunctions: number; readonly codeBodies: number };

function readWasmU32(bytes: Uint8Array, offset: number): { readonly value: number; readonly next: number } {
  let value = 0;
  let shift = 0;
  for (let index = 0; index < 5; index += 1) {
    if (offset >= bytes.byteLength) throw new Error("truncated wasm u32");
    const byte = bytes[offset++];if(byte===undefined)throw Error("Truncated component integer");
    value += (byte & 0x7f) * 2 ** shift;
    if ((byte & 0x80) === 0) {
      if (value > 0xffff_ffff) throw new Error("wasm u32 overflow");
      return { value, next: offset };
    }
    shift += 7;
  }
  throw new Error("wasm u32 exceeds five bytes");
}

export function inspectWasmCoreStructure(bytes: Uint8Array): WasmCoreStructure {
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

export function assertComponentizableCore(bytes: Uint8Array): WasmCoreStructure {
  const structure = inspectWasmCoreStructure(bytes);
  if (structure.definedFunctions > COMPONENT_FUNCTION_MAX) throw new Error(`core module has ${structure.definedFunctions} defined functions; component limit is ${COMPONENT_FUNCTION_MAX}`);
  if (structure.definedFunctions !== structure.codeBodies) throw new Error("core function and code section counts disagree");
  if (!WebAssembly.validate(bytes)) throw new Error("WebAssembly parser rejected core module");
  return structure;
}

/** 🔎️ Recomputes a staged row's identities without the registry verifier: WebCrypto digests of the raw component and the
 * semantic descriptor, the first-party Pack round trip, and the receipt read as plain JSON, which must name exactly those bytes. */
export async function verifyIndependentOracles(rawPath: string, descriptorPackPath: string, receiptPath: string): Promise<{ rawSha256: string; descriptorSha256: string; receipt: Record<string, unknown> }> {
  for (const [path, label, maximum] of [[rawPath, "raw component", SHIPPED_COMPONENT_MAX_BYTES], [descriptorPackPath, "descriptor pack", ARTIFACT_MAX_BYTES], [receiptPath, "component receipt", ARTIFACT_MAX_BYTES]] as const) assertRegularBounded(path, label, maximum);
  const raw = readFileSync(rawPath);
  const pack = readFileSync(descriptorPackPath);
  const decoded = decodePackValue(pack) as Record<string, unknown>;
  if (!Buffer.from(encodePackValue(decoded)).equals(pack)) throw new Error("independent Pack oracle rejected non-canonical descriptor bytes");
  if ((decoded.manifest as Record<string, unknown>)?.pluginId !== PLUGIN_ID || "hashes" in decoded) throw new Error("stdio descriptor is not the semantic stdio descriptor");
  const receipt = JSON.parse(readFileSync(receiptPath, "utf8")) as Record<string, unknown>;
  const rawSha256 = await webSha256(raw);
  const descriptorSha256 = await webSha256(pack);
  if (receipt.pluginId !== PLUGIN_ID || receipt.profile !== SHIPPED_COMPONENT_PROFILE || receipt.wasmSha256 !== rawSha256 || receipt.wasmBytes !== raw.byteLength || receipt.semanticDescriptorSha256 !== descriptorSha256) throw new Error("WebCrypto digests disagree with the stdio component receipt");
  return { rawSha256, descriptorSha256, receipt };
}

export function pathIsWithin(root: string, candidate: string): boolean {
  const rel = relative(root, candidate);
  return rel === "" || (!rel.startsWith("..") && !isAbsolute(rel));
}
