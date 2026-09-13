import { createHash } from "node:crypto";
import { cargoTargetDirectory, cargoBuildDirectory } from "../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🟦️.ts";
import { closeSync, existsSync, fstatSync, lstatSync, openSync, readdirSync, readFileSync, readSync, realpathSync, statSync } from "node:fs";
import { isAbsolute, dirname, join, relative, resolve } from "node:path";
import { isDeepStrictEqual } from "node:util";
import { BundleScript, getWorkspaceRoot, resolveTestLevel, runVitest } from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { APP_CHANNEL_VERSION, clonePackValue, decodePackValue, encodePackValue, packValueToExactJson } from "../../../../🟦️.ts";
import type { PackValue } from "../../../../🟦️.ts";
import { CATALOG_ID, COMPONENT_PACKAGE_ID, DESCRIPTOR_JSON_REL_PATH, PluginDescriptorHashes, PluginRegistryEntry, findPluginCargoFiles, parsePluginCargo } from "../🔎️discovery/🟦️.ts";
import { tomlBlocksAfterHeader } from "../🔎️discovery/🟦️.ts";

export class RegistryTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runVitest(this.root, rest, "./🧪️tests/🎚️config/🟦️.ts");
  }
}


//#endregion 🔖️DescriptorGate

//#region 🔖️CatalogCompleteness
export const CATALOG_NODE_MAX = 256;

export const CATALOG_DEPENDENCY_MAX = 128;

export const CATALOG_ARTIFACT_MAX_BYTES = 64 * 1024 * 1024;

export const CATALOG_DESCRIPTOR_MAX_BYTES = 4 * 1024 * 1024;

export const CATALOG_DIAGNOSTIC_MAX_BYTES = 4096;

export const CATALOG_COMMIT_MARKER_MAX_BYTES = 64 * 1024;

export const CATALOG_COMMIT_MARKER_FILENAME = "🧾️.catalog-root.json";

export const CATALOG_IO_CHUNK_BYTES = 64 * 1024;

export const CATALOG_PACKAGE_ID = COMPONENT_PACKAGE_ID;

export const CATALOG_SHA256 = /^[0-9a-f]{64}$/;

export const CATALOG_DESCRIPTOR_PACK_FILENAME = "🛂️.descriptor.semio";

export const CATALOG_DESCRIPTOR_TOP_LEVEL = new Set(["descriptorVersion", "packageId", "role", "manifest", "activationEvents", "capabilityRequests", "extensionPoints", "execution", "executionProtocol", "quotas", "contributions", "assets", "hashes"]);

export const CATALOG_MANIFEST_FIELDS = new Set(["pluginId", "label", "version", "apps", "examples", "capabilities", "topicContributions", "commands", "artifactKinds", "dependencies", "contributions"]);


export type CatalogVerificationNode = {
  readonly pluginId: string;
  readonly role: "plugin" | "extension";
  readonly dependsOn: readonly string[];
};


export type CatalogVerificationStatus = "verified" | "failed" | "blocked" | "cancelled";


export type CatalogVerificationProgress = {
  readonly schemaVersion: 1;
  readonly completed: number;
  readonly total: number;
  readonly pluginId: string;
  readonly status: CatalogVerificationStatus;
};


export type CatalogVerificationResult = {
  readonly schemaVersion: 1;
  readonly order: readonly string[];
  readonly results: readonly { readonly pluginId: string; readonly status: CatalogVerificationStatus; readonly diagnostic?: string }[];
  readonly publication: "committed" | "withheld" | "failed" | "not-requested";
  readonly publicationDiagnostic?: string;
};


export type CatalogVerificationExecutor<Receipt> = {
  readonly verify: (node: CatalogVerificationNode) => Promise<Receipt>;
  readonly publish?: (verified: readonly { readonly node: CatalogVerificationNode; readonly receipt: Receipt }[]) => Promise<void>;
};


export type CatalogVerificationControl = {
  readonly cancelled: () => boolean;
  readonly progress?: (progress: CatalogVerificationProgress) => void;
};


export type StrictCatalogDescriptor = {
  readonly entry: PluginRegistryEntry;
  readonly jsonPath: string;
  readonly jsonBytes: Uint8Array;
  readonly packPath: string;
  readonly packBytes: Uint8Array;
  readonly descriptor: Readonly<Record<string, unknown>>;
  readonly hashes: PluginDescriptorHashes;
};


export type CatalogSourceIssue = {
  readonly code: "manifest-invalid" | "identity-conflict" | "descriptor-pair-missing" | "descriptor-pair-incomplete" | "descriptor-invalid" | "dependency-invalid";
  readonly path: string;
  readonly pluginId?: string;
  readonly diagnostic: string;
};


export type CatalogSourceAudit = {
  readonly manifestCount: number;
  readonly entries: readonly PluginRegistryEntry[];
  readonly sources: readonly StrictCatalogDescriptor[];
  readonly order: readonly string[];
  readonly issues: readonly CatalogSourceIssue[];
};


export type CatalogArtifactProgress = {
  readonly pluginId: string;
  readonly artifact: "raw" | "core" | "descriptor";
  readonly bytesRead: number;
  readonly totalBytes: number;
};


export type CatalogFileReceipt = {
  readonly path: string;
  readonly bytes: number;
  readonly sha256: string;
};


export type FreshCatalogCommitMarker = {
  readonly schemaVersion: 1;
  readonly packageId: string;
  readonly pluginId: string;
  readonly packageName: string;
  readonly wasmOut: string;
  readonly raw: CatalogFileReceipt;
  readonly core: CatalogFileReceipt;
  readonly descriptor: CatalogFileReceipt;
  readonly descriptorSha256: string;
};


export type CatalogArtifactControl = {
  readonly cancelled?: () => boolean;
  readonly progress?: (progress: CatalogArtifactProgress) => void;
  readonly afterArtifact?: (artifact: CatalogArtifactProgress["artifact"]) => void;
};


export type CatalogArtifactReceipt = {
  readonly pluginId: string;
  readonly rawSha256: string;
  readonly coreSha256: string;
  readonly descriptorSha256: string;
  readonly rawBytes: Uint8Array;
  readonly coreBytes: Uint8Array;
  readonly descriptorBytes: Uint8Array;
};


export interface FreshCatalogBuildVerifier {
  readonly root: string;
  verify(entry: PluginRegistryEntry, control?: CatalogArtifactControl): CatalogArtifactReceipt;
}


/** 🧯️ Retains at most the public catalog diagnostic budget without splitting a Unicode scalar. */
export function boundedCatalogDiagnostic(value: unknown): string {
  const message = value instanceof Error ? value.message : String(value);
  if (Buffer.byteLength(message) <= CATALOG_DIAGNOSTIC_MAX_BYTES) return message;
  let output = "";
  for (const scalar of message) {
    if (Buffer.byteLength(output) + Buffer.byteLength(scalar) > CATALOG_DIAGNOSTIC_MAX_BYTES - 3) break;
    output += scalar;
  }
  return `${output}...`;
}


/** 🧭️ Returns true only when `candidate` is `root` or is structurally contained below it. */
export function pathIsWithin(root: string, candidate: string): boolean {
  const rel = relative(root, candidate);
  return rel === "" || (!rel.startsWith("..") && !isAbsolute(rel));
}


/** 🧬️ Rejects lossy or unsupported JSON values before they enter the pack identity calculation. */
export function isStrictJsonValue(value: unknown): boolean {
  if (value === null || typeof value === "string" || typeof value === "boolean") return true;
  if (typeof value === "number") return Number.isFinite(value) && (!Number.isInteger(value) || Number.isSafeInteger(value));
  if (Array.isArray(value)) return value.every(isStrictJsonValue);
  if (typeof value !== "object") return false;
  return Object.values(value as Record<string, unknown>).every(isStrictJsonValue);
}


/** 🪞️ Normalizes serde's external enum tags and the pack codec's `kind`/`value` tags for comparison. */
export function normalizeCatalogDescriptorEnums(value: unknown): unknown {
  if (Array.isArray(value)) return value.map(normalizeCatalogDescriptorEnums);
  if (value === null || typeof value !== "object") return value;
  const record = value as Record<string, unknown>;
  const keys = Object.keys(record);
  if (keys.length === 2 && keys.includes("kind") && keys.includes("value") && typeof record.kind === "string") return { [record.kind]: normalizeCatalogDescriptorEnums(record.value) };
  if (keys.length === 1 && keys[0] === "kind" && typeof record.kind === "string") return record.kind;
  return Object.fromEntries(Object.entries(record).map(([key, child]) => [key, normalizeCatalogDescriptorEnums(child)]));
}


