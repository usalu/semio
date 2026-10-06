import { expect, test } from "bun:test";
import { readFileSync, mkdirSync, mkdtempSync, rmSync } from "node:fs";
import { resolve, join } from "node:path";
import { createRequire } from "node:module";
import { spawnSync } from "node:child_process";

test("execution budgets follow portable environment laws and independent native oracles", async () => {
  const owner = resolve(import.meta.dir, ".."), source = join(owner, "🟦️.ts");
  const corpus = JSON.parse(readFileSync(join(owner, "🧫️fixtures/🔣️.json"), "utf8"));
  const schema = JSON.parse(readFileSync(join(owner, "🧬️schema/🔣️.json"), "utf8"));
  const require = createRequire(import.meta.url), ajv = new (require("ajv").default)();
  ajv.addSchema(schema);
  const api = await import(source);
  const read = (api: any, env: Readonly<Record<string, string>>): unknown => {
    try { return { build: api.buildBudgetMs(env), command: api.cmdBudgetMs(env), orchestrator: api.orchestratorBudgetMs(env), daemon: api.daemonBudgetMs(env) }; }
    catch { return null; }
  };
  const values = corpus.cases.map((row: any) => read(api, row.environment));
  expect(values).toEqual(corpus.cases.map((row: any) => row.expected));
  const validate = ajv.compile({ $ref: schema.$id + "#/$defs/Budgets" });
  for (let index = 0; index < values.length; index++) if (values[index] !== null) expect(validate(values[index])).toBe(true);
  const output = process.env.SEMIO_TEST_ARTIFACT_DIR!; expect(output).toBeTruthy(); mkdirSync(output, { recursive: true });
  const temporary = mkdtempSync(join(output, "execution-budget-")), module = join(temporary, "budget.mjs");
  try {
    const bundled = await require("esbuild").build({ entryPoints: [source], outfile: module, bundle: true, platform: "node", format: "esm", metafile: true });
    expect(Object.keys(bundled.metafile.inputs).sort()).toEqual([source.substring(process.cwd().length + 1), join(owner, "🧬️schema/🔣️.json").substring(process.cwd().length + 1)].sort());
    const program = `const rows=JSON.parse(require('node:fs').readFileSync(0,'utf8')).cases;const read=${read.toString()};import(require('node:url').pathToFileURL(process.argv[1]).href).then(api=>console.log(JSON.stringify(rows.map(row=>read(api,row.environment)))));`;
    const child = spawnSync("node", ["-e", program, module], { input: JSON.stringify(corpus), encoding: "utf8", timeout: 15000 });
    expect(child.status, child.stderr).toBe(0); expect(JSON.parse(child.stdout)).toEqual(values);
    const oracle = spawnSync(process.platform === "win32" ? "python" : "python3", ["-c", `import sys,json,decimal\ndata=json.load(sys.stdin); result=[]\nkeys={'build':'SEMIO_BUILD_BUDGET_MS','command':'SEMIO_CMD_BUDGET_MS','orchestrator':'SEMIO_ORCHESTRATOR_BUDGET_MS','daemon':'SEMIO_DAEMON_BUDGET_MS'}\nfor row in data['cases']:\n try:\n  values={kind:decimal.Decimal(row['environment'].get(key,'0').strip() or '0') for kind,key in keys.items()}\n  if any(not value.is_finite() or value!=value.to_integral_value() or value<0 or value>9007199254740991 for value in values.values()): raise ValueError('invalid')\n  result.append({kind:int(value) for kind,value in values.items()})\n except (ValueError,decimal.InvalidOperation): result.append(None)\nprint(json.dumps(result))`], { input: JSON.stringify(corpus), encoding: "utf8", timeout: 15000 });
    expect(oracle.status, oracle.stderr).toBe(0); expect(JSON.parse(oracle.stdout)).toEqual(values);
  } finally { rmSync(temporary, { recursive: true, force: true }); }
});
