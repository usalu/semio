// @vitest-environment jsdom
/** 🎥️ Follow camera and root motion: frames of a walker with a translating root against three's keyframe interpolant, `Matrix4` and `Box3` (travel and in-place root translation, node and bounds follow, the exponential smoothing step, snapping), plus the controller (options, setters, `follow`, `rootMotion`, `preset` and `camera` events) and the custom element attributes. */
import { Box3, Matrix4, Vector3, VectorKeyframeTrack } from "three";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import type { ModelDocument } from "../../../🧬️schema/🟦️.ts";
import type { Frame, RenderBackend, Scene } from "../../🧬️contract/🟦️.ts";
import { cameraOf } from "../../📷️camera/🟦️.ts";
import { ModelViewer } from "../../🕹️controller/🟦️.ts";
import { createFollowState, smoothFollow } from "../../🎥️follow/🟦️.ts";
import { boundsOf, createFrame, frameOf } from "../../🎞️frame/🟦️.ts";
import { sceneOf } from "../../🧱️scene/🟦️.ts";
import { defineModelViewerElement } from "../../🧩️element/🟦️.ts";
import { fixture } from "../🧰️support/🟦️.ts";

type Vector = { document: ModelDocument; times: number[]; smoothing: { smoothingS: number; steps: { timeS: number; dtS: number }[] }; expected: { travelRoot: number[][]; inPlaceRoot: number[][]; bodyOffset: number[]; boundsCentreOffset: number[] } };
const data = fixture<Vector>("🎥️follow");
const scene = sceneOf(data.document);
const track = (new VectorKeyframeTrack("root.position", [0, 2], [1, 0, 1, 5, 0.5, 3]) as unknown as { createInterpolant(): { evaluate(time: number): ArrayLike<number> } }).createInterpolant();
const rootAt = (t: number): Vector3 => new Vector3(...Array.from(track.evaluate(t)));
const worldOf = (frame: Frame, id: string): Vector3 => new Vector3().setFromMatrixPosition(new Matrix4().fromArray(Array.from(frame.matrices.subarray(scene.nodeIndex.get(id)! * 16, scene.nodeIndex.get(id)! * 16 + 16))));
const frame = (timeS: number, extra: Record<string, unknown> = {}): Frame => frameOf(scene, { pose: { animation: "drive", timeS }, camera: cameraOf("perspective"), aspect: 1, ...extra }, createFrame(scene));
const near = (a: Vector3, b: ArrayLike<number>) => expect(a.distanceTo(new Vector3(b[0], b[1], b[2]))).toBeLessThan(1e-5);

describe("root motion", () => {
  it.each(data.times.map((t, i) => [t, i] as const))("travel keeps the sampled root translation at t=%s", (t, i) => {
    near(worldOf(frame(t, { rootMotion: "travel" }), "root"), rootAt(t).toArray());
    near(worldOf(frame(t), "root"), data.expected.travelRoot[i]!);
  });
  it.each(data.times.map((t, i) => [t, i] as const))("in-place removes the horizontal change against the rest translation and keeps y at t=%s", (t, i) => {
    const f = frame(t, { rootMotion: "in-place" });
    const want = rootAt(t);
    want.x = 1;
    want.z = 1;
    near(worldOf(f, "root"), want.toArray());
    near(worldOf(f, "root"), data.expected.inPlaceRoot[i]!);
    near(worldOf(f, "body"), want.clone().add(new Vector3(...data.expected.bodyOffset)).toArray());
  });
  it("moves every node below the root and the geometry with it", () => {
    const box = (f: Frame) => new Box3().set(new Vector3(...boundsOf(scene, f).min), new Vector3(...boundsOf(scene, f).max));
    const travel = box(frame(1, { rootMotion: "travel" })), place = box(frame(1, { rootMotion: "in-place" }));
    near(travel.min.clone().sub(place.min), [2, 0, 1]);
    near(travel.max.clone().sub(place.max), [2, 0, 1]);
  });
  it("ignores a named node that has a parent and falls back to the first root", () => {
    near(worldOf(frame(1, { rootMotion: "in-place", rootNode: "body" }), "root"), data.expected.inPlaceRoot[2]!);
  });
});