/** 🧱️ Produces one stable dependency-first ordering, with ready plugins before ready extensions. */
export function orderCatalogNodes(nodes: readonly CatalogVerificationNode[]): CatalogVerificationNode[] {
  if (nodes.length === 0 || nodes.length > CATALOG_NODE_MAX) throw new Error(`catalog plan must contain 1..${CATALOG_NODE_MAX} nodes, got ${nodes.length}`);
  const byId = new Map<string, CatalogVerificationNode>();
  for (const node of nodes) {
    if (!CATALOG_ID.test(node.pluginId)) throw new Error(`catalog node has invalid plugin id ${JSON.stringify(node.pluginId)}`);
    if (node.role !== "plugin" && node.role !== "extension") throw new Error(`${node.pluginId}: invalid role ${JSON.stringify(node.role)}`);
    if (node.dependsOn.length > CATALOG_DEPENDENCY_MAX) throw new Error(`${node.pluginId}: dependencies exceed ${CATALOG_DEPENDENCY_MAX}`);
    if (byId.has(node.pluginId)) throw new Error(`catalog plan has duplicate plugin id ${JSON.stringify(node.pluginId)}`);
    if (new Set(node.dependsOn).size !== node.dependsOn.length) throw new Error(`${node.pluginId}: duplicate dependency identity`);
    byId.set(node.pluginId, node);
  }
  for (const node of nodes) {
    for (const dependency of node.dependsOn) {
      if (!CATALOG_ID.test(dependency)) throw new Error(`${node.pluginId}: invalid dependency id ${JSON.stringify(dependency)}`);
      if (!byId.has(dependency)) throw new Error(`${node.pluginId} depends on absent catalog node ${JSON.stringify(dependency)}`);
      if (dependency === node.pluginId) throw new Error(`${node.pluginId}: self dependency is a cycle`);
    }
  }
  const indegree = new Map(nodes.map((node) => [node.pluginId, node.dependsOn.length]));
  const dependents = new Map<string, string[]>();
  for (const node of nodes) for (const dependency of node.dependsOn) dependents.set(dependency, [...(dependents.get(dependency) ?? []), node.pluginId]);
  const compare = (left: CatalogVerificationNode, right: CatalogVerificationNode): number => (left.role === right.role ? left.pluginId.localeCompare(right.pluginId) : left.role === "plugin" ? -1 : 1);
  const ready = nodes.filter((node) => indegree.get(node.pluginId) === 0).sort(compare);
  const ordered: CatalogVerificationNode[] = [];
  while (ready.length > 0) {
    const node = ready.shift()!;
    ordered.push(node);
    for (const dependentId of dependents.get(node.pluginId) ?? []) {
      const next = indegree.get(dependentId)! - 1;
      indegree.set(dependentId, next);
      if (next === 0) {
        ready.push(byId.get(dependentId)!);
        ready.sort(compare);
      }
    }
  }
  if (ordered.length !== nodes.length) throw new Error(`catalog dependency cycle includes ${nodes.filter((node) => !ordered.includes(node)).map(({ pluginId }) => pluginId).sort().join(", ")}`);
  return ordered;
}


/** 🧾️ Verifies a bounded graph and exposes exactly one all-row publication call after full success. */
export async function executeCatalogVerificationPlan<Receipt>(nodes: readonly CatalogVerificationNode[], executor: CatalogVerificationExecutor<Receipt>, control: CatalogVerificationControl): Promise<CatalogVerificationResult> {
  const ordered = orderCatalogNodes(nodes);
  const results: { pluginId: string; status: CatalogVerificationStatus; diagnostic?: string }[] = [];
  const resultById = new Map<string, CatalogVerificationStatus>();
  const verified: { node: CatalogVerificationNode; receipt: Receipt }[] = [];
  for (const node of ordered) {
    let status: CatalogVerificationStatus;
    let diagnostic: string | undefined;
    if (control.cancelled()) {
      status = "cancelled";
      diagnostic = "catalog verification cancelled";
    } else {
      const unavailable = node.dependsOn.find((dependency) => resultById.get(dependency) !== "verified");
      if (unavailable) {
        status = "blocked";
        diagnostic = `dependency ${unavailable} was not verified`;
      } else {
        try {
          verified.push({ node, receipt: await executor.verify(node) });
          status = "verified";
        } catch (error) {
          status = "failed";
          diagnostic = boundedCatalogDiagnostic(error);
        }
      }
    }
    resultById.set(node.pluginId, status);
    results.push({ pluginId: node.pluginId, status, ...(diagnostic ? { diagnostic } : {}) });
    control.progress?.({ schemaVersion: 1, completed: results.length, total: ordered.length, pluginId: node.pluginId, status });
  }
  if (results.some(({ status }) => status !== "verified") || control.cancelled()) return { schemaVersion: 1, order: ordered.map(({ pluginId }) => pluginId), results, publication: "withheld" };
  if (!executor.publish) return { schemaVersion: 1, order: ordered.map(({ pluginId }) => pluginId), results, publication: "not-requested" };
  try {
    await executor.publish(verified);
    return { schemaVersion: 1, order: ordered.map(({ pluginId }) => pluginId), results, publication: "committed" };
  } catch (error) {
    return { schemaVersion: 1, order: ordered.map(({ pluginId }) => pluginId), results, publication: "failed", publicationDiagnostic: boundedCatalogDiagnostic(error) };
  }
}


/** 📄️ Reads one bounded regular non-symlink file and rejects a path that resolves outside `root`. */
export function readCatalogFile(path: string, root: string, limit: number): Uint8Array {
  const info = lstatSync(path);
  if (info.isSymbolicLink() || !info.isFile()) throw new Error(`${path} must be a regular non-symlink file`);
  if (info.size > limit) throw new Error(`${path} exceeds ${limit} bytes`);
  const realRoot = realpathSync(root);
  const realPath = realpathSync(path);
  if (!pathIsWithin(realRoot, realPath)) throw new Error(`${path} resolves outside ${root}`);
  return readFileSync(path);
}


/** 🧭️ Rejects duplicated object names before the platform JSON decoder can collapse them. */
export function rejectDuplicateJsonObjectNames(source: string): void {
  let index = 0;
  const whitespace = (): void => {
    while (index < source.length && /\s/u.test(source[index]!)) index++;
  };
  const string = (): string => {
    if (source[index] !== '"') throw new Error(`expected JSON string at byte ${index}`);
    const start = index++;
    while (index < source.length) {
      const character = source[index++]!;
      if (character === '"') return JSON.parse(source.slice(start, index)) as string;
      if (character === "\\") index++;
    }
    throw new Error("unterminated JSON string");
  };
  const value = (depth: number): void => {
    if (depth > 128) throw new Error("JSON nesting exceeds 128 levels");
    whitespace();
    if (source[index] === "{") {
      index++;
      whitespace();
      const names = new Set<string>();
      if (source[index] === "}") {
        index++;
        return;
      }
      while (true) {
        whitespace();
        const name = string();
        if (names.has(name)) throw new Error(`duplicate object field ${JSON.stringify(name)}`);
        names.add(name);
        whitespace();
        if (source[index++] !== ":") throw new Error(`expected JSON colon at byte ${index - 1}`);
        value(depth + 1);
        whitespace();
        const separator = source[index++];
        if (separator === "}") return;
        if (separator !== ",") throw new Error(`expected JSON object separator at byte ${index - 1}`);
      }
    }
    if (source[index] === "[") {
      index++;
      whitespace();
      if (source[index] === "]") {
        index++;
        return;
      }
      while (true) {
        value(depth + 1);
        whitespace();
        const separator = source[index++];
        if (separator === "]") return;
        if (separator !== ",") throw new Error(`expected JSON array separator at byte ${index - 1}`);
      }
    }
    if (source[index] === '"') {
      string();
      return;
    }
    const token = source.slice(index).match(/^(?:true|false|null|-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?)/u)?.[0];
    if (!token) throw new Error(`invalid JSON value at byte ${index}`);
    index += token.length;
  };
  value(0);
  whitespace();
  if (index !== source.length) throw new Error(`trailing JSON input at byte ${index}`);
}


export type CatalogDescriptorIdentity = Readonly<{
  pluginId: string;
  packageId: string;
  role: "plugin" | "extension";
  extends?: string;
  dependsOn: readonly string[];
}>;


