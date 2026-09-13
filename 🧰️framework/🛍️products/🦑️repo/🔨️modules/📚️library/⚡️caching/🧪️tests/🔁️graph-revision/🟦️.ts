import assert from "node:assert/strict";
import { copyFileSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { createRequire } from "node:module";

/** 🔁️ Compares persistent graph reloads under native Bun and Node after implementation, policy and helper edits. */
export async function testGraphRevision(workspace: string, output: string): Promise<void> {
  const require = createRequire(import.meta.url), fixture = JSON.parse(readFileSync(join(import.meta.dir, "🔣️.json"), "utf8"));
  assert.deepEqual(require("jsonschema").validate(fixture, JSON.parse(readFileSync(join(import.meta.dir, "🧬️schema/🔣️.json"), "utf8"))).errors, []);
  const { runTool } = await import("../../🚀️bootstrap/📦️dependencies/📜️script.ts");
  const root = mkdtempSync(join(output, "graph-revision-")), controller = new AbortController(), stop = (): void => controller.abort();
  process.once("SIGINT", stop); process.once("SIGTERM", stop);
  let passed = false;
  try {
    const results = [];
    for (const runtime of fixture.runtimes) {
      const directory = join(root, runtime);
      for (const path of fixture.files) { const destination = join(directory, fixture.library, path); mkdirSync(dirname(destination), { recursive: true }); copyFileSync(join(workspace, fixture.library, path), destination); }
      writeFileSync(join(directory, "package.json"), '{"type":"module","name":"graph-revision-fixture"}');
      writeFileSync(join(directory, "nx.json"), "{}");
      mkdirSync(join(directory, "package"));
      writeFileSync(join(directory, "package/📋️project.json"), '{"name":"fixture","targets":{"build":{"options":{"command":"bun ./📜️script.ts build"}}}}');
      writeFileSync(join(directory, "package/📜️script.ts"), 'console.log("fixture");');
      writeFileSync(join(directory, "📜️script.ts"), `import assert from "node:assert/strict"; import { readFileSync, writeFileSync } from "node:fs"; import { join } from "node:path"; import { pathToFileURL } from "node:url";
const fixture = ${JSON.stringify(fixture)}, root=process.cwd(), library=join(root,fixture.library), core=join(library,"🟨️.mjs"), policy=join(library,"⚡️caching/🔣️policy.json");
const plugin=(await import(pathToFileURL(core).href)).default, results=[];
const run=async stage => { const rows=await plugin.createNodesV2[1](["package/📋️project.json"],{}, {workspaceRoot:root}); const target=Object.values(rows[0][1].projects)[0].targets.build; results.push({stage,cache:target.cache,executor:target.executor}); };
await run("initial");
const value=JSON.parse(readFileSync(policy,"utf8")); value.uncachedExact.push("build"); writeFileSync(policy,JSON.stringify(value)); await run("policy");
writeFileSync(core,readFileSync(core,"utf8").replace('const DEFAULT_EXECUTOR = "nx:run-commands";','const DEFAULT_EXECUTOR = "nx:noop";')); await run("implementation");
for(const [index,name] of fixture.helpers.entries()){ const path=join(library,name), before=readFileSync(path,"utf8"); writeFileSync(path,'throw new Error("graph helper rejected");\\n'+before); await assert.rejects(run("invalid"),/graph helper rejected/); writeFileSync(path,before); await run("helper-"+index); }
const taxonomy=join(library,"🔣️taxonomy.json"), before=readFileSync(taxonomy,"utf8"); writeFileSync(taxonomy,"{"); await assert.rejects(run("invalid-taxonomy")); writeFileSync(taxonomy,before); await run("taxonomy");
assert.deepEqual(results,fixture.expected); console.log(JSON.stringify(results));
`);
      const result = await runTool(runtime, [join(directory, "📜️script.ts")], directory, AbortSignal.any([controller.signal, AbortSignal.timeout(20000)]), true, { ...process.env, NX_WORKSPACE_ROOT: directory, REPO_ROOT: directory, NODE_PATH: join(workspace, "node_modules") });
      results.push(JSON.parse(result.trim()));
    }
    assert.deepEqual(results[0], results[1]);
    console.log("[DEBUG] Native Bun/Node graph revisions match after policy/code changes, both helpers and taxonomy rejection/recovery without restarting the process PASS");
    passed = true;
  } finally { controller.abort(); process.off("SIGINT", stop); process.off("SIGTERM", stop); if (passed) rmSync(root, { recursive: true, force: true }); }
}
