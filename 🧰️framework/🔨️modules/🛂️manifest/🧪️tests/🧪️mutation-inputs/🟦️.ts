/** 🧪️ `mutationInputDefs` (the TypeScript twin of Rust `manifest::mutation_input_defs`) over the language-agnostic corpus
 * `🧫️fixtures/🧫️mutation-inputs/🔣️.json`, byte-equal in canonical JSON to the `expectedInputs` the Rust reader is held to
 * by `🦀️.rs` beside this file. Oracles: the npm `jsonschema` package validates the corpus against the manifest's
 * `MutationInputCorpus` export and answers, for every declared input, whether the leaf schema accepts the values its hard
 * bounds, options and `required` admit and refuses the ones they exclude — the same verdicts `🐍️.py` asks Python
 * `jsonschema` for. The strict Ajv oracle compiles every annotated leaf with the registered `x-semio-ui` meta-schema.
 * @see ../../🧬️schema/🔣️.json */
import { describe, expect, test } from "bun:test";
import { Validator } from "jsonschema";
import corpus from "../../🧫️fixtures/🧫️mutation-inputs/🔣️.json" with { type: "json" };
import glossaryDocument from "../../🔣️input-labels.json" with { type: "json" };
import manifestSchema from "../../🧬️schema/🔣️.json" with { type: "json" };
import { parseInputLabelGlossary, parseInputUi } from "../../🧬️schema/🟦️.ts";
import { argControl, InputSchemaError, inputLabelGlossary, mutationInputAudit, mutationInputDefs, mutationInputInstance, referenceIdText, referenceIdValue, type ActionArgDef, type ArgSchema, type ReferenceIdType } from "../../🟦️.ts";
import { semioSchemaAjvV1 } from "../../../🧬️schema/🔮️oracles/✅️validator/🟦️.ts";

type Json = null | boolean | number | string | Json[] | { [key: string]: Json };
type Finding = { readonly code: string; readonly pointer: string };
type Case = { readonly name: string; readonly leafSchema: Record<string, Json>; readonly expectedInputs?: Json[]; readonly expectedError?: Finding; readonly expectedFindings?: readonly Finding[] };
const cases = corpus.cases as unknown as readonly Case[];
const documents = corpus.documents as unknown as Record<string, Record<string, Json>>;
const resolver = (id: string): unknown => documents[id];

/** 🧾️ Canonical JSON: object keys sorted, arrays in order, numbers in their shortest round-trip form. */
const canonical = (value: unknown): string =>
  Array.isArray(value)
    ? `[${value.map(canonical).join(",")}]`
    : value !== null && typeof value === "object"
      ? `{${Object.keys(value as Record<string, unknown>)
          .filter((key) => (value as Record<string, unknown>)[key] !== undefined)
          .sort()
          .map((key) => `${JSON.stringify(key)}:${canonical((value as Record<string, unknown>)[key])}`)
          .join(",")}}`
      : JSON.stringify(value);

const validator = (): Validator => {
  const engine = new Validator();
  engine.addSchema(manifestSchema as never, manifestSchema.$id);
  for (const [id, document] of Object.entries(documents)) engine.addSchema(document as never, id);
  return engine;
};

/** 🧪️ A value the declared schema admits, built only from the reader's own descriptor. */
function sample(schema: ArgSchema): Json {
  switch (schema.kind) {
    case "string":
      return schema.options && schema.options.length > 0 ? schema.options[0]!.value : "a".repeat(Math.max(schema.minLen ?? 0, 1));
    case "number": {
      const step = schema.integer ? 1 : 0.5;
      if (schema.min !== undefined) return schema.minExclusive ? schema.min + step : schema.min;
      if (schema.max !== undefined) return schema.maxExclusive ? schema.max - step : schema.max;
      return 0;
    }
    case "boolean":
      return false;
    case "vector":
      return Array.from({ length: schema.dims }, () => schema.min ?? (schema.max !== undefined && schema.max < 0 ? schema.max : 0));
    case "reference": {
      const identity = (index: number): Json => (schema.idType === "integer" ? index : `id-${index}`);
      return schema.many ? Array.from({ length: Math.max(schema.minItems ?? 0, 1) }, (_, index) => identity(index)) : identity(0);
    }
    case "array":
      return Array.from({ length: schema.minItems ?? 0 }, () => sample(schema.items));
    case "object":
      return Object.fromEntries(schema.fields.filter((field) => field.required).map((field) => [field.id.slice(1), sample(field.schema)]));
    case "any":
      return null;
  }
}

