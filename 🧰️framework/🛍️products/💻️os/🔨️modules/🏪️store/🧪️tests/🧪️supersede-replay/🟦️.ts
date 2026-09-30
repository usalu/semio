/** ✏️ Supersede-replay corpus: Ajv validates its shape, and an independent Report-mode replay of the demo family built on
 * fast-json-patch reproduces every expected state, per-mutation outcome and finalize verdict the Rust store is checked
 * against (`🏪️store/🧪️tests/🧪️supersede-replay/🦀️.rs`). */
import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import Ajv from "ajv";
import { applyPatch, type Operation } from "fast-json-patch";

//#region 🧮️DemoOracle
type Snapshot = { n: number | null };
type DemoOperation = { operation: "setN"; n: number } | { operation: "addN"; delta: number } | { operation: "deleteN" } | { operation: "restoreN"; n?: number | null };
type Level = "info" | "warning" | "error" | "fatal";
type Message = { level: Level; code: string };
type Outcome = { edit: number; op: number; worst: Level | null; codes: string[]; superseded: boolean; withdrawn: boolean };
type InvalidInput = { invalid: { schema: string; payloadHex: string } };
type Case = { name: string; initial: Snapshot; edits: DemoOperation[][]; supersessions: { edit: number; op: number; replacement: DemoOperation | InvalidInput | "withdrawn" }[]; expected: { state: Snapshot; outcomes: Outcome[]; blocksFinalize: boolean } };

const read = (path: string) => JSON.parse(readFileSync(new URL(path, import.meta.url), "utf8"));
const rank: Record<Level, number> = { info: 0, warning: 1, error: 2, fatal: 3 };
const saturate = (value: number) => Math.min(2147483647, Math.max(-2147483648, value));
const missing: Message = { level: "error", code: "mutation.target-missing" };

/** 🧮️ One demo operation as the JSON Patch it writes and the messages it raises against `state`. */
function diff(operation: DemoOperation, state: Snapshot): { patch: Operation[]; messages: Message[] } {
  switch (operation.operation) {
    case "setN":
      return state.n === null ? { patch: [], messages: [missing] } : { patch: [{ op: "replace", path: "/n", value: operation.n }], messages: [] };
    case "addN":
      return state.n === null ? { patch: [], messages: [missing] } : { patch: [{ op: "replace", path: "/n", value: saturate(state.n + operation.delta) }], messages: [{ level: "info", code: "mutation.cascade" }] };
    case "deleteN":
      return { patch: state.n === null ? [] : [{ op: "replace", path: "/n", value: null }], messages: [] };
    case "restoreN":
      return { patch: [{ op: "replace", path: "/n", value: operation.n ?? null }], messages: [] };
  }
}

/** ⏪️ Replays every recorded operation in applied order, each through its effective input, keeping and recording outcomes. */
function replay(testCase: Case): Case["expected"] {
  let state: Snapshot = structuredClone(testCase.initial);
  const replacements = new Map(testCase.supersessions.map((supersession) => [`${supersession.edit}/${supersession.op}`, supersession.replacement]));
  const outcomes: Outcome[] = [];
  testCase.edits.forEach((edit, editIndex) =>
    edit.forEach((original, opIndex) => {
      const key = `${editIndex}/${opIndex}`;
      const superseded = replacements.has(key);
      const replacement = replacements.get(key);
      if (replacement === "withdrawn") {
        outcomes.push({ edit: editIndex, op: opIndex, worst: null, codes: [], superseded, withdrawn: true });
        return;
      }
      if (replacement !== undefined && "invalid" in replacement) {
        outcomes.push({ edit: editIndex, op: opIndex, worst: "fatal", codes: ["mutation.invariant"], superseded, withdrawn: false });
        return;
      }
      const { patch, messages } = diff(replacement ?? original, state);
      state = applyPatch(state, patch, true, false).newDocument;
      const worst = messages.reduce<Level | null>((known, message) => (known === null || rank[message.level] > rank[known] ? message.level : known), null);
      outcomes.push({ edit: editIndex, op: opIndex, worst, codes: messages.map((message) => message.code), superseded, withdrawn: false });
    }),
  );
  return { state, outcomes, blocksFinalize: outcomes.some((outcome) => outcome.worst === "error" || outcome.worst === "fatal") };
}
//#endregion 🧮️DemoOracle

//#region 🧪️Corpus
const corpus = read("../../🧫️fixtures/🧫️supersede-replay/🔣️.json");
const schema = read("../../🧬️schema/🔣️supersede-replay/🔣️.json");

test("🧬️ the supersede-replay corpus satisfies its JSON Schema", () => {
  const validate = new Ajv({ strict: true, allErrors: true, allowUnionTypes: true }).compile(schema);
  expect(validate(corpus), JSON.stringify(validate.errors)).toBe(true);
});

test("🧮️ an independent fast-json-patch replay reproduces every expected state and outcome", () => {
  for (const testCase of corpus.cases as Case[]) {
    expect(replay(testCase), testCase.name).toEqual(testCase.expected);
  }
});
//#endregion 🧪️Corpus
