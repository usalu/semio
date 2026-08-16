/** 🧊️ Puzzle 5D editor — World3d window: typed twin of `🦀️component.rs`'s view-model. Mirrors the
 * pane's `render(envelope: &Puzzle5dScene, precompute: &Puzzle5dPrecomputeSession)` boundary — the 3D
 * world-scene projection plus the live brush/fill precompute session state a mutation-capable
 * surface carries (absent entirely from the viewer's read-only twin, see `👁️viewer/…/🟦️component.ts`). */

/** ✏️ The World3d window's typed view-model — mirrors the Rust `render()` boundary's inputs. */
export interface Puzzle5dWorld3dViewModel {
  windowKindId: "puzzle5d-3d";
  bodyKey: "puzzle.5d.play.3d";
  surfaceId: "puzzle.5d.play.3d";
}

export const PUZZLE5D_WORLD3D_WINDOW_KIND_ID = "puzzle5d-3d" as const;
export const PUZZLE5D_WORLD3D_BODY_KEY = "puzzle.5d.play.3d" as const;
export const PUZZLE5D_WORLD3D_SURFACE_ID = "puzzle.5d.play.3d" as const;
