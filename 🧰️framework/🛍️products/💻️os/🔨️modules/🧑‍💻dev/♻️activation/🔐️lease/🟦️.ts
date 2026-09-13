/** 🧩️ Semantic activation lease owner. */

import { constants as fsConstants, createReadStream, createWriteStream, copyFileSync, cpSync, existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, realpathSync, renameSync, rmSync, rmdirSync, statSync, unlinkSync, watch, writeFileSync } from "node:fs";

import { basename, dirname, isAbsolute, join, relative, resolve } from "node:path";

import { getWorkspaceRoot } from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

const repoRoot = getWorkspaceRoot();

import { pluginOutRoot } from "../../../🔌️plugin/🏗️build/📋️plan/🟦️.ts";



//#endregion 🔖️PollHelpers

//#region 🔖️PluginBuildLease
/** 🔐️ Identifies the collaboration process preparing and watching one playground's plugin components.
 * Followers can reuse the catalog after the holder publishes `registryReady`. */
type PluginBuildLease = { readonly pid: number; readonly port: number; readonly startedAt: number; registryReady: boolean };

/** ⏳️ Bounds how long collaboration followers wait for the holder's catalog readiness. */
const PLUGIN_BUILD_LEASE_READY_TIMEOUT_MS = 60_000;

function pluginBuildLeaseDir(): string {
  return join(repoRoot, "target/semio-dev-leases");
}

function pluginBuildLeasePath(variant: string): string {
  return join(pluginBuildLeaseDir(), `plugin-build-${variant}.json`);
}

/** @emoji 💀️ True when `pid` no longer exists on this machine (cross-platform: `process.kill(pid, 0)`
 * is a liveness probe on POSIX and Windows alike, never an actual kill). */
function isPidAlive(pid: number): boolean {
  try {
    process.kill(pid, 0);
    return true;
  } catch {
    return false;
  }
}

function readPluginBuildLease(path: string): PluginBuildLease | undefined {
  try {
    return JSON.parse(readFileSync(path, "utf8")) as PluginBuildLease;
  } catch {
    return undefined;
  }
}

/** @emoji 🔐️ Claims the plugin-build lease for `variant`: atomically creates the lease file (`wx` —
 * fails with `EEXIST` when another live holder exists) or takes over a stale one (dead `pid`) in
 * place. Returns `"holder"` for this process, or the live `"follower"` lease otherwise. */
function acquirePluginBuildLease(variant: string, port: number): { readonly role: "holder" } | { readonly role: "follower"; readonly lease: PluginBuildLease } {
  mkdirSync(pluginBuildLeaseDir(), { recursive: true });
  const path = pluginBuildLeasePath(variant);
  for (;;) {
    try {
      writeFileSync(path, JSON.stringify({ pid: process.pid, port, startedAt: Date.now(), registryReady: false } satisfies PluginBuildLease, null, 2), { flag: "wx" });
      return { role: "holder" };
    } catch (error) {
      if ((error as NodeJS.ErrnoException).code !== "EEXIST") throw error;
    }
    const existing = readPluginBuildLease(path);
    if (!existing || !isPidAlive(existing.pid)) {
      try {
        rmSync(path, { force: true });
      } catch {
        // 🏁️ Raced with another taker's own stale-cleanup; the atomic `wx` retry above is the real gate.
      }
      continue;
    }
    return { role: "follower", lease: existing };
  }
}

/** @emoji ✅️ Flips `registryReady` once the holder's registry catalog + engine wasm are on disk. No-op
 * if this process no longer owns the lease (lost to a stale-takeover race). */
function markPluginBuildLeaseReady(variant: string): void {
  const path = pluginBuildLeasePath(variant);
  const lease = readPluginBuildLease(path);
  if (!lease || lease.pid !== process.pid) return;
  writeFileSync(path, JSON.stringify({ ...lease, registryReady: true } satisfies PluginBuildLease, null, 2));
}

