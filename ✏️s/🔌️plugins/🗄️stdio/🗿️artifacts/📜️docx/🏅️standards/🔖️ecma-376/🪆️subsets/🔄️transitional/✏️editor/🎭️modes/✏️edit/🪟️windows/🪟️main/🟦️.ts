/** 📄️ Docx transitional editor — `main` window: typed twin of `🦀️.rs`'s
 * `DocumentWindowKit` view-model. Mirrors the Rust `render()` boundary's output shape — one page
 * per top-level `DocxDocument.body` block. */

/** 📄️ One rendered page — mirrors the framework `DocumentPage` shape (`framework.window.document`). */
export interface DocxTransitionalMainPage {
  text: string;
}

/** ✏️ The `main` window's typed view-model — the TS mirror of the Rust `render()` boundary's
 * input (a bare `DocxSnapshot`). */
export interface DocxTransitionalMainViewModel {
  windowKindId: "framework.window.document";
  bodyKey: "framework.window.document";
  pages: DocxTransitionalMainPage[];
}

/** ✏️ Strict optimistic-concurrency payload for one prefilled document text draft. */
export interface DocxTransitionalSetPage {
  page: number;
  item: number;
  revision: string;
  text: string;
}

export const DOCX_TRANSITIONAL_MAIN_WINDOW_KIND_ID = "framework.window.document" as const;
export const DOCX_TRANSITIONAL_MAIN_BODY_KEY = "framework.window.document" as const;
