/** ✅️ Schema validation and fixtures
 *
 * The JSON-schema subset validator and the schema fixture stages built on it.
 */

//#region 🔌️Adapters
import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { testFilenameForKind, testTaxonomy } from "../🗂️taxonomy/🟦️.ts";
import { moduleExportSchema, parseSchemaUri, readJson, readSchemaCatalog, resolveSchemaExport, schemaExportCompletenessDiagnostics, type SchemaHarnessDiagnosticCode, schemaTreeFiles } from "../📐️schema-contracts/🟦️.ts";
//#endregion 🔌️Adapters

//#region 🧬️SchemaValidation
/** ❌️ One structural validation failure, located in the instance. */
export type SchemaValidationError = Readonly<{ instancePath: string; keyword: string; detail: string }>;

/**
 * 🧬️ Validates an instance against a draft-07 schema.
 *
 * This is OUR implementation, on purpose: the harness may not take a runtime dependency on a
 * validator, and a validator nobody can read is a contract nobody can audit. Its verdicts are held
 * against ajv case by case in `🧪️tests/🧬️schema-invariants/🟦️.ts`, so "our own" never means "unverified".
 */
export function validateAgainstJsonSchema(schema: unknown, instance: unknown, root: unknown = schema): SchemaValidationError[] {
  return validateNode(schema, instance, "", root, 0);
}

function validateNode(schema: unknown, instance: unknown, path: string, root: unknown, depth: number): SchemaValidationError[] {
  if (schema === true || schema === undefined) return [];
  if (schema === false) return [{ instancePath: path, keyword: "false", detail: "the schema accepts nothing" }];
  if (schema === null || typeof schema !== "object" || Array.isArray(schema)) return [];
  if (depth > 64) return [{ instancePath: path, keyword: "$ref", detail: "reference chain exceeded 64 levels" }];
  const node = schema as Record<string, unknown>;
  if (typeof node.$ref === "string") {
    const target = resolveLocalPointer(root, node.$ref);
    if (target === undefined) return [{ instancePath: path, keyword: "$ref", detail: `${node.$ref} does not resolve inside this document` }];
    return validateNode(target, instance, path, root, depth + 1);
  }
  const errors: SchemaValidationError[] = [];
  const type = node.type;
  const types = typeof type === "string" ? [type] : Array.isArray(type) ? (type as string[]) : [];
  if (types.length > 0 && !types.some((candidate) => matchesJsonType(candidate, instance))) errors.push({ instancePath: path, keyword: "type", detail: `expected ${types.join(" or ")}` });
  if (Array.isArray(node.enum) && !node.enum.some((candidate) => deepEqualJson(candidate, instance))) errors.push({ instancePath: path, keyword: "enum", detail: "value is not one of the enumerated values" });
  if (Object.hasOwn(node, "const") && !deepEqualJson(node.const, instance)) errors.push({ instancePath: path, keyword: "const", detail: `expected ${JSON.stringify(node.const)}` });
  if (Array.isArray(node.allOf)) for (const branch of node.allOf) errors.push(...validateNode(branch, instance, path, root, depth + 1));
  if (Array.isArray(node.anyOf) && !node.anyOf.some((branch) => validateNode(branch, instance, path, root, depth + 1).length === 0)) errors.push({ instancePath: path, keyword: "anyOf", detail: "no branch accepts the value" });
  if (Array.isArray(node.oneOf)) {
    const accepted = node.oneOf.filter((branch) => validateNode(branch, instance, path, root, depth + 1).length === 0).length;
    if (accepted !== 1) errors.push({ instancePath: path, keyword: "oneOf", detail: `${accepted} branches accept the value, exactly one must` });
  }
  if (Object.hasOwn(node, "not") && validateNode(node.not, instance, path, root, depth + 1).length === 0) errors.push({ instancePath: path, keyword: "not", detail: "the value matches a forbidden schema" });
  if (typeof instance === "string") errors.push(...validateString(node, instance, path));
  if (typeof instance === "number") errors.push(...validateNumber(node, instance, path));
  if (Array.isArray(instance)) errors.push(...validateArray(node, instance, path, root, depth));
  else if (instance !== null && typeof instance === "object") errors.push(...validateObject(node, instance as Record<string, unknown>, path, root, depth));
  return errors;
}

