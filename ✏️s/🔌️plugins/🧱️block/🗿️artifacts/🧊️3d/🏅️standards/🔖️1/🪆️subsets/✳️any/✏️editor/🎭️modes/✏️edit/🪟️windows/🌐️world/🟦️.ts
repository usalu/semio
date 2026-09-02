/** 🌐️ Block 3D editor — world window: typed twin of `🦀️.rs`'s view-model. Mirrors the
 * window's `render(definition: &Block3dSnapshot, config: &Block3dConfig, window_id: &str)` boundary
 * — the world-3d scene payload (meshes/instances/vortices/camera) plus the per-window view state
 * (arrangement/spacing/active representation/brush) a mutation-capable surface carries, absent
 * entirely from the viewer's read-only twin (see `👁️viewer/…/🌐️world/🟦️.ts`). */

/** ✏️ The world window's typed view-model — mirrors the Rust `render()` boundary's inputs. */
export interface Block3dWorldViewModel {
  windowKindId: "block3d-world";
  bodyKey: "block3d.play.world";
  surfaceId: "block3d.play.world";
  windowId: string;
  representationIds: string[];
  arrangement: "overlap" | "x" | "y" | "z";
  spacing: number;
  activeUtility: "select" | "surfaceBrush";
}

export const BLOCK3D_WORLD_WINDOW_KIND_ID = "block3d-world" as const;
export const BLOCK3D_WORLD_BODY_KEY = "block3d.play.world" as const;
export const BLOCK3D_PLAY_SURFACE_ID = "block3d.play.world" as const;
