/** 🪟️ PDF/X Document (1.7) editor -- `main` window: typed twin of `component.rs`'s `DocumentWindowKit`
 * view model. Mirrors the Rust `render()` boundary's output shape -- one summary line per PDF page
 * (`MediaBox`/`CropBox` geometry plus the page's own extracted/authored `text`; see the Rust file's
 * own doc comment for the honest scope of what `text` is). */

/** 📄️ One rendered page line -- mirrors the framework `DocumentPage` shape
 * (`framework.window.document`). */
export interface Pdf17XDocumentPage {
  text: string;
}

/** ✏️ The `main` window's typed view-model -- the TS mirror of the Rust `render()` boundary's
 * input (a bare `PdfSnapshot`). */
export interface Pdf17XDocumentViewModel {
  windowKindId: "framework.window.document";
  bodyKey: "framework.window.document";
  pages: Pdf17XDocumentPage[];
}

/** ✏️ `set-page` payload shape -- mirrors `Pdf17XEditorCommand::SetPage`. APPENDS to the page's
 * existing text (there is no "replace" primitive in the current mutation vocabulary -- see the Rust
 * surface root's own doc comment). */
export interface Pdf17XSetPage {
  index: number;
  text: string;
}

export const PDF17X_MAIN_WINDOW_KIND_ID = "framework.window.document" as const;
export const PDF17X_MAIN_BODY_KEY = "framework.window.document" as const;
