/** 🧭️ Trusted bootstrap bundle types, codec capture, generation receipts, plugin module records and package selection.
 * @see ../../../📦️packages/🦀️rust/📜️script.ts
 */
import { type CompilerAdmissionControl } from "@semio-tech/process/compiler-capacity";
import { type FileObservationControlV1 } from "@semio-tech/framework-filesystem/observation";
import { COMPONENT_MODULE_DIRECTORIES } from "@semio-tech/s-dev-catalog-ts/generated/plugins";
import { createHash } from "node:crypto";
import { closeSync, existsSync, fsyncSync, lstatSync, mkdirSync, openSync, readdirSync, rmSync, writeFileSync, writeSync, type Stats } from "node:fs";
import { dirname, join } from "node:path";
import { captureOwnedCommandResultV1 } from "@semio-tech/process/owned-execution";
import { blake3Hex } from "@semio-tech/framework-hash";
import { decodePackValue, encodePackValue, packValueToExactJson } from "@semio-tech/framework-os";
import { DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES } from "@semio-tech/framework-os/modules/directory/schema/document-open";
import { DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES, type DocumentExecutionProtocolV1 } from "@semio-tech/framework/directory/schema";
import { type FreshBuildControlV1, type FreshComponentReceiptV1 } from "@semio-tech/framework-plugin/describe/source-epoch";
import { COMPONENT_RECEIPT_FILE, componentReceiptBindsV1, componentReceiptValueV1, encodeComponentReceiptV1, type ComponentReceiptV1 } from "@semio-tech/framework-plugin/describe/component-receipt";
import { ensureGuestSlimTypstFontsAt } from "@semio-tech/framework-plugin/browser-bundle/materialization/guestslim-fonts";
import { ensurePreview2ShimVendorAt } from "@semio-tech/framework-plugin/browser-bundle/materialization/preview2-shim-patches";
import { hostShimSource } from "@semio-tech/framework-plugin/browser-bundle/materialization/host-shim-source";
import { PLUGIN_HOST_SHIM_FILE } from "@semio-tech/framework-plugin/browser-bundle/materialization/contract";
import { PREVIEW2_VENDOR_RELATIVE } from "@semio-tech/framework-plugin/browser-bundle/imports";
import { pluginComponentBridgeSource } from "@semio-tech/framework-plugin/browser-bundle/materialization/component-bridge-source";
import { transpilePluginComponentAsync } from "@semio-tech/framework-plugin/browser-bundle/materialization";
import { MODULE_BRIDGE_FILE, moduleDirectoryName } from "@semio-tech/plugin-registry/deployment";
import { decodeTrustedPluginModuleBundleV1, encodeTrustedPluginModuleBundleV1, TRUSTED_PLUGIN_MODULE_BUNDLE_MAX_BYTES, TRUSTED_PLUGIN_MODULE_DESCRIPTOR_JSON_FILE, TRUSTED_PLUGIN_MODULE_DESCRIPTOR_PACK_FILE, TRUSTED_PLUGIN_MODULE_FILE_MAX_BYTES, TRUSTED_PLUGIN_MODULE_SCHEMA } from "@semio-tech/plugin-registry/deployment/schema";
import { utf8OrderV1, type TrustedPluginModuleBundleV1 } from "@semio-tech/framework/plugin-module";
import { readStableBuildFile } from "@semio-tech/repo-lib/build";
import { type LocalHubHostV1, withLocalHubHostTaskV1 } from "../../../🚀️local-bootstrap/🏃️execution/🟦️.ts";
import { documentOpenNeutralObject, trustedBootstrapBrowserActorV1, type TrustedBootstrapBrowserActorV1, type TrustedBootstrapCodec, trustedBootstrapCodecOrder, trustedBootstrapDependencies, trustedBootstrapDependencyEncoding, trustedBootstrapDescriptorClaims, trustedBootstrapExecutionProtocol, trustedBootstrapIdentity, type TrustedBootstrapIdentityV1, type TrustedBootstrapOpenTargetV1, trustedBootstrapPluginModuleBlobPath, trustedBootstrapPluginModuleManifestPath, trustedBootstrapPluginModuleRecordV1, type TrustedBootstrapPluginModuleRecordV1, trustedBootstrapResolveDependencies } from "../🧮️bootstrap-claims/🟦️.ts";
import { trustedBootstrapReadRegular } from "../🧭️bootstrap-selection/🟦️.ts";

export type TrustedBootstrapBundleV1=Readonly<{profileId:string;generationId:string;bundleSha256:string;bundlePath:string}>;
export type TrustedBootstrapMaterializationV1 = TrustedBootstrapBundleV1&Readonly<{producerPath:string}>;
export type TrustedBootstrapCurrentPointerV1 = Readonly<{ profileId: string; generationId: string; bundleSha256: string; publicationRevision: string }>;
export type TrustedBootstrapPublishedV1 = TrustedBootstrapBundleV1 & Readonly<{ publicationRevision: string; currentSha256: string }>;

export function trustedBootstrapWriteNew(path: string, bytes: Uint8Array, check: () => void): void {
  check();
  const output = openSync(path, "wx", 0o600);
  let complete = false;
  try {
    let written = 0;
    while (written < bytes.byteLength) {
      check();
      const count = writeSync(output, bytes, written, Math.min(64 * 1024, bytes.byteLength - written));
      if (count <= 0) throw new Error("trusted staging write made no progress");
      written += count;
    }
    check();
    fsyncSync(output);
    check();
    complete = true;
  } finally {
    try {
      closeSync(output);
    } catch (error) {
      complete = false;
      throw error;
    } finally {
      if (!complete) rmSync(path, { force: true });
    }
  }
}

