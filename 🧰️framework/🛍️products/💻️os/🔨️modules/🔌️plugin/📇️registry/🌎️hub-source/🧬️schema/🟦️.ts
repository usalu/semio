/**
 * 🧩️ TypeScript projection of the hub's trusted plugin module contract: `TrustedPluginModuleBundleV1`
 * (one package's browser plugin module) and `TrustedPluginModuleIndexV1` (every module of one catalog
 * generation). Every bound and identifier is read from the normative draft-07 module, and every rule is
 * the one the hub enforces in `🧩️plugin-module/🦀️.rs`; both replay one language-agnostic fixture.
 * @see ../../../../../../../../🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧬️schema/🔣️.json
 * @see ../../../../../../../../🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧩️plugin-module/🦀️.rs
 * @see ../../../../../../../../🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/🧩️plugin-module/🔣️.json
 * The sibling `🔣️.json` owns the browser's own persisted store of installed modules (`PluginModuleStoreRecordV1`).
 */
import { blake3Hex } from "../../../../../../../🔨️modules/🔏️hash/🟦️.ts";
import schemaModule from "../../../../../../../../🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧬️schema/🔣️.json";
import storeSchemaModule from "./🔣️.json";
import { MODULE_BRIDGE_FILE, MODULE_VENDOR_DIRECTORY } from "../../📦️deployment/🟦️.ts";

const defs = schemaModule.$defs;
const bundleProperties = defs.TrustedPluginModuleBundleV1.properties;
const indexEntryProperties = defs.TrustedPluginModuleIndexEntryV1.properties;

export const TRUSTED_PLUGIN_MODULE_SCHEMA: string = bundleProperties.schema.const;
export const TRUSTED_PLUGIN_MODULE_INDEX_SCHEMA: string = defs.TrustedPluginModuleIndexV1.properties.schema.const;
export const TRUSTED_PLUGIN_MODULE_BUNDLE_MAX_BYTES: number = defs.TrustedBundlePluginModuleV1.properties.byteLength.maximum;
export const TRUSTED_PLUGIN_MODULE_FILE_MAX_BYTES: number = defs.TrustedPluginModuleFileV1.properties.byteLength.maximum;
export const TRUSTED_PLUGIN_MODULE_INDEX_MAX_MODULES: number = defs.TrustedPluginModuleIndexV1.properties.modules.maxItems;
export const TRUSTED_PLUGIN_MODULE_DESCRIPTOR_JSON_FILE = "🔣️.json";
export const TRUSTED_PLUGIN_MODULE_DESCRIPTOR_PACK_FILE = "🛂️.descriptor.semio";
const IDENTITY_MAX_BYTES: number = defs.identity.maxLength;
const PATH_MAX_CHARS: number = defs.TrustedPluginModulePathV1.maxLength;
const PATH_PATTERN = new RegExp(defs.TrustedPluginModulePathV1.pattern, "u");
const DIRECTORY_MAX_CHARS: number = bundleProperties.moduleDirectory.maxLength;
const DIRECTORY_PATTERN = new RegExp(bundleProperties.moduleDirectory.pattern, "u");
const DIGEST_PATTERN = new RegExp(defs.digest.pattern, "u");
const FILES_MIN: number = bundleProperties.files.minItems;
const FILES_MAX: number = bundleProperties.files.maxItems;
const DEPENDENCIES_MAX: number = indexEntryProperties.dependencies.maxItems;
const DIALECT_KINDS_MAX: number = indexEntryProperties.dialectArtifactKinds.maxItems;
const BUNDLE_KEYS: readonly string[] = defs.TrustedPluginModuleBundleV1.required;
const FILE_KEYS: readonly string[] = defs.TrustedPluginModuleFileV1.required;
const INDEX_KEYS: readonly string[] = defs.TrustedPluginModuleIndexV1.required;
const INDEX_ENTRY_KEYS: readonly string[] = defs.TrustedPluginModuleIndexEntryV1.required;

