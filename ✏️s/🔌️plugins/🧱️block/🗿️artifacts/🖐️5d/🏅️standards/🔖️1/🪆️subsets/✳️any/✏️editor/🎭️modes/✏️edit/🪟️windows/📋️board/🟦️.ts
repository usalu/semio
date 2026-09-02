/** 📋️ Block 5D editor — Board window: typed twin of `🦀️.rs`'s view-model. A lightweight
 * 2D-projection summary surface (part kind label + 2d grip count), matching `render()`'s inputs. */

/** 📋️ The Board window's typed view-model — the TS mirror of the Rust `render()` boundary's inputs
 * (a bare `Block5dSnapshot`, resolved locale labels). */
export interface Block5dBoardViewModel {
  windowKindId: "block5d-board";
  bodyKey: "block5d.play.board";
  partLabel: string;
  gripCount: number;
}

export const BLOCK5D_BOARD_WINDOW_KIND_ID = "block5d-board" as const;
export const BLOCK5D_BOARD_BODY_KEY = "block5d.play.board" as const;
