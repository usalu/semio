#!/usr/bin/env bun
/** ⚙️ Registers the native UI package and its semantic UI-axis generator commands. */
import { BundleScript, ScriptRouter, buildBudgetMs, resolveTestLevel, runBundleScriptMain, runCargoTestBudgeted, runCmd } from "../../../../🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { CheckAxesScript, GenerateAxesScript, PreviewGeneratedScript } from "../../🎚️axes/🏃️execution/🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestBudgeted([], this.root, ["--features", "tui-terminal,wgpu", ...rest]);
  }
}

/**
 * 🧊️ The retained wgpu engine's own unit laws: `--lib`, because every one of the
 * `🧪️tests/🔬️targets-wgpu-*` case directories is mounted into the library with `#[cfg(test)]
 * #[path = …]` rather than declared as a `[[test]]` binary — `--lib` is what compiles and runs them,
 * and leaving it off drags in integration targets this lane does not own.
 */
class TestWgpuEngineScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestBudgeted([], this.root, ["--features", "wgpu-engine", "--lib", ...rest]);
  }
}

class CheckWasmScript extends BundleScript {
  run(): void {
    const check = (args: string[]) => runCmd("cargo", ["check", "-p", "semio-framework-ui", ...args], { cwd: this.root, budgetMs: buildBudgetMs() });
    check(["--target", "wasm32-unknown-unknown", "--features", "tui"]);
    check(["--target", "wasm32-unknown-unknown", "--features", "tui-bindgen"]);
    check(["--target", "wasm32-wasip2", "--features", "wgpu"]);
  }
}

class CheckWgpuEngineWasmScript extends BundleScript {
  run(): void {
    runCmd("cargo", ["check", "-p", "semio-framework-ui", "--target", "wasm32-unknown-unknown", "--features", "wgpu-engine"], { cwd: this.root, budgetMs: buildBudgetMs() });
  }
}

if (import.meta.main) {
  const router = new ScriptRouter(import.meta.dir).register("generate", GenerateAxesScript).register("preview-generated", PreviewGeneratedScript).register("check", CheckAxesScript).register("test", TestScript).register("test-wgpu-engine", TestWgpuEngineScript).register("check-wasm", CheckWasmScript).register("check-wgpu-engine-wasm", CheckWgpuEngineWasmScript);
  await runBundleScriptMain(router, import.meta.url);
}
