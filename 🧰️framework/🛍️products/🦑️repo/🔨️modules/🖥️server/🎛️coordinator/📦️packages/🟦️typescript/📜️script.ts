#!/usr/bin/env bun
/** 🧭️ Coordinator package router: `bun ./📜️script.ts build|test|policy`. */
import type { BundleLinter } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { dependencyBoundaryBreachesForBundleDir, getWorkspaceRoot, BundleScript, ScriptRouter, runBundleScriptMain, runVitest, resolveTestLevel, runCmd, defineLint } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { dirname, join } from "node:path";

export const policy = defineLint("@repo/server/coordinator-bundle", (l: BundleLinter) => {
  const repoRoot = getWorkspaceRoot();
  return dependencyBoundaryBreachesForBundleDir(repoRoot, l.root());
});

class BuildScript extends BundleScript {
  run(): void {
    const ext = process.platform === "win32" ? ".exe" : "";
    const ownerRoot = join(dirname(import.meta.dir), "..");
    runCmd("go", ["build", "-o", `server${ext}`, "."], { cwd: ownerRoot });
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runVitest(this.root, rest, "🧪️vitest.config.ts");
  }
}

const router = new ScriptRouter(import.meta.dir).register("build", BuildScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "build" });
