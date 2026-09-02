// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/Board2dHost/component.tsx
/** @emoji 🧩️ `Board2dHost` — board-2d `ComponentSceneHost`: drives the board wasm session (fixture
 * sync, coalesced event drain/flush, marquee/pick pointer routing, catalogue fixture-drop preview),
 * plus the cross-pane live-mirror peer registry that keeps a triptych of panes on the same
 * `controllerId` in sync during a gesture without a plugin round trip. Reuses `World3dHost`'s
 * window-instance context and `Interpreter`'s surface context-menu plumbing. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { useCallback, useContext, useEffect, useLayoutEffect, useMemo, useRef, useState, type DragEvent, type KeyboardEvent, type MouseEvent } from "react";
import {
  useLabel,
  useShellScopeOptional,
  useCanvasAppearanceSync,
  ContextMenuController,
  type ContextMenuItem,
  registerIntroductionSurfaceResolver,
  windowElementId,
  type CanvasPickTarget,
  pickMostSpecificCanvasTarget,
  CATALOGUE_DRAG_MIME,
  getActiveCatalogueDragPayload,
} from "@semio-tech/ui-react";
import { syncSessionCanvasTheme } from "@semio-tech/ui-styling";
import { type ComponentSceneHostProps, type Board2dScene } from "@semio-tech/framework";
import { type Board2dWasmSession, type Board2dPeer, type BoardPeerScope, BoardSessionFactoryContext, createBoardPeerScope } from "../WasmSessionLoader/🟦️.tsx";
import { useMapContextMenuSpecs } from "../ShellHost/🟦️.tsx";
import { parseSelectionIds } from "../InkCanvasHost/🟦️.tsx";
// 🐢️ Direct element-to-element imports — `World3dHost`/`Interpreter` already landed in a prior batch.
import { WindowInstanceIdContext } from "../World3dHost/🟦️.tsx";
import { useShellContextMenuFallback, openSurfaceContextMenu, type SurfaceContextMenuResult } from "../Interpreter/🟦️.tsx";
// #endregion 🔌️Adapters

//#region 🔖️Board2dHost
//#region Types
type BoardCamera = { readonly x: number; readonly y: number; readonly zoom: number };
type BoardEventRow = { readonly name: string; readonly payload?: unknown };
type Puzzle2dFixtureDropPayload = {
  readonly kindId: string;
  readonly catalogSlice: string;
  readonly shape?: string;
  readonly radius?: number;
  readonly width?: number;
  readonly height?: number;
  readonly iconKind?: string;
};
//#endregion Types

//#region Parsing
function parseBoardCamera(json: string): BoardCamera | null {
  try {
    const parsed = JSON.parse(json) as Partial<BoardCamera>;
    if (typeof parsed.x !== "number" || typeof parsed.y !== "number" || typeof parsed.zoom !== "number") return null;
    return { x: parsed.x, y: parsed.y, zoom: parsed.zoom };
  } catch {
    return null;
  }
}

export function board2dCameraActionArgs(cameraJson: string): { readonly camera: BoardCamera } | null {
  const camera = parseBoardCamera(cameraJson);
  return camera ? { camera } : null;
}

export function parsePuzzle2dCatalogueDragPayload(encoded: string | null | undefined): Puzzle2dFixtureDropPayload | null {
  if (!encoded) return null;
  try {
    const parsed = JSON.parse(encoded) as Partial<Puzzle2dFixtureDropPayload>;
    if (typeof parsed.kindId !== "string") return null;
    return {
      kindId: parsed.kindId,
      catalogSlice: typeof parsed.catalogSlice === "string" ? parsed.catalogSlice : "nodes",
      shape: typeof parsed.shape === "string" ? parsed.shape : undefined,
      radius: typeof parsed.radius === "number" ? parsed.radius : undefined,
      width: typeof parsed.width === "number" ? parsed.width : undefined,
      height: typeof parsed.height === "number" ? parsed.height : undefined,
      iconKind: typeof parsed.iconKind === "string" ? parsed.iconKind : undefined,
    };
  } catch {
    return null;
  }
}
//#endregion Parsing

//#region BoardEvents
const PUZZLE2D_TRANSIENT_EVENT_NAMES = new Set(["preselect", "brushPreview", "linkCompatibleNodes", "linkTargetRing"]);
const PUZZLE2D_FLUSH_NOW_EVENT_NAMES = new Set(["select", "preselectCancel", "brushCandidates", "brushPlace", "edgeCreate", "edgeDelete", "nodeDelete"]);

/** @emoji 📬️ Drops transient rows, coalesces `camera` to its latest value and `nodeMove` to one row per id (unless a `nodeDragEnd` follows), and flags whether the buffer should flush immediately. */
export function coalesceBoard2dEvents(rows: readonly BoardEventRow[]): { readonly flushNow: boolean; readonly eventsJson: string } {
  const hasDragEnd = rows.some((row) => row.name === "nodeDragEnd");
  let flushNow = false;
  let lastCamera: BoardEventRow | null = null;
  const nodeMoveById = new Map<string, BoardEventRow>();
  const rest: BoardEventRow[] = [];

  for (const row of rows) {
    if (PUZZLE2D_TRANSIENT_EVENT_NAMES.has(row.name)) continue;
    if (row.name === "camera") {
      lastCamera = row;
      continue;
    }
    if (row.name === "nodeMove") {
      if (hasDragEnd) continue;
      const id = (row.payload as { readonly id?: unknown } | undefined)?.id;
      if (typeof id === "string") {
        nodeMoveById.set(id, row);
        continue;
      }
    }
    if (PUZZLE2D_FLUSH_NOW_EVENT_NAMES.has(row.name)) flushNow = true;
    rest.push(row);
  }

  const coalesced: BoardEventRow[] = [];
  if (lastCamera) coalesced.push(lastCamera);
  coalesced.push(...nodeMoveById.values());
  coalesced.push(...rest);
  return { flushNow, eventsJson: JSON.stringify(coalesced) };
}

