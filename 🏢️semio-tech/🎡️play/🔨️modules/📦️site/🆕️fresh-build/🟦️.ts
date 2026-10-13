import { viteCacheDirectory } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🌐️vite/🟦️.ts";
import { join } from "node:path";

/** 🆕️ The release graph a fresh play build runs: the cached native generators and toolchains at t = 0, then the one play build driver task that compiles every component, runs every job and assembles the pages. */
export const PLAY_FRESH_BUILD_TARGET = "@semio-tech/semio-tech-play:build";

/** 🧮️ Machine resources that bound how many Nx tasks a release runs at once. */
export type PlayFreshResources = Readonly<{ cpus: number; memoryBytes: number }>;

export const PLAY_FRESH_RESERVE_BYTES = 8 * 1024 ** 3;
export const PLAY_FRESH_BYTES_PER_TASK = 3 * 1024 ** 3;

/** ⚖️ Concurrent Nx tasks of the release graph: its generators run beside each other before the driver, one per core, held back by the memory one generator needs. */
export function playFreshParallel(resources: PlayFreshResources): number {
  return Math.max(1, Math.min(resources.cpus, Math.floor((resources.memoryBytes - PLAY_FRESH_RESERVE_BYTES) / PLAY_FRESH_BYTES_PER_TASK)));
}

/** 🎛️ Arguments of the ONE Nx invocation a fresh release is. Nothing wraps it, so no Nx run nests inside an Nx task, and the driver bounds its own output. */
export function playFreshBuildArgs(resources: PlayFreshResources): readonly string[] {
  return ["run", PLAY_FRESH_BUILD_TARGET, "--skip-nx-cache", "--skip-remote-cache", `--parallel=${playFreshParallel(resources)}`, "--output-style=stream"];
}

/** 🆕️ The environment of that invocation: release mode, a cold driver generation (`SEMIO_RELEASE_GENERATION=fresh`), no daemon and no cache reads. */
export function playFreshBuildEnvironment(): Readonly<Record<string, string>> {
  return { SEMIO_BUILD_MODE: "release", SEMIO_RELEASE_GENERATION: "fresh", NX_DAEMON: "false", NX_SKIP_NX_CACHE: "true", NX_SKIP_REMOTE_CACHE: "true", NX_TUI: "false" };
}

/** 🌐️ Places Vite state within an explicitly selected invocation cache. */
export function playViteCacheDirectory(fallback: string, environment: Readonly<Record<string, string | undefined>>): string {
  return viteCacheDirectory(environment.NX_CACHE_DIRECTORY ? join(environment.NX_CACHE_DIRECTORY, "vite/semio-tech-play") : fallback, environment);
}

if (import.meta.vitest) {
  const { registerTests1 } = await import("../../../🧪️tests/🧪️playfreshbuild/🟦️.ts");
  await registerTests1(import.meta.vitest, { playFreshBuildEnvironment, playViteCacheDirectory, playFreshBuildArgs, playFreshParallel, PLAY_FRESH_BUILD_TARGET });
}
