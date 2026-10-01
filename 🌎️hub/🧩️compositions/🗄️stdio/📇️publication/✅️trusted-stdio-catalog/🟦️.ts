import { createHash } from "node:crypto";
import { existsSync, lstatSync, mkdirSync, readdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import { BundleScript, getWorkspaceRoot } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { publicationWasmPath } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🛂️descriptor-verification/🟦️.ts";

const CATALOG_ARTIFACT_MAX_BYTES = 64 * 1024 * 1024;

export type TrustedStdioNativeCodecV1 = {
  readonly artifactKind: string;
  readonly artifactSchema: string;
  readonly packSchemaHash: string;
  readonly protocolSourceSha256: string;
  readonly factoryId: string;
  readonly extension: string;
};


export type TrustedStdioNativeCodecReceiptV1 = TrustedStdioNativeCodecV1 & {
  readonly protocolPath: string;
  readonly artifactDefinitionPath: string;
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


/** 🗄️ Resolves this concrete publication owner's source root in the selected workspace. */
export function stdioOwnerRoot(repoRoot: string): string {
  return resolve(repoRoot, relative(getWorkspaceRoot(), resolve(import.meta.dirname, "../..")));
}

/** 🗄️ Locates the authored native-codec projection at the concrete Stdio composition owner. */
export function stdioNativeCodecFactoriesPath(stdioRoot: string): string {
  const catalogDir = join(stdioRoot, "🔌️plugin", "📇️catalog");
  const file = readdirSync(catalogDir).find((name) => name.endsWith("native-codec-factories.json"));
  if (!file) throw new Error("stdio native-codec-factories.json is absent");
  return join(catalogDir, file);
}


/** 🗄️ Reads first-party stdio native codec receipts without trusting generated catalog files. */
export function loadFirstPartyStdioNativeCodecReceipts(repoRoot = getWorkspaceRoot()): TrustedStdioNativeCodecReceiptV1[] {
  const stdioRoot = stdioOwnerRoot(repoRoot);
  const factoriesPath = stdioNativeCodecFactoriesPath(stdioRoot);
  const factoryInfo = lstatSync(factoriesPath);
  if (!factoryInfo.isFile() || factoryInfo.isSymbolicLink() || factoryInfo.size > 64 * 1024) throw new Error("stdio native codec receipts must be a bounded regular non-symlink file");
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
  const receiptFields = ["artifact", "factory_id", "descriptor_codec_id", "runtime_capability_id", "artifact_kind", "artifact_schema", "extension", "pack_schema_hash", "protocol_source_sha256", "protocol_path", "definition_path"].sort();
  for (const row of document.receipts) {
    if (!row || Array.isArray(row) || JSON.stringify(Object.keys(row).sort()) !== JSON.stringify(receiptFields)) throw new Error("stdio native codec receipt fields are not closed");
    if (typeof row.artifact_kind !== "string" || typeof row.artifact_schema !== "string" || typeof row.pack_schema_hash !== "string" || typeof row.factory_id !== "string" || typeof row.extension !== "string" || typeof row.protocol_path !== "string" || typeof row.protocol_source_sha256 !== "string" || typeof row.definition_path !== "string") {
      throw new Error("stdio native codec receipt is incomplete");
    }
    if (!/^✏️s\/🔌️plugins\/🗄️stdio\/🗿️artifacts\/[^/]+\/📜️artifact-definition\.json$/u.test(row.definition_path) || !row.protocol_path.startsWith(`${row.definition_path.slice(0, row.definition_path.lastIndexOf("/"))}/`) || !row.protocol_path.endsWith("/📡️.protocol.semio") || row.protocol_path.split("/").some((segment) => segment === "." || segment === "..")) throw new Error("stdio native codec receipt source ownership is invalid");
    receipts.push({
      artifactKind: row.artifact_kind,
      artifactSchema: row.artifact_schema,
      packSchemaHash: row.pack_schema_hash,
      protocolSourceSha256: row.protocol_source_sha256,
      factoryId: row.factory_id,
      extension: row.extension,
      protocolPath: resolve(repoRoot, row.protocol_path),
      artifactDefinitionPath: resolve(repoRoot, row.definition_path),
    });
  }
  return receipts;
}


/** 🗄️ Admits only first-party stdio native codec receipts with separate declared pack identities and independently verified protocol source SHA-256. */
export function verifyStdioNativeCodecReceipts(receipts: readonly TrustedStdioNativeCodecReceiptV1[]): TrustedStdioNativeCodecV1[] {
  if (receipts.length === 0 || receipts.length > 1024) throw new Error("empty native codec catalog");
  const seen = new Set<string>();
  const codecs: TrustedStdioNativeCodecV1[] = [];
  for (const row of receipts) {
    if (!/^[0-9a-f]{64}$/.test(row.packSchemaHash) || row.packSchemaHash === "0".repeat(64)) throw new Error(`${row.artifactKind}: pack schema digest is missing`);
    if (seen.has(row.factoryId)) throw new Error(`${row.artifactKind}: duplicate native codec`);
    seen.add(row.factoryId);
    if (!existsSync(row.protocolPath)) throw new Error(`${row.artifactKind}: protocol file is missing`);
    const info = lstatSync(row.protocolPath);
    if (info.isSymbolicLink() || !info.isFile() || info.size > 1024 * 1024) throw new Error(`${row.artifactKind}: protocol must be a regular non-symlink file`);
    const digest = createHash("sha256").update(readFileSync(row.protocolPath)).digest("hex");
    if (digest !== row.protocolSourceSha256) throw new Error(`${row.artifactKind}: protocol source digest mismatch`);
    if (!existsSync(row.artifactDefinitionPath)) throw new Error(`${row.artifactKind}: artifact definition file is missing`);
    const definitionInfo = lstatSync(row.artifactDefinitionPath);
    if (definitionInfo.isSymbolicLink() || !definitionInfo.isFile() || definitionInfo.size > 1024 * 1024) throw new Error(`${row.artifactKind}: artifact definition must be a bounded regular non-symlink file`);
    const definition = JSON.parse(readFileSync(row.artifactDefinitionPath, "utf8")) as { readonly id?: unknown; readonly codecs?: readonly { readonly native_factory?: Record<string, unknown> }[] };
    const factories = definition.codecs?.filter((codec) => codec.native_factory?.factory_id === row.factoryId) ?? [];
    const factory = factories[0]?.native_factory;
    if (definition.id !== row.artifactKind || factories.length !== 1 || factory?.artifact_kind !== row.artifactKind || factory?.artifact_schema !== row.artifactSchema || factory?.extension !== row.extension || factory?.pack_schema_hash !== row.packSchemaHash) throw new Error(`${row.artifactKind}: pack schema identity mismatch`);
    codecs.push({ artifactKind: row.artifactKind, artifactSchema: row.artifactSchema, packSchemaHash: row.packSchemaHash, protocolSourceSha256: row.protocolSourceSha256, factoryId: row.factoryId, extension: row.extension });
  }
  codecs.sort((left, right) => { const a = [left.artifactKind, left.artifactSchema, left.factoryId].join("\0"), b = [right.artifactKind, right.artifactSchema, right.factoryId].join("\0"); return a < b ? -1 : a > b ? 1 : 0; });
  return codecs;
}


/** 🗄️ Reports whether a stdio component can be admitted under the 64 MiB trusted-catalog bound. */
export function stdioComponentAdmission(repoRoot: string): TrustedStdioComponentAdmissionV1 {
  const limitBytes = CATALOG_ARTIFACT_MAX_BYTES;
  const candidates: string[] = [];
  const publication = publicationWasmPath(repoRoot, "semio_hub_stdio.wasm");
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
export function buildTrustedStdioCatalogV1(repoRoot = getWorkspaceRoot(), receipts?: readonly TrustedStdioNativeCodecReceiptV1[]): TrustedStdioCatalogV1 {
  const codecs = verifyStdioNativeCodecReceipts(receipts ?? loadFirstPartyStdioNativeCodecReceipts(repoRoot));
  const json = codecs.find((row) => row.artifactKind === "s.stdio.json");
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
  const catalog = buildTrustedStdioCatalogV1(repoRoot, options.receipts);
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
