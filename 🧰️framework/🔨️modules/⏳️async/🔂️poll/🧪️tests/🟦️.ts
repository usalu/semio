/** 🔂️ Preserves immediate one-poll completion under an independent synchronous observer. */
import { test, expect } from "bun:test";
import { existsSync, readFileSync } from "node:fs";
import { resolve } from "node:path";
import Ajv from "ajv/dist/2020.js";
import { NEVER, of } from "rxjs";

type Scalar = number | string | boolean | null;
type Row = Readonly<{id: string; state: "Ready" | "Pending"; value?: Scalar; expected: Readonly<{state: "Ready" | "Pending"; value?: Scalar; polls: number}>}>;
const owner = resolve(import.meta.dir, ".."), read = (path: string): string => readFileSync(resolve(owner, path), "utf8");
const fixture = JSON.parse(read("🧫️fixtures/🔣️.json")) as {cases: Row[]};

test("every language-neutral readiness row matches an independent synchronous observer", () => {
  const validate = new Ajv({strict: true}).compile(JSON.parse(read("🧬️schema/🔣️.json")));
  expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  for (const row of fixture.cases) {
    let observed: {state: "Ready" | "Pending"; value?: Scalar; polls: number} = {state: "Pending", polls: 1};
    const subscription = (row.state === "Ready" ? of(row.value!) : NEVER).subscribe(value => { observed = {state: "Ready", value, polls: 1}; });
    subscription.unsubscribe();
    expect(observed, row.id).toEqual(row.expected);
  }
  expect(validate({...fixture, implicitBlocking: true})).toBe(false);
});

test("the actual neutral implementations poll once and refuse pending completion", async () => {
  expect(existsSync(resolve(owner, "🟦️.ts")), "actual neutral TypeScript poll owner").toBe(true);
  expect(existsSync(resolve(owner, "🦀️.rs")), "actual neutral Rust poll owner").toBe(true);
  const subject = await import(resolve(owner, "🟦️.ts")) as {resolveReady<T>(poll: () => {ready: true; value: T} | {ready: false}): T};
  for (const row of fixture.cases) {
    let polls = 0;
    const poll = () => { polls++; return row.state === "Ready" ? {ready: true as const, value: row.value!} : {ready: false as const}; };
    if (row.state === "Ready") expect(subject.resolveReady(poll)).toEqual(row.value!);
    else expect(() => subject.resolveReady(poll)).toThrow();
    expect(polls).toBe(row.expected.polls);
  }
});
