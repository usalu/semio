/** 🧬️ PdfSnapshot (1.4) schema — the document's real page tree, mirroring the Rust
 *  `PdfSnapshot` shape 1:1. `width`/`height` are the page's /MediaBox extent; `text` is its shown
 *  text (the operand bytes of the text-showing operators, not font-decoded). */
export interface PageDoc {
  width: number;
  height: number;
  text: string;
}
export interface PdfSnapshot {
  /** @state artifact */ schema: string;
  /** @state artifact */ pages: PageDoc[];
}