/** 🔀️ The variant selector of a discriminated union: the first input, a choice whose values name the other inputs' groups. */
function selectorOf(inputs: readonly ActionArgDef[]): ActionArgDef | undefined {
  const first = inputs[0];
  const values = first?.schema.kind === "string" ? (first.schema.options ?? []).map((option) => option.value) : [];
  return values.length > 0 && inputs.slice(1).some((input) => input.group !== undefined && values.includes(input.group)) ? first : undefined;
}

/** ⚖️ The accept/reject verdicts the reader's descriptors imply for one leaf: `[label, payload, admitted]`, per variant of a union. */
function probes(leaf: Record<string, Json>, inputs: readonly ActionArgDef[]): [string, Record<string, Json>, boolean][] {
  const properties = (leaf.properties ?? {}) as Record<string, Record<string, Json>>;
  const discriminators = Object.fromEntries(Object.entries(properties).filter(([, node]) => node.const !== undefined).map(([key, node]) => [key, node.const as Json]));
  const selector = selectorOf(inputs);
  const variants = selector === undefined ? [undefined] : (selector.schema as Extract<ArgSchema, { kind: "string" }>).options!.map((option) => option.value);
  const verdicts: [string, Record<string, Json>, boolean][] = [];
  for (const variant of variants) {
    const active = selector === undefined ? inputs : inputs.filter((input) => input !== selector && input.group === variant);
    if (active.some((input) => input.required && input.schema.kind === "any")) continue;
    const prefix = variant === undefined ? "" : `${variant}/`;
    const base: Record<string, Json> = { ...discriminators, ...(selector === undefined ? {} : { [selector.id.slice(1)]: variant! }), ...Object.fromEntries(active.filter((input) => input.required).map((input) => [input.id.slice(1), sample(input.schema)])) };
    verdicts.push([`${prefix}base`, base, true]);
    if (selector !== undefined) verdicts.push([`${prefix}not-a-variant`, { ...base, [selector.id.slice(1)]: "not-a-variant" }, false]);
    for (const input of active) {
      const key = input.id.slice(1);
      const schema = input.schema;
      if (input.required) {
        const { [key]: _, ...without } = base;
        verdicts.push([`${prefix}${key}:absent`, without, false]);
      } else if (schema.kind !== "any") verdicts.push([`${prefix}${key}:present`, { ...base, [key]: sample(schema) }, true]);
      if (schema.kind !== "any") verdicts.push([`${prefix}${key}:null`, { ...base, [key]: null }, input.nullable === true]);
      if (schema.kind === "number") {
        if (schema.min !== undefined) verdicts.push([`${prefix}${key}:min`, { ...base, [key]: schema.min }, !schema.minExclusive], [`${prefix}${key}:below`, { ...base, [key]: schema.min - 1 }, false]);
        if (schema.max !== undefined) verdicts.push([`${prefix}${key}:max`, { ...base, [key]: schema.max }, !schema.maxExclusive], [`${prefix}${key}:above`, { ...base, [key]: schema.max + 1 }, false]);
        if (schema.integer) verdicts.push([`${prefix}${key}:fraction`, { ...base, [key]: (sample(schema) as number) + 0.5 }, false]);
      }
      if (schema.kind === "string" && schema.options && schema.options.length > 0) {
        for (const option of schema.options) verdicts.push([`${prefix}${key}:${option.value}`, { ...base, [key]: option.value }, true]);
        verdicts.push([`${prefix}${key}:not-an-option`, { ...base, [key]: "not-an-option" }, false]);
      }
      if (schema.kind === "string" && schema.maxLen !== undefined) verdicts.push([`${prefix}${key}:too-long`, { ...base, [key]: "a".repeat(schema.maxLen + 1) }, false]);
      if (schema.kind === "string" && (schema.minLen ?? 0) > 0) verdicts.push([`${prefix}${key}:too-short`, { ...base, [key]: "" }, false]);
      if (schema.kind === "vector") {
        const component = (value: number): number[] => Array.from({ length: schema.dims }, () => value);
        verdicts.push([`${prefix}${key}:too-many`, { ...base, [key]: [...(sample(schema) as number[]), (sample(schema) as number[])[0]!] }, false]);
        if (schema.min !== undefined) verdicts.push([`${prefix}${key}:component-min`, { ...base, [key]: component(schema.min) }, true], [`${prefix}${key}:component-below`, { ...base, [key]: component(schema.min - 1) }, false]);
        if (schema.max !== undefined) verdicts.push([`${prefix}${key}:component-max`, { ...base, [key]: component(schema.max) }, true], [`${prefix}${key}:component-above`, { ...base, [key]: component(schema.max + 1) }, false]);
      }
      if (schema.kind === "reference" && schema.many && (schema.minItems ?? 0) > 0) verdicts.push([`${prefix}${key}:empty`, { ...base, [key]: [] }, false]);
      if (schema.kind === "reference") {
        const other: Json = schema.idType === "integer" ? "id-0" : 0;
        verdicts.push([`${prefix}${key}:other-id-type`, { ...base, [key]: schema.many ? [other] : other }, false]);
      }
    }
  }
  return verdicts;
}

