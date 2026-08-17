/** 🎞️ Pptx transitional editor — `main` window: typed twin of `🦀️component.rs`'s
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

/** ✏️ `set-page` payload shape — mirrors `PptxTransitionalEditorCommand::SetPage`. Only writes
 * the FIRST text-bearing shape on the addressed slide (see the Rust window's own doc comment). */
export interface PptxTransitionalSetPage {
  index: number;
  text: string;
}

export const PPTX_TRANSITIONAL_MAIN_WINDOW_KIND_ID = "framework.window.document" as const;
export const PPTX_TRANSITIONAL_MAIN_BODY_KEY = "framework.window.document" as const;
