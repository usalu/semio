import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import Ajv from "ajv";
import { CATALOG_LANGUAGES_V1, defineCatalogV1, placeholdersV1, textV1 } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/📖️catalog/🟦️.ts";
import schema from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/📖️catalog/🧬️schema/🔣️json/🟦️.ts";
import catalog from "../🔣️.json" with { type: "json" };
import { DIRECTION_TEXTS_V1 } from "../🟦️.ts";

test("the catalog satisfies the strict independent validator and the language contract", () => {
  const ajv = new Ajv({ strict: true, allErrors: true });
  expect(ajv.validate(schema, catalog), JSON.stringify(ajv.errors)).toBe(true);
  expect(defineCatalogV1(catalog).languages).toEqual([...CATALOG_LANGUAGES_V1]);
  expect(DIRECTION_TEXTS_V1.id).toBe("direction");
});

test("every message renders in both languages with exactly its placeholders and differs between them", () => {
  for (const [key, leaf] of Object.entries(catalog.messages)) {
    const names = placeholdersV1(leaf.en), values = Object.fromEntries(names.map((name) => [name, "v" + name]));
    for (const language of CATALOG_LANGUAGES_V1) {
      const text = textV1(DIRECTION_TEXTS_V1.catalog, key, language, values);
      for (const name of names) expect(text, `${key} ${language}`).toContain("v" + name);
    }
    expect(leaf.en === leaf.de, key).toBe(false);
  }
});

test("every message is used by the law and every key the law names exists", () => {
  const code = readFileSync(join(import.meta.dir, "..", "..", "🟦️.ts"), "utf8"), keys = Object.keys(catalog.messages);
  for (const key of keys) expect(code.includes(JSON.stringify(key)), `${key} is unused`).toBe(true);
  for (const match of code.matchAll(/\brefuseDirectionV1\(\s*"[a-z]+"\s*,\s*("(?:[^"\\]|\\.)*")/gu)) expect(keys, match[1]).toContain(JSON.parse(match[1]!));
});
