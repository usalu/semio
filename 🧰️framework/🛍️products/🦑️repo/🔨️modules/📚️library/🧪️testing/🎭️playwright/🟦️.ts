import { runBunx } from "../../🏃️process/🍞️bun/🟦️.ts";
import { playPollingEnv, repoToolCacheEnv } from "../../🏃️process/🌿️environment/🟦️.ts";
import { findWorkspaceRoot } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

/** ▶️Playwright test run in bundle directory; browsers land in the shared cache root. */
export function runPlaywright(bundleRoot: string, config: string, segments: string[] = []): void {
  runBunx(["playwright", "test", "--config", config, ...segments], bundleRoot, repoToolCacheEnv(findWorkspaceRoot(bundleRoot), playPollingEnv()));
}
