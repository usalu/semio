#!/usr/bin/env bun
/** 🧭️ Coordinator Next.js package router: `bun ./📜️script.ts build|dev|start|test|policy`. */
import type { BundleLinter } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

export const policy = defineLint("@repo/server/coordinator-bundle", (l: BundleLinter) => {
  const repoRoot = getWorkspaceRoot();
  return dependencyBoundaryBreachesForBundleDir(repoRoot, l.root());
});

/** ⚙️ Every Next.js invocation runs inside this package, never at the repository root. */
const PACKAGE_DIR = import.meta.dir;

/** 🏗️ Compiles the coordinator's own Next.js surface; the Go and Rust servers build from their own packages. */
class BuildScript extends BundleScript {
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { level, rest } = resolveTestLevel(segments);
    await runCanonicalGoTests(join(import.meta.dir, "../.."), [...goLevelTestArgs(level), ...rest]);
    await runVitest(this.root, rest, "../../🧪️tests/🎚️config/🟦️.ts");
  }
}


await runBundleScriptMain(router, import.meta.url, { defaultCommand: "build" });
