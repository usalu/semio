//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

//#endregion 🧲️Header

//#region 🔌️Adapters
import { describe, expect, test } from "bun:test";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import cases from "../../🧪️tests/🧬️schema-invariants/🔣️.json";
import protocolSchema from "../../🧬️schema/🔣️.json";
import {
  type SchemaBoundFixture,
  type SchemaDiagnostic,
  SCHEMA_FIXTURE_STAGES,
  TAXONOMY_REL_PATH,
  TEST_DOMAIN_REL_PATH,
  clearSchemaContractCache,
  discoverSchemaFixtures,
  fixtureUrisIn,
  isFixtureOwnedPath,
  isJsonSchemaDefinition,
  matchesTaxonomyPathPattern,
  parseFeature,
  parseSchemaUri,
  readSchemaCatalog,
  repoRootFromHere,
  resolveFixtures,
  resolveSchemaExport,
  schemaContractDiagnostics,
  schemaExportCompletenessDiagnostics,
  schemaFixtureIsolationDiagnostics,
  schemaOwnerEligibilityDiagnostics,
  schemaPlacementDiagnostics,
  schemaResolutionDiagnostics,
  schemaScopeEligibility,
  runSchemaFixture,
  validateAgainstJsonSchema,
} from "./🟦️.ts";
//#endregion 🔌️Adapters

//#region 🧭️Scaffold
const repoRoot = repoRootFromHere();
const CATALOG_REL = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️schema-catalog.json";
const WRITER_OWNER = "✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any";
const WRITER_ID = "https://semio.tech/schema/s/writer/writer/artifact.json";
const HUB_OWNER = "🌎️hub/💡️inference";
const HUB_ID = "https://semio.tech/schema/hub/inference.json";
const CASE_DIR = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧬️schema-invariants";
const FORMAT_FILENAMES: Readonly<Record<string, string>> = { "🔣️jsonschema": "🔣️.json", "🦀️rust": "🦀️.rs", "🟦️typescript": "🟦️.ts", "🔗️graphql": "🔗️.graphql", "🛰️protobuf": "🛰️.proto" };

type CatalogScopeRow = { path: string; formats: Record<string, string>; exports: Record<string, { file: string; facet: string }>; dependsOn: string[]; hashes: Record<string, string> };

/** 📚️ One export row of a module-root document: the normative file that carries it, and its facet. */
const rootExportRow = (...names: string[]): Record<string, { file: string; facet: string }> => Object.fromEntries(names.map((name) => [name, { file: "🔣️.json", facet: "schema" }]));

/** 📝️ Writes one file into a synthetic repository, creating its parents. */
function write(root: string, rel: string, body: string): void {
  mkdirSync(join(root, dirname(rel)), { recursive: true });
  writeFileSync(join(root, rel), body);
}

/** 🧪️ A synthetic repository carrying the real taxonomy, optionally declaring `schema://`. */
function scaffold(declareResolution = true): string {
  const root = mkdtempSync(join(tmpdir(), "schema-invariants-"));
  const taxonomy = JSON.parse(readFileSync(join(repoRoot, TAXONOMY_REL_PATH), "utf8")) as Record<string, unknown>;
  if (!declareResolution) delete taxonomy.schemaExportResolution;
  write(root, TAXONOMY_REL_PATH, JSON.stringify(taxonomy));
  clearSchemaContractCache();
  return root;
}

function discard(root: string): void {
  rmSync(root, { recursive: true, force: true });
  clearSchemaContractCache();
}

