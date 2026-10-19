/** 🧰️ Board2dHost session state: the scene, peer scope and window instance, the container, canvas, session and scheduler refs, the pending lanes, the gesture recognizer, the selection, epoch, error, context menu and suggestion state, the labels, the dispatchers, the vitals publishing, the render scheduler and the container size reader. */
import { useCallback, useContext, useEffect, useMemo, useRef, useState } from "react";
import { useLabel, useShellScopeOptional, registerIntroductionSurfaceResolver, windowElementId } from "@semio-tech/ui-react";
import { type DemandFrameSchedulerV1, type Board2dWasmSession } from "@semio-tech/framework/canvas/react-renderer";
import { GestureRecognizer, type ComponentSceneHostProps, type Board2dScene } from "@semio-tech/framework";
import { type Board2dPeer, BoardSessionFactoryContext, createBoardPeerScope } from "../../🪪️WasmSessionLoader/🟦️.tsx";
import { useMapContextMenuSpecs, useShellContextMenuFallback, type SurfaceContextMenuResult } from "../../🗣️Interpreter/🖱️surface-menu/🟦️.ts";
import { WindowInstanceIdContext } from "../../🗣️Interpreter/🌉️scene-host-contract/🟦️.ts";
import { type Board2dStatus, board2dStatusJson, type BoardCamera, type BoardEventRow } from "../🔬️board-parsing/🟦️.ts";
import { boardWorldToScreen } from "../📍️drop-and-camera/🟦️.ts";