export function trustedBootstrapFsyncDirectory(path: string): void {
  if (process.platform === "win32") return;
  const directory = openSync(path, "r");
  try {
    fsyncSync(directory);
  } finally {
    closeSync(directory);
  }
}

/** 📏️ The most native codecs one bootstrap package may contribute, the bound of the stdio codec catalog itself. */
const TRUSTED_BOOTSTRAP_PACKAGE_CODEC_MAX = 1024;

/** 🪪️ Projects only the schema-admitted fixed native codec closure into immutable generation inputs. */
export function projectTrustedBootstrapCodecsV1(stdio: unknown, gis: unknown): Readonly<{ gisVersion: string; codecs: Readonly<Record<"gis" | "stdio", readonly TrustedBootstrapCodec[]>> }> {
  const fail = (): never => {
    throw new Error("trusted codec source is not the exact bounded native closure");
  };
  const record = (value: unknown, keys: readonly string[]): Record<string, any> => {
    if (!value || typeof value !== "object" || Array.isArray(value) || JSON.stringify(Object.keys(value).sort()) !== JSON.stringify([...keys].sort())) return fail();
    return value as Record<string, any>;
  };
  const identity = (value: unknown): value is string => typeof value === "string" && value.length > 0 && value.length <= 256 && !/[\u0000-\u001f\u007f]/u.test(value);
  const digest = (value: unknown): value is string => typeof value === "string" && /^(?!0{64}$)[0-9a-f]{64}$/u.test(value);
  const s = record(stdio, ["schema", "provider_id", "plugin_id", "package_id", "receipts"]);
  const g = record(gis, ["schema", "pluginId", "packageId", "packageVersion", "receipts"]);
  if (s.schema !== "semio.stdio.native-openable-catalog-provider/v1" || s.provider_id !== "stdio/native-codecs/v1" || s.plugin_id !== "stdio" || s.package_id !== "semio:stdio" || !Array.isArray(s.receipts) || s.receipts.length === 0 || s.receipts.length > TRUSTED_BOOTSTRAP_PACKAGE_CODEC_MAX) return fail();
  if (
    g.schema !== "semio.gis.native-codec-receipts/v1" ||
    g.pluginId !== "gis" ||
    g.packageId !== "semio:gis" ||
    g.packageVersion !== "0.1.0" ||
    !Array.isArray(g.receipts) ||
    g.receipts.length !== 2
  )
    return fail();
  const stdioRows = s.receipts.map((value: unknown) => {
    const fields = ["artifact", "factory_id", "descriptor_codec_id", "runtime_capability_id", "artifact_kind", "artifact_schema", "extension"];
    const row = record(value, [...fields, "pack_schema_hash", "protocol_source_sha256", "protocol_path", "definition_path"]);
    if (!fields.every((field) => identity(row[field])) || !digest(row.pack_schema_hash) || !digest(row.protocol_source_sha256) || typeof row.definition_path !== "string" || !/^✏️s\/🔌️plugins\/🗄️stdio\/🗿️artifacts\/[^/]+\/📜️artifact-definition\.json$/u.test(row.definition_path) || typeof row.protocol_path !== "string" || row.protocol_path.length > 1024 || !/^✏️s\/🔌️plugins\/🗄️stdio\/🗿️artifacts\/.+\/📡️\.protocol\.semio$/u.test(row.protocol_path))
      return fail();
    return Object.freeze({ artifactKind: row.artifact_kind as string, artifactSchema: row.artifact_schema as string, packSchemaHash: row.pack_schema_hash });
  });
  const gisRows = g.receipts.map((value: unknown) => {
    const row = record(value, ["factoryId", "kind", "schema", "extension", "capability", "packRecord", "protocolPath", "protocolBytes", "protocolSha256", "packSchemaHash"]);
    const family = row.kind === "s.gis.gismap" ? { id: "gismap", schema: "map", owner: "🗺️gismap" } : row.kind === "s.gis.gisterrain" ? { id: "gisterrain", schema: "terrain", owner: "🏔️gisterrain" } : fail();
    if (
      row.factoryId !== `gis.${family.id}.v1` ||
      row.schema !== `gis.${family.schema}` ||
      row.extension !== family.id ||
      row.capability !== `s.gis.${family.id}.codec.document` ||
      row.protocolPath !== `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/${family.owner}/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/📸️snapshot/📡️.protocol.semio` ||
      !Number.isSafeInteger(row.protocolBytes) ||
      row.protocolBytes < 1 ||
      row.protocolBytes > 65536 ||
      !digest(row.protocolSha256) ||
      !digest(row.packSchemaHash)
    )
      return fail();
    return Object.freeze({ artifactKind: row.kind as string, artifactSchema: row.schema as string, packSchemaHash: row.packSchemaHash as string });
  });
  for (const rows of [stdioRows, gisRows]) if (new Set(rows.map((row: TrustedBootstrapCodec) => JSON.stringify([row.artifactKind, row.artifactSchema]))).size !== rows.length) return fail();
  return Object.freeze({ gisVersion: g.packageVersion as string, codecs: Object.freeze({ stdio: Object.freeze(stdioRows.sort(trustedBootstrapCodecOrder)), gis: Object.freeze(gisRows.sort(trustedBootstrapCodecOrder)) }) });
}

