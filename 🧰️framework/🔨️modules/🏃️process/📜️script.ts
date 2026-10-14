#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "./🧭️routing/📥️invocation/🏃️process/🟦️.ts";
/** 🏃️ Runs neutral process ownership and execution contracts. */
import { resolve } from "node:path";
import { mkdirSync } from "node:fs";
import { runOwnedCommand } from "./🎛️owned-execution/🟦️.ts";
import { TEST_LEVEL_BUDGET_MS } from "./🧪️testing/🎚️budget/🟦️.ts";
import { BundleScript, ScriptRouter, scriptInvocationBudget } from "./🧭️routing/🟦️.ts";
import { runScriptMain } from "./🧭️routing/🚪️entrypoint/🟦️.ts";
import {executeCommandV1} from "./🧭️routing/🎛️command/🟦️.ts";
import {parseCommandArgumentsV1,resolveCommandConfigurationV1} from "./🧭️routing/🎛️command/⚙️configuration/🟦️.ts";

/** 🎛️ Runs a configured neutral command with exact context and artifact custody. */
class CommandScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
  const plan=resolveCommandConfigurationV1(parseCommandArgumentsV1(segments),process.env);
  const result=await executeCommandV1(plan.request,plan.policy,{invocation:this.invocation,environment:plan.environment,onProgress:event=>{process.stderr.write(`[DEBUG] General command ${event.phase} elapsedMs=${event.elapsedMs}\n`);}});process.stdout.write(result.stdout);process.stderr.write(result.stderr);process.exitCode=result.reason==="exit"?result.status??1:1;console.error(`[DEBUG] General command receipt=${result.receiptPath} reason=${result.reason} status=${result.status}`);
 }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length !== 1) throw Error("Expected one process contract suite");
    if (!process.env.SEMIO_TEST_ARTIFACT_DIR) throw Error("Process contract tests require explicit SEMIO_TEST_ARTIFACT_DIR");
    const output = resolve(process.env.SEMIO_TEST_ARTIFACT_DIR);
    mkdirSync(output, { recursive: true });
    const functionSuites: Readonly<Record<string, readonly [string, string]>> = {
      "resource-leases": ["🔒️leases/🧪️tests/🔒️resource-leases/🟦️.ts", "testResourceLeases"],
      "cargo-build-lease": ["📦️artifacts/🏗️native-build/🔒️lease/🧪️tests/🟦️.ts", "proveCargoBuildLeasesV1"],
      "native-artifact-publication": ["📦️artifacts/🏗️native-build/🧪️tests/📦️publication/🟦️.ts", "testCargoArtifactPublication"],
    };
    const functionSuite = functionSuites[segments[0]!];
    if (functionSuite) {
      await runOwnedCommand(process.execPath,["-e","const suite=await import(process.argv[1]);await suite[process.argv[2]](process.argv[3]);",resolve(this.root,functionSuite[0]),functionSuite[1],output],this.repoRoot,"process:"+segments[0],scriptInvocationBudget(this.invocation,TEST_LEVEL_BUDGET_MS.fundamental),{signal:this.invocation.control.signal});return;
    }
    if(segments[0]==="artifact-publication"){await (await import("./📦️artifacts/📤️publication/🧪️tests/🟦️.ts")).testArtifactPublication(output);return;}
    if (segments[0] === "artifact-files") {
      const { testArtifactFiles } = await import("./📦️artifacts/🗂️files/🧪️tests/🟦️.ts");
      await testArtifactFiles(output);
      return;
    }
    const suites: Readonly<Record<string, { source: string; budgetMs: number; testNamePattern?: string }>> = {
      "compiler-state": { source: "🏗️build/🟦️typescript/🧪️tests/🟦️.ts", budgetMs: TEST_LEVEL_BUDGET_MS.quick },
      capture: { source: "📥️capture/🧪️tests/🟦️.ts", budgetMs: TEST_LEVEL_BUDGET_MS.fundamental },
      command: { source: "🧭️routing/🎛️command/🧪️tests/🟦️.ts", budgetMs: TEST_LEVEL_BUDGET_MS.quick },
      "command-boundary": { source: "🧭️routing/🎛️command/🧪️tests/🚧️boundary/🟦️.ts", budgetMs: TEST_LEVEL_BUDGET_MS.quick },
      "command-cold": { source: "🧭️routing/🎛️command/🧪️tests/❄️cold/🟦️.ts", budgetMs: TEST_LEVEL_BUDGET_MS.quick },
      "managed-build-mode": { source: "🏗️build/🧪️tests/🏗️managed/🟦️.ts", budgetMs: TEST_LEVEL_BUDGET_MS.quick },
      "cargo-profile-policy": {source:"🏗️build/🧪️tests/🏗️native/🟦️.ts",budgetMs:TEST_LEVEL_BUDGET_MS.quick,testNamePattern:"Central Cargo profile policy"},
      "native-build-mode": { source: "🏗️build/🧪️tests/🏗️native/🟦️.ts", budgetMs: TEST_LEVEL_BUDGET_MS.quick },
      "go-build-mode": { source: "📦️artifacts/🐹️go-build/🧪️tests/🟦️.ts", budgetMs: TEST_LEVEL_BUDGET_MS.quick },
      "compiler-capacity": { source: "🏗️build/🎟️capacity/🧪️tests/🟦️.ts", budgetMs: TEST_LEVEL_BUDGET_MS.quick },
      "native-queued-cancellation": { source: "📦️artifacts/🏗️native-build/🧩️component-pass/🧪️tests/🟦️.ts", budgetMs: TEST_LEVEL_BUDGET_MS.quick, testNamePattern: "queued exclusive compiler cancellation|unit graph compilation cancellation|native build plan is schema-first|profile mode selection|unlaned same-profile Cargo jobs|the original finite caller budget" },
      "native-artifacts": { source: "📦️artifacts/🏗️native-build/🧪️tests/🟦️.ts", budgetMs: TEST_LEVEL_BUDGET_MS.quick },
      "exact-cargo-laws": { source: "🧪️testing/🦀️cargo/🎯️exact/🧪️tests/🟦️.ts", budgetMs: TEST_LEVEL_BUDGET_MS.fundamental },
      "owner-context": { source: "📋️context/🧪️tests/🟦️.ts", budgetMs: TEST_LEVEL_BUDGET_MS.fundamental },
      "vitest-driver": { source: "🧪️testing/🧪️vitest/🧪️tests/🟦️.ts", budgetMs: TEST_LEVEL_BUDGET_MS.fundamental },
      "cargo-test-leases": {source:"🧪️testing/🦀️cargo/🔒️lease/🧪️tests/🟦️.ts",budgetMs:TEST_LEVEL_BUDGET_MS.quick},
      "cargo-profile": { source: "🧪️testing/🦀️cargo/🧪️tests/🟦️.ts", budgetMs: TEST_LEVEL_BUDGET_MS.quick, testNamePattern: "Cargo compiles the exact libtest profile" },
      "cargo-driver": { source: "🧪️testing/🦀️cargo/🧪️tests/🟦️.ts", budgetMs: TEST_LEVEL_BUDGET_MS.quick },
      "test-command": { source: "🧪️testing/🎛️execution/🧪️tests/🟦️.ts", budgetMs: TEST_LEVEL_BUDGET_MS.fundamental },
      routing: { source: "🧭️routing/🧪️tests/🟦️.ts", budgetMs: TEST_LEVEL_BUDGET_MS.fundamental },
      "execution-budget": { source: "⏱️budget/🧪️tests/🟦️.ts", budgetMs: TEST_LEVEL_BUDGET_MS.fundamental },
      budget: { source: "🧪️testing/🎚️budget/🧪️tests/🟦️.ts", budgetMs: TEST_LEVEL_BUDGET_MS.fundamental },
      "owned-execution": { source: "🎛️owned-execution/🧪️tests/🟦️.ts", budgetMs: 120_000 },
      "process-tree-termination": { source: "🪓️termination/🧪️tests/🟦️.ts", budgetMs: 120_000 },
    };
    const suite = suites[segments[0]!];
    if (!suite) throw Error(`Unknown process contract suite: ${segments[0]}`);
    const sources = [resolve(this.root, suite.source), ...(segments[0] === "go-build-mode" ? [resolve(this.root,"../📁️filesystem/🛡️durability/🧪️tests/🟦️.ts")] : []), ...(segments[0] === "capture" ? [resolve(this.root, "📥️capture/📥️invocation/🧪️tests/🟦️.ts")] : []), ...(segments[0] === "exact-cargo-laws" ? [resolve(this.root, "🧪️testing/🦀️cargo/🎯️exact/📥️invocation/🧪️tests/🟦️.ts")] : []), ...(segments[0] === "native-artifacts" ? ["🧾️receipt", "🎰️jobserver", "🎟️admission", "📈️telemetry", "🗃️stage-cache", "🧩️component-pass"].map((facet) => resolve(this.root, `📦️artifacts/🏗️native-build/${facet}/🧪️tests/🟦️.ts`)) : []), ...(segments[0] === "owned-execution" ? [resolve(this.root, "🎛️owned-execution/🧪️tests/📤️stdout/🟦️.ts"),resolve(this.root, "🎛️owned-execution/🧪️tests/📬️output/🟦️.ts")] : [])];
    if (["routing", "command", "capture", "compiler-state"].includes(segments[0]!)) await runOwnedCommand(process.execPath, [Bun.resolveSync("typescript/bin/tsc", this.root), "--noEmit", "--strict", ...(segments[0] === "compiler-state" ? ["--noUncheckedIndexedAccess"] : []), "--skipLibCheck", "--allowImportingTsExtensions", "--resolveJsonModule", "--esModuleInterop", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--types", "bun", ...sources], this.repoRoot, "process:" + segments[0] + ":types", scriptInvocationBudget(this.invocation, suite.budgetMs), { signal: this.invocation.control.signal });
    await runOwnedCommand(process.execPath, ["test", ...sources, ...(suite.testNamePattern ? ["--test-name-pattern", suite.testNamePattern] : [])], this.repoRoot, `process:${segments[0]}`, scriptInvocationBudget(this.invocation, suite.budgetMs), { env: { ...process.env, SEMIO_TEST_ARTIFACT_DIR: output }, signal: this.invocation.control.signal });
  }
}

if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => runScriptMain(new ScriptRouter(import.meta.dir,resolve(import.meta.dir,"../..")).register("command",CommandScript).register("test", TestScript), { invocation: original }));
