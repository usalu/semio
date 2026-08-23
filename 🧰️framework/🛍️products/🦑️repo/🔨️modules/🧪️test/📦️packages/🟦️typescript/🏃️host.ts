#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🟦️ TypeScript native host. Invoked as
//   bun 🏃️host.ts --plan <plan.json> --out <results.jsonl> [--adapter <abs path>]
// It loads the case's explicit `🟦️component.ts` adapter and executes exactly the scenarios the
// coordinator planned. It never discovers anything and never parses a feature file — that is what
// keeps the five language hosts provably in agreement.

//#endregion 🧲️Header

//#region 🔌️Adapters
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, isAbsolute, join } from "node:path";
import { type AdapterOutcome, type TestAdapter, type TestCasePlan, type TestResult, digest, makeAdapterContext, projectionHash, repoRootFromHere, setDigest, testId, validateRegistration } from "./📦️index.ts";
//#endregion 🔌️Adapters

//#region 🎛️Arguments
/** 🎛️ The three things a host is ever told: the plan, where to write results, and which adapter to load. */
export type HostArgs = Readonly<{ planPath: string; outPath: string; adapterPath: string | null }>;

/** 🎛️ Parses the uniform host argument vector every language host accepts. */
export function parseHostArgs(argv: readonly string[]): HostArgs {
  const read = (flag: string): string | null => {
    const index = argv.indexOf(flag);
    return index === -1 ? null : (argv[index + 1] ?? null);
  };
  const planPath = read("--plan");
  const outPath = read("--out");
  if (planPath === null || outPath === null) throw new Error("usage: bun 🏃️host.ts --plan <plan.json> --out <results.jsonl> [--adapter <path>]");
  return { planPath, outPath, adapterPath: read("--adapter") };
}
//#endregion 🎛️Arguments

//#region 🏃️Execution
/** 🏃️ Turns one adapter outcome into the owned result record, writing binary payloads out by path. */
export function resultFor(plan: TestCasePlan, scenarioId: string, level: TestResult["level"], role: TestResult["role"], status: TestResult["status"], durationMs: number, outcome: AdapterOutcome | null, diagnostics: TestResult["diagnostics"], seed?: string): TestResult {
  const raw = outcome?.raw;
  const rawPath = raw === undefined ? undefined : join(plan.outputDir, `${scenarioId}.${role}.raw`);
  if (raw !== undefined && rawPath !== undefined) {
    mkdirSync(dirname(rawPath), { recursive: true });
    writeFileSync(rawPath, raw);
  }
  const projectionPath = outcome === null ? undefined : join(plan.outputDir, `${scenarioId}.${role}.projection.json`);
  if (outcome !== null && projectionPath !== undefined) {
    mkdirSync(dirname(projectionPath), { recursive: true });
    writeFileSync(projectionPath, `${JSON.stringify(outcome.projection, null, 2)}\n`);
  }
  return {
    testId: testId(plan.owner, plan.case, scenarioId, plan.implementation, role),
    owner: plan.owner,
    case: plan.case,
    scenario: scenarioId,
    implementation: plan.implementation,
    role,
    level,
    status,
    durationMs,
    seed,
    featureHash: plan.featureHash,
    fixtureHash: setDigest(plan.fixtures.map((fixture) => [fixture.name, fixture.digest] as const)),
    output: {
      rawHash: raw === undefined ? digest("") : digest(raw),
      projectionHash: outcome === null ? digest("") : projectionHash(plan.comparison, outcome.projection),
      rawPath,
      projectionPath,
      projection: outcome?.projection,
    },
    diagnostics,
  };
}

/** 🏃️ Executes every planned scenario against one loaded adapter, in plan order. */
export async function runAdapter(repoRoot: string, plan: TestCasePlan, adapter: TestAdapter): Promise<TestResult[]> {
  const results: TestResult[] = [];
  const registrationProblems = validateRegistration(plan, adapter, plan.role);
  for (const scenario of plan.scenarios) {
    const handler = adapter.scenarios[scenario.id]?.[plan.role];
    const started = Date.now();
    if (handler === undefined) {
      results.push(resultFor(plan, scenario.id, scenario.level, plan.role, "errored", Date.now() - started, null, [{ severity: "error", message: `adapter has no ${plan.role} registration for scenario ${scenario.id}`, detail: registrationProblems.join("\n") }], scenario.seed));
      continue;
    }
    try {
      const outcome = await handler(makeAdapterContext(repoRoot, plan, scenario, plan.role));
      results.push(resultFor(plan, scenario.id, scenario.level, plan.role, "passed", Date.now() - started, outcome, outcome.diagnostics ?? [], scenario.seed));
    } catch (error) {
      const failure = error as Error;
      const status = failure.name === "AssertionError" ? "failed" : "errored";
      results.push(resultFor(plan, scenario.id, scenario.level, plan.role, status, Date.now() - started, null, [{ severity: "error", message: failure.message, detail: failure.stack ?? "" }], scenario.seed));
    }
  }
  return results;
}
//#endregion 🏃️Execution

//#region 🚪️Entry
/** 🚪️ Host entry: load plan, load adapter, execute, emit JSONL. Any failure is a result, never a skip. */
export async function main(argv: readonly string[]): Promise<number> {
  const args = parseHostArgs(argv);
  const repoRoot = repoRootFromHere();
  const plan = JSON.parse(readFileSync(args.planPath, "utf8")) as TestCasePlan;
  const adapterRel = args.adapterPath ?? plan.adapters.typescript;
  if (adapterRel === undefined) throw new Error(`plan for ${plan.owner}::${plan.case} declares no typescript adapter`);
  const adapterAbs = isAbsolute(adapterRel) ? adapterRel : join(repoRoot, adapterRel);
  const module = (await import(adapterAbs)) as { default?: TestAdapter };
  const adapter = module.default;
  if (adapter === undefined || adapter.implementation !== "typescript") throw new Error(`${adapterRel} must default-export defineTestAdapter({ implementation: "typescript", … })`);
  const results = await runAdapter(repoRoot, plan, adapter);
  mkdirSync(dirname(args.outPath), { recursive: true });
  writeFileSync(args.outPath, results.map((result) => JSON.stringify(result)).join("\n") + (results.length > 0 ? "\n" : ""));
  return results.every((result) => result.status === "passed") ? 0 : 1;
}

if (import.meta.main) {
  process.exit(await main(process.argv.slice(2)));
}
//#endregion 🚪️Entry
