/** 👁️ Puzzle 2d editor — Overview window: typed twin of `🦀️component.rs`'s view boundary. Mirrors
 * the pane's `render(documentJson: &str, envelope: &Puzzle2dScene) -> UiNode` — the fixture JSON plus
 * the mutation-capable runtime/active-utility state a surface carries (absent from the viewer's
 * read-only twin, see `👁️viewer/…/🟦️component.ts`). This is the one pane with brush/select utilities
 * (see `🪛️utilities/{🖌️brush,🖱️select}/🟦️component.ts` for their own typed twins). */

/** ✏️ The Overview window's typed view-model — mirrors the Rust `render()` boundary's inputs. */
export interface Puzzle2dOverviewViewModel {
  windowKindId: "2d-overview";
  bodyKey: "puzzle2d.play.overview";
  surfaceId: "puzzle2d.play.composite.2d-overview";
  documentJson: string;
  activeUtilityId: string;
}

export const PUZZLE2D_PLAY_OVERVIEW_WINDOW_KIND_ID = "2d-overview" as const;
export const PUZZLE2D_PLAY_OVERVIEW_BODY_KEY = "puzzle2d.play.overview" as const;
export const PUZZLE2D_PLAY_OVERVIEW_SURFACE_ID = "puzzle2d.play.composite.2d-overview" as const;
