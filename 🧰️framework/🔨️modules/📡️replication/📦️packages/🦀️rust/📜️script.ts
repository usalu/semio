#!/usr/bin/env bun
/** 🖥️ `semio-framework-replication` task router: `bun ./📜️script.ts test [quick|long|exhaustive] [args…]`. */
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runCargo } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts";

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runCargo(this.repoRoot, ["test", "-p", "semio-framework-replication", ...rest]);
  }
}

class BuildScript extends BundleScript {
  run(segments: string[]): void {
    runCargo(this.repoRoot, ["build", "-p", "semio-framework-replication", ...segments]);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("build", BuildScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
