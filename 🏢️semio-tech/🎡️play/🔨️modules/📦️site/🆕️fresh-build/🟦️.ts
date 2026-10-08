import { join, resolve } from "node:path";

/** 🆕️ Builds a caller-owned environment for one isolated release cache generation. */
export function playFreshBuildEnvironment(workspace: string, generation: string, inherited: Readonly<Record<string, string | undefined>>): Record<string, string | undefined> {
  const environment = { ...inherited }, root = resolve(workspace, generation);
  for (const key of Object.keys(environment)) if (key.startsWith("NX_TASK_") || key === "NX_FORCE_REUSE_CACHED_GRAPH") delete environment[key];
  return { ...environment, SEMIO_BUILD_MODE: "ship", NX_DAEMON: "false", NX_SKIP_NX_CACHE: "true", NX_SKIP_REMOTE_CACHE: "true", NX_CACHE_DIRECTORY: join(root, "nx"), NX_WORKSPACE_DATA_DIRECTORY: join(root, "nx-data"), CARGO_TARGET_DIR: join(root, "cargo/target"), CARGO_BUILD_TARGET_DIR: join(root, "cargo/target"), CARGO_BUILD_BUILD_DIR: join(root, "cargo/build") };
}

/** 🌐️ Places Vite state within an explicitly selected invocation cache. */
export function playViteCacheDirectory(fallback: string, environment: Readonly<Record<string, string | undefined>>): string {
  return environment.NX_CACHE_DIRECTORY ? join(environment.NX_CACHE_DIRECTORY, "vite/semio-tech-play") : fallback;
}

export const PLAY_FRESH_BUILD_ARGS = ["run", "@semio-tech/semio-tech-play:build", "--skip-nx-cache", "--skip-remote-cache", "--parallel=4", "--output-style=stream"] as const;

if (import.meta.vitest) {
  const { registerTests1 } = await import("../../../🧪️tests/🧪️playfreshbuild/🟦️.ts");
  await registerTests1(import.meta.vitest, { playFreshBuildEnvironment, playViteCacheDirectory, PLAY_FRESH_BUILD_ARGS });
}