export type TrustedPluginModuleFileV1 = Readonly<{ path: string; byteLength: number; sha256: string; blake3: string }>;
export type TrustedPluginModuleBundleV1 = Readonly<{
  schema: string;
  pluginId: string;
  packageId: string;
  version: string;
  sourceComponentSha256: string;
  sourceDescriptorByteSha256: string;
  moduleDirectory: string;
  entry: string;
  files: readonly TrustedPluginModuleFileV1[];
}>;
export type TrustedPluginModuleIndexEntryV1 = Readonly<{
  pluginId: string;
  packageId: string;
  version: string;
  componentSha256: string;
  descriptorByteSha256: string;
  dependencies: readonly string[];
  dialectArtifactKinds: readonly string[];
  extendsPluginId: string | null;
  bundleSha256: string;
  bundleByteLength: number;
  entry: string;
}>;
export type TrustedPluginModuleIndexV1 = Readonly<{ schema: string; generationId: string; modules: readonly TrustedPluginModuleIndexEntryV1[] }>;
/** 🪪️ The package a plugin module must have been derived from. */
export type TrustedPluginModuleSourceV1 = Readonly<{ pluginId: string; packageId: string; version: string; componentSha256: string; descriptorByteSha256: string }>;

/** 🚫️ Every refusal of this contract, one type so a caller can tell a refused module from a failed transport. */
export class TrustedPluginModuleRefusalV1 extends Error {
  override readonly name = "TrustedPluginModuleRefusalV1";
}

const refuse = (reason: string): never => {
  throw new TrustedPluginModuleRefusalV1(`trusted plugin module ${reason}`);
};

const utf8 = new TextEncoder();

/** 🔢️ Byte order of the UTF-8 encodings — the order the hub sorts paths and plugin ids by. */
export function utf8OrderV1(left: string, right: string): number {
  const a = utf8.encode(left), b = utf8.encode(right);
  for (let index = 0; index < Math.min(a.length, b.length); index++) if (a[index] !== b[index]) return a[index]! - b[index]!;
  return a.length - b.length;
}

function exactRecord(value: unknown, keys: readonly string[]): Record<string, unknown> {
  if (value === null || typeof value !== "object" || Array.isArray(value)) return refuse("record is not an object");
  const record = value as Record<string, unknown>;
  if (Object.keys(record).sort().join("\u0000") !== [...keys].sort().join("\u0000")) return refuse("record fields differ from the schema");
  return record;
}

function validIdentity(value: unknown): value is string {
  return typeof value === "string" && value.length > 0 && utf8.encode(value).length <= IDENTITY_MAX_BYTES && value.trim() === value;
}

function validPackageId(value: unknown): value is string {
  if (!validIdentity(value) || !value.startsWith("semio:")) return false;
  const name = value.slice("semio:".length);
  return name.length > 0 && !name.startsWith("-") && !name.endsWith("-") && !name.includes("--") && /^[a-z0-9-]+$/u.test(name);
}

const validDigest = (value: unknown): value is string => typeof value === "string" && DIGEST_PATTERN.test(value);
const validLength = (value: unknown, maximum: number): value is number => typeof value === "number" && Number.isSafeInteger(value) && value >= 1 && value <= maximum;

/** 📛️ `TrustedPluginModulePathV1`: 1–16 segments, none empty, `.`, `..` or URL-significant. */
export function validTrustedPluginModulePathV1(value: unknown): value is string {
  return typeof value === "string" && [...value].length <= PATH_MAX_CHARS && PATH_PATTERN.test(value);
}

function validFile(value: unknown): TrustedPluginModuleFileV1 {
  const record = exactRecord(value, FILE_KEYS);
  if (!validTrustedPluginModulePathV1(record.path) || !validLength(record.byteLength, TRUSTED_PLUGIN_MODULE_FILE_MAX_BYTES) || !validDigest(record.sha256) || !validDigest(record.blake3)) return refuse("file record is invalid");
  return Object.freeze({ path: record.path, byteLength: record.byteLength, sha256: record.sha256, blake3: record.blake3 });
}

