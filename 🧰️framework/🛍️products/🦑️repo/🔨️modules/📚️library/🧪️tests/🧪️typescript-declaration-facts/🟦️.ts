import { expect, test } from "bun:test";
import Ajv from "ajv/dist/2020";
import { closeSync, constants, fstatSync, lstatSync, openSync, readFileSync } from "node:fs";
import { join, parse, resolve, sep } from "node:path";
import * as discovery from "../../🔍️discovery/🟦️.ts";
import { compilerFacts, compilerParseDiagnostics, strictSourceDiagnostics, type Facts, type Vector } from "./🧪️oracle/🟦️.ts";

//#region 🔒️Inputs
/** 🔒️ Reads one exact regular test asset after checking nonsymlink ancestors. */
function asset(relativePath: string): string {
  const path = resolve(import.meta.dir, relativePath), root = parse(path).root;
  const segments = path.slice(root.length).split(sep);
  if (segments.some((segment) => !segment || segment === "." || segment === ".." || segment.toLocaleLowerCase("en-US") === "compose")) throw new Error("Invalid declaration fixture path");
  let ancestor = root;
  for (const segment of segments.slice(0, -1)) {
    ancestor = join(ancestor, segment);
    const state = lstatSync(ancestor);
    if (!state.isDirectory() || state.isSymbolicLink()) throw new Error("Unsafe declaration fixture ancestor");
  }
  const named = lstatSync(path, { bigint: true });
  if (!named.isFile() || named.isSymbolicLink()) throw new Error("Unsafe declaration fixture leaf");
  const fd = openSync(path, constants.O_RDONLY | (constants.O_NOFOLLOW ?? 0));
  try {
    const before = fstatSync(fd, { bigint: true });
    if (before.dev !== named.dev || before.ino !== named.ino || before.size !== named.size || before.mtimeNs !== named.mtimeNs || before.ctimeNs !== named.ctimeNs) throw new Error("Declaration fixture changed before read");
    const bytes = readFileSync(fd), after = fstatSync(fd, { bigint: true }), endpoint = lstatSync(path, { bigint: true });
    if (before.dev !== after.dev || before.ino !== after.ino || before.size !== after.size || before.mtimeNs !== after.mtimeNs || before.ctimeNs !== after.ctimeNs || endpoint.isSymbolicLink() || endpoint.dev !== before.dev || endpoint.ino !== before.ino) throw new Error("Declaration fixture changed during read");
    return new TextDecoder("utf-8", { fatal: true, ignoreBOM: true }).decode(bytes);
  } finally { closeSync(fd); }
}

const schema = JSON.parse(asset("🧬️schema/🔣️.json"));
const vectors = JSON.parse(asset("🔣️.json")) as { readonly schemaVersion: 1; readonly cases: readonly Vector[] };
const ajv = new Ajv({ strict: true, allErrors: true });
const validateVectors = ajv.compile(schema);
const validateFacts = ajv.compile({ $defs: schema.$defs, $ref: "#/$defs/expected" });

/** 🧭️ Obtains only the public owned source inspector, with no compiler fallback. */
function inspector(): (source: string, language: Vector["language"]) => Facts {
  const value = Reflect.get(discovery, "inspectTypeScriptDeclarationFacts");
  expect(typeof value).toBe("function");
  return value as (source: string, language: Vector["language"]) => Facts;
}

/** 📏️ Validates all returned coordinates against the exact unchanged source string. */
function coordinateBounds(source: string, facts: Facts): void {
  for (const row of [...facts.declarations, ...facts.aliases, ...facts.diagnostics]) {
    expect(Number.isInteger(row.span.start)).toBe(true);
    expect(Number.isInteger(row.span.end)).toBe(true);
    expect(row.span.start).toBeGreaterThanOrEqual(0);
    expect(row.span.end).toBeGreaterThanOrEqual(row.span.start);
    expect(row.span.end).toBeLessThanOrEqual(source.length);
  }
}
//#endregion 🔒️Inputs

//#region 🧪️Declarations
test("TypeScript declaration facts use the closed neutral schema", () => {
  expect(validateVectors(vectors), JSON.stringify(validateVectors.errors)).toBe(true);
  expect(new Set(vectors.cases.map((row) => row.id)).size).toBe(vectors.cases.length);
  expect(validateFacts({ declarations: [], aliases: [], diagnostics: [] })).toBe(false);
  expect(validateFacts({ completeness: "complete", declarations: [], aliases: [], diagnostics: [], foreign: true })).toBe(false);
});

for (const row of vectors.cases) {
  test("TypeScript declaration reference: " + row.id, () => {
    const source = row.sourceLines.join("\n"), actual = compilerFacts(source, row.language);
    expect(validateFacts(actual), JSON.stringify(validateFacts.errors)).toBe(true);
    coordinateBounds(source, actual);
    expect(actual).toEqual(row.expected);
  });
  test("TypeScript declaration subject: " + row.id, () => {
    const source = row.sourceLines.join("\n"), actual = inspector()(source, row.language);
    expect(validateFacts(actual), JSON.stringify(validateFacts.errors)).toBe(true);
    coordinateBounds(source, actual);
    expect(actual).toEqual(row.expected);
  });
}

