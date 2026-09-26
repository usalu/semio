/** 🔍️ How a hub document's plugin module is resolved (`🌎️hub-source/🔍️resolution`), replayed from the language-agnostic
 * fixture `🌎️hub-source/🧫️fixtures/🔍️resolution/🔣️.json`: hub program ids, staged-module roots, the source decision
 * (a staged module whose every file matches the catalog's bundle → local, one that differs or is missing → hub, a
 * complete stored copy → store), the owner of a dialect and the same-generation closure of a hub program. Ajv and
 * `node:crypto` are the independent oracles: Ajv for every id and index, SHA-256 for every staged-file comparison. */
import { describe, expect, it } from "vitest";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import Ajv from "ajv";
import { decodeTrustedPluginModuleBundleV1, PLUGIN_MODULE_SOURCES_V1, trustedPluginModuleSourceOfEntryV1, validateTrustedPluginModuleIndexV1, type TrustedPluginModuleFileV1 } from "../../🔨️modules/🔌️plugin/📇️registry/🌎️hub-source/🧬️schema/🟦️.ts";
import {
  hubCatalogClosureV1,
  hubCatalogOnlyPluginsV1,
  hubCatalogOwnerOfDialectV1,
  hubProgramIdV1,
  localPluginModuleRootV1,
  parseHubProgramIdV1,
  resolvePluginModuleSourceV1,
} from "../../🔨️modules/🔌️plugin/📇️registry/🌎️hub-source/🔍️resolution/🟦️.ts";

const here = (path: string) => JSON.parse(readFileSync(fileURLToPath(new URL(path, import.meta.url)), "utf8"));
const fixture = here("../../🔨️modules/🔌️plugin/📇️registry/🌎️hub-source/🧫️fixtures/🔍️resolution/🔣️.json");
const storeSchema = here("../../🔨️modules/🔌️plugin/📇️registry/🌎️hub-source/🧬️schema/🔣️.json");
const hubSchema = here("../../../../../🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧬️schema/🔣️.json");
const hexBytes = (value: string) => Uint8Array.from(value.match(/../gu) ?? [], (pair) => Number.parseInt(pair, 16));
const ajv = new Ajv({ strict: false, allErrors: true }).addSchema({ ...hubSchema, $ref: undefined }).addSchema(storeSchema);
const programIdOracle = ajv.getSchema(`${storeSchema.$id}#/$defs/HubProgramIdV1`)!;
const indexOracle = ajv.getSchema(`${hubSchema.$id}#/$defs/TrustedPluginModuleIndexV1`)!;
const bundle = decodeTrustedPluginModuleBundleV1(new TextEncoder().encode(fixture.bundle.manifestUtf8), trustedPluginModuleSourceOfEntryV1(fixture.bundle.entry));

/** 🧮️ The oracle's own decision: SHA-256 of every staged file against the manifest, by `node:crypto`. */
function oracleSource(testCase: any): string {
  if (testCase.stored === "complete") return "store";
  if (testCase.local === null || localPluginModuleRootV1(testCase.local.moduleUrl, bundle.entry) === null) return "hub";
  const matches = bundle.files.every((file) => {
    const hex = testCase.local.files[file.path];
    return typeof hex === "string" && hex.length / 2 === file.byteLength && createHash("sha256").update(hexBytes(hex)).digest("hex") === file.sha256;
  });
  return matches ? "local" : "hub";
}

