import { expect, test } from "bun:test";
import { createHash } from "node:crypto";
import { existsSync, readFileSync } from "node:fs";
import { resolve } from "node:path";
import Ajv from "ajv/dist/2020.js";
import JSON5 from "json5";
import { parse as parseJsonDocument, type ParseError } from "jsonc-parser";
import TOML from "@iarna/toml";
import { createToken, Lexer } from "chevrotain";
import { rustTokens, rustTokenPairs } from "../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";

type Specimen = Readonly<{ path: string; sha256: string; bytes: number; source: string }>;
type Mapping = Readonly<{ role: string; source: string; destination: string; sha256: string }>;
type Capture = Readonly<{ inputs: readonly Specimen[]; callerRows: readonly unknown[]; constructorReferences: readonly Readonly<{ path: string; ownerType: string; operation: string; line: number }>[]; compileReferenceRefusals: readonly Readonly<{ path: string; error: string }>[] }>;
type Corpus = Readonly<{
  owner: string;
  package: Readonly<{ name: string; library: string; manifest: string }>;
  capture: Readonly<{ path: string; sha256: string; inputs: number; callerCandidates: number; constructorReferences: number }>;
  mappings: readonly Mapping[];
  specificFamilies: readonly Readonly<{ source: string; sha256: string; owner: string }>[];
  originalLawCohorts: readonly Readonly<{ source: string; sha256: string; count: number }>[];
  consumers: readonly Readonly<{ owner: string; manifest: string; source: string; root: string; boundary: "complete-package" | "language-only" }>[];
  composition: Readonly<Record<string, unknown>>;
  independentCases: readonly Readonly<{ id: string; text: string; escaped: string; jsonWire: string }>[];
  refusedEscapes: readonly Readonly<{ id: string; escaped: string; error: string }>[];
  lexicalCases: readonly Readonly<{ id: string; text: string; tokens: readonly Readonly<{ kind: string; text: string; start: number; end: number }>[] }>[];
}>;
const owner = resolve(import.meta.dir, "../.."), root = resolve(owner, "../../.."), read = (path: string): string => readFileSync(resolve(root, path), "utf8"), sha = (value: string): string => createHash("sha256").update(value).digest("hex");
const corpus = JSON.parse(readFileSync(resolve(owner, "🧫️fixtures/🧱️ownership/🔣️.json"), "utf8")) as Corpus;
const captured = JSON.parse(read(corpus.capture.path)) as Capture;
const specimens = new Map(captured.inputs.map(row => [row.path, row]));
const grammar = corpus.mappings.find(row => row.role === "grammar")!;
const actualGrammar = (): string => read(existsSync(resolve(root, grammar.destination)) ? grammar.destination : grammar.source);
const tokens = (source: string): readonly string[] => rustTokens(source).map(token => token.text);
const method = (source: string, ownerType: string, name: string): Readonly<{ parameters: string; result: string; body: string }> => {
  const stream = rustTokens(source), pairs = rustTokenPairs(stream);
  for (let index = 0; index < stream.length - 2; index++) {
    if (stream[index]?.text !== "impl" || stream[index + 1]?.text !== ownerType || stream[index + 2]?.text !== "{") continue;
    const end = pairs.get(index + 2)!;
    for (let cursor = index + 3; cursor < end; cursor++) {
      if (stream[cursor]?.text !== "fn" || stream[cursor + 1]?.text !== name || stream[cursor + 2]?.text !== "(") continue;
      const argsEnd = pairs.get(cursor + 2)!, bodyStart = stream.findIndex((token, offset) => offset > argsEnd && token.text === "{");
      if (bodyStart < 0 || bodyStart > end) throw new Error(`missing ${ownerType}::${name} body`);
      const bodyEnd = pairs.get(bodyStart)!;
      return { parameters: source.slice(stream[cursor + 2]!.start, stream[argsEnd]!.end), result: source.slice(stream[argsEnd]!.end, stream[bodyStart]!.start), body: source.slice(stream[bodyStart]!.start, stream[bodyEnd]!.end) };
    }
  }
  throw new Error(`missing ${ownerType}::${name}`);
};

