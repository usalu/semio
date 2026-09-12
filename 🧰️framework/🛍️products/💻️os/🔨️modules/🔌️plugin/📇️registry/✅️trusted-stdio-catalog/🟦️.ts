import { createHash } from "node:crypto";
import { closeSync, existsSync, fstatSync, lstatSync, mkdirSync, mkdtempSync, openSync, readdirSync, readFileSync, readSync, realpathSync, rmSync, statSync, writeFileSync } from "node:fs";
import { isAbsolute, basename, dirname, join, relative, resolve, sep } from "node:path";
import { authorArtifactScaffold, BundleScript, canonicalPrimaryFilenameForKind, discoverCatalogPackages, discoverPackageProblems, discoverPackages, getWorkspaceRoot, inspectRustModuleGraph, inspectRustModuleGraphFacts, loadCatalogTaxonomy, parseRegistryCatalogProjection, registryCatalogInputView, registryCatalogProjectedInputView, registryExampleCatalog, runBundleScriptMain, runVitest, resolveTestLevel, ScriptRouter, validateGeneratorContractsAgainstWorkspace } from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { CATALOG_ARTIFACT_MAX_BYTES } from "../✅️catalog-verification/🟦️.ts";
import { publicationWasmPath } from "../🛂️descriptor-verification/🟦️.ts";
import { PLUGIN_AREAS, PluginRegistryEntry } from "../🔎️discovery/🟦️.ts";




export type TrustedStdioNativeCodecV1 = {
  readonly artifactKind: string;
  readonly artifactSchema: string;
  readonly packSchemaHash: string;
  readonly factoryId: string;
  readonly extension: string;
};


export type TrustedStdioNativeCodecReceiptV1 = TrustedStdioNativeCodecV1 & {
  readonly protocolPath: string;
};


export type TrustedStdioOpenTargetV1 = TrustedStdioNativeCodecV1 & {
  readonly role: "editor";
  readonly surfaceId: "stdio.json.editor";
};


export type TrustedStdioComponentAdmissionV1 = {
  readonly status: "admitted" | "exceeds-limit" | "absent";
  readonly limitBytes: number;
  readonly byteLength?: number;
  readonly path?: string;
};


export type TrustedStdioCatalogV1 = {
  readonly schemaVersion: 1;
  readonly pluginId: "stdio";
  readonly packageId: "semio:stdio";
  readonly version: string;
  readonly nativeCodecs: readonly TrustedStdioNativeCodecV1[];
  readonly openTargets: readonly TrustedStdioOpenTargetV1[];
  readonly publication: "committed" | "withheld" | "failed";
  readonly hubBundle: "withheld";
  readonly componentAdmission: TrustedStdioComponentAdmissionV1;
};


export type TrustedStdioCatalogPublicationV1 = {
  readonly publication: TrustedStdioCatalogV1["publication"];
  readonly catalog: TrustedStdioCatalogV1;
  readonly catalogSha256: string;
  readonly outPath: string;
};


export const TRUSTED_STDIO_FACTORY_SCHEMA = "semio.stdio.native-openable-catalog-provider/v1";

export const TRUSTED_STDIO_VERSION = "0.1.0";

export const TRUSTED_STDIO_CATALOG_FILENAME = "trusted-stdio-catalog.json";


/** 🗄️ Resolves the stdio owner root from the discovered Cargo crate path. */
export function stdioOwnerRoot(repoRoot: string, entries?: readonly PluginRegistryEntry[]): string {
  const entry = entries?.find((row) => row.pluginId === "stdio");
  if (entry) return resolve(repoRoot, entry.cratePath, "..", "..");
  for (const area of PLUGIN_AREAS) {
    const areaRoot = join(repoRoot, area);
    if (!existsSync(areaRoot)) continue;
    const stdio = readdirSync(areaRoot).find((name) => name.includes("stdio"));
    if (stdio) return join(areaRoot, stdio);
  }
  throw new Error("stdio plugin crate is absent");
}


/** 🗄️ Locates the first-party native-codec factory manifest under the stdio registry. */
export function stdioNativeCodecFactoriesPath(stdioRoot: string): string {
  const registry = readdirSync(stdioRoot).find((name) => name.endsWith("registry"));
  if (!registry) throw new Error("stdio registry directory is absent");
  const registryDir = join(stdioRoot, registry);
  const file = readdirSync(registryDir).find((name) => name.endsWith("native-codec-factories.json"));
  if (!file) throw new Error("stdio native-codec-factories.json is absent");
  return join(registryDir, file);
}


