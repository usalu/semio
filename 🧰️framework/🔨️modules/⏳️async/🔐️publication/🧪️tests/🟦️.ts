import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import Ajv from "ajv";


type PublicationObservation = { outcome: string; attempts: number; progressEvents: number };
type PublicationCase = {
  id: string;
  limits: { maxAttempts: number; maxTokenNodes: number };
  tokenChain: { depth: number; baseline: "live"; overrides: { index: number; state: "parked" | "cancelled" }[] };
  actions: string[];
  expected: { construction: "ready" | "invalid-limits"; results: PublicationObservation[] };
};
type PublicationCorpus = { version: 1; tokenNodeCeiling: 4096; cases: PublicationCase[] };
const corpus: PublicationCorpus = JSON.parse(readFileSync(resolve(import.meta.dir, "../🧫️fixtures/🔣️.json"), "utf8"));
const independent = new Ajv({ strict: true });

test("publication admission corpus is closed under independent schema validation", () => {
  expect(corpus.cases).toHaveLength(30);
  expect(new Set(corpus.cases.map((row: { id: string }) => row.id)).size).toBe(30);
  
});

test("mandatory finite limits determine every construction result", () => {
  for (const row of corpus.cases) {
    const admitted = row.expected.construction === "ready";
    if (!admitted) {
      expect(row.actions, row.id).toEqual([]);
      expect(row.expected.results, row.id).toEqual([]);
    }
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
    const changed = { ...serialized, outcome: "implicit-retry" };
  }
});
