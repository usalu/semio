import { expect, test } from "bun:test";
import { readFileSync, mkdtempSync } from "node:fs";
import { join } from "node:path";
import { createRequire } from "node:module";
import { fileURLToPath } from "node:url";
import Ajv from "ajv";
import { buildBudgetMs, cmdBudgetMs, daemonBudgetMs, defaultBudgetMs, orchestratorBudgetMs } from "../../🏃️process/🟦️.ts";

const fixture = JSON.parse(readFileSync(new URL("./🔣️.json", import.meta.url), "utf8"));
const schema = JSON.parse(readFileSync(new URL("./🛂️schema/🔣️.json", import.meta.url), "utf8"));
const libraryPath = fileURLToPath(new URL("../../📦️packages/🟦️typescript/🟦️.ts", import.meta.url));
const execa = createRequire(import.meta.url)("execa");
const budgetKeys = ["SEMIO_BUILD_BUDGET_MS", "SEMIO_CMD_BUDGET_MS", "SEMIO_ORCHESTRATOR_BUDGET_MS", "SEMIO_DAEMON_BUDGET_MS"];

/** 🧼️ Keeps the budget contract independent of the developer's launch environment. */
function cleanEnv(): NodeJS.ProcessEnv {
  return { ...process.env, ...Object.fromEntries(budgetKeys.map(key => [key, undefined])), SEMIO_COVERAGE: "0" };
}

test("process budgets follow neutral defaults and opt-in overrides", () => {
  expect(new Ajv({ strict: true }).validate(schema, fixture)).toBe(true);
  const previous = budgetKeys.map(key => process.env[key]);
  const readers = { build: buildBudgetMs, command: cmdBudgetMs, orchestrator: orchestratorBudgetMs, daemon: daemonBudgetMs };
  try {
    for (const key of budgetKeys) delete process.env[key];
    expect(Object.fromEntries(Object.entries(readers).map(([kind, read]) => [kind, read()]))).toEqual(fixture.defaults);
    expect(defaultBudgetMs("cargo")).toBe(fixture.defaults.build);
    expect(defaultBudgetMs("bun")).toBe(fixture.defaults.command);
    for (const value of [125, 0]) {
      for (const key of budgetKeys) process.env[key] = String(value);
      expect(Object.values(readers).map(read => read())).toEqual([value, value, value, value]);
    }
  } finally {
    budgetKeys.forEach((key, index) => {
      if (previous[index] === undefined) delete process.env[key];
      else process.env[key] = previous[index];
    });
  }
});

for (const row of fixture.processes) {
  test(`process budgets match Execa completion and timeout: ${row.name}`, async () => {
    const child = `await new Promise(resolve => setTimeout(resolve, ${row.durationMs})); console.log("build-complete");`;
    const code = `const { runProbe } = await import(${JSON.stringify(libraryPath)}); try { const result = runProbe(process.execPath, ["-e", ${JSON.stringify(child)}], { budgetMs: ${row.budgetMs} }); console.log(JSON.stringify({ status: result.status, output: result.stdout.trim(), timeout: false })); } catch (error) { console.log(JSON.stringify({ status: null, output: "", timeout: error.code === "ETIMEDOUT" })); }`;
    const [reference, actual] = await Promise.all([
      execa(process.execPath, ["-e", child], { env: cleanEnv(), timeout: row.budgetMs, reject: false }),
      execa(process.execPath, ["-e", code], { env: cleanEnv(), timeout: 10000, reject: false }),
    ]);
    expect(actual.code).toBe(0);
    const result = JSON.parse(actual.stdout);
    expect(result.timeout).toBe(row.timeout);
    expect(result.output).toBe(row.output);
    expect(Boolean(reference.timedOut)).toBe(result.timeout);
    expect(reference.stdout).toBe(result.output);
    if (!row.timeout) expect(result.status).toBe(reference.code);
    console.log(`[DEBUG] ${row.name}: ${JSON.stringify(result)}`);
  }, 12000);
}

for (const runner of ["runCmd", "runCmdStatus", "runTestBudgeted"]) {
  test(`process budgets let ${runner} complete with the unlimited build preset`, async () => {
    const code = `const { ${runner}, buildBudgetMs } = await import(${JSON.stringify(libraryPath)}); await ${runner}(process.execPath, ["-e", "await Bun.sleep(120); console.log('build-complete')"], { budgetMs: buildBudgetMs() });`;
    const result = await execa(process.execPath, ["-e", code], { env: { ...cleanEnv(), SEMIO_CMD_BUDGET_MS: "1", SEMIO_TEST_BUDGET_MS: "1" }, timeout: 5000, reject: false });
    expect(result.code, result.stderr).toBe(0);
    expect(result.stdout).toBe("build-complete");
    expect(result.stderr).not.toContain("[budget]");
  });
}