/** 🗄️ Reads first-party stdio native codec receipts without trusting generated catalog files. */
export function loadFirstPartyStdioNativeCodecReceipts(repoRoot = getWorkspaceRoot(), entries?: readonly PluginRegistryEntry[]): TrustedStdioNativeCodecReceiptV1[] {
  const stdioRoot = stdioOwnerRoot(repoRoot, entries);
  const factoriesPath = stdioNativeCodecFactoriesPath(stdioRoot);
  const document = JSON.parse(readFileSync(factoriesPath, "utf8")) as {
    readonly schema?: unknown;
    readonly plugin_id?: unknown;
    readonly package_id?: unknown;
    readonly receipts?: readonly Record<string, unknown>[];
  };
  if (document.schema !== TRUSTED_STDIO_FACTORY_SCHEMA || document.plugin_id !== "stdio" || document.package_id !== "semio:stdio") {
    throw new Error("stdio native-codec-factories.json identity is not first-party");
  }
  if (!Array.isArray(document.receipts)) throw new Error("empty native codec catalog");
  const receipts: TrustedStdioNativeCodecReceiptV1[] = [];
  for (const row of document.receipts) {
    if (typeof row.artifact_kind !== "string" || typeof row.document_schema !== "string" || typeof row.pack_schema_sha256 !== "string" || typeof row.factory_id !== "string" || typeof row.extension !== "string" || typeof row.protocol_path !== "string") {
      throw new Error("stdio native codec receipt is incomplete");
    }
    receipts.push({
      artifactKind: row.artifact_kind,
      artifactSchema: row.document_schema,
      packSchemaHash: row.pack_schema_sha256,
      factoryId: row.factory_id,
      extension: row.extension,
      protocolPath: resolve(stdioRoot, row.protocol_path),
    });
  }
  return receipts;
}


/** 🗄️ Admits only first-party stdio native codec receipts whose protocol SHA-256 matches Node crypto. */
export function verifyStdioNativeCodecReceipts(receipts: readonly TrustedStdioNativeCodecReceiptV1[]): TrustedStdioNativeCodecV1[] {
  if (receipts.length === 0) throw new Error("empty native codec catalog");
  const seen = new Set<string>();
  const codecs: TrustedStdioNativeCodecV1[] = [];
  for (const row of receipts) {
    if (!/^[0-9a-f]{64}$/.test(row.packSchemaHash) || row.packSchemaHash === "0".repeat(64)) throw new Error(`${row.artifactKind}: pack schema digest is missing`);
    if (seen.has(row.artifactKind)) throw new Error(`${row.artifactKind}: duplicate native codec`);
    seen.add(row.artifactKind);
    if (!existsSync(row.protocolPath)) throw new Error(`${row.artifactKind}: protocol file is missing`);
    const info = lstatSync(row.protocolPath);
    if (info.isSymbolicLink() || !info.isFile()) throw new Error(`${row.artifactKind}: protocol must be a regular non-symlink file`);
    const digest = createHash("sha256").update(readFileSync(row.protocolPath)).digest("hex");
    if (digest !== row.packSchemaHash) throw new Error(`${row.artifactKind}: pack schema digest mismatch`);
    codecs.push({ artifactKind: row.artifactKind, artifactSchema: row.artifactSchema, packSchemaHash: row.packSchemaHash, factoryId: row.factoryId, extension: row.extension });
  }
  codecs.sort((left, right) => (left.artifactKind < right.artifactKind ? -1 : left.artifactKind > right.artifactKind ? 1 : 0));
  return codecs;
}


