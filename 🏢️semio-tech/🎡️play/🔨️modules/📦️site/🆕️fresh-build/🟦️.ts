import { join, resolve } from "node:path";

/** 🆕️ Builds a caller-owned environment for one isolated release cache generation. */
export function playFreshBuildEnvironment(workspace: string, generation: string, inherited: Readonly<Record<string, string | undefined>>): Record<string, string | undefined> {
  const environment = { ...inherited }, root = resolve(workspace, generation);
  for (const key of Object.keys(environment)) if (key.startsWith("NX_TASK_") || key === "NX_FORCE_REUSE_CACHED_GRAPH") delete environment[key];
  return { ...environment, SEMIO_BUILD_MODE: "ship", NX_DAEMON: "false", NX_SKIP_NX_CACHE: "true", NX_SKIP_REMOTE_CACHE: "true", NX_CACHE_DIRECTORY: join(root, "n"), NX_WORKSPACE_DATA_DIRECTORY: join(root, "d"), CARGO_TARGET_DIR: join(root, "t"), CARGO_BUILD_TARGET_DIR: join(root, "t"), CARGO_BUILD_BUILD_DIR: join(root, "b"), SEMIO_CARGO_PREBUILD_PLAN: join(root, "p.json"), SEMIO_NX_OUTPUT_LOG: join(root, "o.log") };
}

/** 🌐️ Places Vite state within an explicitly selected invocation cache. */
export function playViteCacheDirectory(fallback: string, environment: Readonly<Record<string, string | undefined>>): string {
  return environment.NX_CACHE_DIRECTORY ? join(environment.NX_CACHE_DIRECTORY, "vite/semio-tech-play") : fallback;
}

/** 🪪️ Prefix of each fresh generation directly under the repository cache root; with one-letter stores it keeps every rustc output path shallower than the shared `cargo/build` layout, inside the 256-unit path budget. */
export const PLAY_FRESH_GENERATION_PREFIX = "f";

/** 🗺️ The Cargo workspaces whose plugin and extension components one release compiles together, one Cargo pass per profile. The standalone extension crates under `✏️s` each own their workspace, so they stay narrow builds that find the shared units fresh. */
export const PLAY_COMPONENT_WORKSPACES = ["🌎️hub"] as const;

/** 🧮️ Machine resources that bound how many Nx tasks a release runs at once. */
export type PlayFreshResources = Readonly<{ cpus: number; memoryBytes: number }>;

/** ⚖️ Concurrent Nx tasks: one per core, held back by the memory a wasm-opt or transpile task needs beside the compiler's own. */
export function playFreshParallel(resources: PlayFreshResources): number {
  return Math.max(1, Math.min(resources.cpus, Math.floor((resources.memoryBytes - PLAY_FRESH_RESERVE_BYTES) / PLAY_FRESH_BYTES_PER_TASK)));
}

/** 🎛️ Arguments of the one Nx run that builds the whole release graph. */
export function playFreshBuildArgs(resources: PlayFreshResources): readonly string[] {
  return ["run", "@semio-tech/semio-tech-play:build", "--skip-nx-cache", "--skip-remote-cache", `--parallel=${playFreshParallel(resources)}`, "--output-style=stream"];
}

export const PLAY_FRESH_RESERVE_BYTES = 8 * 1024 ** 3;
export const PLAY_FRESH_BYTES_PER_TASK = 3 * 1024 ** 3;

if (import.meta.vitest) {
  const { registerTests1 } = await import("../../../🧪️tests/🧪️playfreshbuild/🟦️.ts");
  await registerTests1(import.meta.vitest, { playFreshBuildEnvironment, playViteCacheDirectory, playFreshBuildArgs, playFreshParallel, PLAY_COMPONENT_WORKSPACES, PLAY_FRESH_GENERATION_PREFIX });
}