/** 📸️ Captures both codec sources once before any long component build can outlive them. */
export function captureTrustedBootstrapCodecsV1(repoRoot: string, check: (stage?: string) => void): ReturnType<typeof projectTrustedBootstrapCodecsV1> {
  const admission = { remaining: 128 * 1024 };
  let stdio: Uint8Array | undefined, gis: Uint8Array | undefined;
  try {
    stdio = readStableBuildFile(join(repoRoot, "🌎️hub/🧩️compositions/🗄️stdio/🔌️plugin/📇️catalog/📜️native-codec-factories.json"), 64 * 1024, admission, check);
    gis = readStableBuildFile(join(repoRoot, "🌎️hub/🧩️compositions/🌍️gis/📇️native-codecs/🔣️.json"), 64 * 1024, admission, check);
    check("project-codecs");
    const decoder = new TextDecoder("utf-8", { fatal: true });
    return projectTrustedBootstrapCodecsV1(JSON.parse(decoder.decode(stdio)), JSON.parse(decoder.decode(gis)));
  } catch (error) {
    throw new Error(`trusted codec capture: ${(error as Error).message}`);
  } finally {
    stdio?.fill(0);
    gis?.fill(0);
  }
}

export type TrustedBootstrapGenerationReceiptV1 = Readonly<{
  identity: TrustedBootstrapIdentityV1;
  dependencies: readonly TrustedBootstrapIdentityV1[];
  component: Readonly<{ byteLength: number; sha256: string }>;
  descriptor: Readonly<{ byteLength: number; sha256: string }>;
  componentReceipt: ComponentReceiptV1;
  executionProtocol: Readonly<DocumentExecutionProtocolV1>;
  browserActor: TrustedBootstrapBrowserActorV1;
  pluginModule: TrustedBootstrapPluginModuleRecordV1;
}>;

/** 🧾️ The digest a plugin module's `🧾️receipt.json` must carry: the SHA-256 of the package receipt's canonical encoding. */
function trustedBootstrapReceiptSha256(receipt: ComponentReceiptV1): string {
  return createHash("sha256").update(encodeComponentReceiptV1(receipt)).digest("hex");
}

/** 🔗️ A catalog record's receipt, admitted only as a shipped receipt naming exactly the record's package, component and descriptor. */
export function trustedBootstrapRecordReceipt(record: any): ComponentReceiptV1 {
  const receipt = componentReceiptValueV1(record.receipt);
  if (!componentReceiptBindsV1(receipt, { profile: "release", pluginId: record.pluginId, packageId: record.packageId, wasmSha256: record.component.sha256, semanticDescriptorSha256: record.descriptor.sha256 }) || receipt.wasmBytes !== record.component.byteLength) throw new Error(`trusted ${record.pluginId} receipt does not bind its component to its semantic descriptor`);
  return receipt;
}

/** 🧩️ Reads one generation's plugin module manifest, bound to its record and its package's digests. */
function trustedBootstrapReadPluginModule(root: string, receipt: TrustedBootstrapGenerationReceiptV1, check: () => void): TrustedPluginModuleBundleV1 {
  const bytes = readStableBuildFile(join(root, receipt.pluginModule.path), TRUSTED_PLUGIN_MODULE_BUNDLE_MAX_BYTES, { remaining: TRUSTED_PLUGIN_MODULE_BUNDLE_MAX_BYTES }, check);
  if (bytes.byteLength !== receipt.pluginModule.byteLength || createHash("sha256").update(bytes).digest("hex") !== receipt.pluginModule.sha256 || blake3Hex(bytes) !== receipt.pluginModule.blake3) throw new Error("generation plugin module manifest differs from its record");
  return decodeTrustedPluginModuleBundleV1(bytes, { ...receipt.identity, componentSha256: receipt.component.sha256, descriptorByteSha256: receipt.descriptor.sha256, receiptSha256: trustedBootstrapReceiptSha256(receipt.componentReceipt) });
}

/**
 * 🧳️ Carries one package's plugin module from a verified source generation into a new stage: every file is
 * reread and checked against its manifest, and the packed and JSON descriptors are replaced by the package's
 * (possibly rewritten) `descriptor`, so the carried module is derived from exactly the package it ships with.
 */
export function trustedBootstrapCarryPluginModule(sourceRoot: string, stageRoot: string, sourceReceipt: TrustedBootstrapGenerationReceiptV1, record: any, descriptor: Uint8Array, check: () => void): TrustedBootstrapPluginModuleRecordV1 {
  const source = trustedBootstrapReadPluginModule(sourceRoot, sourceReceipt, check);
  const componentReceipt = trustedBootstrapRecordReceipt(record);
  const replacements = new Map<string, Uint8Array>([
    [`${source.moduleDirectory}/${TRUSTED_PLUGIN_MODULE_DESCRIPTOR_PACK_FILE}`, descriptor],
    [`${source.moduleDirectory}/${TRUSTED_PLUGIN_MODULE_DESCRIPTOR_JSON_FILE}`, Buffer.from(`${JSON.stringify(packValueToExactJson(decodePackValue(descriptor)), null, 2)}\n`, "utf8")],
    [`${source.moduleDirectory}/${COMPONENT_RECEIPT_FILE}`, Buffer.from(encodeComponentReceiptV1(componentReceipt), "utf8")],
  ]);
  const blobs = join(stageRoot, "plugin-modules");
  mkdirSync(blobs, { recursive: true, mode: 0o700 });
  const files = source.files.map((file) => {
    const bytes = replacements.get(file.path) ?? readStableBuildFile(join(sourceRoot, trustedBootstrapPluginModuleBlobPath(file.sha256)), file.byteLength, { remaining: file.byteLength }, check);
    const carried = { path: file.path, byteLength: bytes.byteLength, sha256: createHash("sha256").update(bytes).digest("hex"), blake3: blake3Hex(bytes) };
    if (!replacements.has(file.path) && (carried.sha256 !== file.sha256 || carried.blake3 !== file.blake3)) throw new Error("trusted rotation plugin module file differs from its manifest");
    if (!existsSync(join(blobs, carried.sha256))) trustedBootstrapWriteNew(join(blobs, carried.sha256), bytes, check);
    return carried;
  });
  const descriptorByteSha256 = createHash("sha256").update(descriptor).digest("hex");
  const manifest = encodeTrustedPluginModuleBundleV1({ ...source, sourceComponentSha256: record.component.sha256, sourceDescriptorByteSha256: descriptorByteSha256, files });
  decodeTrustedPluginModuleBundleV1(manifest, { pluginId: record.pluginId, packageId: record.packageId, version: record.version, componentSha256: record.component.sha256, descriptorByteSha256, receiptSha256: trustedBootstrapReceiptSha256(componentReceipt) });
  trustedBootstrapWriteNew(join(stageRoot, trustedBootstrapPluginModuleManifestPath(record.pluginId)), manifest, check);
  return Object.freeze({ path: trustedBootstrapPluginModuleManifestPath(record.pluginId), byteLength: manifest.byteLength, sha256: createHash("sha256").update(manifest).digest("hex"), blake3: blake3Hex(manifest) });
}