function validateString(node: Record<string, unknown>, instance: string, path: string): SchemaValidationError[] {
  const errors: SchemaValidationError[] = [];
  const length = [...instance].length;
  if (typeof node.minLength === "number" && length < node.minLength) errors.push({ instancePath: path, keyword: "minLength", detail: `shorter than ${node.minLength}` });
  if (typeof node.maxLength === "number" && length > node.maxLength) errors.push({ instancePath: path, keyword: "maxLength", detail: `longer than ${node.maxLength}` });
  if (typeof node.pattern === "string" && !new RegExp(node.pattern, "u").test(instance)) errors.push({ instancePath: path, keyword: "pattern", detail: `does not match ${node.pattern}` });
  return errors;
}

function validateNumber(node: Record<string, unknown>, instance: number, path: string): SchemaValidationError[] {
  const errors: SchemaValidationError[] = [];
  if (typeof node.minimum === "number" && instance < node.minimum) errors.push({ instancePath: path, keyword: "minimum", detail: `below ${node.minimum}` });
  if (typeof node.maximum === "number" && instance > node.maximum) errors.push({ instancePath: path, keyword: "maximum", detail: `above ${node.maximum}` });
  if (typeof node.exclusiveMinimum === "number" && instance <= node.exclusiveMinimum) errors.push({ instancePath: path, keyword: "exclusiveMinimum", detail: `not above ${node.exclusiveMinimum}` });
  if (typeof node.exclusiveMaximum === "number" && instance >= node.exclusiveMaximum) errors.push({ instancePath: path, keyword: "exclusiveMaximum", detail: `not below ${node.exclusiveMaximum}` });
  if (typeof node.multipleOf === "number" && node.multipleOf > 0 && Math.abs(instance / node.multipleOf - Math.round(instance / node.multipleOf)) > 1e-9) errors.push({ instancePath: path, keyword: "multipleOf", detail: `not a multiple of ${node.multipleOf}` });
  return errors;
}

function validateArray(node: Record<string, unknown>, instance: readonly unknown[], path: string, root: unknown, depth: number): SchemaValidationError[] {
  const errors: SchemaValidationError[] = [];
  if (typeof node.minItems === "number" && instance.length < node.minItems) errors.push({ instancePath: path, keyword: "minItems", detail: `fewer than ${node.minItems} items` });
  if (typeof node.maxItems === "number" && instance.length > node.maxItems) errors.push({ instancePath: path, keyword: "maxItems", detail: `more than ${node.maxItems} items` });
  if (node.uniqueItems === true && instance.some((item, index) => instance.findIndex((candidate) => deepEqualJson(candidate, item)) !== index)) errors.push({ instancePath: path, keyword: "uniqueItems", detail: "items are not unique" });
  if (Array.isArray(node.items)) {
    for (const [index, item] of instance.entries()) {
      const positional = node.items[index];
      if (positional !== undefined) errors.push(...validateNode(positional, item, `${path}/${index}`, root, depth + 1));
      else if (node.additionalItems === false) errors.push({ instancePath: `${path}/${index}`, keyword: "additionalItems", detail: "the tuple declares no schema for this position" });
      else if (node.additionalItems !== undefined) errors.push(...validateNode(node.additionalItems, item, `${path}/${index}`, root, depth + 1));
    }
    return errors;
  }
  if (node.items !== undefined) for (const [index, item] of instance.entries()) errors.push(...validateNode(node.items, item, `${path}/${index}`, root, depth + 1));
  return errors;
}

