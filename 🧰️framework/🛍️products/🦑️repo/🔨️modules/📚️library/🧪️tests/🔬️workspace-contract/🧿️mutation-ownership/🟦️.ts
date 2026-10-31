import Ajv, { type AnySchema } from "ajv";
import { describe, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { loadCatalogTaxonomy } from "../../../🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
import { inspectRustMutationMetadataFacts } from "../../../🔍️discovery/🏛️rust-structure/🟦️.ts";
import { contractRoot } from "../🏗️shared-fixtures/🟦️.ts";
import { registerMutationRootDiscoveryProjectsTests } from "./🔶️mutation-root-discovery-projects/🟦️.ts";
import { registerRecognizesBinaryLeafTagTests } from "./🔸️recognizes-binary-leaf-tag/🟦️.ts";
import { registerRequiresOneEnabledReachableTests } from "./🟠️requires-one-enabled-reachable/🟦️.ts";
import { registerRequiresFreshCleanTerminalTests } from "./🟢️requires-fresh-clean-terminal/🟦️.ts";

/** 🧷️ Admits one authored scenario file as the JSON Schema document its row declares. */
function schemaFile(files: Readonly<Record<string, unknown>>, path: string): AnySchema {
  const body = files[path];
  if (typeof body === "boolean" || (body !== null && typeof body === "object")) return body as AnySchema;
  throw Error(`scenario file ${path} is not a JSON Schema document`);
}

describe("direct mutation ownership", () => {
  registerMutationRootDiscoveryProjectsTests();
  registerRecognizesBinaryLeafTagTests();
  registerRequiresOneEnabledReachableTests();
  registerRequiresFreshCleanTerminalTests();
});

describe("mutation metadata facts", () => {
  test("records exact declaration and alias syntax without decoy inference", () => {
    const fixturePath = join(contractRoot, "../🧫️fixtures/🪪️mutation-metadata/🔣️.json");
    const fixture = JSON.parse(readFileSync(fixturePath, "utf8"));
    
    
    const facts = inspectRustMutationMetadataFacts(fixture.source);
    expect(facts.declarations.map(({ name, kind, visibility, modulePath, derives, mutationLeaf }) => ({ name, kind, visibility, modulePath, derives, state: mutationLeaf.state, contracts: mutationLeaf.contracts, detail: mutationLeaf.detail }))).toEqual(fixture.expected.declarations);
    expect(facts.crateAliases).toEqual(fixture.expected.aliases);
    expect(facts.manualMutationLeafImpls).toEqual(fixture.expected.manualMutationLeafImpls);
    expect(facts.declarations.some((item) => item.name === "Fake")).toBe(false);
  });
});

describe("schema scope catalog", () => {
    const casesPath = join(contractRoot, "../🧫️fixtures/🧬️schema-scope-catalog/🔣️.json");
    const cases = JSON.parse(readFileSync(casesPath, "utf8")) as { contract: string; cases: { id: string; files: Record<string, unknown>; expected: { scopes: Record<string, { path: string; level: string; exports: Record<string, { file: string; facet: string }>; dependsOn: string[] }>; diagnosticCodes: string[]; diagnosticPaths?: string[]; placementPaths: string[] }; schemaGrammar?: Record<string, boolean>; schemaCompile?: Record<string, boolean>; grantExamples?: { input: unknown; expected: boolean }[]; typescriptExports?: Record<string, Record<string, boolean>> }[] };
    const authority = JSON.parse(readFileSync(join(contractRoot, "../../🧬️schema/🧬️schema-scope-catalog/🔣️.json"), "utf8"));
    const ajv = new Ajv({ strict: true });
    const admitScope = ajv.compile(authority);
    const grantAuthority = JSON.parse(readFileSync(join(contractRoot, "../../../../../../🔨️modules/🌱️value/🗂️ordered/♻️retirement/🧬️schema/🔣️.json"), "utf8"));
    ajv.addSchema(grantAuthority);
    const admitGrant = ajv.compile({ $ref: grantAuthority.$id + "#/$defs/Grant" });
    
    for (const row of cases.cases) test("catalogues declared scopes and reports every retired placement and unresolved binding: "+row.id, async () => {
      const { inventorySchemaScopes, renderSchemaCatalog } = await import("../../../🔍️discovery/🧾️schema-scope-catalog/🟦️.ts");
      const taxonomy = loadCatalogTaxonomy();
      const root = mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR ?? tmpdir(), "s"));
      try {
        for (const [rel, body] of Object.entries(row.files)) {
          const abs = join(root, rel);
          expect(abs.length, row.id).toBeLessThanOrEqual(256);
          expect(Array.from(abs).length, row.id).toBeLessThanOrEqual(256);
          mkdirSync(dirname(abs), { recursive: true });
          writeFileSync(abs, typeof body === "string" ? body : `${JSON.stringify(body, null, 2)}\n`);
        }
        for (const [path, expected] of Object.entries(row.schemaGrammar ?? {})) expect(new Ajv({ strict: false }).validateSchema(schemaFile(row.files, path)), row.id).toBe(expected);
        for (const [path, expected] of Object.entries(row.schemaCompile ?? {})) {
          let accepted = true;
          try { new Ajv({ strict: true, strictTypes: false }).compile(schemaFile(row.files, path)); } catch { accepted = false; }
          expect(accepted, row.id).toBe(expected);
        }
        for (const [path, declarations] of Object.entries(row.typescriptExports ?? {})) {
          const ts = await import("typescript"), source = ts.createSourceFile(path, String(row.files[path] ?? ""), ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
          const names = new Set(source.statements.flatMap(statement => ts.isInterfaceDeclaration(statement) || ts.isTypeAliasDeclaration(statement) || ts.isClassDeclaration(statement) || ts.isEnumDeclaration(statement) || ts.isFunctionDeclaration(statement) ? statement.name ? [statement.name.text] : [] : []));
          for (const [name, present] of Object.entries(declarations)) expect(names.has(name), row.id + ": " + path + "/" + name).toBe(present);
        }
        for (const example of row.grantExamples ?? []) expect(admitGrant(example.input), JSON.stringify(admitGrant.errors)).toBe(example.expected);
        const inventory = inventorySchemaScopes(root, taxonomy);
        const scopes = Object.fromEntries(Object.entries(inventory.catalog.scopes).map(([id, scope]) => [id, { path: scope.path, level: scope.level, exports: scope.exports, dependsOn: [...scope.dependsOn] }]));
        for (const scope of Object.values(scopes)) expect(admitScope(scope), JSON.stringify(admitScope.errors)).toBe(true);
        expect({ case: row.id, scopes }).toEqual({ case: row.id, scopes: row.expected.scopes });
        expect({ case: row.id, codes: inventory.diagnostics.map(({ code }) => code).sort() }).toEqual({ case: row.id, codes: [...row.expected.diagnosticCodes].sort() });
        if (row.expected.diagnosticPaths) expect(inventory.diagnostics.map(({ path }) => path).sort()).toEqual([...row.expected.diagnosticPaths].sort());
        expect({ case: row.id, paths: inventory.placement.map(({ path }) => path).sort() }).toEqual({ case: row.id, paths: [...row.expected.placementPaths].sort() });
        expect(renderSchemaCatalog(inventory.catalog)).toBe(renderSchemaCatalog(inventorySchemaScopes(root, taxonomy).catalog));
      } finally {
        rmSync(root, { recursive: true, force: true });
      }
      if (row === cases.cases.at(-1)) {}
    });

  test("resolves scope ids and export bindings only from declared identity", async () => {
    const { schemaScopeIdFromDocumentId, parseSchemaExportUri, schemaScopeOwnerLevel } = await import("../../../🔍️discovery/🧾️schema-scope-catalog/🟦️.ts"), { resolveSchemaFacetKind } = await import("../../../🔍️discovery/⚖️fixed-contract-resolution/🟦️.ts");
    const taxonomy = loadCatalogTaxonomy();
    expect(schemaScopeIdFromDocumentId("https://json.schemas.assets.semio-tech.com/s/trinity/jack/artifact.json", taxonomy)).toBe("s.trinity.jack");
    expect(schemaScopeIdFromDocumentId("https://json.schemas.assets.semio-tech.com/hub/inference/contract.json", taxonomy)).toBe("hub.inference");
    expect(schemaScopeIdFromDocumentId("urn:semio:hub:inference", taxonomy)).toBe(null);
    expect(schemaScopeIdFromDocumentId("https://json.schemas.assets.semio-tech.com/contract.json", taxonomy)).toBe(null);
    expect(parseSchemaExportUri("schema://hub.inference/InferenceApproval", taxonomy)).toEqual({ scopeId: "hub.inference", exportId: "InferenceApproval" });
    expect(parseSchemaExportUri("schema://hub.inference/inferenceApproval", taxonomy)).toBe(null);
    expect(parseSchemaExportUri("local://hub.inference/InferenceApproval", taxonomy)).toBe(null);
    expect(schemaScopeOwnerLevel("✏️s/🔌️plugins/🔱️trinity", taxonomy)).toBe("plugin-root");
    expect(schemaScopeOwnerLevel("🌎️hub/📦️packages/🦀️rust/🧪️fixtures/👥️presence-lease-v1", taxonomy)).toBe(null);
    expect(resolveSchemaFacetKind("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema", taxonomy)).toBe("📜️interface");
    expect(resolveSchemaFacetKind("🌎️hub/💡️inference/🧬️schema", taxonomy)).toBe(taxonomy.schemaDefaultFacetKind);
  });

  test("cross-checks the Rust export registry dump against the catalog on the resolution key", async () => {
    const { schemaRustEntryDiagnostics } = await import("../../../🔍️discovery/🧾️schema-scope-catalog/🟦️.ts");
    const taxonomy = loadCatalogTaxonomy();
    const casesPath = join(contractRoot, "../🧫️fixtures/🧬️schema-rust-entries/🔣️.json");
    const cases = JSON.parse(readFileSync(casesPath, "utf8")) as { contract: string; cases: { id: string; scopes: Record<string, { path: string; formats: Record<string, string>; exports: Record<string, { file: string; facet: string }> }>; dump: { contractId: string; generator: string; entries: { scope: string; export: string; format: string }[] }; complete: boolean; expectedCodes: string[] }[] };
    const authority = JSON.parse(readFileSync(join(contractRoot, "../../🧬️schema/🧬️schema-rust-entries/🔣️.json"), "utf8"));
    
    for (const row of cases.cases) {
      const catalog = { contractId: taxonomy.schemaExportResolution.catalogContractId, catalogVersion: 1 as const, generator: taxonomy.schemaExportResolution.generator, taxonomySchemaVersion: taxonomy.schemaVersion, jsonSchemaDialect: taxonomy.schemaJsonDialect, scopes: Object.fromEntries(Object.entries(row.scopes).map(([id, scope]) => [id, { ...scope, level: "artifact-standard-subset", facetKind: taxonomy.schemaDefaultFacetKind, dependsOn: [], hashes: {} }])) };
      const findings = schemaRustEntryDiagnostics(catalog, row.dump, taxonomy, row.complete);
      expect({ case: row.id, codes: findings.map(({ code }) => code).sort() }).toEqual({ case: row.id, codes: [...row.expectedCodes].sort() });
      expect(findings.every(({ path, detail }) => path === taxonomy.schemaExportResolution.catalogPath && detail.length > 16)).toBe(true);
    }
  });

  test("declares one draft-07 dialect that an independent validator accepts", async () => {
    const library = await import("../../../🔍️discovery/🔐️taxonomy-authority/🟦️.ts");
    const taxonomy = library.loadCatalogTaxonomy();
    expect(taxonomy.schemaJsonDialect).toBe(taxonomy.mutationPayloadSchemaAuthority.jsonSchemaDialect);
    const document = { $schema: taxonomy.schemaJsonDialect, $id: "https://json.schemas.assets.semio-tech.com/repo/library/probe.json", title: "Probe", type: "object", additionalProperties: false, properties: { id: { type: "string" } }, required: ["id"] };
    const validate = new Ajv({ strict: true }).compile(document);
    expect(validate({ id: "a" })).toBe(true);
    expect(validate({ id: 1 })).toBe(false);
  });
});
