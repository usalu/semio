#!/usr/bin/env bun
import { BundleScript, ScriptRouter, buildBudgetMs, resolveTestLevel, runBundleScriptMain, runCargoTestBudgeted, runTestBudgeted } from "@semio-tech/repo-lib";

/** 🎟️ Runs the input oracle or delegates native filters to the shared budget. */
class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments[0] === "source") {
      if (segments.length !== 1) throw new Error("UI-host source test accepts no arguments");
      const { testInputAdmissionFixture } = await import("../../📥️input/🎟️admission/📜️script.ts");
      testInputAdmissionFixture();
      return;
    }
    const { rest } = resolveTestLevel(segments);
    await runCargoTestBudgeted(["semio-framework-ui-host"], this.repoRoot, ["--no-fail-fast", ...rest]);
  }
}

/** 🖥️ Checks the native host library through the shared build budget. */
class CheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("UI-host native check accepts no arguments");
    await runTestBudgeted("cargo", ["check", "-p", "semio-framework-ui-host", "--lib"], { cwd: this.repoRoot, budgetMs: buildBudgetMs() });
  }
}

/** 🌐️ Checks the browser host target through the shared build budget. */
class CheckWasmScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("UI-host Wasm check accepts no arguments");
    await runTestBudgeted("cargo", ["check", "-p", "semio-framework-ui-host", "--lib", "--target", "wasm32-unknown-unknown"], { cwd: this.repoRoot, budgetMs: buildBudgetMs() });
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("check", CheckScript).register("check-wasm", CheckWasmScript);
if (import.meta.main) await runBundleScriptMain(router, import.meta.url);
