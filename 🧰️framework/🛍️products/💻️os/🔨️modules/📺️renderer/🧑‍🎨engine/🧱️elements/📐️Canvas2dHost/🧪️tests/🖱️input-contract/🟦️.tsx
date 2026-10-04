/** 🖱️ Mounted Canvas2d input oracle: the real host owns catalogue DOM events while the
 * installed infinite-canvas session seam owns pointer/cancel/double-click events. Both must publish
 * the shared language-neutral fixture without changing MIME text, coordinates, or modifiers. */
import { cleanup, fireEvent, render } from "@semio-tech/ui-react/test";
import { createElement, type MouseEvent as ReactMouseEvent, type PointerEvent as ReactPointerEvent, type WheelEvent as ReactWheelEvent } from "react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import fixture from "../../🧫️fixtures/🖱️input-contract/🔣️.json" with { type: "json" };
import surfaceBehavior from "../../../../🧫️fixtures/🎬️surface-behavior/🔣️.json" with { type: "json" };

import Ajv2020 from "ajv/dist/2020";

import catalogueTerminal from "../../../../../../../../../🔨️modules/🖱️ui/🧫️fixtures/🛒️canvas-catalogue-terminal/🔣️.json" with { type: "json" };
import catalogueTerminalSchema from "../../../../../../../../../🔨️modules/🖱️ui/🧬️schema/🛒️canvas-catalogue-terminal/🔣️.json" with { type: "json" };
import cataloguePointerTransfer from "../../../../../../../../../🔨️modules/🖱️ui/🧫️fixtures/🛒️canvas-catalogue-pointer-transfer/🔣️.json" with { type: "json" };
import cataloguePointerTransferSchema from "../../../../../../../../../🔨️modules/🖱️ui/🧬️schema/🛒️canvas-catalogue-pointer-transfer/🔣️.json" with { type: "json" };
import cameraGestures from "../../../../../../../../../🔨️modules/🖱️ui/🧫️fixtures/🧭️canvas2d-camera-gestures/🔣️.json" with { type: "json" };
import cameraGesturesSchema from "../../../../../../../../../🔨️modules/🖱️ui/🧬️schema/🧭️canvas2d-camera-gestures/🔣️.json" with { type: "json" };
import { Tree, catalogueTreeDragController, getActiveCataloguePointerDragData } from "@semio-tech/ui-react";

import framingFixture from "../../../../../../../../../🔨️modules/🖱️ui/🎬️scene/📷️framing/🧫️fixtures/🔣️.json" with { type: "json" };

import { mountedHost, canvasInputSessions, settle, bounds, type HostAction } from "../../🔮️oracles/🖱️mounted-input/🟦️.tsx";
import { Canvas2dHost, wheelCameraAtScreen } from "../../🟦️.tsx";

