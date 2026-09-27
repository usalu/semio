/** 🎞️ Pptx transitional editor — `main` window: typed twin of `🦀️.rs`'s
 * `DocumentWindowKit` view-model. Mirrors the Rust `render()` boundary's output shape — one page
 * per slide, its text the concatenation of every text-bearing shape on that slide. */

/** 🎞️ One rendered page — mirrors the framework `DocumentPage` shape (`framework.window.document`). */
export interface PptxTransitionalMainPage {
  text: string;
}

/** ✏️ The `main` window's typed view-model — the TS mirror of the Rust `render()` boundary's
 * input (a bare `PptxSnapshot`). */
export interface PptxTransitionalMainViewModel {
  windowKindId: "framework.window.document";
  bodyKey: "framework.window.document";
  pages: PptxTransitionalMainPage[];
}

/** ✏️ Strict optimistic-concurrency payload for one prefilled document text draft. */
export interface PptxTransitionalSetPage {
  page: number;
  item: number;
  revision: string;
  text: string;
}

export const PPTX_TRANSITIONAL_MAIN_WINDOW_KIND_ID = "framework.window.document" as const;
export const PPTX_TRANSITIONAL_MAIN_BODY_KEY = "framework.window.document" as const;
