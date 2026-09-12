#!/usr/bin/env bun
/** 🧭️ Coordinator package router: `bun ./📜️script.ts build|test|policy`. */
import type { BundleLinter } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { dependencyBoundaryBreachesForBundleDir, getWorkspaceRoot, BundleScript, ScriptRouter, runBundleScriptMain, runVitest, resolveTestLevel, runCmd, daemonBudgetOpts, defineLint, goLevelTestArgs, runCanonicalGoBuild, runCanonicalGoTests } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { dirname, join } from "node:path";

export const policy = defineLint("@repo/server/coordinator-bundle", (l: BundleLinter) => {
  const repoRoot = getWorkspaceRoot();
  return dependencyBoundaryBreachesForBundleDir(repoRoot, l.root());
});

class BuildScript extends BundleScript {
  run(): void {
    const ext = process.platform === "win32" ? ".exe" : "";
    const ownerRoot = join(dirname(import.meta.dir), "..");
    runCanonicalGoBuild(ownerRoot, ["-o", `server${ext}`, "."]);
  }
}

/** ▶️ Runs the executable restored or built by the Nx prerequisite. */
class DevScript extends BundleScript {
  run(args: string[]): void {
    const ownerRoot = join(import.meta.dir, "../..");
    runCmd(join(ownerRoot, process.platform === "win32" ? "server.exe" : "server"), args, { cwd: ownerRoot, ...daemonBudgetOpts() });
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { level, rest } = resolveTestLevel(segments);
    await runCanonicalGoTests(join(import.meta.dir, "../.."), [...goLevelTestArgs(level), ...rest]);
    await runVitest(this.root, rest, "vitest.config.ts");
  }
}

const router = new ScriptRouter(import.meta.dir).register("build", BuildScript).register("dev", DevScript).register("start", DevScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "build" });
