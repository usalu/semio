#!/usr/bin/env bun
/** 🖥️ `semio-framework-pack` task router: `bun ./📜️script.ts test [quick|long|exhaustive] [args…]`. */
import { resolveTestLevel, runCargo } from "../../../../🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

import { buildCargoArtifacts } from "../../../../🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts";

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runCargo(["test", "-p", "semio-framework-pack", ...rest], this.repoRoot);
  }
}

class BuildScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await buildCargoArtifacts(`${this.root}/Cargo.toml`, segments, this.repoRoot);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("build", BuildScript);

await runScriptMain(router, { defaultCommand: "test" });