/** 🛑️ An abort signal for one child process that follows the bootstrap's own cancellation and deadline. */
function trustedBootstrapAbortSignal(control: FreshBuildControlV1): { readonly signal: AbortSignal; close(): void } {
  const abort = new AbortController();
  const poll = setInterval(() => {
    if (control.cancelled() || control.remainingMs() <= 0) abort.abort(new Error("trusted catalog build cancelled"));
  }, 250);
  return { signal: abort.signal, close: () => clearInterval(poll) };
}

/**
 * 🧩️ Materializes one package's browser plugin module from its own fresh component and staged semantic descriptor —
 * host shim, jco component module, core Wasm, bridge, both descriptor forms, the package's canonical component receipt
 * and the vendored imports — and stages it content-addressed in the generation: every file once under
 * `plugin-modules/<sha256>` (shared across packages), the canonical manifest at `packages/<plugin>/plugin-module.json`.
 */
export async function trustedBootstrapPluginModuleV1(repoRoot: string, request: Readonly<{ pluginId: string; outputName: string }>, stage: string, stageRoot: string, work: string, receipt: FreshComponentReceiptV1, control: FreshBuildControlV1, check: () => void): Promise<TrustedBootstrapPluginModuleRecordV1> {
  const moduleRoot = join(work, "plugin-module");
  const directory = moduleDirectoryName(request.pluginId, COMPONENT_MODULE_DIRECTORIES);
  const moduleDir = join(moduleRoot, directory);
  const vendor = join(moduleRoot, PREVIEW2_VENDOR_RELATIVE);
  mkdirSync(moduleDir, { recursive: true, mode: 0o700 });
  ensurePreview2ShimVendorAt(vendor, repoRoot);
  if (!ensureGuestSlimTypstFontsAt(moduleRoot, repoRoot)) throw new Error("trusted plugin module needs the staged guestslim typst font seed");
  const descriptor = trustedBootstrapReadRegular(join(stage, "descriptor.semio"), DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES, `fresh ${request.pluginId} descriptor for its plugin module`, check);
  if (descriptor.byteLength !== receipt.descriptor.byteLength || createHash("sha256").update(descriptor).digest("hex") !== receipt.descriptor.sha256) throw new Error("fresh descriptor differs from its receipt before plugin module materialization");
  const componentBase = request.outputName.slice(0, -".wasm".length);
  writeFileSync(join(moduleDir, PLUGIN_HOST_SHIM_FILE), hostShimSource());
  const abort = trustedBootstrapAbortSignal(control);
  try {
    await transpilePluginComponentAsync(join(stage, "component.wasm"), moduleDir, componentBase, { repoRoot, preview2VendorDir: vendor, signal: abort.signal, optimize: false });
  } finally {
    abort.close();
  }
  check();
  writeFileSync(join(moduleDir, MODULE_BRIDGE_FILE), pluginComponentBridgeSource(componentBase, request.outputName));
  writeFileSync(join(moduleDir, TRUSTED_PLUGIN_MODULE_DESCRIPTOR_JSON_FILE), `${JSON.stringify(packValueToExactJson(decodePackValue(descriptor)), null, 2)}\n`);
  writeFileSync(join(moduleDir, TRUSTED_PLUGIN_MODULE_DESCRIPTOR_PACK_FILE), descriptor);
  writeFileSync(join(moduleDir, COMPONENT_RECEIPT_FILE), encodeComponentReceiptV1(receipt.componentReceipt));
  rmSync(join(moduleDir, `${componentBase}.d.ts`), { force: true });
  rmSync(join(moduleDir, "interfaces"), { recursive: true, force: true });
  const walk = (relative: string): string[] =>
    readdirSync(join(moduleRoot, relative), { withFileTypes: true }).flatMap((entry) => {
      const path = relative ? `${relative}/${entry.name}` : entry.name;
      if (entry.isDirectory()) return walk(path);
      if (!entry.isFile() || entry.name.startsWith(".")) throw new Error(`trusted plugin module carries a non-file ${path}`);
      return [path];
    });
  const blobs = join(stageRoot, "plugin-modules");
  mkdirSync(blobs, { recursive: true, mode: 0o700 });
  const files = walk("")
    .sort(utf8OrderV1)
    .map((path) => {
      check();
      const bytes = readStableBuildFile(join(moduleRoot, path), TRUSTED_PLUGIN_MODULE_FILE_MAX_BYTES, { remaining: TRUSTED_PLUGIN_MODULE_FILE_MAX_BYTES }, check);
      const file = { path, byteLength: bytes.byteLength, sha256: createHash("sha256").update(bytes).digest("hex"), blake3: blake3Hex(bytes) };
      if (!existsSync(join(blobs, file.sha256))) trustedBootstrapWriteNew(join(blobs, file.sha256), bytes, check);
      return file;
    });
  const bundle = {
    schema: TRUSTED_PLUGIN_MODULE_SCHEMA,
    pluginId: request.pluginId,
    packageId: receipt.packageId,
    version: receipt.version,
    sourceComponentSha256: receipt.component.sha256,
    sourceDescriptorByteSha256: receipt.descriptor.sha256,
    moduleDirectory: directory,
    entry: `${directory}/${MODULE_BRIDGE_FILE}`,
    files,
  };
  const manifest = encodeTrustedPluginModuleBundleV1(bundle);
  decodeTrustedPluginModuleBundleV1(manifest, { pluginId: request.pluginId, packageId: receipt.packageId, version: receipt.version, componentSha256: receipt.component.sha256, descriptorByteSha256: receipt.descriptor.sha256, receiptSha256: trustedBootstrapReceiptSha256(receipt.componentReceipt) });
  trustedBootstrapWriteNew(join(stage, "plugin-module.json"), manifest, check);
  trustedBootstrapFsyncDirectory(blobs);
  rmSync(moduleRoot, { recursive: true, force: true });
  return Object.freeze({ path: trustedBootstrapPluginModuleManifestPath(request.pluginId), byteLength: manifest.byteLength, sha256: createHash("sha256").update(manifest).digest("hex"), blake3: blake3Hex(manifest) });
}

