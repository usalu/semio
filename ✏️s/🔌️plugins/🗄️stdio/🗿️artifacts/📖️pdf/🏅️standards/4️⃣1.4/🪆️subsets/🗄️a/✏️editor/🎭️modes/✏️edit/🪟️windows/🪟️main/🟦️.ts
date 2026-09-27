/** 🪟️ PDF/A Document (1.4) editor -- `main` window: typed twin of `🦀️.rs`'s `DocumentWindowKit`
 * view model. Mirrors the Rust `render()` boundary's output shape -- one summary line per PDF page
 * (`MediaBox`/`CropBox` geometry plus the page's own extracted/authored `text`; see the Rust file's
 * own doc comment for the honest scope of what `text` is). */

/** 📄️ One rendered page line -- mirrors the framework `DocumentPage` shape
 * (`framework.window.document`). */
export interface Pdf14ADocumentPage {
  text: string;
}

/** ✏️ The `main` window's typed view-model -- the TS mirror of the Rust `render()` boundary's
 * input (a bare `PdfSnapshot`). */
export interface Pdf14ADocumentViewModel {
  windowKindId: "framework.window.document";
  bodyKey: "framework.window.document";
  pages: Pdf14ADocumentPage[];
}

/** ✏️ Strict optimistic-concurrency payload for one prefilled document text draft. */
export interface Pdf14ASetPage {
  page: number;
  item: number;
  revision: string;
  text: string;
}

export const PDF14A_MAIN_WINDOW_KIND_ID = "framework.window.document" as const;
export const PDF14A_MAIN_BODY_KEY = "framework.window.document" as const;