/** 🧩️ Checks every rule of one manifest value and binds it to the package it claims to derive from. */
export function validateTrustedPluginModuleBundleV1(value: unknown, source: TrustedPluginModuleSourceV1): TrustedPluginModuleBundleV1 {
  const record = exactRecord(value, BUNDLE_KEYS);
  if (record.schema !== TRUSTED_PLUGIN_MODULE_SCHEMA || !validIdentity(record.pluginId) || !validPackageId(record.packageId) || !validIdentity(record.version) || !validDigest(record.sourceComponentSha256) || !validDigest(record.sourceDescriptorByteSha256))
    return refuse("identity or schema is invalid");
  if (typeof record.moduleDirectory !== "string" || [...record.moduleDirectory].length > DIRECTORY_MAX_CHARS || !DIRECTORY_PATTERN.test(record.moduleDirectory) || !validTrustedPluginModulePathV1(record.entry)) return refuse("directory or entry is not a canonical path");
  if (!Array.isArray(record.files) || record.files.length < FILES_MIN || record.files.length > FILES_MAX) return refuse("file count is out of bounds");
  const files = record.files.map(validFile);
  const directory = `${record.moduleDirectory}/`, vendor = `${MODULE_VENDOR_DIRECTORY}/`;
  if (files.some((file) => !file.path.startsWith(directory) && !file.path.startsWith(vendor))) return refuse("file lies outside its module directory and the vendor root");
  if (files.some((file, index) => index > 0 && utf8OrderV1(files[index - 1]!.path, file.path) >= 0)) return refuse("files are not in strictly ascending byte order");
  const file = (name: string) => files.find((candidate) => candidate.path === `${directory}${name}`);
  if (record.entry !== `${directory}${MODULE_BRIDGE_FILE}` || !file(MODULE_BRIDGE_FILE) || !file(TRUSTED_PLUGIN_MODULE_DESCRIPTOR_JSON_FILE)) return refuse("entry or JSON descriptor is missing");
  if (file(TRUSTED_PLUGIN_MODULE_DESCRIPTOR_PACK_FILE)?.sha256 !== record.sourceDescriptorByteSha256) return refuse("packed descriptor is not the package's own descriptor");
  if (record.pluginId !== source.pluginId || record.packageId !== source.packageId || record.version !== source.version || record.sourceComponentSha256 !== source.componentSha256 || record.sourceDescriptorByteSha256 !== source.descriptorByteSha256)
    return refuse("was not derived from its package");
  return Object.freeze({
    schema: record.schema,
    pluginId: record.pluginId,
    packageId: record.packageId,
    version: record.version,
    sourceComponentSha256: record.sourceComponentSha256,
    sourceDescriptorByteSha256: record.sourceDescriptorByteSha256,
    moduleDirectory: record.moduleDirectory,
    entry: record.entry,
    files: Object.freeze(files),
  });
}

/** 📤️ The canonical manifest bytes: compact JSON in schema field order and one trailing newline. */
export function encodeTrustedPluginModuleBundleV1(bundle: TrustedPluginModuleBundleV1): Uint8Array {
  const ordered = {
    schema: bundle.schema,
    pluginId: bundle.pluginId,
    packageId: bundle.packageId,
    version: bundle.version,
    sourceComponentSha256: bundle.sourceComponentSha256,
    sourceDescriptorByteSha256: bundle.sourceDescriptorByteSha256,
    moduleDirectory: bundle.moduleDirectory,
    entry: bundle.entry,
    files: bundle.files.map((file) => ({ path: file.path, byteLength: file.byteLength, sha256: file.sha256, blake3: file.blake3 })),
  };
  return utf8.encode(`${JSON.stringify(ordered)}\n`);
}

/** 📖️ Decodes one manifest, requiring the exact canonical bytes its own encoding produces. */
export function decodeTrustedPluginModuleBundleV1(bytes: Uint8Array, source: TrustedPluginModuleSourceV1): TrustedPluginModuleBundleV1 {
  if (bytes.byteLength === 0 || bytes.byteLength > TRUSTED_PLUGIN_MODULE_BUNDLE_MAX_BYTES) return refuse("manifest exceeds its bound");
  let value: unknown;
  try {
    value = JSON.parse(new TextDecoder("utf-8", { fatal: true }).decode(bytes));
  } catch {
    return refuse("manifest is not UTF-8 JSON");
  }
  const bundle = validateTrustedPluginModuleBundleV1(value, source);
  const canonical = encodeTrustedPluginModuleBundleV1(bundle);
  if (canonical.byteLength !== bytes.byteLength || canonical.some((byte, index) => byte !== bytes[index])) return refuse("manifest is not canonical");
  return bundle;
}

