// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/🖥️Board2dHost/component.tsx
/** @emoji 🧩️ `🖥️Board2dHost` — board-2d `ComponentSceneHost`: drives the board wasm session (fixture
 * sync, coalesced event drain/flush, marquee/pick pointer routing, catalogue fixture-drop preview),
 * plus the cross-pane live-mirror peer registry that keeps a triptych of panes on the same
 * `controllerId` in sync during a gesture without a plugin round trip. Reuses `World3dHost`'s
 * window-instance context and `🟦️Interpreter`'s surface context-menu plumbing. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { useCallback, useContext, useEffect, useLayoutEffect, useMemo, useRef, useState, type DragEvent, type KeyboardEvent, type MouseEvent } from "react";
import {
  useLabel,
  useShellScopeOptional,
  useCanvasAppearanceSync,
  ContextMenuController,
  registerIntroductionSurfaceResolver,
  windowElementId,
  type CanvasPickTarget,
  pickMostSpecificCanvasTarget,
  CATALOGUE_DRAG_MIME,
  getActiveCatalogueDragPayload,
} from "@semio-tech/ui-react";
import { STYLING_METRICS, syncSessionCanvasTheme } from "@semio-tech/ui-styling";
import {
  EMPTY_GESTURE_POINTERS,
  applyPinchToCamera,
  gestureIsMultiTouch,
  gesturePointerDown,
  gesturePointerMove,
  gesturePointerUp,
  pinchFrame,
  pinchStep,
  type ComponentSceneHostProps,
  type Board2dScene,
  type ContextMenuItemSpec,
  type GesturePointers,
  type PinchFrame,
} from "@semio-tech/framework";
import { type Board2dWasmSession, type Board2dPeer, type BoardPeerScope, BoardSessionFactoryContext, createBoardPeerScope } from "../🪪️WasmSessionLoader/🟦️.tsx";
import { useMapContextMenuSpecs } from "../🏛️ShellHost/🟦️.tsx";
import { createCoalescingActionDispatcher } from "../🛠️ShellHelpers/🟦️.tsx";
import { parseSelectionIds } from "../🖋️InkCanvasHost/🟦️.tsx";
// 🐢️ Direct element-to-element imports — `World3dHost`/`🟦️Interpreter` already landed in a prior batch.
import { WindowInstanceIdContext } from "../🌐️World3dHost/🟦️.tsx";
import { useToolRunTraceCursorEcho } from "../🌐️World3dHost/⏯️tool-run-trace/🟦️.tsx";
import { ToolRunTrace2dLayer } from "../📐️Canvas2dHost/⏯️tool-run-trace/🟦️.tsx";
import { board2dToolRunTracePathForShape, board2dToolRunTraceShapes } from "./⏯️tool-run-trace/🟦️.tsx";
import { useShellContextMenuFallback, openSurfaceContextMenu, type SurfaceContextMenuResult } from "../🗣️Interpreter/🟦️.tsx";
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
/** @emoji 🩺️ The document vitals this host publishes as `data-board-*` attributes on its container — node/edge counts
 * and every node's world position — so a headless probe (and the shell's own tests) can read what the guest last
 * painted without a guest round trip, the board twin of `World3dHost`'s `data-instances-json`. */
function board2dVitals(fixtureJson: string): { readonly nodes: number; readonly edges: number; readonly handles: number; readonly positionsJson: string } {
  try {
    const fixture = JSON.parse(fixtureJson) as { nodes?: { id?: string; x?: number; y?: number; handles?: unknown[] }[]; edges?: unknown[] };
    const positions: Record<string, [number, number]> = {};
    let handles = 0;
    for (const node of fixture.nodes ?? []) {
      if (typeof node.id === "string" && typeof node.x === "number" && typeof node.y === "number") positions[node.id] = [node.x, node.y];
      handles += node.handles?.length ?? 0;
    }
    return { nodes: fixture.nodes?.length ?? 0, edges: fixture.edges?.length ?? 0, handles, positionsJson: JSON.stringify(positions) };
  } catch {
    return { nodes: -1, edges: -1, handles: -1, positionsJson: "{}" };
  }
}

/** @emoji 🗂️ The kind ids this board knows, by the engine's own hover domain (`resolveElementKindHover`
 * answers `"node" | "handle" | "edge" | "wire"`). Read straight off the scene's kind catalogs, so the
 * host hard-codes no panel's id scheme and no owning plugin. */
export function board2dKindDomainById(glyphCatalogsJson: string): ReadonlyMap<string, string> {
  const out = new Map<string, string>();
  let catalogs: unknown;
  try {
    catalogs = JSON.parse(glyphCatalogsJson);
  } catch {
    return out;
  }
  if (typeof catalogs !== "object" || catalogs === null) return out;
  const slices: readonly (readonly [string, string])[] = [
    ["nodeKinds", "node"],
    ["handleKinds", "handle"],
    ["edgeKinds", "edge"],
    ["wireKinds", "wire"],
  ];
  for (const [key, domain] of slices) {
    const rows = (catalogs as Record<string, unknown>)[key];
    if (!Array.isArray(rows)) continue;
    for (const row of rows) {
      const id = (row as { readonly id?: unknown })?.id;
      if (typeof id === "string" && id.length > 0 && !out.has(id)) out.set(id, domain);
    }
  }
  return out;
}

