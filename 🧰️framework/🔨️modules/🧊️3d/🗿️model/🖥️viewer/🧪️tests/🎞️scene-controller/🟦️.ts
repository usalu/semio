// @vitest-environment jsdom
/** 🎞️ Controller and custom element: `scenes`, `activeScene`, `setScene` (progress-free, no buffer upload: the backend loads once), the `scene` event and status, the `scene` attribute, an unknown scene and refitting the camera to the scene. */
import { beforeEach, afterEach, describe, expect, it, vi } from "vitest";
import type { ModelDocument } from "../../../🧬️schema/🟦️.ts";
import { tessellate } from "../../../🔺️tessellation/🟦️.ts";
import { directoryNear, readFixture } from "../../../🧪️tests/🧰️support/🧫️reader/🟦️.ts";
import type { Frame, RenderBackend, Scene } from "../../🧬️contract/🟦️.ts";
import { ModelViewer, type ViewerStatus } from "../../🕹️controller/🟦️.ts";

const data = readFixture(directoryNear(import.meta.url, "../../../🧫️fixtures"), "🎞️scenes");
const resolved = (await tessellate(data.document, data.quality, { report: () => undefined, cancelled: () => false })) as ModelDocument;

class Recorder implements RenderBackend {
  readonly kind = "recorder";
  loads = 0;
  draws: number[] = [];
  load(_scene: Scene) { this.loads++; }
  resize() {}
  render(frame: Frame) { this.draws.push(frame.drawCount); }
  readPixels() { return new Uint8ClampedArray(4); }
  dispose() {}
}

let queue: FrameRequestCallback[] = [];
beforeEach(() => {
  queue = [];
  vi.stubGlobal("requestAnimationFrame", (cb: FrameRequestCallback) => (queue.push(cb), queue.length));
  vi.stubGlobal("cancelAnimationFrame", () => {});
});
afterEach(() => { vi.unstubAllGlobals(); document.body.innerHTML = ""; });
const flush = () => { const run = queue; queue = []; run.forEach((cb) => cb(0)); };

const make = (scene?: string) => {
  const canvas = document.createElement("canvas");
  canvas.getBoundingClientRect = () => ({ left: 0, top: 0, width: 200, height: 100, right: 200, bottom: 100, x: 0, y: 0, toJSON: () => ({}) });
  document.body.append(canvas);
  const backend = new Recorder();
  const statuses: string[] = [];
  const viewer = new ModelViewer({ canvas, backend: () => backend, scene, strings: { label: "l", description: "d", status: (s: ViewerStatus) => (statuses.push(s.kind === "scene" ? `scene:${s.scene}` : s.kind), "") } });
  return { viewer, backend, statuses };
};

describe("🎞️ viewer controller scenes", () => {
  it("lists the scenes, starts on the first or the option's scene and switches without re-uploading", async () => {
    const { viewer, backend, statuses } = make();
    expect(viewer.scenes).toEqual([]);
    await viewer.setDocument(resolved);
    expect(viewer.scenes.map((s) => s.id)).toEqual(["body", "structure", "thermal"]);
    expect(viewer.scenes[0]!.title).toEqual({ en: "Body", de: "Körper" });
    expect(viewer.activeScene).toBe("body");
    flush();
    const events: string[] = [];
    viewer.on("scene", (e) => events.push(e.scene));
    viewer.setScene("thermal");
    flush();
    viewer.setScene("structure");
    flush();
    viewer.setScene(null);
    flush();
    expect(events).toEqual(["thermal", "structure", "body"]);
    expect(backend.loads).toBe(1);
    expect(backend.draws.length).toBeGreaterThanOrEqual(4);
    expect(statuses).toContain("scene:thermal");
    expect(viewer.activeScene).toBe("body");
    expect(() => viewer.setScene("nope")).toThrow(/unknown scene/u);
    expect(make("thermal").viewer.scenes).toEqual([]);
    const other = make("thermal");
    await other.viewer.setDocument(resolved);
    expect(other.viewer.activeScene).toBe("thermal");
  });

  it("draws only the primitives of the active scene and refits the camera to it", async () => {
    const { viewer, backend } = make();
    await viewer.setDocument(resolved);
    flush();
    const body = backend.draws.at(-1)!;
    const camera = viewer.getCamera().extent;
    viewer.setScene("thermal");
    flush();
    const thermal = backend.draws.at(-1)!;
    expect(thermal).not.toBe(body);
    expect(thermal).toBe(2);
    expect(viewer.getCamera().extent).not.toBe(camera);
  });
});
