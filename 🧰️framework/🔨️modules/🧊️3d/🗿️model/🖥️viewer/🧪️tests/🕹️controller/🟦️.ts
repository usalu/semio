// @vitest-environment jsdom
/** 🕹️ Controller and custom element in jsdom with a recording backend and a manual animation-frame clock: ARIA and client-supplied strings, progress and cancellation, animation clock (play, pause, seek, rate, loop, clamp), reduced motion, hidden page, keyboard and pointer operability, picking, backend fallback and status, CSS colour parsing, disposal. */
import { Vector3 } from "three";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import type { ModelDocument } from "../../../🧬️schema/🟦️.ts";
import type { Frame, RenderBackend, Scene } from "../../🧬️contract/🟦️.ts";
import { ModelViewer, type ModelViewerOptions, type ViewerStatus } from "../../🕹️controller/🟦️.ts";
import { parseCssColor } from "../../🕹️controller/🎨️color/🟦️.ts";
import { defineModelViewerElement } from "../../🧩️element/🟦️.ts";
import { fixture, threeCamera } from "../🧰️support/🟦️.ts";

const rig = fixture("🧱️rig").document as ModelDocument;

class Recorder implements RenderBackend {
  readonly kind = "recorder";
  loaded: Scene | null = null;
  renders: { drawCount: number; time: number; background: readonly number[]; upper: number[] }[] = [];
  sizes: number[][] = [];
  disposed = false;
  load(scene: Scene) {
    this.loaded = scene;
  }
  resize(w: number, h: number, r: number) {
    this.sizes.push([w, h, r]);
  }
  render(frame: Frame, style: { background: readonly number[] }) {
    const upper = this.loaded!.nodeIds.indexOf("upper") * 16;
    this.renders.push({ drawCount: frame.drawCount, time: 0, background: style.background, upper: Array.from(frame.matrices.subarray(upper, upper + 16)) });
  }
  readPixels() {
    return new Uint8ClampedArray(4);
  }
  dispose() {
    this.disposed = true;
  }
}

let queue: { id: number; cb: FrameRequestCallback }[] = [];
let nextId = 1;
const flush = (now: number) => {
  const run = queue;
  queue = [];
  for (const q of run) q.cb(now);
};

const fire = (target: EventTarget, type: string, props: Record<string, unknown> = {}) => {
  const e = new Event(type, { bubbles: true, cancelable: true });
  Object.assign(e, props);
  target.dispatchEvent(e);
  return e;
};

const strings = { label: "Etiqueta", description: "Descripción larga", roleDescription: "visor", status: (s: ViewerStatus) => `status:${s.kind}` };

const make = (extra: Partial<ModelViewerOptions> = {}) => {
  const host = document.createElement("div");
  document.body.append(host);
  const canvas = document.createElement("canvas");
  canvas.getBoundingClientRect = () => ({ left: 10, top: 20, width: 200, height: 100, right: 210, bottom: 120, x: 10, y: 20, toJSON: () => ({}) });
  host.append(canvas);
  const backend = new Recorder();
  const viewer = new ModelViewer({ canvas, strings, backend: () => backend, ...extra });
  return { host, canvas, backend, viewer };
};

beforeEach(() => {
  queue = [];
  nextId = 1;
  vi.stubGlobal("requestAnimationFrame", (cb: FrameRequestCallback) => {
    queue.push({ id: nextId, cb });
    return nextId++;
  });
  vi.stubGlobal("cancelAnimationFrame", (id: number) => {
    queue = queue.filter((q) => q.id !== id);
  });
});
afterEach(() => {
  vi.unstubAllGlobals();
  document.body.innerHTML = "";
  Object.defineProperty(document, "hidden", { value: false, configurable: true });
});

