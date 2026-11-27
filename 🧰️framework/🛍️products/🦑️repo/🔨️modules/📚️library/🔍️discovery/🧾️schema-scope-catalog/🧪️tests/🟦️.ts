import { afterAll, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { minimatch } from "minimatch";
import { SCHEMA_EXPORT_COMPLETENESS_EXCLUDED_FORMATS, inventorySchemaScopes, schemaScopeOwnerLevel, walkSchemaTree } from "../🟦️.ts";
import { loadCatalogTaxonomy } from "../../🔐️taxonomy-authority/🟦️.ts";
import { TAXONOMY_REL_PATH } from "../../../../🧪️test/🗂️taxonomy/🟦️.ts";
import { repoRootFromHere } from "../../../../🧪️test/🧭️case-discovery/🟦️.ts";
import { clearSchemaContractCache, readSchemaCatalog } from "../../../../🧪️test/📐️schema-contracts/🟦️.ts";
import { schemaModuleDirectoriesOnDisk } from "../../../../🧪️test/📐️schema-contracts/🩺️placement-diagnostics/🟦️.ts";
import { schemaTreeFiles } from "../../../../🧪️test/📐️schema-contracts/🏗️scope-eligibility-and-tree/🟦️.ts";

const corpus = JSON.parse(readFileSync(new URL("./📥️.json", import.meta.url), "utf8")) as { cases: { id: string; path: string; level: string | null }[] };
const taxonomy = loadCatalogTaxonomy();
const owners = taxonomy.schemaScopeOwnerLevels;
for (const row of corpus.cases) test("schema state lane ownership: " + row.id, () => {
  const match = (pattern: string) => minimatch(row.path, pattern, { dot: true });
  const excluded = [...owners.excludedOwnerPathPatterns, ...owners.fixtureOwnerPathPatterns].some(match);
  const independent = excluded ? null : Object.entries(owners.levels).find(([, level]) => level.pathPatterns.some(match))?.[0] ?? null;
  expect(independent).toBe(row.level);
  expect(schemaScopeOwnerLevel(row.path, taxonomy)).toBe(row.level);
});

test("all declared state lane kinds own schema children", () => {
  for (const lane of ["🎚️config", "👥️presence", "🫧️transient"]) {
    expect(taxonomy.surfaceChildDirs).toContain(lane);
    expect(taxonomy.modeChildDirs).toContain(lane);
    expect(taxonomy.windowChildDirs).toContain(lane);
  }
  for (const key of ["configChildDirs", "presenceChildDirs", "transientChildDirs"] as const) expect(taxonomy[key]).toContain("🧬️schema");
});

const ID_BASE = loadCatalogTaxonomy().schemaExportResolution.idBase;
const MODULES = "🧰️framework/🛍️products/🦑️repo/🔨️modules";
const DIALECT = "http://json-schema.org/draft-07/schema#";
const scratchRoots: string[] = [];
afterAll(() => {
  for (const root of scratchRoots) rmSync(root, { recursive: true, force: true });
  clearSchemaContractCache();
});

function put(root: string, rel: string, body: unknown): void {
  mkdirSync(join(root, dirname(rel)), { recursive: true });
  writeFileSync(join(root, rel), typeof body === "string" ? body : `${JSON.stringify(body, null, 2)}\n`);
}

function definitionTree(): string {
  const root = mkdtempSync(join(tmpdir(), "schema-scope-catalog-law-"));
  scratchRoots.push(root);
  put(root, TAXONOMY_REL_PATH, readFileSync(join(repoRootFromHere(), TAXONOMY_REL_PATH), "utf8"));
  put(root, `${MODULES}/🧩️alpha/🧬️schema/🔣️.json`, { $schema: DIALECT, $id: `${ID_BASE}alpha/schema.json`, title: "Alpha", type: "object", properties: { name: { type: "string" } } });
  put(root, `${MODULES}/🧩️beta/🧬️schema/🔣️.json`, { $schema: DIALECT, title: "Beta", type: "object" });
  put(root, `${MODULES}/🧩️gamma/🧬️schema/🔣️.json`, { $schema: DIALECT, $id: "urn:semio:gamma:v1", type: "object" });
  put(root, `${MODULES}/🧩️delta/🧬️schema/🔣️gate/🔣️.json`, { $schema: DIALECT, $id: "semio.delta.gate/v1", type: "object" });
  put(root, `${MODULES}/🧩️epsilon/🧬️schema/🔣️.json`, { type: "object", properties: { name: { type: "string" } } });
  put(root, `${MODULES}/🧩️alpha/🧪️tests/🧫️fixtures/specimen/🧬️schema/🔣️.json`, { $schema: DIALECT, $id: `${ID_BASE}alpha/specimen.json`, type: "object" });
  put(root, `${MODULES}/🧩️alpha/node_modules/pkg/🧬️schema/🔣️.json`, { $schema: DIALECT, type: "object" });
  put(root, `${MODULES}/🧩️alpha/.state/🧬️schema/🔣️.json`, { $schema: DIALECT, type: "object" });
  clearSchemaContractCache();
  return root;
}

test("law: the generator and the schema-contract rule name one set of schema definition modules", () => {
  const root = definitionTree();
  const inventory = inventorySchemaScopes(root);
  const rule = [...schemaModuleDirectoriesOnDisk(root)].sort();
  const expected = ["alpha", "beta", "delta", "gamma"].map((name) => `${MODULES}/🧩️${name}/🧬️schema`);
  expect(rule).toEqual(expected);
  expect([...inventory.definitionModules]).toEqual(expected);
});

test("law: every definition module is a catalog row or a finding of the generator, and every row is a definition module", () => {
  const root = definitionTree();
  const inventory = inventorySchemaScopes(root);
  const rows = new Map(Object.entries(inventory.catalog.scopes).map(([id, scope]) => [scope.path, id]));
  expect([...rows.values()]).toEqual(["alpha"]);
  for (const path of rows.keys()) expect(inventory.definitionModules).toContain(path);
  for (const path of inventory.definitionModules.filter((candidate) => !rows.has(candidate))) expect(inventory.diagnostics.some((finding) => finding.code === "schema-module-id-missing" && (finding.path === path || finding.path.startsWith(`${path}/`)))).toBe(true);
});

test("law: the schema-contract rule's tree walk is the generator's tree walk", () => {
  const root = definitionTree();
  const walked = [...walkSchemaTree(root, loadCatalogTaxonomy()).files].sort();
  expect(schemaTreeFiles(root)).toEqual(walked);
  expect(walked.some((path) => path.includes("node_modules") || path.includes("/.state/"))).toBe(false);
  expect(schemaTreeFiles(root, `${MODULES}/🧩️alpha`)).toEqual(walked.filter((path) => path.startsWith(`${MODULES}/🧩️alpha/`)));
});

test("the repository test platform's four schema modules are catalogued, none is a stale gap", () => {
  const { catalog } = readSchemaCatalog(repoRootFromHere());
  const paths = new Set(Object.values(catalog!.scopes).map((scope) => scope.path));
  for (const module of ["🎯️acceptance", "🏭️inventory", "🏭️inventory/🔌️providers", ""]) expect(paths.has(`${MODULES}/🧪️test/${module ? `${module}/` : ""}🧬️schema`)).toBe(true);
});

function checkerTree(files: Record<string, unknown>): string {
  const root = mkdtempSync(join(tmpdir(), "schema-scope-catalog-checker-"));
  scratchRoots.push(root);
  put(root, TAXONOMY_REL_PATH, readFileSync(join(repoRootFromHere(), TAXONOMY_REL_PATH), "utf8"));
  for (const [rel, body] of Object.entries(files)) put(root, `${MODULES}/${rel}`, body);
  clearSchemaContractCache();
  return root;
}

const findingsOf = (root: string, code: string) => inventorySchemaScopes(root).diagnostics.filter((finding) => finding.code === code);

test("a cross-scope deep pointer resolves by its first $defs segment, a missing first segment is still reported", () => {
  const root = checkerTree({
    "🧩️alpha/🧬️schema/🔣️.json": { $schema: DIALECT, $id: `${ID_BASE}alpha/schema.json`, title: "Alpha", type: "object", $defs: { Thing: { type: "object", properties: { name: { type: "string" } } } } },
    "🧩️beta/🧬️schema/🔣️.json": { $schema: DIALECT, $id: `${ID_BASE}beta/schema.json`, title: "Beta", type: "object", properties: { good: { $ref: `${ID_BASE}alpha/schema.json#/$defs/Thing/properties/name` }, whole: { $ref: `${ID_BASE}alpha/schema.json#/$defs/Thing` }, bad: { $ref: `${ID_BASE}alpha/schema.json#/$defs/Missing/properties/name` } } },
  });
  const unresolved = findingsOf(root, "schema-ref-unresolved").map((finding) => finding.detail);
  expect(unresolved).toHaveLength(1);
  expect(unresolved[0]).toContain("Missing");
  expect(unresolved[0]).not.toContain("Thing");
});

test("a root document without $id is reported once, a facet document without $id once at its own path", () => {
  const root = checkerTree({
    "🧩️alpha/🧬️schema/🔣️.json": { $schema: DIALECT, title: "Alpha", type: "object" },
    "🧩️beta/🧬️schema/🔣️.json": { $schema: DIALECT, $id: `${ID_BASE}beta/schema.json`, title: "Beta", type: "object" },
    "🧩️beta/🧬️schema/🔺️diff/🔣️.json": { $schema: DIALECT, title: "BetaDiff", type: "object" },
  });
  const missing = findingsOf(root, "schema-module-id-missing").map((finding) => finding.path).sort();
  expect(missing).toEqual([`${MODULES}/🧩️alpha/🧬️schema`, `${MODULES}/🧩️beta/🧬️schema/🔺️diff/🔣️.json`].sort());
});

const deltaExports = ["EquationEdgesDelta", "EquationEdgeRemoval", "EquationEdgeInsertion", "EquationEdgeRelocation", "EquationEdgesModification", "Ghost"];
const deltaDefs = Object.fromEntries(deltaExports.map((name) => [name, { type: "object" }]));
const deltaRust = "use protocol::MutationDiff;\n\nprotocol::list_delta! {\n    /// 🧩 Positional row delta.\n    pub EquationEdgesDelta { removal: EquationEdgeRemoval, insertion: EquationEdgeInsertion, relocation: EquationEdgeRelocation, modification: EquationEdgesModification, row: EquationEdge, patch: EquationEdgePatch, key: id, values_only }\n}\n";

test("types a list_delta! macro declares count as declared in rust, an undeclared export is still incomplete", () => {
  const root = checkerTree({
    "🧩️alpha/🧬️schema/🔣️.json": { $schema: DIALECT, $id: `${ID_BASE}alpha/schema.json`, title: "Alpha", type: "object", $defs: deltaDefs },
    "🧩️alpha/🧬️schema/🦀️.rs": `${deltaRust}pub struct Alpha {}\n`,
  });
  const incomplete = findingsOf(root, "schema-export-incomplete").map((finding) => finding.detail);
  expect(incomplete).toHaveLength(1);
  expect(incomplete[0]).toContain("Ghost");
});

test("a plain_list_delta! and a row_patch! macro declare their types too; names in a comment or string do not", () => {
  const rust = "protocol::plain_list_delta! { pub PlainDelta { removal: PlainRemoval, insertion: PlainInsertion, relocation: PlainRelocation, row: String } }\nprotocol::row_patch! { pub RowPatchType of Row { set { a: u32 } } }\n// CommentOnly\nconst S: &str = \"StringOnly\";\npub struct Alpha {}\n";
  const defs = Object.fromEntries(["PlainDelta", "PlainRemoval", "PlainInsertion", "PlainRelocation", "RowPatchType", "CommentOnly", "StringOnly", "Row"].map((name) => [name, { type: "object" }]));
  const root = checkerTree({
    "🧩️alpha/🧬️schema/🔣️.json": { $schema: DIALECT, $id: `${ID_BASE}alpha/schema.json`, title: "Alpha", type: "object", $defs: defs },
    "🧩️alpha/🧬️schema/🦀️.rs": rust,
  });
  const incomplete = findingsOf(root, "schema-export-incomplete").map((finding) => /export (\w+)/u.exec(finding.detail)![1]).sort();
  expect(incomplete).toEqual(["CommentOnly", "Row", "StringOnly"]);
});

test("the wit format is excluded from export completeness, with the reason declared by the policy", () => {
  expect(Object.keys(SCHEMA_EXPORT_COMPLETENESS_EXCLUDED_FORMATS)).toEqual(["📜️wit"]);
  expect(SCHEMA_EXPORT_COMPLETENESS_EXCLUDED_FORMATS["📜️wit"]).toMatch(/\S{20}/u);
  const root = checkerTree({
    "🧩️alpha/🧬️schema/🔣️.json": { $schema: DIALECT, $id: `${ID_BASE}alpha/schema.json`, title: "Alpha", type: "object", $defs: { Thing: { type: "object" } } },
    "🧩️alpha/🧬️schema/📜️.wit": "package semio:alpha@1.0.0;\n\ninterface config {\n    record editor-config { x: f64 }\n}\n",
  });
  expect(findingsOf(root, "schema-export-incomplete")).toEqual([]);
});

test("a schema named like a corpus is a corpus by its role: expectation rows at the root or under a case list, not a nested runtime field", () => {
  const catalog = { type: "object", required: ["admission"], properties: { admission: { type: "object", properties: { expectedCounts: { type: "object", properties: { targets: { type: "integer" } } } } } } };
  const corpus = { type: "object", required: ["input", "expected"], properties: { input: { type: "string" }, expected: { type: "string" } } };
  const root = checkerTree({
    "🧩️alpha/🧬️schema/🔣️.json": { $schema: DIALECT, $id: `${ID_BASE}alpha/schema.json`, title: "Alpha", type: "object", $defs: { CatalogCases: catalog, ParserCases: corpus } },
  });
  const flagged = findingsOf(root, "schema-fixture-defines-schema").map((finding) => finding.detail);
  expect(flagged).toHaveLength(1);
  expect(flagged[0]).toContain("ParserCases");
});

test("a corpus-named schema whose case rows sit in a list, directly or by reference, is still a corpus", () => {
  const row = { type: "object", required: ["id", "expected"], properties: { id: { type: "string" }, expected: { type: "string" } } };
  const root = checkerTree({
    "🧩️alpha/🧬️schema/🔣️.json": { $schema: DIALECT, $id: `${ID_BASE}alpha/schema.json`, title: "Alpha", type: "object", $defs: { Row: row, Corpus: { type: "object", properties: { distributions: { type: "array", items: { $ref: "#/$defs/Row" } } } }, FontFixtures: { type: "array", items: { type: "object", properties: { text: { type: "string" }, expectedLines: { type: "array" } } } } } },
  });
  const flagged = findingsOf(root, "schema-fixture-defines-schema").map((finding) => finding.detail.split(" ")[0]).sort();
  expect(flagged).toEqual(["Corpus", "FontFixtures"]);
});