/** 🧬️ Writes one `🧬️schema/` module and returns the catalog row that names its files. */
function writeSchemaModule(root: string, ownerRel: string, id: string, defs: Record<string, unknown>, sources: Readonly<Record<string, string>>, rootExport = false): { path: string; formats: Record<string, string> } {
  const moduleRel = `${ownerRel}/🧬️schema`;
  // 🎯️A single-export module IS its export and says so with the taxonomy's root keyword; a multi-export
  // module lists them under `$defs`. Both forms are written here so both are actually exercised.
  const single = Object.entries(defs)[0]!;
  const document = rootExport ? { $schema: "http://json-schema.org/draft-07/schema#", $id: id, title: single[0], ...(single[1] as Record<string, unknown>) } : { $schema: "http://json-schema.org/draft-07/schema#", $id: id, $defs: defs };
  write(root, `${moduleRel}/🔣️.json`, `${JSON.stringify(document, null, 2)}\n`);
  // 📚️`path` is the MODULE directory and every `formats` value is relative to it — the shape the
  // generator emits, so this fixture repository and the real catalog are read by the same code.
  const formats: Record<string, string> = { "🔣️jsonschema": "🔣️.json" };
  for (const [format, source] of Object.entries(sources)) {
    write(root, `${moduleRel}/${FORMAT_FILENAMES[format]!}`, source);
    formats[format] = FORMAT_FILENAMES[format]!;
  }
  return { path: moduleRel, formats };
}

function writeCatalog(root: string, scopes: Record<string, CatalogScopeRow>, duplicateScopeId?: string): void {
  const text = JSON.stringify({ scopes }, null, 2);
  const body = duplicateScopeId === undefined ? text : text.replace('"scopes": {', `"scopes": {\n    ${JSON.stringify(duplicateScopeId)}: ${JSON.stringify(scopes[duplicateScopeId])},`);
  write(root, CATALOG_REL, `${body}\n`);
}

const ARTIFACT_DEF = { type: "object", additionalProperties: false, required: ["title"], properties: { title: { type: "string" } } } as const;

/** 🧪️ The repository every resolution case resolves against: one writer scope, four formats. */
function resolutionRepo(options: { declareResolution?: boolean; omitCatalog?: boolean; duplicateScopeId?: string; aliasScopeId?: string; fixtureOwnedScope?: boolean; dropRustExport?: boolean; rootExport?: boolean } = {}): string {
  const root = scaffold(options.declareResolution !== false);
  if (options.fixtureOwnedScope === true) {
    const scope = writeSchemaModule(root, "🌎️hub/🧪️fixtures/✅️inference-approval-v1", "https://semio.tech/schema/hub/inference-approval-fixture.json", { Approval: { type: "object" } }, {});
    writeCatalog(root, { "hub.inference": { ...scope, exports: rootExportRow("Approval"), dependsOn: [], hashes: {} } });
    return root;
  }
  const scope = writeSchemaModule(root, WRITER_OWNER, WRITER_ID, { Artifact: ARTIFACT_DEF }, {
    "🦀️rust": options.dropRustExport === true ? "pub struct WriterDocument {\n    pub title: String,\n}\n" : "pub struct Artifact {\n    pub title: String,\n}\n",
    "🟦️typescript": "export interface Artifact {\n  title: string;\n}\n",
    "🔗️graphql": "type Artifact {\n  title: String!\n}\n",
  }, options.rootExport === true);
  const row: CatalogScopeRow = { ...scope, exports: rootExportRow("Artifact"), dependsOn: [], hashes: {} };
  const scopes: Record<string, CatalogScopeRow> = { "s.writer.writer": row };
  if (options.aliasScopeId !== undefined) scopes[options.aliasScopeId] = { ...row };
  if (options.omitCatalog !== true) writeCatalog(root, scopes, options.duplicateScopeId);
  return root;
}

