import { render } from "@semio-tech/ui-react/test";
import { createElement, type MouseEvent as ReactMouseEvent, type PointerEvent as ReactPointerEvent, type WheelEvent as ReactWheelEvent } from "react";
import { vi } from "vitest";
import { Tree, catalogueTreeDragController } from "@semio-tech/ui-react";
import fixture from "../../🧫️fixtures/🖱️input-contract/🔣️.json";
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
        onPointerMove: (event: ReactPointerEvent<HTMLCanvasElement>) => {
          const [x, y] = local(event);
          session.pointerMove(x, y, {shift:event.shiftKey,ctrl:event.ctrlKey,meta:event.metaKey,alt:event.altKey});
        },
        onPointerUp: (event: ReactPointerEvent<HTMLCanvasElement>) => {
          const [x, y] = local(event);
          session.pointerUp(x, y, { shift: event.shiftKey, ctrl: event.ctrlKey, meta: event.metaKey, alt: event.altKey });
        },
        onWheel: (event: ReactWheelEvent<HTMLCanvasElement>) => {
          const [x, y] = local(event);
          session.wheel(x, y, event.deltaY);
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

import { Canvas2dHost, wheelCameraAtScreen } from "../../🟦️.tsx";

export const bounds = { x: 0, y: 0, left: 0, top: 0, right: fixture.surface.width, bottom: fixture.surface.height, width: fixture.surface.width, height: fixture.surface.height, toJSON: () => ({}) } as DOMRect;

export type HostAction = { readonly controllerId: string; readonly action: string; readonly args?: Record<string, unknown> };

/** 🖼️ The surface geometry every input fixture in this suite declares. `left`/`top` are optional
 * because only the pointer-transfer fixture places its surface away from the viewport origin; the
 * others start at `0, 0` and say so by omission. */
type MountedSurface = {
  readonly id: string;
  readonly controllerId: string;
  readonly width: number;
  readonly height: number;
  readonly left?: number;
  readonly top?: number;
};

export function mountedHost(
  surface: MountedSurface = fixture.surface,
  dispatchAction?: (action: HostAction) => void | Promise<unknown>,
  catalogueSource?: { readonly mime: string; readonly rawPayload: string; readonly types: readonly string[] },
  camera = { x: 0, y: 0, zoom: 1 },
) {
  const actions: HostAction[] = [];
  const left = surface.left ?? 0;
  const top = surface.top ?? 0;
  const surfaceBounds = { ...bounds, x: left, y: top, left, top, right: left + surface.width, bottom: top + surface.height, width: surface.width, height: surface.height } as DOMRect;
  const canvasHost = createElement(Canvas2dHost, {
      node: {
        type: "componentScene",
        surfaceId: surface.id,
        controllerId: surface.controllerId,
        componentKind: "canvas-2d",
        canvas2d: { cameraX: camera.x, cameraY: camera.y, zoom: camera.zoom, layersJson: "[]" },
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
          createElement(Tree, {
            sections: [{ id: "catalogue", label: "Catalogue", items: [{ id: "catalogue-pointer-source", label: "Source", draggable: true, dragData: Object.fromEntries(catalogueSource.types.map(type => [type, type === catalogueSource.mime ? catalogueSource.rawPayload : ""])) }] }],
            dragAndDropController: catalogueTreeDragController(catalogueSource.mime),
          }),
          canvasHost,
        )
      : canvasHost,
  );
  const host = view.container.querySelector(".semio-canvas-2d-host") as HTMLDivElement;
  const canvas = view.container.querySelector('[data-testid="canvas-input"]') as HTMLCanvasElement;
  host.getBoundingClientRect = () => surfaceBounds;
  canvas.getBoundingClientRect = () => surfaceBounds;
  return { actions, host, canvas, unmount: view.unmount };
}

export async function settle(): Promise<void> {
  await Promise.resolve();
  await Promise.resolve();
  await Promise.resolve();
}


export function canvasInputSessions(): any[] { return seam.sessions; }
