/** 🧊️ Puzzle 3d editor — the one `World3d` window kind: typed twin of `🦀️.rs`'s `render()`
 * boundary. One KIND, many INSTANCES (the default layout splits it into an orthographic "Top" and a
 * three-point "Perspective" pane) — every view-local option (camera, grid, LOD, vortex display, sun,
 * selection method) is composed from that exact instance's `WindowConfig`. */

export const PUZZLE3D_MAIN_WINDOW_KIND_ID = "puzzle3d-main" as const;
export const PUZZLE3D_MAIN_WINDOW_INSTANCE_TOP = "puzzle3d-main-top" as const;
export const PUZZLE3D_MAIN_WINDOW_INSTANCE_PERSPECTIVE = "puzzle3d-main-perspective" as const;
export const PUZZLE3D_MAIN_BODY_KEY = "puzzle3d.play.composite" as const;
export const PUZZLE3D_MAIN_SURFACE_VIEWPORT = "puzzle.3d.play.viewport" as const;

/** 🪟️ Per-instance camera/LOD/grid/vortex/selection projection of `Puzzle3dWindowConfig`. */
export interface Puzzle3dMainWindowOptions {
  cameraPosition: [number, number, number];
  cameraTarget: [number, number, number];
  cameraZoom: number;
  gridVisible: boolean;
  gridSnapEnabled: boolean;
  gridSpacing: number;
  lodAutomatic: boolean;
  lodDepthVariable: boolean;
  lodManual: number;
  vortexShow: "always" | "selected";
  vortexDirection: "outwards" | "inwards";
  transformMove: boolean;
  transformRotate: boolean;
}

/** 🧱️ The Main window's typed view-model — the TS mirror of the Rust `render()` boundary's inputs:
 * pre-computed `instancesJson`/`meshesJson` (from the app's geometry cache), the active utility, and
 * this instance's own materialized `Puzzle3dMainWindowOptions`. */
export interface Puzzle3dMainViewModel {
  windowKindId: "puzzle3d-main";
  bodyKey: "puzzle3d.play.composite";
  surfaceId: "puzzle.3d.play.viewport";
  instancesJson: string;
  meshesJson: string;
  activeUtility: "select" | "brush" | "fill" | "volumeBrush";
  options: Puzzle3dMainWindowOptions;
}

export * from "./🪛️utilities/🔄️transform/🟦️";
export * from "./🪛️utilities/🖌️brush/🟦️";
export * from "./🪛️utilities/🧊️volume-brush/🟦️";
export * from "./🪛️utilities/🚚️world-relocate/🟦️";
