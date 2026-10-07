/** 🎛️ The declaration rules of the `schema-mutation-input-ui` gate (design §22.8) over `🧫️fixtures/🧫️mutation-input-declarations/🔣️.json`
 * every planted payload schema yields exactly the listed declaration rows and
 * `[code, pointer]` findings, and an independent oracle reaches the same findings — a strict Ajv resolves every `$ref`, tells a
 * number by validating one, and evaluates the rule schemas the gate schema states (`leafShowsInput` over the leaf, `numericDeclared`,
 * `labelResolved`, `widgetDeclared`, `multilineControlled` over each input) on what it resolves by its own traversal; a leaf its
 * descriptor declares withdraw-only (`editable: false`) is judged by neither. The gate's widget vocabulary is the
 * one Ajv reads from the strict vocabulary, its top-level inputs are the ones the manifest reader reads, and the multi-line rule is
 * armed exactly when that reader hands a `multiline` input the multi-line control. */
import { describe, expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import type Ajv from "ajv";
import { mutationInputAudit } from "../../../../../../🔨️modules/🛂️manifest/🟦️.ts";
import { semioSchemaAjvV1 } from "../../../../../../🔨️modules/🧬️schema/🔮️oracles/✅️validator/🟦️.ts";
import { type MutationInputDeclaration, type MutationInputDeclarationCode, mutationInputControls, mutationInputDeclarationFindings, mutationInputDeclarations, mutationInputMultilineArmed, mutationInputWidgetVocabulary } from "../../🧬️schema/📋️orchestration/🟦️.ts";

type Json = Record<string, unknown>;
type Finding = readonly [MutationInputDeclarationCode, string];
type Case = { readonly id: string; readonly input: { readonly schema: Json }; readonly editable?: boolean; readonly controls?: Readonly<Record<string, string>>; readonly declarations: readonly MutationInputDeclaration[]; readonly findings: readonly Finding[] };
type Fixture = { readonly documents: readonly (Json & { readonly $id: string })[]; readonly cases: readonly Case[] };
type Resolved = { readonly key: string; readonly type: string | null; readonly labelled: boolean; readonly glossary: boolean; readonly ui: Json; readonly control: string | null };

const repoRoot = resolve(import.meta.dir, "../../../../../../..");
const read = (path: string): Json => JSON.parse(readFileSync(resolve(repoRoot, path), "utf8")) as Json;
const fixture = read("🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧫️fixtures/🧫️mutation-input-declarations/🔣️.json") as unknown as Fixture;
const schema = read("🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/🔣️mutation-input-declarations/🔣️.json") as Json & { readonly $id: string };
const manifest = read("🧰️framework/🔨️modules/🛂️manifest/🧬️schema/🔣️.json") as Json & { readonly $id: string };
const glossary = new Set(Object.keys(read("🧰️framework/🔨️modules/🛂️manifest/🔣️input-labels.json").labels as Json));
const documents = new Map<string, Json>([[manifest.$id, manifest], ...fixture.documents.map((document) => [document.$id, document] as const)]);
const RULES: readonly (readonly [string, MutationInputDeclarationCode])[] = [["widgetDeclared", "widgetUndeclared"], ["numericDeclared", "numericUndeclared"], ["labelResolved", "labelAbsent"], ["multilineControlled", "multilineUncontrolled"]];
const ITEM_FACETS = ["unit", "step", "precision", "snaps", "snapSource", "displayUnit", "displayFactor"];

/** 🧬️ The strict Ajv oracle holding the gate schema and the fixture's shared documents. */
function oracleAjv(): Ajv {
  const ajv = semioSchemaAjvV1({ allErrors: true });
  ajv.addSchema(schema);
  for (const document of fixture.documents) ajv.addSchema(document);
  return ajv;
}

/** 🔮️ The independent oracle: every leaf input of a planted payload schema by its own traversal — Ajv resolves each `$ref`, a node
 * without a declared type is a number when Ajv validates both a plain number and the binary64 word against it — as the
 * `resolvedInput` record the gate schema's rules judge, keyed by payload pointer in reading order. */
function oracleInputs(ajv: Ajv, entry: Case): (readonly [string, Resolved])[] {
  const found: (readonly [string, Resolved])[] = [];
  const settle = (start: Json): { readonly node: Json; readonly ui: Json } => {
    let node = start;
    let ui: Json = {};
    for (;;) {
      ui = { ...((node["x-semio-ui"] as Json | undefined) ?? {}), ...ui };
      const branches = (node.oneOf ?? node.anyOf) as Json[] | undefined;
      const values = (branches ?? []).filter((branch) => !(branch.type === "null" && Object.keys(branch).length === 1));
      if (typeof node.$ref === "string") node = ajv.getSchema(node.$ref)!.schema as Json;
      else if (branches !== undefined && values.length === 1 && branches.length > 1) node = values[0]!;
      else return { node, ui };
    }
  };
  const typeOf = (node: Json): string | null => {
    const declared = Array.isArray(node.type) ? node.type.find((name) => name !== "null") : node.type;
    if (typeof declared === "string") return declared;
    if (node.properties !== undefined) return "object";
    const validate = ajv.compile({ anyOf: node.anyOf ?? node.oneOf ?? [false] });
    return validate(0.5) && validate({ bits: "4034000000000000" }) ? "number" : null;
  };
  const push = (pointer: string, key: string, type: string | null, ui: Json, shown: boolean): void => void found.push([pointer, { key, type, labelled: shown && ui.widget !== "hidden", glossary: glossary.has(key), ui, control: entry.controls?.[pointer] ?? null }]);
  const visit = (object: Json, base: string): void => {
    for (const [key, raw] of Object.entries((object.properties ?? {}) as Record<string, Json>)) {
      const { node, ui } = settle(raw);
      if (node.const !== undefined || ui.role === "discriminator") continue;
      const type = typeOf(node);
      push(`${base}/${key}`, key, type, ui, true);
      if (ui.widget === "hidden" || ui.ref !== undefined || ui.widget === "reference" || ui.role === "target") continue;
      if (type === "object") visit(node, `${base}/${key}`);
      if (type !== "array" || node.items === undefined) continue;
      const item = settle(node.items as Json);
      const itemType = typeOf(item.node);
      const numeric = itemType === "number" || itemType === "integer";
      if (ui.widget === "vector" || ui.widget === "color" || (numeric && node.minItems === node.maxItems && [2, 3, 4].includes(node.minItems as number))) continue;
      const merged = numeric ? { ...Object.fromEntries(Object.entries(ui).filter(([name]) => ITEM_FACETS.includes(name))), ...item.ui } : item.ui;
      if (numeric || merged.widget !== undefined) push(`${base}/${key}/-`, "-", itemType, merged, false);
      if (itemType === "object") visit(item.node, `${base}/${key}/-`);
    }
  };
  const root = settle(entry.input.schema).node;
  if (root.properties !== undefined) visit(root, "");
  else {
    const variants = ((root.oneOf ?? root.anyOf ?? []) as Json[]).map((branch) => settle(branch).node);
    const pinned = (key: string): unknown[] => variants.map((variant) => (variant.properties as Record<string, Json> | undefined)?.[key]?.const);
    const selector = Object.keys((variants[0]?.properties ?? {}) as Json).find((key) => new Set(pinned(key)).size === variants.length && pinned(key).every((value) => typeof value === "string"));
    if (selector !== undefined) push(`/${selector}`, selector, "string", settle((variants[0]!.properties as Record<string, Json>)[selector]!).ui, true);
    for (const variant of variants) visit(variant, "");
  }
  return found;
}

/** ⚖️ The oracle's findings: none for a withdraw-only leaf; else `inputless` at the root when the leaf fails `leafShowsInput`, then
 * every rule schema of the gate schema an input fails, as `[code, pointer]` in reading order, each once; the multi-line rule is
 * judged only where the case states the reader's controls. */
function oracleFindings(ajv: Ajv, entry: Case): Finding[] {
  const found = new Map<string, Finding>();
  const inputs = oracleInputs(ajv, entry);
  const leaf = { editable: entry.editable ?? true, shown: inputs.filter(([pointer, input]) => pointer.lastIndexOf("/") === 0 && input.labelled).length };
  if (!ajv.getSchema(`${schema.$id}#/definitions/resolvedLeaf`)!(leaf)) throw new Error(`the oracle resolved no leaf: ${JSON.stringify(leaf)}`);
  if (!leaf.editable) return [];
  if (!ajv.getSchema(`${schema.$id}#/definitions/leafShowsInput`)!(leaf)) found.set("inputless", ["inputless", ""]);
  for (const [pointer, input] of inputs) {
    for (const [rule, code] of RULES) {
      if (rule === "multilineControlled" && entry.controls === undefined) continue;
      if (!ajv.getSchema(`${schema.$id}#/definitions/${rule}`)!(input)) found.set(`${code}\n${pointer}`, [code, pointer]);
    }
  }
  return [...found.values()];
}

describe("🎛️ the schema-mutation-input-ui declaration rules", () => {
  const ajv = oracleAjv();
  const widgets = mutationInputWidgetVocabulary(repoRoot, (id) => documents.get(id));

  test("the examples plant every rule", () => {
    expect(new Set(fixture.cases.flatMap((entry) => entry.findings.map(([code]) => code)))).toEqual(new Set<MutationInputDeclarationCode>([...RULES.map(([, code]) => code), "inputless"]));
    expect(fixture.cases.some((entry) => entry.findings.length === 0 && entry.editable !== false)).toBe(true);
    expect(fixture.cases.some((entry) => entry.editable === false)).toBe(true);
  });

  test("the widget vocabulary is the one the strict vocabulary registers for x-semio-ui", () => {
    const registered = (ajv.getSchema(`${manifest.$id}#/$defs/InputUi/properties/widget`)!.schema as { readonly enum: readonly string[] }).enum;
    expect([...widgets]).toEqual([...registered]);
    expect(widgets.has("dictionary")).toBe(false);
    expect(() => mutationInputWidgetVocabulary(repoRoot, () => undefined)).toThrow("states no x-semio-ui widget vocabulary");
  });

  for (const entry of fixture.cases) {
    const controls = entry.controls === undefined ? null : new Map(Object.entries(entry.controls));
    test(`${entry.id}: the gate reads the listed declarations and findings`, () => {
      const rows = mutationInputDeclarations(entry.input.schema, (id) => documents.get(id));
      expect(rows).toEqual(entry.declarations.map((row) => ({ ...row })));
      expect(mutationInputDeclarationFindings(rows, widgets, controls, entry.editable ?? true).map(({ code, pointer }) => [code, pointer])).toEqual(entry.findings.map((finding) => [...finding]));
    });
    test(`${entry.id}: the Ajv oracle resolves the same inputs and fails the same rules`, () => {
      const inputs = oracleInputs(ajv, entry);
      const resolved = ajv.getSchema(`${schema.$id}#/definitions/resolvedInput`)!;
      for (const [pointer, input] of inputs) expect(resolved(input), `${pointer}: ${JSON.stringify(resolved.errors)}`).toBe(true);
      expect(inputs.map(([pointer, input]) => [pointer, input.type, input.ui.widget ?? null, input.ui.role ?? null])).toEqual(entry.declarations.map((row) => [row.pointer, row.type, row.widget, row.role]));
      expect(oracleFindings(ajv, entry)).toEqual(entry.findings.map((finding) => [...finding] as unknown as Finding));
    });
    test(`${entry.id}: the gate walks the top-level inputs the manifest reader reads`, () => {
      const audit = mutationInputAudit(entry.input.schema, (id) => documents.get(id));
      expect(new Set(entry.declarations.filter((row) => row.pointer.lastIndexOf("/") === 0).map((row) => row.pointer))).toEqual(new Set(audit.inputs.map((input) => input.id)));
    });
  }

  test("a label the glossary supplies is counted as inferred and never refused; the reader refuses every label the gate finds absent", () => {
    const inferred: string[] = [];
    for (const entry of fixture.cases) {
      const rows = mutationInputDeclarations(entry.input.schema, (id) => documents.get(id));
      const glossed = rows.filter((row) => Object.values(row.label ?? {}).includes("inferred")).map((row) => row.pointer);
      const absent = mutationInputDeclarationFindings(rows, widgets, null).filter((finding) => finding.code === "labelAbsent").map((finding) => finding.pointer);
      const refused = mutationInputAudit(entry.input.schema, (id) => documents.get(id)).findings.filter((finding) => finding.code === "labelMissing" || finding.code === "localeMissing").map((finding) => finding.pointer);
      expect(glossed.filter((pointer) => absent.includes(pointer))).toEqual([]);
      expect(absent.filter((pointer) => !refused.includes(pointer)), entry.id).toEqual([]);
      inferred.push(...glossed);
    }
    expect(inferred).toEqual(["/name", "/position", "/zone", "/zone/id", "/version", "/kind", "/snapshot", "/snapshot/width"]);
  });

  test("without the reader's controls the multi-line rule is silent, and it is armed exactly when the reader hands multiline its control", () => {
    const entry = fixture.cases.find((candidate) => candidate.controls !== undefined)!;
    const rows = mutationInputDeclarations(entry.input.schema, (id) => documents.get(id));
    expect(mutationInputDeclarationFindings(rows, widgets, null)).toEqual([]);
    const withdrawn = fixture.cases.find((candidate) => candidate.editable === false)!;
    const judged = mutationInputDeclarationFindings(mutationInputDeclarations(withdrawn.input.schema, (id) => documents.get(id)), widgets, null, true).map(({ code, pointer }) => [code, pointer]);
    expect(judged).toEqual([["numericUndeclared", "/snapshot/width"], ["labelAbsent", "/snapshot/plantedKey"]]);
    const controls = mutationInputControls(mutationInputAudit(entry.input.schema, (id) => documents.get(id)).inputs);
    expect([...controls.keys()]).toEqual(Object.keys(entry.controls!));
    expect(mutationInputMultilineArmed()).toBe(controls.get("/notes") === "multiline");
    const nested = mutationInputControls(mutationInputAudit(fixture.cases[0]!.input.schema, (id) => documents.get(id)).inputs);
    expect(nested.get("/frame/width")).toBe("number");
    expect(nested.has("/weights/-")).toBe(false);
  });
});
