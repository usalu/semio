/** 🧲️ TypeScript twin of the engine-surface RETENTION law.
 *
 * Two independent jobs, neither of which the Rust law can do:
 *
 * 1. It re-derives every authored case a SECOND time, from the same
 *    `🧫️fixtures/🧲️engine-surface-retention/🔣️.json` oracle, through a reference mirror written
 *    against the fixture's declared rules rather than against the wgpu shell's source. Two
 *    implementations that disagree mean the oracle is under-specified.
 * 2. It pins the rule to the renderer that ALREADY held it: React's node-graph host effect is keyed
 *    on `[surfaceId]` alone and its cleanup is documented as unmounting exactly once, when the
 *    window closes — never on a render or a paint. The wgpu shell now says the same thing with
 *    `EngineSurfaceRegistration.window_id`. This test reads both sources, so a change on either side
 *    fails here instead of silently splitting the two renderers.
 *
 * Rust law: `🧱️elements/🐚️Shell/🧪️tests/🧲️engine-surface-retention/🦀️.rs`.
 */
import { describe, expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join } from "node:path";

const engineRoot = join(import.meta.dir, "..", "..");
const law = JSON.parse(readFileSync(join(engineRoot, "🧫️fixtures", "🧲️engine-surface-retention", "🔣️.json"), "utf8"));

type Bounds = readonly [number, number, number, number];
type Drain = { surfaceId: string; windowId: string; kind: string; controllerId: string; bounds: Bounds; created?: boolean };
type Frame = { drain: Drain[]; liveWindows: string[] };
type Retained = { surfaceId: string; windowId: string; kind: string; controllerId: string; bounds: Bounds };

/** 🧲️ The reference mirror, written from the fixture's `rules` and from nothing else. */
const mirror = (state: Retained[], frame: Frame): { state: Retained[]; created: string[] } => {
  // ownerIsTheWindowInstance + aClosedWindowDropsItsSurface
  const kept = state.filter((surface) => frame.liveWindows.includes(surface.windowId));
  const created: string[] = [];
  // aRepaintRefreshesTheProjection + creationIsAnnouncedOnce; everything not mentioned is untouched
  // (aFrameWithoutARepaintKeepsTheSurface).
  for (const entry of frame.drain) {
    if (entry.created) created.push(entry.surfaceId);
    if (entry.kind === "world3d") continue;
    const existing = kept.findIndex((surface) => surface.surfaceId === entry.surfaceId);
    const next: Retained = { surfaceId: entry.surfaceId, windowId: entry.windowId, kind: entry.kind, controllerId: entry.controllerId, bounds: entry.bounds };
    if (existing >= 0) kept[existing] = next;
    else kept.push(next);
  }
  return { state: kept, created };
};

const resolve = (state: Retained[], x: number, y: number): Retained | undefined => state.find(({ bounds: [bx, by, bw, bh] }) => x >= bx && x < bx + bw && y >= by && y < by + bh);

describe("🧲️ engine surfaces are retained by their window instance, not by a painted frame", () => {
  test("every authored case is declared with the rules it exercises", () => {
    expect(law.cases.length).toBe(6);
    expect(Object.keys(law.rules)).toContain("aFrameWithoutARepaintKeepsTheSurface");
    expect(law.provenance.implementations.rust).toContain("🧲️engine-surface-retention");
  });

  for (const authored of law.cases as Array<Record<string, never> & { name: string; frames: Frame[]; expectedResolve: Array<{ at: [number, number]; surfaceId: string | null; controllerId?: string }>; expectedCreated?: string[]; expectedCreatedPerFrame?: string[][] }>) {
    test(authored.name, () => {
      let state: Retained[] = [];
      const createdPerFrame: string[][] = [];
      for (const frame of authored.frames) {
        const stepped = mirror(state, frame);
        state = stepped.state;
        createdPerFrame.push(stepped.created);
      }
      if (authored.expectedCreatedPerFrame) expect(createdPerFrame).toEqual(authored.expectedCreatedPerFrame);
      if (authored.expectedCreated) expect(createdPerFrame[createdPerFrame.length - 1] ?? []).toEqual(authored.expectedCreated);
      for (const probe of authored.expectedResolve) {
        const resolved = resolve(state, probe.at[0], probe.at[1]);
        expect(resolved?.surfaceId ?? null).toBe(probe.surfaceId);
        if (probe.controllerId) expect(resolved?.controllerId).toBe(probe.controllerId);
      }
    });
  }

  test("React already ties a node-graph host's lifetime to its window, never to a paint", () => {
    const source = readFileSync(join(engineRoot, "🧱️elements", "🕸️NodeGraph", "🟦️.tsx"), "utf8");
    const unmount = source.indexOf('console.log("[DEBUG] node-graph host unmount surface=%s", surfaceId);');
    expect(unmount).toBeGreaterThan(0);
    // The effect this cleanup belongs to closes on `[surfaceId]` — no scene, no paint, no refresh
    // signal — which is the same retention authority the wgpu shell now names `window_id`.
    expect(source.slice(unmount, unmount + 600)).toContain("}, [surfaceId]);");
  });

  test("the wgpu shell names the same authority and retires on it alone", () => {
    const shell = readFileSync(join(engineRoot, "🧱️elements", "🐚️Shell", "🎯️targets", "🧊️wgpu", "🦀️.rs"), "utf8");
    expect(shell).toContain("fn mirror_engine_surface_states(");
    expect(shell).toContain("live_window_ids.contains(&surface.window_id.as_str())");
    const registration = readFileSync(join(engineRoot, "🧱️elements", "⚙️EngineCanvas", "🎯️targets", "🧊️wgpu", "🦀️.rs"), "utf8");
    expect(registration).toContain("pub struct EngineSurfaceRegistration {");
    expect(registration.slice(registration.indexOf("pub struct EngineSurfaceRegistration {"), registration.indexOf("pub struct EngineSurfaceRegistration {") + 400)).toContain("pub window_id: String,");
  });
});
