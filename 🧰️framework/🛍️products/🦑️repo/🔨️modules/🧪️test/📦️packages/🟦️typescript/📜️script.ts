#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import { resolveTestLevel } from "../../../../../../🔨️modules/🏃️process/🧪️testing/🎚️budget/🟦️.ts";
/** 🧭️ `@semio-tech/repo-test` router: `bun ./📜️script.ts <lint|test [level]>`. */
import { join } from "node:path";
import { repoTestArtifactEnvironment } from "../../../📚️library/🏃️process/🌿️environment/🧪️test-output/🟦️.ts";
import { runBunx, runRepositoryTestCommand } from "../../../📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

class LintScript extends BundleScript {
  run(): void {
    runBunx(["tsc", "-p", "tsconfig.json", "--noEmit"], this.root);
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments[0] === "fixture-isolation") {
      if (segments.length !== 1) throw new Error("Expected test fixture-isolation");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧬️schema-invariants/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source, "-t", "placement, ownership and fixture isolation|schema protocol|module-member carve-out|repository that declares no submodules|every generator case"], { cwd: this.repoRoot, env: repoTestArtifactEnvironment(this.repoRoot, "fixture-isolation"), budgetMs: 30_000 });
      return;
    }
    if (segments[0] === "discovery-boundaries") {
      if (segments.length !== 1) throw new Error("Expected test discovery-boundaries");
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🚷️discovery-boundaries/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source], { cwd: this.repoRoot, env: repoTestArtifactEnvironment(this.repoRoot, "discovery-boundaries"), budgetMs: 30_000 });
      return;
    }
    const { rest } = resolveTestLevel(segments);
    await runRepositoryTestCommand(process.execPath, ["test", "../../🧪️tests/🚷️discovery-boundaries/🟦️.ts", "../../🧪️tests/🧪️test-platform/🟦️.ts", "../../🧪️tests/🧬️schema-invariants/🟦️.ts", "../../🧪️tests/📐️test-layout/🟦️.ts", "../../🧪️tests/🧭️fixture-resolution/🟦️.ts", "../../🧪️tests/🧬️mutation-fixtures/🟦️.ts", ...rest], { cwd: this.root });
  }
}

const router = new ScriptRouter(import.meta.dir).register("lint", LintScript).register("test", TestScript);

await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original }));
