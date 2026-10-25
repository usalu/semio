import { expect, test } from "bun:test";
import { readdirSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import Ajv from "ajv";
import { CATALOG_LANGUAGES_V1, defineCatalogV1, placeholdersV1, textV1 } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/📖️catalog/🟦️.ts";
import schema from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/📖️catalog/🧬️schema/🔣️json/🟦️.ts";
import catalog from "../🔣️.json" with { type: "json" };
import { VERIFY_TEXTS_V1 } from "../🟦️.ts";

const LANGUAGE_NEUTRAL: readonly string[] = ["census.agnostic", "census.agnostic-framework-names", "census.agnostic-rule-framework-names-files", "census.agnostic-rule-utility-machines-files", "census.agnostic-utility-machines-utilities-vocabulary", "census.debug-tags", "census.debug-tags-2", "census.debug-tags-files-lines-oracle", "census.docstrings", "census.docstrings-2", "census.docstrings-emoji-unique-repeats", "census.docstrings-rule-emoji-unique-files", "census.docstrings-rule-files-at-emoji", "census.interactivity-commands-declarations-unreachable-non", "census.interface-owners", "census.interface-owners-files-imports-oracle", "census.mutation-caps", "census.mutation-caps-bounded-caplawmissing", "census.mutation-caps-bounded-missing", "census.production-placeholders", "census.production-placeholders-files-production-test", "census.source", "interactivity.apps", "interactivity.apps-actions", "interactivity.apps-actions-dispositionfailures-productionjoinfailures-acceptedcommands", "interactivity.apps-descriptors-extensions-apps-playgrounds", "interactivity.p1q-b1-b6", "interactivity.p1w", "interactivity.p1x", "interactivity.p1y", "interactivity.p1z", "interactivity.p3mn", "interactivity.p5d", "interactivity.p5e", "lanes.build-owners", "lanes.build-owners-2", "lanes.build-owners-sync-removed-added", "lanes.composed-child-refs-assets-mismatched", "lanes.generated-corruption", "lanes.generated-corruption-scanned-jco-bindings", "lanes.history-closure", "lanes.history-closure-2", "lanes.mutation-outcome-law", "lanes.package-purity", "lanes.semantic-vocabulary", "lanes.semantic-vocabulary-roots", "lanes.semantic-vocabulary-scanned-breaches-high", "lanes.taxonomy-implementation-mode-clean-errors", "lanes.taxonomy-implementation-progress-phase-visited", "lanes.taxonomy-mode", "lanes.taxonomy-mode-clean-errors-warnings", "lanes.taxonomy-mode-clean-errors-warnings-2", "lanes.taxonomy-mode-scopes-dirty-from", "lanes.taxonomy-progress", "lanes.zero-touch", "run.abstraction-ownership", "run.artifact-field-parity-missing-extra", "tool-jobs.interactivity", "tool-jobs.interactivity-fixed-operation-fixture-laws", "tool-jobs.interactivity-production-hosts-production-invocations"];
const owner = join(import.meta.dir, "..", "..");
const sources = (directory: string): string[] => readdirSync(directory, { withFileTypes: true }).flatMap((entry) => entry.isDirectory() ? (["🧪️tests", "📖️texts", "node_modules", ".cache"].includes(entry.name) ? [] : sources(join(directory, entry.name))) : /\.(?:ts|mjs)$/u.test(entry.name) ? [join(directory, entry.name)] : []);

test("the catalog satisfies the strict independent validator and the language contract", () => {
  const ajv = new Ajv({ strict: true, allErrors: true });
  expect(ajv.validate(schema, catalog), JSON.stringify(ajv.errors)).toBe(true);
  expect(defineCatalogV1(catalog).languages).toEqual([...CATALOG_LANGUAGES_V1]);
  expect(VERIFY_TEXTS_V1.id).toBe("verify");
});

test("every message renders in both languages with exactly its placeholders and differs between them unless it is language-neutral", () => {
  for (const [key, leaf] of Object.entries(catalog.messages)) {
    const names = placeholdersV1(leaf.en), values = Object.fromEntries(names.map((name) => [name, "v" + name]));
    for (const language of CATALOG_LANGUAGES_V1) {
      const text = textV1(VERIFY_TEXTS_V1.catalog, key, language, values);
      for (const name of names) expect(text, `${key} ${language}`).toContain("v" + name);
    }
    expect(leaf.en === leaf.de, key).toBe(LANGUAGE_NEUTRAL.includes(key));
  }
});

test("every message is used by the code it belongs to and every key the code names exists", () => {
  const code = sources(owner).map((path) => readFileSync(path, "utf8")).join("\n");
  const keys = Object.keys(catalog.messages);
  for (const key of keys) expect(code.includes(JSON.stringify(key)), `${key} is unused`).toBe(true);
  for (const match of code.matchAll(/\b(?:sayVerifyV1|refuseVerifyV1)\(\s*(?:"(?:usage|invalid|missing|refused|failed|breach|stale|unsupported)"\s*,\s*)?("(?:[^"\\]|\\.)*")/gu)) expect(keys, match[1]).toContain(JSON.parse(match[1]!));
});
