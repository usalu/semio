import { act, cleanup, fireEvent, render } from "@semio-tech/ui-react/test";
import { createShellI18nInstance, disposeShellI18nInstance, UiI18nProvider } from "@semio-tech/ui-react";
import type { UiComponentSceneNode } from "@semio-tech/framework";
import { afterEach, expect, it, vi } from "vitest";
import { TiledMapHost } from "../../🟦️.tsx";
import fixture from "../🧫️fixtures/🩹️gpu-recovery/🔣️.json";

const { createMapSession } = vi.hoisted(() => ({ createMapSession: vi.fn() }));
vi.mock("../../../🪪️WasmSessionLoader/🟦️.tsx", async (original) => ({
  ...(await original<typeof import("../../../🪪️WasmSessionLoader/🟦️.tsx")>()),
  createMapSession,
}));

const node: UiComponentSceneNode = {
  type: "componentScene",
  controllerId: "controller",
  surfaceId: "surface",
  componentKind: "tiled-map",
  tiledMap: {
    mapDescriptorJson: "{}",
    cameraJson: JSON.stringify(fixture.initialCamera),
    renderMode: "vector",
    vectorStyle: "default",
    lodMode: "automatic",
    tileUrlTemplate: "/tiles/{z}/{x}/{y}.png",
    vectorTileUrlTemplate: "/tiles/{z}/{x}/{y}.pbf",
    layerVisibilityJson: "{}",
    layerStrokeScaleJson: "{}",
    selectionJson: "[]",
    hoverJson: "{}",
    selectionMethod: "rectangle",
    selectionMode: "feature",
  },
};

function stubSession() {
  const camera = { ...fixture.initialCamera };
  return {
    camera,
    attachCanvas: vi.fn(async () => {}),
    attachCanvas2d: vi.fn((_canvas: HTMLCanvasElement, _width: number, _height: number, _dpr: number): void => { throw new Error("Canvas2D unavailable"); }),
    setSize: vi.fn(),
    renderFrame: vi.fn(),
    setCamera: vi.fn((x: number, y: number, zoom: number) => Object.assign(camera, { x, y, zoom })),
    cameraJson: () => JSON.stringify(camera),
    cameraLimitsJson: () => '{"min":0.05,"max":64}',
    fitWorldCamera: vi.fn(),
    pointerDownScreen: vi.fn(),
    syncMapJson: vi.fn(),
    visibleVectorTilesJson: () => "[]",
    visibleVectorTilesRevision: () => 1,
    prefetchVectorTilesJson: () => "[]",
    setRenderMode: vi.fn(),
    setVectorStyle: vi.fn(),
    setLodMode: vi.fn(),
    setLayerVisibilityJson: vi.fn(),
    setLayerStrokeScaleJson: vi.fn(),
    syncInteraction: vi.fn(),
    setMapThemeJson: vi.fn(),
    rendererReady: () => true,
    free: vi.fn(),
  };
}

afterEach(() => {
  cleanup();
  vi.restoreAllMocks();
  vi.useRealTimers();
  createMapSession.mockReset();
});

it.each(fixture.fallbackCases)("uses Canvas2D after a $stage failure", async ({ stage, renderer, camera }) => {
  vi.useFakeTimers();
  vi.spyOn(HTMLElement.prototype, "clientWidth", "get").mockReturnValue(fixture.viewport.width);
  vi.spyOn(HTMLElement.prototype, "clientHeight", "get").mockReturnValue(fixture.viewport.height);
  const session = stubSession();
  session.attachCanvas2d.mockImplementation((canvas) => { canvas.dataset.mapRenderer = renderer; });
  const error = new Error("GPU unavailable");
  const pendingAttach = Promise.withResolvers<void>();
  if (stage === "attach") session.attachCanvas.mockRejectedValueOnce(error);
  else if (stage === "timeout") session.attachCanvas.mockReturnValueOnce(pendingAttach.promise);
  else session.renderFrame.mockImplementationOnce(() => {
    Object.assign(session.camera, fixture.liveCamera);
    throw error;
  });
  createMapSession.mockResolvedValueOnce(session);
  const language = createShellI18nInstance("en");
  try {
    const view = render(<UiI18nProvider instance={language}><TiledMapHost node={node} onAction={vi.fn()} /></UiI18nProvider>);
    const oldCanvas = view.container.querySelector("canvas")!;
    oldCanvas.focus();
    if (stage === "timeout") {
      await act(async () => { await vi.advanceTimersByTimeAsync(fixture.gpuAttachLimitMs); });
    }
    for (let attempt = 0; attempt < 3; attempt++) {
      await act(async () => { await vi.advanceTimersByTimeAsync(100); });
    }
    const canvas = view.container.querySelector("canvas")!;
    expect(canvas).not.toBe(oldCanvas);
    expect(canvas.dataset.mapRenderer).toBe(renderer);
    expect(document.activeElement).toBe(canvas);
    expect(session.attachCanvas2d).toHaveBeenCalledExactlyOnceWith(canvas, fixture.viewport.width, fixture.viewport.height, 1);
    expect(createMapSession).toHaveBeenCalledOnce();
    expect(session.free).not.toHaveBeenCalled();
    expect(session.camera).toEqual(camera);
    expect(view.container.querySelector('[role="alert"]')).toBeNull();
    expect(session.renderFrame).toHaveBeenCalled();
    pendingAttach.resolve();
    fireEvent.pointerDown(canvas, { pointerId: 1, button: 1, clientX: fixture.pointer.x, clientY: fixture.pointer.y });
    expect(session.pointerDownScreen).toHaveBeenCalledExactlyOnceWith(fixture.pointer.x, fixture.pointer.y, 1);
    view.unmount();
    expect(session.free).toHaveBeenCalledOnce();
  } finally {
    disposeShellI18nInstance(language);
  }
});

