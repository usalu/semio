import assert from "node:assert/strict";
import { mkdtempSync, readFileSync, writeFileSync, rmSync } from "node:fs";
import { createRequire } from "node:module";
import { join, relative, resolve } from "node:path";
import { spawnSync } from "node:child_process";
import { pathToFileURL } from "node:url";
import { runInNewContext } from "node:vm";

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
    const ts = require("typescript"), pluginPath = resolve(import.meta.dir, "../../../🟨️.mjs");
    const source = ts.createSourceFile("plugin.mjs", readFileSync(pluginPath, "utf8"), ts.ScriptTarget.Latest, true);
    const names = ["createScriptInputCache", "commandImports", "relativeScriptInputs"], reads = new Map<string, number>();
    const definitions = names.map(name => {
      const node = source.statements.find((node: any) => ts.isFunctionDeclaration(node) && node.name?.text === name);
      assert.ok(node, `${name}: graph discovery requires a cache scoped to one pass`);
      return node.getText(source).replaceAll("import.meta.url", JSON.stringify(pathToFileURL(pluginPath).href));
    });
    const discovery = runInNewContext(`${definitions.join("\n")}\n({ createScriptInputCache, relativeScriptInputs });`, {
      SCRIPT_IMPORT_CACHE: new Map(), createRequire, resolve, relative, nxPath: (path: string) => path.replaceAll("\\", "/"),
      readFileSync: (path: string, encoding: BufferEncoding) => { reads.set(path, (reads.get(path) ?? 0) + 1); return readFileSync(path, encoding); },
    });
    const pass = discovery.createScriptInputCache();
    for (let index = 0; index < fixture.discovery.repetitions; index++) for (const path of fixture.discovery.entries) {
      const inputs = Array.from(discovery.relativeScriptInputs([join(root, path)], root, pass)) as string[];
      assert.deepEqual(normalize(inputs), normalize(cacheInternals.relativeScriptInputs([join(root, path)], root)));
    }
    for (const [path, count] of reads) assert.ok(count <= fixture.discovery.maximumReadsPerFile, `${path}: read ${count} times in one graph pass`);
    const nodeClosure = execute(pathToFileURL(resolve(import.meta.dir, "../../../🟨️.mjs")).href, `api.cacheInternals.relativeScriptInputs([${JSON.stringify(entry)}], ${JSON.stringify(root)}).map(path => path.replace("{workspaceRoot}/", "")).sort()`);
    assert.deepEqual(nodeClosure, original, "Node-hosted Nx must resolve the same TypeScript imports as Bun and esbuild");
    writeFileSync(join(root, "🟦️.ts"), fixture.changedSource); writeFileSync(join(root, "replacement.ts"), fixture.replacement);
    const changed = normalize(cacheInternals.relativeScriptInputs([entry], root));
    assert.deepEqual(normalize(Array.from(discovery.relativeScriptInputs([entry], root, discovery.createScriptInputCache()))), changed, "A new graph pass must observe changed transitive imports");
    const rebuilt = await build();
    assert.deepEqual(changed, oracle(rebuilt)); assert.deepEqual(executeBundle(rebuilt), fixture.changedExecution);
    assert.ok(changed.includes("replacement.ts")); assert.ok(!changed.includes("value.ts"));
    const factory = join(root, fixture.factory.entry); writeFileSync(factory, fixture.factory.source);
    assert.deepEqual(normalize(cacheInternals.relativeScriptInputs([factory], root)), fixture.factory.runtimeInputs);
    assert.equal(execute(pathToFileURL(factory).href, "api.value"), fixture.factory.result);
    console.log(`[DEBUG] Command inputs match esbuild and native Node; ${fixture.discovery.repetitions * fixture.discovery.entries.length} closures read each source once per graph pass and refresh after source edits PASS`);
  } finally { rmSync(root, { recursive: true, force: true }); }
}