export function validateCatalogExecutionProtocol(pluginId: string, candidate: unknown): void {
  if (candidate === null || typeof candidate !== "object" || Array.isArray(candidate)) throw new Error(`${pluginId}: descriptor executionProtocol must be an object`);
  const protocol = candidate as Record<string, unknown>;
  if (Object.keys(protocol).length !== 1 || !("appChannelVersion" in protocol)) throw new Error(`${pluginId}: descriptor executionProtocol must contain exactly appChannelVersion`);
  if (protocol.appChannelVersion !== APP_CHANNEL_VERSION) throw new Error(`${pluginId}: descriptor executionProtocol.appChannelVersion is unsupported`);
}


export function validateCatalogDescriptorValue(entry: CatalogDescriptorIdentity, descriptor: unknown): PluginDescriptorHashes {
  if (descriptor === null || typeof descriptor !== "object" || Array.isArray(descriptor) || !isStrictJsonValue(descriptor)) throw new Error(`${entry.pluginId}: descriptor is not a lossless JSON object`);
  const record = descriptor as Record<string, unknown>;
  const unknownTop = Object.keys(record).filter((key) => !CATALOG_DESCRIPTOR_TOP_LEVEL.has(key));
  if (unknownTop.length > 0) throw new Error(`${entry.pluginId}: descriptor has unknown fields ${unknownTop.sort().join(", ")}`);
  if (record.descriptorVersion !== 1 || record.role !== entry.role) throw new Error(`${entry.pluginId}: descriptor version/role does not match source identity`);
  if (record.packageId !== entry.packageId || !CATALOG_PACKAGE_ID.test(String(record.packageId))) throw new Error(`${entry.pluginId}: descriptor packageId does not match the complete Cargo component identity`);
  const manifest = record.manifest;
  if (manifest === null || typeof manifest !== "object" || Array.isArray(manifest)) throw new Error(`${entry.pluginId}: descriptor manifest must be an object`);
  const manifestRecord = manifest as Record<string, unknown>;
  const unknownManifest = Object.keys(manifestRecord).filter((key) => !CATALOG_MANIFEST_FIELDS.has(key));
  if (unknownManifest.length > 0) throw new Error(`${entry.pluginId}: manifest has unknown fields ${unknownManifest.sort().join(", ")}`);
  if (manifestRecord.pluginId !== entry.pluginId || !CATALOG_ID.test(String(manifestRecord.pluginId))) throw new Error(`${entry.pluginId}: manifest.pluginId does not match the Cargo component identity`);
  if (typeof manifestRecord.label !== "string" || typeof manifestRecord.version !== "string" || !Array.isArray(manifestRecord.apps) || !Array.isArray(manifestRecord.examples)) throw new Error(`${entry.pluginId}: manifest required fields do not decode`);
  for (const field of ["capabilities", "topicContributions", "commands", "artifactKinds", "dependencies", "contributions"] as const) if (manifestRecord[field] !== undefined && !Array.isArray(manifestRecord[field])) throw new Error(`${entry.pluginId}: manifest.${field} must be an array`);
  for (const field of ["activationEvents", "capabilityRequests", "extensionPoints", "assets"] as const) if (record[field] !== undefined && !Array.isArray(record[field])) throw new Error(`${entry.pluginId}: descriptor.${field} must be an array`);
  if (!["declarative", "linked", "isolated", "exclusive", "cold"].includes(String(record.execution))) throw new Error(`${entry.pluginId}: descriptor execution mode does not decode`);
  validateCatalogExecutionProtocol(entry.pluginId, record.executionProtocol);
  if (record.quotas === null || typeof record.quotas !== "object" || Array.isArray(record.quotas) || record.contributions === null || typeof record.contributions !== "object" || Array.isArray(record.contributions)) throw new Error(`${entry.pluginId}: descriptor quotas/contributions must be objects`);
  const hashes = record.hashes;
  if (hashes === null || typeof hashes !== "object" || Array.isArray(hashes)) throw new Error(`${entry.pluginId}: descriptor hashes must be an object`);
  const hashRecord = hashes as Record<string, unknown>;
  if (Object.keys(hashRecord).sort().join(",") !== "coreWasmSha256,descriptorSha256,wasmSha256") throw new Error(`${entry.pluginId}: descriptor hashes must contain exactly raw/core/descriptor SHA-256`);
  for (const [name, value] of Object.entries(hashRecord)) if (typeof value !== "string" || !CATALOG_SHA256.test(value)) throw new Error(`${entry.pluginId}: hashes.${name} must be lowercase 64-hex`);
  if (entry.role === "extension" && (!entry.extends || entry.dependsOn[0] !== entry.extends)) throw new Error(`${entry.pluginId}: extension host must be its first dependency`);
  if (entry.role === "plugin" && entry.extends !== undefined) throw new Error(`${entry.pluginId}: plugin cannot declare an extension host`);
  return hashRecord as PluginDescriptorHashes;
}


export type FreshCatalogPackageIdentityV1 = Readonly<{
  pluginId: string;
  packageId: string;
  version: string;
  role: "plugin" | "extension";
  execution: "isolated";
  wasmSha256: string;
  coreWasmSha256: string;
}>;


export const CATALOG_PLACEHOLDER_PLUGIN_IDS: readonly string[] = ["empty", "assembly-failed", "unknown", "placeholder"];

export const CATALOG_PLACEHOLDER_VERSION = "0.0.0";


export type OwnerDescriptorPairV1 = Readonly<{
  pluginId: string;
  packageId: string;
  role: "plugin" | "extension";
  version: string;
  execution: string;
  extends?: string;
  hashes: PluginDescriptorHashes;
  descriptor: Readonly<Record<string, unknown>>;
}>;


/** 🚫️ Refuses the `pluginId: "empty"` / `0.0.0` stub shape a failed component assembly mints, which
 * otherwise decodes, packs and hashes exactly like a real descriptor (the four CAD extension owners). */
export function rejectPlaceholderCatalogIdentity(pluginId: string, version: string): void {
  if (CATALOG_PLACEHOLDER_PLUGIN_IDS.includes(pluginId) || version === CATALOG_PLACEHOLDER_VERSION) throw new Error(`${pluginId}: descriptor carries the placeholder identity (pluginId=${JSON.stringify(pluginId)}, version=${JSON.stringify(version)}) a failed assembly mints`);
}


/** 🧬️ Verifies one emitted JSON/pack descriptor pair against the exact raw/core artifacts it
 * describes, deriving identity from the descriptor itself rather than from owner discovery. */
export function verifyDescriptorPairBytesV1(jsonBytes: Uint8Array, packBytes: Uint8Array, expected: Readonly<{ wasmSha256: string; coreWasmSha256: string }>): OwnerDescriptorPairV1 {
  if (jsonBytes.byteLength === 0 || jsonBytes.byteLength > CATALOG_DESCRIPTOR_MAX_BYTES || packBytes.byteLength === 0 || packBytes.byteLength > CATALOG_DESCRIPTOR_MAX_BYTES) throw new Error("emitted descriptor forms exceed the fixed boundary");
  let descriptor: unknown;
  let packed: PackValue;
  try {
    const json = new TextDecoder("utf-8", { fatal: true }).decode(jsonBytes);
    rejectDuplicateJsonObjectNames(json);
    descriptor = JSON.parse(json);
    packed = decodePackValue(packBytes);
  } catch (error) {
    throw new Error(`emitted descriptor forms do not decode: ${boundedCatalogDiagnostic(error)}`);
  }
  if (descriptor === null || typeof descriptor !== "object" || Array.isArray(descriptor)) throw new Error("emitted descriptor is not a JSON object");
  const record = descriptor as Record<string, any>;
  const role = record.role === "extension" ? "extension" : "plugin";
  const host = role === "extension" ? (record.manifest?.dependencies?.[0]?.pluginId as string | undefined) : undefined;
  const entry: CatalogDescriptorIdentity = { pluginId: String(record.manifest?.pluginId), packageId: String(record.packageId), role, extends: host, dependsOn: host ? [host] : [] };
  const hashes = validateCatalogDescriptorValue(entry, descriptor);
  if (hashes.wasmSha256 !== expected.wasmSha256 || hashes.coreWasmSha256 !== expected.coreWasmSha256) throw new Error(`${entry.pluginId}: emitted descriptor hashes do not name the exact raw/core artifacts it was emitted from`);
  if (!Buffer.from(encodePackValue(packed)).equals(Buffer.from(packBytes))) throw new Error(`${entry.pluginId}: emitted descriptor pack is not canonical or contains trailing bytes`);
  if (!isDeepStrictEqual(normalizeCatalogDescriptorEnums(descriptor), normalizeCatalogDescriptorEnums(packValueToExactJson(packed)))) throw new Error(`${entry.pluginId}: emitted descriptor JSON and pack forms disagree`);
  const blanked = clonePackValue(packed) as Record<string, any>;
  blanked.hashes.descriptorSha256 = "";
  if (createHash("sha256").update(encodePackValue(blanked)).digest("hex") !== hashes.descriptorSha256) throw new Error(`${entry.pluginId}: emitted descriptor self-hash mismatch`);
  rejectPlaceholderCatalogIdentity(entry.pluginId, String(record.manifest.version));
  return { pluginId: entry.pluginId, packageId: entry.packageId, role, version: String(record.manifest.version), execution: String(record.execution), extends: host, hashes, descriptor: packed as Record<string, unknown> };
}