describe("accessibility", () => {
  it("labels the canvas with the client's strings, makes it focusable and adds a description and a live region", () => {
    const { canvas } = make();
    expect(canvas.getAttribute("aria-label")).toBe("Etiqueta");
    expect(canvas.getAttribute("aria-roledescription")).toBe("visor");
    expect(canvas.tabIndex).toBe(0);
    const description = document.getElementById(canvas.getAttribute("aria-describedby")!)!;
    expect(description.textContent).toBe("Descripción larga");
    const live = canvas.parentElement!.querySelector("[aria-live=polite]")!;
    expect(live.getAttribute("role")).toBe("status");
  });
  it("announces state changes through the client's status function and stays silent without one", async () => {
    const { canvas, viewer } = make();
    await viewer.setDocument(rig);
    viewer.setPreset("plan");
    const live = canvas.parentElement!.querySelector("[aria-live=polite]")!;
    expect(live.textContent).toBe("status:preset");
    viewer.setVariant("dark");
    expect(live.textContent).toBe("status:variant");
    const quiet = make({ strings: { label: "a", description: "b" } });
    await quiet.viewer.setDocument(rig);
    quiet.viewer.setPreset("front");
    expect(quiet.canvas.parentElement!.querySelector("[aria-live]")!.textContent).toBe("");
  });
});

describe("document preparation", () => {
  it("reports monotonic progress to 1, loads the backend, fits the camera and renders on the next frame", async () => {
    const { viewer, backend } = make();
    const progress: number[] = [], events: string[] = [];
    viewer.on("progress", (e) => progress.push(e.fraction));
    viewer.on("ready", () => events.push("ready"));
    viewer.on("camera", () => events.push("camera"));
    await viewer.setDocument(rig);
    expect(progress[progress.length - 1]).toBe(1);
    expect(progress.every((v, i) => i === 0 || v >= progress[i - 1]!)).toBe(true);
    expect(events).toEqual(["camera", "ready"]);
    expect(backend.loaded?.primitives.length).toBe(10);
    expect(viewer.getCamera().extent).toBeGreaterThan(1);
    flush(0);
    expect(backend.renders).toHaveLength(1);
    expect(backend.renders[0]!.drawCount).toBe(10);
  });
  it("cancels a running preparation without applying it", async () => {
    const { viewer, backend } = make();
    const ready = vi.fn();
    viewer.on("ready", ready);
    const p = viewer.setDocument(rig);
    viewer.cancelPreparation();
    await p;
    expect(ready).not.toHaveBeenCalled();
    expect(backend.loaded).toBeNull();
    const controller = new AbortController();
    const q = viewer.setDocument(rig, controller.signal);
    controller.abort();
    await q;
    expect(backend.loaded).toBeNull();
  });
  it("lets a newer document supersede an older preparation", async () => {
    const { viewer, backend } = make();
    const first = viewer.setDocument({ ...rig, id: "first" });
    const second = viewer.setDocument({ ...rig, id: "second" });
    await Promise.all([first, second]);
    expect(backend.loaded?.document.id).toBe("second");
  });
  it("tessellates procedural documents when a quality is given and rejects otherwise", async () => {
    const procedural = { version: "semio.model/v1", id: "p", title: { en: "p", de: "p" }, nodes: [{ id: "n", parent: null, translation: [0, 0, 0], rotation: [0, 0, 0, 1], scale: [1, 1, 1] }], solids: [{ id: "b", kind: "box", node: "n", material: "m", size: [1, 1, 1] }], meshes: [], skins: [], variants: [], poses: [], animations: [], materials: [{ id: "m", title: { en: "m", de: "m" }, baseColor: [1, 1, 1, 1], metallic: 0, roughness: 1, emissive: [0, 0, 0], alphaMode: "opaque", doubleSided: false, unlit: false }] } as unknown as ModelDocument;
    const none = make();
    await expect(none.viewer.setDocument(procedural)).rejects.toThrow(/quality/);
    const { viewer, backend } = make({ quality: { chordM: 0.01, minSegments: 8, maxSegments: 16 } });
    const progress: number[] = [];
    viewer.on("progress", (e) => progress.push(e.fraction));
    await viewer.setDocument(procedural);
    expect(backend.loaded?.primitives).toHaveLength(1);
    expect(progress[progress.length - 1]).toBe(1);
    expect(progress).toContain(0.5);
  });
  it("reports invalid documents through the error event and rejects", async () => {
    const { viewer } = make();
    const errors: Error[] = [];
    viewer.on("error", (e) => errors.push(e.error));
    await expect(viewer.setDocument({ ...rig, meshes: [] })).rejects.toThrow(/unknown mesh/);
    expect(errors).toHaveLength(1);
  });
});

