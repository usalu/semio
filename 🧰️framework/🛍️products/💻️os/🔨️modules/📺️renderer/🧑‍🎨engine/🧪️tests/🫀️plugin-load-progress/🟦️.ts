/** 🫀️ The plugin-load liveness accounting: what counts as progress, who it counts for, and why a
 * slow-but-moving load may not be failed as dead.
 *
 * Pins the six-pane demonstrator boot regression (ticket 26/08/28,
 * `📓️fix-2026-09-16-concurrent-boot-timeout-and-close-fault.md`): five shells load ONE bundled
 * component, their small descriptor requests queue behind the shard workers' own multi-hundred-MB
 * module fetches, and the panes that lost the race died on `timeout loading demonstrator after
 * 30002 ms with no progress for 30000 ms` — an idle window they were never idle in. */

import { describe, expect, it, beforeEach } from "vitest";
import { fetchDescriptorManifest } from "../../../../../../../🔨️modules/🎠️kernel/🟦️.ts";
import { SHARD_LIVENESS_POLICY } from "../../../../../../../🔨️modules/🎭️actor/📮️shard-client/🟦️.ts";
import {
  beginPluginLoadV1,
  endPluginLoadV1,
  notePluginLoadProgress,
  notePluginLoadProgressForInFlightV1,
  pluginLoadProgressAt,
  pluginLoadRemainingMs,
  pluginLoadsInFlightV1,
  resetPluginLoadProgressForTestsV1,
  sharedDescriptorManifestV1,
  withPluginLoadInFlightV1,
  PLUGIN_LOAD_CEILING_MS,
  PLUGIN_LOAD_IDLE_TIMEOUT_MS,
} from "../../🧱️elements/🔌️PluginRuntime/🫀️load-progress/🟦️.ts";
import { stubFetch } from "../../../../../🧪️tests/🌐️fetch-stub/🟦️.ts";

const IDLE = SHARD_LIVENESS_POLICY.pluginLoadIdleTimeoutMs;
const CEILING = SHARD_LIVENESS_POLICY.pluginLoadCeilingMs;

beforeEach(() => resetPluginLoadProgressForTestsV1());

describe("plugin load deadline", () => {
  it("reads both budgets from the one schema-owned liveness policy", () => {
    expect(PLUGIN_LOAD_IDLE_TIMEOUT_MS).toBe(IDLE);
    expect(PLUGIN_LOAD_CEILING_MS).toBe(CEILING);
  });

  it("gives up only after a whole idle window with nothing reported", () => {
    expect(pluginLoadRemainingMs(0, undefined, IDLE - 1)).toBe(1);
    expect(pluginLoadRemainingMs(0, undefined, IDLE)).toBe(0);
  });

  /** ⏱️ LAW: a load that keeps being told about progress keeps its deadline — the ONLY thing that may
   * end it early is silence. This is the exact case the six-pane boot failed: the pipeline was moving
   * for the whole 30 s and nothing was reporting it. */
  it("never expires while progress keeps arriving, up to the attempt ceiling", () => {
    let lastProgressAtMs = 0;
    for (let nowMs = 0; nowMs < CEILING - IDLE; nowMs += IDLE - 1) {
      expect(pluginLoadRemainingMs(0, lastProgressAtMs, nowMs)).toBeGreaterThan(0);
      lastProgressAtMs = nowMs;
    }
    expect(pluginLoadRemainingMs(0, CEILING, CEILING)).toBe(0);
  });

  it("caps a forever-progressing load at the ceiling rather than the idle window", () => {
    expect(pluginLoadRemainingMs(0, CEILING - 1, CEILING - 1)).toBe(1);
    expect(pluginLoadRemainingMs(0, CEILING + 1_000, CEILING)).toBe(0);
  });

  it("treats a progress stamp older than the attempt start as no progress at all", () => {
    expect(pluginLoadRemainingMs(1_000, 0, 1_000)).toBe(IDLE);
  });
});