test("TypeScript declaration inspector rejects an unspecified or unsupported language", () => {
  const inspect = inspector() as (source: string, language: unknown) => Facts;
  for (const language of [undefined, null, "", "js", "jsx", "TS", 0, {}]) expect(() => inspect("", language)).toThrow(TypeError);
});

test("TypeScript declaration grammar has strict standalone source types", () => {
  const source = asset("../../🔍️discovery/🟦️.ts"), begin = "//#region 🟦️TypeScriptDeclarationFacts", finish = "//#endregion 🟦️TypeScriptDeclarationFacts";
  const start = source.indexOf(begin), end = source.indexOf(finish);
  expect(start).toBeGreaterThanOrEqual(0);
  expect(end).toBeGreaterThan(start);
  expect(source.indexOf(begin, start + begin.length)).toBe(-1);
  expect(source.indexOf(finish, end + finish.length)).toBe(-1);
  expect(strictSourceDiagnostics(source.slice(start, end + finish.length), resolve(import.meta.dir, "🟦️virtual-grammar.ts"))).toEqual([]);
});

test("TypeScript declaration compiler oracle has strict source types", () => {
  expect(strictSourceDiagnostics(asset("🧪️oracle/🟦️.ts"), resolve(import.meta.dir, "🧪️oracle/🟦️.ts"))).toEqual([]);
});
//#endregion 🧪️Declarations

//#region 🧪️MalformedDeclarations
const malformedSchema = JSON.parse(asset("🧪️malformed/🧬️schema/🔣️.json"));
const malformed = JSON.parse(asset("🧪️malformed/🔣️.json")) as { readonly schemaVersion: 1; readonly cases: readonly { readonly id: string; readonly language: "ts"; readonly source: string; readonly compilerDiagnostics: readonly { readonly code: number; readonly start: number; readonly length: number }[]; readonly expected: { readonly completeness: "incomplete"; readonly providerInference: "forbidden" } }[] };
const validateMalformed = ajv.compile(malformedSchema);

test("TypeScript malformed declaration cases use the closed neutral schema", () => {
  expect(validateMalformed(malformed), JSON.stringify(validateMalformed.errors)).toBe(true);
  expect(new Set(malformed.cases.map((row) => row.id)).size).toBe(malformed.cases.length);
  expect(validateMalformed({ schemaVersion: 1, cases: [] })).toBe(false);
});

for (const row of malformed.cases) {
  test("TypeScript malformed declaration reference: " + row.id, () => {
    const diagnostics = compilerParseDiagnostics(row.source, row.language);
    expect(diagnostics.length).toBeGreaterThan(0);
    expect(diagnostics).toEqual(row.compilerDiagnostics);
    for (const diagnostic of diagnostics) {
      expect(diagnostic.start).toBeGreaterThanOrEqual(0);
      expect(diagnostic.start + diagnostic.length).toBeLessThanOrEqual(row.source.length);
    }
  });
  test("TypeScript malformed declaration subject: " + row.id, () => {
    const facts = inspector()(row.source, row.language);
    expect(validateFacts(facts), JSON.stringify(validateFacts.errors)).toBe(true);
    coordinateBounds(row.source, facts);
    expect(facts.completeness).toBe(row.expected.completeness);
    expect(facts.diagnostics.length).toBeGreaterThan(0);
  });
}
//#endregion 🧪️MalformedDeclarations

//#region 🧪️UnsupportedDeclarations
const unsupportedSchema = JSON.parse(asset("🧪️unsupported/🧬️schema/🔣️.json"));
const unsupported = JSON.parse(asset("🧪️unsupported/🔣️.json")) as { readonly schemaVersion: 1; readonly cases: readonly { readonly id: string; readonly language: "ts"; readonly source: string; readonly compilerDiagnostics: readonly never[]; readonly expected: { readonly completeness: "incomplete"; readonly forbiddenDiagnosticCodes: readonly string[] } }[] };
const validateUnsupported = ajv.compile(unsupportedSchema);

test("TypeScript unsupported declaration cases use the closed neutral schema", () => {
  expect(validateUnsupported(unsupported), JSON.stringify(validateUnsupported.errors)).toBe(true);
  expect(new Set(unsupported.cases.map((row) => row.id)).size).toBe(unsupported.cases.length);
  expect(validateUnsupported({ schemaVersion: 1, cases: [] })).toBe(false);
});

for (const row of unsupported.cases) {
  test("TypeScript unsupported declaration reference: " + row.id, () => {
    expect(compilerParseDiagnostics(row.source, row.language)).toEqual(row.compilerDiagnostics);
  });
  test("TypeScript unsupported declaration subject: " + row.id, () => {
    const facts = inspector()(row.source, row.language);
    expect(validateFacts(facts), JSON.stringify(validateFacts.errors)).toBe(true);
    coordinateBounds(row.source, facts);
    expect(facts.completeness).toBe(row.expected.completeness);
    expect(facts.diagnostics.length).toBeGreaterThan(0);
    for (const diagnostic of facts.diagnostics) expect(row.expected.forbiddenDiagnosticCodes).not.toContain(diagnostic.code);
  });
}
//#endregion 🧪️UnsupportedDeclarations