/** 🧪️ The repository every catalog case is judged against: two scopes, five formats each. */
function catalogRepo(options: { rootExport?: boolean; recursiveRef?: boolean; unresolvedLocalRef?: boolean; crossScopeRef?: boolean; declareDependency?: boolean; filePathRef?: boolean; dropRustExport?: boolean; dropTypescriptExport?: boolean; dropProtoExport?: boolean; dropGraphqlExport?: boolean } = {}): string {
  const root = scaffold(true);
  const properties: Record<string, unknown> = { title: { type: "string" } };
  if (options.recursiveRef === true) properties.children = { type: "array", items: { $ref: "#/$defs/Artifact" } };
  if (options.unresolvedLocalRef === true) properties.author = { $ref: "#/$defs/Author" };
  if (options.crossScopeRef === true) properties.approval = { $ref: `${HUB_ID}#/$defs/Approval` };
  if (options.filePathRef === true) properties.approval = { $ref: "../../🌎️hub/💡️inference/🧬️schema/🔣️.json#/$defs/Approval" };
  const writer = writeSchemaModule(root, WRITER_OWNER, WRITER_ID, { Artifact: { type: "object", additionalProperties: false, required: ["title"], properties } }, {
    "🦀️rust": options.dropRustExport === true ? "pub struct WriterDocument {\n    pub title: String,\n}\n" : "pub struct Artifact {\n    pub title: String,\n}\n",
    "🟦️typescript": options.dropTypescriptExport === true ? "export interface WriterDocument {\n  title: string;\n}\n" : "export interface Artifact {\n  title: string;\n}\n",
    "🔗️graphql": options.dropGraphqlExport === true ? "type WriterDocument {\n  title: String!\n}\n" : "type Artifact {\n  title: String!\n}\n",
    "🛰️protobuf": options.dropProtoExport === true ? "message WriterDocument {\n  string title = 1;\n}\n" : "message Artifact {\n  string title = 1;\n}\n",
  }, options.rootExport === true);
  const hub = writeSchemaModule(root, HUB_OWNER, HUB_ID, { Approval: { type: "object", additionalProperties: false, required: ["artifactId"], properties: { artifactId: { type: "string" } } } }, {
    "🦀️rust": "pub struct Approval {\n    pub artifact_id: String,\n}\n",
    "🟦️typescript": "export interface Approval {\n  artifactId: string;\n}\n",
    "🔗️graphql": "type Approval {\n  artifactId: String!\n}\n",
    "🛰️protobuf": "message Approval {\n  string artifact_id = 1;\n}\n",
  });
  writeCatalog(root, {
    "s.writer.writer": { ...writer, exports: rootExportRow("Artifact"), dependsOn: options.declareDependency === true ? ["hub.inference"] : [], hashes: {} },
    "hub.inference": { ...hub, exports: rootExportRow("Approval"), dependsOn: [], hashes: {} },
  });
  return root;
}

const codesOf = (found: readonly SchemaDiagnostic[]): string[] => found.map((entry) => entry.code).sort();
const locatedOf = (found: readonly SchemaDiagnostic[]): string[] => found.map((entry) => `${entry.code} ${entry.path ?? ""}`).sort();

/** 🔮️ ajv, the third-party draft-07 reference every verdict in this file is held against. */
async function ajv(): Promise<{ validate(schema: unknown, instance: unknown): boolean }> {
  const { default: Ajv } = await import("ajv");
  const instance = new Ajv({ strict: false, allErrors: true });
  return { validate: (schema, data) => instance.validate(schema as object, data) === true };
}
//#endregion 🧭️Scaffold

//#region 🧪️Tests
describe("🔣️ schema invariant cases are themselves contracted", () => {
  test("the case file validates against the test module's own descriptor schema", async () => {
    const { default: Ajv } = await import("ajv");
    const compiler = new Ajv({ strict: false, allErrors: true });
    compiler.addSchema(protocolSchema, "protocol");
    const validate = compiler.getSchema("protocol#/$defs/SchemaInvariantCases");
    expect(validate).toBeDefined();
    expect(validate!(cases)).toBe(true);
  });

  test("every case collection carries at least one case", () => {
    expect(cases.eligibilityCases.length).toBeGreaterThan(0);
    expect(cases.treeCases.length).toBeGreaterThan(0);
    expect(cases.resolutionCases.length).toBeGreaterThan(0);
    expect(cases.catalogCases.length).toBeGreaterThan(0);
    expect(cases.validationCases.length).toBeGreaterThan(0);
    expect(cases.pipelineCases.length).toBeGreaterThan(0);
  });
});

