/** 🗂️ Catalogue-drop payloads, relocate and volume-brush gestures, the catalogue-drop and selection preview stores and the catalogue drop ghost.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | World3dHost}, which re-exports every public name below so import specifiers stay unchanged.
 */

import { useLayoutEffect, Suspense } from "react";
import type { ThreeCamera } from "@semio-tech/ui-react";
import { type WorldMeshRecord } from "../🔭️scene-schema/🟦️.ts";
import { type MeshStylePalette, useThree } from "../🎨️mesh-render/🟦️.tsx";
import { GlbInstanceMesh } from "../🧩️glb-surface/🟦️.tsx";
import { GUMBALL_TRANSFORM_EPSILON } from "../🧮️gumball-math/🟦️.ts";
import { DemandInvalidateOnToken, raycastGroundPoint, worldGhostMeshUrl } from "../🎯️marquee-and-bridges/🟦️.tsx";

//#region 🔖️World3dHost
/** 🎞️ Demand-frameloop kick when the shared catalogue-drop preview store changes — HTML5 drags never fire pointermove, so the ghost must invalidate on dragover-driven store updates alone. */
export function CatalogueDropPreviewInvalidate({ preview }: { readonly preview: Puzzle3dCatalogueDropPreview | null }) {
  const invalidate = useThree((state) => state.invalidate);
  useLayoutEffect(() => {
    if (preview) invalidate();
  }, [invalidate, preview]);
  return null;
}

//#region CatalogueDrop
type Puzzle3dCatalogueDropPayload = {
  readonly objectKind: string;
  readonly meshUrl?: string;
};

type Puzzle3dCatalogueDropPreview = Puzzle3dCatalogueDropPayload & {
  readonly origin: readonly [number, number, number];
};

export function parsePuzzle3dCatalogueDragPayload(encoded: string | null | undefined): Puzzle3dCatalogueDropPayload | null {
  if (!encoded) return null;
  try {
    const parsed = JSON.parse(encoded) as Partial<Puzzle3dCatalogueDropPayload>;
    if (typeof parsed.objectKind !== "string" || !parsed.objectKind) return null;
    return {
      objectKind: parsed.objectKind,
      meshUrl: typeof parsed.meshUrl === "string" && parsed.meshUrl ? parsed.meshUrl : undefined,
    };
  } catch {
    return null;
  }
}

export function snapWorldPointToGrid(point: readonly [number, number, number], gridSnapEnabled: boolean, gridFactor: number): [number, number, number] {
  if (!gridSnapEnabled || gridFactor <= 0) return [point[0], point[1], point[2]];
  const snap = (value: number) => Math.round(value / gridFactor) * gridFactor;
  return [snap(point[0]), snap(point[1]), snap(point[2])];
}

//#region WorldRelocateGesture
/** 🚚️ Which object a Relocate-utility press grabs. A press with NOTHING selected grabs the object
 * under the pointer outright (direct manipulation, no select-then-drag ceremony); a press on a DIFFERENT
 * object than the selected one grabs nothing and falls through to the ordinary marquee/pick path, so the
 * user can re-select without the scene jumping under the cursor.
 * `selectedIds` is the leftover/object id list {@link world3dGumballSelectionArgsV1} already resolves.
 *
 * 🎯️ A press on EMPTY GROUND while a selection exists grabs the selection's anchor and treats the press
 * as the gesture's BASE POINT — the two-point move ("pick a base point, pick a target point") every CAD
 * tool spells this way, and exactly what {@link world3dRelocateDispatchArgsV1} already computes:
 * `origin + (to - from)`, a travel DELTA, never "put the object under the cursor". The base point
 * therefore never had to be on the object, and requiring it there made the utility unusable whenever the
 * grab point was occluded, off-screen or simply small on screen — measured on the live `:6013` shell
 * (wasm #53, ticket 26/09/02 wave B34, `📓️2026-09-12-wave-B34-interaction-scope.md` §2): a press at
 * `local={x:739,y:342}` resolved the ground point `[9.76,42.24,0]` with `ids=["seed-left-001"]` live and
 * nothing under the pointer, i.e. refused only because the press sat 265 px from the one object's
 * projected origin.
 * The cost is deliberate: inside this utility an empty-ground press is a base point rather than a
 * marquee, and a press without travel commits nothing ({@link GUMBALL_TRANSFORM_EPSILON}), so Escape and
 * `clearSelection` stay the way to drop a selection. */
