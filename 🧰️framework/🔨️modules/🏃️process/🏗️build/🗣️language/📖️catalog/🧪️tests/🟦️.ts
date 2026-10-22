import { expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { fileURLToPath } from "node:url";
import Ajv from "ajv";
import { testPythonV1 } from "../../../../🧪️testing/🧰️toolchain/🟦️.ts";
import { CATALOG_LANGUAGES_V1, LanguageTextErrorV1, bindCatalogV1, catalogHandleV1, defineCatalogV1, handleTextV1, parseTemplateV1, placeholdersV1, qualifiedKeyV1, textV1, type LanguageCatalogV1 } from "../🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with { type: "json" };
import corpus from "./🔣️.json" with { type: "json" };
import corpusSchema from "./🧬️schema/🔣️.json" with { type: "json" };

type Case = Readonly<{ name: string; key: string; language: string; values: Record<string, string | number>; expected?: string; refusal?: string }>;
type Oracle = Readonly<{ render: readonly Readonly<{ name: string; text?: string; refusal?: string }>[]; templates: readonly Readonly<{ template: string; placeholders?: string[]; refusal?: string }>[]; documents: readonly Readonly<{ name: string; valid: boolean }>[] }>;

const catalog = defineCatalogV1(corpus.catalog);
const render = corpus.render as unknown as readonly Case[];
const refusal = (run: () => unknown): string | undefined => {
  try { run(); } catch (error) { return error instanceof LanguageTextErrorV1 ? error.reason : `other:${String(error)}`; }
  return undefined;
};
const oracle = (): Oracle => {
  const run = spawnSync(testPythonV1() ?? "python3", ["-I", fileURLToPath(new URL("./🐍️oracle.py", import.meta.url))], { encoding: "utf8", input: JSON.stringify(corpus), maxBuffer: 1 << 24 });
  if (run.status !== 0) throw new Error(`oracle failed: ${run.stderr}`);
  return JSON.parse(run.stdout) as Oracle;
};

test("the schema and the corpus satisfy the strict independent validator", () => {
  const ajv = new Ajv({ strict: true, allErrors: true });
  expect(ajv.validate(corpusSchema, corpus), JSON.stringify(ajv.errors)).toBe(true);
  expect(ajv.validate(schema, corpus.catalog), JSON.stringify(ajv.errors)).toBe(true);
  expect(CATALOG_LANGUAGES_V1).toEqual(["en", "de"]);
});

for (const row of render) {
  test(`render: ${row.name}`, () => {
    if (row.refusal !== undefined) return void expect(refusal(() => textV1(catalog, row.key, row.language, row.values))).toBe(row.refusal);
    expect(textV1(catalog, row.key, row.language, row.values)).toBe(row.expected!);
  });
}

test("the templates are parsed like the corpus says", () => {
  for (const row of corpus.templates as readonly Readonly<{ template: string; placeholders?: string[]; refusal?: string }>[]) {
    if (row.refusal !== undefined) expect(refusal(() => parseTemplateV1(row.template)), row.template).toBe(row.refusal);
    else expect([...placeholdersV1(row.template)], row.template).toEqual(row.placeholders!);
  }
});

test("the admission of documents agrees with the corpus and with the strict schema validator", () => {
  const ajv = new Ajv({ strict: true, allErrors: true });
  for (const row of corpus.documents) {
    expect(refusal(() => defineCatalogV1(row.document)) === undefined, row.name).toBe(row.valid);
    expect(ajv.validate(schema, row.document), `${row.name} ${JSON.stringify(ajv.errors)}`).toBe(row.schemaValid);
  }
});

test("the independent oracle renders, parses and admits exactly like the implementation", () => {
  const other = oracle();
  for (const [index, row] of render.entries()) {
    const got = other.render[index]!;
    expect(got.name).toBe(row.name);
    expect(got.text, row.name).toBe(row.expected);
    expect(got.refusal, row.name).toBe(row.refusal);
  }
  for (const [index, row] of corpus.templates.entries()) {
    expect(other.templates[index]!.placeholders, row.template).toEqual((row as { placeholders?: string[] }).placeholders);
    expect(other.templates[index]!.refusal, row.template).toBe((row as { refusal?: string }).refusal);
  }
  for (const [index, row] of corpus.documents.entries()) expect(other.documents[index]!.valid, row.name).toBe(row.valid);
});

test("a catalog is bound once per namespace and found again by its qualified key", () => {
  const first = bindCatalogV1("demo", corpus.catalog);
  expect(bindCatalogV1("demo", JSON.parse(JSON.stringify(corpus.catalog)))).toEqual(first);
  expect(catalogHandleV1("demo").catalog).toEqual(first.catalog);
  expect(qualifiedKeyV1("demo.nested.key-1").key).toBe("nested.key-1");
  expect(handleTextV1(first, "greet", "de", { name: "Ada" })).toBe("Hallo Ada");
  const changed = JSON.parse(JSON.stringify(corpus.catalog)) as { messages: Record<string, { en: string; de: string }> };
  changed.messages.plain!.en = "Changed.";
  expect(refusal(() => bindCatalogV1("demo", changed))).toBe("catalog-conflict");
  expect(refusal(() => bindCatalogV1("other", corpus.catalog))).toBe("invalid-catalog");
  expect(refusal(() => catalogHandleV1("absent"))).toBe("unknown-catalog");
  expect(refusal(() => qualifiedKeyV1("nodot"))).toBe("unknown-key");
});

test("a refusal names the reason and the detail and never holds English prose", () => {
  const error = (() => { try { textV1(catalog, "greet", "en"); } catch (caught) { return caught as LanguageTextErrorV1; } throw new Error("expected"); })();
  expect(error.reason).toBe("missing-placeholder");
  expect(error.message).toBe("missing-placeholder:greet:name");
  expect(error.message).not.toContain(" ");
});

test("the loaded catalogs are immutable and offer exactly the schema languages", () => {
  const typed: LanguageCatalogV1 = catalog;
  expect(Object.isFrozen(typed)).toBe(true);
  expect(typed.languages).toEqual([...CATALOG_LANGUAGES_V1]);
});