test("the language-neutral ownership schema refuses implicit composition and extra authority", () => {
  const schema = JSON.parse(readFileSync(resolve(owner, "🧬️schema/🧱️ownership/🔣️.json"), "utf8")), validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
  expect(validate(corpus), JSON.stringify(validate.errors)).toBe(true);
  for (const [field, value] of [["fragments", "builtin"], ["macroMatchers", "automatic"], ["unknownFragment", "ignore"], ["defaultFamilyKnowledge", "general-owner"], ["formatClassification", "general-owner"], ["publicCompatibilityForwards", true]] as const) {
    const mutant = structuredClone(corpus) as { composition: Record<string, unknown> };
    mutant.composition[field] = value;
    expect(validate(mutant), field).toBe(false);
  }
  const extra = { ...corpus, absentArtifactFallback: true };
  expect(validate(extra)).toBe(false);
  for (const field of ["composition", "capture", "originalLawCohorts"]) {
    const mutant = structuredClone(corpus) as Record<string, unknown>;
    delete mutant[field];
    expect(validate(mutant), field).toBe(false);
  }
});

test("independent named-reference resolution validates explicit fragment selection and missing-reference refusals", () => {
  const rows = JSON.parse(readFileSync(resolve(owner, "🧫️fixtures/🧩️selection/🔣️.json"), "utf8")) as readonly Readonly<{ id: string; uses: readonly string[]; selectedFragments: readonly string[]; expectedMissing: readonly string[] }>[];
  const schema = JSON.parse(readFileSync(resolve(owner, "🧬️schema/🧩️selection/🔣️.json"), "utf8")), validate = new Ajv({ strict: true }).compile(schema);
  expect(validate(rows)).toBe(true);
  for (const row of rows) {
    const definitions = Object.fromEntries(row.selectedFragments.map(id => [id, { type: "object" }])), missing: string[] = [];
    for (const id of row.uses) {
      try {
        new Ajv({ strict: true }).compile({ $ref: `#/$defs/${encodeURIComponent(id.replaceAll("~", "~0").replaceAll("/", "~1"))}`, $defs: definitions });
      } catch (error) {
        expect(String(error), `${row.id}:${id}`).toContain("can't resolve reference");
        missing.push(id);
      }
    }
    expect(missing, row.id).toEqual(row.expectedMissing);
  }
  expect(validate([{ ...rows[0], implicitDefaults: true }])).toBe(false);
  expect(validate([{ ...rows[1], selectedFragments: ["terms", "terms"] }])).toBe(false);
  const absent = { ...rows[0] } as Record<string, unknown>;
  delete absent.selectedFragments;
  expect(validate([absent])).toBe(false);
});

test("every original specimen and constructor reference retains its exact captured authority", () => {
  expect(sha(read(corpus.capture.path))).toBe(corpus.capture.sha256);
  expect(captured.inputs.length).toBe(corpus.capture.inputs);
  expect(captured.callerRows.length).toBe(corpus.capture.callerCandidates);
  expect(captured.constructorReferences.length).toBe(corpus.capture.constructorReferences);
  expect(specimens.size).toBe(captured.inputs.length);
  for (const row of captured.inputs) {
    expect(sha(row.source), row.path).toBe(row.sha256);
    expect(Buffer.byteLength(row.source), row.path).toBe(row.bytes);
  }
  for (const row of captured.constructorReferences) {
    const source = specimens.get(row.path)!.source, stream = rustTokens(source);
    expect(stream.some((token, index) => token.text === row.ownerType && stream[index + 1]?.text === "::" && stream[index + 2]?.text === row.operation && stream[index + 3]?.text === "("), `${row.path}:${row.line}`).toBe(true);
  }
  for (const row of captured.compileReferenceRefusals) expect(row.error).toContain("Unsupported Rust compile expression");
});

test("all original lexer compiler grammar and literal law cohorts remain captured and unedited", () => {
  for (const row of corpus.originalLawCohorts) {
    expect(specimens.get(row.source)?.sha256, row.source).toBe(row.sha256);
    expect(sha(read(row.source)), row.source).toBe(row.sha256);
    const sourceTokens = tokens(specimens.get(row.source)!.source), laws = sourceTokens.filter((token, index) => token === "async_test" && sourceTokens[index - 1] === "::" || token === "test" && sourceTokens[index - 1] === "[" && sourceTokens[index - 2] === "#").length;
    expect(laws, row.source).toBe(row.count);
  }
  for (const row of corpus.mappings) expect(sha(read(row.source)), row.source).toBe(row.sha256);
});

test("all seven current family assets stay at their actual specific owners", () => {
  const originalFamilies = captured.inputs.filter(row => row.path.includes("/👪️family/") && row.path.endsWith("/📖️.grammar.semio"));
  expect(new Set(corpus.specificFamilies.map(row => row.source))).toEqual(new Set(originalFamilies.map(row => row.path)));
  for (const row of corpus.specificFamilies) {
    expect(row.source.startsWith(`${row.owner}/`)).toBe(true);
    expect(row.source.startsWith(`${corpus.owner}/`)).toBe(false);
    expect(sha(read(row.source)), row.source).toBe(row.sha256);
  }
});

