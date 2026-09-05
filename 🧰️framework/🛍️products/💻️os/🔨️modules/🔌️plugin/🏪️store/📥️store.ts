#!/usr/bin/env bun
/** @emoji 🏪 Runtime-installable extension store — unpack `.semio` packages, materialize for native/web, dev-server install + SSE. */
import { createHash } from "node:crypto";
import { existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync, watch, writeFileSync } from "node:fs";
import { basename, dirname, join } from "node:path";
import { tmpdir } from "node:os";
import { decodePackValue, encodePackValue } from "@semio-tech/framework-os";
import { decodeOwnedZip, encodeOwnedZip } from "./🟦️.ts";
import { installationDirectoryCollision, installationDirectoryEmoji } from "../../🧩️extension/🟦️.ts";
import { MODULE_BRIDGE_FILE, MODULE_EXTENSION_ROUTE, moduleRoutePath } from "../📇️registry/📦️deployment/🟦️.ts";
import {
  PLUGIN_HOST_SHIM_FILE,
  PREVIEW2_VENDOR_RELATIVE,
  SHARD_WORKER_FILE,
  ensurePreview2ShimVendorAt,
  hostShimSource,
  pluginComponentBridgeSource,
  shardWorkerSource,
  transpilePluginComponent,
  type PluginWebMaterializeContext,
} from "../📦️packages/🟦️typescript/🟦️.ts";

//#region 🔖️Constants
export const EXTENSION_STATIC_ROUTE = MODULE_EXTENSION_ROUTE;
export const EXTENSION_INSTALL_PATH = `${EXTENSION_STATIC_ROUTE}/install`;
export const EXTENSION_WATCH_PATH = `${EXTENSION_STATIC_ROUTE}/watch`;
export const EXTENSION_WATCH_MARKER = "👀️extension-watch.json";
export const EXTENSION_INSTALL_META = "📥️install.json";
export const EXTENSION_COMPONENT_FILE = "component.wasm";
export const EXTENSION_MANIFEST_ZIP_ENTRY = "manifest.semio";
export const EXTENSION_MANIFEST_ZIP_ENTRY_EMOJI = "🛂️manifest.semio";
export const EXTENSION_PACKAGE_FORMAT = 1;
export const EXTENSION_PACKAGE_ENVELOPE_TOKEN = "os.extension.pack v1";

const SEMIO_BINARY_MAGIC = new Uint8Array([0x89, 0x53, 0x45, 0x4d, 0x0d, 0x0a, 0x1a, 0x0a]);
const FOLDER_WATCH_DEBOUNCE_MS = 200;
//#endregion 🔖️Constants

//#region 🔖️Types
export type ExtensionManifestRecord = {
  readonly extensionId: string;
  readonly directoryName: string;
  readonly label: string;
  readonly version: string;
  readonly extends: string;
  readonly capabilities?: readonly unknown[];
  readonly contributions?: readonly unknown[];
};

/** @emoji 📦 On-disk `.sxt` manifest (`🛂️manifest.semio` inside the zip), camelCase JSON matching `ExtensionPackageManifest`. */
export type ExtensionPackageManifestRecord = {
  readonly extensionId: string;
  readonly directoryName: string;
  readonly label: string;
  readonly version: string;
  readonly extends: string;
  readonly capabilities: readonly string[];
  readonly contributions: unknown;
  readonly packageFormat: number;
};

export type InstalledExtensionRecord = {
  readonly extensionId: string;
  readonly directoryName: string;
  readonly version: string;
  readonly label: string;
  readonly extends: string;
  readonly moduleUrl: string;
  readonly packageHash: string;
  readonly installedAt: number;
};

export type ExtensionInstallResult = {
  readonly extensionId: string;
  readonly version: string;
  readonly moduleUrl: string;
};

export type ExtensionMaterializeInput = {
  readonly extensionId: string;
  readonly directoryName: string;
  readonly version: string;
  readonly wasmBytes: Uint8Array;
  readonly assets: ReadonlyMap<string, Uint8Array>;
  readonly outDir: string;
  readonly materializeCtx: PluginWebMaterializeContext;
};