describe("in-flight load roster", () => {
  it("counts concurrent loads of one plugin and clears only when the last one ends", () => {
    beginPluginLoadV1("demonstrator", 10);
    beginPluginLoadV1("demonstrator", 20);
    expect(pluginLoadsInFlightV1()).toEqual(["demonstrator"]);
    endPluginLoadV1("demonstrator");
    expect(pluginLoadsInFlightV1()).toEqual(["demonstrator"]);
    endPluginLoadV1("demonstrator");
    expect(pluginLoadsInFlightV1()).toEqual([]);
  });

  /** 🫀️ LAW: a shard beat is progress for every load queued behind that shard's module pipeline —
   * without it the ONE phase that takes minutes proves nothing to the deadline that is timing it. */
  it("stamps every in-flight load from one shard beat and nothing that is not loading", () => {
    beginPluginLoadV1("demonstrator", 10);
    beginPluginLoadV1("puzzle", 10);
    notePluginLoadProgress("settled", 10);
    expect(notePluginLoadProgressForInFlightV1(5_000)).toBe(2);
    expect(pluginLoadProgressAt("demonstrator")).toBe(5_000);
    expect(pluginLoadProgressAt("puzzle")).toBe(5_000);
    expect(pluginLoadProgressAt("settled")).toBe(10);
  });

  it("carries a load past a whole idle window on shard beats alone", () => {
    beginPluginLoadV1("demonstrator", 0);
    for (let nowMs = IDLE - 1; nowMs < CEILING - IDLE; nowMs += IDLE - 1) {
      notePluginLoadProgressForInFlightV1(nowMs);
      expect(pluginLoadRemainingMs(0, pluginLoadProgressAt("demonstrator"), nowMs)).toBeGreaterThan(0);
    }
  });

  it("takes a failed load off the roster so it cannot keep later deadlines alive", async () => {
    await expect(withPluginLoadInFlightV1("demonstrator", async () => { throw new Error("descriptor unavailable"); })).rejects.toThrow("descriptor unavailable");
    expect(pluginLoadsInFlightV1()).toEqual([]);
    expect(notePluginLoadProgressForInFlightV1()).toBe(0);
  });
});

describe("shared descriptor request", () => {
  it("answers every concurrent pane from ONE request per module url", async () => {
    let requests = 0;
    const fetchManifest = async () => { requests += 1; return { pluginId: "demonstrator" }; };
    const panes = await Promise.all([1, 2, 3, 4, 5, 6].map(() => sharedDescriptorManifestV1("demonstrator", "/plugin-modules/demonstrator/bridge.js", fetchManifest)));
    expect(requests).toBe(1);
    for (const pane of panes) expect(pane).toEqual({ pluginId: "demonstrator" });
  });

  it("gives a later url — a hot swap — its own request, and never reuses a settled one", async () => {
    let requests = 0;
    const fetchManifest = async () => { requests += 1; return requests; };
    await sharedDescriptorManifestV1("demonstrator", "/a.js", fetchManifest);
    await sharedDescriptorManifestV1("demonstrator", "/b.js", fetchManifest);
    await sharedDescriptorManifestV1("demonstrator", "/a.js", fetchManifest);
    expect(requests).toBe(3);
  });

  /** 🔐️ LAW: sharing is per PLUGIN and url, never per url alone — the fetch's "this descriptor names
   * the plugin I asked for" check is per-caller, so a second plugin must get its own checked answer. */
  it("never hands one plugin's descriptor to another plugin on the same url", async () => {
    const asked: string[] = [];
    const panes = await Promise.all(["demonstrator", "impostor"].map((pluginId) => sharedDescriptorManifestV1(pluginId, "/shared.js", async () => { asked.push(pluginId); return pluginId; })));
    expect(asked.sort()).toEqual(["demonstrator", "impostor"]);
    expect(panes).toEqual(["demonstrator", "impostor"]);
  });

  it("settles every waiter of a failed request and retries on the next one", async () => {
    let requests = 0;
    const failing = async () => { requests += 1; throw new Error("plugin.descriptor-unavailable"); };
    const waiters = [sharedDescriptorManifestV1("demonstrator", "/a.js", failing), sharedDescriptorManifestV1("demonstrator", "/a.js", failing)];
    for (const waiter of waiters) await expect(waiter).rejects.toThrow("plugin.descriptor-unavailable");
    expect(requests).toBe(1);
    await expect(sharedDescriptorManifestV1("demonstrator", "/a.js", failing)).rejects.toThrow("plugin.descriptor-unavailable");
    expect(requests).toBe(2);
  });
});

