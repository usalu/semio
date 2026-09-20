/** @emoji 🖱️ Mounted Canvas2d input oracle: the real host owns catalogue DOM events while the
 * installed infinite-canvas session seam owns pointer/cancel/double-click events. Both must publish
 * the shared language-neutral fixture without changing MIME text, coordinates, or modifiers. */
import { cleanup, fireEvent, render } from "@semio-tech/ui-react/test";
import { createElement, type MouseEvent as ReactMouseEvent, type PointerEvent as ReactPointerEvent } from "react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import fixture from "../../🧫️fixtures/🖱️input-contract/🔣️.json" with { type: "json" };

import Ajv2020 from "ajv/dist/2020";
import layoutCatalogue from "../../../../../../../../../../✏️s/🔌️plugins/📏️layout/🧫️fixtures/🛍️canvas-catalogue/🔣️.json" with { type: "json" };
import layoutCatalogueSchema from "../../../../../../../../../../✏️s/🔌️plugins/📏️layout/🧬️schema/🛍️canvas-catalogue/🔣️.json" with { type: "json" };

import catalogueTerminal from "../../../../../../../../../🔨️modules/🖱️ui/🧪️fixtures/🛒️canvas-catalogue-terminal/🔣️.json" with { type: "json" };
import catalogueTerminalSchema from "../../../../../../../../../🔨️modules/🖱️ui/🧬️schema/🛒️canvas-catalogue-terminal/🔣️.json" with { type: "json" };
import cataloguePointerTransfer from "../../../../../../../../../🔨️modules/🖱️ui/🧪️fixtures/🛒️canvas-catalogue-pointer-transfer/🔣️.json" with { type: "json" };
import cataloguePointerTransferSchema from "../../../../../../../../../🔨️modules/🖱️ui/🧬️schema/🛒️canvas-catalogue-pointer-transfer/🔣️.json" with { type: "json" };
import { Catalogue, catalogueTreeDragController } from "@semio-tech/ui-react";

const seam = vi.hoisted(() => ({ sessions: [] as any[] }));

vi.mock("@semio-tech/infinite-canvas-react-renderer", async (importOriginal) => {
  const actual = (await importOriginal()) as Record<string, unknown>;
  const React = await import("react");
  return {
    ...actual,
    GraphWasmCanvas: ({ className, sessionFactory }: { readonly className?: string; readonly sessionFactory: () => any }) => {
      const session = React.useMemo(sessionFactory, [sessionFactory]);
      React.useEffect(() => {
        session.setSize(fixture.surface.width, fixture.surface.height, 1);
        seam.sessions.push(session);
        return () => session.dispose?.();
      }, [session]);
      const local = (event: { readonly clientX: number; readonly clientY: number; readonly currentTarget: Element }) => {
        const rect = event.currentTarget.getBoundingClientRect();
        return [event.clientX - rect.left, event.clientY - rect.top] as const;
      };
      return createElement("canvas", {
        className,
        "data-testid": "canvas-input",
        onPointerDown: (event: ReactPointerEvent<HTMLCanvasElement>) => {
          const [x, y] = local(event);
          session.pointerDown(x, y, event.button, false, { shift: event.shiftKey, ctrl: event.ctrlKey, meta: event.metaKey, alt: event.altKey });
        },
        onPointerUp: (event: ReactPointerEvent<HTMLCanvasElement>) => {
          const [x, y] = local(event);
          session.pointerUp(x, y, { shift: event.shiftKey, ctrl: event.ctrlKey, meta: event.metaKey, alt: event.altKey });
        },
        onPointerCancel: () => session.pointerCancel(),
        onDoubleClick: (event: ReactMouseEvent<HTMLCanvasElement>) => {
          const [x, y] = local(event);
          session.doubleClick(x, y);
        },
      });
    },
  };
});

import { Canvas2dHost } from "../../🟦️.tsx";

const bounds = { x: 0, y: 0, left: 0, top: 0, right: fixture.surface.width, bottom: fixture.surface.height, width: fixture.surface.width, height: fixture.surface.height, toJSON: () => ({}) } as DOMRect;

type HostAction = { readonly controllerId: string; readonly action: string; readonly args?: Record<string, unknown> };