/** @emoji 🐢️ Live cross-pane mirror payload extracted from a batch of freshly-drained rows — positions/selection/preselect only, everything else (camera, brush/link chrome, hover) stays pane-local. */
export type Puzzle2dLiveMirrorMutations = {
  readonly positions: readonly { readonly id: string; readonly x: number; readonly y: number }[];
  readonly selectionIds: readonly string[] | null;
  readonly preselect: { readonly ids: readonly string[]; readonly removedIds: readonly string[] } | null;
  readonly clearPreselect: boolean;
};

function stringArray(value: unknown): readonly string[] {
  return Array.isArray(value) ? value.filter((entry): entry is string => typeof entry === "string") : [];
}

/**
 * @emoji 🐢️ Classifies a batch of raw board-event rows (as seen straight off `drainEventsJson`, before
 * the transient-event filter/coalescer runs) into the subset worth mirroring imperatively into sibling
 * panes: latest node position per id (from `nodeMove` frames and/or a terminal `nodeDragEnd`), and the
 * live selection/preselect state (`select`/`preselectCancel` commit or restore selection and clear
 * preselect; `preselect` sets the live marquee highlight). Multiple rows of the same kind in one batch
 * collapse to the latest.
 */
export function collectPuzzle2dLiveMirrorMutations(rows: readonly BoardEventRow[]): Puzzle2dLiveMirrorMutations {
  const positionsById = new Map<string, { readonly id: string; readonly x: number; readonly y: number }>();
  let selectionIds: readonly string[] | null = null;
  let preselect: { readonly ids: readonly string[]; readonly removedIds: readonly string[] } | null = null;
  let clearPreselect = false;

  for (const row of rows) {
    const payload = row.payload as Record<string, unknown> | undefined;
    switch (row.name) {
      case "nodeMove": {
        const id = payload?.id;
        const x = payload?.x;
        const y = payload?.y;
        if (typeof id === "string" && typeof x === "number" && typeof y === "number") positionsById.set(id, { id, x, y });
        break;
      }
      case "nodeDragEnd": {
        const moves = payload?.moves;
        if (!Array.isArray(moves)) break;
        for (const move of moves as readonly Record<string, unknown>[]) {
          const id = move.id;
          const x = move.x;
          const y = move.y;
          if (typeof id === "string" && typeof x === "number" && typeof y === "number") positionsById.set(id, { id, x, y });
        }
        break;
      }
      case "preselect": {
        preselect = { ids: stringArray(payload?.ids), removedIds: stringArray(payload?.removedIds) };
        clearPreselect = false;
        break;
      }
      case "preselectCancel": {
        selectionIds = stringArray(payload?.ids);
        preselect = null;
        clearPreselect = true;
        break;
      }
      case "select": {
        selectionIds = stringArray(payload?.ids);
        preselect = null;
        clearPreselect = true;
        break;
      }
      default:
        break;
    }
  }

  return { positions: [...positionsById.values()], selectionIds, preselect, clearPreselect };
}
//#endregion BoardEvents

//#region SelectionMenu
function puzzle2dEntityFlag(entity: Record<string, unknown> | undefined, key: "hidden" | "locked"): boolean {
  return Boolean(entity && entity[key] === true);
}

/** @emoji 🖱️ Right-click menu for the current selection: Hide/Show, Lock/Unlock, Duplicate, Select same kind, Zoom to selection, Delete — mirrors the premigration canvas context menu. */
//#endregion SelectionMenu

//#region FixtureDrop
/** @emoji 👻️ Builds a world-space fixture-drop preview so every peer pane shares the same ghost (screen coords would desync under different cameras). */
export function puzzle2dFixtureDropPreviewJson(payload: Puzzle2dFixtureDropPayload, worldX: number, worldY: number): string {
  return JSON.stringify({ nodeKind: payload.kindId, x: worldX, y: worldY, shape: payload.shape, radius: payload.radius, width: payload.width, height: payload.height, iconKind: payload.iconKind });
}

/** @emoji 📐️ Inverse of the canonical `screenX = (worldX - camera.x) * zoom + width / 2` transform shared across board renderers. */
export function puzzle2dScreenToWorld(cameraJson: string, containerSize: { readonly w: number; readonly h: number }, screen: { readonly x: number; readonly y: number }): { readonly x: number; readonly y: number } | null {
  const camera = parseBoardCamera(cameraJson);
  if (!camera) return null;
  const zoom = camera.zoom || 1;
  return {
    x: camera.x + (screen.x - containerSize.w / 2) / zoom,
    y: camera.y + (screen.y - containerSize.h / 2) / zoom,
  };
}

/** @emoji 📐️ The canonical `screenX = (worldX - camera.x) * zoom + width / 2` transform shared across
 * board renderers — the missing inverse of {@link puzzle2dScreenToWorld}, needed for demonstration
 * targeting (a world point/entity → the viewport pixel a ghost cursor animates to). */
