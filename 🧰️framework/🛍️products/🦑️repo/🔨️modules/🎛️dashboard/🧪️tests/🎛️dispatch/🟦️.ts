import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import Ajv2020 from "ajv/dist/2020";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import ts from "typescript";
import schema from "../../🧬️schema/🎛️dispatch/🔣️.json" with { type: "json" };
import fixture from "../🧫️fixtures/🎛️dispatch/🔣️.json" with { type: "json" };

test("dashboard test dispatch retains one current context through every selected suite", async () => {
  const oracle = new Ajv2020({ strict: true }).compile(schema);
  expect(oracle(fixture), JSON.stringify(oracle.errors)).toBe(true);
  expect(oracle({ cases: [{ segments: ["quick"] }] })).toBe(false);
  const file = new URL("../../📦️packages/🦀️rust/📜️script.ts", import.meta.url), text = readFileSync(file, "utf8"), tree = ts.createSourceFile(file.pathname, text, ts.ScriptTarget.Latest, true);
  expect(ts.transpileModule(text, { compilerOptions: { target: ts.ScriptTarget.ES2022 }, reportDiagnostics: true }).diagnostics).toEqual([]);
  const command = tree.statements.find((node): node is ts.ClassDeclaration => ts.isClassDeclaration(node) && node.name?.text === "TestScript");
  expect(command).toBeDefined();
  const compiled = ts.transpileModule(command!.getText(tree), { compilerOptions: { target: ts.ScriptTarget.ES2022 }, reportDiagnostics: true });
  expect(compiled.diagnostics).toEqual([]);
  const database = new Database(":memory:");
  try { for (const row of fixture.cases) {
    const invocation = Object.freeze({ fixture: "received", control: { signal: new AbortController().signal } }), control = Object.freeze({ signal: new AbortController().signal }), options = Object.freeze({ invocation, control, workspace: process.cwd(), mode: "prototype", network: { offline: true } });
    const selected: unknown[][] = [], commands: unknown[][] = [], owned: unknown[][] = [], battles: unknown[][] = [], installed: unknown[][] = [], native: unknown[][] = [], factories: unknown[][] = [];
    const events: string[] = [], authorityCall = (call: unknown[]) => (call[1] as string[]).some(value => value.replaceAll("\\", "/").endsWith("🧭️authority/🟦️.ts"));
    const ports = {
      BundleScript: class { root = join(process.cwd(), "dashboard"); repoRoot = process.cwd(); invocation = invocation; },
      process: { env: {}, execPath: process.execPath }, join, crates: ["dashboard-fixture"], BATTLE_SUITES: ["coverage", "journeys", "load", "smoke"],
      dashboardOptions: (...args: unknown[]) => { factories.push(args); return options; },
      dashboardExecutable: async (...args: unknown[]) => { expect(args[2]).toBe(options); events.push("select"); selected.push(args); return "installed-dashboard"; },
      runCmd: (...args: unknown[]) => { events.push(authorityCall(args) ? "authority" : "command"); commands.push(args); },
      devToolingEnv: (env: unknown = {}) => env, repoTestArtifactEnvironment: () => ({}),
      compilerAdmissionBudgetV1: (value: unknown) => { expect(value).toBe(control); return {}; },
      mkdir: async () => {},
      runOwnedCommand: async (...args: unknown[]) => { events.push("owned"); owned.push(args); },
      battle: async (...args: unknown[]) => { expect(args[3]).toBe(options); events.push("battle"); battles.push(args); },
      resolveTestLevel: (segments: string[]) => ({ rest: segments.slice(1) }),
      withInstalledDashboardV1: async (workspace: unknown, context: unknown, body: (path: string) => Promise<void>) => { expect(context).toBe(options); events.push("installed"); installed.push([workspace, context]); await body("installed-dashboard"); },
      runRepositoryCargoTests: async (...args: unknown[]) => { events.push("native"); native.push(args); },
    };
    const Command = new Function(...Object.keys(ports), compiled.outputText + ";return TestScript;")(...Object.values(ports));
    await new Command().run([...row.segments]);
    expect(factories).toEqual([[invocation, process.cwd()]]);
    expect(selected.length, row.segments.join(" ")).toBe(row.executableCalls);
    for (const call of selected) expect(call.slice(1)).toEqual([process.cwd(), options]);
    expect(commands.filter(authorityCall)).toHaveLength(1);
    expect(events.slice(0, 2)).toEqual(["select", "authority"]);
    expect(events.slice(2)).not.toContain("authority");
    expect(owned.length).toBe(Number(row.owned));
    if (row.owned) expect((owned[0]![5] as { signal: unknown }).signal).toBe(control.signal);
    expect(battles.map(call => call[2])).toEqual(row.battle ? [row.battle] : []);
    expect(installed.length).toBe(Number(row.installed));
    expect(native.length).toBe(Number(row.native));
    if (row.native) expect(native[0]![2]).toBe(invocation);
    const actual = { executableCalls: selected.length, owned: owned.length > 0, battle: battles[0]?.[2] ?? null, installed: installed.length > 0, native: native.length > 0 };
    const independent = database.query("select case when ?1='quick' then 2 else 1 end executableCalls, ?1='execution' owned, case when ?1 in ('coverage','journeys','load','smoke') then ?1 else null end battle, ?1='quick' installed, ?1='quick' native").get(row.segments[0]!) as { executableCalls: number; owned: number; battle: string | null; installed: number; native: number };
    expect(actual).toEqual({ ...independent, owned: Boolean(independent.owned), installed: Boolean(independent.installed), native: Boolean(independent.native) });
  } } finally { database.close(); }
  console.log(`[DEBUG] Actual dashboard test dispatcher retained the received context across ${fixture.cases.length} shared suite vectors; process ports were isolated`);
});