/** 🧬️ Verifies fresh diagnostic JSON and packed descriptor bytes without discovering or publishing owner state. */
export function verifyFreshCatalogPackageV1(jsonBytes: Uint8Array, packBytes: Uint8Array, expected: FreshCatalogPackageIdentityV1): PluginDescriptorHashes {
  let pair: OwnerDescriptorPairV1;
  try {
    pair = verifyDescriptorPairBytesV1(jsonBytes, packBytes, expected);
  } catch (error) {
    throw new Error(`${expected.pluginId}: ${boundedCatalogDiagnostic(error)}`);
  }
  if (pair.pluginId !== expected.pluginId || pair.packageId !== expected.packageId || pair.role !== expected.role || pair.version !== expected.version || pair.execution !== expected.execution) throw new Error(`${expected.pluginId}: fresh descriptor identity, execution, or component hashes differ`);
  return pair.hashes;
}


/** 🔐️ Strict-decodes and cross-checks one owner-root JSON/pack `PackageDescriptor` pair. */
export function validateCatalogDescriptorPair(entry: PluginRegistryEntry, repoRoot: string): StrictCatalogDescriptor {
  const jsonPath = resolve(repoRoot, entry.cratePath, ...DESCRIPTOR_JSON_REL_PATH);
  const ownerRoot = dirname(jsonPath);
  const packPath = join(ownerRoot, CATALOG_DESCRIPTOR_PACK_FILENAME);
  if (!pathIsWithin(resolve(repoRoot), jsonPath) || !pathIsWithin(resolve(repoRoot), packPath)) throw new Error(`${entry.pluginId}: descriptor owner escapes the repository`);
  const jsonBytes = readCatalogFile(jsonPath, repoRoot, CATALOG_DESCRIPTOR_MAX_BYTES);
  const packBytes = readCatalogFile(packPath, repoRoot, CATALOG_DESCRIPTOR_MAX_BYTES);
  let descriptor: unknown;
  let packed: PackValue;
  try {
    const json = Buffer.from(jsonBytes).toString("utf8");
    rejectDuplicateJsonObjectNames(json);
    descriptor = JSON.parse(json);
  } catch (error) {
    throw new Error(`${entry.pluginId}: descriptor JSON does not decode: ${boundedCatalogDiagnostic(error)}`);
  }
  try {
    packed = decodePackValue(packBytes);
  } catch (error) {
    throw new Error(`${entry.pluginId}: descriptor pack does not decode: ${boundedCatalogDiagnostic(error)}`);
  }
  if (descriptor === null || typeof descriptor !== "object" || Array.isArray(descriptor) || !isStrictJsonValue(descriptor)) throw new Error(`${entry.pluginId}: descriptor is not a lossless JSON object`);
  const record = descriptor as Record<string, unknown>;
  const unknownTop = Object.keys(record).filter((key) => !CATALOG_DESCRIPTOR_TOP_LEVEL.has(key));
  if (unknownTop.length > 0) throw new Error(`${entry.pluginId}: descriptor has unknown fields ${unknownTop.sort().join(", ")}`);
  if (record.descriptorVersion !== 1 || record.role !== entry.role) throw new Error(`${entry.pluginId}: descriptor version/role does not match source identity`);
  if (record.packageId !== entry.packageId || !CATALOG_PACKAGE_ID.test(String(record.packageId))) throw new Error(`${entry.pluginId}: descriptor packageId does not match the complete Cargo component identity`);
  const manifest = record.manifest;
  if (manifest === null || typeof manifest !== "object" || Array.isArray(manifest)) throw new Error(`${entry.pluginId}: descriptor manifest must be an object`);
  const manifestRecord = manifest as Record<string, unknown>;
  const unknownManifest = Object.keys(manifestRecord).filter((key) => !CATALOG_MANIFEST_FIELDS.has(key));
  if (unknownManifest.length > 0) throw new Error(`${entry.pluginId}: manifest has unknown fields ${unknownManifest.sort().join(", ")}`);
  if (manifestRecord.pluginId !== entry.pluginId || !CATALOG_ID.test(String(manifestRecord.pluginId))) throw new Error(`${entry.pluginId}: manifest.pluginId does not match the Cargo component identity`);
  if (typeof manifestRecord.label !== "string" || typeof manifestRecord.version !== "string" || !Array.isArray(manifestRecord.apps) || !Array.isArray(manifestRecord.examples)) throw new Error(`${entry.pluginId}: manifest required fields do not decode`);
  for (const field of ["capabilities", "topicContributions", "commands", "artifactKinds", "dependencies", "contributions"] as const) if (manifestRecord[field] !== undefined && !Array.isArray(manifestRecord[field])) throw new Error(`${entry.pluginId}: manifest.${field} must be an array`);
  for (const field of ["activationEvents", "capabilityRequests", "extensionPoints", "assets"] as const) if (record[field] !== undefined && !Array.isArray(record[field])) throw new Error(`${entry.pluginId}: descriptor.${field} must be an array`);
  if (!["declarative", "linked", "isolated", "exclusive", "cold"].includes(String(record.execution))) throw new Error(`${entry.pluginId}: descriptor execution mode does not decode`);
  validateCatalogExecutionProtocol(entry.pluginId, record.executionProtocol);
  if (record.quotas === null || typeof record.quotas !== "object" || Array.isArray(record.quotas) || record.contributions === null || typeof record.contributions !== "object" || Array.isArray(record.contributions)) throw new Error(`${entry.pluginId}: descriptor quotas/contributions must be objects`);
  const hashes = record.hashes;
  if (hashes === null || typeof hashes !== "object" || Array.isArray(hashes)) throw new Error(`${entry.pluginId}: descriptor hashes must be an object`);
  const hashRecord = hashes as Record<string, unknown>;
  if (Object.keys(hashRecord).sort().join(",") !== "coreWasmSha256,descriptorSha256,wasmSha256") throw new Error(`${entry.pluginId}: descriptor hashes must contain exactly raw/core/descriptor SHA-256`);
  for (const [name, value] of Object.entries(hashRecord)) if (typeof value !== "string" || !CATALOG_SHA256.test(value)) throw new Error(`${entry.pluginId}: hashes.${name} must be lowercase 64-hex`);
  if (!Buffer.from(encodePackValue(packed)).equals(Buffer.from(packBytes))) throw new Error(`${entry.pluginId}: descriptor pack is not canonical or contains trailing bytes`);
  if (!isDeepStrictEqual(normalizeCatalogDescriptorEnums(descriptor), normalizeCatalogDescriptorEnums(packValueToExactJson(packed)))) throw new Error(`${entry.pluginId}: descriptor JSON and pack forms disagree`);
  const blanked = clonePackValue(packed) as Record<string, unknown>;
  (blanked.hashes as Record<string, unknown>).descriptorSha256 = "";
  const actualDescriptorHash = createHash("sha256").update(encodePackValue(blanked)).digest("hex");
  if (actualDescriptorHash !== hashRecord.descriptorSha256) throw new Error(`${entry.pluginId}: descriptor self-hash mismatch`);
  const exactHashes = hashRecord as PluginDescriptorHashes;
  if (!entry.hashes || entry.hashes.wasmSha256 !== exactHashes.wasmSha256 || entry.hashes.coreWasmSha256 !== exactHashes.coreWasmSha256 || entry.hashes.descriptorSha256 !== exactHashes.descriptorSha256) throw new Error(`${entry.pluginId}: rendered registry hashes do not exactly match the source descriptor`);
  if (entry.role === "extension" && (!entry.extends || entry.dependsOn[0] !== entry.extends)) throw new Error(`${entry.pluginId}: extension host must be its first dependency`);
  if (entry.role === "plugin" && entry.extends !== undefined) throw new Error(`${entry.pluginId}: plugin cannot declare an extension host`);
  return { entry, jsonPath, jsonBytes, packPath, packBytes, descriptor: packed as Record<string, unknown>, hashes: exactHashes };
}


