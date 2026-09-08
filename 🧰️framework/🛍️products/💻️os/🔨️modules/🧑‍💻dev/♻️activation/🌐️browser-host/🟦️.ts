import { createHash } from "node:crypto";
import { closeSync, existsSync, lstatSync, mkdirSync, openSync, readFileSync, readSync, readdirSync, realpathSync, renameSync, rmSync, writeFileSync } from "node:fs";
import { isAbsolute, join, relative, resolve } from "node:path";
import { ACTIVATION_RECEIPT_FILE, nextActivationReceipt, parseActivationReceipt, publishActivationReceipt, readActivationReceipt } from "../🟦️.ts";

export const TEST_BROWSER_MODULE_ROOT_ENV = "SEMIO_TEST_BROWSER_MODULE_ROOT";
export const TEST_BROWSER_ACTIVATION_ROOT_ENV = "SEMIO_TEST_BROWSER_ACTIVATION_ROOT";
export const TEST_BROWSER_HOST_RECEIPT_ENV = "SEMIO_TEST_BROWSER_HOST_RECEIPT";
export const TEST_BROWSER_HOST_RECEIPT_FILE = "🔣️receipt.json";

const DIGEST = /^[0-9a-f]{64}$/u;
const MODULE_DIRECTORIES = ["🪞️vendor", "🧵️shard", "🪐️space", "🌍️gis"] as const;
const MODULE_FILES_MAX = 512;
const MODULE_BYTES_MAX = 768 * 1024 * 1024;
const HOST_COMPONENT_MAX_BYTES = 64 * 1024 * 1024;
const HOST_DESCRIPTOR_JSON_MAX_BYTES = 4 * 1024 * 1024;

export type TestBrowserHostStagingReceiptV1 = Readonly<{
  schema: "semio.os.test-browser-host-staging/v1";
  variant: "s";
  profile: "dev";
  moduleSetSha256: string;
  activationReceiptSha256: string;
  host: Readonly<{ pluginId: "space"; componentByteLength: number; componentSha256: string; coreSha256: string; descriptorSha256: string }>;
  selectedGis: Readonly<{ generationId: string; currentSha256: string }>;
}>;

export type TestBrowserHostRootsV1 = Readonly<{
  artifactRoot: string;
  browserHostRoot: string;
  moduleRoot: string;
  activationRoot: string;
  receiptPath: string;
  receipt: TestBrowserHostStagingReceiptV1;
}>;

type Environment = Readonly<Record<string, string | undefined>>;
type FileEntry = Readonly<{ relativePath: string; path: string; size: number; sha256: string }>;

function exactKeys(value: object, keys: readonly string[]): boolean {
  return JSON.stringify(Object.keys(value).sort()) === JSON.stringify([...keys].sort());
}

function record(value: unknown): value is Record<string, unknown> {
  return value !== null && typeof value === "object" && !Array.isArray(value);
}

function digest(value: unknown): value is string {
  return typeof value === "string" && DIGEST.test(value);
}

/** 🧾️ Parses the closed browser-host receipt without admitting extra authority fields. */
export function parseTestBrowserHostStagingReceiptV1(value: unknown): TestBrowserHostStagingReceiptV1 {
  if (!record(value) || !exactKeys(value, ["schema", "variant", "profile", "moduleSetSha256", "activationReceiptSha256", "host", "selectedGis"])) throw new Error("Invalid test browser-host receipt");
  const row = value;
  const host = record(row.host) ? row.host : undefined;
  const selectedGis = record(row.selectedGis) ? row.selectedGis : undefined;
  if (
    row.schema !== "semio.os.test-browser-host-staging/v1" ||
    row.variant !== "s" ||
    row.profile !== "dev" ||
    !digest(row.moduleSetSha256) ||
    !digest(row.activationReceiptSha256) ||
    !host || !exactKeys(host, ["pluginId", "componentByteLength", "componentSha256", "coreSha256", "descriptorSha256"]) ||
    host.pluginId !== "space" || typeof host.componentByteLength !== "number" || !Number.isSafeInteger(host.componentByteLength) || host.componentByteLength < 1 || host.componentByteLength > HOST_COMPONENT_MAX_BYTES ||
    !digest(host.componentSha256) || !digest(host.coreSha256) || !digest(host.descriptorSha256) ||
    !selectedGis || !exactKeys(selectedGis, ["generationId", "currentSha256"]) ||
    !digest(selectedGis.generationId) || !digest(selectedGis.currentSha256)
  ) throw new Error("Invalid test browser-host receipt fields");
  return Object.freeze({
    schema: "semio.os.test-browser-host-staging/v1",
    variant: "s",
    profile: "dev",
    moduleSetSha256: row.moduleSetSha256,
    activationReceiptSha256: row.activationReceiptSha256,
    host: Object.freeze({ pluginId: "space", componentByteLength: host.componentByteLength, componentSha256: host.componentSha256, coreSha256: host.coreSha256, descriptorSha256: host.descriptorSha256 }),
    selectedGis: Object.freeze({ generationId: selectedGis.generationId, currentSha256: selectedGis.currentSha256 }),
  });
}

