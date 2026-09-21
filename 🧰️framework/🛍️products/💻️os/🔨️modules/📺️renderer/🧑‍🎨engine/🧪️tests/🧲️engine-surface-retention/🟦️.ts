/** 🧲️ TypeScript twin of the engine-surface RETENTION law.
 *
 * Two independent jobs, neither of which the Rust law can do:
 *
 * 1. It re-derives every authored case a SECOND time, from the same
 *    `🧫️fixtures/🧲️engine-surface-retention/🔣️.json` oracle, through a reference mirror written
 *    against the fixture's declared rules rather than against the wgpu shell's source. Two
 *    implementations that disagree mean the oracle is under-specified.
 * 2. Chromium exercises retained pointer and wheel delivery against the declared owners.
 *    The mounted React host lifetime is exercised in `🔬️engine-contract/🟦️.ts`, including
 *    session retention through refreshes and exactly one release when the host unmounts.
 *
 * Rust law: `🧱️elements/🐚️Shell/🧪️tests/🧲️engine-surface-retention/🦀️.rs`.
 */
import { describe, expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join } from "node:path";

test("Chromium retires every pointer capture with its removed scene and admits a fresh owner", async () => {
  const fixture = JSON.parse(readFileSync(join(import.meta.dir, "../../🧫️fixtures/🪪️scene-pointer-owner/🔣️.json"), "utf8"));
  const { chromium } = await import("playwright");
  const browser = await chromium.launch({ headless: true });
  try {
    const page = await browser.newPage({ viewport: { width: 500, height: 360 }, hasTouch: true });
    await page.setContent('<style>body{margin:0;touch-action:none}canvas{width:500px;height:360px}</style><canvas></canvas>');
    await page.evaluate(() => {
      const canvas = document.querySelector("canvas")!;
      const pointers: number[] = [];
      Object.assign(window, { oldCapture: canvas, capturePointers: pointers });
      canvas.addEventListener("pointerdown", (event) => {
        pointers.push(event.pointerId);
        canvas.setPointerCapture(event.pointerId);
      });
    });
    const cdp = await page.context().newCDPSession(page);
    const touches = Array.from({ length: fixture.captureCapacity }, (_, id) => ({ id, x: 100 + id, y: 100 }));
    await cdp.send("Input.dispatchTouchEvent", { type: "touchStart", touchPoints: touches });
    expect(await page.evaluate(() => {
      const state = window as unknown as { oldCapture: HTMLCanvasElement; capturePointers: number[] };
      return state.capturePointers.filter((pointer) => state.oldCapture.hasPointerCapture(pointer)).length;
    })).toBe(fixture.captureCapacity);
    await page.evaluate(() => {
      const state = window as unknown as { oldCapture: HTMLCanvasElement };
      const fresh = document.createElement("canvas");
      fresh.addEventListener("pointerdown", (event) => {
        fresh.setPointerCapture(event.pointerId);
        Object.assign(window, { freshCaptured: fresh.hasPointerCapture(event.pointerId) });
      });
      state.oldCapture.replaceWith(fresh);
    });
    await cdp.send("Input.dispatchTouchEvent", { type: "touchCancel", touchPoints: [] });
    expect(await page.evaluate(() => {
      const state = window as unknown as { oldCapture: HTMLCanvasElement; capturePointers: number[] };
      return state.capturePointers.some((pointer) => state.oldCapture.hasPointerCapture(pointer));
    })).toBe(false);
    await page.mouse.move(100, 100);
    await page.mouse.down();
    expect(await page.evaluate(() => (window as unknown as { freshCaptured: boolean }).freshCaptured)).toBe(true);
    await page.mouse.up();
    console.info("[DEBUG] Chromium retired sixteen removed-node pointer captures and accepted the successor");
  } finally {
    await browser.close();
  }
});