export type ExtensionMaterializer = (input: ExtensionMaterializeInput) => Promise<{ readonly moduleUrl: string }>;

export type ExtensionSourceEvent =
  | { readonly kind: "snapshot"; readonly extensions: readonly InstalledExtensionRecord[] }
  | { readonly kind: "installed"; readonly extensionId: string; readonly version: string; readonly installedAt: number }
  | { readonly kind: "uninstalled"; readonly extensionId: string };

export type ExtensionStore = {
  readonly installRoot: string;
  installFromBytes(bytes: Uint8Array): Promise<ExtensionInstallResult>;
  installFromUrl(url: string): Promise<ExtensionInstallResult>;
  uninstall(extensionId: string): Promise<void>;
  listInstalled(): Promise<readonly InstalledExtensionRecord[]>;
};

type BackboneServerRequest = { readonly url?: string; readonly method?: string; readonly on: (event: string, listener: () => void) => void };
type BackboneServerResponse = {
  statusCode: number;
  setHeader(name: string, value: string): void;
  write(chunk: string): void;
  end(chunk?: string): void;
};
//#endregion 🔖️Types

//#region 🔖️Package
function packageContentHash(bytes: Uint8Array): string {
  return createHash("sha256").update(bytes).digest("hex");
}

function unwrapSemioEnvelope(bytes: Uint8Array): Uint8Array {
  if (bytes.length < 12 || !SEMIO_BINARY_MAGIC.every((value, index) => bytes[index] === value)) {
    return bytes;
  }
  const tokenLen = new DataView(bytes.buffer, bytes.byteOffset + 8, 4).getUint32(0, true);
  const payloadStart = 12 + tokenLen;
  if (payloadStart > bytes.length) throw new Error("truncated semio extension package envelope");
  return bytes.subarray(payloadStart);
}

/** @emoji 📨 Wraps deflate zip bytes in the Wave-1.A semio binary envelope (`os.extension.pack v1`). */
export function wrapExtensionPackageEnvelope(zipBytes: Uint8Array): Uint8Array {
  const tokenBytes = new TextEncoder().encode(EXTENSION_PACKAGE_ENVELOPE_TOKEN);
  const out = new Uint8Array(SEMIO_BINARY_MAGIC.length + 4 + tokenBytes.length + zipBytes.length);
  out.set(SEMIO_BINARY_MAGIC, 0);
  new DataView(out.buffer, out.byteOffset + 8, 4).setUint32(0, tokenBytes.length, true);
  out.set(tokenBytes, 12);
  out.set(zipBytes, 12 + tokenBytes.length);
  return out;
}

function buildExtensionZipPayload(manifest: ExtensionPackageManifestRecord, componentWasm: Uint8Array, assets: ReadonlyMap<string, Uint8Array>): Uint8Array {
  installationDirectoryEmoji(manifest.directoryName);
  if (componentWasm.length === 0) throw new Error("extension component.wasm is empty");
  if (manifest.packageFormat !== EXTENSION_PACKAGE_FORMAT) throw new Error(`invalid extension package format ${manifest.packageFormat}`);
  const manifestBytes = encodePackValue(manifest);
  const files = new Map<string, Uint8Array>([
    [EXTENSION_MANIFEST_ZIP_ENTRY_EMOJI, manifestBytes],
    [EXTENSION_COMPONENT_FILE, componentWasm],
  ]);
  const assetNames = [...assets.keys()].sort();
  for (const name of assetNames) {
    const payload = assets.get(name);
    if (!payload) continue;
    files.set(name.startsWith("assets/") ? name : `assets/${name}`, payload);
  }
  return encodeOwnedZip(files);
}

/** @emoji 📦 Packs manifest + wasip2 component bytes into a `.sxt` stream (semio envelope + deterministic deflate zip). */
export function packExtensionPackage(input: { readonly manifest: ExtensionPackageManifestRecord; readonly componentWasm: Uint8Array; readonly assets?: ReadonlyMap<string, Uint8Array> }): Uint8Array {
  const zipBytes = buildExtensionZipPayload(input.manifest, input.componentWasm, input.assets ?? new Map());
  return wrapExtensionPackageEnvelope(zipBytes);
}