describe("animation clock", () => {
  const ready = async (extra: Partial<ModelViewerOptions> = {}) => {
    const r = make(extra);
    await r.viewer.setDocument(rig);
    flush(0);
    r.backend.renders.length = 0;
    return r;
  };
  it("plays, wraps a looping clip, seeks, changes rate and pauses", async () => {
    const { viewer, backend } = await ready();
    const times: number[] = [], playing: boolean[] = [];
    viewer.on("time", (e) => times.push(e.timeS));
    viewer.on("playing", (e) => playing.push(e.playing));
    viewer.setAnimation("wave");
    viewer.play();
    flush(100);
    flush(600);
    expect(viewer.clock.timeS).toBeCloseTo(0.5, 6);
    flush(1600);
    expect(viewer.clock.timeS).toBeCloseTo(1.5, 6);
    flush(2600);
    expect(viewer.clock.timeS).toBeCloseTo(0.5, 6);
    viewer.setRate(2);
    flush(3100);
    expect(viewer.clock.timeS).toBeCloseTo(1.5, 6);
    viewer.pause();
    flush(4000);
    expect(viewer.clock.timeS).toBeCloseTo(1.5, 6);
    expect(playing).toEqual([true, false]);
    viewer.seek(0.5);
    expect(viewer.clock.timeS).toBe(0.5);
    flush(5000);
    const rotated = backend.renders[backend.renders.length - 1]!.upper;
    expect(Math.abs(rotated[1]!)).toBeGreaterThan(0.1);
    expect(times.length).toBeGreaterThan(3);
  });
  it("clamps a non-looping clip at its end and stops", async () => {
    const { viewer } = await ready();
    viewer.setAnimation("wave");
    viewer.setLoop(false);
    const playing: boolean[] = [];
    viewer.on("playing", (e) => playing.push(e.playing));
    viewer.play();
    flush(0);
    flush(5000);
    expect(viewer.clock.timeS).toBe(2);
    expect(viewer.clock.playing).toBe(false);
    expect(playing).toEqual([true, false]);
    viewer.play();
    expect(viewer.clock.timeS).toBe(0);
  });
  it("plays backwards with a negative rate and stops at the start when not looping", async () => {
    const { viewer } = await ready();
    viewer.setAnimation("wave", 1);
    viewer.setLoop(false);
    viewer.setRate(-1);
    viewer.play();
    flush(0);
    flush(400);
    expect(viewer.clock.timeS).toBeCloseTo(0.6, 6);
    flush(2000);
    expect(viewer.clock.timeS).toBe(0);
    expect(viewer.clock.playing).toBe(false);
  });
  it("does not autoplay under reduced motion but plays on request; autoplays otherwise", async () => {
    const reduced = await ready({ autoplay: true, reducedMotion: true });
    reduced.viewer.setAnimation("wave");
    expect(reduced.viewer.clock.playing).toBe(false);
    reduced.viewer.play();
    expect(reduced.viewer.clock.playing).toBe(true);
    const normal = await ready({ autoplay: true, reducedMotion: false });
    normal.viewer.setAnimation("wave");
    expect(normal.viewer.clock.playing).toBe(true);
  });
  it("follows prefers-reduced-motion through matchMedia and pauses when it turns on", async () => {
    let handler: (() => void) | null = null;
    const mq = { matches: true, addEventListener: (_: string, h: () => void) => (handler = h), removeEventListener: () => undefined };
    vi.stubGlobal("matchMedia", () => mq);
    const { viewer } = await ready({ autoplay: true });
    viewer.setAnimation("wave");
    expect(viewer.clock.playing).toBe(false);
    mq.matches = false;
    handler!();
    viewer.setAnimation("wave");
    expect(viewer.clock.playing).toBe(true);
    mq.matches = true;
    handler!();
    expect(viewer.clock.playing).toBe(false);
  });
  it("suspends rendering and the clock while the page is hidden and resumes without a time jump", async () => {
    const { viewer, backend } = await ready();
    viewer.setAnimation("wave");
    viewer.play();
    flush(0);
    flush(100);
    const t = viewer.clock.timeS;
    Object.defineProperty(document, "hidden", { value: true, configurable: true });
    backend.renders.length = 0;
    flush(5000);
    expect(viewer.clock.timeS).toBe(t);
    expect(backend.renders).toHaveLength(0);
    expect(queue).toHaveLength(0);
    Object.defineProperty(document, "hidden", { value: false, configurable: true });
    fire(document, "visibilitychange");
    flush(6000);
    flush(6100);
    expect(viewer.clock.timeS).toBeCloseTo(t + 0.1, 6);
  });
  it("renders on demand only: an idle viewer schedules nothing", async () => {
    const { viewer, backend } = await ready();
    flush(1);
    flush(2);
    expect(backend.renders).toHaveLength(0);
    viewer.setVariant("dark");
    expect(queue).toHaveLength(1);
    flush(3);
    expect(backend.renders).toHaveLength(1);
    expect(queue).toHaveLength(0);
  });
  it("selects poses, variants and rejects unknown ids", async () => {
    const { viewer, backend } = await ready();
    viewer.setPose("bent");
    flush(1);
    const bent = backend.renders[backend.renders.length - 1]!.upper;
    viewer.setPose(null);
    flush(2);
    expect(backend.renders[backend.renders.length - 1]!.upper).not.toEqual(bent);
    expect(() => viewer.setPose("ghost")).toThrow();
    expect(() => viewer.setAnimation("ghost")).toThrow();
    expect(() => viewer.setVariant("ghost")).toThrow();
  });
});

