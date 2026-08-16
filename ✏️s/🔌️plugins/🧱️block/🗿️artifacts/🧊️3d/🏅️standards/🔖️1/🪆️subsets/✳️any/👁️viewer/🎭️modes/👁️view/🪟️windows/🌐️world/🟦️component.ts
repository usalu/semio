/** 🌐️ Block 3D viewer — world window: typed twin of `🦀️component.rs`'s pure `render(document:
 * &Block3dSnapshot)` boundary — the read-only world-3d scene payload (meshes/instances/vortices/
 * camera) only, no per-window view state (arrangement/spacing/active utility/brush — those are
 * editor-only, see `✏️editor/…/🌐️world/🟦️component.ts`). */

/** 👁️ The world window's typed view-model — mirrors the Rust `render()` boundary's sole input. */
export interface Block3dViewWorldViewModel {
  windowKindId: "framework.window.mesh";
  bodyKey: "framework.window.mesh";
  surfaceId: "block3d.view.world";
}

export const BLOCK3D_VIEW_WORLD_WINDOW_KIND_ID = "framework.window.mesh" as const;
export const BLOCK3D_VIEW_WORLD_BODY_KEY = "framework.window.mesh" as const;
export const BLOCK3D_VIEW_SURFACE_ID = "block3d.view.world" as const;
