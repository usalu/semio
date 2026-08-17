/** 🪟️ PDF/X Document (1.7) viewer -- `main` window: READ-ONLY typed twin of `component.rs`'s
 * `DocumentWindowKit` view model. Mirrors the Rust `render()` boundary's output shape -- one summary
 * line per PDF page, no mutation-shaped exports (no `SetPage` payload type). */

/** 📄️ One rendered page line -- mirrors the framework `DocumentPage` shape
 * (`framework.window.document`). */
export interface Pdf17XDocumentPage {
  text: string;
}

/** 👁️ The `main` window's typed view-model -- the TS mirror of the Rust `render()` boundary's
 * input (a bare `PdfSnapshot`). */
export interface Pdf17XDocumentViewModel {
  windowKindId: "framework.window.document";
  bodyKey: "framework.window.document";
  pages: Pdf17XDocumentPage[];
}

export const PDF17X_MAIN_WINDOW_KIND_ID = "framework.window.document" as const;
export const PDF17X_MAIN_BODY_KEY = "framework.window.document" as const;
