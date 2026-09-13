/** 🧩️ Semantic activation readiness owner. */

import {
  BundleScript,
  ScriptRouter,
  buildBudgetMs,
  daemonBudgetOpts,
  describeDevPortOccupant,
  devServerUrl,
  getWorkspaceRoot,
  getRepoMetaDir,
  isDevPortInUse,
  loadFrameworkOsPlaygroundCatalog,
  wgpuDevPlayUrl,
  runBundleScriptMain,
  runCmd,
  runCmdStatus,
  runBunxStatus,
  runNodeBinStatus,
  runProbe,
  runVitest,
  spawnDaemon,
  type SpawnDaemonHandle,
  runViteBunxDev,
  frameworkOsPlaygroundDefaultPort,
  frameworkOsLockedPrefsEnv,
  resolveTestLevel,
  atTestLevel,
  cargoProfileDir,
  selectComponentWasmProfile,
  semioBuildMode,
  semioShipEnv,
} from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";



/** @emoji 🐚️ Fixed port for `dev multi` — the multi-shell harness (`🧪️tests/🧪️multi-shell-harness/🟦️.tsx`), free in the 60xx range
 * used by every other os-dev variant/launch.json entry. */
const FRAMEWORK_OS_MULTI_HARNESS_PORT = "6071";

//#region 🔖️PollHelpers
/** @emoji 🏛️ THE RULE (poll census, W6): a deadline-bounded poll is legitimate here if and only if
 * the thing it waits on is EXTERNAL — a TCP port or HTTP endpoint belonging to a process we did not
 * instrument, a lease file another `dev` invocation owns, a filesystem lock — and therefore emits no
 * observable event we could await instead. The moment we hold the resource's own handle (a spawned
 * child's `exit` event, a stream, a promise it already exposes), polling it on a timer is NOT
 * legitimate; await the handle instead ([[awaitChildExit]] below replaced two such polls).
 * [[awaitTcpReady]] / [[awaitHttpOk]] exist for the legitimate case only — real external readiness
 * checks this process has no better signal for. A future poll census should find every `Bun.sleep`
 * loop either inside one of these three helpers, or commented at its call site explaining which
 * external resource it legitimately waits on (see `waitForPluginBuildLeaseReady`'s lease-file poll
 * and `prebuildParityPlugin`'s mkdir-lock poll — both fs-based waits on a PID/lock this process holds
 * no handle for, so neither fits a TCP/HTTP shape). */

type PollOutcome = "ready" | "dead" | "timeout";

/** @emoji ⏳️ Deadline-bounded poll for a TCP `port` on `host` to reach the wanted state — open
 * (`mode: "open"`, the default: something is now listening) or closed (`mode: "closed"`: nothing is
 * listening anymore). Checks every `intervalMs`, capped at `deadlineMs` total from the call, and can
 * race an optional `isDead()` predicate (e.g. `child.exitCode !== null`) so a spawn that already died
 * does not have to wait out the full deadline before its caller finds out. `probe`/`sleep`/`now` are
 * test-only injection points — production callers rely on the defaults ([[isDevPortInUse]]/
 * `Bun.sleep`/`Date.now`). Never throws; callers turn the [[PollOutcome]] into whatever error
 * message fits their own call site. */
async function awaitTcpReady(
  host: string,
  port: number,
  opts: {
    readonly deadlineMs: number;
    readonly intervalMs: number;
    readonly mode?: "open" | "closed";
    readonly isDead?: () => boolean;
    readonly probe?: (host: string, port: number) => boolean;
    readonly sleep?: (ms: number) => Promise<void>;
    readonly now?: () => number;
  },
): Promise<PollOutcome> {
  const mode = opts.mode ?? "open";
  const probe = opts.probe ?? isDevPortInUse;
  const sleep = opts.sleep ?? ((ms: number) => Bun.sleep(ms));
  const now = opts.now ?? Date.now;
  const deadline = now() + opts.deadlineMs;
  while (now() < deadline) {
    const inUse = probe(host, port);
    if (mode === "open" ? inUse : !inUse) return "ready";
    if (opts.isDead?.()) return "dead";
    await sleep(opts.intervalMs);
  }
  return "timeout";
}

/** @emoji 🌐️ Deadline-bounded poll for `url` to answer any HTTP response at all — per THE RULE
 * above, a `fetch` that throws (connection refused, DNS not up yet) just means the server isn't
 * listening yet, not a real failure. Does not inspect `response.ok`; callers that need a specific
 * status/body check the fetched response themselves once they have their own handle to it — this
 * helper only proves *something* is answering on `url`. Shares [[awaitTcpReady]]'s deadline/isDead/
 * injection shape and [[PollOutcome]]. */
async function awaitHttpOk(
  url: string,
  opts: {
    readonly deadlineMs: number;
    readonly intervalMs: number;
    readonly init?: RequestInit;
    readonly isDead?: () => boolean;
    readonly fetchImpl?: typeof fetch;
    readonly sleep?: (ms: number) => Promise<void>;
    readonly now?: () => number;
  },
): Promise<PollOutcome> {
  const fetchImpl = opts.fetchImpl ?? fetch;
  const sleep = opts.sleep ?? ((ms: number) => Bun.sleep(ms));
  const now = opts.now ?? Date.now;
  const deadline = now() + opts.deadlineMs;
  while (now() < deadline) {
    if (opts.isDead?.()) return "dead";
    try {
      await fetchImpl(url, opts.init);
      return "ready";
    } catch {
      await sleep(opts.intervalMs);
    }
  }
  return "timeout";
}

/** @emoji 🧵️ Resolves once `child` exits — Node's own `'exit'` event, not a poll — or with
 * `"timeout"` after `deadlineMs`, whichever comes first. This is what THE RULE above means by
 * "await the handle instead": a `ChildProcess` we spawned already tells us when it exits, so
 * re-checking `child.exitCode` on a `Bun.sleep` timer is exactly the "wired but inert" shape this
 * ticket exists to remove. Handles the case where `child` already exited before this was called (its
 * `exitCode` is set synchronously before `'exit'` fires, so a late listener would otherwise hang
 * forever). `timeoutAfter` is a test-only injection point for the deadline race; production callers
 * keep the real `setTimeout`. */
async function awaitChildExit(child: SpawnDaemonHandle["child"], deadlineMs: number, opts: { readonly timeoutAfter?: (ms: number) => Promise<"timeout"> } = {}): Promise<"exited" | "timeout"> {
  const timeoutAfter = opts.timeoutAfter ?? ((ms: number) => new Promise<"timeout">((resolve) => setTimeout(() => resolve("timeout"), ms)));
  const exited = new Promise<"exited">((resolve) => {
    if (child.exitCode !== null) {
      resolve("exited");
      return;
    }
    child.once("exit", () => resolve("exited"));
  });
  return Promise.race([exited, timeoutAfter(deadlineMs)]);
}

export { FRAMEWORK_OS_MULTI_HARNESS_PORT, PollOutcome, awaitChildExit, awaitHttpOk, awaitTcpReady };
