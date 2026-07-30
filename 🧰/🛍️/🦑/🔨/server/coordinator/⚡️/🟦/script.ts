#!/usr/bin/env bun
/** 🧭 Coordinator package router: `bun ./script.ts build|test|policy`. */
import type { BundleLinter } from "../../../../../../../🔨/math/⚡️/🟦/graph/dsl/core/js/📦.ts";
import { dependencyBoundaryBreachesForBundleDir } from "../../../../../../../🔨/math/⚡️/🟦/graph/dsl/core/js/📦.ts";
import { getWorkspaceRoot } from "../../../../../../../🔨/math/⚡️/🟦/graph/dsl/core/js/📦.ts";
import { BundleScript, ScriptRouter, runBundleScriptMain, runVitest, resolveTestLevel, runCmd } from "../../../../../../../🔨/math/⚡️/🟦/graph/dsl/core/js/📦.ts";
import { defineLint } from "../../../../../../../🔨/math/⚡️/🟦/graph/dsl/core/js/📦.ts";

export const policy = defineLint("@repo/server/coordinator-bundle", (l: BundleLinter) => {
  const repoRoot = getWorkspaceRoot();
  return dependencyBoundaryBreachesForBundleDir(repoRoot, l.root());
});

class BuildScript extends BundleScript {
  run(): void {
    const ext = process.platform === "win32" ? ".exe" : "";
    runCmd("go", ["build", "-o", `server${ext}`, "."], { cwd: this.root });
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runVitest(this.root, rest, "js/vitest.config.ts");
  }
}

const router = new ScriptRouter(import.meta.dir).register("build", BuildScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "build" });
