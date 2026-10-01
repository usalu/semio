#!/usr/bin/env bun
/** 🧭️ `@semio-tech/repo-test` router: `bun ./📜️script.ts <lint|test [level]>`. */
import { join } from "node:path";
import { repoTestArtifactEnvironment } from "../../../📚️library/🏃️process/🌿️environment/🧪️test-output/🟦️.ts";
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runBunx, runTestBudgeted } from "../../../📚️library/📦️packages/🟦️typescript/🟦️.ts";

class LintScript extends BundleScript {
  run(): void {
    runBunx(["tsc", "-p", "tsconfig.json", "--noEmit"], this.root);
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments[0] === "discovery-boundaries") {
      if (segments.length !== 1) throw new Error("Expected test discovery-boundaries");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🚷️discovery-boundaries/🟦️.ts");
      await runTestBudgeted(process.execPath, ["test", source], { cwd: this.repoRoot, env: repoTestArtifactEnvironment(this.repoRoot, "discovery-boundaries"), budgetMs: 30_000 });
      return;
    }
    const { rest } = resolveTestLevel(segments);
    await runTestBudgeted(process.execPath, ["test", "../../🧪️tests/🚷️discovery-boundaries/🟦️.ts", "../../🧪️tests/🧪️test-platform/🟦️.ts", "../../🧪️tests/🧬️schema-invariants/🟦️.ts", "../../🧪️tests/📐️test-layout/🟦️.ts", "../../🧪️tests/🧭️fixture-resolution/🟦️.ts", "../../🧪️tests/🧬️mutation-fixtures/🟦️.ts", ...rest], { cwd: this.root });
  }
}

const router = new ScriptRouter(import.meta.dir).register("lint", LintScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url);
