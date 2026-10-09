#!/usr/bin/env bun
import { runOwnedCommand } from "../../../🏃️process/🎛️owned-execution/🟦️.ts";
import { resolve } from "node:path";
import { runCargoTestsV1, readCargoTestPolicyV1 } from "../../../🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import { resolveTestLevel, testLevelBudgetMs } from "../../../🏃️process/🧪️testing/🎚️budget/🟦️.ts";
import { runBudgetedTestCommand } from "../../../🏃️process/🧪️testing/🎛️execution/🟦️.ts";
import { buildBudgetMs, cmdBudgetMs } from "../../../🏃️process/⏱️budget/🟦️.ts";
/** ⚙️ Registers the native UI package and its semantic UI-axis generator commands. */

import { BundleScript, ScriptRouter } from "../../../🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { CheckAxesScript, GenerateAxesScript, PreviewGeneratedScript } from "../../🎚️axes/🏃️execution/🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments[0] === "wgpu-engine") {
      await new ScriptRouter(this.root, this.repoRoot).register("wgpu-engine", TestWgpuEngineScript).run(segments);
      return;
    }
    if (segments[0] === "worker-retirement") {
      if (segments.length !== 1) throw Error("Expected test worker-retirement");
      if (!process.env.SEMIO_TEST_ARTIFACT_DIR) throw Error("SEMIO_TEST_ARTIFACT_DIR must name caller-owned ticket output");
      await runBudgetedTestCommand(process.execPath, ["test", resolve(this.root, "../../🧬️contract/♻️retirement/👷️worker/🧪️tests/🟦️.ts")], { cwd: this.repoRoot, budgetMs: testLevelBudgetMs(), env: process.env, throwOnFailure: true });
      return;
    }
    if (segments[0] === "control-commit") {
      if (segments.length !== 1) throw Error("Expected test control-commit");
      if (!process.env.SEMIO_TEST_ARTIFACT_DIR) throw Error("SEMIO_TEST_ARTIFACT_DIR must name caller-owned ticket output");
      await runBudgetedTestCommand(process.execPath, ["test", resolve(this.root, "../../🧪️tests/🎚️ring-press/🟦️.ts")], { cwd: this.repoRoot, budgetMs: testLevelBudgetMs(), env: process.env, throwOnFailure: true });
      return;
    }
    if (segments[0] === "feature-ownership") {
      if (segments.length !== 1) throw Error("Expected test feature-ownership");
      if (!process.env.SEMIO_TEST_ARTIFACT_DIR) throw Error("SEMIO_TEST_ARTIFACT_DIR must name caller-owned ticket output");
      await runBudgetedTestCommand(process.execPath, ["test", resolve(this.root, "../../🧪️tests/🧊️feature-ownership/🟦️.ts")], { cwd: this.repoRoot, budgetMs: testLevelBudgetMs(), env: process.env, throwOnFailure: true });
      return;
    }
    if (segments[0] === "physical-close") {
      if (segments.length !== 1) throw Error("Expected test physical-close");
      if (!process.env.SEMIO_TEST_ARTIFACT_DIR) throw Error("SEMIO_TEST_ARTIFACT_DIR must name caller-owned ticket output");
      await runBudgetedTestCommand(process.execPath, ["test", resolve(this.root, "../../🧪️tests/📦️prepared-close/🟦️.ts")], { cwd: this.repoRoot, budgetMs: testLevelBudgetMs(), env: process.env, throwOnFailure: true });
      return;
    }
    if (segments[0] === "physical-close-native") {
      if (segments.length !== 1) throw Error("Expected test physical-close-native");
      if (!process.env.SEMIO_TEST_ARTIFACT_DIR) throw Error("SEMIO_TEST_ARTIFACT_DIR must name caller-owned ticket output");
      await runCargoTestsV1({ manifestPath: resolve(this.root, "Cargo.toml"), packages: [], cwd: this.root, extraArgs: ["--features", "wgpu-engine", "--lib", "--offline", "-E", "test(physical_job_close)", "--success-output", "immediate"] }, readCargoTestPolicyV1(process.env));
      return;
    }
    if (segments[0] === "prepared-close") {
      if (segments.length !== 1) throw Error("Expected test prepared-close");
      if (!process.env.SEMIO_TEST_ARTIFACT_DIR) throw Error("SEMIO_TEST_ARTIFACT_DIR must name caller-owned ticket output");
      await runBudgetedTestCommand(process.execPath, ["test", resolve(this.root, "../../🧪️tests/📦️prepared-close/🟦️.ts")], { cwd: this.repoRoot, budgetMs: testLevelBudgetMs(), env: process.env, throwOnFailure: true });
      return;
    }
    if (segments[0] === "commands") {
      if (segments.length !== 1) throw Error("Expected test commands");
      if (!process.env.SEMIO_TEST_ARTIFACT_DIR) throw Error("SEMIO_TEST_ARTIFACT_DIR must name caller-owned ticket output");
      await runBudgetedTestCommand(process.execPath, ["test", resolve(this.root, "../../🧪️tests/🧭️native-command/🟦️.ts")], { cwd: this.repoRoot, budgetMs: testLevelBudgetMs(), env: process.env, throwOnFailure: true });
      return;
    }
    const { rest } = resolveTestLevel(segments);
    await runCargoTestsV1({ manifestPath: resolve(this.root, "Cargo.toml"), packages: [], cwd: this.root, extraArgs: ["--features", "tui-terminal,wgpu", ...rest] }, readCargoTestPolicyV1(process.env));
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
    await runCargoTestsV1({ manifestPath: resolve(this.root, "Cargo.toml"), packages: [], cwd: this.root, extraArgs: ["--features", "wgpu-engine", "--lib", ...rest] }, readCargoTestPolicyV1(process.env));
  }
}