/** 🧷️ Retains the exact metadata needed to rebind every staged descriptor to its bundle. */
export function trustedBootstrapGenerationReceipt(record: any): TrustedBootstrapGenerationReceiptV1 {
  const identity = trustedBootstrapIdentity({ pluginId: record.pluginId, packageId: record.packageId, version: record.version });
  return Object.freeze({
    identity, dependencies: trustedBootstrapDependencies(record.dependencies),
    component: Object.freeze({ byteLength: record.component.byteLength, sha256: record.component.sha256 }),
    descriptor: Object.freeze({ byteLength: record.descriptor.byteLength, sha256: record.descriptor.sha256 }),
    componentReceipt: trustedBootstrapRecordReceipt(record),
    executionProtocol: trustedBootstrapExecutionProtocol(record.executionProtocol),
    browserActor: trustedBootstrapBrowserActorV1(record.browserActor, { componentSha256: record.component.sha256, descriptorByteSha256: record.descriptor.sha256 }, record.browserActor?.kind === "closed-browser-actor" ? "wasm" : "react"),
    pluginModule: trustedBootstrapPluginModuleRecordV1(record.pluginModule, identity.pluginId),
  });
}

/** 🧱️ Revalidates every fixed generation input and its path identity immediately before publication. */
export function trustedBootstrapVerifyGeneration(root: string, bundle: Uint8Array, receipts: ReadonlyMap<string, TrustedBootstrapGenerationReceiptV1>, check: (phase?: string) => void): void {
  try {
    check();
    if (receipts.size === 0 || !receipts.has("stdio") || !bundle.byteLength || bundle.byteLength > 4 * 1024 * 1024) throw new Error("generation input closure differs");
    const packages = JSON.parse(new TextDecoder("utf-8", { fatal: true }).decode(bundle)).packages;
    if (!Array.isArray(packages) || packages.length !== receipts.size) throw new Error("generation bundle dependency closure differs");
    const bundleReceipts = new Map(packages.map((record) => [record.pluginId, trustedBootstrapGenerationReceipt(record)]));
    if (bundleReceipts.size !== receipts.size) throw new Error("generation bundle dependency identity is duplicated");
    const selected = trustedBootstrapDependencies([...bundleReceipts.values()].map((receipt) => receipt.identity));
    for (const [plugin, receipt] of receipts) {
      const expected = bundleReceipts.get(plugin);
      if (!expected || JSON.stringify(expected) !== JSON.stringify(trustedBootstrapGenerationReceipt({ ...receipt.identity, ...receipt, receipt: receipt.componentReceipt }))) throw new Error("generation bundle metadata differs from its retained receipt");
    }
    const plugins = [...bundleReceipts.keys()].sort();
    const carriesActor = (plugin: string): boolean => bundleReceipts.get(plugin)!.browserActor.kind === "closed-browser-actor";
    const moduleFiles = new Map<string, Readonly<{ byteLength: number; sha256: string }>>();
    for (const plugin of plugins) {
      for (const file of trustedBootstrapReadPluginModule(root, receipts.get(plugin)!, check).files) {
        const known = moduleFiles.get(file.sha256);
        if (known && known.byteLength !== file.byteLength) throw new Error("generation plugin modules disagree about one content address");
        moduleFiles.set(file.sha256, { byteLength: file.byteLength, sha256: file.sha256 });
      }
    }
    const directories: (readonly [string, readonly string[]])[] = [
      [root, ["packages", "plugin-modules", "trusted-catalog.json"]],
      [join(root, "packages"), plugins],
      [join(root, "plugin-modules"), [...moduleFiles.keys()]],
    ];
    for (const plugin of plugins) {
      directories.push([join(root, "packages", plugin), carriesActor(plugin) ? ["browser", "component.wasm", "descriptor.semio", "plugin-module.json"] : ["component.wasm", "descriptor.semio", "plugin-module.json"]]);
      if (carriesActor(plugin)) directories.push([join(root, "packages", plugin, "browser"), ["closed-actor.mjs"]]);
    }
    const identities = new Map<string, Stats>();
    for (const [path, entries] of directories) {
      check();
      const info = lstatSync(path);
      if (info.isSymbolicLink() || !info.isDirectory() || JSON.stringify(readdirSync(path).sort()) !== JSON.stringify([...entries].sort())) throw new Error("generation directory is not the exact regular closure");
      identities.set(path, info);
    }
    const files = [{ path: join(root, "trusted-catalog.json"), byteLength: bundle.byteLength, sha256: createHash("sha256").update(bundle).digest("hex"), maximum: 4 * 1024 * 1024 }];
    const descriptorReceipts = new Map<string, TrustedBootstrapGenerationReceiptV1>();
    for (const plugin of plugins) {
      const receipt = receipts.get(plugin)!;
      descriptorReceipts.set(join(root, "packages", plugin, "descriptor.semio"), receipt);
      files.push({ path: join(root, "packages", plugin, "component.wasm"), byteLength: receipt.component.byteLength, sha256: receipt.component.sha256, maximum: DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES });
      files.push({ path: join(root, "packages", plugin, "descriptor.semio"), byteLength: receipt.descriptor.byteLength, sha256: receipt.descriptor.sha256, maximum: DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES });
      files.push({ path: join(root, receipt.pluginModule.path), byteLength: receipt.pluginModule.byteLength, sha256: receipt.pluginModule.sha256, maximum: TRUSTED_PLUGIN_MODULE_BUNDLE_MAX_BYTES });
      const actor = trustedBootstrapBrowserActorV1(receipt.browserActor, { componentSha256: receipt.component.sha256, descriptorByteSha256: receipt.descriptor.sha256 }, carriesActor(plugin) ? "wasm" : "react");
      if (actor.kind === "closed-browser-actor") {
        if (actor.path !== `packages/${plugin}/browser/closed-actor.mjs`) throw new Error("generation actor path differs from its own package closure");
        files.push({ path: join(root, "packages", plugin, "browser", "closed-actor.mjs"), byteLength: actor.byteLength, sha256: actor.sha256, maximum: 67_108_864 });
      }
    }
    for (const file of moduleFiles.values()) files.push({ path: join(root, trustedBootstrapPluginModuleBlobPath(file.sha256)), byteLength: file.byteLength, sha256: file.sha256, maximum: TRUSTED_PLUGIN_MODULE_FILE_MAX_BYTES });
    const admission = { remaining: files.reduce((sum, file) => sum + file.maximum, 0) };
    for (const file of files) {
      if (!Number.isSafeInteger(file.byteLength) || file.byteLength <= 0 || file.byteLength > file.maximum || !/^[0-9a-f]{64}$/u.test(file.sha256)) throw new Error("generation file receipt is not bounded");
      identities.set(file.path, lstatSync(file.path));
      const bytes = readStableBuildFile(file.path, file.maximum, admission, check);
      try {
        if (bytes.byteLength !== file.byteLength || createHash("sha256").update(bytes).digest("hex") !== file.sha256) throw new Error("generation bytes differ from their verified receipt");
        const receipt = descriptorReceipts.get(file.path);
        if (receipt) {
          const claims = trustedBootstrapDescriptorClaims(bytes);
          const dependencies = trustedBootstrapResolveDependencies(claims, selected);
          if (JSON.stringify(claims.identity) !== JSON.stringify(receipt.identity) || claims.executionProtocol.appChannelVersion !== receipt.executionProtocol.appChannelVersion || !trustedBootstrapDependencyEncoding(dependencies).equals(trustedBootstrapDependencyEncoding(receipt.dependencies))) throw new Error("generation descriptor identity, protocol or dependencies differ from its receipt");
        }
      } finally {
        bytes.fill(0);
      }
    }
    check("identity");
    for (const [path, before] of identities) {
      const after = lstatSync(path);
      if (after.isSymbolicLink() || before.dev !== after.dev || before.ino !== after.ino || before.mode !== after.mode || before.size !== after.size || before.mtimeMs !== after.mtimeMs || before.ctimeMs !== after.ctimeMs)
        throw new Error("generation path changed across the final fence");
    }
  } catch (error) {
    throw new Error(`trusted generation fence: ${(error as Error).message}`);
  }
}

