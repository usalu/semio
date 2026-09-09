import assert from "node:assert/strict";
import { mkdtempSync, readFileSync, writeFileSync, rmSync } from "node:fs";
import { createRequire } from "node:module";
import { join, relative, resolve } from "node:path";
import { spawnSync } from "node:child_process";
import { pathToFileURL } from "node:url";

/** 🔗️ Checks runtime import ownership against esbuild, including source-byte changes and erased types. */
export async function testCommandImportClosure(workspace: string, output: string): Promise<void> {
  const require = createRequire(import.meta.url), fixture = JSON.parse(readFileSync(join(import.meta.dir, "../../🧫️fixtures/command-imports/🔣️.json"), "utf8"));
  const { cacheInternals } = await import("../../../🟨️.mjs"), root = mkdtempSync(join(output, "command-imports-"));
  try {
    for (const [path, source] of Object.entries(fixture.files)) writeFileSync(join(root, path), source as string);
    const entry = join(root, fixture.entry), normalize = (paths: string[]) => paths.map(path => path.replace("{workspaceRoot}/", "")).sort();
    const original = normalize(cacheInternals.relativeScriptInputs([entry], root));
    assert.deepEqual(original, fixture.runtimeInputs.toSorted());
    const build = () => require("esbuild").build({ entryPoints: [entry], absWorkingDir: root, bundle: true, write: false, platform: "node", format: "esm", packages: "external", metafile: true, logLevel: "silent" });
    const oracle = (result: any) => Object.keys(result.metafile.inputs).map(path => relative(root, resolve(root, path))).sort();
    const execute = (url: string, expression: string) => {
      const result = spawnSync("node", ["--input-type=module", "-e", `const api = await import(${JSON.stringify(url)}); console.log(JSON.stringify(${expression}));`], { cwd: root, encoding: "utf8", timeout: 10000 });
      assert.equal(result.status, 0, result.stderr); return JSON.parse(result.stdout);
    };
    const executeBundle = (result: any) => execute(`data:text/javascript;base64,${Buffer.from(result.outputFiles[0].contents).toString("base64")}`, "{result:api.result,lazy:(await api.lazy()).value,sideEffect:globalThis.fixtureLoaded,typeExportSideEffect:Boolean(globalThis.fixtureTypeExportLoaded)}");
    const built = await build();
    assert.deepEqual(original, oracle(built)); assert.deepEqual(executeBundle(built), fixture.execution);
    const nodeClosure = execute(pathToFileURL(resolve(import.meta.dir, "../../../🟨️.mjs")).href, `api.cacheInternals.relativeScriptInputs([${JSON.stringify(entry)}], ${JSON.stringify(root)}).map(path => path.replace("{workspaceRoot}/", "")).sort()`);
    assert.deepEqual(nodeClosure, original, "Node-hosted Nx must resolve the same TypeScript imports as Bun and esbuild");
    writeFileSync(join(root, "🟦️.ts"), fixture.changedSource); writeFileSync(join(root, "replacement.ts"), fixture.replacement);
    const changed = normalize(cacheInternals.relativeScriptInputs([entry], root));
    const rebuilt = await build();
    assert.deepEqual(changed, oracle(rebuilt)); assert.deepEqual(executeBundle(rebuilt), fixture.changedExecution);
    assert.ok(changed.includes("replacement.ts")); assert.ok(!changed.includes("value.ts"));
    const factory = join(root, fixture.factory.entry); writeFileSync(factory, fixture.factory.source);
    assert.deepEqual(normalize(cacheInternals.relativeScriptInputs([factory], root)), fixture.factory.runtimeInputs);
    assert.equal(execute(pathToFileURL(factory).href, "api.value"), fixture.factory.result);
    console.log("[DEBUG] Command inputs follow runtime imports and createRequire factories, ignore erased types, retain compiler resolution inputs and refresh on source-byte edits; esbuild and native Node execution PASS");
  } finally { rmSync(root, { recursive: true, force: true }); }
}
