/** 🦥️ The viewpoints whose drawing is a chunk of its own: the axonometric depiction (volume solids, kernel tessellation, hidden lines) is loaded the first time a figure asks for it, so the planar pages never load it.
 *
 * @see ../🟦️.ts — `sceneOf`, which draws synchronously and needs the viewpoint ready
 * @see ./⚛️hook/🟦️.tsx — the hook and the gate of the React components */
import { loadAxonometric } from "@teaching/staffage";

let axonometricLoaded = false;

/** ✅️ Whether `sceneOf` can draw the viewpoint now: the planar viewpoints always, the axonometric one after {@link ensureViewpoint}. */
export const viewpointReady = (viewpoint: string): boolean => viewpoint !== "axonometric" || axonometricLoaded;

/** 📥️ Loads what drawing the viewpoint needs, once; resolves at once for the planar viewpoints and the interactive model. */
export async function ensureViewpoint(viewpoint: string): Promise<void> {
  if (viewpoint !== "axonometric") return;
  await loadAxonometric();
  axonometricLoaded = true;
}
