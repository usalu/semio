#!/usr/bin/env bun
/** 🖥️ `semio-framework-replication` task router: `bun ./📜️script.ts test [quick|long|exhaustive] [args…]`. */
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runCargo } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargo(["test", "-p", "semio-framework-replication", ...rest], this.repoRoot);
  }
}

class BuildScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runCargo(["build", "-p", "semio-framework-replication", ...segments], this.repoRoot);
  }
}

class SourceTestScript extends BundleScript {
  async run(): Promise<void> {
    await import("../../../🌱️value/🗂️ordered/🧪️fixtures/📜️script.ts");
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("build", BuildScript).register("test-source", SourceTestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