/** 🧩️ One never-executed fixture plugin module: its entry, both descriptor forms (the packed one the package's
 * own descriptor) and one vendored import — the TS twin of the hub's `fixture_plugin_module_files`. */
type TrustedBootstrapFixturePluginModuleV1 = Readonly<{ record: TrustedBootstrapPluginModuleRecordV1; manifest: Uint8Array; blobs: ReadonlyMap<string, Uint8Array> }>;

export function trustedBootstrapFixturePluginModule(identity: Readonly<{ pluginId: string; packageId: string; version: string }>, componentSha256: string, descriptor: Uint8Array, componentReceipt: ComponentReceiptV1): TrustedBootstrapFixturePluginModuleV1 {
  const utf8 = (text: string) => Buffer.from(text, "utf8");
  const contents: [string, Uint8Array][] = [
    [`${identity.pluginId}/${MODULE_BRIDGE_FILE}`, utf8("export async function createActorApi() {}\n")],
    [`${identity.pluginId}/${TRUSTED_PLUGIN_MODULE_DESCRIPTOR_JSON_FILE}`, utf8(JSON.stringify({ manifest: { pluginId: identity.pluginId } }))],
    [`${identity.pluginId}/${TRUSTED_PLUGIN_MODULE_DESCRIPTOR_PACK_FILE}`, descriptor],
    [`${identity.pluginId}/${COMPONENT_RECEIPT_FILE}`, utf8(encodeComponentReceiptV1(componentReceipt))],
    [`${PREVIEW2_VENDOR_RELATIVE}/io.js`, utf8("export const streams = {};\n")],
  ];
  const blobs = new Map<string, Uint8Array>();
  const files = contents
    .sort(([left], [right]) => utf8OrderV1(left, right))
    .map(([path, bytes]) => {
      const sha256 = createHash("sha256").update(bytes).digest("hex");
      blobs.set(sha256, bytes);
      return { path, byteLength: bytes.byteLength, sha256, blake3: blake3Hex(bytes) };
    });
  const descriptorByteSha256 = createHash("sha256").update(descriptor).digest("hex");
  const manifest = encodeTrustedPluginModuleBundleV1({ schema: TRUSTED_PLUGIN_MODULE_SCHEMA, ...identity, sourceComponentSha256: componentSha256, sourceDescriptorByteSha256: descriptorByteSha256, moduleDirectory: identity.pluginId, entry: `${identity.pluginId}/${MODULE_BRIDGE_FILE}`, files });
  decodeTrustedPluginModuleBundleV1(manifest, { ...identity, componentSha256, descriptorByteSha256, receiptSha256: trustedBootstrapReceiptSha256(componentReceipt) });
  return Object.freeze({ record: Object.freeze({ path: trustedBootstrapPluginModuleManifestPath(identity.pluginId), byteLength: manifest.byteLength, sha256: createHash("sha256").update(manifest).digest("hex"), blake3: blake3Hex(manifest) }), manifest, blobs });
}