/** @emoji 🔓️ SHA-256 hex digest of the full `.sxt` bytes (matches install-store dedup). */
export function extensionPackageContentHash(bytes: Uint8Array): string {
  return packageContentHash(bytes);
}

function zipEntryBytes(files: ReadonlyMap<string, Uint8Array>, predicate: (name: string) => boolean): Uint8Array | undefined {
  for (const [name, payload] of files) if (predicate(name)) return payload;
  return undefined;
}

function decodeExtensionManifest(manifestBytes: Uint8Array): ExtensionManifestRecord {
  let decoded: unknown;
  try {
    decoded = decodePackValue(manifestBytes);
  } catch {
    decoded = JSON.parse(new TextDecoder().decode(manifestBytes));
  }
  if (!decoded || typeof decoded !== "object") throw new Error("extension manifest is not a pack object");
  const row = decoded as Record<string, unknown>;
  const extensionId = row.extensionId;
  installationDirectoryEmoji(row.directoryName);
  const label = row.label;
  const version = row.version;
  const extendsHost = row.extends;
  if (typeof extensionId !== "string" || !extensionId) throw new Error("extension manifest missing extensionId");
  if (typeof label !== "string") throw new Error("extension manifest missing label");
  if (typeof version !== "string" || !version) throw new Error("extension manifest missing version");
  return {
    extensionId,
    directoryName: row.directoryName as string,
    label,
    version,
    extends: typeof extendsHost === "string" ? extendsHost : "",
    capabilities: Array.isArray(row.capabilities) ? row.capabilities : undefined,
    contributions: Array.isArray(row.contributions) ? row.contributions : undefined,
  };
}

/** @emoji 📦 Unpacks a Wave-1.A extension package (semio envelope + deflate zip) into wasm bytes, manifest, and optional assets. */
export function unpackExtensionPackage(bytes: Uint8Array): {
  readonly manifest: ExtensionManifestRecord;
  readonly wasmBytes: Uint8Array;
  readonly assets: Map<string, Uint8Array>;
  readonly packageHash: string;
} {
  const packageHash = packageContentHash(bytes);
  const zipBytes = unwrapSemioEnvelope(bytes);
  const files = decodeOwnedZip(zipBytes);
  const manifestBytes =
    zipEntryBytes(files, (name) => name === EXTENSION_MANIFEST_ZIP_ENTRY_EMOJI || name.endsWith(EXTENSION_MANIFEST_ZIP_ENTRY)) ??
    (() => {
      throw new Error(`extension package missing ${EXTENSION_MANIFEST_ZIP_ENTRY}`);
    })();
  const wasmBytes =
    zipEntryBytes(files, (name) => name === EXTENSION_COMPONENT_FILE || name.endsWith(`/${EXTENSION_COMPONENT_FILE}`)) ??
    (() => {
      throw new Error(`extension package missing ${EXTENSION_COMPONENT_FILE}`);
    })();
  const assets = new Map<string, Uint8Array>();
  for (const [name, payload] of files) {
    if (name === EXTENSION_COMPONENT_FILE || name.endsWith(EXTENSION_MANIFEST_ZIP_ENTRY) || name.endsWith(EXTENSION_MANIFEST_ZIP_ENTRY_EMOJI)) continue;
    if (name.startsWith("assets/")) assets.set(name.slice("assets/".length), payload);
  }
  return { manifest: decodeExtensionManifest(manifestBytes), wasmBytes, assets, packageHash };
}
//#endregion 🔖️Package

//#region 🔖️Materializers
/** @emoji 🦀 Native host materializer — keeps raw `component.wasm` on disk for wasmtime `Component::from_binary`. */
export const nativeMaterialize: ExtensionMaterializer = async ({ wasmBytes, outDir, directoryName }) => {
  installationDirectoryEmoji(directoryName);
  mkdirSync(outDir, { recursive: true });
  writeFileSync(join(outDir, EXTENSION_COMPONENT_FILE), wasmBytes);
  return { moduleUrl: `${EXTENSION_STATIC_ROUTE}/${directoryName}/${EXTENSION_COMPONENT_FILE}` };
};