/** 📇️ Checks every rule of one generation index: canonical entries in strictly ascending plugin order. */
export function validateTrustedPluginModuleIndexV1(value: unknown): TrustedPluginModuleIndexV1 {
  const record = exactRecord(value, INDEX_KEYS);
  if (record.schema !== TRUSTED_PLUGIN_MODULE_INDEX_SCHEMA || !validDigest(record.generationId) || !Array.isArray(record.modules) || record.modules.length > TRUSTED_PLUGIN_MODULE_INDEX_MAX_MODULES) return refuse("index shape is invalid");
  const modules = record.modules.map((candidate): TrustedPluginModuleIndexEntryV1 => {
    const entry = exactRecord(candidate, INDEX_ENTRY_KEYS);
    const dependencies = entry.dependencies;
    const dialects = entry.dialectArtifactKinds;
    if (
      !validIdentity(entry.pluginId) ||
      !validPackageId(entry.packageId) ||
      !validIdentity(entry.version) ||
      !validDigest(entry.componentSha256) ||
      !validDigest(entry.descriptorByteSha256) ||
      !validDigest(entry.bundleSha256) ||
      !validLength(entry.bundleByteLength, TRUSTED_PLUGIN_MODULE_BUNDLE_MAX_BYTES) ||
      !validTrustedPluginModulePathV1(entry.entry) ||
      !Array.isArray(dependencies) ||
      dependencies.length > DEPENDENCIES_MAX ||
      !dependencies.every(validIdentity) ||
      new Set(dependencies).size !== dependencies.length ||
      !Array.isArray(dialects) ||
      dialects.length > DIALECT_KINDS_MAX ||
      !dialects.every(validIdentity) ||
      dialects.some((kind, index) => index > 0 && utf8OrderV1(dialects[index - 1] as string, kind as string) >= 0) ||
      (entry.extendsPluginId !== null && (!validIdentity(entry.extendsPluginId) || !dependencies.includes(entry.extendsPluginId)))
    )
      return refuse("index entry is invalid");
    return Object.freeze({
      pluginId: entry.pluginId,
      packageId: entry.packageId,
      version: entry.version,
      componentSha256: entry.componentSha256,
      descriptorByteSha256: entry.descriptorByteSha256,
      dependencies: Object.freeze([...(dependencies as string[])]),
      dialectArtifactKinds: Object.freeze([...(dialects as string[])]),
      extendsPluginId: entry.extendsPluginId as string | null,
      bundleSha256: entry.bundleSha256,
      bundleByteLength: entry.bundleByteLength,
      entry: entry.entry,
    });
  });
  if (modules.some((entry, index) => index > 0 && utf8OrderV1(modules[index - 1]!.pluginId, entry.pluginId) >= 0)) return refuse("index is not in strictly ascending plugin order");
  return Object.freeze({ schema: record.schema, generationId: record.generationId, modules: Object.freeze(modules) });
}

/** 🪪️ The package an index entry says its module derives from. */
export function trustedPluginModuleSourceOfEntryV1(entry: TrustedPluginModuleIndexEntryV1): TrustedPluginModuleSourceV1 {
  return { pluginId: entry.pluginId, packageId: entry.packageId, version: entry.version, componentSha256: entry.componentSha256, descriptorByteSha256: entry.descriptorByteSha256 };
}

async function sha256Hex(bytes: Uint8Array): Promise<string> {
  const digest = new Uint8Array(await crypto.subtle.digest("SHA-256", bytes as Uint8Array<ArrayBuffer>));
  return Array.from(digest, (byte) => byte.toString(16).padStart(2, "0")).join("");
}

