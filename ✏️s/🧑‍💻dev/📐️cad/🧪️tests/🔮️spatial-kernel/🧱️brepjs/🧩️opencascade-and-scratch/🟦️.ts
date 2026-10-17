/// <reference types="vite/client" />
/// <reference types="vitest/importMeta" />
/** 🧱️ Opencascade and scratch: test support exporting `isBrepjsTestRun`, `openCascadeWasmNeedsNodeResolve`, `createOpenCascadeLocateFile`, `MESH_TRANSFER_CACHE_MAX` and more. */
/** 🧩️ The OpenCascade wasm locator of the brepjs kernel, the quantised vector key and the extrusion direction.
 *
 * @see ../🟦️.ts — the brepjs kernel module, kept ONLY as a vitest differential oracle
 */
import { ownedOpenCascadeWasmBundledUrl, resolveOwnedOpenCascadeWasmFileUrl } from "@semio-tech/cad-engine-ts/brepjs";
import { type Vec3 } from "@semio-tech/framework-3d-js";

// #region 🧩️OpenCascade
export const isBrepjsTestRun = import.meta.env.VITEST === "true" || import.meta.env.MODE === "test" || Boolean(import.meta.vitest);

export const openCascadeWasmNeedsNodeResolve =
  (import.meta.env.VITEST || import.meta.env.MODE === "test") && (ownedOpenCascadeWasmBundledUrl.includes("@fs") || ownedOpenCascadeWasmBundledUrl.includes("node_modules/brepjs-opencascade"));

/** 📂️ Builds `locateFile` for OpenCascade: Vite asset URL in browser, `createRequire` on disk in Vitest. */
export async function createOpenCascadeLocateFile(): Promise<(path: string) => string> {
  if (!openCascadeWasmNeedsNodeResolve) {
    return (path) => (path === "brepjs_single.wasm" ? ownedOpenCascadeWasmBundledUrl : path);
  }
  const wasmFile = await resolveOwnedOpenCascadeWasmFileUrl();
  return (path) => (path === "brepjs_single.wasm" ? wasmFile : path);
}

// #endregion 🧩️OpenCascade
// #region ♻️BrepjsScratch
export const MESH_TRANSFER_CACHE_MAX = 64;

function vec3KeyQuantized(x: number, y: number, z: number, invTol: number): number {
  const ix = Math.round(x * invTol);
  const iy = Math.round(y * invTol);
  const iz = Math.round(z * invTol);
  return ((ix * 73856093) ^ (iy * 19349663) ^ (iz * 83492791)) >>> 0;
}

export function extrudeDirection(direction: Vec3, distance: number): Vec3 {
  const len = Math.hypot(direction[0], direction[1], direction[2]);
  const dist = Math.abs(distance) || len || 1e-6;
  if (len > 1e-12) {
    const s = dist / len;
    return [direction[0] * s, direction[1] * s, direction[2] * s];
  } else {
    return [0, 0, dist];
  }
}
// #endregion ♻️BrepjsScratch