/** 🧮️ Independently enumerates discovered component manifests and audits source identities without reading generated catalog files. */
export function auditPluginCatalogSources(repoRoot = getWorkspaceRoot(), control?: { readonly cancelled?: () => boolean; readonly progress?: (completed: number, total: number, path: string) => void; readonly ownerDescriptors?: "required" | "ignored" }): CatalogSourceAudit {
  const manifestPaths = findPluginCargoFiles(repoRoot);
  if (manifestPaths.length === 0 || manifestPaths.length > CATALOG_NODE_MAX) throw new Error(`catalog discovery returned ${manifestPaths.length} manifests; expected 1..${CATALOG_NODE_MAX}`);
  const entries: PluginRegistryEntry[] = [];
  let sources: StrictCatalogDescriptor[] = [];
  const issues: CatalogSourceIssue[] = [];
  const byPlugin = new Map<string, string>();
  const byPackage = new Map<string, string>();
  for (let index = 0; index < manifestPaths.length; index++) {
    const manifestPath = manifestPaths[index]!;
    if (control?.cancelled?.()) throw new Error("catalog source audit cancelled");
    let entry: PluginRegistryEntry | undefined;
    try {
      const info = lstatSync(manifestPath);
      if (info.isSymbolicLink() || !info.isFile()) throw new Error("Cargo manifest is not a regular non-symlink file");
      const manifestText = readFileSync(manifestPath, "utf8");
      const semioBlock = tomlBlocksAfterHeader(manifestText.split("\n"), (line) => line === "[package.metadata.semio]")[0]?.join("\n") ?? "";
      const rawRole = semioBlock.match(/^role\s*=\s*"([^"]+)"/m)?.[1];
      if (rawRole !== "plugin" && rawRole !== "extension") throw new Error(`metadata.semio.role must be plugin or extension, got ${JSON.stringify(rawRole)}`);
      entry = parsePluginCargo(manifestPath, repoRoot, undefined, control?.ownerDescriptors);
      if (!CATALOG_ID.test(entry.pluginId) || entry.role !== rawRole) throw new Error("Cargo component/role identity is malformed");
      if (rawRole === "extension" && (!entry.extends || entry.dependsOn[0] !== entry.extends)) throw new Error("extension must declare extends as its first dependency");
      if (rawRole === "plugin" && entry.extends !== undefined) throw new Error("plugin must not declare extends");
      const previousPlugin = byPlugin.get(entry.pluginId);
      const previousPackage = byPackage.get(entry.packageName);
      if (previousPlugin || previousPackage) {
        issues.push({ code: "identity-conflict", path: relative(repoRoot, manifestPath), pluginId: entry.pluginId, diagnostic: boundedCatalogDiagnostic(`duplicate identity conflicts with ${previousPlugin ?? previousPackage}`) });
      } else {
        byPlugin.set(entry.pluginId, manifestPath);
        byPackage.set(entry.packageName, manifestPath);
        entries.push(entry);
      }
    } catch (error) {
      issues.push({ code: "manifest-invalid", path: relative(repoRoot, manifestPath), diagnostic: boundedCatalogDiagnostic(error) });
    }
    if (entry && control?.ownerDescriptors !== "ignored") {
      const jsonPath = resolve(repoRoot, entry.cratePath, ...DESCRIPTOR_JSON_REL_PATH);
      const packPath = join(dirname(jsonPath), CATALOG_DESCRIPTOR_PACK_FILENAME);
      const jsonExists = existsSync(jsonPath);
      const packExists = existsSync(packPath);
      if (!jsonExists && !packExists) {
        issues.push({ code: "descriptor-pair-missing", path: relative(repoRoot, dirname(jsonPath)), pluginId: entry.pluginId, diagnostic: "owner-root descriptor JSON and pack are both missing" });
      } else if (!jsonExists || !packExists) {
        issues.push({ code: "descriptor-pair-incomplete", path: relative(repoRoot, dirname(jsonPath)), pluginId: entry.pluginId, diagnostic: `owner-root descriptor ${jsonExists ? "pack" : "JSON"} is missing` });
      } else {
        try {
          sources.push(validateCatalogDescriptorPair(entry, repoRoot));
        } catch (error) {
          issues.push({ code: "descriptor-invalid", path: relative(repoRoot, dirname(jsonPath)), pluginId: entry.pluginId, diagnostic: boundedCatalogDiagnostic(error) });
        }
      }
    }
    control?.progress?.(index + 1, manifestPaths.length, relative(repoRoot, manifestPath));
  }
  // 🔗️ A declared runtime dependency names a PLUGIN ID, so it either resolves against the discovered
  // catalog or it is an authoring defect — never silently dropped, the way the previous Cargo-package
  // derivation had to drop linked sub-crates (`draw-fsm`, `imperative-control`) that are not plugins.
  for (const entry of entries) {
    for (const dependencyId of entry.dependsOn) {
      if (byPlugin.has(dependencyId)) continue;
      issues.push({ code: "dependency-invalid", path: relative(repoRoot, byPlugin.get(entry.pluginId) ?? "Cargo.toml"), pluginId: entry.pluginId, diagnostic: boundedCatalogDiagnostic(`metadata.semio.depends-on names "${dependencyId}", which no discovered crate provides`) });
    }
  }
  const entryById = new Map(entries.map((entry) => [entry.pluginId, entry]));
  sources = sources.map((source) => ({ ...source, entry: entryById.get(source.entry.pluginId) ?? source.entry }));
  let order: string[] = [];
  try {
    order = orderCatalogNodes(entries).map(({ pluginId }) => pluginId);
  } catch (error) {
    issues.push({ code: "dependency-invalid", path: "Cargo.toml", diagnostic: boundedCatalogDiagnostic(error) });
  }
  issues.sort((left, right) => `${left.pluginId ?? ""}:${left.code}:${left.path}`.localeCompare(`${right.pluginId ?? ""}:${right.code}:${right.path}`));
  return { manifestCount: manifestPaths.length, entries, sources, order, issues };
}


/** #️⃣ Hashes one bounded build artifact in chunks with containment, progress and cancellation checks. */
export function sha256CatalogArtifact(path: string, containmentRoot: string, pluginId = "catalog", artifact: CatalogArtifactProgress["artifact"] = "raw", control: CatalogArtifactControl = {}): string {
  const info = lstatSync(path);
  if (info.isSymbolicLink() || !info.isFile()) throw new Error(`${artifact} artifact must be a regular non-symlink file`);
  if (info.size > CATALOG_ARTIFACT_MAX_BYTES) throw new Error(`${artifact} artifact exceeds ${CATALOG_ARTIFACT_MAX_BYTES} bytes`);
  const realRoot = realpathSync(containmentRoot);
  const realPath = realpathSync(path);
  if (!pathIsWithin(realRoot, realPath)) throw new Error(`${artifact} artifact escapes the fresh build root`);
  const hash = createHash("sha256");
  const buffer = Buffer.allocUnsafe(CATALOG_IO_CHUNK_BYTES);
  const descriptor = openSync(path, "r");
  let bytesRead = 0;
  try {
    while (bytesRead < info.size) {
      if (control.cancelled?.()) throw new Error("catalog artifact verification cancelled");
      const length = readSync(descriptor, buffer, 0, Math.min(buffer.length, info.size - bytesRead), bytesRead);
      if (length === 0) throw new Error(`${artifact} artifact changed while hashing`);
      hash.update(buffer.subarray(0, length));
      bytesRead += length;
      control.progress?.({ pluginId, artifact, bytesRead, totalBytes: info.size });
    }
  } finally {
    closeSync(descriptor);
  }
  if (statSync(path).size !== info.size) throw new Error(`${artifact} artifact changed while hashing`);
  return hash.digest("hex");
}


export function readVerifiedCatalogArtifact(path: string, containmentRoot: string, limit: number, pluginId: string, artifact: CatalogArtifactProgress["artifact"], control: CatalogArtifactControl): { readonly bytes: Uint8Array; readonly sha256: string } {
  const pathInfo = lstatSync(path);
  if (pathInfo.isSymbolicLink() || !pathInfo.isFile()) throw new Error(`${artifact} artifact must be a regular non-symlink file`);
  const realRoot = realpathSync(containmentRoot);
  const realPath = realpathSync(path);
  if (!pathIsWithin(realRoot, realPath)) throw new Error(`${artifact} artifact escapes the fresh build root`);
  const descriptor = openSync(path, "r");
  try {
    const before = fstatSync(descriptor);
    if (!before.isFile() || before.size > limit) throw new Error(`${artifact} artifact exceeds ${limit} bytes`);
    const bytes = Buffer.allocUnsafe(before.size);
    const hash = createHash("sha256");
    let bytesRead = 0;
    while (bytesRead < before.size) {
      if (control.cancelled?.()) throw new Error("catalog artifact verification cancelled");
      const length = readSync(descriptor, bytes, bytesRead, Math.min(CATALOG_IO_CHUNK_BYTES, before.size - bytesRead), bytesRead);
      if (length === 0) throw new Error(`${artifact} artifact changed while reading verified bytes`);
      hash.update(bytes.subarray(bytesRead, bytesRead + length));
      bytesRead += length;
      control.progress?.({ pluginId, artifact, bytesRead, totalBytes: before.size });
    }
    const after = fstatSync(descriptor);
    if (after.size !== before.size) throw new Error(`${artifact} artifact changed while reading verified bytes`);
    control.afterArtifact?.(artifact);
    return { bytes, sha256: hash.digest("hex") };
  } finally {
    closeSync(descriptor);
  }
}


