#!/usr/bin/env bun
/** 🔲️ The canonical native Pixels task owner retains the complete original package workload. */
import { resolve } from "node:path";
import { runCargoTestsV1, readCargoTestPolicyV1 } from "../../../🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runCargoTestsV1({ manifestPath: resolve(this.root,"Cargo.toml"),packages:["semio-framework-pixels","semio-framework-deflate","semio-framework-io-base64","semio-framework-io-binary-source"],cwd:resolve(this.root,"../.."),extraArgs:segments },readCargoTestPolicyV1(process.env));
  }
}

await runScriptMain(new ScriptRouter(import.meta.dir).register("test",TestScript),{defaultCommand:"test"});
