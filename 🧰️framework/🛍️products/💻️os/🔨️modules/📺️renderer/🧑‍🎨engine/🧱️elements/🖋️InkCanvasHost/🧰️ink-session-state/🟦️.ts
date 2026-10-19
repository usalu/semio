/** 🧰️ InkCanvasHost session state: the scene and window instance, the refs, the draft document and the drag, marquee, text edit, table edit and context menu state, the labels, the selection, hover and interaction lanes and the local presence publishing. */
import { useCallback, useContext, useEffect, useMemo, useRef, useState } from "react";
import { GestureRecognizer, type ComponentSceneHostProps, windowElementId } from "@semio-tech/framework";
import { registerIntroductionSurfaceResolver, useLabel, type IntroductionResolvedGeometry, type SelectionMarqueePoint } from "@semio-tech/ui-react";
import { type SurfaceContextMenuResult } from "../../🗣️Interpreter/🖱️surface-menu/🟦️.ts";
import { WindowInstanceIdContext } from "../../🗣️Interpreter/🌉️scene-host-contract/🟦️.ts";
import { useLocalPresenceActorIdV1 , usePresenceRuntimeKeyV1} from "../../👕️canvas-presence/🟦️.tsx";
import { PRESENCE_VIEW_PUBLISH_MIN_INTERVAL_MS, clearLocalPresenceWindowViewV1, publishLocalPresenceWindowViewV1 } from "../../👕️canvas-presence/🟦️.ts";
import { type InkCamera, type InkDocument, type InkPendingLive, parseInkScene, parseSelectionIds, type Vec2 } from "../🧾️ink-model/🟦️.ts";
import { findInkItem, flattenInkItems, inkItemBounds, inkPresenceCanvasV1, worldToScreen } from "../📐️ink-geometry/🟦️.ts";
import { type InkDragState, type InkTableEditState, type InkTextEditState } from "../🧲️ink-host-state/🟦️.ts";

