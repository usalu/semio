/** 🟠️ The context menu phase of Canvas2dHost. */
import { useCallback, useEffect, type MouseEvent } from "react";
import { openSurfaceContextMenu } from "../../🗣️Interpreter/🖱️surface-menu/🟦️.ts";
import type { useSceneState } from "../🔶️scene/🟦️.tsx";

/** 🟡 Builds onContextMenu of Canvas2dHost. */
export function useContextMenu({ requestContextMenu, node, windowInstanceId, mapContextMenu, shellContextMenuFallback, setContextMenu, containerRef, setViewportSize }: { readonly requestContextMenu: ((request: import("../../../../../../../../../\uD83E\uDDF0\uFE0Fframework/\uD83D\uDD28\uFE0Fmodules/\uD83D\uDEC2\uFE0Fmanifest/\uD83D\uDCCB\uFE0Fcontext-menu/\uD83D\uDFE6\uFE0F").UiContextMenuRequest) => Promise<readonly import("../../../../../../../../../\uD83E\uDDF0\uFE0Fframework/\uD83D\uDCE6\uFE0Fpackages/\uD83D\uDFE6\uFE0Ftypescript/\uD83D\uDFE6\uFE0F").ContextMenuItemSpec[]>) | undefined; readonly node: import("../../../../../../../../../\uD83E\uDDF0\uFE0Fframework/\uD83D\uDCE6\uFE0Fpackages/\uD83D\uDFE6\uFE0Ftypescript/\uD83D\uDFE6\uFE0F").UiComponentSceneNode } & Pick<ReturnType<typeof useSceneState>, "windowInstanceId" | "mapContextMenu" | "shellContextMenuFallback" | "setContextMenu" | "containerRef" | "setViewportSize">) {
  //#region ContextMenu
  /** 🖱️ No layer pick/selection is tracked at this level (`Canvas2dScene` carries only camera + `layersJson`) — `hits`/`selection` stay empty per surface convention until layer picking lands here. */
  const onContextMenu = useCallback(
    (event: MouseEvent<HTMLDivElement>): void => {
      if (!requestContextMenu) return;
      event.preventDefault();
      event.stopPropagation();
      void (async () => {
        const menu = await openSurfaceContextMenu(
          requestContextMenu,
          {
            menu: { id: "canvas2d", args: null },
            surface: { surfaceId: node.surfaceId, kind: "canvas2d", hits: [], selection: [] },
            windowInstanceId: windowInstanceId ?? undefined,
            point: { x: event.clientX, y: event.clientY },
          },
          mapContextMenu,
          shellContextMenuFallback,
        );
        setContextMenu({ x: event.clientX, y: event.clientY, ...menu });
      })();
    },
    [mapContextMenu, node.surfaceId, requestContextMenu, shellContextMenuFallback, windowInstanceId],
  );
  //#endregion ContextMenu

  useEffect(() => {
    const element = containerRef.current;
    if (!element) return;
    const observer = new ResizeObserver((entries) => {
      const entry = entries[0];
      if (!entry) return;
      setViewportSize({ width: entry.contentRect.width, height: entry.contentRect.height });
    });
    observer.observe(element);
    return () => observer.disconnect();
  }, []);
  return { onContextMenu };
}
