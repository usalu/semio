/** 🖱️ Board2dHost context menu and drop: the context menu handler, the catalogue drop preview and drop handlers, the host registration, the pointer tracking and the tool-run trace lanes. */
import { useCallback, useEffect, useMemo, type DragEvent, type MouseEvent } from "react";
import { type CanvasPickTarget, pickMostSpecificCanvasTarget, CATALOGUE_DRAG_MIME, getActiveCatalogueDragPayload, getActiveCataloguePointerDragData } from "@semio-tech/ui-react";
import { suggestionSubmenuTarget } from "../../🎣️suggestion-submenu/🟦️.ts";
import { parseSelectionIds } from "../../🖋️InkCanvasHost/🧾️ink-model/🟦️.ts";
import { useToolRunTraceCursorEcho } from "../../🌐️World3dHost/⏯️tool-run-trace/🟦️.tsx";
import { board2dToolRunTracePathForShape, board2dToolRunTraceShapes } from "../⏯️tool-run-trace/🟦️.tsx";
import { openSurfaceContextMenu } from "../../🗣️Interpreter/🖱️surface-menu/🟦️.ts";
import { publishLocalActiveToolV1 } from "../../👕️canvas-presence/🟦️.ts";
import { board2dVitals, type BoardDropPayload, parseBoardCamera, parseBoardCatalogueDragPayload } from "../🔬️board-parsing/🟦️.ts";
import { boardCatalogueDropHostContainsPoint, boardCatalogueDropPointOverRect, type BoardCatalogueDropRect, boardDropPreviewJson, boardScreenToWorld, registerBoardCatalogueDropHost } from "../📍️drop-and-camera/🟦️.ts";
import { pushBoardDropPreview } from "../🔄️session-sync/🟦️.ts";
import type { BoardCore4 } from "../👆️board-pointer-and-keyboard/🟦️.ts";

