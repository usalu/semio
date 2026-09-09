import assert from "node:assert/strict";
import { readFileSync, mkdtempSync, writeFileSync, rmSync } from "node:fs";
import { createRequire } from "node:module";
import { join, resolve } from "node:path";
import { spawnSync } from "node:child_process";

/** 🚦️ Runs the real baseline executor and compares workflow outputs with its schema-valid JSON and native Git commit. */
export async function testCiBaselineCommand(workspace: string, output: string): Promise<void> {
  const require = createRequire(import.meta.url), lodash = require("lodash"), ajv = new (require("ajv"))();
  const fixture = JSON.parse(readFileSync(join(import.meta.dir, "../../🧫️fixtures/🚦️baseline-command/🔣️.json"), "utf8")), root = resolve(import.meta.dir, "../.."), script = join(root, "📜️script.ts");
  const project = JSON.parse(readFileSync(join(root, "../📋️project.json"), "utf8")), target = project.targets[fixture.target];
  assert.equal(project.name, fixture.project); assert.equal(target.cache, false); assert.deepEqual(target.outputs, []); assert.equal(target.options.command, fixture.command);
  const bundle = await require("esbuild").build({ entryPoints: [script], absWorkingDir: workspace, bundle: true, write: false, platform: "node", format: "esm", packages: "external", metafile: true, logLevel: "silent" });
  const inputs = Object.keys(bundle.metafile.inputs);
  assert.ok(inputs.length <= fixture.maximumImports); assert.ok(inputs.every(path => !/🧪️tests|📦️packages\/🟦️typescript\/🟦️.ts/.test(path)));
  const schema = JSON.parse(readFileSync(join(root, "🧭️baseline/🧬️schema/🔣️.json"), "utf8")), valid = ajv.compile({ ...schema, $ref: "#/definitions/selection" });
  const context = JSON.parse(readFileSync(join(root, "🧭️baseline/🌿️environment/🧫️fixtures/🌿️workflow-context/🔣️.json"), "utf8"));
  const directory = mkdtempSync(join(output, "ci-command-"));
  try {
    const eventPath = join(directory, "event.json"), outputPath = join(directory, "output.txt");
    writeFileSync(eventPath, JSON.stringify(context.event));
    let execution: ReturnType<typeof spawnSync> | undefined, head = "";
    for (let attempt = 0; attempt < 3; attempt++) {
      const git = spawnSync("git", ["rev-parse", "--verify", "HEAD^{commit}"], { cwd: workspace, encoding: "utf8" }); assert.equal(git.status, 0, git.stderr); head = git.stdout.trim();
      writeFileSync(outputPath, "");
      execution = spawnSync(process.execPath, [script, ...fixture.arguments], { cwd: workspace, env: { ...process.env, ...context.environment, GITHUB_SHA: head, GITHUB_EVENT_PATH: eventPath, GITHUB_OUTPUT: outputPath, GITHUB_TOKEN: "" }, encoding: "utf8", timeout: 30000, maxBuffer: 1024 * 1024 });
      if (execution.status === 0 || !String(execution.stderr).includes("Invalid GitHub checkout")) break;
    }
    assert.equal(execution?.status, 0, String(execution?.stderr));
    const result = JSON.parse(String(execution!.stdout).trim());
    assert.equal(valid(result.baseline), true); assert.equal(result.baseline.mode, "all"); assert.equal(result.baseline.reason, "full-requested"); assert.equal(result.baseline.head, head); assert.equal(result.historyIssue, null);
    const outputs = lodash.fromPairs(readFileSync(outputPath, "utf8").trim().split("\n").map(line => line.split("=")));
    assert.deepEqual(Object.keys(outputs), fixture.outputKeys); assert.deepEqual(outputs, { mode: "all", base: "", head });
    const invalid = spawnSync(process.execPath, [script, "baseline", "--base=HEAD~1"], { cwd: workspace, encoding: "utf8", timeout: 30000 });
    assert.notEqual(invalid.status, 0); assert.match(invalid.stderr, /accepts only --full/);
    const parse = require("jsonc-parser").parse;
    for (const file of [".vscode/🧩️launch.seed.jsonc", ".vscode/launch.json"]) assert.equal(parse(readFileSync(join(workspace, file), "utf8")).configurations.filter((row: any) => row.command === `bun nx run ${fixture.project}:${fixture.target}`).length, 1);
    console.log("[DEBUG] Native CI baseline executor preserves checkout identity, emits matching workflow outputs, rejects overrides and has one editor entry PASS");
  } finally { rmSync(directory, { recursive: true, force: true }); }
}
