// @vitest-environment jsdom

import Ajv2020 from "ajv/dist/2020.js";
import { act, cleanup, render } from "@semio-tech/ui-react/test";
import { uiI18n } from "@semio-tech/ui-react";
import { Matrix3, Vector3 } from "three";
import { afterEach, expect, it, vi } from "vitest";
import { Paint2dHost } from "../../🟦️.tsx";
import type { RasterWasmSession } from "../../../🪪️WasmSessionLoader/🟦️.tsx";
import rasterBackingStore from "../🧫️fixtures/raster-backing-store.json";
import fixture from "../🧫️fixtures/🧭️navigator-camera/🔣️.json";

const createRasterSession = vi.hoisted(() => vi.fn(() => new Promise<RasterWasmSession>(() => {})));
vi.mock("../../../🪪️WasmSessionLoader/🟦️.tsx", async (load) => ({ ...await load<Record<string, unknown>>(), createRasterSession }));

afterEach(() => { cleanup(); vi.restoreAllMocks(); vi.unstubAllGlobals(); createRasterSession.mockReset(); });

it("the mounted React Navigator publishes the authored content camera on wheel", async () => {
  await uiI18n.changeLanguage("en");
  for (const sample of fixture.cases) {
    const actions: any[] = [];
    const scene = { documentSyncJson: '{"schema":"raster.document","layers":[]}', assetsJson: "{}", cameraJson: JSON.stringify(sample.camera), selectionJson: "[]", activeUtility: "brush", brushSize: 12, brushOpacity: 1, brushColor: "#000000", brushHardness: 1,paintTarget:"pixels" as const,maskValue:255,fillTolerance:24,editingOptions:{filterKind:"brightness" as const,filterAmount:0,imageGrid:{x:0,y:0,width:512,height:512,bilinear:true}}, viewMode: "navigator" as const, ...(sample.viewport ? { compositeViewportJson: JSON.stringify(sample.viewport) } : {}) };
    const view = render(<Paint2dHost node={{ type: "componentScene", surfaceId: "navigator-camera", controllerId: "raster", componentKind: "paint2d", paint2d: scene }} onAction={action => { actions.push(action); }} />);
    const host = view.container.querySelector<HTMLElement>('[data-surface-id="navigator-camera"]')!;
    const [x, y, width, height] = sample.surface;
    vi.spyOn(host, "getBoundingClientRect").mockReturnValue({ x, y, width, height, left: x, top: y, right: x + width, bottom: y + height, toJSON: () => ({}) });
    await act(async () => { host.querySelector(".z-30")!.dispatchEvent(new WheelEvent("wheel", { bubbles: true, cancelable: true, clientX: sample.point[0], clientY: sample.point[1], deltaY: sample.deltaY })); });
    expect(actions, sample.id).toHaveLength(1);
    expect(actions[0].action).toBe("setCamera");
    expect(actions[0].controllerId).toBe("raster");
    expect(actions[0].args.surfaceId).toBe("navigator-camera");
    for (const key of ["x", "y", "zoom"] as const) expect(actions[0].args.camera[key], sample.id).toBeCloseTo(sample.expected[key], 10);
    cleanup();
  }
});

it("the resulting camera preserves the wheel anchor through Three matrix inversion", () => {
  for (const sample of fixture.cases) {
    const viewport = sample.viewport ?? { width: 800, height: 600 };
    const screen = new Vector3(sample.point[0] - sample.surface[0], sample.point[1] - sample.surface[1], 1);
    const unproject = (camera: typeof sample.camera) => screen.clone().applyMatrix3(new Matrix3().set(camera.zoom, 0, viewport.width / 2 - camera.x * camera.zoom, 0, camera.zoom, viewport.height / 2 - camera.y * camera.zoom, 0, 0, 1).invert());
    expect(unproject(sample.camera).distanceTo(unproject(sample.expected)), sample.id).toBeLessThan(1e-9);
  }
});