/** @emoji 🖱️ Resolves a hovered chrome element's DOM id to `(domain, kindId)`. A catalogue row is named
 * `<sectionId>.<kindId>` (puzzle 2d: `puzzle2d-play-kinds.nodes.beam`), so the trailing dot segment — or
 * the whole id, for a bare-kind row like puzzle 3d's — is the candidate, and it counts only when this
 * board's OWN catalogs know it. That keeps the host generic: no panel id prefix, no plugin name. */
export function board2dKindHoverFromElementId(elementId: string | null | undefined, kindDomainById: ReadonlyMap<string, string>): { readonly domain: string; readonly kindId: string } | null {
  if (!elementId) return null;
  for (const candidate of [elementId.slice(elementId.lastIndexOf(".") + 1), elementId]) {
    const domain = kindDomainById.get(candidate);
    if (domain) return { domain, kindId: candidate };
  }
  return null;
}

/** @emoji 🩺️ The board's boot/sync verdict as one probe row — whether the last fixture parsed, how big it
 * was, why it was refused, how many drained rows are still waiting for a flush, and which guest scene
 * revision this pane last applied. The board twin of `World3dHost`'s `data-status-json`. */
export type Board2dStatus = {
  fixtureParsed: boolean | null;
  fixtureChars: number;
  refusalReason: string;
  pendingEvents: number;
  guestRevision: number;
};

export function board2dStatusJson(status: Board2dStatus): string {
  return JSON.stringify(status);
}

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
/** @emoji 🕹️ Reads `Board2dScene.transformFlags` (`{"move":boolean,"rotate":boolean}`); anything missing
 * or malformed leaves both handles on rather than silently disarming the gumball. */
export function parseBoard2dTransformFlags(encoded: string | null | undefined): { readonly move: boolean; readonly rotate: boolean } {
  if (!encoded) return { move: true, rotate: true };
  try {
    const parsed = JSON.parse(encoded) as { move?: unknown; rotate?: unknown };
    return { move: typeof parsed.move === "boolean" ? parsed.move : true, rotate: typeof parsed.rotate === "boolean" ? parsed.rotate : true };
  } catch {
    return { move: true, rotate: true };
  }
}
/** @emoji 🖍️ Reads `Board2dScene.areaBrushSize` (`{"width":number,"height":number}`). A missing or
 * malformed payload answers `0`, which the engine reads as "keep the extent you have" — an area brush
 * that silently collapsed to a zero-area rectangle would paint regions no fill placement can satisfy. */
export function parseBoard2dAreaBrushSize(encoded: string | null | undefined): { readonly width: number; readonly height: number } {
  if (!encoded) return { width: 0, height: 0 };
  try {
    const parsed = JSON.parse(encoded) as { width?: unknown; height?: unknown };
    return { width: typeof parsed.width === "number" && parsed.width > 0 ? parsed.width : 0, height: typeof parsed.height === "number" && parsed.height > 0 ? parsed.height : 0 };
  } catch {
    return { width: 0, height: 0 };
  }
}
/** @emoji 🐁️ Classifies every entity id the fixture carries into the `vortex`-domain granularity a
 * pick or hover reports it under — the client twin of the guest's `puzzle2d_selection_targets`. A
 * `node:handle` id nested under a node is a `handle`, an id in `edges` is an `edge`, everything else
 * (including an id the document does not carry yet) is a `node`, so a just-painted entity is never
 * dropped on the way to the framework. */
export function board2dGranularityById(fixtureJson: string): ReadonlyMap<string, string> {
  const byId = new Map<string, string>();
  try {
    const fixture = JSON.parse(fixtureJson) as { nodes?: { id?: unknown; handles?: { id?: unknown }[] }[]; edges?: { id?: unknown }[] };
    for (const node of fixture.nodes ?? []) {
      if (typeof node.id === "string") byId.set(node.id, "node");
      for (const handle of node.handles ?? []) if (typeof handle.id === "string") byId.set(handle.id, "handle");
    }
    for (const edge of fixture.edges ?? []) if (typeof edge.id === "string") byId.set(edge.id, "edge");
  } catch {
    /* a refused fixture classifies nothing — every id then reports as a node */
  }
  return byId;
}

/** @emoji 🐁️ The `interactionHover` wire shape — mirrors `world3dHoverActionArgs`, so one id and one
 * granularity is all a board pointermove costs. An empty `targets` clears the domain's hover. */
export function board2dHoverActionArgs(domainId: string, granularity: string, id: string | null | undefined) {
  return { domainId, channel: "pointer", targets: JSON.stringify(id ? [{ granularity, id }] : []) };
}

/** @emoji 🐁️ The LAST `hover` row of a drained batch — the engine's live answer; `undefined` when the
 * batch carries none (leave the current hover alone), `null` when the pointer left every entity. */
export function latestBoard2dHoverId(rows: readonly BoardEventRow[]): string | null | undefined {
  let hovered: string | null | undefined;
  for (const row of rows) {
    if (row.name !== "hover") continue;
    const id = (row.payload as { readonly id?: unknown } | undefined)?.id;
    hovered = typeof id === "string" && id.length > 0 ? id : null;
  }
  return hovered;
}

/** @emoji 💡️ One placement candidate row of the handle-suggestions popup. */
export type Board2dSuggestionCandidate = { readonly index: number; readonly nodeLabel: string; readonly handleLabel: string; readonly icon?: string; readonly color?: string };

