import { expect, test } from "bun:test";
import Ajv from "ajv";
import { Database } from "bun:sqlite";
import { readFileSync } from "node:fs";
import { emitUiAxesRust } from "@semio-tech/ui/axes/projection";

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

const protocol = read("../../../../../../🔨️modules/⚙️runtime/🗣️language/🔣️.json");
const shared = read("../../../../../../🔨️modules/⚙️runtime/🗣️language/🧪️tests/🔣️.json");

const resolve = (environment: Record<string, string>): string | null => {
  const explicit = environment[protocol.explicitVariable];
  if (explicit) return intlLanguage(explicit.trim());
  for (const entry of (environment[protocol.preferenceListVariable] ?? "").split(protocol.preferenceListSeparator)) { const named = entry ? intlLanguage(entry) : null; if (named) return named; }
  const name = protocol.environmentOrder.find((candidate: string) => environment[candidate]);
  return name === undefined ? null : intlLanguage(environment[name]!);
};

test("Ajv independently validates the shipped language chain and refuses every invalid declaration", () => {
  expect(validate(declaration), JSON.stringify(validate.errors)).toBe(true);
  for (const row of vectors.invalidDeclarations) expect(validate(row), JSON.stringify(row)).toBe(false);
  for (const row of vectors.choiceOrders) expect(validate(row.declaration), JSON.stringify(row)).toBe(true);
  expect(declaration.choices.every((choice: {language:string}) => supported.includes(choice.language))).toBe(true);
});

test("Intl.Locale independently negotiates the system values of the shared vectors", () => {
  for (const row of vectors.negotiate) expect(intlLanguage(row.value), JSON.stringify(row)).toBe(row.language);
});

test("an independent resolver follows the declared chain for every shared environment", () => {
  const rows = shared.resolve.filter((row: { args: string[]; os: string[] }) => row.args.length === 0 && row.os.length === 0);
  expect(rows.length).toBeGreaterThanOrEqual(15);
  for (const row of rows) expect(resolve(row.environment), JSON.stringify(row)).toBe(row.expected?.language ?? null);
  for (const row of vectors.choiceOrders) expect(resolve({}), JSON.stringify(row)).toBe(row.expected);
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
  const title = "The language resolves from explicit choices or system settings and otherwise requires a choice";
  expect(feature.includes(`Scenario: ${title}`)).toBe(true);
  for (const name of ["Ajv independently validates the shipped language chain and refuses every invalid declaration", "Intl.Locale independently negotiates the system values of the shared vectors", "an independent resolver follows the declared chain for every shared environment"]) expect(own.includes(`test(${JSON.stringify(name)}`), name).toBe(true);
  for (const name of ["the_language_resolves_explicit_layers_and_system_without_a_default", "compaction_keeps_the_system_language_free_unless_the_journal_chose_one", "system_values_name_a_supported_language_per_the_shared_vectors", "the_declared_chain_resolves_the_shared_environments", "no_declared_choice_is_selected_when_the_system_names_none", "a_declaration_without_supported_choices_is_refused"]) expect(rust.slice(0, rust.indexOf(`fn ${name}(`)).trimEnd().endsWith("#[test]"), name).toBe(true);
});

test("SQLite independently resolves explicit language choice transitions without an initial selection", () => {
  const database = new Database(":memory:");
  try { for(const row of vectors.selection) { let selected: string|null=null, outcome="pending"; for(const action of row.actions) { const next=database.query("SELECT CASE WHEN ? IN ('1','down','tab') AND ? IS NULL THEN 'en' WHEN ?='up' AND ? IS NULL THEN 'de' WHEN ?='1' THEN 'en' WHEN ?='2' THEN 'de' WHEN ? IN ('up','down','tab') THEN CASE WHEN ?='en' THEN 'de' ELSE 'en' END ELSE ? END selected").get(action,selected,action,selected,action,action,action,selected,selected) as {selected:string|null}; selected=next.selected; if(action==='escape')outcome='cancelled'; else if(action==='enter'&&selected)outcome='accepted'; } expect({selected,outcome}).toEqual({selected:row.selected,outcome:row.outcome}); }} finally { database.close(); }
});

test("SQLite independently distinguishes cancellation from an accepted language event",()=>{const db=new Database(":memory:");try{for(const row of vectors.publication){const actual=db.query("select case when ?1 then 1 else 0 end events,case when ?1 then 'saved' else 'cancelled' end outcome").get(row.committed?1:0)as{events:number;outcome:string};expect(actual).toEqual({events:row.events,outcome:row.outcome});}}finally{db.close();}});