/** 🧩️ Writes one fixture plugin module beneath a generation root, replacing an earlier one's manifest. */
export function trustedBootstrapWriteFixturePluginModule(root: string, module: TrustedBootstrapFixturePluginModuleV1): void {
  mkdirSync(join(root, "plugin-modules"), { recursive: true });
  mkdirSync(dirname(join(root, module.record.path)), { recursive: true });
  writeFileSync(join(root, module.record.path), module.manifest);
  for (const [sha256, bytes] of module.blobs) writeFileSync(join(root, trustedBootstrapPluginModuleBlobPath(sha256)), bytes);
}

/** 🧾️ The shipped receipt a never-executed fixture component carries for exactly these bytes and this semantic descriptor. */
function trustedBootstrapFixtureReceipt(identity: Readonly<{ pluginId: string; packageId: string }>, component: Readonly<{ byteLength: number; sha256: string }>, descriptorSha256: string): ComponentReceiptV1 {
  return componentReceiptValueV1({ schema: "semio.plugin.component-receipt/v1", pluginId: identity.pluginId, packageId: identity.packageId, profile: "release", target: "wasm32-wasip2", wasmSha256: component.sha256, wasmBytes: component.byteLength, semanticDescriptorSha256: descriptorSha256, toolchain: "rustc 1.99.0-nightly (9f36de775bc636c8e88c31a173c2bcb6995956a0)", sourceTreeDigest: "3".repeat(64), provenanceSha256: "4".repeat(64) });
}

/** 🧫️ Constructs only neutral descriptor projections for physical publication-fence laws. */
export function trustedBootstrapStageFixturePackages(fixture: any, actor: any, component: Uint8Array): { packages: any[]; descriptors: Map<string, Buffer>; modules: Map<string, TrustedBootstrapFixturePluginModuleV1> } {
  const descriptors = new Map(["gis", "stdio"].map((plugin) => [plugin, Buffer.from(encodePackValue(fixture.descriptors[plugin]))]));
  const identity = (bytes: Uint8Array) => ({ byteLength: bytes.byteLength, sha256: createHash("sha256").update(bytes).digest("hex") });
  const records = ["gis", "stdio"].map((pluginId) => {
    const source = { component: identity(component), descriptor: identity(descriptors.get(pluginId)!) };
    return {
      pluginId, packageId: `semio:${pluginId}`, version: "0.1.0", ...source,
      receipt: trustedBootstrapFixtureReceipt({ pluginId, packageId: `semio:${pluginId}` }, source.component, source.descriptor.sha256),
      dependencies: pluginId === "gis" ? [{ pluginId: "stdio", packageId: "semio:stdio", version: "0.1.0" }] : [],
      executionProtocol: fixture.executionProtocol,
      browserActor: pluginId === "gis" ? { ...actor, path: "packages/gis/browser/closed-actor.mjs", sourceComponentSha256: source.component.sha256, sourceDescriptorByteSha256: source.descriptor.sha256 } : { kind: "none" },
    };
  });
  const modules = new Map(records.map((record) => [record.pluginId, trustedBootstrapFixturePluginModule(record, record.component.sha256, descriptors.get(record.pluginId)!, record.receipt)]));
  const packages = records.map((record) => ({ ...record, pluginModule: modules.get(record.pluginId)!.record }));
  return { packages, descriptors, modules };
}

