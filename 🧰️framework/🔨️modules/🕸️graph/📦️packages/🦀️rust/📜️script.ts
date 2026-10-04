#!/usr/bin/env bun
/** 📜️ `@semio-tech/framework-graph` task router. */
import { BundleScript, ScriptRouter } from "../../../🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { CheckGeneratedScript, GenerateScript, LintScript, ManifestContractScript, PreviewGeneratedScript, TestScript } from "../../🛂️manifest/🏃️execution/🟦️.ts";

import { resolve } from "node:path";

/** 🦀️ Runs the complete default Graph native library cohort through its managed owner. */
class NativeTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("test-native accepts no arguments");
    const {runCargoTestsV1,readCargoTestPolicyV1} = await import("../../../🏃️process/🧪️testing/🦀️cargo/🟦️.ts");
    await runCargoTestsV1({manifestPath:resolve(this.root,"Cargo.toml"),packages:["semio-framework-graph"],cwd:this.root,extraArgs:["--lib","--no-fail-fast"]},readCargoTestPolicyV1(process.env));
  }
}

const router = new ScriptRouter(import.meta.dir)
  .register("generate", GenerateScript)
  .register("preview-generated", PreviewGeneratedScript)
  .register("check-generated", CheckGeneratedScript)
  .register("test", TestScript)
  .register("test-native", NativeTestScript)
  .register("test-manifest-contract", ManifestContractScript)
  .register("lint", LintScript);

if (import.meta.main) await runScriptMain(router, { defaultCommand: "generate" });
