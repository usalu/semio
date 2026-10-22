#!/usr/bin/env repo
import{scriptCompilerAdmissionControlV1}from"../../../../🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import{createIssuedResourceControlV1}from"../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { receiveScriptProcessInvocation } from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { resolve } from "node:path";
import { readCargoTestPolicyV1, runAdmittedCargoCommandV1, runAdmittedCargoTestsV1 as runCargoTestsV1 } from "../../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts";
import { runBudgetedTestCommand } from "../../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts";
import { buildBudgetMs } from "../../../../🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";

import { BundleScript, ScriptRouter } from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { isMainModuleV1 } from "../../../../⚙️runtime/🏁️main/🟦️.ts";

/** 💾️ Runs every resident library assertion through the shared native budget. */
class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const cargoPhase=createIssuedResourceControlV1(this.invocation,"cargo:script-entry",0);
    if (segments.length) throw new Error("Resident native test accepts no arguments");
    await runCargoTestsV1({ manifestPath: resolve(this.root, "Cargo.toml"), packages: ["semio-framework-value-resident"], cwd: this.root, extraArgs: ["--lib"] }, readCargoTestPolicyV1(process.env), this.repoRoot, this.invocation,cargoPhase);
  }
}

/** 🌐️ Checks both toolchain-declared guest targets without parallel Cargo work. */
class CheckWasmScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("Resident Wasm check accepts no arguments");
    for (const target of ["wasm32-wasip2", "wasm32-unknown-unknown"]) {
      await runAdmittedCargoCommandV1(this.invocation,scriptCompilerAdmissionControlV1(this.invocation,"cargo:caller",0),this.repoRoot,["check", "--manifest-path", "Cargo.toml", "--lib", "--target", target],this.root,process.env,buildBudgetMs());
    }
  }
}

const router = new ScriptRouter(import.meta.dirname).register("test", TestScript).register("check-wasm", CheckWasmScript);
if (isMainModuleV1(import.meta.url)) await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original }));
