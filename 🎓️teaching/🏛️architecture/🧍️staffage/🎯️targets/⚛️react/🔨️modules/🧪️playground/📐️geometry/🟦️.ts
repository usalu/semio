/** 📐️ The geometry of the 2D playgrounds: how one position of the shared scene (x ahead, z sideways, the asset's left +) lands on a stage in pixels in each viewpoint, and how a pointer movement becomes a new position.
 *
 * Side elevation: paper right is ahead (x), the figures stand on the ground line. Front elevation: paper right is the asset's left (z), the line is `depthM` long and its middle is z = 0. Plan: paper right is ahead (x), paper up is the asset's left (z) and the stage is `lengthM` by `depthM` with z = 0 on its middle row; there is no ground line.
 * The 3D playground uses the same position through `placeNode` (`../🧊️model/🧩️compose/🟦️.ts`), so one placement is at one place in all three.
 *
 * @see ../🧱️compose/🟦️.ts — `linePosition`
 * @see ../../📍️placed/🟦️.tsx — the figure placed at a stage position
 */
import type { PlanarViewpoint } from "@teaching/staffage";
import type { Playground } from "../../💾️persistence/🟦️.ts";
import { snap } from "../🧱️compose/🟦️.ts";

/** 📏️ The padding around the ground, pixels. */
export const PAD_PX = 24;
/** 📏️ The room under the ground line for the scale, pixels. */
export const GROUND_GAP_PX = 34;
/** 📏️ The least height the figures are given, metres. */
export const MIN_HEIGHT_M = 2;

/** 🧭️ The extent of the ground in a viewpoint, metres: `acrossM` along the paper's horizontal, `downM` along its vertical (only the plan has one). */
export const extentM = (viewpoint: PlanarViewpoint, ground: Pick<Playground, "lengthM" | "depthM">): { readonly acrossM: number; readonly downM: number } => (viewpoint === "front" ? { acrossM: ground.depthM, downM: 0 } : viewpoint === "plan" ? { acrossM: ground.lengthM, downM: ground.depthM } : { acrossM: ground.lengthM, downM: 0 });

/** 🖼️ The stage in pixels: its size and the row of the ground line (in the plan the row of z = 0). */
export function stageGeometry(viewpoint: PlanarViewpoint, ground: Pick<Playground, "lengthM" | "depthM">, tallestM: number, metrePx: number): { readonly widthPx: number; readonly heightPx: number; readonly groundPx: number } {
  const { acrossM, downM } = extentM(viewpoint, ground);
  const widthPx = 2 * PAD_PX + acrossM * metrePx;
  if (viewpoint === "plan") return { widthPx, heightPx: Math.max(160, 2 * PAD_PX + downM * metrePx), groundPx: PAD_PX + (downM / 2) * metrePx };
  const heightPx = Math.max(160, tallestM * metrePx + GROUND_GAP_PX + 24);
  return { widthPx, heightPx, groundPx: heightPx - GROUND_GAP_PX };
}

/** 🔍️ The zoom that fits the ground into a width: pixels per millimetre of paper divided by the CSS pixels of a millimetre. */
export const fitZoomOf = (widthPx: number, acrossM: number, mmPerM: number, cssPxPerMm: number): number => Math.max(0.05, (widthPx - 2 * PAD_PX) / (acrossM * mmPerM * cssPxPerMm));

/** 🧲️ Where a pointer drag puts a placement: the side elevation moves it along x, the front elevation sideways, the plan both (paper up is +z). Positions snap to 5 cm and stay on the ground. */
export function dragPosition(viewpoint: PlanarViewpoint, start: { readonly xM: number; readonly zM: number }, deltaPx: { readonly x: number; readonly y: number }, metrePx: number, ground: Pick<Playground, "lengthM" | "depthM">): { readonly xM?: number; readonly zM?: number } {
  const grid = (value: number): number => Number(snap(value).toFixed(3));
  const clampX = (value: number): number => Math.min(Math.max(grid(value), 0), ground.lengthM);
  const clampZ = (value: number): number => Math.min(Math.max(grid(value), -ground.depthM / 2), ground.depthM / 2);
  if (viewpoint === "front") return { zM: clampZ(start.zM + deltaPx.x / metrePx) };
  if (viewpoint === "plan") return { xM: clampX(start.xM + deltaPx.x / metrePx), zM: clampZ(start.zM - deltaPx.y / metrePx) };
  return { xM: clampX(start.xM + deltaPx.x / metrePx) };
}

/** 🎯️ The box a placement can be grabbed by, relative to its ground point in pixels: standing figures by their height and width (front: depth), in the plan by their footprint. */
export function hitBox(viewpoint: PlanarViewpoint, dimensions: { readonly heightM: number; readonly widthM: number; readonly depthM: number } | undefined, metrePx: number): { readonly left: number; readonly top: number; readonly width: number; readonly height: number } {
  const height = Math.max(dimensions?.heightM ?? 1, 0.4) * metrePx;
  const length = Math.max(dimensions?.widthM ?? 0.5, 0.5) * metrePx;
  const depth = Math.max(dimensions?.depthM ?? 0.5, 0.5) * metrePx;
  if (viewpoint === "plan") return { left: -length / 2, top: -depth / 2, width: length, height: depth };
  const width = viewpoint === "front" ? depth : length;
  return { left: -width / 2, top: -height, width, height };
}
