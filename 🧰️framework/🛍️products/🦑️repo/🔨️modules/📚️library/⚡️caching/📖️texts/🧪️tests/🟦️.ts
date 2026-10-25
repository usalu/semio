import { expect, test } from "bun:test";
import { readdirSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import Ajv from "ajv";
import { CATALOG_LANGUAGES_V1, defineCatalogV1, placeholdersV1, textV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/📖️catalog/🟦️.ts";
import schema from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/📖️catalog/🧬️schema/🔣️json/🟦️.ts";
import catalog from "../🔣️.json" with { type: "json" };
import { CACHING_TEXTS_V1 } from "../🟦️.ts";

const LANGUAGE_NEUTRAL: readonly string[] = [];
const owner = join(import.meta.dir, "..", "..");
const sources = (directory: string): string[] => readdirSync(directory, { withFileTypes: true }).flatMap((entry) => entry.isDirectory() ? (["🧪️tests", "📖️texts", "node_modules", ".cache"].includes(entry.name) ? [] : sources(join(directory, entry.name))) : /\.(?:ts|mjs)$/u.test(entry.name) ? [join(directory, entry.name)] : []);

test("the catalog satisfies the strict independent validator and the language contract", () => {
  const ajv = new Ajv({ strict: true, allErrors: true });
  expect(ajv.validate(schema, catalog), JSON.stringify(ajv.errors)).toBe(true);
  expect(defineCatalogV1(catalog).languages).toEqual([...CATALOG_LANGUAGES_V1]);
  expect(CACHING_TEXTS_V1.id).toBe("caching");
});

test("every message renders in both languages with exactly its placeholders and differs between them unless it is language-neutral", () => {
  for (const [key, leaf] of Object.entries(catalog.messages)) {
    const names = placeholdersV1(leaf.en), values = Object.fromEntries(names.map((name) => [name, "v" + name]));
    for (const language of CATALOG_LANGUAGES_V1) {
      const text = textV1(CACHING_TEXTS_V1.catalog, key, language, values);
      for (const name of names) expect(text, `${key} ${language}`).toContain("v" + name);
    }
    expect(leaf.en === leaf.de, key).toBe(LANGUAGE_NEUTRAL.includes(key));
  }
});

test("every message is used by the code it belongs to and every key the code names exists", () => {
  const code = sources(owner).map((path) => readFileSync(path, "utf8")).join("\n");
  const keys = Object.keys(catalog.messages);
  for (const key of keys) expect(code.includes(JSON.stringify(key)), `${key} is unused`).toBe(true);
  for (const match of code.matchAll(/\b(?:refuseCachingV1)\(\s*(?:"(?:usage|invalid|missing|refused|failed|breach|stale|unsupported|cancelled)"\s*,\s*)?("(?:[^"\\]|\\.)*")/gu)) expect(keys, match[1]).toContain(JSON.parse(match[1]!));
});
