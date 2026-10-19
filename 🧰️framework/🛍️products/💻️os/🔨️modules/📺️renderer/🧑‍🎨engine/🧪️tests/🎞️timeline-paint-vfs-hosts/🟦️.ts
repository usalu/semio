/** 🎞 Graph timeline, vcs history, paint 2d and virtual file system hosts. */
import { act as reactAct, createElement } from "react";
import { renderToStaticMarkup } from "react-dom/server";
import { buildVirtualFileSystemSceneRows, detectShellLocale, uiI18n } from "@semio-tech/ui-react";
import { VirtualFileSystemHost, virtualFileSystemNavigation } from "../../🧱️elements/🗣️Interpreter/🗄️virtual-file-system/🟦️.tsx";
import { graphTimelineAuthorsFixture, presenceOverlayFixture, virtualFileSystemInteractionFixture } from "@semio-tech/framework/ui/testing/fixtures";
import * as flowSessionLoader from "../../🧱️elements/🪪️WasmSessionLoader/🟦️.tsx";
import { cleanup, fireEvent, render, waitFor } from "@semio-tech/ui-react/test";
import { afterEach, describe, expect, it, vi } from "vitest";
import { type UiComponentSceneNode } from "@semio-tech/framework";
import { Paint2dHost, GraphTimelineHost } from "../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";
import { noopAction, renderContractTree } from "../🧰️engine-contract-helpers/🟦️.ts";

