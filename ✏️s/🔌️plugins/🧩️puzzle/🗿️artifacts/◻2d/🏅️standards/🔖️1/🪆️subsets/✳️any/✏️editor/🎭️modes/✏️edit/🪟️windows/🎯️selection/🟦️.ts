/** 🎯️ Puzzle 2d editor — Selection window: typed twin of `🦀️.rs`'s view boundary. Mirrors
 * the pane's `render(documentJson: &str, envelope: &Puzzle2dScene) -> UiNode` — the fixture JSON plus
 * the mutation-capable runtime/active-utility state a surface carries (absent from the viewer's
 * read-only twin, see `👁️viewer/…/🟦️.ts`). */

/** ✏️ The Selection window's typed view-model — mirrors the Rust `render()` boundary's inputs. */
export interface Puzzle2dSelectionViewModel {
  windowKindId: "2d-selection";
  bodyKey: "puzzle2d.play.selection";
  surfaceId: "puzzle2d.play.composite.2d-selection";
  documentJson: string;
  activeUtilityId: string;
}

export const PUZZLE2D_PLAY_SELECTION_WINDOW_KIND_ID = "2d-selection" as const;
export const PUZZLE2D_PLAY_SELECTION_BODY_KEY = "puzzle2d.play.selection" as const;
export const PUZZLE2D_PLAY_SELECTION_SURFACE_ID = "puzzle2d.play.composite.2d-selection" as const;