export function puzzle2dWorldToScreen(cameraJson: string, containerSize: { readonly w: number; readonly h: number }, world: { readonly x: number; readonly y: number }): { readonly x: number; readonly y: number } | null {
  const camera = parseBoardCamera(cameraJson);
  if (!camera) return null;
  const zoom = camera.zoom || 1;
  return {
    x: (world.x - camera.x) * zoom + containerSize.w / 2,
    y: (world.y - camera.y) * zoom + containerSize.h / 2,
  };
}
//#endregion FixtureDrop

//#region Sync
function applyToSession(session: Board2dWasmSession | null, action: (session: Board2dWasmSession) => void): void {
  if (!session) return;
  try {
    action(session);
    session.renderFrame();
  } catch {
    /* session not ready */
  }
}

/** @emoji 🔁️ Re-parses the fixture and silently re-applies selection/camera, since `parseFixtureJson` resets both to the fixture's own defaults. */
function applyFixtureToSession(session: Board2dWasmSession, scene: Board2dScene): void {
  session.parseFixtureJson(scene.fixtureJson);
  session.setSelectionOptions?.(scene.selectionMethod, "replace", true, true, true);
  if (session.setSelectionIdsJsonSilent) session.setSelectionIdsJsonSilent(scene.selectionJson);
  else session.setSelectionIdsJson(scene.selectionJson);
  const camera = parseBoardCamera(scene.cameraJson);
  if (camera) {
    if (session.setCameraSilent) session.setCameraSilent(camera.x, camera.y, camera.zoom);
    else session.setCamera(camera.x, camera.y, camera.zoom);
  }
}
//#endregion Sync

//#region PeerSync
export function registerBoard2dPeer(scope: BoardPeerScope, controllerId: string, surfaceId: string, peer: Board2dPeer): void {
  let peers = scope.peers.get(controllerId);
  if (!peers) {
    peers = new Map();
    scope.peers.set(controllerId, peers);
  }
  peers.set(surfaceId, peer);
}

export function unregisterBoard2dPeer(scope: BoardPeerScope, controllerId: string, surfaceId: string, peer: Board2dPeer | null): void {
  const peers = scope.peers.get(controllerId);
  if (!peers || !peer || peers.get(surfaceId) !== peer) return;
  peers.delete(surfaceId);
  if (peers.size === 0) scope.peers.delete(controllerId);
  endPuzzle2dPeerGesture(scope, controllerId, surfaceId, peer);
}

export function board2dPeers(scope: BoardPeerScope, controllerId: string, excludeSurfaceId: string): readonly Board2dPeer[] {
  const peers = scope.peers.get(controllerId);
  if (!peers) return [];
  const result: Board2dPeer[] = [];
  for (const [surfaceId, peer] of peers) if (surfaceId !== excludeSurfaceId) result.push(peer);
  return result;
}

export function beginPuzzle2dPeerGesture(scope: BoardPeerScope, controllerId: string, surfaceId: string, peer: Board2dPeer | null): void {
  if (peer && scope.peers.get(controllerId)?.get(surfaceId) === peer) scope.gestures.set(controllerId, { surfaceId, peer });
}

export function endPuzzle2dPeerGesture(scope: BoardPeerScope, controllerId: string, surfaceId: string, peer: Board2dPeer | null): void {
  const owner = scope.gestures.get(controllerId);
  if (owner?.surfaceId === surfaceId && owner.peer === peer) scope.gestures.delete(controllerId);
}

/** @emoji 🙅️ True when a *different* pane owns the live gesture for this controller — the caller should defer applying an echoed scene. */
export function puzzle2dPeerOwnsGesture(scope: BoardPeerScope, controllerId: string, surfaceId: string): boolean {
  const owner = scope.gestures.get(controllerId);
  return owner !== undefined && owner.surfaceId !== surfaceId;
}

export function pushPuzzle2dLiveMirrorMutations(scope: BoardPeerScope, controllerId: string, surfaceId: string, mutations: Puzzle2dLiveMirrorMutations): void {
  if (mutations.positions.length === 0 && !mutations.selectionIds && !mutations.preselect && !mutations.clearPreselect) return;
  const peers = board2dPeers(scope, controllerId, surfaceId);
  if (peers.length === 0) return;
  const positionsJson = mutations.positions.length > 0 ? JSON.stringify(mutations.positions) : null;
  const selectionJson = mutations.selectionIds ? JSON.stringify(mutations.selectionIds) : null;
  const preselectJson = mutations.preselect ? JSON.stringify(mutations.preselect) : mutations.clearPreselect ? JSON.stringify({ ids: [], removedIds: [] }) : null;
  for (const peer of peers) {
    try {
      if (positionsJson) peer.session.setNodePositionsJson?.(positionsJson);
      if (selectionJson) peer.session.setSelectionIdsJsonSilent?.(selectionJson);
      if (preselectJson) peer.session.setPreselectStateJsonSilent?.(preselectJson);
    } catch {
      /* peer session not ready */
    }
  }
}

export function notifyPuzzle2dPeersGestureEnded(scope: BoardPeerScope, controllerId: string, surfaceId: string, flushed: boolean): void {
  for (const peer of board2dPeers(scope, controllerId, surfaceId)) {
    try {
      peer.onPeerGestureEnded(flushed);
    } catch {
      /* peer session not ready */
    }
  }
}

/** @emoji 👻️ Pushes a world-space catalogue fixture-drop ghost into every pane of `controllerId` (including the source). */
export function pushPuzzle2dFixtureDropPreview(scope: BoardPeerScope, controllerId: string, previewJson: string | null): void {
  const peers = scope.peers.get(controllerId);
  if (!peers) return;
  for (const peer of peers.values()) {
    try {
      if (previewJson) peer.session.setFixtureDropPreviewJson?.(previewJson);
      else peer.session.clearFixtureDropPreview?.();
      peer.session.renderFrame?.();
    } catch {
      /* peer session not ready */
    }
  }
}
//#endregion PeerSync