function validateObject(node: Record<string, unknown>, instance: Record<string, unknown>, path: string, root: unknown, depth: number): SchemaValidationError[] {
  const errors: SchemaValidationError[] = [];
  const keys = Object.keys(instance);
  if (Array.isArray(node.required)) for (const key of node.required) if (typeof key === "string" && !Object.hasOwn(instance, key)) errors.push({ instancePath: path, keyword: "required", detail: `missing property ${JSON.stringify(key)}` });
  if (typeof node.minProperties === "number" && keys.length < node.minProperties) errors.push({ instancePath: path, keyword: "minProperties", detail: `fewer than ${node.minProperties} properties` });
  if (typeof node.maxProperties === "number" && keys.length > node.maxProperties) errors.push({ instancePath: path, keyword: "maxProperties", detail: `more than ${node.maxProperties} properties` });
  const properties = (node.properties ?? {}) as Record<string, unknown>;
  const patterns = Object.entries((node.patternProperties ?? {}) as Record<string, unknown>);
  for (const key of keys) {
    let matched = false;
    if (Object.hasOwn(properties, key)) {
      matched = true;
      errors.push(...validateNode(properties[key], instance[key], `${path}/${key}`, root, depth + 1));
    }
    for (const [pattern, sub] of patterns) {
      if (!new RegExp(pattern, "u").test(key)) continue;
      matched = true;
      errors.push(...validateNode(sub, instance[key], `${path}/${key}`, root, depth + 1));
    }
    if (matched) continue;
    if (node.additionalProperties === false) errors.push({ instancePath: `${path}/${key}`, keyword: "additionalProperties", detail: `property ${JSON.stringify(key)} is not declared` });
    else if (node.additionalProperties !== undefined) errors.push(...validateNode(node.additionalProperties, instance[key], `${path}/${key}`, root, depth + 1));
  }
  if (node.propertyNames !== undefined) for (const key of keys) errors.push(...validateNode(node.propertyNames, key, `${path}/${key}`, root, depth + 1));
  return errors;
}

function matchesJsonType(type: string, instance: unknown): boolean {
  switch (type) {
    case "object":
      return instance !== null && typeof instance === "object" && !Array.isArray(instance);
    case "array":
      return Array.isArray(instance);
    case "string":
      return typeof instance === "string";
    case "number":
      return typeof instance === "number";
    case "integer":
      return typeof instance === "number" && Number.isInteger(instance);
    case "boolean":
      return typeof instance === "boolean";
    case "null":
      return instance === null;
    default:
      return true;
  }
}

function deepEqualJson(a: unknown, b: unknown): boolean {
  if (a === b) return true;
  if (Array.isArray(a) && Array.isArray(b)) return a.length === b.length && a.every((item, index) => deepEqualJson(item, b[index]));
  if (a === null || b === null || typeof a !== "object" || typeof b !== "object") return false;
  const left = a as Record<string, unknown>;
  const right = b as Record<string, unknown>;
  const keys = Object.keys(left);
  return keys.length === Object.keys(right).length && keys.every((key) => Object.hasOwn(right, key) && deepEqualJson(left[key], right[key]));
}

function resolveLocalPointer(root: unknown, ref: string): unknown {
  if (!ref.startsWith("#")) return undefined;
  const pointer = ref.slice(1);
  if (pointer.length === 0) return root;
  let node: unknown = root;
  for (const raw of pointer.split("/").slice(1)) {
    const key = decodeURIComponent(raw).replace(/~1/gu, "/").replace(/~0/gu, "~");
    if (node === null || typeof node !== "object") return undefined;
    node = (node as Record<string, unknown>)[key];
    if (node === undefined) return undefined;
  }
  return node;
}
//#endregion 🧬️SchemaValidation

//#region 🧫️SchemaFixtures
/**
 * 🪜️ The stages every schema-bound fixture runs through, in order.
 *
 * They are NAMED and REPORTED because "the fixture failed" answers nothing: a fixture that fails at
 * `contract-resolution` says the catalog is wrong, one that fails at `structural-validation` says the
 * example is wrong, and one that fails at `domain` says the implementation is. A negative fixture
 * declares the stage it is supposed to fail at, so a case that starts failing EARLIER — for an
 * entirely different reason — stops counting as the proof it was written to be.
 */
export const SCHEMA_FIXTURE_STAGES = ["metadata", "contract-resolution", "parse", "structural-validation", "domain", "assertions"] as const;
export type SchemaFixtureStage = (typeof SCHEMA_FIXTURE_STAGES)[number];

/** 🎯️ What a fixture binds to: a named export of a scope, in one format. Never a file path. */
export type SchemaFixtureTarget = Readonly<{ scope: string; export: string; format: string }>;

/** 🎯️ The stage, verdict and code a negative fixture asserts. */
export type SchemaStageExpectation = Readonly<{ stage: SchemaFixtureStage; result: "passed" | "failed"; code: string | null }>;

