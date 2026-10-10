#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import { resolveTestLevel } from "../../../../🔨️modules/🏃️process/🧪️testing/🎚️budget/🟦️.ts";
/** ❓️ `semio-framework-quiz` task router: `bun ./📜️script.ts test [quick|long|exhaustive] [args…]` and `bun ./📜️script.ts build [args…]`. */
import { runRepositoryCargoTests } from "../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

import { buildRepositoryCargoArtifacts } from "../../../🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts";

/** 🧪️ Runs the crate's unit and fixture suites at the requested level. */
class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runRepositoryCargoTests(["semio-framework-quiz"], this.repoRoot, this.invocation.control, rest);
  }
}

/** 🏗️ Builds the crate's cargo artifacts. */
class BuildScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await buildRepositoryCargoArtifacts(`${this.root}/Cargo.toml`, segments, this.repoRoot);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("build", BuildScript);

await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({ defaultCommand: "test" }) }));