export function world3dRelocateDragTargetV1(pressedId: string | null | undefined, selectedIds: readonly string[]): string | null {
  if (!pressedId) return selectedIds[0] ?? null;
  if (selectedIds.length === 0) return pressedId;
  return selectedIds.includes(pressedId) ? pressedId : null;
}

/** 🚚️ The ONE `worldRelocate` payload a finished Relocate drag commits — the absolute world origin
 * the guest's `world_relocate` arm decodes as `{objectId, position}`, never an incremental pose delta
 * (`gumballTransformDeltaBetweenPoses`'s `translateSelection` shape, which this verb deliberately is not).
 * The ground-plane travel `from → to` is added to the grabbed object's own origin and snapped exactly like
 * a catalogue drop; a drag that lands inside {@link GUMBALL_TRANSFORM_EPSILON} of where it started returns
 * `null` so a click-without-travel never writes a document edit. */
export function world3dRelocateDispatchArgsV1(
  objectId: string,
  origin: readonly [number, number, number],
  from: readonly [number, number, number],
  to: readonly [number, number, number],
  snap?: { readonly gridSnapEnabled: boolean; readonly gridFactor: number },
): { readonly objectId: string; readonly position: readonly [number, number, number] } | null {
  const moved: [number, number, number] = [origin[0] + (to[0] - from[0]), origin[1] + (to[1] - from[1]), origin[2] + (to[2] - from[2])];
  const position = snap ? snapWorldPointToGrid(moved, snap.gridSnapEnabled, snap.gridFactor) : moved;
  if (Math.abs(position[0] - origin[0]) < GUMBALL_TRANSFORM_EPSILON && Math.abs(position[1] - origin[1]) < GUMBALL_TRANSFORM_EPSILON && Math.abs(position[2] - origin[2]) < GUMBALL_TRANSFORM_EPSILON) {
    return null;
  }
  return { objectId, position };
}

export type World3dRelocateSession = {
  readonly objectId: string;
  readonly origin: readonly [number, number, number];
  readonly from: readonly [number, number, number];
  readonly objectKind: string;
  readonly meshUrl?: string;
};
//#endregion WorldRelocateGesture

//#region WorldVolumeBrushGesture
/** 🧊️ Grid-snapped ground origin a Volume Brush hover previews and an Alt+click commits, from the
 * raw `raycastGroundPoint` hit. Always snaps (the guest re-snaps by its own `gridSpacing` anyway), so a
 * placed target volume, a dropped catalogue object and a relocated object all land on one lattice.
 * `null` when the pointer ray misses the ground plane, which is also what keeps the ghost box off screen. */
export function world3dVolumeBrushOriginV1(ground: readonly [number, number, number] | null, gridFactor: number): [number, number, number] | null {
  return ground ? snapWorldPointToGrid(ground, true, gridFactor) : null;
}

/** 🧊️ True when a Volume Brush pointer press must COMMIT a target volume instead of opening a
 * marquee — the utility armed and Alt held. The gesture reads the ground through the host's own pointer
 * handlers (like relocate and catalogue drop) rather than an invisible r3f plane inside the canvas, which
 * the instance layer occluded on every click that landed on an object
 * (`📓️2026-09-11-wave-B1-battery-extension.md` §5 defect 12). */
export function world3dVolumeBrushCommits(volumeBrushMode: boolean, altKey: boolean): boolean {
  return volumeBrushMode && altKey;
}
//#endregion WorldVolumeBrushGesture

export function resolveCatalogueDropOrigin(clientX: number, clientY: number, hostRect: DOMRect, camera: ThreeCamera | null, gridSnapEnabled: boolean, gridFactor: number): [number, number, number] | null {
  if (!camera) return null;
  const hit = raycastGroundPoint(clientX, clientY, hostRect, camera);
  if (!hit) return null;
  return snapWorldPointToGrid(hit, gridSnapEnabled, gridFactor);
}

