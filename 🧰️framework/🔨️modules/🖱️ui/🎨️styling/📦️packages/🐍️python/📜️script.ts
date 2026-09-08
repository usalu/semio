#!/usr/bin/env bun
/** @emoji ⚙️ Delegates styling generation and Python import smoke test. */
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runCmd, runTestBudgeted } from "../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

/** 📥️ Synchronizes this separately locked Python environment without running application work. */
class DepsScript extends BundleScript {
  run(): void {
    runCmd("uv", ["sync", "--locked", "--project", import.meta.dir], { cwd: import.meta.dir });
  }
}

class GenerateScript extends BundleScript {
  run(): void {
    console.log("[nx-generate] styling artifacts ready");
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    resolveTestLevel(segments);
    await runTestBudgeted("uv", ["run", "--locked", "--no-sync", "python", "-c", "from importlib import import_module; styling = import_module('🎨️styling.🐍️'); assert styling.BOARD_LIGHT; assert styling.STYLING_TOKENS['primary']"], { cwd: import.meta.dir });
  }
}

if (import.meta.main) {
  const router = new ScriptRouter(import.meta.dir).register("deps", DepsScript).register("generate", GenerateScript).register("test", TestScript);
  await runBundleScriptMain(router, import.meta.url);
}