it.each(fixture.cases)("retries the map after a $stage GPU failure", async ({ stage, locale, unavailable, retry }) => {
  vi.useFakeTimers();
  vi.spyOn(HTMLElement.prototype, "clientWidth", "get").mockReturnValue(fixture.viewport.width);
  vi.spyOn(HTMLElement.prototype, "clientHeight", "get").mockReturnValue(fixture.viewport.height);
  const first = stubSession();
  const recovered = stubSession();
  const pendingAttach = Promise.withResolvers<void>();
  const error = new Error("GPU unavailable");
  if (stage === "loader") createMapSession.mockRejectedValueOnce(error);
  else createMapSession.mockResolvedValueOnce(first);
  if (stage === "attach") first.attachCanvas.mockRejectedValueOnce(error);
  if (stage === "timeout") first.attachCanvas.mockReturnValueOnce(pendingAttach.promise);
  if (stage === "frame") first.renderFrame.mockImplementationOnce(() => {
    Object.assign(first.camera, fixture.liveCamera);
    throw error;
  });
  createMapSession.mockResolvedValueOnce(recovered);
  const language = createShellI18nInstance(locale === "de" ? "de" : "en");
  try {
    const view = render(<UiI18nProvider instance={language}><TiledMapHost node={node} onAction={vi.fn()} /></UiI18nProvider>);
    const oldCanvas = view.container.querySelector("canvas")!;
    if (stage === "timeout") {
      await act(async () => { await vi.advanceTimersByTimeAsync(fixture.gpuAttachLimitMs - 1); });
      expect(view.container.querySelector('[role="alert"]')).toBeNull();
    }
    await act(async () => { await vi.advanceTimersByTimeAsync(stage === "timeout" ? 1 : 100); });
    await act(async () => { await vi.advanceTimersByTimeAsync(100); });
    expect(view.getByRole("alert").textContent).toBe(unavailable);
    expect(first.free).toHaveBeenCalledTimes(stage === "loader" ? 0 : 1);
    const frames = first.renderFrame.mock.calls.length;
    await act(async () => { await vi.advanceTimersByTimeAsync(1000); });
    expect(first.renderFrame).toHaveBeenCalledTimes(frames);

    fireEvent.click(view.getByRole("button", { name: retry }));
    await act(async () => { await vi.advanceTimersByTimeAsync(100); });
    const newCanvas = view.container.querySelector("canvas")!;
    expect(newCanvas).not.toBe(oldCanvas);
    expect(recovered.attachCanvas).toHaveBeenCalledExactlyOnceWith(newCanvas, fixture.viewport.width, fixture.viewport.height, 1);
    expect(view.container.querySelector('[role="alert"]')).toBeNull();
    expect(recovered.camera).toEqual(stage === "frame" ? fixture.liveCamera : fixture.initialCamera);
    expect(recovered.renderFrame).toHaveBeenCalled();

    pendingAttach.resolve();
    await act(async () => { await vi.advanceTimersByTimeAsync(100); });
    expect(first.free).toHaveBeenCalledTimes(stage === "loader" ? 0 : 1);
    expect(view.container.querySelector('[role="alert"]')).toBeNull();
    fireEvent.pointerDown(newCanvas, { pointerId: 1, button: 1, clientX: fixture.pointer.x, clientY: fixture.pointer.y });
    expect(recovered.pointerDownScreen).toHaveBeenCalledExactlyOnceWith(fixture.pointer.x, fixture.pointer.y, 1);
    view.unmount();
    expect(recovered.free).toHaveBeenCalledOnce();
  } finally {
    disposeShellI18nInstance(language);
  }
});
