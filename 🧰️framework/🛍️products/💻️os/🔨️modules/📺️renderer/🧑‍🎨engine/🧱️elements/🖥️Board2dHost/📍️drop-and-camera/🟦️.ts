/** 📍️ Board2dHost drop and camera: the entity flag, the catalogue drop preview json and drop host registry, the screen and world conversion and the pinch camera. */
import { STYLING_METRICS } from "@semio-tech/ui-styling";
import { applyPinchToCamera } from "@semio-tech/framework";
import { type BoardCamera, type BoardDropPayload, parseBoardCamera } from "../🔬️board-parsing/🟦️.ts";

//#region 🔖️Board2dHost
//#region SelectionMenu
function boardEntityFlag(entity: Record<string, unknown> | undefined, key: "hidden" | "locked"): boolean {
  return Boolean(entity && entity[key] === true);
}

/** 🖱️ Right-click menu for the current selection: Hide/Show, Lock/Unlock, Duplicate, Select same kind, Zoom to selection, Delete — mirrors the premigration canvas context menu. */
//#endregion SelectionMenu
//#region DropPreview
const BOARD_CATALOGUE_DROP_FALLBACK_RADIUS = 20;
const BOARD_CATALOGUE_DROP_FALLBACK_EXTENT = 40;

function finitePositive(value: number | undefined): number | undefined {
  return typeof value === "number" && Number.isFinite(value) && value > 0 ? value : undefined;
}

/** 👻️ Builds a world-space catalogue-drop preview so every peer pane shares the same ghost (screen coords would desync under different cameras). A part row that names only its kind still paints: a circle falls back to the 5d default radius and a rectangle to that diameter. */
export function boardDropPreviewJson(payload: BoardDropPayload, worldX: number, worldY: number): string {
  const rectangle = payload.shape === "rectangle";
  return JSON.stringify({
    nodeKind: payload.kindId,
    x: worldX,
    y: worldY,
    shape: rectangle ? "rectangle" : "circle",
    radius: finitePositive(payload.radius) ?? BOARD_CATALOGUE_DROP_FALLBACK_RADIUS,
    width: finitePositive(payload.width) ?? (rectangle ? BOARD_CATALOGUE_DROP_FALLBACK_EXTENT : undefined),
    height: finitePositive(payload.height) ?? (rectangle ? BOARD_CATALOGUE_DROP_FALLBACK_EXTENT : undefined),
    iconKind: payload.iconKind,
  });
}

export type BoardCatalogueDropRect = { readonly left: number; readonly top: number; readonly right: number; readonly bottom: number };

/** 🎯️ True when a client point sits inside a board pane's viewport box. */
export function boardCatalogueDropPointOverRect(clientX: number, clientY: number, rect: BoardCatalogueDropRect): boolean {
  return clientX >= rect.left && clientX <= rect.right && clientY >= rect.top && clientY <= rect.bottom;
}

const boardCatalogueDropHostHitTests = new Map<string, { readonly controllerId: string; readonly hitTest: (clientX: number, clientY: number) => boolean }>();

/** 🎯️ Registers a board pane's hit-test so one pane does not clear the shared ghost while the pointer is over a sibling pane of the same controller. */
export function registerBoardCatalogueDropHost(controllerId: string, hostId: string, hitTest: (clientX: number, clientY: number) => boolean): () => void {
  const key = `${controllerId}\0${hostId}`;
  boardCatalogueDropHostHitTests.set(key, { controllerId, hitTest });
  return () => {
    boardCatalogueDropHostHitTests.delete(key);
  };
}

/** 🎯️ True when any registered board pane of `controllerId` contains the client point. */
export function boardCatalogueDropHostContainsPoint(controllerId: string, clientX: number, clientY: number): boolean {
  for (const entry of boardCatalogueDropHostHitTests.values()) {
    if (entry.controllerId !== controllerId) continue;
    if (entry.hitTest(clientX, clientY)) return true;
  }
  return false;
}

/** 📐️ Inverse of the canonical `screenX = (worldX - camera.x) * zoom + width / 2` transform shared across board renderers. */
export function boardScreenToWorld(cameraJson: string, containerSize: { readonly w: number; readonly h: number }, screen: { readonly x: number; readonly y: number }): { readonly x: number; readonly y: number } | null {
  const camera = parseBoardCamera(cameraJson);
  if (!camera) return null;
  const zoom = camera.zoom || 1;
  return {
    x: camera.x + (screen.x - containerSize.w / 2) / zoom,
    y: camera.y + (screen.y - containerSize.h / 2) / zoom,
  };
}

/** 📐️ The canonical `screenX = (worldX - camera.x) * zoom + width / 2` transform shared across
 * board renderers — the missing inverse of {@link boardScreenToWorld}, needed for demonstration
 * targeting (a world point/entity → the viewport pixel a ghost cursor animates to). */
export function boardWorldToScreen(cameraJson: string, containerSize: { readonly w: number; readonly h: number }, world: { readonly x: number; readonly y: number }): { readonly x: number; readonly y: number } | null {
  const camera = parseBoardCamera(cameraJson);
  if (!camera) return null;
  const zoom = camera.zoom || 1;
  return {
    x: (world.x - camera.x) * zoom + containerSize.w / 2,
    y: (world.y - camera.y) * zoom + containerSize.h / 2,
  };
}

/** 🤏️ The zoom bounds a board camera may never leave — the SAME `ZOOM_MIN`/`ZOOM_MAX` the Rust
 * engine's `clamp_zoom` applies (`🧰️framework/🔨️modules/🖼️canvas/🦀️.rs`), read from the generated styling token
 * table so a pinch and a wheel can never disagree about the ceiling. */
export const BOARD_2D_ZOOM_BOUNDS = { min: STYLING_METRICS.camera.zoomMin, max: STYLING_METRICS.camera.zoomMax } as const;

/** 🤏️ Applies one two-finger step to the board camera carried by `cameraJson`, returning the pose
 * the host writes back silently. Pure: the whole pinch law is `🕹️interaction/👆️gesture`'s
 * {@link applyPinchToCamera} plus this surface's own bounds — nothing here is board-specific except
 * where the camera is read from. `null` when the camera JSON is unreadable. */
export function board2dPinchCamera(cameraJson: string, step: Parameters<typeof applyPinchToCamera>[1], containerSize: { readonly w: number; readonly h: number }): BoardCamera | null {
  const camera = parseBoardCamera(cameraJson);
  if (!camera) return null;
  return applyPinchToCamera(camera, step, containerSize, BOARD_2D_ZOOM_BOUNDS);
}
//#endregion DropPreview
//#endregion 🔖️Board2dHost