it("keeps the raster backing store intact on identical observations and retired callbacks", async () => {
  await uiI18n.changeLanguage("en");
  let size = rasterBackingStore.initial;
  vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockImplementation(() => new DOMRect(0, 0, size.width, size.height));
  vi.stubGlobal("devicePixelRatio", size.dpr);
  const observers: { notify: () => void; active: boolean }[] = [];
  vi.stubGlobal("ResizeObserver", class {
    private readonly owner: typeof observers[number];

    constructor(notify: () => void) { this.owner = { notify, active: false }; }
    observe() { this.owner.active = true; observers.push(this.owner); }
    disconnect() { this.owner.active = false; }
  });
  const width = vi.spyOn(HTMLCanvasElement.prototype, "width", "set");
  const height = vi.spyOn(HTMLCanvasElement.prototype, "height", "set");
  const cameraJson = '{"x":0,"y":0,"zoom":1}';
  const attached = Promise.withResolvers<void>();
  const noop = () => {};
  const session: RasterWasmSession = {
    gpuReady: () => true, attachCanvas: vi.fn(() => attached.promise), setSize: vi.fn(),
    renderFrame: () => false, renderProgressJson: () => '{"pending":false,"completed":0,"total":0,"error":null}',
    cancelRender: noop, setCamera: noop, wheelScreen: noop, pointerDownScreen: noop, pointerMoveScreen: noop,
    pointerUpScreen: noop, pointerCancelScreen: noop, syncDocumentJson: noop, uploadLayerImage: noop,
    uploadRasterImageKey: noop, setActiveUtility: noop, setBrushSize: noop, setBrushOpacity: noop,
    syncInteraction: noop, setCanvasThemeJson: noop, cameraJson: () => cameraJson, setViewMode: noop,
    pickTargetsAtScreenJson: () => "[]", marqueeHitsJson: () => "[]", navigatorFitCameraJson: () => cameraJson,
    navigatorViewportOverlayJson: () => "{}", free: noop,
  };
  createRasterSession.mockResolvedValueOnce(session);
  const scene = {
    documentSyncJson: '{"schema":"raster.document","layers":[]}', assetsJson: "{}", cameraJson, selectionJson: "[]",
    activeUtility: "brush", brushSize: 12, brushOpacity: 1, brushColor: "#000000", brushHardness: 1,
    paintTarget: "pixels" as const, maskValue: 255, fillTolerance: 24, viewMode: "navigator" as const,
    editingOptions: { filterKind: "brightness" as const, filterAmount: 0, imageGrid: { x: 0, y: 0, width: 512, height: 512, bilinear: true } },
  };
  const view = render(<Paint2dHost node={{ type: "componentScene", surfaceId: "raster-size", controllerId: "raster", componentKind: "paint2d", paint2d: scene }} onAction={noop} />);
  await act(async () => {});
  const canvas = view.container.querySelector("canvas")!;
  expect([width.mock.calls.length, height.mock.calls.length]).toEqual([0, 0]);
  expect(session.attachCanvas).toHaveBeenCalledWith(canvas, size.width, size.height, 1);
  await act(async () => attached.resolve());
  const owner = observers.find(observer => observer.active)!;
  expect(owner).toBeDefined();
  expect(session.setSize).not.toHaveBeenCalled();
  for (const sample of rasterBackingStore.updates) {
    size = sample;
    vi.stubGlobal("devicePixelRatio", size.dpr);
    width.mockClear();
    height.mockClear();
    vi.mocked(session.setSize).mockClear();
    act(() => owner.notify());
    expect([width.mock.calls.length, height.mock.calls.length]).toEqual(sample.writes);
    const independent = new Vector3(size.width, size.height, 0).round().multiplyScalar(size.dpr).round();
    expect([independent.x, independent.y]).toEqual(sample.backing);
    expect([canvas.width, canvas.height]).toEqual(sample.backing);
    expect(vi.mocked(session.setSize).mock.calls).toEqual(sample.logicalResize ? [[Math.round(size.width), Math.round(size.height), size.dpr]] : []);
  }
  view.unmount();
  expect(owner.active).toBe(false);
  size = rasterBackingStore.retired;
  width.mockClear();
  height.mockClear();
  vi.mocked(session.setSize).mockClear();
  act(() => owner.notify());
  expect([width.mock.calls.length, height.mock.calls.length]).toEqual([0, 0]);
  expect(session.setSize).not.toHaveBeenCalled();
});

it("publishes viewport dimensions once per size and recipient", async () => {
  await uiI18n.changeLanguage("en");
  vi.spyOn(HTMLCanvasElement.prototype, "getContext").mockReturnValue(null);
  let size = rasterBackingStore.initial;
  vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockImplementation(() => new DOMRect(0, 0, size.width, size.height));
  const observers: (() => void)[] = [];
  vi.stubGlobal("ResizeObserver", class implements ResizeObserver {
    constructor(readonly notify: ResizeObserverCallback) {}
    observe(target: Element) {
      if (target.hasAttribute("data-surface-id")) observers.push(() => this.notify([], this));
    }
    unobserve() {}
    disconnect() {}
  });
  const scene = {
    documentSyncJson: '{"schema":"raster.document","layers":[]}', assetsJson: "{}", cameraJson: '{"x":0,"y":0,"zoom":1}', selectionJson: "[]",
    activeUtility: "brush", brushSize: 12, brushOpacity: 1, brushColor: "#000000", brushHardness: 1,
    paintTarget: "pixels" as const, maskValue: 255, fillTolerance: 24, viewMode: "composite" as const,
    editingOptions: { filterKind: "brightness" as const, filterAmount: 0, imageGrid: { x: 0, y: 0, width: 512, height: 512, bilinear: true } },
  };
  const node = { type: "componentScene" as const, surfaceId: "raster-composite", controllerId: "raster", componentKind: "paint2d" as const, paint2d: scene };
  const argsFor = (surfaceId: string) => {
    const { x: width, y: height } = new Vector3(size.width, size.height, 0).round();
    return { surfaceId, width, height };
  };
  const initial = vi.fn();
  const current = vi.fn();
  const view = render(<Paint2dHost node={node} onAction={initial} />);
  expect(initial.mock.calls).toEqual([[{ controllerId: node.controllerId, action: "setCompositeViewport", args: argsFor(node.surfaceId) }]]);
  view.rerender(<Paint2dHost node={node} onAction={current} />);
  expect(current).not.toHaveBeenCalled();
  size = rasterBackingStore.updates[1]!;
  act(() => observers.at(-1)!());
  expect(current.mock.calls).toEqual([[{ controllerId: node.controllerId, action: "setCompositeViewport", args: argsFor(node.surfaceId) }]]);
  const replacement = { ...node, controllerId: "another-raster", surfaceId: "another-composite" };
  const retired = observers.at(-1)!;
  current.mockClear();
  view.rerender(<Paint2dHost node={replacement} onAction={current} />);
  expect(current.mock.calls).toEqual([[{ controllerId: replacement.controllerId, action: "setCompositeViewport", args: argsFor(replacement.surfaceId) }]]);
  current.mockClear();
  size = rasterBackingStore.retired;
  act(() => retired());
  expect(current).not.toHaveBeenCalled();
});