function mountedHost(
  surface = fixture.surface,
  dispatchAction?: (action: HostAction) => void | Promise<unknown>,
  catalogueSource?: { readonly mime: string; readonly rawPayload: string },
) {
  const actions: HostAction[] = [];
  const left = "left" in surface ? surface.left : 0;
  const top = "top" in surface ? surface.top : 0;
  const surfaceBounds = { ...bounds, x: left, y: top, left, top, right: left + surface.width, bottom: top + surface.height, width: surface.width, height: surface.height } as DOMRect;
  const canvasHost = createElement(Canvas2dHost, {
      node: {
        type: "componentScene",
        surfaceId: surface.id,
        controllerId: surface.controllerId,
        componentKind: "canvas-2d",
        canvas2d: { cameraX: 0, cameraY: 0, zoom: 1, layersJson: "[]" },
      },
      onAction: (action: HostAction) => {
        actions.push(action);
        return dispatchAction?.(action) ?? Promise.resolve();
      },
    } as never);
  const view = render(
    catalogueSource
      ? createElement(
          "div",
          null,
          createElement(Catalogue, {
            title: "Catalogue",
            mime: catalogueSource.mime,
            items: [{ id: "catalogue-pointer-source", label: "Source", payload: JSON.parse(catalogueSource.rawPayload) as Record<string, unknown> }],
          }),
          canvasHost,
        )
      : canvasHost,
  );
  const host = view.container.querySelector(".semio-canvas-2d-host") as HTMLDivElement;
  const canvas = view.container.querySelector('[data-testid="canvas-input"]') as HTMLCanvasElement;
  host.getBoundingClientRect = () => surfaceBounds;
  canvas.getBoundingClientRect = () => surfaceBounds;
  return { actions, host, canvas };
}

async function settle(): Promise<void> {
  await Promise.resolve();
  await Promise.resolve();
  await Promise.resolve();
}

describe("🖱️ Canvas2d mounted input contract", () => {
  beforeEach(() => {
    seam.sessions.length = 0;
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
      cancelled: true,
    });
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

  it.each(cataloguePointerTransfer.cases)("routes $id pointer catalogue ownership through the mounted canvas host", async row => {
    const { actions } = mountedHost(cataloguePointerTransfer.surface);
    const controller = catalogueTreeDragController(cataloguePointerTransfer.mime);
    controller.pointerPaletteDrag?.begin(row.rawPayload);
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
        types: [cataloguePointerTransfer.mime],
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
    const { actions } = mountedHost(cataloguePointerTransfer.surface, undefined, { mime: cataloguePointerTransfer.mime, rawPayload: row.rawPayload });
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
      types: [cataloguePointerTransfer.mime],
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
    let finishLeave: (() => void) | null = null;
    const starts: string[] = [];
    const { actions, host } = mountedHost(catalogueTerminal.surface, action => {
      starts.push(action.action);
      if (action.action !== "canvasDragLeave") return Promise.resolve();
      return new Promise<void>((resolve, reject) => {
        finishLeave = () => row.leaveOutcome === "reject" ? reject(new Error("fixture cleanup refusal")) : resolve();
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
      finishLeave?.();
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

  it("validates the Layout catalogue renderer envelope with the independent JSON Schema oracle", () => {
    expect(new Ajv2020({ strict: true }).compile(layoutCatalogueSchema)(layoutCatalogue)).toBe(true);
  });

  it.each(layoutCatalogue.cases.filter(row => row.kind !== null))("forwards the actual Layout catalogue $id envelope", async row => {
    const { actions, host } = mountedHost({ ...fixture.surface, id: row.args.surfaceId, controllerId: "layout-play" });
    const args = row.args as typeof row.args & { types?: string[]; dragData?: string };
    const dropping = row.action === "canvasDrop";
    const event = new MouseEvent(dropping ? "drop" : "dragover", { bubbles: true, cancelable: true, clientX: args.x, clientY: args.y });
    Object.defineProperty(event, "dataTransfer", { value: {
      types: args.types ?? [layoutCatalogue.mime, `${layoutCatalogue.kindMimePrefix}${row.kind}`],
      getData: (mime: string) => {
        if (!dropping) throw new Error("drag-over cannot read protected source data");
        return mime === layoutCatalogue.mime ? args.dragData! : "";
      },
    } });
    host.dispatchEvent(event);
    await settle();
    expect(actions).toEqual(dropping
      ? [
          { controllerId: "layout-play", action: "canvasDragLeave", args: { surfaceId: row.args.surfaceId } },
          { controllerId: "layout-play", action: row.action, args: row.args },
        ]
      : [{ controllerId: "layout-play", action: row.action, args: row.args }]);
    if (dropping) expect(JSON.parse(args.dragData!)).toEqual({ kind: row.kind });
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
});
