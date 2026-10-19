/** 🧬️ Descriptor pair of the catalog verification: the execution protocol and descriptor shape validation, the placeholder identity refusal, the descriptor pair decoding and the semantic and on disk descriptor pair verification. */
import { createHash } from "node:crypto";
import { dirname, join, resolve } from "node:path";
import { isDeepStrictEqual } from "node:util";
import { APP_CHANNEL_VERSION, decodePackValue, encodePackValue, packValueToExactJson } from "@semio-tech/framework-os";
import type { PackValue } from "@semio-tech/framework-os";
import { CATALOG_ID, DESCRIPTOR_JSON_REL_PATH, type CompiledComponentOwnerV1 } from "../../🔎️discovery/🟦️.ts";
import { CATALOG_DESCRIPTOR_MAX_BYTES, CATALOG_DESCRIPTOR_PACK_FILENAME, CATALOG_DESCRIPTOR_TOP_LEVEL, CATALOG_MANIFEST_FIELDS, CATALOG_PACKAGE_ID, CATALOG_PLACEHOLDER_PLUGIN_IDS, CATALOG_PLACEHOLDER_VERSION, type CatalogDescriptorIdentity, type OwnerDescriptorPairV1, type StrictCatalogDescriptor } from "../🟦️.ts";
import { boundedCatalogDiagnostic, isStrictJsonValue, normalizeCatalogDescriptorEnums, pathIsWithin, rejectDuplicateJsonObjectNames } from "../🛡️guards/🟦️.ts";
import { readCatalogFile } from "../📥️artifact-io/🟦️.ts";

//#region 🔖️CatalogCompleteness
export function validateCatalogExecutionProtocol(pluginId: string, candidate: unknown): void {
  if (candidate === null || typeof candidate !== "object" || Array.isArray(candidate)) throw new Error(`${pluginId}: descriptor executionProtocol must be an object`);
  const protocol = candidate as Record<string, unknown>;
  if (Object.keys(protocol).length !== 1 || !("appChannelVersion" in protocol)) throw new Error(`${pluginId}: descriptor executionProtocol must contain exactly appChannelVersion`);
  if (protocol.appChannelVersion !== APP_CHANNEL_VERSION) throw new Error(`${pluginId}: descriptor executionProtocol.appChannelVersion is unsupported`);
}

/** 🧩️ Admits every member of a semantic descriptor value against the identity it must carry; a byte binding is an unknown member. */
function validateCatalogDescriptorShape(entry: CatalogDescriptorIdentity, descriptor: unknown): Record<string, unknown> {
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
  for (const field of ["capabilities", "topicContributions", "commands", "artifactKinds", "hostedArtifactKinds", "dependencies", "contributions"] as const) if (manifestRecord[field] !== undefined && !Array.isArray(manifestRecord[field])) throw new Error(`${entry.pluginId}: manifest.${field} must be an array`);
  for (const field of ["activationEvents", "capabilityRequests", "extensionPoints", "assets"] as const) if (record[field] !== undefined && !Array.isArray(record[field])) throw new Error(`${entry.pluginId}: descriptor.${field} must be an array`);
  if (!["declarative", "linked", "isolated", "exclusive", "cold"].includes(String(record.execution))) throw new Error(`${entry.pluginId}: descriptor execution mode does not decode`);
  validateCatalogExecutionProtocol(entry.pluginId, record.executionProtocol);
  if (record.quotas === null || typeof record.quotas !== "object" || Array.isArray(record.quotas) || record.contributions === null || typeof record.contributions !== "object" || Array.isArray(record.contributions)) throw new Error(`${entry.pluginId}: descriptor quotas/contributions must be objects`);
  if (entry.role === "extension" && (!entry.extends || entry.dependsOn[0] !== entry.extends)) throw new Error(`${entry.pluginId}: extension host must be its first dependency`);
  if (entry.role === "plugin" && entry.extends !== undefined) throw new Error(`${entry.pluginId}: plugin cannot declare an extension host`);
  return record;
}

/** 🫥️ Admits a SEMANTIC descriptor value — a guest's `describe()` answer or a committed descriptor — which carries no
 * byte identity: the binding to shipped bytes lives in the generated `ComponentReceiptV1` (ticket 26/09/23 wave A2). */
export function validateSemanticCatalogDescriptorValue(entry: CatalogDescriptorIdentity, descriptor: unknown): void {
  validateCatalogDescriptorShape(entry, descriptor);
}

