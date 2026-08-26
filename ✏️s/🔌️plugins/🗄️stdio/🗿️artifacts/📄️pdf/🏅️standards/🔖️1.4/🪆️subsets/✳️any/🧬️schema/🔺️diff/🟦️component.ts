/** 🔺️ PdfDiff (1.4) — a handcrafted sparse diff over the document's page tree, mirroring the Rust
 *  `PdfDiff` shape 1:1. `removed`/`modified` indices address the BASE state (removals applied
 *  descending); `added` indices address the FINAL state (insertions applied ascending). There is
 *  no `snapshot?: PdfSnapshot` full-replace slot — even `SetSnapshot`'s diff is `between`. */
import type { PageDoc } from '../📸️snapshot/🟦️component.ts';

export interface PdfPageDiff {
  width?: number;
  height?: number;
  text?: string;
}
export interface PdfPageModified {
  index: number;
  diff: PdfPageDiff;
}
export interface PdfPageAdded {
  index: number;
  page: PageDoc;
}
export interface PdfPagesDiff {
  removed?: number[];
  modified?: PdfPageModified[];
  added?: PdfPageAdded[];
}
export interface PdfDiff {
  pages?: PdfPagesDiff;
}