function sha256File(path: string): string {
  const hash = createHash("sha256");
  const descriptor = openSync(path, "r");
  const chunk = Buffer.allocUnsafe(64 * 1024);
  try {
    while (true) {
      const count = readSync(descriptor, chunk, 0, chunk.byteLength, null);
      if (count === 0) break;
      hash.update(chunk.subarray(0, count));
    }
  } finally {
    chunk.fill(0);
    closeSync(descriptor);
  }
  return hash.digest("hex");
}

function regularDirectory(path: string, label: string): string {
  const info = lstatSync(path);
  if (!info.isDirectory() || info.isSymbolicLink()) throw new Error(`${label} is not an owned regular directory`);
  return realpathSync(path);
}

function regularFile(path: string, label: string): string {
  const info = lstatSync(path);
  if (!info.isFile() || info.isSymbolicLink()) throw new Error(`${label} is not an owned regular file`);
  return realpathSync(path);
}

function ownedChild(root: string, child: string, label: string): void {
  const path = relative(root, child);
  if (!path || path.startsWith("..") || isAbsolute(path)) throw new Error(`${label} is outside its ticket owner`);
}

function moduleEntries(moduleRoot: string): readonly FileEntry[] {
  const entries: FileEntry[] = [];
  let total = 0;
  const walk = (directory: string): void => {
    for (const entry of readdirSync(directory, { withFileTypes: true }).sort((left, right) => Buffer.from(left.name).compare(Buffer.from(right.name)))) {
      const path = join(directory, entry.name);
      if (entry.isSymbolicLink()) throw new Error("Test browser-host module tree contains a symlink");
      if (entry.isDirectory()) walk(path);
      else if (entry.isFile()) {
        const info = lstatSync(path);
        total += info.size;
        if (entries.length >= MODULE_FILES_MAX || total > MODULE_BYTES_MAX) throw new Error("Test browser-host module tree exceeds its bound");
        entries.push(Object.freeze({ relativePath: relative(moduleRoot, path).replaceAll("\\", "/"), path, size: info.size, sha256: sha256File(path) }));
      } else throw new Error("Test browser-host module tree contains an unsupported node");
    }
  };
  walk(moduleRoot);
  return Object.freeze(entries.sort((left, right) => Buffer.from(left.relativePath).compare(Buffer.from(right.relativePath))));
}

function entriesDigest(entries: readonly FileEntry[], prefix = ""): string {
  const hash = createHash("sha256");
  for (const entry of entries) hash.update(`${JSON.stringify([prefix + entry.relativePath, entry.size, entry.sha256])}\n`);
  return hash.digest("hex");
}

function moduleSubset(entries: readonly FileEntry[], directory: string): readonly FileEntry[] {
  const prefix = `${directory}/`;
  return entries.filter((entry) => entry.relativePath.startsWith(prefix)).map((entry) => Object.freeze({ ...entry, relativePath: entry.relativePath.slice(prefix.length) }));
}

function exactModuleEntries(moduleRoot: string): readonly FileEntry[] {
  const names = readdirSync(moduleRoot, { withFileTypes: true });
  if (names.some((entry) => !entry.isDirectory() || !(MODULE_DIRECTORIES as readonly string[]).includes(entry.name)) || names.length !== MODULE_DIRECTORIES.length) throw new Error("Test browser-host module set is not exact");
  const entries = moduleEntries(moduleRoot);
  for (const directory of MODULE_DIRECTORIES) if (!entries.some((entry) => entry.relativePath.startsWith(`${directory}/`))) throw new Error(`Test browser-host module ${directory} is empty`);
  return entries;
}

function onlyCoreComponent(entries: readonly FileEntry[]): FileEntry {
  const components = entries.filter((entry) => entry.relativePath.endsWith("_component.core.wasm"));
  if (components.length !== 1) throw new Error("Test browser host must contain one exact core component");
  return components[0]!;
}

function hostDescriptorComponentHashes(entries: readonly FileEntry[]): Readonly<{ componentSha256: string; coreSha256: string }> {
  const json = entries.find((entry) => entry.relativePath === "🔣️.json");
  if (!json || json.size > HOST_DESCRIPTOR_JSON_MAX_BYTES) throw new Error("Test browser host descriptor JSON is absent or exceeds its bound");
  const value: unknown = JSON.parse(readFileSync(json.path, "utf8"));
  const hashes = record(value) && record(value.hashes) ? value.hashes : undefined;
  if (!hashes || !digest(hashes.wasmSha256) || !digest(hashes.coreWasmSha256)) throw new Error("Test browser host descriptor omits its component identities");
  return Object.freeze({ componentSha256: hashes.wasmSha256, coreSha256: hashes.coreWasmSha256 });
}