describe("framework renderer hosts", () => {
  afterEach(() => cleanup());

  it("renders every GraphTimeline author in authored order with stable overlapping avatars", () => {
    const column = {
      checkpointId: graphTimelineAuthorsFixture.checkpointId,
      timestamp: "1",
      labels: ["main"],
      authors: graphTimelineAuthorsFixture.authors,
      description: "authored checkpoint",
      lane: 0,
      alternativeIds: [],
    };
    const mounted = render(
      createElement(GraphTimelineHost, {
        node: {
          type: "componentScene",
          surfaceId: "vcs.play.history.authors",
          controllerId: "vcs-play",
          componentKind: "graph-timeline",
          graphTimeline: { columnsJson: JSON.stringify([column]) },
        },
        onAction: noopAction,
      }),
    );
    const avatars = [...mounted.container.querySelectorAll<HTMLElement>('[data-slot="avatar"]')];
    expect(avatars.map((avatar) => avatar.id)).toEqual(graphTimelineAuthorsFixture.authors.map((author) => author.id));
    expect(avatars[0]?.parentElement?.classList.contains("-space-x-2")).toBe(true);
    expect(graphTimelineAuthorsFixture.avatarSize - graphTimelineAuthorsFixture.overlap).toBe(graphTimelineAuthorsFixture.advance);
    const image = avatars[0]?.querySelector<HTMLImageElement>('[data-slot="avatar-image"]');
    const fallback = avatars[0]?.querySelector<HTMLElement>('[data-slot="avatar-fallback"]');
    expect(image?.getAttribute("alt")).toBe("Ada Lovelace");
    expect(image?.getAttribute("src")).toBe(graphTimelineAuthorsFixture.authors[0]?.avatar);
    expect(image?.hidden).toBe(true);
    expect(fallback?.hidden).toBe(false);
    reactAct(() => image!.dispatchEvent(new Event("load")));
    expect(image?.hidden).toBe(false);
    expect(fallback?.hidden).toBe(true);
    expect(avatars[1]?.querySelector('[data-slot="avatar-image"]')).toBeNull();
    expect(avatars[1]?.querySelector('[data-slot="avatar-fallback"]')?.getAttribute("aria-label")).toBe("Grace Hopper");
    mounted.rerender(
      createElement(GraphTimelineHost, {
        node: {
          type: "componentScene",
          surfaceId: "vcs.play.history.authors",
          controllerId: "vcs-play",
          componentKind: "graph-timeline",
          graphTimeline: { columnsJson: JSON.stringify([{ ...column, authors: [{ ...column.authors[0], avatar: graphTimelineAuthorsFixture.replacementAvatar }, column.authors[1]] }]) },
        },
        onAction: noopAction,
      }),
    );
    const replacedImage = mounted.container.querySelector<HTMLImageElement>('#ada [data-slot="avatar-image"]');
    expect(replacedImage?.getAttribute("src")).toBe(graphTimelineAuthorsFixture.replacementAvatar);
    expect(replacedImage?.hidden).toBe(true);
  });

  it("renders vcs history host with an ancestor graph fork", () => {
    const columns = [
      {
        checkpointId: "c3",
        timestamp: "3",
        labels: ["feature-b"],
        authors: [],
        parentCheckpointId: "c2",
        description: "branch b",
        lane: 2,
        alternativeIds: ["b"],
      },
      {
        checkpointId: "c2",
        timestamp: "2",
        labels: ["feature-a"],
        authors: [],
        parentCheckpointId: "c1",
        description: "branch a",
        lane: 1,
        alternativeIds: ["a"],
      },
      {
        checkpointId: "c1",
        timestamp: "1",
        labels: ["main"],
        authors: [],
        description: "root",
        lane: 0,
        alternativeIds: [],
      },
    ];
    const markup = renderToStaticMarkup(
      createElement(GraphTimelineHost, {
        node: {
          type: "componentScene",
          surfaceId: "vcs.play.history",
          controllerId: "vcs-play",
          componentKind: "graph-timeline",
          graphTimeline: {
            columnsJson: JSON.stringify(columns),
          },
        },
        onAction: noopAction,
      }),
    );
    expect(markup).toContain("semio-graph-timeline-host");
    expect(markup).toContain('id="vcs.play.history.table"');
    expect(markup).toContain('d="M ');
    expect(markup.match(/<circle /g)?.length).toBe(3);
    expect(markup).toContain("branch b");
    expect(markup).toContain("feature-b");
  });

  it.each(["complete", "cancel"])("raster compositor %s settles frame demand and keeps canvas theme", async (mode) => {
    await uiI18n.changeLanguage("en");
    const fixture=(await import("../../🧱️elements/🖌️Paint2dHost/🧪️tests/🧫️fixtures/🔁️session/🔣️.json")).default;
    const uploadRasterImageKey=vi.fn(),syncDocumentJson=vi.fn(),attachCanvas=vi.fn(async()=>{}),setCamera=vi.fn();
    const canvasContext = vi.spyOn(HTMLCanvasElement.prototype, "getContext").mockReturnValue(null);
    let frames = 0,
      cancelled = false;
    const pending = () => !cancelled && (mode === "cancel" || frames < 12);
    const renderFrame = vi.fn(() => {
      frames++;
      return pending();
    });
    const cancelRender = vi.fn(() => {
      cancelled = true;
    });
    const setCanvasThemeJson = vi.fn();
    const session: flowSessionLoader.RasterWasmSession = {
      gpuReady: () => true,
      attachCanvas,
      setSize: () => {},
      renderFrame,
      setCamera,
      wheelScreen: () => {},
      pointerDownScreen: () => {},
      pointerMoveScreen: () => {},
      pointerUpScreen: () => {},
      pointerCancelScreen: () => {},
      syncDocumentJson,
      uploadLayerImage: () => {},
      uploadRasterImageKey,
      setActiveUtility: () => {},
      setBrushSize: () => {},
      setBrushOpacity: () => {},
      syncInteraction: () => {},
      setCanvasThemeJson,
      cameraJson: () => '{"x":0,"y":0,"zoom":1}',
      setViewMode: () => {},
      pickTargetsAtScreenJson: () => "[]",
      marqueeHitsJson: () => "[]",
      navigatorFitCameraJson: () => '{"x":0,"y":0,"zoom":1}',
      navigatorViewportOverlayJson: () => '{"x":0,"y":0,"width":1,"height":1}',
      free: vi.fn(),
      renderProgressJson: () => JSON.stringify({ pending: pending(), completed: Math.min(frames, 12), total: 12, error: null }),
      cancelRender,
    };
    const factory = vi.spyOn(flowSessionLoader, "createRasterSession").mockResolvedValue(session);
    const originalObserver = globalThis.ResizeObserver;
    vi.stubGlobal(
      "ResizeObserver",
      class {
        observe() {}
        unobserve() {}
        disconnect() {}
      },
    );
    const node:UiComponentSceneNode={type:"componentScene",surfaceId:"raster.play.viewport",controllerId:"raster-play",componentKind:"paint-2d",paint2d:{documentSyncJson:JSON.stringify(fixture.document),assetsJson:JSON.stringify(fixture.assets),cameraJson:JSON.stringify(fixture.camera),selectionJson:"[]",activeUtility:"selectMarquee",brushSize:24,brushOpacity:1,brushColor:"#2878dc",brushHardness:1,paintTarget:"pixels" as const,maskValue:255,fillTolerance:24,viewMode:"composite"}};
    const view=render(createElement(Paint2dHost,{node,onAction:noopAction}));
    try {
      await waitFor(() => expect(setCanvasThemeJson).toHaveBeenCalled());
      expect(setCanvasThemeJson.mock.calls.length).toBeGreaterThanOrEqual(2);
      await waitFor(()=>expect(uploadRasterImageKey).toHaveBeenCalledTimes(fixture.uploads));
      expect(syncDocumentJson).toHaveBeenCalledWith(JSON.stringify(fixture.document));
      expect(uploadRasterImageKey.mock.calls[0]).toEqual(["source",new Uint8Array(Buffer.from(fixture.assets.source.data,"base64"))]);
      expect(setCamera).toHaveBeenCalledWith(fixture.camera.x,fixture.camera.y,fixture.camera.zoom);
      if (mode === "cancel") {
        await waitFor(() => expect(view.getByRole("button", { name: "Cancel rendering" })).toBeTruthy());
        fireEvent.click(view.getByRole("button", { name: "Cancel rendering" }));
        expect(cancelRender).toHaveBeenCalled();
      } else await waitFor(() => expect(frames).toBeGreaterThanOrEqual(12));
      await waitFor(() => expect(view.container.querySelector('progress[aria-label="Rendering image"]')).toBeNull());
      view.rerender(createElement(Paint2dHost,{node:{...node,paint2d:{...node.paint2d!,cameraJson:JSON.stringify(fixture.updatedCamera),selectionJson:'["paint"]'}},onAction:noopAction}));
      await waitFor(()=>expect(setCamera).toHaveBeenCalledWith(fixture.updatedCamera.x,fixture.updatedCamera.y,fixture.updatedCamera.zoom));
      expect(attachCanvas).toHaveBeenCalledTimes(fixture.attachments);
      expect(uploadRasterImageKey).toHaveBeenCalledTimes(fixture.uploads);
      const settled = renderFrame.mock.calls.length;
      await new Promise((resolve) => setTimeout(resolve, 80));
      expect(renderFrame.mock.calls.length).toBe(settled);
    } finally {
      view.unmount();
      factory.mockRestore();
      canvasContext.mockRestore();
      vi.stubGlobal("ResizeObserver", originalObserver);
    }
    expect(session.free).toHaveBeenCalledOnce();
  });

  it("retires a mounted paint gesture on pointer cancel without an up or action commit", async () => {
    const attachCanvas = vi.fn(async () => {});
    const pointerDownScreen = vi.fn();
    const pointerUpScreen = vi.fn();
    const pointerCancelScreen = vi.fn();
    const onAction = vi.fn();
    const session: flowSessionLoader.RasterWasmSession = {
      gpuReady: () => true,
      attachCanvas,
      setSize: () => {},
      renderFrame: vi.fn(() => false),
      setCamera: () => {},
      wheelScreen: () => {},
      pointerDownScreen,
      pointerMoveScreen: () => {},
      pointerUpScreen,
      pointerCancelScreen,
      syncDocumentJson: () => {},
      uploadLayerImage: () => {},
      uploadRasterImageKey: () => {},
      setActiveUtility: () => {},
      setBrushSize: () => {},
      setBrushOpacity: () => {},
      syncInteraction: () => {},
      setCanvasThemeJson: () => {},
      cameraJson: () => '{"x":0,"y":0,"zoom":1}',
      setViewMode: () => {},
      pickTargetsAtScreenJson: () => "[]",
      marqueeHitsJson: () => "[]",
      navigatorFitCameraJson: () => '{"x":0,"y":0,"zoom":1}',
      navigatorViewportOverlayJson: () => '{"x":0,"y":0,"width":1,"height":1}',
      free: vi.fn(),
      renderProgressJson: () => JSON.stringify({ pending: false, completed: 0, total: 0, error: null }),
      cancelRender: () => {},
    };
    const factory = vi.spyOn(flowSessionLoader, "createRasterSession").mockResolvedValue(session);
    const originalObserver = globalThis.ResizeObserver;
    vi.stubGlobal(
      "ResizeObserver",
      class {
        observe() {}
        unobserve() {}
        disconnect() {}
      },
    );
    const bounds = vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockReturnValue(new DOMRect(0, 0, 640, 480));
    const view = render(
      createElement(Paint2dHost, {
        node: {
          type: "componentScene",
          surfaceId: "raster.cancel",
          controllerId: "raster",
          componentKind: "paint-2d",
          paint2d: {
            documentSyncJson: '{"schema":"raster.document","id":"raster","layers":[]}',
            assetsJson: "{}",
            cameraJson: '{"x":0,"y":0,"zoom":1}',
            selectionJson: "[]",
            activeUtility: "paintBrush",
            brushSize: 24,
            brushOpacity: 1,
            brushColor: "#2878dc",
            brushHardness: 1,paintTarget:"pixels" as const,maskValue:255,fillTolerance:24,
            viewMode: "composite",
          },
        },
        onAction,
      }),
    );
    try {
      const surface = view.container.querySelector('.semio-paint-2d-canvas-surface [class*="z-30"]') as HTMLElement;
      Object.defineProperty(surface, "hasPointerCapture", { configurable: true, value: () => false });
      await waitFor(() => expect(attachCanvas).toHaveBeenCalledOnce());
      const actionsBeforeCancel = onAction.mock.calls.length;
      fireEvent.pointerDown(surface, { pointerId: 7, button: 0, clientX: 10, clientY: 12 });
      expect(pointerDownScreen).toHaveBeenCalledOnce();
      fireEvent.pointerCancel(surface, { pointerId: 7, button: 0, clientX: 20, clientY: 22 });
      expect(pointerCancelScreen).toHaveBeenCalledOnce();
      expect(pointerUpScreen).not.toHaveBeenCalled();
      expect(onAction.mock.calls.slice(actionsBeforeCancel).map(([action]) => action.action)).toEqual(["interactionHover"]);
      fireEvent.pointerDown(surface, { pointerId: 8, button: 0, clientX: 30, clientY: 32 });
      expect(pointerDownScreen).toHaveBeenCalledTimes(2);
    } finally {
      view.unmount();
      factory.mockRestore();
      bounds.mockRestore();
      vi.stubGlobal("ResizeObserver", originalObserver);
    }
  });

  it("synchronizes raster selection and hover through the current native interaction API", async () => {
    const syncInteraction = vi.fn();
    const session: flowSessionLoader.RasterWasmSession = {
      gpuReady: () => true,
      attachCanvas: async () => {},
      setSize: () => {},
      renderFrame: () => false,
      setCamera: () => {},
      wheelScreen: () => {},
      pointerDownScreen: () => {},
      pointerMoveScreen: () => {},
      pointerUpScreen: () => {},
      pointerCancelScreen: () => {},
      syncDocumentJson: () => {},
      uploadLayerImage: () => {},
      uploadRasterImageKey: () => {},
      setActiveUtility: () => {},
      setBrushSize: () => {},
      setBrushOpacity: () => {},
      syncInteraction,
      setCanvasThemeJson: () => {},
      cameraJson: () => '{"x":0,"y":0,"zoom":1}',
      setViewMode: () => {},
      pickTargetsAtScreenJson: () => "[]",
      marqueeHitsJson: () => "[]",
      navigatorFitCameraJson: () => '{"x":0,"y":0,"zoom":1}',
      navigatorViewportOverlayJson: () => '{"x":0,"y":0,"width":1,"height":1}',
      free: vi.fn(),
      renderProgressJson: () => JSON.stringify({ pending: false, completed: 0, total: 0, error: null }),
      cancelRender: () => {},
    };
    const factory = vi.spyOn(flowSessionLoader, "createRasterSession").mockResolvedValue(session);
    const originalObserver = globalThis.ResizeObserver;
    vi.stubGlobal(
      "ResizeObserver",
      class {
        observe() {}
        unobserve() {}
        disconnect() {}
      },
    );
    const content = (vector: (typeof presenceOverlayFixture.cases)[number]) =>
      createElement(Paint2dHost, {
        node: {
          type: "componentScene",
          surfaceId: "raster.play.viewport",
          controllerId: "raster-play",
          componentKind: "paint-2d",
          paint2d: {
            documentSyncJson: '{"schema":"raster.document","id":"raster","layers":[]}',
            assetsJson: "{}",
            cameraJson: '{"x":0,"y":0,"zoom":1}',
            selectionJson: JSON.stringify(vector.expected.selected ? [vector.update.nodeKey] : []),
            hoveredId: vector.expected.hovered ? vector.update.nodeKey : undefined,
            activeUtility: "selectMarquee",
            brushSize: 24,
            brushOpacity: 1,
            brushColor: "#2878dc",
            brushHardness: 1,paintTarget:"pixels" as const,maskValue:255,fillTolerance:24,
            viewMode: "composite",
          },
        },
        onAction: noopAction,
      });
    const view = render(content(presenceOverlayFixture.cases[0]!));
    try {
      for (const vector of presenceOverlayFixture.cases) {
        view.rerender(content(vector));
        await waitFor(() => expect(syncInteraction).toHaveBeenLastCalledWith(JSON.stringify(vector.expected.selected ? [vector.update.nodeKey] : []), vector.expected.hovered ? vector.update.nodeKey : null));
      }
    } finally {
      view.unmount();
      factory.mockRestore();
      vi.stubGlobal("ResizeObserver", originalObserver);
    }
    expect(session.free).toHaveBeenCalledOnce();
  });

  it("renders paint-2d host canvas surface from document sync scene", () => {
    const markup = renderToStaticMarkup(
      createElement(Paint2dHost, {
        node: {
          type: "componentScene",
          surfaceId: "raster.play.viewport",
          controllerId: "raster-play",
          componentKind: "paint-2d",
          paint2d: {
            documentSyncJson: '{"schema":"raster.document","id":"raster","layers":[]}',
            assetsJson: "{}",
            cameraJson: '{"x":0,"y":0,"zoom":1}',
            selectionJson: "[]",
            activeUtility: "selectMarquee",
            brushSize: 24,
            brushOpacity: 1,
            brushColor: "#2878dc",
            brushHardness: 1,paintTarget:"pixels" as const,maskValue:255,fillTolerance:24,
            viewMode: "composite",
          },
        },
        onAction: noopAction,
      }),
    );
    expect(markup).toContain("semio-paint-2d-canvas-surface");
    expect(markup).toContain('data-surface-id="raster.play.viewport"');
    expect(markup).toContain('data-view-mode="composite"');
  });

  it("renders paint-2d navigator host with the composite viewport overlay channel", () => {
    const markup = renderToStaticMarkup(
      createElement(Paint2dHost, {
        node: {
          type: "componentScene",
          surfaceId: "raster.play.navigator",
          controllerId: "raster-play",
          componentKind: "paint-2d",
          paint2d: {
            documentSyncJson: '{"schema":"raster.document","id":"raster","layers":[]}',
            assetsJson: "{}",
            cameraJson: '{"x":0,"y":0,"zoom":1}',
            selectionJson: "[]",
            activeUtility: "selectMarquee",
            brushSize: 24,
            brushOpacity: 1,
            brushColor: "#2878dc",
            brushHardness: 1,paintTarget:"pixels" as const,maskValue:255,fillTolerance:24,
            viewMode: "navigator",
            compositeViewportJson: '{"width":640,"height":480}',
          },
        },
        onAction: noopAction,
      }),
    );
    expect(markup).toContain("semio-paint-2d-canvas-surface");
    expect(markup).toContain('data-view-mode="navigator"');
  });

  it("renders paint-2d host empty fallback without a scene", () => {
    const markup = renderToStaticMarkup(
      createElement(Paint2dHost, {
        node: {
          type: "componentScene",
          surfaceId: "raster.play.composite",
          controllerId: "raster-play",
          componentKind: "paint-2d",
        },
        onAction: noopAction,
      }),
    );
    expect(markup).toContain("semio-paint-2d-empty");
  });

  it("validates and flattens the shared raw virtual file system interaction law", () => {
    for (const vector of virtualFileSystemInteractionFixture.visibility) {
      const rows = buildVirtualFileSystemSceneRows(virtualFileSystemInteractionFixture.rows, new Set(vector.expandedRowIds));
      expect(
        rows.map((row) => row.id),
        vector.id,
      ).toEqual(vector.visibleRowIds);
      expect(
        rows.map((row) => row.level),
        vector.id,
      ).toEqual(vector.levels);
    }
    for (const vector of virtualFileSystemInteractionFixture.navigation) {
      const row = virtualFileSystemInteractionFixture.rows.find((candidate) => candidate.id === vector.rowId)!;
      expect(virtualFileSystemNavigation(row), vector.rowId).toEqual(vector.action ? { action: vector.action, args: vector.args } : null);
    }
  });

  it("mounts localized raw virtual file system controls, owns expansion, and dispatches navigateUri only on double-click", async () => {
    const previousLocale = detectShellLocale(uiI18n.resolvedLanguage || uiI18n.language);
    await uiI18n.changeLanguage("en");
    const onAction = vi.fn();
    const view = render(
      createElement(VirtualFileSystemHost, {
        node: {
          type: "componentScene",
          surfaceId: "vfs.interaction",
          controllerId: "vfs-controller",
          componentKind: "virtual-file-system",
          virtualFileSystem: {
            schemaJson: JSON.stringify({
              fileNodeKinds: {
                root: { id: "root", name: "Root", descriptors: [] },
                branch: { id: "branch", name: "Branch", descriptors: [] },
                leaf: { id: "leaf", name: "Leaf", descriptors: [] },
              },
              descriptorKinds: {},
              descriptorColumnIds: [],
            }),
            rowsJson: JSON.stringify(virtualFileSystemInteractionFixture.rows),
          },
        },
        onAction,
      }),
    );
    try {
      const folder = view.container.querySelector('tr[data-row-id="folder"]') as HTMLElement;
      expect(folder).toBeTruthy();
      expect(view.container.querySelector('tr[data-row-id="instance"]')).toBeTruthy();
      const toggle = folder.querySelector("button[data-vfs-expand]") as HTMLButtonElement;
      const english = virtualFileSystemInteractionFixture.chrome.find((pack) => pack.locale === "en")!;
      expect(toggle.getAttribute("aria-label")).toBe(english.collapse);
      expect(toggle.getAttribute("aria-expanded")).toBe("true");
      fireEvent.click(toggle);
      expect(view.container.querySelector('tr[data-row-id="instance"]')).toBeNull();
      const collapsedToggle = view.container.querySelector('tr[data-row-id="folder"] button[data-vfs-expand]') as HTMLButtonElement;
      expect(collapsedToggle.getAttribute("aria-label")).toBe(english.expand);
      expect(collapsedToggle.getAttribute("aria-expanded")).toBe("false");
      fireEvent.click(collapsedToggle);
      const instance = view.container.querySelector('tr[data-row-id="instance"]') as HTMLElement;
      expect(instance).toBeTruthy();
      fireEvent.click(instance, { detail: 1 });
      fireEvent.click(instance, { detail: 2 });
      expect(onAction).toHaveBeenLastCalledWith({
        controllerId: "vfs-controller",
        action: "openInstance",
        args: { surfaceId: "vfs.interaction", instanceId: "inst-7" },
      });
      expect(onAction.mock.calls.filter(([action]) => action.action === "selectRows")).toHaveLength(1);
      const german = virtualFileSystemInteractionFixture.chrome.find((pack) => pack.locale === "de")!;
      await reactAct(async () => {
        await uiI18n.changeLanguage("de");
      });
      expect((view.container.querySelector('tr[data-row-id="folder"] button[data-vfs-expand]') as HTMLButtonElement).getAttribute("aria-label")).toBe(german.collapse);
    } finally {
      view.unmount();
      await uiI18n.changeLanguage(previousLocale);
    }
  });

  it("interprets virtual file system component scenes", async () => {
    const doc = {
      schemaJson: JSON.stringify({
        fileNodeKinds: { instance: { id: "instance", name: "Instance", descriptors: [] } },
        descriptorKinds: {},
        descriptorColumnIds: [],
      }),
      rowsJson: JSON.stringify([{ id: "row-1", fileNodeKindId: "instance", name: "Draw", path: "/draw", level: 0 }]),
    };
    const docBytes = [13, 2];
    const pushLength = (value: number) => {
      for (let remaining = value; ; remaining = Math.floor(remaining / 128)) {
        docBytes.push((remaining % 128) | (remaining >= 128 ? 128 : 0));
        if (remaining < 128) break;
      }
    };
    for (const [key, value] of Object.entries(doc)) {
      const keyBytes = new TextEncoder().encode(key);
      const valueBytes = new TextEncoder().encode(value);
      docBytes.push(6);
      pushLength(keyBytes.length);
      docBytes.push(...keyBytes, 6);
      pushLength(valueBytes.length);
      docBytes.push(...valueBytes);
    }
    const markup = renderContractTree({
      key: "s.play.media-vfs",
      component: {
        type: "surface",
        kind: "virtual-file-system",
        docSchema: "virtual-file-system@1",
        doc: { bytes: docBytes },
        bindings: [],
      },
    });
    expect(markup).toContain("Draw");
  });
});
