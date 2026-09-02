/** 📄️ Docx transitional viewer — `main` window: typed twin of `🦀️.rs`'s
 * `DocumentWindowKit` view-model. Read-only mirror of the Rust `render()` boundary's output — no
 * edit-target fields, no command payload types (a viewer's `Command` never carries a document
 * mutation). */

/** 👁️ One rendered page — mirrors the framework `DocumentPage` shape (`framework.window.document`). */
export interface DocxTransitionalMainPage {
  text: string;
}

/** 👁️ The `main` window's typed view-model — the TS mirror of the Rust `render()` boundary's
 * input (a bare `DocxSnapshot`). */
export interface DocxTransitionalMainViewModel {
  windowKindId: "framework.window.document";
  bodyKey: "framework.window.document";
  pages: DocxTransitionalMainPage[];
}

export const DOCX_TRANSITIONAL_MAIN_WINDOW_KIND_ID = "framework.window.document" as const;
export const DOCX_TRANSITIONAL_MAIN_BODY_KEY = "framework.window.document" as const;