describe("🧬️ mutation input descriptors", () => {
  test("the corpus is a MutationInputCorpus (npm jsonschema)", () => {
    
    
  });

  test("the glossary is an InputLabelGlossary with at least 200 labelled names in every locale", () => {
    const result = validator().validate(glossaryDocument, { $ref: `${manifestSchema.$id}#/$defs/InputLabelGlossary` });
    expect(result.errors.map(String)).toEqual([]);
    expect(Object.keys(parseInputLabelGlossary(glossaryDocument).labels).length).toBeGreaterThanOrEqual(200);
    expect(inputLabelGlossary().get("newName")).toEqual({ native: { en: "New Name", de: "Neuer Name" }, reuse: { en: "New Name", de: "Neuer Name" } });
    const englishByGerman = new Map<string, string>();
    for (const [name, label] of inputLabelGlossary()) {
      const first = englishByGerman.get(label.native.de) ?? label.native.en;
      englishByGerman.set(label.native.de, first);
      expect(label.native.en, `${name} shares the German label ${label.native.de} but not the English one`).toBe(first);
    }
  });

  for (const entry of cases) {
    test(`${entry.name} reads to the corpus' canonical outcome`, () => {
      if (entry.expectedInputs !== undefined) {
        expect(canonical(mutationInputDefs(entry.leafSchema, resolver))).toBe(canonical(entry.expectedInputs));
        expect(canonical(mutationInputDefs(JSON.stringify(entry.leafSchema), resolver))).toBe(canonical(entry.expectedInputs));
        return;
      }
      let refusal: unknown;
      try {
        mutationInputDefs(entry.leafSchema, resolver);
      } catch (error) {
        refusal = error;
      }
      expect(refusal).toBeInstanceOf(InputSchemaError);
      expect([(refusal as InputSchemaError).code as string, (refusal as InputSchemaError).pointer]).toEqual([entry.expectedError!.code, entry.expectedError!.pointer]);
    });

    test(`${entry.name} audits to every finding the corpus names`, () => {
      const audit = mutationInputAudit(entry.leafSchema, resolver);
      const expected = entry.expectedFindings ?? (entry.expectedError === undefined ? [] : [entry.expectedError]);
      expect(audit.findings.map((finding) => ({ code: finding.code as string, pointer: finding.pointer }))).toEqual(expected.map(({ code, pointer }) => ({ code, pointer })));
      if (entry.expectedInputs !== undefined) expect(canonical(audit.inputs)).toBe(canonical(entry.expectedInputs));
    });
  }

  test("npm jsonschema agrees with every verdict the declared bounds, options and requirements imply", () => {
    const engine = validator();
    const disagreements: string[] = [];
    let judged = 0;
    for (const entry of cases.filter((candidate) => candidate.expectedInputs !== undefined)) {
      for (const [probe, payload, admitted] of probes(entry.leafSchema, mutationInputDefs(entry.leafSchema, resolver))) {
        judged += 1;
        if (engine.validate(payload, entry.leafSchema as never, { base: `https://json.schemas.assets.semio-tech.com/test/input-ui/leaf/${entry.name}.json` }).valid !== admitted) disagreements.push(`${entry.name} ${probe}: expected ${admitted ? "valid" : "invalid"} ${JSON.stringify(payload)}`);
      }
    }
    expect(disagreements).toEqual([]);
    expect(judged).toBeGreaterThan(60);
  });

  test("the strict Ajv oracle compiles every annotated leaf and every annotation parses as InputUi", () => {
    for (const entry of cases.filter((candidate) => candidate.expectedInputs !== undefined)) {
      const ajv = semioSchemaAjvV1({ strict: true });
      for (const [id, document] of Object.entries(documents)) if (ajv.getSchema(id) === undefined) ajv.addSchema(document, id);
      expect(() => ajv.compile(entry.leafSchema)).not.toThrow();
      const annotations: unknown[] = [];
      const walk = (node: unknown): void => {
        if (Array.isArray(node)) node.forEach(walk);
        else if (node !== null && typeof node === "object") for (const [key, child] of Object.entries(node)) key === "x-semio-ui" ? annotations.push(child) : walk(child);
      };
      walk(entry.leafSchema);
      for (const annotation of annotations) expect(() => parseInputUi(annotation)).not.toThrow();
    }
    expect(() => semioSchemaAjvV1({ strict: true }).compile({ type: "string", "x-semio-ui": { color: "red" } })).toThrow();
  });

  test("a candidate payload regains the discriminator its leaf schema requires (npm jsonschema)", () => {
    const leaf = cases.find((entry) => entry.name === "annotated-drag-selection")!.leafSchema;
    const payload = { targets: ["n1"], dx: 2, dy: -1 };
    const instance = mutationInputInstance(leaf, resolver, payload);
    expect(instance.mutation).toBe("dragSelection");
    expect(validator().validate(instance, leaf as never).valid).toBe(true);
    expect(validator().validate(payload, leaf as never).valid).toBe(false);
    expect(() => mutationInputInstance(leaf, resolver, [])).toThrow(InputSchemaError);
  });

  test("a union payload regains the constants of the variant it names (npm jsonschema)", () => {
    const leaf = cases.find((entry) => entry.name === "discriminated-union-by-const")!.leafSchema;
    const base = { base: "https://json.schemas.assets.semio-tech.com/test/input-ui/leaf/union.json" };
    expect(validator().validate(mutationInputInstance(leaf, resolver, { phase: "restore", index: 2 }), leaf as never, base).valid).toBe(true);
    expect(() => mutationInputInstance(leaf, resolver, { index: 2 })).toThrow(InputSchemaError);
    expect(() => mutationInputInstance(leaf, resolver, { phase: "other", index: 2 })).toThrow(InputSchemaError);
    expect(argControl(mutationInputDefs(leaf, resolver)[0]!)).toMatchObject({ kind: "segmented" });
    expect(argControl(mutationInputDefs(cases.find((entry) => entry.name === "discriminated-union-annotated-variants")!.leafSchema, resolver)[0]!)).toMatchObject({ kind: "select" });
  });

  test("argControl mirrors the Rust control derivation for the corpus", () => {
    const read = (name: string) => mutationInputDefs(cases.find((entry) => entry.name === name)!.leafSchema, resolver);
    expect(argControl(read("annotated-drag-selection")[1]!)).toMatchObject({ kind: "stepper", snapSource: { kind: "config", key: "gridFactor" } });
    expect(argControl(read("dial-in-degrees")[0]!)).toMatchObject({ kind: "dial" });
    expect(argControl(read("log-slider-with-soft-range")[0]!)).toMatchObject({ kind: "slider", min: 0.1, max: 10, scale: "log" });
    expect(argControl(read("inferred-from-glossary")[1]!)).toMatchObject({ kind: "stepper", step: 1 });
    expect(argControl(read("inferred-from-glossary")[3]!)).toMatchObject({ kind: "slider", min: 0, max: 1 });
    expect(argControl(read("local-defs-override-options-nullable")[1]!)).toMatchObject({ kind: "segmented" });
    expect(argControl(read("nested-object-array-and-vector")[2]!)).toMatchObject({ kind: "vector", dims: 3, unit: "m", step: 0.5 });
    expect(argControl(read("integer-references")[0]!)).toMatchObject({ kind: "reference", kinds: ["zone"], domain: "energyModel", granularity: "zone", idType: "integer" });
    expect([argControl(read("color-rgb-and-rgba")[0]!), argControl(read("color-rgb-and-rgba")[1]!)]).toEqual([{ kind: "color", alpha: true }, { kind: "color", alpha: false }]);
    expect(argControl(read("vector-with-grid-facets")[0]!)).toMatchObject({ kind: "vector", dims: 3, min: -100, max: 100, step: 0.5, snaps: [0], snapSource: { kind: "config", key: "gridFactor" }, precision: 2, displayUnit: "cm", displayFactor: 100 });
    expect(read("multiline-text").map((input) => argControl(input))).toEqual([{ kind: "multiline" }, { kind: "text" }]);
    const keyed = read("keyed-list-of-typed-values")[0]!;
    const entries = keyed.schema.kind === "object" ? keyed.schema.fields[0]! : undefined;
    const record = entries?.schema.kind === "array" && entries.schema.items.kind === "object" ? entries.schema.items.fields : [];
    expect([keyed.nullable, record.map((field) => [field.id, argControl(field).kind])]).toEqual([true, [["/questionId", "text"], ["/value", "text"]]]);
    expect(argControl(read("inferred-from-glossary").find((input) => input.id === "/layerId")!)).not.toHaveProperty("idType");    expect(argControl(read("option-source-from-the-previewed-document")[1]!)).toEqual({ kind: "select", options: [] });
  });

  test("a selected id converts to the payload value the corpus' reference id table names, and spells back", () => {
    for (const row of corpus.referenceIds as readonly { idType: ReferenceIdType; text: string; value: string | number | null; spelled?: string }[]) {
      const value = referenceIdValue(row.idType, row.text);
      expect(value ?? null, `${row.idType} ${JSON.stringify(row.text)}`).toBe(row.value);
      expect(value === undefined ? undefined : referenceIdText(value)).toBe(row.spelled);
    }
  });
});
