/** 🔄️ Board2dHost session sync: the application of events, highlights and snapshots to the board session and the peer registry with its gesture ownership, live mirror pushes and drop preview pushes. */
import { type Board2dScene } from "@semio-tech/framework";
import { type Board2dWasmSession, type Board2dPeer, type BoardPeerScope } from "../../🪪️WasmSessionLoader/🟦️.tsx";
import { parseBoardCamera } from "../🔬️board-parsing/🟦️.ts";
import type { BoardLiveMirrorMutations } from "../📡️board-events/🟦️.ts";

//#region 🔖️Board2dHost
//#region Sync
/** 🔁️ Applies one scene sync through the demand handle: its calls invalidate the canvas, so every sync of one React
 * commit paints ONE coalesced frame instead of one synchronous full repaint each (ticket 26/09/23 F1: one scene update
 * repainted a board up to 16 times). */
export function applyToSession(session: Board2dWasmSession | null, action: (session: Board2dWasmSession) => void): void {
  if (!session) return;
  try {
    action(session);
  } catch {
    /* session not ready */
  }
}

/** 🔗️ Forwards what an open time-travel draft references (`Board2dScene.highlightedIdsJson`) to the board session — `[]`
 * when the scene carries none — exactly as the wgpu board sync does (`sync_board_engine`, `⚙️EngineCanvas/🎯️targets/🧊️wgpu`).
 * A snapshot re-parse keeps the engine's highlight, so only a changed id list is forwarded again. */
export function applyBoard2dHighlightedIds(session: Board2dWasmSession | null, scene: Pick<Board2dScene, "highlightedIdsJson">): void {
  applyToSession(session, (live) => live.setHighlightedIdsJson?.(scene.highlightedIdsJson ?? "[]"));
}

/** 🔁️ Re-parses the snapshot and silently re-applies selection/camera, since `loadBoardSnapshotJson` resets both to the snapshot's own defaults. */
export function applySnapshotToSession(session: Board2dWasmSession, scene: Board2dScene): boolean {
  const parsed = session.loadBoardSnapshotJson(scene.snapshotJson);
  if (!parsed) {
    console.error(`[board-2d] engine refused the snapshot (${scene.snapshotJson.length} chars) — nothing is painted until a snapshot parses`);
    (globalThis as { __semioBoard2dRefusedSnapshot?: string }).__semioBoard2dRefusedSnapshot = scene.snapshotJson;
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
  endBoardPeerGesture(scope, controllerId, surfaceId, peer);
}

export function board2dPeers(scope: BoardPeerScope, controllerId: string, excludeSurfaceId: string): readonly Board2dPeer[] {
  const peers = scope.peers.get(controllerId);
  if (!peers) return [];
  const result: Board2dPeer[] = [];
  for (const [surfaceId, peer] of peers) if (surfaceId !== excludeSurfaceId) result.push(peer);
  return result;
}

export function beginBoardPeerGesture(scope: BoardPeerScope, controllerId: string, surfaceId: string, peer: Board2dPeer | null): void {
  if (peer && scope.peers.get(controllerId)?.get(surfaceId) === peer) scope.gestures.set(controllerId, { surfaceId, peer });
}

export function endBoardPeerGesture(scope: BoardPeerScope, controllerId: string, surfaceId: string, peer: Board2dPeer | null): void {
  const owner = scope.gestures.get(controllerId);
  if (owner?.surfaceId === surfaceId && owner.peer === peer) scope.gestures.delete(controllerId);
}

/** 🙅️ True when a *different* pane owns the live gesture for this controller — the caller should defer applying an echoed scene. */
export function boardPeerOwnsGesture(scope: BoardPeerScope, controllerId: string, surfaceId: string): boolean {
  const owner = scope.gestures.get(controllerId);
  return owner !== undefined && owner.surfaceId !== surfaceId;
}

export function pushBoardLiveMirrorMutations(scope: BoardPeerScope, controllerId: string, surfaceId: string, mutations: BoardLiveMirrorMutations): void {
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

export function notifyBoardPeersGestureEnded(scope: BoardPeerScope, controllerId: string, surfaceId: string, flushed: boolean): void {
  for (const peer of board2dPeers(scope, controllerId, surfaceId)) {
    try {
      peer.onPeerGestureEnded(flushed);
    } catch {
      /* peer session not ready */
    }
  }
}

/** 👻️ Pushes a world-space catalogue catalogue-drop ghost into every pane of `controllerId` (including the source). */
export function pushBoardDropPreview(scope: BoardPeerScope, controllerId: string, previewJson: string | null): void {
  const peers = scope.peers.get(controllerId);
  if (!peers) return;
  for (const peer of peers.values()) {
    try {
      if (previewJson) peer.session.setDropPreviewJson?.(previewJson);
      else peer.session.clearDropPreview?.();
      peer.session.renderFrame?.();
    } catch {
      /* peer session not ready */
    }
  }
}
//#endregion PeerSync
//#endregion 🔖️Board2dHost