//#region WorldCatalogueDropPreviewStore
/** 👻️ Shared world-space catalogue-drop ghosts keyed by controller — every {@link World3dHost} pane of that controller subscribes so the preview is never clipped to the hovered window. */
const worldCatalogueDropPreviewByController = new Map<string, Puzzle3dCatalogueDropPreview>();
const worldCatalogueDropPreviewListeners = new Set<() => void>();
const worldCatalogueDropHostHitTests = new Map<string, { readonly controllerId: string; readonly hitTest: (clientX: number, clientY: number) => boolean }>();

function worldCatalogueDropPreviewsEqual(a: Puzzle3dCatalogueDropPreview | null | undefined, b: Puzzle3dCatalogueDropPreview | null | undefined): boolean {
  if (a === b) return true;
  if (!a || !b) return false;
  return a.objectKind === b.objectKind && a.meshUrl === b.meshUrl && a.origin[0] === b.origin[0] && a.origin[1] === b.origin[1] && a.origin[2] === b.origin[2];
}

function notifyWorldCatalogueDropPreviewListeners(): void {
  for (const listener of worldCatalogueDropPreviewListeners) listener();
}

/** 👻️ Subscribe to shared world-space catalogue-drop previews (all open World3d panes). */
export function subscribeWorldCatalogueDropPreview(listener: () => void): () => void {
  worldCatalogueDropPreviewListeners.add(listener);
  return () => {
    worldCatalogueDropPreviewListeners.delete(listener);
  };
}

/** 👻️ Current shared world-space catalogue-drop preview for `controllerId`, or `null` when no catalogue drag is live. */
export function getWorldCatalogueDropPreview(controllerId: string): Puzzle3dCatalogueDropPreview | null {
  return worldCatalogueDropPreviewByController.get(controllerId) ?? null;
}

/** 👻️ SSR snapshot for {@link useSyncExternalStore} — catalogue drops never hydrate with a live ghost. */
export function getWorldCatalogueDropPreviewServerSnapshot(_controllerId: string): Puzzle3dCatalogueDropPreview | null {
  return null;
}

/** 👻️ Publish a world-space catalogue-drop ghost visible in every World3d pane of `controllerId`. */
export function setWorldCatalogueDropPreview(controllerId: string, preview: Puzzle3dCatalogueDropPreview | null): void {
  const previous = worldCatalogueDropPreviewByController.get(controllerId) ?? null;
  if (worldCatalogueDropPreviewsEqual(previous, preview)) return;
  if (preview) worldCatalogueDropPreviewByController.set(controllerId, preview);
  else worldCatalogueDropPreviewByController.delete(controllerId);
  notifyWorldCatalogueDropPreviewListeners();
}

/** 👻️ Clears the shared catalogue-drop ghost across all World3d panes of `controllerId`. */
export function clearWorldCatalogueDropPreview(controllerId: string): void {
  setWorldCatalogueDropPreview(controllerId, null);
}

/** 🎯️ Registers a World3d pane's hit-test so shared clear logic can tell whether the pointer is over *any* pane of that controller. */
export function registerWorldCatalogueDropHost(controllerId: string, hostId: string, hitTest: (clientX: number, clientY: number) => boolean): () => void {
  const key = `${controllerId}\0${hostId}`;
  worldCatalogueDropHostHitTests.set(key, { controllerId, hitTest });
  return () => {
    worldCatalogueDropHostHitTests.delete(key);
  };
}

/** 🎯️ True when any registered World3d pane of `controllerId` contains the client point. */
export function worldCatalogueDropHostContainsPoint(controllerId: string, clientX: number, clientY: number): boolean {
  for (const entry of worldCatalogueDropHostHitTests.values()) {
    if (entry.controllerId !== controllerId) continue;
    if (entry.hitTest(clientX, clientY)) return true;
  }
  return false;
}
//#endregion WorldCatalogueDropPreviewStore

