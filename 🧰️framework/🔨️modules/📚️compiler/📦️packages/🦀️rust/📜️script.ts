#!/usr/bin/env repo
import{createIssuedResourceControlV1}from"../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { receiveScriptProcessInvocation } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { resolve } from "node:path";
import { BundleScript, ScriptRouter } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { readCargoTestPolicyV1, runAdmittedCargoTestsV1 as runCargoTestsV1 } from "../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts";
import { resolveTestLevel } from "../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
/** 📚️ Executes the actual compiler package and all original syntax laws under admitted native policy. */
class NativeScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const cargoPhase=createIssuedResourceControlV1(this.invocation,"cargo:script-entry",0);
    const { rest } = resolveTestLevel(segments);
    await runCargoTestsV1({ manifestPath: resolve(this.root, "Cargo.toml"), packages: ["semio-framework-compiler"], cwd: this.root, extraArgs: rest.length ? rest : ["--lib"] }, readCargoTestPolicyV1(process.env), this.repoRoot, this.invocation,cargoPhase);
  }
}
await receiveScriptProcessInvocation(process.env, original => runScriptMain(new ScriptRouter(import.meta.dirname).register("test-native", NativeScript), { invocation: original, ...({ defaultCommand: "test-native" }) }));