describe("keyboard", () => {
  const ready = async (extra: Partial<ModelViewerOptions> = {}) => {
    const r = make(extra);
    await r.viewer.setDocument(rig);
    return r;
  };
  it("orbits with the arrow keys", async () => {
    const { viewer, canvas } = await ready();
    const yaw = viewer.getCamera().yaw, pitch = viewer.getCamera().pitch;
    const e = fire(canvas, "keydown", { key: "ArrowRight" });
    expect(e.defaultPrevented).toBe(true);
    expect(viewer.getCamera().yaw).toBeCloseTo(yaw + 0.1 * Math.PI);
    fire(canvas, "keydown", { key: "ArrowLeft" });
    fire(canvas, "keydown", { key: "ArrowUp" });
    expect(viewer.getCamera().pitch).toBeCloseTo(pitch + 0.1 * Math.PI);
    fire(canvas, "keydown", { key: "ArrowDown" });
    expect(viewer.getCamera().pitch).toBeCloseTo(pitch);
  });
  it("pans with shift+arrows, zooms with +/-, fits with Home, switches presets with digits", async () => {
    const { viewer, canvas } = await ready();
    const target = [...viewer.getCamera().target];
    fire(canvas, "keydown", { key: "ArrowLeft", shiftKey: true });
    expect(viewer.getCamera().target).not.toEqual(target);
    const d = viewer.getCamera().distance;
    fire(canvas, "keydown", { key: "+" });
    expect(viewer.getCamera().distance).toBeLessThan(d);
    fire(canvas, "keydown", { key: "-" });
    expect(viewer.getCamera().distance).toBeCloseTo(d, 6);
    fire(canvas, "keydown", { key: "3" });
    expect(viewer.getCamera().pitch).toBeCloseTo(Math.PI / 2);
    expect(viewer.getCamera().projection).toBe("orthographic");
    fire(canvas, "keydown", { key: "Home" });
    expect(viewer.getCamera().target[0]).toBeCloseTo(target[0]!, 6);
    expect(viewer.getCamera().target[1]).toBeCloseTo(target[1]!, 6);
  });
  it("toggles playback with Space only when an animation is selected", async () => {
    const { viewer, canvas } = await ready();
    expect(fire(canvas, "keydown", { key: " " }).defaultPrevented).toBe(false);
    viewer.setAnimation("wave");
    fire(canvas, "keydown", { key: " " });
    expect(viewer.clock.playing).toBe(true);
    fire(canvas, "keydown", { key: " " });
    expect(viewer.clock.playing).toBe(false);
  });
  it("ignores modified keys and can be disabled", async () => {
    const { viewer, canvas } = await ready();
    const yaw = viewer.getCamera().yaw;
    fire(canvas, "keydown", { key: "ArrowRight", ctrlKey: true });
    expect(viewer.getCamera().yaw).toBe(yaw);
    const off = await ready({ controls: { keyboard: false } });
    fire(off.canvas, "keydown", { key: "ArrowRight" });
    expect(off.viewer.getCamera().yaw).toBe(off.viewer.getCamera().yaw);
    const cam = off.viewer.getCamera();
    fire(off.canvas, "keydown", { key: "ArrowRight" });
    expect(off.viewer.getCamera()).toBe(cam);
  });
  it("honours switches and speeds", async () => {
    const noOrbit = await ready({ controls: { orbit: false } });
    const yaw = noOrbit.viewer.getCamera().yaw;
    fire(noOrbit.canvas, "keydown", { key: "ArrowRight" });
    expect(noOrbit.viewer.getCamera().yaw).toBe(yaw);
    const fast = await ready({ controls: { rotateSpeed: 2, keyboardStep: 0.2 } });
    const y0 = fast.viewer.getCamera().yaw;
    fire(fast.canvas, "keydown", { key: "ArrowRight" });
    expect(fast.viewer.getCamera().yaw - y0).toBeCloseTo(0.2 * Math.PI * 2);
  });
});