describe("🖱️ Canvas2d mounted input contract", () => {
  beforeEach(() => {
    canvasInputSessions().length = 0;
    vi.stubGlobal("ResizeObserver", class {
      observe() {}
      disconnect() {}
    });
  });

  afterEach(() => {
    cleanup();
    vi.unstubAllGlobals();
  });

  it("preserves all four modifiers and publishes one cancelled terminal action", async () => {
    const { actions, canvas } = mountedHost();
    fireEvent.pointerDown(canvas, {
      clientX: fixture.pointer.down.x,
      clientY: fixture.pointer.down.y,
      button: fixture.pointer.down.button,
      shiftKey: fixture.pointer.down.modifiers.shift,
      ctrlKey: fixture.pointer.down.modifiers.ctrl,
      metaKey: fixture.pointer.down.modifiers.meta,
      altKey: fixture.pointer.down.modifiers.alt,
    });
    fireEvent.pointerCancel(canvas);
    await settle();
    expect(actions.map(({ action }) => action)).toEqual(["canvasPointerDown", "canvasPointerUp"]);
    expect(actions[0]).toEqual({
      controllerId: fixture.surface.controllerId,
      action: "canvasPointerDown",
      args: {
        surfaceId: fixture.surface.id,
        x: fixture.pointer.down.x,
        y: fixture.pointer.down.y,
        button: fixture.pointer.down.button,
        ...fixture.pointer.down.modifiers,
        width: fixture.surface.width,
        height: fixture.surface.height,
        worldX: fixture.pointer.down.world.x,
        worldY: fixture.pointer.down.world.y,
      },
    });
    expect(actions[1]?.args).toEqual({
      surfaceId: fixture.surface.id,
      x: fixture.pointer.cancel.x,
      y: fixture.pointer.cancel.y,
      shift: false,
      ctrl: false,
      meta: false,
      alt: false,
      width: fixture.surface.width,
      height: fixture.surface.height,
      worldX: fixture.pointer.cancel.world.x,
      worldY: fixture.pointer.cancel.world.y,
      cancelled: true,
    });
  });

  it("matches the Draw terminal receipt, stale continuation, and fresh-gesture policy", async () => {
    const law = surfaceBehavior.cases.find(({ id }) => id === "draw-canvas2d")!;
    const surface = { id: "draw.surface.behavior", controllerId: law.controllerId, width: 800, height: 600 };
    const { actions, canvas } = mountedHost(surface);
    fireEvent.pointerDown(canvas, { pointerId: 7, button: 0, clientX: law.points.down.x, clientY: law.points.down.y });
    fireEvent.pointerMove(canvas, { pointerId: 7, buttons: 1, clientX: law.points.move.x, clientY: law.points.move.y });
    fireEvent.pointerCancel(canvas, { pointerId: 7, clientX: law.points.terminal.x, clientY: law.points.terminal.y });
    await settle();
    expect(actions.map(({ action }) => action)).toEqual([...law.acceptedBeforeCancel.map(({ action }) => action), law.publishedOnCancel[0]!.action]);
    expect(actions.at(-1)?.args?.cancelled).toBe(true);

    const cancelledCount = actions.length;
    fireEvent.pointerMove(canvas, { pointerId: 7, buttons: 1, clientX: law.points.move.x + 20, clientY: law.points.move.y + 20 });
    fireEvent.pointerUp(canvas, { pointerId: 7, button: 0, clientX: law.points.terminal.x + 20, clientY: law.points.terminal.y + 20 });
    fireEvent.pointerCancel(canvas, { pointerId: 7 });
    await settle();
    expect(actions).toHaveLength(cancelledCount);

    fireEvent.pointerDown(canvas, { pointerId: 8, button: 0, clientX: law.points.down.x, clientY: law.points.down.y });
    fireEvent.pointerMove(canvas, { pointerId: 8, buttons: 1, clientX: law.points.move.x, clientY: law.points.move.y });
    fireEvent.pointerUp(canvas, { pointerId: 8, button: 0, clientX: law.points.terminal.x, clientY: law.points.terminal.y });
    await settle();
    expect(actions.slice(cancelledCount).map(({ action }) => action)).toEqual(["canvasPointerDown", "canvasPointerMove", "canvasPointerUp"]);
    expect(actions.at(-1)?.args?.cancelled).toBe(false);
  });

  it("preserves catalogue MIME data and surface coordinates through drag-over and terminal leave/drop", async () => {
    const { actions, host } = mountedHost();
    const dataTransfer = {
      types: fixture.catalogue.types,
      getData: (mime: string) => (mime === fixture.catalogue.mime ? fixture.catalogue.rawPayload : "Catalogue item"),
    };
    for (const type of ["dragover", "drop"]) {
      const event = new MouseEvent(type, { bubbles: true, cancelable: true, clientX: fixture.catalogue.point.x, clientY: fixture.catalogue.point.y });
      Object.defineProperty(event, "dataTransfer", { value: dataTransfer });
      host.dispatchEvent(event);
    }
    await settle();
    expect(actions).toEqual([
      {
        controllerId: fixture.surface.controllerId,
        action: "canvasDragOver",
        args: {
          surfaceId: fixture.surface.id,
          x: fixture.catalogue.point.x,
          y: fixture.catalogue.point.y,
          width: fixture.surface.width,
          height: fixture.surface.height,
          types: fixture.catalogue.types,
        },
      },
      {
        controllerId: fixture.surface.controllerId,
        action: "canvasDragLeave",
        args: { surfaceId: fixture.surface.id },
      },
      {
        controllerId: fixture.surface.controllerId,
        action: "canvasDrop",
        args: {
          surfaceId: fixture.surface.id,
          x: fixture.catalogue.point.x,
          y: fixture.catalogue.point.y,
          width: fixture.surface.width,
          height: fixture.surface.height,
          dragData: fixture.catalogue.rawPayload,
        },
      },
    ]);
  });

  it("validates the shared catalogue terminal fixture", () => {
    expect(new Ajv2020({ strict: true }).compile(catalogueTerminalSchema)(catalogueTerminal)).toBe(true);
  });

  it("validates the renderer-neutral catalogue pointer transfer fixture", () => {
    expect(new Ajv2020({ strict: true }).compile(cataloguePointerTransferSchema)(cataloguePointerTransfer)).toBe(true);
  });

  it("validates the renderer-neutral Canvas2d camera gesture fixture", () => {
    expect(new Ajv2020({ strict: true }).compile(cameraGesturesSchema)(cameraGestures)).toBe(true);
  });

  it("keeps the exact world point under an off-centre wheel anchor and clamps both zoom limits", () => {
    const { initialCamera, anchor, zoomInDeltaY, zoomOutDeltaY, zoomInFactor, zoomOutFactor, zoomMinimum, zoomMaximum } = cameraGestures.wheel;
    const { width, height } = cameraGestures.surface;
    const world = {
      x: (anchor.x - width * 0.5) / initialCamera.zoom + initialCamera.x,
      y: (anchor.y - height * 0.5) / initialCamera.zoom + initialCamera.y,
    };
    const zoomedIn = wheelCameraAtScreen(initialCamera, anchor.x, anchor.y, zoomInDeltaY, width, height);
    expect(zoomedIn.zoom).toBe(initialCamera.zoom * zoomInFactor);
    expect((anchor.x - width * 0.5) / zoomedIn.zoom + zoomedIn.x).toBe(world.x);
    expect((anchor.y - height * 0.5) / zoomedIn.zoom + zoomedIn.y).toBe(world.y);
    expect(wheelCameraAtScreen(initialCamera, anchor.x, anchor.y, zoomOutDeltaY, width, height).zoom).toBe(initialCamera.zoom * zoomOutFactor);
    expect(wheelCameraAtScreen({ ...initialCamera, zoom: zoomMinimum }, anchor.x, anchor.y, zoomOutDeltaY, width, height).zoom).toBe(zoomMinimum);
    expect(wheelCameraAtScreen({ ...initialCamera, zoom: zoomMaximum }, anchor.x, anchor.y, zoomInDeltaY, width, height).zoom).toBe(zoomMaximum);
  });

  it("publishes the mounted host's exact cursor-anchored wheel camera once after settling", async () => {
    vi.useFakeTimers();
    try {
      const { initialCamera, anchor, zoomInDeltaY, expectedAction } = cameraGestures.wheel;
      const { actions, canvas } = mountedHost(cameraGestures.surface, undefined, undefined, initialCamera);
      canvasInputSessions().at(-1)?.setSize(cameraGestures.surface.width, cameraGestures.surface.height, 1);
      canvas.dispatchEvent(new WheelEvent("wheel", { bubbles: true, clientX: anchor.x, clientY: anchor.y, deltaY: zoomInDeltaY }));
      expect(actions).toEqual([]);
      await vi.advanceTimersByTimeAsync(cameraGestures.wheel.settleDelayMs - 1);
      expect(actions).toEqual([]);
      await vi.advanceTimersByTimeAsync(1);
      expect(actions).toEqual([{
        controllerId: cameraGestures.surface.controllerId,
        action: expectedAction,
        args: {
          surfaceId: cameraGestures.surface.id,
          camera: wheelCameraAtScreen(initialCamera, anchor.x, anchor.y, zoomInDeltaY, cameraGestures.surface.width, cameraGestures.surface.height),
        },
      }]);
    } finally {
      vi.useRealTimers();
    }
  });

  it.each(cameraGestures.wheelBursts)("applies every $id wheel event in order and publishes one settled camera", async row => {
    vi.useFakeTimers();
    try {
      const { initialCamera, anchor, expectedAction } = cameraGestures.wheel;
      const { width, height } = cameraGestures.surface;
      const expected = row.deltas.reduce((camera, deltaY) => wheelCameraAtScreen(camera, anchor.x, anchor.y, deltaY, width, height), initialCamera);
      expect(expected.zoom).toBe(initialCamera.zoom * row.expectedFactors[0] * row.expectedFactors[1]);
      if (row.id === "zoom-in-then-out-events") expect(expected.zoom).not.toBe(initialCamera.zoom);
      const { actions, canvas } = mountedHost(cameraGestures.surface, undefined, undefined, initialCamera);
      canvasInputSessions().at(-1)?.setSize(width, height, 1);
      for (const deltaY of row.deltas) {
        canvas.dispatchEvent(new WheelEvent("wheel", { bubbles: true, clientX: anchor.x, clientY: anchor.y, deltaY }));
      }
      expect(actions).toEqual([]);
      await vi.advanceTimersByTimeAsync(120);
      const cameraActions = actions.filter(({ action }) => action === expectedAction);
      expect(cameraActions).toHaveLength(row.expectedActionCount);
      expect(cameraActions[0]?.args?.camera).toEqual(expected);
    } finally {
      vi.useRealTimers();
    }
  });

  it("releases every sequentially closed Canvas window beyond the retained surface capacity", async () => {
    vi.useFakeTimers();
    try {
      const law = cameraGestures.sequentialWindowLifetime;
      const retiredActions: HostAction[][] = [];
      for (let index = 0; index < law.mountCount; index += 1) {
        const surface = { ...cameraGestures.surface, id: `sequential-canvas-window-${index}` };
        const { actions, canvas, unmount } = mountedHost(surface, undefined, undefined, cameraGestures.wheel.initialCamera);
        expect(document.querySelectorAll(".semio-canvas-2d-host")).toHaveLength(law.maximumLiveWindows);
        canvas.dispatchEvent(new WheelEvent("wheel", { bubbles: true, clientX: surface.width / 2, clientY: surface.height / 2, deltaY: cameraGestures.wheel.zoomInDeltaY }));
        retiredActions.push(actions);
        unmount();
        expect(document.querySelectorAll(".semio-canvas-2d-host")).toHaveLength(law.liveWindowsAfterClose);
      }
      await vi.advanceTimersByTimeAsync(120);
      expect(retiredActions.flat().filter(({ action }) => action === cameraGestures.wheel.expectedAction)).toHaveLength(law.retiredDeadlineActions);
    } finally {
      vi.useRealTimers();
    }
  });

  it("keeps sibling Canvas cameras independent under one document surface identity", async () => {
    vi.useFakeTimers();
    try {
      const law = cameraGestures.siblingWindowLifetime;
      const mounted = law.cameras.map(camera => mountedHost(cameraGestures.surface, undefined, undefined, camera));
      for (const session of canvasInputSessions()) session.setSize(cameraGestures.surface.width, cameraGestures.surface.height, 1);
      const wheel = (canvas: HTMLCanvasElement) => canvas.dispatchEvent(new WheelEvent("wheel", { bubbles: true, clientX: 200, clientY: 150, deltaY: cameraGestures.wheel.zoomInDeltaY }));
      for (const { canvas } of mounted) wheel(canvas);
      await vi.advanceTimersByTimeAsync(120);
      mounted.forEach(({ actions }, index) => {
        expect(actions).toHaveLength(1);
        expect(actions[0]?.args?.camera).toEqual({ ...law.cameras[index], zoom: law.expectedZooms[index] });
      });
      mounted[0]!.actions.length = 0;
      mounted[0]!.unmount();
      wheel(mounted[1]!.canvas);
      await vi.advanceTimersByTimeAsync(120);
      expect(mounted[0]!.actions).toHaveLength(law.retiredActions);
      expect((mounted[1]!.actions.at(-1)?.args?.camera as { zoom: number }).zoom).toBeCloseTo(law.survivingZoom);
    } finally {
      vi.useRealTimers();
    }
  });

  it("silently retires an active Canvas gesture when its window unmounts", async () => {
    const law = cameraGestures.sequentialWindowLifetime;
    const { actions, canvas, unmount } = mountedHost();
    fireEvent.pointerDown(canvas, { clientX: 10, clientY: 10, button: 0 });
    await settle();
    expect(actions.map(({ action }) => action)).toEqual(law.activeGestureActions);
    actions.length = 0;
    unmount();
    fireEvent.pointerCancel(canvas);
    fireEvent.pointerUp(canvas, { clientX: 10, clientY: 10, button: 0 });
    await settle();
    expect(actions).toHaveLength(law.closedGestureActions);
  });

  it("keeps same-key camera ownership and retires every replaced or removed debounce", async () => {
    vi.useFakeTimers();
    try {
      const [initial, refreshed, replacement] = cameraGestures.mountLifetime.steps;
      const actionsA: HostAction[] = [];
      const actionsB: HostAction[] = [];
      const onActionA = (action: HostAction) => {
        actionsA.push(action);
        return Promise.resolve();
      };
      const onActionB = (action: HostAction) => {
        actionsB.push(action);
        return Promise.resolve();
      };
      const host = (key: string, authoredZoom: number, onAction: (action: HostAction) => Promise<void>) =>
        createElement(Canvas2dHost, {
          key,
          node: {
            type: "componentScene",
            surfaceId: cameraGestures.surface.id,
            controllerId: cameraGestures.surface.controllerId,
            componentKind: "canvas-2d",
            canvas2d: { cameraX: 0, cameraY: 0, zoom: authoredZoom, layersJson: "[]" },
          },
          onAction,
        } as never);
      const view = render(host(initial.key, initial.authoredZoom, onActionA));
      const canvas = () => view.container.querySelector('[data-testid="canvas-input"]') as HTMLCanvasElement;
      const wheel = () => {
        canvas().getBoundingClientRect = () => bounds;
        canvas().dispatchEvent(new WheelEvent("wheel", { bubbles: true, clientX: fixture.surface.width / 2, clientY: fixture.surface.height / 2, deltaY: cameraGestures.wheel.zoomInDeltaY }));
      };

      wheel();
      view.rerender(host(refreshed.key, refreshed.authoredZoom, onActionA));
      await vi.advanceTimersByTimeAsync(120);
      expect((actionsA.at(-1)?.args?.camera as { readonly zoom: number }).zoom).toBeCloseTo(initial.expectedZoom, 12);

      wheel();
      await vi.advanceTimersByTimeAsync(120);
      expect((actionsA.at(-1)?.args?.camera as { readonly zoom: number }).zoom).toBeCloseTo(refreshed.expectedZoom, 12);

      wheel();
      const retiredCount = actionsA.length;
      view.rerender(host(replacement.key, replacement.authoredZoom, onActionB));
      await vi.advanceTimersByTimeAsync(120);
      expect(actionsA.length - retiredCount).toBe(cameraGestures.mountLifetime.retiredDeadlineActions);

      wheel();
      await vi.advanceTimersByTimeAsync(120);
      expect((actionsB.at(-1)?.args?.camera as { readonly zoom: number }).zoom).toBeCloseTo(replacement.expectedZoom, 12);

      wheel();
      const removedCount = actionsB.length;
      view.unmount();
      await vi.advanceTimersByTimeAsync(120);
      expect(actionsB.length - removedCount).toBe(cameraGestures.mountLifetime.removedCameraActions);
    } finally {
      vi.useRealTimers();
    }
  });

  it("preserves a same-key scene refresh and resets that key after a real component remount", async () => {
    vi.useFakeTimers();
    try {
      const law = cameraGestures.sameKeySceneRemount;
      const actions: HostAction[] = [];
      const onAction = (action: HostAction) => {
        actions.push(action);
        return Promise.resolve();
      };
      const host = (authoredZoom: number) =>
        createElement(Canvas2dHost, {
          key: law.key,
          node: {
            type: "componentScene",
            surfaceId: cameraGestures.surface.id,
            controllerId: cameraGestures.surface.controllerId,
            componentKind: "canvas-2d",
            canvas2d: { cameraX: 0, cameraY: 0, zoom: authoredZoom, layersJson: "[]" },
          },
          onAction,
        } as never);
      const view = render(host(law.initialAuthoredZoom));
      const wheel = () => {
        const canvas = view.container.querySelector('[data-testid="canvas-input"]') as HTMLCanvasElement;
        canvas.getBoundingClientRect = () => bounds;
        canvas.dispatchEvent(new WheelEvent("wheel", { bubbles: true, clientX: fixture.surface.width / 2, clientY: fixture.surface.height / 2, deltaY: cameraGestures.wheel.zoomInDeltaY }));
      };

      wheel();
      await vi.advanceTimersByTimeAsync(120);
      view.rerender(host(law.refreshAuthoredZoom));
      wheel();
      await vi.advanceTimersByTimeAsync(120);
      expect((actions.at(-1)?.args?.camera as { readonly zoom: number }).zoom).toBeCloseTo(law.expectedRefreshZoom, 12);

      wheel();
      const retiredCount = actions.length;
      view.rerender(createElement("div", { key: law.key, "data-component": law.intermediateComponent }));
      await vi.advanceTimersByTimeAsync(120);
      expect(actions.length - retiredCount).toBe(law.retiredDeadlineActions);

      view.rerender(host(law.remountAuthoredZoom));
      const remountedCount = actions.length;
      wheel();
      await vi.advanceTimersByTimeAsync(120);
      expect(actions.length - remountedCount).toBe(law.remountedCameraActions);
      expect((actions.at(-1)?.args?.camera as { readonly zoom: number }).zoom).toBeCloseTo(law.expectedRemountZoom, 12);
    } finally {
      vi.useRealTimers();
    }
  });

  it.each(cameraGestures.panCases.filter(({ id }) => id !== "primary-active-pan"))("matches the mounted React $id button contract", async row => {
    vi.useFakeTimers();
    try {
      const { actions, canvas } = mountedHost(cameraGestures.surface, undefined, undefined, cameraGestures.wheel.initialCamera);
      fireEvent.pointerDown(canvas, { clientX: row.start.x, clientY: row.start.y, button: row.button });
      fireEvent.pointerMove(canvas, { clientX: row.end.x, clientY: row.end.y, button: row.button });
      fireEvent.pointerUp(canvas, { clientX: row.end.x, clientY: row.end.y, button: row.button });
      await settle();
      expect(actions.filter(({ action }) => action !== cameraGestures.wheel.expectedAction).map(({ action }) => action)).toEqual(row.expectedActions);
      await vi.advanceTimersByTimeAsync(120);
      const cameraActions = actions.filter(({ action }) => action === cameraGestures.wheel.expectedAction);
      expect(cameraActions).toHaveLength(row.expectedCameraAction ? 1 : 0);
      if (row.expectedCameraAction) expect(cameraActions[0]?.args?.camera).toEqual(row.expectedCamera);
    } finally {
      vi.useRealTimers();
    }
  });

  it("owns the declared MIME roster until pointer cancellation and keeps native drag separate", () => {
    const row = cataloguePointerTransfer.cases[0]!;
    const controller = catalogueTreeDragController(cataloguePointerTransfer.mime);
    const types = [...row.types];
    controller.pointerPaletteDrag?.begin(row.rawPayload, types);
    types.length = 0;
    expect(getActiveCataloguePointerDragData()).toEqual({ payload: row.rawPayload, types: row.types });
    controller.pointerPaletteDrag?.cancel();
    expect(getActiveCataloguePointerDragData()).toBeNull();
    controller.onDragStart?.({ items: [], section: { id: "catalogue" }, sourceItem: { id: "source", label: "source", dragData: { [cataloguePointerTransfer.mime]: row.rawPayload } } });
    expect(getActiveCataloguePointerDragData()).toBeNull();
    controller.onDragEnd?.({ items: [], section: { id: "catalogue" }, sourceItem: { id: "source", label: "source" } });
  });

  it.each(cataloguePointerTransfer.cases)("routes $id pointer catalogue ownership through the mounted canvas host", async row => {
    const { actions } = mountedHost(cataloguePointerTransfer.surface);
    const controller = catalogueTreeDragController(cataloguePointerTransfer.mime);
    controller.pointerPaletteDrag?.begin(row.rawPayload, row.types);
    for (const point of row.moves) {
      fireEvent.pointerMove(window, { clientX: point.x, clientY: point.y, pointerId: 7 });
    }
    if (row.terminal === "up") {
      fireEvent.pointerUp(window, { clientX: row.terminalPoint.x, clientY: row.terminalPoint.y, pointerId: 7 });
    } else {
      fireEvent.pointerCancel(window, { clientX: row.terminalPoint.x, clientY: row.terminalPoint.y, pointerId: 7 });
    }
    controller.pointerPaletteDrag?.cancel();
    await settle();
    expect(actions.map(({ action }) => action)).toEqual(row.expectedActions);
    const over = actions.find(action => action.action === "canvasDragOver");
    if (over) {
      const firstInside = row.moves.find(point => point.x >= cataloguePointerTransfer.surface.left && point.x <= cataloguePointerTransfer.surface.left + cataloguePointerTransfer.surface.width && point.y >= cataloguePointerTransfer.surface.top && point.y <= cataloguePointerTransfer.surface.top + cataloguePointerTransfer.surface.height);
      expect(over.args).toEqual({
        surfaceId: cataloguePointerTransfer.surface.id,
        x: (firstInside?.x ?? 0) - cataloguePointerTransfer.surface.left,
        y: (firstInside?.y ?? 0) - cataloguePointerTransfer.surface.top,
        width: cataloguePointerTransfer.surface.width,
        height: cataloguePointerTransfer.surface.height,
        types: row.types,
      });
    }
    const drop = actions.find(action => action.action === "canvasDrop");
    if (drop) {
      expect(drop.args).toEqual({
        surfaceId: cataloguePointerTransfer.surface.id,
        x: row.terminalPoint.x - cataloguePointerTransfer.surface.left,
        y: row.terminalPoint.y - cataloguePointerTransfer.surface.top,
        width: cataloguePointerTransfer.surface.width,
        height: cataloguePointerTransfer.surface.height,
        dragData: row.rawPayload,
      });
    }
  });

  it("preserves preview-before-terminal order when the real Catalogue threshold move reaches Canvas first", async () => {
    const row = cataloguePointerTransfer.cases[0]!;
    const point = row.moves[0]!;
    const { actions } = mountedHost(cataloguePointerTransfer.surface, undefined, { mime: cataloguePointerTransfer.mime, rawPayload: row.rawPayload, types: row.types });
    const handle = document.querySelector('#catalogue-pointer-source [data-slot="drag-handle"]') as HTMLElement | null;
    expect(handle).not.toBeNull();
    fireEvent.pointerDown(handle!, { button: 0, clientX: 8, clientY: 8, pointerId: 7 });
    fireEvent.pointerMove(window, { clientX: point.x, clientY: point.y, pointerId: 7 });
    fireEvent.pointerUp(window, { clientX: row.terminalPoint.x, clientY: row.terminalPoint.y, pointerId: 7 });
    await settle();
    expect(actions.map(({ action }) => action)).toEqual(row.expectedActions);
    expect(actions[0]?.args).toEqual({
      surfaceId: cataloguePointerTransfer.surface.id,
      x: point.x - cataloguePointerTransfer.surface.left,
      y: point.y - cataloguePointerTransfer.surface.top,
      width: cataloguePointerTransfer.surface.width,
      height: cataloguePointerTransfer.surface.height,
      types: row.types,
    });
    expect(actions[2]?.args).toEqual({
      surfaceId: cataloguePointerTransfer.surface.id,
      x: row.terminalPoint.x - cataloguePointerTransfer.surface.left,
      y: row.terminalPoint.y - cataloguePointerTransfer.surface.top,
      width: cataloguePointerTransfer.surface.width,
      height: cataloguePointerTransfer.surface.height,
      dragData: row.rawPayload,
    });
  });

  it.each(catalogueTerminal.cases)("serializes $terminal catalogue cleanup before the terminal raw drop", async row => {
    // 🧯️ Held on an object, not a `let`: a resolver assigned only inside the dispatch callback leaves
    // the local narrowed to its `null` initialiser at the call below, while the property keeps its
    // declared type — which is what this law actually holds (the cleanup finishes out of band).
    const leave: { finish?: () => void } = {};
    const starts: string[] = [];
    const { actions, host } = mountedHost(catalogueTerminal.surface, action => {
      starts.push(action.action);
      if (action.action !== "canvasDragLeave") return Promise.resolve();
      return new Promise<void>((resolve, reject) => {
        leave.finish = () => row.leaveOutcome === "reject" ? reject(new Error("fixture cleanup refusal")) : resolve();
      });
    });
    let dataReads = 0;
    const event = new MouseEvent("drop", { bubbles: true, cancelable: true, clientX: row.point.x, clientY: row.point.y });
    Object.defineProperty(event, "dataTransfer", { value: {
      types: row.types,
      getData: (mime: string) => {
        dataReads += 1;
        if (!row.types.includes(mime)) throw new Error("a foreign drop must not read catalogue data");
        return row.rawPayload;
      },
    } });
    host.dispatchEvent(event);
    const isCatalogue = row.types.includes(catalogueTerminal.mime);
    expect(starts).toEqual(isCatalogue ? ["canvasDragLeave"] : []);
    if (isCatalogue) {
      expect(dataReads).toBe(1);
      host.getBoundingClientRect = () => ({ ...bounds, left: 500, top: 500, x: 500, y: 500 } as DOMRect);
      leave.finish?.();
      await settle();
    } else {
      expect(dataReads).toBe(0);
    }
    expect(starts).toEqual(row.expectedActions);
    expect(actions.map(({ action }) => action)).toEqual(row.expectedActions);
    const drop = actions.find(action => action.action === "canvasDrop");
    if (drop) {
      expect(drop.args).toEqual({
        surfaceId: catalogueTerminal.surface.id,
        x: row.point.x,
        y: row.point.y,
        width: catalogueTerminal.surface.width,
        height: catalogueTerminal.surface.height,
        dragData: row.rawPayload,
      });
    }
  });

  it("publishes the installed canvas session's double-click contract", () => {
    const { actions, canvas } = mountedHost();
    fireEvent.doubleClick(canvas, { clientX: fixture.doubleClick.x, clientY: fixture.doubleClick.y });
    expect(actions).toEqual([
      {
        controllerId: fixture.surface.controllerId,
        action: "canvasDoubleClick",
        args: {
          surfaceId: fixture.surface.id,
          x: fixture.doubleClick.x,
          y: fixture.doubleClick.y,
          width: fixture.surface.width,
          height: fixture.surface.height,
        },
      },
    ]);
  });


it("publishes initial framing once and keeps navigation through same-window refreshes", async () => {
  vi.useFakeTimers();
  try {
    const law = framingFixture.cases.find(item => item.name === "point")!;
    const actions: HostAction[] = [];
    const onAction = (action: HostAction) => { actions.push(action); return Promise.resolve(); };
    const host = (revision: number) => createElement(Canvas2dHost, {
      node: { type: "componentScene",surfaceId: "framed-canvas",controllerId: "framed-owner",componentKind: "canvas-2d",
        canvas2d: { cameraX: 0,cameraY: 0,zoom: 8,layersJson: "[]",framing: { ...law.request,revision } } },
      onAction,
    } as never);
    const view = render(host(1));
    await vi.advanceTimersByTimeAsync(120);
    expect(actions.at(-1)?.args?.camera).toEqual({ x: law.expected![0],y: law.expected![1],zoom: law.expected![2] });
    const canvas = view.container.querySelector('[data-testid="canvas-input"]') as HTMLCanvasElement;
    canvas.getBoundingClientRect = () => bounds;
    canvas.dispatchEvent(new WheelEvent("wheel",{ bubbles: true,clientX: 50,clientY: 50,deltaY: -1 }));
    await vi.advanceTimersByTimeAsync(120);
    const navigated = actions.at(-1)?.args?.camera;
    const count = actions.length;
    view.rerender(host(1));
    await vi.advanceTimersByTimeAsync(120);
    expect(actions).toHaveLength(count);
    expect(actions.at(-1)?.args?.camera).toEqual(navigated);
    view.rerender(host(2));
    await vi.advanceTimersByTimeAsync(120);
    expect(actions).toHaveLength(count+1);
    expect(actions.at(-1)?.args?.camera).toEqual({ x: law.expected![0],y: law.expected![1],zoom: law.expected![2] });
    view.unmount();
  } finally {
    cleanup();
    vi.useRealTimers();
  }
});

});

import drawNudges from "../../../../../../../../../../✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🕹️nudge-selection/🧫️fixtures/🔣️.json" with {type:"json"};
import {keyboardEventMatchesChord} from "../../../🛠️ShellHelpers/🟦️.tsx";

it("matches Draw nudge bindings against independently dispatched DOM arrow events",()=>{
  const target=document.createElement("div");
  for(const row of drawNudges) {
    let matched=false;
    const receive=(event:KeyboardEvent)=>{matched=keyboardEventMatchesChord(event,row.keys);};
    target.addEventListener("keydown",receive);
    fireEvent.keyDown(target,{key:row.eventKey,shiftKey:row.keys.startsWith("shift+")});
    expect(matched,row.action).toBe(true);
    fireEvent.keyDown(target,{key:row.eventKey,shiftKey:!row.keys.startsWith("shift+")});
    expect(matched,row.action).toBe(false);
    target.removeEventListener("keydown",receive);
  }
});
