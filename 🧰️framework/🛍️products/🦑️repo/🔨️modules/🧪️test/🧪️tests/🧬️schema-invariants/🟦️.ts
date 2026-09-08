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
  type SchemaDiagnosticCode,
  GRAPHQL_EXPORT_KEYWORDS,
  SCHEMA_DIAGNOSTIC_CODES,
  SCHEMA_DIAGNOSTIC_CODE_TABLE,
  SCHEMA_DIAGNOSTIC_EMITTERS,
  SCHEMA_FIXTURE_STAGES,
  TAXONOMY_REL_PATH,
  TEST_DOMAIN_REL_PATH,
  clearSchemaContractCache,
  declaresSchemaExport,
  declaresSchemaExportParser,
  discoverSchemaFixtures,
  fixtureUrisIn,
  isFixtureOwnedPath,
  isJsonSchemaDefinition,
  leafDescriptorCoverage,
  matchesTaxonomyPathPattern,
  mutationLeafDirectories,
  mutationLeafSchemaId,
  parseFeature,
  parseSchemaUri,
  readLeafDescriptors,
  readSchemaCatalog,
  repoRootFromHere,
  resolveFixtures,
  resolvePayloadSchemas,
  resolveSchemaExport,
  schemaContractDiagnostics,
  schemaDiagnosticCodesEmittedBy,
  schemaExportCompletenessDiagnostics,
  schemaFixtureIsolationDiagnostics,
  schemaMeasurementDisagreementDiagnostics,
  schemaOwnerEligibilityDiagnostics,
  schemaPlacementDiagnostics,
  schemaResolutionDiagnostics,
  schemaScopeEligibility,
  schemaTreeFiles,
  submodulePaths,
  runSchemaFixture,
  validateAgainstJsonSchema,
} from "../../📦️packages/🟦️typescript/🟦️.ts";
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
function writeSchemaModule(root: string, ownerRel: string, id: string, defs: Record<string, unknown>, sources: Readonly<Record<string, string>>, rootExport = false, extra: Record<string, unknown> = {}): { path: string; formats: Record<string, string> } {
  const moduleRel = `${ownerRel}/🧬️schema`;
  // 🎯️A single-export module IS its export and says so with the taxonomy's root keyword; a multi-export
  // module lists them under `$defs`. Both forms are written here so both are actually exercised.
  const single = Object.entries(defs)[0]!;
  const document = rootExport ? { $schema: "http://json-schema.org/draft-07/schema#", $id: id, title: single[0], ...(single[1] as Record<string, unknown>), ...extra } : { $schema: "http://json-schema.org/draft-07/schema#", $id: id, ...extra, $defs: defs };
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
    "🟦️typescript": typescriptModule("Artifact"),
    "🔗️graphql": "type Artifact {\n  title: String!\n}\n",
  }, options.rootExport === true);
  const row: CatalogScopeRow = { ...scope, exports: rootExportRow("Artifact"), dependsOn: [], hashes: {} };
  const scopes: Record<string, CatalogScopeRow> = { "s.writer.writer": row };
  if (options.aliasScopeId !== undefined) scopes[options.aliasScopeId] = { ...row };
  if (options.omitCatalog !== true) writeCatalog(root, scopes, options.duplicateScopeId);
  return root;
}

type CatalogCaseOptions = {
  rootExport?: boolean;
  recursiveRef?: boolean;
  unresolvedLocalRef?: boolean;
  crossScopeRef?: boolean;
  declareDependency?: boolean;
  filePathRef?: boolean;
  dropRustExport?: boolean;
  dropTypescriptExport?: boolean;
  dropProtoExport?: boolean;
  dropGraphqlExport?: boolean;
  internalDefinitionsRef?: boolean;
  unresolvedInternalRef?: boolean;
  crossScopeDefinitionsRef?: boolean;
  mutationAggregate?: boolean;
  restrictedFormats?: readonly string[];
  invalidRestrictedFormats?: unknown;
  implementRestrictedFormats?: readonly string[];
  dropTypescriptParser?: boolean;
  typescriptParserAsConst?: boolean;
  graphqlKeyword?: "type" | "input" | "enum" | "interface" | "union" | "scalar";
};

/**
 * 🚪️ One TypeScript module for an export: the type, and — unless the case removes it — the
 * `parse<Export>()` entry point the contract requires beside it.
 */
