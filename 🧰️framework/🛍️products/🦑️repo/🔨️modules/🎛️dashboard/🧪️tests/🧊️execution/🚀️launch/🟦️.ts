import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import Ajv from "ajv";
import { EventEmitter } from "node:events";
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import ts from "typescript";
import schema from "../../../../📚️library/⚡️caching/🚀️bootstrap/🎛️dashboard/📦️installation/🧬️schema/🚀️launch/🔣️.json" with { type: "json" };
import fixture from "../../🧫️fixtures/🧊️execution/🔣️.json" with { type: "json" };

test("dashboard controls install missing modes once and retain stale installed control access", async () => {
  const oracle = new Ajv({ strict: true }).compile(schema);
  expect(oracle(fixture.launch), JSON.stringify(oracle.errors)).toBe(true);
  expect(oracle([{ ...fixture.launch[0], installed: "yes" }])).toBe(false);
  const file = new URL("../../../📦️installation/🟦️.ts", import.meta.url), source = readFileSync(file, "utf8"), tree = ts.createSourceFile(file.pathname, source, ts.ScriptTarget.Latest, true);
  const names = ["mayRunStale", "rebuildDashboard", "launchDashboard"], functions = tree.statements.filter((node): node is ts.FunctionDeclaration => ts.isFunctionDeclaration(node) && names.includes(node.name?.text ?? ""));
  expect(functions).toHaveLength(names.length);
  const compiled = ts.transpileModule(functions.map(node => node.getText(tree).replace(/^export\s+/u, "")).join("\n"), { compilerOptions: { target: ts.ScriptTarget.ES2022 }, reportDiagnostics: true });
  expect(compiled.diagnostics).toEqual([]);
  const database = new Database(":memory:");
  try { for (const row of fixture.launch) {
    const original = new AbortController(), control = Object.freeze({ signal: original.signal }), preparation = Object.freeze({ signal: AbortSignal.any([original.signal]) }), invocation = Object.freeze({ fixture: "received" }), options = Object.freeze({ mode: row.mode, control, invocation, workspace: process.cwd() });
    const events: string[] = [], failures = new Error(row.name), dashboardPackageRoot = resolve(import.meta.dir, "../../../📦️packages/🦀️rust");
    let leased = false, checks = 0, rebuilds = 0, launches = 0;
    const ports = {
      process: { execPath: process.execPath, env: { SEMIO_BUILD_MODE: "ambient-other-mode" } }, join, dashboardPackageRoot, AbortController, AbortSignal, setTimeout, clearTimeout,
      console: { log: () => {} },
      readBuildMode: (mode: string) => { expect(mode).toBe(row.mode); return mode; },
      compilerAdmissionBudgetV1: (value: unknown) => { expect<unknown>([control, preparation]).toContain(value); return value === preparation ? row.preparationBudget : row.originalBudget; },
      buildPreparationBudgetMs: () => 21600000,
      withRequestedIssuedResourceControlV1: async (value: unknown, budget: number, body: (phase: unknown) => Promise<void>) => { expect(value).toBe(control); expect(budget).toBe(21600000); events.push("phase"); try { await body(preparation); } finally { events.push("phase-settled"); } },
      scriptResourceProcessEnvironmentV1: (value: unknown, env: object) => { expect(value).toBe(preparation); return { ...env, preparation }; },
      dashboardInstalled: (root: string, mode: string) => { expect(root).toBe(process.cwd()); expect(mode).toBe(row.mode); events.push("exists"); return row.installed; },
      staleDashboard: async (root: string, mode: string) => { expect(root).toBe(process.cwd()); expect(mode).toBe(row.mode); checks++; events.push("stale"); await Promise.resolve(); return row.stale ?? undefined; },
      scriptInvocationEnvironment: (value: unknown, env: object) => { expect(value).toBe(invocation); return { ...env, received: invocation }; },
      runOwnedCommand: async (command: string, args: string[], cwd: string, label: string, budget: number, child: { signal: AbortSignal; env: Record<string, unknown> }) => {
        expect([command, args, cwd, label, budget]).toEqual([process.execPath, [join(dashboardPackageRoot, "📜️script.ts"), "build"], dashboardPackageRoot, "dashboard:rebuild", row.preparationBudget]);
        expect(child.signal).toBe(preparation.signal); expect(child.env.preparation).toBe(preparation); expect(child.env.SEMIO_SCRIPT_POLICY).toBeUndefined(); expect(child.env.received).toBe(invocation); expect(child.env.SEMIO_BUILD_MODE).toBe(row.mode);
        rebuilds++; events.push("rebuild"); await Promise.resolve();
        if (row.outcome === "rebuild-cancelled") original.abort(failures);
        if (row.outcome.startsWith("rebuild-")) throw failures;
      },
      withInstalledDashboardV1: async (root: string, context: unknown, body: (path: string) => Promise<number>) => {
        expect(root).toBe(process.cwd()); expect(context).toBe(options); expect(events.includes("phase")).toBe(events.includes("phase-settled")); expect(row.installed || rebuilds === 1).toBe(true);
        leased = true; events.push("reader"); try { return await body("selected-immutable-dashboard"); } finally { leased = false; events.push("released"); }
      },
      spawn: (path: string, args: readonly string[], child: { cwd: string; signal: AbortSignal; env: Record<string, unknown> }) => {
        expect(path).toBe("selected-immutable-dashboard"); expect(args).toEqual(row.args); expect(child.cwd).toBe(process.cwd()); expect(leased).toBe(true);
        expect(child.env.received).toBe(invocation); expect(child.env.preparation).toBeUndefined(); expect(child.env.SEMIO_BUILD_MODE).toBe(row.mode); expect(child.signal.aborted).toBe(false);
        launches++; events.push("spawn"); const emitter = new EventEmitter();
        queueMicrotask(() => { expect(leased).toBe(true); if (row.outcome === "launch-cancelled") { original.abort(failures); expect(child.signal.aborted).toBe(true); emitter.emit("error", child.signal.reason); } else emitter.emit("close", row.exitCode, null); });
        return emitter;
      },
    };
    const launch = new Function(...Object.keys(ports), compiled.outputText + ";return launchDashboard;")(...Object.values(ports));
    const pending = launch(process.cwd(), row.args, options);
    if (row.outcome === "complete") expect(await pending, row.name).toBe(row.exitCode); else await expect(pending).rejects.toBe(failures);
    expect(row.preparationBudget).toBe((database.query("select min(21600000,case when ?1=0 then 21600000 else ?1 end) value").get(row.originalBudget) as { value: number }).value);
    expect([checks, rebuilds], row.name).toEqual([row.stalenessChecks, row.rebuilds]);
    const independent = database.query("with policy as (select ?1 in ('tasks','logs','stop','kill','open') or (?1='daemon' and ?2 in ('status','stop')) allowed), decision as (select not(allowed and ?3) inspect from policy) select inspect checks, inspect and ?4 rebuilds from decision").get(row.args[0], row.args[1] ?? "", Number(row.installed), Number(row.stale !== null)) as { checks: number; rebuilds: number };
    expect({ checks, rebuilds }).toEqual(independent);
    expect(launches).toBe(row.outcome.startsWith("rebuild-") ? 0 : 1);
    expect(events.filter(event => event === "reader")).toHaveLength(launches); expect(events.filter(event => event === "released")).toHaveLength(launches);
    if (rebuilds && launches) expect(events.indexOf("rebuild")).toBeLessThan(events.indexOf("reader"));
    expect(leased).toBe(false);
  } } finally { database.close(); }
  console.log(`[DEBUG] Actual launch and rebuild bodies matched ${fixture.launch.length} shared installation vectors and independent SQLite/Ajv decisions; no native process was launched`);
});