describe("descriptor fetch progress", () => {
  const descriptor = JSON.stringify({ manifest: { pluginId: "demonstrator", apps: [] } });

  const streamingResponse = (chunks: readonly string[]): Response => {
    const encoder = new TextEncoder();
    let index = 0;
    return {
      ok: true,
      headers: { get: () => "application/json" },
      body: { getReader: () => ({ read: async () => (index < chunks.length ? { done: false, value: encoder.encode(chunks[index++]!) } : { done: true, value: undefined }) }) },
      text: async () => chunks.join(""),
    } as unknown as Response;
  };

  const withFetch = async <T,>(response: Response, run: () => Promise<T>): Promise<T> => {
    const original = globalThis.fetch;
    globalThis.fetch = stubFetch(async () => response);
    try { return await run(); } finally { globalThis.fetch = original; }
  };

  /** 📡️ LAW: a descriptor read that streams for minutes must beat per chunk — `response.text()` is
   * one opaque await, and the deadline timing it cannot tell a slow body from a dead connection. */
  it("reports the headers and every streamed chunk as progress", async () => {
    const chunks = [descriptor.slice(0, 10), descriptor.slice(10, 25), descriptor.slice(25)];
    let beats = 0;
    const manifest = await withFetch(streamingResponse(chunks), () => fetchDescriptorManifest("demonstrator", "/plugin-modules/demonstrator/bridge.js", undefined, () => { beats += 1; }));
    expect(manifest.pluginId).toBe("demonstrator");
    expect(beats).toBe(1 + chunks.length);
  });

  it("still reads a body with no reader, and still beats once for the headers", async () => {
    const unreadable = { ok: true, headers: { get: () => "application/json" }, text: async () => descriptor } as unknown as Response;
    let beats = 0;
    const manifest = await withFetch(unreadable, () => fetchDescriptorManifest("demonstrator", "/plugin-modules/demonstrator/bridge.js", undefined, () => { beats += 1; }));
    expect(manifest.pluginId).toBe("demonstrator");
    expect(beats).toBe(1);
  });

  /** 🔤️ A chunk boundary is a BYTE boundary, and this repo's descriptors are full of multi-byte emoji
   * app ids — the streaming read must decode across chunks, never chunk by chunk. */
  it("keeps a descriptor split mid-emoji intact", async () => {
    const payload = JSON.stringify({ manifest: { pluginId: "demonstrator", apps: [{ id: "🧺️demonstrator" }] } });
    const raw = new TextEncoder().encode(payload);
    const split = payload.indexOf("🧺") + 1;
    const halves = [raw.slice(0, split), raw.slice(split)];
    let index = 0;
    let beats = 0;
    const response = {
      ok: true,
      headers: { get: () => "application/json" },
      body: { getReader: () => ({ read: async () => (index < halves.length ? { done: false, value: halves[index++]! } : { done: true, value: undefined }) }) },
    } as unknown as Response;
    const manifest = await withFetch(response, () => fetchDescriptorManifest("demonstrator", "/plugin-modules/demonstrator/bridge.js", undefined, () => { beats += 1; }));
    expect((manifest.apps as readonly { readonly id: string }[])[0]!.id).toBe("🧺️demonstrator");
    expect(beats).toBe(3);
  });
});