function typescriptModule(exported: string, options: { parser?: "function" | "const" | "none"; field?: string } = {}): string {
  const type = `export interface ${exported} {\n  ${options.field ?? "title"}: string;\n}\n`;
  if (options.parser === "none") return type;
  if (options.parser === "const") return `${type}export const parse${exported} = (value: unknown): ${exported} => value as ${exported};\n`;
  return `${type}export function parse${exported}(value: unknown): ${exported} {\n  return value as ${exported};\n}\n`;
}

/** 🔗️ One GraphQL declaration of an export, written with the keyword the case names. */
function graphqlModule(exported: string, keyword: CatalogCaseOptions["graphqlKeyword"] = "type"): string {
  switch (keyword) {
    case "enum":
      return `enum ${exported} {\n  TITLE\n}\n`;
    case "scalar":
      return `scalar ${exported}\n`;
    case "union":
      return `type WriterDocument {\n  title: String!\n}\n\nunion ${exported} = WriterDocument\n`;
    default:
      return `${keyword} ${exported} {\n  title: String!\n}\n`;
  }
}

/** 🧬️ The `$id` of the one mutation leaf the aggregate cases reference, derived by the rule under test. */
const LEAF_SEMANTIC_KIND = "create-artifact";
const LEAF_SCOPE_ID = "s.writer.writer.mutation.create-artifact";
const LEAF_OWNER = `${WRITER_OWNER}/🧬️schema/🧬️mutations/🌱️create`;
const AGGREGATE_ID = "https://semio.tech/schema/s/writer/writer/mutations.json";

