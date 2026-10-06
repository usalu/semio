/** 🛂️ Supersede-law corpus (design §3, §22.1): Ajv validates its shape, and an independent TS statement of the two decision
 * tables reproduces every row the Rust store is checked against (`🦀️.rs` beside this file). The input table is modelled over
 * real demo operations folded with fast-json-patch: a withdrawal folds its target as a no-op, an admitted input folds in its
 * place and a refused one folds as a fatal no-op. The unit table says which operations belong to a cross-artifact unit. Both
 * tables are total: the corpus lists every combination of their domains, and fast-check sweeps the fold over random bases. */
import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import Ajv from "ajv";
import fc from "fast-check";
import { applyPatch, type Operation as Patch } from "fast-json-patch";

//#region 🧮️Twin
type Capability = "plain" | "foreignStep";
type ReplacementKind = "withdrawn" | "recorded" | "plain" | "foreignStep" | "otherSchema" | "garbage";
type Folds = "noOp" | "replacement" | "fatalNoOp";
type InputRow = { name: string; original: Capability; replacement: ReplacementKind; admitted: boolean; folds: Folds };
type Planning = "plain" | "planner" | "foreignPlanner";
type UnitRow = { name: string; operation: Planning; origin: "owner" | "transaction"; group: boolean; unit: "none" | "group" | "operation"; supersede: "authored" | "history.unit-spans-documents" };
type Snapshot = { n: number };
type Operation = { delta: number; plansForeignSteps: boolean };
type Replacement = { kind: "withdrawn" } | { kind: "input"; schema: string; operation: Operation | null };

const read = (path: string) => JSON.parse(readFileSync(new URL(path, import.meta.url), "utf8"));
const SCHEMA = "demo/v1";
const patch = (operation: Operation, state: Snapshot): Patch[] => [{ op: "replace", path: "/n", value: state.n + operation.delta }];

/** 🛂️ The supersede law: whether `original` takes `replacement` in a document of {@link SCHEMA}. */
function admits(original: Operation, replacement: Replacement): boolean {
  if (replacement.kind === "withdrawn") return true;
  if (replacement.schema !== SCHEMA || replacement.operation === null) return false;
  if (original.plansForeignSteps) return JSON.stringify(replacement.operation) === JSON.stringify(original);
  return !replacement.operation.plansForeignSteps;
}

/** 🎬️ What a fold site folds for `original` under `replacement`: the next state and whether it carries the fatal message. */
function fold(state: Snapshot, original: Operation, replacement: Replacement | null): { state: Snapshot; folds: Folds | "original" } {
  if (replacement === null) return { state: applyPatch(structuredClone(state), patch(original, state), true, false).newDocument, folds: "original" };
  if (!admits(original, replacement)) return { state, folds: "fatalNoOp" };
  if (replacement.kind === "withdrawn") return { state, folds: "noOp" };
  return { state: applyPatch(structuredClone(state), patch(replacement.operation!, state), true, false).newDocument, folds: "replacement" };
}

/** 🧱️ The operations a corpus row names. */
function staged(row: Pick<InputRow, "original" | "replacement">): { original: Operation; replacement: Replacement } {
  const original: Operation = { delta: row.original === "plain" ? 2 : 1000, plansForeignSteps: row.original === "foreignStep" };
  switch (row.replacement) {
    case "withdrawn":
      return { original, replacement: { kind: "withdrawn" } };
    case "recorded":
      return { original, replacement: { kind: "input", schema: SCHEMA, operation: { ...original } } };
    case "plain":
      return { original, replacement: { kind: "input", schema: SCHEMA, operation: { delta: 5, plansForeignSteps: false } } };
    case "foreignStep":
      return { original, replacement: { kind: "input", schema: SCHEMA, operation: { delta: 1001, plansForeignSteps: true } } };
    case "otherSchema":
      return { original, replacement: { kind: "input", schema: "other/v1", operation: { delta: 5, plansForeignSteps: false } } };
    case "garbage":
      return { original, replacement: { kind: "input", schema: SCHEMA, operation: null } };
  }
}

/** 🧷️ The cross-artifact unit of an operation: `none` when it is its own unit. */
function unit(row: Pick<UnitRow, "operation" | "origin" | "group">): UnitRow["unit"] {
  const spans = row.origin === "transaction" || row.operation === "foreignPlanner";
  return spans ? (row.group ? "group" : "operation") : "none";
}
//#endregion 🧮️Twin

//#region 🧪️Corpus
const corpus = read("../../🧫️fixtures/🧫️supersede-law/🔣️.json") as { inputs: InputRow[]; units: UnitRow[] };
const schema = read("../../🧬️schema/🔣️supersede-law/🔣️.json");

test("🧬️ the supersede-law corpus satisfies its JSON Schema", () => {
  const validate = new Ajv({ strict: true, allErrors: true, allowUnionTypes: true }).compile(schema);
  expect(validate(corpus), JSON.stringify(validate.errors)).toBe(true);
});

test("🛂️ the twin reproduces every input row: what is admitted and what every fold site folds", () => {
  for (const row of corpus.inputs) {
    const { original, replacement } = staged(row);
    expect(admits(original, replacement), row.name).toBe(row.admitted);
    expect(fold({ n: 1 }, original, replacement).folds, row.name).toBe(row.folds);
  }
});

test("🧷️ the twin reproduces every unit row, and a supersession is refused exactly for an operation of a unit", () => {
  for (const row of corpus.units) {
    expect(unit(row), row.name).toBe(row.unit);
    expect(row.supersede, row.name).toBe(row.unit === "none" ? "authored" : "history.unit-spans-documents");
  }
});

test("📏️ both tables are total over their domains and name every row once", () => {
  const inputs = new Set(corpus.inputs.map((row) => `${row.original}/${row.replacement}`));
  for (const original of ["plain", "foreignStep"]) for (const replacement of ["withdrawn", "recorded", "plain", "foreignStep", "otherSchema", "garbage"]) expect(inputs.has(`${original}/${replacement}`), `${original}/${replacement}`).toBe(true);
  expect(inputs.size).toBe(corpus.inputs.length);
  const units = new Set(corpus.units.map((row) => `${row.operation}/${row.origin}/${row.group}`));
  for (const operation of ["plain", "planner", "foreignPlanner"]) for (const origin of ["owner", "transaction"]) for (const group of [false, true]) expect(units.has(`${operation}/${origin}/${group}`), `${operation}/${origin}/${group}`).toBe(true);
  expect(units.size).toBe(corpus.units.length);
  const names = [...corpus.inputs, ...corpus.units].map((row) => row.name);
  expect(new Set(names).size).toBe(names.length);
});
//#endregion 🧪️Corpus

//#region 🎲️Properties
test("🎲️ on any base a withdrawal and a refused input leave the state alone, the recorded input folds as the original", () => {
  fc.assert(
    fc.property(fc.integer({ min: -1000, max: 1000 }), fc.constantFrom(...corpus.inputs), (n, row) => {
      const { original, replacement } = staged(row);
      const folded = fold({ n }, original, replacement);
      if (row.folds !== "replacement") expect(folded.state).toEqual({ n });
      if (row.replacement === "recorded") expect(folded.state).toEqual(fold({ n }, original, null).state);
      expect(folded.folds).toBe(row.folds);
    }),
    { numRuns: 300 },
  );
});
//#endregion 🎲️Properties
