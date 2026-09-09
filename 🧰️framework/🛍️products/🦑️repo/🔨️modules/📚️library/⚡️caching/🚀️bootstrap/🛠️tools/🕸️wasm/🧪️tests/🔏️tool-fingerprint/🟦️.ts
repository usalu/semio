import assert from "node:assert/strict";
import { existsSync, readFileSync, mkdtempSync, rmSync } from "node:fs";
import { createRequire } from "node:module";
import { join, resolve } from "node:path";
import { spawnSync } from "node:child_process";

/** 🔏️ Proves hashing works without application dependencies or acquired tools and matches native version output. */
export async function testWasmToolFingerprint(workspace: string, output: string): Promise<void> {
  const require = createRequire(import.meta.url), fixture = JSON.parse(readFileSync(join(import.meta.dir, "🔣️.json"), "utf8")), script = resolve(import.meta.dir, "../../📜️script.ts");
  const built = await require("esbuild").build({ entryPoints: [script], absWorkingDir: workspace, bundle: true, write: false, platform: "node", format: "esm", packages: "external", metafile: true, logLevel: "silent" });
  assert.ok(Object.keys(built.metafile.inputs).length <= fixture.maximumImports);
  assert.ok(Object.keys(built.metafile.inputs).every(path => !/🧪️tests|📦️packages\/🟦️typescript\/🟦️.ts/.test(path)));
  const { binaryenIdentity, preparedBinaryen, binaryenDirectory } = await import("../../📜️script.ts");
  const directory = mkdtempSync(join(output, "tool-fingerprint-"));
  const environment = { ...process.env, PATH: "", SEMIO_WASM_BINDGEN_BIN: "", SEMIO_WASM_OPT_BIN: "", REPO_ROOT: workspace, NX_WORKSPACE_ROOT: workspace };
  try {
    const cold = spawnSync(process.execPath, [script, fixture.command], { cwd: directory, env: environment, encoding: "utf8", timeout: 30000 });
    assert.equal(cold.status, 0, cold.stderr);
    const value = JSON.parse(cold.stdout); assert.deepEqual(Object.keys(value), fixture.tools);
    for (const tool of fixture.unavailable) assert.equal(value[tool], "unavailable");
    assert.equal(value["wasm-opt"], binaryenIdentity()); assert.equal(existsSync(binaryenDirectory(directory)), false);
    const optimizer = preparedBinaryen(workspace), oracle = spawnSync(optimizer, ["--version"], { encoding: "utf8", timeout: 10000 }); assert.equal(oracle.status, 0, oracle.stderr);
    const override = spawnSync(process.execPath, [script, fixture.command], { cwd: directory, env: { ...environment, SEMIO_WASM_OPT_BIN: optimizer }, encoding: "utf8", timeout: 30000 });
    assert.equal(override.status, 0, override.stderr); assert.equal(JSON.parse(override.stdout)["wasm-opt"], oracle.stdout.trim());
    const invalid = spawnSync(process.execPath, [script, fixture.command, "--install"], { cwd: directory, env: environment, encoding: "utf8", timeout: 30000 });
    assert.notEqual(invalid.status, 0); assert.match(invalid.stderr, /accepts no arguments/);
    const policy = JSON.parse(readFileSync(resolve(import.meta.dir, "../../../../../🔣️policy.json"), "utf8"));
    assert.ok(policy.toolchains.wasm.commands.includes(`bun ${JSON.stringify(script.slice(workspace.length + 1))} fingerprint`));
    console.log("[DEBUG] Isolated WASM fingerprint has no application/test imports, acquires no tools and matches native optimizer version output PASS");
  } finally { rmSync(directory, { recursive: true, force: true }); }
}