test("Chromium captures the topmost scene through release outside every scene", async () => {
  const { default: Ajv } = await import("ajv/dist/2020.js");
  const fixtureRoot = join(import.meta.dir, "../../🧫️fixtures/🪪️scene-pointer-owner");
  const fixture = JSON.parse(readFileSync(join(fixtureRoot, "🔣️.json"), "utf8"));
  const validate = new Ajv({ strict: true }).compile(JSON.parse(readFileSync(join(fixtureRoot, "📐️schema.json"), "utf8")));
  expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  const { chromium } = await import("playwright");
  const browser = await chromium.launch({ headless: true });
  try {
    const page = await browser.newPage({ viewport: { width: 500, height: 360 } });
    const cdp = await page.context().newCDPSession(page);
    for (const scenario of fixture.cases) {
      await page.setContent('<style>body{margin:0;touch-action:none}canvas{position:absolute}</style>');
      await page.evaluate(({ surfaces, moves }) => {
        const events: { surface: string; type: string }[] = [];
        const deltas: [number, number][] = [];
        const positions = new Map<number, [number, number]>();
        Object.assign(window, { capturedSceneEvents: events, sceneMotionDeltas: deltas, sceneMenuOpen: false, scenePointer: 0 });
        for (const surface of surfaces) {
          const canvas = document.createElement("canvas");
          const [left, top, width, height] = surface.bounds;
          canvas.dataset.surface = surface.surfaceId;
          Object.assign(canvas.style, { left: left + "px", top: top + "px", width: width + "px", height: height + "px" });
          canvas.addEventListener("pointerdown", (event) => {
            canvas.setPointerCapture(event.pointerId);
            positions.set(event.pointerId, [event.clientX, event.clientY]);
            Object.assign(window, { scenePointer: event.pointerId });
            events.push({ surface: surface.surfaceId, type: event.type });
          });
          canvas.addEventListener("pointermove", (event) => {
            if (moves && event.buttons !== 0) {
              const previous = positions.get(event.pointerId)!;
              deltas.push([event.clientX - previous[0], event.clientY - previous[1]]);
              positions.set(event.pointerId, [event.clientX, event.clientY]);
              events.push({ surface: surface.surfaceId, type: event.type });
            }
          });
          for (const name of ["pointerup", "wheel"]) canvas.addEventListener(name, (event) => events.push({ surface: surface.surfaceId, type: event.type }));
          canvas.addEventListener("contextmenu", (event) => {
            event.preventDefault();
            Object.assign(window, { sceneMenuOpen: true });
          });
          document.body.append(canvas);
        }
      }, { surfaces: fixture.surfaces as { surfaceId: string; bounds: number[] }[], moves: scenario.steps.includes("move") });
      let outside = false;
      for (const step of scenario.steps) {
        if (step === "chrome" || step === "modal") {
          await page.evaluate((modal) => {
            const button = document.createElement("button");
            Object.assign(button.style, { position: "absolute", inset: "0", zIndex: "10" });
            if (modal) button.addEventListener("pointerdown", () => button.remove());
            document.body.append(button);
          }, step === "modal");
        } else if (step === "press" || step === "secondaryPress") {
          await page.mouse.move(fixture.press[0], fixture.press[1]);
          await page.mouse.down({ button: step === "secondaryPress" ? "right" : "left" });
          outside = false;
        } else if (step === "move") {
          await page.mouse.move(fixture.release[0], fixture.release[1]);
          outside = true;
        } else if (step === "release" || step === "secondaryRelease") {
          if (!outside) await page.mouse.move(fixture.release[0], fixture.release[1]);
          await page.mouse.up({ button: step === "secondaryRelease" ? "right" : "left" });
          outside = true;
        } else if (step === "wheel") {
          await page.mouse.move(fixture.press[0], fixture.press[1]);
          await page.mouse.wheel(0, 20);
          await page.waitForFunction(() => (window as unknown as { capturedSceneEvents: { type: string }[] }).capturedSceneEvents.some((event) => event.type === "wheel"));
        } else if (step === "foreignPress") {
          await cdp.send("Input.dispatchTouchEvent", { type: "touchStart", touchPoints: [{ id: 78, x: fixture.release[0], y: fixture.release[1] }] });
        } else if (step === "foreignRelease" || step === "foreignCancel") {
          await cdp.send("Input.dispatchTouchEvent", { type: step === "foreignRelease" ? "touchEnd" : "touchCancel", touchPoints: [] });
        } else if (step === "refresh" || step === "replace") {
          await page.evaluate(({ owner, replace }) => {
            const canvas = document.querySelector(`canvas[data-surface="${owner}"]`)!;
            if (replace) canvas.replaceWith(canvas.cloneNode());
            else canvas.setAttribute("data-scene-revision", "2");
          }, { owner: fixture.expectedOwner as string, replace: step === "replace" });
        } else if (step === "cancel") {
          await page.evaluate(() => {
            const pointer = (window as unknown as { scenePointer: number }).scenePointer;
            for (const canvas of document.querySelectorAll("canvas")) if (canvas.hasPointerCapture(pointer)) canvas.releasePointerCapture(pointer);
          });
        }
      }
      const observed = await page.evaluate(() => ({
        events: (window as unknown as { capturedSceneEvents: { surface: string; type: string }[] }).capturedSceneEvents,
        menuOpen: (window as unknown as { sceneMenuOpen: boolean }).sceneMenuOpen,
        deltas: (window as unknown as { sceneMotionDeltas: [number, number][] }).sceneMotionDeltas,
      }));
      expect(observed.events, scenario.name).toEqual(scenario.events.map((type: string) => ({ surface: fixture.expectedOwner, type })));
      expect(observed.menuOpen, scenario.name).toBe(scenario.menuOpen);
      expect(observed.deltas, scenario.name).toEqual(scenario.events.includes("pointermove") ? [fixture.moveDelta] : []);
    }
    console.log("[DEBUG] Chromium matched captured scene, chrome, modal, wheel, secondary, cancelled, refreshed, and replaced owner sequences");
  } finally {
    await browser.close();
  }
});

