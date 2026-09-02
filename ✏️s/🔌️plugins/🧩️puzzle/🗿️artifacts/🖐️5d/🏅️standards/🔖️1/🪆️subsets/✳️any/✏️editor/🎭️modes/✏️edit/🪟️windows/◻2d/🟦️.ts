/** ◻️ Puzzle 5D editor — Board2d window: typed twin of `🦀️.rs`'s view-model. Mirrors the
 * pane's `render(envelope: &Puzzle5dScene)` boundary — the flat 2D projection (board nodes/handles/
 * edges plus glyph catalogs and placement compatibility) of the unified 5d document (absent entirely
 * from the viewer's read-only twin, see `👁️viewer/…/🟦️.ts`). */

/** ✏️ The Board2d window's typed view-model — mirrors the Rust `render()` boundary's inputs. */
export interface Puzzle5dBoard2dViewModel {
  windowKindId: "puzzle5d-2d";
  bodyKey: "puzzle.5d.play.2d";
  surfaceId: "puzzle.5d.play.2d";
}

export const PUZZLE5D_BOARD2D_WINDOW_KIND_ID = "puzzle5d-2d" as const;
export const PUZZLE5D_BOARD2D_BODY_KEY = "puzzle.5d.play.2d" as const;
export const PUZZLE5D_BOARD2D_SURFACE_ID = "puzzle.5d.play.2d" as const;
