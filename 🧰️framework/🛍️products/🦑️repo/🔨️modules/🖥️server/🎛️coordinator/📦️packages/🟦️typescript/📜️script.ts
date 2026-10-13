#!/usr/bin/env bun
import { readBuildMode, buildModeEnvironment } from "../../../../../../../🔨️modules/🏃️process/🏗️build/🎚️mode/🟦️.ts";
import { withCompilerState } from "../../../../📚️library/⚡️caching/🏗️state/🟦️.ts";
import { repoCacheDirectory } from "../../../../📚️library/⚡️caching/🟦️.ts";
import { runOwnedCommand } from "../../../../../../../🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts";
import { createRequire } from "node:module";
import { resolveTestLevel } from "../../../../../../../🔨️modules/🏃️process/🧪️testing/🎚️budget/🟦️.ts";
/** 🧭️ Coordinator Next.js package router: `bun ./📜️script.ts build|dev|start|test|policy`. */
import { join } from "node:path";
import type { BundleLinter } from "../../../../📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { defineLint, dependencyBoundaryBreachesForBundleDir, devToolingEnv, getWorkspaceRoot, goLevelTestArgs, runBunx, runCanonicalGoTests, runVitest } from "../../../../📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runRepoScriptMain } from "../../../../📚️library/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

export const policy = defineLint("@repo/server/coordinator-bundle", (l: BundleLinter) => {
  const repoRoot = getWorkspaceRoot();
  return dependencyBoundaryBreachesForBundleDir(repoRoot, l.root());
});

/** ⚙️ Every Next.js invocation runs inside this package, never at the repository root. */
const PACKAGE_DIR = import.meta.dir;

/** 🏗️ Compiles the coordinator's own Next.js surface; the Go and Rust servers build from their own packages. */
class BuildScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const mode = readBuildMode(process.env.SEMIO_BUILD_MODE), require = createRequire(join(PACKAGE_DIR, "package.json"));
    await withCompilerState({ root: repoCacheDirectory(this.repoRoot), owner: "repo-coordinator:next", compiler: `next:${require("next/package.json").version}`, configuration: PACKAGE_DIR, mode, signal: this.invocation.control.signal }, async () => {
      await runOwnedCommand(process.execPath, [require.resolve("next/dist/bin/next"), "build", ...segments], PACKAGE_DIR, "repo-coordinator:next", 600_000, { env: buildModeEnvironment(mode, devToolingEnv()), signal: this.invocation.control.signal });
    });
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

await runRepoScriptMain(router, import.meta.url, { defaultCommand: "build" });
