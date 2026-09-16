/** @emoji 🫀️ The plugin-load liveness clock — the ONE place that decides whether a program load that
 * is taking minutes is still moving or has stopped.
 *
 * It exists as its own leaf because both halves of that decision live in different files: the deadline
 * rule is read by the shell's `loadPluginModuleResilient` (`🛠️ShellHelpers/🟦️.tsx`) while the proofs of
 * progress are stamped by the runtime's `loadPluginModule` and by every shard-worker beat
 * (`🔌️PluginRuntime/🟦️.tsx`). Nothing here touches React, the DOM or a `Worker`, so the accounting is
 * testable on its own — see `🧪️tests/🫀️plugin-load-progress/🟦️.ts`. */

import { SHARD_LIVENESS_POLICY } from "../../../../../../../../🔨️modules/🎭️actor/📮️shard-client/🟦️.ts";

/** 🫀️ Both numbers come from the ONE schema-owned liveness policy — never a literal here.
 * `pluginLoadIdleTimeoutMs` is an IDLE budget, not a total one: the deadline is pushed forward every
 * time this load's pipeline reports progress, so a multi-hundred-MB wasm component fetching, compiling
 * and instantiating for two minutes on a loaded machine survives while a plugin whose module 404s or
 * whose worker died still fails within one idle window. `pluginLoadCeilingMs` bounds the whole attempt
 * regardless of progress, so a load that keeps being told about progress can never wedge the boot. */
export const PLUGIN_LOAD_IDLE_TIMEOUT_MS = SHARD_LIVENESS_POLICY.pluginLoadIdleTimeoutMs;
export const PLUGIN_LOAD_CEILING_MS = SHARD_LIVENESS_POLICY.pluginLoadCeilingMs;

/** ⏱️ Pure deadline rule, split out so it can be tested without a clock or a `Worker`: given when the
 * attempt started, when this plugin last reported progress and what time it is now, say how much
 * longer to wait (`0` means give up now). */
export function pluginLoadRemainingMs(startedAtMs: number, lastProgressAtMs: number | undefined, nowMs: number, idleTimeoutMs: number = PLUGIN_LOAD_IDLE_TIMEOUT_MS, ceilingMs: number = PLUGIN_LOAD_CEILING_MS): number {
  const idleRemaining = Math.max(lastProgressAtMs ?? startedAtMs, startedAtMs) + idleTimeoutMs - nowMs;
  const ceilingRemaining = startedAtMs + ceilingMs - nowMs;
  return Math.max(0, Math.min(idleRemaining, ceilingRemaining));
}

/** 🫀️ Per-plugin proof-of-progress clock: a load that is still moving (descriptor bytes arriving,
 * manifest registered, module handed over, a shard beating through its compile) must not be killed
 * just because the whole pipeline is slow under load — only one that has stopped moving. */
const pluginLoadProgress = new Map<string, number>();

export function notePluginLoadProgress(pluginId: string, atMs: number = Date.now()): void {
  pluginLoadProgress.set(pluginId, atMs);
}

export function pluginLoadProgressAt(pluginId: string): number | undefined {
  return pluginLoadProgress.get(pluginId);
}

/** 🫀️ Plugins whose load has not returned yet, by refcount — several panes of one page load the SAME
 * plugin concurrently, and each has its own call in flight. */
const pluginLoadsInFlight = new Map<string, number>();

export function beginPluginLoadV1(pluginId: string, atMs: number = Date.now()): void {
  pluginLoadsInFlight.set(pluginId, (pluginLoadsInFlight.get(pluginId) ?? 0) + 1);
  notePluginLoadProgress(pluginId, atMs);
}

export function endPluginLoadV1(pluginId: string): void {
  const outstanding = (pluginLoadsInFlight.get(pluginId) ?? 0) - 1;
  if (outstanding > 0) pluginLoadsInFlight.set(pluginId, outstanding);
  else pluginLoadsInFlight.delete(pluginId);
}

export function pluginLoadsInFlightV1(): readonly string[] {
  return [...pluginLoadsInFlight.keys()];
}

/** 🫀️ Runs the phase whose slowness the idle deadline must forgive with this plugin on the in-flight
 * roster — and takes it off again however that phase ends, so a failed load can never leave a ghost on
 * the roster keeping later loads' deadlines alive. */
export async function withPluginLoadInFlightV1<T>(pluginId: string, run: () => Promise<T>): Promise<T> {
  beginPluginLoadV1(pluginId);
  try {
    return await run();
  } finally {
    endPluginLoadV1(pluginId);
  }
}

/** 🫀️ Stamps every in-flight load with one proof that the module pipeline moved, and answers how many
 * loads that was.
 *
 * 🐛️ The idle deadline only ever saw the three stamps the loader makes itself, so the ONE phase that
 * actually takes minutes — the shard workers fetching, compiling and instantiating a multi-hundred-MB
 * component, which is also what starves every other request on the origin — proved nothing to it. In
 * the six-pane demonstrator boot (ticket 26/08/28) each pane's own small descriptor request queued
 * behind those module fetches, reported no progress for 30 s, and was failed as dead: `timeout loading
 * demonstrator after 30002 ms with no progress for 30000 ms`. A shard beat IS the missing proof — the
 * worker only beats while its event loop runs, so a beat says the pipeline every waiting load is
 * queued behind is alive. `PLUGIN_LOAD_CEILING_MS` still bounds the attempt, so forgiveness is not
 * unbounded. */
export function notePluginLoadProgressForInFlightV1(atMs: number = Date.now()): number {
  for (const pluginId of pluginLoadsInFlight.keys()) notePluginLoadProgress(pluginId, atMs);
  return pluginLoadsInFlight.size;
}

/** 🚦️ One descriptor request per module url, shared by every pane loading it.
 *
 * 🐛️ Six shells booting one page asked for the SAME `🔣️.json` six times, each taking one of the
 * browser's six per-origin connections away from the module fetches they were all waiting on. The
 * descriptor is immutable for a url, so the first request answers all of them; a hot swap busts the
 * url and so gets its own request. */
const descriptorRequestsByUrl = new Map<string, Promise<unknown>>();

export function sharedDescriptorManifestV1<T>(pluginId: string, moduleUrl: string, fetchManifest: () => Promise<T>): Promise<T> {
  // 🔐️ Keyed by the plugin too, never by the url alone: the fetch's own identity check ("this
  // descriptor names the plugin I asked for") is per-caller, and sharing one answer across two
  // different plugin ids would hand the second caller a manifest nobody checked for it.
  const key = `${pluginId}\n${moduleUrl}`;
  const shared = descriptorRequestsByUrl.get(key);
  if (shared) return shared as Promise<T>;
  const request = fetchManifest().finally(() => descriptorRequestsByUrl.delete(key));
  descriptorRequestsByUrl.set(key, request);
  // 🚑️ A rejection settles every waiter; this bookkeeping copy must never become one of its own.
  request.catch(() => {});
  return request;
}

/** 🧪️ Empties both ledgers — for suites that assert on roster membership across cases. */
export function resetPluginLoadProgressForTestsV1(): void {
  pluginLoadProgress.clear();
  pluginLoadsInFlight.clear();
  descriptorRequestsByUrl.clear();
}