/** 🗄️ Reports whether a stdio component can be admitted under the 64 MiB trusted-catalog bound. */
export function stdioComponentAdmission(repoRoot: string): TrustedStdioComponentAdmissionV1 {
  const limitBytes = CATALOG_ARTIFACT_MAX_BYTES;
  const candidates: string[] = [];
  const publication = publicationWasmPath(repoRoot, "semio_s_plugin_stdio.wasm");
  if (existsSync(publication)) candidates.push(publication);
  const development = resolve(repoRoot, "🧰️framework", "🛍️products", "💻️os", "🔨️modules", "🧑‍💻dev");
  if (existsSync(development)) {
    const modules = readdirSync(development).find((name) => name.includes("plugin-modules"));
    if (modules) {
      const stdioDir = readdirSync(join(development, modules)).find((name) => name.includes("stdio"));
      if (stdioDir) {
        for (const name of readdirSync(join(development, modules, stdioDir))) {
          if (name.endsWith(".wasm")) candidates.push(join(development, modules, stdioDir, name));
        }
      }
    }
  }
  if (candidates.length === 0) return { status: "absent", limitBytes };
  const chosen = candidates.map((path) => ({ path, byteLength: lstatSync(path).size })).sort((left, right) => right.byteLength - left.byteLength)[0]!;
  const path = relative(repoRoot, chosen.path).replaceAll("\\", "/");
  if (chosen.byteLength > limitBytes) return { status: "exceeds-limit", limitBytes, byteLength: chosen.byteLength, path };
  return { status: "admitted", limitBytes, byteLength: chosen.byteLength, path };
}


/** 🗄️ Builds the nonempty first-party stdio native-codec catalog without inventing hashes or a hub bundle. */
export function buildTrustedStdioCatalogV1(repoRoot = getWorkspaceRoot(), entries?: readonly PluginRegistryEntry[], receipts?: readonly TrustedStdioNativeCodecReceiptV1[]): TrustedStdioCatalogV1 {
  const codecs = verifyStdioNativeCodecReceipts(receipts ?? loadFirstPartyStdioNativeCodecReceipts(repoRoot, entries));
  if (codecs.length !== 26) throw new Error(`stdio native codec catalog must contain 26 receipts, found ${codecs.length}`);
  const json = codecs.find((row) => row.artifactKind === "stdio.json");
  if (!json || json.factoryId !== "stdio.native.json.v1") throw new Error("stdio.json native codec is absent");
  return {
    schemaVersion: 1,
    pluginId: "stdio",
    packageId: "semio:stdio",
    version: TRUSTED_STDIO_VERSION,
    nativeCodecs: codecs,
    openTargets: [{ ...json, role: "editor", surfaceId: "stdio.json.editor" }],
    publication: "committed",
    hubBundle: "withheld",
    componentAdmission: stdioComponentAdmission(repoRoot),
  };
}


/** 🗄️ Canonical catalog bytes hashed by the Node crypto publication oracle. */
export function encodeTrustedStdioCatalogV1(catalog: TrustedStdioCatalogV1): string {
  return `${JSON.stringify(catalog, null, 2)}\n`;
}


/** 🗄️ Writes a verified nonempty stdio catalog and returns the Node crypto receipt. */
export function publishTrustedStdioCatalogV1(options: { readonly outDir: string; readonly repoRoot?: string; readonly receipts?: readonly TrustedStdioNativeCodecReceiptV1[] }): TrustedStdioCatalogPublicationV1 {
  const repoRoot = options.repoRoot ?? getWorkspaceRoot();
  const catalog = buildTrustedStdioCatalogV1(repoRoot, undefined, options.receipts);
  const encoded = encodeTrustedStdioCatalogV1(catalog);
  mkdirSync(options.outDir, { recursive: true });
  const outPath = join(options.outDir, TRUSTED_STDIO_CATALOG_FILENAME);
  writeFileSync(outPath, encoded);
  return { publication: catalog.publication, catalog, catalogSha256: createHash("sha256").update(encoded).digest("hex"), outPath };
}


/** 🗄️ Publishes the first-party stdio native-codec catalog without claiming hub bundle admission. */
export class TrustedCatalogPublishScript extends BundleScript {
  run(segments: string[]): void {
    const option = (name: string): string | undefined => {
      const index = segments.indexOf(name);
      return index < 0 ? undefined : segments[index + 1];
    };
    const outDir = option("--out") ?? join(this.root, "🤖️generated");
    const receipt = publishTrustedStdioCatalogV1({ outDir, repoRoot: getWorkspaceRoot() });
    console.log(`trusted stdio catalog ${receipt.publication}: ${receipt.catalog.nativeCodecs.length} codecs, open=${receipt.catalog.openTargets[0]?.artifactKind}, hubBundle=${receipt.catalog.hubBundle}, component=${receipt.catalog.componentAdmission.status}, sha256=${receipt.catalogSha256} -> ${receipt.outPath}`);
  }
}