/** @emoji 🌐 Web materializer — jco transpile + bridge (see `🟦️.ts`). */
export const webMaterialize: ExtensionMaterializer = async ({ wasmBytes, assets, outDir, directoryName, materializeCtx }) => {
  installationDirectoryEmoji(directoryName);
  mkdirSync(outDir, { recursive: true });
  writeFileSync(join(outDir, EXTENSION_COMPONENT_FILE), wasmBytes);
  for (const [rel, payload] of assets) {
    const assetPath = join(outDir, "assets", rel);
    mkdirSync(dirname(assetPath), { recursive: true });
    writeFileSync(assetPath, payload);
  }
  ensurePreview2ShimVendorAt(materializeCtx.preview2VendorDir, materializeCtx.repoRoot);
  const jsBase = EXTENSION_COMPONENT_FILE.replace(/\.wasm$/, "");
  const componentBase = `${jsBase}_component`;
  const artifactDir = mkdtempSync(join(tmpdir(), "semio-ext-jco-"));
  const artifactPath = join(artifactDir, EXTENSION_COMPONENT_FILE);
  try {
    writeFileSync(artifactPath, wasmBytes);
    transpilePluginComponent(artifactPath, outDir, componentBase, materializeCtx);
    writeFileSync(join(outDir, PLUGIN_HOST_SHIM_FILE), hostShimSource());
    writeFileSync(join(outDir, SHARD_WORKER_FILE), shardWorkerSource());
    writeFileSync(join(outDir, MODULE_BRIDGE_FILE), pluginComponentBridgeSource(componentBase, EXTENSION_COMPONENT_FILE));
  } finally {
    rmSync(artifactDir, { recursive: true, force: true });
  }
  return { moduleUrl: `${EXTENSION_STATIC_ROUTE}/${directoryName}/${MODULE_BRIDGE_FILE}` };
};
//#endregion 🔖️Materializers

//#region 🔖️Store
function readInstallMeta(dir: string): InstalledExtensionRecord | undefined {
  const metaPath = join(dir, EXTENSION_INSTALL_META);
  if (!existsSync(metaPath)) return undefined;
  if (lstatSync(dir).isSymbolicLink() || lstatSync(metaPath).isSymbolicLink()) throw new Error("Extension installation metadata must not follow a symlink");
  const meta = JSON.parse(readFileSync(metaPath, "utf8")) as InstalledExtensionRecord;
  installationDirectoryEmoji(meta.directoryName);
  if (meta.directoryName !== basename(dir) || typeof meta.extensionId !== "string" || !meta.extensionId) throw new Error("Extension installation metadata identity mismatch");
  return meta;
}

function scanInstalledExtensions(installRoot: string): InstalledExtensionRecord[] {
  if (!existsSync(installRoot)) return [];
  const rows: InstalledExtensionRecord[] = [];
  for (const entry of readdirSync(installRoot, { withFileTypes: true })) {
    if (!entry.isDirectory() || entry.name.startsWith("_") || entry.name.startsWith(".")) continue;
    const meta = readInstallMeta(join(installRoot, entry.name));
    if (meta) rows.push(meta);
  }
  rows.sort((a, b) => a.extensionId.localeCompare(b.extensionId));
  return rows;
}

function writeWatchMarker(installRoot: string, event: ExtensionSourceEvent): void {
  mkdirSync(installRoot, { recursive: true });
  writeFileSync(join(installRoot, EXTENSION_WATCH_MARKER), `${JSON.stringify({ ...event, emittedAt: Date.now() })}\n`);
}

/** @emoji 🏪 Creates an extension store rooted at `installRoot`, using `materializer` for browser or native layouts. */