test("independent JSON codecs validate shared escape goldens and strict refusals", () => {
  for (const row of corpus.independentCases) {
    const errors: ParseError[] = [];
    expect(JSON.parse(row.jsonWire), row.id).toBe(row.text);
    expect(JSON5.parse(row.jsonWire), row.id).toBe(row.text);
    expect(parseJsonDocument(row.jsonWire, errors), row.id).toBe(row.text);
    expect(errors, row.id).toEqual([]);
    expect(JSON.stringify(row.text), row.id).toBe(row.jsonWire);
    expect(row.jsonWire, row.id).toBe(`"${row.escaped}"`);
  }
  for (const row of corpus.refusedEscapes) {
    const errors: ParseError[] = [];
    expect(() => JSON.parse(`"${row.escaped}"`), row.id).toThrow();
    parseJsonDocument(`"${row.escaped}"`, errors);
    expect(errors.length, row.id).toBeGreaterThan(0);
  }
});

test("the existing independent lexer produces the declared significant token and byte-range goldens", () => {
  const space = createToken({ name: "Space", pattern: /[ \t\r\n]+/, group: Lexer.SKIPPED, line_breaks: true }), ident = createToken({ name: "Ident", pattern: /[A-Za-z_][A-Za-z0-9_.\/-]*/ }), int = createToken({ name: "Int", pattern: /[0-9]+/ }), equals = createToken({ name: "Equals", pattern: /=/ }), comma = createToken({ name: "Comma", pattern: /,/ });
  const lexer = new Lexer([space, ident, int, equals, comma]);
  for (const row of corpus.lexicalCases) {
    const result = lexer.tokenize(row.text);
    expect(result.errors, row.id).toEqual([]);
    expect(result.tokens.map(token => ({ kind: token.tokenType.name, text: token.image, start: token.startOffset, end: token.endOffset! + 1 })), row.id).toEqual(row.tokens);
  }
});

test.each(corpus.consumers)("the actual $owner language binding directly selects the general provider", (consumer) => {
    const text = read(consumer.manifest), native = Bun.TOML.parse(text) as { dependencies?: Record<string, string | { package?: string; path?: string }> }, thirdParty = TOML.parse(text) as typeof native;
    expect(native).toEqual(thirdParty);
    const dependencies = Object.entries(native.dependencies ?? {}), kernel = dependencies.filter(([name, row]) => name === "semio-framework-os-kernel" || typeof row === "object" && row.package === "semio-framework-os-kernel");
    if (consumer.boundary === "complete-package") expect(kernel, `${consumer.manifest}: current concrete product edge`).toEqual([]);
    expect(dependencies.some(([name, row]) => name === corpus.package.name || typeof row === "object" && row.package === corpus.package.name), consumer.manifest).toBe(true);
    expect(tokens(read(consumer.source)).includes("os_dsl"), consumer.source).toBe(false);
});

test("actual lower vocabulary owns the generic definitions without a product facade", () => {
  const deficits = corpus.mappings.filter(row => !existsSync(resolve(root, row.destination))).map(row => ({ role: row.role, currentOwner: row.source, expectedOwner: row.destination }));
  expect(deficits, "real generic definitions remain mounted by OS Kernel").toEqual([]);
  expect(existsSync(resolve(root, corpus.package.manifest)), "actual generic Cargo owner").toBe(true);
  for (const row of corpus.mappings) expect(tokens(read(row.destination)).includes("os_dsl"), row.destination).toBe(false);
});

test("the actual recognizer requires selected fragment and macro registrations and refuses missing fragments", () => {
  const source = actualGrammar(), compile = method(source, "Recognizer", "compile"), stream = tokens(source);
  expect(compile.parameters, "actual implicit one-argument constructor").toContain("FragmentRegistry");
  expect(compile.parameters).toContain("MacroMatcher");
  expect(compile.result, "missing selected fragment must produce an owned error").toContain("Result");
  expect(tokens(compile.body).includes("Err"), "actual missing fragment must refuse").toBe(true);
  expect(stream.includes("default_macros"), "specific matcher defaults cannot remain lower").toBe(false);
  expect(stream.includes("verify_protocol_bytes"), "specific shallow envelope classifier cannot remain lower").toBe(false);
  expect(stream.some((token, index) => token === "fn" && stream[index + 1] === "builtin"), "embedded family registry cannot remain lower").toBe(false);
  for (const row of corpus.specificFamilies) expect(source.includes(row.source.split("/👪️family/")[1]!), row.source).toBe(false);
});
