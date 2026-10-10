#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../../🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import { resolve } from "node:path";
import { runCargoTestsV1, readCargoTestPolicyV1 } from "../../../../🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import { resolveTestLevel } from "../../../../🏃️process/🧪️testing/🎚️budget/🟦️.ts";
import { runBudgetedTestCommand } from "../../../../🏃️process/🧪️testing/🎛️execution/🟦️.ts";
import { buildBudgetMs } from "../../../../🏃️process/⏱️budget/🟦️.ts";

import { BundleScript, ScriptRouter } from "../../../../🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

/** 🎟️ Runs the input oracle or delegates native filters to the shared budget. */
class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments[0] === "source") {
      const scope = segments[1];
      if (segments.length > 2 || (scope && scope !== "input-admission" && scope !== "ordered-scroll")) throw new Error("UI-host source test scope must be input-admission or ordered-scroll");
      if (!scope || scope === "input-admission") {
        const { testInputAdmissionFixture } = await import("../../📥️input/🎟️admission/🧪️tests/🔬️input-admission/🟦️.ts");
        testInputAdmissionFixture();
      }
      if (!scope || scope === "ordered-scroll") {
        const { testOrderedScrollFixture } = await import("../../📥️input/🎡️ordered-scroll/🧪️tests/🔬️ordered-scroll/🟦️.ts");
        testOrderedScrollFixture();
      }
      return;
    }
    const { rest } = resolveTestLevel(segments);
    await runBudgetedTestCommand(process.execPath, ["../../🧪️tests/🌐️browser-host/🟨️.js"], { cwd: this.root, budgetMs: buildBudgetMs() });
    await runCargoTestsV1({ manifestPath: resolve(this.root, "Cargo.toml"), packages: ["semio-framework-ui-host"], cwd: this.root, extraArgs: ["--no-fail-fast", ...rest] }, readCargoTestPolicyV1(process.env));
  }
}

/** 🖥️ Checks the native host library through the shared build budget. */
class CheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("UI-host native check accepts no arguments");
    await runBudgetedTestCommand("cargo", ["check", "--manifest-path", "Cargo.toml", "--lib"], { cwd: this.root, budgetMs: buildBudgetMs() });
  }
}

/** 🌐️ Checks the browser host target through the shared build budget. */
class CheckWasmScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("UI-host Wasm check accepts no arguments");
    await runBudgetedTestCommand("cargo", ["check", "--manifest-path", "Cargo.toml", "--lib", "--target", "wasm32-unknown-unknown"], { cwd: this.root, budgetMs: buildBudgetMs() });
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("check", CheckScript).register("check-wasm", CheckWasmScript);
if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original }));