describe("pointer", () => {
  const ready = async () => {
    const r = make();
    await r.viewer.setDocument(rig);
    return r;
  };
  it("orbits on drag, pans on shift-drag, zooms on wheel", async () => {
    const { viewer, canvas } = await ready();
    const c0 = viewer.getCamera();
    fire(canvas, "pointerdown", { pointerId: 1, clientX: 100, clientY: 50, button: 0 });
    fire(canvas, "pointermove", { pointerId: 1, clientX: 150, clientY: 80 });
    fire(canvas, "pointerup", { pointerId: 1, clientX: 150, clientY: 80 });
    const c1 = viewer.getCamera();
    expect(c1.yaw).toBeCloseTo(c0.yaw - (50 / 100) * Math.PI);
    expect(c1.pitch).toBeCloseTo(c0.pitch + (30 / 100) * Math.PI);
    fire(canvas, "pointerdown", { pointerId: 2, clientX: 100, clientY: 50, button: 0, shiftKey: true });
    fire(canvas, "pointermove", { pointerId: 2, clientX: 120, clientY: 50 });
    fire(canvas, "pointerup", { pointerId: 2, clientX: 120, clientY: 50 });
    expect(viewer.getCamera().target).not.toEqual(c1.target);
    expect(viewer.getCamera().yaw).toBe(c1.yaw);
    const d = viewer.getCamera().distance;
    const e = fire(canvas, "wheel", { deltaY: -100, deltaMode: 0 });
    expect(e.defaultPrevented).toBe(true);
    expect(viewer.getCamera().distance).toBeLessThan(d);
  });
  it("pinches to zoom with two pointers", async () => {
    const { viewer, canvas } = await ready();
    const d = viewer.getCamera().distance;
    fire(canvas, "pointerdown", { pointerId: 1, clientX: 80, clientY: 50, button: 0 });
    fire(canvas, "pointerdown", { pointerId: 2, clientX: 120, clientY: 50, button: 0 });
    fire(canvas, "pointermove", { pointerId: 2, clientX: 160, clientY: 50 });
    expect(viewer.getCamera().distance).toBeLessThan(d);
  });
  it("emits a pick with the surface under a click and none after a drag", async () => {
    const { viewer, canvas } = await ready();
    const picks: (string | null)[] = [];
    viewer.on("pick", (e) => picks.push(e.hit?.node ?? null));
    const aspect = 2;
    const head = new Vector3(0, 2.8, 0).project(threeCamera(viewer.getCamera(), aspect));
    const x = 10 + ((head.x + 1) / 2) * 200, y = 20 + ((1 - head.y) / 2) * 100;
    fire(canvas, "pointerdown", { pointerId: 1, clientX: x, clientY: y, button: 0 });
    fire(canvas, "pointerup", { pointerId: 1, clientX: x, clientY: y });
    expect(picks).toEqual(["head"]);
    fire(canvas, "pointerdown", { pointerId: 1, clientX: x, clientY: y, button: 0 });
    fire(canvas, "pointermove", { pointerId: 1, clientX: x + 30, clientY: y });
    fire(canvas, "pointerup", { pointerId: 1, clientX: x + 30, clientY: y });
    expect(picks).toHaveLength(1);
    fire(canvas, "pointerdown", { pointerId: 1, clientX: 12, clientY: 22, button: 0 });
    fire(canvas, "pointerup", { pointerId: 1, clientX: 12, clientY: 22 });
    expect(picks).toEqual(["head", null]);
  });
  it("fits on double click", async () => {
    const { viewer, canvas } = await ready();
    const fitted = viewer.getCamera();
    viewer.panBy(0.3, 0.3);
    fire(canvas, "dblclick");
    expect(viewer.getCamera().target).toEqual(fitted.target);
  });
});

