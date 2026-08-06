#!/usr/bin/env bun
/** @emoji ⚙️ Delegates styling generation and Python import smoke test. */
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runTestBudgeted } from "../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️lib/📦️packages/🟦️typescript/📦️index.ts";
import { generateStylingArtifacts } from "../../../🎨️styling/📦️packages/🦀️rust/📜️script.ts";

class GenerateScript extends BundleScript {
  run(): void {
    generateStylingArtifacts();
  }
}

class TestScript extends BundleScript {
  run(segments: string[]): void {
    resolveTestLevel(segments);
    runTestBudgeted("uv", ["run", "python", "-c", "from styling.generated import BOARD_LIGHT, STYLING_TOKENS; assert STYLING_TOKENS['primary']"], { cwd: import.meta.dir });
  }
}

if (import.meta.main) {
  const router = new ScriptRouter(import.meta.dir).register("generate", GenerateScript).register("test", TestScript);
  await runBundleScriptMain(router, import.meta.url);
}
