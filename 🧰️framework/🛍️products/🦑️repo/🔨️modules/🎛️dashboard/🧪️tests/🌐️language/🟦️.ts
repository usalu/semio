import { expect, test } from "bun:test";
import Ajv from "ajv";
import { readFileSync } from "node:fs";
import { emitUiAxesRust } from "../../../../../../🔨️modules/🖱️ui/🎚️axes/📽️projection/🟦️.ts";

const text = (path: string) => readFileSync(new URL(path, import.meta.url), "utf8");
const read = (path: string) => JSON.parse(text(path));
const declaration = read("../../⚙️preferences/🌐️language/🔣️.json");
const schema = read("../../🧬️schema/⚙️preferences/🌐️language/🔣️.json");
const vectors = read("../🧫️fixtures/🌐️language/🔣️.json");
const axes = read("../../../../../../🔨️modules/🖱️ui/🎚️axes/🔣️.json");
const supported: string[] = axes.locales.map((entry: { id: string }) => entry.id);
const validate = new Ajv({ strict: true }).compile(schema);

const intlLanguage = (value: string): string | null => {
  const [head = ""] = value.split(/[.@]/u);
  try { const language = new Intl.Locale(head.replaceAll("_", "-")).language; return supported.includes(language) ? language : null; } catch { return null; }
};

const resolve = (chain: { systemVariables: string[]; fallback: string[] }, environment: Record<string, string>): string => {
  for (const name of chain.systemVariables) { const named = environment[name] === undefined ? null : intlLanguage(environment[name]!); if (named) return named; }
  return chain.fallback[0]!;
};

test("Ajv independently validates the shipped language chain and refuses every invalid declaration", () => {
  expect(validate(declaration), JSON.stringify(validate.errors)).toBe(true);
  for (const row of vectors.invalidDeclarations) expect(validate(row), JSON.stringify(row)).toBe(false);
  for (const row of vectors.fallbackOrders) expect(validate(row.declaration), JSON.stringify(row)).toBe(true);
  expect(declaration.fallback.every((id: string) => supported.includes(id))).toBe(true);
});

test("Intl.Locale independently negotiates the system values of the shared vectors", () => {
  for (const row of vectors.negotiate) expect(intlLanguage(row.value), JSON.stringify(row)).toBe(row.language);
});

test("an independent resolver follows the declared chain for every shared environment", () => {
  for (const row of vectors.resolve) expect(resolve(declaration, row.environment), JSON.stringify(row)).toBe(row.expected);
  for (const row of vectors.fallbackOrders) expect(resolve(row.declaration, {}), JSON.stringify(row)).toBe(row.expected);
});

test("the dashboard holds no default language in code", () => {
  const preferences = text("../../⚙️preferences/🦀️.rs");
  expect(preferences).not.toMatch(/Locale::default\(\)/u);
  expect(preferences).not.toMatch(/Locale::parse\([^\n]*unwrap_or_default/u);
  expect(preferences).not.toMatch(/language:\s*"[a-z]{2}"\.into\(\)/u);
  expect(preferences).toContain("language::declared()");
  const generated = emitUiAxesRust(axes);
  expect(generated).not.toMatch(/\bDefault\b|#\[default\]/u);
});

test("every scenario of the language chain is proved by a test of this file and by the named Rust tests", () => {
  const feature = text("../⚙️preferences/🥒️.feature");
  const own = text(import.meta.url);
  const rust = text("../../⚙️preferences/🧪️tests/🔬️unit/🦀️.rs") + "\n" + text("../../⚙️preferences/🌐️language/🧪️tests/🔬️unit/🦀️.rs");
  const title = "The language falls back from the stored choice to the system and then to the declared chain";
  expect(feature.includes(`Scenario: ${title}`)).toBe(true);
  for (const name of ["Ajv independently validates the shipped language chain and refuses every invalid declaration", "Intl.Locale independently negotiates the system values of the shared vectors", "an independent resolver follows the declared chain for every shared environment"]) expect(own.includes(`test(${JSON.stringify(name)}`), name).toBe(true);
  for (const name of ["the_language_comes_from_the_system_and_then_the_declared_fallback_below_every_preference_layer", "compaction_keeps_the_system_language_free_unless_the_journal_chose_one", "system_values_name_a_supported_language_per_the_shared_vectors", "the_declared_chain_resolves_the_shared_environments", "the_first_declared_fallback_is_used_when_the_system_names_none", "a_declaration_without_a_supported_fallback_is_refused"]) expect(rust.slice(0, rust.indexOf(`fn ${name}(`)).trimEnd().endsWith("#[test]"), name).toBe(true);
});
