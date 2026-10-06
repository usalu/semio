import { expect, test } from "bun:test";

import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { spawnSync } from "node:child_process";


type Observation = Readonly<{ outcomes: readonly string[]; attempts: 1; progressEvents: 1; gateHeld: true; reacquired: true }>;
type Row = Readonly<{ id: string; depth: number; maxTokenNodes: number; actions: readonly string[]; expected: Observation }>;
type Corpus = Readonly<{ version: 1; cases: readonly Row[] }>;
const fixture = resolve(import.meta.dir, "../🧫️fixtures/🔣️.json");
const corpus: Corpus = JSON.parse(readFileSync(fixture, "utf8"));

test("checkpoint observations have one closed independent schema", () => {
  expect(new Set(corpus.cases.map(row => row.id)).size).toBe(8);
  const row = corpus.cases[0];
  if (!row) throw Error("missing checkpoint corpus");
  
});

test("every checkpoint retains its admitted bounds and consumes no scheduler turn", () => {
  for (const row of corpus.cases) {
    expect(row.maxTokenNodes, row.id).toBe(row.depth);
    expect(row.expected.outcomes.length, row.id).toBe(row.actions.filter(action => action === "checkpoint").length);
    expect(row.expected.attempts, row.id).toBe(1);
    expect(row.expected.progressEvents, row.id).toBe(1);
    if (row.actions.some(action => action.endsWith("-parent"))) expect(row.depth, row.id).toBeGreaterThan(1);
  }
});

test("independent Node ancestry observations produce the exact checkpoint fixture", () => {
  const script = "const fs = require(\"node:fs\"), assert = require(\"node:assert/strict\");\nconst corpus = JSON.parse(fs.readFileSync(process.argv[1], \"utf8\"));\nconst rows = corpus.cases.map(row => {\n  const states = Array.from({ length: row.depth }, () => 0), outcomes = [];\n  for (const action of row.actions) {\n    switch (action) {\n      case \"checkpoint\": outcomes.push([\"live\", \"parked\", \"cancelled\"][Math.max(...states)]); break;\n      case \"cancel-current\": states[states.length - 1] = 2; break;\n      case \"park-current\": states[states.length - 1] = Math.max(states.at(-1), 1); break;\n      case \"unpark-current\": if (states.at(-1) === 1) states[states.length - 1] = 0; break;\n      case \"cancel-parent\": states[0] = 2; break;\n      case \"park-parent\": states[0] = Math.max(states[0], 1); break;\n      case \"unpark-parent\": if (states[0] === 1) states[0] = 0; break;\n      case \"cancel-other\": break;\n      default: throw Error(action);\n    }\n  }\n  const observation = { outcomes, attempts: 1, progressEvents: 1, gateHeld: true, reacquired: true };\n  assert.deepEqual(observation, row.expected, row.id);\n  return { id: row.id, observation };\n});\nprocess.stdout.write(JSON.stringify(rows));";
  const child = spawnSync("node", ["-e", script, fixture], { encoding: "utf8", timeout: 30_000 });
  expect(child.error).toBeUndefined();
  expect(child.status, child.stderr).toBe(0);
  expect(JSON.parse(child.stdout)).toEqual(corpus.cases.map(row => ({ id: row.id, observation: row.expected })));
  console.log("[DEBUG] Publication checkpoint independent Node cases=" + corpus.cases.length);
});
