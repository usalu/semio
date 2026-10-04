import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import Ajv from "ajv";
import { validateJsonSchemaSubset } from "../../../🧬️schema/✅️validator/🟦️.ts";

type PublicationObservation = { outcome: string; attempts: number; progressEvents: number };
type PublicationCase = {
  id: string;
  limits: { maxAttempts: number; maxTokenNodes: number };
  tokenChain: { depth: number; baseline: "live"; overrides: { index: number; state: "parked" | "cancelled" }[] };
  actions: string[];
  expected: { construction: "ready" | "invalid-limits"; results: PublicationObservation[] };
};
type PublicationCorpus = { version: 1; tokenNodeCeiling: 4096; cases: PublicationCase[] };

const schema = JSON.parse(readFileSync(resolve(import.meta.dir, "../🧬️schema/🔣️.json"), "utf8"));
const corpus: PublicationCorpus = JSON.parse(readFileSync(resolve(import.meta.dir, "../🧫️fixtures/🔣️.json"), "utf8"));
const independent = new Ajv({ strict: true });
const oracle = independent.compile(schema);
const limitsOracle = independent.compile(schema.$defs.Limits);
const resultOracle = independent.compile(schema.$defs.Result);

test("publication admission corpus is closed under independent schema validation", () => {
  expect(validateJsonSchemaSubset(schema, corpus)).toEqual([]);
  expect(oracle(corpus)).toBe(true);
  expect(corpus.cases).toHaveLength(30);
  expect(new Set(corpus.cases.map((row: { id: string }) => row.id)).size).toBe(30);
  for (const changed of [
    { ...corpus, owner: "os" },
    { ...corpus, tokenNodeCeiling: 4097 },
    { ...corpus, cases: [] },
    { ...corpus, cases: [{ ...corpus.cases[0], defaultControl: true }] },
    { ...corpus, cases: [{ ...corpus.cases[0], actions: ["blocking-begin"] }] },
    { ...corpus, cases: [{ ...corpus.cases[0], limits: { maxAttempts: 1 } }] },
  ]) {
    expect(validateJsonSchemaSubset(schema, changed).length).toBeGreaterThan(0);
    expect(oracle(changed)).toBe(false);
  }
});

test("mandatory finite limits determine every construction result", () => {
  for (const row of corpus.cases) {
    const admitted = row.expected.construction === "ready";
    expect(validateJsonSchemaSubset(schema.$defs.Limits, row.limits).length === 0, row.id).toBe(admitted);
    expect(limitsOracle(row.limits), row.id).toBe(admitted);
    if (!admitted) {
      expect(row.actions, row.id).toEqual([]);
      expect(row.expected.results, row.id).toEqual([]);
    }
  }
  for (const changed of [
    {},
    { maxAttempts: 1 },
    { maxAttempts: 0, maxTokenNodes: 1 },
    { maxAttempts: 1, maxTokenNodes: 0 },
    { maxAttempts: 1, maxTokenNodes: 4097 },
    { maxAttempts: 4294967296, maxTokenNodes: 1 },
    { maxAttempts: 1.5, maxTokenNodes: 1 },
    { maxAttempts: 1, maxTokenNodes: 1, unlimited: false },
  ]) {
    expect(validateJsonSchemaSubset(schema.$defs.Limits, changed).length).toBeGreaterThan(0);
    expect(limitsOracle(changed)).toBe(false);
  }
});

test("token ancestry and cumulative progress are explicit finite fixture authority", () => {
  for (const row of corpus.cases) {
    const indices = row.tokenChain.overrides.map((entry: { index: number }) => entry.index);
    expect(new Set(indices).size, row.id).toBe(indices.length);
    expect(indices.every((index: number) => index < row.tokenChain.depth), row.id).toBe(true);
    expect(row.expected.results.length, row.id).toBe(row.actions.filter((action: string) => action === "try").length);
    let previous = 0;
    for (const result of row.expected.results) {
      expect(result.attempts, row.id).toBeGreaterThanOrEqual(previous);
      expect(result.attempts, row.id).toBeLessThanOrEqual(row.limits.maxAttempts);
      expect(result.progressEvents, row.id).toBe(result.attempts);
      previous = result.attempts;
    }
  }
});

test("serialized observations agree with the independent result oracle", () => {
  for (const row of corpus.cases) for (const result of row.expected.results) {
    const serialized = JSON.parse(JSON.stringify(result));
    expect(validateJsonSchemaSubset(schema.$defs.Result, serialized)).toEqual([]);
    expect(resultOracle(serialized)).toBe(true);
    const changed = { ...serialized, outcome: "implicit-retry" };
    expect(validateJsonSchemaSubset(schema.$defs.Result, changed).length).toBeGreaterThan(0);
    expect(resultOracle(changed)).toBe(false);
  }
});