/** 🔐️ Accepts a file's bytes only when their length and both digests are the manifest's own. */
export async function verifyTrustedPluginModuleFileV1(file: TrustedPluginModuleFileV1, bytes: Uint8Array): Promise<boolean> {
  return bytes.byteLength === file.byteLength && (await sha256Hex(bytes)) === file.sha256 && blake3Hex(bytes) === file.blake3;
}

/** 🔐️ SHA-256 of one manifest's bytes, the content address its index entry names. */
export function trustedPluginModuleBundleSha256V1(bytes: Uint8Array): Promise<string> {
  return sha256Hex(bytes);
}

/** 🗄️ The store's constants, read from the sibling schema module (`PluginModuleStoreV1`). */
export const PLUGIN_MODULE_STORE_V1: Readonly<{ name: string; storeRoot: string; serveRoute: string; lockPrefix: string; storeLock: string; programSeparator: string }> = Object.freeze({ ...storeSchemaModule["x-semio-constants"] });
/** 🔁️ The transfer retry policy of one plugin module file, read from the sibling schema module (`PluginModuleTransferRetryV1`). */
export const PLUGIN_MODULE_TRANSFER_RETRY_V1: Readonly<{ transientStatuses: readonly number[]; maxAttempts: number; backoffInitialMs: number; backoffMaxMs: number }> = Object.freeze({ ...storeSchemaModule["x-semio-transfer-retry"] });
/** 🪪️ `HubProgramIdV1`'s pattern and `PluginModuleSourceV1`'s values, read from the sibling schema module. */
export const HUB_PROGRAM_ID_PATTERN = new RegExp(storeSchemaModule.$defs.HubProgramIdV1.pattern, "u");
export const HUB_PROGRAM_ID_MAX_CHARS: number = storeSchemaModule.$defs.HubProgramIdV1.maxLength;
export const PLUGIN_MODULE_SOURCES_V1: readonly string[] = Object.freeze([...storeSchemaModule.$defs.PluginModuleSourceV1.enum]);
const STORE_RECORD_SCHEMA: string = storeSchemaModule.$defs.PluginModuleStoreRecordV1.properties.schema.const;
const STORE_RECORD_KEYS: readonly string[] = storeSchemaModule.$defs.PluginModuleStoreRecordV1.required;
const STORE_RECORD_TIME_MAX: number = storeSchemaModule.$defs.PluginModuleStoreRecordV1.properties.installedAtMs.maximum;

/** 🗄️ One installed plugin module of one catalog generation (`PluginModuleStoreRecordV1`). */
export type PluginModuleStoreRecordV1 = Readonly<{ schema: string; generationId: string; entry: TrustedPluginModuleIndexEntryV1; installedAtMs: number }>;

/** 🗄️ Checks one store record: its generation, its index entry (every index-entry rule) and its install time. */
export function validatePluginModuleStoreRecordV1(value: unknown): PluginModuleStoreRecordV1 {
  const record = exactRecord(value, STORE_RECORD_KEYS);
  if (record.schema !== STORE_RECORD_SCHEMA || !validDigest(record.generationId) || typeof record.installedAtMs !== "number" || !Number.isSafeInteger(record.installedAtMs) || record.installedAtMs < 0 || record.installedAtMs > STORE_RECORD_TIME_MAX)
    return refuse("store record is invalid");
  const index = validateTrustedPluginModuleIndexV1({ schema: TRUSTED_PLUGIN_MODULE_INDEX_SCHEMA, generationId: record.generationId, modules: [record.entry] });
  return Object.freeze({ schema: STORE_RECORD_SCHEMA, generationId: record.generationId, entry: index.modules[0]!, installedAtMs: record.installedAtMs });
}

/** 🗄️ A new store record for one verified index entry of one generation. */
export function pluginModuleStoreRecordV1(generationId: string, entry: TrustedPluginModuleIndexEntryV1, installedAtMs: number): PluginModuleStoreRecordV1 {
  return validatePluginModuleStoreRecordV1({ schema: STORE_RECORD_SCHEMA, generationId, entry, installedAtMs });
}