/** ♾️ The repository's unlimited build budget (`SEMIO_BUILD_BUDGET_MS` unset, so [[BUILD_BUDGET_MS]] is `0`)
 * expressed inside the fresh-component process contract, whose own window is 1..86_400_000 ms and therefore
 * admits neither a zero nor an infinite deadline. */
export const UNLIMITED_TRUSTED_BUILD_DEADLINE_MS = 86_400_000;

/** 🔍️ Supplies bounded physical observation from the actual existing build owner. */
export function trustedBootstrapFileObservationControlV1(control:FreshBuildControlV1):FileObservationControlV1{return{maxBytes:128*1024*1024,maxWork:65536,chunkBytes:1024*1024,cancelled:()=>control.cancelled(),remainingMs:()=>control.remainingMs(),onProgress:step=>control.checkpoint("observe-file:"+step.phase,step.bytes,step.totalBytes)};}

/** ⏳️ Owns interrupt and progress observation for a bounded catalog build, reading a zero deadline as the
 * repository-wide unlimited budget rather than as one that already expired. */
export function trustedBootstrapBuildControl(deadlineMs: number): { control: FreshBuildControlV1; close(): void } {
  const deadline = deadlineMs > 0 ? Math.min(deadlineMs, UNLIMITED_TRUSTED_BUILD_DEADLINE_MS) : UNLIMITED_TRUSTED_BUILD_DEADLINE_MS;
  const diagnosticsRoot = process.env.SEMIO_TEST_ARTIFACT_DIR;
  let interrupted = false;
  const interrupt = (): void => {
    interrupted = true;
  };
  process.on("SIGINT", interrupt);
  process.on("SIGTERM", interrupt);
  const started = Date.now();
  return {
    control: Object.freeze({
      ...(diagnosticsRoot ? { diagnosticsRoot } : {}),
      cancelled: () => interrupted,
      remainingMs: () => Math.max(0, deadline - (Date.now() - started)),
      checkpoint(stage, completed, total) {
        console.log(`trusted-stdio-gis-bootstrap ${stage}: ${completed}/${total}`);
      },
    }),
    close() {
      process.off("SIGINT", interrupt);
      process.off("SIGTERM", interrupt);
    },
  };
}

/** 🎯️ The document-open targets one package's exact compiled descriptor declares, answered by the hub
 * binary that validates this generation (`os-hub trusted-catalog open-targets`, the Rust pairing rule
 * `descriptor_open_targets` in `🔏️trusted-catalog/🦀️.rs` that `validate_descriptor_open_target` also
 * applies), each bound to the package's own verified codec row. The rule has one implementation; this is
 * its caller. */
export async function trustedBootstrapHubOpenTargetsV1(host:LocalHubHostV1,phaseControl:CompilerAdmissionControl,pluginId:string,descriptor:Uint8Array,codecs:readonly TrustedBootstrapCodec[],unowned:ReadonlySet<string>,unbound:ReadonlySet<string>=new Set()):Promise<readonly TrustedBootstrapOpenTargetV1[]>{
  return withLocalHubHostTaskV1(host,phaseControl,async context=>{
  if(!descriptor.byteLength||descriptor.byteLength>DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES)throw Error("Bounded actual compiled descriptor required");
  const answer=await captureOwnedCommandResultV1(context.binaryPath,["trusted-catalog","open-targets"],context.root,{control:context.control,env:context.environment,input:{version:1,base64:Buffer.from(descriptor).toString("base64")},output:{maximumBytes:4194304,maximumLines:65536}});
  if(answer.status!==0||answer.signal!==null)throw Error(`hub open-target answer for ${pluginId} failed (${answer.status??answer.signal}): ${answer.stderr.trim().slice(0,512)}`);
  const document = documentOpenNeutralObject(JSON.parse(String(answer.stdout)), ["schema", "targets"]);
  if (document.schema !== "semio.hub.trusted-catalog-descriptor-open-targets/v1" || !Array.isArray(document.targets)) throw new Error(`hub open-target answer for ${pluginId} is outside its schema`);
  return Object.freeze(
    document.targets.filter((value: any) => !unowned.has(String(value?.artifactKind)) && !unbound.has(`${String(value?.artifactKind)} ${String(value?.artifactSchema)}`)).map((value: unknown) => {
      const row = documentOpenNeutralObject(value, ["artifactKind", "artifactSchema", "surfaceId", "appId", "windowKindId", "role", "rendererTarget", "parentDialect", "grant"]);
      const codec = codecs.find((candidate) => candidate.artifactKind === row.artifactKind && candidate.artifactSchema === row.artifactSchema);
      if (!codec) throw new Error(`trusted bootstrap open target ${row.artifactKind} has no verified codec row`);
      const dialect = documentOpenNeutralObject(row.parentDialect, ["artifactKind", "standard", "subset"]);
      const grant = documentOpenNeutralObject(row.grant, ["read", "write", "observe"]);
      return Object.freeze({
        pluginId,
        artifactKind: String(row.artifactKind),
        artifactSchema: String(row.artifactSchema),
        packSchemaHash: codec.packSchemaHash,
        surfaceId: String(row.surfaceId),
        appId: String(row.appId),
        windowKindId: String(row.windowKindId),
        role: row.role === "viewer" ? ("viewer" as const) : ("editor" as const),
        rendererTarget: "wasm" as const,
        parentDialect: Object.freeze({ artifactKind: String(dialect.artifactKind), standard: String(dialect.standard), subset: String(dialect.subset) }),
        grant: Object.freeze({ read: grant.read === true, write: grant.write === true, observe: grant.observe === true }),
      });
    }),
  );
  });
}