//#region Board2dHost
export function Board2dHost({ node, onAction, requestContextMenu }: ComponentSceneHostProps) {
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
  const bootSyncedRef = useRef(false);
  const pendingFixtureSceneRef = useRef<Board2dScene | null>(null);
  const pendingEventRowsRef = useRef<BoardEventRow[]>([]);
  const hoverActiveRef = useRef(false);
  const cameraInteractionActiveRef = useRef(false);
  const cameraSettleTimeoutRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const renderScheduledRef = useRef(false);
  const pendingCameraDispatchRef = useRef<{ readonly camera: BoardCamera } | null>(null);
  const pendingSelectionJsonRef = useRef<string | null>(null);
  const onPeerGestureEndedRef = useRef<(flushed: boolean) => void>(() => {});
  const [sessionEpoch, setSessionEpoch] = useState(0);
  const [sessionError, setSessionError] = useState<Error | null>(null);
  const [contextMenu, setContextMenu] = useState<(SurfaceContextMenuResult & { readonly x: number; readonly y: number }) | null>(null);
  const contextMenuTitleLabel = useLabel(contextMenu?.titleKey ?? "ui.surfaceContextMenu.board");

  const dispatch = useCallback(
    (action: string, args?: Record<string, unknown>) => {
      onAction({ controllerId: node.controllerId, action, args: { surfaceId: node.surfaceId, ...args } });
    },
    [peerScope, node.controllerId, node.surfaceId, onAction],
  );

  const mapContextMenu = useMapContextMenuSpecs(dispatch);
  const shellContextMenuFallback = useShellContextMenuFallback();

  /** @emoji 🎞️ Coalesces renderFrame() to at most one per animation frame, no matter how many raw pointer/wheel events fire in between — mirrors the premigration `scheduleInputInvalidate()` pattern. */
  const scheduleRender = useCallback((): void => {
    if (renderScheduledRef.current) return;
    renderScheduledRef.current = true;
    requestAnimationFrame(() => {
      renderScheduledRef.current = false;
      try {
        sessionRef.current?.renderFrame();
      } catch {
        /* gpu not ready */
      }
    });
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
      // known gap, not a silent guess: `scene.fixtureJson`'s node schema isn't a framework-owned shape
      // this file can safely parse. `canvasPoint` (world coordinates) is fully supported.
      canvasPoint: (x, y) => {
        const cameraJson = sceneRef.current?.cameraJson;
        if (!cameraJson) return null;
        const screen = puzzle2dWorldToScreen(cameraJson, readContainerSize(), { x, y });
        if (!screen) return null;
        const rect = containerRef.current?.getBoundingClientRect();
        if (!rect) return null;
        return { x: rect.left + screen.x, y: rect.top + screen.y, visible: true };
      },
    });
  }, [windowInstanceId, readContainerSize]);

  //#region BoardEventFlush
  const drainIntoBuffer = useCallback((): void => {
    const session = sessionRef.current;
    if (!session) return;
    try {
      const json = session.drainEventsJson();
      if (!json || json === "[]") return;
      const rows = JSON.parse(json) as BoardEventRow[];
      pendingEventRowsRef.current.push(...rows);
      pushPuzzle2dLiveMirrorMutations(peerScope, node.controllerId, node.surfaceId, collectPuzzle2dLiveMirrorMutations(rows));
    } catch {
      /* session not ready */
    }
  }, [peerScope, node.controllerId, node.surfaceId]);

  const dispatchBufferedEvents = useCallback((): void => {
    if (pendingEventRowsRef.current.length === 0) return;
    const { eventsJson } = coalesceBoard2dEvents(pendingEventRowsRef.current);
    pendingEventRowsRef.current = [];
    if (eventsJson && eventsJson !== "[]") dispatch("applyBoardEvents", { eventsJson });
  }, [dispatch]);

  const drainAndMaybeFlush = useCallback((): void => {
    drainIntoBuffer();
    if (pendingEventRowsRef.current.length === 0) return;
    const { flushNow } = coalesceBoard2dEvents(pendingEventRowsRef.current);
    if (flushNow) dispatchBufferedEvents();
  }, [drainIntoBuffer, dispatchBufferedEvents]);

  const flushBoardEvents = useCallback((): void => {
    drainIntoBuffer();
    dispatchBufferedEvents();
  }, [drainIntoBuffer, dispatchBufferedEvents]);

  const applyPendingFixtureIfReady = useCallback(
    (session: Board2dWasmSession): void => {
      const pendingScene = pendingFixtureSceneRef.current;
      if (!pendingScene) return;
      if (session.defersDescriptorSyncFromJs?.() || cameraInteractionActiveRef.current || puzzle2dPeerOwnsGesture(peerScope, node.controllerId, node.surfaceId)) return;
      pendingFixtureSceneRef.current = null;
      applyToSession(session, (s) => applyFixtureToSession(s, pendingScene));
    },
    [peerScope, node.controllerId, node.surfaceId],
  );

  /** @emoji 🐢️ Mirror of `applyPendingFixtureIfReady` for the selection-only echo — a peer-owned gesture defers the plugin's `selectionJson` so it doesn't clobber a mirrored preselect highlight mid-marquee. */
  const applyPendingSelectionIfReady = useCallback(
    (session: Board2dWasmSession): void => {
      const pendingSelectionJson = pendingSelectionJsonRef.current;
      if (pendingSelectionJson === null) return;
      if (puzzle2dPeerOwnsGesture(peerScope, node.controllerId, node.surfaceId)) return;
      pendingSelectionJsonRef.current = null;
      applyToSession(session, (s) => {
        if (s.setSelectionIdsJsonSilent) s.setSelectionIdsJsonSilent(pendingSelectionJson);
        else s.setSelectionIdsJson(pendingSelectionJson);
      });
    },
    [peerScope, node.controllerId, node.surfaceId],
  );

  onPeerGestureEndedRef.current = (flushed: boolean): void => {
    const session = sessionRef.current;
    if (!session) return;
    if (flushed) {
      pendingFixtureSceneRef.current = null;
      pendingSelectionJsonRef.current = null;
      return;
    }
    applyPendingFixtureIfReady(session);
    applyPendingSelectionIfReady(session);
  };

  /**
   * @emoji 🫧️ Call when a gesture on this pane ends, right before flushing. Drains first so we know
   * whether a commit is about to go out; if so, drops any pending fixture/selection stashed mid-gesture
   * instead of applying it — that stashed snapshot is stale (typically from an early mid-gesture flush,
   * e.g. the `select` event a node-drag's pointerdown pushes) and the flush response due back in a moment
   * will supersede it anyway, so applying it here would flicker: correct live state -> stale snapshot ->
   * correct committed state. Returns whether a flush is pending, so the caller can pass it on to peers.
   */
  const settleGestureEnd = useCallback(
    (session: Board2dWasmSession): boolean => {
      drainIntoBuffer();
      const flushed = pendingEventRowsRef.current.length > 0;
      if (flushed) {
        pendingFixtureSceneRef.current = null;
        pendingSelectionJsonRef.current = null;
      } else {
        applyPendingFixtureIfReady(session);
        applyPendingSelectionIfReady(session);
      }
      return flushed;
    },
    [applyPendingFixtureIfReady, applyPendingSelectionIfReady, drainIntoBuffer],
  );

  /** @emoji 🐁️ Marks a wheel-zoom gesture in flight so scene-driven camera echoes (which lag several ticks behind during a fast scroll) don't fight the live local zoom — mirrors `defersDescriptorSyncFromJs` for pan/drag, which the engine doesn't track for wheel. */
  const beginCameraInteraction = useCallback((): void => {
    cameraInteractionActiveRef.current = true;
    if (cameraSettleTimeoutRef.current) clearTimeout(cameraSettleTimeoutRef.current);
    cameraSettleTimeoutRef.current = setTimeout(() => {
      cameraInteractionActiveRef.current = false;
      cameraSettleTimeoutRef.current = null;
      const session = sessionRef.current;
      if (session) applyPendingFixtureIfReady(session);
      const pendingCamera = pendingCameraDispatchRef.current;
      if (pendingCamera) {
        pendingCameraDispatchRef.current = null;
        dispatch("setCamera", pendingCamera);
      }
    }, 350);
  }, [applyPendingFixtureIfReady, dispatch]);

  useEffect(
    () => () => {
      if (cameraSettleTimeoutRef.current) clearTimeout(cameraSettleTimeoutRef.current);
    },
    [],
  );
  //#endregion BoardEventFlush

  //#region SessionLifecycle
  useLayoutEffect(() => {
    const canvas = canvasRef.current;
    const container = containerRef.current;
    if (!canvas || !container) return undefined;
    if (!factory) throw new Error("The current app has no registered board session factory.");
    let disposed = false;
    let resizeObserver: ResizeObserver | null = null;
    let raf = 0;
    let owner: Board2dWasmSession | null = null;
    let peer: Board2dPeer | null = null;
    let booting = false;
    const release = (): void => {
      const session = owner;
      owner = null;
      if (sessionRef.current === session) sessionRef.current = null;
      session?.free();
    };
    const fail = (error: unknown): void => {
      resizeObserver?.disconnect();
      unregisterBoard2dPeer(peerScope, node.controllerId, node.surfaceId, peer);
      release();
      if (!disposed) setSessionError(error instanceof Error ? error : new Error(String(error)));
    };

    void factory.create().then((session) => {
      if (disposed) {
        session.free();
        return;
      }
      owner = session;
      sessionRef.current = session;
      peer = { session, onPeerGestureEnded: (flushed) => onPeerGestureEndedRef.current(flushed) };
      peerRef.current = peer;
      registerBoard2dPeer(peerScope, node.controllerId, node.surfaceId, peer);

      const applySize = (): void => {
        const nextDpr = globalThis.devicePixelRatio || 1;
        const { w, h } = readContainerSize();
        session.setSize(w, h, nextDpr);
      };

      const boot = async (): Promise<void> => {
        let { w, h } = readContainerSize();
        for (let attempt = 0; attempt < 240 && (w < 64 || h < 64); attempt += 1) {
          await new Promise<void>((resolve) => {
            if (typeof globalThis.requestAnimationFrame === "function") globalThis.requestAnimationFrame(() => resolve());
            else queueMicrotask(resolve);
          });
          if (disposed) return;
          ({ w, h } = readContainerSize());
        }
        const dpr = globalThis.devicePixelRatio || 1;
        await session.attach_canvas(canvas, w, h, dpr);
        if (disposed) return;
        applySize();
        syncSessionCanvasTheme(session);
        const tick = () => {
          if (disposed) return;
          try {
            session.renderFrame();
          } catch {
            /* gpu not ready */
          }
          raf = requestAnimationFrame(tick);
        };
        raf = requestAnimationFrame(tick);
        setSessionEpoch((epoch) => epoch + 1);
      };

      resizeObserver =
        typeof ResizeObserver === "undefined"
          ? null
          : new ResizeObserver(() => {
              applySize();
            });
      resizeObserver?.observe(container);
      booting = true;
      void boot().finally(() => {
        booting = false;
        if (disposed) release();
      }).catch(fail);
    }).catch(fail);

    return () => {
      disposed = true;
      resizeObserver?.disconnect();
      if (raf) cancelAnimationFrame(raf);
      unregisterBoard2dPeer(peerScope, node.controllerId, node.surfaceId, peer);
      if (peerRef.current === peer) peerRef.current = null;
      if (sessionRef.current === owner) sessionRef.current = null;
      if (!booting) release();
    };
  }, [node.controllerId, node.surfaceId, readContainerSize, factory?.create, factory?.pluginId, factory?.appId, factory?.instanceId, peerScope]);
  //#endregion SessionLifecycle

  //#region SceneSync
  useEffect(() => {
    if (!scene) return;
    const session = sessionRef.current;
    if (!session) return;
    if (session.defersDescriptorSyncFromJs?.() || cameraInteractionActiveRef.current || puzzle2dPeerOwnsGesture(peerScope, node.controllerId, node.surfaceId)) {
      pendingFixtureSceneRef.current = scene;
      return;
    }
    applyToSession(session, (s) => applyFixtureToSession(s, scene));
    if (!bootSyncedRef.current) {
      bootSyncedRef.current = true;
      try {
        session.drainEventsJson();
      } catch {
        /* session not ready */
      }
    }
  }, [peerScope, sessionEpoch, scene?.fixtureJson, node.controllerId, node.surfaceId]);

  useEffect(() => {
    if (!scene) return;
    applyToSession(sessionRef.current, (session) => session.setKindCatalogsJson(scene.glyphCatalogsJson));
  }, [sessionEpoch, scene?.glyphCatalogsJson]);

  useEffect(() => {
    if (!scene) return;
    applyToSession(sessionRef.current, (session) => session.setHandleLinkCompatJson?.(scene.placementCompatibilityJson));
  }, [sessionEpoch, scene?.placementCompatibilityJson]);

  useEffect(() => {
    if (!scene) return;
    const session = sessionRef.current;
    if (!session) return;
    if (puzzle2dPeerOwnsGesture(peerScope, node.controllerId, node.surfaceId)) {
      pendingSelectionJsonRef.current = scene.selectionJson;
      return;
    }
    applyToSession(session, (s) => {
      if (s.setSelectionIdsJsonSilent) s.setSelectionIdsJsonSilent(scene.selectionJson);
      else s.setSelectionIdsJson(scene.selectionJson);
    });
  }, [peerScope, sessionEpoch, scene?.selectionJson, node.controllerId, node.surfaceId]);

  useEffect(() => {
    if (!scene) return;
    const session = sessionRef.current;
    if (!session || session.defersDescriptorSyncFromJs?.() || cameraInteractionActiveRef.current) return;
    applyToSession(session, (s) => {
      const camera = parseBoardCamera(scene.cameraJson);
      if (!camera) return;
      if (s.setCameraSilent) s.setCameraSilent(camera.x, camera.y, camera.zoom);
      else s.setCamera(camera.x, camera.y, camera.zoom);
    });
  }, [sessionEpoch, scene?.cameraJson]);

  useEffect(() => {
    if (!scene) return;
    applyToSession(sessionRef.current, (session) => session.setHoveredIdSilent?.(scene.hoveredId ?? null));
  }, [sessionEpoch, scene?.hoveredId]);

  useEffect(() => {
    if (!scene) return;
    applyToSession(sessionRef.current, (session) => session.setActiveUtility?.(scene.activeUtility ?? "select"));
  }, [sessionEpoch, scene?.activeUtility]);

  useEffect(() => {
    if (!scene || !board2dHostShellScope) return;
    const updateOptions = () => {
      const mode = board2dHostShellScope.selection.get();
      applyToSession(sessionRef.current, (session) => session.setSelectionOptions?.(scene.selectionMethod, mode, true, true, true));
    };
    updateOptions();
    return board2dHostShellScope.selection.subscribe(updateOptions);
  }, [sessionEpoch, scene?.selectionMethod, board2dHostShellScope]);

  useEffect(() => {
    if (!scene) return;
    applyToSession(sessionRef.current, (session) => session.setGridSnapEnabled?.(scene.gridSnapEnabled));
  }, [sessionEpoch, scene?.gridSnapEnabled]);

  useEffect(() => {
    if (!scene) return;
    applyToSession(sessionRef.current, (session) => session.setGridFactor?.(scene.gridFactor));
  }, [sessionEpoch, scene?.gridFactor]);

  useEffect(() => {
    if (!scene || scene.suggestionOffset <= 0) return;
    applyToSession(sessionRef.current, (session) => session.setSuggestionOffset?.(scene.suggestionOffset));
  }, [sessionEpoch, scene?.suggestionOffset]);

  useEffect(() => {
    if (!scene) return;
    applyToSession(sessionRef.current, (session) => session.setBrushKindWeights?.(scene.brushWeightsJson));
  }, [sessionEpoch, scene?.brushWeightsJson]);

  useEffect(() => {
    if (!scene) return;
    applyToSession(sessionRef.current, (session) => {
      if (scene.lodMode === "automatic") {
        session.setAutomaticLod?.(true);
      } else {
        session.setAutomaticLod?.(false);
        session.setForcedDrawLodLabel?.(scene.lodMode);
      }
    });
  }, [sessionEpoch, scene?.lodMode]);
  //#endregion SceneSync

  useCanvasAppearanceSync(
    () => {
      syncSessionCanvasTheme(sessionRef.current);
      try {
        sessionRef.current?.renderFrame();
      } catch {
        /* gpu not ready */
      }
    },
    true,
    board2dHostShellScope?.rootRef.current ?? undefined,
  );

  //#region Pointer
  useEffect(() => {
    const canvas = canvasRef.current;
    const container = containerRef.current;
    if (!canvas || !container || !scene?.interactive) return undefined;

    const clientToLocal = (clientX: number, clientY: number): { x: number; y: number } => {
      const rect = canvas.getBoundingClientRect();
      return { x: clientX - rect.left, y: clientY - rect.top };
    };

    const onPointerDown = (event: PointerEvent): void => {
      event.stopPropagation();
      const session = sessionRef.current;
      if (!session) return;
      const point = clientToLocal(event.clientX, event.clientY);
      if (event.button === 0 || event.button === 1) {
        canvas.setPointerCapture?.(event.pointerId);
      }
      beginPuzzle2dPeerGesture(peerScope, node.controllerId, node.surfaceId, peerRef.current);
      session.pointerDownScreen(point.x, point.y, event.button, event.shiftKey, event.metaKey || event.ctrlKey);
      scheduleRender();
    };

    const onPointerMove = (event: PointerEvent): void => {
      const session = sessionRef.current;
      if (!session) return;
      const point = clientToLocal(event.clientX, event.clientY);
      session.pointerMoveScreen(point.x, point.y, event.shiftKey, event.metaKey || event.ctrlKey, event.altKey);
      scheduleRender();
      drainAndMaybeFlush();
    };

    const onPointerUp = (event: PointerEvent): void => {
      const session = sessionRef.current;
      if (!session) return;
      const point = clientToLocal(event.clientX, event.clientY);
      session.pointerUpScreen(point.x, point.y, event.shiftKey, event.metaKey || event.ctrlKey, event.altKey);
      if (canvas.hasPointerCapture?.(event.pointerId)) canvas.releasePointerCapture(event.pointerId);
      endPuzzle2dPeerGesture(peerScope, node.controllerId, node.surfaceId, peerRef.current);
      const flushed = settleGestureEnd(session);
      scheduleRender();
      dispatchBufferedEvents();
      notifyPuzzle2dPeersGestureEnded(peerScope, node.controllerId, node.surfaceId, flushed);
    };

    const onPointerEnter = (): void => {
      hoverActiveRef.current = true;
    };

    const onPointerLeave = (event: PointerEvent): void => {
      hoverActiveRef.current = false;
      const session = sessionRef.current;
      if (!session) return;
      session.pointerLeaveScreen?.(event.altKey);
      endPuzzle2dPeerGesture(peerScope, node.controllerId, node.surfaceId, peerRef.current);
      const flushed = settleGestureEnd(session);
      scheduleRender();
      dispatchBufferedEvents();
      notifyPuzzle2dPeersGestureEnded(peerScope, node.controllerId, node.surfaceId, flushed);
    };

    /** @emoji 🐁️ Wheel-zoom stays instant locally (WASM renders every tick via `scheduleRender`); only the React-visible camera echo and event flush are deferred until the gesture settles via `beginCameraInteraction`'s timeout. */
    const onWheel = (event: WheelEvent): void => {
      event.preventDefault();
      event.stopPropagation();
      const session = sessionRef.current;
      if (!session) return;
      beginCameraInteraction();
      const point = clientToLocal(event.clientX, event.clientY);
      const delta = event.deltaY * (event.deltaMode === WheelEvent.DOM_DELTA_LINE ? 16 : event.deltaMode === WheelEvent.DOM_DELTA_PAGE ? 400 : 1);
      session.wheelScreen(point.x, point.y, delta);
      scheduleRender();
      const cameraArgs = board2dCameraActionArgs(session.cameraJson());
      if (cameraArgs) pendingCameraDispatchRef.current = cameraArgs;
      drainIntoBuffer();
    };

    canvas.addEventListener("pointerdown", onPointerDown);
    canvas.addEventListener("pointerenter", onPointerEnter);
    canvas.addEventListener("pointerleave", onPointerLeave);
    window.addEventListener("pointermove", onPointerMove);
    window.addEventListener("pointerup", onPointerUp);
    container.addEventListener("wheel", onWheel, { passive: false });
    return () => {
      canvas.removeEventListener("pointerdown", onPointerDown);
      canvas.removeEventListener("pointerenter", onPointerEnter);
      canvas.removeEventListener("pointerleave", onPointerLeave);
      window.removeEventListener("pointermove", onPointerMove);
      window.removeEventListener("pointerup", onPointerUp);
      container.removeEventListener("wheel", onWheel);
    };
  }, [peerScope, beginCameraInteraction, dispatchBufferedEvents, drainAndMaybeFlush, drainIntoBuffer, node.controllerId, node.surfaceId, scheduleRender, scene?.interactive, settleGestureEnd]);
  //#endregion Pointer

  //#region Keyboard
  useEffect(() => {
    if (!scene?.interactive) return undefined;
    const isEditableTarget = (target: EventTarget | null): boolean => {
      if (!(target instanceof HTMLElement)) return false;
      return target.tagName === "INPUT" || target.tagName === "TEXTAREA" || target.isContentEditable;
    };
    const onKeyDown = (event: globalThis.KeyboardEvent): void => {
      if (!hoverActiveRef.current || isEditableTarget(event.target)) return;
      const session = sessionRef.current;
      if (!session) return;
      if (event.key === "Escape") {
        if (session.cancelAreaSelect?.()) {
          event.preventDefault();
          endPuzzle2dPeerGesture(peerScope, node.controllerId, node.surfaceId, peerRef.current);
          const flushed = settleGestureEnd(session);
          try {
            session.renderFrame();
          } catch {
            /* gpu not ready */
          }
          dispatchBufferedEvents();
          notifyPuzzle2dPeersGestureEnded(peerScope, node.controllerId, node.surfaceId, flushed);
        }
        return;
      }
      if (event.key === "Tab" && scene.activeUtility === "brush") {
        event.preventDefault();
        session.brushCycleCandidate?.(!event.shiftKey);
        try {
          session.renderFrame();
        } catch {
          /* gpu not ready */
        }
        flushBoardEvents();
        return;
      }
    };
    window.addEventListener("keydown", onKeyDown);
    return () => window.removeEventListener("keydown", onKeyDown);
  }, [peerScope, dispatch, dispatchBufferedEvents, flushBoardEvents, node.controllerId, node.surfaceId, scene?.activeUtility, scene?.interactive, scene?.selectionJson, settleGestureEnd]);
  //#endregion Keyboard

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
          } catch {
            /* gpu not ready */
          }
          dispatch("setSelection", { ids: selectionIds });
        }
        const hits = targets.map((target) => ({ domain: target.domain, id: target.id, label: target.label }));
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
          mapContextMenu,
          shellContextMenuFallback,
        );
        setContextMenu({ x: event.clientX, y: event.clientY, ...menu });
      })();
    },
    [dispatch, mapContextMenu, node.surfaceId, requestContextMenu, scene?.interactive, scene?.selectionJson, shellContextMenuFallback],
  );
  //#endregion ContextMenu

  //#region FixtureDropHandlers
  const onDragOver = useCallback(
    (event: DragEvent<HTMLDivElement>): void => {
      if (!scene?.interactive || !event.dataTransfer.types.includes(CATALOGUE_DRAG_MIME)) return;
      const session = sessionRef.current;
      if (!session?.setFixtureDropPreviewJson) return;
      const payload = parsePuzzle2dCatalogueDragPayload(getActiveCatalogueDragPayload());
      if (!payload) return;
      const rect = event.currentTarget.getBoundingClientRect();
      const world = puzzle2dScreenToWorld(session.cameraJson(), readContainerSize(), { x: event.clientX - rect.left, y: event.clientY - rect.top });
      if (!world) return;
      event.preventDefault();
      pushPuzzle2dFixtureDropPreview(peerScope, node.controllerId, puzzle2dFixtureDropPreviewJson(payload, world.x, world.y));
    },
    [peerScope, node.controllerId, readContainerSize, scene?.interactive],
  );

  const onDragLeave = useCallback((): void => {
    /* Keep the shared peer ghost while the pointer moves between panes of the same controller. */
  }, []);

  const onDrop = useCallback(
    (event: DragEvent<HTMLDivElement>): void => {
      if (!scene?.interactive) return;
      const encoded = event.dataTransfer.getData(CATALOGUE_DRAG_MIME) || getActiveCatalogueDragPayload();
      const payload = parsePuzzle2dCatalogueDragPayload(encoded);
      const session = sessionRef.current;
      pushPuzzle2dFixtureDropPreview(peerScope, node.controllerId, null);
      if (!payload || !session) return;
      event.preventDefault();
      const rect = event.currentTarget.getBoundingClientRect();
      const world = puzzle2dScreenToWorld(session.cameraJson(), readContainerSize(), { x: event.clientX - rect.left, y: event.clientY - rect.top });
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
    [peerScope, dispatch, node.controllerId, readContainerSize, scene?.interactive],
  );

  useEffect(() => {
    const onDragEnd = (): void => {
      queueMicrotask(() => {
        if (!getActiveCatalogueDragPayload()) pushPuzzle2dFixtureDropPreview(peerScope, node.controllerId, null);
      });
    };
    window.addEventListener("dragend", onDragEnd);
    return () => window.removeEventListener("dragend", onDragEnd);
  }, [peerScope, node.controllerId]);
  //#endregion FixtureDropHandlers

  if (sessionError) throw sessionError;
  if (!scene) return <div className="semio-board-2d-empty text-muted-foreground p-2 text-xs">{emptySceneLabel}</div>;

  return (
    <div
      ref={containerRef}
      className="semio-board-2d-host absolute inset-0 box-border min-h-0 min-w-0 overflow-hidden select-none"
      data-surface-id={node.surfaceId}
      style={{ touchAction: "none" }}
      onContextMenu={onContextMenu}
      onDragOver={onDragOver}
      onDragLeave={onDragLeave}
      onDrop={onDrop}
    >
      <canvas ref={canvasRef} className="absolute inset-0 block size-full touch-none outline-none focus:outline-none" />
      <ContextMenuController
        title={contextMenuTitleLabel}
        open={contextMenu != null}
        position={contextMenu ?? { x: 0, y: 0 }}
        items={contextMenu?.items ?? []}
        onOpenChange={(open) => {
          if (!open) setContextMenu(null);
        }}
      />
    </div>
  );
}
//#endregion Board2dHost
//#endregion 🔖️Board2dHost