/** 🧪️ The repository every catalog case is judged against: two scopes, five formats each. */
function catalogRepo(options: CatalogCaseOptions = {}): string {
  const root = scaffold(true);
  const properties: Record<string, unknown> = { title: { type: "string" } };
  const extra: Record<string, unknown> = {};
  if (options.recursiveRef === true) properties.children = { type: "array", items: { $ref: "#/$defs/Artifact" } };
  if (options.unresolvedLocalRef === true) properties.author = { $ref: "#/$defs/Author" };
  if (options.crossScopeRef === true) properties.approval = { $ref: `${HUB_ID}#/$defs/Approval` };
  if (options.crossScopeDefinitionsRef === true) properties.approval = { $ref: `${HUB_ID}#/definitions/approvalState` };
  if (options.filePathRef === true) properties.approval = { $ref: "../../🌎️hub/💡️inference/🧬️schema/🔣️.json#/$defs/Approval" };
  // 🔗️A helper lives in `definitions` and is addressed as `#/definitions/<helper>`; it is deliberately
  // NOT an export, so an internal reference to it can never be required to be export-addressed.
  if (options.internalDefinitionsRef === true) {
    extra.definitions = { author: { type: "object", additionalProperties: false, properties: { name: { type: "string" } } } };
    properties.author = { $ref: "#/definitions/author" };
  }
  if (options.unresolvedInternalRef === true) properties.author = { $ref: "#/definitions/author" };
  const restricted = options.restrictedFormats ?? (options.invalidRestrictedFormats === undefined ? null : options.invalidRestrictedFormats);
  const artifact: Record<string, unknown> = { type: "object", additionalProperties: false, required: ["title"], properties };
  if (restricted !== null) artifact["x-semio-formats"] = restricted;
  // 🏗️An export declaring restricted support is only proof of the rule if the formats it did not
  // declare actually stop implementing it; `implementRestrictedFormats` says which ones still do.
  const implemented = (format: string): boolean => (options.implementRestrictedFormats === undefined ? true : options.implementRestrictedFormats.includes(format));
  const writer = writeSchemaModule(root, WRITER_OWNER, WRITER_ID, { Artifact: artifact }, {
    "🦀️rust": options.dropRustExport === true || !implemented("🦀️rust") ? "pub struct WriterDocument {\n    pub title: String,\n}\n" : "pub struct Artifact {\n    pub title: String,\n}\n",
    "🟦️typescript": options.dropTypescriptExport === true || !implemented("🟦️typescript") ? typescriptModule("WriterDocument") : typescriptModule("Artifact", { parser: options.dropTypescriptParser === true ? "none" : options.typescriptParserAsConst === true ? "const" : "function" }),
    "🔗️graphql": options.dropGraphqlExport === true || !implemented("🔗️graphql") ? "type WriterDocument {\n  title: String!\n}\n" : graphqlModule("Artifact", options.graphqlKeyword),
    "🛰️protobuf": options.dropProtoExport === true || !implemented("🛰️protobuf") ? "message WriterDocument {\n  string title = 1;\n}\n" : "message Artifact {\n  string title = 1;\n}\n",
  }, options.rootExport === true, extra);
  const hub = writeSchemaModule(root, HUB_OWNER, HUB_ID, { Approval: { type: "object", additionalProperties: false, required: ["artifactId"], properties: { artifactId: { type: "string" } } } }, {
    "🦀️rust": "pub struct Approval {\n    pub artifact_id: String,\n}\n",
    "🟦️typescript": typescriptModule("Approval", { field: "artifactId" }),
    "🔗️graphql": "type Approval {\n  artifactId: String!\n}\n",
    "🛰️protobuf": "message Approval {\n  string artifact_id = 1;\n}\n",
  });
  const writerExports = rootExportRow("Artifact");
  const scopes: Record<string, CatalogScopeRow> = {
    "s.writer.writer": { ...writer, exports: writerExports, dependsOn: [], hashes: {} },
    "hub.inference": { ...hub, exports: rootExportRow("Approval"), dependsOn: [], hashes: {} },
  };
  if (options.mutationAggregate === true) {
    // 🧬️A mutation leaf is its own scope and its whole document IS the payload contract, so the
    // aggregate's branches address it by absolute `$id` with no fragment.
    const leafId = mutationLeafSchemaId(root, WRITER_ID, LEAF_SEMANTIC_KIND);
    const leaf = writeSchemaModule(root, LEAF_OWNER, leafId, { Payload: { type: "object", additionalProperties: false, required: ["title"], properties: { title: { type: "string" } } } }, {}, true);
    scopes[LEAF_SCOPE_ID] = { ...leaf, exports: rootExportRow("Payload"), dependsOn: [], hashes: {} };
    write(root, `${writer.path}/🧬️mutations/🔣️.json`, `${JSON.stringify({ $schema: "http://json-schema.org/draft-07/schema#", $id: AGGREGATE_ID, $defs: { ArtifactMutation: { oneOf: [{ type: "object", additionalProperties: false, required: ["mutation", "payload"], properties: { mutation: { const: "createArtifact" }, payload: { $ref: leafId } } }] } } }, null, 2)}\n`);
    write(root, `${writer.path}/🧬️mutations/🦀️.rs`, "pub enum ArtifactMutation {\n    CreateArtifact,\n}\n");
    write(root, `${writer.path}/🧬️mutations/🟦️.ts`, "export type ArtifactMutation = { mutation: \"createArtifact\" };\nexport function parseArtifactMutation(value: unknown): ArtifactMutation {\n  return value as ArtifactMutation;\n}\n");
    write(root, `${writer.path}/🧬️mutations/🔗️.graphql`, "union ArtifactMutation = CreateArtifact\n");
    write(root, `${writer.path}/🧬️mutations/🛰️.proto`, "message ArtifactMutation {\n  string mutation = 1;\n}\n");
    writerExports.ArtifactMutation = { file: "🧬️mutations/🔣️.json", facet: "mutations" };
  }
  if (options.declareDependency === true) scopes["s.writer.writer"] = { ...scopes["s.writer.writer"]!, dependsOn: options.mutationAggregate === true ? [LEAF_SCOPE_ID] : ["hub.inference"] };
  writeCatalog(root, scopes);
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
    expect(cases.exportPresenceCases.length).toBeGreaterThan(0);
    expect(cases.treeCases.length).toBeGreaterThan(0);
    expect(cases.resolutionCases.length).toBeGreaterThan(0);
    expect(cases.catalogCases.length).toBeGreaterThan(0);
    expect(cases.mutationLeafIdCases.length).toBeGreaterThan(0);
    expect(cases.validationCases.length).toBeGreaterThan(0);
    expect(cases.pipelineCases.length).toBeGreaterThan(0);
  });
});

/**
 * 🔠️ The diagnostic-code table is exported so the root `schema check` can adopt it instead of
 * keeping a second vocabulary (ledger row 97). That only holds if the table, the protocol enum the
 * findings are serialised against, and the codes actually emitted are one set.
 */