describe("follow", () => {
  it("tracks the world translation of a node (snapping on the first frame)", () => {
    const state = createFollowState();
    frame(1, { follow: { mode: { node: "body" }, state, dtS: 0, smoothingS: 0.25 } });
    near(new Vector3(...state.target), rootAt(1).add(new Vector3(...data.expected.bodyOffset)).toArray());
  });
  it("tracks the centre of the posed bounds", () => {
    const state = createFollowState();
    const f = frame(1, { follow: { mode: "bounds", state, dtS: 0, smoothingS: 0.25 } });
    const box = new Box3().set(new Vector3(...boundsOf(scene, f).min), new Vector3(...boundsOf(scene, f).max));
    near(box.getCenter(new Vector3()), state.target);
    near(new Vector3(...state.target), rootAt(1).add(new Vector3(...data.expected.boundsCentreOffset)).toArray());
  });
  it("looks through the followed target, not the request's target", () => {
    const state = createFollowState();
    const camera = { ...cameraOf("perspective"), target: [50, 50, 50] as [number, number, number] };
    const f = frame(1, { camera, follow: { mode: { node: "root" }, state, dtS: 0, smoothingS: 0 } });
    const plain = frame(1, { camera: { ...camera, target: state.target } });
    expect(Array.from(f.view)).toEqual(Array.from(plain.view));
  });
  it("smooths with target += (point - target) * (1 - exp(-dt / tau)) and is deterministic", () => {
    const run = () => {
      const state = createFollowState();
      const seen: number[][] = [];
      let expected: Vector3 | null = null;
      for (const step of data.smoothing.steps) {
        const point = rootAt(step.timeS).add(new Vector3(0, 1, 0));
        frame(step.timeS, { follow: { mode: { node: "body" }, state, dtS: step.dtS, smoothingS: data.smoothing.smoothingS } });
        expected = expected ? expected.lerp(point, 1 - Math.exp(-step.dtS / data.smoothing.smoothingS)) : point;
        near(expected, state.target);
        seen.push([...state.target]);
      }
      return seen;
    };
    expect(run()).toEqual(run());
  });
  it("snaps with a zero time constant and does not move at zero elapsed time once ready", () => {
    const state = createFollowState();
    smoothFollow(state, [1, 2, 3], 0, 0.25);
    expect([...state.target]).toEqual([1, 2, 3]);
    smoothFollow(state, [9, 9, 9], 0, 0.25);
    expect([...state.target]).toEqual([1, 2, 3]);
    smoothFollow(state, [9, 9, 9], 0.1, 0);
    expect([...state.target]).toEqual([9, 9, 9]);
  });
  it("allocates no new arrays for the target across frames", () => {
    const state = createFollowState();
    const target = state.target;
    for (const t of data.times) frame(t, { follow: { mode: "bounds", state, dtS: 0.1, smoothingS: 0.25 } });
    expect(state.target).toBe(target);
  });
});

class Recorder implements RenderBackend {
  readonly kind = "recorder";
  frames: { root: Vector3; view: number[] }[] = [];
  scene: Scene | null = null;
  load(s: Scene) { this.scene = s; }
  resize() {}
  render(f: Frame) { this.frames.push({ root: worldOf(f, "root"), view: Array.from(f.view) }); }
  readPixels() { return new Uint8ClampedArray(4); }
  dispose() {}
}

let queue: FrameRequestCallback[] = [];
const flush = (now: number) => { const run = queue; queue = []; for (const cb of run) cb(now); };
const strings = { label: "l", description: "d" };
const make = (extra: Record<string, unknown> = {}) => {
  const canvas = document.createElement("canvas");
  document.body.append(canvas);
  canvas.getBoundingClientRect = () => ({ left: 0, top: 0, width: 100, height: 100, right: 100, bottom: 100, x: 0, y: 0, toJSON: () => ({}) });
  const backend = new Recorder();
  return { backend, canvas, viewer: new ModelViewer({ canvas, strings, backend: () => backend, autoplay: false, reducedMotion: false, ...extra }) };
};

beforeEach(() => {
  queue = [];
  vi.stubGlobal("requestAnimationFrame", (cb: FrameRequestCallback) => (queue.push(cb), queue.length));
  vi.stubGlobal("cancelAnimationFrame", () => {});
});
afterEach(() => { vi.unstubAllGlobals(); document.body.innerHTML = ""; });

