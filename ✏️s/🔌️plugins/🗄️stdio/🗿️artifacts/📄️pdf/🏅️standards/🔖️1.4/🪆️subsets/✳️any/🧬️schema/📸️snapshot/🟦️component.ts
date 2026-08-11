/** 🧬️ PdfSnapshot (1.4) schema — the intentionally-frozen pre-real-codec stub (see the Rust
 *  module's own doc comment; W0 recon: kept minimally alive under its own path, no object-graph
 *  model). */
export interface PageDoc {
  width: number;
  height: number;
  text: string;
}
export interface PdfSnapshot {
  /** @state persistent */ schema: string;
  /** @state persistent */ page: PageDoc;
}
