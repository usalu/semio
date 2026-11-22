/** 🟠️ The catalogue drag enter phase of useWorldPointerAndDrop. */
import { useCallback, useEffect, useRef, type DragEvent } from "../../../⚛️react-port/🟦️.ts";
import { CATALOGUE_DRAG_MIME, getActiveCatalogueDragPayload, getActiveCataloguePointerDragData } from "@semio-tech/ui-react";
import { clientPointOverHost } from "../../🔗️instance-selection/🟦️.ts";
import { parsePuzzle3dCatalogueDragPayload, registerWorldCatalogueDropHost, worldCatalogueDropHostContainsPoint } from "../../🗂️catalogue-drop/🟦️.tsx";
import type { useActiveUtilityState } from "../🔶️active-utility/🟦️.ts";
import type { useHandlePointerUpState } from "../🔸️handle-pointer-up/🟦️.ts";

/** 🟡 Builds onCatalogueDragEnter, onCatalogueDragLeave, onCatalogueDragOver, onCatalogueDrop of useWorldPointerAndDrop. */
export function useCatalogueDragEnterState({ scene, catalogueDragEncodedRef, catalogueDragDepthRef, updateCatalogueDropPreviewAt, commitCatalogueDropAt, clearCatalogueDrop, windowInstanceId, node, hostRef }: Pick<ReturnType<typeof useActiveUtilityState>, "scene" | "catalogueDragEncodedRef" | "catalogueDragDepthRef" | "windowInstanceId" | "node" | "hostRef"> & Pick<ReturnType<typeof useHandlePointerUpState>, "updateCatalogueDropPreviewAt" | "commitCatalogueDropAt" | "clearCatalogueDrop">) {
  const pointerDrag = useRef<{ readonly pointerId: number; readonly payload: string } | null>(null);
  const onCatalogueDragEnter = useCallback(
    (event: DragEvent<HTMLDivElement>) => {
      if (!scene) return;
      if (!event.dataTransfer.types.includes(CATALOGUE_DRAG_MIME) && !getActiveCatalogueDragPayload()) return;
      event.preventDefault();
      const encoded = getActiveCatalogueDragPayload();
      if (encoded) catalogueDragEncodedRef.current = encoded;
      catalogueDragDepthRef.current += 1;
      updateCatalogueDropPreviewAt(event.clientX, event.clientY);
    },
    [scene, updateCatalogueDropPreviewAt],
  );

  const onCatalogueDragLeave = useCallback(
    (event: DragEvent<HTMLDivElement>) => {
      if (!scene) return;
      catalogueDragDepthRef.current = Math.max(0, catalogueDragDepthRef.current - 1);
      if (event.relatedTarget instanceof Node && event.currentTarget.contains(event.relatedTarget)) return;
      if (!worldCatalogueDropHostContainsPoint(node.controllerId, event.clientX, event.clientY)) clearCatalogueDrop();
    },
    [clearCatalogueDrop, node.controllerId, scene],
  );

  const onCatalogueDragOver = useCallback(
    (event: DragEvent<HTMLDivElement>) => {
      if (!scene) return;
      if (!event.dataTransfer.types.includes(CATALOGUE_DRAG_MIME) && !getActiveCatalogueDragPayload()) return;
      const encoded = getActiveCatalogueDragPayload();
      if (!parsePuzzle3dCatalogueDragPayload(encoded) && !event.dataTransfer.types.includes(CATALOGUE_DRAG_MIME)) return;
      event.preventDefault();
      event.dataTransfer.dropEffect = "copy";
      if (encoded) catalogueDragEncodedRef.current = encoded;
      updateCatalogueDropPreviewAt(event.clientX, event.clientY);
    },
    [scene, updateCatalogueDropPreviewAt],
  );

  const onCatalogueDrop = useCallback(
    (event: DragEvent<HTMLDivElement>) => {
      if (!scene) return;
      event.preventDefault();
      const encoded = event.dataTransfer.getData(CATALOGUE_DRAG_MIME) || getActiveCatalogueDragPayload() || catalogueDragEncodedRef.current;
      commitCatalogueDropAt(event.clientX, event.clientY, encoded);
      clearCatalogueDrop();
    },
    [clearCatalogueDrop, commitCatalogueDropAt, scene],
  );

  useEffect(() => {
    const hostId = windowInstanceId ?? node.surfaceId;
    return registerWorldCatalogueDropHost(node.controllerId, hostId, (clientX, clientY) => {
      const host = hostRef.current;
      if (!host) return false;
      return clientPointOverHost(clientX, clientY, host.getBoundingClientRect());
    });
  }, [node.controllerId, node.surfaceId, windowInstanceId]);

  useEffect(() => {
    const onWindowDragOver = (event: globalThis.DragEvent) => {
      if (!getActiveCatalogueDragPayload() && !event.dataTransfer?.types.includes(CATALOGUE_DRAG_MIME)) return;
      if (!parsePuzzle3dCatalogueDragPayload(getActiveCatalogueDragPayload()) && !event.dataTransfer?.types.includes(CATALOGUE_DRAG_MIME)) return;
      event.preventDefault();
      const encoded = getActiveCatalogueDragPayload();
      if (encoded) catalogueDragEncodedRef.current = encoded;
      updateCatalogueDropPreviewAt(event.clientX, event.clientY);
    };

    const onPointerMove = (event: PointerEvent) => {
      const data = getActiveCataloguePointerDragData();
      if (!data) {
        if (!pointerDrag.current) return;
        pointerDrag.current = null;
        if (!getActiveCatalogueDragPayload()) clearCatalogueDrop();
        return;
      }
      const tracked = pointerDrag.current;
      if ((tracked && tracked.pointerId !== event.pointerId) || !parsePuzzle3dCatalogueDragPayload(data.payload)) return;
      pointerDrag.current = { pointerId: event.pointerId, payload: data.payload };
      updateCatalogueDropPreviewAt(event.clientX, event.clientY);
    };

    const onPointerUp = (event: PointerEvent) => {
      const tracked = pointerDrag.current;
      if (event.button !== 0 || (tracked && tracked.pointerId !== event.pointerId)) return;
      const encoded = getActiveCataloguePointerDragData()?.payload ?? tracked?.payload;
      pointerDrag.current = null;
      if (!parsePuzzle3dCatalogueDragPayload(encoded)) return;
      const host = hostRef.current;
      if (!host) return;
      const rect = host.getBoundingClientRect();
      if (clientPointOverHost(event.clientX, event.clientY, rect)) {
        commitCatalogueDropAt(event.clientX, event.clientY, encoded);
        clearCatalogueDrop();
        return;
      }
      if (!worldCatalogueDropHostContainsPoint(node.controllerId, event.clientX, event.clientY)) clearCatalogueDrop();
    };

    const onPointerCancel = (event: PointerEvent) => {
      if (pointerDrag.current?.pointerId !== event.pointerId) return;
      pointerDrag.current = null;
      clearCatalogueDrop();
    };

    const onDragEnd = () => {
      queueMicrotask(() => {
        if (!getActiveCatalogueDragPayload()) clearCatalogueDrop();
      });
    };

    window.addEventListener("dragover", onWindowDragOver, true);
    window.addEventListener("pointermove", onPointerMove);
    window.addEventListener("pointerup", onPointerUp, true);
    window.addEventListener("pointercancel", onPointerCancel, true);
    window.addEventListener("dragend", onDragEnd);
    return () => {
      window.removeEventListener("dragover", onWindowDragOver, true);
      window.removeEventListener("pointermove", onPointerMove);
      window.removeEventListener("pointerup", onPointerUp, true);
      window.removeEventListener("pointercancel", onPointerCancel, true);
      window.removeEventListener("dragend", onDragEnd);
    };
  }, [clearCatalogueDrop, commitCatalogueDropAt, node.controllerId, updateCatalogueDropPreviewAt]);
  return { onCatalogueDragEnter, onCatalogueDragLeave, onCatalogueDragOver, onCatalogueDrop };
}
