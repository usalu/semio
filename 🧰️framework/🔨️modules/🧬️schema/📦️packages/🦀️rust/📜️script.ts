#!/usr/bin/env bun
/** 📜️ `@semio-tech/framework-schema` task router. */
import { resolve } from "node:path";
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runCargoTestBudgeted, runCmd } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { CheckScript, GenerateScript, PreviewGeneratedScript } from "../../🏷️entity-kinds/🏃️execution/🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestBudgeted(["semio-framework-schema"], this.repoRoot, rest);
    runCmd(process.execPath, ["test", resolve(this.root, "../../🧪️tests/🩹️fragment-validation-oracle/🟦️.ts")], { cwd: this.repoRoot });
  }
}

const router = new ScriptRouter(import.meta.dir)
  .register("generate", GenerateScript)
  .register("preview-generated", PreviewGeneratedScript)
  .register("check", CheckScript)
  .register("test", TestScript);

if (import.meta.main) await runBundleScriptMain(router, import.meta.url, { defaultCommand: "generate" });