test("Chromium routes retained scene pointer and wheel events through the same live window owners", async () => {
  const { chromium } = await import("playwright");
  const browser = await chromium.launch({ headless: true });
  try {
    const page = await browser.newPage({ viewport: { width: 1600, height: 1200 } });
    for (const authored of law.cases as { name: string; frames: Frame[]; expectedResolve: { at: [number, number]; surfaceId: string | null }[] }[]) {
      let retained: Retained[] = [];
      for (const frame of authored.frames) retained = mirror(retained, frame).state;
      await page.setContent('<style>body{margin:0;overflow:hidden}canvas{position:absolute}</style>');
      await page.evaluate((rows) => {
        const events: { surfaceId: string; windowId: string; type: string }[] = [];
        Object.assign(window, { sceneEvents: events });
        for (const row of rows) {
          const host = document.createElement("div");
          host.dataset.window = row.windowId;
          const canvas = document.createElement("canvas");
          canvas.dataset.surface = row.surfaceId;
          const [x, y, width, height] = row.bounds;
          Object.assign(canvas.style, { left: x + "px", top: y + "px", width: width + "px", height: height + "px" });
          for (const type of ["pointerdown", "wheel"]) canvas.addEventListener(type, () => events.push({ surfaceId: row.surfaceId, windowId: host.dataset.window!, type }));
          host.append(canvas);
          document.body.append(host);
        }
      }, retained);
      for (const probe of authored.expectedResolve) {
        if (!probe.surfaceId) continue;
        const owner = retained.find((row) => row.surfaceId === probe.surfaceId)!;
        await page.evaluate(() => { (window as unknown as { sceneEvents: unknown[] }).sceneEvents.length = 0; });
        await page.mouse.click(...probe.at);
        await page.mouse.wheel(0, 20);
        await page.waitForFunction(() => (window as unknown as { sceneEvents: unknown[] }).sceneEvents.length === 2);
        expect(await page.evaluate(() => (window as unknown as { sceneEvents: unknown[] }).sceneEvents)).toEqual([
          { surfaceId: probe.surfaceId, windowId: owner.windowId, type: "pointerdown" },
          { surfaceId: probe.surfaceId, windowId: owner.windowId, type: "wheel" },
        ]);
      }
    }
    await page.close();
  } finally {
    await browser.close();
  }
}, 30000);

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

  test("the wgpu shell names the same authority and retires on it alone", () => {
    const shell = readFileSync(join(engineRoot, "🧱️elements", "🐚️Shell", "🎯️targets", "🧊️wgpu", "🦀️.rs"), "utf8");
    expect(shell).toContain("fn mirror_engine_surface_states(");
    expect(shell).toContain("live_window_ids.contains(&surface.window_id.as_str())");
    const registration = readFileSync(join(engineRoot, "🧱️elements", "⚙️EngineCanvas", "🎯️targets", "🧊️wgpu", "🦀️.rs"), "utf8");
    expect(registration).toContain("pub struct EngineSurfaceRegistration {");
    expect(registration.slice(registration.indexOf("pub struct EngineSurfaceRegistration {"), registration.indexOf("pub struct EngineSurfaceRegistration {") + 400)).toContain("pub window_id: String,");
  });
});