// #region Board2dHost
/** 🖱️ Builds the context menu and catalogue drop handlers and the tool-run trace lanes. */
export function useBoardContextMenuAndDrop(core: BoardCore4) {
  const { containerRef, dispatch, mapContextMenu, node, peerScope, readContainerSize, requestContextMenu, scene, sceneRef, sessionRef, setContextMenu, shellContextMenuFallback, windowInstanceId , presenceRuntimeKey } = core;

  //#region ContextMenu
  const onContextMenu = useCallback(
    (event: MouseEvent<HTMLDivElement>): void => {
      if (!scene?.interactive || !requestContextMenu) return;
      const session = sessionRef.current;
      if (!session?.pickTargetsAtScreenJson) return;
      const pickTargetsAtScreenJson = session.pickTargetsAtScreenJson.bind(session);
      event.preventDefault();
      event.stopPropagation();
      void (async () => {
        const rect = event.currentTarget.getBoundingClientRect();
        const sx = event.clientX - rect.left;
        const sy = event.clientY - rect.top;
        let targets: CanvasPickTarget[] = [];
        try {
          targets = JSON.parse(pickTargetsAtScreenJson(sx, sy)) as CanvasPickTarget[];
        } catch {
          targets = [];
        }
        const best = pickMostSpecificCanvasTarget(targets);
        let selectionIds = parseSelectionIds(scene.selectionJson);
        if (best && !selectionIds.includes(best.id)) {
          selectionIds = [best.id];
          if (session.setSelectionIdsJsonSilent) session.setSelectionIdsJsonSilent(JSON.stringify(selectionIds));
          try {
            session.renderFrame();
          } catch {}
          dispatch("applyBoardEvents", { eventsJson: JSON.stringify([{ name: "select", payload: { ids: selectionIds, exitHighlightIds: [] } }]) });
        }
        const hits = targets.map((target) => ({ domain: target.domain, id: target.id, label: target.label }));
        const suggest = { target: null as string | null };
        const menu = await openSurfaceContextMenu(
          requestContextMenu,
          {
            menu: { id: "board2d", args: null },
            surface: {
              surfaceId: node.surfaceId,
              kind: "board2d",
              hits,
              selection: selectionIds.length > 0 ? [{ domain: "node", ids: selectionIds }] : [],
            },
            point: { x: event.clientX, y: event.clientY },
          },
          (specs) => {
            suggest.target = suggestionSubmenuTarget(specs);
            return mapContextMenu(specs);
          },
          shellContextMenuFallback,
        );
        setContextMenu({ x: event.clientX, y: event.clientY, ...menu, suggestTarget: suggest.target });
      })();
    },
    [dispatch, mapContextMenu, node.surfaceId, requestContextMenu, scene?.interactive, scene?.selectionJson, shellContextMenuFallback],
  );
  //#endregion ContextMenu

  //#region DropPreviewHandlers
  const clearCatalogueDropPreview = useCallback(() => {
    pushBoardDropPreview(peerScope, node.controllerId, null);
  }, [peerScope, node.controllerId]);

  const placeCatalogueDrop = useCallback(
    (payload: BoardDropPayload, clientX: number, clientY: number, rect: BoardCatalogueDropRect) => {
      const session = sessionRef.current;
      if (!session) return;
      const world = boardScreenToWorld(session.cameraJson(), readContainerSize(), { x: clientX - rect.left, y: clientY - rect.top });
      dispatch("addNode", {
        kind: payload.kindId,
        x: world?.x,
        y: world?.y,
        shape: payload.shape,
        radius: payload.radius,
        width: payload.width,
        height: payload.height,
        iconKind: payload.iconKind,
      });
    },
    [dispatch, readContainerSize],
  );

  const previewCatalogueDropAt = useCallback(
    (clientX: number, clientY: number): boolean => {
      const host = containerRef.current;
      const session = sessionRef.current;
      if (!sceneRef.current?.interactive || !host || !session?.setDropPreviewJson) return false;
      const rect = host.getBoundingClientRect();
      if (!boardCatalogueDropPointOverRect(clientX, clientY, rect)) {
        if (!boardCatalogueDropHostContainsPoint(node.controllerId, clientX, clientY)) clearCatalogueDropPreview();
        return false;
      }
      const payload = parseBoardCatalogueDragPayload(getActiveCatalogueDragPayload() ?? getActiveCataloguePointerDragData()?.payload);
      if (!payload) return false;
      const world = boardScreenToWorld(session.cameraJson(), readContainerSize(), { x: clientX - rect.left, y: clientY - rect.top });
      if (!world) return false;
      pushBoardDropPreview(peerScope, node.controllerId, boardDropPreviewJson(payload, world.x, world.y));
      return true;
    },
    [clearCatalogueDropPreview, node.controllerId, peerScope, readContainerSize],
  );

  const onDragOver = useCallback(
    (event: DragEvent<HTMLDivElement>): void => {
      if (!scene?.interactive) return;
      if (!event.dataTransfer.types.includes(CATALOGUE_DRAG_MIME) && !getActiveCatalogueDragPayload()) return;
      if (!sessionRef.current?.setDropPreviewJson) return;
      if (!parseBoardCatalogueDragPayload(getActiveCatalogueDragPayload()) && !event.dataTransfer.types.includes(CATALOGUE_DRAG_MIME)) return;
      event.preventDefault();
      previewCatalogueDropAt(event.clientX, event.clientY);
    },
    [previewCatalogueDropAt, scene?.interactive],
  );

  const onDragLeave = useCallback((): void => {}, []);

  const onDrop = useCallback(
    (event: DragEvent<HTMLDivElement>): void => {
      if (!scene?.interactive) return;
      const encoded = event.dataTransfer.getData(CATALOGUE_DRAG_MIME) || getActiveCatalogueDragPayload();
      const payload = parseBoardCatalogueDragPayload(encoded);
      const session = sessionRef.current;
      const rect = event.currentTarget.getBoundingClientRect();
      clearCatalogueDropPreview();
      if (!payload || !session) return;
      event.preventDefault();
      placeCatalogueDrop(payload, event.clientX, event.clientY, rect);
    },
    [clearCatalogueDropPreview, placeCatalogueDrop, scene?.interactive],
  );

  useEffect(() => {
    const hostId = windowInstanceId ?? node.surfaceId;
    return registerBoardCatalogueDropHost(node.controllerId, hostId, (clientX, clientY) => {
      const host = containerRef.current;
      if (!host) return false;
      return boardCatalogueDropPointOverRect(clientX, clientY, host.getBoundingClientRect());
    });
  }, [node.controllerId, node.surfaceId, windowInstanceId]);

  useEffect(() => {
    const onPointerMove = (event: PointerEvent) => {
      if (!getActiveCataloguePointerDragData() && !getActiveCatalogueDragPayload()) return;
      previewCatalogueDropAt(event.clientX, event.clientY);
    };
    const onPointerUp = (event: PointerEvent) => {
      const data = getActiveCataloguePointerDragData();
      if (!data) return;
      const host = containerRef.current;
      const payload = parseBoardCatalogueDragPayload(data.payload);
      if (!host || !payload || !sceneRef.current?.interactive) return;
      const rect = host.getBoundingClientRect();
      if (!boardCatalogueDropPointOverRect(event.clientX, event.clientY, rect)) {
        if (!boardCatalogueDropHostContainsPoint(node.controllerId, event.clientX, event.clientY)) clearCatalogueDropPreview();
        return;
      }
      clearCatalogueDropPreview();
      placeCatalogueDrop(payload, event.clientX, event.clientY, rect);
    };
    const onDragEnd = (): void => {
      queueMicrotask(() => {
        if (!getActiveCatalogueDragPayload()) clearCatalogueDropPreview();
      });
    };
    window.addEventListener("pointermove", onPointerMove);
    window.addEventListener("pointerup", onPointerUp, true);
    window.addEventListener("pointercancel", clearCatalogueDropPreview, true);
    window.addEventListener("dragend", onDragEnd);
    return () => {
      window.removeEventListener("pointermove", onPointerMove);
      window.removeEventListener("pointerup", onPointerUp, true);
      window.removeEventListener("pointercancel", clearCatalogueDropPreview, true);
      window.removeEventListener("dragend", onDragEnd);
    };
  }, [clearCatalogueDropPreview, node.controllerId, placeCatalogueDrop, previewCatalogueDropAt]);
  //#endregion DropPreviewHandlers

  useEffect(() => {
    if (presenceRuntimeKey) publishLocalActiveToolV1(presenceRuntimeKey, scene?.activeUtility ?? null);
  }, [scene?.activeUtility]);

  const toolRunTraceCamera = useMemo(() => parseBoardCamera(scene?.cameraJson ?? "") ?? { x: 0, y: 0, zoom: 1 }, [scene?.cameraJson]);
  const boardVitals = useMemo(() => board2dVitals(scene?.snapshotJson ?? ""), [scene?.snapshotJson]);
  const toolRunTracePathForShape = useMemo(() => board2dToolRunTracePathForShape(board2dToolRunTraceShapes(scene?.glyphCatalogsJson ?? "")), [scene?.glyphCatalogsJson]);
  const onToolRunTraceCursor = useToolRunTraceCursorEcho(windowInstanceId);
  return { boardVitals, onContextMenu, onDragLeave, onDragOver, onDrop, onToolRunTraceCursor, toolRunTraceCamera, toolRunTracePathForShape };
}

export type BoardCore5 = BoardCore4 & ReturnType<typeof useBoardContextMenuAndDrop>;
// #endregion Board2dHost