/** @emoji 🕰️ Follower-side wait for the holder's `registryReady` flag, capped at
 * `PLUGIN_BUILD_LEASE_READY_TIMEOUT_MS`. Returns `true` when the holder is ready, when the lease file
 * vanishes (holder released/finished), or when its `pid` dies mid-wait — either way nothing is left to
 * wait on.
 *
 * 🚨️ Returns `false` on timeout rather than throwing. This lease is a build-deduplication OPTIMISATION
 * for the two-user launchers, never a precondition for running `dev` at all: a holder that is merely
 * slow (heavy cargo contention) or wedged must degrade a second `dev` into doing its own build, not
 * abort it. Throwing here made a single stale lease file break the primary `dev` workflow outright. */
async function waitForPluginBuildLeaseReady(variant: string, deadlineMs: number): Promise<boolean> {
  const path = pluginBuildLeasePath(variant);
  const deadline = Date.now() + deadlineMs;
  while (Date.now() < deadline) {
    const lease = readPluginBuildLease(path);
    if (!lease || lease.registryReady || !isPidAlive(lease.pid)) return true;
    // 🏛️ THE RULE (see 🔖️PollHelpers above): legitimate poll, not routed through a helper — the lease
    // holder is another `dev` process entirely, identified only by a `pid` in this on-disk lease file.
    // We hold no handle to it (no child object, no stream, no promise), only its pid, so there is no
    // event to await; a lease file + `isPidAlive` liveness check is the only signal available.
    await Bun.sleep(500);
  }
  return false;
}

/** @emoji 🧾️ Whether the shared build outputs a follower intends to serve are actually on disk — the
 * generated playground catalog plus a non-empty `🔌️plugin-modules/`. Checked instead of trusting the
 * lease flag alone, so a follower never serves an empty module directory just because some other
 * process claimed readiness. */
function pluginBuildOutputsPresent(): boolean {
  try {
    return existsSync(pluginOutRoot) && readdirSync(pluginOutRoot).length > 0;
  } catch {
    return false;
  }
}

/** @emoji 🪓️ Forcibly takes the lease for this process after a follower gave up waiting, so it can do
 * the build itself. Best-effort: losing the ensuing `wx` race just means somebody else holds it and we
 * build anyway, which is wasteful but always correct. */
function takeOverPluginBuildLease(variant: string, port: number): void {
  const path = pluginBuildLeasePath(variant);
  try {
    rmSync(path, { force: true });
  } catch {
    // 🏁️ A vanished lease is the desired end state either way.
  }
  try {
    mkdirSync(pluginBuildLeaseDir(), { recursive: true });
    writeFileSync(path, JSON.stringify({ pid: process.pid, port, startedAt: Date.now(), registryReady: false } satisfies PluginBuildLease, null, 2));
  } catch {
    // 🏁️ Unwritable lease dir is not a reason to refuse to build.
  }
}

/** @emoji 🔓️ Releases this process's own plugin-build lease (no-op if it was never the holder, or lost
 * the lease to a stale-takeover race) — called from `exit`/`SIGINT` so the next `dev` process for the
 * same variant can immediately claim the lease instead of waiting out a dead holder's timeout. */
function releasePluginBuildLease(variant: string): void {
  const path = pluginBuildLeasePath(variant);
  const lease = readPluginBuildLease(path);
  if (!lease || lease.pid !== process.pid) return;
  try {
    rmSync(path, { force: true });
  } catch {
    // 🏁️ Best-effort: a vanished lease file is already the desired end state.
  }
}

export { PLUGIN_BUILD_LEASE_READY_TIMEOUT_MS, PluginBuildLease, acquirePluginBuildLease, isPidAlive, markPluginBuildLeaseReady, pluginBuildLeaseDir, pluginBuildLeasePath, pluginBuildOutputsPresent, readPluginBuildLease, releasePluginBuildLease, takeOverPluginBuildLease, waitForPluginBuildLeaseReady };