export function CatalogueDropGhost({ preview, meshes, palette }: { readonly preview: Puzzle3dCatalogueDropPreview; readonly meshes: readonly WorldMeshRecord[]; readonly palette: MeshStylePalette }) {
  const style = palette.highlighted;
  const url = worldGhostMeshUrl(preview, meshes);
  const position = preview.origin as [number, number, number];
  const invalidateToken = `${preview.objectKind}:${url ?? ""}:${position.join(",")}`;
  return (
    <group position={position} raycast={() => null}>
      <DemandInvalidateOnToken token={invalidateToken} />
      {url ? (
        <Suspense fallback={null}>
          <GlbInstanceMesh url={url} color={style.meshColor} emissive={style.meshColor} emissiveIntensity={0.6} opacity={0.88} borderColor={palette.neutral.lineColor} revision="highlighted" pickEnabled={false} />
        </Suspense>
      ) : (
        <mesh raycast={() => null}>
          <boxGeometry args={[1, 1, 1]} />
          <meshBasicMaterial color={style.meshColor} transparent opacity={0.42} depthWrite={false} />
        </mesh>
      )}
    </group>
  );
}
//#endregion CatalogueDrop

//#region WorldSelectionPreviewStore
export type WorldSelectionPreview = {
  readonly sourceId: string;
  readonly mergedComponentIds: readonly number[] | null;
  readonly mergedInstanceIds: readonly string[] | null;
};

/** 🎯️ Live selection previews keyed by controller so every sibling World3d pane paints the same in-progress marquee selection before the plugin round-trip commits it. */
const worldSelectionPreviewByController = new Map<string, WorldSelectionPreview>();
const worldSelectionPreviewListeners = new Set<() => void>();

function worldSelectionPreviewsEqual(a: WorldSelectionPreview | null | undefined, b: WorldSelectionPreview | null | undefined): boolean {
  if (a === b) return true;
  if (!a || !b || a.sourceId !== b.sourceId) return false;
  const idsEqual = <T,>(left: readonly T[] | null, right: readonly T[] | null) => left === right || (left !== null && right !== null && left.length === right.length && left.every((id, index) => id === right[index]));
  return idsEqual(a.mergedComponentIds, b.mergedComponentIds) && idsEqual(a.mergedInstanceIds, b.mergedInstanceIds);
}

function notifyWorldSelectionPreviewListeners(): void {
  for (const listener of worldSelectionPreviewListeners) listener();
}

/** 🎯️ Subscribes a World3d pane to live selection previews from sibling panes. */
export function subscribeWorldSelectionPreview(listener: () => void): () => void {
  worldSelectionPreviewListeners.add(listener);
  return () => {
    worldSelectionPreviewListeners.delete(listener);
  };
}

/** 🎯️ Returns the live selection preview for `controllerId`, or `null` outside an in-progress gesture. */
export function getWorldSelectionPreview(controllerId: string): WorldSelectionPreview | null {
  return worldSelectionPreviewByController.get(controllerId) ?? null;
}

/** 🎯️ SSR snapshot for {@link useSyncExternalStore}; selection gestures never hydrate in progress. */
export function getWorldSelectionPreviewServerSnapshot(_controllerId: string): WorldSelectionPreview | null {
  return null;
}

/** 🎯️ Publishes an in-progress marquee result to every World3d pane using the same controller. */
export function setWorldSelectionPreview(controllerId: string, preview: WorldSelectionPreview | null): void {
  const previous = worldSelectionPreviewByController.get(controllerId) ?? null;
  if (worldSelectionPreviewsEqual(previous, preview)) return;
  if (preview) worldSelectionPreviewByController.set(controllerId, preview);
  else worldSelectionPreviewByController.delete(controllerId);
  notifyWorldSelectionPreviewListeners();
}

/** 🎯️ Clears a preview only when `sourceId` still owns it, preventing an idle sibling pane from cancelling the active pane's gesture. */
export function clearWorldSelectionPreview(controllerId: string, sourceId?: string): void {
  const current = worldSelectionPreviewByController.get(controllerId);
  if (!current || (sourceId && current.sourceId !== sourceId)) return;
  setWorldSelectionPreview(controllerId, null);
}
//#endregion WorldSelectionPreviewStore
//#endregion 🔖️World3dHost