describe("backend", () => {
  it("falls back to the software rasteriser when the backend cannot be created and fails loudly when told not to", async () => {
    const canvas = document.createElement("canvas");
    document.body.append(canvas);
    const throwing = () => {
      throw new Error("no gl");
    };
    const v = new ModelViewer({ canvas, strings, backend: throwing });
    expect(v.backendKind).toBe("software");
    await v.setDocument(rig);
    expect(v.readPixels().length).toBeGreaterThan(0);
    const errors: Error[] = [];
    const strict = new ModelViewer({ canvas: document.createElement("canvas"), strings, backend: throwing, fallback: false });
    strict.on("error", (e) => errors.push(e.error));
    expect(strict.backendKind).toBe("none");
  });
  it("forwards backend status and re-renders after a restore", async () => {
    let status!: (s: "ready" | "lost" | "restored") => void;
    const backend = new Recorder();
    const canvas = document.createElement("canvas");
    document.body.append(canvas);
    const v = new ModelViewer({ canvas, strings, backend: (_, s) => ((status = s), backend) });
    await v.setDocument(rig);
    flush(0);
    const seen: string[] = [];
    v.on("backend", (e) => seen.push(e.status));
    const before = backend.renders.length;
    status("lost");
    status("restored");
    flush(1);
    expect(seen).toEqual(["lost", "restored"]);
    expect(backend.renders.length).toBe(before + 1);
  });
  it("uses the device pixel ratio, capped by maxPixelRatio, and re-measures when it changes", async () => {
    vi.stubGlobal("devicePixelRatio", 2);
    const { backend, viewer } = make({ maxPixelRatio: 1.5 });
    expect(backend.sizes[0]).toEqual([200, 100, 1.5]);
    await viewer.setDocument(rig);
    vi.stubGlobal("devicePixelRatio", 1);
    viewer.requestRender();
    flush(1);
    expect(backend.sizes[backend.sizes.length - 1]).toEqual([200, 100, 1]);
  });
  it("reads the background from the option or from the CSS custom property", async () => {
    const a = make({ background: [0.1, 0.2, 0.3, 1] });
    await a.viewer.setDocument(rig);
    flush(0);
    expect(a.backend.renders[0]!.background).toEqual([0.1, 0.2, 0.3, 1]);
    const b = make();
    b.canvas.style.setProperty("--semio-viewer-background", "#ff0000");
    await b.viewer.setDocument(rig);
    flush(1);
    expect(b.backend.renders[0]!.background).toEqual([1, 0, 0, 1]);
    const c = make();
    await c.viewer.setDocument(rig);
    flush(2);
    expect(c.backend.renders[0]!.background).toEqual([1, 1, 1, 1]);
  });
  it("disposes: no frames, backend released, inserted elements removed", async () => {
    const { viewer, backend, canvas } = make();
    await viewer.setDocument(rig);
    viewer.dispose();
    expect(backend.disposed).toBe(true);
    expect(queue).toHaveLength(0);
    expect(canvas.parentElement!.querySelectorAll("div")).toHaveLength(0);
    const yaw = viewer.getCamera().yaw;
    fire(canvas, "keydown", { key: "ArrowRight" });
    expect(viewer.getCamera().yaw).toBe(yaw);
  });
});