test("process budgets preserve explicit async test deadlines", async () => {
  const code = `const { runTestBudgeted } = await import(${JSON.stringify(libraryPath)}); await runTestBudgeted(process.execPath, ["-e", "await Bun.sleep(2000)"], { budgetMs: 100 });`;
  const result = await execa(process.execPath, ["-e", code], { env: cleanEnv(), timeout: 5000, reject: false });
  expect(result.code).not.toBe(0);
  expect(result.stderr).toContain("[budget]");
});

test("process budgets let captured nextest compilation finish before budgeted assertions", async () => {
  const artifactRoot = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!artifactRoot) throw new Error("SEMIO_TEST_ARTIFACT_DIR must name the active ticket generated directory");
  const directory = mkdtempSync(join(artifactRoot, "unlimited-nextest-"));
  const code = `
    const { mock } = await import("bun:test");
    const native = await import("node:child_process");
    const spawn = native.spawn, spawnSync = native.spawnSync;
    mock.module("node:child_process", () => ({
      ...native,
      spawnSync: (cmd, args, opts) => cmd === "cargo" ? { status: 0 } : spawnSync(cmd, args, opts),
      spawn: (cmd, args, opts) => cmd === "cargo" ? spawn(process.execPath, ["-e", args.includes("list") ? "await Bun.sleep(120); console.log('{}')" : "console.log('assertions-complete')"], opts) : spawn(cmd, args, opts),
    }));
    const { runCargoTestBudgeted } = await import(${JSON.stringify(libraryPath)});
    await runCargoTestBudgeted([], process.cwd());
  `;
  const result = await execa(process.execPath, ["-e", code], { env: { ...cleanEnv(), SEMIO_BUILD_BUDGET_MS: "0", SEMIO_TEST_BUDGET_MS: "1000", SEMIO_TEST_ARTIFACT_DIR: directory }, timeout: 5000, reject: false });
  expect(result.code, result.stderr).toBe(0);
  expect(result.stdout).toBe("assertions-complete");
});

for (const nextest of [true, false]) {
  test(`process budgets separate coverage compilation from assertions: nextest=${nextest}`, async () => {
    const code = `
      const { mock } = await import("bun:test");
      const native = await import("node:child_process");
      const spawn = native.spawn, spawnSync = native.spawnSync;
      mock.module("node:child_process", () => ({
        ...native,
        spawnSync: (cmd, args, opts) => cmd === "cargo" ? { status: ${nextest ? 0 : 1} } : spawnSync(cmd, args, opts),
        spawn: (cmd, args, opts) => {
          if (cmd !== "cargo") return spawn(cmd, args, opts);
          const building = args.includes("--no-run") || args.includes("--list");
          const reporting = args.includes("report");
          if (!building && !reporting && !args.includes("--no-clean")) throw new Error("Coverage must reuse its compiled artifacts");
          return spawn(process.execPath, ["-e", building ? "await Bun.sleep(300); console.log('coverage-built')" : reporting ? "console.log('coverage-reported')" : "console.log('assertions-complete')"], opts);
        },
      }));
      const { runCargoTestBudgeted } = await import(${JSON.stringify(libraryPath)});
      await runCargoTestBudgeted([], process.cwd());
    `;
    const result = await execa(process.execPath, ["-e", code], { env: { ...cleanEnv(), SEMIO_COVERAGE: "1", SEMIO_TEST_BUDGET_MS: "200" }, timeout: 5000, reject: false });
    expect(result.code, result.stderr).toBe(0);
    expect(result.stdout).toBe("coverage-built\nassertions-complete\ncoverage-reported");
  });
}

test("process budgets enforce an explicitly selected build deadline", async () => {
  const code = `const { runCmd, buildBudgetMs } = await import(${JSON.stringify(libraryPath)}); runCmd(process.execPath, ["-e", "await Bun.sleep(2000)"], { budgetMs: buildBudgetMs() });`;
  const result = await execa(process.execPath, ["-e", code], { env: { ...cleanEnv(), SEMIO_BUILD_BUDGET_MS: "100" }, timeout: 5000, reject: false });
  expect(result.code).not.toBe(0);
  expect(result.stderr).toContain("exceeded 100ms");
});
