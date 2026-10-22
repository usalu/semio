#!/usr/bin/env repo
import{createIssuedResourceControlV1}from"../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { receiveScriptProcessInvocation } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { resolve } from "node:path";
import { BundleScript, ScriptRouter } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { readCargoTestPolicyV1, runAdmittedCargoTestsV1 as runCargoTestsV1 } from "../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts";
import { resolveTestLevel } from "../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { runReferenceScript, runOwnershipScript } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";

/** 🗣️ Runs every neutral selection, lexical, diagnostic and literal reference law. */
class ReferenceScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runReferenceScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}

/** 🚮️ Runs the full neutral reference corpus and its physically absent product projection. */
class OwnershipScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runOwnershipScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}

/** 🗣️ Executes the complete actual neutral lexical, grammar, literal and idiom cohorts. */
class NativeScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const cargoPhase=createIssuedResourceControlV1(this.invocation,"cargo:script-entry",0);
    const { rest } = resolveTestLevel(segments);
    await runCargoTestsV1({ manifestPath: resolve(this.root, "Cargo.toml"), packages: ["semio-framework-dsl"], cwd: this.root, extraArgs: rest }, readCargoTestPolicyV1(process.env), this.repoRoot, this.invocation,cargoPhase);
  }
}

const router = new ScriptRouter(import.meta.dirname).register("test-reference", ReferenceScript).register("test-ownership", OwnershipScript).register("test-native", NativeScript);
await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({ defaultCommand: "test-ownership" }) }));
