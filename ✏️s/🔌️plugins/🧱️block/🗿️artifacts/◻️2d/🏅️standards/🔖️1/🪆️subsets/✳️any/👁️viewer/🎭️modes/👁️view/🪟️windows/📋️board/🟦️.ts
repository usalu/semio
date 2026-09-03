/** 📋️ Block 2D viewer — board window: typed twin of `🦀️.rs`'s view-model. Read-only mirror
 * of the rim geometry `render()` produces — real per-handle-kind and per-handle-template data (no
 * mutation-shaped fields), matching the viewer's `ViewEmit`-only contract. */

/** 👁️ One handle-kind catalog row, read straight off `Block2dSnapshot.handleKinds`. */
export interface Block2dViewHandleKind {
  id: string;
  label: string;
  color: string;
}

/** 👁️ One rim-handle template instance, read straight off `Block2dSnapshot.handles`. */
export interface Block2dViewHandle {
  id: string;
  handleKind: string;
  angleDegrees: number;
  radius: number;
}

/** 👁️ The board window's typed view-model — the TS mirror of the Rust `render()` boundary's inputs
 * (a bare `Block2dSnapshot`, no runtime/config state: a viewer has none of those). */
export interface Block2dViewBoardViewModel {
  windowKindId: "block2d-view-board";
  bodyKey: "block2d.view.board";
  surfaceId: "block2d.view.board2d/board";
  nodeKindLabel: string;
  handleKinds: Block2dViewHandleKind[];
  handles: Block2dViewHandle[];
}

export const BLOCK2D_VIEW_BOARD_WINDOW_KIND_ID = "block2d-view-board" as const;
export const BLOCK2D_VIEW_BOARD_BODY_KEY = "block2d.view.board" as const;
export const BLOCK2D_VIEW_BOARD_SURFACE_ID = "block2d.view.board2d/board" as const;