export function catalogArtifactReceipt(path: string, containmentRoot: string, relativePath: string, pluginId: string, artifact: CatalogArtifactProgress["artifact"], control: CatalogArtifactControl): CatalogFileReceipt {
  const sha256 = sha256CatalogArtifact(path, containmentRoot, pluginId, artifact, control);
  return { path: relativePath, bytes: lstatSync(path).size, sha256 };
}


/** 🧾️ Computes the exact immutable receipt a producer must publish last for one fresh row. */
export function createFreshCatalogCommitMarker(source: StrictCatalogDescriptor, buildRoot: string, control: CatalogArtifactControl = {}): FreshCatalogCommitMarker {
  const exactRoot = realpathSync(resolve(buildRoot));
  const rowRoot = join(exactRoot, source.entry.pluginId);
  const rawRelative = join("raw", source.entry.wasmOut);
  const coreRelative = join("core", source.entry.wasmOut);
  const descriptorRelative = join("descriptor", CATALOG_DESCRIPTOR_PACK_FILENAME);
  const packageId = source.entry.packageId;
  if (!CATALOG_PACKAGE_ID.test(packageId) || source.descriptor.packageId !== packageId) throw new Error(`${source.entry.pluginId}: descriptor packageId cannot identify a commit marker`);
  return {
    schemaVersion: 1,
    packageId,
    pluginId: source.entry.pluginId,
    packageName: source.entry.packageName,
    wasmOut: source.entry.wasmOut,
    raw: catalogArtifactReceipt(join(rowRoot, rawRelative), exactRoot, rawRelative, source.entry.pluginId, "raw", control),
    core: catalogArtifactReceipt(join(rowRoot, coreRelative), exactRoot, coreRelative, source.entry.pluginId, "core", control),
    descriptor: catalogArtifactReceipt(join(rowRoot, descriptorRelative), exactRoot, descriptorRelative, source.entry.pluginId, "descriptor", control),
    descriptorSha256: source.hashes.descriptorSha256,
  };
}


export function requireExactCatalogRow(rowRoot: string, wasmOut: string): void {
  const expected = [CATALOG_COMMIT_MARKER_FILENAME, "core", "descriptor", "raw"].sort();
  if (!isDeepStrictEqual(readdirSync(rowRoot).sort(), expected)) throw new Error(`${rowRoot}: catalog row is not the exact committed triplet`);
  for (const [directory, filename] of [["raw", wasmOut], ["core", wasmOut], ["descriptor", CATALOG_DESCRIPTOR_PACK_FILENAME]] as const) {
    const root = join(rowRoot, directory);
    const info = lstatSync(root);
    if (info.isSymbolicLink() || !info.isDirectory() || !isDeepStrictEqual(readdirSync(root), [filename])) throw new Error(`${root}: catalog artifact directory is not exact`);
  }
}


/** 🏗️ Binds completion evidence to a caller-owned isolated root and rejects ambient target/cache authority. */
export function createFreshCatalogBuildVerifier(repoRoot: string, buildRoot: string): FreshCatalogBuildVerifier {
  if (!isAbsolute(buildRoot)) throw new Error("fresh catalog build root must be absolute");
  const resolvedRepo = resolve(repoRoot);
  const resolvedBuild = resolve(buildRoot);
  const sharedTarget = cargoTargetDirectory(resolvedRepo);
  const sharedBuild = cargoBuildDirectory(resolvedRepo);
  const developmentCache = resolve(resolvedRepo, "🧰️framework", "🛍️products", "💻️os", "🔨️modules", "🔌️plugin", "📦️packages", "🟦️typescript", "dist");
  if (pathIsWithin(sharedTarget, resolvedBuild)) throw new Error("fresh catalog verification cannot use the ambient shared target");
  if (pathIsWithin(sharedBuild, resolvedBuild)) throw new Error("fresh catalog verification cannot use the ambient shared build directory");
  if (pathIsWithin(developmentCache, resolvedBuild)) throw new Error("fresh catalog verification cannot use the development cache");
  if (resolvedBuild === resolvedRepo) throw new Error("fresh catalog build root must be a dedicated directory");
  const rootInfo = lstatSync(resolvedBuild);
  if (rootInfo.isSymbolicLink() || !rootInfo.isDirectory()) throw new Error("fresh catalog build root must be a regular non-symlink directory");
  const exactRoot = realpathSync(resolvedBuild);
  return {
    root: exactRoot,
    verify(entry, control = {}) {
      const rowRoot = join(exactRoot, entry.pluginId);
      const markerPath = join(rowRoot, CATALOG_COMMIT_MARKER_FILENAME);
      if (!existsSync(markerPath)) throw new Error(`${entry.pluginId}: catalog commit marker is missing`);
      const markerBytes = readCatalogFile(markerPath, exactRoot, CATALOG_COMMIT_MARKER_MAX_BYTES);
      let marker: unknown;
      try {
        const json = Buffer.from(markerBytes).toString("utf8");
        rejectDuplicateJsonObjectNames(json);
        marker = JSON.parse(json);
      } catch (error) {
        throw new Error(`${entry.pluginId}: catalog commit marker does not decode: ${boundedCatalogDiagnostic(error)}`);
      }
      if (marker === null || typeof marker !== "object" || Array.isArray(marker) || !isStrictJsonValue(marker)) throw new Error(`${entry.pluginId}: catalog commit marker is not an object`);
      const markerRecord = marker as Record<string, unknown>;
      const markerFields = ["core", "descriptor", "descriptorSha256", "packageId", "packageName", "pluginId", "raw", "schemaVersion", "wasmOut"];
      if (!isDeepStrictEqual(Object.keys(markerRecord).sort(), markerFields)) throw new Error(`${entry.pluginId}: catalog commit marker fields are not exact`);
      if (markerRecord.schemaVersion !== 1 || markerRecord.pluginId !== entry.pluginId || markerRecord.packageId !== entry.packageId || markerRecord.packageName !== entry.packageName || markerRecord.wasmOut !== entry.wasmOut || typeof markerRecord.descriptorSha256 !== "string" || !CATALOG_SHA256.test(markerRecord.descriptorSha256)) throw new Error(`${entry.pluginId}: catalog commit marker identity is not the carried Cargo identity`);
      if (!Buffer.from(markerBytes).equals(Buffer.from(`${JSON.stringify(marker)}\n`))) throw new Error(`${entry.pluginId}: catalog commit marker is not canonical JSON`);
      requireExactCatalogRow(rowRoot, entry.wasmOut);
      const rawPath = join(rowRoot, "raw", entry.wasmOut);
      const corePath = join(rowRoot, "core", entry.wasmOut);
      const descriptorPath = join(rowRoot, "descriptor", CATALOG_DESCRIPTOR_PACK_FILENAME);
      const raw = readVerifiedCatalogArtifact(rawPath, exactRoot, CATALOG_ARTIFACT_MAX_BYTES, entry.pluginId, "raw", control);
      const core = readVerifiedCatalogArtifact(corePath, exactRoot, CATALOG_ARTIFACT_MAX_BYTES, entry.pluginId, "core", control);
      const descriptor = readVerifiedCatalogArtifact(descriptorPath, exactRoot, CATALOG_DESCRIPTOR_MAX_BYTES, entry.pluginId, "descriptor", control);
      if (raw.sha256 === core.sha256) throw new Error(`${entry.pluginId}: raw component and extracted core identities are not distinct`);
      const fileReceipt = (path: string, bytes: Uint8Array, sha256: string): CatalogFileReceipt => ({ path, bytes: bytes.byteLength, sha256 });
      const expectedRaw = fileReceipt(join("raw", entry.wasmOut), raw.bytes, raw.sha256);
      const expectedCore = fileReceipt(join("core", entry.wasmOut), core.bytes, core.sha256);
      const descriptorFileSha256 = createHash("sha256").update(descriptor.bytes).digest("hex");
      const expectedDescriptor = fileReceipt(join("descriptor", CATALOG_DESCRIPTOR_PACK_FILENAME), descriptor.bytes, descriptorFileSha256);
      if (!isDeepStrictEqual(markerRecord.raw, expectedRaw) || !isDeepStrictEqual(markerRecord.core, expectedCore) || !isDeepStrictEqual(markerRecord.descriptor, expectedDescriptor)) throw new Error(`${entry.pluginId}: catalog commit marker artifact receipts disagree with the staged row`);
      let packed: PackValue;
      try {
        packed = decodePackValue(descriptor.bytes);
      } catch (error) {
        throw new Error(`${entry.pluginId}: staged descriptor pack does not decode: ${boundedCatalogDiagnostic(error)}`);
      }
      if (!Buffer.from(encodePackValue(packed)).equals(Buffer.from(descriptor.bytes))) throw new Error(`${entry.pluginId}: staged descriptor pack is not canonical`);
      const hashes = validateCatalogDescriptorValue(entry, packValueToExactJson(packed));
      const blanked = clonePackValue(packed) as Record<string, unknown>;
      (blanked.hashes as Record<string, unknown>).descriptorSha256 = "";
      const descriptorSha256 = createHash("sha256").update(encodePackValue(blanked)).digest("hex");
      if (hashes.wasmSha256 !== raw.sha256 || hashes.coreWasmSha256 !== core.sha256 || hashes.descriptorSha256 !== descriptorSha256 || markerRecord.descriptorSha256 !== descriptorSha256) throw new Error(`${entry.pluginId}: staged descriptor identity disagrees with the committed artifacts`);
      if (!Buffer.from(readCatalogFile(markerPath, exactRoot, CATALOG_COMMIT_MARKER_MAX_BYTES)).equals(Buffer.from(markerBytes))) throw new Error(`${entry.pluginId}: catalog commit marker changed during verification`);
      return {
        pluginId: entry.pluginId,
        rawSha256: raw.sha256,
        coreSha256: core.sha256,
        descriptorSha256,
        rawBytes: raw.bytes,
        coreBytes: core.bytes,
        descriptorBytes: descriptor.bytes,
      };
    },
  };
}

