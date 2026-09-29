#!/usr/bin/env bun
/** ❓️ `semio-framework-quiz` task router: `bun ./📜️script.ts test [quick|long|exhaustive] [args…]` and `bun ./📜️script.ts build [args…]`. */
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runCargoTestBudgeted } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

import { buildCargoArtifacts } from "../../../🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts";

/** 🧪️ Runs the crate's unit and fixture suites at the requested level. */
class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestBudgeted(["semio-framework-quiz"], this.repoRoot, rest);
  }
}

/** 🏗️ Builds the crate's cargo artifacts. */
class BuildScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await buildCargoArtifacts(`${this.root}/Cargo.toml`, segments, this.repoRoot);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("build", BuildScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
