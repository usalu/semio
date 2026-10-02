import { expect, test } from "bun:test";
import { createRequire } from "node:module";
import { readFileSync, mkdirSync, mkdtempSync, rmSync } from "node:fs";
import { resolve, join } from "node:path";
import { spawnSync } from "node:child_process";

test("test commands require explicit budgets and preserve failure and cancellation", async () => {
  const owner = resolve(import.meta.dir, ".."), source = join(owner, "🟦️.ts"), require = createRequire(import.meta.url);
  const fixture = JSON.parse(readFileSync(join(owner, "🧫️fixtures/🔣️.json"), "utf8")), schema = JSON.parse(readFileSync(join(owner, "🧬️schema/🔣️.json"), "utf8"));
  expect(new (require("ajv").default)().validate(schema, fixture)).toBe(true);
  const api = await import(source);
  const output = process.env.SEMIO_TEST_ARTIFACT_DIR!; expect(output).toBeTruthy(); mkdirSync(output, { recursive: true });
  const temporary = mkdtempSync(join(output, "test-command-")), module = join(temporary, "execution.mjs");
  try {
    const bundle = await require("esbuild").build({ entryPoints: [source], outfile: module, bundle: true, platform: "node", format: "esm", metafile: true });
    expect(Object.keys(bundle.metafile.inputs).every(path => path.startsWith("🧰️framework/🔨️modules/🏃️process/"))).toBe(true);
    for (const row of fixture.cases) {
      if (!row.throwOnFailure) continue;
      const cancellation = new AbortController(), timer = row.abortMs === null ? undefined : setTimeout(() => cancellation.abort(), row.abortMs);
      let observed = "success";
      try { await api.runBudgetedTestCommand("node", ["-e", row.program], { cwd: temporary, budgetMs: row.budgetMs, signal: cancellation.signal, throwOnFailure: row.throwOnFailure }); }
      catch (error) { observed = String(error); }
      finally { if (timer) clearTimeout(timer); }
      expect(observed).toContain(row.expected);
    }
    for (const runtime of [process.execPath, "node"]) for (const row of fixture.cases) {
      const program = `const row=JSON.parse(process.argv[2]),cancellation=new AbortController();if(row.abortMs!==null)setTimeout(()=>cancellation.abort(),row.abortMs);import(require('node:url').pathToFileURL(process.argv[1]).href).then(async api=>{try{await api.runBudgetedTestCommand('node',['-e',row.program],{cwd:process.cwd(),budgetMs:row.budgetMs,signal:cancellation.signal,throwOnFailure:row.throwOnFailure});console.log('success')}catch(error){console.log(String(error))}});`;
      const child = spawnSync(runtime, ["-e", program, module, JSON.stringify(row)], { cwd: temporary, encoding: "utf8", timeout: 4000 });
      expect(child.status, child.stderr).toBe(row.throwOnFailure ? 0 : row.exitCode);
      if (row.throwOnFailure) expect(child.stdout).toContain(row.expected);
    }
    const oracle = spawnSync("node", ["-e", fixture.cases[1].program], { cwd: temporary, encoding: "utf8" }); expect(oracle.status).toBe(fixture.cases[1].exitCode);
    await expect(api.runBudgetedTestCommand("node", ["-e", ""], { cwd: temporary, budgetMs: -1, throwOnFailure: true })).rejects.toThrow("Invalid test command budget");
  } finally { rmSync(temporary, { recursive: true, force: true }); }
});
