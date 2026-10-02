#!/usr/bin/env bun
import { resolveTestLevel } from "../../../🏃️process/🧪️testing/🎚️budget/🟦️.ts";
/** 🖥️ `semio-framework-pack` task router: `bun ./📜️script.ts test [quick|long|exhaustive] [args…]`. */
import { resolve } from "node:path";
import { runCargoTestsV1, readCargoTestPolicyV1 } from "../../../🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

import { buildCargoArtifacts , readCargoArtifactBuildPolicyV1 } from "../../../🏃️process/📦️artifacts/🏗️native-build/🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestsV1({ manifestPath: resolve(this.root, "Cargo.toml"), packages: ["semio-framework-pack"], cwd: this.root, extraArgs: rest }, readCargoTestPolicyV1(process.env));
  }
}

class BuildScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await buildCargoArtifacts(`${this.root}/Cargo.toml`, segments, readCargoArtifactBuildPolicyV1(process.env,this.root));
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("build", BuildScript);

await runScriptMain(router, { defaultCommand: "test" });