/** 🚫️ Refuses the `pluginId: "empty"` / `0.0.0` stub shape a failed component assembly mints, which
 * otherwise decodes, packs and hashes exactly like a real descriptor (the four CAD extension owners). */
export function rejectPlaceholderCatalogIdentity(pluginId: string, version: string): void {
  if (CATALOG_PLACEHOLDER_PLUGIN_IDS.includes(pluginId) || version === CATALOG_PLACEHOLDER_VERSION) throw new Error(`${pluginId}: descriptor carries the placeholder identity (pluginId=${JSON.stringify(pluginId)}, version=${JSON.stringify(version)}) a failed assembly mints`);
}

/** 🧾️ Decodes and identifies one JSON/pack descriptor pair: both forms strict, the pack canonical, the forms equal. */
function decodeDescriptorPairBytesV1(jsonBytes: Uint8Array, packBytes: Uint8Array): { record: Record<string, any>; packed: PackValue; entry: CatalogDescriptorIdentity; role: "plugin" | "extension"; host: string | undefined } {
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
  if (!Buffer.from(encodePackValue(packed)).equals(Buffer.from(packBytes))) throw new Error(`${entry.pluginId}: emitted descriptor pack is not canonical or contains trailing bytes`);
  if (!isDeepStrictEqual(normalizeCatalogDescriptorEnums(descriptor), normalizeCatalogDescriptorEnums(packValueToExactJson(packed)))) throw new Error(`${entry.pluginId}: emitted descriptor JSON and pack forms disagree`);
  return { record, packed, entry, role, host };
}

/** 🧬️ Verifies one emitted SEMANTIC JSON/pack descriptor pair, deriving identity from the descriptor itself rather than
 * from owner discovery: strict forms, a canonical pack, equal forms, no byte-identity binding and no placeholder identity. */
export function verifySemanticDescriptorPairBytesV1(jsonBytes: Uint8Array, packBytes: Uint8Array): OwnerDescriptorPairV1 {
  const { record, packed, entry, role, host } = decodeDescriptorPairBytesV1(jsonBytes, packBytes);
  validateSemanticCatalogDescriptorValue(entry, record);
  rejectPlaceholderCatalogIdentity(entry.pluginId, String(record.manifest.version));
  return { pluginId: entry.pluginId, packageId: entry.packageId, role, version: String(record.manifest.version), execution: String(record.execution), extends: host, semanticSha256: createHash("sha256").update(packBytes).digest("hex"), descriptor: packed as Record<string, unknown> };
}

/** 🔐️ Strict-decodes and cross-checks one owner-root SEMANTIC JSON/pack `PackageDescriptor` pair against the discovered
 * component identity: strict equal forms, a canonical pack and no byte-identity binding. */
export function validateCatalogDescriptorPair(entry: CompiledComponentOwnerV1, repoRoot: string): StrictCatalogDescriptor {
  const jsonPath = resolve(repoRoot, entry.cratePath, ...DESCRIPTOR_JSON_REL_PATH);
  const ownerRoot = dirname(jsonPath);
  const packPath = join(ownerRoot, CATALOG_DESCRIPTOR_PACK_FILENAME);
  if (!pathIsWithin(resolve(repoRoot), jsonPath) || !pathIsWithin(resolve(repoRoot), packPath)) throw new Error(`${entry.pluginId}: descriptor owner escapes the repository`);
  const jsonBytes = readCatalogFile(jsonPath, repoRoot, CATALOG_DESCRIPTOR_MAX_BYTES);
  const packBytes = readCatalogFile(packPath, repoRoot, CATALOG_DESCRIPTOR_MAX_BYTES);
  let decoded: ReturnType<typeof decodeDescriptorPairBytesV1>;
  try {
    decoded = decodeDescriptorPairBytesV1(jsonBytes, packBytes);
  } catch (error) {
    throw new Error(`${entry.pluginId}: committed descriptor pair refused: ${boundedCatalogDiagnostic(error)}`);
  }
  validateSemanticCatalogDescriptorValue(entry, decoded.record);
  return { entry, jsonPath, jsonBytes, packPath, packBytes, descriptor: decoded.packed as Record<string, unknown>, semanticSha256: createHash("sha256").update(packBytes).digest("hex") };
}
//#endregion 🔖️CatalogCompleteness
