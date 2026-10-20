import { expect, test } from "bun:test";
import Ajv from "ajv";
import { languageFlagV1, languageRefusalV1, languageSubtagV1, osLanguageListV1, resolveLanguageV1 } from "../🟦️.ts";
import protocol from "../🔣️.json" with { type: "json" };
import schema from "../🧬️schema/🔣️.json" with { type: "json" };
import fixture from "./🔣️.json" with { type: "json" };

type Row = Readonly<{ name: string; args: string[]; environment: Record<string, string>; os: string[]; expected?: { language: string; source: string }; refusal?: string }>;

test("the language protocol satisfies its schema for the independent validator", () => {
  const validate = new Ajv({ strict: true }).compile(schema);
  expect(validate(protocol), JSON.stringify(validate.errors)).toBe(true);
});

for (const row of fixture.resolve as unknown as readonly Row[]) {
  test(`language resolution: ${row.name}`, () => {
    const run = (): ReturnType<typeof resolveLanguageV1> => resolveLanguageV1(fixture.languages, row.args, row.environment, () => row.os);
    if (row.expected) return void expect(run()).toEqual(row.expected);
    let error: Error | undefined;
    try { run(); } catch (caught) { error = caught as Error; }
    expect(error, row.name).toBeDefined();
    const lines = error!.message.split("\n");
    expect(lines.length, "one line per offered language").toBe(fixture.languages.length);
    if (row.refusal === "none") for (const choice of ["--lang=en|de", "SEMIO_LANGUAGE=en|de"]) expect(lines.every((line) => line.includes(choice)), choice).toBe(true);
    if (row.refusal === "flag") expect(lines[0]).toContain("Unknown language");
    if (row.refusal === "flag") expect(lines[1]).toContain("Unbekannte Sprache");
    if (row.refusal === "variable") expect(lines[0]).toContain("SEMIO_LANGUAGE names a language that is not offered");
    if (row.refusal === "variable") expect(lines[1]).toContain("nennt eine nicht angebotene Sprache");
  });
}

test("the language subtag agrees with the platform locale parser", () => {
  for (const [tag, language] of fixture.tags as unknown as readonly (readonly [string, string])[]) {
    expect(languageSubtagV1(tag), tag).toBe(language);
    expect(new Intl.Locale(tag.replace(/[.@].*$/u, "").replace("_", "-")).language, tag).toBe(language);
  }
  for (const tag of fixture.neutral) expect(languageSubtagV1(tag), JSON.stringify(tag)).toBeUndefined();
  expect(languageSubtagV1(undefined)).toBeUndefined();
});

test("the flag reader stops at the separator and keeps the last value", () => {
  expect(languageFlagV1(["a", "--lang", "de", "--", "--lang=en"])).toBe("de");
  expect(languageFlagV1(["--lang=en", "--lang=de"])).toBe("de");
  expect(languageFlagV1([])).toBeUndefined();
});

test("the refusal is built for any offered language set and names its choices", () => {
  expect(languageRefusalV1(["en"]).message.split("\n")).toHaveLength(1);
  expect(languageRefusalV1(["en", "de", "fr"]).message).toContain("--lang=en|de|fr");
});

test("the operating system list is a list of language tags on every platform", () => {
  const list = osLanguageListV1();
  expect(Array.isArray(list)).toBe(true);
  for (const tag of list) expect(languageSubtagV1(tag), tag).toMatch(/^[a-z]{2,3}$/u);
});
