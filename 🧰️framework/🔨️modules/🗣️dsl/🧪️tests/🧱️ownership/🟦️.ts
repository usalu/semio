import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import Ajv from "ajv/dist/2020.js";
import JSON5Codec from "json5";
import { parse as parseJsonDocument, type ParseError } from "jsonc-parser";
import TOML from "@iarna/toml";
import { createToken, Lexer } from "chevrotain";

const JSON5: Readonly<{parse(text: string): unknown}> = JSON5Codec;

type Corpus = Readonly<{
  composition: Readonly<Record<string, unknown>>;
  independentCases: readonly Readonly<{id: string; text: string; escaped: string; jsonWire: string}>[];
  refusedEscapes: readonly Readonly<{id: string; escaped: string; error: string}>[];
  lexicalCases: readonly Readonly<{id: string; text: string; tokens: Readonly<{kind: string; text: string; start: number; end: number}>[]}>[];
}>;
const owner = resolve(import.meta.dir, "../.."), root = resolve(owner, "../../.."), read = (path: string): string => readFileSync(resolve(root, path), "utf8");
const corpus = JSON.parse(readFileSync(resolve(owner, "🧫️fixtures/🧱️ownership/🔣️.json"), "utf8")) as Corpus;
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
  const rows = JSON.parse(readFileSync(resolve(owner, "🧫️fixtures/🧩️selection/🔣️.json"), "utf8")) as readonly Readonly<{ id: string; uses: readonly string[]; selectedFragments: readonly string[]; expectedMissing: string[] }>[];
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

test("canonical diagnostic projections preserve the shared language-neutral fault schema and Serde oracle", () => {
  const validate = new Ajv({strict: true, allErrors: true}).compile(JSON.parse(read("🧰️framework/🔨️modules/⚠️diagnostic/🧬️schema/🧯️fault/🔣️.json")));
  const fixture = JSON.parse(read("🧰️framework/🔨️modules/⚠️diagnostic/🧫️fixtures/🧯️fault/🔣️.json"));
  expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  expect(validate({...fixture, productDefaults: true})).toBe(false);
  for (const row of fixture.cases) {
    expect(JSON5.parse(JSON.stringify(row.expected))).toEqual(row.expected);
    expect(parseJsonDocument(JSON.stringify(row.expected))).toEqual(row.expected);
  }
  const laws = read("🧰️framework/🔨️modules/⚠️diagnostic/🧪️tests/🔬️fault-describe/🦀️.rs");
  expect(laws).toContain("serde_json::from_value");
  expect(laws).toContain("assert_eq!(actual, row[\"expected\"]");
});

test("actual neutral packages expose canonical lexical and diagnostic ownership", () => {
  const manifest = read("🧰️framework/🔨️modules/🗣️dsl/📦️packages/🦀️rust/Cargo.toml");
  const own = TOML.parse(manifest) as {dependencies: Record<string, unknown>};
  expect(Bun.TOML.parse(manifest)).toEqual(own);
  expect(Object.keys(own.dependencies).sort()).toEqual(["semio-framework-diagnostic", "semio-framework-value"]);
  const library = read("🧰️framework/🔨️modules/🗣️dsl/📦️packages/🦀️rust/🦀️.rs");
  for (const path of ["🔤️token/🦀️.rs", "🔍️lexer/🦀️.rs", "🎖️trust/🦀️.rs", "📖️grammar/🦀️.rs", "🗣️idiom/🦀️.rs"]) expect(library).toContain(path);
  expect(library).toContain("pub use semio_framework_diagnostic::{TextError,Limits,TextSpan");
  const grammar = read("🧰️framework/🔨️modules/🗣️dsl/📖️grammar/🦀️.rs");
  expect(grammar).toContain("pub fn compile(grammar: &GrammarFile, registry: &FragmentRegistry, macros: Vec<MacroMatcher>) -> Result<Self, TextError>");
  expect(grammar).toContain("unregistered grammar fragment");
});
