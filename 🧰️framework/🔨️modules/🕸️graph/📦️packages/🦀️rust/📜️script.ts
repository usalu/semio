#!/usr/bin/env repo
import { receiveScriptProcessInvocation } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
/** 📜️ `@semio-tech/framework-graph` task router. */
import { BundleScript, ScriptRouter } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { CheckGeneratedScript, GenerateScript, LintScript, ManifestContractScript, PreviewGeneratedScript, TestScript } from "../../🛂️manifest/🏃️execution/🟦️.ts";
import { runNativeTestScript } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";
import { isMainModuleV1 } from "../../../⚙️runtime/🏁️main/🟦️.ts";

/** 🦀️ Runs the complete default Graph native library cohort through its managed owner. */
class NativeTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runNativeTestScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}

const router = new ScriptRouter(import.meta.dirname)
  .register("generate", GenerateScript)
  .register("preview-generated", PreviewGeneratedScript)
  .register("check-generated", CheckGeneratedScript)
  .register("test", TestScript)
  .register("test-native", NativeTestScript)
  .register("test-manifest-contract", ManifestContractScript)
  .register("lint", LintScript);

if (isMainModuleV1(import.meta.url)) await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, defaultCommand: "generate" }));
