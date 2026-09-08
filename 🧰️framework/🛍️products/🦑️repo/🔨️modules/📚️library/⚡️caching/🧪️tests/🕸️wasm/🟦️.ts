import assert from "node:assert/strict";
import { chmodSync, mkdirSync, mkdtempSync, readFileSync, realpathSync, rmSync, writeFileSync } from "node:fs";
import { spawnSync } from "node:child_process";
import { createRequire } from "node:module";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

/** 🧠️ Verifies the declared optimizer against native Binaryen and WebAssembly execution. */
export async function testWasmOptimizer(workspace: string, output: string): Promise<void> {
  const require = createRequire(import.meta.url), fixture = JSON.parse(readFileSync(join(dirname(fileURLToPath(import.meta.url)), "🔣️.json"), "utf8"));
  const source = readFileSync(join(workspace, fixture.manifest), "utf8"), manifest = Bun.TOML.parse(source) as any;
  assert.deepEqual(manifest, require("smol-toml").parse(source));
  for (const profile of fixture.profiles) assert.deepEqual(manifest.package.metadata["wasm-pack"]?.profile?.[profile]?.["wasm-opt"], fixture.optimizer, `${profile} must optimize Rust bulk-memory instructions`);
  for (const path of fixture.disabledCustom) {
    const source = readFileSync(join(workspace, path), "utf8"), manifest = Bun.TOML.parse(source) as any;
    assert.deepEqual(manifest, require("smol-toml").parse(source));
    assert.equal(manifest.package.metadata["wasm-pack"].profile.custom?.["wasm-opt"], false, `${path}: custom profiles must preserve the declared optimizer policy`);
  }
  const directory = mkdtempSync(join(output, "wasm-optimizer-"));
  try {
    const { wasmPackEnvironment } = await import("../../../📦️packages/🟦️typescript/🟦️.ts");
    const suffix = process.platform === "win32" ? ".exe" : "", delimiter = process.platform === "win32" ? ";" : ":";
    const bindings = join(directory, fixture.tools.bindgenDirectory), native = join(directory, fixture.tools.optimizerDirectory), ambient = join(directory, fixture.tools.ambientDirectory);
    for (const path of [bindings, native, ambient]) mkdirSync(path);
    const bindgen = join(bindings, "wasm-bindgen" + suffix), selected = join(native, "wasm-opt" + suffix);
    for (const path of [bindgen, selected, join(ambient, "wasm-opt" + suffix)]) { writeFileSync(path, "fixture tool"); chmodSync(path, 0o755); }
    const env = wasmPackEnvironment(directory, bindgen, { PATH: ambient + delimiter + process.env.PATH, SEMIO_WASM_OPT_BIN: selected });
    assert.equal(realpathSync(Bun.which("wasm-opt", { PATH: env.PATH })!), realpathSync(selected));
    const oracle = spawnSync(process.platform === "win32" ? "where.exe" : "/usr/bin/which", ["wasm-opt"], { encoding: "utf8", env: { ...process.env, ...env } });
    assert.equal(oracle.status, 0, oracle.stderr);
    assert.equal(realpathSync(oracle.stdout.trim().split(/\r?\n/)[0]!), realpathSync(selected));
    assert.throws(() => wasmPackEnvironment(directory, bindgen, { SEMIO_WASM_OPT_BIN: join(native, fixture.tools.invalidName) }), /wasm-opt/);
    const shadow = join(bindings, "wasm-opt" + suffix); writeFileSync(shadow, "shadow tool"); chmodSync(shadow, 0o755);
    assert.throws(() => wasmPackEnvironment(directory, bindgen, { SEMIO_WASM_OPT_BIN: selected }), /shadow/);
    const input = join(directory, "input.wat"), target = join(directory, "output.wasm");
    writeFileSync(input, fixture.module);
    const optimizer = require.resolve("binaryen/bin/wasm-opt"), result = Bun.spawnSync([process.execPath, optimizer, input, "-o", target, ...fixture.optimizer], { cwd: workspace, stdout: "pipe", stderr: "pipe", timeout: 30_000 });
    assert.equal(result.exitCode, 0, result.stderr.toString());
    const instance = new WebAssembly.Instance(new WebAssembly.Module(readFileSync(target))), memory = new Uint8Array((instance.exports.memory as WebAssembly.Memory).buffer);
    memory.set(fixture.initial);
    (instance.exports.copy as (...args: number[]) => void)(...fixture.copy);
    const memoryOracle = Uint8Array.from(fixture.initial);
    memoryOracle.copyWithin(fixture.copy[0], fixture.copy[1], fixture.copy[1] + fixture.copy[2]);
    assert.deepEqual([...memoryOracle], fixture.expected);
    assert.deepEqual([...memory.slice(0, fixture.initial.length)], [...memoryOracle]);
    for (const row of fixture.saturation) {
      const saturated = Math.min(0xffffffff, Math.max(0, Math.trunc(row.value))) | 0;
      assert.equal(saturated, row.expected);
      assert.equal((instance.exports.saturate as (value: number) => number)(row.value), saturated);
    }
    console.log("[DEBUG] Release/custom optimizer metadata and native Binaryen bulk-memory/saturating conversion execution PASS");
  } finally { rmSync(directory, { recursive: true, force: true }); }
}
