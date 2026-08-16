/** 🔍️ Puzzle 2d editor — Detail window: typed twin of `🦀️component.rs`'s view boundary. Mirrors the
 * pane's `render(documentJson: &str, envelope: &Puzzle2dScene) -> UiNode` — the fixture JSON plus the
 * mutation-capable runtime/active-utility state a surface carries (absent from the viewer's read-only
 * twin, see `👁️viewer/…/🟦️component.ts`). */

/** ✏️ The Detail window's typed view-model — mirrors the Rust `render()` boundary's inputs. */
export interface Puzzle2dDetailViewModel {
  windowKindId: "2d-detail";
  bodyKey: "puzzle2d.play.detail";
  surfaceId: "puzzle2d.play.composite.2d-detail";
  documentJson: string;
  activeUtilityId: string;
}

export const PUZZLE2D_PLAY_DETAIL_WINDOW_KIND_ID = "2d-detail" as const;
export const PUZZLE2D_PLAY_DETAIL_BODY_KEY = "puzzle2d.play.detail" as const;
export const PUZZLE2D_PLAY_DETAIL_SURFACE_ID = "puzzle2d.play.composite.2d-detail" as const;
