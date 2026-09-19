#!/usr/bin/env bun
/** 🧭️ Coordinator Next.js package router: `bun ./📜️script.ts build|dev|start|test|policy`. */
import { join } from "node:path";
import type { BundleLinter } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter, defineLint, dependencyBoundaryBreachesForBundleDir, devToolingEnv, getWorkspaceRoot, goLevelTestArgs, resolveTestLevel, runBundleScriptMain, runBunx, runCanonicalGoTests, runVitest } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

export const policy = defineLint("@repo/server/coordinator-bundle", (l: BundleLinter) => {
  const repoRoot = getWorkspaceRoot();
  return dependencyBoundaryBreachesForBundleDir(repoRoot, l.root());
});

/** ⚙️ Every Next.js invocation runs inside this package, never at the repository root. */
const PACKAGE_DIR = import.meta.dir;

/** 🏗️ Compiles the coordinator's own Next.js surface; the Go and Rust servers build from their own packages. */
class BuildScript extends BundleScript {
  run(segments: string[]): void {
    runBunx(["next", "build", ...segments], PACKAGE_DIR, devToolingEnv());
  }
}

/** 🛠️ Serves the coordinator through Next's development server, rebuilding on every edit. */
class DevScript extends BundleScript {
  run(segments: string[]): void {
    runBunx(["next", "dev", ...segments], PACKAGE_DIR, devToolingEnv());
  }
}

/** ▶️ Serves the standalone output produced by `build`. */
class StartScript extends BundleScript {
  run(segments: string[]): void {
    runBunx(["next", "start", ...segments], PACKAGE_DIR, devToolingEnv());
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { level, rest } = resolveTestLevel(segments);
    await runCanonicalGoTests(join(import.meta.dir, "../.."), [...goLevelTestArgs(level), ...rest]);
    await runVitest(this.root, rest, "../../🧪️tests/🎚️config/🟦️.ts");
  }
}

const router = new ScriptRouter(import.meta.dir).register("build", BuildScript).register("dev", DevScript).register("start", StartScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "build" });