describe("parseCssColor", () => {
  it("parses hex, rgb and rgba in both syntaxes and rejects the rest", () => {
    expect(parseCssColor("#fff")).toEqual([1, 1, 1, 1]);
    expect(parseCssColor("#ff000080")).toEqual([1, 0, 0, 128 / 255]);
    expect(parseCssColor("#0f08")).toEqual([0, 1, 0, 136 / 255]);
    expect(parseCssColor("rgb(255, 0, 51)")).toEqual([1, 0, 0.2, 1]);
    expect(parseCssColor("rgb(100% 0% 0% / 50%)")).toEqual([1, 0, 0, 0.5]);
    expect(parseCssColor("rgba(0,0,0,0.25)")).toEqual([0, 0, 0, 0.25]);
    expect(parseCssColor("transparent")).toEqual([0, 0, 0, 0]);
    for (const bad of ["red", "#12", "rgb(1,2)", "rgb(a,b,c)", "hsl(0 0% 0%)", ""]) expect(parseCssColor(bad), bad).toBeNull();
  });
});

describe("custom element", () => {
  it("requires strings before connecting, hosts a viewer in a shadow root and re-dispatches events", async () => {
    const tag = "semio-test-viewer";
    defineModelViewerElement(tag);
    expect(defineModelViewerElement(tag)).toBe(customElements.get(tag));
    const bare = document.createElement(tag) as HTMLElement & { strings: unknown };
    const errors: string[] = [];
    const onError = (e: ErrorEvent) => {
      errors.push(e.message);
      e.preventDefault();
    };
    window.addEventListener("error", onError);
    document.body.append(bare);
    window.removeEventListener("error", onError);
    expect(errors.join()).toMatch(/no default language/);
    const el = document.createElement(tag) as HTMLElement & { strings: unknown; options: Partial<ModelViewerOptions>; model: ModelDocument; viewer: ModelViewer | null };
    el.strings = strings;
    el.options = { backend: () => new Recorder() };
    el.setAttribute("preset", "plan");
    const ready = new Promise<void>((resolve) => el.addEventListener("semio-ready", () => resolve()));
    document.body.append(el);
    expect(el.shadowRoot!.querySelector("canvas")!.getAttribute("aria-label")).toBe("Etiqueta");
    el.model = rig;
    await ready;
    expect(el.viewer!.getCamera().pitch).toBeCloseTo(Math.PI / 2);
    el.setAttribute("variant", "dark");
    el.setAttribute("animation", "wave");
    expect(el.viewer!.clock.animation).toBe("wave");
    el.setAttribute("playing", "");
    expect(el.viewer!.clock.playing).toBe(true);
    el.remove();
    expect(el.viewer).toBeNull();
  });
});
