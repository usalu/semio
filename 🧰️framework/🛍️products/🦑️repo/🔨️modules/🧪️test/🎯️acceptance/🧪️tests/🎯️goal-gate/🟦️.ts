//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🎯️ Goal-gate laws: the acceptance schema interpreter agrees with Ajv (the third-party JSON Schema 2020-12 oracle) on
// every fixture record, the repository goal plan validates under both, and scripted runs reproduce the fixture's gating,
// requirement, record-precedence and browser-budget outcomes.

//#endregion 🧲️Header

//#region 🔌️Adapters
import { describe, expect, test } from "bun:test";
import { mkdtempSync, readFileSync, rmSync, writeFileSync, existsSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import Ajv2020 from "ajv/dist/2020";
import {
  ACCEPTANCE_SCHEMA_REL_PATH,
  GOAL_PLAN_REL_PATH,
  acceptanceCheckResult,
  acceptanceSchemaViolations,
  readGoalPlan,
  runGoalGate,
  type AcceptanceStatus,
  type GoalGateExecutor,
  type GoalPlan,
} from "../../📋️orchestration/🟦️.ts";
//#endregion 🔌️Adapters

const repoRoot = join(import.meta.dir, "../../../../../../../..");
const fixture = JSON.parse(readFileSync(join(import.meta.dir, "..", "..", "🧫️fixtures", "🎯️goal-gate", "🔣️.json"), "utf8")) as {
  records: { name: string; definition: "checkResult" | "goalPlan" | "goalSummary"; valid: boolean; value: unknown }[];
  runs: {
    name: string;
    options: { hub: string | null; serve: string | null };
    steps: { id: string; mode: "serial" | "parallel"; gatesRest: boolean; checks: { id: string; requires: ("hub" | "serve" | "backends")[]; browsers: number }[] }[];
    exits: Record<string, number>;
    records: Record<string, AcceptanceStatus>;
    expected: { verdict: AcceptanceStatus; statuses: Record<string, AcceptanceStatus>; executed: string[] };
  }[];
};
const schema = JSON.parse(readFileSync(join(repoRoot, ACCEPTANCE_SCHEMA_REL_PATH), "utf8"));
const ajv = new Ajv2020({ allErrors: true, strict: false });
ajv.addSchema(schema);
const oracle = (definition: string) => ajv.getSchema(`${schema.$id}#/$defs/${definition}`)!;

describe("acceptance schema", () => {
  for (const record of fixture.records) {
    test(`${record.name}: interpreter and Ajv agree (${record.valid ? "valid" : "invalid"})`, () => {
      expect(acceptanceSchemaViolations(repoRoot, record.definition, record.value).length === 0).toBe(record.valid);
      expect(oracle(record.definition)(record.value)).toBe(record.valid);
    });
  }

  test("the repository goal plan validates under the interpreter and Ajv", () => {
    const plan = readGoalPlan(repoRoot);
    expect(oracle("goalPlan")(plan)).toBe(true);
    expect(plan.steps.flatMap((step) => step.checks).every((check) => check.criteria.length > 0)).toBe(true);
    expect(JSON.parse(readFileSync(join(repoRoot, GOAL_PLAN_REL_PATH), "utf8")).schema).toBe("semio.acceptance.goal-plan/v1");
  });

  test("a built check result validates under Ajv", () => {
    const result = acceptanceCheckResult({ check: "built", status: "pass", startedAt: new Date(Date.now() - 1500), measured: { rows: 3 }, summary: { en: "three rows", de: "drei Zeilen" } });
    expect(oracle("checkResult")(result)).toBe(true);
    expect(result.durationMs).toBeGreaterThanOrEqual(1000);
  });
});

describe("goal gate runs", () => {
  for (const run of fixture.runs) {
    test(run.name, async () => {
      const dir = mkdtempSync(join(tmpdir(), "semio-goal-gate-law-"));
      try {
        const plan: GoalPlan = {
          schema: "semio.acceptance.goal-plan/v1",
          id: "law-plan",
          title: { en: "Law plan", de: "Gesetzesplan" },
          steps: run.steps.map((step) => ({
            id: step.id,
            title: { en: step.id, de: step.id },
            mode: step.mode,
            gatesRest: step.gatesRest,
            checks: step.checks.map((check) => ({ id: check.id, title: { en: check.id, de: check.id }, criteria: ["1.1"], project: "workspace", target: "verify", args: [], requires: check.requires, browsers: check.browsers })),
          })),
        };
        const planPath = join(dir, "plan.json");
        writeFileSync(planPath, JSON.stringify(plan));
        const executed: string[] = [];
        let browsers = 0;
        let peakBrowsers = 0;
        const execute: GoalGateExecutor = async (_repoRoot, check, _args, env) => {
          executed.push(check.id);
          browsers += check.browsers;
          peakBrowsers = Math.max(peakBrowsers, browsers);
          await new Promise((resolveDelay) => setTimeout(resolveDelay, 20));
          const recorded = run.records[check.id];
          if (recorded) writeFileSync(env.SEMIO_ACCEPTANCE_RESULT!, JSON.stringify(acceptanceCheckResult({ check: check.id, status: recorded, startedAt: new Date(), measured: { scripted: true }, summary: { en: "scripted", de: "geskriptet" } })));
          browsers -= check.browsers;
          return run.exits[check.id] ?? 0;
        };
        const summary = await runGoalGate(repoRoot, { ...run.options, hubBinary: null, users: [], planPath, outDir: join(dir, "out"), only: [], includeOptional: false, maxBrowsers: 1, maxParallel: 4, signal: new AbortController().signal, execute });
        expect(summary.verdict).toBe(run.expected.verdict);
        expect(Object.fromEntries(summary.steps.flatMap((step) => step.checks.map((check) => [check.check, check.status])))).toEqual(run.expected.statuses);
        expect([...executed].sort()).toEqual([...run.expected.executed].sort());
        expect(peakBrowsers).toBeLessThanOrEqual(Math.max(1, ...run.steps.flatMap((step) => step.checks.map((check) => check.browsers))));
        expect(oracle("goalSummary")(JSON.parse(readFileSync(join(dir, "out", "summary.json"), "utf8")))).toBe(true);
        expect(readFileSync(join(dir, "out", "summary.de.md"), "utf8")).toContain("Ergebnis");
        expect(existsSync(join(dir, "out", "summary.en.md"))).toBe(true);
      } finally {
        rmSync(dir, { recursive: true, force: true });
      }
    });
  }
});