describe("🏛️ owner eligibility", () => {
  test("every declared level and exclusion is decided by position, not by filename", () => {
    for (const row of cases.eligibilityCases) {
      const verdict = schemaScopeEligibility(repoRoot, row.path);
      expect(`${row.id}:${verdict.eligible}:${verdict.level ?? ""}`).toBe(`${row.id}:${row.eligible}:${row.level ?? ""}`);
    }
  });

  test("a trailing slash never changes a verdict", () => {
    for (const row of cases.eligibilityCases) expect(schemaScopeEligibility(repoRoot, `${row.path}/`).eligible).toBe(row.eligible);
  });

  test("the glob vocabulary the taxonomy writes its levels in is honoured exactly", () => {
    expect(matchesTaxonomyPathPattern("✏️s/🔌️plugins/*", "✏️s/🔌️plugins/✒️writer")).toBe(true);
    expect(matchesTaxonomyPathPattern("✏️s/🔌️plugins/*", "✏️s/🔌️plugins/✒️writer/🗿️artifacts")).toBe(false);
    expect(matchesTaxonomyPathPattern("**/🧪️*", "🌎️hub/🧪️fixtures")).toBe(true);
    expect(matchesTaxonomyPathPattern("**/🧪️*", "🌎️hub/🧪️fixtures/x")).toBe(false);
    expect(matchesTaxonomyPathPattern("**/🧪️*/**", "🌎️hub/🧪️fixtures/x")).toBe(true);
    expect(isFixtureOwnedPath(repoRoot, "🌎️hub/🧪️fixtures/✅️approval/🧬️.schema.json")).toBe(true);
    expect(isFixtureOwnedPath(repoRoot, `${TEST_DOMAIN_REL_PATH}/🧬️schema/🔣️.json`)).toBe(false);
    expect(isFixtureOwnedPath(repoRoot, `${TEST_DOMAIN_REL_PATH}/🧪️tests/x/🔣️.json`)).toBe(true);
    expect(matchesTaxonomyPathPattern("🧰️framework/🛍️products/*/🔨️modules/**", "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine")).toBe(true);
  });
});

describe("📍️ placement, ownership and fixture isolation", () => {
  for (const row of cases.treeCases) {
    test(`${row.invariant} · ${row.id}`, () => {
      const root = scaffold();
      try {
        for (const file of row.files) write(root, file.path, `${JSON.stringify(file.json, null, 2)}\n`);
        const found = [...schemaPlacementDiagnostics(root), ...schemaOwnerEligibilityDiagnostics(root), ...schemaFixtureIsolationDiagnostics(root)];
        expect(locatedOf(found)).toEqual(row.expect.map((entry) => `${entry.code} ${entry.path}`).sort());
      } finally {
        discard(root);
      }
    });
  }

  test("a configuration instance naming its validating schema is never a definition", () => {
    expect(isJsonSchemaDefinition({ $schema: "../🧬️schema/🔣️.json", retainedCommandLimit: 64 })).toBe(false);
    expect(isJsonSchemaDefinition({ $schema: "http://json-schema.org/draft-07/schema#", type: "object" })).toBe(true);
    expect(isJsonSchemaDefinition({ $schema: "https://json-schema.org/draft/2020-12/schema" })).toBe(true);
    expect(isJsonSchemaDefinition({ type: "object" })).toBe(false);
  });
});