describe("controller", () => {
  it("keeps travel and no follow by default and plays the translating root through", async () => {
    const { viewer, backend } = make();
    await viewer.setDocument(data.document);
    expect([viewer.follow, viewer.rootMotion]).toEqual(["off", "travel"]);
    viewer.setAnimation("drive");
    viewer.seek(1);
    viewer.render();
    near(backend.frames.at(-1)!.root, data.expected.travelRoot[2]!);
  });
  it("plays in place when asked by option or setter and emits rootMotion", async () => {
    const seen: string[] = [];
    const { viewer, backend } = make({ rootMotion: "in-place" });
    viewer.on("rootMotion", (e) => seen.push(e.rootMotion));
    await viewer.setDocument(data.document);
    viewer.setAnimation("drive");
    viewer.seek(2);
    viewer.render();
    near(backend.frames.at(-1)!.root, data.expected.inPlaceRoot[3]!);
    viewer.setRootMotion("travel");
    viewer.render();
    near(backend.frames.at(-1)!.root, data.expected.travelRoot[3]!);
    expect(seen).toEqual(["travel"]);
  });
  it("follows a node while playing, smoothing per frame, and emits follow and camera", async () => {
    const events: string[] = [];
    const { viewer } = make({ follow: { node: "body" }, followSmoothingS: 0.25 });
    viewer.on("camera", () => events.push("camera"));
    viewer.on("follow", (e) => events.push(`follow:${JSON.stringify(e.follow)}`));
    await viewer.setDocument(data.document);
    viewer.setAnimation("drive");
    viewer.play();
    flush(0);
    flush(500);
    flush(1000);
    const t = viewer.getCamera().target;
    const point = rootAt(viewer.clock.timeS).add(new Vector3(0, 1, 0));
    expect(new Vector3(...t).distanceTo(point)).toBeGreaterThan(0);
    expect(new Vector3(...t).x).toBeGreaterThan(2);
    expect(new Vector3(...t).x).toBeLessThan(point.x);
    viewer.setFollow("off");
    const frozen = [...viewer.getCamera().target];
    viewer.seek(0);
    viewer.render();
    expect([...viewer.getCamera().target]).toEqual(frozen);
    expect(events).toContain("camera");
    expect(events).toContain("follow:\"off\"");
  });
  it("emits preset when a preset is chosen and null when the camera is set freely", async () => {
    const { viewer } = make();
    const seen: unknown[] = [];
    viewer.on("preset", (e) => seen.push(e.preset));
    await viewer.setDocument(data.document);
    viewer.setPreset("front");
    viewer.orbitBy(0.1, 0);
    expect(seen).toEqual(["front", null]);
  });
});

describe("custom element", () => {
  it("mirrors the follow and root-motion attributes, adopts a constructable style sheet and re-dispatches the new events", async () => {
    const tag = defineModelViewerElement("semio-follow-test");
    const el = document.createElement(tag.name ? "semio-follow-test" : "semio-follow-test") as HTMLElement & { strings: unknown; options: unknown; viewer: ModelViewer | null; model: ModelDocument };
    el.strings = strings;
    el.options = { backend: () => new Recorder(), reducedMotion: false };
    el.setAttribute("follow", "node:body");
    el.setAttribute("root-motion", "in-place");
    document.body.append(el);
    const names: string[] = [];
    el.addEventListener("semio-rootMotion", () => names.push("rootMotion"));
    el.model = data.document;
    await new Promise((resolve) => setTimeout(resolve, 0));
    expect(el.viewer!.follow).toEqual({ node: "body" });
    expect(el.viewer!.rootMotion).toBe("in-place");
    el.setAttribute("root-motion", "travel");
    expect(el.viewer!.rootMotion).toBe("travel");
    expect(names.length).toBeGreaterThanOrEqual(1);
    const adopted = ((el.shadowRoot as unknown as { adoptedStyleSheets?: unknown[] }).adoptedStyleSheets?.length ?? 0) > 0;
    expect(adopted || el.shadowRoot!.querySelector("style") !== null).toBe(true);
  });
});