describe("🔠️ the diagnostic code table is the single vocabulary", () => {
  const protocolCodes = (protocolSchema as { $defs: { SchemaDiagnosticCode: { enum: string[] } } }).$defs.SchemaDiagnosticCode.enum;
  const protocolEmitters = (protocolSchema as { $defs: { SchemaDiagnosticEmitter: { enum: string[] } } }).$defs.SchemaDiagnosticEmitter.enum;
  const expectedCodes = (rows: readonly { code: string }[]): string[] => rows.map((row) => row.code);
  const everyExpectedCode = [...expectedCodes(cases.treeCases.flatMap((entry) => entry.expect)), ...expectedCodes(cases.catalogCases.flatMap((entry) => entry.expect)), ...cases.resolutionCases.flatMap((row) => row.expect.codes), ...cases.pipelineCases.flatMap((row) => (row.expect.code === null || row.expect.code === undefined ? [] : [row.expect.code]))];

  test("the exported table and the protocol enum declare exactly the same codes", () => {
    expect([...SCHEMA_DIAGNOSTIC_CODES].sort()).toEqual([...protocolCodes].sort());
  });

  test("every code carries a one-line description a reader can act on", () => {
    for (const code of SCHEMA_DIAGNOSTIC_CODES) {
      const { description } = SCHEMA_DIAGNOSTIC_CODE_TABLE[code];
      expect(`${code}:${description.length > 20}:${description.includes("\n")}`).toBe(`${code}:true:false`);
    }
  });

  test("every code names the emitters that raise it, out of the vocabulary the protocol declares", () => {
    expect([...SCHEMA_DIAGNOSTIC_EMITTERS].sort()).toEqual([...protocolEmitters].sort());
    for (const code of SCHEMA_DIAGNOSTIC_CODES) {
      const { emitters } = SCHEMA_DIAGNOSTIC_CODE_TABLE[code];
      const unknown = (emitters as readonly string[]).filter((emitter) => !(SCHEMA_DIAGNOSTIC_EMITTERS as readonly string[]).includes(emitter));
      expect(`${code}:${emitters.length > 0}:${unknown.join(",")}`).toBe(`${code}:true:`);
    }
    expect([...schemaDiagnosticCodesEmittedBy("harness"), ...schemaDiagnosticCodesEmittedBy("check")].length).toBeGreaterThan(SCHEMA_DIAGNOSTIC_CODES.length);
  });

  /**
   * 🏭️ The table holds codes only the root `schema check` raises, so the boundary has to be stated
   * somewhere a second implementation can read it: the vector names them, and this asserts the table
   * agrees and that no case in the vector ever expects one out of the harness.
   */
  test("the codes the vector reserves for schema check are exactly the ones the table withholds from the harness", () => {
    expect([...cases.checkOnlyDiagnosticCodes].sort()).toEqual(SCHEMA_DIAGNOSTIC_CODES.filter((code) => !(SCHEMA_DIAGNOSTIC_CODE_TABLE[code].emitters as readonly string[]).includes("harness")).sort());
    for (const code of cases.checkOnlyDiagnosticCodes) expect(`${code}:${(SCHEMA_DIAGNOSTIC_CODE_TABLE[code as SchemaDiagnosticCode].emitters as readonly string[]).includes("check")}`).toBe(`${code}:true`);
    const reserved = new Set<string>(cases.checkOnlyDiagnosticCodes);
    expect(everyExpectedCode.filter((code) => reserved.has(code))).toEqual([]);
  });

  test("every code a case expects is one the table declares, and one the harness emits", () => {
    const declared = new Set<string>(SCHEMA_DIAGNOSTIC_CODES);
    const harness = new Set<string>(schemaDiagnosticCodesEmittedBy("harness"));
    for (const code of everyExpectedCode) expect(`${code}:${declared.has(code)}:${harness.has(code)}`).toBe(`${code}:true:true`);
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

  /**
   * 🔨️ The one thing the collection test decides itself is the module-member carve-out, and its
   * directory name is vocabulary (`schemaScopeOwnerLevels.moduleMemberDirName`, ledger row 114). A
   * repository that renames the directory must keep the carve-out; renaming it here is the only way to
   * tell a taxonomy read apart from a literal that happens to agree with the taxonomy.
   */
  test("the module-member carve-out is the directory name the taxonomy declares, never a literal", () => {
    const root = scaffold();
    try {
      const taxonomy = JSON.parse(readFileSync(join(root, TAXONOMY_REL_PATH), "utf8")) as { schemaScopeOwnerLevels: Record<string, unknown> };
      taxonomy.schemaScopeOwnerLevels.moduleMemberDirName = "🧱️units";
      writeFileSync(join(root, TAXONOMY_REL_PATH), JSON.stringify(taxonomy));
      clearSchemaContractCache();
      expect(isFixtureOwnedPath(root, "🧰️framework/🧱️units/🧪️test/🧬️schema/🔣️.json")).toBe(false);
      expect(isFixtureOwnedPath(root, "🧰️framework/🔨️modules/🧪️test/🧬️schema/🔣️.json")).toBe(true);
      delete taxonomy.schemaScopeOwnerLevels.moduleMemberDirName;
      writeFileSync(join(root, TAXONOMY_REL_PATH), JSON.stringify(taxonomy));
      clearSchemaContractCache();
      expect(isFixtureOwnedPath(root, "🧰️framework/🧱️units/🧪️test/🧬️schema/🔣️.json")).toBe(true);
    } finally {
      discard(root);
    }
  });
});

/**
 * 🚶️ What the schema tree walk REACHES, and what it declines to. A walker that audits a directory
 * nobody in this repository can change reports findings nobody can act on, and one that audits build
 * output reports the same finding once per generated copy — both make a count that cannot be closed.
 */
/**
 * 🍃️ A mutation leaf sits at depth 1 or 2 under `🧬️mutations`. Reading one level reported the
 * grouping directories as undeclared leaves — they can never carry a descriptor — and never reached
 * the real leaves beneath them at all, so the coverage ratio was measured against the wrong set twice.
 */
describe("🍃️ mutation leaves at depth one and two", () => {
  const OWNER = `${WRITER_OWNER}`;
  const descriptor = (kind: string, variant: string) => ({ schemaVersion: 1, owner: `${OWNER}/🧬️schema/🧬️mutations`, semanticKind: kind, displayName: kind, emoji: "🌱️", aggregateVariant: variant, payloadSchema: "🧬️schema/🔣️.json", outcomeClasses: ["applied"] });
  const payload = { $schema: "http://json-schema.org/draft-07/schema#", $id: `https://semio.tech/schema/s/writer/writer/mutation/x/schema.json`, type: "object" };

  function repoWithLeaves(): string {
    const root = scaffold();
    const mutations = `${OWNER}/🧬️schema/🧬️mutations`;
    // 🍃️A one-segment leaf, beside a two-segment one: the grouping directory carries no descriptor,
    // its child carries one, and the verb directory's own name yields no kind at all.
    write(root, `${mutations}/🪄️change-title/🔣️.json`, `${JSON.stringify(descriptor("change-title", "ChangeTitle"), null, 2)}\n`);
    write(root, `${mutations}/🪄️change-title/🧬️schema/🔣️.json`, `${JSON.stringify(payload, null, 2)}\n`);
    write(root, `${mutations}/🔑️access-rule/🌱️create/🔣️.json`, `${JSON.stringify(descriptor("create-access-rule", "CreateAccessRule"), null, 2)}\n`);
    write(root, `${mutations}/🔑️access-rule/🌱️create/🧬️schema/🔣️.json`, `${JSON.stringify(payload, null, 2)}\n`);
    // 🧫️A collection under `🧬️mutations` holds example data; its case file is not a leaf descriptor.
    write(root, `${mutations}/🧪️tests/🧪️replays-a-change/🔣️.json`, `${JSON.stringify({ schemaVersion: 1 }, null, 2)}\n`);
    // 📝️A codec facet is a facet whatever it contains.
    write(root, `${mutations}/📝️text/🔣️.json`, `${JSON.stringify(payload, null, 2)}\n`);
    clearSchemaContractCache();
    return root;
  }

  test("a grouping directory is never a leaf, and every leaf beneath it is one", () => {
    const root = repoWithLeaves();
    try {
      expect(mutationLeafDirectories(root, OWNER)).toEqual(["🔑️access-rule/🌱️create", "🪄️change-title"]);
    } finally {
      discard(root);
    }
  });

  test("a two-segment leaf resolves its payload contract and is named by its descriptor", () => {
    const root = repoWithLeaves();
    try {
      const rows = resolvePayloadSchemas(root, OWNER);
      expect(rows.map((row) => `${row.kind}:${row.schema === null ? "refused" : "resolved"}`)).toEqual(["change-title:resolved", "create-access-rule:resolved"]);
      expect(rows.map((row) => row.leaf)).toContain(`${OWNER}/🧬️schema/🧬️mutations/🔑️access-rule/🌱️create`);
      expect(readLeafDescriptors(root, OWNER).size).toBe(2);
      expect(leafDescriptorCoverage(root, OWNER)).toEqual({ leaves: 2, described: 2, missing: [] });
    } finally {
      discard(root);
    }
  });

  test("a leaf the taxonomy's name pattern admits and nobody described is still reported", () => {
    const root = repoWithLeaves();
    try {
      write(root, `${OWNER}/🧬️schema/🧬️mutations/🪄️change-subtitle/🧬️schema/🔣️.json`, `${JSON.stringify(payload, null, 2)}\n`);
      clearSchemaContractCache();
      expect(mutationLeafDirectories(root, OWNER)).toContain("🪄️change-subtitle");
      expect(leafDescriptorCoverage(root, OWNER).missing).toEqual(["🪄️change-subtitle"]);
    } finally {
      discard(root);
    }
  });
});

/**
 * ⚖️ `--under` narrows BOTH measurements or the gate lies by omission, and the two measurements are
 * held against each other so a subtree can never read clean because one of them declined to look.
 */
describe("⚖️ the two measurements", () => {
  test("a subtree filter still measures export completeness, over that subtree's scopes only", () => {
    const root = catalogRepo({ dropRustExport: true });
    try {
      expect(codesOf(schemaResolutionDiagnostics(root, WRITER_OWNER))).toEqual(["schema-export-incomplete"]);
      expect(schemaResolutionDiagnostics(root, HUB_OWNER)).toEqual([]);
      expect(codesOf(schemaContractDiagnostics(root, WRITER_OWNER))).toEqual(["schema-export-incomplete"]);
    } finally {
      discard(root);
    }
  });

  test("a cross-scope reference out of the filtered subtree still resolves", () => {
    const root = catalogRepo({ crossScopeRef: true });
    try {
      // 🔗️`s.writer.writer` refs `hub.inference`, which the filter excludes from REPORTING but never
      // from the `$id` index: a narrowed index would report the legal reference as unresolved.
      expect(codesOf(schemaResolutionDiagnostics(root, WRITER_OWNER))).toEqual(["schema-cross-scope-dependency-forbidden"]);
    } finally {
      discard(root);
    }
  });

  test("a schema module the catalog does not name is a stale catalog, not a clean subtree", () => {
    const root = catalogRepo();
    try {
      expect(schemaMeasurementDisagreementDiagnostics(root)).toEqual([]);
      write(root, `${WRITER_OWNER}/✏️editor/🎚️config/🧬️schema/🔣️.json`, `${JSON.stringify({ $schema: "http://json-schema.org/draft-07/schema#", $id: "https://semio.tech/schema/s/writer/writer/config/schema.json", $defs: { Config: { type: "object" } } }, null, 2)}\n`);
      clearSchemaContractCache();
      const found = schemaMeasurementDisagreementDiagnostics(root);
      expect(locatedOf(found)).toEqual([`schema-catalog-stale ${WRITER_OWNER}/✏️editor/🎚️config/🧬️schema`]);
      expect(codesOf(schemaContractDiagnostics(root, `${WRITER_OWNER}/✏️editor`))).toEqual(["schema-catalog-stale"]);
    } finally {
      discard(root);
    }
  });

  test("a catalog row whose module the walk never reaches is the same disagreement, from the other side", () => {
    const root = catalogRepo();
    try {
      rmSync(join(root, HUB_OWNER, "🧬️schema", "🔣️.json"));
      clearSchemaContractCache();
      const found = schemaMeasurementDisagreementDiagnostics(root, undefined, HUB_OWNER);
      expect(found.map((entry) => `${entry.code} ${entry.scope}`)).toEqual(["schema-catalog-stale hub.inference"]);
    } finally {
      discard(root);
    }
  });
});

describe("🚶️ the tree walk's boundaries", () => {
  const definition = { $schema: "http://json-schema.org/draft-07/schema#", $id: "https://semio.tech/schema/foreign/schema.json", type: "object" };

  test("a git submodule declared in .gitmodules is a foreign repository and is never walked", () => {
    const root = scaffold();
    try {
      write(root, ".gitmodules", '[submodule "recherche"]\n\tpath = ♻️mit-bestand/🔎️recherche\n\turl = https://example.invalid/recherche.git\n');
      write(root, "♻️mit-bestand/🔎️recherche/contracts/lane_schema.json", `${JSON.stringify(definition)}\n`);
      write(root, "♻️mit-bestand/📋️bericht/contracts/lane_schema.json", `${JSON.stringify(definition)}\n`);
      clearSchemaContractCache();
      expect([...submodulePaths(root)]).toEqual(["♻️mit-bestand/🔎️recherche"]);
      const reached = schemaTreeFiles(root, "♻️mit-bestand");
      expect(reached).toEqual(["♻️mit-bestand/📋️bericht/contracts/lane_schema.json"]);
      // 📍️The sibling that is NOT a submodule still reports, so the skip is the declaration and not the prefix.
      expect(codesOf(schemaPlacementDiagnostics(root, reached))).toEqual(["schema-placement-outside-module"]);
    } finally {
      discard(root);
    }
  });

  test("a repository that declares no submodules carves nothing out", () => {
    const root = scaffold();
    try {
      write(root, "♻️mit-bestand/🔎️recherche/contracts/lane_schema.json", `${JSON.stringify(definition)}\n`);
      clearSchemaContractCache();
      expect([...submodulePaths(root)]).toEqual([]);
      expect(schemaTreeFiles(root, "♻️mit-bestand")).toEqual(["♻️mit-bestand/🔎️recherche/contracts/lane_schema.json"]);
    } finally {
      discard(root);
    }
  });

  test("build output under dist/ is generated, never an authority", () => {
    const root = scaffold();
    try {
      write(root, `${WRITER_OWNER}/📦️packages/🟦️typescript/dist/🧬️schema/📜️artifact-definition.json`, `${JSON.stringify(definition)}\n`);
      write(root, `${WRITER_OWNER}/📦️packages/🟦️typescript/🧬️schema/📜️artifact-definition.json`, `${JSON.stringify(definition)}\n`);
      clearSchemaContractCache();
      expect(schemaTreeFiles(root, WRITER_OWNER)).toEqual([`${WRITER_OWNER}/📦️packages/🟦️typescript/🧬️schema/📜️artifact-definition.json`]);
    } finally {
      discard(root);
    }
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

/**
 * 🔎️ What "the export exists in this format" MEANS, per format — the rule the completeness check is
 * built on, asserted directly so a change to it is caught here and not only through a case repository.
 */
describe("🔎️ per-format export presence", () => {
  for (const row of cases.exportPresenceCases) {
    test(`${row.format} · ${row.id}`, () => {
      expect(declaresSchemaExport(row.format, row.source, row.export)).toBe(row.present);
      const parser = (row as { parser?: boolean }).parser;
      if (parser !== undefined) expect(declaresSchemaExportParser(row.format, row.source, row.export)).toBe(parser);
    });
  }

  // 🦀️Row 148 in prose: a scope that OWNS its type elsewhere and republishes it by name has declared
  // the export, and one that hides it in a group or a glob has not. The vector above carries the same
  // twelve verdicts as data; these three restate the boundary where a regex is easiest to get wrong.
  test("a rust re-export declares an export only when the statement names it, alone", () => {
    expect(declaresSchemaExport("🦀️rust", "pub use inner::Artifact;\n", "Artifact")).toBe(true);
    expect(declaresSchemaExport("🦀️rust", "pub use inner::{Artifact};\n", "Artifact")).toBe(false);
    expect(declaresSchemaExport("🦀️rust", "pub use inner::*;\n", "Artifact")).toBe(false);
    expect(declaresSchemaExport("🦀️rust", "pub use inner::ArtifactDraft;\n", "Artifact")).toBe(false);
    expect(declaresSchemaExport("🦀️rust", "pub use inner::ArtifactV1 as Artifact;\n", "Artifact")).toBe(true);
    expect(declaresSchemaExport("🦀️rust", "pub use inner::Artifact as ArtifactV1;\n", "Artifact")).toBe(false);
    expect(declaresSchemaExport("🦀️rust", "pub(crate) use inner::Artifact;\n", "Artifact")).toBe(false);
    expect(declaresSchemaExport("🦀️rust", "use inner::Artifact;\n", "Artifact")).toBe(false);
  });

  test("graphql accepts every type-system keyword the contract lists, and nothing else", () => {
    for (const keyword of GRAPHQL_EXPORT_KEYWORDS) expect(`${keyword}:${declaresSchemaExport("🔗️graphql", `${keyword} Artifact {\n  title: String!\n}\n`, "Artifact")}`).toBe(`${keyword}:true`);
    expect(declaresSchemaExport("🔗️graphql", "scalar Artifact\n", "Artifact")).toBe(true);
    expect(declaresSchemaExport("🔗️graphql", "union Artifact = WriterDocument\n", "Artifact")).toBe(true);
    expect(declaresSchemaExport("🔗️graphql", "directive @Artifact on FIELD\n", "Artifact")).toBe(false);
    expect(declaresSchemaExport("🔗️graphql", "type ArtifactDraft {\n  title: String!\n}\n", "Artifact")).toBe(false);
  });

  test("a typescript export is the type AND its parse function, in either declaration form", () => {
    const type = "export interface Artifact {\n  title: string;\n}\n";
    expect(declaresSchemaExport("🟦️typescript", type, "Artifact")).toBe(true);
    expect(declaresSchemaExportParser("🟦️typescript", type, "Artifact")).toBe(false);
    expect(declaresSchemaExportParser("🟦️typescript", `${type}export function parseArtifact(value: unknown): Artifact {\n  return value as Artifact;\n}\n`, "Artifact")).toBe(true);
    expect(declaresSchemaExportParser("🟦️typescript", `${type}export async function parseArtifact(value: unknown): Promise<Artifact> {\n  return value as Artifact;\n}\n`, "Artifact")).toBe(true);
    expect(declaresSchemaExportParser("🟦️typescript", `${type}export const parseArtifact = (value: unknown): Artifact => value as Artifact;\n`, "Artifact")).toBe(true);
    expect(declaresSchemaExportParser("🟦️typescript", `${type}export let parseArtifact = (value: unknown): Artifact => value as Artifact;\n`, "Artifact")).toBe(true);
    expect(declaresSchemaExportParser("🟦️typescript", `${type}export declare function parseArtifact(value: unknown): Artifact;\n`, "Artifact")).toBe(true);
    // 🚪️A parser for a DIFFERENT export, and an unexported one, are both absent as far as a consumer is concerned.
    expect(declaresSchemaExportParser("🟦️typescript", `${type}export function parseArtifactDraft(value: unknown): Artifact {\n  return value as Artifact;\n}\n`, "Artifact")).toBe(false);
    expect(declaresSchemaExportParser("🟦️typescript", `${type}function parseArtifact(value: unknown): Artifact {\n  return value as Artifact;\n}\n`, "Artifact")).toBe(false);
  });

  test("no other format is asked for an entry point beside its entity", () => {
    for (const format of ["🦀️rust", "🛰️protobuf", "🔗️graphql", "🔣️jsonschema"]) expect(`${format}:${declaresSchemaExportParser(format, "", "Artifact")}`).toBe(`${format}:true`);
  });
});

describe("🧬️ a mutation leaf's own $id", () => {
  for (const row of cases.mutationLeafIdCases) {
    test(row.id, () => {
      const root = scaffold();
      try {
        expect(mutationLeafSchemaId(root, row.rootModuleId, row.semanticKind)).toBe(row.expected);
      } finally {
        discard(root);
      }
    });
  }

  test("the semantic kind keys the leaf, so a two-segment leaf directory still gets one id", () => {
    const root = scaffold();
    try {
      // 🧬️Deriving the id from the leaf DIRECTORY instead would give `🩻️node-skin/🔗️bind` and
      // `🎛️sampler/🔗️bind` the same last segment; the semantic kind is what keeps them apart.
      const ids = cases.mutationLeafIdCases.map((row) => mutationLeafSchemaId(root, row.rootModuleId, row.semanticKind));
      expect(new Set(ids).size).toBe(ids.length);
      for (const row of cases.mutationLeafIdCases) expect(mutationLeafSchemaId(root, row.rootModuleId, row.semanticKind).endsWith(`/${row.semanticKind}/schema.json`)).toBe(true);
    } finally {
      discard(root);
    }
  });

  test("a facet filename never deepens the scope a leaf hangs off", () => {
    const root = scaffold();
    try {
      expect(mutationLeafSchemaId(root, "https://semio.tech/schema/hub/inference/snapshot.json", "approve")).toBe(mutationLeafSchemaId(root, "https://semio.tech/schema/hub/inference/schema.json", "approve"));
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