export function createExtensionStore(options: { readonly installRoot: string; readonly repoRoot: string; readonly materializer: ExtensionMaterializer }): ExtensionStore {
  const { installRoot, repoRoot, materializer } = options;
  const preview2VendorDir = join(installRoot, PREVIEW2_VENDOR_RELATIVE);
  const materializeCtx: PluginWebMaterializeContext = { repoRoot, preview2VendorDir };

  async function materializeInstalled(manifest: ExtensionManifestRecord, wasmBytes: Uint8Array, assets: Map<string, Uint8Array>, packageHash: string): Promise<ExtensionInstallResult> {
    installationDirectoryEmoji(manifest.directoryName);
    if (existsSync(installRoot) && lstatSync(installRoot).isSymbolicLink()) throw new Error("Extension install root must not be a symlink");
    const outDir = join(installRoot, manifest.directoryName);
    const existing = existsSync(outDir) ? readInstallMeta(outDir) : undefined;
    if (existsSync(outDir) && existing?.extensionId !== manifest.extensionId) throw new Error("Extension directory is not owned by this public identity");
    const siblings = existsSync(installRoot) ? readdirSync(installRoot).filter((name) => name !== manifest.directoryName) : [];
    if (installationDirectoryCollision(manifest.directoryName, siblings)) throw new Error("Extension directory conflicts with a sibling emoji");
    if (scanInstalledExtensions(installRoot).some((entry) => entry.extensionId === manifest.extensionId && entry.directoryName !== manifest.directoryName)) throw new Error("Extension identity already owns a different directory");
    if (existing) rmSync(outDir, { recursive: true, force: true });
    mkdirSync(outDir, { recursive: true });
    const { moduleUrl } = await materializer({ extensionId: manifest.extensionId, directoryName: manifest.directoryName, version: manifest.version, wasmBytes, assets, outDir, materializeCtx });
    const installedAt = Date.now();
    const record: InstalledExtensionRecord = {
      extensionId: manifest.extensionId,
      directoryName: manifest.directoryName,
      version: manifest.version,
      label: manifest.label,
      extends: manifest.extends,
      moduleUrl,
      packageHash,
      installedAt,
    };
    writeFileSync(join(outDir, EXTENSION_INSTALL_META), `${JSON.stringify(record, null, 2)}\n`);
    writeWatchMarker(installRoot, { kind: "installed", extensionId: manifest.extensionId, version: manifest.version, installedAt });
    console.log(`[DEBUG] extension store installed ${manifest.extensionId}@${manifest.version} -> ${moduleUrl}`);
    return { extensionId: manifest.extensionId, version: manifest.version, moduleUrl };
  }

  return {
    installRoot,
    async installFromBytes(bytes) {
      const unpacked = unpackExtensionPackage(bytes);
      return materializeInstalled(unpacked.manifest, unpacked.wasmBytes, unpacked.assets, unpacked.packageHash);
    },
    async installFromUrl(url) {
      const response = await fetch(url);
      if (!response.ok) throw new Error(`extension download failed (${response.status}) for ${url}`);
      const bytes = new Uint8Array(await response.arrayBuffer());
      return this.installFromBytes(bytes);
    },
    async uninstall(extensionId) {
      if (existsSync(installRoot) && lstatSync(installRoot).isSymbolicLink()) throw new Error("Extension install root must not be a symlink");
      const installed = scanInstalledExtensions(installRoot).filter((entry) => entry.extensionId === extensionId);
      if (installed.length > 1) throw new Error("Extension identity owns multiple directories");
      if (installed[0]) rmSync(join(installRoot, installed[0].directoryName), { recursive: true, force: true });
      writeWatchMarker(installRoot, { kind: "uninstalled", extensionId });
      console.log(`[DEBUG] extension store uninstalled ${extensionId}`);
    },
    async listInstalled() {
      return scanInstalledExtensions(installRoot);
    },
  };
}