/** @emoji 💡️ The open handle-suggestions popup the guest published, or `null` when this board has none.
 * `pending` means the slot has not resolved yet; an empty `candidates` on a resolved slot is the polite
 * refusal a document with no free handle (Nakagin) gives. */
export type Board2dSuggestionMenu = {
  readonly open: boolean;
  readonly x: number;
  readonly y: number;
  readonly windowId?: string;
  readonly handleId?: string;
  readonly hoveredIndex: number;
  readonly pending: boolean;
  readonly candidates: readonly Board2dSuggestionCandidate[];
};

export function parseBoard2dSuggestionMenu(encoded: string | null | undefined): Board2dSuggestionMenu | null {
  if (!encoded) return null;
  try {
    const parsed = JSON.parse(encoded) as Partial<Board2dSuggestionMenu>;
    if (parsed.open !== true) return null;
    const candidates = Array.isArray(parsed.candidates)
      ? parsed.candidates.filter((candidate): candidate is Board2dSuggestionCandidate => typeof candidate?.index === "number" && typeof candidate?.nodeLabel === "string")
      : [];
    return {
      open: true,
      x: typeof parsed.x === "number" ? parsed.x : 0,
      y: typeof parsed.y === "number" ? parsed.y : 0,
      windowId: typeof parsed.windowId === "string" && parsed.windowId.length > 0 ? parsed.windowId : undefined,
      handleId: typeof parsed.handleId === "string" && parsed.handleId.length > 0 ? parsed.handleId : undefined,
      hoveredIndex: typeof parsed.hoveredIndex === "number" ? parsed.hoveredIndex : 0,
      pending: parsed.pending === true,
      candidates,
    };
  } catch {
    return null;
  }
}

/** @emoji 🪟️ True when THIS pane owns the open popup — a menu naming no window is owned by whichever
 * pane renders it, exactly like `worldSuggestionMenuOwnsWindow`. */
export function board2dSuggestionMenuOwnsWindow(menu: Board2dSuggestionMenu | null, windowInstanceId: string | undefined): boolean {
  if (!menu?.open) return false;
  return !menu.windowId || menu.windowId === windowInstanceId;
}

/** @emoji 💡️ The popup's rows: hovering one PREVIEWS it (`hoverSuggestion`), clicking one places it
 * (`acceptSuggestion`). `closeOnSelect={false}` plus these two actions is what makes the preview a
 * "just looking" state distinct from the commit. */
export function board2dSuggestionMenuItems(menu: Board2dSuggestionMenu, labels: { readonly checkingPlacement: string; readonly noPlacement: string }): ContextMenuItemSpec[] {
  if (menu.pending) return [{ id: "pending", label: labels.checkingPlacement, disabled: true }];
  if (menu.candidates.length === 0) return [{ id: "empty", label: labels.noPlacement, disabled: true }];
  return menu.candidates.map((candidate) => ({
    id: `suggestion-${candidate.index}`,
    label: `${candidate.nodeLabel} · ${candidate.handleLabel}`,
    icon: candidate.icon ?? "circle-dot",
    checked: candidate.index === menu.hoveredIndex,
    action: "acceptSuggestion",
    args: { index: candidate.index, ...(menu.handleId ? { handleId: menu.handleId } : {}) },
    hoverAction: "hoverSuggestion",
    hoverArgs: { index: candidate.index, ...(menu.handleId ? { handleId: menu.handleId } : {}) },
  }));
}
//#endregion Parsing

//#region BoardEvents
// 🐁️ `hover` is the highest-frequency row the engine emits and it is NOT an `applyBoardEvents` payload:
// it travels on the framework's own `interactionHover` lane through {@link latestBoard2dHoverId}, so a
// pointermove never queues a retained board-events job. Listing it here is what keeps it out of the batch.
const PUZZLE2D_TRANSIENT_EVENT_NAMES = new Set(["preselect", "brushPreview", "linkCompatibleNodes", "linkTargetRing", "transformPreview", "hover"]);
const PUZZLE2D_FLUSH_NOW_EVENT_NAMES = new Set(["select", "preselectCancel", "brushCandidates", "brushPlace", "edgeCreate", "edgeDelete", "nodeDelete", "nodeRotate", "regionCreate", "regionMove", "regionResize"]);

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
      case "transformPreview":
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

/** @emoji 🤏️ The zoom bounds a board camera may never leave — the SAME `ZOOM_MIN`/`ZOOM_MAX` the Rust
 * engine's `clamp_zoom` applies (`♾️infinite/🖼️canvas/🦀️.rs`), read from the generated styling token
 * table so a pinch and a wheel can never disagree about the ceiling. */
export const BOARD_2D_ZOOM_BOUNDS = { min: STYLING_METRICS.camera.zoomMin, max: STYLING_METRICS.camera.zoomMax } as const;

/** @emoji 🤏️ Applies one two-finger step to the board camera carried by `cameraJson`, returning the pose
 * the host writes back silently. Pure: the whole pinch law is `🕹️interaction/👆️gesture`'s
 * {@link applyPinchToCamera} plus this surface's own bounds — nothing here is board-specific except
 * where the camera is read from. `null` when the camera JSON is unreadable. */
