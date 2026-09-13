import assert from "node:assert/strict";
import { copyFileSync, existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { dirname, join } from "node:path";

/** 🦀️ Native Cargo leaves finish without launching detached storage management behind Nx. */
export async function testCargoCleanupBoundary(workspace: string, output: string): Promise<void> {
  const require = createRequire(import.meta.url), fixture = JSON.parse(readFileSync(join(import.meta.dir, "🔣️.json"), "utf8"));
  assert.deepEqual(require("jsonschema").validate(fixture, JSON.parse(readFileSync(join(import.meta.dir, "🧬️schema/🔣️.json"), "utf8"))).errors, []);
  const project = JSON.parse(readFileSync(join(import.meta.dir, "../../📋️project.json"), "utf8")), [owner, target] = fixture.rootTarget.split(":");
  assert.equal(project.name, owner); assert.equal(project.targets[target].cache, false);
  assert.equal(project.targets[target].options.command, "bun ./📜️script.ts cache-prune");
  const source = readFileSync(join(workspace, fixture.implementation), "utf8"), ts = require("typescript");
  const syntax = ts.createSourceFile(fixture.implementation, source, ts.ScriptTarget.Latest, true);
  const native = syntax.statements.find((node: any) => ts.isClassDeclaration(node) && node.name?.text === "NativeScript");
  assert.ok(native);
  const code = ts.transpileModule(native.getText(syntax), { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText;
  const { runTool } = await import("../../🚀️bootstrap/📦️dependencies/📜️script.ts");
  const root = mkdtempSync(join(output, "cargo-cleanup-")), controller = new AbortController(), signal = AbortSignal.any([controller.signal, AbortSignal.timeout(60000)]);
  const stop = (): void => controller.abort();
  process.once("SIGINT", stop); process.once("SIGTERM", stop);
  let passed = false;
  try {
    const cargo = JSON.parse(readFileSync(join(import.meta.dir, "../../🧫️fixtures/📦️native-dependencies/🔣️.json"), "utf8")).cargo;
    for (const [path, contents] of Object.entries(cargo.files)) writeFileSync(join(root, path), contents as string);
    writeFileSync(join(root, "package.json"), JSON.stringify({ name: "workspace", private: true }));
    writeFileSync(join(root, "nx.json"), "{}");
    const graph = await require("esbuild").build({ absWorkingDir: workspace, entryPoints: [fixture.implementation], bundle: true, write: false, metafile: true, platform: "node", packages: "external", format: "esm", logLevel: "silent" });
    for (const source of Object.keys(graph.metafile.inputs)) {
      const destination = join(root, source); mkdirSync(dirname(destination), { recursive: true }); copyFileSync(join(workspace, source), destination);
    }
    writeFileSync(join(root, dirname(fixture.implementation), "../📜️script.ts"), `import { writeFileSync } from "node:fs"; writeFileSync(${JSON.stringify(join(root, fixture.cleanupMarker))}, "Detached cleanup started");\n`);
    writeFileSync(join(root, "project.json"), JSON.stringify({ name: "fixture", targets: Object.fromEntries(fixture.operations.map((operation: string) => [operation, {
      executor: "nx:run-commands", cache: false, options: { command: `bun "${fixture.implementation}" native cargo ${operation} --manifest Cargo.toml` }
    }])) }));
    const env = { ...process.env, NX_DAEMON: "false", NX_WORKSPACE_ROOT: root, NX_WORKSPACE_ROOT_PATH: root, REPO_ROOT: root, NX_WORKSPACE_DATA_DIRECTORY: join(root, ".nx/data"), NX_CACHE_DIRECTORY: join(root, ".nx/cache"), SEMIO_TEST_ARTIFACT_DIR: root, NODE_PATH: join(workspace, "node_modules"), CARGO_TARGET_DIR: join(root, "target"), CARGO_BUILD_BUILD_DIR: join(root, "build"), FORCE_COLOR: "0", NO_COLOR: "1" };
    for (const operation of fixture.operations) {
      const log = await runTool("node", [require.resolve("nx/bin/nx.js"), "run", `fixture:${operation}`, "--outputStyle=stream"], root, signal, true, env);
      writeFileSync(join(root, `${operation}.log`), log);
      assert.equal(existsSync(join(root, fixture.cleanupStamp)), false, `${operation} must not schedule cleanup after completing its Nx leaf`);
      assert.equal(existsSync(join(root, fixture.cleanupMarker)), false, `${operation} launched detached cleanup`);
    }
    assert.ok(existsSync(join(root, "dist/build/.nx-artifact.json")), "Build still publishes owned artifacts");
    const oracle = JSON.parse(await runTool("cargo", ["metadata", "--locked", "--offline", "--no-deps", "--format-version=1"], root, signal, true, env));
    assert.equal(oracle.packages[0].name, cargo.package);
    for (const entrypoint of fixture.entrypoints) assert.ok(!code.includes(entrypoint), "Cargo must not orchestrate cleanup indirectly");
    console.log("[DEBUG] Native Nx Cargo build/check/test and Cargo metadata succeed without cleanup stamps or detached cleanup processes PASS");
    passed = true;
  } finally { controller.abort(); process.off("SIGINT", stop); process.off("SIGTERM", stop); if (passed) rmSync(root, { recursive: true, force: true }); }
}