describe("🔍️ hub program resolution", () => {
  it("names a hub program by plugin and bundle, and reads no other program id as one", () => {
    expect([...PLUGIN_MODULE_SOURCES_V1]).toEqual(["store", "local", "hub"]);
    for (const row of fixture.programIds) {
      expect(hubProgramIdV1(row.pluginId, row.bundleSha256)).toBe(row.programId);
      expect(parseHubProgramIdV1(row.programId)).toEqual({ pluginId: row.pluginId, bundleSha256: row.bundleSha256 });
      expect(programIdOracle(row.programId), row.programId).toBe(true);
    }
    for (const programId of fixture.hostileProgramIds) {
      expect(parseHubProgramIdV1(programId), programId).toBe(null);
      expect(programIdOracle(programId), programId).toBe(false);
    }
  });

  it("finds the root a bundle's paths resolve against inside a staged module, only when the staged entry is the bundle's", () => {
    for (const row of fixture.roots) expect(localPluginModuleRootV1(row.moduleUrl, row.entry), row.moduleUrl).toBe(row.root);
  });

  for (const testCase of fixture.sourceCases) {
    it(`resolves ${testCase.id} to ${testCase.source}`, async () => {
      expect(oracleSource(testCase), "the oracle agrees with the fixture").toBe(testCase.source);
      const root = testCase.local === null ? null : localPluginModuleRootV1(testCase.local.moduleUrl, bundle.entry);
      const reads: string[] = [];
      const readLocal = root === null ? null : async (file: TrustedPluginModuleFileV1) => {
        reads.push(file.path);
        const hex = testCase.local.files[file.path];
        return typeof hex === "string" ? hexBytes(hex) : null;
      };
      let verified = 0;
      const resolution = await resolvePluginModuleSourceV1({ bundle, storedComplete: testCase.stored === "complete", readLocal, signal: new AbortController().signal, onVerified: (bytes) => (verified += bytes) });
      expect(resolution.source).toBe(testCase.source);
      if (testCase.stored === "complete") expect(reads).toEqual([]);
      if (resolution.source === "local") {
        expect(resolution.files.map(([file]) => file.path).sort()).toEqual(bundle.files.map((file) => file.path).sort());
        for (const [file, bytes] of resolution.files) expect(createHash("sha256").update(bytes).digest("hex")).toBe(file.sha256);
        expect(verified).toBe(bundle.files.reduce((sum, file) => sum + file.byteLength, 0));
      }
      if (resolution.source === "hub" && readLocal !== null) expect(reads.length, "stops at the first staged file that is absent or differs").toBeLessThanOrEqual(bundle.files.length);
    });
  }

  it("stops comparing a staged module on cancellation", async () => {
    const abort = new AbortController();
    abort.abort();
    await expect(resolvePluginModuleSourceV1({ bundle, storedComplete: false, readLocal: async () => null, signal: abort.signal })).rejects.toThrow();
  });

  it("resolves the owner of a dialect and the same-generation closure of a hub program", () => {
    expect(indexOracle(fixture.index), JSON.stringify(indexOracle.errors)).toBe(true);
    const index = validateTrustedPluginModuleIndexV1(fixture.index);
    for (const row of fixture.ownerCases) expect(hubCatalogOwnerOfDialectV1(index, row.artifactKind)?.pluginId ?? null, row.artifactKind).toBe(row.owner);
    for (const row of fixture.closureCases) expect(hubCatalogClosureV1(index, row.pluginId)?.map((entry) => entry.pluginId) ?? null, row.id).toEqual(row.closure);
  });

  it("lists every hub plugin this device's build lacks, never an extension, pinned to its generation's bundle", () => {
    const index = validateTrustedPluginModuleIndexV1(fixture.index);
    for (const row of fixture.catalogOnlyCases) {
      const rows = hubCatalogOnlyPluginsV1(index, new Set(row.local));
      expect(rows.map((entry) => entry.pluginId), row.id).toEqual(row.hubOnly);
      for (const entry of rows) {
        const indexed = index.modules.find((module) => module.pluginId === entry.pluginId)!;
        expect([entry.generationId, entry.bundleSha256, entry.dependencies], row.id).toEqual([index.generationId, indexed.bundleSha256, indexed.dependencies]);
        expect(indexed.extendsPluginId, `${row.id}: ${entry.pluginId} is a plugin`).toBeNull();
      }
    }
  });
});
