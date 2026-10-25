import { availableParallelism } from "node:os";
import { repoCacheDirectory } from "../../⚡️caching/🟦️.ts";

/** 🧵️ Resolves the shared build task and worker capacity for this machine. */
export function semioBuildParallel(): number {
  const override = process.env.SEMIO_BUILD_PARALLEL?.trim();
  if (override) {
    const parsed = Number.parseInt(override, 10);
    if (Number.isFinite(parsed) && parsed > 0) return parsed;
  }
  return Math.max(1, availableParallelism());
}

/** 🚩️ Neutral task parallelism sized for this machine. */
export function semioBuildParallelFlag(): readonly [parallel: string, count: string] {
  return ["--parallel", String(semioBuildParallel())];
}

/** 🧰️ Carries neutral build policy without IDE-injected node options. */
export function devToolingEnv(extra: Partial<NodeJS.ProcessEnv> = {}): NodeJS.ProcessEnv {
  const env = { ...process.env, ...extra };
  delete env.NODE_OPTIONS;
  delete env.VSCODE_INSPECTOR_OPTIONS;
  env.SEMIO_BUILD_TERMINAL ??= "false";
  env.SEMIO_BUILD_DYNAMIC_OUTPUT ??= "false";
  env.SEMIO_BUILD_VERBOSE ??= "false";
  env.SEMIO_BUILD_PARALLEL ??= String(semioBuildParallel());
  return env;
}

/** ⚡️ Routes Go's build cache and Playwright's browser downloads into the shared cache root; never overrides a value the caller set explicitly. */
export function repoToolCacheEnv(repoRoot: string, extra: NodeJS.ProcessEnv = process.env): NodeJS.ProcessEnv {
  const env = { ...extra };
  env.GOCACHE ??= repoCacheDirectory(repoRoot, "go");
  env.PLAYWRIGHT_BROWSERS_PATH ??= repoCacheDirectory(repoRoot, "tools", "ms-playwright");
  return env;
}

/** 👁️ Uses native watcher events unless the explicit environment requires polling. */
export function playPollingEnv(extra: Partial<NodeJS.ProcessEnv> = {}): NodeJS.ProcessEnv {
  const pollingDefault = process.env.WATCHPACK_POLLING !== undefined ? {} : process.env.DEVCONTAINER === "true" ? { WATCHPACK_POLLING: "true", CHOKIDAR_USEPOLLING: "true" } : {};
  return devToolingEnv({
    ...pollingDefault,
    ...extra,
  });
}