describe("🔗️ export resolution", () => {
  for (const row of cases.resolutionCases) {
    test(row.id, () => {
      const root = resolutionRepo(row as unknown as Parameters<typeof resolutionRepo>[0]);
      try {
        const { resolved, diagnostics } = resolveSchemaExport(root, row.uri, (row as { format?: string }).format);
        expect(resolved === null ? null : resolved.path).toEqual(row.expect.resolved ? (row.expect as { path?: string }).path ?? resolved!.path : null);
        expect(codesOf(diagnostics)).toEqual([...row.expect.codes].sort());
      } finally {
        discard(root);
      }
    });
  }

  test("a resolved export validates its example exactly as ajv does", async () => {
    const root = resolutionRepo();
    try {
      const { resolved } = resolveSchemaExport(root, "schema://s.writer.writer/Artifact");
      expect(resolved).not.toBeNull();
      const module = JSON.parse(readFileSync(join(root, resolved!.path), "utf8")) as { $defs: Record<string, unknown> };
      const oracle = await ajv();
      for (const [instance, valid] of [[{ title: "a" }, true], [{}, false], [{ title: "a", extra: 1 }, false], [{ title: 4 }, false]] as const) {
        expect(validateAgainstJsonSchema(module.$defs.Artifact, instance, module).length === 0).toBe(valid);
        expect(oracle.validate(module.$defs.Artifact, instance)).toBe(valid);
      }
    } finally {
      discard(root);
    }
  });

  test("a malformed uri is refused before the catalog is even read", () => {
    expect(parseSchemaUri("schema://s.writer.writer/Artifact")).toEqual({ scope: "s.writer.writer", export: "Artifact" });
    expect(parseSchemaUri("schema://s.writer.writer/artifact")).toBeNull();
    expect(parseSchemaUri("shared://s.writer.writer/Artifact")).toBeNull();
    expect(parseSchemaUri("schema://s.writer.writer")).toBeNull();
    expect(parseSchemaUri("schema://S.Writer/Artifact")).toBeNull();
  });
});

describe("📚️ catalog completeness and cross-scope dependencies", () => {
  for (const row of cases.catalogCases) {
    test(row.id, () => {
      const root = catalogRepo(row as unknown as Parameters<typeof catalogRepo>[0]);
      try {
        const found = schemaResolutionDiagnostics(root);
        expect(codesOf(found)).toEqual(row.expect.map((entry) => entry.code).sort());
        for (const entry of row.expect) {
          const format = (entry as { format?: string }).format;
          if (format !== undefined) expect(found.some((candidate) => candidate.code === entry.code && candidate.format === format)).toBe(true);
        }
      } finally {
        discard(root);
      }
    });
  }

  test("a scope that declares no json schema implementation has no normative definition", () => {
    const root = scaffold();
    try {
      expect(codesOf(schemaExportCompletenessDiagnostics(root, "s.empty", { path: "✏️s/🔌️plugins/✒️writer", formats: {}, exports: { Artifact: { file: "🔣️.json", facet: "schema" } }, dependsOn: [], hashes: {} }))).toEqual(["schema-format-unavailable"]);
    } finally {
      discard(root);
    }
  });
});

describe("🧬️ structural validation agrees with ajv", () => {
  test("every declared validation case gets the same verdict from both implementations", async () => {
    const oracle = await ajv();
    for (const row of cases.validationCases) {
      const ours = validateAgainstJsonSchema(row.schema, row.instance).length === 0;
      expect(`${row.id}:${ours}`).toBe(`${row.id}:${row.valid}`);
      expect(`${row.id}:${oracle.validate(row.schema, row.instance)}`).toBe(`${row.id}:${row.valid}`);
    }
  });

  test("a failure names the keyword and the place in the instance", () => {
    const errors = validateAgainstJsonSchema({ type: "object", required: ["title"], properties: { words: { type: "array", items: { type: "string" } } } }, { words: [1] });
    expect(errors.map((error) => `${error.instancePath} ${error.keyword}`).sort()).toEqual(["/words/0 type", " required"].sort());
  });
});