/** 🧫️ One schema-bound fixture: an example that BINDS to an export and never defines one. */
export type SchemaTestExample = Readonly<{
  id: string;
  uri: string;
  target: SchemaFixtureTarget;
  /** 📄️ Case-relative data file. Mutually exclusive with `inline`. */
  data?: string;
  /** 📄️ Inline example, for a case whose subject is small enough to read beside its expectation. */
  inline?: unknown;
  expect: SchemaStageExpectation;
}>;

/** 🪜️ What one stage did. `skipped` means an earlier stage already decided the outcome. */
export type SchemaStageOutcome = Readonly<{ stage: SchemaFixtureStage; result: "passed" | "failed" | "skipped"; code: SchemaHarnessDiagnosticCode | null; detail: string }>;

/** 📤️ One fixture's staged report — the shape `test schema` prints and a gate reads. */
export type SchemaFixtureReport = Readonly<{ fixture: string; uri: string; target: SchemaFixtureTarget; stages: readonly SchemaStageOutcome[]; outcome: "passed" | "failed"; expected: SchemaStageExpectation; detail: string }>;

const SCHEMA_FIXTURE_COLLECTION_KEY = "schemaFixtures";

function stage(name: SchemaFixtureStage, result: "passed" | "failed" | "skipped", code: SchemaHarnessDiagnosticCode | null, detail: string): SchemaStageOutcome {
  return { stage: name, result, code, detail };
}

/**
 * 🪜️ Runs one schema-bound fixture through the six stages and reports every one of them.
 *
 * Execution STOPS at the first failing stage and the rest are reported as `skipped` rather than
 * silently omitted: a report that lists four stages when six exist reads as four passes.
 */
export function runSchemaFixture(repoRoot: string, fixture: SchemaTestExample, caseDirRel: string): SchemaFixtureReport {
  const stages: SchemaStageOutcome[] = [];
  const expected: SchemaStageExpectation = fixture.expect;
  const finish = (): SchemaFixtureReport => {
    for (const name of SCHEMA_FIXTURE_STAGES) if (!stages.some((entry) => entry.stage === name)) stages.push(stage(name, "skipped", null, "an earlier stage decided the outcome"));
    const ordered = SCHEMA_FIXTURE_STAGES.map((name) => stages.find((entry) => entry.stage === name)!);
    const failed = ordered.find((entry) => entry.result === "failed") ?? null;
    const matches = failed === null ? expected.result === "passed" : expected.result === "failed" && expected.stage === failed.stage && (expected.code === null || expected.code === failed.code);
    return {
      fixture: `${caseDirRel}::${fixture.id}`,
      uri: fixture.uri,
      target: fixture.target,
      stages: ordered,
      outcome: matches ? "passed" : "failed",
      expected,
      detail: matches ? (failed === null ? "every stage passed, as declared" : `failed at ${failed.stage} with ${failed.code}, as declared`) : failed === null ? `declared a ${expected.result} at ${expected.stage}, and every stage passed` : `declared ${expected.result} at ${expected.stage}${expected.code === null ? "" : ` with ${expected.code}`}, and it failed at ${failed.stage} with ${failed.code}`,
    };
  };

  const metadata = schemaFixtureMetadataProblems(fixture);
  if (metadata !== null) {
    stages.push(stage("metadata", "failed", "schema-fixture-metadata-invalid", metadata));
    return finish();
  }
  stages.push(stage("metadata", "passed", null, "the descriptor names a scope, an export, a format and one subject"));

  const { resolved, diagnostics } = resolveSchemaExport(repoRoot, fixture.uri, fixture.target.format);
  if (resolved === null) {
    const first = diagnostics[0];
    stages.push(stage("contract-resolution", "failed", first?.code ?? "schema-scope-unknown", first?.detail ?? `${fixture.uri} does not resolve`));
    return finish();
  }
  stages.push(stage("contract-resolution", "passed", null, `${fixture.uri} → ${resolved.path} (${resolved.digest})`));

  let subject: unknown;
  if (fixture.data === undefined) subject = fixture.inline;
  else {
    const abs = join(repoRoot, caseDirRel, fixture.data);
    if (!existsSync(abs)) {
      stages.push(stage("parse", "failed", "schema-fixture-parse-failed", `${caseDirRel}/${fixture.data} does not exist`));
      return finish();
    }
    try {
      subject = JSON.parse(readFileSync(abs, "utf8"));
    } catch (error) {
      stages.push(stage("parse", "failed", "schema-fixture-parse-failed", error instanceof Error ? error.message : String(error)));
      return finish();
    }
  }
  stages.push(stage("parse", "passed", null, "the subject decoded"));

  const module = readJson(join(repoRoot, resolved.path));
  const exportSchema = moduleExportSchema(repoRoot, module, resolved.export);
  if (exportSchema === undefined) {
    stages.push(stage("structural-validation", "failed", "schema-export-unknown", `${resolved.path} declares ${resolved.export} neither in $defs nor as its root export`));
    return finish();
  }
  const errors = validateAgainstJsonSchema(exportSchema, subject, module);
  if (errors.length > 0) {
    stages.push(stage("structural-validation", "failed", "schema-instance-invalid", errors.map((error) => `${error.instancePath || "/"} ${error.keyword}: ${error.detail}`).join("; ")));
    return finish();
  }
  stages.push(stage("structural-validation", "passed", null, `the subject satisfies $defs.${resolved.export}`));

  const { catalog } = readSchemaCatalog(repoRoot);
  const domain = catalog === null ? [] : schemaExportCompletenessDiagnostics(repoRoot, resolved.scope, catalog.scopes[resolved.scope]!).filter((entry) => entry.export === null || entry.export === resolved.export);
  if (domain.length > 0) {
    stages.push(stage("domain", "failed", domain[0]!.code, domain[0]!.detail));
    return finish();
  }
  stages.push(stage("domain", "passed", null, `every declared format of ${resolved.scope} implements ${resolved.export}`));
  stages.push(stage("assertions", "passed", null, "the outcome is compared against the declared expectation"));
  return finish();
}

