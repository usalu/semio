#!/usr/bin/env bun
/** 🧭️ `@semio-tech/repo-lib` router: `bun ./📜️script.ts <lint|test [level]>`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runBunx, resolveTestLevel, runTestBudgeted } from "../../../../../../../🧰️framework/🛍️product/🦑️repo/🔨️module/📚️lib/⚡️implementation/🟦️typescript/📦️index.ts";

class LintScript extends BundleScript {
  run(): void {
    runBunx(["tsc", "-p", "tsconfig.json", "--noEmit"], this.root);
  }
}

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runTestBudgeted(process.execPath, ["test", "./🧪️index.test.ts", ...rest], { cwd: this.root });
  }
}

const router = new ScriptRouter(import.meta.dir).register("lint", LintScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url);