// #region InkCanvasHost
/** 🧰️ Holds the refs, state, parsed lanes and presence publishing of the ink window. */
export function useInkSessionState({ node, onAction, requestContextMenu }: ComponentSceneHostProps) {
  const scene = node.inkCanvas;
  const windowInstanceId = useContext(WindowInstanceIdContext);
  const rootRef = useRef<HTMLDivElement | null>(null);
  const gestureActiveRef = useRef(false);
  const rafRef = useRef<number | null>(null);
  const pendingLiveRef = useRef<InkPendingLive | null>(null);
  const [draftDoc, setDraftDoc] = useState<InkDocument | null>(null);
  const [gestureRecognizer] = useState(() => new GestureRecognizer());
  const pinchCameraRef = useRef<InkCamera | null>(null);
  const [dragState, setDragState] = useState<InkDragState | null>(null);
  const [marqueePoints, setMarqueePoints] = useState<readonly SelectionMarqueePoint[]>([]);
  const [textEdit, setTextEdit] = useState<InkTextEditState | null>(null);
  const [tableEdit, setTableEdit] = useState<InkTableEditState | null>(null);
  const [contextMenu, setContextMenu] = useState<(SurfaceContextMenuResult & { readonly x: number; readonly y: number }) | null>(null);
  const contextMenuTitleLabel = useLabel(contextMenu?.titleKey ?? "ui.surfaceContextMenu.ink");
  const emptySceneLabel = useLabel("ui.host.emptyScene");

  const sceneDoc = useMemo(() => parseInkScene(scene?.documentJson), [scene?.documentJson]);
  const doc = draftDoc ?? sceneDoc;
  const docRef = useRef(doc);
  docRef.current = doc;
  const selectedIds = useMemo(() => parseSelectionIds(scene?.selectionJson), [scene?.selectionJson]);
  const selectedSet = useMemo(() => new Set(selectedIds), [selectedIds]);
  const hoveredId = scene?.hoveredId ?? null;
  const isNavigator = scene?.viewMode === "navigator";
  const interactive = scene?.interactive ?? false;
  const interactionDomain = useMemo(
    () => (scene?.interactionDomain?.id && scene.interactionDomain.granularityId ? { domainId: scene.interactionDomain.id, granularity: scene.interactionDomain.granularityId } : null),
    [scene?.interactionDomain],
  );
  const presenceRuntimeKey = usePresenceRuntimeKeyV1();
  const presenceWindowId = windowInstanceId ?? node.surfaceId ?? "ink";
  const localPresenceActor = useLocalPresenceActorIdV1(presenceRuntimeKey);
  const presencePointerRef = useRef<Vec2 | null>(null);
  const presencePublishAtRef = useRef(0);
  const publishInkPresenceView = useCallback(
    (force: boolean) => {
      const now = Date.now();
      if (isNavigator || (!force && now - presencePublishAtRef.current < PRESENCE_VIEW_PUBLISH_MIN_INTERVAL_MS)) return;
      const root = rootRef.current;
      const camera = docRef.current?.camera;
      if (!root || !camera) return;
      presencePublishAtRef.current = now;
      const size: [number, number] = [root.clientWidth, root.clientHeight];
      const pointer = presencePointerRef.current;
      if (!presenceRuntimeKey) return;
      publishLocalPresenceWindowViewV1(presenceRuntimeKey, presenceWindowId, { windowId: presenceWindowId, space: "canvas", kind: { kind: "canvas", ...inkPresenceCanvasV1(camera, size) }, size, ...(pointer ? { pointer: [pointer[0], pointer[1], 0] as const } : {}) });
    },
    [isNavigator, presenceWindowId, presenceRuntimeKey],
  );
  useEffect(() => publishInkPresenceView(true), [publishInkPresenceView, doc?.camera.x, doc?.camera.y, doc?.camera.zoom]);
  useEffect(() => presenceRuntimeKey ? () => clearLocalPresenceWindowViewV1(presenceWindowId, presenceRuntimeKey) : undefined, [presenceWindowId, presenceRuntimeKey]);

  useEffect(() => {
    if (!gestureActiveRef.current) setDraftDoc(null);
  }, [scene?.documentJson]);

  useEffect(() => {
    if (!windowInstanceId) return;
    return registerIntroductionSurfaceResolver(windowElementId(windowInstanceId), {
      canvasPoint: (x, y) => {
        const root = rootRef.current;
        const document = docRef.current;
        if (!root || !document) return null;
        const rect = root.getBoundingClientRect();
        const screen = worldToScreen(document.camera, x, y);
        return { x: rect.left + screen.x, y: rect.top + screen.y, visible: true };
      },
      entity: (domain, entityId): IntroductionResolvedGeometry | null => {
        const root = rootRef.current;
        const document = docRef.current;
        if (!root || !document) return null;
        const rect = root.getBoundingClientRect();
        if (domain === "block") {
          const items = flattenInkItems(document.blocks);
          const block = entityId === "*" ? items[0] : findInkItem(document, entityId);
          if (!block) return null;
          const bounds = inkItemBounds(block);
          const topLeft = worldToScreen(document.camera, bounds.x, bounds.y);
          const bottomRight = worldToScreen(document.camera, bounds.x + bounds.width, bounds.y + bounds.height);
          return {
            point: { x: rect.left + (topLeft.x + bottomRight.x) / 2, y: rect.top + (topLeft.y + bottomRight.y) / 2 },
            rect: { x: rect.left + topLeft.x, y: rect.top + topLeft.y, width: bottomRight.x - topLeft.x, height: bottomRight.y - topLeft.y },
            visible: true,
          };
        }
        if (domain === "stroke") {
          const block = entityId === "*" ? flattenInkItems(document.blocks).find((item) => item.kind === "stroke") : findInkItem(document, entityId);
          if (!block || block.kind !== "stroke") return null;
          const polyline = block.points.map(([px, py]) => {
            const screen = worldToScreen(document.camera, block.x + px, block.y + py);
            return { x: rect.left + screen.x, y: rect.top + screen.y };
          });
          const point=polyline[Math.floor(polyline.length/2)];if(!point)return null;
          return{point,polyline,visible:true};
        }
        return null;
      },
    });
  }, [windowInstanceId]);
  return { contextMenu, contextMenuTitleLabel, doc, dragState, emptySceneLabel, gestureActiveRef, gestureRecognizer, hoveredId, interactionDomain, interactive, isNavigator, localPresenceActor, marqueePoints, node, onAction, pendingLiveRef, pinchCameraRef, presencePointerRef, presenceWindowId, publishInkPresenceView, rafRef, requestContextMenu, rootRef, scene, sceneDoc, selectedIds, selectedSet, setContextMenu, setDraftDoc, setDragState, setMarqueePoints, setTableEdit, setTextEdit, tableEdit, textEdit, windowInstanceId , presenceRuntimeKey };
}

export type InkCore1 = ReturnType<typeof useInkSessionState>;
// #endregion InkCanvasHost
