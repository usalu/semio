import { expect, test } from "bun:test";
import Ajv from "ajv";
import { parseTree, getNodeValue, type Node as JsonNode, type ParseError } from "jsonc-parser";
import { spawnSync } from "node:child_process";
import { mkdirSync, writeFileSync } from "node:fs";
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import * as TOML from "@iarna/toml";
import axes from "../../🎚️axes/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import fixture from "../🧫️fixtures/🔣️.json";
import { Locale, Terminology, LocalizedLabel, LocaleContractError, type LabelValue } from "../🟦️.ts";

const ajv = new Ajv({ strict: true });
const validate = ajv.compile(schema), matrix = ajv.compile({ ...schema.definitions.localizedLabel });
const repoRoot = resolve(import.meta.dir, "../../../../..");
const source = (path: string): string => readFileSync(join(repoRoot, path), "utf8");
function duplicates(node: JsonNode): boolean {
  if (node.type === "object") {
    const names = (node.children ?? []).map(property => property.children![0]!.value as string);
    if (new Set(names).size !== names.length) return true;
  }
  return (node.children ?? []).some(duplicates);
}
function oracle(raw: string): { accepted: boolean; value: unknown } {
  const errors: ParseError[] = [], tree = parseTree(raw, errors, { allowTrailingComma: false, disallowComments: true });
  if (!tree || errors.length || duplicates(tree)) return { accepted: false, value: null };
  const value: unknown = getNodeValue(tree);
  return { accepted: matrix(value), value };
}
function retain(name: string, value: unknown): void {
  const output = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!output) throw Error("SEMIO_TEST_ARTIFACT_DIR must name caller-owned ticket output");
  mkdirSync(output, { recursive: true });
  writeFileSync(join(output, "ui-locale-" + name + ".json"), JSON.stringify(value));
}

test("closed portable label and locale authority corpus agrees with canonical axes", () => {
  expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  expect(fixture.axes.locales).toEqual(axes.locales.map(row => row.id));
  expect(fixture.axes.terminologies).toEqual(axes.terminologies.map(row => row.id));
  expect(new Set(fixture.labels.map(row => row.id)).size).toBe(fixture.labels.length);
  expect(new Set(fixture.locales.map(row => row.id)).size).toBe(fixture.locales.length);
  expect(validate({ ...fixture, defaultLocale: "en" })).toBe(false);
  expect(validate({ ...fixture, ownership: { ...fixture.ownership, localeAuthority: "default" } })).toBe(false);
  expect(validate({ ...fixture, labels: fixture.labels.map((row,index)=>index===0?{...row,cells:{...row.cells,extra:"text"}}:row) })).toBe(false);
});

test("UI native manifest has only canonical lower value and locale providers", () => {
  const manifest = TOML.parse(source(fixture.ownership.uiManifest));
  expect(Object.keys(manifest.dependencies ?? {})).not.toContain(fixture.ownership.forbidden);
  expect(Object.keys(manifest.dependencies ?? {})).toContain(fixture.ownership.package);
});

test("UI code has direct neutral value bindings without the product DSL alias", () => {
  expect(source(fixture.ownership.uiRoot)).not.toContain("semio_framework_os_kernel");
  expect(source(fixture.ownership.wgpuRoot)).not.toMatch(/pub\s+use\s+dsl::/);
});

test("locale carriers and defining macro have one lower canonical physical owner", () => {
  const observation = spawnSync("node", ["--eval", "const fs=require('node:fs'),path=require('node:path'),p=JSON.parse(fs.readFileSync(0,'utf8'));process.stdout.write(JSON.stringify(p.map(x=>fs.existsSync(path.join(process.cwd(),x)))));"], { cwd: repoRoot, input: JSON.stringify([fixture.ownership.localeSource, fixture.ownership.oldLocaleSource]), encoding: "utf8", timeout: 4000 });
  expect(observation.status, observation.stderr || String(observation.error)).toBe(0);
  expect(JSON.parse(observation.stdout ?? "")).toEqual([true, false]);
  expect(source(fixture.ownership.kernelRoot)).not.toContain("os_locale");
  expect(source(fixture.ownership.pluginSource)).not.toContain("macro_rules! app_labels");
  expect(source(fixture.ownership.localeSource)).toContain("semio_framework_value");
});

test("the retained node wire law names its actual neutral viewport owner", () => {
  expect(source(fixture.ownership.wireTest)).not.toContain("semio_framework_os_kernel::Viewport2d");
  expect(source(fixture.ownership.wireTest)).toContain(fixture.ownership.viewport);
});

test("generated native locale selection has no implicit first-entry default", () => {
  expect(source(fixture.ownership.axisProjection)).not.toContain("#[default]");
  expect(source(fixture.ownership.axisProjection)).not.toMatch(/derive\([^\n]*\bDefault\b/);
});

for (const row of fixture.labels) test("strict label " + row.id, () => {
  const reference = oracle(row.rawJson);
  expect(reference.accepted).toBe(row.accepted);
  const decode = (): LocalizedLabel => LocalizedLabel.fromValue(row.value as LabelValue);
  if (row.accepted) {
    const actual = decode();
    for (const terminology of fixture.axes.terminologies) for (const locale of fixture.axes.locales) expect(actual.resolve(Terminology.parse(terminology), Locale.parse(locale))).toBe((row.cells as Record<string, string>)[terminology + "." + locale]);
  } else {
    let error: unknown;
    try { decode(); } catch (caught) { error = caught; }
    expect(error).toBeInstanceOf(LocaleContractError);
    expect((error as LocaleContractError).path).toBe(row.errorPath!);
  }
  retain(row.id, { accepted: row.accepted, errorPath: row.errorPath, oracleAccepted: reference.accepted });
});

test("independent Node Intl language-tag parser agrees without selecting an ambient locale", () => {
  const script = "const rows=" + JSON.stringify(fixture.locales) + ";console.log(JSON.stringify(rows.map(row=>{let locale=null;try{const candidate=row.mode==='id'?row.input:new Intl.Locale(row.input).language;if(['en','de'].includes(candidate))locale=candidate;}catch{}return{id:row.id,accepted:locale!==null,locale};})));";
  const result = spawnSync("node", ["--eval", script], { encoding: "utf8" });
  expect(result.status, result.stderr).toBe(0);
  expect(JSON.parse(result.stdout)).toEqual(fixture.locales.map(row => ({ id: row.id, accepted: row.accepted, locale: row.locale })));
  retain("node-authority", JSON.parse(result.stdout));
});

for (const row of fixture.locales) test("explicit locale authority " + row.id, () => {
  const parse = (): Locale => row.mode === "id" ? Locale.parse(row.input) : Locale.fromLanguageTag(row.input);
  if (row.accepted) expect(parse().id).toBe(row.locale!);
  else expect(parse).toThrow(LocaleContractError);
});

test("label resolution refuses absent locale and terminology authority", () => {
  const label = LocalizedLabel.fromValue(fixture.labels[0]!.value as LabelValue);
  expect(() => Locale.parse(undefined)).toThrow("locale");
  expect(() => label.resolve(Terminology.parse("native"), undefined as unknown as Locale)).toThrow("locale");
  expect(() => label.resolve(undefined as unknown as Terminology, Locale.parse("de"))).toThrow("terminology");
});