function schemaFixtureMetadataProblems(fixture: SchemaTestExample): string | null {
  if (typeof fixture.id !== "string" || fixture.id.length === 0) return "the fixture declares no id";
  const parsed = parseSchemaUri(fixture.uri ?? "");
  if (parsed === null) return `${JSON.stringify(fixture.uri)} is not a schema://<scope id>/<ExportId> reference`;
  const target = fixture.target;
  if (target === undefined || target === null || typeof target !== "object") return "the fixture declares no target";
  if (target.scope !== parsed.scope || target.export !== parsed.export) return `target ${target.scope}/${target.export} disagrees with the uri ${fixture.uri}`;
  if (typeof target.format !== "string" || target.format.length === 0) return "the target names no format";
  if ((fixture.data === undefined) === (fixture.inline === undefined)) return "a fixture declares exactly one of `data` and `inline`";
  const expectation = fixture.expect;
  if (expectation === undefined || !(SCHEMA_FIXTURE_STAGES as readonly string[]).includes(expectation.stage) || (expectation.result !== "passed" && expectation.result !== "failed")) return "the fixture declares no {stage, result} expectation";
  if (expectation.result === "passed" && expectation.code !== null) return "a passing expectation carries no code";
  return null;
}

/** 🧫️ Every schema-bound fixture in the tree, found by the DECLARATION key, never by shape. */
export function discoverSchemaFixtures(repoRoot: string, under = ""): { caseDir: string; fixtures: readonly SchemaTestExample[] }[] {
  const taxonomy = testTaxonomy(repoRoot);
  const caseFile = testFilenameForKind(taxonomy, taxonomy.testContributionFileKindId);
  const found: { caseDir: string; fixtures: readonly SchemaTestExample[] }[] = [];
  for (const rel of schemaTreeFiles(repoRoot, under)) {
    if (!rel.endsWith(`/${caseFile}`)) continue;
    const document = readJson(join(repoRoot, rel));
    const declared = (document as Record<string, unknown> | undefined)?.[SCHEMA_FIXTURE_COLLECTION_KEY];
    if (!Array.isArray(declared) || declared.length === 0) continue;
    found.push({ caseDir: rel.slice(0, rel.lastIndexOf("/")), fixtures: declared as SchemaTestExample[] });
  }
  return found;
}
//#endregion 🧫️SchemaFixtures