// #region Board2dHost
/** 🧰️ Holds the refs, state, labels, dispatchers and render scheduling of the board window. */
export function useBoardSessionState({ node, onAction, requestContextMenu }: ComponentSceneHostProps) {
  const scene = node.board2d;
  const factory = useContext(BoardSessionFactoryContext);
  const emptyPeerScope = useMemo(createBoardPeerScope, []);
  const peerScope = factory?.scope ?? emptyPeerScope;
  const peerRef = useRef<Board2dPeer | null>(null);
  const board2dHostShellScope = useShellScopeOptional();
  const windowInstanceId = useContext(WindowInstanceIdContext);
  const sceneRef = useRef(scene);
  sceneRef.current = scene;
  const emptySceneLabel = useLabel("ui.host.emptyScene");
  const containerRef = useRef<HTMLDivElement>(null);
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const sessionRef = useRef<Board2dWasmSession | null>(null);
  const rawSessionRef = useRef<Board2dWasmSession | null>(null);
  const schedulerRef = useRef<DemandFrameSchedulerV1 | null>(null);
  const bootSyncedRef = useRef(false);
  const pendingSnapshotSceneRef = useRef<Board2dScene | null>(null);
  const pendingEventRowsRef = useRef<BoardEventRow[]>([]);
  const hoverActiveRef = useRef(false);
  const cameraInteractionActiveRef = useRef(false);
  const cameraSettleTimeoutRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const vitalsPendingRef = useRef(false);
  const pendingCameraDispatchRef = useRef<{ readonly camera: BoardCamera } | null>(null);
  const [gestureRecognizer] = useState(() => new GestureRecognizer());
  const pendingSelectionJsonRef = useRef<string | null>(null);
  const onPeerGestureEndedRef = useRef<(flushed: boolean) => void>(() => {});
  const boardStatusRef = useRef<Board2dStatus>({ snapshotParsed: null, snapshotChars: 0, refusalReason: "", pendingEvents: 0, guestRevision: 0 });
  const [localSelectionJson, setLocalSelectionJson] = useState<string | null>(null);
  const [sessionEpoch, setSessionEpoch] = useState(0);
  const [sessionError, setSessionError] = useState<Error | null>(null);
  const [contextMenu, setContextMenu] = useState<(SurfaceContextMenuResult & { readonly x: number; readonly y: number; readonly suggestTarget: string | null }) | null>(null);
  /** 🔦️ The trace record of the suggestion row under the pointer — the board paints only that placement. */
  const [suggestionFocus, setSuggestionFocus] = useState<bigint | null>(null);
  const contextMenuTitleLabel = useLabel(contextMenu?.titleKey ?? "ui.surfaceContextMenu.board");
  const suggestionMenuTitleLabel = useLabel("ui.surfaceContextMenu.placementSuggestions");
  const suggestionCheckingPlacementLabel = useLabel("ui.host.checkingPlacement");
  const suggestionNoPlacementLabel = useLabel("ui.host.noPlacement");

  const dispatch = useCallback(
    (action: string, args?: Record<string, unknown>) => {
      onAction({ controllerId: node.controllerId, action, args: { surfaceId: node.surfaceId, ...args } });
    },
    [peerScope, node.controllerId, node.surfaceId, onAction],
  );

  /** 🏁️ `dispatch`'s awaitable twin — the only shape {@link createCoalescingActionDispatcher}'s
   * "at most one round trip outstanding" gate can actually arm, since `dispatch` throws `onAction`'s
   * promise away (World3dHost wave B33: 72 hover turns enqueued by one 70-move storm, 11 settled). */
  const dispatchSettled = useCallback(
    (action: string, args?: Record<string, unknown>) => Promise.resolve(onAction({ controllerId: node.controllerId, action, args: { surfaceId: node.surfaceId, ...args } })),
    [node.controllerId, node.surfaceId, onAction],
  );

  /** 💡️ The popup is a per-window surface, so the verbs that open, preview, place or close it
   * carry the exact window instance they belong to — a sibling pane must not adopt another pane's menu. */
  const dispatchSuggestion = useCallback(
    (action: string, args?: Record<string, unknown>) => {
      dispatch(action, { windowId: windowInstanceId ?? undefined, ...args });
    },
    [dispatch, windowInstanceId],
  );

  const mapContextMenu = useMapContextMenuSpecs(
    useCallback((action: string, args?: Record<string, unknown>) => (action === "openHandleSuggestions" ? dispatchSuggestion(action, args) : dispatch(action, args)), [dispatch, dispatchSuggestion]),
  );
  const mapSuggestionMenu = useMapContextMenuSpecs(dispatchSuggestion);
  const shellContextMenuFallback = useShellContextMenuFallback();

  /** 🩺️ Republishes the live probe vitals straight onto the container, the way
   * `data-board-snapshot-parsed` already is: a gumball drag and a marquee update these every frame, and
   * routing that through React state would re-render the whole pane on each pointer move. */
  const publishBoardVitals = useCallback((): void => {
    const container = containerRef.current;
    if (!container) return;
    const session = rawSessionRef.current;
    try {
      container.setAttribute("data-board-interaction-json", session?.interactionJson?.() ?? "{}");
      container.setAttribute("data-board-transform-json", session?.transformGumballJson?.() ?? "{}");
      container.setAttribute("data-board-target-regions-json", session?.targetRegionsJson?.() ?? "[]");
      // 🩺️ `data-board-positions-json` names NODES only, so nothing in the DOM could ever be aimed at a
      // handle — `connect`, `openHandleSuggestions` and `createEdge` all take handle ids. The engine
      // answers viewport-bounded and capped, so a 358-handle document publishes a bounded attribute.
      container.setAttribute("data-board-handle-positions-json", session?.handlePositionsJson?.() ?? "{}");
    } catch {
      /* session not ready */
    }
    container.setAttribute("data-board-status-json", board2dStatusJson(boardStatusRef.current));
  }, []);

  const publishBoardVitalsRef = useRef(publishBoardVitals);
  publishBoardVitalsRef.current = publishBoardVitals;

  /** 🎞️ Coalesces renderFrame() to at most one per animation frame, no matter how many raw pointer/wheel events fire in
   * between, through the one shared demand scheduler — the paint also republishes the probe vitals. */
  const scheduleRender = useCallback((): void => {
    vitalsPendingRef.current = true;
    schedulerRef.current?.invalidate();
  }, []);

  const readContainerSize = useCallback((): { w: number; h: number } => {
    const container = containerRef.current;
    if (!container) return { w: 1, h: 1 };
    const rect = container.getBoundingClientRect();
    return {
      w: Math.max(1, Math.round(rect.width || container.clientWidth)),
      h: Math.max(1, Math.round(rect.height || container.clientHeight)),
    };
  }, []);

  useEffect(() => {
    if (!windowInstanceId) return;
    return registerIntroductionSurfaceResolver(windowElementId(windowInstanceId), {
      // 🎯️ `entity` targeting (board nodes/edges/handles by id) needs an id→screen API the board-2d wasm
      // engine doesn't expose yet (mirroring the dag engine's `entity_screen_json` would be the fix) — a
      // known gap, not a silent guess: `scene.snapshotJson`'s node schema isn't a framework-owned shape
      // this file can safely parse. `canvasPoint` (world coordinates) is fully supported.
      canvasPoint: (x, y) => {
        const cameraJson = sceneRef.current?.cameraJson;
        if (!cameraJson) return null;
        const screen = boardWorldToScreen(cameraJson, readContainerSize(), { x, y });
        if (!screen) return null;
        const rect = containerRef.current?.getBoundingClientRect();
        if (!rect) return null;
        return { x: rect.left + screen.x, y: rect.top + screen.y, visible: true };
      },
    });
  }, [windowInstanceId, readContainerSize]);
  return { board2dHostShellScope, boardStatusRef, bootSyncedRef, cameraInteractionActiveRef, cameraSettleTimeoutRef, canvasRef, containerRef, contextMenu, contextMenuTitleLabel, dispatch, dispatchSettled, dispatchSuggestion, emptySceneLabel, factory, gestureRecognizer, hoverActiveRef, localSelectionJson, mapContextMenu, mapSuggestionMenu, node, onPeerGestureEndedRef, peerRef, peerScope, pendingCameraDispatchRef, pendingEventRowsRef, pendingSelectionJsonRef, pendingSnapshotSceneRef, publishBoardVitals, publishBoardVitalsRef, rawSessionRef, readContainerSize, requestContextMenu, scene, sceneRef, scheduleRender, schedulerRef, sessionEpoch, sessionError, sessionRef, setContextMenu, setLocalSelectionJson, setSessionEpoch, setSessionError, setSuggestionFocus, shellContextMenuFallback, suggestionCheckingPlacementLabel, suggestionFocus, suggestionMenuTitleLabel, suggestionNoPlacementLabel, vitalsPendingRef, windowInstanceId };
}

export type BoardCore1 = ReturnType<typeof useBoardSessionState>;
// #endregion Board2dHost
