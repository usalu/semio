#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import { resolve } from "node:path";
import { BundleScript, ScriptRouter } from "../../../🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { runCargoTestsV1, readCargoTestPolicyV1 } from "../../../🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import { resolveTestLevel } from "../../../🏃️process/🧪️testing/🎚️budget/🟦️.ts";
/** 📚️ Executes the actual compiler package and all original syntax laws under admitted native policy. */
class NativeScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestsV1({ manifestPath: resolve(this.root, "Cargo.toml"), packages: ["semio-framework-compiler"], cwd: this.root, extraArgs: rest.length ? rest : ["--lib"] }, readCargoTestPolicyV1(process.env));
  }
}
await receiveScriptProcessInvocation(process.env, original => runScriptMain(new ScriptRouter(import.meta.dir).register("test-native", NativeScript), { invocation: original, ...({ defaultCommand: "test-native" }) }));