/** @emoji 🗂️ Default dev install directory beside `🔌️plugin-modules`. */
export function defaultExtensionInstallRoot(repoRoot: string): string {
  return join(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧩️extension-modules");
}
//#endregion 🔖️Store

function readRequestBody(req: { on(event: string, listener: (...args: unknown[]) => void): void }): Promise<Buffer> {
  return new Promise((resolve, reject) => {
    const chunks: Buffer[] = [];
    req.on("data", (chunk) => chunks.push(Buffer.from(chunk as Uint8Array)));
    req.on("end", () => resolve(Buffer.concat(chunks)));
    req.on("error", (error) => reject(error));
  });
}

//#region 🔌️ExtensionStoreVitePlugin
/** @emoji 🔌 Vite middleware: `POST /🧩️extension-modules/install`, `GET /🧩️extension-modules/watch` SSE (mirrors plugin hot-swap). */
export function semioExtensionStoreVitePlugin(options: { readonly installRoot: string; readonly repoRoot: string; readonly materializer?: ExtensionMaterializer }) {
  const store = createExtensionStore({
    installRoot: options.installRoot,
    repoRoot: options.repoRoot,
    materializer: options.materializer ?? webMaterialize,
  });
  return {
    name: "semio-extension-store",
    configureServer(server: { middlewares: { use: (handler: (req: BackboneServerRequest, res: BackboneServerResponse, next: () => void) => void) => void } }) {
      const subscribers = new Set<BackboneServerResponse>();
      mkdirSync(store.installRoot, { recursive: true });
      const markerPath = join(store.installRoot, EXTENSION_WATCH_MARKER);
      let debounceTimer: ReturnType<typeof setTimeout> | undefined;
      watch(store.installRoot, (_eventType, filename) => {
        if (filename !== EXTENSION_WATCH_MARKER) return;
        if (debounceTimer) clearTimeout(debounceTimer);
        debounceTimer = setTimeout(() => {
          if (!existsSync(markerPath)) return;
          let marker: ExtensionSourceEvent & { emittedAt?: number };
          try {
            marker = JSON.parse(readFileSync(markerPath, "utf8")) as ExtensionSourceEvent & { emittedAt?: number };
          } catch {
            return;
          }
          const { emittedAt: _ignored, ...event } = marker;
          const payload = `data: ${JSON.stringify(event)}\n\n`;
          for (const sub of subscribers) sub.write(payload);
        }, FOLDER_WATCH_DEBOUNCE_MS);
      });
      server.middlewares.use(async (req, res, next) => {
        const requestPath = moduleRoutePath(req.url ?? "");
        if (requestPath === EXTENSION_WATCH_PATH && req.method === "GET") {
          res.statusCode = 200;
          res.setHeader("content-type", "text/event-stream");
          res.setHeader("cache-control", "no-cache");
          res.setHeader("connection", "keep-alive");
          res.write(": connected\n\n");
          const snapshot: ExtensionSourceEvent = { kind: "snapshot", extensions: await store.listInstalled() };
          res.write(`data: ${JSON.stringify(snapshot)}\n\n`);
          subscribers.add(res);
          req.on("close", () => subscribers.delete(res));
          return;
        }
        if (requestPath !== EXTENSION_INSTALL_PATH || req.method !== "POST") return next();
        try {
          const contentType = (req as { headers?: Record<string, string> }).headers?.["content-type"] ?? "";
          let result: ExtensionInstallResult;
          if (contentType.includes("application/json")) {
            const body = JSON.parse((await readRequestBody(req)).toString("utf8")) as { url?: string };
            if (!body.url) throw new Error("JSON body must include { url }");
            result = await store.installFromUrl(body.url);
          } else {
            const bytes = await readRequestBody(req);
            if (bytes.length === 0) throw new Error("empty extension package body");
            result = await store.installFromBytes(new Uint8Array(bytes));
          }
          res.statusCode = 200;
          res.setHeader("content-type", "application/json");
          res.end(JSON.stringify(result));
        } catch (error) {
          res.statusCode = 400;
          res.setHeader("content-type", "application/json");
          res.end(JSON.stringify({ error: error instanceof Error ? error.message : String(error) }));
        }
      });
    },
  };
}
//#endregion 🔌️ExtensionStoreVitePlugin

//#region 🧪️Tests
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;
  const legacyFflateZip = Uint8Array.from(
    Buffer.from(
      "UEsDBBQAAAgIAAAAIQAAHXf+kQAAAJwAAAAVAAAA8J+bgu+4j21hbmlmZXN0LnNlbWlvY2U11DPSM+ZPy6woKS1K1Tu8JzcnsbSEpSg1MYWxmP/D/OUr3+/oV3CDSDMyCgqws/MkJxYkJmXmZJZkphbzMLIxsfMm5+eVFGUmlZZk5ucV8zCws6dWlKTmpRSzMbNzg5nFQAnPFDZGdtacxKTUHDYWdt6CxOTsxPRUt/yi3MQSVgYw+GDPzl6WWgRSzcYAAFBLAwQUAAAACAAAACEAzjNLHAoAAAAIAAAADgAAAGNvbXBvbmVudC53YXNtY0gszmVkYGAAAFBLAwQUAAAICAAAACEA2f/rzhIAAAAQAAAAGAAAAGFzc2V0cy9pY29ucy/wn6ep77iPLnR4dHMvOrwntSpT4cP8nt73O/q5AFBLAQIUABQAAAgIAAAAIQAAHXf+kQAAAJwAAAAVAAAAAAAAAAAAAAAAAAAAAADwn5uC77iPbWFuaWZlc3Quc2VtaW9QSwECFAAUAAAACAAAACEAzjNLHAoAAAAIAAAADgAAAAAAAAAAAAAAAADEAAAAY29tcG9uZW50Lndhc21QSwECFAAUAAAICAAAACEA2f/rzhIAAAAQAAAAGAAAAAAAAAAAAAAAAAD6AAAAYXNzZXRzL2ljb25zL/Cfp6nvuI8udHh0UEsFBgAAAAADAAMAxQAAAEIBAAAAAA==",
      "base64",
    ),
  );
  const fixtureManifest: ExtensionPackageManifestRecord = {
    extensionId: "fixture.ümlaut",
    directoryName: "🧩️fixture-umlaut",
    label: "🧩️ Fixture",
    version: "1.2.3",
    extends: "s",
    capabilities: ["read"],
    contributions: [],
    packageFormat: 1,
  };
  const fixtureWasm = new Uint8Array([0, 97, 115, 109, 1, 0, 0, 0]);
  const fixtureAsset = new TextEncoder().encode("Grüezi 🌍️\n");

  describe("authored extension installation identity", () => {
    it("retains the declared physical name independently of the public extension ID", async () => {
      const vector = JSON.parse(readFileSync(new URL("../../🧩️extension/🧪️installation.json", import.meta.url), "utf8"));
      const root = mkdtempSync(join(tmpdir(), "semio-authored-install-"));
      const writes: string[] = [];
      try {
        const store = createExtensionStore({ installRoot: root, repoRoot: root, materializer: async (input) => {
          writes.push(input.outDir);
          return { moduleUrl: `${MODULE_EXTENSION_ROUTE}/${input.directoryName}/module.js` };
        } });
        const packed = packExtensionPackage({ manifest: vector.manifest, componentWasm: fixtureWasm });
        const installed = await store.installFromBytes(packed);
        expect(installed.extensionId).toBe(vector.manifest.extensionId);
        expect(writes).toEqual([join(root, vector.manifest.directoryName)]);
        expect((await store.listInstalled())[0].directoryName).toBe(vector.manifest.directoryName);
        expect(existsSync(join(root, vector.manifest.extensionId))).toBe(false);
        const collision = { ...vector.manifest, extensionId: "other-id", directoryName: "🧩️another" };
        await expect(store.installFromBytes(packExtensionPackage({ manifest: collision, componentWasm: fixtureWasm }))).rejects.toThrow(/sibling emoji/);
        expect(writes).toHaveLength(1);
        await store.uninstall(vector.manifest.extensionId);
        expect(existsSync(join(root, vector.manifest.directoryName))).toBe(false);
      } finally { rmSync(root, { recursive: true, force: true }); }
    });

    it("agrees with JSON Schema and independent emoji identity checks", async () => {
      const { default: Ajv } = await import("ajv");
      const emojiRegex = (await import("emoji-regex")).default;
      const { installationDirectoryEmoji, installationDirectoryCollision } = await import("../../🧩️extension/🟦️.ts");
      const schema = JSON.parse(readFileSync(new URL("../../🧩️extension/📐️directory.schema.json", import.meta.url), "utf8"));
      const vector = JSON.parse(readFileSync(new URL("../../🧩️extension/🧪️installation.json", import.meta.url), "utf8"));
      const cases = JSON.parse(readFileSync(new URL("../📇️registry/📦️deployment/🧪️cases.json", import.meta.url), "utf8"));
      const validate = new Ajv({ strict: true }).compile(schema);
      for (const name of [...cases.validDirectories, ...cases.invalidDirectories]) {
        const valid = cases.validDirectories.includes(name);
        expect(validate(name), name).toBe(valid);
        if (valid) expect(installationDirectoryEmoji(name)).toBe([...name.replaceAll("\uFE0F", "").matchAll(emojiRegex())][0][0]);
        else expect(() => installationDirectoryEmoji(name)).toThrow();
      }
      for (const row of vector.collisions) expect(installationDirectoryCollision(row.directoryName, row.siblings) ?? null).toBe(row.conflict);
    });
  });

  describe("extension package ZIP ownership", () => {
    it("decodes the pinned fflate UTF-8/DEFLATE fixture and preserves its hash", () => {
      expect(extensionPackageContentHash(legacyFflateZip)).toBe("43675c79f03ba52f45cc57eecabee2a9334e93957128e7975a2979528d14efa9");
      const decoded = decodeOwnedZip(legacyFflateZip);
      expect(decodePackValue(decoded.get(EXTENSION_MANIFEST_ZIP_ENTRY_EMOJI)!)).toMatchObject({ extensionId: fixtureManifest.extensionId, label: fixtureManifest.label, version: fixtureManifest.version });
      expect(decoded.get(EXTENSION_COMPONENT_FILE)).toEqual(fixtureWasm);
      expect(decoded.get("assets/icons/🧩️.txt")).toEqual(fixtureAsset);
      expect(() => unpackExtensionPackage(wrapExtensionPackageEnvelope(legacyFflateZip))).toThrow(/Installation directory/);
    });

    it("encodes deterministic synchronous packages with UTF-8 asset names", () => {
      const input = { manifest: fixtureManifest, componentWasm: fixtureWasm, assets: new Map([["icons/🧩️.txt", fixtureAsset]]) };
      const first = packExtensionPackage(input);
      const second = packExtensionPackage(input);
      expect(first).toEqual(second);
      const unpacked = unpackExtensionPackage(first);
      expect(unpacked.packageHash).toBe(extensionPackageContentHash(first));
      expect(unpacked.manifest).toMatchObject({ extensionId: fixtureManifest.extensionId, label: fixtureManifest.label, version: fixtureManifest.version });
      expect(unpacked.wasmBytes).toEqual(fixtureWasm);
      expect(unpacked.assets.get("icons/🧩️.txt")).toEqual(fixtureAsset);
    });

    it("rejects an entry whose declared expansion exceeds the owned bound", () => {
      const packed = packExtensionPackage({ manifest: fixtureManifest, componentWasm: fixtureWasm });
      const zipStart = 12 + new TextEncoder().encode(EXTENSION_PACKAGE_ENVELOPE_TOKEN).length;
      const corrupted = packed.slice(zipStart);
      const data = new DataView(corrupted.buffer, corrupted.byteOffset, corrupted.byteLength);
      const end = corrupted.length - 22;
      const central = data.getUint32(end + 16, true);
      data.setUint32(central + 24, 256 * 1024 * 1024 + 1, true);
      expect(() => unpackExtensionPackage(wrapExtensionPackageEnvelope(corrupted))).toThrow("decoded size limit");
    });
  });
}
//#endregion 🧪️Tests