//#endregion 🔖️CatalogCompleteness

/** 🪪️ A registry build-planning identity, never an executable factory or verified catalog grant. */
export type NativeCatalogSelectionIdentityV1 = Readonly<{ pluginId: string; packageId: string; version: string }>;

export type NativeCatalogSelectionPackageV1 = NativeCatalogSelectionIdentityV1 & Readonly<{ dependencies: readonly NativeCatalogSelectionIdentityV1[]; receiptCount: number }>;

export type NativeCatalogProviderPreviewV1 = Readonly<{ pluginId: string; packageId: string; receiptCount: number }>;

export type NativeCatalogSelectionInputV1 = Readonly<{
  packages: readonly NativeCatalogSelectionPackageV1[];
  profiles: readonly Readonly<{ id: string; roots: readonly NativeCatalogSelectionIdentityV1[] }>[];
  availableProviders: readonly NativeCatalogProviderPreviewV1[];
}>;


/** 🧭️ Plans only a bounded dependency-first selected provider inventory; no bytes or authority are published. */
export function planNativeCatalogSelectionV1(input: NativeCatalogSelectionInputV1, profileId: string): readonly NativeCatalogSelectionPackageV1[] {
  const deny = (): never => { throw new Error("native catalog selection denied"); };
  const identity = (row: NativeCatalogSelectionIdentityV1): string => {
    if (!/^[a-z0-9]+(?:-[a-z0-9]+)*$/.test(row.pluginId) || row.pluginId.length > 128 || row.packageId !== `semio:${row.pluginId}` || !/^[0-9]+\.[0-9]+\.[0-9]+$/.test(row.version) || row.version.length > 64) deny();
    return `${row.pluginId}/${row.packageId}/${row.version}`;
  };
  if (!input.packages.length || input.packages.length > 256 || !input.profiles.length || input.profiles.length > 256 || input.availableProviders.length > 256) deny();
  const packages = new Map<string, NativeCatalogSelectionPackageV1>();
  const owners = new Set<string>();
  for (const row of input.packages) {
    const key = identity(row);
    if (packages.has(key) || owners.has(row.packageId) || row.dependencies.length > 256 || !Number.isSafeInteger(row.receiptCount) || row.receiptCount < 1 || row.receiptCount > 16384) deny();
    if (new Set(row.dependencies.map(identity)).size !== row.dependencies.length) deny();
    packages.set(key, row);
    owners.add(row.packageId);
  }
  const providers = new Map<string, NativeCatalogProviderPreviewV1>();
  for (const row of input.availableProviders) {
    const key = `${row.pluginId}/${row.packageId}`;
    if (row.packageId !== `semio:${row.pluginId}` || providers.has(key) || !Number.isSafeInteger(row.receiptCount) || row.receiptCount < 1 || row.receiptCount > 16384) deny();
    providers.set(key, row);
  }
  if (new Set(input.profiles.map(row => row.id)).size !== input.profiles.length) deny();
  const profile = input.profiles.find(row => row.id === profileId);
  if (!profile || !profile.roots.length || profile.roots.length > 256 || new Set(profile.roots.map(identity)).size !== profile.roots.length) deny();
  const visiting = new Set<string>();
  const completed = new Set<string>();
  const selected: NativeCatalogSelectionPackageV1[] = [];
  const compare = (a: NativeCatalogSelectionIdentityV1, b: NativeCatalogSelectionIdentityV1): number => identity(a) < identity(b) ? -1 : identity(a) > identity(b) ? 1 : 0;
  let totalReceipts = 0;
  const visit = (key: string): void => {
    if (completed.has(key)) return;
    if (visiting.has(key)) deny();
    const row = packages.get(key);
    if (!row) deny();
    visiting.add(key);
    for (const dependency of [...row.dependencies].sort(compare)) visit(identity(dependency));
    const provider = providers.get(`${row.pluginId}/${row.packageId}`);
    if (!provider || provider.receiptCount !== row.receiptCount) deny();
    totalReceipts += row.receiptCount;
    if (totalReceipts > 16384) deny();
    visiting.delete(key);
    completed.add(key);
    selected.push(Object.freeze({ ...row, dependencies: Object.freeze(row.dependencies.map(dependency => Object.freeze({ ...dependency }))) }));
  };
  for (const root of [...profile.roots].sort(compare)) visit(identity(root));
  return Object.freeze(selected);
}


/** 🧪️ Exercises private preview staging; callback inventories are not native receipts or activation inputs. */
export async function previewNativeCatalogSelectionV1(
  plan: readonly NativeCatalogSelectionPackageV1[],
  preview: (identity: NativeCatalogSelectionIdentityV1) => Promise<NativeCatalogProviderPreviewV1>,
  checkpoint: () => void,
  preflight: () => void,
): Promise<readonly NativeCatalogProviderPreviewV1[]> {
  const staged: NativeCatalogProviderPreviewV1[] = [];
  for (const row of plan) {
    checkpoint();
    const result = await preview(row);
    checkpoint();
    if (result.pluginId !== row.pluginId || result.packageId !== row.packageId || result.receiptCount !== row.receiptCount) throw new Error("native catalog preview identity denied");
    staged.push(Object.freeze({ ...result }));
  }
  checkpoint();
  preflight();
  return Object.freeze(staged);
}


export type SchemaValidator = ((value: unknown) => boolean) & { readonly errors?: unknown };


/** 🧬️ Compiles one named export of the registry schema module against the repository draft-07 dialect. */
export async function registrySchemaValidator(exportId: string): Promise<SchemaValidator> {
  const { default: Ajv } = await import("ajv");
  const document = JSON.parse(readFileSync(join(import.meta.dir, "..", "🧬️schema/🔣️.json"), "utf8"));
  const ajv = new Ajv({ strict: true, allErrors: true });
  ajv.addSchema(document);
  const validate = ajv.getSchema(`${document.$id}#/$defs/${exportId}`);
  if (!validate) throw new Error(`registry schema: unknown export ${exportId}`);
  return validate as SchemaValidator;
}


