import assert from "node:assert/strict";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { join } from "node:path";

/** 🧪️ Compares native component optimization with Binaryen's independent JavaScript distribution. */
export async function testPluginCoreOptimization(workspace: string, output: string): Promise<void> {
  const fixture = JSON.parse(readFileSync(join(import.meta.dir, "../../🧫️fixtures/🕸️native-optimization/🔣️.json"), "utf8")), require = createRequire(import.meta.url);
  const { optimizePluginCoreModulesAsync } = await import("../../📦️packages/🟦️typescript/🟦️.ts");
  mkdirSync(output, { recursive: true }); const root = mkdtempSync(join(output, "component-optimizer-"));
  try {
    const input = join(root, "input.wat"), original = join(root, "input.wasm"), oracle = join(root, "oracle.wasm"), js = require.resolve("binaryen/bin/wasm-opt");
    writeFileSync(input, fixture.module);
    for (const [path, flags] of [[original, []], [oracle, fixture.flags]] as [string, string[]][]) {
      const result = Bun.spawnSync([process.execPath, js, input, "-o", path, ...flags], { stdout: "pipe", stderr: "pipe", timeout: 30000 });
      assert.equal(result.exitCode, 0, result.stderr.toString());
    }
    const bytes = readFileSync(original), expected = readFileSync(oracle);
    for (const name of [...fixture.cores, fixture.foreign]) writeFileSync(join(root, name), bytes);
    await optimizePluginCoreModulesAsync(root, fixture.component, { repoRoot: workspace, preview2VendorDir: root, optimize: false, wasmOptBin: "missing-optimizer" });
    for (const name of fixture.cores) assert.deepEqual(readFileSync(join(root, name)), bytes);
    await optimizePluginCoreModulesAsync(root, fixture.component, { repoRoot: workspace, preview2VendorDir: root, optimize: true, signal: AbortSignal.timeout(30000) });
    for (const name of fixture.cores) {
      assert.deepEqual(readFileSync(join(root, name)), expected);
      const instance = new WebAssembly.Instance(new WebAssembly.Module(expected));
      assert.equal((instance.exports.answer as () => number)(), fixture.expected);
    }
    assert.deepEqual(readFileSync(join(root, fixture.foreign)), bytes);
    console.log("[DEBUG] Native component optimization selects owned cores, preserves dev outputs and matches JavaScript Binaryen bytes/runtime PASS");
  } finally { rmSync(root, { recursive: true, force: true }); }
}
