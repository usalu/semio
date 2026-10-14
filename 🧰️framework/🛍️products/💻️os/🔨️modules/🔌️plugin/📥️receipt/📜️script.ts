#!/usr/bin/env bun
import { resolve } from "node:path";
import { BundleScript, ScriptRouter, advanceScriptInvocation, receiveScriptProcessInvocation, runScriptMain, scriptInvocationBudget } from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runOwnedCommand } from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { runVitestV1, type VitestPolicyV1 } from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧪️vitest/🟦️.ts";
import { testLevelBudgetMs } from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";

/** 🧾️ Owns the whole existing OS loader liveness and receipt refusal corpus without loading repository command policy. */
class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw Error("test accepts no arguments");
    const artifacts = process.env.SEMIO_TEST_ARTIFACT_DIR;
    if (!artifacts) throw Error("Package receipt tests require explicit SEMIO_TEST_ARTIFACT_DIR");
    const cwd = resolve(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript");
    const policy: VitestPolicyV1 = { version: 1, cwd, toolPath: resolve(this.repoRoot, "node_modules/vitest/vitest.mjs"), runtime: "node", coverageRuntime: "node", cacheRoot: resolve(artifacts, "vite"), coverageDirectory: resolve(artifacts, "coverage"), budgetMs: scriptInvocationBudget(this.invocation, testLevelBudgetMs("long")) };
    const environment = { ...process.env, SEMIO_TEST_LEVEL: "long", SEMIO_VITEST_POLICY: JSON.stringify(policy) };
    const options = { signal: this.invocation.control.signal, onProgress: () => advanceScriptInvocation(this.invocation, "package-receipt", "running") };
    await advanceScriptInvocation(this.invocation, "package-receipt:types", "running");
    await runOwnedCommand(process.execPath, [Bun.resolveSync("typescript/bin/tsc", this.repoRoot), "--noEmit", "--strict", "--noUncheckedIndexedAccess", "--skipLibCheck", "--allowImportingTsExtensions", "--resolveJsonModule", "--esModuleInterop", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--types", "bun", resolve(this.root, "🟦️.ts")], this.repoRoot, "os:package-receipt:types", scriptInvocationBudget(this.invocation, 30000), options);
    await runOwnedCommand(process.execPath, ["test", resolve(this.root, "../🖨️describe/🏷️component-receipt/🧪️tests/🟦️.ts")], this.repoRoot, "os:component-receipt:laws", scriptInvocationBudget(this.invocation, testLevelBudgetMs("long")), options);
    await advanceScriptInvocation(this.invocation, "package-receipt:laws", "running");
    const receivingPolicy = { ...policy, budgetMs: scriptInvocationBudget(this.invocation, testLevelBudgetMs("long")) };
    await runVitestV1(receivingPolicy, ["../../../../🧪️tests/🫀️plugin-load-progress/🟦️.ts", "--silent=false", "--reporter=verbose"], "../../🧪️tests/🎚️config/🟦️.ts", { ...environment, SEMIO_VITEST_POLICY: JSON.stringify(receivingPolicy) }, this.invocation.control.signal);
    await advanceScriptInvocation(this.invocation, "package-receipt:laws", "complete");
    console.log("[DEBUG] Existing whole OS loader liveness and receipt refusal corpus completed");
  }
}

if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => runScriptMain(new ScriptRouter(import.meta.dir).register("test", TestScript), { invocation: original }));