/** 🧫️ Independent AJV/Kahn closure oracle over neutral data; never reads a live factory or production descriptor. */
export async function nativeCatalogSelectionOracleV1(): Promise<void> {
  const fixtureRoot = join(import.meta.dir, "..", "🧫️fixtures/📦️native-catalog-selection");
  const fixture = JSON.parse(readFileSync(join(fixtureRoot, "🔣️.json"), "utf8"));
  const validate = await registrySchemaValidator("NativeCatalogSelectionV1");
  if (!validate(fixture)) throw new Error(`native catalog selection fixture denied: ${JSON.stringify(validate.errors)}`);
  const neutral = (candidate: any, profileId: string, mutation: string): { code: string; receiptCount: number; calls: string[] } => {
    const denied = (calls: string[] = []) => ({ code: "denied", receiptCount: 0, calls });
    if (!validate(candidate)) return denied();
    const same = (a: any, b: any) => a.pluginId === b.pluginId && a.packageId === b.packageId && a.version === b.version;
    if (new Set(candidate.packages.map((item: any) => item.packageId)).size !== candidate.packages.length || new Set(candidate.availableProviders.map((item: any) => item.packageId)).size !== candidate.availableProviders.length) return denied();
    if ([...candidate.packages, ...candidate.availableProviders].some((item: any) => item.packageId !== `semio:${item.pluginId}`)) return denied();
    const profile = candidate.profiles.find((item: any) => item.id === profileId);
    if (!profile || new Set(profile.roots.map((item: any) => item.packageId)).size !== profile.roots.length) return denied();
    const wanted: any[] = [];
    const pending = [...profile.roots];
    while (pending.length) {
      const identity = pending.shift();
      const item = candidate.packages.find((item: any) => same(item, identity));
      if (!item) return denied();
      if (wanted.some(item => same(item, identity))) continue;
      wanted.push(item);
      pending.push(...item.dependencies);
    }
    const ordered: any[] = [];
    while (ordered.length < wanted.length) {
      const next = wanted.filter(item => !ordered.includes(item) && item.dependencies.every((dependency: any) => ordered.some(item => same(item, dependency)))).sort((a, b) => a.packageId < b.packageId ? -1 : a.packageId > b.packageId ? 1 : 0)[0];
      if (!next) return denied();
      const provider = candidate.availableProviders.find((item: any) => item.pluginId === next.pluginId && item.packageId === next.packageId);
      if (!provider || provider.receiptCount !== next.receiptCount) return denied();
      ordered.push(next);
    }
    const calls: string[] = [];
    for (const item of ordered) {
      if (mutation === "cancel-before-first") return denied(calls);
      calls.push(item.packageId);
      if (mutation === "unselected-result" || mutation === "cancel-after-first" || mutation === "selected-provider-failure" && item.pluginId === "vcs") return denied(calls);
    }
    if (mutation === "registry-conflict") return denied(calls);
    return { code: "planned", receiptCount: ordered.reduce((sum, item) => sum + item.receiptCount, 0), calls };
  };
  for (const row of fixture.cases) {
    const candidate = structuredClone(fixture);
    let profileId = row.profileId;
    const vcs = candidate.packages.find((item: any) => item.pluginId === "vcs");
    const stdio = candidate.packages.find((item: any) => item.pluginId === "stdio");
    const root = candidate.profiles.find((item: any) => item.id === "native-stdio-vcs-v1").roots[0];
    const vcsProvider = candidate.availableProviders.find((item: any) => item.pluginId === "vcs");
    switch (row.mutation) {
      case "unknown-profile": profileId = "unavailable"; break;
      case "duplicate-root": candidate.profiles[1].roots.push({ ...root }); break;
      case "missing-root": candidate.profiles[1].roots = []; break;
      case "missing-dependency": candidate.packages = candidate.packages.filter((item: any) => item.pluginId !== "stdio"); break;
      case "dependency-version": vcs.dependencies[0].version = "9.9.9"; break;
      case "cycle": stdio.dependencies.push({ ...root }); break;
      case "duplicate-package": candidate.packages.push({ ...stdio }); break;
      case "foreign-package": vcs.packageId = "semio:other"; break;
      case "duplicate-provider": candidate.availableProviders.push({ ...candidate.availableProviders[0] }); break;
      case "missing-provider": candidate.availableProviders = candidate.availableProviders.filter((item: any) => item.pluginId !== "vcs"); break;
      case "provider-package": vcsProvider.packageId = "semio:other"; break;
      case "provider-count": vcsProvider.receiptCount = 2; break;
    }
    const calls: string[] = [];
    let receiptCount = 0;
    let code = "denied";
    const independent = neutral(candidate, profileId, row.mutation);
    if (!isDeepStrictEqual(independent, { code: row.expectedCode, receiptCount: row.expectedReceiptCount, calls: row.expectedCalls })) throw new Error(`independent native selection case ${row.id} differs: ${JSON.stringify(independent)}`);
    try {
      const plan = planNativeCatalogSelectionV1(candidate, profileId);
      if (!Object.isFrozen(plan) || plan.some(item => !Object.isFrozen(item) || !Object.isFrozen(item.dependencies) || item.dependencies.some(dependency => !Object.isFrozen(dependency)))) throw new Error("selection inventory is mutable");
      const result = await previewNativeCatalogSelectionV1(plan, async selected => {
        calls.push(selected.packageId);
        if (row.mutation === "selected-provider-failure" && selected.pluginId === "vcs") throw new Error("selected preview unavailable");
        if (row.mutation === "unselected-result") return candidate.availableProviders[1];
        return candidate.availableProviders.find((provider: any) => provider.packageId === selected.packageId);
      }, () => {
        if (row.mutation === "cancel-before-first" || (row.mutation === "cancel-after-first" && calls.length === 1)) throw new Error("cancelled");
      }, () => {
        if (row.mutation === "registry-conflict") throw new Error("preview conflict");
      });
      receiptCount = result.reduce((sum, item) => sum + item.receiptCount, 0);
      code = "planned";
    } catch {}
    if (code !== row.expectedCode || receiptCount !== row.expectedReceiptCount || !isDeepStrictEqual(calls, row.expectedCalls)) throw new Error(`native selection case ${row.id} differs: ${JSON.stringify({ code, receiptCount, calls })}`);
  }
  const positive = fixture.cases.filter((row: any) => row.expectedCode === "planned").length;
  console.log(`native-catalog-selection-oracle cases=${fixture.cases.length} positive=${positive} denied=${fixture.cases.length - positive} authority=planning-only published=0`);
}


export class NativeCatalogSelectionCheckScript extends BundleScript {
  async run(): Promise<void> {
    await nativeCatalogSelectionOracleV1();
  }
}


/** 🛂️ Fails closed unless every discovered source and explicit fresh-root artifact verifies. */
export class CatalogCompleteScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const repoRoot = getWorkspaceRoot();
    const option = (name: string): string | undefined => {
      const index = segments.indexOf(name);
      return index < 0 ? undefined : segments[index + 1];
    };
    const buildRoot = option("--build-root") ?? process.env.SEMIO_CATALOG_FRESH_BUILD_ROOT;
    const cancelFile = option("--cancel-file");
    if (!buildRoot || !isAbsolute(buildRoot)) throw new Error("usage: catalog-complete --build-root <absolute fresh build root> [--cancel-file <path>]");
    const cancelled = (): boolean => cancelFile !== undefined && existsSync(cancelFile);
    const audit = auditPluginCatalogSources(repoRoot, {
      cancelled,
      ownerDescriptors: "ignored",
      progress(completed, total, path) { console.log(`catalog-complete source ${completed}/${total}: ${path}`); },
    });
    if (audit.issues.length > 0) {
      console.error(`catalog-complete source preflight failed (${audit.issues.length} issue(s), ${audit.manifestCount} manifests):`);
      for (const issue of audit.issues) console.error(`  - [${issue.code}] ${issue.pluginId ?? issue.path}: ${issue.diagnostic}`);
      throw new Error(`catalog-complete refused unverified source catalog (${audit.issues.length} issue(s))`);
    }
    const verifier = createFreshCatalogBuildVerifier(repoRoot, buildRoot);
    const result = await executeCatalogVerificationPlan(audit.entries, {
      async verify(node) {
        const entry = audit.entries.find(({ pluginId }) => pluginId === node.pluginId);
        if (!entry) throw new Error(`${node.pluginId}: Cargo source identity is absent`);
        return verifier.verify(entry, { cancelled });
      },
    }, {
      cancelled,
      progress(event) { console.log(`catalog-complete artifact ${event.completed}/${event.total}: ${event.pluginId} ${event.status}`); },
    });
    if (result.results.some(({ status }) => status !== "verified")) {
      for (const row of result.results.filter(({ status }) => status !== "verified")) console.error(`  - [${row.status}] ${row.pluginId}: ${row.diagnostic ?? "unverified"}`);
      throw new Error("catalog-complete refused unverified fresh build artifacts");
    }
    console.log(`plugin catalog is complete: ${audit.manifestCount} source identities and fresh raw/core/descriptor artifacts verified from ${verifier.root}.`);
  }
}