describe("🧫️ schema-bound fixtures run through named stages", () => {
  for (const row of cases.pipelineCases) {
    test(row.id, () => {
      const root = resolutionRepo({ dropRustExport: (row as { dropRustExport?: boolean }).dropRustExport === true, rootExport: (row as { rootExport?: boolean }).rootExport === true });
      try {
        const report = runSchemaFixture(root, row.fixture as unknown as SchemaBoundFixture, CASE_DIR);
        expect(report.stages.map((entry) => entry.stage)).toEqual([...SCHEMA_FIXTURE_STAGES]);
        const failed = report.stages.find((entry) => entry.result === "failed") ?? null;
        expect(`${row.id}:${failed?.stage ?? null}:${failed?.code ?? null}`).toBe(`${row.id}:${row.expect.failedStage}:${row.expect.code}`);
        expect(report.outcome).toBe(row.expect.outcome as "passed" | "failed");
      } finally {
        discard(root);
      }
    });
  }

  test("every stage is reported, and the ones after a failure are reported as skipped", () => {
    const root = resolutionRepo();
    try {
      const report = runSchemaFixture(root, { id: "x", uri: "schema://s.writer.absent/Artifact", target: { scope: "s.writer.absent", export: "Artifact", format: "🔣️jsonschema" }, inline: {}, expect: { stage: "contract-resolution", result: "failed", code: "schema-scope-unknown" } }, CASE_DIR);
      expect(report.stages.length).toBe(SCHEMA_FIXTURE_STAGES.length);
      expect(report.stages.map((entry) => entry.result)).toEqual(["passed", "failed", "skipped", "skipped", "skipped", "skipped"]);
      expect(report.outcome).toBe("passed");
    } finally {
      discard(root);
    }
  });
});

describe("🔗️ the fixture resolver speaks schema://", () => {
  test("a feature's schema:// reference is extracted beside the other three schemes", () => {
    const feature = parseFeature('@capability-x\nFeature: f\n  @id-s\n  @level-quick\n  @mode-conformance\n  Scenario: s\n    Given schema://s.writer.writer/Artifact and shared://v.bin and local://d.csv\n    Then y\n');
    expect(fixtureUrisIn(feature)).toEqual(["local://d.csv", "schema://s.writer.writer/Artifact", "shared://v.bin"]);
  });

  test("a schema:// fixture resolves to the catalog's file, pinned by digest", () => {
    const root = resolutionRepo();
    try {
      const discovered = { owner: WRITER_OWNER, ownerName: "✳️any", case: "c", caseDir: `${WRITER_OWNER}/🧪️tests/c`, featurePath: `${WRITER_OWNER}/🧪️tests/c/🥒️.feature`, adapters: {}, sharedFixtureDir: null, localFixtureDir: null, projectName: "p" };
      const { fixtures, missing } = resolveFixtures(root, discovered, ["schema://s.writer.writer/Artifact"]);
      expect(missing).toEqual([]);
      expect(fixtures[0]!.scope).toBe("schema");
      expect(fixtures[0]!.path).toBe(`${WRITER_OWNER}/🧬️schema/🔣️.json`);
      expect(fixtures[0]!.digest.length).toBe(32);
    } finally {
      discard(root);
    }
  });

  test("an unresolvable schema:// fixture is missing, never silently defaulted", () => {
    const root = resolutionRepo();
    try {
      const discovered = { owner: WRITER_OWNER, ownerName: "✳️any", case: "c", caseDir: `${WRITER_OWNER}/🧪️tests/c`, featurePath: `${WRITER_OWNER}/🧪️tests/c/🥒️.feature`, adapters: {}, sharedFixtureDir: null, localFixtureDir: null, projectName: "p" };
      const { fixtures, missing, diagnostics } = resolveFixtures(root, discovered, ["schema://s.writer.absent/Artifact"]);
      expect(fixtures).toEqual([]);
      expect(missing).toEqual(["schema://s.writer.absent/Artifact"]);
      expect(codesOf(diagnostics)).toEqual(["schema-scope-unknown"]);
    } finally {
      discard(root);
    }
  });
});

