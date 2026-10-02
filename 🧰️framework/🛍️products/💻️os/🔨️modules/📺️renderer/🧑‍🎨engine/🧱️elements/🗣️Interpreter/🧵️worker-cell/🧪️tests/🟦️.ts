import { test, expect } from "bun:test";
import { readFileSync, mkdirSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { createRequire } from "node:module";
import Ajv from "ajv";
import { createAdmittedWorkerCell, createWorkerCell } from "../🟦️.ts";
const require = createRequire(import.meta.url);
const memoize: (factory: (owner: string) => State) => ((owner: string) => State) = require("lodash/memoize");
type State = { locale: string; retained: string[]; constructions: number };
const fixture = JSON.parse(readFileSync(resolve(import.meta.dir, "../🧫️fixtures/🔣️.json"), "utf8"));
const root = resolve(import.meta.dir, "../../../../../../../../../../../../..");
const interpreter = resolve(import.meta.dir, "../../🎯️targets/🧊️wgpu/🦀️.rs");

test("closed independent lifecycle corpus retains exact admission and thread identities", () => {
  const validate = new Ajv({ strict: true }).compile(JSON.parse(readFileSync(resolve(import.meta.dir, "../🧬️schema/🔣️.json"), "utf8")));
  expect(validate(fixture)).toBe(true);
  expect(validate({ ...fixture, schemaVersion: 2 })).toBe(false);
  const actual = [];
  for (const row of fixture.cases) {
    const cell = createAdmittedWorkerCell<State>();
    const ready = new Set<string>();
    const reference = memoize(() => ({ locale: "", retained: [], constructions: 1 }));
    const observations = [], oracle = [];
    for (const operation of row.operations) {
      if (operation.action === "admit") {
        cell.admit(operation.owner, () => ({ locale: operation.locale, retained: [], constructions: 1 }), state => { state.locale = operation.locale; });
        ready.add(operation.owner); reference(operation.owner).locale = operation.locale;
      } else if (operation.action === "retain") {
        cell.read(operation.owner)!.retained.push(operation.value); reference(operation.owner).retained.push(operation.value);
      }
      const state = cell.read(operation.owner), expected = ready.has(operation.owner) ? reference(operation.owner) : undefined;
      observations.push(state ? { owner: operation.owner, state: "ready", ...structuredClone(state) } : { owner: operation.owner, state: "unadmitted" });
      oracle.push(expected ? { owner: operation.owner, state: "ready", ...structuredClone(expected) } : { owner: operation.owner, state: "unadmitted" });
    }
    expect(observations).toEqual(row.expected); expect(oracle).toEqual(row.expected); actual.push({ id: row.id, observations });
  }
  const output = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!output) throw new Error("Caller-owned SEMIO_TEST_ARTIFACT_DIR is required");
  mkdirSync(output, { recursive: true }); writeFileSync(resolve(output, "worker-cell-lifecycle.json"), JSON.stringify(actual, null, 2));
});

test("nonDefault construction is lazy and scoped rather than implicit", () => {
  let calls = 0; const cell = createWorkerCell(() => ({ secret: ++calls }));
  expect(calls).toBe(0); expect(cell.read("a")).toBe(cell.read("a"));
  expect(cell.read("b").secret).toBe(2); expect(cell.read("a").secret).toBe(1);
});

test("actual Rust worker source requires owned factories and explicit locale admission", () => {
  const source = readFileSync(interpreter, "utf8");
  expect(source).not.toMatch(/impl<T: Default|test_worker_cell<T: Default/);
  expect(source).toContain("initialize: fn() -> T");
  expect(source).toContain("pub(crate) fn admit_ui_locale");
  expect(source).toContain('expect("retained UI requires explicit locale admission")');
});
