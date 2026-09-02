/** 📋️ Block 2D editor — board window: typed twin of `🦀️.rs`'s view-model. Mirrors the
 * window's `render(definition: &Block2dSnapshot, labels: &Block2dLabels) -> UiNode` boundary — a
 * two-line rim-summary surface (block2d's only window kind; the full node-kind editing surface
 * lives in the document/inspection panels, not this window). */

/** ✏️ The board window's typed view-model — mirrors the Rust `render()` boundary's inputs. */
export interface Block2dBoardViewModel {
  windowKindId: "block2d-board";
  bodyKey: "block2d.play.board";
  surfaceId: "block2d.play.board2d/board";
  nodeKindLabel: string;
  handleKindCount: number;
  handleCount: number;
}

export const BLOCK2D_BOARD_WINDOW_KIND_ID = "block2d-board" as const;
export const BLOCK2D_BOARD_BODY_KEY = "block2d.play.board" as const;
export const BLOCK2D_BOARD_SURFACE_ID = "block2d.play.board2d/board" as const;