/** 🔐 Closes an already atomically materialized Space/GIS module set with an exact activation and receipt. */
export function closeTestBrowserHostStagingV1(
  artifactRootInput: string,
  selectedGis: Readonly<{ generationId: string; currentSha256: string }>,
  hostComponent: Readonly<{ byteLength: number; sha256: string }>,
): TestBrowserHostRootsV1 {
  if (!digest(selectedGis.generationId) || !digest(selectedGis.currentSha256)) throw new Error("Invalid selected GIS current identity");
  if (!Number.isSafeInteger(hostComponent.byteLength) || hostComponent.byteLength < 1 || hostComponent.byteLength > HOST_COMPONENT_MAX_BYTES || !digest(hostComponent.sha256)) throw new Error("Invalid test browser host component identity");
  if (!isAbsolute(artifactRootInput)) throw new Error("Test browser-host owner must be absolute");
  const artifactRoot = regularDirectory(artifactRootInput, "Test artifact root");
  if (!artifactRoot.split(/[\\/]/u).includes("🗑️generated")) throw new Error("Test browser-host owner is not ticket generated");
  const browserHostRoot = join(artifactRoot, "browser-host");
  const moduleRoot = regularDirectory(join(browserHostRoot, "modules"), "Test browser module root");
  const activationRoot = join(browserHostRoot, "activation");
  mkdirSync(activationRoot, { recursive: true, mode: 0o700 });
  regularDirectory(activationRoot, "Test browser activation root");
  const entries = exactModuleEntries(moduleRoot);
  const hostEntries = moduleSubset(entries, "🪐️space");
  const component = onlyCoreComponent(hostEntries);
  const descriptor = hostEntries.find((entry) => entry.relativePath === "🛂️.descriptor.semio");
  const descriptorHashes = hostDescriptorComponentHashes(hostEntries);
  if (!descriptor) throw new Error("Test browser host descriptor is absent");
  if (descriptorHashes.componentSha256 !== hostComponent.sha256 || descriptorHashes.coreSha256 !== component.sha256) throw new Error("Test browser host descriptor component identity differs from its materialized owner");
  const supportEntries = [...moduleSubset(entries, "🪞️vendor"), ...moduleSubset(entries, "🧵️shard")];
  const supportSha256 = entriesDigest(supportEntries, "support/");
  const plugins = (["gis", "space"] as const).map((pluginId) => {
    const directory = pluginId === "space" ? "🪐️space" : "🌍️gis";
    const moduleSha256 = entriesDigest(moduleSubset(entries, directory), `${pluginId}/`);
    return Object.freeze({ pluginId, artifactSha256: createHash("sha256").update(supportSha256 + moduleSha256).digest("hex") });
  });
  const previous = existsSync(join(activationRoot, ACTIVATION_RECEIPT_FILE)) ? readActivationReceipt(activationRoot) : undefined;
  publishActivationReceipt(activationRoot, nextActivationReceipt("s", "dev", plugins, previous));
  const activationPath = regularFile(join(activationRoot, ACTIVATION_RECEIPT_FILE), "Test browser activation receipt");
  const activationReceiptSha256 = sha256File(activationPath);
  const receipt = parseTestBrowserHostStagingReceiptV1({
    schema: "semio.os.test-browser-host-staging/v1",
    variant: "s",
    profile: "dev",
    moduleSetSha256: entriesDigest(entries),
    activationReceiptSha256,
    host: { pluginId: "space", componentByteLength: hostComponent.byteLength, componentSha256: hostComponent.sha256, coreSha256: component.sha256, descriptorSha256: descriptor.sha256 },
    selectedGis,
  });
  const receiptPath = join(browserHostRoot, TEST_BROWSER_HOST_RECEIPT_FILE);
  const temporary = join(browserHostRoot, `.browser-host-receipt-${process.pid}.stage`);
  try {
    writeFileSync(temporary, `${JSON.stringify(receipt)}\n`, { flag: "wx", mode: 0o600 });
    renameSync(temporary, receiptPath);
  } finally {
    rmSync(temporary, { force: true });
  }
  return resolveTestBrowserHostRootsV1({
    SEMIO_TEST_ARTIFACT_DIR: artifactRoot,
    [TEST_BROWSER_MODULE_ROOT_ENV]: moduleRoot,
    [TEST_BROWSER_ACTIVATION_ROOT_ENV]: activationRoot,
    [TEST_BROWSER_HOST_RECEIPT_ENV]: receiptPath,
  })!;
}

