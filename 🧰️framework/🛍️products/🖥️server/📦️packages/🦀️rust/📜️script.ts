#!/usr/bin/env bun
/** 🖥️ `semio-framework-server` task router: `bun ./📜️script.ts test [quick|long|exhaustive] [args…]`. */
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runCargo } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runCargo(["test", "-p", "semio-framework-server", ...rest], this.repoRoot);
  }
}

class BuildScript extends BundleScript {
  run(segments: string[]): void {
    runCargo(["build", "-p", "semio-framework-server", ...segments], this.repoRoot);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("build", BuildScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