export function board2dPinchCamera(cameraJson: string, step: Parameters<typeof applyPinchToCamera>[1], containerSize: { readonly w: number; readonly h: number }): BoardCamera | null {
  const camera = parseBoardCamera(cameraJson);
  if (!camera) return null;
  return applyPinchToCamera(camera, step, containerSize, BOARD_2D_ZOOM_BOUNDS);
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
function applyFixtureToSession(session: Board2dWasmSession, scene: Board2dScene): boolean {
  const parsed = session.parseFixtureJson(scene.fixtureJson);
  if (!parsed) {
    console.error(`[board-2d] engine refused the fixture (${scene.fixtureJson.length} chars) — nothing is painted until a fixture parses`);
    (globalThis as { __semioBoard2dRefusedFixture?: string }).__semioBoard2dRefusedFixture = scene.fixtureJson;
  }
  session.setSelectionOptions?.(scene.selectionMethod, "replace", true, true, true);
  if (session.setSelectionIdsJsonSilent) session.setSelectionIdsJsonSilent(scene.selectionJson);
  else session.setSelectionIdsJson(scene.selectionJson);
  const camera = parseBoardCamera(scene.cameraJson);
  if (camera) {
    if (session.setCameraSilent) session.setCameraSilent(camera.x, camera.y, camera.zoom);
    else session.setCamera(camera.x, camera.y, camera.zoom);
  }
  return parsed;
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
  const gesturePointersRef = useRef<GesturePointers>(EMPTY_GESTURE_POINTERS);
  const pinchFrameRef = useRef<PinchFrame | null>(null);
  const pendingSelectionJsonRef = useRef<string | null>(null);
  const onPeerGestureEndedRef = useRef<(flushed: boolean) => void>(() => {});
  const boardStatusRef = useRef<Board2dStatus>({ fixtureParsed: null, fixtureChars: 0, refusalReason: "", pendingEvents: 0, guestRevision: 0 });
  const [localSelectionJson, setLocalSelectionJson] = useState<string | null>(null);
  const [sessionEpoch, setSessionEpoch] = useState(0);
  const [sessionError, setSessionError] = useState<Error | null>(null);
  const [contextMenu, setContextMenu] = useState<(SurfaceContextMenuResult & { readonly x: number; readonly y: number }) | null>(null);
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

  /** @emoji 🏁️ `dispatch`'s awaitable twin — the only shape {@link createCoalescingActionDispatcher}'s
   * "at most one round trip outstanding" gate can actually arm, since `dispatch` throws `onAction`'s
   * promise away (World3dHost wave B33: 72 hover turns enqueued by one 70-move storm, 11 settled). */
  const dispatchSettled = useCallback(
    (action: string, args?: Record<string, unknown>) => Promise.resolve(onAction({ controllerId: node.controllerId, action, args: { surfaceId: node.surfaceId, ...args } })),
    [node.controllerId, node.surfaceId, onAction],
  );

  /** @emoji 💡️ The popup is a per-window surface, so the verbs that open, preview, place or close it
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

  /** @emoji 🩺️ Republishes the live probe vitals straight onto the container, the way
   * `data-board-fixture-parsed` already is: a gumball drag and a marquee update these every frame, and
   * routing that through React state would re-render the whole pane on each pointer move. */
  const publishBoardVitals = useCallback((): void => {
    const container = containerRef.current;
    if (!container) return;
    const session = sessionRef.current;
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
      publishBoardVitals();
    });
  }, [publishBoardVitals]);

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

  //#region SuggestionMenu
  const suggestionMenu = useMemo(() => parseBoard2dSuggestionMenu(scene?.suggestionMenuJson), [scene?.suggestionMenuJson]);
  const suggestionMenuOwnsThisWindow = board2dSuggestionMenuOwnsWindow(suggestionMenu, windowInstanceId ?? undefined);
  const suggestionMenuOwnsThisWindowRef = useRef(false);
  suggestionMenuOwnsThisWindowRef.current = suggestionMenuOwnsThisWindow;
  const closeSuggestionMenu = useCallback(() => dispatchSuggestion("closeHandleSuggestions"), [dispatchSuggestion]);

  // 💡️ The provisional paint. The popup lists what the GUEST resolved, but the ghost on the canvas is
  // drawn by THIS pane's engine, so the open handle and the previewed index are mirrored into the local
  // slot — hovering a row moves the ghost this frame instead of a round trip later. The commit is never
  // mirrored: `acceptSuggestion` places through the guest so the placement is exactly one document edit.
  useEffect(() => {
    const session = sessionRef.current;
    if (!session) return;
    if (!suggestionMenuOwnsThisWindow || !suggestionMenu?.handleId) {
      applyToSession(session, (s) => s.brushCancelSlot?.());
      return;
    }
    applyToSession(session, (s) => {
      s.brushOpenSlot?.(suggestionMenu.handleId!);
      s.brushSetCandidateIndex?.(suggestionMenu.hoveredIndex);
    });
  }, [sessionEpoch, suggestionMenu?.handleId, suggestionMenu?.hoveredIndex, suggestionMenuOwnsThisWindow]);

  // 🪟️ A pane that does NOT render the popup still has to be able to dismiss it, otherwise an open menu
  // owned by a sibling gates this pane's ordinary context menu with no way out — the same hole
  // `World3dHost` closes with its own escape/outside-pointer path.
  useEffect(() => {
    if (!suggestionMenu?.open || suggestionMenuOwnsThisWindow) return undefined;
    const onKeyDown = (event: globalThis.KeyboardEvent): void => {
      if (event.key === "Escape") closeSuggestionMenu();
    };
    window.addEventListener("keydown", onKeyDown);
    return () => window.removeEventListener("keydown", onKeyDown);
  }, [closeSuggestionMenu, suggestionMenu?.open, suggestionMenuOwnsThisWindow]);
  //#endregion SuggestionMenu

  //#region Hover
  const granularityById = useMemo(() => board2dGranularityById(scene?.fixtureJson ?? ""), [scene?.fixtureJson]);
  const granularityByIdRef = useRef(granularityById);
  granularityByIdRef.current = granularityById;
  const interactionDomainId = scene?.domainId;
  /** @emoji 🩺️ Imperative mirror of the painted hover — no React state, so a pointermove never re-renders the host. */
  const publishHoverPaint = useCallback((id: string | null) => {
    const container = containerRef.current;
    if (!container) return;
    if (id === null) container.removeAttribute("data-board-hover-paint-id");
    else container.setAttribute("data-board-hover-paint-id", id);
  }, []);

  /** @emoji 🖱️ Transitive KIND hover: pointing at a catalogue kind row anywhere in the chrome paints every
   * node/handle/edge of that kind on EVERY board pane, the 2d twin of puzzle 3d's `hoveredKindId`. The
   * engine already owns the whole mechanism (`set_hovered_kind_silent` → `ids_matching_kind_hover` →
   * `hovered_style_kind`); nothing called it, which is why 2B had to leave the catalogue rows unbound.
   *
   * 🧲️ It listens on the document rather than taking a per-row action, for two reasons: a hover is not a
   * document verb (it must never cost a guest round trip or a repaint), and a per-row `UiValue` arg map
   * is exactly the tree-row cost that starves sibling panels. The row is recognised by ITS OWN kind id
   * matched against this board's catalogs, so no panel id scheme and no plugin name is hard-coded here. */
  const kindDomainById = useMemo(() => board2dKindDomainById(scene?.glyphCatalogsJson ?? ""), [scene?.glyphCatalogsJson]);
  const kindDomainByIdRef = useRef(kindDomainById);
  kindDomainByIdRef.current = kindDomainById;
  useEffect(() => {
    const container = containerRef.current;
    let painted: string | null = null;
    const paint = (hover: { readonly domain: string; readonly kindId: string } | null): void => {
      const next = hover ? `${hover.domain}:${hover.kindId}` : null;
      if (next === painted) return;
      painted = next;
      applyToSession(sessionRef.current, (session) => session.setHoveredKindSilent?.(hover?.domain ?? null, hover?.kindId ?? null));
      if (!container) return;
      if (next === null) container.removeAttribute("data-board-hovered-kind");
      else container.setAttribute("data-board-hovered-kind", next);
    };
    const onPointerOver = (event: globalThis.PointerEvent): void => {
      const target = event.target instanceof globalThis.Element ? event.target.closest<globalThis.HTMLElement>("[id]") : null;
      paint(board2dKindHoverFromElementId(target?.id, kindDomainByIdRef.current));
    };
    globalThis.document.addEventListener("pointerover", onPointerOver, true);
    return () => {
      globalThis.document.removeEventListener("pointerover", onPointerOver, true);
      paint(null);
    };
  }, [sessionEpoch]);
  /** @emoji 🐁️ At most one `interactionHover` round trip outstanding, the rest coalesced onto the latest
   * target — the board twin of `World3dHost`'s `dispatchInstanceHover`. An app declaring no interaction
   * domain publishes nothing rather than dispatching a verb no window kind owns. */
  const dispatchBoardHover = useMemo(
    () =>
      createCoalescingActionDispatcher<string | null>((id) => {
        if (!interactionDomainId) return undefined;
        return dispatchSettled("interactionHover", board2dHoverActionArgs(interactionDomainId, (id && granularityByIdRef.current.get(id)) || "node", id));
      }),
    [dispatchSettled, interactionDomainId],
  );
  //#endregion Hover

  //#region BoardEventFlush
  const drainIntoBuffer = useCallback((): void => {
    const session = sessionRef.current;
    if (!session) return;
    try {
      const json = session.drainEventsJson();
      if (!json || json === "[]") return;
      const rows = JSON.parse(json) as BoardEventRow[];
      pendingEventRowsRef.current.push(...rows);
      // 🐁️ Hover leaves the board-events batch here: the engine already painted it locally this frame,
      // and the framework copy travels on its own coalesced `interactionHover` lane so the outliner rows
      // and the sibling panes follow without a whole-surface republish per pointermove.
      const hovered = latestBoard2dHoverId(rows);
      if (hovered !== undefined) {
        publishHoverPaint(hovered);
        dispatchBoardHover(hovered);
      }
      const mutations = collectPuzzle2dLiveMirrorMutations(rows);
      pushPuzzle2dLiveMirrorMutations(peerScope, node.controllerId, node.surfaceId, mutations);
      // 🕹️ The engine's own `select` row is this pane's OPTIMISTIC selection; `scene.selectionJson`
      // stays the guest-confirmed one, so a probe can tell the two apart the way 3d's does.
      if (mutations.selectionIds) setLocalSelectionJson(JSON.stringify(mutations.selectionIds));
      boardStatusRef.current.pendingEvents = pendingEventRowsRef.current.length;
      publishBoardVitals();
    } catch {
      /* session not ready */
    }
  }, [dispatchBoardHover, peerScope, node.controllerId, node.surfaceId, publishBoardVitals, publishHoverPaint]);

  const dispatchBufferedEvents = useCallback((): void => {
    if (pendingEventRowsRef.current.length === 0) return;
    const { eventsJson } = coalesceBoard2dEvents(pendingEventRowsRef.current);
    pendingEventRowsRef.current = [];
    boardStatusRef.current.pendingEvents = 0;
    publishBoardVitals();
    if (eventsJson && eventsJson !== "[]") dispatch("applyBoardEvents", { eventsJson });
  }, [dispatch, publishBoardVitals]);

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

  /** @emoji 🩺️ Records one fixture-apply verdict into the status vitals and republishes them, so a
   * refused fixture names itself in the DOM instead of only in the console. */
  const recordFixtureVerdict = useCallback(
    (applied: Board2dScene, parsed: boolean): void => {
      boardStatusRef.current.fixtureParsed = parsed;
      boardStatusRef.current.fixtureChars = applied.fixtureJson.length;
      boardStatusRef.current.refusalReason = parsed ? "" : "engine refused the fixture";
      boardStatusRef.current.guestRevision += 1;
      containerRef.current?.setAttribute("data-board-fixture-parsed", String(parsed));
      publishBoardVitals();
    },
    [publishBoardVitals],
  );

  const applyPendingFixtureIfReady = useCallback(
    (session: Board2dWasmSession): void => {
      const pendingScene = pendingFixtureSceneRef.current;
      if (!pendingScene) return;
      if (session.defersDescriptorSyncFromJs?.() || cameraInteractionActiveRef.current || puzzle2dPeerOwnsGesture(peerScope, node.controllerId, node.surfaceId)) return;
      pendingFixtureSceneRef.current = null;
      applyToSession(session, (s) => recordFixtureVerdict(pendingScene, applyFixtureToSession(s, pendingScene)));
    },
    [peerScope, node.controllerId, node.surfaceId, recordFixtureVerdict],
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
    applyToSession(session, (s) => recordFixtureVerdict(scene, applyFixtureToSession(s, scene)));
    if (!bootSyncedRef.current) {
      bootSyncedRef.current = true;
      try {
        session.drainEventsJson();
      } catch {
        /* session not ready */
      }
    }
  }, [peerScope, recordFixtureVerdict, sessionEpoch, scene?.fixtureJson, node.controllerId, node.surfaceId]);

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
    setLocalSelectionJson(null);
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

  // 🐁️ The guest's hover echo paints the panes the pointer is NOT over — an outliner or catalogue row
  // hovered in a panel highlights the node on every canvas this way. It is deliberately NOT applied to
  // the pane under the pointer: that pane's engine already painted its own raycast this frame, and an
  // echo lagging a round trip behind would blink the live hover off and back on.
  useEffect(() => {
    if (!scene || hoverActiveRef.current) return;
    publishHoverPaint(scene.hoveredId ?? null);
    applyToSession(sessionRef.current, (session) => session.setHoveredIdSilent?.(scene.hoveredId ?? null));
  }, [publishHoverPaint, sessionEpoch, scene?.hoveredId]);

  useEffect(() => {
    if (!scene) return;
    applyToSession(sessionRef.current, (session) => session.setActiveUtility?.(scene.activeUtility ?? "select"));
  }, [sessionEpoch, scene?.activeUtility]);

  useEffect(() => {
    if (!scene || !board2dHostShellScope) return;
    const updateOptions = () => {
      const mode = board2dHostShellScope.selection.get();
      // 🎯️ The owning app's selectable-kind filter; an older scene omits the flags and reads as on.
      // Engine argument order is (nodes, edges, handles).
      applyToSession(sessionRef.current, (session) =>
        session.setSelectionOptions?.(scene.selectionMethod, mode, scene.selectableNodes !== false, scene.selectableEdges !== false, scene.selectableHandles !== false),
      );
    };
    updateOptions();
    return board2dHostShellScope.selection.subscribe(updateOptions);
  }, [sessionEpoch, scene?.selectionMethod, scene?.selectableNodes, scene?.selectableEdges, scene?.selectableHandles, board2dHostShellScope]);

  useEffect(() => {
    if (!scene) return;
    applyToSession(sessionRef.current, (session) => session.setGridVisible?.(scene.gridVisible !== false));
  }, [sessionEpoch, scene?.gridVisible]);

  useEffect(() => {
    if (!scene) return;
    applyToSession(sessionRef.current, (session) => session.setGridSnapEnabled?.(scene.gridSnapEnabled));
  }, [sessionEpoch, scene?.gridSnapEnabled]);

  // 🕹️ `setTransformGumballFlag` composes which gumball handles the select utility offers; a scene
  // that declares none leaves the engine's own default (move + rotate, never scale).
  useEffect(() => {
    if (!scene) return;
    const flags = parseBoard2dTransformFlags(scene.transformFlags);
    applyToSession(sessionRef.current, (session) => session.setTransformFlags?.(flags.move, flags.rotate));
    publishBoardVitals();
  }, [publishBoardVitals, sessionEpoch, scene?.transformFlags]);

  useEffect(() => {
    if (!scene) return;
    applyToSession(sessionRef.current, (session) => session.setGridFactor?.(scene.gridFactor));
  }, [sessionEpoch, scene?.gridFactor]);

  // 🖍️ What ONE area-brush click paints; a click-drag states its own rectangle and ignores this.
  useEffect(() => {
    if (!scene) return;
    const size = parseBoard2dAreaBrushSize(scene.areaBrushSize);
    applyToSession(sessionRef.current, (session) => session.setAreaBrushExtent?.(size.width, size.height));
  }, [sessionEpoch, scene?.areaBrushSize]);

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

  // 🩺️ `data-board-interaction-json`/`data-board-transform-json` are written ONLY here and from
  // `scheduleRender`, never from JSX — a React re-render must not stamp a stale frame over the live one.
  useEffect(() => {
    publishBoardVitals();
  });

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

    /** @emoji 🤏️ Abandons the single-pointer lane the moment a SECOND contact lands: the marquee/pick
     * gesture the first finger started must not keep growing under a pinch, and its pointer capture must
     * not swallow the second finger's moves. */
    const yieldToPinch = (session: Board2dWasmSession, point: { x: number; y: number }): void => {
      session.cancelAreaSelect?.();
      session.pointerUpScreen(point.x, point.y, false, false, false);
      for (const tracked of gesturePointersRef.current.pointers) {
        if (canvas.hasPointerCapture?.(tracked.pointerId)) canvas.releasePointerCapture(tracked.pointerId);
      }
    };

    const onPointerDown = (event: PointerEvent): void => {
      event.stopPropagation();
      const session = sessionRef.current;
      if (!session) return;
      const point = clientToLocal(event.clientX, event.clientY);
      gesturePointersRef.current = gesturePointerDown(gesturePointersRef.current, { pointerId: event.pointerId, x: point.x, y: point.y });
      if (gestureIsMultiTouch(gesturePointersRef.current)) {
        yieldToPinch(session, point);
        pinchFrameRef.current = pinchFrame(gesturePointersRef.current);
        beginCameraInteraction();
        return;
      }
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
      gesturePointersRef.current = gesturePointerMove(gesturePointersRef.current, { pointerId: event.pointerId, x: point.x, y: point.y });
      if (gestureIsMultiTouch(gesturePointersRef.current)) {
        const next = pinchFrame(gesturePointersRef.current);
        const previous = pinchFrameRef.current;
        pinchFrameRef.current = next;
        if (!next || !previous) return;
        beginCameraInteraction();
        const camera = board2dPinchCamera(session.cameraJson(), pinchStep(previous, next), readContainerSize());
        if (!camera) return;
        if (session.setCameraSilent) session.setCameraSilent(camera.x, camera.y, camera.zoom);
        else session.setCamera(camera.x, camera.y, camera.zoom);
        pendingCameraDispatchRef.current = { camera };
        scheduleRender();
        return;
      }
      session.pointerMoveScreen(point.x, point.y, event.shiftKey, event.metaKey || event.ctrlKey, event.altKey);
      scheduleRender();
      drainAndMaybeFlush();
    };

    const onPointerUp = (event: PointerEvent): void => {
      const session = sessionRef.current;
      if (!session) return;
      const wasMultiTouch = gestureIsMultiTouch(gesturePointersRef.current);
      gesturePointersRef.current = gesturePointerUp(gesturePointersRef.current, event.pointerId);
      // 🤏️ Re-seed from the contacts that REMAIN: a pinch ending one finger at a time must not diff the
      // next frame against a frame the lifted finger was still in, which would snap the camera.
      pinchFrameRef.current = pinchFrame(gesturePointersRef.current);
      if (canvas.hasPointerCapture?.(event.pointerId)) canvas.releasePointerCapture(event.pointerId);
      if (wasMultiTouch) return;
      const point = clientToLocal(event.clientX, event.clientY);
      session.pointerUpScreen(point.x, point.y, event.shiftKey, event.metaKey || event.ctrlKey, event.altKey);
      endPuzzle2dPeerGesture(peerScope, node.controllerId, node.surfaceId, peerRef.current);
      const flushed = settleGestureEnd(session);
      scheduleRender();
      dispatchBufferedEvents();
      notifyPuzzle2dPeersGestureEnded(peerScope, node.controllerId, node.surfaceId, flushed);
    };

    const onPointerCancel = (event: PointerEvent): void => {
      gesturePointersRef.current = gesturePointerUp(gesturePointersRef.current, event.pointerId);
      pinchFrameRef.current = pinchFrame(gesturePointersRef.current);
      if (canvas.hasPointerCapture?.(event.pointerId)) canvas.releasePointerCapture(event.pointerId);
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
    window.addEventListener("pointercancel", onPointerCancel);
    container.addEventListener("wheel", onWheel, { passive: false });
    return () => {
      canvas.removeEventListener("pointerdown", onPointerDown);
      canvas.removeEventListener("pointerenter", onPointerEnter);
      canvas.removeEventListener("pointerleave", onPointerLeave);
      window.removeEventListener("pointermove", onPointerMove);
      window.removeEventListener("pointerup", onPointerUp);
      window.removeEventListener("pointercancel", onPointerCancel);
      container.removeEventListener("wheel", onWheel);
    };
  }, [peerScope, beginCameraInteraction, dispatch, dispatchBufferedEvents, drainAndMaybeFlush, drainIntoBuffer, node.controllerId, node.surfaceId, readContainerSize, scheduleRender, scene?.activeUtility, scene?.interactive, settleGestureEnd]);
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
    };
    window.addEventListener("keydown", onKeyDown);
    return () => window.removeEventListener("keydown", onKeyDown);
  }, [peerScope, dispatch, dispatchBufferedEvents, node.controllerId, node.surfaceId, scene?.interactive, scene?.selectionJson, settleGestureEnd]);

  // 🔁️ `tab` walks the open slot's candidates forward, `shift+tab` back — the same two chords the app
  // binds to `cycleBrushCandidate`/`cycleBrushCandidateBack`. This listener runs in the CAPTURE phase so
  // its `preventDefault` reaches the shell's keybinding dispatcher (which bails on `defaultPrevented`)
  // BEFORE it fires: the slot must advance exactly one step, not two. The local engine cycles first so
  // the ghost moves this frame, and the flushed `brushCandidates` row carries the new index to the guest.
  // It is armed by the armed brush OR by an open popup, which is why the picker is reachable without the tool.
  useEffect(() => {
    if (!scene?.interactive) return undefined;
    const armed = scene.activeUtility === "brush";
    const onTabCapture = (event: globalThis.KeyboardEvent): void => {
      if (event.key !== "Tab" || !(armed ? hoverActiveRef.current : suggestionMenuOwnsThisWindowRef.current)) return;
      const session = sessionRef.current;
      if (!session) return;
      event.preventDefault();
      session.brushCycleCandidate?.(!event.shiftKey);
      try {
        session.renderFrame();
      } catch {
        /* gpu not ready */
      }
      flushBoardEvents();
    };
    window.addEventListener("keydown", onTabCapture, true);
    return () => window.removeEventListener("keydown", onTabCapture, true);
  }, [flushBoardEvents, scene?.activeUtility, scene?.interactive]);
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
          // 🕹️ The guest owns no `setSelection` verb: a right-click pick travels as the engine's own
          // `select` board event, which `applyBoardEvents` turns into the framework selection write.
          dispatch("applyBoardEvents", { eventsJson: JSON.stringify([{ name: "select", payload: { ids: selectionIds, exitHighlightIds: [] } }]) });
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

  const toolRunTraceCamera = useMemo(() => parseBoardCamera(scene?.cameraJson ?? "") ?? { x: 0, y: 0, zoom: 1 }, [scene?.cameraJson]);
  const boardVitals = useMemo(() => board2dVitals(scene?.fixtureJson ?? ""), [scene?.fixtureJson]);
  const toolRunTracePathForShape = useMemo(() => board2dToolRunTracePathForShape(board2dToolRunTraceShapes(scene?.glyphCatalogsJson ?? "")), [scene?.glyphCatalogsJson]);
  const onToolRunTraceCursor = useToolRunTraceCursorEcho(windowInstanceId);

  if (sessionError) throw sessionError;
  if (!scene) return <div className="semio-board-2d-empty text-muted-foreground p-2 text-xs">{emptySceneLabel}</div>;

  return (
    <div
      ref={containerRef}
      className="semio-board-2d-host absolute inset-0 box-border min-h-0 min-w-0 overflow-hidden select-none"
      data-surface-id={node.surfaceId}
      data-window-instance-id={windowInstanceId ?? ""}
      data-board-nodes={boardVitals.nodes}
      data-board-edges={boardVitals.edges}
      data-board-handles={boardVitals.handles}
      data-board-positions-json={boardVitals.positionsJson}
      data-board-selection-json={localSelectionJson ?? scene.selectionJson}
      data-board-guest-selection-json={scene.selectionJson}
      data-board-camera-json={scene.cameraJson}
      data-board-hovered-id={scene.hoveredId ?? ""}
      data-board-active-utility={scene.activeUtility ?? ""}
      data-board-suggestion-menu-json={scene.suggestionMenuJson ?? ""}
      data-board-status-json={board2dStatusJson(boardStatusRef.current)}
      style={{ touchAction: "none" }}
      onContextMenu={onContextMenu}
      onDragOver={onDragOver}
      onDragLeave={onDragLeave}
      onDrop={onDrop}
    >
      <canvas ref={canvasRef} className="absolute inset-0 block size-full touch-none outline-none focus:outline-none" />
      <ToolRunTrace2dLayer lane={scene.toolRunTrace} camera={toolRunTraceCamera} pathForShape={toolRunTracePathForShape} onCursor={onToolRunTraceCursor} />
      <ContextMenuController
        title={contextMenuTitleLabel}
        open={contextMenu != null && !suggestionMenuOwnsThisWindow}
        position={contextMenu ?? { x: 0, y: 0 }}
        items={contextMenu?.items ?? []}
        onOpenChange={(open) => {
          if (!open) setContextMenu(null);
        }}
      />
      {suggestionMenuOwnsThisWindow && suggestionMenu ? (
        <ContextMenuController
          title={suggestionMenuTitleLabel}
          open
          closeOnSelect={false}
          position={{ x: suggestionMenu.x, y: suggestionMenu.y }}
          items={mapSuggestionMenu(board2dSuggestionMenuItems(suggestionMenu, { checkingPlacement: suggestionCheckingPlacementLabel, noPlacement: suggestionNoPlacementLabel }))}
          onOpenChange={(open) => {
            if (!open) closeSuggestionMenu();
          }}
        />
      ) : null}
    </div>
  );
}
//#endregion Board2dHost
//#endregion 🔖️Board2dHost