/** 🛂 Revalidates every served byte before a test-only Vite host may start. */
export function resolveTestBrowserHostRootsV1(environment: Environment): TestBrowserHostRootsV1 | null {
  const values = [environment[TEST_BROWSER_MODULE_ROOT_ENV], environment[TEST_BROWSER_ACTIVATION_ROOT_ENV], environment[TEST_BROWSER_HOST_RECEIPT_ENV]];
  if (values.every((value) => value === undefined)) return null;
  if (values.some((value) => value === undefined) || !environment.SEMIO_TEST_ARTIFACT_DIR) throw new Error("Test browser-host roots require one complete ticket authority");
  const artifactRoot = regularDirectory(environment.SEMIO_TEST_ARTIFACT_DIR, "Test artifact root");
  if (!isAbsolute(environment.SEMIO_TEST_ARTIFACT_DIR) || !artifactRoot.split(/[\\/]/u).includes("🗑️generated")) throw new Error("Test browser-host artifact root is not ticket owned");
  const browserHostRoot = regularDirectory(join(artifactRoot, "browser-host"), "Test browser-host root");
  const moduleRoot = regularDirectory(values[0]!, "Test browser module root");
  const activationRoot = regularDirectory(values[1]!, "Test browser activation root");
  const receiptPath = regularFile(values[2]!, "Test browser-host receipt");
  for (const [path, label] of [[moduleRoot, "module root"], [activationRoot, "activation root"], [receiptPath, "receipt"]] as const) ownedChild(artifactRoot, path, label);
  if (moduleRoot !== join(browserHostRoot, "modules") || activationRoot !== join(browserHostRoot, "activation") || receiptPath !== join(browserHostRoot, TEST_BROWSER_HOST_RECEIPT_FILE)) throw new Error("Test browser-host paths do not match their fixed layout");
  const receipt = parseTestBrowserHostStagingReceiptV1(JSON.parse(readFileSync(receiptPath, "utf8")));
  const entries = exactModuleEntries(moduleRoot);
  if (entriesDigest(entries) !== receipt.moduleSetSha256) throw new Error("Test browser-host module set changed after closure");
  const activationPath = regularFile(join(activationRoot, ACTIVATION_RECEIPT_FILE), "Test browser activation receipt");
  if (sha256File(activationPath) !== receipt.activationReceiptSha256) throw new Error("Test browser-host activation changed after closure");
  const activation = parseActivationReceipt(readActivationReceipt(activationRoot));
  if (activation.variant !== "s" || activation.profile !== "dev" || JSON.stringify(activation.plugins.map((row) => row.pluginId)) !== JSON.stringify(["gis", "space"])) throw new Error("Test browser-host activation is not the exact session");
  const hostEntries = moduleSubset(entries, "🪐️space");
  const descriptorHashes = hostDescriptorComponentHashes(hostEntries);
  if (descriptorHashes.componentSha256 !== receipt.host.componentSha256 || descriptorHashes.coreSha256 !== receipt.host.coreSha256 || onlyCoreComponent(hostEntries).sha256 !== receipt.host.coreSha256 || hostEntries.find((entry) => entry.relativePath === "🛂️.descriptor.semio")?.sha256 !== receipt.host.descriptorSha256) throw new Error("Test browser host identity changed after closure");
  return Object.freeze({ artifactRoot, browserHostRoot, moduleRoot, activationRoot, receiptPath, receipt });
}

/** 🧪 Creates only the fixed directory skeleton; component materializers remain its exclusive writers. */
export function prepareTestBrowserHostRootsV1(artifactRootInput: string): Readonly<{ browserHostRoot: string; moduleRoot: string; activationRoot: string; receiptPath: string }> {
  if (!isAbsolute(artifactRootInput)) throw new Error("Test browser-host owner must be absolute");
  const artifactRoot = regularDirectory(artifactRootInput, "Test artifact root");
  if (!artifactRoot.split(/[\\/]/u).includes("🗑️generated")) throw new Error("Test browser-host owner is not ticket generated");
  const browserHostRoot = join(artifactRoot, "browser-host");
  const moduleRoot = join(browserHostRoot, "modules");
  const activationRoot = join(browserHostRoot, "activation");
  const extensionRoot = join(browserHostRoot, "extensions");
  mkdirSync(moduleRoot, { recursive: true, mode: 0o700 });
  mkdirSync(activationRoot, { recursive: true, mode: 0o700 });
  mkdirSync(extensionRoot, { recursive: true, mode: 0o700 });
  return Object.freeze({ browserHostRoot, moduleRoot, activationRoot, receiptPath: join(browserHostRoot, TEST_BROWSER_HOST_RECEIPT_FILE) });
}