class CheckWasmScript extends BundleScript {
  async run(): Promise<void> {
    const check = (args: string[]) => runOwnedCommand("cargo", ["check", "--manifest-path",resolve(this.root,"Cargo.toml"), "-p", "semio-framework-ui", ...args], this.root, "tool:owner", buildBudgetMs(), {env: process.env});
    await check(["--target", "wasm32-unknown-unknown", "--features", "tui"]);
    await check(["--target", "wasm32-unknown-unknown", "--features", "tui-bindgen"]);
    await check(["--target", "wasm32-wasip2", "--features", "wgpu"]);
  }
}

class CheckWgpuEngineWasmScript extends BundleScript {
  async run(): Promise<void> {
    await runOwnedCommand("cargo", ["check", "--manifest-path",resolve(this.root,"Cargo.toml"), "-p", "semio-framework-ui", "--target", "wasm32-unknown-unknown", "--features", "wgpu-engine"], this.root, "tool:owner", buildBudgetMs(), {env: process.env});
  }
}

class CheckCommandTypesScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw Error("Expected check commands");
    await runOwnedCommand(process.execPath, ["x", "tsc", "--noEmit", "--incremental", "false", "-p", resolve(this.root, "tsconfig.json")], this.root, "ui-native-command-types", cmdBudgetMs(), { env: process.env });
  }
}

class CheckWgpuEngineScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await new ScriptRouter(this.root, this.repoRoot).register("wasm", CheckWgpuEngineWasmScript).run(segments);
  }
}

class CheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (!segments.length) {
      await new CheckAxesScript(this.root, this.repoRoot).run();
      return;
    }
    await new ScriptRouter(this.root, this.repoRoot).register("wasm", CheckWasmScript).register("wgpu-engine", CheckWgpuEngineScript).register("commands", CheckCommandTypesScript).run(segments);
  }
}

/** 🧭️ Supplies the native UI package's actual command owner tree. */
export function createUiNativeRouter(root = import.meta.dir, repoRoot?: string): ScriptRouter {
  return new ScriptRouter(root, repoRoot).register("generate", GenerateAxesScript).register("preview-generated", PreviewGeneratedScript).register("check", CheckScript).register("test", TestScript);
}

if (import.meta.main) await runScriptMain(createUiNativeRouter());
