#!/usr/bin/env repo
import { mkdirSync } from "node:fs";
import { resolve } from "node:path";
import { BundleScript, ScriptRouter, receiveScriptProcessInvocation, runScriptMain, scriptInvocationBudget } from "../../📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runOwnedCommand } from "../../📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { TEST_LEVEL_BUDGET_MS } from "../../📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { isMainModuleV1 } from "../../../⚙️runtime/🏁️main/🟦️.ts";

/** 🎚️ Runs every owned build-mode law under the original process authority. */
class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length !== 0) throw new Error("Expected test");
    if (!process.env.SEMIO_TEST_ARTIFACT_DIR) throw new Error("Process contract tests require explicit SEMIO_TEST_ARTIFACT_DIR");
    const output = resolve(process.env.SEMIO_TEST_ARTIFACT_DIR);
    mkdirSync(output, { recursive: true });
    await runOwnedCommand(process.execPath, ["test", resolve(this.root, "🧪️tests/🟦️.ts")], this.repoRoot, "process:build-mode", scriptInvocationBudget(this.invocation, TEST_LEVEL_BUDGET_MS.fundamental), { env: { ...process.env, SEMIO_TEST_ARTIFACT_DIR: output }, signal: this.invocation.control.signal });
  }
}

if (isMainModuleV1(import.meta.url)) await receiveScriptProcessInvocation(process.env, original => runScriptMain(new ScriptRouter(import.meta.dirname, resolve(import.meta.dirname, "../../../..")).register("test", TestScript), { invocation: original }));