/**
 * 🤝️ The catalog generator (`📚️library`) and this harness are two independent implementations of one
 * invariant set, so they are held against ONE vector: the generator's own cases, run through these
 * checkers. Only the code-free halves are compared — which PATHS are misplaced, and which LEVEL each
 * eligible owner sits on — because the two implementations name their diagnostics differently and a
 * translation table between two vocabularies would be the adapter this ticket exists to remove.
 */
describe("🤝️ parity with the catalog generator's own vector", () => {
  const VECTOR = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️schema-scope-catalog/🧫️fixtures/🔣️.json";
  const PLACEMENT_CODES = new Set(["schema-placement-forbidden-filename", "schema-placement-outside-module", "schema-contracts-directory-forbidden", "schema-fixture-defines-schema"]);
  type GeneratorCase = { id: string; files: Record<string, unknown>; expected: { scopes: Record<string, { path: string; level: string }>; diagnosticCodes: string[]; placementPaths: string[] } };

  const vector = (): GeneratorCase[] => {
    expect(existsSync(join(repoRoot, VECTOR)), `the catalog generator's vector is expected at ${VECTOR}`).toBe(true);
    return (JSON.parse(readFileSync(join(repoRoot, VECTOR), "utf8")) as { cases: GeneratorCase[] }).cases;
  };

  test("every generator case places the same files and levels as this harness does", () => {
    for (const row of vector()) {
      const root = scaffold();
      try {
        for (const [rel, body] of Object.entries(row.files)) write(root, rel, typeof body === "string" ? body : `${JSON.stringify(body, null, 2)}\n`);
        const found = [...schemaPlacementDiagnostics(root), ...schemaFixtureIsolationDiagnostics(root)];
        const misplaced = [...new Set(found.filter((entry) => PLACEMENT_CODES.has(entry.code)).map((entry) => entry.path ?? ""))].sort();
        expect(`${row.id}:${misplaced.join(",")}`).toBe(`${row.id}:${[...row.expected.placementPaths].sort().join(",")}`);
        for (const [id, scope] of Object.entries(row.expected.scopes)) {
          const verdict = schemaScopeEligibility(root, scope.path.replace(/\/🧬️schema$/u, ""));
          expect(`${row.id}:${id}:${verdict.eligible}:${verdict.level ?? ""}`).toBe(`${row.id}:${id}:true:${scope.level}`);
        }
        if (row.expected.diagnosticCodes.includes("module-level-ineligible")) expect(schemaOwnerEligibilityDiagnostics(root).map((entry) => entry.code)).toEqual(["schema-owner-ineligible"]);
      } finally {
        discard(root);
      }
    }
  });
});

describe("🩺️ the committed tree is measured by the same checkers", () => {
  test("the repository declares where schema:// resolves, or says so", () => {
    const { catalog, diagnostics } = readSchemaCatalog(repoRoot);
    if (catalog === null) expect(codesOf(diagnostics).every((code) => code === "schema-export-resolution-undeclared" || code === "schema-catalog-missing")).toBe(true);
    else expect(Object.keys(catalog.scopes).length).toBeGreaterThan(0);
  });

  test("this module's own schema-bound fixtures are discoverable by declaration", () => {
    for (const entry of discoverSchemaFixtures(repoRoot, TEST_DOMAIN_REL_PATH)) expect(entry.fixtures.length).toBeGreaterThan(0);
  });

  // 🧭️Scoped to the module this worker owns. The rest of the tree is mid-migration under other
  // partitions, and asserting over it here would report their work as this module's breakage.
  test("the test platform's own subtree carries no schema-contract finding", () => {
    expect(schemaContractDiagnostics(repoRoot, TEST_DOMAIN_REL_PATH).map((entry) => `${entry.code} ${entry.path ?? ""}`)).toEqual([]);
  });
});
//#endregion 🧪️Tests
