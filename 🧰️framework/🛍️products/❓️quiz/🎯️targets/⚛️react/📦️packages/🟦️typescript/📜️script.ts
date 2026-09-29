#!/usr/bin/env bun
/** ⚛️ `@semio-tech/quiz-react` router: `bun ./📜️script.ts <test [quick|long|exhaustive] | typecheck>`. */
import { join } from "node:path";
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runCmd, runVitest } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments, "quick");
    await runVitest(this.root, rest, "../../🧪️tests/🎚️config/🟦️.ts");
  }
}

class TypecheckScript extends BundleScript {
  run(segments: string[]): void {
    runCmd(process.execPath, [join(this.repoRoot, "node_modules", "typescript", "bin", "tsc"), "--noEmit", "-p", "tsconfig.json", ...segments], { cwd: this.root });
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("typecheck", TypecheckScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
