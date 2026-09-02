/** 🎲️ Puzzle 2d viewer — Board window: typed twin of `🦀️.rs`'s view-model. Read-only mirror
 * of the shared `framework.window.mesh` scene payload `render()` produces — no mutation-shaped fields
 * (no selection, no brush/fill utility, no engagement session), matching the viewer's `ViewEmit`-only
 * contract. */

/** 👁️ One node placed in the flattened world-3d scene — real per-node position/kind/label read
 * straight off `Puzzle2dSnapshot.nodes`. `meshId` is "sphere" for a circle node, "box" for a
 * rectangle node (the mesh-engine's built-in placeholder geometry, not the fixture's own shape). */
export interface Puzzle2dViewBoardInstance {
  id: string;
  meshId: "sphere" | "box";
  position: [number, number, number];
  rotation: [number, number, number, number];
  scale: [number, number, number];
  label: string;
}

/** 👁️ The Board window's typed view-model — the TS mirror of the Rust `render()` boundary's inputs
 * (a bare `Puzzle2dSnapshot`, no runtime/config/utility state: a viewer has none of those). */
export interface Puzzle2dViewBoardViewModel {
  windowKindId: "framework.window.mesh";
  bodyKey: "framework.window.mesh";
  instances: Puzzle2dViewBoardInstance[];
}

export const PUZZLE2D_VIEW_BOARD_WINDOW_KIND_ID = "framework.window.mesh" as const;
export const PUZZLE2D_VIEW_BOARD_BODY_KEY = "framework.window.mesh" as const;
